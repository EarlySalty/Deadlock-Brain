#![forbid(unsafe_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use brain_contracts::{SourceRecordV2, SourceVisibility};
use brain_storage::{ApplyOutcome, RecordSink, StorageError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("Datei liegt ausserhalb des Connector Roots: {0}")]
    OutsideRoot(PathBuf),
}

pub type Result<T> = std::result::Result<T, IngestionError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileState {
    pub revision: u64,
    pub content_hash: String,
    pub tombstone: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileCheckpoint {
    #[serde(default)]
    pub files: BTreeMap<String, FileState>,
}

impl FileCheckpoint {
    pub fn from_json(value: &str) -> Result<Self> {
        Ok(serde_json::from_str(value)?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[derive(Debug, Clone)]
pub struct FileConnector {
    root: PathBuf,
    source_id: String,
    visibility: SourceVisibility,
    allowed_scopes: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub records: Vec<SourceRecordV2>,
    pub checkpoint: FileCheckpoint,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApplySummary {
    pub inserted: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub tombstoned: usize,
    pub ignored_stale: usize,
}

impl FileConnector {
    pub fn new(
        root: impl Into<PathBuf>,
        source_id: impl Into<String>,
        visibility: SourceVisibility,
        allowed_scopes: BTreeSet<String>,
    ) -> Self {
        Self {
            root: root.into(),
            source_id: source_id.into(),
            visibility,
            allowed_scopes,
        }
    }

    pub fn scan(&self, previous: &FileCheckpoint) -> Result<ScanResult> {
        let mut paths = Vec::new();
        collect_files(&self.root, &mut paths)?;
        paths.sort();

        let mut next = previous.clone();
        let mut seen = BTreeSet::new();
        let mut records = Vec::new();

        for path in paths {
            let logical_id = self.logical_id(&path)?;
            seen.insert(logical_id.clone());
            let content = fs::read(&path)?;
            let content_hash = digest(&content);
            let previous_state = previous.files.get(&logical_id);

            if previous_state
                .is_some_and(|state| !state.tombstone && state.content_hash == content_hash)
            {
                continue;
            }

            let revision = previous_state.map_or(1, |state| state.revision + 1);
            let record = SourceRecordV2 {
                source_id: self.source_id.clone(),
                logical_id: logical_id.clone(),
                revision,
                content_hash: content_hash.clone(),
                content: String::from_utf8_lossy(&content).into_owned(),
                visibility: self.visibility,
                allowed_scopes: self.allowed_scopes.clone(),
                tombstone: false,
                valid_from: None,
                valid_to: None,
                metadata: BTreeMap::from([("connector".to_string(), "file".to_string())]),
            };
            records.push(record);
            next.files.insert(
                logical_id,
                FileState {
                    revision,
                    content_hash,
                    tombstone: false,
                },
            );
        }

        for (logical_id, state) in &previous.files {
            if seen.contains(logical_id) || state.tombstone {
                continue;
            }
            let revision = state.revision + 1;
            let content_hash = digest(format!("tombstone:{logical_id}:{revision}").as_bytes());
            records.push(SourceRecordV2 {
                source_id: self.source_id.clone(),
                logical_id: logical_id.clone(),
                revision,
                content_hash: content_hash.clone(),
                content: String::new(),
                visibility: self.visibility,
                allowed_scopes: self.allowed_scopes.clone(),
                tombstone: true,
                valid_from: None,
                valid_to: None,
                metadata: BTreeMap::from([("connector".to_string(), "file".to_string())]),
            });
            next.files.insert(
                logical_id.clone(),
                FileState {
                    revision,
                    content_hash,
                    tombstone: true,
                },
            );
        }

        records.sort_by(|left, right| left.logical_id.cmp(&right.logical_id));
        Ok(ScanResult {
            records,
            checkpoint: next,
        })
    }

    pub fn apply_scan(
        &self,
        sink: &mut impl RecordSink,
        scan: &ScanResult,
    ) -> Result<ApplySummary> {
        let mut summary = ApplySummary::default();
        for record in scan.records.iter().cloned() {
            match sink.apply(record)? {
                ApplyOutcome::Inserted => summary.inserted += 1,
                ApplyOutcome::Updated => summary.updated += 1,
                ApplyOutcome::Unchanged => summary.unchanged += 1,
                ApplyOutcome::Tombstoned => summary.tombstoned += 1,
                ApplyOutcome::IgnoredStale => summary.ignored_stale += 1,
            }
        }
        Ok(summary)
    }

    fn logical_id(&self, path: &Path) -> Result<String> {
        let relative = path
            .strip_prefix(&self.root)
            .map_err(|_| IngestionError::OutsideRoot(path.to_path_buf()))?;
        Ok(relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/"))
    }
}

fn collect_files(root: &Path, output: &mut Vec<PathBuf>) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            collect_files(&entry.path(), output)?;
        } else if file_type.is_file() {
            output.push(entry.path());
        }
    }
    Ok(())
}

fn digest(content: &[u8]) -> String {
    hex::encode(Sha256::digest(content))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use brain_storage::MemoryStore;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn update_delete_and_restart_are_deterministic() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join("heroes")).unwrap();
        fs::write(root.path().join("heroes/abrams.md"), "v1").unwrap();

        let connector = FileConnector::new(
            root.path(),
            "docs",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        let mut store = MemoryStore::default();

        let first = connector.scan(&FileCheckpoint::default()).unwrap();
        assert_eq!(first.records.len(), 1);
        assert_eq!(first.records[0].revision, 1);
        assert_eq!(
            connector.apply_scan(&mut store, &first).unwrap().inserted,
            1
        );

        let unchanged = connector.scan(&first.checkpoint).unwrap();
        assert!(unchanged.records.is_empty());

        fs::write(root.path().join("heroes/abrams.md"), "v2").unwrap();
        let updated = connector.scan(&first.checkpoint).unwrap();
        assert_eq!(updated.records[0].revision, 2);
        assert_eq!(
            connector.apply_scan(&mut store, &updated).unwrap().updated,
            1
        );

        fs::remove_file(root.path().join("heroes/abrams.md")).unwrap();
        let deleted = connector.scan(&updated.checkpoint).unwrap();
        assert!(deleted.records[0].tombstone);
        assert_eq!(deleted.records[0].revision, 3);
        assert_eq!(
            connector
                .apply_scan(&mut store, &deleted)
                .unwrap()
                .tombstoned,
            1
        );

        let serialized = deleted.checkpoint.to_json().unwrap();
        let restarted = FileCheckpoint::from_json(&serialized).unwrap();
        let after_restart = connector.scan(&restarted).unwrap();
        assert!(after_restart.records.is_empty());
    }

    #[test]
    fn scan_order_is_stable() {
        let root = tempdir().unwrap();
        fs::write(root.path().join("z.md"), "z").unwrap();
        fs::write(root.path().join("a.md"), "a").unwrap();

        let connector = FileConnector::new(
            root.path(),
            "docs",
            SourceVisibility::Public,
            BTreeSet::new(),
        );
        let scan = connector.scan(&FileCheckpoint::default()).unwrap();
        let ids: Vec<_> = scan
            .records
            .iter()
            .map(|record| record.logical_id.as_str())
            .collect();
        assert_eq!(ids, vec!["a.md", "z.md"]);
    }
}
