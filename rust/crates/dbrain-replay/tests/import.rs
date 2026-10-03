mod common;
use brain_contracts::{Principal, source::origin_from_record};
use brain_ingestion::document_set::{DocumentSetSource, prepare_document_batch};
use brain_storage::PgStore;
use common::*;
use dbrain_replay::{import::*, *};
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::collections::{BTreeMap, BTreeSet};

fn pinned(bytes: &[u8], match_id: &str) -> ReplayRequest {
    let mut request = request();
    request.source.expected_sha256 = Some(format!("{:x}", Sha256::digest(bytes)));
    request.source.match_reference = Some(MatchReference {
        match_id: match_id.into(),
        evidence_ref: "synthetic-authorship".into(),
    });
    request
}

#[test]
fn adapter_preserves_raw_and_unknowns_without_publication_or_egress() {
    let bytes = minimal();
    let request = pinned(&bytes, "adapter-test");
    let report = decode(&bytes, &request).unwrap();
    validate_import(&report, &request).unwrap();
    let source = DocumentSetSource {
        source_id: source_identity(&report),
        configuration: "test".into(),
        visibility: brain_contracts::SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from(["test-only".into()]),
        tombstone_metadata: BTreeMap::new(),
    };
    let batch = prepare_document_batch(&source, &report_documents(&report).unwrap(), None).unwrap();
    for record in &batch.records {
        let origin = origin_from_record(record).unwrap();
        assert_eq!(origin.raw_sha256, record.content_hash);
        assert_ne!(origin.raw_sha256, report.artifact.sha256);
        assert!(origin.origin_artifacts.contains(&report.artifact.sha256));
        assert_eq!(record.metadata["replay_raw_sha256"], report.artifact.sha256);
        assert!(!origin.policy.publication_allowed);
        assert!(!origin.policy.provider_egress_allowed);
        assert_eq!(origin.validity, report.validity);
    }
    let stored: Vec<ReplayObservation> = serde_json::from_str(
        &batch
            .records
            .iter()
            .find(|r| r.logical_id.starts_with("observations/"))
            .unwrap()
            .content,
    )
    .unwrap();
    assert_eq!(stored, report.observations);
}

#[test]
fn adapter_rejects_inconsistent_reports_and_unpinned_rights() {
    let bytes = minimal();
    let request = pinned(&bytes, "tamper-test");
    let report = decode(&bytes, &request).unwrap();
    for field in ["generation", "sha", "source", "locator", "id", "coaching"] {
        let mut changed = report.clone();
        match field {
            "generation" => changed.generation_id = "0".repeat(64),
            "sha" => changed.artifact.sha256 = "0".repeat(64),
            "source" => changed.artifact.source.rights.scope = "another".into(),
            "locator" => changed.observations[0].raw.file_byte_offset = 0,
            "id" => changed.observations[0].observation_id = "0".repeat(64),
            "coaching" => changed.coaching_eligible = true,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_import(&changed, &request),
            Err(ReplayFailure::InvalidWorkerOutput),
            "{field}"
        );
    }
    let mut denied = request.clone();
    denied.source.rights.local_processing_allowed = false;
    assert_eq!(
        validate_import(&report, &denied),
        Err(ReplayFailure::RightsDenied)
    );
    denied = request;
    denied.source.expected_sha256 = None;
    assert_eq!(
        validate_import(&report, &denied),
        Err(ReplayFailure::InvalidWorkerOutput)
    );
}

#[test]
fn chunk_boundary_never_creates_an_empty_observation_document() {
    let bytes = minimal();
    let request = pinned(&bytes, "chunk-boundary");
    let mut report = decode(&bytes, &request).unwrap();
    let initial = serde_json::to_string(&report.observations[0])
        .unwrap()
        .len();
    let ObservationKind::FileHeader { game_directory, .. } = &mut report.observations[0].event
    else {
        panic!("header fixture");
    };
    *game_directory =
        Observed::known("x".repeat(
            brain_ingestion::document_set::MAX_DOCUMENT_BYTES - 2 - initial + "citadel".len(),
        ));
    let documents = report_documents(&report).unwrap();
    let chunks: Vec<_> = documents
        .iter()
        .filter(|d| d.logical_id.starts_with("observations/"))
        .collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(
        chunks[0].content.len(),
        brain_ingestion::document_set::MAX_DOCUMENT_BYTES
    );
    let observations: Vec<ReplayObservation> = serde_json::from_str(&chunks[0].content).unwrap();
    assert_eq!(observations.len(), 1);
}

