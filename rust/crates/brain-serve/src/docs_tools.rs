use brain_contracts::{
    public_api::valid_public_docs_section_url, tools::SearchDocsRequest, AuthorizedContext,
    Evidence, EvidenceKind, PortError, Query, SourceVisibility, ToolEvidenceDependency,
    ToolExecution, ToolRequest, ToolResult, ToolValidationPurpose, Usage,
};
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;

pub(super) const MAX_BYTES: usize = 16_384;
pub(super) const MAX_SECTIONS: usize = SearchDocsRequest::MAX_RESULTS;
const MAX_EXCERPT_CHARS: usize = 1500;

fn invalid() -> PortError {
    PortError::InvalidResponse("Dokumentenbeleg ist ungültig.".into())
}

pub(super) fn scopes(
    query: &Query,
    context: &AuthorizedContext,
) -> Result<SourceVisibility, PortError> {
    query
        .validate()
        .map_err(|_| PortError::InvalidResponse("Ungültige Dokumentenanfrage.".into()))?;
    context.check_deadline()?;
    if query.conversation_id != context.conversation_id {
        return Err(PortError::PermissionDenied(
            "Dokumentenanfrage ist nicht freigegeben.".into(),
        ));
    }
    let principal = &context.principal;
    if principal.actor_id == "second-brain"
        && principal.channel == "internal"
        && principal.scopes == BTreeSet::from(["second_brain.internal".into()])
        && principal.provider_egress.is_empty()
    {
        return Ok(SourceVisibility::Internal);
    }
    if (principal.scopes.contains("docs.public") || principal.scopes.contains("bot.public"))
        && principal.provider_egress.contains("public")
    {
        return Ok(SourceVisibility::Public);
    }
    Err(PortError::PermissionDenied(
        "Dokumentenanfrage ist nicht freigegeben.".into(),
    ))
}

