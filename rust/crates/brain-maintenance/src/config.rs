use std::path::{Component, Path, PathBuf};

use anyhow::{ensure, Result};
use brain_contracts::source::SourcePolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceConfig {
    pub repo_roots: Vec<PathBuf>,
    pub repositories: Vec<RepositoryConfig>,
    pub docs_repo: PathBuf,
    pub docs_origin: String,
    pub docs_ref: String,
    #[serde(default)]
    pub private_document_root: Option<PathBuf>,
    pub bounds: Bounds,
    pub codex: CodexConfig,
    pub prompt_version: String,
    pub internal_doc_scopes: std::collections::BTreeSet<String>,
    pub jev: brain_jev::transport::JevTransportConfig,
    pub new_document_format: String,
    pub registered_assets: Vec<AssetProvenance>,
    #[serde(skip)]
    pub loaded_from: Option<PathBuf>,
    #[serde(skip)]
    pub canonical_documents:
        std::collections::BTreeMap<String, brain_ingestion::document_set::CoreDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetProvenance {
    pub src: String,
    pub git_path: String,
    pub sha256: String,
    pub source_locator: String,
    pub alt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryConfig {
    pub id: String,
    pub path: PathBuf,
    pub origin: String,
    pub source_ref: String,
    pub source_paths: Vec<String>,
    pub evidence_paths: Vec<String>,
    pub doc_targets: Vec<String>,
    pub output_paths: std::collections::BTreeMap<String, String>,
    pub policy: SourcePolicy,
    pub document_policy: SourcePolicy,
    #[serde(default)]
    pub target_document_policies: std::collections::BTreeMap<String, SourcePolicy>,
    #[serde(default)]
    pub code_only_migration_targets: std::collections::BTreeSet<String>,
    pub code_only_export_approved: bool,
    pub deployed_sha_file: Option<PathBuf>,
}

impl RepositoryConfig {
    pub fn document_policy_for(&self, target: &str) -> &SourcePolicy {
        self.target_document_policies
            .get(target)
            .unwrap_or(&self.document_policy)
    }
}

pub fn require_registered(config: &MaintenanceConfig, repo: &RepositoryConfig) -> Result<()> {
    if let Some(path) = &config.loaded_from {
        let bytes = crate::integration::runtime_config::read_bounded(path, 256 * 1024)?;
        let current: MaintenanceConfig = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("Aktuelle Registrierung ist ungültig"))?;
        current.validate()?;
        ensure!(
            serde_json::to_value(&current)? == serde_json::to_value(config)?,
            "Registrierung oder Rechte wurden verändert"
        );
    }
    let current = config
        .repositories
        .iter()
        .find(|r| r.id == repo.id)
        .ok_or_else(|| anyhow::anyhow!("Repo fehlt in der aktuellen Registrierung"))?;
    ensure!(
        serde_json::to_value(current)? == serde_json::to_value(repo)?,
        "Repo stimmt nicht mit der aktuellen Registrierung überein"
    );
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub max_repositories: usize,
    pub max_files: usize,
    pub max_blob_bytes: usize,
    pub max_bundle_bytes: usize,
    pub max_output_bytes: usize,
    pub git_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodexConfig {
    pub executable: PathBuf,
    pub model: String,
    pub reasoning_effort: String,
    pub timeout_ms: u64,
}

pub fn safe_relative(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty() && !path.starts_with('-'),
        "Pfad fehlt oder beginnt mit einem Schalter"
    );
    ensure!(
        !path.contains(['\\', ':', '\0', '\n', '\r']),
        "Ungültiger Pfad"
    );
    ensure!(
        Path::new(path)
            .components()
            .all(|part| matches!(part, Component::Normal(_))),
        "Pfad muss innerhalb des Repos liegen"
    );
    ensure!(
        path.split('/')
            .all(|part| !part.is_empty() && part != "." && part != ".."),
        "Ungültiger Pfadbestandteil"
    );
    Ok(())
}

pub fn safe_doc_target(path: &str) -> Result<()> {
    safe_relative(path)?;
    ensure!(
        (path.starts_with("internal/") || path.starts_with("public/"))
            && (path.ends_with(".md") || path.ends_with(".html")),
        "Nur freigegebene Textdokumente in internal/public sind erlaubt"
    );
    Ok(())
}

impl MaintenanceConfig {
    pub fn validate(&self) -> Result<()> {
        self.jev.validate()?;
        ensure!(
            self.registered_assets.len() <= 64,
            "Zu viele registrierte Bildquellen"
        );
        let mut assets = std::collections::BTreeSet::new();
        for asset in &self.registered_assets {
            safe_relative(&asset.git_path)?;
            ensure!(
                assets.insert(&asset.src)
                    && !asset.src.is_empty()
                    && !asset.src.contains([':', '\\'])
                    && !asset.src.split('/').any(|p| p == "..")
                    && !asset.src.chars().any(char::is_control),
                "Ungültiger registrierter Bildpfad"
            );
            ensure!(
                asset.sha256.len() == 64
                    && asset
                        .sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    && !asset.source_locator.trim().is_empty()
                    && !asset.alt.trim().is_empty(),
                "Bildprovenanz oder Bildbeschreibung fehlt"
            );
        }
        ensure!(
            self.new_document_format == "html",
            "Neue Dokumentation muss HTML verwenden"
        );
        ensure!(
            !self.internal_doc_scopes.is_empty()
                && self.internal_doc_scopes.len() <= 64
                && self
                    .internal_doc_scopes
                    .iter()
                    .all(|s| !s.trim().is_empty() && !s.chars().any(char::is_control)),
            "Interne Dokumentscopes fehlen"
        );
        ensure!(
            self.docs_repo.is_absolute() && self.codex.executable.is_absolute(),
            "Absolute Repo- und CLI-Pfade sind erforderlich"
        );
        ensure!(
            !self.docs_origin.is_empty() && !self.prompt_version.is_empty(),
            "Quellenidentität und Promptversion fehlen"
        );
        validate_ref(&self.docs_ref)?;
        ensure!(
            !self.repo_roots.is_empty() && self.repo_roots.iter().all(|p| p.is_absolute()),
            "Absolute Suchwurzeln sind erforderlich"
        );
        ensure!(
            self.bounds.max_repositories > 0
                && self.bounds.max_repositories <= 256
                && self.repositories.len() <= self.bounds.max_repositories,
            "Repo-Grenze überschritten"
        );
        ensure!(
            (1..=2048).contains(&self.bounds.max_files),
            "Ungültige Dateigrenze"
        );
        ensure!(
            (1..=4 * 1024 * 1024).contains(&self.bounds.max_blob_bytes),
            "Ungültige Blobgrenze"
        );
        ensure!(
            self.bounds.max_bundle_bytes >= self.bounds.max_blob_bytes
                && self.bounds.max_bundle_bytes <= 32 * 1024 * 1024,
            "Ungültige Kontextgrenze"
        );
        ensure!(
            (1..=4 * 1024 * 1024).contains(&self.bounds.max_output_bytes),
            "Ungültige Ausgabegrenze"
        );
        ensure!(
            (1..=120_000).contains(&self.bounds.git_timeout_ms)
                && (1..=1_800_000).contains(&self.codex.timeout_ms),
            "Ungültige Zeitgrenze"
        );
        ensure!(
            !self.codex.model.is_empty() && !self.codex.model.chars().any(char::is_control),
            "Modell fehlt"
        );
        ensure!(
            ["low", "medium", "high", "xhigh"].contains(&self.codex.reasoning_effort.as_str()),
            "Ungültige Reasoning-Einstellung"
        );
        let mut ids = std::collections::BTreeSet::new();
        let mut targets = std::collections::BTreeSet::new();
        let mut outputs = std::collections::BTreeSet::new();
        for repo in &self.repositories {
            ensure!(
                !["rs-relay", "ai-coach", "TradingBot"]
                    .iter()
                    .any(|excluded| repo.id.eq_ignore_ascii_case(excluded)
                        || repo
                            .path
                            .file_name()
                            .is_some_and(|n| n.to_string_lossy().eq_ignore_ascii_case(excluded))),
                "Repo ist vom Auftrag ausgeschlossen"
            );
            ensure!(
                !repo.id.is_empty() && ids.insert(&repo.id),
                "Repo-ID fehlt oder ist doppelt"
            );
            ensure!(
                repo.path.is_absolute() && !repo.origin.is_empty(),
                "Repo-Pfad oder Origin fehlt"
            );
            validate_ref(&repo.source_ref)?;
            ensure!(
                !repo.source_paths.is_empty() && !repo.doc_targets.is_empty(),
                "Quellpfade oder Dokumentziele fehlen"
            );
            ensure!(
                !repo.document_policy.provider_egress_allowed
                    || repo.policy.provider_egress_allowed,
                "Dokumentpolicy darf Exportrechte nicht erweitern"
            );
            // Die Freigabe der abgeleiteten Nutzerhilfe steht am Dokumentziel.
            // Sie gibt weder Quellcode noch interne Begleitdokumente frei.
            ensure!(
                repo.document_policy.visibility == brain_contracts::SourceVisibility::Public
                    || !repo.document_policy.allowed_scopes.is_empty(),
                "Interne Dokumente brauchen explizite Scopes"
            );
            for path in &repo.source_paths {
                safe_relative(path)?;
            }
            ensure!(!repo.evidence_paths.is_empty(), "Erste Evidenzpfade fehlen");
            for path in &repo.evidence_paths {
                safe_relative(path)?;
                ensure!(
                    repo.source_paths
                        .iter()
                        .any(|scope| path == scope || path.starts_with(&format!("{scope}/"))),
                    "Evidenz liegt außerhalb des Quellbereichs"
                );
            }
            for path in &repo.doc_targets {
                let document_policy = repo.document_policy_for(path);
                safe_doc_target(path)?;
                ensure!(
                    !path.starts_with("public/")
                        || (document_policy.visibility
                            == brain_contracts::SourceVisibility::Public
                            && document_policy.publication_allowed),
                    "Öffentliches Ziel braucht öffentliche Dokumentpolicy"
                );
                ensure!(targets.insert(path), "Dokumentziel hat mehrere Eigentümer");
                let output = repo
                    .output_paths
                    .get(path)
                    .map_or(path.as_str(), String::as_str);
                safe_doc_target(output)?;
                ensure!(
                    !output.starts_with("public/")
                        || (document_policy.visibility
                            == brain_contracts::SourceVisibility::Public
                            && document_policy.publication_allowed),
                    "Öffentlicher Ausgabepfad braucht öffentliche Dokumentpolicy"
                );
                ensure!(
                    outputs.insert(output),
                    "Physischer Ausgabepfad hat mehrere logische Eigentümer"
                );
                ensure!(
                    output.ends_with(".html"),
                    "Neues oder überarbeitetes Dokument braucht einen HTML-Ausgabepfad"
                );
            }
            ensure!(
                repo.target_document_policies
                    .keys()
                    .all(|p| repo.doc_targets.contains(p)),
                "Policy hat kein registriertes Dokumentziel"
            );
            for policy in repo.target_document_policies.values() {
                ensure!(
                    policy.visibility == brain_contracts::SourceVisibility::Public
                        || !policy.allowed_scopes.is_empty(),
                    "Interne Dokumente brauchen Scopes"
                );
                ensure!(
                    !policy.provider_egress_allowed || repo.policy.provider_egress_allowed,
                    "Dokumentpolicy erweitert Exportrechte"
                );
            }
            ensure!(repo.code_only_migration_targets.iter().all(|target|
                repo.doc_targets.contains(target) && target.starts_with("internal/")
                    && !target.starts_with("internal/public-candidates/")
                    && repo.document_policy_for(target).provider_egress_allowed),
                "Codebasierte Migration braucht ein ausdrücklich freigegebenes internes Technikziel");
            ensure!(
                repo.output_paths
                    .keys()
                    .all(|key| repo.doc_targets.contains(key)),
                "Ausgabepfad ist keiner logischen Seite zugeordnet"
            );
            if let Some(path) = &repo.deployed_sha_file {
                ensure!(
                    path.is_absolute(),
                    "Deploy-Beleg muss einen absoluten Pfad haben"
                );
            }
        }
        for repo in &self.repositories {
            for target in &repo.doc_targets {
                let output = repo.output_paths.get(target).unwrap_or(target);
                ensure!(
                    output == target || !targets.contains(output),
                    "Ausgabepfad überschreibt eine andere logische Seite"
                );
            }
        }
        Ok(())
    }
}

pub fn validate_ref(reference: &str) -> Result<()> {
    ensure!(
        reference.starts_with("refs/remotes/origin/")
            && reference
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"/_-.".contains(&c))
            && !reference.contains(".."),
        "Nur explizite origin-Referenzen sind erlaubt"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_paths_enforce_policy_and_unique_ownership() {
        let baseline: MaintenanceConfig =
            serde_json::from_str(include_str!("../config/maintenance.example.json")).unwrap();
        assert!(baseline.validate().is_ok());
        let mut public_output = baseline.clone();
        let first_target = public_output.repositories[0].doc_targets[0].clone();
        public_output.repositories[0]
            .output_paths
            .insert(first_target, "public/unerlaubt.html".into());
        assert!(public_output.validate().is_err());

        let mut collision = baseline;
        let first_repo = &collision.repositories[0];
        let first_target = &first_repo.doc_targets[0];
        let first_output = first_repo
            .output_paths
            .get(first_target)
            .unwrap_or(first_target)
            .clone();
        let second_target = collision.repositories[1].doc_targets[0].clone();
        collision.repositories[1]
            .output_paths
            .insert(second_target, first_output);
        assert!(collision.validate().is_err());
        let mut alias: MaintenanceConfig =
            serde_json::from_str(include_str!("../config/maintenance.example.json")).unwrap();
        let first = alias.repositories[0].doc_targets[0].clone();
        let second = alias.repositories[1].doc_targets[0].clone();
        alias.repositories[0]
            .output_paths
            .insert(first, second.clone());
        alias.repositories[1]
            .output_paths
            .insert(second, "internal/alias-neu.html".into());
        assert_eq!(
            alias.validate().unwrap_err().to_string(),
            "Ausgabepfad überschreibt eine andere logische Seite"
        );
    }
    #[test]
    fn target_escape_and_environment_files_are_rejected() {
        for path in [
            "../public/x.md",
            "public/../x.md",
            "public//x.md",
            "/public/x.md",
            "public/.env",
            "private/x.md",
        ] {
            assert!(safe_doc_target(path).is_err(), "{path}");
        }
        assert!(safe_doc_target("internal/bot/befehle.md").is_ok());
    }
    #[test]
    fn branch_resolution_accepts_only_origin_refs() {
        assert!(validate_ref("refs/remotes/origin/main").is_ok());
        for reference in [
            "HEAD",
            "main",
            "--upload-pack=x",
            "refs/remotes/origin/../x",
        ] {
            assert!(validate_ref(reference).is_err());
        }
    }
}
