use axum::{
    body::Body,
    http::{header, Request},
    Router,
};
use brain_api::{internal::InternalApiService, ApiService};
use brain_contracts::{
    AnswerProfile, AnswerResponse, AnswerStatus, AuthorizedContext, Budget, Evidence, PortError,
    Query, RetrievalPort, Usage, CONTRACT_VERSION,
};
use brain_kernel::AnswerKernelPort;
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    io::Write,
    sync::{Arc, Mutex},
};
use tower::ServiceExt;
use tracing::instrument::WithSubscriber;

const SECRET: &str = "synthetic-private-credential";
const RAW_ID: &str = "private-client-id";
const RAW_TEXT: &str = "synthetic-sensitive-question";

struct Kernel;
impl AnswerKernelPort for Kernel {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        AnswerResponse {
            contract_version: CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: AnswerStatus::InsufficientEvidence,
            text: "synthetic-sensitive-answer".into(),
            citations: Vec::new(),
            usage: Usage::default(),
        }
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer(query, context)
    }
}
struct Empty;
impl RetrievalPort for Empty {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        Ok(Vec::new())
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

fn policy(internal: bool) -> PolicyEngine {
    let (actor, channel, scope) = if internal {
        ("second-brain", "internal", "second_brain.internal")
    } else {
        ("docs-client", "docs", "docs.public")
    };
    PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
        SECRET,
        actor,
        channel,
        BTreeSet::from([scope.into()]),
        BTreeSet::new(),
    )]))
}
fn router(internal: bool) -> Router {
    if internal {
        brain_api::internal::router(InternalApiService::new(
            policy(true),
            Empty,
            "fixture-release".into(),
            1000,
            Budget::default(),
        ))
    } else {
        brain_api::router(
            ApiService::new(
                policy(false),
                Kernel,
                "fixture-release",
                1000,
                Budget::default(),
            )
            .with_retrieval(Empty),
        )
    }
}
fn body(internal: bool) -> Vec<u8> {
    serde_json::to_vec(&Query {
        request_id: RAW_ID.into(),
        conversation_id: "synthetic-private-conversation".into(),
        text: RAW_TEXT.into(),
        requested_scopes: BTreeSet::from([if internal {
            "second_brain.internal"
        } else {
            "docs.public"
        }
        .into()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
        domain: None,
    })
    .unwrap()
}

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);
impl Write for Buffer {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buffer {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
async fn call(router: Router, request: Request<Body>) -> (u16, Vec<serde_json::Value>, String) {
    let output = Buffer::default();
    let subscriber = tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_writer(output.clone())
        .finish();
    let response = router
        .oneshot(request)
        .with_subscriber(subscriber)
        .await
        .unwrap();
    let text = String::from_utf8(output.0.lock().unwrap().clone()).unwrap();
    let events = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    (response.status().as_u16(), events, text)
}
fn request(route: &str, body: Vec<u8>, auth: Option<&str>, content_type: &str) -> Request<Body> {
    let mut request = Request::builder()
        .method("POST")
        .uri(route)
        .header(header::CONTENT_TYPE, content_type);
    if let Some(auth) = auth {
        request = request.header(header::AUTHORIZATION, auth);
    }
    request.body(Body::from(body)).unwrap()
}
fn check_event(event: &serde_json::Value, route: &str, consumer: &str, status: u16) {
    assert_eq!(event.as_object().unwrap().len(), 7);
    assert_eq!(event["event"], "authenticated_request");
    assert_eq!(event["route"], route);
    assert_eq!(event["consumer_id"], consumer);
    assert_eq!(event["status"], status);
    assert!(event["duration_ms"].is_u64());
    let id = event["request_id"].as_str().unwrap();
    let digest = id
        .strip_prefix("client-sha256:")
        .or_else(|| id.strip_prefix("server-sha256:"))
        .unwrap();
    assert_eq!(digest.len(), 64);
    assert!(digest
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
}
fn check_redaction(text: &str) {
    for raw in [
        SECRET,
        RAW_ID,
        RAW_TEXT,
        "synthetic-private-conversation",
        "synthetic-sensitive-answer",
        "forged-consumer",
        "sensitive-url",
    ] {
        assert!(!text.contains(raw));
    }
}

#[tokio::test]
async fn public_and_private_success_and_business_status_are_audited_once() {
    for (route, internal, consumer) in [
        ("/v1/answer", false, "docs-client"),
        ("/v1/retrieve", false, "docs-client"),
        ("/v1/operator/query", true, "second-brain"),
    ] {
        let (status, events, text) = call(
            router(internal),
            request(
                &format!("{route}?sensitive-url=1"),
                body(internal),
                Some(&format!("Bearer {SECRET}")),
                "application/json",
            ),
        )
        .await;
        assert_eq!(status, 200);
        assert_eq!(events.len(), 1);
        check_event(&events[0], route, consumer, status);
        assert_eq!(events[0]["outcome"], "insufficient_evidence");
        assert_eq!(
            events[0]["request_id"],
            format!("client-sha256:{:x}", Sha256::digest(RAW_ID.as_bytes()))
        );
        check_redaction(&text);
    }
}

#[tokio::test]
async fn parser_media_size_and_scope_errors_are_audited_after_authentication() {
    for (route, internal, consumer) in [
        ("/v1/answer", false, "docs-client"),
        ("/v1/retrieve", false, "docs-client"),
        ("/v1/operator/query", true, "second-brain"),
    ] {
        let mut forbidden: serde_json::Value = serde_json::from_slice(&body(internal)).unwrap();
        forbidden["requested_scopes"] = serde_json::json!(["forged-consumer"]);
        let mut injected = forbidden.clone();
        injected["request_id"] = serde_json::json!(format!("{RAW_ID}\n{SECRET}"));
        injected["consumer_id"] = serde_json::json!("forged-consumer");
        for (payload, media, expected) in [
            (b"{".to_vec(), "application/json", 400),
            (injected.to_string().into_bytes(), "application/json", 400),
            (
                forbidden.to_string().into_bytes(),
                "application/json",
                if internal { 400 } else { 403 },
            ),
            (body(internal), "text/plain", 415),
            (vec![b'x'; 65537], "application/json", 413),
        ] {
            let (status, events, text) = call(
                router(internal),
                request(route, payload, Some(&format!("Bearer {SECRET}")), media),
            )
            .await;
            assert_eq!(status, expected);
            assert_eq!(events.len(), 1);
            check_event(&events[0], route, consumer, status);
            check_redaction(&text);
        }
    }
}

#[tokio::test]
async fn invalid_missing_and_duplicate_credentials_never_create_consumer_events() {
    for (route, internal) in [
        ("/v1/answer", false),
        ("/v1/retrieve", false),
        ("/v1/operator/query", true),
    ] {
        for auth in [
            None,
            Some("Bearer invalid-fixture"),
            Some("Basic invalid-fixture"),
        ] {
            let (status, events, _) = call(
                router(internal),
                request(route, body(internal), auth, "application/json"),
            )
            .await;
            assert_eq!(status, if internal { 403 } else { 401 });
            assert!(events.is_empty());
        }
        let mut req = request(
            route,
            body(internal),
            Some(&format!("Bearer {SECRET}")),
            "application/json",
        );
        req.headers_mut().append(
            header::AUTHORIZATION,
            format!("Bearer {SECRET}").parse().unwrap(),
        );
        let (_, events, _) = call(router(internal), req).await;
        assert!(events.is_empty());
    }
}

#[tokio::test]
async fn authenticated_identity_denied_on_other_transport_is_still_audited() {
    let public = brain_api::router(ApiService::new(
        policy(true),
        Kernel,
        "fixture-release",
        1000,
        Budget::default(),
    ));
    let (status, events, _) = call(
        public,
        request(
            "/v1/answer",
            body(true),
            Some(&format!("Bearer {SECRET}")),
            "application/json",
        ),
    )
    .await;
    assert_eq!(status, 401);
    assert_eq!(events.len(), 1);
    check_event(&events[0], "/v1/answer", "second-brain", 401);
    let private = brain_api::internal::router(InternalApiService::new(
        policy(false),
        Empty,
        "fixture-release".into(),
        1000,
        Budget::default(),
    ));
    let (status, events, _) = call(
        private,
        request(
            "/v1/operator/query",
            body(false),
            Some(&format!("Bearer {SECRET}")),
            "application/json",
        ),
    )
    .await;
    assert_eq!(status, 403);
    assert_eq!(events.len(), 1);
    check_event(&events[0], "/v1/operator/query", "docs-client", 403);
}

#[tokio::test]
async fn arbitrary_grant_account_data_is_hashed_and_body_identity_is_ignored() {
    let actor = "private-account-fixture@example.invalid";
    let channel = "private-channel-fixture";
    let policy = PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
        SECRET,
        actor,
        channel,
        BTreeSet::from(["docs.public".into()]),
        BTreeSet::new(),
    )]));
    let api = brain_api::router(ApiService::new(
        policy,
        Kernel,
        "fixture-release",
        1000,
        Budget::default(),
    ));
    let mut payload: serde_json::Value = serde_json::from_slice(&body(false)).unwrap();
    payload["consumer_id"] = serde_json::json!("forged-consumer");
    let (status, events, text) = call(
        api,
        request(
            "/v1/answer",
            payload.to_string().into_bytes(),
            Some(&format!("Bearer {SECRET}")),
            "application/json",
        ),
    )
    .await;
    assert_eq!(status, 400);
    assert_eq!(events.len(), 1);
    let mut digest = Sha256::new();
    digest.update((actor.len() as u64).to_be_bytes());
    digest.update(actor.as_bytes());
    digest.update(channel.as_bytes());
    check_event(
        &events[0],
        "/v1/answer",
        &format!("grant-sha256:{:x}", digest.finalize()),
        400,
    );
    assert!(!text.contains(actor));
    assert!(!text.contains(channel));
    check_redaction(&text);
}

struct SlowKernel;
impl AnswerKernelPort for SlowKernel {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        std::thread::sleep(std::time::Duration::from_millis(30));
        Kernel.answer(query, context)
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer(query, context)
    }
}

#[tokio::test]
async fn deadline_emits_one_controlled_event_without_waiting_for_worker() {
    let api = brain_api::router(ApiService::new(
        policy(false),
        SlowKernel,
        "fixture-release",
        1,
        Budget::default(),
    ));
    let (status, events, text) = call(
        api,
        request(
            "/v1/answer",
            body(false),
            Some(&format!("Bearer {SECRET}")),
            "application/json",
        ),
    )
    .await;
    assert_eq!(status, 504);
    assert_eq!(events.len(), 1);
    check_event(&events[0], "/v1/answer", "docs-client", 504);
    assert_eq!(events[0]["outcome"], "deadline_exceeded");
    check_redaction(&text);
}
