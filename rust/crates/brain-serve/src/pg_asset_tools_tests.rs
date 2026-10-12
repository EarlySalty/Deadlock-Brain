use super::*;
use brain_contracts::tools::ToolCall;

fn request(name: ToolName, arguments: Value) -> ToolRequest {
    ToolCall {
        id: "asset-test".into(),
        name,
        arguments,
    }
    .validate(&definitions())
    .unwrap()
}

fn comparison_request() -> ToolRequest {
    request(
        ToolName::Compare,
        json!({"entities":[1,2,3,4],
        "stats":(0..12).map(|index|format!("stats.{}_{index}", "k".repeat(110))).collect::<Vec<_>>()}),
    )
}

#[test]
fn definitions_validate_typed_requests_and_separate_historical_selection() {
    let definitions = definitions();
    brain_contracts::tools::validate_definitions(&definitions).unwrap();
    for (name, arguments) in [
        (ToolName::GetItem, json!({"item":1})),
        (ToolName::GetAbility, json!({"ability":1})),
        (ToolName::GetAbility, json!({"hero":1,"slot":4})),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["weapon.damage"]}),
        ),
    ] {
        assert!(canonical(name));
        assert_eq!(serde_json::to_value(name).unwrap(), name.as_str());
        let call = ToolCall {
            id: "schema-test".into(),
            name,
            arguments: arguments.clone(),
        };
        assert!(!historical(&call.validate(&definitions).unwrap()));
        for patch in ["current", "old"] {
            let mut call = call.clone();
            call.arguments["patch"] = json!(patch);
            assert_eq!(
                historical(&call.validate(&definitions).unwrap()),
                patch != "current"
            );
        }
        let mut call = call;
        call.arguments["sql"] = json!("SELECT 1");
        assert!(call.validate(&definitions).is_err());
    }
    assert!(!canonical(ToolName::DeadlockData));
}

fn identity(entity: u64) -> Value {
    json!({"entity_id":entity,"kind":"hero","key":format!("hero_{entity}"),
        "citation":entity.to_string(),"patch":null,"source_revision":"commit-a"})
}

fn fact(entity: u64, key: &str, citation: u64) -> Value {
    json!({"entity_id":entity,"stat_key":key,"value":"x".repeat(512),"unit":"damage",
        "condition":{"rank":3,"citation":"source-condition"},"citation":citation.to_string(),
        "patch":null,"source_revision":"commit-a"})
}

fn card(rows: Vec<Value>) -> Value {
    let mut identity = identity(1);
    identity["kind"] = json!("ability");
    json!({"identity":identity,
        "facts":{"facts":rows,"omitted_count":0,"truncated":false},
        "links":{"links":[],"omitted_count":0,"truncated":false}})
}

fn result(value: Value) -> Option<ToolResult> {
    bounded_card_result(
        "asset-test",
        &request(ToolName::GetAbility, json!({"ability":1})),
        value,
    )
    .unwrap()
}

#[test]
fn citations_are_read_only_from_receipted_rows_not_arbitrary_source_json() {
    let mut row = fact(1, "ability.damage", 5);
    row["value"] = json!({"citation":"source-value","facts":{"citation":"nested-source"},
        "links":[{"citation":"source-link"}]});
    row["condition"] =
        json!({"citation":"source-condition","identity":{"citation":"source-identity"}});
    let link = json!({"entity_id":2,"kind":"has_ability","citation":"6","patch":null,
        "source_revision":"assets-a","identity":identity(2)});
    let mut value = card(vec![row.clone()]);
    value["links"]["links"] = json!([link]);
    let bounded = result(value.clone()).unwrap();
    assert_eq!(bounded.result, value);
    assert_eq!(bounded.evidence_ids, ["1", "2", "5", "6"]);
    for pointer in [
        "/identity",
        "/facts/facts/0",
        "/links/links/0",
        "/links/links/0/identity",
    ] {
        for key in ["citation", "patch", "source_revision"] {
            let mut missing = value.clone();
            missing
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key);
            assert!(result(missing).is_none());
        }
    }
    let mut missing = value.clone();
    missing["facts"]["facts"][0]["condition"] = Value::Null;
    assert!(result(missing).is_none());
    let mut foreign = value.clone();
    foreign["facts"]["facts"][0]["entity_id"] = json!(3);
    assert!(result(foreign).is_none());
    let mut historical = value;
    historical["facts"]["facts"][0]["patch"] = json!("unconfirmed");
    assert!(result(historical).is_none());
}

