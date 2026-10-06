use brain_contracts::{
    source::{origin_from_record, SourceRevision},
    value::Observed,
    PortError, SourceRecordV2, SourceVisibility,
};
use brain_storage::source_versions::{DOCUMENT_METADATA_KEY, ORIGINAL_VERSION_KEY};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const KNOWLEDGE_PROJECTION_VERSION: &str = "wiki-spielwissen-semantic-v1";
pub const KNOWLEDGE_CHUNKER_VERSION: &str =
    "wiki-spielwissen-semantic-v1+utf8-window-v1-1024-overlap192+atomic-facts";
pub const KNOWLEDGE_BYTE_BASIS: &str = "wiki-spielwissen-semantic-utf8";
pub(crate) const MAX_INDEX_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedFact {
    pub byte_start: usize,
    pub byte_end: usize,
    pub fact_id: String,
    pub subject: String,
    pub evidence_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeProjection {
    pub text: String,
    pub raw_sha256: String,
    pub semantic_sha256: String,
    pub raw_byte_end: usize,
    pub source_locator: String,
    pub document_evidence_status: String,
    pub facts: Vec<ProjectedFact>,
}

fn invalid() -> PortError {
    PortError::InvalidResponse("knowledge_projection_binding".into())
}

fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

fn field<'a>(value: &'a Value, key: &str) -> Result<&'a str, PortError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| valid_text(s))
        .ok_or_else(invalid)
}

fn status(value: &Value) -> Result<&str, PortError> {
    let status = field(value, "evidence_status")?;
    if !matches!(
        status,
        "extracted_value" | "source_statement" | "hypothesis"
    ) {
        return Err(invalid());
    }
    Ok(status)
}

fn canonical(value: &Value) -> Value {
    match value {
        Value::Object(object) => {
            let ordered: BTreeMap<_, _> = object
                .iter()
                .map(|(key, value)| (key.clone(), canonical(value)))
                .collect();
            Value::Object(ordered.into_iter().collect())
        }
        Value::Array(values) => Value::Array(values.iter().map(canonical).collect()),
        _ => value.clone(),
    }
}

