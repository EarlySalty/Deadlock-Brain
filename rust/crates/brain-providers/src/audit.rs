use super::{OpenAiCompatibleProvider, ProviderError, Result};
use brain_contracts::{
    response_audit::{
        diagnostic_bytes, diagnostic_text, ResponseCheck, ResponseDeviation, ResponseDisposition,
    },
    AuthorizedContext, Evidence, Query,
};

pub struct OutputDeviation<'a> {
    pub check: ResponseCheck,
    pub raw_output: &'a [u8],
    pub complete: bool,
    pub disposition: ResponseDisposition,
}

pub(super) fn check(name: &str, field: &str, expected: &str) -> ResponseCheck {
    ResponseCheck {
        check: name.into(),
        field: field.into(),
        expected: expected.into(),
    }
}

impl OpenAiCompatibleProvider {
    pub fn record_response_deviation(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        output: OutputDeviation<'_>,
    ) -> Result<()> {
        let person_id = context.discord.as_ref().and_then(|discord| discord.user_id);
        let project = |value: &str| diagnostic_text(value, person_id);
        let raw = diagnostic_bytes(output.raw_output, person_id);
        let identifiers_redacted = raw != output.raw_output;
        let record = ResponseDeviation {
            request_id: project(&query.request_id),
            model: project(&self.config.model),
            check: ResponseCheck {
                check: project(&output.check.check),
                field: project(&output.check.field),
                expected: project(&output.check.expected),
            },
            raw_output: raw,
            raw_output_complete: output.complete,
            source_ids: evidence
                .iter()
                .map(|item| project(&item.source_id))
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
            evidence_ids: evidence
                .iter()
                .map(|item| project(&item.evidence_id))
                .collect(),
            disposition: output.disposition,
            identifiers_redacted,
        };
        let stored = record.validate().and_then(|()| {
            self.audit
                .as_ref()
                .map_or(Ok(()), |audit| audit.append(&record))
        });
        eprintln!(
            "{}",
            serde_json::json!({
                "event":"response_deviation", "request_id":record.request_id,
                "check":record.check.check, "field":record.check.field,
                "disposition":record.disposition,
                "audit_status": if stored.is_err() { "failed" } else if self.audit.is_some() { "stored" } else { "disabled" }
            })
        );
        stored.map_err(|_| ProviderError::AuditUnavailable)
    }
}
