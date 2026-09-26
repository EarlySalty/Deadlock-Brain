//! A bounded typed rule language, never executable source code or model-authored truth.
use crate::{CorpusRelease, DocumentRevision, PortError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
pub const RULE_EVALUATOR_VERSION: &str = "brain.rules.v1";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validity {
    pub patch: String,
    pub mode: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumericFact {
    pub fact_id: String,
    pub subject_id: String,
    pub predicate: String,
    pub quantity: Quantity,
    pub validity: Validity,
    pub source: DocumentRevision,
    pub verified: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum RuleExpr {
    Constant { quantity: Quantity },
    Fact { fact_id: String },
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Min { left: Box<Self>, right: Box<Self> },
    Max { left: Box<Self>, right: Box<Self> },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypedRule {
    pub rule_id: String,
    pub rule_version: String,
    pub validity: Validity,
    pub source: DocumentRevision,
    pub verified: bool,
    pub expression: RuleExpr,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleEvaluation {
    pub quantity: Quantity,
    pub rule_id: String,
    pub rule_version: String,
    pub evaluator_version: String,
    pub knowledge_release: String,
    pub input_fact_ids: BTreeSet<String>,
}
pub const LEGACY_DOMAIN_CONTRACT_VERSION: &str = "brain.domain.v1";
pub const DOMAIN_CONTRACT_VERSION: &str = "brain.domain.v2";
pub use crate::domain_knowledge::*;
pub const DOMAIN_DEPENDENCIES_METADATA_KEY: &str = "brain.domain.dependencies";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "payload",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum DomainObject {
    NumericFact(NumericFact),
    Rule(TypedRule),
    HeroCard(Box<DomainKnowledgeCard>),
    BuildCatalog(Box<BuildCatalogRef>),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredDomainObject {
    pub contract_version: String,
    pub object: DomainObject,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainSnapshot {
    pub release: CorpusRelease,
    pub validity: Validity,
    pub facts: Vec<NumericFact>,
    pub rules: Vec<TypedRule>,
    #[serde(default)]
    pub cards: Vec<DomainKnowledgeCard>,
    #[serde(default)]
    pub catalogs: Vec<BuildCatalogRef>,
    /// Effective historical AND current ACLs, used only inside the trusted domain path.
    #[serde(skip)]
    pub records: Vec<crate::SourceRecordV2>,
    #[serde(skip)]
    pub object_sources: std::collections::BTreeMap<String, DocumentRevision>,
}
/// Implementations authorize BOTH the typed object and its canonical source revision.
pub trait DomainStorePort: Send + Sync {
    fn read_domain(
        &self,
        context: &crate::AuthorizedContext,
        validity: &Validity,
    ) -> Result<DomainSnapshot, PortError>;
}

pub trait RuleEvaluatorPort: Send + Sync {
    fn evaluate(
        &self,
        rule: &TypedRule,
        facts: &[NumericFact],
        release: &CorpusRelease,
        validity: &Validity,
    ) -> Result<RuleEvaluation, PortError>;
}
