//! sqlx-Pools für das zentrale Postgres-Brain.

use anyhow::{anyhow, Result};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::{path::Path, str::FromStr, time::Duration};
use zeroize::Zeroizing;

#[path = "pg_secrets.rs"]
mod secrets;

pub const DSN_ENV: &str = "DEADLOCK_CENTRAL_DSN";
pub const SCHEMA: &str = "brain";

/// Nutzt ein bereits vom autorisierten Infisical-Loader übergebenes Secret.
/// Ohne übergebenes Secret bleibt der native Credential-Transport maßgeblich.
/// Jeder neue Pool erzwingt Read-only auf allen seinen Verbindungen.
pub async fn pg_pool_read_only() -> Result<PgPool> {
    match std::env::var(DSN_ENV) {
        Ok(dsn) => connect_pool(&Zeroizing::new(dsn), true).await,
        Err(std::env::VarError::NotPresent) => {
            pg_pool_from_config(
                &crate::config::repo_root().join("config/infisical.json"),
                true,
            )
            .await
        }
        Err(_) => Err(anyhow!("Der übergebene Datenbankzugang ist ungültig.")),
    }
}

pub async fn infisical_environment(path: &Path) -> Result<Vec<(String, Zeroizing<String>)>> {
    secrets::environment(path).await
}

pub async fn pg_pool_from_config(path: &Path, read_only: bool) -> Result<PgPool> {
    let dsn = secrets::database_dsn(path).await?;
    connect_pool(&dsn, read_only).await
}

fn connect_options(dsn: &str, read_only: bool) -> Result<PgConnectOptions> {
    let mut options = PgConnectOptions::from_str(dsn)
        .map_err(|_| anyhow!("Der Datenbankzugang aus Infisical ist ungültig."))?;
    if read_only {
        // Als letzte Startoption setzen: auch ein gegenteiliger DSN-Parameter
        // darf den ausdrücklich lesenden Aufruf nicht schreibend öffnen.
        options = options.options([("default_transaction_read_only", "on")]);
    }
    Ok(options)
}

async fn connect_pool(dsn: &str, read_only: bool) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(connect_options(dsn, read_only)?)
        .await
        .map_err(|_| anyhow!("Verbindung zur zentralen Postgres fehlgeschlagen."))
}

pub async fn pg_pool() -> Result<PgPool> {
    let dsn = Zeroizing::new(
        std::env::var(DSN_ENV)
            .map_err(|_| anyhow!("{DSN_ENV} ist nicht gesetzt; DSN wird nicht ausgegeben."))?,
    );
    connect_pool(&dsn, false).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_is_a_connection_startup_option() {
        let options = connect_options("postgresql:///brain_unit", true).unwrap();
        assert_eq!(
            options.get_options(),
            Some("-c default_transaction_read_only=on")
        );
    }

    #[test]
    fn read_only_cannot_be_downgraded_by_dsn_options() {
        let options = connect_options(
            "postgresql:///brain_unit?options=-c%20default_transaction_read_only%3Doff",
            true,
        )
        .unwrap();
        assert!(options
            .get_options()
            .unwrap()
            .ends_with("-c default_transaction_read_only=on"));
    }

    #[test]
    fn write_pool_does_not_invent_a_read_only_setting() {
        let options = connect_options("postgresql:///brain_unit", false).unwrap();
        assert_eq!(options.get_options(), None);
    }

    #[test]
    fn malformed_dsn_is_never_reflected_in_the_error() {
        let marker = "not-a-valid-dsn-private-marker";
        let error = connect_options(marker, true).unwrap_err().to_string();
        assert!(!error.contains(marker));
        assert_eq!(error, "Der Datenbankzugang aus Infisical ist ungültig.");
    }
}
