use std::collections::BTreeSet;

use brain_contracts::provider_input::{
    grounded_turn_input_ceiling, grounded_turn_payload, transport_input_ceiling, ToolWireFormat,
};
use brain_contracts::tools::{
    ToolAnalyticsSelection, ToolBoonRange, ToolGameRuleTopic, ToolProgression,
};
use brain_contracts::{
    Evidence, EvidenceKind, ModelBlock, PinnedGameContext, Query, SourceVisibility, ToolCall,
    ToolConversation, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolMessage, ToolName,
    ToolRequest, ToolResult, ToolSubrequest, Usage,
};
use serde_json::{json, Value};

fn schema(value: &Value) -> Value {
    match value {
        Value::Object(fields) => {
            let properties: serde_json::Map<_, _> = fields
                .iter()
                .map(|(key, value)| (key.clone(), schema(value)))
                .collect();
            json!({"type":"object","properties":properties,"required":fields.keys().collect::<Vec<_>>(),"additionalProperties":false})
        }
        Value::Array(values) => {
            json!({"type":"array","items":values.first().map_or(json!({"type":"integer"}), schema)})
        }
        Value::String(_) => json!({"type":"string"}),
        Value::Number(number) if number.is_i64() || number.is_u64() => json!({"type":"integer"}),
        Value::Number(_) => json!({"type":"number"}),
        Value::Bool(_) => json!({"type":"boolean"}),
        Value::Null => json!({"type":"null"}),
    }
}

fn definition(call: &ToolCall) -> ToolDefinition {
    ToolDefinition {
        name: call.name,
        description: call.name.as_str().into(),
        input_schema: schema(&call.arguments),
    }
}

fn call(name: ToolName, arguments: Value) -> ToolCall {
    ToolCall {
        id: name.as_str().into(),
        name,
        arguments,
    }
}

fn validate(call: &ToolCall) -> ToolRequest {
    call.validate(&[definition(call)]).unwrap()
}

fn scenario() -> Value {
    json!({
        "progression":{"kind":"boons","value":0},
        "ability_points":0,
        "total_spirit":0.0,
        "target":{
            "kind":"player",
            "values":{"health":0.0,"bullet_resist":-0.25,"spirit_resist":-0.5},
            "distance":{"value":0.0,"unit":"meters"},
            "hit_chance":0.0,
            "headshot_fraction":0.0,
            "states":[{"at_seconds":0.0,"values":{"health":0.0,"bullet_resist":-0.75}}]
        }
    })
}

fn analytics() -> Value {
    json!({
        "min_average_badge":0,
        "max_average_badge":116,
        "min_unix_timestamp":0,
        "max_unix_timestamp":86400
    })
}

fn extended_calls() -> Vec<ToolCall> {
    vec![
        call(
            ToolName::GameRules,
            json!({"topic":"resist_stacking","entity":{"kind":"hero","id":1},"game_time_seconds":0.0,"scenario":scenario()}),
        ),
        call(
            ToolName::HeroCompare,
            json!({"hero_ids":[1,2],"metrics":["weapon_dps"],"scenario":scenario(),"boon_range":{"min_boons":0,"max_boons":40},"analytics":analytics()}),
        ),
        call(
            ToolName::EntityProfile,
            json!({"entity":{"kind":"hero","id":1},"fields":["win_rate"],"analytics":analytics()}),
        ),
    ]
}

fn game() -> PinnedGameContext {
    PinnedGameContext {
        client_version: 6759,
        language: brain_contracts::tools::ToolLanguage::German,
        mechanic_revision: "contract-fixture".into(),
    }
}

fn execution(call: &ToolCall, request: &ToolRequest) -> ToolExecution {
    ToolExecution {
        result: ToolResult {
            call_id: call.id.clone(),
            name: call.name,
            result: json!({"status":"contract_fixture"}),
            evidence_ids: vec!["contract-evidence".into()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: Some(game()),
            evidence: vec![Evidence {
                evidence_id: "contract-evidence".into(),
                source_id: "contract-fixture".into(),
                logical_id: "contract-fixture".into(),
                revision: 1,
                kind: EvidenceKind::Fact,
                content: "Vertragsprobe ohne Spielwerte".into(),
                citation: "contract-fixture".into(),
                visibility: SourceVisibility::Public,
                allowed_scopes: BTreeSet::new(),
                score: 1.0,
                provenance: None,
                patch: None,
            }],
        }],
        usage: Usage::default(),
    }
}

