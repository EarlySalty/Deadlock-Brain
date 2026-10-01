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
        Self::parse(&bytes)
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
    let infisical_config = deadlock_brain_core::config::repo_root().join("config/infisical.json");
    let mut secrets = deadlock_brain_core::pg::infisical_environment(&infisical_config)
        .await
        .map_err(|_| "MCP-Secret konnte nicht aus Infisical geladen werden")?;
    let token = resolve_token(&config, |reference| {
        secrets
            .iter()
            .position(|(name, _)| name == reference)
            .map(|index| secrets.swap_remove(index).1)
    })?;
    drop(secrets);
    let client = AsyncBrainClient::new_local(&config.endpoint, token.as_str(), config.timeout())
        .map_err(|_| "BrainClient konnte nicht erstellt werden")?;

    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut input = stdin.lock();
    loop {
        let frame = match read_frame(&mut input, MAX_FRAME_BYTES) {
            Ok(FrameRead::Eof) | Err(_) => break,
            Ok(FrameRead::Frame(frame)) if frame.iter().all(u8::is_ascii_whitespace) => continue,
            Ok(FrameRead::Frame(frame)) => frame,
            Ok(FrameRead::TooLarge) => {
                let out = rpc_error(Value::Null, -32700, "frame too large");
                let _ = writeln!(stdout, "{out}");
                let _ = stdout.flush();
                continue;
            }
        };
        let request: Value = match serde_json::from_slice(&frame) {
            Ok(request) => request,
            Err(_) => {
                let out = rpc_error(Value::Null, -32700, "parse error");
                let _ = writeln!(stdout, "{out}");
                let _ = stdout.flush();
                continue;
            }
        };
        if let Some(out) = handle(&client, &config.scopes, request).await {
            let _ = writeln!(stdout, "{out}");
            let _ = stdout.flush();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};

    const EXAMPLE_CONFIG: &[u8] = include_bytes!("../../../../../config/brain-mcp.example.json");

    fn synthetic_token_lookup(reference: &str) -> Option<Zeroizing<String>> {
        (reference == "BRAIN_SERVE_API_TOKEN")
            .then(|| Zeroizing::new("synthetic-mcp-credential".to_string()))
    }

    #[test]
    fn example_config_uses_existing_secret_reference_and_scopes() {
        let config = Config::parse(EXAMPLE_CONFIG).unwrap();
        assert_eq!(config.secret_reference, "BRAIN_SERVE_API_TOKEN");
        assert_eq!(config.scopes, BTreeSet::from(["docs.public".to_string()]));
        assert_eq!(config.timeout_ms, 8_000);
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
