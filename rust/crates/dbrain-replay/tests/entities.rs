mod common;
use common::*;
use dbrain_replay::*;
use prost::Message;
use valveprotos::common::*;

// Original synthetic bitstreams following the pinned Source 2 wire schema.
// These are codec tests, NOT evidence that a real game's property semantics are verified.
fn field_code(op: usize) -> Vec<bool> {
    let weights = [
        36271, 10334, 1375, 646, 4128, 35, 3, 521, 2942, 560, 471, 10530, 251, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 310, 2, 1, 1837, 149, 300, 634, 1, 1, 1, 76, 271, 99, 25474,
    ];
    type HuffmanNode = (u32, usize, Vec<(usize, Vec<bool>)>);
    let mut nodes: Vec<HuffmanNode> = weights
        .into_iter()
        .enumerate()
        .map(|(i, w)| (w, i, vec![(i, vec![])]))
        .collect();
    let mut serial = nodes.len();
    while nodes.len() > 1 {
        // Min weight, then highest node ordinal first, as specified by the codec's Huffman tree.
        nodes.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let (left_w, _, left) = nodes.pop().unwrap();
        let (right_w, _, right) = nodes.pop().unwrap();
        let codes = left
            .into_iter()
            .map(|(i, p)| (i, [vec![false], p].concat()))
            .chain(
                right
                    .into_iter()
                    .map(|(i, p)| (i, [vec![true], p].concat())),
            )
            .collect();
        nodes.push((left_w + right_w, serial, codes));
        serial += 1;
    }
    nodes
        .pop()
        .unwrap()
        .2
        .into_iter()
        .find(|(i, _)| *i == op)
        .unwrap()
        .1
}
fn field(bits: &mut Bits, health: Option<u32>) {
    if health.is_some() {
        for b in field_code(0) {
            bits.bits(u64::from(b), 1);
        }
    }
    for b in field_code(39) {
        bits.bits(u64::from(b), 1);
    }
    if let Some(health) = health {
        bits.uvarint(health);
    }
}
fn entity_message(action: u64, health: Option<u32>) -> (u32, Vec<u8>) {
    let mut bits = Bits::default();
    bits.ubitvar(4);
    bits.bits(action, 2);
    if action == 2 {
        // A single class takes zero class-id bits. The parser reads a 17-bit network serial,
        // but does not expose it; the adapter must never claim the local creation ordinal is it.
        bits.bits(77, 17);
        bits.uvarint(0);
    }
    if action == 0 || action == 2 {
        field(&mut bits, health);
    }
    let msg = CsvcMsgPacketEntities {
        max_entries: Some(16),
        updated_entries: Some(1),
        entity_data: Some(bits.bytes),
        ..Default::default()
    };
    (SvcMessages::SvcPacketEntities as u32, msg.encode_to_vec())
}
fn setup() -> Vec<Vec<u8>> {
    let serializers = CsvcMsgFlattenedSerializer {
        symbols: vec!["SyntheticPawn".into(), "uint32".into(), "m_iHealth".into()],
        fields: vec![ProtoFlattenedSerializerFieldT {
            var_type_sym: Some(1),
            var_name_sym: Some(2),
            ..Default::default()
        }],
        serializers: vec![ProtoFlattenedSerializerT {
            serializer_name_sym: Some(0),
            serializer_version: Some(0),
            fields_index: vec![0],
        }],
    }
    .encode_to_vec();
    let mut prefixed = varint(serializers.len() as u32);
    prefixed.extend(serializers);
    let sendtables = CDemoSendTables {
        data: Some(prefixed),
    };
    let classes = CDemoClassInfo {
        classes: vec![c_demo_class_info::ClassT {
            class_id: Some(0),
            network_name: Some("SyntheticPawn".into()),
            table_name: Some("SyntheticPawn".into()),
        }],
    };
    let mut baseline = Bits::default();
    field(&mut baseline, None);
    let tables = CDemoStringTables {
        tables: vec![c_demo_string_tables::TableT {
            table_name: Some("instancebaseline".into()),
            items: vec![c_demo_string_tables::ItemsT {
                str: Some("0".into()),
                data: Some(baseline.bytes),
            }],
            ..Default::default()
        }],
    };
    let full = CDemoFullPacket {
        string_table: Some(tables),
        packet: None,
    };
    vec![
        command(
            EDemoCommands::DemSendTables,
            -1,
            &sendtables.encode_to_vec(),
            false,
        ),
        command(
            EDemoCommands::DemClassInfo,
            -1,
            &classes.encode_to_vec(),
            false,
        ),
        packet(
            -1,
            &[(
                SvcMessages::SvcCreateStringTable as u32,
                CsvcMsgCreateStringTable {
                    name: Some("instancebaseline".into()),
                    num_entries: Some(0),
                    string_data: Some(vec![]),
                    ..Default::default()
                }
                .encode_to_vec(),
            )],
        ),
        command(
            EDemoCommands::DemFullPacket,
            0,
            &full.encode_to_vec(),
            false,
        ),
        server(0, Some(0.02)),
    ]
}
fn entity_request() -> ReplayRequest {
    let mut r = request();
    r.selection.entity_classes = vec!["SyntheticPawn".into()];
    r.selection.sample_every_ticks = 1;
    r
}
#[test]
fn actual_entity_create_update_leave_recreate_delete_mapping() {
    let mut commands = setup();
    for (tick, action, health) in [
        (1, 2, Some(123)),
        (2, 0, Some(0)),
        (3, 1, None),
        (4, 2, Some(55)),
        (5, 3, None),
    ] {
        commands.push(packet(tick, &[entity_message(action, health)]));
    }
    let report = decode(&container(&commands, false), &entity_request()).unwrap();
    let entities: Vec<_> = report
        .observations
        .iter()
        .filter(|o| matches!(o.event, ObservationKind::EntityState { .. }))
        .collect();
    assert_eq!(entities.len(), 5);
    let actions: Vec<_> = entities
        .iter()
        .map(|o| match &o.event {
            ObservationKind::EntityState { action, .. } => *action,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        actions,
        vec![
            EntityAction::Create,
            EntityAction::Update,
            EntityAction::LeaveVisibility,
            EntityAction::Create,
            EntityAction::Delete
        ]
    );
    for (n, o) in entities.iter().enumerate() {
        let Observed::Known { value } = &o.entity else {
            panic!("missing observed entity");
        };
        assert_eq!(value.network_index, 4);
        assert_eq!(value.network_serial, None);
        assert_eq!(value.creation_ordinal, if n < 3 { 1 } else { 2 });
        assert_eq!(value.class_name, "SyntheticPawn");
        assert_eq!(o.raw.entity_callback_index, Some(0));
    }
    let ObservationKind::EntityState { fields, .. } = &entities[1].event else {
        unreachable!()
    };
    assert_eq!(
        fields["m_iHealth"].value,
        Observed::known(ReplayScalar::Unsigned(0))
    );
    assert!(matches!(
        fields["m_iTeamNum"].value,
        Observed::Unknown { .. }
    ));
    assert!(matches!(fields["m_iHealth"].unit, Observed::Unknown { .. }));
    assert!(!report.coaching_eligible);
}
#[test]
fn entity_sampling_keeps_lifecycle_and_does_not_reset_missing_fields_to_zero() {
    let mut commands = setup();
    commands.push(packet(1, &[entity_message(2, Some(7))]));
    commands.push(packet(2, &[entity_message(0, Some(11))]));
    commands.push(packet(121, &[entity_message(0, None)]));
    commands.push(packet(122, &[entity_message(3, None)]));
    let mut r = entity_request();
    r.selection.sample_every_ticks = 120;
    let report = decode(&container(&commands, false), &r).unwrap();
    let entities: Vec<_> = report
        .observations
        .iter()
        .filter(|o| matches!(o.event, ObservationKind::EntityState { .. }))
        .collect();
    assert_eq!(entities.len(), 3);
    assert_eq!(entities[1].time.tick, 121);
    let ObservationKind::EntityState { fields, .. } = &entities[1].event else {
        unreachable!()
    };
    assert_eq!(
        fields["m_iHealth"].value,
        Observed::known(ReplayScalar::Unsigned(11))
    );
}
#[test]
fn full_entity_reparse_is_identical_across_worker_processes() {
    let mut commands = setup();
    commands.push(packet(1, &[entity_message(2, Some(99))]));
    let bytes = container(&commands, true);
    assert_eq!(
        decode(&bytes, &entity_request()).unwrap(),
        decode(&bytes, &entity_request()).unwrap()
    );
}
#[test]
fn entity_index_budget_is_enforced() {
    let mut commands = setup();
    commands.push(packet(1, &[entity_message(2, Some(99))]));
    let mut r = entity_request();
    r.budget.max_entities = 2;
    assert_eq!(
        decode(&container(&commands, false), &r),
        Err(ReplayFailure::BudgetExceeded)
    );
}