fn query() -> Query {
    Query {
        request_id: "contract-request".into(),
        conversation_id: "contract-conversation".into(),
        text: "Vertragsprobe".into(),
        answer_context: None,
        domain: None,
        requested_scopes: BTreeSet::new(),
        profile: Default::default(),
        patch: None,
        mode: None,
    }
}

fn conversation(calls: &[ToolCall]) -> ToolConversation {
    ToolConversation {
        messages: vec![
            ToolMessage::Assistant {
                blocks: calls
                    .iter()
                    .cloned()
                    .map(|call| ModelBlock::ToolUse { call })
                    .collect(),
            },
            ToolMessage::ToolResults {
                results: calls
                    .iter()
                    .rev()
                    .map(|call| execution(call, &validate(call)).result)
                    .collect(),
            },
        ],
    }
}

#[test]
fn achter_werkzeugname_und_sechs_geschlossene_regelthemen() {
    let names = [
        ToolName::EntityFind,
        ToolName::EntityProfile,
        ToolName::HeroCompare,
        ToolName::DamageCalculate,
        ToolName::PatchHistory,
        ToolName::BuildPlan,
        ToolName::GameRules,
        ToolName::ServerKnowledge,
    ];
    assert_eq!(names.len(), 8);
    for name in names {
        assert_eq!(serde_json::to_value(name).unwrap(), name.as_str());
        assert_eq!(
            serde_json::from_value::<ToolName>(json!(name.as_str())).unwrap(),
            name
        );
    }
    for (topic, wire) in [
        (ToolGameRuleTopic::KillBounty, "kill_bounty"),
        (ToolGameRuleTopic::Comeback, "comeback"),
        (ToolGameRuleTopic::Urn, "urn"),
        (ToolGameRuleTopic::Midboss, "midboss"),
        (ToolGameRuleTopic::ResistStacking, "resist_stacking"),
        (ToolGameRuleTopic::Resources, "resources"),
    ] {
        let call = call(ToolName::GameRules, json!({"topic":wire}));
        let request = validate(&call);
        let ToolSubrequest::GameRules(rules) = request.subrequest() else {
            panic!("Falsche Unteranfrage");
        };
        assert_eq!(rules.topic, topic);
        assert_eq!(rules.entity, None);
        assert_eq!(rules.game_time_seconds, None);
        assert_eq!(rules.scenario, None);
        assert_eq!(request.arguments(), &call.arguments);
    }
    assert!(serde_json::from_value::<ToolName>(json!("rules_execute")).is_err());
    assert!(serde_json::from_value::<ToolGameRuleTopic>(json!("formula")).is_err());
}

