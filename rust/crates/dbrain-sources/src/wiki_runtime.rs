//! C5 opt-in composition: trusted parser -> shared IR -> existing SourceStore /
//! PgStore -> reviewed NumericFact / immutable CorpusRelease. No production DSN,
//! default writer, model, Lua VM, template evaluator or latest-release switch.
use crate::{wiki_capture_io::stage_sources_with_pool, Result, SourcesError};
use brain_contracts::{
    domain::{
        DomainObject, NumericFact, Quantity, StoredDomainObject, Validity, DOMAIN_CONTRACT_VERSION,
    },
    source::{origin_from_record, Versioned},
    wiki::IrValue,
    CorpusRelease, DocumentRevision, DocumentStorePort, SourceBatch, SourceCheckpoint,
    SourceRecordV2,
};
use brain_storage::{ApplyOutcome, PgStore};
use dbrain_s12_wiki_probe::{
    knowledge::{ProjectionReview, WikiIr},
    model::PARSER_VERSION,
    sha256,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    time::Duration,
};

fn invalid(message: impl Into<String>) -> SourcesError {
    SourcesError::invalid_input(message)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseRequest {
    pub release_id: String,
    pub knowledge_version: String,
    /// Operator-assigned pilot validity. Never inferred from a wiki timestamp.
    pub validity: Validity,
    pub created_at_epoch: i64,
    /// No implicit approval. Empty/absent review produces IR + raw, zero facts.
    pub review: Option<ProjectionReview>,
}
#[derive(Debug, Serialize)]
pub struct WikiReleasePlan {
    pub release: CorpusRelease,
    pub raw_records: Vec<SourceRecordV2>,
    pub ir_records: Vec<SourceRecordV2>,
    pub fact_records: Vec<SourceRecordV2>,
    pub withheld_fields: BTreeMap<String, String>,
}
fn stable(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 512 && !s.chars().any(char::is_control)
}
fn revision(source: &SourceRecordV2) -> DocumentRevision {
    DocumentRevision {
        source_id: source.source_id.clone(),
        logical_id: source.logical_id.clone(),
        revision: source.revision,
        content_hash: source.content_hash.clone(),
    }
}

/// This adapter intentionally refuses decimals not exactly representable by the
/// existing f64 Domain contract. The exact decimal remains in the common Wiki IR.
fn exact_binary_decimal(text: &str) -> Option<f64> {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (whole, frac) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let mut numerator: u128 = format!("{whole}{frac}").parse().ok()?;
    let mut denominator = 10u128.checked_pow(frac.len().try_into().ok()?)?;
    while denominator % 5 == 0 {
        if !numerator.is_multiple_of(5) {
            return None;
        }
        numerator /= 5;
        denominator /= 5;
    }
    while numerator != 0 && numerator.is_multiple_of(2) {
        numerator /= 2;
    }
    if numerator > (1u128 << 53) {
        return None;
    }
    let value: f64 = text.parse().ok()?;
    value.is_finite().then_some(value)
}
fn derive(
    raw: &SourceRecordV2,
    content: String,
    kind: &str,
    metadata: BTreeMap<String, String>,
) -> Result<SourceRecordV2> {
    let mut origin = origin_from_record(raw).map_err(invalid)?;
    let mut record = raw.clone();
    let generation = sha256(&serde_json::to_vec(&(
        &content,
        &metadata,
        &origin.policy,
        PARSER_VERSION,
    ))?);
    record.logical_id = format!("{}:{kind}:{generation}", raw.logical_id);
    record.content_hash = sha256(content.as_bytes());
    record.content = content;
    record.metadata = metadata;
    origin.origin_artifacts.insert(format!(
        "{}:r{}:{}",
        raw.logical_id, raw.revision, raw.content_hash
    ));
    origin.identity.logical_id = record.logical_id.clone();
    origin.raw_sha256 = record.content_hash.clone();
    origin.parser_revision = PARSER_VERSION.into();
    origin.derivation_family = brain_contracts::value::Observed::known(format!("wiki-{kind}"));
    origin.bind_record(&mut record).map_err(invalid)?;
    Ok(record)
}

/// Build only from the non-deserializable, trusted WikiIr extraction capability.
/// Stored first-observation raw records are substituted only after validation in stage.
pub fn plan_release(ir: &WikiIr, request: &ReleaseRequest) -> Result<WikiReleasePlan> {
    plan_with_sources(ir, request, ir.sources().to_vec())
}
fn plan_with_sources(
    ir: &WikiIr,
    request: &ReleaseRequest,
    raw_records: Vec<SourceRecordV2>,
) -> Result<WikiReleasePlan> {
    if [
        &request.release_id,
        &request.knowledge_version,
        &request.validity.patch,
        &request.validity.mode,
    ]
    .iter()
    .any(|s| !stable(s))
        || request.created_at_epoch <= 0
        || !ir.report().discovery_complete
        || ir.report().discovery_scope.is_none()
        || !ir.report().source_policy.raw_retention_allowed
    {
        return Err(invalid(
            "scoped complete capture, retention approval and explicit pilot release required",
        ));
    }
    if request
        .review
        .as_ref()
        .is_some_and(|r| !stable(&r.decision_ref))
    {
        return Err(invalid("empty review decision"));
    }
    let mut heads = BTreeMap::<String, &SourceRecordV2>::new();
    for source in &raw_records {
        source
            .validate()
            .map_err(|_| invalid("invalid raw source"))?;
        origin_from_record(source).map_err(invalid)?;
        let head = heads.entry(source.logical_id.clone()).or_insert(source);
        if source.revision > head.revision {
            *head = source;
        }
    }
    // Missing/denied/suppressed pages must not be replaced with an older revision.
    for page in &ir.report().pages {
        if heads
            .get(&page.source_id)
            .is_none_or(|r| Some(r.revision as i64) != page.latest_revision)
        {
            return Err(invalid(
                "selected head unavailable; no historical fallback or partial release",
            ));
        }
    }
    let mut ir_records = Vec::new();
    for page in &ir.report().pages {
        let raw = heads[&page.source_id];
        let mut data = ir.contract().data.clone();
        data.sources = raw_records
            .iter()
            .filter(|s| s.logical_id == page.source_id)
            .cloned()
            .collect();
        data.artifacts = data
            .sources
            .iter()
            .map(origin_from_record)
            .collect::<std::result::Result<_, _>>()
            .map_err(invalid)?;
        data.fields
            .retain(|f| f.source_revision.logical_id == page.source_id);
        data.aliases.retain(|a| a.source.page_id == page.page_id);
        data.entities.retain(|id, _| *id == page.page_id);
        data.dependencies
            .retain(|d| d.dependent.logical_id == page.source_id);
        data.dependency_completeness
            .retain(|id, _| id == &page.source_id);
        // Preserve the exact dependency closure heads in metadata. A changed
        // template invalidates just its dependants, not unrelated pages.
        let mut deps = BTreeMap::new();
        let mut pending: Vec<_> = page.dependency_ids.iter().copied().collect();
        let mut visited = BTreeSet::from([page.page_id]);
        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }
            if let Some(p) = ir.report().pages.iter().find(|p| p.page_id == id) {
                if let Some(raw) = heads.get(&p.source_id) {
                    deps.insert(p.source_id.clone(), revision(raw));
                }
                pending.extend(&p.dependency_ids);
            }
        }
        let metadata = BTreeMap::from([
            ("wiki_ir_contract".into(), "brain.ir.v1".into()),
            ("parser_revision".into(), PARSER_VERSION.into()),
            ("dependencies".into(), serde_json::to_string(&deps)?),
            ("canonical_publication".into(), "false".into()),
        ]);
        ir_records.push(derive(
            raw,
            serde_json::to_string(&Versioned::new(data))?,
            "ir",
            metadata,
        )?);
    }
    let mut fact_records = Vec::new();
    let mut withheld_fields = BTreeMap::new();
    let mut predicates = BTreeSet::new();
    for field in ir.fields() {
        let reason = if request
            .review
            .as_ref()
            .is_none_or(|r| r.approved_fields.get(&field.id) != Some(&field.source_revision))
        {
            Some("unreviewed_exact_revision")
        } else if !field.unknowns.is_empty() || field.condition.is_some() || field.variant.is_some()
        {
            Some("unknown_or_context_preserved_only_in_common_ir")
        } else {
            None
        };
        if let Some(reason) = reason {
            withheld_fields.insert(field.id.clone(), reason.into());
            continue;
        }
        let review = request.review.as_ref().unwrap();
        let raw = heads
            .get(&field.source_revision.logical_id)
            .ok_or_else(|| invalid("field without source"))?;
        if revision(raw) != field.source_revision {
            return Err(invalid("field provenance differs from stored raw"));
        }
        let page = ir
            .report()
            .pages
            .iter()
            .find(|p| p.source_id == raw.logical_id)
            .ok_or_else(|| invalid("missing page"))?;
        let mut closure = vec![page.page_id];
        let mut visited = BTreeSet::new();
        let mut reviewed_closure = true;
        while let Some(id) = closure.pop() {
            if !visited.insert(id) {
                continue;
            }
            let p = ir
                .report()
                .pages
                .iter()
                .find(|p| p.page_id == id)
                .ok_or_else(|| invalid("dependency not captured"))?;
            let dep = heads
                .get(&p.source_id)
                .ok_or_else(|| invalid("dependency head unavailable"))?;
            reviewed_closure &= p.previewable()
                && p.dependencies_complete
                && review.source_revisions.get(&p.source_id) == Some(&revision(dep));
            closure.extend(&p.dependency_ids);
        }
        if !reviewed_closure {
            withheld_fields.insert(
                field.id.clone(),
                "dependency_closure_not_exactly_reviewed".into(),
            );
            continue;
        }
        let IrValue::Quantity { decimal, unit } = &field.value else {
            withheld_fields.insert(
                field.id.clone(),
                "non_numeric_value_preserved_in_common_ir".into(),
            );
            continue;
        };
        let Some(value) = exact_binary_decimal(decimal) else {
            withheld_fields.insert(field.id.clone(), "decimal_not_exact_in_domain_f64".into());
            continue;
        };
        if !predicates.insert((&field.subject_id, &field.predicate)) {
            return Err(invalid("conflicting reviewed predicate"));
        }
        let fact = NumericFact {
            fact_id: format!("{}:r{}:{}", field.id, raw.revision, raw.content_hash),
            subject_id: field.subject_id.clone(),
            predicate: field.predicate.clone(),
            quantity: Quantity {
                value,
                unit: unit.as_str().into(),
            },
            validity: request.validity.clone(),
            source: field.source_revision.clone(),
            verified: true,
        };
        let reviewed_sources: BTreeMap<_, _> = ir
            .report()
            .pages
            .iter()
            .filter(|p| visited.contains(&p.page_id))
            .map(|p| (p.source_id.clone(), revision(heads[&p.source_id])))
            .collect();
        let metadata = BTreeMap::from([
            ("domain_contract".into(), DOMAIN_CONTRACT_VERSION.into()),
            ("wiki_field".into(), serde_json::to_string(field)?),
            ("review_ref".into(), review.decision_ref.clone()),
            (
                brain_contracts::domain::DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
                serde_json::to_string(&Versioned::new(
                    reviewed_sources.values().cloned().collect::<Vec<_>>(),
                ))?,
            ),
            (
                "reviewed_sources".into(),
                serde_json::to_string(&reviewed_sources)?,
            ),
        ]);
        fact_records.push(derive(
            raw,
            serde_json::to_string(&StoredDomainObject {
                contract_version: DOMAIN_CONTRACT_VERSION.into(),
                object: DomainObject::NumericFact(fact),
            })?,
            "fact",
            metadata,
        )?);
    }
    let mut source_revisions = BTreeMap::<String, BTreeMap<String, u64>>::new();
    for record in heads
        .values()
        .copied()
        .chain(ir_records.iter())
        .chain(fact_records.iter())
    {
        source_revisions
            .entry(record.source_id.clone())
            .or_default()
            .insert(record.logical_id.clone(), record.revision);
    }
    Ok(WikiReleasePlan {
        release: CorpusRelease {
            release_id: request.release_id.clone(),
            knowledge_version: request.knowledge_version.clone(),
            patch: request.validity.patch.clone(),
            created_at_epoch: request.created_at_epoch,
            source_revisions,
        },
        raw_records,
        ir_records,
        fact_records,
        withheld_fields,
    })
}

