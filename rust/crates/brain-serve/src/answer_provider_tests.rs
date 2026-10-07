use super::{AnswerProvider, AnswerProviderPort};
use brain_contracts::{
    provider_input::{grounded_turn_input_ceiling, ToolWireFormat},
    Accounted, AuthorizedContext, Budget, Evidence, EvidenceKind, ModelBlock, PortError, Principal,
    ProviderFinishReason, ProviderTurn, Query, RequestDeadline, SourceVisibility, ToolCall,
    ToolConversation, ToolDefinition, ToolMessage, ToolName, ToolResult, UsageAccounting,
};
use brain_providers::{CodexSubscriptionProvider, OpenAiCompatibleProvider, ProviderConfig};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

#[derive(Clone, Copy)]
enum Branch {
    OpenAi,
    Subscription,
}

impl Branch {
    fn provider(self, address: String) -> AnswerProvider {
        match self {
            Self::OpenAi => {
                let mut config = ProviderConfig::new("synthetic-key", address, "fixture-model");
                config.retry_attempts = 1;
                AnswerProvider::OpenAi(OpenAiCompatibleProvider::new(config).unwrap())
            }
            Self::Subscription => AnswerProvider::Subscription(
                CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(address))
                    .unwrap(),
            ),
        }
    }

    fn wire(self) -> ToolWireFormat {
        match self {
            Self::OpenAi => ToolWireFormat::OpenAiCompatible,
            Self::Subscription => ToolWireFormat::Native,
        }
    }

    fn response(self, tool: bool, malformed: bool) -> Value {
        let text = if malformed {
            "{broken".into()
        } else {
            json!({"text":"Strukturprobe", "cited_evidence_ids":["e1"]}).to_string()
        };
        match self {
            Self::OpenAi => {
                let message = if tool {
                    json!({"role":"assistant","content":null,"tool_calls":[{"id":"next","type":"function","function":{"name":"entity_find","arguments":"{\"query\":\"Probe\",\"language\":\"german\"}"}}]})
                } else {
                    json!({"role":"assistant","content":text})
                };
                json!({"model":"fixture-model", "choices":[{"message":message,"finish_reason":if tool {"tool_calls"} else {"stop"}}],"usage":{"prompt_tokens":7,"completion_tokens":3}})
            }
            Self::Subscription => {
                let content = if tool {
                    json!([{"type":"tool_use","id":"next","name":"entity_find","input":{"query":"Probe","language":"german"}}])
                } else {
                    json!([{"type":"text","text":text}])
                };
                json!({"model":"gpt-6-luna", "content":content,"stop_reason":if tool {"tool_use"} else {"end_turn"},"usage":{"input_tokens":7,"output_tokens":3}})
            }
        }
    }
}

fn query() -> Query {
    Query {
        request_id: "synthetic-request".into(),
        conversation_id: "synthetic-conversation".into(),
        text: "Öffentliche Strukturprobe".into(),
        domain: None,
        requested_scopes: BTreeSet::from(["docs.public".into()]),
        profile: Default::default(),
        patch: Some("synthetic-patch".into()),
        mode: None,
    }
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: None,
        principal: Principal {
            actor_id: "synthetic-actor".into(),
            channel: "test".into(),
            scopes: BTreeSet::from(["docs.public".into()]),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: query().conversation_id,
        knowledge_release: "synthetic-release".into(),
        deadline_ms: 10_000,
        budget: Budget {
            max_network_rounds: 1,
            max_input_tokens: 12_000,
            max_output_tokens: 17,
            max_cost_micros: 0,
        },
        request_deadline: Some(RequestDeadline::after(Duration::from_secs(10))),
    }
}

