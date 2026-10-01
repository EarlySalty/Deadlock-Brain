//! Historical v1 data -> current v2, real dump/restore, privilege and failure probes.
//! Only scripts/test_brain_storage_upgrade.sh may provision this throwaway cluster.
use brain_contracts::{
    store::ConversationOwnershipPort, CorpusSnapshot, DocumentStorePort, Principal,
    SnapshotReadPort, SourceBatch, SourceCheckpoint, SourceRecordV2,
};
use brain_storage::{LocalPgReader, PgStore};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool, Row,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Duration,
};

const OWNER: &str = "brain_c11_owner";
const RUNTIME: &str = "brain_c11_runtime";
const PORT: u16 = 55441;
const V1_SCHEMA: &str = include_str!("fixtures/storage_v1/schema.sql");
const DATA: &str = include_str!("fixtures/storage_v1/data.json");

fn options(root: &Path, user: &str, database: &str) -> PgConnectOptions {
    PgConnectOptions::new_without_pgpass()
        .host(&root.join("socket").to_string_lossy())
        .port(PORT)
        .username(user)
        .database(database)
        .password("")
        .ssl_mode(PgSslMode::Disable)
}
async fn connect(root: &Path, user: &str, database: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options(root, user, database))
        .await
        .unwrap()
}
async fn new_database(admin: &PgPool, root: &Path, name: &str) -> PgPool {
    assert!(
        name.starts_with("brain_c11_")
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    );
    sqlx::query(&format!("CREATE DATABASE {name}"))
        .execute(admin)
        .await
        .unwrap();
    connect(root, OWNER, name).await
}
fn fixture() -> Value {
    serde_json::from_str(DATA).unwrap()
}

async fn seed_v1(pool: &PgPool) {
    // Deliberately NOT PgStore::apply/migrate or current contract serialization.
    sqlx::raw_sql(V1_SCHEMA).execute(pool).await.unwrap();
    for r in fixture()["records"].as_array().unwrap() {
        sqlx::query(
            "INSERT INTO brain.source_record_revisions
            (source_id,logical_id,revision,content_hash,tombstone,record_json,created_at)
            VALUES($1,$2,$3,$4,$5,$6,'2026-09-24T12:00:00Z')",
        )
        .bind(r["source_id"].as_str().unwrap())
        .bind(r["logical_id"].as_str().unwrap())
        .bind(r["revision"].as_i64().unwrap())
        .bind(r["content_hash"].as_str().unwrap())
        .bind(r["tombstone"].as_bool().unwrap())
        .bind(r)
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO brain.source_record_heads
            (source_id,logical_id,revision,content_hash,tombstone,record_json,updated_at)
            VALUES($1,$2,$3,$4,$5,$6,'2026-09-24T12:00:00Z')
            ON CONFLICT(source_id,logical_id) DO UPDATE SET revision=EXCLUDED.revision,
            content_hash=EXCLUDED.content_hash,tombstone=EXCLUDED.tombstone,record_json=EXCLUDED.record_json
            WHERE brain.source_record_heads.revision<EXCLUDED.revision")
            .bind(r["source_id"].as_str().unwrap()).bind(r["logical_id"].as_str().unwrap())
            .bind(r["revision"].as_i64().unwrap()).bind(r["content_hash"].as_str().unwrap())
            .bind(r["tombstone"].as_bool().unwrap()).bind(r)
            .execute(pool).await.unwrap();
    }
    for r in fixture()["releases"].as_array().unwrap() {
        sqlx::query(
            "INSERT INTO brain.corpus_releases_v1
            (release_id,knowledge_version,patch,release_json,created_at)
            VALUES($1,$2,$3,$4,'2026-09-24T12:00:00Z')",
        )
        .bind(r["release_id"].as_str().unwrap())
        .bind(r["knowledge_version"].as_str().unwrap())
        .bind(r["patch"].as_str().unwrap())
        .bind(r)
        .execute(pool)
        .await
        .unwrap();
    }
}

