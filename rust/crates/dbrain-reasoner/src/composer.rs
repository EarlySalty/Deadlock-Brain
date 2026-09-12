use std::cmp::Ordering;

use crate::{
    BuildItem, BuildObject, Confidence, DamageType, Evidence, EvidenceKind, HeroModel, PatchDelta,
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

fn has_defense(item: &ScoredItem) -> bool {
    item.item.defense_kind.iter().any(|kind| {
        let kind = lower(kind);
        ["shield", "resist", "barrier", "armor", "defense"]
            .iter()
            .any(|needle| kind.contains(needle))
    })
}

fn is_shield(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("shield")
        || name.contains("reactive barrier")
        || item
            .item
            .defense_kind
            .iter()
            .any(|kind| lower(kind).contains("shield"))
}

fn is_can_buy_one(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    item.item.is_active
        && (has_defense(item)
            || [
                "metal skin",
                "dispel magic",
                "counterspell",
                "vampiric burst",
                "spirit resilience",
                "bullet resilience",
            ]
            .iter()
            .any(|needle| name.contains(needle)))
}

fn is_tryhard(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("slowing hex")
        || (item.item.is_active
            && matches!(item.item.damage_axis, DamageType::Spirit)
            && !has_defense(item))
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
        sell_priority: None,
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
        if is_can_buy_one(item) {
            can_buy.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_shield(item) {
            shields.push(build_item(item, hero, patch_sources(item, deltas)));
        } else if is_tryhard(item) {
            tryhard.push(build_item(item, hero, patch_sources(item, deltas)));
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
        rationale: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BuyPhase, DamagePlan, ItemModel, ItemScore, PurchaseBonuses, SlotType, WeaponProfile,
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
