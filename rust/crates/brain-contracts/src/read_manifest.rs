//! Vollständige Release- und Rechteansicht ohne Dokumentkörper.
use crate::{CorpusRelease, CorpusSnapshot, DocumentHead, DocumentRevision, PortError, Principal};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentDescriptor {
    pub head: DocumentHead,
    pub content_hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceVisibility;

    fn fixture() -> ReleaseReadManifest {
        let head = DocumentHead {
            source_id: "quelle".into(),
            logical_id: "dokument".into(),
            revision: 1,
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            metadata: BTreeMap::new(),
        };
        ReleaseReadManifest {
            release: CorpusRelease {
                release_id: "release".into(),
                knowledge_version: "wissen".into(),
                patch: "patch".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    "quelle".into(),
                    BTreeMap::from([("dokument".into(), 1)]),
                )]),
            },
            revisions: vec![DocumentDescriptor {
                head: head.clone(),
                content_hash: "a".repeat(64),
            }],
            heads: vec![head],
        }
    }

    #[test]
    fn manifest_checks_all_pins_even_when_access_is_denied() {
        let principal = Principal {
            actor_id: "akteur".into(),
            channel: "kanal".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::new(),
        };
        let mut manifest = fixture();
        assert_eq!(
            manifest.authorized(&principal, false, false).unwrap().len(),
            1
        );
        manifest.heads[0].allowed_scopes.insert("intern".into());
        assert!(manifest
            .authorized(&principal, false, false)
            .unwrap()
            .is_empty());
        manifest.revisions.clear();
        assert!(manifest.authorized(&principal, false, false).is_err());
    }

    #[test]
    fn manifest_rejects_missing_duplicate_old_and_unpinned_heads() {
        let original = fixture();
        for mutation in 0..4 {
            let mut manifest = original.clone();
            match mutation {
                0 => manifest.heads.clear(),
                1 => manifest.heads.push(manifest.heads[0].clone()),
                2 => manifest.revisions[0].head.revision = 2,
                _ => manifest.heads[0].logical_id = "anderes-dokument".into(),
            }
            assert!(manifest.validate().is_err());
        }
    }

    #[test]
    fn versioned_provider_origin_requires_a_current_grant() {
        use crate::{
            source::*,
            value::{Observed, UnknownReason},
        };
        let mut manifest = fixture();
        let descriptor = &mut manifest.revisions[0];
        let origin = OriginArtifact {
            identity: SourceIdentity {
                source_id: descriptor.head.source_id.clone(),
                logical_id: descriptor.head.logical_id.clone(),
            },
            source_revision: SourceRevision::Http {
                body_sha256: descriptor.content_hash.clone(),
                etag: None,
                last_modified: None,
            },
            raw_sha256: descriptor.content_hash.clone(),
            locator: "fixture://document".into(),
            parser_revision: "fixture-v1".into(),
            parser_family: "fixture".into(),
            schema_version: Observed::unknown(UnknownReason::NotPresent),
            schema_sha256: Observed::unknown(UnknownReason::NotPresent),
            retrieved_at: Observed::unknown(UnknownReason::NotPresent),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::unknown(UnknownReason::NotPresent),
            origin_artifacts: BTreeSet::new(),
            derivation_family: Observed::unknown(UnknownReason::NotPresent),
            validity: GameValidity::unknown(),
            policy: SourcePolicy {
                visibility: descriptor.head.visibility,
                allowed_scopes: descriptor.head.allowed_scopes.clone(),
                authorization_ref: Observed::unknown(UnknownReason::NotPresent),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: true,
                provider_egress_allowed: true,
                raw_retention_allowed: false,
            },
        };
        descriptor.head.metadata.insert(
            ORIGIN_METADATA_KEY.into(),
            serde_json::to_string(&Versioned::new(origin)).unwrap(),
        );
        let principal = Principal {
            actor_id: "akteur".into(),
            channel: "kanal".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        };
        assert_eq!(
            manifest.authorized(&principal, false, false).unwrap().len(),
            1
        );
        assert!(manifest
            .authorized(&principal, true, false)
            .unwrap()
            .is_empty());
        manifest.heads[0].metadata = manifest.revisions[0].head.metadata.clone();
        assert_eq!(
            manifest.authorized(&principal, true, false).unwrap().len(),
            1
        );
    }
}

