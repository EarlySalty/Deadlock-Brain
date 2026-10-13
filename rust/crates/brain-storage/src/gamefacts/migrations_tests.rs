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
