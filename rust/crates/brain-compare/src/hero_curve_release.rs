use anyhow::{ensure, Context, Result};
use brain_contracts::{
    game_mode::GameMode, source::origin_from_record, DocumentRevision, PortError, Principal,
    ReleaseReadManifest, RequestDeadline, SnapshotReadPort, SourceRecordV2,
};
use brain_storage::compare_artifact::{
    CompareArtifact, CompareArtifactBody, CompareCalculationVerifier,
};
use dbrain_reasoner::{
    BoonRange, CalculationModels, CalculationScenario, CalculationTarget, DamageModifiers,
    GrowthMetric, HeroCurveComparison, ModelSource, ProgressionInput, ReloadConvention,
    SpiritInput,
};
use std::{collections::BTreeMap, time::Duration};

use crate::hero_curve_artifact::prepare_hero_curve_artifact;
pub use crate::hero_curve_artifact::{HERO_CURVE_MECHANISM, LEGACY_HERO_CURVE_MECHANISM};

const ASSETS_SOURCE: &str = "deadlock_assets_api";
const ASSETS_PREFIX: &str = "https://api.deadlock-api.com/v1/assets/";

struct ReleaseAsset {
    document: DocumentRevision,
    client_version: i64,
}

fn asset_document(manifest: &ReleaseReadManifest, kind: &str) -> Result<ReleaseAsset> {
    let mut found = Vec::new();
    for descriptor in &manifest.revisions {
        let head = &descriptor.head;
        if head.source_id != ASSETS_SOURCE || head.tombstone {
            continue;
        }
        let Some(rest) = head.logical_id.strip_prefix(ASSETS_PREFIX) else {
            continue;
        };
        let Some((path, query)) = rest.split_once('?') else {
            continue;
        };
        let params: BTreeMap<_, _> = query
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .collect();
        if path != kind || params.get("language") != Some(&"english") {
            continue;
        }
        let client_version = params
            .get("client_version")
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0)
            .context("Spielstand der Assetquelle fehlt")?;
        found.push(ReleaseAsset {
            document: DocumentRevision {
                source_id: head.source_id.clone(),
                logical_id: head.logical_id.clone(),
                revision: head.revision,
                content_hash: descriptor.content_hash.clone(),
            },
            client_version,
        });
    }
    ensure!(
        found.len() == 1,
        "Der Quellenstand enthält keine eindeutige Assetquelle für {kind}"
    );
    Ok(found.remove(0))
}

fn model_source(record: &SourceRecordV2, asset: &ReleaseAsset, kind: &str) -> Result<ModelSource> {
    let origin = origin_from_record(record).map_err(anyhow::Error::msg)?;
    ensure!(
        record.logical_id == asset.document.logical_id
            && record.content_hash == asset.document.content_hash
            && origin.locator == record.logical_id,
        "Assetquelle widerspricht dem Quellenstand"
    );
    Ok(ModelSource {
        client_version: asset.client_version,
        document_id: record.logical_id.clone(),
        original_url: origin.locator,
        kind: kind.into(),
        language: "english".into(),
        json_pointer: String::new(),
    })
}

struct ReleaseModels {
    models: CalculationModels,
    documents: [DocumentRevision; 2],
}

fn release_models(reader: &dyn SnapshotReadPort, release_id: &str) -> Result<ReleaseModels> {
    let manifest = reader.read_manifest_until(release_id, None)?;
    manifest.validate()?;
    ensure!(
        manifest.release.release_id == release_id,
        "Quellenstand wurde nicht gefunden"
    );
    let heroes = asset_document(&manifest, "heroes")?;
    let items = asset_document(&manifest, "items")?;
    ensure!(
        heroes.client_version == items.client_version,
        "Helden und Items haben unterschiedliche Spielstände"
    );
    let records = reader.read_documents_until(
        release_id,
        &[heroes.document.clone(), items.document.clone()],
        None,
    )?;
    ensure!(records.len() == 2, "Assetquellen fehlen im Quellenstand");
    let payload = |record: &SourceRecordV2| -> Result<serde_json::Value> {
        Ok(serde_json::from_str(&record.content)?)
    };
    let models = dbrain_reasoner::calculation_models_for_mode(
        &payload(&records[0])?,
        &payload(&records[1])?,
        &model_source(&records[0], &heroes, "heroes")?,
        &model_source(&records[1], &items, "items")?,
        GameMode::Normal,
    )?;
    Ok(ReleaseModels {
        models,
        documents: [heroes.document, items.document],
    })
}

