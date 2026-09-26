use brain_contracts::{
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, Principal, ProviderAnswer, Query, RetrievalPort, SourceRecordV2,
    SourceVisibility, Usage,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
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
