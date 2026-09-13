//! One real retrieval call in a fresh read-only process; no chat, AI or publish.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    io,
    path::Path,
    process::Stdio,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::Command,
    task::JoinHandle,
};

#[path = "../../dbrain-reasoner/examples/support/mod.rs"]
mod support;

type Error = Box<dyn std::error::Error>;
const DEADLINE: Duration = Duration::from_secs(20);
const PIPE_LIMIT: usize = 16 * 1024 * 1024;

struct Observation {
    timed_out: bool,
    exit_code: Option<i32>,
    wait_ms: u128,
    stdout: Vec<u8>,
    stderr_bytes: usize,
    pipe_error: bool,
}

fn read_pipe(pipe: impl AsyncRead + Unpin + Send + 'static) -> JoinHandle<io::Result<Vec<u8>>> {
    tokio::spawn(async move {
        let mut bytes = Vec::new();
        pipe.take((PIPE_LIMIT + 1) as u64)
            .read_to_end(&mut bytes)
            .await?;
        if bytes.len() > PIPE_LIMIT {
            return Err(io::Error::other("Prozessausgabe überschreitet Messgrenze"));
        }
        Ok(bytes)
    })
}

async fn collect(mut task: JoinHandle<io::Result<Vec<u8>>>) -> io::Result<Vec<u8>> {
    match tokio::time::timeout(Duration::from_secs(1), &mut task).await {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => Err(io::Error::other("Ausgabeleser fehlgeschlagen")),
        Err(_) => {
            task.abort();
            let _ = task.await;
            Err(io::Error::other("Ausgabeleser wurde nicht abgeschlossen"))
        }
    }
}

async fn run_child(binary: &Path, args: &[&str], deadline: Duration) -> io::Result<Observation> {
    let started = Instant::now();
    let mut child = Command::new(binary)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let stdout = read_pipe(child.stdout.take().expect("piped stdout"));
    let stderr = read_pipe(child.stderr.take().expect("piped stderr"));
    // Like the bot, the 20-second timeout wraps waiting for the process. The
    // outer elapsed measurement also includes spawn, pipe collection and JSON.
    let waited = tokio::time::timeout(deadline, child.wait()).await;
    let wait_ms = started.elapsed().as_millis();
    let timed_out = waited.is_err();
    let status = match waited {
        Ok(Ok(status)) => Some(status),
        _ => {
            let _ = child.start_kill();
            // Always reap our worker. It starts no descendants. Preserve the
            // timeout/failure status even when killing/reaping succeeds.
            let _ = child.wait().await;
            None
        }
    };
    let (stdout, stderr) = tokio::join!(collect(stdout), collect(stderr));
    let pipe_error = stdout.is_err() || stderr.is_err();
    Ok(Observation {
        timed_out,
        exit_code: status.and_then(|status| status.code()),
        wait_ms,
        stdout: stdout.unwrap_or_default(),
        stderr_bytes: stderr.map_or(0, |bytes| bytes.len()),
        pipe_error,
    })
}

async fn worker(hero: &str) -> Result<(), Error> {
    // The parent never reads the inherited Infisical descriptor. The existing
    // loader checks it and sets CLOEXEC before obtaining the database secret.
    let pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&pool)
        .await?;
    if read_only != "on" {
        return Err("Read-only-Zugang nicht bestätigt".into());
    }
    let result = dbrain_retrieval::ask_context(
        &pool,
        &format!("Build für {hero}"),
        &dbrain_retrieval::AskContextOptions {
            limit_events: 80,
            include_unverified: false,
            max_claims: 12,
            game_wiki_dir: None,
        },
    )
    .await?;
    serde_json::to_writer(std::io::stdout().lock(), &result)?;
    Ok(())
}

