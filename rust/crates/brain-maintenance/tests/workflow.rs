use brain_contracts::{source::SourcePolicy, value::Observed, SourceVisibility};
use brain_maintenance::{
    author::{self, Action, AuthorProposal, Citation, IndependentReview, VerifiedDocument},
    config::{MaintenanceConfig, RepositoryConfig},
    digest, scanner,
};
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
        serde_json::from_str(include_str!("../config/smoke.example.json")).unwrap();
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

fn verified(config: &MaintenanceConfig, scan: &scanner::ScanResult) -> VerifiedDocument {
    verified_for(config, scan, "internal/system.html")
}
fn verified_for(
    config: &MaintenanceConfig,
    scan: &scanner::ScanResult,
    target: &str,
) -> VerifiedDocument {
    let content = scan.documents[target].clone().unwrap();
    let citations = vec![Citation {
        path: scan.source_blobs[0].path.clone(),
        sha256: scan.source_blobs[0].sha256.clone(),
    }];
    let proposal = AuthorProposal {
        source_sha: scan.source_sha.clone(),
        target: target.into(),
        before_sha256: Some(digest(content.as_bytes())),
        action: Action::Keep,
        content,
        citations: citations.clone(),
        open_questions: vec![],
    };
    let review = IndependentReview {
        approved: true,
        source_sha: scan.source_sha.clone(),
        proposal_sha256: digest(&serde_json::to_vec(&proposal).unwrap()),
        citations,
        findings: vec![],
    };
    VerifiedDocument {
        export_binding: Some(
            brain_maintenance::author::export_binding(
                config,
                &config.repositories[0],
                scan,
                target,
            )
            .unwrap(),
        ),
        reviewer_input_tokens: None,
        reviewer_output_tokens: None,
        proposal,
        review,
        evidence_sha256: digest(&serde_json::to_vec(scan).unwrap()),
        model: config.codex.model.clone(),
        prompt_version: config.prompt_version.clone(),
        author_run_id: "controlled-author-run".into(),
        reviewer_run_id: "controlled-review-run".into(),
    }
}

#[tokio::test]
async fn scan_references_keep_hashes_and_rehydrate_without_retaining_raw_text() {
    use brain_maintenance::integration::scan_checkpoint::ScanCheckpoint;
    let (_dir, config, repo) = fixture();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    assert!(scan
        .source_blobs
        .iter()
        .all(|blob| !blob.origin.policy.raw_retention_allowed));
    let checkpoint = ScanCheckpoint::capture(&scan, "explicit-reader".into());
    assert!(checkpoint
        .scan
        .source_blobs
        .iter()
        .all(|blob| blob.content.is_empty()));
    assert!(checkpoint
        .scan
        .documents
        .values()
        .flatten()
        .all(String::is_empty));
    let encoded = serde_json::to_vec(&checkpoint).unwrap();
    let mut resumed: ScanCheckpoint = serde_json::from_slice(&encoded).unwrap();
    resumed.hydrate_sources(&repo).unwrap();
    assert_eq!(
        serde_json::to_value(&resumed.scan.source_blobs).unwrap(),
        serde_json::to_value(&scan.source_blobs).unwrap()
    );
    assert_eq!(resumed.reader_release, "explicit-reader");
    resumed.scan.source_blobs[0].content = "verbotene persistierte Bytes".into();
    assert!(resumed.hydrate_sources(&repo).is_err());
}