pub(super) fn validate_evidence_access(
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
    purpose: ToolValidationPurpose,
) -> Result<(), PortError> {
    let allowed = scopes(query, context)?;
    if allowed == SourceVisibility::Internal && purpose != ToolValidationPurpose::Cache {
        return Err(PortError::PermissionDenied(
            "Interne Dokumentenbelege dürfen nicht veröffentlicht oder an einen Modellanbieter gesendet werden.".into(),
        ));
    }
    validate_evidence(evidence)?;
    for item in evidence {
        if item.visibility != allowed
            || item.allowed_scopes.is_empty()
            || !item.allowed_scopes.is_subset(&context.principal.scopes)
            || item.allowed_scopes.iter().any(|scope| match allowed {
                SourceVisibility::Public => scope != "docs.public" && scope != "bot.public",
                SourceVisibility::Internal => scope != "second_brain.internal",
                _ => true,
            })
        {
            return Err(PortError::PermissionDenied(
                "Dokumentenbeleg ist für diesen Zweck nicht freigegeben.".into(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct SectionView {
    pub(super) page: String,
    pub(super) title: String,
    pub(super) heading_path: Vec<String>,
    pub(super) excerpt: String,
    pub(super) excerpt_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) url: Option<String>,
    pub(super) citation: String,
    pub(super) patch: Option<String>,
    pub(super) source_revision: String,
}

pub(super) struct Section {
    value: SectionView,
    evidence: Evidence,
}

impl Section {
    pub(super) fn new(mut value: SectionView, mut evidence: Evidence) -> Result<Self, PortError> {
        validate_evidence(std::slice::from_ref(&evidence))?;
        if !bounded_text(&value.page, 2048)
            || !bounded_text(&value.title, 512)
            || value.heading_path.len() > 16
            || value
                .heading_path
                .iter()
                .any(|heading| !bounded_text(heading, 256))
            || value.source_revision.len() != 40
            || !value
                .source_revision
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
            || value.citation != evidence.evidence_id
            || value.patch != evidence.patch
        {
            return Err(invalid());
        }
        match evidence.visibility {
            SourceVisibility::Public => {
                let Some(url) = &value.url else {
                    return Err(invalid());
                };
                if url != &evidence.citation
                    || !url.starts_with(&format!(
                        "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/{}#L",
                        value.source_revision, value.page
                    ))
                {
                    return Err(invalid());
                }
            }
            SourceVisibility::Internal => {
                if value.url.is_some() || !value.page.starts_with("internal/") {
                    return Err(invalid());
                }
            }
            _ => return Err(invalid()),
        }
        value.excerpt_truncated |= value.excerpt.chars().count() > MAX_EXCERPT_CHARS;
        value.excerpt = value.excerpt.chars().take(MAX_EXCERPT_CHARS).collect();
        evidence.content = serde_json::to_string(&value).map_err(|_| invalid())?;
        Ok(Self { value, evidence })
    }
}

fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
}

fn validate_evidence(evidence: &[Evidence]) -> Result<(), PortError> {
    let mut ids = BTreeSet::new();
    for item in evidence {
        item.validate().map_err(|_| invalid())?;
        if item.kind != EvidenceKind::Prose
            || [&item.evidence_id, &item.source_id, &item.logical_id]
                .into_iter()
                .any(|id| !bounded_text(id, 512))
            || !match item.visibility {
                SourceVisibility::Public => valid_public_docs_section_url(&item.citation),
                SourceVisibility::Internal => {
                    item.citation == format!("receipt/{}", item.evidence_id)
                }
                _ => false,
            }
            || !ids.insert(&item.evidence_id)
        {
            return Err(invalid());
        }
    }
    Ok(())
}

pub(super) fn bounded_evidence(sections: Vec<Section>) -> Result<Vec<Evidence>, PortError> {
    let mut evidence = sections
        .into_iter()
        .map(|section| section.evidence)
        .collect::<Vec<_>>();
    validate_evidence(&evidence)?;
    evidence.truncate(MAX_SECTIONS);
    while serde_json::to_vec(&evidence).map_err(|_| invalid())?.len() > MAX_BYTES {
        if evidence.pop().is_none() {
            return Err(invalid());
        }
    }
    Ok(evidence)
}

pub(super) fn bounded_execution(
    id: &str,
    request: &ToolRequest,
    mut sections: Vec<Section>,
    mut truncated: bool,
) -> Result<ToolExecution, PortError> {
    if !bounded_text(id, 512) {
        return Err(invalid());
    }
    validate_evidence(
        &sections
            .iter()
            .map(|section| section.evidence.clone())
            .collect::<Vec<_>>(),
    )?;
    truncated |= sections.len() > MAX_SECTIONS;
    sections.truncate(MAX_SECTIONS);
    loop {
        let empty = sections.is_empty();
        let mut result = ToolResult {
            call_id: id.into(),
            name: request.name(),
            result: json!({"sections":sections.iter().map(|section| &section.value).collect::<Vec<_>>(),"truncated":truncated}),
            evidence_ids: sections
                .iter()
                .map(|section| section.evidence.evidence_id.clone())
                .collect(),
            is_error: empty,
        };
        if empty {
            result.result["error"] = json!(if truncated {
                "result_too_large"
            } else {
                "no_matching_sections"
            });
        }
        let dependencies = if empty {
            Vec::new()
        } else {
            vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: None,
                evidence: sections
                    .iter()
                    .map(|section| section.evidence.clone())
                    .collect(),
            }]
        };
        let execution = ToolExecution {
            result,
            dependencies,
            usage: Usage::default(),
        };
        let mut worst = execution.clone();
        worst.result.call_id = "\"".repeat(512);
        if serde_json::to_vec(&worst).map_err(|_| invalid())?.len() <= MAX_BYTES {
            return Ok(execution);
        }
        if sections.pop().is_none() {
            return Err(invalid());
        }
        truncated = true;
    }
}

#[cfg(test)]
#[path = "docs_tools_tests.rs"]
mod tests;
