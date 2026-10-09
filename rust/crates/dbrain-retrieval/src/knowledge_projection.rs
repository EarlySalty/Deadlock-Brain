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
pub const PUBLIC_GAME_FACTS_PROJECTION_VERSION: &str = "public-game-facts-compact-v1";
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

fn factual_grant(
    record: &SourceRecordV2,
    origin: &brain_contracts::source::OriginArtifact,
) -> Result<bool, PortError> {
    let Some(encoded) = record.metadata.get("brain.public_game_authorization") else {
        return Ok(false);
    };
    let grant: Value = serde_json::from_str(encoded).map_err(|_| invalid())?;
    let previous: brain_contracts::source::SourcePolicy =
        serde_json::from_value(grant["previous_policy"].clone()).map_err(|_| invalid())?;
    if grant["contract_version"] != "public-game-facts-v1"
        || grant.as_object().is_none_or(|object| object.len() != 8)
        || grant["publication_basis"] != "operator_public_factual_game_data"
        || grant["raw_asset_redistribution_authorized"] != false
        || grant["original_license_declaration_retained"] != true
        || !grant["previous_store_revision"]
            .as_u64()
            .is_some_and(|revision| revision > 0 && revision < record.revision)
        || !matches!(&origin.policy.authorization_ref, Observed::Known { value } if grant["authorization_ref"].as_str() == Some(value.as_str()) && valid_text(value))
        || !previous.raw_retention_allowed
        || !origin.policy.raw_retention_allowed
        || previous.license != origin.policy.license
        || record.visibility != SourceVisibility::Public
        || record.allowed_scopes != BTreeSet::from(["bot.public".into()])
        || !origin.policy.publication_allowed
        || (origin.policy.provider_egress_allowed
            && !grant["provider_egress_ref"]
                .as_str()
                .is_some_and(valid_text))
        || (!origin.policy.provider_egress_allowed && !grant["provider_egress_ref"].is_null())
    {
        return Err(invalid());
    }
    Ok(true)
}

pub fn public_game_factual_authorization(
    record: &SourceRecordV2,
    document: &Value,
    origin: &brain_contracts::source::OriginArtifact,
) -> Result<bool, PortError> {
    if !factual_grant(record, origin)? {
        return Ok(false);
    }
    if document["source_kind"] != "game_file"
        || document["metadata"]["app_id"] != 1422450
        || document["source_id"].as_str() != Some(record.source_id.as_str())
        || document["document_id"].as_str() != Some(record.logical_id.as_str())
    {
        return Err(invalid());
    }
    let metadata = &document["metadata"];
    let safe_path = |path: &str| {
        !path.contains(['\\', ':'])
            && std::path::Path::new(path)
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
    };
    let path = field(metadata, "relative_path")?;
    if !safe_path(path) || record.logical_id != format!("game:1422450:{path}") {
        return Err(invalid());
    }
    if matches!(
        record.source_id.as_str(),
        "legacy-entities" | "legacy-patchnotes"
    ) {
        let binding = &metadata["legacy_game_snapshot"];
        let source = field(binding, "upstream_source")?;
        let kind = field(binding, "entity_type")?;
        let external = field(binding, "external_id")?;
        let id = binding["snapshot_id"]
            .as_i64()
            .filter(|value| *value > 0)
            .ok_or_else(invalid)?;
        let kind_allowed = if record.source_id == "legacy-entities" {
            matches!(source, "deadlock_data" | "deadlock_assets_api")
                && (matches!(
                    kind,
                    "hero"
                        | "hero_card"
                        | "ability"
                        | "ability_card"
                        | "item"
                        | "item_card"
                        | "npc_unit"
                ) || (source == "deadlock_data"
                    && kind == "localization"
                    && matches!(external, "english" | "german")))
        } else {
            matches!(source, "deadlock_data" | "deadlock_patchnotes_db")
                && kind == "patchnote"
                && binding["original_url"].as_str().is_some_and(|url| {
                    url.starts_with("https://forums.playdeadlock.com/threads/")
                        || url.starts_with("https://store.steampowered.com/news/app/1422450/")
                        || url.starts_with(
                            "https://steamcommunity.com/games/1422450/announcements/detail/",
                        )
                })
        };
        if binding["contract_version"] != "legacy-game-snapshot-v1"
            || !kind_allowed
            || binding["payload_hash"].as_str() != Some(record.content_hash.as_str())
            || binding["source_document_id"]
                .as_i64()
                .is_none_or(|value| value <= 0)
            || !binding["original_document_sha256"]
                .as_str()
                .is_some_and(|value| {
                    value.len() == 64
                        && value
                            .bytes()
                            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                })
            || path
                != format!(
                    "legacy/{source}/{kind}/{:x}",
                    Sha256::digest(external.as_bytes())
                )
            || document["revision"].as_str()
                != Some(format!("legacy-snapshot:{id}:{}", record.content_hash).as_str())
            || document["source_locator"].as_str()
                != Some(format!("brain.entity_snapshots/{id}").as_str())
            || !metadata["original_source_document_metadata"].is_object()
        {
            return Err(invalid());
        }
        return Ok(true);
    }
    let repository = match record.source_id.as_str() {
        "deadlock-wiki-deadlock-data" => "https://github.com/deadlock-wiki/deadlock-data",
        "steamtracking-gametracking-deadlock" => {
            "https://github.com/SteamTracking/GameTracking-Deadlock"
        }
        _ => return Err(invalid()),
    };
    let original_path = field(metadata, "original_relative_path")?;
    let allowed_path = if record.source_id == "deadlock-wiki-deadlock-data" {
        (original_path.starts_with("data/json/") && original_path.ends_with(".json"))
            || matches!(
                original_path,
                "data/localizations/english.json" | "data/localizations/german.json"
            )
    } else {
        original_path.starts_with("game/citadel/pak01_dir/scripts/")
            || ((original_path.starts_with("game/citadel/resource/localization/")
                || original_path.starts_with("game/citadel/pak01_dir/resource/localization/"))
                && (original_path.ends_with("_english.txt")
                    || original_path.ends_with("_german.txt")))
    };
    let revision = field(document, "revision")?;
    if !safe_path(original_path)
        || !allowed_path
        || !matches!(revision.len(), 40 | 64)
        || !revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || metadata["source_revision"].as_str() != Some(revision)
        || metadata["provenance"]["git_commit"].as_str() != Some(revision)
        || metadata["provenance"]["repository_url"].as_str() != Some(repository)
        || metadata["extraction"]["version"] != "game-files-v2"
    {
        return Err(invalid());
    }
    Ok(true)
}

