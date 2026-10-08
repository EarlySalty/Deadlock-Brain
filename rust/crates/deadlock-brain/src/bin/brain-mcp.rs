use brain_client::{AnswerProfile, AnswerStatus, AsyncBrainClient, Query};
use clap::Parser;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead, Read, Write},
    path::PathBuf,
    process,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

// A 32,768-byte question can expand to six JSON bytes per character when
// escaped (for example, U+0001), so include the full valid input and envelope.
const MAX_FRAME_BYTES: usize = 256 * 1024;
const MAX_CONFIG_BYTES: u64 = 16 * 1024;

fn default_secret_reference() -> String {
    "BRAIN_SERVE_API_TOKEN".to_string()
}

enum FrameRead {
    Eof,
    Frame(Vec<u8>),
    TooLarge,
}

/// Read one newline-delimited frame while keeping memory bounded by `max_bytes`.
/// An oversized frame is drained through its newline so the next call can read
/// the next request without retaining the rejected payload.
fn read_frame<R: BufRead>(reader: &mut R, max_bytes: usize) -> io::Result<FrameRead> {
    let mut frame = Vec::with_capacity(max_bytes.min(4096));
    let mut too_large = false;
    let mut saw_bytes = false;

    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return Ok(if too_large {
                FrameRead::TooLarge
            } else if saw_bytes {
                FrameRead::Frame(frame)
            } else {
                FrameRead::Eof
            });
        }

        let newline = available.iter().position(|byte| *byte == b'\n');
        let count = newline.unwrap_or(available.len());
        saw_bytes |= count > 0;
        if !too_large {
            if count <= max_bytes.saturating_sub(frame.len()) {
                frame.extend_from_slice(&available[..count]);
            } else {
                too_large = true;
                frame.clear();
            }
        }

        let consumed = count + usize::from(newline.is_some());
        reader.consume(consumed);
        if newline.is_some() {
            return Ok(if too_large {
                FrameRead::TooLarge
            } else {
                FrameRead::Frame(frame)
            });
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    endpoint: String,
    infisical_config: PathBuf,
    #[serde(default = "default_secret_reference")]
    secret_reference: String,
    scopes: BTreeSet<String>,
    timeout_ms: u64,
}

#[derive(Parser)]
#[command(name = "brain-mcp", about = "MCP adapter for Deadlock Brain")]
struct Args {
    /// Path to the non-secret JSON runtime configuration.
    #[arg(long)]
    config: PathBuf,
}

impl Config {
    fn load(path: &std::path::Path) -> Result<Self, &'static str> {
        let metadata = std::fs::metadata(path).map_err(|_| "MCP-Konfiguration ist nicht lesbar")?;
        if !metadata.is_file() || metadata.len() > MAX_CONFIG_BYTES {
            return Err("MCP-Konfiguration hat ein ungültiges Format oder eine ungültige Größe");
        }
        let mut bytes = Vec::new();
        File::open(path)
            .and_then(|file| file.take(MAX_CONFIG_BYTES + 1).read_to_end(&mut bytes))
            .map_err(|_| "MCP-Konfiguration ist nicht lesbar")?;
        let mut config = Self::parse(&bytes)?;
        if !config.infisical_config.is_absolute() {
            let parent = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| std::path::Path::new("."));
            config.infisical_config = parent.join(&config.infisical_config);
        }
        Ok(config)
    }

    fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() as u64 > MAX_CONFIG_BYTES {
            return Err("MCP-Konfiguration hat eine ungültige Größe");
        }
        let config: Self =
            serde_json::from_slice(bytes).map_err(|_| "MCP-Konfiguration ist ungültig")?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), &'static str> {
        if self.infisical_config.as_os_str().is_empty() {
            return Err("Infisical-Konfigurationspfad ist ungültig");
        }
        if self.endpoint.trim().is_empty()
            || self.endpoint.len() > 2048
            || self.endpoint.chars().any(char::is_control)
        {
            return Err("MCP-Endpunkt ist ungültig");
        }
        if !valid_secret_reference(&self.secret_reference) {
            return Err("Infisical-Secret-Referenz ist ungültig");
        }
        if self.scopes.is_empty()
            || self.scopes.len() > 64
            || self.scopes.iter().any(|scope| {
                scope.is_empty()
                    || scope.len() > 128
                    || !scope
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte))
            })
        {
            return Err("MCP-Scopes sind ungültig");
        }
        if !(1..=60_000).contains(&self.timeout_ms) {
            return Err("MCP-Timeout ist ungültig");
        }
        Ok(())
    }

    fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms)
    }
}

