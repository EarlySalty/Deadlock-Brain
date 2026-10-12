use brain_job::{assets::storage::Store, store::Counts};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

async fn pool(scratch: &scratch_pg::ScratchPg, user: &str, database: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(scratch.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username(user)
                .database(database)
                .ssl_mode(PgSslMode::Disable)
                .disable_statement_logging(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn raw_reversion_dedup_rollback_publication_and_source_separation_use_postgres() {
    let scratch = scratch_pg::ScratchPg::start();
    let admin = pool(&scratch, "brain_core_test", "postgres").await;
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(
        std::path::PathBuf::from(directory),
        scratch.directory.join("data")
    );
    sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER; CREATE ROLE brain_ingest LOGIN NOSUPERUSER; CREATE ROLE brain_service LOGIN NOSUPERUSER; CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;").execute(&admin).await.unwrap();
    sqlx::query("CREATE DATABASE assets_test OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let owner = pool(&scratch, "brain_migrate", "assets_test").await;
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .execute(&owner)
    .await
    .unwrap();
    sqlx::query("ALTER ROLE brain_ingest SET default_transaction_isolation='repeatable read'")
        .execute(&admin)
        .await
        .unwrap();
    let pool = pool(&scratch, "brain_ingest", "assets_test").await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    assert!(Store::start(pool.clone(), 300).await.is_err());
    let mut counts = Counts::default();
    let a = br#"[{"id":3247040238,"class_name":"viscous_goo_grenade","type":"ability","name":"Splatter","properties":{"AbilityCooldown":{"value":"26.0"}}}]"#;
    let b = br#"[{"id":3247040238,"class_name":"viscous_goo_grenade","type":"ability","name":"Splatter changed","properties":{"AbilityCooldown":{"value":24.0}}}]"#;
    let en = "https://api.deadlock-api.com/v1/assets/items?language=english";
    let de = "https://api.deadlock-api.com/v1/assets/items?language=german";
    store.object(en, "en", a, &mut counts).await.unwrap();
    let first: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.fact")
        .fetch_one(&pool)
        .await
        .unwrap();
    store.object(en, "en", a, &mut counts).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.fact")
            .fetch_one(&pool)
            .await
            .unwrap(),
        first
    );
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.sources_unchanged, 1);
    store.object(de, "de", a, &mut counts).await.unwrap();
    let gamefiles = brain_job::store::Store::start(pool.clone(), 300)
        .await
        .unwrap();
    let source = brain_job::git::Source::ALL[0];
    let source_id = gamefiles.source(source).await.unwrap();
    let run = gamefiles
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let bytes = br#"{"viscous_goo_grenade":{"AbilityCooldown":26.0}}"#;
    let mut gamefiles_counts = Counts::default();
    gamefiles
        .object(
            &run,
            source,
            "data/json/ability-data.json",
            Some(brain_job::git::Blob {
                bytes: bytes.to_vec(),
                hash: Sha256::digest(bytes).to_vec(),
            }),
            &mut gamefiles_counts,
        )
        .await
        .unwrap();
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&gamefiles_counts, false).await.unwrap();
    store.object(en, "en", b, &mut counts).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_fact WHERE stat_key='ability.AbilityCooldown'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        3
    );
    store.object(en, "en", a, &mut counts).await.unwrap();
    assert_eq!(counts.raw_inserted, 3);
    assert_eq!(counts.raw_reused, 1);
    let actual: (String, String, Vec<u8>, String) = sqlx::query_as("SELECT f.value_num::text,r.locator,r.value_sha256,r.extractor FROM serve.current_fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.source s ON s.id=f.source_id WHERE f.stat_key='ability.AbilityCooldown' AND s.locator=$1").bind(en).fetch_one(&pool).await.unwrap();
    assert_eq!(
        actual,
        (
            "26.0".into(),
            "/0/properties/AbilityCooldown/value".into(),
            Sha256::digest(br#""26.0""#).to_vec(),
            "brain-assets".into()
        )
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_alias WHERE alias_norm='splatter changed'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    let before: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap();
    assert!(store
        .object(
            en,
            "en",
            br#"[{"id":1,"class_name":"bad","type":"ability","name":"a","name":"b"}]"#,
            &mut counts
        )
        .await
        .is_err());
    assert_eq!(sqlx::query_as::<_, (i64,i64,i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap(), before);
    store.finish(&counts, 0, false).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM ops.job_run WHERE job='ingest-assets' AND status='succeeded'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        1
    );
    let fenced = Store::start(pool.clone(), 300).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE ops.job_lease SET epoch=epoch+1 WHERE lease_key='brain-job:ingest-assets'")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert!(fenced.object(en, "en", b, &mut counts).await.is_err());
    assert_eq!(sqlx::query_as::<_, (i64,i64,i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap(), before);
    pool.close().await;
    owner.close().await;
    admin.close().await;
}
