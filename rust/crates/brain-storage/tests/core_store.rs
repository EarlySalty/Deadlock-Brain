use brain_contracts::{
    CorpusRelease, DocumentStorePort, Principal, SnapshotReadPort, SourceBatch, SourceCheckpoint,
    SourceRecordV2, SourceVisibility,
};
use brain_storage::{MemoryRepository, PgStore};
use std::collections::{BTreeMap, BTreeSet};
fn record(id: &str, revision: u64, text: &str) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "core-fixture".into(),
        logical_id: id.into(),
        revision,
        content_hash: text.into(),
        content: text.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
fn batch(generation: u64, records: Vec<SourceRecordV2>) -> SourceBatch {
    SourceBatch {
        expected_generation: generation - 1,
        checkpoint: SourceCheckpoint {
            source_id: "core-fixture".into(),
            configuration: "fixture-config-v1".into(),
            generation,
            state: serde_json::json!({"generation":generation}),
        },
        records,
    }
}
fn release() -> CorpusRelease {
    CorpusRelease {
        release_id: "fixture-release-1".into(),
        knowledge_version: "v1".into(),
        patch: "p1".into(),
        created_at_epoch: 0,
        source_revisions: BTreeMap::from([(
            "core-fixture".into(),
            BTreeMap::from([("a".into(), 1), ("b".into(), 1)]),
        )]),
    }
}
async fn exercise(store: &dyn DocumentStorePort) {
    let first = batch(1, vec![record("a", 1, "one"), record("b", 1, "two")]);
    let lease = store
        .claim("core-fixture", "worker-a", 30000)
        .await
        .unwrap();
    assert!(store
        .claim("core-fixture", "worker-b", 30000)
        .await
        .is_err());
    assert!(!store.commit(&first, &lease).await.unwrap().replayed);
    assert!(store.commit(&first, &lease).await.unwrap().replayed);
    let second = store
        .claim("core-fixture", "worker-b", 30000)
        .await
        .unwrap();
    assert!(second.fence > lease.fence);
    // The first mutation would succeed, the second conflicts. Neither may become visible.
    let broken = batch(2, vec![record("a", 2, "new"), record("b", 1, "conflict")]);
    assert!(store.commit(&broken, &second).await.is_err());
    assert_eq!(
        store.checkpoint("core-fixture").await.unwrap().unwrap(),
        first.checkpoint
    );
    let valid = batch(2, vec![record("a", 2, "new"), record("b", 2, "updated")]);
    assert!(store.commit(&valid, &lease).await.is_err());
    assert!(!store.commit(&valid, &second).await.unwrap().replayed);
    assert!(store.commit(&first, &lease).await.is_err());
    let r = release();
    store.publish(&r).await.unwrap();
    store.publish(&r).await.unwrap();
    let mut conflicting = r.clone();
    conflicting.patch = "p2".into();
    assert!(store.publish(&conflicting).await.is_err());
    let mut missing = r;
    missing.release_id = "missing".into();
    missing
        .source_revisions
        .get_mut("core-fixture")
        .unwrap()
        .insert("missing-document".into(), 1);
    assert!(store.publish(&missing).await.is_err());
}
#[tokio::test]
async fn memory_atomicity_fences_release_and_restart_contract() {
    let store = MemoryRepository::default();
    exercise(&store).await;
    assert_eq!(
        store.read_snapshot("fixture-release-1").unwrap().revisions[0].content,
        "one"
    );
    assert!(store.read_snapshot("unknown").is_err());
    let principal = Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::from(["public".into()]),
    };
    assert_eq!(
        store
            .read_snapshot("fixture-release-1")
            .unwrap()
            .authorized(&principal, false)
            .unwrap()
            .len(),
        2
    );
    let mut private = record("a", 3, "restricted");
    private.visibility = SourceVisibility::Private;
    private.allowed_scopes.insert("private".into());
    store.apply_record(private).unwrap();
    let hits = store
        .read_snapshot("fixture-release-1")
        .unwrap()
        .authorized(&principal, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].logical_id, "b");
    let mut deleted = record("b", 3, "deleted");
    deleted.tombstone = true;
    store.apply_record(deleted).unwrap();
    assert!(store
        .read_snapshot("fixture-release-1")
        .unwrap()
        .authorized(&principal, false)
        .unwrap()
        .is_empty());
}
#[tokio::test]
async fn expired_memory_worker_cannot_commit() {
    let store = MemoryRepository::default();
    let old = store.claim("core-fixture", "crashed", 1).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let live = store
        .claim("core-fixture", "restarted", 30000)
        .await
        .unwrap();
    let b = batch(1, vec![record("a", 1, "v1")]);
    assert!(store.commit(&b, &old).await.is_err());
    store.commit(&b, &live).await.unwrap();
}
#[test]
fn wire_revisions_cannot_overflow_postgres_bigint() {
    assert!(record("a", u64::MAX, "x").validate().is_err());
    assert!(record("a", i64::MAX as u64, "x").validate().is_ok());
    let mut b = batch(1, vec![record("a", 1, "x")]);
    b.records[0].source_id = "other".into();
    assert!(b.validate().is_err());
}

