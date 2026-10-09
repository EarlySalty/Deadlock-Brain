#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CONTRACT_VERSION: &str = "brain.v1";
mod deadline;
pub use deadline::RequestDeadline;
pub mod discord_task;
pub mod domain;
pub mod domain_knowledge;
pub mod embedding;
pub mod entity_profile;
pub mod external;
pub mod feeds;
pub mod internal_api;
pub mod invite;
pub mod lexical;
pub mod maintenance;
pub mod postgres;
pub mod provider_input;
pub mod public_api;
pub mod read_manifest;
pub mod replay;
pub mod response_audit;
pub mod retrieval;
pub use read_manifest::{DocumentDescriptor, ReleaseReadManifest};
pub mod source;
pub use retrieval::{ChunkProvenance, DocumentHead};
pub mod store;
pub mod tools;
pub use tools::{
    ModelBlock, PinnedGameContext, ProviderFinishReason, ProviderTurn, ToolCall, ToolConversation,
    ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolMessage,
    ToolName, ToolRequest, ToolResult, ToolSubrequest, ToolValidationPurpose,
};
pub mod value;
pub mod wiki;
pub use embedding::{EmbeddingIdentity, EmbeddingOutput, EmbeddingProviderPort};
pub use public_api::{
    ApiErrorDetail, ApiErrorEnvelope, PublicAnswerResponse, PublicCitation, PUBLIC_API_VERSION,
};
pub use store::{
    BatchReceipt, CorpusSnapshot, DocumentStorePort, Lease, SnapshotReadPort, SourceBatch,
    SourceCheckpoint, StoreFuture,
};

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
    #[error("Contract Größenlimit überschritten")]
    LimitExceeded,
    #[error("Widersprüchlicher Antwortort")]
    InvalidAnswerContext,
    #[error("Anfragegebundene Inhalte dürfen nicht als Wissensquelle gespeichert werden")]
    RequestScopedSource,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerInputKind {
    Message,
    Mention,
    SlashCommand,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordAnswerContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_thread: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_direct_message: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_kind: Option<AnswerInputKind>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwitchAnswerContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_partner: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_kind: Option<AnswerInputKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "platform", rename_all = "snake_case")]
pub enum AnswerContext {
    Discord(DiscordAnswerContext),
    Twitch(TwitchAnswerContext),
}

