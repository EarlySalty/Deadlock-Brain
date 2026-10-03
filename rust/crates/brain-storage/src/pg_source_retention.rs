use crate::{source_retention, PgStore};
use brain_contracts::PortError;

impl PgStore {
    pub async fn check_source_retention_schema(&self) -> Result<(), PortError> {
        source_retention::check_source_retention_schema(&self.pool).await
    }

    pub async fn purge_owned_source_history(
        &self,
        request: &source_retention::SourceRetentionRequest,
    ) -> Result<source_retention::SourceRetentionReport, PortError> {
        source_retention::purge_owned_source_history(&self.pool, request).await
    }
}
