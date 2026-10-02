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
    let content = scan.documents["internal/system.html"].clone().unwrap();
    let citations = vec![Citation {
        path: scan.source_blobs[0].path.clone(),
        sha256: scan.source_blobs[0].sha256.clone(),
    }];
    let proposal = AuthorProposal {
        source_sha: scan.source_sha.clone(),
        target: "internal/system.html".into(),
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
async fn code_only_migration_keeps_previous_private_text_local_and_binds_all_rights() {
    let (_dir, mut config, mut repo) = fixture();
    let initial = scanner::scan(&config, &repo, None).await.unwrap();
    let target = "internal/system.html";
    let content = initial.documents[target].clone().unwrap();
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
