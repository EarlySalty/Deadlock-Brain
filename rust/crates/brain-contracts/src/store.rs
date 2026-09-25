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
}
/// Ownership is immutable. Implementations must compare-and-create atomically.
pub trait ConversationOwnershipPort: Send + Sync {
    fn claim_conversation(&self, conversation: &str, actor: &str) -> StoreResult<()>;
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
            records.push(visible);
        }
        Ok(records)
    }
}
pub fn record_allowed(record: &SourceRecordV2, principal: &Principal, provider: bool) -> bool {
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
