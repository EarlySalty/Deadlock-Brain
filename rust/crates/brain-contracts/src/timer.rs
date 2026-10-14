use serde::{Deserialize, Serialize};

use crate::PortError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimerKind {
    Urn,
    SoulBox,
    MidBoss,
    Rejuvenator,
}

impl TimerKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Urn => "urn",
            Self::SoulBox => "soul_box",
            Self::MidBoss => "mid_boss",
            Self::Rejuvenator => "rejuvenator",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetTimerRequest {
    pub timer: TimerKind,
    pub patch: Option<String>,
}

impl GetTimerRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        if let Some(patch) = &self.patch {
            if patch.trim().is_empty() || patch.len() > 128 || patch.chars().any(char::is_control) {
                return Err(super::invalid(
                    "Patchangabe benötigt ein bis 128 Bytes ohne Steuerzeichen",
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ToolCall, ToolDefinition, ToolName, ToolSubrequest};
    use serde_json::json;

    #[test]
    fn timer_request_is_closed_and_has_bounded_patch_text() {
        let definition = ToolDefinition {
            name: ToolName::GetTimer,
            description: "Kartentimer".into(),
            input_schema: json!({"type":"object","properties":{
                "timer":{"type":"string","enum":["urn","soul_box","mid_boss","rejuvenator"]},
                "patch":{"type":"string","minLength":1,"maxLength":128}},
                "required":["timer"],"additionalProperties":false}),
        };
        for timer in ["urn", "soul_box", "mid_boss", "rejuvenator"] {
            let request = ToolCall {
                id: "timer".into(),
                name: ToolName::GetTimer,
                arguments: json!({"timer":timer,"patch":"current"}),
            }
            .validate(std::slice::from_ref(&definition))
            .unwrap();
            assert!(
                matches!(request.subrequest(), ToolSubrequest::GetTimer(value) if value.timer.as_str() == timer)
            );
        }
        for arguments in [
            json!({}),
            json!({"timer":"Seelenbox"}),
            json!({"timer":"urn","sql":"SELECT 1"}),
            json!({"timer":"urn","scopes":["private"]}),
            json!({"timer":"urn","patch":" "}),
            json!({"timer":"urn","patch":"x".repeat(129)}),
            json!({"timer":"urn","patch":"current\n"}),
        ] {
            assert!(ToolCall {
                id: "timer".into(),
                name: ToolName::GetTimer,
                arguments,
            }
            .validate(std::slice::from_ref(&definition))
            .is_err());
        }
    }
}
