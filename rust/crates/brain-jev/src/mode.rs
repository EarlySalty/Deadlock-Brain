//! Shadow observations have no authorization, routing or answer authority.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JevMode {
    #[default]
    Disabled,
    Shadow,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShadowObservation {
    pub mode: JevMode,
    pub contract_accepted: Option<bool>,
}
impl JevMode {
    pub fn observe(self, request: &str, response: &str) -> ShadowObservation {
        let contract_accepted = match self {
            Self::Disabled => None,
            Self::Shadow => Some(super::validate_json(request, response).is_ok()),
        };
        ShadowObservation {
            mode: self,
            contract_accepted,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_does_not_parse_or_activate_and_active_is_unrepresentable() {
        assert_eq!(
            JevMode::default()
                .observe("invalid", "invalid")
                .contract_accepted,
            None
        );
        assert!(serde_json::from_str::<JevMode>("\"active\"").is_err());
        assert_eq!(
            JevMode::Shadow
                .observe("invalid", "invalid")
                .contract_accepted,
            Some(false)
        );
    }
}
