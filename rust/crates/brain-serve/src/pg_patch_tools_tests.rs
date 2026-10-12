use super::*;
use brain_contracts::{tools::ToolLanguage, Principal};

fn query() -> Query {
    Query {
        request_id: "r1".into(),
        conversation_id: "c1".into(),
        text: "Was änderte sich für Baba?".into(),
        answer_context: None,
        domain: None,
        requested_scopes: BTreeSet::new(),
        profile: Default::default(),
        patch: None,
        mode: None,
    }
}

fn context() -> AuthorizedContext {
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
    }
}

fn row(id: i64, statement: String) -> Value {
    json!({"id":id,"entity_id":7,"stat_key":null,"change_key":format!("line/{id}"),"old":null,"new":null,
        "change_kind":"statement","line_text":statement,"patch":"p1","citation":id.to_string(),"source_revision":"feed1"})
}

#[test]
fn canonical_patch_scope_and_conversation_guard_matches_p1() {
    let query = query();
    let context = context();
    assert!(PgPatchTools::<()>::check(&query, &context).is_ok());
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        PgPatchTools::<()>::check(&query, &denied),
        Err(PortError::PermissionDenied(_))
    ));
    denied.principal.provider_egress.insert("private".into());
    assert!(PgPatchTools::<()>::check(&query, &denied).is_err());
    let mut foreign = query;
    foreign.conversation_id = "c2".into();
    assert!(PgPatchTools::<()>::check(&foreign, &context).is_err());
}

#[test]
fn canonical_definition_is_unique_closed_and_numeric() {
    let definition = definition();
    definition.validate().unwrap();
    let call = ToolCall {
        id: "p1".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":7,"limit":20}),
    };
    assert!(matches!(
        call.validate(std::slice::from_ref(&definition))
            .unwrap()
            .subrequest(),
        ToolSubrequest::CanonicalPatchHistory(_)
    ));
    for arguments in [
        json!({"entity":{"kind":"hero","id":7}}),
        json!({"entity":7,"url":"https://example.com"}),
        json!({"entity":7,"limit":21}),
    ] {
        assert!(ToolCall {
            arguments,
            ..call.clone()
        }
        .validate(std::slice::from_ref(&definition))
        .is_err());
    }
}

#[test]
fn missing_and_ambiguous_patch_resolution_has_no_evidence() {
    let missing = select_patch(Vec::new());
    assert_eq!(missing.error_code(), Some("patch_not_found"));
    let ambiguous = select_patch(vec![
        json!({"id":9,"label":"p1"}),
        json!({"id":1,"label":"p1"}),
    ]);
    assert_eq!(ambiguous.error_code(), Some("patch_ambiguous"));
    let error = error_execution("p1", ToolName::PatchHistory, missing.error_code().unwrap());
    assert!(error.result.is_error);
    assert!(error.dependencies.is_empty());
    assert!(error.result.evidence_ids.is_empty());
    assert!(matches!(
        select_patch(vec![json!({"id":1})]),
        PatchResolution::Found(_)
    ));
}

