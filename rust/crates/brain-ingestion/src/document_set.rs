use super::{digest, next_revision, IngestionError, Result};
use brain_contracts::{
    source::OriginArtifact, SourceBatch, SourceCheckpoint, SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_DOCUMENTS_PER_SOURCE: usize = 5_000;
pub const MAX_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        let previous_state = state.documents.get(&document.logical_id);
        if same_configuration
            && previous_state.is_some_and(|s| !s.tombstone && s.content_hash == hash)
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
