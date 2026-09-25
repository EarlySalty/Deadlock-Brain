use dbrain_s12_wiki_probe::knowledge::{compare_ir, extract, MappingProfile, ValueKind};
use serde_json::Value;
use std::{path::Path, process::Command};
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_dbrain-s12-wiki-probe"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env_remove("DATABASE_URL")
        .env_remove("DEADLOCK_CENTRAL_DSN")
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn extract_cli_returns_common_ir_without_publication() {
    let out = cli(&[
        "extract",
        "fixtures/pilot.capture.json",
        "fixtures/completion.mapping.json",
    ]);
    assert!(out.status.success());
    let ir: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(ir["version"], "wiki-ir-v1");
    assert_eq!(ir["report"]["production_publishable"], false);
    assert_eq!(
        ir["sources"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["logical_id"] == "synthetic_wiki:page:101")
            .count(),
        2
    );
}
#[test]
fn project_cli_is_byte_deterministic_and_matches_shared_contract_golden() {
    let args = [
        "project",
        "fixtures/pilot.capture.json",
        "fixtures/completion.mapping.json",
        "fixtures/completion-release.json",
        "fixtures/completion-review.json",
        "101",
        "de",
    ];
    let a = cli(&args);
    let b = cli(&args);
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
    assert_eq!(a.stdout, b.stdout);
    let card: Value = serde_json::from_slice(&a.stdout).unwrap();
    let golden: Value =
        serde_json::from_str(include_str!("../fixtures/completion-card.golden.json")).unwrap();
    assert_eq!(card, golden);
}
#[test]
fn projection_requires_a_review_and_a_real_bound_hero() {
    assert_eq!(
        cli(&[
            "project",
            "fixtures/pilot.capture.json",
            "fixtures/completion.mapping.json",
            "fixtures/completion-release.json",
            "fixtures/completion.mapping.json",
            "101",
            "de"
        ])
        .status
        .code(),
        Some(2)
    );
    assert_eq!(
        cli(&[
            "project",
            "fixtures/pilot.capture.json",
            "fixtures/completion.mapping.json",
            "fixtures/completion-release.json",
            "fixtures/completion-review.json",
            "999",
            "de"
        ])
        .status
        .code(),
        Some(2)
    );
}
#[test]
fn unchanged_ir_delta_cli_schedules_no_work() {
    let out = cli(&[
        "ir-delta",
        "fixtures/pilot.capture.json",
        "fixtures/completion.mapping.json",
        "fixtures/pilot.capture.json",
        "fixtures/completion.mapping.json",
    ]);
    assert!(out.status.success());
    let delta: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(delta["reproject_heroes"], serde_json::json!([]));
    assert_eq!(delta["embedding_jobs_scheduled"], 0);
}
#[test]
fn changing_one_field_mapping_only_reprojects_its_bound_hero() {
    let bytes = include_bytes!("../fixtures/pilot.capture.json");
    let mut profile: MappingProfile =
        serde_json::from_str(include_str!("../fixtures/completion.mapping.json")).unwrap();
    let a = extract(bytes, &profile).unwrap();
    profile.fields[0].kind = ValueKind::Text;
    let b = extract(bytes, &profile).unwrap();
    let delta = compare_ir(&a, &b).unwrap();
    assert_eq!(
        delta.reproject_heroes.into_iter().collect::<Vec<_>>(),
        vec!["synthetic_wiki:page:101"]
    );
    assert!(delta.reparse_pages.is_empty());
    assert!(delta.raw_content_changed.is_empty());
    assert_ne!(
        a.fields()
            .iter()
            .find(|f| f.predicate == "health")
            .unwrap()
            .id,
        b.fields()
            .iter()
            .find(|f| f.predicate == "health")
            .unwrap()
            .id
    );
}
#[test]
fn documentation_check_script_resolves_its_own_directory() {
    let script =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("check.sh")).unwrap();
    assert!(script.contains("${BASH_SOURCE[0]}"));
}