fn evidence(id: &str) -> Evidence {
    Evidence {
        evidence_id: id.into(),
        source_id: "synthetic-source".into(),
        logical_id: id.into(),
        revision: 1,
        kind: EvidenceKind::Fact,
        content: format!("Strukturbeleg {id}"),
        citation: format!("synthetic:{id}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["docs.public".into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    }
}

fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: ToolName::EntityFind,
        description: "Öffentliche Entität suchen".into(),
        input_schema: json!({"type":"object","properties":{"query":{"type":"string"},"language":{"type":"string","enum":["german"]}},"required":["query","language"],"additionalProperties":false}),
    }]
}

fn history() -> ToolConversation {
    let mut messages = Vec::new();
    for id in ["first", "second"] {
        messages.push(ToolMessage::Assistant {
            blocks: vec![
                ModelBlock::Text {
                    text: format!("Strukturturn {id}"),
                },
                ModelBlock::ToolUse {
                    call: ToolCall {
                        id: id.into(),
                        name: ToolName::EntityFind,
                        arguments: json!({"query":"Probe","language":"german"}),
                    },
                },
            ],
        });
        messages.push(ToolMessage::ToolResults {
            results: vec![ToolResult {
                call_id: id.into(),
                name: ToolName::EntityFind,
                result: json!({"probe":id}),
                evidence_ids: vec!["e1".into()],
                is_error: false,
            }],
        });
    }
    ToolConversation { messages }
}

fn read_payload(stream: &mut TcpStream) -> (String, Value) {
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    let mut data = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        let count = stream.read(&mut buffer).unwrap();
        assert_ne!(count, 0);
        data.extend_from_slice(&buffer[..count]);
        if let Some(end) = data.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
            let headers = String::from_utf8(data[..end].to_vec()).unwrap();
            let length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().unwrap())
                })
                .unwrap();
            if data.len() >= end + 4 + length {
                return (
                    headers,
                    serde_json::from_slice(&data[end + 4..end + 4 + length]).unwrap(),
                );
            }
        }
    }
}

fn fixture(
    branch: Branch,
    status: &str,
    response: Value,
) -> (AnswerProvider, thread::JoinHandle<Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}/v1", listener.local_addr().unwrap());
    let status = status.to_owned();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let (headers, payload) = read_payload(&mut stream);
        match branch {
            Branch::OpenAi => {
                assert!(headers.starts_with("POST /v1/chat/completions "));
                assert!(headers
                    .to_ascii_lowercase()
                    .contains("authorization: bearer synthetic-key"));
            }
            Branch::Subscription => {
                assert!(headers.starts_with("POST /v1/messages "));
                assert!(!headers.to_ascii_lowercase().contains("authorization:"));
            }
        }
        let body = response.to_string();
        write!(
            stream,
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        payload
    });
    (branch.provider(address), server)
}

fn assert_accounting(accounting: &UsageAccounting) {
    assert_eq!(accounting.observed.input_tokens, 7);
    assert_eq!(accounting.observed.output_tokens, 3);
    assert_eq!(accounting.observed.network_rounds, 1);
    assert_eq!(accounting.reserved.output_tokens, 0);
    assert!(!accounting.unaccounted);
    assert!(accounting.charged().input_tokens > 7);
}

#[test]
fn beide_enumzweige_delegieren_alle_vier_methoden_ueber_http() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        for method in 0..4 {
            let (provider, server) = fixture(branch, "200 OK", branch.response(false, false));
            let query = query();
            let context = context();
            let original = context.clone();
            let evidence = [evidence("e1"), evidence("uncited")];
            let turn = match method {
                0 => ProviderTurn::from(provider.answer(&query, &context, &evidence).unwrap()),
                1 => {
                    let accounted = provider
                        .answer_accounted(&query, &context, &evidence)
                        .unwrap();
                    assert_accounting(&accounted.accounting);
                    ProviderTurn::from(accounted.value)
                }
                2 => provider
                    .answer_turn(
                        &query,
                        &context,
                        &evidence,
                        &[],
                        &ToolConversation::default(),
                    )
                    .unwrap(),
                _ => {
                    let accounted = provider
                        .answer_turn_accounted(
                            &query,
                            &context,
                            &evidence,
                            &[],
                            &ToolConversation::default(),
                        )
                        .unwrap();
                    assert_accounting(&accounted.accounting);
                    accounted.value
                }
            };
            let answer = turn.into_answer().unwrap();
            assert_eq!(answer.cited_evidence_ids, ["e1"]);
            assert_eq!(answer.usage.network_rounds, 1);
            let payload = server.join().unwrap();
            let body: Value = serde_json::from_str(
                payload["messages"][usize::from(matches!(branch, Branch::OpenAi))]["content"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(body["query"], query.text);
            assert_eq!(body["evidence"].as_array().unwrap().len(), 2);
            assert_eq!(body["evidence"][1]["id"], "uncited");
            assert_eq!(payload["max_tokens"], 17);
            assert_eq!(context, original);
        }
    }
}

