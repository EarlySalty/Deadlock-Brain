use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderMap, StatusCode},
    routing::post,
    Router,
};
use brain_client::BrainClient;
use brain_contracts::{
    AnswerProfile, AnswerStatus, Budget, DocumentStorePort, Query, SourceVisibility,
};
use brain_ingestion::FileConnector;
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_providers::{OpenAiCompatibleProvider, ProviderConfig};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
async fn provider(
    State(calls): State<Arc<AtomicUsize>>,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    assert_eq!(
        headers.get(header::AUTHORIZATION).unwrap(),
        "Bearer fixture-provider-token"
    );
    calls.fetch_add(1, Ordering::SeqCst);
    let request: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let input: serde_json::Value =
        serde_json::from_str(request["messages"][1]["content"].as_str().unwrap()).unwrap();
    let id = input["evidence"][0]["id"].as_str().unwrap();
    let answer =
        serde_json::json!({"text":"Abrams: belegte Fixture-Antwort.","cited_evidence_ids":[id]})
            .to_string();
    (StatusCode::OK,[(header::CONTENT_TYPE,"application/json")],serde_json::json!({"model":"fixture-model","choices":[{"message":{"content":answer}}],"usage":{"prompt_tokens":16,"completion_tokens":10}}).to_string())
}
fn query(id: &str) -> Query {
    Query {
        request_id: id.into(),
        conversation_id: "http-conversation".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::from(["docs.public".into()]),
        profile: AnswerProfile::Explain,
        patch: Some("p1".into()),
        mode: None,
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn file_to_store_retrieval_provider_http_api_typed_client_and_acl_revocation() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("abrams.md"), "Abrams fixture knowledge.").unwrap();
    let connector = FileConnector::new(
        root.path(),
        "fixture-files",
        SourceVisibility::Public,
        BTreeSet::new(),
    );
    let store = MemoryRepository::default();
    let lease = store
        .claim("fixture-files", "http-fixture", 30000)
        .await
        .unwrap();
    let batch = connector.prepare_batch(None).unwrap();
    connector
        .commit_batch(&store, &batch, &lease)
        .await
        .unwrap();
    let release = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let provider_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let provider_address = provider_listener.local_addr().unwrap();
    let provider_router = Router::new()
        .route("/chat/completions", post(provider))
        .with_state(calls.clone());
    let (provider_stop, provider_stopped) = tokio::sync::oneshot::channel::<()>();
    let provider_task = tokio::spawn(async move {
        axum::serve(provider_listener, provider_router)
            .with_graceful_shutdown(async {
                let _ = provider_stopped.await;
            })
            .await
            .unwrap();
    });
    // Blocking reqwest clients are created off the async reactor.
    let transport = tokio::task::spawn_blocking(move || {
        OpenAiCompatibleProvider::new(ProviderConfig::new(
            "fixture-provider-token",
            format!("http://{provider_address}"),
            "fixture-model",
        ))
        .unwrap()
    })
    .await
    .unwrap();
    let kernel = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(store.clone(), 10), transport),
        8,
        Duration::from_secs(30),
    );
    let credentials = CredentialRegistry::new(vec![
        AuthGrant::from_secret(
            "fixture-brain-token",
            "actor-a",
            "test",
            BTreeSet::from(["docs.public".into()]),
            BTreeSet::from(["public".into()]),
        ),
        AuthGrant::from_secret(
            "fixture-other-token",
            "actor-b",
            "test",
            BTreeSet::from(["docs.public".into()]),
            BTreeSet::from(["public".into()]),
        ),
    ]);
    let service = brain_api::ApiService::new(
        PolicyEngine::new(credentials),
        kernel,
        "r1",
        5000,
        Budget::default(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let api_task = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(service))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let answer = tokio::task::spawn_blocking(move || {
        let client = BrainClient::new(
            &format!("http://{address}"),
            "fixture-brain-token",
            Duration::from_secs(10),
        )
        .unwrap();
        let first = client.answer(&query("q1")).unwrap();
        first.validate("q1").unwrap();
        assert_eq!(first.status, AnswerStatus::Answered);
        let second = client.answer(&query("q2")).unwrap();
        assert_eq!(second.text, first.text);
        assert_eq!(second.request_id, "q2");
        let encoded = serde_json::to_string(&second).unwrap();
        for forbidden in [
            "fixture-provider-token",
            "fixture-files",
            "abrams.md",
            "allowed_scopes",
            "input_tokens",
            "fixture-model",
        ] {
            assert!(!encoded.contains(forbidden));
        }
        let other = BrainClient::new(
            &format!("http://{address}"),
            "fixture-other-token",
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(other.answer(&query("q3")).is_err());
        let invalid = BrainClient::new(
            &format!("http://{address}"),
            "invalid-token",
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(invalid.answer(&query("q4")).is_err());
        first
    })
    .await
    .unwrap();
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let mut revoked = batch.records[0].clone();
    revoked.revision += 1;
    revoked.visibility = SourceVisibility::Private;
    revoked.allowed_scopes.insert("private".into());
    store.apply_record(revoked).unwrap();
    let revoked_answer = tokio::task::spawn_blocking(move || {
        BrainClient::new(
            &format!("http://{address}"),
            "fixture-brain-token",
            Duration::from_secs(10),
        )
        .unwrap()
        .answer(&query("q5"))
        .unwrap()
    })
    .await
    .unwrap();
    assert_ne!(revoked_answer.status, AnswerStatus::Answered);
    assert!(revoked_answer.citations.is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    stop.send(()).unwrap();
    api_task.await.unwrap();
    provider_stop.send(()).unwrap();
    provider_task.await.unwrap();
}
