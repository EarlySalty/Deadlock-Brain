use crate::{
    author::{self, Action, AuthorProposal, Citation, IndependentReview, VerifiedDocument},
    config::{MaintenanceConfig, RepositoryConfig},
    digest, scanner,
};
use brain_contracts::{source::SourcePolicy, value::Observed, SourceVisibility};
use std::{collections::BTreeSet, fs, path::Path, process::Command};

fn git(path: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "user.name=Prüfung",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(args)
        .current_dir(path)
        .output()
        .unwrap();
    assert!(output.status.success(), "Git-Prüfaufbau fehlgeschlagen");
    String::from_utf8(output.stdout).unwrap().trim().into()
}
fn pin(path: &Path) {
    git(path, &["add", "."]);
    git(path, &["commit", "-m", "Prüfbeleg"]);
    git(path, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
}

fn fixture() -> (tempfile::TempDir, MaintenanceConfig, RepositoryConfig) {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source");
    let docs = dir.path().join("docs");
    fs::create_dir_all(source.join("src")).unwrap();
    fs::create_dir_all(docs.join("internal")).unwrap();
    for (path, origin) in [
        (&source, "https://github.com/test/source.git"),
        (&docs, "https://github.com/test/docs.git"),
    ] {
        git(path, &["init", "-b", "main"]);
        git(path, &["remote", "add", "origin", origin]);
    }
    fs::write(
        source.join("src/service.rs"),
        "pub fn bereit() -> bool { true }\n",
    )
    .unwrap();
    fs::write(docs.join("internal/system.html"),"<!doctype html><html lang='de'><head><title>Prüfbeleg</title></head><body><main><h1>Prüfbeleg</h1><section id='zustand'><p>Der Helfer liefert true.</p></section></main></body></html>").unwrap();
    pin(&source);
    pin(&docs);
    let mut config: MaintenanceConfig =
        serde_json::from_str(include_str!("../../config/smoke.example.json")).unwrap();
    config.docs_repo = docs;
    config.docs_origin = "https://github.com/test/docs.git".into();
    config.repo_roots = vec![dir.path().to_owned()];
    let mut policy:SourcePolicy=serde_json::from_value(serde_json::json!({"visibility":"public","allowed_scopes":[],"authorization_ref":{"status":"known","value":"controlled-test"},"license":{"status":"unknown","reason":"not_present"},"publication_allowed":true,"provider_egress_allowed":true,"raw_retention_allowed":false})).unwrap();
    let mut document_policy = policy.clone();
    document_policy.visibility = SourceVisibility::Internal;
    document_policy.allowed_scopes = BTreeSet::from(["internal_docs".into()]);
    document_policy.publication_allowed = false;
    policy.authorization_ref = Observed::known("controlled-test".into());
    let repo = RepositoryConfig {
        code_only_migration_targets: BTreeSet::new(),
        target_document_policies: std::collections::BTreeMap::new(),
        id: "source".into(),
        path: source,
        origin: "https://github.com/test/source.git".into(),
        source_ref: "refs/remotes/origin/main".into(),
        source_paths: vec!["src".into()],
        evidence_paths: vec!["src/service.rs".into()],
        doc_targets: vec!["internal/system.html".into()],
        output_paths: std::collections::BTreeMap::new(),
        policy,
        document_policy,
        code_only_export_approved: true,
        deployed_sha_file: None,
    };
    config.repositories = vec![repo.clone()];
    config.validate().unwrap();
    (dir, config, repo)
}

use super::*;
use crate::integration::runtime_config::LocalImport;
use brain_contracts::CorpusRelease;
use std::os::unix::fs::PermissionsExt;

#[tokio::test]
async fn public_import_uses_only_approved_registered_git_blobs() {
    let (_dir, mut config, mut repo) = fixture();
    let target = "public/discord-server/module/voice-lanes.html";
    fs::create_dir_all(config.docs_repo.join("public/discord-server/module")).unwrap();
    let html = "<!doctype html><html lang='de'><head><title>Lanes</title></head><body><main><h1>Lanes</h1><p>Casual, Ranked und Street Brawl.</p></main></body></html>";
    fs::write(config.docs_repo.join(target), html).unwrap();
    pin(&config.docs_repo);
    repo.path = config.docs_repo.clone();
    repo.origin = config.docs_origin.clone();
    repo.doc_targets.push(target.into());
    let mut policy = repo.policy.clone();
    policy.allowed_scopes = BTreeSet::from(["bot.public".into()]);
    repo.target_document_policies.insert(target.into(), policy);
    config.repositories = vec![repo];
    fs::write(config.docs_repo.join(target), "Ungeprüfter Arbeitsstand").unwrap();
    let documents = approved_public_documents(&config).await.unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].logical_id, target);
    assert_eq!(documents[0].content, html);
    assert_eq!(documents[0].origin.raw_sha256, digest(html.as_bytes()));
    config.repositories[0]
        .target_document_policies
        .get_mut(target)
        .unwrap()
        .authorization_ref = Observed::unknown(brain_contracts::value::UnknownReason::NotPresent);
    assert!(approved_public_documents(&config).await.is_err());
}

#[test]
fn publication_release_identity_accepts_short_and_utf8_job_ids() {
    let (_dir, config, repo) = fixture();
    let mut ids = BTreeSet::new();
    for id in ["x", "abcdé", "docs-gewöhnlicher-Auftrag"] {
        let mut spec = job_spec(
            &config,
            &repo,
            "internal/system.html",
            &"a".repeat(40),
            None,
            "fixture",
        );
        spec.id = id.into();
        spec.idempotency_key = id.into();
        spec.validate().unwrap();
        let (release, version) = publication_release_identity(&spec.id);
        assert_eq!(release, format!("maintenance-{}", digest(id.as_bytes())));
        assert_eq!(version, format!("docs-{}", digest(id.as_bytes())));
        assert!(ids.insert(release));
    }
}

