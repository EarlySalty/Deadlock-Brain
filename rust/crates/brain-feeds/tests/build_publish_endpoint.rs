use brain_contracts::feeds::{
    BuildPublishErrorClass, BuildPublishRequest, BuildPublishState, BuildPublishStatus,
    BUILD_PUBLISH_VERSION,
};
use brain_feeds::build_publish::{BuildPublishPort, HttpBuildPublishClient, PublishError};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    thread,
    time::{Duration, Instant},
};

const TOKEN: &str = "synthetic-endpoint-token";
const TIMEOUT: Duration = Duration::from_secs(5);

fn isolated(test_name: &str) -> bool {
    isolated_proxy(test_name, false)
}

fn isolated_proxy(test_name: &str, expect_https_connect: bool) -> bool {
    const CHILD_ENV: &str = "BRAIN_FEEDS_ENDPOINT_TEST_CHILD";
    if std::env::var(CHILD_ENV).as_deref() == Ok(test_name) {
        return false;
    }
    let proxy = TcpListener::bind("127.0.0.1:0").unwrap();
    proxy.set_nonblocking(true).unwrap();
    let proxy_url = format!("http://{}", proxy.local_addr().unwrap());
    let proxy_server = expect_https_connect.then(|| {
        serve(
            proxy.try_clone().unwrap(),
            (0..2)
                .map(|_| Reply {
                    code: 403,
                    body: String::new(),
                    location: None,
                })
                .collect(),
        )
    });
    let mut child = Command::new(std::env::current_exe().unwrap());
    child
        .args(["--exact", test_name, "--nocapture"])
        .env(CHILD_ENV, test_name)
        .env("NO_PROXY", "")
        .env("no_proxy", "");
    for name in [
        "HTTP_PROXY",
        "http_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
    ] {
        child.env(name, &proxy_url);
    }
    let output = child.output().unwrap();
    assert!(
        output.status.success(),
        "{test_name} failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if let Some(server) = proxy_server {
        for received in server.join().unwrap() {
            assert!(received.head.starts_with("CONNECT 127.0.0.1:"));
            assert!(!received
                .head
                .to_ascii_lowercase()
                .contains("authorization:"));
            assert!(!received.head.contains(TOKEN));
            assert!(received.body.is_empty());
        }
    }
    assert!(matches!(proxy.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
    true
}

fn bound_reply(request: &BuildPublishRequest, state: BuildPublishState) -> Reply {
    Reply {
        code: 200,
        body: serde_json::to_string(&BuildPublishStatus {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: request.request_id.clone(),
            request_sha256: request.request_sha256().unwrap(),
            state,
            submitted_at: 1,
            updated_at: 2,
        })
        .unwrap(),
        location: None,
    }
}

#[test]
fn publish_waits_for_terminal_and_binds_every_poll_to_the_original_hash() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/gateway", listener.local_addr().unwrap());
    let server = serve(
        listener,
        vec![
            bound_reply(&request, BuildPublishState::Queued),
            bound_reply(&request, BuildPublishState::Running),
            bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            ),
        ],
    );
    let done = client(&endpoint)
        .unwrap()
        .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
        .unwrap();
    assert_eq!(
        done.state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 3);
    for poll in &seen {
        assert_bound_get(poll, &request, "/gateway");
    }
}

fn assert_bound_get(seen: &Seen, request: &BuildPublishRequest, base_path: &str) {
    assert!(seen.head.starts_with(&format!(
        "GET {base_path}/builds/v1/publish/{}?request_sha256={} HTTP/1.1\r\n",
        request.request_id,
        request.request_sha256().unwrap()
    )));
    assert!(seen.body.is_empty());
}

