use brain_contracts::{
    answer_contract::{citable_values, provider_citable_values},
    provider_input::grounded_messages,
    Evidence, Query,
};
use serde_json::{json, Value};

#[test]
fn compact_catalog_keeps_supplied_entity_values_without_duplicate_proof_fields() {
    let document = json!({
        "contract_version": brain_contracts::entity_profile::ENTITY_PROFILE_VERSION,
        "entity": {"name":"Abrams","aliases":["abrams"]},
        "patch":"2026-10-11",
        "facts":[{
            "fact_id":"health-42", "subject":"Abrams", "predicate":"health",
            "value":650, "unit":"HP", "qualifiers":{"level/variant~":3},
            "validity":{"from_patch":"2026-10-11"},
            "source_ref":0, "logical_id":"original-42", "observed_at":42,
            "locator":"https://example.invalid/42", "policy":{"revision":42}
        }],
        "context":[{"value":2,"unit":"m/s"}],
        "patch_story":[{"patch_date":"2026-10-11","old_value":600,"new_value":650,"conditions":{"level":3},"provenance":{"revision":42}}],
        "sources":[{"source_id":"deadlock-assets-heroes","original_revision":"proof-42"}]
    });
    let evidence: Evidence = serde_json::from_value(json!({
        "evidence_id":"abrams-proof", "source_id":"brain.entity_profile_entities_v1", "logical_id":"abrams",
        "revision":1, "kind":"prose", "content":document.to_string(),
        "citation":"synthetic", "visibility":"public", "allowed_scopes":[], "score":1.0,
        "provenance":null, "patch":null
    })).unwrap();
    let expected = json!([
        {"kind":"game_value","field":"/facts/0/value","value":"650"},
        {"kind":"game_value","field":"/facts/0/qualifiers/level~1variant~0","value":"3"},
        {"kind":"game_value","field":"/facts/0/validity/from_patch","value":"2026-10-11"},
        {"kind":"game_value","field":"/context/0/value","value":"2"},
        {"kind":"game_value","field":"/patch_story/0/patch_date","value":"2026-10-11"},
        {"kind":"game_value","field":"/patch_story/0/old_value","value":"600"},
        {"kind":"game_value","field":"/patch_story/0/new_value","value":"650"},
        {"kind":"game_value","field":"/patch_story/0/conditions/level","value":"3"},
        {"kind":"other","field":"/patch","value":"2026-10-11"}
    ]);
    let supplied = provider_citable_values(&evidence);
    assert_eq!(serde_json::to_value(&supplied).unwrap(), expected);
    let validation = citable_values(&evidence);
    assert!(supplied.iter().all(|value| validation.contains(value)));
    assert!(validation
        .iter()
        .any(|value| value.field == "/facts/0/observed_at"));
    assert!(supplied.iter().all(|value| !value.value.contains("42")));
    let query: Query = serde_json::from_value(json!({
        "request_id":"synthetic", "conversation_id":"synthetic", "text":"Wie viel Leben hat Abrams?"
    }))
    .unwrap();
    let messages = grounded_messages(&query, std::slice::from_ref(&evidence));
    let data: Value = serde_json::from_str(&messages[1].content).unwrap();
    assert_eq!(data["evidence"].as_array().unwrap().len(), 1);
    assert_eq!(data["evidence"][0]["id"], evidence.evidence_id);
    assert_eq!(data["evidence"][0]["content"], evidence.content);
    assert_eq!(data["evidence"][0]["citable_values"], expected);
}

#[test]
fn value_catalog_does_not_restore_filtered_discord_identifiers() {
    for (source, visibility, purpose) in [
        ("discord.public-live.v1", "request_scoped", None),
        (
            "deadlock-assets-heroes",
            "public",
            Some("bot_task:concierge"),
        ),
    ] {
        let evidence: Evidence = serde_json::from_value(json!({
            "evidence_id":"synthetic-proof", "source_id":source, "logical_id":"synthetic",
            "revision":1, "kind":"fact", "content":"count: 1494373349944459355\ncapacity: 3\nversion: v12 <@42>",
            "citation":"synthetic", "visibility":visibility, "allowed_scopes":[], "score":1.0,
            "provenance":null, "patch":null
        })).unwrap();
        let mut query: Query = serde_json::from_value(json!({
            "request_id":"synthetic", "conversation_id":"synthetic", "text":"Welche Angebote gibt es?"
        })).unwrap();
        if let Some(purpose) = purpose {
            query.answer_context =
                serde_json::from_value(json!({"platform":"discord","purpose":purpose})).unwrap();
        }
        let messages = grounded_messages(&query, &[evidence]);
        let data: Value = serde_json::from_str(&messages[1].content).unwrap();
        assert_eq!(
            data["evidence"][0]["citable_values"],
            json!([
                {"kind":"other","field":"capacity","value":"3"}
            ])
        );
        for raw in ["1494373349944459355", "<@42>"] {
            assert!(!messages[1].content.contains(raw));
        }
    }
}
