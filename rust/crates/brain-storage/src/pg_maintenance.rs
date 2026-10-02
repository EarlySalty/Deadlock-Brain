//! Dauerhafte Pflegejobs im bestehenden PgStore.
use crate::{pg_jobs::database_error, PgStore};
use brain_contracts::{maintenance::*, CorpusRelease, PortError};
use sqlx::{postgres::PgRow, PgConnection, Row};

const MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-02-brain-maintenance-v1.sql");
const ENQUEUE_MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-02-brain-maintenance-enqueue-v2.sql");
const READER_MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-02-brain-maintenance-reader-v3.sql");
const NO_CHANGE_MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-02-brain-maintenance-no-change-v4.sql");
const ROW: &str = "id,spec_json,status,checkpoint_json,attempts,error_code,owner,fence,(extract(epoch FROM lease_until)*1000)::bigint AS expires,superseded_by";
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn json<T: serde::Serialize>(value: &T) -> Result<serde_json::Value, PortError> {
    let value = serde_json::to_value(value).map_err(|_| invalid("invalid maintenance payload"))?;
    if value.to_string().len() > 65536 {
        return Err(invalid("maintenance payload too large"));
    }
    Ok(value)
}
fn decode(row: PgRow) -> Result<MaintenanceJob, PortError> {
    let spec: MaintenanceJobSpec =
        serde_json::from_value(row.try_get("spec_json").map_err(database_error)?)
            .map_err(|_| invalid("invalid stored maintenance specification"))?;
    let status: String = row.try_get("status").map_err(database_error)?;
    let status: MaintenanceStatus = serde_json::from_value(serde_json::Value::String(status))
        .map_err(|_| invalid("invalid stored maintenance status"))?;
    let checkpoint =
        serde_json::from_value(row.try_get("checkpoint_json").map_err(database_error)?)
            .map_err(|_| invalid("invalid stored maintenance checkpoint"))?;
    let owner: Option<String> = row.try_get("owner").map_err(database_error)?;
    let expires: Option<i64> = row.try_get("expires").map_err(database_error)?;
    let fence: i64 = row.try_get("fence").map_err(database_error)?;
    let lease = match (owner, expires) {
        (Some(owner), Some(expires_at_ms)) if fence > 0 => Some(MaintenanceLease {
            job_id: spec.id.clone(),
            owner,
            fence: fence as u64,
            expires_at_ms,
        }),
        (None, None) => None,
        _ => return Err(invalid("invalid stored maintenance lease")),
    };
    Ok(MaintenanceJob {
        spec,
        status,
        checkpoint,
        lease,
        attempts: row.try_get::<i32, _>("attempts").map_err(database_error)? as u32,
        error_code: row.try_get("error_code").map_err(database_error)?,
        superseded_by: row.try_get("superseded_by").map_err(database_error)?,
    })
}
fn lease_inputs(lease: &MaintenanceLease) -> Result<i64, PortError> {
    bounded(&lease.job_id, 512)?;
    bounded(&lease.owner, 512)?;
    if lease.fence == 0 || lease.fence > i64::MAX as u64 {
        return Err(invalid("invalid maintenance fence"));
    }
    Ok(lease.fence as i64)
}
fn ttl(ttl_ms: u64) -> Result<i64, PortError> {
    if !(1..=MAX_MAINTENANCE_LEASE_MS).contains(&ttl_ms) {
        return Err(invalid("invalid maintenance lease duration"));
    }
    Ok(ttl_ms as i64)
}

