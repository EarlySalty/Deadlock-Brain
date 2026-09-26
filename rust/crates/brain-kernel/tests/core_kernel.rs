use brain_contracts::{
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, Principal, ProviderAnswer, Query, RetrievalPort, SourceRecordV2,
    SourceVisibility, Usage,
};
use brain_feeds::{deadlock_assets, FeedPolicy};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use dbrain_sources::core::http::SourceHttpResponse;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
fn record(revision: u64) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: "a".into(),
        revision,
        content_hash: format!("hash-{revision}"),
        content: "Abrams fixture evidence".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
async fn store() -> MemoryRepository {
    let s = MemoryRepository::default();
    s.apply_record(record(1)).unwrap();
    let release = s.release_from_heads("r1", "v1", "p1").unwrap();
    s.publish(&release).await.unwrap();
    s
}
fn query() -> Query {
    Query {
        domain: None,
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "actor-a".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 2000,
        budget: Budget::default(),
    }
}

#[tokio::test]
async fn release_pinned_patch_agnostic_fact_and_mode_fail_closed() {
    let s = MemoryRepository::default();
    let mut fact = record(1);
    fact.logical_id = "entity/hero/Abrams".into();
    fact.content = "hero: Abrams\nhealth: 650".into();
    fact.metadata.insert("kind".into(), "fact".into());
    s.apply_record(fact).unwrap();
    let release = s.release_from_heads("r1", "v1", "p1").unwrap();
    s.publish(&release).await.unwrap();
    let kernel = Kernel::new(
        ReleaseRetriever::new(s.clone(), 10),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            revoke: None,
            forged: false,
        },
    );
    let mut request = query();
    request.profile = AnswerProfile::Fact;
    request.text = "Abrams Gesundheit".into();
    request.patch = Some("p1".into());
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::Answered
    );
    request.patch = Some("p0".into());
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::InsufficientEvidence
    );
    request.patch = Some("p1".into());
    request.mode = Some("ranked".into());
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::InsufficientEvidence
    );
}

