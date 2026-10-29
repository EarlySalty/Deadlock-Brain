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
    assert!(ordered(MIGRATIONS).is_ok());
    assert_eq!(
        MIGRATIONS.iter().map(|(v, _)| *v).collect::<Vec<_>>(),
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 23, 24, 26, 28, 30, 31, 32, 33, 34, 35, 36, 37,
            38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54
        ]
    );
    assert_eq!(
        sql_and_checksum(MIGRATIONS.iter().find(|(v, _)| *v == 4).unwrap().1)
            .1
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>(),
        "bc849a1a5e130487e8da4052168ca07a0b81bf4fb06f6f26a6a180b641d7ebee"
    );
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

#[tokio::test]
async fn receipt_audit_upgrade_preserves_applied_core_and_serving_columns() {
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
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&pool).await.unwrap();
    let store = PgStore::new(pool.clone());
    let initial: Vec<_> = MIGRATIONS
        .iter()
        .copied()
        .filter(|(v, _)| !matches!(*v, 6 | 28))
        .collect();
    store.migrate_gamefacts_registered(&initial).await.unwrap();
    let columns_sql = "SELECT attname::text,atttypid::bigint FROM pg_attribute WHERE attrelid='serve.published_receipt'::regclass AND attnum>0 AND NOT attisdropped ORDER BY attnum";
    let columns: Vec<(String, i64)> = sqlx::query_as(columns_sql).fetch_all(&pool).await.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let before = recorded(&mut connection).await.unwrap();
    drop(connection);
    store.migrate_gamefacts().await.unwrap();
    store.migrate_gamefacts().await.unwrap();
    store.check_gamefacts_schema().await.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let after = recorded(&mut connection).await.unwrap();
    drop(connection);
    assert_eq!(
        after
            .iter()
            .filter(|(v, _)| !matches!(*v, 6 | 28))
            .cloned()
            .collect::<Vec<_>>(),
        before
    );
    assert!(after.iter().any(|(v, _)| *v == 6));
    assert!(after.iter().any(|(v, _)| *v == 28));
    assert_eq!(
        sqlx::query_as::<_, (String, i64)>(columns_sql)
            .fetch_all(&pool)
            .await
            .unwrap(),
        columns
    );
    let sampled: i64 = sqlx::query_scalar("SELECT count(*) FROM ops.sample_receipt_audit()")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(sampled, 0);
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
    let legacy: Vec<_> = MIGRATIONS.iter().copied().filter(|(v, _)| *v < 8).collect();
    migrator
        .migrate_gamefacts_registered(&legacy)
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
    assert_eq!(
        versions,
        MIGRATIONS.iter().map(|(v, _)| *v).collect::<Vec<_>>()
    );
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
    sqlx::raw_sql("ALTER ROLE brain_ingest LOGIN; ALTER ROLE brain_migrate LOGIN")
        .execute(&pool)
        .await
        .unwrap();
    let mut stores = Vec::new();
    for role in ["brain_ingest", "brain_migrate"] {
        let role_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(options.clone().username(role))
            .await
            .unwrap();
        let users: (String, String) =
            sqlx::query_as("SELECT session_user::text,current_user::text")
                .fetch_one(&role_pool)
                .await
                .unwrap();
        assert_eq!(users, (role.into(), role.into()));
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
        ("ALTER TABLE canon.receipt ALTER COLUMN application_xid DROP DEFAULT", "ALTER TABLE canon.receipt ALTER COLUMN application_xid SET DEFAULT pg_current_xact_id()"),
        ("ALTER TABLE canon.receipt ALTER COLUMN application_xid SET DEFAULT '1'::xid8", "ALTER TABLE canon.receipt ALTER COLUMN application_xid SET DEFAULT pg_current_xact_id()"),
        ("ALTER TABLE canon.receipt ALTER COLUMN application_xid DROP NOT NULL", "ALTER TABLE canon.receipt ALTER COLUMN application_xid SET NOT NULL"),
        ("ALTER TABLE canon.receipt RENAME COLUMN application_xid TO bad_application", "ALTER TABLE canon.receipt RENAME COLUMN bad_application TO application_xid"),
        ("ALTER TABLE canon.receipt ALTER COLUMN application_xid DROP DEFAULT; ALTER TABLE canon.receipt ALTER COLUMN application_xid TYPE text USING application_xid::text; ALTER TABLE canon.receipt ALTER COLUMN application_xid SET DEFAULT pg_current_xact_id()::text", "ALTER TABLE canon.receipt ALTER COLUMN application_xid DROP DEFAULT; ALTER TABLE canon.receipt ALTER COLUMN application_xid TYPE xid8 USING application_xid::xid8; ALTER TABLE canon.receipt ALTER COLUMN application_xid SET DEFAULT pg_current_xact_id()"),
        ("GRANT INSERT(application_xid) ON canon.receipt TO brain_ingest", "REVOKE INSERT(application_xid) ON canon.receipt FROM brain_ingest"),
        ("GRANT UPDATE(application_xid) ON canon.receipt TO brain_ingest", "REVOKE UPDATE(application_xid) ON canon.receipt FROM brain_ingest"),
        ("ALTER TABLE canon.receipt DISABLE TRIGGER object_rows", "ALTER TABLE canon.receipt ENABLE TRIGGER object_rows"),
        ("ALTER FUNCTION canon.check_object_rows() SET search_path=public", "ALTER FUNCTION canon.check_object_rows() SET search_path=pg_catalog"),
        ("ALTER FUNCTION canon.check_object_rows() OWNER TO brain_core_test", "ALTER FUNCTION canon.check_object_rows() OWNER TO brain_migrate"),
        ("GRANT EXECUTE ON FUNCTION canon.check_object_rows() TO brain_ingest", "REVOKE EXECUTE ON FUNCTION canon.check_object_rows() FROM brain_ingest"),
        ("ALTER INDEX canon.receipt_object_application RENAME TO missing_application_index", "ALTER INDEX canon.missing_application_index RENAME TO receipt_object_application"),
        ("DROP INDEX canon.receipt_object_application; CREATE INDEX receipt_object_application ON canon.receipt(raw_object_id,ingest_run_id,extractor,application_xid) WHERE extractor='stub'", "DROP INDEX canon.receipt_object_application; CREATE INDEX receipt_object_application ON canon.receipt(raw_object_id,ingest_run_id,extractor,application_xid)"),
    ] {
        sqlx::raw_sql(break_schema).execute(&pool).await.unwrap();
        assert_schema_checks(&stores, false).await;
        sqlx::raw_sql(restore_schema).execute(&pool).await.unwrap();
        assert_schema_checks(&stores, true).await;
    }
    let published_sql = sql_and_checksum(MIGRATIONS.iter().find(|(v, _)| *v == 10).unwrap().1).0;
    for (relation, function, column) in [
        ("receipt", "guard_receipt_insert", "ingest_run_id"),
        ("entity", "guard_reference_insert", "receipt_id"),
        ("entity_identity", "guard_reference_insert", "receipt_id"),
        ("alias", "guard_reference_insert", "receipt_id"),
        ("relation", "guard_reference_insert", "receipt_id"),
    ] {
        let restore = format!("CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.{relation} FOR EACH ROW EXECUTE FUNCTION canon.{function}()");
        for replacement in [
            String::new(),
            format!("CREATE TRIGGER guard_published_insert AFTER INSERT ON canon.{relation} FOR EACH ROW EXECUTE FUNCTION canon.{function}()"),
            format!("CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.{relation} FOR EACH STATEMENT EXECUTE FUNCTION canon.{function}()"),
            format!("CREATE TRIGGER guard_published_insert BEFORE UPDATE OF {column} ON canon.{relation} FOR EACH ROW EXECUTE FUNCTION canon.{function}()"),
            format!("CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.{relation} FOR EACH ROW WHEN (false) EXECUTE FUNCTION canon.{function}()"),
            format!("CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.{relation} FOR EACH ROW EXECUTE FUNCTION canon.guard_fact_integrity()"),
            format!("CREATE CONSTRAINT TRIGGER guard_published_insert AFTER INSERT ON canon.{relation} FOR EACH ROW EXECUTE FUNCTION canon.{function}()"),
        ] {
            sqlx::raw_sql(&format!("DROP TRIGGER guard_published_insert ON canon.{relation}; {replacement}"))
                .execute(&pool).await.unwrap();
            assert_schema_checks(&stores, false).await;
            sqlx::raw_sql(&format!("DROP TRIGGER IF EXISTS guard_published_insert ON canon.{relation}; {restore}"))
                .execute(&pool).await.unwrap();
            assert_schema_checks(&stores, true).await;
        }
        for state in ["DISABLE", "ENABLE REPLICA", "ENABLE ALWAYS"] {
            sqlx::raw_sql(&format!(
                "ALTER TABLE canon.{relation} {state} TRIGGER guard_published_insert"
            ))
            .execute(&pool)
            .await
            .unwrap();
            assert_schema_checks(&stores, false).await;
            sqlx::raw_sql(&format!(
                "ALTER TABLE canon.{relation} ENABLE TRIGGER guard_published_insert"
            ))
            .execute(&pool)
            .await
            .unwrap();
            assert_schema_checks(&stores, true).await;
        }
    }
    for (function, delimiter) in [
        ("guard_receipt_insert", "$receipt$"),
        ("guard_reference_insert", "$reference$"),
    ] {
        for (change, restore) in [
            ("OWNER TO brain_core_test", "OWNER TO brain_migrate"),
            ("SECURITY INVOKER", "SECURITY DEFINER"),
            ("SET search_path=public", "SET search_path=pg_catalog"),
            ("RENAME TO missing_published_guard", "RENAME TO unused"),
        ] {
            sqlx::raw_sql(&format!("ALTER FUNCTION canon.{function}() {change}"))
                .execute(&pool)
                .await
                .unwrap();
            assert_schema_checks(&stores, false).await;
            let restoration = if change.starts_with("RENAME") {
                format!("ALTER FUNCTION canon.missing_published_guard() RENAME TO {function}")
            } else {
                format!("ALTER FUNCTION canon.{function}() {restore}")
            };
            sqlx::raw_sql(&restoration).execute(&pool).await.unwrap();
            assert_schema_checks(&stores, true).await;
        }
        for role in [
            "PUBLIC",
            "brain_ingest",
            "brain_tool",
            "brain_service",
            "brain_readonly",
        ] {
            sqlx::raw_sql(&format!(
                "GRANT EXECUTE ON FUNCTION canon.{function}() TO {role}"
            ))
            .execute(&pool)
            .await
            .unwrap();
            assert_schema_checks(&stores, false).await;
            sqlx::raw_sql(&format!(
                "REVOKE EXECUTE ON FUNCTION canon.{function}() FROM {role}"
            ))
            .execute(&pool)
            .await
            .unwrap();
            assert_schema_checks(&stores, true).await;
        }
        let body = published_sql.split(delimiter).nth(1).unwrap();
        sqlx::raw_sql(&format!("CREATE OR REPLACE FUNCTION canon.{function}() RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $drift$ BEGIN RETURN NEW; END; $drift$"))
            .execute(&pool).await.unwrap();
        assert_schema_checks(&stores, false).await;
        sqlx::raw_sql(&format!("CREATE OR REPLACE FUNCTION canon.{function}() RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS {delimiter}{body}{delimiter}"))
            .execute(&pool).await.unwrap();
        assert_schema_checks(&stores, true).await;
    }
    let guard_sql = sql_and_checksum(MIGRATIONS.iter().find(|(v, _)| *v == 9).unwrap().1).0;
    let guard_body = guard_sql.split("$rows$").nth(1).unwrap();
    let restore_guard = format!("CREATE OR REPLACE FUNCTION canon.check_object_rows() RETURNS trigger LANGUAGE plpgsql SET search_path=pg_catalog AS $rows${guard_body}$rows$");
    sqlx::raw_sql("CREATE OR REPLACE FUNCTION canon.check_object_rows() RETURNS trigger LANGUAGE plpgsql SET search_path=pg_catalog AS $rows$ BEGIN RETURN NEW; END; $rows$")
        .execute(&pool).await.unwrap();
    assert_schema_checks(&stores, false).await;
    sqlx::raw_sql(&restore_guard).execute(&pool).await.unwrap();
    assert_schema_checks(&stores, true).await;
    sqlx::raw_sql("DROP TRIGGER object_rows ON canon.receipt; CREATE TRIGGER object_rows AFTER INSERT ON canon.receipt FOR EACH ROW EXECUTE FUNCTION canon.check_object_rows()")
        .execute(&pool).await.unwrap();
    assert_schema_checks(&stores, false).await;
    sqlx::raw_sql("DROP TRIGGER object_rows ON canon.receipt; CREATE CONSTRAINT TRIGGER object_rows AFTER INSERT ON canon.receipt DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION canon.check_object_rows()")
        .execute(&pool).await.unwrap();
    assert_schema_checks(&stores, true).await;
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

