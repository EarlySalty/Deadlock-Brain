use super::{invalid, validate_ids, validate_numeric_id, validate_numeric_ids};
use crate::PortError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetItemRequest {
    pub item: u64,
    pub patch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetAbilityRequest {
    pub ability: Option<u64>,
    pub hero: Option<u64>,
    pub slot: Option<u8>,
    pub patch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompareRequest {
    pub entities: Vec<u64>,
    pub stats: Vec<String>,
    pub patch: Option<String>,
}

fn validate_patch(patch: &Option<String>) -> Result<(), PortError> {
    if patch.as_deref().is_some_and(|patch| {
        patch.trim().is_empty() || patch.len() > 128 || patch.chars().any(char::is_control)
    }) {
        return Err(invalid("Ungültiger Quellenstand"));
    }
    Ok(())
}

impl GetItemRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        validate_numeric_id(self.item)?;
        validate_patch(&self.patch)
    }
}

impl GetAbilityRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        match (self.ability, self.hero, self.slot) {
            (Some(ability), None, None) => validate_numeric_id(ability)?,
            (None, Some(hero), Some(slot)) if (1..=4).contains(&slot) => {
                validate_numeric_id(hero)?;
            }
            _ => {
                return Err(invalid(
                    "Fähigkeit benötigt entweder eine Entitäts-ID oder Held und Slot eins bis vier",
                ));
            }
        }
        validate_patch(&self.patch)
    }
}

impl CompareRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        if !(2..=4).contains(&self.entities.len()) || !(1..=12).contains(&self.stats.len()) {
            return Err(invalid(
                "Vergleich benötigt zwei bis vier Entitäten und ein bis zwölf Werte",
            ));
        }
        validate_numeric_ids(&self.entities)?;
        validate_ids(&self.stats)?;
        for key in &self.stats {
            if key.len() > 128
                || !key
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.'))
                || !key
                    .bytes()
                    .next()
                    .is_some_and(|byte| byte.is_ascii_alphabetic())
            {
                return Err(invalid("Ungültiger kanonischer Statistikschlüssel"));
            }
        }
        validate_patch(&self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn item(value: Value) -> bool {
        serde_json::from_value::<GetItemRequest>(value)
            .is_ok_and(|request| request.validate().is_ok())
    }

    fn ability(value: Value) -> bool {
        serde_json::from_value::<GetAbilityRequest>(value)
            .is_ok_and(|request| request.validate().is_ok())
    }

    fn compare(value: Value) -> bool {
        serde_json::from_value::<CompareRequest>(value)
            .is_ok_and(|request| request.validate().is_ok())
    }

    #[test]
    fn canonical_ids_patch_text_and_closed_arguments_are_checked() {
        for value in [
            json!({"item":1}),
            json!({"item":i64::MAX,"patch":"current"}),
        ] {
            assert!(item(value));
        }
        for value in [
            json!({}),
            json!({"item":0}),
            json!({"item":i64::MAX as u64+1}),
            json!({"item":-1}),
            json!({"item":"Toxic Bullets"}),
            json!({"item":1,"patch":" "}),
            json!({"item":1,"patch":"ä".repeat(65)}),
            json!({"item":1,"patch":"a\n"}),
            json!({"item":1,"sql":"SELECT 1"}),
        ] {
            assert!(!item(value.clone()), "{value}");
        }
    }

    #[test]
    fn ability_selection_is_exclusive_and_slots_are_one_to_four() {
        for value in [
            json!({"ability":i64::MAX}),
            json!({"hero":1,"slot":1}),
            json!({"hero":1,"slot":4,"patch":"current"}),
        ] {
            assert!(ability(value));
        }
        for value in [
            json!({}),
            json!({"ability":1,"hero":2,"slot":1}),
            json!({"ability":1,"slot":1}),
            json!({"hero":1}),
            json!({"slot":1}),
            json!({"hero":1,"slot":0}),
            json!({"hero":1,"slot":5}),
            json!({"hero":1,"slot":1.5}),
            json!({"hero":0,"slot":1}),
            json!({"ability":0}),
            json!({"ability":1,"patch":"a\n"}),
        ] {
            assert!(!ability(value.clone()), "{value}");
        }
    }

    #[test]
    fn comparison_requires_distinct_bounded_entities_and_stat_keys() {
        assert!(compare(json!({"entities":[1,2],"stats":["weapon.damage"]})));
        assert!(compare(
            json!({"entities":[1,2,3,4],"stats":(0..12).map(|n| format!("stats.value_{n}")).collect::<Vec<_>>()})
        ));
        for value in [
            json!({"entities":[1],"stats":["stats.health"]}),
            json!({"entities":[1,2,3,4,5],"stats":["stats.health"]}),
            json!({"entities":[1,1],"stats":["stats.health"]}),
            json!({"entities":[0,2],"stats":["stats.health"]}),
            json!({"entities":[1,2],"stats":[]}),
            json!({"entities":[1,2],"stats":(0..13).map(|n|format!("stats.a{n}")).collect::<Vec<_>>()}),
            json!({"entities":[1,2],"stats":["stats.health","stats.health"]}),
            json!({"entities":[1,2],"stats":["stats.health;"]}),
            json!({"entities":[1,2],"stats":["stats health"]}),
            json!({"entities":[1,2],"stats":["1health"]}),
            json!({"entities":[1,2],"stats":["x".repeat(129)]}),
            json!({"entities":[1,2],"stats":["stats.health"],"patch":" "}),
            json!({"entities":[1,2],"stats":["stats.health"],"sql":"SELECT 1"}),
        ] {
            assert!(!compare(value.clone()), "{value}");
        }
    }
}
