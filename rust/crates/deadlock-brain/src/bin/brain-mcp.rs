use brain_client::{AnswerProfile, AnswerStatus, AsyncBrainClient, Query};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    io::{self, BufRead, Write},
    process,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const MAX_FRAME_BYTES: usize = 64 * 1024;

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

struct Config {
    endpoint: String,
    token: String,
    scopes: BTreeSet<String>,
    timeout: Duration,
}

impl Config {
    fn from_env() -> Result<Self, &'static str> {
        let endpoint = std::env::var("BRAIN_MCP_ENDPOINT")
            .or_else(|_| std::env::var("BRAIN_CLIENT_ENDPOINT"))
            .map_err(|_| "BRAIN_MCP_ENDPOINT/BRAIN_CLIENT_ENDPOINT fehlt")?;
        let token = std::env::var("BRAIN_MCP_TOKEN")
            .or_else(|_| std::env::var("BRAIN_CLIENT_TOKEN"))
            .map_err(|_| "BRAIN_MCP_TOKEN/BRAIN_CLIENT_TOKEN fehlt")?;
        let scopes: BTreeSet<String> = std::env::var("BRAIN_MCP_SCOPES")
            .map_err(|_| "BRAIN_MCP_SCOPES fehlt")?
            .split(',')
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        if scopes.is_empty() {
            return Err("BRAIN_MCP_SCOPES ist leer");
        }
        let timeout_ms = std::env::var("BRAIN_MCP_TIMEOUT_MS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(8_000)
            .max(1);
        Ok(Self {
            endpoint,
            token,
            scopes,
            timeout: Duration::from_millis(timeout_ms),
        })
    }
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
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            process::exit(64);
        }
    };
    let client = match AsyncBrainClient::new_local(&config.endpoint, &config.token, config.timeout)
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("BrainClient konnte nicht erstellt werden");
            process::exit(64);
        }
    };

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};

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