impl AnswerContext {
    pub fn validate(&self) -> Result<()> {
        if let Self::Discord(context) = self {
            if (context.is_direct_message == Some(true) && context.is_thread == Some(true))
                || (context.is_thread == Some(false) && context.thread_name.is_some())
                || (context.is_direct_message == Some(true)
                    && (context.channel_name.is_some()
                        || context.category_name.is_some()
                        || context.topic.is_some()
                        || context.thread_name.is_some()))
            {
                return Err(ContractError::InvalidAnswerContext);
            }
        }
        let (names, descriptions) = match self {
            Self::Discord(context) => (
                vec![
                    &context.channel_name,
                    &context.category_name,
                    &context.thread_name,
                ],
                vec![&context.topic, &context.purpose],
            ),
            Self::Twitch(context) => (vec![&context.channel_name], Vec::new()),
        };
        if names.into_iter().flatten().any(|name| {
            name.trim().is_empty() || name.len() > 512 || name.chars().any(char::is_control)
        }) || descriptions
            .into_iter()
            .flatten()
            .any(|text| text.trim().is_empty() || text.len() > 4096 || text.contains('\0'))
        {
            return Err(ContractError::LimitExceeded);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub request_id: String,
    pub conversation_id: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_context: Option<AnswerContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<domain::DomainRequest>,
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
        if let Some(request) = &self.domain {
            request.validate()?;
        }
        if let Some(context) = &self.answer_context {
            context.validate()?;
        }
        if self.text.len() > 32_768 || self.requested_scopes.len() > 64 {
            return Err(ContractError::LimitExceeded);
        }
        if [&self.request_id, &self.conversation_id]
            .into_iter()
            .chain(self.requested_scopes.iter())
            .chain(self.patch.iter())
            .chain(self.mode.iter())
            .any(|id| id.trim().is_empty() || id.len() > 512 || id.chars().any(char::is_control))
        {
            return Err(ContractError::InvalidStableId);
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
    #[serde(skip)]
    pub discord: Option<DiscordRequestContext>,
    pub principal: Principal,
    pub conversation_id: String,
    pub knowledge_release: String,
    pub deadline_ms: u64,
    pub budget: Budget,
    #[serde(skip)]
    pub request_deadline: Option<RequestDeadline>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceVisibility {
    Public,
    Internal,
    Private,
    RequestScoped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordRequestContext {
    pub user_id: Option<u64>,
    pub request_id: String,
    pub scope: String,
    pub allow_discord_reads: bool,
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
        if self.visibility == SourceVisibility::RequestScoped {
            return Err(ContractError::RequestScopedSource);
        }
        if [&self.source_id, &self.logical_id, &self.content_hash]
            .iter()
            .any(|id| id.trim().is_empty() || id.len() > 512 || id.chars().any(char::is_control))
        {
            return Err(ContractError::InvalidStableId);
        }
        if self.revision == 0 || self.revision > i64::MAX as u64 {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ChunkProvenance>,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageAccounting {
    pub observed: Usage,
    pub reserved: Usage,
    pub unaccounted: bool,
}

impl Usage {
    pub fn accumulate(&mut self, other: &Self) -> bool {
        self.provider = other.provider.clone().or_else(|| self.provider.clone());
        self.model = other.model.clone().or_else(|| self.model.clone());
        let mut overflow = false;
        macro_rules! add {
            ($field:ident) => {
                self.$field = self.$field.checked_add(other.$field).unwrap_or_else(|| {
                    overflow = true;
                    self.$field.saturating_add(other.$field)
                });
            };
        }
        add!(input_tokens);
        add!(output_tokens);
        add!(network_rounds);
        add!(cost_micros);
        !overflow
    }
}

impl UsageAccounting {
    pub fn observed(usage: Usage) -> Self {
        Self {
            observed: usage,
            ..Self::default()
        }
    }

    pub fn accumulate(&mut self, other: &Self) {
        self.unaccounted |= other.unaccounted;
        self.unaccounted |= !self.observed.accumulate(&other.observed);
        self.unaccounted |= !self.reserved.accumulate(&other.reserved);
        let mut total = self.observed.clone();
        self.unaccounted |= !total.accumulate(&self.reserved);
    }

    pub fn charged(&self) -> Usage {
        let mut total = self.observed.clone();
        total.accumulate(&self.reserved);
        total
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accounted<T> {
    pub value: T,
    pub accounting: UsageAccounting,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{error}")]
pub struct PortFailure {
    pub error: PortError,
    pub accounting: Option<Box<UsageAccounting>>,
}

impl PortFailure {
    pub fn before_call(error: PortError) -> Self {
        Self::accounted(error, UsageAccounting::default())
    }

    pub fn accounted(error: PortError, accounting: UsageAccounting) -> Self {
        Self {
            error,
            accounting: Some(Box::new(accounting)),
        }
    }
}

impl From<PortError> for PortFailure {
    fn from(error: PortError) -> Self {
        Self {
            error,
            accounting: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerStatus {
    Answered,
    Unverified,
    BuildRejected,
    InsufficientEvidence,
    UnauthorizedEvidence,
    Unavailable,
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
    pub cited_evidence_ids: Vec<String>,
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
    pub source_revisions: BTreeMap<String, BTreeMap<String, u64>>,
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
    #[error("permission denied: {0}")]
    PermissionDenied(String),
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

    fn retrieve_with_usage(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> std::result::Result<(Vec<Evidence>, Usage), PortError> {
        self.retrieve(query, context)
            .map(|evidence| (evidence, Usage::default()))
    }

    fn validate_publication(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _evidence: &[Evidence],
    ) -> std::result::Result<(), PortError> {
        Err(PortError::Unavailable(
            "canonical publication validation unavailable".into(),
        ))
    }

    fn validate_evidence(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _evidence: &[Evidence],
        _for_provider: bool,
    ) -> std::result::Result<(), PortError> {
        Err(PortError::Unavailable(
            "canonical evidence validation unavailable".into(),
        ))
    }
}

pub trait AnswerProviderPort: Send + Sync {
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<Accounted<ProviderAnswer>, PortFailure> {
        self.answer(query, context, evidence)
            .map(|value| Accounted {
                accounting: UsageAccounting::observed(value.usage.clone()),
                value,
            })
            .map_err(PortFailure::from)
    }

    fn answer_turn_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[tools::ToolDefinition],
        conversation: &tools::ToolConversation,
    ) -> std::result::Result<Accounted<tools::ProviderTurn>, PortFailure> {
        self.answer_turn(query, context, evidence, tools, conversation)
            .map(|value| Accounted {
                accounting: UsageAccounting::observed(value.usage().clone()),
                value,
            })
            .map_err(PortFailure::from)
    }

    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError>;

    fn answer_turn(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[tools::ToolDefinition],
        conversation: &tools::ToolConversation,
    ) -> std::result::Result<tools::ProviderTurn, PortError> {
        if query.domain.is_some() || !tools.is_empty() || !conversation.messages.is_empty() {
            return Err(PortError::Unavailable(
                "Provider unterstützt nur belegte Textanfragen ohne Werkzeuggespräch".into(),
            ));
        }
        query
            .validate()
            .map_err(|_| PortError::InvalidResponse("Ungültige Textanfrage".into()))?;
        let turn = ProviderTurn::from(self.answer(query, context, evidence)?);
        turn.validate(&[])?;
        Ok(turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anfragebezogene_quellen_duerfen_nicht_persistiert_werden() {
        let mut source = SourceRecordV2 {
            source_id: "discord".into(),
            logical_id: "kanal".into(),
            revision: 1,
            content_hash: "hash".into(),
            content: "Inhalt".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        assert_eq!(source.validate(), Ok(()));
        source.visibility = SourceVisibility::Private;
        assert_eq!(source.validate(), Ok(()));
        source.visibility = SourceVisibility::RequestScoped;
        assert_eq!(source.validate(), Err(ContractError::RequestScopedSource));
    }

    #[test]
    fn query_rejects_empty_identity_fields() {
        let query = Query {
            answer_context: None,
            domain: None,
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
            answer_context: None,
            domain: None,
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
    fn ortsvertrag_bleibt_optional_und_lehnt_ids_und_dm_guildfelder_ab() {
        let query: Query = serde_json::from_value(serde_json::json!({
            "request_id":"r", "conversation_id":"c", "text":"Frage"
        }))
        .unwrap();
        assert!(query.answer_context.is_none());
        assert!(serde_json::to_value(query)
            .unwrap()
            .get("answer_context")
            .is_none());
        for field in [
            "channel_id",
            "guild_id",
            "user_id",
            "requested_scopes",
            "instructions",
        ] {
            let mut value = serde_json::json!({"platform":"discord"});
            value[field] = serde_json::json!("123");
            assert!(serde_json::from_value::<AnswerContext>(value).is_err());
        }
        for field in ["channel_name", "category_name", "topic", "thread_name"] {
            let mut value = serde_json::json!({"platform":"discord", "is_direct_message":true});
            value[field] = serde_json::json!("Guildwert");
            assert!(serde_json::from_value::<AnswerContext>(value)
                .unwrap()
                .validate()
                .is_err());
        }
        let context: AnswerContext = serde_json::from_value(serde_json::json!({
            "platform":"discord", "is_direct_message":true, "input_kind":"message"
        }))
        .unwrap();
        assert_eq!(context.validate(), Ok(()));
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
            provenance: None,
            patch: None,
        };
        assert_eq!(
            evidence.validate(),
            Err(ContractError::InvalidEvidenceScore)
        );
    }
}
