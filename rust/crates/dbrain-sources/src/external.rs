//! Shared source -> IR staging, not a second facts store or public wire contract.
//! The existing SourceStore persists raw documents and validated projections.
use crate::{Result, SourcesError};
use brain_contracts::{
    external::ExternalSourceIr,
    source::Versioned,
    value::{Observed, UnknownReason},
    SourceVisibility,
};
use deadlock_brain_core::http::SourceHttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

mod strict_json;
pub(crate) fn parse_json_strict(raw: &[u8]) -> std::result::Result<Value, serde_json::Error> {
    strict_json::parse(raw)
}

pub const IR_VERSION: u32 = 1;
pub const MAX_SOURCE_BYTES: usize = 8 * 1024 * 1024;

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
/// Deterministic serde JSON representation, intentionally not claimed as RFC 8785.
pub fn normalized_hash(value: &Value) -> String {
    fn ordered(value: &Value) -> Value {
        match value {
            Value::Object(map) => {
                let sorted: std::collections::BTreeMap<_, _> =
                    map.iter().map(|(k, v)| (k.clone(), ordered(v))).collect();
                Value::Object(sorted.into_iter().collect())
            }
            Value::Array(values) => Value::Array(values.iter().map(ordered).collect()),
            other => other.clone(),
        }
    }
    sha256(ordered(value).to_string().as_bytes())
}

pub use brain_contracts::external::{Provenance, SourceRevision, Validation};
#[derive(Debug, Clone)]
pub struct SourceIr {
    raw: Vec<u8>,
    contract: ExternalSourceIr,
}
impl SourceIr {
    pub fn from_http(
        source: &str,
        parser_revision: &str,
        response: SourceHttpResponse,
    ) -> Result<Self> {
        let hash = sha256(&response.content);
        let revision = SourceRevision::Http {
            body_sha256: hash,
            etag: response.headers.get("etag").cloned(),
            last_modified: response.headers.get("last-modified").cloned(),
        };
        let mut ir = Self::from_json(
            source,
            &response.url,
            parser_revision,
            revision,
            response.observed_at,
            response.content,
        )?;
        ir.contract.transport = json!({"status":response.status,"headers":response.headers,"attempts":response.attempts,"from_cache":false,"raw_representation":"http_entity_bytes"});
        if response.status != 200 {
            ir.quarantine(format!("http_status:{}", response.status));
        }
        let content_type = ir.contract.transport["headers"]["content-type"]
            .as_str()
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if content_type != "application/json"
            && !(content_type.starts_with("application/") && content_type.ends_with("+json"))
        {
            ir.quarantine("unexpected_content_type");
        }
        let encoding = ir.contract.transport["headers"]["content-encoding"]
            .as_str()
            .unwrap_or("identity");
        if !encoding.eq_ignore_ascii_case("identity") {
            ir.quarantine("unsupported_content_encoding");
        }
        Ok(ir)
    }

