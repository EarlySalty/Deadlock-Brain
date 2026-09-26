//! Offline capture/report formats ONLY. No canonical runtime knowledge contracts.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const CAPTURE_VERSION: &str = "s12-capture-v1";
pub const PARSER_VERSION: &str = "s12-syntax-probe-v2";
pub const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_CONTENT_BYTES: usize = 512 * 1024;
pub const MAX_PAGES: usize = 10_000;
pub const MAX_CANDIDATES: usize = 2_000;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Capture {
    pub format: String,
    pub source_key: String,
    pub retrieved_at: i64,
    pub policy: CapturePolicy,
    pub siteinfo: Value,
    pub discovery: Vec<DiscoveryBatch>,
    pub pages: Vec<PageCapture>,
    #[serde(default)]
    pub classification: Vec<Classification>,
    #[serde(default)]
    pub hero_bindings: Vec<HeroBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapturePolicy {
    pub offline_review_allowed: bool,
    pub decision_ref: Option<String>,
    pub source_license: Option<String>,
    // Recorded, never promoted to permission by a source's own rightsinfo.
    pub publication_allowed: bool,
    pub provider_egress_allowed: bool,
    pub media_download_allowed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryBatch {
    pub namespace: i64,
    #[serde(default)]
    pub request_continue: BTreeMap<String, String>,
    pub response: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PageCapture {
    pub page_id: i64,
    pub response: Value,
    /// Complete prop=templates|links response (including continuation) is required
    /// before a negative dependency statement can be used for selective deltas.
    pub dependencies_complete: bool,
    pub access_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PageKind {
    Hero,
    Ability,
    Item,
    Mechanic,
    Rule,
    PatchHistory,
    Data,
    Template,
    Module,
    Lore,
    Guide,
    Localization,
    Redirect,
    MediaMetadata,
    Other,
    Unclassified,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Classification {
    pub page_id: i64,
    pub kind: PageKind,
    pub build_critical: bool,
    /// Explicit operator decision, not an implicit omission from discovery.
    pub approved_exclusion: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HeroBinding {
    pub hero_page_id: i64,
    pub locale: String,
    pub ability_page_ids: Vec<i64>,
    pub mechanic_page_ids: Vec<i64>,
    pub item_page_ids: Vec<i64>,
    pub rule_page_ids: Vec<i64>,
    #[serde(default)]
    pub alias_page_ids: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Discovered,
    Fetched,
    Parsed,
    Quarantined,
    Unavailable,
    PolicyBlocked,
    ApprovedExclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub locator: String,
}

impl Diagnostic {
    pub fn new(code: &str, locator: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            locator: locator.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Candidate {
    /// Raw JSON pointer or UTF-8 byte range, NEVER a validated Fact ID.
    pub locator: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RevisionProbe {
    pub revision_id: i64,
    pub source_time: Option<i64>,
    pub retrieved_at: i64,
    pub content_model: String,
    pub raw_sha256: Option<String>,
    pub candidates: Vec<Candidate>,
    pub diagnostics: Vec<Diagnostic>,
    pub syntax_parsed: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PageProbe {
    pub source_language: Option<String>,
    pub upstream_url: Option<String>,
    pub page_id: i64,
    pub source_id: String,
    pub namespace: i64,
    pub title: String,
    pub kind: PageKind,
    pub build_critical: bool,
    pub stage: Stage,
    pub revisions: Vec<RevisionProbe>,
    pub latest_revision: Option<i64>,
    pub dependency_ids: BTreeSet<i64>,
    pub unresolved_dependencies: BTreeSet<String>,
    pub dependencies_complete: bool,
    pub redirect_target: Option<i64>,
    pub diagnostics: Vec<Diagnostic>,
}

impl PageProbe {
    pub fn latest(&self) -> Option<&RevisionProbe> {
        self.revisions
            .iter()
            .find(|revision| Some(revision.revision_id) == self.latest_revision)
    }

    pub fn previewable(&self) -> bool {
        self.stage == Stage::Parsed && self.latest().is_some_and(|r| r.syntax_parsed)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Coverage {
    pub discovered: usize,
    pub fetched: usize,
    pub parsed: usize,
    pub normalized: usize,
    pub validated: usize,
    pub published: usize,
    pub quarantined: usize,
    pub unavailable: usize,
    pub policy_blocked: usize,
    pub approved_exclusion: usize,
    pub build_critical_unvalidated: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SourceRef {
    pub source_id: String,
    pub revision_id: i64,
    pub raw_sha256: String,
    pub candidate_locators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CardPreview {
    pub hero_source_id: String,
    pub locale: String,
    pub patch: Option<String>,
    pub mode: Option<String>,
    pub canonical_knowledge_release: Option<String>,
    pub sections: BTreeMap<String, Vec<SourceRef>>,
    pub unknowns: BTreeSet<String>,
    pub publishable: bool,
    pub preview_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Report {
    pub source_policy: CapturePolicy,
    pub site_default_language: Option<String>,
    pub report_version: String,
    pub parser_version: String,
    pub capture_sha256: String,
    pub source_key: String,
    pub retrieved_at: i64,
    pub discovery_complete: bool,
    pub namespace_names: BTreeMap<i64, String>,
    pub missing_namespaces: BTreeSet<i64>,
    pub continuation_pending: BTreeMap<i64, BTreeMap<String, String>>,
    pub pages: Vec<PageProbe>,
    pub coverage: BTreeMap<PageKind, Coverage>,
    pub card_previews: Vec<CardPreview>,
    pub diagnostics: Vec<Diagnostic>,
    pub blockers: BTreeSet<String>,
    pub mode: String,
    pub integration_verified: bool,
    pub production_publishable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Impact {
    pub changed_pages: BTreeSet<i64>,
    pub removed_pages: BTreeSet<i64>,
    pub reparse_pages: BTreeSet<i64>,
    pub reproject_heroes: BTreeSet<String>,
    pub raw_content_changed: BTreeSet<i64>,
    pub full_reconcile_required: bool,
    pub embedding_jobs_scheduled: usize,
    pub publication_performed: bool,
}
