use crate::{
    limits, sources::InputObjects, Alias, Entity, Extractor, Fact, FactSet, FactValue, InputObject,
    Outcome, RawObject, Receipt, Rejection,
};
use brain_storage::gamefacts::receipts;
use dbrain_sources::external::parse_json_strict;
use serde_json::{value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};

pub const SOURCE: &str = "item-lineage";
pub const PATH: &str = "item-lineage.json";
pub const ENTITY_KIND: &str = "lineage";
pub const ENTITY_KEY: &str = "item_lineage";
const SCHEMA: &str = "item-lineage/v1";
const DATA: &[u8] = include_bytes!("../data/item-lineage.json");
const NAME_BYTES: usize = 128;
const FIELD_BYTES: usize = 1024;

pub struct ItemLineage;

pub fn load(_: &Path, _: bool) -> InputObjects {
    Box::new(std::iter::once_with(|| {
        Ok(InputObject {
            revision: format!("sha256:{:x}", Sha256::digest(DATA)),
            path: PATH.into(),
            bytes: DATA.to_vec(),
        })
    }))
}

impl Extractor for ItemLineage {
    fn name(&self) -> &'static str {
        "item_lineage"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == SOURCE && path == PATH
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        match extract(&raw.bytes)
            .and_then(|set| limits::validate(&set).map(|()| set))
            .and_then(|set| receipts::verify(&raw.bytes, &set).map(|()| set))
        {
            Ok(set) => Outcome::Accepted(set),
            Err(rejection) => Outcome::Rejected(rejection),
        }
    }
}

fn malformed() -> Rejection {
    Rejection::new(
        "malformed_item_lineage",
        "Item-Historie entspricht nicht dem Quellenvertrag",
    )
}

fn receipt(kind: &str, locator: String, value: &[u8]) -> Receipt {
    Receipt {
        locator_kind: kind.into(),
        locator,
        value_sha256: Sha256::digest(value).into(),
    }
}

fn text(entry: &Value, field: &str) -> Option<String> {
    entry[field]
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= FIELD_BYTES)
        .map(str::to_owned)
}

fn key(entry: &Value, field: &str) -> Result<Option<String>, Rejection> {
    match entry.get(field) {
        None => Ok(None),
        Some(value) => {
            let key = value.as_str().ok_or_else(malformed)?;
            let valid = key.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
                && key.len() <= limits::KEY_BYTES
                && key
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
            valid.then(|| Some(key.to_owned())).ok_or_else(malformed)
        }
    }
}

fn nullable_text(entry: &Value, field: &str) -> Result<(), Rejection> {
    match &entry[field] {
        Value::Null => Ok(()),
        Value::String(_) if text(entry, field).is_some() => Ok(()),
        _ => Err(malformed()),
    }
}

fn link(entry: &Value, field: &str) -> Result<(), Rejection> {
    nullable_text(entry, field)?;
    match entry[field].as_str() {
        Some(url) if !url.starts_with("https://") || url.contains(char::is_whitespace) => {
            Err(malformed())
        }
        _ => Ok(()),
    }
}

fn date(entry: &Value) -> Result<(), Rejection> {
    match &entry["patch_date"] {
        Value::Null => Ok(()),
        Value::String(day) if chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d").is_ok() => Ok(()),
        _ => Err(malformed()),
    }
}

struct Checked {
    item: String,
    former: Option<String>,
}

fn check(entry: &Value) -> Result<Checked, Rejection> {
    let fields = entry.as_object().ok_or_else(malformed)?;
    let allowed = [
        "item",
        "event",
        "former_name",
        "lang",
        "patch_date",
        "patch_label",
        "patch_url",
        "source_url",
        "source_quote",
        "successor",
    ];
    if fields.keys().any(|name| !allowed.contains(&name.as_str())) {
        return Err(malformed());
    }
    let item = key(entry, "item")?.ok_or_else(malformed)?;
    let successor = key(entry, "successor")?;
    date(entry)?;
    nullable_text(entry, "patch_label")?;
    link(entry, "patch_url")?;
    link(entry, "source_url")?;
    if entry["source_url"].is_null() || text(entry, "source_quote").is_none() {
        return Err(malformed());
    }
    let former = match entry["event"].as_str() {
        Some("renamed") if successor.is_none() => {
            let name = text(entry, "former_name")
                .filter(|name| name.len() <= NAME_BYTES)
                .ok_or_else(malformed)?;
            if !matches!(entry["lang"].as_str(), Some("en" | "de")) {
                return Err(malformed());
            }
            Some(name)
        }
        Some("removed") if !fields.contains_key("former_name") && !fields.contains_key("lang") => {
            None
        }
        _ => return Err(malformed()),
    };
    if successor.as_deref() == Some(item.as_str()) {
        return Err(malformed());
    }
    Ok(Checked { item, former })
}

