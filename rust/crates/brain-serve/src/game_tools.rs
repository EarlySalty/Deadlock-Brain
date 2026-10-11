use brain_contracts::{
    source::{origin_from_record, SourceRevision, SourceTimestamp},
    value::Observed,
    SourceRecordV2,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const GAME_FILES_SOURCE: &str = "deadlock-wiki-deadlock-data";
pub const CONVARS_LOGICAL_ID: &str = "game:1422450:data/json/convars.json";

pub type Result<T> = std::result::Result<T, &'static str>;

pub struct GameFiles {
    convars: Value,
    revision: String,
    retrieved_at: i64,
}

impl GameFiles {
    pub fn from_record(record: &SourceRecordV2) -> Result<Self> {
        if record.source_id != GAME_FILES_SOURCE
            || record.logical_id != CONVARS_LOGICAL_ID
            || record.tombstone
            || format!("{:x}", Sha256::digest(record.content.as_bytes())) != record.content_hash
        {
            return Err("game_files_record_invalid");
        }
        let origin = origin_from_record(record).map_err(|_| "game_files_origin_missing")?;
        let SourceRevision::Api {
            original_revision: Some(revision),
            ..
        } = origin.source_revision
        else {
            return Err("game_files_revision_missing");
        };
        let Observed::Known {
            value: SourceTimestamp::UnixSeconds(retrieved_at),
        } = origin.retrieved_at
        else {
            return Err("game_files_retrieval_missing");
        };
        if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("game_files_revision_invalid");
        }
        let convars = serde_json::from_str(&record.content).map_err(|_| "game_files_invalid")?;
        Ok(Self {
            convars,
            revision,
            retrieved_at,
        })
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }

    pub fn retrieved_at(&self) -> i64 {
        self.retrieved_at
    }

    fn number(&self, name: &str) -> Result<f64> {
        let value = &self.convars[name];
        value
            .get("value")
            .unwrap_or(value)
            .as_f64()
            .filter(|number| number.is_finite())
            .ok_or("game_files_value_missing")
    }

    fn flag(&self, name: &str) -> Result<bool> {
        let value = &self.convars[name];
        value
            .get("value")
            .unwrap_or(value)
            .as_bool()
            .ok_or("game_files_value_missing")
    }
}

fn clock(seconds: f64) -> String {
    let seconds = seconds.max(0.0).round() as u64;
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

fn entity<'a>(misc_entities: &'a Value, class_name: &str) -> Result<&'a Value> {
    let mut matching = misc_entities
        .as_array()
        .ok_or("mirror_entities_invalid")?
        .iter()
        .filter(|row| row["class_name"] == class_name);
    let row = matching.next().ok_or("mirror_entity_missing")?;
    if matching.next().is_some() {
        return Err("mirror_entity_ambiguous");
    }
    Ok(row)
}

fn seconds(row: &Value, field: &str) -> Result<f64> {
    row[field]
        .as_f64()
        .filter(|value| value.is_finite() && *value >= 0.0)
        .ok_or("mirror_value_missing")
}

fn objective(files: &GameFiles, prefix: &str, objective: &str, label: &str) -> Result<Value> {
    let name = |suffix: &str| format!("citadel_{prefix}_{suffix}");
    let enabled_flag = if prefix == "crate" {
        "spawn_enabled"
    } else {
        "enabled"
    };
    let first = files.number(&name("spawn_initial_delay"))?;
    let mut row = json!({"objective":objective,"label":label,"basis":"game_files",
        "enabled":files.flag(&name(enabled_flag))?,
        "first_spawn_seconds":first,"first_spawn_clock":clock(first),
        "respawn_interval_seconds":files.number(&name("respawn_interval"))?,
        "reward_souls_base":files.number(&name("reward_base"))?,
        "reward_souls_time_multiplier":files.number(&name("reward_time_multiplier"))?});
    let extra: &[(&str, &str)] = if prefix == "crate" {
        &[("client_notification_time", "announce_seconds_before")]
    } else {
        &[
            ("spawn_window", "spawn_window_seconds"),
            ("warning_time", "warning_seconds_before"),
            ("early_warning_time", "early_warning_seconds_before"),
        ]
    };
    for (suffix, field) in extra {
        row[*field] = json!(files.number(&name(suffix))?);
    }
    Ok(row)
}

