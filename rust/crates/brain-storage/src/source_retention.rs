use brain_contracts::PortError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub const OWNED_SOURCE_RETENTION_SQL: &str = include_str!("pg_source_retention.sql");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRetentionRequest {
    source_id: String,
    generation: i64,
}

impl SourceRetentionRequest {
    pub fn new(source_id: &str, generation: u64) -> Result<Self, PortError> {
        let suffix = source_id
            .strip_prefix("google-sheet/")
            .or_else(|| source_id.strip_prefix("youtube-core/"));
        if suffix.is_none_or(|suffix| suffix.trim().is_empty())
            || source_id.len() > 512
            || source_id.chars().any(char::is_control)
            || generation == 0
            || generation > i64::MAX as u64
        {
            return Err(invalid(
                "Ungültige Quelle oder Speicherfassung für die Aufbewahrung.",
            ));
        }
        Ok(Self {
            source_id: source_id.into(),
            generation: generation as i64,
        })
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn generation(&self) -> u64 {
        self.generation as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRetentionReport {
    pub source_id: String,
    pub generation: u64,
    pub candidate_revisions: u64,
    pub deleted_revisions: u64,
    pub release_blocked_revisions: u64,
    pub checkpoint_blocked_revisions: u64,
    pub missing_source_revisions: u64,
    pub blocking_release_ids: Vec<String>,
}

impl SourceRetentionReport {
    pub fn storage_purge_complete(&self) -> bool {
        self.deleted_revisions == self.candidate_revisions
            && self.release_blocked_revisions == 0
            && self.checkpoint_blocked_revisions == 0
            && self.blocking_release_ids.is_empty()
    }

    fn validate(&self, request: &SourceRetentionRequest) -> Result<(), PortError> {
        let accounted = self
            .deleted_revisions
            .checked_add(self.release_blocked_revisions)
            .and_then(|n| n.checked_add(self.checkpoint_blocked_revisions));
        if self.source_id != request.source_id
            || self.generation != request.generation as u64
            || accounted != Some(self.candidate_revisions)
            || self.missing_source_revisions > self.candidate_revisions
            || (self.release_blocked_revisions == 0) != self.blocking_release_ids.is_empty()
            || self
                .blocking_release_ids
                .iter()
                .any(|id| id.trim().is_empty())
        {
            return Err(invalid(
                "Der Speicherbeleg zur Aufbewahrung ist unvollständig.",
            ));
        }
        Ok(())
    }
}

fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable(
        "Die Quellenaufbewahrung konnte nicht im Speicher geprüft werden.".into(),
    )
}

pub async fn check_source_retention_schema(pool: &PgPool) -> Result<(), PortError> {
    let installed: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace
         WHERE n.nspname='brain' AND p.proname='purge_owned_source_history_v1'
         AND p.oid=to_regprocedure('brain.purge_owned_source_history_v1(text,bigint)')
         AND p.prosecdef AND has_function_privilege(current_user,p.oid,'EXECUTE'))",
    )
    .fetch_one(pool)
    .await
    .map_err(database_error)?;
    if !installed {
        return Err(invalid(
            "Der begrenzte Aufbewahrungspfad ist noch nicht eingerichtet.",
        ));
    }
    Ok(())
}