#[test]
fn bound_poll_rejects_a_foreign_hash_and_zero_build_id() {
    for bad_state in [false, true] {
        let request = request();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let mut bad = bound_reply(
            &request,
            BuildPublishState::Succeeded {
                hero_build_id: if bad_state { 0 } else { 5 },
            },
        );
        if !bad_state {
            bad.body = bad
                .body
                .replace(&request.request_sha256().unwrap(), &"0".repeat(64));
        }
        let server = serve(
            listener,
            vec![bound_reply(&request, BuildPublishState::Queued), bad],
        );
        assert!(matches!(
            client(&endpoint)
                .unwrap()
                .publish_and_wait(&request, TIMEOUT, Duration::ZERO),
            Err(PublishError::InvalidResponse(_))
        ));
        assert_eq!(server.join().unwrap().len(), 2);
    }
}

fn error_reply(code: u16) -> Reply {
    Reply {
        code,
        body: "{}".into(),
        location: None,
    }
}

#[test]
fn retries_reuse_identical_request_and_failed_states_remain_failed() {
    let request = brain_feeds::build_publish::deterministic_request(request()).unwrap();
    assert_eq!(
        brain_feeds::build_publish::deterministic_request(request.clone()).unwrap(),
        request
    );
    let mut changed = request.clone();
    changed.payload["new"] = serde_json::json!(1);
    assert_ne!(
        brain_feeds::build_publish::deterministic_request(changed)
            .unwrap()
            .request_id,
        request.request_id
    );
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = serve(
        listener,
        vec![
            error_reply(404),
            error_reply(503),
            error_reply(404),
            error_reply(429),
            error_reply(404),
            bound_reply(
                &request,
                BuildPublishState::Failed {
                    error_class: BuildPublishErrorClass::RateLimited,
                },
            ),
        ],
    );
    let failed = client(&endpoint)
        .unwrap()
        .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
        .unwrap();
    assert_eq!(
        failed.state,
        BuildPublishState::Failed {
            error_class: BuildPublishErrorClass::RateLimited
        }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 6);
    for lookup in seen.iter().step_by(2) {
        assert_bound_get(lookup, &request, "");
    }
    let attempts: Vec<_> = seen.iter().skip(1).step_by(2).collect();
    assert_eq!(attempts.len(), 3);
    for attempt in &attempts {
        assert!(attempt.head.starts_with("POST /builds/v1/publish "));
        assert!(attempt
            .head
            .contains(&format!("idempotency-key: {}\r\n", request.request_id)));
        assert_eq!(attempt.body, attempts[0].body);
        assert_eq!(
            serde_json::from_slice::<BuildPublishRequest>(&attempt.body).unwrap(),
            request
        );
    }
}

#[test]
fn rate_limit_and_invalid_wait_do_not_report_publication() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = serve(
        listener,
        (0..3)
            .map(|_| Reply {
                code: 429,
                body: "{}".into(),
                location: None,
            })
            .collect(),
    );
    let client = client(&endpoint).unwrap();
    for wait in [Duration::ZERO, Duration::from_secs(121)] {
        assert!(matches!(
            client.publish_and_wait(&request, wait, Duration::ZERO),
            Err(PublishError::InvalidRequest(_))
        ));
    }
    assert_eq!(
        client.publish_and_wait(&request, TIMEOUT, Duration::ZERO),
        Err(PublishError::RateLimited)
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 3);
    for lookup in &seen {
        assert_bound_get(lookup, &request, "");
    }
}

#[test]
fn confirmed_success_after_repeated_poll_throttling_remains_successful() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/gateway", listener.local_addr().unwrap());
    let server = serve(
        listener,
        vec![
            bound_reply(&request, BuildPublishState::Queued),
            error_reply(429),
            error_reply(429),
            bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            ),
        ],
    );
    assert_eq!(
        client(&endpoint)
            .unwrap()
            .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
            .unwrap()
            .state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 4);
    for lookup in &seen {
        assert_bound_get(lookup, &request, "/gateway");
    }
}