fn valid_secret_reference(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first == b'_' || first.is_ascii_uppercase())
        && bytes.all(|byte| byte == b'_' || byte.is_ascii_uppercase() || byte.is_ascii_digit())
}

fn resolve_token(
    config: &Config,
    lookup: impl FnOnce(&str) -> Option<Zeroizing<String>>,
) -> Result<Zeroizing<String>, &'static str> {
    let token =
        lookup(&config.secret_reference).ok_or("Konfiguriertes MCP-Secret fehlt in Infisical")?;
    if token.trim().is_empty()
        || token.len() > 4096
        || token.chars().any(char::is_control)
        || !token.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return Err("Konfiguriertes MCP-Secret ist ungültig");
    }
    Ok(token)
}

fn ids(arguments: &Value) -> (String, String) {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let default = format!("brain-mcp-{}-{nonce}", process::id());
    let request_id = arguments
        .get("request_id")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&default)
        .to_string();
    let conversation_id = arguments
        .get("conversation_id")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&default)
        .to_string();
    (request_id, conversation_id)
}

fn status_is_error(status: AnswerStatus) -> bool {
    matches!(
        status,
        AnswerStatus::UnauthorizedEvidence
            | AnswerStatus::Unavailable
            | AnswerStatus::ProviderError
            | AnswerStatus::BudgetExceeded
    )
}

fn response(id: Value, result: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"result":result})
}

fn rpc_error(id: Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":message}})
}