fn extracted_facts(path: &str, content: &str) -> Result<Vec<Value>, PortError> {
    let (facts, parse_status) =
        brain_storage::entity_profile::derivation::game_file_facts::extract_facts(path, content);
    if facts.is_empty() {
        return Err(PortError::InvalidResponse(format!(
            "public_game_facts_empty_or_unparsed:{path}:{parse_status}"
        )));
    }
    Ok(facts)
}

fn legacy_factual_projection(record: &SourceRecordV2) -> Result<KnowledgeProjection, PortError> {
    if record.content.len() > MAX_INDEX_BYTES {
        return Err(PortError::BudgetExceeded);
    }
    let origin = origin_from_record(record).map_err(|_| invalid())?;
    let raw_sha256 = format!("{:x}", Sha256::digest(record.content.as_bytes()));
    if !factual_grant(record, &origin)?
        || record.tombstone
        || record.content_hash != raw_sha256
        || origin.parser_family != "brain_legacy"
        || !matches!(&origin.source_revision, SourceRevision::Api { api_version, .. } if api_version == "brain_legacy.archive.v1")
    {
        return Err(invalid());
    }
    let mut values = Vec::new();
    let patch = record.source_id == "legacy-patchnotes";
    if record.source_id == "legacy-entities" {
        let (kind, name) = record
            .logical_id
            .strip_prefix("entity/")
            .and_then(|identity| identity.split_once('/'))
            .ok_or_else(invalid)?;
        let upstream = record.metadata.get("entity_source").ok_or_else(invalid)?;
        let external = record
            .metadata
            .get("entity_external_id")
            .ok_or_else(invalid)?;
        if !matches!(kind, "hero" | "ability" | "item" | "npc")
            || !matches!(upstream.as_str(), "deadlock_data" | "deadlock_assets_api")
            || origin.origin_artifacts != BTreeSet::from([format!("{upstream}:{external}")])
            || !origin.locator.starts_with("brain_legacy.entities#")
            || origin.derivation_family
                != Observed::known("brain_legacy.entities+entity_aliases".into())
            || record.content.lines().next() != Some(format!("{kind}: {name}").as_str())
        {
            return Err(invalid());
        }
        for line in record
            .content
            .lines()
            .filter(|line| !line.trim().is_empty())
        {
            let (key, value) = line.split_once(": ").ok_or_else(invalid)?;
            if !valid_text(key) || value.is_empty() {
                return Err(invalid());
            }
            let value = if matches!(key, "External ID" | "Source") {
                Value::String(value.into())
            } else {
                serde_json::from_str::<Value>(value).unwrap_or_else(|_| Value::String(value.into()))
            };
            values.push(serde_json::json!({"field": key, "value": value}));
        }
        for (key, expected) in [("Source", upstream), ("External ID", external)] {
            if values.iter().filter(|value| value["field"] == key).count() != 1
                || !values.iter().any(|value| {
                    value["field"] == key && value["value"].as_str() == Some(expected.as_str())
                })
            {
                return Err(invalid());
            }
        }
    } else if patch {
        if !record.logical_id.starts_with("patch/")
            || origin.derivation_family
                != Observed::known("brain_legacy.patch_events+patch_event_enrichments".into())
            || origin.origin_artifacts.is_empty()
            || !origin.origin_artifacts.iter().all(|url| {
                url.starts_with("https://forums.playdeadlock.com/threads/")
                    || url.starts_with("https://store.steampowered.com/news/app/1422450/")
            })
        {
            return Err(invalid());
        }
        let mut urls = BTreeSet::new();
        let mut statements = 0usize;
        for line in record
            .content
            .lines()
            .filter(|line| !line.trim().is_empty())
        {
            if let Some(statement) = line.strip_prefix("- ") {
                if statement.is_empty() {
                    return Err(invalid());
                }
                statements += 1;
                values.push(serde_json::json!({"statement": statement}));
            } else {
                let (key, value) = line.split_once(": ").ok_or_else(invalid)?;
                if statements != 0
                    || !matches!(key, "Patch" | "URL" | "Posted" | "Source kind")
                    || value.is_empty()
                {
                    return Err(invalid());
                }
                if key == "URL" {
                    urls.insert(value.to_owned());
                }
                values.push(serde_json::json!({"field": key, "value": value}));
            }
        }
        if statements == 0 || urls != origin.origin_artifacts {
            return Err(invalid());
        }
    } else {
        return Err(invalid());
    }
    let payload = serde_json::to_string(&values).map_err(|_| invalid())?;
    let mut facts = extracted_facts(&format!("{}.json", record.logical_id), &payload)?;
    for fact in &mut facts {
        fact["qualifiers"]["legacy_text_format"] = Value::String("brain_legacy.archive.v1".into());
        if patch {
            fact["evidence_status"] = Value::String("source_statement".into());
        }
    }
    project_facts(
        record,
        &origin.locator,
        if patch {
            "source_statement"
        } else {
            "extracted_value"
        },
        facts,
        true,
    )
}

