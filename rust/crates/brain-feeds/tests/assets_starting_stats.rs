use brain_contracts::{source::origin_from_record, SourceVisibility};
use brain_feeds::{deadlock_assets, FeedError, FeedPolicy};
use dbrain_sources::core::http::SourceHttpResponse;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const FIELDS: [&str; 5] = [
    "max_health",
    "max_move_speed",
    "sprint_speed",
    "stamina",
    "base_health_regen",
];

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
fn each_pinned_hero_has_exactly_five_field_bound_starting_stats() {
    let body = include_str!("fixtures/heroes.json");
    let source: Value = serde_json::from_str(body).unwrap();
    let batch =
        deadlock_assets::prepare_batch("heroes", response(body), None, &policy(), None).unwrap();
    let digest = hex::encode(Sha256::digest(body.as_bytes()));
    assert_eq!(
        batch.records.len(),
        source.as_array().unwrap().len() * (FIELDS.len() + 1)
    );
    for (index, hero) in source.as_array().unwrap().iter().enumerate() {
        let id = hero["id"].as_i64().unwrap();
        let name = hero["name"].as_str().unwrap();
        for field in FIELDS {
            let key = format!("starting_stats.{field}.value");
            let logical_id = format!("asset/hero/{id}/{key}");
            let matching: Vec<_> = batch
                .records
                .iter()
                .filter(|record| record.logical_id == logical_id)
                .collect();
            assert_eq!(matching.len(), 1, "{logical_id}");
            let record = matching[0];
            let label = field.replace('_', " ");
            let pointer = format!("/{index}/starting_stats/{field}/value");
            let value = source.pointer(&pointer).unwrap();
            assert!(value.is_number(), "{pointer}");
            assert_eq!(record.content, format!("Hero: {name}\n{label}: {value}\n"));
            assert_eq!(record.metadata["fact_key"], key);
            assert_eq!(record.metadata["field"], label);
            assert_eq!(record.metadata["name"], name);
            assert_eq!(record.metadata["entity_external_id"], id.to_string());
            assert_eq!(record.metadata["source_pointer"], pointer);
            assert_eq!(record.metadata["http_body_sha256"], digest);
            assert_eq!(record.visibility, SourceVisibility::Public);
            assert_eq!(
                record.allowed_scopes,
                BTreeSet::from(["game.public".into()])
            );
            assert!(record.validate().is_ok());
            let origin = origin_from_record(record).unwrap();
            assert_eq!(origin.parser_revision, deadlock_assets::PARSER_REVISION);
            assert!(origin.locator.ends_with(&format!("#{pointer}")));
            assert_eq!(origin.raw_sha256, record.content_hash);
        }
    }
}

#[test]
fn any_missing_or_non_numeric_pinned_stat_quarantines_the_entire_batch() {
    let original: Value = serde_json::from_str(include_str!("fixtures/heroes.json")).unwrap();
    for field in FIELDS {
        let mut missing = original.clone();
        missing[0]["starting_stats"][field]
            .as_object_mut()
            .unwrap()
            .remove("value");
        assert!(matches!(
            deadlock_assets::prepare_batch(
                "heroes",
                response(&missing.to_string()),
                None,
                &policy(),
                None
            ),
            Err(FeedError::Quarantined(_))
        ));
        for invalid in [Value::Null, Value::String("3".into())] {
            let mut changed = original.clone();
            changed[0]["starting_stats"][field]["value"] = invalid;
            assert!(matches!(
                deadlock_assets::prepare_batch(
                    "heroes",
                    response(&changed.to_string()),
                    None,
                    &policy(),
                    None
                ),
                Err(FeedError::Quarantined(_))
            ));
        }
    }
}
