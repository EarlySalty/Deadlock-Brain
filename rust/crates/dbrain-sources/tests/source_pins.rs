use std::{fs, path::Path, process::Command};

use dbrain_sources::{
    deadlock_data::{preflight, PARSER_REVISION},
    source_pins::{DeadlockDataPin, SourcePins},
    PullDeadlockDataOptions,
};

fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .current_dir(repo)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "Fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .env("GIT_AUTHOR_DATE", "2026-09-01T00:00:00Z")
        .env("GIT_COMMITTER_DATE", "2026-09-01T00:00:00Z")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

fn fixture() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "--quiet"]);
    git(
        dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/deadlock-wiki/deadlock-data.git",
        ],
    );
    fs::create_dir(dir.path().join("data")).unwrap();
    fs::write(dir.path().join("data/version.txt"), "ClientVersion=123\n").unwrap();
    git(dir.path(), &["add", "data"]);
    git(dir.path(), &["commit", "--quiet", "-m", "fixture v1"]);
    let commit = git(dir.path(), &["rev-parse", "HEAD"]);
    (dir, commit)
}

fn options(repo: &Path, commit: &str) -> PullDeadlockDataOptions {
    PullDeadlockDataOptions {
        repo_dir: repo.to_owned(),
        pin: DeadlockDataPin {
            commit: commit.into(),
            parser_revision: PARSER_REVISION.into(),
            schema_version: Some(1),
            data_version: Some("123".into()),
        },
        update_repo: false,
    }
}

#[test]
fn valid_pin_and_same_pin_are_reproducible_despite_head_and_worktree_changes() {
    let (repo, old) = fixture();
    let old_options = options(repo.path(), &old);
    let first = preflight(&old_options).unwrap().materialize_data().unwrap();
    fs::write(repo.path().join("data/version.txt"), "ClientVersion=456\n").unwrap();
    git(repo.path(), &["add", "data"]);
    git(repo.path(), &["commit", "--quiet", "-m", "fixture v2"]);
    fs::write(
        repo.path().join("data/version.txt"),
        "uncommitted wrong content\n",
    )
    .unwrap();
    let second = preflight(&old_options).unwrap().materialize_data().unwrap();
    assert_eq!(
        fs::read(first.path().join("data/version.txt")).unwrap(),
        fs::read(second.path().join("data/version.txt")).unwrap()
    );
    assert_eq!(preflight(&old_options).unwrap().commit(), old);
}

#[test]
fn update_only_occurs_after_explicit_configuration_change() {
    let (repo, old) = fixture();
    let config_file = repo.path().join("source-pins.json");
    let mut config = SourcePins {
        schema_version: 1,
        deadlock_data: options(repo.path(), &old).pin,
    };
    fs::write(&config_file, serde_json::to_vec(&config).unwrap()).unwrap();
    fs::write(repo.path().join("data/version.txt"), "ClientVersion=456\n").unwrap();
    git(repo.path(), &["add", "data"]);
    git(repo.path(), &["commit", "--quiet", "-m", "fixture v2"]);
    let new = git(repo.path(), &["rev-parse", "HEAD"]);
    let loaded = SourcePins::read(&config_file).unwrap();
    assert_eq!(loaded.deadlock_data.commit, old);
    config.deadlock_data.commit = new.clone();
    config.deadlock_data.data_version = Some("456".into());
    fs::write(&config_file, serde_json::to_vec(&config).unwrap()).unwrap();
    let updated = SourcePins::read(&config_file).unwrap();
    assert_eq!(
        preflight(&PullDeadlockDataOptions {
            repo_dir: repo.path().to_owned(),
            pin: updated.deadlock_data,
            update_repo: false
        })
        .unwrap()
        .commit(),
        new
    );
    assert_eq!(loaded.deadlock_data.commit, old);
}

#[test]
fn absent_commit_noncommit_wrong_origin_and_version_are_errors() {
    let (repo, commit) = fixture();
    let mut opts = options(repo.path(), &"f".repeat(40));
    assert!(preflight(&opts)
        .unwrap_err()
        .to_string()
        .contains("unavailable locally"));
    opts.pin.commit = git(
        repo.path(),
        &["rev-parse", &format!("{commit}:data/version.txt")],
    );
    assert!(preflight(&opts).is_err());
    opts.pin.commit = commit;
    opts.pin.data_version = Some("wrong".into());
    assert!(preflight(&opts)
        .unwrap_err()
        .to_string()
        .contains("data_version"));
    opts.pin.data_version = None;
    opts.update_repo = true;
    assert!(preflight(&opts).is_err());
    opts.update_repo = false;
    git(
        repo.path(),
        &[
            "remote",
            "set-url",
            "origin",
            "https://example.invalid/unapproved.git",
        ],
    );
    assert!(preflight(&opts)
        .unwrap_err()
        .to_string()
        .contains("source identity"));
}
