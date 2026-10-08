use std::{
    collections::BTreeSet,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use brain_contracts::{
    provider_input::{grounded_turn_input_ceiling, grounded_turn_payload, transport_input_ceiling, ToolWireFormat},
    Accounted, AnswerProfile, AnswerProviderPort, AnswerResponse, AnswerStatus, AuthorizedContext,
    Budget, Evidence, EvidenceKind, ModelBlock, PinnedGameContext, PortError, PortFailure, Principal,
    ProviderAnswer, ProviderTurn, Query, RetrievalPort, SourceVisibility, ToolCall, ToolConversation,
    ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolLanguage,
    ToolMessage, ToolName, ToolRequest, ToolResult, ToolValidationPurpose, Usage,
};
use brain_kernel::{AnswerKernelPort, GameContextResolver, Kernel};
use brain_providers::{CodexSubscriptionProvider, OpenAiCompatibleProvider, ProviderConfig};
use serde_json::{json, Value};

#[derive(Clone, Copy)]
enum Kind {
    Compatible,
    Subscription,
}

impl Kind {
    fn format(self) -> ToolWireFormat {
        match self {
            Self::Compatible => ToolWireFormat::OpenAiCompatible,
            Self::Subscription => ToolWireFormat::Native,
        }
    }

    fn response(self, call: Option<&ToolCall>, citations: &[&str]) -> Value {
        let answer = json!({"text":"Geprüfte Antwort", "cited_evidence_ids":citations}).to_string();
        match self {
            Self::Compatible => {
                let message = match call {
                    Some(call) => json!({"content":null,"tool_calls":[{"id":call.id,"type":"function","function":{"name":call.name,"arguments":call.arguments.to_string()}}]}),
                    None => json!({"content":answer}),
                };
                json!({"model":"fixture-model","choices":[{"finish_reason":if call.is_some() {"tool_calls"} else {"stop"},"message":message}],"usage":{"prompt_tokens":1,"completion_tokens":1}})
            }
            Self::Subscription => {
                let content = match call {
                    Some(call) => json!([{"type":"tool_use","id":call.id,"name":call.name,"input":call.arguments}]),
                    None => json!([{"type":"text","text":answer}]),
                };
                json!({"model":"gpt-6-luna","stop_reason":if call.is_some() {"tool_use"} else {"end_turn"},"content":content,"usage":{"input_tokens":1,"output_tokens":1}})
            }
        }
    }

    fn payload(self, evidence: &[Evidence], conversation: &ToolConversation) -> Value {
        let mut payload = grounded_turn_payload(&query(), evidence, &[definition()], conversation, self.format()).unwrap();
        payload["stream"] = json!(false);
        match self {
            Self::Compatible => {
                payload["model"] = json!("fixture-model");
                payload["tool_choice"] = json!("auto");
            }
            Self::Subscription => {
                payload["model"] = json!("gpt-6-luna");
                payload["tool_choice"] = json!({"type":"auto"});
                payload["output_config"] = json!({"effort":"low"});
            }
        }
        payload
    }
}

fn query() -> Query {
    Query {
        request_id: "tool-loop-request".into(),
        conversation_id: "tool-loop-conversation".into(),
        text: "Erkläre Abrams und Warden".into(),
        domain: None,
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: None,
        principal: Principal {
            actor_id: "fixture-actor".into(),
            channel: "fixture-channel".into(),
            scopes: BTreeSet::from(["fixture-internal".into()]),
            provider_egress: BTreeSet::from(["public".into(), "internal".into()]),
        },
        conversation_id: query().conversation_id,
        knowledge_release: "fixture-release".into(),
        deadline_ms: 60_000,
        budget: Budget { max_input_tokens: 100_000, ..Budget::default() },
        request_deadline: None,
    }
}

fn pin() -> PinnedGameContext {
    PinnedGameContext {
        client_version: 6759,
        language: ToolLanguage::German,
        mechanic_revision: "fixture-mechanics".into(),
    }
}

fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::EntityFind,
        description: "Entität suchen".into(),
        input_schema: json!({"type":"object","properties":{"query":{"type":"string","minLength":1},"language":{"type":"string","enum":["german"]}},"required":["query","language"],"additionalProperties":false}),
    }
}

fn call(index: usize) -> ToolCall {
    ToolCall {
        id: format!("call-{index}"),
        name: ToolName::EntityFind,
        arguments: json!({"query":if index == 1 {"Abrams"} else {"Warden"},"language":"german"}),
    }
}

