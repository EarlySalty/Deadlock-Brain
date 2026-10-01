use serde_json::{json, Value};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const TOKEN: &str = "synthetic-private-pipe-token";

fn launch(dir: &std::path::Path, endpoint: &str, timeout: u64) -> Child {
    fs::write(
        dir.join("infisical.json"),
        serde_json::to_vec(&json!({
            "secret_values_fd": 3, "project_id": "fixture", "environment": "fixture",
            "secret_path": "/fixture", "socket_path": "/never-contact-infisical"
        }))
        .unwrap(),
    )
    .unwrap();
    let config = dir.join("client.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "endpoint": endpoint, "infisical_config": "infisical.json",
            "secret_reference": "BRAIN_SERVE_API_TOKEN"
        }))
        .unwrap(),
    )
    .unwrap();
    // The only secret transport is a private anonymous pipe inherited as FD 3.
    // The shell command and all arguments contain metadata, never secret values.
    Command::new("/bin/bash")
        .env_clear()
        .args(["-c", "exec 3<&0; exec \"$@\"", "fixture"])
        .arg(env!("CARGO_BIN_EXE_deadlock-brain"))
        .args(["answer", "--client-config"])
        .arg(config)
        .args([
            "--scope",
            "docs.public",
            "--request-id",
            "fixture-request",
            "--timeout-ms",
        ])
        .arg(timeout.to_string())
        .arg("fixture question")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn answer_reads_explicit_config_and_private_pipe_then_uses_typed_client() {
    let dir = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "fixture HTTP request missing");
                    thread::park_timeout(Duration::from_millis(2));
                }
                Err(_) => panic!("fixture accept failed"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(2)))
            .unwrap();
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
        assert!(head.starts_with("POST /v1/answer HTTP/1.1\r\n"));
        assert!(head
            .to_ascii_lowercase()
            .contains(&format!("authorization: bearer {TOKEN}\r\n")));
        assert!(length <= 16 * 1024);
        let mut bytes = vec![0; length];
        reader.read_exact(&mut bytes).unwrap();
        let query: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(query["request_id"], "fixture-request");
        assert_eq!(query["requested_scopes"], json!(["docs.public"]));
        let body = json!({"contract_version":"brain.public.v1", "request_id":"fixture-request",
            "knowledge_release":"fixture-release", "status":"answered", "text":"fixture answer",
            "citations":[{"citation_id":"fixture-citation", "label":"fixture source"}]}).to_string();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).unwrap();
    });
    let mut child = launch(dir.path(), &endpoint, 4000);
    child
        .stdin
        .take()
        .unwrap()
        .write_all(
            json!({"BRAIN_SERVE_API_TOKEN":TOKEN})
                .to_string()
                .as_bytes(),
        )
        .unwrap();
    let output = child.wait_with_output().unwrap();
    server.join().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(answer["text"], "fixture answer");
    assert!(!String::from_utf8_lossy(&output.stdout).contains(TOKEN));
}

#[test]
fn answer_missing_secret_is_sanitized_without_contacting_http() {
    let dir = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut child = launch(
        dir.path(),
        &format!("http://{}", listener.local_addr().unwrap()),
        1000,
    );
    child
        .stdin
        .take()
        .unwrap()
        .write_all(json!({"OTHER_SECRET":TOKEN}).to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Konfiguriertes Secret fehlt"));
    assert!(!stderr.contains(TOKEN));
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[test]
fn answer_startup_budget_bounds_a_private_pipe_that_never_replies() {
    let dir = tempfile::tempdir().unwrap();
    let child = launch(dir.path(), "http://127.0.0.1:1", 80);
    let started = Instant::now();
    // Keep the writer open: the loader's own 30-second bound must not determine
    // the CLI's deadline. wait_with_output would close stdin, so wait explicitly.
    let mut child = child;
    let writer = child.stdin.take().unwrap();
    let output = child.wait_with_output().unwrap();
    drop(writer);
    assert!(!output.status.success());
    assert!(started.elapsed() < Duration::from_secs(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("nicht rechtzeitig"));
}
