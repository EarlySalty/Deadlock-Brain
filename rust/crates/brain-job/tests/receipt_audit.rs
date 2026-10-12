use brain_job::{
    extract::{raw_locator_hashes, RawLocatorHash},
    git::{Blob, Source},
    receipt_audit::audit_pool,
    store::{Counts, Store},
};
use brain_storage::PgStore;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let root = PathBuf::from(format!("/tmp/brain-p9-audit.{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::create_dir(root.join("socket")).unwrap();
        let scratch = Self { root };
        scratch.command(
            "initdb",
            &["--auth=trust", "--no-locale", "--username=fixture_dba"],
        );
        scratch.command(
            "pg_ctl",
            &[
                "--wait",
                "--log",
                scratch.root.join("postgres.log").to_str().unwrap(),
                "--options",
                &format!(
                    "-k {} -p 55471 -h '' -c fsync=off",
                    scratch.root.join("socket").display()
                ),
                "start",
            ],
        );
        scratch
    }

    fn command(&self, program: &str, args: &[&str]) {
        let output = Command::new(format!("/usr/lib/postgresql/16/bin/{program}"))
            .arg("--pgdata")
            .arg(self.root.join("data"))
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(3)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(self.root.join("socket").to_str().unwrap())
                    .port(55471)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(sqlx::postgres::PgSslMode::Disable),
            )
            .await
            .unwrap()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let stopped = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("--pgdata")
            .arg(self.root.join("data"))
            .args(["--wait", "--mode=immediate", "stop"])
            .output();
        if stopped.is_ok_and(|output| output.status.success()) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

async fn ingest(pool: &PgPool, revision: &str, document: Value) {
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let run = store.begin_run(source_id, revision).await.unwrap().unwrap();
    let bytes = serde_json::to_vec(&document).unwrap();
    let mut counts = Counts::default();
    store
        .object(
            &run,
            source,
            "data/json/hero-data.json",
            Some(Blob {
                hash: Sha256::digest(&bytes).to_vec(),
                bytes,
            }),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&run).await.unwrap();
    store.finish(&counts, false).await.unwrap();
}

#[tokio::test]
async fn real_receipt_audit_quarantines_only_corrupted_fact_and_bounds_sample() {
    let scratch = Scratch::new();
    let admin = scratch.pool("postgres", "fixture_dba").await;
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(directory, scratch.root.join("data").to_str().unwrap());
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
        GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE",
    )
    .execute(&admin)
    .await
    .unwrap();
    sqlx::query("CREATE DATABASE brain_p9 OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let owner = scratch.pool("brain_p9", "brain_migrate").await;
    let owner_store = PgStore::new(owner.clone());
    owner_store.migrate_gamefacts().await.unwrap();
    owner_store.check_gamefacts_schema().await.unwrap();
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(versions, [1, 6]);
    let writer = scratch.pool("brain_p9", "brain_ingest").await;
    ingest(
        &writer,
        "fixture-a",
        json!({"baba":{"Name":"Baba","MaxHealth":100,"Stamina":3}}),
    )
    .await;
    let raw: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=1")
        .fetch_one(&owner)
        .await
        .unwrap();
    let stored: Vec<(String, Vec<u8>)> = sqlx::query_as("SELECT locator,value_sha256 FROM canon.receipt WHERE raw_object_id=1 ORDER BY id LIMIT 200")
        .fetch_all(&owner).await.unwrap();
    let locators: Vec<_> = stored.iter().map(|(locator, _)| locator.as_str()).collect();
    let hashes = raw_locator_hashes(&raw, "json_pointer", &locators).unwrap();
    assert_eq!(hashes.len(), stored.len());
    for (actual, (_, expected)) in hashes.into_iter().zip(stored) {
        assert_eq!(actual, RawLocatorHash::Found(expected));
    }
    let bad: i64 =
        sqlx::query_scalar("SELECT id FROM canon.receipt WHERE locator='/baba/MaxHealth'")
            .fetch_one(&owner)
            .await
            .unwrap();
    sqlx::query(
        "UPDATE canon.receipt SET value_sha256=sha256(convert_to('corrupted','UTF8')) WHERE id=$1",
    )
    .bind(bad)
    .execute(&owner)
    .await
    .unwrap();
    let result = audit_pool(&writer, 16 * 1024 * 1024).await.unwrap();
    assert_eq!(
        (
            result.sampled,
            result.objects,
            result.matched,
            result.mismatched,
            result.locator_not_found,
            result.unsupported_locator_kind
        ),
        (3, 1, 2, 1, 0, 0)
    );
    let quarantine: Vec<i64> =
        sqlx::query_scalar("SELECT receipt_id FROM canon.receipt_quarantine ORDER BY receipt_id")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(quarantine, [bad]);
    let canonical: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.fact")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(canonical, 3);
    let visible: Vec<String> =
        sqlx::query_scalar("SELECT stat_key FROM serve.current_fact ORDER BY stat_key")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(visible, ["identity.name", "stats.stamina"]);
    let profile: Value =
        sqlx::query_scalar("SELECT stats FROM serve.hero_profile WHERE key='baba'")
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(profile["facts"].as_array().unwrap().len(), 1);
    assert_eq!(profile["facts"][0]["stat_key"], "stats.stamina");
    let run: Value = sqlx::query_scalar("SELECT counts FROM ops.job_run WHERE job='audit-receipts' AND status='succeeded' ORDER BY id DESC LIMIT 1")
        .fetch_one(&owner).await.unwrap();
    assert_eq!(run, result.value());
    let second = audit_pool(&writer, 16 * 1024 * 1024).await.unwrap();
    assert_eq!(
        (second.sampled, second.matched, second.mismatched),
        (2, 2, 0)
    );
    let raw_count: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(raw_count, 1);
    let missing: i64 =
        sqlx::query_scalar("SELECT id FROM canon.receipt WHERE locator='/baba/Stamina'")
            .fetch_one(&owner)
            .await
            .unwrap();
    sqlx::query("UPDATE canon.receipt SET locator='/baba/missing' WHERE id=$1")
        .bind(missing)
        .execute(&owner)
        .await
        .unwrap();
    let missing_result = audit_pool(&writer, 16 * 1024 * 1024).await.unwrap();
    assert_eq!(
        (missing_result.matched, missing_result.locator_not_found),
        (1, 1)
    );
    let flagged: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM canon.receipt_quarantine WHERE receipt_id=$1)",
    )
    .bind(missing)
    .fetch_one(&owner)
    .await
    .unwrap();
    assert!(flagged);
    let unknown: i64 =
        sqlx::query_scalar("SELECT id FROM canon.receipt WHERE locator='/baba/Name'")
            .fetch_one(&owner)
            .await
            .unwrap();
    sqlx::query("UPDATE canon.receipt SET locator_kind='future_pointer' WHERE id=$1")
        .bind(unknown)
        .execute(&owner)
        .await
        .unwrap();
    let partial = audit_pool(&writer, 16 * 1024 * 1024).await.unwrap();
    assert_eq!(
        (
            partial.sampled,
            partial.unsupported_locator_kind,
            partial.mismatched,
            partial.locator_not_found
        ),
        (1, 1, 0, 0)
    );
    let flagged: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM canon.receipt_quarantine WHERE receipt_id=$1)",
    )
    .bind(unknown)
    .fetch_one(&owner)
    .await
    .unwrap();
    assert!(!flagged);
    let status: String = sqlx::query_scalar(
        "SELECT status FROM ops.job_run WHERE job='audit-receipts' ORDER BY id DESC LIMIT 1",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(status, "partial");
    sqlx::query("UPDATE canon.receipt SET locator_kind='json_pointer' WHERE id=$1")
        .bind(unknown)
        .execute(&owner)
        .await
        .unwrap();
    let mut weapon = serde_json::Map::new();
    for n in 0..250 {
        weapon.insert(format!("field{n}"), json!(n));
    }
    ingest(
        &writer,
        "fixture-b",
        json!({"baba":{"Name":"Baba","Weapon":weapon}}),
    )
    .await;
    let bounded = audit_pool(&writer, 16 * 1024 * 1024).await.unwrap();
    assert_eq!(bounded.sampled, 200);
    assert_eq!(bounded.matched, 200);
    assert_eq!(
        bounded.mismatched + bounded.locator_not_found + bounded.unsupported_locator_kind,
        0
    );
    writer.close().await;
    owner.close().await;
    admin.close().await;
}