#[test]
fn beide_enumzweige_erhalten_werkzeuge_und_die_gesamte_historie() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        for accounted in [false, true] {
            for tool_response in [false, true] {
                let (provider, server) =
                    fixture(branch, "200 OK", branch.response(tool_response, false));
                let context = context();
                let evidence = [evidence("e1"), evidence("uncited")];
                let tools = tools();
                let history = history();
                let turn = if accounted {
                    let Accounted { value, accounting } = provider
                        .answer_turn_accounted(&query(), &context, &evidence, &tools, &history)
                        .unwrap();
                    assert_accounting(&accounting);
                    let expected = grounded_turn_input_ceiling(
                        &query(),
                        &evidence,
                        &tools,
                        &history,
                        branch.wire(),
                    )
                    .unwrap();
                    assert!(accounting.charged().input_tokens >= expected);
                    value
                } else {
                    provider
                        .answer_turn(&query(), &context, &evidence, &tools, &history)
                        .unwrap()
                };
                if tool_response {
                    assert!(matches!(
                        turn,
                        ProviderTurn::ToolCalls {
                            finish_reason: ProviderFinishReason::ToolUse,
                            ..
                        }
                    ));
                } else {
                    assert_eq!(turn.into_answer().unwrap().cited_evidence_ids, ["e1"]);
                }
                let payload = server.join().unwrap();
                assert_eq!(payload["tools"].as_array().unwrap().len(), 1);
                let messages = payload["messages"].as_array().unwrap();
                assert_eq!(
                    messages.len(),
                    5 + usize::from(matches!(branch, Branch::OpenAi))
                );
                let wire = payload.to_string();
                for marker in ["first", "second", "uncited", "entity_find"] {
                    assert!(wire.contains(marker));
                }
            }
        }
    }
}

#[test]
fn beide_enumzweige_erhalten_fehlerabrechnung_nach_transport() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        for turn in [false, true] {
            for status in ["200 OK", "400 Bad Request"] {
                let (provider, server) = fixture(branch, status, branch.response(false, true));
                let context = context();
                let evidence = [evidence("e1")];
                let failure = if turn {
                    provider
                        .answer_turn_accounted(&query(), &context, &evidence, &tools(), &history())
                        .unwrap_err()
                } else {
                    provider
                        .answer_accounted(&query(), &context, &evidence)
                        .unwrap_err()
                };
                assert_accounting(failure.accounting.as_deref().unwrap());
                if status == "200 OK" {
                    assert!(matches!(failure.error, PortError::InvalidResponse(_)));
                } else {
                    assert!(matches!(failure.error, PortError::Unavailable(_)));
                }
                server.join().unwrap();
            }
        }
    }
}

#[test]
fn beide_enumzweige_erhalten_reservierung_bei_fehlender_http_abrechnung() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        let (provider, server) = fixture(branch, "400 Bad Request", json!({"error":"synthetic"}));
        let failure = provider
            .answer_accounted(&query(), &context(), &[evidence("e1")])
            .unwrap_err();
        let accounting = failure.accounting.unwrap();
        assert_eq!(accounting.observed.network_rounds, 1);
        assert_eq!(accounting.observed.input_tokens, 0);
        assert!(accounting.reserved.input_tokens > 0);
        assert_eq!(accounting.reserved.output_tokens, 17);
        server.join().unwrap();
    }
}

