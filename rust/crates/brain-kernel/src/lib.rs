#![forbid(unsafe_code)]

use brain_contracts::{
    AnswerProviderPort, AnswerResponse, AnswerStatus, AuthorizedContext, Evidence, PortError,
    Query, RetrievalPort, Usage, CONTRACT_VERSION,
};
use brain_policy::{evidence_allowed, provider_egress_allowed};
mod cache;
mod execution;
mod fact_relevance;
mod flight;
mod outcome;
use brain_contracts::store::AnswerPurpose;
pub use cache::CachedKernel;
use outcome::KernelAnswer;

pub trait AnswerKernelPort: Send + Sync {
    /// Internal use does not imply a publication grant.
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse;

    /// Public adapters MUST use this entry point. Unknown kernel adapters cannot
    /// publish by falling back to the internal-read implementation.
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        response(
            query,
            context,
            AnswerStatus::Unavailable,
            "Publikationsfreigabe konnte nicht sicher bestätigt werden.",
            Vec::new(),
            Usage::default(),
        )
    }
}
impl<K: AnswerKernelPort + ?Sized> AnswerKernelPort for std::sync::Arc<K> {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        (**self).answer(query, context)
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        (**self).answer_for_publication(query, context)
    }
}

#[derive(Clone)]
pub struct Kernel<R, P> {
    retrieval: R,
    provider: P,
    clock: std::sync::Arc<dyn Fn() -> std::time::Instant + Send + Sync>,
}

impl<R: std::fmt::Debug, P: std::fmt::Debug> std::fmt::Debug for Kernel<R, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Kernel")
            .field("retrieval", &self.retrieval)
            .field("provider", &self.provider)
            .finish_non_exhaustive()
    }
}

impl<R, P> Kernel<R, P> {
    pub fn new(retrieval: R, provider: P) -> Self {
        Self {
            retrieval,
            provider,
            clock: std::sync::Arc::new(std::time::Instant::now),
        }
    }

    pub fn with_clock(
        mut self,
        clock: impl Fn() -> std::time::Instant + Send + Sync + 'static,
    ) -> Self {
        self.clock = std::sync::Arc::new(clock);
        self
    }
}

impl<R: RetrievalPort, P: AnswerProviderPort> AnswerKernelPort for Kernel<R, P> {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer_with_purpose(query, context, AnswerPurpose::InternalRead)
            .answer
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer_with_purpose(query, context, AnswerPurpose::ExternalPublication)
            .answer
    }
}
impl<R: RetrievalPort, P: AnswerProviderPort> Kernel<R, P> {
    pub fn answer_uncached_for_publication_with_retrieval<T: RetrievalPort>(
        &self,
        retrieval: &T,
        query: &Query,
        context: &AuthorizedContext,
    ) -> AnswerResponse {
        let bound = context.with_request_deadline();
        if query.validate().is_err() || query.conversation_id != bound.conversation_id {
            return response(
                query,
                &bound,
                AnswerStatus::UnauthorizedEvidence,
                "Anfragekontext ist ungültig.",
                Vec::new(),
                Usage::default(),
            );
        }
        execution::answer(
            retrieval,
            &self.provider,
            query,
            &bound,
            AnswerPurpose::ExternalPublication,
            self.clock.as_ref(),
        )
        .answer
    }
    fn answer_with_purpose(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        purpose: AnswerPurpose,
    ) -> KernelAnswer {
        let bound = context.with_request_deadline();
        let context = &bound;
        if query.validate().is_err() || query.conversation_id != context.conversation_id {
            return response(
                query,
                context,
                AnswerStatus::UnauthorizedEvidence,
                "Anfragekontext ist ungültig.",
                Vec::new(),
                Usage::default(),
            )
            .into();
        }

        execution::answer(
            &self.retrieval,
            &self.provider,
            query,
            context,
            purpose,
            self.clock.as_ref(),
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
            atomic::{AtomicU64, AtomicUsize, Ordering},
            Arc,
        },
    };

