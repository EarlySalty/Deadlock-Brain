use super::{apply_gamefacts_migrations, check_gamefacts, GAMEFACTS_MIGRATIONS};
use sqlx::{Connection, PgConnection};

#[path = "scratch_pg.rs"]
mod scratch_pg;

const V1_MIGRATIONS: &[(i32, &str)] = &[GAMEFACTS_MIGRATIONS[0]];
const APPEND_THREE: &str = "BEGIN;
    CREATE TABLE ops.migration_order (
        ordinal integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        version integer NOT NULL UNIQUE,
        recorded_versions integer[] NOT NULL,
        transaction_id bigint NOT NULL DEFAULT txid_current(),
        migration_owner text NOT NULL DEFAULT current_user,
        migration_lock_held boolean NOT NULL
    );
    INSERT INTO ops.migration_order(version,recorded_versions,migration_lock_held)
        VALUES (3,ARRAY(SELECT version FROM ops.gamefacts_schema_version ORDER BY version),
            EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND pid=pg_backend_pid()
                AND classid=(742110026112::bigint >> 32)::oid
                AND objid=(742110026112::bigint & 4294967295)::oid AND objsubid=1 AND granted));
    COMMIT;";
const APPEND_TWO: &str = "BEGIN;
    INSERT INTO ops.migration_order(version,recorded_versions,migration_lock_held)
        VALUES (2,ARRAY(SELECT version FROM ops.gamefacts_schema_version ORDER BY version),
            EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND pid=pg_backend_pid()
                AND classid=(742110026112::bigint >> 32)::oid
                AND objid=(742110026112::bigint & 4294967295)::oid AND objsubid=1 AND granted));
    COMMIT;";

async fn fixture() -> (scratch_pg::ScratchPg, PgConnection) {
    let scratch = scratch_pg::ScratchPg::start();
    let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
        .host(scratch.directory.join("socket").to_str().unwrap())
        .port(55439)
        .username("brain_core_test")
        .password("")
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);
    let mut admin = PgConnection::connect_with(&options.clone().database("postgres"))
        .await
        .unwrap();
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&mut admin)
        .await
        .unwrap();
    assert_eq!(directory, scratch.directory.join("data").to_str().unwrap());
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE",
    )
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::query("CREATE DATABASE brain_gamefacts_contract OWNER brain_migrate")
        .execute(&mut admin)
        .await
        .unwrap();
    admin.close().await.unwrap();
    let connection = PgConnection::connect_with(&options.database("brain_gamefacts_contract"))
        .await
        .unwrap();
    (scratch, connection)
}

async fn stamps(connection: &mut PgConnection) -> Vec<(i32, i64)> {
    sqlx::query_as(
        "SELECT version,xmin::text::bigint FROM ops.gamefacts_schema_version ORDER BY version",
    )
    .fetch_all(connection)
    .await
    .unwrap()
}

