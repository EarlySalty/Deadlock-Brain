use brain_contracts::{Principal, SnapshotReadPort};
use brain_storage::{LocalPgReader, PgStore};
use serde_json::{json, Value};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::BTreeSet, fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt, path::Path,
    process::Command,
};

fn endpoint(socket: &str, database: &str) -> Value {
    json!({
        "socket": socket,
        "port": 55439,
        "database": database,
        "username": "brain_core_test",
        "auth_env": null
    })
}

fn policy(visibility: &str, scopes: &[&str]) -> Value {
    json!({
        "visibility": visibility,
        "allowed_scopes": scopes,
        "provider_egress_allowed": false,
        "publication_allowed": false,
        "raw_retention_allowed": true
    })
}

fn import(config: &Path) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-legacy-import"))
        .env_clear()
        .arg("--config")
        .arg(config)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "import failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

async fn snapshot(socket: &str, release: &str) -> brain_contracts::CorpusSnapshot {
    let socket = socket.to_string();
    let release = release.to_string();
    tokio::task::spawn_blocking(move || {
        LocalPgReader::new(&socket, 55439, "brain_core_test", "brain_pilot_test")
            .unwrap()
            .read_snapshot(&release)
            .unwrap()
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "scripts/test_brain_serve.sh: synthetic legacy archive and target in scratch PostgreSQL"]
async fn cli_reads_archive_and_replays_tombstones_and_revokes() {
    let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
    assert!(socket.ends_with("/.core-test-pg") && Path::new(&socket).is_absolute());
    let scratch = Path::new(&socket).parent().unwrap();
    let legacy = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(&socket)
                .port(55439)
                .username("brain_core_test")
                .database("brain_legacy_test"),
        )
        .await
        .unwrap();
    for statement in [
        "CREATE SCHEMA brain_legacy",
        "CREATE TABLE brain_legacy.entities (id bigint PRIMARY KEY, entity_type text NOT NULL, canonical_name text NOT NULL, primary_external_id text, source text, metadata jsonb)",
        "CREATE TABLE brain_legacy.entity_aliases (id bigint PRIMARY KEY, entity_id bigint NOT NULL, alias text NOT NULL, alias_kind text)",
        "CREATE TABLE brain_legacy.patch_events (id bigint PRIMARY KEY, patch_external_id text, patch_title text, patch_url text, posted_at timestamptz, source_kind text, line_index bigint, section text, entity_name text, raw_line text, metadata jsonb)",
        "CREATE TABLE brain_legacy.patch_event_enrichments (id bigint PRIMARY KEY, patch_event_id bigint NOT NULL, stat_name text, old_value text, new_value text, unit text)",
        "INSERT INTO brain_legacy.entities VALUES (25, 'hero', 'Warden', 'hero_25', 'deadlock_assets_api', '{\"source_language\":\"en\"}')",
        "INSERT INTO brain_legacy.entity_aliases VALUES (1, 25, 'Guardian', 'canonical')",
        "INSERT INTO brain_legacy.patch_events VALUES (1, 'p1', 'Update p1', 'https://example.invalid/p1', to_timestamp(1700000000), 'steam', 0, NULL, 'Warden', 'Warden: first', '{\"source_language\":\"en\"}')",
        "INSERT INTO brain_legacy.patch_events VALUES (2, 'p2', 'Update p2', 'https://example.invalid/p2', to_timestamp(1700000001), 'steam', 0, NULL, 'Warden', 'Warden: second', '{\"source_language\":\"en\"}')",
        "INSERT INTO brain_legacy.patch_event_enrichments VALUES (1, 1, 'health', '500', '600', NULL)",
    ] {
        sqlx::query(statement).execute(&legacy).await.unwrap();
    }
    let target = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(&socket)
                .port(55439)
                .username("brain_core_test")
                .database("brain_pilot_test"),
        )
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&target)
        .await
        .unwrap();
    assert!(address.is_none());
    PgStore::new(target.clone()).migrate_core().await.unwrap();

    let config_path = scratch.join("legacy-cli-config.json");
    let report_path = scratch.join("legacy-cli-report.json");
    let mut config = json!({
        "legacy": endpoint(&socket, "brain_legacy_test"),
        "target": endpoint(&socket, "brain_pilot_test"),
        "snapshot_label": "synthetic-archive",
        "snapshot_epoch": 1700000002,
        "owner": "scratch-import",
        "release": {"id_prefix": "scratch", "knowledge_version": "fixture-v1", "patch": "p1"},
        "sources": {
            "legacy-entities": policy("public", &["game.public"]),
            "legacy-patchnotes": policy("public", &["game.public"])
        },
        "report": report_path
    });
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&config_path)
        .unwrap();
    file.write_all(config.to_string().as_bytes()).unwrap();
    drop(file);
    let first_path = config_path.clone();
    let first = tokio::task::spawn_blocking(move || import(&first_path))
        .await
        .unwrap();
    assert_eq!(first["release_documents"], 3);
    assert_eq!(first["legacy_table_counts"]["patch_events"], 2);
    assert_eq!(first["legacy_table_counts"]["entities"], 1);
    assert_eq!(first["sources"][0]["changed_records"], 1);
    assert_eq!(first["sources"][1]["changed_records"], 2);
    assert_eq!(
        serde_json::from_slice::<Value>(&std::fs::read(&report_path).unwrap()).unwrap(),
        first
    );
    let release = first["release_id"].as_str().unwrap().to_string();
    let initial = snapshot(&socket, &release).await;
    assert_eq!(initial.revisions.len(), 3);
    let game = Principal {
        actor_id: "scratch-game".into(),
        channel: "test".into(),
        scopes: BTreeSet::from(["game.public".into()]),
        provider_egress: BTreeSet::new(),
    };
    assert_eq!(initial.authorized(&game, false).unwrap().len(), 3);

    let replay_path = config_path.clone();
    let replay = tokio::task::spawn_blocking(move || import(&replay_path))
        .await
        .unwrap();
    assert_eq!(replay["release_id"], first["release_id"]);
    for source in replay["sources"].as_array().unwrap() {
        assert_eq!(source["changed_records"], 0);
        assert_eq!(source["tombstones"], 0);
    }
    sqlx::query("DELETE FROM brain_legacy.patch_events WHERE id=2")
        .execute(&legacy)
        .await
        .unwrap();
    let deleted_path = config_path.clone();
    let deleted = tokio::task::spawn_blocking(move || import(&deleted_path))
        .await
        .unwrap();
    assert_eq!(deleted["release_documents"], 2);
    assert_eq!(deleted["sources"][1]["tombstones"], 1);
    let tombstones: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.source_record_heads WHERE source_id='legacy-patchnotes' AND logical_id='patch/p2' AND tombstone")
        .fetch_one(&target)
        .await
        .unwrap();
    assert_eq!(tombstones, 1);
    assert_eq!(
        snapshot(&socket, &release)
            .await
            .authorized(&game, false)
            .unwrap()
            .len(),
        2
    );

    config["sources"]["legacy-entities"] = policy("private", &["brain.internal"]);
    std::fs::write(&config_path, config.to_string()).unwrap();
    let revoked_path = config_path.clone();
    let revoked = tokio::task::spawn_blocking(move || import(&revoked_path))
        .await
        .unwrap();
    assert_eq!(revoked["sources"][0]["changed_records"], 1);
    assert_eq!(
        snapshot(&socket, &release)
            .await
            .authorized(&game, false)
            .unwrap()
            .len(),
        1
    );
    let internal = Principal {
        scopes: BTreeSet::from(["brain.internal".into()]),
        ..game.clone()
    };
    assert_eq!(
        snapshot(&socket, revoked["release_id"].as_str().unwrap())
            .await
            .authorized(&internal, false)
            .unwrap()
            .len(),
        1
    );
    sqlx::query("DELETE FROM brain_legacy.patch_event_enrichments")
        .execute(&legacy)
        .await
        .unwrap();
    sqlx::query("DELETE FROM brain_legacy.patch_events")
        .execute(&legacy)
        .await
        .unwrap();
    let empty_path = config_path.clone();
    let empty = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_brain-legacy-import"))
            .env_clear()
            .arg("--config")
            .arg(empty_path)
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    assert!(!empty.status.success());
    assert!(String::from_utf8_lossy(&empty.stderr).contains("legacy import failed"));
    assert_eq!(
        serde_json::from_slice::<Value>(&std::fs::read(&report_path).unwrap()).unwrap(),
        revoked
    );
    assert_eq!(
        snapshot(&socket, &release)
            .await
            .authorized(&game, false)
            .unwrap()
            .len(),
        1
    );
    target.close().await;
    legacy.close().await;
}
