//! Opt-in real process + scratch PostgreSQL + loopback provider; never a production DSN.
mod common;
use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderMap},
    routing::post,
    Router,
};
use brain_client::BrainClient;
use brain_contracts::{
    AnswerProfile, AnswerStatus, CorpusRelease, Query, SourceRecordV2, SourceVisibility,
};
use brain_storage::PgStore;
use common::{Service, API_TOKEN, OTHER_TOKEN};
use serde_json::{json, Value};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Clone)]
struct Provider {
    calls: Arc<AtomicUsize>,
    delay_ms: Arc<AtomicU64>,
    started: Arc<tokio::sync::Notify>,
}

async fn answer(
    State(state): State<Provider>,
    headers: HeaderMap,
    body: Bytes,
) -> ([(header::HeaderName, &'static str); 1], String) {
    assert_eq!(
        headers.get(header::AUTHORIZATION).unwrap(),
        &format!("Bearer {}", common::PROVIDER_TOKEN)
    );
    let request: Value = serde_json::from_slice(&body).unwrap();
    let input: Value =
        serde_json::from_str(request["messages"][1]["content"].as_str().unwrap()).unwrap();
    state.calls.fetch_add(1, Ordering::SeqCst);
    state.started.notify_one();
    tokio::time::sleep(Duration::from_millis(state.delay_ms.load(Ordering::SeqCst))).await;
    let grounded = json!({"text": "Abrams has a verified fixture.", "cited_evidence_ids": [input["evidence"][0]["id"]]}).to_string();
    (
        [(header::CONTENT_TYPE, "application/json")],
        json!({
            "model": request["model"], "choices": [{"message": {"content": grounded}}],
            "usage": {"prompt_tokens": 16, "completion_tokens": 10}
        })
        .to_string(),
    )
}

fn query(id: &str) -> Query {
    Query {
        request_id: id.into(),
        conversation_id: format!("conversation-{id}"),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::from(["docs.public".into()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
        domain: None,
    }
}

async fn ask(
    address: &str,
    token: &str,
    query: Query,
) -> Result<brain_contracts::PublicAnswerResponse, brain_client::ClientError> {
    let address = address.to_string();
    let token = token.to_string();
    tokio::task::spawn_blocking(move || {
        BrainClient::new(&address, &token, Duration::from_secs(6))
            .unwrap()
            .answer(&query)
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "scripts/test_brain_serve.sh: isolated scratch PostgreSQL and real brain-serve child"]
async fn binary_loopback_health_readiness_shutdown_and_no_fallback() {
    let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
    assert!(socket.ends_with("/.core-test-pg") && std::path::Path::new(&socket).is_absolute());
    // Fixed database name created only by the dedicated disposable cluster wrapper.
    let options = PgConnectOptions::new()
        .host(&socket)
        .port(55439)
        .username("brain_core_test")
        .database("brain_serve_test");
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options.clone())
        .await
        .unwrap();
    let tcp: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(tcp.is_none());
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    store
        .apply(&SourceRecordV2 {
            source_id: "c1-fixture".into(),
            logical_id: "abrams".into(),
            revision: 1,
            content_hash: "c1-fixture-hash".into(),
            content: "Abrams has a verified fixture.".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        })
        .await
        .unwrap();
    let release = CorpusRelease {
        release_id: "pilot-r1".into(),
        knowledge_version: "pilot-knowledge-v1".into(),
        patch: "c1-patch".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "c1-fixture".into(),
            BTreeMap::from([("abrams".into(), 1)]),
        )]),
    };
    store.publish_release(&release).await.unwrap();
    let provider = Provider {
        calls: Arc::new(AtomicUsize::new(0)),
        delay_ms: Arc::new(AtomicU64::new(0)),
        started: Arc::new(tokio::sync::Notify::new()),
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let provider_address = listener.local_addr().unwrap();
    let (provider_stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let app = Router::new()
        .route("/chat/completions", post(answer))
        .with_state(provider.clone());
    let provider_task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let mut config = common::config();
    config["postgres"]["socket_dir"] = json!(socket);
    config["postgres"]["port"] = json!(55439);
    config["postgres"]["username"] = json!("brain_core_test");
    config["postgres"]["database"] = json!("brain_serve_test");
    config["provider"]["base_url"] = json!(format!("http://{provider_address}"));
    config["kernel"]["cache_entries"] = json!(0);
    config["timeouts"]["postgres_connect_ms"] = json!(500);
    config["timeouts"]["readiness_ms"] = json!(1000);
    config["credentials"].as_array_mut().unwrap().push(json!({
        "token_env": "BRAIN_SERVE_OTHER_TOKEN", "actor_id": "another-client", "channel": "pilot",
        "scopes": ["docs.public"], "provider_egress": ["public"]
    }));
    let mut environment = common::credentials();
    environment.push(("BRAIN_SERVE_OTHER_TOKEN", OTHER_TOKEN));

    // Unknown/mismatched releases and a occupied bind fail before announcing readiness.
    for (field, invalid, expected) in [
        ("id", "missing-release", "release_unavailable"),
        (
            "knowledge_version",
            "wrong-version",
            "knowledge_version_mismatch",
        ),
    ] {
        let mut bad = config.clone();
        bad["release"][field] = json!(invalid);
        let mut child = Service::spawn(&bad, &environment);
        assert!(!child.wait(Duration::from_secs(8)).success());
        assert!(child.log().contains(expected), "{}", child.log());
        assert!(!child.log().contains("listening"));
    }
    let occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mut bad = config.clone();
    bad["bind"] = json!(occupied.local_addr().unwrap().to_string());
    let mut child = Service::spawn(&bad, &environment);
    assert!(!child.wait(Duration::from_secs(8)).success());
    assert!(child.log().contains("bind_failed"));
    drop(occupied);
    assert_eq!(
        provider.calls.load(Ordering::SeqCst),
        0,
        "startup must not call a provider"
    );

    let mut service = Service::spawn(&config, &environment);
    let address = service.address();
    common::assert_ready(&address);
    assert_eq!(
        common::get(&address, "/healthz"),
        (200, r#"{"status":"ok"}"#.into())
    );
    assert_eq!(
        common::get(&address, "/readyz"),
        (200, r#"{"status":"ready"}"#.into())
    );
    let response = ask(&address, API_TOKEN, query("first")).await.unwrap();
    assert_eq!(response.status, AnswerStatus::Answered);
    assert_eq!(response.knowledge_release, release.release_id);
    assert_eq!(response.citations.len(), 1);
    let count = provider.calls.load(Ordering::SeqCst);
    for path in [
        "/ask",
        "/query",
        "/v1/ask",
        "/v1/query",
        "/legacy/answer",
        "/answer",
    ] {
        for method in ["GET", "POST"] {
            assert_eq!(
                common::request(&address, method, path, Some(API_TOKEN), Some(json!({}))).0,
                404
            );
        }
    }
    for token in [None, Some("invalid-fixture-token")] {
        assert_eq!(
            common::request(
                &address,
                "POST",
                "/v1/answer",
                token,
                Some(serde_json::to_value(query("unauthorized")).unwrap())
            )
            .0,
            401
        );
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), count);

    // The manifest is pinned, not "whatever release currently exists"; liveness is independent.
    sqlx::query("DELETE FROM brain.corpus_releases_v1 WHERE release_id=$1")
        .bind(&release.release_id)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(common::get(&address, "/readyz").0, 503);
    assert_eq!(common::get(&address, "/healthz").0, 200);
    let mut changed = release.clone();
    changed.knowledge_version = "replaced-knowledge".into();
    store.publish_release(&changed).await.unwrap();
    assert_eq!(common::get(&address, "/readyz").0, 503);
    sqlx::query("DELETE FROM brain.corpus_releases_v1 WHERE release_id=$1")
        .bind(&release.release_id)
        .execute(&pool)
        .await
        .unwrap();
    store.publish_release(&release).await.unwrap();
    common::assert_ready(&address);

    // Disable connections only for our scratch database, never stop a host PostgreSQL service.
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.database("postgres"))
        .await
        .unwrap();
    sqlx::query("ALTER DATABASE brain_serve_test ALLOW_CONNECTIONS false")
        .execute(&admin)
        .await
        .unwrap();
    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='brain_serve_test'",
    )
    .execute(&admin)
    .await
    .unwrap();
    assert_eq!(common::get(&address, "/readyz").0, 503);
    assert_eq!(common::get(&address, "/healthz").0, 200);
    assert!(ask(&address, API_TOKEN, query("db-down")).await.is_err());
    assert_eq!(provider.calls.load(Ordering::SeqCst), count);
    sqlx::query("ALTER DATABASE brain_serve_test ALLOW_CONNECTIONS true")
        .execute(&admin)
        .await
        .unwrap();
    common::assert_ready(&address);
    service.stop();

    // Persisted ownership survives a real process restart, unlike the former in-test policy.
    let mut service = Service::spawn(&config, &environment);
    let address = service.address();
    common::assert_ready(&address);
    assert!(ask(&address, OTHER_TOKEN, query("first")).await.is_err());
    assert_eq!(provider.calls.load(Ordering::SeqCst), count);
    // Consume any previous notification before observing this in-flight request.
    let _ = tokio::time::timeout(Duration::from_millis(10), provider.started.notified()).await;
    provider.delay_ms.store(500, Ordering::SeqCst);
    let in_flight_address = address.clone();
    let in_flight =
        tokio::spawn(async move { ask(&in_flight_address, API_TOKEN, query("drain")).await });
    tokio::time::timeout(Duration::from_secs(4), provider.started.notified())
        .await
        .unwrap();
    let started = Instant::now();
    service.signal("TERM");
    assert_eq!(
        in_flight.await.unwrap().unwrap().status,
        AnswerStatus::Answered
    );
    assert!(
        service.wait(Duration::from_secs(5)).success(),
        "{}",
        service.log()
    );
    assert!(
        started.elapsed() >= Duration::from_millis(350),
        "must drain the in-flight response"
    );
    assert!(service.log().contains("shutdown_started") && service.log().contains("stopped"));
    for (_, secret) in &environment {
        assert!(!service.log().contains(secret));
    }
    let socket: std::net::SocketAddr = address.strip_prefix("http://").unwrap().parse().unwrap();
    assert!(std::net::TcpStream::connect_timeout(&socket, Duration::from_millis(100)).is_err());

    // Real SCRAM password authentication through BOTH PgStore and LocalPgReader, and an
    // unprivileged service role: no CREATE/ALTER privileges or startup migrations are needed.
    sqlx::query("CREATE ROLE brain_serve_fixture LOGIN PASSWORD 'synthetic-c1-database-password'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("GRANT USAGE ON SCHEMA brain TO brain_serve_fixture")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("GRANT SELECT ON brain.core_schema_version, brain.corpus_releases_v1, brain.source_record_revisions, brain.source_record_heads, brain.source_jobs_v1, brain.source_checkpoints_v1 TO brain_serve_fixture")
        .execute(&pool).await.unwrap();
    sqlx::query("GRANT SELECT, INSERT ON brain.conversation_owners_v1 TO brain_serve_fixture")
        .execute(&pool)
        .await
        .unwrap();
    let mut password_config = config.clone();
    password_config["postgres"]["username"] = json!("brain_serve_fixture");
    password_config["postgres"]["auth"] = json!("password");
    password_config["postgres"]["password_env"] = json!("BRAIN_SERVE_PG_PASSWORD");
    let mut password_env = environment.clone();
    password_env.push(("BRAIN_SERVE_PG_PASSWORD", "synthetic-wrong-password"));
    let mut bad_password = Service::spawn(&password_config, &password_env);
    assert!(!bad_password.wait(Duration::from_secs(5)).success());
    assert!(bad_password.log().contains("database_unavailable"));
    assert!(!bad_password.log().contains("synthetic-wrong-password"));
    password_env.pop();
    password_env.push(("BRAIN_SERVE_PG_PASSWORD", common::PG_PASSWORD));
    let mut service = Service::spawn(&password_config, &password_env);
    let address = service.address();
    common::assert_ready(&address);
    assert_eq!(
        ask(&address, API_TOKEN, query("scram"))
            .await
            .unwrap()
            .status,
        AnswerStatus::Answered
    );
    assert!(!service.log().contains(common::PG_PASSWORD));
    // Removing a required privilege makes readiness fail closed without granting itself rights.
    sqlx::query("REVOKE INSERT ON brain.conversation_owners_v1 FROM brain_serve_fixture")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(common::get(&address, "/readyz").0, 503);
    let mut missing_permission = Service::spawn(&password_config, &password_env);
    assert!(!missing_permission.wait(Duration::from_secs(5)).success());
    assert!(missing_permission
        .log()
        .contains("database_permissions_missing"));
    service.signal("INT");
    assert!(service.wait(Duration::from_secs(5)).success());
    provider_stop.send(()).unwrap();
    provider_task.await.unwrap();
    admin.close().await;
    pool.close().await;
}
