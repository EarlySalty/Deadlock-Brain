use brain_job::{
    ingest, receipt_audit, Alias, Entity, Extractor, Fact, FactSet, FactValue, GameFactsStore,
    InputObject, Outcome, RawObject, Receipt, Relation,
};
use brain_storage::{gamefacts::receipts, PgStore};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;
struct Fixture;
impl Extractor for Fixture {
    fn name(&self) -> &'static str {
        "audit_fixture"
    }
    fn accepts(&self, _: &str, _: &str) -> bool {
        true
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        let key_path = if raw.path.starts_with("boundary_") {
            raw.path.rsplit_once('_').unwrap().0
        } else {
            &raw.path
        };
        let key = format!("hero_{key_path}");
        let raw_receipt = Receipt {
            locator_kind: "raw_bytes".into(),
            locator: "".into(),
            value_sha256: Sha256::digest(&raw.bytes).into(),
        };
        let mut r = Receipt {
            locator_kind: "json_pointer".into(),
            locator: "/x".into(),
            value_sha256: [0; 32],
        };
        r.value_sha256 = Sha256::digest(receipts::value_bytes(&raw.bytes, &r).unwrap()).into();
        let mut set = FactSet {
            entities: vec![Entity {
                kind: "hero".into(),
                key: key.clone(),
                receipt: raw_receipt.clone(),
            }],
            facts: vec![Fact {
                entity_key: key.clone(),
                stat_key: "health".into(),
                value: FactValue::Number(number(&raw.bytes)),
                receipt: r,
            }],
            ..FactSet::default()
        };
        if raw.path == "corrupt" || raw.path.starts_with("boundary_") {
            set.aliases.push(Alias {
                entity_key: key.clone(),
                alias: "Testheld".into(),
                lang: "de".into(),
                receipt: raw_receipt.clone(),
            });
            set.relations.push(Relation {
                from_key: key.clone(),
                kind: "related_to".into(),
                to_key: key,
                receipt: raw_receipt,
            });
        }
        if matches!(
            raw.path.as_str(),
            "boundary_same_old" | "boundary_same_rejected_old"
        ) {
            set.facts.clear();
        }
        if raw.path == "bulk" {
            set.facts = (0..201)
                .map(|i| Fact {
                    stat_key: format!("stat_{i}"),
                    ..set.facts[0].clone()
                })
                .collect();
        }
        Outcome::Accepted(set)
    }
}
const ONE: &[u8] = br#"{"x":1}"#;
fn number(bytes: &[u8]) -> f64 {
    serde_json::from_slice::<serde_json::Value>(bytes).unwrap()["x"]
        .as_f64()
        .unwrap()
}
async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}
async fn seed(store: &GameFactsStore, paths: &[&str], bytes: &[u8]) {
    let report = ingest(
        store,
        "audit_fixture",
        paths.iter().map(|p| {
            Ok(InputObject {
                revision: "r1".into(),
                path: (*p).into(),
                bytes: bytes.to_vec(),
            })
        }),
        &[&Fixture],
    )
    .await
    .unwrap();
    assert_eq!(report.counts.accepted, i64::try_from(paths.len()).unwrap());
    assert_eq!(report.counts.rejected, 0);
}
async fn role_visibility(admin: &PgPool, receipt_id: i64) {
    for role in ["brain_tool", "brain_service"] {
        let mut tx = admin.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL ROLE {role}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let visible: i64 =
            sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE id=$1")
                .bind(receipt_id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();
        assert_eq!(visible, 0);
        let visible: i64 =
            sqlx::query_scalar("SELECT count(*) FROM serve.current_fact WHERE receipt_id=$1")
                .bind(receipt_id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();
        assert_eq!(visible, 0);
        let profile:serde_json::Value=sqlx::query_scalar("SELECT stats FROM serve.hero_profile WHERE entity_id=(SELECT id FROM serve.current_entity WHERE key='hero_corrupt')").fetch_one(&mut *tx).await.unwrap();
        assert_eq!(profile["facts"], serde_json::json!([]));
        tx.rollback().await.unwrap();
    }
}
async fn drift_fact(admin: &PgPool, value: f64) -> i64 {
    sqlx::query_scalar("SELECT f.receipt_id FROM canon.fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.key='hero_drift' AND f.value_num=$1")
        .bind(value).fetch_one(admin).await.unwrap()
}
async fn drift_serving(admin: &PgPool) -> (Option<f64>, Option<f64>) {
    let fact = sqlx::query_scalar("SELECT value_num FROM serve.current_fact WHERE entity_id=(SELECT id FROM canon.entity WHERE key='hero_drift')")
        .fetch_optional(admin).await.unwrap();
    let profile: Option<serde_json::Value> = sqlx::query_scalar("SELECT stats->'facts'->0->'value' FROM serve.hero_profile WHERE entity_id=(SELECT id FROM canon.entity WHERE key='hero_drift')")
        .fetch_optional(admin).await.unwrap();
    (fact, profile.and_then(|v| v.as_f64()))
}
async fn stale_value_stays_hidden(admin: &PgPool, writer: &PgPool, store: &GameFactsStore) {
    seed(store, &["drift"], br#"{"x":500}"#).await;
    seed(store, &["drift"], br#"{"x":550}"#).await;
    assert_eq!(drift_serving(admin).await, (Some(550.0), Some(550.0)));
    let older = drift_fact(admin, 500.0).await;
    let newest = drift_fact(admin, 550.0).await;
    let corrupt = br#"{"x":551}"#;
    sqlx::query("UPDATE raw.object SET bytes=$1,content_sha256=$2 WHERE id=(SELECT raw_object_id FROM canon.receipt WHERE id=$3)")
        .bind(corrupt.as_slice()).bind(Sha256::digest(corrupt).as_slice()).bind(newest).execute(admin).await.unwrap();
    let report = receipt_audit::run(writer).await.unwrap();
    let mismatches = format!(
        "SELECT count(*) FROM ops.receipt_audit WHERE outcome='mismatch' AND run_id={}",
        report.run_id
    );
    assert_eq!(count(admin, &mismatches).await, 2);
    assert_eq!(drift_serving(admin).await, (None, None));
    assert_eq!(count(admin, "SELECT count(*) FROM canon.fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.key='hero_drift'").await, 2);
    assert_eq!(count(admin, &format!("SELECT count(*) FROM canon.receipt_quarantine WHERE cause='shadowed' AND audited_receipt_id={newest} AND receipt_id={older}")).await, 1);
    assert_eq!(
        count(
            admin,
            "SELECT count(*) FROM canon.receipt_quarantine WHERE cause='shadowed'"
        )
        .await,
        2
    );
    sqlx::query("WITH r AS(INSERT INTO ops.receipt_audit_run DEFAULT VALUES RETURNING id) INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome) SELECT id,$1,'locator_not_found' FROM r")
        .bind(older).execute(admin).await.unwrap();
    assert_eq!(count(admin, &format!("SELECT count(*) FROM canon.receipt_quarantine WHERE cause='audited' AND receipt_id={older}")).await, 1);
    seed(store, &["drift"], br#"{"x":600}"#).await;
    assert_eq!(drift_serving(admin).await, (Some(600.0), Some(600.0)));
}
async fn apply_fixture(store: &GameFactsStore, run: i64, path: &str, bytes: &[u8]) -> i64 {
    let raw = store
        .capture(run, "audit_fixture", "r1", path, bytes.to_vec())
        .await
        .unwrap();
    let Outcome::Accepted(set) = Fixture.extract(&raw) else {
        panic!("fixture rejected")
    };
    store.apply(run, &raw, Fixture.name(), &set).await.unwrap();
    raw.id
}
async fn boundary_visibility(admin: &PgPool, key: &str, expected: i64) {
    let entity: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
        .bind(key)
        .fetch_one(admin)
        .await
        .unwrap();
    for role in ["brain_tool", "brain_service"] {
        let mut tx = admin.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL ROLE {role}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        for view in [
            "current_entity",
            "current_fact",
            "current_alias",
            "current_relation",
        ] {
            let sql = if view == "current_entity" {
                "SELECT count(*) FROM serve.current_entity WHERE id=$1".to_owned()
            } else {
                let field = if view == "current_relation" {
                    "from_entity"
                } else {
                    "entity_id"
                };
                format!("SELECT count(*) FROM serve.{view} WHERE {field}=$1")
            };
            let visible: i64 = sqlx::query_scalar(&sql)
                .bind(entity)
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(visible, expected, "{role} {view}");
        }
        tx.rollback().await.unwrap();
    }
}
async fn rank_and_late_arrival_boundaries(admin: &PgPool, writer: &PgPool, store: &GameFactsStore) {
    for (late, object_rejected) in [(false, false), (true, false), (false, true), (true, true)] {
        let prefix = match (late, object_rejected) {
            (false, false) => "boundary_same",
            (true, false) => "boundary_late",
            (false, true) => "boundary_same_rejected",
            (true, true) => "boundary_late_rejected",
        };
        let key = format!("hero_{prefix}");
        let old_run = store.begin_run("audit_fixture").await.unwrap();
        if !late {
            apply_fixture(store, old_run, &format!("{prefix}_old"), br#"{"x":500}"#).await;
        }
        let new_run = if late {
            store.begin_run("audit_fixture").await.unwrap()
        } else {
            old_run
        };
        let newest = apply_fixture(store, new_run, &format!("{prefix}_new"), br#"{"x":550}"#).await;
        store
            .finish_run(
                new_run,
                true,
                brain_job::Counts {
                    accepted: if late { 1 } else { 2 },
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        boundary_visibility(admin, &key, 1).await;
        let corrupt_sql = if object_rejected {
            "UPDATE raw.object SET content_sha256=$1 WHERE id=$2"
        } else {
            "UPDATE canon.receipt SET value_sha256=$1 WHERE raw_object_id=$2"
        };
        sqlx::query(corrupt_sql)
            .bind(vec![0u8; 32])
            .bind(newest)
            .execute(admin)
            .await
            .unwrap();
        let report = receipt_audit::run(writer).await.unwrap();
        let outcome = if object_rejected {
            "object_rejected"
        } else {
            "mismatch"
        };
        assert_eq!(
            count(
                admin,
                &format!(
                    "SELECT count(*) FROM ops.receipt_audit WHERE run_id={} AND outcome='{outcome}'",
                    report.run_id
                )
            )
            .await,
            4
        );
        assert_eq!(report.rejected_objects, i32::from(object_rejected));
        boundary_visibility(admin, &key, 0).await;
        if late {
            apply_fixture(store, old_run, &format!("{prefix}_old"), br#"{"x":500}"#).await;
            store
                .finish_run(
                    old_run,
                    true,
                    brain_job::Counts {
                        accepted: 1,
                        ..Default::default()
                    },
                )
                .await
                .unwrap();
            boundary_visibility(admin, &key, 0).await;
        }
        assert_eq!(count(admin, &format!("SELECT count(*) FROM serve.published_receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path LIKE '{prefix}_%' ")).await, 0);
        assert_eq!(count(admin, &format!("SELECT count(*) FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path LIKE '{prefix}_%' ")).await, if late { 8 } else { 7 });
        seed(store, &[&format!("{prefix}_healthy")], br#"{"x":600}"#).await;
        boundary_visibility(admin, &key, 1).await;
    }
}
#[tokio::test]
async fn real_store_quarantine_poison_continuation_idempotence_and_cap() {
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
    schema.check_gamefacts_schema().await.unwrap();
    assert_eq!(
        count(
            &admin,
            "SELECT count(*) FROM ops.gamefacts_schema_version WHERE version=6"
        )
        .await,
        1
    );
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest")
                    .execute(&mut *c)
                    .await?;
                sqlx::query("SET default_transaction_isolation='repeatable read'")
                    .execute(c)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    seed(&store, &["corrupt"], ONE).await;
    let corrupt: i64 =
        sqlx::query_scalar("SELECT receipt_id FROM canon.fact WHERE stat_key='health'")
            .fetch_one(&admin)
            .await
            .unwrap();
    sqlx::query("UPDATE canon.receipt SET value_sha256=$1 WHERE id=$2")
        .bind(vec![0u8; 32])
        .bind(corrupt)
        .execute(&admin)
        .await
        .unwrap();
    let first = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            first.sampled,
            first.verified_receipts,
            first.rejected_receipts,
            first.rejected_objects
        ),
        (4, 3, 1, 0)
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt_quarantine").await,
        1
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.fact").await, 1);
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.receipt").await, 4);
    role_visibility(&admin, corrupt).await;
    for view in ["current_entity", "current_alias", "current_relation"] {
        assert_eq!(
            count(&admin, &format!("SELECT count(*) FROM serve.{view}")).await,
            1
        );
    }
    let second = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            second.sampled,
            second.verified_receipts,
            second.rejected_receipts
        ),
        (3, 3, 0)
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt_quarantine").await,
        1
    );
    seed(
        &store,
        &[
            "oversize",
            "collision",
            "missing",
            "good",
            "unsupported",
            "contenthash",
            "invalid_utf8",
            "parser",
        ],
        ONE,
    )
    .await;
    sqlx::query(
        "UPDATE raw.object SET bytes=convert_to(repeat('x',$1),'UTF8') WHERE path='oversize'",
    )
    .bind(i32::try_from(brain_job::limits::RAW_BYTES + 1).unwrap())
    .execute(&admin)
    .await
    .unwrap();
    let collision = br#"{"x":1,"x":2}"#;
    sqlx::query("UPDATE raw.object SET bytes=$1,content_sha256=$2 WHERE path='collision'")
        .bind(collision.as_slice())
        .bind(Sha256::digest(collision).as_slice())
        .execute(&admin)
        .await
        .unwrap();
    sqlx::raw_sql("UPDATE canon.receipt SET locator='/missing' WHERE locator_kind='json_pointer' AND raw_object_id IN(SELECT id FROM raw.object WHERE path='missing'); UPDATE canon.receipt SET locator_kind='future_pointer' WHERE raw_object_id IN(SELECT id FROM raw.object WHERE path='unsupported'); UPDATE raw.object SET bytes=decode('ff','hex') WHERE path='unsupported'").execute(&admin).await.unwrap();
    sqlx::query("UPDATE raw.object SET content_sha256=$1 WHERE path='contenthash'")
        .bind(vec![0u8; 32])
        .execute(&admin)
        .await
        .unwrap();
    for (path, bytes) in [
        ("invalid_utf8", b"\xff".as_slice()),
        ("parser", b"{".as_slice()),
    ] {
        sqlx::query("UPDATE raw.object SET bytes=$1,content_sha256=$2 WHERE path=$3")
            .bind(bytes)
            .bind(Sha256::digest(bytes).as_slice())
            .bind(path)
            .execute(&admin)
            .await
            .unwrap();
    }
    let poison = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            poison.sampled,
            poison.verified_receipts,
            poison.rejected_receipts,
            poison.rejected_objects
        ),
        (19, 6, 13, 5)
    );
    for code in [
        "raw_size_limit",
        "locator_collision",
        "content_hash_mismatch",
        "invalid_utf8",
        "parser_rejection",
    ] {
        assert_eq!(count(&admin, &format!("SELECT count(*) FROM ops.receipt_audit_rejection WHERE run_id={} AND code='{code}'", poison.run_id)).await, 1);
    }
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt_quarantine").await,
        12
    );
    assert_eq!(count(&admin,"SELECT count(*) FROM ops.receipt_audit a JOIN canon.receipt r ON r.id=a.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE a.outcome='match' AND o.path='good'").await,2);
    assert_eq!(count(&admin,"SELECT count(*) FROM canon.receipt_quarantine q JOIN canon.receipt r ON r.id=q.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path IN('oversize','collision','contenthash','invalid_utf8','parser')").await,10);
    assert_eq!(count(&admin,"SELECT count(*) FROM canon.receipt_quarantine q JOIN canon.receipt r ON r.id=q.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path='unsupported'").await,0);
    let rejected_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT receipt_id FROM ops.receipt_audit WHERE run_id=$1 AND outcome='object_rejected'",
    )
    .bind(poison.run_id)
    .fetch_all(&admin)
    .await
    .unwrap();
    assert_eq!(rejected_ids.len(), 10);
    for role in ["brain_tool", "brain_service"] {
        let mut tx = admin.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL ROLE {role}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let visible: i64 =
            sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE id=ANY($1)")
                .bind(&rejected_ids)
                .fetch_one(&mut *tx)
                .await
                .unwrap();
        assert_eq!(visible, 0);
        tx.rollback().await.unwrap();
    }
    let again = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            again.sampled,
            again.verified_receipts,
            again.rejected_receipts,
            again.rejected_objects
        ),
        (8, 6, 2, 0)
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt").await,
        20
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.fact").await, 9);
    assert_eq!(count(&admin,"SELECT count(*) FROM ops.receipt_audit_run WHERE status<>'succeeded' OR finished IS NULL").await,0);
    assert!(
        sqlx::query("INSERT INTO canon.receipt_quarantine(receipt_id,run_id) VALUES($1,$2)")
            .bind(corrupt)
            .bind(first.run_id)
            .execute(&writer)
            .await
            .is_err()
    );
    seed(&store, &["transaction_poison"], ONE).await;
    sqlx::query("UPDATE raw.object SET content_sha256=$1 WHERE path='transaction_poison'")
        .bind(vec![0u8; 32])
        .execute(&admin)
        .await
        .unwrap();
    let before = count(&admin, "SELECT count(*) FROM ops.receipt_audit_run").await;
    sqlx::raw_sql(
        "ALTER TABLE ops.receipt_audit ADD CONSTRAINT fail_audit CHECK(outcome<>'object_rejected') NOT VALID",
    )
    .execute(&admin)
    .await
    .unwrap();
    assert!(receipt_audit::run(&writer).await.is_err());
    assert_eq!(
        count(&admin, "SELECT count(*) FROM ops.receipt_audit_run").await,
        before
    );
    sqlx::raw_sql("ALTER TABLE ops.receipt_audit DROP CONSTRAINT fail_audit")
        .execute(&admin)
        .await
        .unwrap();
    receipt_audit::run(&writer).await.unwrap();
    stale_value_stays_hidden(&admin, &writer, &store).await;
    rank_and_late_arrival_boundaries(&admin, &writer, &store).await;
    seed(&store, &["bulk"], ONE).await;
    let capped = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(capped.sampled, 200);
    let (a, b) = tokio::join!(receipt_audit::run(&writer), receipt_audit::run(&writer));
    assert_eq!(a.unwrap().sampled, 200);
    assert_eq!(b.unwrap().sampled, 200);
    writer.close().await;
    admin.close().await;
}