fn valid_build(ask: &Value) -> bool {
    ask["retrieval_meta"]["route"] == "build_reasoner"
        && ask["build_context_schema"] == "reasoner_build_v1"
        && ask["result_text"]
            .as_str()
            .is_some_and(|text| !text.trim().is_empty())
        && ask["prompt"]
            .as_str()
            .is_some_and(|text| !text.trim().is_empty())
        && ask["build_context"]["core"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
}

async fn measure(output: &Path, hero: &str) -> Result<(), Error> {
    if output.exists() {
        return Err("Ausgabe existiert bereits".into());
    }
    let binary = std::env::current_exe()?;
    let binary_hash = format!("{:x}", Sha256::digest(std::fs::read(&binary)?));
    let started_at = chrono::Utc::now();
    let started = Instant::now();
    let observation = run_child(&binary, &["--read-only-worker", hero], DEADLINE).await?;
    let ask: Option<Value> = serde_json::from_slice(&observation.stdout).ok();
    let valid = ask.as_ref().is_some_and(valid_build);
    let elapsed_ms = started.elapsed().as_millis();
    let passed = !observation.timed_out
        && observation.exit_code == Some(0)
        && !observation.pipe_error
        && valid
        && elapsed_ms < DEADLINE.as_millis();
    let evidence = json!({
        "contract":"read_only_process_equivalent_not_production_cli",
        "binary_sha256":binary_hash,"hero":hero,"started_at":started_at.to_rfc3339(),
        "deadline_ms":DEADLINE.as_millis(),"elapsed_ms":elapsed_ms,
        "process_wait_ms":observation.wait_ms,"timed_out":observation.timed_out,
        "exit_code":observation.exit_code,"pipe_error":observation.pipe_error,
        "stdout_bytes":observation.stdout.len(),"stderr_bytes":observation.stderr_bytes,
        "valid_build_response":valid,"passed":passed,
        "read_only_required_by_worker":true,"ask_context_call_limit":1,
        "completed_ask_context_calls":if valid {Some(1)} else {None},
        "options":{"limit_events":80,"include_unverified":false,"max_claims":12,"game_wiki_dir":null},
        "limits":"Includes child process start, Infisical, pool, read-only check, DB, Reasoner, JSON and collection. Does not execute the production CLI or its settings loader. Evidence hashing/writing is outside the measured call. Pipes bounded to 16 MiB; reader cleanup bounded to 1 second, unlike the legacy caller.",
        "chat_sent":false,"ai_called":false,"upload_performed":false,
        "ask":ask,
    });
    support::write_new(output, &serde_json::to_vec_pretty(&evidence)?)?;
    if !passed {
        return Err("Einzel-Ask-Prozessvertrag nicht bestanden; siehe Messdatei".into());
    }
    println!("Einzel-Ask für {hero}: {elapsed_ms} ms, lesender Prozessvertrag bestanden.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [mode, hero] if mode == "--read-only-worker" => {
            if worker(hero).await.is_err() {
                // Never forward errors from the database/credential path.
                std::process::exit(1);
            }
            Ok(())
        }
        [output, hero] => measure(Path::new(output), hero).await,
        _ => Err("Aufruf: ask_latency AUSGABE HELD (vorhandener Infisical-FD erforderlich)".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn oversized_and_unclosed_pipes_are_bounded() {
        assert!(collect(read_pipe(tokio::io::repeat(0))).await.is_err());
        let (reader, _writer_kept_open) = tokio::io::duplex(8);
        assert!(collect(read_pipe(reader)).await.is_err());
    }

    #[tokio::test]
    async fn process_success_failure_and_timeout_are_distinct() {
        let ok = run_child(Path::new("/bin/echo"), &["{}"], Duration::from_secs(2))
            .await
            .unwrap();
        assert_eq!(ok.exit_code, Some(0));
        assert!(!ok.timed_out && !ok.pipe_error);
        assert!(serde_json::from_slice::<Value>(&ok.stdout).is_ok());
        assert!(!valid_build(&json!({})));
        let failed = run_child(Path::new("/bin/false"), &[], Duration::from_secs(2))
            .await
            .unwrap();
        assert_eq!(failed.exit_code, Some(1));
        assert!(!failed.timed_out);
        let timed = run_child(Path::new("/bin/sleep"), &["2"], Duration::from_millis(20))
            .await
            .unwrap();
        assert!(timed.timed_out);
        assert_eq!(timed.exit_code, None);
        assert!(!timed.pipe_error);
    }
}
