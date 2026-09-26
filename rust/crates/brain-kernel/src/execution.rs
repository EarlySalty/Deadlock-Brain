use super::*;
use std::{collections::BTreeSet, time::Instant};
/// Infrastructure/corrupt-reader failures are not claims about the caller's permissions.
/// Both categories fail closed, but only an explicit denial is UnauthorizedEvidence.
pub(super) fn validation_status(error: &PortError) -> AnswerStatus {
    match error {
        PortError::PermissionDenied(_) => AnswerStatus::UnauthorizedEvidence,
        PortError::BudgetExceeded => AnswerStatus::BudgetExceeded,
        PortError::Unavailable(_) | PortError::InvalidResponse(_) => AnswerStatus::Unavailable,
    }
}
fn remaining(
    context: &AuthorizedContext,
    usage: &Usage,
    elapsed: u64,
) -> Option<AuthorizedContext> {
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
fn combined(a: &Usage, b: &Usage) -> Option<Usage> {
    Some(Usage {
        provider: b.provider.clone().or_else(|| a.provider.clone()),
        model: b.model.clone().or_else(|| a.model.clone()),
        input_tokens: a.input_tokens.checked_add(b.input_tokens)?,
        output_tokens: a.output_tokens.checked_add(b.output_tokens)?,
        network_rounds: a.network_rounds.checked_add(b.network_rounds)?,
        cost_micros: a.cost_micros.checked_add(b.cost_micros)?,
    })
}
pub(super) fn answer<R: RetrievalPort, P: AnswerProviderPort>(
    retrieval: &R,
    provider: &P,
    query: &Query,
    context: &AuthorizedContext,
) -> AnswerResponse {
    let started = Instant::now();
    let fail = |status, message: &str, usage: Usage| {
        response(query, context, status, message, Vec::new(), usage)
    };
    if context.deadline_ms == 0
        || context.deadline_ms > 60000
        || context.knowledge_release.trim().is_empty()
        || !query.requested_scopes.is_subset(&context.principal.scopes)
    {
        return fail(
            AnswerStatus::UnauthorizedEvidence,
            "Anfragekontext ist ungültig.",
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
    let Some(provider_context) = remaining(
        context,
        &retrieval_usage,
        started.elapsed().as_millis() as u64,
    ) else {
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
    if evidence.is_empty() {
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
    if let Err(error) = retrieval.validate_evidence(query, context, &evidence, false) {
        return fail(
            validation_status(&error),
            "Evidenz konnte nicht sicher bestätigt werden.",
            retrieval_usage,
        );
    }
    // Canonical domain proofs exist only for explicitly typed domain requests.
    // A source document containing lookalike JSON is ordinary source content.
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
                return response(
                    query,
                    context,
                    status,
                    domain.text,
                    evidence,
                    retrieval_usage,
                );
            }
        }
    }
    // No model can certify legality or turn incomplete domain input into a build.
    if matches!(query.profile, brain_contracts::AnswerProfile::Build) {
        return fail(AnswerStatus::InsufficientEvidence, "Keine geprüfte Rule-/Buildantwort vorhanden. Eine deterministische Buildanfrage mit Patch und Modus ist erforderlich.", retrieval_usage);
    }
    if matches!(query.profile, brain_contracts::AnswerProfile::Fact) {
        let Some(fact) = evidence
            .into_iter()
            .find(|item| item.kind == EvidenceKind::Fact)
        else {
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
        return response(
            query,
            context,
            AnswerStatus::Answered,
            fact.content.clone(),
            vec![fact],
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
    let egress = evidence.iter().all(|item| {
        let class = match item.visibility {
            brain_contracts::SourceVisibility::Public => "public",
            brain_contracts::SourceVisibility::Internal => "internal",
            brain_contracts::SourceVisibility::Private => "private",
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
    if let Err(error) = retrieval.validate_evidence(query, context, &evidence, true) {
        return fail(
            validation_status(&error),
            "Provider-Evidenz konnte nicht sicher bestätigt werden.",
            retrieval_usage,
        );
    }
    let Some(provider_context) = remaining(
        context,
        &retrieval_usage,
        started.elapsed().as_millis() as u64,
    ) else {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Request Deadline erreicht.",
            retrieval_usage,
        );
    };
    let answer = match provider.answer(query, &provider_context, &evidence) {
        Ok(answer) => answer,
        Err(PortError::BudgetExceeded) => {
            return fail(
                AnswerStatus::BudgetExceeded,
                "Provider Budget ist ausgeschöpft.",
                retrieval_usage,
            )
        }
        Err(_) => {
            return fail(
                AnswerStatus::ProviderError,
                "Antwortprovider nicht verfügbar.",
                retrieval_usage,
            )
        }
    };
    let Some(usage) = combined(&retrieval_usage, &answer.usage) else {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Nutzungszähler übergelaufen.",
            retrieval_usage,
        );
    };
    if remaining(context, &usage, started.elapsed().as_millis() as u64).is_none() {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Provider überschreitet das Request Budget.",
            usage,
        );
    }
    let ids: BTreeSet<_> = answer.cited_evidence_ids.iter().collect();
    if answer.text.trim().is_empty()
        || answer.text.len() > 64 * 1024
        || ids.is_empty()
        || ids.len() != answer.cited_evidence_ids.len()
        || ids
            .iter()
            .any(|id| !evidence.iter().any(|e| &e.evidence_id == *id))
    {
        return fail(
            AnswerStatus::ProviderError,
            "Antwort enthält ungültige Quellenreferenzen.",
            usage,
        );
    }
    evidence.retain(|item| ids.contains(&item.evidence_id));
    // Re-read canonical content/ACL after the network call; never publish stale authorized data.
    if let Err(error) = retrieval.validate_evidence(query, context, &evidence, false) {
        return fail(
            validation_status(&error),
            "Evidenz konnte nach dem Provider-Aufruf nicht sicher bestätigt werden.",
            usage,
        );
    }
    if started.elapsed().as_millis() >= context.deadline_ms as u128 {
        return fail(
            AnswerStatus::BudgetExceeded,
            "Request Deadline erreicht.",
            usage,
        );
    }
    response(
        query,
        context,
        AnswerStatus::Answered,
        answer.text,
        evidence,
        usage,
    )
}
