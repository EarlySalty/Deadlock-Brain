use crate::mechanics;
use crate::{
    BuyPhase, Confidence, DamageType, HeroModel, ItemModel, ItemScore, MetaIndex, PatchDelta,
    ReasonerConfig, ReasonerError, Result, ScoredItem,
};

pub fn build_item_model(loaded: &ItemModel) -> Result<ItemModel> {
    if loaded.item_id == 0 || loaded.name.trim().is_empty() {
        return Err(ReasonerError::Data(
            "Geladenes Item-Modell enthält keine ID oder keinen Namen".to_string(),
        ));
    }
    Ok(loaded.clone())
}

pub fn score_item(
    item: &ItemModel,
    hero: &HeroModel,
    meta: &MetaIndex,
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> ScoredItem {
    if item.disabled || !item.shopable {
        return ScoredItem {
            item: item.clone(),
            score: ItemScore {
                combat_value: 0.0,
                per_slot_value: 0.0,
                per_soul_value: 0.0,
                purchase_bonus_value: 0.0,
                condition_factor: 0.0,
                active_value: 0.0,
                passive_value: 0.0,
                meta_support: 0.0,
                total: 0.0,
            },
            confidence: Confidence::Low,
            buy_phase: BuyPhase::Late,
            sources: Vec::new(),
        };
    }
    let condition_factor = mechanics::condition_factor_for_hero(item, hero, cfg);
    let active_value = mechanics::active_value(item, hero, cfg);
    let passive_value = mechanics::passive_value(item, hero, cfg);
    let combat_value = mechanics::combat_window_value(item, hero, cfg);
    let purchase_bonus_value = mechanics::purchase_bonus_value_with_config(item, hero, cfg);
    let per_slot_value = mechanics::per_slot_value(combat_value, purchase_bonus_value);
    let per_soul_value = mechanics::per_soul_value(combat_value, item.cost);
    let meta_support = meta_value(item.item_id, meta);
    let patch_support = patch_value(item, hero, deltas);
    let total = per_slot_value + meta_support + patch_support;
    let confidence = if meta.sample_ok.contains(&item.item_id) {
        Confidence::High
    } else {
        Confidence::Low
    };
    let mut sources = Vec::new();
    if combat_value > 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail: format!("Kampffenster {} s, Kanalanteil {}; Slot-Wert in Schaden oder effektivem Leben pro Sekunde. total nutzt den Slot-Wert; frühe Käufe nutzen per_soul_value.",cfg.combat_window_seconds,cfg.channel_uptime),
        });
    }
    if matches!(item.condition, crate::ConditionKind::StateBound { .. }) {
        sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic, detail:"Annahme für hit_rate: gleichverteiltes Restleben zwischen 0 und 100 Prozent, keine gemessene Trefferquote.".into() });
    }
    sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic,detail:"Annahmen zur Rotation: Melee aus Archetyp oder Ability-Klasse, Dash aus Mobility-Rolle. Kaufphase mit frühester Skalierungsstufe: Upgrade-Kosten 1/2/5, Ability-Unlocks auf Level 1/3/5/8; sonst Kosten-Fallback.".into() });
    if item.properties.contains_key("HealthStealPctHero")
        || item.passive_properties.contains_key("HealthStealPctHero")
    {
        sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic, detail:"Max-HP-Entzug zählt Verlust am Ziel und eigenen Lebensgewinn. Annahme: Ziel hat das Basisleben des Helden; keine Gegnerdaten vorhanden. Bewertung als Bruttowirkung der Procs im Fenster, ohne Rückgabe nach Debuff-Ende.".into() });
    }
    if meta_support != 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Meta,
            detail: "Meta als Nebensignal".to_string(),
        });
    }
    if patch_support != 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Patch,
            detail: "Patch-Delta".to_string(),
        });
    }
    ScoredItem {
        item: item.clone(),
        score: ItemScore {
            combat_value,
            per_slot_value,
            per_soul_value,
            purchase_bonus_value,
            condition_factor,
            active_value,
            passive_value,
            meta_support,
            total,
        },
        confidence,
        buy_phase: mechanics::buy_phase_for_hero(item, hero),
        sources,
    }
}

pub const CORE_SCORE_QUANTILE: f64 = 0.5;

pub fn core_score_threshold(scored: &[ScoredItem], cost: i64) -> Option<f64> {
    let mut values = scored
        .iter()
        .filter(|item| item.item.cost == cost && item.item.shopable && !item.item.disabled)
        .map(|item| item.score.per_slot_value)
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    if values.is_empty() {
        return None;
    }
    let position = (values.len() - 1) as f64 * CORE_SCORE_QUANTILE;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    Some(values[lower] + (values[upper] - values[lower]) * position.fract())
}

