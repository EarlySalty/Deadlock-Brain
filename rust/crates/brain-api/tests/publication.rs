//! Publication is distinct from internal read and provider egress permission.
#[path = "support/domain_fixture.rs"]
mod domain_fixture;
use brain_api::ApiService;
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, Principal, ProviderAnswer, PublicAnswerResponse, Query, RetrievalPort,
    SourceRecordV2, SourceVisibility, Usage,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

fn record(id: &str, revision: u64, publication: bool, fact: bool) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: format!("publication-{id}"),
        logical_id: "entity/hero/Abrams".into(),
        revision,
        content_hash: "a".repeat(64),
        content: if fact {
            "hero: Abrams\nhealth: 650".into()
        } else {
            format!("Abrams private input {id}")
        },
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    if fact {
        record.metadata.insert("kind".into(), "fact".into());
    }
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: format!("fixture:{id}"),
        parser_revision: "fixture-v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("en".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: Observed::known("fixture-grant".into()),
            license: Observed::known("fixture".into()),
            publication_allowed: publication,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record
}

#[tokio::test]
async fn current_canonical_egress_revocation_rejects_pinned_provider_inputs() {
    let store = MemoryRepository::default();
    let original = record("a", 1, true, false);
    store.apply_record(original.clone()).unwrap();
    publish(&store).await;
    let retrieval = ReleaseRetriever::new(store.clone(), 10);
    let q = query(AnswerProfile::Explain);
    let hits = retrieval.retrieve(&q, &context()).unwrap();
    assert_eq!(hits.len(), 1);
    retrieval
        .validate_evidence(&q, &context(), &hits, true)
        .unwrap();
    let mut revoked = original;
    revoked.revision = 2;
    let mut origin = brain_contracts::source::origin_from_record(&revoked).unwrap();
    origin.policy.provider_egress_allowed = false;
    origin.bind_record(&mut revoked).unwrap();
    store.apply_record(revoked).unwrap();
    assert!(retrieval
        .validate_evidence(&q, &context(), &hits, true)
        .is_err());
    retrieval
        .validate_evidence(&q, &context(), &hits, false)
        .unwrap();
}

#[tokio::test]
async fn provider_requires_current_origin_for_canonical_inputs_but_preserves_legacy_inputs() {
    use brain_contracts::source::ORIGIN_METADATA_KEY;
    for canonical in [false, true] {
        let store = MemoryRepository::default();
        let mut original = record("a", 1, true, false);
        if !canonical {
            original.metadata.remove(ORIGIN_METADATA_KEY);
        }
        store.apply_record(original.clone()).unwrap();
        publish(&store).await;
        let retrieval = ReleaseRetriever::new(store.clone(), 10);
        let q = query(AnswerProfile::Explain);
        let hits = retrieval.retrieve(&q, &context()).unwrap();
        assert_eq!(hits.len(), 1);
        retrieval
            .validate_evidence(&q, &context(), &hits, true)
            .unwrap();
        let mut head = original;
        head.revision = 2;
        head.metadata.remove(ORIGIN_METADATA_KEY);
        store.apply_record(head).unwrap();
        let provider = retrieval.validate_evidence(&q, &context(), &hits, true);
        assert_eq!(
            provider.is_err(),
            canonical,
            "wrong provider result for canonical={canonical}"
        );
        retrieval
            .validate_evidence(&q, &context(), &hits, false)
            .unwrap();
    }
}

#[test]
fn current_origin_egress_checks_canonical_identity_acl_and_version() {
    use brain_contracts::{source::ORIGIN_METADATA_KEY, DocumentHead};
    let record = record("a", 1, true, false);
    let principal = context().principal;
    let original = DocumentHead::from(&record);
    assert!(original.allowed(&principal, true));
    for (pointer, value) in [
        ("/contract_version", serde_json::json!("brain.ir.v99")),
        ("/data/identity/source_id", serde_json::json!("other")),
        ("/data/identity/logical_id", serde_json::json!("other")),
        ("/data/policy/visibility", serde_json::json!("private")),
        ("/data/policy/allowed_scopes", serde_json::json!(["other"])),
        (
            "/data/policy/provider_egress_allowed",
            serde_json::json!(false),
        ),
    ] {
        let mut head = original.clone();
        let mut origin: serde_json::Value =
            serde_json::from_str(&head.metadata[ORIGIN_METADATA_KEY]).unwrap();
        *origin.pointer_mut(pointer).unwrap() = value;
        head.metadata
            .insert(ORIGIN_METADATA_KEY.into(), origin.to_string());
        assert!(!head.allowed(&principal, true), "accepted {pointer}");
        assert!(
            head.allowed(&principal, false),
            "internal ACL changed for {pointer}"
        );
    }
    let mut head = original;
    head.metadata
        .insert(ORIGIN_METADATA_KEY.into(), "malformed".into());
    assert!(!head.allowed(&principal, true));
    assert!(head.allowed(&principal, false));
}

