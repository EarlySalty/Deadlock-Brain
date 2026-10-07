use brain_contracts::*;
use brain_providers::{OpenAiCompatibleProvider, PriceCeiling, ProviderConfig};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
fn query() -> Query {
    Query {
        domain: None,
        request_id: "fixture-request".into(),
        conversation_id: "fixture-conversation".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: None,
        request_deadline: None,
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "fixture-conversation".into(),
        knowledge_release: "fixture-release".into(),
        deadline_ms: 500,
        budget: Budget::default(),
    }
}
fn json_response(status: &str, body: &str) -> String {
    format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len())
}
fn fixture<F: FnOnce(ProviderConfig) -> R, R>(
    responses: Vec<(Duration, String)>,
    f: F,
) -> (R, usize) {
    let (result, calls, _) = fixture_requests(responses, f);
    (result, calls)
}
fn fixture_requests<F: FnOnce(ProviderConfig) -> R, R>(
    responses: Vec<(Duration, String)>,
    f: F,
) -> (R, usize, Vec<serde_json::Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let server_stop = stop.clone();
    let server_calls = calls.clone();
    let requests = Arc::new(std::sync::Mutex::new(Vec::new()));
    let server_requests = requests.clone();
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(3);
        for (delay, response) in responses {
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if server_stop.load(Ordering::SeqCst) || Instant::now() > deadline {
                            return;
                        }
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(e) => panic!("{e}"),
                }
            };
            server_calls.fetch_add(1, Ordering::SeqCst);
            stream
                .set_read_timeout(Some(Duration::from_millis(200)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        bytes.extend_from_slice(&buffer[..n]);
                        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                            let headers = String::from_utf8_lossy(&bytes[..end]);
                            let len = headers
                                .lines()
                                .filter_map(|l| l.split_once(':'))
                                .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                                .and_then(|(_, v)| v.trim().parse::<usize>().ok())
                                .unwrap_or(0);
                            if bytes.len() >= end + 4 + len {
                                break;
                            }
                        }
                        if bytes.len() > 65536 {
                            break;
                        }
                    }
                }
            }
            let end = bytes
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .unwrap();
            server_requests
                .lock()
                .unwrap()
                .push(serde_json::from_slice(&bytes[end + 4..]).unwrap());
            let (headers, body) = response.split_once("\r\n\r\n").unwrap();
            let _ = stream.write_all(headers.as_bytes());
            let _ = stream.write_all(b"\r\n\r\n");
            thread::sleep(delay);
            let _ = stream.write_all(body.as_bytes());
        }
        while !server_stop.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    server_calls.fetch_add(1, Ordering::SeqCst);
                    let _ =
                        stream.write_all(json_response("500 Unexpected Request", "{}").as_bytes());
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(2))
                }
                Err(error) => panic!("{error}"),
            }
        }
    });
    let mut config = ProviderConfig::new(
        "fixture-token",
        format!("http://{address}"),
        "fixture-model",
    );
    config.retry_attempts = 2;
    config.retry_backoff = Duration::from_millis(1);
    config.timeout = Duration::from_millis(100);
    let result = f(config);
    stop.store(true, Ordering::SeqCst);
    server.join().unwrap();
    (
        result,
        calls.load(Ordering::SeqCst),
        Arc::try_unwrap(requests).unwrap().into_inner().unwrap(),
    )
}
const CHAT: &str = r#"{"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"content":"fixture answer"}}],"usage":{"prompt_tokens":12,"completion_tokens":3}}"#;

