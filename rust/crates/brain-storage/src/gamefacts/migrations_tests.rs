use super::*;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

#[path = "../../tests/support/scratch_pg.rs"]
mod scratch_pg;

#[test]
fn registry_is_positive_unique_and_ordered_with_gaps() {
    assert!(ordered(&[(1, ""), (6, "")]).is_ok());
    assert!(ordered(&[(1, ""), (2, ""), (6, "")]).is_ok());
    for registry in [
        vec![(0, "")],
        vec![(-1, "")],
        vec![(1, ""), (1, "")],
        vec![(6, ""), (2, "")],
    ] {
        assert!(ordered(&registry).is_err());
    }
    assert_eq!(
        sql_and_checksum(MIGRATIONS[0].1)
            .1
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>(),
        "536bae3589643fb9d91bf5faea96a5d6c4e9cbe748737a2ea43e03723a10cbcf"
    );
}

#[tokio::test]
async fn sparse_versions_apply_a_missing_lower_version_without_replaying_higher_versions() {
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
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate").execute(&pool).await.unwrap();
    let store = PgStore::new(pool.clone());
    let six = "BEGIN; CREATE TABLE ops.core2_six(marker integer PRIMARY KEY); INSERT INTO ops.core2_six VALUES(6); COMMIT;";
    let two = "BEGIN; CREATE TABLE ops.core2_two(marker integer PRIMARY KEY); INSERT INTO ops.core2_two VALUES(2); COMMIT;";
    let initial = [(1, MIGRATIONS[0].1), (6, six)];
    store.migrate_gamefacts_registered(&initial).await.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let before = recorded(&mut connection).await.unwrap();
    drop(connection);
    assert_eq!(before.iter().map(|(v, _)| *v).collect::<Vec<_>>(), [1, 6]);
    let expanded = [(1, MIGRATIONS[0].1), (2, two), (6, six)];
    store.migrate_gamefacts_registered(&expanded).await.unwrap();
    store.migrate_gamefacts_registered(&expanded).await.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let after = recorded(&mut connection).await.unwrap();
    drop(connection);
    assert_eq!(after.iter().map(|(v, _)| *v).collect::<Vec<_>>(), [1, 2, 6]);
    assert_eq!(before[0], after[0]);
    assert_eq!(before[1], after[2]);
    for table in ["core2_two", "core2_six"] {
        let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM ops.{table}"))
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }
    let changed = six.replace("VALUES(6)", "VALUES(66)");
    assert!(store
        .migrate_gamefacts_registered(&[(1, MIGRATIONS[0].1), (2, two), (6, &changed)])
        .await
        .is_err());
    sqlx::query("INSERT INTO ops.gamefacts_schema_version VALUES(7,$1)")
        .bind(vec![0_u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    assert!(store.migrate_gamefacts_registered(&expanded).await.is_err());
    sqlx::query("DELETE FROM ops.gamefacts_schema_version WHERE version=7")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE ops.gamefacts_schema_version SET checksum=$1 WHERE version=1")
        .bind(vec![0_u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    assert!(store.migrate_gamefacts_registered(&expanded).await.is_err());
    pool.close().await;
}

async fn integrity_receipt(pool: &sqlx::PgPool, raw: i64, run: i64) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'integrity','raw_bytes','',$3) RETURNING id")
        .bind(raw).bind(run).bind(vec![0_u8; 32]).fetch_one(pool).await.unwrap()
}

async fn insert_integrity_fact(
    pool: &sqlx::PgPool,
    entity: i64,
    key: &str,
    run: i64,
    receipt: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,$2,120,$3,$4)")
        .bind(entity).bind(key).bind(run).bind(receipt).execute(pool).await.map(|_| ())
}

async fn replace_integrity_fact(
    pool: &sqlx::PgPool,
    entity: i64,
    key: &str,
    run: i64,
    receipt: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT canon.write_run_fact($1,$2,125,NULL,NULL,$3,$4)")
        .bind(entity)
        .bind(key)
        .bind(run)
        .bind(receipt)
        .execute(pool)
        .await
        .map(|_| ())
}

fn assert_integrity_denied(result: Result<(), sqlx::Error>) {
    assert_eq!(
        result
            .unwrap_err()
            .as_database_error()
            .unwrap()
            .code()
            .as_deref(),
        Some("23514")
    );
}

async fn integrity_snapshot(
    pool: &sqlx::PgPool,
) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
    let facts = sqlx::query_scalar("SELECT to_jsonb(f) FROM canon.fact f ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap();
    let served = sqlx::query_scalar("SELECT to_jsonb(f) FROM serve.current_fact f ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap();
    (facts, served)
}

async fn assert_integrity_waiting(
    admin: &sqlx::PgPool,
    pid: i32,
    waiter: &mut tokio::task::JoinHandle<Result<(), sqlx::Error>>,
) {
    loop {
        tokio::select! {
            result = &mut *waiter => panic!("fact/publication operation bypassed row lock: {result:?}"),
            blocked = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE pid=$1 AND wait_event_type='Lock')"
            ).bind(pid).fetch_one(admin) => {
                if blocked.unwrap() { return; }
            }
        }
    }
}

