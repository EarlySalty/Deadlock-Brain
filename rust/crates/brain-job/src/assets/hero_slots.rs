use super::{leaves, rejection};
use crate::{limits, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Rejection, Relation};
use serde_json::value::RawValue;
use std::collections::BTreeSet;

pub struct HeroSlots;

impl Extractor for HeroSlots {
    fn name(&self) -> &'static str {
        "assets_hero_slots"
    }

    fn accepts(&self, source: &str, path: &str) -> bool {
        source == "assets"
            && (matches!(path, "heroes/en.json" | "heroes/en.invalid-response")
                || path.strip_prefix("heroes/en/").is_some_and(|tail| {
                    let tail = match tail.split_once('/') {
                        Some(("observed", part)) => {
                            return part.strip_suffix(".snapshot").is_some_and(index);
                        }
                        Some((hash, part))
                            if hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()) =>
                        {
                            part
                        }
                        Some(_) => return false,
                        None => tail,
                    };
                    tail.strip_suffix(".json")
                        .or_else(|| tail.strip_suffix(".oversized"))
                        .or_else(|| tail.strip_suffix(".collision"))
                        .is_some_and(index)
                }))
    }

    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        if raw.path.ends_with(".snapshot") {
            return Outcome::Accepted(FactSet::default());
        }
        if raw.path.ends_with(".collision") {
            return Outcome::Rejected(rejection("assets_hero_slot_ambiguous"));
        }
        if raw.path.ends_with(".oversized") {
            return Outcome::Rejected(rejection("raw_too_large"));
        }
        match extract(raw) {
            Ok(set) => Outcome::Accepted(set),
            Err(error) => Outcome::Rejected(error),
        }
    }
}

fn index(id: &str) -> bool {
    !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit())
}

fn extract(raw: &RawObject) -> Result<FactSet, Rejection> {
    let object: &RawValue =
        serde_json::from_slice(&raw.bytes).map_err(|_| rejection("assets_hero_json"))?;
    if !object.get().starts_with('{') {
        return Err(rejection("assets_hero_object"));
    }
    let fields = leaves(object, String::new()).map_err(rejection)?;
    let hero = fields
        .iter()
        .find(|field| field.pointer == "/class_name" && field.textual)
        .ok_or_else(|| rejection("assets_hero_identity"))?;
    let hero_key = limits::normalize_key(&hero.text);
    if !hero_key.starts_with("hero_") {
        return Err(rejection("assets_hero_identity"));
    }
    let mut set = FactSet::default();
    let mut seen = BTreeSet::new();
    for slot in 1..=4 {
        let pointer = format!("/items/signature{slot}");
        let ability = fields
            .iter()
            .find(|field| field.pointer == pointer && field.textual)
            .filter(|field| !field.text.trim().is_empty())
            .ok_or_else(|| rejection("assets_hero_slot_reference"))?;
        let ability_key = limits::normalize_key(&ability.text);
        if !seen.insert(ability_key.clone()) {
            return Err(rejection("assets_hero_slot_ambiguous"));
        }
        for (suffix, value, receipt) in [
            (
                "target",
                FactValue::Text(ability_key.clone()),
                ability.receipt(),
            ),
            ("slot", FactValue::Text(slot.to_string()), ability.receipt()),
            ("hero", FactValue::Text(hero_key.clone()), hero.receipt()),
        ] {
            set.facts.push(Fact {
                entity_key: hero_key.clone(),
                stat_key: format!("p2_slot_{slot}_{suffix}"),
                value,
                receipt,
            });
        }
        set.relations.push(Relation {
            from_key: hero_key.clone(),
            kind: "has_ability".into(),
            to_key: ability_key,
            receipt: ability.receipt(),
        });
    }
    limits::validate(&set)?;
    Ok(set)
}

#[cfg(test)]
#[path = "hero_slots_tests.rs"]
mod tests;