#[test]
fn identity_names_carry_their_own_receipts_and_match_the_displayed_values() {
    let mut value = card(Vec::new());
    value["identity"]["name_en"] = json!("Toxic Bullets");
    value["identity"]["facts"] = json!([{
        "stat_key":"name_en","value":"Toxic Bullets","condition":{},
        "citation":"9","patch":null,"source_revision":"assets-a"
    }]);
    let bounded = result(value.clone()).unwrap();
    assert_eq!(bounded.result, value);
    assert_eq!(bounded.evidence_ids, ["1", "9"]);
    for (pointer, replacement) in [
        ("/identity/facts/0/citation", Value::Null),
        ("/identity/facts/0/patch", json!("unconfirmed")),
        ("/identity/facts/0/source_revision", Value::Null),
        ("/identity/facts/0/value", json!("Different name")),
        ("/identity/facts", json!([])),
        ("/identity/key", Value::Null),
    ] {
        let mut malformed = value.clone();
        *malformed.pointer_mut(pointer).unwrap() = replacement;
        assert!(result(malformed).is_none());
    }
    let mut malformed = value;
    malformed["identity"]["facts"][0]["entity_id"] = json!(2);
    assert!(result(malformed).is_none());
}

#[test]
fn relation_endpoint_identity_must_match_the_displayed_endpoint() {
    let mut value = card(Vec::new());
    value["links"]["links"] = json!([{
        "entity_id":2,"kind":"has_ability","citation":"6","patch":null,
        "source_revision":"assets-a","identity":identity(3)
    }]);
    assert!(result(value.clone()).is_none());
    value["links"]["links"][0]["identity"] = identity(2);
    assert_eq!(result(value).unwrap().evidence_ids, ["1", "2", "6"]);
}

#[test]
fn slot_selection_keeps_at_least_one_relation_receipt_when_trimming() {
    let mut row = fact(1, "ability.damage", 5);
    row["value"] = json!("x".repeat(MAX_RESULT_BYTES));
    let mut value = card(vec![row]);
    let link = json!({"entity_id":2,"kind":"has_ability","slot":3,
        "from_entity_id":2,"to_entity_id":1,"citation":"6","patch":null,
        "source_revision":"assets-a","identity":identity(2)});
    value["abilities"] = json!({"links":[link.clone()],"omitted_count":0,"truncated":false});
    let selection = request(ToolName::GetAbility, json!({"hero":2,"slot":3}));
    let bounded = bounded_card_result("slot-test", &selection, value.clone())
        .unwrap()
        .unwrap();
    assert_eq!(bounded.result["abilities"]["links"], json!([link]));
    assert_eq!(bounded.evidence_ids, ["1", "2", "6"]);
    assert_eq!(bounded.result["facts"]["omitted_count"], 1);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
    value["abilities"]["links"] = json!([]);
    assert!(bounded_card_result("slot-test", &selection, value)
        .unwrap()
        .is_none());
}

#[test]
fn sections_preserve_counts_and_require_receipts_even_on_trimmed_rows() {
    let original = (0..12)
        .map(|index| {
            let mut row = fact(1, &format!("ability.value_{index}"), index + 10);
            row["value"] = json!("x".repeat(2500));
            row
        })
        .collect::<Vec<_>>();
    let mut value = card(original.clone());
    value["facts"]["omitted_count"] = json!(7);
    value["facts"]["truncated"] = json!(true);
    value["links"]["omitted_count"] = json!(3);
    value["links"]["truncated"] = json!(true);
    let bounded = result(value.clone()).unwrap();
    let rows = bounded.result["facts"]["facts"].as_array().unwrap();
    assert!(!rows.is_empty() && rows.len() < original.len());
    assert!(rows.iter().all(|row| original.contains(row)));
    assert_eq!(
        bounded.result["facts"]["omitted_count"],
        7 + original.len() - rows.len()
    );
    assert_eq!(bounded.result["links"]["omitted_count"], 3);
    assert_eq!(bounded.result["identity"], value["identity"]);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
    for (pointer, replacement) in [
        ("/facts", json!([])),
        ("/facts/facts", json!({})),
        ("/facts/omitted_count", json!(-1)),
        ("/facts/omitted_count", json!(u64::MAX)),
        ("/facts/truncated", json!(false)),
        ("/links/omitted_count", Value::Null),
        ("/facts/facts/11/citation", Value::Null),
    ] {
        let mut malformed = value.clone();
        *malformed.pointer_mut(pointer).unwrap() = replacement;
        assert!(result(malformed).is_none());
    }
}

