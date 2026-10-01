//! Deterministic publication revocation at the actual single-flight handoff.
use super::*;
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
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
const WATCHDOG: Duration = Duration::from_secs(5);
fn record(id: &str, revision: u64, publication: bool) -> SourceRecordV2 {
    let mut r = SourceRecordV2 {
        source_id: "publication-flight".into(),
        logical_id: id.into(),
        revision,
        content_hash: id.repeat(64),
        content: format!("Abrams evidence {id}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: r.source_id.clone(),
            logical_id: r.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: r.content_hash.clone(),
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
            visibility: r.visibility,
            allowed_scopes: BTreeSet::new(),
            authorization_ref: Observed::known("fixture".into()),
            license: Observed::unknown(UnknownReason::NotPresent),
            publication_allowed: publication,
            provider_egress_allowed: true,
            raw_retention_allowed: false,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut r)
    .unwrap();
    r
}
async fn store(publication: bool) -> MemoryRepository {
    let s = MemoryRepository::default();
    s.apply_record(record("a", 1, true)).unwrap();
    s.apply_record(record("b", 1, publication)).unwrap();
    s.publish(&s.release_from_heads("r1", "v1", "p1").unwrap())
        .await
        .unwrap();
    s
}
fn query(id: &str) -> Query {
    Query {
        request_id: id.into(),
        conversation_id: "c1".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 15000,
        budget: Budget::default(),
        request_deadline: None,
    }
}
fn provider_answer(evidence: &[Evidence]) -> ProviderAnswer {
    assert_eq!(evidence.len(), 2);
    assert!(evidence.iter().any(|e| e.logical_id == "b"));
    ProviderAnswer {
        text: "sensitive B output".into(),
        cited_evidence_ids: vec![evidence
            .iter()
            .find(|e| e.logical_id == "a")
            .unwrap()
            .evidence_id
            .clone()],
        usage: Usage::default(),
    }
}
struct Provider {
    entered: mpsc::Sender<()>,
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
        self.entered.send(()).unwrap();
        self.release.lock().unwrap().recv_timeout(WATCHDOG).unwrap();
        Ok(provider_answer(evidence))
    }
}
struct FollowerGate {
    entered: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
}
struct Retrieval {
    inner: ReleaseRetriever<MemoryRepository>,
    follower: FollowerGate,
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
        provider: bool,
    ) -> Result<(), PortError> {
        self.inner.validate_evidence(q, c, e, provider)
    }
    fn validate_publication(
        &self,
        q: &Query,
        c: &AuthorizedContext,
        e: &[Evidence],
    ) -> Result<(), PortError> {
        if q.request_id == "follower" {
            assert_eq!(e.len(), 2, "uncited B must survive the flight handoff");
            self.follower.entered.send(()).unwrap();
            self.follower
                .release
                .lock()
                .unwrap()
                .recv_timeout(WATCHDOG)
                .unwrap();
        }
        self.inner.validate_publication(q, c, e)
    }
}
fn assert_denied(answer: &AnswerResponse) {
    assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("sensitive B"));
}