#[test]
fn bounding_prunes_complete_rows_and_never_shortens_statements() {
    let statement = "Baba: vollständige Änderung mit Umlauten äöüß. ".repeat(20);
    let rows = (1..=20)
        .map(|id| row(id, statement.clone()))
        .collect::<Vec<_>>();
    let result = bound_result(
        &"\"".repeat(512),
        ToolName::PatchHistory,
        json!({"changes":rows,"omitted_count":0,"truncated":false}),
    )
    .unwrap();
    assert!(!result.is_error);
    assert!(serde_json::to_vec(&result).unwrap().len() <= MAX_BYTES);
    let rows = result.result["changes"].as_array().unwrap();
    assert!(!rows.is_empty() && rows.len() < 20);
    assert_eq!(
        result.result["omitted_count"].as_u64().unwrap() as usize,
        20 - rows.len()
    );
    assert_eq!(result.result["truncated"], true);
    assert!(rows.iter().all(|row| row["line_text"] == statement));
    let ids = rows
        .iter()
        .map(|row| row["citation"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(ids, result.evidence_ids.iter().cloned().collect());
    let oversized = bound_result(
        "p1",
        ToolName::PatchHistory,
        json!({"changes":[row(1,"x".repeat(MAX_BYTES))],"omitted_count":0}),
    )
    .unwrap();
    assert!(oversized.is_error);
    assert!(oversized.evidence_ids.is_empty());
}

#[test]
fn patch_receipt_evidence_is_stable_and_preserves_p1_metadata() {
    let content = json!({"citation":"42","source_id":1,"revision_id":2,"raw_object_id":3,"source_revision":"feed1",
        "locator_kind":"json_pointer","locator":"/raw_content","value_sha256":"ab".repeat(32),
        "extractor":"brain-job","extractor_version":"1","patch":null});
    let first = receipt_evidence(content.clone()).unwrap();
    let second = receipt_evidence(content.clone()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.patch, None);
    assert_eq!(first.content, content.to_string());
    assert_eq!(first.source_id, "canon.source/1");
    assert_eq!(first.logical_id, "receipt/42");
    assert_eq!(first.revision, 2);
    let ledger = serde_json::from_str::<Value>(&first.content).unwrap();
    assert_eq!(ledger["patch"], Value::Null);
    assert_eq!(ledger["raw_object_id"], 3);
    let statement = row(42, "Baba: health increased from 600 to 650".into());
    let call = ToolCall {
        id: "p1".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":7}),
    };
    let request = call.validate(&[definition()]).unwrap();
    let execution = ToolExecution {
        result: ToolResult {
            call_id: call.id.clone(),
            name: call.name,
            result: json!({"changes":[statement]}),
            evidence_ids: vec!["42".into()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence: vec![first],
        }],
        usage: Usage::default(),
    };
    let game = PinnedGameContext {
        client_version: 6759,
        language: ToolLanguage::German,
        mechanic_revision: "legacy".into(),
    };
    assert!(execution.validate_for(&call, &request, Some(&game)).is_ok());
    let mut inconsistent = content;
    inconsistent["patch"] = json!("p1");
    assert!(receipt_evidence(inconsistent).is_err());
}

#[test]
fn empty_history_reports_a_gap_without_claiming_unchanged_values() {
    let execution = empty_execution(
        "p1",
        ToolName::PatchHistory,
        json!({"current_official_patch":{"id":1,"label":"p1"},"changes":[],"error":"no_patch_changes","completeness":"unknown"}),
    );
    assert!(execution.result.is_error);
    assert!(execution.dependencies.is_empty());
    assert!(execution.result.evidence_ids.is_empty());
    assert_eq!(
        execution.result.result["current_official_patch"]["label"],
        "p1"
    );
    assert_eq!(execution.result.result["completeness"], "unknown");
}

#[test]
fn historical_profile_bounds_keep_mapping_receipts_separate() {
    let fact = json!({"citation":"42","patch":"p1","source_revision":"game1","condition":{},
        "value":"äöüß".repeat(300),"patch_mapping":{"citation":"43","official_citation":"44","patch":"p0"},
        "snapshot":{"citation":"45","official_citation":"46","patch":"p1"}});
    let value = json!({"identity":{"citation":"41","patch":"p1","facts":[]},
        "stats":{"facts":vec![fact.clone();20],"omitted_count":0}});
    let result = bound_profile_result(&"\"".repeat(512), ToolName::GetHero, value).unwrap();
    assert!(!result.is_error);
    assert!(serde_json::to_vec(&result).unwrap().len() <= MAX_BYTES);
    assert_eq!(result.evidence_ids, ["41", "42", "43", "44", "45", "46"]);
    assert!(result.result["stats"]["facts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| *row == fact));
    assert!(result.result["stats"]["omitted_count"].as_u64().unwrap() > 0);
    assert_eq!(result.result["stats"]["truncated"], true);
}

#[test]
fn fact_payloads_cannot_inject_receipt_dependencies() {
    let value = json!({"identity":{"citation":"41"},"stats":{"facts":[{
        "citation":"42","value":{"citation":"999"},"condition":{"citation":"not-a-receipt"},
        "patch_mapping":{"citation":"43","official_citation":"44"},
        "snapshot":{"citation":"45","official_citation":"46"}}]}});
    let result = bound_profile_result("h1", ToolName::GetHero, value).unwrap();
    assert_eq!(result.evidence_ids, ["41", "42", "43", "44", "45", "46"]);
    assert_eq!(
        result.result["stats"]["facts"][0]["value"]["citation"],
        "999"
    );
}

#[test]
fn official_head_corrections_change_history_and_profile_dependencies() {
    let history = json!({"current_official_patch":{"head_citation":"60"},
        "range":{"since":{"head_citation":"61"},"until":{"head_citation":"62"}},
        "changes":[{"citation":"42","head_citation":"63", "old":{"head_citation":"999"},
            "new":{"citation":"not-a-receipt"}}]});
    let first = bound_result("p1", ToolName::PatchHistory, history.clone()).unwrap();
    assert_eq!(first.evidence_ids, ["42", "60", "61", "62", "63"]);
    for key in ["since", "until"] {
        let mut corrected = history.clone();
        corrected["range"][key]["head_citation"] = json!("70");
        let result = bound_result("p1", ToolName::PatchHistory, corrected).unwrap();
        assert_ne!(first.evidence_ids, result.evidence_ids);
    }
    let mut corrected = history;
    corrected["current_official_patch"]["head_citation"] = json!("70");
    let result = bound_result("p1", ToolName::PatchHistory, corrected).unwrap();
    assert_ne!(first.evidence_ids, result.evidence_ids);
    let profile = json!({"requested_patch":{"head_citation":"60"},"identity":{"citation":"41"},
        "stats":{"facts":[{"citation":"42", "patch_mapping":{"citation":"43",
            "official_citation":"44","head_citation":"61"},
            "closing_mapping":{"citation":"45","official_citation":"46","head_citation":"62"},
            "snapshot":{"citation":"47","official_citation":"48","head_citation":"60"}}]}});
    let first = bound_profile_result("h1", ToolName::GetHero, profile.clone()).unwrap();
    assert_eq!(
        first.evidence_ids,
        ["41", "42", "43", "44", "45", "46", "47", "48", "60", "61", "62"]
    );
    for key in ["patch_mapping", "closing_mapping", "snapshot"] {
        let mut corrected = profile.clone();
        corrected["stats"]["facts"][0][key]["head_citation"] = json!("70");
        let result = bound_profile_result("h1", ToolName::GetHero, corrected).unwrap();
        assert_ne!(first.evidence_ids, result.evidence_ids);
    }
    let mut corrected = profile;
    corrected["requested_patch"]["head_citation"] = json!("70");
    let result = bound_profile_result("h1", ToolName::GetHero, corrected).unwrap();
    assert_ne!(first.evidence_ids, result.evidence_ids);
}

#[test]
fn canonical_wrapper_routes_only_new_shape_and_historical_hero() {
    let mut query = query();
    let canonical = ToolCall {
        id: "p1".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":7}),
    }
    .validate(&[definition()])
    .unwrap();
    assert!(handled(&query, &canonical));
    let hero = ToolCall {
        id: "h1".into(),
        name: ToolName::GetHero,
        arguments: json!({"hero":7}),
    }
    .validate(&[historical_definition()])
    .unwrap();
    assert!(!handled(&query, &hero));
    query.patch = Some("p1".into());
    assert!(handled(&query, &hero));
    query.patch = Some("current".into());
    assert!(!handled(&query, &hero));
    let historic = ToolCall {
        id: "h2".into(),
        name: ToolName::GetHero,
        arguments: json!({"hero":7,"patch":"p1"}),
    }
    .validate(&[historical_definition()])
    .unwrap();
    assert!(handled(&query, &historic));
}

struct Delegate;

impl ToolExecutionPort for Delegate {
    fn required_calls(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        Ok(vec![ToolCall {
            id: "h1".into(),
            name: ToolName::GetHero,
            arguments: json!({"hero":7}),
        }])
    }
    fn definitions(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(vec![historical_definition()])
    }
    fn execute(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        Ok(error_execution(id, request.name(), "delegate"))
    }
    fn execute_accounted(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: Option<&PinnedGameContext>,
        _id: &str,
        _request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        Err(PortFailure::accounted(
            PortError::Unavailable("delegate".into()),
            UsageAccounting {
                unaccounted: true,
                observed: Usage {
                    input_tokens: 17,
                    ..Usage::default()
                },
                ..UsageAccounting::default()
            },
        ))
    }
    fn validate_build_plan(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: &PinnedGameContext,
        _request: &BuildPlanRequest,
        _execution: &ToolExecution,
        _purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn validate_dependencies(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _pin: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        if dependencies.is_empty() {
            Ok(())
        } else {
            Err(PortError::Unavailable("delegate".into()))
        }
    }
}

#[test]
fn wrapper_keeps_delegate_accounting_and_required_calls_and_guards_revalidation() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let pool = {
        let _enter = runtime.enter();
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy_with(sqlx::postgres::PgConnectOptions::new_without_pgpass())
    };
    let tools = PgPatchTools::new(Delegate, pool, runtime.handle().clone());
    let query = query();
    let context = context();
    let calls = tools.required_calls(&query, &context, None).unwrap();
    assert_eq!(calls.len(), 1);
    let definitions = tools.definitions(&query, &context, None).unwrap();
    assert_eq!(definitions.len(), 2);
    brain_contracts::tools::validate_definitions(&definitions).unwrap();
    let request = calls[0].validate(&definitions).unwrap();
    assert_eq!(
        tools
            .execute(&query, &context, None, "h1", &request)
            .unwrap()
            .result
            .result["error"],
        "delegate"
    );
    let failure = tools
        .execute_accounted(&query, &context, None, "h1", &request)
        .unwrap_err();
    let accounting = failure.accounting.unwrap();
    assert!(accounting.unaccounted);
    assert_eq!(accounting.observed.input_tokens, 17);
    assert!(tools
        .validate_dependencies(&query, &context, None, &[], ToolValidationPurpose::Cache)
        .is_ok());
    let request = ToolCall {
        id: "p1".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":7}),
    }
    .validate(&definitions)
    .unwrap();
    let dependency = ToolEvidenceDependency {
        request: request.clone(),
        game_context: None,
        evidence: Vec::new(),
    };
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &[dependency],
            ToolValidationPurpose::Publication
        )
        .is_err());
    let mut denied = context;
    denied.principal.provider_egress.clear();
    assert!(matches!(
        tools.execute(&query, &denied, None, "p1", &request),
        Err(PortError::PermissionDenied(_))
    ));
}

#[derive(Default)]
struct TraceSink(std::sync::Mutex<Vec<Vec<String>>>);

impl super::super::answer_trace::TraceSink for TraceSink {
    fn write(&self, record: super::super::answer_trace::TraceRecord) -> Result<(), PortError> {
        self.0.lock().unwrap().push(record.tool_calls);
        Ok(())
    }
}

struct TraceProbe(super::super::answer_trace::TracedTools<PgPatchTools<Delegate>>);

impl brain_kernel::AnswerKernelPort for TraceProbe {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        let calls = self.0.required_calls(query, context, None).unwrap();
        let definitions = self.0.definitions(query, context, None).unwrap();
        let request = calls[0].validate(&definitions).unwrap();
        self.0
            .execute(query, context, None, "h1", &request)
            .unwrap();
        let request = ToolCall {
            id: "p1".into(),
            name: ToolName::PatchHistory,
            arguments: json!({"entity":7}),
        }
        .validate(&definitions)
        .unwrap();
        let mut denied = context.clone();
        denied.principal.provider_egress.clear();
        assert!(self
            .0
            .execute_accounted(query, &denied, None, "p1", &request)
            .is_err());
        brain_contracts::AnswerResponse {
            contract_version: brain_contracts::CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: brain_contracts::AnswerStatus::InsufficientEvidence,
            text: "Keine Belege verfügbar.".into(),
            citations: Vec::new(),
            usage: Usage::default(),
        }
    }
}

#[test]
fn append_only_from_traced_wiring_records_each_actual_call_once() {
    use super::super::answer_trace::{TracedKernel, TracedTools};
    use brain_kernel::AnswerKernelPort;
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let pool = {
        let _enter = runtime.enter();
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy_with(sqlx::postgres::PgConnectOptions::new_without_pgpass())
    };
    let tools = TracedTools(PgPatchTools::from_traced(
        TracedTools(Delegate),
        pool,
        runtime.handle().clone(),
    ));
    let sink = std::sync::Arc::new(TraceSink::default());
    let kernel = TracedKernel::new(TraceProbe(tools), Some(sink.clone()));
    kernel.answer(&query(), &context());
    let traces = sink.0.lock().unwrap();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0], ["get_hero", "patch_history"]);
}

#[path = "pg_patch_tools_pg_contract.rs"]
mod pg_contract;
