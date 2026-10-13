use brain_storage::PgStore;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

#[path = "scratch_pg.rs"]
mod scratch_pg;
pub use scratch_pg::ScratchPg;

pub async fn fixture(max_connections: u32) -> (ScratchPg, PgPool, PgPool) {
    let pg = ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    PgStore::new(writer.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    (pg, admin, writer)
}

pub async fn growth(pool: &PgPool) -> (i64, i64, i64) {
    sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)")
        .fetch_one(pool).await.unwrap()
}

pub async fn rejection_codes(pool: &PgPool, run: i64) -> Vec<String> {
    sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
        .bind(run)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}
