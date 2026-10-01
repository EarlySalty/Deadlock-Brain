//! External-source contracts. Validation is a claim to revalidate, not authority.
pub use crate::source::SourceRevision;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source: String,
    pub locator: String,
    pub source_revision: SourceRevision,
    pub parser_revision: String,
    pub parser_family: String,
    pub raw_sha256: String,
    pub schema_sha256: Option<String>,
    pub observed_at: i64,
    /// Explicit shared upstream artifacts, not source/portal names. Empty means unknown.
    pub origin_artifacts: BTreeSet<String>,
    pub derivation_family: Option<String>,
    /// Permissions are not inferred from public availability or a code license.
    pub publication_authorized: bool,
    pub provider_egress_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum Validation {
    Validated { extra_fields: Vec<String> },
    Quarantined { reasons: Vec<String> },
}

use crate::{
    source::*,
    value::{Observed, UnknownReason},
    SourceVisibility,
};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum FieldState {
    Present,
    Unknown { reason: UnknownReason },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldProvenance {
    /// RFC 6901 JSON pointer into the exact raw JSON artifact; empty is the root.
    pub locator: String,
    pub source_revision: SourceRevision,
    pub parser_revision: String,
    pub raw_sha256: String,
    pub state: FieldState,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalSourceIr {
    pub payload: Observed<Value>,
    pub provenance: Provenance,
    pub validation: Validation,
    pub transport: Value,
    pub schema_version: Observed<String>,
    pub field_provenance: BTreeMap<String, FieldProvenance>,
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub license: Observed<String>,
}
impl ExternalSourceIr {
    pub fn origin_artifact(&self) -> OriginArtifact {
        let p = &self.provenance;
        OriginArtifact {
            identity: SourceIdentity {
                source_id: p.source.clone(),
                logical_id: p.locator.clone(),
            },
            source_revision: p.source_revision.clone(),
            raw_sha256: p.raw_sha256.clone(),
            locator: p.locator.clone(),
            parser_revision: p.parser_revision.clone(),
            parser_family: p.parser_family.clone(),
            schema_version: self.schema_version.clone(),
            schema_sha256: observed_option(p.schema_sha256.clone()),
            retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(p.observed_at)),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::unknown(UnknownReason::NotPresent),
            origin_artifacts: p.origin_artifacts.clone(),
            derivation_family: observed_option(p.derivation_family.clone()),
            policy: SourcePolicy {
                visibility: self.visibility,
                allowed_scopes: self.allowed_scopes.clone(),
                authorization_ref: Observed::unknown(UnknownReason::NotPresent),
                license: self.license.clone(),
                publication_allowed: p.publication_authorized,
                provider_egress_allowed: p.provider_egress_authorized,
                raw_retention_allowed: false,
            },
            validity: GameValidity::unknown(),
        }
    }
    /// Null/missing/quarantined data cannot be read as a number, including zero.
    pub fn field(&self, pointer: &str) -> Observed<Value> {
        if matches!(self.validation, Validation::Quarantined { .. }) {
            return Observed::unknown(UnknownReason::Quarantined);
        }
        let Observed::Known { value } = &self.payload else {
            return Observed::unknown(UnknownReason::NotPresent);
        };
        match value.pointer(pointer) {
            None => Observed::unknown(UnknownReason::NotPresent),
            Some(Value::Null) => Observed::unknown(UnknownReason::ExplicitNull),
            Some(v) => Observed::known(v.clone()),
        }
    }
    pub fn refresh_field_provenance(&mut self) {
        fn walk(
            value: &Value,
            pointer: &str,
            p: &Provenance,
            out: &mut BTreeMap<String, FieldProvenance>,
        ) {
            out.insert(
                pointer.into(),
                FieldProvenance {
                    locator: pointer.into(),
                    source_revision: p.source_revision.clone(),
                    parser_revision: p.parser_revision.clone(),
                    raw_sha256: p.raw_sha256.clone(),
                    state: if value.is_null() {
                        FieldState::Unknown {
                            reason: UnknownReason::ExplicitNull,
                        }
                    } else {
                        FieldState::Present
                    },
                },
            );
            match value {
                Value::Object(map) => {
                    for (k, v) in map {
                        walk(
                            v,
                            &format!("{pointer}/{}", k.replace('~', "~0").replace('/', "~1")),
                            p,
                            out,
                        );
                    }
                }
                Value::Array(values) => {
                    for (i, v) in values.iter().enumerate() {
                        walk(v, &format!("{pointer}/{i}"), p, out);
                    }
                }
                _ => {}
            }
        }
        self.field_provenance.clear();
        if let Observed::Known { value } = &self.payload {
            walk(value, "", &self.provenance, &mut self.field_provenance);
        }
    }
}
