#![forbid(unsafe_code)]

use brain_contracts::{
    Accounted, AnswerProviderPort, AnswerResponse, AnswerStatus, AuthorizedContext, Evidence,
    PinnedGameContext, PortError, PortFailure, Query, RetrievalPort, ToolDefinition,
    ToolEvidenceDependency, ToolExecutionPort, ToolValidationPurpose, Usage, UsageAccounting,
    CONTRACT_VERSION,
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
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        let value = self.answer(query, context);
        let accounting = UsageAccounting {
            observed: value.usage.clone(),
            unaccounted: true,
            ..UsageAccounting::default()
        };
        Accounted { value, accounting }
    }

    fn answer_for_publication_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        let value = self.answer_for_publication(query, context);
        let accounting = UsageAccounting {
            observed: value.usage.clone(),
            unaccounted: true,
            ..UsageAccounting::default()
        };
        Accounted { value, accounting }
    }

    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse;

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

pub use brain_contracts::tools::GameContextResolver;

#[derive(Clone)]
struct ToolBinding {
    port: std::sync::Arc<dyn ToolExecutionPort>,
    resolver: std::sync::Arc<dyn GameContextResolver>,
    provider_identity: String,
}

#[derive(Clone, Debug)]
struct ToolSession {
    game_context: Option<PinnedGameContext>,
    definitions: Vec<ToolDefinition>,
    provider_identity: String,
}