#[test]
fn beide_enumzweige_erlauben_den_geprueften_erstturn_ohne_werkzeugbelege() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        let (provider, server) = fixture(branch, "200 OK", branch.response(true, false));
        let context = context();
        let original = context.clone();
        let accounted = provider
            .answer_turn_accounted(
                &query(),
                &context,
                &[],
                &tools(),
                &ToolConversation::default(),
            )
            .unwrap();
        assert_accounting(&accounted.accounting);
        assert!(matches!(
            accounted.value,
            ProviderTurn::ToolCalls {
                finish_reason: ProviderFinishReason::ToolUse,
                ..
            }
        ));
        let payload = server.join().unwrap();
        assert_eq!(payload["tools"].as_array().unwrap().len(), 1);
        assert_eq!(
            payload["messages"].as_array().unwrap().len(),
            1 + usize::from(matches!(branch, Branch::OpenAi))
        );
        assert_eq!(context, original);
    }
}

#[test]
fn beide_enumzweige_pruefen_erstturnrechte_und_die_urspruengliche_deadline() {
    for branch in [Branch::OpenAi, Branch::Subscription] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let provider = branch.provider(format!("http://{}", listener.local_addr().unwrap()));
        for denied in 0..5 {
            let mut context = context();
            match denied {
                0 => context.principal.provider_egress.clear(),
                1 => context.conversation_id = "wrong-conversation".into(),
                2 => context.request_deadline.as_ref().unwrap().cancel(),
                3 => context.request_deadline = Some(RequestDeadline::after(Duration::ZERO)),
                _ => context.budget.max_network_rounds = 0,
            }
            let original = context.clone();
            assert!(provider.answer(&query(), &context, &[]).is_err());
            assert!(provider
                .answer_turn(&query(), &context, &[], &[], &ToolConversation::default())
                .is_err());
            let failure = provider
                .answer_accounted(&query(), &context, &[])
                .unwrap_err();
            assert_eq!(
                failure.accounting.as_deref(),
                Some(&UsageAccounting::default())
            );
            let failure = provider
                .answer_turn_accounted(&query(), &context, &[], &[], &ToolConversation::default())
                .unwrap_err();
            assert_eq!(
                failure.accounting.as_deref(),
                Some(&UsageAccounting::default())
            );
            let failure = provider
                .answer_turn_accounted(
                    &query(),
                    &context,
                    &[],
                    &tools(),
                    &ToolConversation::default(),
                )
                .unwrap_err();
            assert_eq!(
                failure.accounting.as_deref(),
                Some(&UsageAccounting::default())
            );
            assert_eq!(context, original);
        }
        let mut unbound = context();
        unbound.request_deadline = None;
        assert!(provider
            .answer_turn_accounted(
                &query(),
                &unbound,
                &[],
                &tools(),
                &ToolConversation::default()
            )
            .is_err());
        let mut private = evidence("private");
        private.visibility = SourceVisibility::Private;
        let failure = provider
            .answer_accounted(&query(), &unbound, &[private])
            .unwrap_err();
        assert_eq!(
            failure.accounting.as_deref(),
            Some(&UsageAccounting::default())
        );
        let mut bad_history = history();
        if let ToolMessage::ToolResults { results } = &mut bad_history.messages[1] {
            results[0].evidence_ids = vec!["missing".into()];
        }
        let failure = provider
            .answer_turn_accounted(
                &query(),
                &context(),
                &[evidence("e1")],
                &tools(),
                &bad_history,
            )
            .unwrap_err();
        assert_eq!(
            failure.accounting.as_deref(),
            Some(&UsageAccounting::default())
        );
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }
}
