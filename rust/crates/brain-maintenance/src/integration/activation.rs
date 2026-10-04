use super::{
    artifacts::Artifacts,
    config_writer::ConfigWriter,
    runtime_config::{read_bounded, RuntimeConfig},
};
use anyhow::{ensure, Result};
use brain_contracts::{maintenance::MaintenanceActivationProof, CorpusRelease};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Stdio, sync::LazyLock, time::Duration};

#[derive(Default)]
struct RestartSchedule {
    next: Option<tokio::time::Instant>,
}

impl RestartSchedule {
    fn record(&mut self, now: tokio::time::Instant) {
        self.next = Some(now + Duration::from_secs(21));
    }

    async fn wait(&self) {
        if let Some(next) = self.next {
            tokio::time::sleep_until(next).await;
        }
    }
}

static RESTART_SCHEDULE: LazyLock<tokio::sync::Mutex<RestartSchedule>> =
    LazyLock::new(|| tokio::sync::Mutex::new(RestartSchedule::default()));

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationTarget {
    #[default]
    Standard,
    SecondBrainInternal,
}

impl ActivationTarget {
    fn internal_index(self, config: &brain_serve::Config) -> Result<usize> {
        let matches: Vec<_> = config
            .credentials
            .iter()
            .enumerate()
            .filter(|(_, grant)| grant.actor_id == "second-brain" && grant.channel == "internal")
            .map(|(index, _)| index)
            .collect();
        ensure!(matches.len() == 1, "activation_internal_grant");
        Ok(matches[0])
    }

