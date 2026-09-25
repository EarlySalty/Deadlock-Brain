#![forbid(unsafe_code)]
//! Local-only Source 2 replay decoding through a resource-limited Rust child process.
//! No downloader, external service, population writer, or coaching inference exists here.

pub use deadlock_brain_core::replay::*;
use sha2::{Digest, Sha256};

mod decode;
mod frame;
mod stringtable;
mod supervisor;
pub use supervisor::{WorkerDecoder, worker_stdio};

pub const HASTE_REVISION: &str = "bfb292d4798031350861ad297aa26753267a1ea6";
pub const SCHEMA_REVISION: &str = "4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6";
pub const DUNGERS_REVISION: &str = "5e1e2aac76a027987911de3ef3d23ecfd992a7fb";
pub const EXTRACTION_REVISION: &str = "network-observations/v1";

pub(crate) fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn hash_parts(parts: &[&[u8]]) -> String {
    let mut digest = Sha256::new();
    for part in parts {
        digest.update((part.len() as u64).to_le_bytes());
        digest.update(part);
    }
    format!("{:x}", digest.finalize())
}
/// Fingerprint actual compiled adapter source AND lockfile, not a mutable Git HEAD or README.
pub fn parser_revision() -> String {
    format!(
        "haste_core@{HASTE_REVISION}/adapter@{}",
        hash_parts(&[
            include_bytes!("lib.rs"),
            include_bytes!("frame.rs"),
            include_bytes!("decode.rs"),
            include_bytes!("supervisor.rs"),
            include_bytes!("stringtable.rs"),
            include_bytes!("../../../Cargo.lock"),
            include_bytes!("../Cargo.toml"),
            include_bytes!("../../../Cargo.toml"),
            include_bytes!("../../deadlock-brain-core/src/replay.rs"),
        ])
    )
}
pub(crate) fn generation_id(
    artifact: &ReplayArtifact,
    selection: &ReplaySelection,
) -> Result<String, ReplayFailure> {
    let bytes = serde_json::to_vec(selection).map_err(|_| ReplayFailure::InvalidRequest)?;
    Ok(hash_parts(&[
        CONTRACT_VERSION.as_bytes(),
        artifact.sha256.as_bytes(),
        &artifact.byte_length.to_le_bytes(),
        parser_revision().as_bytes(),
        SCHEMA_REVISION.as_bytes(),
        EXTRACTION_REVISION.as_bytes(),
        &bytes,
    ]))
}
pub fn capabilities() -> Vec<ReplayCapability> {
    [
        ("source2_container_and_snappy", "bounded", true),
        (
            "tick_and_observed_server_interval",
            "observed_not_game_clock",
            true,
        ),
        (
            "entity_lifecycle_and_network_properties",
            "schema_selected_unverified",
            true,
        ),
        ("raw_locator_and_deterministic_reparse", "implemented", true),
        ("match_identity_from_demo", "unknown", false),
        (
            "hero_names_items_kda_objectives",
            "unknown_without_field_validation",
            false,
        ),
        ("position_world_units", "unknown_raw_components_only", false),
        (
            "game_clock_pause_and_pregame",
            "unknown_without_anchor",
            false,
        ),
        ("coaching", "prohibited", false),
    ]
    .into_iter()
    .map(
        |(name, implementation, synthetic_tested)| ReplayCapability {
            name: name.into(),
            implementation: implementation.into(),
            synthetic_tested,
            real_replay_verified: false,
        },
    )
    .collect()
}
fn valid_ref(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 256 && !s.chars().any(char::is_control)
}
pub fn validate_request(request: &ReplayRequest) -> Result<(), ReplayFailure> {
    let s = &request.source;
    if !s.rights.local_processing_allowed
        || !s.rights.raw_retention_allowed
        || s.rights.external_egress_allowed
        || !valid_ref(&s.rights.authorization_ref)
        || !valid_ref(&s.rights.scope)
    {
        return Err(ReplayFailure::RightsDenied);
    }
    if !valid_ref(&s.source_id)
        || !valid_ref(&s.source_revision)
        || !valid_ref(&s.raw_object_ref)
        || s.retrieved_at_unix_ms.is_some_and(|v| v < 0)
        || s.expected_sha256.as_ref().is_some_and(|h| {
            h.len() != 64
                || !h
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        })
        || s.match_reference
            .as_ref()
            .is_some_and(|m| !valid_ref(&m.match_id) || !valid_ref(&m.evidence_ref))
    {
        return Err(ReplayFailure::InvalidRequest);
    }
    let b = &request.budget;
    if b.max_file_bytes == 0
        || b.max_file_bytes > 1024 * 1024 * 1024
        || b.max_command_bytes == 0
        || b.max_command_bytes > 2 * 1024 * 1024
        || b.max_total_decoded_bytes == 0
        || b.max_total_decoded_bytes > 8 * 1024 * 1024 * 1024
        || b.max_commands == 0
        || b.max_commands > 5_000_000
        || b.max_packets == 0
        || b.max_packets > 20_000_000
        || b.max_entities == 0
        || b.max_entities > 16_384
        || b.max_observations == 0
        || b.max_observations > 200_000
        || b.max_output_bytes == 0
        || b.max_output_bytes > 64 * 1024 * 1024
        || b.memory_bytes < 32 * 1024 * 1024
        || b.memory_bytes > 2 * 1024 * 1024 * 1024
        || b.cpu_seconds == 0
        || b.cpu_seconds > 300
        || b.wall_time_ms == 0
        || b.wall_time_ms > 600_000
    {
        return Err(ReplayFailure::InvalidRequest);
    }
    let selection = &request.selection;
    if selection.sample_every_ticks == 0
        || selection.entity_classes.is_empty()
        || selection.entity_classes.len() > 16
        || selection.entity_classes.iter().any(|s| {
            s.is_empty()
                || s.len() > 128
                || !s.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
        })
    {
        return Err(ReplayFailure::InvalidRequest);
    }
    let mut classes = selection.entity_classes.clone();
    classes.sort();
    classes.dedup();
    if classes.len() != selection.entity_classes.len() {
        return Err(ReplayFailure::InvalidRequest);
    }
    Ok(())
}
