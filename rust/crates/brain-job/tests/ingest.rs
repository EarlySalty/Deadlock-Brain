use brain_job::{
    ingest, limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, GameFactsStore,
    InputObject, Outcome, RawObject, Receipt, Relation,
};
use brain_storage::PgStore;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../../brain-storage/tests/support/gamefacts_pg.rs"]
pub mod gamefacts_pg;
use gamefacts_pg::{count, growth as stored_counts, rejection_codes};

struct Stub(AtomicUsize);
struct Corrected(&'static str, AtomicUsize);
static STREAM_EXTRACTED: AtomicUsize = AtomicUsize::new(0);

fn stream(repository: &std::path::Path, fetch: bool) -> brain_job::sources::InputObjects {
    let repository = repository.to_path_buf();
    Box::new((0..3).map(move |index| {
        anyhow::ensure!(!fetch && repository.is_absolute(), "invalid stub input");
        anyhow::ensure!(
            STREAM_EXTRACTED.load(Ordering::SeqCst) == index,
            "next object loaded before previous extraction"
        );
        let saved = std::process::Command::new("/usr/bin/psql")
            .arg("-h").arg(repository.join("socket"))
            .args(["-p", "55439", "-U", "brain_core_test", "-d", "postgres", "-w", "-Atqc",
                "SELECT count(*) FROM canon.fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.key LIKE 'hero_stream%'"])
            .env("PGPASSFILE", "/dev/null").env_remove("PGOPTIONS").env_remove("PGSERVICE")
            .output()?;
        anyhow::ensure!(saved.status.success(), "scratch query failed");
        anyhow::ensure!(String::from_utf8(saved.stdout)?.trim().parse::<usize>()? == index,
            "next object loaded before previous fact was committed");
        input(&format!("stream{index}"), "stream_revision")
    }))
}

impl Extractor for Corrected {
    fn version(&self) -> &'static str {
        self.0
    }
    fn name(&self) -> &'static str {
        "stub"
    }
    fn accepts(&self, source: &str, _: &str) -> bool {
        source == "stub"
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        self.1.fetch_add(1, Ordering::Relaxed);
        let Outcome::Accepted(mut set) = Stub(AtomicUsize::new(0)).extract(raw) else {
            return Outcome::Ignored;
        };
        set.facts.truncate(1);
        if let Some(fact) = set.facts.first_mut() {
            fact.value = FactValue::Number(150.0);
        }
        Outcome::Accepted(set)
    }
}

impl Extractor for Stub {
    fn name(&self) -> &'static str {
        "stub"
    }
    fn accepts(&self, source: &str, _: &str) -> bool {
        source == "stub"
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        self.0.fetch_add(1, Ordering::Relaxed);
        if raw.path.starts_with("stream") {
            STREAM_EXTRACTED.fetch_add(1, Ordering::SeqCst);
        }
        if raw.path == "ignored" {
            return Outcome::Ignored;
        }
        if raw.path == "empty" {
            return Outcome::Accepted(FactSet::default());
        }
        let receipt = Receipt {
            locator_kind: "raw_bytes".into(),
            locator: "".into(),
            value_sha256: Sha256::digest(&raw.bytes).into(),
        };
        let key = format!("hero_{}", raw.path);
        let mut set = FactSet {
            entities: vec![Entity {
                kind: "hero".into(),
                key: key.clone(),
                receipt: receipt.clone(),
            }],
            aliases: vec![Alias {
                entity_key: key.clone(),
                alias: key.clone(),
                lang: "en".into(),
                receipt: receipt.clone(),
            }],
            facts: vec![Fact {
                entity_key: key.clone(),
                stat_key: if raw.path == "sql" {
                    "sql_fail"
                } else {
                    "health"
                }
                .into(),
                value: FactValue::Number(serde_json::from_slice(&raw.bytes).unwrap_or(100.0)),
                receipt: receipt.clone(),
            }],
            relations: vec![Relation {
                from_key: key.clone(),
                kind: "related_to".into(),
                to_key: key,
                receipt,
            }],
        };
        match raw.path.as_str() {
            "large" => set.facts[0].value = FactValue::Text("x".repeat(limits::TEXT_BYTES + 1)),
            "collision" => {
                set.facts[0].stat_key = "Max-Health".into();
                let mut twin = set.facts[0].clone();
                twin.stat_key = "MAX HEALTH".into();
                set.facts.push(twin);
            }
            _ => {}
        }
        Outcome::Accepted(set)
    }
}

