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
// Required numeric fields in the pinned Deadlock API Hero.StartingStats schema.
const HERO_STARTING_FACTS: &[&str] = &[
    "max_health",
    "max_move_speed",
    "sprint_speed",
    "stamina",
    "base_health_regen",
];

fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(clean(s)).filter(|s| !s.is_empty()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn hero_starting_facts(payload: &Value) -> Result<Vec<(&'static str, String)>> {
    let stats = payload
        .get("starting_stats")
        .and_then(Value::as_object)
        .ok_or_else(|| FeedError::Quarantined("hero starting_stats missing or invalid".into()))?;
    HERO_STARTING_FACTS
        .iter()
        .map(|field| {
            let number = stats
                .get(*field)
                .and_then(Value::as_object)
                .and_then(|stat| stat.get("value"))
                .and_then(Value::as_number)
                .ok_or_else(|| {
                    FeedError::Quarantined(format!(
                        "hero starting_stats.{field}.value missing or not numeric"
                    ))
                })?;
            Ok((*field, number.to_string()))
        })
        .collect()
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
        let hero_facts = if entity.entity_type == "hero" {
            hero_starting_facts(&entity.payload)?
        } else {
            Vec::new()
        };
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
        if let Some(name) = &name {
            metadata.insert("name".into(), name.clone());
        }
        out.push(CoreDocument {
            logical_id,
            content,
            metadata,
            origin: origin.clone(),
        });
        for (field, value) in hero_facts {
            let fact_key = format!("starting_stats.{field}.value");
            let field_label = field.replace('_', " ");
            let source_pointer = format!("/starting_stats/{field}/value");
            let mut field_origin = origin.clone();
            field_origin.locator = format!(
                "{}#/{}/starting_stats/{field}/value",
                provenance.locator, entity.external_id
            );
            let mut field_metadata = BTreeMap::from([
                ("connector".into(), SOURCE.into()),
                ("kind".into(), "fact".into()),
                ("locator".into(), field_origin.locator.clone()),
                ("asset_kind".into(), kind.into()),
                ("http_body_sha256".into(), body_sha.clone()),
                ("fact_key".into(), fact_key.clone()),
                ("field".into(), field_label.clone()),
                ("source_pointer".into(), source_pointer),
            ]);
            if let Some(name) = &name {
                field_metadata.insert("name".into(), name.clone());
            }
            out.push(CoreDocument {
                logical_id: format!("asset/hero/{}/{fact_key}", entity.external_id),
                content: format!(
                    "Hero: {}\n{field_label}: {value}\n",
                    name.as_deref().unwrap_or("unknown"),
                ),
                metadata: field_metadata,
                origin: field_origin,
            });
        }
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

    #[test]
    fn typed_hero_numbers_become_field_bound_records() {
        let heroes = include_str!("../tests/fixtures/heroes.json");
        let batch = prepare_batch("heroes", response(heroes), None, &policy(), None).unwrap();
        let fact = batch
            .records
            .iter()
            .find(|record| record.logical_id == "asset/hero/25/starting_stats.max_health.value")
            .unwrap();
        assert!(fact.content.contains("Hero: Warden"));
        assert!(fact.content.contains("max health: 770"));
        assert_eq!(fact.metadata["fact_key"], "starting_stats.max_health.value");
        assert_eq!(fact.metadata["field"], "max health");
        assert_eq!(
            fact.metadata["source_pointer"],
            "/starting_stats/max_health/value"
        );
        let origin = origin_from_record(fact).unwrap();
        assert!(origin
            .locator
            .ends_with("#/25/starting_stats/max_health/value"));
        assert!(origin
            .origin_artifacts
            .iter()
            .all(|artifact| artifact.contains("@sha256:")));
        assert_eq!(batch.records.len(), 12);
    }

    #[test]
    fn missing_or_non_numeric_hero_stat_quarantines_the_whole_batch() {
        let original: Value =
            serde_json::from_str(include_str!("../tests/fixtures/heroes.json")).unwrap();
        for bad in [Value::Null, Value::String("770".into())] {
            let mut changed = original.clone();
            changed[0]["starting_stats"]["max_health"]["value"] = bad;
            let result = prepare_batch(
                "heroes",
                response(&changed.to_string()),
                None,
                &policy(),
                None,
            );
            assert!(matches!(result, Err(FeedError::Quarantined(_))));
        }
        let mut missing = original;
        missing[0]["starting_stats"]
            .as_object_mut()
            .unwrap()
            .remove("stamina");
        assert!(matches!(
            prepare_batch(
                "heroes",
                response(&missing.to_string()),
                None,
                &policy(),
                None
            ),
            Err(FeedError::Quarantined(_))
        ));
    }
}
