fn q2_item(name: &str) -> ItemModel {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../testdata/warden-q2-original-6766.json")).unwrap();
    let payload = fixture["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == name)
        .unwrap();
    crate::item::build_item_model(&crate::data::item_model_from_payload(payload).unwrap()).unwrap()
}

fn q2_hero() -> HeroModel {
    let mut model = hero();
    model.cost_bonuses = ["weapon", "spirit", "vitality"]
        .into_iter()
        .map(|slot| {
            (
                slot.into(),
                vec![CostBonus {
                    gold_threshold: 0,
                    bonus: 0.0,
                }],
            )
        })
        .collect();
    model
}

fn q2_assert_eligible(hero: &HeroModel, item: &ItemModel, cfg: &ReasonerConfig) {
    let full = evaluate_inventory(hero, std::slice::from_ref(item), cfg);
    let fast = evaluate_inventory_fast(hero, std::slice::from_ref(item), cfg);
    assert_eq!(full.score, fast.score);
    assert_eq!(
        full.scenarios,
        fast.scenarios
            .iter()
            .cloned()
            .map(|mut scenario| {
                scenario.sequence = full
                    .scenarios
                    .iter()
                    .find(|s| s.name == scenario.name)
                    .unwrap()
                    .sequence
                    .clone();
                scenario
            })
            .collect::<Vec<_>>()
    );
    let excluded = crate::publish::unpublishable_items(hero, std::slice::from_ref(item), cfg);
    assert!(excluded.is_empty(), "{:?}", full.unknown_effects);
    assert!(full
        .assumptions
        .iter()
        .any(|value| value.contains(&format!("{}:", item.name))));
}

#[test]
fn q2_spirit_resilience_uses_own_health_threshold_and_typed_damage() {
    let item = q2_item("Spirit Resilience");
    assert_eq!(item.properties["TechResist"], 30.0);
    assert_eq!(item.properties["TechResistBelowThreshold"], 15.0);
    assert_eq!(item.properties["HealthThreshold"], 50.0);
    let mut renamed = item.clone();
    renamed.description.clear();
    renamed.name = "Neutral item".into();
    for (fraction, enabled) in [(0.499, true), (0.5, false), (0.501, false)] {
        assert_eq!(active(&renamed, 0.0, fraction, false, 0.0, 0.0), enabled);
    }
    let hero = q2_hero();
    let cfg = ReasonerConfig {
        combat_window_seconds: 5.0,
        incoming_pressure_dps: Some(140.0),
        incoming_weapon_share: 0.0,
        ..ReasonerConfig::default()
    };
    let mut base_only = item.clone();
    base_only.properties.remove("TechResistBelowThreshold");
    base_only
        .passive_properties
        .remove("TechResistBelowThreshold");
    base_only
        .conditional_properties
        .remove("TechResistBelowThreshold");
    let with = evaluate_inventory(&hero, std::slice::from_ref(&item), &cfg);
    let without = evaluate_inventory(&hero, &[base_only], &cfg);
    assert!(with.scenarios[1].remaining_health > without.scenarios[1].remaining_health);
    assert!(with.scenarios[1].item_condition_seconds[&item.item_id] > 0.0);
    assert!(with.scenarios[1].item_condition_seconds[&item.item_id] < 5.0);
    let mut stats = aggregate_stats(&hero, std::slice::from_ref(&item));
    assert!((stats.spirit_resist - 0.3).abs() < 1e-12);
    apply(&mut stats, "TechResistBelowThreshold", 15.0);
    assert!((stats.spirit_resist - 0.405).abs() < 1e-12);
    let bullet_only = ReasonerConfig {
        incoming_weapon_share: 1.0,
        ..cfg.clone()
    };
    assert_eq!(
        evaluate_inventory(&hero, std::slice::from_ref(&item), &bullet_only).scenarios[1]
            .incoming_health_damage,
        evaluate_inventory(&hero, &[], &bullet_only).scenarios[1].incoming_health_damage
    );
    q2_assert_eligible(&hero, &item, &cfg);
    for invalid in [-1.0, 101.0, f64::NAN] {
        let mut broken = item.clone();
        broken.properties.insert("HealthThreshold".into(), invalid);
        assert!(!crate::publish::unpublishable_items(&hero, &[broken], &cfg).is_empty());
    }
}

#[test]
fn q2_battle_vest_loses_buffs_below_own_threshold_but_retains_resistance() {
    let item = q2_item("Battle Vest");
    assert_eq!(item.properties["LifeThreshold"], 65.0);
    assert_eq!(item.properties["BaseAttackDamagePercent"], 18.0);
    assert_eq!(item.properties["BonusFireRate"], 7.0);
    let mut renamed = item.clone();
    renamed.description.clear();
    renamed.name = "Neutral item".into();
    for (fraction, enabled) in [(0.649, false), (0.65, false), (0.651, true), (1.0, true)] {
        assert_eq!(active(&renamed, 0.0, fraction, false, 0.0, 0.0), enabled);
    }
    let mut hero = q2_hero();
    hero.weapon.bullet_damage = 100.0;
    hero.weapon.shots_per_second = 1.0;
    hero.weapon.clip_size = 100.0;
    let cfg = ReasonerConfig {
        combat_window_seconds: 2.2,
        incoming_pressure_dps: Some(200.0),
        incoming_weapon_share: 1.0,
        ..ReasonerConfig::default()
    };
    let stats = aggregate_stats(&hero, std::slice::from_ref(&item));
    assert_eq!(stats.weapon, 0.0);
    assert_eq!(stats.rate, 0.0);
    assert!((stats.bullet_resist - 0.18).abs() < 1e-12);
    let result = evaluate_inventory(&hero, std::slice::from_ref(&item), &cfg);
    assert!((result.scenarios[0].weapon_damage - 354.0).abs() < 1e-9);
    assert!((result.scenarios[1].weapon_damage - 336.0).abs() < 1e-9);
    assert!((result.scenarios[1].incoming_health_damage - 360.8).abs() < 1e-9);
    assert!((result.scenarios[1].item_condition_seconds[&item.item_id] - 1.4).abs() < 1e-9);
    assert!((crate::mechanics::condition_factor_for_hero(&item, &hero, &cfg) - 0.35).abs() < 1e-12);
    q2_assert_eligible(&hero, &item, &cfg);
    let mut invalid = item;
    invalid.properties.remove("LifeThreshold");
    assert!(!crate::publish::unpublishable_items(&hero, &[invalid], &cfg).is_empty());
}

#[test]
fn q2_boundless_spirit_multiplies_flat_shop_and_buff_spirit_once() {
    let boundless = q2_item("Boundless Spirit");
    assert_eq!(boundless.properties["TechPower"], 30.0);
    assert_eq!(boundless.properties["TechPowerPercent"], 15.0);
    let mut hero = q2_hero();
    hero.base_spirit_power = 20.0;
    hero.cost_bonuses.insert(
        "spirit".into(),
        vec![CostBonus {
            gold_threshold: 0,
            bonus: 10.0,
        }],
    );
    hero.scaling.push(crate::ScalingStat {
        stat: "ERoundsPerSecond".into(),
        per_level: 0.0,
        per_spirit: Some(0.01),
    });
    let mut scaling = ability(1, 10.0, 20.0);
    scaling.scaling.push(crate::ScalingStat {
        stat: "Damage".into(),
        per_level: 0.0,
        per_spirit: Some(1.0),
    });
    hero.abilities = vec![scaling];
    let cfg = ReasonerConfig {
        combat_window_seconds: 1.0,
        ..ReasonerConfig::default()
    };
    let stats = aggregate_stats(&hero, std::slice::from_ref(&boundless));
    assert!((stats.spirit - 69.0).abs() < 1e-12);
    assert_eq!(stats.health, 75.0);
    assert_eq!(stats.regeneration, 0.0);
    let result = evaluate_inventory(&hero, std::slice::from_ref(&boundless), &cfg);
    assert!((result.scenarios[0].ability_damage - 79.0).abs() < 1e-9);
    let mut buff = item(2, "TechPower", 40.0);
    buff.conditional_properties.insert("TechPower".into());
    buff.condition = ConditionKind::ActiveCooldown {
        uptime: 1.0,
        cooldown: 10.0,
    };
    buff.properties.insert("AbilityCooldown".into(), 10.0);
    buff.properties.insert("AbilityDuration".into(), 1.0);
    let buffed = evaluate_inventory(&hero, &[boundless.clone(), buff], &cfg);
    assert!((buffed.scenarios[0].ability_damage - 125.0).abs() < 1e-9);
    let mut scenario = event_scenario(1.0);
    scenario.spirit = crate::SpiritInput::Total(100.0);
    let calculated = simulate_calculation(
        &hero,
        std::slice::from_ref(&boundless),
        &scenario,
        &hero.weapon_timing,
        &BTreeMap::new(),
    );
    assert!((calculated.ability_damage - 110.0).abs() < 1e-9);
    let scalar = crate::item::spirit_fire_rate_value(&boundless, &hero, &cfg);
    assert!((scalar.spirit_power - 37.5).abs() < 1e-12);
    q2_assert_eligible(&hero, &boundless, &cfg);
    let other = item(3, "TechPowerPercent", 10.0);
    assert!((aggregate_stats(&hero, &[boundless.clone(), other]).spirit - 75.0).abs() < 1e-9);
    assert_eq!(
        aggregate_stats(&hero, &[boundless.clone(), boundless]).spirit,
        stats.spirit
    );
}

fn q2_assert_excluded(name: &str, raw_key: &str) -> ItemModel {
    let item = q2_item(name);
    let hero = q2_hero();
    let cfg = ReasonerConfig::default();
    let result = evaluate_inventory(&hero, std::slice::from_ref(&item), &cfg);
    assert!(result
        .unknown_effects
        .iter()
        .any(|effect| effect.contains(raw_key)));
    assert_eq!(
        crate::publish::unpublishable_items(&hero, std::slice::from_ref(&item), &cfg).len(),
        1
    );
    assert!(result
        .assumptions
        .iter()
        .any(|value| value.contains(&format!("{raw_key}="))));
    item
}

#[test]
fn q2_monster_rounds_does_not_apply_npc_effects_to_hero_combat() {
    let item = q2_assert_excluded("Monster Rounds", "NonPlayerBonusWeaponPower");
    let stats = aggregate_stats(&q2_hero(), &[item]);
    assert_eq!(stats.weapon, 0.0);
    assert_eq!(stats.bullet_resist, 0.0);
    assert_eq!(stats.regeneration, 0.0);
}

#[test]
fn q2_veil_walker_does_not_invent_veil_healing_or_stealth() {
    let item = q2_assert_excluded("Veil Walker", "HealOnVeil");
    let stats = aggregate_stats(&q2_hero(), &[item]);
    assert_eq!(stats.health, 125.0);
    assert_eq!(stats.spirit, 6.0);
    assert_eq!(stats.regeneration, 0.0);
}

#[test]
fn q2_siphon_bullets_does_not_treat_temporary_hp_steal_as_permanent() {
    let item = q2_assert_excluded("Siphon Bullets", "HealthStealPctHero");
    let cfg = ReasonerConfig::default();
    let hero = q2_hero();
    let result = evaluate_inventory(&hero, std::slice::from_ref(&item), &cfg);
    assert_eq!(result.proc_damage, 0.0);
    assert_eq!(crate::mechanics::active_value(&item, &hero, &cfg), 0.0);
}

#[tokio::test]
#[ignore = "requires read-only original API mirror via DEADLOCK_CENTRAL_DSN"]
async fn q2_warden_dry_publish_reports_eligibility_from_original_mirror() {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").expect("DEADLOCK_CENTRAL_DSN is missing");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET default_transaction_read_only=on")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&dsn)
        .await
        .unwrap_or_else(|_| panic!("Read-only original API mirror is unavailable"));
    let ctx = crate::ReasonerCtx {
        pool,
        ai: None,
        config: ReasonerConfig {
            use_ai: false,
            ..ReasonerConfig::default()
        },
    };
    let models = crate::data::load_models_from_mirror(&ctx, "Warden")
        .await
        .unwrap();
    let excluded = crate::publish::unpublishable_items(&models.hero, &models.items, &ctx.config);
    for (name, eligible) in [
        ("Monster Rounds", false),
        ("Spirit Resilience", true),
        ("Veil Walker", false),
        ("Siphon Bullets", false),
        ("Boundless Spirit", true),
        ("Battle Vest", true),
    ] {
        let item = models.items.iter().find(|item| item.name == name).unwrap();
        let blocked = excluded
            .iter()
            .find(|excluded| excluded.item_id == item.item_id);
        let evaluation = evaluate_inventory(&models.hero, std::slice::from_ref(item), &ctx.config);
        let baseline = evaluate_inventory(&models.hero, &[], &ctx.config);
        let effects = crate::publish::blocking_unknown_effects(&evaluation, &baseline);
        assert_eq!(blocked.is_none(), eligible, "{name}: {effects:?}");
        assert!(!crate::mechanics::item_model_provenance(item).is_empty());
        println!(
            "Q2_ITEM {} {} eligible={} raw={:?} exclusion={:?}",
            item.item_id, item.name, eligible, item.properties, effects
        );
        for assumption in evaluation
            .assumptions
            .iter()
            .filter(|value| value.starts_with(&format!("{}:", item.name)))
        {
            println!("Q2_PROVENANCE {assumption}");
        }
    }
    let build = crate::reason_build_for_publish_with_options(
        &ctx,
        "Warden",
        crate::ReasonerOptions {
            seed_path: None,
            persist: false,
        },
    )
    .await
    .unwrap();
    crate::publish::validate_publish_current(&ctx.pool, &build)
        .await
        .unwrap();
    crate::publish::validate_publish_items(
        &ctx.pool,
        &crate::publish::publish_task_payload(&build),
    )
    .await
    .unwrap();
    let provenance = crate::publish::validate_build_provenance(&build).unwrap();
    for name in ["Monster Rounds", "Veil Walker", "Siphon Bullets"] {
        let id = models
            .items
            .iter()
            .find(|item| item.name == name)
            .unwrap()
            .item_id;
        assert!(!build
            .core
            .iter()
            .chain(build.situations.iter().flat_map(|block| &block.items))
            .any(|item| item.item_id == id));
    }
    println!(
        "Q2_DRY_PUBLISH_VALIDATED client_version={} source_run={} excluded={} steps={}",
        provenance.origin.client_version,
        provenance.origin.source_run_id,
        excluded.len(),
        provenance.purchase_plan.as_ref().unwrap().steps.len()
    );
}