#[test]
fn missing_status_is_checked_before_submitting_the_unchanged_request() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/gateway", listener.local_addr().unwrap());
    let server = serve(
        listener,
        vec![
            error_reply(404),
            bound_reply(&request, BuildPublishState::Queued),
            bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            ),
        ],
    );
    assert_eq!(
        client(&endpoint)
            .unwrap()
            .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
            .unwrap()
            .state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 3);
    assert_bound_get(&seen[0], &request, "/gateway");
    assert!(seen[1].head.starts_with("POST /gateway/builds/v1/publish "));
    assert!(seen[1]
        .head
        .contains(&format!("idempotency-key: {}\r\n", request.request_id)));
    assert_eq!(
        serde_json::from_slice::<BuildPublishRequest>(&seen[1].body).unwrap(),
        request
    );
    assert_bound_get(&seen[2], &request, "/gateway");
}

#[test]
fn existing_terminal_states_are_reused_without_any_post() {
    let request = request();
    for state in [
        BuildPublishState::Succeeded { hero_build_id: 456 },
        BuildPublishState::Failed {
            error_class: BuildPublishErrorClass::RateLimited,
        },
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server = serve(
            listener,
            vec![
                bound_reply(&request, state.clone()),
                bound_reply(&request, state.clone()),
            ],
        );
        let client = client(&endpoint).unwrap();
        for _ in 0..2 {
            assert_eq!(
                client
                    .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
                    .unwrap()
                    .state,
                state
            );
        }
        let seen = server.join().unwrap();
        assert_eq!(seen.len(), 2);
        for lookup in &seen {
            assert_bound_get(lookup, &request, "");
        }
    }
}

#[test]
fn existing_running_publish_is_polled_without_a_post() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = serve(
        listener,
        vec![
            bound_reply(&request, BuildPublishState::Running),
            bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            ),
        ],
    );
    assert_eq!(
        client(&endpoint)
            .unwrap()
            .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
            .unwrap()
            .state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 2);
    for lookup in &seen {
        assert_bound_get(lookup, &request, "");
    }
}

#[test]
fn lost_post_response_is_recovered_by_hash_without_resubmitting() {
    let request = request();
    for state in [
        BuildPublishState::Succeeded { hero_build_id: 456 },
        BuildPublishState::Queued,
        BuildPublishState::Running,
        BuildPublishState::Failed {
            error_class: BuildPublishErrorClass::Timeout,
        },
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let mut replies = vec![
            Some(error_reply(404)),
            None,
            Some(bound_reply(&request, state.clone())),
        ];
        let expected = if state.is_terminal() {
            state
        } else {
            let terminal = BuildPublishState::Succeeded { hero_build_id: 456 };
            replies.push(Some(bound_reply(&request, terminal.clone())));
            terminal
        };
        let expected_requests = replies.len();
        let server = serve_optional(listener, replies);
        assert_eq!(
            client(&endpoint)
                .unwrap()
                .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
                .unwrap()
                .state,
            expected
        );
        let seen = server.join().unwrap();
        assert_eq!(seen.len(), expected_requests);
        assert!(seen[1].head.starts_with("POST /builds/v1/publish "));
        assert_eq!(
            serde_json::from_slice::<BuildPublishRequest>(&seen[1].body).unwrap(),
            request
        );
        for lookup in seen
            .iter()
            .enumerate()
            .filter_map(|(i, seen)| (i != 1).then_some(seen))
        {
            assert_bound_get(lookup, &request, "");
        }
    }
}

#[test]
fn recovery_keeps_polling_during_transient_lookup_errors() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = serve_optional(
        listener,
        vec![
            Some(error_reply(404)),
            None,
            Some(error_reply(503)),
            Some(error_reply(429)),
            Some(bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            )),
        ],
    );
    assert_eq!(
        client(&endpoint)
            .unwrap()
            .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
            .unwrap()
            .state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 5);
    assert!(seen[1].head.starts_with("POST /builds/v1/publish "));
    for lookup in &seen[2..] {
        assert_bound_get(lookup, &request, "");
    }
}

