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
        let key = format!("hero_{}", raw.path);
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
                value: FactValue::Number(1.0),
                receipt: r,
            }],
            ..FactSet::default()
        };
        if raw.path == "corrupt" {
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
async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}
async fn seed(store: &GameFactsStore, paths: &[&str]) {
    let report = ingest(
        store,
        "audit_fixture",
        paths.iter().map(|p| {
            Ok(InputObject {
                revision: "r1".into(),
                path: (*p).into(),
                bytes: br#"{"x":1}"#.to_vec(),
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
    let sql = brain_job::limits::render_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-p9-receipt-audit-v1.sql"
    ));
    let body = sql
        .trim()
        .strip_prefix("BEGIN;")
        .unwrap()
        .trim()
        .strip_suffix("COMMIT;")
        .unwrap();
    let mut tx = admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::raw_sql(body).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    schema.check_gamefacts_schema().await.unwrap();
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
    seed(&store, &["corrupt"]).await;
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
        &["oversize", "collision", "missing", "good", "unsupported"],
    )
    .await;
    sqlx::query(
        "UPDATE raw.object SET bytes=convert_to(repeat('x',$1),'UTF8') WHERE path='oversize'",
    )
    .bind(16 * 1024 * 1024 + 1)
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
    sqlx::raw_sql("UPDATE canon.receipt SET locator='/missing' WHERE locator_kind='json_pointer' AND raw_object_id IN(SELECT id FROM raw.object WHERE path='missing'); UPDATE canon.receipt SET locator_kind='kv_path' WHERE raw_object_id IN(SELECT id FROM raw.object WHERE path='unsupported'); UPDATE raw.object SET bytes=decode('ff','hex') WHERE path='unsupported'").execute(&admin).await.unwrap();
    let poison = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            poison.sampled,
            poison.verified_receipts,
            poison.rejected_receipts,
            poison.rejected_objects
        ),
        (13, 6, 7, 2)
    );
    assert_eq!(count(&admin,"SELECT count(*) FROM ops.receipt_audit_rejection WHERE code IN('raw_size_limit','locator_collision')").await,2);
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt_quarantine").await,
        2
    );
    assert_eq!(count(&admin,"SELECT count(*) FROM ops.receipt_audit a JOIN canon.receipt r ON r.id=a.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE a.outcome='match' AND o.path='good'").await,2);
    assert_eq!(count(&admin,"SELECT count(*) FROM canon.receipt_quarantine q JOIN canon.receipt r ON r.id=q.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path IN('oversize','collision','unsupported')").await,0);
    let again = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            again.sampled,
            again.verified_receipts,
            again.rejected_receipts,
            again.rejected_objects
        ),
        (12, 6, 6, 2)
    );
    assert_eq!(
        count(&admin, "SELECT count(*) FROM canon.receipt").await,
        14
    );
    assert_eq!(count(&admin, "SELECT count(*) FROM canon.fact").await, 6);
    assert_eq!(count(&admin,"SELECT count(*) FROM ops.receipt_audit_run WHERE status<>'succeeded' OR finished IS NULL").await,0);
    assert!(
        sqlx::query("INSERT INTO canon.receipt_quarantine(receipt_id,run_id) VALUES($1,$2)")
            .bind(corrupt)
            .bind(first.run_id)
            .execute(&writer)
            .await
            .is_err()
    );
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
    seed(&store, &["bulk"]).await;
    let capped = receipt_audit::run(&writer).await.unwrap();
    assert_eq!(capped.sampled, 200);
    let (a, b) = tokio::join!(receipt_audit::run(&writer), receipt_audit::run(&writer));
    assert_eq!(a.unwrap().sampled, 200);
    assert_eq!(b.unwrap().sampled, 200);
    writer.close().await;
    admin.close().await;
}
