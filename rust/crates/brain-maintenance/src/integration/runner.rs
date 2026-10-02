use super::{
    activation::ActivationPlan,
    artifacts::Artifacts,
    runtime_config::{read_bounded, RuntimeConfig},
    scan_checkpoint::ScanCheckpoint,
};
use crate::{
    author,
    config::{MaintenanceConfig, RepositoryConfig},
    digest,
    lease::while_leased,
    scanner, triage,
};
use anyhow::{ensure, Context, Result};
use brain_contracts::{
    maintenance::*,
    source::{origin_from_record, SourceRevision},
    DocumentStorePort, PortError,
};
use brain_ingestion::document_set::{prepare_document_batch, CoreDocument, DocumentSetSource};
use brain_storage::PgStore;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{collections::BTreeMap, io::Write, path::Path, time::Duration};
use zeroize::Zeroizing;

pub fn load_maintenance(path: &Path) -> Result<MaintenanceConfig> {
    let mut config: MaintenanceConfig = serde_json::from_slice(&read_bounded(path, 256 * 1024)?)
        .map_err(|_| anyhow::anyhow!("maintenance_config_schema"))?;
    config.validate()?;
    config.loaded_from = Some(path.to_owned());
    Ok(config)
}

pub struct Runner {
    runtime: RuntimeConfig,
    store: PgStore,
    artifacts: Artifacts,
    jev: Zeroizing<String>,
    owner: String,
}

/// Prüft Rechte und Belege, bevor der Provider überhaupt aufgerufen wird.
pub async fn guarded_provider_dispatch<T, F, Fut>(
    store: &PgStore,
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    scan: &scanner::ScanResult,
    job: &MaintenanceJobSpec,
    dispatch: F,
) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    ensure!(
        job.repo_id == registration_id(repo, &job.target_path)
            && job.policy == *repo.document_policy_for(&job.target_path)
            && job.source_sha == scan.source_sha,
        "provider_job_binding"
    );
    let guard = store.lock_maintenance_provider_policy(job).await?;
    author::validate_provider_input(config, repo, scan, &job.target_path).await?;
    let result = dispatch().await?;
    guard.commit().await?;
    Ok(result)
}

pub fn registration_id(repo: &RepositoryConfig, target: &str) -> String {
    format!("{}:{}", repo.id, &digest(target.as_bytes())[..24])
}
fn source_id(repo: &RepositoryConfig, target: &str) -> String {
    format!("maintenance-docs:{}", registration_id(repo, target))
}
fn document_is_html(metadata: &BTreeMap<String, String>) -> bool {
    metadata
        .get("content_format")
        .is_some_and(|format| format == "html")
        || metadata
            .get("output_path")
            .is_some_and(|path| path.ends_with(".html"))
}
fn job_spec(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    target: &str,
    sha: &str,
    deployed: Option<String>,
    salt: &str,
) -> MaintenanceJobSpec {
    let key = digest(
        &serde_json::to_vec(&json!([
            repo.id,
            target,
            sha,
            config.prompt_version,
            config.codex.model,
            repo.document_policy_for(target),
            salt
        ]))
        .expect("JSON-Daten"),
    );
    MaintenanceJobSpec {
        id: format!("docs-{key}"),
        repo_id: registration_id(repo, target),
        source_sha: sha.into(),
        deployed_sha: deployed,
        target_path: target.into(),
        idempotency_key: key,
        prompt_version: config.prompt_version.clone(),
        model: config.codex.model.clone(),
        policy: repo.document_policy_for(target).clone(),
    }
}
pub fn automatic_job_spec(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    target: &str,
    sha: &str,
    active_release: &str,
    previous: Option<&MaintenanceJobSpec>,
) -> Option<MaintenanceJobSpec> {
    if previous.is_some_and(|job| {
        job.source_sha == sha
            && job.prompt_version == config.prompt_version
            && job.model == config.codex.model
            && job.policy == *repo.document_policy_for(target)
    }) {
        return None;
    }
    let transition = previous.map(|job| job.id.as_str()).unwrap_or("initial");
    Some(job_spec(
        config,
        repo,
        target,
        sha,
        None,
        &format!("automatic:{active_release}:{transition}"),
    ))
}

fn local_job_spec(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    document: &CoreDocument,
    package_hash: &str,
) -> MaintenanceJobSpec {
    let SourceRevision::Git { commit } = &document.origin.source_revision else {
        unreachable!("Geprüfte Gitquelle");
    };
    let mut spec = job_spec(
        config,
        repo,
        &document.logical_id,
        commit,
        None,
        package_hash,
    );
    let key = digest(
        &serde_json::to_vec(&json!([repo.id, document.logical_id, document])).expect("JSON-Daten"),
    );
    spec.id = format!("local-{key}");
    spec.idempotency_key = spec.id.clone();
    spec.policy = document.origin.policy.clone();
    spec
}