#[test]
fn neue_argumente_bleiben_typisiert_einschliesslich_null_und_negativwerten() {
    let calls = extended_calls();
    let request = validate(&calls[0]);
    let ToolSubrequest::GameRules(rules) = request.subrequest() else {
        panic!("Falsche Unteranfrage");
    };
    assert_eq!(rules.entity.as_ref().unwrap().id, 1);
    assert_eq!(rules.game_time_seconds, Some(0.0));
    let scenario = rules.scenario.as_ref().unwrap();
    assert_eq!(scenario.progression, ToolProgression::Boons(0));
    assert_eq!(scenario.ability_points, Some(0));
    assert_eq!(scenario.total_spirit, Some(0.0));
    let target = scenario.target.as_ref().unwrap();
    assert_eq!(target.values.health, Some(0.0));
    assert_eq!(target.values.bullet_resist, Some(-0.25));
    assert_eq!(target.values.spirit_resist, Some(-0.5));
    assert_eq!(target.distance.as_ref().unwrap().value, 0.0);
    assert_eq!(target.hit_chance, Some(0.0));
    assert_eq!(target.headshot_fraction, Some(0.0));
    assert_eq!(target.states[0].at_seconds, 0.0);
    assert_eq!(target.states[0].values.bullet_resist, Some(-0.75));
    let expected = ToolAnalyticsSelection {
        min_average_badge: Some(0),
        max_average_badge: Some(116),
        min_unix_timestamp: 0,
        max_unix_timestamp: 86400,
    };
    let request = validate(&calls[1]);
    let ToolSubrequest::HeroCompare(compare) = request.subrequest() else {
        panic!("Falsche Unteranfrage");
    };
    assert_eq!(
        compare.boon_range,
        Some(ToolBoonRange {
            min_boons: 0,
            max_boons: 40
        })
    );
    assert_eq!(compare.analytics.as_ref(), Some(&expected));
    let request = validate(&calls[2]);
    let ToolSubrequest::EntityProfile(profile) = request.subrequest() else {
        panic!("Falsche Unteranfrage");
    };
    assert_eq!(profile.analytics.as_ref(), Some(&expected));
}

#[test]
fn alte_profil_und_vergleichsargumente_behalten_optionale_defaults() {
    for mut call in extended_calls().into_iter().skip(1) {
        call.arguments.as_object_mut().unwrap().remove("analytics");
        call.arguments.as_object_mut().unwrap().remove("boon_range");
        let request = validate(&call);
        let serialized = serde_json::to_value(request.subrequest()).unwrap();
        assert!(serialized["arguments"].get("analytics").is_none());
        assert!(serialized["arguments"].get("boon_range").is_none());
        match request.subrequest() {
            ToolSubrequest::HeroCompare(compare) => {
                assert_eq!(compare.boon_range, None);
                assert_eq!(compare.analytics, None);
            }
            ToolSubrequest::EntityProfile(profile) => assert_eq!(profile.analytics, None),
            _ => panic!("Falsche Unteranfrage"),
        }
    }
}

#[test]
fn boonbereich_hat_nur_strukturgrenzen_keine_erfundene_spieldomaene() {
    let mut call = extended_calls().remove(1);
    for (min, max) in [(0, 0), (40, 40), (0, u32::MAX)] {
        call.arguments["boon_range"] = json!({"min_boons":min,"max_boons":max});
        let request = validate(&call);
        let ToolSubrequest::HeroCompare(compare) = request.subrequest() else {
            panic!("Falsche Unteranfrage");
        };
        assert_eq!(
            compare.boon_range,
            Some(ToolBoonRange {
                min_boons: min,
                max_boons: max
            })
        );
    }
    for range in [
        json!({"min_boons":2,"max_boons":1}),
        json!({"min_boons":-1,"max_boons":40}),
        json!({"min_boons":0,"max_boons":u64::from(u32::MAX)+1}),
        json!({"min_boons":0,"max_boons":40.5}),
        json!({"min_boons":0}),
        json!({"min_boons":0,"max_boons":40,"step":10}),
    ] {
        call.arguments["boon_range"] = range;
        assert!(call.validate(&[definition(&call)]).is_err());
    }
}

#[test]
fn analytics_grenzen_stimmen_mit_dem_originalen_api_pin_ueberein() {
    let pin: Value = serde_json::from_str(include_str!(
        "../../dbrain-sources/tests/fixtures/external/openapi-20260925.json"
    ))
    .unwrap();
    for path in ["/v1/analytics/hero-stats", "/v1/analytics/item-stats"] {
        let parameters = pin["paths"][path]["get"]["parameters"].as_array().unwrap();
        for name in ["min_average_badge", "max_average_badge"] {
            let parameter = parameters.iter().find(|p| p["name"] == name).unwrap();
            assert_eq!(parameter["schema"]["minimum"], 0);
            assert_eq!(parameter["schema"]["maximum"], 116);
            assert_eq!(parameter["schema"]["format"], "int32");
        }
        for name in ["min_unix_timestamp", "max_unix_timestamp"] {
            let parameter = parameters.iter().find(|p| p["name"] == name).unwrap();
            assert_eq!(parameter["schema"]["format"], "int64");
        }
    }
    for mut call in extended_calls().into_iter().skip(1) {
        for (min, max) in [(Some(0), Some(0)), (Some(116), Some(116)), (None, None)] {
            call.arguments["analytics"] = json!({
                "min_average_badge":min,"max_average_badge":max,
                "min_unix_timestamp":0,"max_unix_timestamp":i64::MAX
            });
            validate(&call);
        }
        call.arguments["analytics"] = json!({"min_unix_timestamp":0,"max_unix_timestamp":3600});
        validate(&call);
    }
}

