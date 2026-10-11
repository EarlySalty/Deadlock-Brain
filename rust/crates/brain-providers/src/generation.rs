use super::{circuit, hardening, ProviderError, Result};
use brain_contracts::RequestDeadline;
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct GenerationEndpoint {
    api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub retry_attempts_on_429: u32,
    pub max_response_bytes: usize,
}

impl std::fmt::Debug for GenerationEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenerationEndpoint")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("retry_attempts_on_429", &self.retry_attempts_on_429)
            .field("max_response_bytes", &self.max_response_bytes)
            .finish()
    }
}

impl GenerationEndpoint {
    pub fn new(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        timeout: Duration,
        retry_attempts_on_429: u32,
        max_response_bytes: usize,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            timeout,
            retry_attempts_on_429,
            max_response_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct GenerationMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct TextGenerationRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [GenerationMessage<'a>],
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedText {
    pub content: String,
    pub model: String,
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationError {
    Unavailable,
    RateLimited,
    Timeout,
    InvalidOutput,
}

#[derive(Clone)]
pub struct TextGenerationProvider {
    client: Client,
    endpoint: GenerationEndpoint,
    circuit: Arc<Mutex<circuit::Circuit>>,
}

impl std::fmt::Debug for TextGenerationProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextGenerationProvider")
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct Completion {
    model: Option<String>,
    choices: Vec<Choice>,
    usage: Option<CompletionUsage>,
}

#[derive(Deserialize)]
struct Choice {
    message: Option<ChoiceMessage>,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct CompletionUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
}

impl TextGenerationProvider {
    pub fn new(endpoint: GenerationEndpoint) -> Result<Self> {
        hardening::endpoint_loopback(&endpoint.base_url)?;
        if endpoint.api_key.trim().is_empty()
            || endpoint.timeout.is_zero()
            || endpoint.timeout > Duration::from_secs(600)
            || endpoint.retry_attempts_on_429 > 8
            || !(128..=8 * 1024 * 1024).contains(&endpoint.max_response_bytes)
        {
            return Err(ProviderError::InvalidConfig);
        }
        let client = Client::builder()
            .timeout(endpoint.timeout)
            .connect_timeout(endpoint.timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .build()?;
        Ok(Self {
            client,
            endpoint,
            circuit: Arc::new(Mutex::new(circuit::Circuit::default())),
        })
    }

    pub fn generate(
        &self,
        request: &TextGenerationRequest<'_>,
        deadline: &RequestDeadline,
    ) -> std::result::Result<GeneratedText, GenerationError> {
        if request.model.trim().is_empty() || request.messages.is_empty() {
            return Err(GenerationError::Unavailable);
        }
        circuit::guarded(&self.circuit, Instant::now, |_| {
            self.send(request, deadline)
        })
        .map_err(|error| classify(&error))
    }

    fn send(
        &self,
        request: &TextGenerationRequest<'_>,
        deadline: &RequestDeadline,
    ) -> Result<GeneratedText> {
        let payload = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "stream": false,
            "reasoning_effort": "none",
        });
        let url = format!(
            "{}/chat/completions",
            self.endpoint.base_url.trim_end_matches('/')
        );
        let mut attempt = 0;
        loop {
            let remaining = deadline
                .remaining()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let response = self
                .client
                .post(&url)
                .bearer_auth(&self.endpoint.api_key)
                .json(&payload)
                .timeout(self.endpoint.timeout.min(remaining))
                .send()?;
            deadline
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let status = response.status();
            if status.is_success() {
                let bytes = hardening::read_bounded(response, self.endpoint.max_response_bytes)?;
                return completion(&bytes, request.model);
            }
            if status != StatusCode::TOO_MANY_REQUESTS
                || attempt >= self.endpoint.retry_attempts_on_429
            {
                return Err(ProviderError::HttpStatus { status });
            }
            let Ok(delay) = hardening::retry_after(&response) else {
                return Err(ProviderError::HttpStatus { status });
            };
            let delay = delay.unwrap_or_default();
            if deadline
                .remaining()
                .map_or(true, |remaining| delay >= remaining)
            {
                return Err(ProviderError::HttpStatus { status });
            }
            deadline
                .wait(delay)
                .map_err(|_| ProviderError::BudgetExceeded)?;
            attempt += 1;
        }
    }
}

fn completion(bytes: &[u8], requested_model: &str) -> Result<GeneratedText> {
    let invalid = || ProviderError::InvalidResponse("invalid generation".into());
    let completion: Completion = serde_json::from_slice(bytes).map_err(|_| invalid())?;
    let [choice] = completion.choices.as_slice() else {
        return Err(invalid());
    };
    let content = choice
        .message
        .as_ref()
        .and_then(|message| message.content.as_deref())
        .map(str::trim)
        .filter(|content| !content.is_empty())
        .ok_or_else(invalid)?;
    let model = completion
        .model
        .filter(|model| !model.trim().is_empty())
        .unwrap_or_else(|| requested_model.to_owned());
    Ok(GeneratedText {
        content: content.to_owned(),
        model,
        prompt_tokens: completion.usage.as_ref().and_then(|u| u.prompt_tokens),
        completion_tokens: completion.usage.as_ref().and_then(|u| u.completion_tokens),
    })
}

fn classify(error: &ProviderError) -> GenerationError {
    match error {
        ProviderError::HttpStatus { status } if *status == StatusCode::TOO_MANY_REQUESTS => {
            GenerationError::RateLimited
        }
        ProviderError::Http(error) if error.is_timeout() => GenerationError::Timeout,
        ProviderError::BudgetExceeded => GenerationError::Timeout,
        ProviderError::InvalidResponse(_) | ProviderError::ResponseTooLarge => {
            GenerationError::InvalidOutput
        }
        _ => GenerationError::Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    fn request_body(stream: &mut std::net::TcpStream) -> serde_json::Value {
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut data = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let count = stream.read(&mut buffer).unwrap();
            data.extend_from_slice(&buffer[..count]);
            let text = String::from_utf8_lossy(&data).into_owned();
            if let Some((head, body)) = text.split_once("\r\n\r\n") {
                let length = head
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|value| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap();
                if body.len() >= length {
                    return serde_json::from_str(body).unwrap();
                }
            }
        }
    }

    fn reply(stream: &mut std::net::TcpStream, status: &str, extra: &str, body: &str) {
        write!(
            stream,
            "HTTP/1.1 {status}\r\n{extra}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
    }

    fn provider(address: std::net::SocketAddr, retries: u32) -> TextGenerationProvider {
        TextGenerationProvider::new(GenerationEndpoint::new(
            "fixture-key",
            format!("http://{address}/v1"),
            Duration::from_secs(2),
            retries,
            4096,
        ))
        .unwrap()
    }

    fn generate(
        provider: &TextGenerationProvider,
    ) -> std::result::Result<GeneratedText, GenerationError> {
        let messages = [GenerationMessage {
            role: "user",
            content: "Vorgabe",
        }];
        provider.generate(
            &TextGenerationRequest {
                model: "fixture-model",
                messages: &messages,
                max_tokens: 32,
                temperature: 0.5,
            },
            &RequestDeadline::after(Duration::from_secs(5)),
        )
    }

    #[test]
    fn sends_plain_body_retries_429_and_returns_content_only() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = request_body(&mut stream);
            assert_eq!(
                body,
                serde_json::json!({"model":"fixture-model","messages":[{"role":"user","content":"Vorgabe"}],"max_tokens":32,"temperature":0.5,"stream":false,"reasoning_effort":"none"})
            );
            reply(
                &mut stream,
                "429 Too Many Requests",
                "Retry-After: 0\r\n",
                "{}",
            );
            let (mut stream, _) = listener.accept().unwrap();
            request_body(&mut stream);
            reply(
                &mut stream,
                "200 OK",
                "",
                r#"{"model":"fixture-model","choices":[{"message":{"content":"  Titel  ","reasoning_content":"geheim"}}],"usage":{"prompt_tokens":7,"completion_tokens":2}}"#,
            );
        });
        let generated = generate(&provider(address, 1)).unwrap();
        server.join().unwrap();
        assert_eq!(
            generated,
            GeneratedText {
                content: "Titel".into(),
                model: "fixture-model".into(),
                prompt_tokens: Some(7),
                completion_tokens: Some(2),
            }
        );
    }

    #[test]
    fn failures_are_typed_without_fallback() {
        for (status, body, expected) in [
            ("429 Too Many Requests", "{}", GenerationError::RateLimited),
            ("401 Unauthorized", "{}", GenerationError::Unavailable),
            (
                "200 OK",
                r#"{"choices":[]}"#,
                GenerationError::InvalidOutput,
            ),
            (
                "200 OK",
                r#"{"choices":[{"message":{"content":null,"reasoning_content":"nur Denken"}}]}"#,
                GenerationError::InvalidOutput,
            ),
            ("200 OK", &"x".repeat(5000), GenerationError::InvalidOutput),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let body = body.to_owned();
            let server = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                request_body(&mut stream);
                reply(&mut stream, status, "", &body);
            });
            assert_eq!(generate(&provider(address, 0)), Err(expected));
            server.join().unwrap();
        }
    }

    #[test]
    fn retry_after_beyond_deadline_stops_as_rate_limited() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            request_body(&mut stream);
            reply(
                &mut stream,
                "429 Too Many Requests",
                "Retry-After: 3600\r\n",
                "{}",
            );
        });
        assert_eq!(
            generate(&provider(address, 3)),
            Err(GenerationError::RateLimited)
        );
        server.join().unwrap();
    }

    #[test]
    fn endpoint_is_hardened_and_key_redacted() {
        for (url, key) in [
            ("http://example.com/v1", "key"),
            ("https://user@example.com/v1", "key"),
            ("https://example.com/v1?x=1", "key"),
            ("https://example.com/v1", " "),
        ] {
            assert!(TextGenerationProvider::new(GenerationEndpoint::new(
                key,
                url,
                Duration::from_secs(1),
                1,
                4096
            ))
            .is_err());
        }
        let endpoint = GenerationEndpoint::new(
            "do-not-log",
            "https://example.com/v1",
            Duration::from_secs(1),
            1,
            4096,
        );
        assert!(!format!("{endpoint:?}").contains("do-not-log"));
        assert!(TextGenerationProvider::new(endpoint).is_ok());
    }
}
