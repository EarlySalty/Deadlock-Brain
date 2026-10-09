use super::*;
use brain_contracts::{
    provider_input::{grounded_turn_input_ceiling_with_quality, ToolWireFormat},
    EvidenceKind, ModelBlock, ProviderTurn, ToolConversation, ToolMessage,
};
use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};
pub(super) fn validation_status(error: &PortError) -> AnswerStatus {
    match error {
        PortError::PermissionDenied(_) => AnswerStatus::UnauthorizedEvidence,
        PortError::BudgetExceeded => AnswerStatus::BudgetExceeded,
        PortError::Unavailable(_) | PortError::InvalidResponse(_) => AnswerStatus::Unavailable,
    }
}
fn account_failure(
    accounting: &mut UsageAccounting,
    failure: PortFailure,
    context: &AuthorizedContext,
) -> PortError {
    if let Some(reported) = failure.accounting {
        accounting.accumulate(&reported);
    } else {
        accounting.accumulate(&UsageAccounting {
            reserved: Usage {
                input_tokens: u64::from(context.budget.max_input_tokens),
                output_tokens: u64::from(context.budget.max_output_tokens),
                network_rounds: context.budget.max_network_rounds,
                cost_micros: context.budget.max_cost_micros,
                ..Usage::default()
            },
            unaccounted: true,
            ..UsageAccounting::default()
        });
    }
    failure.error
}

fn remaining(
    context: &AuthorizedContext,
    usage: &Usage,
    elapsed: u64,
) -> Option<AuthorizedContext> {
    context.check_deadline().ok()?;
    let mut next = context.clone();
    next.deadline_ms = next.deadline_ms.checked_sub(elapsed).filter(|v| *v > 0)?;
    next.budget.max_network_rounds = next
        .budget
        .max_network_rounds
        .checked_sub(usage.network_rounds)?;
    next.budget.max_input_tokens = next
        .budget
        .max_input_tokens
        .checked_sub(u32::try_from(usage.input_tokens).ok()?)?;
    next.budget.max_output_tokens = next
        .budget
        .max_output_tokens
        .checked_sub(u32::try_from(usage.output_tokens).ok()?)?;
    next.budget.max_cost_micros = next.budget.max_cost_micros.checked_sub(usage.cost_micros)?;
    Some(next)
}
pub(super) fn validate_output<R: RetrievalPort>(
    retrieval: &R,
    query: &Query,
    context: &AuthorizedContext,
    dependencies: &[Evidence],
    purpose: AnswerPurpose,
) -> Result<(), PortError> {
    match purpose {
        AnswerPurpose::InternalRead => {
            retrieval.validate_evidence(query, context, dependencies, false)
        }
        AnswerPurpose::ExternalPublication => {
            retrieval.validate_publication(query, context, dependencies)
        }
    }
}