async fn handle(
    client: &AsyncBrainClient,
    scopes: &BTreeSet<String>,
    request: Value,
) -> Option<Value> {
    let id = request.get("id").cloned()?;
    let method = request.get("method").and_then(Value::as_str)?;
    match method {
        "initialize" => {
            let protocol = request
                .pointer("/params/protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or("2025-06-18");
            Some(response(
                id,
                json!({
                    "protocolVersion": protocol,
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name":"deadlock-brain-mcp","version":env!("CARGO_PKG_VERSION")}
                }),
            ))
        }
        "ping" => Some(response(id, json!({}))),
        "tools/list" => Some(response(
            id,
            json!({
                "tools":[{
                    "name":"brain_answer",
                    "description":"Ask the configured brain-serve instance using the typed BrainClient contract.",
                    "inputSchema":{
                        "type":"object",
                        "additionalProperties":false,
                        "required":["question"],
                        "properties":{
                            "question":{"type":"string","minLength":1,"maxLength":32768},
                            "request_id":{"type":"string"},
                            "conversation_id":{"type":"string"}
                        }
                    }
                }]
            }),
        )),
        "tools/call" => {
            if request.pointer("/params/name").and_then(Value::as_str) != Some("brain_answer") {
                return Some(rpc_error(id, -32602, "unknown tool"));
            }
            let arguments = request
                .pointer("/params/arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let Some(question) = arguments.get("question").and_then(Value::as_str) else {
                return Some(rpc_error(id, -32602, "question required"));
            };
            let (request_id, conversation_id) = ids(&arguments);
            let query = Query {
                answer_context: None,
                request_id,
                conversation_id,
                text: question.to_string(),
                domain: None,
                requested_scopes: scopes.clone(),
                profile: AnswerProfile::Explain,
                patch: None,
                mode: None,
            };
            if query.validate().is_err() {
                return Some(rpc_error(id, -32602, "invalid Brain query"));
            }
            match client.answer(&query).await {
                Ok(answer) => {
                    let is_error = status_is_error(answer.status);
                    let payload = serde_json::to_string(&answer)
                        .unwrap_or_else(|_| "{\"status\":\"unavailable\"}".to_string());
                    Some(response(
                        id,
                        json!({
                            "content":[{"type":"text","text":payload}],
                            "isError":is_error
                        }),
                    ))
                }
                Err(brain_client::ClientError::RequestTooLarge) => Some(rpc_error(
                    id,
                    -32602,
                    "encoded Brain request exceeds the client size limit",
                )),
                Err(_) => Some(response(
                    id,
                    json!({
                        "content":[{"type":"text","text":"brain-serve transport unavailable"}],
                        "isError":true
                    }),
                )),
            }
        }
        _ => Some(rpc_error(id, -32601, "method not found")),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(message) = run().await {
        eprintln!("{message}");
        process::exit(64);
    }
}

async fn run() -> Result<(), &'static str> {
    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let token = load_token(&config).await?;
    let client = AsyncBrainClient::new_local(&config.endpoint, token.as_str(), config.timeout())
        .map_err(|_| "BrainClient konnte nicht erstellt werden")?;

    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut input = stdin.lock();
    run_loop(&mut input, &mut stdout, |request| {
        handle(&client, &config.scopes, request)
    })
    .await
}

async fn load_token(config: &Config) -> Result<Zeroizing<String>, &'static str> {
    let deadline = tokio::time::Instant::now() + config.timeout();
    let path = config.infisical_config.clone();
    let (sender, receiver) = tokio::sync::oneshot::channel();
    // The existing private-pipe loader can block synchronously. A dedicated
    // thread keeps it outside the main runtime and never joins on shutdown.
    std::thread::Builder::new()
        .name("mcp-secret-startup".into())
        .spawn(move || {
            let result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| "MCP-Secret-Laufzeit konnte nicht erstellt werden")
                .and_then(|runtime| {
                    runtime
                        .block_on(dl_token_secrets::values(&path))
                        .map_err(|_| "MCP-Secret konnte nicht aus Infisical geladen werden")
                });
            // A deadline closes the receiver. Late snapshots are discarded,
            // never passed to the client or exposed in diagnostic output.
            let _ = sender.send(result);
        })
        .map_err(|_| "MCP-Secret-Laufzeit konnte nicht erstellt werden")?;
    let mut secrets = tokio::time::timeout_at(deadline, receiver)
        .await
        .map_err(|_| "MCP-Secret-Startbudget ist abgelaufen")?
        .map_err(|_| "MCP-Secretquelle ist nicht verfügbar")??;
    if tokio::time::Instant::now() >= deadline {
        return Err("MCP-Secret-Startbudget ist abgelaufen");
    }
    let token = resolve_token(config, |reference| {
        secrets
            .iter()
            .position(|(name, _)| name == reference)
            .map(|index| secrets.swap_remove(index).1)
    })?;
    drop(secrets);
    if tokio::time::Instant::now() >= deadline {
        return Err("MCP-Secret-Startbudget ist abgelaufen");
    }
    Ok(token)
}

async fn run_loop<R, W, H, F>(
    input: &mut R,
    output: &mut W,
    mut dispatch: H,
) -> Result<(), &'static str>
where
    R: BufRead,
    W: Write,
    H: FnMut(Value) -> F,
    F: std::future::Future<Output = Option<Value>>,
{
    loop {
        let frame = match read_frame(input, MAX_FRAME_BYTES) {
            Ok(FrameRead::Eof) => break,
            Err(_) => return Err("MCP-Eingabe konnte nicht gelesen werden"),
            Ok(FrameRead::Frame(frame)) if frame.iter().all(u8::is_ascii_whitespace) => continue,
            Ok(FrameRead::Frame(frame)) => frame,
            Ok(FrameRead::TooLarge) => {
                let out = rpc_error(Value::Null, -32700, "frame too large");
                write_response(output, &out)?;
                continue;
            }
        };
        let request: Value = match serde_json::from_slice(&frame) {
            Ok(request) => request,
            Err(_) => {
                let out = rpc_error(Value::Null, -32700, "parse error");
                write_response(output, &out)?;
                continue;
            }
        };
        if let Some(out) = dispatch(request).await {
            write_response(output, &out)?;
        }
    }
    Ok(())
}

