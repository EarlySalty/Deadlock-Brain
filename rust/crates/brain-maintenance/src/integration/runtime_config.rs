use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::Read,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfig {
    pub maintenance_config: PathBuf,
    pub postgres: brain_serve::config::Postgres,
    pub serve_config: PathBuf,
    pub serve_unit: String,
    pub artifact_dir: PathBuf,
    pub status_file: PathBuf,
    pub lease_ttl_ms: u64,
    pub max_jobs_per_tick: usize,
    pub health_timeout_ms: u64,
    pub infisical_config: PathBuf,
    pub jev_secret: String,
    pub retry_delay_ms: u64,
    #[serde(default)]
    pub local_imports: Vec<LocalImport>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalImport {
    pub path: PathBuf,
    pub targets: std::collections::BTreeSet<String>,
    pub sha256: String,
    pub reviewer_id: String,
    pub review_ref: String,
}

pub fn read_bounded(path: &Path, maximum: usize) -> Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NONBLOCK | nix::libc::O_NOFOLLOW | nix::libc::O_CLOEXEC)
        .open(path)
        .map_err(|_| anyhow::anyhow!("config_read"))?;
    ensure!(file.metadata()?.is_file(), "config_file");
    let mut bytes = Vec::new();
    file.take(maximum as u64 + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= maximum, "config_size");
    Ok(bytes)
}

impl RuntimeConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = read_bounded(path, brain_serve::bot_toml::MAX_BYTES)?;
        let config: Self =
            brain_serve::bot_toml::section(&bytes, &["brain", "maintenance", "runtime"])
                .map_err(|_| anyhow::anyhow!("runtime_config_schema"))?;
        ensure!(
            path.is_absolute()
                && config.maintenance_config == path
                && config.serve_config == path
                && config.infisical_config == path,
            "shared_bot_toml_required"
        );
        for path in [
            &config.maintenance_config,
            &config.serve_config,
            &config.artifact_dir,
            &config.status_file,
            &config.postgres.socket_dir,
            &config.infisical_config,
        ] {
            ensure!(path.is_absolute(), "absolute_config_paths");
        }
        ensure!(config.serve_unit == "brain-serve.service", "serve_unit");
        ensure!((1000..=1800000).contains(&config.lease_ttl_ms), "lease_ttl");
        ensure!((1..=16).contains(&config.max_jobs_per_tick), "job_limit");
        ensure!(
            (1000..=60000).contains(&config.health_timeout_ms),
            "health_timeout"
        );
        ensure!(
            config.lease_ttl_ms >= config.health_timeout_ms * 3 + 10000,
            "activation_lease_budget"
        );
        ensure!(config.jev_secret == "JEV", "jev_secret_name");
        ensure!(
            (1000..=86_400_000).contains(&config.retry_delay_ms),
            "retry_delay"
        );
        ensure!(config.local_imports.len() <= 32, "local_import_limit");
        for import in &config.local_imports {
            ensure!(
                !import.targets.is_empty() && import.targets.len() <= 64,
                "local_import_targets"
            );
            for target in &import.targets {
                crate::config::safe_doc_target(target)?;
            }
            ensure!(
                import.path.is_absolute()
                    && import.sha256.len() == 64
                    && import
                        .sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    && !import.reviewer_id.is_empty()
                    && !import.review_ref.is_empty(),
                "local_import_review"
            );
        }
        ensure!(
            config.postgres.username == "brain_ingest"
                || config.postgres.username == "brain_migrate",
            "maintenance_database_role"
        );
        ensure!(
            config.postgres.socket_dir.is_absolute()
                && config.postgres.port > 0
                && (2..=16).contains(&config.postgres.max_connections),
            "postgres_endpoint"
        );
        ensure!(
            !config.postgres.database.is_empty()
                && config.postgres.database.len() <= 63
                && config
                    .postgres
                    .database
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_'),
            "postgres_database"
        );
        ensure!(
            config.postgres.username != "brain_migrate"
                || config.postgres.auth == brain_serve::config::DatabaseAuth::Peer,
            "migration_peer_required"
        );
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runtime_bindet_genau_die_gemeinsame_bot_toml_ohne_json_fallback() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("bot.toml");
        let mut root = brain_serve::bot_toml::document(include_bytes!(
            "../../../../../ops/brain-maintenance/runtime.example.toml"
        ))
        .unwrap();
        for field in ["maintenance_config", "serve_config", "infisical_config"] {
            root["brain"]["maintenance"]["runtime"][field] =
                toml::Value::String(path.to_str().unwrap().into());
        }
        std::fs::write(&path, toml::to_string(&root).unwrap()).unwrap();
        assert!(RuntimeConfig::load(&path).is_ok());
        for field in ["maintenance_config", "serve_config", "infisical_config"] {
            let mut invalid = root.clone();
            invalid["brain"]["maintenance"]["runtime"][field] =
                toml::Value::String("/fremde/bot.toml".into());
            std::fs::write(&path, toml::to_string(&invalid).unwrap()).unwrap();
            assert!(RuntimeConfig::load(&path).is_err());
        }
        std::fs::write(
            &path,
            serde_json::to_vec(&root["brain"]["maintenance"]["runtime"]).unwrap(),
        )
        .unwrap();
        assert!(RuntimeConfig::load(&path).is_err());
    }
}
