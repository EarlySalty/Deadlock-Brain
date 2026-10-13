use brain_job::{ingest, limits, registry::EXTRACTORS, GameFactsStore, InputObject};
use brain_storage::PgStore;
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;
fn input(bytes: Vec<u8>, language: &str) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: "assets-checkpoint".into(),
        path: format!("items/{language}.json"),
        bytes,
    })
}
async fn count(pool: &sqlx::PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql)
        .fetch_one(pool)
        .await
        .expect("scratch metadata")
}
#[tokio::test]
async fn actual_assets_poison_isolation_and_subsequent_valid_run() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .expect("scratch DB");
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.expect("scratch roles");
    let schema = PgStore::new(admin.clone());
    schema.migrate_gamefacts().await.expect("frozen migration");
    schema
        .check_gamefacts_schema()
        .await
        .expect("schema contract");
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .expect("ingest role");
    let store = GameFactsStore::new(writer.clone());
    let oversized = serde_json::to_vec(&serde_json::json!({"id":1,"class_name":"a","type":"ability","image":"ü".repeat(limits::TEXT_BYTES)})).expect("poison JSON");
    let collision = br#"{"id":2,"class_name":"b","type":"ability","properties":{"Weapon-Damage":{"value":1},"weapon damage":{"value":2}}}"#.to_vec();
    let first = ingest(
        &store,
        "assets",
        [input(oversized, "en"), input(collision, "en")],
        EXTRACTORS,
    )
    .await
    .expect("poison run completes");
    assert_eq!(first.counts.accepted, 0);
    assert_eq!(first.counts.rejected, 2);
    assert_eq!(first.counts.ignored, 0);
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(first.run_id)
            .fetch_all(&admin)
            .await
            .expect("rejections");
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    let finished =
        "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded' AND finished IS NOT NULL";
    assert_eq!(count(&admin, finished).await, 1);
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.receipt").await, 0);
    let bytes = br#"{"id":4294967295,"class_name":"Valid_Ability","type":"ability","name":"Car\/bine","properties":{"Damage":{"value":"1.2300e+2","postfix":"%","scale_function":{"class_name":"scale_function_tech_damage","stat_scale":0.25}}}}"#.to_vec();
    let german = br#"{"id":4294967295,"class_name":"Valid_Ability","type":"ability","name":"Gewehr","properties":{"Damage":{"value":"999","postfix":"%","scale_function":{"class_name":"scale_function_tech_damage","stat_scale":0.25}}}}"#.to_vec();
    let second = ingest(
        &store,
        "assets",
        [input(bytes.clone(), "en"), input(german.clone(), "de")],
        EXTRACTORS,
    )
    .await
    .expect("valid next run");
    assert_eq!(second.counts.accepted, 2);
    assert_eq!(second.counts.rejected, 0);
    assert_eq!(second.counts.ignored, 0);
    assert_eq!(count(&admin, finished).await, 2);
    assert_eq!(
        count(&admin, "SELECT count(*) FROM ops.ingest_rejection").await,
        2
    );
    let original: Vec<u8> = sqlx::query_scalar(
        "SELECT bytes FROM raw.object WHERE ingest_run_id=$1 AND path='items/en.json'",
    )
    .bind(second.run_id)
    .fetch_one(&admin)
    .await
    .expect("raw bytes");
    assert_eq!(original, bytes);
    let hash: Vec<u8> = sqlx::query_scalar(
        "SELECT p.value_sha256 FROM serve.p2_fact f JOIN serve.published_receipt p ON p.id=(f.fact->>'citation')::bigint WHERE f.stat_key='ability_damage'",
    )
    .fetch_one(&admin)
    .await
    .expect("served numeric receipt");
    assert_eq!(hash, Sha256::digest(br#""1.2300e+2""#).to_vec());
    let alias_hash: Vec<u8> = sqlx::query_scalar(
        "SELECT p.value_sha256 FROM serve.current_alias a JOIN serve.published_receipt p ON p.id=a.receipt_id WHERE a.alias_norm='car/bine'",
    )
    .fetch_one(&admin)
    .await
    .expect("served alias receipt");
    assert_eq!(alias_hash, Sha256::digest(br#""Car\/bine""#).to_vec());
    let fact: serde_json::Value =
        sqlx::query_scalar("SELECT fact FROM serve.p2_fact WHERE stat_key='ability_damage'")
            .fetch_one(&admin)
            .await
            .expect("qualified fact");
    assert_eq!(fact["value"].as_f64(), Some(123.0));
    assert_eq!(fact["unit"], "%");
    assert_eq!(fact["condition"]["scale_value"], "0.25");
    assert!(fact["qualifiers"]
        .as_array()
        .is_some_and(|rows| rows.len() == 5));
    let objects = count(&admin, "SELECT count(*) FROM raw.object").await;
    let facts = count(&admin, "SELECT count(*) FROM canon.fact").await;
    let third = ingest(
        &store,
        "assets",
        [input(bytes, "en"), input(german, "de")],
        EXTRACTORS,
    )
    .await
    .expect("unchanged run");
    assert_eq!(third.counts.accepted, 0);
    assert_eq!(third.counts.rejected, 0);
    assert_eq!(third.counts.ignored, 2);
    assert_eq!(
        count(&admin, "SELECT count(*) FROM raw.object").await,
        objects
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.fact").await,
        facts
    );
    writer.close().await;
    admin.close().await;
}