async fn preflight_fixture() -> (scratch_pg::ScratchPg, sqlx::PgPool, PgStore) {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    let identity: (String, String, bool, bool) = sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user")
        .fetch_one(&writer).await.unwrap();
    assert_eq!(
        identity,
        ("brain_ingest".into(), "brain_ingest".into(), false, false)
    );
    let error = sqlx::query("SELECT * FROM serve.current_fact LIMIT 0")
        .fetch_all(&writer)
        .await
        .unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
    let writer = PgStore::new(writer);
    writer.check_gamefacts_schema().await.unwrap();
    (pg, admin, writer)
}

async fn assert_preflight_fault(
    admin: &sqlx::PgPool,
    writer: &PgStore,
    fault: &str,
    restoration: &str,
) {
    let before = published_upgrade_snapshot(admin).await;
    sqlx::raw_sql(fault).execute(admin).await.unwrap();
    assert!(
        matches!(
            writer.check_gamefacts_schema().await,
            Err(PortError::InvalidResponse(_))
        ),
        "{fault}"
    );
    assert!(
        matches!(
            PgStore::new(admin.clone()).migrate_gamefacts().await,
            Err(PortError::InvalidResponse(_))
        ),
        "{fault}"
    );
    assert_eq!(published_upgrade_snapshot(admin).await, before);
    sqlx::raw_sql(restoration).execute(admin).await.unwrap();
    writer.check_gamefacts_schema().await.unwrap();
    assert_eq!(published_upgrade_snapshot(admin).await, before);
    println!("preflight rejected before writes, restored: {fault}");
}