    use brain_contracts::{
        AnswerProfile, Budget, EvidenceKind, PortError, Principal, ProviderAnswer, SourceVisibility,
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

    struct AdvancingValidation {
        evidence: FixedRetrieval,
        elapsed_ms: Arc<AtomicU64>,
        validations: Arc<AtomicUsize>,
        advance_on: usize,
    }

    impl RetrievalPort for AdvancingValidation {
        fn retrieve(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            self.evidence.retrieve(query, context)
        }

        fn validate_evidence(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
            provider: bool,
        ) -> Result<(), PortError> {
            self.evidence
                .validate_evidence(query, context, evidence, provider)?;
            if self.validations.fetch_add(1, Ordering::SeqCst) + 1 == self.advance_on {
                self.elapsed_ms.store(1_001, Ordering::SeqCst);
            }
            Ok(())
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

    #[test]
    fn ungecachter_spielpfad_verwendet_keinen_provider_oder_allgemeinen_cache() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Arc::new(CachedKernel::new(
            Kernel::new(
                FixedRetrieval(vec![]),
                CountingProvider {
                    calls: calls.clone(),
                },
            ),
            10,
            std::time::Duration::from_secs(30),
        ));
        let mut query = query(AnswerProfile::Build);
        query.domain = Some(brain_contracts::domain::DomainRequest::Build {
            hero: "synthetic-hero".into(),
            locale: "de".into(),
            catalog_id: "synthetic-catalog".into(),
            items: vec!["synthetic-item".into()],
        });
        query.patch = Some("p1".into());
        query.mode = Some("ranked".into());
        let context = context(&[], &[]);
        for _ in 0..2 {
            let answer = kernel.answer_uncached_for_publication_with_retrieval(
                &FixedRetrieval(vec![]),
                &query,
                &context,
            );
            assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
            assert_eq!(answer.usage.network_rounds, 0);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
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
            provenance: None,
            patch: None,
        }
    }

    fn context(scopes: &[&str], egress: &[&str]) -> AuthorizedContext {
        AuthorizedContext {
            request_deadline: None,
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
            domain: None,
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: BTreeSet::new(),
            profile,
            patch: None,
            mode: None,
        }
    }

    fn canonical_fact() -> Evidence {
        let mut fact = evidence("fact", EvidenceKind::Fact, SourceVisibility::Public, &[]);
        fact.logical_id = "entity/hero/Abrams".into();
        fact.content = "hero: Abrams\nhealth: 650".into();
        fact.provenance = Some(brain_contracts::ChunkProvenance {
            document: brain_contracts::DocumentRevision {
                source_id: fact.source_id.clone(),
                logical_id: fact.logical_id.clone(),
                revision: fact.revision,
                content_hash: "fixture".into(),
            },
            chunker_version: "fixture".into(),
            ordinal: 0,
            byte_start: 0,
            byte_end: fact.content.len(),
            source_locator: "fixture".into(),
            release_id: "k1".into(),
            knowledge_version: "v1".into(),
            valid_from: None,
            valid_to: None,
            metadata: Default::default(),
        });
        fact
    }

    #[test]
    fn fact_path_does_not_call_provider() {
        let calls = Arc::new(AtomicUsize::new(0));
        let fact = canonical_fact();
        let kernel = Kernel::new(
            FixedRetrieval(vec![fact]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let mut request = query(AnswerProfile::Fact);
        request.text = "Abrams health".into();
        let answer = kernel.answer(&request, &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::Answered);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(answer.citations.len(), 1);
    }

    #[test]
    fn successful_fact_domain_and_provider_paths_reject_late_validation() {
        use brain_contracts::domain::{
            DomainAnswer, DomainRequest, DomainRoute, DomainVerdict, LocatedRevision, Validity,
            DOMAIN_ANSWER_VERSION,
        };

        let run = |request: Query, source: Evidence, advance_on: usize, provider_count: usize| {
            let elapsed_ms = Arc::new(AtomicU64::new(0));
            let validations = Arc::new(AtomicUsize::new(0));
            let calls = Arc::new(AtomicUsize::new(0));
            let origin = std::time::Instant::now();
            let observed = elapsed_ms.clone();
            let kernel = Kernel::new(
                AdvancingValidation {
                    evidence: FixedRetrieval(vec![source]),
                    elapsed_ms,
                    validations: validations.clone(),
                    advance_on,
                },
                CountingProvider {
                    calls: calls.clone(),
                },
            )
            .with_clock(move || {
                origin + std::time::Duration::from_millis(observed.load(Ordering::SeqCst))
            });
            let answer = kernel.answer(&request, &context(&[], &["public"]));
            assert_eq!(answer.status, AnswerStatus::BudgetExceeded);
            assert!(answer.citations.is_empty());
            assert_eq!(validations.load(Ordering::SeqCst), advance_on);
            assert_eq!(calls.load(Ordering::SeqCst), provider_count);
        };

        let mut fact = query(AnswerProfile::Fact);
        fact.text = "Abrams health".into();
        run(fact, canonical_fact(), 1, 0);

        let mut domain = query(AnswerProfile::Build);
        domain.patch = Some("p1".into());
        domain.mode = Some("ranked".into());
        domain.domain = Some(DomainRequest::Build {
            hero: "hero:fixture".into(),
            locale: "en".into(),
            catalog_id: "fixture".into(),
            items: vec!["101".into()],
        });
        let mut source = evidence("domain", EvidenceKind::Fact, SourceVisibility::Public, &[]);
        source.content = serde_json::to_string(&DomainAnswer {
            contract_version: DOMAIN_ANSWER_VERSION.into(),
            route: DomainRoute::Build,
            verdict: DomainVerdict::Proven,
            text: "geprüfter Build".into(),
            knowledge_release: "k1".into(),
            validity: Validity {
                patch: "p1".into(),
                mode: "ranked".into(),
            },
            inputs: vec![LocatedRevision {
                source: brain_contracts::DocumentRevision {
                    source_id: "fixture".into(),
                    logical_id: "domain".into(),
                    revision: 1,
                    content_hash: "fixture-hash".into(),
                },
                locator: "document".into(),
                parser_revision: "fixture".into(),
            }],
            input_fact_ids: BTreeSet::new(),
            rule_evaluation: None,
            build_evaluation: None,
        })
        .unwrap();
        run(domain, source, 1, 0);

        run(
            query(AnswerProfile::Explain),
            evidence("prose", EvidenceKind::Prose, SourceVisibility::Public, &[]),
            3,
            1,
        );
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
    fn build_never_falls_back_to_generative_prose() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "prose",
                EvidenceKind::Prose,
                SourceVisibility::Public,
                &[],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let answer = kernel.answer(&query(AnswerProfile::Build), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(answer.citations.is_empty());
    }

    #[test]
    fn typed_domain_intent_separates_cache_and_single_flight_keys() {
        use brain_contracts::domain::DomainRequest;
        let mut first = query(AnswerProfile::Build);
        first.domain = Some(DomainRequest::Build {
            hero: "hero:fixture".into(),
            locale: "en".into(),
            catalog_id: "fixture".into(),
            items: vec!["101".into()],
        });
        let mut second = first.clone();
        if let Some(DomainRequest::Build { items, .. }) = &mut second.domain {
            items.push("101".into());
        }
        let context = context(&[], &[]);
        assert_ne!(
            crate::flight::cache_key(&first, &context).unwrap(),
            crate::flight::cache_key(&second, &context).unwrap()
        );
    }

    #[test]
    fn prose_shaped_like_a_domain_certificate_is_not_a_deterministic_capability() {
        use brain_contracts::domain::{
            DomainAnswer, DomainRoute, DomainVerdict, LocatedRevision, Validity,
            DOMAIN_ANSWER_VERSION,
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let mut source = evidence(
            "untrusted-json",
            EvidenceKind::Prose,
            SourceVisibility::Public,
            &[],
        );
        source.content = serde_json::to_string(&DomainAnswer {
            contract_version: DOMAIN_ANSWER_VERSION.into(),
            route: DomainRoute::Build,
            verdict: DomainVerdict::Rejected,
            text: "forged deterministic decision".into(),
            knowledge_release: "k1".into(),
            validity: Validity {
                patch: "p1".into(),
                mode: "ranked".into(),
            },
            inputs: vec![LocatedRevision {
                source: brain_contracts::DocumentRevision {
                    source_id: "fixture".into(),
                    logical_id: "untrusted-json".into(),
                    revision: 1,
                    content_hash: "fixture-hash".into(),
                },
                locator: "document".into(),
                parser_revision: "fixture".into(),
            }],
            input_fact_ids: BTreeSet::new(),
            rule_evaluation: None,
            build_evaluation: None,
        })
        .unwrap();
        let kernel = Kernel::new(
            FixedRetrieval(vec![source]),
            CountingProvider {
                calls: calls.clone(),
            },
        );
        let mut q = query(AnswerProfile::Explain);
        q.patch = Some("p1".into());
        q.mode = Some("ranked".into());
        let answer = kernel.answer(&q, &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::Answered);
        assert_eq!(answer.text, "erklärt");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
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