fn extract(bytes: &[u8]) -> Result<FactSet, Rejection> {
    let document = parse_json_strict(bytes).map_err(|_| malformed())?;
    let object = document.as_object().ok_or_else(malformed)?;
    if object.len() != 2 || document["schema"] != SCHEMA {
        return Err(malformed());
    }
    let parsed = document["entries"].as_array().ok_or_else(malformed)?;
    #[derive(serde::Deserialize)]
    struct Envelope<'a> {
        #[serde(borrow)]
        entries: Vec<&'a RawValue>,
    }
    let envelope: Envelope<'_> = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    if parsed.is_empty() || parsed.len() != envelope.entries.len() {
        return Err(malformed());
    }
    let mut set = FactSet::default();
    set.entities.push(Entity {
        kind: ENTITY_KIND.into(),
        key: ENTITY_KEY.into(),
        receipt: receipt("raw_bytes", String::new(), bytes),
    });
    let mut seen = BTreeSet::new();
    for (index, (entry, lexeme)) in parsed.iter().zip(envelope.entries).enumerate() {
        let checked = check(entry)?;
        set.facts.push(Fact {
            entity_key: ENTITY_KEY.into(),
            stat_key: format!("entry_{index}"),
            value: FactValue::Text(lexeme.get().to_owned()),
            receipt: receipt(
                "json_pointer",
                format!("/entries/{index}"),
                lexeme.get().as_bytes(),
            ),
        });
        let Some(former) = checked.former else {
            continue;
        };
        if !seen.insert((checked.item.clone(), limits::normalize_alias(&former))) {
            return Err(malformed());
        }
        let fields: std::collections::BTreeMap<&str, &RawValue> =
            serde_json::from_str(lexeme.get()).map_err(|_| malformed())?;
        let name = fields.get("former_name").ok_or_else(malformed)?.get();
        set.aliases.push(Alias {
            entity_key: checked.item,
            alias: former,
            lang: entry["lang"].as_str().ok_or_else(malformed)?.into(),
            receipt: receipt(
                "json_pointer",
                format!("/entries/{index}/former_name"),
                name.as_bytes(),
            ),
        });
    }
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(bytes: &[u8]) -> RawObject {
        RawObject {
            id: 1,
            source: SOURCE.into(),
            revision: "sha256:test".into(),
            path: PATH.into(),
            content_sha256: Sha256::digest(bytes).into(),
            bytes: bytes.to_vec(),
        }
    }

    #[test]
    fn embedded_lineage_is_accepted_with_cited_aliases() {
        let Outcome::Accepted(set) = ItemLineage.extract(&raw(DATA)) else {
            panic!("embedded item lineage must be accepted");
        };
        assert!(set
            .aliases
            .iter()
            .any(|alias| alias.entity_key == "upgrade_reduce_debuff_duration"
                && alias.alias == "Debuff Remover"));
        assert!(set
            .aliases
            .iter()
            .any(|alias| alias.entity_key == "upgrade_tech_purge"
                && alias.alias == "Improved Spirit Armor"));
        assert_eq!(set.entities.len(), 1);
    }

    #[test]
    fn removed_entries_cannot_carry_former_names() {
        let bytes = br#"{"schema":"item-lineage/v1","entries":[{"item":"upgrade_x","event":"removed","former_name":"X","lang":"en","patch_date":null,"patch_label":null,"patch_url":null,"source_url":"https://deadlock.wiki/","source_quote":"q"}]}"#;
        assert!(matches!(
            ItemLineage.extract(&raw(bytes)),
            Outcome::Rejected(_)
        ));
    }
}
