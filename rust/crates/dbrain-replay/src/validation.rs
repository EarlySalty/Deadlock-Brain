//! C10 diagnostic orchestration, not a decoder, store, or gameplay engine.
//! Only aggregate metadata crosses stdout. Independent references and the real Store/Learning
//! path remain unverified. Never turn codec determinism into real-match or coaching approval.
use dbrain_replay::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Passed,
    Failed,
    Unverified,
}
#[derive(Serialize)]
struct Check {
    status: Status,
    failure: Option<ReplayFailure>,
}
impl Check {
    fn unverified() -> Self {
        Self {
            status: Status::Unverified,
            failure: None,
        }
    }
    fn test(ok: bool) -> Self {
        Self {
            status: if ok { Status::Passed } else { Status::Failed },
            failure: None,
        }
    }
    fn failure(reason: ReplayFailure) -> Self {
        Self {
            status: Status::Failed,
            failure: Some(reason),
        }
    }
}
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum Implementation {
    Supported,
    Unsupported,
    Unknown,
    ParserDependent,
}
#[derive(Serialize)]
struct FieldCoverage {
    implementation: Implementation,
    verification: Status,
    known: u64,
    unknown: u64,
    // Fixed reason names, never observed field values or user-supplied text.
    unknown_reasons: BTreeMap<String, u64>,
}
impl FieldCoverage {
    fn new(implementation: Implementation) -> Self {
        Self {
            implementation,
            verification: Status::Unverified,
            known: 0,
            unknown: 0,
            unknown_reasons: BTreeMap::new(),
        }
    }
    fn observe<T>(&mut self, value: &Observed<T>) {
        match value {
            Observed::Known { .. } => self.known += 1,
            Observed::Unknown { reason } => {
                self.unknown += 1;
                *self
                    .unknown_reasons
                    .entry(format!("{reason:?}"))
                    .or_default() += 1;
            }
        }
    }
}
#[derive(Serialize)]
struct ResourceProbe {
    budget: DecodeBudget,
    completed: bool,
    failure: Option<ReplayFailure>,
    // A limit setting is neither a CPU/RSS measurement nor evidence that the limit was hit.
    enforcement_verified: bool,
}
#[derive(Serialize)]
pub struct ValidationReport {
    format_version: &'static str,
    validator_revision: String,
    status: &'static str,
    pub technical_validation_passed: bool,
    real_match_verified: bool,
    integration_verified: bool,
    coaching_eligible: bool,
    contract_version: &'static str,
    parser_revision: String,
    schema_revision: &'static str,
    input_budget: DecodeBudget,
    raw_sha256: Option<String>,
    raw_bytes: Option<u64>,
    observation_count: Option<usize>,
    observations_sha256: Vec<String>,
    checks: BTreeMap<&'static str, Check>,
    fields: BTreeMap<&'static str, FieldCoverage>,
    resource_probes: BTreeMap<&'static str, ResourceProbe>,
}
const TECHNICAL_CHECKS: &[&str] = &[
    "first_decode",
    "second_decode",
    "determinism",
    "contract_roundtrip",
    "locator_bounds",
    "duplicate_classification",
    "selection_reparse",
    "truncated_copy",
];
const UNVERIFIED_CHECKS: &[&str] = &[
    "independent_field_reference",
    "common_contract_bridge",
    "postgres_store",
    "domain_learning_port",
    "raw_locator_reference",
    "cpu_limit_enforcement",
    "memory_limit_enforcement",
    "parser_revision_reparse",
    "durable_duplicate",
    "durable_quarantine",
];
// The existing decoder's selected network paths. This list is report-only, not an extractor.
const NETWORK_FIELDS: &[&str] = &[
    "m_iTeamNum",
    "m_iHealth",
    "m_iMaxHealth",
    "m_hPawn",
    "m_CCitadelHeroComponent.m_spawnedHero.m_nHeroID",
    "m_CCitadelHeroComponent.m_loadingHero.m_nHeroID",
    "CBodyComponent.m_cellX",
    "CBodyComponent.m_cellY",
    "CBodyComponent.m_cellZ",
    "CBodyComponent.m_vecX",
    "CBodyComponent.m_vecY",
    "CBodyComponent.m_vecZ",
];
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn observation_digest(report: &ReplayReport) -> Result<String, ReplayFailure> {
    serde_json::to_vec(&report.observations)
        .map(|v| digest(&v))
        .map_err(|_| ReplayFailure::InvalidWorkerOutput)
}
impl ValidationReport {
    fn new(request: &ReplayRequest) -> Self {
        use Implementation::*;
        let mut fields = BTreeMap::new();
        for (name, implementation) in [
            ("container.header", Supported),
            ("compression.snappy", Supported),
            ("header.patch_version", ParserDependent),
            ("header.build_number", ParserDependent),
            ("header.game_directory", ParserDependent),
            ("file_info.playback_ticks", ParserDependent),
            ("file_info.playback_seconds", ParserDependent),
            ("time.tick", Supported),
            ("time.tick_interval_seconds", ParserDependent),
            ("time.game_time_seconds", Unknown),
            ("entity.network_index", ParserDependent),
            ("entity.creation_ordinal", ParserDependent),
            ("entity.network_serial", Unknown),
            ("entity.class", ParserDependent),
            ("entity.lifecycle", ParserDependent),
            ("entity.units", Unknown),
            ("player_mapping", Unsupported),
            ("hero_mapping", Unsupported),
            ("events.kill_death_assist", Unsupported),
            ("events.item_purchase", Unsupported),
            ("events.ability_upgrade", Unsupported),
            ("events.objective", Unsupported),
            ("events.opaque_packet", ParserDependent),
            ("raw_locator", Supported),
        ] {
            fields.insert(name, FieldCoverage::new(implementation));
        }
        for path in NETWORK_FIELDS {
            fields.insert(path, FieldCoverage::new(ParserDependent));
        }
        Self {
            format_version: "brain.replay.validation.v1",
            validator_revision: digest(
                concat!(include_str!("validation.rs"), include_str!("main.rs")).as_bytes(),
            ),
            status: "blocked",
            technical_validation_passed: false,
            real_match_verified: false,
            integration_verified: false,
            coaching_eligible: false,
            contract_version: CONTRACT_VERSION,
            parser_revision: parser_revision(),
            schema_revision: SCHEMA_REVISION,
            input_budget: request.budget.clone(),
            raw_sha256: None,
            raw_bytes: None,
            observation_count: None,
            observations_sha256: vec![],
            checks: TECHNICAL_CHECKS
                .iter()
                .chain(UNVERIFIED_CHECKS)
                .map(|&name| (name, Check::unverified()))
                .collect(),
            fields,
            resource_probes: BTreeMap::new(),
        }
    }
    fn known(&mut self, field: &'static str) {
        self.fields.get_mut(field).expect("fixed field").known += 1;
    }
    fn observe<T>(&mut self, field: &'static str, value: &Observed<T>) {
        self.fields
            .get_mut(field)
            .expect("fixed field")
            .observe(value);
    }
    fn coverage(&mut self, report: &ReplayReport) {
        self.known("container.header");
        for o in &report.observations {
            self.known("time.tick");
            self.known("raw_locator");
            self.observe("time.tick_interval_seconds", &o.time.tick_interval_seconds);
            self.observe("time.game_time_seconds", &o.time.game_time_seconds);
            if o.raw.compressed {
                self.known("compression.snappy");
            }
            if let Observed::Known { value } = &o.entity {
                for field in [
                    "entity.network_index",
                    "entity.creation_ordinal",
                    "entity.class",
                ] {
                    self.known(field);
                }
                if value.network_serial.is_some() {
                    self.known("entity.network_serial");
                } else {
                    self.observe::<u32>(
                        "entity.network_serial",
                        &Observed::unknown(UnknownReason::SerialNotExposed),
                    );
                }
            }
            match &o.event {
                ObservationKind::FileHeader {
                    patch_version,
                    build_number,
                    game_directory,
                } => {
                    self.observe("header.patch_version", patch_version);
                    self.observe("header.build_number", build_number);
                    self.observe("header.game_directory", game_directory);
                }
                ObservationKind::FileInfo {
                    playback_ticks,
                    playback_seconds,
                } => {
                    self.observe("file_info.playback_ticks", playback_ticks);
                    self.observe("file_info.playback_seconds", playback_seconds);
                }
                ObservationKind::EntityState { fields, .. } => {
                    self.known("entity.lifecycle");
                    for path in NETWORK_FIELDS {
                        if let Some(field) = fields.get(*path) {
                            self.observe(path, &field.value);
                            self.observe("entity.units", &field.unit);
                        }
                    }
                }
                ObservationKind::UnknownPacket { .. } => self.known("events.opaque_packet"),
                ObservationKind::ServerInfo { .. } => (),
            }
        }
    }
}

