use crate::*;
use brain_contracts::{
    CorpusRelease, DocumentStorePort, Principal, SourceRecordV2, SourceVisibility,
    source::origin_from_record,
};
use brain_ingestion::document_set::{
    CoreDocument, DocumentSetSource, MAX_DOCUMENT_BYTES, current_pins, prepare_document_batch,
};
use brain_storage::PgStore;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, ReplayFailure>;

#[derive(Debug, Serialize)]
pub struct ImportReceipt {
    pub source_id: String,
    pub release_id: String,
    #[serde(serialize_with = "serialize_outcome")]
    pub outcome: ObservationCommit,
    pub changed_records: usize,
    pub active_observations: usize,
}

fn serialize_outcome<S: serde::Serializer>(
    outcome: &ObservationCommit,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(match outcome {
        ObservationCommit::Inserted => "inserted",
        ObservationCommit::DuplicateGeneration => "duplicate_generation",
        ObservationCommit::DuplicateMatch => "duplicate_match",
        ObservationCommit::Reparsed => "reparsed",
    })
}

pub fn source_identity(report: &ReplayReport) -> String {
    let identity = report
        .artifact
        .source
        .match_reference
        .as_ref()
        .map_or(report.artifact.sha256.as_str(), |m| m.match_id.as_str());
    let kind = if report.artifact.source.match_reference.is_some() {
        "match"
    } else {
        "raw"
    };
    format!(
        "replay:{}",
        hash_parts(&[
            report.artifact.source.rights.scope.as_bytes(),
            kind.as_bytes(),
            identity.as_bytes()
        ])
    )
}

pub fn validate_import(report: &ReplayReport, request: &ReplayRequest) -> Result<()> {
    validate_request(request)?;
    if request.source.expected_sha256.as_ref() != Some(&report.artifact.sha256)
        || report.artifact.byte_length < 16
        || report.artifact.byte_length > request.budget.max_file_bytes
        || report.observations.is_empty()
    {
        return Err(ReplayFailure::InvalidWorkerOutput);
    }
    let mut normalized = request.clone();
    normalized.selection.entity_classes.sort();
    let artifact = ReplayArtifact {
        source: normalized.source.clone(),
        sha256: report.artifact.sha256.clone(),
        byte_length: report.artifact.byte_length,
    };
    crate::supervisor::validate_report(report, &artifact, &normalized)?;
    if serde_json::to_vec(report)
        .map_err(|_| ReplayFailure::InvalidWorkerOutput)?
        .len() as u64
        > request.budget.max_output_bytes
    {
        return Err(ReplayFailure::BudgetExceeded);
    }
    Ok(())
}

fn document(report: &ReplayReport, logical_id: String, content: String) -> CoreDocument {
    let mut origin = report.origin_artifact();
    origin
        .origin_artifacts
        .insert(report.artifact.sha256.clone());
    CoreDocument {
        logical_id,
        content,
        metadata: BTreeMap::from([
            ("replay_raw_sha256".into(), report.artifact.sha256.clone()),
            ("replay_generation_id".into(), report.generation_id.clone()),
            (
                "replay_contract_version".into(),
                report.contract_version.clone(),
            ),
            (
                "replay_selection".into(),
                serde_json::to_string(&report.selection).expect("serializable selection"),
            ),
            (
                "replay_match_reference".into(),
                serde_json::to_string(&report.artifact.source.match_reference)
                    .expect("serializable match"),
            ),
            ("egress".into(), "none".into()),
        ]),
        origin,
    }
}

pub fn report_documents(report: &ReplayReport) -> Result<Vec<CoreDocument>> {
    let mut header = report.clone();
    header.observations.clear();
    let mut documents = vec![document(
        report,
        "report".into(),
        serde_json::to_string(&header).map_err(|_| ReplayFailure::InvalidWorkerOutput)?,
    )];
    let mut chunk = String::from("[");
    let mut index = 0;
    let mut count = 0;
    for observation in &report.observations {
        let encoded =
            serde_json::to_string(observation).map_err(|_| ReplayFailure::InvalidWorkerOutput)?;
        if encoded.len() + 2 > MAX_DOCUMENT_BYTES {
            return Err(ReplayFailure::BudgetExceeded);
        }
        if chunk.len() + encoded.len() + usize::from(count > 0) + 1 > MAX_DOCUMENT_BYTES {
            chunk.push(']');
            documents.push(document(report, format!("observations/{index:06}"), chunk));
            chunk = String::from("[");
            index += 1;
            count = 0;
        }
        if count > 0 {
            chunk.push(',');
        }
        chunk.push_str(&encoded);
        count += 1;
    }
    if count > 0 {
        chunk.push(']');
        documents.push(document(report, format!("observations/{index:06}"), chunk));
    }
    Ok(documents)
}