impl Runner {
    pub async fn open(runtime: RuntimeConfig) -> Result<Self> {
        let config = load_maintenance(&runtime.maintenance_config)?;
        let snapshot: BTreeMap<_, _> = dl_token_secrets::values(&runtime.infisical_config)
            .await
            .map_err(|_| anyhow::anyhow!("secret_source_unavailable"))?
            .into_iter()
            .collect();
        let jev = snapshot
            .get(&runtime.jev_secret)
            .cloned()
            .context("jev_secret_missing")?;
        let pg = &runtime.postgres;
        let mut options = PgConnectOptions::new_without_pgpass()
            .host(pg.socket_dir.to_str().context("postgres_socket")?)
            .port(pg.port)
            .username(&pg.username)
            .database(&pg.database)
            .password("")
            .ssl_mode(sqlx::postgres::PgSslMode::Disable)
            .options([("statement_timeout", "120000"), ("lock_timeout", "10000")]);
        if pg.auth == brain_serve::config::DatabaseAuth::Password {
            let name = pg.password_env.as_deref().context("postgres_secret_name")?;
            options = options.password(snapshot.get(name).context("postgres_secret_missing")?);
        }
        let pool = PgPoolOptions::new()
            .max_connections(pg.max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .connect_with(options)
            .await
            .map_err(|_| anyhow::anyhow!("maintenance_database_unavailable"))?;
        let artifacts = Artifacts::open(&runtime.artifact_dir)?;
        let owner = format!(
            "brain-maintain-{}-{}",
            std::process::id(),
            chrono::Utc::now().timestamp_micros()
        );
        config.validate()?;
        Ok(Self {
            store: PgStore::new(pool),
            runtime,
            artifacts,
            jev,
            owner,
        })
    }

    pub async fn migrate(&self) -> Result<()> {
        ensure!(
            self.runtime.postgres.username == "brain_migrate",
            "migration_owner_required"
        );
        self.store.migrate_maintenance().await?;
        Ok(())
    }

    async fn registration(&self, repo: &RepositoryConfig, target: &str, sha: &str) -> Result<()> {
        self.store
            .advance_maintenance_source(&MaintenanceSourceRegistration {
                repo_id: registration_id(repo, target),
                source_id: source_id(repo, target),
                discovered_sha: sha.into(),
                policy: Some(repo.document_policy_for(target).clone()),
                authorization_ref: Some("workspace-documentation-maintenance-2026-10-02".into()),
            })
            .await?;
        Ok(())
    }

    async fn check_registry_policy(&self, repo: &RepositoryConfig, target: &str) -> Result<()> {
        ensure!(
            self.store
                .maintenance_sources()
                .await?
                .iter()
                .any(
                    |registration| registration.repo_id == registration_id(repo, target)
                        && registration.source_id == source_id(repo, target)
                        && registration.policy.as_ref() == Some(repo.document_policy_for(target))
                ),
            "registry_policy_revoked_or_changed"
        );
        Ok(())
    }

    pub async fn register_config(&self) -> Result<serde_json::Value> {
        self.require_operator()?;
        let config = load_maintenance(&self.runtime.maintenance_config)?;
        for repo in &config.repositories {
            for target in &repo.doc_targets {
                let sha =
                    scanner::resolve_ref(&repo.path, &repo.source_ref, &config.bounds).await?;
                self.store
                    .register_maintenance_source(&MaintenanceSourceRegistration {
                        repo_id: registration_id(repo, target),
                        source_id: source_id(repo, target),
                        discovered_sha: sha,
                        policy: Some(repo.document_policy_for(target).clone()),
                        authorization_ref: Some(format!(
                            "explicit-config-{}",
                            digest(&read_bounded(&self.runtime.maintenance_config, 256 * 1024)?)
                        )),
                    })
                    .await?;
            }
        }
        self.status().await
    }

    fn require_operator(&self) -> Result<u32> {
        use std::os::unix::fs::MetadataExt;
        let uid = nix::unistd::geteuid().as_raw();
        let metadata = std::fs::symlink_metadata(&self.runtime.maintenance_config)?;
        ensure!(
            uid == 1000 && (metadata.uid() == uid || metadata.uid() == 0),
            "operator_identity_required"
        );
        Ok(uid)
    }

    pub async fn tick(&self) -> Result<serde_json::Value> {
        self.store.check_maintenance_schema().await?;
        let config = load_maintenance(&self.runtime.maintenance_config)?;
        let known: std::collections::BTreeSet<_> = config
            .repositories
            .iter()
            .flat_map(|repo| {
                repo.doc_targets
                    .iter()
                    .map(move |target| source_id(repo, target))
            })
            .collect();
        for mut registration in self.store.maintenance_sources().await? {
            if registration.source_id.starts_with("maintenance-docs:")
                && !known.contains(&registration.source_id)
            {
                registration.policy = None;
                registration.authorization_ref = None;
                self.store
                    .register_maintenance_source(&registration)
                    .await?;
            }
        }
        for repo in &config.repositories {
            crate::config::require_registered(&config, repo)?;
            let branch = repo
                .source_ref
                .strip_prefix("refs/remotes/origin/")
                .context("source_ref")?;
            crate::process::run(
                Path::new("/usr/bin/git"),
                &[
                    "-c".into(),
                    "core.hooksPath=/dev/null".into(),
                    "fetch".into(),
                    "--no-tags".into(),
                    "origin".into(),
                    branch.into(),
                ],
                &repo.path,
                &[],
                config.bounds.git_timeout_ms,
                65536,
            )
            .await?;
            let sha = scanner::resolve_ref(&repo.path, &repo.source_ref, &config.bounds).await?;
            for target in &repo.doc_targets {
                let recent = self
                    .store
                    .maintenance_jobs(&registration_id(repo, target), 100)
                    .await?;
                let active_release = brain_serve::Config::load(&self.runtime.serve_config)?
                    .release
                    .id;
                let Some(spec) = automatic_job_spec(
                    &config,
                    repo,
                    target,
                    &sha,
                    &active_release,
                    recent.first().map(|job| &job.spec),
                ) else {
                    self.registration(repo, target, &sha).await?;
                    continue;
                };
                let previous = recent.first().map(|j| j.spec.source_sha.as_str());
                let changed = match previous {
                    Some(previous) if previous != sha => {
                        dbrain_sources::git_source::PinnedRepository::open(&repo.path, previous)?
                            .diff(&sha)?
                            .into_iter()
                            .flat_map(|change| change.old_path.into_iter().chain(change.new_path))
                            .any(|path| {
                                repo.source_paths.iter().any(|scope| {
                                    path == *scope || path.starts_with(&format!("{scope}/"))
                                }) && crate::scanner::approved_code_path(&path)
                            })
                    }
                    Some(_) => false,
                    None => true,
                };
                if previous.is_some()
                    && !changed
                    && recent.first().is_some_and(|j| {
                        j.spec.policy == *repo.document_policy_for(target)
                            && j.spec.prompt_version == config.prompt_version
                            && j.spec.model == config.codex.model
                    })
                {
                    self.registration(repo, target, previous.context("previous_source")?)
                        .await?;
                    continue;
                }
                self.registration(repo, target, &sha).await?;
                self.store
                    .enqueue_maintenance(&spec, MaintenanceStatus::Planned)
                    .await?;
            }
        }
        for discovered in scanner::discover(&config)? {
            let identity = format!(
                "discovered:{}",
                digest(discovered.path.to_string_lossy().as_bytes())
            );
            self.store
                .register_maintenance_source(&MaintenanceSourceRegistration {
                    repo_id: identity,
                    source_id: "policy_pending".into(),
                    discovered_sha: "unverified".into(),
                    policy: None,
                    authorization_ref: None,
                })
                .await?;
        }
        self.enqueue_local_imports(&config).await?;
        for _ in 0..self.runtime.max_jobs_per_tick * 5 {
            let Some(mut job) = self
                .store
                .claim_maintenance(&self.owner, self.runtime.lease_ttl_ms)
                .await?
            else {
                break;
            };
            let lease = job.lease.clone().context("claimed_job_without_lease")?;
            let config = load_maintenance(&self.runtime.maintenance_config)?;
            if let Some(repo) = config
                .repositories
                .iter()
                .find(|r| registration_id(r, &job.spec.target_path) == job.spec.repo_id)
            {
                let current =
                    scanner::resolve_ref(&repo.path, &repo.source_ref, &config.bounds).await?;
                if !job.checkpoint.artifact_refs.contains_key("local_review")
                    && self.local_document(&job).await?.is_none()
                    && (!scanner::relevant_source_matches(&config, repo, &job.spec.source_sha)
                        .await?
                        || job.spec.policy != *repo.document_policy_for(&job.spec.target_path))
                {
                    if let Some(replacement) = self
                        .store
                        .maintenance_jobs(&job.spec.repo_id, 100)
                        .await?
                        .iter()
                        .find(|j| {
                            j.spec.source_sha == current
                                && j.spec.policy == *repo.document_policy_for(&job.spec.target_path)
                                && j.spec.id != job.spec.id
                        })
                    {
                        self.store
                            .supersede_maintenance(&lease, &replacement.spec.id)
                            .await?;
                        continue;
                    }
                }
            }
            if let Err(error) = self.advance(&mut job).await {
                // Nur ein stabiler Fehlercode wird gespeichert, niemals Inhalte aus Abhängigkeiten.
                let code = if error.to_string().contains("review") {
                    "REVIEW_REQUIRED"
                } else {
                    "STAGE_FAILED"
                };
                self.store
                    .retry_maintenance(&lease, code, self.runtime.retry_delay_ms)
                    .await?;
            }
        }
        self.status().await
    }

    async fn enqueue_local_imports(&self, config: &MaintenanceConfig) -> Result<()> {
        for import in &self.runtime.local_imports {
            let bytes = read_bounded(&import.path, 8 * 1024 * 1024)?;
            ensure!(
                digest(&bytes) == import.sha256,
                "local_import_changed_since_review"
            );
            let documents: Vec<CoreDocument> = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("local_import_schema"))?;
            ensure!(documents.len() <= 64, "local_import_limit");
            for document in documents {
                let repo = config
                    .repositories
                    .iter()
                    .find(|r| r.doc_targets.contains(&document.logical_id))
                    .context("local_import_unregistered_target")?;
                let SourceRevision::Git { commit } = &document.origin.source_revision else {
                    anyhow::bail!("local_import_source_revision");
                };
                let spec = local_job_spec(config, repo, &document, &import.sha256);
                if self.store.maintenance_job(&spec.id).await?.is_some() {
                    continue;
                }
                ensure!(
                    document.origin.policy == *repo.document_policy_for(&document.logical_id),
                    "local_import_policy"
                );
                ensure!(
                    digest(document.content.as_bytes()) == document.origin.raw_sha256,
                    "local_import_hash"
                );
                crate::html::validate_html_with_assets(
                    None,
                    &document.content,
                    &config.registered_assets,
                )?;
                let pinned =
                    dbrain_sources::git_source::PinnedRepository::open(&repo.path, commit)?;
                pinned.require_origin(&[&repo.origin])?;
                self.check_registry_policy(repo, &document.logical_id)
                    .await?;
                self.store
                    .enqueue_maintenance(&spec, MaintenanceStatus::Planned)
                    .await?;
                self.artifacts.put(&serde_json::to_vec(&json!({"document":document,"reviewer_id":import.reviewer_id,"review_ref":import.review_ref,"package_sha256":import.sha256}))?,"json")?;
            }
        }
        Ok(())
    }

