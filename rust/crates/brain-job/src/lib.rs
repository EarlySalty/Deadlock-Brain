#![forbid(unsafe_code)]

pub mod assets;
pub mod config;
pub mod hero;
pub mod patch_extract;
pub mod registry;
pub mod sources;
pub mod timers;
pub use brain_storage::gamefacts::{
    limits, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject, Receipt,
    Rejection, Relation,
};
pub use sources::{InputObjects, Source};

pub trait Extractor: Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str {
        "1"
    }
    fn accepts(&self, source: &str, path: &str) -> bool;
    fn extract(&self, raw: &RawObject) -> Outcome;
}

pub fn extractor_id(name: &str, version: &str) -> String {
    format!("v1:{}:{name}:{}:{version}", name.len(), version.len())
}

#[derive(Debug)]
pub enum Outcome {
    Accepted(FactSet),
    Rejected(Rejection),
    Ignored,
}

#[derive(Debug)]
pub struct InputObject {
    pub revision: String,
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct RunReport {
    pub run_id: i64,
    pub counts: Counts,
}

pub async fn ingest<I>(
    store: &GameFactsStore,
    source: &str,
    objects: I,
    extractors: &[&dyn Extractor],
) -> anyhow::Result<RunReport>
where
    I: IntoIterator<Item = anyhow::Result<InputObject>>,
{
    let mut run_guard = store.lock_run().await?;
    let run_id = store.begin_run(source).await?;
    let mut counts = Counts::default();
    let result = process(store, run_id, source, objects, extractors, &mut counts).await;
    store
        .finish_run_guarded(&mut run_guard, run_id, result.is_ok(), counts)
        .await?;
    result?;
    Ok(RunReport { run_id, counts })
}

async fn process<I>(
    store: &GameFactsStore,
    run_id: i64,
    source: &str,
    objects: I,
    extractors: &[&dyn Extractor],
    counts: &mut Counts,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = anyhow::Result<InputObject>>,
{
    let mut supported = false;
    for object in objects {
        let object = object?;
        let raw = store
            .capture(run_id, source, &object.revision, &object.path, object.bytes)
            .await?;
        let Some(extractor) = extractors.iter().find(|e| e.accepts(source, &raw.path)) else {
            counts.ignored += 1;
            continue;
        };
        supported = true;
        let discriminator = extractor_id(extractor.name(), extractor.version());
        if store
            .was_applied_in_run(run_id, raw.id, &discriminator)
            .await?
        {
            counts.ignored += 1;
            continue;
        }
        let extracted = if raw.bytes.len() > limits::RAW_BYTES {
            Outcome::Rejected(Rejection::new(
                "raw_too_large",
                "Rohobjekt überschreitet die Extraktionsgrenze",
            ))
        } else {
            extractor.extract(&raw)
        };
        let outcome = match extracted {
            Outcome::Accepted(set) => match store.apply(run_id, &raw, &discriminator, &set).await {
                Ok(()) => Outcome::Accepted(set),
                Err(rejection) => Outcome::Rejected(rejection),
            },
            outcome => outcome,
        };
        match outcome {
            Outcome::Accepted(_) => counts.accepted += 1,
            Outcome::Rejected(rejection) => {
                store
                    .reject(run_id, raw.id, &discriminator, &rejection)
                    .await?;
                counts.rejected += 1;
            }
            Outcome::Ignored => counts.ignored += 1,
        }
        store.finish_run(run_id, false, *counts).await?;
    }
    anyhow::ensure!(
        supported || counts.ignored == 0,
        "Quelle enthält keine unterstützten Objekte: {source}"
    );
    Ok(())
}

pub mod receipt_audit;
