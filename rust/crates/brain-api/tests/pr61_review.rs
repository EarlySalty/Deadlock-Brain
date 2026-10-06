//! Independent PR #61 review regressions; synthetic data only.
use brain_api::ApiService;
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget,
    DocumentStorePort, Evidence, PortError, ProviderAnswer, PublicAnswerResponse,
    Query, SourceRecordV2, SourceVisibility, Usage,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{collections::{BTreeMap, BTreeSet}, sync::{Arc, atomic::{AtomicUsize, Ordering}}, time::Duration};

fn record(id: &str, revision: u64, content: &str) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "review-fixture".into(), logical_id: id.into(), revision,
        content_hash: "a".repeat(64), content: content.into(),
        visibility: SourceVisibility::Public, allowed_scopes: BTreeSet::new(),
        tombstone: false, valid_from: None, valid_to: None, metadata: BTreeMap::new(),
    }
}
fn bind_origin(record: &mut SourceRecordV2, publication: bool, patch: Option<&str>) {
    let mut validity = GameValidity::unknown();
    if let Some(patch) = patch { validity.patch = Observed::known(patch.into()); }
    OriginArtifact {
        identity: SourceIdentity { source_id: record.source_id.clone(), logical_id: record.logical_id.clone() },
        source_revision: SourceRevision::Api { api_version: "fixture-v1".into(), original_revision: Some("source-rev-1".into()) },
        raw_sha256: record.content_hash.clone(), locator: "fixture:source".into(),
        parser_revision: "fixture-parser-1".into(), parser_family: "review".into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("en".into()), origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility, allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: Observed::known("fixture-internal-use-only".into()),
            license: Observed::known("fixture".into()), publication_allowed: publication,
            provider_egress_allowed: true, raw_retention_allowed: false,
        }, validity,
    }.bind_record(record).unwrap();
}
fn query(profile: AnswerProfile) -> Query {
    Query { request_id: "review-request-1".into(), conversation_id: "review-conversation".into(),
        text: "Abrams health".into(), requested_scopes: BTreeSet::new(), profile,
        patch: None, mode: None, domain: None }
}
fn api<K: AnswerKernelPort>(kernel: K) -> ApiService<K> {
    let grants = CredentialRegistry::new(vec![AuthGrant::from_secret(
        "review-token", "review-public-consumer", "twitch", BTreeSet::new(), BTreeSet::from(["public".into()]),
    )]);
    ApiService::new(PolicyEngine::new(grants), kernel, "review-release", 10_000, Budget::default())
}
fn call<K: AnswerKernelPort>(api: &ApiService<K>, query: &Query) -> PublicAnswerResponse {
    let result = api.handle_answer(Some("Bearer review-token"), &serde_json::to_vec(query).unwrap());
    assert_eq!(result.status, 200, "{}", result.body);
    serde_json::from_str(&result.body).unwrap()
}
async fn store(records: Vec<SourceRecordV2>) -> MemoryRepository {
    let store = MemoryRepository::default();
    for record in records { store.apply_record(record).unwrap(); }
    let release = store.release_from_heads("review-release", "review-knowledge", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}
fn revoke(store: &MemoryRepository) {
    let mut changed = record("b", 2, "Abrams health REVOKED_SYNTHETIC_SOURCE_VALUE");
    changed.visibility = SourceVisibility::Private;
    changed.allowed_scopes.insert("review.denied".into());
    store.apply_record(changed).unwrap();
}
struct PartialCitationProvider { revoke_during_call: Option<MemoryRepository>, calls: Arc<AtomicUsize> }
impl AnswerProviderPort for PartialCitationProvider {
    fn answer(&self, _query: &Query, _context: &AuthorizedContext, evidence: &[Evidence]) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert!(evidence.iter().any(|e| e.logical_id == "b"));
        let cited = evidence.iter().find(|e| e.logical_id == "a").unwrap();
        if let Some(store) = &self.revoke_during_call { revoke(store); }
        Ok(ProviderAnswer { text: "REVOKED_SYNTHETIC_SOURCE_VALUE".into(),
            cited_evidence_ids: vec![cited.evidence_id.clone()],
            usage: Usage { input_tokens: 20, output_tokens: 10, network_rounds: 1, ..Usage::default() } })
    }
}
struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(&self, _: &Query, _: &AuthorizedContext, _: &[Evidence]) -> Result<ProviderAnswer, PortError> {
        panic!("fact route must not need a model");
    }
}
async fn two_sources() -> MemoryRepository {
    store(vec![record("a", 1, "Abrams health public reference"),
        record("b", 1, "Abrams health REVOKED_SYNTHETIC_SOURCE_VALUE")]).await
}
#[tokio::test]
async fn uncited_prompt_source_revoked_during_call_must_block_publication() {
    let store = two_sources().await;
    let api = api(Kernel::new(ReleaseRetriever::new(store.clone(), 10), PartialCitationProvider {
        revoke_during_call: Some(store), calls: Arc::new(AtomicUsize::new(0)),
    }));
    let answer = call(&api, &query(AnswerProfile::Explain));
    eprintln!("UNCITED_REVOKE: status={:?}, text={}", answer.status, answer.text);
    assert_ne!(answer.status, AnswerStatus::Answered, "revoked prompt input must taint the entire generated answer");
}
#[tokio::test]
async fn cached_answer_must_track_uncited_prompt_dependencies() {
    let store = two_sources().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let api = api(CachedKernel::new(Kernel::new(ReleaseRetriever::new(store.clone(), 10), PartialCitationProvider {
        revoke_during_call: None, calls: calls.clone(),
    }), 8, Duration::from_secs(60)));
    let mut request = query(AnswerProfile::Explain);
    assert_eq!(call(&api, &request).status, AnswerStatus::Answered);
    revoke(&store);
    request.request_id = "review-request-2".into();
    let answer = call(&api, &request);
    eprintln!("CACHE_UNCITED_REVOKE: status={:?}, model_calls={}, text={}", answer.status, calls.load(Ordering::SeqCst), answer.text);
    assert_ne!(answer.status, AnswerStatus::Answered, "cache must reauthorize all inputs, not only displayed citations");
}
#[tokio::test]
async fn public_fact_response_must_enforce_source_publication_policy() {
    let mut fact = record("entity/hero/Abrams", 1, "hero: Abrams\nhealth: 650");
    fact.metadata.insert("kind".into(), "fact".into());
    bind_origin(&mut fact, false, None);
    let store = store(vec![fact]).await;
    let api = api(Kernel::new(ReleaseRetriever::new(store, 10), NoProvider));
    let answer = call(&api, &query(AnswerProfile::Fact));
    eprintln!("PUBLICATION_DENIED: status={:?}, text={}", answer.status, answer.text);
    assert_ne!(answer.status, AnswerStatus::Answered, "public consumer must not receive source content with publication_allowed=false");
}
#[tokio::test]
async fn explicit_origin_patch_mismatch_must_not_be_relabelled_as_release_patch() {
    let mut fact = record("entity/hero/Abrams", 1, "hero: Abrams\nhealth: 650");
    fact.metadata.insert("kind".into(), "fact".into());
    bind_origin(&mut fact, true, Some("p0"));
    let store = store(vec![fact]).await;
    let api = api(Kernel::new(ReleaseRetriever::new(store, 10), NoProvider));
    let mut request = query(AnswerProfile::Fact); request.patch = Some("p1".into());
    let answer = call(&api, &request);
    eprintln!("ORIGIN_PATCH_MISMATCH: status={:?}, text={}", answer.status, answer.text);
    assert_ne!(answer.status, AnswerStatus::Answered, "a p0 source fact is not a p1 fact merely because its corpus release is p1");
}
#[tokio::test]
async fn fact_cache_must_recheck_newly_visible_conflicts() {
    let mut a = record("entity/hero/Abrams", 1, "hero: Abrams\nhealth: 650");
    a.metadata.insert("kind".into(), "fact".into());
    let mut b = a.clone(); b.source_id = "second-source".into(); b.content = "hero: Abrams\nhealth: 999".into();
    let store = store(vec![a, b.clone()]).await;
    b.revision = 2; b.tombstone = true; store.apply_record(b.clone()).unwrap();
    let api = api(CachedKernel::new(Kernel::new(ReleaseRetriever::new(store.clone(), 10), NoProvider), 8, Duration::from_secs(60)));
    let mut request = query(AnswerProfile::Fact);
    let first = call(&api, &request); assert_eq!(first.status, AnswerStatus::Answered);
    b.revision = 3; b.tombstone = false; store.apply_record(b).unwrap();
    request.request_id = "review-request-2".into();
    let answer = call(&api, &request);
    let fresh_api = self::api(Kernel::new(ReleaseRetriever::new(store, 10), NoProvider));
    let fresh = call(&fresh_api, &request);
    eprintln!("FACT_CONFLICT_CACHE: cached_status={:?}, uncached_status={:?}, text={}", answer.status, fresh.status, answer.text);
    assert_eq!(fresh.status, AnswerStatus::InsufficientEvidence, "fresh evaluation must detect the restored conflicting source");
    assert_ne!(answer.status, AnswerStatus::Answered, "a cached deterministic fact must stop answering when another pinned assertion becomes visible");
}
