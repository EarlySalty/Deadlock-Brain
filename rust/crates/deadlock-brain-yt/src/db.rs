use std::path::PathBuf;

use anyhow::Result;
use sqlx::postgres::PgPool;

pub fn repo_root() -> PathBuf {
    deadlock_brain_core::db::repo_root()
}

pub fn default_feed_config_path() -> PathBuf {
    repo_root().join("config/youtube_feeds.json")
}

/// Baut den zentralen Postgres-Pool. Ersetzt das frühere SQLite-`open_db`;
/// nach dem PG-Cutover kommen alle Verbindungen aus [`deadlock_brain_core::pg`].
pub async fn pg_pool() -> Result<PgPool> {
    deadlock_brain_core::pg::pg_pool().await
}

pub fn now_epoch_seconds() -> i64 {
    deadlock_brain_core::db::now_epoch_seconds().unwrap_or(0)
}
