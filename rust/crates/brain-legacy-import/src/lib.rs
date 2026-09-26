#![forbid(unsafe_code)]

pub mod pg;

use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    CorpusRelease, SourceBatch, SourceCheckpoint, SourceRecordV2, SourceVisibility,
};
use brain_ingestion::document_set::{
    current_pins, prepare_document_batch, CoreDocument, DocumentSetSource,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const PARSER_REVISION: &str = "brain-legacy-import.v1";
pub const PARSER_FAMILY: &str = "brain_legacy";
pub const ARCHIVE_API_VERSION: &str = "brain_legacy.archive.v1";
pub const PATCHNOTES_SOURCE: &str = "legacy-patchnotes";
pub const ENTITIES_SOURCE: &str = "legacy-entities";
pub const MAX_DOCUMENTS_PER_SOURCE: usize = 5_000;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("invalid legacy import state: {0}")]
    Invalid(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Port(#[from] brain_contracts::PortError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, ImportError>;

fn invalid(message: impl Into<String>) -> ImportError {
    ImportError::Invalid(message.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePolicyConfig {
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub provider_egress_allowed: bool,
    pub publication_allowed: bool,
    pub raw_retention_allowed: bool,
}

impl SourcePolicyConfig {
    fn validate(&self) -> Result<()> {
        if self.visibility != SourceVisibility::Public && self.allowed_scopes.is_empty() {
            return Err(invalid("non-public legacy source requires explicit scopes"));
        }
        if self
            .allowed_scopes
            .iter()
            .any(|s| s.trim().is_empty() || s.chars().any(char::is_control))
        {
            return Err(invalid("invalid scope"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportContext {
    pub snapshot_label: String,
    pub snapshot_epoch: i64,
    pub schema_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyDocument {
    pub logical_id: String,
    pub content: String,
    pub kind: &'static str,
    pub locator: String,
    pub source_time: Observed<SourceTimestamp>,
    pub language: Observed<String>,
    pub origin_artifacts: BTreeSet<String>,
    pub patch: Observed<String>,
    pub derivation_family: String,
    pub legacy_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacySource {
    pub source_id: &'static str,
    pub documents: Vec<LegacyDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchLineRow {
    pub event_id: i64,
    pub patch_external_id: String,
    pub patch_title: Option<String>,
    pub patch_url: Option<String>,
    pub posted_at_epoch: Option<i64>,
    pub posted_at_iso: Option<String>,
    pub source_kind: Option<String>,
    pub line_index: Option<i64>,
    pub section: Option<String>,
    pub entity_name: Option<String>,
    pub raw_line: Option<String>,
    pub language: Option<String>,
    pub stat_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityRow {
    pub entity_id: i64,
    pub entity_type: String,
    pub canonical_name: String,
    pub primary_external_id: Option<String>,
    pub source: Option<String>,
    pub metadata: serde_json::Value,
    pub aliases: Vec<(String, Option<String>)>,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn clean(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect::<String>()
        .trim()
        .to_string()
}

fn present(value: &Option<String>) -> Option<String> {
    value.as_deref().map(clean).filter(|v| !v.is_empty())
}

fn single<T: Clone + Ord>(values: &BTreeSet<T>) -> Observed<T> {
    match values.len() {
        0 => Observed::unknown(UnknownReason::NotPresent),
        1 => Observed::known(values.iter().next().cloned().unwrap()),
        _ => Observed::unknown(UnknownReason::Unmapped),
    }
}

pub fn patch_documents(rows: &[PatchLineRow]) -> Result<LegacySource> {
    let mut grouped: BTreeMap<&str, Vec<&PatchLineRow>> = BTreeMap::new();
    for row in rows {
        if clean(&row.patch_external_id).is_empty() {
            return Err(invalid("patch event without external id"));
        }
        grouped.entry(&row.patch_external_id).or_default().push(row);
    }
    let mut documents = Vec::with_capacity(grouped.len());
    for (external_id, mut lines) in grouped {
        lines.sort_by_key(|row| (row.line_index, row.event_id));
        let mut headers = BTreeSet::new();
        let mut urls = BTreeSet::new();
        let mut times = BTreeSet::new();
        let mut languages = BTreeSet::new();
        for row in &lines {
            headers.insert((
                present(&row.patch_title),
                present(&row.patch_url),
                present(&row.posted_at_iso),
                present(&row.source_kind),
            ));
            if let Some(url) = present(&row.patch_url) {
                urls.insert(url);
            }
            if let Some(epoch) = row.posted_at_epoch {
                times.insert(epoch);
            }
            if let Some(language) = present(&row.language) {
                languages.insert(language);
            }
        }
        let mut content = String::new();
        for (title, url, posted, kind) in &headers {
            content.push_str(&format!(
                "Patch: {}\nURL: {}\nPosted: {}\nSource kind: {}\n",
                title.as_deref().unwrap_or("unknown"),
                url.as_deref().unwrap_or("unknown"),
                posted.as_deref().unwrap_or("unknown"),
                kind.as_deref().unwrap_or("unknown"),
            ));
        }
        content.push('\n');
        let mut line_count = 0usize;
        for row in &lines {
            let Some(raw) = present(&row.raw_line) else {
                continue;
            };
            line_count += 1;
            let mut line = String::from("- ");
            if let Some(section) = present(&row.section) {
                line.push_str(&format!("[{section}] "));
            }
            match present(&row.entity_name) {
                Some(entity) if !raw.to_lowercase().starts_with(&entity.to_lowercase()) => {
                    line.push_str(&format!("{entity}: {raw}"))
                }
                _ => line.push_str(&raw),
            }
            if let Some(stat) = present(&row.stat_name) {
                line.push_str(&format!(
                    " (change: {stat} {} -> {}{})",
                    present(&row.old_value).unwrap_or_else(|| "unknown".into()),
                    present(&row.new_value).unwrap_or_else(|| "unknown".into()),
                    present(&row.unit)
                        .map(|unit| format!(" {unit}"))
                        .unwrap_or_default(),
                ));
            }
            content.push_str(&line);
            content.push('\n');
        }
        if line_count == 0 {
            return Err(invalid(format!("patch {external_id} has no lines")));
        }
        let locator = match urls.len() {
            1 => urls.iter().next().cloned().unwrap(),
            _ => format!("brain_legacy.patch_events#patch_external_id={external_id}"),
        };
        documents.push(LegacyDocument {
            logical_id: format!("patch/{}", clean(external_id)),
            content,
            kind: "prose",
            locator,
            source_time: match single(&times) {
                Observed::Known { value } => Observed::known(SourceTimestamp::UnixSeconds(value)),
                Observed::Unknown { reason } => Observed::unknown(reason),
            },
            language: single(&languages),
            origin_artifacts: urls,
            patch: Observed::known(clean(external_id)),
            derivation_family: "brain_legacy.patch_events+patch_event_enrichments".into(),
            legacy_rows: lines.len(),
        });
    }
    Ok(LegacySource {
        source_id: PATCHNOTES_SOURCE,
        documents,
    })
}

fn metadata_value(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) => Some(clean(s)).filter(|s| !s.is_empty()),
        serde_json::Value::Object(map) => {
            let sorted: BTreeMap<_, _> = map.iter().collect();
            serde_json::to_string(&sorted).ok()
        }
        other => Some(other.to_string()),
    }
}

pub fn entity_documents(rows: &[EntityRow]) -> Result<LegacySource> {
    let mut seen = BTreeSet::new();
    let mut documents = Vec::with_capacity(rows.len());
    let mut sorted: Vec<&EntityRow> = rows.iter().collect();
    sorted.sort_by(|a, b| {
        (&a.entity_type, &a.canonical_name, a.entity_id).cmp(&(
            &b.entity_type,
            &b.canonical_name,
            b.entity_id,
        ))
    });
    for row in sorted {
        let entity_type = clean(&row.entity_type);
        let name = clean(&row.canonical_name);
        if entity_type.is_empty() || name.is_empty() {
            return Err(invalid("entity without type or canonical name"));
        }
        let logical_id = format!("entity/{entity_type}/{name}");
        if !seen.insert(logical_id.clone()) {
            return Err(invalid(format!("duplicate legacy entity {logical_id}")));
        }
        let mut content = format!("{entity_type}: {name}\n");
        let external = present(&row.primary_external_id);
        if let Some(id) = &external {
            content.push_str(&format!("External ID: {id}\n"));
        }
        if let Some(source) = present(&row.source) {
            content.push_str(&format!("Source: {source}\n"));
        }
        let aliases: BTreeSet<String> = row
            .aliases
            .iter()
            .map(|(alias, _)| clean(alias))
            .filter(|alias| !alias.is_empty() && alias != &name)
            .collect();
        if !aliases.is_empty() {
            content.push_str(&format!(
                "Aliases: {}\n",
                aliases.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        if let serde_json::Value::Object(map) = &row.metadata {
            let sorted: BTreeMap<_, _> = map.iter().collect();
            for (key, value) in sorted {
                if let Some(value) = metadata_value(value) {
                    content.push_str(&format!("{}: {value}\n", clean(key)));
                }
            }
        }
        let source = present(&row.source);
        documents.push(LegacyDocument {
            locator: format!(
                "brain_legacy.entities#{}",
                external.clone().unwrap_or_else(|| logical_id.clone())
            ),
            logical_id,
            content,
            kind: "fact",
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::unknown(UnknownReason::NotPresent),
            origin_artifacts: match (source, external) {
                (Some(source), Some(id)) => BTreeSet::from([format!("{source}:{id}")]),
                _ => BTreeSet::new(),
            },
            patch: Observed::unknown(UnknownReason::NotPresent),
            derivation_family: "brain_legacy.entities+entity_aliases".into(),
            legacy_rows: 1 + row.aliases.len(),
        });
    }
    Ok(LegacySource {
        source_id: ENTITIES_SOURCE,
        documents,
    })
}

pub fn configuration(source_id: &str, policy: &SourcePolicyConfig) -> Result<String> {
    Ok(sha256_hex(
        serde_json::to_string(&(source_id, PARSER_REVISION, policy))?.as_bytes(),
    ))
}

fn core_document(
    source_id: &str,
    document: &LegacyDocument,
    policy: &SourcePolicyConfig,
    context: &ImportContext,
) -> CoreDocument {
    let mut metadata = BTreeMap::from([
        ("connector".to_string(), PARSER_FAMILY.to_string()),
        ("kind".into(), document.kind.into()),
        ("locator".into(), document.locator.clone()),
        ("legacy_rows".into(), document.legacy_rows.to_string()),
        ("legacy_snapshot".into(), context.snapshot_label.clone()),
    ]);
    if let Observed::Known { value } = &document.patch {
        metadata.insert("legacy_patch".into(), value.clone());
    }
    CoreDocument {
        logical_id: document.logical_id.clone(),
        content: document.content.clone(),
        metadata,
        origin: OriginArtifact {
            identity: SourceIdentity {
                source_id: source_id.into(),
                logical_id: document.logical_id.clone(),
            },
            source_revision: SourceRevision::Api {
                api_version: ARCHIVE_API_VERSION.into(),
                original_revision: Some(context.snapshot_label.clone()),
            },
            raw_sha256: sha256_hex(document.content.as_bytes()),
            locator: document.locator.clone(),
            parser_revision: PARSER_REVISION.into(),
            parser_family: PARSER_FAMILY.into(),
            schema_version: Observed::known(format!(
                "{ARCHIVE_API_VERSION}@{}",
                context.snapshot_label
            )),
            schema_sha256: Observed::known(context.schema_sha256.clone()),
            retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(context.snapshot_epoch)),
            source_time: document.source_time.clone(),
            language: document.language.clone(),
            origin_artifacts: document.origin_artifacts.clone(),
            derivation_family: Observed::known(document.derivation_family.clone()),
            policy: SourcePolicy {
                visibility: policy.visibility,
                allowed_scopes: policy.allowed_scopes.clone(),
                authorization_ref: Observed::unknown(UnknownReason::NotPresent),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: policy.publication_allowed,
                provider_egress_allowed: policy.provider_egress_allowed,
                raw_retention_allowed: policy.raw_retention_allowed,
            },
            validity: GameValidity {
                patch: document.patch.clone(),
                ..GameValidity::unknown()
            },
        },
    }
}

pub fn prepare_batch(
    source: &LegacySource,
    policy: &SourcePolicyConfig,
    context: &ImportContext,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    policy.validate()?;
    if source.documents.len() > MAX_DOCUMENTS_PER_SOURCE {
        return Err(invalid("legacy source exceeds document limit"));
    }
    let set = DocumentSetSource {
        source_id: source.source_id.into(),
        configuration: configuration(source.source_id, policy)?,
        visibility: policy.visibility,
        allowed_scopes: policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([
            ("connector".into(), PARSER_FAMILY.into()),
            ("legacy_snapshot".into(), context.snapshot_label.clone()),
        ]),
    };
    let documents: Vec<CoreDocument> = source
        .documents
        .iter()
        .map(|d| core_document(source.source_id, d, policy, context))
        .collect();
    prepare_document_batch(&set, &documents, previous).map_err(|e| invalid(e.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseConfig {
    pub id_prefix: String,
    pub knowledge_version: String,
    pub patch: String,
}

pub fn release_from_checkpoints(
    checkpoints: &[SourceCheckpoint],
    config: &ReleaseConfig,
    created_at_epoch: i64,
) -> Result<CorpusRelease> {
    let mut pins: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    for checkpoint in checkpoints {
        let documents = current_pins(checkpoint).map_err(|e| invalid(e.to_string()))?;
        if !documents.is_empty()
            && pins
                .insert(checkpoint.source_id.clone(), documents)
                .is_some()
        {
            return Err(invalid("duplicate source checkpoint"));
        }
    }
    let count: usize = pins.values().map(BTreeMap::len).sum();
    if count == 0 || count > 10_000 {
        return Err(invalid("release document count out of range"));
    }
    let digest = sha256_hex(
        serde_json::to_string(&(&config.knowledge_version, &config.patch, &pins))?.as_bytes(),
    );
    Ok(CorpusRelease {
        release_id: format!("{}-{}", config.id_prefix, &digest[..16]),
        knowledge_version: config.knowledge_version.clone(),
        patch: config.patch.clone(),
        created_at_epoch,
        source_revisions: pins,
    })
}

pub fn snapshot_digest(records: &[SourceRecordV2]) -> String {
    let mut sorted: Vec<&SourceRecordV2> = records.iter().collect();
    sorted.sort_by(|a, b| (&a.source_id, &a.logical_id).cmp(&(&b.source_id, &b.logical_id)));
    let mut hasher = Sha256::new();
    for record in sorted {
        hasher.update(record.source_id.as_bytes());
        hasher.update([0]);
        hasher.update(record.logical_id.as_bytes());
        hasher.update([0]);
        hasher.update(record.revision.to_le_bytes());
        hasher.update(record.content_hash.as_bytes());
        hasher.update([0]);
    }
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests;
