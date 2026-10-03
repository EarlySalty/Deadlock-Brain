use std::collections::{BTreeMap, BTreeSet};

use brain_contracts::{source::origin_from_record, SourceRecordV2};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::Row;
use thiserror::Error;

use crate::{pg_jobs::lock_source, ApplyOutcome, PgStore, StorageError};

pub const ORIGINAL_VERSION_KEY: &str = "wiki-spielwissen.original_revision";
pub const DOCUMENT_METADATA_KEY: &str = "wiki-spielwissen.document";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreRevision {
    OriginalWiki(u64),
    LocalMonotonic,
}

#[derive(Debug, Clone)]
pub struct VersionedSourceRecord {
    pub record: SourceRecordV2,
    pub original_revision: String,
    pub revision: StoreRevision,
    pub fact_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionProof {
    pub source_id: String,
    pub document_id: String,
    pub original_revision: String,
    pub store_revision: u64,
    pub locally_allocated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportConflict {
    pub source_id: String,
    pub document_id: String,
    pub original_revision: String,
    pub store_revision: u64,
    pub reason: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct VersionImportSummary {
    pub committed: bool,
    pub inserted: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub conflicts: Vec<ImportConflict>,
    pub skipped_reasons: BTreeMap<String, usize>,
    pub source_count: usize,
    pub document_count: usize,
    pub version_count: usize,
    pub fact_count: usize,
    pub versions: Vec<VersionProof>,
}

#[derive(Debug, Error)]
pub enum VersionImportError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("Ungültiger Versionsimport: {0}")]
    Invalid(String),
}

impl From<sqlx::Error> for VersionImportError {
    fn from(error: sqlx::Error) -> Self {
        Self::Storage(StorageError::Database(error))
    }
}

impl From<serde_json::Error> for VersionImportError {
    fn from(error: serde_json::Error) -> Self {
        Self::Storage(StorageError::Json(error))
    }
}

fn semantic_record(record: &SourceRecordV2) -> Result<serde_json::Value, VersionImportError> {
    let mut value = serde_json::to_value(record)?;
    value["revision"] = serde_json::Value::Null;
    if let Some(metadata) = value["metadata"].as_object_mut() {
        metadata.remove("wiki-spielwissen.store_revision_kind");
        for key in [
            DOCUMENT_METADATA_KEY,
            brain_contracts::source::ORIGIN_METADATA_KEY,
        ] {
            if let Some(encoded) = metadata.get(key).and_then(serde_json::Value::as_str) {
                let mut decoded: serde_json::Value = serde_json::from_str(encoded)?;
                if key == DOCUMENT_METADATA_KEY {
                    if let Some(object) = decoded.as_object_mut() {
                        object.remove("observed_at");
                    }
                } else if let Some(object) = decoded["data"].as_object_mut() {
                    object.remove("retrieved_at");
                    if object
                        .get("source_revision")
                        .is_some_and(|revision| revision["kind"] == "wiki")
                    {
                        if let Some((encoded, original)) = record
                            .metadata
                            .get(DOCUMENT_METADATA_KEY)
                            .zip(record.metadata.get(ORIGINAL_VERSION_KEY))
                        {
                            let document: serde_json::Value = serde_json::from_str(encoded)?;
                            if document["contract_version"].as_str() == Some("wiki-spielwissen-v1")
                            {
                                object.insert(
                                    "source_revision".into(),
                                    serde_json::json!({
                                        "kind": "api", "api_version": "wiki-spielwissen-v1",
                                        "original_revision": original,
                                    }),
                                );
                            }
                        }
                    }
                }
                metadata.insert(key.into(), decoded);
            }
        }
    }
    Ok(value)
}

fn validate(input: &VersionedSourceRecord) -> Result<(), VersionImportError> {
    if input.original_revision.trim().is_empty()
        || input.original_revision.chars().any(char::is_control)
        || input.record.tombstone
        || input.record.metadata.get(ORIGINAL_VERSION_KEY) != Some(&input.original_revision)
        || !input.record.metadata.contains_key(DOCUMENT_METADATA_KEY)
    {
        return Err(VersionImportError::Invalid("Versionsherkunft fehlt".into()));
    }
    input.record.validate().map_err(StorageError::from)?;
    let origin = origin_from_record(&input.record).map_err(VersionImportError::Invalid)?;
    let document: serde_json::Value = serde_json::from_str(
        input
            .record
            .metadata
            .get(DOCUMENT_METADATA_KEY)
            .ok_or_else(|| VersionImportError::Invalid("Dokumentnachweis fehlt".into()))?,
    )?;
    if document["revision"].as_str() != Some(input.original_revision.as_str())
        || document["source_id"].as_str() != Some(input.record.source_id.as_str())
        || document["document_id"].as_str() != Some(input.record.logical_id.as_str())
        || document["content"].as_str() != Some(input.record.content.as_str())
        || document["content_sha256"].as_str() != Some(input.record.content_hash.as_str())
        || document["facts"].as_array().map(Vec::len) != Some(input.fact_count)
        || !origin.policy.raw_retention_allowed
        || !input
            .record
            .metadata
            .get("wiki-spielwissen.provenance_evidence_ref")
            .is_some_and(|value| !value.trim().is_empty() && !value.chars().any(char::is_control))
        || !matches!(&origin.policy.authorization_ref, brain_contracts::value::Observed::Known { value } if !value.trim().is_empty() && !value.chars().any(char::is_control))
    {
        return Err(VersionImportError::Invalid(
            "Dokumentnachweis oder Operatorfreigabe passt nicht".into(),
        ));
    }
    let redistribution = document["license"]["redistribution_allowed"].as_bool() == Some(true)
        && document["license"]["name"].as_str().is_some_and(|name| {
            !name.trim().is_empty() && !name.trim().eq_ignore_ascii_case("unverified")
        });
    if (!redistribution
        && (origin.policy.publication_allowed || origin.policy.provider_egress_allowed))
        || (input.record.visibility == brain_contracts::SourceVisibility::Public
            && !origin.policy.publication_allowed)
    {
        return Err(VersionImportError::Invalid(
            "Lizenz erlaubt diese Weitergabe nicht".into(),
        ));
    }
    if let brain_contracts::source::SourceRevision::Api {
        original_revision, ..
    } = &origin.source_revision
    {
        if original_revision.as_deref() != Some(input.original_revision.as_str()) {
            return Err(VersionImportError::Invalid(
                "Originalrevision stimmt nicht mit der Herkunft überein".into(),
            ));
        }
    }
    let numeric_wiki = document["source_kind"].as_str() == Some("wiki")
        && input
            .original_revision
            .bytes()
            .all(|byte| byte.is_ascii_digit());
    let wiki_revision = input
        .original_revision
        .parse::<u64>()
        .ok()
        .filter(|revision| numeric_wiki && *revision > 0 && *revision <= i64::MAX as u64);
    match (&origin.source_revision, input.revision) {
        (
            brain_contracts::source::SourceRevision::Wiki {
                page_id,
                revision_id,
            },
            StoreRevision::OriginalWiki(revision),
        ) if wiki_revision == Some(revision)
            && u64::try_from(*revision_id).ok() == Some(revision)
            && input.record.revision == revision
            && input.record.logical_id
                == format!("wiki:{}:page:{}", input.record.source_id, page_id) => {}
        (
            brain_contracts::source::SourceRevision::Api { api_version, .. },
            StoreRevision::OriginalWiki(revision),
        ) if wiki_revision == Some(revision)
            && api_version == "wiki-spielwissen-v1"
            && document["contract_version"].as_str() == Some(api_version.as_str())
            && document["source_locator"].as_str() == Some(origin.locator.as_str())
            && (input.record.logical_id
                == format!(
                    "wiki:{}:url:{:x}",
                    input.record.source_id,
                    Sha256::digest(origin.locator.as_bytes())
                )
                || input
                    .record
                    .logical_id
                    .strip_prefix(&format!("wiki:{}:page:", input.record.source_id))
                    .and_then(|page| page.parse::<i64>().ok())
                    .is_some_and(|page| page > 0)) => {}
        (_, StoreRevision::LocalMonotonic)
            if !numeric_wiki
                && !matches!(
                    &origin.source_revision,
                    brain_contracts::source::SourceRevision::Wiki { .. }
                ) => {}
        _ => {
            return Err(VersionImportError::Invalid(
                "Wiki- und Store-Revision stimmen nicht überein".into(),
            ));
        }
    }
    semantic_record(&input.record)?;
    Ok(())
}

impl PgStore {
    pub async fn import_source_versions(
        &self,
        inputs: &[VersionedSourceRecord],
    ) -> Result<VersionImportSummary, VersionImportError> {
        if inputs.len() > 10_000 {
            return Err(VersionImportError::Invalid(
                "Batch überschreitet 10.000 Dokumentversionen".into(),
            ));
        }
        for input in inputs {
            validate(input)?;
        }
        let sources: BTreeSet<_> = inputs
            .iter()
            .map(|input| input.record.source_id.clone())
            .collect();
        let documents: BTreeSet<_> = inputs
            .iter()
            .map(|input| {
                (
                    input.record.source_id.clone(),
                    input.record.logical_id.clone(),
                )
            })
            .collect();
        let versions: BTreeSet<_> = inputs
            .iter()
            .map(|input| {
                (
                    input.record.source_id.clone(),
                    input.record.logical_id.clone(),
                    input.original_revision.clone(),
                )
            })
            .collect();
        let mut summary = VersionImportSummary {
            source_count: sources.len(),
            document_count: documents.len(),
            version_count: versions.len(),
            ..VersionImportSummary::default()
        };
        let mut ordered: Vec<_> = inputs.iter().collect();
        ordered.sort_by(|a, b| {
            (&a.record.source_id, &a.record.logical_id)
                .cmp(&(&b.record.source_id, &b.record.logical_id))
        });
        let mut tx = self.pool.begin().await?;
        for source in sources {
            lock_source(&mut tx, &source).await?;
        }
        let mut history: BTreeMap<(String, String), Vec<SourceRecordV2>> = BTreeMap::new();
        let mut persisted = BTreeSet::new();
        let mut provisional_unchanged = Vec::new();
        let mut counted_facts = BTreeSet::new();
        let mut pending = Vec::new();
        for input in ordered {
            let key = (
                input.record.source_id.clone(),
                input.record.logical_id.clone(),
            );
            if !history.contains_key(&key) {
                let lock_key = serde_json::to_string(&(&key.0, &key.1))?;
                sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
                    .bind(lock_key)
                    .execute(&mut *tx)
                    .await?;
                let rows = sqlx::query("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 ORDER BY revision")
                    .bind(&key.0).bind(&key.1).fetch_all(&mut *tx).await?;
                let mut records = Vec::new();
                for row in rows {
                    let record: SourceRecordV2 = serde_json::from_value(
                        row.try_get::<serde_json::Value, _>("record_json")?,
                    )?;
                    persisted.insert((
                        record.source_id.clone(),
                        record.logical_id.clone(),
                        record.revision,
                    ));
                    records.push(record);
                }
                history.insert(key.clone(), records);
            }
            let records = history
                .get_mut(&key)
                .ok_or_else(|| VersionImportError::Invalid("Versionsbestand fehlt".into()))?;
            let mut existing = None;
            for record in records.iter() {
                if matches_original_revision(record, input)? {
                    existing = Some(record);
                    break;
                }
            }
            if let Some(existing) = existing {
                if semantic_record(existing)? == semantic_record(&input.record)? {
                    let locally_allocated = existing
                        .metadata
                        .get("wiki-spielwissen.store_revision_kind")
                        .is_some_and(|kind| kind == "local_monotonic")
                        || input.revision == StoreRevision::LocalMonotonic;
                    let version_proof = proof(input, existing.revision, locally_allocated);
                    if persisted.contains(&(
                        existing.source_id.clone(),
                        existing.logical_id.clone(),
                        existing.revision,
                    )) {
                        summary.unchanged += 1;
                        if counted_facts.insert((
                            key.0.clone(),
                            key.1.clone(),
                            input.original_revision.clone(),
                        )) {
                            summary.fact_count += input.fact_count;
                        }
                        summary.versions.push(version_proof);
                    } else {
                        provisional_unchanged.push(version_proof);
                    }
                } else {
                    summary.conflicts.push(conflict(
                        input,
                        existing.revision,
                        "original_revision_content_or_facts_conflict",
                    ));
                }
                continue;
            }
            let maximum = records
                .iter()
                .map(|record| record.revision)
                .max()
                .unwrap_or(0);
            let revision = maximum
                .checked_add(1)
                .filter(|revision| *revision <= i64::MAX as u64)
                .ok_or_else(|| {
                    VersionImportError::Invalid("Store-Revision ist ausgeschöpft".into())
                })?;
            let mut publish_head = true;
            if let StoreRevision::OriginalWiki(incoming) = input.revision {
                for record in records.iter() {
                    if known_wiki_revision(record)?.is_some_and(|known| known > incoming) {
                        publish_head = false;
                    }
                }
            }
            let mut record = input.record.clone();
            let mut origin = origin_from_record(&record).map_err(VersionImportError::Invalid)?;
            if matches!(
                origin.source_revision,
                brain_contracts::source::SourceRevision::Wiki { .. }
            ) {
                origin.source_revision = brain_contracts::source::SourceRevision::Api {
                    api_version: "wiki-spielwissen-v1".into(),
                    original_revision: Some(input.original_revision.clone()),
                };
            }
            record.revision = revision;
            record.metadata.insert(
                "wiki-spielwissen.store_revision_kind".into(),
                "local_monotonic".into(),
            );
            origin
                .bind_record(&mut record)
                .map_err(VersionImportError::Invalid)?;
            records.push(record.clone());
            pending.push((
                record,
                proof(input, revision, true),
                input.fact_count,
                publish_head,
            ));
        }
        if !summary.conflicts.is_empty() {
            summary.skipped_reasons.insert(
                "atomic_batch_conflict".into(),
                pending.len() + provisional_unchanged.len(),
            );
            tx.rollback().await?;
            return Ok(summary);
        }
        for (record, proof, facts, publish_head) in pending {
            let outcome = if publish_head {
                Self::apply_connection(&mut tx, &record).await?
            } else {
                ApplyOutcome::IgnoredStale
            };
            match outcome {
                ApplyOutcome::Inserted => summary.inserted += 1,
                ApplyOutcome::Updated => summary.updated += 1,
                ApplyOutcome::Unchanged => summary.unchanged += 1,
                ApplyOutcome::IgnoredStale => {
                    let record_json = serde_json::to_value(&record)?;
                    sqlx::query("INSERT INTO brain.source_record_revisions (source_id, logical_id, revision, content_hash, tombstone, record_json) VALUES ($1,$2,$3,$4,false,$5)")
                        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
                        .bind(&record.content_hash).bind(record_json).execute(&mut *tx).await?;
                    summary.updated += 1;
                }
                ApplyOutcome::Tombstoned => {
                    return Err(VersionImportError::Invalid(
                        "Versionsimport wurde nicht vollständig angewendet".into(),
                    ));
                }
            }
            summary.fact_count += facts;
            summary.versions.push(proof);
        }
        tx.commit().await?;
        summary.unchanged += provisional_unchanged.len();
        summary.versions.extend(provisional_unchanged);
        summary.committed = true;
        Ok(summary)
    }
}

fn matches_original_revision(
    record: &SourceRecordV2,
    input: &VersionedSourceRecord,
) -> Result<bool, VersionImportError> {
    if let Some(original) = record.metadata.get(ORIGINAL_VERSION_KEY) {
        return Ok(original == &input.original_revision);
    }
    if let StoreRevision::OriginalWiki(original) = input.revision {
        return Ok(known_wiki_revision(record)? == Some(original));
    }
    Ok(false)
}

fn known_wiki_revision(record: &SourceRecordV2) -> Result<Option<u64>, VersionImportError> {
    if let Some(encoded) = record.metadata.get(DOCUMENT_METADATA_KEY) {
        let document: serde_json::Value = serde_json::from_str(encoded)?;
        if document["source_kind"].as_str() == Some("wiki") {
            return Ok(document["revision"].as_str().and_then(|revision| {
                revision
                    .bytes()
                    .all(|byte| byte.is_ascii_digit())
                    .then(|| revision.parse::<u64>().ok())
                    .flatten()
            }));
        }
    }
    if record
        .metadata
        .contains_key(brain_contracts::source::ORIGIN_METADATA_KEY)
    {
        let origin = origin_from_record(record).map_err(VersionImportError::Invalid)?;
        match origin.source_revision {
            brain_contracts::source::SourceRevision::Wiki { revision_id, .. } => {
                return Ok(u64::try_from(revision_id).ok());
            }
            brain_contracts::source::SourceRevision::Api {
                api_version,
                original_revision,
            } if api_version == "wiki-spielwissen-v1"
                && record
                    .logical_id
                    .starts_with(&format!("wiki:{}:", record.source_id)) =>
            {
                return Ok(original_revision.and_then(|original| {
                    original
                        .bytes()
                        .all(|byte| byte.is_ascii_digit())
                        .then(|| original.parse::<u64>().ok())
                        .flatten()
                }));
            }
            _ => {}
        }
    }
    Ok(None)
}

fn proof(input: &VersionedSourceRecord, revision: u64, locally_allocated: bool) -> VersionProof {
    VersionProof {
        source_id: input.record.source_id.clone(),
        document_id: input.record.logical_id.clone(),
        original_revision: input.original_revision.clone(),
        store_revision: revision,
        locally_allocated,
    }
}

fn conflict(input: &VersionedSourceRecord, revision: u64, reason: &str) -> ImportConflict {
    ImportConflict {
        source_id: input.record.source_id.clone(),
        document_id: input.record.logical_id.clone(),
        original_revision: input.original_revision.clone(),
        store_revision: revision,
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::SourceVisibility;
    use serde_json::json;

    fn record(observed: &str, fact_value: u64, unit: &str) -> SourceRecordV2 {
        SourceRecordV2 {
            source_id: "fixture".into(),
            logical_id: "wiki:fixture:page:123".into(),
            revision: 1,
            content_hash: "a".repeat(64),
            content: "content".into(),
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::from(["source.review:fixture".into()]),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::from([
                (ORIGINAL_VERSION_KEY.into(), "build-123".into()),
                (DOCUMENT_METADATA_KEY.into(), json!({"observed_at": observed, "facts": [{"value": fact_value, "unit": unit, "qualifiers": {"mode": "test"}}]}).to_string()),
                (brain_contracts::source::ORIGIN_METADATA_KEY.into(), json!({"contract_version": "brain.ir.v1", "data": {"retrieved_at": observed, "parser_revision": "parser-v1"}}).to_string()),
            ]),
        }
    }

    fn wiki_version(
        url_identity: bool,
        revision: &str,
        content: &str,
        observed: i64,
    ) -> VersionedSourceRecord {
        use brain_contracts::source::{
            GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision,
            SourceTimestamp,
        };
        use brain_contracts::value::{Observed, UnknownReason};

        let locator = "https://example.org/Page";
        let mut record = record("2026-10-03T12:00:00Z", 12, "seconds");
        record.logical_id = if url_identity {
            format!("wiki:fixture:url:{:x}", Sha256::digest(locator.as_bytes()))
        } else {
            "wiki:fixture:page:123".into()
        };
        record.revision = revision.parse().unwrap_or(1);
        record.content = content.into();
        record.content_hash = format!("{:x}", Sha256::digest(content.as_bytes()));
        record.metadata = BTreeMap::from([
            (ORIGINAL_VERSION_KEY.into(), revision.into()),
            (DOCUMENT_METADATA_KEY.into(), json!({
                "contract_version": "wiki-spielwissen-v1", "source_kind": "wiki", "source_id": record.source_id,
                "document_id": record.logical_id, "source_locator": locator, "title": "Page", "language": "und",
                "revision": revision, "observed_at": if observed == 1_791_028_800 { "2026-10-03T12:00:00Z" } else { "2026-10-04T12:00:00Z" }, "content": content,
                "content_sha256": record.content_hash, "evidence_status": "source_statement",
                "license": {"name": "unverified", "url": null, "attribution": "Fixture", "redistribution_allowed": false},
                "metadata": {}, "facts": [],
            }).to_string()),
            ("wiki-spielwissen.provenance_evidence_ref".into(), "evidence:fixture".into()),
        ]);
        let numeric_revision = revision.parse::<i64>().ok();
        let origin = OriginArtifact {
            identity: SourceIdentity {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
            },
            source_revision: match (url_identity, numeric_revision) {
                (false, Some(revision_id)) => SourceRevision::Wiki {
                    page_id: 123,
                    revision_id,
                },
                _ => SourceRevision::Api {
                    api_version: "wiki-spielwissen-v1".into(),
                    original_revision: Some(revision.into()),
                },
            },
            raw_sha256: record.content_hash.clone(),
            locator: locator.into(),
            parser_revision: "parser-v1".into(),
            parser_family: "dbrain-sources/wiki-spielwissen".into(),
            schema_version: Observed::known("wiki-spielwissen-v1".into()),
            schema_sha256: Observed::unknown(UnknownReason::NotPresent),
            retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(observed)),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::unknown(UnknownReason::NotPresent),
            origin_artifacts: BTreeSet::from([format!(
                "fixture:{}:{}",
                record.logical_id, revision
            )]),
            derivation_family: Observed::known("wiki-spielwissen-v1".into()),
            policy: SourcePolicy {
                visibility: record.visibility,
                allowed_scopes: record.allowed_scopes.clone(),
                authorization_ref: Observed::known("operator:fixture".into()),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: false,
                provider_egress_allowed: false,
                raw_retention_allowed: true,
            },
            validity: GameValidity::unknown(),
        };
        origin.bind_record(&mut record).unwrap();
        VersionedSourceRecord {
            record,
            original_revision: revision.into(),
            revision: numeric_revision.map_or(StoreRevision::LocalMonotonic, |revision| {
                StoreRevision::OriginalWiki(revision as u64)
            }),
            fact_count: 0,
        }
    }

    fn change_document(input: &mut VersionedSourceRecord, field: &str, value: serde_json::Value) {
        let mut document: serde_json::Value =
            serde_json::from_str(&input.record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document[field] = value;
        input
            .record
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
    }

    #[test]
    fn original_wiki_order_accepts_page_and_url_origins() {
        for url_identity in [false, true] {
            for revision in ["123", "456", "9223372036854775807"] {
                let version = wiki_version(url_identity, revision, "content", 1_791_028_800);
                validate(&version).unwrap();
                assert!(!proof(&version, version.record.revision, false).locally_allocated);
            }
        }
    }

    #[test]
    fn url_wiki_repeat_ignores_observation_but_detects_content_conflict() {
        let first = wiki_version(true, "456", "Aktueller Inhalt", 1_791_028_800);
        let repeated = wiki_version(true, "456", "Aktueller Inhalt", 1_791_115_200);
        let conflicting = wiki_version(true, "456", "Anderer Inhalt", 1_791_115_200);
        for version in [&first, &repeated, &conflicting] {
            validate(version).unwrap();
        }
        assert_eq!(
            semantic_record(&first.record).unwrap(),
            semantic_record(&repeated.record).unwrap()
        );
        assert_ne!(
            semantic_record(&first.record).unwrap(),
            semantic_record(&conflicting.record).unwrap()
        );
    }

    #[test]
    fn numeric_wiki_cannot_use_local_order_or_mismatched_original_revision() {
        for url_identity in [false, true] {
            let mut version = wiki_version(url_identity, "456", "content", 1_791_028_800);
            version.revision = StoreRevision::LocalMonotonic;
            assert!(validate(&version).is_err());
            let mut version = wiki_version(url_identity, "456", "content", 1_791_028_800);
            version.revision = StoreRevision::OriginalWiki(123);
            assert!(validate(&version).is_err());
        }
        let mut version = wiki_version(true, "456", "content", 1_791_028_800);
        let mut origin = origin_from_record(&version.record).unwrap();
        origin.source_revision = brain_contracts::source::SourceRevision::Api {
            api_version: "wiki-spielwissen-v1".into(),
            original_revision: Some("123".into()),
        };
        origin.bind_record(&mut version.record).unwrap();
        assert!(validate(&version).is_err());
    }

    #[test]
    fn original_wiki_api_order_requires_matching_url_identity_and_contract() {
        for (field, value) in [
            ("source_kind", json!("game_file")),
            ("source_locator", json!("https://example.org/Other")),
            ("contract_version", json!("other-v1")),
        ] {
            let mut version = wiki_version(true, "456", "content", 1_791_028_800);
            change_document(&mut version, field, value);
            assert!(validate(&version).is_err());
        }
        let mut version = wiki_version(false, "456", "content", 1_791_028_800);
        let mut origin = origin_from_record(&version.record).unwrap();
        origin.source_revision = brain_contracts::source::SourceRevision::Api {
            api_version: "wiki-spielwissen-v1".into(),
            original_revision: Some("456".into()),
        };
        origin.bind_record(&mut version.record).unwrap();
        validate(&version).unwrap();
        let mut version = wiki_version(true, "456", "content", 1_791_028_800);
        let mut origin = origin_from_record(&version.record).unwrap();
        origin.locator = "https://example.org/Other".into();
        change_document(&mut version, "source_locator", json!(origin.locator));
        origin.bind_record(&mut version.record).unwrap();
        assert!(validate(&version).is_err());
        let mut version = wiki_version(false, "456", "content", 1_791_028_800);
        let mut origin = origin_from_record(&version.record).unwrap();
        origin.source_revision = brain_contracts::source::SourceRevision::Wiki {
            page_id: 999,
            revision_id: 456,
        };
        origin.bind_record(&mut version.record).unwrap();
        assert!(validate(&version).is_err());
    }

    #[test]
    fn api_git_and_unknown_versions_cannot_claim_original_wiki_order() {
        for revision in [
            "abcdef0123456789abcdef0123456789abcdef01".to_string(),
            format!("unknown:{:x}", Sha256::digest(b"content")),
        ] {
            let mut version = wiki_version(true, &revision, "content", 1_791_028_800);
            validate(&version).unwrap();
            assert!(proof(&version, 1, true).locally_allocated);
            version.revision = StoreRevision::OriginalWiki(456);
            version.record.revision = 456;
            assert!(validate(&version).is_err());
        }
    }

    #[tokio::test]
    #[ignore = "requires isolated PostgreSQL Unix socket: BRAIN_CORE_TEST_PG_SOCKET"]
    async fn postgres_original_wiki_history_does_not_reset_heads_or_release_pins() {
        use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

        let socket =
            std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("explicit scratch socket required");
        assert!(socket.ends_with("/.core-test-pg"));
        assert!(std::path::Path::new(&socket).is_absolute());
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect_with(
                PgConnectOptions::new()
                    .host(&socket)
                    .port(55439)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(address.is_none());
        let user: String = sqlx::query_scalar("SELECT current_user::text")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(user, "brain_core_test");
        sqlx::raw_sql("DROP SCHEMA IF EXISTS brain CASCADE")
            .execute(&pool)
            .await
            .unwrap();
        let store = PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        for url_identity in [false, true] {
            let mut incoming = wiki_version(url_identity, "456", "Altbestand", 1_791_028_800);
            let source = format!("legacy-{url_identity}");
            let mut origin = origin_from_record(&incoming.record).unwrap();
            incoming.record.source_id = source.clone();
            incoming.record.logical_id = if url_identity {
                format!(
                    "wiki:{source}:url:{:x}",
                    Sha256::digest(origin.locator.as_bytes())
                )
            } else {
                format!("wiki:{source}:page:123")
            };
            change_document(&mut incoming, "source_id", json!(source));
            let id = incoming.record.logical_id.clone();
            change_document(&mut incoming, "document_id", json!(id));
            origin.identity.source_id = incoming.record.source_id.clone();
            origin.identity.logical_id = incoming.record.logical_id.clone();
            origin.bind_record(&mut incoming.record).unwrap();
            let mut legacy = incoming.record.clone();
            legacy.metadata.remove(ORIGINAL_VERSION_KEY);
            legacy.metadata.remove(DOCUMENT_METADATA_KEY);
            store.apply(&legacy).await.unwrap();
            assert_eq!(known_wiki_revision(&legacy).unwrap(), Some(456));
            assert!(matches_original_revision(&legacy, &incoming).unwrap());
            let rejected = store
                .import_source_versions(std::slice::from_ref(&incoming))
                .await
                .unwrap();
            assert!(!rejected.committed);
            assert_eq!(rejected.conflicts.len(), 1);
            let head: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
                .bind(&legacy.source_id).bind(&legacy.logical_id).fetch_one(&pool).await.unwrap();
            assert_eq!(head, serde_json::to_value(&legacy).unwrap());
        }
        for url_identity in [false, true] {
            let newer = wiki_version(url_identity, "456", "Aktueller Inhalt", 1_791_028_800);
            let older = wiki_version(url_identity, "123", "Älterer Inhalt", 1_791_115_200);
            store.apply(&newer.record).await.unwrap();
            let first = store
                .import_source_versions(std::slice::from_ref(&newer))
                .await
                .unwrap();
            assert!(first.committed);
            assert_eq!(first.unchanged, 1);
            assert_eq!(first.versions[0].store_revision, 456);
            assert!(!first.versions[0].locally_allocated);
            let release = brain_contracts::CorpusRelease {
                release_id: format!("wiki-version-order-{url_identity}"),
                knowledge_version: "v1".into(),
                patch: "unknown".into(),
                created_at_epoch: 0,
                source_revisions: BTreeMap::from([(
                    "fixture".into(),
                    BTreeMap::from([(newer.record.logical_id.clone(), 456)]),
                )]),
            };
            store.publish_release(&release).await.unwrap();
            let before = store.snapshot(&release.release_id).await.unwrap();
            let historical = store
                .import_source_versions(std::slice::from_ref(&older))
                .await
                .unwrap();
            assert!(historical.committed);
            assert!(historical.conflicts.is_empty());
            assert_eq!(historical.versions[0].store_revision, 457);
            assert!(historical.versions[0].locally_allocated);
            let revisions: Vec<i64> = sqlx::query_scalar("SELECT revision FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 ORDER BY revision")
                .bind(&newer.record.source_id).bind(&newer.record.logical_id).fetch_all(&pool).await.unwrap();
            assert_eq!(revisions, vec![456, 457]);
            assert_eq!(store.snapshot(&release.release_id).await.unwrap(), before);
            assert_eq!(before.heads[0].revision, 456);
            assert_eq!(before.heads[0].content, "Aktueller Inhalt");
            assert_eq!(before.revisions[0].revision, 456);
            let mut repeated = wiki_version(url_identity, "456", "Aktueller Inhalt", 1_791_115_200);
            let mut origin = origin_from_record(&repeated.record).unwrap();
            origin.source_revision = brain_contracts::source::SourceRevision::Api {
                api_version: "wiki-spielwissen-v1".into(),
                original_revision: Some("456".into()),
            };
            repeated.record.revision = 1;
            origin.bind_record(&mut repeated.record).unwrap();
            let replay = store
                .import_source_versions(&[repeated, older])
                .await
                .unwrap();
            assert!(replay.committed);
            assert_eq!(replay.unchanged, 2);
            assert_eq!(replay.inserted + replay.updated, 0);
            let conflicting = wiki_version(url_identity, "456", "Anderer Inhalt", 1_791_115_200);
            let rejected = store.import_source_versions(&[conflicting]).await.unwrap();
            assert!(!rejected.committed);
            assert_eq!(rejected.conflicts.len(), 1);
            assert_eq!(
                rejected.conflicts[0].reason,
                "original_revision_content_or_facts_conflict"
            );
            assert_eq!(store.snapshot(&release.release_id).await.unwrap(), before);
            let unknown = wiki_version(
                url_identity,
                &format!("unknown:{:x}", Sha256::digest(b"unknown")),
                "unknown",
                1_791_115_200,
            );
            let unknown_import = store
                .import_source_versions(std::slice::from_ref(&unknown))
                .await
                .unwrap();
            assert!(unknown_import.committed);
            assert_eq!(unknown_import.versions[0].store_revision, 458);
            let older_unknown = wiki_version(url_identity, "122", "Noch älter", 1_791_115_200);
            assert!(
                store
                    .import_source_versions(std::slice::from_ref(&older_unknown))
                    .await
                    .unwrap()
                    .committed
            );
            let unknown_snapshot = store.snapshot(&release.release_id).await.unwrap();
            assert_eq!(unknown_snapshot.heads[0].content, "unknown");
            let next = wiki_version(url_identity, "458", "Neuere Wiki-Version", 1_791_115_200);
            let next_import = store
                .import_source_versions(std::slice::from_ref(&next))
                .await
                .unwrap();
            assert!(next_import.committed);
            assert!(next_import.conflicts.is_empty());
            assert_eq!(next_import.versions[0].store_revision, 460);
            let late = wiki_version(
                url_identity,
                "457",
                "Verspätete Wiki-Version",
                1_791_115_200,
            );
            let local = wiki_version(
                url_identity,
                "f000000000000000000000000000000000000000",
                "Lokaler Kopf",
                1_791_115_200,
            );
            assert!(
                store
                    .import_source_versions(std::slice::from_ref(&local))
                    .await
                    .unwrap()
                    .committed
            );
            assert!(
                store
                    .import_source_versions(std::slice::from_ref(&late))
                    .await
                    .unwrap()
                    .committed
            );
            let snapshot = store.snapshot(&release.release_id).await.unwrap();
            assert_eq!(snapshot.release, before.release);
            assert_eq!(snapshot.revisions, before.revisions);
            assert_eq!(snapshot.heads[0].content, "Lokaler Kopf");
            let replay = store
                .import_source_versions(&[unknown, next, late, local])
                .await
                .unwrap();
            assert!(replay.committed);
            assert_eq!(replay.unchanged, 4);
            assert_eq!(replay.inserted + replay.updated, 0);
            assert_eq!(store.snapshot(&release.release_id).await.unwrap(), snapshot);
            let fresh = wiki_version(url_identity, "459", "Nicht committen", 1_791_115_200);
            let conflict = wiki_version(url_identity, "456", "Konflikt", 1_791_115_200);
            let rejected = store
                .import_source_versions(&[fresh, conflict])
                .await
                .unwrap();
            assert!(!rejected.committed);
            assert_eq!(rejected.conflicts.len(), 1);
            assert_eq!(store.snapshot(&release.release_id).await.unwrap(), snapshot);
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2")
                .bind(&newer.record.source_id).bind(&newer.record.logical_id).fetch_one(&pool).await.unwrap();
            assert_eq!(count, 7);
        }
        for (game, originals) in [
            (true, ["9", "10"]),
            (
                false,
                [
                    "f000000000000000000000000000000000000000",
                    "a000000000000000000000000000000000000000",
                ],
            ),
        ] {
            let make = |revision: &str, content: &str| {
                let mut version = wiki_version(true, revision, content, 1_791_028_800);
                let mut origin = origin_from_record(&version.record).unwrap();
                version.record.source_id = format!("ordered-{game}");
                version.record.logical_id = if game {
                    "game:1422450:scripts/test.txt".into()
                } else {
                    format!(
                        "wiki:ordered-{game}:url:{:x}",
                        Sha256::digest(origin.locator.as_bytes())
                    )
                };
                version.revision = StoreRevision::LocalMonotonic;
                change_document(
                    &mut version,
                    "source_kind",
                    json!(if game { "game_file" } else { "wiki" }),
                );
                let source = version.record.source_id.clone();
                let id = version.record.logical_id.clone();
                change_document(&mut version, "source_id", json!(source));
                change_document(&mut version, "document_id", json!(id));
                origin.identity.source_id = version.record.source_id.clone();
                origin.identity.logical_id = version.record.logical_id.clone();
                origin.bind_record(&mut version.record).unwrap();
                version
            };
            let first = make(originals[0], "Erster Inhalt");
            let second = make(originals[1], "Zweiter Inhalt");
            let result = store
                .import_source_versions(&[first.clone(), second.clone()])
                .await
                .unwrap();
            assert!(result.committed);
            assert_eq!(
                result
                    .versions
                    .iter()
                    .map(|proof| proof.original_revision.as_str())
                    .collect::<Vec<_>>(),
                originals
            );
            assert_eq!(
                result
                    .versions
                    .iter()
                    .map(|proof| proof.store_revision)
                    .collect::<Vec<_>>(),
                [1, 2]
            );
            let head: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
                .bind(&second.record.source_id).bind(&second.record.logical_id).fetch_one(&pool).await.unwrap();
            assert_eq!(head["content"], "Zweiter Inhalt");
            for version in [second, first] {
                let result = store.import_source_versions(&[version]).await.unwrap();
                assert!(result.committed);
                assert_eq!(result.unchanged, 1);
            }
        }
        pool.close().await;
    }

    #[test]
    fn observation_and_local_revision_do_not_change_semantics() {
        let first = record("2026-10-03T12:00:00Z", 12, "seconds");
        let mut repeated = record("2026-10-04T12:00:00Z", 12, "seconds");
        repeated.revision = 999;
        assert_eq!(
            semantic_record(&first).unwrap(),
            semantic_record(&repeated).unwrap()
        );
    }

    #[test]
    fn changed_fact_or_unit_is_a_semantic_conflict() {
        let first = record("2026-10-03T12:00:00Z", 12, "seconds");
        for changed in [
            record("2026-10-03T12:00:00Z", 13, "seconds"),
            record("2026-10-03T12:00:00Z", 12, "milliseconds"),
        ] {
            assert_ne!(
                semantic_record(&first).unwrap(),
                semantic_record(&changed).unwrap()
            );
        }
    }

    #[test]
    fn changed_content_and_rights_remain_conflicts() {
        let first = record("2026-10-03T12:00:00Z", 12, "seconds");
        let mut changed = first.clone();
        changed.content = "other".into();
        assert_ne!(
            semantic_record(&first).unwrap(),
            semantic_record(&changed).unwrap()
        );
        let mut changed = first.clone();
        changed.visibility = SourceVisibility::Public;
        assert_ne!(
            semantic_record(&first).unwrap(),
            semantic_record(&changed).unwrap()
        );
    }
}