pub fn project_knowledge(
    record: &SourceRecordV2,
) -> Result<Option<KnowledgeProjection>, PortError> {
    let Some(encoded) = record.metadata.get(DOCUMENT_METADATA_KEY) else {
        return Ok(None);
    };
    if record.content.len() > MAX_INDEX_BYTES {
        return Err(PortError::BudgetExceeded);
    }
    let origin = origin_from_record(record).map_err(|_| invalid())?;
    let document: Value = serde_json::from_str(encoded).map_err(|_| invalid())?;
    let raw_sha256 = format!("{:x}", Sha256::digest(record.content.as_bytes()));
    let revision = field(&document, "revision")?;
    let locator = field(&document, "source_locator")?;
    let evidence_status = status(&document)?;
    if field(&document, "contract_version")? != "wiki-spielwissen-v1"
        || !matches!(field(&document, "source_kind")?, "wiki" | "game_file")
        || field(&document, "source_id")? != record.source_id
        || field(&document, "document_id")? != record.logical_id
        || document.get("content").and_then(Value::as_str) != Some(record.content.as_str())
        || field(&document, "content_sha256")? != raw_sha256
        || record.content_hash != raw_sha256
        || record
            .metadata
            .get(ORIGINAL_VERSION_KEY)
            .map(String::as_str)
            != Some(revision)
        || origin.locator != locator
        || origin.origin_artifacts.is_empty()
        || origin
            .origin_artifacts
            .iter()
            .any(|value| !valid_text(value))
        || !matches!(&origin.derivation_family, Observed::Known { value } if valid_text(value))
        || !origin.policy.raw_retention_allowed
        || !matches!(&origin.policy.authorization_ref, Observed::Known { value } if valid_text(value))
        || !record
            .metadata
            .get("wiki-spielwissen.provenance_evidence_ref")
            .is_some_and(|value| valid_text(value))
    {
        return Err(invalid());
    }
    match &origin.source_revision {
        SourceRevision::Wiki {
            page_id,
            revision_id,
        } if field(&document, "source_kind")? == "wiki"
            && record.logical_id == format!("wiki:{}:page:{page_id}", record.source_id)
            && revision.parse::<i64>().ok() == Some(*revision_id) => {}
        SourceRevision::Api {
            api_version,
            original_revision,
        } if api_version == "wiki-spielwissen-v1"
            && original_revision.as_deref() == Some(revision) => {}
        _ => return Err(invalid()),
    }
    field(&document, "title")?;
    field(&document, "language")?;
    let observed = chrono::DateTime::parse_from_rfc3339(field(&document, "observed_at")?)
        .map_err(|_| invalid())?;
    if observed.offset().local_minus_utc() != 0
        || !document.get("metadata").is_some_and(Value::is_object)
    {
        return Err(invalid());
    }
    let license = document.get("license").ok_or_else(invalid)?;
    let license_name = field(license, "name")?;
    field(license, "attribution")?;
    if !license
        .get("url")
        .is_some_and(|value| value.is_null() || value.is_string())
    {
        return Err(invalid());
    }
    let redistribution = license
        .get("redistribution_allowed")
        .and_then(Value::as_bool)
        .ok_or_else(invalid)?
        && !license_name.trim().eq_ignore_ascii_case("unverified");
    if (!redistribution
        && (origin.policy.publication_allowed || origin.policy.provider_egress_allowed))
        || (record.visibility == SourceVisibility::Public && !origin.policy.publication_allowed)
    {
        return Err(invalid());
    }
    let mut facts: Vec<_> = document
        .get("facts")
        .and_then(Value::as_array)
        .ok_or_else(invalid)?
        .iter()
        .collect();
    let mut ids = BTreeSet::new();
    for fact in &facts {
        let id = field(fact, "fact_id")?;
        field(fact, "subject")?;
        field(fact, "predicate")?;
        status(fact)?;
        if !ids.insert(id)
            || fact.get("value").is_none()
            || !fact.get("qualifiers").is_some_and(Value::is_object)
        {
            return Err(invalid());
        }
        for key in ["unit", "source_span"] {
            if !fact
                .get(key)
                .is_some_and(|value| value.is_null() || value.as_str().is_some_and(valid_text))
            {
                return Err(invalid());
            }
        }
        if fact.as_object().is_none_or(|object| object.len() != 8) {
            return Err(invalid());
        }
    }
    facts.sort_by(|a, b| a["fact_id"].as_str().cmp(&b["fact_id"].as_str()));
    let mut projection = KnowledgeProjection {
        text: record.content.clone(),
        raw_sha256,
        semantic_sha256: String::new(),
        raw_byte_end: record.content.len(),
        source_locator: locator.into(),
        document_evidence_status: evidence_status.into(),
        facts: Vec::with_capacity(facts.len()),
    };
    for fact in facts {
        let text = serde_json::to_string_pretty(&canonical(fact)).map_err(|_| invalid())?;
        if projection
            .text
            .len()
            .checked_add(text.len())
            .and_then(|n| n.checked_add(1))
            .is_none_or(|n| n > MAX_INDEX_BYTES)
        {
            return Err(PortError::BudgetExceeded);
        }
        projection.text.push('\n');
        let start = projection.text.len();
        projection.text.push_str(&text);
        projection.facts.push(ProjectedFact {
            byte_start: start,
            byte_end: projection.text.len(),
            fact_id: field(fact, "fact_id")?.into(),
            subject: field(fact, "subject")?.into(),
            evidence_status: status(fact)?.into(),
        });
    }
    projection.semantic_sha256 = format!("{:x}", Sha256::digest(projection.text.as_bytes()));
    Ok(Some(projection))
}

impl KnowledgeProjection {
    pub fn provenance_metadata(
        &self,
        record: &SourceRecordV2,
        start: usize,
    ) -> BTreeMap<String, String> {
        let mut metadata: BTreeMap<_, _> = record
            .metadata
            .iter()
            .filter(|(key, _)| key.as_str() != DOCUMENT_METADATA_KEY)
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        metadata.insert("byte_basis".into(), KNOWLEDGE_BYTE_BASIS.into());
        metadata.insert(
            "knowledge_projection_version".into(),
            KNOWLEDGE_PROJECTION_VERSION.into(),
        );
        metadata.insert("knowledge_raw_sha256".into(), self.raw_sha256.clone());
        metadata.insert(
            "knowledge_semantic_sha256".into(),
            self.semantic_sha256.clone(),
        );
        let fact = self.facts.iter().find(|fact| fact.byte_start == start);
        metadata.insert(
            "evidence_status".into(),
            fact.map_or(&self.document_evidence_status, |fact| &fact.evidence_status)
                .clone(),
        );
        if let Some(fact) = fact {
            metadata.insert("fact_id".into(), fact.fact_id.clone());
        }
        metadata
    }
}
