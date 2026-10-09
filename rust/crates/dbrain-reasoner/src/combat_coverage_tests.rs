    #[test]
    fn actual_target_receiver_keeps_untyped_damage_independent_of_typed_modifiers() {
        let stats = InventoryStats {
            bullet_shred: 40.0,
            spirit_shred: 70.0,
            weapon_amp: 0.6,
            spirit_amp: 0.9,
            bullet_leech: 50.0,
            spirit_leech: 80.0,
            ..InventoryStats::default()
        };
        let mut scenario = explicit_test_scenario(3.0);
        scenario.target.health = 1000.0;
        scenario.target.regeneration = 5.0;
        scenario.target.shields = [30.0, 100.0, 200.0];
        let modifiers = crate::DamageModifiers {
            resist: 0.5,
            independent_resists: vec![0.25],
            point_shreds: vec![0.2],
            relative_reductions: vec![0.5],
            amplifications: vec![0.5],
            damage_reductions: vec![0.2],
        };
        for kind in [DamageType::None, DamageType::Hybrid] {
            let mut plain = SimulationTarget::new(scenario.target.health, Some(&scenario));
            let mut changed_scenario = scenario.clone();
            changed_scenario.target.bullet = modifiers.clone();
            changed_scenario.target.spirit = modifiers.clone();
            changed_scenario.target.changes = vec![crate::TargetChange {
                at_seconds: 1.0,
                bullet: crate::DamageModifiers {
                    resist: -0.5,
                    amplifications: vec![2.0],
                    damage_reductions: vec![0.9],
                    ..modifiers.clone()
                },
                spirit: crate::DamageModifiers {
                    resist: 0.9,
                    amplifications: vec![3.0],
                    damage_reductions: vec![0.7],
                    ..modifiers.clone()
                },
            }];
            let mut changed =
                SimulationTarget::new(scenario.target.health, Some(&changed_scenario));
            let mut events = Vec::new();
            let mut leech = 0.0;
            for (at, health) in [(0.0, 950.0), (1.2, 876.0), (2.0, 800.0)] {
                for target in [&mut plain, &mut changed] {
                    assert_eq!(
                        damage_event(
                            80.0,
                            &kind,
                            &stats,
                            1.0,
                            at,
                            &mut events,
                            &mut leech,
                            target
                        ),
                        80.0
                    );
                    assert!((target.health - health).abs() < 1e-9);
                    assert_eq!(target.shields.remaining, [0.0, 100.0, 200.0]);
                }
                assert!(events.is_empty());
                assert_eq!(leech, 0.0);
                println!("synthetic actual receiver kind={kind:?} at={at} health={health} damage=80 shields={:?}", changed.shields.remaining);
            }
        }
        let mut typed_scenario = scenario.clone();
        typed_scenario.target.bullet = modifiers.clone();
        typed_scenario.target.spirit = modifiers;
        for kind in [DamageType::Weapon, DamageType::Spirit] {
            let mut target = SimulationTarget::new(1000.0, Some(&typed_scenario));
            assert!((target.receive_damage(100.0, &kind, 0.0, 0.0, 0.0) - 94.5).abs() < 1e-9);
        }
        let mut generic = SimulationTarget::new(1000.0, Some(&scenario));
        assert_eq!(
            generic.receive_damage(100.0, &DamageType::None, 0.0, 0.0, 0.5),
            150.0
        );
        assert_eq!(generic.health, 880.0);
        assert_eq!(generic.shields.remaining, [0.0, 100.0, 200.0]);
        assert_eq!(
            generic.receive_damage(2000.0, &DamageType::None, 0.0, 0.0, 0.0),
            880.0
        );
        assert_eq!(generic.killed_at, Some(0.0));
        assert_eq!(
            generic.receive_damage(80.0, &DamageType::None, 2.0, 0.0, 0.0),
            0.0
        );
    }

    #[test]
    fn public_untyped_recorded_proc_ignores_timed_bullet_and_spirit_modifiers() {
        let raw: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap();
        assert_eq!(raw["provenance"]["client_version"], 6759);
        let payload = raw["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["id"] == 395867183)
            .unwrap();
        let mut proc = crate::data::item_model_from_payload(payload).unwrap();
        proc.property_damage_types.clear();
        proc.proc_cooldown = Some(0.4);
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.weapon.shots_per_second = 2.5;
        let mut scenario = explicit_test_scenario(1.0);
        scenario.use_abilities = false;
        scenario.target.shields = [30.0, 100.0, 200.0];
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        let anchor = std::time::Instant::now();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            std::time::Duration::from_secs(60),
            move || anchor,
        );
        let expected = simulate_calculation(
            &hero,
            std::slice::from_ref(&proc),
            &scenario,
            &timing,
            &innate,
        );
        assert_eq!(expected.shots, 3.0);
        assert_eq!(expected.proc_damage, 120.0);
        assert_eq!(expected.weapon_damage, 0.0);
        assert_eq!(expected.target_remaining_health, 1910.0);
        assert_eq!(expected.target_remaining_shields, [0.0, 100.0, 200.0]);
        scenario.target.bullet.resist = 0.8;
        scenario.target.spirit.resist = 0.5;
        scenario.target.spirit.amplifications = vec![2.0];
        scenario.target.changes = vec![crate::TargetChange {
            at_seconds: 0.4,
            bullet: crate::DamageModifiers {
                resist: -0.2,
                amplifications: vec![1.0],
                ..crate::DamageModifiers::default()
            },
            spirit: crate::DamageModifiers {
                resist: 0.9,
                amplifications: vec![3.0],
                ..crate::DamageModifiers::default()
            },
        }];
        for actual in [
            simulate_calculation(
                &hero,
                std::slice::from_ref(&proc),
                &scenario,
                &timing,
                &innate,
            ),
            simulate_calculation_with_deadline(
                &hero,
                std::slice::from_ref(&proc),
                &scenario,
                &timing,
                &innate,
                &deadline,
            )
            .unwrap(),
        ] {
            assert_eq!(actual, expected);
        }
        println!("recorded Mystic Shot version=6759; synthetic missing type/cooldown/hero/scenario: proc={} health={} shields={:?}", expected.proc_damage, expected.target_remaining_health, expected.target_remaining_shields);
    }

    #[test]
    fn stack_bonus_shred_matches_default_fast_and_binding_paths() {
        let cfg = ReasonerConfig {
            combat_window_seconds: 1.0,
            incoming_pressure_dps: Some(0.0),
            ..ReasonerConfig::default()
        };
        for (recorded, unshredded) in [(false, 60.0), (true, 30.6)] {
            let hero = stack_test_hero(recorded);
            for shred in [0.0, 20.0] {
                let items = [item(7, "BulletResistReduction", shred)];
                let refs = [&items[0]];
                let bindings = BTreeMap::new();
                let results = [
                    evaluate_inventory(&hero, &items, &cfg),
                    evaluate_inventory_fast(&hero, &items, &cfg),
                    evaluate_inventory_refs_fast(&hero, &refs, &cfg),
                    evaluate_inventory_with_bindings(&hero, &items, &cfg, &bindings),
                    evaluate_inventory_refs_fast_with_bindings(&hero, &refs, &cfg, &bindings),
                ];
                for (path, result) in results.iter().enumerate() {
                    let duel = &result.scenarios[0];
                    let expected = unshredded * (1.0 + shred / 100.0);
                    println!("stack recorded={recorded} path={path} shred={shred} shots={} damage={} expected={expected}", duel.shots, duel.weapon_damage);
                    assert_eq!(duel.shots, 3.0);
                    assert!((duel.weapon_damage - expected).abs() < 1e-9);
                    assert_eq!(result.score, results[0].score);
                    assert_eq!(result.scenarios.len(), results[0].scenarios.len());
                    for (actual, reference) in result.scenarios.iter().zip(&results[0].scenarios) {
                        let mut actual = actual.clone();
                        actual.sequence = reference.sequence.clone();
                        assert_eq!(&actual, reference);
                    }
                }
            }
        }
    }

    #[test]
    fn explicit_stack_shred_is_applied_once_with_target_resistance() {
        let hero = stack_test_hero(false);
        for (resistance, expected) in [(0.0, 72.0), (0.25, 57.0)] {
            let mut scenario = explicit_test_scenario(1.0);
            scenario.target.bullet.resist = resistance;
            let result = simulate_calculation(
                &hero,
                &[item(7, "BulletResistReduction", 20.0)],
                &scenario,
                &crate::WeaponTiming::default(),
                &BTreeMap::new(),
            );
            println!(
                "explicit resistance={resistance} shots={} damage={} expected={expected}",
                result.shots, result.weapon_damage
            );
            assert_eq!(result.shots, 3.0);
            assert!((result.weapon_damage - expected).abs() < 1e-9);
        }
    }

    #[test]
    fn duplicate_items_have_one_stat_shop_and_effect_population_in_public_simulations() {
        let mut hero = hero();
        hero.weapon.shots_per_second = 2.5;
        hero.cost_bonuses.insert(
            "weapon".into(),
            vec![
                CostBonus {
                    gold_threshold: 800,
                    bonus: 9.0,
                },
                CostBonus {
                    gold_threshold: 1600,
                    bonus: 12.0,
                },
            ],
        );
        let mut proc = item(7, "ProcBonusPhysicalDamage", 100.0);
        proc.condition = ConditionKind::ShotBound;
        proc.proc_cooldown = Some(1.0);
        proc.properties
            .extend([("ProcChance".into(), 100.0), ("WeaponPower".into(), 25.0)]);
        proc.property_damage_types
            .insert("ProcBonusPhysicalDamage".into(), DamageType::Weapon);
        let single = vec![proc.clone()];
        let duplicate = vec![proc.clone(), proc];
        let mut scenario = explicit_test_scenario(1.0);
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        let anchor = std::time::Instant::now();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            std::time::Duration::from_secs(60),
            move || anchor,
        );
        let worker = deadline.clone();
        assert_eq!(worker, deadline);
        assert_eq!(
            worker.expires_at(),
            anchor + std::time::Duration::from_secs(60)
        );
        let plain = simulate_calculation(&hero, &single, &scenario, &timing, &innate);
        for (path, result) in [
            simulate_calculation(&hero, &duplicate, &scenario, &timing, &innate),
            simulate_calculation_with_deadline(
                &hero, &duplicate, &scenario, &timing, &innate, &worker,
            )
            .unwrap(),
        ]
        .iter()
        .enumerate()
        {
            println!(
                "duplicate proc path={path} weapon={} proc={} shots={}",
                result.weapon_damage, result.proc_damage, result.shots
            );
            assert!((plain.weapon_damage - 40.2).abs() < 1e-9);
            assert_eq!(plain.proc_damage, 100.0);
            assert_eq!(result, &plain);
        }
        assert_eq!(
            aggregate_stats(&hero, &single),
            aggregate_stats(&hero, &duplicate)
        );
        assert_eq!(aggregate_stats(&hero, &duplicate).weapon, 34.0);
        assert_eq!(
            shop_bonuses(&hero, &[&duplicate[0], &duplicate[1]])["weapon"],
            9.0
        );
        let cfg = ReasonerConfig {
            combat_window_seconds: 1.0,
            ..ReasonerConfig::default()
        };
        assert_eq!(
            evaluate_inventory(&hero, &single, &cfg),
            evaluate_inventory(&hero, &duplicate, &cfg)
        );
        hero.cost_bonuses.clear();
        hero.weapon.shots_per_second = 10.0;
        hero.weapon.clip_size = 2.0;
        hero.abilities = vec![ability(11, 2.0, 100.0), ability(10, 1.0, 100.0)];
        let mut refill = item(8, "AmmoReloadPercent", 50.0);
        refill.imbueable = true;
        refill
            .properties
            .extend([("Damage".into(), 44.0), ("AbilityCooldown".into(), 30.0)]);
        refill
            .property_damage_types
            .insert("Damage".into(), DamageType::Spirit);
        scenario.window_seconds = 3.0;
        scenario.imbues.insert(8, 10);
        let plain = simulate_calculation(
            &hero,
            std::slice::from_ref(&refill),
            &scenario,
            &timing,
            &innate,
        );
        let duplicate = [refill.clone(), refill];
        for (path, result) in [
            simulate_calculation(&hero, &duplicate, &scenario, &timing, &innate),
            simulate_calculation_with_deadline(
                &hero, &duplicate, &scenario, &timing, &innate, &worker,
            )
            .unwrap(),
        ]
        .iter()
        .enumerate()
        {
            println!(
                "duplicate refill path={path} weapon={} proc={} shots={} activations={:?}",
                result.weapon_damage, result.proc_damage, result.shots, result.item_activations
            );
            assert_eq!(plain.shots, 5.0);
            assert_eq!(plain.proc_damage, 44.0);
            assert_eq!(plain.item_activations[&8], vec![0.2]);
            assert_eq!(result, &plain);
        }
        assert_eq!(worker.remaining(), Ok(std::time::Duration::from_secs(60)));
    }

    #[test]
    fn recorded_proc_duplicates_match_both_public_simulations_with_controlled_deadline() {
        let raw: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap();
        assert_eq!(raw["provenance"]["client_version"], 6759);
        let payload = raw["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["id"] == 395867183)
            .unwrap();
        let proc = crate::data::item_model_from_payload(payload).unwrap();
        assert_eq!(proc.condition, ConditionKind::ShotBound);
        let mut hero = hero();
        hero.weapon.shots_per_second = 2.5;
        let single = [proc.clone()];
        let duplicate = [proc.clone(), proc];
        let scenario = explicit_test_scenario(1.0);
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        let anchor = std::time::Instant::now();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            std::time::Duration::from_secs(60),
            move || anchor,
        );
        let plain = simulate_calculation(&hero, &single, &scenario, &timing, &innate);
        assert_eq!(plain.shots, 3.0);
        assert_eq!(plain.weapon_damage, 30.0);
        assert_eq!(plain.proc_damage, 40.0);
        assert_eq!(
            aggregate_stats(&hero, &single),
            aggregate_stats(&hero, &duplicate)
        );
        assert_eq!(
            shop_bonuses(&hero, &[&single[0]]),
            shop_bonuses(&hero, &[&duplicate[0], &duplicate[1]])
        );
        for items in [&single[..], &duplicate[..]] {
            assert_eq!(
                simulate_calculation(&hero, items, &scenario, &timing, &innate),
                plain
            );
            assert_eq!(
                simulate_calculation_with_deadline(
                    &hero, items, &scenario, &timing, &innate, &deadline,
                )
                .unwrap(),
                plain
            );
        }
        println!(
            "recorded Mystic Shot id=395867183 version=6759 synthetic hero/scenario weapon={} proc={} shots={} spirit_stat={}",
            plain.weapon_damage, plain.proc_damage, plain.shots, aggregate_stats(&hero, &single).spirit
        );
    }

    #[test]
    fn public_simulation_rejects_controlled_expiry_and_shared_cancellation() {
        use std::sync::{Arc, Mutex};
        use std::time::{Duration, Instant};

        let hero = hero();
        let items = [item(7, "WeaponPower", 25.0)];
        let scenario = explicit_test_scenario(1.0);
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        let clock = Arc::new(Mutex::new(Instant::now()));
        let anchor = *clock.lock().unwrap();
        let source = clock.clone();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            Duration::from_secs(60),
            move || *source.lock().unwrap(),
        );
        let worker = deadline.clone();
        let plain = simulate_calculation(&hero, &items, &scenario, &timing, &innate);
        let run = || {
            simulate_calculation_with_deadline(&hero, &items, &scenario, &timing, &innate, &worker)
        };
        *clock.lock().unwrap() = anchor + Duration::from_secs(59);
        assert_eq!(run().unwrap(), plain);
        assert_eq!(worker.remaining(), Ok(Duration::from_secs(1)));
        deadline.cancel();
        assert!(matches!(run(), Err(crate::ReasonerError::Data(_))));
        let source = clock.clone();
        let expires =
            brain_contracts::RequestDeadline::after_with_clock(Duration::from_secs(1), move || {
                *source.lock().unwrap()
            });
        assert_eq!(expires.expires_at(), worker.expires_at());
        assert_ne!(expires, worker);
        *clock.lock().unwrap() = anchor + Duration::from_secs(60);
        assert!(matches!(
            simulate_calculation_with_deadline(
                &hero, &items, &scenario, &timing, &innate, &expires,
            ),
            Err(crate::ReasonerError::Data(_))
        ));
        assert_eq!(worker.expires_at(), anchor + Duration::from_secs(60));
        println!(
            "controlled public simulation: success at 59s, shared cancellation, expiry at 60s"
        );
    }

    #[test]
    fn public_simulation_rejects_partial_work_on_controlled_deadline() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        };
        use std::time::{Duration, Instant};

        let mut hero = hero();
        hero.weapon.shots_per_second = 100.0;
        let scenario = explicit_test_scenario(1.0);
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        for cancel in [false, true] {
            let anchor = Instant::now();
            let reads = Arc::new(AtomicUsize::new(0));
            let source = reads.clone();
            let control: Arc<Mutex<Option<brain_contracts::RequestDeadline>>> =
                Arc::new(Mutex::new(None));
            let cancel_source = control.clone();
            let deadline = brain_contracts::RequestDeadline::after_with_clock(
                Duration::from_secs(60),
                move || {
                    let read = source.fetch_add(1, Ordering::SeqCst);
                    if read >= 4 {
                        if cancel {
                            cancel_source.lock().unwrap().as_ref().unwrap().cancel();
                        } else {
                            return anchor + Duration::from_secs(60);
                        }
                    }
                    anchor
                },
            );
            *control.lock().unwrap() = Some(deadline.clone());
            let result = simulate_calculation_with_deadline(
                &hero,
                &[],
                &scenario,
                &timing,
                &innate,
                &deadline.clone(),
            );
            assert!(matches!(result, Err(crate::ReasonerError::Data(_))));
            assert!(reads.load(Ordering::SeqCst) >= 6);
            assert_eq!(deadline.expires_at(), anchor + Duration::from_secs(60));
            println!(
                "controlled partial simulation cancel={cancel} clock_reads={}",
                reads.load(Ordering::SeqCst)
            );
            control.lock().unwrap().take();
        }
    }

    #[test]
    fn zero_length_magazine_cycles_never_requeue_at_the_same_timestamp() {
        let mut hero = hero();
        hero.weapon.clip_size = 1.0;
        hero.weapon.reload_duration = 0.0;
        let mut scenario = event_scenario(1.0);
        scenario.hit_fraction = 0.0;
        scenario.reload_convention = crate::ReloadConvention::AfterLastShot;
        let timing = crate::WeaponTiming::default();
        let innate = BTreeMap::new();
        let anchor = std::time::Instant::now();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            std::time::Duration::from_secs(60),
            move || anchor,
        );
        let result = simulate_calculation(&hero, &[], &scenario, &timing, &innate);
        assert_eq!(result.shots, 1.0);
        assert!(result.weapon_timing_unknown);
        assert_eq!(result.reloads, 1);
        assert_eq!(result.weapon_damage, 0.0);
        assert_eq!(result.target_remaining_health, scenario.target.health);
        assert_eq!(result.elapsed_seconds, scenario.window_seconds);
        assert_eq!(result.end_reason, CombatEndReason::WindowElapsed);
        assert_eq!(
            simulate_calculation_with_deadline(&hero, &[], &scenario, &timing, &innate, &deadline,)
                .unwrap(),
            result
        );
        assert_eq!(deadline.remaining(), Ok(std::time::Duration::from_secs(60)));
    }

    #[test]
    fn zero_reload_keeps_valid_firing_cycles_and_after_last_shot_boundaries() {
        for (clip, convention, reload, shots) in [
            (1.0, crate::ReloadConvention::AfterFireInterval, 0.0, 3.0),
            (2.0, crate::ReloadConvention::AfterLastShot, 0.0, 5.0),
            (1.0, crate::ReloadConvention::AfterLastShot, 0.025, 5.0),
        ] {
            let mut hero = hero();
            hero.weapon.clip_size = clip;
            hero.weapon.shots_per_second = 20.0;
            hero.weapon.reload_duration = reload;
            for hit in [0.0, 1.0] {
                let mut scenario = event_scenario(0.125);
                scenario.target.regeneration = 0.0;
                scenario.hit_fraction = hit;
                scenario.reload_convention = convention;
                let result = simulate_calculation(
                    &hero,
                    &[],
                    &scenario,
                    &crate::WeaponTiming::default(),
                    &BTreeMap::new(),
                );
                assert_eq!(result.shots, shots, "{clip} {convention:?} {reload} {hit}");
                assert!(!result.weapon_timing_unknown);
                assert!((result.weapon_damage - shots * 10.0 * hit).abs() < 1e-9);
                assert_eq!(result.elapsed_seconds, scenario.window_seconds);
            }
        }
    }

    #[test]
    fn invalid_reload_never_schedules_backwards_through_public_inventory_paths() {
        let cfg = ReasonerConfig {
            combat_window_seconds: 1.0,
            incoming_pressure_dps: Some(0.0),
            ..ReasonerConfig::default()
        };
        let bindings = BTreeMap::new();
        let anchor = std::time::Instant::now();
        let deadline = brain_contracts::RequestDeadline::after_with_clock(
            std::time::Duration::from_secs(60),
            move || anchor,
        );
        for reload in [-0.5, f64::NAN, f64::INFINITY] {
            let mut hero = hero();
            hero.weapon.clip_size = 2.0;
            hero.weapon.reload_duration = reload;
            for result in [
                evaluate_inventory(&hero, &[], &cfg),
                evaluate_inventory_fast(&hero, &[], &cfg),
                evaluate_inventory_refs_fast(&hero, &[], &cfg),
                evaluate_inventory_with_bindings(&hero, &[], &cfg, &bindings),
                evaluate_inventory_refs_fast_with_bindings(&hero, &[], &cfg, &bindings),
                evaluate_inventory_with_deadline(&hero, &[], &cfg, &bindings, Some(&deadline))
                    .unwrap(),
            ] {
                assert!(result.score.is_finite());
                assert!(!result.unknown_effects.is_empty());
                for scenario in result.scenarios {
                    assert!(scenario.weapon_timing_unknown);
                    assert_eq!(scenario.shots, 2.0);
                    assert_eq!(scenario.reloads, 1);
                    assert_eq!(scenario.elapsed_seconds, 1.0);
                    assert!(scenario.weapon_damage <= 20.0);
                }
            }
            for convention in [
                crate::ReloadConvention::AfterLastShot,
                crate::ReloadConvention::AfterFireInterval,
            ] {
                let mut scenario = event_scenario(1.0);
                scenario.hit_fraction = 0.0;
                scenario.reload_convention = convention;
                let timing = crate::WeaponTiming::default();
                let innate = BTreeMap::new();
                let result = simulate_calculation(&hero, &[], &scenario, &timing, &innate);
                assert_eq!(result.shots, 2.0);
                assert_eq!(result.weapon_damage, 0.0);
                assert_eq!(
                    simulate_calculation_with_deadline(
                        &hero,
                        &[],
                        &scenario,
                        &timing,
                        &innate,
                        &deadline,
                    )
                    .unwrap(),
                    result
                );
            }
        }
    }

    #[test]
    fn bonus_procs_require_certain_chance_for_shots_and_casts() {
        let cfg = ReasonerConfig {
            combat_window_seconds: 2.0,
            ..ReasonerConfig::default()
        };
        for (property, damage_type) in [
            ("ProcBonusMagicDamage", DamageType::Spirit),
            ("ProcBonusPhysicalDamage", DamageType::Weapon),
        ] {
            for condition in [ConditionKind::ShotBound, ConditionKind::None] {
                let mut hero = hero();
                if matches!(condition, ConditionKind::None) {
                    hero.weapon.bullet_damage = 0.0;
                    hero.abilities = vec![ability(1, 10.0, 1000.0)];
                }
                for chance in [None, Some(0.0), Some(50.0), Some(100.0)] {
                    let mut proc = item(1, property, 100.0);
                    proc.condition = condition.clone();
                    proc.proc_cooldown = Some(1.0);
                    proc.property_damage_types
                        .insert(property.into(), damage_type.clone());
                    if let Some(chance) = chance {
                        proc.properties.insert("ProcChance".into(), chance);
                    }
                    let full = evaluate_inventory(&hero, std::slice::from_ref(&proc), &cfg);
                    let certain = chance == Some(100.0);
                    assert!(full.scenarios.iter().all(|scenario| {
                        if certain {
                            scenario.proc_damage > 0.0
                        } else {
                            scenario.proc_damage == 0.0
                        }
                    }));
                    assert_eq!(
                        full.unknown_effects
                            .iter()
                            .any(|text| text.contains("deterministischer Bonus-Proc")),
                        !certain
                    );
                    assert_eq!(
                        full.score,
                        evaluate_inventory_fast(&hero, &[proc], &cfg).score
                    );
                }
            }
        }
    }
