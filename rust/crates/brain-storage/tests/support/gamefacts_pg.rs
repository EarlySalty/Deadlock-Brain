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

pub async fn assert_count(pool: &PgPool, sql: &str, expected: i64) {
    assert_eq!(count(pool, sql).await, expected);
}

pub fn fill_to_row_limit(
    mut set: brain_storage::gamefacts::FactSet,
) -> brain_storage::gamefacts::FactSet {
    use brain_storage::gamefacts::limits;
    let rows = set.entities.len() + set.aliases.len() + set.facts.len() + set.relations.len();
    let template = set
        .facts
        .first()
        .expect("boundary fixture requires a fact")
        .clone();
    set.facts.extend((rows..limits::ROWS).map(|index| {
        let mut fact = template.clone();
        fact.stat_key = format!("boundary_{index}");
        fact
    }));
    assert!(limits::validate(&set).is_ok());
    set
}

pub async fn assert_boundary_applications(pool: &PgPool, run: i64) {
    let applications: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT raw_object_id,count(*) FROM canon.receipt WHERE ingest_run_id=$1 GROUP BY raw_object_id,application_xid ORDER BY min(id)",
    )
    .bind(run)
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(applications.len(), 3);
    assert_eq!(applications[0].0, applications[2].0);
    assert_ne!(applications[0].0, applications[1].0);
    for (_, rows) in applications {
        assert_eq!(rows as usize, brain_storage::gamefacts::limits::ROWS);
    }
}