#[test]
fn rang_und_zeitfehler_werden_in_profil_und_vergleich_abgewiesen() {
    for mut call in extended_calls().into_iter().skip(1) {
        for (field, value) in [
            ("min_average_badge", json!(-1)),
            ("max_average_badge", json!(117)),
            ("min_average_badge", json!(117)),
            ("min_average_badge", json!(1.5)),
            ("min_unix_timestamp", json!(-1)),
            ("max_unix_timestamp", json!(0)),
            ("max_unix_timestamp", json!(-1)),
            ("max_unix_timestamp", json!(1.5)),
            ("max_unix_timestamp", json!(u64::MAX)),
            ("rank", json!("ascendant")),
        ] {
            call.arguments["analytics"] = analytics();
            call.arguments["analytics"][field] = value;
            assert!(call.validate(&[definition(&call)]).is_err(), "{field}");
        }
        for selection in [
            json!({"min_average_badge":116,"max_average_badge":0,"min_unix_timestamp":0,"max_unix_timestamp":86400}),
            json!({"min_unix_timestamp":86400,"max_unix_timestamp":86400}),
            json!({"min_unix_timestamp":86400,"max_unix_timestamp":3600}),
            json!({"min_average_badge":0,"max_average_badge":116}),
        ] {
            call.arguments["analytics"] = selection;
            assert!(call.validate(&[definition(&call)]).is_err());
        }
    }
}

#[test]
fn regeln_verwerfen_freie_felder_themen_und_ungueltige_szenarien() {
    let mut call = extended_calls().remove(0);
    let original = call.arguments.clone();
    for (field, value) in [
        ("topic", json!("formula")),
        ("topic", json!("")),
        ("game_time_seconds", json!(-1.0)),
        ("game_time_seconds", json!("0")),
        ("entity", json!({"kind":"hero","id":0})),
        ("entity", json!({"kind":"hero","id":1,"parameters":{}})),
        (
            "scenario",
            json!({"progression":{"kind":"boons","value":-1}}),
        ),
        (
            "scenario",
            json!({"progression":{"kind":"boons","value":0},"parameters":{}}),
        ),
        ("parameters", json!({"multiplier":10})),
    ] {
        call.arguments = original.clone();
        call.arguments[field] = value;
        assert!(call.validate(&[definition(&call)]).is_err(), "{field}");
    }
    call.arguments = json!({"game_time_seconds":0.0});
    assert!(call.validate(&[definition(&call)]).is_err());
}

#[test]
fn serverbindungen_und_ausfuehrbare_argumente_bleiben_gesperrt() {
    for original in extended_calls() {
        for field in [
            "client_version",
            "mechanic_revision",
            "patch_membership",
            "actor_id",
            "expression",
            "sql",
            "url",
        ] {
            let mut call = original.clone();
            call.arguments[field] = json!("untrusted");
            assert!(definition(&call).validate().is_err());
            assert!(call.validate(&[definition(&call)]).is_err());
            if original.name != ToolName::GameRules {
                let mut nested = original.clone();
                nested.arguments["analytics"][field] = json!("untrusted");
                assert!(definition(&nested).validate().is_err());
                assert!(nested.validate(&[definition(&nested)]).is_err());
            }
        }
        for field in ["patch", "patch_label"] {
            let mut call = original.clone();
            call.arguments[field] = json!("untrusted");
            assert!(call.validate(&[definition(&call)]).is_err());
            if original.name != ToolName::GameRules {
                let mut nested = original.clone();
                nested.arguments["analytics"][field] = json!("untrusted");
                assert!(nested.validate(&[definition(&nested)]).is_err());
            }
        }
    }
}

