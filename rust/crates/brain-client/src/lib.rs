#![forbid(unsafe_code)]

use std::time::Duration;

use brain_contracts::{PublicAnswerResponse as AnswerResponse, Query};
use reqwest::{
    blocking::Client,
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use std::{io::Read, net::IpAddr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("ungueltige Brain Basis URL")]
    InvalidBaseUrl,
    #[error("ungueltiges Bearer Token")]
    InvalidToken,
    #[error("Brain HTTP Fehler: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Brain antwortete mit HTTP {status}: {body}")]
    HttpStatus { status: StatusCode, body: String },
    #[error("Brain Query ist ungültig: {0}")]
    InvalidQuery(String),
    #[error("Brain Antwort ist kein gültiger Contract: {0}")]
    Contract(#[from] serde_json::Error),
    #[error("Brain Antwort verletzt den öffentlichen Contract")]
    InvalidResponse,
    #[error("Brain Antwort überschreitet das Größenlimit")]
    ResponseTooLarge,
    #[error("Brain Antwort konnte nicht vollständig gelesen werden")]
    BodyRead(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Clone)]
pub struct BrainClient {
    client: Client,
    base_url: String,
    bearer: HeaderValue,
}

impl std::fmt::Debug for BrainClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BrainClient")
            .field("base_url", &self.base_url)
            .field("bearer", &"<redacted>")
            .finish()
    }
}

impl BrainClient {
    pub fn new(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let base_url = base_url.trim().trim_end_matches('/');
        let parsed = reqwest::Url::parse(base_url).map_err(|_| ClientError::InvalidBaseUrl)?;
        let loopback = parsed.host_str().is_some_and(|h| {
            h == "localhost"
                || h.trim_matches(['[', ']'])
                    .parse::<IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !(parsed.scheme() == "https" || (parsed.scheme() == "http" && loopback))
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
            || timeout.is_zero()
            || timeout > Duration::from_secs(60)
        {
            return Err(ClientError::InvalidBaseUrl);
        }
        if bearer_token.is_empty()
            || bearer_token.len() > 4096
            || bearer_token.chars().any(char::is_whitespace)
        {
            return Err(ClientError::InvalidToken);
        }
        let mut bearer = HeaderValue::from_str(&format!("Bearer {bearer_token}"))
            .map_err(|_| ClientError::InvalidToken)?;
        bearer.set_sensitive(true);
        let client = Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .build()?;
        Ok(Self {
            client,
            base_url: base_url.to_string(),
            bearer,
        })
    }

    pub fn answer(&self, query: &Query) -> Result<AnswerResponse> {
        query
            .validate()
            .map_err(|error| ClientError::InvalidQuery(error.to_string()))?;

        let response = self
            .client
            .post(format!("{}/v1/answer", self.base_url))
            .header(AUTHORIZATION, self.bearer.clone())
            .header(CONTENT_TYPE, "application/json")
            .json(query)
            .send()?;

        let status = response.status();
        const MAX_BYTES: usize = 512 * 1024;
        if response
            .content_length()
            .is_some_and(|n| n > MAX_BYTES as u64)
        {
            return Err(ClientError::ResponseTooLarge);
        }
        let is_json = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                v.split(';')
                    .next()
                    .is_some_and(|m| m.trim().eq_ignore_ascii_case("application/json"))
            });
        let mut bytes = Vec::new();
        response
            .take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > MAX_BYTES {
            return Err(ClientError::ResponseTooLarge);
        }
        if status != StatusCode::OK {
            let code = serde_json::from_slice::<brain_contracts::ApiErrorEnvelope>(&bytes)
                .ok()
                .map(|e| e.error.code)
                .filter(|code| {
                    code.len() <= 64 && code.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                })
                .unwrap_or_else(|| "http_error".into());
            return Err(ClientError::HttpStatus { status, body: code });
        }
        if !is_json {
            return Err(ClientError::InvalidResponse);
        }
        let answer: AnswerResponse = serde_json::from_slice(&bytes)?;
        answer
            .validate(&query.request_id)
            .map_err(|_| ClientError::InvalidResponse)?;
        Ok(answer)
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

    use brain_contracts::{
        AnswerProfile, AnswerStatus, PublicCitation, PUBLIC_API_VERSION as CONTRACT_VERSION,
    };

    use super::*;

    fn query() -> Query {
        Query {
            request_id: "request-1".into(),
            conversation_id: "conversation-1".into(),
            text: "Was macht Abrams?".into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        }
    }

    #[test]
    fn sends_typed_query_with_bearer_and_reads_typed_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = vec![0_u8; 8192];
            let read = stream.read(&mut request).unwrap();
            let raw = String::from_utf8_lossy(&request[..read]);
            assert!(raw.starts_with("POST /v1/answer HTTP/1.1\r\n"));
            assert!(raw
                .lines()
                .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-token")));
            assert!(raw.contains("\"request_id\":\"request-1\""));

            let body = serde_json::to_string(&AnswerResponse {
                contract_version: CONTRACT_VERSION.into(),
                request_id: "request-1".into(),
                knowledge_release: "release-1".into(),
                status: AnswerStatus::Answered,
                text: "Antwort".into(),
                citations: vec![PublicCitation {
                    citation_id: "cite-fixture".into(),
                    label: "Beleg 1".into(),
                }],
            })
            .unwrap();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        });

        let client = BrainClient::new(
            &format!("http://{addr}"),
            "test-token",
            Duration::from_secs(2),
        )
        .unwrap();
        let response = client.answer(&query()).unwrap();
        server.join().unwrap();

        assert_eq!(response.status, AnswerStatus::Answered);
        assert_eq!(response.request_id, "request-1");
    }

    #[test]
    fn debug_never_contains_bearer_token() {
        let client = BrainClient::new(
            "http://127.0.0.1:1",
            "do-not-print-me",
            Duration::from_secs(1),
        )
        .unwrap();
        let debug = format!("{client:?}");
        assert!(!debug.contains("do-not-print-me"));
        assert!(debug.contains("<redacted>"));
    }
}
