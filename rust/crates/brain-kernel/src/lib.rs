#![forbid(unsafe_code)]

use brain_contracts::{
    AnswerProviderPort, AnswerResponse, AnswerStatus, AuthorizedContext, Evidence, EvidenceKind,
    PortError, Query, RetrievalPort, Usage, CONTRACT_VERSION,
};
use brain_policy::{evidence_allowed, provider_egress_allowed};
mod cache;
mod execution;
mod flight;
pub use cache::CachedKernel;

pub trait AnswerKernelPort: Send + Sync {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse;
}

#[derive(Debug, Clone)]
pub struct Kernel<R, P> {
    retrieval: R,
    provider: P,
}

impl<R, P> Kernel<R, P> {
    pub fn new(retrieval: R, provider: P) -> Self {
        Self {
            retrieval,
            provider,
        }
    }
}

impl<R, P> AnswerKernelPort for Kernel<R, P>
where
    R: RetrievalPort,
    P: AnswerProviderPort,
{
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        if query.validate().is_err() || query.conversation_id != context.conversation_id {
            return response(
                query,
                context,
                AnswerStatus::UnauthorizedEvidence,
                "Anfragekontext ist ungültig.",
                Vec::new(),
                Usage::default(),
            );
        }

        execution::answer(&self.retrieval, &self.provider, query, context)
    }
}

fn response(
    query: &Query,
    context: &AuthorizedContext,
    status: AnswerStatus,
    text: impl Into<String>,
    citations: Vec<Evidence>,
    usage: Usage,
) -> AnswerResponse {
    AnswerResponse {
        contract_version: CONTRACT_VERSION.to_string(),
        request_id: query.request_id.clone(),
        knowledge_release: context.knowledge_release.clone(),
        status,
        text: text.into(),
        citations,
        usage,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeSet,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    use brain_contracts::{
        AnswerProfile, Budget, PortError, Principal, ProviderAnswer, SourceVisibility,
    };

    use super::*;

    #[derive(Clone)]
    struct FixedRetrieval(Vec<Evidence>);

    impl RetrievalPort for FixedRetrieval {
        fn retrieve(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            Ok(self.0.clone())
        }
        fn validate_evidence(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            evidence: &[Evidence],
            _provider: bool,
        ) -> Result<(), PortError> {
            if evidence.iter().all(|item| self.0.contains(item)) {
                Ok(())
            } else {
                Err(PortError::InvalidResponse(
                    "fixture evidence mismatch".into(),
                ))
            }
        }
    }

    #[derive(Clone)]
    struct CountingProvider {
        calls: Arc<AtomicUsize>,
    }

    impl AnswerProviderPort for CountingProvider {
        fn answer(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            _evidence: &[Evidence],
        ) -> Result<ProviderAnswer, PortError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(ProviderAnswer {
                text: "erklärt".into(),
                cited_evidence_ids: _evidence
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect(),
                usage: Usage {
                    provider: Some("fixture".into()),
                    model: Some("fixture".into()),
                    input_tokens: 10,
                    output_tokens: 2,
                    network_rounds: 1,
                    cost_micros: 0,
                },
            })
        }
    }

    fn evidence(
        id: &str,
        kind: EvidenceKind,
        visibility: SourceVisibility,
        scopes: &[&str],
    ) -> Evidence {
        Evidence {
            evidence_id: id.into(),
            source_id: "fixture".into(),
            logical_id: id.into(),
            revision: 1,
            kind,
            content: format!("Inhalt {id}"),
            citation: format!("fixture:{id}"),
            visibility,
            allowed_scopes: scopes.iter().map(|scope| (*scope).to_string()).collect(),
            score: 1.0,
            patch: None,
        }
    }

    fn context(scopes: &[&str], egress: &[&str]) -> AuthorizedContext {
        AuthorizedContext {
            principal: Principal {
                actor_id: "actor".into(),
                channel: "test".into(),
                scopes: scopes.iter().map(|scope| (*scope).to_string()).collect(),
                provider_egress: egress.iter().map(|item| (*item).to_string()).collect(),
            },
            conversation_id: "c1".into(),
            knowledge_release: "k1".into(),
            deadline_ms: 1_000,
            budget: Budget::default(),
        }
    }

    fn query(profile: AnswerProfile) -> Query {
        Query {
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: BTreeSet::new(),
            profile,
            patch: None,
            mode: None,
        }
    }

    #[test]
    fn fact_path_does_not_call_provider() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "fact",
                EvidenceKind::Fact,
                SourceVisibility::Public,
                &[],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let answer = kernel.answer(&query(AnswerProfile::Fact), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::Answered);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(answer.citations.len(), 1);
    }

    #[test]
    fn unauthorized_evidence_never_reaches_provider() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "private",
                EvidenceKind::Prose,
                SourceVisibility::Private,
                &["scrim.private"],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(answer.citations.is_empty());
    }

    #[test]
    fn internal_evidence_requires_internal_egress() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "internal",
                EvidenceKind::Prose,
                SourceVisibility::Internal,
                &["docs.internal"],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let answer = kernel.answer(
            &query(AnswerProfile::Explain),
            &context(&["docs.internal"], &["public"]),
        );
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn explain_path_calls_provider_for_public_evidence() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "public",
                EvidenceKind::Prose,
                SourceVisibility::Public,
                &[],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::Answered);
        assert_eq!(answer.text, "erklärt");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
