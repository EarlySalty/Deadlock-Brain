use std::collections::BTreeSet;

use brain_contracts::{
    tools::{canonical_patch_history_definition, CanonicalPatchHistoryRequest, ToolLanguage},
    Evidence, EvidenceKind, PinnedGameContext, SourceVisibility, ToolCall, ToolDefinition,
    ToolEvidenceDependency, ToolExecution, ToolName, ToolResult, ToolSubrequest, Usage,
};
use serde_json::{json, Value};

fn legacy_definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::PatchHistory,
        description: "Patchverlauf".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["entity"],"properties":{
            "entity":{"type":"object","additionalProperties":false,"required":["kind","id"],"properties":{
                "kind":{"type":"string","enum":["hero","ability","item"]},"id":{"type":"integer","minimum":1}}},
            "from_patch":{"type":"string"},"to_patch":{"type":"string"},
            "fields":{"type":"array","items":{"type":"string"}},
            "historical_client_versions":{"type":"array","items":{"type":"integer"}}}}),
    }
}

fn call(arguments: Value) -> ToolCall {
    ToolCall {
        id: "patch-call".into(),
        name: ToolName::PatchHistory,
        arguments,
    }
}

#[test]
fn numeric_canonical_ids_and_legacy_objects_have_distinct_requests() {
    let definition = canonical_patch_history_definition();
    definition.validate().unwrap();
    let request = call(json!({"entity":1}))
        .validate(std::slice::from_ref(&definition))
        .unwrap();
    assert_eq!(
        request.subrequest(),
        &ToolSubrequest::CanonicalPatchHistory(CanonicalPatchHistoryRequest {
            entity: 1,
            since: None,
            until: None,
            limit: None,
        })
    );
    for limit in [1, 20] {
        let call = call(
            json!({"entity":i64::MAX,"since":"ä".repeat(64),"until":"2026-10-01","limit":limit}),
        );
        let request = call.validate(std::slice::from_ref(&definition)).unwrap();
        assert_eq!(request.arguments(), &call.arguments);
        assert_eq!(
            request.subrequest(),
            &ToolSubrequest::CanonicalPatchHistory(CanonicalPatchHistoryRequest {
                entity: i64::MAX as u64,
                since: Some("ä".repeat(64)),
                until: Some("2026-10-01".into()),
                limit: Some(limit),
            })
        );
        assert_eq!(
            serde_json::to_value(request.subrequest()).unwrap()["name"],
            "canonical_patch_history"
        );
        assert_eq!(request.name(), ToolName::PatchHistory);
    }
    let request = call(json!({"entity":{"kind":"hero","id":1},"from_patch":"legacy","to_patch":"newer", "fields":["health"],"historical_client_versions":[6759]})).validate(&[legacy_definition()]).unwrap();
    let ToolSubrequest::PatchHistory(request) = request.subrequest() else {
        panic!("Legacy request changed")
    };
    assert_eq!(request.entity.id, 1);
    assert_eq!(request.historical_client_versions, [6759]);
    assert_eq!(request.from_patch.as_deref(), Some("legacy"));
    assert_eq!(request.to_patch.as_deref(), Some("newer"));
    assert_eq!(request.fields, ["health"]);
}

#[test]
fn canonical_bounds_missing_ids_and_mixed_modes_are_rejected() {
    let definition = canonical_patch_history_definition();
    for arguments in [
        json!({}),
        json!({"entity":0}),
        json!({"entity":-1}),
        json!({"entity":1.5}),
        json!({"entity":"1"}),
        json!({"entity":i64::MAX as u64+1}),
        json!({"entity":1,"limit":0}),
        json!({"entity":1,"limit":21}),
        json!({"entity":1,"limit":1.5}),
        json!({"entity":1,"since":" "}),
        json!({"entity":1,"until":"ä".repeat(65)}),
        json!({"entity":1,"since":"p\n"}),
        json!({"entity":1,"from_patch":"legacy"}),
        json!({"entity":1,"fields":["health"]}),
        json!({"entity":1,"historical_client_versions":[6759]}),
        json!({"entity":1,"url":"https://example.com"}),
    ] {
        assert!(
            call(arguments.clone())
                .validate(std::slice::from_ref(&definition))
                .is_err(),
            "{arguments}"
        );
    }
    assert!(call(json!({"entity":{"kind":"hero","id":1},"since":"p"}))
        .validate(&[legacy_definition()])
        .is_err());
    assert!(call(json!({"entity":{"kind":"hero","id":1}}))
        .validate(&[definition])
        .is_err());
}

