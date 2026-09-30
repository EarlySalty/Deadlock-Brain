//! PR #61 regressions using the real release retriever and canonical current heads.
use brain_contracts::{
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, Principal, ProviderAnswer, Query, SourceRecordV2, SourceVisibility, Usage,
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

fn record(id: &str, revision: u64) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: format!("review-{id}"),
        logical_id: "entity/hero/Abrams".into(),
        revision,
        content_hash: format!("hash-{id}-{revision}"),
        content: format!("Abrams input {id}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
fn query(profile: AnswerProfile, text: &str) -> Query {
    Query {
        request_id: "review-request".into(),
        conversation_id: "review-conversation".into(),
        text: text.into(),
        requested_scopes: BTreeSet::new(),
        profile,
        patch: None,
        mode: None,
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        request_deadline: None,
        principal: Principal {
            actor_id: "review-actor".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "review-conversation".into(),
        knowledge_release: "review-release".into(),
        deadline_ms: 5000,
        budget: Budget::default(),
    }
}
fn revoke(store: &MemoryRepository, mut b: SourceRecordV2, tombstone: bool) {
    b.revision = 2;
    if tombstone {
        b.tombstone = true;
    } else {
        b.visibility = SourceVisibility::Private;
        b.allowed_scopes.insert("revoked".into());
    }
    store.apply_record(b).unwrap();
}
async fn publish(store: &MemoryRepository) {
    let release = store
        .release_from_heads("review-release", "v1", "p1")
        .unwrap();
    store.publish(&release).await.unwrap();
}
struct PartialCitationProvider {
    calls: Arc<AtomicUsize>,
    revoke: Option<(MemoryRepository, bool)>,
}
impl AnswerProviderPort for PartialCitationProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        if call == 0 {
            assert_eq!(evidence.len(), 2, "provider must have consumed A and B");
        } else if evidence.len() != 2 {
            return Err(PortError::InvalidResponse(
                "fixture needs both inputs".into(),
            ));
        }
        assert!(evidence.iter().any(|e| e.source_id == "review-b"));
        if let Some((store, tombstone)) = &self.revoke {
            revoke(store, record("b", 1), *tombstone);
        }
        Ok(ProviderAnswer {
            text: "answer containing uncited B input".into(),
            cited_evidence_ids: vec![evidence
                .iter()
                .find(|e| e.source_id == "review-a")
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

#[tokio::test]
async fn uncited_provider_input_revoked_during_call_must_not_publish_or_cache() {
    for tombstone in [false, true] {
        let store = MemoryRepository::default();
        store.apply_record(record("a", 1)).unwrap();
        store.apply_record(record("b", 1)).unwrap();
        publish(&store).await;
        let cache = CachedKernel::new(
            Kernel::new(
                ReleaseRetriever::new(store.clone(), 10),
                PartialCitationProvider {
                    calls: Arc::new(AtomicUsize::new(0)),
                    revoke: Some((store.clone(), tombstone)),
                },
            ),
            8,
            Duration::from_secs(30),
        );
        let q = query(AnswerProfile::Explain, "Abrams");
        let first = cache.answer(&q, &context());
        assert_eq!(
            first.status,
            AnswerStatus::UnauthorizedEvidence,
            "tombstone={tombstone}"
        );
        assert!(first.citations.is_empty());
        assert!(!first.text.contains("uncited B"));
        // The same request must not recover a successful cached answer from the failed call.
        let second = cache.answer(&q, &context());
        assert!(!second.text.contains("uncited B"));
    }
}

#[tokio::test]
async fn cache_hit_revalidates_uncited_input_acl_and_tombstone() {
    for tombstone in [false, true] {
        let store = MemoryRepository::default();
        store.apply_record(record("a", 1)).unwrap();
        store.apply_record(record("b", 1)).unwrap();
        publish(&store).await;
        let calls = Arc::new(AtomicUsize::new(0));
        let cache = CachedKernel::new(
            Kernel::new(
                ReleaseRetriever::new(store.clone(), 10),
                PartialCitationProvider {
                    calls: calls.clone(),
                    revoke: None,
                },
            ),
            8,
            Duration::from_secs(30),
        );
        let q = query(AnswerProfile::Explain, "Abrams");
        let first = cache.answer(&q, &context());
        assert_eq!(first.status, AnswerStatus::Answered);
        assert_eq!(first.citations.len(), 1);
        assert_eq!(first.citations[0].source_id, "review-a");
        // Internal input dependencies must not expand even the internal response's citations.
        assert!(!serde_json::to_string(&first).unwrap().contains("review-b"));
        assert_eq!(cache.answer(&q, &context()).usage.network_rounds, 0);
        revoke(&store, record("b", 1), tombstone);
        let hit = cache.answer(&q, &context());
        assert_eq!(
            hit.status,
            AnswerStatus::UnauthorizedEvidence,
            "tombstone={tombstone}"
        );
        assert!(hit.citations.is_empty());
        assert!(!hit.text.contains("uncited B"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("fact selection must remain provider-free")
    }
}
#[tokio::test]
async fn fact_reuse_rechecks_newly_visible_conflict_and_alias_collision() {
    for alias_collision in [false, true] {
        let store = MemoryRepository::default();
        let mut a = record("a", 1);
        a.content = "hero: Abrams\nAliases: Guardian\nhealth: 650".into();
        a.metadata.insert("kind".into(), "fact".into());
        let mut b = record("b", 1);
        if alias_collision {
            b.logical_id = "entity/hero/Bebop".into();
            b.content = "hero: Bebop\nAliases: Guardian\nhealth: 700".into();
        } else {
            // Two field records of the SAME entity, not two name-colliding owners.
            a.source_id = "review-facts".into();
            a.logical_id = "asset/hero/25/a".into();
            b.source_id = "review-facts".into();
            b.logical_id = "asset/hero/25/b".into();
            b.content = "hero: Abrams\nhealth: 700".into();
        }
        b.metadata.insert("kind".into(), "fact".into());
        store.apply_record(a).unwrap();
        store.apply_record(b.clone()).unwrap();
        publish(&store).await;
        revoke(&store, b.clone(), false);
        let cache = CachedKernel::new(
            Kernel::new(ReleaseRetriever::new(store.clone(), 20), NoProvider),
            8,
            Duration::from_secs(30),
        );
        let q = query(
            AnswerProfile::Fact,
            if alias_collision {
                "Guardian health"
            } else {
                "Abrams health"
            },
        );
        assert_eq!(cache.answer(&q, &context()).status, AnswerStatus::Answered);
        b.revision = 3;
        store.apply_record(b).unwrap();
        let fresh =
            Kernel::new(ReleaseRetriever::new(store, 20), NoProvider).answer(&q, &context());
        assert_eq!(
            fresh.status,
            AnswerStatus::InsufficientEvidence,
            "alias={alias_collision}"
        );
        let reused = cache.answer(&q, &context());
        assert_eq!(reused.status, fresh.status, "alias={alias_collision}");
        assert!(reused.citations.is_empty());
    }
}

struct NoEmbedding;
impl brain_contracts::EmbeddingProviderPort for NoEmbedding {
    fn embed(
        &self,
        _: &[String],
        _: &brain_contracts::EmbeddingIdentity,
        _: &AuthorizedContext,
    ) -> Result<brain_contracts::EmbeddingOutput, PortError> {
        panic!("fact selection must not use embedding egress")
    }
}

#[tokio::test]
async fn hybrid_fact_limit_cannot_hide_a_pinned_conflicting_value() {
    let store = MemoryRepository::default();
    for (id, value) in [("a", 650), ("b", 700)] {
        let mut fact = record(id, 1);
        fact.source_id = "review-facts".into();
        fact.logical_id = format!("asset/hero/25/{id}");
        fact.content = format!("hero: Abrams\nhealth: {value}");
        fact.metadata.insert("kind".into(), "fact".into());
        store.apply_record(fact).unwrap();
    }
    publish(&store).await;
    let dense = dbrain_retrieval::DenseIndex {
        release_id: "review-release".into(),
        entries: Vec::new(),
        identity: brain_contracts::EmbeddingIdentity {
            provider: "fixture".into(),
            model: "fixture".into(),
            revision: "v1".into(),
            dimension: 2,
            pooling: "mean".into(),
            query_prefix: String::new(),
            document_prefix: String::new(),
            normalized: true,
        },
    };
    let hybrid = dbrain_retrieval::HybridRetriever::new(store, NoEmbedding, dense, 1).unwrap();
    let cache = CachedKernel::new(Kernel::new(hybrid, NoProvider), 8, Duration::from_secs(30));
    let answer = cache.answer(&query(AnswerProfile::Fact, "Abrams health"), &context());
    assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
    assert!(answer.citations.is_empty());
}

struct CountedStore {
    inner: MemoryRepository,
    snapshots: Arc<AtomicUsize>,
    heads: Arc<AtomicUsize>,
}
impl brain_contracts::SnapshotReadPort for CountedStore {
    fn read_snapshot(&self, release: &str) -> Result<brain_contracts::CorpusSnapshot, PortError> {
        self.snapshots.fetch_add(1, Ordering::SeqCst);
        brain_contracts::SnapshotReadPort::read_snapshot(&self.inner, release)
    }
    fn read_heads(
        &self,
        documents: &[brain_contracts::DocumentRevision],
    ) -> Result<Vec<brain_contracts::DocumentHead>, PortError> {
        assert!(documents.len() <= 256);
        self.heads.fetch_add(1, Ordering::SeqCst);
        brain_contracts::SnapshotReadPort::read_heads(&self.inner, documents)
    }
}
#[tokio::test]
async fn fresh_fact_selection_reuses_the_release_index_and_bounds_head_reads() {
    let store = MemoryRepository::default();
    let mut a = record("a", 1);
    a.content = "hero: Abrams\nhealth: 650".into();
    a.metadata.insert("kind".into(), "fact".into());
    store.apply_record(a).unwrap();
    // Include enough unrelated pinned facts to exercise the release index,
    // rather than measuring a one-document special case.
    for index in 1..256 {
        let mut unrelated = record(&format!("unrelated-{index}"), 1);
        unrelated.logical_id = format!("entity/hero/Unrelated{index}");
        unrelated.content = format!("hero: Unrelated{index}\nhealth: 650");
        unrelated.metadata.insert("kind".into(), "fact".into());
        store.apply_record(unrelated).unwrap();
    }
    publish(&store).await;
    let snapshots = Arc::new(AtomicUsize::new(0));
    let heads = Arc::new(AtomicUsize::new(0));
    let counted = CountedStore {
        inner: store,
        snapshots: snapshots.clone(),
        heads: heads.clone(),
    };
    let cache = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(counted, 10), NoProvider),
        8,
        Duration::from_secs(30),
    );
    let q = query(AnswerProfile::Fact, "Abrams health");
    let started = std::time::Instant::now();
    for _ in 0..128 {
        assert_eq!(cache.answer(&q, &context()).status, AnswerStatus::Answered);
    }
    assert_eq!(
        snapshots.load(Ordering::SeqCst),
        1,
        "do not reload or rebuild the immutable release per fact request"
    );
    assert_eq!(
        heads.load(Ordering::SeqCst),
        128 * 2,
        "a unique fact needs selection and final ACL head batches"
    );
    println!("fact reuse: 128 requests, 256 pinned facts, 1 release snapshot, 256 bounded head batches, no providers, elapsed {:?}", started.elapsed());
}

struct GatedFactRetrieval {
    inner: ReleaseRetriever<MemoryRepository>,
    first: std::sync::atomic::AtomicBool,
    ready: std::sync::mpsc::Sender<()>,
    release: std::sync::Mutex<std::sync::mpsc::Receiver<()>>,
}
impl brain_contracts::RetrievalPort for GatedFactRetrieval {
    fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        let selected = brain_contracts::RetrievalPort::retrieve(&self.inner, q, c)?;
        if self.first.swap(false, Ordering::SeqCst) {
            self.ready.send(()).unwrap();
            self.release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
        }
        Ok(selected)
    }
    fn validate_evidence(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        brain_contracts::RetrievalPort::validate_evidence(&self.inner, q, c, e, provider)
    }
}
#[tokio::test]
async fn concurrent_fact_requests_reselect_instead_of_sharing_an_old_winner() {
    for alias in [false, true] {
        let store = MemoryRepository::default();
        let mut a = record("a", 1);
        a.source_id = "review-facts".into();
        a.logical_id = "asset/hero/25/a".into();
        a.content = "hero: Abrams\nAliases: Guardian\nhealth: 650".into();
        a.metadata.insert("kind".into(), "fact".into());
        let mut b = a.clone();
        b.logical_id = if alias {
            "asset/hero/26/b"
        } else {
            "asset/hero/25/b"
        }
        .into();
        b.content = if alias {
            "hero: Bebop\nAliases: Guardian\nhealth: 700"
        } else {
            "hero: Abrams\nhealth: 700"
        }
        .into();
        store.apply_record(a).unwrap();
        store.apply_record(b.clone()).unwrap();
        publish(&store).await;
        revoke(&store, b.clone(), false);
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let retriever = GatedFactRetrieval {
            inner: ReleaseRetriever::new(store.clone(), 10),
            first: std::sync::atomic::AtomicBool::new(true),
            ready: ready_tx,
            release: std::sync::Mutex::new(release_rx),
        };
        let cache = Arc::new(CachedKernel::new(
            Kernel::new(retriever, NoProvider),
            8,
            Duration::from_secs(30),
        ));
        let q = query(
            AnswerProfile::Fact,
            if alias {
                "Guardian health"
            } else {
                "Abrams health"
            },
        );
        let first_cache = cache.clone();
        let first_q = q.clone();
        let first = std::thread::spawn(move || first_cache.answer(&first_q, &context()));
        ready_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        b.revision = 3;
        store.apply_record(b).unwrap();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let second = std::thread::spawn(move || {
            let answer = cache.answer(&q, &context());
            done_tx.send(answer).unwrap();
        });
        let result = done_rx.recv_timeout(Duration::from_secs(2));
        release_tx.send(()).unwrap();
        first.join().unwrap();
        second.join().unwrap();
        let result = result.expect("a fact follower must independently select current candidates, not await an old single-flight result");
        assert_eq!(
            result.status,
            AnswerStatus::InsufficientEvidence,
            "alias={alias}"
        );
        assert!(result.citations.is_empty());
    }
}
