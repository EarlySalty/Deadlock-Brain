use std::collections::{BTreeMap, BTreeSet};

use crate::{
    families::MechanicAxis, meta::MetaIndexWithSources, HeroModel, ItemModel, PopulationItem,
    PopulationPrior, ReasonerError, Result,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Playstyle {
    Weapon,
    Spirit,
    Tank,
}

impl Playstyle {
    pub fn parse(requested: Option<&str>) -> Result<Option<Self>> {
        match requested {
            None => Ok(None),
            Some("weapon") => Ok(Some(Self::Weapon)),
            Some("spirit") => Ok(Some(Self::Spirit)),
            Some("tank") => Ok(Some(Self::Tank)),
            Some(_) => Err(ReasonerError::Data("Unbekannter Spielstil.".into())),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Weapon => "Waffen",
            Self::Spirit => "Spirit",
            Self::Tank => "Tank",
        }
    }

    pub(crate) fn allows_item(self, hero: &HeroModel, item: &ItemModel) -> bool {
        self.allows_axes(&crate::families::item_axes(item), spirit_feeds_weapon(hero))
    }

    fn allows_axes(self, axes: &BTreeSet<MechanicAxis>, spirit_weapon: bool) -> bool {
        use MechanicAxis::{Defense, Spirit, Support, Sustain, Weapon};
        let primary = match self {
            Self::Weapon => axes.contains(&Weapon) || (axes.contains(&Spirit) && spirit_weapon),
            Self::Spirit => axes.contains(&Spirit),
            Self::Tank => {
                axes.contains(&Defense) || axes.contains(&Sustain) || axes.contains(&Support)
            }
        };
        primary || (!axes.contains(&Weapon) && !axes.contains(&Spirit))
    }

    pub(crate) fn condition(
        self,
        meta: &MetaIndexWithSources,
        hero: &HeroModel,
        items: &[ItemModel],
    ) -> MetaIndexWithSources {
        let allowed: BTreeSet<_> = items
            .iter()
            .filter(|item| self.allows_item(hero, item))
            .map(|item| item.item_id)
            .collect();
        let mut conditioned = meta.clone();
        conditioned.population = filter_population(&meta.population, &allowed);
        conditioned
    }
}

fn spirit_feeds_weapon(hero: &HeroModel) -> bool {
    let scaling = crate::mechanics::weapon_spirit_scaling(hero);
    [
        scaling.bullet_damage,
        scaling.clip_size,
        scaling.rounds_per_second,
    ]
    .into_iter()
    .any(|coefficient| coefficient.is_finite() && coefficient > 0.0)
}

fn filter_population(prior: &PopulationPrior, allowed: &BTreeSet<i64>) -> PopulationPrior {
    let ids: BTreeSet<_> = prior
        .ranked_by_prevalence()
        .into_iter()
        .chain(prior.staples())
        .chain(prior.positions().into_iter().map(|(id, _)| id))
        .filter(|id| allowed.contains(id))
        .collect();
    let imbues: BTreeMap<_, _> = ids
        .iter()
        .filter_map(|id| prior.imbue_target(*id).map(|target| (*id, target)))
        .collect();
    PopulationPrior::from_items(ids.into_iter().map(|id| PopulationItem {
        item_id: id,
        prevalence: prior.prevalence(id),
        median_position: prior.median_position(id),
        is_staple: prior.is_staple(id),
    }))
    .with_imbue_targets(imbues)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requested_style_is_explicit_and_unknown_styles_fail_closed() {
        assert_eq!(Playstyle::parse(None).unwrap(), None);
        assert_eq!(
            Playstyle::parse(Some("weapon")).unwrap(),
            Some(Playstyle::Weapon)
        );
        assert_eq!(
            Playstyle::parse(Some("spirit")).unwrap(),
            Some(Playstyle::Spirit)
        );
        assert_eq!(
            Playstyle::parse(Some("tank")).unwrap(),
            Some(Playstyle::Tank)
        );
        for value in ["", "unknown", " weapon", "Weapon"] {
            assert!(Playstyle::parse(Some(value)).is_err());
        }
    }

    #[test]
    fn offensive_axes_follow_the_requested_style_and_weapon_scaling() {
        use MechanicAxis::{Defense, Spirit, Support, Weapon};
        let weapon = BTreeSet::from([Weapon]);
        let spirit = BTreeSet::from([Spirit]);
        let defense = BTreeSet::from([Defense]);
        assert!(Playstyle::Weapon.allows_axes(&weapon, false));
        assert!(!Playstyle::Weapon.allows_axes(&spirit, false));
        assert!(Playstyle::Weapon.allows_axes(&spirit, true));
        assert!(Playstyle::Spirit.allows_axes(&spirit, false));
        assert!(!Playstyle::Spirit.allows_axes(&weapon, true));
        assert!(!Playstyle::Tank.allows_axes(&weapon, false));
        assert!(!Playstyle::Tank.allows_axes(&spirit, true));
        assert!(Playstyle::Tank.allows_axes(&BTreeSet::from([Support, Weapon]), false));
        for style in [Playstyle::Weapon, Playstyle::Spirit, Playstyle::Tank] {
            assert!(style.allows_axes(&defense, false));
            assert!(style.allows_axes(&BTreeSet::new(), false));
        }
    }

    fn hero() -> HeroModel {
        serde_json::from_value(serde_json::json!({
            "hero_id": 25, "name": "Warden", "archetype": "brawler",
            "base_health": 1000.0, "level_curve": [],
            "purchase_bonuses": {"spirit": [], "weapon": [], "vitality": []},
            "scaling": [], "abilities": [],
            "weapon": {"bullet_damage": 20.0, "shots_per_second": 2.0,
                "clip_size": 10.0, "reload_duration": 1.0, "range": 20.0,
                "falloff_start_range": 10.0, "falloff_end_range": 30.0,
                "sustained_dps": 33.333333333333336},
            "damage_plan": {"weapon_dps": 33.333333333333336,
                "spirit_dps": 0.0, "weapon_share": 1.0, "primary_axis": "Weapon"}
        }))
        .unwrap()
    }

    fn hero_with_scaling(stats: &[(&str, f64)]) -> HeroModel {
        let mut hero = hero();
        hero.scaling = stats
            .iter()
            .map(|(stat, value)| crate::ScalingStat {
                stat: (*stat).into(),
                per_level: 0.0,
                per_spirit: Some(*value),
            })
            .collect();
        hero
    }

    fn spirit_item() -> ItemModel {
        serde_json::from_value(serde_json::json!({
            "item_id": 1, "name": "Spirititem", "class_name": "spirit_item",
            "slot": "Spirit", "tier": 1, "cost": 800, "is_active": false,
            "shopable": true, "disabled": false, "damage_axis": "Spirit",
            "defense_kind": [], "properties": {"SpiritPower": 10.0},
            "passive_properties": {}, "condition": "None",
            "proc_cooldown": null, "imbueable": false
        }))
        .unwrap()
    }

    #[test]
    fn finite_direct_rate_overrides_positive_alias_for_weapon_style() {
        let item = spirit_item();
        assert_eq!(
            crate::families::item_axes(&item),
            BTreeSet::from([MechanicAxis::Spirit])
        );
        for direct in [0.0, -0.1] {
            for stats in [
                [("ERoundsPerSecond", direct), ("EFireRate", 50.0)],
                [("EFireRate", 50.0), ("ERoundsPerSecond", direct)],
            ] {
                let hero = hero_with_scaling(&stats);
                assert_eq!(
                    crate::mechanics::weapon_spirit_scaling(&hero).rounds_per_second,
                    direct
                );
                assert!(!spirit_feeds_weapon(&hero));
                assert!(!Playstyle::Weapon.allows_item(&hero, &item));
            }
        }
    }

    #[test]
    fn unknown_scaling_keys_do_not_admit_spirit_items_to_weapon_style() {
        let item = spirit_item();
        for stat in [
            "BulletDamage",
            "UnknownBulletDamageBonus",
            "bullet_damage",
            "UnknownClipSize",
            "UnknownFireRate",
            "rounds_per_second",
            "ERoundsPerSecondAlias",
            "eFireRate",
        ] {
            let hero = hero_with_scaling(&[(stat, 1.0)]);
            assert!(!spirit_feeds_weapon(&hero), "{stat}");
            assert!(!Playstyle::Weapon.allows_item(&hero, &item), "{stat}");
        }
    }

    #[test]
    fn only_finite_positive_effective_weapon_coefficients_admit_spirit_items() {
        let item = spirit_item();
        for stat in [
            "EBulletDamage",
            "EClipSize",
            "ERoundsPerSecond",
            "EFireRate",
        ] {
            let hero = hero_with_scaling(&[(stat, 0.01)]);
            assert!(spirit_feeds_weapon(&hero), "{stat}");
            assert!(Playstyle::Weapon.allows_item(&hero, &item), "{stat}");
            for value in [0.0, -0.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let hero = hero_with_scaling(&[(stat, value)]);
                assert!(!spirit_feeds_weapon(&hero), "{stat}: {value}");
                assert!(
                    !Playstyle::Weapon.allows_item(&hero, &item),
                    "{stat}: {value}"
                );
            }
        }
    }

    #[test]
    fn fire_rate_fallback_uses_the_effective_base_rate_and_recovers_nonfinite_direct_values() {
        let item = spirit_item();
        for base_rate in [0.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut hero = hero_with_scaling(&[("EFireRate", 50.0)]);
            hero.weapon.shots_per_second = base_rate;
            assert!(!spirit_feeds_weapon(&hero));
            assert!(!Playstyle::Weapon.allows_item(&hero, &item));
        }
        for direct in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let hero = hero_with_scaling(&[("ERoundsPerSecond", direct), ("EFireRate", 50.0)]);
            assert!(spirit_feeds_weapon(&hero));
            assert!(Playstyle::Weapon.allows_item(&hero, &item));
        }
    }

    #[test]
    fn direct_rounds_per_second_scaling_allows_spirit_on_weapon_builds() {
        let mut hero = hero();
        hero.scaling.push(crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: Some(0.01),
        });
        assert!(spirit_feeds_weapon(&hero));
        hero.scaling[0].per_spirit = Some(0.0);
        assert!(!spirit_feeds_weapon(&hero));
        hero.scaling[0].per_spirit = Some(f64::NAN);
        assert!(!spirit_feeds_weapon(&hero));
    }

    #[test]
    fn excluded_upgrade_components_remain_in_the_inventory_catalog() {
        let item = |id: i64, properties: serde_json::Value| -> ItemModel {
            serde_json::from_value(serde_json::json!({
                "item_id": id, "name": format!("Item {id}"),
                "class_name": format!("item_{id}"), "slot": "Weapon",
                "tier": 1, "cost": 800, "is_active": false, "shopable": true,
                "disabled": false, "damage_axis": "Weapon", "defense_kind": [],
                "properties": properties, "passive_properties": {},
                "condition": "None", "proc_cooldown": null, "imbueable": false
            }))
            .unwrap()
        };
        let parent = item(1, serde_json::json!({"BaseAttackDamagePercent": 20.0}));
        let mut upgrade = item(
            2,
            serde_json::json!({"BaseAttackDamagePercent": 30.0, "Health": 200.0}),
        );
        upgrade.component_items = vec![parent.class_name.clone()];
        upgrade.cost = 3200;
        let meta = MetaIndexWithSources {
            index: crate::meta::build_meta_index(&[], &[], &[], &crate::ReasonerConfig::default()),
            author_builds: Vec::new(),
            hero_ability_orders: BTreeMap::new(),
            core_layouts: Default::default(),
            combinations: BTreeMap::new(),
            population: PopulationPrior::default(),
            observations: Vec::new(),
            family: None,
        };
        let planned = crate::plan_build_with_playstyle(
            &hero(),
            &[parent, upgrade],
            &meta,
            &[],
            &[],
            &crate::ReasonerConfig::default(),
            Some("tank"),
        )
        .unwrap();
        assert!(planned.scored.iter().all(|scored| scored.item.item_id != 1));
        assert!(planned.scored.iter().any(|scored| scored.item.item_id == 2));
        assert!(planned.build.core.iter().all(|item| item.item_id != 1));
    }

    #[test]
    fn conditioning_preserves_evidence_without_promoting_population_staples() {
        let prior = PopulationPrior::from_items([
            PopulationItem {
                item_id: 1,
                prevalence: 0.8,
                median_position: Some(2.0),
                is_staple: true,
            },
            PopulationItem {
                item_id: 2,
                prevalence: 0.1,
                median_position: Some(9.0),
                is_staple: false,
            },
            PopulationItem {
                item_id: 3,
                prevalence: 0.9,
                median_position: Some(1.0),
                is_staple: true,
            },
        ])
        .with_imbue_targets(BTreeMap::from([(1, 10), (2, 20), (3, 30)]));
        let filtered = filter_population(&prior, &BTreeSet::from([1, 2]));
        assert_eq!(filtered.staples(), vec![1]);
        assert_eq!(filtered.prevalence(2), 0.1);
        assert_eq!(filtered.median_position(2), Some(9.0));
        assert_eq!(filtered.imbue_target(2), Some(20));
        assert!(!filtered.is_staple(2));
        assert_eq!(filtered.prevalence(3), 0.0);
        assert_eq!(filtered.imbue_target(3), None);
        assert_eq!(prior.staples(), vec![1, 3]);
    }
}