fn appended_migrations() -> [(i32, &'static str); 3] {
    [GAMEFACTS_MIGRATIONS[0], (3, APPEND_THREE), (2, APPEND_TWO)]
}

#[tokio::test]
async fn real_postgres_append_order_separate_transactions_and_rerun_skips() {
    let (_scratch, mut connection) = fixture().await;
    let migrations = appended_migrations();
    apply_gamefacts_migrations(&mut connection, &migrations)
        .await
        .unwrap();
    let applied: Vec<(i32, Vec<i32>, i64, String, bool)> = sqlx::query_as(
        "SELECT version,recorded_versions,transaction_id,migration_owner,migration_lock_held FROM ops.migration_order ORDER BY ordinal",
    )
    .fetch_all(&mut connection)
    .await
    .unwrap();
    assert_eq!(applied.len(), 2);
    assert_eq!(applied[0].0, 3);
    assert_eq!(applied[0].1, [1]);
    assert_eq!(applied[1].0, 2);
    assert_eq!(applied[1].1, [1, 3]);
    assert!(applied.iter().all(|row| row.3 == "brain_migrate"));
    assert!(applied.iter().all(|row| row.4));
    let before = stamps(&mut connection).await;
    assert_eq!(
        before.iter().map(|row| row.0).collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert_ne!(before[0].1, before[1].1);
    assert_ne!(before[0].1, before[2].1);
    assert_ne!(before[1].1, before[2].1);
    assert_eq!(applied[0].2, before[2].1);
    assert_eq!(applied[1].2, before[1].1);
    apply_gamefacts_migrations(&mut connection, &migrations)
        .await
        .unwrap();
    assert_eq!(stamps(&mut connection).await, before);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ops.migration_order")
        .fetch_one(&mut connection)
        .await
        .unwrap();
    assert_eq!(count, 2);
    let reordered = [migrations[2], migrations[0], migrations[1]];
    check_gamefacts(&mut connection, &reordered).await.unwrap();
    apply_gamefacts_migrations(&mut connection, &reordered)
        .await
        .unwrap();
    assert_eq!(stamps(&mut connection).await, before);
    connection.close().await.unwrap();
}

#[tokio::test]
async fn real_postgres_failed_append_rolls_back_only_its_version() {
    let (_scratch, mut connection) = fixture().await;
    let migrations = appended_migrations();
    let broken = [
        migrations[0],
        migrations[1],
        (
            2,
            "BEGIN; CREATE TABLE ops.rollback_probe(id integer);
            UPDATE ops.migration_order SET version=99;
            SELECT 1/0; COMMIT;",
        ),
    ];
    assert!(apply_gamefacts_migrations(&mut connection, &broken)
        .await
        .is_err());
    let before = stamps(&mut connection).await;
    assert_eq!(before.iter().map(|row| row.0).collect::<Vec<_>>(), [1, 3]);
    let rollback_absent: bool =
        sqlx::query_scalar("SELECT to_regclass('ops.rollback_probe') IS NULL")
            .fetch_one(&mut connection)
            .await
            .unwrap();
    assert!(rollback_absent);
    let order: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.migration_order ORDER BY ordinal")
            .fetch_all(&mut connection)
            .await
            .unwrap();
    assert_eq!(order, [3]);
    check_gamefacts(&mut connection, &migrations[..2])
        .await
        .unwrap();
    assert!(check_gamefacts(&mut connection, &migrations).await.is_err());
    apply_gamefacts_migrations(&mut connection, &migrations)
        .await
        .unwrap();
    let after = stamps(&mut connection).await;
    assert_eq!(after[0], before[0]);
    assert_eq!(after[2], before[1]);
    check_gamefacts(&mut connection, &migrations).await.unwrap();
    connection.close().await.unwrap();
}

#[tokio::test]
async fn real_postgres_failed_v1_leaves_no_schema_or_stamp() {
    let (_scratch, mut connection) = fixture().await;
    let broken = GAMEFACTS_MIGRATIONS[0]
        .1
        .replace("CREATE EXTENSION", "SELECT 1/0; CREATE EXTENSION");
    assert!(apply_gamefacts_migrations(&mut connection, &[(1, &broken)])
        .await
        .is_err());
    let absent: bool = sqlx::query_scalar(
        "SELECT to_regclass('ops.gamefacts_schema_version') IS NULL
        AND NOT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_tool')",
    )
    .fetch_one(&mut connection)
    .await
    .unwrap();
    assert!(absent);
    apply_gamefacts_migrations(&mut connection, V1_MIGRATIONS)
        .await
        .unwrap();
    check_gamefacts(&mut connection, V1_MIGRATIONS)
        .await
        .unwrap();
    let no_extension_tables: bool = sqlx::query_scalar(
        "SELECT to_regclass('canon.item') IS NULL AND to_regclass('canon.timer') IS NULL",
    )
    .fetch_one(&mut connection)
    .await
    .unwrap();
    assert!(no_extension_tables);
    connection.close().await.unwrap();
}

#[tokio::test]
async fn real_postgres_missing_extra_and_positive_version_contract() {
    let (_scratch, mut connection) = fixture().await;
    apply_gamefacts_migrations(&mut connection, V1_MIGRATIONS)
        .await
        .unwrap();
    let migrations = appended_migrations();
    assert!(check_gamefacts(&mut connection, &migrations).await.is_err());
    for (version, expected_code) in [(0, "23514"), (-1, "23514"), (1, "23505")] {
        let error = sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES ($1)")
            .bind(version)
            .execute(&mut connection)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some(expected_code)
        );
    }
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES (2)")
        .execute(&mut connection)
        .await
        .unwrap();
    assert!(check_gamefacts(&mut connection, V1_MIGRATIONS)
        .await
        .is_err());
    assert!(apply_gamefacts_migrations(&mut connection, V1_MIGRATIONS)
        .await
        .is_err());
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES (6)")
        .execute(&mut connection)
        .await
        .unwrap();
    let before = stamps(&mut connection).await;
    assert!(apply_gamefacts_migrations(&mut connection, &migrations)
        .await
        .is_err());
    assert_eq!(stamps(&mut connection).await, before);
    let untouched: bool = sqlx::query_scalar("SELECT to_regclass('ops.migration_order') IS NULL")
        .fetch_one(&mut connection)
        .await
        .unwrap();
    assert!(untouched);
    sqlx::query("DELETE FROM ops.gamefacts_schema_version WHERE version IN (2,6)")
        .execute(&mut connection)
        .await
        .unwrap();
    check_gamefacts(&mut connection, V1_MIGRATIONS)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ops.gamefacts_schema_version WHERE version=1")
        .execute(&mut connection)
        .await
        .unwrap();
    assert!(check_gamefacts(&mut connection, V1_MIGRATIONS)
        .await
        .is_err());
    assert!(apply_gamefacts_migrations(&mut connection, V1_MIGRATIONS)
        .await
        .is_err());
    assert!(stamps(&mut connection).await.is_empty());
    connection.close().await.unwrap();
}

#[tokio::test]
async fn real_postgres_appends_keep_v1_security_and_accept_additional_serialize_triggers() {
    let (_scratch, mut connection) = fixture().await;
    let additional = [
        GAMEFACTS_MIGRATIONS[0],
        (
            3,
            "BEGIN; CREATE TABLE docs.serialized_extension(id integer);
            CREATE TRIGGER extension_write_lock BEFORE INSERT ON docs.serialized_extension
                FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write(); COMMIT;",
        ),
    ];
    apply_gamefacts_migrations(&mut connection, &additional)
        .await
        .unwrap();
    check_gamefacts(&mut connection, &additional).await.unwrap();
    let trigger_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_trigger WHERE tgfoid='ops.serialize_gamefacts_write()'::regprocedure
        AND tgenabled='O' AND tgtype & 1 = 0 AND tgtype & 2 = 2",
    )
    .fetch_one(&mut connection)
    .await
    .unwrap();
    assert_eq!(trigger_count, 15);
    let before = stamps(&mut connection).await;
    let insecure = [
        additional[0],
        additional[1],
        (
            4,
            "BEGIN; GRANT INSERT ON canon.fact TO brain_service; COMMIT;",
        ),
    ];
    assert!(apply_gamefacts_migrations(&mut connection, &insecure)
        .await
        .is_err());
    assert_eq!(stamps(&mut connection).await, before);
    check_gamefacts(&mut connection, &additional).await.unwrap();
    sqlx::raw_sql(
        "DROP TRIGGER extension_write_lock ON docs.serialized_extension;
        ALTER TABLE canon.fact DISABLE TRIGGER fact_write_lock",
    )
    .execute(&mut connection)
    .await
    .unwrap();
    assert!(check_gamefacts(&mut connection, &additional).await.is_err());
    sqlx::query("ALTER TABLE canon.fact ENABLE TRIGGER fact_write_lock")
        .execute(&mut connection)
        .await
        .unwrap();
    check_gamefacts(&mut connection, &additional).await.unwrap();
    connection.close().await.unwrap();
}
