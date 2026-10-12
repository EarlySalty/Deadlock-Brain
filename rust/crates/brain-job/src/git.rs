use anyhow::{bail, ensure, Context, Result};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
};

pub const SOURCES: [Source; 2] = [
    Source {
        key: "deadlock-data",
        url: "https://github.com/deadlock-wiki/deadlock-data.git",
    },
    Source {
        key: "gametracking",
        url: "https://github.com/SteamTracking/GameTracking-Deadlock.git",
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    pub key: &'static str,
    pub url: &'static str,
}

impl Source {
    pub const ALL: [Self; 2] = SOURCES;

    pub fn url(self) -> &'static str {
        self.url
    }
}

pub struct Blob {
    pub bytes: Vec<u8>,
    pub hash: Vec<u8>,
}

pub struct Repository {
    path: PathBuf,
    timeout: Duration,
}

pub struct Change {
    pub path: String,
    pub oid: Option<String>,
}

pub struct Changes {
    child: Child,
    reader: BufReader<ChildStdout>,
    diff: bool,
    timeout: Duration,
}

pub struct Blobs {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    timeout: Duration,
    limit: usize,
}

fn command(path: &Path) -> Command {
    let mut command = Command::new("/usr/bin/git");
    command
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_ASKPASS", "/bin/false")
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "gc.auto=0",
            "-c",
            "protocol.ext.allow=never",
            "-c",
            "protocol.file.allow=never",
            "-c",
            "core.deltaBaseCacheLimit=16m",
            "-c",
            "pack.windowMemory=16m",
            "-c",
            "pack.threads=1",
        ])
        .arg("--git-dir")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    command
}

impl Repository {
    pub async fn open(state: &Path, source: Source, timeout_seconds: u32) -> Result<Self> {
        let timeout = Duration::from_secs(u64::from(timeout_seconds));
        let cache = state.join("public-gamefiles-v1");
        tokio::fs::create_dir_all(&cache)
            .await
            .context("git_cache_create")?;
        ensure!(
            !std::fs::symlink_metadata(&cache)
                .context("git_cache_metadata")?
                .file_type()
                .is_symlink(),
            "git_cache_symlink"
        );
        let path = cache.join(format!("{}.git", source.key));
        if !path.exists() {
            let mut child = command(&path)
                .args(["init", "--bare"])
                .spawn()
                .context("git_init_spawn")?;
            ensure!(
                tokio::time::timeout(timeout, child.wait())
                    .await
                    .context("git_init_timeout")??
                    .success(),
                "git_init_failed"
            );
        }
        ensure!(
            std::fs::symlink_metadata(&path)
                .context("git_repo_metadata")?
                .is_dir()
                && !std::fs::symlink_metadata(&path)?.file_type().is_symlink(),
            "git_repo_path"
        );
        Ok(Self { path, timeout })
    }

    pub async fn fetch(&self, source: Source) -> Result<String> {
        let timeout = self.timeout;
        let mut child = command(&self.path)
            .args([
                "fetch",
                "--depth=1",
                "--no-tags",
                "--no-recurse-submodules",
                "--no-write-fetch-head",
                source.url,
                "+HEAD:refs/brain-job/fetched",
            ])
            .spawn()
            .context("git_fetch_spawn")?;
        ensure!(
            tokio::time::timeout(timeout, child.wait())
                .await
                .context("git_fetch_timeout")??
                .success(),
            "git_fetch_failed"
        );
        let mut child = command(&self.path)
            .args(["rev-parse", "--verify", "refs/brain-job/fetched^{commit}"])
            .stdout(Stdio::piped())
            .spawn()
            .context("git_revision_spawn")?;
        let mut output = child.stdout.take().context("git_revision_stdout")?;
        let mut bytes = Vec::new();
        tokio::time::timeout(timeout, (&mut output).take(129).read_to_end(&mut bytes))
            .await
            .context("git_revision_timeout")??;
        ensure!(bytes.len() <= 65, "git_revision_size");
        ensure!(
            tokio::time::timeout(timeout, child.wait())
                .await
                .context("git_revision_timeout")??
                .success(),
            "git_revision_failed"
        );
        let revision = std::str::from_utf8(&bytes)
            .context("git_revision_utf8")?
            .trim()
            .to_owned();
        ensure!(valid_oid(&revision), "git_revision_invalid");
        let mut pin = command(&self.path)
            .args([
                "update-ref",
                &format!("refs/brain-job/observed/{revision}"),
                &revision,
            ])
            .spawn()
            .context("git_pin_spawn")?;
        ensure!(
            tokio::time::timeout(timeout, pin.wait())
                .await
                .context("git_pin_timeout")??
                .success(),
            "git_pin_failed"
        );
        Ok(revision)
    }

