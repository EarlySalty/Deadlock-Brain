#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CONTRACT_VERSION: &str = "brain.v1";

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ContractError {
    #[error("request_id fehlt")]
    MissingRequestId,
    #[error("query text fehlt")]
    MissingQueryText,
    #[error("conversation_id fehlt")]
    MissingConversationId,
    #[error("ungueltiger Belegscore")]
    InvalidEvidenceScore,
    #[error("ungueltige Revision")]
    InvalidRevision,
    #[error("ungueltige stabile ID")]
    InvalidStableId,
}

pub type Result<T> = std::result::Result<T, ContractError>;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerProfile {
    Fact,
    #[default]
    Explain,
    Build,
    Coaching,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub request_id: String,
    pub conversation_id: String,
    pub text: String,
    #[serde(default)]
    pub requested_scopes: BTreeSet<String>,
    #[serde(default)]
    pub profile: AnswerProfile,
    #[serde(default)]
    pub patch: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

impl Query {
    pub fn validate(&self) -> Result<()> {
        if self.request_id.trim().is_empty() {
            return Err(ContractError::MissingRequestId);
        }
        if self.conversation_id.trim().is_empty() {
            return Err(ContractError::MissingConversationId);
        }
        if self.text.trim().is_empty() {
            return Err(ContractError::MissingQueryText);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Principal {
    pub actor_id: String,
    pub channel: String,
    pub scopes: BTreeSet<String>,
    #[serde(default)]
    pub provider_egress: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub max_network_rounds: u32,
    pub max_input_tokens: u32,
    pub max_output_tokens: u32,
    pub max_cost_micros: u64,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_network_rounds: 4,
            max_input_tokens: 12_000,
            max_output_tokens: 2_000,
            max_cost_micros: 50_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizedContext {
    pub principal: Principal,
    pub conversation_id: String,
    pub knowledge_release: String,
    pub deadline_ms: u64,
    pub budget: Budget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceVisibility {
    Public,
    Internal,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecordV2 {
    pub source_id: String,
    pub logical_id: String,
    pub revision: u64,
    pub content_hash: String,
    pub content: String,
    pub visibility: SourceVisibility,
    #[serde(default)]
    pub allowed_scopes: BTreeSet<String>,
    #[serde(default)]
    pub tombstone: bool,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl SourceRecordV2 {
    pub fn validate(&self) -> Result<()> {
        if self.source_id.trim().is_empty()
            || self.logical_id.trim().is_empty()
            || self.content_hash.trim().is_empty()
        {
            return Err(ContractError::InvalidStableId);
        }
        if self.revision == 0 {
            return Err(ContractError::InvalidRevision);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentRevision {
    pub logical_id: String,
    pub revision: u64,
    pub content_hash: String,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chunk {
    pub chunk_id: String,
    pub document: DocumentRevision,
    pub ordinal: u32,
    pub text: String,
    #[serde(default)]
    pub neighbor_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Fact,
    Rule,
    Prose,
    Mechanic,
    Population,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub evidence_id: String,
    pub source_id: String,
    pub logical_id: String,
    pub revision: u64,
    pub kind: EvidenceKind,
    pub content: String,
    pub citation: String,
    pub visibility: SourceVisibility,
    #[serde(default)]
    pub allowed_scopes: BTreeSet<String>,
    pub score: f64,
    #[serde(default)]
    pub patch: Option<String>,
}

impl Evidence {
    pub fn validate(&self) -> Result<()> {
        if self.evidence_id.trim().is_empty()
            || self.source_id.trim().is_empty()
            || self.logical_id.trim().is_empty()
        {
            return Err(ContractError::InvalidStableId);
        }
        if self.revision == 0 {
            return Err(ContractError::InvalidRevision);
        }
        if !self.score.is_finite() {
            return Err(ContractError::InvalidEvidenceScore);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Usage {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub network_rounds: u32,
    pub cost_micros: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerStatus {
    Answered,
    InsufficientEvidence,
    UnauthorizedEvidence,
    ProviderError,
    BudgetExceeded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerResponse {
    pub contract_version: String,
    pub request_id: String,
    pub knowledge_release: String,
    pub status: AnswerStatus,
    pub text: String,
    #[serde(default)]
    pub citations: Vec<Evidence>,
    #[serde(default)]
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderAnswer {
    pub text: String,
    #[serde(default)]
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorpusRelease {
    pub release_id: String,
    pub knowledge_version: String,
    pub patch: String,
    pub created_at_epoch: i64,
    #[serde(default)]
    pub source_revisions: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub fact_id: String,
    pub subject_id: String,
    pub key: String,
    pub value: serde_json::Value,
    pub unit: Option<String>,
    pub source_revision: DocumentRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    pub effect_id: String,
    pub subject_id: String,
    pub expression: String,
    pub source_revision: DocumentRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub rule_id: String,
    pub subject_id: String,
    pub expression: String,
    pub source_revision: DocumentRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SynergyEvidence {
    pub synergy_id: String,
    pub left_id: String,
    pub right_id: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroKnowledgeCard {
    pub hero_id: String,
    pub knowledge_release: String,
    pub facts: Vec<Fact>,
    pub rules: Vec<Rule>,
    pub synergies: Vec<SynergyEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayArtifact {
    pub replay_id: String,
    pub source_id: String,
    pub content_hash: String,
    pub schema_revision: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayObservation {
    pub observation_id: String,
    pub replay_id: String,
    pub occurred_at_ms: u64,
    pub kind: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PopulationSlice {
    pub slice_id: String,
    pub patch: String,
    pub available_at_epoch: i64,
    pub sample_size: u64,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaChange {
    pub source_id: String,
    pub schema_revision: String,
    pub previous_revision: Option<String>,
    pub breaking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeExport {
    pub export_id: String,
    pub knowledge_release: String,
    pub patch: String,
    pub source_set_hash: String,
    pub fact_set_hash: String,
    pub rule_set_hash: String,
    pub redaction_profile: String,
    pub approved: bool,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PortError {
    #[error("port nicht verfuegbar: {0}")]
    Unavailable(String),
    #[error("ungueltige Portantwort: {0}")]
    InvalidResponse(String),
    #[error("Portbudget ueberschritten")]
    BudgetExceeded,
}

pub trait RetrievalPort: Send + Sync {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> std::result::Result<Vec<Evidence>, PortError>;
}

pub trait AnswerProviderPort: Send + Sync {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_rejects_empty_identity_fields() {
        let query = Query {
            request_id: String::new(),
            conversation_id: "c1".into(),
            text: "Abrams".into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Fact,
            patch: None,
            mode: None,
        };
        assert_eq!(query.validate(), Err(ContractError::MissingRequestId));
    }

    #[test]
    fn contract_roundtrip_keeps_release_and_scopes() {
        let mut scopes = BTreeSet::new();
        scopes.insert("docs.public".to_string());
        let query = Query {
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: "Was kann Abrams?".into(),
            requested_scopes: scopes,
            profile: AnswerProfile::Explain,
            patch: Some("2026-09-24".into()),
            mode: None,
        };
        let json = serde_json::to_string(&query).unwrap();
        let decoded: Query = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, query);
    }

    #[test]
    fn evidence_rejects_non_finite_score() {
        let evidence = Evidence {
            evidence_id: "e1".into(),
            source_id: "wiki".into(),
            logical_id: "hero/abrams".into(),
            revision: 1,
            kind: EvidenceKind::Fact,
            content: "Abrams".into(),
            citation: "wiki:hero/abrams".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: f64::NAN,
            patch: None,
        };
        assert_eq!(
            evidence.validate(),
            Err(ContractError::InvalidEvidenceScore)
        );
    }
}