#[test]
fn actual_call_id_escaping_and_evidence_ids_are_in_the_result_budget() {
    let mut row = fact(1, "ability.description_passive", 5);
    row["value"] = json!("");
    row["citation"] = json!("9".repeat(512));
    let value = card(vec![row]);
    let baseline = bounded_card_result(
        "x",
        &request(ToolName::GetAbility, json!({"ability":1})),
        value.clone(),
    )
    .unwrap()
    .unwrap();
    let baseline_bytes = serde_json::to_vec(&baseline).unwrap().len();
    let mut value = value;
    value["facts"]["facts"][0]["value"] = json!("x".repeat(MAX_RESULT_BYTES - baseline_bytes));
    let exact = bounded_card_result(
        "x",
        &request(ToolName::GetAbility, json!({"ability":1})),
        value.clone(),
    )
    .unwrap()
    .unwrap();
    assert_eq!(serde_json::to_vec(&exact).unwrap().len(), MAX_RESULT_BYTES);
    assert_eq!(exact.result["facts"]["facts"].as_array().unwrap().len(), 1);
    for call_id in ["\"".repeat(512), "\\".repeat(512), "ä".repeat(256)] {
        let bounded = bounded_card_result(
            &call_id,
            &request(ToolName::GetAbility, json!({"ability":1})),
            value.clone(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(bounded.call_id, call_id);
        assert!(bounded.result["facts"]["facts"]
            .as_array()
            .unwrap()
            .is_empty());
        assert_eq!(bounded.result["facts"]["omitted_count"], 1);
        assert_eq!(bounded.result["facts"]["truncated"], true);
        assert_eq!(bounded.evidence_ids, ["1"]);
        assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
    }
}

fn comparison() -> Value {
    let keys = (0..12)
        .map(|index| format!("stats.{}_{index}", "k".repeat(110)))
        .collect::<Vec<_>>();
    let mut facts = Vec::new();
    let mut cells = Vec::new();
    for entity in 1..=4 {
        for (index, key) in keys.iter().enumerate() {
            let total = if entity == 4 && index == 11 { 0 } else { 1 };
            if total > 0 {
                facts.push(fact(entity, key, entity * 100 + index as u64));
            }
            cells.push(json!({"entity_id":entity,"stat_key":key,"total_count":total}));
        }
    }
    json!({"entities":(1..=4).map(identity).collect::<Vec<_>>(),"stats":{"facts":facts},"cells":cells})
}

#[test]
fn full_four_by_twelve_grid_and_identities_survive_bounded_comparison() {
    let value = comparison();
    let original = value["stats"]["facts"].as_array().unwrap().clone();
    let bounded =
        bounded_comparison_result(&"\"".repeat(512), &comparison_request(), value.clone())
            .unwrap()
            .unwrap();
    assert_eq!(bounded.result["entities"], value["entities"]);
    let cells = bounded.result["cells"].as_array().unwrap();
    assert_eq!(cells.len(), 48);
    assert_eq!(cells[47]["status"], "missing");
    assert!(cells.iter().any(|cell| cell["status"] == "omitted"));
    let retained = bounded.result["stats"]["facts"].as_array().unwrap();
    assert!(retained.len() < 47);
    assert!(retained.iter().all(|row| original.contains(row)));
    assert_eq!(
        bounded.result["stats"]["omitted_count"],
        47 - retained.len()
    );
    assert_eq!(bounded.result["stats"]["truncated"], true);
    assert!((1..=4).all(|entity| bounded.evidence_ids.contains(&entity.to_string())));
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
}

#[test]
fn comparison_rejects_incomplete_duplicate_and_foreign_grid_cells() {
    let value = comparison();
    let mut cases = Vec::new();
    let mut missing = value.clone();
    missing["cells"].as_array_mut().unwrap().pop();
    cases.push(missing);
    let mut duplicate = value.clone();
    duplicate["cells"][47] = duplicate["cells"][46].clone();
    cases.push(duplicate);
    let mut foreign = value.clone();
    foreign["stats"]["facts"][0]["entity_id"] = json!(5);
    cases.push(foreign);
    let mut foreign = value.clone();
    foreign["stats"]["facts"][0]["stat_key"] = json!("stats.other");
    cases.push(foreign);
    let mut count = value.clone();
    count["cells"][0]["total_count"] = json!(0);
    cases.push(count);
    let mut duplicate = value.clone();
    duplicate["entities"][3] = duplicate["entities"][2].clone();
    cases.push(duplicate);
    let mut overflow = value;
    for cell in overflow["cells"].as_array_mut().unwrap() {
        cell["total_count"] = json!(u64::MAX);
    }
    cases.push(overflow);
    for malformed in cases {
        assert!(
            bounded_comparison_result("grid-test", &comparison_request(), malformed)
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn identities_are_never_removed_to_fit_the_budget() {
    let mut value = card(Vec::new());
    value["identity"]["key"] = json!("x".repeat(MAX_RESULT_BYTES));
    assert!(result(value).is_none());
    for call_id in [String::new(), "x".repeat(513), "x\n".into()] {
        assert!(bounded_card_result(
            &call_id,
            &request(ToolName::GetAbility, json!({"ability":1})),
            card(Vec::new())
        )
        .is_err());
    }
}

#[test]
fn selectors_are_rejected_at_the_typed_call_boundary() {
    for (name, arguments) in [
        (ToolName::GetItem, json!({"item":0})),
        (ToolName::GetItem, json!({"item":"Toxic Bullets"})),
        (ToolName::GetAbility, json!({})),
        (ToolName::GetAbility, json!({"ability":1,"hero":2,"slot":3})),
        (ToolName::GetAbility, json!({"hero":2,"slot":0})),
        (ToolName::GetAbility, json!({"hero":2,"slot":5})),
        (
            ToolName::Compare,
            json!({"entities":[1,1],"stats":["weapon.damage"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["weapon.damage","weapon.damage"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["weapon.damage;"]}),
        ),
    ] {
        assert!(ToolCall {
            id: "invalid-selector".into(),
            name,
            arguments
        }
        .validate(&definitions())
        .is_err());
    }
}

#[test]
fn card_results_are_bound_to_the_requested_identity_and_current_selection() {
    let value = card(Vec::new());
    for arguments in [json!({"ability":2}), json!({"ability":1,"patch":"old"})] {
        assert!(bounded_card_result(
            "bound-card",
            &request(ToolName::GetAbility, arguments),
            value.clone()
        )
        .unwrap()
        .is_none());
    }
    let mut item = value;
    item["identity"]["kind"] = json!("item");
    assert!(bounded_card_result(
        "bound-card",
        &request(ToolName::GetItem, json!({"item":2})),
        item.clone()
    )
    .unwrap()
    .is_none());
    assert!(bounded_card_result(
        "bound-card",
        &request(ToolName::GetItem, json!({"item":1})),
        item
    )
    .unwrap()
    .is_some());
}

fn ability_link(hero: u64, ability: u64, slot: u8, citation: u64) -> Value {
    json!({"entity_id":hero,"kind":"has_ability","slot":slot,
        "from_entity_id":hero,"to_entity_id":ability,"citation":citation.to_string(),
        "patch":null,"source_revision":"assets-a","identity":identity(hero)})
}

#[test]
fn slot_selection_preserves_the_actual_receipt_not_an_unrelated_link() {
    let selection = request(ToolName::GetAbility, json!({"hero":2,"slot":3}));
    let selected = ability_link(2, 1, 3, 6);
    let mut unrelated = ability_link(3, 1, 1, 7);
    unrelated["identity"]["key"] = json!("x".repeat(MAX_RESULT_BYTES));
    let mut value = card(Vec::new());
    value["abilities"] = json!({"links":[unrelated,selected.clone()],
        "omitted_count":0,"truncated":false});
    let bounded = bounded_card_result("selected-link", &selection, value.clone())
        .unwrap()
        .unwrap();
    assert_eq!(bounded.result["abilities"]["links"], json!([selected]));
    assert_eq!(bounded.result["abilities"]["omitted_count"], 1);
    assert_eq!(bounded.evidence_ids, ["1", "2", "6"]);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
    for (pointer, replacement) in [
        ("/abilities/links/1/slot", json!(2)),
        ("/abilities/links/1/to_entity_id", json!(4)),
        ("/abilities/links/1/from_entity_id", json!(4)),
        ("/abilities/links/1/citation", Value::Null),
    ] {
        let mut malformed = value.clone();
        *malformed.pointer_mut(pointer).unwrap() = replacement;
        assert!(bounded_card_result("selected-link", &selection, malformed)
            .unwrap()
            .is_none());
    }
    value["abilities"]["links"]
        .as_array_mut()
        .unwrap()
        .push(ability_link(2, 4, 3, 8));
    assert!(bounded_card_result("selected-link", &selection, value)
        .unwrap()
        .is_none());
}

#[test]
fn item_cost_and_effect_facts_are_not_discarded_to_fit_the_budget() {
    let selection = request(ToolName::GetItem, json!({"item":1}));
    let mut cost = fact(1, "cost", 5);
    cost["value"] = json!(3000);
    cost["unit"] = json!("souls");
    let mut effect = fact(1, "description_passive", 6);
    effect["value"] = json!("Verringert eingehende Heilung.");
    let mut value = card(vec![cost.clone(), effect.clone()]);
    value["identity"]["kind"] = json!("item");
    let mut link = ability_link(2, 1, 3, 7);
    link["identity"]["key"] = json!("x".repeat(MAX_RESULT_BYTES));
    value["links"]["links"] = json!([link]);
    let bounded = bounded_card_result(&"\\".repeat(512), &selection, value.clone())
        .unwrap()
        .unwrap();
    assert_eq!(bounded.result["facts"]["facts"], json!([cost, effect]));
    assert_eq!(bounded.result["facts"]["omitted_count"], 0);
    assert_eq!(bounded.result["links"]["omitted_count"], 1);
    assert_eq!(bounded.evidence_ids, ["1", "5", "6"]);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_RESULT_BYTES);
    value["facts"]["facts"][1]["value"] = json!("x".repeat(MAX_RESULT_BYTES));
    assert!(bounded_card_result("oversized-item", &selection, value)
        .unwrap()
        .is_none());
}

#[test]
fn comparison_preserves_partial_and_missing_cell_status() {
    let selection = request(
        ToolName::Compare,
        json!({"entities":[1,2],"stats":["weapon.damage"]}),
    );
    let row = fact(1, "weapon.damage", 5);
    let value = json!({"entities":[identity(1),identity(2)],"stats":{"facts":[row.clone()]},
        "cells":[{"entity_id":1,"stat_key":"weapon.damage","total_count":2},
            {"entity_id":2,"stat_key":"weapon.damage","total_count":0}]});
    let bounded = bounded_comparison_result("partial-cell", &selection, value)
        .unwrap()
        .unwrap();
    assert_eq!(bounded.result["stats"]["facts"], json!([row]));
    assert_eq!(bounded.result["cells"][0]["status"], "partial");
    assert_eq!(bounded.result["cells"][0]["omitted_count"], 1);
    assert_eq!(bounded.result["cells"][1]["status"], "missing");
    assert_eq!(bounded.result["cells"][1]["omitted_count"], 0);
    assert_eq!(bounded.result["stats"]["omitted_count"], 1);
    assert_eq!(bounded.result["stats"]["truncated"], true);
    assert_eq!(bounded.evidence_ids, ["1", "2", "5"]);
}

#[test]
fn comparison_cannot_substitute_other_entities_stats_or_history() {
    let value = comparison();
    for arguments in [
        json!({"entities":[1,2,3,5],"stats":comparison_request().arguments()["stats"]}),
        json!({"entities":[1,2,3,4],"stats":["stats.other"]}),
        json!({"entities":[1,2,3,4],"stats":comparison_request().arguments()["stats"],"patch":"old"}),
    ] {
        assert!(bounded_comparison_result(
            "bound-grid",
            &request(ToolName::Compare, arguments),
            value.clone()
        )
        .unwrap()
        .is_none());
    }
}