fn evidence(index: usize) -> Vec<Evidence> {
    [false, true].into_iter().map(|uncited| Evidence {
        evidence_id: format!("e-{index}-{}", if uncited {"uncited"} else {"cited"}),
        source_id: "fixture-assets".into(),
        logical_id: format!("entity/{index}/{uncited}"),
        revision: 1,
        kind: EvidenceKind::Fact,
        content: if uncited {"Unzitierte Abhängigkeit äöüß漢字 ".repeat(80)} else {format!("Entität {index}")},
        citation: format!("fixture:{index}:{uncited}"),
        visibility: if uncited {SourceVisibility::Internal} else {SourceVisibility::Public},
        allowed_scopes: if uncited {BTreeSet::from(["fixture-internal".into()])} else {BTreeSet::new()},
        score: 1.0,
        provenance: None,
        patch: None,
    }).collect()
}

fn result(index: usize) -> ToolResult {
    ToolResult {
        call_id: call(index).id,
        name: ToolName::EntityFind,
        result: json!({"entity_id":index}),
        evidence_ids: vec![evidence(index)[0].evidence_id.clone()],
        is_error: false,
    }
}

fn conversation(rounds: usize) -> ToolConversation {
    ToolConversation { messages: (1..=rounds).flat_map(|index| [
        ToolMessage::Assistant { blocks: vec![ModelBlock::ToolUse { call: call(index) }] },
        ToolMessage::ToolResults { results: vec![result(index)] },
    ]).collect() }
}

fn ceiling(evidence: &[Evidence], conversation: &ToolConversation) -> u64 {
    [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible].into_iter().map(|format| {
        grounded_turn_input_ceiling(&query(), evidence, &[definition()], conversation, format).unwrap()
    }).max().unwrap()
}

fn charge(kind: Kind, evidence: &[Evidence], conversation: &ToolConversation) -> u64 {
    ceiling(evidence, conversation).max(transport_input_ceiling(&kind.payload(evidence, conversation), true).unwrap())
}

#[derive(Clone)]
struct TurnInput {
    context: AuthorizedContext,
    evidence: Vec<Evidence>,
    conversation: ToolConversation,
}

enum ConcreteProvider {
    Compatible(OpenAiCompatibleProvider),
    Subscription(CodexSubscriptionProvider),
}

struct RecordingProvider {
    inner: ConcreteProvider,
    inputs: Arc<Mutex<Vec<TurnInput>>>,
}

impl RecordingProvider {
    fn new(kind: Kind, address: &str, inputs: Arc<Mutex<Vec<TurnInput>>>) -> Self {
        let inner = match kind {
            Kind::Compatible => {
                let mut config = ProviderConfig::new("fixture-only", address, "fixture-model");
                config.retry_attempts = 1;
                ConcreteProvider::Compatible(OpenAiCompatibleProvider::new(config).unwrap())
            }
            Kind::Subscription => ConcreteProvider::Subscription(CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(address)).unwrap()),
        };
        Self { inner, inputs }
    }
}

impl AnswerProviderPort for RecordingProvider {
    fn answer(&self, query: &Query, context: &AuthorizedContext, evidence: &[Evidence]) -> Result<ProviderAnswer, PortError> {
        match &self.inner {
            ConcreteProvider::Compatible(provider) => provider.answer(query, context, evidence),
            ConcreteProvider::Subscription(provider) => provider.answer(query, context, evidence),
        }
    }

    fn answer_turn_accounted(&self, query: &Query, context: &AuthorizedContext, evidence: &[Evidence], tools: &[ToolDefinition], conversation: &ToolConversation) -> Result<Accounted<ProviderTurn>, PortFailure> {
        self.inputs.lock().unwrap().push(TurnInput { context: context.clone(), evidence: evidence.to_vec(), conversation: conversation.clone() });
        match &self.inner {
            ConcreteProvider::Compatible(provider) => provider.answer_turn_accounted(query, context, evidence, tools, conversation),
            ConcreteProvider::Subscription(provider) => provider.answer_turn_accounted(query, context, evidence, tools, conversation),
        }
    }
}

struct NoRetrieval;

impl RetrievalPort for NoRetrieval {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        panic!("Der Werkzeugpfad darf nicht in Retrieval wechseln")
    }
}

