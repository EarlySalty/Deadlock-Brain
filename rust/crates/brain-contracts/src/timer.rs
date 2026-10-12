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
            super::validate_canonical_text(patch)?;
        }
        Ok(())
    }
}
