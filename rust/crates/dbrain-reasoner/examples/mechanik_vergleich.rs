use std::{collections::BTreeSet, fs};

use dbrain_reasoner::*;
use serde_json::{json, Value};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let evidence: Value = serde_json::from_slice(&fs::read(&args[1])?)?;
    let raw: Value = serde_json::from_slice(&fs::read(&args[2])?)?;
    let entry = &evidence["heroes"][0];
    let mut hero: HeroModel = serde_json::from_value(entry["model"].clone())?;
    let before: Vec<ScoredItem> = serde_json::from_value(entry["item_scores"].clone())?;
    let cfg = ReasonerConfig {
        patch_tag: evidence["patch_tag"].as_str().unwrap().into(),
        use_ai: false,
        ..Default::default()
    };
    hero.damage_plan = mechanics::damage_plan(&hero, &cfg);
    let mut after = Vec::new();
    for previous in &before {
        let mut model = item::build_item_model(&previous.item)?;
        if let Some(source) = raw
            .as_array()
            .unwrap()
            .iter()
            .find(|source| source["id"].as_i64() == Some(model.item_id))
        {
            model.conditional_properties = source["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(_, property)| {
                    property["usage_flags"].as_array().is_some_and(|flags| {
                        flags.iter().any(|flag| flag == "ConditionallyApplied")
                    })
                })
                .map(|(name, _)| name.clone())
                .collect();
        }
        let mut score = item::score_item(
            &model,
            &hero,
            &MetaIndex {
                by_item: Default::default(),
                sample_ok: Default::default(),
            },
            &[],
            &cfg,
        );
        score.score.meta_support = previous.score.meta_support;
        score.score.total += previous.score.meta_support;
        after.push(score);
    }
    let layout: CoreLayoutStats = serde_json::from_value(entry["layout"].clone())?;
    let build = composer::compose_build_with_layout(&hero, &after, &[], &cfg, &layout)?;
    let baseline = composer::compose_build_with_layout(&hero, &before, &[], &cfg, &layout)?;
    let ids: Vec<i64> = serde_json::from_value(entry["reference_779996"]["ids"].clone())?;
    let author = AuthorBuild {
        author: "779996 v45".into(),
        version: 45,
        published_at: None,
        last_updated_at: None,
        patch_tag: None,
        core_item_ids: ids.clone(),
        buy_order: ids,
    };
    let weapons = before
        .iter()
        .filter(|item| {
            item.item.slot == SlotType::Weapon && author.core_item_ids.contains(&item.item.item_id)
        })
        .map(|item| item.item.item_id)
        .collect::<BTreeSet<_>>();
    let hits = |build: &BuildObject| {
        build
            .core
            .iter()
            .filter(|item| weapons.contains(&item.item_id))
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
    };
    let result = json!({"contract":"Mechanikvergleich auf eingefrorenem Helden-/Item-/Referenzstand. Identische skalare Meta-Nebensignale und Layout; Paarstatistik in beiden Seiten deaktiviert. Rohasset nur zum Wiederherstellen ursprünglicher Bedingungsflags. Kein neuer DB-Zugriff.","before": {"build":baseline,"metrics":backtest::backtest_metrics(&baseline,&author),"weapon_hits":hits(&baseline)},"after":{"build":build,"metrics":backtest::backtest_metrics(&build,&author),"weapon_hits":hits(&build)},"reference_weapon_count":weapons.len(),"scores":after});
    fs::write(&args[3], serde_json::to_vec_pretty(&result)?)?;
    eprintln!(
        "{}",
        json!({"before":result["before"]["metrics"],"after":result["after"]["metrics"],"weapon_hits":result["after"]["weapon_hits"],"reference_weapon_count":weapons.len()})
    );
    Ok(())
}
