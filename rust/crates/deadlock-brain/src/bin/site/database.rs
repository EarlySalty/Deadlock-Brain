use anyhow::{ensure, Context, Result};
use brain_contracts::postgres::{DatabaseAuth, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;
use std::{path::PathBuf, time::Duration};

#[derive(Debug, clap::Args)]
pub struct ConnectionArgs {
    #[arg(
        long,
        required_unless_present = "pg_config",
        conflicts_with = "pg_config"
    )]
    pub infisical_config: Option<PathBuf>,
    #[arg(long, required_unless_present = "infisical_config")]
    pub pg_config: Option<PathBuf>,
}

fn validate(config: &Postgres, role: &str) -> Result<()> {
    ensure!(
        config.socket_dir.is_absolute()
            && config.socket_dir.to_str().is_some()
            && config.port > 0
            && config.username == role
            && config.auth == DatabaseAuth::Peer
            && config.password_env.is_none()
            && (1..=4).contains(&config.max_connections)
            && !config.database.trim().is_empty()
            && config.database.len() <= 128
            && !config.database.chars().any(char::is_control),
        "Die Site benötigt einen begrenzten lokalen Peer-Zugang."
    );
    Ok(())
}

impl ConnectionArgs {
    pub async fn pool(&self, role: &str) -> Result<PgPool> {
        match (&self.pg_config, &self.infisical_config) {
            (Some(path), None) => {
                ensure!(
                    path.is_absolute() && std::env::var_os("PGOPTIONS").is_none(),
                    "Die lokale Datenbankkonfiguration ist ungültig."
                );
                let config: Postgres = serde_json::from_slice(&std::fs::read(path)?)
                    .context("Die lokale Datenbankkonfiguration ist ungültig.")?;
                validate(&config, role)?;
                let options = PgConnectOptions::new_without_pgpass()
                    .host(config.socket_dir.to_str().unwrap())
                    .port(config.port)
                    .database(&config.database)
                    .username(&config.username)
                    .password("")
                    .ssl_mode(PgSslMode::Disable)
                    .application_name("brain-site");
                Ok(PgPoolOptions::new()
                    .max_connections(config.max_connections)
                    .acquire_timeout(Duration::from_secs(15))
                    .connect_with(options)
                    .await
                    .context("Der lokale Datenbankzugang ist nicht verfügbar.")?)
            }
            (None, Some(path)) => deadlock_brain_core::pg::pg_pool_from_config(path, false).await,
            _ => anyhow::bail!("Ein eindeutiger Datenbankzugang fehlt."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_configuration_is_local_bounded_and_role_specific() {
        let mut config: Postgres = serde_json::from_value(serde_json::json!({
            "socket_dir":"/run/deadlock-brain-postgresql", "port":5446,
            "username":"brain_site", "database":"brain", "auth":"peer",
            "password_env":null, "max_connections":4
        }))
        .unwrap();
        assert!(validate(&config, "brain_site").is_ok());
        assert!(validate(&config, "brain_migrate").is_err());
        config.max_connections = 5;
        assert!(validate(&config, "brain_site").is_err());
        config.max_connections = 4;
        config.socket_dir = "remote.example".into();
        assert!(validate(&config, "brain_site").is_err());
        config.socket_dir = "/run/postgres".into();
        config.auth = DatabaseAuth::Password;
        assert!(validate(&config, "brain_site").is_err());
    }
}
