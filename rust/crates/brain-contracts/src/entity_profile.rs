use crate::source::OriginArtifact;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const ENTITY_PROFILE_VERSION: &str = "brain-entity-profile-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Hero,
    Ability,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileSourceKind {
    GameFile,
    Wiki,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PatchValidity {
    Unknown {
        reason: String,
    },
    Known {
        from_patch: String,
        to_patch_exclusive: Option<String>,
        evidence_ref: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityIdentity {
    pub entity_key: String,
    pub kind: EntityKind,
    pub name: String,
    pub aliases: Vec<String>,
    pub identity_evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileProvenance {
    pub source_kind: ProfileSourceKind,
    pub origin: OriginArtifact,
    pub original_revision: String,
    pub observed_at: String,
    pub source_span: Option<String>,
    pub license: Value,
    pub document_metadata: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityProfileFact {
    pub fact_id: String,
    pub subject: String,
    pub predicate: String,
    pub value: Value,
    pub unit: Option<String>,
    pub qualifiers: Map<String, Value>,
    pub evidence_status: String,
    pub validity: PatchValidity,
    pub provenance: ProfileProvenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileConflict {
    pub predicate: String,
    pub preferred_fact_id: Option<String>,
    pub fact_ids: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityProfile {
    pub contract_version: String,
    pub entity: EntityIdentity,
    pub patch: Option<String>,
    pub source_state: Vec<String>,
    pub facts: Vec<EntityProfileFact>,
    pub context: Vec<EntityProfileFact>,
    pub conflicts: Vec<ProfileConflict>,
    pub patch_story: Vec<Value>,
    pub unknowns: Vec<String>,
}