#[test]
fn final_post_attempt_is_still_recovered_before_returning_its_error() {
    let request = request();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = serve_optional(
        listener,
        vec![
            Some(error_reply(404)),
            Some(error_reply(503)),
            Some(error_reply(404)),
            Some(error_reply(429)),
            Some(error_reply(404)),
            None,
            Some(bound_reply(
                &request,
                BuildPublishState::Succeeded { hero_build_id: 456 },
            )),
        ],
    );
    assert_eq!(
        client(&endpoint)
            .unwrap()
            .publish_and_wait(&request, TIMEOUT, Duration::ZERO)
            .unwrap()
            .state,
        BuildPublishState::Succeeded { hero_build_id: 456 }
    );
    let seen = server.join().unwrap();
    assert_eq!(seen.len(), 7);
    for lookup in seen.iter().step_by(2) {
        assert_bound_get(lookup, &request, "");
    }
    for attempt in seen.iter().skip(1).step_by(2) {
        assert!(attempt.head.starts_with("POST /builds/v1/publish "));
        assert!(attempt
            .head
            .contains(&format!("idempotency-key: {}\r\n", request.request_id)));
        assert_eq!(attempt.body, seen[1].body);
    }
}

#[test]
fn preflight_and_recovery_reject_a_foreign_hash_or_invalid_build_id() {
    let request = request();
    for recover in [false, true] {
        for zero_build_id in [false, true] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let mut bad = bound_reply(
                &request,
                BuildPublishState::Succeeded {
                    hero_build_id: if zero_build_id { 0 } else { 456 },
                },
            );
            if !zero_build_id {
                bad.body = bad
                    .body
                    .replace(&request.request_sha256().unwrap(), &"0".repeat(64));
            }
            let replies = if recover {
                vec![Some(error_reply(404)), None, Some(bad)]
            } else {
                vec![Some(bad)]
            };
            let expected_requests = replies.len();
            let server = serve_optional(listener, replies);
            assert!(matches!(
                client(&endpoint)
                    .unwrap()
                    .publish_and_wait(&request, TIMEOUT, Duration::ZERO),
                Err(PublishError::InvalidResponse(_))
            ));
            let seen = server.join().unwrap();
            assert_eq!(seen.len(), expected_requests);
            assert_bound_get(&seen[expected_requests - 1], &request, "");
            if !recover {
                assert_bound_get(&seen[0], &request, "");
            }
        }
    }
}

fn client(endpoint: &str) -> Result<HttpBuildPublishClient, PublishError> {
    HttpBuildPublishClient::new(endpoint, TOKEN, TIMEOUT)
}

#[test]
fn constructor_requires_a_bounded_explicit_bearer_without_environment_lookup() {
    for token in ["", " ", "has space", "line\nbreak", &"a".repeat(4097)] {
        assert!(matches!(
            HttpBuildPublishClient::new("http://127.0.0.1:9", token, TIMEOUT),
            Err(PublishError::Unauthorized)
        ));
    }
    assert!(
        HttpBuildPublishClient::new("http://127.0.0.1:9", "synthetic-direct-token", TIMEOUT)
            .is_ok()
    );
}

#[test]
fn constructor_rejects_userinfo_that_impersonates_loopback() {
    if isolated("constructor_rejects_userinfo_that_impersonates_loopback") {
        return;
    }
    assert!(
        matches!(
            client("http://127.0.0.1:1234@example.invalid"),
            Err(PublishError::InvalidRequest(_))
        ),
        "userinfo must not be accepted as the destination host"
    );
}

