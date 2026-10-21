use brain_job::docs::source as docs_source;

use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_REPOSITORY: AtomicU64 = AtomicU64::new(0);

struct Repository(PathBuf);

impl Repository {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "brain-p5-source-{}-{}",
            std::process::id(),
            NEXT_REPOSITORY.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        command(&path, &["init", "--quiet"]);
        Self(path)
    }

    fn commit(&self) {
        command(&self.0, &["add", "."]);
        command(
            &self.0,
            &[
                "-c",
                "user.name=P5 Test",
                "-c",
                "user.email=p5-test@localhost",
                "commit",
                "--quiet",
                "-m",
                "fixture",
            ],
        );
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn command(path: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(["-c", "core.hooksPath=/dev/null"])
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "fixture Git command failed");
}

#[test]
fn docs_source_streams_only_scoped_html_at_one_commit() {
    let repository = Repository::new();
    for path in [
        "public/guides/baba.html",
        "internal/brain/runbook.html",
        "public/page.txt",
        "unscoped.html",
    ] {
        let destination = repository.0.join(path);
        std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
        std::fs::write(destination, format!("original {path}")).unwrap();
    }
    repository.commit();
    let loader = brain_job::sources::SOURCES
        .iter()
        .find(|(name, _)| *name == "deadlock-docs")
        .expect("registered Docs source")
        .1;
    assert!(brain_job::registry::EXTRACTORS
        .iter()
        .any(|extractor| extractor.name() == "deadlock-docs"
            && extractor.accepts("deadlock-docs", "public/guides/baba.html")));
    let mut source = loader(&repository.0, false);
    let first = source.next().unwrap().unwrap();
    std::fs::write(repository.0.join("public/guides/baba.html"), "changed").unwrap();
    repository.commit();
    let second = source.next().unwrap().unwrap();
    assert_eq!(first.path, "internal/brain/runbook.html");
    assert_eq!(second.path, "public/guides/baba.html");
    assert_eq!(first.revision, second.revision);
    assert_eq!(second.bytes, b"original public/guides/baba.html");
    assert!(source.next().is_none());
    assert!(source.next().is_none());
    assert_eq!(
        docs_source::REPOSITORY_URL,
        "https://github.com/EarlySalty/Deadlock-Docs.git"
    );
}

#[test]
fn docs_source_failure_is_terminal_for_the_iterator() {
    let repository = Repository::new();
    let mut source = docs_source::load(&repository.0, false);
    assert!(source.next().unwrap().is_err());
    assert!(source.next().is_none());
}
