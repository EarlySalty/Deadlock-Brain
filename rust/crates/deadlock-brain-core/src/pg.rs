//! sqlx `PgPool`-Fundament fuer das zentrale Postgres-Brain.

use anyhow::{anyhow, Result};
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{path::Path, str::FromStr, time::Duration};

#[path = "pg_secrets.rs"]
mod secrets;

pub async fn pg_pool_read_only() -> Result<PgPool> {
    pg_pool_from_config(
        &crate::config::repo_root().join("config/infisical.json"),
        true,
    )
    .await
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

/// Env-Variable mit dem zentralen Postgres-DSN. Der Wert wird nie geloggt oder
/// ausgegeben (Secret-Hygiene).
pub const DSN_ENV: &str = "DEADLOCK_CENTRAL_DSN";

/// Postgres-Schema mit den Brain-Tabellen.
pub const SCHEMA: &str = "brain";

/// Baut einen kleinen async Verbindungspool gegen die zentrale Postgres.
///
/// Das DSN kommt aus [`DSN_ENV`]. Weder das DSN noch die konkreten
/// Verbindungsdetails werden im Fehlerfall ausgegeben.
pub async fn pg_pool() -> Result<PgPool> {
    let dsn = std::env::var(DSN_ENV)
        .map_err(|_| anyhow!("{DSN_ENV} ist nicht gesetzt; DSN wird nicht ausgegeben."))?;
    PgPoolOptions::new()
        .max_connections(4)
        .connect(&dsn)
        .await
        .map_err(|_| {
            anyhow!("Verbindung zur zentralen Postgres fehlgeschlagen (DSN aus {DSN_ENV}).")
        })
}
