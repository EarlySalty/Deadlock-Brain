//! Public projection: deliberately excludes source paths, ACLs, prompts, usage and provider metadata.
use crate::{AnswerStatus, PortError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
pub const PUBLIC_API_VERSION: &str = "brain.public.v1";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCitation {
    pub citation_id: String,
    pub label: String,
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
            || self.text.len() > 64 * 1024
            || self.citations.len() > 100
            || ids.len() != self.citations.len()
            || self.citations.iter().any(|c| {
                c.citation_id.is_empty()
                    || c.citation_id.len() > 128
                    || c.label.is_empty()
                    || c.label.len() > 256
            })
            || (self.status == AnswerStatus::Answered
                && (self.text.trim().is_empty() || self.citations.is_empty()))
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
