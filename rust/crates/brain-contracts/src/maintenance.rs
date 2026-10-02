//! Dauerhafte Dokumentationsarbeit mit getrennten Quell-, Deploy- und Aktivierungsnachweisen.
use crate::{source::SourcePolicy, PortError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const MAINTENANCE_VERSION: &str = "brain.maintenance.v1";
pub const MAX_MAINTENANCE_LEASE_MS: u64 = 1_800_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceStatus {
    Planned,
    DiscoveredPolicyPending,
    SourceReview,
    Author,
    Reviewer,
    Publish,
    Activated,
    Failed,
    Superseded,
}
impl MaintenanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::DiscoveredPolicyPending => "discovered_policy_pending",
            Self::SourceReview => "source_review",
            Self::Author => "author",
            Self::Reviewer => "reviewer",
            Self::Publish => "publish",
            Self::Activated => "activated",
            Self::Failed => "failed",
            Self::Superseded => "superseded",
        }
    }
    pub fn can_transition(self, next: Self) -> bool {
        use MaintenanceStatus::*;
        matches!(
            (self, next),
            (Planned, SourceReview | DiscoveredPolicyPending)
                | (DiscoveredPolicyPending, SourceReview)
                | (SourceReview, Author | Reviewer)
                | (Author, Reviewer)
                | (Reviewer, Author | Publish)
                | (Publish, Activated)
        ) || (self != Activated && self != Superseded && next == Failed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceJobSpec {
    pub id: String,
    pub repo_id: String,
    pub source_sha: String,
    pub deployed_sha: Option<String>,
    pub target_path: String,
    pub idempotency_key: String,
    pub prompt_version: String,
    pub model: String,
    pub policy: SourcePolicy,
}
impl MaintenanceJobSpec {
    pub fn validate(&self) -> Result<(), PortError> {
        for value in [
            &self.id,
            &self.repo_id,
            &self.idempotency_key,
            &self.prompt_version,
            &self.model,
        ] {
            bounded(value, 512)?;
        }
        git_sha(&self.source_sha)?;
        if let Some(sha) = &self.deployed_sha {
            git_sha(sha)?;
        }
        bounded(&self.target_path, 1024)?;
        if self.target_path.starts_with('/')
            || self
                .target_path
                .split('/')
                .any(|s| s.is_empty() || s == ".." || s == ".")
            || self.target_path.contains('\\')
        {
            return Err(invalid("invalid documentation path"));
        }
        if self.policy.allowed_scopes.len() > 64 {
            return Err(invalid("too many source scopes"));
        }
        for scope in &self.policy.allowed_scopes {
            bounded(scope, 512)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceLease {
    pub job_id: String,
    pub owner: String,
    pub fence: u64,
    pub expires_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceReviewProof {
    pub reviewer_id: String,
    pub author_run_id: String,
    pub reviewer_run_id: String,
    pub source_sha: String,
    pub document_sha256: String,
    pub accepted: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenancePublicationProof {
    pub release_id: String,
    pub source_id: String,
    pub logical_id: String,
    pub document_revision: u64,
    pub document_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceActivationProof {
    pub active_release_id: String,
    pub verified_at_epoch: i64,
    /// Verweis auf gemessenen Dienst- oder Lesepfadnachweis.
    pub evidence_ref: String,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceCheckpoint {
    pub artifact_refs: BTreeMap<String, String>,
    pub review: Option<MaintenanceReviewProof>,
    pub publication: Option<MaintenancePublicationProof>,
    pub activation: Option<MaintenanceActivationProof>,
}
impl MaintenanceCheckpoint {
    pub fn validate(
        &self,
        spec: &MaintenanceJobSpec,
        next: MaintenanceStatus,
    ) -> Result<(), PortError> {
        if self.artifact_refs.len() > 32 {
            return Err(invalid("too many maintenance artifacts"));
        }
        for (key, value) in &self.artifact_refs {
            bounded(key, 128)?;
            bounded(value, 2048)?;
        }
        if let Some(review) = &self.review {
            bounded(&review.reviewer_id, 512)?;
            bounded(&review.author_run_id, 512)?;
            bounded(&review.reviewer_run_id, 512)?;
            if review.author_run_id == review.reviewer_run_id {
                return Err(invalid("author and reviewer must use separate runs"));
            }
            git_sha(&review.source_sha)?;
            sha256(&review.document_sha256)?;
            if review.source_sha != spec.source_sha {
                return Err(invalid("review source revision mismatch"));
            }
        }
        if let Some(publication) = &self.publication {
            bounded(&publication.release_id, 512)?;
            bounded(&publication.source_id, 512)?;
            bounded(&publication.logical_id, 1024)?;
            sha256(&publication.document_sha256)?;
            if publication.document_revision == 0 || publication.document_revision > i64::MAX as u64
            {
                return Err(invalid("invalid publication revision"));
            }
            if self
                .review
                .as_ref()
                .is_none_or(|r| !r.accepted || r.document_sha256 != publication.document_sha256)
            {
                return Err(invalid("publication lacks matching accepted review"));
            }
        }
        if matches!(
            next,
            MaintenanceStatus::Publish | MaintenanceStatus::Activated
        ) && self.review.as_ref().is_none_or(|r| !r.accepted)
        {
            return Err(invalid("publication requires accepted review"));
        }
        if next == MaintenanceStatus::Activated {
            let p = self
                .publication
                .as_ref()
                .ok_or_else(|| invalid("activation requires publication proof"))?;
            let a = self
                .activation
                .as_ref()
                .ok_or_else(|| invalid("activation requires live proof"))?;
            bounded(&a.evidence_ref, 2048)?;
            if a.active_release_id != p.release_id || a.verified_at_epoch <= 0 {
                return Err(invalid("activation release mismatch"));
            }
        }
        if matches!(
            next,
            MaintenanceStatus::Author
                | MaintenanceStatus::Reviewer
                | MaintenanceStatus::Publish
                | MaintenanceStatus::Activated
        ) && !spec.policy.provider_egress_allowed
        {
            return Err(invalid("source policy forbids provider work"));
        }
        if matches!(
            next,
            MaintenanceStatus::Publish | MaintenanceStatus::Activated
        ) && spec.policy.visibility == crate::SourceVisibility::Public
            && !spec.policy.publication_allowed
        {
            return Err(invalid("source policy forbids publication"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceJob {
    pub spec: MaintenanceJobSpec,
    pub status: MaintenanceStatus,
    pub attempts: u32,
    pub error_code: Option<String>,
    pub checkpoint: MaintenanceCheckpoint,
    pub lease: Option<MaintenanceLease>,
    pub superseded_by: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceSourceRegistration {
    pub repo_id: String,
    pub source_id: String,
    pub discovered_sha: String,
    pub policy: Option<SourcePolicy>,
    pub authorization_ref: Option<String>,
}
impl MaintenanceSourceRegistration {
    pub fn validate(&self) -> Result<(), PortError> {
        bounded(&self.repo_id, 512)?;
        bounded(&self.source_id, 512)?;
        git_sha(&self.discovered_sha)?;
        if let Some(reference) = &self.authorization_ref {
            bounded(reference, 2048)?;
        }
        if self.policy.is_some() && self.authorization_ref.is_none() {
            return Err(invalid(
                "registered policy requires authorization reference",
            ));
        }
        Ok(())
    }
}

pub trait MaintenanceStorePort: Send + Sync {
    fn enqueue_maintenance<'a>(
        &'a self,
        spec: &'a MaintenanceJobSpec,
        initial: MaintenanceStatus,
    ) -> crate::StoreFuture<'a, MaintenanceJob>;
    fn claim_maintenance<'a>(
        &'a self,
        owner: &'a str,
        ttl_ms: u64,
    ) -> crate::StoreFuture<'a, Option<MaintenanceJob>>;
    fn renew_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        ttl_ms: u64,
    ) -> crate::StoreFuture<'a, MaintenanceLease>;
    fn transition_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        next: MaintenanceStatus,
        checkpoint: &'a MaintenanceCheckpoint,
    ) -> crate::StoreFuture<'a, MaintenanceJob>;
    fn retry_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        error_code: &'a str,
        delay_ms: u64,
    ) -> crate::StoreFuture<'a, MaintenanceJob>;
    fn supersede_maintenance<'a>(
        &'a self,
        lease: &'a MaintenanceLease,
        replacement_id: &'a str,
    ) -> crate::StoreFuture<'a, MaintenanceJob>;
    fn maintenance_job<'a>(&'a self, id: &'a str)
        -> crate::StoreFuture<'a, Option<MaintenanceJob>>;
    fn maintenance_jobs<'a>(
        &'a self,
        repo_id: &'a str,
        limit: u32,
    ) -> crate::StoreFuture<'a, Vec<MaintenanceJob>>;
    fn register_maintenance_source<'a>(
        &'a self,
        registration: &'a MaintenanceSourceRegistration,
    ) -> crate::StoreFuture<'a, ()>;
    fn maintenance_sources(&self) -> crate::StoreFuture<'_, Vec<MaintenanceSourceRegistration>>;
}

pub fn bounded(value: &str, max: usize) -> Result<(), PortError> {
    if value.trim().is_empty() || value.len() > max || value.chars().any(char::is_control) {
        return Err(invalid("invalid maintenance identifier"));
    }
    Ok(())
}
fn git_sha(value: &str) -> Result<(), PortError> {
    if ![40, 64].contains(&value.len())
        || !value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(invalid("invalid Git revision"));
    }
    Ok(())
}
fn sha256(value: &str) -> Result<(), PortError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(invalid("invalid document hash"));
    }
    Ok(())
}
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terminal_states_do_not_restart_and_review_is_ordered() {
        assert!(!MaintenanceStatus::Activated.can_transition(MaintenanceStatus::Author));
        assert!(!MaintenanceStatus::Failed.can_transition(MaintenanceStatus::Publish));
        assert!(!MaintenanceStatus::Planned.can_transition(MaintenanceStatus::Activated));
        assert!(MaintenanceStatus::Reviewer.can_transition(MaintenanceStatus::Publish));
    }
}
