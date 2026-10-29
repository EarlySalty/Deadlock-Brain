use super::*;
use brain_contracts::{Budget, Principal, ToolName, ToolResult, CONTRACT_VERSION};
use std::sync::{Barrier, Mutex};

fn query() -> Query {
    serde_json::from_value(json!({"request_id":"trace", "conversation_id":"trace", "text":"Synthetic private question", "profile":"explain"})).unwrap()
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: Default::default(),
            provider_egress: Default::default(),
        },
        conversation_id: "trace".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 1_000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    }
}

fn request() -> ToolRequest {
    ToolCall {id:"fixture".into(),name:ToolName::GetHero,arguments:json!({"hero":1})}.validate(&[ToolDefinition {
        name:ToolName::GetHero,description:"Fixture".into(),input_schema:json!({"type":"object","additionalProperties":false,"required":["hero"],"properties":{"hero":{"type":"integer","minimum":1}}})
    }]).unwrap()
}

#[derive(Default)]
struct Sink(Mutex<Vec<TraceRecord>>);
impl TraceSink for Sink {
    fn write(&self, record: TraceRecord) -> Result<(), PortError> {
        self.0.lock().unwrap().push(record);
        Ok(())
    }
}

fn evidence() -> Evidence {
    brain_contracts::canonical_receipt::format_receipt(&CanonicalReceipt {
        raw_receipt_id: 42,
        raw_object_id: 7,
        ingest_run_id: 3,
        source: "fixture".into(),
        source_revision: "r1".into(),
        path: "hero.json".into(),
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: "ab".repeat(32),
        extractor: "fixture-v1".into(),
        patch: None,
    })
    .unwrap()
}

struct ToolProbe;
impl ToolExecutionPort for ToolProbe {
    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        Ok(ToolExecution {
            result: ToolResult {
                call_id: id.into(),
                name: request.name(),
                result: json!({}),
                evidence_ids: vec![evidence().evidence_id, "not-a-receipt".into()],
                is_error: false,
            },
            dependencies: vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: None,
                evidence: vec![evidence()],
            }],
            usage: Usage::default(),
        })
    }
    fn validate_dependencies(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        _: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        self.execute(query, context, pin, "revalidation", &request())?;
        Ok(())
    }
}

struct Probe {
    methods: Mutex<Vec<&'static str>>,
    barrier: Option<Arc<Barrier>>,
}

impl Probe {
    fn run(
        &self,
        method: &'static str,
        query: &Query,
        context: &AuthorizedContext,
    ) -> AnswerResponse {
        self.methods.lock().unwrap().push(method);
        let tools = TracedTools(ToolProbe);
        let request = request();
        tools
            .execute_accounted(query, context, None, "one", &request)
            .unwrap();
        tools
            .validate_dependencies(
                query,
                context,
                None,
                &[],
                ToolValidationPurpose::Publication,
            )
            .unwrap();
        if let Some(barrier) = &self.barrier {
            barrier.wait();
        }
        AnswerResponse {
            contract_version: CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: if method.contains("publication") {
                AnswerStatus::UnauthorizedEvidence
            } else {
                AnswerStatus::Answered
            },
            text: "Synthetic private answer".into(),
            citations: Vec::new(),
            usage: Usage {
                input_tokens: 9,
                output_tokens: 4,
                cost_micros: 7,
                ..Default::default()
            },
        }
    }
    fn accounted(
        &self,
        method: &'static str,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        Accounted {
            value: self.run(method, query, context),
            accounting: UsageAccounting {
                observed: Usage {
                    input_tokens: 12,
                    output_tokens: 6,
                    cost_micros: 8,
                    ..Default::default()
                },
                reserved: Usage {
                    input_tokens: 13,
                    output_tokens: 7,
                    cost_micros: 10,
                    ..Default::default()
                },
                unaccounted: true,
            },
        }
    }
}
impl AnswerKernelPort for Probe {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.run("answer", query, context)
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.run("publication", query, context)
    }
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.accounted("accounted", query, context)
    }
    fn answer_for_publication_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.accounted("publication_accounted", query, context)
    }
}