#[test]
fn constructor_rejects_host_spoofing_userinfo_and_unsupported_urls() {
    if isolated("constructor_rejects_host_spoofing_userinfo_and_unsupported_urls") {
        return;
    }
    for endpoint in [
        "http://localhost:1234@example.invalid",
        "http://[::1]:1234@example.invalid",
        "http://127.0.0.1.example.invalid:1234",
        "http://localhost.example.invalid:1234",
        "http://localhost.:1234",
        "http://example.invalid:1234",
        "http://192.168.1.1:1234",
        "http://0.0.0.0:1234",
        "http://[::]:1234",
        "http://[::ffff:127.0.0.1]:1234",
        "http://user@127.0.0.1:1234",
        "http://:password@localhost:1234",
        "https://user:password@example.invalid",
        "https://user@example.invalid",
        "https://:password@example.invalid",
        "https://user%40localhost@example.invalid",
        "ftp://127.0.0.1:1234",
        "file:///tmp/build",
        "ws://127.0.0.1:1234",
        "wss://example.invalid",
        "//127.0.0.1:1234",
        "not a URL",
        "https://",
        "https://example.invalid:invalid",
        "https://example.invalid:65536",
        "http://127.0.0.1:65536",
        "http://127.0.0.1:0",
        "https://example.invalid:0",
    ] {
        assert!(
            matches!(client(endpoint), Err(PublishError::InvalidRequest(_))),
            "unsafe endpoint accepted: {endpoint}"
        );
    }
}

#[test]
fn constructor_rejects_queries_and_fragments_including_empty_ones() {
    if isolated("constructor_rejects_queries_and_fragments_including_empty_ones") {
        return;
    }
    for base in ["https://example.invalid", "http://127.0.0.1:1234/base"] {
        for suffix in [
            "?",
            "?key=fixture",
            "#",
            "#fragment",
            "?key=fixture#fragment",
        ] {
            let endpoint = format!("{base}{suffix}");
            assert!(
                matches!(client(&endpoint), Err(PublishError::InvalidRequest(_))),
                "query/fragment must not swallow the API path: {endpoint}"
            );
        }
    }
}

#[test]
fn constructor_accepts_https_and_real_loopback_hosts_with_valid_ports_and_paths() {
    if isolated("constructor_accepts_https_and_real_loopback_hosts_with_valid_ports_and_paths") {
        return;
    }
    for endpoint in [
        "https://example.invalid",
        "https://example.invalid/",
        "https://example.invalid:443/api",
        "https://example.invalid:8443/api/",
        "https://example.invalid:65535/api",
        "http://localhost",
        "http://localhost:80",
        "http://LOCALHOST:1234/base",
        "http://127.0.0.1",
        "http://127.0.0.1:1",
        "http://127.0.0.2:1234",
        "http://127.255.255.254:65535/api/",
        "http://[::1]",
        "http://[::1]:1234/api",
        "http://[0:0:0:0:0:0:0:1]:1234/api/",
        "https://example.invalid/api/%3Fkey%3Dfixture/%23fragment",
    ] {
        assert!(
            client(endpoint).is_ok(),
            "valid endpoint rejected: {endpoint}"
        );
    }
}

fn request() -> BuildPublishRequest {
    BuildPublishRequest {
        contract_version: BUILD_PUBLISH_VERSION.into(),
        request_id: "endpoint-1_test".into(),
        hero_id: 25,
        hero_name: "Warden".into(),
        build_name: "Synthetic endpoint regression".into(),
        payload: serde_json::json!({"categories": []}),
        caller: "brain-test".into(),
    }
}

fn status_body() -> String {
    let request = request();
    serde_json::to_string(&BuildPublishStatus {
        contract_version: BUILD_PUBLISH_VERSION.into(),
        request_id: request.request_id.clone(),
        request_sha256: request.request_sha256().unwrap(),
        state: BuildPublishState::Queued,
        submitted_at: 1,
        updated_at: 1,
    })
    .unwrap()
}

