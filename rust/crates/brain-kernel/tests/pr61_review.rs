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
