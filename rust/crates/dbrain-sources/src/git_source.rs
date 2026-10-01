//! Read-only, bounded Git object access. Never resolves HEAD, branches or tags;
//! never reads the worktree, fetches missing objects, executes filters or hooks.
use crate::{
    external::{SourceIr, SourceRevision},
    Result, SourcesError,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    io::Read,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub const MAX_HISTORY_PINS: usize = 32;
pub const MAX_GIT_FILES: usize = 2048;
pub const MAX_GIT_TOTAL_BYTES: usize = 32 * 1024 * 1024;
const GIT_TIMEOUT: Duration = Duration::from_secs(20);

pub fn validate_commit(commit: &str) -> Result<()> {
    if !matches!(commit.len(), 40 | 64)
        || !commit
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err(SourcesError::invalid_input(
            "Git revision must be a full lowercase 40/64-hex commit, never HEAD/ref/abbreviation",
        ));
    }
    Ok(())
}
fn validate_path(path: &str) -> Result<()> {
    if path.is_empty()
        || path.contains(['\0', '\\', ':', '\n', '\r'])
        || path.starts_with('-')
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
        || path
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
    {
        return Err(SourcesError::invalid_input("unsafe Git source path"));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct PinnedRepository {
    repo: PathBuf,
    commit: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitRevisionInfo {
    pub commit: String,
    pub parents: Vec<String>,
    pub commit_time: i64,
    pub shallow_repository: bool,
    pub missing_parents: Vec<String>,
    /// A commit time is NOT a game patch or effective time.
    pub game_valid_from: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChange {
    pub kind: String,
    pub old_path: Option<String>,
    pub new_path: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBlob {
    pub path: String,
    pub oid: String,
    pub size: usize,
}

impl PinnedRepository {
    pub fn open(repo: &Path, commit: &str) -> Result<Self> {
        validate_commit(commit)?;
        let kind = git(repo, &["cat-file", "-t", commit], 128)?;
        if kind != b"commit\n" {
            return Err(SourcesError::invalid_input(
                "Git pin is not a commit object",
            ));
        }
        Ok(Self {
            repo: repo.to_path_buf(),
            commit: commit.into(),
        })
    }
    pub fn commit(&self) -> &str {
        &self.commit
    }
    /// Verify the configured repository identity before a specific adapter
    /// labels blobs as upstream evidence. This is not signature verification.
    pub fn require_origin(&self, allowed: &[&str]) -> Result<()> {
        if allowed.is_empty() || allowed.len() > 8 {
            return Err(SourcesError::invalid_input(
                "explicit bounded Git origin allowlist required",
            ));
        }
        let bytes = git(&self.repo, &["config", "--get", "remote.origin.url"], 4096)?;
        let origin = std::str::from_utf8(&bytes)
            .map_err(|_| SourcesError::invalid_input("invalid Git origin encoding"))?
            .trim();
        if !allowed.contains(&origin) {
            return Err(SourcesError::invalid_input(
                "configured Git origin does not match source identity",
            ));
        }
        Ok(())
    }
    pub fn revision_info(&self) -> Result<GitRevisionInfo> {
        let bytes = git(
            &self.repo,
            &["cat-file", "commit", &self.commit],
            1024 * 1024,
        )?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| SourcesError::invalid_input("non-UTF8 commit metadata"))?;
        let header = text.split("\n\n").next().unwrap_or("");
        let parents: Vec<String> = header
            .lines()
            .filter_map(|l| l.strip_prefix("parent ").map(str::to_owned))
            .collect();
        for parent in &parents {
            validate_commit(parent)?;
        }
        let time = header
            .lines()
            .find_map(|l| l.strip_prefix("committer "))
            .and_then(|s| s.rsplit(' ').nth(1))
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| SourcesError::invalid_input("missing Git commit timestamp"))?;
        let mut missing = Vec::new();
        for parent in &parents {
            if git(
                &self.repo,
                &["cat-file", "-e", &format!("{parent}^{{commit}}")],
                128,
            )
            .is_err()
            {
                missing.push(parent.clone());
            }
        }
        let shallow = git(&self.repo, &["rev-parse", "--is-shallow-repository"], 128)? == b"true\n";
        Ok(GitRevisionInfo {
            commit: self.commit.clone(),
            parents,
            commit_time: time,
            shallow_repository: shallow,
            missing_parents: missing,
            game_valid_from: None,
        })
    }
    /// Caller supplies the exact finite history to inspect. No implicit traversal.
    pub fn history(repo: &Path, pins: &[String]) -> Result<Vec<GitRevisionInfo>> {
        if pins.is_empty() || pins.len() > MAX_HISTORY_PINS {
            return Err(SourcesError::invalid_input(
                "history needs 1..32 explicit commit pins",
            ));
        }
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for pin in pins {
            if seen.insert(pin) {
                out.push(Self::open(repo, pin)?.revision_info()?);
            }
        }
        Ok(out)
    }
    pub fn files(&self, prefix: &str) -> Result<Vec<GitBlob>> {
        validate_path(prefix)?;
        let bytes = git(
            &self.repo,
            &[
                "--literal-pathspecs",
                "ls-tree",
                "-r",
                "-z",
                "-l",
                &self.commit,
                "--",
                prefix,
            ],
            1024 * 1024,
        )?;
        let mut files = Vec::new();
        let mut total = 0usize;
        for entry in bytes.split(|b| *b == 0).filter(|e| !e.is_empty()) {
            let tab = entry
                .iter()
                .position(|b| *b == b'\t')
                .ok_or_else(|| SourcesError::invalid_input("invalid Git tree record"))?;
            let fields: Vec<&str> = std::str::from_utf8(&entry[..tab])
                .map_err(|_| SourcesError::invalid_input("invalid Git tree metadata"))?
                .split_whitespace()
                .collect();
            if fields.len() != 4 || !matches!(fields[0], "100644" | "100755") || fields[1] != "blob"
            {
                return Err(SourcesError::invalid_input(
                    "source tree contains symlink/submodule/non-blob",
                ));
            }
            let path = std::str::from_utf8(&entry[tab + 1..])
                .map_err(|_| SourcesError::invalid_input("non-UTF8 Git source path"))?
                .to_owned();
            validate_path(&path)?;
            let size: usize = fields[3]
                .parse()
                .map_err(|_| SourcesError::invalid_input("invalid Git blob size"))?;
            total = total
                .checked_add(size)
                .ok_or_else(|| SourcesError::invalid_input("Git tree size overflow"))?;
            if size > crate::external::MAX_SOURCE_BYTES
                || total > MAX_GIT_TOTAL_BYTES
                || files.len() >= MAX_GIT_FILES
            {
                return Err(SourcesError::invalid_input(
                    "Git snapshot exceeds file/byte budget",
                ));
            }
            files.push(GitBlob {
                path,
                oid: fields[2].into(),
                size,
            });
        }
        Ok(files)
    }
    pub fn read_blob(&self, path: &str) -> Result<Vec<u8>> {
        validate_path(path)?;
        let files = self.files(path)?;
        let blob = files
            .iter()
            .find(|f| f.path == path)
            .ok_or_else(|| SourcesError::invalid_input("pinned Git blob missing"))?;
        let bytes = git(&self.repo, &["cat-file", "blob", &blob.oid], blob.size)?;
        if bytes.len() != blob.size {
            return Err(SourcesError::invalid_input("pinned Git blob size mismatch"));
        }
        Ok(bytes)
    }
    pub fn json_ir(
        &self,
        source: &str,
        repository_id: &str,
        path: &str,
        parser: &str,
    ) -> Result<SourceIr> {
        if repository_id.trim().is_empty() {
            return Err(SourcesError::invalid_input("repository identity required"));
        }
        let raw = self.read_blob(path)?;
        let mut ir = SourceIr::from_json(
            source,
            &format!("{repository_id}/blob/{}/{path}", self.commit),
            parser,
            SourceRevision::Git {
                commit: self.commit.clone(),
            },
            deadlock_brain_core::now_epoch_seconds()?,
            raw,
        )?;
        ir.add_origin(&format!("{repository_id}@{}:{path}", self.commit))?;
        Ok(ir)
    }
    /// Materialize only pinned data blobs into a private temporary staging tree.
    /// No checkout/archive extraction, symlinks, executable bits or source code runs.
    pub fn materialize_data(&self) -> Result<tempfile::TempDir> {
        let files = self.files("data")?;
        let dir = tempfile::tempdir()?;
        for blob in files {
            let bytes = git(&self.repo, &["cat-file", "blob", &blob.oid], blob.size)?;
            if bytes.len() != blob.size {
                return Err(SourcesError::invalid_input(
                    "Git materialization size mismatch",
                ));
            }
            let target = dir.path().join(&blob.path);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(target, bytes)?;
        }
        Ok(dir)
    }
    /// Renames use Git's bounded exact-content matching; deletions are explicit.
    pub fn diff(&self, other_commit: &str) -> Result<Vec<GitChange>> {
        Self::open(&self.repo, other_commit)?;
        let bytes = git(
            &self.repo,
            &[
                "diff",
                "--no-ext-diff",
                "--no-textconv",
                "--name-status",
                "-z",
                "--find-renames=100%",
                "-l2048",
                &self.commit,
                other_commit,
                "--",
            ],
            1024 * 1024,
        )?;
        let tokens: Vec<_> = bytes.split(|b| *b == 0).filter(|s| !s.is_empty()).collect();
        let mut out = Vec::new();
        let mut i = 0;
        let text = |v: &[u8]| -> Result<String> {
            Ok(std::str::from_utf8(v)
                .map_err(|_| SourcesError::invalid_input("non-UTF8 Git diff"))?
                .to_owned())
        };
        while i < tokens.len() {
            let status = text(tokens[i])?;
            i += 1;
            if i >= tokens.len() {
                return Err(SourcesError::invalid_input("truncated Git diff"));
            }
            let path = text(tokens[i])?;
            validate_path(&path)?;
            i += 1;
            let (kind, old_path, new_path) = match status.as_str() {
                "A" => ("added", None, Some(path)),
                "D" => ("deleted", Some(path), None),
                "M" | "T" => ("modified", Some(path.clone()), Some(path)),
                "R100" if i < tokens.len() => {
                    let next = text(tokens[i])?;
                    validate_path(&next)?;
                    i += 1;
                    ("renamed", Some(path), Some(next))
                }
                _ => return Err(SourcesError::invalid_input("unsupported Git diff status")),
            };
            if out.len() >= MAX_GIT_FILES {
                return Err(SourcesError::invalid_input("Git diff file limit"));
            }
            out.push(GitChange {
                kind: kind.into(),
                old_path,
                new_path,
            });
        }
        Ok(out)
    }
}

fn git(repo: &Path, args: &[&str], max_bytes: usize) -> Result<Vec<u8>> {
    let mut command = Command::new("git");
    command
        .args([
            "--no-optional-locks",
            "--no-replace-objects",
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "core.fsmonitor=false",
            "-c",
            "protocol.allow=never",
        ])
        .args(args)
        .current_dir(repo)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_NO_LAZY_FETCH", "1")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null");
    for name in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_CONFIG_COUNT",
    ] {
        command.env_remove(name);
    }
    let mut child = command.spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| SourcesError::invariant("missing git stdout"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| SourcesError::invariant("missing git stderr"))?;
    let result = thread::scope(|scope| -> Result<Vec<u8>> {
        let out = scope.spawn(move || {
            let mut data = Vec::new();
            stdout
                .take(max_bytes as u64 + 1)
                .read_to_end(&mut data)
                .map(|_| data)
        });
        let err = scope.spawn(move || {
            let mut data = Vec::new();
            stderr.take(8193).read_to_end(&mut data).map(|_| data)
        });
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait()? {
                break status;
            }
            if start.elapsed() > GIT_TIMEOUT {
                let _ = child.kill();
                let _ = child.wait();
                return Err(SourcesError::invalid_input("Git object command timeout"));
            }
            thread::sleep(Duration::from_millis(5));
        };
        let data = out
            .join()
            .map_err(|_| SourcesError::invariant("Git stdout reader panicked"))??;
        let error = err
            .join()
            .map_err(|_| SourcesError::invariant("Git stderr reader panicked"))??;
        if data.len() > max_bytes || error.len() > 8192 {
            return Err(SourcesError::invalid_input("Git command output limit"));
        }
        if !status.success() {
            return Err(SourcesError::invalid_input(format!(
                "pinned Git object unavailable or invalid (exit {:?})",
                status.code()
            )));
        }
        Ok(data)
    });
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cmd(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00Z")
            .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00Z")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().into()
    }
    fn repo() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        cmd(dir.path(), &["init", "-q"]);
        cmd(
            dir.path(),
            &["config", "user.email", "fixture@example.invalid"],
        );
        cmd(dir.path(), &["config", "user.name", "Fixture"]);
        std::fs::create_dir(dir.path().join("data")).unwrap();
        std::fs::write(dir.path().join("data/a.json"), b" {\"id\":1}\n").unwrap();
        cmd(dir.path(), &["add", "data"]);
        cmd(dir.path(), &["commit", "-qm", "fixture one"]);
        let pin = cmd(dir.path(), &["rev-parse", "HEAD"]);
        (dir, pin)
    }
    #[test]
    fn rejects_implicit_and_abbreviated_revisions() {
        for revision in [
            "",
            "HEAD",
            "main",
            "HEAD~1",
            "abcdef1",
            "--all",
            "refs/heads/main",
        ] {
            assert!(validate_commit(revision).is_err());
        }
        assert!(validate_commit(&"a".repeat(40)).is_ok());
    }
    #[test]
    fn dirty_worktree_is_never_read_and_raw_is_exact() {
        let (dir, pin) = repo();
        let repo = PinnedRepository::open(dir.path(), &pin).unwrap();
        std::fs::write(dir.path().join("data/a.json"), "{broken").unwrap();
        assert_eq!(repo.read_blob("data/a.json").unwrap(), b" {\"id\":1}\n");
        let ir = repo
            .json_ir("fixture", "fixture/repo", "data/a.json", "parser-v1")
            .unwrap();
        assert_eq!(
            ir.provenance().raw_sha256,
            crate::external::sha256(b" {\"id\":1}\n")
        );
        let stage = repo.materialize_data().unwrap();
        assert_eq!(
            std::fs::read(stage.path().join("data/a.json")).unwrap(),
            ir.raw()
        );
        assert!(repo.read_blob("../a").is_err());
        assert!(repo.read_blob("data/missing.json").is_err());
    }
    #[test]
    fn bounded_history_diff_rename_delete_and_missing_pin() {
        let (dir, a) = repo();
        cmd(dir.path(), &["mv", "data/a.json", "data/b.json"]);
        cmd(dir.path(), &["commit", "-qm", "rename"]);
        let b = cmd(dir.path(), &["rev-parse", "HEAD"]);
        let repo = PinnedRepository::open(dir.path(), &a).unwrap();
        assert_eq!(repo.diff(&b).unwrap()[0].kind, "renamed");
        cmd(dir.path(), &["rm", "data/b.json"]);
        cmd(dir.path(), &["commit", "-qm", "delete"]);
        let c = cmd(dir.path(), &["rev-parse", "HEAD"]);
        assert_eq!(
            PinnedRepository::open(dir.path(), &b)
                .unwrap()
                .diff(&c)
                .unwrap()[0]
                .kind,
            "deleted"
        );
        let history = PinnedRepository::history(dir.path(), &[a.clone(), b, c]).unwrap();
        assert_eq!(history.len(), 3);
        assert_eq!(history[1].parents, vec![a]);
        assert!(history[0].game_valid_from.is_none());
        assert!(PinnedRepository::history(dir.path(), &vec!["a".repeat(40); 33]).is_err());
        assert!(PinnedRepository::open(dir.path(), &"0".repeat(40)).is_err());
    }
    #[test]
    fn merge_parents_and_shallow_boundary_are_reported() {
        let (dir, root) = repo();
        cmd(dir.path(), &["checkout", "-qb", "side"]);
        std::fs::write(dir.path().join("data/side.json"), b"{}").unwrap();
        cmd(dir.path(), &["add", "data"]);
        cmd(dir.path(), &["commit", "-qm", "side"]);
        cmd(dir.path(), &["checkout", "-qb", "mainline", &root]);
        std::fs::write(dir.path().join("data/main.json"), b"{}").unwrap();
        cmd(dir.path(), &["add", "data"]);
        cmd(dir.path(), &["commit", "-qm", "mainline"]);
        cmd(dir.path(), &["merge", "--no-ff", "-qm", "merge", "side"]);
        let pin = cmd(dir.path(), &["rev-parse", "HEAD"]);
        let info = PinnedRepository::open(dir.path(), &pin)
            .unwrap()
            .revision_info()
            .unwrap();
        assert_eq!(info.parents.len(), 2);
        assert!(info.missing_parents.is_empty());
        let dest = tempfile::tempdir().unwrap();
        let shallow = dest.path().join("shallow");
        let url = format!("file://{}", dir.path().display());
        cmd(
            dest.path(),
            &[
                "clone",
                "--depth",
                "1",
                "--no-local",
                &url,
                shallow.to_str().unwrap(),
            ],
        );
        let boundary = PinnedRepository::open(&shallow, &pin)
            .unwrap()
            .revision_info()
            .unwrap();
        assert!(boundary.shallow_repository);
        assert_eq!(boundary.missing_parents.len(), 2);
        assert_eq!(boundary.parents, info.parents);
    }
    #[cfg(unix)]
    #[test]
    fn rejects_symlink_and_blob_instead_of_commit() {
        let (dir, pin) = repo();
        let blob = cmd(dir.path(), &["rev-parse", &format!("{pin}:data/a.json")]);
        assert!(PinnedRepository::open(dir.path(), &blob).is_err());
        std::os::unix::fs::symlink("/etc/passwd", dir.path().join("data/link")).unwrap();
        cmd(dir.path(), &["add", "data/link"]);
        cmd(dir.path(), &["commit", "-qm", "synthetic symlink"]);
        let pin = cmd(dir.path(), &["rev-parse", "HEAD"]);
        assert!(PinnedRepository::open(dir.path(), &pin)
            .unwrap()
            .materialize_data()
            .is_err());
    }
    #[test]
    fn source_adapter_requires_the_declared_origin() {
        let (dir, pin) = repo();
        let pinned = PinnedRepository::open(dir.path(), &pin).unwrap();
        assert!(pinned
            .require_origin(&["https://fixture.invalid/source.git"])
            .is_err());
        cmd(
            dir.path(),
            &[
                "remote",
                "add",
                "origin",
                "https://fixture.invalid/source.git",
            ],
        );
        assert!(pinned
            .require_origin(&["https://fixture.invalid/source.git"])
            .is_ok());
        assert!(pinned
            .require_origin(&["https://fixture.invalid/another.git"])
            .is_err());
    }
    #[test]
    fn worktree_git_file_is_supported() {
        let (dir, pin) = repo();
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("worktree");
        cmd(
            dir.path(),
            &["worktree", "add", "--detach", path.to_str().unwrap(), &pin],
        );
        assert!(path.join(".git").is_file());
        assert_eq!(
            PinnedRepository::open(&path, &pin)
                .unwrap()
                .read_blob("data/a.json")
                .unwrap(),
            b" {\"id\":1}\n"
        );
    }
}
