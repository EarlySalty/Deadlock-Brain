use anyhow::{ensure, Result};
use brain_contracts::{
    source::origin_from_record, CorpusRelease, CorpusSnapshot, SourceVisibility,
};
use brain_maintenance::{
    digest,
    integration::{
        activation::{ActivationPlan, ActivationTarget},
        artifacts::Artifacts,
        config_writer::ConfigWriter,
        runner::require_operator_config,
        runtime_config::{read_bounded, RuntimeConfig},
    },
};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
enum Target {
    Standard,
    SecondBrainInternal,
    EntityProfiles,
    EntityProfilesInternal,
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    config: PathBuf,
    #[arg(long, value_enum)]
    target: Target,
    #[arg(long)]
    base_release_id: String,
    #[arg(long)]
    base_release_sha256: String,
    #[arg(long)]
    candidate_release_id: String,
    #[arg(long)]
    candidate_release_sha256: String,
    #[arg(long)]
    expected_serve_config_sha256: String,
    #[arg(long, required = true, action = clap::ArgAction::Append)]
    allow_source: Vec<String>,
    #[arg(long)]
    apply: bool,
    #[arg(long)]
    maintenance_credential_stdin: bool,
}

#[derive(Serialize)]
struct Binding<'a> {
    version: &'static str,
    target: Target,
    base_release_id: &'a str,
    base_release_sha256: &'a str,
    candidate_release_id: &'a str,
    candidate_release_sha256: &'a str,
    expected_serve_config_sha256: &'a str,
    runtime_config_path: &'a Path,
    runtime_config_sha256: &'a str,
    allowed_sources: &'a BTreeSet<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Prepared {
    request_sha256: String,
    journal_ref: String,
    confirmed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    old_config: String,
    new_config: String,
    release_id: String,
    knowledge_version: String,
    #[serde(default)]
    target: ActivationTarget,
    #[serde(default)]
    old_bindings_sha256: Option<String>,
    #[serde(default)]
    new_bindings_sha256: Option<String>,
}

#[derive(Serialize)]
struct Report {
    target: Target,
    status: &'static str,
    verified_release_bindings_sha256: Option<String>,
    candidate_published: bool,
    activation_performed: bool,
    readiness_verified: bool,
    base_release_id: String,
    base_release_sha256: String,
    candidate_release_id: String,
    candidate_release_sha256: String,
    observed_serve_config_sha256: String,
    journal_ref: Option<String>,
    success: bool,
}

impl Cli {
    fn activation_target(&self) -> ActivationTarget {
        match self.target {
            Target::Standard | Target::EntityProfiles => ActivationTarget::Standard,
            Target::SecondBrainInternal | Target::EntityProfilesInternal => {
                ActivationTarget::SecondBrainInternal
            }
        }
    }

    fn sources(&self) -> Result<BTreeSet<String>> {
        for id in [&self.base_release_id, &self.candidate_release_id] {
            ensure!(safe_id(id), "release_identity");
        }
        for hash in [
            &self.base_release_sha256,
            &self.candidate_release_sha256,
            &self.expected_serve_config_sha256,
        ] {
            ensure!(valid_hash(hash), "release_hash");
        }
        ensure!(
            self.base_release_id != self.candidate_release_id,
            "distinct_releases"
        );
        let sources: BTreeSet<_> = self.allow_source.iter().cloned().collect();
        ensure!(
            !sources.is_empty()
                && sources.len() <= 64
                && sources.len() == self.allow_source.len()
                && sources.iter().all(|source| safe_id(source)),
            "source_allowlist"
        );
        if self.target == Target::SecondBrainInternal {
            ensure!(
                sources.iter().all(|source| internal_feed_source(source)),
                "internal_source_allowlist"
            );
        }
        Ok(sources)
    }

    fn identity(&self, sources: &BTreeSet<String>, runtime_hash: &str) -> Result<String> {
        Ok(digest(&serde_json::to_vec(&Binding {
            version: "brain.candidate-activation.v1",
            target: self.target,
            base_release_id: &self.base_release_id,
            base_release_sha256: &self.base_release_sha256,
            candidate_release_id: &self.candidate_release_id,
            candidate_release_sha256: &self.candidate_release_sha256,
            expected_serve_config_sha256: &self.expected_serve_config_sha256,
            runtime_config_path: &self.config,
            runtime_config_sha256: runtime_hash,
            allowed_sources: sources,
        })?))
    }

    fn report(&self, bytes: &[u8], journal: Option<String>) -> Report {
        Report {
            target: self.target,
            status: "validated",
            verified_release_bindings_sha256: None,
            candidate_published: true,
            activation_performed: false,
            readiness_verified: false,
            base_release_id: self.base_release_id.clone(),
            base_release_sha256: self.base_release_sha256.clone(),
            candidate_release_id: self.candidate_release_id.clone(),
            candidate_release_sha256: self.candidate_release_sha256.clone(),
            observed_serve_config_sha256: digest(bytes),
            journal_ref: journal,
            success: true,
        }
    }
}

fn safe_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 512
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:/-".contains(&b))
}

fn valid_hash(hash: &str) -> bool {
    hash.len() == 64
        && hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn internal_feed_source(source: &str) -> bool {
    source
        .strip_prefix("google-sheet/")
        .or_else(|| source.strip_prefix("youtube-core/"))
        .is_some_and(|id| {
            !id.is_empty()
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
        })
}

fn protected_file(path: &Path, maximum: usize) -> Result<Vec<u8>> {
    require_operator_config(path)?;
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        path.is_absolute()
            && metadata.is_file()
            && metadata.permissions().mode() & 0o077 == 0
            && metadata.nlink() == 1
            && std::fs::canonicalize(path)? == path,
        "operator_config_unprotected"
    );
    read_bounded(path, maximum)
}

fn check_hash(release: &CorpusRelease, id: &str, hash: &str) -> Result<()> {
    brain_storage::validate_release(release)?;
    ensure!(release.release_id == id, "release_identity");
    ensure!(
        digest(&serde_json::to_vec(release)?) == hash,
        "release_hash_mismatch"
    );
    Ok(())
}

fn check_transition(
    base: &CorpusRelease,
    candidate: &CorpusRelease,
    allowed: &BTreeSet<String>,
) -> Result<()> {
    ensure!(candidate.patch == base.patch, "release_patch_changed");
    ensure!(
        candidate.created_at_epoch >= base.created_at_epoch,
        "release_time_regressed"
    );
    ensure!(
        candidate.knowledge_version != base.knowledge_version,
        "knowledge_version_unchanged"
    );
    let sources: BTreeSet<_> = base
        .source_revisions
        .keys()
        .chain(candidate.source_revisions.keys())
        .cloned()
        .collect();
    let changed: BTreeSet<_> = sources
        .iter()
        .filter(|source| {
            base.source_revisions.get(*source) != candidate.source_revisions.get(*source)
        })
        .cloned()
        .collect();
    ensure!(&changed == allowed, "source_allowlist_mismatch");
    for (source, pins) in &base.source_revisions {
        let new = candidate.source_revisions.get(source);
        for (logical, revision) in pins {
            if let Some(proposed) = new.and_then(|pins| pins.get(logical)) {
                ensure!(proposed >= revision, "document_pin_regressed");
            } else {
                ensure!(allowed.contains(source), "document_pin_removed");
            }
        }
    }
    Ok(())
}