fn source(report: &ReplayReport) -> DocumentSetSource {
    DocumentSetSource {
        source_id: source_identity(report),
        configuration: "brain.replay.store.v1".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from([report.artifact.source.rights.scope.clone()]),
        tombstone_metadata: BTreeMap::from([("egress".into(), "none".into())]),
    }
}

fn release(source_id: &str, pins: BTreeMap<String, u64>) -> Result<CorpusRelease> {
    let source_revisions = BTreeMap::from([(source_id.to_owned(), pins)]);
    let digest =
        hash(&serde_json::to_vec(&source_revisions).map_err(|_| ReplayFailure::StoreFailure)?);
    Ok(CorpusRelease {
        release_id: format!("replay:{digest}"),
        knowledge_version: "brain.replay.store.v1".into(),
        patch: "unknown".into(),
        created_at_epoch: 0,
        source_revisions,
    })
}

fn preserve(record: &SourceRecordV2) -> Result<CoreDocument> {
    Ok(CoreDocument {
        logical_id: record.logical_id.clone(),
        content: record.content.clone(),
        metadata: record.metadata.clone(),
        origin: origin_from_record(record).map_err(|_| ReplayFailure::StoreFailure)?,
    })
}

pub async fn import_report(
    pool: &PgPool,
    report: &ReplayReport,
    request: &ReplayRequest,
) -> Result<ImportReceipt> {
    validate_import(report, request)?;
    let source = source(report);
    let store = PgStore::new(pool.clone());
    let lease = store
        .claim(
            &source.source_id,
            &format!("replay-import:{}", std::process::id()),
            60_000,
        )
        .await
        .map_err(|_| ReplayFailure::StoreFailure)?;
    let result = import_leased(pool, &store, &lease, &source, report).await;
    let released = sqlx::query("UPDATE brain.source_jobs_v1 SET state='idle',lease_until=clock_timestamp() WHERE source_id=$1 AND owner=$2 AND fence=$3 AND state='running'")
        .bind(&lease.source_id).bind(&lease.owner).bind(lease.fence as i64).execute(pool).await;
    if released.is_err() && result.is_ok() {
        return Err(ReplayFailure::StoreFailure);
    }
    result
}

async fn import_leased(
    pool: &PgPool,
    store: &PgStore,
    lease: &brain_contracts::Lease,
    source: &DocumentSetSource,
    report: &ReplayReport,
) -> Result<ImportReceipt> {
    let previous = store
        .checkpoint(&source.source_id)
        .await
        .map_err(|_| ReplayFailure::StoreFailure)?;
    let rows: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 ORDER BY logical_id",
    )
    .bind(&source.source_id)
    .fetch_all(pool)
    .await
    .map_err(|_| ReplayFailure::StoreFailure)?;
    let mut heads: BTreeMap<String, SourceRecordV2> = rows
        .into_iter()
        .map(|v| {
            let record: SourceRecordV2 =
                serde_json::from_value(v).map_err(|_| ReplayFailure::StoreFailure)?;
            record.validate().map_err(|_| ReplayFailure::StoreFailure)?;
            Ok((record.logical_id.clone(), record))
        })
        .collect::<Result<_>>()?;
    let active: Option<ReplayReport> = heads
        .get("report")
        .filter(|r| !r.tombstone)
        .map(|r| serde_json::from_str(&r.content).map_err(|_| ReplayFailure::StoreFailure))
        .transpose()?;
    let outcome = active.as_ref().map_or(ObservationCommit::Inserted, |old| {
        classify_replay_duplicate(
            &ReplayDedupKeys::from_report(report),
            &ReplayDedupKeys::from_report(old),
        )
    });
    let keep = matches!(
        outcome,
        ObservationCommit::DuplicateGeneration | ObservationCommit::DuplicateMatch
    );
    let mut documents = heads
        .values()
        .filter(|r| !r.tombstone && (r.logical_id.starts_with("raw/") || keep))
        .map(preserve)
        .collect::<Result<Vec<_>>>()?;
    if !keep {
        documents.extend(report_documents(report)?);
    }
    let artifact =
        serde_json::to_string(&report.artifact).map_err(|_| ReplayFailure::InvalidWorkerOutput)?;
    let mut stable_artifact = report.artifact.clone();
    stable_artifact.source.retrieved_at_unix_ms = None;
    let evidence_id = format!(
        "raw/{}",
        hash(
            &serde_json::to_vec(&stable_artifact)
                .map_err(|_| ReplayFailure::InvalidWorkerOutput)?
        )
    );
    if !documents.iter().any(|d| d.logical_id == evidence_id) {
        documents.push(document(report, evidence_id, artifact));
    }
    let batch = prepare_document_batch(source, &documents, previous.as_ref())
        .map_err(|_| ReplayFailure::StoreFailure)?;
    let checkpoint = if batch.records.is_empty() {
        previous.as_ref().ok_or(ReplayFailure::StoreFailure)?
    } else {
        &batch.checkpoint
    };
    let release = release(
        &source.source_id,
        current_pins(checkpoint).map_err(|_| ReplayFailure::StoreFailure)?,
    )?;
    for record in &batch.records {
        heads.insert(record.logical_id.clone(), record.clone());
    }
    let expected: Vec<_> = heads.into_values().collect();
    if batch.records.is_empty() {
        let snapshot = store
            .snapshot(&release.release_id)
            .await
            .map_err(|_| ReplayFailure::StoreFailure)?;
        if snapshot.release != release {
            return Err(ReplayFailure::StoreFailure);
        }
    } else {
        store
            .commit_batches_and_publish_checked(&[(&batch, lease)], &release, &expected)
            .await
            .map_err(|_| ReplayFailure::StoreFailure)?;
    }
    let active_observations = documents
        .iter()
        .filter(|d| d.logical_id.starts_with("observations/"))
        .try_fold(0usize, |count, d| {
            let observations: Vec<ReplayObservation> =
                serde_json::from_str(&d.content).map_err(|_| ReplayFailure::StoreFailure)?;
            Ok::<_, ReplayFailure>(count + observations.len())
        })?;
    Ok(ImportReceipt {
        source_id: source.source_id.clone(),
        release_id: release.release_id,
        outcome,
        changed_records: batch.records.len(),
        active_observations,
    })
}