#[derive(Debug, Clone)]
pub struct ReleaseReadManifest {
    pub release: CorpusRelease,
    pub revisions: Vec<DocumentDescriptor>,
    pub heads: Vec<DocumentHead>,
}

impl ReleaseReadManifest {
    pub fn from_snapshot(snapshot: &CorpusSnapshot) -> Self {
        let project = |record: &crate::SourceRecordV2| {
            let mut head = DocumentHead::from(record);
            head.metadata
                .retain(|key, _| matches!(key.as_str(), "brain.origin" | "egress" | "patch"));
            head
        };
        Self {
            release: snapshot.release.clone(),
            revisions: snapshot
                .revisions
                .iter()
                .map(|record| DocumentDescriptor {
                    head: project(record),
                    content_hash: record.content_hash.clone(),
                })
                .collect(),
            heads: snapshot.heads.iter().map(project).collect(),
        }
    }

    pub fn validate(&self) -> Result<(), PortError> {
        let invalid = || PortError::InvalidResponse("Ungültiges Release-Lesemanifest".into());
        let count: usize = self
            .release
            .source_revisions
            .values()
            .map(BTreeMap::len)
            .sum();
        if count > 10000 || count != self.revisions.len() || count != self.heads.len() {
            return Err(invalid());
        }
        let mut heads = BTreeMap::new();
        for head in &self.heads {
            head.validate()?;
            head.canonical_origin()?;
            if heads
                .insert((&head.source_id, &head.logical_id), head)
                .is_some()
            {
                return Err(invalid());
            }
        }
        let mut seen = BTreeSet::new();
        for descriptor in &self.revisions {
            let revision = &descriptor.head;
            revision.validate()?;
            let origin = revision.canonical_origin()?;
            if descriptor.content_hash.len() != 64
                || !descriptor
                    .content_hash
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                || origin.is_some_and(|origin| origin.raw_sha256 != descriptor.content_hash)
                || !seen.insert((&revision.source_id, &revision.logical_id))
                || self
                    .release
                    .source_revisions
                    .get(&revision.source_id)
                    .and_then(|pins| pins.get(&revision.logical_id))
                    != Some(&revision.revision)
                || !heads
                    .get(&(&revision.source_id, &revision.logical_id))
                    .is_some_and(|head| head.revision >= revision.revision)
            {
                return Err(invalid());
            }
        }
        Ok(())
    }

    pub fn authorized(
        &self,
        principal: &Principal,
        provider: bool,
        publication: bool,
    ) -> Result<Vec<DocumentRevision>, PortError> {
        self.validate()?;
        let heads: BTreeMap<_, _> = self
            .heads
            .iter()
            .map(|head| ((&head.source_id, &head.logical_id), head))
            .collect();
        let mut keys = Vec::new();
        for descriptor in &self.revisions {
            let revision = &descriptor.head;
            let current = heads[&(&revision.source_id, &revision.logical_id)];
            if !revision.allowed(principal, provider) || !current.allowed(principal, provider) {
                continue;
            }
            if provider
                && revision.canonical_origin()?.is_some()
                && current.canonical_origin()?.is_none()
            {
                continue;
            }
            if publication {
                let pinned = revision.canonical_origin()?;
                let head = current.canonical_origin()?;
                if pinned
                    .as_ref()
                    .is_some_and(|origin| !origin.policy.publication_allowed)
                    || head
                        .as_ref()
                        .is_some_and(|origin| !origin.policy.publication_allowed)
                    || (pinned.is_some() && head.is_none())
                {
                    continue;
                }
            }
            keys.push(DocumentRevision {
                source_id: revision.source_id.clone(),
                logical_id: revision.logical_id.clone(),
                revision: revision.revision,
                content_hash: descriptor.content_hash.clone(),
            });
        }
        Ok(keys)
    }
}