#[test]
fn submit_and_status_share_safe_dotted_request_ids() {
    if isolated("submit_and_status_share_safe_dotted_request_ids") {
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/base", listener.local_addr().unwrap());
    let mut request = request();
    request.request_id = "build.1".into();
    let response = serde_json::to_string(&BuildPublishStatus {
        contract_version: BUILD_PUBLISH_VERSION.into(),
        request_id: request.request_id.clone(),
        request_sha256: request.request_sha256().unwrap(),
        state: BuildPublishState::Queued,
        submitted_at: 1,
        updated_at: 1,
    })
    .unwrap();
    let server = serve(
        listener,
        vec![
            Reply {
                code: 202,
                body: response.clone(),
                location: None,
            },
            Reply {
                code: 200,
                body: response,
                location: None,
            },
        ],
    );
    let client = client(&endpoint).unwrap();
    let submitted = client.submit(&request).unwrap();
    assert_eq!(client.status(&request.request_id).unwrap(), submitted);
    let seen = server.join().unwrap();
    assert!(seen[1]
        .head
        .starts_with("GET /base/builds/v1/publish/build.1 HTTP/1.1\r\n"));
    for id in [
        "",
        ".",
        "..",
        "a/b",
        "a\\b",
        "%2e",
        "%2f",
        "a?b",
        "a#b",
        "a b",
        &"a".repeat(129),
    ] {
        request.request_id = id.into();
        assert!(
            matches!(
                client.submit(&request),
                Err(PublishError::InvalidRequest(_))
            ),
            "submit accepted {id:?}"
        );
        assert!(
            matches!(client.status(id), Err(PublishError::InvalidRequest(_))),
            "status accepted {id:?}"
        );
    }
}

struct Reply {
    code: u16,
    body: String,
    location: Option<String>,
}

#[derive(Debug)]
struct Seen {
    head: String,
    body: Vec<u8>,
}

fn accept(listener: &TcpListener) -> TcpStream {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        match listener.accept() {
            Ok((stream, _)) => return stream,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < deadline, "local stub received no request");
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) => panic!("local stub accept failed: {error}"),
        }
    }
}

fn serve(listener: TcpListener, replies: Vec<Reply>) -> thread::JoinHandle<Vec<Seen>> {
    serve_optional(listener, replies.into_iter().map(Some).collect())
}

fn serve_optional(
    listener: TcpListener,
    replies: Vec<Option<Reply>>,
) -> thread::JoinHandle<Vec<Seen>> {
    listener.set_nonblocking(true).unwrap();
    thread::spawn(move || {
        let mut seen = Vec::new();
        for reply in replies {
            let mut stream = accept(&listener);
            stream.set_read_timeout(Some(TIMEOUT)).unwrap();
            stream.set_write_timeout(Some(TIMEOUT)).unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut head = String::new();
            let mut length = 0;
            loop {
                let mut line = String::new();
                assert_ne!(reader.read_line(&mut line).unwrap(), 0);
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
                head.push_str(&line);
                assert!(head.len() <= 16 * 1024);
            }
            assert!(length <= 16 * 1024);
            let mut body = vec![0; length];
            reader.read_exact(&mut body).unwrap();
            let refused_connect = head.starts_with("CONNECT ")
                && reply.as_ref().is_some_and(|reply| reply.code == 403);
            seen.push(Seen { head, body });
            let Some(reply) = reply else {
                continue;
            };
            let mut response = format!(
                "HTTP/1.1 {} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
                reply.code,
                reply.body.len()
            );
            if let Some(location) = reply.location {
                response.push_str(&format!("Location: {location}\r\n"));
            }
            response.push_str("\r\n");
            response.push_str(&reply.body);
            if let Err(error) = stream.write_all(response.as_bytes()) {
                assert!(
                    refused_connect
                        && matches!(
                            error.kind(),
                            std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset
                        ),
                    "local stub response failed: {error}"
                );
            }
        }
        seen
    })
}

