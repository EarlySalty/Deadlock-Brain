use brain_job::{
    ingest, registry::EXTRACTORS, timers::TimerExtractor, Extractor, FactValue, GameFactsStore,
    InputObject, Outcome, RawObject,
};
use brain_storage::{gamefacts::receipts, PgStore};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Row,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const PATH: &str = "data/json/generic-data.json";

fn input(value: &str) -> InputObject {
    InputObject {
        revision: "public-fixture".into(),
        path: PATH.into(),
        bytes: format!("{{\"RejuvParams\":{{\"RejuvinatorBuffDuration\":{value},\"RejuvinatorDropDuration\":[]}}}}")
            .into_bytes(),
    }
}

#[test]
fn actual_extractor_preserves_numeric_text_conditions_and_verified_receipts() {
    let object = input("0.12345678901234567890123456789");
    let raw = RawObject {
        id: 1,
        source: "deadlock-data".into(),
        revision: object.revision,
        path: object.path,
        content_sha256: Sha256::digest(&object.bytes).into(),
        bytes: object.bytes,
    };
    let Outcome::Accepted(set) = TimerExtractor.extract(&raw) else {
        panic!("valid timer object was not accepted")
    };
    assert_eq!(set.entities.len(), 1);
    assert_eq!(set.facts.len(), 1);
    receipts::verify(&raw.bytes, &set).unwrap();
    let FactValue::Text(text) = &set.facts[0].value else {
        panic!("source number must not be stored as a rounded float")
    };
    let payload: Value = serde_json::from_str(text).unwrap();
    assert_eq!(payload["value"], "0.12345678901234567890123456789");
    assert_eq!(payload["unit"], "s");
    assert_eq!(payload["condition"]["source_entity"], "RejuvParams");
    assert_eq!(payload["condition"]["flags"], serde_json::json!([]));
    assert_eq!(
        set.facts[0].receipt.locator,
        "/RejuvParams/RejuvinatorBuffDuration"
    );
    assert!(matches!(
        TimerExtractor.extract(&RawObject {
            source: "unrelated".into(),
            ..raw
        }),
        Outcome::Ignored
    ));
}

#[tokio::test]
async fn timer_poison_objects_reject_atomically_and_the_real_run_succeeds() {
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
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    let large = input(&format!(
        "1.{}",
        "1".repeat(brain_job::limits::TEXT_BYTES + 1)
    ));
    let collision = InputObject {
        revision: "public-fixture".into(),
        path: PATH.into(),
        bytes: br#"{"RejuvParams":{"RejuvinatorBuffDuration":120,"rejuvinatorbuffduration":180}}"#
            .to_vec(),
    };
    let mut good = input("120.00");
    good.bytes =
        br#"{"RejuvParams":{"RejuvinatorBuffDuration":120.00,"RejuvinatorDropDuration":6}}"#
            .to_vec();
    let expected_bytes = good.bytes.clone();
    let report = ingest(
        &store,
        "deadlock-data",
        [Ok(large), Ok(collision), Ok(good)],
        EXTRACTORS,
    )
    .await
    .unwrap();
    assert_eq!(
        (
            report.counts.accepted,
            report.counts.rejected,
            report.counts.ignored
        ),
        (1, 2, 0)
    );
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(report.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    let status: String = sqlx::query_scalar(
        "SELECT status FROM ops.ingest_run WHERE id=$1 AND finished IS NOT NULL",
    )
    .bind(report.run_id)
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(status, "succeeded");
    let counts: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)")
        .fetch_one(&admin).await.unwrap();
    assert_eq!(counts, (3, 2, 4));
    let row = sqlx::query("SELECT f.value_text,r.value_sha256,o.bytes,o.revision FROM serve.current_fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE f.stat_key='timer_buff_duration_seconds'")
        .fetch_one(&admin).await.unwrap();
    assert_eq!(row.get::<Vec<u8>, _>("bytes"), expected_bytes);
    assert_eq!(
        row.get::<Vec<u8>, _>("value_sha256"),
        Sha256::digest(b"120.00").to_vec()
    );
    let payload: Value = serde_json::from_str(&row.get::<String, _>("value_text")).unwrap();
    assert_eq!(payload["value"], "120.00");
    assert_eq!(row.get::<String, _>("revision"), "public-fixture");
    let next = ingest(&store, "deadlock-data", [Ok(input("125"))], EXTRACTORS)
        .await
        .unwrap();
    assert_eq!((next.counts.accepted, next.counts.rejected), (1, 0));
    let mut reader = admin.begin().await.unwrap();
    sqlx::raw_sql(
        "SET TRANSACTION READ ONLY; SET LOCAL ROLE brain_tool; SET LOCAL statement_timeout='500ms'",
    )
    .execute(&mut *reader)
    .await
    .unwrap();
    let query = include_str!("../../brain-serve/src/timer_query.sql");
    let facts: Vec<Value> = sqlx::query_scalar(query)
        .bind("rejuvenator")
        .fetch_all(&mut *reader)
        .await
        .unwrap();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["value"], "125");
    assert_eq!(facts[0]["unit"], "s");
    assert_eq!(facts[0]["condition"]["source_entity"], "RejuvParams");
    assert!(facts[0]["citation"].as_str().is_some());
    assert_eq!(facts[0]["source"], "deadlock-data");
    assert_eq!(facts[0]["patch"], Value::Null);
    let urn: Vec<Value> = sqlx::query_scalar(query)
        .bind("urn")
        .fetch_all(&mut *reader)
        .await
        .unwrap();
    assert!(urn.is_empty());
    reader.commit().await.unwrap();
    writer.close().await;
    admin.close().await;
}
