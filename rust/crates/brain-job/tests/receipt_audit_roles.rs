use brain_job::{Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt};
use brain_storage::PgStore;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

async fn setup() -> (scratch_pg::ScratchPg, PgConnectOptions, PgPool) {
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
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    (pg, options, admin)
}

async fn publish(pool: &PgPool, key: &str, facts: usize) {
    let store = GameFactsStore::new(pool.clone());
    let run = store.begin_run("role_fixture").await.unwrap();
    let bytes = br#"{"x":1}"#;
    let raw = store
        .capture(run, "role_fixture", "r1", key, bytes.to_vec())
        .await
        .unwrap();
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(bytes).into(),
    };
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: receipt.clone(),
        }],
        facts: (0..facts)
            .map(|i| Fact {
                entity_key: key.into(),
                stat_key: format!("stat_{i}"),
                value: FactValue::Number(1.0),
                receipt: receipt.clone(),
            })
            .collect(),
        ..FactSet::default()
    };
    store.apply(run, &raw, "role_fixture", &set).await.unwrap();
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
}

async fn permission_denied(pool: &PgPool, sql: &str) {
    let error = sqlx::query(sql).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().and_then(|e| e.code()).as_deref(),
        Some("42501"),
        "{sql}: {error}"
    );
}

#[tokio::test]
async fn ingest_audits_through_owner_sample_without_serving_read_rights() {
    let (_pg, options, admin) = setup().await;
    let writer = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_as::<_, (String, String)>("SELECT session_user::text,current_user::text")
            .fetch_one(&writer)
            .await
            .unwrap(),
        ("brain_ingest".into(), "brain_ingest".into())
    );
    publish(&writer, "hero_role", 201).await;
    for view in [
        "published_receipt",
        "current_entity",
        "current_fact",
        "current_alias",
        "current_relation",
    ] {
        permission_denied(&writer, &format!("SELECT * FROM serve.{view} LIMIT 0")).await;
        let allowed: bool =
            sqlx::query_scalar("SELECT has_table_privilege('brain_ingest',$1,'SELECT')")
                .bind(format!("serve.{view}"))
                .fetch_one(&admin)
                .await
                .unwrap();
        assert!(!allowed);
    }
    permission_denied(
        &writer,
        "ALTER FUNCTION ops.sample_receipt_audit() SECURITY INVOKER",
    )
    .await;
    let protected: bool = sqlx::query_scalar("SELECT p.prosecdef AND r.rolname='brain_migrate' AND p.proconfig=ARRAY['search_path=pg_catalog, pg_temp'] AND NOT has_schema_privilege('brain_ingest','ops','CREATE') AND NOT EXISTS(SELECT 1 FROM aclexplode(p.proacl) a WHERE a.grantee=0) FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner WHERE p.oid='ops.sample_receipt_audit()'::regprocedure")
        .fetch_one(&admin).await.unwrap();
    assert!(protected);
    for role in ["brain_tool", "brain_service"] {
        let allowed: bool = sqlx::query_scalar(
            "SELECT has_function_privilege($1,'ops.sample_receipt_audit()','EXECUTE')",
        )
        .bind(role)
        .fetch_one(&admin)
        .await
        .unwrap();
        assert!(!allowed);
        let mut tx = admin.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL ROLE {role}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let error = sqlx::query("SELECT * FROM ops.sample_receipt_audit()")
            .execute(&mut *tx)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().and_then(|e| e.code()).as_deref(),
            Some("42501")
        );
        tx.rollback().await.unwrap();
        for object in ["ops.receipt_audit", "raw.object"] {
            let allowed: bool = sqlx::query_scalar("SELECT has_table_privilege($1,$2,'SELECT')")
                .bind(role)
                .bind(object)
                .fetch_one(&admin)
                .await
                .unwrap();
            assert!(!allowed);
        }
    }
    let all: Vec<i64> = sqlx::query_scalar("SELECT id FROM serve.published_receipt ORDER BY id")
        .fetch_all(&admin)
        .await
        .unwrap();
    assert_eq!(all.len(), 202);
    let initial: Vec<i64> = sqlx::query_scalar("SELECT id FROM ops.sample_receipt_audit()")
        .fetch_all(&writer)
        .await
        .unwrap();
    assert_eq!(initial, all[..200]);
    let report = brain_job::receipt_audit::run(&writer).await.unwrap();
    assert_eq!(
        (
            report.sampled,
            report.verified_receipts,
            report.rejected_receipts,
            report.rejected_objects
        ),
        (200, 200, 0, 0)
    );
    let next: Vec<i64> = sqlx::query_scalar("SELECT id FROM ops.sample_receipt_audit()")
        .fetch_all(&writer)
        .await
        .unwrap();
    let expected: Vec<_> = all[200..].iter().chain(&all[..198]).copied().collect();
    assert_eq!(next, expected);
    writer.close().await;
    admin.close().await;
}
