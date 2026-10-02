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
        let mut value: serde_json::Value = serde_json::from_slice(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        value["release"] = serde_json::json!({"id": release.release_id, "knowledge_version": release.knowledge_version});
        let new_bytes = if old_config.release.id == release.release_id
            && old_config.release.knowledge_version == release.knowledge_version
        {
            old_bytes.clone()
        } else {
            serde_json::to_vec_pretty(&value)?
        };
        brain_serve::Config::parse(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let journal = Journal {
            old_config: artifacts.put(&old_bytes, "json")?,
            new_config: artifacts.put(&new_bytes, "json")?,
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
        let old: serde_json::Value = serde_json::from_slice(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        let mut proposed: serde_json::Value = serde_json::from_slice(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        proposed["release"] = old["release"].clone();
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
                read_bounded(&self.path, 65536)? == expected,
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
