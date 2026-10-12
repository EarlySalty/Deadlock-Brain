use crate::{
    limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt,
    Rejection,
};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{json, value::RawValue, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
pub struct Assets;
impl Extractor for Assets {
    fn name(&self) -> &'static str {
        "assets"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == "assets" && matches!(path, "items/en.json" | "items/de.json")
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        match extract(raw) {
            Ok(set) if set.entities.is_empty() => Outcome::Ignored,
            Ok(set) => Outcome::Accepted(set),
            Err(error) => Outcome::Rejected(error),
        }
    }
}
#[derive(Debug)]
struct Leaf<'a> {
    pointer: String,
    lexeme: &'a str,
    text: String,
    numeric: bool,
    textual: bool,
}
impl Leaf<'_> {
    fn value_sha256(&self) -> [u8; 32] {
        let hex = brain_storage::entity_profile::derivation::sha256(self.lexeme.as_bytes());
        let digit = |b: u8| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
        let bytes = hex.as_bytes();
        std::array::from_fn(|i| digit(bytes[2 * i]) * 16 + digit(bytes[2 * i + 1]))
    }
    fn receipt(&self) -> Receipt {
        Receipt {
            locator_kind: "json_pointer".into(),
            locator: self.pointer.clone(),
            value_sha256: self.value_sha256(),
        }
    }
}
#[derive(Debug)]
struct Record<'a> {
    id: u64,
    class_name: String,
    kind: &'static str,
    leaves: Vec<Leaf<'a>>,
    prefix: String,
}
#[derive(Debug)]
struct Field<'record, 'raw> {
    leaf: &'record Leaf<'raw>,
    stat: String,
    condition: Value,
    unit: Option<&'static str>,
    alias_language: Option<&'static str>,
}
struct Members<'a>(Vec<(String, &'a RawValue)>);
impl<'de: 'a, 'a> Deserialize<'de> for Members<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MembersVisitor;
        impl<'de> Visitor<'de> for MembersVisitor {
            type Value = Members<'de>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an Assets object without colliding members")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut keys = BTreeSet::new();
                let mut members = Vec::new();
                while let Some((key, value)) = map.next_entry::<String, &RawValue>()? {
                    if !keys.insert(limits::normalize_key(&key)) {
                        return Err(serde::de::Error::custom("key_collision"));
                    }
                    members.push((key, value));
                }
                Ok(Members(members))
            }
        }
        deserializer.deserialize_map(MembersVisitor)
    }
}
fn leaves<'a>(raw: &'a RawValue, prefix: String) -> Result<Vec<Leaf<'a>>, &'static str> {
    let mut pending = vec![(prefix, raw)];
    let mut result = Vec::new();
    while let Some((pointer, raw)) = pending.pop() {
        let lexeme = raw.get();
        let mut leaf = Leaf {
            pointer: pointer.clone(),
            lexeme,
            text: String::new(),
            numeric: false,
            textual: false,
        };
        match lexeme.as_bytes().first() {
            Some(b'{') => {
                let members: Members<'a> = serde_json::from_str(lexeme).map_err(|error| {
                    if error.to_string().starts_with("key_collision") {
                        "key_collision"
                    } else {
                        "assets_json"
                    }
                })?;
                for (key, child) in members.0.into_iter().rev() {
                    let escaped = key.replace('~', "~0").replace('/', "~1");
                    pending.push((format!("{pointer}/{escaped}"), child));
                }
            }
            Some(b'[') => {
                let children: Vec<&'a RawValue> =
                    serde_json::from_str(lexeme).map_err(|_| "assets_json")?;
                for (position, child) in children.into_iter().enumerate().rev() {
                    pending.push((format!("{pointer}/{position}"), child));
                }
            }
            Some(b'"') => {
                leaf.text = serde_json::from_str(lexeme).map_err(|_| "assets_json")?;
                leaf.textual = true;
            }
            Some(b'-' | b'0'..=b'9') => {
                leaf.text = lexeme.to_owned();
                leaf.numeric = true;
            }
            _ => leaf.text = lexeme.to_owned(),
        }
        result.push(leaf);
    }
    Ok(result)
}
fn records(bytes: &[u8]) -> Result<Vec<Record<'_>>, &'static str> {
    let raw: &RawValue = serde_json::from_slice(bytes).map_err(|_| "assets_json")?;
    let array = raw.get().starts_with('[');
    let objects = if array {
        serde_json::from_str::<Vec<&RawValue>>(raw.get()).map_err(|_| "assets_array")?
    } else {
        vec![raw]
    };
    let mut result = Vec::new();
    let mut ids = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for (position, object) in objects.into_iter().enumerate() {
        if !object.get().starts_with('{') {
            return Err("assets_object");
        }
        let prefix = if array {
            format!("/{position}")
        } else {
            String::new()
        };
        let leaves = leaves(object, prefix.clone())?;
        let text = |pointer: &str| {
            leaves
                .iter()
                .find(|leaf| leaf.pointer == format!("{prefix}{pointer}") && leaf.textual)
                .map(|leaf| leaf.text.as_str())
        };
        let kind = match text("/type") {
            Some("ability") => "ability",
            Some("upgrade") => "item",
            _ => continue,
        };
        let id = leaves
            .iter()
            .find(|leaf| leaf.pointer == format!("{prefix}/id") && leaf.numeric)
            .ok_or("assets_identity")?
            .text
            .parse::<u64>()
            .map_err(|_| "assets_identity")?;
        let class_name = text("/class_name").ok_or("assets_identity")?.to_owned();
        if !ids.insert(id) || !classes.insert(limits::normalize_key(&class_name)) {
            return Err("key_collision");
        }
        result.push(Record {
            id,
            class_name,
            kind,
            leaves,
            prefix,
        });
    }
    Ok(result)
}
fn numeric(value: &str) -> bool {
    serde_json::from_str::<serde_json::Number>(value).is_ok()
}
impl<'a> Record<'a> {
    fn fields(&self, language: &'static str) -> Result<Vec<Field<'_, 'a>>, &'static str> {
        let prefix = &self.prefix;
        let index: BTreeMap<_, _> = self
            .leaves
            .iter()
            .map(|leaf| (leaf.pointer.as_str(), leaf))
            .collect();
        let mut result = Vec::new();
        for leaf in &self.leaves {
            let path = leaf.pointer.strip_prefix(prefix).ok_or("assets_pointer")?;
            let parts: Vec<_> = path.split('/').skip(1).collect();
            let is_number = leaf.numeric || (leaf.textual && numeric(&leaf.text));
            let (stat, mut condition) = match parts.as_slice() {
                ["id"] if leaf.numeric => ("identity.game_id".into(), json!({})),
                ["class_name"] => ("identity.key".into(), json!({})),
                ["name"] if leaf.textual => ("identity.name".into(), json!({"lang": language})),
                ["image"] | ["image_webp"] if leaf.textual => {
                    (format!("identity.{}", parts[0]), json!({}))
                }
                ["cost"] | ["item_tier"] if self.kind == "item" && leaf.numeric => (
                    format!("item.{}", if parts[0] == "cost" { "cost" } else { "tier" }),
                    json!({}),
                ),
                ["item_slot_type"] if self.kind == "item" && leaf.textual => {
                    ("item.slot".into(), json!({}))
                }
                ["activation"] if self.kind == "item" && leaf.textual => {
                    ("item.activation".into(), json!({}))
                }
                ["description", field] if leaf.textual => (
                    format!("{}.description_{}", self.kind, unescape(field)),
                    json!({"lang": language}),
                ),
                ["properties", field, "value"] if is_number => {
                    let mut condition = json!({});
                    let scale = format!("{prefix}/properties/{field}/scale_function");
                    for (source, target) in [
                        ("class_name", "scale_class"),
                        ("specific_stat_scale_type", "scale_type"),
                        ("stat_scale", "scale_value"),
                    ] {
                        if let Some(value) = index.get(format!("{scale}/{source}").as_str()) {
                            qualifier(&mut condition, target, value)?;
                        }
                    }
                    qualifiers(
                        &mut condition,
                        &index,
                        &format!("{scale}/scaling_stats/"),
                        "scale_stat",
                    )?;
                    (format!("{}.{}", self.kind, unescape(field)), condition)
                }
                ["upgrades", tier, "property_upgrades", entry, "bonus"]
                    if self.kind == "ability" && is_number =>
                {
                    for pointer in [
                        format!("{prefix}/upgrades"),
                        format!("{prefix}/upgrades/{tier}/property_upgrades"),
                    ] {
                        if index
                            .get(pointer.as_str())
                            .is_none_or(|leaf| !leaf.lexeme.starts_with('['))
                        {
                            return Err("assets_upgrade_type");
                        }
                    }
                    let tier = tier
                        .parse::<u64>()
                        .map_err(|_| "assets_upgrade_tier")?
                        .checked_add(1)
                        .ok_or("assets_upgrade_tier")?;
                    let pointer = &leaf.pointer;
                    let parent = pointer.strip_suffix("/bonus").ok_or("assets_pointer")?;
                    if index.contains_key(format!("{parent}/upgrade_type").as_str())
                        || index.contains_key(format!("{parent}/scale_stat_filter").as_str())
                    {
                        continue;
                    }
                    let name = index
                        .get(format!("{parent}/name").as_str())
                        .filter(|leaf| leaf.textual)
                        .ok_or("assets_upgrade_name")?;
                    let mut condition = json!({"upgrade_tier": tier, "operation": "delta"});
                    let scope = format!("{prefix}/upgrades/{}/property_upgrades/", parts[1]);
                    let normalized = limits::normalize_key(&name.text);
                    let mut same_names = Vec::new();
                    for (pointer, value) in &index {
                        if pointer.starts_with(&scope)
                            && pointer.ends_with("/name")
                            && value.textual
                            && limits::normalize_key(&value.text) == normalized
                        {
                            if value.text != name.text {
                                return Err("key_collision");
                            }
                            same_names.push(*pointer);
                        }
                    }
                    if same_names.len() > 1 {
                        condition["upgrade_entry"] =
                            json!(entry.parse::<u64>().map_err(|_| "assets_upgrade_entry")?);
                    }
                    (format!("ability.{}", name.text), condition)
                }
                _ => continue,
            };
            if let ["properties", property, "value"] = parts.as_slice() {
                qualifiers(
                    &mut condition,
                    &index,
                    &format!("{prefix}/properties/{property}/usage_flags/"),
                    "usage_flag",
                )?;
            }
            result.push(Field {
                leaf,
                unit: match stat.as_str() {
                    "item.cost" => Some("souls"),
                    "ability.AbilityCooldown"
                    | "ability.AbilityDuration"
                    | "ability.DebuffDuration"
                    | "ability.ReloadTime" => Some("s"),
                    _ => None,
                },
                alias_language: match parts.as_slice() {
                    ["class_name"] => Some("und"),
                    ["name"] if !leaf.text.is_empty() => Some(language),
                    _ => None,
                },
                stat,
                condition,
            });
        }
        Ok(result)
    }
    fn components(&self) -> Result<Vec<&Leaf<'a>>, &'static str> {
        if self.kind != "item" {
            return Ok(Vec::new());
        }
        let parent = format!("{}/component_items", self.prefix);
        if self
            .leaves
            .iter()
            .find(|leaf| leaf.pointer == parent)
            .is_some_and(|leaf| !leaf.lexeme.starts_with('['))
        {
            return Err("assets_component_type");
        }
        let scope = format!("{parent}/");
        let mut result = Vec::new();
        let mut seen = BTreeSet::new();
        for leaf in &self.leaves {
            let Some(position) = leaf.pointer.strip_prefix(&scope) else {
                continue;
            };
            if position.contains('/') {
                continue;
            }
            if !leaf.textual {
                return Err("assets_component_type");
            }
            let normalized = limits::normalize_key(&leaf.text);
            if normalized == limits::normalize_key(&self.class_name) {
                return Err("assets_component_cycle");
            }
            if !seen.insert(normalized) {
                return Err("key_collision");
            }
            result.push(leaf);
        }
        Ok(result)
    }
}
fn qualifier(condition: &mut Value, key: &str, leaf: &Leaf<'_>) -> Result<(), &'static str> {
    if !leaf.textual && !leaf.numeric {
        return Err("assets_qualifier_type");
    }
    condition[key] = Value::String(leaf.text.clone());
    condition[format!("{key}_locator")] = Value::String(leaf.pointer.clone());
    Ok(())
}
fn qualifiers(
    condition: &mut Value,
    index: &BTreeMap<&str, &Leaf<'_>>,
    scope: &str,
    label: &str,
) -> Result<(), &'static str> {
    for (pointer, value) in index {
        if let Some(position) = pointer.strip_prefix(scope) {
            if !position.contains('/') {
                qualifier(condition, &format!("{label}_{position}"), value)?;
            }
        }
    }
    Ok(())
}
fn unescape(part: &str) -> String {
    part.replace("~1", "/").replace("~0", "~")
}
fn rejection(code: &'static str) -> Rejection {
    Rejection::new(code, "Assets-Objekt verletzt den Datenvertrag")
}
fn extract(raw: &RawObject) -> Result<FactSet, Rejection> {
    let language = match raw.path.as_str() {
        "items/de.json" => "de",
        _ => "en",
    };
    let mut set = FactSet::default();
    let mut qualified = false;
    for record in records(&raw.bytes).map_err(rejection)? {
        let upgrades = format!("{}/upgrades/", record.prefix);
        qualified |= record.leaves.iter().any(|leaf| {
            leaf.pointer.starts_with(&upgrades)
                || leaf.pointer.ends_with("/scale_function")
                || (leaf.pointer.ends_with("/usage_flags") && leaf.lexeme != "[]")
        });
        let key = limits::normalize_key(&record.class_name);
        let fields = record.fields(language).map_err(rejection)?;
        let identity = fields
            .iter()
            .find(|field| field.stat == "identity.key")
            .ok_or_else(|| rejection("assets_identity"))?;
        set.entities.push(Entity {
            kind: record.kind.into(),
            key: key.clone(),
            receipt: identity.leaf.receipt(),
        });
        for field in fields {
            if let Some(lang) = field.alias_language {
                set.aliases.push(Alias {
                    entity_key: key.clone(),
                    alias: limits::normalize_alias(&field.leaf.text),
                    lang: lang.into(),
                    receipt: field.leaf.receipt(),
                });
                if field.stat == "identity.name" {
                    continue;
                }
            }
            if field.unit.is_some() || field.condition != json!({}) {
                qualified = true;
                continue;
            }
            let value = if field.stat == "identity.game_id" {
                FactValue::Text(record.id.to_string())
            } else if field.leaf.numeric || field.leaf.pointer.ends_with("/value") {
                let number = field
                    .leaf
                    .text
                    .parse()
                    .map_err(|_| rejection("invalid_number"))?;
                FactValue::Number(number)
            } else {
                FactValue::Text(field.leaf.text.clone())
            };
            set.facts.push(Fact {
                entity_key: key.clone(),
                stat_key: limits::normalize_key(&field.stat.replacen('.', "_", 1)),
                value,
                receipt: field.leaf.receipt(),
            });
        }
        for (position, leaf) in record
            .components()
            .map_err(rejection)?
            .into_iter()
            .enumerate()
        {
            set.facts.push(Fact {
                entity_key: key.clone(),
                stat_key: format!("component_item_{position}"),
                value: FactValue::Text(limits::normalize_key(&leaf.text)),
                receipt: leaf.receipt(),
            });
        }
    }
    limits::validate(&set)?;
    if qualified {
        return Err(Rejection::new("assets_qualified_unsupported", "Bedingungen oder Einheiten können im aktuellen Faktenvertrag nicht verlustfrei gespeichert werden"));
    }
    Ok(set)
}
#[cfg(test)]
#[path = "assets/tests.rs"]
mod tests;
