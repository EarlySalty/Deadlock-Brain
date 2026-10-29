use serde::{Deserialize, Serialize};

use crate::PortError;

pub const MECHANIC_MAX_MINUTE: u16 = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MechanicKind {
    LaneTroopers,
    SoulSharing,
    Denizens,
    Objectives,
    MidBoss,
    Rejuvenator,
    HeroKills,
    Respawn,
    Urn,
    Breakables,
    SinnersSacrifice,
    ItemTiers,
    Levels,
    StreetBrawl,
    CoreAbilities,
}

impl MechanicKind {
    pub const ALL: [Self; 15] = [
        Self::LaneTroopers,
        Self::SoulSharing,
        Self::Denizens,
        Self::Objectives,
        Self::MidBoss,
        Self::Rejuvenator,
        Self::HeroKills,
        Self::Respawn,
        Self::Urn,
        Self::Breakables,
        Self::SinnersSacrifice,
        Self::ItemTiers,
        Self::Levels,
        Self::StreetBrawl,
        Self::CoreAbilities,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaneTroopers => "lane_troopers",
            Self::SoulSharing => "soul_sharing",
            Self::Denizens => "denizens",
            Self::Objectives => "objectives",
            Self::MidBoss => "mid_boss",
            Self::Rejuvenator => "rejuvenator",
            Self::HeroKills => "hero_kills",
            Self::Respawn => "respawn",
            Self::Urn => "urn",
            Self::Breakables => "breakables",
            Self::SinnersSacrifice => "sinners_sacrifice",
            Self::ItemTiers => "item_tiers",
            Self::Levels => "levels",
            Self::StreetBrawl => "street_brawl",
            Self::CoreAbilities => "core_abilities",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveKind {
    Guardian,
    Walker,
    Patron,
    BaseGuardian,
    Shrine,
}

impl ObjectiveKind {
    pub const ALL: [Self; 5] = [
        Self::Guardian,
        Self::Walker,
        Self::Patron,
        Self::BaseGuardian,
        Self::Shrine,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Guardian => "guardian",
            Self::Walker => "walker",
            Self::Patron => "patron",
            Self::BaseGuardian => "base_guardian",
            Self::Shrine => "shrine",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetMechanicRequest {
    pub mechanic: MechanicKind,
    pub minute: Option<u16>,
    pub patch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective: Option<ObjectiveKind>,
}

impl GetMechanicRequest {
    pub(super) fn validate(&self) -> Result<(), PortError> {
        if self
            .minute
            .is_some_and(|minute| minute > MECHANIC_MAX_MINUTE)
        {
            return Err(super::invalid("Spielminute liegt außerhalb von 0 bis 120"));
        }
        if self.objective.is_some() && self.mechanic != MechanicKind::Objectives {
            return Err(super::invalid("objective gilt nur für mechanic objectives"));
        }
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
    use super::{MechanicKind, ObjectiveKind};
    use crate::{ToolCall, ToolDefinition, ToolName, ToolSubrequest};
    use serde_json::json;

    fn definition() -> ToolDefinition {
        ToolDefinition {
            name: ToolName::GetMechanic,
            description: "Spielmechanik".into(),
            input_schema: json!({"type":"object","properties":{
                "mechanic":{"type":"string","enum":MechanicKind::ALL.map(MechanicKind::as_str)},
                "minute":{"type":"integer","minimum":0,"maximum":120},
                "patch":{"type":"string","minLength":1,"maxLength":128},
                "objective":{"type":"string","enum":ObjectiveKind::ALL.map(ObjectiveKind::as_str)}},
                "required":["mechanic"],"additionalProperties":false}),
        }
    }

    #[test]
    fn mechanic_request_is_closed_and_bounded() {
        let definition = definition();
        for mechanic in MechanicKind::ALL {
            let request = ToolCall {
                id: "mechanic".into(),
                name: ToolName::GetMechanic,
                arguments: json!({"mechanic":mechanic.as_str(),"minute":16}),
            }
            .validate(std::slice::from_ref(&definition))
            .unwrap();
            assert!(matches!(
                request.subrequest(),
                ToolSubrequest::GetMechanic(value)
                    if value.mechanic == mechanic && value.minute == Some(16)
            ));
        }
        let walker = ToolCall {
            id: "mechanic".into(),
            name: ToolName::GetMechanic,
            arguments: json!({"mechanic":"objectives","objective":"walker"}),
        }
        .validate(std::slice::from_ref(&definition))
        .unwrap();
        assert!(matches!(
            walker.subrequest(),
            ToolSubrequest::GetMechanic(value) if value.objective == Some(ObjectiveKind::Walker)
        ));
        for arguments in [
            json!({}),
            json!({"mechanic":"Trooper"}),
            json!({"mechanic":"lane_troopers","minute":121}),
            json!({"mechanic":"lane_troopers","minute":-1}),
            json!({"mechanic":"lane_troopers","minute":1.5}),
            json!({"mechanic":"lane_troopers","sql":"SELECT 1"}),
            json!({"mechanic":"lane_troopers","patch":" "}),
            json!({"mechanic":"lane_troopers","objective":"walker"}),
            json!({"mechanic":"objectives","objective":"tower"}),
        ] {
            assert!(ToolCall {
                id: "mechanic".into(),
                name: ToolName::GetMechanic,
                arguments,
            }
            .validate(std::slice::from_ref(&definition))
            .is_err());
        }
    }
}