struct BoundGame;

impl GameContextResolver for BoundGame {
    fn resolve(&self, _: &Query, _: &AuthorizedContext) -> Result<Option<PinnedGameContext>, PortError> {
        Ok(Some(pin()))
    }

    fn validate(&self, _: &Query, _: &AuthorizedContext, game: Option<&PinnedGameContext>) -> Result<(), PortError> {
        assert_eq!(game, Some(&pin()));
        Ok(())
    }
}

struct Tools {
    revoked: Arc<AtomicBool>,
    executions: Arc<Mutex<Vec<AuthorizedContext>>>,
}

impl ToolExecutionPort for Tools {
    fn definitions(&self, _: &Query, _: &AuthorizedContext, _: Option<&PinnedGameContext>) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(vec![definition()])
    }

    fn execute(&self, query: &Query, context: &AuthorizedContext, game: Option<&PinnedGameContext>, call_id: &str, request: &ToolRequest) -> Result<ToolExecution, PortError> {
        assert_eq!(query, &crate::query());
        assert_eq!(context.principal, crate::context().principal);
        assert_eq!(game, Some(&pin()));
        let index = if call_id == "call-1" {1} else {2};
        assert_eq!(request, &call(index).validate(&[definition()]).unwrap());
        self.executions.lock().unwrap().push(context.clone());
        Ok(ToolExecution {
            result: result(index),
            dependencies: vec![ToolEvidenceDependency { request: request.clone(), game_context: game.cloned(), evidence: evidence(index) }],
            usage: Usage::default(),
        })
    }

    fn validate_dependencies(&self, _: &Query, _: &AuthorizedContext, game: Option<&PinnedGameContext>, dependencies: &[ToolEvidenceDependency], _: ToolValidationPurpose) -> Result<(), PortError> {
        assert_eq!(game, Some(&pin()));
        for (offset, dependency) in dependencies.iter().enumerate() {
            assert_eq!(dependency.evidence, evidence(offset + 1));
            if self.revoked.load(Ordering::SeqCst) && dependency.evidence.iter().any(|item| item.evidence_id.ends_with("uncited")) {
                return Err(PortError::PermissionDenied("Unzitierter Beleg wurde gesperrt".into()));
            }
        }
        Ok(())
    }
}

fn read_request(stream: &mut TcpStream) -> (String, Value) {
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    let mut data = Vec::new();
    loop {
        let mut buffer = [0_u8; 4096];
        let count = stream.read(&mut buffer).unwrap();
        assert_ne!(count, 0);
        data.extend_from_slice(&buffer[..count]);
        if let Some(end) = data.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
            let headers = String::from_utf8(data[..end].to_vec()).unwrap();
            let length: usize = headers.lines().find_map(|line| line.split_once(':').filter(|(key, _)| key.eq_ignore_ascii_case("content-length")).map(|(_, value)| value.trim().parse().unwrap())).unwrap();
            if data.len() >= end + 4 + length {
                return (headers, serde_json::from_slice(&data[end + 4..end + 4 + length]).unwrap());
            }
        }
    }
}

struct Server {
    address: String,
    thread: JoinHandle<(TcpListener, Vec<Value>)>,
}

