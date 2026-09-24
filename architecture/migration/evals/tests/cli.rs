use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "s10-eval-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn call(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dbrain-s10-evals"))
        .args(args)
        .output()
        .unwrap()
}
fn input(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(name)
        .to_str()
        .unwrap()
        .into()
}
fn load(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
#[test]
fn help_is_available() {
    assert!(call(&["--help"]).status.success());
}
#[test]
fn unknown_command_fails() {
    assert_eq!(call(&["invented"]).status.code(), Some(64));
}
#[test]
fn design_report_does_not_claim_execution_or_release() {
    let t = Temp::new();
    let out = t.0.join("design.json");
    let r = call(&[
        "design",
        &input("profile.json"),
        &input("cases.json"),
        out.to_str().unwrap(),
    ]);
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let v = load(&out);
    assert_eq!(v["case_count"], 40);
    assert_eq!(v["release_approved"], false);
    assert_eq!(v["e2e_executed"], false);
}
#[test]
fn existing_report_is_never_overwritten() {
    let t = Temp::new();
    let out = t.0.join("existing.json");
    fs::write(&out, b"preserve").unwrap();
    assert_eq!(
        call(&[
            "design",
            &input("profile.json"),
            &input("cases.json"),
            out.to_str().unwrap()
        ])
        .status
        .code(),
        Some(64)
    );
    assert_eq!(fs::read(out).unwrap(), b"preserve");
}
#[test]
fn oversized_input_is_rejected() {
    let t = Temp::new();
    let source = t.0.join("large.json");
    let out = t.0.join("out.json");
    fs::write(&source, vec![b' '; 4 * 1024 * 1024 + 1]).unwrap();
    assert_eq!(
        call(&[
            "design",
            source.to_str().unwrap(),
            &input("cases.json"),
            out.to_str().unwrap()
        ])
        .status
        .code(),
        Some(64)
    );
    assert!(!out.exists());
}
#[test]
fn malformed_source_text_is_not_echoed() {
    let t = Temp::new();
    let source = t.0.join("bad.json");
    let out = t.0.join("out.json");
    fs::write(&source, b"synthetic-private-canary-should-not-appear").unwrap();
    let r = call(&[
        "design",
        source.to_str().unwrap(),
        &input("cases.json"),
        out.to_str().unwrap(),
    ]);
    assert_eq!(r.status.code(), Some(64));
    assert!(!String::from_utf8_lossy(&r.stderr).contains("synthetic-private-canary"));
    assert!(r.stdout.is_empty());
}
#[test]
fn missing_run_evidence_returns_nonzero_and_a_blocked_report() {
    let t = Temp::new();
    let profile = load(Path::new(&input("profile.json")));
    let raw = t.0.join("run.json");
    let out = t.0.join("assessment.json");
    let r = json!({"schema_version":1,"kind":"mock","variant":"B","versions":profile["versions"],"hardware_fingerprint":"fixture","profile_sha256":"f".repeat(64),"dataset_sha256":profile["dataset_sha256"],"evaluator":"answer_model","label_revision":null,"workload":profile["workload"],"measurement_started_us":0,"measurement_finished_us":1,"offered_requests":0,"raw_artifact_sha256":null,"resources":{"peak_ram_mib":null,"cpu_seconds":null,"io_read_bytes":null,"io_write_bytes":null,"ingest_lag_seconds":null,"rebuild_seconds":null,"max_queue_depth":null,"recovered_after_overload":null},"samples":[]});
    fs::write(&raw, serde_json::to_vec(&r).unwrap()).unwrap();
    assert_eq!(
        call(&[
            "assess",
            &input("profile.json"),
            &input("cases.json"),
            raw.to_str().unwrap(),
            &"a".repeat(40),
            out.to_str().unwrap()
        ])
        .status
        .code(),
        Some(2)
    );
    let v = load(&out);
    assert_ne!(v["status"], "pass");
    assert_eq!(v["release_approved"], false);
    assert!(v["blockers"].as_array().unwrap().len() > 5);
}
#[cfg(unix)]
#[test]
fn reports_are_private_by_default() {
    use std::os::unix::fs::PermissionsExt;
    let t = Temp::new();
    let out = t.0.join("design.json");
    assert!(call(&[
        "design",
        &input("profile.json"),
        &input("cases.json"),
        out.to_str().unwrap()
    ])
    .status
    .success());
    assert_eq!(
        fs::metadata(out).unwrap().permissions().mode() & 0o777,
        0o600
    );
}
#[test]
fn baseline_is_labelled_as_stage_not_service_latency() {
    let t = Temp::new();
    let out = t.0.join("baseline.json");
    let result = call(&["baseline", out.to_str().unwrap()]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let v = load(&out);
    assert_eq!(v["kind"], "stage_fixture");
    assert_eq!(v["raw_samples"].as_array().unwrap().len(), 5);
    assert_eq!(v["quality_reference_passed"], true);
    assert!(v["e2e_latency_ms"].is_null());
    assert!(v["speedup"].is_null());
    assert_eq!(v["release_approved"], false);
}
