use crate::{
    sha256_hex,
    source_sync::{origin, SourceRights},
    FeedError, Result,
};
use brain_contracts::{SourceBatch, SourceCheckpoint};
use brain_ingestion::document_set::{
    prepare_document_batch, CoreDocument, DocumentSetSource, MAX_DOCUMENT_BYTES,
};
use dbrain_sources::google_sheet::{parse_complete_csv_rows, sheet_csv_url};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PARSER: &str = "brain-sheet-csv.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetSource {
    pub sheet_id: String,
    pub gids: BTreeSet<String>,
    pub rights: SourceRights,
}

impl SheetSource {
    pub fn validate(&self) -> Result<()> {
        self.rights.validate()?;
        if self.sheet_id.is_empty()
            || !self
                .sheet_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || self.gids.is_empty()
            || self
                .gids
                .iter()
                .any(|gid| gid.is_empty() || !gid.bytes().all(|b| b.is_ascii_digit()))
        {
            return Err(FeedError::Invalid("Ungültige Tabellenidentität.".into()));
        }
        Ok(())
    }

    pub fn source_id(&self) -> String {
        format!("google-sheet/{}", self.sheet_id)
    }
}

#[derive(Debug, Clone)]
pub struct SheetTabBody {
    pub gid: String,
    pub bytes: Vec<u8>,
    pub complete: bool,
}

pub fn prepare_batch(
    source: &SheetSource,
    tabs: &[SheetTabBody],
    observed_at: i64,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    source.validate()?;
    let gids: BTreeSet<_> = tabs.iter().map(|tab| tab.gid.clone()).collect();
    if tabs.len() != gids.len() || gids != source.gids || observed_at <= 0 {
        return Err(FeedError::Invalid(
            "Tabellenabruf ist nicht vollständig.".into(),
        ));
    }
    let source_id = source.source_id();
    let mut documents = Vec::new();
    for tab in tabs {
        if !tab.complete || tab.bytes.len() > MAX_DOCUMENT_BYTES {
            return Err(FeedError::Invalid(
                "Tabellenabruf ist unvollständig oder zu groß.".into(),
            ));
        }
        let text = std::str::from_utf8(&tab.bytes)
            .map_err(|_| FeedError::Invalid("CSV ist nicht UTF-8.".into()))?;
        let rows = parse_complete_csv_rows(text).map_err(|e| FeedError::Invalid(e.into()))?;
        if rows.is_empty()
            || !rows.iter().flatten().any(|cell| !cell.trim().is_empty())
            || text.trim_start().starts_with('<')
        {
            return Err(FeedError::Invalid(
                "Tabellenantwort enthält kein CSV.".into(),
            ));
        }
        let logical = format!("tab/{}", tab.gid);
        let url = sheet_csv_url(&source.sheet_id, &tab.gid);
        let hash = sha256_hex(&tab.bytes);
        documents.push(CoreDocument {
            logical_id: logical.clone(),
            content: text.into(),
            metadata: BTreeMap::from([
                ("connector".into(), "google-sheet".into()),
                ("kind".into(), "prose".into()),
                ("locator".into(), url.clone()),
                ("sheet_id".into(), source.sheet_id.clone()),
                ("gid".into(), tab.gid.clone()),
                ("raw_sha256".into(), hash.clone()),
                ("rows".into(), rows.len().to_string()),
            ]),
            origin: origin(
                &source_id,
                &logical,
                &url,
                PARSER,
                &hash,
                observed_at,
                &source.rights,
            ),
        });
    }
    let set = DocumentSetSource {
        source_id,
        configuration: sha256_hex(&serde_json::to_vec(&(PARSER, source))?),
        visibility: source.rights.policy.visibility,
        allowed_scopes: source.rights.policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([("connector".into(), "google-sheet".into())]),
    };
    Ok(prepare_document_batch(&set, &documents, previous)?)
}
