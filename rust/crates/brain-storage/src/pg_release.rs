use crate::{memory_repository::validate_release, pg_jobs::database_error, PgStore};
use brain_contracts::{CorpusRelease, CorpusSnapshot, PortError, SourceRecordV2};
use sqlx::Row;
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

impl PgStore {
    pub async fn publish_release(&self, release: &CorpusRelease) -> Result<(), PortError> {
        validate_release(release)?;
        let mut tx = self.pool.begin().await.map_err(database_error)?;
        let key = format!("core-release:{}", release.release_id);
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(key)
            .execute(&mut *tx)
            .await
            .map_err(database_error)?;
        let value = serde_json::to_value(release).map_err(|_| invalid("invalid release JSON"))?;
        if let Some(old) = sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(&release.release_id)
        .fetch_optional(&mut *tx)
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
                    .bind(source).bind(logical).bind(*revision as i64).fetch_one(&mut *tx).await.map_err(database_error)?;
                if !found {
                    return Err(invalid("release references missing revision"));
                }
            }
        }
        sqlx::query("INSERT INTO brain.corpus_releases_v1(release_id,knowledge_version,patch,release_json) VALUES($1,$2,$3,$4)")
            .bind(&release.release_id).bind(&release.knowledge_version).bind(&release.patch).bind(value).execute(&mut *tx).await.map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        Ok(())
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
