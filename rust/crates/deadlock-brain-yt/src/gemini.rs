use std::{
    fmt,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use wait_timeout::ChildExt;

use crate::db;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeminiErrorKind {
    NotLoggedIn,
    RateLimited,
    ProfileLocked,
    Refused,
    Timeout,
    Unknown,
}

impl GeminiErrorKind {
    pub fn pauses_run(self) -> bool {
        matches!(
            self,
            Self::NotLoggedIn | Self::RateLimited | Self::ProfileLocked
        )
    }
}

impl fmt::Display for GeminiErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::NotLoggedIn => "not_logged_in",
            Self::RateLimited => "rate_limited",
            Self::ProfileLocked => "profile_locked",
            Self::Refused => "refused",
            Self::Timeout => "timeout",
            Self::Unknown => "unknown",
        };
        formatter.write_str(text)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}: {message}")]
pub struct GeminiError {
    pub kind: GeminiErrorKind,
    pub message: String,
}

#[derive(Debug, Serialize)]
struct WorkerRequest<'a> {
    prompt: &'a str,
}

#[derive(Debug, Deserialize)]
struct WorkerResponse {
    status: String,
    text: Option<String>,
    kind: Option<GeminiErrorKind>,
    message: Option<String>,
}

pub fn run_login() -> anyhow::Result<()> {
    let status = Command::new(python_executable())
        .arg(worker_path())
        .arg("login")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("worker login failed with status {status}");
    }
    Ok(())
}

pub fn analyze_url(url: &str, prompt: &str) -> Result<String, GeminiError> {
    run_analyze(url, prompt).and_then(parse_worker_response)
}

fn run_analyze(url: &str, prompt: &str) -> Result<String, GeminiError> {
    let mut child = Command::new(python_executable())
        .arg(worker_path())
        .arg("analyze")
        .arg("--url")
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| GeminiError {
            kind: GeminiErrorKind::Unknown,
            message: error.to_string(),
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        let payload =
            serde_json::to_vec(&WorkerRequest { prompt }).map_err(|error| GeminiError {
                kind: GeminiErrorKind::Unknown,
                message: error.to_string(),
            })?;
        stdin.write_all(&payload).map_err(|error| GeminiError {
            kind: GeminiErrorKind::Unknown,
            message: error.to_string(),
        })?;
        stdin.write_all(b"\n").map_err(|error| GeminiError {
            kind: GeminiErrorKind::Unknown,
            message: error.to_string(),
        })?;
    }

    let timeout = std::env::var("GEMINI_WORKER_TIMEOUT_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(900);
    match child.wait_timeout(Duration::from_secs(timeout)) {
        Ok(Some(_)) => {
            let output = child.wait_with_output().map_err(|error| GeminiError {
                kind: GeminiErrorKind::Unknown,
                message: error.to_string(),
            })?;
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                return Err(GeminiError {
                    kind: GeminiErrorKind::Unknown,
                    message: stderr,
                });
            }
            Ok(stdout)
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            Err(GeminiError {
                kind: GeminiErrorKind::Timeout,
                message: "worker timeout".to_string(),
            })
        }
        Err(error) => Err(GeminiError {
            kind: GeminiErrorKind::Unknown,
            message: error.to_string(),
        }),
    }
}

fn parse_worker_response(stdout: String) -> Result<String, GeminiError> {
    let line = stdout.lines().next().unwrap_or_default();
    let response: WorkerResponse = serde_json::from_str(line).map_err(|error| GeminiError {
        kind: GeminiErrorKind::Unknown,
        message: format!("invalid worker json: {error}"),
    })?;
    match response.status.as_str() {
        "ok" => Ok(response.text.unwrap_or_default()),
        "error" => Err(GeminiError {
            kind: response.kind.unwrap_or(GeminiErrorKind::Unknown),
            message: response.message.unwrap_or_default(),
        }),
        _ => Err(GeminiError {
            kind: GeminiErrorKind::Unknown,
            message: "invalid worker status".to_string(),
        }),
    }
}

pub fn send_pause_alert(kind: GeminiErrorKind, message: &str) {
    let Some(webhook) = std::env::var("GEMINI_INGEST_ALERT_WEBHOOK")
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        return;
    };
    let _ = reqwest::blocking::Client::new()
        .post(webhook)
        .json(&serde_json::json!({
            "content": format!("⚠️ Deadlock-Brain YouTube-Ingestion pausiert [{kind}]: {message}"),
            "kind": kind.to_string(),
            "message": message,
        }))
        .send();
}

fn python_executable() -> String {
    std::env::var("GEMINI_PYTHON").unwrap_or_else(|_| "python3".to_string())
}

fn worker_path() -> PathBuf {
    std::env::var_os("GEMINI_WORKER_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| db::repo_root().join("rust/python_worker/gemini_browser.py"))
}