fn metadata_dependencies(
    record: &mut SourceRecordV2,
    dependencies: Vec<brain_contracts::DocumentRevision>,
) {
    record.metadata.insert(
        brain_contracts::domain::DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
        serde_json::to_string(&brain_contracts::source::Versioned::new(dependencies)).unwrap(),
    );
}

fn domain_dependency(id: &str, publication: bool) -> SourceRecordV2 {
    let mut dependency = record(id, 1, publication, false);
    dependency.metadata.insert("patch".into(), "p1".into());
    dependency.metadata.insert("mode".into(), "ranked".into());
    dependency
}

#[tokio::test]
async fn publication_checks_direct_and_transitive_metadata_dependencies_pinned_and_current() {
    use brain_contracts::domain::{DomainAnswer, DomainRequest};
    for transitive in [false, true] {
        for revoke_current in [false, true] {
            let store = MemoryRepository::default();
            let mut data = domain_fixture::records("r1", "p1", 1, "500");
            let dependency = domain_dependency("hidden", revoke_current);
            let mut parent = domain_dependency("parent", true);
            metadata_dependencies(&mut parent, vec![domain_fixture::revision(&dependency)]);
            let source = if transitive { &parent } else { &dependency };
            metadata_dependencies(
                data.iter_mut()
                    .find(|r| r.logical_id == "fact-damage")
                    .unwrap(),
                vec![domain_fixture::revision(source)],
            );
            data.extend([dependency.clone(), parent]);
            for record in data {
                store.apply_record(record).unwrap();
            }
            publish(&store).await;
            let kernel = Kernel::new(ReleaseRetriever::new(store.clone(), 10), NoProvider);
            let mut q = domain_fixture::query(
                &DomainRequest::Rule {
                    rule_id: "fixture-dps".into(),
                },
                "p1",
            );
            q.conversation_id = "c1".into();
            let internal = kernel.answer(&q, &context());
            assert_eq!(internal.status, AnswerStatus::Answered);
            let proof: DomainAnswer = serde_json::from_str(&internal.citations[0].content).unwrap();
            assert!(proof
                .inputs
                .iter()
                .any(|input| input.source.source_id == dependency.source_id));
            let api = service(CachedKernel::new(
                kernel.clone(),
                8,
                Duration::from_secs(30),
            ));
            if revoke_current {
                assert_eq!(external(&api, &q).status, AnswerStatus::Answered);
                let mut revoked = dependency;
                revoked.revision = 2;
                let mut origin = brain_contracts::source::origin_from_record(&revoked).unwrap();
                origin.policy.publication_allowed = false;
                origin.bind_record(&mut revoked).unwrap();
                store.apply_record(revoked).unwrap();
            }
            assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
            denied(&external(&api, &q));
            denied(&external(&service(kernel), &q));
        }
    }
}

