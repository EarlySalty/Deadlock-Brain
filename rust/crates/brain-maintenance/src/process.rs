use anyhow::{bail, ensure, Context, Result};
use nix::{
    sys::signal::{killpg, Signal},
    unistd::Pid,
};
use std::os::unix::process::CommandExt;
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
};

async fn bounded_read(reader: impl AsyncRead + Unpin, max: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take((max + 1) as u64)
        .read_to_end(&mut bytes)
        .await?;
    ensure!(
        bytes.len() <= max,
        "Prozessausgabe überschreitet die Grenze"
    );
    Ok(bytes)
}

struct ProcessGroup(Pid);
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        match killpg(self.0, Signal::SIGKILL) {
            Ok(()) | Err(nix::errno::Errno::ESRCH) => {}
            Err(_) => eprintln!("Prozessgruppe konnte beim Abbruch nicht beendet werden"),
        }
    }
}

pub async fn run(
    executable: &Path,
    args: &[String],
    cwd: &Path,
    input: &[u8],
    timeout_ms: u64,
    max_output: usize,
) -> Result<Vec<u8>> {
    let result = run_observed(executable, args, cwd, input, timeout_ms, max_output).await?;
    ensure!(
        result.exit_code == Some(0),
        "Prozess fehlgeschlagen, Exitcode {:?}",
        result.exit_code
    );
    Ok(result.output)
}

pub struct ProcessResult {
    pub output: Vec<u8>,
    pub exit_code: Option<i32>,
}

enum ProcessInput<'a> {
    Bytes(&'a [u8]),
    File(std::fs::File),
}

pub async fn run_with_credential(
    executable: &Path,
    args: &[String],
    cwd: &Path,
    credential: std::fs::File,
    timeout_ms: u64,
    max_output: usize,
) -> Result<Vec<u8>> {
    ensure!(credential.metadata()?.is_file(), "credential_regular_file");
    let result = run_observed_input(
        executable,
        args,
        cwd,
        ProcessInput::File(credential),
        timeout_ms,
        max_output,
    )
    .await?;
    ensure!(
        result.exit_code == Some(0),
        "Prozess fehlgeschlagen, Exitcode {:?}",
        result.exit_code
    );
    Ok(result.output)
}

pub async fn run_observed(
    executable: &Path,
    args: &[String],
    cwd: &Path,
    input: &[u8],
    timeout_ms: u64,
    max_output: usize,
) -> Result<ProcessResult> {
    run_observed_input(
        executable,
        args,
        cwd,
        ProcessInput::Bytes(input),
        timeout_ms,
        max_output,
    )
    .await
}

async fn run_observed_input(
    executable: &Path,
    args: &[String],
    cwd: &Path,
    input: ProcessInput<'_>,
    timeout_ms: u64,
    max_output: usize,
) -> Result<ProcessResult> {
    let (stdin, input_bytes) = match input {
        ProcessInput::Bytes(bytes) => (Stdio::piped(), Some(bytes)),
        ProcessInput::File(file) => (Stdio::from(file), None),
    };
    let mut command = Command::new(executable);
    command.as_std_mut().process_group(0);
    let mut child = command
        .args(args)
        .current_dir(cwd)
        .stdin(stdin)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("Prozess konnte nicht gestartet werden")?;
    let _group = ProcessGroup(Pid::from_raw(i32::try_from(
        child.id().context("Prozess-ID fehlt")?,
    )?));
    let stdin = child.stdin.take();
    let stdout = child.stdout.take().context("Prozess-Ausgabe fehlt")?;
    let stderr = child.stderr.take().context("Prozess-Fehlerkanal fehlt")?;
    let work = async {
        let write = async move {
            if let Some(input) = input_bytes {
                let mut stdin = stdin.context("Prozess-Eingabe fehlt")?;
                stdin.write_all(input).await?;
                stdin.shutdown().await?;
                drop(stdin);
            }
            anyhow::Ok(())
        };
        let ((), output, _diagnostics, status) = tokio::try_join!(
            write,
            bounded_read(stdout, max_output),
            bounded_read(stderr, max_output),
            async { Ok::<_, anyhow::Error>(child.wait().await?) }
        )?;
        // Fremde Prozessausgabe wird nicht in Fehler oder Logs übernommen.
        Ok(ProcessResult {
            output,
            exit_code: status.code(),
        })
    };
    match tokio::time::timeout(Duration::from_millis(timeout_ms), work).await {
        Ok(result) => result,
        Err(_) => {
            match killpg(_group.0, Signal::SIGKILL) {
                Ok(()) | Err(nix::errno::Errno::ESRCH) => {}
                Err(_) => {
                    bail!("Prozesszeit überschritten; Prozessgruppe konnte nicht beendet werden")
                }
            }
            if let Err(error) = child.kill().await {
                ensure!(
                    matches!(
                        error.kind(),
                        std::io::ErrorKind::InvalidInput | std::io::ErrorKind::NotFound
                    ),
                    "Prozesszeit überschritten; Prozess konnte nicht beendet werden"
                );
            }
            bail!("Prozesszeit überschritten")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn stdin_is_closed_before_waiting_for_the_child() {
        let dir = tempfile::tempdir().unwrap();
        let output = run(
            Path::new("/bin/cat"),
            &[],
            dir.path(),
            b"harmloser Test",
            1000,
            100,
        )
        .await
        .unwrap();
        assert_eq!(output, b"harmloser Test");
    }
    #[tokio::test]
    async fn oversized_output_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(
            run(Path::new("/bin/cat"), &[], dir.path(), b"zu lang", 1000, 2)
                .await
                .is_err()
        );
    }
    #[tokio::test]
    async fn timeout_stops_wrapper_and_child_process() {
        let dir = tempfile::tempdir().unwrap();
        let pidfile = dir.path().join("child.pid");
        let args = vec![
            "-c".into(),
            "sleep 60 & printf '%s' \"$!\" > \"$1\"; wait".into(),
            "fixture".into(),
            pidfile.to_string_lossy().into_owned(),
        ];
        assert!(run(Path::new("/bin/sh"), &args, dir.path(), &[], 100, 100)
            .await
            .is_err());
        let pid: u32 = std::fs::read_to_string(pidfile).unwrap().parse().unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        if let Ok(stat) = std::fs::read_to_string(format!("/proc/{pid}/stat")) {
            assert!(
                stat.contains(") Z "),
                "Kindprozess läuft nach Timeout weiter"
            );
        }
    }
    #[tokio::test]
    async fn dropping_the_future_stops_wrapper_and_child_process() {
        let dir = tempfile::tempdir().unwrap();
        let cwd = dir.path().to_owned();
        let pidfile = cwd.join("aborted.pid");
        let args = vec![
            "-c".into(),
            "sleep 60 & printf '%s' \"$!\" > \"$1\"; wait".into(),
            "fixture".into(),
            pidfile.to_string_lossy().into_owned(),
        ];
        let task =
            tokio::spawn(
                async move { run(Path::new("/bin/sh"), &args, &cwd, &[], 10_000, 100).await },
            );
        for _ in 0..100 {
            if std::fs::read_to_string(&pidfile).is_ok_and(|value| value.parse::<u32>().is_ok()) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let pid: u32 = std::fs::read_to_string(&pidfile).unwrap().parse().unwrap();
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        tokio::time::sleep(Duration::from_millis(50)).await;
        if let Ok(stat) = std::fs::read_to_string(format!("/proc/{pid}/stat")) {
            assert!(
                stat.contains(") Z "),
                "Kindprozess läuft nach Abbruch weiter"
            );
        }
    }
}
