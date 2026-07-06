use std::collections::BTreeSet;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedItem {
    pub defense_kind: Vec<String>,
    pub damage_axis: String,
}

pub fn classify_item(item: &Value) -> ClassifiedItem {
    let slot_type = item
        .get("item_slot_type")
        .and_then(Value::as_str)
        .or_else(|| item.get("slot_type").and_then(Value::as_str));
    let properties = item.get("properties").unwrap_or(&Value::Null);
    classify_properties(properties, slot_type)
}

pub fn classify_properties(properties: &Value, slot_type: Option<&str>) -> ClassifiedItem {
    let mut defense = BTreeSet::new();
    let mut has_weapon = false;
    let mut has_spirit = false;
    let mut has_headshot_bonus = false;

    let Value::Object(properties) = properties else {
        return ClassifiedItem {
            defense_kind: Vec::new(),
            damage_axis: "utility".to_string(),
        };
    };

    for (property_name, property) in properties {
        if property_name == "HeadShotBonusDamage" && is_active_property(property) {
            has_headshot_bonus = true;
        }

        let Some(property_type) = property
            .get("provided_property_type")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
        else {
            continue;
        };
        if !is_active_property(property) {
            continue;
        }

        if property_type.contains("ARMOR_DAMAGE_RESIST") || property_type.contains("TECH_RESIST") {
            defense.insert("percent_resist".to_string());
        }
        if property_type.contains("BARRIER_HEALTH") {
            defense.insert("flat_shield".to_string());
        }
        if property_type.contains("HEALTH_MAX") || property_type.contains("BASE_HEALTH_PERCENT") {
            defense.insert("flat_health".to_string());
        }
        if property_type.contains("HEALTH_REGEN") {
            defense.insert("regen".to_string());
        }
        if property_type.contains("TECH_POWER") {
            has_spirit = true;
        }
        if property_type.contains("WEAPON_POWER")
            || property_type.contains("WEAPON_DAMAGE_INCREASE")
        {
            has_weapon = true;
        }
    }

    // Live API exception: Headshot Booster exposes its weapon damage proc as a
    // stable property key without provided_property_type.
    if !has_weapon && !has_spirit && slot_type == Some("weapon") && has_headshot_bonus {
        has_weapon = true;
    }

    let damage_axis = match (has_weapon, has_spirit) {
        (true, true) => "hybrid",
        (true, false) => "weapon",
        (false, true) => "spirit",
        (false, false) => "utility",
    }
    .to_string();

    ClassifiedItem {
        defense_kind: defense.into_iter().collect(),
        damage_axis,
    }
}

fn is_active_property(property: &Value) -> bool {
    match property.get("value") {
        Some(Value::Null) | None => false,
        Some(Value::Number(number)) => number.as_f64().map(|value| value != 0.0).unwrap_or(true),
        Some(Value::String(text)) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return false;
            }
            trimmed
                .parse::<f64>()
                .map(|value| value != 0.0)
                .unwrap_or(true)
        }
        Some(Value::Bool(value)) => *value,
        Some(Value::Array(values)) => !values.is_empty(),
        Some(Value::Object(values)) => !values.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_fixture;

    fn fixture_item(name: &str) -> Value {
        let items = load_fixture("mo_item_catalog_subset.json");
        let Value::Array(items) = items else {
            panic!("fixture must be item array");
        };
        items
            .into_iter()
            .find(|item| item.get("name").and_then(Value::as_str) == Some(name))
            .unwrap_or_else(|| panic!("missing fixture item {name}"))
    }

    #[test]
    fn bullet_armor_is_resist_not_weapon_axis() {
        let classified = classify_item(&fixture_item("Bullet Armor"));
        assert_eq!(classified.defense_kind, vec!["percent_resist"]);
        assert_ne!(classified.damage_axis, "weapon");
    }

    #[test]
    fn weapon_shielding_is_hybrid_defense_not_weapon_axis() {
        let classified = classify_item(&fixture_item("Weapon Shielding"));
        assert!(classified.defense_kind.contains(&"flat_shield".to_string()));
        assert!(classified
            .defense_kind
            .contains(&"percent_resist".to_string()));
        assert_ne!(classified.damage_axis, "weapon");
    }

    #[test]
    fn headshot_booster_is_weapon_axis() {
        let classified = classify_item(&fixture_item("Headshot Booster"));
        assert_eq!(classified.damage_axis, "weapon");
    }

    #[test]
    fn spirit_armor_is_spirit_resist() {
        let classified = classify_item(&fixture_item("Spirit Armor"));
        assert_eq!(classified.defense_kind, vec!["percent_resist"]);
        assert_eq!(classified.damage_axis, "utility");
    }
}