pub fn above_core_threshold(item: &ScoredItem, scored: &[ScoredItem]) -> bool {
    core_score_threshold(scored, item.item.cost)
        .is_some_and(|threshold| item.score.per_slot_value > threshold)
}

pub fn score_items(
    hero: &HeroModel,
    items: &[ItemModel],
    meta: &MetaIndex,
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> Vec<ScoredItem> {
    let mut scored = items
        .iter()
        .filter(|item| item.shopable && !item.disabled)
        .map(|item| score_item(item, hero, meta, deltas, cfg))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .score
            .total
            .total_cmp(&left.score.total)
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    scored
}

fn meta_value(item_id: i64, meta: &MetaIndex) -> f64 {
    let Some(support) = meta.by_item.get(&item_id) else {
        return 0.0;
    };
    let prevalence = if support.prevalence > 1.0 {
        (1.0 + support.prevalence).ln() / 10.0
    } else {
        support.prevalence.clamp(0.0, 1.0)
    };
    let winrate = support.winrate_pp.unwrap_or_default().abs().min(100.0) / 100.0;
    let lift = support.lift_pp.unwrap_or_default().abs().min(100.0) / 100.0;
    (prevalence * 0.15
        + winrate * 0.05
        + lift * 0.05
        + support.author_hits.max(0) as f64 * 0.1
        + support.claim_hits.max(0) as f64 * 0.02)
        .min(0.5)
}

fn patch_value(item: &ItemModel, hero: &HeroModel, deltas: &[PatchDelta]) -> f64 {
    deltas
        .iter()
        .filter_map(|delta| match delta.target {
            crate::DeltaTarget::Item(id) if id == item.item_id => {
                Some(delta.sign as f64 * delta.magnitude.abs().min(100.0) / 10.0)
            }
            crate::DeltaTarget::Hero(id) if id == hero.hero_id => {
                let axis_match = match hero.damage_plan.primary_axis {
                    DamageType::Weapon => delta.mechanic.contains("weapon"),
                    DamageType::Spirit => delta.mechanic.contains("spirit"),
                    DamageType::Hybrid | DamageType::None => true,
                };
                axis_match.then_some(delta.sign as f64 * delta.magnitude.abs() / 20.0)
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{AbilityModel, ConditionKind, DamagePlan, PurchaseBonuses, WeaponProfile};

    fn hero() -> HeroModel {
        HeroModel {
            hero_id: 25,
            name: "Warden".to_string(),
            archetype: "brawler".to_string(),
            base_health: 600.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: Vec::new(),
                weapon: Vec::new(),
                vitality: Vec::new(),
            },
            scaling: Vec::new(),
            weapon: WeaponProfile {
                bullet_damage: 10.0,
                shots_per_second: 5.0,
                clip_size: 20.0,
                reload_duration: 2.0,
                range: 0.0,
                falloff_start_range: 0.0,
                falloff_end_range: 0.0,
                sustained_dps: 0.0,
            },
            abilities: vec![AbilityModel {
                ability_id: 1,
                class_name: "ability1".to_string(),
                slot: 1,
                roles: Vec::new(),
                scaling: Vec::new(),
                channel_time: None,
                charges: 0,
                cooldown: 0.0,
                scaling_step: None,
                damage_type: DamageType::Weapon,
                base_effect: 0.0,
                tick_rate: None,
                duration: None,
            }],
            damage_plan: DamagePlan {
                weapon_dps: 40.0,
                spirit_dps: 10.0,
                weapon_share: 0.8,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    #[test]
    fn total_ranks_slots_independently_of_soul_efficiency() {
        let item = ItemModel {
            item_id: 1,
            name: "fixture".into(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 800,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: vec![],
            properties: [("WeaponDamage".into(), 10.0)].into_iter().collect(),
            passive_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        let expensive = ItemModel {
            item_id: 2,
            cost: 6400,
            ..item.clone()
        };
        let meta = MetaIndex {
            by_item: Default::default(),
            sample_ok: Default::default(),
        };
        let cfg = ReasonerConfig::default();
        let cheap = score_item(&item, &hero(), &meta, &[], &cfg);
        let costly = score_item(&expensive, &hero(), &meta, &[], &cfg);
        assert_eq!(cheap.score.total, costly.score.total);
        assert_eq!(
            cheap.score.per_soul_value,
            8.0 * costly.score.per_soul_value
        );
        assert_eq!(cheap.score.total, cheap.score.per_slot_value);
        let mut disabled = expensive.clone();
        disabled.disabled = true;
        assert_eq!(
            score_items(&hero(), &[item, disabled], &meta, &[], &cfg).len(),
            1
        );
        let values = [10.0, 20.0, 30.0, 40.0].map(|value| {
            let mut scored = cheap.clone();
            scored.score.per_slot_value = value;
            scored
        });
        assert_eq!(core_score_threshold(&values, 800), Some(25.0));
        assert!(!above_core_threshold(&values[1], &values));
        assert!(above_core_threshold(&values[2], &values));
    }

    #[test]
    fn build_item_model_validates_loaded_model_without_parsing_payload_again() {
        let item = ItemModel {
            item_id: 1,
            name: "Veil Walker".to_string(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: Vec::new(),
            properties: BTreeMap::new(),
            passive_properties: BTreeMap::new(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        assert_eq!(build_item_model(&item).unwrap(), item);
    }

    #[test]
    fn score_separates_active_passive_and_meta() {
        let item = ItemModel {
            item_id: 1,
            name: "x".to_string(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: true,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: Vec::new(),
            properties: [("WeaponDamage".to_string(), 10.0)].into_iter().collect(),
            passive_properties: [("PassiveHealth".to_string(), 100.0)].into_iter().collect(),
            condition: ConditionKind::ActiveCooldown {
                uptime: 0.5,
                cooldown: 20.0,
            },
            proc_cooldown: None,
            imbueable: false,
        };
        let meta = MetaIndex {
            by_item: [(
                1,
                crate::MetaSupport {
                    prevalence: 0.8,
                    winrate_pp: Some(2.0),
                    lift_pp: Some(1.0),
                    author_hits: 1,
                    claim_hits: 0,
                    avg_buy_time_relative: None,
                },
            )]
            .into_iter()
            .collect(),
            sample_ok: [1].into_iter().collect(),
        };
        let score = score_item(&item, &hero(), &meta, &[], &ReasonerConfig::default()).score;
        assert!(score.active_value > 0.0);
        assert!(score.passive_value > 0.0);
        assert_eq!(score.condition_factor, 0.5);
        assert!(score.meta_support < 0.5);
    }

    #[tokio::test]
    #[ignore = "benötigt echten Postgres-Snapshot über DEADLOCK_CENTRAL_DSN"]
    async fn scores_warden_reference_items_from_real_snapshot() {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").expect("DEADLOCK_CENTRAL_DSN fehlt");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET default_transaction_read_only = on")
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect(&dsn)
            .await
            .unwrap_or_else(|_| panic!("Verbindung für Warden-Echtdatenlauf fehlgeschlagen"));
        let ctx = crate::ReasonerCtx {
            pool,
            ai: None,
            config: ReasonerConfig::default(),
        };
        let hero = crate::hero::load_built_hero_model(&ctx, "Warden")
            .await
            .unwrap();
        assert!(
            hero.damage_plan.weapon_share > 0.6,
            "{:?}",
            hero.damage_plan
        );
        let willpower = hero
            .abilities
            .iter()
            .find(|ability| ability.class_name == "ability_warden_high_alert")
            .unwrap();
        let step = willpower
            .scaling_step
            .as_ref()
            .expect("Willpower-Skalierungsstufe fehlt");
        assert_eq!(step.upgrade_index, 2);
        assert_eq!((step.from, step.to), (0.8, 3.5));
        let items = crate::data::load_item_models(&ctx).await.unwrap();
        let meta = MetaIndex {
            by_item: BTreeMap::new(),
            sample_ok: std::collections::BTreeSet::new(),
        };
        let scored = score_items(&hero, &items, &meta, &[], &ctx.config);
        println!("WARDEN_MODEL {}", serde_json::to_string(&hero).unwrap());
        for (rank, item) in scored.iter().enumerate() {
            println!(
                "WARDEN_SCORE {} {}",
                rank + 1,
                serde_json::to_string(item).unwrap()
            );
        }
        let mut below = Vec::new();
        for name in [
            "Veil Walker",
            "Mercurial Magnum",
            "Siphon Bullets",
            "Quicksilver Reload",
        ] {
            let score = scored
                .iter()
                .find(|item| item.item.name == name)
                .unwrap_or_else(|| panic!("{name}"));
            let threshold =
                core_score_threshold(&scored, score.item.cost).expect("Kern-Schwelle fehlt");
            println!(
                "WARDEN_REFERENCE {name}: score={} threshold={threshold}",
                score.score.per_slot_value
            );
            if !above_core_threshold(score, &scored) {
                below.push(name);
            }
        }
        assert!(below.is_empty(), "Unter der Kern-Schwelle: {below:?}");
    }
}