#[test]
fn local_operator_reloads_internal_scopes_without_provider_egress() {
    let (dir, mut config, _repo) = fixture();
    let path = dir.path().join("maintenance.json");
    config.internal_doc_scopes = BTreeSet::from(["ops_docs".into()]);
    fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    let record = brain_contracts::SourceRecordV2 {
        source_id: "own-test".into(),
        logical_id: "internal/test".into(),
        revision: 1,
        content_hash: digest(b"Beleg"),
        content: "Beleg".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from(["ops_docs".into()]),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    let first = local_operator_principal(1000, &path).unwrap();
    assert!(brain_contracts::store::record_allowed(
        &record, &first, false
    ));
    assert!(!brain_contracts::store::record_allowed(
        &record, &first, true
    ));
    config.internal_doc_scopes = BTreeSet::from(["internal_docs".into()]);
    fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    let second = local_operator_principal(1000, &path).unwrap();
    assert!(!brain_contracts::store::record_allowed(
        &record, &second, false
    ));
    assert!(second.provider_egress.is_empty());
}

async fn setup() -> (
    tempfile::TempDir,
    Runner,
    MaintenanceConfig,
    RepositoryConfig,
    scanner::ScanResult,
) {
    let (dir, mut config, repo) = fixture();
    let private = dir.path().join("private");
    fs::create_dir_all(private.join("internal")).unwrap();
    fs::set_permissions(&private, fs::Permissions::from_mode(0o700)).unwrap();
    fs::set_permissions(private.join("internal"), fs::Permissions::from_mode(0o700)).unwrap();
    let file = private.join("internal/system.html");
    fs::copy(config.docs_repo.join("internal/system.html"), &file).unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    config.private_document_root = Some(private);
    let config_path = dir.path().join("maintenance.json");
    fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host("/tmp/brain-maintenance-test-20261002/socket")
                .port(55447)
                .username("brain_maintenance_test")
                .database("postgres")
                .password(""),
        )
        .await
        .unwrap();
    let store = PgStore::new(pool.clone());
    store.check_maintenance_schema().await.unwrap();
    let release_id = format!("runner-fixture-{}", chrono::Utc::now().timestamp_micros());
    store
        .publish_release(&CorpusRelease {
            release_id: release_id.clone(),
            knowledge_version: "runner-test".into(),
            patch: "fixture".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::new(),
        })
        .await
        .unwrap();
    let mut serve: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../config/brain-serve.example.json"
    ))
    .unwrap();
    serve["release"] = json!({"id":release_id,"knowledge_version":"runner-test"});
    let serve_path = dir.path().join("serve.json");
    fs::write(&serve_path, serde_json::to_vec(&serve).unwrap()).unwrap();
    fs::set_permissions(&serve_path, fs::Permissions::from_mode(0o600)).unwrap();
    let mut runtime: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../ops/brain-maintenance/runtime.example.json"
    ))
    .unwrap();
    runtime["maintenance_config"] = json!(config_path);
    runtime["serve_config"] = json!(serve_path);
    runtime["artifact_dir"] = json!(dir.path().join("artifacts"));
    runtime["status_file"] = json!(dir.path().join("status.json"));
    let runtime: RuntimeConfig = serde_json::from_value(runtime).unwrap();
    let artifacts = Artifacts::open(&runtime.artifact_dir).unwrap();
    let runner = Runner {
        runtime,
        store,
        pool,
        artifacts,
        jev: Zeroizing::new("fixture-unused".into()),
        owner: release_id,
    };
    let config = load_maintenance(&runner.runtime.maintenance_config).unwrap();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    runner
        .store
        .register_maintenance_source(&MaintenanceSourceRegistration {
            repo_id: registration_id(&repo, "internal/system.html"),
            source_id: source_id(&repo, "internal/system.html"),
            discovered_sha: scan.source_sha.clone(),
            policy: Some(repo.document_policy.clone()),
            authorization_ref: Some("controlled-test".into()),
        })
        .await
        .unwrap();
    (dir, runner, config, repo, scan)
}

async fn stage(
    runner: &Runner,
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    scan: &scanner::ScanResult,
    reviewer: bool,
) -> MaintenanceJob {
    let spec = job_spec(
        config,
        repo,
        "internal/system.html",
        &scan.source_sha,
        None,
        &runner.owner,
    );
    runner
        .store
        .enqueue_maintenance(&spec, MaintenanceStatus::Planned)
        .await
        .unwrap();
    let mut checkpoint = MaintenanceCheckpoint::default();
    let pin = brain_serve::Config::load(&runner.runtime.serve_config)
        .unwrap()
        .release
        .id;
    checkpoint.artifact_refs.insert(
        "scan".into(),
        runner
            .artifacts
            .put(
                &serde_json::to_vec(&ScanCheckpoint::capture(scan, pin)).unwrap(),
                "json",
            )
            .unwrap(),
    );
    checkpoint.artifact_refs.insert(
        "triage".into(),
        runner
            .artifacts
            .put(
                br#"{"status":"stale","source_review_required":true,"error_code":null}"#,
                "json",
            )
            .unwrap(),
    );
    for next in [MaintenanceStatus::SourceReview, MaintenanceStatus::Author] {
        let job = runner
            .store
            .claim_maintenance(&runner.owner, 900000)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            job.spec.id, spec.id,
            "Nur die isolierte Prüfwarteschlange darf benutzt werden"
        );
        runner
            .store
            .transition_maintenance(job.lease.as_ref().unwrap(), next, &checkpoint)
            .await
            .unwrap();
    }
    let mut job = runner
        .store
        .claim_maintenance(&runner.owner, 900000)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(job.spec.id, spec.id);
    if reviewer {
        let draft = draft(config, scan);
        checkpoint.artifact_refs.insert(
            "draft".into(),
            runner
                .artifacts
                .put(&serde_json::to_vec(&draft).unwrap(), "json")
                .unwrap(),
        );
        runner
            .store
            .transition_maintenance(
                job.lease.as_ref().unwrap(),
                MaintenanceStatus::Reviewer,
                &checkpoint,
            )
            .await
            .unwrap();
        job = runner
            .store
            .claim_maintenance(&runner.owner, 900000)
            .await
            .unwrap()
            .unwrap();
    }
    job
}

