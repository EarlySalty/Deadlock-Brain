//! Release-bound domain projections. Values remain in the existing cards, Wiki IR,
//! NumericFact/TypedRule and frozen Reasoner models, not in a second stats catalog.
use crate::{
    domain::*, value::Observed, wiki::IrField, DocumentRevision, Effect, HeroKnowledgeCard,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const DOMAIN_ANSWER_VERSION: &str = "brain.domain-answer.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocatedRevision {
    pub source: DocumentRevision,
    /// Source selector, including a JSON pointer for frozen model/rule documents.
    pub locator: String,
    pub parser_revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalizedAlias {
    pub text: String,
    pub locale: String,
    pub provenance: LocatedRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainKnowledgeCard {
    pub card: HeroKnowledgeCard,
    pub validity: Validity,
    pub source: LocatedRevision,
    pub dependencies: Vec<LocatedRevision>,
    pub aliases: Vec<LocalizedAlias>,
    /// Lossless C4 fields, including Unknown, condition and variant. They are NOT
    /// automatically executable facts. Only reviewed card.facts are direct facts.
    pub fields: Vec<IrField>,
    pub effects: Vec<Effect>,
    pub unknowns: BTreeSet<String>,
    pub review_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemAlias {
    pub item_id: i64,
    pub alias: LocalizedAlias,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildCatalogRef {
    pub catalog_id: String,
    pub validity: Validity,
    /// JSON array of EXISTING dbrain_reasoner::ItemModel, frozen by its producer.
    pub models: LocatedRevision,
    /// EXISTING dbrain_reasoner::InventoryRules; no defaults are invented here.
    pub inventory_rules: LocatedRevision,
    /// Original inputs used to produce frozen models, never a second stats copy.
    pub dependencies: Vec<LocatedRevision>,
    /// Producer/reviewer must preserve unknown legality fields from upstream IR.
    /// An empty set is an explicit assertion, not a serde default.
    pub unknown_legality_fields: BTreeSet<String>,
    /// Verified NumericFact in this release, unit `1`.
    pub active_limit_fact_id: String,
    pub aliases: Vec<ItemAlias>,
    pub review_ref: String,
}

/// Typed intent carried by the optional Query.domain field. Existing text-only
/// requests are unchanged. Builds never fall back to prose/LLM parsing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DomainRequest {
    Fact {
        hero: String,
        locale: String,
        predicate: String,
    },
    Rule {
        rule_id: String,
    },
    Build {
        hero: String,
        locale: String,
        catalog_id: String,
        items: Vec<String>,
    },
    Card {
        hero: String,
        locale: String,
    },
}

impl DomainRequest {
    pub fn validate(&self) -> crate::Result<()> {
        let bounded = |s: &str| !s.trim().is_empty() && s.len() <= 512;
        let stable = |s: &str| bounded(s) && !s.chars().any(char::is_control);
        let valid = match self {
            Self::Fact {
                hero,
                locale,
                predicate,
            } => bounded(hero) && matches!(locale.as_str(), "de" | "en") && stable(predicate),
            Self::Rule { rule_id } => stable(rule_id),
            Self::Card { hero, locale } => bounded(hero) && matches!(locale.as_str(), "de" | "en"),
            Self::Build {
                hero,
                locale,
                catalog_id,
                items,
            } => {
                if items.len() > 64 {
                    return Err(crate::ContractError::LimitExceeded);
                }
                bounded(hero)
                    && matches!(locale.as_str(), "de" | "en")
                    && stable(catalog_id)
                    && items.iter().all(|s| bounded(s))
            }
        };
        if valid {
            Ok(())
        } else {
            Err(crate::ContractError::InvalidStableId)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainRoute {
    Fact,
    Rule,
    Build,
    Card,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainVerdict {
    Proven,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildEvaluation {
    pub legal: Observed<bool>,
    pub reason: String,
    pub item_ids: Vec<i64>,
    pub spent_souls: Observed<i64>,
    pub consumed_ids: Vec<i64>,
    pub evaluator_version: String,
}

/// This certificate is output only. The retriever recomputes it from authorized
/// canonical revisions during validation, including cache hits and provider egress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainAnswer {
    pub contract_version: String,
    pub route: DomainRoute,
    pub verdict: DomainVerdict,
    pub text: String,
    pub knowledge_release: String,
    pub validity: Validity,
    pub inputs: Vec<LocatedRevision>,
    pub input_fact_ids: BTreeSet<String>,
    pub rule_evaluation: Option<RuleEvaluation>,
    pub build_evaluation: Option<BuildEvaluation>,
}
