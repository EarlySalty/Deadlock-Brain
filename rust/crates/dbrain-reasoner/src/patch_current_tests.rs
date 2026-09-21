use super::*;
use crate::{
    ConditionKind, DamagePlan, DamageType, ItemModel, PurchaseBonuses, ScalingStat, SlotType,
    WeaponProfile,
};
use std::collections::{BTreeMap, BTreeSet};

fn field(value: f64, label: &str) -> crate::SnapshotField {
    crate::SnapshotField {
        value,
        fetched_at: Some(100.0),
        source: "fixture".into(),
        label: label.into(),
    }
}

fn hero() -> HeroModel {
    HeroModel {
        base_spirit_power: 0.0,
        standard_level_up_upgrades: BTreeMap::from([
            ("MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL".into(), 0.28),
        ]),
        standard_upgrade_levels: BTreeSet::new(),
        level_rewards: BTreeMap::new(),
        cost_bonuses: BTreeMap::new(),
        hero_id: 25,
        name: "Warden".into(),
        archetype: "brawler".into(),
        base_health: 600.0,
        level_curve: Vec::new(),
        purchase_bonuses: PurchaseBonuses { weapon: Vec::new(), spirit: Vec::new(), vitality: Vec::new() },
        scaling: vec![ScalingStat { stat: "EFireRate".into(), per_level: 0.0, per_spirit: Some(0.25) }],
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
        abilities: vec![
            serde_json::from_value(serde_json::json!({
                "ability_id":1,"class_name":"ability_warden_crowd_control","slot":1,"roles":["Damage","Control"],
                "scaling":[],"channel_time":null,"charges":1,"cooldown":12.0,"scaling_step":null,"damage_type":"Spirit",
                "base_effect":60.0,"properties":{"Damage":60.0,"ForwardVelocity":800.0,"AbilityCooldown":12.0},"upgrades":[]
            })).unwrap(),
            serde_json::from_value(serde_json::json!({
                "ability_id":2,"class_name":"ability_warden_high_alert","slot":2,"roles":["Sustain"],
                "scaling":[{"stat":"CombatBarrier","per_level":0.0,"per_spirit":0.8}],"channel_time":null,"charges":1,"cooldown":40.0,
                "scaling_step":{"upgrade_index":2,"stat":"CombatBarrier","from":0.8,"to":3.5},"damage_type":"Spirit","base_effect":0.0,
                "properties":{"CombatBarrier":100.0,"AbilityCooldown":40.0},
                "upgrades":[{"property_upgrades":[]},{"property_upgrades":[]},{"property_upgrades":[
                    {"name":"StatusResistancePercent","bonus":"40"},
                    {"name":"CombatBarrier","bonus":"2.7","upgrade_type":"EAddToScale","scale_stat_filter":"ETechPower"}
                ]}]
            })).unwrap(),
        ],
        damage_plan: DamagePlan { weapon_dps: 20.0, spirit_dps: 5.0, weapon_share: 0.8, primary_axis: DamageType::Weapon },
    }
}

fn event(entity: &str, raw_line: &str) -> Value {
    serde_json::json!({
        "entity_name": entity,
        "raw_line": raw_line,
        "posted_at_epoch": 200.0,
        "patch_external_id": "2026-09-16"
    })
}

#[test]
fn current_warden_patch_updates_level_scaling_and_ability_upgrades() {
    let mut hero = hero();
    let snapshots = vec![
        PatchSnapshot {
            target: DeltaTarget::Hero(25),
            name: "Warden".into(),
            fields: BTreeMap::from([
                (
                    "scaling.EFireRate".into(),
                    field(0.25, "Fire Rate Spirit Scaling"),
                ),
                (
                    "standard_level_up_upgrades.MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL"
                        .into(),
                    field(0.28, "Bullet Damage per Boon"),
                ),
            ]),
        },
        PatchSnapshot {
            target: DeltaTarget::Ability(1),
            name: "Alchemical Flask".into(),
            fields: BTreeMap::from([(
                "properties.ForwardVelocity".into(),
                field(800.0, "Projectile Speed"),
            )]),
        },
        PatchSnapshot {
            target: DeltaTarget::Ability(2),
            name: "Willpower".into(),
            fields: BTreeMap::from([
                (
                    "upgrade.2.CombatBarrier.bonus".into(),
                    field(2.7, "Spirit Scaling"),
                ),
                (
                    "upgrade.2.StatusResistancePercent.bonus".into(),
                    field(40.0, "Debuff Resistance"),
                ),
            ]),
        },
    ];
    let events = vec![
        event(
            "Warden",
            "Warden: Bullet damage per boon reduced from 0.28 to 0.25",
        ),
        event(
            "Warden",
            "Warden: Fire Rate spirit scaling reduced from 0.25 to 0.21",
        ),
        event(
            "Warden",
            "Warden: Alchemical Flask projectical range and speed reduced by 30%",
        ),
        event(
            "Warden",
            "Warden: Willpower T3 spirit scaling reduced from +2.7 to +2.1",
        ),
        event(
            "Warden",
            "Warden: Willpower T3 debuff resistance reduced from 40% to 30%",
        ),
    ];
    let mut deltas = compute_patch_delta_with_snapshots(&hero, &events, &snapshots);
    assert_eq!(deltas.len(), 5);
    assert!(
        deltas.iter().all(|delta| delta.application.is_some()),
        "{deltas:#?}"
    );
    apply_scored_patch_delta(
        &mut hero,
        &mut [],
        &mut deltas,
        &crate::MetaIndex {
            by_item: BTreeMap::new(),
            sample_ok: BTreeSet::new(),
        },
        &crate::ReasonerConfig::default(),
    );
    assert!(
        (hero.standard_level_up_upgrades["MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL"] - 0.25)
            .abs()
            < 1e-12
    );
    assert!(
        (hero
            .scaling
            .iter()
            .find(|stat| stat.stat == "EFireRate")
            .unwrap()
            .per_spirit
            .unwrap()
            - 0.21)
            .abs()
            < 1e-12
    );
    assert!((hero.abilities[0].properties["ForwardVelocity"] - 560.0).abs() < 1e-9);
    let t3 = hero.abilities[1].upgrades[2]["property_upgrades"]
        .as_array()
        .unwrap();
    assert!(
        (t3.iter().find(|p| p["name"] == "CombatBarrier").unwrap()["bonus"]
            .as_f64()
            .unwrap()
            - 2.1)
            .abs()
            < 1e-12
    );
    assert!(
        (t3.iter()
            .find(|p| p["name"] == "StatusResistancePercent")
            .unwrap()["bonus"]
            .as_f64()
            .unwrap()
            - 30.0)
            .abs()
            < 1e-12
    );
    assert!((hero.abilities[1].scaling_step.as_ref().unwrap().to - 2.9).abs() < 1e-12);
}

