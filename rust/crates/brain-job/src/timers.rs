use crate::{
    limits, Alias, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt,
    Rejection,
};
use serde_json::json;
use std::collections::BTreeSet;

#[path = "timer_values.rs"]
mod values;

#[path = "timer_source.rs"]
pub mod source;

pub struct TimerExtractor;

impl Extractor for TimerExtractor {
    fn name(&self) -> &'static str {
        "timers_v2"
    }

    fn accepts(&self, source: &str, path: &str) -> bool {
        values::accepts(source, path)
    }

    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        match facts(raw) {
            Ok(set) => Outcome::Accepted(set),
            Err(rejection) => Outcome::Rejected(rejection),
        }
    }
}

pub(crate) fn facts(raw: &RawObject) -> Result<FactSet, Rejection> {
    let values = values::parse(
        &raw.source,
        &raw.path,
        &raw.bytes,
        limits::RAW_BYTES,
        |key| Ok(limits::normalize_key(key)),
    )
    .map_err(|code| Rejection::new(code, "Timerobjekt entspricht nicht dem Quellvertrag"))?;
    let mut set = FactSet::default();
    let mut entities = BTreeSet::new();
    for value in values {
        limits::validate_numeric_lexeme(&value.numeric)?;
        let entity_key = format!(
            "timer_{}__{}_{}",
            value.timer,
            limits::normalize_key(&raw.source),
            limits::normalize_key(&value.source_entity)
        );
        let receipt = Receipt {
            locator_kind: value.locator_kind.into(),
            locator: value.locator,
            value_sha256: value.value_sha256,
        };
        if entities.insert(entity_key.clone()) {
            set.entities.push(Entity {
                kind: "timer".into(),
                key: entity_key.clone(),
                receipt: receipt.clone(),
            });
            set.aliases.push(Alias {
                entity_key: entity_key.clone(),
                alias: value.timer.into(),
                lang: "und".into(),
                receipt: receipt.clone(),
            });
        }
        let payload = json!({
            "value":value.numeric,
            "unit":"s",
            "condition":{"source_entity":value.source_entity,"flags":value.flags}
        });
        set.facts.push(Fact {
            entity_key,
            stat_key: limits::normalize_key(value.stat),
            value: FactValue::Text(payload.to_string()),
            receipt,
        });
    }
    limits::validate(&set)?;
    Ok(set)
}
