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
pub trait RuleEvaluatorPort: Send + Sync {
    fn evaluate(
        &self,
        rule: &TypedRule,
        facts: &[NumericFact],
        release: &CorpusRelease,
        validity: &Validity,
    ) -> Result<RuleEvaluation, PortError>;
}