#[derive(Clone)]
pub struct Kernel<R, P> {
    retrieval: R,
    provider: P,
    tools: Option<ToolBinding>,
    quality_filters: bool,
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
            tools: None,
            quality_filters: true,
            clock: std::sync::Arc::new(std::time::Instant::now),
        }
    }

    pub fn with_quality_filters(mut self, enabled: bool) -> Self {
        self.quality_filters = enabled;
        self
    }

    pub fn with_tools(
        mut self,
        port: impl ToolExecutionPort + 'static,
        resolver: impl GameContextResolver + 'static,
        provider_identity: impl Into<String>,
    ) -> Self {
        self.tools = Some(ToolBinding {
            port: std::sync::Arc::new(port),
            resolver: std::sync::Arc::new(resolver),
            provider_identity: provider_identity.into(),
        });
        self
    }

    fn prepare_tools(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Option<ToolSession>, PortError> {
        if query.domain.is_some() {
            return Ok(None);
        }
        let Some(binding) = &self.tools else {
            return Ok(None);
        };
        context.check_deadline()?;
        if binding.provider_identity.trim().is_empty() {
            return Err(PortError::Unavailable("Provideridentität fehlt".into()));
        }
        let game_context = binding.resolver.resolve(query, context)?;
        context.check_deadline()?;
        if let Some(pin) = &game_context {
            pin.validate()?;
        }
        let definitions = binding
            .port
            .definitions(query, context, game_context.as_ref())?;
        context.check_deadline()?;
        brain_contracts::tools::validate_definitions(&definitions)?;
        Ok(Some(ToolSession {
            game_context,
            definitions,
            provider_identity: binding.provider_identity.clone(),
        }))
    }

    fn validate_tools(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        session: &ToolSession,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
        answer_purpose: AnswerPurpose,
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        let binding = self
            .tools
            .as_ref()
            .ok_or_else(|| PortError::Unavailable("Werkzeuganschluss fehlt".into()))?;
        binding
            .resolver
            .validate(query, context, session.game_context.as_ref())?;
        context.check_deadline()?;
        execution::validate_tool_evidence(query, context, dependencies, purpose)?;
        binding.port.validate_dependencies(
            query,
            context,
            session.game_context.as_ref(),
            dependencies,
            purpose,
        )?;
        context.check_deadline()?;
        if purpose == ToolValidationPurpose::Provider
            && answer_purpose == AnswerPurpose::ExternalPublication
        {
            binding.port.validate_dependencies(
                query,
                context,
                session.game_context.as_ref(),
                dependencies,
                ToolValidationPurpose::Publication,
            )?;
        }
        context.check_deadline()
    }

    fn validate_build_executions(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        session: &ToolSession,
        dependencies: &[ToolEvidenceDependency],
        executions: &[brain_contracts::ToolExecution],
        purpose: AnswerPurpose,
    ) -> Result<(), PortError> {
        if !self.quality_filters {
            return context.check_deadline();
        }
        for execution in executions {
            context.check_deadline()?;
            let pin = session.game_context.as_ref().ok_or_else(|| {
                PortError::InvalidResponse("Spielbindung der Buildprüfung fehlt".into())
            })?;
            let dependency = execution
                .dependencies
                .first()
                .ok_or_else(|| PortError::InvalidResponse("Build-Unteranfrage fehlt".into()))?;
            let brain_contracts::ToolSubrequest::BuildPlan(request) =
                dependency.request.subrequest()
            else {
                return Err(PortError::InvalidResponse(
                    "Build-Unteranfrage fehlt".into(),
                ));
            };
            if execution.result.is_error
                || execution.result.name != brain_contracts::ToolName::BuildPlan
                || execution.result.evidence_ids.is_empty()
                || !execution
                    .dependencies
                    .iter()
                    .all(|dependency| dependencies.contains(dependency))
            {
                return Err(PortError::InvalidResponse(
                    "Build-Ergebnis ist nicht an die Antwortbelege gebunden".into(),
                ));
            }
            execution.validate_for(
                &brain_contracts::ToolCall {
                    id: execution.result.call_id.clone(),
                    name: dependency.request.name(),
                    arguments: dependency.request.arguments().clone(),
                },
                &dependency.request,
                Some(pin),
            )?;
            let binding = self
                .tools
                .as_ref()
                .ok_or_else(|| PortError::Unavailable("Werkzeuganschluss fehlt".into()))?;
            binding
                .port
                .validate_build_plan(query, context, pin, request, execution, purpose)?;
            context.check_deadline()?;
        }
        Ok(())
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
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.answer_with_purpose(query, context, AnswerPurpose::InternalRead)
            .into_accounted()
    }

    fn answer_for_publication_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.answer_with_purpose(query, context, AnswerPurpose::ExternalPublication)
            .into_accounted()
    }

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
    fn answer_with_purpose(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        purpose: AnswerPurpose,
    ) -> KernelAnswer {
        let bound = context.with_request_deadline();
        let context = &bound;
        if query.validate().is_err()
            || query.conversation_id != context.conversation_id
            || !(1..=60_000).contains(&context.deadline_ms)
            || context.knowledge_release.trim().is_empty()
            || !query.requested_scopes.is_subset(&context.principal.scopes)
        {
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

        let session = match self.prepare_tools(query, context) {
            Ok(session) => session,
            Err(error) => {
                return response(
                    query,
                    context,
                    execution::validation_status(&error),
                    "Werkzeugkontext konnte nicht sicher gebunden werden.",
                    Vec::new(),
                    Usage::default(),
                )
                .into();
            }
        };
        self.answer_prepared(query, context, purpose, session.as_ref())
    }

    fn answer_prepared(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        purpose: AnswerPurpose,
        session: Option<&ToolSession>,
    ) -> KernelAnswer {
        if let Some(session) = session.filter(|session| !session.definitions.is_empty()) {
            return execution::answer_tools(self, query, context, session, purpose);
        }
        execution::answer(self, query, context, purpose, self.clock.as_ref())
    }

    fn validate_reuse(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        answer: &KernelAnswer,
        purpose: AnswerPurpose,
        session: Option<&ToolSession>,
    ) -> Result<(), PortError> {
        if !answer.dependencies.is_empty() {
            execution::validate_output(
                &self.retrieval,
                query,
                context,
                &answer.dependencies,
                purpose,
            )?;
        }
        if let Some(session) = session.filter(|session| !session.definitions.is_empty()) {
            self.validate_tools(
                query,
                context,
                session,
                &answer.tool_dependencies,
                ToolValidationPurpose::Cache,
                purpose,
            )?;
            self.validate_tools(
                query,
                context,
                session,
                &answer.tool_dependencies,
                ToolValidationPurpose::Provider,
                purpose,
            )?;
            self.validate_build_executions(
                query,
                context,
                session,
                &answer.tool_dependencies,
                &answer.build_executions,
                purpose,
            )?;
            if answer.answer.status == AnswerStatus::Answered
                && query.profile == brain_contracts::AnswerProfile::Build
                && !execution::has_build_citation(
                    &answer.build_executions,
                    answer
                        .answer
                        .citations
                        .iter()
                        .map(|item| item.evidence_id.as_str()),
                )
            {
                return Err(PortError::Unavailable(
                    "Geprüftes Build-Ergebnis fehlt".into(),
                ));
            }
        } else if !answer.tool_dependencies.is_empty() || !answer.build_executions.is_empty() {
            return Err(PortError::Unavailable(
                "Gebundener Werkzeugkontext fehlt".into(),
            ));
        }
        context.check_deadline()
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
            discord: None,
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
            answer_context: None,
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
    fn relaxed_quality_calls_model_for_empty_knowledge_and_marks_delivery_unverified() {
        for profile in [
            AnswerProfile::Explain,
            AnswerProfile::Fact,
            AnswerProfile::Build,
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            let kernel = Kernel::new(
                FixedRetrieval(Vec::new()),
                CountingProvider {
                    calls: calls.clone(),
                },
            )
            .with_quality_filters(false);
            let answer = kernel.answer(&query(profile), &context(&[], &["public"]));
            assert_eq!(answer.status, AnswerStatus::Unverified);
            assert!(!answer.text.is_empty());
            assert!(answer.citations.is_empty());
            assert_eq!(calls.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn relaxed_quality_does_not_bypass_private_or_internal_egress() {
        for (visibility, scopes) in [
            (SourceVisibility::Private, vec!["private"]),
            (SourceVisibility::Internal, vec!["docs.internal"]),
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            let kernel = Kernel::new(
                FixedRetrieval(vec![evidence(
                    "blocked",
                    EvidenceKind::Prose,
                    visibility,
                    &scopes,
                )]),
                CountingProvider {
                    calls: calls.clone(),
                },
            )
            .with_quality_filters(false);
            let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
            assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            assert!(answer.citations.is_empty());
        }
    }

    #[test]
    fn relaxed_quality_without_evidence_still_requires_query_egress() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(Vec::new()),
            CountingProvider {
                calls: calls.clone(),
            },
        )
        .with_quality_filters(false);
        let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &[]));
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn relaxed_quality_rejects_foreign_request_scoped_channel_content() {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "foreign",
                EvidenceKind::Prose,
                SourceVisibility::RequestScoped,
                &["discord.request:foreign"],
            )]),
            CountingProvider {
                calls: calls.clone(),
            },
        )
        .with_quality_filters(false);
        let mut context = context(
            &["discord.request:foreign", "discord.request:own"],
            &["public", "discord_request"],
        );
        context.discord = Some(brain_contracts::DiscordRequestContext {
            user_id: None,
            request_id: "r1".into(),
            scope: "discord.request:own".into(),
            allow_discord_reads: true,
        });
        let answer = kernel.answer(&query(AnswerProfile::Explain), &context);
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(answer.citations.is_empty());
    }

    #[test]
    fn relaxed_tool_quality_allows_uncited_final_but_keeps_private_dependency_checks() {
        let call = find_call("first");
        let (kernel, _) = tool_kernel(std::slice::from_ref(&call), vec![final_turn(&[])]);
        let answer = kernel
            .with_quality_filters(false)
            .answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::Unverified);
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&[])],
        );
        state.lock().unwrap().private_input = true;
        let answer = kernel
            .with_quality_filters(false)
            .answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(state.lock().unwrap().calls, 1);
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

    #[derive(Clone)]
    struct BuildCheck {
        query: Query,
        context: AuthorizedContext,
        pin: PinnedGameContext,
        request: brain_contracts::tools::BuildPlanRequest,
        execution: brain_contracts::ToolExecution,
        purpose: AnswerPurpose,
    }

    #[derive(Default)]
    struct ToolState {
        version: i64,
        resolves: usize,
        calls: usize,
        executions: Vec<(String, brain_contracts::ToolRequest, AuthorizedContext)>,
        invalid_source: bool,
        revoke_cache: bool,
        block_final: Option<Arc<std::sync::Barrier>>,
        deny_publication: bool,
        deny_provider: bool,
        revoke_provider_on_cache: bool,
        validations: Vec<ToolValidationPurpose>,
        private_input: bool,
        internal_input: bool,
        mismatch_result: bool,
        cancel_tool: bool,
        tool_usage: Usage,
        provider_failure: Option<PortFailure>,
        provider_fail_on: usize,
        tool_failure: Option<PortFailure>,
        tool_fail_on: usize,
        contexts: Vec<AuthorizedContext>,
        observed: Vec<brain_contracts::ToolConversation>,
        mutate_final: Option<fn(&mut ToolState)>,
        approve_build: bool,
        executed_builds: Vec<brain_contracts::ToolExecution>,
        build_query: Option<Query>,
        build_checks: Vec<BuildCheck>,
        build_failure: Option<PortError>,
        build_failure_on: usize,
        cancel_build_validation_on: usize,
        build_is_error: bool,
        tool_reservation: Usage,
        required_calls: Vec<brain_contracts::ToolCall>,
    }

    #[derive(Clone)]
    struct TestResolver(Arc<std::sync::Mutex<ToolState>>);

    impl GameContextResolver for TestResolver {
        fn resolve(
            &self,
            _query: &Query,
            context: &AuthorizedContext,
        ) -> Result<Option<PinnedGameContext>, PortError> {
            context.check_deadline()?;
            let mut state = self.0.lock().unwrap();
            state.resolves += 1;
            Ok(Some(PinnedGameContext {
                client_version: state.version,
                language: brain_contracts::tools::ToolLanguage::German,
                mechanic_revision: "fixture-mechanics".into(),
            }))
        }
        fn validate(
            &self,
            _query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
        ) -> Result<(), PortError> {
            context.check_deadline()?;
            if pin.map(|pin| pin.client_version) != Some(self.0.lock().unwrap().version) {
                return Err(PortError::Unavailable("Version hat gewechselt".into()));
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestTools {
        state: Arc<std::sync::Mutex<ToolState>>,
        definitions: Vec<ToolDefinition>,
    }
    impl ToolExecutionPort for TestTools {
        fn required_calls(
            &self,
            _query: &Query,
            context: &AuthorizedContext,
            _pin: Option<&PinnedGameContext>,
        ) -> Result<Vec<brain_contracts::ToolCall>, PortError> {
            context.check_deadline()?;
            Ok(self.state.lock().unwrap().required_calls.clone())
        }

        fn execute_accounted(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            call_id: &str,
            request: &brain_contracts::ToolRequest,
        ) -> Result<Accounted<brain_contracts::ToolExecution>, PortFailure> {
            let mut state = self.state.lock().unwrap();
            if state.executions.len() + 1 == state.tool_fail_on {
                state
                    .executions
                    .push((call_id.into(), request.clone(), context.clone()));
                return Err(state.tool_failure.clone().unwrap());
            }
            drop(state);
            let value = self
                .execute(query, context, pin, call_id, request)
                .map_err(PortFailure::from)?;
            Ok(Accounted {
                accounting: UsageAccounting {
                    observed: value.usage.clone(),
                    reserved: self.state.lock().unwrap().tool_reservation.clone(),
                    unaccounted: false,
                },
                value,
            })
        }

        fn definitions(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            _pin: Option<&PinnedGameContext>,
        ) -> Result<Vec<ToolDefinition>, PortError> {
            Ok(self.definitions.clone())
        }
        fn execute(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            call_id: &str,
            request: &brain_contracts::ToolRequest,
        ) -> Result<brain_contracts::ToolExecution, PortError> {
            context.check_deadline()?;
            let mut state = self.state.lock().unwrap();
            state
                .executions
                .push((call_id.into(), request.clone(), context.clone()));
            if state.cancel_tool {
                context.request_deadline.as_ref().unwrap().cancel();
            }
            let mut uncited = evidence(
                &format!("{call_id}-uncited"),
                EvidenceKind::Fact,
                SourceVisibility::Public,
                &[],
            );
            if state.private_input {
                uncited.visibility = SourceVisibility::Private;
                uncited.allowed_scopes.insert("private".into());
            }
            if state.internal_input {
                uncited.visibility = SourceVisibility::Internal;
                uncited.allowed_scopes.insert("docs.internal".into());
            }
            let execution = brain_contracts::ToolExecution {
                result: brain_contracts::ToolResult {
                    call_id: if state.mismatch_result {
                        "other".into()
                    } else {
                        call_id.into()
                    },
                    name: request.name(),
                    result: serde_json::json!({"status":"fixture"}),
                    evidence_ids: vec![format!("{call_id}-cited")],
                    is_error: state.build_is_error
                        && request.name() == brain_contracts::ToolName::BuildPlan,
                },
                dependencies: vec![ToolEvidenceDependency {
                    request: request.clone(),
                    game_context: if request.name() == brain_contracts::ToolName::ServerKnowledge {
                        None
                    } else {
                        pin.cloned()
                    },
                    evidence: vec![
                        evidence(
                            &format!("{call_id}-cited"),
                            EvidenceKind::Fact,
                            SourceVisibility::Public,
                            &[],
                        ),
                        uncited,
                    ],
                }],
                usage: state.tool_usage.clone(),
            };
            if request.name() == brain_contracts::ToolName::BuildPlan {
                state.executed_builds.push(execution.clone());
                state.build_query = Some(query.clone());
            }
            Ok(execution)
        }
        fn validate_build_plan(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: &PinnedGameContext,
            request: &brain_contracts::tools::BuildPlanRequest,
            execution: &brain_contracts::ToolExecution,
            purpose: AnswerPurpose,
        ) -> Result<(), PortError> {
            context.check_deadline()?;
            let mut state = self.state.lock().unwrap();
            state.build_checks.push(BuildCheck {
                query: query.clone(),
                context: context.clone(),
                pin: pin.clone(),
                request: request.clone(),
                execution: execution.clone(),
                purpose,
            });
            if state.cancel_build_validation_on == state.build_checks.len() {
                context.request_deadline.as_ref().unwrap().cancel();
            }
            if state.build_failure_on == state.build_checks.len() {
                return Err(state.build_failure.clone().unwrap());
            }
            let mut expected_query = state.build_query.clone().unwrap();
            expected_query.request_id = query.request_id.clone();
            if !state.approve_build
                || serde_json::to_value(&expected_query).unwrap()
                    != serde_json::to_value(query).unwrap()
                || !state.executed_builds.contains(execution)
                || execution.dependencies[0].request.subrequest()
                    != &brain_contracts::ToolSubrequest::BuildPlan(request.clone())
                || execution.dependencies[0].game_context.as_ref() != Some(pin)
            {
                return Err(PortError::InvalidResponse("Buildprüfung verweigert".into()));
            }
            Ok(())
        }
        fn validate_dependencies(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            dependencies: &[ToolEvidenceDependency],
            purpose: ToolValidationPurpose,
        ) -> Result<(), PortError> {
            let mut state = self.state.lock().unwrap();
            state.validations.push(purpose);
            if state.revoke_cache && purpose == ToolValidationPurpose::Cache {
                state.invalid_source = true;
            }
            if state.revoke_provider_on_cache && purpose == ToolValidationPurpose::Cache {
                state.deny_provider = true;
            }
            for dependency in dependencies {
                if dependency.game_context.as_ref() != pin
                    && dependency.request.name() != brain_contracts::ToolName::ServerKnowledge
                {
                    return Err(PortError::InvalidResponse("Falsche Spielbindung".into()));
                }
                if state.invalid_source
                    && dependency
                        .evidence
                        .iter()
                        .any(|item| item.evidence_id.ends_with("uncited"))
                {
                    return Err(PortError::PermissionDenied(
                        "Quelle wurde zurückgenommen".into(),
                    ));
                }
            }
            if state.deny_provider
                && purpose == ToolValidationPurpose::Provider
                && dependencies.iter().any(|dependency| {
                    dependency
                        .evidence
                        .iter()
                        .any(|item| item.evidence_id.ends_with("uncited"))
                })
            {
                return Err(PortError::PermissionDenied(
                    "Weitergabe nicht freigegeben".into(),
                ));
            }
            if state.deny_publication
                && purpose == ToolValidationPurpose::Publication
                && !dependencies.is_empty()
            {
                return Err(PortError::PermissionDenied(
                    "Veröffentlichung nicht freigegeben".into(),
                ));
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TurnProvider {
        state: Arc<std::sync::Mutex<ToolState>>,
        turns: Arc<std::sync::Mutex<std::collections::VecDeque<brain_contracts::ProviderTurn>>>,
    }
    impl AnswerProviderPort for TurnProvider {
        fn answer_turn_accounted(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
            definitions: &[ToolDefinition],
            conversation: &brain_contracts::ToolConversation,
        ) -> Result<Accounted<brain_contracts::ProviderTurn>, PortFailure> {
            let mut state = self.state.lock().unwrap();
            if state.calls + 1 == state.provider_fail_on {
                state.calls += 1;
                state.contexts.push(context.clone());
                state.observed.push(conversation.clone());
                return Err(state.provider_failure.clone().unwrap());
            }
            drop(state);
            let value = self
                .answer_turn(query, context, evidence, definitions, conversation)
                .map_err(PortFailure::from)?;
            Ok(Accounted {
                accounting: UsageAccounting::observed(value.usage().clone()),
                value,
            })
        }

        fn answer(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            _evidence: &[Evidence],
        ) -> Result<ProviderAnswer, PortError> {
            panic!("Werkzeugweg darf den Textport nicht verwenden")
        }
        fn answer_turn(
            &self,
            _query: &Query,
            context: &AuthorizedContext,
            initial: &[Evidence],
            _definitions: &[ToolDefinition],
            conversation: &brain_contracts::ToolConversation,
        ) -> Result<brain_contracts::ProviderTurn, PortError> {
            let expected_ids: Vec<_> = conversation
                .messages
                .iter()
                .filter_map(|message| match message {
                    brain_contracts::ToolMessage::ToolResults { results } => Some(results),
                    brain_contracts::ToolMessage::Assistant { .. } => None,
                })
                .flatten()
                .flat_map(|result| {
                    [
                        format!("{}-cited", result.call_id),
                        format!("{}-uncited", result.call_id),
                    ]
                })
                .collect();
            assert_eq!(
                initial
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect::<Vec<_>>(),
                expected_ids
            );
            let turn = self
                .turns
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| PortError::Unavailable("Fixtureturn fehlt".into()))?;
            let mut state = self.state.lock().unwrap();
            state.calls += 1;
            state.contexts.push(context.clone());
            state.observed.push(conversation.clone());
            if matches!(turn, brain_contracts::ProviderTurn::Final { .. }) {
                if let Some(mutate) = state.mutate_final {
                    mutate(&mut state);
                }
            }
            let barrier = state
                .block_final
                .clone()
                .filter(|_| matches!(turn, brain_contracts::ProviderTurn::Final { .. }));
            drop(state);
            if let Some(barrier) = barrier {
                barrier.wait();
                barrier.wait();
            }
            Ok(turn)
        }
    }

    fn closed_schema(value: &serde_json::Value) -> serde_json::Value {
        use serde_json::{json, Value};
        match value {
            Value::Object(fields) => {
                json!({"type":"object","properties":fields.iter().map(|(key,value)| (key.clone(),closed_schema(value))).collect::<serde_json::Map<_,_>>(),"required":fields.keys().collect::<Vec<_>>(),"additionalProperties":false})
            }
            Value::Array(values) => {
                json!({"type":"array","items":values.first().map(closed_schema).unwrap_or(json!({"type":"integer"}))})
            }
            Value::String(_) => json!({"type":"string"}),
            Value::Number(value) if value.is_u64() || value.is_i64() => json!({"type":"integer"}),
            Value::Number(_) => json!({"type":"number"}),
            Value::Null => json!({"type":"null"}),
            Value::Bool(_) => json!({"type":"boolean"}),
        }
    }
    fn tool_call(
        id: &str,
        name: brain_contracts::ToolName,
        arguments: serde_json::Value,
    ) -> brain_contracts::ToolCall {
        brain_contracts::ToolCall {
            id: id.into(),
            name,
            arguments,
        }
    }
    fn find_call(id: &str) -> brain_contracts::ToolCall {
        tool_call(
            id,
            brain_contracts::ToolName::EntityFind,
            serde_json::json!({"query":"Warden","language":"german"}),
        )
    }
    fn build_call(id: &str) -> brain_contracts::ToolCall {
        tool_call(
            id,
            brain_contracts::ToolName::BuildPlan,
            serde_json::json!({"hero_id":1,"playstyle":"weapon","budget":10000}),
        )
    }

    fn tool_turn(calls: Vec<brain_contracts::ToolCall>) -> brain_contracts::ProviderTurn {
        brain_contracts::ProviderTurn::ToolCalls {
            blocks: calls
                .into_iter()
                .map(|call| brain_contracts::ModelBlock::ToolUse { call })
                .collect(),
            finish_reason: brain_contracts::ProviderFinishReason::ToolUse,
            usage: Usage {
                input_tokens: 10,
                output_tokens: 20,
                network_rounds: 1,
                cost_micros: 2,
                ..Usage::default()
            },
        }
    }
    fn final_turn(ids: &[&str]) -> brain_contracts::ProviderTurn {
        ProviderAnswer {
            text: "belegte Fixtureantwort".into(),
            cited_evidence_ids: ids.iter().map(|id| (*id).into()).collect(),
            usage: Usage {
                input_tokens: 30,
                output_tokens: 40,
                network_rounds: 1,
                cost_micros: 3,
                ..Usage::default()
            },
        }
        .into()
    }
    fn tool_kernel(
        calls: &[brain_contracts::ToolCall],
        turns: Vec<brain_contracts::ProviderTurn>,
    ) -> (
        Kernel<FixedRetrieval, TurnProvider>,
        Arc<std::sync::Mutex<ToolState>>,
    ) {
        let state = Arc::new(std::sync::Mutex::new(ToolState {
            version: 6759,
            ..ToolState::default()
        }));
        let mut definitions = Vec::new();
        for call in calls {
            if definitions
                .iter()
                .any(|definition: &ToolDefinition| definition.name == call.name)
            {
                continue;
            }
            definitions.push(ToolDefinition {
                name: call.name,
                description: "Fixturewerkzeug".into(),
                input_schema: closed_schema(&call.arguments),
            });
        }
        let provider = TurnProvider {
            state: state.clone(),
            turns: Arc::new(std::sync::Mutex::new(turns.into())),
        };
        let kernel = Kernel::new(FixedRetrieval(Vec::new()), provider).with_tools(
            TestTools {
                state: state.clone(),
                definitions,
            },
            TestResolver(state.clone()),
            "fixture-provider/config-v1",
        );
        (kernel, state)
    }

    #[test]
    fn discord_tasks_offer_tools_and_complete_the_tool_loop() {
        for purpose in ["bot_task:concierge", "bot_task:faq"] {
            let find = find_call("find");
            let knowledge = tool_call(
                "knowledge",
                brain_contracts::ToolName::ServerKnowledge,
                serde_json::json!({"question":"Wie funktioniert die Urne?"}),
            );
            let (kernel, state) = tool_kernel(
                &[find.clone(), knowledge.clone()],
                vec![
                    tool_turn(vec![find, knowledge]),
                    final_turn(&["find-cited", "knowledge-cited"]),
                ],
            );
            let mut request = query(AnswerProfile::Explain);
            request.text = "Wie funktioniert die Urne?".into();
            request.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            let authorization = context(&[], &["public"]);
            let context = authorization.with_request_deadline();
            let session = kernel.prepare_tools(&request, &context).unwrap().unwrap();
            assert_eq!(session.definitions.len(), 2);
            let answer = kernel.answer_prepared(
                &request,
                &context,
                AnswerPurpose::ExternalPublication,
                Some(&session),
            );
            assert_eq!(answer.answer.status, AnswerStatus::Answered);
            assert_eq!(answer.answer.citations.len(), 2);
            assert_eq!(answer.tool_dependencies.len(), 2);
            let state = state.lock().unwrap();
            assert_eq!(state.resolves, 1);
            assert_eq!(state.calls, 2);
            assert_eq!(state.executions.len(), 2);
            assert_eq!(state.observed[1].messages.len(), 2);
            assert!(state
                .validations
                .contains(&ToolValidationPurpose::Publication));
        }
    }

    #[test]
    fn discord_tasks_do_not_bypass_tool_privacy_or_current_grants() {
        for purpose in ["bot_task:concierge", "bot_task:faq"] {
            for private in [true, false] {
                let call = find_call("first");
                let (kernel, state) = tool_kernel(
                    std::slice::from_ref(&call),
                    vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
                );
                {
                    let mut state = state.lock().unwrap();
                    state.private_input = private;
                    state.deny_provider = !private;
                }
                let mut request = query(AnswerProfile::Explain);
                request.answer_context = Some(brain_contracts::AnswerContext::Discord(
                    brain_contracts::DiscordAnswerContext {
                        purpose: Some(purpose.into()),
                        ..Default::default()
                    },
                ));
                let answer = kernel.answer(&request, &context(&[], &["public"]));
                assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
                assert!(answer.citations.is_empty());
                let state = state.lock().unwrap();
                assert_eq!(state.calls, 1);
                assert_eq!(state.executions.len(), 1);
            }
        }
    }

    #[test]
    fn discord_tasks_keep_the_typed_domain_route_separate() {
        for purpose in ["bot_task:concierge", "bot_task:faq"] {
            let (kernel, state) = tool_kernel(&[find_call("unused")], Vec::new());
            let mut request = query(AnswerProfile::Build);
            request.patch = Some("p1".into());
            request.mode = Some("ranked".into());
            request.domain = Some(brain_contracts::domain::DomainRequest::Build {
                hero: "hero:fixture".into(),
                locale: "en".into(),
                catalog_id: "fixture".into(),
                items: vec!["101".into()],
            });
            request.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            assert!(kernel
                .prepare_tools(&request, &context(&[], &["public"]))
                .unwrap()
                .is_none());
            let state = state.lock().unwrap();
            assert_eq!(state.resolves, 0);
            assert_eq!(state.calls, 0);
            assert!(state.executions.is_empty());
        }
    }

    #[test]
    fn buildschutz_verweigert_gewoehnliche_belege_trotz_builddefinition() {
        for ordinary in [
            find_call("ordinary"),
            tool_call(
                "ordinary",
                brain_contracts::ToolName::EntityProfile,
                serde_json::json!({"entity":{"kind":"hero","id":1},"fields":["weapon_dps"]}),
            ),
            tool_call(
                "ordinary",
                brain_contracts::ToolName::ServerKnowledge,
                serde_json::json!({"question":"Items für Warden?"}),
            ),
        ] {
            let (kernel, state) = tool_kernel(
                &[ordinary.clone(), build_call("unused")],
                vec![tool_turn(vec![ordinary]), final_turn(&["ordinary-cited"])],
            );
            let answer =
                kernel.answer_accounted(&query(AnswerProfile::Build), &context(&[], &["public"]));
            assert_eq!(answer.value.status, AnswerStatus::Unavailable);
            assert!(answer.value.citations.is_empty());
            assert_eq!(answer.accounting.observed.network_rounds, 2);
            assert!(answer.accounting.reserved.input_tokens > 0);
            let state = state.lock().unwrap();
            assert_eq!(state.calls, 2);
            assert!(state.build_checks.is_empty());
        }
    }

    #[test]
    fn buildschutz_alter_port_ohne_pruefanschluss_schliesst_sicher() {
        struct LegacyTools(TestTools);
        impl ToolExecutionPort for LegacyTools {
            fn definitions(
                &self,
                query: &Query,
                context: &AuthorizedContext,
                pin: Option<&PinnedGameContext>,
            ) -> Result<Vec<ToolDefinition>, PortError> {
                self.0.definitions(query, context, pin)
            }
            fn execute(
                &self,
                query: &Query,
                context: &AuthorizedContext,
                pin: Option<&PinnedGameContext>,
                call_id: &str,
                request: &brain_contracts::ToolRequest,
            ) -> Result<brain_contracts::ToolExecution, PortError> {
                self.0.execute(query, context, pin, call_id, request)
            }
            fn validate_dependencies(
                &self,
                query: &Query,
                context: &AuthorizedContext,
                pin: Option<&PinnedGameContext>,
                dependencies: &[ToolEvidenceDependency],
                purpose: ToolValidationPurpose,
            ) -> Result<(), PortError> {
                self.0
                    .validate_dependencies(query, context, pin, dependencies, purpose)
            }
        }
        let call = build_call("build");
        let (mut kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
        );
        state.lock().unwrap().approve_build = true;
        kernel.tools.as_mut().unwrap().port = Arc::new(LegacyTools(TestTools {
            state: state.clone(),
            definitions: vec![ToolDefinition {
                name: call.name,
                description: "Fixturewerkzeug".into(),
                input_schema: closed_schema(&call.arguments),
            }],
        }));
        let answer =
            kernel.answer_accounted(&query(AnswerProfile::Build), &context(&[], &["public"]));
        assert_eq!(answer.value.status, AnswerStatus::Unavailable);
        assert!(answer.value.citations.is_empty());
        assert_eq!(answer.accounting.observed.network_rounds, 1);
        let state = state.lock().unwrap();
        assert_eq!(state.executed_builds.len(), 1);
        assert!(state.build_checks.is_empty());
        assert_eq!(state.calls, 1);
    }

    #[test]
    fn buildschutz_akzeptiert_nur_geprueftes_tatsaechliches_ergebnis_mit_zweck_und_deadline() {
        for purpose in [
            AnswerPurpose::InternalRead,
            AnswerPurpose::ExternalPublication,
        ] {
            let call = build_call("build");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
            );
            {
                let mut state = state.lock().unwrap();
                state.approve_build = true;
                state.internal_input = true;
                state.deny_publication = purpose == AnswerPurpose::InternalRead;
            }
            let request = query(AnswerProfile::Build);
            let outcome = kernel.answer_with_purpose(
                &request,
                &context(&["docs.internal"], &["public", "internal"]),
                purpose,
            );
            assert_eq!(outcome.answer.status, AnswerStatus::Answered);
            assert_eq!(outcome.build_executions.len(), 1);
            assert_eq!(
                outcome.build_executions[0].dependencies[0].evidence.len(),
                2
            );
            let reused = outcome.clone();
            assert!(Arc::ptr_eq(
                &outcome.build_executions,
                &reused.build_executions
            ));
            assert!(outcome.retained_bytes() > std::mem::size_of_val(&outcome));
            assert!(serde_json::to_value(&outcome.answer)
                .unwrap()
                .get("build_executions")
                .is_none());
            let state = state.lock().unwrap();
            assert_eq!(state.build_checks.len(), 2);
            for check in &state.build_checks {
                assert_eq!(check.query.request_id, request.request_id);
                assert_eq!(check.query.profile, AnswerProfile::Build);
                assert_eq!(check.context.principal, state.contexts[0].principal);
                assert_eq!(
                    check.context.request_deadline,
                    state.contexts[0].request_deadline
                );
                assert_eq!(check.pin.client_version, state.version);
                assert_eq!(check.request.hero_id, 1);
                assert_eq!(check.request.budget, Some(10000));
                assert_eq!(check.execution, state.executed_builds[0]);
                assert_eq!(check.purpose, purpose);
                assert!(
                    check.context.budget.max_output_tokens
                        < state.contexts[0].budget.max_output_tokens
                );
            }
        }
    }

    #[test]
    fn buildschutz_fehlerergebnis_und_unzitierter_build_sind_keine_buildantwort() {
        for is_error in [false, true] {
            let build = build_call("build");
            let ordinary = find_call("ordinary");
            let (kernel, state) = tool_kernel(
                &[build.clone(), ordinary.clone()],
                vec![
                    tool_turn(vec![build, ordinary]),
                    final_turn(if is_error {
                        &["build-cited"]
                    } else {
                        &["ordinary-cited"]
                    }),
                ],
            );
            {
                let mut state = state.lock().unwrap();
                state.approve_build = true;
                state.build_is_error = is_error;
            }
            let outcome =
                kernel.answer_accounted(&query(AnswerProfile::Build), &context(&[], &["public"]));
            assert_eq!(outcome.value.status, AnswerStatus::Unavailable);
            assert!(outcome.value.citations.is_empty());
            assert_eq!(outcome.accounting.observed.network_rounds, 2);
            if is_error {
                assert!(state.lock().unwrap().build_checks.is_empty());
            }
        }
    }

    #[test]
    fn buildschutz_wiederverwendung_prueft_ergebnis_anfrage_pin_und_quellen_erneut() {
        let call = build_call("build");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
        );
        state.lock().unwrap().approve_build = true;
        let request = query(AnswerProfile::Build);
        let initial_context = context(&[], &["public"]);
        let context = initial_context.with_request_deadline();
        let session = kernel.prepare_tools(&request, &context).unwrap().unwrap();
        let answer = kernel.answer_prepared(
            &request,
            &context,
            AnswerPurpose::InternalRead,
            Some(&session),
        );
        assert_eq!(answer.answer.status, AnswerStatus::Answered);
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &answer,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_ok());
        let mut changed = answer.clone();
        Arc::make_mut(&mut changed.build_executions)[0]
            .result
            .result = serde_json::json!({"changed":true});
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &changed,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        let mut other_call = call;
        other_call.arguments["hero_id"] = serde_json::json!(2);
        let other_request = other_call.validate(&session.definitions).unwrap();
        let mut changed = answer.clone();
        Arc::make_mut(&mut changed.build_executions)[0].dependencies[0].request =
            other_request.clone();
        Arc::make_mut(&mut changed.tool_dependencies)[0].request = other_request;
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &changed,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        let mut other_query = request.clone();
        other_query.text.push_str(" anderer Held");
        assert!(kernel
            .validate_reuse(
                &other_query,
                &context,
                &answer,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        let mut changed = answer.clone();
        Arc::make_mut(&mut changed.build_executions)[0].dependencies[0]
            .game_context
            .as_mut()
            .unwrap()
            .client_version += 1;
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &changed,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        let mut changed = answer.clone();
        Arc::make_mut(&mut changed.build_executions)[0].dependencies[0].evidence[0].revision += 1;
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &changed,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        state.lock().unwrap().approve_build = false;
        assert!(kernel
            .validate_reuse(
                &request,
                &context,
                &answer,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .is_err());
        assert_eq!(state.lock().unwrap().calls, 2);
    }

    #[test]
    fn buildschutz_cache_prueft_tatsaechlichen_plan_neu_ohne_doppelte_abrechnung() {
        let call = build_call("build");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
        );
        state.lock().unwrap().approve_build = true;
        let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
        let request = query(AnswerProfile::Build);
        let context = context(&[], &["public"]);
        let fresh = cache.answer_accounted(&request, &context);
        assert_eq!(fresh.value.status, AnswerStatus::Answered);
        assert_eq!(fresh.accounting.observed.network_rounds, 2);
        let mut follower = request;
        follower.request_id = "cache-build".into();
        let reused = cache.answer_accounted(&follower, &context);
        assert_eq!(reused.value.status, AnswerStatus::Answered);
        assert_eq!(reused.value.request_id, follower.request_id);
        assert_eq!(reused.accounting, UsageAccounting::default());
        assert_eq!(
            state
                .lock()
                .unwrap()
                .build_checks
                .last()
                .unwrap()
                .query
                .request_id,
            follower.request_id
        );
        state.lock().unwrap().executed_builds[0].result.result =
            serde_json::json!({"changed":true});
        let revoked = cache.answer_accounted(&follower, &context);
        assert_eq!(revoked.value.status, AnswerStatus::Unavailable);
        assert!(revoked.value.citations.is_empty());
        assert_eq!(revoked.accounting, UsageAccounting::default());
        assert_eq!(state.lock().unwrap().calls, 2);
    }

    #[test]
    fn buildschutz_flight_folger_prueft_den_plan_neu_statt_leaderfreigabe_zu_uebernehmen() {
        for refuse in [false, true] {
            let call = build_call("build");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
            );
            let request = query(AnswerProfile::Build);
            let context = context(&[], &["public"]);
            let session = kernel
                .prepare_tools(&request, &context.with_request_deadline())
                .unwrap()
                .unwrap();
            let key = crate::flight::request_key(
                &request,
                &context,
                AnswerPurpose::InternalRead,
                Some(&session),
            )
            .unwrap();
            let barrier = Arc::new(std::sync::Barrier::new(2));
            {
                let mut state = state.lock().unwrap();
                state.approve_build = true;
                state.block_final = Some(barrier.clone());
                if refuse {
                    state.build_failure_on = 3;
                    state.build_failure =
                        Some(PortError::PermissionDenied("Prüfung zurückgenommen".into()));
                }
            }
            let cache = Arc::new(CachedKernel::new(
                kernel,
                8,
                std::time::Duration::from_secs(30),
            ));
            let leader_cache = cache.clone();
            let leader_request = request.clone();
            let leader_context = context.clone();
            let leader = std::thread::spawn(move || {
                leader_cache.answer_accounted(&leader_request, &leader_context)
            });
            barrier.wait();
            let follower_cache = cache.clone();
            let mut follower_request = request;
            follower_request.request_id = "flight-build".into();
            let follower = std::thread::spawn(move || {
                follower_cache.answer_accounted(&follower_request, &context)
            });
            cache.flights.wait_for_waiter(&key);
            barrier.wait();
            let leader = leader.join().unwrap();
            assert_eq!(leader.value.status, AnswerStatus::Answered);
            assert_eq!(leader.accounting.observed.network_rounds, 2);
            let follower = follower.join().unwrap();
            assert_eq!(
                follower.value.status,
                if refuse {
                    AnswerStatus::UnauthorizedEvidence
                } else {
                    AnswerStatus::Answered
                }
            );
            assert_eq!(follower.accounting, UsageAccounting::default());
            assert_eq!(follower.value.request_id, "flight-build");
            if refuse {
                assert!(follower.value.citations.is_empty());
            }
            let state = state.lock().unwrap();
            assert_eq!(state.calls, 2);
            assert_eq!(state.executed_builds.len(), 1);
            assert_eq!(state.build_checks.len(), 3);
        }
    }

    #[test]
    fn buildschutz_frist_rechte_und_portfehler_behalten_kumulierte_reservierte_usage() {
        for (failure, cancel, final_failure, expected) in [
            (
                Some(PortError::PermissionDenied("Prüfung verweigert".into())),
                false,
                false,
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                Some(PortError::Unavailable("Prüfer fehlt".into())),
                false,
                false,
                AnswerStatus::Unavailable,
            ),
            (
                Some(PortError::InvalidResponse("Ergebnis ungültig".into())),
                false,
                true,
                AnswerStatus::Unavailable,
            ),
            (None, true, false, AnswerStatus::BudgetExceeded),
        ] {
            let call = build_call("build");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
            );
            {
                let mut state = state.lock().unwrap();
                state.approve_build = true;
                state.tool_usage = Usage {
                    input_tokens: 5,
                    output_tokens: 3,
                    network_rounds: 1,
                    cost_micros: 3,
                    ..Usage::default()
                };
                state.tool_reservation = Usage {
                    output_tokens: 7,
                    cost_micros: 11,
                    ..Usage::default()
                };
                state.build_failure = failure;
                state.build_failure_on = if state.build_failure.is_some() {
                    if final_failure {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                };
                state.cancel_build_validation_on = usize::from(cancel);
            }
            let outcome =
                kernel.answer_accounted(&query(AnswerProfile::Build), &context(&[], &["public"]));
            assert_eq!(outcome.value.status, expected);
            assert!(outcome.value.citations.is_empty());
            assert_eq!(outcome.value.usage, outcome.accounting.observed);
            assert_eq!(
                outcome.accounting.observed.input_tokens,
                if final_failure { 45 } else { 15 }
            );
            assert_eq!(
                outcome.accounting.observed.output_tokens,
                if final_failure { 63 } else { 23 }
            );
            assert_eq!(
                outcome.accounting.observed.network_rounds,
                if final_failure { 3 } else { 2 }
            );
            assert_eq!(
                outcome.accounting.observed.cost_micros,
                if final_failure { 8 } else { 5 }
            );
            assert_eq!(outcome.accounting.reserved.output_tokens, 7);
            assert_eq!(outcome.accounting.reserved.cost_micros, 11);
            assert!(outcome.accounting.reserved.input_tokens > 0);
            assert!(!outcome.accounting.unaccounted);
            assert_eq!(
                state.lock().unwrap().calls,
                if final_failure { 2 } else { 1 }
            );
        }
    }

    #[test]
    fn buildschutz_prueft_unzitierte_quellen_pin_und_zweck_nach_finalturn() {
        for (mutate, expected) in [
            (
                (|state: &mut ToolState| state.invalid_source = true) as fn(&mut ToolState),
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                (|state: &mut ToolState| state.deny_provider = true) as fn(&mut ToolState),
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                (|state: &mut ToolState| state.deny_publication = true) as fn(&mut ToolState),
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                (|state: &mut ToolState| state.version += 1) as fn(&mut ToolState),
                AnswerStatus::Unavailable,
            ),
        ] {
            let call = build_call("build");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["build-cited"])],
            );
            {
                let mut state = state.lock().unwrap();
                state.approve_build = true;
                state.mutate_final = Some(mutate);
            }
            let outcome = kernel.answer_for_publication_accounted(
                &query(AnswerProfile::Build),
                &context(&[], &["public"]),
            );
            assert_eq!(outcome.value.status, expected);
            assert!(outcome.value.citations.is_empty());
            assert_eq!(outcome.accounting.observed.network_rounds, 2);
            assert!(outcome.accounting.reserved.input_tokens > 0);
            assert_eq!(state.lock().unwrap().calls, 2);
        }
    }

    #[test]
    fn buildschutz_deterministischer_domainweg_umgeht_nicht_den_vertrag_und_bleibt_nutzbar() {
        use brain_contracts::domain::{
            DomainAnswer, DomainRequest, DomainRoute, DomainVerdict, LocatedRevision, Validity,
            DOMAIN_ANSWER_VERSION,
        };
        let call = build_call("build");
        let (mut kernel, state) = tool_kernel(&[call], Vec::new());
        let mut request = query(AnswerProfile::Build);
        request.patch = Some("p1".into());
        request.mode = Some("ranked".into());
        request.domain = Some(DomainRequest::Build {
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
        kernel.retrieval = FixedRetrieval(vec![source]);
        let outcome = kernel.answer_accounted(&request, &context(&[], &["public"]));
        assert_eq!(outcome.value.status, AnswerStatus::Answered);
        assert_eq!(outcome.value.citations.len(), 1);
        assert_eq!(outcome.accounting.observed.network_rounds, 0);
        let state = state.lock().unwrap();
        assert_eq!(state.calls, 0);
        assert_eq!(state.resolves, 0);
        assert!(state.build_checks.is_empty());
    }

    #[test]
    fn required_tool_runs_before_model_and_retains_accounting_and_citation_checks() {
        for cited in [false, true] {
            let call = find_call("required");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![final_turn(if cited { &["required-cited"] } else { &[] })],
            );
            {
                let mut state = state.lock().unwrap();
                state.required_calls = vec![call];
                state.tool_usage.network_rounds = 1;
            }
            let answer = kernel
                .with_quality_filters(false)
                .answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
            assert_eq!(
                answer.value.status,
                if cited {
                    AnswerStatus::Answered
                } else {
                    AnswerStatus::Unavailable
                }
            );
            assert_eq!(answer.accounting.observed.network_rounds, 2);
            let state = state.lock().unwrap();
            assert_eq!(state.calls, 1);
            assert_eq!(state.executions.len(), 1);
            assert_eq!(state.observed[0].messages.len(), 2);
            assert_eq!(state.contexts[0].budget.max_network_rounds, 3);
        }
    }

    #[test]
    fn required_tool_failure_stops_before_provider_and_keeps_observed_usage() {
        let call = find_call("required");
        let (kernel, state) = tool_kernel(std::slice::from_ref(&call), Vec::new());
        {
            let mut state = state.lock().unwrap();
            state.required_calls = vec![call];
            state.tool_fail_on = 1;
            state.tool_failure = Some(PortFailure::accounted(
                PortError::Unavailable("fixture".into()),
                UsageAccounting::observed(Usage {
                    network_rounds: 1,
                    ..Usage::default()
                }),
            ));
        }
        let answer =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.value.status, AnswerStatus::Unavailable);
        assert_eq!(answer.accounting.observed.network_rounds, 1);
        assert_eq!(state.lock().unwrap().calls, 0);
    }

    #[test]
    fn werkzeugloop_beschafft_belege_ohne_initiales_textprofil_und_traegt_alle_unteranfragen() {
        let first = find_call("first");
        let second = tool_call(
            "second",
            brain_contracts::ToolName::GameRules,
            serde_json::json!({"topic":"urn"}),
        );
        let (kernel, state) = tool_kernel(
            &[first.clone(), second.clone()],
            vec![
                tool_turn(vec![first, second]),
                final_turn(&["first-cited", "second-cited"]),
            ],
        );
        let request = query(AnswerProfile::Fact);
        let answer = kernel.answer_with_purpose(
            &request,
            &context(&[], &["public"]),
            AnswerPurpose::ExternalPublication,
        );
        assert_eq!(answer.answer.status, AnswerStatus::Answered);
        assert_eq!(answer.answer.citations.len(), 2);
        assert_eq!(answer.dependencies.len(), 0);
        assert_eq!(answer.tool_dependencies.len(), 2);
        assert_eq!(answer.tool_dependencies[0].evidence.len(), 2);
        assert!(matches!(
            answer.tool_dependencies[1].request.subrequest(),
            brain_contracts::ToolSubrequest::GameRules(_)
        ));
        assert_eq!(answer.answer.usage.network_rounds, 2);
        assert_eq!(answer.answer.usage.input_tokens, 40);
        assert_eq!(answer.answer.usage.output_tokens, 60);
        assert_eq!(answer.answer.usage.cost_micros, 5);
        let state = state.lock().unwrap();
        assert_eq!(state.resolves, 1);
        assert_eq!(state.calls, 2);
        assert_eq!(state.observed[1].messages.len(), 2);
        assert_eq!(
            state.contexts[0].request_deadline,
            state.contexts[1].request_deadline
        );
        for (_, _, executed) in &state.executions {
            assert_eq!(executed.principal, state.contexts[0].principal);
            assert_eq!(executed.knowledge_release, "k1");
            assert_eq!(
                executed.request_deadline,
                state.contexts[0].request_deadline
            );
        }
        assert!(
            state.contexts[1].budget.max_input_tokens < state.contexts[0].budget.max_input_tokens
        );
    }

    #[test]
    fn boonkurven_analytics_und_regeln_erhalten_die_tatsaechliche_erweiterte_unteranfrage() {
        let call = tool_call(
            "compare",
            brain_contracts::ToolName::HeroCompare,
            serde_json::json!({
                "hero_ids":[1,2],
                "metrics":["weapon_dps"],
                "scenario":{"progression":{"kind":"boons","value":20}},
                "boon_range":{"min_boons":0,"max_boons":35},
                "analytics":{"min_average_badge":50,"max_average_badge":70,"min_unix_timestamp":100,"max_unix_timestamp":200}
            }),
        );
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![
                tool_turn(vec![call.clone()]),
                final_turn(&["compare-cited"]),
            ],
        );
        let request = query(AnswerProfile::Explain);
        let answer = kernel.answer_with_purpose(
            &request,
            &context(&[], &["public"]),
            AnswerPurpose::InternalRead,
        );
        assert_eq!(answer.answer.status, AnswerStatus::Answered);
        let brain_contracts::ToolSubrequest::HeroCompare(compare) =
            answer.tool_dependencies[0].request.subrequest()
        else {
            panic!("Typisierte Vergleichsanfrage fehlt")
        };
        assert_eq!(compare.boon_range.as_ref().unwrap().max_boons, 35);
        assert_eq!(
            compare.analytics.as_ref().unwrap().min_average_badge,
            Some(50)
        );
        assert_eq!(
            compare.scenario.progression,
            brain_contracts::tools::ToolProgression::Boons(20)
        );
        assert_eq!(
            state.lock().unwrap().executions[0].1,
            answer.tool_dependencies[0].request
        );
        let reused = answer.clone();
        assert!(Arc::ptr_eq(
            &answer.tool_dependencies,
            &reused.tool_dependencies
        ));
        assert!(answer.retained_bytes() > std::mem::size_of_val(&answer));
        assert!(serde_json::to_value(&answer.answer)
            .unwrap()
            .get("tool_dependencies")
            .is_none());
    }

    #[test]
    fn unbekannte_felder_actor_und_version_werden_vor_ausfuehrung_verweigert() {
        let valid = find_call("first");
        for field in [
            "unknown",
            "actor_id",
            "client_version",
            "url",
            "deadline_ms",
            "scopes",
        ] {
            let mut call = valid.clone();
            call.arguments[field] = serde_json::json!("manipuliert");
            let (kernel, state) =
                tool_kernel(std::slice::from_ref(&valid), vec![tool_turn(vec![call])]);
            let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
            assert_eq!(answer.status, AnswerStatus::Unavailable);
            assert!(state.lock().unwrap().executions.is_empty());
        }
        let call = tool_call(
            "first",
            brain_contracts::ToolName::GameRules,
            serde_json::json!({"topic":"urn"}),
        );
        let (kernel, state) = tool_kernel(&[valid], vec![tool_turn(vec![call])]);
        assert_eq!(
            kernel
                .answer(&query(AnswerProfile::Explain), &context(&[], &["public"]))
                .status,
            AnswerStatus::Unavailable
        );
        assert!(state.lock().unwrap().executions.is_empty());
        assert!(serde_json::from_value::<brain_contracts::ToolCall>(
            serde_json::json!({"id":"x","name":"sql","arguments":{}})
        )
        .is_err());
    }

    #[test]
    fn unzitierter_privater_input_und_falsche_ergebniszuordnung_stoppen_die_folgerunde() {
        for private in [false, true] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            {
                let mut state = state.lock().unwrap();
                state.private_input = private;
                state.mismatch_result = !private;
            }
            let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
            assert_eq!(
                answer.status,
                if private {
                    AnswerStatus::UnauthorizedEvidence
                } else {
                    AnswerStatus::Unavailable
                }
            );
            assert_eq!(state.lock().unwrap().calls, 1);
            assert!(answer.citations.is_empty());
        }
    }

    #[test]
    fn vollstaendige_belege_und_pin_werden_nach_finalturn_erneut_geprueft() {
        for (mutate, expected) in [
            (
                (|state: &mut ToolState| state.invalid_source = true) as fn(&mut ToolState),
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                (|state: &mut ToolState| state.deny_publication = true) as fn(&mut ToolState),
                AnswerStatus::UnauthorizedEvidence,
            ),
            (
                (|state: &mut ToolState| state.version += 1) as fn(&mut ToolState),
                AnswerStatus::Unavailable,
            ),
        ] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            state.lock().unwrap().mutate_final = Some(mutate);
            let answer = kernel
                .answer_for_publication(&query(AnswerProfile::Explain), &context(&[], &["public"]));
            assert_eq!(answer.status, expected);
            assert!(answer.citations.is_empty());
            assert_eq!(state.lock().unwrap().calls, 2);
        }
    }

    #[test]
    fn fehlende_belege_und_unvollstaendige_abschlussgruende_sind_keine_antwort() {
        for turn in [
            final_turn(&[]),
            final_turn(&["missing"]),
            brain_contracts::ProviderTurn::Final {
                answer: ProviderAnswer {
                    text: "abgeschnitten".into(),
                    cited_evidence_ids: vec![],
                    usage: Usage {
                        network_rounds: 1,
                        ..Usage::default()
                    },
                },
                finish_reason: brain_contracts::ProviderFinishReason::MaxTokens,
            },
        ] {
            let call = find_call("first");
            let (kernel, _) = tool_kernel(&[call], vec![turn]);
            assert_ne!(
                kernel
                    .answer(&query(AnswerProfile::Explain), &context(&[], &["public"]))
                    .status,
                AnswerStatus::Answered
            );
        }
    }

    #[test]
    fn wiederholte_call_ids_werden_vor_zweiter_ausfuehrung_abgewiesen() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), tool_turn(vec![call.clone()])],
        );
        assert_eq!(
            kernel
                .answer(&query(AnswerProfile::Explain), &context(&[], &["public"]))
                .status,
            AnswerStatus::Unavailable
        );
        assert_eq!(state.lock().unwrap().executions.len(), 1);
    }

    #[test]
    fn tool_abbruch_behält_die_urspruengliche_deadline_und_verhindert_folgerunden() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        state.lock().unwrap().cancel_tool = true;
        let answer = kernel.answer(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(answer.status, AnswerStatus::BudgetExceeded);
        assert_eq!(state.lock().unwrap().calls, 1);
        assert_eq!(answer.usage.network_rounds, 1);
    }

    #[test]
    fn zweite_runde_retries_und_externe_toolnutzung_teilen_dasselbe_budget() {
        let call = find_call("first");
        for (rounds, tool_rounds, final_rounds, expected_calls) in
            [(1, 0, 1, 1), (2, 1, 1, 1), (2, 0, 2, 2)]
        {
            let mut final_answer = final_turn(&["first-cited"]);
            if let brain_contracts::ProviderTurn::Final { answer, .. } = &mut final_answer {
                answer.usage.network_rounds = final_rounds;
            }
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_answer],
            );
            state.lock().unwrap().tool_usage.network_rounds = tool_rounds;
            let mut context = context(&[], &["public"]);
            context.budget.max_network_rounds = rounds;
            assert_eq!(
                kernel
                    .answer(&query(AnswerProfile::Explain), &context)
                    .status,
                AnswerStatus::BudgetExceeded
            );
            assert_eq!(state.lock().unwrap().calls, expected_calls);
        }
        for (output, cost) in [(59, 50_000), (2_000, 4)] {
            let (kernel, _) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            let mut context = context(&[], &["public"]);
            context.budget.max_output_tokens = output;
            context.budget.max_cost_micros = cost;
            assert_eq!(
                kernel
                    .answer(&query(AnswerProfile::Explain), &context)
                    .status,
                AnswerStatus::BudgetExceeded
            );
        }
    }

    #[test]
    fn wiederholte_modellhistorie_ist_vor_dem_zweiten_versand_vollstaendig_reserviert() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        let request = query(AnswerProfile::Explain);
        let mut context = context(&[], &["public"]);
        let session = kernel
            .prepare_tools(&request, &context.with_request_deadline())
            .unwrap()
            .unwrap();
        let input = [
            brain_contracts::provider_input::ToolWireFormat::Native,
            brain_contracts::provider_input::ToolWireFormat::OpenAiCompatible,
        ]
        .into_iter()
        .map(|format| {
            brain_contracts::provider_input::grounded_turn_input_ceiling(
                &request,
                &[],
                &session.definitions,
                &brain_contracts::ToolConversation::default(),
                format,
            )
            .unwrap()
        })
        .max()
        .unwrap();
        context.budget.max_input_tokens = u32::try_from(input).unwrap();
        assert_eq!(
            kernel.answer(&request, &context).status,
            AnswerStatus::BudgetExceeded
        );
        assert_eq!(state.lock().unwrap().calls, 1);
    }

    #[test]
    fn cache_hit_prueft_unzitierte_quellen_und_grants_und_loest_pin_requestgebunden() {
        for source in [true, false] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
            let request = query(AnswerProfile::Explain);
            let context = context(&[], &["public"]);
            assert_eq!(
                cache.answer_for_publication(&request, &context).status,
                AnswerStatus::Answered
            );
            let mut next = request.clone();
            next.request_id = "second-request".into();
            assert_eq!(
                cache.answer_for_publication(&next, &context).usage,
                Usage::default()
            );
            {
                let mut state = state.lock().unwrap();
                state.invalid_source = source;
                state.deny_publication = !source;
            }
            assert_eq!(
                cache.answer_for_publication(&next, &context).status,
                AnswerStatus::UnauthorizedEvidence
            );
            let state = state.lock().unwrap();
            assert_eq!(state.resolves, 3);
            assert_eq!(state.calls, 2);
        }
    }

    #[test]
    fn single_flight_follower_prueft_vollstaendige_typisierte_toolabhaengigkeiten() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        let request = query(AnswerProfile::Explain);
        let context = context(&[], &["public"]);
        let session = kernel
            .prepare_tools(&request, &context.with_request_deadline())
            .unwrap()
            .unwrap();
        let key = crate::flight::request_key(
            &request,
            &context,
            AnswerPurpose::ExternalPublication,
            Some(&session),
        )
        .unwrap();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        {
            let mut state = state.lock().unwrap();
            state.block_final = Some(barrier.clone());
            state.revoke_cache = true;
        }
        let cache = Arc::new(CachedKernel::new(
            kernel,
            8,
            std::time::Duration::from_secs(30),
        ));
        let leader_cache = cache.clone();
        let leader_request = request.clone();
        let leader_context = context.clone();
        let leader = std::thread::spawn(move || {
            leader_cache.answer_for_publication(&leader_request, &leader_context)
        });
        barrier.wait();
        let follower_cache = cache.clone();
        let mut follower_request = request.clone();
        follower_request.request_id = "follower".into();
        let follower_context = context.clone();
        let follower = std::thread::spawn(move || {
            follower_cache.answer_for_publication(&follower_request, &follower_context)
        });
        cache.flights.wait_for_waiter(&key);
        barrier.wait();
        assert_eq!(leader.join().unwrap().status, AnswerStatus::Answered);
        let follower = follower.join().unwrap();
        assert_eq!(follower.status, AnswerStatus::UnauthorizedEvidence);
        assert!(follower.citations.is_empty());
        assert_eq!(state.lock().unwrap().calls, 2);
        assert_eq!(state.lock().unwrap().executions.len(), 1);
    }

    #[test]
    fn versionswechsel_bildet_vor_cache_und_flight_einen_neuen_schluessel() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![
                tool_turn(vec![call.clone()]),
                final_turn(&["first-cited"]),
                tool_turn(vec![call.clone()]),
                final_turn(&["first-cited"]),
            ],
        );
        let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
        let request = query(AnswerProfile::Explain);
        let context = context(&[], &["public"]);
        assert_eq!(
            cache.answer(&request, &context).status,
            AnswerStatus::Answered
        );
        state.lock().unwrap().version += 1;
        assert_eq!(
            cache.answer(&request, &context).status,
            AnswerStatus::Answered
        );
        assert_eq!(state.lock().unwrap().calls, 4);
        assert_eq!(state.lock().unwrap().resolves, 2);
    }

    #[test]
    fn actor_scopes_provider_mechanik_und_szenario_trennen_wiederverwendung() {
        let call = find_call("first");
        let (kernel, _) = tool_kernel(&[call], Vec::new());
        let request = query(AnswerProfile::Explain);
        let context = context(&[], &["public"]);
        let session = kernel
            .prepare_tools(&request, &context.with_request_deadline())
            .unwrap()
            .unwrap();
        let key = crate::flight::request_key(
            &request,
            &context,
            AnswerPurpose::InternalRead,
            Some(&session),
        )
        .unwrap();
        let mut other = context.clone();
        other.principal.actor_id = "other".into();
        assert_ne!(
            key,
            crate::flight::request_key(
                &request,
                &other,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .unwrap()
        );
        other = context.clone();
        other.principal.scopes.insert("narrower".into());
        assert_ne!(
            key,
            crate::flight::request_key(
                &request,
                &other,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .unwrap()
        );
        let mut other_session = session.clone();
        other_session.provider_identity = "other-provider-config".into();
        assert_ne!(
            key,
            crate::flight::request_key(
                &request,
                &context,
                AnswerPurpose::InternalRead,
                Some(&other_session)
            )
            .unwrap()
        );
        other_session = session.clone();
        other_session
            .game_context
            .as_mut()
            .unwrap()
            .mechanic_revision = "other-mechanics".into();
        assert_ne!(
            key,
            crate::flight::request_key(
                &request,
                &context,
                AnswerPurpose::InternalRead,
                Some(&other_session)
            )
            .unwrap()
        );
        let mut other_request = request.clone();
        other_request.text.push_str(" mit 20 Boons");
        assert_ne!(
            key,
            crate::flight::request_key(
                &other_request,
                &context,
                AnswerPurpose::InternalRead,
                Some(&session)
            )
            .unwrap()
        );
    }

    #[test]
    fn provider_egress_und_publikationsfreigabe_gelten_auch_fuer_unzitierte_toolinputs() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        assert_eq!(
            kernel
                .answer(&query(AnswerProfile::Explain), &context(&[], &[]))
                .status,
            AnswerStatus::UnauthorizedEvidence
        );
        assert_eq!(state.lock().unwrap().calls, 1);
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        state.lock().unwrap().deny_publication = true;
        assert_eq!(
            kernel
                .answer_for_publication(&query(AnswerProfile::Explain), &context(&[], &["public"]))
                .status,
            AnswerStatus::UnauthorizedEvidence
        );
        assert_eq!(state.lock().unwrap().calls, 1);
    }

    #[test]
    fn internal_read_traegt_den_zweck_frisch_ohne_publikationsrecht() {
        for purpose in [
            AnswerPurpose::InternalRead,
            AnswerPurpose::ExternalPublication,
        ] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            {
                let mut state = state.lock().unwrap();
                state.internal_input = true;
                state.deny_publication = true;
            }
            let outcome = kernel.answer_with_purpose(
                &query(AnswerProfile::Explain),
                &context(&["docs.internal"], &["public", "internal"]),
                purpose,
            );
            let state = state.lock().unwrap();
            if purpose == AnswerPurpose::InternalRead {
                assert_eq!(outcome.answer.status, AnswerStatus::Answered);
                assert_eq!(outcome.tool_dependencies[0].evidence.len(), 2);
                assert_eq!(outcome.accounting.observed.network_rounds, 2);
                assert_eq!(state.calls, 2);
                assert!(!state
                    .validations
                    .contains(&ToolValidationPurpose::Publication));
            } else {
                assert_eq!(outcome.answer.status, AnswerStatus::UnauthorizedEvidence);
                assert!(outcome.answer.citations.is_empty());
                assert_eq!(outcome.accounting.observed.network_rounds, 1);
                assert_eq!(state.calls, 1);
                assert!(state
                    .validations
                    .contains(&ToolValidationPurpose::Publication));
            }
        }
    }

    #[test]
    fn internal_read_cache_trennt_publikation_und_prueft_weitergabe_erneut() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![
                tool_turn(vec![call.clone()]),
                final_turn(&["first-cited"]),
                tool_turn(vec![call.clone()]),
            ],
        );
        {
            let mut state = state.lock().unwrap();
            state.internal_input = true;
            state.deny_publication = true;
        }
        let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
        let request = query(AnswerProfile::Explain);
        let context = context(&["docs.internal"], &["public", "internal"]);
        let fresh = cache.answer_accounted(&request, &context);
        assert_eq!(fresh.value.status, AnswerStatus::Answered);
        assert_eq!(fresh.accounting.observed.network_rounds, 2);
        let mut next = request.clone();
        next.request_id = "cache-reader".into();
        let reused = cache.answer_accounted(&next, &context);
        assert_eq!(reused.value.status, AnswerStatus::Answered);
        assert_eq!(reused.value.request_id, next.request_id);
        assert_eq!(reused.accounting, UsageAccounting::default());
        {
            let state = state.lock().unwrap();
            assert_eq!(state.calls, 2);
            assert!(state.validations.contains(&ToolValidationPurpose::Cache));
            assert!(!state
                .validations
                .contains(&ToolValidationPurpose::Publication));
        }
        let publication = cache.answer_for_publication_accounted(&next, &context);
        assert_eq!(publication.value.status, AnswerStatus::UnauthorizedEvidence);
        assert!(publication.value.citations.is_empty());
        assert_eq!(publication.accounting.observed.network_rounds, 1);
        assert_eq!(state.lock().unwrap().calls, 3);
        assert_eq!(cache.answer(&next, &context).status, AnswerStatus::Answered);
        state.lock().unwrap().revoke_provider_on_cache = true;
        let revoked = cache.answer_accounted(&next, &context);
        assert_eq!(revoked.value.status, AnswerStatus::UnauthorizedEvidence);
        assert!(revoked.value.citations.is_empty());
        assert_eq!(revoked.accounting, UsageAccounting::default());
        assert_eq!(state.lock().unwrap().calls, 3);
    }

    #[test]
    fn internal_read_flight_folger_bewahrt_zweck_und_prueft_unzitierte_weitergabe() {
        for revoke in [false, true] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            let request = query(AnswerProfile::Explain);
            let context = context(&["docs.internal"], &["public", "internal"]);
            let session = kernel
                .prepare_tools(&request, &context.with_request_deadline())
                .unwrap()
                .unwrap();
            let key = crate::flight::request_key(
                &request,
                &context,
                AnswerPurpose::InternalRead,
                Some(&session),
            )
            .unwrap();
            assert_ne!(
                key,
                crate::flight::request_key(
                    &request,
                    &context,
                    AnswerPurpose::ExternalPublication,
                    Some(&session),
                )
                .unwrap()
            );
            let barrier = Arc::new(std::sync::Barrier::new(2));
            {
                let mut state = state.lock().unwrap();
                state.internal_input = true;
                state.deny_publication = true;
                state.block_final = Some(barrier.clone());
                state.revoke_provider_on_cache = revoke;
            }
            let cache = Arc::new(CachedKernel::new(
                kernel,
                8,
                std::time::Duration::from_secs(30),
            ));
            let leader_cache = cache.clone();
            let leader_request = request.clone();
            let leader_context = context.clone();
            let leader = std::thread::spawn(move || {
                leader_cache.answer_accounted(&leader_request, &leader_context)
            });
            barrier.wait();
            let follower_cache = cache.clone();
            let mut follower_request = request.clone();
            follower_request.request_id = "internal-follower".into();
            let follower = std::thread::spawn(move || {
                follower_cache.answer_accounted(&follower_request, &context)
            });
            cache.flights.wait_for_waiter(&key);
            barrier.wait();
            let leader = leader.join().unwrap();
            assert_eq!(leader.value.status, AnswerStatus::Answered);
            assert_eq!(leader.accounting.observed.network_rounds, 2);
            let follower = follower.join().unwrap();
            assert_eq!(
                follower.value.status,
                if revoke {
                    AnswerStatus::UnauthorizedEvidence
                } else {
                    AnswerStatus::Answered
                }
            );
            assert_eq!(follower.value.request_id, "internal-follower");
            assert_eq!(follower.accounting, UsageAccounting::default());
            if revoke {
                assert!(follower.value.citations.is_empty());
            }
            let state = state.lock().unwrap();
            assert_eq!(state.calls, 2);
            assert_eq!(state.executions.len(), 1);
            assert!(state.validations.contains(&ToolValidationPurpose::Cache));
            assert!(!state
                .validations
                .contains(&ToolValidationPurpose::Publication));
        }
    }

    #[test]
    fn internal_read_prueft_scope_und_weitergabe_auch_vor_finaler_ausgabe() {
        for (scopes, egress) in [
            (Vec::new(), vec!["public", "internal"]),
            (vec!["docs.internal"], vec!["public"]),
        ] {
            let call = find_call("first");
            let (kernel, state) = tool_kernel(
                std::slice::from_ref(&call),
                vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
            );
            state.lock().unwrap().internal_input = true;
            let result =
                kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&scopes, &egress));
            assert_eq!(result.value.status, AnswerStatus::UnauthorizedEvidence);
            assert!(result.value.citations.is_empty());
            assert_eq!(result.accounting.observed.network_rounds, 1);
            assert_eq!(state.lock().unwrap().calls, 1);
        }
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        state.lock().unwrap().mutate_final = Some(|state| state.deny_provider = true);
        let result =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::UnauthorizedEvidence);
        assert!(result.value.citations.is_empty());
        assert_eq!(result.accounting.observed.network_rounds, 2);
        assert_eq!(state.lock().unwrap().calls, 2);
    }

    #[test]
    fn fehlerabrechnung_provider_kumuliert_vorherigen_turn_tool_und_retries() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()])],
        );
        let failed_usage = Usage {
            input_tokens: 17,
            output_tokens: 7,
            network_rounds: 2,
            cost_micros: 5,
            ..Usage::default()
        };
        {
            let mut state = state.lock().unwrap();
            state.tool_usage = Usage {
                input_tokens: 5,
                output_tokens: 3,
                network_rounds: 1,
                cost_micros: 3,
                ..Usage::default()
            };
            state.provider_fail_on = 2;
            state.provider_failure = Some(PortFailure::accounted(
                PortError::BudgetExceeded,
                UsageAccounting {
                    observed: failed_usage,
                    reserved: Usage {
                        input_tokens: 100,
                        output_tokens: 11,
                        cost_micros: 25,
                        ..Usage::default()
                    },
                    unaccounted: false,
                },
            ));
        }
        let outcome =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(outcome.value.status, AnswerStatus::BudgetExceeded);
        assert_eq!(outcome.value.usage, outcome.accounting.observed);
        assert_eq!(outcome.accounting.observed.input_tokens, 32);
        assert_eq!(outcome.accounting.observed.output_tokens, 30);
        assert_eq!(outcome.accounting.observed.network_rounds, 4);
        assert_eq!(outcome.accounting.observed.cost_micros, 10);
        assert_eq!(outcome.accounting.reserved.output_tokens, 11);
        assert_eq!(outcome.accounting.reserved.cost_micros, 25);
        assert!(!outcome.accounting.unaccounted);
        let state = state.lock().unwrap();
        assert_eq!(state.calls, 2);
        assert_eq!(state.executions.len(), 1);
        assert_eq!(
            state.contexts[0].request_deadline,
            state.contexts[1].request_deadline
        );
        assert_eq!(state.contexts[1].budget.max_network_rounds, 2);
        assert_eq!(state.contexts[1].budget.max_output_tokens, 1977);
        assert_eq!(state.contexts[1].budget.max_cost_micros, 49995);
        assert!(outcome.value.citations.is_empty());
    }

    #[test]
    fn fehlerabrechnung_spaeterer_toolcall_behaelt_alle_bisherigen_erfolge() {
        let calls = vec![find_call("first"), find_call("second"), find_call("third")];
        let (kernel, state) = tool_kernel(&calls, vec![tool_turn(calls.clone())]);
        {
            let mut state = state.lock().unwrap();
            state.tool_usage = Usage {
                input_tokens: 5,
                output_tokens: 3,
                network_rounds: 1,
                cost_micros: 3,
                ..Usage::default()
            };
            state.tool_fail_on = 2;
            state.tool_failure = Some(PortFailure::accounted(
                PortError::PermissionDenied("fixture".into()),
                UsageAccounting::observed(Usage {
                    input_tokens: 7,
                    output_tokens: 2,
                    network_rounds: 1,
                    cost_micros: 4,
                    ..Usage::default()
                }),
            ));
        }
        let result =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::UnauthorizedEvidence);
        assert_eq!(result.value.usage.input_tokens, 22);
        assert_eq!(result.value.usage.output_tokens, 25);
        assert_eq!(result.value.usage.network_rounds, 3);
        assert_eq!(result.value.usage.cost_micros, 9);
        assert!(!result.accounting.unaccounted);
        let state = state.lock().unwrap();
        assert_eq!(state.calls, 1);
        assert_eq!(
            state
                .executions
                .iter()
                .map(|entry| entry.0.as_str())
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
        assert_eq!(
            state.executions[1].2.request_deadline,
            state.contexts[0].request_deadline
        );
        assert_eq!(state.executions[1].2.budget.max_network_rounds, 2);
    }

    #[test]
    fn fehlerabrechnung_fehlende_toolabrechnung_reserviert_rest_ohne_nullmessung() {
        let calls = vec![find_call("first"), find_call("second")];
        let (kernel, state) = tool_kernel(&calls, vec![tool_turn(calls.clone())]);
        {
            let mut state = state.lock().unwrap();
            state.tool_fail_on = 2;
            state.tool_failure = Some(PortError::Unavailable("fixture".into()).into());
        }
        let result =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::Unavailable);
        assert!(result.accounting.unaccounted);
        assert_eq!(result.accounting.observed.network_rounds, 1);
        assert_eq!(result.accounting.observed.input_tokens, 10);
        assert_eq!(result.accounting.charged().network_rounds, 4);
        assert_eq!(result.accounting.charged().input_tokens, 12000);
        assert_eq!(result.accounting.charged().output_tokens, 2000);
        assert_eq!(state.lock().unwrap().calls, 1);
    }

    #[test]
    fn fehlerabrechnung_lokale_verweigerung_hat_keinen_erfundenen_verbrauch() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(&[call], vec![]);
        {
            let mut state = state.lock().unwrap();
            state.provider_fail_on = 1;
            state.provider_failure = Some(PortFailure::before_call(PortError::BudgetExceeded));
        }
        let result =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::BudgetExceeded);
        assert_eq!(result.accounting, UsageAccounting::default());
        assert!(state.lock().unwrap().executions.is_empty());
    }

    #[test]
    fn fehlerabrechnung_budgetfehler_nach_zweiter_antwort_verliert_keine_usage() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        let mut context = context(&[], &["public"]);
        context.budget.max_output_tokens = 59;
        let result = kernel.answer_accounted(&query(AnswerProfile::Explain), &context);
        assert_eq!(result.value.status, AnswerStatus::BudgetExceeded);
        assert_eq!(result.accounting.observed.input_tokens, 40);
        assert_eq!(result.accounting.observed.output_tokens, 60);
        assert_eq!(result.accounting.observed.network_rounds, 2);
        assert!(result.accounting.reserved.input_tokens > 0);
        assert_eq!(state.lock().unwrap().calls, 2);
    }

    #[test]
    fn fehlerabrechnung_fehler_werden_nicht_gecached_und_hits_nicht_doppelt_abgerechnet() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()]), final_turn(&["first-cited"])],
        );
        {
            let mut state = state.lock().unwrap();
            state.provider_fail_on = 1;
            state.provider_failure = Some(PortFailure::accounted(
                PortError::InvalidResponse("fixture".into()),
                UsageAccounting::observed(Usage {
                    input_tokens: 7,
                    output_tokens: 2,
                    network_rounds: 1,
                    ..Usage::default()
                }),
            ));
        }
        let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
        let request = query(AnswerProfile::Explain);
        let context = context(&[], &["public"]);
        let failed = cache.answer_accounted(&request, &context);
        assert_eq!(failed.value.status, AnswerStatus::Unavailable);
        assert_eq!(failed.accounting.observed.input_tokens, 7);
        let success = cache.answer_accounted(&request, &context);
        assert_eq!(success.value.status, AnswerStatus::Answered);
        assert_eq!(success.accounting.observed.network_rounds, 2);
        let reused = cache.answer_accounted(&request, &context);
        assert_eq!(reused.value.status, AnswerStatus::Answered);
        assert_eq!(reused.accounting, UsageAccounting::default());
        assert_eq!(state.lock().unwrap().calls, 3);
    }

    #[test]
    fn fehlerabrechnung_textport_behaelt_providerverbrauch_bei_fehler() {
        struct FailingText;
        impl AnswerProviderPort for FailingText {
            fn answer(
                &self,
                _: &Query,
                _: &AuthorizedContext,
                _: &[Evidence],
            ) -> Result<ProviderAnswer, PortError> {
                Err(PortError::Unavailable("fixture".into()))
            }
            fn answer_accounted(
                &self,
                _: &Query,
                _: &AuthorizedContext,
                _: &[Evidence],
            ) -> Result<Accounted<ProviderAnswer>, PortFailure> {
                Err(PortFailure::accounted(
                    PortError::Unavailable("fixture".into()),
                    UsageAccounting::observed(Usage {
                        input_tokens: 17,
                        output_tokens: 7,
                        network_rounds: 2,
                        cost_micros: 5,
                        ..Usage::default()
                    }),
                ))
            }
        }
        let kernel = Kernel::new(
            FixedRetrieval(vec![evidence(
                "public",
                EvidenceKind::Prose,
                SourceVisibility::Public,
                &[],
            )]),
            FailingText,
        );
        let result =
            kernel.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::ProviderError);
        assert_eq!(result.value.usage.input_tokens, 17);
        assert_eq!(result.accounting.observed.network_rounds, 2);
        assert_eq!(result.accounting.observed.output_tokens, 7);
    }

    #[test]
    fn fehlerabrechnung_flight_leader_behaelt_usage_bei_abgebrochener_deadline() {
        let call = find_call("first");
        let (kernel, state) = tool_kernel(
            std::slice::from_ref(&call),
            vec![tool_turn(vec![call.clone()])],
        );
        {
            let mut state = state.lock().unwrap();
            state.cancel_tool = true;
            state.tool_usage = Usage {
                input_tokens: 5,
                output_tokens: 3,
                network_rounds: 1,
                ..Usage::default()
            };
        }
        let cache = CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
        let result =
            cache.answer_accounted(&query(AnswerProfile::Explain), &context(&[], &["public"]));
        assert_eq!(result.value.status, AnswerStatus::BudgetExceeded);
        assert_eq!(result.accounting.observed.network_rounds, 2);
        assert_eq!(result.accounting.observed.input_tokens, 15);
        assert_eq!(result.accounting.observed.output_tokens, 23);
        assert_eq!(state.lock().unwrap().calls, 1);
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