#[tokio::test]
async fn fact_integrity_upgrade_blocks_real_ingest_withdrawal_of_legacy_published_facts() {
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
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&admin).await.unwrap();
    let migrator = PgStore::new(admin.clone());
    migrator
        .migrate_gamefacts_registered(&MIGRATIONS[..2])
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    let roles: (String, String, bool) = sqlx::query_as("SELECT session_user::text,current_user::text,(SELECT rolsuper FROM pg_roles WHERE rolname=current_user)")
        .fetch_one(&writer).await.unwrap();
    assert_eq!(roles, ("brain_ingest".into(), "brain_ingest".into(), false));
    let store = crate::gamefacts::GameFactsStore::new(writer.clone());
    let published = store.begin_run("integrity").await.unwrap();
    let failed = store.begin_run("integrity").await.unwrap();
    let other_failed = store.begin_run("integrity").await.unwrap();
    let raw = store
        .capture(published, "integrity", "r1", "hero.json", b"120".to_vec())
        .await
        .unwrap();
    let published_receipt = integrity_receipt(&writer, raw.id, published).await;
    let failed_receipt = integrity_receipt(&writer, raw.id, failed).await;
    let other_receipt = integrity_receipt(&writer, raw.id, other_failed).await;
    let entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','hero_integrity',$1) RETURNING id")
        .bind(published_receipt).fetch_one(&writer).await.unwrap();
    for key in ["vulnerable", "legacy_visible"] {
        insert_integrity_fact(&writer, entity, key, failed, published_receipt)
            .await
            .unwrap();
    }
    insert_integrity_fact(&writer, entity, "legacy_pending", failed, other_receipt)
        .await
        .unwrap();
    insert_integrity_fact(
        &writer,
        entity,
        "coherent_published",
        published,
        published_receipt,
    )
    .await
    .unwrap();
    sqlx::query("UPDATE ops.ingest_run SET status='succeeded' WHERE id=$1")
        .bind(published)
        .execute(&writer)
        .await
        .unwrap();
    let visible: f64 =
        sqlx::query_scalar("SELECT value_num FROM serve.current_fact WHERE stat_key='vulnerable'")
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(visible, 120.0);
    replace_integrity_fact(&writer, entity, "vulnerable", failed, failed_receipt)
        .await
        .unwrap();
    let withdrawn: bool = sqlx::query_scalar(
        "SELECT NOT EXISTS(SELECT 1 FROM serve.current_fact WHERE stat_key='vulnerable')",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert!(withdrawn);
    let before = integrity_snapshot(&admin).await;
    migrator.migrate_gamefacts().await.unwrap();
    migrator.migrate_gamefacts().await.unwrap();
    PgStore::new(writer.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    assert_eq!(integrity_snapshot(&admin).await, before);
    for (key, run, receipt) in [
        ("legacy_visible", failed, failed_receipt),
        ("legacy_pending", failed, failed_receipt),
        ("coherent_published", published, published_receipt),
    ] {
        assert_integrity_denied(replace_integrity_fact(&writer, entity, key, run, receipt).await);
        assert_eq!(integrity_snapshot(&admin).await, before);
    }
    for (key, run, receipt) in [
        ("mismatched_published", failed, published_receipt),
        ("mismatched_pending", failed, other_receipt),
        ("direct_published", published, published_receipt),
    ] {
        assert_integrity_denied(insert_integrity_fact(&writer, entity, key, run, receipt).await);
        assert_eq!(integrity_snapshot(&admin).await, before);
    }
    let no_update =
        sqlx::query("UPDATE canon.fact SET value_num=999 WHERE stat_key='legacy_visible'")
            .execute(&writer)
            .await
            .unwrap_err();
    assert_eq!(
        no_update.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
    let denied_read = sqlx::query("SELECT value_num FROM serve.current_fact")
        .fetch_all(&writer)
        .await
        .unwrap_err();
    assert_eq!(
        denied_read.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
    let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id=$1")
        .bind(published)
        .fetch_one(&writer)
        .await
        .unwrap();
    assert_eq!(status, "succeeded");
    insert_integrity_fact(&writer, entity, "coherent_pending", failed, failed_receipt)
        .await
        .unwrap();
    replace_integrity_fact(&writer, entity, "coherent_pending", failed, failed_receipt)
        .await
        .unwrap();
    let coherent: f64 =
        sqlx::query_scalar("SELECT value_num FROM canon.fact WHERE stat_key='coherent_pending'")
            .fetch_one(&writer)
            .await
            .unwrap();
    assert_eq!(coherent, 125.0);
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&writer)
            .await
            .unwrap();
    assert_eq!(versions, [1, 7, 8]);
    let mut replacement = writer.begin().await.unwrap();
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'concurrent_pending',130,$2,$3)")
        .bind(entity).bind(failed).bind(failed_receipt)
        .execute(&mut *replacement).await.unwrap();
    let mut publisher = writer.acquire().await.unwrap();
    let publisher_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *publisher)
        .await
        .unwrap();
    let mut publication = tokio::spawn(async move {
        sqlx::query("UPDATE ops.ingest_run SET status='succeeded' WHERE id=$1")
            .bind(failed)
            .execute(&mut *publisher)
            .await
            .map(|_| ())
    });
    assert_integrity_waiting(&admin, publisher_pid, &mut publication).await;
    replacement.commit().await.unwrap();
    publication.await.unwrap().unwrap();
    let visible: f64 = sqlx::query_scalar(
        "SELECT value_num FROM serve.current_fact WHERE stat_key='concurrent_pending'",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(visible, 130.0);
    let mut publishing = writer.begin().await.unwrap();
    sqlx::query("UPDATE ops.ingest_run SET status='succeeded' WHERE id=$1")
        .bind(other_failed)
        .execute(&mut *publishing)
        .await
        .unwrap();
    let mut inserter = writer.acquire().await.unwrap();
    let inserter_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *inserter)
        .await
        .unwrap();
    let mut insertion = tokio::spawn(async move {
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'concurrent_published',999,$2,$3)")
            .bind(entity).bind(other_failed).bind(other_receipt)
            .execute(&mut *inserter).await.map(|_| ())
    });
    assert_integrity_waiting(&admin, inserter_pid, &mut insertion).await;
    publishing.commit().await.unwrap();
    let after_publication = integrity_snapshot(&admin).await;
    assert_integrity_denied(insertion.await.unwrap());
    assert_eq!(integrity_snapshot(&admin).await, after_publication);
    writer.close().await;
    admin.close().await;
}