#[tokio::test]
async fn publication_follower_rechecks_uncited_head_after_successful_leader_and_cache_fill() {
    for revoke in [false, true] {
        let store = store(true).await;
        let (provider_tx, provider_rx) = mpsc::channel();
        let (release_provider, provider_release) = mpsc::channel();
        let (follower_tx, follower_rx) = mpsc::channel();
        let (release_follower, follower_release) = mpsc::channel();
        let calls = Arc::new(AtomicUsize::new(0));
        let cache = Arc::new(CachedKernel::new(
            Kernel::new(
                Retrieval {
                    inner: ReleaseRetriever::new(store.clone(), 10),
                    follower: FollowerGate {
                        entered: follower_tx,
                        release: Mutex::new(follower_release),
                    },
                },
                Provider {
                    entered: provider_tx,
                    release: Mutex::new(provider_release),
                    calls: calls.clone(),
                },
            ),
            8,
            Duration::from_secs(30),
        ));
        let purpose = AnswerPurpose::ExternalPublication;
        let key = cache_key(&query("leader"), &context(), purpose).unwrap();
        let leader_cache = cache.clone();
        let leader = std::thread::spawn(move || {
            leader_cache.answer_for_purpose(&query("leader"), &context(), purpose)
        });
        provider_rx.recv_timeout(WATCHDOG).unwrap();
        let follower_cache = cache.clone();
        let follower = std::thread::spawn(move || {
            follower_cache.answer_for_purpose(&query("follower"), &context(), purpose)
        });
        // Actual coordinator registration, not an assumed scheduling delay.
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
            assert!(started.elapsed() < WATCHDOG);
            std::thread::yield_now();
        }
        release_provider.send(()).unwrap();
        let first = leader.join().unwrap();
        assert_eq!(first.status, AnswerStatus::Answered);
        assert_eq!(first.citations.len(), 1);
        follower_rx.recv_timeout(WATCHDOG).unwrap();
        if revoke {
            store.apply_record(record("b", 2, false)).unwrap();
        }
        release_follower.send(()).unwrap();
        let shared = follower.join().unwrap();
        let cached = cache.answer_for_purpose(&query("cache-hit"), &context(), purpose);
        if revoke {
            assert_denied(&shared);
            assert_denied(&cached);
        } else {
            assert_eq!(shared.status, AnswerStatus::Answered);
            assert_eq!(cached.status, AnswerStatus::Answered);
            assert_eq!(shared.citations.len(), 1);
            assert_eq!(shared.request_id, "follower");
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn publication_internal_and_external_flights_never_coalesce() {
    let store = store(false).await;
    let (entered, observed) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = Arc::new(CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store, 10),
            Provider {
                entered,
                release: Mutex::new(released),
                calls: calls.clone(),
            },
        ),
        8,
        Duration::from_secs(30),
    ));
    let internal_cache = cache.clone();
    let internal =
        std::thread::spawn(move || internal_cache.answer(&query("internal"), &context()));
    observed.recv_timeout(WATCHDOG).unwrap();
    let external = std::thread::spawn(move || {
        cache.answer_for_purpose(
            &query("external"),
            &context(),
            AnswerPurpose::ExternalPublication,
        )
    });
    // External must independently enter the provider before the internal flight completes.
    observed.recv_timeout(WATCHDOG).unwrap();
    release.send(()).unwrap();
    release.send(()).unwrap();
    assert_eq!(internal.join().unwrap().status, AnswerStatus::Answered);
    assert_denied(&external.join().unwrap());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn publication_cache_keys_and_reuse_are_separated_from_internal_reading() {
    struct Immediate(Arc<AtomicUsize>);
    impl AnswerProviderPort for Immediate {
        fn answer(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            e: &[Evidence],
        ) -> Result<ProviderAnswer, PortError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(provider_answer(e))
        }
    }
    for publication in [false, true] {
        let calls = Arc::new(AtomicUsize::new(0));
        let cache = CachedKernel::new(
            Kernel::new(
                ReleaseRetriever::new(store(publication).await, 10),
                Immediate(calls.clone()),
            ),
            8,
            Duration::from_secs(30),
        );
        let q = query("same");
        let c = context();
        assert_ne!(
            cache_key(&q, &c, AnswerPurpose::InternalRead).unwrap(),
            cache_key(&q, &c, AnswerPurpose::ExternalPublication).unwrap()
        );
        assert_eq!(cache.answer(&q, &c).status, AnswerStatus::Answered);
        let external = cache.answer_for_purpose(&q, &c, AnswerPurpose::ExternalPublication);
        if publication {
            assert_eq!(external.status, AnswerStatus::Answered);
        } else {
            assert_denied(&external);
        }
        assert_eq!(
            calls.load(Ordering::SeqCst),
            2,
            "no internal cache entry may be reused for publication"
        );
        assert_eq!(cache.answer(&q, &c).status, AnswerStatus::Answered);
        assert_eq!(
            calls.load(Ordering::SeqCst),
            2,
            "denied publication must not evict internal cache"
        );
    }
}
