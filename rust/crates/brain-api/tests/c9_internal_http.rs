//! Getrennte öffentliche und interne Transportverträge mit synthetischen Daten.
use brain_api::{internal::InternalApiService, ApiService};
use brain_client::{AsyncInternalBrainClient, ClientError, InternalStatus};
use brain_contracts::{
    AnswerProfile, AnswerResponse, AuthorizedContext, Budget, Evidence, EvidenceKind, PortError,
    Query, RetrievalPort, SourceVisibility,
};
use brain_kernel::AnswerKernelPort;
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use std::{
    collections::BTreeSet,
    os::unix::fs::{MetadataExt, PermissionsExt},
    time::Duration,
};

struct LocalFixture;
impl RetrievalPort for LocalFixture {
    fn retrieve(&self, _: &Query, context: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        assert_eq!(context.knowledge_release, "fixture-internal-release");
        Ok(vec![Evidence {
            evidence_id: "fixture-e".into(),
            source_id: "fixture-s".into(),
            logical_id: "fixture-l".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "Synthetischer Operatorausschnitt".into(),
            citation: "fixture-source".into(),
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::from(["second_brain.internal".into()]),
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
        for_provider: bool,
    ) -> Result<(), PortError> {
        assert!(!for_provider);
        Ok(())
    }
}
struct NeverCalled;
impl AnswerKernelPort for NeverCalled {
    fn answer(&self, _: &Query, _: &AuthorizedContext) -> AnswerResponse {
        panic!("Unerwarteter interner Modellaufruf")
    }
    fn answer_for_publication(&self, _: &Query, _: &AuthorizedContext) -> AnswerResponse {
        panic!("Unerwarteter öffentlicher Modellaufruf")
    }
}
fn budget() -> Budget {
    Budget {
        max_network_rounds: 1,
        max_input_tokens: 1000,
        max_output_tokens: 1000,
        max_cost_micros: 1,
    }
}
fn query() -> Query {
    Query {
        answer_context: None,
        request_id: "fixture-request".into(),
        conversation_id: "fixture-conversation".into(),
        text: "Operatorausschnitt".into(),
        domain: None,
        requested_scopes: BTreeSet::from(["second_brain.internal".into()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}

#[tokio::test]
async fn private_antwort_docs_abweisung_und_oeffentliche_nichterreichbarkeit() {
    let root = tempfile::tempdir().unwrap();
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let path = root.path().join("operator.sock");
    let listener = tokio::net::UnixListener::bind(&path).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
        "internal-fixture-token",
        "second-brain",
        "internal",
        BTreeSet::from(["second_brain.internal".into()]),
        BTreeSet::new(),
    )]);
    let router = brain_api::internal::router(InternalApiService::new(
        PolicyEngine::new(registry),
        LocalFixture,
        "fixture-internal-release".into(),
        1000,
        budget(),
    ));
    let internal = tokio::spawn(async move { axum::serve(listener, router).await });
    let owner = std::fs::metadata(&path).unwrap().uid();
    let client = AsyncInternalBrainClient::new(
        &path,
        owner,
        "internal-fixture-token",
        Duration::from_secs(2),
    )
    .unwrap();
    let answer = client.query(&query()).await.unwrap();
    assert_eq!(answer.status, InternalStatus::Answered);
    assert_eq!(answer.excerpts.len(), 1);
    let docs =
        AsyncInternalBrainClient::new(&path, owner, "docs-fixture-token", Duration::from_secs(2))
            .unwrap();
    assert!(
        matches!(docs.query(&query()).await, Err(ClientError::HttpStatus { status, .. }) if status == reqwest::StatusCode::FORBIDDEN)
    );
    assert!(AsyncInternalBrainClient::new(
        &path,
        owner.wrapping_add(1),
        "internal-fixture-token",
        Duration::from_secs(2)
    )
    .is_err());

    let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
        "docs-fixture-token",
        "docs-client",
        "docs",
        BTreeSet::from(["docs.public".into()]),
        BTreeSet::from(["public".into()]),
    )]);
    let public_router = brain_api::router(ApiService::new(
        PolicyEngine::new(registry),
        NeverCalled,
        "fixture-docs-release",
        1000,
        budget(),
    ));
    let tcp = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", tcp.local_addr().unwrap());
    let public = tokio::spawn(async move { axum::serve(tcp, public_router).await });
    let http = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    for route in ["/v1/answer", "/v1/retrieve"] {
        let response = http
            .post(format!("{endpoint}{route}"))
            .bearer_auth("internal-fixture-token")
            .json(&query())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
        let response = http
            .post(format!("{endpoint}{route}"))
            .bearer_auth("docs-fixture-token")
            .json(&query())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
    }
    for token in ["docs-fixture-token", "internal-fixture-token"] {
        let response = http
            .post(format!("{endpoint}/v1/operator/query"))
            .bearer_auth(token)
            .json(&query())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    }
    public.abort();
    internal.abort();
    let _ = public.await;
    let _ = internal.await;
}
