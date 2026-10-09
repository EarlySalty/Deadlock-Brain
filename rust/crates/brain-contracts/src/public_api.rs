//! Public projection: deliberately excludes source paths, ACLs, prompts, usage and provider metadata.
use crate::{AnswerStatus, PortError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
pub const PUBLIC_API_VERSION: &str = "brain.public.v1";
pub const UNVERIFIED_PREFIX: &str = "Ungeprüft: ";
pub const MAX_PUBLIC_ANSWER_TEXT_BYTES: usize = 64 * 1024;
pub const DEADLOCK_API_SOURCE_ID: &str = "deadlock-api-live-v1";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCitation {
    pub citation_id: String,
    pub label: String,
}
impl PublicCitation {
    pub fn from_evidence(evidence: &crate::Evidence, index: usize) -> Self {
        use sha2::{Digest, Sha256};
        Self {
            citation_id: format!("cite-{:x}", Sha256::digest(evidence.evidence_id.as_bytes())),
            label: deadlock_api_label(evidence)
                .unwrap_or_else(|| format!("Beleg {}", index.saturating_add(1))),
        }
    }
}
fn deadlock_api_label(evidence: &crate::Evidence) -> Option<String> {
    if evidence.source_id != DEADLOCK_API_SOURCE_ID
        || evidence.visibility != crate::SourceVisibility::Public
        || !evidence.evidence_id.starts_with("deadlock-api:v1:")
        || evidence.citation.len() > 256
    {
        return None;
    }
    let interval = evidence
        .citation
        .strip_prefix("Deadlock-API: ")?
        .strip_suffix(" (UTC), Ranked")?;
    let (start_text, end_text) = interval.split_once(" bis ")?;
    let format = "%Y-%m-%d %H:%M:%S%:z";
    let start = chrono::DateTime::parse_from_str(start_text, format).ok()?;
    let end = chrono::DateTime::parse_from_str(end_text, format).ok()?;
    if end <= start
        || start.offset().local_minus_utc() != 0
        || end.offset().local_minus_utc() != 0
        || start.format(format).to_string() != start_text
        || end.format(format).to_string() != end_text
    {
        return None;
    }
    Some(format!(
        "Deadlock-API: {} bis {} (UTC), Ranked",
        start.format(format),
        end.format(format)
    ))
}
/// Öffentlich freigegebene Originalbelege ohne interne Quellenadressen oder ACLs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicEvidence {
    pub citation_id: String,
    pub label: String,
    pub text: String,
    pub kind: crate::EvidenceKind,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicRetrievalResponse {
    pub contract_version: String,
    pub request_id: String,
    pub knowledge_release: String,
    pub status: AnswerStatus,
    pub evidence: Vec<PublicEvidence>,
    pub truncated: bool,
    pub out_of_domain: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicAnswerResponse {
    pub contract_version: String,
    pub request_id: String,
    pub knowledge_release: String,
    pub status: AnswerStatus,
    pub text: String,
    pub citations: Vec<PublicCitation>,
}
impl PublicAnswerResponse {
    pub fn validate(&self, expected_request: &str) -> Result<(), PortError> {
        let ids: BTreeSet<_> = self.citations.iter().map(|c| &c.citation_id).collect();
        if self.contract_version != PUBLIC_API_VERSION
            || self.request_id != expected_request
            || self.knowledge_release.trim().is_empty()
            || self.text.len() > MAX_PUBLIC_ANSWER_TEXT_BYTES
            || self.citations.len() > 100
            || ids.len() != self.citations.len()
            || self.citations.iter().any(|c| {
                c.citation_id.is_empty()
                    || c.citation_id.len() > 128
                    || c.label.is_empty()
                    || c.label.len() > 256
            })
            || (matches!(
                self.status,
                AnswerStatus::Answered | AnswerStatus::BuildRejected
            ) && (self.text.trim().is_empty() || self.citations.is_empty()))
        {
            return Err(PortError::InvalidResponse(
                "invalid public answer contract".into(),
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiErrorEnvelope {
    pub error: ApiErrorDetail,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api_evidence() -> crate::Evidence {
        crate::Evidence {
            evidence_id: format!("deadlock-api:v1:1:{}", "0".repeat(64)),
            source_id: DEADLOCK_API_SOURCE_ID.into(),
            logical_id: "private/logical-path".into(),
            revision: 1,
            kind: crate::EvidenceKind::Prose,
            content: "private content".into(),
            citation: "Deadlock-API: 2026-10-06 00:00:00+00:00 bis 2026-10-09 00:00:00+00:00 (UTC), Ranked".into(),
            visibility: crate::SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            provenance: None,
            patch: None,
        }
    }

    #[test]
    fn public_api_citation_preserves_only_validated_source_and_period() {
        let evidence = api_evidence();
        let citation = PublicCitation::from_evidence(&evidence, 0);
        assert_eq!(citation.label, evidence.citation);
        assert_eq!(
            citation.citation_id,
            PublicCitation::from_evidence(&evidence, 5).citation_id
        );
        let output = serde_json::to_string(&citation).unwrap();
        for private in [
            &evidence.evidence_id,
            &evidence.logical_id,
            &evidence.content,
        ] {
            assert!(!output.contains(private.as_str()));
        }
    }

    #[test]
    fn unrecognized_private_or_malformed_source_labels_remain_redacted() {
        let evidence = api_evidence();
        let mut unknown = evidence.clone();
        unknown.source_id = "private/source".into();
        let fallback = PublicCitation::from_evidence(&unknown, 0).label;
        let mut private = evidence.clone();
        private.visibility = crate::SourceVisibility::Private;
        assert_eq!(PublicCitation::from_evidence(&private, 0).label, fallback);
        let mut unrelated = evidence.clone();
        unrelated.evidence_id = "document:1".into();
        assert_eq!(PublicCitation::from_evidence(&unrelated, 0).label, fallback);
        for citation in [
            evidence.citation.replace("2026-10-06", "2026-02-30"),
            evidence.citation.replace("2026-10-06", "2026-10-09"),
            evidence.citation.replace("+00:00", "+02:00"),
            format!("{}\nprivate/source", evidence.citation),
            "Deadlock-API: private/source".into(),
        ] {
            let mut invalid = evidence.clone();
            invalid.citation = citation;
            assert_eq!(PublicCitation::from_evidence(&invalid, 0).label, fallback);
        }
    }
}
