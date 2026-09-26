//! Real loopback HTTP, authored fixtures only. No provider, bot or production DSN.
use brain_client::{AsyncBrainClient, ClientError, MAX_RESPONSE_BYTES};
use brain_contracts::{AnswerProfile, AnswerStatus, Query};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::{Duration, Instant},
};

fn query() -> Query {
    Query {
        domain: None,
        request_id: "fixture-request".into(),
        conversation_id: "isolated-conversation".into(),
        text: "Abrams äöü 🧪".into(),
        requested_scopes: BTreeSet::from(["docs.public".into()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}
fn answer() -> Value {
    json!({"contract_version":"brain.public.v1", "request_id":"fixture-request", "knowledge_release":"fixture-release", "status":"answered", "text":"Belegte Antwort äöü 🧪", "citations":[{"citation_id":"opaque-1", "label":"Beleg 1"}]})
}
fn fixture(
    status: u16,
    headers: String,
    body: String,
    delay: Duration,
) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let n = stream.read(&mut buffer).unwrap();
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..n]);
            if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                let header = String::from_utf8_lossy(&request[..end]);
                let size: usize = header
                    .lines()
                    .find_map(|l| {
                        l.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|v| v.trim().parse().unwrap())
                    })
                    .unwrap_or(0);
                if request.len() >= end + 4 + size {
                    break;
                }
            }
            assert!(request.len() <= 70 * 1024);
        }
        thread::sleep(delay);
        let reply =
            format!("HTTP/1.1 {status} Fixture\r\n{headers}Connection: close\r\n\r\n{body}");
        // Oversize/timeout/cancellation tests intentionally close the socket early.
        let _ = stream.write_all(reply.as_bytes());
        String::from_utf8(request).unwrap()
    });
    (endpoint, handle)
}
async fn exchange(
    status: u16,
    content_type: &str,
    body: String,
) -> (
    Result<brain_contracts::PublicAnswerResponse, ClientError>,
    String,
) {
    let headers = format!(
        "Content-Type: {content_type}\r\nContent-Length: {}\r\n",
        body.len()
    );
    let (endpoint, worker) = fixture(status, headers, body, Duration::ZERO);
    let client = AsyncBrainClient::new(&endpoint, "fixture-token", Duration::from_secs(3)).unwrap();
    let result = client.answer(&query()).await;
    (result, worker.join().unwrap())
}

#[tokio::test]
async fn preserves_typed_wire_auth_and_unicode_without_authority_fields() {
    let (result, request) =
        exchange(200, "application/json; charset=utf-8", answer().to_string()).await;
    let response = result.unwrap();
    assert_eq!(response.status, AnswerStatus::Answered);
    assert_eq!(response.text, "Belegte Antwort äöü 🧪");
    assert!(request.starts_with("POST /v1/answer HTTP/1.1\r\n"));
    assert!(request
        .lines()
        .any(|l| l.eq_ignore_ascii_case("authorization: Bearer fixture-token")));
    let body = request.split_once("\r\n\r\n").unwrap().1;
    let wire: Value = serde_json::from_str(body).unwrap();
    assert_eq!(wire, serde_json::to_value(query()).unwrap());
    assert!(wire.get("principal").is_none());
    assert!(wire.get("provider").is_none());
}

#[tokio::test]
async fn rejects_contract_drift_request_mismatch_and_unproven_answers() {
    let mut cases = Vec::new();
    for (field, value) in [
        ("contract_version", json!("brain.v999")),
        ("request_id", json!("another-user")),
        ("knowledge_release", json!("")),
        ("text", json!("")),
        ("citations", json!([])),
        ("provider_secret", json!("must-not-escape")),
    ] {
        let mut body = answer();
        body[field] = value;
        cases.push(body);
    }
    let mut duplicate = answer();
    duplicate["citations"] =
        json!([{"citation_id":"dup", "label":"1"}, {"citation_id":"dup", "label":"2"}]);
    cases.push(duplicate);
    for body in cases {
        assert!(exchange(200, "application/json", body.to_string())
            .await
            .0
            .is_err());
    }
    assert!(exchange(200, "text/html", answer().to_string())
        .await
        .0
        .is_err());
    assert!(exchange(200, "application/json", "not-json".into())
        .await
        .0
        .is_err());
}

