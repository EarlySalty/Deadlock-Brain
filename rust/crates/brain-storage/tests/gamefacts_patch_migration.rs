use brain_storage::{gamefacts::limits, PgStore};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgConnectOptions, postgres::PgPoolOptions, postgres::PgSslMode};

#[path = "support/scratch_pg.rs"]
mod scratch_pg;

#[tokio::test]
async fn patch_migration_preserves_existing_facts_and_checks_patch_identity() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; CREATE ROLE brain_tool NOLOGIN; GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&pool).await.unwrap();
    let v1 = limits::render_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .replace(
        "{{HERO_PROFILE}}",
        include_str!("../src/gamefacts/hero_profile.sql"),
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *tx)
        .await
        .unwrap();
    let body = v1
        .trim()
        .strip_prefix("BEGIN;")
        .unwrap()
        .trim()
        .strip_suffix("COMMIT;")
        .unwrap();
    sqlx::raw_sql(body).execute(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version,checksum) VALUES(1,$1)")
        .bind(Sha256::digest(v1.as_bytes()).to_vec())
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    sqlx::raw_sql("INSERT INTO ops.ingest_run(source,status) VALUES('fixture','succeeded');
        INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES('fixture','opaque-commit-date','hero',decode(repeat('00',32),'hex'),decode(repeat('00',32),'hex'),'{}',1);
        INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES(1,1,'fixture','raw_bytes','',decode(repeat('00',32),'hex'));
        INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','hero_baba',1),('patch','patch_10',1),('patch','patch_20',1),('patch','unproven_patch',1);
        INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES(1,'health',500,1,1)")
        .execute(&pool).await.unwrap();
    let before: Value = sqlx::query_scalar("SELECT to_jsonb(f) FROM canon.fact f WHERE id=1")
        .fetch_one(&pool)
        .await
        .unwrap();
    let storage_before: i64 = sqlx::query_scalar(
        "SELECT relfilenode::bigint FROM pg_class WHERE oid='canon.fact'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let store = PgStore::new(pool.clone());
    store.migrate_gamefacts().await.unwrap();
    store.check_gamefacts_schema().await.unwrap();
    store.migrate_gamefacts().await.unwrap();
    let after: Value = sqlx::query_scalar(
        "SELECT to_jsonb(f)-'valid_from_patch'-'valid_to_patch' FROM canon.fact f WHERE id=1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(before, after);
    let storage_after: i64 = sqlx::query_scalar(
        "SELECT relfilenode::bigint FROM pg_class WHERE oid='canon.fact'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(storage_before, storage_after);
    let null_boundaries: bool = sqlx::query_scalar(
        "SELECT valid_from_patch IS NULL AND valid_to_patch IS NULL FROM canon.fact WHERE id=1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(null_boundaries);
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        versions,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 23, 24, 26, 28, 30, 31, 32, 33, 34, 35, 36]
    );
    sqlx::raw_sql("INSERT INTO ops.ingest_run(source) VALUES('pending_boundary_fixture');
        INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES(1,2,'fixture','raw_bytes','',decode(repeat('00',32),'hex'));
        INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES(1,'health',500,2,2)")
        .execute(&pool).await.unwrap();
    let reference_index: String =
        sqlx::query_scalar("SELECT pg_get_indexdef('canon.fact_patch_reference'::regclass)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(reference_index.contains("stat_key COLLATE \"C\", entity_id"));
    for column in ["valid_from_patch", "valid_to_patch"] {
        for (id, code) in [(1, "23514"), (4, "23514"), (9999, "23503")] {
            let error = sqlx::query(&format!("UPDATE canon.fact SET {column}=$1 WHERE id=2"))
                .bind(id)
                .execute(&pool)
                .await
                .unwrap_err();
            assert_eq!(
                error.as_database_error().unwrap().code().as_deref(),
                Some(code)
            );
        }
    }
    sqlx::query("UPDATE canon.fact SET valid_from_patch=2,valid_to_patch=3 WHERE id=2")
        .execute(&pool)
        .await
        .unwrap();
    for sql in [
        "UPDATE canon.entity SET kind='hero' WHERE id=2",
        "UPDATE canon.entity SET key='invalid_patch' WHERE id=3",
        "UPDATE canon.entity SET key='patch_99' WHERE id=2",
        "DELETE FROM canon.entity WHERE id=3",
    ] {
        assert!(sqlx::query(sql).execute(&pool).await.is_err());
    }
    let grants_unchanged: bool = sqlx::query_scalar(
        "SELECT
        NOT has_column_privilege('brain_ingest','canon.fact','valid_from_patch','INSERT,UPDATE')
        AND NOT has_column_privilege('brain_ingest','canon.fact','valid_to_patch','INSERT,UPDATE')
        AND NOT has_table_privilege('brain_tool','canon.fact','SELECT,INSERT,UPDATE,DELETE')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(grants_unchanged);
    sqlx::query("UPDATE canon.fact SET valid_from_patch=NULL,valid_to_patch=NULL WHERE id=2")
        .execute(&pool)
        .await
        .unwrap();
    let columns: Vec<String> = sqlx::query_scalar("SELECT column_name::text FROM information_schema.columns WHERE table_schema='canon' AND table_name='fact' AND column_name IN ('valid_from_patch','valid_to_patch') AND is_nullable='YES' ORDER BY column_name")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(columns, ["valid_from_patch", "valid_to_patch"]);
    println!("P4_MIGRATION_PROOF sparse [1,2,3,4,6,7,8,9,10,11,13,23,28,31], upgrade and rerun, unchanged published fact/relfilenode, null unproven history, pending fact patch identity guards and foreign keys, unchanged grants");
    pool.close().await;
}
