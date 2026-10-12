pub mod config;
pub mod extract;
pub mod git;
pub mod store;

use anyhow::Result;
use config::Config;
use git::{Repository, Source};
use std::collections::BTreeMap;
use store::{Counts, Run, Store};

const IDENTITY_PATHS: [&str; 2] = ["data/json/hero-data.json", "data/json/ability-data.json"];
const LOCALIZATION_PATHS: [&str; 2] = [
    "data/localizations/english.json",
    "data/localizations/german.json",
];

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
    let mut identities_changed = false;
    let mut localizations = BTreeMap::new();
    while let Some(change) = store.during(changes.next()).await? {
        if source == Source::ALL[0] {
            identities_changed |= IDENTITY_PATHS.contains(&change.path.as_str());
            if LOCALIZATION_PATHS.contains(&change.path.as_str()) {
                localizations.insert(change.path, change.oid);
                continue;
            }
        }
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
    if identities_changed && !run.full {
        let mut dependencies = repository.paths(&run.target, &LOCALIZATION_PATHS)?;
        while let Some(change) = store.during(dependencies.next()).await? {
            localizations.entry(change.path).or_insert(change.oid);
        }
    }
    for (path, oid) in localizations {
        if store.completed(run, &path).await? {
            continue;
        }
        let blob = if let Some(oid) = oid {
            Some(store.during(blobs.read(&oid)).await?)
        } else {
            None
        };
        store.object(run, source, &path, blob, counts).await?;
    }
    store.during(blobs.finish()).await?;
    store.publish(run).await?;
    counts.sources_completed += 1;
    Ok(())
}

#[cfg(test)]
#[path = "ingest_tests.rs"]
mod tests;
