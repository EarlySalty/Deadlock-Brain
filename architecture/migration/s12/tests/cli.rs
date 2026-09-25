use dbrain_s12_wiki_probe::model::MAX_INPUT_BYTES;
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/pilot.capture.json")
}
fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_dbrain-s12-wiki-probe"));
    command.env_clear(); // No DB/provider/network configuration is needed or inherited.
    command
}
fn parse(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}
static COUNTER: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "s12-probe-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn analyze_works_without_runtime_environment_and_returns_nonacceptance() {
    let out = command().arg("analyze").arg(fixture()).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(parse(&out)["production_publishable"], false);
    assert_eq!(parse(&out)["mode"], "prepare_only");
}

#[test]
fn production_readiness_fails_even_on_a_valid_fixture() {
    let out = command()
        .arg("analyze")
        .arg(fixture())
        .arg("--require-production-ready")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(3));
    assert_eq!(parse(&out)["integration_verified"], false);
    assert!(parse(&out)["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|b| b == "G1_contracts_not_integrated"));
}

#[test]
fn unchanged_delta_is_deterministic_and_schedules_nothing() {
    let out = command()
        .arg("delta")
        .arg(fixture())
        .arg(fixture())
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(parse(&out)["reparse_pages"], serde_json::json!([]));
    assert_eq!(parse(&out)["embedding_jobs_scheduled"], 0);
    assert_eq!(parse(&out)["publication_performed"], false);
}

#[test]
fn invalid_arguments_cannot_activate_live_network_or_publishing() {
    for args in [
        vec!["--allow-wiki-network"],
        vec!["publish"],
        vec!["analyze"],
        vec!["delta", "one"],
    ] {
        let out = command().args(args).output().unwrap();
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
    }
}

#[test]
fn help_explains_that_zero_is_not_production_acceptance() {
    let out = command().arg("--help").output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("NOT production acceptance"));
}

#[test]
fn invalid_capture_errors_do_not_echo_untrusted_source_content() {
    let tmp = Scratch::new();
    let file = tmp.0.join("invalid.json");
    fs::write(&file, b"secret-marker-never-echo-invalid-json").unwrap();
    let out = command().arg("analyze").arg(file).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("secret-marker"));
}

#[test]
fn oversized_regular_input_is_rejected_before_parsing() {
    let tmp = Scratch::new();
    let file = tmp.0.join("large.json");
    fs::File::create(&file)
        .unwrap()
        .set_len(MAX_INPUT_BYTES as u64 + 1)
        .unwrap();
    let out = command().arg("analyze").arg(file).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
}

#[test]
fn directories_are_not_input_captures() {
    let tmp = Scratch::new();
    let out = command().arg("analyze").arg(&tmp.0).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[cfg(unix)]
#[test]
fn symlinks_are_not_followed_by_the_offline_cli() {
    let tmp = Scratch::new();
    let link = tmp.0.join("capture-link");
    std::os::unix::fs::symlink(fixture(), &link).unwrap();
    let out = command().arg("analyze").arg(link).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
}
