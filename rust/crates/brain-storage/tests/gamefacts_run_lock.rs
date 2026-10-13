use brain_storage::{
    gamefacts::{Counts, Entity, FactSet, GameFactsStore, Receipt},
    PgStore,
};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgConnection, PgPool,
};
use tokio::task::JoinHandle;

#[path = "support/scratch_pg.rs"]
mod scratch_pg;

async fn assert_waiting(pool: &PgPool, waiter: &mut JoinHandle<Result<PgConnection, sqlx::Error>>) {
    loop {
        tokio::select! {
            result = &mut *waiter => panic!("concurrent run acquired the held lock: {result:?}"),
            waiting = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND NOT granted)"
            ).fetch_one(pool) => {
                if waiting.unwrap() {
                    assert!(!waiter.is_finished());
                    return;
                }
            }
        }
    }
}

fn waiter(store: &GameFactsStore) -> JoinHandle<Result<PgConnection, sqlx::Error>> {
    let store = store.clone();
    tokio::spawn(async move { store.lock_run().await })
}

#[tokio::test]
async fn owned_run_lock_serializes_publication_without_consuming_pool_capacity() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    let independent = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&pool).await.unwrap();
    PgStore::new(pool.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let first = GameFactsStore::new(pool.clone());
    let second = GameFactsStore::new(independent.clone());
    for empty in [false, true] {
        let mut guard = first.lock_run().await.unwrap();
        let guard_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut guard)
            .await
            .unwrap();
        let pool_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_ne!(guard_pid, pool_pid);
        let mut blocked = waiter(&second);
        assert_waiting(&pool, &mut blocked).await;
        let run = first.begin_run("lock_test").await.unwrap();
        let bytes = b"{}".to_vec();
        let raw = first
            .capture(
                run,
                "lock_test",
                "r1",
                if empty { "empty" } else { "entity" },
                bytes.clone(),
            )
            .await
            .unwrap();
        assert!(!first.was_applied(raw.id, "v1:4:stub:1:1").await.unwrap());
        let set = FactSet {
            entities: if empty {
                Vec::new()
            } else {
                vec![Entity {
                    kind: "hero".into(),
                    key: "hero_lock_test".into(),
                    receipt: Receipt {
                        locator_kind: "raw_bytes".into(),
                        locator: String::new(),
                        value_sha256: Sha256::digest(&bytes).into(),
                    },
                }]
            },
            ..FactSet::default()
        };
        first.apply(run, &raw, "v1:4:stub:1:1", &set).await.unwrap();
        assert!(!first.was_applied(raw.id, "v1:4:stub:1:1").await.unwrap());
        assert_waiting(&pool, &mut blocked).await;
        first
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
        assert_waiting(&pool, &mut blocked).await;
        drop(guard);
        let next_guard = blocked.await.unwrap().unwrap();
        assert!(second.was_applied(raw.id, "v1:4:stub:1:1").await.unwrap());
        let receipts: i64 =
            sqlx::query_scalar("SELECT count(*) FROM canon.receipt WHERE raw_object_id=$1")
                .bind(raw.id)
                .fetch_one(&independent)
                .await
                .unwrap();
        assert_eq!(receipts, 1);
        drop(next_guard);
    }
    let failure: anyhow::Result<()> = async {
        let _guard = first.lock_run().await?;
        anyhow::bail!("terminal run failure")
    }
    .await;
    assert!(failure.is_err());
    let (ready, acquired) = tokio::sync::oneshot::channel();
    let holder = tokio::spawn(async move {
        let _guard = first.lock_run().await.unwrap();
        ready.send(()).unwrap();
        std::future::pending::<()>().await;
    });
    acquired.await.unwrap();
    let mut cancelled_waiter = waiter(&second);
    assert_waiting(&pool, &mut cancelled_waiter).await;
    cancelled_waiter.abort();
    assert!(cancelled_waiter.await.unwrap_err().is_cancelled());
    let mut blocked = waiter(&second);
    assert_waiting(&pool, &mut blocked).await;
    holder.abort();
    assert!(holder.await.unwrap_err().is_cancelled());
    drop(blocked.await.unwrap().unwrap());
    independent.close().await;
    pool.close().await;
}