#[derive(Debug, Serialize)]
pub struct InternalExample {
    pub release_id: String,
    pub source_id: String,
    pub revision: u64,
    pub generation_id: String,
    pub parser_revision: String,
    pub schema_revision: String,
    pub extraction_revision: String,
    pub match_reference: Option<MatchReference>,
    pub provenance: brain_contracts::replay::ObservationProvenance,
    pub observation: ReplayObservation,
}

pub async fn query_example(
    pool: &PgPool,
    release_id: &str,
    principal: &Principal,
) -> Result<InternalExample> {
    let snapshot = PgStore::new(pool.clone())
        .snapshot(release_id)
        .await
        .map_err(|_| ReplayFailure::StoreFailure)?;
    let records = snapshot
        .authorized(principal, false)
        .map_err(|_| ReplayFailure::StoreFailure)?;
    let header = records
        .iter()
        .find(|r| r.logical_id == "report")
        .ok_or(ReplayFailure::RightsDenied)?;
    let mut report: ReplayReport =
        serde_json::from_str(&header.content).map_err(|_| ReplayFailure::StoreFailure)?;
    let record = records
        .iter()
        .filter(|r| r.source_id == header.source_id && r.logical_id.starts_with("observations/"))
        .min_by_key(|r| &r.logical_id)
        .ok_or(ReplayFailure::RightsDenied)?;
    let observation: ReplayObservation =
        serde_json::from_str::<Vec<ReplayObservation>>(&record.content)
            .map_err(|_| ReplayFailure::StoreFailure)?
            .into_iter()
            .next()
            .ok_or(ReplayFailure::StoreFailure)?;
    let identity = serde_json::to_vec(&(
        &observation.raw,
        &observation.time,
        &observation.entity,
        &observation.event,
    ))
    .map_err(|_| ReplayFailure::StoreFailure)?;
    if record.source_id != source_identity(&report)
        || [header, record].iter().any(|r| {
            r.metadata.get("replay_generation_id") != Some(&report.generation_id)
                || r.metadata.get("replay_raw_sha256") != Some(&report.artifact.sha256)
        })
        || observation.observation_id != hash_parts(&[report.generation_id.as_bytes(), &identity])
    {
        return Err(ReplayFailure::StoreFailure);
    }
    report.observations.push(observation.clone());
    let mut provenance = report
        .observation_provenance(&observation.observation_id)
        .ok_or(ReplayFailure::StoreFailure)?;
    provenance.origin.policy = origin_from_record(record)
        .map_err(|_| ReplayFailure::StoreFailure)?
        .policy;
    Ok(InternalExample {
        release_id: release_id.into(),
        source_id: record.source_id.clone(),
        revision: record.revision,
        generation_id: report.generation_id.clone(),
        parser_revision: report.parser_revision.clone(),
        schema_revision: report.schema_revision.clone(),
        extraction_revision: report.extraction_revision.clone(),
        match_reference: report.artifact.source.match_reference.clone(),
        provenance,
        observation,
    })
}
