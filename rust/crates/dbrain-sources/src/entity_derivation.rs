use crate::{git_source::PinnedRepository, SourcesError};
use brain_contracts::{entity_profile::EntityProfile, source::SourcePolicy, Principal};
use brain_storage::{
    entity_profile::derivation::{
        derive_git_profile, git_document_identity, GitBlobEvidence, GitDocumentReceipt,
        StoredGitBinding,
    },
    PgStore,
};
use std::collections::{BTreeMap, BTreeSet};

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

pub async fn derive_git_entity_profile(
    store: &PgStore,
    release_id: &str,
    operator: &Principal,
    entity_key: &str,
    repositories: &BTreeMap<(String, String), PinnedRepository>,
) -> crate::Result<VerifiedGitProfile> {
    let snapshot = store
        .snapshot(release_id)
        .await
        .map_err(|error| SourcesError::invariant(error.to_string()))?;
    let authorized = snapshot
        .authorized(operator, false)
        .map_err(|error| SourcesError::invalid_input(error.to_string()))?;
    let mut bindings = Vec::new();
    let mut blobs = Vec::new();
    let mut seen = BTreeSet::new();
    for visible in authorized {
        let record = snapshot
            .revisions
            .iter()
            .find(|record| {
                record.source_id == visible.source_id
                    && record.logical_id == visible.logical_id
                    && record.revision == visible.revision
            })
            .ok_or_else(|| SourcesError::invalid_input("Originalpin fehlt"))?;
        let rows = store
            .stored_git_entity_bindings(entity_key, record)
            .await
            .map_err(|error| SourcesError::invariant(error.to_string()))?;
        if rows.is_empty() {
            continue;
        }
        let (commit, repository, path) = git_document_identity(record)
            .map_err(|error| SourcesError::invalid_input(error.to_string()))?;
        let pinned = repositories
            .get(&(record.source_id.clone(), commit.clone()))
            .ok_or_else(|| {
                SourcesError::invalid_input("Eigener gepinnter Git-Repositoryzugang fehlt")
            })?;
        if pinned.commit() != commit {
            return Err(SourcesError::invalid_input(
                "Repositoryzuordnung widerspricht dem vollständigen Originalcommit",
            ));
        }
        pinned.require_origin(&[repository.as_str(), &format!("{repository}.git")])?;
        if seen.insert((
            record.source_id.clone(),
            record.logical_id.clone(),
            record.revision,
        )) {
            blobs.push(GitBlobEvidence {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
                store_revision: record.revision,
                git_commit: commit,
                repository_url: repository,
                bytes: pinned.read_blob(&path)?,
            });
        }
        bindings.extend(rows);
    }
    let refreshed = store
        .snapshot(release_id)
        .await
        .map_err(|error| SourcesError::invariant(error.to_string()))?;
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
        .map_err(|error| SourcesError::invariant(error.to_string()))?;
    let (profile, receipt) = derive_git_profile(
        entity_key,
        &refreshed,
        operator,
        &bindings,
        &blobs,
        &live_story,
    )
    .map_err(|error| SourcesError::invariant(error.to_string()))?;
    Ok(VerifiedGitProfile {
        profile,
        original_pins: bindings,
        receipt,
    })
}
