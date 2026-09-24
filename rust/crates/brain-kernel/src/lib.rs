#![forbid(unsafe_code)]

use brain_contracts::{
    AnswerProviderPort, AnswerResponse, AnswerStatus, AuthorizedContext, Evidence, EvidenceKind,
    PortError, Query, RetrievalPort, Usage, CONTRACT_VERSION,
};
use brain_policy::{evidence_allowed, provider_egress_allowed};

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

        let retrieved = match self.retrieval.retrieve(query, context) {
            Ok(evidence) => evidence,
            Err(PortError::BudgetExceeded) => {
                return response(
                    query,
                    context,
                    AnswerStatus::BudgetExceeded,
                    "Retrieval Budget ist ausgeschöpft.",
                    Vec::new(),
                    Usage::default(),
                )
            }
            Err(_) => {
                return response(
                    query,
                    context,
                    AnswerStatus::InsufficientEvidence,
                    "Evidenz konnte nicht geladen werden.",
                    Vec::new(),
                    Usage::default(),
                )
            }
        };

        let had_retrieved = !retrieved.is_empty();
        let mut evidence = retrieved
            .into_iter()
            .filter(|item| item.validate().is_ok())
            .filter(|item| evidence_allowed(&context.principal, item))
            .collect::<Vec<_>>();

        if evidence.is_empty() {
            return response(
                query,
                context,
                if had_retrieved {
                    AnswerStatus::UnauthorizedEvidence
                } else {
                    AnswerStatus::InsufficientEvidence
                },
                if had_retrieved {
                    "Gefundene Evidenz ist für diesen Kontext nicht freigegeben."
                } else {
                    "Für diese Frage liegt keine ausreichende Evidenz vor."
                },
                Vec::new(),
                Usage::default(),
            );
        }

        evidence.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| left.evidence_id.cmp(&right.evidence_id))
        });

        if matches!(query.profile, brain_contracts::AnswerProfile::Fact) {
            if let Some(fact) = evidence
                .iter()
                .find(|item| matches!(item.kind, EvidenceKind::Fact))
                .cloned()
            {
                return response(
                    query,
                    context,
                    AnswerStatus::Answered,
                    fact.content.clone(),
                    vec![fact],
                    Usage::default(),
                );
            }
        }

        if context.budget.max_network_rounds == 0 {
            return response(
                query,
                context,
                AnswerStatus::BudgetExceeded,
                "Für diese Anfrage sind keine Modellrunden freigegeben.",
                evidence,
                Usage::default(),
            );
        }

        let egress_class = if evidence
            .iter()
            .all(|item| matches!(item.visibility, brain_contracts::SourceVisibility::Public))
        {
            "public"
        } else {
            "internal"
        };
        if !provider_egress_allowed(&context.principal, egress_class) {
            return response(
                query,
                context,
                AnswerStatus::UnauthorizedEvidence,
                "Die Evidenz darf nicht an den Antwortprovider übertragen werden.",
                evidence,
                Usage::default(),
            );
        }

        let provider = match self.provider.answer(query, context, &evidence) {
            Ok(answer) => answer,
            Err(PortError::BudgetExceeded) => {
                return response(
                    query,
                    context,
                    AnswerStatus::BudgetExceeded,
                    "Provider Budget ist ausgeschöpft.",
                    evidence,
                    Usage::default(),
                )
            }
            Err(_) => {
                return response(
                    query,
                    context,
                    AnswerStatus::ProviderError,
                    "Der Antwortprovider ist nicht verfügbar.",
                    evidence,
                    Usage::default(),
                )
            }
        };

        if provider.usage.network_rounds > context.budget.max_network_rounds
            || provider.usage.input_tokens > context.budget.max_input_tokens as u64
            || provider.usage.output_tokens > context.budget.max_output_tokens as u64
            || provider.usage.cost_micros > context.budget.max_cost_micros
        {
            return response(
                query,
                context,
                AnswerStatus::BudgetExceeded,
                "Provider Nutzung überschreitet das freigegebene Budget.",
                evidence,
                provider.usage,
            );
        }

        response(
            query,
            context,
            AnswerStatus::Answered,
            provider.text,
            evidence,
            provider.usage,
        )
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