    pub fn from_json(
        source: &str,
        locator: &str,
        parser_revision: &str,
        revision: SourceRevision,
        observed_at: i64,
        raw: Vec<u8>,
    ) -> Result<Self> {
        if source.trim().is_empty()
            || locator.trim().is_empty()
            || parser_revision.trim().is_empty()
        {
            return Err(SourcesError::invalid_input(
                "source, locator and parser revision are required",
            ));
        }
        if raw.len() > MAX_SOURCE_BYTES {
            return Err(SourcesError::invalid_input("source IR byte limit"));
        }
        let hash = sha256(&raw);
        match &revision {
            SourceRevision::Git { commit } => {
                crate::git_source::validate_commit(commit)?;
            }
            SourceRevision::Http { body_sha256, .. } if body_sha256 != &hash => {
                return Err(SourcesError::invalid_input(
                    "source revision/raw hash mismatch",
                ));
            }
            _ => {}
        }
        let parsed = parse_json_strict(&raw);
        let (payload, validation) = match parsed {
            Ok(payload) => (
                Observed::known(payload),
                Validation::Validated {
                    extra_fields: Vec::new(),
                },
            ),
            Err(_) => (
                Observed::unknown(UnknownReason::Quarantined),
                Validation::Quarantined {
                    reasons: vec!["malformed_json_or_utf8".into()],
                },
            ),
        };
        let mut contract = ExternalSourceIr {
            payload,
            validation,
            provenance: Provenance {
                source: source.into(),
                locator: locator.into(),
                source_revision: revision,
                parser_revision: parser_revision.into(),
                parser_family: "dbrain-sources".into(),
                raw_sha256: hash,
                schema_sha256: None,
                observed_at,
                origin_artifacts: BTreeSet::new(),
                derivation_family: None,
                publication_authorized: false,
                provider_egress_authorized: false,
            },
            transport: Value::Null,
            schema_version: Observed::unknown(UnknownReason::NotPresent),
            field_provenance: BTreeMap::new(),
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::from([format!("source.review:{source}")]),
            license: Observed::unknown(UnknownReason::NotPresent),
        };
        contract.refresh_field_provenance();
        contract
            .origin_artifact()
            .validate()
            .map_err(SourcesError::invalid_input)?;
        Ok(Self { raw, contract })
    }
    /// Text/proto inputs retain their exact blob bytes; UTF-8 is checked without
    /// lossy conversion. They use the same provenance and quarantine envelope.
    pub fn from_text(
        source: &str,
        locator: &str,
        parser_revision: &str,
        revision: SourceRevision,
        observed_at: i64,
        raw: Vec<u8>,
    ) -> Result<Self> {
        let text = std::str::from_utf8(&raw).map(str::to_owned);
        let mut ir = Self::from_json(source, locator, parser_revision, revision, observed_at, raw)?;
        match text {
            Ok(text) => {
                ir.contract.payload = Observed::known(Value::String(text));
                ir.contract.validation = Validation::Validated {
                    extra_fields: Vec::new(),
                };
            }
            Err(_) => {
                ir.contract.payload = Observed::unknown(UnknownReason::Quarantined);
                ir.contract.validation = Validation::Quarantined {
                    reasons: vec!["invalid_text_utf8".into()],
                };
            }
        }
        ir.contract.refresh_field_provenance();
        Ok(ir)
    }
    pub fn contract(&self) -> Versioned<&ExternalSourceIr> {
        Versioned::new(&self.contract)
    }
    pub fn pin_schema_version(&mut self, version: &str) -> Result<()> {
        if version.trim().is_empty() {
            return Err(SourcesError::invalid_input("empty schema version"));
        }
        self.contract.schema_version = Observed::known(version.into());
        Ok(())
    }
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    pub fn provenance(&self) -> &Provenance {
        &self.contract.provenance
    }
    pub fn validation(&self) -> &Validation {
        &self.contract.validation
    }
    pub fn is_quarantined(&self) -> bool {
        matches!(self.contract.validation, Validation::Quarantined { .. })
    }
    pub fn payload(&self) -> Result<&Value> {
        if self.is_quarantined() {
            return Err(SourcesError::invalid_input("source is quarantined"));
        }
        match &self.contract.payload {
            Observed::Known { value } => Ok(value),
            Observed::Unknown { .. } => {
                Err(SourcesError::invariant("validated source lacks payload"))
            }
        }
    }
    pub fn quarantine(&mut self, reason: impl Into<String>) {
        let reason = reason.into();
        match &mut self.contract.validation {
            Validation::Quarantined { reasons } => {
                if !reasons.contains(&reason) {
                    reasons.push(reason);
                    reasons.sort();
                }
            }
            _ => {
                self.contract.validation = Validation::Quarantined {
                    reasons: vec![reason],
                }
            }
        }
    }
    pub fn note_extra_fields(&mut self, fields: Vec<String>) {
        if let Validation::Validated { extra_fields } = &mut self.contract.validation {
            extra_fields.extend(fields);
            extra_fields.sort();
            extra_fields.dedup();
        }
    }
    pub fn pin_schema(&mut self, hash: &str) {
        self.contract.provenance.schema_sha256 = Some(hash.into());
    }
    pub fn add_origin(&mut self, artifact: &str) -> Result<()> {
        if artifact.trim().is_empty() {
            return Err(SourcesError::invalid_input("empty origin artifact"));
        }
        self.contract
            .provenance
            .origin_artifacts
            .insert(artifact.into());
        Ok(())
    }
    pub fn set_derivation_family(&mut self, family: &str) {
        self.contract.provenance.derivation_family =
            (!family.trim().is_empty()).then(|| family.to_owned());
    }
    /// Distinguishes parser-only reprocessing and source observations in the
    /// existing document unique key, without changing source/entity identities.
    pub fn document_key(&self, external_id: &str) -> String {
        let mut derivation = json!({"source_revision":self.contract.provenance.source_revision,"parser_revision":self.contract.provenance.parser_revision,"schema_sha256":self.contract.provenance.schema_sha256,"validation":self.contract.validation,"http_status":self.contract.transport.get("status")});
        if let Observed::Known { value } = &self.contract.schema_version {
            derivation["schema_version"] = json!(value);
        }
        format!("{external_id}@{}", normalized_hash(&derivation))
    }
    pub fn metadata(&self) -> Value {
        json!({"contract":Versioned::new(&self.contract),"source_ir_version":IR_VERSION,"provenance":self.contract.provenance,"validation":self.contract.validation,"normalized_sha256":match &self.contract.payload { Observed::Known { value } => Some(normalized_hash(value)), Observed::Unknown { .. } => None },"transport":self.contract.transport})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceGroups {
    /// Correlated groups, explicitly NOT a claim of independent corroboration.
    pub correlated_groups: Vec<Vec<String>>,
    pub unknown_origin_records: Vec<String>,
}

pub fn group_provenance(records: &[Provenance]) -> ProvenanceGroups {
    // Set semantics first; repeated imports of the same derivation add no vote.
    let mut unique: std::collections::BTreeMap<String, Provenance> =
        std::collections::BTreeMap::new();
    for record in records {
        let key = normalized_hash(
            &json!({"source":record.source,"locator":record.locator,"revision":record.source_revision,"parser":record.parser_revision,"raw":record.raw_sha256}),
        );
        unique
            .entry(key)
            .and_modify(|existing| {
                existing
                    .origin_artifacts
                    .extend(record.origin_artifacts.iter().cloned());
                existing.observed_at = existing.observed_at.min(record.observed_at);
                existing.publication_authorized &= record.publication_authorized;
                existing.provider_egress_authorized &= record.provider_egress_authorized;
            })
            .or_insert_with(|| record.clone());
    }
    let rows: Vec<_> = unique.into_iter().collect();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut unknown = Vec::new();
    for (i, (key, record)) in rows.iter().enumerate() {
        if record.origin_artifacts.is_empty() {
            unknown.push(key.clone());
        }
        let mut merged = vec![i];
        let mut keep = Vec::new();
        for group in groups {
            let related = group.iter().any(|j| {
                let other = &rows[*j].1;
                record.raw_sha256 == other.raw_sha256
                    || !record.origin_artifacts.is_disjoint(&other.origin_artifacts)
                    || (record.derivation_family.is_some()
                        && record.derivation_family == other.derivation_family)
            });
            if related {
                merged.extend(group);
            } else {
                keep.push(group);
            }
        }
        keep.push(merged);
        groups = keep;
    }
    let mut correlated_groups: Vec<Vec<String>> = groups
        .into_iter()
        .map(|group| {
            let mut keys: Vec<_> = group.into_iter().map(|i| rows[i].0.clone()).collect();
            keys.sort();
            keys
        })
        .collect();
    correlated_groups.sort();
    unknown.sort();
    ProvenanceGroups {
        correlated_groups,
        unknown_origin_records: unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ir(raw: &[u8], parser: &str) -> SourceIr {
        SourceIr::from_json(
            "fixture",
            "fixture://record",
            parser,
            SourceRevision::Http {
                body_sha256: sha256(raw),
                etag: None,
                last_modified: None,
            },
            0,
            raw.into(),
        )
        .unwrap()
    }
    #[test]
    fn raw_hash_is_not_normalized_hash() {
        let a = ir(b" {\"a\": 1}\n", "v1");
        let b = ir(b"{\"a\":1}", "v1");
        assert_ne!(
            a.contract.provenance.raw_sha256,
            b.contract.provenance.raw_sha256
        );
        assert_eq!(
            a.metadata()["normalized_sha256"],
            b.metadata()["normalized_sha256"]
        );
        assert_eq!(a.raw(), b" {\"a\": 1}\n");
    }
    #[test]
    fn parser_revision_is_not_a_game_or_source_revision() {
        let a = ir(b"{}", "v1");
        let b = ir(b"{}", "v2");
        assert_eq!(
            a.contract.provenance.source_revision,
            b.contract.provenance.source_revision
        );
        assert_ne!(a.document_key("record"), b.document_key("record"));
        assert_eq!(
            a.document_key("record"),
            ir(b"{}", "v1").document_key("record")
        );
        assert!(!a.contract.provenance.publication_authorized);
    }
    #[test]
    fn malformed_preserved_but_cannot_be_projected() {
        for bytes in [b"{broken".as_slice(), b"\xff"] {
            let mut a = ir(bytes, "v1");
            a.quarantine("schema_unknown");
            a.quarantine("schema_unknown");
            assert!(a.is_quarantined());
            assert!(a.payload().is_err());
            assert_eq!(a.raw(), bytes);
            assert_eq!(
                a.metadata()["validation"]["reasons"]
                    .as_array()
                    .unwrap()
                    .len(),
                2
            );
        }
    }
    #[test]
    fn mirrors_and_shared_origin_are_not_independent() {
        let mut a = ir(b"{}", "v1");
        a.add_origin("game-state:commit:artifact").unwrap();
        let mut b = ir(b"{\"mirror\":true}", "other-parser");
        b.add_origin("game-state:commit:artifact").unwrap();
        let groups = group_provenance(&[
            a.contract.provenance.clone(),
            a.contract.provenance.clone(),
            b.contract.provenance.clone(),
        ]);
        assert_eq!(groups.correlated_groups.len(), 1);
        assert_eq!(groups.correlated_groups[0].len(), 2);
        assert!(groups.unknown_origin_records.is_empty());
        assert_eq!(
            group_provenance(&[ir(b"{}", "v1").contract.provenance])
                .unknown_origin_records
                .len(),
            1
        );
    }
    #[test]
    fn transitive_origins_merge_deterministically() {
        let mut a = ir(b"1", "v1");
        a.add_origin("x").unwrap();
        let mut b = ir(b"2", "v1");
        b.add_origin("y").unwrap();
        let mut c = ir(b"3", "v1");
        c.add_origin("x").unwrap();
        c.add_origin("y").unwrap();
        let records = vec![
            a.contract.provenance,
            b.contract.provenance,
            c.contract.provenance,
        ];
        assert_eq!(group_provenance(&records).correlated_groups.len(), 1);
        let mut reversed = records.clone();
        reversed.reverse();
        assert_eq!(group_provenance(&records), group_provenance(&reversed));
    }
}