/// Explicit opt-in only. The isolated socket must belong to a scratch core-test cluster.
#[tokio::test]
#[ignore = "requires isolated PostgreSQL Unix socket: BRAIN_CORE_TEST_PG_SOCKET"]
async fn postgres_atomicity_fences_release_and_restart_contract() {
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    let socket =
        std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("explicit scratch socket required");
    assert!(
        socket.ends_with("/.core-test-pg"),
        "refusing non-scratch socket"
    );
    assert!(std::path::Path::new(&socket).is_absolute());
    let options = PgConnectOptions::new()
        .host(&socket)
        .port(55439)
        .username("brain_core_test")
        .database("postgres");
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_with(options.clone())
        .await
        .unwrap();
    let addresses: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(addresses.is_none(), "test cluster must not use TCP");
    let user: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(user, "brain_core_test");
    // This dedicated test schema is reset ONLY after the explicit scratch socket/user checks.
    sqlx::raw_sql("DROP SCHEMA IF EXISTS brain CASCADE")
        .execute(&pool)
        .await
        .unwrap();
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    store.migrate_core().await.unwrap();
    exercise(&store).await;
    let before = store.snapshot("fixture-release-1").await.unwrap();
    assert_eq!(before.revisions[0].content, "one");
    assert_eq!(before.heads[0].content, "new");
    let native_socket = socket.clone();
    let native_snapshot = tokio::task::spawn_blocking(move || {
        use brain_contracts::store::ConversationOwnershipPort;
        let reader =
            brain_storage::LocalPgReader::new(&native_socket, 55439, "brain_core_test", "postgres")
                .unwrap();
        reader
            .claim_conversation("persisted-conversation", "actor-a")
            .unwrap();
        let restarted =
            brain_storage::LocalPgReader::new(&native_socket, 55439, "brain_core_test", "postgres")
                .unwrap();
        restarted
            .claim_conversation("persisted-conversation", "actor-a")
            .unwrap();
        assert!(restarted
            .claim_conversation("persisted-conversation", "actor-b")
            .is_err());
        reader.read_snapshot("fixture-release-1").unwrap()
    })
    .await
    .unwrap();
    assert_eq!(native_snapshot, before);
    let wrong = record("a", 2, "different");
    assert!(store.apply(&wrong).await.is_err());
    drop(store);
    pool.close().await;
    let reopened = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options)
        .await
        .unwrap();
    let restarted = PgStore::new(reopened.clone());
    assert_eq!(
        restarted
            .checkpoint("core-fixture")
            .await
            .unwrap()
            .unwrap()
            .generation,
        2
    );
    assert_eq!(
        restarted.snapshot("fixture-release-1").await.unwrap(),
        before
    );
    let expired = restarted.claim("core-fixture", "expired", 1).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let replacement = restarted
        .claim("core-fixture", "replacement", 30000)
        .await
        .unwrap();
    let third = batch(3, vec![record("a", 3, "third")]);
    assert!(restarted.commit(&third, &expired).await.is_err());
    restarted.commit(&third, &replacement).await.unwrap();
    reopened.close().await;
}