impl Server {
    fn new(kind: Kind, replies: Vec<Value>, revoked: Arc<AtomicBool>, revoke_on_final: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = format!("http://{}/v1", listener.local_addr().unwrap());
        let thread = thread::spawn(move || {
            let mut requests = Vec::new();
            let total = replies.len();
            for (index, reply) in replies.into_iter().enumerate() {
                let (mut stream, _) = listener.accept().unwrap();
                let (headers, payload) = read_request(&mut stream);
                match kind {
                    Kind::Compatible => {
                        assert!(headers.starts_with("POST /v1/chat/completions "));
                        assert!(headers.to_ascii_lowercase().contains("authorization: bearer fixture-only"));
                    }
                    Kind::Subscription => {
                        assert!(headers.starts_with("POST /v1/messages "));
                        assert!(!headers.to_ascii_lowercase().contains("authorization:"));
                    }
                }
                requests.push(payload);
                if revoke_on_final && index + 1 == total {
                    revoked.store(true, Ordering::SeqCst);
                }
                let body = reply.to_string();
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
            (listener, requests)
        });
        Self { address, thread }
    }

    fn finish(self) -> Vec<Value> {
        let (listener, requests) = self.thread.join().unwrap();
        listener.set_nonblocking(true).unwrap();
        assert_eq!(listener.accept().unwrap_err().kind(), std::io::ErrorKind::WouldBlock);
        requests
    }
}

fn run(kind: Kind, server: &Server, context: &AuthorizedContext, revoked: Arc<AtomicBool>, inputs: Arc<Mutex<Vec<TurnInput>>>, executions: Arc<Mutex<Vec<AuthorizedContext>>>) -> Accounted<AnswerResponse> {
    let now = Instant::now();
    Kernel::new(NoRetrieval, RecordingProvider::new(kind, &server.address, inputs))
        .with_tools(Tools { revoked, executions }, BoundGame, "fixture-provider")
        .with_clock(move || now)
        .answer_accounted(&query(), context)
}

#[test]
fn beide_provider_fuehren_belegte_folgerunden_mit_allen_abhaengigkeiten() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        let revoked = Arc::new(AtomicBool::new(false));
        let inputs = Arc::new(Mutex::new(Vec::new()));
        let executions = Arc::new(Mutex::new(Vec::new()));
        let server = Server::new(kind, vec![kind.response(Some(&call(1)), &[]), kind.response(Some(&call(2)), &[]), kind.response(None, &["e-2-cited"])], revoked.clone(), false);
        let outcome = run(kind, &server, &context(), revoked, inputs.clone(), executions.clone());
        assert_eq!(outcome.value.status, AnswerStatus::Answered);
        assert_eq!(outcome.value.citations, vec![evidence(2)[0].clone()]);
        let requests = server.finish();
        let inputs = inputs.lock().unwrap();
        assert_eq!(inputs.len(), 3);
        assert_eq!(requests.len(), 3);
        assert_eq!(executions.lock().unwrap().len(), 2);
        let mut expected_charge = 0;
        for (round, (input, request)) in inputs.iter().zip(requests).enumerate() {
            let all: Vec<_> = (1..=round).flat_map(evidence).collect();
            assert_eq!(input.evidence, all);
            assert_eq!(input.conversation, conversation(round));
            assert_eq!(input.context.request_deadline.unwrap().expires_at(), inputs[0].context.request_deadline.unwrap().expires_at());
            assert_eq!(input.context.budget.max_network_rounds, context().budget.max_network_rounds - round as u32);
            let mut expected_payload = kind.payload(&all, &conversation(round));
            expected_payload["max_tokens"] = json!(input.context.budget.max_output_tokens);
            assert_eq!(request, expected_payload);
            assert_eq!(u64::from(input.context.budget.max_input_tokens), u64::from(context().budget.max_input_tokens) - expected_charge);
            expected_charge += charge(kind, &all, &conversation(round));
        }
        for execution in executions.lock().unwrap().iter() {
            assert_eq!(execution.request_deadline.unwrap().expires_at(), inputs[0].context.request_deadline.unwrap().expires_at());
        }
        assert_eq!(outcome.accounting.charged().input_tokens, expected_charge);
        assert_eq!(outcome.accounting.observed.input_tokens, 3);
        assert_eq!(outcome.accounting.charged().network_rounds, 3);
        assert!(!outcome.accounting.unaccounted);
    }
}

#[test]
fn unzitierte_utf8_belege_stoppen_folgerunde_vor_provideraufruf() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        let revoked = Arc::new(AtomicBool::new(false));
        let inputs = Arc::new(Mutex::new(Vec::new()));
        let executions = Arc::new(Mutex::new(Vec::new()));
        let server = Server::new(kind, vec![kind.response(Some(&call(1)), &[])], revoked.clone(), false);
        let mut budget = context();
        let first = charge(kind, &[], &conversation(0));
        let cited_only = charge(kind, &evidence(1)[..1], &conversation(1));
        budget.budget.max_input_tokens = u32::try_from(first + cited_only + 1).unwrap();
        assert!(ceiling(&evidence(1), &conversation(1)) > cited_only + 1);
        let outcome = run(kind, &server, &budget, revoked, inputs.clone(), executions.clone());
        assert_eq!(outcome.value.status, AnswerStatus::BudgetExceeded);
        assert!(outcome.value.citations.is_empty());
        assert_eq!(inputs.lock().unwrap().len(), 1);
        assert_eq!(executions.lock().unwrap().len(), 1);
        assert_eq!(server.finish().len(), 1);
        assert_eq!(outcome.accounting.charged().input_tokens, first);
        assert_eq!(outcome.accounting.charged().network_rounds, 1);
    }
}

