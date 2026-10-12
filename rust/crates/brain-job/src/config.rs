use anyhow::{ensure, Context, Result};
use brain_contracts::postgres::{DatabaseAuth, Postgres};
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub postgres: Postgres,
    pub infisical_config: PathBuf,
    pub state_directory: PathBuf,
    pub lease_seconds: u32,
    pub git_timeout_seconds: u32,
    pub max_blob_bytes: usize,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        use std::io::Read;
        let mut data = Vec::new();
        std::fs::File::open(path)
            .context("config_read")?
            .take(32 * 1024 + 1)
            .read_to_end(&mut data)
            .context("config_read")?;
        ensure!(data.len() <= 32 * 1024, "config_size");
        let config: Self =
            serde_json::from_slice(&data).map_err(|_| anyhow::anyhow!("config_invalid"))?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        let pg = &self.postgres;
        ensure!(
            pg.socket_dir.is_absolute()
                && pg.port != 0
                && pg.username == "brain_ingest"
                && !pg.database.is_empty()
                && pg.database.len() <= 63
                && pg
                    .database
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_')
                && pg.auth == DatabaseAuth::Password
                && pg.password_env.as_deref().is_some_and(secret_name)
                && (1..=3).contains(&pg.max_connections),
            "config_postgres"
        );
        ensure!(
            self.infisical_config.is_absolute() && self.state_directory.is_absolute(),
            "config_path"
        );
        ensure!(
            (30..=900).contains(&self.lease_seconds)
                && (30..=3600).contains(&self.git_timeout_seconds)
                && (1024..=32 * 1024 * 1024).contains(&self.max_blob_bytes),
            "config_limits"
        );
        Ok(())
    }

    pub fn state(&self) -> Result<PathBuf> {
        let state = std::env::var_os("STATE_DIRECTORY")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.state_directory.clone());
        ensure!(
            state.is_absolute() && !state.to_string_lossy().contains(':'),
            "state_directory"
        );
        std::fs::create_dir_all(&state).context("state_create")?;
        ensure!(
            !std::fs::symlink_metadata(&state)
                .context("state_metadata")?
                .file_type()
                .is_symlink(),
            "state_symlink"
        );
        Ok(state)
    }

    pub async fn pool(&self) -> Result<PgPool> {
        reject_ambient()?;
        let name = self
            .postgres
            .password_env
            .as_deref()
            .context("config_password_reference")?;
        let values = deadlock_brain_core::pg::infisical_environment(&self.infisical_config)
            .await
            .map_err(|_| anyhow::anyhow!("infisical_load"))?;
        let password = values
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value)
            .context("infisical_password_missing")?;
        ensure!(
            !password.is_empty()
                && password.len() <= 4096
                && !password.chars().any(char::is_control),
            "infisical_password_invalid"
        );
        let options = PgConnectOptions::new_without_pgpass()
            .host(self.postgres.socket_dir.to_str().context("config_socket")?)
            .port(self.postgres.port)
            .username("brain_ingest")
            .database(&self.postgres.database)
            .password(password)
            .ssl_mode(PgSslMode::Disable)
            .application_name("brain-job")
            .options([("statement_timeout", "60000"), ("lock_timeout", "5000")])
            .disable_statement_logging();
        PgPoolOptions::new()
            .max_connections(self.postgres.max_connections)
            .acquire_timeout(Duration::from_secs(15))
            .connect_with(options)
            .await
            .map_err(|_| anyhow::anyhow!("postgres_connect"))
    }
}

fn secret_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

fn reject_ambient() -> Result<()> {
    ensure!(
        !std::env::vars_os().any(|(name, _)| {
            let name = name.to_string_lossy();
            name.starts_with("PG")
                || matches!(
                    name.as_ref(),
                    "DATABASE_URL" | "DEADLOCK_CENTRAL_DSN" | "INFISICAL_TOKEN_FILE"
                )
        }),
        "ambient_credentials_rejected"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_configuration_requires_ingest_and_bounded_inputs() {
        let template: serde_json::Value =
            serde_json::from_str(include_str!("../brain-job.example.json")).unwrap();
        let config: Config = serde_json::from_value(template.clone()).unwrap();
        config.validate().unwrap();
        for (field, value) in [
            ("lease_seconds", serde_json::json!(0)),
            ("max_blob_bytes", serde_json::json!(33554433)),
            ("state_directory", serde_json::json!("relative")),
        ] {
            let mut changed = template.clone();
            changed[field] = value;
            assert!(serde_json::from_value::<Config>(changed)
                .unwrap()
                .validate()
                .is_err());
        }
        for (field, value) in [
            ("username", serde_json::json!("brain_service")),
            ("auth", serde_json::json!("peer")),
            ("password_env", serde_json::json!("not-a-secret-reference")),
        ] {
            let mut changed = template.clone();
            changed["postgres"][field] = value;
            assert!(serde_json::from_value::<Config>(changed)
                .unwrap()
                .validate()
                .is_err());
        }
        let mut unknown = template;
        unknown["dsn"] = serde_json::json!("forbidden");
        assert!(serde_json::from_value::<Config>(unknown).is_err());
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), vec![b' '; 32 * 1024 + 1]).unwrap();
        assert!(Config::load(file.path()).is_err());
    }
}
