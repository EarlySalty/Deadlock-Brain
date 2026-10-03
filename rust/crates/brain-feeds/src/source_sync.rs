use crate::{sha256_hex, FeedError, FeedPolicy, Result};
use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    CorpusRelease, SourceBatch, SourceVisibility,
};
use brain_ingestion::document_set::current_pins;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRights {
    #[serde(default)]
    pub ingestion_allowed: bool,
    #[serde(default)]
    pub authorization_ref: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub history_retention_months: u8,
    pub policy: FeedPolicy,
}

impl SourceRights {
    pub fn validate(&self) -> Result<()> {
        if !self.ingestion_allowed
            || self
                .authorization_ref
                .as_ref()
                .is_none_or(|s| s.trim().is_empty() || s.chars().any(char::is_control))
            || !self.policy.raw_retention_allowed
            || self.history_retention_months != 12
            || self.license.is_some()
            || self.policy.visibility != SourceVisibility::Internal
            || self.policy.publication_allowed
            || self.policy.provider_egress_allowed
            || self.policy.allowed_scopes.is_empty()
            || self.policy.allowed_scopes.iter().any(|scope| {
                scope.trim().is_empty()
                    || scope.contains('*')
                    || scope.chars().any(char::is_control)
            })
        {
            return Err(FeedError::Invalid(
                "Quellenrechte und Aufbewahrung sind nicht freigegeben.".into(),
            ));
        }
        Ok(())
    }
}

pub fn require_history_retention() -> Result<()> {
    Err(FeedError::Invalid("Der Kernspeicher erzwingt die bestätigte Aufbewahrung von zwölf Monaten noch nicht. Quellenschreiben bleibt gesperrt.".into()))
}

pub(crate) fn origin(
    source: &str,
    logical: &str,
    locator: &str,
    parser: &str,
    raw_hash: &str,
    observed_at: i64,
    rights: &SourceRights,
) -> OriginArtifact {
    OriginArtifact {
        identity: SourceIdentity {
            source_id: source.into(),
            logical_id: logical.into(),
        },
        source_revision: SourceRevision::Api {
            api_version: parser.into(),
            original_revision: Some(format!("sha256:{raw_hash}")),
        },
        raw_sha256: raw_hash.into(),
        locator: locator.into(),
        parser_revision: parser.into(),
        parser_family: source.split('/').next().unwrap_or(source).into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(observed_at)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::unknown(UnknownReason::NotPresent),
        origin_artifacts: BTreeSet::from([format!("{locator}@sha256:{raw_hash}")]),
        derivation_family: Observed::known(source.into()),
        policy: SourcePolicy {
            visibility: rights.policy.visibility,
            allowed_scopes: rights.policy.allowed_scopes.clone(),
            authorization_ref: Observed::known(
                rights.authorization_ref.clone().unwrap_or_default(),
            ),
            license: rights.license.clone().map_or_else(
                || Observed::unknown(UnknownReason::NotPresent),
                Observed::known,
            ),
            publication_allowed: rights.policy.publication_allowed,
            provider_egress_allowed: rights.policy.provider_egress_allowed,
            raw_retention_allowed: rights.policy.raw_retention_allowed,
        },
        validity: GameValidity::unknown(),
    }
}

pub fn candidate_release(
    base: &CorpusRelease,
    batches: &[SourceBatch],
    epoch: i64,
) -> Result<CorpusRelease> {
    if batches.is_empty() || epoch <= 0 {
        return Err(FeedError::Invalid(
            "Kandidatenrelease benötigt vollständige Quellen.".into(),
        ));
    }
    let mut release = base.clone();
    let mut seen = BTreeSet::new();
    for batch in batches {
        let source = &batch.checkpoint.source_id;
        if !(source.starts_with("google-sheet/") || source.starts_with("youtube-core/"))
            || !seen.insert(source)
        {
            return Err(FeedError::Invalid(
                "Unzulässige oder doppelte Quelle.".into(),
            ));
        }
        batch.validate()?;
        release
            .source_revisions
            .insert(source.clone(), current_pins(&batch.checkpoint)?);
    }
    let hash = sha256_hex(&serde_json::to_vec(&(
        &base.release_id,
        &release.source_revisions,
    ))?);
    release.release_id = format!("source-sync-{}", &hash[..32]);
    release.knowledge_version = format!("{}+sources.{}", base.knowledge_version, &hash[..12]);
    release.created_at_epoch = epoch;
    Ok(release)
}
