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

#[derive(Debug)]
struct LoadResult {
    requests: usize,
    workers: usize,
    statuses: BTreeMap<String, usize>,
    client_errors: usize,
    elapsed: Duration,
}

fn run_load(address: &str, workers: usize, requests: usize, label: &str) -> LoadResult {
    let started = Instant::now();
    let handles: Vec<_> = (0..workers)
        .map(|worker| {
            let address = address.to_string();
            let label = label.to_string();
            std::thread::spawn(move || {
                let client = BrainClient::new(&address, API_TOKEN, Duration::from_secs(8)).unwrap();
                let mut statuses = BTreeMap::new();
                let mut errors = 0usize;
                for index in (worker..requests).step_by(workers) {
                    let id = format!("{label}-{index}");
                    match client.answer(&query(&id)) {
                        Ok(answer) => {
                            let status = serde_json::to_value(answer.status)
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .to_string();
                            *statuses.entry(status).or_insert(0usize) += 1;
                        }
                        Err(_) => errors += 1,
                    }
                }
                (statuses, errors)
            })
        })
        .collect();
    let mut statuses = BTreeMap::new();
    let mut client_errors = 0usize;
    for handle in handles {
        let (worker_statuses, errors) = handle.join().unwrap();
        client_errors += errors;
        for (status, count) in worker_statuses {
            *statuses.entry(status).or_insert(0usize) += count;
        }
    }
    LoadResult {
        requests,
        workers,
        statuses,
        client_errors,
        elapsed: started.elapsed(),
    }
}

fn assert_load(result: &LoadResult) {
    assert_eq!(
        result.client_errors, 0,
        "{} workers produced transport errors: {result:?}",
        result.workers
    );
    assert_eq!(
        result.statuses.values().sum::<usize>(),
        result.requests,
        "every request must produce a typed answer: {result:?}"
    );
    assert_eq!(
        result
            .statuses
            .get("unauthorized_evidence")
            .copied()
            .unwrap_or(0),
        0,
        "technical pool pressure must never be misclassified: {result:?}"
    );
    assert!(
        result
            .statuses
            .keys()
            .all(|status| status == "answered" || status == "unavailable"),
        "load produced an unexpected status: {result:?}"
    );
    assert!(
        result.elapsed < Duration::from_secs(60),
        "bounded load exceeded test envelope: {result:?}"
    );
}

fn pool_stats(log: &str) -> Value {
    log.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find(|value| value["event"] == "postgres_pool_stats")
        .expect("postgres pool stats event")
}

async fn reader_connection_count(pool: &sqlx::PgPool) -> i64 {
    sqlx::query_scalar(
        "SELECT count(*) FROM pg_stat_activity WHERE datname='brain_serve_test' AND application_name='deadlock-brain-reader'",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn reader_lock_waiters(pool: &sqlx::PgPool) -> i64 {
    sqlx::query_scalar(
        "SELECT count(*) FROM pg_stat_activity WHERE datname='brain_serve_test' AND application_name='deadlock-brain-reader' AND wait_event_type='Lock'",
    )
    .fetch_one(pool)
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
    config["postgres"]["max_connections"] = json!(4);
    config["provider"]["base_url"] = json!(format!("http://{provider_address}"));
    config["kernel"]["cache_entries"] = json!(0);
    config["timeouts"]["postgres_connect_ms"] = json!(500);
    config["timeouts"]["postgres_pool_wait_ms"] = json!(150);
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

    // Required bounded-load envelope. The server runs with max_connections=12 while brain-serve
    // is hard-capped at four shared connections; all request-side DB work uses that same pool.
    for workers in [8usize, 16, 32] {
        let load_address = address.clone();
        let result = tokio::task::spawn_blocking(move || {
            run_load(&load_address, workers, 600, &format!("load-{workers}"))
        })
        .await
        .unwrap();
        assert_load(&result);
        let observed_connections = reader_connection_count(&pool).await;
        assert!(
            observed_connections <= 4,
            "brain-serve exceeded its configured hard pool cap"
        );
        eprintln!(
            "{}",
            json!({
                "event": "db_pool_load",
                "requests": result.requests,
                "workers": result.workers,
                "statuses": result.statuses,
                "client_errors": result.client_errors,
                "elapsed_ms": result.elapsed.as_millis(),
                "observed_reader_connections": observed_connections
            })
        );
    }

    // Saturate every pool slot behind a real table lock. The next request must wait only for the
    // configured pool budget, return typed Unavailable, and recover after the lock is released.
    let mut lock_tx = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE brain.conversation_owners_v1 IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *lock_tx)
        .await
        .unwrap();
    let mut blockers = Vec::new();
    for index in 0..4 {
        let blocker_address = address.clone();
        blockers.push(tokio::spawn(async move {
            ask(
                &blocker_address,
                API_TOKEN,
                query(&format!("pool-blocker-{index}")),
            )
            .await
        }));
    }
    let wait_deadline = Instant::now() + Duration::from_secs(3);
    while reader_lock_waiters(&pool).await < 4 {
        assert!(
            Instant::now() < wait_deadline,
            "all four shared pool connections did not reach the intentional lock"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(reader_connection_count(&pool).await, 4);
    let exhausted_started = Instant::now();
    let exhausted = ask(&address, API_TOKEN, query("pool-exhausted"))
        .await
        .unwrap();
    let exhausted_elapsed = exhausted_started.elapsed();
    assert_eq!(exhausted.status, AnswerStatus::Unavailable);
    assert!(
        exhausted_elapsed >= Duration::from_millis(100)
            && exhausted_elapsed < Duration::from_secs(2),
        "pool wait was not bounded by the configured acquire budget: {exhausted_elapsed:?}"
    );
    lock_tx.rollback().await.unwrap();
    for blocker in blockers {
        let answer = blocker.await.unwrap().unwrap();
        assert_eq!(answer.status, AnswerStatus::Answered);
    }
    assert_eq!(
        ask(&address, API_TOKEN, query("pool-recovered"))
            .await
            .unwrap()
            .status,
        AnswerStatus::Answered
    );
    let count = provider.calls.load(Ordering::SeqCst);

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
    assert_eq!(
        ask(&address, API_TOKEN, query("db-down"))
            .await
            .unwrap()
            .status,
        AnswerStatus::Unavailable
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), count);
    sqlx::query("ALTER DATABASE brain_serve_test ALLOW_CONNECTIONS true")
        .execute(&admin)
        .await
        .unwrap();
    common::assert_ready(&address);
    service.stop();
    let stats = pool_stats(&service.log());
    assert_eq!(stats["max_connections"], 4);
    assert_eq!(stats["peak_connections"], 4);
    assert!(
        stats["created_connections"].as_u64().unwrap() < 32,
        "connections must be reused rather than created per operation: {stats}"
    );
    assert!(
        stats["reused_checkouts"].as_u64().unwrap() > 1_000,
        "load must exercise connection reuse: {stats}"
    );
    assert!(stats["wait_count"].as_u64().unwrap() > 0, "{stats}");
    assert!(stats["wait_timeout_count"].as_u64().unwrap() > 0, "{stats}");
    assert!(
        stats["wait_max_micros"].as_u64().unwrap() >= 100_000,
        "bounded pool wait was not observed: {stats}"
    );
    eprintln!("{}", json!({"event": "db_pool_metrics", "stats": stats}));

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

    // Real SCRAM password authentication through the single runtime LocalPgReader pool, and an
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
    assert!(
        bad_password.log().contains("database_unavailable"),
        "{}",
        bad_password.log()
    );
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
