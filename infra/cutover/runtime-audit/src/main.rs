use brain_runtime_audit::{collect, render, AuditResult, MAX_OUTPUT_BYTES};
use std::io::{Read, Write};
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant};

// Operator metadata query only, not an application/LLM timeout or a runtime SLO.
const QUERY_TIMEOUT: Duration = Duration::from_secs(10);

fn read_systemd(args: &[&str]) -> AuditResult<String> {
    // Fixed executable, no shell, no caller-supplied command, no inherited stdin.
    let mut child = Command::new("/usr/bin/systemctl")
        .args(args)
        .env("LC_ALL", "C")
        .env("SYSTEMD_COLORS", "0")
        .env("SYSTEMD_PAGER", "")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "systemctl_unavailable")?;
    let Some(stdout) = child.stdout.take() else {
        let _ = child.kill();
        let _ = child.wait();
        return Err("systemctl_stdout_unavailable");
    };
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take((MAX_OUTPUT_BYTES + 1) as u64)
            .read_to_end(&mut bytes)?;
        Ok::<_, std::io::Error>(bytes)
    });
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < QUERY_TIMEOUT => {
                thread::sleep(Duration::from_millis(20))
            }
            _ => {
                // Only this tool's own systemctl child is terminated, never a service.
                let _ = child.kill();
                let _ = child.wait();
                return Err("systemctl_timeout_or_wait_failure");
            }
        }
    };
    let bytes = reader
        .join()
        .map_err(|_| "systemctl_reader_failed")?
        .map_err(|_| "systemctl_read_failed")?;
    if !status.success() {
        return Err("systemctl_query_failed");
    }
    if bytes.len() > MAX_OUTPUT_BYTES {
        return Err("systemd_output_too_large");
    }
    String::from_utf8(bytes).map_err(|_| "systemctl_non_utf8_output")
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("brain-runtime-audit snapshot\nread-only user-systemd metadata; no deployment or gate approval.\nExit 0: complete collection; 1: collection failed; 2: invalid arguments.");
        return ExitCode::SUCCESS;
    }
    if args.len() != 1 || args[0] != "snapshot" {
        eprintln!("Usage: brain-runtime-audit snapshot | --help");
        return ExitCode::from(2);
    }
    match collect(&mut read_systemd) {
        Ok(units) => match std::io::stdout()
            .lock()
            .write_all(render(&units).as_bytes())
        {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => {
                eprintln!("runtime_inventory_failed: output_unavailable");
                ExitCode::from(1)
            }
        },
        Err(reason) => {
            eprintln!("runtime_inventory_failed: {reason}; cutover_authorized=false");
            ExitCode::from(1)
        }
    }
}
