#![allow(dead_code)]
//! Test-only process harness. Synthetic snapshots cross a private anonymous pipe, never files.
use serde_json::{json, Value};
#[path = "../../../../test-support/bot_toml.rs"]
mod fixture_toml;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
    process::{Child, ChildStdin, Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

pub const API_TOKEN: &str = "synthetic-c1-client-token";
pub const PROVIDER_TOKEN: &str = "synthetic-c1-provider-token";
pub const OTHER_TOKEN: &str = "synthetic-c1-other-client-token";
pub const PG_PASSWORD: &str = "synthetic-c1-database-password";

pub fn config() -> Value {
    let root = brain_serve::bot_toml::document(include_bytes!(
        "../../../../../config/brain-serve.example.toml"
    ))
    .unwrap();
    let mut value = serde_json::to_value(&root["brain"]["serve"]).unwrap();
    value["bind"] = json!("127.0.0.1:0");
    value
}

pub fn credentials() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BRAIN_SERVE_API_TOKEN", API_TOKEN),
        ("BRAIN_SERVE_PROVIDER_API_KEY", PROVIDER_TOKEN),
    ]
}

pub struct Service {
    child: Child,
    log: PathBuf,
    _directory: tempfile::TempDir,
    _snapshot_pipe: Option<ChildStdin>,
}

impl Service {
    pub fn spawn(config: &Value, environment: &[(&str, &str)]) -> Self {
        Self::spawn_with_snapshot_delay(config, environment, Duration::ZERO, false)
    }

    pub fn spawn_with_snapshot_delay(
        config: &Value,
        environment: &[(&str, &str)],
        delay: Duration,
        keep_open: bool,
    ) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let config_path = directory.path().join("bot.toml");
        let infisical = json!({
            "project_id":"synthetic-project", "environment":"synthetic", "secret_path":"/",
            "socket_path":directory.path().join("unused-infisical.sock"), "secret_values_fd":3
        });
        let mut config_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&config_path)
            .unwrap();
        let mut serve = config.clone();
        let operator = serve.as_object_mut().unwrap().remove("internal_operator");
        let mut root = json!({"brain":{"serve":serve,"infisical":infisical}});
        if let Some(operator) = operator {
            root["brain"]["operator"] = operator;
        }
        config_file
            .write_all(&fixture_toml::section(&root, &[]))
            .unwrap();
        let log = directory.path().join("service.log");
        let output = File::create(&log).unwrap();
        // Test-only shell duplicates the anonymous stdin pipe into the explicit
        // FD without unsafe pre_exec or process-global descriptor mutations.
        let mut child = Command::new("/bin/bash")
            .args(["-c", "exec 3<&0; exec \"$@\"", "brain-serve-fixture"])
            .arg(env!("CARGO_BIN_EXE_brain-serve"))
            .arg("--config")
            .arg(&config_path)
            .env_clear()
            // Hostile ambient values deliberately differ from the snapshot and
            // cannot supply missing credentials or override explicit config.
            .env("BRAIN_SERVE_CONFIG", "/ignored/ambient-config.json")
            .env("BRAIN_SERVE_PROVIDER_API_KEY", "synthetic-ambient-provider")
            .env("BRAIN_SERVE_API_TOKEN", "synthetic-ambient-api")
            .env("BRAIN_SERVE_PG_PASSWORD", "synthetic-ambient-password")
            .envs(environment.iter().copied().filter(|(name, _)| {
                matches!(
                    *name,
                    "DATABASE_URL" | "PGHOST" | "PGPASSWORD" | "BRAIN_LEGACY_URL"
                )
            }))
            .stdin(Stdio::piped())
            .stdout(Stdio::from(output.try_clone().unwrap()))
            .stderr(Stdio::from(output))
            .spawn()
            .unwrap();
        let snapshot: std::collections::BTreeMap<_, _> = environment.iter().copied().collect();
        let raw = serde_json::to_vec(&snapshot).unwrap();
        assert!(
            raw.len() <= 4096,
            "synthetic snapshot must fit the private pipe atomically"
        );
        let mut pipe = child.stdin.take().unwrap();
        if let Err(error) = pipe.write_all(&raw) {
            // Config rejection may close stdin before it reads any snapshot.
            assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
        }
        thread::sleep(delay);
        let snapshot_pipe = keep_open.then_some(pipe);
        Self {
            child,
            log,
            _directory: directory,
            _snapshot_pipe: snapshot_pipe,
        }
    }

    pub fn log(&self) -> String {
        std::fs::read_to_string(&self.log).unwrap_or_default()
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn address(&mut self) -> String {
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            let log = self.log();
            for line in log.lines() {
                if let Ok(value) = serde_json::from_str::<Value>(line) {
                    if value["event"] == "listening" {
                        let address = value["address"].as_str().unwrap();
                        let parsed: std::net::SocketAddr = address.parse().unwrap();
                        assert!(parsed.ip().is_loopback());
                        assert_ne!(parsed.port(), 0);
                        return format!("http://{address}");
                    }
                }
            }
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "service exited before bind: {log}"
            );
            assert!(Instant::now() < deadline, "service did not bind: {log}");
            thread::sleep(Duration::from_millis(20));
        }
    }

    pub fn signal(&mut self, signal: &str) {
        assert!(
            self.child.try_wait().unwrap().is_none(),
            "service already exited: {}",
            self.log()
        );
        assert!(Command::new("kill")
            .arg(format!("-{signal}"))
            .arg(self.child.id().to_string())
            .status()
            .unwrap()
            .success());
    }

    pub fn wait(&mut self, timeout: Duration) -> ExitStatus {
        self.child
            .wait_timeout(timeout)
            .unwrap()
            .expect("service exceeded process deadline")
    }

    pub fn stop(&mut self) {
        self.signal("TERM");
        let status = self.wait(Duration::from_secs(15));
        assert!(status.success(), "ungraceful service exit: {}", self.log());
        assert!(self.log().contains("shutdown_started"));
        assert!(self.log().contains("stopped"));
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

pub fn request(
    address: &str,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (u16, String) {
    let url = format!("{address}{path}");
    let method = method.parse::<reqwest::Method>().unwrap();
    let token = token.map(str::to_owned);
    // Blocking reqwest must be created/dropped off the test's Tokio executor.
    thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(4))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let mut request = client.request(method, url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().unwrap();
        let code = response.status().as_u16();
        (code, response.text().unwrap())
    })
    .join()
    .unwrap()
}

pub fn get(address: &str, path: &str) -> (u16, String) {
    request(address, "GET", path, None, None)
}

pub fn assert_ready(address: &str) {
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        if get(address, "/readyz").0 == 200 {
            break;
        }
        assert!(Instant::now() < deadline, "readiness did not recover");
        thread::sleep(Duration::from_millis(25));
    }
}