pub(crate) async fn lock_publication_job(
    connection: &mut PgConnection,
    lease: &MaintenanceLease,
) -> Result<(MaintenanceJob, MaintenanceSourceRegistration), PortError> {
    let fence = lease_inputs(lease)?;
    let query=format!("SELECT {ROW} FROM brain.maintenance_jobs_v1 WHERE id=$1 AND owner=$2 AND fence=$3 AND status='publish' AND lease_until>clock_timestamp() FOR UPDATE");
    let job = decode(
        sqlx::query(&query)
            .bind(&lease.job_id)
            .bind(&lease.owner)
            .bind(fence)
            .fetch_optional(&mut *connection)
            .await
            .map_err(database_error)?
            .ok_or_else(|| invalid("stale maintenance publication lease"))?,
    )?;
    job.checkpoint
        .validate(&job.spec, MaintenanceStatus::Publish)?;
    let registration: serde_json::Value = sqlx::query_scalar(
        "SELECT registration_json FROM brain.maintenance_sources_v1 WHERE repo_id=$1 FOR SHARE",
    )
    .bind(&job.spec.repo_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(database_error)?
    .ok_or_else(|| invalid("maintenance publication source not registered"))?;
    let registration: MaintenanceSourceRegistration = serde_json::from_value(registration)
        .map_err(|_| invalid("invalid maintenance publication registry"))?;
    registration.validate()?;
    if registration.repo_id != job.spec.repo_id
        || registration.policy.as_ref() != Some(&job.spec.policy)
        || registration.discovered_sha != job.spec.source_sha
    {
        return Err(invalid("maintenance publication registry policy changed"));
    }
    Ok((job, registration))
}

pub(crate) fn validate_publication_record(
    job: &MaintenanceJob,
    record: &brain_contracts::SourceRecordV2,
) -> Result<(), PortError> {
    let origin = brain_contracts::source::origin_from_record(record)
        .map_err(|_| invalid("maintenance publication requires versioned source origin"))?;
    if origin.policy != job.spec.policy
        || record.tombstone
        || record.visibility != job.spec.policy.visibility
        || record.allowed_scopes != job.spec.policy.allowed_scopes
    {
        return Err(invalid(
            "maintenance publication changed source access policy",
        ));
    }
    if !matches!(&origin.source_revision,brain_contracts::source::SourceRevision::Git {commit} if commit==&job.spec.source_sha)
    {
        return Err(invalid(
            "maintenance publication changed reviewed source revision",
        ));
    }
    Ok(())
}

pub(crate) async fn record_publication_checkpoint(
    connection: &mut PgConnection,
    lease: &MaintenanceLease,
    job: &MaintenanceJob,
    proof: MaintenancePublicationProof,
) -> Result<(), PortError> {
    let mut checkpoint = job.checkpoint.clone();
    if checkpoint
        .publication
        .as_ref()
        .is_some_and(|old| old != &proof)
    {
        return Err(invalid("maintenance publication replay changed proof"));
    }
    checkpoint.publication = Some(proof);
    checkpoint.validate(&job.spec, MaintenanceStatus::Publish)?;
    let rows=sqlx::query("UPDATE brain.maintenance_jobs_v1 SET checkpoint_json=$4,updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND status='publish' AND lease_until>clock_timestamp()").bind(&lease.job_id).bind(&lease.owner).bind(lease_inputs(lease)?).bind(json(&checkpoint)?).execute(connection).await.map_err(database_error)?;
    if rows.rows_affected() != 1 {
        return Err(invalid("maintenance lease expired during corpus commit"));
    }
    Ok(())
}
impl PgStore {
    pub async fn set_maintenance_artifact(
        &self,
        lease: &MaintenanceLease,
        key: &str,
        reference: &str,
    ) -> Result<(), PortError> {
        bounded(key, 128)?;
        bounded(reference, 2048)?;
        let rows=sqlx::query("UPDATE brain.maintenance_jobs_v1 SET checkpoint_json=jsonb_set(checkpoint_json,ARRAY['artifact_refs',$4],to_jsonb($5::text),true),updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp()")
            .bind(&lease.job_id).bind(&lease.owner).bind(lease_inputs(lease)?).bind(key).bind(reference)
            .execute(&self.pool).await.map_err(database_error)?;
        if rows.rows_affected() != 1 {
            return Err(invalid("stale maintenance artifact lease"));
        }
        Ok(())
    }
    /// Hält die Publikationssperre auch über Pinwechsel und Gesundheitsprüfung.
    /// Bei fehlgeschlagener Aktion oder abschließender Fenceprüfung wird zurückgerollt.
    pub async fn activate_maintenance_checked<F, Fut, R, Rollback>(
        &self,
        lease: &MaintenanceLease,
        timeout_ms: u64,
        activate: F,
        rollback: R,
    ) -> Result<MaintenanceJob, PortError>
    where
        F: FnOnce(MaintenancePublicationProof) -> Fut,
        Fut: std::future::Future<Output = Result<MaintenanceActivationProof, PortError>>,
        R: FnOnce() -> Rollback,
        Rollback: std::future::Future<Output = Result<(), PortError>>,
    {
        if !(1000..=120000).contains(&timeout_ms) {
            return Err(invalid("maintenance activation timeout"));
        }
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let (job, _) = lock_publication_job(&mut tx, lease).await?;
        let publication = job
            .checkpoint
            .publication
            .clone()
            .ok_or_else(|| invalid("maintenance activation requires publication"))?;
        let result = async {
            let proof = tokio::time::timeout(std::time::Duration::from_millis(timeout_ms), activate(publication))
                .await.map_err(|_| invalid("maintenance activation timed out"))??;
            let mut checkpoint = job.checkpoint.clone();
            checkpoint.activation = Some(proof);
            checkpoint.validate(&job.spec, MaintenanceStatus::Activated)?;
            let query = format!("UPDATE brain.maintenance_jobs_v1 SET status='activated',checkpoint_json=$4,owner=NULL,lease_until=NULL,error_code=NULL,updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND status='publish' AND lease_until>clock_timestamp() RETURNING {ROW}");
            let finished = decode(sqlx::query(&query)
                .bind(&lease.job_id).bind(&lease.owner).bind(lease_inputs(lease)?)
                .bind(json(&checkpoint)?).fetch_optional(&mut *tx).await.map_err(database_error)?
                .ok_or_else(|| invalid("maintenance lease expired during activation"))?)?;
            Ok(finished)
        }.await;
        match result {
            Err(error) => {
                rollback()
                    .await
                    .map_err(|_| invalid("maintenance activation rollback failed"))?;
                tx.rollback().await.map_err(database_error)?;
                Err(error)
            }
            Ok(finished) => {
                if let Err(error) = tx.commit().await {
                    if self
                        .maintenance_job(&lease.job_id)
                        .await
                        .ok()
                        .flatten()
                        .as_ref()
                        == Some(&finished)
                    {
                        return Ok(finished);
                    }
                    rollback()
                        .await
                        .map_err(|_| invalid("maintenance activation rollback failed"))?;
                    return Err(database_error(error));
                }
                Ok(finished)
            }
        }
    }

    /// Übernimmt ausschließlich eine bereits autorisierte Registry-Policy für einen wartenden Job.
    pub async fn approve_maintenance_policy(&self, id: &str) -> Result<MaintenanceJob, PortError> {
        bounded(id, 512)?;
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let query=format!("SELECT {ROW} FROM brain.maintenance_jobs_v1 WHERE id=$1 AND status='discovered_policy_pending' AND owner IS NULL AND fence<9223372036854775807 FOR UPDATE");
        let mut job = decode(
            sqlx::query(&query)
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(database_error)?
                .ok_or_else(|| invalid("maintenance job is not awaiting policy"))?,
        )?;
        let registration: serde_json::Value = sqlx::query_scalar(
            "SELECT registration_json FROM brain.maintenance_sources_v1 WHERE repo_id=$1 FOR SHARE",
        )
        .bind(&job.spec.repo_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(database_error)?
        .ok_or_else(|| invalid("maintenance source policy not registered"))?;
        let registration: MaintenanceSourceRegistration = serde_json::from_value(registration)
            .map_err(|_| invalid("invalid registered maintenance policy"))?;
        registration.validate()?;
        job.spec.policy = registration
            .policy
            .ok_or_else(|| invalid("maintenance source policy not authorized"))?;
        let reference = registration
            .authorization_ref
            .ok_or_else(|| invalid("maintenance source policy lacks authorization reference"))?;
        job.spec.validate()?;
        job.checkpoint
            .artifact_refs
            .insert("policy_authorization_ref".into(), reference);
        job.checkpoint
            .validate(&job.spec, MaintenanceStatus::SourceReview)?;
        let query=format!("UPDATE brain.maintenance_jobs_v1 SET spec_json=$2,checkpoint_json=$3,status='source_review',fence=fence+1,error_code=NULL,available_at=clock_timestamp(),updated_at=now() WHERE id=$1 AND status='discovered_policy_pending' AND owner IS NULL RETURNING {ROW}");
        let result = decode(
            sqlx::query(&query)
                .bind(id)
                .bind(json(&job.spec)?)
                .bind(json(&job.checkpoint)?)
                .fetch_one(&mut *tx)
                .await
                .map_err(database_error)?,
        )?;
        tx.commit().await.map_err(database_error)?;
        Ok(result)
    }
    /// Explizite additive Migration durch den Owner bei unveränderter Core-Schemaversion.
    pub async fn migrate_maintenance(&self) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        sqlx::raw_sql("SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms'; SELECT pg_advisory_xact_lock(742110026113::bigint)").execute(&mut *tx).await.map_err(database_error)?;
        for migration in [
            MIGRATION,
            ENQUEUE_MIGRATION,
            READER_MIGRATION,
            NO_CHANGE_MIGRATION,
        ] {
            let body = migration
                .split_once("BEGIN;")
                .and_then(|(_, rest)| rest.trim().strip_suffix("COMMIT;"))
                .ok_or_else(|| invalid("invalid maintenance migration"))?;
            sqlx::raw_sql(body)
                .execute(&mut *tx)
                .await
                .map_err(database_error)?;
        }
        tx.commit().await.map_err(database_error)
    }
    /// Ausschließlich lesende Schemaprüfung für den Pflegeprozess.
    pub async fn check_maintenance_schema(&self) -> Result<(), PortError> {
        sqlx::query("SELECT j.id,j.repo_id,j.idempotency_key,j.spec_json,j.enqueue_spec_json,j.status,j.checkpoint_json,j.attempts,j.error_code,j.owner,j.fence,j.lease_until,j.available_at,j.superseded_by,j.created_at,j.updated_at,s.repo_id,s.registration_json,s.updated_at FROM brain.maintenance_jobs_v1 j,brain.maintenance_sources_v1 s LIMIT 0").fetch_all(&self.pool).await.map_err(database_error)?;
        Ok(())
    }
    pub async fn enqueue_maintenance(
        &self,
        spec: &MaintenanceJobSpec,
        initial: MaintenanceStatus,
    ) -> Result<MaintenanceJob, PortError> {
        spec.validate()?;
        if !matches!(
            initial,
            MaintenanceStatus::Planned | MaintenanceStatus::DiscoveredPolicyPending
        ) {
            return Err(invalid("invalid initial maintenance status"));
        }
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        sqlx::query("INSERT INTO brain.maintenance_jobs_v1(id,repo_id,idempotency_key,spec_json,enqueue_spec_json,status) VALUES($1,$2,$3,$4,$4,$5) ON CONFLICT DO NOTHING").bind(&spec.id).bind(&spec.repo_id).bind(&spec.idempotency_key).bind(json(spec)?).bind(initial.as_str()).execute(&mut *tx).await.map_err(database_error)?;
        let query = format!("SELECT {ROW},enqueue_spec_json FROM brain.maintenance_jobs_v1 WHERE id=$1 OR idempotency_key=$2 FOR UPDATE");
        let mut rows = sqlx::query(&query)
            .bind(&spec.id)
            .bind(&spec.idempotency_key)
            .fetch_all(&mut *tx)
            .await
            .map_err(database_error)?;
        if rows.len() != 1 {
            return Err(invalid("maintenance identity collision"));
        }
        let original: serde_json::Value = rows[0]
            .try_get("enqueue_spec_json")
            .map_err(database_error)?;
        if original != json(spec)? {
            return Err(invalid("maintenance idempotency payload collision"));
        }
        let job = decode(rows.remove(0))?;
        tx.commit().await.map_err(database_error)?;
        Ok(job)
    }
    pub async fn claim_maintenance(
        &self,
        owner: &str,
        ttl_ms: u64,
    ) -> Result<Option<MaintenanceJob>, PortError> {
        bounded(owner, 512)?;
        let ttl = ttl(ttl_ms)?;
        // Erschöpfte, abgelaufene Jobs bleiben sichtbar als Fehler statt still zu verschwinden.
        sqlx::query("UPDATE brain.maintenance_jobs_v1 SET status='failed',error_code='ATTEMPTS_EXHAUSTED',owner=NULL,lease_until=NULL,updated_at=now() WHERE attempts>=100 AND status IN ('planned','source_review','author','reviewer','publish') AND (lease_until IS NULL OR lease_until<=clock_timestamp())").execute(&self.pool).await.map_err(database_error)?;
        let query = "WITH candidate AS (SELECT id FROM brain.maintenance_jobs_v1 WHERE status IN ('planned','source_review','author','reviewer','publish') AND available_at<=clock_timestamp() AND (lease_until IS NULL OR lease_until<=clock_timestamp()) AND attempts<100 AND fence<9223372036854775807 ORDER BY available_at,created_at,id FOR UPDATE SKIP LOCKED LIMIT 1) UPDATE brain.maintenance_jobs_v1 j SET owner=$1,fence=j.fence+1,lease_until=clock_timestamp()+($2::bigint*interval '1 millisecond'),attempts=j.attempts+1,updated_at=now() FROM candidate WHERE j.id=candidate.id RETURNING j.id,j.spec_json,j.status,j.checkpoint_json,j.attempts,j.error_code,j.owner,j.fence,(extract(epoch FROM j.lease_until)*1000)::bigint AS expires,j.superseded_by".to_owned();
        sqlx::query(&query)
            .bind(owner)
            .bind(ttl)
            .fetch_optional(&self.pool)
            .await
            .map_err(database_error)?
            .map(decode)
            .transpose()
    }
    pub async fn renew_maintenance(
        &self,
        lease: &MaintenanceLease,
        ttl_ms: u64,
    ) -> Result<MaintenanceLease, PortError> {
        let fence = lease_inputs(lease)?;
        let ttl = ttl(ttl_ms)?;
        let expires = sqlx::query_scalar::<_,i64>("UPDATE brain.maintenance_jobs_v1 SET lease_until=clock_timestamp()+($4::bigint*interval '1 millisecond'),updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp() RETURNING (extract(epoch FROM lease_until)*1000)::bigint").bind(&lease.job_id).bind(&lease.owner).bind(fence).bind(ttl).fetch_optional(&self.pool).await.map_err(database_error)?.ok_or_else(|| invalid("stale maintenance lease"))?;
        Ok(MaintenanceLease {
            expires_at_ms: expires,
            ..lease.clone()
        })
    }
    /// Stufenabschluss gibt die Lease frei; der nächste Worker braucht eine neue Fence.
    pub async fn transition_maintenance(
        &self,
        lease: &MaintenanceLease,
        next: MaintenanceStatus,
        checkpoint: &MaintenanceCheckpoint,
    ) -> Result<MaintenanceJob, PortError> {
        let fence = lease_inputs(lease)?;
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let query = format!("SELECT {ROW} FROM brain.maintenance_jobs_v1 WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp() FOR UPDATE");
        let job = decode(
            sqlx::query(&query)
                .bind(&lease.job_id)
                .bind(&lease.owner)
                .bind(fence)
                .fetch_optional(&mut *tx)
                .await
                .map_err(database_error)?
                .ok_or_else(|| invalid("stale maintenance lease"))?,
        )?;
        if !job.status.can_transition(next) {
            return Err(invalid("invalid maintenance transition"));
        }
        if job.checkpoint.review.as_ref().is_some_and(|r| r.accepted)
            && next != MaintenanceStatus::Author
            && job.checkpoint.review != checkpoint.review
        {
            return Err(invalid("accepted maintenance review cannot be replaced"));
        }
        checkpoint.validate(&job.spec, next)?;
        if let Some(proof) = &checkpoint.publication {
            let row = sqlx::query("SELECT c.release_json,r.content_hash,r.record_json FROM brain.corpus_releases_v1 c JOIN brain.source_record_revisions r ON r.source_id=$2 AND r.logical_id=$3 AND r.revision=$4 WHERE c.release_id=$1").bind(&proof.release_id).bind(&proof.source_id).bind(&proof.logical_id).bind(proof.document_revision as i64).fetch_optional(&mut *tx).await.map_err(database_error)?.ok_or_else(|| invalid("publication proof absent from corpus"))?;
            let release: CorpusRelease =
                serde_json::from_value(row.try_get("release_json").map_err(database_error)?)
                    .map_err(|_| invalid("invalid stored release"))?;
            let record: brain_contracts::SourceRecordV2 =
                serde_json::from_value(row.try_get("record_json").map_err(database_error)?)
                    .map_err(|_| invalid("invalid published maintenance record"))?;
            validate_publication_record(&job, &record)?;
            if release
                .source_revisions
                .get(&proof.source_id)
                .and_then(|pins| pins.get(&proof.logical_id))
                != Some(&proof.document_revision)
                || row
                    .try_get::<String, _>("content_hash")
                    .map_err(database_error)?
                    != proof.document_sha256
            {
                return Err(invalid("publication proof does not match release"));
            }
        }
        let query = format!("UPDATE brain.maintenance_jobs_v1 SET status=$4,checkpoint_json=$5,owner=NULL,lease_until=NULL,error_code=CASE WHEN $4='failed' THEN 'STAGE_FAILED' ELSE NULL END,available_at=clock_timestamp(),updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp() RETURNING {ROW}");
        let job = decode(
            sqlx::query(&query)
                .bind(&lease.job_id)
                .bind(&lease.owner)
                .bind(fence)
                .bind(next.as_str())
                .bind(json(checkpoint)?)
                .fetch_optional(&mut *tx)
                .await
                .map_err(database_error)?
                .ok_or_else(|| invalid("maintenance lease expired during transition"))?,
        )?;
        tx.commit().await.map_err(database_error)?;
        Ok(job)
    }
    /// Behält Stufe und Checkpoint bei und verzögert den nächsten Versuch nach einem Fehler.
    pub async fn retry_maintenance(
        &self,
        lease: &MaintenanceLease,
        error_code: &str,
        delay_ms: u64,
    ) -> Result<MaintenanceJob, PortError> {
        let fence = lease_inputs(lease)?;
        bounded(error_code, 128)?;
        if !error_code
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
            || delay_ms > 86_400_000
        {
            return Err(invalid("invalid maintenance retry"));
        }
        let query = format!("UPDATE brain.maintenance_jobs_v1 SET owner=NULL,lease_until=NULL,error_code=$4,available_at=clock_timestamp()+($5::bigint*interval '1 millisecond'),status=CASE WHEN attempts>=100 THEN 'failed' ELSE status END,updated_at=now() WHERE id=$1 AND owner=$2 AND fence=$3 AND lease_until>clock_timestamp() RETURNING {ROW}");
        decode(
            sqlx::query(&query)
                .bind(&lease.job_id)
                .bind(&lease.owner)
                .bind(fence)
                .bind(error_code)
                .bind(delay_ms as i64)
                .fetch_optional(&self.pool)
                .await
                .map_err(database_error)?
                .ok_or_else(|| invalid("stale maintenance lease"))?,
        )
    }
    pub async fn supersede_maintenance(
        &self,
        lease: &MaintenanceLease,
        replacement_id: &str,
    ) -> Result<MaintenanceJob, PortError> {
        let fence = lease_inputs(lease)?;
        bounded(replacement_id, 512)?;
        if replacement_id == lease.job_id {
            return Err(invalid("job cannot supersede itself"));
        }
        let query=format!("UPDATE brain.maintenance_jobs_v1 j SET status='superseded',superseded_by=$4,owner=NULL,lease_until=NULL,updated_at=now() WHERE j.id=$1 AND j.owner=$2 AND j.fence=$3 AND j.lease_until>clock_timestamp() AND EXISTS(SELECT 1 FROM brain.maintenance_jobs_v1 n WHERE n.id=$4 AND n.repo_id=j.repo_id AND n.spec_json->>'target_path'=j.spec_json->>'target_path' AND n.created_at>=j.created_at) RETURNING {ROW}");
        decode(
            sqlx::query(&query)
                .bind(&lease.job_id)
                .bind(&lease.owner)
                .bind(fence)
                .bind(replacement_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(database_error)?
                .ok_or_else(|| invalid("stale lease or invalid replacement"))?,
        )
    }
    pub async fn maintenance_job(&self, id: &str) -> Result<Option<MaintenanceJob>, PortError> {
        bounded(id, 512)?;
        let query = format!("SELECT {ROW} FROM brain.maintenance_jobs_v1 WHERE id=$1");
        sqlx::query(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(database_error)?
            .map(decode)
            .transpose()
    }
    pub async fn maintenance_jobs(
        &self,
        repo_id: &str,
        limit: u32,
    ) -> Result<Vec<MaintenanceJob>, PortError> {
        bounded(repo_id, 512)?;
        if !(1..=1000).contains(&limit) {
            return Err(invalid("invalid maintenance status limit"));
        }
        let query=format!("SELECT {ROW} FROM brain.maintenance_jobs_v1 WHERE repo_id=$1 ORDER BY created_at DESC,id LIMIT $2");
        sqlx::query(&query)
            .bind(repo_id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(database_error)?
            .into_iter()
            .map(decode)
            .collect()
    }
    pub async fn register_maintenance_source(
        &self,
        registration: &MaintenanceSourceRegistration,
    ) -> Result<(), PortError> {
        registration.validate()?;
        sqlx::query("INSERT INTO brain.maintenance_sources_v1(repo_id,registration_json) VALUES($1,$2) ON CONFLICT(repo_id) DO UPDATE SET registration_json=EXCLUDED.registration_json,updated_at=now()").bind(&registration.repo_id).bind(json(registration)?).execute(&self.pool).await.map_err(database_error)?;
        Ok(())
    }
    pub async fn maintenance_sources(
        &self,
    ) -> Result<Vec<MaintenanceSourceRegistration>, PortError> {
        let rows=sqlx::query_scalar::<_,serde_json::Value>("SELECT registration_json FROM brain.maintenance_sources_v1 ORDER BY repo_id LIMIT 1001").fetch_all(&self.pool).await.map_err(database_error)?;
        if rows.len() > 1000 {
            return Err(invalid("maintenance registry limit exceeded"));
        }
        rows.into_iter()
            .map(|v| {
                serde_json::from_value(v).map_err(|_| invalid("invalid maintenance registration"))
            })
            .collect()
    }

    /// Aktualisiert nur den Quellstand einer bereits ausdrücklich freigegebenen Registrierung.
    pub async fn advance_maintenance_source(
        &self,
        registration: &MaintenanceSourceRegistration,
    ) -> Result<(), PortError> {
        registration.validate()?;
        if registration.policy.is_none() {
            return Err(invalid("maintenance policy approval required"));
        }
        let changed = sqlx::query("UPDATE brain.maintenance_sources_v1 SET registration_json=jsonb_set(registration_json,'{discovered_sha}',to_jsonb($2::text)),updated_at=now() WHERE repo_id=$1 AND registration_json->>'source_id'=$3 AND registration_json->'policy'=$4")
            .bind(&registration.repo_id)
            .bind(&registration.discovered_sha)
            .bind(&registration.source_id)
            .bind(json(registration)?.get("policy").cloned().ok_or_else(|| invalid("maintenance policy missing"))?)
            .execute(&self.pool).await.map_err(database_error)?;
        if changed.rows_affected() != 1 {
            return Err(invalid("maintenance registry policy revoked or changed"));
        }
        Ok(())
    }

    /// Bindet einen Provideraufruf an die aktuelle Freigabe; ein Widerruf wird danach wirksam.
    pub async fn lock_maintenance_provider_policy(
        &self,
        spec: &MaintenanceJobSpec,
    ) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, PortError> {
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let value: Option<serde_json::Value> = sqlx::query_scalar(
            "SELECT registration_json FROM brain.maintenance_sources_v1 WHERE repo_id=$1 FOR SHARE",
        )
        .bind(&spec.repo_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(database_error)?;
        let current: MaintenanceSourceRegistration =
            serde_json::from_value(value.ok_or_else(|| invalid("maintenance registry missing"))?)
                .map_err(|_| invalid("maintenance registry JSON"))?;
        current.validate()?;
        if current.policy.as_ref() != Some(&spec.policy)
            || current.discovered_sha != spec.source_sha
        {
            return Err(invalid("maintenance provider policy revoked or changed"));
        }
        Ok(tx)
    }
}

impl MaintenanceStorePort for PgStore {
    fn approve_maintenance_policy<'a>(
        &'a self,
        id: &'a str,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceJob> {
        Box::pin(PgStore::approve_maintenance_policy(self, id))
    }
    fn enqueue_maintenance<'a>(
        &'a self,
        spec: &'a MaintenanceJobSpec,
        initial: MaintenanceStatus,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceJob> {
        Box::pin(PgStore::enqueue_maintenance(self, spec, initial))
    }
    fn claim_maintenance<'a>(
        &'a self,
        owner: &'a str,
        ttl_ms: u64,
    ) -> brain_contracts::StoreFuture<'a, Option<MaintenanceJob>> {
        Box::pin(PgStore::claim_maintenance(self, owner, ttl_ms))
    }
    fn renew_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        ttl_ms: u64,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceLease> {
        Box::pin(PgStore::renew_maintenance(self, lease, ttl_ms))
    }
    fn transition_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        next: MaintenanceStatus,
        checkpoint: &'a MaintenanceCheckpoint,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceJob> {
        Box::pin(PgStore::transition_maintenance(
            self, lease, next, checkpoint,
        ))
    }
    fn retry_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        error_code: &'a str,
        delay_ms: u64,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceJob> {
        Box::pin(PgStore::retry_maintenance(
            self, lease, error_code, delay_ms,
        ))
    }
    fn supersede_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        replacement_id: &'a str,
    ) -> brain_contracts::StoreFuture<'a, MaintenanceJob> {
        Box::pin(PgStore::supersede_maintenance(self, lease, replacement_id))
    }
    fn maintenance_job<'a>(
        &'a self,
        id: &'a str,
    ) -> brain_contracts::StoreFuture<'a, Option<MaintenanceJob>> {
        Box::pin(PgStore::maintenance_job(self, id))
    }
    fn maintenance_jobs<'a>(
        &'a self,
        repo_id: &'a str,
        limit: u32,
    ) -> brain_contracts::StoreFuture<'a, Vec<MaintenanceJob>> {
        Box::pin(PgStore::maintenance_jobs(self, repo_id, limit))
    }
    fn register_maintenance_source<'a>(
        &'a self,
        registration: &'a MaintenanceSourceRegistration,
    ) -> brain_contracts::StoreFuture<'a, ()> {
        Box::pin(PgStore::register_maintenance_source(self, registration))
    }
    fn maintenance_sources(
        &self,
    ) -> brain_contracts::StoreFuture<'_, Vec<MaintenanceSourceRegistration>> {
        Box::pin(PgStore::maintenance_sources(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        source::SourcePolicy,
        value::{Observed, UnknownReason},
        SourceVisibility,
    };
    use std::collections::BTreeSet;
    fn spec() -> MaintenanceJobSpec {
        MaintenanceJobSpec {
            id: "maintenance-race-fixture".into(),
            repo_id: "fixture-repo".into(),
            source_sha: "a".repeat(40),
            deployed_sha: None,
            target_path: "docs/help.md".into(),
            idempotency_key: "maintenance-race-fixture-v1".into(),
            prompt_version: "v2".into(),
            model: "jev-1.13.0".into(),
            policy: SourcePolicy {
                visibility: SourceVisibility::Public,
                allowed_scopes: BTreeSet::new(),
                authorization_ref: Observed::known("fixture".into()),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: true,
                provider_egress_allowed: true,
                raw_retention_allowed: true,
            },
        }
    }
    #[test]
    fn review_requires_separate_runs_and_exact_source() {
        let s = spec();
        let mut cp = MaintenanceCheckpoint {
            review: Some(MaintenanceReviewProof {
                reviewer_id: "reviewer".into(),
                author_run_id: "author-run".into(),
                reviewer_run_id: "review-run".into(),
                source_sha: s.source_sha.clone(),
                document_sha256: "c".repeat(64),
                accepted: true,
            }),
            ..Default::default()
        };
        assert!(cp.validate(&s, MaintenanceStatus::Publish).is_ok());
        cp.review.as_mut().unwrap().reviewer_run_id = "author-run".into();
        assert!(cp.validate(&s, MaintenanceStatus::Publish).is_err());
        cp.review.as_mut().unwrap().reviewer_run_id = "review-run".into();
        cp.review.as_mut().unwrap().source_sha = "b".repeat(40);
        assert!(cp.validate(&s, MaintenanceStatus::Publish).is_err());
        assert!(MaintenanceCheckpoint::default()
            .validate(&s, MaintenanceStatus::Activated)
            .is_err());
        let mut internal = s.clone();
        internal.policy.visibility = SourceVisibility::Private;
        internal.policy.publication_allowed = false;
        cp.review.as_mut().unwrap().source_sha = s.source_sha;
        assert!(cp.validate(&internal, MaintenanceStatus::Publish).is_ok());
        internal.policy.visibility = SourceVisibility::Public;
        assert!(cp.validate(&internal, MaintenanceStatus::Publish).is_err());
    }
    /// Ausschließlich eigener lokaler Scratch-Cluster ohne TCP, ENV oder Credentials.
    #[tokio::test]
    #[ignore = "requires dedicated /tmp/brain-maintenance-test-20261002/socket PostgreSQL fixture"]
    async fn postgres_queue_races_expiry_and_checkpoint_restart() {
        let options = sqlx::postgres::PgConnectOptions::new()
            .host("/tmp/brain-maintenance-test-20261002/socket")
            .port(55447)
            .username("brain_maintenance_test")
            .database("postgres");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options.clone())
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
        assert_eq!(user, "brain_maintenance_test");
        let store = PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        store.migrate_maintenance().await.unwrap();
        store.migrate_maintenance().await.unwrap();
        store.check_maintenance_schema().await.unwrap();
        for role in ["brain_ingest", "brain_service", "brain_readonly"] {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname=$1)")
                    .bind(role)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            if !exists {
                sqlx::raw_sql(&format!(
                    "CREATE ROLE {role} LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE"
                ))
                .execute(&pool)
                .await
                .unwrap();
            }
        }
        let grants = include_str!("../../../../ops/brain-postgres/grants.sql")
            .lines()
            .filter(|line| !line.starts_with('\\'))
            .collect::<Vec<_>>()
            .join("\n");
        sqlx::raw_sql(&grants).execute(&pool).await.unwrap();
        let owner_pool = pool;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options.username("brain_ingest"))
            .await
            .unwrap();
        let runtime_user: String = sqlx::query_scalar("SELECT current_user::text")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(runtime_user, "brain_ingest");
        assert!(
            sqlx::query("CREATE TABLE brain.worker_forbidden_ddl(id integer)")
                .execute(&pool)
                .await
                .is_err()
        );
        assert!(
            sqlx::query("DELETE FROM brain.maintenance_jobs_v1 WHERE false")
                .execute(&pool)
                .await
                .is_err()
        );
        let store = PgStore::new(pool.clone());
        store.check_maintenance_schema().await.unwrap();
        let mut s = spec();
        let run = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        s.id = format!("maintenance-race-{run}");
        s.idempotency_key = s.id.clone();
        let (left, right) = tokio::join!(
            store.enqueue_maintenance(&s, MaintenanceStatus::Planned),
            store.enqueue_maintenance(&s, MaintenanceStatus::Planned)
        );
        assert_eq!(left.unwrap(), right.unwrap());
        let mut collision = s.clone();
        collision.source_sha = "b".repeat(40);
        assert!(store
            .enqueue_maintenance(&collision, MaintenanceStatus::Planned)
            .await
            .is_err());
        let (a, b) = tokio::join!(
            store.claim_maintenance("a", 1000),
            store.claim_maintenance("b", 1000)
        );
        let a = a.unwrap();
        let b = b.unwrap();
        assert_eq!(usize::from(a.is_some()) + usize::from(b.is_some()), 1);
        let expired = a.or(b).unwrap().lease.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        let live = store
            .claim_maintenance("restarted", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        assert!(live.fence > expired.fence);
        assert!(store.renew_maintenance(&expired, 30000).await.is_err());
        assert!(store
            .retry_maintenance(&expired, "OLD_WORKER", 0)
            .await
            .is_err());
        let cp = MaintenanceCheckpoint {
            artifact_refs: std::collections::BTreeMap::from([(
                "prepared".into(),
                "fixture-artifact-sha".into(),
            )]),
            ..Default::default()
        };
        assert!(store
            .transition_maintenance(&expired, MaintenanceStatus::SourceReview, &cp)
            .await
            .is_err());
        store.renew_maintenance(&live, 60000).await.unwrap();
        store
            .transition_maintenance(&live, MaintenanceStatus::SourceReview, &cp)
            .await
            .unwrap();
        let restarted = PgStore::new(pool.clone());
        assert_eq!(
            restarted
                .maintenance_job(&s.id)
                .await
                .unwrap()
                .unwrap()
                .checkpoint,
            cp
        );
        let lease = restarted
            .claim_maintenance("review", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        assert!(restarted
            .transition_maintenance(&lease, MaintenanceStatus::Activated, &cp)
            .await
            .is_err());
        restarted
            .retry_maintenance(&lease, "TEMPORARY_FAILURE", 0)
            .await
            .unwrap();
        assert!(restarted
            .retry_maintenance(&lease, "OLD_WORKER", 0)
            .await
            .is_err());
        let lease = restarted
            .claim_maintenance("final", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        restarted
            .transition_maintenance(&lease, MaintenanceStatus::Failed, &cp)
            .await
            .unwrap();
        assert!(restarted
            .claim_maintenance("none", 30000)
            .await
            .unwrap()
            .is_none());
        // Corpus-Nachweis und ACL müssen auch bei korrekt gefenctem Worker stimmen.
        let mut publication_spec = s.clone();
        publication_spec.id.push_str("-publish");
        publication_spec.idempotency_key = publication_spec.id.clone();
        restarted
            .enqueue_maintenance(&publication_spec, MaintenanceStatus::Planned)
            .await
            .unwrap();
        for next in [MaintenanceStatus::SourceReview, MaintenanceStatus::Reviewer] {
            let lease = restarted
                .claim_maintenance("prepare", 30000)
                .await
                .unwrap()
                .unwrap()
                .lease
                .unwrap();
            restarted
                .transition_maintenance(&lease, next, &MaintenanceCheckpoint::default())
                .await
                .unwrap();
        }
        use sha2::Digest;
        let document = "fixture reviewed document";
        let hash = format!("{:x}", sha2::Sha256::digest(document.as_bytes()));
        let mut approved = MaintenanceCheckpoint {
            review: Some(MaintenanceReviewProof {
                reviewer_id: "independent-fixture".into(),
                author_run_id: "fixture-author-turn".into(),
                reviewer_run_id: "fixture-review-turn".into(),
                source_sha: publication_spec.source_sha.clone(),
                document_sha256: hash.clone(),
                accepted: true,
            }),
            ..Default::default()
        };
        let lease = restarted
            .claim_maintenance("reviewer", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        restarted
            .transition_maintenance(&lease, MaintenanceStatus::Publish, &approved)
            .await
            .unwrap();
        let lease = restarted
            .claim_maintenance("publisher", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        approved.publication = Some(MaintenancePublicationProof {
            release_id: format!("fixture-release-{run}"),
            source_id: publication_spec.id.clone(),
            logical_id: publication_spec.target_path.clone(),
            document_revision: 1,
            document_sha256: hash.clone(),
        });
        approved.activation = Some(MaintenanceActivationProof {
            active_release_id: approved.publication.as_ref().unwrap().release_id.clone(),
            verified_at_epoch: 1,
            evidence_ref: "isolated-fixture-live-read".into(),
        });
        assert!(restarted
            .transition_maintenance(&lease, MaintenanceStatus::Activated, &approved)
            .await
            .is_err());
        let mut record = brain_contracts::SourceRecordV2 {
            source_id: publication_spec.id.clone(),
            logical_id: publication_spec.target_path.clone(),
            revision: 1,
            content_hash: hash,
            content: document.into(),
            visibility: SourceVisibility::Private,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: Default::default(),
        };
        restarted.apply(&record).await.unwrap();
        let mut release = CorpusRelease {
            release_id: approved.publication.as_ref().unwrap().release_id.clone(),
            knowledge_version: "fixture".into(),
            patch: "fixture".into(),
            created_at_epoch: 1,
            source_revisions: std::collections::BTreeMap::from([(
                record.source_id.clone(),
                std::collections::BTreeMap::from([(record.logical_id.clone(), 1)]),
            )]),
        };
        restarted.publish_release(&release).await.unwrap();
        assert!(restarted
            .transition_maintenance(&lease, MaintenanceStatus::Activated, &approved)
            .await
            .is_err());
        record.revision = 2;
        record.visibility = SourceVisibility::Public;
        use brain_contracts::source::{
            GameValidity, OriginArtifact, SourceIdentity, SourceRevision,
        };
        let origin = OriginArtifact {
            identity: SourceIdentity {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
            },
            source_revision: SourceRevision::Git {
                commit: publication_spec.source_sha.clone(),
            },
            raw_sha256: record.content_hash.clone(),
            locator: "fixture://reviewed-document".into(),
            parser_revision: "fixture-v1".into(),
            parser_family: "documentation".into(),
            schema_version: Observed::unknown(UnknownReason::NotPresent),
            schema_sha256: Observed::unknown(UnknownReason::NotPresent),
            retrieved_at: Observed::unknown(UnknownReason::NotPresent),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::known("de".into()),
            origin_artifacts: BTreeSet::new(),
            derivation_family: Observed::known("fixture".into()),
            policy: publication_spec.policy.clone(),
            validity: GameValidity::unknown(),
        };
        let mut wrong_origin = origin.clone();
        wrong_origin.policy.raw_retention_allowed = false;
        wrong_origin.bind_record(&mut record).unwrap();
        restarted.apply(&record).await.unwrap();
        release.release_id.push_str("-wrong-policy");
        release
            .source_revisions
            .get_mut(&record.source_id)
            .unwrap()
            .insert(record.logical_id.clone(), 2);
        restarted.publish_release(&release).await.unwrap();
        approved.publication.as_mut().unwrap().release_id = release.release_id.clone();
        approved.publication.as_mut().unwrap().document_revision = 2;
        approved.activation.as_mut().unwrap().active_release_id = release.release_id.clone();
        assert!(restarted
            .transition_maintenance(&lease, MaintenanceStatus::Activated, &approved)
            .await
            .is_err());
        record.revision = 3;
        origin.bind_record(&mut record).unwrap();
        restarted.apply(&record).await.unwrap();
        release.release_id.push_str("-public");
        release
            .source_revisions
            .get_mut(&record.source_id)
            .unwrap()
            .insert(record.logical_id.clone(), 3);
        restarted.publish_release(&release).await.unwrap();
        approved.publication.as_mut().unwrap().release_id = release.release_id.clone();
        approved.publication.as_mut().unwrap().document_revision = 3;
        approved.activation.as_mut().unwrap().active_release_id = release.release_id;
        restarted
            .transition_maintenance(&lease, MaintenanceStatus::Activated, &approved)
            .await
            .unwrap();
        assert!(restarted.renew_maintenance(&lease, 30000).await.is_err());
        let mut waiting = s.clone();
        waiting.id.push_str("-policy");
        waiting.repo_id = waiting.id.clone();
        waiting.idempotency_key = waiting.id.clone();
        waiting.policy.provider_egress_allowed = false;
        restarted
            .enqueue_maintenance(&waiting, MaintenanceStatus::DiscoveredPolicyPending)
            .await
            .unwrap();
        assert!(restarted
            .approve_maintenance_policy(&waiting.id)
            .await
            .is_err());
        let mut registration = MaintenanceSourceRegistration {
            repo_id: waiting.repo_id.clone(),
            source_id: waiting.repo_id.clone(),
            discovered_sha: waiting.source_sha.clone(),
            policy: None,
            authorization_ref: None,
        };
        restarted
            .register_maintenance_source(&registration)
            .await
            .unwrap();
        assert!(restarted
            .approve_maintenance_policy(&waiting.id)
            .await
            .is_err());
        registration.policy = Some(s.policy.clone());
        registration.authorization_ref = Some("fixture-user-authorization".into());
        restarted
            .register_maintenance_source(&registration)
            .await
            .unwrap();
        let ready = restarted
            .approve_maintenance_policy(&waiting.id)
            .await
            .unwrap();
        assert_eq!(ready.status, MaintenanceStatus::SourceReview);
        assert_eq!(ready.spec.policy, s.policy);
        // Der automatische Worker hält noch die alte Freigabe, während der Betreiber widerruft.
        let approved_snapshot = registration.clone();
        registration.policy = None;
        registration.authorization_ref = None;
        restarted
            .register_maintenance_source(&registration)
            .await
            .unwrap();
        assert!(restarted
            .advance_maintenance_source(&approved_snapshot)
            .await
            .is_err());
        let mut provider_spec = waiting.clone();
        provider_spec.policy = approved_snapshot.policy.clone().unwrap();
        assert!(restarted
            .lock_maintenance_provider_policy(&provider_spec)
            .await
            .is_err());
        assert!(restarted
            .maintenance_sources()
            .await
            .unwrap()
            .into_iter()
            .find(|r| r.repo_id == registration.repo_id)
            .unwrap()
            .policy
            .is_none());
        let replay = restarted
            .enqueue_maintenance(&waiting, MaintenanceStatus::DiscoveredPolicyPending)
            .await
            .unwrap();
        assert_eq!(replay, ready);
        assert_eq!(
            ready.checkpoint.artifact_refs["policy_authorization_ref"],
            "fixture-user-authorization"
        );
        let lease = restarted
            .claim_maintenance("authorized", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        restarted
            .transition_maintenance(&lease, MaintenanceStatus::Failed, &ready.checkpoint)
            .await
            .unwrap();
        // Der echte Corpus-Commit verweigert auch eine während des Schreibens abgelaufene Joblease.
        let mut guarded = s.clone();
        guarded.id.push_str("-guard");
        guarded.repo_id = guarded.id.clone();
        guarded.idempotency_key = guarded.id.clone();
        let registry = MaintenanceSourceRegistration {
            repo_id: guarded.repo_id.clone(),
            source_id: guarded.id.clone(),
            discovered_sha: guarded.source_sha.clone(),
            policy: Some(guarded.policy.clone()),
            authorization_ref: Some("fixture-guarded-publish".into()),
        };
        restarted
            .register_maintenance_source(&registry)
            .await
            .unwrap();
        restarted
            .enqueue_maintenance(&guarded, MaintenanceStatus::Planned)
            .await
            .unwrap();
        for next in [MaintenanceStatus::SourceReview, MaintenanceStatus::Reviewer] {
            let l = restarted
                .claim_maintenance("guard-prepare", 30000)
                .await
                .unwrap()
                .unwrap()
                .lease
                .unwrap();
            restarted
                .transition_maintenance(&l, next, &MaintenanceCheckpoint::default())
                .await
                .unwrap();
        }
        let mut reviewed = approved.clone();
        reviewed.publication = None;
        reviewed.activation = None;
        let l = restarted
            .claim_maintenance("guard-review", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        restarted
            .transition_maintenance(&l, MaintenanceStatus::Publish, &reviewed)
            .await
            .unwrap();
        let mut guarded_record = record.clone();
        guarded_record.source_id = guarded.id.clone();
        guarded_record.revision = 1;
        guarded_record.metadata.clear();
        let mut guarded_origin = origin.clone();
        guarded_origin.identity.source_id = guarded.id.clone();
        guarded_origin.bind_record(&mut guarded_record).unwrap();
        let mut neighbor = guarded_record.clone();
        neighbor.logical_id = "internal/unberuehrte-seite.html".into();
        let mut neighbor_origin = guarded_origin.clone();
        neighbor_origin.identity.logical_id = neighbor.logical_id.clone();
        neighbor_origin.bind_record(&mut neighbor).unwrap();
        let seed = brain_contracts::SourceBatch {
            expected_generation: 0,
            checkpoint: brain_contracts::SourceCheckpoint {
                source_id: guarded.id.clone(),
                configuration: "guard-fixture".into(),
                generation: 1,
                state: serde_json::json!({}),
            },
            records: vec![neighbor.clone()],
        };
        let seed_lease =
            brain_contracts::DocumentStorePort::claim(&restarted, &guarded.id, "seed", 30000)
                .await
                .unwrap();
        let mut base = restarted
            .snapshot(&approved.publication.as_ref().unwrap().release_id)
            .await
            .unwrap()
            .release;
        base.release_id.push_str("-neighbor");
        base.source_revisions.insert(
            guarded.id.clone(),
            std::collections::BTreeMap::from([(neighbor.logical_id.clone(), 1)]),
        );
        restarted
            .commit_batches_and_publish(&[(&seed, &seed_lease)], &base)
            .await
            .unwrap();
        let expected_heads = vec![guarded_record.clone(), neighbor.clone()];
        let batch = brain_contracts::SourceBatch {
            expected_generation: 1,
            checkpoint: brain_contracts::SourceCheckpoint {
                source_id: guarded.id.clone(),
                configuration: "guard-fixture".into(),
                generation: 2,
                state: serde_json::json!({}),
            },
            records: vec![guarded_record.clone()],
        };
        let source_lease = brain_contracts::DocumentStorePort::claim(
            &restarted,
            &guarded.id,
            "fixture-source",
            30000,
        )
        .await
        .unwrap();
        let base_id = base.release_id.clone();
        let mut guarded_release = restarted.snapshot(&base_id).await.unwrap().release;
        guarded_release.release_id.push_str("-guarded");
        guarded_release.source_revisions.insert(
            guarded.id.clone(),
            std::collections::BTreeMap::from([
                (guarded.target_path.clone(), 1),
                (neighbor.logical_id.clone(), 1),
            ]),
        );
        let short = restarted
            .claim_maintenance("short-publisher", 100)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        sqlx::raw_sql("CREATE OR REPLACE FUNCTION brain.fixture_maintenance_delay() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.source_id LIKE '%-guard' THEN PERFORM pg_sleep(0.2); END IF; RETURN NEW; END $$; DROP TRIGGER IF EXISTS fixture_maintenance_delay ON brain.source_record_revisions; CREATE TRIGGER fixture_maintenance_delay BEFORE INSERT ON brain.source_record_revisions FOR EACH ROW EXECUTE FUNCTION brain.fixture_maintenance_delay()").execute(&owner_pool).await.unwrap();
        assert!(restarted
            .commit_maintenance_batches_and_publish_checked(
                &short,
                &base_id,
                &[(&batch, &source_lease)],
                &guarded_release,
                &expected_heads
            )
            .await
            .is_err());
        assert!(
            brain_contracts::DocumentStorePort::checkpoint(&restarted, &guarded.id)
                .await
                .unwrap()
                .unwrap()
                .generation
                == 1
        );
        assert!(restarted
            .snapshot(&guarded_release.release_id)
            .await
            .is_err());
        sqlx::raw_sql("DROP TRIGGER fixture_maintenance_delay ON brain.source_record_revisions; DROP FUNCTION brain.fixture_maintenance_delay()").execute(&owner_pool).await.unwrap();
        let live = restarted
            .claim_maintenance("live-publisher", 30000)
            .await
            .unwrap()
            .unwrap()
            .lease
            .unwrap();
        assert!(restarted
            .commit_maintenance_batches_and_publish_checked(
                &short,
                &base_id,
                &[(&batch, &source_lease)],
                &guarded_release,
                &expected_heads
            )
            .await
            .is_err());
        let mut losing = guarded_release.clone();
        losing.source_revisions.remove(&publication_spec.id);
        assert!(restarted
            .commit_maintenance_batches_and_publish_checked(
                &live,
                &base_id,
                &[(&batch, &source_lease)],
                &losing,
                &expected_heads
            )
            .await
            .is_err());
        let mut lost_neighbor = guarded_release.clone();
        lost_neighbor
            .source_revisions
            .get_mut(&guarded.id)
            .unwrap()
            .remove(&neighbor.logical_id);
        assert!(restarted
            .commit_maintenance_batches_and_publish_checked(
                &live,
                &base_id,
                &[(&batch, &source_lease)],
                &lost_neighbor,
                &[guarded_record.clone()],
            )
            .await
            .is_err());
        assert_eq!(
            brain_contracts::DocumentStorePort::checkpoint(&restarted, &guarded.id)
                .await
                .unwrap()
                .unwrap()
                .generation,
            1
        );
        restarted
            .commit_maintenance_batches_and_publish_checked(
                &live,
                &base_id,
                &[(&batch, &source_lease)],
                &guarded_release,
                &expected_heads,
            )
            .await
            .unwrap();
        let saved = restarted
            .maintenance_job(&guarded.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            saved.checkpoint.publication.as_ref().unwrap().release_id,
            guarded_release.release_id
        );
        let imported = restarted
            .snapshot(&guarded_release.release_id)
            .await
            .unwrap();
        assert_eq!(
            imported.release.source_revisions[&publication_spec.id],
            release.source_revisions[&publication_spec.id]
        );
        let rolled_back = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rollback_flag = rolled_back.clone();
        assert!(restarted
            .activate_maintenance_checked(
                &live,
                1000,
                |_| async { Err(PortError::Unavailable("fixture activation failure".into())) },
                || async move {
                    rollback_flag.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                }
            )
            .await
            .is_err());
        assert!(rolled_back.load(std::sync::atomic::Ordering::SeqCst));
        assert_eq!(
            restarted
                .maintenance_job(&guarded.id)
                .await
                .unwrap()
                .unwrap()
                .status,
            MaintenanceStatus::Publish
        );
        let activation = MaintenanceActivationProof {
            active_release_id: guarded_release.release_id.clone(),
            verified_at_epoch: 1,
            evidence_ref: "guard-fixture-health".into(),
        };
        restarted
            .activate_maintenance_checked(
                &live,
                1000,
                |_| async { Ok(activation) },
                || async { Ok(()) },
            )
            .await
            .unwrap();
        pool.close().await;
        owner_pool.close().await;
    }
}