    async fn local_document(
        &self,
        job: &MaintenanceJob,
    ) -> Result<Option<(CoreDocument, String, String)>> {
        for import in &self.runtime.local_imports {
            let bytes = read_bounded(&import.path, 8 * 1024 * 1024)?;
            ensure!(
                digest(&bytes) == import.sha256,
                "local_import_changed_since_review"
            );
            let docs: Vec<CoreDocument> = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("local_import_schema"))?;
            for document in docs {
                if document.logical_id == job.spec.target_path
                    && matches!(&document.origin.source_revision,SourceRevision::Git{commit} if commit==&job.spec.source_sha)
                {
                    let config = load_maintenance(&self.runtime.maintenance_config)?;
                    let repo = config
                        .repositories
                        .iter()
                        .find(|r| r.doc_targets.contains(&document.logical_id))
                        .context("local_repo")?;
                    let expected = local_job_spec(&config, repo, &document, &import.sha256);
                    if expected.id == job.spec.id {
                        return Ok(Some((
                            document,
                            import.reviewer_id.clone(),
                            import.review_ref.clone(),
                        )));
                    }
                }
            }
        }
        Ok(None)
    }

    async fn advance(&self, job: &mut MaintenanceJob) -> Result<()> {
        let (config, reader_release) = self.current_config().await?;
        let repo = config
            .repositories
            .iter()
            .find(|r| registration_id(r, &job.spec.target_path) == job.spec.repo_id)
            .context("job_repo_unregistered")?;
        let current_registry = self.store.maintenance_sources().await?;
        ensure!(
            current_registry
                .iter()
                .any(|r| r.repo_id == job.spec.repo_id
                    && r.policy.as_ref() == Some(&job.spec.policy)),
            "registry_policy_revoked_or_changed"
        );
        ensure!(
            repo.document_policy_for(&job.spec.target_path) == &job.spec.policy,
            "job_policy_revoked"
        );
        ensure!(
            job.checkpoint.artifact_refs.contains_key("local_review")
                || self.local_document(job).await?.is_some()
                || scanner::relevant_source_matches(&config, repo, &job.spec.source_sha).await?,
            "job_source_superseded"
        );
        let mut lease = job.lease.clone().context("job_lease")?;
        let mut checkpoint = job.checkpoint.clone();
        if job.checkpoint.publication.is_none()
            && matches!(
                job.status,
                MaintenanceStatus::Author
                    | MaintenanceStatus::Reviewer
                    | MaintenanceStatus::Publish
            )
        {
            if let Some(reference) = checkpoint.artifact_refs.get("scan") {
                let scan = self.read_scan(&config, repo, reference).await?;
                if let Err(error) =
                    author::validate_provider_input(&config, repo, &scan, &job.spec.target_path)
                        .await
                {
                    if error.to_string() == "Dokumenttext weicht vom gepinnten Blob ab" {
                        let archive = self
                            .artifacts
                            .put(&serde_json::to_vec(&checkpoint)?, "json")?;
                        self.store
                            .reset_maintenance_inputs(&lease, &archive)
                            .await?;
                        return Ok(());
                    }
                    return Err(error);
                }
            }
        }
        let next = match job.status {
            MaintenanceStatus::Planned => MaintenanceStatus::SourceReview,
            MaintenanceStatus::SourceReview => {
                if let Some(unactivated) =
                    self.store.maintenance_unactivated_basis(&job.spec).await?
                {
                    checkpoint.artifact_refs.insert(
                        "unactivated_basis".into(),
                        serde_json::to_string(&unactivated)?,
                    );
                }
                checkpoint.artifact_refs.insert(
                    "before_document_sha256".into(),
                    config
                        .canonical_documents
                        .get(&job.spec.target_path)
                        .map(|doc| digest(doc.content.as_bytes()))
                        .unwrap_or_else(|| "missing".into()),
                );
                if let Some((document, reviewer, reference)) = self.local_document(job).await? {
                    checkpoint.artifact_refs.insert(
                        "document".into(),
                        self.artifacts
                            .put(&serde_json::to_vec(&document)?, "json")?,
                    );
                    checkpoint
                        .artifact_refs
                        .insert("local_review".into(), reference);
                    checkpoint.review = Some(MaintenanceReviewProof {
                        reviewer_id: reviewer.clone(),
                        author_run_id: format!("local-author:{}", job.spec.id),
                        reviewer_run_id: format!("local-review:{reviewer}"),
                        source_sha: job.spec.source_sha.clone(),
                        document_sha256: digest(document.content.as_bytes()),
                        accepted: true,
                    });
                    MaintenanceStatus::Reviewer
                } else {
                    let before_sha = config
                        .canonical_documents
                        .get(&job.spec.target_path)
                        .and_then(|doc| match &doc.origin.source_revision {
                            SourceRevision::Git { commit } => Some(commit.as_str()),
                            _ => None,
                        });
                    let scan =
                        scanner::scan_pinned(&config, repo, before_sha, Some(&job.spec.source_sha))
                            .await?;
                    ensure!(scan.source_sha == job.spec.source_sha, "job_scan_revision");
                    let triage = guarded_provider_dispatch(
                        &self.store,
                        &config,
                        repo,
                        &scan,
                        &job.spec,
                        || {
                            while_leased(
                                &self.store,
                                &mut lease,
                                self.runtime.lease_ttl_ms,
                                triage::triage(
                                    &config,
                                    repo,
                                    &scan,
                                    &job.spec.target_path,
                                    &self.jev,
                                ),
                            )
                        },
                    )
                    .await?;
                    ensure!(triage.error_code.is_none(), "jev_review_transport");
                    checkpoint.artifact_refs.insert(
                        "scan".into(),
                        self.artifacts.put(
                            &serde_json::to_vec(&ScanCheckpoint::capture(
                                &scan,
                                reader_release.clone(),
                            ))?,
                            "json",
                        )?,
                    );
                    checkpoint.artifact_refs.insert(
                        "triage".into(),
                        self.artifacts.put(&serde_json::to_vec(&triage)?, "json")?,
                    );
                    if triage.status == "current"
                        && scanner::exportable_document(&config, &scan, &job.spec.target_path)
                            .is_some()
                    {
                        MaintenanceStatus::NoChange
                    } else {
                        MaintenanceStatus::Author
                    }
                }
            }
            MaintenanceStatus::Author => {
                let reference = checkpoint
                    .artifact_refs
                    .get("scan")
                    .context("scan_checkpoint")?;
                let scan = self.read_scan(&config, repo, reference).await?;
                let draft =
                    guarded_provider_dispatch(&self.store, &config, repo, &scan, &job.spec, || {
                        while_leased(
                            &self.store,
                            &mut lease,
                            self.runtime.lease_ttl_ms,
                            author::propose(&config, repo, &scan, &job.spec.target_path),
                        )
                    })
                    .await?;
                checkpoint.artifact_refs.insert(
                    "draft".into(),
                    self.artifacts.put(&serde_json::to_vec(&draft)?, "json")?,
                );
                MaintenanceStatus::Reviewer
            }
            MaintenanceStatus::Reviewer => {
                if checkpoint.review.is_none() {
                    let scan = self
                        .read_scan(
                            &config,
                            repo,
                            checkpoint
                                .artifact_refs
                                .get("scan")
                                .context("scan_checkpoint")?,
                        )
                        .await?;
                    let draft: author::DraftDocument = serde_json::from_slice(
                        &self.artifacts.read(
                            checkpoint
                                .artifact_refs
                                .get("draft")
                                .context("draft_checkpoint")?,
                        )?,
                    )?;
                    let verified = guarded_provider_dispatch(
                        &self.store,
                        &config,
                        repo,
                        &scan,
                        &job.spec,
                        || {
                            while_leased(
                                &self.store,
                                &mut lease,
                                self.runtime.lease_ttl_ms,
                                author::review_draft(&config, repo, &scan, draft),
                            )
                        },
                    )
                    .await?;
                    let document =
                        author::prepare_document(&config, repo, &scan, &verified).await?;
                    checkpoint.review = Some(MaintenanceReviewProof {
                        reviewer_id: "codex-independent-document-review".into(),
                        author_run_id: verified.author_run_id.clone(),
                        reviewer_run_id: verified.reviewer_run_id.clone(),
                        source_sha: job.spec.source_sha.clone(),
                        document_sha256: digest(document.content.as_bytes()),
                        accepted: verified.review.approved,
                    });
                    checkpoint.artifact_refs.insert(
                        "verified".into(),
                        self.artifacts
                            .put(&serde_json::to_vec(&verified)?, "json")?,
                    );
                    checkpoint.artifact_refs.insert(
                        "document".into(),
                        self.artifacts
                            .put(&serde_json::to_vec(&document)?, "json")?,
                    );
                }
                ensure!(
                    checkpoint.review.as_ref().is_some_and(|r| r.accepted),
                    "independent_review_rejected"
                );
                MaintenanceStatus::Publish
            }
            MaintenanceStatus::Publish => {
                self.publish_and_activate(&config, repo, job, &lease)
                    .await?;
                return Ok(());
            }
            _ => anyhow::bail!("job_stage_unclaimed"),
        };
        self.store
            .transition_maintenance(&lease, next, &checkpoint)
            .await?;
        Ok(())
    }

    async fn current_config(&self) -> Result<(MaintenanceConfig, String)> {
        let mut config = load_maintenance(&self.runtime.maintenance_config)?;
        let serve = brain_serve::Config::load(&self.runtime.serve_config)?;
        let snapshot = self.store.snapshot(&serve.release.id).await?;
        for repo in &config.repositories {
            for target in &repo.doc_targets {
                if let Some(record) = snapshot.revisions.iter().find(|r| {
                    r.source_id == source_id(repo, target)
                        && r.logical_id == *target
                        && !r.tombstone
                }) {
                    record.validate()?;
                    let origin = origin_from_record(record).map_err(anyhow::Error::msg)?;
                    ensure!(record.logical_id == *target, "canonical_document_identity");
                    config.canonical_documents.insert(
                        target.clone(),
                        CoreDocument {
                            logical_id: record.logical_id.clone(),
                            content: record.content.clone(),
                            metadata: record.metadata.clone(),
                            origin,
                        },
                    );
                }
            }
        }
        Ok((config, serve.release.id))
    }

    async fn read_scan(
        &self,
        config: &MaintenanceConfig,
        repo: &RepositoryConfig,
        reference: &str,
    ) -> Result<scanner::ScanResult> {
        crate::config::require_registered(config, repo)?;
        let mut checkpoint: ScanCheckpoint =
            serde_json::from_slice(&self.artifacts.read(reference)?)
                .map_err(|_| anyhow::anyhow!("legacy_raw_scan_blocked"))?;
        checkpoint.hydrate_sources(repo)?;
        let snapshot = self.store.snapshot(&checkpoint.reader_release).await?;
        let docs = dbrain_sources::git_source::PinnedRepository::open(
            &config.docs_repo,
            &checkpoint.scan.docs_sha,
        )?;
        docs.require_origin(&[&config.docs_origin])?;
        for (target, hash) in &checkpoint.document_hashes {
            let text = if hash.is_none() {
                None
            } else if let Some(record) = snapshot.revisions.iter().find(|record| {
                record.source_id == source_id(repo, target)
                    && record.logical_id == *target
                    && !record.tombstone
            }) {
                let origin = origin_from_record(record).map_err(anyhow::Error::msg)?;
                ensure!(
                    origin.identity.source_id == record.source_id
                        && origin.identity.logical_id == *target
                        && origin.raw_sha256 == record.content_hash,
                    "referenced_document_identity_changed"
                );
                Some(record.content.clone())
            } else {
                let path = checkpoint
                    .scan
                    .document_paths
                    .get(target)
                    .context("referenced_document_path")?;
                if target.starts_with("internal/") && config.private_document_root.is_some() {
                    scanner::read_private_document(config, path)?
                } else {
                    Some(String::from_utf8(docs.read_blob(path)?)?)
                }
            };
            ensure!(
                text.as_ref().map(|text| digest(text.as_bytes())) == *hash,
                "referenced_document_hash_changed"
            );
            checkpoint.scan.documents.insert(target.clone(), text);
        }
        author::recheck_source(config, repo, &checkpoint.scan).await?;
        Ok(checkpoint.scan)
    }

    async fn publish_and_activate(
        &self,
        config: &MaintenanceConfig,
        repo: &RepositoryConfig,
        job: &MaintenanceJob,
        lease: &MaintenanceLease,
    ) -> Result<()> {
        let _lock = self.artifacts.publication_lock()?;
        crate::config::require_registered(config, repo)?;
        let (fresh_config, _) = self.current_config().await?;
        let config = &fresh_config;
        let repo = config
            .repositories
            .iter()
            .find(|candidate| registration_id(candidate, &job.spec.target_path) == job.spec.repo_id)
            .context("publication_repository_missing")?;
        ensure!(
            repo.doc_targets.contains(&job.spec.target_path)
                && repo.document_policy_for(&job.spec.target_path) == &job.spec.policy,
            "publication_target_removed_or_policy_changed"
        );
        if job.checkpoint.publication.is_none() {
            let current_before = config
                .canonical_documents
                .get(&job.spec.target_path)
                .map(|doc| digest(doc.content.as_bytes()))
                .unwrap_or_else(|| "missing".into());
            if job.checkpoint.artifact_refs.get("before_document_sha256") != Some(&current_before) {
                let archive = self
                    .artifacts
                    .put(&serde_json::to_vec(&job.checkpoint)?, "json")?;
                self.store.reset_maintenance_inputs(lease, &archive).await?;
                return Ok(());
            }
            if let Some(reference) = job.checkpoint.artifact_refs.get("scan") {
                let scan = self.read_scan(config, repo, reference).await?;
                author::validate_provider_input(config, repo, &scan, &job.spec.target_path).await?;
            }
        }
        let reference = job
            .checkpoint
            .artifact_refs
            .get("document")
            .context("document_checkpoint")?;
        let mut document: CoreDocument = serde_json::from_slice(&self.artifacts.read(reference)?)?;
        ensure!(
            document.logical_id == job.spec.target_path
                && document.origin.policy == job.spec.policy,
            "document_policy_or_target_changed"
        );
        document.origin.identity.source_id = source_id(repo, &document.logical_id);
        if document_is_html(&document.metadata) {
            let projection = dbrain_retrieval::html_projection::project_html(&document.content)?;
            projection.bind_metadata(&mut document.metadata);
        } else {
            document
                .metadata
                .insert("content_format".into(), "markdown".into());
        }
        if job.checkpoint.artifact_refs.contains_key("local_review") {
            self.check_registry_policy(repo, &job.spec.target_path)
                .await?;
        } else {
            self.registration(repo, &job.spec.target_path, &job.spec.source_sha)
                .await?;
        }
        if job.checkpoint.publication.is_none() {
            let serve = brain_serve::Config::load(&self.runtime.serve_config)?;
            let base = self.store.snapshot(&serve.release.id).await?.release;
            let source = DocumentSetSource {
                source_id: document.origin.identity.source_id.clone(),
                configuration: config.prompt_version.clone(),
                visibility: job.spec.policy.visibility,
                allowed_scopes: job.spec.policy.allowed_scopes.clone(),
                tombstone_metadata: BTreeMap::new(),
            };
            let previous = self.store.checkpoint(&source.source_id).await?;
            let expected_record =
                prepare_document_batch(&source, std::slice::from_ref(&document), None)?
                    .records
                    .into_iter()
                    .next()
                    .context("maintenance_expected_record")?;
            let batch = prepare_document_batch(&source, &[document], previous.as_ref())?;
            ensure!(batch.records.len() <= 1, "maintenance_document_delta");
            let existing_revision = if batch.records.is_empty() {
                let state: brain_ingestion::document_set::DocumentSetCheckpoint =
                    serde_json::from_value(batch.checkpoint.state.clone())?;
                Some(
                    state
                        .documents
                        .get(&job.spec.target_path)
                        .context("maintenance_existing_document_state")?
                        .revision,
                )
            } else {
                None
            };
            let revision = existing_revision.unwrap_or_else(|| batch.records[0].revision);
            let mut release = base.clone();
            let already_active = existing_revision.is_some()
                && base
                    .source_revisions
                    .get(&source.source_id)
                    .and_then(|pins| pins.get(&job.spec.target_path))
                    == Some(&revision);
            if !already_active {
                release.release_id = format!("maintenance-{}", &job.spec.id[5..]);
                release.knowledge_version = format!("docs-{}", &job.spec.id[5..]);
                release.created_at_epoch = chrono::Utc::now().timestamp();
                release
                    .source_revisions
                    .entry(source.source_id.clone())
                    .or_default()
                    .insert(job.spec.target_path.clone(), revision);
            }
            let plan = ActivationPlan::prepare(&self.runtime, &self.artifacts, &release)?;
            // Aktivierungsjournal wird als normaler, noch unveröffentlichter Checkpoint persistiert.
            let journal = plan.journal_ref().to_owned();
            self.store
                .set_maintenance_artifact(lease, "activation", &journal)
                .await?;
            crate::config::require_registered(config, repo)?;
            if existing_revision.is_some() {
                self.store
                    .reuse_maintenance_revision(
                        lease,
                        &base.release_id,
                        &release,
                        revision,
                        &expected_record,
                    )
                    .await?;
            } else {
                let source_lease = self
                    .store
                    .claim(&source.source_id, &self.owner, 60000)
                    .await?;
                self.store
                    .commit_maintenance_batches_and_publish_checked(
                        lease,
                        &base.release_id,
                        &[(&batch, &source_lease)],
                        &release,
                        &batch.records,
                    )
                    .await?;
            }
        }
        let current = self
            .store
            .maintenance_job(&job.spec.id)
            .await?
            .context("published_job_missing")?;
        let proof = current
            .checkpoint
            .publication
            .context("publication_proof_missing")?;
        let reference = current
            .checkpoint
            .artifact_refs
            .get("activation")
            .context("activation_checkpoint")?;
        let mut plan =
            ActivationPlan::load(&self.runtime, &self.artifacts, reference, &proof.release_id)?;
        if plan.needs_rebase()? {
            let serve = brain_serve::Config::load(&self.runtime.serve_config)?;
            let mut release = self.store.snapshot(&serve.release.id).await?.release;
            if release
                .source_revisions
                .get(&proof.source_id)
                .and_then(|pins| pins.get(&proof.logical_id))
                .is_some_and(|revision| *revision > proof.document_revision)
            {
                let replacement = self
                    .store
                    .maintenance_jobs(&job.spec.repo_id, 100)
                    .await?
                    .into_iter()
                    .find(|candidate| {
                        candidate
                            .checkpoint
                            .publication
                            .as_ref()
                            .is_some_and(|publication| {
                                publication.source_id == proof.source_id
                                    && publication.logical_id == proof.logical_id
                                    && publication.document_revision > proof.document_revision
                            })
                    });
                if let Some(replacement) = replacement {
                    self.store
                        .supersede_maintenance(lease, &replacement.spec.id)
                        .await?;
                    return Ok(());
                }
                self.store.finish_superseded_publication(lease).await?;
                return Ok(());
            }
            let base_id = release.release_id.clone();
            let key = digest(&serde_json::to_vec(&json!([
                job.spec.id,
                base_id,
                proof.document_revision
            ]))?);
            release.release_id = format!("maintenance-rebase-{key}");
            release.knowledge_version = format!("docs-rebase-{key}");
            release.created_at_epoch = chrono::Utc::now().timestamp();
            release
                .source_revisions
                .entry(proof.source_id.clone())
                .or_default()
                .insert(proof.logical_id.clone(), proof.document_revision);
            plan = ActivationPlan::prepare(&self.runtime, &self.artifacts, &release)?;
            if let Err(error) = self
                .store
                .rebase_maintenance_publication(lease, &base_id, &release, plan.journal_ref())
                .await
            {
                if error
                    .to_string()
                    .contains("rebase would restore superseded target")
                {
                    self.store.finish_superseded_publication(lease).await?;
                    return Ok(());
                }
                return Err(error.into());
            }
        }
        let activate = plan.clone();
        let activated = self
            .store
            .activate_maintenance_checked(
                lease,
                self.runtime.health_timeout_ms * 2,
                |_| async move {
                    activate
                        .activate()
                        .await
                        .map_err(|_| PortError::Unavailable("activation_failed".into()))
                },
                || async move {
                    plan.rollback()
                        .await
                        .map_err(|_| PortError::Unavailable("activation_rollback_failed".into()))
                },
            )
            .await;
        if let Err(error) = activated {
            if error
                .to_string()
                .contains("maintenance publication target superseded")
            {
                self.store.finish_superseded_publication(lease).await?;
                return Ok(());
            }
            return Err(error.into());
        }
        Ok(())
    }

    pub async fn status(&self) -> Result<serde_json::Value> {
        let config = load_maintenance(&self.runtime.maintenance_config)?;
        let mut jobs = Vec::new();
        for repo in &config.repositories {
            for target in &repo.doc_targets {
                for job in self
                    .store
                    .maintenance_jobs(&registration_id(repo, target), 100)
                    .await?
                {
                    let mut usage = json!({"jev":null,"author":null,"reviewer":null});
                    if let Some(reference) = job.checkpoint.artifact_refs.get("triage") {
                        let result: triage::TriageOutcome =
                            serde_json::from_slice(&self.artifacts.read(reference)?)?;
                        usage["jev"] = json!({"input_tokens":result.input_tokens,"output_tokens":result.output_tokens});
                    }
                    if let Some(reference) = job.checkpoint.artifact_refs.get("draft") {
                        let result: author::DraftDocument =
                            serde_json::from_slice(&self.artifacts.read(reference)?)?;
                        usage["author"] = json!({"input_tokens":result.proof.input_tokens,"output_tokens":result.proof.output_tokens});
                    }
                    if let Some(reference) = job.checkpoint.artifact_refs.get("verified") {
                        let result: author::VerifiedDocument =
                            serde_json::from_slice(&self.artifacts.read(reference)?)?;
                        usage["reviewer"] = json!({"input_tokens":result.reviewer_input_tokens,"output_tokens":result.reviewer_output_tokens});
                    }
                    jobs.push(json!({"id":job.spec.id,"repo":repo.id,"target":target,"source_sha":job.spec.source_sha,
                    "deployed_sha":job.spec.deployed_sha,"status":job.status,"attempts":job.attempts,
                    "error_code":job.error_code,"publication":job.checkpoint.publication,"activation":job.checkpoint.activation,"usage":usage}));
                }
            }
        }
        let active = brain_serve::Config::load(&self.runtime.serve_config)?;
        let status = json!({"checked_at":chrono::Utc::now().to_rfc3339(),"active_release":active.release.id,
            "jobs":jobs,"registered_repositories":config.repositories.len(),
            "pending_sources":self.store.maintenance_sources().await?.iter().filter(|r|r.policy.is_none()).count()});
        let parent = self.runtime.status_file.parent().context("status_parent")?;
        std::fs::create_dir_all(parent)?;
        let mut staged = tempfile::NamedTempFile::new_in(parent)?;
        staged.write_all(&serde_json::to_vec_pretty(&status)?)?;
        staged
            .persist(&self.runtime.status_file)
            .map_err(|_| anyhow::anyhow!("status_commit"))?;
        Ok(status)
    }

    pub async fn import_reviewed(&self) -> Result<serde_json::Value> {
        self.require_operator()?;
        self.store.check_maintenance_schema().await?;
        let config = load_maintenance(&self.runtime.maintenance_config)?;
        self.enqueue_local_imports(&config).await?;
        for _ in 0..self.runtime.max_jobs_per_tick * 5 {
            let Some(mut job) = self
                .store
                .claim_maintenance(&self.owner, self.runtime.lease_ttl_ms)
                .await?
            else {
                break;
            };
            let lease = job.lease.clone().context("job_lease")?;
            if !job.checkpoint.artifact_refs.contains_key("local_review")
                && self.local_document(&job).await?.is_none()
            {
                self.store
                    .retry_maintenance(&lease, "LOCAL_IMPORT_ONLY", 1000)
                    .await?;
                break;
            }
            if self.advance(&mut job).await.is_err() {
                self.store
                    .retry_maintenance(&lease, "LOCAL_IMPORT_FAILED", self.runtime.retry_delay_ms)
                    .await?;
            }
        }
        self.status().await
    }

    /// Lokaler Betriebspfad. Die Unix-Identität kommt vom Prozess, niemals aus Eingaben.
    pub async fn query(&self, text: &str) -> Result<serde_json::Value> {
        let uid = self.require_operator()?;
        ensure!(!text.trim().is_empty() && text.len() <= 2048, "query_size");
        let serve = brain_serve::Config::load(&self.runtime.serve_config)?;
        let snapshot = self.store.snapshot(&serve.release.id).await?;
        let principal = brain_contracts::Principal {
            actor_id: format!("unix:{uid}"),
            channel: "local-operator".into(),
            scopes: std::collections::BTreeSet::from(["internal_docs".into()]),
            provider_egress: Default::default(),
        };
        let records = snapshot.authorized(&principal, false)?;
        let words: Vec<_> = text
            .to_lowercase()
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        let mut matches = Vec::new();
        for record in records {
            if !record.source_id.starts_with("maintenance-docs:") {
                continue;
            }
            let registrations = self.store.maintenance_sources().await?;
            let current = registrations
                .iter()
                .find(|r| r.source_id == record.source_id)
                .context("document_registry_missing")?;
            let origin = origin_from_record(&record).map_err(anyhow::Error::msg)?;
            ensure!(
                current.policy.as_ref() == Some(&origin.policy),
                "document_registry_rights_changed"
            );
            let (text, text_sha256) = if document_is_html(&record.metadata) {
                let projection = dbrain_retrieval::html_projection::project_html(&record.content)?;
                (projection.text, projection.semantic_sha256)
            } else {
                (record.content.clone(), digest(record.content.as_bytes()))
            };
            let lower = text.to_lowercase();
            let score = words
                .iter()
                .filter(|word| lower.contains(word.as_str()))
                .count();
            if score > 0 {
                matches.push((
                    score,
                    json!({"logical_id":record.logical_id,"source_sha":origin.source_revision,
                "document_sha256":record.content_hash,"text_sha256":text_sha256,"text":text}),
                ));
            }
        }
        matches.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
        Ok(
            json!({"release_id":snapshot.release.release_id,"actor_id":principal.actor_id,
            "results":matches.into_iter().take(3).map(|(_,value)|value).collect::<Vec<_>>()}),
        )
    }
}
