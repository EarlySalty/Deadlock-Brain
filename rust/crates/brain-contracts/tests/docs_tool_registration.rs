use brain_contracts::{
    tools::{SearchDocsRequest, ToolSubrequest},
    Evidence, EvidenceKind, SourceVisibility, ToolCall, ToolDefinition, ToolEvidenceDependency,
    ToolExecution, ToolName, ToolResult, Usage,
};
use serde_json::json;
use std::collections::BTreeSet;

#[test]
fn search_docs_is_a_typed_non_game_bound_tool() {
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
    let evidence = Evidence {
        evidence_id: "51".into(),
        source_id: "docs.public/1".into(),
        logical_id: "receipt/51".into(),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: "Baba Early Game".into(),
        citation: "https://github.com/EarlySalty/Deadlock-Docs/blob/7b0f0e645f026214f28b99dddbb18bd477bc2819/public/billy.html#L35-L38".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["docs.public".into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    let execution = ToolExecution {
        result: ToolResult {
            call_id: call.id.clone(),
            name: call.name,
            result: json!({"sections":[{"citation":"51","patch":null}]}),
            evidence_ids: vec!["51".into()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence: vec![evidence],
        }],
        usage: Usage::default(),
    };
    execution.validate_for(&call, &request, None).unwrap();
    let mut forbidden = call.clone();
    forbidden.arguments["scope"] = json!("internal");
    assert!(forbidden.validate(&[definition]).is_err());
}
