use crate::{git_source::PinnedRepository, SourcesError};
use brain_contracts::{entity_profile::EntityProfile, source::SourcePolicy, Principal};
use brain_storage::{
    entity_profile::derivation::{
        derive_git_profile_from_evidence_cached, git_evidence_identity, GitBlobEvidence,
        GitDocumentReceipt, GitProofBatchCache, StoredGitBinding,
    },
    PgStore,
};
use std::collections::BTreeMap;

pub use brain_storage::entity_profile::derivation::GIT_GAME_FACT_AUTHORIZATION;
pub type OriginalFactPin = StoredGitBinding;

#[derive(Debug)]
pub struct VerifiedGitProfile {
    profile: EntityProfile,
    original_pins: Vec<StoredGitBinding>,
    receipt: GitDocumentReceipt,
}

impl VerifiedGitProfile {
    pub fn profile(&self) -> &EntityProfile {
        &self.profile
    }
    pub fn original_pins(&self) -> &[StoredGitBinding] {
        &self.original_pins
    }
    pub fn receipt(&self) -> &GitDocumentReceipt {
        &self.receipt
    }
    pub fn policy(&self) -> SourcePolicy {
        brain_storage::entity_profile::derivation::derived_policy()
    }
}

type BlobCacheKey = (String, String, u64, String, String, String, String);

/// Laufinterne Originalblobs und Parserergebnisse, ohne gespeicherte Zugriffsrechte.
#[derive(Debug, Default)]
pub struct BatchDeriver {
    blobs: BTreeMap<BlobCacheKey, GitBlobEvidence>,
    proof_cache: GitProofBatchCache,
}

impl BatchDeriver {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn derive(
        &mut self,
        store: &PgStore,
        release_id: &str,
        operator: &Principal,
        entity_key: &str,
        repositories: &BTreeMap<(String, String), PinnedRepository>,
    ) -> crate::Result<VerifiedGitProfile> {
        let failed = |step, source| SourcesError::EntityDerivation {
            entity_key: entity_key.to_owned(),
            step,
            source,
        };
        // Nur zwischen Entitäten verdrängen, damit laufende Blobbelege erhalten bleiben.
        if self
            .blobs
            .values()
            .map(|blob| blob.bytes.len())
            .sum::<usize>()
            > 64 * 1024 * 1024
        {
            self.blobs.clear();
        }
        let (evidence, _) = store
            .entity_derivation_inputs(release_id, operator, entity_key)
            .await
            .map_err(|error| failed("original_inputs", error))?;
        let mut keys = Vec::new();
        for header in &evidence.originals {
            let record = &header.descriptor;
            let (commit, repository, path) = git_evidence_identity(header)?;
            let pinned = repositories
                .get(&(record.head.source_id.clone(), commit.clone()))
                .ok_or_else(|| {
                    SourcesError::invalid_input("Eigener gepinnter Git-Repositoryzugang fehlt")
                })?;
            if pinned.commit() != commit {
                return Err(SourcesError::invalid_input(
                    "Repositoryzuordnung widerspricht dem vollständigen Originalcommit",
                ));
            }
            pinned.require_origin(&[repository.as_str(), &format!("{repository}.git")])?;
            let key = (
                record.head.source_id.clone(),
                record.head.logical_id.clone(),
                record.head.revision,
                repository.clone(),
                commit.clone(),
                path.clone(),
                record.content_hash.clone(),
            );
            if !self.blobs.contains_key(&key) {
                self.blobs.insert(
                    key.clone(),
                    GitBlobEvidence {
                        source_id: record.head.source_id.clone(),
                        logical_id: record.head.logical_id.clone(),
                        store_revision: record.head.revision,
                        git_commit: commit,
                        repository_url: repository,
                        bytes: pinned.read_blob(&path)?,
                    },
                );
            }
            keys.push(key);
        }
        // Nach der Blobarbeit dieselbe frische Rechteprüfung wie im bisherigen Producer.
        let (evidence, bindings) = store
            .entity_derivation_inputs(release_id, operator, entity_key)
            .await
            .map_err(|error| failed("fresh_original_inputs", error))?;
        let identity = bindings
            .iter()
            .find(|binding| binding.semantic_projection.is_some())
            .ok_or_else(|| SourcesError::invalid_input("Git-Bindungsidentität fehlt"))?;
        let mut story_identity = identity.binding_identity.clone();
        for binding in &bindings {
            for alias in &binding.binding_identity.aliases {
                if !story_identity.aliases.contains(alias) {
                    story_identity.aliases.push(alias.clone());
                }
            }
        }
        let live_story = store
            .entity_patch_story(&story_identity)
            .await
            .map_err(|error| failed("patch_story", error))?;
        let blobs: Vec<_> = keys.iter().map(|key| &self.blobs[key]).collect();
        let (profile, receipt) = derive_git_profile_from_evidence_cached(
            entity_key,
            &evidence,
            operator,
            &bindings,
            &blobs,
            &live_story,
            &mut self.proof_cache,
        )
        .map_err(|error| failed("profile_proof", error))?;
        Ok(VerifiedGitProfile {
            profile,
            original_pins: bindings,
            receipt,
        })
    }
}

pub async fn derive_git_entity_profile(
    store: &PgStore,
    release_id: &str,
    operator: &Principal,
    entity_key: &str,
    repositories: &BTreeMap<(String, String), PinnedRepository>,
) -> crate::Result<VerifiedGitProfile> {
    BatchDeriver::new()
        .derive(store, release_id, operator, entity_key, repositories)
        .await
}
