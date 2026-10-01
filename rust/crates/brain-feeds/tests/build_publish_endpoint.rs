//! Constructor and transport regressions. Only authored data and loopback stubs.
use brain_contracts::feeds::{
    BuildPublishRequest, BuildPublishState, BuildPublishStatus, BUILD_PUBLISH_VERSION,
};
use brain_feeds::build_publish::{BuildPublishPort, HttpBuildPublishClient, PublishError};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    thread,
    time::{Duration, Instant},
};

const TOKEN_ENV: &str = "BRAIN_FEEDS_ENDPOINT_USERINFO_TEST_TOKEN";
const TOKEN: &str = "synthetic-endpoint-token";
const TIMEOUT: Duration = Duration::from_secs(5);

// Install synthetic credentials and proxy settings only in a child process, not
// in the parallel test runner. The proxy trap also proves plaintext loopback
// requests cannot be diverted. No external endpoint is contacted.
fn isolated(test_name: &str) -> bool {
    isolated_proxy(test_name, false)
}

fn isolated_proxy(test_name: &str, expect_https_connect: bool) -> bool {
    const CHILD_ENV: &str = "BRAIN_FEEDS_ENDPOINT_TEST_CHILD";
    if std::env::var(CHILD_ENV).as_deref() == Ok(test_name) {
        assert_eq!(std::env::var(TOKEN_ENV).unwrap(), TOKEN);
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
                    body: "{}".into(),
                    location: None,
                })
                .collect(),
        )
    });
    let mut child = Command::new(std::env::current_exe().unwrap());
    child
        .args(["--exact", test_name, "--nocapture"])
        .env(CHILD_ENV, test_name)
        .env(TOKEN_ENV, TOKEN)
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

fn client(endpoint: &str) -> Result<HttpBuildPublishClient, PublishError> {
    HttpBuildPublishClient::new(endpoint, TOKEN_ENV, TIMEOUT)
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
    // Construction only: no DNS lookup, TLS handshake or external HTTP request.
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
            seen.push(Seen { head, body });
            write!(
                stream,
                "HTTP/1.1 {} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
                reply.code,
                reply.body.len()
            )
            .unwrap();
            if let Some(location) = reply.location {
                write!(stream, "Location: {location}\r\n").unwrap();
            }
            write!(stream, "\r\n{}", reply.body).unwrap();
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
    // The proxy refuses both CONNECT requests without forwarding anything.
    // Even a regression that bypasses the proxy can reach only this local trap.
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
    // Submit IDs are JSON/header data, not URL segments; keep their existing
    // broader contract rather than tightening it to match the status route.
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
    let ids = [
        "../steam",
        "//example.invalid",
        "id?query",
        "id#fragment",
        "id%2Fpath",
        "id@host",
    ];
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/api", listener.local_addr().unwrap());
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
    let client = client(&endpoint).unwrap();
    for id in ids {
        let mut request = request();
        request.request_id = id.into();
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
