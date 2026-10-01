use s14_replay_audit::{CAPABILITY_HEADER, CORPUS_HEADER, MAX_MANIFEST_BYTES};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "s14-audit-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, content: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
    fn run(&self, corpus: &[u8], caps: &[u8]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
            .arg(self.file("corpus.tsv", corpus))
            .arg(self.file("caps.tsv", caps))
            .output()
            .unwrap()
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn empty_manifest_exits_two_without_false_authorization() {
    let t = Temp::new();
    let out = t.run(CORPUS_HEADER.as_bytes(), CAPABILITY_HEADER.as_bytes());
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stderr.is_empty());
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("\"status\":\"blocked\""));
    assert!(text.contains("\"implementation_authorized\":false"));
    assert!(text.contains("\"integration_verified\":false"));
    assert!(text.contains("\"real_matches\":0"));
}
#[test]
fn missing_file_has_redacted_error() {
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .args(["/missing/sentinel-private-match-name", "/missing/other"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "s14_replay_audit:manifest_unreadable\n"
    );
}
#[test]
fn invalid_utf8_is_not_lossily_accepted() {
    let out = Temp::new().run(&[0xff], CAPABILITY_HEADER.as_bytes());
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8(out.stderr)
        .unwrap()
        .contains("manifest_not_utf8"));
}
#[test]
fn oversized_manifest_is_rejected() {
    let out = Temp::new().run(
        &vec![b'x'; MAX_MANIFEST_BYTES + 1],
        CAPABILITY_HEADER.as_bytes(),
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8(out.stderr)
        .unwrap()
        .contains("manifest_too_large"));
}
#[test]
fn malformed_manifest_is_invalid_not_only_blocked() {
    let out = Temp::new().run(b"sentinel-secret", CAPABILITY_HEADER.as_bytes());
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "s14_replay_audit:corpus:1:invalid_header\n"
    );
}
#[test]
fn directory_is_rejected() {
    let t = Temp::new();
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .arg(&t.0)
        .arg(&t.0)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8(out.stderr)
        .unwrap()
        .contains("manifest_not_regular"));
}
#[cfg(unix)]
#[test]
fn symlink_is_rejected() {
    let t = Temp::new();
    let target = t.file("target.tsv", CORPUS_HEADER.as_bytes());
    let link = t.0.join("link.tsv");
    std::os::unix::fs::symlink(target, &link).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .arg(link)
        .arg(t.file("caps.tsv", CAPABILITY_HEADER.as_bytes()))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8(out.stderr)
        .unwrap()
        .contains("manifest_not_regular"));
}
#[test]
fn help_describes_evidence_boundary() {
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("never grants implementation"));
}
#[test]
fn invalid_cutoff_is_rejected_before_io() {
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .args(["a", "b", "--cutoff-epoch", "sentinel-secret"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "s14_replay_audit:invalid_cutoff\n"
    );
}
#[test]
fn unknown_argument_is_rejected() {
    let out = Command::new(env!("CARGO_BIN_EXE_s14-replay-audit"))
        .args(["a", "b", "--publish", "1000"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "s14_replay_audit:invalid_arguments\n"
    );
}
