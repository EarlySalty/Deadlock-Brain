use serde_json::json;
use std::{fs, os::unix::fs::PermissionsExt, process::Command};

#[test]
fn match_import_rejects_ambient_pgoptions_before_a_database_connection() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("match.json");
    let schema = dbrain_sources::schema_watch::OpenApiSnapshot::pinned().unwrap();
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "socket_dir":dir.path().join("absent-private-socket"), "port":55449,
            "database":"fixture", "username":"fixture", "owner":"fixture",
            "account_id":"12345", "match_id":"12345", "base_release_id":"base",
            "release_id":"new", "created_at_epoch":1, "schema_sha256":schema.schema_sha256
        }))
        .unwrap(),
    )
    .unwrap();
    fs::set_permissions(&config, fs::Permissions::from_mode(0o600)).unwrap();
    for name in ["PGOPTIONS", "PGPASSWORD", "DATABASE_URL", "PGSSLKEY"] {
        let output = Command::new(env!("CARGO_BIN_EXE_brain-match-ingest"))
            .env_clear()
            .env(name, "synthetic-ambient-rejected")
            .args(["revoke", "--config"])
            .arg(&config)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains("ambient database credentials are forbidden"),
            "{error}"
        );
        assert!(!error.contains("synthetic-ambient-rejected"));
        assert!(!error.contains("Postgres connection failed"));
    }
}
