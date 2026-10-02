use super::{
    artifacts::Artifacts,
    config_writer::ConfigWriter,
    runtime_config::{read_bounded, RuntimeConfig},
};
use anyhow::{ensure, Result};
use brain_contracts::{maintenance::MaintenanceActivationProof, CorpusRelease};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Stdio, time::Duration};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    old_config: String,
    new_config: String,
    release_id: String,
    knowledge_version: String,
}

#[derive(Clone)]
pub struct ActivationPlan {
    path: PathBuf,
    unit: String,
    old_bytes: Vec<u8>,
    new_bytes: Vec<u8>,
    release_id: String,
    journal_ref: String,
    health_timeout_ms: u64,
}

impl ActivationPlan {
    pub fn prepare(
        config: &RuntimeConfig,
        artifacts: &Artifacts,
        release: &CorpusRelease,
        writer: &ConfigWriter,
    ) -> Result<Self> {
        writer.require_path(&config.serve_config)?;
        let old_bytes = writer.read()?;
        let old_config = brain_serve::Config::parse(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let mut value = brain_serve::bot_toml::document(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        value["brain"]["serve"]["release"] = toml::Value::try_from(
            serde_json::json!({"id": release.release_id, "knowledge_version": release.knowledge_version}),
        )?;
        let new_bytes = if old_config.release.id == release.release_id
            && old_config.release.knowledge_version == release.knowledge_version
        {
            old_bytes.clone()
        } else {
            toml::to_string_pretty(&value)?.into_bytes()
        };
        brain_serve::Config::parse(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let journal = Journal {
            old_config: artifacts.put(&old_bytes, "toml")?,
            new_config: artifacts.put(&new_bytes, "toml")?,
            release_id: release.release_id.clone(),
            knowledge_version: release.knowledge_version.clone(),
        };
        let journal_ref = artifacts.put(&serde_json::to_vec(&journal)?, "json")?;
        Self::load(config, artifacts, &journal_ref, &release.release_id)
    }

    pub fn load(
        config: &RuntimeConfig,
        artifacts: &Artifacts,
        reference: &str,
        release_id: &str,
    ) -> Result<Self> {
        let journal: Journal = serde_json::from_slice(&artifacts.read(reference)?)
            .map_err(|_| anyhow::anyhow!("activation_journal_schema"))?;
        ensure!(
            journal.release_id == release_id,
            "activation_journal_release"
        );
        let old_bytes = artifacts.read(&journal.old_config)?;
        let new_bytes = artifacts.read(&journal.new_config)?;
        brain_serve::Config::parse(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let new = brain_serve::Config::parse(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        ensure!(
            new.release.id == journal.release_id
                && new.release.knowledge_version == journal.knowledge_version,
            "activation_journal_pin"
        );
        let old = brain_serve::bot_toml::document(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        let mut proposed = brain_serve::bot_toml::document(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        proposed["brain"]["serve"]["release"] = old["brain"]["serve"]["release"].clone();
        ensure!(old == proposed, "activation_changed_unrelated_config");
        Ok(Self {
            path: config.serve_config.clone(),
            unit: config.serve_unit.clone(),
            old_bytes,
            new_bytes,
            release_id: release_id.to_owned(),
            journal_ref: reference.to_owned(),
            health_timeout_ms: config.health_timeout_ms,
        })
    }

    pub fn journal_ref(&self) -> &str {
        &self.journal_ref
    }
    pub fn needs_rebase(&self, writer: &ConfigWriter) -> Result<bool> {
        writer.require_path(&self.path)?;
        let current = writer.read()?;
        Ok(current != self.old_bytes && current != self.new_bytes)
    }

    async fn restart_and_health(&self, expected: &[u8]) -> Result<()> {
        let serve = brain_serve::Config::parse(expected)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        ensure!(serve.bind.ip().is_loopback(), "health_loopback");
        if self.old_bytes != self.new_bytes {
            let mut child = tokio::process::Command::new("/usr/bin/systemctl")
                .args(["--user", "restart", self.unit.as_str()])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .kill_on_drop(true)
                .spawn()?;
            let status =
                tokio::time::timeout(Duration::from_millis(self.health_timeout_ms), child.wait())
                    .await
                    .map_err(|_| anyhow::anyhow!("serve_restart_timeout"))??;
            ensure!(status.success(), "serve_restart_failed");
        }
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_millis(1000))
            .build()
            .map_err(|_| anyhow::anyhow!("health_client"))?;
        let deadline = tokio::time::Instant::now() + Duration::from_millis(self.health_timeout_ms);
        while tokio::time::Instant::now() < deadline {
            ensure!(
                read_bounded(&self.path, brain_serve::bot_toml::MAX_BYTES)? == expected,
                "serve_config_changed"
            );
            if let Ok(mut reply) = client
                .get(format!("http://{}/readyz", serve.bind))
                .send()
                .await
            {
                if reply.status().is_success() {
                    let mut bytes = Vec::new();
                    while let Ok(Some(chunk)) = reply.chunk().await {
                        bytes.extend_from_slice(&chunk);
                        ensure!(bytes.len() <= 4096, "health_body_size");
                    }
                    let parsed: serde_json::Value =
                        serde_json::from_slice(&bytes).unwrap_or_default();
                    if parsed["status"] == "ready"
                        && parsed["release_id"] == serve.release.id
                        && parsed["knowledge_version"] == serve.release.knowledge_version
                    {
                        return Ok(());
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        anyhow::bail!("serve_health_failed")
    }

    pub async fn activate(&self, writer: &ConfigWriter) -> Result<MaintenanceActivationProof> {
        writer.require_path(&self.path)?;
        writer.replace(&self.old_bytes, &self.new_bytes)?;
        self.restart_and_health(&self.new_bytes).await?;
        let verified_at_epoch = i64::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        )?;
        Ok(MaintenanceActivationProof {
            active_release_id: self.release_id.clone(),
            verified_at_epoch,
            evidence_ref: self.journal_ref.clone(),
        })
    }

    pub async fn rollback(&self, writer: &ConfigWriter) -> Result<()> {
        writer.require_path(&self.path)?;
        writer.replace(&self.new_bytes, &self.old_bytes)?;
        self.restart_and_health(&self.old_bytes).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, os::unix::fs::PermissionsExt};

    #[test]
    fn aktivierung_erhaelt_private_grants_und_alle_fremden_tabellen() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = directory.path().join("bot.toml");
        let mut root = brain_serve::bot_toml::document(include_bytes!(
            "../../../../../config/brain-serve.example.toml"
        ))
        .unwrap();
        root["brain"].as_table_mut().unwrap().insert("operator".into(), toml::Value::try_from(
            serde_json::json!({"socket":"/run/user/1000/brain-operator/operator.sock",
                "release":{"id":"fixture-internal-r1","knowledge_version":"fixture-internal-v1"},
                "credential":{"token_env":"BRAIN_SERVE_SECOND_BRAIN_TOKEN","actor_id":"second-brain",
                    "channel":"internal","scopes":["second_brain.internal"],"provider_egress":[],
                    "release":{"id":"fixture-internal-r1","knowledge_version":"fixture-internal-v1"}}
            })).unwrap());
        root["brain"].as_table_mut().unwrap().insert(
            "docs".into(),
            toml::Value::try_from(
                serde_json::json!({"endpoint":"http://127.0.0.1:8788", "timeout_ms":5000}),
            )
            .unwrap(),
        );
        let old = toml::to_string(&root).unwrap().into_bytes();
        std::fs::write(&path, &old).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let mut runtime: RuntimeConfig = brain_serve::bot_toml::section(
            include_bytes!("../../../../../ops/brain-maintenance/runtime.example.toml"),
            &["brain", "maintenance", "runtime"],
        )
        .unwrap();
        runtime.maintenance_config = path.clone();
        runtime.serve_config = path.clone();
        runtime.infisical_config = path.clone();
        runtime.artifact_dir = directory.path().join("artifacts");
        let artifacts = Artifacts::open(&runtime.artifact_dir).unwrap();
        let writer = ConfigWriter::lock(&path).unwrap();
        let release = CorpusRelease {
            release_id: "fixture-public-r2".into(),
            knowledge_version: "fixture-public-v2".into(),
            patch: "fixture".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::new(),
        };
        let plan = ActivationPlan::prepare(&runtime, &artifacts, &release, &writer).unwrap();
        let mut new_root = brain_serve::bot_toml::document(&plan.new_bytes).unwrap();
        assert_eq!(new_root["brain"]["operator"], root["brain"]["operator"]);
        assert_eq!(new_root["brain"]["docs"], root["brain"]["docs"]);
        assert_eq!(
            new_root["brain"]["serve"]["credentials"],
            root["brain"]["serve"]["credentials"]
        );
        new_root["brain"]["serve"]["release"] = root["brain"]["serve"]["release"].clone();
        assert_eq!(new_root, root);
        // Ein ansonsten gültiger Journalvorschlag darf keinen fremden Consumer ändern.
        let mut changed = brain_serve::bot_toml::document(&plan.new_bytes).unwrap();
        changed["brain"]["docs"]["timeout_ms"] = toml::Value::Integer(6000);
        let journal = Journal {
            old_config: artifacts.put(&old, "toml").unwrap(),
            new_config: artifacts
                .put(toml::to_string(&changed).unwrap().as_bytes(), "toml")
                .unwrap(),
            release_id: release.release_id.clone(),
            knowledge_version: release.knowledge_version.clone(),
        };
        let reference = artifacts
            .put(&serde_json::to_vec(&journal).unwrap(), "json")
            .unwrap();
        assert!(
            ActivationPlan::load(&runtime, &artifacts, &reference, &release.release_id).is_err()
        );
        assert_eq!(writer.read().unwrap(), old);
    }
}