/// Both the replay and its authorization manifest must be regular local files outside Git.
/// The canonical and lexical ancestor checks also reject worktrees and symlinked directories.
/// This is a guardrail, not a substitute for the operator's actual authorization/retention policy.
pub fn private_input_path(path: &Path) -> Result<PathBuf, ReplayFailure> {
    let canonical = outside_git(path)?;
    let metadata = path
        .symlink_metadata()
        .map_err(|_| ReplayFailure::InputIo)?;
    if !metadata.is_file() {
        return Err(ReplayFailure::InputIo);
    }
    Ok(canonical)
}
fn outside_git(path: &Path) -> Result<PathBuf, ReplayFailure> {
    let canonical = path.canonicalize().map_err(|_| ReplayFailure::InputIo)?;
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()
            .map_err(|_| ReplayFailure::InputIo)?
            .join(path)
    };
    if absolute
        .ancestors()
        .chain(canonical.ancestors())
        .any(|p| p.join(".git").symlink_metadata().is_ok())
    {
        return Err(ReplayFailure::InvalidRequest);
    }
    Ok(canonical)
}

pub fn validate(
    raw: &Path,
    request: &ReplayRequest,
    decoder: &WorkerDecoder,
) -> Result<ValidationReport, ReplayFailure> {
    validate_request(request)?;
    // A second process must decode exactly the operator-pinned artifact, not a mutable pathname.
    if request.source.expected_sha256.is_none() {
        return Err(ReplayFailure::InvalidRequest);
    }
    // Worker snapshots and the damaged derivative use std::env::temp_dir(). Reject a
    // caller's TMPDIR inside any checkout BEFORE the existing supervisor copies raw bytes.
    outside_git(&std::env::temp_dir())?;
    let mut result = ValidationReport::new(request);
    let first = match decoder.decode(raw, request) {
        Ok(r) => {
            result.checks.insert("first_decode", Check::test(true));
            r
        }
        Err(e) => {
            result.checks.insert("first_decode", Check::failure(e));
            return Ok(result);
        }
    };
    result.raw_sha256 = Some(first.artifact.sha256.clone());
    result.raw_bytes = Some(first.artifact.byte_length);
    result.observation_count = Some(first.observations.len());
    result.observations_sha256.push(observation_digest(&first)?);
    result.coverage(&first);
    let encoded = serde_json::to_vec(&first).map_err(|_| ReplayFailure::InvalidWorkerOutput)?;
    let roundtrip = serde_json::from_slice::<ReplayReport>(&encoded).is_ok_and(|r| r == first);
    result
        .checks
        .insert("contract_roundtrip", Check::test(roundtrip));
    // Bounds are checked independently here; byte-level / state-prefix reference resolution
    // is a different check and stays UNVERIFIED rather than being inferred from these bounds.
    let bounds = !first.observations.is_empty()
        && first.observations.iter().all(|o| {
            o.raw.file_byte_offset >= 16
                && o.raw.stored_byte_length > 0
                && o.raw
                    .file_byte_offset
                    .checked_add(o.raw.stored_byte_length)
                    .is_some_and(|n| n <= first.artifact.byte_length)
                && o.raw.command_index < first.commands
        });
    result.checks.insert("locator_bounds", Check::test(bounds));
    // Each call goes through WorkerDecoder::decode and spawns a fresh sandboxed OS process.
    match decoder.decode(raw, request) {
        Ok(second) => {
            result.checks.insert("second_decode", Check::test(true));
            result
                .observations_sha256
                .push(observation_digest(&second)?);
            result.checks.insert(
                "determinism",
                Check::test(
                    first == second && serde_json::to_vec(&second).ok().as_ref() == Some(&encoded),
                ),
            );
            result.checks.insert(
                "duplicate_classification",
                Check::test(
                    classify_replay_duplicate(
                        &ReplayDedupKeys::from_report(&second),
                        &ReplayDedupKeys::from_report(&first),
                    ) == ObservationCommit::DuplicateGeneration,
                ),
            );
        }
        Err(e) => {
            result.checks.insert("second_decode", Check::failure(e));
        }
    }
    let mut reparse = request.clone();
    reparse.selection.sample_every_ticks = reparse
        .selection
        .sample_every_ticks
        .checked_add(1)
        .unwrap_or(1);
    match decoder.decode(raw, &reparse) {
        Ok(changed) => {
            result.checks.insert(
                "selection_reparse",
                Check::test(
                    first.generation_id != changed.generation_id
                        && classify_replay_duplicate(
                            &ReplayDedupKeys::from_report(&changed),
                            &ReplayDedupKeys::from_report(&first),
                        ) == ObservationCommit::Reparsed,
                ),
            );
        }
        Err(e) => {
            result.checks.insert("selection_reparse", Check::failure(e));
        }
    }
    // Only an eight-byte header prefix from the authorized original. Never overwrite it.
    // Pin the derivative's own hash: a HashMismatch would NOT test corruption handling.
    let mut prefix = [0u8; 8];
    File::open(raw)
        .and_then(|mut f| f.read_exact(&mut prefix))
        .map_err(|_| ReplayFailure::InputIo)?;
    let mut damaged = tempfile::NamedTempFile::new().map_err(|_| ReplayFailure::InputIo)?;
    damaged
        .write_all(&prefix)
        .map_err(|_| ReplayFailure::InputIo)?;
    damaged.flush().map_err(|_| ReplayFailure::InputIo)?;
    let mut corrupt_request = request.clone();
    corrupt_request.source.expected_sha256 = Some(digest(&prefix));
    corrupt_request.source.raw_object_ref = "private:c10-truncated-header-derivative".into();
    let damaged_outcome = decoder.decode(damaged.path(), &corrupt_request);
    result.checks.insert(
        "truncated_copy",
        match damaged_outcome {
            Err(ReplayFailure::DamagedReplay) => Check::test(true),
            Err(e) => Check::failure(e),
            Ok(_) => Check::test(false),
        },
    );
    // No retries and no budget escalation. Outcomes are not guessed CPU or OOM causes.
    for kind in ["cpu", "memory"] {
        let mut probe = request.clone();
        if kind == "cpu" {
            probe.budget.cpu_seconds = 1;
        } else {
            probe.budget.memory_bytes = 32 * 1024 * 1024;
        }
        let outcome = decoder.decode(raw, &probe);
        result.resource_probes.insert(
            kind,
            ResourceProbe {
                budget: probe.budget,
                completed: outcome.is_ok(),
                failure: outcome.err(),
                enforcement_verified: false,
            },
        );
    }
    result.technical_validation_passed = TECHNICAL_CHECKS
        .iter()
        .all(|name| matches!(result.checks[name].status, Status::Passed));
    Ok(result)
}
