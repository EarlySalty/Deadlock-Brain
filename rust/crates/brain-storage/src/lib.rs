#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use brain_contracts::{ContractError, CorpusRelease, SourceRecordV2};
use sqlx::{PgPool, Row};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Contract(#[from] ContractError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("Revision {revision} fuer {source_id}/{logical_id} kollidiert mit anderem Inhalt")]
    RevisionConflict {
        source_id: String,
        logical_id: String,
        revision: u64,
    },
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    Inserted,
    Updated,
    Unchanged,
    Tombstoned,
    IgnoredStale,
}

pub trait RecordSink {
    fn apply(&mut self, record: SourceRecordV2) -> Result<ApplyOutcome>;
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStore {
    heads: BTreeMap<(String, String), SourceRecordV2>,
    history: BTreeMap<(String, String, u64), SourceRecordV2>,
}

impl MemoryStore {
    pub fn head(&self, source_id: &str, logical_id: &str) -> Option<&SourceRecordV2> {
        self.heads
            .get(&(source_id.to_string(), logical_id.to_string()))
    }

    pub fn history(&self, source_id: &str, logical_id: &str) -> Vec<&SourceRecordV2> {
        self.history
            .iter()
            .filter_map(|((source, logical, _), record)| {
                (source == source_id && logical == logical_id).then_some(record)
            })
            .collect()
    }

    pub fn source_revisions(&self) -> BTreeMap<String, BTreeMap<String, u64>> {
        let mut revisions: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
        for record in self.heads.values() {
            revisions
                .entry(record.source_id.clone())
                .or_default()
                .insert(record.logical_id.clone(), record.revision);
        }
        revisions
    }

    pub fn corpus_release(
        &self,
        release_id: impl Into<String>,
        knowledge_version: impl Into<String>,
        patch: impl Into<String>,
        created_at_epoch: i64,
    ) -> CorpusRelease {
        CorpusRelease {
            release_id: release_id.into(),
            knowledge_version: knowledge_version.into(),
            patch: patch.into(),
            created_at_epoch,
            source_revisions: self.source_revisions(),
        }
    }
}

impl RecordSink for MemoryStore {
    fn apply(&mut self, record: SourceRecordV2) -> Result<ApplyOutcome> {
        record.validate()?;
        let head_key = (record.source_id.clone(), record.logical_id.clone());

        if let Some(current) = self.heads.get(&head_key) {
            if record.revision < current.revision {
                return Ok(ApplyOutcome::IgnoredStale);
            }
            if record.revision == current.revision {
                if equivalent_revision(current, &record) {
                    return Ok(ApplyOutcome::Unchanged);
                }
                return Err(StorageError::RevisionConflict {
                    source_id: record.source_id,
                    logical_id: record.logical_id,
                    revision: record.revision,
                });
            }
        }

        let outcome = if record.tombstone {
            ApplyOutcome::Tombstoned
        } else if self.heads.contains_key(&head_key) {
            ApplyOutcome::Updated
        } else {
            ApplyOutcome::Inserted
        };

        self.history.insert(
            (
                record.source_id.clone(),
                record.logical_id.clone(),
                record.revision,
            ),
            record.clone(),
        );
        self.heads.insert(head_key, record);
        Ok(outcome)
    }
}

fn equivalent_revision(left: &SourceRecordV2, right: &SourceRecordV2) -> bool {
    left.content_hash == right.content_hash
        && left.tombstone == right.tombstone
        && left.visibility == right.visibility
        && left.allowed_scopes == right.allowed_scopes
        && left.valid_from == right.valid_from
        && left.valid_to == right.valid_to
        && left.content == right.content
        && left.metadata == right.metadata
}

#[derive(Debug, Clone)]
pub struct PgStore {
    pool: PgPool,
}

impl PgStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply(&self, record: &SourceRecordV2) -> Result<ApplyOutcome> {
        record.validate()?;
        let mut tx = self.pool.begin().await?;
        let lock_key = format!("{}\u{1f}{}", record.source_id, record.logical_id);
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(lock_key)
            .execute(&mut *tx)
            .await?;

        let current = sqlx::query(
            "SELECT revision, content_hash, tombstone, record_json
             FROM brain.source_record_heads
             WHERE source_id = $1 AND logical_id = $2",
        )
        .bind(&record.source_id)
        .bind(&record.logical_id)
        .fetch_optional(&mut *tx)
        .await?;

