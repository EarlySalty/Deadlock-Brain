//! Versioned source semantics shared by Wiki, external adapters, Replay and storage.
//! Source revisions and observation timestamps NEVER establish game validity.
use crate::{
    value::{Observed, UnknownReason},
    SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const IR_VERSION: &str = "brain.ir.v1";
pub const ORIGIN_METADATA_KEY: &str = "brain.origin";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrVersion {
    #[serde(rename = "brain.ir.v1")]
    V1,
}

/// An explicit version is mandatory. Unknown versions/fields fail closed; no fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Versioned<T> {
    pub contract_version: IrVersion,
    pub data: T,
}
impl<T> Versioned<T> {
    pub fn new(data: T) -> Self {
        Self {
            contract_version: IrVersion::V1,
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceIdentity {
    pub source_id: String,
    pub logical_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceRevision {
    Http {
        body_sha256: String,
        etag: Option<String>,
        last_modified: Option<String>,
    },
    Git {
        commit: String,
    },
    Wiki {
        page_id: i64,
        revision_id: i64,
    },
    Api {
        api_version: String,
        original_revision: Option<String>,
    },
    Replay {
        original_revision: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameValidity {
    pub patch: Observed<String>,
    pub mode: Observed<String>,
    /// Game/patch bounds, never source publication/retrieval times.
    pub valid_from: Observed<String>,
    pub valid_to: Observed<String>,
}
impl GameValidity {
    pub fn unknown() -> Self {
        Self {
            patch: Observed::unknown(UnknownReason::NotPresent),
            mode: Observed::unknown(UnknownReason::NotPresent),
            valid_from: Observed::unknown(UnknownReason::NotPresent),
            valid_to: Observed::unknown(UnknownReason::NotPresent),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "unit",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SourceTimestamp {
    UnixSeconds(i64),
    UnixMilliseconds(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePolicy {
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub authorization_ref: Observed<String>,
    pub license: Observed<String>,
    pub publication_allowed: bool,
    pub provider_egress_allowed: bool,
    pub raw_retention_allowed: bool,
}

/// Exact raw artifact and derivation identity. This is data, not a rights grant.
/// Authorization must also check current ACLs/tombstones through the existing store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginArtifact {
    pub identity: SourceIdentity,
    pub source_revision: SourceRevision,
    pub raw_sha256: String,
    pub locator: String,
    pub parser_revision: String,
    pub parser_family: String,
    pub schema_version: Observed<String>,
    pub schema_sha256: Observed<String>,
    pub retrieved_at: Observed<SourceTimestamp>,
    pub source_time: Observed<SourceTimestamp>,
    pub language: Observed<String>,
    /// Explicit common upstream artifact identities, not purported independent votes.
    pub origin_artifacts: BTreeSet<String>,
    pub derivation_family: Observed<String>,
    pub policy: SourcePolicy,
    pub validity: GameValidity,
}

pub fn observed_option<T>(value: Option<T>) -> Observed<T> {
    value.map_or_else(
        || Observed::unknown(UnknownReason::NotPresent),
        Observed::known,
    )
}
fn valid_id(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}
fn valid_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
impl OriginArtifact {
    pub fn validate(&self) -> Result<(), String> {
        if [
            &self.identity.source_id,
            &self.identity.logical_id,
            &self.locator,
            &self.parser_revision,
            &self.parser_family,
        ]
        .into_iter()
        .any(|s| !valid_id(s))
            || !valid_hash(&self.raw_sha256)
            || self.policy.allowed_scopes.iter().any(|s| !valid_id(s))
        {
            return Err("invalid origin identity, locator, parser, hash or scope".into());
        }
        let valid_revision = match &self.source_revision {
            SourceRevision::Http { body_sha256, .. } => body_sha256 == &self.raw_sha256,
            SourceRevision::Git { commit } => {
                (commit.len() == 40 || commit.len() == 64)
                    && commit.bytes().all(|b| b.is_ascii_hexdigit())
            }
            SourceRevision::Wiki {
                page_id,
                revision_id,
            } => *page_id > 0 && *revision_id > 0,
            SourceRevision::Api {
                api_version,
                original_revision,
            } => valid_id(api_version) && original_revision.as_ref().is_none_or(|s| valid_id(s)),
            SourceRevision::Replay { original_revision } => valid_id(original_revision),
        };
        if !valid_revision {
            return Err("invalid original source revision".into());
        }
        if let Observed::Known { value } = &self.schema_sha256 {
            if !valid_hash(value) {
                return Err("invalid schema fingerprint".into());
            }
        }
        Ok(())
    }

    /// Metadata bridge to the EXISTING SourceRecordV2/DocumentStorePort, no second store.
    /// The caller must supply a real monotonic store revision (never a fabricated Git->u64).
    pub fn bind_record(&self, record: &mut SourceRecordV2) -> Result<(), String> {
        self.check_record(record)?;
        let encoded = serde_json::to_string(&Versioned::new(self)).map_err(|e| e.to_string())?;
        record.metadata.insert(ORIGIN_METADATA_KEY.into(), encoded);
        Ok(())
    }
    pub fn check_record(&self, record: &SourceRecordV2) -> Result<(), String> {
        self.validate()?;
        record.validate().map_err(|e| e.to_string())?;
        if self.identity.source_id != record.source_id
            || self.identity.logical_id != record.logical_id
            || self.raw_sha256 != record.content_hash
            || self.policy.visibility != record.visibility
            || self.policy.allowed_scopes != record.allowed_scopes
        {
            return Err("origin/source revision or ACL mismatch".into());
        }
        if let SourceRevision::Wiki { revision_id, .. } = self.source_revision {
            if u64::try_from(revision_id).ok() != Some(record.revision) {
                return Err("wiki revision mismatch".into());
            }
        }
        Ok(())
    }
}

pub fn origin_from_record(record: &SourceRecordV2) -> Result<OriginArtifact, String> {
    let encoded = record
        .metadata
        .get(ORIGIN_METADATA_KEY)
        .ok_or("missing versioned origin")?;
    let origin: Versioned<OriginArtifact> =
        serde_json::from_str(encoded).map_err(|e| e.to_string())?;
    origin.data.check_record(record)?;
    Ok(origin.data)
}
