//! Reviewer regression probes for PR #61 at 9635592. Synthetic data only.
use brain_contracts::{
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, Evidence,
    EvidenceKind, PortError, Principal, ProviderAnswer, Query, RetrievalPort, SourceVisibility,
    Usage,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

const PRIVATE_TEXT: &str = "SYNTHETIC_REVOKED_SOURCE_CONTENT";

fn source(id: &str, private: bool) -> Evidence {
    Evidence {
        evidence_id: id.into(),
        source_id: "review-fixture".into(),
        logical_id: id.into(),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: if private {
            PRIVATE_TEXT
        } else {
            "Public background"
        }
        .into(),
        citation: format!("fixture:{id}"),
        visibility: if private {
            SourceVisibility::Private
        } else {
            SourceVisibility::Public
        },
        allowed_scopes: if private {
            BTreeSet::from(["review.private".into()])
        } else {
            BTreeSet::new()
        },
        score: 1.0,
        provenance: None,
        patch: None,
    }
}
#[derive(Clone)]
struct Reader {
    revoked: Arc<AtomicBool>,
}
impl RetrievalPort for Reader {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        let mut result = vec![source("public", false)];
        if !self.revoked.load(Ordering::SeqCst) {
            result.push(source("private", true));
        }
        Ok(result)
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        if self.revoked.load(Ordering::SeqCst)
            && evidence.iter().any(|item| item.evidence_id == "private")
        {
            return Err(PortError::PermissionDenied("source revoked".into()));
        }
        Ok(())
    }
}
struct Provider {
    revoked: Arc<AtomicBool>,
    revoke_during_call: bool,
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
        let text = evidence
            .iter()
            .find(|item| item.evidence_id == "private")
            .map(|item| item.content.clone())
            .unwrap_or_else(|| "Public-only answer".into());
        if self.revoke_during_call {
            self.revoked.store(true, Ordering::SeqCst);
        }
        Ok(ProviderAnswer {
            text,
            cited_evidence_ids: vec!["public".into()],
            usage: Usage {
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}
fn request() -> Query {
    Query {
        request_id: "review-1".into(),
        conversation_id: "review-conversation".into(),
        text: "Summarize the evidence".into(),
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
            actor_id: "reviewer".into(),
            channel: "test".into(),
            scopes: BTreeSet::from(["review.private".into()]),
            provider_egress: BTreeSet::from(["public".into(), "private".into()]),
        },
        conversation_id: "review-conversation".into(),
        knowledge_release: "review-release".into(),
        deadline_ms: 5_000,
        request_deadline: None,
        budget: Budget::default(),
    }
}
#[test]
fn uncited_input_revoked_during_provider_call_must_block_publication() {
    let revoked = Arc::new(AtomicBool::new(false));
    let kernel = Kernel::new(
        Reader {
            revoked: revoked.clone(),
        },
        Provider {
            revoked,
            revoke_during_call: true,
            calls: Arc::new(AtomicUsize::new(0)),
        },
    );
    let answer = kernel.answer(&request(), &context());
    assert_eq!(
        answer.status,
        AnswerStatus::UnauthorizedEvidence,
        "must validate all provider inputs, not only citations; got {answer:?}"
    );
    assert!(!answer.text.contains(PRIVATE_TEXT));
}
#[test]
fn cached_answer_must_track_uncited_inputs() {
    let revoked = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = CachedKernel::new(
        Kernel::new(
            Reader {
                revoked: revoked.clone(),
            },
            Provider {
                revoked: revoked.clone(),
                revoke_during_call: false,
                calls: calls.clone(),
            },
        ),
        4,
        Duration::from_secs(60),
    );
    let first = kernel.answer(&request(), &context());
    assert_eq!(first.status, AnswerStatus::Answered);
    assert!(first.text.contains(PRIVATE_TEXT));
    revoked.store(true, Ordering::SeqCst);
    let mut next = request();
    next.request_id = "review-2".into();
    let answer = kernel.answer(&next, &context());
    assert!(
        !answer.text.contains(PRIVATE_TEXT),
        "cached response retained revoked input, calls={}, status={:?}",
        calls.load(Ordering::SeqCst),
        answer.status
    );
}

#[derive(Clone)]
struct FactReader {
    visible: Arc<AtomicBool>,
    alias_collision: bool,
    calls: Arc<AtomicUsize>,
}
impl RetrievalPort for FactReader {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let mut evidence = vec![fact("a", "Abrams", "650")];
        if self.visible.load(Ordering::SeqCst) {
            evidence.push(fact(
                "b",
                if self.alias_collision {
                    "Warden"
                } else {
                    "Abrams"
                },
                "700",
            ));
        }
        Ok(evidence)
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        Ok(())
    }
}
fn fact(id: &str, hero: &str, health: &str) -> Evidence {
    let mut item = source(id, false);
    item.kind = EvidenceKind::Fact;
    item.logical_id = format!("entity/hero/{hero}");
    item.content = format!("hero: {hero}\nAliases: Guardian\nhealth: {health}");
    item.provenance = Some(brain_contracts::ChunkProvenance {
        document: brain_contracts::DocumentRevision {
            source_id: item.source_id.clone(),
            logical_id: item.logical_id.clone(),
            revision: 1,
            content_hash: id.into(),
        },
        chunker_version: "fixture".into(),
        ordinal: 0,
        byte_start: 0,
        byte_end: item.content.len(),
        source_locator: id.into(),
        release_id: "review-release".into(),
        knowledge_version: "v1".into(),
        valid_from: None,
        valid_to: None,
        metadata: Default::default(),
    });
    item
}
#[test]
fn facts_reselect_newly_visible_conflicts_and_aliases_without_model_calls() {
    for alias_collision in [false, true] {
        let visible = Arc::new(AtomicBool::new(false));
        let retrieval_calls = Arc::new(AtomicUsize::new(0));
        let provider_calls = Arc::new(AtomicUsize::new(0));
        let reader = FactReader {
            visible: visible.clone(),
            alias_collision,
            calls: retrieval_calls.clone(),
        };
        let cached = CachedKernel::new(
            Kernel::new(
                reader.clone(),
                Provider {
                    revoked: Arc::new(AtomicBool::new(false)),
                    revoke_during_call: false,
                    calls: provider_calls.clone(),
                },
            ),
            4,
            Duration::from_secs(60),
        );
        let mut query = request();
        query.profile = AnswerProfile::Fact;
        query.text = if alias_collision {
            "Guardian health"
        } else {
            "Abrams health"
        }
        .into();
        assert_eq!(
            cached.answer(&query, &context()).status,
            AnswerStatus::Answered
        );
        visible.store(true, Ordering::SeqCst);
        assert_eq!(
            cached.answer(&query, &context()).status,
            AnswerStatus::InsufficientEvidence
        );
        let fresh = Kernel::new(
            reader,
            Provider {
                revoked: Arc::new(AtomicBool::new(false)),
                revoke_during_call: false,
                calls: provider_calls.clone(),
            },
        );
        assert_eq!(
            fresh.answer(&query, &context()).status,
            AnswerStatus::InsufficientEvidence
        );
        assert_eq!(retrieval_calls.load(Ordering::SeqCst), 3);
        assert_eq!(provider_calls.load(Ordering::SeqCst), 0);
    }
}

#[test]
fn revoked_uncited_inputs_do_not_create_successful_cache_entries() {
    let revoked = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let cached = CachedKernel::new(
        Kernel::new(
            Reader {
                revoked: revoked.clone(),
            },
            Provider {
                revoked: revoked.clone(),
                revoke_during_call: true,
                calls: calls.clone(),
            },
        ),
        4,
        Duration::from_secs(60),
    );
    for _ in 0..2 {
        revoked.store(false, Ordering::SeqCst);
        let answer = cached.answer(&request(), &context());
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert!(!answer.text.contains(PRIVATE_TEXT));
    }
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn concurrent_fact_reuse_reselects_instead_of_sharing_a_stale_selection() {
    use std::sync::{mpsc, Mutex};
    struct GatedFacts {
        reader: FactReader,
        entered: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    impl RetrievalPort for GatedFacts {
        fn retrieve(&self, q: &Query, c: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
            let result = self.reader.retrieve(q, c)?;
            if q.request_id == "leader" {
                self.entered.send(()).unwrap();
                self.release
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(3))
                    .unwrap();
            }
            Ok(result)
        }
        fn validate_evidence(
            &self,
            q: &Query,
            c: &AuthorizedContext,
            e: &[Evidence],
            p: bool,
        ) -> Result<(), PortError> {
            self.reader.validate_evidence(q, c, e, p)
        }
    }
    let visible = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let cached = Arc::new(CachedKernel::new(
        Kernel::new(
            GatedFacts {
                reader: FactReader {
                    visible: visible.clone(),
                    alias_collision: false,
                    calls: calls.clone(),
                },
                entered: entered_tx,
                release: Mutex::new(release_rx),
            },
            Provider {
                revoked: Arc::new(AtomicBool::new(false)),
                revoke_during_call: false,
                calls: Arc::new(AtomicUsize::new(0)),
            },
        ),
        4,
        Duration::from_secs(60),
    ));
    let mut q = request();
    q.profile = AnswerProfile::Fact;
    q.text = "Abrams health".into();
    q.request_id = "leader".into();
    let leader_cache = cached.clone();
    let leader_q = q.clone();
    let leader = std::thread::spawn(move || leader_cache.answer(&leader_q, &context()));
    entered_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    visible.store(true, Ordering::SeqCst);
    q.request_id = "follower".into();
    let (answer_tx, answer_rx) = mpsc::channel();
    let follower =
        std::thread::spawn(move || answer_tx.send(cached.answer(&q, &context())).unwrap());
    let observed = answer_rx.recv_timeout(Duration::from_secs(2));
    // Always release the first reader, including on an assertion failure.
    release_tx.send(()).unwrap();
    leader.join().unwrap();
    follower.join().unwrap();
    assert_eq!(
        observed
            .expect("fact follower must perform its own current selection")
            .status,
        AnswerStatus::InsufficientEvidence
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