#[tokio::test]
async fn domain_metadata_dependency_cycles_missing_sources_and_limits_fail_closed() {
    use brain_contracts::domain::DomainRequest;
    for case in [
        "cycle",
        "missing",
        "ambiguous",
        "depth",
        "version",
        "oversized",
    ] {
        let store = MemoryRepository::default();
        let mut data = domain_fixture::records("r1", "p1", 1, "500");
        let mut dependencies: Vec<_> = (0..if case == "depth" { 65 } else { 2 })
            .map(|n| domain_dependency(&format!("dep-{n}"), true))
            .collect();
        for index in 0..dependencies.len() - 1 {
            let next = domain_fixture::revision(&dependencies[index + 1]);
            metadata_dependencies(&mut dependencies[index], vec![next]);
        }
        if case == "cycle" {
            let first = domain_fixture::revision(&dependencies[0]);
            metadata_dependencies(&mut dependencies[1], vec![first]);
        }
        if case == "missing" {
            dependencies.pop();
        }
        if case == "ambiguous" {
            let next = domain_fixture::revision(&dependencies[1]);
            metadata_dependencies(&mut dependencies[0], vec![next.clone(), next]);
        }
        if case == "version" {
            dependencies[0].metadata.insert(
                brain_contracts::domain::DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
                "{\"contract_version\":\"brain.ir.v99\",\"data\":[]}".into(),
            );
        }
        if case == "oversized" {
            let next = domain_fixture::revision(&dependencies[1]);
            metadata_dependencies(&mut dependencies[0], vec![next; 65]);
        }
        metadata_dependencies(
            data.iter_mut()
                .find(|r| r.logical_id == "fact-damage")
                .unwrap(),
            vec![domain_fixture::revision(&dependencies[0])],
        );
        data.extend(dependencies);
        for record in data {
            store.apply_record(record).unwrap();
        }
        publish(&store).await;
        let kernel = Kernel::new(ReleaseRetriever::new(store, 10), NoProvider);
        let mut q = domain_fixture::query(
            &DomainRequest::Rule {
                rule_id: "fixture-dps".into(),
            },
            "p1",
        );
        q.conversation_id = "c1".into();
        assert_ne!(
            kernel.answer(&q, &context()).status,
            AnswerStatus::Answered,
            "accepted {case}"
        );
        assert_ne!(
            external(&service(kernel), &q).status,
            AnswerStatus::Answered,
            "published {case}"
        );
    }
}
fn query(profile: AnswerProfile) -> Query {
    Query {
        request_id: "publication-request".into(),
        conversation_id: "c1".into(),
        text: if profile == AnswerProfile::Fact {
            "Abrams health".into()
        } else {
            "Abrams".into()
        },
        requested_scopes: BTreeSet::new(),
        profile,
        patch: None,
        mode: None,
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "actor".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 8000,
        budget: Budget::default(),
        request_deadline: None,
    }
}
fn service<K: AnswerKernelPort>(kernel: K) -> ApiService<K> {
    ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "publication-token",
            "actor",
            "test",
            BTreeSet::new(),
            BTreeSet::from(["public".into()]),
        )])),
        kernel,
        "r1",
        8000,
        Budget::default(),
    )
}
fn external<K: AnswerKernelPort>(service: &ApiService<K>, query: &Query) -> PublicAnswerResponse {
    let response = service.handle_answer(
        Some("Bearer publication-token"),
        &serde_json::to_vec(query).unwrap(),
    );
    assert_eq!(response.status, 200, "{}", response.body);
    let answer: PublicAnswerResponse = serde_json::from_str(&response.body).unwrap();
    answer.validate(&query.request_id).unwrap();
    assert!(!response.body.contains("dependencies"));
    answer
}
fn denied(answer: &PublicAnswerResponse) {
    assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("650"));
    assert!(!answer.text.contains("uncited B"));
    assert!(!answer.text.contains("private input"));
}
async fn publish(store: &MemoryRepository) {
    let release = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&release).await.unwrap();
}
#[derive(Clone)]
struct Provider {
    calls: Arc<AtomicUsize>,
    revoke: Option<MemoryRepository>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(
            evidence.len(),
            2,
            "provider must receive A+B, not only the cited source"
        );
        assert!(evidence.iter().any(|e| e.source_id == "publication-b"));
        if let Some(store) = &self.revoke {
            store.apply_record(record("b", 2, false, false)).unwrap();
        }
        Ok(ProviderAnswer {
            text: "answer derived from uncited B".into(),
            cited_evidence_ids: vec![evidence
                .iter()
                .find(|e| e.source_id == "publication-a")
                .unwrap()
                .evidence_id
                .clone()],
            usage: Usage {
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}
#[derive(Clone)]
struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("facts must stay provider-free")
    }
}

#[tokio::test]
async fn retrieval_returns_original_public_text_without_provider_or_internal_metadata() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true, true)).unwrap();
    publish(&store).await;
    let api = service(Kernel::new(
        ReleaseRetriever::new(store.clone(), 10),
        NoProvider,
    ))
    .with_retrieval(ReleaseRetriever::new(store.clone(), 10));
    let q = query(AnswerProfile::Explain);
    let body = serde_json::to_vec(&q).unwrap();
    let response = api.handle_retrieve(Some("Bearer publication-token"), &body);
    assert_eq!(response.status, 200, "{}", response.body);
    let public: brain_contracts::public_api::PublicRetrievalResponse =
        serde_json::from_str(&response.body).unwrap();
    assert_eq!(public.status, AnswerStatus::Answered);
    assert_eq!(public.evidence.len(), 1);
    assert_eq!(public.evidence[0].text, "hero: Abrams\nhealth: 650");
    assert_eq!(public.evidence[0].kind, brain_contracts::EvidenceKind::Fact);
    for secret in [
        "publication-a",
        "entity/hero",
        "fixture-grant",
        "allowed_scopes",
        "source_id",
        "locator",
        "origin_artifact",
    ] {
        assert!(!response.body.contains(secret));
    }
    assert_eq!(api.handle_retrieve(None, &body).status, 401);
    let mut escalated = q;
    escalated.requested_scopes.insert("admin".into());
    assert_eq!(
        api.handle_retrieve(
            Some("Bearer publication-token"),
            &serde_json::to_vec(&escalated).unwrap()
        )
        .status,
        403
    );
    store.apply_record(record("a", 2, false, true)).unwrap();
    let revoked = api.handle_retrieve(Some("Bearer publication-token"), &body);
    assert_eq!(revoked.status, 403);
    assert!(!revoked.body.contains("650"));
}

