use crate::{
    config::Config,
    docs_extract::{self, MAX_BYTES},
    docs_store::DocsStore,
    git::{Repository, Source},
    store::{Counts, Run},
};
use anyhow::{ensure, Context, Result};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub const SOURCE: Source = Source {
    key: "deadlock-docs",
    url: "https://github.com/EarlySalty/Deadlock-Docs.git",
};

pub async fn ingest(config: &Config) -> Result<Counts> {
    let state = config.state()?;
    let store = DocsStore::start(config.pool().await?, config.lease_seconds).await?;
    let mut counts = Counts::default();
    let result = async {
        let source = store.source().await?;
        let repository = store
            .during(Repository::open(&state, SOURCE, config.git_timeout_seconds))
            .await?;
        if let Some(run) = store.pending(source).await? {
            process(
                &store,
                &repository,
                &state,
                &run,
                config.git_timeout_seconds,
                config.max_blob_bytes.min(MAX_BYTES),
                &mut counts,
            )
            .await?;
        }
        if let Some(run) = store.backfill_run(source).await? {
            process(
                &store,
                &repository,
                &state,
                &run,
                config.git_timeout_seconds,
                config.max_blob_bytes.min(MAX_BYTES),
                &mut counts,
            )
            .await?;
        }
        let target = store.during(repository.fetch(SOURCE)).await?;
        if let Some(run) = store.begin_run(source, &target).await? {
            process(
                &store,
                &repository,
                &state,
                &run,
                config.git_timeout_seconds,
                config.max_blob_bytes.min(MAX_BYTES),
                &mut counts,
            )
            .await?;
        } else {
            counts.sources_unchanged += 1;
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;
    store.finish(&counts, result.is_err()).await?;
    result?;
    Ok(counts)
}

pub async fn process(
    store: &DocsStore,
    repository: &Repository,
    state: &Path,
    run: &Run,
    timeout_seconds: u32,
    max_bytes: usize,
    counts: &mut Counts,
) -> Result<()> {
    store.during(repository.verify_commit(&run.target)).await?;
    store
        .during(verify_modes(state, &run.target, timeout_seconds))
        .await?;
    if run.full {
        if let Some(base) = &run.base {
            let mut deletions = repository.changes(Some(base), &run.target)?;
            while let Some(change) = store.during(deletions.next()).await? {
                if change.oid.is_none()
                    && docs_extract::supported(&change.path)
                    && !store.completed(run, &change.path).await?
                {
                    store.object(run, &change.path, None, counts).await?;
                }
            }
        }
    }
    let mut changes = repository.changes(
        if run.full { None } else { run.base.as_deref() },
        &run.target,
    )?;
    let mut blobs = repository.blobs(max_bytes.min(MAX_BYTES))?;
    while let Some(change) = store.during(changes.next()).await? {
        if !docs_extract::supported(&change.path) || store.completed(run, &change.path).await? {
            continue;
        }
        let blob = if let Some(oid) = change.oid {
            if run.full && run.base.as_deref() == Some(&run.target) {
                Some(store.backfill_blob(run, &change.path).await?)
            } else {
                Some(store.during(blobs.read(&oid)).await?)
            }
        } else {
            None
        };
        if let Some(blob) = &blob {
            ensure!(blob.bytes.len() <= max_bytes.min(MAX_BYTES), "docs_blob_limit");
        }
        store.object(run, &change.path, blob, counts).await?;
    }
    store.during(blobs.finish()).await?;
    store.publish(run).await?;
    counts.sources_completed += 1;
    Ok(())
}

pub async fn verify_modes(state: &Path, revision: &str, timeout_seconds: u32) -> Result<()> {
    ensure!(
        revision.len() == 40 && crate::git::valid_oid(revision),
        "docs_revision"
    );
    let path = state.join("public-gamefiles-v1").join("deadlock-docs.git");
    let timeout = Duration::from_secs(u64::from(timeout_seconds));
    let mut child = Command::new("/usr/bin/git")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .args(["-c", "core.hooksPath=/dev/null", "--git-dir"])
        .arg(path)
        .args(["ls-tree", "-r", "-z", revision, "--", "public", "internal"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("docs_mode_spawn")?;
    let mut reader = BufReader::new(child.stdout.take().context("docs_mode_stdout")?);
    loop {
        let mut bytes = Vec::new();
        loop {
            let buffer = tokio::time::timeout(timeout, reader.fill_buf())
                .await
                .context("docs_mode_timeout")??;
            if buffer.is_empty() {
                break;
            }
            let end = buffer.iter().position(|b| *b == 0);
            let count = end.map_or(buffer.len(), |end| end + 1);
            ensure!(bytes.len().saturating_add(count) <= 4096, "docs_mode_size");
            bytes.extend_from_slice(&buffer[..count]);
            reader.consume(count);
            if end.is_some() {
                break;
            }
        }
        if bytes.is_empty() {
            break;
        }
        ensure!(bytes.pop() == Some(0), "docs_mode_truncated");
        let record = std::str::from_utf8(&bytes).context("docs_mode_utf8")?;
        let (metadata, path) = record.split_once('\t').context("docs_mode_metadata")?;
        if path.ends_with(".html") {
            ensure!(docs_extract::supported(path), "docs_mode_path");
            let fields: Vec<_> = metadata.split(' ').collect();
            ensure!(
                fields.len() == 3
                    && matches!(fields[0], "100644" | "100755")
                    && fields[1] == "blob"
                    && fields[2].len() == 40
                    && crate::git::valid_oid(fields[2]),
                "docs_mode_not_regular"
            );
        }
    }
    ensure!(
        tokio::time::timeout(timeout, child.wait())
            .await
            .context("docs_mode_timeout")??
            .success(),
        "docs_mode_failed"
    );
    Ok(())
}