    pub async fn verify_commit(&self, revision: &str) -> Result<()> {
        ensure!(valid_oid(revision), "git_revision_invalid");
        let mut child = command(&self.path)
            .args(["cat-file", "-e", &format!("{revision}^{{commit}}")])
            .spawn()
            .context("git_commit_spawn")?;
        ensure!(
            tokio::time::timeout(self.timeout, child.wait())
                .await
                .context("git_commit_timeout")??
                .success(),
            "git_commit_missing"
        );
        Ok(())
    }

    pub fn changes(&self, base: Option<&str>, target: &str) -> Result<Changes> {
        ensure!(
            valid_oid(target) && base.is_none_or(valid_oid),
            "git_revision_invalid"
        );
        let mut cmd = command(&self.path);
        if let Some(base) = base {
            cmd.args([
                "diff-tree",
                "--no-commit-id",
                "-r",
                "--raw",
                "-z",
                "--no-abbrev",
                "--no-renames",
                base,
                target,
            ]);
        } else {
            cmd.args(["ls-tree", "-r", "-z", target]);
        }
        let mut child = cmd
            .stdout(Stdio::piped())
            .spawn()
            .context("git_tree_spawn")?;
        let reader = BufReader::new(child.stdout.take().context("git_tree_stdout")?);
        Ok(Changes {
            child,
            reader,
            diff: base.is_some(),
            timeout: self.timeout,
        })
    }

