#![forbid(unsafe_code)]

use std::time::{Duration, Instant};

use brain_contracts::provider_input::{grounded_turn_payload_with_quality, ToolWireFormat};
use brain_contracts::{
    Accounted, AnswerProviderPort, AuthorizedContext, Evidence, PortError, PortFailure,
    ProviderAnswer, ProviderTurn, Query, ToolConversation, ToolDefinition, Usage, UsageAccounting,
};
use reqwest::{blocking::Client, StatusCode};
use thiserror::Error;

#[derive(Clone)]
pub struct ProviderConfig {
    api_key: String,
    subscription: bool,
    pub quality_filters: bool,
    pub base_url: String,
    pub model: String,
    pub timeout: Duration,
    pub retry_attempts: usize,
    pub retry_backoff: Duration,
    pub max_response_bytes: usize,
    pub pricing: Option<PriceCeiling>,
}

#[derive(Debug, Clone, Copy)]
pub struct PriceCeiling {
    pub input_micros_per_token: u64,
    pub output_micros_per_token: u64,
}

mod audit;
pub use audit::OutputDeviation;
mod circuit;
mod embeddings;
mod hardening;
mod transport;

impl std::fmt::Debug for ProviderConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderConfig")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("timeout", &self.timeout)
            .field("retry_attempts", &self.retry_attempts)
            .field("retry_backoff", &self.retry_backoff)
            .field("max_response_bytes", &self.max_response_bytes)
            .finish()
    }
}

