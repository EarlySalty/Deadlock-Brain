use super::*;
use brain_contracts::Principal;

fn request() -> ToolRequest {
    ToolCall {
        id: "patch-call".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":7}),
    }
    .validate(&[canonical_patch_history_definition()])
    .unwrap()
}

fn receipt_row(id: i64, padding: usize) -> Value {
    json!({"receipt_id":id,"source_id":"official-patchnotes","evidence_revision":1,
        "receipt":{"citation":id.to_string(),"raw_object_id":9,"ingest_run_id":1,"source":"official-patchnotes",
            "source_revision":"feed1","path":format!("post/9{}", "x".repeat(padding)),"content_sha256":"ab".repeat(32),
            "locator_kind":"json_pointer","locator":"/raw_content","value_sha256":"cd".repeat(32),
            "extractor":"patchnotes","patch":null}})
}

fn receipt(id: i64, padding: usize) -> Evidence {
    receipt_evidence(receipt_row(id, padding)).unwrap()
}

fn change(id: i64, statement: &str) -> Value {
    json!({"id":id,"entity_id":7,"stat_key":null,"old":null,"new":null,
        "change_kind":"statement","line_text":statement,"patch":"p1","citation":id.to_string(),
        "head_citation":"100","label_citation":"101","released_at_citation":"102","url_citation":"103",
        "binding_citation":"104","entity_citation":"105","stat_citation":null,"source_revision":"feed1"})
}

fn patch_citations(id: i64) -> Value {
    json!({"id":9,"label":"p1","head_citation":id.to_string(),
        "label_citation":(id+1).to_string(),"released_at_citation":(id+2).to_string(),
        "url_citation":(id+3).to_string()})
}

fn history(rows: Vec<Value>) -> Value {
    json!({"current_official_patch":patch_citations(100),
        "range":{"since":null,"until":null},"changes":rows,"omitted_count":0,"truncated":false,
        "completeness":"unknown"})
}

#[test]
fn bounds_cover_the_whole_execution_and_preserve_complete_rows() {
    let statement = "Baba: vollständige Änderung mit Umlauten äöüß. ".repeat(12);
    let value = history((1..=20).map(|id| change(id, &statement)).collect());
    let evidence = (1..=20)
        .chain(100..=105)
        .map(|id| receipt(id, 200))
        .collect::<Vec<_>>();
    let execution = bound_execution(&"\"".repeat(512), &request(), value, &evidence).unwrap();
    assert!(!execution.result.is_error);
    ensure_size(&execution).unwrap();
    assert!(serde_json::to_vec(&execution).unwrap().len() < MAX_BYTES);
    let rows = execution.result.result["changes"].as_array().unwrap();
    assert!(!rows.is_empty() && rows.len() < 20);
    assert_eq!(
        execution.result.result["omitted_count"].as_u64().unwrap() as usize,
        20 - rows.len()
    );
    assert_eq!(execution.result.result["truncated"], true);
    assert!(rows.iter().all(|row| row["line_text"] == statement));
    assert_eq!(execution.result.evidence_ids.len(), rows.len() + 6);
    assert_eq!(execution.dependencies[0].evidence.len(), rows.len() + 6);
    assert!(execution.dependencies[0]
        .evidence
        .iter()
        .all(|evidence| evidence.patch.is_none()));
}