#[tokio::test]
async fn status_errors_do_not_return_arbitrary_upstream_codes_or_messages() {
    for status in [401, 403, 429, 500, 503] {
        let body = json!({"error":{"code":"SECRET_VALUE_WITH_VALID_IDENTIFIER_SYNTAX", "message":"raw private upstream body"}}).to_string();
        let error = exchange(status, "application/json", body)
            .await
            .0
            .unwrap_err();
        let displayed = format!("{error}");
        assert!(!displayed.contains("SECRET_VALUE"));
        assert!(!displayed.contains("private upstream"));
        assert!(matches!(error, ClientError::HttpStatus { body, .. } if body == "http_error"));
    }
}

#[tokio::test]
async fn redirect_is_rejected_without_contacting_redirect_destination() {
    let sink = TcpListener::bind("127.0.0.1:0").unwrap();
    sink.set_nonblocking(true).unwrap();
    let headers = format!(
        "Location: http://{}/collect\r\nContent-Length: 0\r\n",
        sink.local_addr().unwrap()
    );
    let (endpoint, worker) = fixture(307, headers, String::new(), Duration::ZERO);
    let client = AsyncBrainClient::new(&endpoint, "fixture-token", Duration::from_secs(2)).unwrap();
    assert!(matches!(
        client.answer(&query()).await,
        Err(ClientError::HttpStatus { .. })
    ));
    worker.join().unwrap();
    assert_eq!(
        sink.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[tokio::test]
async fn oversized_declared_and_chunked_bodies_are_bounded() {
    let (endpoint, worker) = fixture(
        200,
        format!(
            "Content-Length: {}\r\nContent-Type: application/json\r\n",
            MAX_RESPONSE_BYTES + 1
        ),
        String::new(),
        Duration::ZERO,
    );
    let client = AsyncBrainClient::new(&endpoint, "fixture-token", Duration::from_secs(2)).unwrap();
    assert!(matches!(
        client.answer(&query()).await,
        Err(ClientError::ResponseTooLarge)
    ));
    worker.join().unwrap();
    let chunk = "x".repeat(MAX_RESPONSE_BYTES + 1);
    let body = format!("{:x}\r\n{chunk}\r\n0\r\n\r\n", chunk.len());
    let (endpoint, worker) = fixture(
        200,
        "Transfer-Encoding: chunked\r\nContent-Type: application/json\r\n".into(),
        body,
        Duration::ZERO,
    );
    let client = AsyncBrainClient::new(&endpoint, "fixture-token", Duration::from_secs(2)).unwrap();
    assert!(matches!(
        client.answer(&query()).await,
        Err(ClientError::ResponseTooLarge)
    ));
    worker.join().unwrap();
}

#[tokio::test]
async fn serialized_request_limit_is_checked_before_io() {
    let client = AsyncBrainClient::new(
        "http://127.0.0.1:1",
        "fixture-token",
        Duration::from_secs(1),
    )
    .unwrap();
    let mut q = query();
    q.text = "\u{1}".repeat(20_000);
    q.validate().unwrap();
    assert!(matches!(
        client.answer(&q).await,
        Err(ClientError::RequestTooLarge)
    ));
}

#[tokio::test]
async fn configured_deadline_expires_on_a_nonresponding_fixture() {
    let (endpoint, worker) = fixture(
        200,
        "Content-Length: 0\r\n".into(),
        String::new(),
        Duration::from_millis(250),
    );
    let client =
        AsyncBrainClient::new(&endpoint, "fixture-token", Duration::from_millis(40)).unwrap();
    let error = client.answer(&query()).await.unwrap_err();
    assert!(matches!(error, ClientError::Http(ref e) if e.is_timeout()));
    worker.join().unwrap();
}

#[test]
fn synthetic_contract_throughput_reports_measurements_not_production_slos() {
    let fixture = answer().to_string();
    let iterations = 4096;
    let started = Instant::now();
    for _ in 0..iterations {
        let response: brain_contracts::PublicAnswerResponse =
            serde_json::from_str(std::hint::black_box(&fixture)).unwrap();
        response.validate("fixture-request").unwrap();
        std::hint::black_box(response);
    }
    println!(
        "{}",
        json!({"benchmark":"synthetic_contract_decode_validate", "production_slo":null, "network_included":false, "iterations":iterations, "fixture_bytes":fixture.len(), "elapsed_ns":started.elapsed().as_nanos(), "profile":if cfg!(debug_assertions) {"debug"} else {"release"}})
    );
}
