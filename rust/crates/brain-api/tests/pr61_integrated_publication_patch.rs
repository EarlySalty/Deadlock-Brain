//! Cross-package regressions for PR #61: publication purpose and canonical patch validity.
//! Synthetic records only; no network or PostgreSQL connection is opened.
use brain_api::ApiService;
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, ProviderAnswer, PublicAnswerResponse, Query, SourceRecordV2,
    SourceVisibility,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

fn fact(source: &str, health: u32, publication: bool, patch: Option<&str>) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: source.into(),
        logical_id: "entity/hero/Abrams".into(),
        revision: 1,
        content_hash: "a".repeat(64),
        content: format!("hero: Abrams\nhealth: {health}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([("kind".into(), "fact".into())]),
    };
    let mut validity = GameValidity::unknown();
    if let Some(patch) = patch {
        validity.patch = Observed::known(patch.into());
    }
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some("source-rev-1".into()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: "fixture:integration".into(),
        parser_revision: "fixture-parser-1".into(),
        parser_family: "integration".into(),
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
            authorization_ref: Observed::known("fixture-rights".into()),
            license: Observed::known("fixture".into()),
            publication_allowed: publication,
            provider_egress_allowed: true,
            raw_retention_allowed: false,
        },
        validity,
    }
    .bind_record(&mut record)
    .unwrap();
    record
}
fn query(id: &str) -> Query {
    Query {
        request_id: id.into(),
        conversation_id: "integration-conversation".into(),
        text: "Abrams health".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Fact,
        patch: Some("p1".into()),
        mode: None,
        domain: None,
    }
}
fn policy() -> PolicyEngine {
    PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
        "integration-token",
        "integration-actor",
        "twitch",
        BTreeSet::new(),
        BTreeSet::from(["public".into()]),
    )]))
}
fn internal_context(request: &Query) -> AuthorizedContext {
    policy()
        .authorize_query(
            "integration-token",
            request,
            "integration-release",
            10000,
            Budget::default(),
        )
        .unwrap()
}
async fn repository(records: Vec<SourceRecordV2>) -> MemoryRepository {
    let store = MemoryRepository::default();
    for record in records {
        store.apply_record(record).unwrap();
    }
    let release = store
        .release_from_heads("integration-release", "corpus-v1", "p1")
        .unwrap();
    store.publish(&release).await.unwrap();
    store
}
fn public_answer<K: AnswerKernelPort>(kernel: K, request: &Query) -> PublicAnswerResponse {
    let api = ApiService::new(
        policy(),
        kernel,
        "integration-release",
        10000,
        Budget::default(),
    );
    let response = api.handle_answer(
        Some("Bearer integration-token"),
        &serde_json::to_vec(request).unwrap(),
    );
    assert_eq!(response.status, 200, "{}", response.body);
    serde_json::from_str(&response.body).unwrap()
}
fn assert_not_published(answer: &PublicAnswerResponse) {
    assert_ne!(answer.status, AnswerStatus::Answered);
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("650"));
    assert!(!answer.text.contains("999"));
}
struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("fact-only integration case must not call a provider");
    }
}
#[tokio::test]
async fn internal_cache_cannot_publish_an_unpublishable_canonical_p1_fact() {
    let store = repository(vec![fact("internal-p1", 650, false, Some("p1"))]).await;
    let kernel = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(store, 10), NoProvider),
        8,
        Duration::from_secs(60),
    );
    let request = query("internal-warmup");
    let internal = kernel.answer(&request, &internal_context(&request));
    assert_eq!(
        internal.status,
        AnswerStatus::Answered,
        "publication denial must not revoke internal read permission"
    );
    assert!(internal.text.contains("650"));
    assert_not_published(&public_answer(
        kernel,
        &query("external-after-internal-cache"),
    ));
}
#[tokio::test]
async fn publication_filter_cannot_turn_a_known_old_patch_into_an_external_fallback() {
    let store = repository(vec![
        fact("internal-p1", 650, false, Some("p1")),
        fact("public-p0", 999, true, Some("p0")),
    ])
    .await;
    let kernel = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(store, 10), NoProvider),
        8,
        Duration::from_secs(60),
    );
    let request = query("internal-conflict-selection");
    let internal = kernel.answer(&request, &internal_context(&request));
    assert_eq!(
        internal.status,
        AnswerStatus::Answered,
        "known p0 source must not become a conflicting p1 candidate"
    );
    assert!(internal.text.contains("650"));
    assert!(!internal.text.contains("999"));
    assert_not_published(&public_answer(kernel, &query("external-no-stale-fallback")));
}
#[tokio::test]
async fn publication_permission_does_not_authorize_a_mismatched_patch() {
    let store = repository(vec![fact("public-p0", 999, true, Some("p0"))]).await;
    let kernel = Kernel::new(ReleaseRetriever::new(store, 10), NoProvider);
    assert_not_published(&public_answer(kernel, &query("external-wrong-patch")));
}
#[tokio::test]
async fn unknown_origin_patch_is_not_invented_from_the_corpus_version() {
    let store = repository(vec![fact("public-unknown", 650, true, None)]).await;
    let kernel = Kernel::new(ReleaseRetriever::new(store, 10), NoProvider);
    let request = query("internal-unknown-patch");
    let answer = kernel.answer(&request, &internal_context(&request));
    assert_eq!(
        answer.status,
        AnswerStatus::Answered,
        "preserve the existing generic unknown-validity contract"
    );
    assert!(!answer.citations.is_empty());
    assert!(
        answer
            .citations
            .iter()
            .all(|evidence| evidence.patch.is_none()),
        "a corpus release is not evidence of a known source game patch"
    );
}