pub fn timers(files: &GameFiles, misc_entities: &Value, generic_data: &Value) -> Result<Value> {
    let powerup = entity(misc_entities, "citadel_item_powerup_spawner")?;
    let breakable = entity(misc_entities, "citadel_breakable_item_container")?;
    let midboss = entity(misc_entities, "neutral_camp_midboss")?;
    let rejuv = &generic_data["rejuv_params"];
    let numbers = |field: &str| -> Result<Value> {
        rejuv[field]
            .as_array()
            .filter(|values| {
                !values.is_empty() && values.iter().all(|value| value.as_f64().is_some())
            })
            .map(|values| Value::Array(values.clone()))
            .ok_or("mirror_value_missing")
    };
    let powerup_first = seconds(powerup, "initial_spawn_time")?;
    let breakable_first = seconds(breakable, "initial_spawn_time")?;
    Ok(json!([
        objective(files, "crate", "urn", "Urn")?,
        objective(files, "koth", "unstable_rift", "Unstable Rift")?,
        {"objective":"powerup_camps","label":"Powerup","basis":"assets_mirror",
            "first_spawn_seconds":powerup_first,"first_spawn_clock":clock(powerup_first),
            "respawn_interval_seconds":seconds(powerup, "spawn_interval")?},
        {"objective":"breakable_containers","label":"Breakable Crates","basis":"assets_mirror",
            "first_spawn_seconds":breakable_first,"first_spawn_clock":clock(breakable_first),
            "respawn_seconds":seconds(breakable, "respawn_time")?,
            "powerup_drop_chance_percent":seconds(breakable, "powerup_drop_chance")?},
        {"objective":"mid_boss","label":"Mid-Boss","basis":"assets_mirror",
            "initial_spawn_delay_seconds":seconds(midboss, "initial_spawn_delay_seconds")?,
            "spawn_interval_seconds":seconds(midboss, "spawn_interval_in_seconds")?},
        {"objective":"rejuvenator","label":"Rejuvenator","basis":"assets_mirror",
            "buff_duration_seconds":seconds(rejuv, "rejuvinator_buff_duration")?,
            "drop_duration_seconds":seconds(rejuv, "rejuvinator_drop_duration")?,
            "rebirth_duration_seconds":numbers("rejuvinator_rebirth_duration")?,
            "player_respawn_multiplier":numbers("player_respawn_mult")?,
            "trooper_health_multiplier":numbers("trooper_health_mult")?},
    ]))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Threat {
    Healing,
    BulletResist,
    SpiritResist,
}

impl Threat {
    fn key(self) -> &'static str {
        match self {
            Self::Healing => "healing",
            Self::BulletResist => "bullet_resist",
            Self::SpiritResist => "spirit_resist",
        }
    }

    fn counter(self) -> &'static str {
        match self {
            Self::Healing => "incoming_healing_reduction",
            Self::BulletResist => "bullet_resist_reduction",
            Self::SpiritResist => "spirit_resist_reduction",
        }
    }

    fn of_target(name: &str, property: &Value, value: f64) -> Option<Self> {
        let css = property["css_class"].as_str().unwrap_or_default();
        if value <= 0.0 {
            return None;
        }
        if css == "healing" || name.contains("Lifesteal") || name.contains("HealthRegen") {
            Some(Self::Healing)
        } else if css == "bullet_armor_up" {
            Some(Self::BulletResist)
        } else if css == "tech_armor_up" {
            Some(Self::SpiritResist)
        } else {
            None
        }
    }

    fn of_counter(name: &str, property: &Value, value: f64) -> Option<Self> {
        let css = property["css_class"].as_str().unwrap_or_default();
        if matches!(
            name,
            "HealAmpReceivePenaltyPercent" | "HealAmpRegenPenaltyPercent"
        ) && value < 0.0
        {
            Some(Self::Healing)
        } else if css == "bullet_armor_down" && value != 0.0 {
            Some(Self::BulletResist)
        } else if css == "tech_armor_down" && value != 0.0 {
            Some(Self::SpiritResist)
        } else {
            None
        }
    }
}

pub struct Catalog<'a> {
    pub items_english: &'a Value,
    pub items_german: &'a Value,
    pub heroes_english: &'a Value,
    pub heroes_german: &'a Value,
}

pub enum CounterTarget {
    Item(i64),
    Hero(i64),
}

fn rows(value: &Value) -> Result<&Vec<Value>> {
    value.as_array().ok_or("mirror_entities_invalid")
}

fn by_id(value: &Value, id: i64) -> Result<Option<&Value>> {
    Ok(rows(value)?
        .iter()
        .find(|row| row["id"].as_i64() == Some(id)))
}

