//! Shared read and mutation ports. Releases pin per-document revisions, not a source maximum.
use crate::{CorpusRelease, PortError, Principal, SourceRecordV2, SourceVisibility};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    pin::Pin,
};
pub const STORE_VERSION: &str = "brain.store.v2";
pub type StoreResult<T> = Result<T, PortError>;
pub type StoreFuture<'a, T> = Pin<Box<dyn Future<Output = StoreResult<T>> + Send + 'a>>;

/// Trusted execution purpose, chosen by the server rather than a wire Query.
/// Provider egress remains a separate permission from either answer purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerPurpose {
    InternalRead,
    ExternalPublication,
}

/// Publication requires both the pinned source grant and the current canonical
/// head grant. This deliberately does not change `record_allowed` (internal read).
/// Legacy records without versioned origins retain their existing ACL semantics;
/// a versioned origin may never regain publication by losing its current grant.
pub fn record_publication_allowed(
    record: &SourceRecordV2,
    head: &crate::DocumentHead,
    principal: &Principal,
) -> bool {
    use crate::source::{
        origin_from_record, OriginArtifact, SourceRevision, Versioned, ORIGIN_METADATA_KEY,
    };
    if record.validate().is_err()
        || head.validate().is_err()
        || record.source_id != head.source_id
        || record.logical_id != head.logical_id
        || head.revision < record.revision
        || record.tombstone
        || !record_allowed(record, principal, false)
        || !head.allowed(principal, false)
    {
        return false;
    }
    let versioned = record.metadata.contains_key(ORIGIN_METADATA_KEY);
    if versioned
        && !origin_from_record(record).is_ok_and(|origin| origin.policy.publication_allowed)
    {
        return false;
    }
    let Some(encoded) = head.metadata.get(ORIGIN_METADATA_KEY) else {
        return !versioned;
    };
    let Ok(current) = serde_json::from_str::<Versioned<OriginArtifact>>(encoded) else {
        return false;
    };
    let current = current.data;
    current.validate().is_ok()
        && current.identity.source_id == head.source_id
        && current.identity.logical_id == head.logical_id
        && current.policy.visibility == head.visibility
        && current.policy.allowed_scopes == head.allowed_scopes
        && current.policy.publication_allowed
        && match current.source_revision {
            SourceRevision::Wiki { revision_id, .. } => {
                u64::try_from(revision_id).ok() == Some(head.revision)
            }
            _ => true,
        }
}