fn input(path: &str, revision: &str) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: revision.into(),
        path: path.into(),
        bytes: b"{ \"exact\": 9007199254740993 }\n".to_vec(),
    })
}

async fn run<I>(
    store: &GameFactsStore,
    objects: I,
    extractor: &dyn Extractor,
) -> brain_job::RunReport
where
    I: IntoIterator<Item = anyhow::Result<InputObject>>,
{
    ingest(store, "stub", objects, &[extractor]).await.unwrap()
}

fn assert_counts(report: &brain_job::RunReport, expected: (i64, i64, i64)) {
    let counts = report.counts;
    assert_eq!((counts.accepted, counts.rejected, counts.ignored), expected);
}

async fn assert_count(pool: &PgPool, sql: &str, expected: i64) {
    assert_eq!(count(pool, sql).await, expected);
}

#[tokio::test]
async fn poison_isolation_publication_retry_and_immutable_migration() {
    let (pg, admin, writer) = gamefacts_pg::fixture(1).await;
    let schema = PgStore::new(admin.clone());
    schema.migrate_gamefacts().await.unwrap();
    schema.check_gamefacts_schema().await.unwrap();
    PgStore::new(writer.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    let stub = Stub(AtomicUsize::new(0));
    let first = run(
        &store,
        [
            input("large", "r1"),
            input("collision", "r1"),
            input("good", "r1"),
        ],
        &stub,
    )
    .await;
    assert_counts(&first, (1, 2, 0));
    let reasons = rejection_codes(&admin, first.run_id).await;
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    assert_count(
        &admin,
        "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded' AND finished IS NOT NULL",
        1,
    )
    .await;
    assert_eq!(count(&admin, "SELECT count(*) FROM raw.object").await, 3);
    let raw: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE path='good'")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(raw, input("good", "r1").unwrap().bytes);
    let (_, facts_before, receipts_before) = stored_counts(&admin).await;
    let second = run(&store, [input("good", "r2")], &stub).await;
    assert_counts(&second, (0, 0, 1));
    assert_eq!(
        stored_counts(&admin).await,
        (3, facts_before, receipts_before)
    );
    let revision: String = sqlx::query_scalar(
        "SELECT source_revision FROM serve.current_fact WHERE stat_key='health'",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(revision, "r1");
    sqlx::raw_sql("ALTER TABLE canon.fact ADD CONSTRAINT poison CHECK(stat_key<>'sql_fail')")
        .execute(&admin)
        .await
        .unwrap();
    let sql_poison = run(&store, [input("sql", "r3"), input("sibling", "r3")], &stub).await;
    assert_counts(&sql_poison, (1, 1, 0));
    assert_count(
        &admin,
        "SELECT count(*) FROM canon.entity WHERE key='hero_sql'",
        0,
    )
    .await;
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path='sql'").await, 0);
    let detail: String =
        sqlx::query_scalar("SELECT detail FROM ops.ingest_rejection WHERE run_id=$1")
            .bind(sql_poison.run_id)
            .fetch_one(&admin)
            .await
            .unwrap();
    assert!(detail.contains("23514"));
    let failed = ingest(
        &store,
        "stub",
        [
            input("hidden", "r4"),
            Err(anyhow::anyhow!("upstream unreachable")),
        ],
        &[&stub],
    )
    .await;
    assert!(failed.is_err());
    for projection in [
        "current_entity",
        "current_alias",
        "current_fact",
        "current_relation",
    ] {
        let hidden = if projection == "current_entity" {
            "key='hero_hidden'"
        } else if projection == "current_relation" {
            "from_entity IN(SELECT id FROM canon.entity WHERE key='hero_hidden')"
        } else {
            "entity_id IN(SELECT id FROM canon.entity WHERE key='hero_hidden')"
        };
        assert_count(
            &admin,
            &format!("SELECT count(*) FROM serve.{projection} WHERE {hidden}"),
            0,
        )
        .await;
    }
    let retry = run(&store, [input("hidden", "r4")], &stub).await;
    assert_counts(&retry, (1, 0, 0));
    assert_eq!(stub.0.load(Ordering::Relaxed), 7);
    let crashed = store.begin_run("stub").await.unwrap();
    let crash = sqlx::query(
        "SELECT status,finished IS NOT NULL AS terminal FROM ops.ingest_run WHERE id=$1",
    )
    .bind(crashed)
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(crash.get::<String, _>("status"), "failed");
    assert!(crash.get::<bool, _>("terminal"));
    assert_count(&admin, "SELECT count(*) FROM serve.hero_profile", 3).await;
    let corrected_v1 = Corrected("1", AtomicUsize::new(0));
    let corrected = run(&store, [input("large", "r1")], &corrected_v1).await;
    assert_counts(&corrected, (1, 0, 0));
    assert_eq!(count(&admin, "SELECT count(*) FROM raw.object").await, 6);
    let corrected_v2 = Corrected("2", AtomicUsize::new(0));
    let upgraded = run(&store, [input("good", "r2")], &corrected_v2).await;
    assert_counts(&upgraded, (1, 0, 0));
    let new_version: String =
        sqlx::query_scalar("SELECT extractor FROM canon.receipt WHERE ingest_run_id=$1 LIMIT 1")
            .bind(upgraded.run_id)
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(new_version, brain_job::extractor_id("stub", "2"));
    let rerun = run(&store, [input("good", "r2")], &corrected_v2).await;
    assert_counts(&rerun, (0, 0, 1));
    let rejected_again = run(&store, [input("collision", "r1")], &stub).await;
    assert_counts(&rejected_again, (0, 1, 0));
    let (raw_before_empty, facts_before_empty, before_empty) = stored_counts(&admin).await;
    for (extractor, version, revision, counter, added_receipts) in [
        (&stub as &dyn Extractor, "1", "r1", &stub.0, 1),
        (&corrected_v2, "2", "r2", &corrected_v2.1, 2),
    ] {
        let discriminator = brain_job::extractor_id("stub", version);
        if version == "2" {
            let empty_id = sqlx::query_scalar("SELECT id FROM raw.object WHERE path='empty'")
                .fetch_one(&admin)
                .await
                .unwrap();
            assert!(!store.was_applied(empty_id, &discriminator).await.unwrap());
        }
        let empty = run(&store, [input("empty", revision)], extractor).await;
        assert_counts(&empty, (1, 0, 0));
        let marker = sqlx::query("SELECT o.id,o.bytes,o.content_sha256,r.extractor,r.locator_kind,r.locator,r.value_sha256 FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE r.ingest_run_id=$1 AND o.path='empty'")
            .bind(empty.run_id).fetch_one(&admin).await.unwrap();
        let empty_id = marker.get::<i64, _>("id");
        assert_eq!(marker.get::<String, _>("extractor"), discriminator);
        assert_eq!(marker.get::<String, _>("locator_kind"), "raw_bytes");
        assert_eq!(marker.get::<String, _>("locator"), "");
        let hash = Sha256::digest(marker.get::<Vec<u8>, _>("bytes")).to_vec();
        assert_eq!(marker.get::<Vec<u8>, _>("value_sha256"), hash);
        assert_eq!(marker.get::<Vec<u8>, _>("content_sha256"), hash);
        assert!(store.was_applied(empty_id, &discriminator).await.unwrap());
        let calls_after_empty = counter.load(Ordering::Relaxed);
        let empty_rerun = run(&store, [input("empty", "r2")], extractor).await;
        assert_counts(&empty_rerun, (0, 0, 1));
        assert_eq!(counter.load(Ordering::Relaxed), calls_after_empty);
        assert_eq!(
            stored_counts(&admin).await,
            (
                raw_before_empty + 1,
                facts_before_empty,
                before_empty + added_receipts
            )
        );
    }
    let calls = stub.0.load(Ordering::Relaxed);
    let oversized = ingest(
        &store,
        "stub",
        [Ok(InputObject {
            revision: "r1".into(),
            path: "oversized".into(),
            bytes: vec![b'x'; limits::RAW_BYTES + 1],
        })],
        &[&stub],
    )
    .await
    .unwrap();
    assert_counts(&oversized, (0, 1, 0));
    assert_eq!(stub.0.load(Ordering::Relaxed), calls);
    let row = sqlx::query("SELECT octet_length(o.bytes) AS size,j.code FROM raw.object o JOIN ops.ingest_rejection j ON j.raw_object_id=o.id WHERE j.run_id=$1")
        .bind(oversized.run_id).fetch_one(&admin).await.unwrap();
    assert_eq!(row.get::<i32, _>("size") as usize, limits::RAW_BYTES + 1);
    assert_eq!(row.get::<String, _>("code"), "raw_too_large");
    let loader: brain_job::sources::Source = stream;
    let streamed = ingest(&store, "stub", loader(&pg.directory, false), &[&stub])
        .await
        .unwrap();
    assert_counts(&streamed, (3, 0, 0));
    assert_eq!(STREAM_EXTRACTED.load(Ordering::SeqCst), 3);
    let missing = brain_job::sources::SOURCES[0].1(&pg.directory.join("missing"), false);
    assert!(ingest(&store, "deadlock-data", missing, &[&stub])
        .await
        .is_err());
    assert_eq!(count(&admin, "SELECT count(*) FROM ops.ingest_run WHERE source='deadlock-data' AND status='failed' AND finished IS NOT NULL").await, 1);
    let mut trace = admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_service")
        .execute(&mut *trace)
        .await
        .unwrap();
    sqlx::query("INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms) VALUES($1,'[]','[]','answered','{}',1)")
        .bind(vec![0_u8; 32]).execute(&mut *trace).await.unwrap();
    trace.commit().await.unwrap();
    assert_count(&admin, "SELECT count(*) FROM ops.answer_trace", 1).await;
    let mut denied = admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_service")
        .execute(&mut *denied)
        .await
        .unwrap();
    assert!(sqlx::query("SELECT * FROM ops.answer_trace")
        .fetch_all(&mut *denied)
        .await
        .is_err());
    denied.rollback().await.unwrap();
    for (path, published, values, expected) in [
        ("twins_published", true, [125, 120], 120.0),
        ("twins_fresh", false, [120, 125], 125.0),
    ] {
        let numeric = |value: i32| {
            let mut object = input(path, "r1").unwrap();
            object.bytes = value.to_string().into_bytes();
            Ok(object)
        };
        if published {
            assert_counts(&run(&store, [numeric(120)], &stub).await, (1, 0, 0));
        }
        let twins = run(&store, values.map(numeric), &stub).await;
        assert_counts(&twins, (2, 0, 0));
        let health: f64 = sqlx::query_scalar("SELECT value_num FROM serve.current_fact f JOIN serve.current_entity e ON e.id=f.entity_id WHERE e.key=$1 AND stat_key='health'").bind(format!("hero_{path}")).fetch_one(&admin).await.unwrap();
        assert_eq!(health, expected);
        let before = stored_counts(&admin).await;
        assert_counts(
            &run(&store, [numeric(expected as i32)], &stub).await,
            (0, 0, 1),
        );
        assert_eq!(stored_counts(&admin).await, before);
    }
    assert_counts(
        &run(&store, [input("ignored", "r1")], &stub).await,
        (0, 0, 1),
    );
    sqlx::query("UPDATE ops.gamefacts_schema_version SET checksum=$1 WHERE version=1")
        .bind(vec![0_u8; 32])
        .execute(&admin)
        .await
        .unwrap();
    assert!(schema.check_gamefacts_schema().await.is_err());
    assert!(schema.migrate_gamefacts().await.is_err());
    writer.close().await;
    admin.close().await;
}

#[test]
fn bounds_and_normalized_collisions_match_the_contract() {
    assert_eq!(brain_job::extractor_id("stub", "1"), "v1:4:stub:1:1");
    assert_eq!(brain_job::extractor_id("é:a", ":1"), "v1:4:é:a:2::1");
    assert_ne!(
        brain_job::extractor_id("a:b", "c"),
        brain_job::extractor_id("a", "b:c")
    );
    let raw = RawObject {
        id: 1,
        source: "stub".into(),
        revision: "r1".into(),
        path: "good".into(),
        content_sha256: [0; 32],
        bytes: b"{}".to_vec(),
    };
    let Outcome::Accepted(mut set) = Stub(AtomicUsize::new(0)).extract(&raw) else {
        panic!()
    };
    assert!(limits::validate(&set).is_ok());
    for value in [f64::NAN, f64::INFINITY, limits::NUMBER_ABS * 2.0] {
        set.facts[0].value = FactValue::Number(value);
        assert_eq!(limits::validate(&set).unwrap_err().code, "invalid_number");
    }
    set.facts[0].value = FactValue::Number(limits::NUMBER_ABS);
    assert!(limits::validate(&set).is_ok());
    set.aliases[0].alias = "x".repeat(limits::ALIAS_BYTES + 1);
    assert_eq!(limits::validate(&set).unwrap_err().code, "value_too_large");
    let rejection = brain_job::Rejection::new("test", "ä".repeat(400));
    assert_eq!(rejection.detail.len(), limits::DETAIL_BYTES);
}

#[test]
fn receipt_hashes_use_raw_json_tokens_without_float_roundtrips() {
    let bytes =
        br#"{ "exact": 9007199254740993, "escaped": "ab", "nested": [{"a/b~c":1.2300e+2}] }"#;
    for (pointer, token) in [
        ("/exact", "9007199254740993"),
        ("/escaped", r#""ab""#),
        ("/nested/0/a~1b~0c", "1.2300e+2"),
    ] {
        let receipt = Receipt {
            locator_kind: "json_pointer".into(),
            locator: pointer.into(),
            value_sha256: Sha256::digest(token.as_bytes()).into(),
        };
        let value = brain_storage::gamefacts::receipts::value_bytes(bytes, &receipt).unwrap();
        assert_eq!(value, token.as_bytes());
    }
}

#[test]
#[ignore = "requires the read-only deadlock-data cache at the attested revision"]
fn cached_hero_source_yields_one_unchanged_original_blob() {
    let repository = std::path::PathBuf::from(
        std::env::var_os("BRAIN_CORE2_HERO_REPOSITORY").expect("cache path required"),
    );
    let expected = std::fs::read(repository.join("data/json/hero-data.json")).unwrap();
    let mut objects = brain_job::sources::SOURCES[0].1(&repository, false);
    drop(repository);
    let raw = objects.next().unwrap().unwrap();
    assert_eq!(raw.revision, "2a26e26535a390c309e300e2c7b2763fa5ad7abd");
    assert_eq!(raw.path, "data/json/hero-data.json");
    assert_eq!(raw.bytes.len(), 226_182);
    assert_eq!(raw.bytes, expected);
    assert!(objects.next().is_none());
}
