use anyhow::{ensure, Result};
use brain_contracts::{source::origin_from_record, DocumentRevision, Principal, SnapshotReadPort};
use brain_storage::compare_artifact::{compare_fingerprint, CompareArtifact};
use dbrain_reasoner::{CalculationModels, GrowthMetric, HeroCurveComparison, MeasuredValue};
use std::collections::BTreeSet;

use crate::{
    compare_artifact::prepare_compare_artifact,
    hero_compare_render::{
        BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue, HeroCompareInput,
        HeroCompareSeries, PublicationStatus, VersionBinding,
    },
};

pub fn prepare_hero_curve_artifact(
    reader: &dyn SnapshotReadPort,
    principal: &Principal,
    comparison: &HeroCurveComparison,
    models: &CalculationModels,
    snapshot_id: &str,
    documents: &[DocumentRevision],
    mechanism_version: &str,
) -> Result<CompareArtifact> {
    ensure!(
        comparison.metric == GrowthMetric::Health,
        "Diese Ansicht benötigt die belegte HP-Kennzahl"
    );
    ensure!(
        comparison.client_version > 0 && comparison.client_version == models.client_version,
        "Vergleich und Modelle haben unterschiedliche Clientversionen"
    );
    let left = &comparison.left;
    let right = &comparison.right;
    ensure!(
        left.client_version == comparison.client_version
            && right.client_version == comparison.client_version
            && left.hero_id != right.hero_id
            && left.range == right.range
            && left.scenario == right.scenario,
        "Vergleichsreihen haben unterschiedliche Bedingungen"
    );
    ensure!(
        left.scenario.item_ids.is_empty()
            && left.scenario.purchases.is_empty()
            && left.scenario.ability_order.is_empty()
            && left.scenario.imbues.is_empty()
            && !left.scenario.use_abilities,
        "Die erste HP-Ansicht benötigt ein Szenario ohne Items oder Fähigkeitsverbesserungen"
    );
    ensure!(
        left.range.min_boons <= left.range.max_boons && left.range.max_boons <= 100,
        "Ungültiger Boonbereich"
    );
    let boons: Vec<u32> = (left.range.min_boons..=left.range.max_boons)
        .map(u32::try_from)
        .collect::<Result<_, _>>()?;
    let records = reader.read_documents_until(snapshot_id, documents, None)?;
    ensure!(records.len() == documents.len(), "Rechnungsbelege fehlen");
    let version = VersionBinding {
        snapshot_id: snapshot_id.into(),
        client_version: comparison.client_version.to_string(),
    };
    let binding = CompareBinding {
        result_id: compare_fingerprint(comparison)?,
        version: version.clone(),
        conditions: vec![
            "Lebenspunkte ohne Items und Fähigkeitsverbesserungen".into(),
            format!(
                "Gleiche Boonstände für beide Helden: {} bis {}",
                left.range.min_boons, left.range.max_boons
            ),
        ],
    };
    let sources = records
        .iter()
        .map(|record| {
            let origin = origin_from_record(record).map_err(anyhow::Error::msg)?;
            Ok(CompareSource {
                source_id: format!("{}/{}", record.source_id, record.logical_id),
                evidence: format!(
                    "Originalrevision {:?}; Prüfsumme {}",
                    origin.source_revision, origin.raw_sha256
                ),
                version: version.clone(),
                publication: PublicationStatus::PublicApproved,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let series = |growth: &dbrain_reasoner::HeroGrowth| -> Result<HeroCompareSeries> {
        let hero = models
            .heroes
            .get(&growth.hero_id)
            .ok_or_else(|| anyhow::anyhow!("Kanonische Heldenidentität fehlt"))?;
        ensure!(
            hero.model.hero_id == growth.hero_id
                && hero.source.client_version == comparison.client_version,
            "Heldenidentität widerspricht dem Vergleich"
        );
        ensure!(
            growth.points.len() == boons.len(),
            "Boonreihe ist unvollständig"
        );
        let mut source_ids = BTreeSet::new();
        let values = growth
            .points
            .iter()
            .zip(&boons)
            .map(|(point, boon)| {
                ensure!(
                    u32::try_from(point.boons)? == *boon,
                    "Boonstände widersprechen dem Vergleich"
                );
                let measured = point
                    .metrics
                    .get(&GrowthMetric::Health)
                    .ok_or_else(|| anyhow::anyhow!("HP-Wert fehlt"))?;
                let MeasuredValue::Known {
                    value,
                    unit,
                    sources: evidence,
                    rule,
                } = measured
                else {
                    anyhow::bail!("Nicht belegte Werte werden nicht als null gezeichnet")
                };
                ensure!(
                    value.is_finite()
                        && *value >= 0.0
                        && unit == "health"
                        && !evidence.is_empty()
                        && rule.as_ref().is_some_and(|rule| !rule.is_empty()),
                    "HP-Wert hat keine vollständige Rechnungsbindung"
                );
                for source in evidence {
                    ensure!(
                        source.client_version == comparison.client_version
                            && source.language == hero.source.language,
                        "Messbelege haben unterschiedliche Versionen oder Sprachen"
                    );
                    let matching: Vec<_> = records
                        .iter()
                        .filter(|record| {
                            (record.logical_id == source.document_id
                                || format!("{}/{}", record.source_id, record.logical_id)
                                    == source.document_id)
                                && origin_from_record(record)
                                    .is_ok_and(|origin| origin.locator == source.original_url)
                        })
                        .collect();
                    ensure!(
                        matching.len() == 1,
                        "Messbeleg ist nicht eindeutig an die Originalrevision gebunden"
                    );
                    source_ids.insert(format!(
                        "{}/{}",
                        matching[0].source_id, matching[0].logical_id
                    ));
                }
                Ok(BoonValue {
                    boon: *boon,
                    value: DisplayValue::Quantified(*value),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(HeroCompareSeries {
            hero_id: growth.hero_id.to_string(),
            hero_name: hero.model.name.clone(),
            publication: PublicationStatus::PublicApproved,
            binding: binding.clone(),
            metric: CompareMetric::Hp,
            source_ids: source_ids.into_iter().collect(),
            values,
        })
    };
    let input = HeroCompareInput {
        binding: binding.clone(),
        publication: PublicationStatus::PublicApproved,
        metric: CompareMetric::Hp,
        valid_boon_states: boons.clone(),
        sources,
        heroes: [series(left)?, series(right)?],
    };
    prepare_compare_artifact(
        reader,
        principal,
        &input,
        documents,
        serde_json::to_value(comparison)?,
        mechanism_version.into(),
    )
}
