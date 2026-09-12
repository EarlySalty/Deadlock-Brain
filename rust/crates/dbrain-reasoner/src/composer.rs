use std::cmp::Ordering;

use crate::{
    BuildItem, BuildObject, BuyPhase, Confidence, CoreLayoutStats, Evidence, EvidenceKind,
    HeroModel, PatchDelta, ReasonerConfig, ScoredItem, SituationBlock, SituationKind, SlotType,
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

fn item_order<'a>(scored: &'a [ScoredItem], blocked: &[String]) -> Vec<&'a ScoredItem> {
    let mut items = scored
        .iter()
        .filter(|item| item.item.shopable && !item.item.disabled)
        .filter(|item| item.score.total.is_finite())
        .filter(|item| {
            let id = item.item.item_id.to_string();
            !blocked.iter().any(|issue| {
                let issue = lower(issue);
                issue.contains(&lower(&item.item.name)) || issue.contains(&id)
            })
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        right
            .score
            .total
            .partial_cmp(&left.score.total)
            .unwrap_or(Ordering::Equal)
            .then_with(|| right.item.tier.cmp(&left.item.tier))
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    items
}

fn is_situation_item(item: &ScoredItem) -> bool {
    is_shield(item)
        || is_can_buy_one(item)
        || is_tryhard(item)
        || is_optional(item)
        || is_counter(item)
}

fn phase_rank(phase: &BuyPhase) -> u8 {
    match phase {
        BuyPhase::Lane => 0,
        BuyPhase::Mid => 1,
        BuyPhase::Core => 2,
        BuyPhase::Late => 3,
    }
}

fn slot_index(slot: &SlotType) -> usize {
    match slot {
        SlotType::Weapon => 0,
        SlotType::Vitality => 1,
        SlotType::Spirit => 2,
    }
}

fn core_candidates<'a>(ordered: &[&'a ScoredItem]) -> Vec<&'a ScoredItem> {
    ordered
        .iter()
        .copied()
        .filter(|item| !is_situation_item(item) && item.score.total > 0.0)
        .collect()
}

