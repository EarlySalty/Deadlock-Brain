#[test]
fn fix_e_numeric_direction_ratio_and_duplicate_labels() {
    let event = serde_json::json!({"entity_type":"hero","entity_name":"Warden","patch_external_id":"patch-1","old_value":"0.465","new_value":"0.49","change_type":"nerf","raw_line":"Base Bullet Damage spirit scaling increased from 0.465 to 0.49"});
    let mut duplicate = event.clone();
    duplicate["change_type"] = serde_json::json!("buff");
    let deltas = compute_patch_delta(&hero(), &[event, duplicate]);
    assert_eq!(deltas.len(), 1);
    assert_eq!(deltas[0].sign, 1);
    assert!((deltas[0].magnitude - (0.49 / 0.465 - 1.0)).abs() < 1e-12);
}

#[test]
fn fix_e_missing_snapshot_never_changes_current_model() {
    let mut current = hero();
    let before = current.clone();
    let deltas = compute_patch_delta(
        &current,
        &[
            serde_json::json!({"entity_type":"hero","entity_name":"Warden","old_value":10,"new_value":12,"change_type":"buff","raw_line":"Bullet damage increased from 10 to 12"}),
        ],
    );
    apply_patch_delta(&mut current, &mut [], &deltas);
    assert_eq!(current, before);
}
use super::*;
use crate::{DamagePlan, DamageType, PurchaseBonuses, WeaponProfile};

fn snapshot(target: DeltaTarget, field: &str, value: f64, fetched_at: f64) -> PatchSnapshot {
    PatchSnapshot {
        target,
        name: "Warden".into(),
        fields: BTreeMap::from([(
            field.into(),
            crate::SnapshotField {
                value,
                fetched_at: Some(fetched_at),
                source: "fixture".into(),
                label: String::new(),
            },
        )]),
    }
}

fn event(line: &str, posted_at: f64) -> Value {
    serde_json::json!({"entity_name":"Warden","raw_line":line,"posted_at_epoch":posted_at,"patch_external_id":"patch-1"})
}

#[test]
fn fix_e_snapshot_boundary_and_percent_factor() {
    let snapshot = snapshot(DeltaTarget::Hero(25), "weapon.bullet_damage", 10.0, 100.0);
    for (time, expected) in [(99.0, 10.0), (100.0, 10.0), (101.0, 12.0)] {
        let mut current = hero();
        let deltas = compute_patch_delta_with_snapshots(
            &current,
            &[event("Bullet damage increased by 20%", time)],
            std::slice::from_ref(&snapshot),
        );
        assert_eq!(deltas[0].application.is_some(), time > 100.0);
        apply_patch_delta(&mut current, &mut [], &deltas);
        assert!((current.weapon.bullet_damage - expected).abs() < 1e-10);
    }
}