#[test]
fn typed_canonical_validation_still_bounds_loose_schemas() {
    for arguments in [
        json!({"entity":0}),
        json!({"entity":i64::MAX as u64+1}),
        json!({"entity":1,"limit":0}),
        json!({"entity":1,"limit":21}),
        json!({"entity":1,"since":" "}),
        json!({"entity":1,"until":"ä".repeat(65)}),
        json!({"entity":1,"since":"p\n"}),
        json!({"entity":1,"from_patch":"legacy"}),
    ] {
        let properties = arguments
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let kind = if value.is_number() {
                    "integer"
                } else {
                    "string"
                };
                (key.clone(), json!({"type":kind}))
            })
            .collect::<serde_json::Map<_, _>>();
        let definition = ToolDefinition {
            input_schema: json!({"type":"object","additionalProperties":false,"properties":properties}),
            ..canonical_patch_history_definition()
        };
        assert!(
            call(arguments.clone()).validate(&[definition]).is_err(),
            "{arguments}"
        );
    }
}

fn execution(call: &ToolCall, request: &brain_contracts::ToolRequest) -> ToolExecution {
    ToolExecution {
        result: ToolResult {
            call_id: call.id.clone(),
            name: call.name,
            result: json!({"citation":"42"}),
            evidence_ids: vec!["42".into()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence: vec![Evidence {
                evidence_id: "42".into(),
                source_id: "canon.receipt".into(),
                logical_id: "receipt/42".into(),
                revision: 1,
                kind: EvidenceKind::Fact,
                content: json!({"citation":"42","patch":null}).to_string(),
                citation: "42".into(),
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

#[test]
fn only_canonical_patch_history_executions_are_pin_free() {
    for (arguments, definition, canonical) in [
        (
            json!({"entity":1}),
            canonical_patch_history_definition(),
            true,
        ),
        (
            json!({"entity":{"kind":"hero","id":1}}),
            legacy_definition(),
            false,
        ),
    ] {
        let call = call(arguments);
        let request = call.validate(&[definition]).unwrap();
        let mut execution = execution(&call, &request);
        let game = PinnedGameContext {
            client_version: 6759,
            language: ToolLanguage::German,
            mechanic_revision: "legacy".into(),
        };
        assert_eq!(
            execution.validate_for(&call, &request, None).is_ok(),
            canonical
        );
        assert_eq!(
            execution.validate_for(&call, &request, Some(&game)).is_ok(),
            canonical
        );
        execution.dependencies[0].game_context = Some(game.clone());
        assert_eq!(
            execution.validate_for(&call, &request, Some(&game)).is_ok(),
            !canonical
        );
        execution.dependencies[0].evidence.clear();
        assert!(execution
            .validate_for(&call, &request, Some(&game))
            .is_err());
    }
}

#[test]
fn legacy_tool_names_are_unchanged_and_definitions_remain_unique() {
    let names = [
        ToolName::EntityFind,
        ToolName::EntityProfile,
        ToolName::HeroCompare,
        ToolName::DamageCalculate,
        ToolName::PatchHistory,
        ToolName::BuildPlan,
        ToolName::GameRules,
        ToolName::ServerKnowledge,
        ToolName::DeadlockData,
    ];
    assert_eq!(names.len(), 9);
    for name in names {
        let encoded = serde_json::to_value(name).unwrap();
        assert_eq!(encoded, name.as_str());
        assert_eq!(serde_json::from_value::<ToolName>(encoded).unwrap(), name);
    }
    assert!(brain_contracts::tools::validate_definitions(&[
        legacy_definition(),
        canonical_patch_history_definition()
    ])
    .is_err());
}