#[tokio::test]
#[ignore = "requires dedicated /tmp/brain-maintenance-test-20261002/socket PostgreSQL fixture"]
async fn postgres_revocation_after_scan_blocks_actual_provider_dispatch() {
    use brain_contracts::maintenance::*;
    use brain_maintenance::integration::runner::guarded_provider_dispatch;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let (_dir, mut config, repo) = fixture();
    fs::create_dir_all(config.docs_repo.join("assets")).unwrap();
    fs::write(
        config.docs_repo.join("assets/probe.png"),
        b"controlled-image",
    )
    .unwrap();
    pin(&config.docs_repo);
    config
        .registered_assets
        .push(brain_maintenance::config::AssetProvenance {
            src: "/assets/probe.png".into(),
            git_path: "assets/probe.png".into(),
            sha256: digest(b"controlled-image"),
            source_locator: "controlled-test".into(),
            alt: "Prüfbild".into(),
        });
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host("/tmp/brain-maintenance-test-20261002/socket")
                .port(55447)
                .username("brain_maintenance_test")
                .database("postgres")
                .password(""),
        )
        .await
        .unwrap();
    let store = brain_storage::PgStore::new(pool);
    store.check_maintenance_schema().await.unwrap();
    let id = format!("provider-guard-{}", chrono::Utc::now().timestamp_micros());
    let spec = MaintenanceJobSpec {
        id: id.clone(),
        repo_id: brain_maintenance::integration::runner::registration_id(
            &repo,
            "internal/system.html",
        ),
        idempotency_key: id.clone(),
        source_sha: scan.source_sha.clone(),
        deployed_sha: None,
        target_path: "internal/system.html".into(),
        prompt_version: config.prompt_version.clone(),
        model: config.codex.model.clone(),
        policy: repo.document_policy.clone(),
    };
    let mut registration = MaintenanceSourceRegistration {
        repo_id: spec.repo_id.clone(),
        source_id: format!("maintenance-docs:{id}"),
        discovered_sha: scan.source_sha.clone(),
        policy: Some(spec.policy.clone()),
        authorization_ref: Some("controlled-test".into()),
    };
    store
        .register_maintenance_source(&registration)
        .await
        .unwrap();
    store
        .enqueue_maintenance(&spec, MaintenanceStatus::Planned)
        .await
        .unwrap();
    let calls = AtomicUsize::new(0);
    guarded_provider_dispatch(&store, &config, &repo, &scan, &spec, || async {
        calls.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })
    .await
    .unwrap();
    assert_eq!(calls.swap(0, Ordering::SeqCst), 1);
    registration.policy = None;
    registration.authorization_ref = None;
    store
        .register_maintenance_source(&registration)
        .await
        .unwrap();
    assert!(
        guarded_provider_dispatch(&store, &config, &repo, &scan, &spec, || async {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .await
        .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    registration.policy = Some(spec.policy.clone());
    registration.authorization_ref = Some("new-controlled-test-approval".into());
    store
        .register_maintenance_source(&registration)
        .await
        .unwrap();
    let mut revoked_repo = repo.clone();
    revoked_repo.code_only_export_approved = false;
    config.repositories = vec![revoked_repo.clone()];
    assert!(
        guarded_provider_dispatch(&store, &config, &revoked_repo, &scan, &spec, || async {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .await
        .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    config.repositories = vec![repo.clone()];
    config.registered_assets.clear();
    assert!(
        guarded_provider_dispatch(&store, &config, &repo, &scan, &spec, || async {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .await
        .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let stored = store.maintenance_job(&id).await.unwrap().unwrap();
    assert!(stored.checkpoint.publication.is_none());
    assert!(stored.checkpoint.activation.is_none());
    assert_eq!(stored.status, MaintenanceStatus::Planned);
}

#[tokio::test]
async fn code_only_migration_keeps_previous_private_text_local_and_binds_all_rights() {
    let (dir, mut config, mut repo) = fixture();
    let initial = scanner::scan(&config, &repo, None).await.unwrap();
    let target = "internal/system.html";
    let content = initial.documents[target].clone().unwrap();
    let mut rejected = verified_for(&config, &initial, target);
    rejected.review.approved = false;
    rejected.review.findings = vec!["GESPERRTER_NEGATIVBEFUND".into()];
    let mut origin = initial.source_blobs[0].origin.clone();
    origin.raw_sha256 = digest(content.as_bytes());
    origin.policy = repo.document_policy.clone();
    origin.policy.provider_egress_allowed = false;
    let document = brain_ingestion::document_set::CoreDocument {
        logical_id: target.into(),
        content,
        metadata: Default::default(),
        origin,
    };
    config.canonical_documents.insert(target.into(), document);
    assert!(scanner::scan(&config, &repo, None).await.is_err());
    repo.code_only_migration_targets.insert(target.into());
    config.repositories = vec![repo.clone()];
    let migrated = scanner::scan(&config, &repo, None).await.unwrap();
    assert!(migrated.documents[target].is_some());
    assert!(scanner::exportable_document(&config, &migrated, target).is_none());
    author::validate_provider_input(&config, &repo, &migrated, target)
        .await
        .unwrap();
    use std::os::unix::fs::PermissionsExt;
    let calls = dir.path().join("provider-calls");
    let executable = dir.path().join("provider-fixture");
    let answer = dir.path().join("answer.json");
    let mut proposal = rejected.proposal.clone();
    proposal.action = Action::SourceReview;
    proposal.content.clear();
    proposal.open_questions = vec!["Welcher Aufrufer ist belegt?".into()];
    fs::write(&answer, serde_json::to_vec(&proposal).unwrap()).unwrap();
    fs::write(&executable, format!("#!/bin/sh\nresult=''\nwhile [ $# -gt 0 ]; do\nif [ \"$1\" = '--output-last-message' ]; then shift; result=$1; fi\nshift\ndone\nprompt=$(cat)\ncase \"$prompt\" in *'Der Helfer liefert true.'*|*'GESPERRTER_NEGATIVBEFUND'*) exit 9 ;; esac\nprintf 'x' >> '{}'\ncp '{}' \"$result\"\nprintf '%s\\n' '{{\"type\":\"thread.started\",\"thread_id\":\"safe-new-author\"}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":8,\"output_tokens\":9}}}}'\n", calls.display(), answer.display())).unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    config.codex.executable = executable;
    let proof = author::CodexRunProof {
        run_id: rejected.author_run_id.clone(),
        model: config.codex.model.clone(),
        reasoning_effort: config.codex.reasoning_effort.clone(),
        exit_code: 0,
        event_types: vec![],
        tool_events: 0,
        input_tokens: None,
        output_tokens: None,
    };
    for binding in [
        rejected.export_binding.clone(),
        Some(author::export_binding(&config, &repo, &migrated, target).unwrap()),
    ] {
        let draft = author::DraftDocument {
            proposal: rejected.proposal.clone(),
            proof: proof.clone(),
            export_binding: binding,
        };
        assert!(author::review_draft(&config, &repo, &migrated, draft)
            .await
            .is_err());
        assert!(!calls.exists());
    }
    let payload: serde_json::Value = serde_json::from_str(
        &author::feedback_payload(&config, &repo, &migrated, target, &rejected).unwrap(),
    )
    .unwrap();
    assert_eq!(payload.as_object().unwrap().len(), 2);
    assert!(payload.get("previous_proposal").is_none());
    assert!(payload.get("findings").is_none());
    let new_draft =
        author::propose_with_feedback(&config, &repo, &migrated, target, Some(&rejected))
            .await
            .unwrap();
    author::validate_draft(&config, &repo, &migrated, &new_draft).unwrap();
    assert_eq!(fs::read(&calls).unwrap(), b"x");
    config
        .canonical_documents
        .get_mut(target)
        .unwrap()
        .origin
        .policy
        .publication_allowed = true;
    assert!(scanner::scan(&config, &repo, None).await.is_err());
}

#[tokio::test]
async fn document_export_is_guarded_before_any_provider_call() {
    let (_dir, mut config, mut repo) = fixture();
    repo.document_policy.provider_egress_allowed = false;
    config.repositories = vec![repo.clone()];
    config.codex.executable = "/nicht-vorhanden/codex".into();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let error = author::generate(&config, &repo, &scan, "internal/system.html")
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "Dokumentexport nicht freigegeben");
    let error = brain_maintenance::triage::triage(
        &config,
        &repo,
        &scan,
        "internal/system.html",
        "controlled-not-a-credential",
    )
    .await
    .unwrap_err();
    assert_eq!(error.to_string(), "Dokumentexport nicht freigegeben");

    let mut other_document = scan;
    other_document
        .documents
        .insert("internal/system.html".into(), None);
    other_document
        .documents
        .insert("internal/anderes.html".into(), Some("Interner Text".into()));
    let error = author::generate(&config, &repo, &other_document, "internal/system.html")
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Dokumentinventar weicht von der Registrierung ab"
    );
}

#[tokio::test]
async fn export_approval_revocation_blocks_cached_review_and_feedback_before_call() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, mut config, mut repo) = fixture();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let mut old = verified_for(&config, &scan, "internal/system.html");
    old.review.approved = false;
    let draft = author::DraftDocument {
        proposal: old.proposal.clone(),
        export_binding: old.export_binding.clone(),
        proof: author::CodexRunProof {
            run_id: old.author_run_id.clone(),
            model: config.codex.model.clone(),
            reasoning_effort: config.codex.reasoning_effort.clone(),
            exit_code: 0,
            event_types: vec![],
            tool_events: 0,
            input_tokens: None,
            output_tokens: None,
        },
    };
    let calls = dir.path().join("revoked-provider-calls");
    let executable = dir.path().join("revoked-provider");
    fs::write(
        &executable,
        format!("#!/bin/sh\nprintf 'x' >> '{}'\nexit 9\n", calls.display()),
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    config.codex.executable = executable;
    repo.code_only_export_approved = false;
    config.repositories = vec![repo.clone()];
    assert!(author::review_draft(&config, &repo, &scan, draft)
        .await
        .is_err());
    assert!(author::propose_with_feedback(
        &config,
        &repo,
        &scan,
        "internal/system.html",
        Some(&old)
    )
    .await
    .is_err());
    assert!(
        author::propose(&config, &repo, &scan, "internal/system.html")
            .await
            .is_err()
    );
    assert!(!calls.exists());
    assert!(
        author::validate_provider_input(&config, &repo, &scan, "internal/system.html")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn provider_dispatch_rejects_foreign_or_unregistered_evidence() {
    let (_dir, mut config, repo) = fixture();
    config.codex.executable = "/nicht-vorhanden/codex".into();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let mut foreign = scan.clone();
    foreign.repo_id = "anderes-repo".into();
    for field in 0..3 {
        let mut forged = scan.clone();
        match field {
            0 => forged.source_blobs[0].origin.identity.logical_id = "anderer-pfad.rs".into(),
            1 => {
                forged.source_blobs[0].origin.source_revision =
                    brain_contracts::source::SourceRevision::Git {
                        commit: "0".repeat(40),
                    }
            }
            _ => forged.source_blobs[0].origin.raw_sha256 = "0".repeat(64),
        }
        assert!(
            author::validate_provider_input(&config, &repo, &forged, "internal/system.html")
                .await
                .is_err()
        );
    }
    for input in [&foreign, &scan] {
        let target = if input.repo_id == repo.id {
            "internal/fremd.html"
        } else {
            "internal/system.html"
        };
        assert!(author::generate(&config, &repo, input, target)
            .await
            .is_err());
        assert!(brain_maintenance::triage::triage(
            &config,
            &repo,
            input,
            target,
            "controlled-not-a-credential"
        )
        .await
        .is_err());
    }
    let mut altered = scan;
    altered
        .documents
        .insert("internal/system.html".into(), Some("Fremder Inhalt".into()));
    let error = author::validate_provider_input(&config, &repo, &altered, "internal/system.html")
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Dokumenttext weicht vom gepinnten Blob ab"
    );
}

#[tokio::test]
async fn pinned_source_ignores_dirty_checkout_and_doc_only_commits() {
    let (_dir, config, repo) = fixture();
    let first = scanner::scan(&config, &repo, None).await.unwrap();
    fs::write(repo.path.join("src/service.rs"), "uncommitted false").unwrap();
    assert_eq!(
        scanner::scan(&config, &repo, Some(&first.source_sha))
            .await
            .unwrap()
            .source_blobs[0]
            .content,
        first.source_blobs[0].content
    );
    git(&repo.path, &["restore", "src/service.rs"]);
    fs::write(repo.path.join("README.md"), "Nur Dokumentation.\n").unwrap();
    pin(&repo.path);
    let second = scanner::scan(&config, &repo, Some(&first.source_sha))
        .await
        .unwrap();
    assert!(!second.source_changed);
    assert_eq!(second.source_fingerprint, first.source_fingerprint);
    assert!(
        scanner::relevant_source_matches(&config, &repo, &first.source_sha)
            .await
            .unwrap()
    );
    let retained = scanner::scan_pinned(
        &config,
        &repo,
        Some(&first.source_sha),
        Some(&first.source_sha),
    )
    .await
    .unwrap();
    assert_eq!(retained.source_sha, first.source_sha);
    assert!(
        author::validate_provider_input(&config, &repo, &retained, "internal/system.html")
            .await
            .is_ok()
    );
    let paid = verified(&config, &first);
    author::prepare_document(&config, &repo, &first, &paid)
        .await
        .unwrap();
    fs::write(
        config.docs_repo.join("README.md"),
        "Unabhängige Gitänderung.\n",
    )
    .unwrap();
    pin(&config.docs_repo);
    author::prepare_document(&config, &repo, &first, &paid)
        .await
        .unwrap();
    fs::write(
        config.docs_repo.join("internal/system.html"),
        "Schmutziger Checkout ohne Quelländerung.",
    )
    .unwrap();
    author::prepare_document(&config, &repo, &first, &paid)
        .await
        .unwrap();
}

#[tokio::test]
async fn markdown_keep_retains_format_and_resume_path() {
    let (_dir, mut config, mut repo) = fixture();
    let target = "internal/system.md";
    let text = "# Prüfbeleg\n\nDer Code verwendet `Option<T>` und bleibt unverändert.\n";
    fs::remove_file(config.docs_repo.join("internal/system.html")).unwrap();
    fs::write(config.docs_repo.join(target), text).unwrap();
    pin(&config.docs_repo);
    repo.doc_targets = vec![target.into()];
    repo.output_paths
        .insert(target.into(), "internal/system.html".into());
    config.repositories = vec![repo.clone()];
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let proof = verified_for(&config, &scan, target);
    let document = author::prepare_document(&config, &repo, &scan, &proof)
        .await
        .unwrap();
    assert_eq!(document.content, text);
    assert_eq!(document.metadata["output_path"], target);
    assert_eq!(document.metadata["content_format"], "markdown");
    config.canonical_documents.insert(target.into(), document);
    let resumed = scanner::scan(&config, &repo, Some(&scan.source_sha))
        .await
        .unwrap();
    assert_eq!(resumed.document_paths[target], target);
    assert_eq!(resumed.documents[target].as_deref(), Some(text));
    author::prepare_document(
        &config,
        &repo,
        &resumed,
        &verified_for(&config, &resumed, target),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn legacy_core_html_release_projects_without_mutating_historical_provenance() {
    use brain_contracts::*;
    use brain_ingestion::document_set::{prepare_document_batch, CoreDocument, DocumentSetSource};
    let (_dir, config, repo) = fixture();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let target = "internal/system.html";
    let content = scan.documents[target].clone().unwrap();
    let mut origin = scan.source_blobs[0].origin.clone();
    origin.raw_sha256 = digest(content.as_bytes());
    origin.policy = repo.document_policy.clone();
    let document = CoreDocument {
        logical_id: target.into(),
        content,
        origin,
        metadata: std::collections::BTreeMap::from([
            ("content_format".into(), "html".into()),
            ("parser".into(), "reviewed-docs-v1".into()),
        ]),
    };
    let source = DocumentSetSource {
        source_id: "legacy-reviewed-html".into(),
        configuration: "legacy-v1".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: repo.document_policy.allowed_scopes.clone(),
        tombstone_metadata: Default::default(),
    };
    let batch = prepare_document_batch(&source, &[document], None).unwrap();
    let original = batch.records[0].clone();
    assert!(!original.metadata.contains_key("html_projection_version"));
    let store = brain_storage::MemoryRepository::default();
    store.apply_record(original.clone()).unwrap();
    let release = store
        .release_from_heads("legacy-html", "legacy-v1", "fixture")
        .unwrap();
    store.publish(&release).await.unwrap();
    let retriever = dbrain_retrieval::ReleaseRetriever::new(store.clone(), 3);
    let query = Query {
        request_id: "legacy-q".into(),
        conversation_id: "legacy-c".into(),
        text: "Helfer".into(),
        requested_scopes: Default::default(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
        domain: None,
    };
    let context = AuthorizedContext {
        request_deadline: None,
        principal: Principal {
            actor_id: "operator-fixture".into(),
            channel: "local-fixture".into(),
            scopes: repo.document_policy.allowed_scopes.clone(),
            provider_egress: BTreeSet::from(["internal".into()]),
        },
        conversation_id: query.conversation_id.clone(),
        knowledge_release: release.release_id.clone(),
        deadline_ms: 8000,
        budget: Budget::default(),
    };
    let hits = retriever.retrieve(&query, &context).unwrap();
    assert!(!hits.is_empty());
    assert!(hits.iter().all(|hit| !hit.content.contains('<')));
    retriever
        .validate_evidence(&query, &context, &hits, false)
        .unwrap();
    retriever
        .validate_evidence(&query, &context, &hits, true)
        .unwrap();
    assert_eq!(
        store.read_snapshot(&release.release_id).unwrap().revisions[0],
        original
    );
    let mut inconsistent = original;
    inconsistent
        .metadata
        .insert("html_projection_version".into(), "unbound".into());
    let other = brain_storage::MemoryRepository::default();
    other.apply_record(inconsistent).unwrap();
    other.publish(&release).await.unwrap();
    assert!(dbrain_retrieval::ReleaseRetriever::new(other, 3)
        .retrieve(&query, &context)
        .is_err());
}

#[test]
fn source_rollback_creates_new_transition_but_unchanged_tick_has_none() {
    use brain_maintenance::integration::runner::automatic_job_spec;
    let (_dir, config, repo) = fixture();
    let target = "internal/system.html";
    let a = "a".repeat(40);
    let b = "b".repeat(40);
    let first = automatic_job_spec(&config, &repo, target, &a, "initial", None).unwrap();
    assert!(automatic_job_spec(&config, &repo, target, &a, "active-a", Some(&first)).is_none());
    let second = automatic_job_spec(&config, &repo, target, &b, "active-a", Some(&first)).unwrap();
    let rollback =
        automatic_job_spec(&config, &repo, target, &a, "active-b", Some(&second)).unwrap();
    assert_ne!(rollback.id, first.id);
    assert!(automatic_job_spec(
        &config,
        &repo,
        target,
        &a,
        "active-rollback",
        Some(&rollback)
    )
    .is_none());
    let again = automatic_job_spec(
        &config,
        &repo,
        target,
        &b,
        "active-rollback",
        Some(&rollback),
    )
    .unwrap();
    assert_ne!(again.id, second.id);
}

#[tokio::test]
async fn staging_does_not_touch_canonical_docs_and_uses_document_policy() {
    let (_dir, config, repo) = fixture();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let proof = verified(&config, &scan);
    let original = fs::read(config.docs_repo.join("internal/system.html")).unwrap();
    let staged = author::stage_document(&config, &repo, &scan, &proof)
        .await
        .unwrap();
    assert_eq!(
        fs::read(config.docs_repo.join("internal/system.html")).unwrap(),
        original
    );
    assert!(staged
        .directory
        .path()
        .join(&staged.relative_path)
        .is_file());
    assert_eq!(staged.document.origin.policy, repo.document_policy);
}

#[tokio::test]
async fn resumed_proofs_and_changed_source_are_rejected_before_staging() {
    let (_dir, config, repo) = fixture();
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let proof = verified(&config, &scan);
    let mut wrong = proof.clone();
    wrong.reviewer_run_id = wrong.author_run_id.clone();
    assert!(author::prepare_document(&config, &repo, &scan, &wrong)
        .await
        .is_err());
    wrong = proof.clone();
    wrong.review.source_sha = "0".repeat(40);
    assert!(author::prepare_document(&config, &repo, &scan, &wrong)
        .await
        .is_err());
    fs::write(
        repo.path.join("src/service.rs"),
        "pub fn bereit() -> bool { false }\n",
    )
    .unwrap();
    pin(&repo.path);
    assert!(author::prepare_document(&config, &repo, &scan, &proof)
        .await
        .is_err());
}

#[test]
fn codex_tool_events_are_rejected_even_on_successful_turns() {
    let (_dir, config, _repo) = fixture();
    let raw=b"{\"type\":\"thread.started\",\"thread_id\":\"test\"}\n{\"type\":\"item.completed\",\"item\":{\"type\":\"command_execution\"}}\n{\"type\":\"turn.completed\"}\n";
    assert!(author::validate_events(&config, raw).is_err());
}

#[tokio::test]
async fn persisted_scan_cannot_resume_after_registry_revoke() {
    let (dir, mut config, repo) = fixture();
    let path = dir.path().join("maintenance.json");
    fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    config.loaded_from = Some(path.clone());
    let scan = scanner::scan(&config, &repo, None).await.unwrap();
    let proof = verified(&config, &scan);
    let mut revoked = config.clone();
    revoked.repositories[0].policy.provider_egress_allowed = false;
    revoked.repositories[0]
        .document_policy
        .provider_egress_allowed = false;
    fs::write(&path, serde_json::to_vec(&revoked).unwrap()).unwrap();
    assert!(author::prepare_document(&config, &repo, &scan, &proof)
        .await
        .is_err());
    assert!(scanner::scan(&config, &repo, None).await.is_err());
}