async fn assert_signed_guard_faults(
    admin: &sqlx::PgPool,
    writer: &PgStore,
    version: i32,
    guard: (&str, &str, &str),
) {
    let (function, delimiter, security) = guard;
    let template = MIGRATIONS.iter().find(|(v, _)| *v == version).unwrap().1;
    let sql = sql_and_checksum(template).0;
    let body = sql.split(delimiter).nth(1).unwrap();
    let restore = format!("CREATE OR REPLACE FUNCTION {function}() RETURNS trigger LANGUAGE plpgsql {security} SET search_path=pg_catalog AS {delimiter}{body}{delimiter}");
    let faulty = format!("CREATE OR REPLACE FUNCTION {function}() RETURNS trigger LANGUAGE plpgsql {security} SET search_path=pg_catalog AS $drift$ BEGIN RETURN NEW; END; $drift$");
    assert_preflight_fault(admin, writer, &faulty, &restore).await;
    for (change, undo) in [
        ("OWNER TO brain_core_test", "OWNER TO brain_migrate"),
        (
            if security == "SECURITY DEFINER" {
                "SECURITY INVOKER"
            } else {
                "SECURITY DEFINER"
            },
            security,
        ),
        ("SET search_path=public", "SET search_path=pg_catalog"),
    ] {
        assert_preflight_fault(
            admin,
            writer,
            &format!("ALTER FUNCTION {function}() {change}"),
            &format!("ALTER FUNCTION {function}() {undo}"),
        )
        .await;
    }
    for role in [
        "PUBLIC",
        "brain_ingest",
        "brain_tool",
        "brain_service",
        "brain_readonly",
    ] {
        assert_preflight_fault(
            admin,
            writer,
            &format!("GRANT EXECUTE ON FUNCTION {function}() TO {role}"),
            &format!("REVOKE EXECUTE ON FUNCTION {function}() FROM {role}"),
        )
        .await;
    }
}