#[test]
fn abhaengigkeiten_binden_jedes_neue_argument_ergebnis_und_serverpin() {
    for call in extended_calls() {
        let request = validate(&call);
        let execution = execution(&call, &request);
        assert!(execution
            .validate_for(&call, &request, Some(&game()))
            .is_ok());
        assert!(execution.validate_for(&call, &request, None).is_err());
        let serialized = serde_json::to_value(&execution.dependencies).unwrap();
        assert_eq!(serialized[0]["request"]["arguments"], call.arguments);
        for alternate in changed_arguments(&call) {
            let alternate_request = validate(&alternate);
            assert!(execution
                .validate_for(&alternate, &alternate_request, Some(&game()))
                .is_err());
            let mut stale = execution.clone();
            stale.dependencies[0].request = alternate_request;
            assert!(stale.validate_for(&call, &request, Some(&game())).is_err());
        }
        let mut changed_game = game();
        changed_game.mechanic_revision = "other-revision".into();
        assert!(execution
            .validate_for(&call, &request, Some(&changed_game))
            .is_err());
        let mut changed_game = game();
        changed_game.client_version += 1;
        assert!(execution
            .validate_for(&call, &request, Some(&changed_game))
            .is_err());
        let mut forged = execution.clone();
        forged.result.call_id = "other-call".into();
        assert!(forged.validate_for(&call, &request, Some(&game())).is_err());
        let mut forged = execution;
        forged.result.name = ToolName::BuildPlan;
        assert!(forged.validate_for(&call, &request, Some(&game())).is_err());
    }
}

fn changed_arguments(original: &ToolCall) -> Vec<ToolCall> {
    let mut changed = Vec::new();
    let paths: Vec<Vec<&str>> = if original.name == ToolName::GameRules {
        vec![
            vec!["topic"],
            vec!["entity", "id"],
            vec!["game_time_seconds"],
            vec!["scenario", "total_spirit"],
        ]
    } else {
        let mut paths = vec![
            vec!["analytics", "min_average_badge"],
            vec!["analytics", "max_average_badge"],
            vec!["analytics", "min_unix_timestamp"],
            vec!["analytics", "max_unix_timestamp"],
        ];
        if original.name == ToolName::HeroCompare {
            paths.push(vec!["boon_range", "min_boons"]);
            paths.push(vec!["boon_range", "max_boons"]);
        }
        paths
    };
    for path in paths {
        let mut call = original.clone();
        let mut value = &mut call.arguments;
        for field in &path {
            value = &mut value[*field];
        }
        *value = match *path.last().unwrap() {
            "topic" => json!("resources"),
            "id" => json!(2),
            "max_average_badge" => json!(115),
            "max_unix_timestamp" => json!(90000),
            "max_boons" => json!(41),
            _ => json!(1),
        };
        changed.push(call);
    }
    changed
}