impl CorpusSnapshot {
    /// Canonical domain proofs can depend on many documents, including uncited
    /// rules/cards. Keep the normal read view intact and derive a publication view.
    pub fn authorized_for_publication(
        &self,
        principal: &Principal,
    ) -> StoreResult<Vec<SourceRecordV2>> {
        let records = self.authorized(principal, false)?;
        let heads: BTreeMap<_, _> = self
            .heads
            .iter()
            .map(|r| ((&r.source_id, &r.logical_id), crate::DocumentHead::from(r)))
            .collect();
        Ok(records
            .into_iter()
            .filter(|record| {
                heads
                    .get(&(&record.source_id, &record.logical_id))
                    .is_some_and(|head| record_publication_allowed(record, head, principal))
            })
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCheckpoint {
    pub source_id: String,
    pub configuration: String,
    pub generation: u64,
    pub state: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBatch {
    pub expected_generation: u64,
    pub checkpoint: SourceCheckpoint,
    pub records: Vec<SourceRecordV2>,
}
impl SourceBatch {
    pub fn validate(&self) -> StoreResult<()> {
        if self.checkpoint.source_id.trim().is_empty()
            || self.checkpoint.configuration.trim().is_empty()
            || self.expected_generation.checked_add(1) != Some(self.checkpoint.generation)
            || self.checkpoint.generation > i64::MAX as u64
            || self.records.len() > 10000
        {
            return Err(PortError::InvalidResponse(
                "invalid batch checkpoint".into(),
            ));
        }
        let mut keys = BTreeSet::new();
        for record in &self.records {
            record
                .validate()
                .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
            if record
                .metadata
                .contains_key(crate::source::ORIGIN_METADATA_KEY)
            {
                crate::source::origin_from_record(record).map_err(PortError::InvalidResponse)?;
            }
            if record.source_id != self.checkpoint.source_id || !keys.insert(&record.logical_id) {
                return Err(PortError::InvalidResponse(
                    "cross-source or duplicate batch record".into(),
                ));
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lease {
    pub source_id: String,
    pub owner: String,
    pub fence: u64,
    pub expires_at_ms: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchReceipt {
    pub generation: u64,
    pub replayed: bool,
}
pub trait DocumentStorePort: Send + Sync {
    fn checkpoint<'a>(&'a self, source: &'a str) -> StoreFuture<'a, Option<SourceCheckpoint>>;
    fn claim<'a>(&'a self, source: &'a str, owner: &'a str, ttl_ms: u64) -> StoreFuture<'a, Lease>;
    fn commit<'a>(
        &'a self,
        batch: &'a SourceBatch,
        lease: &'a Lease,
    ) -> StoreFuture<'a, BatchReceipt>;
    fn publish<'a>(&'a self, release: &'a CorpusRelease) -> StoreFuture<'a, ()>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusSnapshot {
    pub release: CorpusRelease,
    pub revisions: Vec<SourceRecordV2>,
    pub heads: Vec<SourceRecordV2>,
}
pub trait SnapshotReadPort: Send + Sync {
    fn read_snapshot(&self, release_id: &str) -> StoreResult<CorpusSnapshot>;
    /// Request adapters override this to bound pool/statement waits as well as handoffs.
    fn read_snapshot_until(
        &self,
        release_id: &str,
        deadline: Option<&crate::RequestDeadline>,
    ) -> StoreResult<CorpusSnapshot> {
        if let Some(deadline) = deadline {
            deadline.check()?;
        }
        let result = self.read_snapshot(release_id);
        if let Some(deadline) = deadline {
            deadline.check()?;
        }
        result
    }
    fn read_heads_until(
        &self,
        documents: &[crate::DocumentRevision],
        deadline: Option<&crate::RequestDeadline>,
    ) -> StoreResult<Vec<crate::DocumentHead>> {
        if let Some(deadline) = deadline {
            deadline.check()?;
        }
        let result = self.read_heads(documents);
        if let Some(deadline) = deadline {
            deadline.check()?;
        }
        result
    }
    /// Fresh bounded primary-key reads. Never fall back to a full corpus snapshot.
    /// Missing heads are absent; database/pool/timeout failures remain Unavailable.
    fn read_heads(
        &self,
        _documents: &[crate::DocumentRevision],
    ) -> StoreResult<Vec<crate::DocumentHead>> {
        Err(PortError::Unavailable(
            "targeted current-head reads not implemented".into(),
        ))
    }
}
/// Ownership is immutable. Implementations must compare-and-create atomically.
pub trait ConversationOwnershipPort: Send + Sync {
    fn claim_conversation(&self, conversation: &str, actor: &str) -> StoreResult<()>;
    fn claim_conversation_until(
        &self,
        conversation: &str,
        actor: &str,
        deadline: &crate::RequestDeadline,
    ) -> StoreResult<()> {
        deadline.check()?;
        let result = self.claim_conversation(conversation, actor);
        deadline.check()?;
        result
    }
}
impl CorpusSnapshot {
    pub fn authorized(
        &self,
        principal: &Principal,
        provider: bool,
    ) -> StoreResult<Vec<SourceRecordV2>> {
        let count: usize = self
            .release
            .source_revisions
            .values()
            .map(BTreeMap::len)
            .sum();
        if count != self.revisions.len() || count > 10000 {
            return Err(PortError::InvalidResponse("incomplete release".into()));
        }
        let heads: BTreeMap<_, _> = self
            .heads
            .iter()
            .map(|r| ((&r.source_id, &r.logical_id), r))
            .collect();
        if heads.len() != self.heads.len() {
            return Err(PortError::InvalidResponse("duplicate head".into()));
        }
        let mut seen = BTreeSet::new();
        let mut records = Vec::new();
        for record in &self.revisions {
            record
                .validate()
                .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
            if !seen.insert((&record.source_id, &record.logical_id))
                || self
                    .release
                    .source_revisions
                    .get(&record.source_id)
                    .and_then(|r| r.get(&record.logical_id))
                    != Some(&record.revision)
            {
                return Err(PortError::InvalidResponse("unpinned revision".into()));
            }
            let head = heads
                .get(&(&record.source_id, &record.logical_id))
                .ok_or_else(|| PortError::InvalidResponse("current ACL missing".into()))?;
            head.validate()
                .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
            if head.revision < record.revision {
                return Err(PortError::InvalidResponse("head predates release".into()));
            }
            if [record, *head]
                .iter()
                .any(|r| r.tombstone || !record_allowed(r, principal, provider))
            {
                continue;
            }
            let mut visible = record.clone();
            visible
                .allowed_scopes
                .extend(head.allowed_scopes.iter().cloned());
            visible.visibility = match (record.visibility, head.visibility) {
                (SourceVisibility::Private, _) | (_, SourceVisibility::Private) => {
                    SourceVisibility::Private
                }
                (SourceVisibility::Internal, _) | (_, SourceVisibility::Internal) => {
                    SourceVisibility::Internal
                }
                _ => SourceVisibility::Public,
            };
            // Preserve current restrictions with the immutable raw/source identity.
            if record
                .metadata
                .contains_key(crate::source::ORIGIN_METADATA_KEY)
            {
                let mut origin = crate::source::origin_from_record(record)
                    .map_err(PortError::InvalidResponse)?;
                origin.policy.visibility = visible.visibility;
                origin.policy.allowed_scopes = visible.allowed_scopes.clone();
                if head
                    .metadata
                    .contains_key(crate::source::ORIGIN_METADATA_KEY)
                {
                    let current = crate::source::origin_from_record(head)
                        .map_err(PortError::InvalidResponse)?;
                    origin.policy.provider_egress_allowed &= current.policy.provider_egress_allowed;
                    origin.policy.publication_allowed &= current.policy.publication_allowed;
                    origin.policy.raw_retention_allowed &= current.policy.raw_retention_allowed;
                } else {
                    // A legacy head cannot assert current rights for a versioned origin.
                    origin.policy.provider_egress_allowed = false;
                    origin.policy.publication_allowed = false;
                }
                origin
                    .bind_record(&mut visible)
                    .map_err(PortError::InvalidResponse)?;
            }
            // Effective provenance may deny egress when a legacy head has no current grant.
            if !record_allowed(&visible, principal, provider) {
                continue;
            }
            records.push(visible);
        }
        Ok(records)
    }
}
pub fn record_allowed(record: &SourceRecordV2, principal: &Principal, provider: bool) -> bool {
    if record
        .metadata
        .contains_key(crate::source::ORIGIN_METADATA_KEY)
    {
        let Ok(origin) = crate::source::origin_from_record(record) else {
            return false;
        };
        if provider && !origin.policy.provider_egress_allowed {
            return false;
        }
    }
    let visibility = match record.visibility {
        SourceVisibility::Public => "public",
        SourceVisibility::Internal => "internal",
        SourceVisibility::Private => "private",
    };
    let acl = (record.visibility == SourceVisibility::Public || !record.allowed_scopes.is_empty())
        && record.allowed_scopes.is_subset(&principal.scopes);
    let egress = record
        .metadata
        .get("egress")
        .map(String::as_str)
        .unwrap_or(visibility);
    acl && (!provider
        || (egress != "none"
            && principal.provider_egress.contains(egress)
            && principal.provider_egress.contains(visibility)))
}
