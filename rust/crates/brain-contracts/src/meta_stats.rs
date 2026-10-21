use serde::{Deserialize, Serialize};

use crate::PortError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetaStatKind {
    Hero,
    Item,
}

impl MetaStatKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Item => "item",
        }
    }

    pub const fn stat_set(self) -> &'static str {
        match self {
            Self::Hero => "meta_hero_stats",
            Self::Item => "meta_item_stats",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetMetaStatsRequest {
    pub kind: MetaStatKind,
    pub game_id: Option<u64>,
    pub entity: Option<u64>,
}

impl GetMetaStatsRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        match (self.game_id, self.entity) {
            (Some(id), None) | (None, Some(id)) if id > 0 && id <= i64::MAX as u64 => Ok(()),
            _ => Err(super::invalid(
                "Metastatistik benötigt genau eine gültige Spiel-ID oder Entitäts-ID",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ToolCall, ToolDefinition, ToolName, ToolSubrequest};
    use serde_json::json;

    #[test]
    fn meta_stats_request_needs_exactly_one_bounded_identifier() {
        let definition = ToolDefinition {
            name: ToolName::GetMetaStats,
            description: "Metastatistik".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["kind"],
                "properties":{"kind":{"type":"string","enum":["hero","item"]},
                "game_id":{"type":"integer","minimum":1},"entity":{"type":"integer","minimum":1}}}),
        };
        for arguments in [
            json!({"kind":"hero","game_id":13}),
            json!({"kind":"item","entity":7}),
        ] {
            let request = ToolCall {
                id: "meta".into(),
                name: ToolName::GetMetaStats,
                arguments,
            }
            .validate(std::slice::from_ref(&definition))
            .unwrap();
            assert!(matches!(
                request.subrequest(),
                ToolSubrequest::GetMetaStats(_)
            ));
        }
        for arguments in [
            json!({"kind":"hero"}),
            json!({"kind":"hero","game_id":1,"entity":2}),
            json!({"kind":"hero","game_id":0}),
            json!({"kind":"hero","game_id":u64::MAX}),
            json!({"kind":"ability","game_id":1}),
            json!({"kind":"hero","game_id":1,"sql":"SELECT 1"}),
        ] {
            assert!(ToolCall {
                id: "meta".into(),
                name: ToolName::GetMetaStats,
                arguments,
            }
            .validate(std::slice::from_ref(&definition))
            .is_err());
        }
    }
}
