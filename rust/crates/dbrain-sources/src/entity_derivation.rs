use crate::{git_source::PinnedRepository, SourcesError};
use brain_contracts::{
    entity_profile::{EntityProfile, EntityProfileFact, ProfileSourceKind},
    source::{SourcePolicy, SourceRevision},
    value::Observed,
    Principal, SourceVisibility,
};
use brain_storage::{entity_profile::assemble_profile, PgStore};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

pub const GIT_GAME_FACT_AUTHORIZATION: &str = "VON_HAUPT.md:2026-10-04T15:05:git-spielfakten";

#[derive(Debug, Clone, Serialize)]
pub struct OriginalFactPin {
    pub source_id: String,
    pub logical_id: String,
    pub store_revision: u64,
    pub git_commit: String,
    pub original_fact: EntityProfileFact,
    pub binding_identity: brain_contracts::entity_profile::EntityIdentity,
}

#[derive(Debug)]
pub struct VerifiedGitProfile {
    profile: EntityProfile,
    original_pins: Vec<OriginalFactPin>,
}

impl VerifiedGitProfile {
    pub fn profile(&self) -> &EntityProfile {
        &self.profile
    }
    pub fn original_pins(&self) -> &[OriginalFactPin] {
        &self.original_pins
    }
    pub fn policy(&self) -> SourcePolicy {
        derived_policy()
    }
}

fn derived_policy() -> SourcePolicy {
    SourcePolicy {
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        authorization_ref: Observed::known(GIT_GAME_FACT_AUTHORIZATION.into()),
        license: Observed::unknown(brain_contracts::value::UnknownReason::NotPresent),
        publication_allowed: true,
        provider_egress_allowed: false,
        raw_retention_allowed: false,
    }
}

