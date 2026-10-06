use super::{
    assemble_profile, compact::compact_document, invalid, project_entity_facts,
    semantic::SemanticProjection,
};
use crate::Result;
use brain_contracts::{
    entity_profile::{EntityIdentity, EntityProfile, EntityProfileFact, ProfileSourceKind},
    source::{SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    CorpusSnapshot, Principal, SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[path = "game_file_facts.rs"]
pub mod game_file_facts;

pub const GIT_DOCUMENT_CONTRACT: &str = "git-entity-document-v1";
pub const GIT_GAME_FACT_AUTHORIZATION: &str = "VON_HAUPT.md:2026-10-04T15:05:git-spielfakten;DELEGATOR-SPIELWISSEN.md:Modell-Steckbrief;A3_F1_VON_D5.md:2026-10-04T17:58";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactPin {
    pub source_id: String,
    pub logical_id: String,
    pub store_revision: u64,
    pub fact_id: String,
    pub raw_sha256: String,
    pub git_commit: String,
    pub repository_url: String,
    pub binding_identity: EntityIdentity,
    pub semantic_projection: SemanticProjection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitDocumentReceipt {
    pub contract_version: String,
    pub entity_key: String,
    pub original_release_id: String,
    pub document_sha256: String,
    pub fact_pins: Vec<FactPin>,
}

#[derive(Debug, Clone)]
pub struct StoredGitBinding {
    pub source_id: String,
    pub logical_id: String,
    pub store_revision: u64,
    pub original_fact: EntityProfileFact,
    pub binding_identity: EntityIdentity,
    pub semantic_projection: Option<SemanticProjection>,
}

#[derive(Debug, Clone)]
pub struct GitBlobEvidence {
    pub source_id: String,
    pub logical_id: String,
    pub store_revision: u64,
    pub git_commit: String,
    pub repository_url: String,
    pub bytes: Vec<u8>,
}

type ParsedGitCacheKey = (String, String, String, String, String, &'static str);

/// Nur unveränderliche Parserergebnisse innerhalb eines Importlaufs, keine Rechte.
#[derive(Debug, Default)]
pub struct GitProofBatchCache {
    parsed: BTreeMap<ParsedGitCacheKey, Arc<ParsedGitFacts>>,
    retained_bytes: usize,
}

#[derive(Debug)]
struct ParsedGitFacts {
    facts: Vec<Value>,
    by_id: BTreeMap<String, usize>,
    status: String,
}

impl GitProofBatchCache {
    pub fn new() -> Self {
        Self::default()
    }

    fn remember(&mut self, key: ParsedGitCacheKey, parsed: &Arc<ParsedGitFacts>) {
        const MAX_RETAINED_BYTES: usize = 256 * 1024 * 1024;
        fn value_bytes(value: &Value) -> usize {
            match value {
                Value::String(text) => text.capacity(),
                Value::Array(values) => values
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Value>())
                    .saturating_add(
                        values
                            .iter()
                            .map(value_bytes)
                            .fold(0usize, usize::saturating_add),
                    ),
                Value::Object(values) => values
                    .iter()
                    .map(|(key, value)| {
                        160usize
                            .saturating_add(key.capacity())
                            .saturating_add(value_bytes(value))
                    })
                    .fold(0usize, usize::saturating_add),
                _ => 0,
            }
        }
        let bytes = parsed
            .facts
            .capacity()
            .saturating_mul(std::mem::size_of::<Value>())
            .saturating_add(
                parsed
                    .facts
                    .iter()
                    .map(value_bytes)
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                parsed
                    .by_id
                    .keys()
                    .map(|key| 96usize.saturating_add(key.capacity()))
                    .fold(0usize, usize::saturating_add),
            );
        // Verdrängung ändert keine Belege. Große Ergebnisse werden bei Bedarf erneut geparst.
        if bytes > MAX_RETAINED_BYTES {
            return;
        }
        if self.retained_bytes.saturating_add(bytes) > MAX_RETAINED_BYTES {
            self.parsed.clear();
            self.retained_bytes = 0;
        }
        self.parsed.insert(key, Arc::clone(parsed));
        self.retained_bytes += bytes;
    }
}

/// Getrennte Originalansicht ohne archivierten Dokumentkörper oder Faktenbaum.
#[derive(Debug, Clone)]
pub struct OriginalEvidenceHeader {
    pub descriptor: brain_contracts::DocumentDescriptor,
    pub document_header: Value,
}

/// Vollständiger Rechtebeweis mit den für diese Entität benötigten Originalheadern.
#[derive(Debug, Clone)]
pub struct VerifiedOriginalEvidence {
    pub manifest: brain_contracts::ReleaseReadManifest,
    pub originals: Vec<OriginalEvidenceHeader>,
}

pub fn git_evidence_identity(header: &OriginalEvidenceHeader) -> Result<(String, String, String)> {
    git_header_identity(&header.document_header)
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn receipt_sha256(receipt: &GitDocumentReceipt) -> Result<String> {
    Ok(sha256(&serde_json::to_vec(receipt)?))
}

pub fn public_qualifier_text(value: &Value) -> Option<String> {
    match value {
        Value::Number(number) => Some(number.to_string()),
        Value::String(text) => {
            let characters: Vec<_> = text.chars().collect();
            (!text.trim().is_empty()
                && !text.contains(['/', '\\', ':'])
                && !text.contains("game_file")
                && !characters.iter().any(|character| character.is_control())
                && characters.iter().enumerate().all(|(index, character)| {
                    *character != '.'
                        || index > 0
                            && characters[index - 1].is_ascii_digit()
                            && characters.get(index + 1).is_some_and(char::is_ascii_digit)
                }))
            .then(|| text.clone())
        }
        _ => None,
    }
}

pub fn public_statement_qualifiers(qualifiers: &serde_json::Map<String, Value>) -> bool {
    !qualifiers.contains_key("semantic_scope")
        && ["unit", "level", "variant", "condition", "ability_name"]
            .iter()
            .all(|key| {
                qualifiers
                    .get(*key)
                    .is_none_or(|value| value.is_null() || public_qualifier_text(value).is_some())
            })
}

fn consumer_statement_qualifiers(
    entity: &EntityIdentity,
    qualifiers: &serde_json::Map<String, Value>,
) -> bool {
    public_statement_qualifiers(qualifiers)
        && ["variant", "condition", "ability_name"].iter().all(|key| {
            qualifiers
                .get(*key)
                .and_then(Value::as_str)
                .is_none_or(|text| {
                    text.to_lowercase() == entity.name.to_lowercase()
                        || !entity
                            .aliases
                            .iter()
                            .any(|alias| alias.to_lowercase() == text.to_lowercase())
                })
        })
}

pub(crate) fn consumer_patch_statement(
    change: &brain_contracts::entity_profile::PatchStoryChange,
) -> Result<brain_contracts::entity_profile::PatchStoryChange> {
    let mut clean = change.clone();
    if !public_statement_qualifiers(&clean.additional_fields) {
        clean.patch_title = None;
        clean.entity_name = None;
        clean.ability_name = None;
        clean.stat_name = None;
        clean.old_value = Value::Null;
        clean.new_value = Value::Null;
        clean.change_type = None;
        clean.numeric_direction = None;
        clean.confidence = Value::Null;
        clean.provenance.source_url = None;
        clean.provenance.evidence_ref = format!(
            "brain.patch_changes:{}",
            sha256(&serde_json::to_vec(change)?)
        );
        clean.original_line.text = None;
        clean.original_line.redistribution_allowed = false;
        clean.additional_fields.clear();
        clean.additional_fields.insert(
            "condition".into(),
            Value::String(
                "Eine belegte öffentliche Beschreibung der nötigen Bedingung oder Variante fehlt"
                    .into(),
            ),
        );
    }
    Ok(clean)
}

pub fn derived_policy() -> SourcePolicy {
    SourcePolicy {
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        authorization_ref: Observed::known(GIT_GAME_FACT_AUTHORIZATION.into()),
        license: Observed::unknown(UnknownReason::NotPresent),
        publication_allowed: true,
        provider_egress_allowed: true,
        raw_retention_allowed: false,
    }
}

pub fn git_document_identity(record: &SourceRecordV2) -> Result<(String, String, String)> {
    let document: Value = serde_json::from_str(
        record
            .metadata
            .get(crate::source_versions::DOCUMENT_METADATA_KEY)
            .ok_or_else(|| invalid("Originaldokument fehlt"))?,
    )?;
    git_header_identity(&document)
}

fn git_header_identity(document: &Value) -> Result<(String, String, String)> {
    if document["source_kind"] != "game_file" {
        return Err(invalid("Git-Ableitung benötigt eine Git-Originalquelle"));
    }
    let metadata = &document["metadata"];
    let repository = metadata["provenance"]["repository_url"]
        .as_str()
        .filter(|url| {
            matches!(
                *url,
                "https://github.com/deadlock-wiki/deadlock-data"
                    | "https://github.com/SteamTracking/GameTracking-Deadlock"
            )
        })
        .ok_or_else(|| invalid("Freigegebener Git-Upstream fehlt"))?;
    let commit = metadata["source_revision"]
        .as_str()
        .ok_or_else(|| invalid("Originalcommit fehlt"))?;
    let commit = commit.strip_prefix("git:").unwrap_or(commit);
    if commit.len() != 40 || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid("Vollständiger Gitcommit fehlt"));
    }
    let path = metadata["original_relative_path"]
        .as_str()
        .ok_or_else(|| invalid("Originalblobpfad fehlt"))?;
    Ok((commit.into(), repository.into(), path.into()))
}

pub fn derive_git_profile(
    entity_key: &str,
    snapshot: &CorpusSnapshot,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
) -> Result<(EntityProfile, GitDocumentReceipt)> {
    // Der vollständige Altvertrag prüft zusätzlich die archivierten Faktenmetadaten.
    snapshot
        .authorized(operator, false)
        .map_err(|_| invalid("Frische Originalautorisierung fehlt"))?;
    let manifest = brain_contracts::ReleaseReadManifest::from_snapshot(snapshot);
    let mut originals = Vec::new();
    let mut archived_facts = Vec::new();
    for record in &snapshot.revisions {
        let selected: Vec<_> = bindings
            .iter()
            .filter(|binding| {
                binding.source_id == record.source_id
                    && binding.logical_id == record.logical_id
                    && binding.store_revision == record.revision
            })
            .collect();
        if selected.is_empty() {
            continue;
        }
        if sha256(record.content.as_bytes()) != record.content_hash {
            return Err(invalid(
                "Archivierter Originalkörper widerspricht seinem Hash",
            ));
        }
        let ids: Vec<_> = selected
            .iter()
            .map(|binding| binding.original_fact.fact_id.clone())
            .collect();
        let projected = project_entity_facts(record, &ids)?;
        archived_facts.extend(projected.iter().cloned().map(|fact| {
            (
                record.source_id.clone(),
                record.logical_id.clone(),
                record.revision,
                fact,
            )
        }));
        if selected
            .iter()
            .zip(projected)
            .any(|(binding, original)| binding.original_fact != original)
        {
            return Err(invalid(
                "Gespeicherte Originalbindung widerspricht dem Git-Original",
            ));
        }
        let mut document: Value =
            serde_json::from_str(&record.metadata[crate::source_versions::DOCUMENT_METADATA_KEY])?;
        let object = document
            .as_object_mut()
            .ok_or_else(|| invalid("Originaldokument fehlt"))?;
        object.remove("content");
        object.remove("facts");
        originals.push(OriginalEvidenceHeader {
            descriptor: manifest
                .revisions
                .iter()
                .find(|descriptor| {
                    descriptor.head.source_id == record.source_id
                        && descriptor.head.logical_id == record.logical_id
                        && descriptor.head.revision == record.revision
                })
                .ok_or_else(|| invalid("Originaldescriptor fehlt"))?
                .clone(),
            document_header: document,
        });
    }
    derive_git_profile_evidence_core(
        entity_key,
        &VerifiedOriginalEvidence {
            manifest,
            originals,
        },
        operator,
        bindings,
        &blobs.iter().collect::<Vec<_>>(),
        live_story,
        Some(&archived_facts),
        &mut GitProofBatchCache::new(),
    )
}

pub fn verify_git_document_receipt(
    record: &SourceRecordV2,
    receipt: &GitDocumentReceipt,
    snapshot: &CorpusSnapshot,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
) -> Result<EntityProfile> {
    verify_receipt_core(record, receipt, &snapshot.release.release_id, || {
        derive_git_profile(
            &receipt.entity_key,
            snapshot,
            operator,
            bindings,
            blobs,
            live_story,
        )
    })
}

fn verify_receipt_core(
    record: &SourceRecordV2,
    receipt: &GitDocumentReceipt,
    release_id: &str,
    derive: impl FnOnce() -> Result<(EntityProfile, GitDocumentReceipt)>,
) -> Result<EntityProfile> {
    record.validate()?;
    let origin = brain_contracts::source::origin_from_record(record)
        .map_err(|_| invalid("Geprüfte eigene Dokumentpolicy fehlt"))?;
    if origin.policy != derived_policy()
        || origin.raw_sha256 != record.content_hash
        || record.metadata.keys().any(|key| {
            !matches!(
                key.as_str(),
                "brain.entity_projection.contract" | "brain.entity_projection.receipt_sha256"
            ) && key != brain_contracts::source::ORIGIN_METADATA_KEY
        })
    {
        return Err(invalid(
            "Abgeleitete Dokumentpolicy oder Metadaten widersprechen dem Vertrag",
        ));
    }
    if record.tombstone
        || record.visibility != SourceVisibility::Public
        || !record.allowed_scopes.is_empty()
        || receipt.contract_version != GIT_DOCUMENT_CONTRACT
        || release_id != receipt.original_release_id
        || record.content_hash != receipt.document_sha256
        || sha256(record.content.as_bytes()) != receipt.document_sha256
        || record
            .metadata
            .get("brain.entity_projection.contract")
            .map(String::as_str)
            != Some(GIT_DOCUMENT_CONTRACT)
        || record
            .metadata
            .get("brain.entity_projection.receipt_sha256")
            != Some(&receipt_sha256(receipt)?)
    {
        return Err(invalid(
            "Abgeleitetes Dokument widerspricht der privaten Quittung",
        ));
    }
    let (profile, mut expected) = derive()?;
    for fresh in &mut expected.fact_pins {
        if let Some(pin) = receipt.fact_pins.iter().find(|pin| {
            pin.source_id == fresh.source_id
                && pin.logical_id == fresh.logical_id
                && pin.store_revision == fresh.store_revision
                && pin.fact_id == fresh.fact_id
        }) {
            let current = &fresh.binding_identity;
            let mut previous = current.clone();
            previous.aliases = pin.binding_identity.aliases.clone();
            previous.identity_evidence = pin.binding_identity.identity_evidence.clone();
            if previous != pin.binding_identity
                || pin
                    .binding_identity
                    .aliases
                    .iter()
                    .any(|alias| !current.aliases.contains(alias))
                || pin
                    .binding_identity
                    .identity_evidence
                    .iter()
                    .any(|evidence| !current.identity_evidence.contains(evidence))
                || fresh.semantic_projection != pin.semantic_projection
            {
                return Err(invalid(
                    "Gepinnte Bindung widerspricht den frischen Originalbelegen",
                ));
            }
            fresh.binding_identity = pin.binding_identity.clone();
        }
    }
    if expected != *receipt || compact_document(&profile)? != record.content {
        return Err(invalid(
            "Gespeichertes Dokument widerspricht den frischen Originalbelegen",
        ));
    }
    Ok(profile)
}

fn matching_entity(
    entity: &EntityIdentity,
    change: &brain_contracts::entity_profile::PatchStoryChange,
) -> bool {
    let matches = |name: Option<&str>| {
        name.is_some_and(|name| {
            std::iter::once(&entity.name)
                .chain(&entity.aliases)
                .any(|candidate| candidate.to_lowercase() == name.to_lowercase())
        })
    };
    match entity.kind {
        brain_contracts::entity_profile::EntityKind::Hero => {
            change.entity_type.as_deref() == Some("hero")
                && change.ability_name.is_none()
                && matches(change.entity_name.as_deref())
        }
        brain_contracts::entity_profile::EntityKind::Item => {
            change.entity_type.as_deref() == Some("item") && matches(change.entity_name.as_deref())
        }
        brain_contracts::entity_profile::EntityKind::Ability => {
            (change.entity_type.as_deref() == Some("ability")
                && matches(change.entity_name.as_deref()))
                || (change.entity_type.as_deref() == Some("hero")
                    && matches(change.ability_name.as_deref()))
        }
    }
}

pub fn consumer_patch_story(
    entity: &EntityIdentity,
    story: &[brain_contracts::entity_profile::PatchStoryChange],
) -> Result<Vec<brain_contracts::entity_profile::PatchStoryChange>> {
    let mut result = Vec::new();
    for change in story.iter().filter(|change| {
        matching_entity(entity, change)
            || entity.kind == brain_contracts::entity_profile::EntityKind::Hero
                && change.entity_type.as_deref() == Some("hero")
                && change.entity_name.as_ref().is_some_and(|name| {
                    std::iter::once(&entity.name)
                        .chain(&entity.aliases)
                        .any(|alias| alias.to_lowercase() == name.to_lowercase())
                })
    }) {
        if change.provenance.relation != "brain.patch_changes"
            || change.provenance.evidence_ref.is_empty()
        {
            return Err(invalid("Patch-Story besitzt keinen tatsächlichen DB-Beleg"));
        }
        let mut clean = change.clone();
        if clean
            .additional_fields
            .get("ability_name")
            .and_then(Value::as_str)
            .is_some_and(|text| {
                entity
                    .aliases
                    .iter()
                    .any(|alias| alias.to_lowercase() == text.to_lowercase())
            })
        {
            clean
                .additional_fields
                .insert("ability_name".into(), Value::String(entity.name.clone()));
        }
        if !consumer_statement_qualifiers(entity, &clean.additional_fields) {
            clean
                .additional_fields
                .insert("semantic_scope".into(), Value::Bool(true));
        }
        let mut clean = consumer_patch_statement(&clean)?;
        clean.provenance.evidence_ref = format!(
            "brain.patch_changes:{}",
            sha256(&serde_json::to_vec(change)?)
        );
        let label = |value: &Option<String>| {
            value
                .as_ref()
                .filter(|value| {
                    !value.is_empty()
                        && !value.contains(['/', '\\', ':'])
                        && !value.chars().any(char::is_control)
                })
                .cloned()
        };
        clean.patch_title = label(&clean.patch_title);
        let canonical_label = |value: &Option<String>| {
            if value.as_ref().is_some_and(|name| {
                std::iter::once(&entity.name)
                    .chain(&entity.aliases)
                    .any(|alias| alias.to_lowercase() == name.to_lowercase())
            }) {
                Some(entity.name.clone())
            } else {
                label(value)
            }
        };
        clean.entity_name = canonical_label(&clean.entity_name);
        clean.ability_name = canonical_label(&clean.ability_name);
        clean.stat_name = label(&clean.stat_name);
        let numeric = |value: &Value| match value {
            Value::Number(_) => value.clone(),
            Value::String(text) if serde_json::from_str::<serde_json::Number>(text).is_ok() => {
                value.clone()
            }
            _ => Value::Null,
        };
        clean.old_value = numeric(&clean.old_value);
        clean.new_value = numeric(&clean.new_value);
        clean.confidence = numeric(&clean.confidence);
        clean.original_line.text = None;
        clean.original_line.redistribution_allowed = false;
        clean.additional_fields.retain(|key, value| {
            matches!(
                key.as_str(),
                "unit" | "level" | "variant" | "condition" | "ability_name"
            ) && public_qualifier_text(value).is_some()
        });
        if let Some(name) = clean.additional_fields.get_mut("ability_name") {
            if name.as_str().is_some_and(|name| {
                entity
                    .aliases
                    .iter()
                    .any(|alias| alias.to_lowercase() == name.to_lowercase())
            }) {
                *name = Value::String(entity.name.clone());
            }
        }
        result.push((
            clean.patch_date.clone(),
            serde_json::to_string(&clean)?,
            clean,
        ));
    }
    result.sort_by(|a, b| (&a.0, &a.2.stat_name, &a.1).cmp(&(&b.0, &b.2.stat_name, &b.1)));
    Ok(result.into_iter().map(|(_, _, change)| change).collect())
}

pub fn derive_git_profile_from_evidence(
    entity_key: &str,
    evidence: &VerifiedOriginalEvidence,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
) -> Result<(EntityProfile, GitDocumentReceipt)> {
    derive_git_profile_evidence_core(
        entity_key,
        evidence,
        operator,
        bindings,
        &blobs.iter().collect::<Vec<_>>(),
        live_story,
        None,
        &mut GitProofBatchCache::new(),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn derive_git_profile_from_evidence_cached(
    entity_key: &str,
    evidence: &VerifiedOriginalEvidence,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[&GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
    cache: &mut GitProofBatchCache,
) -> Result<(EntityProfile, GitDocumentReceipt)> {
    derive_git_profile_evidence_core(
        entity_key, evidence, operator, bindings, blobs, live_story, None, cache,
    )
}

#[allow(clippy::too_many_arguments)]
fn derive_git_profile_evidence_core(
    entity_key: &str,
    evidence: &VerifiedOriginalEvidence,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[&GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
    archived_facts: Option<&[(String, String, u64, EntityProfileFact)]>,
    cache: &mut GitProofBatchCache,
) -> Result<(EntityProfile, GitDocumentReceipt)> {
    let authorized = evidence
        .manifest
        .authorized(operator, false, false)
        .map_err(|_| invalid("Frische Originalautorisierung fehlt"))?;
    let mut headers = BTreeMap::new();
    for header in &evidence.originals {
        let descriptor = &header.descriptor;
        let key = (
            &descriptor.head.source_id,
            &descriptor.head.logical_id,
            descriptor.head.revision,
        );
        let pinned = evidence
            .manifest
            .revisions
            .iter()
            .find(|pinned| {
                (
                    &pinned.head.source_id,
                    &pinned.head.logical_id,
                    pinned.head.revision,
                ) == key
            })
            .ok_or_else(|| invalid("Originalheader ist nicht gepinnt"))?;
        if serde_json::to_value(pinned)? != serde_json::to_value(descriptor)?
            || headers.insert(key, header).is_some()
        {
            return Err(invalid("Originalheader widerspricht dem Manifest"));
        }
    }
    let mut rows: Vec<_> = bindings.iter().collect();
    rows.sort_by(|a, b| {
        (
            &a.source_id,
            &a.logical_id,
            a.store_revision,
            &a.original_fact.fact_id,
        )
            .cmp(&(
                &b.source_id,
                &b.logical_id,
                b.store_revision,
                &b.original_fact.fact_id,
            ))
    });
    let mut seen = BTreeSet::new();
    let mut blob_map = BTreeMap::new();
    for blob in blobs {
        if blob_map
            .insert(
                (&blob.source_id, &blob.logical_id, blob.store_revision),
                blob,
            )
            .is_some()
        {
            return Err(invalid("Gitblobbeleg ist nicht eindeutig"));
        }
    }
    let mut entity: Option<EntityIdentity> = None;
    let mut facts = Vec::new();
    let mut pins = Vec::new();
    let mut unknowns = BTreeSet::new();
    let mut original_facts = BTreeMap::new();
    let mut verified_git = BTreeMap::new();
    let mut blob_facts = BTreeMap::new();
    for binding in rows {
        let key = (
            &binding.source_id,
            &binding.logical_id,
            binding.store_revision,
        );
        if !seen.insert((key, &binding.original_fact.fact_id)) {
            return Err(invalid("Originalbindung ist nicht eindeutig"));
        }
        if !authorized
            .iter()
            .any(|record| (&record.source_id, &record.logical_id, record.revision) == key)
        {
            return Err(invalid("Originalbindung ist nicht mehr autorisiert"));
        }
        let header = headers
            .get(&key)
            .ok_or_else(|| invalid("Unveränderlicher Originalheader fehlt"))?;
        let record = &header.descriptor;
        let document = &header.document_header;
        let blob = blob_map
            .get(&key)
            .ok_or_else(|| invalid("Tatsächlicher Gitblobbeleg fehlt"))?;
        if let std::collections::btree_map::Entry::Vacant(entry) = verified_git.entry(key) {
            let (commit, repository, path) = git_evidence_identity(header)?;
            if document.get("content").is_some()
                || document.get("facts").is_some()
                || document["contract_version"] != "wiki-spielwissen-v1"
                || document["source_id"] != record.head.source_id
                || document["document_id"] != record.head.logical_id
                || document["content_sha256"] != record.content_hash
                || blob.git_commit != commit
                || blob.repository_url != repository
                || sha256(&blob.bytes) != record.content_hash
                || document["metadata"]["original_sha256"] != record.content_hash
            {
                return Err(invalid(
                    "Gitblob oder Originalheader widerspricht dem Beleg",
                ));
            }
            let canonical_path = document["metadata"]["relative_path"]
                .as_str()
                .ok_or_else(|| invalid("Originalressourcenpfad fehlt"))?;
            if canonical_path
                != game_file_facts::canonical_resource_path(
                    document["metadata"]["source_layout"].as_str(),
                    &path,
                    true,
                )
            {
                return Err(invalid(
                    "Originalressourcenpfad widerspricht dem Gitblobpfad",
                ));
            }
            // Der Schlüssel enthält keine Autorisierung; diese wurde oben frisch geprüft.
            let cache_key = (
                repository.clone(),
                commit.clone(),
                path.clone(),
                record.content_hash.clone(),
                canonical_path.to_owned(),
                "game-file-facts-v1",
            );
            let parsed = if let Some(parsed) = cache.parsed.get(&cache_key) {
                Arc::clone(parsed)
            } else {
                let content = std::str::from_utf8(&blob.bytes)
                    .map_err(|_| invalid("Gitblob ist kein belegter Originaltext"))?;
                let (facts, status) = game_file_facts::extract_facts(canonical_path, content);
                let mut by_id = BTreeMap::new();
                for (index, fact) in facts.iter().enumerate() {
                    let id = fact["fact_id"]
                        .as_str()
                        .ok_or_else(|| invalid("Originalparser liefert keine Faktidentität"))?;
                    if by_id.insert(id.to_owned(), index).is_some() {
                        return Err(invalid("Originalblobfeld ist nicht eindeutig"));
                    }
                }
                let parsed = Arc::new(ParsedGitFacts {
                    facts,
                    by_id,
                    status,
                });
                cache.remember(cache_key, &parsed);
                parsed
            };
            if document["metadata"]["parse_status"] != parsed.status {
                return Err(invalid("Originalparserstatus widerspricht dem Gitblob"));
            }
            blob_facts.insert(key, parsed);
            entry.insert((commit, repository));
        }
        let parsed_facts = &blob_facts[&key];
        let index = parsed_facts
            .by_id
            .get(&binding.original_fact.fact_id)
            .ok_or_else(|| invalid("Gebundenes Originalfeld fehlt im Gitblob"))?;
        let parsed = &parsed_facts.facts[*index];
        let mut reconstructed = fact_from_original_header(header, parsed)?;
        // Historische KV-Bindungen enthielten den heute ergänzten Pointer noch nicht.
        if reconstructed.predicate == "file.kv_value"
            && !binding
                .original_fact
                .qualifiers
                .contains_key("source_pointer")
        {
            reconstructed.qualifiers.remove("source_pointer");
        }
        if let Some(archived) = archived_facts
            .and_then(|facts| {
                facts.iter().find(|(source, logical, revision, fact)| {
                    (source, logical, *revision) == key
                        && fact.fact_id == binding.original_fact.fact_id
                })
            })
            .map(|(_, _, _, fact)| fact)
        {
            // Nur der vollständige Altvertrag darf zusätzlich archivierte Qualifier belegen.
            if archived.subject != reconstructed.subject
                || archived.predicate != reconstructed.predicate
                || archived.value != reconstructed.value
                || archived.evidence_status != reconstructed.evidence_status
                || archived.provenance != reconstructed.provenance
                || (archived.predicate == "file.kv_value"
                    && archived.qualifiers.contains_key("json_pointer"))
                || reconstructed
                    .qualifiers
                    .iter()
                    .any(|(key, value)| archived.qualifiers.get(key) != Some(value))
            {
                return Err(invalid(
                    "Gebundener Originalfakt widerspricht seinem Gitblobfeld",
                ));
            }
            reconstructed = archived.clone();
        }
        if reconstructed != binding.original_fact {
            return Err(invalid(
                "Gespeicherte Originalbindung widerspricht dem Git-Original",
            ));
        }
        original_facts.insert((key, binding.original_fact.fact_id.clone()), reconstructed);
        let original = &original_facts[&(key, binding.original_fact.fact_id.clone())];
        let identity = &binding.binding_identity;
        if identity.entity_key != entity_key
            || identity.name.trim().is_empty()
            || identity.name.contains(['/', '\\', ':'])
            || identity.name.chars().any(char::is_control)
            || identity.identity_evidence.is_empty()
            || entity.as_ref().is_some_and(|previous| {
                previous.kind != identity.kind || previous.name != identity.name
            })
        {
            return Err(invalid(
                "Gespeicherte Git-Bindungsidentität widerspricht der Entität",
            ));
        }
        let (commit, repository) = &verified_git[&key];
        if original.provenance.origin.raw_sha256 != record.content_hash {
            return Err(invalid(
                "Originalherkunft widerspricht dem tatsächlichen Gitblob",
            ));
        }
        let combined = entity.get_or_insert_with(|| identity.clone());
        for alias in &identity.aliases {
            if !combined.aliases.contains(alias) {
                combined.aliases.push(alias.clone());
            }
        }
        let Some(semantic) = &binding.semantic_projection else {
            continue;
        };
        if super::semantic::semantic_projection_from_verified_original(
            original,
            &semantic.relative_pointer,
            &record.head.source_id,
            &record.head.logical_id,
            identity,
            &blob_facts[&key].facts,
            &blob_facts[&key].by_id,
        )?
        .as_ref()
            != Some(semantic)
        {
            return Err(invalid(
                "Semantische Projektion widerspricht dem Originalfakt",
            ));
        }
        let mut fact = original.clone();
        fact.predicate = semantic.predicate.clone();
        fact.qualifiers = semantic.qualifiers.clone();
        fact.unit = semantic.unit.clone();
        pins.push(FactPin {
            source_id: record.head.source_id.clone(),
            logical_id: record.head.logical_id.clone(),
            store_revision: record.head.revision,
            fact_id: original.fact_id.clone(),
            raw_sha256: record.content_hash.clone(),
            git_commit: commit.clone(),
            repository_url: repository.clone(),
            binding_identity: identity.clone(),
            semantic_projection: semantic.clone(),
        });
        if fact.qualifiers.contains_key("semantic_scope") {
            unknowns.insert(format!("Für {} fehlt eine belegte öffentliche Beschreibung der verschachtelten Variante oder Bedingung", fact.predicate));
            continue;
        }
        for key in [
            "references_resolved",
            "type_flags",
            "unit_status",
            "unit_inferred",
        ] {
            fact.qualifiers.remove(key);
        }
        if fact
            .unit
            .as_ref()
            .is_some_and(|unit| public_qualifier_text(&Value::String(unit.clone())).is_none())
            || !public_statement_qualifiers(&fact.qualifiers)
            || fact.qualifiers.iter().any(|(key, value)| {
                !matches!(
                    key.as_str(),
                    "level" | "variant" | "condition" | "ability_name" | "numeric_representation"
                ) || match value {
                    Value::String(text) => {
                        text.contains(['/', '\\', ':']) || text.chars().any(char::is_control)
                    }
                    Value::Number(_) => false,
                    _ => true,
                }
            })
        {
            unknowns.insert(format!(
                "Für {} fehlt eine belegte öffentliche Beschreibung der zusätzlichen Bedingungen",
                fact.predicate
            ));
            continue;
        }
        fact.fact_id = format!("derived:{}", facts.len());
        fact.subject = entity_key.into();
        fact.provenance.origin.identity.source_id = "git-game-facts-derived".into();
        fact.provenance.origin.identity.logical_id = entity_key.into();
        fact.provenance.origin.source_revision = SourceRevision::Git {
            commit: commit.clone(),
        };
        fact.provenance.origin.policy = derived_policy();
        fact.provenance.origin.locator = "git-game-facts-derived".into();
        fact.provenance.origin.origin_artifacts.clear();
        fact.provenance.source_span = None;
        fact.provenance.document_metadata.clear();
        fact.provenance.license =
            serde_json::json!({"authorization_ref":GIT_GAME_FACT_AUTHORIZATION});
        fact.provenance.original_revision = commit.clone();
        facts.push(fact);
    }
    let mut entity = entity.ok_or_else(|| invalid("Freigegebene belegte Git-Zahlen fehlen"))?;
    facts.retain(|fact| {
        if consumer_statement_qualifiers(&entity, &fact.qualifiers) {
            true
        } else {
            unknowns.insert(format!(
                "Für {} fehlt eine belegte öffentliche Beschreibung der zusätzlichen Bedingungen",
                fact.predicate
            ));
            false
        }
    });
    let story = consumer_patch_story(&entity, live_story)?;
    entity.aliases.clear();
    entity.identity_evidence = vec![GIT_GAME_FACT_AUTHORIZATION.into()];
    let mut profile = assemble_profile(entity, None, facts, story);
    profile.unknowns.extend(unknowns);
    let receipt = GitDocumentReceipt {
        contract_version: GIT_DOCUMENT_CONTRACT.into(),
        entity_key: entity_key.into(),
        original_release_id: evidence.manifest.release.release_id.clone(),
        document_sha256: sha256(compact_document(&profile)?.as_bytes()),
        fact_pins: pins,
    };
    Ok((profile, receipt))
}

pub fn verify_git_document_receipt_from_evidence(
    record: &SourceRecordV2,
    receipt: &GitDocumentReceipt,
    evidence: &VerifiedOriginalEvidence,
    operator: &Principal,
    bindings: &[StoredGitBinding],
    blobs: &[GitBlobEvidence],
    live_story: &[brain_contracts::entity_profile::PatchStoryChange],
) -> Result<EntityProfile> {
    verify_receipt_core(
        record,
        receipt,
        &evidence.manifest.release.release_id,
        || {
            derive_git_profile_from_evidence(
                &receipt.entity_key,
                evidence,
                operator,
                bindings,
                blobs,
                live_story,
            )
        },
    )
}

fn fact_from_original_header(
    header: &OriginalEvidenceHeader,
    fact: &Value,
) -> Result<EntityProfileFact> {
    use brain_contracts::entity_profile::{PatchValidity, ProfileProvenance};
    let origin = header
        .descriptor
        .head
        .canonical_origin()
        .map_err(|_| invalid("Quellherkunft fehlt"))?
        .ok_or_else(|| invalid("Quellherkunft fehlt"))?;
    let document = &header.document_header;
    let text = |object: &Value, key: &str| {
        object[key]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| invalid("Faktenfeld fehlt"))
    };
    Ok(EntityProfileFact {
        fact_id: text(fact, "fact_id")?,
        subject: text(fact, "subject")?,
        predicate: text(fact, "predicate")?,
        value: fact["value"].clone(),
        unit: serde_json::from_value(fact["unit"].clone())?,
        qualifiers: serde_json::from_value(fact["qualifiers"].clone())?,
        evidence_status: text(fact, "evidence_status")?,
        validity: PatchValidity::Unknown {
            reason: "Quelle belegt keine Patchgrenzen".into(),
        },
        provenance: ProfileProvenance {
            source_kind: ProfileSourceKind::GameFile,
            origin,
            original_revision: text(document, "revision")?,
            observed_at: text(document, "observed_at")?,
            source_span: serde_json::from_value(fact["source_span"].clone())?,
            license: document["license"].clone(),
            document_metadata: serde_json::from_value(document["metadata"].clone())?,
        },
    })
}
