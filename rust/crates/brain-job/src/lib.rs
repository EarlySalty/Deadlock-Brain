pub mod config;
pub mod extract;
pub mod git;
pub mod store;

use anyhow::Result;
use config::Config;
use git::{Repository, Source};
use store::{Counts, Run, Store};

pub async fn ingest(config: &Config) -> Result<Counts> {
    let state = config.state()?;
    let store = Store::start(config.pool().await?, config.lease_seconds).await?;
    let mut counts = Counts::default();
    let result = async {
        for source in Source::ALL {
            let source_id = store.source(source).await?;
            let repository = store
                .during(Repository::open(&state, source, config.git_timeout_seconds))
                .await?;
            if let Some(run) = store.pending(source_id).await? {
                process(
                    &store,
                    &repository,
                    source,
                    &run,
                    config.max_blob_bytes,
                    &mut counts,
                )
                .await?;
            }
            let target = store.during(repository.fetch(source)).await?;
            if let Some(run) = store.begin_run(source_id, &target).await? {
                process(
                    &store,
                    &repository,
                    source,
                    &run,
                    config.max_blob_bytes,
                    &mut counts,
                )
                .await?;
            } else {
                counts.sources_unchanged += 1;
            }
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;
    store.finish(&counts, result.is_err()).await?;
    result?;
    Ok(counts)
}

async fn process(
    store: &Store,
    repository: &Repository,
    source: Source,
    run: &Run,
    max_bytes: usize,
    counts: &mut Counts,
) -> Result<()> {
    store.during(repository.verify_commit(&run.target)).await?;
    if run.full {
        if let Some(base) = &run.base {
            let mut deletions = repository.changes(Some(base), &run.target)?;
            while let Some(change) = store.during(deletions.next()).await? {
                if change.oid.is_none() && !store.completed(run, &change.path).await? {
                    store
                        .object(run, source, &change.path, None, counts)
                        .await?;
                }
            }
        }
    }
    let mut changes = repository.changes(
        if run.full { None } else { run.base.as_deref() },
        &run.target,
    )?;
    let mut blobs = repository.blobs(max_bytes)?;
    while let Some(change) = store.during(changes.next()).await? {
        if store.completed(run, &change.path).await? {
            continue;
        }
        let blob = if let Some(oid) = change.oid {
            Some(store.during(blobs.read(&oid)).await?)
        } else {
            None
        };
        store
            .object(run, source, &change.path, blob, counts)
            .await?;
    }
    store.during(blobs.finish()).await?;
    store.publish(run).await?;
    counts.sources_completed += 1;
    Ok(())
}
