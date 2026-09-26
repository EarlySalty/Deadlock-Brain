//! Canonical Wiki IR leaves. These are data, never approval capabilities.
use crate::DocumentRevision;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueKind {
    Quantity,
    Boolean,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Health,
    Damage,
    Seconds,
    Meters,
    Count,
    Souls,
    Percent,
    PercentagePoint,
    Multiplier,
}
impl Unit {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "health" => Some(Self::Health),
            "damage" => Some(Self::Damage),
            "seconds" | "s" => Some(Self::Seconds),
            "meters" | "m" => Some(Self::Meters),
            "count" => Some(Self::Count),
            "souls" => Some(Self::Souls),
            "percent" | "%" => Some(Self::Percent),
            "percentage_point" | "pp" => Some(Self::PercentagePoint),
            "multiplier" => Some(Self::Multiplier),
            _ => None,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Health => "health",
            Self::Damage => "damage",
            Self::Seconds => "seconds",
            Self::Meters => "meters",
            Self::Count => "count",
            Self::Souls => "souls",
            Self::Percent => "percent",
            Self::PercentagePoint => "percentage_point",
            Self::Multiplier => "multiplier",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum IrValue {
    /// Exact base-ten value, at most 9 fractional digits; no floating point math.
    Quantity {
        decimal: String,
        unit: Unit,
    },
    Boolean {
        value: bool,
    },
    Text {
        value: String,
    },
    Unknown {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocator {
    pub page_id: i64,
    pub revision_id: i64,
    pub content_hash: String,
    pub locator: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IrField {
    pub id: String,
    pub subject_id: String,
    pub predicate: String,
    pub value: IrValue,
    /// Conditions/variants remain source expressions, NOT executable predicates.
    pub condition: Option<String>,
    pub variant: Option<String>,
    pub source_revision: DocumentRevision,
    pub locators: Vec<SourceLocator>,
    pub unknowns: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alias {
    pub text: String,
    pub locale: Option<String>,
    pub subject_id: String,
    pub source: SourceLocator,
}

/// Lossless transport DTO. Deserializing it does NOT confer projection/publication authority.
/// The trusted local extractor and explicit ProjectionReview remain mandatory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiIr {
    /// Source namespace shared by page/alias locators; page IDs alone are not global IDs.
    pub source_id: String,
    pub mapping_version: String,
    pub mapping_review_ref: String,
    pub sources: Vec<crate::SourceRecordV2>,
    pub fields: Vec<IrField>,
    pub aliases: Vec<Alias>,
    pub entities: std::collections::BTreeMap<i64, String>,
    pub artifacts: Vec<crate::source::OriginArtifact>,
    pub dependencies: Vec<Dependency>,
    pub dependency_completeness: std::collections::BTreeMap<String, bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub dependent: crate::source::SourceIdentity,
    pub target: crate::value::Observed<crate::source::SourceIdentity>,
    pub target_revision: crate::value::Observed<crate::source::SourceRevision>,
    pub raw_reference: String,
}
impl IrField {
    /// False for BOTH a present source condition/variant and failed extraction.
    /// A None expression with missing_condition/missing_variant in unknowns is NOT unconditional.
    pub fn is_unconditional_known(&self) -> bool {
        self.condition.is_none()
            && self.variant.is_none()
            && self.unknowns.is_empty()
            && !matches!(self.value, IrValue::Unknown { .. })
    }
}
