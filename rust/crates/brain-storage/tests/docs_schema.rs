#[path = "support/scratch_pg.rs"]
mod scratch_pg;

use brain_storage::PgStore;
use scratch_pg::ScratchPg;
use sqlx::{postgres::PgPoolOptions, PgPool};

struct Database {
    scratch: ScratchPg,
    owner: PgPool,
    ingest: PgPool,
}

async fn pool(scratch: &ScratchPg, database: &str, username: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(4)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(scratch.directory.join("socket").to_str().unwrap())
                .port(55439)
                .database(database)
                .username(username)
                .password("")
                .ssl_mode(sqlx::postgres::PgSslMode::Disable),
        )
        .await
        .unwrap()
}

impl Database {
    async fn new() -> Self {
        let scratch = ScratchPg::start();
        let admin = pool(&scratch, "postgres", "brain_core_test").await;
        let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(directory, scratch.directory.join("data").to_str().unwrap());
        let listeners: String = sqlx::query_scalar("SELECT current_setting('listen_addresses')")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(listeners, "");
        sqlx::raw_sql(
            "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE",
        )
        .execute(&admin)
        .await
        .unwrap();
        sqlx::query("CREATE DATABASE brain_docs_registration OWNER brain_migrate")
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
        let migration_admin = pool(&scratch, "brain_docs_registration", "brain_core_test").await;
        PgStore::new(migration_admin.clone())
            .migrate_gamefacts()
            .await
            .unwrap();
        migration_admin.close().await;
        let owner = pool(&scratch, "brain_docs_registration", "brain_migrate").await;
        let ingest = pool(&scratch, "brain_docs_registration", "brain_ingest").await;
        Self {
            scratch,
            owner,
            ingest,
        }
    }

    async fn close(self) {
        self.ingest.close().await;
        self.owner.close().await;
        drop(self.scratch);
    }
}

async fn stamps(owner: &PgPool) -> Vec<(i32, i64)> {
    sqlx::query_as(
        "SELECT version,xmin::text::bigint FROM ops.gamefacts_schema_version ORDER BY version",
    )
    .fetch_all(owner)
    .await
    .unwrap()
}

#[tokio::test]
async fn canonical_runner_registers_docs_and_reapply_preserves_versions_and_objects() {
    let db = Database::new().await;
    let store = PgStore::new(db.owner.clone());
    store.check_gamefacts_schema().await.unwrap();
    let before = stamps(&db.owner).await;
    assert_eq!(before.iter().filter(|row| row.0 == 5).count(), 1);
    let v1 = before.iter().find(|row| row.0 == 1).unwrap();
    let docs = before.iter().find(|row| row.0 == 5).unwrap();
    assert_ne!(v1.1, docs.1);
    let objects: Vec<(String, i64)> = sqlx::query_as(
        "SELECT c.relname::text,c.oid::bigint FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        WHERE n.nspname='docs' AND c.relname IN ('page','section','current_section') ORDER BY c.relname",
    )
    .fetch_all(&db.owner)
    .await
    .unwrap();
    assert_eq!(objects.len(), 3);
    store.migrate_gamefacts().await.unwrap();
    store.migrate_gamefacts().await.unwrap();
    store.check_gamefacts_schema().await.unwrap();
    assert_eq!(stamps(&db.owner).await, before);
    let after: Vec<(String, i64)> = sqlx::query_as(
        "SELECT c.relname::text,c.oid::bigint FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        WHERE n.nspname='docs' AND c.relname IN ('page','section','current_section') ORDER BY c.relname",
    )
    .fetch_all(&db.owner)
    .await
    .unwrap();
    assert_eq!(after, objects);
    assert!(PgStore::new(db.ingest.clone())
        .migrate_gamefacts()
        .await
        .is_err());
    assert_eq!(stamps(&db.owner).await, before);
    db.close().await;
}

#[tokio::test]
async fn canonical_preflight_rejects_missing_docs_version_without_repairing_ledger() {
    let db = Database::new().await;
    sqlx::query("DELETE FROM ops.gamefacts_schema_version WHERE version=5")
        .execute(&db.owner)
        .await
        .unwrap();
    let before = stamps(&db.owner).await;
    let store = PgStore::new(db.owner.clone());
    assert!(store.check_gamefacts_schema().await.is_err());
    assert_eq!(stamps(&db.owner).await, before);
    assert!(store.migrate_gamefacts().await.is_err());
    assert_eq!(stamps(&db.owner).await, before);
    let sections: i64 = sqlx::query_scalar("SELECT count(*) FROM docs.section")
        .fetch_one(&db.owner)
        .await
        .unwrap();
    assert_eq!(sections, 0);
    db.close().await;
}

#[tokio::test]
async fn canonical_preflight_and_runner_reject_unknown_versions_without_changes() {
    let db = Database::new().await;
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES (99)")
        .execute(&db.owner)
        .await
        .unwrap();
    let before = stamps(&db.owner).await;
    let store = PgStore::new(db.owner.clone());
    assert!(store.check_gamefacts_schema().await.is_err());
    assert!(store.migrate_gamefacts().await.is_err());
    assert_eq!(stamps(&db.owner).await, before);
    db.close().await;
}