#[test]
fn oversized_receipts_and_rows_never_leave_uncited_success() {
    for (statement, padding) in [
        ("x".repeat(MAX_BYTES), 0),
        ("Baba: health changed".into(), MAX_BYTES),
    ] {
        let execution = bound_execution(
            "p1",
            &request(),
            history(vec![change(1, &statement)]),
            &std::iter::once(receipt(1, padding))
                .chain((100..=105).map(|id| receipt(id, 0)))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        assert!(execution.result.is_error);
        assert_eq!(execution.result.result["error"], "result_too_large");
        assert!(execution.result.evidence_ids.is_empty());
        assert!(execution.dependencies.is_empty());
        ensure_size(&execution).unwrap();
    }
    assert!(bound_execution(
        "p1",
        &request(),
        history(vec![change(1, "Baba: health changed")]),
        &[receipt(100, 0)]
    )
    .is_err());
}

#[test]
fn citations_ignore_data_payloads_but_include_all_official_heads() {
    let mut value = history(vec![change(1, "Baba: health changed")]);
    value["changes"][0]["old"] = json!({"citation":"999"});
    value["changes"][0]["new"] = json!({"head_citation":"not-a-receipt"});
    value["range"]["since"] = patch_citations(110);
    value["range"]["until"] = patch_citations(120);
    assert_eq!(
        receipt_ids(&value).unwrap(),
        (100..=105)
            .chain(110..=113)
            .chain(120..=123)
            .chain([1])
            .map(|id| id.to_string())
            .collect()
    );
    value["changes"][0]["citation"] = json!("not-a-receipt");
    assert!(receipt_ids(&value).is_err());
}

#[test]
fn displayed_metadata_and_binding_citations_cannot_be_missing() {
    for field in [
        "head_citation",
        "label_citation",
        "released_at_citation",
        "url_citation",
        "binding_citation",
        "entity_citation",
    ] {
        let mut value = history(vec![change(1, "Baba: health changed")]);
        value["changes"][0][field] = Value::Null;
        assert!(receipt_ids(&value).is_err());
    }
    for field in [
        "head_citation",
        "label_citation",
        "released_at_citation",
        "url_citation",
    ] {
        let mut value = history(vec![change(1, "Baba: health changed")]);
        value["current_official_patch"][field] = Value::Null;
        assert!(receipt_ids(&value).is_err());
    }
}

#[test]
fn receipt_identity_does_not_depend_on_the_requested_patch() {
    let proofs = (100..=105)
        .map(|id| receipt(id, 0))
        .chain([receipt(1, 0)])
        .collect::<Vec<_>>();
    let first = bound_execution(
        "p1",
        &request(),
        history(vec![change(1, "Baba: health changed")]),
        &proofs,
    )
    .unwrap();
    let mut changed = history(vec![change(1, "Baba: health changed")]);
    changed["changes"][0]["patch"] = json!("other-historical-patch");
    let second = bound_execution("p2", &request(), changed, &proofs).unwrap();
    assert_eq!(
        first.dependencies[0].evidence,
        second.dependencies[0].evidence
    );
    let mut row = receipt_row(1, 0);
    row["receipt"]["patch"] = json!("p1");
    assert!(receipt_evidence(row.clone()).is_err());
    row["receipt"]["patch"] = Value::Null;
    row["receipt"]["value_sha256"] = json!("invalid");
    assert!(receipt_evidence(row).is_err());
}

#[test]
fn normalization_still_requires_the_raw_content_hash() {
    let evidence = receipt(1, 0);
    assert_eq!(evidence.source_id, "canon.source/official-patchnotes");
    let content: Value = serde_json::from_str(&evidence.content).unwrap();
    assert!(content.get("content_sha256").is_none());
    for hash in [Value::Null, json!("invalid"), json!("gg".repeat(32))] {
        let mut row = receipt_row(1, 0);
        row["receipt"]["content_sha256"] = hash;
        assert!(receipt_evidence(row).is_err());
    }
}

#[test]
fn canonical_formatter_remaps_only_authoritative_citations_and_seals_the_result() {
    let mut value = history(vec![change(1, "Baba: health changed")]);
    value["changes"][0]["old"] = json!({"citation":"999"});
    value["changes"][0]["new"] = json!({"head_citation":"not-a-receipt"});
    value["changes"][0]["stat_citation"] = json!("106");
    value["range"]["since"] = patch_citations(110);
    value["range"]["until"] = patch_citations(110);
    let ids = receipt_ids(&value).unwrap();
    let proofs = ids
        .iter()
        .map(|id| receipt(id.parse().unwrap(), 0))
        .collect::<Vec<_>>();
    let original = value.clone();
    let execution = bound_execution("patch-call", &request(), value, &proofs).unwrap();
    assert!(!execution.result.is_error);
    let mapped = &execution.result.result;
    for (before, after) in [
        (
            &original["current_official_patch"],
            &mapped["current_official_patch"],
        ),
        (&original["range"]["since"], &mapped["range"]["since"]),
        (&original["range"]["until"], &mapped["range"]["until"]),
    ] {
        for field in [
            "head_citation",
            "label_citation",
            "released_at_citation",
            "url_citation",
        ] {
            let id = before[field].as_str().unwrap().parse().unwrap();
            assert_eq!(after[field], receipt(id, 0).evidence_id);
        }
    }
    for field in [
        "citation",
        "head_citation",
        "label_citation",
        "released_at_citation",
        "url_citation",
        "binding_citation",
        "entity_citation",
        "stat_citation",
    ] {
        let id = original["changes"][0][field]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(mapped["changes"][0][field], receipt(id, 0).evidence_id);
    }
    assert_eq!(
        execution.result.result["changes"][0]["old"]["citation"],
        "999"
    );
    assert_eq!(
        execution.result.result["changes"][0]["new"]["head_citation"],
        "not-a-receipt"
    );
    assert_eq!(
        execution
            .result
            .evidence_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        proofs
            .iter()
            .map(|evidence| evidence.evidence_id.clone())
            .collect()
    );
    assert!(execution
        .result
        .evidence_ids
        .iter()
        .all(|id| id.starts_with(brain_contracts::canonical_receipt::RECEIPT_ID_PREFIX)));
    execution
        .validate_for(
            &ToolCall {
                id: "patch-call".into(),
                name: ToolName::PatchHistory,
                arguments: json!({"entity":7}),
            },
            &request(),
            None,
        )
        .unwrap();
    let mut tampered = execution.clone();
    tampered.result.result["changes"][0]["citation"] = json!("1");
    assert!(tampered
        .validate_for(
            &ToolCall {
                id: "patch-call".into(),
                name: ToolName::PatchHistory,
                arguments: json!({"entity":7}),
            },
            &request(),
            None
        )
        .is_err());
    ensure_size(&execution).unwrap();
}

#[test]
fn missing_or_ambiguous_patches_and_historical_mapping_have_no_evidence() {
    assert_eq!(
        select_patch(Vec::new()).error_code(),
        Some("patch_not_found")
    );
    assert_eq!(
        select_patch(vec![json!({"id":1}), json!({"id":2})]).error_code(),
        Some("patch_ambiguous")
    );
    assert!(matches!(
        select_patch(vec![json!({"id":1})]),
        PatchResolution::Found(_)
    ));
    let execution = historical_patch_unavailable("h1", ToolName::EntityProfile);
    assert!(execution.result.is_error);
    assert!(execution.dependencies.is_empty());
    assert!(execution.result.evidence_ids.is_empty());
    assert_eq!(
        execution.result.result["error"],
        "historical_patch_unavailable"
    );
}

#[test]
fn incomplete_patch_bounds_cannot_turn_into_unbounded_queries() {
    let patch = json!({"id":1,"label":"p1","released_at":"2026-10-01T12:00:00+00:00","head_citation":"100"});
    assert_eq!(
        patch_timestamp(&patch).unwrap(),
        "2026-10-01T12:00:00+00:00"
    );
    for (field, value) in [
        ("id", json!(0)),
        ("label", json!("")),
        ("released_at", Value::Null),
        ("released_at", json!("")),
        ("head_citation", json!("not-a-receipt")),
    ] {
        let mut invalid = patch.clone();
        invalid[field] = value;
        assert!(patch_timestamp(&invalid).is_err());
    }
}

fn query_and_context() -> (Query, AuthorizedContext) {
    (
        Query {
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: "Baba patches".into(),
            answer_context: None,
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: Default::default(),
            patch: None,
            mode: None,
        },
        AuthorizedContext {
            principal: Principal {
                actor_id: "operator".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            discord: None,
            conversation_id: "c1".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 1000,
            budget: Default::default(),
            request_deadline: None,
        },
    )
}

#[test]
fn public_scope_and_matching_conversation_are_required() {
    let (query, context) = query_and_context();
    check(&query, &context).unwrap();
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        check(&query, &denied),
        Err(PortError::PermissionDenied(_))
    ));
    let mut foreign = query;
    foreign.conversation_id = "c2".into();
    assert!(check(&foreign, &context).is_err());
}

#[tokio::test(start_paused = true)]
async fn invalid_requests_and_dependencies_stop_before_database_access() {
    let (query, context) = query_and_context();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy_with(sqlx::postgres::PgConnectOptions::new_without_pgpass());
    let dependency = ToolEvidenceDependency {
        request: request(),
        game_context: None,
        evidence: Vec::new(),
    };
    assert!(
        validate_patch_dependency(&pool, &query, &context, &dependency)
            .await
            .is_err()
    );
    let mut denied = context;
    denied.principal.provider_egress.clear();
    assert!(matches!(
        execute_patch_history(&pool, &query, &denied, "p1", &request()).await,
        Err(PortError::PermissionDenied(_))
    ));
}
