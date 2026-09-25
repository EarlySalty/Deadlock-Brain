//! Transactional reference store for contract tests and explicitly in-memory compositions.
use crate::{ApplyOutcome, MemoryStore, RecordSink};
use brain_contracts::{
    BatchReceipt, CorpusRelease, CorpusSnapshot, DocumentStorePort, Lease, PortError,
    SnapshotReadPort, SourceBatch, SourceCheckpoint, SourceRecordV2, StoreFuture,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Default)]
pub struct MemoryRepository {
    inner: Arc<Mutex<State>>,
}
#[derive(Clone, Default)]
struct State {
    records: MemoryStore,
    releases: BTreeMap<String, CorpusRelease>,
    checkpoints: BTreeMap<String, (SourceCheckpoint, String)>,
    leases: BTreeMap<String, Lease>,
}
fn error(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn now_ms() -> Result<u64, PortError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .map_err(|_| error("clock before epoch"))
}
impl MemoryRepository {
    pub fn apply_record(&self, record: SourceRecordV2) -> Result<ApplyOutcome, PortError> {
        self.inner
            .lock()
            .map_err(|_| error("store poisoned"))?
            .records
            .apply(record)
            .map_err(|e| error(&e.to_string()))
    }
    pub fn release_from_heads(
        &self,
        id: &str,
        version: &str,
        patch: &str,
    ) -> Result<CorpusRelease, PortError> {
        Ok(self
            .inner
            .lock()
            .map_err(|_| error("store poisoned"))?
            .records
            .corpus_release(id, version, patch, (now_ms()? / 1000) as i64))
    }
}
impl DocumentStorePort for MemoryRepository {
    fn checkpoint<'a>(&'a self, source: &'a str) -> StoreFuture<'a, Option<SourceCheckpoint>> {
        Box::pin(async move {
            Ok(self
                .inner
                .lock()
                .map_err(|_| error("store poisoned"))?
                .checkpoints
                .get(source)
                .map(|(c, _)| c.clone()))
        })
    }
    fn claim<'a>(&'a self, source: &'a str, owner: &'a str, ttl_ms: u64) -> StoreFuture<'a, Lease> {
        Box::pin(async move {
            if source.trim().is_empty() || owner.trim().is_empty() || !(1..=60000).contains(&ttl_ms)
            {
                return Err(error("invalid lease"));
            }
            let now = now_ms()?;
            let mut state = self.inner.lock().map_err(|_| error("store poisoned"))?;
            if state
                .leases
                .get(source)
                .is_some_and(|l| l.expires_at_ms > now)
            {
                return Err(error("source already leased"));
            }
            let fence = state
                .leases
                .get(source)
                .map_or(Some(1), |l| l.fence.checked_add(1))
                .filter(|f| *f <= i64::MAX as u64)
                .ok_or_else(|| error("lease fence exhausted"))?;
            let lease = Lease {
                source_id: source.into(),
                owner: owner.into(),
                fence,
                expires_at_ms: now + ttl_ms,
            };
            state.leases.insert(source.into(), lease.clone());
            Ok(lease)
        })
    }
    fn commit<'a>(
        &'a self,
        batch: &'a SourceBatch,
        lease: &'a Lease,
    ) -> StoreFuture<'a, BatchReceipt> {
        Box::pin(async move {
            batch.validate()?;
            if batch.checkpoint.source_id != lease.source_id {
                return Err(error("lease source mismatch"));
            }
            let json = serde_json::to_string(batch).map_err(|_| error("invalid batch JSON"))?;
            let mut state = self.inner.lock().map_err(|_| error("store poisoned"))?;
            let current = state.checkpoints.get(&lease.source_id);
            if let Some((checkpoint, old_json)) = current {
                if checkpoint.generation == batch.checkpoint.generation && old_json == &json {
                    return Ok(BatchReceipt {
                        generation: checkpoint.generation,
                        replayed: true,
                    });
                }
            }
            if current.map_or(0, |(c, _)| c.generation) != batch.expected_generation {
                return Err(error("checkpoint compare-and-swap failed"));
            }
            let live = state
                .leases
                .get(&lease.source_id)
                .ok_or_else(|| error("lease missing"))?;
            if live.owner != lease.owner
                || live.fence != lease.fence
                || live.expires_at_ms <= now_ms()?
            {
                return Err(error("stale lease"));
            }
            // Stage all mutations; an error must not advance either records OR checkpoint.
            let mut staged = state.records.clone();
            for record in &batch.records {
                if staged
                    .apply(record.clone())
                    .map_err(|e| error(&e.to_string()))?
                    == ApplyOutcome::IgnoredStale
                {
                    return Err(error("stale batch record"));
                }
            }
            state.records = staged;
            state
                .checkpoints
                .insert(lease.source_id.clone(), (batch.checkpoint.clone(), json));
            state
                .leases
                .get_mut(&lease.source_id)
                .ok_or_else(|| error("lease missing"))?
                .expires_at_ms = 0;
            Ok(BatchReceipt {
                generation: batch.checkpoint.generation,
                replayed: false,
            })
        })
    }
    fn publish<'a>(&'a self, release: &'a CorpusRelease) -> StoreFuture<'a, ()> {
        Box::pin(async move {
            validate_release(release)?;
            let mut state = self.inner.lock().map_err(|_| error("store poisoned"))?;
            if let Some(old) = state.releases.get(&release.release_id) {
                return if old == release {
                    Ok(())
                } else {
                    Err(error("immutable release conflict"))
                };
            }
            for (source, pins) in &release.source_revisions {
                for (logical, revision) in pins {
                    if !state.records.history.contains_key(&(
                        source.clone(),
                        logical.clone(),
                        *revision,
                    )) {
                        return Err(error("release references missing revision"));
                    }
                }
            }
            state
                .releases
                .insert(release.release_id.clone(), release.clone());
            Ok(())
        })
    }
}
impl SnapshotReadPort for MemoryRepository {
    fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
        let state = self.inner.lock().map_err(|_| error("store poisoned"))?;
        let release = state
            .releases
            .get(release_id)
            .ok_or_else(|| error("unknown release"))?
            .clone();
        let mut revisions = Vec::new();
        let mut heads = Vec::new();
        for (source, pins) in &release.source_revisions {
            for (logical, revision) in pins {
                revisions.push(
                    state
                        .records
                        .history
                        .get(&(source.clone(), logical.clone(), *revision))
                        .ok_or_else(|| error("missing revision"))?
                        .clone(),
                );
                heads.push(
                    state
                        .records
                        .head(source, logical)
                        .ok_or_else(|| error("missing current ACL"))?
                        .clone(),
                );
            }
        }
        Ok(CorpusSnapshot {
            release,
            revisions,
            heads,
        })
    }
}
pub(crate) fn validate_release(release: &CorpusRelease) -> Result<(), PortError> {
    if [
        &release.release_id,
        &release.knowledge_version,
        &release.patch,
    ]
    .iter()
    .any(|s| s.trim().is_empty() || s.len() > 512)
        || release.release_id == "current"
        || release
            .source_revisions
            .values()
            .map(BTreeMap::len)
            .sum::<usize>()
            > 10000
        || release
            .source_revisions
            .values()
            .flat_map(BTreeMap::values)
            .any(|r| *r == 0 || *r > i64::MAX as u64)
    {
        return Err(error("invalid immutable release"));
    }
    Ok(())
}
