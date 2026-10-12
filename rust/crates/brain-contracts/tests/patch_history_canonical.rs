use std::collections::BTreeSet;

use brain_contracts::{
    tools::{CanonicalPatchHistoryRequest, ToolLanguage},
    Evidence, EvidenceKind, PinnedGameContext, SourceVisibility, ToolCall, ToolDefinition,
    ToolEvidenceDependency, ToolExecution, ToolName, ToolResult, ToolSubrequest, Usage,
};
use serde_json::{json, Value};

fn definition(entity: Value) -> ToolDefinition {
    ToolDefinition {
        name: ToolName::PatchHistory,
        description: "Patchverlauf".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["entity"],"properties":{
            "entity":entity,"since":{"type":"string","minLength":1,"maxLength":128},
            "until":{"type":"string","minLength":1,"maxLength":128},
            "limit":{"type":"integer","minimum":1,"maximum":20},
            "from_patch":{"type":"string"},"fields":{"type":"array","items":{"type":"string"}},
            "historical_client_versions":{"type":"array","items":{"type":"integer"}}}}),
    }
}

fn canonical() -> ToolDefinition {
    definition(json!({"type":"integer","minimum":1,"maximum":i64::MAX}))
}

fn legacy() -> ToolDefinition {
    definition(
        json!({"type":"object","additionalProperties":false,"required":["kind","id"],"properties":{
        "kind":{"type":"string","enum":["hero","ability","item"]},"id":{"type":"integer","minimum":1}}}),
    )
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
    for limit in [1, 20] {
        let call = call(
            json!({"entity":i64::MAX,"since":"2026-09-01","until":"2026-10-01","limit":limit}),
        );
        let request = call.validate(&[canonical()]).unwrap();
        assert_eq!(
            request.subrequest(),
            &ToolSubrequest::CanonicalPatchHistory(CanonicalPatchHistoryRequest {
                entity: i64::MAX as u64,
                since: Some("2026-09-01".into()),
                until: Some("2026-10-01".into()),
                limit: Some(limit),
            })
        );
        assert_eq!(
            serde_json::to_value(request.subrequest()).unwrap()["name"],
            "canonical_patch_history"
        );
    }
    let request = call(json!({"entity":{"kind":"hero","id":1},"from_patch":"legacy","historical_client_versions":[6759]})).validate(&[legacy()]).unwrap();
    let ToolSubrequest::PatchHistory(request) = request.subrequest() else {
        panic!("Legacy-Anfrage verändert")
    };
    assert_eq!(request.entity.id, 1);
    assert_eq!(request.historical_client_versions, [6759]);
    assert_eq!(request.from_patch.as_deref(), Some("legacy"));
}

#[test]
fn canonical_bounds_missing_ids_and_mixed_modes_are_rejected() {
    for arguments in [
        json!({}),
        json!({"entity":0}),
        json!({"entity":-1}),
        json!({"entity":1.5}),
        json!({"entity":"1"}),
        json!({"entity":i64::MAX as u64+1}),
        json!({"entity":1,"limit":0}),
        json!({"entity":1,"limit":21}),
        json!({"entity":1,"since":" "}),
        json!({"entity":1,"until":"ä".repeat(65)}),
        json!({"entity":1,"since":"p\n"}),
        json!({"entity":1,"from_patch":"legacy"}),
        json!({"entity":1,"fields":["health"]}),
        json!({"entity":1,"historical_client_versions":[6759]}),
    ] {
        assert!(
            call(arguments.clone()).validate(&[canonical()]).is_err(),
            "{arguments}"
        );
    }
    assert!(call(json!({"entity":{"kind":"hero","id":1},"since":"p"}))
        .validate(&[legacy()])
        .is_err());
    assert!(call(json!({"entity":{"kind":"hero","id":1}}))
        .validate(&[canonical()])
        .is_err());
}

#[test]
fn only_canonical_patch_history_executions_are_pin_free() {
    for (arguments, definition, canonical) in [
        (json!({"entity":1}), canonical(), true),
        (json!({"entity":{"kind":"hero","id":1}}), legacy(), false),
    ] {
        let call = call(arguments);
        let request = call.validate(&[definition]).unwrap();
        let evidence = Evidence {
            evidence_id: "42".into(),
            source_id: "canon.source/1".into(),
            logical_id: "receipt/42".into(),
            revision: 1,
            kind: EvidenceKind::Fact,
            content: json!({"patch":"p1","line_text":"Baba: health 600 -> 650","citation":"42"})
                .to_string(),
            citation: "42".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: Some("p1".into()),
        };
        let mut execution = ToolExecution {
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
                evidence: vec![evidence],
            }],
            usage: Usage::default(),
        };
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
    }
}