fn item(
    item_id: i64,
    name: &str,
    properties: BTreeMap<String, f64>,
    scaling: BTreeMap<String, f64>,
) -> ItemModel {
    ItemModel {
        property_spirit_scaling: scaling,
        property_damage_types: BTreeMap::new(),
        component_items: Vec::new(),
        class_name: String::new(),
        description: String::new(),
        item_id,
        name: name.into(),
        slot: SlotType::Spirit,
        tier: 4,
        cost: 6400,
        is_active: false,
        shopable: true,
        disabled: false,
        damage_axis: DamageType::Spirit,
        defense_kind: Vec::new(),
        passive_properties: properties.clone(),
        properties,
        conditional_properties: BTreeSet::new(),
        condition: ConditionKind::None,
        proc_cooldown: None,
        imbueable: false,
    }
}

#[test]
fn current_item_patch_updates_proc_stats_and_property_scaling() {
    let mut hero = hero();
    let mut items = vec![
        item(
            10,
            "Spiritual Overflow",
            BTreeMap::from([
                ("BonusSpirit".into(), 40.0),
                ("BonusFireRate".into(), 30.0),
                ("BuildUpPerShot".into(), 0.75),
            ]),
            BTreeMap::new(),
        ),
        item(
            20,
            "Mercurial Magnum",
            BTreeMap::from([("BulletsBonusMagicDamage".into(), 25.0)]),
            BTreeMap::from([("BulletsBonusMagicDamage".into(), 0.49)]),
        ),
    ];
    let snapshots = vec![
        PatchSnapshot {
            target: DeltaTarget::Item(10),
            name: "Spiritual Overflow".into(),
            fields: BTreeMap::from([
                ("properties.BonusSpirit".into(), field(40.0, "Spirit Power")),
                ("properties.BonusFireRate".into(), field(30.0, "Fire Rate")),
                (
                    "properties.BuildUpPerShot".into(),
                    field(0.75, "Buildup Per Shot"),
                ),
            ]),
        },
        PatchSnapshot {
            target: DeltaTarget::Item(20),
            name: "Mercurial Magnum".into(),
            fields: BTreeMap::from([
                (
                    "properties.BulletsBonusMagicDamage".into(),
                    field(25.0, "Base Bullet Damage"),
                ),
                (
                    "property_spirit_scaling.BulletsBonusMagicDamage".into(),
                    field(0.49, "Base Bullet Damage Scaling"),
                ),
            ]),
        },
    ];
    let events = vec![
        event(
            "Spiritual Overflow",
            "Spiritual Overflow: Buildup is 35% slower",
        ),
        event(
            "Spiritual Overflow",
            "Spiritual Overflow: Spirit Power on proc reduced from 40 to 30",
        ),
        event(
            "Spiritual Overflow",
            "Spiritual Overflow: Fire Rate reduced from 30% to 25%",
        ),
        event(
            "Mercurial Magnum",
            "Mercurial Magnum: Base bullet damage scaling reduced from 0.49 to 0.38",
        ),
        event(
            "Mercurial Magnum",
            "Mercurial Magnum: Base bullet damage reduced from 25% to 20%",
        ),
    ];
    let mut deltas = compute_patch_delta_with_snapshots(&hero, &events, &snapshots);
    assert_eq!(deltas.len(), 5);
    assert!(
        deltas.iter().all(|delta| delta.application.is_some()),
        "{deltas:#?}"
    );
    apply_scored_patch_delta(
        &mut hero,
        &mut items,
        &mut deltas,
        &crate::MetaIndex {
            by_item: BTreeMap::new(),
            sample_ok: BTreeSet::new(),
        },
        &crate::ReasonerConfig::default(),
    );
    let overflow = &items[0];
    assert!((overflow.properties["BonusSpirit"] - 30.0).abs() < 1e-9);
    assert!((overflow.properties["BonusFireRate"] - 25.0).abs() < 1e-9);
    assert!((overflow.properties["BuildUpPerShot"] - 0.4875).abs() < 1e-9);
    let magnum = &items[1];
    assert!((magnum.properties["BulletsBonusMagicDamage"] - 20.0).abs() < 1e-9);
    assert!((magnum.property_spirit_scaling["BulletsBonusMagicDamage"] - 0.38).abs() < 1e-12);
}