pub(super) fn validate_tool_evidence(
    query: &Query,
    context: &AuthorizedContext,
    dependencies: &[ToolEvidenceDependency],
    purpose: ToolValidationPurpose,
) -> Result<(), PortError> {
    for item in dependencies
        .iter()
        .flat_map(|dependency| &dependency.evidence)
    {
        item.validate()
            .map_err(|_| PortError::InvalidResponse("Ungültiger Werkzeugbeleg".into()))?;
        if !evidence_allowed(&context.principal, item) {
            return Err(PortError::PermissionDenied(
                "Werkzeugbeleg ist nicht freigegeben".into(),
            ));
        }
        if purpose == ToolValidationPurpose::Provider {
            let class = match item.visibility {
                brain_contracts::SourceVisibility::Public => "public",
                brain_contracts::SourceVisibility::Internal => "internal",
                brain_contracts::SourceVisibility::Private => "private",
                brain_contracts::SourceVisibility::RequestScoped => {
                    if !context.discord.as_ref().is_some_and(|request| {
                        request.request_id == query.request_id
                            && item.allowed_scopes == BTreeSet::from([request.scope.clone()])
                    }) {
                        return Err(PortError::PermissionDenied(
                            "Anfragebindung des Werkzeugbelegs fehlt".into(),
                        ));
                    }
                    "discord_request"
                }
            };
            if !provider_egress_allowed(&context.principal, class) {
                return Err(PortError::PermissionDenied(
                    "Werkzeugbeleg darf nicht an den Provider gehen".into(),
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn has_build_citation<'a>(
    executions: &[brain_contracts::ToolExecution],
    mut cited_ids: impl Iterator<Item = &'a str>,
) -> bool {
    cited_ids.any(|id| {
        executions.iter().any(|execution| {
            execution
                .result
                .evidence_ids
                .iter()
                .any(|evidence_id| evidence_id == id)
        })
    })
}

pub(super) fn answer_tools<R: RetrievalPort, P: AnswerProviderPort>(
    kernel: &Kernel<R, P>,
    query: &Query,
    context: &AuthorizedContext,
    session: &ToolSession,
    purpose: AnswerPurpose,
) -> KernelAnswer {
    let started = (kernel.clock)();
    let elapsed = || {
        (kernel.clock)()
            .saturating_duration_since(started)
            .as_millis() as u64
    };
    let mut accounting = UsageAccounting::default();
    let mut dependencies = Vec::<ToolEvidenceDependency>::new();
    let mut build_executions = Vec::<brain_contracts::ToolExecution>::new();
    let mut conversation = ToolConversation::default();
    let mut evidence = Vec::<Evidence>::new();
    let mut seen_calls = BTreeSet::new();
    let result = (|| -> Result<brain_contracts::ProviderAnswer, PortError> {
        let binding = kernel
            .tools
            .as_ref()
            .ok_or_else(|| PortError::Unavailable("Werkzeuganschluss fehlt".into()))?;
        let required =
            binding
                .port
                .required_calls(query, context, session.game_context.as_ref())?;
        let required_ids: BTreeSet<_> = required.iter().map(|call| call.id.clone()).collect();
        let mut pending = (!required.is_empty()).then_some(required);
        let mut required_evidence = Vec::<Vec<String>>::new();
        loop {
            let next = remaining(context, &accounting.charged(), elapsed())
                .ok_or(PortError::BudgetExceeded)?;
            if next.budget.max_network_rounds == 0 || next.budget.max_output_tokens == 0 {
                return Err(PortError::BudgetExceeded);
            }
            kernel.validate_tools(
                query,
                &next,
                session,
                &dependencies,
                ToolValidationPurpose::Provider,
                purpose,
            )?;
            kernel.validate_build_executions(
                query,
                &next,
                session,
                &dependencies,
                &build_executions,
                purpose,
            )?;
            let turn = if let Some(calls) = pending.take() {
                ProviderTurn::ToolCalls {
                    blocks: calls
                        .into_iter()
                        .map(|call| ModelBlock::ToolUse { call })
                        .collect(),
                    finish_reason: brain_contracts::ProviderFinishReason::ToolUse,
                    usage: Usage::default(),
                }
            } else {
                let input = grounded_turn_input_ceiling_with_quality(
                    query,
                    &evidence,
                    &session.definitions,
                    &conversation,
                    ToolWireFormat::Native,
                    kernel.quality_filters,
                )?
                .max(grounded_turn_input_ceiling_with_quality(
                    query,
                    &evidence,
                    &session.definitions,
                    &conversation,
                    ToolWireFormat::OpenAiCompatible,
                    kernel.quality_filters,
                )?);
                let next = remaining(context, &accounting.charged(), elapsed())
                    .ok_or(PortError::BudgetExceeded)?;
                if input > u64::from(next.budget.max_input_tokens) {
                    return Err(PortError::BudgetExceeded);
                }
                let Accounted {
                    value: turn,
                    accounting: mut turn_accounting,
                } = match kernel.provider.answer_turn_accounted(
                    query,
                    &next,
                    &evidence,
                    &session.definitions,
                    &conversation,
                ) {
                    Ok(turn) => turn,
                    Err(failure) => return Err(account_failure(&mut accounting, failure, &next)),
                };
                let reserved_input =
                    input.checked_mul(u64::from(turn_accounting.observed.network_rounds));
                let charge = turn_accounting.charged();
                turn_accounting.accumulate(&UsageAccounting {
                    reserved: Usage {
                        input_tokens: reserved_input
                            .unwrap_or(u64::MAX)
                            .saturating_sub(charge.input_tokens),
                        ..Usage::default()
                    },
                    unaccounted: reserved_input.is_none(),
                    ..UsageAccounting::default()
                });
                accounting.accumulate(&turn_accounting);
                if accounting.unaccounted {
                    return Err(PortError::BudgetExceeded);
                }
                remaining(context, &accounting.charged(), elapsed())
                    .ok_or(PortError::BudgetExceeded)?;
                if turn.usage().network_rounds == 0 {
                    return Err(PortError::InvalidResponse(
                        "Modellrunde hat keinen Netzwerkzähler".into(),
                    ));
                }
                turn
            };
            turn.validate(&session.definitions)?;
            match turn {
                ProviderTurn::Final { answer, .. } => {
                    let ids: BTreeSet<_> = answer.cited_evidence_ids.iter().collect();
                    let insufficient = answer.text.is_empty() && ids.is_empty();
                    let grounded = !ids.is_empty()
                        && ids.len() == answer.cited_evidence_ids.len()
                        && ids
                            .iter()
                            .all(|id| evidence.iter().any(|item| &item.evidence_id == *id));
                    if answer.text.len() > 64 * 1024
                        || (!insufficient && answer.text.trim().is_empty())
                        || (kernel.quality_filters && !insufficient && !grounded)
                    {
                        return Err(PortError::InvalidResponse(
                            "Ungültige finale Werkzeugantwort".into(),
                        ));
                    }
                    let next = remaining(context, &accounting.charged(), elapsed())
                        .ok_or(PortError::BudgetExceeded)?;
                    kernel.validate_tools(
                        query,
                        &next,
                        session,
                        &dependencies,
                        ToolValidationPurpose::Provider,
                        purpose,
                    )?;
                    kernel.validate_build_executions(
                        query,
                        &next,
                        session,
                        &dependencies,
                        &build_executions,
                        purpose,
                    )?;
                    if !insufficient
                        && required_evidence.iter().any(|required| {
                            !required
                                .iter()
                                .any(|id| answer.cited_evidence_ids.contains(id))
                        })
                    {
                        return Err(PortError::Unavailable(
                            "Pflichtbeleg fehlt in der Antwort.".into(),
                        ));
                    }
                    if kernel.quality_filters
                        && query.profile == brain_contracts::AnswerProfile::Build
                        && !insufficient
                        && !has_build_citation(
                            &build_executions,
                            answer.cited_evidence_ids.iter().map(String::as_str),
                        )
                    {
                        return Err(PortError::Unavailable(
                            "Geprüftes Build-Ergebnis fehlt".into(),
                        ));
                    }
                    remaining(context, &accounting.charged(), elapsed())
                        .ok_or(PortError::BudgetExceeded)?;
                    return Ok(answer);
                }
                ProviderTurn::ToolCalls { blocks, .. } => {
                    let calls = blocks
                        .iter()
                        .filter_map(|block| match block {
                            ModelBlock::ToolUse { call } => Some(call),
                            ModelBlock::Text { .. } => None,
                        })
                        .map(|call| {
                            if !seen_calls.insert(call.id.clone()) {
                                return Err(PortError::InvalidResponse(
                                    "Wiederholte Werkzeugaufruf-ID".into(),
                                ));
                            }
                            call.validate(&session.definitions)
                                .map(|request| (call, request))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let mut results = Vec::new();
                    for (call, request) in calls {
                        let next = remaining(context, &accounting.charged(), elapsed())
                            .ok_or(PortError::BudgetExceeded)?;
                        kernel.validate_tools(
                            query,
                            &next,
                            session,
                            &dependencies,
                            ToolValidationPurpose::Provider,
                            purpose,
                        )?;
                        let next = remaining(context, &accounting.charged(), elapsed())
                            .ok_or(PortError::BudgetExceeded)?;
                        let Accounted {
                            value: execution,
                            accounting: tool_accounting,
                        } = match binding.port.execute_accounted(
                            query,
                            &next,
                            session.game_context.as_ref(),
                            &call.id,
                            &request,
                        ) {
                            Ok(execution) => execution,
                            Err(failure) => {
                                return Err(account_failure(&mut accounting, failure, &next))
                            }
                        };
                        accounting.accumulate(&tool_accounting);
                        if accounting.unaccounted {
                            return Err(PortError::BudgetExceeded);
                        }
                        remaining(context, &accounting.charged(), elapsed())
                            .ok_or(PortError::BudgetExceeded)?;
                        execution.validate_for(call, &request, session.game_context.as_ref())?;
                        if required_ids.contains(&call.id) {
                            if execution.result.is_error || execution.result.evidence_ids.is_empty()
                            {
                                return Err(PortError::Unavailable(
                                    "Pflichtwerkzeug lieferte keinen Beleg.".into(),
                                ));
                            }
                            required_evidence.push(execution.result.evidence_ids.clone());
                        }
                        if request.name() == brain_contracts::ToolName::BuildPlan
                            && !execution.result.is_error
                        {
                            build_executions.push(execution.clone());
                        }
                        dependencies.extend(execution.dependencies);
                        let next = remaining(context, &accounting.charged(), elapsed())
                            .ok_or(PortError::BudgetExceeded)?;
                        kernel.validate_tools(
                            query,
                            &next,
                            session,
                            &dependencies,
                            ToolValidationPurpose::Provider,
                            purpose,
                        )?;
                        for item in dependencies
                            .iter()
                            .flat_map(|dependency| &dependency.evidence)
                        {
                            if let Some(previous) = evidence
                                .iter()
                                .find(|previous| previous.evidence_id == item.evidence_id)
                            {
                                if previous != item {
                                    return Err(PortError::InvalidResponse(
                                        "Werkzeugbeleg-ID ist mehrdeutig".into(),
                                    ));
                                }
                            } else {
                                evidence.push(item.clone());
                            }
                        }
                        results.push(execution.result);
                    }
                    conversation
                        .messages
                        .push(ToolMessage::Assistant { blocks });
                    conversation
                        .messages
                        .push(ToolMessage::ToolResults { results });
                    conversation.validate(&session.definitions)?;
                }
            }
        }
    })();
    let outcome: KernelAnswer = match result {
        Ok(answer) if answer.text.is_empty() && answer.cited_evidence_ids.is_empty() => response(
            query,
            context,
            AnswerStatus::InsufficientEvidence,
            "Keine ausreichenden Belege für diese Antwort.",
            Vec::new(),
            accounting.observed.clone(),
        )
        .into(),
        Ok(answer) => {
            let grounded = !answer.cited_evidence_ids.is_empty()
                && answer
                    .cited_evidence_ids
                    .iter()
                    .collect::<BTreeSet<_>>()
                    .len()
                    == answer.cited_evidence_ids.len()
                && answer
                    .cited_evidence_ids
                    .iter()
                    .all(|id| evidence.iter().any(|item| &item.evidence_id == id));
            let status = if grounded
                && (query.profile != brain_contracts::AnswerProfile::Build
                    || kernel.quality_filters)
            {
                AnswerStatus::Answered
            } else {
                eprintln!(
                    "{}",
                    serde_json::json!({"event":"brain_answer_unverified","request_id":query.request_id,"reason":"tool_answer_unchecked"})
                );
                AnswerStatus::Unverified
            };
            let citations = evidence
                .into_iter()
                .filter(|item| grounded && answer.cited_evidence_ids.contains(&item.evidence_id))
                .collect();
            KernelAnswer {
                answer: response(
                    query,
                    context,
                    status,
                    answer.text,
                    citations,
                    accounting.observed.clone(),
                ),
                accounting: accounting.clone(),
                dependencies: std::sync::Arc::from([]),
                tool_dependencies: dependencies.into(),
                build_executions: build_executions.into(),
            }
        }
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"event":"brain_tool_answer_failed","request_id":query.request_id,"error":error.to_string()})
            );
            response(
                query,
                context,
                validation_status(&error),
                "Werkzeugantwort konnte nicht sicher abgeschlossen werden.",
                Vec::new(),
                accounting.observed.clone(),
            )
            .into()
        }
    };
    outcome.with_accounting(accounting)
}

pub(super) fn answer<R: RetrievalPort, P: AnswerProviderPort>(
    kernel: &Kernel<R, P>,
    query: &Query,
    context: &AuthorizedContext,
    purpose: AnswerPurpose,
    now: &dyn Fn() -> Instant,
) -> KernelAnswer {
    let retrieval = &kernel.retrieval;
    let provider = &kernel.provider;
    let quality_filters = kernel.quality_filters;
    let started = now();
    let elapsed_ms = || now().saturating_duration_since(started).as_millis() as u64;
    let expired = || {
        context.check_deadline().is_err()
            || now().saturating_duration_since(started)
                >= Duration::from_millis(context.deadline_ms)
    };
    let fail = |status, message: &str, usage: Usage| {
        KernelAnswer::from(response(query, context, status, message, Vec::new(), usage))
    };
    let publish = |status, text: String, citations: Vec<Evidence>, usage: Usage| {
        if purpose == AnswerPurpose::ExternalPublication {
            if let Err(error) = validate_output(retrieval, query, context, &citations, purpose) {
                return fail(
                    validation_status(&error),
                    "Publikationsfreigabe konnte nicht sicher bestätigt werden.",
                    usage,
                );
            }
        }
        if expired() {
            fail(
                AnswerStatus::BudgetExceeded,
                "Request Deadline erreicht.",
                usage,
            )
        } else {
            KernelAnswer::from(response(query, context, status, text, citations, usage))
        }
    };
    if context.deadline_ms == 0
        || context.deadline_ms > 600_000
        || context.knowledge_release.trim().is_empty()
        || !query.requested_scopes.is_subset(&context.principal.scopes)
    {
        return fail(
            AnswerStatus::UnauthorizedEvidence,
            "Anfragekontext ist ungültig.",
            Usage::default(),
        );
    }
    if expired() {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Request Deadline erreicht.",
            Usage::default(),
        );
    }
    let (retrieved, retrieval_usage) = match retrieval.retrieve_with_usage(query, context) {
        Ok(result) => result,
        Err(PortError::BudgetExceeded) => {
            return fail(
                AnswerStatus::BudgetExceeded,
                "Retrieval Budget ist ausgeschöpft.",
                Usage::default(),
            )
        }
        Err(error @ (PortError::Unavailable(_) | PortError::PermissionDenied(_))) => {
            return fail(
                validation_status(&error),
                "Evidenz konnte nicht sicher geladen werden.",
                Usage::default(),
            );
        }
        Err(_) => {
            return fail(
                AnswerStatus::InsufficientEvidence,
                "Evidenz konnte nicht geladen werden.",
                Usage::default(),
            )
        }
    };
    let Some(provider_context) = remaining(context, &retrieval_usage, elapsed_ms()) else {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Retrieval überschreitet das Request Budget.",
            retrieval_usage,
        );
    };
    if retrieved.len() > 100 {
        return fail(
            AnswerStatus::InsufficientEvidence,
            "Evidence Pack ist zu groß.",
            retrieval_usage,
        );
    }
    let had_retrieved = !retrieved.is_empty();
    let mut evidence: Vec<_> = retrieved
        .into_iter()
        .filter(|item| item.validate().is_ok() && evidence_allowed(&context.principal, item))
        .collect();
    if evidence.is_empty() && (had_retrieved || quality_filters) {
        return fail(
            if had_retrieved {
                AnswerStatus::UnauthorizedEvidence
            } else {
                AnswerStatus::InsufficientEvidence
            },
            "Keine freigegebene ausreichende Evidenz.",
            retrieval_usage,
        );
    }
    evidence.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.evidence_id.cmp(&b.evidence_id))
    });
    if !evidence.is_empty() {
        if let Err(error) = retrieval.validate_evidence(query, context, &evidence, false) {
            return fail(
                validation_status(&error),
                "Evidenz konnte nicht sicher bestätigt werden.",
                retrieval_usage,
            );
        }
    }
    if expired() {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Request Deadline erreicht.",
            retrieval_usage,
        );
    }
    if matches!(
        &query.domain,
        Some(brain_contracts::domain::DomainRequest::Fact { predicate, .. })
            if predicate.starts_with("analytics.")
    ) {
        if query.profile != brain_contracts::AnswerProfile::Fact
            || query.patch.is_some()
            || query.mode.is_some()
            || evidence.len() != 1
            || evidence[0].patch.is_some()
            || evidence[0].source_id != "deadlock_analytics_api"
            || !matches!(
                evidence[0].kind,
                EvidenceKind::Fact | EvidenceKind::Population
            )
        {
            return fail(
                AnswerStatus::InsufficientEvidence,
                "Keine patchunabhängige Analytics-Beobachtung belegt.",
                retrieval_usage,
            );
        }
        if evidence[0].content.len() > 64 * 1024
            || evidence[0].content.len() as u64 > context.budget.max_output_tokens as u64 * 4
        {
            return fail(
                AnswerStatus::BudgetExceeded,
                "Analytics-Antwort überschreitet das Ausgabelimit.",
                retrieval_usage,
            );
        }
        return publish(
            AnswerStatus::Answered,
            evidence[0].content.clone(),
            evidence,
            retrieval_usage,
        );
    }
    if query.domain.is_some() && evidence.len() == 1 {
        if let Ok(domain) =
            serde_json::from_str::<brain_contracts::domain::DomainAnswer>(&evidence[0].content)
        {
            use brain_contracts::domain::{DomainRoute, DomainVerdict, DOMAIN_ANSWER_VERSION};
            if domain.contract_version != DOMAIN_ANSWER_VERSION
                || domain.knowledge_release != context.knowledge_release
                || query.patch.as_deref() != Some(&domain.validity.patch)
                || query.mode.as_deref() != Some(&domain.validity.mode)
                || domain.inputs.is_empty()
            {
                return fail(
                    AnswerStatus::InsufficientEvidence,
                    "Domainnachweis ist unvollständig.",
                    retrieval_usage,
                );
            }
            if domain.route != DomainRoute::Card {
                if domain.text.len() > 64 * 1024
                    || domain.text.len() as u64 > context.budget.max_output_tokens as u64 * 4
                {
                    return fail(
                        AnswerStatus::BudgetExceeded,
                        "Domainantwort überschreitet das Ausgabelimit.",
                        retrieval_usage,
                    );
                }
                let status = match domain.verdict {
                    DomainVerdict::Proven => AnswerStatus::Answered,
                    DomainVerdict::Rejected if domain.route == DomainRoute::Build => {
                        AnswerStatus::BuildRejected
                    }
                    _ => AnswerStatus::InsufficientEvidence,
                };
                return publish(status, domain.text, evidence, retrieval_usage);
            }
        }
    }
    if quality_filters && matches!(query.profile, brain_contracts::AnswerProfile::Build) {
        return fail(AnswerStatus::InsufficientEvidence, "Keine geprüfte Rule-/Buildantwort vorhanden. Eine deterministische Buildanfrage mit Patch und Modus ist erforderlich.", retrieval_usage);
    }
    if matches!(query.profile, brain_contracts::AnswerProfile::Fact)
        && (quality_filters || super::fact_relevance::select(query, &evidence).is_some())
    {
        let Some(fact) = super::fact_relevance::select(query, &evidence) else {
            return fail(
                AnswerStatus::InsufficientEvidence,
                "Keine geprüfte Faktenantwort vorhanden.",
                retrieval_usage,
            );
        };
        if fact.content.len() > 64 * 1024
            || fact.content.len() as u64 > context.budget.max_output_tokens as u64 * 4
        {
            return fail(
                AnswerStatus::BudgetExceeded,
                "Faktenantwort überschreitet das Ausgabelimit.",
                retrieval_usage,
            );
        }
        return publish(
            AnswerStatus::Answered,
            fact.content.clone(),
            vec![fact.clone()],
            retrieval_usage,
        );
    }
    if provider_context.budget.max_network_rounds == 0
        || provider_context.budget.max_output_tokens == 0
    {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Keine Modellrunde freigegeben.",
            retrieval_usage,
        );
    }
    let egress = (!evidence.is_empty() || provider_egress_allowed(&context.principal, "public"))
        && evidence.iter().all(|item| {
            if item.visibility == brain_contracts::SourceVisibility::RequestScoped
                && !context.discord.as_ref().is_some_and(|request| {
                    request.request_id == query.request_id
                        && item.allowed_scopes
                            == std::collections::BTreeSet::from([request.scope.clone()])
                })
            {
                return false;
            }
            let class = match item.visibility {
                brain_contracts::SourceVisibility::Public => "public",
                brain_contracts::SourceVisibility::Internal => "internal",
                brain_contracts::SourceVisibility::Private => "private",
                brain_contracts::SourceVisibility::RequestScoped => "discord_request",
            };
            provider_egress_allowed(&context.principal, class)
        });
    if !egress {
        return fail(
            AnswerStatus::UnauthorizedEvidence,
            "Provider-Egress ist nicht freigegeben.",
            retrieval_usage,
        );
    }
    if !evidence.is_empty() {
        if let Err(error) = retrieval.validate_evidence(query, context, &evidence, true) {
            return fail(
                validation_status(&error),
                "Provider-Evidenz konnte nicht sicher bestätigt werden.",
                retrieval_usage,
            );
        }
    }
    let Some(provider_context) = remaining(context, &retrieval_usage, elapsed_ms()) else {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Request Deadline erreicht.",
            retrieval_usage,
        );
    };
    let mut accounting = UsageAccounting::observed(retrieval_usage);
    let Accounted {
        value: answer,
        accounting: provider_accounting,
    } = match provider.answer_accounted(query, &provider_context, &evidence) {
        Ok(answer) => answer,
        Err(failure) => {
            let error = account_failure(&mut accounting, failure, &provider_context);
            return fail(
                if error == PortError::BudgetExceeded {
                    AnswerStatus::BudgetExceeded
                } else {
                    AnswerStatus::ProviderError
                },
                "Antwortprovider nicht verfügbar.",
                accounting.observed.clone(),
            )
            .with_accounting(accounting);
        }
    };
    accounting.accumulate(&provider_accounting);
    let fail_provider = |status, message: &str| {
        fail(status, message, accounting.observed.clone()).with_accounting(accounting.clone())
    };
    if accounting.unaccounted || remaining(context, &accounting.charged(), elapsed_ms()).is_none() {
        return fail_provider(
            AnswerStatus::BudgetExceeded,
            "Provider überschreitet das Request Budget.",
        );
    }
    let ids: BTreeSet<_> = answer.cited_evidence_ids.iter().collect();
    let insufficient = answer.text.is_empty() && ids.is_empty();
    let grounded = !ids.is_empty()
        && ids.len() == answer.cited_evidence_ids.len()
        && ids
            .iter()
            .all(|id| evidence.iter().any(|e| &e.evidence_id == *id));
    if answer.text.len() > 64 * 1024
        || (!insufficient && answer.text.trim().is_empty())
        || (quality_filters && !insufficient && !grounded)
    {
        return fail_provider(
            AnswerStatus::ProviderError,
            "Antwort enthält ungültige Quellenreferenzen.",
        );
    }
    if !evidence.is_empty() {
        if let Err(error) = validate_output(retrieval, query, context, &evidence, purpose) {
            return fail_provider(
                validation_status(&error),
                "Evidenz konnte nach dem Provider-Aufruf nicht sicher bestätigt werden.",
            );
        }
    }
    if expired() {
        return fail_provider(AnswerStatus::BudgetExceeded, "Request Deadline erreicht.");
    }
    if insufficient {
        return fail_provider(
            AnswerStatus::InsufficientEvidence,
            "Dazu hab ich gerade nichts Genaues, frag am besten direkt im Discord nach.",
        );
    }
    let status = if grounded && query.profile != brain_contracts::AnswerProfile::Build {
        AnswerStatus::Answered
    } else {
        eprintln!(
            "{}",
            serde_json::json!({"event":"brain_answer_unverified","request_id":query.request_id,"reason":if evidence.is_empty() {"no_evidence"} else if !grounded {"citation_invalid"} else {"build_unchecked"}})
        );
        AnswerStatus::Unverified
    };
    let citations = evidence
        .iter()
        .filter(|item| grounded && ids.contains(&item.evidence_id))
        .cloned()
        .collect();
    KernelAnswer {
        answer: response(
            query,
            context,
            status,
            answer.text,
            citations,
            accounting.observed.clone(),
        ),
        accounting,
        dependencies: evidence.into(),
        tool_dependencies: std::sync::Arc::from([]),
        build_executions: std::sync::Arc::from([]),
    }
}
