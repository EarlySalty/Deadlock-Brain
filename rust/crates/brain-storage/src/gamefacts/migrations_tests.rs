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