#[test]
fn all_four_kernel_methods_forward_once_preserving_publication_and_accounting() {
    let sink = Arc::new(Sink::default());
    let kernel = TracedKernel::new(
        Probe {
            methods: Mutex::new(Vec::new()),
            barrier: None,
        },
        Some(sink.clone()),
    );
    let query = query();
    let context = context();
    assert_eq!(
        kernel.answer(&query, &context).status,
        AnswerStatus::Answered
    );
    assert_eq!(
        kernel.answer_for_publication(&query, &context).status,
        AnswerStatus::UnauthorizedEvidence
    );
    let accounted = kernel.answer_accounted(&query, &context);
    assert_eq!(accounted.value.usage.cost_micros, 7);
    assert_eq!(accounted.accounting.observed.cost_micros, 8);
    assert_eq!(accounted.accounting.reserved.cost_micros, 10);
    assert!(accounted.accounting.unaccounted);
    let published = kernel.answer_for_publication_accounted(&query, &context);
    assert_eq!(published.value.status, AnswerStatus::UnauthorizedEvidence);
    assert_eq!(published.accounting, accounted.accounting);
    assert_eq!(
        *kernel.inner.methods.lock().unwrap(),
        vec![
            "answer",
            "publication",
            "accounted",
            "publication_accounted"
        ]
    );
    let records = sink.0.lock().unwrap();
    assert_eq!(records.len(), 4);
    for record in records.iter() {
        assert_eq!(
            record.question_sha256,
            <[u8; 32]>::from(Sha256::digest(query.text.as_bytes()))
        );
        assert_eq!(record.tool_calls, vec!["get_hero"]);
        assert_eq!(record.citation_ids, vec!["42"]);
        assert!(!record.tokens.to_string().contains(&query.text));
    }
    assert_eq!(records[0].status, "answered");
    assert_eq!(records[1].status, "unauthorized_evidence");
    assert_eq!(records[0].tokens["input_tokens"], 9);
    assert_eq!(records[2].tokens["input_tokens"], 12);
    assert_eq!(records[2].tokens["reserved_input_tokens"], 13);
}

#[test]
fn captures_are_nested_and_thread_isolated_with_no_revalidation_calls() {
    let outer = CaptureScope::begin();
    record_call(&request());
    let inner = CaptureScope::begin();
    record_call(&request());
    record_call(&request());
    assert_eq!(inner.take().calls.len(), 2);
    drop(inner);
    assert_eq!(outer.take().calls.len(), 1);
    drop(outer);
    assert!(ACTIVE.with(|active| active.borrow().is_none()));
    let sink = Arc::new(Sink::default());
    let kernel = Arc::new(TracedKernel::new(
        Probe {
            methods: Mutex::new(Vec::new()),
            barrier: Some(Arc::new(Barrier::new(2))),
        },
        Some(sink.clone()),
    ));
    std::thread::scope(|scope| {
        for id in ["first", "second"] {
            let kernel = kernel.clone();
            scope.spawn(move || {
                let mut query = query();
                query.text = id.into();
                kernel.answer_accounted(&query, &context());
                assert!(ACTIVE.with(|active| active.borrow().is_none()));
            });
        }
    });
    let records = sink.0.lock().unwrap();
    assert_eq!(records.len(), 2);
    assert_ne!(records[0].question_sha256, records[1].question_sha256);
    assert!(records
        .iter()
        .all(|record| record.tool_calls == vec!["get_hero"] && record.citation_ids == vec!["42"]));
}

#[test]
fn warm_cached_answers_resolve_original_numeric_receipts_without_tool_execution() {
    struct Cached;
    impl AnswerKernelPort for Cached {
        fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
            AnswerResponse {
                contract_version: CONTRACT_VERSION.into(),
                request_id: query.request_id.clone(),
                knowledge_release: context.knowledge_release.clone(),
                status: AnswerStatus::Answered,
                text: "Fixture".into(),
                citations: vec![evidence()],
                usage: Usage::default(),
            }
        }
    }
    let sink = Arc::new(Sink::default());
    let kernel = TracedKernel::new(Cached, Some(sink.clone()));
    let answer = kernel.answer(&query(), &context());
    assert_eq!(answer.citations[0].evidence_id, evidence().evidence_id);
    let records = sink.0.lock().unwrap();
    assert!(records[0].tool_calls.is_empty());
    assert_eq!(records[0].citation_ids, vec!["42"]);
    let mut capture = Capture::default();
    let mut forged = evidence();
    forged.content = forged.content.replace("42", "43");
    record_evidence(&mut capture, &forged);
    assert!(capture.citations.is_empty());
}

#[test]
fn diagnostic_failure_does_not_change_the_answer_or_accounting() {
    struct Fail;
    impl TraceSink for Fail {
        fn write(&self, _: TraceRecord) -> Result<(), PortError> {
            Err(PortError::Unavailable("Fixture".into()))
        }
    }
    let kernel = TracedKernel::new(
        Probe {
            methods: Mutex::new(Vec::new()),
            barrier: None,
        },
        Some(Arc::new(Fail)),
    );
    let answer = kernel.answer_accounted(&query(), &context());
    assert_eq!(answer.value.status, AnswerStatus::Answered);
    assert_eq!(answer.accounting.observed.cost_micros, 8);
    assert_eq!(kernel.inner.methods.lock().unwrap().len(), 1);
    assert!(ACTIVE.with(|active| active.borrow().is_none()));
}

#[test]
fn stored_text_drops_nul_controls_handles_and_numeric_ids() {
    assert_eq!(
        stored(
            "a\0b\x1b[2J @alice_1 fragt 12345678 zu Patch 5.12 und 99999",
            4000
        ),
        "a b [2J @user fragt [id] zu Patch 5.12 und 99999"
    );
    assert_eq!(stored("E-Mail a@b.de @", 4000), "E-Mail a@user @");
    assert_eq!(stored("äöü@x", 3), "äöü");
    assert_eq!(printable("dl-bot\0-discord\n"), "dl-bot -discord ");
}
