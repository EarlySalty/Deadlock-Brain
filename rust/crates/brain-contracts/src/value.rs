//! Explicit missingness shared by all adapters. No numeric Default or null coercion.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum Observed<T> {
    Known { value: T },
    Unknown { reason: UnknownReason },
}
impl<T> Observed<T> {
    pub fn known(value: T) -> Self {
        Self::Known { value }
    }
    pub fn unknown(reason: UnknownReason) -> Self {
        Self::Unknown { reason }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownReason {
    NotPresent,
    ExplicitNull,
    Unmapped,
    Quarantined,
    IncompleteDependency,
    Unsupported,
    TickOriginNotEstablished,
    InitializationTick,
    SerialNotExposed,
    NotIndependentlyVerified,
}
