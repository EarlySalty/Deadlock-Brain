//! A follower must revalidate the uncited provider input against real current heads.
use super::*;
use brain_contracts::{
    AnswerProfile, Budget, DocumentStorePort, Principal, ProviderAnswer, SourceRecordV2,
    SourceVisibility,
};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
};

fn record(id: &str, revision: u64) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "review-flight".into(),
        logical_id: id.into(),
        revision,
        content_hash: format!("flight-{id}-{revision}"),
        content: format!("Abrams evidence {id}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
struct Retrieval {
    inner: ReleaseRetriever<MemoryRepository>,
    store: MemoryRepository,
    validations: AtomicUsize,
    tombstone: bool,
}
impl RetrievalPort for Retrieval {
    fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        self.inner.retrieve(q, c)
    }
    fn validate_evidence(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        evidence: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        // Leader: initial ACL, provider egress, post-provider ACL. Fourth handoff is
        // the registered follower, after a successful answer has been cached/shared.
        if self.validations.fetch_add(1, Ordering::SeqCst) == 3 {
            let mut revoked = record("b", 2);
            if self.tombstone {
                revoked.tombstone = true;
            } else {
                revoked.visibility = SourceVisibility::Private;
                revoked.allowed_scopes.insert("secret".into());
            }
            self.store.apply_record(revoked).unwrap();
        }
        self.inner.validate_evidence(q, c, evidence, provider)
    }
}
struct Provider {
    ready: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
    calls: Arc<AtomicUsize>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(evidence.len(), 2);
        self.ready.send(()).unwrap();
        self.release
            .lock()
            .unwrap()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        Ok(ProviderAnswer {
            text: "leader text derived from uncited B".into(),
            cited_evidence_ids: vec![evidence
                .iter()
                .find(|e| e.logical_id == "a")
                .unwrap()
                .evidence_id
                .clone()],
            usage: Usage::default(),
        })
    }
}

#[tokio::test]
async fn shared_answer_revalidates_uncited_acl_and_tombstone_after_leader_success() {
    for tombstone in [false, true] {
        let store = MemoryRepository::default();
        store.apply_record(record("a", 1)).unwrap();
        store.apply_record(record("b", 1)).unwrap();
        let release = store.release_from_heads("r1", "v1", "p1").unwrap();
        store.publish(&release).await.unwrap();
        let q = Query {
            request_id: "leader".into(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
            domain: None,
        };
        let c = AuthorizedContext {
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
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let cache = Arc::new(CachedKernel::new(
            Kernel::new(
                Retrieval {
                    inner: ReleaseRetriever::new(store.clone(), 10),
                    store,
                    validations: AtomicUsize::new(0),
                    tombstone,
                },
                Provider {
                    ready: ready_tx,
                    release: Mutex::new(release_rx),
                    calls: calls.clone(),
                },
            ),
            8,
            Duration::from_secs(30),
        ));
        let key = cache_key(&q, &c).unwrap();
        let leader_cache = cache.clone();
        let leader_q = q.clone();
        let leader_c = c.clone();
        let leader = std::thread::spawn(move || leader_cache.answer(&leader_q, &leader_c));
        ready_rx.recv_timeout(Duration::from_secs(3)).unwrap();
        let follower_cache = cache.clone();
        let mut follower_q = q.clone();
        follower_q.request_id = "follower".into();
        let follower_c = c.clone();
        let follower = std::thread::spawn(move || follower_cache.answer(&follower_q, &follower_c));
        let started = Instant::now();
        loop {
            let flight = cache
                .flights
                .flights
                .lock()
                .unwrap()
                .get(&key)
                .unwrap()
                .clone();
            if flight.state.lock().unwrap().waiters == 1 {
                break;
            }
            assert!(started.elapsed() < Duration::from_secs(3));
            std::thread::yield_now();
        }
        release_tx.send(()).unwrap();
        let leader = leader.join().unwrap();
        assert_eq!(leader.status, AnswerStatus::Answered);
        assert_eq!(leader.citations.len(), 1);
        let rejected = follower.join().unwrap();
        assert_eq!(
            rejected.status,
            AnswerStatus::UnauthorizedEvidence,
            "tombstone={tombstone}"
        );
        assert!(rejected.citations.is_empty());
        assert!(!rejected.text.contains("uncited B"));
        assert_eq!(
            cache.answer(&q, &c).status,
            AnswerStatus::UnauthorizedEvidence
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