fn round_trip(bind: &str, host: Option<&str>, base_path: &str, expected_path: &str) {
    let listener = TcpListener::bind(bind).unwrap();
    let address = listener.local_addr().unwrap();
    let authority = host
        .map(|host| format!("{host}:{}", address.port()))
        .unwrap_or_else(|| address.to_string());
    let server = serve(
        listener,
        vec![
            Reply {
                code: 202,
                body: status_body(),
                location: None,
            },
            Reply {
                code: 200,
                body: status_body(),
                location: None,
            },
        ],
    );
    let client = client(&format!("http://{authority}{base_path}")).unwrap();
    let request = request();
    let submitted = client.submit(&request).unwrap();
    assert_eq!(submitted.state, BuildPublishState::Queued);
    assert_eq!(client.status(&request.request_id).unwrap(), submitted);
    let seen = server.join().unwrap();
    assert!(seen[0]
        .head
        .starts_with(&format!("POST {expected_path} HTTP/1.1\r\n")));
    assert!(seen[1].head.starts_with(&format!(
        "GET {expected_path}/{} HTTP/1.1\r\n",
        request.request_id
    )));
    for received in &seen {
        let head = received.head.to_ascii_lowercase();
        assert!(head.contains(&format!("host: {authority}\r\n")));
        assert!(head.contains(&format!("authorization: bearer {TOKEN}\r\n")));
    }
    assert!(seen[0]
        .head
        .contains("idempotency-key: endpoint-1_test\r\n"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&seen[0].body).unwrap(),
        serde_json::to_value(request).unwrap()
    );
    assert!(seen[1].body.is_empty());
}

#[test]
fn submit_and_status_preserve_the_validated_origin_port_and_base_path() {
    if isolated("submit_and_status_preserve_the_validated_origin_port_and_base_path") {
        return;
    }
    for (base, path) in [
        ("", "/builds/v1/publish"),
        ("/", "/builds/v1/publish"),
        ("/gateway", "/gateway/builds/v1/publish"),
        ("/gateway/", "/gateway/builds/v1/publish"),
        ("/gateway//", "/gateway//builds/v1/publish"),
        ("//example.invalid", "//example.invalid/builds/v1/publish"),
        (
            "/gateway/team%20one",
            "/gateway/team%20one/builds/v1/publish",
        ),
        (
            "/gateway/%2Fexample.invalid",
            "/gateway/%2Fexample.invalid/builds/v1/publish",
        ),
        (
            "/gateway/@example.invalid",
            "/gateway/@example.invalid/builds/v1/publish",
        ),
        (
            "/gateway/%3Fkey%3Dfixture/%23fragment",
            "/gateway/%3Fkey%3Dfixture/%23fragment/builds/v1/publish",
        ),
    ] {
        round_trip("127.0.0.1:0", None, base, path);
    }
}

#[test]
fn submit_and_status_support_ipv4_ipv6_and_localhost() {
    if isolated("submit_and_status_support_ipv4_ipv6_and_localhost") {
        return;
    }
    for (bind, host) in [
        ("127.0.0.2:0", None),
        ("[::1]:0", None),
        ("127.0.0.1:0", Some("localhost")),
    ] {
        round_trip(bind, host, "/api", "/api/builds/v1/publish");
    }
}