/// Dedicated scratch-only handle. Construction proves socket, role, DB and server
/// data_directory BEFORE any DDL/write. No arbitrary DSN or global pg_pool fallback.
pub struct ScratchWikiStore {
    pool: PgPool,
    raw_dir: PathBuf,
}
impl ScratchWikiStore {
    pub async fn connect(socket: &Path, raw_dir: &Path) -> Result<Self> {
        let socket = socket.canonicalize()?;
        let marker = socket.join("C5_SCRATCH_ONLY");
        let meta = std::fs::symlink_metadata(&marker)?;
        if !meta.is_file() || std::fs::read_to_string(&marker)? != "C5_SCRATCH_ONLY\n" {
            return Err(invalid("dedicated C5 scratch marker required"));
        }
        let data = socket.join("pg").canonicalize()?;
        let options = PgConnectOptions::new()
            .host(socket.to_str().ok_or_else(|| invalid("non-UTF8 socket"))?)
            .port(55441)
            .username("brain_wiki_c5")
            .database("brain_wiki_c5")
            .password("")
            .ssl_mode(PgSslMode::Disable)
            .application_name("brain-wiki-c5-scratch");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(options)
            .await?;
        let safe: bool = sqlx::query_scalar("SELECT current_database() = 'brain_wiki_c5' AND current_user = 'brain_wiki_c5' AND inet_server_addr() IS NULL AND current_setting('data_directory') = $1")
            .bind(data.to_string_lossy().as_ref()).fetch_one(&pool).await?;
        if !safe {
            pool.close().await;
            return Err(invalid("not the dedicated C5 scratch database"));
        }
        Ok(Self {
            pool,
            raw_dir: raw_dir.into(),
        })
    }
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    pub async fn migrate(&self) -> Result<()> {
        PgStore::new(self.pool.clone())
            .migrate_core()
            .await
            .map_err(|_| invalid("scratch core migration failed"))?;
        // Minimal legacy SourceStore schema for a NEW scratch cluster. These are
        // the existing source_runs/source_documents contracts, not a new facts DB.
        sqlx::raw_sql(include_str!("wiki_scratch.sql"))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn stage(&self, ir: &WikiIr, request: &ReleaseRequest) -> Result<Value> {
        // Serializes this source across capture/delta/reparse runs. The lock
        // transaction itself performs no production writes and releases on error.
        let mut guard = self.pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(format!("wiki-c5:{}", ir.report().source_key))
            .execute(&mut *guard)
            .await?;
        for page in &ir.report().pages {
            let current: Option<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
                .bind(&ir.report().source_key).bind(&page.source_id).fetch_optional(&self.pool).await?;
            if let Some(value) = current {
                let current: SourceRecordV2 = serde_json::from_value(value)?;
                let incoming = ir
                    .sources()
                    .iter()
                    .find(|s| {
                        s.logical_id == page.source_id
                            && Some(s.revision as i64) == page.latest_revision
                    })
                    .ok_or_else(|| invalid("incoming current head unavailable"))?;
                if current.tombstone
                    || current.revision > incoming.revision
                    || origin_from_record(&current).map_err(invalid)?.policy
                        != origin_from_record(incoming).map_err(invalid)?.policy
                {
                    return Err(invalid(
                        "stale capture or current source rights changed; no raw restaging",
                    ));
                }
            }
        }
        let mut canonical = Vec::new();
        for source in ir.sources() {
            let stored: Option<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
                .bind(&source.source_id).bind(&source.logical_id).bind(source.revision as i64).fetch_optional(&self.pool).await?;
            let record = if let Some(stored) = stored {
                let old: SourceRecordV2 = serde_json::from_value(stored)?;
                let previous = origin_from_record(&old).map_err(invalid)?;
                let current = origin_from_record(source).map_err(invalid)?;
                if old.content != source.content
                    || old.content_hash != source.content_hash
                    || previous.policy != current.policy
                    || previous.source_revision != current.source_revision
                    || previous.source_time != current.source_time
                    || previous.language != current.language
                    || previous.locator != current.locator
                {
                    return Err(invalid(
                        "immutable Wiki revision/policy conflict; explicit reconciliation required",
                    ));
                }
                // Preserve first observation. Reparse creates a new IR generation;
                // it must not rewrite parser metadata on immutable raw revisions.
                old
            } else {
                source.clone()
            };
            canonical.push(record);
        }
        let plan = plan_with_sources(ir, request, canonical)?;
        // Validate release identity BEFORE staging, not after partially importing
        // an incompatible invocation under an existing immutable release name.
        let existing: Option<Value> = sqlx::query_scalar(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(&plan.release.release_id)
        .fetch_optional(&self.pool)
        .await?;
        if existing.is_some_and(|value| value != serde_json::to_value(&plan.release).unwrap()) {
            return Err(invalid(
                "immutable release conflict; choose a new explicit release ID",
            ));
        }
        let raw = stage_sources_with_pool(&self.pool, &self.raw_dir, ir).await?;
        let store = PgStore::new(self.pool.clone());
        let mut inserted = 0usize;
        let mut unchanged = 0usize;
        let mut raw_records: Vec<_> = plan.raw_records.iter().collect();
        raw_records.sort_by_key(|r| (&r.source_id, &r.logical_id, r.revision));
        for record in raw_records
            .into_iter()
            .chain(plan.ir_records.iter())
            .chain(plan.fact_records.iter())
        {
            match store
                .apply(record)
                .await
                .map_err(|_| invalid("canonical store write failed"))?
            {
                ApplyOutcome::Inserted | ApplyOutcome::Updated => inserted += 1,
                ApplyOutcome::Unchanged => unchanged += 1,
                ApplyOutcome::IgnoredStale => {
                    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3)")
                        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_one(&self.pool).await?;
                    if !exists {
                        return Err(invalid(
                            "older history missing from canonical store; no partial release",
                        ));
                    }
                    unchanged += 1;
                }
                ApplyOutcome::Tombstoned => {
                    return Err(invalid("unexpected tombstone in scoped capture"))
                }
            }
        }
        store
            .publish_release(&plan.release)
            .await
            .map_err(|_| invalid("scratch release publication failed"))?;
        let snapshot = store
            .snapshot(&plan.release.release_id)
            .await
            .map_err(|_| invalid("release readback failed"))?;
        if snapshot.release != plan.release {
            return Err(invalid("release readback mismatch"));
        }
        guard.commit().await?;
        Ok(
            json!({"raw_staging":raw,"inserted_records":inserted,"unchanged_records":unchanged,
            "ir_records":plan.ir_records.len(),"fact_records":plan.fact_records.len(),
            "withheld_fields":plan.withheld_fields,"release":plan.release,
            "scope":ir.report().discovery_scope,"canonical_publication":false,
            "scratch_release_written":true,"provider_calls":0}),
        )
    }
}

pub const CORE_STAGE_CONFIGURATION: &str = "brain-wiki-core-stage.v1";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiCoreCheckpoint {
    pub configuration: String,
    pub documents: BTreeMap<String, BTreeMap<u64, String>>,
}

pub async fn stage_into_store<S: DocumentStorePort + ?Sized>(
    store: &S,
    ir: &WikiIr,
    request: &ReleaseRequest,
    owner: &str,
) -> Result<Value> {
    let plan = plan_release(ir, request)?;
    let mut records: Vec<&SourceRecordV2> = plan
        .raw_records
        .iter()
        .chain(plan.ir_records.iter())
        .chain(plan.fact_records.iter())
        .collect();
    let source_id = records
        .first()
        .map(|r| r.source_id.clone())
        .ok_or_else(|| invalid("empty wiki plan"))?;
    if records.iter().any(|r| r.source_id != source_id) {
        return Err(invalid("wiki plan spans several sources"));
    }
    let previous = store
        .checkpoint(&source_id)
        .await
        .map_err(|_| invalid("checkpoint unavailable"))?;
    let (mut generation, mut state) = match &previous {
        None => (
            0,
            WikiCoreCheckpoint {
                configuration: CORE_STAGE_CONFIGURATION.into(),
                documents: BTreeMap::new(),
            },
        ),
        Some(cp) => {
            let state: WikiCoreCheckpoint = serde_json::from_value(cp.state.clone())?;
            if cp.configuration != CORE_STAGE_CONFIGURATION
                || state.configuration != CORE_STAGE_CONFIGURATION
            {
                return Err(invalid("foreign checkpoint for wiki source"));
            }
            (cp.generation, state)
        }
    };
    let mut incoming_heads = BTreeMap::<&str, u64>::new();
    for raw in &plan.raw_records {
        let head = incoming_heads
            .entry(&raw.logical_id)
            .or_insert(raw.revision);
        *head = (*head).max(raw.revision);
    }
    for (logical_id, head) in &incoming_heads {
        if state
            .documents
            .get(*logical_id)
            .and_then(|h| h.keys().next_back())
            .is_some_and(|stored| stored > head)
        {
            return Err(invalid(
                "stale capture: a newer Wiki revision is already stored",
            ));
        }
    }
    records.sort_by_key(|r| (r.logical_id.clone(), r.revision));
    let mut pending: BTreeMap<&str, Vec<&SourceRecordV2>> = BTreeMap::new();
    let mut unchanged = 0usize;
    for record in records {
        let history = state.documents.get(&record.logical_id);
        match history.and_then(|h| h.get(&record.revision)) {
            Some(hash) if hash == &record.content_hash => {
                unchanged += 1;
                continue;
            }
            Some(_) => {
                return Err(invalid(
                    "immutable Wiki revision conflict; explicit reconciliation required",
                ))
            }
            None => {}
        }
        if history
            .and_then(|h| h.keys().next_back())
            .is_some_and(|head| *head > record.revision)
        {
            return Err(invalid(
                "stale capture or missing older history; no partial release",
            ));
        }
        pending.entry(&record.logical_id).or_default().push(record);
    }
    let rounds = pending.values().map(Vec::len).max().unwrap_or(0);
    let mut committed = 0usize;
    for round in 0..rounds {
        let batch_records: Vec<SourceRecordV2> = pending
            .values()
            .filter_map(|list| list.get(round).map(|r| (*r).clone()))
            .collect();
        for record in &batch_records {
            state
                .documents
                .entry(record.logical_id.clone())
                .or_default()
                .insert(record.revision, record.content_hash.clone());
        }
        let batch = SourceBatch {
            expected_generation: generation,
            checkpoint: SourceCheckpoint {
                source_id: source_id.clone(),
                configuration: CORE_STAGE_CONFIGURATION.into(),
                generation: generation
                    .checked_add(1)
                    .ok_or_else(|| invalid("generation exhausted"))?,
                state: serde_json::to_value(&state)?,
            },
            records: batch_records,
        };
        batch
            .validate()
            .map_err(|e| invalid(format!("invalid wiki batch: {e:?}")))?;
        let lease = store
            .claim(&source_id, owner, 60_000)
            .await
            .map_err(|e| invalid(format!("lease unavailable: {e:?}")))?;
        store
            .commit(&batch, &lease)
            .await
            .map_err(|e| invalid(format!("canonical store commit failed: {e:?}")))?;
        committed += batch.records.len();
        generation = batch.checkpoint.generation;
    }
    store
        .publish(&plan.release)
        .await
        .map_err(|e| invalid(format!("release publication failed: {e:?}")))?;
    Ok(json!({
        "source_id": source_id,
        "committed_records": committed,
        "unchanged_records": unchanged,
        "batches": rounds,
        "raw_records": plan.raw_records.len(),
        "ir_records": plan.ir_records.len(),
        "fact_records": plan.fact_records.len(),
        "withheld_fields": plan.withheld_fields,
        "release": plan.release,
        "second_store": false,
        "provider_calls": 0,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn domain_numeric_projection_is_exact_or_withheld() {
        assert_eq!(exact_binary_decimal("0"), Some(0.0));
        assert_eq!(exact_binary_decimal("-12.5"), Some(-12.5));
        assert_eq!(exact_binary_decimal("0.125"), Some(0.125));
        assert_eq!(exact_binary_decimal("0.1"), None);
        assert_eq!(exact_binary_decimal("9007199254740993"), None);
    }
}
