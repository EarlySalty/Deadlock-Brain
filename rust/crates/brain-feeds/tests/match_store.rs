use brain_contracts::{
    source::origin_from_record, AnswerProfile, AuthorizedContext, Budget, CorpusRelease,
    DocumentStorePort, Principal, Query, RetrievalPort, SnapshotReadPort, SourceVisibility,
};
use brain_feeds::{
    deadlock_match::{
        commit_match_batch, match_release_from_batches, prepare_match_metadata_batch,
        prepare_revoke_match_batch, MatchScope,
    },
    FeedPolicy,
};
use brain_storage::{LocalPgReader, PgStore};
use dbrain_retrieval::ReleaseRetriever;
use dbrain_sources::core::http::SourceHttpResponse;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::Command,
    time::Duration,
};

const BODY: &[u8] = br#"[{"match_id":92685682,"players":[{"account_id":281768392,"hero_id":18}]}]"#;
const URL: &str = "https://api.deadlock-api.com/v1/matches/metadata?match_ids=92685682&account_ids=281768392&only_filtered_players=true&limit=1&format=json";

fn response() -> SourceHttpResponse {
    SourceHttpResponse {
        url: URL.into(),
        status: 200,
        content: BODY.into(),
        headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
        observed_at: 1_790_000_000,
        attempts: 1,
    }
}

fn scope() -> MatchScope {
    MatchScope {
        account_id: "281768392".into(),
        match_id: "92685682".into(),
    }
}

fn policy() -> FeedPolicy {
    FeedPolicy {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::from(["account:281768392".into()]),
        provider_egress_allowed: false,
        publication_allowed: false,
        raw_retention_allowed: false,
    }
}