#[test]
#[ignore = "requires an explicitly selected disposable peer-auth Postgres cluster"]
fn durable_import_reparse_and_internal_query_use_the_normal_store() {
    let socket = std::env::var("REPLAY_TEST_SOCKET").expect("disposable cluster socket");
    let port: u16 = std::env::var("REPLAY_TEST_PORT")
        .expect("test port")
        .parse()
        .unwrap();
    let database = std::env::var("REPLAY_TEST_DATABASE").expect("test database");
    assert!(socket.starts_with("/tmp/") && database.ends_with("_test"));
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let options = PgConnectOptions::new()
            .host(&socket)
            .port(port)
            .database(&database)
            .username(&std::env::var("USER").unwrap());
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options)
            .await
            .unwrap();
        PgStore::new(pool.clone()).migrate_core().await.unwrap();
        let unique = tempfile::tempdir().unwrap();
        let match_id = unique.path().to_string_lossy();
        let bytes = container(
            &(0..12_000)
                .map(|tick| server(tick, Some(0.03125)))
                .collect::<Vec<_>>(),
            false,
        );
        let request = pinned(&bytes, &match_id);
        let report = decode(&bytes, &request).unwrap();
        assert!(report_documents(&report).unwrap().len() > 2);
        let first = import_report(&pool, &report, &request).await.unwrap();
        assert_eq!(first.outcome, ObservationCommit::Inserted);
        pool.close().await;
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new()
                    .host(&socket)
                    .port(port)
                    .database(&database)
                    .username(&std::env::var("USER").unwrap()),
            )
            .await
            .unwrap();
        let second = import_report(&pool, &report, &request).await.unwrap();
        assert_eq!(second.outcome, ObservationCommit::DuplicateGeneration);
        assert_eq!(second.changed_records, 0);
        assert_eq!(first.release_id, second.release_id);
        let mut reread_request = request.clone();
        reread_request.source.retrieved_at_unix_ms = Some(2_000);
        let mut reread = report.clone();
        reread.artifact.source = reread_request.source.clone();
        let repeated = import_report(&pool, &reread, &reread_request)
            .await
            .unwrap();
        assert_eq!(repeated.changed_records, 0);
        assert_eq!(repeated.release_id, first.release_id);
        let principal = Principal {
            actor_id: "fixture".into(),
            channel: "local".into(),
            scopes: BTreeSet::from(["test-only".into()]),
            provider_egress: BTreeSet::new(),
        };
        let example = query_example(&pool, &first.release_id, &principal)
            .await
            .unwrap();
        assert_eq!(example.provenance.replay_id, report.artifact.sha256);
        assert_eq!(example.provenance.raw, report.observations[0].raw);
        assert!(!example.provenance.origin.policy.publication_allowed);
        assert!(!example.provenance.origin.policy.provider_egress_allowed);
        assert!(
            PgStore::new(pool.clone())
                .snapshot(&first.release_id)
                .await
                .unwrap()
                .authorized(&principal, true)
                .unwrap()
                .is_empty()
        );
        let mut denied = principal.clone();
        denied.scopes.clear();
        assert_eq!(
            query_example(&pool, &first.release_id, &denied)
                .await
                .unwrap_err(),
            ReplayFailure::RightsDenied
        );
        let other_bytes = container(&[server(4, Some(0.03))], false);
        let other_request = pinned(&other_bytes, &format!("{match_id}-other"));
        let other_report = decode(&other_bytes, &other_request).unwrap();
        let other = import_report(&pool, &other_report, &other_request)
            .await
            .unwrap();
        let mut alternate_request = pinned(&other_bytes, &match_id);
        let alternate = decode(&other_bytes, &alternate_request).unwrap();
        let duplicate = import_report(&pool, &alternate, &alternate_request)
            .await
            .unwrap();
        assert_eq!(duplicate.outcome, ObservationCommit::DuplicateMatch);
        assert_eq!(duplicate.active_observations, first.active_observations);
        let snapshot = PgStore::new(pool.clone())
            .snapshot(&duplicate.release_id)
            .await
            .unwrap();
        assert_eq!(
            snapshot
                .revisions
                .iter()
                .filter(|r| r.logical_id.starts_with("raw/"))
                .count(),
            2
        );
        alternate_request.selection.sample_every_ticks += 1;
        let reparsed = decode(&other_bytes, &alternate_request).unwrap();
        let replaced = import_report(&pool, &reparsed, &alternate_request)
            .await
            .unwrap();
        assert_eq!(replaced.outcome, ObservationCommit::Reparsed);
        assert_eq!(replaced.active_observations, reparsed.observations.len());
        let example = query_example(&pool, &replaced.release_id, &principal)
            .await
            .unwrap();
        assert_eq!(example.generation_id, reparsed.generation_id);
        assert_eq!(example.provenance.replay_id, reparsed.artifact.sha256);
        let preserved = query_example(&pool, &other.release_id, &principal)
            .await
            .unwrap();
        assert_eq!(preserved.generation_id, other_report.generation_id);
        let old = query_example(&pool, &first.release_id, &principal)
            .await
            .unwrap();
        assert_eq!(old.generation_id, report.generation_id);
        let snapshot = PgStore::new(pool.clone())
            .snapshot(&replaced.release_id)
            .await
            .unwrap();
        assert_eq!(
            snapshot
                .revisions
                .iter()
                .filter(|r| r.logical_id.starts_with("raw/"))
                .count(),
            2
        );
        let raw_path = unique.path().join("input.dem");
        let request_path = unique.path().join("request.json");
        let cli_request = pinned(&other_bytes, &format!("{match_id}-cli"));
        std::fs::write(&raw_path, &other_bytes).unwrap();
        std::fs::write(&request_path, serde_json::to_vec(&cli_request).unwrap()).unwrap();
        let import_cli = || {
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_dbrain-replay-import"))
                .arg("import")
                .arg(&raw_path)
                .arg(&request_path)
                .arg(env!("CARGO_BIN_EXE_dbrain-replay-worker"))
                .arg("--peer-test")
                .arg(&socket)
                .arg(port.to_string())
                .arg(&database)
                .output()
                .unwrap();
            assert!(output.status.success(), "{:?}", output);
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
        };
        let cli_first = import_cli();
        assert_eq!(cli_first["outcome"], "inserted");
        let cli_second = import_cli();
        assert_eq!(cli_second["outcome"], "duplicate_generation");
        assert_eq!(cli_second["changed_records"], 0);
        assert_eq!(cli_first["release_id"], cli_second["release_id"]);
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_dbrain-replay-import"))
            .arg("query")
            .arg(cli_first["release_id"].as_str().unwrap())
            .arg(&cli_request.source.rights.scope)
            .arg("--peer-test")
            .arg(&socket)
            .arg(port.to_string())
            .arg(&database)
            .output()
            .unwrap();
        assert!(output.status.success(), "{:?}", output);
        let example: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            example["provenance"]["replay_id"],
            cli_request
                .source
                .expected_sha256
                .as_ref()
                .unwrap()
                .as_str()
        );
        assert_eq!(
            example["match_reference"]["match_id"],
            format!("{match_id}-cli")
        );
        assert_eq!(example["schema_revision"], SCHEMA_REVISION);
        assert_eq!(example["parser_revision"], parser_revision());
        assert_eq!(
            example["provenance"]["origin"]["policy"]["publication_allowed"],
            false
        );
        assert_eq!(
            example["provenance"]["origin"]["policy"]["provider_egress_allowed"],
            false
        );
        pool.close().await;
    });
}