async fn assert_guard_trigger_faults(
    admin: &sqlx::PgPool,
    writer: &PgStore,
    guard: (&str, &str, &str, &str),
) {
    let (relation, name, function, event) = guard;
    let restore = format!("DROP TRIGGER IF EXISTS {name} ON {relation}; CREATE TRIGGER {name} BEFORE {event} ON {relation} FOR EACH ROW EXECUTE FUNCTION {function}()");
    for replacement in [
        String::new(),
        format!("CREATE TRIGGER {name} AFTER {event} ON {relation} FOR EACH ROW EXECUTE FUNCTION {function}()"),
        format!("CREATE TRIGGER {name} BEFORE {event} ON {relation} FOR EACH STATEMENT EXECUTE FUNCTION {function}()"),
        format!("CREATE TRIGGER {name} BEFORE UPDATE OF id ON {relation} FOR EACH ROW EXECUTE FUNCTION {function}()"),
        format!("CREATE TRIGGER {name} BEFORE {event} ON {relation} FOR EACH ROW WHEN (false) EXECUTE FUNCTION {function}()"),
        format!("CREATE TRIGGER {name} BEFORE {event} ON {relation} FOR EACH ROW EXECUTE FUNCTION canon.guard_receipt_insert()"),
        format!("CREATE TRIGGER {name} BEFORE {event} ON {relation} FOR EACH ROW EXECUTE FUNCTION {function}('unexpected')"),
        format!("CREATE CONSTRAINT TRIGGER {name} AFTER UPDATE ON {relation} DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION {function}()"),
    ] {
        assert_preflight_fault(admin, writer, &format!("DROP TRIGGER {name} ON {relation}; {replacement}"), &restore).await;
    }
    for state in ["DISABLE", "ENABLE REPLICA", "ENABLE ALWAYS"] {
        assert_preflight_fault(
            admin,
            writer,
            &format!("ALTER TABLE {relation} {state} TRIGGER {name}"),
            &format!("ALTER TABLE {relation} ENABLE TRIGGER {name}"),
        )
        .await;
    }
}

#[tokio::test]
async fn fact_integrity_preflight_rejects_signed_body_and_trigger_drift() {
    let (_pg, admin, writer) = preflight_fixture().await;
    assert_signed_guard_faults(
        &admin,
        &writer,
        8,
        ("canon.guard_fact_integrity", "$fact$", "SECURITY DEFINER"),
    )
    .await;
    assert_guard_trigger_faults(
        &admin,
        &writer,
        (
            "canon.fact",
            "guard_fact_integrity",
            "canon.guard_fact_integrity",
            "INSERT OR UPDATE",
        ),
    )
    .await;
    writer.pool.close().await;
    admin.close().await;
}

#[tokio::test]
async fn publication_preflight_rejects_signed_body_and_conditional_trigger_drift() {
    let (_pg, admin, writer) = preflight_fixture().await;
    assert_signed_guard_faults(
        &admin,
        &writer,
        7,
        ("ops.keep_published_run", "$run$", "SECURITY INVOKER"),
    )
    .await;
    assert_guard_trigger_faults(
        &admin,
        &writer,
        (
            "ops.ingest_run",
            "keep_published_run",
            "ops.keep_published_run",
            "UPDATE OF status",
        ),
    )
    .await;
    let sql = sql_and_checksum(MIGRATIONS.iter().find(|(v, _)| *v == 7).unwrap().1).0;
    let body = sql.split("$fact$").nth(1).unwrap();
    let signature = "canon.write_run_fact(p_entity bigint,p_key text,p_num double precision,p_text text,p_bool boolean,p_run bigint,p_receipt bigint)";
    let restore = format!("CREATE OR REPLACE FUNCTION {signature} RETURNS void LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $fact${body}$fact$");
    let faulty = format!("CREATE OR REPLACE FUNCTION {signature} RETURNS void LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $drift$ BEGIN RETURN; END; $drift$");
    assert_preflight_fault(&admin, &writer, &faulty, &restore).await;
    writer.pool.close().await;
    admin.close().await;
}

async fn assert_signed_execution_faults(admin: &sqlx::PgPool, writer: &PgStore) {
    for (version, function, signature, delimiter) in [
        (
            7,
            "canon.write_run_fact",
            "canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint)",
            "$fact$",
        ),
        (
            7,
            "ops.keep_published_run",
            "ops.keep_published_run()",
            "$run$",
        ),
        (
            8,
            "canon.guard_fact_integrity",
            "canon.guard_fact_integrity()",
            "$fact$",
        ),
        (
            9,
            "canon.check_object_rows",
            "canon.check_object_rows()",
            "$rows$",
        ),
        (
            10,
            "canon.guard_receipt_insert",
            "canon.guard_receipt_insert()",
            "$receipt$",
        ),
        (
            10,
            "canon.guard_reference_insert",
            "canon.guard_reference_insert()",
            "$reference$",
        ),
    ] {
        assert_signed_execution_function_faults(
            admin,
            writer,
            (version, function, signature, delimiter),
        )
        .await;
    }
}