#[test]
fn fix_e_uses_item_field_origin_and_never_changes_foreign_damage_fields() {
    let mut item: ItemModel = serde_json::from_value(serde_json::json!({"item_id":1,"name":"Test","slot":"Weapon","tier":2,"cost":1600,"is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],"properties":{"WeaponDamage":20.0,"BulletResist":30.0},"passive_properties":{"WeaponDamage":20.0},"condition":"None","proc_cooldown":10.0,"imbueable":false})).unwrap();
    let mut snapshot = snapshot(DeltaTarget::Item(1), "properties.WeaponDamage", 20.0, 100.0);
    snapshot.name = "Test".into();
    snapshot.fields.insert(
        "proc_cooldown".into(),
        crate::SnapshotField {
            value: 10.0,
            fetched_at: Some(200.0),
            source: "deadlock_data/item_card".into(),
            label: "Proc Cooldown".into(),
        },
    );
    let mut damage = event("WeaponDamage increased from 10 to 12", 150.0);
    damage["entity_name"] = serde_json::json!("Test");
    let mut cooldown = event("Proc Cooldown reduced from 10 to 8", 150.0);
    cooldown["entity_name"] = serde_json::json!("Test");
    let deltas = compute_patch_delta_with_snapshots(&hero(), &[damage, cooldown], &[snapshot]);
    assert_eq!(
        deltas
            .iter()
            .filter(|delta| delta.application.is_some())
            .count(),
        1
    );
    apply_patch_delta(&mut hero(), std::slice::from_mut(&mut item), &deltas);
    assert_eq!(item.properties["WeaponDamage"], 24.0);
    assert_eq!(item.passive_properties["WeaponDamage"], 24.0);
    assert_eq!(item.properties["BulletResist"], 30.0);
    assert_eq!(item.proc_cooldown, Some(10.0));
}

#[test]
fn fix_e_rejects_ambiguous_tier_lines_and_unbounded_factors() {
    let snapshot = snapshot(DeltaTarget::Hero(25), "weapon.bullet_damage", 10.0, 100.0);
    for line in [
        "Bullet damage per boon increased from 0.2 to 0.3",
        "Bullet damage T3 increased from 10 to 12",
        "Bullet damage increased from 0 to 12",
        "Bullet damage reduced from 10 to 0",
        "Bullet damage increased from 1 to 100",
        "Bullet damage increased from 1/2/3 to 2/3/4",
    ] {
        let mut current = hero();
        let before = current.clone();
        let deltas = compute_patch_delta_with_snapshots(
            &current,
            &[event(line, 101.0)],
            std::slice::from_ref(&snapshot),
        );
        assert!(deltas[0].application.is_none(), "{line}");
        assert!(deltas[0].note.contains("Nicht anwendbar"));
        apply_patch_delta(&mut current, &mut [], &deltas);
        assert_eq!(current, before);
    }
}

#[test]
fn fix_e_duplicate_source_versions_use_raw_numbers_and_earliest_time() {
    let snapshot = snapshot(DeltaTarget::Hero(25), "weapon.bullet_damage", 10.0, 100.0);
    let mut one = event("* Warden: Bullet damage increased from 10 to 12", 99.0);
    one["old_value"] = serde_json::json!(10);
    one["new_value"] = serde_json::json!(0);
    one["change_type"] = serde_json::json!("nerf");
    one["posted_at"] = serde_json::json!("2026-05-22T00:00:00Z");
    let mut two = one.clone();
    two["raw_line"] = serde_json::json!("- Warden: Bullet damage increased from 10 to 12");
    two["new_value"] = serde_json::json!(12);
    two["change_type"] = serde_json::json!("buff");
    two["posted_at_epoch"] = serde_json::json!(101.0);
    two["patch_external_id"] = serde_json::json!("another-source");
    let deltas = compute_patch_delta_with_snapshots(
        &hero(),
        &[one.clone(), two.clone()],
        std::slice::from_ref(&snapshot),
    );
    assert_eq!(deltas.len(), 1);
    assert_eq!(deltas[0].sign, 1);
    assert!((deltas[0].magnitude - 0.2).abs() < 1e-10);
    assert!(deltas[0].application.is_none());
    assert_eq!(
        deltas,
        compute_patch_delta_with_snapshots(&hero(), &[two, one], &[snapshot])
    );
}

#[test]
fn fix_e_cumulative_factor_is_bounded() {
    let snapshot = snapshot(DeltaTarget::Hero(25), "weapon.bullet_damage", 10.0, 100.0);
    let events = [
        event("Bullet damage increased from 10 to 15", 101.0),
        event("Bullet damage increased from 15 to 22.5", 102.0),
    ];
    let deltas = compute_patch_delta_with_snapshots(&hero(), &events, &[snapshot]);
    let mut current = hero();
    apply_patch_delta(&mut current, &mut [], &deltas);
    assert_eq!(current.weapon.bullet_damage, 15.0);
}

#[test]
fn fix_e_rejects_score_below_zero_even_with_plausible_field_ratio() {
    let mut current = hero();
    current.damage_plan = crate::hero::damage_plan(&current, &crate::ReasonerConfig::default());
    let mut item: ItemModel = serde_json::from_value(serde_json::json!({"item_id":1,"name":"Test","slot":"Weapon","tier":2,"cost":1600,"is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],"properties":{"WeaponDamage":100.0,"HealthDrainedPerSecond":15.0},"passive_properties":{},"condition":"None","proc_cooldown":null,"imbueable":false})).unwrap();
    let before = item.clone();
    let mut snapshot = snapshot(
        DeltaTarget::Item(1),
        "properties.WeaponDamage",
        100.0,
        100.0,
    );
    snapshot.name = "Test".into();
    let mut change = event("WeaponDamage reduced from 100 to 50", 101.0);
    change["entity_name"] = serde_json::json!("Test");
    let mut deltas = compute_patch_delta_with_snapshots(&current, &[change], &[snapshot]);
    let meta = crate::MetaIndex {
        by_item: BTreeMap::new(),
        sample_ok: Default::default(),
    };
    apply_scored_patch_delta(
        &mut current,
        std::slice::from_mut(&mut item),
        &mut deltas,
        &meta,
        &crate::ReasonerConfig::default(),
    );
    assert_eq!(item, before);
    assert!(deltas[0].application.is_none());
    assert!(deltas[0]
        .note
        .starts_with("Nicht anwendbar: Feld- oder Score-Faktor"));
}

fn hero() -> HeroModel {
    HeroModel {
        base_spirit_power: 0.0,
        standard_level_up_upgrades: Default::default(),
        standard_upgrade_levels: Default::default(),
        level_rewards: Default::default(),
        cost_bonuses: Default::default(),
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
            shots_per_second: 2.0,
            clip_size: 20.0,
            reload_duration: 2.0,
            range: 30.0,
            falloff_start_range: 20.0,
            falloff_end_range: 40.0,
            sustained_dps: 20.0,
        },
        abilities: Vec::new(),
        damage_plan: DamagePlan {
            weapon_dps: 20.0,
            spirit_dps: 5.0,
            weapon_share: 0.8,
            primary_axis: DamageType::Weapon,
        },
    }
}

#[test]
fn parses_real_warden_style_patch_lines() {
    let events = vec![
        serde_json::json!({
            "entity_type": "hero", "entity_name": "Warden", "change_type": "buff",
            "subject": "Willpower", "raw_line": "Willpower T3 increased from +2.5 spirit power scaling to +2.7",
            "old_value": "+2.5", "new_value": "+2.7", "normalized_line": "Willpower T3 +2.5 to +2.7"
        }),
        serde_json::json!({
            "entity_type": "hero", "entity_name": "Warden", "change_type": "nerf",
            "subject": "Bullet damage per boon", "raw_line": "Bullet damage per boon reduced from 0.34 to 0.28",
            "old_value": "0.34", "new_value": "0.28"
        }),
    ];
    let deltas = compute_patch_delta(&hero(), &events);
    assert_eq!(deltas.len(), 2);
    let scaling = deltas
        .iter()
        .find(|delta| delta.mechanic == "spirit_scaling")
        .unwrap();
    assert_eq!(scaling.sign, 1);
    assert!((scaling.magnitude - 0.08).abs() < 0.0001);
    let bullet = deltas
        .iter()
        .find(|delta| delta.mechanic == "bullet_damage")
        .unwrap();
    assert_eq!(bullet.sign, -1);
    assert!((bullet.magnitude - (1.0 - 0.28 / 0.34)).abs() < 0.0001);
}

#[test]
fn weapon_buffs_recompute_warden_damage_plan() {
    for (mechanic, field, base, sign, magnitude, expected) in [
        ("bullet_damage", "weapon.bullet_damage", 10.0, 1, 0.2, 20.0),
        (
            "fire_rate",
            "weapon.shots_per_second",
            2.0,
            1,
            0.5,
            300.0 / 13.0,
        ),
        (
            "reload",
            "weapon.reload_duration",
            2.0,
            -1,
            0.5,
            200.0 / 11.0,
        ),
    ] {
        let mut hero = hero();
        hero.weapon.sustained_dps = 200.0 / 12.0;
        hero.damage_plan = DamagePlan {
            weapon_dps: 200.0 / 12.0,
            spirit_dps: 12.0,
            weapon_share: 25.0 / 43.0,
            primary_axis: DamageType::Hybrid,
        };
        apply_patch_delta(
            &mut hero,
            &mut [],
            &[PatchDelta {
                target: DeltaTarget::Hero(25),
                mechanic: mechanic.to_string(),
                sign,
                magnitude,
                note: String::new(),
                application: Some(PatchApplication {
                    field: field.into(),
                    snapshot_value: base,
                    fetched_at: 1.0,
                    posted_at: 2.0,
                    source: "fixture".into(),
                }),
            }],
        );
        assert!(
            (hero.weapon.sustained_dps - expected).abs() < 1e-10,
            "{mechanic}: {} != {expected}",
            hero.weapon.sustained_dps
        );
        assert_eq!(hero.damage_plan.weapon_dps, hero.weapon.sustained_dps);
        assert_eq!(hero.damage_plan.weapon_share, 1.0);
        assert_eq!(hero.damage_plan.primary_axis, DamageType::Weapon);
    }
}

#[test]
fn applies_weapon_delta_without_estimating_a_value() {
    let mut hero = hero();
    let delta = PatchDelta {
        target: DeltaTarget::Hero(25),
        mechanic: "bullet_damage".to_string(),
        sign: 1,
        magnitude: 0.25,
        note: String::new(),
        application: Some(PatchApplication {
            field: "weapon.bullet_damage".into(),
            snapshot_value: 10.0,
            fetched_at: 1.0,
            posted_at: 2.0,
            source: "fixture".into(),
        }),
    };
    apply_patch_delta(&mut hero, &mut [], &[delta]);
    assert_eq!(hero.weapon.bullet_damage, 12.5);
}
