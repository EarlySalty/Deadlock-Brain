use crate::{
    limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt,
    Rejection,
};
use brain_storage::gamefacts::receipts::value_bytes;
use dbrain_sources::external::{parse_json_strict, MAX_SOURCE_BYTES};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub struct Hero;

const STATS: &[(&str, &str)] = &[
    ("MaxHealth", "max_health"),
    ("BaseHealthRegen", "base_health_regen"),
    ("MaxMoveSpeed", "max_move_speed"),
    ("SprintSpeed", "sprint_speed"),
    ("Stamina", "stamina"),
    ("LightMeleeDamage", "light_melee_damage"),
    ("HeavyMeleeDamage", "heavy_melee_damage"),
];
const WEAPON: &[(&str, &str)] = &[
    ("BulletDamage", "weapon_bullet_damage"),
    ("RoundsPerSecond", "weapon_rounds_per_second"),
    ("ClipSize", "weapon_clip_size"),
    ("ReloadTime", "weapon_reload_time"),
];

impl Extractor for Hero {
    fn name(&self) -> &'static str {
        "hero"
    }
    fn version(&self) -> &'static str {
        "3"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == "deadlock-data" && path == "data/json/hero-data.json"
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        match extract(raw) {
            Ok(set) => Outcome::Accepted(set),
            Err(rejection) => Outcome::Rejected(rejection),
        }
    }
}

fn malformed() -> Rejection {
    Rejection::new(
        "malformed_hero",
        "Unterstütztes Heldenfeld besitzt einen ungültigen Typ oder Wert",
    )
}

fn object(value: &Value) -> Result<&Map<String, Value>, Rejection> {
    value.as_object().ok_or_else(malformed)
}

fn unique_keys(value: &Value) -> Result<(), Rejection> {
    match value {
        Value::Object(fields) => {
            let mut keys = BTreeSet::new();
            for (key, value) in fields {
                if !keys.insert(limits::normalize_key(key)) {
                    return Err(Rejection::new(
                        "key_collision",
                        "Quellschlüssel kollidieren nach Normalisierung",
                    ));
                }
                unique_keys(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                unique_keys(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn pointer(parent: &str, key: &str) -> String {
    format!("{parent}/{}", key.replace('~', "~0").replace('/', "~1"))
}

fn receipt(raw: &RawObject, locator: String) -> Result<Receipt, Rejection> {
    let mut receipt = Receipt {
        locator_kind: "json_pointer".into(),
        locator,
        value_sha256: [0; 32],
    };
    receipt.value_sha256 = Sha256::digest(value_bytes(&raw.bytes, &receipt)?).into();
    Ok(receipt)
}

fn text(value: &Value) -> Result<FactValue, Rejection> {
    let value = value
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(malformed)?;
    Ok(FactValue::Text(value.to_owned()))
}

fn scalars(
    raw: &RawObject,
    key: &str,
    parent: &str,
    fields: &Map<String, Value>,
    mapping: &[(&str, &str)],
    set: &mut FactSet,
) -> Result<(), Rejection> {
    for (field, stat) in mapping {
        if let Some(value) = fields.get(*field) {
            let number = value.as_f64().ok_or_else(malformed)?;
            set.facts.push(Fact {
                entity_key: key.into(),
                stat_key: (*stat).into(),
                value: FactValue::Number(number),
                receipt: receipt(raw, pointer(parent, field))?,
            });
        }
    }
    Ok(())
}

fn aliases(key: &str, name: &str) -> BTreeSet<(String, &'static str)> {
    let name = limits::normalize_alias(name);
    let mut names = BTreeSet::from([name.clone()]);
    if let Some(rest) = name.strip_prefix("the ") {
        names.insert(rest.trim().to_owned());
    }
    if name.contains(" & ") {
        names.insert(name.replace(" & ", " and "));
        names.extend(
            name.split(" & ")
                .map(str::trim)
                .filter(|part| part.chars().count() >= 3)
                .map(str::to_owned),
        );
    }
    let mut result: BTreeSet<_> = names
        .into_iter()
        .filter(|alias| !alias.is_empty())
        .map(|alias| (alias, "en"))
        .collect();
    if let Some(internal) = key.strip_prefix("hero_").filter(|rest| !rest.is_empty()) {
        let internal = internal.replace('_', " ");
        if !result.iter().any(|(alias, _)| *alias == internal) {
            result.insert((internal, "und"));
        }
    }
    result
}

const SOURCE_TYPOS: &str = include_str!("../data/hero-source-typos.json");

fn source_typos() -> Result<BTreeMap<String, Vec<String>>, Rejection> {
    serde_json::from_str(SOURCE_TYPOS).map_err(|_| {
        Rejection::new(
            "malformed_typo_list",
            "Liste bekannter Schreibfehler für Heldennamen ist ungültig",
        )
    })
}

fn extract(raw: &RawObject) -> Result<FactSet, Rejection> {
    let typos = source_typos()?;
    if raw.bytes.len() > MAX_SOURCE_BYTES {
        return Err(Rejection::new(
            "value_too_large",
            "Heldenquelle überschreitet die bestehende Quellgrenze",
        ));
    }
    let value = parse_json_strict(&raw.bytes).map_err(|_| {
        Rejection::new(
            "malformed_json",
            "Heldenquelle enthält ungültiges JSON oder doppelte dekodierte Schlüssel",
        )
    })?;
    unique_keys(&value)?;
    let heroes = object(&value)?;
    let mut set = FactSet::default();
    for (upstream_key, hero) in heroes {
        let fields = object(hero)?;
        let parent = pointer("", upstream_key);
        let weapon = fields.get("Weapon").map(object).transpose()?;
        let name = fields.get("Name").ok_or_else(malformed)?;
        if name.is_null() && upstream_key == "hero_testhero" {
            continue;
        }
        let name_value = text(name)?;
        let key = limits::normalize_key(upstream_key);
        if !key.starts_with("hero_") {
            return Err(malformed());
        }
        let name_receipt = receipt(raw, pointer(&parent, "Name"))?;
        set.entities.push(Entity {
            kind: "hero".into(),
            key: key.clone(),
            receipt: receipt(raw, parent.clone())?,
        });
        let mut names = aliases(&key, name.as_str().ok_or_else(malformed)?);
        names.extend(
            typos
                .get(&key)
                .into_iter()
                .flatten()
                .map(|typo| limits::normalize_alias(typo))
                .filter(|alias| !alias.is_empty())
                .map(|alias| (alias, "en")),
        );
        for (alias, lang) in names {
            let receipt = if lang == "und" {
                receipt(raw, parent.clone())?
            } else {
                name_receipt.clone()
            };
            set.aliases.push(Alias {
                entity_key: key.clone(),
                alias,
                lang: lang.into(),
                receipt,
            });
        }
        set.facts.push(Fact {
            entity_key: key.clone(),
            stat_key: "name_en".into(),
            value: name_value,
            receipt: name_receipt,
        });
        if let Some(value) = fields.get("Type").filter(|v| !v.is_null()) {
            set.facts.push(Fact {
                entity_key: key.clone(),
                stat_key: "type".into(),
                value: text(value)?,
                receipt: receipt(raw, pointer(&parent, "Type"))?,
            });
        }
        scalars(raw, &key, &parent, fields, STATS, &mut set)?;
        if let Some(weapon) = weapon {
            scalars(
                raw,
                &key,
                &pointer(&parent, "Weapon"),
                weapon,
                WEAPON,
                &mut set,
            )?;
        }
        limits::validate(&set)?;
    }
    Ok(set)
}
