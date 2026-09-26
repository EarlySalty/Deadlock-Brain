//! Shared, typed replay boundary. No database, scheduler, provider, or gameplay inference.
//! A decoded observation is NOT an independently validated fact or coaching evidence.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

pub const CONTRACT_VERSION: &str = "brain.replay.v2";

use crate::source::{GameValidity, OriginArtifact};
pub use crate::value::{Observed, UnknownReason};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRights {
    /// Opaque reference to an actual authorization, not a claim that .dem is anonymous.
    pub authorization_ref: String,
    pub scope: String,
    pub local_processing_allowed: bool,
    pub raw_retention_allowed: bool,
    /// This decoder rejects egress/publication requests; separate reviewed policy is required.
    pub external_egress_allowed: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchReference {
    pub match_id: String,
    /// Evidence for the caller-supplied identity; not inferred from a filename.
    pub evidence_ref: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplaySource {
    pub source_id: String,
    pub source_revision: String,
    /// Opaque access-controlled object reference. Never used to fetch a URL.
    pub raw_object_ref: String,
    pub retrieved_at_unix_ms: Option<i64>,
    pub expected_sha256: Option<String>,
    pub match_reference: Option<MatchReference>,
    pub rights: ReplayRights,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayArtifact {
    pub source: ReplaySource,
    pub sha256: String,
    pub byte_length: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecodeBudget {
    pub max_file_bytes: u64,
    pub max_command_bytes: u64,
    pub max_total_decoded_bytes: u64,
    pub max_commands: u64,
    pub max_packets: u64,
    pub max_entities: u32,
    pub max_observations: u32,
    pub max_output_bytes: u64,
    pub memory_bytes: u64,
    pub cpu_seconds: u64,
    pub wall_time_ms: u64,
}
impl Default for DecodeBudget {
    fn default() -> Self {
        Self {
            max_file_bytes: 512 * 1024 * 1024,
            max_command_bytes: 2 * 1024 * 1024,
            max_total_decoded_bytes: 2 * 1024 * 1024 * 1024,
            max_commands: 2_000_000,
            max_packets: 10_000_000,
            max_entities: 16_384,
            max_observations: 100_000,
            max_output_bytes: 32 * 1024 * 1024,
            memory_bytes: 768 * 1024 * 1024,
            cpu_seconds: 120,
            wall_time_ms: 180_000,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplaySelection {
    /// Sampling in ticks, deliberately not advertised as seconds or game time.
    pub sample_every_ticks: u32,
    pub entity_classes: Vec<String>,
}
impl Default for ReplaySelection {
    fn default() -> Self {
        Self {
            sample_every_ticks: 120,
            entity_classes: vec![
                "CCitadelPlayerPawn".into(),
                "CCitadelPlayerController".into(),
                "CCitadelGameRulesProxy".into(),
            ],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRequest {
    pub source: ReplaySource,
    pub budget: DecodeBudget,
    pub selection: ReplaySelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLocator {
    pub command_index: u64,
    pub file_byte_offset: u64,
    pub stored_byte_length: u64,
    pub compressed: bool,
    /// Hash of the decoded command, or of the specific network-message body.
    pub payload_sha256: String,
    /// Ordinal WITHIN the decoded command, not a fabricated byte offset in compressed bytes.
    pub packet_index: Option<u32>,
    pub entity_callback_index: Option<u32>,
    /// Entity snapshots depend on baselines and prior deltas. Replay the raw prefix from byte 0
    /// through this command; the triggering packet alone is NOT proof of every retained field.
    pub requires_state_prefix: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayTime {
    /// Initialization or missing ticks are explicit Unknown, never tick zero.
    pub tick: Observed<u32>,
    pub tick_interval_seconds: Observed<f32>,
    /// Only a witnessed game-clock anchor could establish game time. No fixed 60 Hz assumption.
    pub game_time_seconds: Observed<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayEntity {
    pub network_index: i32,
    /// Local CREATE ordinal; NEVER represented as a network serial or player identity.
    pub creation_ordinal: u32,
    pub network_serial: Option<u32>,
    pub class_name: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityAction {
    Create,
    Update,
    LeaveVisibility,
    Delete,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ReplayScalar {
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Boolean(bool),
    Vector3([f32; 3]),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayField {
    pub value: Observed<ReplayScalar>,
    pub unit: Observed<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ObservationKind {
    FileHeader {
        patch_version: Observed<i32>,
        build_number: Observed<i32>,
        game_directory: Observed<String>,
    },
    FileInfo {
        playback_ticks: Observed<i32>,
        playback_seconds: Observed<f32>,
    },
    ServerInfo {
        tick_interval_seconds: Observed<f32>,
    },
    EntityState {
        action: EntityAction,
        /// Exact network property paths, not inferred kills, damage, item purchases, or hero names.
        fields: BTreeMap<String, ReplayField>,
    },
    UnknownPacket {
        message_type: u32,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayObservation {
    pub observation_id: String,
    pub time: ReplayTime,
    pub entity: Observed<ReplayEntity>,
    pub raw: RawLocator,
    pub event: ObservationKind,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCapability {
    pub name: String,
    pub implementation: String,
    pub synthetic_tested: bool,
    pub real_replay_verified: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayReport {
    #[serde(deserialize_with = "deserialize_version")]
    pub contract_version: String,
    pub artifact: ReplayArtifact,
    pub parser_revision: String,
    pub schema_revision: String,
    pub extraction_revision: String,
    pub validity: GameValidity,
    /// Network entity key -> canonical entity identity, only when independently mapped.
    pub entity_mapping: BTreeMap<String, Observed<String>>,
    pub selection: ReplaySelection,
    pub generation_id: String,
    pub commands: u64,
    pub packets: u64,
    pub total_decoded_bytes: u64,
    pub observations: Vec<ReplayObservation>,
    pub capabilities: Vec<ReplayCapability>,
    pub real_replay_verified: bool,
    pub coaching_eligible: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFailure {
    InvalidRequest,
    RightsDenied,
    InputIo,
    HashMismatch,
    UnsupportedPlatform,
    InvalidContainer,
    DamagedReplay,
    UnknownStructure,
    BudgetExceeded,
    ParserFailure,
    ParserPanic,
    WorkerFailure,
    WorkerTimeout,
    SandboxFailure,
    InvalidWorkerOutput,
    StoreFailure,
}
impl std::fmt::Display for ReplayFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ReplayFailure {}

/// The only application-facing decoder port. Implementations must isolate unsafe upstream code.
pub trait ReplayDecoder: Send + Sync {
    fn decode(
        &self,
        raw_file: &Path,
        request: &ReplayRequest,
    ) -> Result<ReplayReport, ReplayFailure>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayDedupKeys {
    pub scope: String,
    pub raw_sha256: String,
    pub generation_id: String,
    pub match_id: Option<String>,
    pub parser_revision: String,
    pub schema_revision: String,
    pub extraction_revision: String,
    pub selection: ReplaySelection,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationCommit {
    Inserted,
    DuplicateGeneration,
    DuplicateMatch,
    Reparsed,
}

impl ReplayDedupKeys {
    pub fn from_report(report: &ReplayReport) -> Self {
        Self {
            scope: report.artifact.source.rights.scope.clone(),
            raw_sha256: report.artifact.sha256.clone(),
            generation_id: report.generation_id.clone(),
            match_id: report
                .artifact
                .source
                .match_reference
                .as_ref()
                .map(|m| m.match_id.clone()),
            parser_revision: report.parser_revision.clone(),
            schema_revision: report.schema_revision.clone(),
            extraction_revision: report.extraction_revision.clone(),
            selection: report.selection.clone(),
        }
    }
}
/// Pure duplicate decision shared with the transactional store. Call while holding that store's
/// match/scope lock. DuplicateMatch retains both raw provenances but MUST NOT double the sample.
/// A different scope is NEVER made visible by a duplicate. Reparsed replaces one complete active
/// generation, while preserving immutable prior generations and raw inputs under their own ACLs.
pub fn classify_replay_duplicate(
    incoming: &ReplayDedupKeys,
    existing: &ReplayDedupKeys,
) -> ObservationCommit {
    if incoming.scope != existing.scope {
        return ObservationCommit::Inserted;
    }
    if incoming.generation_id == existing.generation_id {
        return ObservationCommit::DuplicateGeneration;
    }
    let same_match = incoming
        .match_id
        .as_ref()
        .zip(existing.match_id.as_ref())
        .is_some_and(|(a, b)| a == b);
    let same_decoder = incoming.parser_revision == existing.parser_revision
        && incoming.schema_revision == existing.schema_revision
        && incoming.extraction_revision == existing.extraction_revision
        && incoming.selection == existing.selection;
    if same_match && same_decoder {
        return ObservationCommit::DuplicateMatch;
    }
    if incoming.raw_sha256 == existing.raw_sha256 || same_match {
        return ObservationCommit::Reparsed;
    }
    ObservationCommit::Inserted
}
/// S03 implements this against the ONE shared store, inside a transaction.
/// Exact generation is idempotent; a new parser generation never appends to the old one.
/// Match deduplication MUST retain access scope and provenance and MUST NOT expand visibility.
/// Unverified reports remain internal observations, never published facts/population/coaching.
pub trait ObservationStore {
    fn commit_replay(
        &mut self,
        keys: &ReplayDedupKeys,
        report: &ReplayReport,
    ) -> Result<ObservationCommit, ReplayFailure>;
    fn quarantine_replay(
        &mut self,
        source: &ReplaySource,
        reason: ReplayFailure,
    ) -> Result<(), ReplayFailure>;
}

/// Shared orchestration seam, intentionally no second queue/store/provider implementation.
pub fn decode_into_store(
    decoder: &impl ReplayDecoder,
    store: &mut impl ObservationStore,
    path: &Path,
    request: &ReplayRequest,
) -> Result<ObservationCommit, ReplayFailure> {
    match decoder.decode(path, request) {
        Ok(report) => {
            if report.coaching_eligible
                || report.real_replay_verified
                || report.contract_version != CONTRACT_VERSION
                || report.artifact.source != request.source
            {
                store.quarantine_replay(&request.source, ReplayFailure::InvalidWorkerOutput)?;
                return Err(ReplayFailure::InvalidWorkerOutput);
            }
            let keys = ReplayDedupKeys::from_report(&report);
            store.commit_replay(&keys, &report)
        }
        Err(reason) => {
            store.quarantine_replay(&request.source, reason)?;
            Err(reason)
        }
    }
}

fn deserialize_version<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let version = String::deserialize(deserializer)?;
    if version == CONTRACT_VERSION {
        Ok(version)
    } else {
        Err(serde::de::Error::custom(
            "unsupported replay version; v1 requires explicit migration",
        ))
    }
}

/// Source2 initialization sentinel is consumed HERE, never by downstream numeric consumers.
pub fn tick_from_legacy(tick: i32) -> Result<Observed<u32>, ReplayFailure> {
    match tick {
        -1 => Ok(Observed::unknown(UnknownReason::InitializationTick)),
        0.. => Ok(Observed::known(tick as u32)),
        _ => Err(ReplayFailure::InvalidWorkerOutput),
    }
}

/// Explicit read migration for archived brain.replay.v1 reports. This does not grant trust,
/// rewrite source/parser revisions, or authorize an old generation for new publication.
/// Only -1 means initialization; null/missing means NotPresent; all other negatives fail.
pub fn migrate_v1_report(bytes: &[u8]) -> Result<ReplayReport, serde_json::Error> {
    use serde_json::{json, Value};
    fn invalid(message: &str) -> serde_json::Error {
        <serde_json::Error as serde::de::Error>::custom(message)
    }
    let mut wire: Value = serde_json::from_slice(bytes)?;
    if wire["contract_version"] != "brain.replay.v1" {
        return Err(invalid("expected brain.replay.v1"));
    }
    if wire.get("validity").is_some() || wire.get("entity_mapping").is_some() {
        return Err(invalid("v1 cannot inject v2 fields"));
    }
    let observations = wire
        .get_mut("observations")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid("missing observations"))?;
    for observation in observations {
        let time = observation
            .get_mut("time")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| invalid("missing replay time"))?;
        let tick = match time.get("tick") {
            None | Some(Value::Null) => Observed::unknown(UnknownReason::NotPresent),
            Some(value) => {
                let tick = value
                    .as_i64()
                    .and_then(|n| i32::try_from(n).ok())
                    .ok_or_else(|| invalid("invalid v1 tick"))?;
                tick_from_legacy(tick).map_err(|_| invalid("invalid v1 negative tick"))?
            }
        };
        time.insert("tick".into(), serde_json::to_value(tick)?);
    }
    wire["contract_version"] = json!(CONTRACT_VERSION);
    wire["validity"] = serde_json::to_value(GameValidity::unknown())?;
    wire["entity_mapping"] = json!({});
    serde_json::from_value(wire)
}

/// Provenance for a detached observation. Never drop the report-level ACL/parser/raw identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationProvenance {
    pub observation_id: String,
    pub replay_id: String,
    pub generation_id: String,
    pub origin: OriginArtifact,
    pub schema_revision: String,
    pub extraction_revision: String,
    pub raw: RawLocator,
    pub canonical_entity: Observed<String>,
}
impl ReplayReport {
    pub fn origin_artifact(&self) -> OriginArtifact {
        use crate::source::*;
        let source = &self.artifact.source;
        OriginArtifact {
            identity: SourceIdentity {
                source_id: source.source_id.clone(),
                logical_id: source.raw_object_ref.clone(),
            },
            source_revision: SourceRevision::Replay {
                original_revision: source.source_revision.clone(),
            },
            raw_sha256: self.artifact.sha256.clone(),
            locator: source.raw_object_ref.clone(),
            parser_revision: self.parser_revision.clone(),
            parser_family: "dbrain-replay".into(),
            schema_version: Observed::known(self.schema_revision.clone()),
            schema_sha256: Observed::unknown(UnknownReason::NotPresent),
            retrieved_at: observed_option(
                source
                    .retrieved_at_unix_ms
                    .map(SourceTimestamp::UnixMilliseconds),
            ),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::unknown(UnknownReason::NotPresent),
            origin_artifacts: std::collections::BTreeSet::new(),
            derivation_family: Observed::known(self.extraction_revision.clone()),
            policy: SourcePolicy {
                visibility: crate::SourceVisibility::Internal,
                allowed_scopes: std::collections::BTreeSet::from([source.rights.scope.clone()]),
                authorization_ref: Observed::known(source.rights.authorization_ref.clone()),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: false,
                provider_egress_allowed: source.rights.external_egress_allowed,
                raw_retention_allowed: source.rights.raw_retention_allowed,
            },
            validity: self.validity.clone(),
        }
    }
    pub fn observation_provenance(&self, observation_id: &str) -> Option<ObservationProvenance> {
        let observation = self
            .observations
            .iter()
            .find(|o| o.observation_id == observation_id)?;
        let canonical_entity = match &observation.entity {
            Observed::Unknown { reason } => Observed::unknown(*reason),
            Observed::Known { value } => self
                .entity_mapping
                .get(&format!(
                    "{}:{}",
                    value.network_index, value.creation_ordinal
                ))
                .cloned()
                .unwrap_or_else(|| Observed::unknown(UnknownReason::Unmapped)),
        };
        Some(ObservationProvenance {
            observation_id: observation.observation_id.clone(),
            replay_id: self.artifact.sha256.clone(),
            generation_id: self.generation_id.clone(),
            origin: self.origin_artifact(),
            schema_revision: self.schema_revision.clone(),
            extraction_revision: self.extraction_revision.clone(),
            raw: observation.raw.clone(),
            canonical_entity,
        })
    }
}