#[test]
fn knappes_erstbudget_startet_keinen_provider() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        let revoked = Arc::new(AtomicBool::new(false));
        let inputs = Arc::new(Mutex::new(Vec::new()));
        let executions = Arc::new(Mutex::new(Vec::new()));
        let server = Server::new(kind, vec![], revoked.clone(), false);
        let mut budget = context();
        budget.budget.max_input_tokens = u32::try_from(ceiling(&[], &conversation(0)) - 1).unwrap();
        let outcome = run(kind, &server, &budget, revoked, inputs.clone(), executions.clone());
        assert_eq!(outcome.value.status, AnswerStatus::BudgetExceeded);
        assert!(inputs.lock().unwrap().is_empty());
        assert!(executions.lock().unwrap().is_empty());
        assert!(server.finish().is_empty());
        assert_eq!(outcome.accounting.charged(), Usage::default());
    }
}

#[test]
fn rechteverlust_eines_unzitierten_belegs_verhindert_finale_ausgabe() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        let revoked = Arc::new(AtomicBool::new(false));
        let inputs = Arc::new(Mutex::new(Vec::new()));
        let executions = Arc::new(Mutex::new(Vec::new()));
        let server = Server::new(kind, vec![kind.response(Some(&call(1)), &[]), kind.response(None, &["e-1-cited"])], revoked.clone(), true);
        let outcome = run(kind, &server, &context(), revoked, inputs.clone(), executions);
        assert_eq!(outcome.value.status, AnswerStatus::UnauthorizedEvidence);
        assert!(outcome.value.citations.is_empty());
        assert_eq!(inputs.lock().unwrap().len(), 2);
        assert_eq!(server.finish().len(), 2);
        assert_eq!(outcome.accounting.charged().network_rounds, 2);
    }
}

#[test]
fn konkrete_provider_pruefen_unzitierte_scopes_und_egress_vor_folgetransport() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        for remove_scope in [false, true] {
            let revoked = Arc::new(AtomicBool::new(false));
            let inputs = Arc::new(Mutex::new(Vec::new()));
            let server = Server::new(kind, vec![kind.response(Some(&call(1)), &[])], revoked, false);
            let provider = RecordingProvider::new(kind, &server.address, inputs);
            let mut next = context().with_request_deadline();
            let first = provider.answer_turn_accounted(&query(), &next, &[], &[definition()], &conversation(0)).unwrap();
            assert!(matches!(first.value, ProviderTurn::ToolCalls { .. }));
            if remove_scope {
                next.principal.scopes.clear();
            } else {
                next.principal.provider_egress.remove("internal");
            }
            let failure = provider.answer_turn_accounted(&query(), &next, &evidence(1), &[definition()], &conversation(1)).unwrap_err();
            assert!(matches!(failure.error, PortError::InvalidResponse(_)));
            assert_eq!(failure.accounting.unwrap().charged(), Usage::default());
            assert_eq!(server.finish().len(), 1);
        }
    }
}

#[test]
fn konkrete_provider_verwerfen_unbekannte_finalzitate_mit_abrechnung() {
    for kind in [Kind::Compatible, Kind::Subscription] {
        let revoked = Arc::new(AtomicBool::new(false));
        let inputs = Arc::new(Mutex::new(Vec::new()));
        let executions = Arc::new(Mutex::new(Vec::new()));
        let server = Server::new(kind, vec![kind.response(Some(&call(1)), &[]), kind.response(None, &["unknown-evidence"])], revoked.clone(), false);
        let outcome = run(kind, &server, &context(), revoked, inputs.clone(), executions);
        assert_eq!(outcome.value.status, AnswerStatus::Unavailable);
        assert!(outcome.value.citations.is_empty());
        assert_eq!(inputs.lock().unwrap().len(), 2);
        assert_eq!(server.finish().len(), 2);
        assert_eq!(outcome.accounting.charged().network_rounds, 2);
        assert_eq!(outcome.accounting.observed.input_tokens, 2);
        assert!(outcome.accounting.charged().input_tokens >= charge(kind, &[], &conversation(0)) + transport_input_ceiling(&kind.payload(&evidence(1), &conversation(1)), true).unwrap());
    }
}
