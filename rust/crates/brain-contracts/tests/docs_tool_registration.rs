use brain_contracts::{
    canonical_receipt::{format_receipt, CanonicalReceipt},
    tools::{SearchDocsRequest, ToolSubrequest},
    ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolName, ToolResult, Usage,
};
use serde_json::json;

#[test]
fn search_docs_requires_canonical_section_result_binding_without_game_pin() {
    let call = ToolCall {
        id: "docs".into(),
        name: ToolName::SearchDocs,
        arguments: json!({"query":"Baba", "limit":1}),
    };
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Dokumentabschnitte mit Belegen suchen".into(),
        input_schema: SearchDocsRequest::input_schema(),
    };
    let request = call.validate(std::slice::from_ref(&definition)).unwrap();
    assert!(matches!(
        request.subrequest(),
        ToolSubrequest::SearchDocs(_)
    ));
    assert_eq!(serde_json::to_value(call.name).unwrap(), "search_docs");
    assert_eq!(call.name.as_str(), "search_docs");
    let evidence = format_receipt(&CanonicalReceipt {
        raw_receipt_id: 51,
        raw_object_id: 1,
        ingest_run_id: 1,
        source: "deadlock-docs".into(),
        source_revision: "7b0f0e645f026214f28b99dddbb18bd477bc2819".into(),
        path: "public/billy.html".into(),
        locator_kind: "byte_ranges".into(),
        locator: "[[0,42]]".into(),
        value_sha256: "ab".repeat(32),
        extractor: "docs-v1".into(),
        patch: None,
    })
    .unwrap();
    let mut execution = ToolExecution {
        result: ToolResult {
            call_id: call.id.clone(),
            name: call.name,
            result: json!({"sections":[{"citation":evidence.evidence_id,"excerpt":"Baba Early Game","patch":null}]}),
            evidence_ids: vec![evidence.evidence_id.clone()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence: vec![evidence.clone()],
        }],
        usage: Usage::default(),
    };
    assert!(execution.validate_for(&call, &request, None).is_err());
    execution.dependencies[0] = execution.dependencies[0]
        .clone()
        .with_canonical_result(&execution.result)
        .unwrap();
    execution.validate_for(&call, &request, None).unwrap();
    assert_eq!(
        CanonicalReceipt::from_evidence(&execution.dependencies[0].evidence[0])
            .unwrap()
            .raw_receipt_id,
        51
    );
    let mut changed = execution.clone();
    changed.result.result["sections"][0]["excerpt"] = json!("Invented advice");
    assert!(changed.validate_for(&call, &request, None).is_err());
    let mut changed = execution.clone();
    changed.result.result["sections"][0]["citation"] = json!("51");
    assert!(changed.validate_for(&call, &request, None).is_err());
    let mut forbidden = call.clone();
    forbidden.arguments["scope"] = json!("internal");
    assert!(forbidden.validate(&[definition]).is_err());
}
