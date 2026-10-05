use super::{
    assemble_profile,
    compact::compact_document,
    invalid, project_entity_facts,
    semantic::{project_semantic_fact, SemanticProjection},
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
    let authorized = snapshot
        .authorized(operator, false)
        .map_err(|_| invalid("Frische Originalautorisierung fehlt"))?;
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
    let mut originals_by_record = BTreeMap::new();
    let mut verified_git = BTreeMap::new();
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
        let record = snapshot
            .revisions
            .iter()
            .find(|record| (&record.source_id, &record.logical_id, record.revision) == key)
            .ok_or_else(|| invalid("Unveränderlicher Originalpin fehlt"))?;
        if let std::collections::btree_map::Entry::Vacant(entry) = originals_by_record.entry(key) {
            let ids: Vec<_> = bindings
                .iter()
                .filter(|candidate| {
                    (
                        &candidate.source_id,
                        &candidate.logical_id,
                        candidate.store_revision,
                    ) == key
                })
                .map(|candidate| candidate.original_fact.fact_id.clone())
                .collect();
            let originals: BTreeMap<_, _> = project_entity_facts(record, &ids)?
                .into_iter()
                .map(|fact| (fact.fact_id.clone(), fact))
                .collect();
            entry.insert(originals);
        }
        let original = originals_by_record[&key]
            .get(&binding.original_fact.fact_id)
            .ok_or_else(|| invalid("Gespeicherter Originalfakt fehlt"))?;
        if *original != binding.original_fact
            || original.provenance.source_kind != ProfileSourceKind::GameFile
        {
            return Err(invalid(
                "Gespeicherte Originalbindung widerspricht dem Git-Original",
            ));
        }
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
        if let std::collections::btree_map::Entry::Vacant(entry) = verified_git.entry(key) {
            let (commit, repository, _) = git_document_identity(record)?;
            let blob = blob_map
                .get(&key)
                .ok_or_else(|| invalid("Tatsächlicher Gitblobbeleg fehlt"))?;
            let document: Value = serde_json::from_str(
                &record.metadata[crate::source_versions::DOCUMENT_METADATA_KEY],
            )?;
            if blob.git_commit != commit
                || blob.repository_url != repository
                || blob.bytes != record.content.as_bytes()
                || sha256(&blob.bytes) != record.content_hash
                || document["metadata"]["original_sha256"] != record.content_hash
            {
                return Err(invalid(
                    "Gitcommit, Upstream oder Originalblob widerspricht dem Beleg",
                ));
            }
            entry.insert((commit, repository));
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
        let mut fact = project_semantic_fact(original, semantic, record, identity)?;
        pins.push(FactPin {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
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
        original_release_id: snapshot.release.release_id.clone(),
        document_sha256: sha256(compact_document(&profile)?.as_bytes()),
        fact_pins: pins,
    };
    Ok((profile, receipt))
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
        || snapshot.release.release_id != receipt.original_release_id
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
    let (profile, mut expected) = derive_git_profile(
        &receipt.entity_key,
        snapshot,
        operator,
        bindings,
        blobs,
        live_story,
    )?;
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
