use crate::{clean, configuration, FeedError, FeedPolicy, Result};
use brain_contracts::{
    source::{SourceRevision, SourceTimestamp},
    value::{Observed, UnknownReason},
    SourceBatch, SourceCheckpoint,
};
use brain_ingestion::document_set::{prepare_document_batch, CoreDocument, DocumentSetSource};
use dbrain_sources::{
    assets_api::{asset_entities, prepare_assets, SOURCE as ASSETS_SOURCE},
    core::http::SourceHttpResponse,
    schema_watch::DriftReport,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const SOURCE: &str = "deadlock-assets";
pub const PARSER_REVISION: &str = "brain-feeds-assets.v1";
pub const MAX_FACT_BYTES: usize = 6 * 1024;

fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(clean(s)).filter(|s| !s.is_empty()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub fn documents(
    kind: &str,
    response: SourceHttpResponse,
    drift: Option<&DriftReport>,
    policy: &FeedPolicy,
) -> Result<Vec<CoreDocument>> {
    let ir =
        prepare_assets(kind, response, drift).map_err(|e| FeedError::Invalid(e.to_string()))?;
    if ir.is_quarantined() {
        return Err(FeedError::Quarantined(format!("{:?}", ir.validation())));
    }
    let payload = ir
        .payload()
        .map_err(|e| FeedError::Invalid(e.to_string()))?;
    let entities = asset_entities(kind, payload).map_err(|e| FeedError::Invalid(e.to_string()))?;
    let contract = ir.contract().data.clone();
    let provenance = contract.provenance.clone();
    let body_sha = provenance.raw_sha256.clone();
    let api_version = match &contract.schema_version {
        Observed::Known { value } => value.clone(),
        Observed::Unknown { .. } => {
            return Err(FeedError::Quarantined("schema version not pinned".into()))
        }
    };
    let mut out = Vec::with_capacity(entities.len());
    for entity in entities {
        let name = entity.canonical_name.as_deref().map(clean);
        let logical_id = format!("asset/{}/{}", entity.entity_type, entity.external_id);
        let mut content = format!(
            "{}: {}\nExternal ID: {}\nSource: {ASSETS_SOURCE}\n",
            entity.entity_type,
            name.clone().unwrap_or_else(|| "unknown".into()),
            entity.external_id
        );
        if let Value::Object(map) = &entity.payload {
            let sorted: BTreeMap<_, _> = map.iter().collect();
            for (key, value) in sorted {
                if let Some(value) = scalar(value) {
                    let line = format!("{}: {value}\n", clean(key));
                    if content.len() + line.len() > MAX_FACT_BYTES {
                        break;
                    }
                    content.push_str(&line);
                }
            }
        }
        let mut origin = contract.origin_artifact();
        origin.source_revision = SourceRevision::Api {
            api_version: api_version.clone(),
            original_revision: Some(format!("sha256:{body_sha}")),
        };
        origin.locator = format!("{}#/{}", provenance.locator, entity.external_id);
        origin.parser_revision = PARSER_REVISION.into();
        origin.origin_artifacts =
            BTreeSet::from([format!("{}@sha256:{body_sha}", provenance.locator)]);
        origin.retrieved_at = Observed::known(SourceTimestamp::UnixSeconds(provenance.observed_at));
        origin.language = Observed::unknown(UnknownReason::NotPresent);
        origin.policy.publication_allowed = policy.publication_allowed;
        origin.policy.provider_egress_allowed = policy.provider_egress_allowed;
        origin.policy.raw_retention_allowed = policy.raw_retention_allowed;
        let mut metadata = BTreeMap::from([
            ("connector".to_string(), SOURCE.to_string()),
            ("kind".into(), "fact".into()),
            ("locator".into(), origin.locator.clone()),
            ("asset_kind".into(), kind.into()),
            ("http_body_sha256".into(), body_sha.clone()),
        ]);
        if let Some(name) = name {
            metadata.insert("name".into(), name);
        }
        out.push(CoreDocument {
            logical_id,
            content,
            metadata,
            origin,
        });
    }
    Ok(out)
}

pub fn prepare_batch(
    kind: &str,
    response: SourceHttpResponse,
    drift: Option<&DriftReport>,
    policy: &FeedPolicy,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    let source_id = format!("{SOURCE}-{kind}");
    let set = DocumentSetSource {
        configuration: configuration(&source_id, PARSER_REVISION, policy)?,
        source_id,
        visibility: policy.visibility,
        allowed_scopes: policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([("connector".into(), SOURCE.into())]),
    };
    Ok(prepare_document_batch(
        &set,
        &documents(kind, response, drift, policy)?,
        previous,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{source::origin_from_record, SourceVisibility};

    fn response(body: &str) -> SourceHttpResponse {
        SourceHttpResponse {
            url: "https://api.deadlock-api.com/v1/assets/heroes?only_active=true".into(),
            status: 200,
            content: body.as_bytes().to_vec(),
            headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
            observed_at: 1_790_000_000,
            attempts: 1,
        }
    }
    fn policy() -> FeedPolicy {
        FeedPolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["game.public".into()]),
            provider_egress_allowed: false,
            publication_allowed: false,
            raw_retention_allowed: false,
        }
    }

    #[test]
    fn heroes_become_fact_records_with_http_provenance_and_quarantine_blocks() {
        let heroes = include_str!("../tests/fixtures/heroes.json");
        let batch = prepare_batch("heroes", response(heroes), None, &policy(), None).unwrap();
        assert!(!batch.records.is_empty());
        let record = &batch.records[0];
        assert_eq!(record.metadata["kind"], "fact");
        assert!(record.content.len() <= MAX_FACT_BYTES);
        let origin = origin_from_record(record).unwrap();
        assert_eq!(origin.raw_sha256, record.content_hash);
        assert!(origin
            .origin_artifacts
            .iter()
            .all(|a| a.contains("@sha256:")));
        let again = prepare_batch("heroes", response(heroes), None, &policy(), None).unwrap();
        assert_eq!(
            batch
                .records
                .iter()
                .map(|r| &r.content_hash)
                .collect::<Vec<_>>(),
            again
                .records
                .iter()
                .map(|r| &r.content_hash)
                .collect::<Vec<_>>()
        );
        assert!(prepare_batch(
            "heroes",
            response("[{\"name\":\"x\"}]"),
            None,
            &policy(),
            None
        )
        .is_err());
        assert!(prepare_batch("heroes", response("{not json"), None, &policy(), None).is_err());
    }
}