fn numeric(property: &Value) -> Option<f64> {
    let value = &property["value"];
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(|text| text.trim().parse().ok()))
        .filter(|number| number.is_finite() && *number != 0.0)
}

fn effect(catalog: &Catalog, id: i64, name: &str, property: &Value) -> Result<Value> {
    let german = by_id(catalog.items_german, id)?
        .map(|row| &row["properties"][name]["label"])
        .and_then(Value::as_str);
    Ok(json!({"property":name,"value":property["value"],
        "unit":property["postfix"].as_str().map(str::trim),
        "label":property["label"],"label_german":german}))
}

fn names(catalog: &Catalog, items: bool, id: i64) -> Result<Value> {
    let mut names = Map::new();
    let sources = if items {
        [
            ("english", catalog.items_english),
            ("german", catalog.items_german),
        ]
    } else {
        [
            ("english", catalog.heroes_english),
            ("german", catalog.heroes_german),
        ]
    };
    for (language, payload) in sources {
        if let Some(name) = by_id(payload, id)?.and_then(|row| row["name"].as_str()) {
            names.insert(language.into(), json!(name));
        }
    }
    Ok(Value::Object(names))
}

fn signature_owners(catalog: &Catalog) -> Result<BTreeMap<String, Vec<i64>>> {
    let mut owners = BTreeMap::<String, Vec<i64>>::new();
    for hero in rows(catalog.heroes_english)? {
        if hero["player_selectable"].as_bool() != Some(true)
            || hero["disabled"].as_bool() == Some(true)
        {
            continue;
        }
        let Some(id) = hero["id"].as_i64() else {
            continue;
        };
        for (slot, ability) in hero["items"].as_object().into_iter().flatten() {
            if let (true, Some(ability)) = (slot.starts_with("signature"), ability.as_str()) {
                owners.entry(ability.to_owned()).or_default().push(id);
            }
        }
    }
    Ok(owners)
}

fn target_rows<'a>(catalog: &'a Catalog, target: &CounterTarget) -> Result<Vec<&'a Value>> {
    match *target {
        CounterTarget::Item(id) => Ok(vec![
            by_id(catalog.items_english, id)?.ok_or("counter_target_missing")?
        ]),
        CounterTarget::Hero(id) => {
            let owners = signature_owners(catalog)?;
            Ok(rows(catalog.items_english)?
                .iter()
                .filter(|row| {
                    row["type"] == "ability"
                        && row["class_name"]
                            .as_str()
                            .and_then(|class| owners.get(class))
                            .is_some_and(|heroes| heroes.contains(&id))
                })
                .collect())
        }
    }
}