#[tokio::test]
async fn asset_hero_entity_and_field_records_share_identity() {
    let s = MemoryRepository::default();
    for (logical_id, field, value) in [
        ("asset/hero/25", "hero", "Warden"),
        (
            "asset/hero/25/starting_stats.max_health.value",
            "max health",
            "770",
        ),
        ("asset/hero/25/starting_stats.stamina.value", "stamina", "3"),
    ] {
        let mut fact = record(1);
        fact.source_id = "deadlock-assets-heroes".into();
        fact.logical_id = logical_id.into();
        fact.content = format!("Hero: Warden\n{field}: {value}\n");
        fact.metadata.insert("kind".into(), "fact".into());
        fact.metadata.insert("name".into(), "Warden".into());
        fact.metadata
            .insert("connector".into(), "deadlock-assets".into());
        if field != "hero" {
            fact.metadata.insert("field".into(), field.into());
            fact.metadata.insert(
                "fact_key".into(),
                format!("starting_stats.{}.value", field.replace(' ', "_")),
            );
        }
        s.apply_record(fact).unwrap();
    }
    for (name, external_id) in [("Warden", "hero_25"), ("Other", "hero_25")] {
        let mut legacy = record(1);
        legacy.source_id = "legacy-entities".into();
        legacy.logical_id = format!("entity/hero/{name}");
        legacy.content = format!(
            "hero: {name}\nExternal ID: {external_id}\nSource: deadlock_assets_api\nAliases: Guardian\nhealth: 700\n"
        );
        legacy.metadata.insert("kind".into(), "fact".into());
        legacy
            .metadata
            .insert("connector".into(), "brain_legacy".into());
        legacy
            .metadata
            .insert("entity_source".into(), "deadlock_assets_api".into());
        legacy
            .metadata
            .insert("entity_external_id".into(), external_id.into());
        s.apply_record(legacy).unwrap();
    }
    for i in 0..110 {
        let mut unrelated = record(1);
        unrelated.logical_id = format!("entity/hero/Unrelated{i}");
        unrelated.content = format!("hero: Unrelated{i}\nhealth: 500\n");
        unrelated.metadata.insert("kind".into(), "fact".into());
        s.apply_record(unrelated).unwrap();
    }
    let release = s.release_from_heads("r1", "v1", "p1").unwrap();
    s.publish(&release).await.unwrap();
    let kernel = Kernel::new(
        ReleaseRetriever::new(s.clone(), 1),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            revoke: None,
            forged: false,
        },
    );
    let mut request = query();
    request.profile = AnswerProfile::Fact;
    request.text = "Warden max health".into();
    let answer = kernel.answer(&request, &context());
    assert_eq!(answer.status, AnswerStatus::Answered);
    assert!(answer.text.contains("770"));
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(
        answer.citations[0].logical_id,
        "asset/hero/25/starting_stats.max_health.value"
    );
    request.text = "Warden starting_stats.max_health.value 770".into();
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::Answered
    );
    request.text = "Warden starting_stats.max_health.value 3".into();
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::InsufficientEvidence
    );
    request.text = "Guardian max health".into();
    assert_eq!(
        kernel.answer(&request, &context()).status,
        AnswerStatus::InsufficientEvidence
    );
    let mut conflicting = record(2);
    conflicting.source_id = "legacy-entities".into();
    conflicting.logical_id = "entity/hero/Warden".into();
    conflicting.content = "hero: Warden\nExternal ID: hero_25\nSource: deadlock_assets_api\nAliases: Guardian\nmax health: 700\n".into();
    conflicting.metadata.insert("kind".into(), "fact".into());
    conflicting
        .metadata
        .insert("connector".into(), "brain_legacy".into());
    conflicting
        .metadata
        .insert("entity_source".into(), "deadlock_assets_api".into());
    conflicting
        .metadata
        .insert("entity_external_id".into(), "hero_25".into());
    s.apply_record(conflicting).unwrap();
    let release = s.release_from_heads("r2", "v2", "p1").unwrap();
    s.publish(&release).await.unwrap();
    let mut updated_context = context();
    updated_context.knowledge_release = "r2".into();
    request.text = "Warden max health".into();
    assert_eq!(
        kernel.answer(&request, &updated_context).status,
        AnswerStatus::InsufficientEvidence
    );
    request.text = "Warden max health 770".into();
    assert_eq!(
        kernel.answer(&request, &updated_context).status,
        AnswerStatus::InsufficientEvidence
    );
    request.text = "Warden stamina 3".into();
    assert_eq!(
        kernel.answer(&request, &updated_context).status,
        AnswerStatus::Answered
    );
}

