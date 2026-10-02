use crate::{config::RepositoryConfig, digest, scanner::ScanResult};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Persistiert nur feste Quellbelege und Dokumenthashes, keine Rohtexte.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanCheckpoint {
    version: String,
    pub scan: ScanResult,
    pub reader_release: String,
    pub document_hashes: BTreeMap<String, Option<String>>,
}
impl ScanCheckpoint {
    pub fn capture(scan: &ScanResult, reader_release: String) -> Self {
        let document_hashes = scan
            .documents
            .iter()
            .map(|(target, text)| {
                (
                    target.clone(),
                    text.as_ref().map(|text| digest(text.as_bytes())),
                )
            })
            .collect();
        let mut scan = scan.clone();
        for blob in &mut scan.source_blobs {
            blob.content.clear();
        }
        for text in scan.documents.values_mut().flatten() {
            text.clear();
        }
        Self {
            version: "scan-reference-v2".into(),
            scan,
            reader_release,
            document_hashes,
        }
    }
    pub fn hydrate_sources(&mut self, repo: &RepositoryConfig) -> Result<()> {
        ensure!(
            self.version == "scan-reference-v2",
            "legacy_raw_scan_blocked"
        );
        let pin =
            dbrain_sources::git_source::PinnedRepository::open(&repo.path, &self.scan.source_sha)?;
        pin.require_origin(&[&repo.origin])?;
        for blob in &mut self.scan.source_blobs {
            ensure!(blob.content.is_empty(), "raw_scan_retention_blocked");
            let bytes = pin.read_blob(&blob.path)?;
            ensure!(
                digest(&bytes) == blob.sha256 && blob.origin.raw_sha256 == blob.sha256,
                "referenced_source_hash_changed"
            );
            blob.content = String::from_utf8(bytes).context("referenced_source_utf8")?;
        }
        ensure!(
            self.scan.documents.keys().eq(self.document_hashes.keys()),
            "scan_document_reference_changed"
        );
        ensure!(
            self.scan.documents.values().flatten().all(String::is_empty),
            "raw_document_retention_blocked"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_document_bytes_are_not_retained_in_scan_checkpoint() {
        let scan = ScanResult {
            repo_id: "owned".into(),
            source_sha: "a".repeat(40),
            deployed_sha: None,
            source_changed: true,
            changed_paths: vec![],
            source_blobs: vec![],
            docs_sha: "b".repeat(40),
            documents: BTreeMap::from([(
                "internal/a.md".into(),
                Some("Privater ursprünglicher Text".into()),
            )]),
            document_paths: BTreeMap::new(),
            source_fingerprint: "c".repeat(64),
            assets: vec![],
        };
        let checkpoint = ScanCheckpoint::capture(&scan, "explicit-reader-release".into());
        let bytes = serde_json::to_string(&checkpoint).unwrap();
        assert!(!bytes.contains("Privater ursprünglicher Text"));
        assert_eq!(
            checkpoint.document_hashes["internal/a.md"],
            Some(digest("Privater ursprünglicher Text".as_bytes()))
        );
    }
}