pub fn counters(
    catalog: &Catalog,
    target: &CounterTarget,
    allows_item: impl Fn(&Value) -> bool,
    limit: usize,
) -> Result<Value> {
    let mut target_effects = Vec::new();
    let mut threats = BTreeSet::new();
    let mut unmapped = Vec::new();
    for row in target_rows(catalog, target)? {
        let id = row["id"].as_i64().ok_or("mirror_entities_invalid")?;
        for (name, property) in row["properties"].as_object().into_iter().flatten() {
            let Some(value) = numeric(property) else {
                continue;
            };
            if property["label"].is_null() {
                continue;
            }
            let mut described = effect(catalog, id, name, property)?;
            if let CounterTarget::Hero(_) = target {
                described["ability"] = names(catalog, true, id)?;
            }
            match Threat::of_target(name, property, value) {
                Some(threat) => {
                    threats.insert(threat);
                    described["category"] = json!(threat.key());
                    target_effects.push(described);
                }
                None => unmapped.push(described),
            }
        }
    }
    if target_effects.is_empty() && unmapped.is_empty() {
        return Err("counter_target_without_values");
    }
    let owners = signature_owners(catalog)?;
    let mut categories = Vec::new();
    for threat in &threats {
        let mut items = Vec::new();
        let mut abilities = Vec::new();
        for row in rows(catalog.items_english)? {
            let is_item = allows_item(row);
            let heroes = row["class_name"]
                .as_str()
                .and_then(|class| owners.get(class))
                .filter(|_| row["type"] == "ability");
            if !is_item && heroes.is_none() {
                continue;
            }
            let id = row["id"].as_i64().ok_or("mirror_entities_invalid")?;
            let effects = row["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(name, property)| {
                    numeric(property).and_then(|value| Threat::of_counter(name, property, value))
                        == Some(*threat)
                })
                .map(|(name, property)| effect(catalog, id, name, property))
                .collect::<Result<Vec<_>>>()?;
            if effects.is_empty() {
                continue;
            }
            let mut entry = json!({"id":id,"names":names(catalog, true, id)?,"effects":effects});
            if is_item {
                entry["tier"] = row["item_tier"].clone();
                entry["cost"] = row["cost"].clone();
                entry["activation"] = row["activation"].clone();
                items.push(entry);
            } else if let Some(heroes) = heroes {
                entry["heroes"] = heroes
                    .iter()
                    .map(|hero| names(catalog, false, *hero))
                    .collect::<Result<Vec<_>>>()?
                    .into();
                abilities.push(entry);
            }
        }
        items.sort_by_key(|entry| {
            (
                entry["tier"].as_i64().unwrap_or(i64::MAX),
                entry["names"]["english"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            )
        });
        abilities.sort_by_key(|entry| {
            entry["names"]["english"]
                .as_str()
                .unwrap_or_default()
                .to_owned()
        });
        let (items_total, abilities_total) = (items.len(), abilities.len());
        items.truncate(limit);
        abilities.truncate(limit);
        categories.push(json!({"threat":threat.key(),"counter":threat.counter(),
            "items":items,"items_total":items_total,
            "abilities":abilities,"abilities_total":abilities_total}));
    }
    Ok(
        json!({"target_effects":target_effects,"unmapped_target_effects":unmapped,
        "counters":categories,
        "mapping":"Zuordnung nur über die Effektklassen der Spieldaten: Heilung und Lebensraub gegen negative eingehende Heilung, Kugelresistenz gegen Kugelresistenzreduktion, Geistresistenz gegen Geistresistenzreduktion. Effekte ohne Zuordnung haben keinen datengestützten Konter."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn property(value: &str, css: &str, label: &str) -> Value {
        json!({"value":value,"css_class":css,"label":label,"postfix":"\u{a0}%"})
    }

    #[test]
    fn counters_pair_target_effect_classes_with_real_counter_values() {
        let items = json!([
            {"id":1,"name":"Plated Armor","type":"upgrade","item_tier":4,"cost":6400,
                "properties":{"DeflectionPercent":property("30","bullet_armor_up","Deflection Percent"),
                    "BonusHealth":{"value":"130","css_class":"health","label":"Bonus Health"}}},
            {"id":2,"name":"Shredder","type":"upgrade","item_tier":2,"cost":1600,
                "properties":{"BulletResistReduction":property("-8","bullet_armor_down","Bullet Resist")}},
            {"id":3,"name":"Healbane","type":"upgrade","item_tier":2,"cost":1600,
                "properties":{"HealAmpReceivePenaltyPercent":property("-35","damage","Incoming Healing")}},
            {"id":4,"name":"Rocket Barrage","type":"ability","class_name":"ability_rockets",
                "properties":{"BulletArmorReduction":property("-10","bullet_armor_down","Bullet Resist")}}
        ]);
        let german = json!([{"id":1,"name":"Panzerrüstung",
            "properties":{"DeflectionPercent":{"label":"Ablenkungschance"}}}]);
        let heroes = json!([{"id":9,"name":"McGinnis","player_selectable":true,
            "items":{"signature1":"ability_rockets"}}]);
        let catalog = Catalog {
            items_english: &items,
            items_german: &german,
            heroes_english: &heroes,
            heroes_german: &json!([]),
        };
        let result = counters(
            &catalog,
            &CounterTarget::Item(1),
            |row| row["type"] == "upgrade",
            10,
        )
        .unwrap();
        assert_eq!(result["target_effects"][0]["category"], "bullet_resist");
        assert_eq!(result["target_effects"][0]["value"], "30");
        assert_eq!(
            result["target_effects"][0]["label_german"],
            "Ablenkungschance"
        );
        assert_eq!(
            result["unmapped_target_effects"][0]["property"],
            "BonusHealth"
        );
        let counters = result["counters"].as_array().unwrap();
        assert_eq!(counters.len(), 1);
        assert_eq!(counters[0]["items"][0]["names"]["english"], "Shredder");
        assert_eq!(counters[0]["items"][0]["effects"][0]["value"], "-8");
        assert_eq!(counters[0]["items_total"], 1);
        assert_eq!(
            counters[0]["abilities"][0]["heroes"][0]["english"],
            "McGinnis"
        );
    }
}
