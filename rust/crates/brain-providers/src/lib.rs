#![forbid(unsafe_code)]

use std::{
    thread,
    time::{Duration, Instant},
};

use brain_contracts::provider_input::{grounded_messages, ChatMessage};
use brain_contracts::{
    AnswerProviderPort, AuthorizedContext, Evidence, PortError, ProviderAnswer, Query, Usage,
};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone)]
pub struct ProviderConfig {
    api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout: Duration,
    pub retry_attempts: usize,
    pub retry_backoff: Duration,
    pub max_response_bytes: usize,
    /// Explicit price ceiling. None is permitted only for a loopback fixture server.
    pub pricing: Option<PriceCeiling>,
}

#[derive(Debug, Clone, Copy)]
pub struct PriceCeiling {
    pub input_micros_per_token: u64,
    pub output_micros_per_token: u64,
}

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
            base_url: base_url.into(),
            model: model.into(),
            timeout: Duration::from_secs(20),
            retry_attempts: 3,
            retry_backoff: Duration::from_millis(250),
            max_response_bytes: 2 * 1024 * 1024,
            pricing: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider Konfiguration ist unvollständig")]
    InvalidConfig,
    #[error("Provider transport failed")]
    Http(#[from] reqwest::Error),
    #[error("Provider antwortete mit HTTP {status}")]
    HttpStatus { status: StatusCode },
    #[error("Provider Budget ist ausgeschöpft")]
    BudgetExceeded,
    #[error("provider circuit open")]
    CircuitOpen,
    #[error("Provider Antwort überschreitet das Größenlimit")]
    ResponseTooLarge,
    #[error("Provider Antwort ist ungültig: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, ProviderError>;

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleProvider {
    client: Client,
    config: ProviderConfig,
    circuit: std::sync::Arc<std::sync::Mutex<circuit::Circuit>>,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(rename = "max_tokens")]
    max_completion_tokens: u32,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    #[serde(default)]
    model: Option<String>,
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<ProviderUsage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ProviderUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}

impl OpenAiCompatibleProvider {
    pub fn new(config: ProviderConfig) -> Result<Self> {
        if config.api_key.trim().is_empty()
            || config.base_url.trim().is_empty()
            || config.model.trim().is_empty()
        {
            return Err(ProviderError::InvalidConfig);
        }
        hardening::validate_endpoint(&config)?;
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
        })
    }

    fn request(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer> {
        let payload = ChatRequest {
            model: self.config.model.clone(),
            messages: grounded_messages(query, evidence),
            max_completion_tokens: context.budget.max_output_tokens,
            stream: false,
        };

        hardening::authorize(query, context, evidence)?;
        self.send_chat(payload, context, evidence)
    }
}

impl AnswerProviderPort for OpenAiCompatibleProvider {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError> {
        if context.budget.max_network_rounds == 0 || context.budget.max_output_tokens == 0 {
            return Err(PortError::BudgetExceeded);
        }
        if hardening::authorize(query, context, evidence).is_err() {
            return Err(PortError::InvalidResponse(
                "provider context or egress denied".into(),
            ));
        }
        self.with_circuit(|| self.request(query, context, evidence))
            .map_err(|error| match error {
                ProviderError::BudgetExceeded => PortError::BudgetExceeded,
                ProviderError::InvalidResponse(message) => PortError::InvalidResponse(message),
                ProviderError::ResponseTooLarge => {
                    PortError::InvalidResponse("provider response too large".into())
                }
                other => PortError::Unavailable(other.to_string()),
            })
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

    use brain_contracts::{AnswerProfile, Principal};

    use super::*;

    fn query() -> Query {
        Query {
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
                    let body = r#"{"model":"fixture-model","choices":[{"message":{"content":"Abrams Antwort"}}],"usage":{"prompt_tokens":12,"completion_tokens":3}}"#;
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
        // A failed first round is conservatively charged, not silently counted as free.
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
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            assert!(request.contains("\"max_tokens\":17"));
            write!(
                stream,
                "HTTP/1.1 429 Too Many Requests\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
        });

        let mut context = context();
        context.budget.max_network_rounds = 1;
        context.budget.max_output_tokens = 17;
        let error = provider(format!("http://{address}"))
            .answer(&query(), &context, &[])
            .unwrap_err();
        server.join().unwrap();

        assert!(matches!(error, PortError::Unavailable(_)));
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
