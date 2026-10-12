#![forbid(unsafe_code)]

pub mod registry;
pub mod sources;
pub use brain_storage::gamefacts::{
    limits, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject, Receipt,
    Rejection, Relation,
};

pub trait Extractor: Sync {
    fn name(&self) -> &'static str;
    fn accepts(&self, source: &str, path: &str) -> bool;
    fn extract(&self, raw: &RawObject) -> Outcome;
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
    let run_id = store.begin_run(source).await?;
    let mut counts = Counts::default();
    let result = process(store, run_id, source, objects, extractors, &mut counts).await;
    store.finish_run(run_id, result.is_ok(), counts).await?;
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
    for object in objects {
        let object = object?;
        let raw = store
            .capture(run_id, source, &object.revision, &object.path, object.bytes)
            .await?;
        let Some(extractor) = extractors.iter().find(|e| e.accepts(source, &raw.path)) else {
            counts.ignored += 1;
            continue;
        };
        let outcome = match extractor.extract(&raw) {
            Outcome::Accepted(set) => match store.apply(run_id, &raw, extractor.name(), &set).await
            {
                Ok(()) => Outcome::Accepted(set),
                Err(rejection) => Outcome::Rejected(rejection),
            },
            outcome => outcome,
        };
        match outcome {
            Outcome::Accepted(_) => counts.accepted += 1,
            Outcome::Rejected(rejection) => {
                store
                    .reject(run_id, raw.id, extractor.name(), &rejection)
                    .await?;
                counts.rejected += 1;
            }
            Outcome::Ignored => counts.ignored += 1,
        }
        store.finish_run(run_id, false, *counts).await?;
    }
    Ok(())
}