#[tokio::test]
#[ignore = "Benötigt den ausdrücklich isolierten PostgreSQL-Prüfcluster; exklusiv ausführen"]
async fn postgres_selected_tick_claims_only_exact_job_and_unknown_id_claims_none() {
    let (_dir, runner, mut config, repo, scan) = setup().await;
    let mut selected = job_spec(
        &config,
        &repo,
        "internal/system.html",
        &scan.source_sha,
        None,
        &runner.owner,
    );
    selected.id = format!("{}-selected-job", runner.owner);
    selected.idempotency_key = selected.id.clone();
    let mut other = selected.clone();
    other.id = format!("{}-other-job", runner.owner);
    other.idempotency_key = other.id.clone();
    runner
        .store
        .enqueue_maintenance(&other, MaintenanceStatus::Planned)
        .await
        .unwrap();
    runner
        .store
        .enqueue_maintenance(&selected, MaintenanceStatus::Planned)
        .await
        .unwrap();
    config.repositories.clear();
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    runner.tick_for_job(Some("unknown-job")).await.unwrap();
    for spec in [&selected, &other] {
        let job = runner
            .store
            .maintenance_job(&spec.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(job.attempts, 0);
        assert!(job.lease.is_none());
    }
    assert_eq!(
        fs::read_dir(&runner.runtime.artifact_dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".intent"))
            .count(),
        0
    );
    let claimed = runner
        .store
        .claim_maintenance_for_job("selected-worker", 900000, Some(&selected.id))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(claimed.spec.id, selected.id);
    assert!(runner
        .store
        .claim_maintenance_for_job("other-worker", 900000, Some(&selected.id))
        .await
        .unwrap()
        .is_none());
    let mut fresh = other.clone();
    fresh.id = format!("{}-newly-enqueued-job", runner.owner);
    fresh.idempotency_key = fresh.id.clone();
    runner
        .store
        .enqueue_maintenance(&fresh, MaintenanceStatus::Planned)
        .await
        .unwrap();
    assert!(runner
        .store
        .claim_maintenance_for_job("other-worker", 900000, Some("unknown-job"))
        .await
        .unwrap()
        .is_none());
    for spec in [&other, &fresh] {
        let job = runner
            .store
            .maintenance_job(&spec.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(job.attempts, 0);
        assert!(job.lease.is_none());
    }
    runner
        .store
        .transition_maintenance(
            claimed.lease.as_ref().unwrap(),
            MaintenanceStatus::Failed,
            &claimed.checkpoint,
        )
        .await
        .unwrap();
    assert!(runner
        .store
        .claim_maintenance_for_job("other-worker", 900000, Some(&selected.id))
        .await
        .unwrap()
        .is_none());
    let normal = runner
        .store
        .claim_maintenance("normal-worker", 900000)
        .await
        .unwrap()
        .unwrap();
    assert_ne!(normal.spec.id, selected.id);
    runner
        .store
        .transition_maintenance(
            normal.lease.as_ref().unwrap(),
            MaintenanceStatus::Failed,
            &normal.checkpoint,
        )
        .await
        .unwrap();
}
fn draft(config: &MaintenanceConfig, scan: &scanner::ScanResult) -> author::DraftDocument {
    let content = scan.documents["internal/system.html"].clone().unwrap();
    author::DraftDocument {
        export_binding: Some(
            author::export_binding(
                config,
                &config.repositories[0],
                scan,
                "internal/system.html",
            )
            .unwrap(),
        ),
        proposal: AuthorProposal {
            source_sha: scan.source_sha.clone(),
            target: "internal/system.html".into(),
            before_sha256: Some(digest(content.as_bytes())),
            action: Action::Keep,
            content,
            citations: vec![Citation {
                path: scan.source_blobs[0].path.clone(),
                sha256: scan.source_blobs[0].sha256.clone(),
            }],
            open_questions: vec![],
        },
        proof: author::CodexRunProof {
            run_id: "fixture-author".into(),
            model: config.codex.model.clone(),
            reasoning_effort: "high".into(),
            exit_code: 0,
            event_types: vec![],
            tool_events: 0,
            input_tokens: Some(4),
            output_tokens: Some(5),
        },
    }
}
fn rejection(
    config: &MaintenanceConfig,
    scan: &scanner::ScanResult,
    draft: &author::DraftDocument,
) -> VerifiedDocument {
    VerifiedDocument {
        review_contract_version: Some(author::REVIEW_CONTRACT_VERSION.into()),
        review_input_sha256: Some(
            author::review_input_binding(config, &config.repositories[0], scan, &draft.proposal)
                .unwrap(),
        ),
        export_binding: draft.export_binding.clone(),
        proposal: draft.proposal.clone(),
        review: IndependentReview {
            approved: false,
            source_sha: scan.source_sha.clone(),
            proposal_sha256: digest(&serde_json::to_vec(&draft.proposal).unwrap()),
            citations: draft.proposal.citations.clone(),
            findings: vec!["Die Aussage braucht einen zusätzlichen Beleg.".into()],
        },
        evidence_sha256: digest(&serde_json::to_vec(scan).unwrap()),
        model: config.codex.model.clone(),
        prompt_version: config.prompt_version.clone(),
        author_run_id: draft.proof.run_id.clone(),
        reviewer_run_id: "fixture-reviewer".into(),
        reviewer_input_tokens: Some(6),
        reviewer_output_tokens: Some(7),
    }
}

#[tokio::test]
#[ignore = "Benötigt den ausdrücklich isolierten PostgreSQL-Prüfcluster; exklusiv ausführen"]
async fn postgres_local_query_skips_denied_documents_and_preserves_other_results() {
    use brain_contracts::DocumentStorePort;
    use brain_ingestion::document_set::prepare_document_batch;
    let (_dir, runner, _config, repo, scan) = setup().await;
    let mut batches = Vec::new();
    let mut pins = BTreeMap::new();
    for case in ["allowed", "missing", "revoked", "changed"] {
        let id = format!("maintenance-docs:query-{}-{case}", runner.owner);
        let target = format!("internal/{case}.html");
        let content = format!("<html><body><p>Gemeinsamer Prüfbeleg {case}</p></body></html>");
        let mut origin = scan.source_blobs[0].origin.clone();
        origin.raw_sha256 = digest(content.as_bytes());
        origin.policy = repo.document_policy.clone();
        let source = DocumentSetSource {
            source_id: id.clone(),
            configuration: "query-fixture".into(),
            visibility: origin.policy.visibility,
            allowed_scopes: origin.policy.allowed_scopes.clone(),
            tombstone_metadata: Default::default(),
        };
        let document = CoreDocument {
            logical_id: target.clone(),
            content,
            origin,
            metadata: BTreeMap::from([("content_format".into(), "html".into())]),
        };
        let batch = prepare_document_batch(&source, &[document], None).unwrap();
        let lease = runner
            .store
            .claim(&id, "query-fixture", 30000)
            .await
            .unwrap();
        if case != "missing" {
            let mut policy = repo.document_policy.clone();
            if case == "changed" {
                policy.provider_egress_allowed = false;
            }
            runner
                .store
                .register_maintenance_source(&MaintenanceSourceRegistration {
                    repo_id: id.clone(),
                    source_id: id.clone(),
                    discovered_sha: scan.source_sha.clone(),
                    policy: if case == "revoked" {
                        None
                    } else {
                        Some(policy)
                    },
                    authorization_ref: Some("query-fixture".into()),
                })
                .await
                .unwrap();
        }
        pins.insert(id, BTreeMap::from([(target, 1)]));
        batches.push((batch, lease));
    }
    let release = CorpusRelease {
        release_id: format!("query-release-{}", runner.owner),
        knowledge_version: "query-fixture".into(),
        patch: "fixture".into(),
        created_at_epoch: 1,
        source_revisions: pins,
    };
    for (batch, lease) in &batches {
        runner.store.commit(batch, lease).await.unwrap();
    }
    runner.store.publish_release(&release).await.unwrap();
    let mut serve: serde_json::Value =
        serde_json::from_slice(&fs::read(&runner.runtime.serve_config).unwrap()).unwrap();
    serve["release"] =
        json!({"id":release.release_id,"knowledge_version":release.knowledge_version});
    fs::write(
        &runner.runtime.serve_config,
        serde_json::to_vec(&serve).unwrap(),
    )
    .unwrap();
    let result = runner.query("Prüfbeleg").await.unwrap();
    let hits = result["results"].as_array().unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0]["logical_id"], "internal/allowed.html");
    assert_eq!(result["release_id"], release.release_id);
}