pub fn project_knowledge(
    record: &SourceRecordV2,
) -> Result<Option<KnowledgeProjection>, PortError> {
    let Some(encoded) = record.metadata.get(DOCUMENT_METADATA_KEY) else {
        if record
            .metadata
            .contains_key("brain.public_game_authorization")
        {
            return legacy_factual_projection(record).map(Some);
        }
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
    let operator_factual_data = public_game_factual_authorization(record, &document, &origin)?;
    if (!redistribution
        && !operator_factual_data
        && (origin.policy.publication_allowed || origin.policy.provider_egress_allowed))
        || (record.visibility == SourceVisibility::Public && !origin.policy.publication_allowed)
    {
        return Err(invalid());
    }
    let declared = document
        .get("facts")
        .and_then(Value::as_array)
        .ok_or_else(invalid)?;
    let facts = if operator_factual_data {
        let path = field(&document["metadata"], "relative_path")?;
        let snapshot = document["metadata"].get("legacy_game_snapshot").is_some();
        let path = if snapshot {
            format!("{path}.json")
        } else {
            path.to_owned()
        };
        let extracted = extracted_facts(&path, &record.content)?;
        if !(snapshot && declared.is_empty()) && declared != &extracted {
            return Err(PortError::InvalidResponse(
                "public_game_facts_disagree_with_original".into(),
            ));
        }
        extracted
    } else {
        declared.clone()
    };
    project_facts(
        record,
        locator,
        evidence_status,
        facts,
        operator_factual_data,
    )
    .map(Some)
}

fn compact_factual_qualifiers(fact: &mut Value) -> Result<(), PortError> {
    let serialized_value = serde_json::to_string(&fact["value"]).map_err(|_| invalid())?;
    let qualifiers = fact
        .get_mut("qualifiers")
        .and_then(Value::as_object_mut)
        .ok_or_else(invalid)?;
    if qualifiers
        .get("source_pointer")
        .and_then(Value::as_str)
        .is_some_and(|pointer| {
            qualifiers.get("json_pointer").and_then(Value::as_str) == Some(pointer)
        })
    {
        qualifiers.remove("json_pointer");
    }
    if qualifiers
        .get("type_flags")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        qualifiers.remove("type_flags");
    }
    if qualifiers
        .get("numeric_representation")
        .is_some_and(Value::is_null)
    {
        qualifiers.remove("numeric_representation");
    }
    if qualifiers.get("source_lexeme").and_then(Value::as_str) == Some(serialized_value.as_str()) {
        qualifiers.remove("source_lexeme");
    }
    Ok(())
}