#[test]
fn beide_wireformen_erhalten_neue_argumente_und_zaehlen_jede_runde_vollstaendig() {
    let calls = extended_calls();
    let definitions: Vec<_> = calls.iter().map(definition).collect();
    let conversation = conversation(&calls);
    for format in [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible] {
        let payload =
            grounded_turn_payload(&query(), &[], &definitions, &conversation, format).unwrap();
        let count = grounded_turn_input_ceiling(&query(), &[], &definitions, &conversation, format)
            .unwrap();
        let expected: u64 = if format == ToolWireFormat::Native {
            64 + 16 + "system".len() as u64 + payload["system"].as_str().unwrap().len() as u64
        } else {
            64
        } + payload["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tool| tool.to_string().len() as u64)
            .sum::<u64>()
            + payload["messages"]
                .as_array()
                .unwrap()
                .iter()
                .map(|message| {
                    let mut n = 16 + message["role"].as_str().unwrap().len() as u64;
                    match &message["content"] {
                        Value::String(s) => n += s.len() as u64,
                        Value::Array(blocks) => {
                            n += blocks
                                .iter()
                                .map(|block| block.to_string().len() as u64)
                                .sum::<u64>()
                        }
                        Value::Null => {}
                        _ => panic!("Falsche Drahtform"),
                    }
                    if let Some(value) = message.get("tool_calls") {
                        n += value.to_string().len() as u64;
                    }
                    for field in ["tool_call_id", "name"] {
                        if let Some(value) = message.get(field) {
                            n += field.len() as u64 + value.to_string().len() as u64;
                        }
                    }
                    n
                })
                .sum::<u64>()
            + ["tool_choice", "output_config", "response_format", "stop"]
                .iter()
                .filter_map(|field| payload.get(field))
                .map(|value| value.to_string().len() as u64)
                .sum::<u64>();
        assert_eq!(count, expected);
        assert_eq!(count, transport_input_ceiling(&payload, true).unwrap());
        let assistant = usize::from(format != ToolWireFormat::Native) + 1;
        for (index, call) in calls.iter().enumerate() {
            if format == ToolWireFormat::Native {
                let block = &payload["messages"][assistant]["content"][index];
                assert_eq!(block["input"], call.arguments);
                assert_eq!(block["name"], call.name.as_str());
                assert_eq!(block["id"], call.id);
            } else {
                let block = &payload["messages"][assistant]["tool_calls"][index];
                assert_eq!(
                    serde_json::from_str::<Value>(block["function"]["arguments"].as_str().unwrap())
                        .unwrap(),
                    call.arguments
                );
                assert_eq!(block["function"]["name"], call.name.as_str());
                assert_eq!(block["id"], call.id);
            }
        }
        let mut repeated = conversation.clone();
        let mut second_calls = calls.clone();
        for call in &mut second_calls {
            call.id.push_str("-second");
        }
        repeated
            .messages
            .extend(self::conversation(&second_calls).messages);
        let first = grounded_turn_input_ceiling(
            &query(),
            &[],
            &definitions,
            &ToolConversation::default(),
            format,
        )
        .unwrap();
        let twice =
            grounded_turn_input_ceiling(&query(), &[], &definitions, &repeated, format).unwrap();
        assert_eq!(twice - count, count - first + (2 * calls.len() * 7) as u64);
    }
}

#[test]
fn rohe_wireformen_verwerfen_gefaelschte_ids_namen_und_argumente() {
    let calls = extended_calls();
    let definitions: Vec<_> = calls.iter().map(definition).collect();
    let conversation = conversation(&calls);
    for format in [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible] {
        let payload =
            grounded_turn_payload(&query(), &[], &definitions, &conversation, format).unwrap();
        let assistant = usize::from(format != ToolWireFormat::Native) + 1;
        let last = payload["messages"].as_array().unwrap().len() - 1;
        let mut forged = payload.clone();
        let mut invalid_arguments = calls[0].arguments.clone();
        invalid_arguments["topic"] = json!("formula");
        if format == ToolWireFormat::Native {
            forged["messages"][assistant]["content"][0]["input"] = invalid_arguments;
        } else {
            forged["messages"][assistant]["tool_calls"][0]["function"]["arguments"] =
                json!(invalid_arguments.to_string());
        }
        assert!(transport_input_ceiling(&forged, true).is_err());
        let mut forged = payload.clone();
        if format == ToolWireFormat::Native {
            forged["messages"][last]["content"][0]["tool_use_id"] = json!("other-call");
        } else {
            forged["messages"][last]["tool_call_id"] = json!("other-call");
        }
        assert!(transport_input_ceiling(&forged, true).is_err());
        if format == ToolWireFormat::OpenAiCompatible {
            let mut forged = payload;
            forged["messages"][last]["name"] = json!("build_plan");
            assert!(transport_input_ceiling(&forged, true).is_err());
        }
    }
}
