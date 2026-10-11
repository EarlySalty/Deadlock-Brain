use anyhow::{ensure, Context, Result};
use brain_contracts::{
    postgres::Postgres, CorpusSnapshot, DocumentHead, DocumentRevision, PortError, Principal,
    SnapshotReadPort,
};
use brain_maintenance::{
    compare_artifact::publish_compare_artifact,
    hero_curve_release::{prepare_release_hero_curve, ReleaseCurveVerifier},
};
use brain_storage::{
    compare_artifact::{ComparePublication, CompareStage},
    LocalPgReader, PgStore,
};
use serde_json::json;
use std::{collections::BTreeSet, path::PathBuf};

use crate::site;

#[derive(Debug, clap::Args)]
pub struct CompareHeroesArgs {
    #[command(flatten)]
    database: site::database::ConnectionArgs,
    #[arg(long)]
    serve_config: PathBuf,
    #[arg(long)]
    left: String,
    #[arg(long)]
    right: String,
    #[arg(long)]
    publish: Option<String>,
}

struct FrozenRelease(CorpusSnapshot);

impl SnapshotReadPort for FrozenRelease {
    fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
        if release_id != self.0.release.release_id {
            return Err(PortError::Unavailable("Quellenstand fehlt".into()));
        }
        Ok(self.0.clone())
    }

    fn read_heads(&self, documents: &[DocumentRevision]) -> Result<Vec<DocumentHead>, PortError> {
        Ok(self
            .0
            .heads
            .iter()
            .filter(|head| {
                documents.iter().any(|document| {
                    document.source_id == head.source_id && document.logical_id == head.logical_id
                })
            })
            .map(DocumentHead::from)
            .collect())
    }
}

fn public_principal() -> Principal {
    Principal {
        actor_id: "public-site".into(),
        channel: "public-site".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    }
}

async fn frozen_release(args: &CompareHeroesArgs, release: String) -> Result<FrozenRelease> {
    let path = args
        .database
        .pg_config
        .clone()
        .context("Der Vergleich benötigt einen lokalen Peer-Zugang.")?;
    let config: Postgres = serde_json::from_slice(&tokio::fs::read(&path).await?)
        .context("Die lokale Datenbankkonfiguration ist ungültig.")?;
    let snapshot = tokio::task::spawn_blocking(move || -> Result<CorpusSnapshot> {
        let reader = LocalPgReader::new(
            &config.socket_dir,
            config.port,
            &config.username,
            &config.database,
        )?;
        Ok(reader.read_snapshot(&release)?)
    })
    .await??;
    Ok(FrozenRelease(snapshot))
}

pub async fn run(args: CompareHeroesArgs) -> Result<()> {
    let release = site::profiles::release_from_config(&args.serve_config).await?;
    let store = PgStore::new(args.database.pool("brain_migrate").await?);
    let reader = frozen_release(&args, release.clone()).await?;
    let principal = public_principal();
    let artifact =
        prepare_release_hero_curve(&reader, &principal, &release, &args.left, &args.right)?;
    store.save_pending_compare(&artifact).await?;
    let stage = match &args.publish {
        Some(confirmed) => {
            ensure!(
                confirmed == artifact.id(),
                "Die Bestätigung gehört zu einem anderen Vergleich."
            );
            publish_compare_artifact(
                &store,
                &reader,
                &principal,
                &artifact,
                &ReleaseCurveVerifier {
                    reader: &reader,
                    principal: &principal,
                },
            )
            .await?;
            CompareStage::Published
        }
        None => {
            store
                .read_site_compare(artifact.id(), &principal)
                .await?
                .context("Die Vorschau ist nicht lesbar.")?
                .1
        }
    };
    let publication = ComparePublication::for_id(artifact.id());
    let body = artifact.body();
    crate::print_json(&json!({
        "artifact_id": artifact.id(),
        "stage": stage,
        "preview_path": publication.html_path,
        "chart_path": publication.svg_path,
        "public_url": match stage {
            CompareStage::Published => Some(publication.public_link()?),
            CompareStage::Preview => None,
        },
        "release": body.release,
        "mechanism_version": body.mechanism_version,
        "dependencies": body.dependencies,
        "render_model": body.render_model,
    }))
}
