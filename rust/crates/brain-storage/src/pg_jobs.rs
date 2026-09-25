use crate::{ApplyOutcome, PgStore};
use brain_contracts::{
    BatchReceipt, CorpusRelease, DocumentStorePort, Lease, PortError, SourceBatch,
    SourceCheckpoint, StoreFuture,
};
use sqlx::Row;
pub(crate) fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Postgres storage operation failed".into())
}
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

impl PgStore {
    /// Explicit privileged operation. Never called implicitly from a query or constructor.
    pub async fn migrate_core(&self) -> Result<(), PortError> {
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-09-24-brain-contract-v1.sql"
        ))
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-09-25-brain-core-jobs-v2.sql"
        ))
        .execute(&self.pool)
        .await
        .map_err(database_error)?;
        Ok(())
    }
}
impl DocumentStorePort for PgStore {
    fn checkpoint<'a>(&'a self, source: &'a str) -> StoreFuture<'a, Option<SourceCheckpoint>> {
        Box::pin(async move {
            let row = sqlx::query("SELECT generation, configuration, checkpoint_json FROM brain.source_checkpoints_v1 WHERE source_id=$1")
                .bind(source).fetch_optional(&self.pool).await.map_err(database_error)?;
            row.map(|r| {
                let value: serde_json::Value =
                    r.try_get("checkpoint_json").map_err(database_error)?;
                let checkpoint: SourceCheckpoint = serde_json::from_value(value)
                    .map_err(|_| invalid("invalid stored checkpoint"))?;
                let generation: i64 = r.try_get("generation").map_err(database_error)?;
                let configuration: String = r.try_get("configuration").map_err(database_error)?;
                if checkpoint.source_id != source
                    || checkpoint.generation != generation as u64
                    || checkpoint.configuration != configuration
                {
                    return Err(invalid("checkpoint columns disagree"));
                }
                Ok(checkpoint)
            })
            .transpose()
        })
    }
    fn claim<'a>(&'a self, source: &'a str, owner: &'a str, ttl_ms: u64) -> StoreFuture<'a, Lease> {
        Box::pin(async move {
            if [source, owner]
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 512 || s.chars().any(char::is_control))
                || !(1..=60000).contains(&ttl_ms)
            {
                return Err(invalid("invalid lease parameters"));
            }
            let row = sqlx::query("INSERT INTO brain.source_jobs_v1(source_id,owner,fence,lease_until,state) VALUES($1,$2,1,clock_timestamp()+($3::bigint * interval '1 millisecond'),'running') ON CONFLICT(source_id) DO UPDATE SET owner=EXCLUDED.owner,fence=brain.source_jobs_v1.fence+1,lease_until=EXCLUDED.lease_until,state='running',updated_at=now() WHERE brain.source_jobs_v1.lease_until<=clock_timestamp() OR brain.source_jobs_v1.state='idle' RETURNING fence,(extract(epoch FROM lease_until)*1000)::bigint AS expires")
                .bind(source).bind(owner).bind(ttl_ms as i64).fetch_optional(&self.pool).await.map_err(database_error)?
                .ok_or_else(|| invalid("source already leased"))?;
            Ok(Lease {
                source_id: source.into(),
                owner: owner.into(),
                fence: row.try_get::<i64, _>("fence").map_err(database_error)? as u64,
                expires_at_ms: row.try_get::<i64, _>("expires").map_err(database_error)? as u64,
            })
        })
    }
    fn commit<'a>(
        &'a self,
        batch: &'a SourceBatch,
        lease: &'a Lease,
    ) -> StoreFuture<'a, BatchReceipt> {
        Box::pin(async move {
            batch.validate()?;
            if lease.source_id != batch.checkpoint.source_id
                || lease.fence == 0
                || lease.fence > i64::MAX as u64
            {
                return Err(invalid("invalid batch lease"));
            }
            let batch_json =
                serde_json::to_value(batch).map_err(|_| invalid("invalid batch JSON"))?;
            let mut tx = self.pool.begin().await.map_err(database_error)?;
            // Claim and commit serialize on the same source row. Old fences cannot publish.
            let job = sqlx::query("SELECT owner,fence,(lease_until>clock_timestamp() AND state='running') AS active FROM brain.source_jobs_v1 WHERE source_id=$1 FOR UPDATE")
                .bind(&lease.source_id).fetch_optional(&mut *tx).await.map_err(database_error)?.ok_or_else(|| invalid("lease missing"))?;
            let old = sqlx::query("SELECT generation,batch_json FROM brain.source_checkpoints_v1 WHERE source_id=$1 FOR UPDATE")
                .bind(&lease.source_id).fetch_optional(&mut *tx).await.map_err(database_error)?;
            let generation = old
                .as_ref()
                .map(|r| r.try_get::<i64, _>("generation"))
                .transpose()
                .map_err(database_error)?
                .unwrap_or(0) as u64;
            if generation == batch.checkpoint.generation
                && old
                    .as_ref()
                    .map(|r| r.try_get::<serde_json::Value, _>("batch_json"))
                    .transpose()
                    .map_err(database_error)?
                    .as_ref()
                    == Some(&batch_json)
            {
                tx.commit().await.map_err(database_error)?;
                return Ok(BatchReceipt {
                    generation,
                    replayed: true,
                });
            }
            if generation != batch.expected_generation {
                return Err(invalid("checkpoint compare-and-swap failed"));
            }
            if job.try_get::<String, _>("owner").map_err(database_error)? != lease.owner
                || job.try_get::<i64, _>("fence").map_err(database_error)? as u64 != lease.fence
                || !job.try_get::<bool, _>("active").map_err(database_error)?
            {
                return Err(invalid("stale lease"));
            }
            let mut records: Vec<_> = batch.records.iter().collect();
            records.sort_by(|a, b| a.logical_id.cmp(&b.logical_id));
            for record in records {
                let outcome = Self::apply_connection(&mut tx, record)
                    .await
                    .map_err(|_| invalid("batch record conflict"))?;
                if outcome == ApplyOutcome::IgnoredStale {
                    return Err(invalid("stale batch record"));
                }
            }
            let checkpoint_json = serde_json::to_value(&batch.checkpoint)
                .map_err(|_| invalid("invalid checkpoint JSON"))?;
            sqlx::query("INSERT INTO brain.source_checkpoints_v1(source_id,configuration,generation,checkpoint_json,batch_json) VALUES($1,$2,$3,$4,$5) ON CONFLICT(source_id) DO UPDATE SET configuration=EXCLUDED.configuration,generation=EXCLUDED.generation,checkpoint_json=EXCLUDED.checkpoint_json,batch_json=EXCLUDED.batch_json,updated_at=now()")
                .bind(&lease.source_id).bind(&batch.checkpoint.configuration).bind(batch.checkpoint.generation as i64).bind(checkpoint_json).bind(batch_json).execute(&mut *tx).await.map_err(database_error)?;
            let completed = sqlx::query("UPDATE brain.source_jobs_v1 SET state='idle',lease_until=clock_timestamp(),updated_at=now() WHERE source_id=$1 AND lease_until>clock_timestamp() AND state='running'")
                .bind(&lease.source_id).execute(&mut *tx).await.map_err(database_error)?;
            if completed.rows_affected() != 1 {
                return Err(invalid("lease expired during batch"));
            }
            tx.commit().await.map_err(database_error)?;
            Ok(BatchReceipt {
                generation: batch.checkpoint.generation,
                replayed: false,
            })
        })
    }
    fn publish<'a>(&'a self, release: &'a CorpusRelease) -> StoreFuture<'a, ()> {
        Box::pin(async move { self.publish_release(release).await })
    }
}