#[test]
fn https_preserves_proxy_support_without_plaintext_credentials_or_build_data() {
    if isolated_proxy(
        "https_preserves_proxy_support_without_plaintext_credentials_or_build_data",
        true,
    ) {
        return;
    }
    let target = TcpListener::bind("127.0.0.1:0").unwrap();
    target.set_nonblocking(true).unwrap();
    let client = client(&format!("https://{}", target.local_addr().unwrap())).unwrap();
    assert!(matches!(
        client.submit(&request()),
        Err(PublishError::Unavailable(_))
    ));
    assert!(matches!(
        client.status(&request().request_id),
        Err(PublishError::Unavailable(_))
    ));
    assert!(matches!(target.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
}

#[test]
fn submit_and_status_never_follow_redirects() {
    if isolated("submit_and_status_never_follow_redirects") {
        return;
    }
    for code in [301, 302, 303, 307, 308] {
        let trap = TcpListener::bind("127.0.0.1:0").unwrap();
        trap.set_nonblocking(true).unwrap();
        for location in [
            format!("http://{}/stolen", trap.local_addr().unwrap()),
            "/stolen".into(),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let server = serve(
                listener,
                (0..2)
                    .map(|_| Reply {
                        code,
                        body: "{}".into(),
                        location: Some(location.clone()),
                    })
                    .collect(),
            );
            let client = client(&endpoint).unwrap();
            assert_eq!(
                client.submit(&request()),
                Err(PublishError::Unavailable(format!("HTTP {code}")))
            );
            assert_eq!(
                client.status(&request().request_id),
                Err(PublishError::Unavailable(format!("HTTP {code}")))
            );
            let seen = server.join().unwrap();
            assert!(seen[0].head.starts_with("POST /builds/v1/publish "));
            assert!(seen[1]
                .head
                .starts_with("GET /builds/v1/publish/endpoint-1_test "));
        }
        assert!(matches!(trap.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
    }
}

#[test]
fn invalid_request_ids_cannot_change_the_route_or_send_a_request() {
    if isolated("invalid_request_ids_cannot_change_the_route_or_send_a_request") {
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let client = client(&format!("http://{}/api", listener.local_addr().unwrap())).unwrap();
    for id in [
        "",
        "../steam",
        "//example.invalid",
        "id?query",
        "id#fragment",
        "id%2Fpath",
        "id@host",
        "id\r\n",
    ] {
        assert!(matches!(
            client.status(id),
            Err(PublishError::InvalidRequest(_))
        ));
    }
    for id in ["", "id\r\n"] {
        let mut invalid = request();
        invalid.request_id = id.into();
        assert!(matches!(
            client.submit(&invalid),
            Err(PublishError::InvalidRequest(_))
        ));
    }
    assert!(matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
}

#[test]
fn submit_request_ids_are_data_and_cannot_change_the_destination() {
    if isolated("submit_request_ids_are_data_and_cannot_change_the_destination") {
        return;
    }
    let unsafe_ids = [
        "../steam",
        "//example.invalid",
        "id?query",
        "id#fragment",
        "id%2Fpath",
        "id@host",
    ];
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let endpoint = format!("http://{}/api", listener.local_addr().unwrap());
    let client = client(&endpoint).unwrap();
    for id in unsafe_ids {
        let mut request = request();
        request.request_id = id.into();
        assert!(BuildPublishRequest::validate_request_id(id).is_err());
        assert!(request.validate().is_err());
        assert!(matches!(
            client.submit(&request),
            Err(PublishError::InvalidRequest(_))
        ));
        assert!(matches!(
            client.status(id),
            Err(PublishError::InvalidRequest(_))
        ));
    }
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
    let maximum_id = "a".repeat(128);
    let ids = ["build.1", "endpoint-1_test", maximum_id.as_str()];
    let server = serve(
        listener,
        ids.iter()
            .map(|_| Reply {
                code: 401,
                body: "{}".into(),
                location: None,
            })
            .collect(),
    );
    for id in ids {
        let mut request = request();
        request.request_id = id.into();
        BuildPublishRequest::validate_request_id(id).unwrap();
        request.validate().unwrap();
        assert_eq!(client.submit(&request), Err(PublishError::Unauthorized));
    }
    for (id, received) in ids.into_iter().zip(server.join().unwrap()) {
        assert!(received
            .head
            .starts_with("POST /api/builds/v1/publish HTTP/1.1\r\n"));
        assert!(received
            .head
            .contains(&format!("idempotency-key: {id}\r\n")));
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&received.body).unwrap()["request_id"],
            id
        );
    }
}
