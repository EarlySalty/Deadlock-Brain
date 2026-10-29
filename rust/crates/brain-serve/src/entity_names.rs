use brain_contracts::{AuthorizedContext, PortError};
use sqlx::PgPool;
use std::collections::BTreeSet;

const ENTITY_NAMES: &str = "SELECT DISTINCT a.alias_norm FROM serve.current_alias a
    JOIN serve.entity_identity i ON i.entity_id=a.entity_id
    WHERE a.alias_norm=ANY($1::text[]) AND i.kind IN ('hero','item','ability')
        AND (i.kind<>'hero' OR serve.hero_is_playable(i.entity_id))";

pub(super) struct EntityNames {
    pool: PgPool,
    runtime: tokio::runtime::Handle,
}

impl EntityNames {
    pub(super) fn new(pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self { pool, runtime }
    }
}

impl crate::tool_base::EntityNames for EntityNames {
    fn known(
        &self,
        phrases: &[String],
        context: &AuthorizedContext,
    ) -> Result<BTreeSet<String>, PortError> {
        let budget = context.remaining_time()?;
        let names = super::pg_tools::blocking(&self.runtime, async {
            tokio::time::timeout(
                budget,
                sqlx::query_scalar::<_, String>(ENTITY_NAMES)
                    .bind(phrases)
                    .fetch_all(&self.pool),
            )
            .await
        })
        .map_err(|_| PortError::Unavailable("Zeitgrenze für Spielnamen erreicht.".into()))?
        .map_err(|_| {
            PortError::Unavailable("Spielnamen können derzeit nicht gelesen werden.".into())
        })?;
        context.check_deadline()?;
        Ok(names.into_iter().collect())
    }
}
