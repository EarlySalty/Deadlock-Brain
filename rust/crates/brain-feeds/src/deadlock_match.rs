use crate::{clean, configuration, FeedError, FeedPolicy, Result};
use brain_contracts::{
    source::{SourceRevision, SourceTimestamp},
    value::{Observed, UnknownReason},
    BatchReceipt, CorpusRelease, DocumentStorePort, SourceBatch, SourceCheckpoint,
    SourceVisibility,
};
use brain_ingestion::document_set::{
    current_pins, prepare_document_batch, CoreDocument, DocumentSetCheckpoint, DocumentSetSource,
    DocumentState,
};
use dbrain_sources::{
    core::http::SourceHttpResponse,
    deadlock_api::{prepare_match_response, DEMO_QUERY_VERSION},
    external::{sha256, SourceIr},
    schema_watch::OpenApiSnapshot,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const MATCH_SOURCE: &str = "deadlock-match";
pub const DEMO_SOURCE: &str = "deadlock-demo-evidence";
pub const MATCH_PARSER_REVISION: &str = "brain-feeds-match.v1";
pub const DEMO_PARSER_REVISION: &str = "brain-feeds-demo.v1";
const MAX_DEMO_ROWS: usize = 5_000;
const MAX_DEMO_ROW_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchScope {
    pub account_id: String,
    pub match_id: String,
}

#[derive(Debug, Clone)]
pub struct DemoEvidenceResponse {
    pub query_name: String,
    pub response: SourceHttpResponse,
}

fn numeric_id(value: &str, field: &str) -> Result<String> {
    let id = value
        .parse::<u64>()
        .map_err(|_| FeedError::Invalid(format!("{field} must be a canonical numeric ID")))?;
    if id == 0 || id.to_string() != value || (field == "account_id" && id > u32::MAX as u64) {
        return Err(FeedError::Invalid(format!(
            "{field} must be a canonical numeric ID"
        )));
    }
    Ok(value.to_owned())
}

pub fn match_source_id(scope: &MatchScope) -> Result<String> {
    Ok(format!(
        "{MATCH_SOURCE}-{}-{}",
        numeric_id(&scope.account_id, "account_id")?,
        numeric_id(&scope.match_id, "match_id")?
    ))
}

pub fn demo_source_id(scope: &MatchScope) -> Result<String> {
    Ok(format!(
        "{DEMO_SOURCE}-{}-{}",
        numeric_id(&scope.account_id, "account_id")?,
        numeric_id(&scope.match_id, "match_id")?
    ))
}

fn account_scope(account_id: &str) -> String {
    format!("account:{account_id}")
}

fn validate_private_policy(scope: &MatchScope, policy: &FeedPolicy) -> Result<()> {
    let required_scopes =
        BTreeSet::from([account_scope(&numeric_id(&scope.account_id, "account_id")?)]);
    if policy.visibility != SourceVisibility::Private || policy.allowed_scopes != required_scopes {
        return Err(FeedError::Invalid(
            "match evidence requires a private account scope".into(),
        ));
    }
    Ok(())
}

fn source(
    source_id: &str,
    parser_revision: &str,
    scope: &MatchScope,
    policy: &FeedPolicy,
) -> Result<DocumentSetSource> {
    Ok(DocumentSetSource {
        configuration: configuration(source_id, parser_revision, policy)?,
        source_id: source_id.into(),
        visibility: policy.visibility,
        allowed_scopes: policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([
            ("connector".into(), source_id.into()),
            (
                "account_id".into(),
                numeric_id(&scope.account_id, "account_id")?,
            ),
            ("match_id".into(), numeric_id(&scope.match_id, "match_id")?),
        ]),
    })
}

fn allowed_http_origin(url: &Url, production_host: &str) -> bool {
    let production = url.scheme() == "https"
        && url.host_str() == Some(production_host)
        && matches!(url.port(), None | Some(443));
    #[cfg(test)]
    let loopback = url.scheme() == "http"
        && url.port().is_some_and(|port| port != 0)
        && url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(|character| matches!(character, '[' | ']'))
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    #[cfg(not(test))]
    let loopback = false;
    production || loopback
}

pub fn match_metadata_url(scope: &MatchScope) -> Result<String> {
    let account_id = numeric_id(&scope.account_id, "account_id")?;
    let match_id = numeric_id(&scope.match_id, "match_id")?;
    Ok(format!(
        "https://api.deadlock-api.com/v1/matches/metadata?match_ids={match_id}&account_ids={account_id}&only_filtered_players=true&limit=1&format=json"
    ))
}

fn validate_match_locator(value: &str, expected_match_id: &str, account_id: &str) -> Result<()> {
    let url = Url::parse(value)
        .map_err(|_| FeedError::Quarantined("invalid match metadata locator".into()))?;
    if !allowed_http_origin(&url, "api.deadlock-api.com")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.path() != "/v1/matches/metadata"
    {
        return Err(FeedError::Quarantined(
            "match metadata origin or path outside policy".into(),
        ));
    }
    let mut actual = BTreeMap::new();
    for (key, value) in url.query_pairs() {
        if actual
            .insert(key.into_owned(), value.into_owned())
            .is_some()
        {
            return Err(FeedError::Quarantined(
                "repeated match metadata query parameter".into(),
            ));
        }
    }
    let expected = BTreeMap::from([
        ("match_ids".into(), expected_match_id.into()),
        ("account_ids".into(), account_id.into()),
        ("only_filtered_players".into(), "true".into()),
        ("limit".into(), "1".into()),
        ("format".into(), "json".into()),
    ]);
    if actual != expected {
        return Err(FeedError::Quarantined(
            "match metadata locator does not bind the requested identity".into(),
        ));
    }
    Ok(())
}

fn validate_demo_locator(value: &str) -> Result<()> {
    let url = Url::parse(value)
        .map_err(|_| FeedError::Quarantined("invalid demo evidence locator".into()))?;
    if !allowed_http_origin(&url, "demo-extracts.deadlock-api.com")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || !url.path().starts_with("/jobs/")
        || !url.path().ends_with("/result.ndjson")
        || url.query().is_some()
    {
        return Err(FeedError::Quarantined(
            "demo evidence origin or path outside policy".into(),
        ));
    }
    Ok(())
}

fn value_id(value: &Value) -> Option<String> {
    value.as_u64().map(|id| id.to_string()).or_else(|| {
        value
            .as_str()?
            .trim()
            .parse::<u64>()
            .ok()
            .map(|id| id.to_string())
    })
}

fn match_id(row: &Value) -> Option<String> {
    let object = row.as_object()?;
    ["match_id", "matchId"]
        .iter()
        .find_map(|key| object.get(*key).and_then(value_id))
}

fn contains_account(row: &Value, expected_account_id: &str) -> bool {
    row.get("players")
        .and_then(Value::as_array)
        .is_some_and(|players| {
            let mut seen = BTreeSet::new();
            players.iter().all(|player| {
                let account = player.get("account_id").and_then(value_id);
                let alias = player.get("accountId").and_then(value_id);
                let id = account.as_ref().or(alias.as_ref());
                !(account.is_some() && alias.is_some() && account != alias)
                    && id.is_some_and(|id| seen.insert(id.clone()))
            }) && seen.contains(expected_account_id)
        })
}

pub fn match_metadata_documents(
    scope: &MatchScope,
    response: SourceHttpResponse,
    policy: &FeedPolicy,
) -> Result<Vec<CoreDocument>> {
    let account_id = numeric_id(&scope.account_id, "account_id")?;
    let expected_match_id = numeric_id(&scope.match_id, "match_id")?;
    validate_private_policy(scope, policy)?;
    validate_match_locator(&response.url, &expected_match_id, &account_id)?;
    if response.content.len() > 1024 * 1024
        || response.observed_at <= 0
        || !(1..=2).contains(&response.attempts)
    {
        return Err(FeedError::Quarantined(
            "match response exceeded byte, time or retry bounds".into(),
        ));
    }
    let ir = prepare_match_response(response, false)
        .map_err(|error| FeedError::Invalid(error.to_string()))?;
    if ir.is_quarantined() {
        return Err(FeedError::Quarantined(format!("{:?}", ir.validation())));
    }
    let rows = ir
        .payload()
        .map_err(|error| FeedError::Invalid(error.to_string()))?
        .as_array()
        .ok_or_else(|| FeedError::Quarantined("match metadata is not an array".into()))?;
    if rows.len() != 1 || match_id(&rows[0]).as_deref() != Some(expected_match_id.as_str()) {
        return Err(FeedError::Quarantined(
            "match metadata must contain exactly the requested match".into(),
        ));
    }
    let (index, row) = (0, &rows[0]);
    if !contains_account(row, &account_id) {
        return Err(FeedError::Quarantined(
            "match metadata does not bind the requested account".into(),
        ));
    }
    let contract = ir.contract().data;
    let schema_version = match &contract.schema_version {
        Observed::Known { value } => value.clone(),
        Observed::Unknown { .. } => {
            return Err(FeedError::Quarantined(
                "match schema version not pinned".into(),
            ))
        }
    };
    let mut origin = contract.origin_artifact();
    origin.source_revision = SourceRevision::Api {
        api_version: schema_version.clone(),
        original_revision: Some(format!("sha256:{}", contract.provenance.raw_sha256)),
    };
    origin.locator = format!("{}#/{index}", contract.provenance.locator);
    origin.parser_revision = MATCH_PARSER_REVISION.into();
    origin.origin_artifacts = BTreeSet::from([format!(
        "{}@sha256:{}",
        contract.provenance.locator, contract.provenance.raw_sha256
    )]);
    origin.retrieved_at = Observed::known(SourceTimestamp::UnixSeconds(
        contract.provenance.observed_at,
    ));
    origin.language = Observed::unknown(UnknownReason::NotPresent);
    origin.policy.publication_allowed = policy.publication_allowed;
    origin.policy.provider_egress_allowed = policy.provider_egress_allowed;
    origin.policy.raw_retention_allowed = policy.raw_retention_allowed;
    let content = format!(
        "Match ID: {expected_match_id}\nAccount ID: {account_id}\nRaw SHA256: {}\nMetadata: {}\n",
        contract.provenance.raw_sha256,
        serde_json::to_string(row)?
    );
    Ok(vec![CoreDocument {
        logical_id: format!("match/{expected_match_id}/metadata"),
        content,
        metadata: BTreeMap::from([
            ("connector".into(), MATCH_SOURCE.into()),
            ("kind".into(), "fact".into()),
            ("account_id".into(), account_id),
            ("match_id".into(), expected_match_id),
            ("locator".into(), origin.locator.clone()),
            (
                "http_body_sha256".into(),
                contract.provenance.raw_sha256.clone(),
            ),
            (
                "schema_sha256".into(),
                contract
                    .provenance
                    .schema_sha256
                    .clone()
                    .ok_or_else(|| FeedError::Quarantined("match schema hash not pinned".into()))?,
            ),
            ("schema_version".into(), schema_version),
        ]),
        origin,
    }])
}

pub fn prepare_match_metadata_batch(
    scope: &MatchScope,
    response: SourceHttpResponse,
    policy: &FeedPolicy,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    let source_id = match_source_id(scope)?;
    Ok(prepare_document_batch(
        &source(&source_id, MATCH_PARSER_REVISION, scope, policy)?,
        &match_metadata_documents(scope, response, policy)?,
        previous,
    )?)
}

fn prepare_revoke_batch(
    scope: &MatchScope,
    policy: &FeedPolicy,
    previous: &SourceCheckpoint,
    source_id: &str,
    parser_revision: &str,
) -> Result<SourceBatch> {
    validate_private_policy(scope, policy)?;
    let source = source(source_id, parser_revision, scope, policy)?;
    if previous.source_id != source.source_id || previous.configuration != source.configuration {
        return Err(FeedError::Invalid(
            "match checkpoint identity mismatch".into(),
        ));
    }
    let mut state: DocumentSetCheckpoint = serde_json::from_value(previous.state.clone())?;
    if state.configuration != source.configuration {
        return Err(FeedError::Invalid(
            "match checkpoint configuration mismatch".into(),
        ));
    }
    let mut records = Vec::new();
    for (logical_id, prior) in &mut state.documents {
        if prior.tombstone {
            continue;
        }
        let revision = prior
            .revision
            .checked_add(1)
            .ok_or_else(|| FeedError::Invalid("match revision exhausted".into()))?;
        let content_hash = sha256(format!("tombstone:{logical_id}:{revision}").as_bytes());
        records.push(brain_contracts::SourceRecordV2 {
            source_id: source.source_id.clone(),
            logical_id: logical_id.clone(),
            revision,
            content_hash: content_hash.clone(),
            content: String::new(),
            visibility: SourceVisibility::Private,
            allowed_scopes: source.allowed_scopes.clone(),
            tombstone: true,
            valid_from: None,
            valid_to: None,
            metadata: source.tombstone_metadata.clone(),
        });
        *prior = DocumentState {
            revision,
            content_hash,
            tombstone: true,
        };
    }
    let batch = SourceBatch {
        expected_generation: previous.generation,
        checkpoint: SourceCheckpoint {
            source_id: source.source_id,
            configuration: source.configuration,
            generation: previous
                .generation
                .checked_add(1)
                .ok_or_else(|| FeedError::Invalid("match generation exhausted".into()))?,
            state: serde_json::to_value(state)?,
        },
        records,
    };
    batch.validate()?;
    Ok(batch)
}

pub fn prepare_revoke_match_batch(
    scope: &MatchScope,
    policy: &FeedPolicy,
    previous: &SourceCheckpoint,
) -> Result<SourceBatch> {
    prepare_revoke_batch(
        scope,
        policy,
        previous,
        &match_source_id(scope)?,
        MATCH_PARSER_REVISION,
    )
}

pub fn prepare_revoke_demo_batch(
    scope: &MatchScope,
    policy: &FeedPolicy,
    previous: &SourceCheckpoint,
) -> Result<SourceBatch> {
    prepare_revoke_batch(
        scope,
        policy,
        previous,
        &demo_source_id(scope)?,
        DEMO_PARSER_REVISION,
    )
}

pub async fn commit_match_batch<S: DocumentStorePort + ?Sized>(
    store: &S,
    batch: &SourceBatch,
    owner: &str,
) -> Result<BatchReceipt> {
    if ![MATCH_SOURCE, DEMO_SOURCE]
        .iter()
        .any(|kind| batch.checkpoint.source_id.starts_with(&format!("{kind}-")))
    {
        return Err(FeedError::Invalid("not a match evidence batch".into()));
    }
    batch.validate()?;
    let lease = store
        .claim(&batch.checkpoint.source_id, owner, 60_000)
        .await?;
    Ok(store.commit(batch, &lease).await?)
}

pub fn match_release_from_batches(
    base: &CorpusRelease,
    batches: &[SourceBatch],
    release_id: &str,
    created_at_epoch: i64,
) -> Result<CorpusRelease> {
    if release_id.is_empty()
        || release_id == base.release_id
        || created_at_epoch <= 0
        || batches.is_empty()
        || batches.len() > 2
    {
        return Err(FeedError::Invalid("invalid match release identity".into()));
    }
    let mut release = base.clone();
    release.release_id = release_id.into();
    release.created_at_epoch = created_at_epoch;
    let mut sources = BTreeSet::new();
    let mut identity = None;
    for batch in batches {
        let suffix = [MATCH_SOURCE, DEMO_SOURCE]
            .iter()
            .find_map(|kind| batch.checkpoint.source_id.strip_prefix(&format!("{kind}-")))
            .ok_or_else(|| FeedError::Invalid("invalid match release source".into()))?;
        if suffix.is_empty()
            || identity.as_deref().is_some_and(|prior| prior != suffix)
            || !sources.insert(&batch.checkpoint.source_id)
        {
            return Err(FeedError::Invalid("invalid match release source".into()));
        }
        identity = Some(suffix.to_string());
        batch.validate()?;
        let pins = current_pins(&batch.checkpoint)?;
        if pins.is_empty() {
            release.source_revisions.remove(&batch.checkpoint.source_id);
        } else {
            release
                .source_revisions
                .insert(batch.checkpoint.source_id.clone(), pins);
        }
    }
    Ok(release)
}

fn demo_ir(response: SourceHttpResponse) -> Result<SourceIr> {
    validate_demo_locator(&response.url)?;
    if response.status != 200 {
        return Err(FeedError::Quarantined(format!(
            "demo result HTTP status {}",
            response.status
        )));
    }
    if response.observed_at <= 0
        || !(1..=2).contains(&response.attempts)
        || response.content.len() > 8 * 1024 * 1024
        || response
            .headers
            .get("content-encoding")
            .is_some_and(|encoding| !encoding.eq_ignore_ascii_case("identity"))
    {
        return Err(FeedError::Quarantined(
            "demo response exceeded transport bounds".into(),
        ));
    }
    let content_type = response
        .headers
        .get("content-type")
        .map(|value| value.split(';').next().unwrap_or("").trim())
        .unwrap_or("");
    if !matches!(content_type, "application/x-ndjson" | "application/ndjson") {
        return Err(FeedError::Quarantined(
            "demo result content type is not NDJSON".into(),
        ));
    }
    let mut ir = SourceIr::from_text(
        dbrain_sources::deadlock_api::SOURCE,
        &response.url,
        DEMO_PARSER_REVISION,
        SourceRevision::Http {
            body_sha256: sha256(&response.content),
            etag: response.headers.get("etag").cloned(),
            last_modified: response.headers.get("last-modified").cloned(),
        },
        response.observed_at,
        response.content,
    )
    .map_err(|error| FeedError::Invalid(error.to_string()))?;
    let schema =
        OpenApiSnapshot::pinned().map_err(|error| FeedError::Invalid(error.to_string()))?;
    ir.pin_schema(&schema.schema_sha256);
    ir.pin_schema_version(DEMO_QUERY_VERSION)
        .map_err(|error| FeedError::Invalid(error.to_string()))?;
    ir.set_derivation_family("deadlock-api-demo-evidence");
    Ok(ir)
}

pub fn demo_evidence_documents(
    scope: &MatchScope,
    responses: Vec<DemoEvidenceResponse>,
    policy: &FeedPolicy,
) -> Result<Vec<CoreDocument>> {
    let account_id = numeric_id(&scope.account_id, "account_id")?;
    let match_id = numeric_id(&scope.match_id, "match_id")?;
    validate_private_policy(scope, policy)?;
    if responses.is_empty() || responses.len() > 8 {
        return Err(FeedError::Invalid("demo query count must be 1..=8".into()));
    }
    let mut query_names = BTreeSet::new();
    let mut documents = Vec::new();
    for item in responses {
        let query_name = clean(&item.query_name);
        if query_name.is_empty()
            || !query_name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            || !query_names.insert(query_name.clone())
        {
            return Err(FeedError::Invalid(
                "invalid or duplicate demo query name".into(),
            ));
        }
        let ir = demo_ir(item.response)?;
        let text = ir
            .payload()
            .map_err(|error| FeedError::Invalid(error.to_string()))?
            .as_str()
            .ok_or_else(|| FeedError::Quarantined("demo payload is not text".into()))?;
        let contract = ir.contract().data;
        let lines: Vec<_> = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        if lines.is_empty() || documents.len() + lines.len() > MAX_DEMO_ROWS {
            return Err(FeedError::Quarantined(
                "demo row count outside bounds".into(),
            ));
        }
        for (line_index, line) in lines.into_iter().enumerate() {
            if line.len() > MAX_DEMO_ROW_BYTES {
                return Err(FeedError::Quarantined("demo row exceeds byte limit".into()));
            }
            let mut row: Value = serde_json::from_str(line)
                .map_err(|_| FeedError::Quarantined("demo row is not valid JSON".into()))?;
            let object = row
                .as_object_mut()
                .ok_or_else(|| FeedError::Quarantined("demo row is not an object".into()))?;
            for (field, expected) in [("match_id", &match_id), ("account_id", &account_id)] {
                if object.get(field).and_then(value_id).as_deref() != Some(expected.as_str()) {
                    return Err(FeedError::Quarantined(format!(
                        "demo row {field} identity mismatch"
                    )));
                }
            }
            let ordinal = line_index + 1;
            object.insert("match_id".into(), Value::String(match_id.clone()));
            object.insert("account_id".into(), Value::String(account_id.clone()));
            object.insert("query_name".into(), Value::String(query_name.clone()));
            object.insert(
                "query_version".into(),
                Value::String(DEMO_QUERY_VERSION.into()),
            );
            object.insert(
                "evidence_id".into(),
                Value::String(format!("{query_name}:{match_id}:{ordinal:06}")),
            );
            let mut origin = contract.origin_artifact();
            origin.source_revision = SourceRevision::Api {
                api_version: DEMO_QUERY_VERSION.into(),
                original_revision: Some(format!("sha256:{}", contract.provenance.raw_sha256)),
            };
            origin.locator = format!("{}#L{ordinal}", contract.provenance.locator);
            origin.parser_revision = DEMO_PARSER_REVISION.into();
            origin.origin_artifacts = BTreeSet::from([format!(
                "{}@sha256:{}",
                contract.provenance.locator, contract.provenance.raw_sha256
            )]);
            origin.retrieved_at = Observed::known(SourceTimestamp::UnixSeconds(
                contract.provenance.observed_at,
            ));
            origin.language = Observed::unknown(UnknownReason::NotPresent);
            origin.policy.publication_allowed = policy.publication_allowed;
            origin.policy.provider_egress_allowed = policy.provider_egress_allowed;
            origin.policy.raw_retention_allowed = policy.raw_retention_allowed;
            documents.push(CoreDocument {
                logical_id: format!("match/{match_id}/demo/{query_name}/{ordinal:06}"),
                content: format!(
                    "Match ID: {match_id}\nAccount ID: {account_id}\nDemo query: {query_name}\nRaw SHA256: {}\nEvidence: {}\n",
                    contract.provenance.raw_sha256,
                    serde_json::to_string(&row)?
                ),
                metadata: BTreeMap::from([
                    ("connector".into(), DEMO_SOURCE.into()),
                    ("kind".into(), "prose".into()),
                    ("account_id".into(), account_id.clone()),
                    ("match_id".into(), match_id.clone()),
                    ("query_name".into(), query_name.clone()),
                    ("query_version".into(), DEMO_QUERY_VERSION.into()),
                    ("locator".into(), origin.locator.clone()),
                    ("http_body_sha256".into(), contract.provenance.raw_sha256.clone()),
                    (
                        "schema_sha256".into(),
                        contract
                            .provenance
                            .schema_sha256
                            .clone()
                            .ok_or_else(|| FeedError::Quarantined("demo schema hash not pinned".into()))?,
                    ),
                    ("schema_version".into(), DEMO_QUERY_VERSION.into()),
                ]),
                origin,
            });
        }
    }
    Ok(documents)
}

pub fn prepare_demo_evidence_batch(
    scope: &MatchScope,
    responses: Vec<DemoEvidenceResponse>,
    policy: &FeedPolicy,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    let source_id = format!(
        "{DEMO_SOURCE}-{}-{}",
        numeric_id(&scope.account_id, "account_id")?,
        numeric_id(&scope.match_id, "match_id")?
    );
    Ok(prepare_document_batch(
        &source(&source_id, DEMO_PARSER_REVISION, scope, policy)?,
        &demo_evidence_documents(scope, responses, policy)?,
        previous,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::source::origin_from_record;

    fn response(url: &str, content_type: &str, body: &[u8]) -> SourceHttpResponse {
        SourceHttpResponse {
            url: url.into(),
            status: 200,
            content: body.into(),
            headers: BTreeMap::from([("content-type".into(), content_type.into())]),
            observed_at: 1_790_000_000,
            attempts: 1,
        }
    }
    fn scope() -> MatchScope {
        MatchScope {
            account_id: "281768392".into(),
            match_id: "92685682".into(),
        }
    }
    fn policy() -> FeedPolicy {
        FeedPolicy {
            visibility: SourceVisibility::Private,
            allowed_scopes: BTreeSet::from(["account:281768392".into()]),
            provider_egress_allowed: false,
            publication_allowed: false,
            raw_retention_allowed: false,
        }
    }

    fn match_response(body: &[u8]) -> SourceHttpResponse {
        response(
            "https://api.deadlock-api.com/v1/matches/metadata?match_ids=92685682&account_ids=281768392&only_filtered_players=true&limit=1&format=json",
            "application/json",
            body,
        )
    }

    #[test]
    fn match_metadata_becomes_account_scoped_core_record_with_raw_provenance() {
        let body = br#"[{"match_id":92685682,"players":[{"account_id":281768392,"hero_id":18}]}]"#;
        let batch = prepare_match_metadata_batch(
            &scope(),
            response(
                "https://api.deadlock-api.com/v1/matches/metadata?match_ids=92685682&account_ids=281768392&only_filtered_players=true&limit=1&format=json",
                "application/json",
                body,
            ),
            &policy(),
            None,
        )
        .unwrap();
        assert_eq!(batch.records.len(), 1);
        let record = &batch.records[0];
        assert_eq!(record.logical_id, "match/92685682/metadata");
        assert_eq!(record.visibility, SourceVisibility::Private);
        assert_eq!(
            record.allowed_scopes,
            BTreeSet::from(["account:281768392".into()])
        );
        assert_eq!(record.metadata["http_body_sha256"], sha256(body));
        assert!(record.metadata["locator"].ends_with("#/0"));
        assert!(record.metadata.contains_key("schema_sha256"));
        assert_eq!(record.metadata["schema_version"], "0.1.0");
        let origin = origin_from_record(record).unwrap();
        assert_eq!(origin.parser_revision, MATCH_PARSER_REVISION);
        assert!(origin
            .origin_artifacts
            .iter()
            .any(|artifact| artifact.contains(&sha256(body))));
    }

    #[test]
    fn match_raw_revision_changes_on_byte_drift_but_not_on_identical_retry() {
        let body = br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#;
        let first =
            prepare_match_metadata_batch(&scope(), match_response(body), &policy(), None).unwrap();
        let replay = prepare_match_metadata_batch(
            &scope(),
            match_response(body),
            &policy(),
            Some(&first.checkpoint),
        )
        .unwrap();
        assert!(replay.records.is_empty());
        let changed = prepare_match_metadata_batch(
            &scope(),
            match_response(br#"[ {"match_id":92685682,"players":[{"account_id":281768392}]} ]"#),
            &policy(),
            Some(&first.checkpoint),
        )
        .unwrap();
        assert_eq!(changed.records.len(), 1);
        assert_eq!(changed.records[0].revision, 2);
        assert_ne!(
            changed.records[0].metadata["http_body_sha256"],
            first.records[0].metadata["http_body_sha256"]
        );
    }

    #[test]
    fn match_metadata_requires_private_exact_account_scope() {
        let mut public = policy();
        public.visibility = SourceVisibility::Public;
        let body = br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#;
        assert!(match_metadata_documents(&scope(), match_response(body), &public).is_err());

        let mut internal = policy();
        internal.visibility = SourceVisibility::Internal;
        assert!(match_metadata_documents(&scope(), match_response(body), &internal).is_err());

        let mut missing_scope = policy();
        missing_scope.allowed_scopes.clear();
        assert!(match_metadata_documents(&scope(), match_response(body), &missing_scope).is_err());

        let mut wrong_scope = policy();
        wrong_scope.allowed_scopes = BTreeSet::from(["account:1".into()]);
        assert!(match_metadata_documents(&scope(), match_response(body), &wrong_scope).is_err());

        let mut overbroad_scope = policy();
        overbroad_scope.allowed_scopes.insert("game.public".into());
        assert!(
            match_metadata_documents(&scope(), match_response(body), &overbroad_scope).is_err()
        );
    }

    #[test]
    fn match_metadata_accepts_only_the_exact_requested_match_and_account() {
        for body in [
            br#"[{"match_id":1,"players":[{"account_id":281768392}]}]"#.as_slice(),
            br#"[{"match_id":92685682,"players":[{"account_id":281768392}]},{"match_id":1,"players":[]}]"#.as_slice(),
            br#"[{"match_id":92685682,"players":[{"account_id":1}]}]"#.as_slice(),
            br#"[{"match_id":92685682,"players":[{"account_id":281768392},{"account_id":281768392}]}]"#.as_slice(),
            br#"[{"match_id":92685682,"players":[{"account_id":281768392,"accountId":1}]}]"#.as_slice(),
        ] {
            assert!(match_metadata_documents(&scope(), match_response(body), &policy()).is_err());
        }

        let wrong_locator = response(
            "https://api.deadlock-api.com/v1/matches/metadata?match_ids=1",
            "application/json",
            br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#,
        );
        assert!(match_metadata_documents(&scope(), wrong_locator, &policy()).is_err());
        let mut wrong_account_filter =
            match_response(br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#);
        wrong_account_filter.url = wrong_account_filter
            .url
            .replace("account_ids=281768392", "account_ids=1");
        assert!(match_metadata_documents(&scope(), wrong_account_filter, &policy()).is_err());
        let mut repeated_match =
            match_response(br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#);
        repeated_match.url.push_str("&match_ids=92685682");
        assert!(match_metadata_documents(&scope(), repeated_match, &policy()).is_err());

        let wrong_origin = response(
            "https://api.deadlock-api.com.evil.test/v1/matches/metadata?match_ids=92685682",
            "application/json",
            br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#,
        );
        assert!(match_metadata_documents(&scope(), wrong_origin, &policy()).is_err());
    }

    #[test]
    fn match_schema_drift_is_quarantined() {
        assert!(match_metadata_documents(
            &scope(),
            match_response(br#"[{"match_id":92685682,"players":"changed"}]"#),
            &policy(),
        )
        .is_err());
        let mut stale =
            match_response(br#"[{"match_id":92685682,"players":[{"account_id":281768392}]}]"#);
        stale.observed_at = 0;
        assert!(match_metadata_documents(&scope(), stale, &policy()).is_err());
    }

    #[test]
    fn demo_ndjson_becomes_citable_rows_and_rejects_identity_drift() {
        let body = b"{\"match_id\":92685682,\"account_id\":281768392,\"tick\":10}\n{\"match_id\":92685682,\"account_id\":281768392,\"tick\":20}\n";
        let batch = prepare_demo_evidence_batch(
            &scope(),
            vec![DemoEvidenceResponse {
                query_name: "player_state".into(),
                response: response(
                    "https://demo-extracts.deadlock-api.com/jobs/x/result.ndjson",
                    "application/x-ndjson",
                    body,
                ),
            }],
            &policy(),
            None,
        )
        .unwrap();
        assert_eq!(batch.records.len(), 2);
        assert!(batch.records[0].metadata["locator"].ends_with("#L1"));
        assert!(batch.records[0].content.contains(DEMO_QUERY_VERSION));
        assert_eq!(batch.records[0].metadata["http_body_sha256"], sha256(body));
        assert_eq!(
            batch.records[0].metadata["schema_version"],
            DEMO_QUERY_VERSION
        );

        let wrong = DemoEvidenceResponse {
            query_name: "player_state".into(),
            response: response(
                "https://demo-extracts.deadlock-api.com/jobs/x/result.ndjson",
                "application/x-ndjson",
                b"{\"match_id\":1,\"account_id\":281768392}\n",
            ),
        };
        assert!(demo_evidence_documents(&scope(), vec![wrong], &policy()).is_err());

        let wrong_account = DemoEvidenceResponse {
            query_name: "player_state".into(),
            response: response(
                "https://demo-extracts.deadlock-api.com/jobs/x/result.ndjson",
                "application/x-ndjson",
                b"{\"match_id\":92685682,\"account_id\":1}\n",
            ),
        };
        assert!(demo_evidence_documents(&scope(), vec![wrong_account], &policy()).is_err());

        let missing_identity = DemoEvidenceResponse {
            query_name: "player_state".into(),
            response: response(
                "https://demo-extracts.deadlock-api.com/jobs/x/result.ndjson",
                "application/x-ndjson",
                b"{\"tick\":10}\n",
            ),
        };
        assert!(demo_evidence_documents(&scope(), vec![missing_identity], &policy()).is_err());

        let wrong_origin = DemoEvidenceResponse {
            query_name: "player_state".into(),
            response: response(
                "https://demo-extracts.deadlock-api.com.evil.test/jobs/x/result.ndjson",
                "application/x-ndjson",
                b"{\"tick\":10}\n",
            ),
        };
        assert!(demo_evidence_documents(&scope(), vec![wrong_origin], &policy()).is_err());
    }

    #[test]
    fn demo_checkpoint_emits_tombstone_for_removed_evidence_row() {
        let first = prepare_demo_evidence_batch(
            &scope(),
            vec![DemoEvidenceResponse {
                query_name: "player_state".into(),
                response: response(
                    "https://demo-extracts.deadlock-api.com/jobs/x/result.ndjson",
                    "application/x-ndjson",
                    b"{\"match_id\":92685682,\"account_id\":281768392,\"tick\":10}\n{\"match_id\":92685682,\"account_id\":281768392,\"tick\":20}\n",
                ),
            }],
            &policy(),
            None,
        )
        .unwrap();
        let second = prepare_demo_evidence_batch(
            &scope(),
            vec![DemoEvidenceResponse {
                query_name: "player_state".into(),
                response: response(
                    "https://demo-extracts.deadlock-api.com/jobs/y/result.ndjson",
                    "application/x-ndjson",
                    b"{\"match_id\":92685682,\"account_id\":281768392,\"tick\":10}\n",
                ),
            }],
            &policy(),
            Some(&first.checkpoint),
        )
        .unwrap();
        assert_eq!(second.expected_generation, first.checkpoint.generation);
        assert_eq!(second.records.len(), 2);
        assert_eq!(second.records[0].revision, 2);
        assert!(!second.records[0].tombstone);
        assert_ne!(
            second.records[0].metadata["http_body_sha256"],
            first.records[0].metadata["http_body_sha256"]
        );
        assert!(second.records[1].tombstone);
        assert_eq!(
            second.records[1].logical_id,
            "match/92685682/demo/player_state/000002"
        );
        assert_eq!(second.records[1].visibility, SourceVisibility::Private);
        assert_eq!(second.records[1].metadata["account_id"], "281768392");
    }
}