fn principal(scopes: &[&str]) -> Principal {
    Principal {
        actor_id: "fixture".into(),
        channel: "fixture".into(),
        scopes: scopes.iter().map(|scope| (*scope).into()).collect(),
        provider_egress: BTreeSet::from(["public".into(), "private".into()]),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires BRAIN_MATCH_TEST_PG_SOCKET and BRAIN_MATCH_TEST_PG_PORT for a disposable Unix-socket PostgreSQL cluster"]
async fn postgres_match_commit_release_readback_replay_and_revoke() {
    assert!(std::env::var_os("PGPASSWORD").is_none());
    let socket = std::env::var("BRAIN_MATCH_TEST_PG_SOCKET").unwrap();
    assert!(socket.ends_with("/.match-test-pg"));
    let port: u16 = std::env::var("BRAIN_MATCH_TEST_PG_PORT")
        .unwrap()
        .parse()
        .unwrap();
    let user = std::env::var("USER").unwrap();
    let database = format!("brain_match_test_{port}");
    let options = PgConnectOptions::new_without_pgpass()
        .host(&socket)
        .port(port)
        .username(&user)
        .database(&database);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(address.is_none());
    let store = PgStore::new(pool);
    store.migrate_core().await.unwrap();
    let base = CorpusRelease {
        release_id: "match-fixture-base".into(),
        knowledge_version: "match-fixture-knowledge".into(),
        patch: "2026-09-24".into(),
        created_at_epoch: 1_790_000_000,
        source_revisions: BTreeMap::new(),
    };
    store.publish(&base).await.unwrap();
    let scope = scope();
    let policy = policy();
    let first = prepare_match_metadata_batch(&scope, response(), &policy, None).unwrap();
    assert!(
        !commit_match_batch(&store, &first, "fixture-owner")
            .await
            .unwrap()
            .replayed
    );
    let published = match_release_from_batches(
        &base,
        std::slice::from_ref(&first),
        "match-fixture-r1",
        1_790_000_001,
    )
    .unwrap();
    store.publish(&published).await.unwrap();
    let reader = LocalPgReader::new(Path::new(&socket), port, &user, &database).unwrap();
    let (retriever, evidence, query, context) = tokio::task::block_in_place(|| {
        let snapshot = reader.read_snapshot("match-fixture-r1").unwrap();
        assert_eq!(snapshot.revisions.len(), 1);
        assert!(snapshot
            .authorized(&principal(&[]), false)
            .unwrap()
            .is_empty());
        assert!(snapshot
            .authorized(&principal(&["game.public"]), false)
            .unwrap()
            .is_empty());
        let authorized = snapshot
            .authorized(&principal(&["account:281768392"]), false)
            .unwrap();
        assert_eq!(authorized.len(), 1);
        assert_eq!(authorized[0].metadata["match_id"], "92685682");
        assert_eq!(authorized[0].metadata["account_id"], "281768392");
        assert_eq!(authorized[0].metadata["schema_version"], "0.1.0");
        assert!(!authorized[0].metadata["schema_sha256"].is_empty());
        assert!(!authorized[0].metadata["http_body_sha256"].is_empty());
        assert!(!origin_from_record(&authorized[0])
            .unwrap()
            .origin_artifacts
            .is_empty());
        assert!(snapshot
            .authorized(&principal(&["account:281768392"]), true)
            .unwrap()
            .is_empty());
        let query = Query {
            request_id: "match-fixture-request".into(),
            conversation_id: "match-fixture-conversation".into(),
            text: "Match 92685682 account 281768392".into(),
            domain: None,
            requested_scopes: BTreeSet::from(["account:281768392".into()]),
            profile: AnswerProfile::Explain,
            patch: Some("2026-09-24".into()),
            mode: None,
        };
        let context = AuthorizedContext {
            principal: principal(&["account:281768392"]),
            conversation_id: query.conversation_id.clone(),
            knowledge_release: "match-fixture-r1".into(),
            deadline_ms: 8000,
            budget: Budget::default(),
        };
        let retriever = ReleaseRetriever::new(reader.clone(), 6);
        let evidence = retriever.retrieve(&query, &context).unwrap();
        assert!(!evidence.is_empty());
        assert_eq!(evidence[0].visibility, SourceVisibility::Private);
        assert!(evidence[0].content.contains("92685682"));
        retriever
            .validate_evidence(&query, &context, &evidence, false)
            .unwrap();
        assert!(retriever
            .validate_evidence(&query, &context, &evidence, true)
            .is_err());
        let anonymous = AuthorizedContext {
            principal: principal(&[]),
            ..context.clone()
        };
        assert!(retriever.retrieve(&query, &anonymous).is_err());
        let anonymous_query = Query {
            requested_scopes: BTreeSet::new(),
            ..query.clone()
        };
        assert!(retriever
            .retrieve(&anonymous_query, &anonymous)
            .unwrap()
            .is_empty());
        (retriever, evidence, query, context)
    });

    let checkpoint = store
        .checkpoint(&first.checkpoint.source_id)
        .await
        .unwrap()
        .unwrap();
    let unchanged =
        prepare_match_metadata_batch(&scope, response(), &policy, Some(&checkpoint)).unwrap();
    assert!(unchanged.records.is_empty());
    commit_match_batch(&store, &unchanged, "fixture-owner")
        .await
        .unwrap();
    let replay =
        match_release_from_batches(&published, &[unchanged], "match-fixture-r2", 1_790_000_002)
            .unwrap();
    assert_eq!(replay.source_revisions, published.source_revisions);
    store.publish(&replay).await.unwrap();

    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("revoke.json");
    fs::write(
        &path,
        serde_json::to_vec(&json!({
            "socket_dir": socket,
            "port": port,
            "database": database,
            "username": user,
            "owner": "fixture-owner",
            "account_id": scope.account_id,
            "match_id": scope.match_id,
            "base_release_id": "match-fixture-r2",
            "release_id": "match-fixture-r3",
            "created_at_epoch": 1_790_000_003,
            "schema_sha256": dbrain_sources::schema_watch::OpenApiSnapshot::pinned().unwrap().schema_sha256,
            "expected_raw_sha256": null,
        }))
        .unwrap(),
    )
    .unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    let command = Command::new(env!("CARGO_BIN_EXE_brain-match-ingest"))
        .arg("revoke")
        .arg("--config")
        .arg(&path)
        .env_remove("PGPASSWORD")
        .env_remove("DATABASE_URL")
        .output()
        .unwrap();
    assert!(
        command.status.success(),
        "{}",
        String::from_utf8_lossy(&command.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&command.stdout).unwrap();
    assert_eq!(report["release_id"], "match-fixture-r3");
    assert_eq!(report["sources"][0]["tombstones"], 1);
    tokio::task::block_in_place(|| {
        assert!(reader
            .read_snapshot("match-fixture-r3")
            .unwrap()
            .revisions
            .is_empty());
        assert!(reader
            .read_snapshot("match-fixture-r1")
            .unwrap()
            .authorized(&principal(&["account:281768392"]), false)
            .unwrap()
            .is_empty());
        assert!(retriever.retrieve(&query, &context).unwrap().is_empty());
        assert!(retriever
            .validate_evidence(&query, &context, &evidence, false)
            .is_err());
    });
    let new_checkpoint = store
        .checkpoint(&first.checkpoint.source_id)
        .await
        .unwrap()
        .unwrap();
    let repeated = prepare_revoke_match_batch(&scope, &policy, &new_checkpoint).unwrap();
    assert!(repeated.records.is_empty());
    tokio::task::block_in_place(|| {
        drop(retriever);
        drop(reader);
    });
}

#[test]
fn corrupt_hash_and_identity_do_not_form_batches() {
    let batch = prepare_match_metadata_batch(&scope(), response(), &policy(), None).unwrap();
    let mut corrupted = batch.clone();
    corrupted.records[0].content_hash = "0".repeat(64);
    assert!(corrupted.validate().is_err());
    let mut forged_scope = batch.clone();
    forged_scope.records[0].visibility = SourceVisibility::Public;
    assert!(forged_scope.validate().is_err());
    let mut missing = response();
    missing.content = b"[]".to_vec();
    assert!(prepare_match_metadata_batch(&scope(), missing, &policy(), None).is_err());
}
