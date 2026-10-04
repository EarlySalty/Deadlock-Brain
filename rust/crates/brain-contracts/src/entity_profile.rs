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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        through_patch_inclusive: Option<String>,
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
    pub patch_story: Vec<PatchStoryChange>,
    pub unknowns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchStoryChange {
    pub patch_date: String,
    pub patch_title: Option<String>,
    pub entity_type: Option<String>,
    pub entity_name: Option<String>,
    pub ability_name: Option<String>,
    pub stat_name: Option<String>,
    pub old_value: Value,
    pub new_value: Value,
    pub change_type: Option<String>,
    pub numeric_direction: Option<String>,
    pub confidence: Value,
    pub provenance: PatchStoryProvenance,
    pub original_line: RestrictedPatchLine,
    pub additional_fields: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStoryProvenance {
    pub relation: String,
    pub source_url: Option<String>,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedPatchLine {
    pub text: Option<String>,
    pub redistribution_allowed: bool,
}