fn check_rights(
    base: &CorpusSnapshot,
    candidate: &CorpusSnapshot,
    allowed: &BTreeSet<String>,
    target: Target,
) -> Result<()> {
    let internal_scopes = BTreeSet::from(["second_brain.internal".into()]);
    let principal = brain_contracts::Principal {
        actor_id: "candidate-activation".into(),
        channel: "local-operator".into(),
        scopes: match target {
            Target::Standard => BTreeSet::new(),
            Target::SecondBrainInternal
            | Target::EntityProfiles
            | Target::EntityProfilesInternal => internal_scopes.clone(),
        },
        provider_egress: BTreeSet::new(),
    };
    base.authorized(&principal, false)?;
    candidate.authorized(&principal, false)?;
    let previous: BTreeMap<_, _> = base
        .revisions
        .iter()
        .map(|record| ((&record.source_id, &record.logical_id), record))
        .collect();
    let heads: BTreeMap<_, _> = candidate
        .heads
        .iter()
        .map(|record| ((&record.source_id, &record.logical_id), record))
        .collect();
    let base_heads: BTreeMap<_, _> = base
        .heads
        .iter()
        .map(|record| ((&record.source_id, &record.logical_id), record))
        .collect();
    for old in &base.revisions {
        if candidate
            .release
            .source_revisions
            .get(&old.source_id)
            .is_some_and(|pins| pins.contains_key(&old.logical_id))
        {
            continue;
        }
        let head = base_heads
            .get(&(&old.source_id, &old.logical_id))
            .ok_or_else(|| anyhow::anyhow!("removed_head_missing"))?;
        ensure!(
            allowed.contains(&old.source_id) && head.tombstone && head.revision > old.revision,
            "removal_without_tombstone"
        );
        let old_origin =
            origin_from_record(old).map_err(|_| anyhow::anyhow!("base_rights_missing"))?;
        let head_origin =
            origin_from_record(head).map_err(|_| anyhow::anyhow!("tombstone_rights_missing"))?;
        ensure!(
            old_origin.policy == head_origin.policy
                && old.metadata.get("egress") == head.metadata.get("egress"),
            "tombstone_policy_changed"
        );
    }
    for record in &candidate.revisions {
        if !allowed.contains(&record.source_id) {
            continue;
        }
        let old = previous.get(&(&record.source_id, &record.logical_id));
        if old.is_some_and(|old| old.revision == record.revision) {
            continue;
        }
        let head = heads
            .get(&(&record.source_id, &record.logical_id))
            .ok_or_else(|| anyhow::anyhow!("source_head_missing"))?;
        ensure!(
            **head == *record && !record.tombstone,
            "candidate_superseded"
        );
        let origin =
            origin_from_record(record).map_err(|_| anyhow::anyhow!("source_rights_missing"))?;
        ensure!(
            matches!(&origin.policy.authorization_ref,
            brain_contracts::value::Observed::Known { value } if !value.trim().is_empty()),
            "source_authorization_missing"
        );
        match target {
            Target::Standard => ensure!(
                record.visibility == SourceVisibility::Public && origin.policy.publication_allowed,
                "standard_requires_public_source"
            ),
            Target::SecondBrainInternal => ensure!(
                internal_feed_source(&record.source_id)
                    && record.visibility == SourceVisibility::Internal
                    && record.allowed_scopes == internal_scopes
                    && !origin.policy.publication_allowed
                    && !origin.policy.provider_egress_allowed
                    && origin.policy.raw_retention_allowed
                    && matches!(
                        origin.policy.license,
                        brain_contracts::value::Observed::Unknown {
                            reason: brain_contracts::value::UnknownReason::NotPresent
                        }
                    ),
                "internal_source_rights"
            ),
            Target::EntityProfiles | Target::EntityProfilesInternal => {
                if record.source_id == "git-game-facts-derived" {
                    ensure!(
                        origin.policy
                            == brain_storage::entity_profile::derivation::derived_policy(),
                        "entity_profile_document_rights"
                    );
                } else {
                    ensure!(
                        !internal_feed_source(&record.source_id)
                            && record.visibility == SourceVisibility::Internal
                            && !origin.policy.publication_allowed
                            && !origin.policy.provider_egress_allowed
                            && origin.policy.raw_retention_allowed
                            && origin.parser_family == "dbrain-sources/wiki-spielwissen",
                        "entity_profile_source_rights"
                    );
                }
            }
        }
        if let Some(old) = old {
            let old_origin =
                origin_from_record(old).map_err(|_| anyhow::anyhow!("base_rights_missing"))?;
            ensure!(
                origin.policy == old_origin.policy
                    && record.metadata.get("egress") == old.metadata.get("egress"),
                "source_policy_changed"
            );
        }
    }
    Ok(())
}

async fn check_entity_profile_sources(
    runtime: &RuntimeConfig,
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    base: &CorpusSnapshot,
    candidate: &CorpusSnapshot,
    allowed: &BTreeSet<String>,
) -> Result<()> {
    use dbrain_sources::{knowledge_contract, knowledge_import};
    let config =
        brain_maintenance::integration::runner::load_maintenance(&runtime.maintenance_config)?;
    let principal = brain_maintenance::integration::runner::local_operator_principal(
        require_operator_config(&runtime.maintenance_config)?,
        &runtime.maintenance_config,
    )?;
    let visible = candidate.authorized(&principal, false)?;
    for id in allowed {
        if id == "git-game-facts-derived" {
            let records: Vec<_> = candidate
                .revisions
                .iter()
                .filter(|record| record.source_id == *id)
                .collect();
            if records.is_empty() {
                brain_maintenance::integration::entity_profiles::verify_retired_git_documents(
                    store, pool, &config, &principal, candidate,
                )
                .await?;
            }
            for record in records {
                ensure!(
                    visible.contains(record),
                    "entity_profile_document_inaccessible"
                );
                brain_maintenance::integration::entity_profiles::verify_stored_git_document(
                    store,
                    pool,
                    &config,
                    &principal,
                    record,
                    &candidate.release,
                )
                .await?;
            }
            continue;
        }
        if base.release.source_revisions.contains_key(id) {
            let records: Vec<_> = candidate
                .revisions
                .iter()
                .filter(|record| record.source_id == *id)
                .collect();
            let mut wiki = !records.is_empty();
            for record in &records {
                let encoded = record
                    .metadata
                    .get(brain_storage::source_versions::DOCUMENT_METADATA_KEY)
                    .ok_or_else(|| anyhow::anyhow!("entity_profile_document_missing"))?;
                let document: knowledge_contract::KnowledgeDocument =
                    serde_json::from_str(encoded)?;
                wiki &= document.source_kind == knowledge_contract::KnowledgeSourceKind::Wiki;
            }
            if wiki {
                for record in records {
                    ensure!(
                        visible.contains(record),
                        "entity_profile_source_inaccessible"
                    );
                    dbrain_retrieval::knowledge_projection::project_knowledge(record)?
                        .ok_or_else(|| anyhow::anyhow!("entity_profile_document_missing"))?;
                }
                continue;
            }
        }
        let matching: Vec<_> = runtime
            .entity_profile_sources
            .iter()
            .filter(|source| source.extraction.source_id == *id)
            .collect();
        ensure!(matching.len() == 1, "entity_profile_source_registration");
        let source = matching[0];
        let repo = config
            .game_sources
            .iter()
            .find(|repo| repo.id == source.repository_id)
            .ok_or_else(|| anyhow::anyhow!("entity_profile_repository"))?;
        brain_maintenance::config::require_registered_game_source(&config, repo)?;
        let sha =
            brain_maintenance::scanner::resolve_ref(&repo.path, &repo.source_ref, &config.bounds)
                .await?;
        let pinned = dbrain_sources::git_source::PinnedRepository::open(&repo.path, &sha)?;
        pinned.require_origin(&[&repo.origin])?;
        let policy: knowledge_import::ImportPolicy =
            serde_json::from_slice(&protected_file(&source.import_policy, 256 * 1024)?)?;
        let records: Vec<_> = candidate
            .revisions
            .iter()
            .filter(|record| record.source_id == *id)
            .collect();
        ensure!(!records.is_empty(), "entity_profile_source_empty");
        for record in records {
            ensure!(
                visible.contains(record),
                "entity_profile_source_inaccessible"
            );
            dbrain_retrieval::knowledge_projection::project_knowledge(record)?
                .ok_or_else(|| anyhow::anyhow!("entity_profile_document_missing"))?;
            let encoded = record
                .metadata
                .get(brain_storage::source_versions::DOCUMENT_METADATA_KEY)
                .ok_or_else(|| anyhow::anyhow!("entity_profile_document_missing"))?;
            let document: knowledge_contract::KnowledgeDocument = serde_json::from_str(encoded)?;
            ensure!(
                document.source_kind == knowledge_contract::KnowledgeSourceKind::GameFile
                    && document
                        .metadata
                        .get("source_revision")
                        .and_then(serde_json::Value::as_str)
                        == Some(sha.as_str()),
                "entity_profile_git_revision"
            );
            let path = document
                .metadata
                .get("original_relative_path")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| anyhow::anyhow!("entity_profile_original_path"))?;
            brain_maintenance::config::safe_relative(path)?;
            for scopes in [&source.paths, &repo.source_paths.iter().cloned().collect()] {
                ensure!(
                    scopes
                        .iter()
                        .any(|scope| path == scope || path.starts_with(&format!("{scope}/"))),
                    "entity_profile_source_scope"
                );
            }
            ensure!(
                document
                    .metadata
                    .get("original_sha256")
                    .and_then(serde_json::Value::as_str)
                    == Some(digest(&pinned.read_blob(path)?).as_str()),
                "entity_profile_git_blob"
            );
            let origin = origin_from_record(record)
                .map_err(|_| anyhow::anyhow!("entity_profile_original_origin"))?;
            let prepared = knowledge_import::prepare_knowledge_jsonl(
                std::io::Cursor::new(encoded.as_bytes()),
                &policy,
                &origin.parser_revision,
            )?;
            ensure!(
                prepared.records().len() == 1,
                "entity_profile_import_rights"
            );
            let expected = origin_from_record(&prepared.records()[0].record)
                .map_err(|_| anyhow::anyhow!("entity_profile_import_origin"))?;
            ensure!(expected == origin, "entity_profile_original_policy_changed");
        }
    }
    Ok(())
}

fn check_basis(
    bytes: &[u8],
    base: &CorpusRelease,
    expected_hash: &str,
    target: ActivationTarget,
) -> Result<()> {
    ensure!(digest(bytes) == expected_hash, "stale_serve_config");
    let serve =
        brain_serve::Config::parse(bytes).map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    ensure!(serve.bind.ip().is_loopback(), "serve_loopback_required");
    let pin = target.release(&serve)?;
    ensure!(
        pin.id == base.release_id && pin.knowledge_version == base.knowledge_version,
        "stale_base_release"
    );
    Ok(())
}

fn check_config_transition(
    old: &[u8],
    new: &[u8],
    candidate: &CorpusRelease,
    target: ActivationTarget,
) -> Result<()> {
    let mut expected: serde_json::Value = serde_json::from_slice(old)?;
    let pin = serde_json::json!({"id":candidate.release_id,"knowledge_version":candidate.knowledge_version});
    let config =
        brain_serve::Config::parse(old).map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    target.replace_pin(&mut expected, &config, pin)?;
    ensure!(
        expected == serde_json::from_slice::<serde_json::Value>(new)?,
        "journal_unrelated_config"
    );
    Ok(())
}

