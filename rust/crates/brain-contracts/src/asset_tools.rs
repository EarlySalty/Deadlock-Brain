use super::*;

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
    patch.as_deref().map_or(Ok(()), validate_canonical_text)
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
                ))
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
