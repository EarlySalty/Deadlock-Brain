#![forbid(unsafe_code)]
//! One typed Brain transport contract, with synchronous and native async clients.
//! No configuration discovery, provider fallback, deployment or message sending.
pub use brain_contracts::{
    AnswerProfile, AnswerStatus, PublicAnswerResponse, PublicCitation, Query, PUBLIC_API_VERSION,
};
use reqwest::{
    blocking::Client,
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use std::{io::Read, time::Duration};
use thiserror::Error;
use PublicAnswerResponse as AnswerResponse;
mod async_client;
mod transport;
pub use async_client::AsyncBrainClient;
pub use transport::{MAX_REQUEST_BYTES, MAX_RESPONSE_BYTES};

#[derive(Error)]
pub enum ClientError {
    #[error("ungültige Brain Basis-URL oder Frist")]
    InvalidBaseUrl,
    #[error("ungültiges Bearer-Token")]
    InvalidToken,
    #[error("Brain HTTP-Transport fehlgeschlagen")]
    Http(#[from] reqwest::Error),
    #[error("Brain antwortete mit HTTP {status}: {body}")]
    HttpStatus { status: StatusCode, body: String },
    #[error("Brain Query ist ungültig")]
    InvalidQuery(String),
    #[error("Brain Antwort ist kein gültiger Contract")]
    Contract(#[from] serde_json::Error),
    #[error("Brain Antwort verletzt den öffentlichen Contract")]
    InvalidResponse,
    #[error("Brain Anfrage überschreitet das Größenlimit")]
    RequestTooLarge,
    #[error("Brain Antwort überschreitet das Größenlimit")]
    ResponseTooLarge,
    #[error("Brain Antwort konnte nicht vollständig gelesen werden")]
    BodyRead(#[from] std::io::Error),
}
// reqwest's Debug may contain the endpoint. Public diagnostics remain redacted.
impl std::fmt::Debug for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Clone)]
pub struct BrainClient {
    client: Client,
    base_url: String,
    bearer: HeaderValue,
}
impl std::fmt::Debug for BrainClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrainClient")
            .field("endpoint", &"<configured>")
            .field("bearer", &"<redacted>")
            .finish()
    }
}
impl BrainClient {
    pub fn new(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let (base_url, bearer) = transport::endpoint(base_url, bearer_token, timeout)?;
        let client = Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()?;
        Ok(Self {
            client,
            base_url,
            bearer,
        })
    }
    pub fn answer(&self, query: &Query) -> Result<AnswerResponse> {
        let request = transport::encode_request(query)?;
        let response = self
            .client
            .post(format!("{}/v1/answer", self.base_url))
            .header(AUTHORIZATION, self.bearer.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(request)
            .send()?;
        let status = response.status();
        if response
            .content_length()
            .is_some_and(|n| n > MAX_RESPONSE_BYTES as u64)
        {
            return Err(ClientError::ResponseTooLarge);
        }
        let is_json = transport::json_content_type(response.headers());
        let mut bytes = Vec::new();
        response
            .take(MAX_RESPONSE_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        transport::decode_response(status, is_json, &bytes, &query.request_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::BTreeSet,
        io::{Read, Write},
        net::TcpListener,
        thread,
    };
    fn query() -> Query {
        Query {
            domain: None,
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
                contract_version: PUBLIC_API_VERSION.into(),
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
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
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
