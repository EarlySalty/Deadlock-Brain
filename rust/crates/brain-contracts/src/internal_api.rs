//! Privater Operatorvertrag. Dieser Typ ist keine öffentliche Brain-Antwort.
use serde::{Deserialize, Serialize};

pub const INTERNAL_API_VERSION: &str = "brain.internal.operator.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalAudience {
    LocalOperator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalStatus {
    Answered,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InternalExcerpt {
    pub source_id: String,
    pub logical_id: String,
    pub revision: u64,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InternalAnswerResponse {
    pub contract_version: String,
    pub audience: InternalAudience,
    pub request_id: String,
    pub knowledge_release: String,
    pub status: InternalStatus,
    pub excerpts: Vec<InternalExcerpt>,
}

impl InternalAnswerResponse {
    pub fn validate(&self, request_id: &str) -> bool {
        self.contract_version == INTERNAL_API_VERSION
            && self.request_id == request_id
            && !self.knowledge_release.is_empty()
            && !["current", "latest"].contains(&self.knowledge_release.as_str())
            && self.excerpts.len() <= 100
            && (self.status == InternalStatus::Answered) == !self.excerpts.is_empty()
            && self.excerpts.iter().all(|excerpt| {
                !excerpt.source_id.is_empty()
                    && !excerpt.logical_id.is_empty()
                    && excerpt.revision > 0
                    && !excerpt.content.trim().is_empty()
                    && excerpt.content.len() <= 64 * 1024
            })
    }
}