fn definitions() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: ToolName::EntityFind,
        description: "Entität suchen".into(),
        input_schema: serde_json::json!({"type":"object","properties":{"query":{"type":"string"},"language":{"type":"string","enum":["german","english"]}},"required":["query","language"],"additionalProperties":false}),
    }]
}
fn tool_block(id: &str) -> ModelBlock {
    ModelBlock::ToolUse {
        call: ToolCall {
            id: id.into(),
            name: ToolName::EntityFind,
            arguments: serde_json::json!({"query":"Abrams","language":"german"}),
        },
    }
}
fn evidence() -> Evidence {
    Evidence {
        evidence_id: "e1".into(),
        source_id: "fixture".into(),
        logical_id: "hero".into(),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: "Öffentlicher Beleg".into(),
        citation: "fixture:hero".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: None,
        patch: None,
    }
}
fn append_result(conversation: &mut ToolConversation, blocks: Vec<ModelBlock>) {
    let results = blocks
        .iter()
        .filter_map(|block| match block {
            ModelBlock::ToolUse { call } => Some(ToolResult {
                call_id: call.id.clone(),
                name: call.name,
                result: serde_json::json!({"hero_id":1,"text":"Öffentlicher Beleg"}),
                evidence_ids: vec!["e1".into()],
                is_error: false,
            }),
            ModelBlock::Text { .. } => None,
        })
        .collect();
    conversation
        .messages
        .push(ToolMessage::Assistant { blocks });
    conversation
        .messages
        .push(ToolMessage::ToolResults { results });
}
fn turn_context() -> AuthorizedContext {
    context().with_request_deadline().into_owned()
}
fn turn_provider(native: bool, config: ProviderConfig) -> Box<dyn AnswerProviderPort> {
    if native {
        let mut native_config = ProviderConfig::codex_subscription(config.base_url);
        native_config.timeout = config.timeout;
        native_config.max_response_bytes = config.max_response_bytes;
        Box::new(brain_providers::CodexSubscriptionProvider::new(native_config).unwrap())
    } else {
        Box::new(OpenAiCompatibleProvider::new(config).unwrap())
    }
}
fn wire_response(
    native: bool,
    blocks: &[ModelBlock],
    reason: &str,
    input: u64,
    output: u64,
) -> serde_json::Value {
    if native {
        let content: Vec<_> = blocks.iter().map(|block| match block {
            ModelBlock::Text { text } => serde_json::json!({"type":"text","text":text}),
            ModelBlock::ToolUse { call } => serde_json::json!({"type":"tool_use","id":call.id,"name":call.name,"input":call.arguments}),
        }).collect();
        serde_json::json!({"model":"gpt-6-luna","content":content,"stop_reason":reason,"usage":{"input_tokens":input,"output_tokens":output}})
    } else {
        let text: String = blocks
            .iter()
            .filter_map(|block| match block {
                ModelBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        let calls: Vec<_> = blocks.iter().filter_map(|block| match block {ModelBlock::ToolUse { call } => Some(serde_json::json!({"id":call.id,"type":"function","function":{"name":call.name,"arguments":call.arguments.to_string()}})), _ => None}).collect();
        serde_json::json!({"model":"fixture-model","choices":[{"finish_reason":reason,"message":{"role":"assistant","content":if text.is_empty(){serde_json::Value::Null}else{serde_json::json!(text)},"tool_calls":calls}}],"usage":{"prompt_tokens":input,"completion_tokens":output}})
    }
}
fn final_response(native: bool, envelope: serde_json::Value) -> serde_json::Value {
    wire_response(
        native,
        &[ModelBlock::Text {
            text: envelope.to_string(),
        }],
        if native { "end_turn" } else { "stop" },
        11,
        4,
    )
}
fn deduct(context: &AuthorizedContext, usage: &Usage) -> AuthorizedContext {
    let mut remaining = context.clone();
    remaining.budget.max_network_rounds -= usage.network_rounds;
    remaining.budget.max_input_tokens -= u32::try_from(usage.input_tokens).unwrap();
    remaining.budget.max_output_tokens -= u32::try_from(usage.output_tokens).unwrap();
    remaining.budget.max_cost_micros -= usage.cost_micros;
    remaining
}

fn raw_tool_response(native: bool, arguments: &str) -> String {
    if native {
        format!(
            r#"{{"model":"gpt-6-luna","stop_reason":"tool_use","content":[{{"type":"tool_use","id":"call-1","name":"entity_find","input":{arguments}}}],"usage":{{"input_tokens":10,"output_tokens":3}}}}"#
        )
    } else {
        let arguments = serde_json::json!(arguments);
        format!(
            r#"{{"model":"fixture-model","choices":[{{"finish_reason":"tool_calls","message":{{"content":null,"tool_calls":[{{"id":"call-1","type":"function","function":{{"name":"entity_find","arguments":{arguments}}}}}]}}}}],"usage":{{"prompt_tokens":10,"completion_tokens":3}}}}"#
        )
    }
}

#[test]
fn fehlerabrechnung_beide_wireformen_erhalten_usage_nach_parser_und_validierungsfehlern() {
    for native in [false, true] {
        let good = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            17,
            7,
        );
        let mut variants = Vec::new();
        let mut bad_name = good.clone();
        if native {
            bad_name["content"][0]["name"] = serde_json::json!("unknown");
        } else {
            bad_name["choices"][0]["message"]["tool_calls"][0]["function"]["name"] =
                serde_json::json!("unknown");
        }
        variants.push(bad_name);
        variants.push(wire_response(
            native,
            &[tool_block("call-1")],
            "unknown",
            17,
            7,
        ));
        variants.push(wire_response(
            native,
            &[ModelBlock::Text {
                text: r#"{"text":"A","text":"B","cited_evidence_ids":["e1"]}"#.into(),
            }],
            if native { "end_turn" } else { "stop" },
            17,
            7,
        ));
        let mut invalid_args = good.clone();
        if native {
            invalid_args["content"][0]["input"]["actor_id"] = serde_json::json!("foreign");
        } else {
            invalid_args["choices"][0]["message"]["tool_calls"][0]["function"]["arguments"] =
                serde_json::json!(r#"{"query":"A","query":"B","language":"german"}"#);
        }
        variants.push(invalid_args);
        for body in variants {
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                |config| {
                    turn_provider(native, config).answer_turn_accounted(
                        &query(),
                        &turn_context(),
                        &[evidence()],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert_eq!(calls, 1);
            let failure = result.unwrap_err();
            assert!(matches!(failure.error, PortError::InvalidResponse(_)));
            let accounting = failure.accounting.unwrap();
            assert_eq!(accounting.observed.input_tokens, 17);
            assert_eq!(accounting.observed.output_tokens, 7);
            assert_eq!(accounting.observed.network_rounds, 1);
            assert!(accounting.reserved.input_tokens > 0);
            assert_eq!(accounting.reserved.output_tokens, 0);
            assert!(!accounting.unaccounted);
        }
    }
}

#[test]
fn fehlerabrechnung_fehlende_oder_ambige_usage_bleibt_reserviert_statt_gemessen() {
    for native in [false, true] {
        let good = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            10,
            3,
        );
        let mut missing = good.clone();
        missing.as_object_mut().unwrap().remove("usage");
        let mut invalid = good.clone();
        invalid["usage"] = serde_json::json!({"input_tokens":"unknown"});
        let duplicate = format!(
            r#"{{"diagnostic":1,"diagnostic":2,{}"#,
            &good.to_string()[1..]
        );
        for body in [
            missing.to_string(),
            invalid.to_string(),
            duplicate,
            "{broken".into(),
        ] {
            let (result, calls, requests) = fixture_requests(
                vec![(Duration::ZERO, json_response("200 OK", &body))],
                |config| {
                    turn_provider(native, config).answer_turn_accounted(
                        &query(),
                        &turn_context(),
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert_eq!(calls, 1);
            let accounting = result.unwrap_err().accounting.unwrap();
            assert_eq!(accounting.observed.network_rounds, 1);
            assert_eq!(accounting.observed.input_tokens, 0);
            assert_eq!(accounting.observed.output_tokens, 0);
            let ceiling =
                brain_contracts::provider_input::transport_input_ceiling(&requests[0], true)
                    .unwrap();
            assert_eq!(accounting.reserved.input_tokens, ceiling);
            assert_eq!(
                accounting.reserved.output_tokens,
                requests[0]["max_tokens"].as_u64().unwrap()
            );
        }
    }
}

#[test]
fn fehlerabrechnung_retry_und_spaeterer_parserfehler_kumulieren_reservierung_und_messung() {
    let body = wire_response(false, &[tool_block("call-1")], "unknown", 7, 3);
    let (result, calls, requests) = fixture_requests(
        vec![
            (Duration::ZERO, json_response("503 Unavailable", "{}")),
            (Duration::ZERO, json_response("200 OK", &body.to_string())),
        ],
        |mut config| {
            config.pricing = Some(PriceCeiling {
                input_micros_per_token: 1,
                output_micros_per_token: 2,
            });
            let mut context = turn_context();
            context.budget.max_output_tokens = 40;
            OpenAiCompatibleProvider::new(config)
                .unwrap()
                .answer_turn_accounted(
                    &query(),
                    &context,
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
        },
    );
    assert_eq!(calls, 2);
    assert_eq!(requests[0], requests[1]);
    let failure = result.unwrap_err();
    assert!(matches!(failure.error, PortError::InvalidResponse(_)));
    let accounting = failure.accounting.unwrap();
    let ceiling =
        brain_contracts::provider_input::transport_input_ceiling(&requests[0], true).unwrap();
    assert_eq!(accounting.observed.input_tokens, 7);
    assert_eq!(accounting.observed.output_tokens, 3);
    assert_eq!(accounting.observed.network_rounds, 2);
    assert_eq!(accounting.reserved.input_tokens, 2 * ceiling - 7);
    assert_eq!(accounting.reserved.output_tokens, 20);
    assert_eq!(accounting.reserved.cost_micros, 2 * ceiling + 46);
    assert_eq!(accounting.observed.cost_micros, 0);
}

#[test]
fn fehlerabrechnung_http_retry_usage_ist_beobachtet_und_nicht_zweimal_reserviert() {
    let first = serde_json::json!({"usage":{"prompt_tokens":5,"completion_tokens":2}}).to_string();
    let second = serde_json::json!({"usage":{"prompt_tokens":7,"completion_tokens":3}}).to_string();
    let (result, calls, requests) = fixture_requests(
        vec![
            (Duration::ZERO, json_response("503 Unavailable", &first)),
            (Duration::ZERO, json_response("503 Unavailable", &second)),
        ],
        |config| {
            OpenAiCompatibleProvider::new(config)
                .unwrap()
                .answer_turn_accounted(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
        },
    );
    assert_eq!(calls, 2);
    let failure = result.unwrap_err();
    assert!(matches!(failure.error, PortError::Unavailable(_)));
    let accounting = failure.accounting.unwrap();
    assert_eq!(accounting.observed.input_tokens, 12);
    assert_eq!(accounting.observed.output_tokens, 5);
    assert_eq!(accounting.observed.network_rounds, 2);
    let ceiling =
        brain_contracts::provider_input::transport_input_ceiling(&requests[0], true).unwrap();
    assert_eq!(accounting.reserved.input_tokens, 2 * ceiling - 12);
    assert_eq!(accounting.reserved.output_tokens, 0);
}

fn faulty_transient_responses(status: &str, limit: usize) -> Vec<String> {
    let oversized = "x".repeat(limit + 1);
    vec![
        json_response(status, &oversized),
        format!("HTTP/1.1 {status}\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{oversized}\r\n0\r\n\r\n", oversized.len()),
        format!("HTTP/1.1 {status}\r\nContent-Length: {limit}\r\nConnection: close\r\n\r\n{{}}"),
    ]
}

#[test]
fn transiente_http_fehlerkoerper_erhalten_status_retry_und_reservierung() {
    for native in [false, true] {
        for (status, code) in [
            ("503 Unavailable", reqwest::StatusCode::SERVICE_UNAVAILABLE),
            (
                "429 Too Many Requests",
                reqwest::StatusCode::TOO_MANY_REQUESTS,
            ),
        ] {
            for response in faulty_transient_responses(status, 512) {
                let success = raw_tool_response(native, r#"{"query":"A","language":"german"}"#);
                let (result, calls, requests) = fixture_requests(
                    vec![
                        (Duration::ZERO, response),
                        (Duration::ZERO, json_response("200 OK", &success)),
                    ],
                    |mut config| {
                        config.max_response_bytes = 512;
                        config.pricing = Some(PriceCeiling {
                            input_micros_per_token: 1,
                            output_micros_per_token: 2,
                        });
                        let mut context = turn_context();
                        context.budget.max_output_tokens = 40;
                        let deadline = context.request_deadline.as_ref().unwrap().expires_at();
                        let result = turn_provider(native, config).answer_turn_accounted(
                            &query(),
                            &context,
                            &[],
                            &definitions(),
                            &ToolConversation::default(),
                        );
                        assert_eq!(
                            context.request_deadline.as_ref().unwrap().expires_at(),
                            deadline
                        );
                        result
                    },
                );
                let ceiling =
                    brain_contracts::provider_input::transport_input_ceiling(&requests[0], true)
                        .unwrap();
                if native {
                    assert_eq!(calls, 1);
                    let failure = result.unwrap_err();
                    assert_eq!(
                        failure.error,
                        PortError::Unavailable(
                            brain_providers::ProviderError::HttpStatus { status: code }.to_string()
                        )
                    );
                    let accounting = failure.accounting.unwrap();
                    assert_eq!(accounting.observed.network_rounds, 1);
                    assert_eq!(accounting.observed.input_tokens, 0);
                    assert_eq!(accounting.observed.output_tokens, 0);
                    assert_eq!(accounting.reserved.input_tokens, ceiling);
                    assert_eq!(accounting.reserved.output_tokens, 40);
                    assert!(!accounting.unaccounted);
                } else {
                    assert_eq!(calls, 2);
                    assert_eq!(requests[0], requests[1]);
                    let success = result.unwrap();
                    assert!(matches!(success.value, ProviderTurn::ToolCalls { .. }));
                    assert_eq!(success.value.usage().network_rounds, 2);
                    assert_eq!(success.value.usage().input_tokens, ceiling + 10);
                    assert_eq!(success.value.usage().output_tokens, 23);
                    let accounting = success.accounting;
                    assert_eq!(accounting.observed.network_rounds, 2);
                    assert_eq!(accounting.observed.input_tokens, 10);
                    assert_eq!(accounting.observed.output_tokens, 3);
                    assert_eq!(accounting.reserved.input_tokens, 2 * ceiling - 10);
                    assert_eq!(accounting.reserved.output_tokens, 20);
                    assert_eq!(accounting.reserved.cost_micros, 2 * ceiling + 46);
                    assert_eq!(accounting.observed.cost_micros, 0);
                    assert!(!accounting.unaccounted);
                }
            }
        }
    }
}

#[test]
fn transiente_http_fehlerkoerper_erhalten_fruehere_gemessene_usage() {
    let measured =
        serde_json::json!({"usage":{"prompt_tokens":5,"completion_tokens":2}}).to_string();
    for response in faulty_transient_responses("503 Unavailable", 512) {
        let success = raw_tool_response(false, r#"{"query":"A","language":"german"}"#);
        let (result, calls, requests) = fixture_requests(
            vec![
                (Duration::ZERO, json_response("503 Unavailable", &measured)),
                (Duration::ZERO, response),
                (Duration::ZERO, json_response("200 OK", &success)),
            ],
            |mut config| {
                config.retry_attempts = 3;
                config.max_response_bytes = 512;
                config.pricing = Some(PriceCeiling {
                    input_micros_per_token: 1,
                    output_micros_per_token: 2,
                });
                let mut context = turn_context();
                context.budget.max_output_tokens = 39;
                OpenAiCompatibleProvider::new(config)
                    .unwrap()
                    .answer_turn_accounted(
                        &query(),
                        &context,
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
            },
        );
        assert_eq!(calls, 3);
        assert_eq!(requests[0], requests[1]);
        assert_eq!(requests[1], requests[2]);
        let ceiling =
            brain_contracts::provider_input::transport_input_ceiling(&requests[0], true).unwrap();
        let accounting = result.unwrap().accounting;
        assert_eq!(accounting.observed.network_rounds, 3);
        assert_eq!(accounting.observed.input_tokens, 15);
        assert_eq!(accounting.observed.output_tokens, 5);
        assert_eq!(accounting.reserved.input_tokens, 3 * ceiling - 15);
        assert_eq!(accounting.reserved.output_tokens, 13);
        assert_eq!(accounting.reserved.cost_micros, 3 * ceiling + 36);
        assert_eq!(accounting.observed.cost_micros, 0);
        assert!(!accounting.unaccounted);
    }
}

#[test]
fn transiente_http_fehlerkoerper_respektieren_retry_und_budgetgrenzen() {
    for native in [false, true] {
        for limit in 0..3 {
            let response = faulty_transient_responses("503 Unavailable", 512).remove(0);
            let (result, calls, requests) = fixture_requests(
                vec![
                    (Duration::ZERO, response),
                    (Duration::ZERO, json_response("200 OK", CHAT)),
                ],
                |mut config| {
                    config.max_response_bytes = 512;
                    let mut context = turn_context();
                    match limit {
                        0 => config.retry_attempts = 1,
                        1 => context.budget.max_network_rounds = 1,
                        _ => context.budget.max_output_tokens = 1,
                    }
                    turn_provider(native, config).answer_turn_accounted(
                        &query(),
                        &context,
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert_eq!(calls, 1);
            let failure = result.unwrap_err();
            assert!(matches!(failure.error, PortError::Unavailable(_)));
            let accounting = failure.accounting.unwrap();
            assert_eq!(accounting.observed.network_rounds, 1);
            assert_eq!(accounting.observed.input_tokens, 0);
            assert_eq!(accounting.observed.output_tokens, 0);
            assert_eq!(
                accounting.reserved.input_tokens,
                brain_contracts::provider_input::transport_input_ceiling(&requests[0], true)
                    .unwrap()
            );
            assert_eq!(
                accounting.reserved.output_tokens,
                requests[0]["max_tokens"].as_u64().unwrap()
            );
        }
    }
}

#[test]
fn transiente_http_fehlerkoerper_timeout_respektiert_die_urspruengliche_frist() {
    for native in [false, true] {
        for expires in [false, true] {
            let success = raw_tool_response(native, r#"{"query":"A","language":"german"}"#);
            let (result, calls, requests) = fixture_requests(
                vec![
                    (
                        Duration::from_millis(150),
                        json_response("503 Unavailable", "{}"),
                    ),
                    (Duration::ZERO, json_response("200 OK", &success)),
                ],
                |mut config| {
                    config.timeout = Duration::from_millis(100);
                    let mut context = turn_context();
                    context.budget.max_output_tokens = 40;
                    if expires {
                        context.deadline_ms = 5000;
                        context.request_deadline =
                            Some(RequestDeadline::after(Duration::from_millis(50)));
                    }
                    let deadline = context.request_deadline.as_ref().unwrap().expires_at();
                    let result = turn_provider(native, config).answer_turn_accounted(
                        &query(),
                        &context,
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    );
                    assert_eq!(
                        context.request_deadline.as_ref().unwrap().expires_at(),
                        deadline
                    );
                    result
                },
            );
            if !native && !expires {
                assert_eq!(calls, 2);
                let accounting = result.unwrap().accounting;
                assert_eq!(accounting.observed.network_rounds, 2);
                assert_eq!(accounting.observed.input_tokens, 10);
                assert_eq!(accounting.observed.output_tokens, 3);
                assert_eq!(accounting.reserved.output_tokens, 20);
            } else {
                assert_eq!(calls, 1);
                let failure = result.unwrap_err();
                if !native {
                    assert_eq!(failure.error, PortError::BudgetExceeded);
                } else {
                    assert!(matches!(failure.error, PortError::Unavailable(_)));
                }
                let accounting = failure.accounting.unwrap();
                assert_eq!(accounting.observed.network_rounds, 1);
                assert_eq!(accounting.observed.input_tokens, 0);
                assert_eq!(accounting.observed.output_tokens, 0);
                assert_eq!(
                    accounting.reserved.input_tokens,
                    brain_contracts::provider_input::transport_input_ceiling(&requests[0], true)
                        .unwrap()
                );
                assert_eq!(
                    accounting.reserved.output_tokens,
                    requests[0]["max_tokens"].as_u64().unwrap()
                );
            }
        }
    }
}

#[test]
fn fehlerabrechnung_budgetueberschreitung_behaelt_gemessene_usage() {
    for native in [false, true] {
        let body = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            10,
            2001,
        );
        let (result, calls) = fixture(
            vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
            |config| {
                turn_provider(native, config).answer_turn_accounted(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
            },
        );
        assert_eq!(calls, 1);
        let failure = result.unwrap_err();
        assert_eq!(failure.error, PortError::BudgetExceeded);
        let accounting = failure.accounting.unwrap();
        assert_eq!(accounting.observed.output_tokens, 2001);
        assert_eq!(accounting.observed.input_tokens, 10);
        assert_eq!(accounting.observed.network_rounds, 1);
    }
}

#[test]
fn fehlerabrechnung_lokale_validierung_hat_belegte_null_ohne_socket() {
    for native in [false, true] {
        let (results, calls) = fixture(vec![], |config| {
            let provider = turn_provider(native, config);
            let mut exhausted = turn_context();
            exhausted.budget.max_input_tokens = 1;
            let a = provider.answer_turn_accounted(
                &query(),
                &exhausted,
                &[],
                &definitions(),
                &ToolConversation::default(),
            );
            let mut invalid = query();
            invalid.text.clear();
            let b = provider.answer_turn_accounted(
                &invalid,
                &turn_context(),
                &[],
                &definitions(),
                &ToolConversation::default(),
            );
            vec![a, b]
        });
        assert_eq!(calls, 0);
        for result in results {
            assert_eq!(
                result.unwrap_err().accounting,
                Some(Box::new(UsageAccounting::default()))
            );
        }
    }
}

#[test]
fn fehlerabrechnung_erfolg_traegt_messung_und_retriereservierung_getrennt() {
    let (result, calls) = fixture(
        vec![
            (Duration::ZERO, json_response("503 Unavailable", "{}")),
            (Duration::ZERO, json_response("200 OK", CHAT)),
        ],
        |config| {
            OpenAiCompatibleProvider::new(config)
                .unwrap()
                .answer_accounted(&query(), &context(), &[])
        },
    );
    assert_eq!(calls, 2);
    let result = result.unwrap();
    assert_eq!(result.accounting.observed.input_tokens, 12);
    assert_eq!(result.accounting.observed.output_tokens, 3);
    assert_eq!(result.accounting.observed.network_rounds, 2);
    assert!(result.accounting.reserved.input_tokens > 0);
    assert!(result.accounting.reserved.output_tokens > 0);
    assert_eq!(result.value.usage.network_rounds, 2);
    assert!(result.value.usage.input_tokens > result.accounting.observed.input_tokens);
}

#[test]
fn duplicate_raw_arguments_fail_closed_in_both_wire_forms_without_retry() {
    let cases = [
        r#"{"query":"A","query":"B","language":"german"}"#,
        concat!(
            r#"{"query":"A","qu"#,
            "\\u0065",
            r#"ry":"B","language":"german"}"#
        ),
        r#"{"query":"A","language":"english","language":"german"}"#,
        r#"{"query":"A","language":"german","extra":{"value":1,"value":2}}"#,
        r#"{"query":"A","language":"german","extra":[{"value":null,"value":1}]}"#,
        concat!(
            r#"{"query":"A","language":"german","extra":{"ä":1,""#,
            "\\u00e4",
            r#"":2}}"#
        ),
    ];
    for native in [false, true] {
        for (case, arguments) in cases.iter().enumerate() {
            let body = raw_tool_response(native, arguments);
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", &body))],
                |config| {
                    let context = turn_context();
                    let deadline = context.request_deadline.as_ref().unwrap().expires_at();
                    let result = turn_provider(native, config).answer_turn(
                        &query(),
                        &context,
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    );
                    assert_eq!(
                        context.request_deadline.as_ref().unwrap().expires_at(),
                        deadline
                    );
                    result
                },
            );
            assert_eq!(
                result,
                Err(PortError::InvalidResponse("invalid chat schema".into())),
                "native={native}, case={case}"
            );
            assert_eq!(calls, 1);
        }
    }
}

#[test]
fn duplicate_wire_response_keys_including_unknown_nested_objects_are_rejected() {
    for native in [false, true] {
        let good = raw_tool_response(native, r#"{"query":"A","language":"german"}"#);
        let variants = [
            format!(r#"{{"diagnostic":1,"diagnostic":2,{}"#, &good[1..]),
            format!(
                concat!(r#"{{"diagnostic":1,"diag"#, "\\u006e", r#"ostic":2,{}"#),
                &good[1..]
            ),
            format!(r#"{{"diagnostic":{{"value":1,"value":2}},{}"#, &good[1..]),
            format!(r#"{{"diagnostic":[{{"value":1,"value":2}}],{}"#, &good[1..]),
            good.replace(r#""model":""#, r#""model":"ignored","model":""#),
            if native {
                good.replace(
                    r#""input_tokens":10"#,
                    r#""input_tokens":9,"input_tokens":10"#,
                )
            } else {
                good.replace(
                    r#""prompt_tokens":10"#,
                    r#""prompt_tokens":9,"prompt_tokens":10"#,
                )
            },
        ];
        for (case, body) in variants.iter().enumerate() {
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", body))],
                |config| {
                    turn_provider(native, config).answer_turn(
                        &query(),
                        &turn_context(),
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert_eq!(
                result,
                Err(PortError::InvalidResponse("invalid chat schema".into())),
                "native={native}, case={case}"
            );
            assert_eq!(calls, 1);
        }
        let body = format!(
            r#"{{"diagnostic":[{{"value":null}},{{"value":1}}],{}"#,
            &good[1..]
        );
        let (result, calls) = fixture(
            vec![(Duration::ZERO, json_response("200 OK", &body))],
            |config| {
                turn_provider(native, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
            },
        );
        assert!(matches!(result, Ok(ProviderTurn::ToolCalls { .. })));
        assert_eq!(calls, 1);
    }
}

#[test]
fn final_answer_json_rejects_duplicate_decoded_keys_in_both_wire_forms() {
    let cases = [
        r#"{"text":"A","text":"B","cited_evidence_ids":["e1"]}"#,
        r#"{"text":"B","cited_evidence_ids":["foreign"],"cited_evidence_ids":["e1"]}"#,
        concat!(
            r#"{"text":"A","te"#,
            "\\u0078",
            r#"t":"B","cited_evidence_ids":["e1"]}"#
        ),
        concat!(
            r#"{"text":"B","cited_evidence_ids":["foreign"],"cited_evidence_i"#,
            "\\u0064",
            r#"s":["e1"]}"#
        ),
    ];
    for native in [false, true] {
        for tool_turn in [false, true] {
            for (case, envelope) in cases.iter().enumerate() {
                let body = wire_response(
                    native,
                    &[ModelBlock::Text {
                        text: (*envelope).into(),
                    }],
                    if native { "end_turn" } else { "stop" },
                    11,
                    4,
                );
                let (result, calls) = fixture(
                    vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                    |config| {
                        let provider = turn_provider(native, config);
                        if tool_turn {
                            provider.answer_turn(
                                &query(),
                                &turn_context(),
                                &[evidence()],
                                &definitions(),
                                &ToolConversation::default(),
                            )
                        } else {
                            provider
                                .answer(&query(), &context(), &[evidence()])
                                .map(ProviderTurn::from)
                        }
                    },
                );
                assert_eq!(
                    result,
                    Err(PortError::InvalidResponse(
                        "grounded answer envelope missing".into()
                    )),
                    "native={native}, tool_turn={tool_turn}, case={case}"
                );
                assert_eq!(calls, 1);
            }
        }
    }
}

#[test]
fn legacy_final_text_keeps_plain_answers_but_rejects_duplicate_json_without_evidence() {
    for native in [false, true] {
        for (text, valid) in [
            (r#"{"text":"A","text":"B","cited_evidence_ids":[]}"#, false),
            (
                concat!(
                    r#"{"text":"A","te"#,
                    "\\u0078",
                    r#"t":"B","cited_evidence_ids":[]}"#
                ),
                false,
            ),
            (r#"{"nested":[{"value":1,"value":2}]}"#, false),
            (r#"{"left":{"value":null},"right":{"value":1}}"#, true),
            ("[null,1.25,-0.0,1e30]", true),
            ("plain text", true),
        ] {
            let body = wire_response(
                native,
                &[ModelBlock::Text { text: text.into() }],
                if native { "end_turn" } else { "stop" },
                11,
                4,
            );
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                |config| turn_provider(native, config).answer(&query(), &context(), &[]),
            );
            assert_eq!(result.is_ok(), valid);
            if !valid {
                assert!(matches!(result, Err(PortError::InvalidResponse(_))));
            }
            assert_eq!(calls, 1);
        }
    }
}

#[test]
fn both_wire_forms_preserve_tools_nullable_content_and_two_turn_binding() {
    for native in [false, true] {
        for mixed in [false, true] {
            let mut blocks = Vec::new();
            if mixed {
                blocks.push(ModelBlock::Text {
                    text: "Suche".into(),
                });
            }
            blocks.push(tool_block("call-1"));
            blocks.push(tool_block("call-2"));
            let mut first = wire_response(
                native,
                &blocks,
                if native { "tool_use" } else { "tool_calls" },
                10,
                3,
            );
            if native && mixed {
                first["content"].as_array_mut().unwrap().insert(
                    0,
                    serde_json::json!({"type":"thinking","thinking":"nicht ausgeben"}),
                );
            }
            let final_body = final_response(
                native,
                serde_json::json!({"text":"Belegte Antwort","cited_evidence_ids":["e1"]}),
            );
            let ((first_turn, final_turn, total), calls, requests) = fixture_requests(
                vec![
                    (Duration::ZERO, json_response("200 OK", &first.to_string())),
                    (
                        Duration::ZERO,
                        json_response("200 OK", &final_body.to_string()),
                    ),
                ],
                |mut config| {
                    config.retry_attempts = 1;
                    let provider = turn_provider(native, config);
                    let context = turn_context();
                    let initial_deadline = context.request_deadline.as_ref().unwrap().expires_at();
                    let tools = definitions();
                    let mut conversation = ToolConversation::default();
                    let first = provider
                        .answer_turn(&query(), &context, &[evidence()], &tools, &conversation)
                        .unwrap();
                    let ProviderTurn::ToolCalls {
                        blocks: returned,
                        finish_reason,
                        ..
                    } = &first
                    else {
                        panic!("expected tool calls")
                    };
                    assert_eq!(*finish_reason, ProviderFinishReason::ToolUse);
                    assert_eq!(returned, &blocks);
                    append_result(&mut conversation, returned.clone());
                    if let ToolMessage::ToolResults { results } = &mut conversation.messages[1] {
                        results.reverse();
                    }
                    let remaining = deduct(&context, first.usage());
                    assert_eq!(
                        remaining.request_deadline.as_ref().unwrap().expires_at(),
                        initial_deadline
                    );
                    let final_turn = provider
                        .answer_turn(&query(), &remaining, &[evidence()], &tools, &conversation)
                        .unwrap();
                    assert_eq!(
                        final_turn.clone().into_answer().unwrap().cited_evidence_ids,
                        vec!["e1"]
                    );
                    let mut total = first.usage().clone();
                    total.input_tokens = total
                        .input_tokens
                        .checked_add(final_turn.usage().input_tokens)
                        .unwrap();
                    total.output_tokens = total
                        .output_tokens
                        .checked_add(final_turn.usage().output_tokens)
                        .unwrap();
                    total.network_rounds = total
                        .network_rounds
                        .checked_add(final_turn.usage().network_rounds)
                        .unwrap();
                    total.cost_micros = total
                        .cost_micros
                        .checked_add(final_turn.usage().cost_micros)
                        .unwrap();
                    context.request_deadline.as_ref().unwrap().cancel();
                    assert_eq!(
                        provider.answer_turn(
                            &query(),
                            &remaining,
                            &[evidence()],
                            &tools,
                            &conversation
                        ),
                        Err(PortError::BudgetExceeded)
                    );
                    (first, final_turn, total)
                },
            );
            assert_eq!(calls, 2);
            assert_eq!(first_turn.usage().input_tokens, 10);
            assert_eq!(final_turn.usage().input_tokens, 11);
            assert_eq!(total.input_tokens, 21);
            assert_eq!(total.output_tokens, 7);
            assert_eq!(total.network_rounds, 2);
            assert_eq!(total.cost_micros, 0);
            for request in &requests {
                let first = &request["tools"][0];
                if native {
                    assert_eq!(first["name"], "entity_find");
                    assert_eq!(first["input_schema"], definitions()[0].input_schema);
                    assert_eq!(request["tool_choice"], serde_json::json!({"type":"auto"}));
                } else {
                    assert_eq!(first["function"]["name"], "entity_find");
                    assert_eq!(
                        first["function"]["parameters"],
                        definitions()[0].input_schema
                    );
                    assert_eq!(request["tool_choice"], "auto");
                }
                brain_contracts::provider_input::transport_input_ceiling(request, true).unwrap();
            }
            let offset = if native { 1 } else { 2 };
            let assistant = &requests[1]["messages"][offset];
            if native {
                let tool_uses: Vec<_> = assistant["content"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|block| block["type"] == "tool_use")
                    .collect();
                assert_eq!(tool_uses[0]["id"], "call-1");
                assert_eq!(tool_uses[1]["id"], "call-2");
                assert_eq!(
                    requests[1]["messages"][offset + 1]["content"][0]["tool_use_id"],
                    "call-2"
                );
                assert_eq!(
                    requests[1]["messages"][offset + 1]["content"][1]["tool_use_id"],
                    "call-1"
                );
            } else {
                assert_eq!(assistant["tool_calls"][0]["id"], "call-1");
                assert_eq!(assistant["tool_calls"][1]["id"], "call-2");
                assert_eq!(assistant["content"].is_null(), !mixed);
                assert_eq!(
                    requests[1]["messages"][offset + 1]["tool_call_id"],
                    "call-2"
                );
                assert_eq!(
                    requests[1]["messages"][offset + 2]["tool_call_id"],
                    "call-1"
                );
            }
        }
    }
}

#[test]
fn malformed_tool_names_ids_arguments_and_blocks_are_not_repaired() {
    for native in [false, true] {
        let good = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            10,
            3,
        );
        let path: &[&str] = if native {
            &["content"]
        } else {
            &["choices", "message", "tool_calls"]
        };
        let mut variants = Vec::new();
        for case in 0..10 {
            let mut bad = good.clone();
            let call = if native {
                &mut bad[path[0]][0]
            } else {
                &mut bad["choices"][0]["message"]["tool_calls"][0]
            };
            match case {
                0 => {
                    call[if native { "name" } else { "function" }] = if native {
                        serde_json::json!("sql_execute")
                    } else {
                        serde_json::json!({"name":"sql_execute","arguments":"{}"})
                    };
                }
                1 => {
                    call["id"] = serde_json::json!("");
                }
                2 => {
                    call.as_object_mut().unwrap().remove("id");
                }
                3 => {
                    call["id"] = serde_json::json!("bad\nid");
                }
                4 => {
                    if native {
                        call["input"] = serde_json::json!("not object");
                    } else {
                        call["function"]["arguments"] = serde_json::json!("{broken");
                    }
                }
                5 => {
                    let input = serde_json::json!({"query":"Abrams","language":"german","actor_id":"other"});
                    if native {
                        call["input"] = input;
                    } else {
                        call["function"]["arguments"] = serde_json::json!(input.to_string());
                    }
                }
                6 => {
                    let input = serde_json::json!({"query":"Abrams","language":"french"});
                    if native {
                        call["input"] = input;
                    } else {
                        call["function"]["arguments"] = serde_json::json!(input.to_string());
                    }
                }
                7 => {
                    call["extra"] = serde_json::json!("hidden");
                }
                8 => {
                    call["type"] = serde_json::json!("future_tool");
                }
                _ => {
                    let input = serde_json::json!({"language":"german"});
                    if native {
                        call["input"] = input;
                    } else {
                        call["function"]["arguments"] = serde_json::json!(input.to_string());
                    }
                }
            }
            variants.push(bad);
        }
        let mut duplicate = good.clone();
        let calls = if native {
            duplicate["content"].as_array_mut().unwrap()
        } else {
            duplicate["choices"][0]["message"]["tool_calls"]
                .as_array_mut()
                .unwrap()
        };
        calls.push(calls[0].clone());
        variants.push(duplicate);
        let mut missing_args = good.clone();
        if native {
            missing_args["content"][0]
                .as_object_mut()
                .unwrap()
                .remove("input");
        } else {
            missing_args["choices"][0]["message"]["tool_calls"][0]["function"]
                .as_object_mut()
                .unwrap()
                .remove("arguments");
        }
        variants.push(missing_args);
        for bad in variants {
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", &bad.to_string()))],
                |config| {
                    turn_provider(native, config).answer_turn(
                        &query(),
                        &turn_context(),
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert!(matches!(result, Err(PortError::InvalidResponse(_))));
            assert_eq!(calls, 1);
        }
    }
}

#[test]
fn known_finish_reasons_must_match_complete_turns() {
    for native in [false, true] {
        for tool in [false, true] {
            let blocks = if tool {
                vec![tool_block("call-1")]
            } else {
                vec![ModelBlock::Text {
                    text: serde_json::json!({"text":"","cited_evidence_ids":[]}).to_string(),
                }]
            };
            for reason in [
                "future_reason",
                if native { "max_tokens" } else { "length" },
                if native { "refusal" } else { "content_filter" },
                if tool {
                    if native {
                        "end_turn"
                    } else {
                        "stop"
                    }
                } else if native {
                    "tool_use"
                } else {
                    "tool_calls"
                },
            ] {
                let body = wire_response(native, &blocks, reason, 10, 3);
                let (result, calls) = fixture(
                    vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                    |config| {
                        turn_provider(native, config).answer_turn(
                            &query(),
                            &turn_context(),
                            &[],
                            &definitions(),
                            &ToolConversation::default(),
                        )
                    },
                );
                assert!(matches!(result, Err(PortError::InvalidResponse(_))));
                assert_eq!(calls, 1);
            }
        }
        let mut missing = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            10,
            3,
        );
        if native {
            missing.as_object_mut().unwrap().remove("stop_reason");
        } else {
            missing["choices"][0]
                .as_object_mut()
                .unwrap()
                .remove("finish_reason");
        }
        let (result, calls) = fixture(
            vec![(
                Duration::ZERO,
                json_response("200 OK", &missing.to_string()),
            )],
            |config| {
                turn_provider(native, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
            },
        );
        assert!(matches!(result, Err(PortError::InvalidResponse(_))));
        assert_eq!(calls, 1);
    }
}

#[test]
fn tool_history_binding_and_egress_fail_before_network() {
    for native in [false, true] {
        let (results, calls) = fixture(vec![], |config| {
            let provider = turn_provider(native, config);
            let mut valid = ToolConversation::default();
            append_result(&mut valid, vec![tool_block("call-1")]);
            let mut results = Vec::new();
            for case in 0..7 {
                let mut bad = valid.clone();
                if let ToolMessage::ToolResults { results } = &mut bad.messages[1] {
                    match case {
                        0 => {
                            results[0].call_id = "foreign".into();
                        }
                        1 => {
                            results[0].name = ToolName::ServerKnowledge;
                        }
                        2 => {
                            results.clear();
                        }
                        3 => {
                            results.push(results[0].clone());
                        }
                        4 => {
                            results[0].evidence_ids = vec!["unapproved".into()];
                        }
                        5 => {
                            results[0].evidence_ids.push("e1".into());
                        }
                        _ => {}
                    }
                }
                if case == 6 {
                    bad.messages.remove(0);
                }
                results.push(provider.answer_turn(
                    &query(),
                    &turn_context(),
                    &[evidence()],
                    &definitions(),
                    &bad,
                ));
            }
            results.push(provider.answer_turn(
                &query(),
                &context(),
                &[evidence()],
                &definitions(),
                &ToolConversation::default(),
            ));
            let mut private = evidence();
            private.visibility = SourceVisibility::Private;
            results.push(provider.answer_turn(
                &query(),
                &turn_context(),
                &[private],
                &definitions(),
                &valid,
            ));
            let mut invalid = definitions();
            invalid[0].input_schema["properties"]["actor_id"] =
                serde_json::json!({"type":"string"});
            results.push(provider.answer_turn(
                &query(),
                &turn_context(),
                &[evidence()],
                &invalid,
                &ToolConversation::default(),
            ));
            results
        });
        assert_eq!(calls, 0);
        assert!(results
            .iter()
            .all(|result| matches!(result, Err(PortError::InvalidResponse(_)))));
    }
}

#[test]
fn duplicate_call_ids_from_earlier_turns_are_rejected() {
    for native in [false, true] {
        let body = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            10,
            3,
        );
        let (result, calls) = fixture(
            vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
            |config| {
                let mut conversation = ToolConversation::default();
                append_result(&mut conversation, vec![tool_block("call-1")]);
                turn_provider(native, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[evidence()],
                    &definitions(),
                    &conversation,
                )
            },
        );
        assert!(matches!(result, Err(PortError::InvalidResponse(_))));
        assert_eq!(calls, 1);
    }
}

#[test]
fn final_tool_answers_keep_json_citation_and_no_answer_contract() {
    for native in [false, true] {
        for (envelope, valid) in [
            (
                serde_json::json!({"text":"Antwort","cited_evidence_ids":["e1"]}),
                true,
            ),
            (serde_json::json!({"text":"","cited_evidence_ids":[]}), true),
            (
                serde_json::json!({"text":"Antwort","cited_evidence_ids":["foreign"]}),
                false,
            ),
            (
                serde_json::json!({"text":"Antwort","cited_evidence_ids":["e1","e1"]}),
                false,
            ),
            (
                serde_json::json!({"text":"Antwort","cited_evidence_ids":[]}),
                false,
            ),
            (
                serde_json::json!({"text":" ","cited_evidence_ids":[]}),
                false,
            ),
            (
                serde_json::json!({"text":"Antwort","cited_evidence_ids":["e1"],"extra":true}),
                false,
            ),
        ] {
            let body = final_response(native, envelope);
            let (result, calls) = fixture(
                vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                |config| {
                    turn_provider(native, config).answer_turn(
                        &query(),
                        &turn_context(),
                        &[evidence()],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                },
            );
            assert_eq!(result.is_ok(), valid);
            assert_eq!(calls, 1);
        }
        let body = wire_response(
            native,
            &[ModelBlock::Text {
                text: "not JSON".into(),
            }],
            if native { "end_turn" } else { "stop" },
            10,
            3,
        );
        let (result, calls) = fixture(
            vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
            |config| {
                turn_provider(native, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
            },
        );
        assert!(matches!(result, Err(PortError::InvalidResponse(_))));
        assert_eq!(calls, 1);
    }
}

#[test]
fn retries_charge_full_wire_input_output_and_configured_cost() {
    let body = wire_response(false, &[tool_block("call-1")], "tool_calls", 10, 3);
    let ((turn, budget), calls, requests) = fixture_requests(
        vec![
            (Duration::ZERO, json_response("503 Unavailable", "{}")),
            (Duration::ZERO, json_response("200 OK", &body.to_string())),
        ],
        |mut config| {
            config.pricing = Some(PriceCeiling {
                input_micros_per_token: 1,
                output_micros_per_token: 2,
            });
            let mut context = turn_context();
            context.budget.max_output_tokens = 40;
            context.budget.max_cost_micros = 1_000_000;
            let provider = OpenAiCompatibleProvider::new(config).unwrap();
            (
                provider
                    .answer_turn(
                        &query(),
                        &context,
                        &[],
                        &definitions(),
                        &ToolConversation::default(),
                    )
                    .unwrap(),
                context.budget,
            )
        },
    );
    assert_eq!(calls, 2);
    assert_eq!(requests[0], requests[1]);
    let ceiling =
        brain_contracts::provider_input::transport_input_ceiling(&requests[0], true).unwrap();
    assert_eq!(turn.usage().input_tokens, ceiling + 10);
    assert_eq!(turn.usage().output_tokens, 23);
    assert_eq!(turn.usage().network_rounds, 2);
    assert_eq!(turn.usage().cost_micros, ceiling + 10 + 46);
    assert!(turn.usage().input_tokens <= budget.max_input_tokens as u64);
    assert!(turn.usage().output_tokens <= budget.max_output_tokens as u64);
}

fn expected_payload(native: bool, conversation: &ToolConversation) -> serde_json::Value {
    let format = if native {
        brain_contracts::provider_input::ToolWireFormat::Native
    } else {
        brain_contracts::provider_input::ToolWireFormat::OpenAiCompatible
    };
    let mut payload = brain_contracts::provider_input::grounded_turn_payload(
        &query(),
        &[evidence()],
        &definitions(),
        conversation,
        format,
    )
    .unwrap();
    if native {
        payload["tool_choice"] = serde_json::json!({"type":"auto"});
        payload["output_config"] = serde_json::json!({"effort":"low"});
    } else {
        payload["tool_choice"] = serde_json::json!("auto");
    }
    payload
}

#[test]
fn cumulative_budgets_block_repeated_history_before_second_socket() {
    for native in [false, true] {
        let mut history = ToolConversation::default();
        append_result(&mut history, vec![tool_block("call-1")]);
        let first_ceiling = brain_contracts::provider_input::transport_input_ceiling(
            &expected_payload(native, &ToolConversation::default()),
            true,
        )
        .unwrap();
        let next_ceiling = brain_contracts::provider_input::transport_input_ceiling(
            &expected_payload(native, &history),
            true,
        )
        .unwrap();
        assert!(next_ceiling > first_ceiling);
        let body = wire_response(
            native,
            &[tool_block("call-1")],
            if native { "tool_use" } else { "tool_calls" },
            first_ceiling,
            3,
        );
        for limit in 0..if native { 3 } else { 4 } {
            let (result, calls, requests) = fixture_requests(
                vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
                |mut config| {
                    config.retry_attempts = 1;
                    if limit == 3 {
                        config.pricing = Some(PriceCeiling {
                            input_micros_per_token: 1,
                            output_micros_per_token: 1,
                        });
                    }
                    let provider = turn_provider(native, config);
                    let mut context = turn_context();
                    match limit {
                        0 => {
                            context.budget.max_input_tokens =
                                u32::try_from(first_ceiling + next_ceiling - 1).unwrap();
                        }
                        1 => {
                            context.budget.max_network_rounds = 1;
                        }
                        2 => {
                            context.budget.max_output_tokens = 3;
                        }
                        _ => {
                            context.budget.max_output_tokens = 10;
                            context.budget.max_cost_micros = first_ceiling + 10;
                        }
                    }
                    let first = provider
                        .answer_turn(
                            &query(),
                            &context,
                            &[evidence()],
                            &definitions(),
                            &ToolConversation::default(),
                        )
                        .unwrap();
                    provider.answer_turn(
                        &query(),
                        &deduct(&context, first.usage()),
                        &[evidence()],
                        &definitions(),
                        &history,
                    )
                },
            );
            assert_eq!(result, Err(PortError::BudgetExceeded));
            assert_eq!(calls, 1);
            assert_eq!(
                brain_contracts::provider_input::transport_input_ceiling(&requests[0], true)
                    .unwrap(),
                first_ceiling
            );
        }
    }
}

#[test]
fn extended_contract_names_boon_ranges_and_analytics_use_the_same_transport() {
    let definitions = vec![
        ToolDefinition {
            name: ToolName::GameRules,
            description: "Spielregel".into(),
            input_schema: serde_json::json!({"type":"object","properties":{"topic":{"type":"string","enum":["urn"]}},"required":["topic"],"additionalProperties":false}),
        },
        ToolDefinition {
            name: ToolName::HeroCompare,
            description: "Gemeinsamer Vergleich".into(),
            input_schema: serde_json::json!({"type":"object","properties":{"hero_ids":{"type":"array","items":{"type":"integer","minimum":1}},"metrics":{"type":"array","items":{"type":"string"}},"scenario":{"type":"object","properties":{"progression":{"type":"object","properties":{"kind":{"type":"string","enum":["boons"]},"value":{"type":"integer","minimum":0}},"required":["kind","value"],"additionalProperties":false}},"required":["progression"],"additionalProperties":false},"boon_range":{"type":"object","properties":{"min_boons":{"type":"integer"},"max_boons":{"type":"integer"}},"required":["min_boons","max_boons"],"additionalProperties":false},"analytics":{"type":"object","properties":{"min_average_badge":{"type":"integer"},"max_average_badge":{"type":"integer"},"min_unix_timestamp":{"type":"integer"},"max_unix_timestamp":{"type":"integer"}},"required":["min_unix_timestamp","max_unix_timestamp"],"additionalProperties":false}},"required":["hero_ids","metrics","scenario"],"additionalProperties":false}),
        },
    ];
    let blocks = vec![
        ModelBlock::ToolUse {
            call: ToolCall {
                id: "rules-1".into(),
                name: ToolName::GameRules,
                arguments: serde_json::json!({"topic":"urn"}),
            },
        },
        ModelBlock::ToolUse {
            call: ToolCall {
                id: "compare-1".into(),
                name: ToolName::HeroCompare,
                arguments: serde_json::json!({"hero_ids":[1,2],"metrics":["weapon_dps"],"scenario":{"progression":{"kind":"boons","value":20}},"boon_range":{"min_boons":0,"max_boons":35},"analytics":{"min_average_badge":70,"max_average_badge":80,"min_unix_timestamp":1,"max_unix_timestamp":2}}),
            },
        },
    ];
    for native in [false, true] {
        let body = wire_response(
            native,
            &blocks,
            if native { "tool_use" } else { "tool_calls" },
            10,
            3,
        );
        let (result, calls, requests) = fixture_requests(
            vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
            |config| {
                turn_provider(native, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions,
                    &ToolConversation::default(),
                )
            },
        );
        let ProviderTurn::ToolCalls {
            blocks: returned, ..
        } = result.unwrap()
        else {
            panic!("expected extended tool calls")
        };
        assert_eq!(returned, blocks);
        assert_eq!(calls, 1);
        assert_eq!(requests[0]["tools"].as_array().unwrap().len(), 2);
    }
}

#[test]
fn full_native_options_and_cached_input_are_charged() {
    let mut body = wire_response(true, &[tool_block("call-1")], "tool_use", 10, 3);
    body["usage"]["cache_read_input_tokens"] = serde_json::json!(20);
    body["usage"]["cache_creation_input_tokens"] = serde_json::json!(30);
    let (turn, calls) = fixture(
        vec![(Duration::ZERO, json_response("200 OK", &body.to_string()))],
        |config| {
            turn_provider(true, config).answer_turn(
                &query(),
                &turn_context(),
                &[],
                &definitions(),
                &ToolConversation::default(),
            )
        },
    );
    assert_eq!(calls, 1);
    assert_eq!(turn.unwrap().usage().input_tokens, 60);
    for field in ["input_tokens", "output_tokens", "cache_read_input_tokens"] {
        let mut overflow = body.clone();
        overflow["usage"][field] = serde_json::json!(u64::MAX);
        let (result, calls) = fixture(
            vec![(
                Duration::ZERO,
                json_response("200 OK", &overflow.to_string()),
            )],
            |config| {
                turn_provider(true, config).answer_turn(
                    &query(),
                    &turn_context(),
                    &[],
                    &definitions(),
                    &ToolConversation::default(),
                )
            },
        );
        assert_eq!(result, Err(PortError::BudgetExceeded));
        assert_eq!(calls, 1);
    }
}

#[test]
fn grounded_no_answer_signal_is_exact_and_preserves_usage() {
    let evidence = Evidence {
        evidence_id: "e1".into(),
        source_id: "fixture".into(),
        logical_id: "a".into(),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: "Unpassender Auszug".into(),
        citation: "fixture:a".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    for (text, valid) in [("", true), ("Keine passende Antwort", false), (" ", false)] {
        let envelope = serde_json::json!({"text":text,"cited_evidence_ids":[]}).to_string();
        let body = serde_json::json!({"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"content":envelope}}],"usage":{"prompt_tokens":12,"completion_tokens":3}}).to_string();
        let (result, calls) = fixture(
            vec![(Duration::ZERO, json_response("200 OK", &body))],
            |c| {
                OpenAiCompatibleProvider::new(c).unwrap().answer(
                    &query(),
                    &context(),
                    std::slice::from_ref(&evidence),
                )
            },
        );
        assert_eq!(calls, 1);
        assert_eq!(result.is_ok(), valid);
        if let Ok(answer) = result {
            assert!(answer.text.is_empty());
            assert!(answer.cited_evidence_ids.is_empty());
            assert_eq!(answer.usage.output_tokens, 3);
        }
    }
}
#[test]
fn authentication_redirect_and_missing_usage_do_not_retry() {
    for response in [json_response("401 Unauthorized","{}"),"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/secret\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into(),json_response("200 OK",r#"{"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"content":"no usage"}}]}"#),json_response("200 OK",r#"{"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"content":"bad usage"}}],"usage":{}}"#)] {
        let (result,calls)=fixture(vec![(Duration::ZERO,response)],|c|OpenAiCompatibleProvider::new(c).unwrap().answer(&query(),&context(),&[]));assert!(result.is_err());assert_eq!(calls,1);
    }
}
#[test]
fn server_error_retries_but_retry_after_beyond_deadline_does_not() {
    let (result, calls) = fixture(
        vec![
            (Duration::ZERO, json_response("503 Unavailable", "{}")),
            (Duration::ZERO, json_response("200 OK", CHAT)),
        ],
        |c| {
            OpenAiCompatibleProvider::new(c)
                .unwrap()
                .answer(&query(), &context(), &[])
        },
    );
    assert!(result.is_ok());
    assert_eq!(calls, 2);
    let response="HTTP/1.1 429 Too Many Requests\r\nRetry-After: 30\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into();
    let (result, calls) = fixture(vec![(Duration::ZERO, response)], |c| {
        OpenAiCompatibleProvider::new(c)
            .unwrap()
            .answer(&query(), &context(), &[])
    });
    assert_eq!(result.unwrap_err(), PortError::BudgetExceeded);
    assert_eq!(calls, 1);
}
#[test]
fn cost_input_and_egress_fail_before_any_socket() {
    let (results, calls) = fixture(vec![], |mut config| {
        config.pricing = Some(PriceCeiling {
            input_micros_per_token: 100,
            output_micros_per_token: 100,
        });
        let provider = OpenAiCompatibleProvider::new(config).unwrap();
        let mut c = context();
        c.budget.max_cost_micros = 1;
        let a = provider.answer(&query(), &c, &[]);
        c = context();
        c.budget.max_input_tokens = 1;
        let b = provider.answer(&query(), &c, &[]);
        c = context();
        c.principal.provider_egress.clear();
        let d = provider.answer(&query(), &c, &[]);
        vec![a, b, d]
    });
    assert!(results.iter().all(|result| result.is_err()));
    assert_eq!(calls, 0);
}
#[test]
fn bounded_chunked_response_and_timeout_are_errors() {
    let chunk = "x".repeat(512);
    let response=format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",chunk.len(),chunk);
    let (result, calls) = fixture(vec![(Duration::ZERO, response)], |mut c| {
        c.max_response_bytes = 128;
        OpenAiCompatibleProvider::new(c)
            .unwrap()
            .answer(&query(), &context(), &[])
    });
    assert!(matches!(result, Err(PortError::InvalidResponse(_))));
    assert_eq!(calls, 1);
    let (result, _) = fixture(
        vec![(Duration::from_millis(150), json_response("200 OK", CHAT))],
        |mut c| {
            c.retry_attempts = 1;
            OpenAiCompatibleProvider::new(c)
                .unwrap()
                .answer(&query(), &context(), &[])
        },
    );
    assert!(result.is_err());
}
fn identity() -> EmbeddingIdentity {
    EmbeddingIdentity {
        provider: "fixture".into(),
        model: "fixture-model".into(),
        revision: "1".into(),
        dimension: 2,
        pooling: "mean".into(),
        query_prefix: String::new(),
        document_prefix: String::new(),
        normalized: true,
    }
}
#[test]
fn embeddings_restore_indices_and_reject_duplicates_or_dimension_errors() {
    for (body, ok) in [
        (
            r#"{"model":"fixture-model","data":[{"index":1,"embedding":[0,1]},{"index":0,"embedding":[1,0]}],"usage":{"prompt_tokens":2}}"#,
            true,
        ),
        (
            r#"{"model":"fixture-model","data":[{"index":0,"embedding":[0,1]},{"index":0,"embedding":[1,0]}],"usage":{"prompt_tokens":2}}"#,
            false,
        ),
        (
            r#"{"model":"fixture-model","data":[{"index":0,"embedding":[1]},{"index":1,"embedding":[0,1]}],"usage":{"prompt_tokens":2}}"#,
            false,
        ),
    ] {
        let (result, _) = fixture(vec![(Duration::ZERO, json_response("200 OK", body))], |c| {
            OpenAiCompatibleProvider::new(c).unwrap().embed(
                &["first".into(), "second".into()],
                &identity(),
                &context(),
            )
        });
        assert_eq!(result.is_ok(), ok);
        if let Ok(result) = result {
            assert_eq!(result.vectors, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        }
    }
}

#[test]
fn cancelled_provider_and_embedding_calls_never_open_a_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let config = ProviderConfig::new(
        "fixture-token",
        format!("http://{}", listener.local_addr().unwrap()),
        "fixture-model",
    );
    let provider = OpenAiCompatibleProvider::new(config).unwrap();
    let mut c = context();
    let deadline = RequestDeadline::after(Duration::from_secs(5));
    deadline.cancel();
    c.request_deadline = Some(deadline);
    assert_eq!(
        provider.answer(&query(), &c, &[]),
        Err(PortError::BudgetExceeded)
    );
    assert_eq!(
        provider.embed(&["fixture".into()], &identity(), &c),
        Err(PortError::BudgetExceeded)
    );
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn cancellation_during_first_provider_attempt_prevents_a_retry() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let config = ProviderConfig::new(
        "fixture-token",
        format!("http://{}", listener.local_addr().unwrap()),
        "fixture-model",
    );
    let provider = OpenAiCompatibleProvider::new(config).unwrap();
    let mut c = context();
    c.deadline_ms = 5000;
    let deadline = RequestDeadline::after(Duration::from_secs(5));
    c.request_deadline = Some(deadline.clone());
    let worker = thread::spawn(move || {
        provider.answer_turn(
            &query(),
            &c,
            &[],
            &definitions(),
            &ToolConversation::default(),
        )
    });
    let started = Instant::now();
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(started.elapsed() < Duration::from_secs(2));
                thread::yield_now();
            }
            Err(error) => panic!("{error}"),
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut bytes = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        let count = stream.read(&mut buffer).unwrap();
        assert!(count > 0);
        bytes.extend_from_slice(&buffer[..count]);
        assert!(bytes.len() <= 65536);
        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&bytes[..end]);
            let length = headers
                .lines()
                .filter_map(|line| line.split_once(':'))
                .find(|(key, _)| key.eq_ignore_ascii_case("content-length"))
                .unwrap()
                .1
                .trim()
                .parse::<usize>()
                .unwrap();
            if bytes.len() >= end + 4 + length {
                break;
            }
        }
    }
    deadline.cancel();
    stream
        .write_all(json_response("503 Unavailable", "{}").as_bytes())
        .unwrap();
    assert_eq!(worker.join().unwrap(), Err(PortError::BudgetExceeded));
    assert!(Instant::now() < deadline.expires_at());
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
