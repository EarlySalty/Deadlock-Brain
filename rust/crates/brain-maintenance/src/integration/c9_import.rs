//! Geprüfte C9-Pakete erhalten ein eigenes unveränderliches Release ohne Aktivierung.
use crate::{digest, integration::runtime_config::read_bounded};
use anyhow::{ensure, Result};
use brain_contracts::{
    source::SourceRevision, CorpusRelease, CorpusSnapshot, DocumentStorePort, SourceBatch,
    SourceVisibility,
};
use brain_ingestion::document_set::{
    current_pins, prepare_document_batch, CoreDocument, DocumentSetSource,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub(crate) struct Package {
    pub source: DocumentSetSource,
    pub documents: Vec<CoreDocument>,
    pub release: CorpusRelease,
}

impl Package {
    fn validate_replay(&self, snapshot: CorpusSnapshot, expected: &SourceBatch) -> Result<()> {
        let mut actual = snapshot.revisions;
        actual.sort_by(|a, b| a.logical_id.cmp(&b.logical_id));
        let mut expected_release = self.release.clone();
        expected_release.created_at_epoch = snapshot.release.created_at_epoch;
        ensure!(
            snapshot.release == expected_release && actual == expected.records,
            "c9_replay_mismatch"
        );
        Ok(())
    }
}

pub(crate) fn prepare(bytes: &[u8], expected: &str, kind: &str) -> Result<Package> {
    ensure!(
        expected.len() == 64
            && expected.bytes().all(|c| c.is_ascii_hexdigit())
            && digest(bytes) == expected,
        "c9_package_hash"
    );
    let (source_id, scope, visibility, prefix) = match kind {
        "docs" => (
            "docs-c9-public:Deadlock-Docs",
            "docs.public",
            SourceVisibility::Public,
            "c9-docs",
        ),
        "second-brain" => (
            "second-brain-c9:Deadlock-2nd-Brain",
            "second_brain.internal",
            SourceVisibility::Internal,
            "c9-second-brain",
        ),
        _ => anyhow::bail!("c9_package_kind"),
    };
    let documents: Vec<CoreDocument> =
        serde_json::from_slice(bytes).map_err(|_| anyhow::anyhow!("c9_package_schema"))?;
    ensure!(
        !documents.is_empty() && documents.len() <= 100,
        "c9_package_count"
    );
    let scopes = BTreeSet::from([scope.to_owned()]);
    if kind == "second-brain" {
        ensure!(
            documents.len() == 2
                && documents
                    .iter()
                    .map(|document| document.logical_id.as_str())
                    .collect::<BTreeSet<_>>()
                    == BTreeSet::from(["systeme/deadlock-bots.md", "projekte/brain-feeder.md"]),
            "c9_internal_complete_source_set"
        );
    }
    for document in &documents {
        let origin = &document.origin;
        origin
            .validate()
            .map_err(|_| anyhow::anyhow!("c9_origin_invalid"))?;
        ensure!(
            origin.identity.source_id == source_id
                && origin.identity.logical_id == document.logical_id
                && origin.raw_sha256 == digest(document.content.as_bytes())
                && matches!(origin.source_revision, SourceRevision::Git { .. })
                && origin.policy.visibility == visibility
                && origin.policy.allowed_scopes == scopes
                && origin.policy.publication_allowed == (kind == "docs")
                && origin.policy.provider_egress_allowed == (kind == "docs")
                && !origin.policy.raw_retention_allowed,
            "c9_package_policy"
        );
        if kind == "docs" {
            ensure!(
                document
                    .metadata
                    .get("content_format")
                    .is_some_and(|value| value == "html"),
                "c9_html_required"
            );
            let projection = dbrain_retrieval::html_projection::project_html(&document.content)?;
            let mut expected_metadata = BTreeMap::new();
            projection.bind_metadata(&mut expected_metadata);
            ensure!(
                expected_metadata
                    .iter()
                    .all(|(key, value)| document.metadata.get(key) == Some(value)),
                "c9_html_projection"
            );
        } else {
            // Der freigegebene interne Bereich enthält ausschließlich die beiden Betriebsseiten.
            ensure!(
                ["systeme/deadlock-bots.md", "projekte/brain-feeder.md"]
                    .contains(&document.logical_id.as_str()),
                "c9_internal_source_area"
            );
        }
    }
    let source = DocumentSetSource {
        source_id: source_id.into(),
        configuration: format!("{prefix}:{expected}"),
        visibility,
        allowed_scopes: scopes,
        tombstone_metadata: BTreeMap::new(),
    };
    let batch = prepare_document_batch(&source, &documents, None)?;
    let release = CorpusRelease {
        release_id: format!("{prefix}-{expected}"),
        knowledge_version: format!("{prefix}-v1-{expected}"),
        patch: "operator-documentation".into(),
        created_at_epoch: chrono::Utc::now().timestamp(),
        source_revisions: BTreeMap::from([(
            source.source_id.clone(),
            current_pins(&batch.checkpoint)?,
        )]),
    };
    Ok(Package {
        source,
        documents,
        release,
    })
}

impl super::runner::Runner {
    pub async fn import_c9_release(
        &self,
        path: &Path,
        expected: &str,
        kind: &str,
    ) -> Result<serde_json::Value> {
        self.require_operator()?;
        let package = prepare(&read_bounded(path, 16 * 1024 * 1024)?, expected, kind)?;
        let previous = self.store.checkpoint(&package.source.source_id).await?;
        let expected_batch = prepare_document_batch(&package.source, &package.documents, None)?;
        if let Some(previous) = previous {
            ensure!(
                previous.configuration == package.source.configuration,
                "c9_source_already_bound"
            );
            let snapshot = self.store.snapshot(&package.release.release_id).await?;
            package.validate_replay(snapshot, &expected_batch)?;
        } else {
            let lease = self
                .store
                .claim(&package.source.source_id, &self.owner, 60_000)
                .await?;
            self.store
                .commit_batches_and_publish_checked(
                    &[(&expected_batch, &lease)],
                    &package.release,
                    &expected_batch.records,
                )
                .await?;
        }
        Ok(
            serde_json::json!({"status":"imported_not_activated", "release_id":package.release.release_id,
            "knowledge_version":package.release.knowledge_version,"package_sha256":expected,
            "documents":package.documents.len(), "scope":package.source.allowed_scopes}),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy},
        value::{Observed, UnknownReason},
    };
    fn document() -> CoreDocument {
        let unknown = || Observed::unknown(UnknownReason::NotPresent);
        let content = "Synthetischer Betriebsstand ohne echte interne Inhalte".to_owned();
        CoreDocument {
            logical_id: "systeme/deadlock-bots.md".into(),
            content: content.clone(),
            metadata: BTreeMap::new(),
            origin: OriginArtifact {
                identity: SourceIdentity {
                    source_id: "second-brain-c9:Deadlock-2nd-Brain".into(),
                    logical_id: "systeme/deadlock-bots.md".into(),
                },
                source_revision: SourceRevision::Git {
                    commit: "a".repeat(40),
                },
                raw_sha256: digest(content.as_bytes()),
                locator: "fixture:document".into(),
                parser_revision: "fixture-v1".into(),
                parser_family: "fixture".into(),
                schema_version: unknown(),
                schema_sha256: unknown(),
                retrieved_at: Observed::unknown(UnknownReason::NotPresent),
                source_time: Observed::unknown(UnknownReason::NotPresent),
                language: unknown(),
                origin_artifacts: BTreeSet::new(),
                derivation_family: unknown(),
                policy: SourcePolicy {
                    visibility: SourceVisibility::Internal,
                    allowed_scopes: BTreeSet::from(["second_brain.internal".into()]),
                    authorization_ref: Observed::known("fixture-authorization".into()),
                    license: unknown(),
                    publication_allowed: false,
                    provider_egress_allowed: false,
                    raw_retention_allowed: false,
                },
                validity: GameValidity::unknown(),
            },
        }
    }
    fn fixture_documents(candidate: &CoreDocument) -> Vec<CoreDocument> {
        let mut second = document();
        second.logical_id = "projekte/brain-feeder.md".into();
        second.origin.identity.logical_id = second.logical_id.clone();
        vec![candidate.clone(), second]
    }
    fn check(document: &CoreDocument, kind: &str) -> bool {
        let bytes = serde_json::to_vec(&fixture_documents(document)).unwrap();
        prepare(&bytes, &digest(&bytes), kind).is_ok()
    }
    #[test]
    fn unsortiertes_paket_bleibt_beim_replay_identisch_inhalte_und_release_sind_gebunden() {
        let documents = fixture_documents(&document());
        assert!(documents[0].logical_id > documents[1].logical_id);
        let bytes = serde_json::to_vec(&documents).unwrap();
        let package = prepare(&bytes, &digest(&bytes), "second-brain").unwrap();
        let batch = prepare_document_batch(&package.source, &package.documents, None).unwrap();
        assert_eq!(
            batch
                .records
                .iter()
                .map(|r| r.logical_id.as_str())
                .collect::<Vec<_>>(),
            ["projekte/brain-feeder.md", "systeme/deadlock-bots.md"]
        );
        let mut stored_release = package.release.clone();
        stored_release.created_at_epoch = 123;
        let mut stored_records = batch.records.clone();
        stored_records.reverse();
        let stored = CorpusSnapshot {
            release: stored_release,
            revisions: stored_records,
            heads: batch.records.clone(),
        };
        package.validate_replay(stored.clone(), &batch).unwrap();
        let mut changed = stored.clone();
        changed.revisions[0].content.push_str("Veränderte Fixture");
        assert!(package.validate_replay(changed, &batch).is_err());
        let mut changed = stored;
        changed.release.release_id = "fremdes-release".into();
        assert!(package.validate_replay(changed, &batch).is_err());
    }
    #[test]
    fn interne_importrechte_werden_erhalten_und_niemals_hochgestuft() {
        let document = document();
        assert!(check(&document, "second-brain"));
        assert!(!check(&document, "docs"));
        for field in ["scope", "source", "path", "publication", "egress", "hash"] {
            let mut changed = document.clone();
            match field {
                "scope" => {
                    changed
                        .origin
                        .policy
                        .allowed_scopes
                        .insert("docs.public".into());
                }
                "source" => changed.origin.identity.source_id = "foreign-source".into(),
                "path" => {
                    changed.logical_id = "raw/private.md".into();
                    changed.origin.identity.logical_id = changed.logical_id.clone();
                }
                "publication" => changed.origin.policy.publication_allowed = true,
                "egress" => changed.origin.policy.provider_egress_allowed = true,
                "hash" => changed.content.push_str("changed"),
                _ => unreachable!(),
            }
            assert!(!check(&changed, "second-brain"));
        }
        let incomplete = serde_json::to_vec(&vec![document.clone()]).unwrap();
        assert!(prepare(&incomplete, &digest(&incomplete), "second-brain").is_err());
        let bytes = serde_json::to_vec(&fixture_documents(&document)).unwrap();
        assert!(prepare(&bytes, &"0".repeat(64), "second-brain").is_err());
        let package = prepare(&bytes, &digest(&bytes), "second-brain").unwrap();
        let batch = prepare_document_batch(&package.source, &package.documents, None).unwrap();
        assert_eq!(
            batch.records[0].allowed_scopes,
            BTreeSet::from(["second_brain.internal".into()])
        );
        assert!(
            !brain_contracts::source::origin_from_record(&batch.records[0])
                .unwrap()
                .policy
                .publication_allowed
        );
        assert_eq!(package.release.source_revisions.len(), 1);
    }
}
