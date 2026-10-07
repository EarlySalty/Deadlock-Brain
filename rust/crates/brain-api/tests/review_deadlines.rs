//! Actual HTTP/1 TCP regressions. Gates establish worker state before deadlines/disconnects.
use brain_contracts::{
    store::ConversationOwnershipPort, AnswerProfile, AnswerProviderPort, AuthorizedContext, Budget,
    Evidence, EvidenceKind, PortError, ProviderAnswer, Query, RetrievalPort, SourceVisibility,
    Usage,
};
use brain_kernel::{AnswerKernelPort, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use std::{
    collections::BTreeSet,
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{oneshot, Notify},
};

#[derive(Default)]
struct Gate {
    entered: Notify,
    released: Mutex<bool>,
    changed: Condvar,
    deadline: Mutex<Option<brain_contracts::RequestDeadline>>,
}
impl Gate {
    fn block(&self) {
        self.entered.notify_one();
        let (released, _) = self
            .changed
            .wait_timeout_while(
                self.released.lock().unwrap(),
                Duration::from_secs(5),
                |released| !*released,
            )
            .unwrap();
        assert!(*released, "test must release the bounded ownership gate");
    }
    fn release(&self) {
        *self.released.lock().unwrap() = true;
        self.changed.notify_all();
    }
}
struct Ownership(Arc<Gate>);
impl ConversationOwnershipPort for Ownership {
    fn claim_conversation(&self, conversation: &str, _: &str) -> Result<(), PortError> {
        if conversation == "blocked" {
            self.0.block();
        }
        Ok(())
    }
    fn claim_conversation_until(
        &self,
        conversation: &str,
        actor: &str,
        deadline: &brain_contracts::RequestDeadline,
    ) -> Result<(), PortError> {
        // Deliberately uncooperative work: the policy must check its deadline afterwards.
        *self.0.deadline.lock().unwrap() = Some(deadline.clone());
        self.claim_conversation(conversation, actor)
    }
}
struct Retrieval;
impl RetrievalPort for Retrieval {
    fn validate_publication(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
    ) -> Result<(), PortError> {
        self.validate_evidence(q, c, e, false)
    }
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        Ok(vec![Evidence {
            evidence_id: "fixture-evidence".into(),
            source_id: "fixture".into(),
            logical_id: "a".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "authorized fixture".into(),
            citation: "fixture:a".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: None,
        }])
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        Ok(())
    }
}
struct Provider(Arc<AtomicUsize>);
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderAnswer {
            text: "successful fixture".into(),
            cited_evidence_ids: vec![evidence[0].evidence_id.clone()],
            usage: Usage::default(),
        })
    }
}
struct TrackedKernel {
    inner: Kernel<Retrieval, Provider>,
    dropped: Option<oneshot::Sender<()>>,
}
impl AnswerKernelPort for TrackedKernel {
    fn answer_for_publication(
        &self,
        q: &Query,
        c: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        self.inner.answer_for_publication(q, c)
    }
    fn answer(&self, q: &Query, c: &AuthorizedContext) -> brain_contracts::AnswerResponse {
        self.inner.answer(q, c)
    }
}
impl Drop for TrackedKernel {
    fn drop(&mut self) {
        if let Some(tx) = self.dropped.take() {
            let _ = tx.send(());
        }
    }
}
struct Server {
    address: SocketAddr,
    stop: Option<oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<()>>,
    workers_done: oneshot::Receiver<()>,
    calls: Arc<AtomicUsize>,
}
impl Server {
    async fn new(budget_ms: u64, ownership: Option<Arc<Gate>>) -> Self {
        let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture-token",
            "actor",
            "test",
            BTreeSet::new(),
            BTreeSet::from(["public".into()]),
        )]);
        let policy = match ownership {
            Some(gate) => PolicyEngine::with_ownership_store(registry, Arc::new(Ownership(gate))),
            None => PolicyEngine::new(registry),
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let (dropped, workers_done) = oneshot::channel();
        let kernel = TrackedKernel {
            inner: Kernel::new(Retrieval, Provider(calls.clone())),
            dropped: Some(dropped),
        };
        let service =
            brain_api::ApiService::new(policy, kernel, "r1", budget_ms, Budget::default());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (stop, stopped) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, brain_api::router(service))
                .with_graceful_shutdown(async {
                    let _ = stopped.await;
                })
                .await
                .unwrap();
        });
        Self {
            address,
            stop: Some(stop),
            task: Some(task),
            workers_done,
            calls,
        }
    }
    async fn shutdown_transport(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(task) = self.task.take() {
            tokio::time::timeout(Duration::from_secs(2), task)
                .await
                .expect("HTTP transport must stop")
                .unwrap();
        }
    }
    async fn workers_finished(&mut self) {
        tokio::time::timeout(Duration::from_secs(2), &mut self.workers_done)
            .await
            .expect("all blocking worker ownership must end")
            .unwrap();
    }
    async fn finish(mut self) {
        self.shutdown_transport().await;
        self.workers_finished().await;
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}
fn body(conversation: &str) -> String {
    serde_json::to_string(&Query {
        request_id: "deadline-test".into(),
        conversation_id: conversation.into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
        domain: None,
    })
    .unwrap()
}
async fn request(address: SocketAddr, conversation: &str) -> TcpStream {
    let body = body(conversation);
    let mut stream = TcpStream::connect(address).await.unwrap();
    stream.write_all(format!("POST /v1/answer HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer fixture-token\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
    stream
}
async fn partial(address: SocketAddr, auth: &str, size: usize, expect_continue: bool) -> TcpStream {
    let mut stream = TcpStream::connect(address).await.unwrap();
    let expect = if expect_continue {
        "Expect: 100-continue\r\n"
    } else {
        ""
    };
    stream.write_all(format!("POST /v1/answer HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {auth}\r\nContent-Type: application/json\r\nContent-Length: {size}\r\n{expect}Connection: close\r\n\r\n").as_bytes()).await.unwrap();
    if expect_continue {
        let mut headers = Vec::new();
        tokio::time::timeout(Duration::from_secs(2), async {
            while !headers.ends_with(b"\r\n\r\n") {
                headers.push(stream.read_u8().await.unwrap());
                assert!(headers.len() < 8192);
            }
        })
        .await
        .expect("server must begin the admitted body read");
        assert!(String::from_utf8_lossy(&headers).starts_with("HTTP/1.1 100 "));
    } else if size <= 65536 {
        stream.write_all(b"{").await.unwrap();
    }
    stream
}
async fn response(stream: &mut TcpStream, limit: Duration) -> String {
    let mut bytes = Vec::new();
    tokio::time::timeout(limit, stream.read_to_end(&mut bytes))
        .await
        .expect("HTTP response exceeded test bound")
        .unwrap();
    String::from_utf8(bytes).unwrap()
}
fn status(response: &str, expected: u16) {
    assert!(
        response.starts_with(&format!("HTTP/1.1 {expected} ")),
        "{response}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn slow_ownership_uses_original_100ms_budget_and_never_starts_provider() {
    let gate = Arc::new(Gate::default());
    let mut server = Server::new(100, Some(gate.clone())).await;
    let started = Instant::now();
    let mut stream = request(server.address, "blocked").await;
    tokio::time::timeout(Duration::from_secs(1), gate.entered.notified())
        .await
        .unwrap();
    let reply = response(&mut stream, Duration::from_millis(400)).await;
    status(&reply, 504);
    assert!(started.elapsed() < Duration::from_millis(400));
    // Hold ownership for at least 150 ms, rather than racing an arbitrary sleep against dispatch.
    tokio::time::sleep_until((started + Duration::from_millis(150)).into()).await;
    gate.release();
    server.shutdown_transport().await;
    server.workers_finished().await;
    assert_eq!(
        server.calls.load(Ordering::SeqCst),
        0,
        "even a late worker must not start provider work"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn incomplete_body_expires_without_entering_worker() {
    let server = Server::new(100, None).await;
    let mut stream = partial(server.address, "fixture-token", 1024, false).await;
    status(
        &response(&mut stream, Duration::from_millis(400)).await,
        504,
    );
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    let mut normal = request(server.address, "normal").await;
    status(&response(&mut normal, Duration::from_secs(1)).await, 200);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_auth_and_oversized_body_are_rejected_before_body_read() {
    let server = Server::new(1000, None).await;
    let mut invalid = partial(server.address, "wrong-token", 1024, false).await;
    status(
        &response(&mut invalid, Duration::from_millis(400)).await,
        401,
    );
    let mut oversized = partial(server.address, "fixture-token", 65537, false).await;
    status(
        &response(&mut oversized, Duration::from_millis(400)).await,
        413,
    );
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    let mut normal = request(server.address, "normal").await;
    status(&response(&mut normal, Duration::from_secs(1)).await, 200);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unread_keepalive_bodies_close_after_auth_failure_or_timeout() {
    let server = Server::new(100, None).await;
    for (auth, expected) in [("wrong-token", 401), ("fixture-token", 504)] {
        let mut stream = TcpStream::connect(server.address).await.unwrap();
        // HTTP/1.1 defaults to keep-alive. Do not ask the server to close: prove
        // unread-body disposal itself cannot continue outside the admission limit.
        stream.write_all(format!("POST /v1/answer HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {auth}\r\nContent-Type: application/json\r\nContent-Length: 1024\r\n\r\n{{").as_bytes()).await.unwrap();
        status(
            &response(&mut stream, Duration::from_millis(400)).await,
            expected,
        );
    }
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    let mut normal = request(server.address, "normal").await;
    status(&response(&mut normal, Duration::from_secs(1)).await, 200);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chunked_body_without_content_length_keeps_the_byte_limit() {
    let server = Server::new(1000, None).await;
    let mut stream = TcpStream::connect(server.address).await.unwrap();
    let body = "x".repeat(65537);
    let encoded = format!("POST /v1/answer HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer fixture-token\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{body}\r\n0\r\n\r\n", body.len());
    stream.write_all(encoded.as_bytes()).await.unwrap();
    status(
        &response(&mut stream, Duration::from_millis(400)).await,
        413,
    );
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    let mut normal = request(server.address, "normal").await;
    status(&response(&mut normal, Duration::from_secs(1)).await, 200);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn all_64_body_reads_consume_admission_and_disconnect_releases_permits() {
    let server = Server::new(5000, None).await;
    let mut streams = Vec::new();
    for _ in 0..64 {
        streams.push(partial(server.address, "fixture-token", 1024, true).await);
    }
    let mut overloaded = request(server.address, "normal").await;
    status(
        &response(&mut overloaded, Duration::from_secs(1)).await,
        429,
    );
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    drop(streams);
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let mut normal = request(server.address, "normal").await;
            let reply = response(&mut normal, Duration::from_secs(1)).await;
            if reply.starts_with("HTTP/1.1 200 ") {
                break;
            }
            status(&reply, 429);
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("disconnected readers must return their permits");
    assert_eq!(server.calls.load(Ordering::SeqCst), 1);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn all_admitted_body_read_timeouts_return_their_permits() {
    let server = Server::new(5000, None).await;
    let admission_started = Instant::now();
    let mut streams = Vec::new();
    // 100 Continue proves body polling began after admission, rather than relying
    // on write_all or socket scheduling to infer that 64 handlers hold permits.
    for _ in 0..64 {
        streams.push(partial(server.address, "fixture-token", 1024, true).await);
    }
    assert!(
        admission_started.elapsed() < Duration::from_secs(2),
        "fixture must admit all bodies well before the first deadline"
    );
    let mut excess = request(server.address, "normal").await;
    status(&response(&mut excess, Duration::from_secs(1)).await, 429);
    for stream in &mut streams {
        status(&response(stream, Duration::from_secs(6)).await, 504);
    }
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    let mut normal = request(server.address, "normal").await;
    status(&response(&mut normal, Duration::from_secs(1)).await, 200);
    assert_eq!(server.calls.load(Ordering::SeqCst), 1);
    server.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_service_deadlines_fail_closed_instead_of_being_clamped() {
    for budget in [0, 60001] {
        let server = Server::new(budget, None).await;
        let mut stream = request(server.address, "normal").await;
        let reply = response(&mut stream, Duration::from_millis(400)).await;
        if budget == 0 && reply.starts_with("HTTP/1.1 504 ") {
            status(&reply, 504);
        } else {
            status(&reply, 503);
        }
        assert_eq!(server.calls.load(Ordering::SeqCst), 0);
        server.finish().await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn client_abort_cancels_late_worker_before_provider() {
    let gate = Arc::new(Gate::default());
    let mut server = Server::new(5000, Some(gate.clone())).await;
    let stream = request(server.address, "blocked").await;
    tokio::time::timeout(Duration::from_secs(1), gate.entered.notified())
        .await
        .unwrap();
    let deadline = gate.deadline.lock().unwrap().clone().unwrap();
    drop(stream);
    // A transport's graceful-shutdown notification need not be ordered after every
    // handler destructor. Observe the actual request cancellation before releasing work.
    tokio::time::timeout(Duration::from_secs(2), async {
        while deadline.check().is_ok() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("disconnect must cancel the running request");
    assert!(
        Instant::now() < deadline.expires_at(),
        "cancellation, not natural expiry, must release this gate"
    );
    gate.release();
    server.shutdown_transport().await;
    server.workers_finished().await;
    assert_eq!(server.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn queued_blocking_worker_cannot_restart_expired_budget() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(1)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let gate = Arc::new(Gate::default());
        let held = gate.clone();
        let blocker = tokio::task::spawn_blocking(move || held.block());
        gate.entered.notified().await;
        let mut server = Server::new(100, None).await;
        let mut stream = request(server.address, "normal").await;
        status(
            &response(&mut stream, Duration::from_millis(400)).await,
            504,
        );
        gate.release();
        blocker.await.unwrap();
        server.shutdown_transport().await;
        server.workers_finished().await;
        assert_eq!(server.calls.load(Ordering::SeqCst), 0);
    });
}