async fn assert_signed_execution_function_faults(
    admin: &sqlx::PgPool,
    writer: &PgStore,
    target: (i32, &str, &str, &str),
) {
    let (version, function, signature, delimiter) = target;
    let defaults: bool = sqlx::query_scalar("SELECT NOT proisstrict AND provolatile='v' AND proparallel='u' AND NOT proleakproof AND prosupport=0 FROM pg_proc WHERE oid=to_regprocedure($1)")
        .bind(signature).fetch_one(admin).await.unwrap();
    assert!(defaults, "{signature}");
    let template = MIGRATIONS.iter().find(|(v, _)| *v == version).unwrap().1;
    let sql = sql_and_checksum(template).0;
    let definition = sql.split_once(&format!("FUNCTION {function}(")).unwrap().1;
    let (header, rest) = definition.split_once(delimiter).unwrap();
    let body = rest.split_once(delimiter).unwrap().0;
    let restore =
        format!("CREATE OR REPLACE FUNCTION {function}({header}{delimiter}{body}{delimiter};");
    for change in [
        "STRICT",
        "STABLE",
        "IMMUTABLE",
        "PARALLEL RESTRICTED",
        "PARALLEL SAFE",
        "LEAKPROOF",
        "SUPPORT pg_catalog.textlike_support",
    ] {
        assert_preflight_fault(
            admin,
            writer,
            &format!("ALTER FUNCTION {signature} {change}"),
            &restore,
        )
        .await;
    }
}

async fn write_nullable_execution_facts(store: &crate::gamefacts::GameFactsStore) -> i64 {
    use crate::gamefacts::{Entity, Fact, FactSet, FactValue, Receipt};

    let run = store.begin_run("execution").await.unwrap();
    let raw = store
        .capture(
            run,
            "execution",
            "r1",
            "values.json",
            br#"{"number":120,"text":"nullable","boolean":true}"#.to_vec(),
        )
        .await
        .unwrap();
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: raw.content_sha256,
    };
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: "hero_execution".into(),
            receipt: receipt.clone(),
        }],
        facts: [
            ("number", FactValue::Number(120.0)),
            ("text", FactValue::Text("nullable".into())),
            ("boolean", FactValue::Bool(true)),
        ]
        .into_iter()
        .map(|(key, value)| Fact {
            entity_key: "hero_execution".into(),
            stat_key: key.into(),
            value,
            receipt: receipt.clone(),
        })
        .collect(),
        ..FactSet::default()
    };
    store.apply(run, &raw, "execution", &set).await.unwrap();
    run
}

async fn assert_nullable_execution_facts(
    admin: &sqlx::PgPool,
    store: &crate::gamefacts::GameFactsStore,
    run: i64,
) {
    use crate::gamefacts::Counts;

    let values: Vec<serde_json::Value> = sqlx::query_scalar("SELECT jsonb_build_array(stat_key,value_num,value_text,value_bool) FROM canon.fact ORDER BY stat_key")
        .fetch_all(admin).await.unwrap();
    assert_eq!(
        values,
        vec![
            serde_json::json!(["boolean", null, null, true]),
            serde_json::json!(["number", 120, null, null]),
            serde_json::json!(["text", null, "nullable", null]),
        ]
    );
    store
        .finish_run(run, true, Counts::default())
        .await
        .unwrap();
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.current_fact")
        .fetch_one(admin)
        .await
        .unwrap();
    assert_eq!(visible, 3);
}

async fn assert_admin_receipt_corruption_rollback(
    admin: &sqlx::PgPool,
    writer: &PgStore,
    run: i64,
) {
    let before = published_upgrade_snapshot(admin).await;
    let mut corruption = admin.begin().await.unwrap();
    let changed = sqlx::query("UPDATE canon.receipt SET value_sha256=decode(repeat('00',32),'hex') WHERE ingest_run_id=$1")
        .bind(run).execute(&mut *corruption).await.unwrap();
    assert_eq!(changed.rows_affected(), 4);
    writer.check_gamefacts_schema().await.unwrap();
    corruption.rollback().await.unwrap();
    assert_eq!(published_upgrade_snapshot(admin).await, before);
    writer.check_gamefacts_schema().await.unwrap();
}

#[tokio::test]
async fn signed_execution_defaults_are_checked_before_writes_and_allow_nullable_facts() {
    use crate::gamefacts::GameFactsStore;

    let (_pg, admin, writer) = preflight_fixture().await;
    assert!(published_upgrade_snapshot(&admin)
        .await
        .iter()
        .all(Vec::is_empty));
    assert_signed_execution_faults(&admin, &writer).await;
    let store = GameFactsStore::new(writer.pool.clone());
    let run = write_nullable_execution_facts(&store).await;
    assert_nullable_execution_facts(&admin, &store, run).await;
    assert_admin_receipt_corruption_rollback(&admin, &writer, run).await;
    println!("42 signed execution mutations rejected before run/data creation; 3 nullable facts written and published; admin receipt corruption allowed and rolled back");
    writer.pool.close().await;
    admin.close().await;
}

#[tokio::test]
async fn reader_coherence_preflight_rejects_view_body_owner_options_and_grant_drift() {
    let (_pg, admin, writer) = preflight_fixture().await;
    for (name, suffix) in [
        ("entity_identity", " WHERE false"),
        ("hero_profile", " AND false"),
    ] {
        let definition: String = sqlx::query_scalar("SELECT pg_get_viewdef(to_regclass($1),false)")
            .bind(format!("serve.{name}"))
            .fetch_one(&admin)
            .await
            .unwrap();
        let restore = format!("CREATE OR REPLACE VIEW serve.{name} AS {definition}");
        let changed = format!(
            "CREATE OR REPLACE VIEW serve.{name} AS {}{suffix}",
            definition.trim_end().trim_end_matches(';')
        );
        assert_preflight_fault(&admin, &writer, &changed, &restore).await;
        for (fault, restoration) in [
            (
                format!("ALTER VIEW serve.{name} OWNER TO brain_core_test"),
                format!("ALTER VIEW serve.{name} OWNER TO brain_migrate"),
            ),
            (
                format!("ALTER VIEW serve.{name} SET (security_invoker=true)"),
                format!("ALTER VIEW serve.{name} RESET (security_invoker)"),
            ),
            (
                format!("REVOKE SELECT ON serve.{name} FROM brain_tool"),
                format!("GRANT SELECT ON serve.{name} TO brain_tool"),
            ),
            (
                format!("REVOKE SELECT ON serve.{name} FROM brain_service"),
                format!("GRANT SELECT ON serve.{name} TO brain_service"),
            ),
        ] {
            assert_preflight_fault(&admin, &writer, &fault, &restoration).await;
        }
    }
    writer.pool.close().await;
    admin.close().await;
}