fn select_core_items<'a>(
    candidates: &[&'a ScoredItem],
    layout: &CoreLayoutStats,
) -> Vec<&'a ScoredItem> {
    let mut selected: Vec<&ScoredItem> = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    let mut slot_counts = [0usize; 3];
    let mut flex_used = 0usize;
    for tier in 1..=5 {
        let target = layout.target_for_tier(tier);
        if target == 0 {
            continue;
        }
        let mut band = candidates
            .iter()
            .copied()
            .filter(|item| item.item.tier == tier && !used.contains(&item.item.item_id))
            .collect::<Vec<_>>();
        band.sort_by(|left, right| {
            let primary = if tier <= 2 {
                right
                    .score
                    .per_soul_value
                    .total_cmp(&left.score.per_soul_value)
            } else {
                right.score.total.total_cmp(&left.score.total)
            };
            primary
                .then_with(|| right.score.total.total_cmp(&left.score.total))
                .then_with(|| {
                    right
                        .score
                        .per_soul_value
                        .total_cmp(&left.score.per_soul_value)
                })
                .then_with(|| left.item.item_id.cmp(&right.item.item_id))
        });
        for item in band {
            if selected
                .iter()
                .filter(|selected| selected.item.tier == tier)
                .count()
                >= target
            {
                break;
            }
            let slot = slot_index(&item.item.slot);
            if slot_counts[slot] >= 4 {
                if flex_used >= layout.flex_slots {
                    continue;
                }
                flex_used += 1;
            }
            slot_counts[slot] += 1;
            used.insert(item.item.item_id);
            selected.push(item);
        }
    }
    selected.sort_by(|left, right| {
        phase_rank(&left.buy_phase)
            .cmp(&phase_rank(&right.buy_phase))
            .then_with(|| right.score.total.total_cmp(&left.score.total))
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    selected
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
        confidence: if item.score.total <= 0.0 {
            Confidence::Low
        } else {
            item.confidence.clone()
        },
        imbue_target,
        sell_priority: match item.buy_phase {
            BuyPhase::Lane => Some(if item.item.tier <= 1 { 1 } else { 2 }),
            BuyPhase::Mid | BuyPhase::Core | BuyPhase::Late => None,
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
    let mut build = compose_build_with_layout_and_blocklist(
        hero,
        scored,
        deltas,
        cfg,
        blocked,
        meta.core_layouts.for_hero(hero.hero_id),
    );
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
    compose_build_with_layout_and_blocklist(
        hero,
        scored,
        deltas,
        _cfg,
        blocked,
        &CoreLayoutStats::default(),
    )
}

pub fn compose_build_with_layout(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    layout: &CoreLayoutStats,
) -> BuildObject {
    compose_build_with_layout_and_blocklist(hero, scored, deltas, cfg, &[], layout)
}

fn compose_build_with_layout_and_blocklist(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    blocked: &[String],
    layout: &CoreLayoutStats,
) -> BuildObject {
    let ordered = item_order(scored, blocked);
    let selected = select_core_items(&core_candidates(&ordered), layout);
    let selected_ids = selected
        .iter()
        .map(|item| item.item.item_id)
        .collect::<std::collections::BTreeSet<_>>();
    let core = selected
        .into_iter()
        .map(|item| build_item(item, hero, patch_sources(item, deltas)))
        .collect::<Vec<_>>();
    let mut can_buy = Vec::new();
    let mut tryhard = Vec::new();
    let mut shields = Vec::new();
    let mut counters = Vec::new();
    let mut optional = Vec::new();
    for item in ordered {
        if selected_ids.contains(&item.item.item_id) {
            continue;
        }
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
        } else {
            optional.push(build_item(item, hero, patch_sources(item, deltas)));
        }
    }
    can_buy.truncate(6);
    optional.truncate(12);
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
        patch_tag: cfg.patch_tag.clone(),
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
    #[test]
    fn fix_e_nonpositive_scores_never_enter_core_or_keep_high_confidence() {
        let scored = vec![
            item(1, "Zero", 0.0, false, &[]),
            item(2, "Negative", -2.0, false, &[]),
        ];
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 19)], 48),
        );
        assert!(build.core.is_empty());
        assert!(
            build
                .situations
                .iter()
                .flat_map(|block| &block.items)
                .all(|item| item.confidence != Confidence::High)
        );
    }

    #[test]
    fn fix_e_optional_keeps_only_twelve_highest_scores() {
        let scored = (1..=60)
            .map(|id| item(id, &format!("Item {id}"), id as f64, false, &[]))
            .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 19)], 48),
        );
        let optional = build
            .situations
            .iter()
            .find(|block| block.label == "Optional")
            .unwrap();
        assert_eq!(optional.items.len(), 12);
        assert_eq!(
            optional
                .items
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            (30..=41).rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn fix_e_composer_does_not_apply_deltas_to_scores_again() {
        let scored = vec![
            item(1, "First", 10.0, false, &[]),
            item(2, "Second", 9.0, false, &[]),
        ];
        let deltas = crate::patch::compute_patch_delta(
            &hero(),
            &[
                serde_json::json!({"item_id":2,"change_type":"buff","old_value":1,"new_value":100,"raw_line":"Damage increased from 1 to 100"}),
            ],
        );
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &deltas,
            &ReasonerConfig::default(),
            &layout(&[(2, 2)], 0),
        );
        assert_eq!(build.core[0].item_id, 1);
    }
    use super::*;
    use crate::{
        BuyPhase, CoreLayoutBand, CoreLayoutStats, DamagePlan, DamageType, ItemModel, ItemScore,
        PurchaseBonuses, SlotType, WeaponProfile,
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

    fn layout(targets: &[(i64, usize)], flex_slots: usize) -> CoreLayoutStats {
        CoreLayoutStats {
            source_builds: 1,
            total_median: targets.iter().map(|(_, target)| *target).sum::<usize>() as f64,
            total_lower_quartile: 0.0,
            total_upper_quartile: 0.0,
            flex_slots,
            bands: targets
                .iter()
                .map(|(tier, target)| {
                    (
                        *tier,
                        CoreLayoutBand {
                            tier: *tier,
                            median: *target as f64,
                            lower_quartile: 0.0,
                            upper_quartile: 0.0,
                            target: *target,
                        },
                    )
                })
                .collect(),
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
        let build = compose_build_with_layout(
            &hero(),
            &[lane, early, core],
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 2), (2, 1)], 0),
        );
        let payload = crate::publish::publish_task_payload(&build);
        assert_eq!(payload["mod_categories"][0]["mods"][0]["sell_priority"], 1);
        assert_eq!(payload["mod_categories"][0]["mods"][1]["sell_priority"], 2);
        assert!(
            payload["mod_categories"][0]["mods"][2]
                .get("sell_priority")
                .is_none()
        );
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
                candidate.item.slot = match name {
                    "High-Velocity Rounds"
                    | "Opening Rounds"
                    | "Monster Rounds"
                    | "Swift Striker"
                    | "Titanic Magazine"
                    | "Fleetfoot"
                    | "Spiritual Overflow"
                    | "Blood Tribute" => SlotType::Weapon,
                    "Quicksilver Reload"
                    | "Mercurial Magnum"
                    | "Boundless Spirit"
                    | "Transcendent Cooldown" => SlotType::Spirit,
                    _ => SlotType::Vitality,
                };
                scored.push(candidate);
            }
        }
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 3), (2, 6), (3, 2), (4, 8)], 7),
        );
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
        use crate::{AbilityStep, CoreLayoutIndex, MetaIndex};
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
            core_layouts: CoreLayoutIndex {
                by_hero: BTreeMap::from([(25, layout(&[(1, 1)], 0))]),
                overall: CoreLayoutStats::default(),
            },
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
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 1)], 0),
        );
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

    #[test]
    fn fills_each_cost_band_with_the_band_specific_score() {
        let mut cheap_mechanic = item(1, "Cheap Mechanic", 100.0, false, &[]);
        cheap_mechanic.item.tier = 1;
        cheap_mechanic.score.per_soul_value = 1.0;
        let mut cheap_lane = item(2, "Cheap Lane", 10.0, false, &[]);
        cheap_lane.item.tier = 1;
        cheap_lane.score.per_soul_value = 2.0;
        let mut expensive_low = item(3, "Expensive Low", 20.0, false, &[]);
        expensive_low.item.tier = 3;
        let mut expensive_high = item(4, "Expensive High", 30.0, false, &[]);
        expensive_high.item.tier = 3;
        let build = compose_build_with_layout(
            &hero(),
            &[cheap_mechanic, cheap_lane, expensive_low, expensive_high],
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 1), (3, 1)], 0),
        );
        let names = build
            .core
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["Expensive High", "Cheap Lane"]);
    }

    #[test]
    fn orders_core_by_lane_mid_core_and_late_phase() {
        let mut items = [
            (1, "Core", BuyPhase::Core),
            (2, "Lane", BuyPhase::Lane),
            (3, "Late", BuyPhase::Late),
            (4, "Mid", BuyPhase::Mid),
        ]
        .into_iter()
        .map(|(id, name, phase)| {
            let mut item = item(id, name, id as f64, false, &[]);
            item.item.tier = 1;
            item.buy_phase = phase;
            item
        })
        .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 4)], 0),
        );
        items.clear();
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Lane", "Mid", "Core", "Late"]
        );
    }

    #[test]
    fn enforces_four_base_slots_and_only_layout_flex_slots() {
        let scored = (1..=7)
            .map(|id| item(id, &format!("Weapon {id}"), id as f64, false, &[]))
            .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 7)], 2),
        );
        assert_eq!(build.core.len(), 6);
        assert_eq!(build.core.last().map(|item| item.item_id), Some(2));
        assert!(
            build
                .situations
                .iter()
                .flat_map(|block| block.items.iter())
                .any(|item| item.item_id == 1)
        );
    }
}