pub async fn purge_owned_source_history(
    pool: &PgPool,
    request: &SourceRetentionRequest,
) -> Result<SourceRetentionReport, PortError> {
    let mut tx = pool.begin().await.map_err(database_error)?;
    let value: serde_json::Value =
        sqlx::query_scalar("SELECT brain.purge_owned_source_history_v1($1::text,$2::bigint)")
            .bind(&request.source_id)
            .bind(request.generation)
            .fetch_one(&mut *tx)
            .await
            .map_err(database_error)?;
    let report: SourceRetentionReport = serde_json::from_value(value)
        .map_err(|_| invalid("Der Speicherbeleg zur Aufbewahrung ist ungültig."))?;
    report.validate(request)?;
    tx.commit().await.map_err(database_error)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_is_limited_to_the_two_owned_source_families() {
        for source in ["google-sheet/fixture", "youtube-core/fixture"] {
            assert!(SourceRetentionRequest::new(source, 1).is_ok());
        }
        for source in [
            "google-sheet/",
            "youtube-core/ ",
            "legacy/google-sheet/fixture",
            "youtube/fixture",
            "google-sheet/fixture\n",
            "GOOGLE-SHEET/fixture",
        ] {
            assert!(SourceRetentionRequest::new(source, 1).is_err());
        }
        assert!(SourceRetentionRequest::new("google-sheet/fixture", 0).is_err());
        assert!(SourceRetentionRequest::new("google-sheet/fixture", u64::MAX).is_err());
        assert!(
            SourceRetentionRequest::new(&format!("google-sheet/{}", "a".repeat(512)), 1).is_err()
        );
    }

    #[test]
    fn request_preserves_the_largest_signed_generation() {
        let request =
            SourceRetentionRequest::new("youtube-core/fixture", i64::MAX as u64).unwrap();
        assert_eq!(request.source_id(), "youtube-core/fixture");
        assert_eq!(request.generation(), i64::MAX as u64);
        assert!(
            SourceRetentionRequest::new("youtube-core/fixture", i64::MAX as u64 + 1).is_err()
        );
    }

    fn report() -> SourceRetentionReport {
        SourceRetentionReport {
            source_id: "google-sheet/fixture".into(),
            generation: 1,
            candidate_revisions: 3,
            deleted_revisions: 1,
            release_blocked_revisions: 1,
            checkpoint_blocked_revisions: 1,
            missing_source_revisions: 2,
            blocking_release_ids: vec!["immutable-release".into()],
        }
    }

    #[test]
    fn blocked_purge_is_not_a_storage_completion() {
        let request = SourceRetentionRequest::new("google-sheet/fixture", 1).unwrap();
        let mut report = report();
        report.validate(&request).unwrap();
        assert!(!report.storage_purge_complete());
        report.release_blocked_revisions = 0;
        report.checkpoint_blocked_revisions = 0;
        report.deleted_revisions = 3;
        report.blocking_release_ids.clear();
        report.validate(&request).unwrap();
        assert!(report.storage_purge_complete());
    }

    #[test]
    fn response_rejects_generation_identity_and_count_mismatches() {
        let request = SourceRetentionRequest::new("google-sheet/fixture", 1).unwrap();
        let mut response = report();
        response.generation = 2;
        assert!(response.validate(&request).is_err());
        response = report();
        response.source_id = "youtube-core/fixture".into();
        assert!(response.validate(&request).is_err());
        response = report();
        response.deleted_revisions = u64::MAX;
        assert!(response.validate(&request).is_err());
        response = report();
        response.blocking_release_ids.clear();
        assert!(response.validate(&request).is_err());
    }

    #[test]
    fn sql_contract_keeps_current_heads_and_immutable_pins() {
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("interval '12 months'"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("r.revision < h.revision"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("IN SHARE ROW EXCLUSIVE MODE"));
        assert!(
            OWNED_SOURCE_RETENTION_SQL.contains("jsonb_array_elements(p.batch_json->'records')")
        );
        assert!(!OWNED_SOURCE_RETENTION_SQL.contains("DELETE FROM brain.source_record_heads"));
        assert!(!OWNED_SOURCE_RETENTION_SQL.contains("DELETE FROM brain.corpus_releases_v1"));
    }

    #[test]
    fn sql_contract_captures_supersessions_before_deleting_intermediate_revisions() {
        let capture = OWNED_SOURCE_RETENTION_SQL
            .find("INSERT INTO brain.source_retention_supersessions_v1")
            .unwrap();
        let selection = OWNED_SOURCE_RETENTION_SQL.find("FOR candidate IN").unwrap();
        assert!(capture < selection);
        assert!(
            OWNED_SOURCE_RETENTION_SQL[capture..selection]
                .contains("ON CONFLICT (source_id, logical_id, revision) DO NOTHING")
        );
        assert!(
            OWNED_SOURCE_RETENTION_SQL[capture..selection].contains("min(n.created_at)")
        );
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("h.tombstone OR s.superseded_at < cutoff"));
        assert!(!OWNED_SOURCE_RETENTION_SQL.contains("UPDATE brain.source_retention_supersessions_v1"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains(
            "DELETE FROM brain.source_retention_supersessions_v1\n        WHERE source_id = p_source AND logical_id = candidate.logical_id AND revision = candidate.revision"
        ));
    }

    #[test]
    fn sql_contract_fences_jobs_and_materialized_dependencies_without_delete_grants() {
        assert!(OWNED_SOURCE_RETENTION_SQL.contains(
            "brain.source_checkpoints_v1, brain.source_jobs_v1 IN SHARE ROW EXCLUSIVE MODE"
        ));
        assert!(OWNED_SOURCE_RETENTION_SQL
            .contains("LOCK TABLE brain.corpus_releases_v1 IN ACCESS EXCLUSIVE MODE"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("brain.domain.dependencies"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("origin_artifacts"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("btrim(substr(p_source"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("OWNER TO brain_migrate"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("SECURITY DEFINER"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains("SET search_path = pg_catalog"));
        assert!(OWNED_SOURCE_RETENTION_SQL.contains(
            "GRANT EXECUTE ON FUNCTION brain.purge_owned_source_history_v1(text, bigint) TO brain_ingest"
        ));
        assert!(!OWNED_SOURCE_RETENTION_SQL.contains("GRANT DELETE"));
    }
}
