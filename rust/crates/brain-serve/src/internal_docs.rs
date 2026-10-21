use super::docs_tools::{scopes, validate_evidence_access};
use brain_contracts::{
    AuthorizedContext, Evidence, PortError, Query, SourceVisibility, ToolValidationPurpose,
};

pub(super) fn authorize(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
    if scopes(query, context)? != SourceVisibility::Internal {
        return Err(PortError::PermissionDenied(
            "Interne Dokumentenanfrage ist nicht freigegeben.".into(),
        ));
    }
    Ok(())
}

pub(super) fn internal_search_text(query: &Query) -> Result<String, PortError> {
    query
        .validate()
        .map_err(|_| PortError::InvalidResponse("Ungültige Dokumentenanfrage.".into()))?;
    Ok(query
        .text
        .split(|character: char| character.is_whitespace() || character.is_control())
        .filter(|term| !term.is_empty())
        .collect::<Vec<_>>()
        .join(" "))
}

pub(super) fn validate_internal_evidence(
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
    purpose: ToolValidationPurpose,
) -> Result<(), PortError> {
    authorize(query, context)?;
    validate_evidence_access(query, context, evidence, purpose)
}
