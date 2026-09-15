use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::api::ApiClient;

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub slot_type: String,
    pub tier: i64,
    pub cost: i64,
    pub imbue: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    items: HashMap<i64, ItemInfo>,
    abilities: HashSet<i64>,
    ability_names: HashMap<i64, String>,
    heroes: HashMap<i64, String>,
}

impl Catalog {
    pub fn fetch(client: &ApiClient) -> Result<Self> {
        let items_payload = client.fetch_assets_items()?;
        let heroes_payload = client.fetch_assets_heroes()?;
        Self::from_payloads(&items_payload, &heroes_payload)
    }

    pub fn from_payloads(items_payload: &Value, heroes_payload: &Value) -> Result<Self> {
        let entries = items_payload
            .as_array()
            .ok_or_else(|| anyhow!("Item-Katalog ist kein JSON-Array."))?;
        let mut catalog = Catalog::default();
        for entry in entries {
            let Some(id) = value_i64(entry, "id") else {
                continue;
            };
            match value_str(entry, "type").as_deref() {
                Some("upgrade") => {
                    let Some(slot_type) = value_str(entry, "item_slot_type") else {
                        continue;
                    };
                    let info = ItemInfo {
                        name: value_str(entry, "name").unwrap_or_else(|| id.to_string()),
                        slot_type,
                        tier: value_i64(entry, "item_tier").unwrap_or(0),
                        cost: value_i64(entry, "cost").unwrap_or(0),
                        imbue: value_str(entry, "imbue"),
                    };
                    catalog.items.insert(id, info);
                }
                Some("ability") => {
                    catalog.abilities.insert(id);
                    if let Some(name) = value_str(entry, "name") {
                        catalog.ability_names.insert(id, name);
                    }
                }
                _ => {}
            }
        }
        if let Some(heroes) = heroes_payload.as_array() {
            for hero in heroes {
                if let (Some(id), Some(name)) = (value_i64(hero, "id"), value_str(hero, "name")) {
                    catalog.heroes.insert(id, name);
                }
            }
        }
        if catalog.items.is_empty() {
            return Err(anyhow!("Item-Katalog enthaelt keine Kaufobjekte."));
        }
        Ok(catalog)
    }

    pub fn is_purchase(&self, item_id: i64) -> bool {
        self.items.contains_key(&item_id)
    }

    pub fn is_ability(&self, item_id: i64) -> bool {
        self.abilities.contains(&item_id)
    }

    pub fn is_tier5(&self, item_id: i64) -> bool {
        self.items
            .get(&item_id)
            .map(|info| info.tier == 5)
            .unwrap_or(false)
    }

    pub fn imbueable(&self, item_id: i64) -> bool {
        self.items
            .get(&item_id)
            .map(|info| info.imbue.is_some())
            .unwrap_or(false)
    }

    pub fn slot_type(&self, item_id: i64) -> Option<&str> {
        self.items.get(&item_id).map(|info| info.slot_type.as_str())
    }

    pub fn cost(&self, item_id: i64) -> i64 {
        self.items.get(&item_id).map(|info| info.cost).unwrap_or(0)
    }

    pub fn item_name(&self, item_id: i64) -> String {
        self.items
            .get(&item_id)
            .map(|info| info.name.clone())
            .unwrap_or_else(|| item_id.to_string())
    }

    pub fn ability_name(&self, ability_id: i64) -> String {
        self.ability_names
            .get(&ability_id)
            .cloned()
            .unwrap_or_else(|| ability_id.to_string())
    }

    pub fn hero_name(&self, hero_id: i64) -> String {
        self.heroes
            .get(&hero_id)
            .cloned()
            .unwrap_or_else(|| hero_id.to_string())
    }

    pub fn resolve_hero(&self, needle: &str) -> Option<i64> {
        if let Ok(id) = needle.parse::<i64>() {
            if self.heroes.contains_key(&id) {
                return Some(id);
            }
        }
        let lowered = needle.to_lowercase();
        self.heroes
            .iter()
            .find(|(_, name)| name.to_lowercase() == lowered)
            .map(|(id, _)| *id)
    }
}

fn value_i64(value: &Value, key: &str) -> Option<i64> {
    value
        .get(key)
        .and_then(|inner| inner.as_i64().or_else(|| inner.as_f64().map(|f| f as i64)))
}

fn value_str(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .map(|text| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Catalog {
        let items = json!([
            {"id": 100, "type": "upgrade", "name": "Cheap Gun", "item_slot_type": "weapon", "item_tier": 1, "cost": 800},
            {"id": 200, "type": "upgrade", "name": "Big Spirit", "item_slot_type": "spirit", "item_tier": 3, "cost": 3200, "imbue": "imbue_active"},
            {"id": 300, "type": "upgrade", "name": "Endgame", "item_slot_type": "vitality", "item_tier": 5, "cost": 9999},
            {"id": 900, "type": "ability", "name": "ability_dash"}
        ]);
        let heroes = json!([{"id": 8, "name": "Warden"}]);
        Catalog::from_payloads(&items, &heroes).unwrap()
    }

    #[test]
    fn classifies_purchases_abilities_tier5_and_imbue() {
        let catalog = sample();
        assert!(catalog.is_purchase(100));
        assert!(!catalog.is_purchase(900));
        assert!(catalog.is_ability(900));
        assert!(catalog.is_tier5(300));
        assert!(!catalog.is_tier5(100));
        assert!(catalog.imbueable(200));
        assert!(!catalog.imbueable(100));
        assert_eq!(catalog.slot_type(200), Some("spirit"));
        assert_eq!(catalog.cost(200), 3200);
    }

    #[test]
    fn resolves_hero_by_name_and_id() {
        let catalog = sample();
        assert_eq!(catalog.resolve_hero("Warden"), Some(8));
        assert_eq!(catalog.resolve_hero("warden"), Some(8));
        assert_eq!(catalog.resolve_hero("8"), Some(8));
        assert_eq!(catalog.resolve_hero("Nobody"), None);
    }
}
