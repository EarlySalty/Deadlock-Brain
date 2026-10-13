use brain_storage::{
    gamefacts::{
        Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject, Receipt, Rejection,
    },
    PgStore,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgConnection, PgPool,
};
use tokio::task::JoinHandle;

#[path = "support/gamefacts_pg.rs"]
pub mod gamefacts_pg;
use gamefacts_pg::{growth, ScratchPg};

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

async fn fixture() -> (ScratchPg, PgPool, PgPool) {
    let pg = ScratchPg::start();
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
    (pg, pool, independent)
}

#[tokio::test]
async fn owned_run_lock_serializes_publication_without_consuming_pool_capacity() {
    let (_pg, pool, independent) = fixture().await;
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
        let set = entity_set(&bytes, empty);
        first.apply(run, &raw, "v1:4:stub:1:1", &set).await.unwrap();
        assert!(!first.was_applied(raw.id, "v1:4:stub:1:1").await.unwrap());
        assert_waiting(&pool, &mut blocked).await;
        first
            .finish_run_guarded(
                &mut guard,
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

fn entity_set(bytes: &[u8], empty: bool) -> FactSet {
    FactSet {
        entities: if empty {
            Vec::new()
        } else {
            vec![Entity {
                kind: "hero".into(),
                key: "hero_lock_test".into(),
                receipt: raw_receipt(bytes),
            }]
        },
        ..FactSet::default()
    }
}

fn raw_receipt(bytes: &[u8]) -> Receipt {
    Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(bytes).into(),
    }
}

fn health_set(bytes: &[u8], value: f64) -> FactSet {
    FactSet {
        facts: vec![Fact {
            entity_key: "hero_lock_test".into(),
            stat_key: "health".into(),
            value: FactValue::Number(value),
            receipt: raw_receipt(bytes),
        }],
        ..entity_set(bytes, false)
    }
}

async fn snapshot(pool: &PgPool, run: i64) -> Vec<Value> {
    sqlx::query_scalar("SELECT to_jsonb(f) FROM canon.fact f WHERE valid_from_run=$1 ORDER BY id")
        .bind(run)
        .fetch_all(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn run_local_replacement_preserves_published_history_and_object_rollback() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(1).await;
    let store = GameFactsStore::new(writer.clone());
    let mut guard = store.lock_run().await.unwrap();
    let role: String = sqlx::query_scalar("SELECT current_user")
        .fetch_one(&mut guard)
        .await
        .unwrap();
    assert_eq!(role, "brain_ingest");
    sqlx::raw_sql("ALTER TABLE canon.fact ADD CONSTRAINT test_poison CHECK(stat_key<>'poison')")
        .execute(&admin)
        .await
        .unwrap();
    for empty in [false, true] {
        let path = if empty { "empty_twins" } else { "health_twins" };
        let first = store.begin_run("twins").await.unwrap();
        let a = store
            .capture(first, "twins", "r1", path, b"120".to_vec())
            .await
            .unwrap();
        let set = |raw: &RawObject, value| {
            if empty {
                FactSet::default()
            } else {
                health_set(&raw.bytes, value)
            }
        };
        store
            .apply(first, &a, "v1:4:hero:1:1", &set(&a, 120.0))
            .await
            .unwrap();
        store
            .finish_run_guarded(
                &mut guard,
                first,
                true,
                Counts {
                    accepted: 1,
                    ..Counts::default()
                },
            )
            .await
            .unwrap();
        let original = snapshot(&admin, first).await;
        let receipts: Vec<Value> = sqlx::query_scalar(
            "SELECT to_jsonb(r) FROM canon.receipt r WHERE ingest_run_id=$1 ORDER BY id",
        )
        .bind(first)
        .fetch_all(&admin)
        .await
        .unwrap();
        let active = store.begin_run("twins").await.unwrap();
        let b = store
            .capture(active, "twins", "r2", path, b"125".to_vec())
            .await
            .unwrap();
        store
            .apply(active, &b, "v1:4:hero:1:1", &set(&b, 125.0))
            .await
            .unwrap();
        assert!(store.was_applied(a.id, "v1:4:hero:1:1").await.unwrap());
        assert!(!store
            .was_applied_in_run(active, a.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        assert!(store
            .was_applied_in_run(active, b.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        assert!(!store
            .was_applied_in_run(active, b.id, "v1:4:hero:1:2")
            .await
            .unwrap());
        if !empty {
            let mut poisoned = set(&a, 120.0);
            let mut bad = poisoned.facts[0].clone();
            bad.stat_key = "poison".into();
            poisoned.facts.push(bad);
            let before = growth(&admin).await;
            let rejection = store
                .apply(active, &a, "v1:4:hero:1:1", &poisoned)
                .await
                .unwrap_err();
            assert_eq!(rejection.code, "sql_write");
            assert!(rejection.detail.contains("23514"));
            store
                .reject(active, a.id, "v1:4:hero:1:1", &rejection)
                .await
                .unwrap();
            assert_eq!(growth(&admin).await, before);
            let pending: f64 =
                sqlx::query_scalar("SELECT value_num FROM canon.fact WHERE valid_from_run=$1")
                    .bind(active)
                    .fetch_one(&admin)
                    .await
                    .unwrap();
            let visible: f64 = sqlx::query_scalar(
                "SELECT value_num FROM serve.current_fact WHERE stat_key='health'",
            )
            .fetch_one(&admin)
            .await
            .unwrap();
            assert_eq!((pending, visible), (125.0, 120.0));
        }
        store
            .apply(active, &a, "v1:4:hero:1:1", &set(&a, 120.0))
            .await
            .unwrap();
        assert!(store
            .was_applied_in_run(active, a.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        assert!(!store
            .was_applied_in_run(active, b.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        store
            .finish_run(
                active,
                false,
                Counts {
                    accepted: 2,
                    rejected: i64::from(!empty),
                    ignored: 1,
                },
            )
            .await
            .unwrap();
        let published_count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM serve.published_receipt WHERE ingest_run_id=$1",
        )
        .bind(active)
        .fetch_one(&admin)
        .await
        .unwrap();
        assert_eq!(published_count, 0);
        store
            .finish_run_guarded(
                &mut guard,
                active,
                true,
                Counts {
                    accepted: 2,
                    rejected: i64::from(!empty),
                    ignored: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(snapshot(&admin, first).await, original);
        if !empty {
            let visible: f64 = sqlx::query_scalar(
                "SELECT value_num FROM serve.current_fact WHERE stat_key='health'",
            )
            .fetch_one(&admin)
            .await
            .unwrap();
            assert_eq!(visible, 120.0);
        }
        let unchanged_receipts: Vec<Value> = sqlx::query_scalar(
            "SELECT to_jsonb(r) FROM canon.receipt r WHERE ingest_run_id=$1 ORDER BY id",
        )
        .bind(first)
        .fetch_all(&admin)
        .await
        .unwrap();
        assert_eq!(unchanged_receipts, receipts);
        let applications: i64 = sqlx::query_scalar(
            "SELECT count(DISTINCT raw_object_id) FROM canon.receipt WHERE ingest_run_id=$1",
        )
        .bind(active)
        .fetch_one(&admin)
        .await
        .unwrap();
        assert_eq!(applications, 2);
        assert!(store.was_applied(a.id, "v1:4:hero:1:1").await.unwrap());
        if !empty {
            let rejection = store
                .apply(first, &b, "v1:4:hero:1:1", &set(&b, 125.0))
                .await
                .unwrap_err();
            assert!(rejection.detail.contains("23514"));
            assert_eq!(snapshot(&admin, first).await, original);
        }
        assert!(store
            .finish_run(first, false, Counts::default())
            .await
            .is_err());
        let denied = sqlx::query("UPDATE canon.fact SET value_num=999 WHERE valid_from_run=$1")
            .bind(first)
            .execute(&writer)
            .await
            .unwrap_err();
        assert_eq!(
            denied.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
        let failed = store.begin_run("twins").await.unwrap();
        store
            .apply(failed, &b, "v1:4:hero:1:2", &set(&b, 125.0))
            .await
            .unwrap();
        store
            .finish_run(
                failed,
                false,
                Counts {
                    accepted: 1,
                    ..Counts::default()
                },
            )
            .await
            .unwrap();
        let next = store.begin_run("twins").await.unwrap();
        assert!(store
            .was_applied_in_run(next, a.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        assert!(!store
            .was_applied_in_run(next, b.id, "v1:4:hero:1:2")
            .await
            .unwrap());
        for (raw, value, version) in [(&a, 120.0, "v1:4:hero:1:1"), (&b, 125.0, "v1:4:hero:1:2")] {
            store
                .apply(next, raw, version, &set(raw, value))
                .await
                .unwrap();
        }
        assert!(!store
            .was_applied_in_run(next, a.id, "v1:4:hero:1:1")
            .await
            .unwrap());
        assert!(store
            .was_applied_in_run(next, b.id, "v1:4:hero:1:2")
            .await
            .unwrap());
        store
            .finish_run_guarded(
                &mut guard,
                next,
                true,
                Counts {
                    accepted: 2,
                    ..Counts::default()
                },
            )
            .await
            .unwrap();
        if !empty {
            let visible: f64 = sqlx::query_scalar(
                "SELECT value_num FROM serve.current_fact WHERE stat_key='health'",
            )
            .fetch_one(&admin)
            .await
            .unwrap();
            assert_eq!(visible, 125.0);
            assert_eq!(snapshot(&admin, first).await, original);
        }
    }
    drop(guard);
    writer.close().await;
    admin.close().await;
}

#[tokio::test]
async fn historical_raw_replay_republishes_latest_health_and_then_ignores_unchanged_input() {
    let (_pg, pool, independent) = fixture().await;
    let store = GameFactsStore::new(pool.clone());
    let mut first_raw = None;
    let mut previous_growth = None;
    for (index, bytes, value, ignored) in [
        (0, b"120.00".as_slice(), 120.0, false),
        (1, b"125".as_slice(), 125.0, false),
        (2, b"120.00".as_slice(), 120.0, false),
        (3, b"120.00".as_slice(), 120.0, true),
    ] {
        let mut guard = store.lock_run().await.unwrap();
        let run = store.begin_run("health_test").await.unwrap();
        let raw = store
            .capture(run, "health_test", "r1", "hero.json", bytes.to_vec())
            .await
            .unwrap();
        if index == 0 {
            first_raw = Some(raw.id);
        } else if index >= 2 {
            assert_eq!(Some(raw.id), first_raw);
        }
        assert_eq!(
            store.was_applied(raw.id, "v1:4:hero:1:1").await.unwrap(),
            ignored
        );
        if !ignored {
            store
                .apply(run, &raw, "v1:4:hero:1:1", &health_set(bytes, value))
                .await
                .unwrap();
        }
        let counts = Counts {
            accepted: i64::from(!ignored),
            ignored: i64::from(ignored),
            rejected: 0,
        };
        store
            .finish_run_guarded(&mut guard, run, true, counts)
            .await
            .unwrap();
        let published: f64 =
            sqlx::query_scalar("SELECT value_num FROM serve.current_fact WHERE stat_key='health'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(published, value);
        let actual: (i64, i64, i64, String) = sqlx::query_as(
            "SELECT accepted,rejected,ignored,status FROM ops.ingest_run WHERE id=$1",
        )
        .bind(run)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            actual,
            (
                i64::from(!ignored),
                0,
                i64::from(ignored),
                "succeeded".into()
            )
        );
        let rows = growth(&pool).await;
        assert_eq!(
            rows,
            match index {
                0 => (1, 1, 2),
                1 => (2, 2, 4),
                _ => (2, 3, 6),
            }
        );
        if ignored {
            assert_eq!(Some(rows), previous_growth);
        }
        previous_growth = Some(rows);
    }
    let applied: i64 =
        sqlx::query_scalar("SELECT count(DISTINCT ingest_run_id) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(applied, 3);
    independent.close().await;
    pool.close().await;
}

async fn publish_empty(
    store: &GameFactsStore,
    source: &str,
    path: &str,
    extractor: &str,
    bytes: &[u8],
) -> RawObject {
    let mut guard = store.lock_run().await.unwrap();
    let run = store.begin_run(source).await.unwrap();
    let raw = store
        .capture(run, source, "r1", path, bytes.to_vec())
        .await
        .unwrap();
    assert!(!store.was_applied(raw.id, extractor).await.unwrap());
    store
        .apply(run, &raw, extractor, &FactSet::default())
        .await
        .unwrap();
    store
        .finish_run_guarded(
            &mut guard,
            run,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    raw
}

#[tokio::test]
async fn current_application_is_scoped_by_source_path_and_length_delimited_extractor_name() {
    let (_pg, pool, independent) = fixture().await;
    let store = GameFactsStore::new(pool.clone());
    let original = publish_empty(&store, "source", "path", "v1:4:é:x:4:ß:y", b"A").await;
    assert!(store
        .was_applied(original.id, "v1:4:é:x:4:ß:y")
        .await
        .unwrap());
    assert!(!store
        .was_applied(original.id, "v1:4:é:x:1:2")
        .await
        .unwrap());
    let upgraded = publish_empty(&store, "source", "path", "v1:4:é:x:1:2", b"A").await;
    assert_eq!(original.id, upgraded.id);
    assert!(!store
        .was_applied(original.id, "v1:4:é:x:4:ß:y")
        .await
        .unwrap());
    publish_empty(&store, "source", "path", "v1:4:é:x:4:ß:y", b"A").await;
    assert!(!store
        .was_applied(original.id, "v1:4:é:x:1:2")
        .await
        .unwrap());
    publish_empty(&store, "source", "other_path", "v1:4:é:x:4:ß:y", b"B").await;
    publish_empty(&store, "other_source", "path", "v1:4:é:x:4:ß:y", b"B").await;
    publish_empty(&store, "source", "path", "v1:2:é:6:x:ß:y", b"B").await;
    publish_empty(&store, "source", "path", "v1:5:é:xz:4:ß:y", b"B").await;
    assert!(store
        .was_applied(original.id, "v1:4:é:x:4:ß:y")
        .await
        .unwrap());
    let changed = publish_empty(&store, "source", "path", "v1:4:é:x:4:ß:y", b"B").await;
    assert!(!store
        .was_applied(original.id, "v1:4:é:x:4:ß:y")
        .await
        .unwrap());
    assert!(store
        .was_applied(changed.id, "v1:4:é:x:4:ß:y")
        .await
        .unwrap());
    let opaque = publish_empty(&store, "legacy", "path", "stub", b"A").await;
    publish_empty(&store, "legacy", "path", "stub_other", b"B").await;
    assert!(store.was_applied(opaque.id, "stub").await.unwrap());
    publish_empty(&store, "legacy", "path", "stub", b"B").await;
    assert!(!store.was_applied(opaque.id, "stub").await.unwrap());
    independent.close().await;
    pool.close().await;
}

#[tokio::test]
async fn failed_and_rejected_runs_do_not_replace_current_successful_application() {
    let (_pg, pool, independent) = fixture().await;
    let store = GameFactsStore::new(pool.clone());
    let original = publish_empty(&store, "source", "path", "stub", b"A").await;
    let mut guard = store.lock_run().await.unwrap();
    let failed = store.begin_run("source").await.unwrap();
    let changed = store
        .capture(failed, "source", "r2", "path", b"B".to_vec())
        .await
        .unwrap();
    store
        .apply(failed, &changed, "stub", &FactSet::default())
        .await
        .unwrap();
    store
        .finish_run_guarded(
            &mut guard,
            failed,
            false,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    assert!(store.was_applied(original.id, "stub").await.unwrap());
    assert!(!store.was_applied(changed.id, "stub").await.unwrap());
    let rejected = store.begin_run("source").await.unwrap();
    store
        .reject(
            rejected,
            changed.id,
            "stub",
            &Rejection::new("bad_input", "Ungültige Eingabe"),
        )
        .await
        .unwrap();
    store
        .finish_run_guarded(
            &mut guard,
            rejected,
            true,
            Counts {
                rejected: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    assert!(store.was_applied(original.id, "stub").await.unwrap());
    assert!(!store.was_applied(changed.id, "stub").await.unwrap());
    drop(guard);
    publish_empty(&store, "source", "path", "stub", b"B").await;
    assert!(!store.was_applied(original.id, "stub").await.unwrap());
    assert!(store.was_applied(changed.id, "stub").await.unwrap());
    independent.close().await;
    pool.close().await;
}

#[tokio::test]
async fn aborting_blocked_terminal_publication_keeps_guard_until_owned_update_resolves() {
    let (_pg, pool, independent) = fixture().await;
    let first = GameFactsStore::new(pool.clone());
    let second = GameFactsStore::new(independent.clone());
    for empty in [false, true] {
        let mut guard = first.lock_run().await.unwrap();
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut guard)
            .await
            .unwrap();
        let run = first.begin_run("cancel_test").await.unwrap();
        let raw = first
            .capture(
                run,
                "cancel_test",
                "r1",
                if empty { "empty" } else { "entity" },
                b"{}".to_vec(),
            )
            .await
            .unwrap();
        first
            .apply(run, &raw, "stub", &entity_set(&raw.bytes, empty))
            .await
            .unwrap();
        let mut row_lock = independent.begin().await.unwrap();
        sqlx::query("SELECT id FROM ops.ingest_run WHERE id=$1 FOR UPDATE")
            .bind(run)
            .fetch_one(&mut *row_lock)
            .await
            .unwrap();
        let publisher = first.clone();
        let mut publication = tokio::spawn(async move {
            publisher
                .finish_run_guarded(
                    &mut guard,
                    run,
                    true,
                    Counts {
                        accepted: 1,
                        ..Counts::default()
                    },
                )
                .await
        });
        loop {
            tokio::select! {
                result = &mut publication => panic!("publication bypassed row lock: {result:?}"),
                blocked = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE pid=$1 AND wait_event_type='Lock' AND query LIKE 'UPDATE ops.ingest_run%')")
                    .bind(pid).fetch_one(&pool) => {
                    if blocked.unwrap() { break; }
                }
            }
        }
        let mut blocked = waiter(&second);
        assert_waiting(&pool, &mut blocked).await;
        publication.abort();
        assert!(publication.await.unwrap_err().is_cancelled());
        assert_waiting(&pool, &mut blocked).await;
        let still_guarded: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity a JOIN pg_locks l ON l.pid=a.pid WHERE a.pid=$1 AND a.wait_event_type='Lock' AND a.query LIKE 'UPDATE ops.ingest_run%' AND l.locktype='advisory' AND l.granted)")
            .bind(pid).fetch_one(&pool).await.unwrap();
        assert!(still_guarded);
        assert!(!first.was_applied(raw.id, "stub").await.unwrap());
        row_lock.rollback().await.unwrap();
        let mut next_guard = blocked.await.unwrap().unwrap();
        let pending: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE pid=$1 AND wait_event_type='Lock' AND query LIKE 'UPDATE ops.ingest_run%')")
            .bind(pid).fetch_one(&pool).await.unwrap();
        assert!(!pending);
        if !second.was_applied(raw.id, "stub").await.unwrap() {
            let retry = second.begin_run("cancel_test").await.unwrap();
            second
                .apply(retry, &raw, "stub", &entity_set(&raw.bytes, empty))
                .await
                .unwrap();
            second
                .finish_run_guarded(
                    &mut next_guard,
                    retry,
                    true,
                    Counts {
                        accepted: 1,
                        ..Counts::default()
                    },
                )
                .await
                .unwrap();
        }
        assert!(second.was_applied(raw.id, "stub").await.unwrap());
        let published: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM serve.published_receipt WHERE raw_object_id=$1",
        )
        .bind(raw.id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(published, 1);
        drop(next_guard);
    }
    independent.close().await;
    pool.close().await;
}
