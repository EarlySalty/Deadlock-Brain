use anyhow::{ensure, Result};
use brain_contracts::{DocumentRevision, Principal, SnapshotReadPort};
use brain_storage::{
    compare_artifact::{
        compare_dependency, compare_sha256, CompareArtifact, CompareArtifactBody,
        CompareCalculationVerifier, ComparePublication, CompareReleaseBinding,
    },
    PgStore,
};
use serde::{Deserialize, Serialize};

use crate::hero_compare_render::{
    render_hero_compare, BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue,
    HeroCompareInput, HeroCompareSeries, PublicationStatus, RenderedHeroCompare, VersionBinding,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RenderModel {
    result_id: String,
    snapshot_id: String,
    client_version: String,
    conditions: Vec<String>,
    metric: String,
    boons: Vec<u32>,
    sources: Vec<RenderSource>,
    heroes: [RenderHero; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RenderSource {
    id: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RenderHero {
    id: String,
    name: String,
    sources: Vec<String>,
    values: Vec<f64>,
}

impl RenderModel {
    fn from_input(input: &HeroCompareInput) -> Result<Self> {
        render_hero_compare(input)?;
        let hero = |hero: &HeroCompareSeries| -> RenderHero {
            RenderHero {
                id: hero.hero_id.clone(),
                name: hero.hero_name.clone(),
                sources: hero.source_ids.clone(),
                values: hero
                    .values
                    .iter()
                    .map(|point| match point.value {
                        DisplayValue::Quantified(value) => value,
                        _ => unreachable!(),
                    })
                    .collect(),
            }
        };
        Ok(Self {
            result_id: input.binding.result_id.clone(),
            snapshot_id: input.binding.version.snapshot_id.clone(),
            client_version: input.binding.version.client_version.clone(),
            conditions: input.binding.conditions.clone(),
            metric: match input.metric {
                CompareMetric::BaseDps => "base_dps",
                CompareMetric::MagazineDamage => "magazine_damage",
                CompareMetric::Hp => "hp",
            }
            .into(),
            boons: input.valid_boon_states.clone(),
            sources: input
                .sources
                .iter()
                .map(|source| RenderSource {
                    id: source.source_id.clone(),
                    evidence: source.evidence.clone(),
                })
                .collect(),
            heroes: [hero(&input.heroes[0]), hero(&input.heroes[1])],
        })
    }

    fn input(&self) -> Result<HeroCompareInput> {
        let metric = match self.metric.as_str() {
            "base_dps" => CompareMetric::BaseDps,
            "magazine_damage" => CompareMetric::MagazineDamage,
            "hp" => CompareMetric::Hp,
            _ => anyhow::bail!("Unbekannte Vergleichskennzahl"),
        };
        let binding = CompareBinding {
            result_id: self.result_id.clone(),
            version: VersionBinding {
                snapshot_id: self.snapshot_id.clone(),
                client_version: self.client_version.clone(),
            },
            conditions: self.conditions.clone(),
        };
        let hero = |hero: &RenderHero| -> Result<HeroCompareSeries> {
            ensure!(
                hero.values.len() == self.boons.len(),
                "Vergleichsreihe ist unvollständig"
            );
            Ok(HeroCompareSeries {
                hero_id: hero.id.clone(),
                hero_name: hero.name.clone(),
                publication: PublicationStatus::PublicApproved,
                binding: binding.clone(),
                metric,
                source_ids: hero.sources.clone(),
                values: self
                    .boons
                    .iter()
                    .zip(&hero.values)
                    .map(|(boon, value)| BoonValue {
                        boon: *boon,
                        value: DisplayValue::Quantified(*value),
                    })
                    .collect(),
            })
        };
        Ok(HeroCompareInput {
            binding: binding.clone(),
            publication: PublicationStatus::PublicApproved,
            metric,
            valid_boon_states: self.boons.clone(),
            sources: self
                .sources
                .iter()
                .map(|source| CompareSource {
                    source_id: source.id.clone(),
                    evidence: source.evidence.clone(),
                    version: binding.version.clone(),
                    publication: PublicationStatus::PublicApproved,
                })
                .collect(),
            heroes: [hero(&self.heroes[0])?, hero(&self.heroes[1])?],
        })
    }
}

pub fn prepare_compare_artifact(
    reader: &dyn SnapshotReadPort,
    principal: &Principal,
    input: &HeroCompareInput,
    documents: &[DocumentRevision],
    calculation: serde_json::Value,
    mechanism_version: String,
) -> Result<CompareArtifact> {
    ensure!(
        !documents.is_empty() && documents.len() <= 32,
        "Vergleichsabhängigkeiten fehlen"
    );
    let mut model = RenderModel::from_input(input)?;
    let manifest = reader.read_manifest_until(&input.binding.version.snapshot_id, None)?;
    manifest.validate()?;
    let records = reader.read_documents_until(&manifest.release.release_id, documents, None)?;
    ensure!(
        records.len() == documents.len(),
        "Vergleichsabhängigkeiten sind unvollständig"
    );
    let dependencies = records
        .iter()
        .map(compare_dependency)
        .collect::<Result<Vec<_>, _>>()?;
    ensure!(
        dependencies
            .iter()
            .map(|dependency| &dependency.document)
            .eq(documents.iter()),
        "Vergleichsabhängigkeiten widersprechen der Rechnung"
    );
    let expected_sources: std::collections::BTreeSet<_> = documents
        .iter()
        .map(|document| format!("{}/{}", document.source_id, document.logical_id))
        .collect();
    let supplied_sources: std::collections::BTreeSet<_> = model
        .sources
        .iter()
        .map(|source| source.id.clone())
        .collect();
    ensure!(
        expected_sources.len() == documents.len() && supplied_sources == expected_sources,
        "Darstellung enthält nicht alle Rechnungsabhängigkeiten"
    );
    model.result_id = compare_sha256(&serde_json::to_vec(&(
        &calculation,
        &mechanism_version,
        &dependencies,
    ))?);
    let rendered = render_hero_compare(&model.input()?)?;
    let artifact = CompareArtifact::pending(CompareArtifactBody {
        release: CompareReleaseBinding::from_release(&manifest.release)?,
        calculation,
        render_model: serde_json::to_value(model)?,
        mechanism_version,
        dependencies,
        html: rendered.html,
        svg: rendered.svg,
    })?;
    artifact.check_public(reader, principal)?;
    Ok(artifact)
}

pub async fn publish_compare_artifact(
    store: &PgStore,
    reader: &dyn SnapshotReadPort,
    principal: &Principal,
    artifact: &CompareArtifact,
    verifier: &dyn CompareCalculationVerifier,
) -> Result<ComparePublication> {
    render_stored_compare(artifact)?;
    Ok(store
        .publish_compare_artifact(artifact, reader, principal, verifier)
        .await?)
}

pub fn render_stored_compare(artifact: &CompareArtifact) -> Result<RenderedHeroCompare> {
    let model: RenderModel = serde_json::from_value(artifact.body().render_model.clone())?;
    ensure!(
        model.snapshot_id == artifact.body().release.release_id,
        "Vergleich hat einen anderen Datenstand"
    );
    let expected_result = compare_sha256(&serde_json::to_vec(&(
        &artifact.body().calculation,
        &artifact.body().mechanism_version,
        &artifact.body().dependencies,
    ))?);
    ensure!(
        model.result_id == expected_result,
        "Darstellung gehört zu einer anderen Rechnung"
    );
    let expected_sources: std::collections::BTreeSet<_> = artifact
        .body()
        .dependencies
        .iter()
        .map(|dependency| {
            format!(
                "{}/{}",
                dependency.document.source_id, dependency.document.logical_id
            )
        })
        .collect();
    ensure!(
        expected_sources.len() == artifact.body().dependencies.len()
            && model
                .sources
                .iter()
                .map(|source| source.id.clone())
                .collect::<std::collections::BTreeSet<_>>()
                == expected_sources,
        "Quellenbindung der Darstellung ist unvollständig"
    );
    let rendered = render_hero_compare(&model.input()?)?;
    ensure!(
        rendered.html == artifact.body().html && rendered.svg == artifact.body().svg,
        "Vergleichsausgabe passt nicht zur gespeicherten Rechnung"
    );
    Ok(rendered)
}
