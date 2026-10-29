use anyhow::{ensure, Context};
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub socket: PathBuf,
    pub port: u16,
    pub database: String,
    pub user: String,
    #[serde(default)]
    pub source_cache: Option<PathBuf>,
    #[serde(default)]
    auth: Auth,
    #[serde(default)]
    infisical_config: Option<PathBuf>,
}

#[derive(Default, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Auth {
    #[default]
    Peer,
    Infisical,
}

fn path(value: &Path) -> bool {
    value.is_absolute()
        && value
            .to_str()
            .is_some_and(|s| !s.chars().any(char::is_control))
}

impl Config {
    pub fn load(file: &Path) -> anyhow::Result<Self> {
        let mut bytes = Vec::new();
        std::fs::File::open(file)?
            .take(32 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 32 * 1024, "Konfiguration ist zu groß");
        let config: Self = serde_json::from_slice(&bytes)?;
        ensure!(
            path(&config.socket)
                && config.port != 0
                && [&config.database, &config.user]
                    .iter()
                    .all(|s| !s.is_empty()
                        && s.len() <= 63
                        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
                && config.source_cache.as_deref().is_none_or(path)
                && match config.auth {
                    Auth::Peer => config.infisical_config.is_none(),
                    Auth::Infisical =>
                        config.user == "brain_ingest"
                            && config.infisical_config.as_deref().is_some_and(path),
                },
            "Ungültige Datenbank- oder Quellenkonfiguration"
        );
        Ok(config)
    }

    pub async fn secret(
        &self,
        name: &str,
    ) -> anyhow::Result<impl std::ops::Deref<Target = String>> {
        ensure!(
            self.auth == Auth::Infisical && std::env::var_os("INFISICAL_TOKEN_FILE").is_none(),
            "Geheimnisse nur über Infisical"
        );
        let values = deadlock_brain_core::pg::infisical_environment(
            self.infisical_config
                .as_deref()
                .context("Infisical-Konfiguration fehlt")?,
        )
        .await?;
        let value = values
            .into_iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value)
            .with_context(|| format!("{name} fehlt in Infisical"))?;
        ensure!(
            !value.is_empty() && value.len() <= 4096 && !value.chars().any(char::is_control),
            "Ungültiges Geheimnis {name}"
        );
        Ok(value)
    }

    pub async fn pool(&self) -> anyhow::Result<PgPool> {
        ensure!(
            std::env::var_os("PGOPTIONS").is_none(),
            "PGOPTIONS ist unzulässig"
        );
        let values = match self.auth {
            Auth::Peer => Vec::new(),
            Auth::Infisical => {
                ensure!(
                    std::env::var_os("INFISICAL_TOKEN_FILE").is_none(),
                    "Token-Dateien sind unzulässig"
                );
                deadlock_brain_core::pg::infisical_environment(
                    self.infisical_config
                        .as_deref()
                        .context("Infisical-Konfiguration fehlt")?,
                )
                .await?
            }
        };
        let password = match self.auth {
            Auth::Peer => "",
            Auth::Infisical => {
                let password = values
                    .iter()
                    .find(|(name, _)| name == "BRAIN_PG_INGEST_PASSWORD")
                    .map(|(_, value)| value.as_str())
                    .context("Ingest-Zugang fehlt in Infisical")?;
                ensure!(
                    !password.is_empty()
                        && password.len() <= 4096
                        && !password.chars().any(char::is_control),
                    "Ungültiger Ingest-Zugang"
                );
                password
            }
        };
        let options = PgConnectOptions::new_without_pgpass()
            .host(self.socket.to_str().context("Ungültiger Socket-Pfad")?)
            .port(self.port)
            .database(&self.database)
            .username(&self.user)
            .password(password)
            .ssl_mode(PgSslMode::Disable)
            .application_name("brain-job")
            .disable_statement_logging();
        PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(120))
            .connect_with(options)
            .await
            .map_err(|_| anyhow::anyhow!("Datenbankverbindung fehlgeschlagen"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_and_ingest_infisical_configuration_are_explicit_and_bounded() {
        let directory =
            std::env::temp_dir().join(format!("brain-core2-config-{}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let file = directory.join("config.json");
        let mut value = serde_json::json!({"socket":"/run/deadlock-brain-postgresql", "port":5446,
            "database":"brain", "user":"brain_ingest", "source_cache":"/var/cache/brain"});
        std::fs::write(&file, value.to_string()).unwrap();
        assert!(matches!(Config::load(&file).unwrap().auth, Auth::Peer));
        value["auth"] = "infisical".into();
        value["infisical_config"] = "/etc/deadlock-brain/infisical.json".into();
        std::fs::write(&file, value.to_string()).unwrap();
        assert!(matches!(Config::load(&file).unwrap().auth, Auth::Infisical));
        for (key, invalid) in [
            ("user", "brain_service"),
            ("infisical_config", "relative.json"),
            ("source_cache", "relative-cache"),
            ("socket", "/run/invalid\n"),
            ("password", "unconfigured"),
        ] {
            let mut bad = value.clone();
            bad[key] = invalid.into();
            std::fs::write(&file, bad.to_string()).unwrap();
            assert!(Config::load(&file).is_err());
        }
        std::fs::write(&file, vec![b' '; 32 * 1024 + 1]).unwrap();
        assert!(Config::load(&file).is_err());
        std::fs::remove_file(file).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }
}
