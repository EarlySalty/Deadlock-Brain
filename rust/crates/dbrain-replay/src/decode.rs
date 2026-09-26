use crate::{
    frame::{BoundedStream, SharedState, State},
    *,
};
use haste_core::{
    demostream::CmdHeader,
    entities::{DeltaHeader, Entity, fkey_from_path},
    fieldvalue::FieldValue,
    fxhash,
    parser::{Context, Parser, Visitor},
};
use prost::Message;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    io::Read,
    rc::Rc,
};
use valveprotos::common::{
    CDemoClassInfo, CDemoFileHeader, CDemoFileInfo, CDemoStop, CDemoSyncTick,
    CsvcMsgCreateStringTable, CsvcMsgPacketEntities, CsvcMsgServerInfo, CsvcMsgUpdateStringTable,
    EDemoCommands, SvcMessages,
};

fn observed<T>(value: Option<T>) -> Observed<T> {
    value.map_or_else(
        || Observed::unknown(UnknownReason::NotPresent),
        Observed::known,
    )
}

struct Collector {
    request: ReplayRequest,
    state: SharedState,
    observations: Vec<ReplayObservation>,
    generation: String,
    raw: Option<RawLocator>,
    interval: Observed<f32>,
    classes: BTreeMap<u64, String>,
    selected_hashes: BTreeSet<u64>,
    creations: BTreeMap<i32, u32>,
    live: BTreeMap<i32, (ReplayEntity, i32)>,
    unknown_packets: BTreeSet<u32>,
    packet_index: u32,
    entity_callback_index: u32,
    seen_send_tables: bool,
    table_specs: Vec<crate::stringtable::TableSpec>,
}
impl Collector {
    fn emit(
        &mut self,
        ctx: &Context,
        entity: Observed<ReplayEntity>,
        event: ObservationKind,
    ) -> Result<(), ReplayFailure> {
        if self.observations.len() >= self.request.budget.max_observations as usize {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let raw = self.raw.clone().ok_or(ReplayFailure::ParserFailure)?;
        let time = ReplayTime {
            tick: tick_from_legacy(ctx.tick())?,
            tick_interval_seconds: self.interval.clone(),
            game_time_seconds: Observed::unknown(if ctx.tick() == -1 {
                UnknownReason::InitializationTick
            } else {
                UnknownReason::TickOriginNotEstablished
            }),
        };
        let identity = serde_json::to_vec(&(&raw, &time, &entity, &event))
            .map_err(|_| ReplayFailure::ParserFailure)?;
        let observation_id = hash_parts(&[self.generation.as_bytes(), &identity]);
        self.observations.push(ReplayObservation {
            observation_id,
            time,
            entity,
            raw,
            event,
        });
        Ok(())
    }
    fn plain(&mut self, ctx: &Context, event: ObservationKind) -> Result<(), ReplayFailure> {
        self.emit(ctx, Observed::unknown(UnknownReason::NotPresent), event)
    }
}
impl Visitor for Collector {
    type Error = ReplayFailure;
    fn should_track_entity(&self, serializer_name_hash: u64) -> bool {
        self.selected_hashes.contains(&serializer_name_hash)
    }
    fn on_cmd(
        &mut self,
        ctx: &Context,
        header: &CmdHeader,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.raw = self.state.borrow().locator.clone();
        self.packet_index = 0;
        self.entity_callback_index = 0;
        match header.cmd {
            EDemoCommands::DemFileHeader => {
                let msg =
                    CDemoFileHeader::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
                if msg.demo_file_stamp != "PBDEMS2" {
                    return Err(ReplayFailure::InvalidContainer);
                }
                if msg
                    .game_directory
                    .as_deref()
                    .is_some_and(|g| g != "citadel")
                {
                    return Err(ReplayFailure::UnknownStructure);
                }
                self.plain(
                    ctx,
                    ObservationKind::FileHeader {
                        patch_version: observed(msg.patch_version),
                        build_number: observed(msg.build_num),
                        game_directory: observed(msg.game_directory),
                    },
                )?;
            }
            EDemoCommands::DemFileInfo => {
                let msg = CDemoFileInfo::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
                if msg.playback_time.is_some_and(|v| !v.is_finite() || v < 0.0)
                    || msg.playback_ticks.is_some_and(|v| v < 0)
                {
                    return Err(ReplayFailure::UnknownStructure);
                }
                // Dota's match_id is NOT silently reinterpreted as a Deadlock match ID.
                self.plain(
                    ctx,
                    ObservationKind::FileInfo {
                        playback_ticks: observed(msg.playback_ticks),
                        playback_seconds: observed(msg.playback_time),
                    },
                )?;
            }
            EDemoCommands::DemStop => {
                CDemoStop::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            }
            EDemoCommands::DemSyncTick => {
                CDemoSyncTick::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            }
            EDemoCommands::DemClassInfo => {
                if !self.classes.is_empty() {
                    return Err(ReplayFailure::UnknownStructure);
                }
                let msg = CDemoClassInfo::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
                if msg.classes.is_empty()
                    || msg.classes.len() > self.request.budget.max_entities as usize
                {
                    return Err(ReplayFailure::UnknownStructure);
                }
                let count = msg.classes.len();
                let mut ids = BTreeSet::new();
                for (ordinal, c) in msg.classes.into_iter().enumerate() {
                    let id = c.class_id.ok_or(ReplayFailure::UnknownStructure)?;
                    // The pinned backend indexes the vector, not the declared class_id.
                    if id < 0 || id as usize != ordinal || id as usize >= count || !ids.insert(id) {
                        return Err(ReplayFailure::UnknownStructure);
                    }
                    let name = c.network_name.ok_or(ReplayFailure::UnknownStructure)?;
                    if name.is_empty()
                        || name.len() > 128
                        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    {
                        return Err(ReplayFailure::UnknownStructure);
                    }
                    let h = fxhash::hash_bytes(name.as_bytes());
                    if self.classes.insert(h, name).is_some() {
                        return Err(ReplayFailure::UnknownStructure);
                    }
                }
            }
            EDemoCommands::DemSendTables => {
                // haste ignores subsequent schemas. Reject instead of continuing under stale state.
                if self.seen_send_tables {
                    return Err(ReplayFailure::UnknownStructure);
                }
                self.seen_send_tables = true;
            }
            EDemoCommands::DemFullPacket => {
                let full = valveprotos::common::CDemoFullPacket::decode(data)
                    .map_err(|_| ReplayFailure::DamagedReplay)?;
                if let Some(tables) = full.string_table {
                    for table in tables.tables {
                        if table.items.len() > self.request.budget.max_entities as usize {
                            return Err(ReplayFailure::BudgetExceeded);
                        }
                        // Upstream only updates existing tables and would silently omit new ones.
                        if ctx
                            .string_tables()
                            .and_then(|tables| tables.find_table(table.table_name()))
                            .is_none()
                        {
                            return Err(ReplayFailure::UnknownStructure);
                        }
                    }
                }
            }
            EDemoCommands::DemStringTables => {
                // The pinned synchronous parser does not apply this standalone command.
                // Full-packet string tables and network table updates ARE supported.
                return Err(ReplayFailure::UnknownStructure);
            }
            _ => {}
        }
        Ok(())
    }
    fn on_packet(
        &mut self,
        ctx: &Context,
        message_type: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        {
            let mut state = self.state.borrow_mut();
            if state.packets >= self.request.budget.max_packets {
                return Err(ReplayFailure::BudgetExceeded);
            }
            state.packets += 1;
        }
        self.raw = self.state.borrow().locator.clone();
        let raw = self.raw.as_mut().ok_or(ReplayFailure::ParserFailure)?;
        raw.packet_index = Some(self.packet_index);
        raw.payload_sha256 = hash(data);
        self.packet_index = self
            .packet_index
            .checked_add(1)
            .ok_or(ReplayFailure::BudgetExceeded)?;
        self.entity_callback_index = 0;
        if message_type == SvcMessages::SvcServerInfo as u32 {
            let msg = CsvcMsgServerInfo::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            if msg
                .tick_interval
                .is_some_and(|t| !t.is_finite() || !(0.000_001..=1.0).contains(&t))
            {
                return Err(ReplayFailure::UnknownStructure);
            }
            // Do not inherit haste's default tick interval; absent means unknown.
            self.interval = observed(msg.tick_interval);
            self.plain(
                ctx,
                ObservationKind::ServerInfo {
                    tick_interval_seconds: self.interval.clone(),
                },
            )?;
        } else if message_type == SvcMessages::SvcCreateStringTable as u32 {
            let msg =
                CsvcMsgCreateStringTable::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            if msg.num_entries() < 0
                || msg.num_entries() as u32 > self.request.budget.max_entities
                || self.table_specs.len() >= 256
            {
                return Err(ReplayFailure::BudgetExceeded);
            }
            let spec = crate::stringtable::TableSpec::from_message(&msg)?;
            if self.table_specs.iter().any(|table| table.name == spec.name) {
                return Err(ReplayFailure::UnknownStructure);
            }
            let mut expanded = Vec::new();
            let table_data = if msg.data_compressed() {
                let size = snap::raw::decompress_len(msg.string_data())
                    .map_err(|_| ReplayFailure::DamagedReplay)?;
                self.state
                    .borrow_mut()
                    .charge_decoded(size as u64, &self.request.budget)?;
                expanded
                    .try_reserve_exact(size)
                    .map_err(|_| ReplayFailure::BudgetExceeded)?;
                expanded.resize(size, 0);
                let actual = snap::raw::Decoder::new()
                    .decompress(msg.string_data(), &mut expanded)
                    .map_err(|_| ReplayFailure::DamagedReplay)?;
                if actual != size {
                    return Err(ReplayFailure::DamagedReplay);
                }
                &expanded[..]
            } else {
                msg.string_data()
            };
            spec.preflight(
                table_data,
                msg.num_entries() as u32,
                &self.request.budget,
                &self.state,
            )?;
            self.table_specs.push(spec);
        } else if message_type == SvcMessages::SvcUpdateStringTable as u32 {
            let msg =
                CsvcMsgUpdateStringTable::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            if msg.num_changed_entries() < 0
                || msg.num_changed_entries() as u32 > self.request.budget.max_entities
            {
                return Err(ReplayFailure::BudgetExceeded);
            }
            let spec = self
                .table_specs
                .get(msg.table_id() as usize)
                .ok_or(ReplayFailure::UnknownStructure)?;
            spec.preflight(
                msg.string_data(),
                msg.num_changed_entries() as u32,
                &self.request.budget,
                &self.state,
            )?;
        } else if message_type == SvcMessages::SvcPacketEntities as u32 {
            let msg =
                CsvcMsgPacketEntities::decode(data).map_err(|_| ReplayFailure::DamagedReplay)?;
            if msg.updated_entries() < 0
                || msg.updated_entries() as u32 > self.request.budget.max_entities
                || msg.max_entries() < 0
                || msg.max_entries() as u32 > self.request.budget.max_entities
            {
                return Err(ReplayFailure::BudgetExceeded);
            }
        } else if [
            SvcMessages::SvcClearAllStringTables as u32,
            SvcMessages::SvcFullFrameSplit as u32,
            SvcMessages::SvcFlattenedSerializer as u32,
        ]
        .contains(&message_type)
        {
            // These messages mutate decoding state, unlike an opaque gameplay/event message.
            // The pinned synchronous backend does not implement them; stale state is not evidence.
            return Err(ReplayFailure::UnknownStructure);
        } else if self.unknown_packets.insert(message_type) {
            if self.unknown_packets.len() > 1024 {
                return Err(ReplayFailure::BudgetExceeded);
            }
            // One opaque marker per type, not a guessed event and not a dump of private payloads.
            self.plain(ctx, ObservationKind::UnknownPacket { message_type })?;
        }
        Ok(())
    }
    fn on_entity(
        &mut self,
        ctx: &Context,
        delta: DeltaHeader,
        entity: &Entity,
    ) -> Result<(), Self::Error> {
        let index = entity.index();
        if index < 0 || index as u32 >= self.request.budget.max_entities {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let class_name = self
            .classes
            .get(&entity.serializer().serializer_name.hash)
            .ok_or(ReplayFailure::UnknownStructure)?
            .clone();
        let action = if delta == DeltaHeader::CREATE {
            EntityAction::Create
        } else if delta == DeltaHeader::UPDATE {
            EntityAction::Update
        } else if delta == DeltaHeader::LEAVE {
            EntityAction::LeaveVisibility
        } else if delta == DeltaHeader::DELETE {
            EntityAction::Delete
        } else {
            return Err(ReplayFailure::UnknownStructure);
        };
        if action == EntityAction::Create {
            let ordinal = self.creations.entry(index).or_default();
            *ordinal = ordinal
                .checked_add(1)
                .ok_or(ReplayFailure::BudgetExceeded)?;
            self.live.insert(
                index,
                (
                    ReplayEntity {
                        network_index: index,
                        creation_ordinal: *ordinal,
                        network_serial: None,
                        class_name,
                    },
                    ctx.tick(),
                ),
            );
        }
        let (reference, last_tick) = self
            .live
            .get_mut(&index)
            .ok_or(ReplayFailure::UnknownStructure)?;
        let reference = reference.clone();
        let callback_index = self.entity_callback_index;
        self.entity_callback_index = self
            .entity_callback_index
            .checked_add(1)
            .ok_or(ReplayFailure::BudgetExceeded)?;
        if action == EntityAction::Update
            && ctx.tick() >= *last_tick
            && i64::from(ctx.tick()) - i64::from(*last_tick)
                < i64::from(self.request.selection.sample_every_ticks)
        {
            return Ok(());
        }
        *last_tick = ctx.tick();
        let fields = if matches!(action, EntityAction::Delete | EntityAction::LeaveVisibility) {
            self.live.remove(&index);
            BTreeMap::new()
        } else {
            extract_fields(entity)
        };
        let raw = self.raw.as_mut().ok_or(ReplayFailure::ParserFailure)?;
        raw.entity_callback_index = Some(callback_index);
        raw.requires_state_prefix = true;
        self.emit(
            ctx,
            Observed::known(reference),
            ObservationKind::EntityState { action, fields },
        )
    }
}

fn scalar(value: Option<&FieldValue>) -> Observed<ReplayScalar> {
    let Some(value) = value else {
        return Observed::unknown(UnknownReason::NotPresent);
    };
    let scalar = match value {
        FieldValue::I64(v) => ReplayScalar::Signed(*v),
        FieldValue::U64(v) => ReplayScalar::Unsigned(*v),
        FieldValue::F32(v) if v.is_finite() => ReplayScalar::Float(*v),
        FieldValue::Bool(v) => ReplayScalar::Boolean(*v),
        FieldValue::Vector3(v) if v.iter().all(|n| n.is_finite()) => ReplayScalar::Vector3(*v),
        _ => return Observed::unknown(UnknownReason::Unsupported),
    };
    Observed::known(scalar)
}
fn extract_fields(entity: &Entity) -> BTreeMap<String, ReplayField> {
    // Property paths are schema-sensitive. Preserve raw components; do NOT invent world units,
    // resolve handles without a serial, or turn missing counters into zero.
    const PATHS: &[&[&str]] = &[
        &["m_iTeamNum"],
        &["m_iHealth"],
        &["m_iMaxHealth"],
        &["m_hPawn"],
        &["m_CCitadelHeroComponent", "m_spawnedHero", "m_nHeroID"],
        &["m_CCitadelHeroComponent", "m_loadingHero", "m_nHeroID"],
        &["CBodyComponent", "m_cellX"],
        &["CBodyComponent", "m_cellY"],
        &["CBodyComponent", "m_cellZ"],
        &["CBodyComponent", "m_vecX"],
        &["CBodyComponent", "m_vecY"],
        &["CBodyComponent", "m_vecZ"],
    ];
    PATHS
        .iter()
        .map(|path| {
            (
                path.join("."),
                ReplayField {
                    value: scalar(entity.get_field_value(&fkey_from_path(path))),
                    unit: Observed::unknown(UnknownReason::NotIndependentlyVerified),
                },
            )
        })
        .collect()
}

/// Only called after the worker installed OS limits and the syscall sandbox.
pub(crate) fn decode_stream(
    reader: impl Read,
    artifact: ReplayArtifact,
    request: ReplayRequest,
) -> Result<ReplayReport, ReplayFailure> {
    let state = Rc::new(RefCell::new(State::default()));
    let stream = BoundedStream::new(
        reader,
        artifact.byte_length,
        request.budget.clone(),
        state.clone(),
    )?;
    let generation = generation_id(&artifact, &request.selection)?;
    let selected_hashes = request
        .selection
        .entity_classes
        .iter()
        .map(|s| fxhash::hash_bytes(s.as_bytes()))
        .collect();
    let visitor = Collector {
        request: request.clone(),
        state: state.clone(),
        observations: Vec::new(),
        generation: generation.clone(),
        raw: None,
        interval: Observed::unknown(UnknownReason::NotPresent),
        classes: BTreeMap::new(),
        selected_hashes,
        creations: BTreeMap::new(),
        live: BTreeMap::new(),
        unknown_packets: BTreeSet::new(),
        packet_index: 0,
        entity_callback_index: 0,
        seen_send_tables: false,
        table_specs: Vec::new(),
    };
    let mut parser = Parser::from_stream_with_visitor(stream, visitor)
        .map_err(|_| ReplayFailure::ParserFailure)?;
    if let Err(error) = parser.run_to_end() {
        return Err(state
            .borrow()
            .failure
            .or_else(|| error.downcast_ref::<ReplayFailure>().copied())
            .unwrap_or(ReplayFailure::ParserFailure));
    }
    let collector = parser.into_visitor();
    let state = state.borrow();
    Ok(ReplayReport {
        contract_version: CONTRACT_VERSION.into(),
        artifact,
        parser_revision: parser_revision(),
        schema_revision: SCHEMA_REVISION.into(),
        extraction_revision: EXTRACTION_REVISION.into(),
        validity: brain_contracts::source::GameValidity::unknown(),
        entity_mapping: BTreeMap::new(),
        selection: request.selection,
        generation_id: generation,
        commands: state.commands,
        packets: state.packets,
        total_decoded_bytes: state.decoded,
        observations: collector.observations,
        capabilities: capabilities(),
        real_replay_verified: false,
        coaching_eligible: false,
    })
}
