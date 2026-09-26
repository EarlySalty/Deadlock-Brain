//! Regression: a single-flight follower must classify its OWN fresh validation failure.
use super::*;
use brain_contracts::{
    AnswerProfile, Budget, EvidenceKind, Principal, ProviderAnswer, SourceVisibility,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc,
};

struct Retrieval {
    evidence: Evidence,
    validations: Arc<AtomicUsize>,
    error: PortError,
}
impl RetrievalPort for Retrieval {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        Ok(vec![self.evidence.clone()])
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        if self.validations.fetch_add(1, Ordering::SeqCst) == 3 {
            Err(self.error.clone())
        } else {
            Ok(())
        }
    }
}
struct Provider {
    ready: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.ready.send(()).unwrap();
        self.release
            .lock()
            .unwrap()
            .recv_timeout(Duration::from_secs(3))
            .unwrap();
        Ok(ProviderAnswer {
            text: "grounded leader answer".into(),
            cited_evidence_ids: vec![evidence[0].evidence_id.clone()],
            usage: Usage::default(),
        })
    }
}
#[test]
fn shared_answer_follower_revalidates_and_distinguishes_reader_failure_from_denial() {
    for (error, status) in [
        (
            PortError::Unavailable("temporary reader failure".into()),
            AnswerStatus::Unavailable,
        ),
        (
            PortError::PermissionDenied("revoked".into()),
            AnswerStatus::UnauthorizedEvidence,
        ),
    ] {
        let validations = Arc::new(AtomicUsize::new(0));
        let evidence = Evidence {
            evidence_id: "e1".into(),
            source_id: "fixture".into(),
            logical_id: "a".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "Abrams".into(),
            citation: "fixture:a".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            patch: None,
            provenance: None,
        };
        let query = Query {
            request_id: "leader".into(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: Default::default(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
            domain: None,
        };
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: "actor".into(),
                channel: "fixture".into(),
                scopes: Default::default(),
                provider_egress: ["public".into()].into_iter().collect(),
            },
            conversation_id: "c1".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 8000,
            budget: Budget::default(),
        };
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let cache = Arc::new(CachedKernel::new(
            Kernel::new(
                Retrieval {
                    evidence,
                    validations: validations.clone(),
                    error,
                },
                Provider {
                    ready: ready_tx,
                    release: Mutex::new(release_rx),
                },
            ),
            8,
            Duration::from_secs(30),
        ));
        let key = cache_key(&query, &context).unwrap();
        let leader_cache = cache.clone();
        let leader_q = query.clone();
        let leader_c = context.clone();
        let leader = std::thread::spawn(move || leader_cache.answer(&leader_q, &leader_c));
        ready_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        let follower_cache = cache.clone();
        let follower = std::thread::spawn(move || {
            let mut query = query;
            query.request_id = "follower".into();
            follower_cache.answer(&query, &context)
        });
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
            assert!(started.elapsed() < Duration::from_secs(2));
            std::thread::yield_now();
        }
        release_tx.send(()).unwrap();
        assert_eq!(leader.join().unwrap().status, AnswerStatus::Answered);
        let failed = follower.join().unwrap();
        assert_eq!(failed.status, status);
        assert!(failed.citations.is_empty());
        assert!(!failed.text.contains("grounded leader answer"));
        assert_eq!(validations.load(Ordering::SeqCst), 4);
    }
}
