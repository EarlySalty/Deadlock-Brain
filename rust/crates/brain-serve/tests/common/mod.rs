#![allow(dead_code)]
//! Test-only process harness. Each child has a cleared environment and only synthetic credentials.
use serde_json::{json, Value};
use std::{
    fs::File,
    path::PathBuf,
    process::{Child, Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

pub const API_TOKEN: &str = "synthetic-c1-client-token";
pub const PROVIDER_TOKEN: &str = "synthetic-c1-provider-token";
pub const OTHER_TOKEN: &str = "synthetic-c1-other-client-token";
pub const PG_PASSWORD: &str = "synthetic-c1-database-password";

pub fn config() -> Value {
    let mut value: Value = serde_json::from_slice(include_bytes!(
        "../../../../../config/brain-serve.example.json"
    ))
    .unwrap();
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
}

impl Service {
    pub fn spawn(config: &Value, environment: &[(&str, &str)]) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let config_path = directory.path().join("service.json");
        std::fs::write(&config_path, serde_json::to_vec_pretty(config).unwrap()).unwrap();
        let log = directory.path().join("service.log");
        let output = File::create(&log).unwrap();
        let child = Command::new(env!("CARGO_BIN_EXE_brain-serve"))
            .env_clear()
            .env("BRAIN_SERVE_CONFIG", &config_path)
            .envs(environment.iter().copied())
            .stdin(Stdio::null())
            .stdout(Stdio::from(output.try_clone().unwrap()))
            .stderr(Stdio::from(output))
            .spawn()
            .unwrap();
        Self {
            child,
            log,
            _directory: directory,
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