#[tokio::test]
#[ignore = "Benötigt den ausdrücklich isolierten PostgreSQL-Prüfcluster; exklusiv ausführen"]
async fn postgres_runner_private_basis_reset_and_rejected_review_resume() {
    let (dir, runner, mut config, repo, scan) = setup().await;
    let mut resumed = stage(&runner, &config, &repo, &scan, false).await;
    let saved = draft(&config, &scan);
    let reference = runner
        .artifacts
        .put(&serde_json::to_vec(&saved).unwrap(), "json")
        .unwrap();
    runner
        .store
        .set_maintenance_artifact(resumed.lease.as_ref().unwrap(), "draft", &reference)
        .await
        .unwrap();
    let calls = dir.path().join("initial-author-calls");
    let executable = dir.path().join("unexpected-author");
    fs::write(
        &executable,
        format!("#!/bin/sh\nprintf 'x' >> '{}'\nexit 9\n", calls.display()),
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    config.codex.executable = executable;
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    resumed = runner
        .store
        .maintenance_job(&resumed.spec.id)
        .await
        .unwrap()
        .unwrap();
    runner.advance(&mut resumed).await.unwrap();
    let after = runner
        .store
        .maintenance_job(&resumed.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(after.status, MaintenanceStatus::Reviewer);
    assert!(!calls.exists());
    assert_eq!(after.checkpoint.artifact_refs["draft"], reference);
    let lease = runner
        .store
        .claim_maintenance(&runner.owner, 900000)
        .await
        .unwrap()
        .unwrap()
        .lease
        .unwrap();
    runner
        .store
        .transition_maintenance(&lease, MaintenanceStatus::Failed, &after.checkpoint)
        .await
        .unwrap();
    for variant in ["changed", "deleted", "origin", "permissions"] {
        let (_dir, runner, config, repo, mut scan) = setup().await;
        if variant == "origin" {
            scan.source_blobs[0].origin.identity.logical_id = "forged.rs".into();
        }
        let mut job = stage(&runner, &config, &repo, &scan, false).await;
        let paid = runner
            .artifacts
            .put(b"{\"paid_review\":true}", "json")
            .unwrap();
        runner
            .store
            .set_maintenance_artifact(job.lease.as_ref().unwrap(), "verified", &paid)
            .await
            .unwrap();
        runner
            .store
            .set_maintenance_artifact(job.lease.as_ref().unwrap(), "review_rejections", "1")
            .await
            .unwrap();
        let file = config
            .private_document_root
            .as_ref()
            .unwrap()
            .join("internal/system.html");
        if variant == "deleted" {
            fs::remove_file(file).unwrap();
        } else if variant == "permissions" {
            fs::set_permissions(file, fs::Permissions::from_mode(0o644)).unwrap();
        } else {
            fs::write(file, "Geänderte private Fassung").unwrap();
        }
        let outcome = runner.advance(&mut job).await;
        let current = runner
            .store
            .maintenance_job(&job.spec.id)
            .await
            .unwrap()
            .unwrap();
        if variant == "changed" || variant == "deleted" {
            outcome.unwrap();
            assert_eq!(current.status, MaintenanceStatus::SourceReview);
            assert!(current.lease.is_none());
            assert_eq!(current.checkpoint.artifact_refs["review_rejections"], "1");
            let archived: MaintenanceCheckpoint = serde_json::from_slice(
                &runner
                    .artifacts
                    .read(&current.checkpoint.artifact_refs["obsolete_checkpoint"])
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(archived.artifact_refs["verified"], paid);
            let lease = runner
                .store
                .claim_maintenance(&runner.owner, 900000)
                .await
                .unwrap()
                .unwrap()
                .lease
                .unwrap();
            runner
                .store
                .transition_maintenance(&lease, MaintenanceStatus::Failed, &current.checkpoint)
                .await
                .unwrap();
        } else {
            assert!(outcome.is_err());
            assert_eq!(current.status, MaintenanceStatus::Author);
            runner
                .store
                .transition_maintenance(
                    job.lease.as_ref().unwrap(),
                    MaintenanceStatus::Failed,
                    &current.checkpoint,
                )
                .await
                .unwrap();
        }
    }
    let (_dir, runner, config, repo, scan) = setup().await;
    let mut job = stage(&runner, &config, &repo, &scan, true).await;
    let rejected = rejection(&config, &scan, &draft(&config, &scan));
    author::validate_verified_draft(&config, &repo, &scan, &draft(&config, &scan), &rejected)
        .unwrap();
    for variant in ["draft", "run", "source", "evidence", "proposal"] {
        let mut mismatched = rejected.clone();
        match variant {
            "draft" => mismatched.proposal.content.push_str(" anderer Text"),
            "run" => mismatched.author_run_id = "anderer-autorlauf".into(),
            "source" => mismatched.review.source_sha = "f".repeat(40),
            "evidence" => mismatched.evidence_sha256 = "f".repeat(64),
            _ => mismatched.review.proposal_sha256 = "f".repeat(64),
        }
        assert!(author::validate_verified_draft(
            &config,
            &repo,
            &scan,
            &draft(&config, &scan),
            &mismatched
        )
        .is_err());
    }
    let mut questions = rejected.proposal.clone();
    questions.action = Action::SourceReview;
    questions.content.clear();
    questions.open_questions = vec!["Welcher Aufrufer nutzt die Funktion?".into()];
    author::validate_proposal(&config, &repo, &scan, &questions).unwrap();
    let reference = runner
        .artifacts
        .put(&serde_json::to_vec(&rejected).unwrap(), "json")
        .unwrap();
    let review_answer = _dir.path().join("review.json");
    fs::write(
        &review_answer,
        serde_json::to_vec(&rejected.review).unwrap(),
    )
    .unwrap();
    let review_calls = _dir.path().join("review-calls");
    let executable = _dir.path().join("reviewer-fixture");
    fs::write(&executable,format!("#!/bin/sh\nresult=''\nwhile [ $# -gt 0 ]; do\nif [ \"$1\" = '--output-last-message' ]; then shift; result=$1; fi\nshift\ndone\ncat >/dev/null\nprintf 'x' >> '{}'\ncp '{}' \"$result\"\nprintf '%s\\n' '{{\"type\":\"thread.started\",\"thread_id\":\"fixture-reviewer\"}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":6,\"output_tokens\":7}}}}'\n",review_calls.display(),review_answer.display())).unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    let mut provider_config = config.clone();
    provider_config.codex.executable = executable;
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&provider_config).unwrap(),
    )
    .unwrap();
    runner.advance(&mut job).await.unwrap();
    let after = runner
        .store
        .maintenance_job(&job.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(after.status, MaintenanceStatus::Author);
    assert_eq!(after.checkpoint.artifact_refs["review_rejections"], "1");
    assert_eq!(after.checkpoint.artifact_refs["rejected_review"], reference);
    assert!(!after.checkpoint.review.as_ref().unwrap().accepted);
    let lease = runner
        .store
        .claim_maintenance(&runner.owner, 900000)
        .await
        .unwrap()
        .unwrap()
        .lease
        .unwrap();
    let answer = _dir.path().join("proposal.json");
    fs::write(&answer, serde_json::to_vec(&rejected.proposal).unwrap()).unwrap();
    let calls = _dir.path().join("calls");
    let feedback_seen = _dir.path().join("feedback-seen");
    let executable = _dir.path().join("codex-fixture");
    fs::write(&executable, format!("#!/bin/sh\nresult=''\nwhile [ $# -gt 0 ]; do\nif [ \"$1\" = '--output-last-message' ]; then shift; result=$1; fi\nshift\ndone\nprompt=$(cat)\ncase \"$prompt\" in *'Die Aussage braucht einen zusätzlichen Beleg.'*) printf 'yes' > '{}' ;; *) exit 9 ;; esac\nprintf 'x' >> '{}'\ncp '{}' \"$result\"\nprintf '%s\\n' '{{\"type\":\"thread.started\",\"thread_id\":\"fixture-corrective-author\"}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":8,\"output_tokens\":9}}}}'\n",feedback_seen.display(),calls.display(),answer.display())).unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    let mut config = config.clone();
    config.codex.executable = executable;
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    let mut corrective = runner
        .store
        .maintenance_job(&lease.job_id)
        .await
        .unwrap()
        .unwrap();
    runner.advance(&mut corrective).await.unwrap();
    let final_job = runner
        .store
        .maintenance_job(&lease.job_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(final_job.status, MaintenanceStatus::Failed);
    assert_eq!(
        final_job.checkpoint.artifact_refs["blocked_reason"],
        "KNOWN_REJECTED_PROPOSAL"
    );
    assert_eq!(fs::read(calls).unwrap(), b"x");
    assert_eq!(fs::read(feedback_seen).unwrap(), b"yes");
    assert_eq!(fs::read(review_calls).unwrap(), b"x");
    assert_eq!(
        final_job.checkpoint.artifact_refs["rejected_review"],
        reference
    );
    let (_dir, runner, config, repo, scan) = setup().await;
    let mut second = stage(&runner, &config, &repo, &scan, true).await;
    let rejected = rejection(&config, &scan, &draft(&config, &scan));
    let reference = runner
        .artifacts
        .put(&serde_json::to_vec(&rejected).unwrap(), "json")
        .unwrap();
    let lease = second.lease.as_ref().unwrap();
    runner
        .store
        .set_maintenance_artifact(lease, "verified", &reference)
        .await
        .unwrap();
    runner
        .store
        .set_maintenance_artifact(lease, "review_rejections", "1")
        .await
        .unwrap();
    second = runner
        .store
        .maintenance_job(&second.spec.id)
        .await
        .unwrap()
        .unwrap();
    runner.advance(&mut second).await.unwrap();
    let stopped = runner
        .store
        .maintenance_job(&second.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stopped.status, MaintenanceStatus::Failed);
    assert_eq!(stopped.checkpoint.artifact_refs["review_rejections"], "2");
    assert_eq!(
        stopped.checkpoint.artifact_refs["blocked_reason"],
        "REVIEW_REJECTIONS_EXHAUSTED"
    );

    let (_dir, runner, config, repo, scan) = setup().await;
    let mut uncertain = stage(&runner, &config, &repo, &scan, false).await;
    let reference = runner
        .artifacts
        .put(
            &serde_json::to_vec(&rejection(&config, &scan, &draft(&config, &scan))).unwrap(),
            "json",
        )
        .unwrap();
    let lease = uncertain.lease.as_ref().unwrap();
    runner
        .store
        .set_maintenance_artifact(lease, "rejected_review", &reference)
        .await
        .unwrap();
    runner
        .store
        .set_maintenance_artifact(lease, "review_rejections", "1")
        .await
        .unwrap();
    runner
        .store
        .set_maintenance_artifact(lease, "correction_intent", &reference)
        .await
        .unwrap();
    uncertain = runner
        .store
        .maintenance_job(&uncertain.spec.id)
        .await
        .unwrap()
        .unwrap();
    runner.advance(&mut uncertain).await.unwrap();
    let stopped = runner
        .store
        .maintenance_job(&uncertain.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stopped.status, MaintenanceStatus::Failed);
    assert_eq!(
        stopped.checkpoint.artifact_refs["blocked_reason"],
        "CORRECTION_RESULT_UNCERTAIN"
    );
}

#[tokio::test]
#[ignore = "Benötigt den ausdrücklich isolierten PostgreSQL-Prüfcluster; exklusiv ausführen"]
async fn postgres_paid_invalid_results_are_terminal_and_keep_private_receipts() {
    let scratch = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host("/tmp/brain-maintenance-test-20261002/socket")
                .port(55447)
                .username("brain_maintenance_test")
                .database("postgres")
                .password(""),
        )
        .await
        .unwrap();
    let identity: (String, String, Option<String>, String) = sqlx::query_as("SELECT current_user::text,current_database()::text,inet_server_addr()::text,current_setting('port')")
        .fetch_one(&scratch).await.unwrap();
    assert_eq!(
        identity,
        (
            "brain_maintenance_test".into(),
            "postgres".into(),
            None,
            "55447".into()
        )
    );
    sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
        .execute(&scratch)
        .await
        .unwrap();
    for stage_name in ["author", "correction", "reviewer"] {
        for raw in ["", "{", "{}", "semantic"] {
            sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
                .execute(&scratch)
                .await
                .unwrap();
            let (dir, runner, mut config, repo, scan) = setup().await;
            let mut job = stage(&runner, &config, &repo, &scan, stage_name == "reviewer").await;
            if stage_name == "correction" {
                let rejected = rejection(&config, &scan, &draft(&config, &scan));
                let reference = runner
                    .artifacts
                    .put(&serde_json::to_vec(&rejected).unwrap(), "json")
                    .unwrap();
                for (key, value) in [
                    ("rejected_review", reference.as_str()),
                    ("review_rejections", "1"),
                ] {
                    runner
                        .store
                        .set_maintenance_artifact(job.lease.as_ref().unwrap(), key, value)
                        .await
                        .unwrap();
                }
                job = runner
                    .store
                    .maintenance_job(&job.spec.id)
                    .await
                    .unwrap()
                    .unwrap();
            }
            let answer = dir.path().join("answer");
            let body = if raw == "semantic" {
                if stage_name == "reviewer" {
                    let mut review = rejection(&config, &scan, &draft(&config, &scan)).review;
                    review.source_sha = "f".repeat(40);
                    serde_json::to_string(&review).unwrap()
                } else {
                    let mut proposal = draft(&config, &scan).proposal;
                    proposal.source_sha = "f".repeat(40);
                    serde_json::to_string(&proposal).unwrap()
                }
            } else {
                raw.into()
            };
            fs::write(&answer, &body).unwrap();
            let calls = dir.path().join("calls");
            let executable = dir.path().join("provider");
            fs::write(&executable,format!("#!/bin/sh\nresult=''\nwhile [ $# -gt 0 ]; do\nif [ \"$1\" = '--output-last-message' ]; then shift; result=$1; fi\nshift\ndone\ncat >/dev/null\nprintf x >> '{}'\ncp '{}' \"$result\"\nprintf '%s\\n' '{{\"type\":\"thread.started\",\"thread_id\":\"cost-review-run\"}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":21,\"output_tokens\":4}}}}'\n", calls.display(), answer.display())).unwrap();
            fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
            config.codex.executable = executable;
            fs::write(
                &runner.runtime.maintenance_config,
                serde_json::to_vec(&config).unwrap(),
            )
            .unwrap();
            let error = runner.advance(&mut job).await.unwrap_err();
            runner
                .handle_failed_advance(&job, job.lease.as_ref().unwrap(), &error)
                .await
                .unwrap();
            let stored = runner
                .store
                .maintenance_job(&job.spec.id)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(stored.status, MaintenanceStatus::Failed);
            assert!(stored.lease.is_none());
            assert_eq!(
                stored.checkpoint.artifact_refs["blocked_reason"],
                "PAID_RESULT_INVALID_OR_UNCERTAIN"
            );
            assert!(stored.checkpoint.publication.is_none());
            assert!(stored.checkpoint.activation.is_none());
            let reference = stored
                .checkpoint
                .artifact_refs
                .iter()
                .find(|(key, _)| key.contains("_call_intent"))
                .unwrap()
                .1;
            let receipt: serde_json::Value = serde_json::from_slice(
                &runner
                    .artifacts
                    .call_receipt(&digest(reference.as_bytes()))
                    .unwrap()
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(
                runner
                    .artifacts
                    .read(receipt["raw_reference"].as_str().unwrap())
                    .unwrap(),
                body.as_bytes()
            );
            assert_eq!(receipt["proof"]["input_tokens"], 21);
            assert_eq!(fs::read(&calls).unwrap(), b"x");
            assert!(runner
                .store
                .claim_maintenance(&runner.owner, 900000)
                .await
                .unwrap()
                .is_none());
            let status = runner.status().await.unwrap();
            assert!(!serde_json::to_string(&status)
                .unwrap()
                .contains("raw_sha256"));
            assert_eq!(fs::read(calls).unwrap(), b"x");
        }
    }
    sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
        .execute(&scratch)
        .await
        .unwrap();
    let (dir, mut runner, mut config, repo, scan) = setup().await;
    let mut job = stage(&runner, &config, &repo, &scan, false).await;
    let proposal = draft(&config, &scan).proposal;
    let answer = dir.path().join("valid-answer");
    fs::write(&answer, serde_json::to_vec(&proposal).unwrap()).unwrap();
    let calls = dir.path().join("valid-calls");
    let executable = dir.path().join("valid-provider");
    fs::write(&executable,format!("#!/bin/sh\nresult=''\nwhile [ $# -gt 0 ]; do\nif [ \"$1\" = '--output-last-message' ]; then shift; result=$1; fi\nshift\ndone\ncat >/dev/null\nprintf x >> '{}'\ncp '{}' \"$result\"\nprintf '%s\\n' '{{\"type\":\"thread.started\",\"thread_id\":\"valid-author-run\"}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":21,\"output_tokens\":4}}}}'\n",calls.display(),answer.display())).unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    config.codex.executable = executable;
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    let intent = runner.artifacts.put(&serde_json::to_vec(&json!({
        "job":job.spec.id,"stage":"author","export_binding":author::export_binding(&config,&repo,&scan,&job.spec.target_path).unwrap(),
        "rejected_review":null,"model":config.codex.model,"effort":config.codex.reasoning_effort
    })).unwrap(), "json").unwrap();
    runner
        .store
        .set_maintenance_artifact(job.lease.as_ref().unwrap(), "author_call_intent", &intent)
        .await
        .unwrap();
    author::with_call_journal(
        runner.artifacts.clone(),
        digest(intent.as_bytes()),
        author::propose(&config, &repo, &scan, &job.spec.target_path),
    )
    .await
    .unwrap();
    runner.runtime.retry_delay_ms = 1000;
    runner
        .handle_failed_advance(
            &job,
            job.lease.as_ref().unwrap(),
            &anyhow::anyhow!("storage_commit_transient"),
        )
        .await
        .unwrap();
    let retry = runner
        .store
        .maintenance_job(&job.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(retry.status, MaintenanceStatus::Author);
    assert_eq!(retry.error_code.as_deref(), Some("STAGE_FAILED"));
    assert!(retry.lease.is_none());
    tokio::time::sleep(Duration::from_millis(1050)).await;
    job = runner
        .store
        .claim_maintenance(&runner.owner, 900000)
        .await
        .unwrap()
        .unwrap();
    runner.advance(&mut job).await.unwrap();
    let resumed = runner
        .store
        .maintenance_job(&job.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resumed.status, MaintenanceStatus::Reviewer);
    assert!(resumed.checkpoint.artifact_refs.contains_key("draft"));
    assert_eq!(fs::read(&calls).unwrap(), b"x");

    let unrelated = LocalImport {
        path: dir.path().join("unavailable-unrelated-package"),
        targets: BTreeSet::from(["internal/unrelated.html".into()]),
        sha256: digest(b"absent"),
        reviewer_id: "fixture".into(),
        review_ref: "reviewed".into(),
    };
    runner.runtime.local_imports.push(unrelated.clone());
    assert!(runner.local_document(&job).await.unwrap().is_none());
    let mut origin = scan.source_blobs[0].origin.clone();
    origin.policy = repo.document_policy.clone();
    origin.raw_sha256 = digest(proposal.content.as_bytes());
    let document = CoreDocument {
        logical_id: proposal.target,
        content: proposal.content,
        origin,
        metadata: BTreeMap::new(),
    };
    let bytes = serde_json::to_vec(&vec![document.clone()]).unwrap();
    let package = dir.path().join("matching-package.json");
    fs::write(&package, &bytes).unwrap();
    let import = LocalImport {
        path: package.clone(),
        targets: BTreeSet::from([document.logical_id.clone()]),
        sha256: digest(&bytes),
        reviewer_id: "fixture".into(),
        review_ref: "reviewed".into(),
    };
    let mut local = job.clone();
    local.spec = local_job_spec(&config, &repo, &document, &import.sha256);
    runner.runtime.local_imports.push(import);
    assert_eq!(
        runner.local_document(&local).await.unwrap().unwrap().0,
        document
    );
    fs::write(&package, b"changed").unwrap();
    assert_eq!(
        runner.local_document(&local).await.unwrap_err().to_string(),
        "local_import_changed_since_review"
    );
    runner.runtime.local_imports.last_mut().unwrap().sha256 = digest(b"changed");
    assert_eq!(
        runner.local_document(&local).await.unwrap_err().to_string(),
        "local_import_schema"
    );
    fs::remove_file(&package).unwrap();
    assert!(runner.local_document(&local).await.is_err());
    runner.runtime.local_imports = vec![unrelated];
    assert!(runner.local_document(&local).await.unwrap().is_none());
    sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
        .execute(&scratch)
        .await
        .unwrap();
    let (dir, runner, mut config, mut repo, _scan) = setup().await;
    let healthy_path = dir.path().join("healthy");
    git(
        dir.path(),
        &[
            "clone",
            "--no-hardlinks",
            repo.path.to_str().unwrap(),
            healthy_path.to_str().unwrap(),
        ],
    );
    let mut healthy = repo.clone();
    healthy.id = "healthy".into();
    healthy.path = healthy_path;
    healthy.origin = healthy.path.to_string_lossy().into_owned();
    healthy.doc_targets = vec!["internal/healthy.html".into()];
    healthy.policy.provider_egress_allowed = false;
    healthy.document_policy.provider_egress_allowed = false;
    healthy.code_only_export_approved = false;
    git(
        &healthy.path,
        &["remote", "set-url", "origin", &healthy.origin],
    );
    git(
        &healthy.path,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
    repo.origin = dir
        .path()
        .join("missing-remote")
        .to_string_lossy()
        .into_owned();
    git(&repo.path, &["remote", "set-url", "origin", &repo.origin]);
    repo.policy.provider_egress_allowed = false;
    repo.document_policy.provider_egress_allowed = false;
    repo.code_only_export_approved = false;
    config.repositories = vec![repo.clone(), healthy.clone()];
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    runner.register_config().await.unwrap();
    let status = runner.tick().await.unwrap();
    assert!(status["isolated_errors"]
        .as_array()
        .unwrap()
        .iter()
        .any(|error| error["repo"] == "source" && error["code"] == "SOURCE_FETCH_FAILED"));
    let jobs = runner
        .store
        .maintenance_jobs(&registration_id(&healthy, "internal/healthy.html"), 10)
        .await
        .unwrap();
    assert_eq!(jobs.len(), 1);
    assert!(jobs[0].attempts > 0);
    assert!(jobs[0].lease.is_none());
    assert!(jobs[0].checkpoint.publication.is_none());
    let stored_status: serde_json::Value =
        serde_json::from_slice(&fs::read(&runner.runtime.status_file).unwrap()).unwrap();
    assert_eq!(stored_status["isolated_errors"], status["isolated_errors"]);
    sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
        .execute(&scratch)
        .await
        .unwrap();
    let (_dir, runner, mut config, repo, scan) = setup().await;
    let mut transport_job = stage(&runner, &config, &repo, &scan, false).await;
    config.jev.endpoint = "https://127.0.0.1:1/".into();
    fs::write(
        &runner.runtime.maintenance_config,
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    let outcome = crate::triage::triage(
        &config,
        &repo,
        &scan,
        "internal/system.html",
        "fixture-unused",
    )
    .await
    .unwrap();
    assert_eq!(outcome.error_code.as_deref(), Some("JEV_TRANSPORT_FAILED"));
    let error = anyhow::anyhow!(outcome.error_code.unwrap());
    runner
        .handle_failed_advance(
            &transport_job,
            transport_job.lease.as_ref().unwrap(),
            &error,
        )
        .await
        .unwrap();
    transport_job = runner
        .store
        .maintenance_job(&transport_job.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        transport_job.error_code.as_deref(),
        Some("JEV_TRANSPORT_FAILED")
    );
    assert!(transport_job.lease.is_none());
    sqlx::query("TRUNCATE brain.maintenance_jobs_v1, brain.maintenance_sources_v1")
        .execute(&scratch)
        .await
        .unwrap();
    let (_dir, runner, config, repo, scan) = setup().await;
    let job = stage(&runner, &config, &repo, &scan, false).await;
    fs::write(
        repo.path.join("src/service.rs"),
        "pub const USER: u64 = 123456789012345678;\n",
    )
    .unwrap();
    pin(&repo.path);
    let error = scanner::scan(&config, &repo, None).await.unwrap_err();
    assert_eq!(error.to_string(), "context_platform_identifier");
    runner
        .handle_failed_advance(&job, job.lease.as_ref().unwrap(), &error)
        .await
        .unwrap();
    let current = runner
        .store
        .maintenance_job(&job.spec.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(current.status, MaintenanceStatus::Failed);
    assert_eq!(
        current.checkpoint.artifact_refs["blocked_reason"],
        "CONTEXT_REJECTED"
    );
    assert!(current.lease.is_none());
    assert!(!serde_json::to_string(&runner.status().await.unwrap())
        .unwrap()
        .contains("123456789012345678"));
    scratch.close().await;
}
