use serde_json::json;
use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

fn fixture(dir: &std::path::Path) -> std::path::PathBuf {
    let infisical = dir.join("infisical.json");
    fs::write(
        &infisical,
        serde_json::to_vec(&json!({
            "secret_values_fd":3, "project_id":"fixture", "environment":"fixture",
            "secret_path":"/fixture", "socket_path":"/never-contact-infisical"
        }))
        .unwrap(),
    )
    .unwrap();
    let endpoint = |database, secret| {
        json!({
            "socket":dir.join("absent-private-socket"), "port":55449,
            "database":database, "username":"fixture", "auth_secret":secret,
            "infisical_config": "infisical.json"
        })
    };
    let policy = json!({"visibility":"internal", "allowed_scopes":["fixture"],
        "provider_egress_allowed":false, "publication_allowed":false, "raw_retention_allowed":false});
    let config = dir.join("import.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "legacy":endpoint("brain_legacy_fixture", "BRAIN_PG_READONLY_PASSWORD"),
            "target":endpoint("brain_pilot_fixture", "BRAIN_PG_INGEST_PASSWORD"),
            "snapshot_label":"fixture", "snapshot_epoch":1, "owner":"fixture",
            "release":{"id_prefix":"fixture", "knowledge_version":"fixture", "patch":"fixture"},
            "sources":{"legacy-entities":policy, "legacy-patchnotes":policy},
            "report":dir.join("unused-report.json")
        }))
        .unwrap(),
    )
    .unwrap();
    config
}

fn launch(
    config: &std::path::Path,
    values: serde_json::Value,
    ambient: Option<&str>,
) -> std::process::Output {
    let mut command = Command::new("/bin/bash");
    command
        .env_clear()
        .args(["-c", "exec 3<&0; exec \"$@\"", "fixture"])
        .arg(env!("CARGO_BIN_EXE_brain-legacy-import"))
        .arg("--config")
        .arg(config)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(name) = ambient {
        command.env(name, "synthetic-ambient-rejected");
    }
    let mut child = command.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(values.to_string().as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn both_endpoint_references_use_one_private_pipe_snapshot_before_any_database_connection() {
    let dir = tempfile::tempdir().unwrap();
    let config = fixture(dir.path());
    let values = json!({"BRAIN_PG_READONLY_PASSWORD":"synthetic-readonly-password",
        "BRAIN_PG_INGEST_PASSWORD":"synthetic-ingest-password"});
    let output = launch(&config, values, None);
    assert!(!output.status.success());
    // Both endpoint options are prepared before the deliberate nonexistent socket
    // is contacted. Reading the shared one-shot pipe twice fails before this point.
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("legacy connection failed"), "{error}");
    assert!(!error.contains("synthetic-readonly-password"));
    assert!(!error.contains("synthetic-ingest-password"));
    assert!(!dir.path().join("unused-report.json").exists());
    let output = launch(
        &config,
        json!({"BRAIN_PG_READONLY_PASSWORD":"synthetic-readonly-password"}),
        None,
    );
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("configured Infisical secret missing"));
    assert!(!error.contains("legacy connection failed"));
}

#[test]
fn ambient_postgres_startup_options_and_credentials_are_rejected_before_secret_loading() {
    let dir = tempfile::tempdir().unwrap();
    let config = fixture(dir.path());
    for name in ["PGOPTIONS", "PGPASSWORD", "PGSSLKEY"] {
        let output = launch(&config, json!({}), Some(name));
        assert!(!output.status.success());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("PostgreSQL environment options are not supported"));
        assert!(!error.contains("synthetic-ambient-rejected"));
    }
}
