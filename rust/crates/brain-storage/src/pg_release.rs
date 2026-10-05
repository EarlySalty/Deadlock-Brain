use crate::{
    memory_repository::validate_release,
    pg_jobs::{commit_batch_tx, database_error, lock_source},
    PgStore,
};
use brain_contracts::{
    maintenance::{MaintenanceLease, MaintenancePublicationProof},
    BatchReceipt, CorpusRelease, CorpusSnapshot, Lease, PortError, SourceBatch, SourceRecordV2,
};
use sqlx::{Postgres, Row, Transaction};
use std::collections::{BTreeMap, BTreeSet};
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

async fn publish_release_tx(
    tx: &mut Transaction<'_, Postgres>,
    release: &CorpusRelease,
) -> Result<(), PortError> {
    validate_release(release)?;
    let key = format!("core-release:{}", release.release_id);
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
        .bind(key)
        .execute(&mut **tx)
        .await
        .map_err(database_error)?;
    let value = serde_json::to_value(release).map_err(|_| invalid("invalid release JSON"))?;
    if let Some(old) = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
    )
    .bind(&release.release_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(database_error)?
    {
        return if value == old {
            Ok(())
        } else {
            Err(invalid("immutable release conflict"))
        };
    }
    for (source, documents) in &release.source_revisions {
        for (logical, revision) in documents {
            let found = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3)")
                .bind(source).bind(logical).bind(*revision as i64).fetch_one(&mut **tx).await.map_err(database_error)?;
            if !found {
                return Err(invalid("release references missing revision"));
            }
        }
    }
    sqlx::query("INSERT INTO brain.corpus_releases_v1(release_id,knowledge_version,patch,release_json) VALUES($1,$2,$3,$4)")
        .bind(&release.release_id).bind(&release.knowledge_version).bind(&release.patch).bind(value).execute(&mut **tx).await.map_err(database_error)?;
    Ok(())
}

fn reuse_imported_timestamp(
    proposed: &CorpusRelease,
    stored: CorpusRelease,
) -> Result<CorpusRelease, PortError> {
    validate_release(&stored)?;
    if stored.created_at_epoch < 0 {
        return Err(invalid("invalid imported release timestamp"));
    }
    let mut comparable = proposed.clone();
    comparable.created_at_epoch = stored.created_at_epoch;
    if comparable != stored {
        return Err(invalid("immutable imported release conflict"));
    }
    Ok(stored)
}

