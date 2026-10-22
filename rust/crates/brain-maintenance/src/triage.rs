use crate::{
    config::{MaintenanceConfig, RepositoryConfig},
    scanner::ScanResult,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriageOutcome {
    pub status: String,
    pub source_review_required: bool,
    pub error_code: Option<String>,
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
}

pub async fn triage(
    _config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    scan: &ScanResult,
    target: &str,
    _credential: &str,
) -> Result<TriageOutcome> {
    ensure!(
        repo.policy.provider_egress_allowed && repo.code_only_export_approved,
        "Codeexport nicht freigegeben"
    );
    let document = scan
        .documents
        .get(target)
        .ok_or_else(|| anyhow::anyhow!("Dokumentziel nicht registriert"))?;
    ensure!(
        repo.document_policy_for(target).provider_egress_allowed || document.is_none(),
        "Dokumentexport nicht freigegeben"
    );
    crate::require_maintenance_enabled()?;
    anyhow::bail!("Die bisherige Dokumentprüfung ist deaktiviert.")
}
