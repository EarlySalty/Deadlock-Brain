use brain_job::{
    ingest, limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, GameFactsStore,
    InputObject, Outcome, RawObject, Receipt, Relation,
};
use brain_storage::PgStore;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool, Row,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

struct Stub(AtomicUsize);
struct Corrected(&'static str);
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
                value: FactValue::Number(100.0),
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

async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn poison_isolation_publication_retry_and_immutable_migration() {
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
    let schema = PgStore::new(admin.clone());
    schema.migrate_gamefacts().await.unwrap();
    schema.migrate_gamefacts().await.unwrap();
    schema.check_gamefacts_schema().await.unwrap();
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
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    let stub = Stub(AtomicUsize::new(0));
    let first = ingest(
        &store,
        "stub",
        [
            input("large", "r1"),
            input("collision", "r1"),
            input("good", "r1"),
        ],
        &[&stub],
    )
    .await
    .unwrap();
    assert_eq!(
        (
            first.counts.accepted,
            first.counts.rejected,
            first.counts.ignored
        ),
        (1, 2, 0)
    );
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(first.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    assert_eq!(
        count(
            &admin,
            "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded' AND finished IS NOT NULL"
        )
        .await,
        1
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM raw.object").await, 3);
    let raw: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE path='good'")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(raw, input("good", "r1").unwrap().bytes);
    let facts_before = count(&admin, "SELECT count(*) FROM canon.fact").await;
    let receipts_before = count(&admin, "SELECT count(*) FROM canon.receipt").await;
    let second = ingest(&store, "stub", [input("good", "r2")], &[&stub])
        .await
        .unwrap();
    assert_eq!((second.counts.accepted, second.counts.ignored), (0, 1));
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.fact").await,
        facts_before
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt").await,
        receipts_before
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM raw.object").await, 3);
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
    let sql_poison = ingest(
        &store,
        "stub",
        [input("sql", "r3"), input("sibling", "r3")],
        &[&stub],
    )
    .await
    .unwrap();
    assert_eq!(
        (sql_poison.counts.accepted, sql_poison.counts.rejected),
        (1, 1)
    );
    assert_eq!(
        count(
            &admin,
            "SELECT count(*) FROM canon.entity WHERE key='hero_sql'"
        )
        .await,
        0
    );
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
        assert_eq!(
            count(
                &admin,
                &format!("SELECT count(*) FROM serve.{projection} WHERE {hidden}")
            )
            .await,
            0
        );
    }
    let retry = ingest(&store, "stub", [input("hidden", "r4")], &[&stub])
        .await
        .unwrap();
    assert_eq!(retry.counts.accepted, 1);
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
    assert_eq!(
        count(&admin, "SELECT count(*) FROM serve.hero_profile").await,
        3
    );
    let corrected = ingest(&store, "stub", [input("large", "r1")], &[&Corrected("1")])
        .await
        .unwrap();
    assert_eq!(
        (corrected.counts.accepted, corrected.counts.rejected),
        (1, 0)
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM raw.object").await, 6);
    let upgraded = ingest(&store, "stub", [input("good", "r2")], &[&Corrected("2")])
        .await
        .unwrap();
    assert_eq!(upgraded.counts.accepted, 1);
    let new_version: String =
        sqlx::query_scalar("SELECT extractor FROM canon.receipt WHERE ingest_run_id=$1 LIMIT 1")
            .bind(upgraded.run_id)
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(new_version, brain_job::extractor_id("stub", "2"));
    let rerun = ingest(&store, "stub", [input("good", "r2")], &[&Corrected("2")])
        .await
        .unwrap();
    assert_eq!((rerun.counts.accepted, rerun.counts.ignored), (0, 1));
    let rejected_again = ingest(&store, "stub", [input("collision", "r1")], &[&stub])
        .await
        .unwrap();
    assert_eq!(rejected_again.counts.rejected, 1);
    let before_empty = count(&admin, "SELECT count(*) FROM canon.receipt").await;
    let facts_before_empty = count(&admin, "SELECT count(*) FROM canon.fact").await;
    let raw_before_empty = count(&admin, "SELECT count(*) FROM raw.object").await;
    let empty = ingest(&store, "stub", [input("empty", "r1")], &[&stub])
        .await
        .unwrap();
    assert_eq!(
        empty.counts,
        brain_job::Counts {
            accepted: 1,
            rejected: 0,
            ignored: 0
        }
    );
    let marker = sqlx::query("SELECT o.id,o.content_sha256,r.extractor,r.locator_kind,r.locator,r.value_sha256 FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE r.ingest_run_id=$1")
        .bind(empty.run_id).fetch_one(&admin).await.unwrap();
    let empty_id = marker.get::<i64, _>("id");
    let discriminator = brain_job::extractor_id("stub", "1");
    assert_eq!(marker.get::<String, _>("extractor"), discriminator);
    assert_eq!(marker.get::<String, _>("locator_kind"), "raw_bytes");
    assert_eq!(marker.get::<String, _>("locator"), "");
    assert_eq!(
        marker.get::<Vec<u8>, _>("value_sha256"),
        marker.get::<Vec<u8>, _>("content_sha256")
    );
    assert!(store.was_applied(empty_id, &discriminator).await.unwrap());
    let calls_after_empty = stub.0.load(Ordering::Relaxed);
    let empty_rerun = ingest(&store, "stub", [input("empty", "r2")], &[&stub])
        .await
        .unwrap();
    assert_eq!(
        empty_rerun.counts,
        brain_job::Counts {
            accepted: 0,
            rejected: 0,
            ignored: 1
        }
    );
    assert_eq!(stub.0.load(Ordering::Relaxed), calls_after_empty);
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt").await,
        before_empty + 1
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM raw.object").await,
        raw_before_empty + 1
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.fact").await,
        facts_before_empty
    );
    let upgraded_discriminator = brain_job::extractor_id("stub", "2");
    assert!(!store
        .was_applied(empty_id, &upgraded_discriminator)
        .await
        .unwrap());
    let empty_upgraded = ingest(&store, "stub", [input("empty", "r2")], &[&Corrected("2")])
        .await
        .unwrap();
    assert_eq!(
        empty_upgraded.counts,
        brain_job::Counts {
            accepted: 1,
            rejected: 0,
            ignored: 0
        }
    );
    assert!(store
        .was_applied(empty_id, &upgraded_discriminator)
        .await
        .unwrap());
    let empty_upgraded_rerun = ingest(&store, "stub", [input("empty", "r2")], &[&Corrected("2")])
        .await
        .unwrap();
    assert_eq!(
        empty_upgraded_rerun.counts,
        brain_job::Counts {
            accepted: 0,
            rejected: 0,
            ignored: 1
        }
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt").await,
        before_empty + 2
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM raw.object").await,
        raw_before_empty + 1
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.fact").await,
        facts_before_empty
    );
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
    assert_eq!(oversized.counts.rejected, 1);
    assert_eq!(stub.0.load(Ordering::Relaxed), calls);
    let row = sqlx::query("SELECT octet_length(o.bytes) AS size,j.code FROM raw.object o JOIN ops.ingest_rejection j ON j.raw_object_id=o.id WHERE j.run_id=$1")
        .bind(oversized.run_id).fetch_one(&admin).await.unwrap();
    assert_eq!(row.get::<i32, _>("size") as usize, limits::RAW_BYTES + 1);
    assert_eq!(row.get::<String, _>("code"), "raw_too_large");
    let loader: brain_job::sources::Source = stream;
    let streamed = ingest(&store, "stub", loader(&pg.directory, false), &[&stub])
        .await
        .unwrap();
    assert_eq!(streamed.counts.accepted, 3);
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
    assert_eq!(
        count(&admin, "SELECT count(*) FROM ops.answer_trace").await,
        1
    );
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