async fn assert_schema_checks(stores: &[(&str, PgStore)], valid: bool) {
    for (role, store) in stores {
        let result = store.check_gamefacts_schema().await;
        if valid {
            assert!(result.is_ok(), "{role}: {result:?}");
        } else {
            assert!(
                matches!(result, Err(PortError::InvalidResponse(_))),
                "{role}: {result:?}"
            );
            if *role == "brain_migrate" {
                assert!(matches!(
                    store.migrate_gamefacts().await,
                    Err(PortError::InvalidResponse(_))
                ));
            }
        }
    }
}

#[tokio::test]
async fn ingest_preflight_checks_serving_shape_without_serving_read_privileges() {
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
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate").execute(&pool).await.unwrap();
    PgStore::new(pool.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let mut stores = Vec::new();
    for role in ["brain_ingest", "brain_migrate"] {
        let role_pool = PgPoolOptions::new()
            .max_connections(1)
            .after_connect(move |connection, _| {
                Box::pin(async move {
                    sqlx::query(&format!("SET ROLE {role}"))
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect_with(options.clone())
            .await
            .unwrap();
        let current_user: String = sqlx::query_scalar("SELECT current_user")
            .fetch_one(&role_pool)
            .await
            .unwrap();
        assert_eq!(current_user, role);
        stores.push((role, PgStore::new(role_pool)));
    }
    assert_schema_checks(&stores, true).await;
    for (break_schema, restore_schema) in [
        ("GRANT UPDATE(value_text) ON canon.fact TO brain_ingest", "REVOKE UPDATE(value_text) ON canon.fact FROM brain_ingest"),
        ("ALTER TABLE ops.ingest_run DISABLE TRIGGER keep_published_run", "ALTER TABLE ops.ingest_run ENABLE TRIGGER keep_published_run"),
        ("GRANT EXECUTE ON FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) TO brain_service", "REVOKE EXECUTE ON FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) FROM brain_service"),
        ("ALTER FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) SET search_path=public", "ALTER FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) SET search_path=pg_catalog"),
        ("ALTER TABLE canon.fact DISABLE TRIGGER guard_fact_integrity", "ALTER TABLE canon.fact ENABLE TRIGGER guard_fact_integrity"),
        ("ALTER FUNCTION canon.guard_fact_integrity() OWNER TO brain_core_test", "ALTER FUNCTION canon.guard_fact_integrity() OWNER TO brain_migrate"),
        ("ALTER FUNCTION canon.guard_fact_integrity() SET search_path=public", "ALTER FUNCTION canon.guard_fact_integrity() SET search_path=pg_catalog"),
        ("GRANT EXECUTE ON FUNCTION canon.guard_fact_integrity() TO brain_ingest", "REVOKE EXECUTE ON FUNCTION canon.guard_fact_integrity() FROM brain_ingest"),
        ("DROP TRIGGER guard_fact_integrity ON canon.fact; CREATE TRIGGER guard_fact_integrity BEFORE UPDATE OF receipt_id ON canon.fact FOR EACH ROW EXECUTE FUNCTION canon.guard_fact_integrity()", "DROP TRIGGER guard_fact_integrity ON canon.fact; CREATE TRIGGER guard_fact_integrity BEFORE INSERT OR UPDATE ON canon.fact FOR EACH ROW EXECUTE FUNCTION canon.guard_fact_integrity()"),
    ] {
        sqlx::raw_sql(break_schema).execute(&pool).await.unwrap();
        assert_schema_checks(&stores, false).await;
        sqlx::raw_sql(restore_schema).execute(&pool).await.unwrap();
        assert_schema_checks(&stores, true).await;
    }
    stores[1].1.migrate_gamefacts().await.unwrap();
    let serving_columns = [
        ("published_receipt", "revision"),
        ("current_entity", "key"),
        ("current_alias", "alias_norm"),
        ("current_fact", "stat_key"),
        ("current_relation", "kind"),
    ];
    for (relation, column) in serving_columns {
        let error = sqlx::query(&format!("SELECT {column} FROM serve.{relation} LIMIT 0"))
            .fetch_all(&stores[0].1.pool)
            .await
            .unwrap_err();
        assert_eq!(
            error
                .as_database_error()
                .and_then(|error| error.code())
                .as_deref(),
            Some("42501")
        );
        sqlx::raw_sql(&format!(
            "ALTER VIEW serve.{relation} RENAME COLUMN {column} TO incompatible_column"
        ))
        .execute(&pool)
        .await
        .unwrap();
        assert_schema_checks(&stores, false).await;
        sqlx::raw_sql(&format!(
            "ALTER VIEW serve.{relation} RENAME COLUMN incompatible_column TO {column}; ALTER VIEW serve.{relation} RENAME TO incompatible_relation"
        ))
        .execute(&pool)
        .await
        .unwrap();
        assert_schema_checks(&stores, false).await;
        sqlx::raw_sql(&format!(
            "ALTER VIEW serve.incompatible_relation RENAME TO {relation}"
        ))
        .execute(&pool)
        .await
        .unwrap();
        assert_schema_checks(&stores, true).await;
    }
    sqlx::raw_sql("ALTER VIEW serve.current_relation RENAME TO original_relation; CREATE VIEW serve.current_relation AS SELECT ''::text AS kind")
        .execute(&pool).await.unwrap();
    assert_schema_checks(&stores, false).await;
    sqlx::raw_sql(
        "DROP VIEW serve.current_relation; CREATE TABLE serve.current_relation(kind canon.key)",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_schema_checks(&stores, false).await;
    sqlx::raw_sql("DROP TABLE serve.current_relation; ALTER VIEW serve.original_relation RENAME TO current_relation")
        .execute(&pool).await.unwrap();
    assert_schema_checks(&stores, true).await;
    sqlx::query("UPDATE ops.gamefacts_schema_version SET checksum=$1 WHERE version=1")
        .bind(vec![0_u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    assert_schema_checks(&stores, false).await;
    sqlx::query("UPDATE ops.gamefacts_schema_version SET checksum=$1 WHERE version=1")
        .bind(sql_and_checksum(MIGRATIONS[0].1).1)
        .execute(&pool)
        .await
        .unwrap();
    assert_schema_checks(&stores, true).await;
    for (_, store) in stores {
        store.pool.close().await;
    }
    pool.close().await;
}
