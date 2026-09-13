use crate::{DamageType, ItemModel};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AuraEffects {
    pub bullet_resist_reduction_percent: f64,
    pub fire_rate_slow_percent: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ItemInteraction {
    magazine: Option<(f64, f64)>,
    aura: Option<(f64, f64, f64)>,
    pub aura_radius_m: Option<f64>,
    missing_magazine_type: bool,
}

pub fn has_nearby_aura(item: &ItemModel) -> bool {
    [
        "SingleTargetPlayerMultiplier",
        "BulletArmorReduction",
        "FireRateSlow",
    ]
    .iter()
    .all(|key| {
        item.properties
            .get(*key)
            .is_some_and(|value| value.is_finite())
    }) && item.properties["SingleTargetPlayerMultiplier"] > 0.0
}

impl ItemInteraction {
    pub fn from_item(item: &ItemModel) -> Self {
        let value = |key: &str| {
            item.properties
                .get(key)
                .or_else(|| item.passive_properties.get(key))
                .copied()
                .filter(|value| value.is_finite())
        };
        let requested_magazine = item.imbueable && value("BulletsBonusMagicDamage").is_some();
        let typed = matches!(
            item.property_damage_types.get("BulletsBonusMagicDamage"),
            Some(DamageType::Spirit)
        );
        let magazine = (requested_magazine && typed).then(|| {
            (
                value("BulletsBonusMagicDamage").unwrap_or_default(),
                item.property_spirit_scaling
                    .get("BulletsBonusMagicDamage")
                    .copied()
                    .unwrap_or_default(),
            )
        });
        let aura = has_nearby_aura(item).then(|| {
            (
                value("BulletArmorReduction").unwrap_or_default().abs(),
                value("FireRateSlow").unwrap_or_default().abs(),
                value("SingleTargetPlayerMultiplier").unwrap_or(1.0),
            )
        });
        Self {
            magazine,
            aura,
            aura_radius_m: aura
                .and_then(|_| value("Radius"))
                .filter(|radius| *radius > 0.0),
            missing_magazine_type: requested_magazine && !typed,
        }
    }

    pub fn has_magazine_buff(&self) -> bool {
        self.magazine.is_some()
    }
    pub fn has_aura(&self) -> bool {
        self.aura.is_some()
    }

    pub fn magazine_spirit_damage(
        &self,
        base_bullet_damage: f64,
        spirit: f64,
        landed_bullets: f64,
        magazine_buff_active: bool,
    ) -> f64 {
        match self.magazine.filter(|_| magazine_buff_active) {
            Some((base_percent, scale)) => {
                base_bullet_damage * (base_percent + scale * spirit) / 100.0 * landed_bullets
            }
            None => 0.0,
        }
    }

    pub fn aura_effects(&self, nearby_heroes: usize, in_aura_range: bool) -> AuraEffects {
        match self.aura.filter(|_| in_aura_range && nearby_heroes > 0) {
            Some((armor, rate, single)) => {
                let multiplier = if nearby_heroes == 1 { single } else { 1.0 };
                AuraEffects {
                    bullet_resist_reduction_percent: armor * multiplier,
                    fire_rate_slow_percent: rate * multiplier,
                }
            }
            None => AuraEffects::default(),
        }
    }

    pub fn handles_property(&self, property: &str) -> bool {
        match property {
            "BulletsBonusMagicDamage" => self.has_magazine_buff(),
            "BulletArmorReduction" | "FireRateSlow" | "SingleTargetPlayerMultiplier" => {
                self.has_aura()
            }
            "Radius" => self.aura_radius_m.is_some(),
            _ => false,
        }
    }

    pub fn unknown_effects(&self) -> Vec<&'static str> {
        let mut unknown = Vec::new();
        if self.missing_magazine_type {
            unknown.push("Magazin-Spiritbonus hat keinen belegten Spirit-Schadenstyp; Zusatzschaden nicht berechnet.");
        }
        if self.has_aura() && self.aura_radius_m.is_none() {
            unknown.push("Aura-Radius fehlt; Nähe kann nicht aus Rohreichweite bestimmt werden.");
        }
        unknown
    }

    pub fn assumptions(&self) -> Vec<&'static str> {
        let mut assumptions = Vec::new();
        if self.has_magazine_buff() {
            assumptions.push("Magazinbonus: Prozent des Waffengrundschadens einschließlich Levelboni; keine erneute Multiplikation mit Waffenbonus. Annahme: Spirit wird je Treffer aktuell ausgewertet. Imbue-Auslösung und Ende beim nächsten Nachladen müssen aus dem Kampfzustand kommen.");
        }
        if self.has_aura() {
            assumptions.push("Aura: Nähe und Zahl naher gegnerischer Helden sind explizite Szenarioeingaben. Der Alleinzielmultiplikator gilt nur für diese Aura-Absenkung von Kugelresistenz und Feuerrate; keine Verdopplung von Leben oder fremden Effekten.");
        }
        assumptions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raw(name: &str) -> serde_json::Value {
        let rows: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/item-interactions-20260913.json"))
                .unwrap();
        rows.as_array()
            .unwrap()
            .iter()
            .find(|row| row["payload"]["name"] == name)
            .unwrap()["payload"]
            .clone()
    }
    fn model(raw: &serde_json::Value) -> ItemModel {
        let mut item: ItemModel = serde_json::from_value(serde_json::json!({"item_id":1,"name":"Rohgegenprobe","slot":"Weapon","tier":3,"cost":3200,"is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],"properties":{},"passive_properties":{},"condition":"ShotBound","proc_cooldown":null,"imbueable":true})).unwrap();
        for (key, property) in raw["properties"].as_object().unwrap() {
            if let Some(number) = property["value"]
                .as_str()
                .and_then(|text| text.trim_end_matches(['m', '%']).parse::<f64>().ok())
            {
                item.properties.insert(key.clone(), number);
            }
            if property["css_class"] == "tech_damage" {
                item.property_damage_types
                    .insert(key.clone(), DamageType::Spirit);
            }
            if let Some(scale) = property["scale_function"]["stat_scale"].as_f64() {
                item.property_spirit_scaling.insert(key.clone(), scale);
            }
        }
        item
    }
    #[test]
    fn actual_magnum_percentage_is_not_flat_damage_and_requires_buff() {
        let raw = raw("Mercurial Magnum");
        assert_eq!(raw["properties"]["BulletsBonusMagicDamage"]["postfix"], "%");
        assert_eq!(
            raw["properties"]["BulletsBonusMagicDamage"]["postvalue_label"],
            "Base Bullet Damage"
        );
        let item = model(&raw);
        let effect = ItemInteraction::from_item(&item);
        assert!(effect.has_magazine_buff());
        assert!((effect.magazine_spirit_damage(20.0, 100.0, 2.0, true) - 29.6).abs() < 1e-9);
        assert_eq!(effect.magazine_spirit_damage(20.0, 100.0, 2.0, false), 0.0);
        let mut unknown = item;
        unknown.property_damage_types.clear();
        let unsupported = ItemInteraction::from_item(&unknown);
        assert!(!unsupported.has_magazine_buff());
        assert_eq!(unsupported.unknown_effects().len(), 1);
    }
    #[test]
    fn actual_aura_only_doubles_two_debuffs_for_one_nearby_hero() {
        let item = model(&raw("Hunter's Aura"));
        assert!(matches!(
            crate::item::build_item_model(&item).unwrap().condition,
            crate::ConditionKind::None
        ));
        let effect = ItemInteraction::from_item(&item);
        assert_eq!(effect.aura_radius_m, Some(15.0));
        assert_eq!(
            effect.aura_effects(1, true),
            AuraEffects {
                bullet_resist_reduction_percent: 20.0,
                fire_rate_slow_percent: 30.0
            }
        );
        assert_eq!(
            effect.aura_effects(2, true),
            AuraEffects {
                bullet_resist_reduction_percent: 10.0,
                fire_rate_slow_percent: 15.0
            }
        );
        assert_eq!(effect.aura_effects(1, false), AuraEffects::default());
        assert_eq!(effect.aura_effects(0, true), AuraEffects::default());
        assert!(!effect.handles_property("BonusHealth"));
        assert!(!effect.handles_property("BaseAttackDamagePercent"));
        let mut missing_range = item;
        missing_range.properties.remove("Radius");
        assert_eq!(
            ItemInteraction::from_item(&missing_range)
                .unknown_effects()
                .len(),
            1
        );
    }
}
