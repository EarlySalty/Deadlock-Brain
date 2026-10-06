//! PostgreSQL state and a cross-process, transaction-scoped refresh lock.
//! Uses the existing Brain credential bootstrap. Never opens SQLite or writes TOML.

use sqlx::{types::Json, PgPool, Postgres, Transaction};
use tokio::runtime::Runtime;

use super::{Policy, Result, SelectionError, State, Store};

pub(super) struct PgStore {
    transaction: Option<Transaction<'static, Postgres>>,
    pool: PgPool,
    runtime: Runtime,
}

impl PgStore {
    pub fn new() -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()
            .map_err(|_| SelectionError::StateUnavailable)?;
        let pool = runtime.block_on(crate::pg::pg_pool()).map_err(|_| SelectionError::StateUnavailable)?;
        Ok(Self { transaction: None, pool, runtime })
    }
}

impl Store for PgStore {
    fn load(&mut self, policy: &Policy) -> Result<Option<State>> {
        let query = sqlx::query_scalar::<_, Json<State>>(
            "SELECT state FROM brain.model_selection_state WHERE policy_fingerprint = $1"
        ).bind(&policy.fingerprint);
        let row = if let Some(transaction) = &mut self.transaction {
            self.runtime.block_on(query.fetch_optional(&mut **transaction))
        } else {
            self.runtime.block_on(query.fetch_optional(&self.pool))
        }.map_err(|_| SelectionError::StateUnavailable)?;
        Ok(row.map(|Json(state)| state))
    }

    fn try_lock(&mut self, policy: &Policy) -> Result<bool> {
        if self.transaction.is_some() { return Ok(true); }
        let id = u64::from_str_radix(&policy.fingerprint[..16], 16)
            .map_err(|_| SelectionError::StateUnavailable)? as i64;
        let mut transaction = self.runtime.block_on(self.pool.begin())
            .map_err(|_| SelectionError::StateUnavailable)?;
        let acquired: bool = self.runtime.block_on(
            sqlx::query_scalar("SELECT pg_try_advisory_xact_lock($1)")
                .bind(id).fetch_one(&mut *transaction)
        ).map_err(|_| SelectionError::StateUnavailable)?;
        if acquired {
            self.transaction = Some(transaction);
        } else {
            self.runtime.block_on(transaction.rollback()).map_err(|_| SelectionError::StateUnavailable)?;
        }
        Ok(acquired)
    }

    fn save(&mut self, policy: &Policy, state: &State) -> Result<()> {
        let mut transaction = self.transaction.take().ok_or(SelectionError::StateUnavailable)?;
        self.runtime.block_on(async {
            sqlx::query(
                "INSERT INTO brain.model_selection_state (policy_fingerprint, state) VALUES ($1, $2) \
                 ON CONFLICT (policy_fingerprint) DO UPDATE SET state = EXCLUDED.state, updated_at = clock_timestamp()"
            ).bind(&policy.fingerprint).bind(Json(state)).execute(&mut *transaction).await?;
            transaction.commit().await
        }).map_err(|_| SelectionError::StateUnavailable)
    }
}

impl Drop for PgStore {
    fn drop(&mut self) {
        if let Some(transaction) = self.transaction.take() {
            let _ = self.runtime.block_on(transaction.rollback());
        }
        self.runtime.block_on(self.pool.close());
    }
}