fn journal_bytes(
    artifacts: &Artifacts,
    prepared: &Prepared,
    cli: &Cli,
    base: &CorpusRelease,
    candidate: &CorpusRelease,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let journal: Journal = serde_json::from_slice(&artifacts.read(&prepared.journal_ref)?)?;
    ensure!(
        journal.release_id == candidate.release_id
            && journal.knowledge_version == candidate.knowledge_version,
        "journal_release_mismatch"
    );
    let old = artifacts.read(&journal.old_config)?;
    let new = artifacts.read(&journal.new_config)?;
    ensure!(
        journal.target == cli.activation_target(),
        "journal_target_mismatch"
    );
    check_basis(
        &old,
        base,
        &cli.expected_serve_config_sha256,
        cli.activation_target(),
    )?;
    check_config_transition(&old, &new, candidate, cli.activation_target())?;
    let old_config =
        brain_serve::Config::parse(&old).map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    let new_config =
        brain_serve::Config::parse(&new).map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    match (&journal.old_bindings_sha256, &journal.new_bindings_sha256) {
        (Some(old), Some(new)) => ensure!(
            *old == old_config.release_bindings_sha256()
                && *new == new_config.release_bindings_sha256(),
            "journal_bindings_mismatch"
        ),
        (None, None) => ensure!(
            journal.target == ActivationTarget::Standard,
            "legacy_internal_journal"
        ),
        _ => anyhow::bail!("journal_bindings_mismatch"),
    }
    Ok((old, new))
}

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Base,
    Candidate,
}

fn position(current: &[u8], old: &[u8], new: &[u8]) -> Result<Position> {
    if current == old {
        Ok(Position::Base)
    } else if current == new {
        Ok(Position::Candidate)
    } else {
        anyhow::bail!("stale_activation_journal")
    }
}

async fn run(cli: &Cli) -> Result<Report> {
    let allowed = cli.sources()?;
    if cli.maintenance_credential_stdin {
        ensure!(
            matches!(
                cli.target,
                Target::EntityProfiles | Target::EntityProfilesInternal
            ),
            "maintenance_credential_target"
        );
        credential_stdin_to_fd5()?;
    }
    let runtime_bytes = protected_file(&cli.config, 65536)?;
    let runtime = RuntimeConfig::load(&cli.config)?;
    ensure!(
        protected_file(&cli.config, 65536)? == runtime_bytes,
        "runtime_config_changed"
    );
    let identity = cli.identity(&allowed, &digest(&runtime_bytes))?;
    protected_file(&runtime.maintenance_config, 256 * 1024)?;
    let infisical_bytes = protected_file(&runtime.infisical_config, 65536)?;
    let infisical: serde_json::Value = serde_json::from_slice(&infisical_bytes)?;
    ensure!(
        infisical["credential_fd"] == 5
            && infisical
                .get("credential_path")
                .is_none_or(serde_json::Value::is_null)
            && infisical
                .get("secret_values_fd")
                .is_none_or(serde_json::Value::is_null),
        "infisical_fd5_required"
    );
    let pg = &runtime.postgres;
    ensure!(
        pg.socket_dir == Path::new("/run/deadlock-brain-postgresql")
            && pg.port == 5446
            && pg.database == "brain"
            && pg.username == "brain_ingest"
            && pg.auth == brain_serve::config::DatabaseAuth::Password,
        "brain_ingest_endpoint_required"
    );
    let secrets: BTreeMap<_, _> = dl_token_secrets::values_from_config(&infisical_bytes)
        .await
        .map_err(|_| anyhow::anyhow!("secret_source_unavailable"))?
        .into_iter()
        .collect();
    let password = secrets
        .get(
            pg.password_env
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("database_secret_name"))?,
        )
        .ok_or_else(|| anyhow::anyhow!("database_secret_missing"))?;
    let options = PgConnectOptions::new_without_pgpass()
        .host("/run/deadlock-brain-postgresql")
        .port(pg.port)
        .username(&pg.username)
        .database(&pg.database)
        .password(password)
        .ssl_mode(PgSslMode::Disable)
        .options([("statement_timeout", "120000"), ("lock_timeout", "10000")]);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(30))
        .connect_with(options)
        .await?;
    drop(secrets);
    let role_ok: bool = sqlx::query_scalar(
        "SELECT current_user='brain_ingest' AND current_database()='brain' AND NOT r.rolsuper AND NOT r.rolbypassrls AND NOT r.rolcreatedb AND NOT r.rolcreaterole AND has_schema_privilege('brain','USAGE') AND NOT has_schema_privilege('brain','CREATE') AND NOT has_table_privilege('brain.corpus_releases_v1','UPDATE,DELETE') AND NOT has_table_privilege('brain.source_record_revisions','UPDATE,DELETE') FROM pg_roles r WHERE r.rolname=current_user",
    ).fetch_one(&pool).await?;
    ensure!(role_ok, "database_role_unsafe");
    let store = brain_storage::PgStore::new(pool.clone());
    store.check_core_schema().await?;
    let metadata = std::fs::symlink_metadata(&runtime.artifact_dir)?;
    ensure!(
        metadata.uid() == nix::unistd::geteuid().as_raw(),
        "artifact_owner"
    );
    let artifacts = Artifacts::open(&runtime.artifact_dir)?;
    let lock_artifacts = artifacts.clone();
    let _publication_lock =
        tokio::task::spawn_blocking(move || lock_artifacts.publication_lock()).await??;
    let writer = ConfigWriter::lock_async(runtime.serve_config.clone()).await?;
    let current = writer.read()?;
    let serve = brain_serve::Config::parse(&current)
        .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    ensure!(
        serve.bind.ip().is_loopback()
            && serve.postgres.socket_dir == pg.socket_dir
            && serve.postgres.port == pg.port
            && serve.postgres.database == pg.database
            && ["brain_service", "brain_readonly"].contains(&serve.postgres.username.as_str()),
        "serve_brain_reader_endpoint_required"
    );
    let mut guard = pool.begin().await?;
    if cli.apply {
        let guard_base = store.snapshot(&cli.base_release_id).await?;
        let guard_candidate = store.snapshot(&cli.candidate_release_id).await?;
        let mut pins = guard_base.release.source_revisions;
        for (source, documents) in guard_candidate.release.source_revisions {
            pins.entry(source).or_default().extend(documents);
        }
        sqlx::query("SELECT h.source_id FROM brain.source_record_heads h JOIN jsonb_each($1::jsonb) s ON h.source_id=s.key CROSS JOIN LATERAL jsonb_each_text(s.value) d WHERE h.logical_id=d.key FOR SHARE OF h")
            .bind(serde_json::to_value(&pins)?)
            .fetch_all(&mut *guard).await?;
    }
    let base = store.snapshot(&cli.base_release_id).await?;
    let candidate = store.snapshot(&cli.candidate_release_id).await?;
    check_hash(
        &base.release,
        &cli.base_release_id,
        &cli.base_release_sha256,
    )?;
    check_hash(
        &candidate.release,
        &cli.candidate_release_id,
        &cli.candidate_release_sha256,
    )?;
    check_transition(&base.release, &candidate.release, &allowed)?;
    check_rights(&base, &candidate, &allowed, cli.target)?;
    if matches!(
        cli.target,
        Target::EntityProfiles | Target::EntityProfilesInternal
    ) {
        check_entity_profile_sources(&runtime, &store, &pool, &base, &candidate, &allowed).await?;
    }
    let prior = artifacts.call_receipt(&identity)?;
    let mut prepared = if let Some(bytes) = prior {
        let prepared: Prepared = serde_json::from_slice(&bytes)?;
        ensure!(
            prepared.request_sha256 == identity,
            "journal_request_mismatch"
        );
        Some(prepared)
    } else {
        check_basis(
            &current,
            &base.release,
            &cli.expected_serve_config_sha256,
            cli.activation_target(),
        )?;
        None
    };
    if !cli.apply {
        if let Some(prepared) = &prepared {
            let (old, new) =
                journal_bytes(&artifacts, prepared, cli, &base.release, &candidate.release)?;
            position(&current, &old, &new)?;
        }
        guard.rollback().await?;
        return Ok(cli.report(&current, prepared.map(|p| p.journal_ref)));
    }
    if prepared.is_none() {
        let plan = ActivationPlan::prepare_target(
            &runtime,
            &artifacts,
            &candidate.release,
            &writer,
            cli.activation_target(),
        )?;
        let intent = Prepared {
            request_sha256: identity.clone(),
            journal_ref: plan.journal_ref().to_owned(),
            confirmed: false,
        };
        artifacts.save_call_receipt(&identity, &serde_json::to_vec(&intent)?)?;
        prepared = Some(intent);
    }
    let mut prepared = prepared.ok_or_else(|| anyhow::anyhow!("activation_intent_missing"))?;
    let (old, new) = journal_bytes(
        &artifacts,
        &prepared,
        cli,
        &base.release,
        &candidate.release,
    )?;
    let initial_position = position(&current, &old, &new)?;
    ensure!(
        !prepared.confirmed || initial_position == Position::Candidate,
        "confirmed_activation_changed"
    );
    let mut report = cli.report(&current, Some(prepared.journal_ref.clone()));
    let plan = if prepared.confirmed {
        ActivationPlan::prepare_target(
            &runtime,
            &artifacts,
            &candidate.release,
            &writer,
            cli.activation_target(),
        )?
    } else {
        ActivationPlan::load(
            &runtime,
            &artifacts,
            &prepared.journal_ref,
            &candidate.release.release_id,
        )?
    };
    ensure!(
        plan.target() == cli.activation_target(),
        "activation_target_mismatch"
    );
    ensure!(!plan.needs_rebase(&writer)?, "stale_activation_journal");
    match plan.activate(&writer).await {
        Ok(_) => {
            report.activation_performed = !prepared.confirmed;
            report.status = "active_candidate";
            report.readiness_verified = true;
            report.observed_serve_config_sha256 = digest(&new);
            prepared.confirmed = true;
            if artifacts
                .save_call_validation(&identity, &serde_json::to_vec(&prepared)?)
                .is_err()
            {
                report.status = "active_candidate_receipt_failed";
                report.success = false;
            }
        }
        Err(_) if prepared.confirmed => {
            report.status = "blocked_candidate_readiness_unknown";
            report.success = false;
        }
        Err(_) => {
            report.success = false;
            match writer.read().and_then(|bytes| position(&bytes, &old, &new)) {
                Ok(_) => match plan.rollback(&writer).await {
                    Ok(()) => {
                        report.status = "rolled_back_base";
                        report.readiness_verified = true;
                        report.observed_serve_config_sha256 = digest(&old);
                    }
                    Err(_) => report.status = "blocked_rollback_failed",
                },
                Err(_) => report.status = "blocked_config_changed",
            }
        }
    }
    if let Ok(bytes) = writer.read() {
        report.observed_serve_config_sha256 = digest(&bytes);
        let expected = if report.status.starts_with("active_candidate") {
            Some(new.as_slice())
        } else if report.status == "rolled_back_base" {
            Some(old.as_slice())
        } else {
            None
        };
        if expected.is_some_and(|expected| expected != bytes.as_slice()) {
            report.status = "blocked_config_changed";
            report.success = false;
            report.readiness_verified = false;
        }
    } else {
        report.status = "blocked_config_unreadable";
        report.success = false;
        report.readiness_verified = false;
    }
    if report.readiness_verified {
        let verified = if report.status == "rolled_back_base" {
            &old
        } else {
            &new
        };
        report.verified_release_bindings_sha256 = Some(
            brain_serve::Config::parse(verified)
                .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?
                .release_bindings_sha256(),
        );
    }
    if guard.rollback().await.is_err() {
        report.success = false;
        if report.status == "active_candidate" {
            report.status = "active_candidate_guard_release_failed";
        }
    }
    Ok(report)
}