fn hero_id(models: &CalculationModels, name: &str) -> Result<i64> {
    let wanted = name.trim().to_lowercase();
    let ids: Vec<i64> = models
        .heroes
        .iter()
        .filter(|(id, hero)| hero.model.name.to_lowercase() == wanted || id.to_string() == wanted)
        .map(|(id, _)| *id)
        .collect();
    ensure!(ids.len() == 1, "Held {name} wurde nicht eindeutig gefunden");
    Ok(ids[0])
}

fn boon_limit(models: &CalculationModels, hero_id: i64) -> Result<usize> {
    let hero = &models
        .heroes
        .get(&hero_id)
        .context("Kanonische Heldenidentität fehlt")?
        .model;
    Ok(hero
        .level_curve
        .iter()
        .filter(|level| hero.standard_upgrade_levels.contains(&level.level))
        .count())
}

fn empty_scenario() -> CalculationScenario {
    CalculationScenario {
        progression: ProgressionInput::Boons(0),
        expected_level: None,
        expected_unspent_ap: None,
        spirit: SpiritInput::Derived,
        weapon_bonus_percent: None,
        fire_rate_bonus_percent: None,
        item_ids: Vec::new(),
        purchases: Vec::new(),
        inventory_rules: None,
        max_active_items: None,
        ability_order: Vec::new(),
        imbues: BTreeMap::new(),
        secondary_fire: false,
        use_abilities: false,
        target: CalculationTarget {
            health: 2000.0,
            regeneration: 0.0,
            shields: [0.0; 3],
            is_hero: true,
            bullet: DamageModifiers::default(),
            spirit: DamageModifiers::default(),
            changes: Vec::new(),
        },
        hit_fraction: 1.0,
        headshot_fraction: 0.0,
        headshot_bonus: None,
        distance_source_units: None,
        window_seconds: 120.0,
        reload_convention: ReloadConvention::AfterFireInterval,
    }
}

pub fn health_curve(models: &CalculationModels, heroes: [i64; 2]) -> Result<HeroCurveComparison> {
    let max_boons = boon_limit(models, heroes[0])?
        .min(boon_limit(models, heroes[1])?)
        .min(100);
    ensure!(max_boons > 0, "Boonstände fehlen im Quellenstand");
    Ok(dbrain_reasoner::compare_hero_curves(
        models,
        heroes,
        &empty_scenario(),
        BoonRange {
            min_boons: 0,
            max_boons,
        },
        GrowthMetric::Health,
        &RequestDeadline::after(Duration::from_secs(30)),
    )?)
}

fn curve_artifact(
    reader: &dyn SnapshotReadPort,
    principal: &Principal,
    release_id: &str,
    heroes: [i64; 2],
    mechanism_version: &str,
) -> Result<CompareArtifact> {
    let release = release_models(reader, release_id)?;
    let comparison = health_curve(&release.models, heroes)?;
    prepare_hero_curve_artifact(
        reader,
        principal,
        &comparison,
        &release.models,
        release_id,
        &release.documents,
        mechanism_version,
    )
}

pub fn prepare_release_hero_curve(
    reader: &dyn SnapshotReadPort,
    principal: &Principal,
    release_id: &str,
    left: &str,
    right: &str,
) -> Result<CompareArtifact> {
    let models = release_models(reader, release_id)?.models;
    let heroes = [hero_id(&models, left)?, hero_id(&models, right)?];
    curve_artifact(reader, principal, release_id, heroes, HERO_CURVE_MECHANISM)
}

pub struct ReleaseCurveVerifier<'a> {
    pub reader: &'a dyn SnapshotReadPort,
    pub principal: &'a Principal,
}

impl CompareCalculationVerifier for ReleaseCurveVerifier<'_> {
    fn verify(&self, body: &CompareArtifactBody) -> Result<(), PortError> {
        let rejected =
            || PortError::InvalidResponse("Vergleichsrechnung ist nicht bestätigt".into());
        if ![HERO_CURVE_MECHANISM, LEGACY_HERO_CURVE_MECHANISM]
            .contains(&body.mechanism_version.as_str())
        {
            return Err(rejected());
        }
        let comparison: HeroCurveComparison =
            serde_json::from_value(body.calculation.clone()).map_err(|_| rejected())?;
        let rebuilt = curve_artifact(
            self.reader,
            self.principal,
            &body.release.release_id,
            [comparison.left.hero_id, comparison.right.hero_id],
            &body.mechanism_version,
        )
        .map_err(|_| rejected())?;
        if rebuilt.body() != body {
            return Err(rejected());
        }
        Ok(())
    }
}
