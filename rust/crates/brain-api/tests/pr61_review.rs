//! Reviewer regression probes for PR #61, with synthetic credentials and loopback only.
use brain_contracts::store::ConversationOwnershipPort;
use brain_contracts::{
    AnswerProfile, AnswerResponse, AnswerStatus, AuthorizedContext, Budget, PortError, Query,
    Usage, CONTRACT_VERSION,
};
use brain_kernel::AnswerKernelPort;
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use std::{
    collections::BTreeSet,
    sync::{mpsc, Arc, Condvar, Mutex},
    time::Duration,
};

struct BlockedOwnership {
    gate: Arc<(Mutex<bool>, Condvar)>,
}
impl ConversationOwnershipPort for BlockedOwnership {
    fn claim_conversation(&self, _: &str, _: &str) -> Result<(), PortError> {
        let (lock, ready) = &*self.gate;
        let open = lock.lock().unwrap();
        let (open, _) = ready
            .wait_timeout_while(open, Duration::from_secs(5), |open| !*open)
            .unwrap();
        if !*open {
            return Err(PortError::Unavailable("review watchdog".into()));
        }
        Ok(())
    }
}
struct RecordingKernel(mpsc::Sender<()>);
impl AnswerKernelPort for RecordingKernel {
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer(query, context)
    }
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.0.send(()).unwrap();
        AnswerResponse {
            contract_version: CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: AnswerStatus::Unavailable,
            text: "Synthetic review response".into(),
            citations: vec![],
            usage: Usage::default(),
        }
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn kernel_must_not_start_after_http_deadline_during_ownership_check() {
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let (calls, observed) = mpsc::channel();
    let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
        "review-token",
        "review-actor",
        "review",
        BTreeSet::new(),
        BTreeSet::new(),
    )]);
    let service = brain_api::ApiService::new(
        PolicyEngine::with_ownership_store(
            registry,
            Arc::new(BlockedOwnership { gate: gate.clone() }),
        ),
        RecordingKernel(calls),
        "review-release",
        50,
        Budget::default(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(service))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = brain_client::AsyncBrainClient::new(
        &format!("http://{address}"),
        "review-token",
        Duration::from_secs(2),
    )
    .unwrap();
    let query = Query {
        request_id: "review-1".into(),
        conversation_id: "review-conversation".into(),
        text: "Review fixture".into(),
        profile: AnswerProfile::Explain,
        requested_scopes: BTreeSet::new(),
        patch: None,
        mode: None,
        domain: None,
    };
    let result = client.answer(&query).await;
    // Only release the ownership query after the HTTP timeout has definitely been delivered.
    *gate.0.lock().unwrap() = true;
    gate.1.notify_all();
    let late_call =
        tokio::task::spawn_blocking(move || observed.recv_timeout(Duration::from_secs(1)))
            .await
            .unwrap();
    stop.send(()).unwrap();
    server.await.unwrap();
    assert!(
        matches!(result, Err(brain_client::ClientError::HttpStatus { status, .. }) if status.as_u16() == 504)
    );
    assert!(
        late_call.is_err(),
        "the kernel starts with a fresh full budget AFTER the HTTP request has returned 504"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unauthenticated_incomplete_body_is_rejected_before_body_read() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let (calls, _observed) = mpsc::channel();
    let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
        "review-token",
        "review-actor",
        "review",
        BTreeSet::new(),
        BTreeSet::new(),
    )]);
    let service = brain_api::ApiService::new(
        PolicyEngine::new(registry),
        RecordingKernel(calls),
        "review-release",
        50,
        Budget::default(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(service))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
    // The client intentionally withholds two body bytes; one local connection only.
    stream.write_all(b"POST /v1/answer HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n").await.unwrap();
    let mut bytes = Vec::new();
    let count = tokio::time::timeout(Duration::from_millis(500), stream.read_to_end(&mut bytes))
        .await
        .expect("authentication must reject the unfinished body within the test bound")
        .expect("read the HTTP response");
    drop(stream);
    stop.send(()).unwrap();
    server.await.unwrap();
    assert!(count > 0);
    let response = std::str::from_utf8(&bytes).unwrap();
    assert!(response.starts_with("HTTP/1.1 401 "), "{response}");
    let (_, body) = response.split_once("\r\n\r\n").unwrap();
    let error: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(error["error"]["code"], "unauthorized");
    assert_eq!(
        error["error"]["message"],
        "Genau ein Bearer-Header erforderlich"
    );
}
