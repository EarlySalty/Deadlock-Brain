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
async fn current_canonical_egress_revocation_invalidates_a_warm_release_index() {
    for remove_origin in [false, true] {
        let store = MemoryRepository::default();
        let original = record("a", 1, true, false);
        store.apply_record(original.clone()).unwrap();
        publish(&store).await;
        let retrieval = ReleaseRetriever::new(store.clone(), 10);
        let q = query(AnswerProfile::Explain);
        let hits = retrieval.retrieve(&q, &context()).unwrap();
        assert_eq!(hits.len(), 1);
        let mut revoked = original;
        revoked.revision = 2;
        if remove_origin {
            revoked.metadata.remove(brain_contracts::source::ORIGIN_METADATA_KEY);
        } else {
            let mut origin = brain_contracts::source::origin_from_record(&revoked).unwrap();
            origin.policy.provider_egress_allowed = false;
            origin.bind_record(&mut revoked).unwrap();
        }
        store.apply_record(revoked).unwrap();
        assert_eq!(retrieval.retrieve(&q, &context()).unwrap().len(), 1,
            "provider egress revocation must not revoke internal reads");
        assert!(retrieval.validate_evidence(&q, &context(), &hits, true).is_err(),
            "current canonical egress revocation must reject pinned evidence; missing_origin={remove_origin}");
    }
}

#[tokio::test]
async fn publication_checks_metadata_dependencies_transitively_on_fresh_and_cached_answers() {
    use brain_contracts::{domain::*, source::Versioned};
    for nested in [false, true] {
        for revoke_current in [false, true] {
            let store = MemoryRepository::default();
            let mut data = domain_fixture::records("r1", "p1", 1, "500");
            let mut leaf = record("metadata-leaf", 1, revoke_current, false);
            leaf.metadata.insert("patch".into(), "p1".into());
            leaf.metadata.insert("mode".into(), "ranked".into());
            let leaf_ref = domain_fixture::revision(&leaf);
            let dependency_ref = if nested {
                let mut intermediate = record("metadata-intermediate", 1, true, false);
                intermediate.metadata.insert("patch".into(), "p1".into());
                intermediate.metadata.insert("mode".into(), "ranked".into());
                intermediate.metadata.insert(DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
                    serde_json::to_string(&Versioned::new(vec![leaf_ref.clone()])).unwrap());
                let reference = domain_fixture::revision(&intermediate);
                data.push(intermediate);
                reference
            } else {
                leaf_ref.clone()
            };
            data.iter_mut().find(|r| r.logical_id == "fact-damage").unwrap().metadata.insert(
                DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
                serde_json::to_string(&Versioned::new(vec![dependency_ref])).unwrap());
            data.push(leaf.clone());
            for record in data { store.apply_record(record).unwrap(); }
            publish(&store).await;
            let kernel = Kernel::new(ReleaseRetriever::new(store.clone(), 10), NoProvider);
            let mut q = domain_fixture::query(&DomainRequest::Rule { rule_id: "fixture-dps".into() }, "p1");
            q.conversation_id = "c1".into();
            let internal = kernel.answer(&q, &context());
            assert_eq!(internal.status, AnswerStatus::Answered);
            let api = service(CachedKernel::new(kernel.clone(), 8, Duration::from_secs(30)));
            if revoke_current {
                assert_eq!(external(&api, &q).status, AnswerStatus::Answered);
                assert_eq!(external(&api, &q).status, AnswerStatus::Answered);
                leaf.revision = 2;
                let mut origin = brain_contracts::source::origin_from_record(&leaf).unwrap();
                origin.policy.publication_allowed = false;
                origin.bind_record(&mut leaf).unwrap();
                store.apply_record(leaf).unwrap();
            }
            denied(&external(&api, &q));
            denied(&external(&service(kernel.clone()), &q));
            let still_internal = kernel.answer(&q, &context());
            assert_eq!(still_internal.status, AnswerStatus::Answered);
            assert_eq!(still_internal.text, internal.text);
            let proof: DomainAnswer = serde_json::from_str(&still_internal.citations[0].content).unwrap();
            assert!(proof.inputs.iter().any(|input| input.source == leaf_ref),
                "retain indirect dependencies in the complete internal proof");
            if nested {
                assert!(proof.inputs.iter().any(|input| input.source.source_id == "publication-metadata-intermediate"));
            }
        }
    }
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