#[tokio::test]
async fn retrieval_distinguishes_absent_knowledge_from_unavailable_and_denied_publication() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true, true)).unwrap();
    publish(&store).await;
    let api = service(Kernel::new(
        ReleaseRetriever::new(store.clone(), 10),
        NoProvider,
    ))
    .with_retrieval(ReleaseRetriever::new(store.clone(), 10));
    let mut q = query(AnswerProfile::Explain);
    q.text = "xxxxxxxxnomatch".into();
    let response = api.handle_retrieve(
        Some("Bearer publication-token"),
        &serde_json::to_vec(&q).unwrap(),
    );
    assert_eq!(response.status, 200, "{}", response.body);
    let public: brain_contracts::public_api::PublicRetrievalResponse =
        serde_json::from_str(&response.body).unwrap();
    assert_eq!(public.status, AnswerStatus::InsufficientEvidence);
    assert!(public.evidence.is_empty());
    let unavailable = service(Kernel::new(ReleaseRetriever::new(store, 10), NoProvider));
    assert_eq!(
        unavailable
            .handle_retrieve(
                Some("Bearer publication-token"),
                &serde_json::to_vec(&q).unwrap()
            )
            .status,
        503
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retrieval_http_route_enforces_authentication_and_dispatches_without_provider() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true, true)).unwrap();
    publish(&store).await;
    let api = service(Kernel::new(
        ReleaseRetriever::new(store.clone(), 10),
        NoProvider,
    ))
    .with_retrieval(ReleaseRetriever::new(store, 10));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(api)).await.unwrap();
    });
    let body = serde_json::to_string(&query(AnswerProfile::Explain)).unwrap();
    for (authorization, expected) in [
        ("Bearer publication-token", "HTTP/1.1 200"),
        ("Bearer invalid", "HTTP/1.1 401"),
    ] {
        let mut socket = tokio::net::TcpStream::connect(address).await.unwrap();
        let request = format!("POST /v1/retrieve HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nAuthorization: {authorization}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
        socket.write_all(request.as_bytes()).await.unwrap();
        let mut response = String::new();
        socket.read_to_string(&mut response).await.unwrap();
        assert!(response.starts_with(expected), "{response}");
        if expected.ends_with("200") {
            assert!(response.contains("hero: Abrams"));
            assert!(response.contains("no-store"));
            assert!(!response.contains("publication-a"));
        } else {
            assert!(!response.contains("650"));
        }
    }
    task.abort();
}

