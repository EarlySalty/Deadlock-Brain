use brain_storage::PgStore;
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

const SEED: &str = include_str!("fixtures/receipt_audit_seed.sql");

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let root = PathBuf::from(format!("/tmp/brain-p9-quarantine.{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::create_dir(root.join("socket")).unwrap();
        let scratch = Self { root };
        scratch.postgres(
            "initdb",
            &["--auth=trust", "--no-locale", "--username=fixture_dba"],
        );
        scratch.postgres(
            "pg_ctl",
            &[
                "--wait",
                "--log",
                scratch.root.join("postgres.log").to_str().unwrap(),
                "--options",
                &format!(
                    "-k {} -p 55469 -h '' -c fsync=off",
                    scratch.root.join("socket").display()
                ),
                "start",
            ],
        );
        scratch
    }

    fn postgres(&self, program: &str, args: &[&str]) {
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
                    .port(55469)
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

async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

async fn audit(pool: &PgPool, receipt: i64, outcome: &str, observed: Option<Vec<u8>>) {
    sqlx::query("INSERT INTO ops.receipt_audit(job_run_id,receipt_id,outcome,observed_value_sha256) VALUES (2,$1,$2,$3)")
        .bind(receipt).bind(outcome).bind(observed).execute(pool).await.unwrap();
}

async fn forbidden(pool: &PgPool, sql: &str) {
    let error = sqlx::raw_sql(sql).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
}

#[tokio::test]
async fn quarantine_projection_contract_in_real_postgres() {
    let scratch = Scratch::new();
    let admin = scratch.pool("postgres", "fixture_dba").await;
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(directory, scratch.root.join("data").to_str().unwrap());
    let tcp: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert!(tcp.is_none());
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE",
    )
    .execute(&admin)
    .await
    .unwrap();
    sqlx::query("CREATE DATABASE brain_p9 OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let database_admin = scratch.pool("brain_p9", "fixture_dba").await;
    PgStore::new(database_admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let owner = scratch.pool("brain_p9", "brain_migrate").await;
    let ingest = scratch.pool("brain_p9", "brain_ingest").await;
    sqlx::raw_sql(SEED).execute(&ingest).await.unwrap();
    assert_eq!(
        count(&owner, "SELECT count(*) FROM serve.current_fact").await,
        2
    );
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(versions, [1, 6]);
    PgStore::new(owner.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    assert_eq!(
        count(&owner, "SELECT count(*) FROM serve.current_fact").await,
        2
    );
    assert_eq!(
        count(&owner, "SELECT count(*) FROM canon.receipt_quarantine").await,
        0
    );
    sqlx::query(
        "UPDATE canon.receipt SET value_sha256=sha256(convert_to('corrupted','UTF8')) WHERE id=3",
    )
    .execute(&owner)
    .await
    .unwrap();
    let stored = sqlx::query("SELECT r.value_sha256,o.body FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE r.id=3")
        .fetch_one(&owner).await.unwrap();
    let raw: Value = serde_json::from_slice(&stored.get::<Vec<u8>, _>("body")).unwrap();
    assert_eq!(raw["health"], 100);
    let observed: Vec<u8> = sqlx::query_scalar("SELECT sha256(convert_to('100','UTF8'))")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_ne!(stored.get::<Vec<u8>, _>("value_sha256"), observed);
    sqlx::query("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('audit-receipts','fixture','v1')")
        .execute(&ingest).await.unwrap();
    let healthy: Vec<u8> = sqlx::query_scalar("SELECT value_sha256 FROM canon.receipt WHERE id=4")
        .fetch_one(&owner)
        .await
        .unwrap();
    audit(&ingest, 4, "match", Some(healthy)).await;
    audit(&ingest, 3, "mismatch", Some(observed)).await;
    let quarantined: Vec<i64> =
        sqlx::query_scalar("SELECT receipt_id FROM canon.receipt_quarantine ORDER BY receipt_id")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(quarantined, [3]);
    assert_eq!(count(&owner, "SELECT count(*) FROM canon.fact").await, 2);
    let served: Vec<i64> =
        sqlx::query_scalar("SELECT receipt_id FROM serve.current_fact ORDER BY receipt_id")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(served, [4]);
    let profile: Value =
        sqlx::query_scalar("SELECT to_jsonb(h) FROM serve.hero_profile h WHERE entity_id=1")
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(profile["stats"]["facts"], serde_json::json!([]));
    assert_eq!(
        profile["abilities"]["links"][0]["facts"][0]["citation"],
        "4"
    );
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM ops.receipt_audit WHERE outcome='mismatch'"
        )
        .await,
        1
    );
    audit(&ingest, 5, "unsupported_locator_kind", None).await;
    assert_eq!(
        count(&owner, "SELECT count(*) FROM canon.receipt_quarantine").await,
        1
    );
    forbidden(&ingest, "DELETE FROM canon.receipt_quarantine").await;
    forbidden(&ingest, "UPDATE ops.receipt_audit SET outcome='match'").await;
    forbidden(
        &ingest,
        "INSERT INTO canon.receipt_quarantine(receipt_id,job_run_id) VALUES (4,2)",
    )
    .await;
    forbidden(&ingest, "SELECT ops.quarantine_audited_receipts()").await;
    let reader = scratch.pool("brain_p9", "brain_readonly").await;
    let mut tool = reader.acquire().await.unwrap();
    sqlx::query("SET SESSION ROLE brain_tool")
        .execute(&mut *tool)
        .await
        .unwrap();
    let usable: bool = sqlx::query_scalar("SELECT serve.receipt_citation_usable('3')")
        .fetch_one(&mut *tool)
        .await
        .unwrap();
    assert!(!usable);
    let hidden: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt WHERE id=3")
        .fetch_one(&mut *tool)
        .await
        .unwrap();
    assert_eq!(hidden, 0);
    for citation in ["unknown", "9223372036854775808", "0", "-1", ""] {
        let usable: bool = sqlx::query_scalar("SELECT serve.receipt_citation_usable($1)")
            .bind(citation)
            .fetch_one(&mut *tool)
            .await
            .unwrap();
        assert!(!usable);
    }
    let visible: Value =
        sqlx::query_scalar("SELECT stats FROM serve.audited_hero_profile WHERE entity_id=1")
            .fetch_one(&mut *tool)
            .await
            .unwrap();
    assert_eq!(visible["facts"], serde_json::json!([]));
    drop(tool);
    audit(&ingest, 2, "mismatch", None).await;
    let links: Value = sqlx::query_scalar(
        "SELECT abilities->'links' FROM serve.audited_hero_profile WHERE entity_id=1",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(links, serde_json::json!([]));
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM serve.audited_entity WHERE id=2"
        )
        .await,
        0
    );
    audit(&ingest, 1, "mismatch", None).await;
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM serve.audited_entity WHERE id=1"
        )
        .await,
        0
    );
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM serve.audited_hero_profile WHERE entity_id=1"
        )
        .await,
        0
    );
    assert_eq!(
        count(&owner, "SELECT count(*) FROM serve.current_alias").await,
        0
    );
    let mut tx = ingest.begin().await.unwrap();
    sqlx::query("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('audit-receipts','fixture','v1')")
        .execute(&mut *tx).await.unwrap();
    sqlx::query(
        "INSERT INTO ops.receipt_audit(job_run_id,receipt_id,outcome) VALUES (3,4,'mismatch')",
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM canon.receipt_quarantine WHERE receipt_id=4"
        )
        .await,
        0
    );
    assert_eq!(
        count(
            &owner,
            "SELECT count(*) FROM serve.current_fact WHERE receipt_id=4"
        )
        .await,
        1
    );
    PgStore::new(owner.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    reader.close().await;
    ingest.close().await;
    owner.close().await;
    database_admin.close().await;
    admin.close().await;
}