pub async fn derive_git_entity_profile(
    store: &PgStore,
    release_id: &str,
    operator: &Principal,
    entity_key: &str,
    pinned: &PinnedRepository,
) -> crate::Result<VerifiedGitProfile> {
    let profile = store
        .read_entity_profile(entity_key, release_id, operator, None)
        .await
        .map_err(|error| SourcesError::invariant(error.to_string()))?
        .ok_or_else(|| SourcesError::invalid_input("Autorisierte Originalbindung fehlt"))?;
    let snapshot = store
        .snapshot(release_id)
        .await
        .map_err(|_| SourcesError::invalid_input("Originalrelease fehlt"))?;
    let authorized = snapshot
        .authorized(operator, false)
        .map_err(|_| SourcesError::invalid_input("Originalautorisierung fehlt"))?;
    let mut facts = Vec::new();
    let mut original_pins = Vec::new();
    let mut derived_identity = None;
    let mut verified = BTreeSet::new();
    for fact in &profile.facts {
        if fact.provenance.source_kind != ProfileSourceKind::GameFile
            || (!fact.value.is_number()
                && !(fact
                    .qualifiers
                    .get("numeric_representation")
                    .and_then(Value::as_str)
                    == Some("source_numeric_lexeme")
                    && fact.value.as_str().is_some_and(|value| {
                        serde_json::from_str::<serde_json::Number>(value).is_ok()
                    })))
        {
            continue;
        }
        if matches!(
            fact.predicate.as_str(),
            "file.json_value" | "file.kv3_value" | "file.kv1_value"
        ) || fact.qualifiers.contains_key("source_pointer")
        {
            continue;
        }
        let identity = &fact.provenance.origin.identity;
        let record = authorized
            .iter()
            .find(|record| {
                record.source_id == identity.source_id && record.logical_id == identity.logical_id
            })
            .ok_or_else(|| SourcesError::invalid_input("Originalpin ist nicht autorisiert"))?;
        let document: Value = serde_json::from_str(
            record
                .metadata
                .get(brain_storage::source_versions::DOCUMENT_METADATA_KEY)
                .ok_or_else(|| SourcesError::invalid_input("Originaldokument fehlt"))?,
        )?;
        let key = (
            record.source_id.clone(),
            record.logical_id.clone(),
            record.revision,
        );
        if verified.insert(key) {
            let repository = document["metadata"]["provenance"]["repository_url"]
                .as_str()
                .filter(|url| {
                    matches!(
                        *url,
                        "https://github.com/deadlock-wiki/deadlock-data"
                            | "https://github.com/SteamTracking/GameTracking-Deadlock"
                    )
                })
                .ok_or_else(|| SourcesError::invalid_input("Freigegebener Git-Upstream fehlt"))?;
            pinned.require_origin(&[repository])?;
            let revision = document["metadata"]["source_revision"]
                .as_str()
                .unwrap_or("");
            if revision.strip_prefix("git:").unwrap_or(revision) != pinned.commit() {
                return Err(SourcesError::invalid_input(
                    "Gitcommit widerspricht der Originalrevision",
                ));
            }
            let path = document["metadata"]["original_relative_path"]
                .as_str()
                .ok_or_else(|| SourcesError::invalid_input("Originalblobpfad fehlt"))?;
            let blob = pinned.read_blob(path)?;
            let hash =
                crate::knowledge_contract::sha256_content(std::str::from_utf8(&blob).map_err(
                    |_| SourcesError::invalid_input("Git-Zahlenableitung benötigt belegtes UTF-8"),
                )?);
            if hash != record.content_hash
                || document["metadata"]["original_sha256"] != hash
                || record.content.as_bytes() != blob.as_slice()
            {
                return Err(SourcesError::invalid_input(
                    "Originalrevision und tatsächlicher Gitblob widersprechen sich",
                ));
            }
        }
        let original = brain_storage::entity_profile::project_entity_facts(
            record,
            std::slice::from_ref(&fact.fact_id),
        )
        .map_err(|error| SourcesError::invariant(error.to_string()))?
        .remove(0);
        let binding_identity = store
            .stored_entity_binding_identity(entity_key, record, &fact.fact_id)
            .await
            .map_err(|error| SourcesError::invariant(error.to_string()))?;
        if derived_identity.is_none() {
            derived_identity = Some(binding_identity.clone());
        }
        original_pins.push(OriginalFactPin {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
            git_commit: pinned.commit().into(),
            original_fact: original,
            binding_identity,
        });
        let mut derived = fact.clone();
        derived.fact_id = format!("derived:{}", facts.len());
        derived.subject = entity_key.into();
        derived.provenance.origin.identity.source_id = "git-game-facts-derived".into();
        derived.provenance.origin.identity.logical_id = entity_key.into();
        derived.provenance.origin.source_revision = SourceRevision::Git {
            commit: pinned.commit().into(),
        };
        derived.provenance.origin.policy = derived_policy();
        derived.provenance.origin.locator = "git-game-facts-derived".into();
        derived.provenance.origin.origin_artifacts.clear();
        derived.provenance.source_span = None;
        derived.provenance.document_metadata.clear();
        derived.provenance.license =
            serde_json::json!({"authorization_ref":GIT_GAME_FACT_AUTHORIZATION});
        derived.provenance.original_revision = pinned.commit().into();
        facts.push(derived);
    }
    if facts.is_empty() {
        return Err(SourcesError::invalid_input(
            "Freigegebene belegte Git-Zahlen fehlen",
        ));
    }
    let refreshed = store
        .snapshot(release_id)
        .await
        .map_err(|_| SourcesError::invalid_input("Frische Originalautorisierung fehlt"))?;
    let current = refreshed
        .authorized(operator, false)
        .map_err(|_| SourcesError::invalid_input("Originalrechte wurden entzogen"))?;
    for pin in &original_pins {
        if !current.iter().any(|record| {
            record.source_id == pin.source_id
                && record.logical_id == pin.logical_id
                && record.revision == pin.store_revision
                && record.content_hash == pin.original_fact.provenance.origin.raw_sha256
        }) {
            return Err(SourcesError::invalid_input(
                "Originalpin oder Originalrechte haben sich geändert",
            ));
        }
    }
    let mut entity = derived_identity
        .ok_or_else(|| SourcesError::invalid_input("Belegte Git-Identität fehlt"))?;
    entity.aliases.clear();
    entity.identity_evidence = vec![GIT_GAME_FACT_AUTHORIZATION.into()];
    let mut derived = assemble_profile(entity, None, facts, Vec::new());
    derived
        .unknowns
        .push("Anbieterweitergabe ist für diese Ableitung noch nicht freigegeben".into());
    Ok(VerifiedGitProfile {
        profile: derived,
        original_pins,
    })
}
