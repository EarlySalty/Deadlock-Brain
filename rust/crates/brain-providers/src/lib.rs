#![forbid(unsafe_code)]

use std::{
    thread,
    time::{Duration, Instant},
};

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
}

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
        }
    }
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider Konfiguration ist unvollständig")]
    InvalidConfig,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("Provider antwortete mit HTTP {status}")]
    HttpStatus { status: StatusCode },
    #[error("Provider Budget ist ausgeschöpft")]
    BudgetExceeded,
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
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(rename = "max_tokens")]
    max_completion_tokens: u32,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
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
    #[serde(default)]
    prompt_tokens: u64,
    #[serde(default)]
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
        let client = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { client, config })
    }

    fn request(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer> {
        let compact_evidence = evidence
            .iter()
            .map(|item| {
                serde_json::json!({
                    "id": item.evidence_id,
                    "citation": item.citation,
                    "content": item.content,
                })
            })
            .collect::<Vec<_>>();
        let payload = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: "Beantworte ausschließlich mit der gelieferten Deadlock Evidenz. Erfinde keine Quelle.".to_string(),
                },
                ChatMessage {
                    role: "user",
                    content: serde_json::json!({
                        "query": query.text,
                        "evidence": compact_evidence,
                    })
                    .to_string(),
                },
            ],
            max_completion_tokens: context.budget.max_output_tokens,
            stream: false,
        };

        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );
        let attempts = self
            .config
            .retry_attempts
            .max(1)
            .min(context.budget.max_network_rounds as usize);
        let deadline = Instant::now() + Duration::from_millis(context.deadline_ms.max(1));
        let mut last_status = None;

        for attempt in 0..attempts {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(ProviderError::BudgetExceeded);
            }
            let response = self
                .client
                .post(&url)
                .timeout(self.config.timeout.min(remaining))
                .bearer_auth(&self.config.api_key)
                .json(&payload)
                .send();

            match response {
                Ok(response) if response.status().is_success() => {
                    if response
                        .content_length()
                        .is_some_and(|length| length > self.config.max_response_bytes as u64)
                    {
                        return Err(ProviderError::ResponseTooLarge);
                    }
                    let bytes = response.bytes()?;
                    if bytes.len() > self.config.max_response_bytes {
                        return Err(ProviderError::ResponseTooLarge);
                    }
                    let parsed: ChatResponse = serde_json::from_slice(&bytes)
                        .map_err(|error| ProviderError::InvalidResponse(error.to_string()))?;
                    let text = parsed
                        .choices
                        .first()
                        .map(|choice| choice.message.content.trim().to_string())
                        .filter(|content| !content.is_empty())
                        .ok_or_else(|| {
                            ProviderError::InvalidResponse(
                                "choices[0].message.content fehlt".into(),
                            )
                        })?;
                    let usage = parsed.usage.unwrap_or(ProviderUsage {
                        prompt_tokens: 0,
                        completion_tokens: 0,
                    });
                    return Ok(ProviderAnswer {
                        text,
                        usage: Usage {
                            provider: Some("openai_compatible".into()),
                            model: parsed.model.or_else(|| Some(self.config.model.clone())),
                            input_tokens: usage.prompt_tokens,
                            output_tokens: usage.completion_tokens,
                            network_rounds: (attempt + 1) as u32,
                            cost_micros: 0,
                        },
                    });
                }
                Ok(response)
                    if response.status() == StatusCode::TOO_MANY_REQUESTS
                        || response.status().is_server_error() =>
                {
                    last_status = Some(response.status());
                }
                Ok(response) => {
                    return Err(ProviderError::HttpStatus {
                        status: response.status(),
                    });
                }
                Err(error) => {
                    if attempt + 1 == attempts {
                        return Err(ProviderError::Http(error));
                    }
                }
            }

            if attempt + 1 < attempts {
                let backoff = self
                    .config
                    .retry_backoff
                    .saturating_mul((attempt + 1) as u32);
                let remaining = deadline.saturating_duration_since(Instant::now());
                if backoff >= remaining {
                    return Err(ProviderError::BudgetExceeded);
                }
                thread::sleep(backoff);
            }
        }

        Err(ProviderError::HttpStatus {
            status: last_status.unwrap_or(StatusCode::SERVICE_UNAVAILABLE),
        })
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
        self.request(query, context, evidence)
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
        assert_eq!(result.usage.input_tokens, 12);
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
