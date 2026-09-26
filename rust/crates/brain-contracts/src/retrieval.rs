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
        let class = match self.visibility {
            SourceVisibility::Public => "public",
            SourceVisibility::Internal => "internal",
            SourceVisibility::Private => "private",
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
