use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::PortError;

pub const PATCH_LIST_PAGE: u32 = 25;
pub const PATCH_LIST_MAX_OFFSET: u32 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchKind {
    Major,
    Gameplay,
    Hotfix,
}

impl PatchKind {
    pub const ALL: [Self; 3] = [Self::Major, Self::Gameplay, Self::Hotfix];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Gameplay => "gameplay",
            Self::Hotfix => "hotfix",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListPatchesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<PatchKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

fn date(text: &str) -> Result<NaiveDate, PortError> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .ok()
        .filter(|_| text.len() == 10)
        .ok_or_else(|| super::invalid("Patchdatum benötigt das Format JJJJ-MM-TT"))
}

impl ListPatchesRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        let since = self.since.as_deref().map(date).transpose()?;
        let until = self.until.as_deref().map(date).transpose()?;
        if since.zip(until).is_some_and(|(since, until)| since > until) {
            return Err(super::invalid("since darf nicht nach until liegen"));
        }
        if self
            .latest
            .is_some_and(|latest| !(1..=PATCH_LIST_PAGE).contains(&latest))
        {
            return Err(super::invalid("latest erlaubt ein bis 25 Patches"));
        }
        if self.latest.is_some() && self.offset.is_some() {
            return Err(super::invalid("latest gilt nur ohne offset"));
        }
        if self
            .offset
            .is_some_and(|offset| offset > PATCH_LIST_MAX_OFFSET)
        {
            return Err(super::invalid("Patchliste erlaubt höchstens Versatz 10000"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PatchKind;
    use crate::{ToolCall, ToolDefinition, ToolName, ToolSubrequest};
    use serde_json::json;

    fn definition() -> ToolDefinition {
        ToolDefinition {
            name: ToolName::ListPatches,
            description: "Patchliste".into(),
            input_schema: json!({"type":"object","properties":{
                "since":{"type":"string","minLength":10,"maxLength":10},
                "until":{"type":"string","minLength":10,"maxLength":10},
                "kind":{"type":"string","enum":PatchKind::ALL.map(PatchKind::as_str)},
                "latest":{"type":"integer","minimum":1,"maximum":25},
                "offset":{"type":"integer","minimum":0,"maximum":10000}},
                "additionalProperties":false}),
        }
    }

    fn call(arguments: serde_json::Value) -> ToolCall {
        ToolCall {
            id: "patches".into(),
            name: ToolName::ListPatches,
            arguments,
        }
    }

    #[test]
    fn list_request_is_closed_and_bounded() {
        let definition = definition();
        let request = call(json!({"since":"2024-11-21","until":"2025-11-21","kind":"major"}))
            .validate(std::slice::from_ref(&definition))
            .unwrap();
        assert!(matches!(
            request.subrequest(),
            ToolSubrequest::ListPatches(value) if value.kind == Some(PatchKind::Major)
        ));
        assert!(call(json!({}))
            .validate(std::slice::from_ref(&definition))
            .is_ok());
        for arguments in [
            json!({"since":"21.11.2024"}),
            json!({"since":"2024-11-21T00:00:00"}),
            json!({"since":"2025-01-02","until":"2025-01-01"}),
            json!({"kind":"minor"}),
            json!({"latest":0}),
            json!({"latest":26}),
            json!({"latest":5,"offset":0}),
            json!({"offset":10001}),
            json!({"sql":"SELECT 1"}),
        ] {
            assert!(call(arguments)
                .validate(std::slice::from_ref(&definition))
                .is_err());
        }
    }
}