async fn imported_release_retry_tx(
    tx: &mut Transaction<'_, Postgres>,
    proposed: &CorpusRelease,
) -> Result<CorpusRelease, PortError> {
    let stored = sqlx::query(
        "SELECT release_id,knowledge_version,patch,release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
    )
    .bind(&proposed.release_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(database_error)?;
    match stored {
        Some(row) => {
            let release: CorpusRelease =
                serde_json::from_value(row.try_get("release_json").map_err(database_error)?)
                    .map_err(|_| invalid("invalid imported retry release"))?;
            if row
                .try_get::<String, _>("release_id")
                .map_err(database_error)?
                != release.release_id
                || row
                    .try_get::<String, _>("knowledge_version")
                    .map_err(database_error)?
                    != release.knowledge_version
                || row.try_get::<String, _>("patch").map_err(database_error)? != release.patch
            {
                return Err(invalid("imported retry release columns disagree"));
            }
            reuse_imported_timestamp(proposed, release)
        }
        None => Ok(proposed.clone()),
    }
}

fn retired_entity_document(record: &SourceRecordV2) -> bool {
    record.tombstone
        && record.source_id == "git-game-facts-derived"
        && record
            .metadata
            .get("brain.entity_projection.contract")
            .map(String::as_str)
            == Some("git-entity-document-v1")
}

fn withdrawn_imported_original(record: &SourceRecordV2) -> bool {
    if !record.tombstone || record.revision <= 1 {
        return false;
    }
    let Some(document) = record
        .metadata
        .get(crate::source_versions::DOCUMENT_METADATA_KEY)
        .and_then(|encoded| serde_json::from_str::<serde_json::Value>(encoded).ok())
    else {
        return false;
    };
    document["contract_version"] == "wiki-spielwissen-v1"
        && matches!(document["source_kind"].as_str(), Some("game_file" | "wiki"))
        && document["source_id"].as_str() == Some(record.source_id.as_str())
        && document["document_id"].as_str() == Some(record.logical_id.as_str())
        && document["content"].as_str() == Some(record.content.as_str())
        && document["content_sha256"].as_str() == Some(record.content_hash.as_str())
        && document["revision"].as_str().is_some_and(|revision| {
            !revision.is_empty()
                && record
                    .metadata
                    .get(crate::source_versions::ORIGINAL_VERSION_KEY)
                    .map(String::as_str)
                    == Some(revision)
        })
}

async fn verify_withdrawn_original_tx(
    tx: &mut Transaction<'_, Postgres>,
    record: &SourceRecordV2,
) -> Result<(), PortError> {
    let previous = sqlx::query("SELECT revision,content_hash,tombstone,record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
        .bind(&record.source_id).bind(&record.logical_id).bind((record.revision - 1) as i64)
        .fetch_optional(&mut **tx).await.map_err(database_error)?
        .ok_or_else(|| invalid("Belegte aktive Originalrevision fehlt"))?;
    let original: SourceRecordV2 =
        serde_json::from_value(previous.try_get("record_json").map_err(database_error)?)
            .map_err(|_| invalid("Gespeicherte Originalrevision ist ungültig"))?;
    original
        .validate()
        .map_err(|_| invalid("Originalrevision verletzt den Vertrag"))?;
    brain_contracts::source::origin_from_record(&original)
        .map_err(|_| invalid("Originalherkunft fehlt"))?;
    let mut expected = original.clone();
    expected.revision = record.revision;
    expected.tombstone = true;
    let published: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM brain.corpus_releases_v1 WHERE release_json->'source_revisions'->($1::text)->>($2::text)=$3)")
        .bind(&original.source_id).bind(&original.logical_id).bind(original.revision.to_string())
        .fetch_one(&mut **tx).await.map_err(database_error)?;
    if original.tombstone
        || expected != *record
        || !published
        || previous
            .try_get::<i64, _>("revision")
            .map_err(database_error)?
            != original.revision as i64
        || previous
            .try_get::<String, _>("content_hash")
            .map_err(database_error)?
            != original.content_hash
        || previous
            .try_get::<bool, _>("tombstone")
            .map_err(database_error)?
            != original.tombstone
    {
        return Err(invalid(
            "Rücknahme widerspricht dem veröffentlichten Original",
        ));
    }
    Ok(())
}

impl PgStore {
    pub async fn verify_withdrawn_imported_source(&self, source_id: &str) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        lock_source(&mut tx, source_id)
            .await
            .map_err(database_error)?;
        let rows = sqlx::query("SELECT logical_id,revision,content_hash,tombstone,record_json FROM brain.source_record_heads WHERE source_id=$1 FOR SHARE")
            .bind(source_id).fetch_all(&mut *tx).await.map_err(database_error)?;
        if rows.is_empty() {
            return Err(invalid("Zurückgezogene Originalheads fehlen"));
        }
        for row in rows {
            let record: SourceRecordV2 =
                serde_json::from_value(row.try_get("record_json").map_err(database_error)?)
                    .map_err(|_| invalid("Originalhead ist ungültig"))?;
            if record.source_id != source_id
                || !withdrawn_imported_original(&record)
                || row
                    .try_get::<String, _>("logical_id")
                    .map_err(database_error)?
                    != record.logical_id
                || row.try_get::<i64, _>("revision").map_err(database_error)?
                    != record.revision as i64
                || row
                    .try_get::<String, _>("content_hash")
                    .map_err(database_error)?
                    != record.content_hash
                || !row
                    .try_get::<bool, _>("tombstone")
                    .map_err(database_error)?
            {
                return Err(invalid("Originalhead ist nicht vollständig zurückgezogen"));
            }
            verify_withdrawn_original_tx(&mut tx, &record).await?;
        }
        tx.commit().await.map_err(database_error)
    }

    pub async fn persist_entity_document(
        &self,
        mut record: SourceRecordV2,
        receipt_json: &str,
    ) -> Result<SourceRecordV2, PortError> {
        use sha2::{Digest, Sha256};
        record
            .validate()
            .map_err(|_| invalid("Steckbriefdatensatz ist ungültig"))?;
        brain_contracts::source::origin_from_record(&record)
            .map_err(|_| invalid("Steckbriefherkunft fehlt"))?;
        let receipt: serde_json::Value = serde_json::from_str(receipt_json)
            .map_err(|_| invalid("Steckbriefquittung ist ungültig"))?;
        let document: serde_json::Value = serde_json::from_str(&record.content)
            .map_err(|_| invalid("Kompakter Steckbrief ist ungültig"))?;
        let receipt_hash = format!("{:x}", Sha256::digest(receipt_json.as_bytes()));
        if record.tombstone
            || record
                .metadata
                .get("brain.entity_projection.contract")
                .map(String::as_str)
                != Some("git-entity-document-v1")
            || record
                .metadata
                .get("brain.entity_projection.receipt_sha256")
                != Some(&receipt_hash)
            || record.content_hash != format!("{:x}", Sha256::digest(record.content.as_bytes()))
            || receipt["entity_key"].as_str() != Some(record.logical_id.as_str())
            || receipt["document_sha256"].as_str() != Some(record.content_hash.as_str())
            || document["entity"]["entity_key"].as_str() != Some(record.logical_id.as_str())
            || document["contract_version"].as_str()
                != Some(brain_contracts::entity_profile::ENTITY_PROFILE_VERSION)
        {
            return Err(invalid(
                "Steckbrief und private Quittung widersprechen sich",
            ));
        }
        let stable_receipt = |mut value: serde_json::Value| {
            if let Some(fields) = value.as_object_mut() {
                fields.remove("original_release_id");
            }
            value
        };
        let stable_record = |mut record: SourceRecordV2| {
            record.revision = 1;
            record
                .metadata
                .remove("brain.entity_projection.receipt_sha256");
            record
        };
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        lock_source(&mut tx, &record.source_id)
            .await
            .map_err(database_error)?;
        let current: Option<serde_json::Value> = sqlx::query_scalar(
            "SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2 FOR UPDATE",
        )
        .bind(&record.source_id).bind(&record.logical_id)
        .fetch_optional(&mut *tx).await.map_err(database_error)?;
        if let Some(current) = current.filter(|current| {
            !(current["tombstone"] == true
                && current["source_id"] == "git-game-facts-derived"
                && current["metadata"]["brain.entity_projection.contract"]
                    == "git-entity-document-v1")
        }) {
            let current: SourceRecordV2 = serde_json::from_value(current)
                .map_err(|_| invalid("Gespeicherter Steckbrief ist ungültig"))?;
            let stored: Option<String> = sqlx::query_scalar(
                "SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3",
            )
            .bind(&current.source_id).bind(&current.logical_id).bind(current.revision as i64)
            .fetch_optional(&mut *tx).await.map_err(database_error)?;
            let stored = stored.ok_or_else(|| invalid("Private Steckbriefquittung fehlt"))?;
            if current
                .metadata
                .get("brain.entity_projection.receipt_sha256")
                != Some(&format!("{:x}", Sha256::digest(stored.as_bytes())))
            {
                return Err(invalid("Gespeicherte Quittung widerspricht dem Steckbrief"));
            }
            let previous: serde_json::Value = serde_json::from_str(&stored)
                .map_err(|_| invalid("Gespeicherte Steckbriefquittung ist ungültig"))?;
            if stable_record(current.clone()) == stable_record(record.clone())
                && stable_receipt(previous) == stable_receipt(receipt)
            {
                tx.commit().await.map_err(database_error)?;
                return Ok(current);
            }
        }
        let maximum: Option<i64> = sqlx::query_scalar(
            "SELECT max(revision) FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2",
        )
        .bind(&record.source_id).bind(&record.logical_id)
        .fetch_one(&mut *tx).await.map_err(database_error)?;
        record.revision = maximum
            .unwrap_or(0)
            .checked_add(1)
            .filter(|revision| *revision > 0)
            .ok_or_else(|| invalid("Steckbriefrevision ist ausgeschöpft"))?
            as u64;
        Self::apply_connection(&mut tx, &record)
            .await
            .map_err(|_| invalid("Steckbriefspeicherung fehlgeschlagen"))?;
        sqlx::query("INSERT INTO brain.entity_derived_receipts_v1(derived_source_id,derived_logical_id,derived_revision,receipt_json) VALUES($1,$2,$3,$4)")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
            .bind(receipt_json).execute(&mut *tx).await.map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        Ok(record)
    }

    pub fn prepare_imported_release(
        base: &CorpusSnapshot,
        sources: &[String],
        heads: Vec<SourceRecordV2>,
        release_id: &str,
        knowledge_version: &str,
        created_at_epoch: i64,
    ) -> Result<(CorpusRelease, Vec<SourceRecordV2>), PortError> {
        let selected: BTreeSet<_> = sources.iter().cloned().collect();
        if selected.is_empty() || selected.len() != sources.len() || created_at_epoch < 0 {
            return Err(invalid("Eindeutige ausgewählte Importquellen fehlen"));
        }
        let mut expected: BTreeMap<_, _> = base
            .heads
            .iter()
            .filter(|record| !selected.contains(&record.source_id))
            .map(|record| {
                (
                    (record.source_id.clone(), record.logical_id.clone()),
                    record.clone(),
                )
            })
            .collect();
        let mut release = CorpusRelease {
            release_id: release_id.into(),
            knowledge_version: knowledge_version.into(),
            patch: base.release.patch.clone(),
            created_at_epoch,
            source_revisions: base.release.source_revisions.clone(),
        };
        for source in sources {
            release.source_revisions.remove(source);
        }
        let mut seen = BTreeSet::new();
        for record in heads {
            if (record.tombstone
                && !retired_entity_document(&record)
                && !withdrawn_imported_original(&record))
                || !selected.contains(&record.source_id)
            {
                return Err(invalid(
                    "Importquelle ist zurückgezogen oder nicht ausgewählt",
                ));
            }
            record
                .validate()
                .map_err(|_| invalid("Importkopf verletzt den Vertrag"))?;
            brain_contracts::source::origin_from_record(&record)
                .map_err(|_| invalid("Importkopf hat keinen gültigen Herkunftsnachweis"))?;
            seen.insert(record.source_id.clone());
            let pins = release
                .source_revisions
                .entry(record.source_id.clone())
                .or_default();
            if !record.tombstone {
                pins.insert(record.logical_id.clone(), record.revision);
            }
            if expected
                .insert(
                    (record.source_id.clone(), record.logical_id.clone()),
                    record,
                )
                .is_some()
            {
                return Err(invalid("Importquelle enthält einen doppelten Kopf"));
            }
        }
        if seen.len() != selected.len() {
            return Err(invalid(
                "Ausgewählte Importquelle hat keine gespeicherten Dokumente",
            ));
        }
        validate_release(&release)?;
        Ok((release, expected.into_values().collect()))
    }

    pub async fn imported_release_for_retry(
        &self,
        proposed: &CorpusRelease,
    ) -> Result<CorpusRelease, PortError> {
        validate_release(proposed)?;
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        sqlx::query("SET TRANSACTION READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(database_error)?;
        let release = imported_release_retry_tx(&mut tx, proposed).await?;
        tx.commit().await.map_err(database_error)?;
        Ok(release)
    }

    pub async fn publish_imported_heads_checked(
        &self,
        base_release_id: &str,
        selected_sources: &[String],
        release: &CorpusRelease,
        expected_heads: &[SourceRecordV2],
    ) -> Result<usize, PortError> {
        brain_contracts::maintenance::bounded(base_release_id, 512)?;
        validate_release(release)?;
        if base_release_id == release.release_id {
            return Err(invalid("imported release requires a new identity"));
        }
        let sources: BTreeSet<_> = selected_sources.iter().cloned().collect();
        if sources.is_empty() || sources.len() != selected_sources.len() {
            return Err(invalid("explicit unique imported sources required"));
        }
        for source in &sources {
            brain_contracts::maintenance::bounded(source, 512)?;
        }
        let mut expected = BTreeMap::new();
        let mut selected_pins: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
        for record in expected_heads {
            record
                .validate()
                .map_err(|_| invalid("invalid expected imported head"))?;
            if sources.contains(&record.source_id) {
                if record.tombstone
                    && !retired_entity_document(record)
                    && !withdrawn_imported_original(record)
                {
                    return Err(invalid("imported source contains a tombstone"));
                }
                brain_contracts::source::origin_from_record(record)
                    .map_err(|_| invalid("invalid imported source provenance"))?;
                let pins = selected_pins.entry(record.source_id.clone()).or_default();
                if !record.tombstone {
                    pins.insert(record.logical_id.clone(), record.revision);
                }
            }
            if expected
                .insert(
                    (record.source_id.clone(), record.logical_id.clone()),
                    record.clone(),
                )
                .is_some()
            {
                return Err(invalid("duplicate expected imported head"));
            }
        }
        if selected_pins.keys().cloned().collect::<BTreeSet<_>>() != sources {
            return Err(invalid("selected imported source has no heads"));
        }
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let base_value: serde_json::Value = sqlx::query_scalar(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(base_release_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(database_error)?
        .ok_or_else(|| invalid("imported base release missing"))?;
        let base: CorpusRelease = serde_json::from_value(base_value)
            .map_err(|_| invalid("invalid imported base release"))?;
        validate_release(&base)?;
        if base.release_id != base_release_id || release.patch != base.patch {
            return Err(invalid("imported release changed base identity or patch"));
        }
        let mut pins = base.source_revisions.clone();
        for source in &sources {
            pins.insert(source.clone(), selected_pins[source].clone());
        }
        if pins != release.source_revisions {
            return Err(invalid(
                "imported release changed preserved pins or selected head set",
            ));
        }
        let pin_keys: BTreeSet<_> = pins
            .iter()
            .flat_map(|(source, documents)| {
                documents
                    .keys()
                    .map(move |logical| (source.clone(), logical.clone()))
            })
            .collect();
        if expected
            .iter()
            .filter(|(_, record)| !record.tombstone || !sources.contains(&record.source_id))
            .map(|(key, _)| key.clone())
            .collect::<BTreeSet<_>>()
            != pin_keys
        {
            return Err(invalid(
                "expected imported heads do not cover exact release",
            ));
        }
        let locked_sources: BTreeSet<_> = base
            .source_revisions
            .keys()
            .cloned()
            .chain(sources.iter().cloned())
            .collect();
        for source in &locked_sources {
            lock_source(&mut tx, source).await.map_err(database_error)?;
        }
        let source_ids: Vec<_> = sources.iter().cloned().collect();
        let rows = sqlx::query("SELECT source_id,logical_id,revision,content_hash,tombstone,record_json FROM brain.source_record_heads WHERE source_id=ANY($1) FOR SHARE")
            .bind(&source_ids).fetch_all(&mut *tx).await.map_err(database_error)?;
        let mut actual = BTreeMap::new();
        for row in rows {
            let source: String = row.try_get("source_id").map_err(database_error)?;
            let logical: String = row.try_get("logical_id").map_err(database_error)?;
            let revision: i64 = row.try_get("revision").map_err(database_error)?;
            let record: SourceRecordV2 =
                serde_json::from_value(row.try_get("record_json").map_err(database_error)?)
                    .map_err(|_| invalid("invalid stored imported head"))?;
            record
                .validate()
                .map_err(|_| invalid("stored imported head violates contract"))?;
            brain_contracts::source::origin_from_record(&record)
                .map_err(|_| invalid("invalid stored imported provenance"))?;
            if revision <= 0
                || record.source_id != source
                || record.logical_id != logical
                || record.revision != revision as u64
                || record.content_hash
                    != row
                        .try_get::<String, _>("content_hash")
                        .map_err(database_error)?
                || record.tombstone
                    != row
                        .try_get::<bool, _>("tombstone")
                        .map_err(database_error)?
                || (record.tombstone
                    && !retired_entity_document(&record)
                    && !withdrawn_imported_original(&record))
                || actual.insert((source, logical), record).is_some()
            {
                return Err(invalid("stored imported head columns disagree"));
            }
        }
        let selected_expected: BTreeMap<_, _> = expected
            .iter()
            .filter(|((source, _), _)| sources.contains(source))
            .map(|(key, record)| (key.clone(), record.clone()))
            .collect();
        if actual != selected_expected {
            return Err(invalid("imported source heads changed before release"));
        }
        for record in actual
            .values()
            .filter(|record| withdrawn_imported_original(record))
        {
            verify_withdrawn_original_tx(&mut tx, record).await?;
        }
        let base_readback = snapshot_tx(&mut tx, base_release_id).await?;
        if base_readback.release != base {
            return Err(invalid("imported base release readback mismatch"));
        }
        for head in &base_readback.heads {
            if !sources.contains(&head.source_id)
                && expected.get(&(head.source_id.clone(), head.logical_id.clone())) != Some(head)
            {
                return Err(invalid(
                    "preserved current head or ACL changed before release",
                ));
            }
        }
        let pin_json = serde_json::to_value(&pins).map_err(|_| invalid("invalid imported pins"))?;
        let inconsistent: bool = sqlx::query_scalar("SELECT EXISTS (
            SELECT 1 FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d
            LEFT JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint
            LEFT JOIN brain.source_record_heads h ON h.source_id=s.key AND h.logical_id=d.key
            LEFT JOIN brain.source_record_revisions c ON c.source_id=h.source_id AND c.logical_id=h.logical_id AND c.revision=h.revision
            WHERE r.source_id IS NULL OR h.source_id IS NULL OR c.source_id IS NULL
                OR h.revision<r.revision OR h.record_json IS DISTINCT FROM c.record_json
                OR h.content_hash IS DISTINCT FROM c.content_hash OR h.tombstone IS DISTINCT FROM c.tombstone
                OR r.record_json->>'source_id' IS DISTINCT FROM r.source_id
                OR r.record_json->>'logical_id' IS DISTINCT FROM r.logical_id
                OR r.record_json->>'revision' IS DISTINCT FROM r.revision::text
                OR r.record_json->>'content_hash' IS DISTINCT FROM r.content_hash
                OR r.record_json->>'tombstone' IS DISTINCT FROM r.tombstone::text
                OR h.record_json->>'source_id' IS DISTINCT FROM h.source_id
                OR h.record_json->>'logical_id' IS DISTINCT FROM h.logical_id
                OR h.record_json->>'revision' IS DISTINCT FROM h.revision::text
                OR h.record_json->>'content_hash' IS DISTINCT FROM h.content_hash
                OR h.record_json->>'tombstone' IS DISTINCT FROM h.tombstone::text
        )").bind(&pin_json).fetch_one(&mut *tx).await.map_err(database_error)?;
        if inconsistent {
            return Err(invalid(
                "imported release revision or current ACL inconsistent",
            ));
        }
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(format!("core-release:{}", release.release_id))
            .execute(&mut *tx)
            .await
            .map_err(database_error)?;
        let release = imported_release_retry_tx(&mut tx, release).await?;
        publish_release_tx(&mut tx, &release).await?;
        let readback = snapshot_tx(&mut tx, &release.release_id).await?;
        if readback.release != release
            || readback.heads.iter().any(|head| {
                expected.get(&(head.source_id.clone(), head.logical_id.clone())) != Some(head)
            })
        {
            return Err(invalid("imported release readback mismatch"));
        }
        for record in readback.revisions.iter().chain(&readback.heads) {
            record
                .validate()
                .map_err(|_| invalid("invalid imported snapshot record"))?;
            if record
                .metadata
                .contains_key(brain_contracts::source::ORIGIN_METADATA_KEY)
            {
                brain_contracts::source::origin_from_record(record)
                    .map_err(|_| invalid("invalid imported snapshot provenance"))?;
            }
        }
        let _ = readback.authorized(
            &brain_contracts::Principal {
                actor_id: "release-validation".into(),
                channel: "internal".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            false,
        )?;
        let count = readback.revisions.len();
        tx.commit().await.map_err(database_error)?;
        Ok(count)
    }

    /// Bindet eine bereits gespeicherte und unabhängig geprüfte Revision, ohne sie erneut zu schreiben.
    pub async fn reuse_maintenance_revision(
        &self,
        lease: &MaintenanceLease,
        base_id: &str,
        release: &CorpusRelease,
        revision: u64,
        expected: &SourceRecordV2,
    ) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let (job, registration) =
            crate::pg_maintenance::lock_publication_job(&mut tx, lease).await?;
        lock_source(&mut tx, &registration.source_id)
            .await
            .map_err(database_error)?;
        let value: serde_json::Value = sqlx::query_scalar(
            "SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2 FOR UPDATE",
        ).bind(&registration.source_id).bind(&job.spec.target_path)
            .fetch_optional(&mut *tx).await.map_err(database_error)?
            .ok_or_else(|| invalid("maintenance existing target missing"))?;
        let record: SourceRecordV2 = serde_json::from_value(value)
            .map_err(|_| invalid("invalid existing maintenance target"))?;
        crate::pg_maintenance::validate_publication_record(&job, &record)?;
        let normalize = |record: &SourceRecordV2| -> Result<SourceRecordV2, PortError> {
            let mut stable = record.clone();
            stable.revision = 1;
            let mut origin = brain_contracts::source::origin_from_record(record)
                .map_err(|_| invalid("invalid existing maintenance provenance"))?;
            origin.retrieved_at = brain_contracts::value::Observed::unknown(
                brain_contracts::value::UnknownReason::NotPresent,
            );
            origin
                .bind_record(&mut stable)
                .map_err(|_| invalid("invalid normalized maintenance provenance"))?;
            Ok(stable)
        };
        if normalize(&record)? != normalize(expected)? {
            return Err(invalid("maintenance existing document semantics changed"));
        }
        if record.tombstone
            || record.revision != revision
            || job.checkpoint.review.as_ref().is_none_or(|review| {
                !review.accepted || review.document_sha256 != record.content_hash
            })
        {
            return Err(invalid("maintenance existing target superseded"));
        }
        let base_value: serde_json::Value = sqlx::query_scalar(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(base_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(database_error)?;
        let base: CorpusRelease = serde_json::from_value(base_value)
            .map_err(|_| invalid("invalid existing maintenance base"))?;
        let mut pins = base.source_revisions.clone();
        pins.entry(record.source_id.clone())
            .or_default()
            .insert(record.logical_id.clone(), revision);
        if release.patch != base.patch || release.source_revisions != pins {
            return Err(invalid(
                "maintenance existing revision changed foreign pins",
            ));
        }
        publish_release_tx(&mut tx, release).await?;
        crate::pg_maintenance::record_publication_checkpoint(
            &mut tx,
            lease,
            &job,
            MaintenancePublicationProof {
                release_id: release.release_id.clone(),
                source_id: record.source_id,
                logical_id: record.logical_id,
                document_revision: revision,
                document_sha256: record.content_hash,
            },
        )
        .await?;
        tx.commit().await.map_err(database_error)?;
        Ok(())
    }
    /// Verbindet eine unveränderte publizierte Dokumentrevision mit dem jetzt aktiven Basisrelease.
    pub async fn rebase_maintenance_publication(
        &self,
        lease: &MaintenanceLease,
        base_id: &str,
        release: &CorpusRelease,
        activation_ref: &str,
    ) -> Result<(), PortError> {
        brain_contracts::maintenance::bounded(activation_ref, 2048)?;
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let (job, registration) =
            crate::pg_maintenance::lock_publication_job(&mut tx, lease).await?;
        let previous = job
            .checkpoint
            .publication
            .as_ref()
            .ok_or_else(|| invalid("rebase requires existing publication"))?;
        if previous.source_id != registration.source_id
            || previous.logical_id != job.spec.target_path
        {
            return Err(invalid("rebase target identity changed"));
        }
        lock_source(&mut tx, &registration.source_id)
            .await
            .map_err(database_error)?;
        let base_value: serde_json::Value = sqlx::query_scalar(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(base_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(database_error)?
        .ok_or_else(|| invalid("rebase base missing"))?;
        let base: CorpusRelease =
            serde_json::from_value(base_value).map_err(|_| invalid("rebase base JSON"))?;
        let mut expected = base.source_revisions.clone();
        expected
            .entry(previous.source_id.clone())
            .or_default()
            .insert(previous.logical_id.clone(), previous.document_revision);
        if release.patch != base.patch || release.source_revisions != expected {
            return Err(invalid("rebase changed unrelated pins"));
        }
        let value: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
            .bind(&previous.source_id).bind(&previous.logical_id).fetch_optional(&mut *tx).await.map_err(database_error)?.ok_or_else(|| invalid("rebase target missing"))?;
        let record: SourceRecordV2 =
            serde_json::from_value(value).map_err(|_| invalid("rebase target JSON"))?;
        crate::pg_maintenance::validate_publication_record(&job, &record)?;
        if record.revision != previous.document_revision
            || record.content_hash != previous.document_sha256
        {
            return Err(invalid("rebase would restore superseded target"));
        }
        publish_release_tx(&mut tx, release).await?;
        let mut checkpoint = job.checkpoint.clone();
        checkpoint
            .publication
            .as_mut()
            .expect("Publikationsbeleg")
            .release_id = release.release_id.clone();
        checkpoint
            .artifact_refs
            .insert("activation".into(), activation_ref.into());
        checkpoint
            .artifact_refs
            .insert("previous_release".into(), previous.release_id.clone());
        checkpoint.validate(
            &job.spec,
            brain_contracts::maintenance::MaintenanceStatus::Publish,
        )?;
        let changed = sqlx::query("UPDATE brain.maintenance_jobs_v1 SET checkpoint_json=$4,updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp() AND status='publish'")
            .bind(&lease.job_id).bind(&lease.owner).bind(lease.fence as i64).bind(serde_json::to_value(&checkpoint).map_err(|_| invalid("rebase checkpoint JSON"))?)
            .execute(&mut *tx).await.map_err(database_error)?;
        if changed.rows_affected() != 1 {
            return Err(invalid("stale rebase fence"));
        }
        tx.commit().await.map_err(database_error)?;
        Ok(())
    }

    pub async fn publish_release(&self, release: &CorpusRelease) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        publish_release_tx(&mut tx, release).await?;
        tx.commit().await.map_err(database_error)
    }

    pub async fn commit_batches_and_publish(
        &self,
        batches: &[(&SourceBatch, &Lease)],
        release: &CorpusRelease,
    ) -> Result<(Vec<BatchReceipt>, usize), PortError> {
        self.commit_batches_and_publish_inner(batches, release, None, None)
            .await
    }

    pub async fn commit_batches_and_publish_checked(
        &self,
        batches: &[(&SourceBatch, &Lease)],
        release: &CorpusRelease,
        expected_heads: &[SourceRecordV2],
    ) -> Result<(Vec<BatchReceipt>, usize), PortError> {
        self.commit_batches_and_publish_inner(batches, release, Some(expected_heads), None)
            .await
    }

    /// Übernimmt geprüfte Dokumente mit Job-Fence in derselben bestehenden Corpus-Transaktion.
    /// Unberührte Quellen behalten genau die Pins des ausdrücklich genannten Basis-Releases.
    pub async fn commit_maintenance_batches_and_publish_checked(
        &self,
        maintenance_lease: &MaintenanceLease,
        base_release_id: &str,
        batches: &[(&SourceBatch, &Lease)],
        release: &CorpusRelease,
        expected_heads: &[SourceRecordV2],
    ) -> Result<(Vec<BatchReceipt>, usize), PortError> {
        brain_contracts::maintenance::bounded(base_release_id, 512)?;
        self.commit_batches_and_publish_inner(
            batches,
            release,
            Some(expected_heads),
            Some((maintenance_lease, base_release_id)),
        )
        .await
    }

    async fn commit_batches_and_publish_inner(
        &self,
        batches: &[(&SourceBatch, &Lease)],
        release: &CorpusRelease,
        expected_heads: Option<&[SourceRecordV2]>,
        maintenance: Option<(&MaintenanceLease, &str)>,
    ) -> Result<(Vec<BatchReceipt>, usize), PortError> {
        if batches.is_empty() || batches.len() > 2 {
            return Err(invalid("atomic release requires one or two source batches"));
        }
        let mut sources = BTreeSet::new();
        for (batch, lease) in batches {
            if !sources.insert(batch.checkpoint.source_id.clone())
                || lease.source_id != batch.checkpoint.source_id
            {
                return Err(invalid("duplicate or mismatched atomic source"));
            }
            batch.validate()?;
        }
        validate_release(release)?;
        let expected = if let Some(heads) = expected_heads {
            let mut records = BTreeMap::new();
            let mut pins: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
            for record in heads {
                record
                    .validate()
                    .map_err(|_| invalid("invalid expected source head"))?;
                if !sources.contains(&record.source_id)
                    || records
                        .insert(
                            (record.source_id.clone(), record.logical_id.clone()),
                            record.clone(),
                        )
                        .is_some()
                {
                    return Err(invalid("duplicate or unrelated expected source head"));
                }
                if !record.tombstone {
                    pins.entry(record.source_id.clone())
                        .or_default()
                        .insert(record.logical_id.clone(), record.revision);
                }
            }
            let touched_release: BTreeMap<_, _> = release
                .source_revisions
                .iter()
                .filter(|(source, _)| sources.contains(*source))
                .map(|(source, pins)| (source.clone(), pins.clone()))
                .collect();
            if pins
                != if maintenance.is_some() {
                    touched_release
                } else {
                    release.source_revisions.clone()
                }
            {
                return Err(invalid("expected source heads differ from release pins"));
            }
            Some(records)
        } else {
            None
        };
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let maintenance_job = if let Some((lease, base_id)) = maintenance {
            let (job, registration) =
                crate::pg_maintenance::lock_publication_job(&mut tx, lease).await?;
            if sources.len() != 1 || !sources.contains(&registration.source_id) {
                return Err(invalid(
                    "maintenance batch must target only its registered source",
                ));
            }
            lock_source(&mut tx, &registration.source_id)
                .await
                .map_err(database_error)?;
            let base: serde_json::Value = sqlx::query_scalar(
                "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
            )
            .bind(base_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(database_error)?
            .ok_or_else(|| invalid("maintenance base release missing"))?;
            let base: CorpusRelease = serde_json::from_value(base)
                .map_err(|_| invalid("invalid maintenance base release"))?;
            if release.patch != base.patch {
                return Err(invalid("documentation update cannot change the game patch"));
            }
            let preserved: BTreeMap<_, _> = base
                .source_revisions
                .iter()
                .filter(|(source, _)| !sources.contains(*source))
                .map(|(source, pins)| (source.clone(), pins.clone()))
                .collect();
            let proposed: BTreeMap<_, _> = release
                .source_revisions
                .iter()
                .filter(|(source, _)| !sources.contains(*source))
                .map(|(source, pins)| (source.clone(), pins.clone()))
                .collect();
            if preserved != proposed {
                return Err(invalid("maintenance release changed unrelated source pins"));
            }
            let heads = expected_heads
                .ok_or_else(|| invalid("maintenance publication requires exact expected heads"))?;
            let old_rows: Vec<serde_json::Value> = sqlx::query_scalar(
                "SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id<>$2",
            )
            .bind(&registration.source_id)
            .bind(&job.spec.target_path)
            .fetch_all(&mut *tx)
            .await
            .map_err(database_error)?;
            let old_heads = old_rows
                .into_iter()
                .map(|value| {
                    let record: SourceRecordV2 = serde_json::from_value(value)
                        .map_err(|_| invalid("invalid preserved maintenance source head"))?;
                    Ok((record.logical_id.clone(), record))
                })
                .collect::<Result<BTreeMap<_, _>, PortError>>()?;
            let proposed_heads: BTreeMap<_, _> = heads
                .iter()
                .filter(|record| record.logical_id != job.spec.target_path)
                .map(|record| (record.logical_id.clone(), record.clone()))
                .collect();
            let without_target = |pins: Option<&BTreeMap<String, u64>>| -> BTreeMap<String, u64> {
                pins.into_iter()
                    .flat_map(|pins| pins.iter())
                    .filter(|(logical, _)| **logical != job.spec.target_path)
                    .map(|(logical, revision)| (logical.clone(), *revision))
                    .collect()
            };
            if old_heads != proposed_heads
                || without_target(base.source_revisions.get(&registration.source_id))
                    != without_target(release.source_revisions.get(&registration.source_id))
            {
                return Err(invalid(
                    "maintenance changed another document in its source",
                ));
            }
            let document = heads
                .iter()
                .find(|record| {
                    record.source_id == registration.source_id
                        && record.logical_id == job.spec.target_path
                        && !record.tombstone
                })
                .ok_or_else(|| invalid("maintenance target missing from expected heads"))?;
            crate::pg_maintenance::validate_publication_record(&job, document)?;
            if job.checkpoint.review.as_ref().is_none_or(|review| {
                !review.accepted || review.document_sha256 != document.content_hash
            }) {
                return Err(invalid(
                    "maintenance publication differs from accepted document",
                ));
            }
            Some((
                job,
                MaintenancePublicationProof {
                    release_id: release.release_id.clone(),
                    source_id: document.source_id.clone(),
                    logical_id: document.logical_id.clone(),
                    document_revision: document.revision,
                    document_sha256: document.content_hash.clone(),
                },
            ))
        } else {
            None
        };
        for source in &sources {
            lock_source(&mut tx, source).await.map_err(database_error)?;
        }
        if let Some((job, proof)) = &maintenance_job {
            let base_id = maintenance.expect("maintenance publication context").1;
            let base_value: serde_json::Value = sqlx::query_scalar(
                "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
            )
            .bind(base_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(database_error)?;
            let base: CorpusRelease = serde_json::from_value(base_value)
                .map_err(|_| invalid("invalid maintenance base release"))?;
            let current: Option<i64> = sqlx::query_scalar(
                "SELECT revision FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2 FOR UPDATE",
            ).bind(&proof.source_id).bind(&proof.logical_id)
                .fetch_optional(&mut *tx).await.map_err(database_error)?;
            let pinned = base
                .source_revisions
                .get(&proof.source_id)
                .and_then(|pins| pins.get(&proof.logical_id))
                .copied();
            if current.map(|revision| revision as u64) != pinned {
                let observed = job
                    .checkpoint
                    .artifact_refs
                    .get("unactivated_basis")
                    .map(|value| serde_json::from_str::<MaintenancePublicationProof>(value))
                    .transpose()
                    .map_err(|_| invalid("invalid observed unactivated basis"))?;
                let authorized =
                    crate::pg_maintenance::unactivated_publication_basis(&mut tx, &job.spec)
                        .await?;
                if observed.is_none()
                    || observed != authorized
                    || observed.as_ref().is_none_or(|basis| {
                        basis.source_id != proof.source_id
                            || basis.logical_id != proof.logical_id
                            || Some(basis.document_revision)
                                != current.map(|revision| revision as u64)
                    })
                {
                    return Err(invalid("maintenance publication basis superseded"));
                }
            }
        }
        let mut receipts = Vec::with_capacity(batches.len());
        for (batch, lease) in batches {
            receipts.push(commit_batch_tx(&mut tx, batch, lease).await?);
        }
        if let Some(expected) = expected {
            for (batch, _) in batches {
                let stored: Option<serde_json::Value> = sqlx::query_scalar(
                    "SELECT checkpoint_json FROM brain.source_checkpoints_v1 WHERE source_id=$1",
                )
                .bind(&batch.checkpoint.source_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(database_error)?;
                let checkpoint = serde_json::to_value(&batch.checkpoint)
                    .map_err(|_| invalid("invalid expected checkpoint JSON"))?;
                if stored != Some(checkpoint) {
                    return Err(invalid("source checkpoint changed before release"));
                }
            }
            let source_ids: Vec<String> = sources.into_iter().collect();
            let rows = sqlx::query("SELECT source_id,logical_id,revision,content_hash,tombstone,record_json FROM brain.source_record_heads WHERE source_id=ANY($1)")
                .bind(source_ids)
                .fetch_all(&mut *tx)
                .await
                .map_err(database_error)?;
            let mut actual = BTreeMap::new();
            for row in rows {
                let source: String = row.try_get("source_id").map_err(database_error)?;
                let logical: String = row.try_get("logical_id").map_err(database_error)?;
                let revision: i64 = row.try_get("revision").map_err(database_error)?;
                let content_hash: String = row.try_get("content_hash").map_err(database_error)?;
                let tombstone: bool = row.try_get("tombstone").map_err(database_error)?;
                let value: serde_json::Value =
                    row.try_get("record_json").map_err(database_error)?;
                let record: SourceRecordV2 = serde_json::from_value(value)
                    .map_err(|_| invalid("invalid stored source head"))?;
                if revision <= 0
                    || record.source_id != source
                    || record.logical_id != logical
                    || record.revision != revision as u64
                    || record.content_hash != content_hash
                    || record.tombstone != tombstone
                    || actual.insert((source, logical), record).is_some()
                {
                    return Err(invalid("stored source head columns disagree"));
                }
            }
            if actual != expected {
                return Err(invalid("source heads changed before release"));
            }
        }
        publish_release_tx(&mut tx, release).await?;
        let readback = sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(&release.release_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(database_error)?;
        if readback != serde_json::to_value(release).map_err(|_| invalid("invalid release JSON"))? {
            return Err(invalid("atomic release readback mismatch"));
        }
        let pins = serde_json::to_value(&release.source_revisions)
            .map_err(|_| invalid("invalid release pins"))?;
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint JOIN brain.source_record_heads h ON h.source_id=r.source_id AND h.logical_id=r.logical_id")
            .bind(&pins).fetch_one(&mut *tx).await.map_err(database_error)?;
        let expected = release
            .source_revisions
            .values()
            .map(std::collections::BTreeMap::len)
            .sum::<usize>();
        if count < 0 || count as usize != expected {
            return Err(invalid("atomic release readback incomplete"));
        }
        if let Some((job, proof)) = maintenance_job {
            crate::pg_maintenance::record_publication_checkpoint(
                &mut tx,
                maintenance
                    .ok_or_else(|| invalid("maintenance lease missing"))?
                    .0,
                &job,
                proof,
            )
            .await?;
        }
        tx.commit().await.map_err(database_error)?;
        Ok((receipts, expected))
    }

    /// One repeatable-read transaction per call; callers re-open after provider work/cache hits.
    pub async fn snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(database_error)?;
        let snapshot = snapshot_tx(&mut tx, release_id).await?;
        tx.commit().await.map_err(database_error)?;
        Ok(snapshot)
    }
}

async fn snapshot_tx(
    tx: &mut Transaction<'_, Postgres>,
    release_id: &str,
) -> Result<CorpusSnapshot, PortError> {
    let value = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
    )
    .bind(release_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(database_error)?
    .ok_or_else(|| invalid("unknown release"))?;
    let release: CorpusRelease =
        serde_json::from_value(value).map_err(|_| invalid("invalid stored release"))?;
    validate_release(&release)?;
    if release.release_id != release_id {
        return Err(invalid("release identity mismatch"));
    }
    let mut revisions = Vec::new();
    let mut heads = Vec::new();
    for (source, documents) in &release.source_revisions {
        for (logical, revision) in documents {
            let row = sqlx::query("SELECT r.record_json AS historical,h.record_json AS current FROM brain.source_record_revisions r JOIN brain.source_record_heads h USING(source_id,logical_id) WHERE r.source_id=$1 AND r.logical_id=$2 AND r.revision=$3")
                .bind(source).bind(logical).bind(*revision as i64).fetch_optional(&mut **tx).await.map_err(database_error)?.ok_or_else(|| invalid("incomplete release or current ACL"))?;
            let historical: SourceRecordV2 =
                serde_json::from_value(row.try_get("historical").map_err(database_error)?)
                    .map_err(|_| invalid("invalid historical record"))?;
            let current: SourceRecordV2 =
                serde_json::from_value(row.try_get("current").map_err(database_error)?)
                    .map_err(|_| invalid("invalid current record"))?;
            revisions.push(historical);
            heads.push(current);
        }
    }
    Ok(CorpusSnapshot {
        release,
        revisions,
        heads,
    })
}
