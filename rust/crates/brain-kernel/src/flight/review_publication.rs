//! Deterministic publication checks through the real coordinator and canonical heads.
use super::*;
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    store::AnswerPurpose,
    value::{Observed, UnknownReason},
    AnswerProfile, Budget, DocumentStorePort, Principal, ProviderAnswer, SourceRecordV2,
    SourceVisibility,
};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc,
};

fn record(id: &str, revision: u64, publication: bool) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: "publication-flight".into(),
        logical_id: id.into(),
        revision,
        content_hash: "a".repeat(64),
        content: format!("Abrams input {id}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: Default::default(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: Default::default(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: format!("fixture:{id}"),
        parser_revision: "v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("en".into()),
        origin_artifacts: Default::default(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: Default::default(),
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
fn request() -> (Query, AuthorizedContext) {
    (
        Query {
            request_id: "leader".into(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: Default::default(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
            domain: None,
        },
        AuthorizedContext {
            principal: Principal {
                actor_id: "actor".into(),
                channel: "test".into(),
                scopes: Default::default(),
                provider_egress: std::collections::BTreeSet::from(["public".into()]),
            },
            conversation_id: "c1".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 8000,
            budget: Budget::default(),
            request_deadline: None,
        },
    )
}
async fn store() -> MemoryRepository {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1, true)).unwrap();
    store.apply_record(record("b", 1, true)).unwrap();
    let release = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}
struct Retrieval {
    inner: ReleaseRetriever<MemoryRepository>,
    follower_ready: mpsc::Sender<()>,
    follower_release: Arc<(Mutex<bool>, Condvar)>,
}
impl RetrievalPort for Retrieval {
    fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        self.inner.retrieve(q, c)
    }
    fn validate_evidence(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
        p: bool,
    ) -> Result<(), PortError> {
        self.inner.validate_evidence(q, c, e, p)
    }
    fn validate_publication(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
    ) -> Result<(), PortError> {
        // Hold the actual follower's handoff until the leader has returned its
        // successful response and the test has committed the canonical revocation.
        if q.request_id == "follower" {
            self.follower_ready.send(()).unwrap();
            let (lock, signal) = &*self.follower_release;
            let (released, _) = signal
                .wait_timeout_while(lock.lock().unwrap(), Duration::from_secs(5), |released| {
                    !*released
                })
                .unwrap();
            assert!(*released, "test must release the follower validation gate");
        }
        self.inner.validate_publication(q, c, e)
    }
}
struct Provider {
    ready: mpsc::Sender<()>,
    release: Arc<(Mutex<bool>, Condvar)>,
    calls: Arc<AtomicUsize>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        e: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(e.len(), 2);
        self.ready.send(()).unwrap();
        let (lock, signal) = &*self.release;
        let (released, _) = signal
            .wait_timeout_while(lock.lock().unwrap(), Duration::from_secs(5), |released| {
                !*released
            })
            .unwrap();
        assert!(*released, "test must release the provider gate");
        Ok(ProviderAnswer {
            text: "text derived from uncited B".into(),
            cited_evidence_ids: vec![e
                .iter()
                .find(|e| e.logical_id == "a")
                .unwrap()
                .evidence_id
                .clone()],
            usage: Usage::default(),
        })
    }
}
fn unblock(gate: &Arc<(Mutex<bool>, Condvar)>) {
    *gate.0.lock().unwrap() = true;
    gate.1.notify_all();
}