fn credential_stdin_to_fd5() -> Result<()> {
    ensure!(
        std::fs::metadata("/proc/self/fd/0")?.is_file(),
        "credential_regular_file"
    );
    nix::unistd::dup2(0, 5)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let report = match run(&cli).await {
        Ok(report) => report,
        Err(_) => {
            eprintln!("Kandidatenaktivierung gesperrt: Prüfung fehlgeschlagen. Kein Aktivierungsnachweis.");
            std::process::exit(1);
        }
    };
    let success = report.success;
    let written = serde_json::to_vec(&report).ok().is_some_and(|mut bytes| {
        bytes.push(b'\n');
        std::io::stdout().lock().write_all(&bytes).is_ok()
    });
    if !written || !success {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ScratchPg {
        directory: tempfile::TempDir,
    }

    impl ScratchPg {
        fn start() -> Self {
            let instance = Self {
                directory: tempfile::tempdir().unwrap(),
            };
            let data = instance.directory.path().join("data");
            let socket = instance.directory.path().join("socket");
            std::fs::create_dir(&socket).unwrap();
            assert!(
                std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
                    .arg("-D")
                    .arg(&data)
                    .args(["-A", "trust", "-U", "brain_core_test", "--no-locale"])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .unwrap()
                    .success()
            );
            let options = format!("-k {} -p 55442 -c listen_addresses=''", socket.display());
            assert!(
                std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                    .arg("-D")
                    .arg(data)
                    .arg("-l")
                    .arg(instance.directory.path().join("postgres.log"))
                    .args(["-o", &options, "-w", "start"])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .unwrap()
                    .success()
            );
            instance
        }
    }

    impl Drop for ScratchPg {
        fn drop(&mut self) {
            let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                .arg("-D")
                .arg(self.directory.path().join("data"))
                .args(["-m", "immediate", "-w", "stop"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }

    #[test]
    #[ignore = "Wird von der FD-Prozessregression mit einem regulären Test-stdin gestartet"]
    fn credential_transfer_child() {
        credential_stdin_to_fd5().unwrap();
        let bytes = std::fs::read("/proc/self/fd/5").unwrap();
        assert_eq!(digest(&bytes), digest(b"FD-Pruefwert"));
    }

    #[tokio::test]
    async fn credential_stdin_reaches_child_fd5_without_changing_parent_cloexec() {
        use std::os::fd::AsRawFd;
        let dir = tempfile::tempdir().unwrap();
        let credential = tempfile::tempfile().unwrap();
        (&credential).write_all(b"FD-Pruefwert").unwrap();
        let fd = credential.as_raw_fd();
        let before = nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFD).unwrap();
        assert!(
            nix::fcntl::FdFlag::from_bits_retain(before).contains(nix::fcntl::FdFlag::FD_CLOEXEC)
        );
        let output = brain_maintenance::process::run_with_credential(
            &std::env::current_exe().unwrap(),
            &[
                "--exact".into(),
                "tests::credential_transfer_child".into(),
                "--ignored".into(),
                "--nocapture".into(),
            ],
            dir.path(),
            credential.try_clone().unwrap(),
            5000,
            4096,
        )
        .await
        .unwrap();
        assert!(String::from_utf8(output).unwrap().contains("1 passed"));
        assert_eq!(
            nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFD).unwrap(),
            before
        );
    }

    fn release(id: &str, version: &str, own_revision: u64) -> CorpusRelease {
        CorpusRelease {
            release_id: id.into(),
            knowledge_version: version.into(),
            patch: "2026-10-01".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([
                (
                    "patchnotes".into(),
                    BTreeMap::from([("patch".into(), own_revision)]),
                ),
                ("foreign".into(), BTreeMap::from([("document".into(), 7)])),
            ]),
        }
    }

    fn serve_value() -> serde_json::Value {
        let mut value: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../config/brain-serve.example.json"
        ))
        .unwrap();
        value["credentials"][0] = serde_json::json!({
            "token_env": "BRAIN_SERVE_API_TOKEN",
            "actor_id": "pilot-client",
            "channel": "pilot",
            "scopes": [],
            "provider_egress": ["public"]
        });
        value
    }

    fn c9_value() -> serde_json::Value {
        let mut value = serve_value();
        value["release"] = serde_json::json!({"id":"base","knowledge_version":"base-kv"});
        value["internal_operator"] = serde_json::json!({
            "socket":"/run/user/1000/brain-test/operator.sock",
            "release":{"id":"internal-base","knowledge_version":"internal-base-kv"}
        });
        value["credentials"].as_array_mut().unwrap().extend([
            serde_json::json!({"token_env":"DOCS_TEST_SECRET","actor_id":"docs-client","channel":"docs","scopes":["docs.public"],"provider_egress":["public"],"release":{"id":"docs","knowledge_version":"docs-kv"}}),
            serde_json::json!({"token_env":"INTERNAL_TEST_SECRET","actor_id":"second-brain","channel":"internal","scopes":["second_brain.internal"],"provider_egress":[],"release":{"id":"internal-base","knowledge_version":"internal-base-kv"}}),
        ]);
        value
    }

    fn allowed() -> BTreeSet<String> {
        BTreeSet::from(["patchnotes".into()])
    }

    #[test]
    fn transition_preserves_foreign_pins_and_accepts_exact_change() {
        let base = release("base", "base-kv", 1);
        let candidate = release("candidate", "candidate-kv", 2);
        check_transition(&base, &candidate, &allowed()).unwrap();
    }

    #[test]
    fn transition_rejects_foreign_changes_removals_and_regressions() {
        let base = release("base", "base-kv", 2);
        for variant in 0..4 {
            let mut candidate = release("candidate", "candidate-kv", 3);
            match variant {
                0 => {
                    candidate
                        .source_revisions
                        .get_mut("foreign")
                        .unwrap()
                        .insert("document".into(), 8);
                }
                1 => {
                    candidate.source_revisions.remove("foreign");
                }
                2 => {
                    candidate
                        .source_revisions
                        .get_mut("foreign")
                        .unwrap()
                        .clear();
                }
                _ => {
                    candidate
                        .source_revisions
                        .get_mut("patchnotes")
                        .unwrap()
                        .insert("patch".into(), 1);
                }
            }
            assert!(check_transition(&base, &candidate, &allowed()).is_err());
        }
    }

    #[test]
    fn transition_rejects_extra_allowlist_and_release_metadata_changes() {
        let base = release("base", "base-kv", 1);
        let mut candidate = release("candidate", "candidate-kv", 2);
        assert!(check_transition(
            &base,
            &candidate,
            &BTreeSet::from(["patchnotes".into(), "foreign".into()])
        )
        .is_err());
        candidate.patch = "other".into();
        assert!(check_transition(&base, &candidate, &allowed()).is_err());
        candidate.patch = base.patch.clone();
        candidate.knowledge_version = base.knowledge_version.clone();
        assert!(check_transition(&base, &candidate, &allowed()).is_err());
    }

    #[test]
    fn release_hash_is_bound_to_exact_serialized_contract() {
        let release = release("base", "base-kv", 1);
        let hash = digest(&serde_json::to_vec(&release).unwrap());
        check_hash(&release, "base", &hash).unwrap();
        assert!(check_hash(&release, "other", &hash).is_err());
        assert!(check_hash(&release, "base", &"0".repeat(64)).is_err());
        let mut changed = release.clone();
        changed.created_at_epoch += 1;
        assert!(check_hash(&changed, "base", &hash).is_err());
    }

    #[test]
    fn replay_never_accepts_or_rolls_back_newer_configuration() {
        assert_eq!(
            position(b"base", b"base", b"candidate").unwrap(),
            Position::Base
        );
        assert_eq!(
            position(b"candidate", b"base", b"candidate").unwrap(),
            Position::Candidate
        );
        assert!(position(b"newer", b"base", b"candidate").is_err());
    }

    #[test]
    fn cli_requires_explicit_binding_and_refuses_internal_target() {
        let cli = Cli::try_parse_from([
            "brain-candidate-activate",
            "--config",
            "/absolute/runtime.json",
            "--target",
            "second-brain-internal",
            "--base-release-id",
            "base",
            "--base-release-sha256",
            &"a".repeat(64),
            "--candidate-release-id",
            "candidate",
            "--candidate-release-sha256",
            &"b".repeat(64),
            "--expected-serve-config-sha256",
            &"c".repeat(64),
            "--allow-source",
            "patchnotes",
        ])
        .unwrap();
        assert!(cli.sources().is_err());
        assert!(Cli::try_parse_from([
            "brain-candidate-activate",
            "--config",
            "/absolute/runtime.json"
        ])
        .is_err());
    }

    #[test]
    fn independent_consumer_and_operator_pins_cannot_change() {
        let old = c9_value();
        let mut new = old.clone();
        new["release"] = serde_json::json!({"id":"candidate","knowledge_version":"candidate-kv"});
        let candidate = release("candidate", "candidate-kv", 2);
        check_config_transition(
            &serde_json::to_vec(&old).unwrap(),
            &serde_json::to_vec(&new).unwrap(),
            &candidate,
            ActivationTarget::Standard,
        )
        .unwrap();
        new["credentials"][2]["release"]["id"] = serde_json::json!("changed");
        assert!(check_config_transition(
            &serde_json::to_vec(&old).unwrap(),
            &serde_json::to_vec(&new).unwrap(),
            &candidate,
            ActivationTarget::Standard
        )
        .is_err());
        new["credentials"] = old["credentials"].clone();
        new["internal_operator"]["release"]["id"] = serde_json::json!("changed");
        assert!(check_config_transition(
            &serde_json::to_vec(&old).unwrap(),
            &serde_json::to_vec(&new).unwrap(),
            &candidate,
            ActivationTarget::Standard
        )
        .is_err());
    }

    #[test]
    fn stale_basis_rejects_hash_release_and_knowledge_mismatches() {
        let base = release("base", "base-kv", 1);
        let mut value = serve_value();
        value["release"] =
            serde_json::json!({"id":base.release_id,"knowledge_version":base.knowledge_version});
        let bytes = serde_json::to_vec(&value).unwrap();
        check_basis(&bytes, &base, &digest(&bytes), ActivationTarget::Standard).unwrap();
        assert!(check_basis(&bytes, &base, &"0".repeat(64), ActivationTarget::Standard).is_err());
        for field in ["id", "knowledge_version"] {
            let mut changed = value.clone();
            changed["release"][field] = serde_json::json!("newer");
            let changed = serde_json::to_vec(&changed).unwrap();
            assert!(check_basis(
                &changed,
                &base,
                &digest(&changed),
                ActivationTarget::Standard
            )
            .is_err());
        }
    }

    fn record(revision: u64, visibility: SourceVisibility) -> brain_contracts::SourceRecordV2 {
        use brain_contracts::{source::*, value::*};
        let mut record = brain_contracts::SourceRecordV2 {
            source_id: "patchnotes".into(),
            logical_id: "patch".into(),
            revision,
            content_hash: digest(b"public patch"),
            content: "public patch".into(),
            visibility,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        let unknown = || Observed::unknown(UnknownReason::NotPresent);
        OriginArtifact {
            identity: SourceIdentity {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
            },
            source_revision: SourceRevision::Http {
                body_sha256: record.content_hash.clone(),
                etag: None,
                last_modified: None,
            },
            raw_sha256: record.content_hash.clone(),
            locator: "https://example.invalid/patch".into(),
            parser_revision: "1".into(),
            parser_family: "patchnotes".into(),
            schema_version: unknown(),
            schema_sha256: unknown(),
            retrieved_at: Observed::unknown(UnknownReason::NotPresent),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: unknown(),
            origin_artifacts: BTreeSet::new(),
            derivation_family: unknown(),
            policy: SourcePolicy {
                visibility,
                allowed_scopes: BTreeSet::new(),
                authorization_ref: Observed::known("explicit-test".into()),
                license: unknown(),
                publication_allowed: true,
                provider_egress_allowed: false,
                raw_retention_allowed: false,
            },
            validity: GameValidity::unknown(),
        }
        .bind_record(&mut record)
        .unwrap();
        record
    }

    fn snapshot(record: brain_contracts::SourceRecordV2, id: &str) -> CorpusSnapshot {
        CorpusSnapshot {
            release: CorpusRelease {
                release_id: id.into(),
                knowledge_version: id.into(),
                patch: "patch".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    record.source_id.clone(),
                    BTreeMap::from([(record.logical_id.clone(), record.revision)]),
                )]),
            },
            revisions: vec![record.clone()],
            heads: vec![record],
        }
    }

    fn feed_record(revision: u64) -> brain_contracts::SourceRecordV2 {
        let mut record = record(revision, SourceVisibility::Public);
        let mut origin = origin_from_record(&record).unwrap();
        record.source_id = "google-sheet/fixture".into();
        record.visibility = SourceVisibility::Internal;
        record.allowed_scopes = BTreeSet::from(["second_brain.internal".into()]);
        origin.identity.source_id = record.source_id.clone();
        origin.policy.visibility = record.visibility;
        origin.policy.allowed_scopes = record.allowed_scopes.clone();
        origin.policy.publication_allowed = false;
        origin.policy.raw_retention_allowed = true;
        origin.bind_record(&mut record).unwrap();
        record
    }

    #[test]
    fn internal_sources_require_private_rights_and_exact_scope() {
        let base = snapshot(feed_record(1), "base");
        let candidate = snapshot(feed_record(2), "candidate");
        let allowed = BTreeSet::from(["google-sheet/fixture".into()]);
        check_rights(&base, &candidate, &allowed, Target::SecondBrainInternal).unwrap();
        assert!(check_rights(&base, &candidate, &allowed, Target::Standard).is_err());
        for variant in 0..4 {
            let mut changed = candidate.clone();
            let mut origin = origin_from_record(&changed.revisions[0]).unwrap();
            match variant {
                0 => origin.policy.publication_allowed = true,
                1 => origin.policy.provider_egress_allowed = true,
                2 => origin.policy.raw_retention_allowed = false,
                _ => {
                    changed.revisions[0].allowed_scopes.clear();
                    origin.policy.allowed_scopes.clear();
                }
            }
            origin.bind_record(&mut changed.revisions[0]).unwrap();
            changed.heads = changed.revisions.clone();
            assert!(check_rights(&base, &changed, &allowed, Target::SecondBrainInternal).is_err());
        }
    }

    #[test]
    fn entity_variant_preserves_feed_isolation_and_original_rights() {
        let mut first = feed_record(1);
        let mut origin = origin_from_record(&first).unwrap();
        first.source_id = "game-fixture".into();
        first.allowed_scopes = BTreeSet::from(["source.review:game-fixture".into()]);
        origin.identity.source_id = first.source_id.clone();
        origin.policy.allowed_scopes = first.allowed_scopes.clone();
        origin.parser_family = "dbrain-sources/wiki-spielwissen".into();
        origin.bind_record(&mut first).unwrap();
        let mut next = first.clone();
        next.revision = 2;
        let base = snapshot(first, "base");
        let candidate = snapshot(next, "candidate");
        let allowed = BTreeSet::from(["game-fixture".into()]);
        for target in [Target::EntityProfiles, Target::EntityProfilesInternal] {
            check_rights(&base, &candidate, &allowed, target).unwrap();
        }
        assert!(check_rights(&base, &candidate, &allowed, Target::SecondBrainInternal).is_err());
        let feed_base = snapshot(feed_record(1), "base");
        let feed_candidate = snapshot(feed_record(2), "candidate");
        for target in [Target::EntityProfiles, Target::EntityProfilesInternal] {
            assert!(check_rights(
                &feed_base,
                &feed_candidate,
                &BTreeSet::from(["google-sheet/fixture".into()]),
                target
            )
            .is_err());
        }
        for variant in 0..3 {
            let mut changed = candidate.clone();
            let mut origin = origin_from_record(&changed.revisions[0]).unwrap();
            match variant {
                0 => origin.policy.publication_allowed = true,
                1 => {
                    origin.policy.allowed_scopes.insert("foreign.scope".into());
                }
                _ => origin.parser_family = "patchnotes".into(),
            };
            if variant == 1 {
                changed.revisions[0].allowed_scopes = origin.policy.allowed_scopes.clone();
            }
            origin.bind_record(&mut changed.revisions[0]).unwrap();
            changed.heads = changed.revisions.clone();
            for target in [Target::EntityProfiles, Target::EntityProfilesInternal] {
                assert!(check_rights(&base, &changed, &allowed, target).is_err());
            }
        }
    }

    #[test]
    fn spielprofile_waehlen_standardplan_mit_gekoppelten_pins() {
        let cli = Cli::try_parse_from([
            "brain-candidate-activate",
            "--config",
            "/tmp/runtime.json",
            "--target",
            "entity-profiles",
            "--base-release-id",
            "base",
            "--base-release-sha256",
            &"a".repeat(64),
            "--candidate-release-id",
            "candidate",
            "--candidate-release-sha256",
            &"b".repeat(64),
            "--expected-serve-config-sha256",
            &"c".repeat(64),
            "--allow-source",
            "git-game-facts-derived",
        ])
        .unwrap();
        assert_eq!(cli.activation_target(), ActivationTarget::Standard);
        let mut old = c9_value();
        old["credentials"][1]["scopes"] = serde_json::json!(["bot.public"]);
        let base_pin = old["release"].clone();
        old["credentials"][1]["release"] = base_pin.clone();
        old["credentials"][2]["release"] = base_pin.clone();
        old["internal_operator"]["release"] = base_pin;
        let (_dir, artifacts, writer, runtime) = plan_fixture(&old);
        let candidate = release("candidate", "candidate-kv", 2);
        let plan = ActivationPlan::prepare_target(
            &runtime,
            &artifacts,
            &candidate,
            &writer,
            cli.activation_target(),
        )
        .unwrap();
        let journal: Journal =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        let new_bytes = artifacts.read(&journal.new_config).unwrap();
        let new: serde_json::Value = serde_json::from_slice(&new_bytes).unwrap();
        assert_eq!(new["release"]["id"], "candidate");
        for index in [1, 2] {
            assert_eq!(new["credentials"][index]["release"], new["release"]);
            assert_eq!(
                new["credentials"][index]["scopes"],
                old["credentials"][index]["scopes"]
            );
            assert_eq!(
                new["credentials"][index]["provider_egress"],
                old["credentials"][index]["provider_egress"]
            );
        }
        assert_eq!(new["internal_operator"]["release"], new["release"]);
        check_config_transition(
            &serde_json::to_vec(&old).unwrap(),
            &new_bytes,
            &candidate,
            cli.activation_target(),
        )
        .unwrap();
    }

    #[tokio::test]
    async fn local_operator_verifies_raw_sources_without_expanding_the_consumer() {
        use dbrain_sources::{game_files, knowledge_import};
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path().join("repo");
        std::fs::create_dir(&repo).unwrap();
        let git = |args: &[&str]| {
            let output = std::process::Command::new("/usr/bin/git")
                .args(args)
                .current_dir(&repo)
                .output()
                .unwrap();
            assert!(output.status.success());
            String::from_utf8(output.stdout).unwrap().trim().to_owned()
        };
        git(&["init", "--quiet"]);
        let content = br#"{"hero_fixture":{"cooldown":12.5}}"#;
        std::fs::write(repo.join("heroes.json"), content).unwrap();
        git(&["add", "heroes.json"]);
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "Fixture",
        ]);
        git(&[
            "remote",
            "add",
            "origin",
            "https://github.com/deadlock-wiki/deadlock-data.git",
        ]);
        let sha = git(&["rev-parse", "HEAD"]);
        git(&["update-ref", "refs/remotes/origin/main", &sha]);
        let options = game_files::GameFileOptions {
            root: repo.clone(),
            app_id: 1422450,
            source_id: "game-fixture".into(),
            observed_at: "2026-10-04T12:00:00Z".into(),
            build_id: None,
            manifest_id: None,
            source_revision: Some(sha.clone()),
            depot_id: None,
            language: "en".into(),
            attribution: "Fixture".into(),
            license_name: "unverified".into(),
            license_url: None,
            provenance: serde_json::json!({"git_commit":sha,"repository_url":"https://github.com/deadlock-wiki/deadlock-data"}),
            max_file_bytes: 4096,
        };
        let mut jsonl = Vec::new();
        game_files::extract_game_files(&options, &mut jsonl).unwrap();
        let policy = knowledge_import::ImportPolicy {
            sources: BTreeMap::from([(
                options.source_id.clone(),
                knowledge_import::ImportGrant {
                    internal_read_allowed: true,
                    raw_retention_allowed: true,
                    authorization_ref: Some("operator:fixture".into()),
                    provenance_evidence_ref: Some("fixture:git".into()),
                    ..Default::default()
                },
            )]),
        };
        let imported = knowledge_import::prepare_knowledge_jsonl(
            std::io::Cursor::new(jsonl),
            &policy,
            game_files::EXTRACTOR_VERSION,
        )
        .unwrap();
        assert_eq!(imported.records().len(), 1);
        let record = imported.records()[0].record.clone();
        let candidate = snapshot(record, "candidate");
        let policy_path = dir.path().join("policy.json");
        std::fs::write(&policy_path, serde_json::to_vec(&policy).unwrap()).unwrap();
        std::fs::set_permissions(&policy_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let mut config: serde_json::Value =
            serde_json::from_str(include_str!("../../config/maintenance.example.json")).unwrap();
        config["game_sources"] = serde_json::json!([{
            "id":"game-fixture", "path":repo, "origin":"https://github.com/deadlock-wiki/deadlock-data.git",
            "source_ref":"refs/remotes/origin/main", "source_paths":["heroes.json"]
        }]);
        config["internal_doc_scopes"] =
            serde_json::json!(["internal_docs", "source.review:game-fixture"]);
        let config_path = dir.path().join("maintenance.json");
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        let mut runtime: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../ops/brain-maintenance/runtime.example.json"
        ))
        .unwrap();
        runtime["maintenance_config"] = serde_json::json!(config_path);
        runtime["entity_profile_corpus_root"] = serde_json::json!(dir.path());
        runtime["entity_profile_sources"] = serde_json::json!([{
            "repository_id":"game-fixture", "paths":["heroes.json"], "extraction":options,
            "import_policy":policy_path, "canonical_raw_dir":null
        }]);
        let runtime_path = dir.path().join("runtime.json");
        std::fs::write(&runtime_path, serde_json::to_vec(&runtime).unwrap()).unwrap();
        let mut runtime = RuntimeConfig::load(&runtime_path).unwrap();
        let mut serve_value = c9_value();
        let credential = serve_value["credentials"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|grant| grant["actor_id"] == "second-brain")
            .unwrap();
        credential["scopes"] =
            serde_json::json!(["second_brain.internal", "source.review:game-fixture"]);
        assert!(brain_serve::Config::parse(&serde_json::to_vec(&serve_value).unwrap()).is_err());
        let allowed = BTreeSet::from(["game-fixture".into()]);
        let serve = brain_serve::Config::parse(&serde_json::to_vec(&c9_value()).unwrap()).unwrap();
        let credential = serve
            .credentials
            .iter()
            .find(|grant| grant.actor_id == "second-brain")
            .unwrap();
        let consumer = brain_contracts::Principal {
            actor_id: credential.actor_id.clone(),
            channel: credential.channel.clone(),
            scopes: credential.scopes.clone(),
            provider_egress: credential.provider_egress.clone(),
        };
        assert!(candidate.authorized(&consumer, false).unwrap().is_empty());
        let (registered_path, commit, registered_origin, origin, relative_path) =
            brain_storage::LocalPgReader::entity_profile_repository(
                &config_path,
                1000,
                &candidate.revisions[0],
            )
            .unwrap();
        assert_eq!(registered_path, repo);
        assert_eq!(commit, sha);
        assert_eq!(relative_path, "heroes.json");
        let pinned =
            dbrain_sources::git_source::PinnedRepository::open(&registered_path, &commit).unwrap();
        pinned.require_origin(&[&registered_origin]).unwrap();
        assert_eq!(origin, "https://github.com/deadlock-wiki/deadlock-data");
        assert_eq!(pinned.read_blob(&relative_path).unwrap(), content);
        config["game_sources"][0]["source_paths"] = serde_json::json!(["other.json"]);
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        assert!(brain_storage::LocalPgReader::entity_profile_repository(
            &config_path,
            1000,
            &candidate.revisions[0]
        )
        .is_err());
        config["game_sources"][0]["source_paths"] = serde_json::json!(["heroes.json"]);
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        let pool = PgPoolOptions::new().connect_lazy_with(PgConnectOptions::new());
        let store = brain_storage::PgStore::new(pool.clone());
        check_entity_profile_sources(&runtime, &store, &pool, &candidate, &candidate, &allowed)
            .await
            .unwrap();
        let pg = ScratchPg::start();
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55442)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let store = brain_storage::PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        store.migrate_entity_profiles().await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; CREATE TABLE brain.entities(id bigint,entity_type text,canonical_name text,primary_external_id text); CREATE TABLE brain.entity_aliases(id bigint,entity_id bigint,alias text,alias_kind text); INSERT INTO brain.entities VALUES(1,'hero','Fixture','hero_fixture')")
            .execute(&pool).await.unwrap();
        let grants = include_str!("../../../../../ops/brain-postgres/grants.sql")
            .lines()
            .filter(|line| !line.starts_with('\\'))
            .collect::<Vec<_>>()
            .join("\n");
        for _ in 0..2 {
            sqlx::raw_sql(&grants).execute(&pool).await.unwrap();
        }
        sqlx::raw_sql("CREATE TABLE brain.patch_changes(patch_date text,entity_type text,entity_name text,ability_name text,stat_name text,old_value text,new_value text,change_type text,confidence double precision,raw_line text); INSERT INTO brain.patch_changes VALUES('2026-09-16','hero','Fixture',NULL,'Health','500','550','increase',1,'Gesperrte Originalzeile')")
            .execute(&pool).await.unwrap();
        sqlx::raw_sql(&grants).execute(&pool).await.unwrap();
        let ingest_pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55442)
                    .username("brain_ingest")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let ingest = brain_storage::PgStore::new(ingest_pool.clone());
        let catalog = dbrain_sources::entity_binding::load_entity_catalog(&ingest_pool)
            .await
            .unwrap();
        assert_eq!(catalog.len(), 1);
        let loaded_config =
            brain_maintenance::integration::runner::load_maintenance(&config_path).unwrap();
        let principal = brain_maintenance::integration::runner::local_operator_principal(
            require_operator_config(&config_path).unwrap(),
            &config_path,
        )
        .unwrap();
        let pinned = dbrain_sources::git_source::PinnedRepository::open(&repo, &sha).unwrap();
        brain_maintenance::integration::entity_profiles::refresh_git_knowledge(
            &runtime.entity_profile_sources[0],
            &pinned,
            &repo,
            &ingest,
            &ingest_pool,
        )
        .await
        .unwrap();
        let original = &candidate.revisions[0];
        dbrain_sources::entity_binding::bind_stored_document(
            &ingest,
            &original.source_id,
            &original.logical_id,
            original.revision,
            &catalog,
        )
        .await
        .unwrap();
        ingest.publish_release(&candidate.release).await.unwrap();
        let originals = ingest
            .snapshot(&candidate.release.release_id)
            .await
            .unwrap();
        let original = &originals.revisions[0];
        let repositories =
            brain_maintenance::integration::entity_profiles::registered_git_repositories(
                &loaded_config,
                &originals,
                &principal,
            )
            .unwrap();
        let verified = dbrain_sources::entity_binding::derivation::derive_git_entity_profile(
            &ingest,
            &originals.release.release_id,
            &principal,
            &catalog[0].identity.entity_key,
            &repositories,
        )
        .await
        .unwrap();
        let document =
            brain_maintenance::integration::entity_profiles::persist_verified_git_profile(
                &ingest, &verified,
            )
            .await
            .unwrap();
        let sources = vec!["git-game-facts-derived".into()];
        let initial = brain_maintenance::integration::entity_profiles::publish_refreshed_sources(
            &ingest,
            &ingest_pool,
            &candidate.release.release_id,
            &sources,
        )
        .await
        .unwrap()
        .unwrap();
        let derived_allowed = BTreeSet::from([sources[0].clone()]);
        assert!(check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &candidate,
            &candidate,
            &derived_allowed
        )
        .await
        .is_err());
        let initial_snapshot = ingest
            .snapshot(&initial.candidate.release_id)
            .await
            .unwrap();
        check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &candidate,
            &initial_snapshot,
            &derived_allowed,
        )
        .await
        .unwrap();
        assert_eq!(
            brain_maintenance::integration::entity_profiles::retire_removed_git_profiles(
                &ingest,
                &ingest_pool,
                &loaded_config,
                &principal,
                &BTreeSet::new(),
                &allowed,
                dir.path()
            )
            .await
            .unwrap(),
            1
        );
        let retired = brain_maintenance::integration::entity_profiles::publish_refreshed_sources(
            &ingest,
            &ingest_pool,
            &initial.candidate.release_id,
            &sources,
        )
        .await
        .unwrap()
        .unwrap();
        let base = ingest
            .snapshot(&initial.candidate.release_id)
            .await
            .unwrap();
        let empty = ingest
            .snapshot(&retired.candidate.release_id)
            .await
            .unwrap();
        assert!(empty.release.source_revisions[&sources[0]].is_empty());
        assert!(empty.revisions.contains(original));
        check_transition(&base.release, &empty.release, &derived_allowed).unwrap();
        check_rights(&base, &empty, &derived_allowed, Target::EntityProfiles).unwrap();
        check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &base,
            &empty,
            &derived_allowed,
        )
        .await
        .unwrap();
        check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &empty,
            &empty,
            &derived_allowed,
        )
        .await
        .unwrap();
        sqlx::query("UPDATE brain.patch_changes SET new_value='600'")
            .execute(&pool)
            .await
            .unwrap();
        check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &empty,
            &empty,
            &derived_allowed,
        )
        .await
        .unwrap();
        sqlx::query("UPDATE brain.patch_changes SET new_value='550'")
            .execute(&pool)
            .await
            .unwrap();
        assert!(
            brain_maintenance::integration::entity_profiles::publish_refreshed_sources(
                &ingest,
                &ingest_pool,
                &empty.release.release_id,
                &sources
            )
            .await
            .unwrap()
            .is_none()
        );
        let mut forged = empty.clone();
        forged.release.source_revisions.insert(
            sources[0].clone(),
            BTreeMap::from([(document.logical_id.clone(), document.revision)]),
        );
        assert!(check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &base,
            &forged,
            &derived_allowed
        )
        .await
        .is_err());
        let restored =
            brain_maintenance::integration::entity_profiles::persist_verified_git_profile(
                &ingest, &verified,
            )
            .await
            .unwrap();
        assert_eq!(restored.revision, document.revision + 2);
        assert!(check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &empty,
            &empty,
            &derived_allowed
        )
        .await
        .is_err());
        let reappeared =
            brain_maintenance::integration::entity_profiles::publish_refreshed_sources(
                &ingest,
                &ingest_pool,
                &empty.release.release_id,
                &sources,
            )
            .await
            .unwrap()
            .unwrap();
        let reappeared = ingest
            .snapshot(&reappeared.candidate.release_id)
            .await
            .unwrap();
        check_transition(&empty.release, &reappeared.release, &derived_allowed).unwrap();
        check_rights(
            &empty,
            &reappeared,
            &derived_allowed,
            Target::EntityProfiles,
        )
        .unwrap();
        check_entity_profile_sources(
            &runtime,
            &ingest,
            &ingest_pool,
            &empty,
            &reappeared,
            &derived_allowed,
        )
        .await
        .unwrap();
        ingest_pool.close().await;
        pool.close().await;
        config["internal_doc_scopes"] = serde_json::json!(["internal_docs"]);
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        assert!(check_entity_profile_sources(
            &runtime, &store, &pool, &candidate, &candidate, &allowed
        )
        .await
        .is_err());
        config["internal_doc_scopes"] =
            serde_json::json!(["internal_docs", "source.review:game-fixture"]);
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        runtime.entity_profile_sources[0].paths = BTreeSet::from(["other.json".into()]);
        assert!(check_entity_profile_sources(
            &runtime, &store, &pool, &candidate, &candidate, &allowed
        )
        .await
        .is_err());
        runtime.entity_profile_sources[0].paths = BTreeSet::from(["heroes.json".into()]);
        let mut changed_policy = policy;
        changed_policy
            .sources
            .get_mut("game-fixture")
            .unwrap()
            .authorization_ref = Some("operator:changed".into());
        std::fs::write(&policy_path, serde_json::to_vec(&changed_policy).unwrap()).unwrap();
        assert!(check_entity_profile_sources(
            &runtime, &store, &pool, &candidate, &candidate, &allowed
        )
        .await
        .is_err());
    }

    #[test]
    fn allowed_pin_removal_needs_a_newer_unchanged_policy_tombstone() {
        let mut base = snapshot(record(1, SourceVisibility::Public), "base");
        let candidate = CorpusSnapshot {
            release: CorpusRelease {
                release_id: "candidate".into(),
                knowledge_version: "candidate-kv".into(),
                patch: base.release.patch.clone(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::new(),
            },
            revisions: Vec::new(),
            heads: Vec::new(),
        };
        check_transition(&base.release, &candidate.release, &allowed()).unwrap();
        assert!(check_rights(&base, &candidate, &allowed(), Target::Standard).is_err());
        base.heads[0] = record(2, SourceVisibility::Public);
        base.heads[0].tombstone = true;
        check_rights(&base, &candidate, &allowed(), Target::Standard).unwrap();
        let mut origin = origin_from_record(&base.heads[0]).unwrap();
        origin.policy.publication_allowed = false;
        origin.bind_record(&mut base.heads[0]).unwrap();
        assert!(check_rights(&base, &candidate, &allowed(), Target::Standard).is_err());
    }

    #[test]
    fn rights_reject_internal_changes_policy_changes_and_superseded_heads() {
        let base = snapshot(record(1, SourceVisibility::Public), "base");
        let candidate = snapshot(record(2, SourceVisibility::Public), "candidate");
        check_rights(&base, &candidate, &allowed(), Target::Standard).unwrap();
        let internal = snapshot(record(2, SourceVisibility::Internal), "candidate");
        assert!(check_rights(&base, &internal, &allowed(), Target::Standard).is_err());
        let mut changed = candidate.clone();
        let mut origin = origin_from_record(&changed.revisions[0]).unwrap();
        origin.policy.provider_egress_allowed = true;
        origin.bind_record(&mut changed.revisions[0]).unwrap();
        changed.heads = changed.revisions.clone();
        assert!(check_rights(&base, &changed, &allowed(), Target::Standard).is_err());
        let mut egress_changed = candidate.clone();
        egress_changed.revisions[0]
            .metadata
            .insert("egress".into(), "public".into());
        egress_changed.heads = egress_changed.revisions.clone();
        assert!(check_rights(&base, &egress_changed, &allowed(), Target::Standard).is_err());
        let mut superseded = candidate.clone();
        superseded.heads[0] = record(3, SourceVisibility::Public);
        assert!(check_rights(&base, &superseded, &allowed(), Target::Standard).is_err());
    }

    #[test]
    fn durable_intent_and_confirmation_replay_after_reporting_failure() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let artifacts = Artifacts::open(dir.path()).unwrap();
        let id = "a".repeat(64);
        let mut prepared = Prepared {
            request_sha256: id.clone(),
            journal_ref: format!("{}.json", "b".repeat(64)),
            confirmed: false,
        };
        artifacts
            .save_call_receipt(&id, &serde_json::to_vec(&prepared).unwrap())
            .unwrap();
        let resumed: Prepared =
            serde_json::from_slice(&artifacts.call_receipt(&id).unwrap().unwrap()).unwrap();
        assert!(!resumed.confirmed);
        prepared.confirmed = true;
        artifacts
            .save_call_validation(&id, &serde_json::to_vec(&prepared).unwrap())
            .unwrap();
        let resumed: Prepared =
            serde_json::from_slice(&artifacts.call_receipt(&id).unwrap().unwrap()).unwrap();
        assert!(resumed.confirmed);
        assert_eq!(resumed.journal_ref, prepared.journal_ref);
        artifacts
            .save_call_validation(&id, &serde_json::to_vec(&prepared).unwrap())
            .unwrap();
    }

    fn plan_fixture(
        value: &serde_json::Value,
    ) -> (tempfile::TempDir, Artifacts, ConfigWriter, RuntimeConfig) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let artifacts = Artifacts::open(dir.path()).unwrap();
        let path = dir.path().join("serve.json");
        std::fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let writer = ConfigWriter::lock(&path).unwrap();
        let mut runtime: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../ops/brain-maintenance/runtime.example.json"
        ))
        .unwrap();
        runtime["serve_config"] = serde_json::json!(path);
        runtime["artifact_dir"] = serde_json::json!(dir.path());
        let runtime_path = dir.path().join("runtime.json");
        std::fs::write(&runtime_path, serde_json::to_vec(&runtime).unwrap()).unwrap();
        let runtime = RuntimeConfig::load(&runtime_path).unwrap();
        (dir, artifacts, writer, runtime)
    }

    #[test]
    fn internal_plan_changes_only_coupled_pins_and_hashes_loaded_bindings() {
        let old = c9_value();
        let (_dir, artifacts, writer, runtime) = plan_fixture(&old);
        let candidate = release("internal-candidate", "internal-candidate-kv", 2);
        let plan = ActivationPlan::prepare_target(
            &runtime,
            &artifacts,
            &candidate,
            &writer,
            ActivationTarget::SecondBrainInternal,
        )
        .unwrap();
        assert_eq!(plan.target(), ActivationTarget::SecondBrainInternal);
        let journal: Journal =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        let new_bytes = artifacts.read(&journal.new_config).unwrap();
        let new: serde_json::Value = serde_json::from_slice(&new_bytes).unwrap();
        assert_eq!(new["release"], old["release"]);
        assert_eq!(new["credentials"][0], old["credentials"][0]);
        assert_eq!(new["credentials"][1], old["credentials"][1]);
        assert_eq!(
            new["credentials"][2]["release"],
            new["internal_operator"]["release"]
        );
        assert_eq!(
            new["internal_operator"]["release"]["id"],
            candidate.release_id
        );
        check_config_transition(
            &serde_json::to_vec(&old).unwrap(),
            &new_bytes,
            &candidate,
            ActivationTarget::SecondBrainInternal,
        )
        .unwrap();
        let old_config = brain_serve::Config::parse(&serde_json::to_vec(&old).unwrap()).unwrap();
        let new_config = brain_serve::Config::parse(&new_bytes).unwrap();
        assert_ne!(
            old_config.release_bindings_sha256(),
            new_config.release_bindings_sha256()
        );
        assert_eq!(
            journal.new_bindings_sha256.as_deref(),
            Some(new_config.release_bindings_sha256().as_str())
        );
        let loaded = BTreeSet::from([
            ("base".into(), "base-kv".into()),
            ("docs".into(), "docs-kv".into()),
            (
                candidate.release_id.clone(),
                candidate.knowledge_version.clone(),
            ),
        ]);
        let scopes = BTreeSet::from([
            ("docs".into(), "docs.public".into()),
            (candidate.release_id.clone(), "second_brain.internal".into()),
        ]);
        assert_eq!(
            new_config.release_bindings_sha256(),
            brain_serve::config::loaded_release_bindings_sha256(
                ("base", "base-kv"),
                &loaded,
                &scopes,
                true
            )
        );
        let mut wrong_scopes = scopes.clone();
        wrong_scopes.remove(&(candidate.release_id.clone(), "second_brain.internal".into()));
        wrong_scopes.insert(("internal-base".into(), "second_brain.internal".into()));
        assert_ne!(
            new_config.release_bindings_sha256(),
            brain_serve::config::loaded_release_bindings_sha256(
                ("base", "base-kv"),
                &loaded,
                &wrong_scopes,
                true
            )
        );
        assert_eq!(writer.read().unwrap(), serde_json::to_vec(&old).unwrap());
    }

    #[test]
    fn internal_journal_cannot_change_public_pins_or_omit_binding_proof() {
        let old = c9_value();
        let (_dir, artifacts, writer, runtime) = plan_fixture(&old);
        let candidate = release("internal-candidate", "internal-candidate-kv", 2);
        let plan = ActivationPlan::prepare_target(
            &runtime,
            &artifacts,
            &candidate,
            &writer,
            ActivationTarget::SecondBrainInternal,
        )
        .unwrap();
        let mut journal: serde_json::Value =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        journal
            .as_object_mut()
            .unwrap()
            .remove("new_bindings_sha256");
        let reference = artifacts
            .put(&serde_json::to_vec(&journal).unwrap(), "json")
            .unwrap();
        assert!(
            ActivationPlan::load(&runtime, &artifacts, &reference, &candidate.release_id).is_err()
        );
        let mut journal: serde_json::Value =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        let mut altered: serde_json::Value = serde_json::from_slice(
            &artifacts
                .read(journal["new_config"].as_str().unwrap())
                .unwrap(),
        )
        .unwrap();
        altered["credentials"][1]["release"]["id"] = serde_json::json!("other-docs");
        let altered = serde_json::to_vec(&altered).unwrap();
        journal["new_config"] = serde_json::json!(artifacts.put(&altered, "json").unwrap());
        journal["new_bindings_sha256"] = serde_json::json!(brain_serve::Config::parse(&altered)
            .unwrap()
            .release_bindings_sha256());
        let reference = artifacts
            .put(&serde_json::to_vec(&journal).unwrap(), "json")
            .unwrap();
        assert!(
            ActivationPlan::load(&runtime, &artifacts, &reference, &candidate.release_id).is_err()
        );
    }

    #[test]
    fn legacy_standard_journal_still_loads_without_new_fields() {
        let old = c9_value();
        let (_dir, artifacts, writer, runtime) = plan_fixture(&old);
        let candidate = release("candidate", "candidate-kv", 2);
        let plan = ActivationPlan::prepare(&runtime, &artifacts, &candidate, &writer).unwrap();
        let mut journal: serde_json::Value =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        for key in ["target", "old_bindings_sha256", "new_bindings_sha256"] {
            journal.as_object_mut().unwrap().remove(key);
        }
        let reference = artifacts
            .put(&serde_json::to_vec(&journal).unwrap(), "json")
            .unwrap();
        let loaded =
            ActivationPlan::load(&runtime, &artifacts, &reference, &candidate.release_id).unwrap();
        assert_eq!(loaded.target(), ActivationTarget::Standard);
    }

    #[test]
    fn activation_journal_is_durable_and_preserves_other_configuration() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let artifacts = Artifacts::open(dir.path()).unwrap();
        let config_path = dir.path().join("serve.json");
        let mut value = serve_value();
        value["release"] = serde_json::json!({"id":"base","knowledge_version":"base-kv"});
        let bytes = serde_json::to_vec(&value).unwrap();
        std::fs::write(&config_path, &bytes).unwrap();
        std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let writer = ConfigWriter::lock(&config_path).unwrap();
        let mut runtime_value: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../ops/brain-maintenance/runtime.example.json"
        ))
        .unwrap();
        runtime_value["serve_config"] = serde_json::json!(config_path);
        let runtime_path = dir.path().join("runtime.json");
        std::fs::write(&runtime_path, serde_json::to_vec(&runtime_value).unwrap()).unwrap();
        let runtime = RuntimeConfig::load(&runtime_path).unwrap();
        let candidate = release("candidate", "candidate-kv", 2);
        let plan = ActivationPlan::prepare(&runtime, &artifacts, &candidate, &writer).unwrap();
        let journal: Journal =
            serde_json::from_slice(&artifacts.read(plan.journal_ref()).unwrap()).unwrap();
        assert_eq!(artifacts.read(&journal.old_config).unwrap(), bytes);
        let mut new_value: serde_json::Value =
            serde_json::from_slice(&artifacts.read(&journal.new_config).unwrap()).unwrap();
        assert_eq!(new_value["release"]["id"], "candidate");
        new_value["release"] = value["release"].clone();
        assert_eq!(new_value, value);
        assert_eq!(writer.read().unwrap(), bytes);
        writer
            .replace(&bytes, &artifacts.read(&journal.new_config).unwrap())
            .unwrap();
        assert!(!plan.needs_rebase(&writer).unwrap());
        writer
            .replace(&artifacts.read(&journal.new_config).unwrap(), b"newer")
            .unwrap();
        assert!(plan.needs_rebase(&writer).unwrap());
    }
}
