use brain_contracts::{source::*, *};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::result::Result;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

fn source(patch: Option<&str>) -> SourceRecordV2 {
    let content = "hero: Abrams\nhealth: 650";
    let mut r = SourceRecordV2 {
        source_id: "wiki".into(),
        logical_id: "entity/hero/Abrams".into(),
        revision: 1,
        content: content.into(),
        content_hash: "c3518df66dc8b1de972981a1a473a666c9125a499827a9e82c58bd68e219270f".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: [("kind".into(), "fact".into())].into(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: r.source_id.clone(),
            logical_id: r.logical_id.clone(),
        },
        source_revision: SourceRevision::Wiki {
            page_id: 1,
            revision_id: 1,
        },
        raw_sha256: r.content_hash.clone(),
        locator: "wiki:1".into(),
        parser_revision: "test-v1".into(),
        parser_family: "test".into(),
        schema_version: observed_option(None),
        schema_sha256: observed_option(None),
        retrieved_at: observed_option(None),
        source_time: observed_option(None),
        language: observed_option(None),
        origin_artifacts: BTreeSet::new(),
        derivation_family: observed_option(None),
        policy: SourcePolicy {
            visibility: r.visibility,
            allowed_scopes: r.allowed_scopes.clone(),
            authorization_ref: observed_option(None),
            license: observed_option(None),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity {
            patch: observed_option(patch.map(str::to_owned)),
            ..GameValidity::unknown()
        },
    }
    .bind_record(&mut r)
    .unwrap();
    r
}
fn changed_head(r: &SourceRecordV2, patch: Option<&str>) -> SourceRecordV2 {
    let mut head = r.clone();
    let mut origin = origin_from_record(r).unwrap();
    head.revision += 1;
    origin.source_revision = SourceRevision::Wiki {
        page_id: 1,
        revision_id: head.revision as i64,
    };
    origin.validity.patch = observed_option(patch.map(str::to_owned));
    origin.bind_record(&mut head).unwrap();
    head
}
async fn published(r: SourceRecordV2) -> MemoryRepository {
    let store = MemoryRepository::default();
    store.apply_record(r).unwrap();
    let release = store.release_from_heads("r1", "index-v99", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}
fn query() -> Query {
    Query {
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: "Abrams health".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Fact,
        patch: Some("p1".into()),
        mode: None,
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: None,
        request_deadline: None,
        principal: Principal {
            actor_id: "tester".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 5000,
        budget: Budget::default(),
    }
}
struct Provider {
    calls: Arc<AtomicUsize>,
    change: Option<(MemoryRepository, SourceRecordV2)>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some((store, head)) = &self.change {
            store.apply_record(head.clone())?;
        }
        Ok(ProviderAnswer {
            text: "PATCH_BOUND_650".into(),
            cited_evidence_ids: vec![evidence[0].evidence_id.clone()],
            usage: Usage::default(),
        })
    }
}

#[tokio::test]
async fn direct_fact_never_answers_old_or_unknown_as_requested_patch() {
    for (patch, status) in [
        (Some("p0"), AnswerStatus::InsufficientEvidence),
        (None, AnswerStatus::InsufficientEvidence),
        (Some("p1"), AnswerStatus::Answered),
    ] {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            ReleaseRetriever::new(published(source(patch)).await, 1),
            Provider {
                calls: calls.clone(),
                change: None,
            },
        );
        let answer = kernel.answer(&query(), &context());
        assert_eq!(answer.status, status, "{patch:?}");
        if patch != Some("p1") {
            assert!(!answer.text.contains("650"));
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn unknown_fact_remains_answerable_without_game_patch_claim() {
    let kernel = Kernel::new(
        ReleaseRetriever::new(published(source(None)).await, 1),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            change: None,
        },
    );
    let mut q = query();
    q.patch = None;
    let answer = kernel.answer(&q, &context());
    assert_eq!(answer.status, AnswerStatus::Answered);
    assert!(answer.citations.iter().all(|e| e.patch.is_none()));
}

#[tokio::test]
async fn canonical_known_patch_with_legacy_null_still_answers_directly() {
    let mut r = source(Some("p1"));
    r.metadata.insert("patch".into(), "null".into());
    let kernel = Kernel::new(
        ReleaseRetriever::new(published(r).await, 1),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            change: None,
        },
    );
    assert_eq!(
        kernel.answer(&query(), &context()).status,
        AnswerStatus::Answered
    );
}

#[tokio::test]
async fn cached_provider_answer_is_not_reused_after_head_patch_change() {
    let r = source(Some("p1"));
    let store = published(r.clone()).await;
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store.clone(), 1),
            Provider {
                calls: calls.clone(),
                change: None,
            },
        ),
        4,
        Duration::from_secs(60),
    );
    let mut q = query();
    q.profile = AnswerProfile::Explain;
    assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
    q.request_id = "q2".into();
    assert_eq!(kernel.answer(&q, &context()).status, AnswerStatus::Answered);
    assert_eq!(calls.load(Ordering::SeqCst), 1, "warm cache exercised");
    store.apply_record(changed_head(&r, Some("p0"))).unwrap();
    q.request_id = "q3".into();
    let answer = kernel.answer(&q, &context());
    assert_ne!(answer.status, AnswerStatus::Answered);
    assert!(!answer.text.contains("PATCH_BOUND_650"));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn head_patch_change_during_provider_call_blocks_handoff() {
    let r = source(Some("p1"));
    let store = published(r.clone()).await;
    let kernel = Kernel::new(
        ReleaseRetriever::new(store.clone(), 1),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            change: Some((store, changed_head(&r, Some("p0")))),
        },
    );
    let mut q = query();
    q.profile = AnswerProfile::Explain;
    let answer = kernel.answer(&q, &context());
    assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
    assert!(!answer.text.contains("PATCH_BOUND_650"));
}

#[tokio::test]
async fn unknown_patch_fact_cannot_hide_a_conflict_with_known_fact() {
    let known = source(Some("p1"));
    let mut unknown = source(None);
    let mut origin = origin_from_record(&unknown).unwrap();
    unknown.source_id = "other-wiki".into();
    unknown.content = "hero: Abrams\nhealth: 700".into();
    unknown.content_hash =
        "5d21ea6a6f4dcf1224a0d35b1f5e2eaeb208e2ad78c8f1cfe84a8f17af732b39".into();
    origin.identity.source_id = unknown.source_id.clone();
    origin.raw_sha256 = unknown.content_hash.clone();
    origin.bind_record(&mut unknown).unwrap();
    let store = MemoryRepository::default();
    store.apply_record(known).unwrap();
    store.apply_record(unknown.clone()).unwrap();
    let release = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    let old_head = changed_head(&unknown, Some("p0"));
    store.apply_record(old_head.clone()).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store.clone(), 1),
            Provider {
                calls: calls.clone(),
                change: None,
            },
        ),
        4,
        Duration::from_secs(60),
    );
    assert_eq!(
        kernel.answer(&query(), &context()).status,
        AnswerStatus::Answered
    );
    store.apply_record(changed_head(&old_head, None)).unwrap();
    let mut q = query();
    q.request_id = "q2".into();
    assert_eq!(
        kernel.answer(&q, &context()).status,
        AnswerStatus::InsufficientEvidence
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