// Full row equality, including timestamps/JSON/ACLs/leases, not just row counts.
async fn rows(pool: &PgPool) -> BTreeMap<String, Vec<Value>> {
    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT tablename::text FROM pg_tables WHERE schemaname='brain' ORDER BY tablename",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let mut result = BTreeMap::new();
    for table in tables {
        let quoted = table.replace('"', "\"\"");
        let values = sqlx::query_scalar(&format!(
            "SELECT to_jsonb(t) FROM brain.\"{quoted}\" t ORDER BY to_jsonb(t)::text"
        ))
        .fetch_all(pool)
        .await
        .unwrap();
        result.insert(table, values);
    }
    result
}
fn verify_fixture(rows: &BTreeMap<String, Vec<Value>>) {
    assert_eq!(rows["source_record_revisions"].len(), 11);
    assert_eq!(rows["source_record_heads"].len(), 7);
    assert_eq!(rows["corpus_releases_v1"].len(), 2);
    for row in &rows["source_record_revisions"] {
        let original = fixture()["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| {
                r["source_id"] == row["source_id"]
                    && r["logical_id"] == row["logical_id"]
                    && r["revision"] == row["revision"]
            })
            .unwrap()
            .clone();
        assert_eq!(row["record_json"], original);
        assert_eq!(
            row["content_hash"],
            format!(
                "{:x}",
                Sha256::digest(original["content"].as_str().unwrap().as_bytes())
            )
        );
        let record: SourceRecordV2 = serde_json::from_value(original).unwrap();
        record.validate().unwrap();
        assert_eq!(record.content_hash, row["content_hash"].as_str().unwrap());
    }
}
fn visible(snapshot: &CorpusSnapshot, scopes: &[&str], provider: bool) -> BTreeSet<String> {
    let principal = Principal {
        actor_id: "fixture-reader".into(),
        channel: "test".into(),
        scopes: scopes.iter().map(|s| (*s).into()).collect(),
        provider_egress: ["public", "internal", "private"]
            .into_iter()
            .map(String::from)
            .collect(),
    };
    snapshot
        .authorized(&principal, provider)
        .unwrap()
        .iter()
        .map(|r| format!("{}/{}", r.source_id, r.logical_id))
        .collect()
}
fn assert_acl(snapshot: &CorpusSnapshot) {
    let public = BTreeSet::from(["wiki/hero/abrams".into(), "wiki/hero/echo".into()]);
    assert_eq!(visible(snapshot, &[], false), public);
    assert_eq!(visible(snapshot, &["docs.read"], false), public);
    assert_eq!(
        visible(snapshot, &[], true),
        BTreeSet::from(["wiki/hero/abrams".into()])
    );
    let mut ops = public.clone();
    ops.extend(["docs/ops/runbook".into(), "private/notes/review".into()]);
    assert_eq!(visible(snapshot, &["docs.read", "ops.read"], false), ops);
    let mut moderator = public;
    moderator.insert("docs/public/incident".into());
    assert_eq!(visible(snapshot, &["mod.read"], false), moderator);
    // Includes tombstoned Ivy and the private empty-scope deny case, neither can reappear.
    assert_eq!(snapshot.revisions.len(), 7);
}
fn cli(root: &Path, database: &str, user: &str, verb: &str) -> Output {
    let config = root.join(format!("{database}-{user}.json"));
    std::fs::write(
        &config,
        serde_json::to_vec(&json!({
            "socket":root.join("socket"),"port":PORT,"database":database,"user":user
        }))
        .unwrap(),
    )
    .unwrap();
    Command::new(env!("CARGO_BIN_EXE_brain-migrate"))
        .env_clear()
        .env("PGHOST", "/must-not-be-used")
        .env("PGPORT", "1")
        .args([verb, "--config"])
        .arg(config)
        .output()
        .unwrap()
}
fn success(output: Output) {
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn pg_tool(root: &Path, program: &str, database: &str, extra: &[&str]) {
    let bin = std::fs::read_to_string(root.join("pg-bin")).unwrap();
    let output = Command::new(Path::new(bin.trim()).join(program))
        .env_clear()
        .arg("--host")
        .arg(root.join("socket"))
        .args(["--port", "55441", "--username", OWNER, "--no-password"])
        .args(extra)
        .arg(database)
        .output()
        .unwrap();
    success(output);
}

#[test]
fn historical_fixture_checksums_and_hashes_are_pinned() {
    let manifest = include_str!("fixtures/storage_v1/SHA256SUMS");
    for (name, content) in [("schema.sql", V1_SCHEMA), ("data.json", DATA)] {
        assert!(manifest
            .lines()
            .any(|line| line == format!("{:x}  {name}", Sha256::digest(content.as_bytes()))));
    }
    assert!(!V1_SCHEMA.contains("source_checkpoints"));
    assert!(!V1_SCHEMA.contains("source_jobs"));
    for record in fixture()["records"].as_array().unwrap() {
        assert_eq!(
            record["content_hash"],
            format!(
                "{:x}",
                Sha256::digest(record["content"].as_str().unwrap().as_bytes())
            )
        );
    }
}

#[tokio::test]
#[ignore = "requires scripts/test_brain_storage_upgrade.sh; never a production DSN"]
async fn v1_upgrade_restore_and_least_privilege() {
    historical_fixture_checksums_and_hashes_are_pinned();
    let root = PathBuf::from(
        std::env::var_os("BRAIN_C11_SCRATCH").expect("explicit scratch runner required"),
    );
    assert!(root.is_absolute());
    assert_eq!(root.canonicalize().unwrap(), root);
    assert_eq!(root.parent(), Some(Path::new("/tmp")));
    assert!(root
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("brain-c11."));
    assert_eq!(
        std::fs::read_to_string(root.join("marker")).unwrap(),
        "brain-c11-scratch-v1\n"
    );
    let admin = connect(&root, OWNER, "postgres").await;
    let identity = sqlx::query("SELECT current_user::text AS username, inet_server_addr()::text AS address,
        current_setting('data_directory') AS directory, current_setting('listen_addresses') AS listeners")
        .fetch_one(&admin).await.unwrap();
    assert_eq!(identity.get::<String, _>("username"), OWNER);
    assert!(identity.get::<Option<String>, _>("address").is_none());
    assert_eq!(
        identity.get::<String, _>("directory"),
        root.join("data").to_str().unwrap()
    );
    assert_eq!(identity.get::<String, _>("listeners"), "");
    // Only after all independent identity/socket/data-directory checks may tests create DBs/roles.
    let pool = new_database(&admin, &root, "brain_c11_upgrade").await;
    seed_v1(&pool).await;
    let before = rows(&pool).await;
    verify_fixture(&before);
    assert_eq!(before.len(), 3, "real v1 has no checkpoint/job/Fact tables");
    let store = PgStore::new(pool.clone());
    assert!(store.check_core_schema().await.is_err());
    assert!(!cli(&root, "brain_c11_upgrade", OWNER, "check")
        .status
        .success());
    assert_eq!(
        rows(&pool).await,
        before,
        "startup check must never migrate v1"
    );
    let early = store.snapshot("release-early").await.unwrap();
    let latest = store.snapshot("release-latest").await.unwrap();
    assert_acl(&early);
    assert_acl(&latest);

    let backup = root.join("pre-v2.dump");
    pg_tool(
        &root,
        "pg_dump",
        "brain_c11_upgrade",
        &["--format=custom", "--file", backup.to_str().unwrap()],
    );
    success(cli(&root, "brain_c11_upgrade", OWNER, "up"));
    store.check_core_schema().await.unwrap();
    let after = rows(&pool).await;
    assert_eq!(after.len(), 7);
    for (table, values) in &before {
        assert_eq!(&after[table], values, "v1 table changed: {table}");
    }
    verify_fixture(&after);
    assert!(after["source_jobs_v1"].is_empty());
    assert!(after["source_checkpoints_v1"].is_empty());
    assert!(store.checkpoint("wiki").await.unwrap().is_none());
    assert_eq!(store.snapshot("release-early").await.unwrap(), early);
    assert_eq!(store.snapshot("release-latest").await.unwrap(), latest);
    println!("PASS historical v1 -> v2: IDs, hashes, provenance, ACLs, tombstones, release pins and all original row values");

    // No owner membership/DDL privilege for the runtime role, not even database TEMP/CREATE.
    sqlx::raw_sql(
        "CREATE ROLE brain_c11_runtime LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT;
        REVOKE CREATE ON SCHEMA public FROM PUBLIC;
        REVOKE CREATE, TEMPORARY ON DATABASE brain_c11_upgrade FROM PUBLIC;
        GRANT CONNECT ON DATABASE brain_c11_upgrade TO brain_c11_runtime;
        GRANT USAGE ON SCHEMA brain TO brain_c11_runtime;
        GRANT SELECT ON ALL TABLES IN SCHEMA brain TO brain_c11_runtime;
        GRANT INSERT, UPDATE, DELETE ON brain.source_record_revisions, brain.source_record_heads,
            brain.corpus_releases_v1, brain.source_jobs_v1, brain.source_checkpoints_v1,
            brain.conversation_owners_v1 TO brain_c11_runtime;",
    )
    .execute(&pool)
    .await
    .unwrap();
    let runtime_pool = connect(&root, RUNTIME, "brain_c11_upgrade").await;
    let runtime = PgStore::new(runtime_pool.clone());
    let ddl: bool = sqlx::query_scalar("SELECT has_schema_privilege(current_user,'brain','CREATE')
        OR has_database_privilege(current_user,current_database(),'CREATE')
        OR has_database_privilege(current_user,current_database(),'TEMP')
        OR pg_has_role(current_user,'brain_c11_owner','MEMBER')
        OR (SELECT rolsuper OR rolcreatedb OR rolcreaterole FROM pg_roles WHERE rolname=current_user)")
        .fetch_one(&runtime_pool).await.unwrap();
    assert!(!ddl);
    runtime.check_core_schema().await.unwrap();
    success(cli(&root, "brain_c11_upgrade", RUNTIME, "check"));
    assert!(runtime.migrate_core().await.is_err());
    for sql in [
        "CREATE TABLE brain.forbidden(id integer)",
        "ALTER TABLE brain.source_record_heads ADD COLUMN forbidden integer",
        "UPDATE brain.core_schema_version SET schema_version=3",
    ] {
        let error = sqlx::query(sql).execute(&runtime_pool).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
    }
    assert_eq!(rows(&pool).await, after);
    assert_eq!(runtime.snapshot("release-early").await.unwrap(), early);
    let socket = root.join("socket");
    let native = tokio::task::spawn_blocking(move || {
        let reader = LocalPgReader::new(socket, PORT, RUNTIME, "brain_c11_upgrade").unwrap();
        reader
            .claim_conversation("upgrade-conversation", "actor-a")
            .unwrap();
        assert!(reader
            .claim_conversation("upgrade-conversation", "actor-b")
            .is_err());
        reader.read_snapshot("release-early").unwrap()
    })
    .await
    .unwrap();
    assert_eq!(native, early);
    assert_acl(&native);

    // v1 had no persisted checkpoints. Exercise a real v2 resume, then migrate again.
    let first = SourceBatch {
        expected_generation: 0,
        records: vec![],
        checkpoint: SourceCheckpoint {
            source_id: "wiki".into(),
            configuration: "pinned-config-sha256-fixture".into(),
            generation: 1,
            state: json!({"cursor":"page-17", "source_revision":"9b31d609ae4528c77196ed20f7828a150b2c06fd", "deleted":["hero/ivy"]}),
        },
    };
    let old_lease = runtime.claim("wiki", "v2-worker-1", 60000).await.unwrap();
    assert!(!runtime.commit(&first, &old_lease).await.unwrap().replayed);
    let mut record: SourceRecordV2 =
        serde_json::from_value(fixture()["records"][1].clone()).unwrap();
    record.revision = 5;
    record.content = "Abrams: Schaden 56.\n".into();
    record.content_hash = format!("{:x}", Sha256::digest(record.content.as_bytes()));
    let mut second = first.clone();
    second.expected_generation = 1;
    second.checkpoint.generation = 2;
    second.checkpoint.state["cursor"] = json!("page-18");
    second.records.push(record);
    let second_lease = runtime.claim("wiki", "v2-worker-2", 60000).await.unwrap();
    assert!(second_lease.fence > old_lease.fence);
    runtime.commit(&second, &second_lease).await.unwrap();
    runtime.publish(&early.release).await.unwrap();
    let live_lease = runtime
        .claim("wiki", "pending-worker", 60000)
        .await
        .unwrap();
    let populated_v2 = rows(&pool).await;
    success(cli(&root, "brain_c11_upgrade", OWNER, "up"));
    let (a, b) = tokio::join!(store.migrate_core(), store.migrate_core());
    a.unwrap();
    b.unwrap();
    assert_eq!(
        rows(&pool).await,
        populated_v2,
        "repeat/concurrent upgrades must not reset checkpoints, fences or timestamps"
    );
    assert_eq!(
        runtime.checkpoint("wiki").await.unwrap().unwrap(),
        second.checkpoint
    );
    assert!(
        runtime
            .commit(&second, &second_lease)
            .await
            .unwrap()
            .replayed
    );
    let mut third = second.clone();
    third.expected_generation = 2;
    third.checkpoint.generation = 3;
    let mut erased: SourceRecordV2 =
        serde_json::from_value(fixture()["records"][4].clone()).unwrap();
    erased.revision += 1;
    erased.tombstone = true;
    erased.content.clear();
    erased.content_hash = format!("{:x}", Sha256::digest(erased.content.as_bytes()));
    let mut restricted: SourceRecordV2 =
        serde_json::from_value(fixture()["records"][1].clone()).unwrap();
    restricted.revision = 6;
    restricted.visibility = brain_contracts::SourceVisibility::Private;
    restricted.allowed_scopes = BTreeSet::from(["after.cutover".into()]);
    third.records = vec![erased, restricted];
    assert!(runtime.commit(&third, &old_lease).await.is_err());
    runtime.commit(&third, &live_lease).await.unwrap();
    assert!(visible(
        &runtime.snapshot("release-early").await.unwrap(),
        &[],
        false
    )
    .is_empty());

    // The integration branch already had unmarked v2 installations. Adopt one without
    // rewriting its nonempty checkpoints/receipts/leases or its new delete/ACL deltas.
    let populated = rows(&pool).await;
    sqlx::query("DROP TABLE brain.core_schema_version")
        .execute(&pool)
        .await
        .unwrap();
    assert!(store.check_core_schema().await.is_err());
    store.migrate_core().await.unwrap();
    let adopted = rows(&pool).await;
    for (table, values) in &populated {
        if table != "core_schema_version" {
            assert_eq!(&adopted[table], values);
        }
    }
    store.migrate_core().await.unwrap();
    assert_eq!(rows(&pool).await, adopted);
    println!("PASS DDL-free service preflight/read/write, native reader, checkpoint resume/replay, concurrent upgrade idempotence and stale-writer fencing");

    // Restore rehearsal: close/drain the only runtime pool, then fence new connections.
    runtime_pool.close().await;
    sqlx::raw_sql(
        "ALTER ROLE brain_c11_runtime NOLOGIN;
        REVOKE INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA brain FROM brain_c11_runtime",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert!(PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options(&root, RUNTIME, "brain_c11_upgrade"))
        .await
        .is_err());
    let restored = new_database(&admin, &root, "brain_c11_restore_v1").await;
    // pg_restore takes a database flag and the archive as its positional argument.
    pg_tool(
        &root,
        "pg_restore",
        backup.to_str().unwrap(),
        &[
            "--exit-on-error",
            "--single-transaction",
            "--dbname",
            "brain_c11_restore_v1",
        ],
    );
    assert_eq!(rows(&restored).await, before);
    let old_store = PgStore::new(restored.clone());
    assert!(old_store.check_core_schema().await.is_err());
    assert_eq!(old_store.snapshot("release-early").await.unwrap(), early);
    assert_eq!(old_store.snapshot("release-latest").await.unwrap(), latest);
    verify_fixture(&rows(&restored).await);
    restored.close().await;
    pool.close().await;
    println!("PASS actual pg_dump/pg_restore to a fresh v1 database: post-cutover writes excluded, restorepoint and release/ACL semantics exact");

    rejection_probes(&admin, &root).await;
    admin.close().await;
}

async fn rejection_probes(admin: &PgPool, root: &Path) {
    // Prove the old two-COMMIT bug cannot leave newly created v1 tables behind either.
    let fresh = new_database(admin, root, "brain_c11_fresh_partial").await;
    sqlx::raw_sql(
        "CREATE SCHEMA brain; CREATE TABLE brain.source_jobs_v1(source_id text PRIMARY KEY)",
    )
    .execute(&fresh)
    .await
    .unwrap();
    let incomplete = rows(&fresh).await;
    assert!(PgStore::new(fresh.clone()).migrate_core().await.is_err());
    assert_eq!(
        rows(&fresh).await,
        incomplete,
        "v1 DDL survived a failed v2 migration"
    );
    fresh.close().await;
    for (name, damage, expected) in [
        ("brain_c11_legacy_max", "UPDATE brain.corpus_releases_v1 SET release_json=jsonb_set(release_json,'{source_revisions}','{\"wiki\":4,\"docs\":3,\"private\":1}') WHERE release_id='release-early'", "legacy or invalid release pins"),
        ("brain_c11_missing_pin", "UPDATE brain.corpus_releases_v1 SET release_json=jsonb_set(release_json,'{source_revisions,wiki,missing}', '1') WHERE release_id='release-early'", "release revision or current ACL missing"),
        ("brain_c11_bad_identity", "UPDATE brain.source_record_revisions SET content_hash='mismatched-column' WHERE source_id='wiki' AND logical_id='hero/abrams' AND revision=1", "columns and JSON disagree"),
        ("brain_c11_partial_ddl", "CREATE TABLE brain.source_jobs_v1(source_id text PRIMARY KEY)", "core migration failed"),
    ] {
        let pool = new_database(admin, root, name).await;
        seed_v1(&pool).await;
        sqlx::raw_sql(damage).execute(&pool).await.unwrap();
        let before = rows(&pool).await;
        let error = PgStore::new(pool.clone()).migrate_core().await.unwrap_err();
        assert!(error.to_string().contains(expected), "unexpected error: {error}");
        assert_eq!(rows(&pool).await, before, "failed migration partially committed: {name}");
        pool.close().await;
    }
    let pool = new_database(admin, root, "brain_c11_lock").await;
    seed_v1(&pool).await;
    let before = rows(&pool).await;
    let mut writer = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE brain.source_record_heads IN ROW EXCLUSIVE MODE")
        .execute(&mut *writer)
        .await
        .unwrap();
    let store = PgStore::new(pool.clone());
    assert!(store
        .migrate_core()
        .await
        .unwrap_err()
        .to_string()
        .contains("core migration failed"));
    assert_eq!(rows(&pool).await, before);
    writer.rollback().await.unwrap();
    store.migrate_core().await.unwrap();
    sqlx::query(
        "UPDATE brain.core_schema_version SET schema_version=3,store_contract='brain.store.v3'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let future = rows(&pool).await;
    assert!(store.check_core_schema().await.is_err());
    assert!(store.migrate_core().await.is_err());
    assert_eq!(
        rows(&pool).await,
        future,
        "older binary may not overwrite a future version"
    );
    pool.close().await;
    println!("PASS atomic rejection: ambiguous early-v1 release maxima, missing revision, corrupt identity, partial DDL, live writer lock, future version");
}