#[tokio::test]
async fn retrieval_rejects_internal_text_even_when_credentials_allow_internal_reading() {
    let store = MemoryRepository::default();
    let mut private = record("a", 1, true, true);
    let mut origin = brain_contracts::source::origin_from_record(&private).unwrap();
    private.visibility = SourceVisibility::Internal;
    private.allowed_scopes.insert("docs.internal".into());
    origin.policy.visibility = private.visibility;
    origin.policy.allowed_scopes = private.allowed_scopes.clone();
    origin.bind_record(&mut private).unwrap();
    store.apply_record(private).unwrap();
    publish(&store).await;
    let api = ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "publication-token",
            "actor",
            "test",
            BTreeSet::from(["docs.internal".into()]),
            BTreeSet::from(["internal".into()]),
        )])),
        Kernel::new(ReleaseRetriever::new(store.clone(), 10), NoProvider),
        "r1",
        8000,
        Budget::default(),
    )
    .with_retrieval(ReleaseRetriever::new(store, 10));
    let response = api.handle_retrieve(
        Some("Bearer publication-token"),
        &serde_json::to_vec(&query(AnswerProfile::Explain)).unwrap(),
    );
    assert_eq!(response.status, 502, "{}", response.body);
    let error: brain_contracts::ApiErrorEnvelope = serde_json::from_str(&response.body).unwrap();
    assert_eq!(error.error.code, "invalid_retrieval_response");
    assert!(!response.body.contains("650"));
}
#[tokio::test]
async fn publication_denial_preserves_internal_fact_reads_but_blocks_external_text_and_citations() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, false, true)).unwrap();
    publish(&store).await;
    let retrieval = ReleaseRetriever::new(store, 10);
    let q = query(AnswerProfile::Fact);
    let evidence = retrieval.retrieve(&q, &context()).unwrap();
    assert_eq!(evidence.len(), 1);
    retrieval
        .validate_evidence(&q, &context(), &evidence, false)
        .unwrap();
    let kernel = Kernel::new(retrieval, NoProvider);
    assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
    let cached = CachedKernel::new(kernel.clone(), 8, Duration::from_secs(30));
    assert_eq!(cached.answer(&q, &context()).status, AnswerStatus::Answered);
    denied(&external(&service(cached), &q));
    denied(&external(&service(kernel), &q));
}
#[tokio::test]
async fn publication_denial_of_uncited_provider_input_blocks_external_answer() {
    for revoke_during_call in [false, true] {
        let store = MemoryRepository::default();
        store.apply_record(record("a", 1, true, false)).unwrap();
        store
            .apply_record(record("b", 1, revoke_during_call, false))
            .unwrap();
        publish(&store).await;
        let calls = Arc::new(AtomicUsize::new(0));
        let api = service(Kernel::new(
            ReleaseRetriever::new(store.clone(), 10),
            Provider {
                calls: calls.clone(),
                revoke: revoke_during_call.then_some(store),
            },
        ));
        denied(&external(&api, &query(AnswerProfile::Explain)));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
#[tokio::test]
async fn publication_cache_hit_rechecks_uncited_current_head_and_keeps_public_contract() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true, false)).unwrap();
    store.apply_record(record("b", 1, true, false)).unwrap();
    publish(&store).await;
    let calls = Arc::new(AtomicUsize::new(0));
    let api = service(CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store.clone(), 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
            },
        ),
        8,
        Duration::from_secs(30),
    ));
    let q = query(AnswerProfile::Explain);
    let first = external(&api, &q);
    assert_eq!(first.status, AnswerStatus::Answered);
    assert_eq!(first.citations.len(), 1);
    assert_eq!(external(&api, &q).status, AnswerStatus::Answered);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    store.apply_record(record("b", 2, false, false)).unwrap();
    denied(&external(&api, &q));
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "must reject the cache hit rather than silently recompute without B"
    );
}
#[tokio::test]
async fn publication_requires_pinned_and_current_grants_even_when_internal_read_remains_allowed() {
    for (pinned, current, missing_current_origin) in [
        (false, true, false),
        (true, false, false),
        (true, true, true),
    ] {
        let store = MemoryRepository::default();
        store.apply_record(record("a", 1, pinned, true)).unwrap();
        publish(&store).await;
        let mut head = record("a", 2, current, true);
        if missing_current_origin {
            head.metadata
                .remove(brain_contracts::source::ORIGIN_METADATA_KEY);
        }
        store.apply_record(head).unwrap();
        let kernel = Kernel::new(ReleaseRetriever::new(store, 10), NoProvider);
        let q = query(AnswerProfile::Fact);
        assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
        denied(&external(&service(kernel), &q));
    }
}