fn project_facts(
    record: &SourceRecordV2,
    locator: &str,
    evidence_status: &str,
    mut facts: Vec<Value>,
    facts_only: bool,
) -> Result<KnowledgeProjection, PortError> {
    if facts_only && facts.is_empty() {
        return Err(PortError::InvalidResponse("public_game_facts_empty".into()));
    }
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
        text: if facts_only {
            String::new()
        } else {
            record.content.clone()
        },
        raw_sha256: format!("{:x}", Sha256::digest(record.content.as_bytes())),
        semantic_sha256: String::new(),
        raw_byte_end: if facts_only { 0 } else { record.content.len() },
        source_locator: locator.into(),
        document_evidence_status: evidence_status.into(),
        facts: Vec::with_capacity(facts.len()),
    };
    for fact in facts {
        let mut projected = canonical(&fact);
        let text = if facts_only {
            compact_factual_qualifiers(&mut projected)?;
            serde_json::to_string(&projected)
        } else {
            serde_json::to_string_pretty(&projected)
        }
        .map_err(|_| invalid())?;
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
            fact_id: field(&fact, "fact_id")?.into(),
            subject: field(&fact, "subject")?.into(),
            evidence_status: status(&fact)?.into(),
        });
    }
    projection.semantic_sha256 = format!("{:x}", Sha256::digest(projection.text.as_bytes()));
    Ok(projection)
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
        let facts_only = self.raw_byte_end == 0
            && record
                .metadata
                .contains_key("brain.public_game_authorization");
        metadata.insert(
            "knowledge_projection_version".into(),
            if facts_only {
                PUBLIC_GAME_FACTS_PROJECTION_VERSION
            } else {
                KNOWLEDGE_PROJECTION_VERSION
            }
            .into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn record(content: &str) -> SourceRecordV2 {
        SourceRecordV2 {
            source_id: "deadlock-wiki-deadlock-data".into(),
            logical_id: "game:1422450:data/json/test.json".into(),
            revision: 2,
            content_hash: format!("{:x}", Sha256::digest(content.as_bytes())),
            content: content.into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        }
    }

    #[test]
    fn factual_projection_is_compact_and_losslessly_reconstructible() {
        let content =
            r#"{"Name":"Infernus","Escaped":"\/","False":false,"Nothing":null,"Scale":1e2}"#;
        let mut record = record(content);
        record
            .metadata
            .insert("brain.public_game_authorization".into(), "test".into());
        let (mut originals, parse_status) =
            brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
                "data/json/test.json",
                content,
            );
        assert_eq!(parse_status, "json");
        assert_eq!(originals.len(), 5);
        let before = originals.clone();
        let projection = project_facts(
            &record,
            "original",
            "extracted_value",
            originals.clone(),
            true,
        )
        .unwrap();
        assert_eq!(originals, before);
        assert_eq!(projection.raw_byte_end, 0);
        assert_eq!(projection.facts.len(), originals.len());
        assert_eq!(
            projection.semantic_sha256,
            format!("{:x}", Sha256::digest(projection.text.as_bytes()))
        );
        originals.sort_by(|a, b| a["fact_id"].as_str().cmp(&b["fact_id"].as_str()));
        for (binding, original) in projection.facts.iter().zip(&originals) {
            let text = &projection.text[binding.byte_start..binding.byte_end];
            let projected: Value = serde_json::from_str(text).unwrap();
            assert_eq!(text, serde_json::to_string(&canonical(&projected)).unwrap());
            assert_eq!(binding.fact_id, original["fact_id"].as_str().unwrap());
            assert_eq!(binding.subject, original["subject"].as_str().unwrap());
            assert_eq!(
                binding.evidence_status,
                original["evidence_status"].as_str().unwrap()
            );
            let mut reconstructed = projected.clone();
            let replacements = [
                (
                    "json_pointer",
                    projected["qualifiers"]["source_pointer"].clone(),
                ),
                ("type_flags", json!([])),
                ("numeric_representation", Value::Null),
                (
                    "source_lexeme",
                    json!(serde_json::to_string(&projected["value"]).unwrap()),
                ),
            ];
            for (key, replacement) in replacements {
                if original["qualifiers"].get(key).is_some()
                    && reconstructed["qualifiers"].get(key).is_none()
                {
                    reconstructed["qualifiers"][key] = replacement;
                }
            }
            assert_eq!(reconstructed, canonical(original));
        }
        let name = projection
            .facts
            .iter()
            .find(|fact| fact.fact_id == "json:/Name")
            .unwrap();
        let name: Value =
            serde_json::from_str(&projection.text[name.byte_start..name.byte_end]).unwrap();
        for key in [
            "json_pointer",
            "type_flags",
            "numeric_representation",
            "source_lexeme",
        ] {
            assert!(name["qualifiers"].get(key).is_none());
        }
        assert_eq!(name["qualifiers"]["source_pointer"], "/Name");
        assert_eq!(name["qualifiers"]["gameplay_binding"], "uninterpreted");
        assert_eq!(name["qualifiers"]["references_resolved"], false);
        assert_eq!(name["qualifiers"]["unit_status"], "unknown");
        let metadata = projection.provenance_metadata(&record, projection.facts[0].byte_start);
        assert_eq!(
            metadata["knowledge_projection_version"],
            PUBLIC_GAME_FACTS_PROJECTION_VERSION
        );
        assert_eq!(metadata["byte_basis"], KNOWLEDGE_BYTE_BASIS);
        assert_eq!(metadata["knowledge_raw_sha256"], record.content_hash);
    }

    #[test]
    fn factual_projection_preserves_meaningful_qualifiers_values_and_evidence() {
        let fact = json!({
            "fact_id": "numeric", "subject": "game_file:original", "predicate": "file.kv3_value",
            "value": "00100", "unit": "source-unit", "source_span": "original:/Damage",
            "evidence_status": "hypothesis",
            "qualifiers": {
                "source_pointer": "/Damage", "json_pointer": "/different",
                "type_flags": ["resource_name"], "numeric_representation": "source_numeric_lexeme",
                "source_lexeme": "00100", "gameplay_binding": "uninterpreted",
                "references_resolved": false, "unit_status": "unknown", "condition": "unchanged",
                "legacy_text_format": "brain_legacy.archive.v1", "unrecognized": {"keep": true}
            }
        });
        let record = record("original");
        let projection = project_facts(
            &record,
            "original",
            "source_statement",
            vec![fact.clone()],
            true,
        )
        .unwrap();
        let binding = &projection.facts[0];
        let projected: Value =
            serde_json::from_str(&projection.text[binding.byte_start..binding.byte_end]).unwrap();
        assert_eq!(projected, canonical(&fact));
        assert_eq!(binding.evidence_status, "hypothesis");
        assert_eq!(projection.document_evidence_status, "source_statement");
    }

    #[test]
    fn non_factual_projection_keeps_the_existing_bytes_and_format_binding() {
        let record = record(r#"{"Name":"Infernus","Health":100}"#);
        let (mut facts, _) =
            brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
                "data/json/test.json",
                &record.content,
            );
        let projection =
            project_facts(&record, "original", "extracted_value", facts.clone(), false).unwrap();
        facts.sort_by(|a, b| a["fact_id"].as_str().cmp(&b["fact_id"].as_str()));
        let mut expected = record.content.clone();
        for fact in &facts {
            expected.push('\n');
            expected.push_str(&serde_json::to_string_pretty(&canonical(fact)).unwrap());
        }
        assert_eq!(projection.text, expected);
        assert_eq!(projection.raw_byte_end, record.content.len());
        assert_eq!(
            projection.provenance_metadata(&record, projection.facts[0].byte_start)
                ["knowledge_projection_version"],
            KNOWLEDGE_PROJECTION_VERSION
        );
    }
}
