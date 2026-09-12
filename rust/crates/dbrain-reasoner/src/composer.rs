use std::cmp::Ordering;

use crate::{
    BuildItem, BuildObject, BuyPhase, Confidence, Evidence, EvidenceKind, HeroModel, PatchDelta,
    ReasonerConfig, ScoredItem, SituationBlock, SituationKind,
};

fn confidence_rank(confidence: &Confidence) -> u8 {
    match confidence {
        Confidence::Low => 0,
        Confidence::Medium => 1,
        Confidence::High => 2,
    }
}

fn lower(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn is_shield(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("shield") || name.contains("reactive barrier")
}

fn is_can_buy_one(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "metal skin",
        "dispel magic",
        "counterspell",
        "vampiric burst",
        "spirit resilience",
        "bullet resilience",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn is_tryhard(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("slowing hex")
}

fn is_optional(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "healing booster",
        "silencer",
        "spellslinger",
        "toxic bullets",
        "split shot",
        "ricochet",
        "armor piercing rounds",
        "crippling headshot",
        "spellbreaker",
        "plated armor",
        "inhibitor",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn is_counter(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "anti-heal",
        "silencer",
        "spellbreaker",
        "inhibitor",
        "crippling",
        "toxic bullets",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn patched_score(item: &ScoredItem, deltas: &[PatchDelta]) -> f64 {
    item.score.total
        + deltas
            .iter()
            .filter_map(|delta| match delta.target {
                crate::DeltaTarget::Item(item_id) if item_id == item.item.item_id => {
                    Some(delta.sign as f64 * delta.magnitude)
                }
                _ => None,
            })
            .sum::<f64>()
}

fn item_order<'a>(
    scored: &'a [ScoredItem],
    deltas: &[PatchDelta],
    blocked: &[String],
) -> Vec<&'a ScoredItem> {
    let mut items = scored
        .iter()
        .filter(|item| item.item.shopable && !item.item.disabled)
        .filter(|item| {
            let id = item.item.item_id.to_string();
            !blocked.iter().any(|issue| {
                let issue = lower(issue);
                issue.contains(&lower(&item.item.name)) || issue.contains(&id)
            })
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        patched_score(right, deltas)
            .partial_cmp(&patched_score(left, deltas))
            .unwrap_or(Ordering::Equal)
            .then_with(|| right.item.tier.cmp(&left.item.tier))
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    items
}

fn build_item(item: &ScoredItem, hero: &HeroModel, sources: Vec<Evidence>) -> BuildItem {
    let imbue_target = if item.item.imbueable {
        hero.abilities
            .iter()
            .find(|ability| {
                ability.damage_type == item.item.damage_axis
                    || ability.damage_type == hero.damage_plan.primary_axis
            })
            .map(|ability| ability.ability_id)
            .or_else(|| hero.abilities.first().map(|ability| ability.ability_id))
    } else {
        None
    };
    BuildItem {
        item_id: item.item.item_id,
        name: item.item.name.clone(),
        tier: item.item.tier,
        buy_phase: item.buy_phase.clone(),
        why: String::new(),
        confidence: item.confidence.clone(),
        imbue_target,
        sell_priority: match item.buy_phase {
            BuyPhase::Lane => Some(if item.item.tier <= 1 { 1 } else { 2 }),
            BuyPhase::Core | BuyPhase::Late => None,
        },
        sources,
    }
}

fn confidence(items: &[BuildItem]) -> Confidence {
    items
        .iter()
        .map(|item| item.confidence.clone())
        .min_by_key(confidence_rank)
        .unwrap_or(Confidence::Low)
}

fn patch_sources(item: &ScoredItem, deltas: &[PatchDelta]) -> Vec<Evidence> {
    let mut sources = item.sources.clone();
    for delta in deltas {
        if matches!(delta.target, crate::DeltaTarget::Item(item_id) if item_id == item.item.item_id)
        {
            sources.push(Evidence {
                kind: EvidenceKind::Patch,
                detail: delta.note.clone(),
            });
        }
    }
    sources
}

pub fn compose_build(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> BuildObject {
    compose_build_with_blocklist(hero, scored, deltas, cfg, &[])
}

pub fn compose_build_with_sources(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    blocked: &[String],
    meta: &crate::meta::MetaIndexWithSources,
) -> BuildObject {
    let mut build = compose_build_with_blocklist(hero, scored, deltas, cfg, blocked);
    let (order, source) = meta.ability_order(hero.hero_id);
    build.ability_order = order;
    build.rationale = source.detail;
    build
}

pub fn compose_build_with_blocklist(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    _cfg: &ReasonerConfig,
    blocked: &[String],
) -> BuildObject {
    let ordered = item_order(scored, deltas, blocked);
    let mut core = Vec::new();
    let mut can_buy = Vec::new();
    let mut tryhard = Vec::new();
    let mut shields = Vec::new();
    let mut counters = Vec::new();
    let mut optional = Vec::new();
    for item in ordered {
        if is_shield(item) {
            shields.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_can_buy_one(item) {
            can_buy.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_tryhard(item) {
            tryhard.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_optional(item) {
            optional.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_counter(item) {
            counters.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if core.len() < 19 {
            core.push(build_item(item, hero, patch_sources(item, deltas)));
        } else {
            optional.push(build_item(item, hero, patch_sources(item, deltas)));
        }
    }
    can_buy.truncate(6);
    let mut situations = Vec::new();
    if !can_buy.is_empty() {
        situations.push(SituationBlock {
            label: "Can buy 1".to_string(),
            optional: true,
            kind: SituationKind::CanBuyN(1),
            items: can_buy,
        });
    }
    if !tryhard.is_empty() {
        situations.push(SituationBlock {
            label: "Tryhard".to_string(),
            optional: true,
            kind: SituationKind::Tryhard,
            items: tryhard,
        });
    }
    if !shields.is_empty() {
        situations.push(SituationBlock {
            label: "Shields".to_string(),
            optional: true,
            kind: SituationKind::Shields,
            items: shields,
        });
    }
    if !optional.is_empty() {
        situations.push(SituationBlock {
            label: "Optional".to_string(),
            optional: true,
            kind: SituationKind::Optional,
            items: optional,
        });
    }
    if !counters.is_empty() {
        situations.push(SituationBlock {
            label: "Counters".to_string(),
            optional: true,
            kind: SituationKind::Counters,
            items: counters,
        });
    }
    let mut all_items = core.clone();
    all_items.extend(situations.iter().flat_map(|block| block.items.clone()));
    BuildObject {
        hero_id: hero.hero_id,
        hero_name: hero.name.clone(),
        patch_tag: _cfg.patch_tag.clone(),
        name: format!("{} Reasoner Build", hero.name),
        core,
        situations,
        ability_order: Vec::new(),
        confidence: confidence(&all_items),
        rationale: "Skill-Order: keine Quelle".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BuyPhase, DamagePlan, DamageType, ItemModel, ItemScore, PurchaseBonuses, SlotType,
        WeaponProfile,
    };
    use std::collections::BTreeMap;

    fn hero() -> HeroModel {
        HeroModel {
            hero_id: 25,
            name: "Warden".to_string(),
            archetype: "brawler".to_string(),
            base_health: 1.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: Vec::new(),
                weapon: Vec::new(),
                vitality: Vec::new(),
            },
            scaling: Vec::new(),
            weapon: WeaponProfile {
                bullet_damage: 1.0,
                shots_per_second: 1.0,
                clip_size: 1.0,
                reload_duration: 1.0,
                range: 1.0,
                falloff_start_range: 1.0,
                falloff_end_range: 1.0,
                sustained_dps: 1.0,
            },
            abilities: Vec::new(),
            damage_plan: DamagePlan {
                weapon_dps: 1.0,
                spirit_dps: 0.0,
                weapon_share: 1.0,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    fn item(id: i64, name: &str, score: f64, active: bool, defense: &[&str]) -> ScoredItem {
        ScoredItem {
            item: ItemModel {
                item_id: id,
                name: name.to_string(),
                slot: SlotType::Weapon,
                tier: 2,
                cost: 1,
                is_active: active,
                shopable: true,
                disabled: false,
                damage_axis: DamageType::Weapon,
                defense_kind: defense.iter().map(|value| value.to_string()).collect(),
                properties: BTreeMap::new(),
                passive_properties: BTreeMap::new(),
                condition: crate::ConditionKind::None,
                proc_cooldown: None,
                imbueable: false,
            },
            score: ItemScore {
                combat_value: score,
                per_slot_value: 0.0,
                per_soul_value: 0.0,
                purchase_bonus_value: 0.0,
                condition_factor: 1.0,
                active_value: 0.0,
                passive_value: 0.0,
                meta_support: 0.0,
                total: score,
            },
            confidence: Confidence::High,
            buy_phase: BuyPhase::Core,
            sources: vec![Evidence {
                kind: EvidenceKind::Mechanic,
                detail: "fixture".to_string(),
            }],
        }
    }

    #[test]
    fn sells_lane_items_before_early_items_and_keeps_core() {
        let mut lane = item(1, "Lane", 3.0, false, &[]);
        lane.buy_phase = BuyPhase::Lane;
        lane.item.tier = 1;
        let mut early = item(2, "Early", 2.0, false, &[]);
        early.buy_phase = BuyPhase::Lane;
        let mut core = item(3, "Core", 1.0, false, &[]);
        core.item.tier = 1;
        let build = compose_build(
            &hero(),
            &[lane, early, core],
            &[],
            &ReasonerConfig::default(),
        );
        let payload = crate::publish::publish_task_payload(&build);
        assert_eq!(payload["mod_categories"][0]["mods"][0]["sell_priority"], 1);
        assert_eq!(payload["mod_categories"][0]["mods"][1]["sell_priority"], 2);
        assert!(payload["mod_categories"][0]["mods"][2]
            .get("sell_priority")
            .is_none());
    }

    #[test]
    fn assigns_all_warden_seed_items_to_five_reference_blocks() {
        let seed: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../.tasks/2026-09-12-build-reasoner/referenz/lightbringer-warden.json"
        ))
        .unwrap();
        let blocks = seed["bloecke"].as_array().unwrap();
        let mut scored = Vec::new();
        for block in blocks {
            for entry in block["items"].as_array().unwrap() {
                let name = entry["name"].as_str().unwrap();
                let defense = match name {
                    "Spirit Resilience" | "Bullet Resilience" | "Metal Skin"
                    | "Reactive Barrier" | "Witchmail" | "Plated Armor" => vec!["resist"],
                    "Veil Walker" | "Spirit Shielding" | "Weapon Shielding" => vec!["shield"],
                    _ => vec![],
                };
                let mut candidate = item(
                    scored.len() as i64 + 1,
                    name,
                    100.0 - scored.len() as f64,
                    entry["active"].as_bool().unwrap_or(false) || name == "Reactive Barrier",
                    &defense,
                );
                candidate.item.tier = entry["tier"].as_i64().unwrap();
                if name == "Blood Tribute" {
                    candidate.item.damage_axis = DamageType::Spirit;
                }
                scored.push(candidate);
            }
        }
        let build = compose_build(&hero(), &scored, &[], &ReasonerConfig::default());
        assert_eq!(build.situations.len(), 4);
        let actual = std::iter::once(("Core Items", &build.core)).chain(
            build
                .situations
                .iter()
                .map(|block| (block.label.as_str(), &block.items)),
        );
        for ((label, items), expected) in actual.zip(blocks) {
            assert_eq!(label, expected["name"].as_str().unwrap());
            assert_eq!(
                items
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>(),
                expected["items"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| item["name"].as_str().unwrap())
                    .collect::<Vec<_>>(),
                "{label}"
            );
        }
        assert_eq!(
            scored.len(),
            build.core.len()
                + build
                    .situations
                    .iter()
                    .map(|block| block.items.len())
                    .sum::<usize>()
        );
    }

    #[test]
    fn ability_sources_and_sell_priorities_survive_publish_roundtrip() {
        use crate::meta::{AuthorBuildSource, MetaIndexWithSources};
        use crate::{AbilityStep, MetaIndex};
        let steps = |id| {
            vec![AbilityStep {
                ability_id: id,
                currency_type: 2,
                delta: -1,
            }]
        };
        let source = |hero_id, weight, author: &str, id| AuthorBuildSource {
            hero_id,
            weight,
            author: author.to_string(),
            details: serde_json::json!({"abilityOrder":{"currencyChanges":[
                {"abilityId":id,"currencyType":2,"delta":-1},
                {"abilityId":id,"currencyType":1,"delta":-2}
            ]}}),
        };
        let mut meta = MetaIndexWithSources {
            index: MetaIndex {
                by_item: BTreeMap::new(),
                sample_ok: Default::default(),
            },
            author_builds: vec![
                source(25, 1.0, "Lower", 100),
                source(25, 3.0, "Best", 101),
                source(99, 9.0, "Other hero", 999),
            ],
            hero_ability_orders: BTreeMap::from([(25, steps(102))]),
        };
        let mut lane = item(1, "Lane", 1.0, false, &[]);
        lane.buy_phase = BuyPhase::Lane;
        lane.item.tier = 1;
        let compose = |meta: &MetaIndexWithSources| {
            compose_build_with_sources(
                &hero(),
                std::slice::from_ref(&lane),
                &[],
                &ReasonerConfig::default(),
                &[],
                meta,
            )
        };
        let build = compose(&meta);
        let restored_build: BuildObject =
            serde_json::from_value(serde_json::to_value(&build).unwrap()).unwrap();
        let payload = crate::publish::publish_task_payload(&restored_build);
        let restored: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&payload).unwrap()).unwrap();
        assert_eq!(payload["ability_order"][0]["ability_id"], 101);
        assert_eq!(payload["ability_order"][1]["delta"], -2);
        assert_eq!(restored["mod_categories"][0]["mods"][0]["sell_priority"], 1);
        assert_eq!(restored["ability_order"].as_array().unwrap().len(), 2);
        assert!(restored["description"].as_str().unwrap().contains("Best"));
        meta.author_builds.reverse();
        assert_eq!(compose(&meta).ability_order, build.ability_order);
        meta.author_builds
            .push(source(25, f64::NAN, "Invalid weight", 998));
        assert_eq!(compose(&meta).ability_order, build.ability_order);
        meta.author_builds.retain(|source| source.hero_id != 25);
        let mut malformed = source(25, 5.0, "Incomplete", 103);
        malformed.details["abilityOrder"]["currencyChanges"][1]
            .as_object_mut()
            .unwrap()
            .remove("delta");
        meta.author_builds.push(malformed);
        let fallback = compose(&meta);
        assert_eq!(fallback.ability_order, steps(102));
        assert!(fallback.rationale.contains("brain.hero_ability_orders"));
        meta.hero_ability_orders.clear();
        let missing = compose(&meta);
        assert!(missing.ability_order.is_empty());
        assert!(missing.rationale.contains("keine Quelle"));
        meta.author_builds.push(AuthorBuildSource {
            hero_id: 25,
            author: "Snake case".to_string(),
            weight: 1.0,
            details: serde_json::json!({"ability_order": [
                {"ability_id":"104", "currency_type":"2", "delta":"-1"}
            ]}),
        });
        assert_eq!(compose(&meta).ability_order, steps(104));
        let default_build = compose_build(&hero(), &[], &[], &ReasonerConfig::default());
        assert!(default_build.rationale.contains("keine Quelle"));
    }

    #[test]
    fn keeps_core_first_and_emits_reference_situation_order() {
        let scored = vec![
            item(1, "Core Gun", 10.0, false, &[]),
            item(2, "Metal Skin", 9.0, true, &["bullet_resist"]),
            item(3, "Spirit Shielding", 8.0, false, &["shield"]),
            item(4, "Slowing Hex", 7.0, true, &[]),
        ];
        let build = compose_build(&hero(), &scored, &[], &ReasonerConfig::default());
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Core Gun"]
        );
        assert_eq!(
            build
                .situations
                .iter()
                .map(|block| block.label.as_str())
                .collect::<Vec<_>>(),
            vec!["Can buy 1", "Tryhard", "Shields"]
        );
        assert_eq!(
            build.situations[0].items[0].sources[0].kind,
            EvidenceKind::Mechanic
        );
    }
}
