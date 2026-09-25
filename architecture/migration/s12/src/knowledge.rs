//! Shared parser IR and a loss-aware adapter to the EXISTING brain.v1 contracts.
//! Mappings/reviews are trusted operator inputs, never read from wiki programs.
//! No database, scheduler, evaluator, network access or publication is implemented here.
use crate::{analyze, model::*, parse_json, sha256, Result};
use brain_contracts::{
    CorpusRelease, DocumentRevision, Fact, HeroKnowledgeCard, SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const IR_VERSION: &str = "wiki-ir-v1";
pub const PROJECTOR_VERSION: &str = "wiki-contract-projector-v1";

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceLocator {
    pub page_id: i64,
    pub revision_id: i64,
    pub content_hash: String,
    pub locator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMapping {
    pub page_id: i64,
    pub predicate: String,
    /// JSON pointer (`json:/...`), literal path (`lua:/...`) or template path.
    pub value_locator: String,
    pub kind: ValueKind,
    pub unit_locator: Option<String>,
    pub condition_locator: Option<String>,
    pub variant_locator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MappingProfile {
    pub version: String,
    pub review_ref: String,
    pub source_key: String,
    /// Persistent/explicit IDs, not slugs inferred from current display names.
    pub entities: BTreeMap<i64, String>,
    pub fields: Vec<FieldMapping>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Alias {
    pub text: String,
    pub locale: Option<String>,
    pub subject_id: String,
    pub source: SourceLocator,
}

/// Constructible only through extraction; callers cannot inject deserialized
/// "approved" fields into the projector. Serialize is for diagnostics, not import.
#[derive(Debug, Serialize)]
pub struct WikiIr {
    version: &'static str,
    mapping_version: String,
    mapping_review_ref: String,
    report: Report,
    sources: Vec<SourceRecordV2>,
    fields: Vec<IrField>,
    aliases: Vec<Alias>,
    entities: BTreeMap<i64, String>,
}
impl WikiIr {
    pub fn report(&self) -> &Report {
        &self.report
    }
    pub fn fields(&self) -> &[IrField] {
        &self.fields
    }
    pub fn sources(&self) -> &[SourceRecordV2] {
        &self.sources
    }
    pub fn aliases(&self) -> &[Alias] {
        &self.aliases
    }
    /// Ambiguity is unknown, never a fuzzy choice of another hero.
    pub fn resolve_alias(&self, text: &str, locale: &str) -> Option<&str> {
        let key = alias_key(text);
        let subjects: BTreeSet<_> = self
            .aliases
            .iter()
            .filter(|a| a.locale.as_deref() == Some(locale) && alias_key(&a.text) == key)
            .map(|a| a.subject_id.as_str())
            .collect();
        (subjects.len() == 1).then(|| *subjects.first().unwrap())
    }
}
fn alias_key(text: &str) -> String {
    text.replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
fn selector(locator: &str) -> &str {
    locator
        .split_once("@utf8:")
        .map_or(locator, |(path, _)| path)
}
fn candidate<'a>(revision: &'a RevisionProbe, path: &str) -> Option<&'a Candidate> {
    let mut matches = revision
        .candidates
        .iter()
        .filter(|c| c.locator == path || selector(&c.locator) == path);
    let found = matches.next()?;
    matches.next().is_none().then_some(found)
}
fn location(page: &PageProbe, revision: &RevisionProbe, candidate: &Candidate) -> SourceLocator {
    SourceLocator {
        page_id: page.page_id,
        revision_id: revision.revision_id,
        content_hash: revision.raw_sha256.clone().unwrap_or_default(),
        locator: candidate.locator.clone(),
    }
}

/// Fixed decimal grammar. No exponent, locale comma, unit suffix, implicit rounding,
/// percentage conversion or binary float coercion. Missing is not zero.
pub fn decimal(text: &str) -> Option<String> {
    let text = text.trim();
    let negative = text.starts_with('-');
    let unsigned = text
        .strip_prefix('-')
        .or_else(|| text.strip_prefix('+'))
        .unwrap_or(text);
    let (whole, frac) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    if whole.is_empty()
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || !frac.bytes().all(|b| b.is_ascii_digit())
        || frac.len() > 9
        || (unsigned.contains('.') && frac.is_empty())
        || whole.len() > 28
    {
        return None;
    }
    let whole = whole.trim_start_matches('0');
    let whole = if whole.is_empty() { "0" } else { whole };
    let frac = frac.trim_end_matches('0');
    let sign = if negative && (whole != "0" || !frac.is_empty()) {
        "-"
    } else {
        ""
    };
    Some(if frac.is_empty() {
        format!("{sign}{whole}")
    } else {
        format!("{sign}{whole}.{frac}")
    })
}
fn scalar_decimal(value: &Value) -> Option<String> {
    if let Some(s) = value.as_str() {
        decimal(s)
    } else if value.is_i64() || value.is_u64() {
        decimal(&value.to_string())
    } else {
        None
    } // Fractional JSON numbers require a lossless source string/mapping.
}

/// All formats feed the same candidate->IR path. Source metadata is preserved in
/// SourceRecordV2.metadata; source timestamps never populate game validity fields.
pub fn extract(bytes: &[u8], profile: &MappingProfile) -> Result<WikiIr> {
    let report = analyze(bytes)?;
    if profile.source_key != report.source_key
        || profile.version.trim().is_empty()
        || profile.review_ref.trim().is_empty()
        || profile.fields.len() > MAX_CANDIDATES
        || profile.entities.len() > MAX_PAGES
    {
        return Err("invalid/unreviewed mapping profile".into());
    }
    let capture: Capture =
        serde_json::from_value(parse_json(bytes)?).map_err(|_| "invalid capture")?;
    let pages: BTreeMap<_, _> = report.pages.iter().map(|p| (p.page_id, p)).collect();
    for (id, entity) in &profile.entities {
        if !pages.contains_key(id) || entity.trim().is_empty() {
            return Err("unknown page or empty persistent entity mapping".into());
        }
    }
    let mut sources = BTreeMap::new();
    for input in &capture.pages {
        let page = pages[&input.page_id];
        if matches!(page.stage, Stage::PolicyBlocked | Stage::ApprovedExclusion) {
            continue;
        }
        let Some(raw_pages) = input.response.pointer("/query/pages") else {
            continue;
        };
        let raw_pages: Vec<_> = match raw_pages {
            Value::Array(a) => a.iter().collect(),
            Value::Object(o) => o.values().collect(),
            _ => Vec::new(),
        };
        for raw in raw_pages {
            for revision in raw["revisions"].as_array().into_iter().flatten() {
                let Some(id) = revision["revid"].as_i64() else {
                    continue;
                };
                let Some(probe) = page.revisions.iter().find(|r| r.revision_id == id) else {
                    continue;
                };
                let slot = revision.pointer("/slots/main").unwrap_or(revision);
                let Some(content) = slot
                    .get("content")
                    .or_else(|| slot.get("*"))
                    .and_then(Value::as_str)
                else {
                    continue;
                };
                if probe.raw_sha256.as_deref() != Some(&sha256(content.as_bytes())) {
                    continue;
                }
                let mut metadata = BTreeMap::new();
                for (k, v) in [
                    (
                        "parser_version",
                        Value::String(report.parser_version.clone()),
                    ),
                    ("namespace", page.namespace.into()),
                    ("title", page.title.clone().into()),
                    ("source_time", serde_json::json!(probe.source_time)),
                    ("retrieved_at", report.retrieved_at.into()),
                    ("language", serde_json::json!(page.source_language)),
                    ("upstream_url", serde_json::json!(page.upstream_url)),
                    (
                        "policy",
                        serde_json::to_value(&report.source_policy)
                            .map_err(|_| "policy encoding")?,
                    ),
                    ("dependencies", serde_json::json!(page.dependency_ids)),
                    ("stage", serde_json::json!(page.stage)),
                    ("content_model", probe.content_model.clone().into()),
                    ("patch", Value::Null),
                ] {
                    metadata.insert(k.into(), v.to_string());
                }
                let source = SourceRecordV2 {
                    source_id: report.source_key.clone(),
                    logical_id: page.source_id.clone(),
                    revision: id as u64,
                    content_hash: sha256(content.as_bytes()),
                    content: content.into(),
                    visibility: SourceVisibility::Internal,
                    allowed_scopes: BTreeSet::from(["wiki.review".into()]),
                    tombstone: false,
                    valid_from: None,
                    valid_to: None,
                    metadata,
                };
                source
                    .validate()
                    .map_err(|_| "invalid shared source record")?;
                sources.insert((page.page_id, id), source);
            }
        }
    }
    let mut fields = Vec::new();
    let mut seen = BTreeSet::new();
    for mapping in &profile.fields {
        if mapping.predicate.trim().is_empty() || mapping.value_locator.is_empty() {
            return Err("empty field mapping".into());
        }
        let subject = profile
            .entities
            .get(&mapping.page_id)
            .ok_or("field page lacks persistent entity mapping")?;
        let page = pages[&mapping.page_id];
        let Some(revision) = page.latest() else {
            continue;
        };
        let Some(hash) = revision.raw_sha256.as_ref() else {
            continue;
        };
        let mut locators = Vec::new();
        let mut unknowns = BTreeSet::new();
        let raw_value = candidate(revision, &mapping.value_locator);
        if let Some(c) = raw_value {
            locators.push(location(page, revision, c));
        }
        let unit = mapping
            .unit_locator
            .as_ref()
            .and_then(|path| candidate(revision, path));
        if let Some(c) = unit {
            locators.push(location(page, revision, c));
        }
        let unknown = |reason: &str| IrValue::Unknown {
            reason: reason.into(),
        };
        let value = match raw_value.map(|c| &c.value) {
            None => unknown("missing_field"),
            Some(Value::Null) => unknown("explicit_null"),
            Some(value) => match mapping.kind {
                ValueKind::Quantity => match (
                    scalar_decimal(value),
                    unit.and_then(|c| c.value.as_str()).and_then(Unit::parse),
                ) {
                    (Some(decimal), Some(unit)) => IrValue::Quantity { decimal, unit },
                    (None, _) => unknown("invalid_or_lossy_decimal"),
                    (_, None) => unknown("missing_or_unknown_unit"),
                },
                ValueKind::Boolean => match value {
                    Value::Bool(value) => IrValue::Boolean { value: *value },
                    Value::String(s) if s.trim() == "true" || s.trim() == "false" => {
                        IrValue::Boolean {
                            value: s.trim() == "true",
                        }
                    }
                    _ => unknown("unknown_boolean"),
                },
                ValueKind::Text => value.as_str().map_or_else(
                    || unknown("non_text_value"),
                    |s| IrValue::Text { value: s.into() },
                ),
            },
        };
        let mut expression = |path: &Option<String>, kind: &str| -> Option<String> {
            let path = path.as_ref()?;
            let Some(c) = candidate(revision, path) else {
                unknowns.insert(format!("missing_{kind}"));
                return None;
            };
            locators.push(location(page, revision, c));
            let Some(s) = c.value.as_str().filter(|s| !s.trim().is_empty()) else {
                unknowns.insert(format!("unknown_{kind}"));
                return None;
            };
            Some(s.to_string())
        };
        let condition = expression(&mapping.condition_locator, "condition");
        let variant = expression(&mapping.variant_locator, "variant");
        if !page.previewable() {
            unknowns.insert("source_quarantined_or_unavailable".into());
        }
        if !page.dependencies_complete {
            unknowns.insert("dependencies_incomplete".into());
        }
        if let IrValue::Unknown { reason } = &value {
            unknowns.insert(reason.clone());
        }
        let identity = serde_json::json!([
            report.source_key,
            subject,
            mapping,
            profile.version,
            profile.review_ref,
            report.parser_version,
            IR_VERSION
        ]);
        let id = format!("wiki-field:{}", sha256(identity.to_string().as_bytes()));
        if !seen.insert(id.clone()) {
            return Err("duplicate field mapping".into());
        }
        fields.push(IrField {
            id,
            subject_id: subject.clone(),
            predicate: mapping.predicate.clone(),
            value,
            condition,
            variant,
            source_revision: DocumentRevision {
                logical_id: page.source_id.clone(),
                revision: revision.revision_id as u64,
                content_hash: hash.clone(),
                source_id: report.source_key.clone(),
            },
            locators,
            unknowns,
        });
    }
    fields.sort_by(|a, b| a.id.cmp(&b.id));
    let mut aliases = Vec::new();
    for page in report
        .pages
        .iter()
        .filter(|p| p.kind == PageKind::Redirect && p.previewable())
    {
        let Some(subject) = page
            .redirect_target
            .and_then(|id| profile.entities.get(&id))
        else {
            continue;
        };
        let target = pages[&page.redirect_target.unwrap()];
        if !target.previewable() || crate::preview::safe_closure(page.page_id, &pages).is_none() {
            continue;
        }
        let Some(revision) = page.latest() else {
            continue;
        };
        let Some(c) = revision.candidates.first() else {
            continue;
        };
        aliases.push(Alias {
            text: page.title.clone(),
            locale: page.source_language.clone(),
            subject_id: subject.clone(),
            source: location(page, revision, c),
        });
    }
    aliases.sort_by(|a, b| {
        (&a.text, &a.locale, &a.subject_id).cmp(&(&b.text, &b.locale, &b.subject_id))
    });
    Ok(WikiIr {
        version: IR_VERSION,
        mapping_version: profile.version.clone(),
        mapping_review_ref: profile.review_ref.clone(),
        report,
        sources: sources.into_values().collect(),
        fields,
        aliases,
        entities: profile.entities.clone(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionReview {
    pub decision_ref: String,
    /// Approval is pinned to the exact immutable source revision + hash, not just
    /// a stable field ID which could silently approve a later changed number.
    pub approved_fields: BTreeMap<String, DocumentRevision>,
    pub source_revisions: BTreeMap<String, DocumentRevision>,
}

#[derive(Debug, Serialize)]
pub struct CardProjection {
    pub card: HeroKnowledgeCard,
    pub locale: String,
    pub generator_version: &'static str,
    pub review_ref: String,
    pub mapping_review_ref: String,
    pub source_policy_sha256: String,
    pub knowledge_version: String,
    pub mapping_version: String,
    pub unknowns: BTreeSet<String>,
    pub locators: BTreeMap<String, Vec<SourceLocator>>,
    pub dependencies: Vec<DocumentRevision>,
    pub aliases: Vec<Alias>,
    pub rebuild_sha256: String,
    /// No adapter here owns the canonical publisher or can grant rights.
    pub published: bool,
}

pub fn project_card(
    ir: &WikiIr,
    hero_page_id: i64,
    locale: &str,
    release: &CorpusRelease,
    review: &ProjectionReview,
) -> Result<CardProjection> {
    if review.decision_ref.trim().is_empty()
        || release.release_id.trim().is_empty()
        || release.knowledge_version.trim().is_empty()
    {
        return Err("projection needs an explicit reviewed release".into());
    }
    let hero = ir
        .report
        .pages
        .iter()
        .find(|p| p.page_id == hero_page_id && p.kind == PageKind::Hero)
        .ok_or("unknown hero")?;
    let hero_id = ir
        .entities
        .get(&hero_page_id)
        .ok_or("unknown persistent hero ID")?;
    let preview = ir
        .report
        .card_previews
        .iter()
        .find(|c| c.hero_source_id == hero.source_id && c.locale == locale)
        .ok_or("unknown hero/locale binding")?;
    let allowed_sources: BTreeSet<_> = preview
        .sections
        .values()
        .flatten()
        .map(|r| r.source_id.as_str())
        .collect();
    let mut unknowns: BTreeSet<String> = [
        "game_patch_not_inferred_from_wiki_revision",
        "mode_not_representable_in_brain_v1_card",
        "rules_and_synergies_require_domain_validation",
        "not_published",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    unknowns.extend(preview.unknowns.iter().filter(|s| s.contains(":")).cloned());
    let mut dependencies = BTreeMap::new();
    for reference in preview.sections.values().flatten() {
        let pinned = release
            .source_revisions
            .get(&ir.report.source_key)
            .and_then(|p| p.get(&reference.source_id));
        if pinned != Some(&(reference.revision_id as u64)) {
            return Err("release does not pin exact wiki/dependency revision".into());
        }
        dependencies.insert(
            reference.source_id.clone(),
            DocumentRevision {
                source_id: ir.report.source_key.clone(),
                logical_id: reference.source_id.clone(),
                revision: reference.revision_id as u64,
                content_hash: reference.raw_sha256.clone(),
            },
        );
    }
    for (id, dependency) in &dependencies {
        if review.source_revisions.get(id) != Some(dependency) {
            return Err("review does not pin exact dependency revision and hash".into());
        }
    }
    let mut facts = Vec::new();
    let mut locators = BTreeMap::new();
    let mut slots = BTreeSet::new();
    for field in &ir.fields {
        if !allowed_sources.contains(field.source_revision.logical_id.as_str()) {
            continue;
        }
        if review.approved_fields.get(&field.id) != Some(&field.source_revision) {
            unknowns.insert(format!("{}:unreviewed_revision", field.id));
            continue;
        }
        if !field.unknowns.is_empty() || field.condition.is_some() || field.variant.is_some() {
            unknowns.insert(format!(
                "{}:unknown_or_context_not_representable_in_brain_v1_fact",
                field.id
            ));
            continue;
        }
        let (value, unit) = match &field.value {
            IrValue::Quantity { decimal, unit } => {
                (Value::String(decimal.clone()), Some(unit.as_str().into()))
            }
            IrValue::Boolean { value } => (Value::Bool(*value), None),
            IrValue::Text { value } => (Value::String(value.clone()), None),
            IrValue::Unknown { .. } => {
                unknowns.insert(format!("{}:unknown", field.id));
                continue;
            }
        };
        if !slots.insert((&field.subject_id, &field.predicate)) {
            return Err("conflicting subject/predicate facts require quarantine".into());
        }
        let fact_id = format!(
            "{}:r{}:{}",
            field.id, field.source_revision.revision, field.source_revision.content_hash
        );
        locators.insert(fact_id.clone(), field.locators.clone());
        facts.push(Fact {
            fact_id,
            subject_id: field.subject_id.clone(),
            key: field.predicate.clone(),
            value,
            unit,
            source_revision: field.source_revision.clone(),
        });
    }
    facts.sort_by(|a, b| a.fact_id.cmp(&b.fact_id));
    if facts.is_empty() {
        unknowns.insert("no_reviewed_representable_facts".into());
    }
    let referenced_pages: BTreeSet<i64> = ir
        .report
        .pages
        .iter()
        .filter(|p| allowed_sources.contains(p.source_id.as_str()))
        .map(|p| p.page_id)
        .collect();
    let mut result = CardProjection {
        card: HeroKnowledgeCard {
            hero_id: hero_id.clone(),
            knowledge_release: release.release_id.clone(),
            facts,
            rules: Vec::new(),
            synergies: Vec::new(),
        },
        locale: locale.into(),
        generator_version: PROJECTOR_VERSION,
        review_ref: review.decision_ref.clone(),
        mapping_review_ref: ir.mapping_review_ref.clone(),
        source_policy_sha256: sha256(
            &serde_json::to_vec(&ir.report.source_policy).map_err(|_| "policy encoding")?,
        ),
        knowledge_version: release.knowledge_version.clone(),
        mapping_version: ir.mapping_version.clone(),
        unknowns,
        locators,
        dependencies: dependencies.into_values().collect(),
        aliases: ir
            .aliases
            .iter()
            .filter(|a| a.subject_id == *hero_id && referenced_pages.contains(&a.source.page_id))
            .cloned()
            .collect(),
        rebuild_sha256: String::new(),
        published: false,
    };
    result.rebuild_sha256 = sha256(&serde_json::to_vec(&result).map_err(|_| "card encoding")?);
    Ok(result)
}

/// Extend the existing dependency delta with mapping/IR changes. This is still
/// a pure invalidation result, not a queue or a second rebuild implementation.
pub fn compare_ir(before: &WikiIr, after: &WikiIr) -> Result<Impact> {
    let mut impact = crate::compare(&before.report, &after.report)?;
    let metadata_changed = before.version != after.version
        || before.mapping_version != after.mapping_version
        || before.mapping_review_ref != after.mapping_review_ref;
    let mut sources = BTreeSet::new();
    for ir in [before, after] {
        for field in &ir.fields {
            let old = before.fields.iter().find(|f| f.id == field.id);
            let new = after.fields.iter().find(|f| f.id == field.id);
            if old != new {
                sources.insert(field.source_revision.logical_id.clone());
            }
        }
        for page_id in ir.entities.keys() {
            if before.entities.get(page_id) != after.entities.get(page_id) {
                if let Some(page) = ir.report.pages.iter().find(|p| p.page_id == *page_id) {
                    sources.insert(page.source_id.clone());
                }
            }
        }
    }
    for card in before
        .report
        .card_previews
        .iter()
        .chain(&after.report.card_previews)
    {
        if metadata_changed
            || sources.contains(&card.hero_source_id)
            || card
                .sections
                .values()
                .flatten()
                .any(|r| sources.contains(&r.source_id))
        {
            impact.reproject_heroes.insert(card.hero_source_id.clone());
        }
    }
    Ok(impact)
}
