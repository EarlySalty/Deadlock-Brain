use brain_job::{
    ingest, registry::EXTRACTORS, timers::TimerExtractor, Extractor, FactValue, GameFactsStore,
    InputObject, Outcome, RawObject,
};
use brain_storage::{gamefacts::receipts, PgStore};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool, Row,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const PATH: &str = "data/json/generic-data.json";

fn counts(report: &brain_job::RunReport) -> (i64, i64, i64) {
    (
        report.counts.accepted,
        report.counts.rejected,
        report.counts.ignored,
    )
}

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
    for prefix in [&[0xef, 0xbb, 0xbf][..], &[0xc2, 0xa0][..]] {
        let mut bytes = prefix.to_vec();
        bytes.extend_from_slice(&raw.bytes);
        let bom = RawObject {
            content_sha256: Sha256::digest(&bytes).into(),
            bytes,
            ..raw.clone()
        };
        assert!(matches!(TimerExtractor.extract(&bom), Outcome::Rejected(_)));
    }
    assert!(matches!(
        TimerExtractor.extract(&RawObject {
            source: "unrelated".into(),
            ..raw
        }),
        Outcome::Ignored
    ));
}

struct Database {
    _pg: scratch_pg::ScratchPg,
    admin: PgPool,
    writer: PgPool,
}

async fn raw_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn timer_reader(pool: &PgPool) -> sqlx::Transaction<'_, sqlx::Postgres> {
    let mut reader = pool.begin().await.unwrap();
    sqlx::raw_sql(
        "SET TRANSACTION READ ONLY; SET LOCAL ROLE brain_tool; SET LOCAL statement_timeout='500ms'",
    )
    .execute(&mut *reader)
    .await
    .unwrap();
    reader
}

async fn database() -> Database {
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
    Database {
        _pg: pg,
        admin,
        writer,
    }
}

#[tokio::test]
async fn timer_poison_objects_reject_atomically_and_the_real_run_succeeds() {
    let db = database().await;
    let admin = db.admin.clone();
    let writer = db.writer.clone();
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
    assert_eq!(counts(&report), (1, 2, 0));
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(report.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(reasons, ["invalid_number", "key_collision"]);
    let status: String = sqlx::query_scalar(
        "SELECT status FROM ops.ingest_run WHERE id=$1 AND finished IS NOT NULL",
    )
    .bind(report.run_id)
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(status, "succeeded");
    let stored_counts: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)")
        .fetch_one(&admin).await.unwrap();
    assert_eq!(stored_counts, (3, 2, 4));
    let repeat = InputObject {
        revision: "unchanged-content-new-revision".into(),
        path: PATH.into(),
        bytes: expected_bytes.clone(),
    };
    let repeat = ingest(&store, "deadlock-data", [Ok(repeat)], EXTRACTORS)
        .await
        .unwrap();
    assert_eq!(counts(&repeat), (0, 0, 1));
    assert_eq!(raw_count(&admin).await, 3);
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
    let mut reader = timer_reader(&admin).await;
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
    for (wood, expected) in [
        (
            r#", "citadel_breakable_prop_wooden_crate":{"RespawnTime":90}"#,
            2,
        ),
        ("", 1),
    ] {
        let object = InputObject {
            revision: format!("variant-{expected}"),
            path: "data/json/misc-data.json".into(),
            bytes: format!(r#"{{"citadel_breakable_prop_drop_gold":{{"InitialSpawnTime":300}},"neutral_camp_midboss":{{"SpawnIntervalInSeconds":420}}{wood}}}"#).into_bytes(),
        };
        let report = ingest(&store, "deadlock-data", [Ok(object)], EXTRACTORS)
            .await
            .unwrap();
        assert_eq!(counts(&report), (1, 0, 0));
        let facts: Vec<Value> = sqlx::query_scalar(query)
            .bind("soul_box")
            .fetch_all(&mut *reader)
            .await
            .unwrap();
        assert_eq!(facts.len(), expected);
    }
    reader.commit().await.unwrap();
    writer.close().await;
    admin.close().await;
}

#[tokio::test]
#[ignore = "requires a pinned public GameTracking repository in P3_TIMER_REPOSITORY"]
async fn pinned_public_timers_have_verified_receipts_and_bounded_reads() {
    let repository =
        std::env::var_os("P3_TIMER_REPOSITORY").expect("pinned public repository required");
    let repository = std::path::Path::new(&repository);
    let db = database().await;
    let store = GameFactsStore::new(db.writer.clone());
    let loader = brain_job::sources::SOURCES
        .iter()
        .find(|(name, _)| *name == "gametracking")
        .unwrap()
        .1;
    let first = ingest(
        &store,
        "gametracking",
        loader(repository, false),
        EXTRACTORS,
    )
    .await
    .unwrap();
    assert_eq!(counts(&first), (2, 0, 0));
    let before = raw_count(&db.admin).await;
    let repeat = ingest(
        &store,
        "gametracking",
        loader(repository, false),
        EXTRACTORS,
    )
    .await
    .unwrap();
    assert_eq!(counts(&repeat), (0, 0, 2));
    assert_eq!(raw_count(&db.admin).await, before);
    sqlx::raw_sql("ANALYZE").execute(&db.admin).await.unwrap();
    let mut reader = timer_reader(&db.admin).await;
    let query = include_str!("../../brain-serve/src/timer_query.sql");
    for timer in ["soul_box", "mid_boss", "rejuvenator"] {
        let facts: Vec<Value> = sqlx::query_scalar(query)
            .bind(timer)
            .fetch_all(&mut *reader)
            .await
            .unwrap();
        assert!(!facts.is_empty());
        assert!(serde_json::to_vec(&facts).unwrap().len() < 16_384);
        assert!(facts
            .iter()
            .all(|f| f["citation"].as_str().is_some() && f["patch"].is_null()));
    }
    assert!(sqlx::query_scalar::<_, Value>(query)
        .bind("urn")
        .fetch_all(&mut *reader)
        .await
        .unwrap()
        .is_empty());
    let plan: Vec<String> = sqlx::query_scalar(&format!("EXPLAIN (ANALYZE, BUFFERS) {query}"))
        .bind("soul_box")
        .fetch_all(&mut *reader)
        .await
        .unwrap();
    println!("P3_PUBLIC_PLAN\n{}", plan.join("\n"));
    reader.commit().await.unwrap();
    db.writer.close().await;
    db.admin.close().await;
}
