//! sqlx `PgPool`-Fundament fuer das zentrale Postgres-Brain.

use anyhow::{anyhow, Result};
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{path::Path, str::FromStr, time::Duration};

#[path = "pg_secrets.rs"]
mod secrets;

pub(crate) use secrets::Config as InfisicalConfig;

pub async fn pg_pool_read_only() -> Result<PgPool> {
    pg_pool_from_config(&crate::config::repo_root().join("config/bot.toml"), true).await
}

pub async fn infisical_environment(
    path: &Path,
) -> Result<Vec<(String, zeroize::Zeroizing<String>)>> {
    secrets::environment(path).await
}

pub async fn pg_pool_from_config(path: &Path, read_only: bool) -> Result<PgPool> {
    let dsn = secrets::database_dsn(path).await?;
    let mut options = PgConnectOptions::from_str(&dsn)
        .map_err(|_| anyhow!("Der Datenbankzugang aus Infisical ist ungültig."))?;
    if read_only {
        options = options.options([("default_transaction_read_only", "on")]);
    }
    PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| anyhow!("Verbindung zur zentralen Postgres fehlgeschlagen."))
}

/// Postgres-Schema mit den Brain-Tabellen.
pub const SCHEMA: &str = "brain";

/// Verwendet den vorhandenen Infisical-Zugang für schreibende Brain-Aufträge.
/// Zugang und Datenbankrechte entsprechen dem bisherigen zentralen Zugang.
pub async fn pg_pool() -> Result<PgPool> {
    pg_pool_from_config(&crate::config::repo_root().join("config/bot.toml"), false).await
}
