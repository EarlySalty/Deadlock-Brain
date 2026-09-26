mod common;
use common::*;
use dbrain_replay::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{io::Write, process::Command};

// Authored wire fixtures only. These tests are NOT a real-match acceptance.
fn invoke(bytes: &[u8], request: &ReplayRequest) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let mut raw = tempfile::NamedTempFile::new_in(dir.path()).unwrap();
    raw.write_all(bytes).unwrap();
    let mut config = tempfile::NamedTempFile::new_in(dir.path()).unwrap();
    serde_json::to_writer(&mut config, request).unwrap();
    Command::new(env!("CARGO_BIN_EXE_dbrain-replay-worker"))
        .args(["validate"])
        .arg(raw.path())
        .arg(config.path())
        .output()
        .unwrap()
}
fn pinned(bytes: &[u8]) -> ReplayRequest {
    let mut r = request();
    r.source.expected_sha256 = Some(format!("{:x}", Sha256::digest(bytes)));
    r.source.source_id = "PRIVATE-SOURCE-CANARY".into();
    r.source.source_revision = "PRIVATE-REVISION-CANARY".into();
    r.source.raw_object_ref = "PRIVATE-OBJECT-CANARY".into();
    r.source.rights.authorization_ref = "PRIVATE-AUTH-CANARY".into();
    r.source.rights.scope = "PRIVATE-SCOPE-CANARY".into();
    r.source.match_reference = Some(MatchReference {
        match_id: "PRIVATE-MATCH-CANARY".into(),
        evidence_ref: "PRIVATE-EVIDENCE-CANARY".into(),
    });
    r
}
fn report(output: &std::process::Output) -> Value {
    // The real-match/reference/Store/Learning gate MUST NOT become green on a codec success.
    assert_eq!(output.status.code(), Some(2));
    serde_json::from_slice(&output.stdout).expect("sanitized validation JSON")
}
#[test]
fn validation_compares_fresh_decodes_without_promoting_synthetic_data() {
    let bytes = container(&[server(0, Some(0.03125))], true);
    let output = invoke(&bytes, &pinned(&bytes));
    let v = report(&output);
    assert_eq!(v["status"], "blocked");
    assert_eq!(v["technical_validation_passed"], true);
    assert_eq!(v["real_match_verified"], false);
    assert_eq!(v["integration_verified"], false);
    assert_eq!(v["coaching_eligible"], false);
    for key in [
        "first_decode",
        "second_decode",
        "determinism",
        "contract_roundtrip",
        "locator_bounds",
        "duplicate_classification",
        "selection_reparse",
        "truncated_copy",
    ] {
        assert_eq!(v["checks"][key]["status"], "passed", "{key}");
    }
    for key in [
        "independent_field_reference",
        "common_contract_bridge",
        "postgres_store",
        "domain_learning_port",
        "raw_locator_reference",
        "cpu_limit_enforcement",
        "memory_limit_enforcement",
    ] {
        assert_eq!(v["checks"][key]["status"], "unverified", "{key}");
    }
    assert_eq!(v["observations_sha256"][0], v["observations_sha256"][1]);
    assert!(v["observations_sha256"][0].as_str().unwrap().len() == 64);
    assert_eq!(v["fields"]["header.build_number"]["known"], 1);
    assert_eq!(v["fields"]["time.tick"]["known"], 2);
    assert_eq!(v["fields"]["time.game_time_seconds"]["known"], 0);
    assert_eq!(v["fields"]["time.game_time_seconds"]["unknown"], 2);
    assert_eq!(v["fields"]["entity.network_index"]["known"], 0);
    assert_eq!(
        v["fields"]["player_mapping"]["implementation"],
        "unsupported"
    );
    assert_eq!(v["fields"]["hero_mapping"]["verification"], "unverified");
    assert!(v["resource_probes"]["cpu"]["budget"]["cpu_seconds"] == 1);
    assert_eq!(
        v["resource_probes"]["memory"]["budget"]["memory_bytes"],
        32 * 1024 * 1024
    );
    let public = String::from_utf8(output.stdout).unwrap();
    for private in [
        "PRIVATE-",
        "authorization_ref",
        "raw_object_ref",
        "match_id",
        "file_byte_offset",
        "game_directory\":\"citadel",
    ] {
        assert!(!public.contains(private), "leaked {private}");
    }
    assert!(output.stderr.is_empty());
}
#[test]
fn validation_requires_an_explicit_raw_hash_pin() {
    let out = invoke(&minimal(), &request());
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap().trim(),
        "replay_quarantined:InvalidRequest"
    );
}
#[test]
fn validation_rejects_rights_before_using_raw_input() {
    let mut r = pinned(&minimal());
    r.source.rights.local_processing_allowed = false;
    let out = invoke(&minimal(), &r);
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap().trim(),
        "replay_quarantined:RightsDenied"
    );
}
#[test]
fn failed_initial_decode_keeps_every_unrun_check_unverified() {
    let bytes = b"PBDEMS2\0";
    let out = invoke(bytes, &pinned(bytes));
    let v = report(&out);
    assert_eq!(v["technical_validation_passed"], false);
    assert_eq!(v["checks"]["first_decode"]["failure"], "damaged_replay");
    assert_eq!(v["checks"]["determinism"]["status"], "unverified");
    assert_eq!(v["checks"]["postgres_store"]["status"], "unverified");
    assert!(v["observations_sha256"].as_array().unwrap().is_empty());
    assert_eq!(v["fields"]["header.build_number"]["known"], 0);
}
#[test]
fn mismatched_hash_is_not_reported_as_a_successful_real_decode() {
    let mut r = pinned(&minimal());
    r.source.expected_sha256 = Some("0".repeat(64));
    let out = invoke(&minimal(), &r);
    let v = report(&out);
    assert_eq!(v["checks"]["first_decode"]["failure"], "hash_mismatch");
    assert_eq!(v["technical_validation_passed"], false);
}
#[test]
fn validation_refuses_raw_data_inside_a_git_checkout() {
    let dir = tempfile::tempdir().unwrap();
    // Worktrees use a .git file; ordinary checkouts use a directory. Neither is allowed.
    std::fs::write(dir.path().join(".git"), "gitdir: unused").unwrap();
    let raw = dir.path().join("raw");
    std::fs::write(&raw, minimal()).unwrap();
    let mut config = tempfile::NamedTempFile::new().unwrap();
    serde_json::to_writer(&mut config, &pinned(&minimal())).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_dbrain-replay-worker"))
        .arg("validate")
        .arg(raw)
        .arg(config.path())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap().trim(),
        "replay_quarantined:InvalidRequest"
    );
}
#[test]
fn validation_refuses_an_authorization_manifest_inside_git() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join(".git")).unwrap();
    let config = dir.path().join("request.json");
    std::fs::write(&config, serde_json::to_vec(&pinned(&minimal())).unwrap()).unwrap();
    let mut raw = tempfile::NamedTempFile::new().unwrap();
    raw.write_all(&minimal()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_dbrain-replay-worker"))
        .arg("validate")
        .arg(raw.path())
        .arg(config)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap().trim(),
        "replay_quarantined:InvalidRequest"
    );
}

#[test]
fn validation_refuses_temporary_replay_copies_inside_git() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir(repo.path().join(".git")).unwrap();
    let mut raw = tempfile::NamedTempFile::new().unwrap();
    raw.write_all(&minimal()).unwrap();
    let mut config = tempfile::NamedTempFile::new().unwrap();
    serde_json::to_writer(&mut config, &pinned(&minimal())).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_dbrain-replay-worker"))
        .arg("validate")
        .arg(raw.path())
        .arg(config.path())
        .env("TMPDIR", repo.path())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(
        out.stdout.is_empty(),
        "must reject before writing replay snapshots inside Git"
    );
    assert_eq!(
        String::from_utf8(out.stderr).unwrap().trim(),
        "replay_quarantined:InvalidRequest"
    );
    assert_eq!(std::fs::read_dir(repo.path()).unwrap().count(), 1);
}
