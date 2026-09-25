use super::{FileCheckpoint, FileConnector, IngestionError, Result};
use brain_contracts::{BatchReceipt, DocumentStorePort, Lease, SourceBatch, SourceCheckpoint};
impl FileConnector {
    /// Prepare on a feeder worker (filesystem work is synchronous). Nothing is persisted here.
    /// Keep this batch unchanged for retry after an ambiguous commit acknowledgement.
    pub fn prepare_batch(&self, previous: Option<&SourceCheckpoint>) -> Result<SourceBatch> {
        let generation = previous.map_or(0, |c| c.generation);
        let checkpoint = match previous {
            Some(stored) if stored.source_id == self.source_id => {
                let cp: FileCheckpoint = serde_json::from_value(stored.state.clone())?;
                if cp.configuration.as_ref() != Some(&stored.configuration) {
                    return Err(IngestionError::InvalidState(
                        "checkpoint configuration mismatch".into(),
                    ));
                }
                cp
            }
            Some(_) => {
                return Err(IngestionError::InvalidState(
                    "checkpoint source mismatch".into(),
                ))
            }
            None => FileCheckpoint::default(),
        };
        let scan = self.scan(&checkpoint)?;
        let configuration = scan
            .checkpoint
            .configuration
            .clone()
            .ok_or_else(|| IngestionError::InvalidState("missing configuration".into()))?;
        let batch = SourceBatch {
            expected_generation: generation,
            checkpoint: SourceCheckpoint {
                source_id: self.source_id.clone(),
                configuration,
                generation: super::next_revision(generation)?,
                state: serde_json::to_value(scan.checkpoint)?,
            },
            records: scan.records,
        };
        batch.validate()?;
        Ok(batch)
    }
    /// The store checks the fencing token and atomically commits every record AND the cursor.
    pub async fn commit_batch(
        &self,
        store: &dyn DocumentStorePort,
        batch: &SourceBatch,
        lease: &Lease,
    ) -> Result<BatchReceipt> {
        if batch.checkpoint.source_id != self.source_id {
            return Err(IngestionError::InvalidState("batch source mismatch".into()));
        }
        Ok(store.commit(batch, lease).await?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::SourceVisibility;
    use brain_storage::MemoryRepository;
    use std::{collections::BTreeSet, fs};
    #[test]
    fn missing_root_and_bad_utf8_never_create_deletions() {
        let root = tempfile::tempdir().unwrap();
        let connector = FileConnector::new(
            root.path(),
            "files",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        fs::write(root.path().join("a.md"), "one").unwrap();
        let scan = connector.scan(&FileCheckpoint::default()).unwrap();
        fs::write(root.path().join("a.md"), [0xff]).unwrap();
        assert!(connector.scan(&scan.checkpoint).is_err());
        fs::remove_dir_all(root.path()).unwrap();
        assert!(connector.scan(&scan.checkpoint).is_err());
    }
    #[test]
    fn changed_acl_revises_unchanged_content_and_rejects_other_source() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("a.md"), "same").unwrap();
        let public = FileConnector::new(
            root.path(),
            "files",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        let first = public.scan(&FileCheckpoint::default()).unwrap();
        let private = FileConnector::new(
            root.path(),
            "files",
            SourceVisibility::Private,
            BTreeSet::from(["secret".into()]),
        );
        let second = private.scan(&first.checkpoint).unwrap();
        assert_eq!(second.records[0].revision, 2);
        assert_eq!(second.records[0].content, "same");
        assert_eq!(second.records[0].visibility, SourceVisibility::Private);
        let other = FileConnector::new(
            root.path(),
            "other",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        assert!(other.scan(&second.checkpoint).is_err());
    }
    #[test]
    fn overflowing_revision_is_an_error_not_a_restart() {
        assert!(super::super::next_revision(i64::MAX as u64).is_err());
        assert!(super::super::next_revision(u64::MAX).is_err());
    }
    #[tokio::test]
    async fn durable_batch_replay_and_restart_preserve_revisions() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("a.md"), "one").unwrap();
        let connector = FileConnector::new(
            root.path(),
            "files",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        let store = MemoryRepository::default();
        let lease = store.claim("files", "fixture", 30000).await.unwrap();
        let prepared = connector.prepare_batch(None).unwrap();
        connector
            .commit_batch(&store, &prepared, &lease)
            .await
            .unwrap();
        assert!(
            connector
                .commit_batch(&store, &prepared, &lease)
                .await
                .unwrap()
                .replayed
        );
        let cp = store.checkpoint("files").await.unwrap().unwrap();
        let restarted = connector.prepare_batch(Some(&cp)).unwrap();
        assert!(restarted.records.is_empty());
        fs::write(root.path().join("a.md"), "two").unwrap();
        let update = connector.prepare_batch(Some(&cp)).unwrap();
        assert_eq!(update.records[0].revision, 2);
        let next_lease = store.claim("files", "restarted", 30000).await.unwrap();
        connector
            .commit_batch(&store, &update, &next_lease)
            .await
            .unwrap();
    }
}
