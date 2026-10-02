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

impl PgStore {
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
        let value = sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(release_id)
        .fetch_optional(&mut *tx)
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
                    .bind(source).bind(logical).bind(*revision as i64).fetch_optional(&mut *tx).await.map_err(database_error)?.ok_or_else(|| invalid("incomplete release or current ACL"))?;
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
        tx.commit().await.map_err(database_error)?;
        Ok(CorpusSnapshot {
            release,
            revisions,
            heads,
        })
    }
}