#[tokio::test]
async fn mutation_preflight_rejects_table_and_column_grants_without_restricting_admin() {
    let (_pg, admin, writer) = preflight_fixture().await;
    for (relation, column) in [
        ("raw.object", "revision"),
        ("canon.receipt", "extractor"),
        ("canon.entity", "receipt_id"),
        ("canon.entity_identity", "receipt_id"),
        ("canon.alias", "receipt_id"),
        ("canon.fact", "receipt_id"),
        ("canon.relation", "receipt_id"),
    ] {
        for privilege in [
            format!("UPDATE({column})"),
            "UPDATE".into(),
            "DELETE".into(),
            "TRUNCATE".into(),
        ] {
            assert_preflight_fault(
                &admin,
                &writer,
                &format!("GRANT {privilege} ON {relation} TO brain_ingest"),
                &format!("REVOKE {privilege} ON {relation} FROM brain_ingest"),
            )
            .await;
        }
    }
    let admin_update: bool = sqlx::query_scalar(
        "SELECT has_any_column_privilege(current_user,'canon.receipt','UPDATE')",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert!(admin_update);
    sqlx::query("UPDATE canon.receipt SET extractor=extractor")
        .execute(&admin)
        .await
        .unwrap();
    writer.check_gamefacts_schema().await.unwrap();
    writer.pool.close().await;
    admin.close().await;
}

async fn patch_object(
    store: &crate::gamefacts::GameFactsStore,
    pool: &sqlx::PgPool,
    path: &str,
    title: &str,
    posted_at: &str,
    applied: bool,
) {
    let body = serde_json::json!({"id": path, "title": title, "url": "u", "posted_at": posted_at});
    patch_bytes(store, pool, path, body.to_string().into_bytes(), applied).await;
}

async fn patch_bytes(
    store: &crate::gamefacts::GameFactsStore,
    pool: &sqlx::PgPool,
    path: &str,
    bytes: Vec<u8>,
    applied: bool,
) {
    let run = store.begin_run("deadlock_patchnotes_db").await.unwrap();
    let raw = store
        .capture(run, "deadlock_patchnotes_db", "r", path, bytes)
        .await
        .unwrap();
    if applied {
        receipt_application(pool, raw.id, run, 1).await.unwrap();
    }
    sqlx::query("INSERT INTO ops.patch_observation(path,raw_object_id,run_id) VALUES($1,$2,$3)")
        .bind(path)
        .bind(raw.id)
        .bind(run)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn freshness_views_are_readable_by_ingest() {
    let (_pg, _admin, writer) = preflight_fixture().await;
    let store = crate::gamefacts::GameFactsStore::new(writer.pool.clone());
    let pool = &writer.pool;
    patch_object(&store, pool, "1", "A", "2026-10-01T00:00:00Z", true).await;
    patch_object(&store, pool, "1", "B", "2026-10-02T00:00:00Z", true).await;
    patch_object(&store, pool, "1", "A", "2026-10-01T00:00:00Z", true).await;
    patch_object(&store, pool, "2", "I", "infinity", true).await;
    patch_object(&store, pool, "3", "C", "2026-10-03T00:00:00Z", true).await;
    patch_object(&store, pool, "3", "D", "2026-10-04T00:00:00Z", false).await;
    patch_object(&store, pool, "4", "E", "2026-10-05T00:00:00Z", false).await;
    patch_object(&store, pool, "4", "F", "", false).await;
    patch_object(&store, pool, "4", "E", "2026-10-05T00:00:00Z", false).await;
    patch_object(&store, pool, "5", "G", "2026-10-06T00:00:00Z", true).await;
    patch_bytes(&store, pool, "5", vec![0xff, 0xfe], false).await;
    patch_bytes(&store, pool, "6", b"{".to_vec(), false).await;
    patch_bytes(&store, pool, "7", br#"{"title":"N\u0000"}"#.to_vec(), false).await;
    let max = i64::MAX.to_string();
    patch_object(&store, pool, &max, "M", "2026-10-07T00:00:00Z", false).await;
    let run = store.begin_run("deadlock_patchnotes_db").await.unwrap();
    let body = serde_json::json!({"title": "O", "posted_at": "2026-10-08T00:00:00Z"});
    store
        .capture(
            run,
            "deadlock_patchnotes_db",
            "r",
            "9223372036854775808",
            body.to_string().into_bytes(),
        )
        .await
        .unwrap();
    let rejected = sqlx::query(
        "INSERT INTO ops.patch_observation(path,raw_object_id,run_id)
         SELECT path,id,ingest_run_id FROM raw.object WHERE path='9223372036854775808'",
    )
    .execute(pool)
    .await;
    assert!(rejected.is_err());
    let listing: Vec<(i64, String)> =
        sqlx::query_as("SELECT patch_id,label FROM ops.patch_listing ORDER BY patch_id")
            .fetch_all(pool)
            .await
            .unwrap();
    assert_eq!(
        listing,
        vec![
            (1, "A".into()),
            (3, "D".into()),
            (4, "E".into()),
            (5, "G".into()),
            (i64::MAX, "M".into())
        ]
    );
    for view in ["ops.patch_freshness", "ops.game_data_freshness"] {
        sqlx::query(&format!("SELECT * FROM {view}"))
            .fetch_all(pool)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn publication_predicate_exposes_only_live_published_receipts() {
    let (_pg, admin, writer) = preflight_fixture().await;
    let store = crate::gamefacts::GameFactsStore::new(writer.pool.clone());
    let run = write_nullable_execution_facts(&store).await;
    let ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM canon.receipt ORDER BY id")
        .fetch_all(&admin)
        .await
        .unwrap();
    assert_eq!(ids.len(), 4);
    let protected: bool = sqlx::query_scalar(
        "SELECT p.prosecdef AND p.proisstrict AND p.provolatile='s'
            AND p.proconfig=ARRAY['search_path=pg_catalog, pg_temp']
            AND r.rolname='brain_migrate'
            AND NOT has_function_privilege('brain_ingest',p.oid,'EXECUTE')
            AND NOT has_function_privilege('brain_readonly',p.oid,'EXECUTE')
            AND NOT EXISTS(SELECT 1 FROM aclexplode(p.proacl) a WHERE a.grantee=0)
        FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner
        WHERE p.oid='serve.receipt_is_published(bigint)'::regprocedure",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert!(protected);
    for published in [false, true] {
        if published {
            store
                .finish_run(run, true, crate::gamefacts::Counts::default())
                .await
                .unwrap();
        }
        for role in ["brain_tool", "brain_service"] {
            let restricted: bool = sqlx::query_scalar(
                "SELECT NOT EXISTS(SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
                    WHERE n.nspname IN('canon','raw') AND c.relkind IN('r','p','v','m','f')
                    AND (has_table_privilege($1,c.oid,'SELECT') OR has_any_column_privilege($1,c.oid,'SELECT')))",
            )
            .bind(role)
            .fetch_one(&admin)
            .await
            .unwrap();
            assert!(restricted, "{role} canonical/raw access");
            let mut tx = admin.begin().await.unwrap();
            sqlx::query(&format!("SET LOCAL ROLE {role}"))
                .execute(&mut *tx)
                .await
                .unwrap();
            let matching: bool = sqlx::query_scalar(
                "SELECT bool_and(serve.receipt_is_published(id)=$2
                    AND serve.receipt_is_published(id)=EXISTS(SELECT 1 FROM serve.published_receipt p WHERE p.id=candidate.id))
                FROM unnest($1::bigint[]) candidate(id)",
            )
            .bind(&ids)
            .bind(published)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
            assert!(matching, "{role} published={published}");
            let hidden: bool = sqlx::query_scalar(
                "SELECT NOT serve.receipt_is_published(9223372036854775807)
                    AND serve.receipt_is_published(NULL) IS NULL",
            )
            .fetch_one(&mut *tx)
            .await
            .unwrap();
            assert!(hidden);
            tx.rollback().await.unwrap();
        }
    }
    writer.pool.close().await;
    admin.close().await;
}

async fn receipt_application(
    pool: &sqlx::PgPool,
    raw: i64,
    run: i64,
    rows: usize,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) SELECT $1,$2,'cap','raw_bytes','',decode(repeat('00',32),'hex') FROM generate_series(1,$3)")
        .bind(raw).bind(run).bind(rows as i32).execute(&mut *tx).await?;
    tx.commit().await
}

#[tokio::test]
async fn application_cap_upgrade_preserves_legacy_receipts_and_rejects_direct_4097() {
    use crate::gamefacts::{limits, FactSet, GameFactsStore};
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&admin).await.unwrap();
    let migrator = PgStore::new(admin.clone());
    let legacy: Vec<_> = MIGRATIONS.iter().copied().filter(|(v, _)| *v < 9).collect();
    migrator
        .migrate_gamefacts_registered(&legacy)
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    let users: (String, String, bool) = sqlx::query_as("SELECT session_user::text,current_user::text,(SELECT rolsuper FROM pg_roles WHERE rolname=current_user)")
        .fetch_one(&writer).await.unwrap();
    assert_eq!(users, ("brain_ingest".into(), "brain_ingest".into(), false));
    let store = GameFactsStore::new(writer.clone());
    let run = store.begin_run("cap").await.unwrap();
    let a = store
        .capture(run, "cap", "r1", "hero.json", b"120".to_vec())
        .await
        .unwrap();
    receipt_application(&writer, a.id, run, limits::ROWS)
        .await
        .unwrap();
    let before: Vec<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(r) FROM canon.receipt r ORDER BY id")
            .fetch_all(&admin)
            .await
            .unwrap();
    let served: Vec<(String, String)> = sqlx::query_as("SELECT attname::text,format_type(atttypid,atttypmod) FROM pg_attribute WHERE attrelid='serve.published_receipt'::regclass AND attnum>0 AND NOT attisdropped ORDER BY attnum")
        .fetch_all(&admin).await.unwrap();
    migrator.migrate_gamefacts().await.unwrap();
    PgStore::new(writer.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    let after: Vec<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(r)-'application_xid' FROM canon.receipt r ORDER BY id")
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(before, after);
    let shape: Vec<(String, String)> = sqlx::query_as("SELECT attname::text,format_type(atttypid,atttypmod) FROM pg_attribute WHERE attrelid='serve.published_receipt'::regclass AND attnum>0 AND NOT attisdropped ORDER BY attnum")
        .fetch_all(&admin).await.unwrap();
    assert_eq!(shape, served);
    let defaults: bool =
        sqlx::query_scalar("SELECT bool_and(application_xid IS NOT NULL) FROM canon.receipt")
            .fetch_one(&writer)
            .await
            .unwrap();
    assert!(defaults);
    assert_integrity_denied(receipt_application(&writer, a.id, run, limits::ROWS + 1).await);
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(rows, limits::ROWS as i64);
    receipt_application(&writer, a.id, run, limits::ROWS)
        .await
        .unwrap();
    let spoof = sqlx::query("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256,application_xid) VALUES($1,$2,'cap','raw_bytes','',decode(repeat('00',32),'hex'),'1'::xid8)")
        .bind(a.id).bind(run).execute(&writer).await.unwrap_err();
    assert_eq!(
        spoof.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
    store
        .apply(run, &a, "cap", &FactSet::default())
        .await
        .unwrap();
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(rows, (2 * limits::ROWS + 1) as i64);
    writer.close().await;
    admin.close().await;
}

async fn published_upgrade_snapshot(pool: &sqlx::PgPool) -> Vec<Vec<serde_json::Value>> {
    let mut snapshot = Vec::new();
    for table in [
        "raw.object",
        "ops.ingest_run",
        "canon.receipt",
        "canon.entity",
        "canon.entity_identity",
        "canon.alias",
        "canon.fact",
        "canon.relation",
    ] {
        snapshot.push(
            sqlx::query_scalar(&format!(
                "SELECT to_jsonb(r) FROM {table} r ORDER BY to_jsonb(r)::text"
            ))
            .fetch_all(pool)
            .await
            .unwrap(),
        );
    }
    snapshot
}

#[tokio::test]
async fn published_insert_upgrade_preserves_legacy_rows_and_serving_shape() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&admin).await.unwrap();
    let migrator = PgStore::new(admin.clone());
    let legacy: Vec<_> = MIGRATIONS
        .iter()
        .copied()
        .filter(|(v, _)| *v < 10)
        .collect();
    migrator
        .migrate_gamefacts_registered(&legacy)
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.username("brain_ingest"))
        .await
        .unwrap();
    let users: (String, String) = sqlx::query_as("SELECT session_user::text,current_user::text")
        .fetch_one(&writer)
        .await
        .unwrap();
    assert_eq!(users, ("brain_ingest".into(), "brain_ingest".into()));
    let store = crate::gamefacts::GameFactsStore::new(writer.clone());
    let run = store.begin_run("upgrade").await.unwrap();
    let raw = store
        .capture(run, "upgrade", "r1", "hero.json", b"120".to_vec())
        .await
        .unwrap();
    let receipt = integrity_receipt(&writer, raw.id, run).await;
    let entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','hero_upgrade',$1) RETURNING id")
        .bind(receipt).fetch_one(&writer).await.unwrap();
    insert_integrity_fact(&writer, entity, "health", run, receipt)
        .await
        .unwrap();
    store
        .finish_run(run, true, crate::gamefacts::Counts::default())
        .await
        .unwrap();
    let late = integrity_receipt(&writer, raw.id, run).await;
    sqlx::raw_sql(&format!("INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','hero_legacy_late',{late}); INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES({entity},{late}); INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) VALUES({entity},'legacy','de',{late}); INSERT INTO canon.relation(from_entity,kind,to_entity,receipt_id) VALUES({entity},'legacy',{entity},{late})"))
        .execute(&writer).await.unwrap();
    let before = published_upgrade_snapshot(&admin).await;
    let columns: Vec<(String, String, String)> = sqlx::query_as("SELECT c.relname::text,a.attname::text,pg_catalog.format_type(a.atttypid,a.atttypmod) FROM pg_catalog.pg_namespace n JOIN pg_catalog.pg_class c ON c.relnamespace=n.oid JOIN pg_catalog.pg_attribute a ON a.attrelid=c.oid WHERE n.nspname='serve' AND c.relkind='v' AND a.attnum>0 AND NOT a.attisdropped ORDER BY c.relname,a.attnum")
        .fetch_all(&admin).await.unwrap();
    migrator.migrate_gamefacts().await.unwrap();
    migrator.migrate_gamefacts().await.unwrap();
    PgStore::new(writer.clone())
        .check_gamefacts_schema()
        .await
        .unwrap();
    assert_eq!(published_upgrade_snapshot(&admin).await, before);
    let relations: Vec<_> = columns.iter().map(|(name, _, _)| name.clone()).collect();
    let after_columns: Vec<(String, String, String)> = sqlx::query_as("SELECT c.relname::text,a.attname::text,pg_catalog.format_type(a.atttypid,a.atttypmod) FROM pg_catalog.pg_namespace n JOIN pg_catalog.pg_class c ON c.relnamespace=n.oid JOIN pg_catalog.pg_attribute a ON a.attrelid=c.oid WHERE n.nspname='serve' AND c.relkind='v' AND c.relname::text=ANY($1) AND a.attnum>0 AND NOT a.attisdropped ORDER BY c.relname,a.attnum")
        .bind(relations).fetch_all(&admin).await.unwrap();
    assert_eq!(columns, after_columns);
    assert_integrity_denied(sqlx::query("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'late','raw_bytes','',decode(repeat('00',32),'hex'))")
        .bind(raw.id).bind(run).execute(&writer).await.map(|_| ()));
    assert_eq!(published_upgrade_snapshot(&admin).await, before);
    writer.close().await;
    admin.close().await;
}
