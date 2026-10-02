use super::{digest, next_revision, IngestionError, Result};
use brain_contracts::{
    source::OriginArtifact, SourceBatch, SourceCheckpoint, SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_DOCUMENTS_PER_SOURCE: usize = 5_000;
pub const MAX_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoreDocument {
    pub logical_id: String,
    pub content: String,
    pub metadata: BTreeMap<String, String>,
    pub origin: OriginArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSetSource {
    pub source_id: String,
    pub configuration: String,
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub tombstone_metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentState {
    pub revision: u64,
    pub content_hash: String,
    /// Semantic provenance and current rights; absent on legacy checkpoints forces one refresh.
    #[serde(default)]
    pub semantic_hash: Option<String>,
    pub tombstone: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSetCheckpoint {
    pub configuration: String,
    pub documents: BTreeMap<String, DocumentState>,
}

fn invalid(message: &str) -> IngestionError {
    IngestionError::InvalidState(message.into())
}

fn record(
    source: &DocumentSetSource,
    document: &CoreDocument,
    revision: u64,
) -> Result<SourceRecordV2> {
    let content_hash = digest(document.content.as_bytes());
    let mut record = SourceRecordV2 {
        source_id: source.source_id.clone(),
        logical_id: document.logical_id.clone(),
        revision,
        content_hash: content_hash.clone(),
        content: document.content.clone(),
        visibility: source.visibility,
        allowed_scopes: source.allowed_scopes.clone(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: document.metadata.clone(),
    };
    let mut origin = document.origin.clone();
    origin.identity.source_id = source.source_id.clone();
    origin.identity.logical_id = document.logical_id.clone();
    origin.raw_sha256 = content_hash;
    origin.policy.visibility = source.visibility;
    origin.policy.allowed_scopes = source.allowed_scopes.clone();
    origin
        .bind_record(&mut record)
        .map_err(IngestionError::InvalidState)?;
    Ok(record)
}

fn semantic_hash(source: &DocumentSetSource, document: &CoreDocument) -> Result<String> {
    let mut stable = document.clone();
    // A new observation of the same immutable artifact is not a new source revision.
    stable.origin.retrieved_at = brain_contracts::value::Observed::unknown(
        brain_contracts::value::UnknownReason::NotPresent,
    );
    let normalized = record(source, &stable, 1)?;
    Ok(digest(&serde_json::to_vec(&normalized)?))
}

pub fn prepare_document_batch(
    source: &DocumentSetSource,
    documents: &[CoreDocument],
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    if source.source_id.trim().is_empty() || source.configuration.trim().is_empty() {
        return Err(invalid("source identity required"));
    }
    if source.visibility != SourceVisibility::Public && source.allowed_scopes.is_empty() {
        return Err(invalid("non-public source requires explicit scopes"));
    }
    if documents.is_empty() {
        return Err(invalid("empty read never tombstones an existing source"));
    }
    if documents.len() > MAX_DOCUMENTS_PER_SOURCE
        || documents
            .iter()
            .any(|d| d.content.len() > MAX_DOCUMENT_BYTES)
    {
        return Err(invalid("document count or size limit"));
    }
    let (generation, state) = match previous {
        None => (0, DocumentSetCheckpoint::default()),
        Some(stored) if stored.source_id == source.source_id => {
            let state: DocumentSetCheckpoint = serde_json::from_value(stored.state.clone())?;
            if state.configuration != stored.configuration {
                return Err(invalid("checkpoint configuration mismatch"));
            }
            (stored.generation, state)
        }
        Some(_) => return Err(invalid("checkpoint source mismatch")),
    };
    let same_configuration = state.configuration == source.configuration;
    let mut next = DocumentSetCheckpoint {
        configuration: source.configuration.clone(),
        documents: state.documents.clone(),
    };
    let mut records = Vec::new();
    let mut seen = BTreeSet::new();
    for document in documents {
        if !seen.insert(document.logical_id.as_str()) {
            return Err(invalid("duplicate logical id"));
        }
        let hash = digest(document.content.as_bytes());
        let semantics = semantic_hash(source, document)?;
        let previous_state = state.documents.get(&document.logical_id);
        if same_configuration
            && previous_state.is_some_and(|s| {
                !s.tombstone
                    && s.content_hash == hash
                    && s.semantic_hash.as_ref() == Some(&semantics)
            })
        {
            continue;
        }
        let revision = next_revision(previous_state.map_or(0, |s| s.revision))?;
        let record = record(source, document, revision)?;
        next.documents.insert(
            document.logical_id.clone(),
            DocumentState {
                revision,
                content_hash: hash,
                semantic_hash: Some(semantics),
                tombstone: false,
            },
        );
        records.push(record);
    }
    for (logical_id, previous_state) in &state.documents {
        if seen.contains(logical_id.as_str()) || previous_state.tombstone {
            continue;
        }
        let revision = next_revision(previous_state.revision)?;
        let content_hash = digest(format!("tombstone:{logical_id}:{revision}").as_bytes());
        records.push(SourceRecordV2 {
            source_id: source.source_id.clone(),
            logical_id: logical_id.clone(),
            revision,
            content_hash: content_hash.clone(),
            content: String::new(),
            visibility: source.visibility,
            allowed_scopes: source.allowed_scopes.clone(),
            tombstone: true,
            valid_from: None,
            valid_to: None,
            metadata: source.tombstone_metadata.clone(),
        });
        next.documents.insert(
            logical_id.clone(),
            DocumentState {
                revision,
                content_hash,
                semantic_hash: None,
                tombstone: true,
            },
        );
    }
    records.sort_by(|a, b| a.logical_id.cmp(&b.logical_id));
    let batch = SourceBatch {
        expected_generation: generation,
        checkpoint: SourceCheckpoint {
            source_id: source.source_id.clone(),
            configuration: source.configuration.clone(),
            generation: next_revision(generation)?,
            state: serde_json::to_value(&next)?,
        },
        records,
    };
    batch.validate()?;
    Ok(batch)
}

pub fn current_pins(checkpoint: &SourceCheckpoint) -> Result<BTreeMap<String, u64>> {
    let state: DocumentSetCheckpoint = serde_json::from_value(checkpoint.state.clone())?;
    if state.configuration != checkpoint.configuration {
        return Err(invalid("checkpoint configuration mismatch"));
    }
    Ok(state
        .documents
        .into_iter()
        .filter(|(_, s)| !s.tombstone)
        .map(|(id, s)| (id, s.revision))
        .collect())
}

#[cfg(test)]
mod semantic_tests {
    use super::*;
    use brain_contracts::{
        source::{GameValidity, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp},
        value::{Observed, UnknownReason},
    };

    fn fixture() -> (DocumentSetSource, CoreDocument) {
        let source = DocumentSetSource {
            source_id: "fixture".into(),
            configuration: "unchanged".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["game.public".into()]),
            tombstone_metadata: BTreeMap::new(),
        };
        let unknown = || Observed::unknown(UnknownReason::NotPresent);
        let document = CoreDocument {
            logical_id: "one".into(),
            content: "same bytes".into(),
            metadata: BTreeMap::new(),
            origin: OriginArtifact {
                identity: SourceIdentity {
                    source_id: source.source_id.clone(),
                    logical_id: "one".into(),
                },
                source_revision: SourceRevision::Git {
                    commit: "a".repeat(40),
                },
                raw_sha256: digest(b"same bytes"),
                locator: "file:one".into(),
                parser_revision: "1".into(),
                parser_family: "fixture".into(),
                schema_version: unknown(),
                schema_sha256: unknown(),
                retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(1)),
                source_time: Observed::unknown(UnknownReason::NotPresent),
                language: unknown(),
                origin_artifacts: BTreeSet::new(),
                derivation_family: unknown(),
                policy: SourcePolicy {
                    visibility: source.visibility,
                    allowed_scopes: source.allowed_scopes.clone(),
                    authorization_ref: unknown(),
                    license: unknown(),
                    publication_allowed: true,
                    provider_egress_allowed: true,
                    raw_retention_allowed: true,
                },
                validity: GameValidity::unknown(),
            },
        };
        (source, document)
    }

    #[test]
    fn unchanged_bytes_preserve_semantic_updates_and_ignore_only_read_time() {
        let (source, document) = fixture();
        let initial =
            prepare_document_batch(&source, std::slice::from_ref(&document), None).unwrap();
        let previous = Some(&initial.checkpoint);
        let mut observed = document.clone();
        observed.origin.retrieved_at = Observed::known(SourceTimestamp::UnixSeconds(2));
        assert!(prepare_document_batch(&source, &[observed], previous)
            .unwrap()
            .records
            .is_empty());
        for field in ["revision", "source_time", "language", "policy", "metadata"] {
            let mut changed = document.clone();
            match field {
                "revision" => {
                    changed.origin.source_revision = SourceRevision::Git {
                        commit: "b".repeat(40),
                    }
                }
                "source_time" => {
                    changed.origin.source_time = Observed::known(SourceTimestamp::UnixSeconds(2))
                }
                "language" => changed.origin.language = Observed::known("de".into()),
                "policy" => changed.origin.policy.provider_egress_allowed = false,
                "metadata" => {
                    changed.metadata.insert("published_at".into(), "2".into());
                }
                _ => unreachable!(),
            }
            let refreshed = prepare_document_batch(&source, &[changed], previous).unwrap();
            assert_eq!(refreshed.records.len(), 1, "{field}");
            assert_eq!(refreshed.records[0].revision, 2);
        }
        let mut restricted = source.clone();
        restricted.visibility = SourceVisibility::Internal;
        restricted.allowed_scopes = BTreeSet::from(["private".into()]);
        assert_eq!(
            prepare_document_batch(&restricted, std::slice::from_ref(&document), previous)
                .unwrap()
                .records
                .len(),
            1
        );
        let mut legacy = initial.checkpoint;
        legacy.state["documents"]["one"]
            .as_object_mut()
            .unwrap()
            .remove("semantic_hash");
        assert_eq!(
            prepare_document_batch(&source, &[document], Some(&legacy))
                .unwrap()
                .records
                .len(),
            1
        );
    }
}