struct InternalOnlyKernel;
impl AnswerKernelPort for InternalOnlyKernel {
    fn answer(&self, _: &Query, _: &AuthorizedContext) -> brain_contracts::AnswerResponse {
        panic!("the public API must never fall back to an internal-only kernel")
    }
}
#[test]
fn publication_is_server_selected_and_unknown_kernel_adapters_fail_closed() {
    let api = service(InternalOnlyKernel);
    let q = query(AnswerProfile::Fact);
    let answer = external(&api, &q);
    assert_eq!(answer.status, AnswerStatus::Unavailable);
    assert!(answer.citations.is_empty());
    for field in [
        "purpose",
        "answer_purpose",
        "context",
        "publication_allowed",
    ] {
        let mut body = serde_json::to_value(&q).unwrap();
        body[field] = serde_json::json!("internal_read");
        let response = api.handle_answer(
            Some("Bearer publication-token"),
            &serde_json::to_vec(&body).unwrap(),
        );
        assert_eq!(
            response.status, 400,
            "client field {field} must not select internal use"
        );
    }
}
struct ReadOnlyRetrieval(ReleaseRetriever<MemoryRepository>);
impl RetrievalPort for ReadOnlyRetrieval {
    fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        self.0.retrieve(q, c)
    }
    fn validate_evidence(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
        p: bool,
    ) -> Result<(), PortError> {
        self.0.validate_evidence(q, c, e, p)
    }
}
#[tokio::test]
async fn publication_cannot_use_a_retriever_without_canonical_publication_validation() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, false, true)).unwrap();
    publish(&store).await;
    let kernel = Kernel::new(
        ReadOnlyRetrieval(ReleaseRetriever::new(store, 10)),
        NoProvider,
    );
    let q = query(AnswerProfile::Fact);
    assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
    let answer = external(&service(kernel), &q);
    assert_eq!(answer.status, AnswerStatus::Unavailable);
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("650"));
}

#[tokio::test]
async fn publication_checks_transitive_domain_dependencies_without_changing_internal_proofs() {
    let store = MemoryRepository::default();
    let mut data = domain_fixture::records("r1", "p1", 1, "500");
    let dependency = data
        .iter_mut()
        .find(|r| r.logical_id == "mechanics")
        .unwrap();
    let mut origin =
        brain_contracts::source::origin_from_record(&record("a", 1, true, false)).unwrap();
    origin.identity.source_id = dependency.source_id.clone();
    origin.identity.logical_id = dependency.logical_id.clone();
    origin.raw_sha256 = dependency.content_hash.clone();
    origin.bind_record(dependency).unwrap();
    let mut revoked = dependency.clone();
    revoked.revision = 2;
    origin.policy.publication_allowed = false;
    origin.bind_record(&mut revoked).unwrap();
    for record in data {
        store.apply_record(record).unwrap();
    }
    publish(&store).await;
    let kernel = Kernel::new(ReleaseRetriever::new(store.clone(), 10), NoProvider);
    let mut q = domain_fixture::query(
        &domain_fixture::build("Fixture Hero", "en", &["101", "102", "103"]),
        "p1",
    );
    q.conversation_id = "c1".into();
    let internal = kernel.answer(&q, &context());
    assert_eq!(internal.status, AnswerStatus::Answered);
    assert_eq!(internal.citations.len(), 1);
    assert_ne!(internal.citations[0].logical_id, "mechanics");
    let proof: brain_contracts::domain::DomainAnswer =
        serde_json::from_str(&internal.citations[0].content).unwrap();
    assert!(proof
        .inputs
        .iter()
        .any(|input| input.source.logical_id == "mechanics"));
    let api = service(CachedKernel::new(
        kernel.clone(),
        8,
        Duration::from_secs(30),
    ));
    assert_eq!(external(&api, &q).status, AnswerStatus::Answered);
    store.apply_record(revoked).unwrap();
    let still_internal = kernel.answer(&q, &context());
    assert_eq!(still_internal.status, AnswerStatus::Answered);
    assert_eq!(still_internal.text, internal.text);
    let rejected = external(&api, &q);
    denied(&rejected);
    assert!(!rejected.text.contains(&internal.text));
    // Cached or fresh, no domain source can substitute publication for reading.
    denied(&external(&service(kernel), &q));
}

#[tokio::test]
async fn publication_fully_granted_facts_remain_successful() {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true, true)).unwrap();
    publish(&store).await;
    let answer = external(
        &service(Kernel::new(ReleaseRetriever::new(store, 10), NoProvider)),
        &query(AnswerProfile::Fact),
    );
    assert_eq!(answer.status, AnswerStatus::Answered);
    assert!(answer.text.contains("650"));
    assert_eq!(answer.citations.len(), 1);
}
