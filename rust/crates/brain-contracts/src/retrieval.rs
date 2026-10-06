//! Shared retrieval provenance and live authorization projection. No document bodies in head reads.
use crate::{DocumentRevision, PortError, Principal, SourceRecordV2, SourceVisibility};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkProvenance {
    pub document: DocumentRevision,
    pub chunker_version: String,
    pub ordinal: u32,
    /// Half-open UTF-8 byte range in the unmodified canonical document.
    pub byte_start: usize,
    pub byte_end: usize,
    pub source_locator: String,
    pub release_id: String,
    pub knowledge_version: String,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    /// Original metadata, including patch, mode, language, aliases and upstream provenance.
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHead {
    pub source_id: String,
    pub logical_id: String,
    pub revision: u64,
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub tombstone: bool,
    pub metadata: BTreeMap<String, String>,
}
impl From<&SourceRecordV2> for DocumentHead {
    fn from(record: &SourceRecordV2) -> Self {
        Self {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            revision: record.revision,
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            tombstone: record.tombstone,
            metadata: record.metadata.clone(),
        }
    }
}
impl DocumentHead {
    /// Validate the canonical policy projection without requiring a document body.
    pub fn canonical_origin(&self) -> Result<Option<crate::source::OriginArtifact>, PortError> {
        use crate::source::{OriginArtifact, SourceRevision, Versioned, ORIGIN_METADATA_KEY};
        let Some(encoded) = self.metadata.get(ORIGIN_METADATA_KEY) else {
            return Ok(None);
        };
        let current: Versioned<OriginArtifact> = serde_json::from_str(encoded)
            .map_err(|_| PortError::InvalidResponse("invalid current origin".into()))?;
        let current = current.data;
        if current.validate().is_err()
            || current.identity.source_id != self.source_id
            || current.identity.logical_id != self.logical_id
            || current.policy.visibility != self.visibility
            || current.policy.allowed_scopes != self.allowed_scopes
            || matches!(current.source_revision, SourceRevision::Wiki { revision_id, .. }
                if u64::try_from(revision_id).ok() != Some(self.revision))
        {
            return Err(PortError::InvalidResponse(
                "inconsistent current origin".into(),
            ));
        }
        Ok(Some(current))
    }
    pub fn validate(&self) -> Result<(), PortError> {
        if self.revision == 0
            || self.revision > i64::MAX as u64
            || [&self.source_id, &self.logical_id]
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 512 || s.chars().any(char::is_control))
        {
            return Err(PortError::InvalidResponse(
                "invalid current document head".into(),
            ));
        }
        Ok(())
    }
    pub fn allowed(&self, principal: &Principal, provider: bool) -> bool {
        if self.visibility == SourceVisibility::RequestScoped {
            return false;
        }
        if provider
            && !self.canonical_origin().is_ok_and(|origin| {
                origin.is_none_or(|origin| origin.policy.provider_egress_allowed)
            })
        {
            return false;
        }
        let class = match self.visibility {
            SourceVisibility::Public => "public",
            SourceVisibility::Internal => "internal",
            SourceVisibility::Private => "private",
            SourceVisibility::RequestScoped => return false,
        };
        let egress = self
            .metadata
            .get("egress")
            .map(String::as_str)
            .unwrap_or(class);
        !self.tombstone
            && (self.visibility == SourceVisibility::Public || !self.allowed_scopes.is_empty())
            && self.allowed_scopes.is_subset(&principal.scopes)
            && (!provider
                || (egress != "none"
                    && principal.provider_egress.contains(egress)
                    && principal.provider_egress.contains(class)))
    }
}