#[tokio::test]
async fn publication_follower_rechecks_uncited_head_after_successful_leader() {
    let store = store().await;
    let (ready, entered) = mpsc::channel();
    let (follower_ready, follower_entered) = mpsc::channel();
    let follower_release = Arc::new((Mutex::new(false), Condvar::new()));
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = Arc::new(CachedKernel::new(
        Kernel::new(
            Retrieval {
                inner: ReleaseRetriever::new(store.clone(), 10),
                follower_ready,
                follower_release: follower_release.clone(),
            },
            Provider {
                ready,
                release: gate.clone(),
                calls: calls.clone(),
            },
        ),
        8,
        Duration::from_secs(30),
    ));
    let (q, c) = request();
    let key = cache_key_for_purpose(&q, &c, AnswerPurpose::ExternalPublication).unwrap();
    let leader_cache = cache.clone();
    let leader_q = q.clone();
    let leader_c = c.clone();
    let leader =
        std::thread::spawn(move || leader_cache.answer_for_publication(&leader_q, &leader_c));
    entered.recv_timeout(Duration::from_secs(3)).unwrap();
    let follower_cache = cache.clone();
    let mut follower_q = q.clone();
    let follower_c = c.clone();
    follower_q.request_id = "follower".into();
    let follower =
        std::thread::spawn(move || follower_cache.answer_for_publication(&follower_q, &follower_c));
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
    unblock(&gate);
    let leader = leader.join().unwrap();
    assert_eq!(leader.status, AnswerStatus::Answered);
    assert_eq!(leader.citations.len(), 1);
    follower_entered
        .recv_timeout(Duration::from_secs(3))
        .unwrap();
    store.apply_record(record("b", 2, false)).unwrap();
    unblock(&follower_release);
    let follower = follower.join().unwrap();
    assert_eq!(follower.request_id, "follower");
    assert_eq!(follower.status, AnswerStatus::UnauthorizedEvidence);
    assert!(follower.citations.is_empty());
    assert!(!follower.text.contains("uncited B"));
    assert_eq!(
        cache.answer_for_publication(&q, &c).status,
        AnswerStatus::UnauthorizedEvidence
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(cache.flights.flights.lock().unwrap().is_empty());
}

#[tokio::test]
async fn publication_and_internal_requests_do_not_share_flights_or_cache_entries() {
    let store = store().await;
    let (ready, entered) = mpsc::channel();
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = Arc::new(CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store.clone(), 10),
            Provider {
                ready,
                release: gate.clone(),
                calls: calls.clone(),
            },
        ),
        8,
        Duration::from_secs(30),
    ));
    let (q, c) = request();
    let internal_key = cache_key_for_purpose(&q, &c, AnswerPurpose::InternalRead).unwrap();
    let external_key = cache_key_for_purpose(&q, &c, AnswerPurpose::ExternalPublication).unwrap();
    assert_ne!(internal_key, external_key);
    let internal_cache = cache.clone();
    let internal_q = q.clone();
    let internal_c = c.clone();
    let internal = std::thread::spawn(move || internal_cache.answer(&internal_q, &internal_c));
    entered.recv_timeout(Duration::from_secs(3)).unwrap();
    let external_cache = cache.clone();
    let external_q = q.clone();
    let external_c = c.clone();
    let external =
        std::thread::spawn(move || external_cache.answer_for_publication(&external_q, &external_c));
    // Receiving a second provider entry proves that the external request did not
    // join the internal flight. No elapsed-time sleep is used as synchronization.
    entered.recv_timeout(Duration::from_secs(3)).unwrap();
    {
        let flights = cache.flights.flights.lock().unwrap();
        assert!(flights.contains_key(&internal_key));
        assert!(flights.contains_key(&external_key));
    }
    unblock(&gate);
    assert_eq!(internal.join().unwrap().status, AnswerStatus::Answered);
    assert_eq!(external.join().unwrap().status, AnswerStatus::Answered);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(cache.answer(&q, &c).status, AnswerStatus::Answered);
    assert_eq!(
        cache.answer_for_publication(&q, &c).status,
        AnswerStatus::Answered
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    store.apply_record(record("b", 2, false)).unwrap();
    assert_eq!(
        cache.answer(&q, &c).status,
        AnswerStatus::Answered,
        "publication denial is not an internal read denial"
    );
    assert_eq!(
        cache.answer_for_publication(&q, &c).status,
        AnswerStatus::UnauthorizedEvidence
    );
}