impl ProviderConfig {
    pub fn new(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            subscription: false,
            quality_filters: true,
            base_url: base_url.into(),
            model: model.into(),
            timeout: Duration::from_secs(20),
            retry_attempts: 3,
            retry_backoff: Duration::from_millis(250),
            max_response_bytes: 2 * 1024 * 1024,
            pricing: None,
        }
    }

    pub fn codex_subscription(base_url: impl Into<String>) -> Self {
        let mut config = Self::new("", base_url, "gpt-6-luna");
        config.subscription = true;
        config.retry_attempts = 1;
        config
    }
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider Konfiguration ist unvollständig")]
    InvalidConfig,
    #[error("Provider transport failed")]
    Http(#[from] reqwest::Error),
    #[error("Provider body transport failed")]
    BodyTransport(#[source] std::io::Error),
    #[error("Provider antwortete mit HTTP {status}")]
    HttpStatus { status: StatusCode },
    #[error("Provider Budget ist ausgeschöpft")]
    BudgetExceeded,
    #[error("provider circuit open")]
    CircuitOpen,
    #[error("Provider Antwort überschreitet das Größenlimit")]
    ResponseTooLarge,
    #[error("local response audit unavailable")]
    AuditUnavailable,
    #[error("Provider Antwort ist ungültig: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, ProviderError>;

#[derive(Clone)]
pub struct OpenAiCompatibleProvider {
    client: Client,
    config: ProviderConfig,
    circuit: std::sync::Arc<std::sync::Mutex<circuit::Circuit>>,
    audit: Option<std::sync::Arc<dyn brain_contracts::response_audit::ResponseAuditPort>>,
}

impl std::fmt::Debug for OpenAiCompatibleProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiCompatibleProvider")
            .field("config", &self.config)
            .field("audit_enabled", &self.audit.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct CodexSubscriptionProvider(OpenAiCompatibleProvider);

impl CodexSubscriptionProvider {
    pub fn with_response_audit(
        mut self,
        audit: std::sync::Arc<dyn brain_contracts::response_audit::ResponseAuditPort>,
    ) -> Self {
        self.0 = self.0.with_response_audit(audit);
        self
    }

    pub fn new(config: ProviderConfig) -> Result<Self> {
        if !config.subscription {
            return Err(ProviderError::InvalidConfig);
        }
        OpenAiCompatibleProvider::new(config).map(Self)
    }
}

impl AnswerProviderPort for CodexSubscriptionProvider {
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<Accounted<ProviderAnswer>, PortFailure> {
        self.0.answer_accounted(query, context, evidence)
    }

    fn answer_turn_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> std::result::Result<Accounted<ProviderTurn>, PortFailure> {
        self.0
            .answer_turn_accounted(query, context, evidence, tools, conversation)
    }

    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError> {
        self.0.answer(query, context, evidence)
    }

    fn answer_turn(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> std::result::Result<ProviderTurn, PortError> {
        self.0
            .answer_turn(query, context, evidence, tools, conversation)
    }
}

impl OpenAiCompatibleProvider {
    fn report_failure(&self, request_id: Option<&str>, error: &ProviderError) {
        let category = match error {
            ProviderError::HttpStatus { status } => format!("http_{}", status.as_u16()),
            ProviderError::Http(error) if error.is_builder() || error.is_decode() => {
                "invalid_config".into()
            }
            ProviderError::Http(error) if error.is_timeout() => "transport_timeout".into(),
            ProviderError::Http(error) if error.is_connect() => "transport_connect".into(),
            ProviderError::Http(_) => "transport".into(),
            ProviderError::BodyTransport(_) => "transport_body".into(),
            ProviderError::InvalidConfig => "invalid_config".into(),
            ProviderError::BudgetExceeded => "budget_exceeded".into(),
            ProviderError::CircuitOpen => "circuit_open".into(),
            ProviderError::AuditUnavailable => "audit_unavailable".into(),
            ProviderError::ResponseTooLarge => "response_too_large".into(),
            ProviderError::InvalidResponse(message) => match message.as_str() {
                "invalid chat schema" => "chat_schema",
                "model or choice count mismatch" => "model_or_choice_mismatch",
                "empty provider answer" => "empty_answer",
                "provider usage missing" => "usage_missing",
                "grounded answer envelope missing" => "grounded_envelope",
                "unknown, duplicate or missing citation" => "citation_invalid",
                "invalid provider context or query egress" => "context_egress_denied",
                "evidence egress denied" => "evidence_egress_denied",
                "request evidence denied" => "request_evidence_denied",
                "invalid evidence" => "evidence_invalid",
                "too many evidence items" => "evidence_count",
                _ => "invalid_response",
            }
            .into(),
        };
        self.report_category(request_id, category);
    }

    fn report_category(&self, request_id: Option<&str>, category: String) {
        self.report_event("provider_failure", request_id, category);
    }

    fn report_event(&self, event: &'static str, request_id: Option<&str>, category: String) {
        let request_id =
            request_id.map(|id| brain_contracts::response_audit::diagnostic_text(id, None));
        eprintln!(
            "{}",
            serde_json::json!({"event":event, "request_id":request_id, "error_class":category})
        );
    }

    pub fn with_response_audit(
        mut self,
        audit: std::sync::Arc<dyn brain_contracts::response_audit::ResponseAuditPort>,
    ) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn new(config: ProviderConfig) -> Result<Self> {
        if (!config.subscription && config.api_key.trim().is_empty())
            || config.base_url.trim().is_empty()
            || config.model.trim().is_empty()
        {
            return Err(ProviderError::InvalidConfig);
        }
        hardening::validate_endpoint(&config)?;
        if config.subscription {
            let url =
                reqwest::Url::parse(&config.base_url).map_err(|_| ProviderError::InvalidConfig)?;
            if !url.host_str().is_some_and(|host| {
                host.trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
            }) || config.model != "gpt-6-luna"
                || !config.api_key.is_empty()
                || config.retry_attempts != 1
                || config.pricing.is_some()
            {
                return Err(ProviderError::InvalidConfig);
            }
        }
        let client = Client::builder()
            .timeout(config.timeout)
            .connect_timeout(config.timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .build()?;
        Ok(Self {
            client,
            config,
            circuit: std::sync::Arc::new(std::sync::Mutex::new(circuit::Circuit::default())),
            audit: None,
        })
    }

    fn request(
        &self,
        context: &AuthorizedContext,
        request: transport::ChatRequest<'_>,
        accounting: &mut UsageAccounting,
        transport_failure: &mut Option<bool>,
    ) -> Result<ProviderTurn> {
        context
            .check_deadline()
            .map_err(|_| ProviderError::BudgetExceeded)?;
        let format = if self.config.subscription {
            ToolWireFormat::Native
        } else {
            ToolWireFormat::OpenAiCompatible
        };
        let mut payload = grounded_turn_payload_with_quality(
            request.query,
            request.evidence,
            request.tools,
            request.conversation,
            format,
            self.config.quality_filters,
        )
        .map_err(provider_contract_error)?;
        payload["model"] = serde_json::json!(self.config.model);
        payload["max_tokens"] = serde_json::json!(context.budget.max_output_tokens);
        payload["stream"] = serde_json::json!(false);
        if self.config.model == "accounts/fireworks/models/deepseek-v4p1-flash" {
            payload["reasoning_effort"] = serde_json::json!("none");
        }
        self.send_chat(payload, context, request, accounting, transport_failure)
    }
}

fn provider_contract_error(error: PortError) -> ProviderError {
    match error {
        PortError::BudgetExceeded => ProviderError::BudgetExceeded,
        _ => ProviderError::InvalidResponse("invalid tool contract".into()),
    }
}

impl AnswerProviderPort for OpenAiCompatibleProvider {
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<Accounted<ProviderAnswer>, PortFailure> {
        let Accounted { value, accounting } = self.answer_turn_accounted(
            query,
            context,
            evidence,
            &[],
            &ToolConversation::default(),
        )?;
        let value = value
            .into_answer()
            .map_err(|error| PortFailure::accounted(error, accounting.clone()))?;
        Ok(Accounted { value, accounting })
    }

    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError> {
        self.answer_turn(query, context, evidence, &[], &ToolConversation::default())?
            .into_answer()
    }

    fn answer_turn(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> std::result::Result<ProviderTurn, PortError> {
        self.answer_turn_accounted(query, context, evidence, tools, conversation)
            .map(|turn| turn.value)
            .map_err(|failure| failure.error)
    }

    fn answer_turn_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> std::result::Result<Accounted<ProviderTurn>, PortFailure> {
        if (!tools.is_empty() || !conversation.messages.is_empty())
            && context.request_deadline.is_none()
        {
            return Err(PortFailure::before_call(PortError::InvalidResponse(
                "tool turns require a bound request deadline".into(),
            )));
        }
        let bound = context.with_request_deadline();
        let context = &bound;
        context.check_deadline().map_err(PortFailure::before_call)?;
        if context.budget.max_network_rounds == 0 || context.budget.max_output_tokens == 0 {
            return Err(PortFailure::before_call(PortError::BudgetExceeded));
        }
        if let Err(error) = hardening::authorize_turn(query, context, evidence, tools, conversation)
        {
            self.report_failure(Some(&query.request_id), &error);
            return Err(PortFailure::before_call(PortError::InvalidResponse(
                "provider context or egress denied".into(),
            )));
        }
        let mut accounting = UsageAccounting::default();
        let result = self.with_circuit(|transport_failure| {
            self.request(
                context,
                transport::ChatRequest {
                    query,
                    evidence,
                    tools,
                    conversation,
                },
                &mut accounting,
                transport_failure,
            )
        });
        eprintln!(
            "{}",
            serde_json::json!({
                "event": "brain_provider_usage",
                "request_id": query.request_id,
                "provider": accounting.observed.provider,
                "model": accounting.observed.model,
                "input_tokens": accounting.observed.input_tokens,
                "output_tokens": accounting.observed.output_tokens,
                "network_rounds": accounting.observed.network_rounds,
                "unaccounted": accounting.unaccounted
                    || accounting.reserved.output_tokens > 0,
                "success": result.is_ok()
            })
        );
        match result {
            Ok(value) => Ok(Accounted { value, accounting }),
            Err(error) => {
                self.report_failure(Some(&query.request_id), &error);
                let error = match error {
                    ProviderError::BudgetExceeded => PortError::BudgetExceeded,
                    ProviderError::InvalidResponse(message) => PortError::InvalidResponse(message),
                    ProviderError::ResponseTooLarge => {
                        PortError::InvalidResponse("provider response too large".into())
                    }
                    other => PortError::Unavailable(other.to_string()),
                };
                Err(PortFailure::accounted(error, accounting))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeSet,
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use brain_contracts::provider_input::grounded_messages;
    use brain_contracts::{AnswerProfile, Principal};

    use super::*;

    fn query() -> Query {
        Query {
            answer_context: None,
            domain: None,
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: "Was macht Abrams?".into(),
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
                actor_id: "test".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "c1".into(),
            knowledge_release: "k1".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
        }
    }

    fn provider(url: String) -> OpenAiCompatibleProvider {
        let mut config = ProviderConfig::new("fixture-token", url, "fixture-model");
        config.retry_attempts = 2;
        config.retry_backoff = Duration::from_millis(1);
        OpenAiCompatibleProvider::new(config).unwrap()
    }

    fn read_request(stream: &mut std::net::TcpStream) -> String {
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut data = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(count) => {
                    data.extend_from_slice(&buffer[..count]);
                    if let Some(header_end) =
                        data.windows(4).position(|window| window == b"\r\n\r\n")
                    {
                        let headers = String::from_utf8_lossy(&data[..header_end + 4]);
                        let length = headers
                            .lines()
                            .find_map(|line| {
                                line.split_once(':').and_then(|(name, value)| {
                                    name.eq_ignore_ascii_case("content-length")
                                        .then(|| value.trim().parse::<usize>().ok())
                                        .flatten()
                                })
                            })
                            .unwrap_or(0);
                        if data.len() >= header_end + 4 + length {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }
        String::from_utf8_lossy(&data).into_owned()
    }

    #[test]
    fn relaxed_quality_delivers_model_text_without_exposing_envelope_or_ids() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for text in [
                "Pocket unter Druck setzen <@123456789012345678>.",
                "```json\n{\"text\":\"Abstand halten\",\"cited_evidence_ids\":[\"unknown\"],\"extra\":\"nicht ausliefern\"}\n```",
                "{\"text\":\"Deckung nutzen\",\"cited_evidence_ids\":[]}",
                "Ohne passende Quellen trotzdem antworten.",
                "{\"text\":\"Teilantwort <#123456789012345678>",
                "Deckung nutzen [[ev-657479902dd7919055375b33f82b0d5c926c3837a917be14]] [[]]",
                "Teilantwort [[ev-unvollständig",
            ] {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_request(&mut stream);
                let (_, body) = request.split_once("\r\n\r\n").unwrap();
                let payload: serde_json::Value = serde_json::from_str(body).unwrap();
                let system = payload["system"].as_str().unwrap();
                assert!(system.contains("NEVER"));
                assert!(system.contains("MUST NOT"));
                assert!(!system.contains("gib exakt {\"text\":\"\""));
                let stop_reason = if text.starts_with("Ohne passende") { "max_tokens" } else { "end_turn" };
                let response = serde_json::json!({"model":"gpt-6-luna","stop_reason":stop_reason,"content":[{"type":"text","text":text}],"usage":{"input_tokens":12,"output_tokens":3}}).to_string();
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response.len(), response).unwrap();
            }
        });
        let mut config = ProviderConfig::codex_subscription(format!("http://{address}/v1"));
        config.quality_filters = false;
        let provider = CodexSubscriptionProvider::new(config).unwrap();
        for expected in [
            "Pocket unter Druck setzen .",
            "Abstand halten",
            "Deckung nutzen",
            "Ohne passende Quellen trotzdem antworten.",
            "Teilantwort ",
            "Deckung nutzen  ",
            "Teilantwort ",
        ] {
            let response = provider
                .answer_accounted(&query(), &context(), &[])
                .unwrap();
            assert_eq!(response.value.text, expected);
            assert!(response.value.cited_evidence_ids.is_empty());
            assert_eq!(response.accounting.observed.input_tokens, 12);
            assert_eq!(response.accounting.observed.output_tokens, 3);
        }
        server.join().unwrap();
    }

    #[test]
    fn subscription_uses_native_messages_without_credentials_or_tools() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            assert!(request.starts_with("POST /v1/messages "));
            assert!(!request.to_ascii_lowercase().contains("authorization:"));
            let (_, body) = request.split_once("\r\n\r\n").unwrap();
            let payload: serde_json::Value = serde_json::from_str(body).unwrap();
            assert_eq!(payload["model"], "gpt-6-luna");
            assert_eq!(payload["output_config"]["effort"], "low");
            assert_eq!(payload["stream"], false);
            assert!(payload["system"]
                .as_str()
                .unwrap()
                .contains("cited_evidence_ids"));
            assert_eq!(payload["messages"].as_array().unwrap().len(), 1);
            assert_eq!(payload["tools"], serde_json::json!([]));
            assert_eq!(payload["tool_choice"], serde_json::json!({"type":"none"}));
            let response = r#"{"model":"gpt-6-luna","stop_reason":"end_turn","content":[{"type":"thinking","thinking":"Nicht ausgeben"},{"type":"text","text":"Antwort"}],"usage":{"input_tokens":12,"output_tokens":3}}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response.len(),
                response
            )
            .unwrap();
        });
        let provider = CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(format!(
            "http://{address}/v1"
        )))
        .unwrap();
        let answer = provider.answer(&query(), &context(), &[]).unwrap();
        assert_eq!(answer.text, "Antwort");
        assert_eq!(answer.usage.provider.as_deref(), Some("codex_subscription"));
        server.join().unwrap();
        assert!(
            CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(
                "https://example.com/v1"
            ))
            .is_err()
        );
    }

    #[test]
    fn retries_rate_limit_and_parses_typed_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for index in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_request(&mut stream);
                assert!(request.contains("authorization: Bearer fixture-token"));
                if index == 0 {
                    write!(
                        stream,
                        "HTTP/1.1 429 Too Many Requests\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                    )
                    .unwrap();
                } else {
                    let body = r#"{"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"content":"Abrams Antwort"}}],"usage":{"prompt_tokens":12,"completion_tokens":3}}"#;
                    write!(
                        stream,
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    )
                    .unwrap();
                }
            }
        });

        let result = provider(format!("http://{address}"))
            .answer(&query(), &context(), &[])
            .unwrap();
        assert_eq!(result.text, "Abrams Antwort");
        assert_eq!(result.usage.network_rounds, 2);
        assert!(result.usage.input_tokens > 12);
        assert!(result.usage.input_tokens <= context().budget.max_input_tokens as u64);
        assert!(result.usage.output_tokens > 3);
        assert!(result.usage.output_tokens <= context().budget.max_output_tokens as u64);
        server.join().unwrap();
    }

    #[test]
    fn invalid_json_is_rejected_without_fallback() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let _ = read_request(&mut stream);
            let body = "{not-json";
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        });

        let error = provider(format!("http://{address}"))
            .answer(&query(), &context(), &[])
            .unwrap_err();
        assert!(matches!(error, PortError::InvalidResponse(_)));
        server.join().unwrap();
    }

    #[test]
    fn request_budget_limits_retries_and_output_tokens() {
        for (model, reasoning_effort) in [
            ("fixture-model", None),
            (
                "accounts/fireworks/models/deepseek-v4p1-flash",
                Some("none"),
            ),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let server = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_request(&mut stream);
                let (_, body) = request.split_once("\r\n\r\n").unwrap();
                let body: serde_json::Value = serde_json::from_str(body).unwrap();
                let mut expected = serde_json::json!({
                    "model": model,
                    "messages": grounded_messages(&query(), &[]),
                    "max_tokens": 17,
                    "stream": false,
                });
                if let Some(effort) = reasoning_effort {
                    expected["reasoning_effort"] = serde_json::json!(effort);
                }
                assert_eq!(body, expected);
                write!(
                stream,
                "HTTP/1.1 429 Too Many Requests\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
                .unwrap();
            });

            let mut context = context();
            context.budget.max_network_rounds = 1;
            context.budget.max_output_tokens = 17;
            let mut config =
                ProviderConfig::new("fixture-token", format!("http://{address}"), model);
            config.retry_attempts = 2;
            config.retry_backoff = Duration::from_millis(1);
            let error = OpenAiCompatibleProvider::new(config)
                .unwrap()
                .answer(&query(), &context, &[])
                .unwrap_err();
            server.join().unwrap();

            assert!(matches!(error, PortError::Unavailable(_)));
        }
    }

    #[test]
    fn zero_round_budget_blocks_before_network() {
        let mut context = context();
        context.budget.max_network_rounds = 0;
        let error = provider("http://127.0.0.1:1".to_string())
            .answer(&query(), &context, &[])
            .unwrap_err();
        assert!(matches!(error, PortError::BudgetExceeded));
    }

    #[test]
    fn debug_redacts_token() {
        let config = ProviderConfig::new("do-not-log", "http://localhost", "fixture");
        let debug = format!("{config:?}");
        assert!(!debug.contains("do-not-log"));
        assert!(debug.contains("<redacted>"));
    }
}