        let had_head = current.is_some();
        if let Some(ref row) = current {
            let revision: i64 = row.try_get("revision")?;
            let current_revision = revision as u64;
            if record.revision < current_revision {
                tx.commit().await?;
                return Ok(ApplyOutcome::IgnoredStale);
            }
            if record.revision == current_revision {
                let current_json: serde_json::Value = row.try_get("record_json")?;
                let current_record: SourceRecordV2 = serde_json::from_value(current_json)?;
                if equivalent_revision(&current_record, record) {
                    tx.commit().await?;
                    return Ok(ApplyOutcome::Unchanged);
                }
                return Err(StorageError::RevisionConflict {
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                    revision: record.revision,
                });
            }
        }

        let record_json = serde_json::to_value(record)?;
        let inserted_revision = sqlx::query(
            "INSERT INTO brain.source_record_revisions
                (source_id, logical_id, revision, content_hash, tombstone, record_json)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (source_id, logical_id, revision) DO NOTHING",
        )
        .bind(&record.source_id)
        .bind(&record.logical_id)
        .bind(record.revision as i64)
        .bind(&record.content_hash)
        .bind(record.tombstone)
        .bind(record_json.clone())
        .execute(&mut *tx)
        .await?;

        if inserted_revision.rows_affected() == 0 {
            let existing_json: serde_json::Value = sqlx::query_scalar(
                "SELECT record_json
                 FROM brain.source_record_revisions
                 WHERE source_id = $1 AND logical_id = $2 AND revision = $3",
            )
            .bind(&record.source_id)
            .bind(&record.logical_id)
            .bind(record.revision as i64)
            .fetch_one(&mut *tx)
            .await?;
            let existing_record: SourceRecordV2 = serde_json::from_value(existing_json)?;
            if !equivalent_revision(&existing_record, record) {
                return Err(StorageError::RevisionConflict {
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                    revision: record.revision,
                });
            }
        }

        sqlx::query(
            "INSERT INTO brain.source_record_heads
                (source_id, logical_id, revision, content_hash, tombstone, record_json, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, now())
             ON CONFLICT (source_id, logical_id) DO UPDATE SET
                revision = EXCLUDED.revision,
                content_hash = EXCLUDED.content_hash,
                tombstone = EXCLUDED.tombstone,
                record_json = EXCLUDED.record_json,
                updated_at = now()",
        )
        .bind(&record.source_id)
        .bind(&record.logical_id)
        .bind(record.revision as i64)
        .bind(&record.content_hash)
        .bind(record.tombstone)
        .bind(record_json)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(if record.tombstone {
            ApplyOutcome::Tombstoned
        } else if had_head {
            ApplyOutcome::Updated
        } else {
            ApplyOutcome::Inserted
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;
    use brain_contracts::SourceVisibility;

    fn record(revision: u64, hash: &str, tombstone: bool) -> SourceRecordV2 {
        SourceRecordV2 {
            source_id: "fixture".into(),
            logical_id: "hero/abrams".into(),
            revision,
            content_hash: hash.into(),
            content: if tombstone {
                String::new()
            } else {
                hash.into()
            },
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        }
    }

    #[test]
    fn stale_update_cannot_resurrect_tombstone() {
        let mut store = MemoryStore::default();
        assert_eq!(
            store.apply(record(1, "v1", false)).unwrap(),
            ApplyOutcome::Inserted
        );
        assert_eq!(
            store.apply(record(2, "deleted", true)).unwrap(),
            ApplyOutcome::Tombstoned
        );
        assert_eq!(
            store.apply(record(1, "late-v1", false)).unwrap(),
            ApplyOutcome::IgnoredStale
        );
        assert!(store.head("fixture", "hero/abrams").unwrap().tombstone);
        assert_eq!(store.history("fixture", "hero/abrams").len(), 2);
    }

    #[test]
    fn duplicate_revision_is_idempotent_but_conflict_is_rejected() {
        let mut store = MemoryStore::default();
        let first = record(1, "v1", false);
        assert_eq!(store.apply(first.clone()).unwrap(), ApplyOutcome::Inserted);
        assert_eq!(store.apply(first).unwrap(), ApplyOutcome::Unchanged);

        let error = store.apply(record(1, "different", false)).unwrap_err();
        assert!(matches!(error, StorageError::RevisionConflict { .. }));
    }

    #[test]
    fn release_pins_each_logical_revision() {
        let mut store = MemoryStore::default();
        store.apply(record(1, "v1", false)).unwrap();
        store.apply(record(4, "v4", false)).unwrap();

        let mut second = record(2, "item-v2", false);
        second.logical_id = "item/headshot-booster".into();
        store.apply(second).unwrap();

        let release = store.corpus_release("r1", "k1", "2026-09-24", 1);
        let fixture = release.source_revisions.get("fixture").unwrap();
        assert_eq!(fixture.get("hero/abrams"), Some(&4));
        assert_eq!(fixture.get("item/headshot-booster"), Some(&2));
    }
}