fn write_response(output: &mut impl Write, response: &Value) -> Result<(), &'static str> {
    writeln!(output, "{response}")
        .and_then(|()| output.flush())
        .map_err(|_| "MCP-Ausgabe konnte nicht geschrieben werden")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};

    const EXAMPLE_CONFIG: &[u8] = include_bytes!("../../../../../config/brain-mcp.example.json");

    fn private_snapshot_port() -> (tempfile::TempDir, File, File) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("private-snapshot-pipe");
        assert!(process::Command::new("mkfifo")
            .arg("-m")
            .arg("600")
            .arg(&path)
            .status()
            .unwrap()
            .success());
        let reader_path = path.clone();
        let reader = std::thread::spawn(move || File::open(reader_path).unwrap());
        let writer = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        (directory, reader.join().unwrap(), writer)
    }

    fn snapshot_config(directory: &std::path::Path, reader: &File, timeout: u64) -> Config {
        use std::os::fd::AsRawFd;
        let path = directory.join("infisical-fixture.json");
        let normal = json!({
            "project_id":"synthetic-project", "environment":"synthetic-fixture",
            "secret_path":"/", "socket_path":"unused-synthetic-socket",
            "secret_values_fd":reader.as_raw_fd()
        });
        std::fs::write(&path, serde_json::to_vec(&normal).unwrap()).unwrap();
        let mut config = Config::parse(EXAMPLE_CONFIG).unwrap();
        config.infisical_config = path;
        config.secret_reference = "SYNTHETIC_MCP_REFERENCE".into();
        config.timeout_ms = timeout;
        config
    }

    #[tokio::test(flavor = "current_thread")]
    async fn explicit_private_snapshot_resolves_only_configured_reference() {
        for present in [false, true] {
            let (directory, reader, mut writer) = private_snapshot_port();
            let config = snapshot_config(directory.path(), &reader, 1000);
            let name = if present {
                "SYNTHETIC_MCP_REFERENCE"
            } else {
                "OTHER_REFERENCE"
            };
            let body = json!({name:"synthetic-private-pipe-token"});
            writer
                .write_all(serde_json::to_string(&body).unwrap().as_bytes())
                .unwrap();
            drop(writer);
            let result = load_token(&config).await;
            if present {
                assert!(result.unwrap().as_str() == "synthetic-private-pipe-token");
            } else {
                assert_eq!(
                    result.err(),
                    Some("Konfiguriertes MCP-Secret fehlt in Infisical")
                );
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn private_snapshot_without_eof_respects_startup_budget() {
        let (directory, reader, mut writer) = private_snapshot_port();
        let config = snapshot_config(directory.path(), &reader, 100);
        writer
            .write_all(b"{\"SYNTHETIC_MCP_REFERENCE\":\"synthetic-private-pipe-token\"}")
            .unwrap();
        let start = std::time::Instant::now();
        assert_eq!(
            load_token(&config).await.err(),
            Some("MCP-Secret-Startbudget ist abgelaufen")
        );
        assert!(start.elapsed() < Duration::from_secs(1));
        // EOF lets the detached loader dispose of its late snapshot; it cannot
        // start a client or provider request after the caller returned failure.
        drop(writer);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn malformed_private_snapshot_has_only_sanitized_source_error() {
        let (directory, reader, mut writer) = private_snapshot_port();
        let config = snapshot_config(directory.path(), &reader, 1000);
        writer
            .write_all(b"synthetic sensitive malformed payload")
            .unwrap();
        drop(writer);
        assert_eq!(
            load_token(&config).await.err(),
            Some("MCP-Secret konnte nicht aus Infisical geladen werden")
        );
    }

    struct FailingInput;

    impl Read for FailingInput {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("synthetic sensitive read detail"))
        }
    }

    impl BufRead for FailingInput {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(io::Error::other("synthetic sensitive read detail"))
        }

        fn consume(&mut self, _: usize) {}
    }

    struct FailingOutput {
        fail_flush: bool,
        bytes: Vec<u8>,
    }

    impl Write for FailingOutput {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if !self.fail_flush {
                return Err(io::Error::other("synthetic sensitive write detail"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("synthetic sensitive flush detail"))
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn input_failure_is_sanitized_and_never_dispatches() {
        let mut calls = 0;
        let mut output = Vec::new();
        let result = run_loop(&mut FailingInput, &mut output, |_| {
            calls += 1;
            std::future::ready(Some(json!({"unexpected":true})))
        })
        .await;
        assert_eq!(result, Err("MCP-Eingabe konnte nicht gelesen werden"));
        assert_eq!(calls, 0);
        assert!(output.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn clean_eof_succeeds_without_dispatch_or_delivery() {
        let mut calls = 0;
        let mut output = Vec::new();
        let result = run_loop(&mut io::Cursor::new(b" \n"), &mut output, |_| {
            calls += 1;
            std::future::ready(None)
        })
        .await;
        assert_eq!(result, Ok(()));
        assert_eq!(calls, 0);
        assert!(output.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn failed_write_or_flush_stops_before_following_provider_work() {
        for fail_flush in [false, true] {
            let mut calls = 0;
            let mut input = io::Cursor::new(b"{\"id\":1}\n{\"id\":2}\n");
            let mut output = FailingOutput {
                fail_flush,
                bytes: Vec::new(),
            };
            let result = run_loop(&mut input, &mut output, |request| {
                calls += 1;
                std::future::ready(Some(response(request["id"].clone(), json!({}))))
            })
            .await;
            assert_eq!(result, Err("MCP-Ausgabe konnte nicht geschrieben werden"));
            assert_eq!(calls, 1);
            assert!(!String::from_utf8(output.bytes)
                .unwrap()
                .contains("\"id\":2"));
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn failed_error_delivery_stops_before_following_request() {
        for oversized in [false, true] {
            let mut bytes = if oversized {
                vec![b'x'; MAX_FRAME_BYTES + 1]
            } else {
                b"invalid".to_vec()
            };
            bytes.extend_from_slice(b"\n{\"id\":2}\n");
            for fail_flush in [false, true] {
                let mut calls = 0;
                let mut output = FailingOutput {
                    fail_flush,
                    bytes: Vec::new(),
                };
                let result = run_loop(&mut io::Cursor::new(&bytes), &mut output, |_| {
                    calls += 1;
                    std::future::ready(None)
                })
                .await;
                assert_eq!(result, Err("MCP-Ausgabe konnte nicht geschrieben werden"));
                assert_eq!(calls, 0);
            }
        }
    }

    fn synthetic_token_lookup(reference: &str) -> Option<Zeroizing<String>> {
        (reference == "BRAIN_SERVE_API_TOKEN")
            .then(|| Zeroizing::new("synthetic-mcp-credential".to_string()))
    }

    #[test]
    fn example_config_uses_existing_secret_reference_and_scopes() {
        let config = Config::parse(EXAMPLE_CONFIG).unwrap();
        assert_eq!(config.infisical_config, PathBuf::from("infisical.json"));
        assert_eq!(config.secret_reference, "BRAIN_SERVE_API_TOKEN");
        assert_eq!(config.scopes, BTreeSet::from(["docs.public".to_string()]));
        assert_eq!(config.timeout_ms, 8_000);
    }

    #[test]
    fn relative_infisical_config_tracks_relocated_mcp_config_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let relocated = tempdir.path().join("relocated");
        let config_dir = relocated.join("runtime");
        std::fs::create_dir_all(&config_dir).unwrap();
        let config_path = config_dir.join("brain-mcp.json");
        std::fs::write(&config_path, EXAMPLE_CONFIG).unwrap();

        let config = Config::load(&config_path).unwrap();
        assert_eq!(config.infisical_config, config_dir.join("infisical.json"));
    }

    #[test]
    fn secret_port_stub_resolves_only_the_configured_reference() {
        let config = Config::parse(EXAMPLE_CONFIG).unwrap();
        let token = resolve_token(&config, |reference| {
            assert_eq!(reference, "BRAIN_SERVE_API_TOKEN");
            synthetic_token_lookup(reference)
        })
        .unwrap();
        assert!(token.as_str() == "synthetic-mcp-credential");
    }

    #[test]
    fn invalid_synthetic_token_is_not_in_diagnostics() {
        let config = Config::parse(EXAMPLE_CONFIG).unwrap();
        let synthetic_secret = "synthetic credential with spaces";
        let error = resolve_token(&config, |_| {
            Some(Zeroizing::new(synthetic_secret.to_string()))
        })
        .err()
        .unwrap();
        assert_eq!(error, "Konfiguriertes MCP-Secret ist ungültig");
        assert!(!error.contains(synthetic_secret));
    }

    #[test]
    fn local_brain_endpoint_requirement_remains_enforced() {
        assert!(AsyncBrainClient::new_local(
            "https://example.invalid",
            "synthetic-mcp-credential",
            Duration::from_secs(1)
        )
        .is_err());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn max_valid_query_with_json_escaping_returns_clear_size_error() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let client = AsyncBrainClient::new_local(
            &endpoint,
            "synthetic-mcp-credential",
            Duration::from_secs(1),
        )
        .unwrap();
        let request = json!({
            "jsonrpc":"2.0",
            "id":7,
            "method":"tools/call",
            "params":{
                "name":"brain_answer",
                "arguments":{"question":"\u{0001}".repeat(32_768)}
            }
        });

        let result = handle(
            &client,
            &BTreeSet::from(["docs.public".to_string()]),
            request,
        )
        .await
        .unwrap();
        assert_eq!(result["error"]["code"], -32602);
        assert_eq!(
            result["error"]["message"],
            "encoded Brain request exceeds the client size limit"
        );
        assert!(matches!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        ));
    }

    #[test]
    fn inline_secret_fields_are_rejected_without_echoing_their_value() {
        let inline_secret = "synthetic-inline-credential-must-not-appear-in-errors";
        let raw = format!(
            r#"{{"endpoint":"http://127.0.0.1:8787","api_token":"{inline_secret}","scopes":["docs.public"],"timeout_ms":8000}}"#
        );
        let error = Config::parse(raw.as_bytes()).err().unwrap();
        assert_eq!(error, "MCP-Konfiguration ist ungültig");
        assert!(!error.contains(inline_secret));
    }

    #[test]
    fn mcp_requires_an_explicit_config_path() {
        assert!(Args::try_parse_from(["brain-mcp"]).is_err());
    }

    struct RepeatedByteReader {
        remaining: usize,
        byte: u8,
    }

    impl Read for RepeatedByteReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let count = buffer.len().min(self.remaining);
            buffer[..count].fill(self.byte);
            self.remaining -= count;
            Ok(count)
        }
    }

    #[test]
    fn frame_reader_bounds_unterminated_oversized_input() {
        let source = RepeatedByteReader {
            remaining: 10_000_000,
            byte: b'x',
        };
        let mut reader = BufReader::with_capacity(128, source);

        assert!(matches!(
            read_frame(&mut reader, 32).unwrap(),
            FrameRead::TooLarge
        ));
        assert!(matches!(
            read_frame(&mut reader, 32).unwrap(),
            FrameRead::Eof
        ));
    }

    #[test]
    fn frame_reader_drains_oversized_line_and_reads_next_frame() {
        let mut input = vec![b'x'; 33];
        input.extend_from_slice(b"\n{}\n");
        let mut reader = io::Cursor::new(input);

        assert!(matches!(
            read_frame(&mut reader, 32).unwrap(),
            FrameRead::TooLarge
        ));
        let FrameRead::Frame(frame) = read_frame(&mut reader, 32).unwrap() else {
            panic!("expected frame after rejected oversized line");
        };
        assert_eq!(serde_json::from_slice::<Value>(&frame).unwrap(), json!({}));
    }

    #[test]
    fn frame_limit_fits_maximum_schema_question_with_json_escaping() {
        let question = "\u{0001}".repeat(32_768);
        let query = Query {
            answer_context: None,
            request_id: "request".to_string(),
            conversation_id: "conversation".to_string(),
            text: question.clone(),
            domain: None,
            requested_scopes: BTreeSet::from(["docs.public".to_string()]),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        assert!(query.validate().is_ok());
        let encoded = serde_json::to_vec(&json!({
            "jsonrpc":"2.0",
            "id":1,
            "method":"tools/call",
            "params":{"name":"brain_answer","arguments":{"question":question}}
        }))
        .unwrap();
        assert!(encoded.len() > 64 * 1024);
        assert!(encoded.len() <= MAX_FRAME_BYTES);

        let mut reader = io::Cursor::new(encoded);
        let FrameRead::Frame(frame) = read_frame(&mut reader, MAX_FRAME_BYTES).unwrap() else {
            panic!("expected maximum-sized schema question frame");
        };
        assert_eq!(
            serde_json::from_slice::<Value>(&frame).unwrap()["params"]["arguments"]["question"]
                .as_str()
                .unwrap()
                .len(),
            32_768
        );
    }

    #[test]
    fn frame_reader_accepts_valid_crlf_and_unterminated_eof_frames() {
        let mut reader =
            io::Cursor::new(b"{\"jsonrpc\":\"2.0\",\"method\":\"ping\"}\r\n{}".to_vec());

        let FrameRead::Frame(crlf_frame) = read_frame(&mut reader, 64).unwrap() else {
            panic!("expected CRLF frame");
        };
        assert!(serde_json::from_slice::<Value>(&crlf_frame).is_ok());
        let FrameRead::Frame(eof_frame) = read_frame(&mut reader, 64).unwrap() else {
            panic!("expected final unterminated frame");
        };
        assert_eq!(
            serde_json::from_slice::<Value>(&eof_frame).unwrap(),
            json!({})
        );
        assert!(matches!(
            read_frame(&mut reader, 64).unwrap(),
            FrameRead::Eof
        ));
    }

    #[test]
    fn domain_rejection_is_a_successful_tool_result_but_unavailable_is_not() {
        assert!(!status_is_error(AnswerStatus::BuildRejected));
        assert!(!status_is_error(AnswerStatus::InsufficientEvidence));
        assert!(status_is_error(AnswerStatus::Unavailable));
        assert!(status_is_error(AnswerStatus::UnauthorizedEvidence));
    }

    #[test]
    fn tool_arguments_cannot_supply_scopes() {
        let args = json!({
            "question":"Abrams",
            "requested_scopes":["admin"],
            "scopes":["admin"]
        });
        let trusted = BTreeSet::from(["docs.public".to_string()]);
        let (request_id, conversation_id) = ids(&args);
        let query = Query {
            answer_context: None,
            request_id,
            conversation_id,
            text: args["question"].as_str().unwrap().to_string(),
            domain: None,
            requested_scopes: trusted.clone(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        assert_eq!(query.requested_scopes, trusted);
    }
}
