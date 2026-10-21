use crate::{
    limits, meta_source::SOURCE, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject,
    Receipt, Rejection,
};
use dbrain_sources::external::parse_json_strict;
use serde_json::{value::RawValue, Value};
use sha2::{Digest, Sha256};

pub struct MetaStats;

pub const ENTITY_KIND: &str = "meta_stats";
pub const STAT_SETS: [(&str, &str, &str, &str); 2] = [
    ("hero-stats.json", "meta_hero_stats", "hero_id", "hero"),
    ("item-stats.json", "meta_item_stats", "item_id", "item"),
];
const FIELD_BYTES: usize = 64;

impl Extractor for MetaStats {
    fn name(&self) -> &'static str {
        "meta_stats"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == SOURCE && STAT_SETS.iter().any(|set| set.0 == path)
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
        "malformed_meta_stats",
        "Metastatistik enthält eine ungültige Zeile oder einen ungültigen Wert",
    )
}

fn receipt(kind: &str, locator: String, value: &[u8]) -> Receipt {
    Receipt {
        locator_kind: kind.into(),
        locator,
        value_sha256: Sha256::digest(value).into(),
    }
}

fn field(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= FIELD_BYTES
        && name.as_bytes()[0].is_ascii_lowercase()
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

fn row_id(row: &Value, id_field: &str) -> Result<u64, Rejection> {
    let fields = row.as_object().ok_or_else(malformed)?;
    if !fields
        .iter()
        .all(|(name, value)| field(name) && value.is_number())
    {
        return Err(malformed());
    }
    fields
        .get(id_field)
        .and_then(Value::as_u64)
        .filter(|id| *id > 0)
        .ok_or_else(malformed)
}

fn extract(raw: &RawObject) -> Result<FactSet, Rejection> {
    let (_, entity, id_field, prefix) = STAT_SETS
        .iter()
        .find(|set| set.0 == raw.path)
        .ok_or_else(malformed)?;
    let malformed_json = || {
        Rejection::new(
            "malformed_json",
            "Metastatistik enthält ungültiges JSON oder doppelte dekodierte Schlüssel",
        )
    };
    let value = parse_json_strict(&raw.bytes).map_err(|_| malformed_json())?;
    let rows: Vec<&RawValue> = serde_json::from_slice(&raw.bytes).map_err(|_| malformed_json())?;
    let parsed = value.as_array().ok_or_else(malformed)?;
    if parsed.is_empty() || parsed.len() != rows.len() {
        return Err(malformed());
    }
    let mut set = FactSet::default();
    set.entities.push(Entity {
        kind: ENTITY_KIND.into(),
        key: (*entity).into(),
        receipt: receipt("raw_bytes", String::new(), &raw.bytes),
    });
    for (index, (row, text)) in parsed.iter().zip(rows).enumerate() {
        let id = row_id(row, id_field)?;
        set.facts.push(Fact {
            entity_key: (*entity).into(),
            stat_key: format!("{prefix}_{id}"),
            value: FactValue::Text(text.get().to_owned()),
            receipt: receipt("json_pointer", format!("/{index}"), text.get().as_bytes()),
        });
    }
    limits::validate(&set)?;
    Ok(set)
}