#[tokio::test]
async fn assets_feed_field_answers_through_release_kernel() {
    let body = include_str!("../../brain-feeds/tests/fixtures/heroes.json");
    let response = SourceHttpResponse {
        url: "https://api.deadlock-api.com/v1/assets/heroes?only_active=true".into(),
        status: 200,
        content: body.as_bytes().to_vec(),
        headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
        observed_at: 1_790_000_000,
        attempts: 1,
    };
    let policy = FeedPolicy {
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        provider_egress_allowed: false,
        publication_allowed: false,
        raw_retention_allowed: false,
    };
    let batch = deadlock_assets::prepare_batch("heroes", response, None, &policy, None).unwrap();
    let store = MemoryRepository::default();
    for record in batch.records {
        store.apply_record(record).unwrap();
    }
    let mut legacy = record(1);
    legacy.source_id = "legacy-entities".into();
    legacy.logical_id = "entity/hero/Warden".into();
    legacy.content =
        "hero: Warden\nExternal ID: 25\nSource: deadlock_assets_api\nhealth: 700".into();
    legacy.metadata.insert("kind".into(), "fact".into());
    legacy
        .metadata
        .insert("connector".into(), "brain_legacy".into());
    legacy
        .metadata
        .insert("entity_source".into(), "deadlock_assets_api".into());
    legacy
        .metadata
        .insert("entity_external_id".into(), "hero_25".into());
    store.apply_record(legacy).unwrap();
    let release = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    let kernel = Kernel::new(
        ReleaseRetriever::new(store, 10),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            revoke: None,
            forged: false,
        },
    );
    let mut request = query();
    request.profile = AnswerProfile::Fact;
    request.text = "Warden max health".into();
    request.patch = Some("p1".into());
    let answer = kernel.answer(&request, &context());
    assert_eq!(answer.status, AnswerStatus::Answered);
    assert!(answer.text.contains("max health: 770"));
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(
        answer.citations[0].logical_id,
        "asset/hero/25/starting_stats.max_health.value"
    );
}
struct Provider {
    calls: Arc<AtomicUsize>,
    revoke: Option<MemoryRepository>,
    forged: bool,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _q: &Query,
        _c: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some(store) = &self.revoke {
            let mut changed = record(2);
            changed.visibility = SourceVisibility::Private;
            changed.allowed_scopes.insert("private".into());
            store.apply_record(changed).unwrap();
        }
        Ok(ProviderAnswer {
            text: "fixture grounded answer".into(),
            cited_evidence_ids: if self.forged {
                vec!["invented".into()]
            } else {
                evidence.iter().map(|e| e.evidence_id.clone()).collect()
            },
            usage: Usage {
                input_tokens: 10,
                output_tokens: 5,
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}
#[tokio::test]
async fn cache_separates_principal_release_and_rechecks_current_acl() {
    let s = store().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(s.clone(), 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
                forged: false,
            },
        ),
        8,
        Duration::from_secs(30),
    );
    let first = cache.answer(&query(), &context());
    assert_eq!(first.status, AnswerStatus::Answered);
    let mut q = query();
    q.request_id = "q2".into();
    let hit = cache.answer(&q, &context());
    assert_eq!(hit.request_id, "q2");
    assert_eq!(hit.text, first.text);
    assert_eq!(hit.usage.network_rounds, 0);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let mut c = context();
    c.principal.actor_id = "actor-b".into();
    assert_eq!(cache.answer(&q, &c).status, AnswerStatus::Answered);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    c = context();
    c.knowledge_release = "r2".into();
    assert_ne!(cache.answer(&q, &c).status, AnswerStatus::Answered);
    let mut blocked = record(2);
    blocked.visibility = SourceVisibility::Private;
    blocked.allowed_scopes.insert("private".into());
    s.apply_record(blocked).unwrap();
    let revoked = cache.answer(&q, &context());
    assert_ne!(revoked.status, AnswerStatus::Answered);
    assert!(revoked.citations.is_empty());
    assert!(!revoked.text.contains("grounded answer"));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn revocation_during_provider_call_prevents_publication() {
    let s = store().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = Kernel::new(
        ReleaseRetriever::new(s.clone(), 10),
        Provider {
            calls,
            revoke: Some(s),
            forged: false,
        },
    );
    let answer = kernel.answer(&query(), &context());
    assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("grounded answer"));
}
#[tokio::test]
async fn forged_provider_citation_is_rejected_and_failure_not_cached() {
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store().await, 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
                forged: true,
            },
        ),
        10,
        Duration::from_secs(30),
    );
    for _ in 0..2 {
        let answer = cache.answer(&query(), &context());
        assert_eq!(answer.status, AnswerStatus::ProviderError);
        assert!(answer.citations.is_empty());
    }
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
struct Metered(ReleaseRetriever<MemoryRepository>);
impl RetrievalPort for Metered {
    fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        self.0.retrieve(q, c)
    }
    fn retrieve_with_usage(
        &self,
        q: &Query,
        c: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        Ok((
            self.0.retrieve(q, c)?,
            Usage {
                input_tokens: 12,
                network_rounds: 1,
                ..Usage::default()
            },
        ))
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
async fn retrieval_and_provider_share_a_budget() {
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = Kernel::new(
        Metered(ReleaseRetriever::new(store().await, 10)),
        Provider {
            calls: calls.clone(),
            revoke: None,
            forged: false,
        },
    );
    let good = kernel.answer(&query(), &context());
    assert_eq!(good.status, AnswerStatus::Answered);
    assert_eq!(good.usage.network_rounds, 2);
    assert_eq!(good.usage.input_tokens, 22);
    let mut c = context();
    c.budget.max_network_rounds = 1;
    assert_eq!(
        kernel.answer(&query(), &c).status,
        AnswerStatus::BudgetExceeded
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