    pub fn blobs(&self, limit: usize) -> Result<Blobs> {
        let mut child = command(&self.path)
            .args(["cat-file", "--batch-command", "--buffer"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("git_blob_spawn")?;
        Ok(Blobs {
            input: child.stdin.take().context("git_blob_stdin")?,
            output: BufReader::new(child.stdout.take().context("git_blob_stdout")?),
            child,
            timeout: self.timeout,
            limit,
        })
    }
}

fn valid_mode(value: &str) -> bool {
    matches!(value, "000000" | "100644" | "100755" | "120000")
}

pub fn valid_oid(value: &str) -> bool {
    matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

async fn delimited(
    reader: &mut BufReader<ChildStdout>,
    delimiter: u8,
    limit: usize,
    timeout: Duration,
) -> Result<Option<Vec<u8>>> {
    let mut bytes = Vec::new();
    loop {
        let buffer = tokio::time::timeout(timeout, reader.fill_buf())
            .await
            .context("git_stream_timeout")??;
        if buffer.is_empty() {
            ensure!(bytes.is_empty(), "git_truncated_record");
            return Ok(None);
        }
        let end = buffer.iter().position(|b| *b == delimiter);
        let count = end.map_or(buffer.len(), |i| i + 1);
        ensure!(
            bytes.len().saturating_add(count) <= limit,
            "git_record_size"
        );
        bytes.extend_from_slice(&buffer[..count]);
        reader.consume(count);
        if end.is_some() {
            bytes.pop();
            return Ok(Some(bytes));
        }
    }
}

impl Changes {
    pub async fn next(&mut self) -> Result<Option<Change>> {
        let Some(record) = delimited(&mut self.reader, 0, 4096, self.timeout).await? else {
            ensure!(
                tokio::time::timeout(self.timeout, self.child.wait())
                    .await
                    .context("git_tree_timeout")??
                    .success(),
                "git_tree_failed"
            );
            return Ok(None);
        };
        if self.diff {
            let header = std::str::from_utf8(&record).context("git_diff_utf8")?;
            let fields: Vec<_> = header.split(' ').collect();
            ensure!(
                fields.len() == 5
                    && fields[0].strip_prefix(':').is_some_and(valid_mode)
                    && valid_mode(fields[1])
                    && valid_oid(fields[2])
                    && valid_oid(fields[3]),
                "git_diff_metadata"
            );
            let path = delimited(&mut self.reader, 0, 2049, self.timeout)
                .await?
                .context("git_diff_path")?;
            let path = checked_path(&path)?;
            let oid = match fields[4] {
                "D" => {
                    ensure!(
                        fields[1] == "000000" && fields[3].bytes().all(|b| b == b'0'),
                        "git_diff_deletion"
                    );
                    None
                }
                "A" | "M" | "T" => {
                    ensure!(
                        matches!(fields[1], "100644" | "100755" | "120000"),
                        "git_diff_mode"
                    );
                    Some(fields[3].to_owned())
                }
                _ => bail!("git_diff_action"),
            };
            Ok(Some(Change { path, oid }))
        } else {
            let split = record
                .iter()
                .position(|b| *b == b'\t')
                .context("git_tree_metadata")?;
            let header = std::str::from_utf8(&record[..split]).context("git_tree_utf8")?;
            let fields: Vec<_> = header.split(' ').collect();
            ensure!(
                fields.len() == 3
                    && fields[1] == "blob"
                    && valid_oid(fields[2])
                    && matches!(fields[0], "100644" | "100755" | "120000"),
                "git_tree_metadata"
            );
            Ok(Some(Change {
                path: checked_path(&record[split + 1..])?,
                oid: Some(fields[2].to_owned()),
            }))
        }
    }
}

fn checked_path(bytes: &[u8]) -> Result<String> {
    let path = std::str::from_utf8(bytes).context("git_path_utf8")?;
    ensure!(
        !path.is_empty()
            && path.len() <= 512
            && !path.starts_with('/')
            && !path.chars().any(char::is_control)
            && !path.split('/').any(|part| matches!(part, "" | "." | "..")),
        "git_path_invalid"
    );
    Ok(path.to_owned())
}

impl Blobs {
    pub async fn read(&mut self, oid: &str) -> Result<Blob> {
        ensure!(valid_oid(oid), "git_blob_oid");
        tokio::time::timeout(
            self.timeout,
            self.input
                .write_all(format!("contents {oid}\nflush\n").as_bytes()),
        )
        .await
        .context("git_blob_timeout")??;
        self.input.flush().await.context("git_blob_flush")?;
        let header = delimited(&mut self.output, b'\n', 256, self.timeout)
            .await?
            .context("git_blob_header")?;
        let header = std::str::from_utf8(&header).context("git_blob_metadata")?;
        let fields: Vec<_> = header.split(' ').collect();
        ensure!(
            fields.len() == 3 && fields[0] == oid && fields[1] == "blob",
            "git_blob_type"
        );
        let size: usize = fields[2].parse().context("git_blob_size")?;
        ensure!(
            size <= self.limit && fields[2].bytes().all(|b| b.is_ascii_digit()),
            "git_blob_limit"
        );
        let mut bytes = Vec::with_capacity(size);
        let mut hash = Sha256::new();
        let mut chunk = [0; 64 * 1024];
        while bytes.len() < size {
            let length = chunk.len().min(size - bytes.len());
            tokio::time::timeout(self.timeout, self.output.read_exact(&mut chunk[..length]))
                .await
                .context("git_blob_timeout")??;
            hash.update(&chunk[..length]);
            bytes.extend_from_slice(&chunk[..length]);
        }
        let delimiter = tokio::time::timeout(self.timeout, self.output.read_u8())
            .await
            .context("git_blob_timeout")??;
        ensure!(delimiter == b'\n', "git_blob_delimiter");
        Ok(Blob {
            bytes,
            hash: hash.finalize().to_vec(),
        })
    }

    pub async fn finish(mut self) -> Result<()> {
        self.input.shutdown().await.context("git_blob_shutdown")?;
        drop(self.input);
        ensure!(
            tokio::time::timeout(self.timeout, self.child.wait())
                .await
                .context("git_blob_exit_timeout")??
                .success(),
            "git_blob_failed"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_git(path: &Path, args: &[&str], input: &[u8]) -> String {
        use std::io::Write;
        let mut child = std::process::Command::new("/usr/bin/git")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_AUTHOR_NAME", "Fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
            .env("GIT_COMMITTER_NAME", "Fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
            .args(["--git-dir"])
            .arg(path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    #[tokio::test]
    async fn real_git_streams_full_diff_deletion_and_bounded_binary_blobs() {
        let state = tempfile::tempdir().unwrap();
        let repository = Repository::open(state.path(), Source::ALL[0], 30)
            .await
            .unwrap();
        let bytes = [0, 1, 2, b'\n', 255];
        let oid = fixture_git(&repository.path, &["hash-object", "-w", "--stdin"], &bytes);
        let tree = fixture_git(
            &repository.path,
            &["mktree"],
            format!("100644 blob {oid}\tdata.bin\n").as_bytes(),
        );
        let a = fixture_git(&repository.path, &["commit-tree", &tree], b"a\n");
        let empty = fixture_git(&repository.path, &["mktree"], b"");
        let b = fixture_git(&repository.path, &["commit-tree", &empty, "-p", &a], b"b\n");
        repository.verify_commit(&a).await.unwrap();
        let mut full = repository.changes(None, &a).unwrap();
        let change = full.next().await.unwrap().unwrap();
        assert_eq!(change.path, "data.bin");
        assert_eq!(change.oid.as_deref(), Some(oid.as_str()));
        assert!(full.next().await.unwrap().is_none());
        let mut blobs = repository.blobs(5).unwrap();
        let blob = blobs.read(&oid).await.unwrap();
        assert_eq!(blob.bytes, bytes);
        assert_eq!(blob.hash, Sha256::digest(bytes).to_vec());
        blobs.finish().await.unwrap();
        let mut bounded = repository.blobs(4).unwrap();
        assert!(bounded.read(&oid).await.is_err());
        drop(bounded);
        let mut wrong_type = repository.blobs(4096).unwrap();
        assert!(wrong_type.read(&a).await.is_err());
        drop(wrong_type);
        let mut deleted = repository.changes(Some(&a), &b).unwrap();
        let deletion = deleted.next().await.unwrap().unwrap();
        assert_eq!(deletion.path, "data.bin");
        assert!(deletion.oid.is_none());
        assert!(deleted.next().await.unwrap().is_none());
        let mut unchanged = repository.changes(Some(&a), &a).unwrap();
        assert!(unchanged.next().await.unwrap().is_none());
        let mut restored = repository.changes(Some(&b), &a).unwrap();
        assert_eq!(
            restored.next().await.unwrap().unwrap().oid.as_deref(),
            Some(oid.as_str())
        );
    }

    #[test]
    fn metadata_paths_and_object_ids_reject_executable_input() {
        for invalid in ["", "../outside", "/absolute", "x//y", "x/./y", "line\nfeed"] {
            assert!(checked_path(invalid.as_bytes()).is_err());
        }
        assert!(checked_path(b"folder/file with spaces.json").is_ok());
        assert!(valid_oid(&"a".repeat(40)));
        assert!(valid_oid(&"a".repeat(64)));
        assert!(!valid_oid(&"A".repeat(40)));
        assert!(!valid_oid("--exec"));
    }
}
