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
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let server_stop = stop.clone();
    let server_calls = calls.clone();
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
            thread::sleep(delay);
            let _ = stream.write_all(response.as_bytes());
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
    (result, calls.load(Ordering::SeqCst))
}
const CHAT: &str = r#"{"model":"fixture-model","choices":[{"message":{"content":"fixture answer"}}],"usage":{"prompt_tokens":12,"completion_tokens":3}}"#;
#[test]
fn authentication_redirect_and_missing_usage_do_not_retry() {
    for response in [json_response("401 Unauthorized","{}"),"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/secret\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into(),json_response("200 OK",r#"{"model":"fixture-model","choices":[{"message":{"content":"no usage"}}]}"#),json_response("200 OK",r#"{"model":"fixture-model","choices":[{"message":{"content":"bad usage"}}],"usage":{}}"#)] {
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
