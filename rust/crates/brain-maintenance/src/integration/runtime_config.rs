use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfig {
    #[serde(skip)]
    pub loaded_from: Option<PathBuf>,
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
    #[serde(default)]
    pub entity_profile_corpus_root: Option<PathBuf>,
    #[serde(default)]
    pub entity_profile_sources: Vec<EntityProfileSource>,
    #[serde(default)]
    pub entity_profile_patch_raw_dir: Option<PathBuf>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityProfileSource {
    pub repository_id: String,
    pub paths: std::collections::BTreeSet<String>,
    pub extraction: dbrain_sources::game_files::GameFileOptions,
    pub import_policy: PathBuf,
    pub canonical_raw_dir: Option<PathBuf>,
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
    let file = File::open(path).map_err(|_| anyhow::anyhow!("config_read"))?;
    ensure!(file.metadata()?.is_file(), "config_file");
    let mut bytes = Vec::new();
    file.take(maximum as u64 + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= maximum, "config_size");
    Ok(bytes)
}

impl RuntimeConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let mut config: Self = serde_json::from_slice(&read_bounded(path, 65536)?)
            .map_err(|_| anyhow::anyhow!("runtime_config_schema"))?;
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
        if let Some(root) = &config.entity_profile_corpus_root {
            ensure!(root.is_absolute(), "entity_profile_corpus_root");
        }
        if let Some(raw_dir) = &config.entity_profile_patch_raw_dir {
            ensure!(
                raw_dir.is_absolute() && config.entity_profile_corpus_root.is_some(),
                "entity_profile_patch_raw_dir"
            );
        }
        ensure!(
            config.entity_profile_sources.len() <= 32,
            "entity_profile_source_limit"
        );
        let mut identities = std::collections::BTreeSet::new();
        for source in &config.entity_profile_sources {
            ensure!(
                !source.repository_id.is_empty()
                    && config.entity_profile_corpus_root.is_some()
                    && !source.extraction.source_id.is_empty()
                    && identities.insert(&source.extraction.source_id)
                    && source.extraction.build_id.is_none()
                    && source.extraction.manifest_id.is_none()
                    && !source.paths.is_empty()
                    && source.paths.len() <= 64
                    && source.import_policy.is_absolute()
                    && source
                        .canonical_raw_dir
                        .as_ref()
                        .is_none_or(|path| path.is_absolute()),
                "entity_profile_source"
            );
            for path in &source.paths {
                crate::config::safe_relative(path)?;
            }
        }
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
        config.loaded_from = Some(path.to_owned());
        Ok(config)
    }
}