    pub fn release(self, config: &brain_serve::Config) -> Result<&brain_serve::config::Release> {
        match self {
            Self::Standard => Ok(&config.release),
            Self::SecondBrainInternal => {
                let grant = &config.credentials[self.internal_index(config)?];
                let release = grant
                    .release
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("activation_internal_pin"))?;
                let operator = config
                    .internal_operator
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("activation_internal_operator"))?;
                ensure!(
                    release.id == operator.release.id
                        && release.knowledge_version == operator.release.knowledge_version,
                    "activation_internal_pin_mismatch"
                );
                Ok(release)
            }
        }
    }

    pub fn replace_pin(
        self,
        value: &mut serde_json::Value,
        config: &brain_serve::Config,
        pin: serde_json::Value,
    ) -> Result<()> {
        match self {
            Self::Standard => {
                let matches_current = |release: &brain_serve::config::Release| {
                    release.id == config.release.id
                        && release.knowledge_version == config.release.knowledge_version
                };
                let mut public_docs_bound = false;
                for (index, grant) in config.credentials.iter().enumerate() {
                    if grant.actor_id == "docs-client"
                        && grant.channel == "docs"
                        && grant.scopes == std::collections::BTreeSet::from(["bot.public".into()])
                        && grant.release.as_ref().is_some_and(matches_current)
                    {
                        value["credentials"][index]["release"] = pin.clone();
                        public_docs_bound = true;
                    }
                }
                if public_docs_bound
                    && config
                        .internal_operator
                        .as_ref()
                        .is_some_and(|operator| matches_current(&operator.release))
                {
                    let index = self.internal_index(config)?;
                    if config.credentials[index]
                        .release
                        .as_ref()
                        .is_some_and(matches_current)
                    {
                        value["credentials"][index]["release"] = pin.clone();
                        value["internal_operator"]["release"] = pin.clone();
                    }
                }
                value["release"] = pin;
            }
            Self::SecondBrainInternal => {
                let index = self.internal_index(config)?;
                value["credentials"][index]["release"] = pin.clone();
                value["internal_operator"]["release"] = pin;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn neustarts_einschliesslich_rollback_respektieren_drei_starts_pro_minute() {
        let start = tokio::time::Instant::now();
        let mut schedule = RestartSchedule::default();
        let mut attempts = Vec::new();
        for requested in [0, 1, 2, 3, 4, 5] {
            let now = start + Duration::from_secs(requested);
            let actual = schedule.next.map_or(now, |next| next.max(now));
            attempts.push(actual);
            schedule.record(actual);
            assert!(
                attempts
                    .iter()
                    .filter(|attempt| actual.duration_since(**attempt) < Duration::from_secs(60))
                    .count()
                    <= 3
            );
        }
        assert_eq!(attempts[3].duration_since(start), Duration::from_secs(63));
        assert_eq!(
            attempts[4].duration_since(attempts[3]),
            Duration::from_secs(21)
        );
    }

    #[test]
    fn public_cutover_preserves_caller_scopes_and_moves_shared_pins_together() {
        let mut value: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../../../config/brain-serve.example.json"
        ))
        .unwrap();
        let old = value["release"].clone();
        value["credentials"] = json!([
            {"token_env":"TWITCH_INTERNAL_API_TOKEN","actor_id":"twitch-bot","channel":"twitch","scopes":["bot.public"],"provider_egress":["public"]},
            {"token_env":"BRAIN_SERVE_DOCS_PUBLIC_TOKEN","actor_id":"docs-client","channel":"docs","scopes":["bot.public"],"provider_egress":["public"],"release":old},
            {"token_env":"BRAIN_SERVE_SECOND_BRAIN_TOKEN","actor_id":"second-brain","channel":"internal","scopes":["second_brain.internal"],"provider_egress":[],"release":old}
        ]);
        value["internal_operator"] = json!({"socket":"/tmp/brain-operator.sock","release":old});
        let config = brain_serve::Config::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
        let next = json!({"id":"maintenance-next","knowledge_version":"docs-next"});
        ActivationTarget::Standard
            .replace_pin(&mut value, &config, next.clone())
            .unwrap();
        assert_eq!(value["release"], next);
        assert_eq!(value["credentials"][1]["release"], next);
        assert_eq!(value["credentials"][2]["release"], next);
        assert_eq!(value["internal_operator"]["release"], next);
        assert_eq!(
            value["credentials"][2]["scopes"],
            json!(["second_brain.internal"])
        );
        assert_eq!(value["credentials"][2]["provider_egress"], json!([]));
        assert_eq!(
            value["internal_operator"]["socket"],
            "/tmp/brain-operator.sock"
        );
        brain_serve::Config::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
        let separate = json!({"id":"internal-separate","knowledge_version":"internal-v1"});
        value["credentials"][2]["release"] = separate.clone();
        value["internal_operator"]["release"] = separate.clone();
        let config = brain_serve::Config::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
        let later = json!({"id":"maintenance-later","knowledge_version":"docs-later"});
        ActivationTarget::Standard
            .replace_pin(&mut value, &config, later.clone())
            .unwrap();
        assert_eq!(value["release"], later);
        assert_eq!(value["credentials"][1]["release"], later);
        assert_eq!(value["credentials"][2]["release"], separate);
        assert_eq!(value["internal_operator"]["release"], separate);
        brain_serve::Config::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    old_config: String,
    new_config: String,
    release_id: String,
    knowledge_version: String,
    #[serde(default)]
    target: ActivationTarget,
    #[serde(default)]
    old_bindings_sha256: Option<String>,
    #[serde(default)]
    new_bindings_sha256: Option<String>,
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
    target: ActivationTarget,
}

impl ActivationPlan {
    pub fn prepare(
        config: &RuntimeConfig,
        artifacts: &Artifacts,
        release: &CorpusRelease,
        writer: &ConfigWriter,
    ) -> Result<Self> {
        Self::prepare_target(
            config,
            artifacts,
            release,
            writer,
            ActivationTarget::Standard,
        )
    }

    pub fn prepare_target(
        config: &RuntimeConfig,
        artifacts: &Artifacts,
        release: &CorpusRelease,
        writer: &ConfigWriter,
        target: ActivationTarget,
    ) -> Result<Self> {
        writer.require_path(&config.serve_config)?;
        let old_bytes = writer.read()?;
        let old_config = brain_serve::Config::parse(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let old_pin = target.release(&old_config)?;
        let mut value: serde_json::Value = serde_json::from_slice(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        target.replace_pin(&mut value, &old_config, serde_json::json!({"id": release.release_id, "knowledge_version": release.knowledge_version}))?;
        let new_bytes = if old_pin.id == release.release_id
            && old_pin.knowledge_version == release.knowledge_version
        {
            old_bytes.clone()
        } else {
            serde_json::to_vec_pretty(&value)?
        };
        let new_config = brain_serve::Config::parse(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let journal = Journal {
            old_config: artifacts.put(&old_bytes, "json")?,
            new_config: artifacts.put(&new_bytes, "json")?,
            release_id: release.release_id.clone(),
            knowledge_version: release.knowledge_version.clone(),
            target,
            old_bindings_sha256: Some(old_config.release_bindings_sha256()),
            new_bindings_sha256: Some(new_config.release_bindings_sha256()),
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
        let old_config = brain_serve::Config::parse(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let new_config = brain_serve::Config::parse(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let new_pin = journal.target.release(&new_config)?;
        ensure!(
            new_pin.id == journal.release_id
                && new_pin.knowledge_version == journal.knowledge_version,
            "activation_journal_pin"
        );
        let old_pin = journal.target.release(&old_config)?;
        let old: serde_json::Value = serde_json::from_slice(&old_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        let mut proposed: serde_json::Value = serde_json::from_slice(&new_bytes)
            .map_err(|_| anyhow::anyhow!("serve_config_schema"))?;
        journal.target.replace_pin(
            &mut proposed,
            &old_config,
            serde_json::json!({"id":old_pin.id,"knowledge_version":old_pin.knowledge_version}),
        )?;
        ensure!(old == proposed, "activation_changed_unrelated_config");
        match (&journal.old_bindings_sha256, &journal.new_bindings_sha256) {
            (Some(old), Some(new)) => ensure!(
                *old == old_config.release_bindings_sha256()
                    && *new == new_config.release_bindings_sha256(),
                "activation_journal_bindings"
            ),
            (None, None) => ensure!(
                journal.target == ActivationTarget::Standard,
                "activation_legacy_internal_journal"
            ),
            _ => anyhow::bail!("activation_journal_bindings"),
        }
        Ok(Self {
            path: config.serve_config.clone(),
            unit: config.serve_unit.clone(),
            old_bytes,
            new_bytes,
            release_id: release_id.to_owned(),
            journal_ref: reference.to_owned(),
            health_timeout_ms: config.health_timeout_ms,
            target: journal.target,
        })
    }

    pub fn target(&self) -> ActivationTarget {
        self.target
    }

    pub fn journal_ref(&self) -> &str {
        &self.journal_ref
    }
    pub fn needs_rebase(&self, writer: &ConfigWriter) -> Result<bool> {
        writer.require_path(&self.path)?;
        let current = writer.read()?;
        Ok(current != self.old_bytes && current != self.new_bytes)
    }

    pub async fn wait_for_restart_window(&self) {
        if self.old_bytes != self.new_bytes {
            RESTART_SCHEDULE.lock().await.wait().await;
        }
    }

    async fn restart_and_health(&self, expected: &[u8]) -> Result<()> {
        let serve = brain_serve::Config::parse(expected)
            .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
        let bindings_sha256 = serve.release_bindings_sha256();
        ensure!(serve.bind.ip().is_loopback(), "health_loopback");
        if self.old_bytes != self.new_bytes {
            {
                let mut schedule = RESTART_SCHEDULE.lock().await;
                schedule.wait().await;
                schedule.record(tokio::time::Instant::now());
            }
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
                        && parsed["release_bindings_sha256"] == bindings_sha256
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
