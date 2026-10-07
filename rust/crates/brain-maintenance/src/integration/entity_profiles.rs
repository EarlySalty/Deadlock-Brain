use super::artifacts::Artifacts;
use anyhow::{ensure, Context, Result};
use brain_contracts::entity_profile::EntityProfile;
use std::{
    collections::BTreeSet,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
struct GitKnowledgeStep {
    source_id: String,
    commit: String,
    step: &'static str,
    document_path: Option<String>,
}

impl std::fmt::Display for GitKnowledgeStep {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Spielquellenimport: {}", self.step)
    }
}

impl std::error::Error for GitKnowledgeStep {}

pub(super) fn refresh_failure(error: &anyhow::Error) -> serde_json::Value {
    let mut status = serde_json::json!({"code": "ENTITY_PROFILE_REFRESH_FAILED"});
    if let Some(dbrain_sources::SourcesError::EntityDerivation {
        entity_key, step, ..
    }) = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<dbrain_sources::SourcesError>())
    {
        status["entity_key"] = serde_json::json!(entity_key);
        status["step"] = serde_json::json!(step);
    }
    if let Some(step) = error.downcast_ref::<GitKnowledgeStep>() {
        status["source_id"] = serde_json::json!(step.source_id);
        status["commit"] = serde_json::json!(step.commit);
        status["step"] = serde_json::json!(step.step);
        if let Some(path) = &step.document_path {
            status["document_path"] = serde_json::json!(path);
        }
    }
    if let Some(validation) =
        error.downcast_ref::<dbrain_sources::knowledge_contract::KnowledgeValidationErrors>()
    {
        status["validation"] = serde_json::json!(validation.errors.iter().map(|issue| {
            serde_json::json!({
                "line": issue.line,
                "cause": if issue.message.contains("allocation_budget_exceeded_preserved_as_text") {
                    "allocation_budget_exceeded_preserved_as_text"
                } else if issue.message == "Nichtleerer Text ohne Steuerzeichen erforderlich" {
                    "Nichtleerer Text ohne Steuerzeichen erforderlich"
                } else { "Wissensvertrag verletzt" }
            })
        }).collect::<Vec<_>>());
    } else if let Some(database) = error.chain().find_map(|cause| {
        if let Some(database) = cause.downcast_ref::<sqlx::Error>() {
            return Some(database);
        }
        let storage = cause
            .downcast_ref::<brain_storage::StorageError>()
            .or_else(
                || match cause.downcast_ref::<dbrain_sources::SourcesError>() {
                    Some(dbrain_sources::SourcesError::Storage(storage))
                    | Some(dbrain_sources::SourcesError::EntityDerivation {
                        source: storage,
                        ..
                    }) => Some(storage),
                    _ => None,
                },
            );
        match storage {
            Some(brain_storage::StorageError::Database(database)) => Some(database),
            _ => None,
        }
    }) {
        status["cause"] = serde_json::json!("Datenbankfehler");
        status["error_class"] = serde_json::json!(match database
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref()
        {
            Some("57014") => "query_cancelled",
            Some("55P03") => "lock_not_available",
            _ => "database_error",
        });
        if let Some(database) = database.as_database_error() {
            status["sqlstate"] = serde_json::json!(database.code());
        }
    } else if let Some(port) = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<brain_contracts::PortError>())
    {
        let (class, sqlstate) = match port {
            brain_contracts::PortError::Unavailable(message)
                if message == "postgres_query_cancelled" =>
            {
                ("query_cancelled", Some("57014"))
            }
            brain_contracts::PortError::Unavailable(message)
                if message == "postgres_lock_not_available" =>
            {
                ("lock_not_available", Some("55P03"))
            }
            brain_contracts::PortError::Unavailable(_) => ("unavailable", None),
            brain_contracts::PortError::BudgetExceeded => ("budget_exceeded", None),
            brain_contracts::PortError::InvalidResponse(_) => ("invalid_response", None),
            _ => ("port_error", None),
        };
        status["cause"] = serde_json::json!("Steckbriefaktualisierung abgebrochen");
        status["error_class"] = serde_json::json!(class);
        if let Some(sqlstate) = sqlstate {
            status["sqlstate"] = serde_json::json!(sqlstate);
        }
    } else if let Some(io) = error.downcast_ref::<std::io::Error>() {
        status["cause"] = serde_json::json!(format!("Dateizugriff: {:?}", io.kind()));
    } else {
        status["cause"] = serde_json::json!("Steckbriefaktualisierung abgebrochen");
    }
    status
}

pub async fn refresh_git_knowledge(
    source: &super::runtime_config::EntityProfileSource,
    pinned: &dbrain_sources::git_source::PinnedRepository,
    repository_path: &Path,
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
) -> Result<serde_json::Value> {
    use dbrain_sources::{game_files, knowledge_contract, knowledge_import};
    let step = |step| GitKnowledgeStep {
        source_id: source.extraction.source_id.clone(),
        commit: pinned.commit().to_owned(),
        step,
        document_path: None,
    };
    let staged = tempfile::tempdir()?;
    let mut files = std::collections::BTreeMap::new();
    for prefix in &source.paths {
        for blob in pinned.files(prefix).context(step("Git-Dateiliste"))? {
            files.insert(blob.path.clone(), blob);
        }
    }
    ensure!(
        !files.is_empty(),
        "Spielquellen enthalten keine gepinnten Dateien"
    );
    for path in files.keys() {
        let target = staged.path().join(path);
        std::fs::create_dir_all(
            target
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Quellpfad fehlt"))?,
        )?;
        std::fs::write(target, pinned.read_blob(path).context(step("Git-Blob"))?)
            .context(step("Dateibereitstellung"))?;
    }
    let mut options = source.extraction.clone();
    options.root = staged.path().to_owned();
    options.source_revision = Some(pinned.commit().to_owned());
    options.observed_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    options.provenance["git_commit"] = serde_json::json!(pinned.commit());
    let mut output = tempfile::tempfile()?;
    let inventory =
        game_files::extract_game_files(&options, &mut output).context(step("Extraktion"))?;
    ensure!(
        inventory.gaps.is_empty(),
        "Spielquellenextraktion meldet offene Lücken"
    );
    use std::io::{Seek, SeekFrom};
    output.seek(SeekFrom::Start(0))?;
    let input = knowledge_contract::validate_knowledge_jsonl(std::io::BufReader::new(output))
        .map_err(|error| {
            let mut context = step("JSONL-Validierung");
            context.document_path = error.errors.first().and_then(|issue| {
                inventory
                    .files
                    .iter()
                    .filter(|file| file.disposition == "extracted")
                    .nth(issue.line.checked_sub(1)?)
                    .map(|file| file.relative_path.clone())
            });
            anyhow::Error::new(error).context(context)
        })?;
    let policy: knowledge_import::ImportPolicy = serde_json::from_slice(
        &super::runtime_config::read_bounded(&source.import_policy, 256 * 1024)?,
    )
    .context(step("Importfreigabe"))?;
    let prepared = knowledge_import::prepare_validated_knowledge(
        &input,
        &policy,
        game_files::EXTRACTOR_VERSION,
    )
    .context(step("Importvorbereitung"))?;
    if let Some(raw_dir) = &source.canonical_raw_dir {
        dbrain_sources::deadlock_data::pull_deadlock_data_with_pool(
            pool,
            raw_dir,
            dbrain_sources::deadlock_data::PullDeadlockDataOptions {
                repo_dir: repository_path.to_owned(),
                pin: dbrain_sources::source_pins::DeadlockDataPin {
                    commit: pinned.commit().to_owned(),
                    parser_revision: dbrain_sources::deadlock_data::PARSER_REVISION.into(),
                    schema_version: None,
                    data_version: None,
                },
                update_repo: false,
            },
        )
        .await?;
        dbrain_normalize::normalize_entities(pool, false).await?;
    }
    let imported = knowledge_import::import_prepared_knowledge(store, prepared)
        .await
        .context(step("Dokumentspeicherung"))?;
    ensure!(imported.complete, "Spielquellenimport ist unvollständig");
    Ok(
        serde_json::json!({"source_id": options.source_id, "commit": pinned.commit(), "documents": inventory.documents, "facts": inventory.facts, "import": imported}),
    )
}

pub async fn refresh_patch_history(
    pool: &sqlx::PgPool,
    raw_dir: &Path,
) -> Result<serde_json::Value> {
    let imported = dbrain_sources::patchnotes_db::pull_patchnotes_with_pool(
        pool,
        raw_dir,
        dbrain_sources::patchnotes_db::PullPatchnotesOptions,
    )
    .await?;
    let parsed = dbrain_normalize::parse_patchnotes(pool, false).await?;
    Ok(serde_json::json!({"import": imported, "parsed": parsed}))
}

pub struct PreparedRefresh {
    pub base: brain_contracts::CorpusRelease,
    pub candidate: brain_contracts::CorpusRelease,
    pub changed_sources: Vec<String>,
}

pub async fn retire_removed_git_profiles(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    _principal: &brain_contracts::Principal,
    current_keys: &BTreeSet<String>,
    owned_sources: &BTreeSet<String>,
    corpus_root: &Path,
) -> Result<usize> {
    use brain_storage::entity_profile::derivation::{GitDocumentReceipt, GIT_DOCUMENT_CONTRACT};
    let rows: Vec<serde_json::Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id='git-game-facts-derived' ORDER BY logical_id")
        .fetch_all(pool).await?;
    let root = corpus_root.canonicalize()?;
    let mut retired = 0;
    for row in rows {
        let mut record: brain_contracts::SourceRecordV2 = serde_json::from_value(row)?;
        if current_keys.contains(&record.logical_id)
            || record
                .metadata
                .get("brain.entity_projection.contract")
                .map(String::as_str)
                != Some(GIT_DOCUMENT_CONTRACT)
        {
            continue;
        }
        let revision = if record.tombstone {
            record.revision - 1
        } else {
            record.revision
        };
        let private: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision as i64).fetch_one(pool).await?;
        let receipt: GitDocumentReceipt = serde_json::from_str(&private)?;
        ensure!(
            receipt.entity_key == record.logical_id
                && receipt.document_sha256 == record.content_hash
                && record.content_hash == crate::digest(record.content.as_bytes())
                && record
                    .metadata
                    .get("brain.entity_projection.receipt_sha256")
                    == Some(&crate::digest(private.as_bytes())),
            "Entfernter Steckbrief widerspricht seiner Quittung"
        );
        if !receipt
            .fact_pins
            .iter()
            .any(|pin| owned_sources.contains(&pin.source_id))
        {
            continue;
        }
        let stored: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision as i64).fetch_one(pool).await?;
        let previous: brain_contracts::SourceRecordV2 = serde_json::from_value(stored)?;
        let mut expected = previous.clone();
        expected.revision = record.revision;
        expected.tombstone = record.tombstone;
        ensure!(
            !previous.tombstone && expected == record,
            "entity_profile_retirement_changed"
        );
        let rendered = retirement_documents(store, pool, config, &previous).await?;
        if retire_git_profile(store, &mut record, &rendered, &root).await? {
            retired += 1;
        }
    }
    Ok(retired)
}

async fn retire_git_profile(
    store: &brain_storage::PgStore,
    record: &mut brain_contracts::SourceRecordV2,
    history: &[crate::entity_profile_render::RenderedEntityProfile],
    root: &Path,
) -> Result<bool> {
    let rendered = history
        .last()
        .ok_or_else(|| anyhow::anyhow!("Gespeicherte Steckbriefhistorie fehlt"))?;
    ensure!(
        record.source_id == "git-game-facts-derived"
            && record.logical_id
                == serde_json::from_str::<serde_json::Value>(&rendered.brain_document)?["entity"]
                    ["entity_key"]
                    .as_str()
                    .unwrap_or("")
            && record.content == rendered.brain_document,
        "Entfernter Steckbrief ist nicht zugeordnet"
    );
    if let Some(path) = owned_retirement_html(root, history)? {
        std::fs::remove_file(path)?;
    }
    if !record.tombstone {
        record.revision = record
            .revision
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("Steckbriefrevision ist ausgeschöpft"))?;
        record.tombstone = true;
        store.apply(record).await?;
        return Ok(true);
    }
    Ok(false)
}

fn owned_retirement_html(
    root: &Path,
    history: &[crate::entity_profile_render::RenderedEntityProfile],
) -> Result<Option<PathBuf>> {
    let rendered = history
        .last()
        .ok_or_else(|| anyhow::anyhow!("Gespeicherte Steckbriefhistorie fehlt"))?;
    ensure!(
        history
            .iter()
            .all(|revision| revision.public_relative_path == rendered.public_relative_path),
        "Historische HTML-Pfade widersprechen der Entität"
    );
    let path = root.join(&rendered.public_relative_path);
    if let Some(parent) = path.parent().filter(|parent| parent.exists()) {
        ensure!(
            parent.canonicalize()? == parent && parent.starts_with(root),
            "Entfernter HTML-Pfad liegt außerhalb des Corpus"
        );
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                let content = std::fs::read(&path)?;
                if history
                    .iter()
                    .any(|revision| content == revision.public_html.as_bytes())
                {
                    return Ok(Some(path));
                }
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(None)
}

pub async fn publish_refreshed_sources(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    base_id: &str,
    sources: &[String],
) -> Result<Option<PreparedRefresh>> {
    use brain_contracts::SourceRecordV2;
    if sources.is_empty() {
        return Ok(None);
    }
    let base = store.snapshot(base_id).await?;
    let rows: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT record_json FROM brain.source_record_heads WHERE source_id=ANY($1) ORDER BY source_id,logical_id",
    )
    .bind(sources)
    .fetch_all(pool)
    .await?;
    let heads: Vec<SourceRecordV2> = rows
        .into_iter()
        .map(serde_json::from_value)
        .collect::<std::result::Result<_, _>>()?;
    let mut pins = base.release.source_revisions.clone();
    for source in sources {
        pins.remove(source);
    }
    for head in &heads {
        let source_pins = pins.entry(head.source_id.clone()).or_default();
        if !head.tombstone {
            source_pins.insert(head.logical_id.clone(), head.revision);
        }
    }
    let changed_sources: Vec<_> = sources
        .iter()
        .filter(|source| base.release.source_revisions.get(*source) != pins.get(*source))
        .cloned()
        .collect();
    if changed_sources.is_empty() {
        return Ok(None);
    }
    let identity = crate::digest(&serde_json::to_vec(&serde_json::json!({
        "base": base.release.release_id, "patch": base.release.patch, "pins": pins
    }))?);
    let (candidate, expected_heads) = brain_storage::PgStore::prepare_imported_release(
        &base,
        sources,
        heads,
        &format!("entity-profiles-{identity}"),
        &format!("entity-profiles-{identity}"),
        chrono::Utc::now().timestamp(),
    )?;
    let candidate = store.imported_release_for_retry(&candidate).await?;
    let records = base
        .revisions
        .into_iter()
        .filter(|record| !sources.contains(&record.source_id))
        .chain(
            expected_heads
                .iter()
                .filter(|record| sources.contains(&record.source_id) && !record.tombstone)
                .cloned(),
        )
        .collect();
    dbrain_retrieval::preflight_release_index(candidate.clone(), records)?;
    let count = store
        .publish_imported_heads_checked(base_id, sources, &candidate, &expected_heads)
        .await?;
    let verified = store.snapshot(&candidate.release_id).await?;
    ensure!(
        verified.release == candidate && count == verified.revisions.len(),
        "entity_profile_release_readback"
    );
    let actual: std::collections::BTreeMap<_, _> = verified
        .heads
        .iter()
        .map(|head| ((&head.source_id, &head.logical_id), head))
        .collect();
    ensure!(
        actual.len()
            == expected_heads
                .iter()
                .filter(|head| candidate
                    .source_revisions
                    .get(&head.source_id)
                    .is_some_and(|pins| pins.contains_key(&head.logical_id)))
                .count()
            && expected_heads
                .iter()
                .filter(|head| candidate
                    .source_revisions
                    .get(&head.source_id)
                    .is_some_and(|pins| pins.contains_key(&head.logical_id)))
                .all(
                    |head| actual.get(&(&head.source_id, &head.logical_id)).copied() == Some(head)
                ),
        "entity_profile_heads_changed"
    );
    Ok(Some(PreparedRefresh {
        base: base.release,
        candidate,
        changed_sources,
    }))
}

pub async fn activate_refreshed_sources(
    runtime: &super::runtime_config::RuntimeConfig,
    prepared: &PreparedRefresh,
) -> Result<()> {
    activate_refreshed_sources_with_expected_config(runtime, prepared, None).await
}

pub async fn activate_refreshed_sources_with_expected_config(
    runtime: &super::runtime_config::RuntimeConfig,
    prepared: &PreparedRefresh,
    expected_config_sha256: Option<&str>,
) -> Result<()> {
    let runtime_path = runtime
        .loaded_from
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("entity_profile_runtime_path"))?;
    let executable = std::env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("entity_profile_binary_directory"))?
        .join("brain-candidate-activate");
    let serve_bytes = super::runtime_config::read_bounded(&runtime.serve_config, 65536)?;
    if let Some(expected) = expected_config_sha256 {
        ensure!(
            crate::digest(&serve_bytes) == expected,
            "entity_profile_resume_config_changed"
        );
    }
    let mut args = vec![
        "--config".into(),
        runtime_path.to_string_lossy().into_owned(),
        "--target".into(),
        "entity-profiles".into(),
        "--base-release-id".into(),
        prepared.base.release_id.clone(),
        "--base-release-sha256".into(),
        crate::digest(&serde_json::to_vec(&prepared.base)?),
        "--candidate-release-id".into(),
        prepared.candidate.release_id.clone(),
        "--candidate-release-sha256".into(),
        crate::digest(&serde_json::to_vec(&prepared.candidate)?),
        "--expected-serve-config-sha256".into(),
        crate::digest(&serve_bytes),
        "--apply".into(),
        "--maintenance-credential-stdin".into(),
    ];
    for source in &prepared.changed_sources {
        args.extend(["--allow-source".into(), source.clone()]);
    }
    let credential = std::fs::File::open("/proc/self/fd/5")?;
    let output = crate::process::run_with_credential(
        &executable,
        &args,
        runtime.artifact_dir.as_path(),
        credential,
        180_000 + runtime.health_timeout_ms * 3,
        65536,
    )
    .await?;
    let report: serde_json::Value = serde_json::from_slice(&output)?;
    ensure!(
        report["success"] == true
            && report["readiness_verified"] == true
            && report["status"] == "active_candidate"
            && report["candidate_release_id"] == prepared.candidate.release_id,
        "entity_profile_activation_unverified"
    );
    let serve = brain_serve::Config::load(&runtime.serve_config)?;
    let active = super::activation::ActivationTarget::Standard.release(&serve)?;
    ensure!(
        active.id == prepared.candidate.release_id
            && active.knowledge_version == prepared.candidate.knowledge_version,
        "entity_profile_activation_changed"
    );
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedEntityProfile {
    pub entity_key: String,
    pub profile_sha256: String,
    pub brain_document_ref: String,
    pub public_html_ref: String,
    pub public_relative_path: PathBuf,
}

pub async fn persist_verified_git_profile(
    store: &brain_storage::PgStore,
    verified: &dbrain_sources::entity_binding::derivation::VerifiedGitProfile,
) -> Result<brain_contracts::SourceRecordV2> {
    let (record, receipt) =
        entity_profile_document(verified.profile(), verified.receipt(), verified.policy())?;
    Ok(store.persist_entity_document(record, &receipt).await?)
}

pub fn registered_git_repositories(
    config: &crate::config::MaintenanceConfig,
    snapshot: &brain_contracts::CorpusSnapshot,
    principal: &brain_contracts::Principal,
) -> Result<
    std::collections::BTreeMap<(String, String), dbrain_sources::git_source::PinnedRepository>,
> {
    use brain_storage::entity_profile::derivation::git_document_identity;
    let mut repositories = std::collections::BTreeMap::new();
    for record in snapshot.authorized(principal, false)? {
        let Some(encoded) = record
            .metadata
            .get(brain_storage::source_versions::DOCUMENT_METADATA_KEY)
        else {
            continue;
        };
        let document: serde_json::Value = serde_json::from_str(encoded)?;
        if document["source_kind"] != "game_file" {
            continue;
        }
        let (commit, origin, path) = git_document_identity(&record)?;
        let matching: Vec<_> = config
            .game_sources
            .iter()
            .filter(|source| {
                source.origin.trim_end_matches(".git") == origin.trim_end_matches(".git")
            })
            .collect();
        ensure!(
            matching.len() == 1,
            "Originalrepository ist nicht eindeutig registriert"
        );
        let source = matching[0];
        crate::config::require_registered_game_source(config, source)?;
        crate::config::safe_relative(&path)?;
        ensure!(
            source
                .source_paths
                .iter()
                .any(|scope| path == *scope || path.starts_with(&format!("{scope}/"))),
            "Originalblob liegt außerhalb des registrierten Umfangs"
        );
        let key = (record.source_id, commit.clone());
        if let std::collections::btree_map::Entry::Vacant(entry) = repositories.entry(key) {
            let pinned = dbrain_sources::git_source::PinnedRepository::open(&source.path, &commit)?;
            pinned.require_origin(&[&source.origin])?;
            entry.insert(pinned);
        }
    }
    Ok(repositories)
}

pub async fn verify_retired_git_documents(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    _principal: &brain_contracts::Principal,
    candidate: &brain_contracts::CorpusSnapshot,
    corpus_root: &Path,
) -> Result<()> {
    use brain_storage::entity_profile::derivation::derived_policy;
    let rows: Vec<serde_json::Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id='git-game-facts-derived' ORDER BY logical_id")
        .fetch_all(pool).await?;
    let heads: Vec<brain_contracts::SourceRecordV2> = rows
        .into_iter()
        .map(serde_json::from_value)
        .collect::<std::result::Result<_, _>>()?;
    ensure!(!heads.is_empty(), "entity_profile_document_empty");
    let root = corpus_root.canonicalize()?;
    let pins = candidate
        .release
        .source_revisions
        .get("git-game-facts-derived");
    for head in heads {
        if !head.tombstone {
            ensure!(
                pins.and_then(|pins| pins.get(&head.logical_id)) == Some(&head.revision),
                "entity_profile_retirement_head"
            );
            continue;
        }
        ensure!(
            pins.is_none_or(|pins| !pins.contains_key(&head.logical_id)),
            "entity_profile_retirement_pins"
        );
        ensure!(
            head.tombstone && head.revision > 1,
            "entity_profile_retirement_head"
        );
        let previous: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&head.source_id).bind(&head.logical_id).bind((head.revision - 1) as i64)
            .fetch_one(pool).await?;
        let previous: brain_contracts::SourceRecordV2 = serde_json::from_value(previous)?;
        let mut expected = previous.clone();
        expected.revision = head.revision;
        expected.tombstone = true;
        ensure!(
            !previous.tombstone && expected == head,
            "entity_profile_retirement_changed"
        );
        let origin = brain_contracts::source::origin_from_record(&previous)
            .map_err(|_| anyhow::anyhow!("entity_profile_document_rights"))?;
        ensure!(
            origin.policy == derived_policy(),
            "entity_profile_document_rights"
        );
        let history = retirement_documents(store, pool, config, &previous).await?;
        ensure!(
            owned_retirement_html(&root, &history)?.is_none(),
            "Eigene zurückgezogene HTML-Seite ist noch veröffentlicht"
        );
    }
    Ok(())
}

pub async fn verify_stored_git_document(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    principal: &brain_contracts::Principal,
    record: &brain_contracts::SourceRecordV2,
    release: &brain_contracts::CorpusRelease,
) -> Result<()> {
    verify_git_document(store, pool, config, principal, record, release).await
}

async fn retirement_documents(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    record: &brain_contracts::SourceRecordV2,
) -> Result<Vec<crate::entity_profile_render::RenderedEntityProfile>> {
    let rows: Vec<serde_json::Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision<=$3 ORDER BY revision")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
        .fetch_all(pool).await?;
    let mut history = Vec::new();
    for row in rows {
        let revision: brain_contracts::SourceRecordV2 = serde_json::from_value(row)?;
        if !revision.tombstone {
            history.push(retirement_document(store, pool, config, &revision).await?);
        }
    }
    ensure!(
        history
            .last()
            .is_some_and(|rendered| rendered.brain_document == record.content),
        "entity_profile_retirement_changed"
    );
    Ok(history)
}

async fn retirement_document(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    record: &brain_contracts::SourceRecordV2,
) -> Result<crate::entity_profile_render::RenderedEntityProfile> {
    use brain_storage::entity_profile::derivation::{
        derived_policy, git_document_identity, GitDocumentReceipt, GIT_DOCUMENT_CONTRACT,
    };
    record.validate()?;
    let stored: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_one(pool).await?;
    ensure!(
        serde_json::from_value::<brain_contracts::SourceRecordV2>(stored)? == *record,
        "entity_profile_retirement_changed"
    );
    let private: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_one(pool).await?;
    let receipt: GitDocumentReceipt = serde_json::from_str(&private)?;
    let origin = brain_contracts::source::origin_from_record(record).map_err(anyhow::Error::msg)?;
    ensure!(
        record.source_id == "git-game-facts-derived"
            && !record.tombstone
            && receipt.contract_version == GIT_DOCUMENT_CONTRACT
            && receipt.entity_key == record.logical_id
            && !receipt.fact_pins.is_empty()
            && receipt.document_sha256 == record.content_hash
            && origin.raw_sha256 == record.content_hash
            && origin.policy == derived_policy()
            && record.content_hash == crate::digest(record.content.as_bytes())
            && record
                .metadata
                .get("brain.entity_projection.contract")
                .map(String::as_str)
                == Some(GIT_DOCUMENT_CONTRACT)
            && record
                .metadata
                .get("brain.entity_projection.receipt_sha256")
                == Some(&crate::digest(private.as_bytes())),
        "entity_profile_retirement_receipt"
    );
    let snapshot = store.snapshot(&receipt.original_release_id).await?;
    let mut origins = std::collections::BTreeMap::new();
    for pin in &receipt.fact_pins {
        let original = snapshot
            .revisions
            .iter()
            .find(|original| {
                original.source_id == pin.source_id
                    && original.logical_id == pin.logical_id
                    && original.revision == pin.store_revision
            })
            .ok_or_else(|| anyhow::anyhow!("Unveränderlicher Originalpin fehlt"))?;
        ensure!(
            !original.tombstone
                && snapshot
                    .heads
                    .iter()
                    .any(|head| head.source_id == pin.source_id
                        && head.logical_id == pin.logical_id
                        && head.revision >= pin.store_revision),
            "Originalhead fehlt"
        );
        let (commit, url, path) = git_document_identity(original)?;
        let original_origin =
            brain_contracts::source::origin_from_record(original).map_err(anyhow::Error::msg)?;
        ensure!(
            commit == pin.git_commit
                && url == pin.repository_url
                && original_origin.raw_sha256 == pin.raw_sha256
                && pin.binding_identity.entity_key == receipt.entity_key,
            "entity_profile_retirement_original"
        );
        let matching: Vec<_> = config
            .game_sources
            .iter()
            .filter(|source| source.origin.trim_end_matches(".git") == url.trim_end_matches(".git"))
            .collect();
        ensure!(
            matching.len() == 1,
            "Originalrepository ist nicht eindeutig registriert"
        );
        crate::config::require_registered_game_source(config, matching[0])?;
        crate::config::safe_relative(&path)?;
        ensure!(
            matching[0]
                .source_paths
                .iter()
                .any(|scope| path == *scope || path.starts_with(&format!("{scope}/"))),
            "Originalblob liegt außerhalb des registrierten Umfangs"
        );
        origins.insert(
            (pin.source_id.clone(), pin.logical_id.clone(), commit),
            original_origin,
        );
    }
    let mut compact: serde_json::Value = serde_json::from_str(&record.content)?;
    let sources = compact["sources"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Gespeicherte Quellen fehlen"))?
        .clone();
    for section in ["facts", "context"] {
        for fact in compact[section]
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("Gespeicherte Fakten fehlen"))?
        {
            let source = fact["source_ref"]
                .as_u64()
                .and_then(|index| sources.get(index as usize))
                .ok_or_else(|| anyhow::anyhow!("Gespeicherter Quellenverweis fehlt"))?;
            let key = (
                source["source_id"].as_str().unwrap_or("").to_owned(),
                fact["logical_id"].as_str().unwrap_or("").to_owned(),
                source["original_revision"]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
            );
            let mut original_origin = origins
                .iter()
                .find(|((_, _, commit), _)| commit == &key.2)
                .map(|(_, origin)| origin.clone())
                .ok_or_else(|| anyhow::anyhow!("Gespeicherter Fakt hat keinen Originalpin"))?;
            original_origin.identity.source_id = "git-game-facts-derived".into();
            original_origin.identity.logical_id = record.logical_id.clone();
            original_origin.source_revision = brain_contracts::source::SourceRevision::Git {
                commit: key.2.clone(),
            };
            original_origin.policy = derived_policy();
            original_origin.locator = "git-game-facts-derived".into();
            original_origin.origin_artifacts.clear();
            ensure!(
                key.0 == "git-game-facts-derived"
                    && key.1 == record.logical_id
                    && fact["policy"] == serde_json::to_value(&original_origin.policy)?
                    && fact["locator"].as_str() == Some(original_origin.locator.as_str()),
                "entity_profile_retirement_original"
            );
            fact["provenance"] = serde_json::json!({"source_kind":fact["source_kind"], "origin":original_origin,
                "original_revision":key.2, "observed_at":fact["observed_at"], "source_span":fact["source_span"],
                "license":fact["license"], "document_metadata":{}});
        }
    }
    compact["patch_story"] = serde_json::to_value(stored_patch_story(record)?)?;
    let profile: EntityProfile = serde_json::from_value(compact)?;
    ensure!(
        profile.entity.entity_key == record.logical_id,
        "entity_profile_retirement_entity"
    );
    let rendered = crate::entity_profile_render::render_entity_profile(&profile)?;
    ensure!(
        rendered.brain_document == record.content,
        "entity_profile_retirement_document"
    );
    Ok(rendered)
}

fn stored_patch_story(
    record: &brain_contracts::SourceRecordV2,
) -> Result<Vec<brain_contracts::entity_profile::PatchStoryChange>> {
    let compact: serde_json::Value = serde_json::from_str(&record.content)?;
    Ok(compact["patch_story"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Gespeicherte Patchbelege fehlen"))?
        .iter()
        .map(|change| {
            let mut change = change.clone();
            change["additional_fields"] = change["conditions"].take();
            change["original_line"] =
                serde_json::json!({"text":null,"redistribution_allowed":false});
            serde_json::from_value(change)
        })
        .collect::<std::result::Result<_, _>>()?)
}

async fn verify_git_document(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    config: &crate::config::MaintenanceConfig,
    principal: &brain_contracts::Principal,
    record: &brain_contracts::SourceRecordV2,
    release: &brain_contracts::CorpusRelease,
) -> Result<()> {
    use brain_storage::entity_profile::derivation::verify_git_document_receipt;
    let private: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_one(pool).await?;
    let receipt: brain_storage::entity_profile::derivation::GitDocumentReceipt =
        serde_json::from_str(&private)?;
    let snapshot = store.snapshot(&receipt.original_release_id).await?;
    let repositories = registered_git_repositories(config, &snapshot, principal)?;
    let verified = dbrain_sources::entity_binding::derivation::derive_git_entity_profile(
        store,
        &receipt.original_release_id,
        principal,
        &receipt.entity_key,
        &repositories,
    )
    .await?;
    let mut blobs = Vec::new();
    let mut seen = BTreeSet::new();
    for binding in verified.original_pins() {
        ensure!(
            release
                .source_revisions
                .get(&binding.source_id)
                .and_then(|pins| pins.get(&binding.logical_id))
                == Some(&binding.store_revision),
            "Dokumentoriginal gehört nicht zum Kandidatenstand"
        );
        if !seen.insert((
            &binding.source_id,
            &binding.logical_id,
            binding.store_revision,
        )) {
            continue;
        }
        let original = snapshot
            .revisions
            .iter()
            .find(|original| {
                original.source_id == binding.source_id
                    && original.logical_id == binding.logical_id
                    && original.revision == binding.store_revision
            })
            .ok_or_else(|| anyhow::anyhow!("Unveränderlicher Originalpin fehlt"))?;
        let (commit, origin, path) =
            brain_storage::entity_profile::derivation::git_document_identity(original)?;
        let pinned = repositories
            .get(&(binding.source_id.clone(), commit.clone()))
            .ok_or_else(|| anyhow::anyhow!("Gepinnter Repositoryzugang fehlt"))?;
        blobs.push(brain_storage::entity_profile::derivation::GitBlobEvidence {
            source_id: binding.source_id.clone(),
            logical_id: binding.logical_id.clone(),
            store_revision: binding.store_revision,
            git_commit: commit,
            repository_url: origin,
            bytes: pinned.read_blob(&path)?,
        });
    }
    let mut identity = verified
        .original_pins()
        .first()
        .ok_or_else(|| anyhow::anyhow!("Originalbindung fehlt"))?
        .binding_identity
        .clone();
    for binding in verified.original_pins() {
        for alias in &binding.binding_identity.aliases {
            if !identity.aliases.contains(alias) {
                identity.aliases.push(alias.clone());
            }
        }
    }
    let story = store.entity_patch_story(&identity).await?;
    let current = store.snapshot(&receipt.original_release_id).await?;
    verify_git_document_receipt(
        record,
        &receipt,
        &current,
        principal,
        verified.original_pins(),
        &blobs,
        &story,
    )?;
    Ok(())
}

fn entity_profile_document(
    profile: &EntityProfile,
    receipt: &brain_storage::entity_profile::derivation::GitDocumentReceipt,
    policy: brain_contracts::source::SourcePolicy,
) -> Result<(brain_contracts::SourceRecordV2, String)> {
    use brain_contracts::{
        source::{GameValidity, OriginArtifact, SourceIdentity, SourceRevision},
        value::{Observed, UnknownReason},
        SourceRecordV2,
    };
    use brain_storage::entity_profile::derivation::{derived_policy, GIT_DOCUMENT_CONTRACT};
    let rendered = crate::entity_profile_render::render_entity_profile(profile)?;
    let hash = crate::digest(rendered.brain_document.as_bytes());
    ensure!(
        receipt.contract_version == GIT_DOCUMENT_CONTRACT
            && receipt.entity_key == profile.entity.entity_key
            && receipt.document_sha256 == hash
            && policy == derived_policy(),
        "Geprüftes Profil widerspricht seinem Dokumentvertrag"
    );
    let receipt_json = serde_json::to_string(receipt)?;
    let mut record = SourceRecordV2 {
        source_id: "git-game-facts-derived".into(),
        logical_id: profile.entity.entity_key.clone(),
        revision: 1,
        content_hash: hash.clone(),
        content: rendered.brain_document,
        visibility: policy.visibility,
        allowed_scopes: policy.allowed_scopes.clone(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: std::collections::BTreeMap::from([
            (
                "brain.entity_projection.contract".into(),
                GIT_DOCUMENT_CONTRACT.into(),
            ),
            (
                "brain.entity_projection.receipt_sha256".into(),
                crate::digest(receipt_json.as_bytes()),
            ),
        ]),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: GIT_DOCUMENT_CONTRACT.into(),
            original_revision: Some(hash.clone()),
        },
        raw_sha256: hash,
        locator: "git-game-facts-derived".into(),
        parser_revision: profile.contract_version.clone(),
        parser_family: "entity-profile-render".into(),
        schema_version: Observed::known(profile.contract_version.clone()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: Default::default(),
        derivation_family: Observed::known(GIT_DOCUMENT_CONTRACT.into()),
        policy,
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .map_err(|_| anyhow::anyhow!("Abgeleitete Dokumentherkunft ist ungültig"))?;
    Ok((record, receipt_json))
}

pub fn render_and_export_profiles(
    artifacts: &Artifacts,
    profiles: &[EntityProfile],
    corpus_root: &Path,
) -> Result<Vec<PreparedEntityProfile>> {
    let corpus_root = corpus_root.canonicalize()?;
    let prepared = stage_profiles(artifacts, profiles, |profile| {
        let rendered = crate::entity_profile_render::render_entity_profile(profile)?;
        Ok((
            rendered.brain_document,
            rendered.public_html,
            rendered.public_relative_path,
        ))
    })?;
    for (profile, expected) in profiles.iter().zip(&prepared) {
        let path = crate::entity_profile_render::write_public_html(&corpus_root, profile)?;
        ensure!(
            path == corpus_root.join(&expected.public_relative_path),
            "Exportpfad widerspricht dem vorbereiteten Steckbrief"
        );
        ensure!(
            std::fs::read(path)? == artifacts.read(&expected.public_html_ref)?,
            "HTML-Export widerspricht dem vorbereiteten Steckbrief"
        );
    }
    Ok(prepared)
}

pub fn stage_profiles<F>(
    artifacts: &Artifacts,
    profiles: &[EntityProfile],
    mut render: F,
) -> Result<Vec<PreparedEntityProfile>>
where
    F: FnMut(&EntityProfile) -> Result<(String, String, PathBuf)>,
{
    let mut identities = BTreeSet::new();
    let mut prepared = Vec::new();
    for profile in profiles {
        ensure!(
            identities.insert(&profile.entity.entity_key),
            "Entität ist im Wartungslauf mehrfach vorhanden"
        );
        let profile_sha256 = crate::digest(&serde_json::to_vec(profile)?);
        let (document, html, path) = render(profile)?;
        ensure!(
            !path.as_os_str().is_empty()
                && path
                    .components()
                    .all(|part| matches!(part, Component::Normal(_))),
            "Steckbriefpfad muss relativ sein"
        );
        ensure!(
            path.starts_with("site/entities")
                && path
                    .extension()
                    .is_some_and(|extension| extension == "html"),
            "Steckbriefpfad liegt außerhalb der vorhandenen Site"
        );
        prepared.push(PreparedEntityProfile {
            entity_key: profile.entity.entity_key.clone(),
            profile_sha256,
            brain_document_ref: artifacts.put(document.as_bytes(), "json")?,
            public_html_ref: artifacts.put(html.as_bytes(), "html")?,
            public_relative_path: path,
        });
    }
    Ok(prepared)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::entity_profile::{EntityIdentity, EntityKind, ENTITY_PROFILE_VERSION};

    struct ScratchPg {
        directory: tempfile::TempDir,
    }

    impl ScratchPg {
        fn start() -> Self {
            let directory = tempfile::tempdir().unwrap();
            let instance = Self { directory };
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
            let options = format!("-k {} -p 55441 -c listen_addresses=''", socket.display());
            assert!(
                std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                    .arg("-D")
                    .arg(&data)
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

    #[tokio::test]
    async fn import_validation_failure_is_visible_without_raw_values() {
        let directory = tempfile::tempdir().unwrap();
        let git = |args: &[&str]| {
            let output = std::process::Command::new("/usr/bin/git")
                .current_dir(directory.path())
                .args([
                    "-c",
                    "core.hooksPath=/dev/null",
                    "-c",
                    "user.name=Prüfung",
                    "-c",
                    "user.email=test@example.invalid",
                ])
                .args(args)
                .output()
                .unwrap();
            assert!(output.status.success());
            String::from_utf8(output.stdout).unwrap().trim().to_owned()
        };
        git(&["init", "--quiet"]);
        std::fs::write(directory.path().join("stats.json"), br#"{"health":550}"#).unwrap();
        git(&["add", "stats.json"]);
        git(&["commit", "--quiet", "-m", "Prüfbeleg"]);
        let commit = git(&["rev-parse", "HEAD"]);
        let source = super::super::runtime_config::EntityProfileSource {
            repository_id: "fixture".into(),
            paths: BTreeSet::from(["stats.json".into()]),
            extraction: dbrain_sources::game_files::GameFileOptions {
                root: directory.path().to_owned(),
                app_id: 1422450,
                source_id: "fixture".into(),
                observed_at: "2026-10-05T03:00:00Z".into(),
                build_id: None,
                manifest_id: None,
                source_revision: None,
                depot_id: None,
                language: "privater\nWert".into(),
                attribution: "Prüfbeleg".into(),
                license_name: "unverified".into(),
                license_url: None,
                provenance: serde_json::json!({}),
                max_file_bytes: 8388608,
            },
            import_policy: directory.path().join("unbenutzt.json"),
            canonical_raw_dir: None,
        };
        let pinned =
            dbrain_sources::git_source::PinnedRepository::open(directory.path(), &commit).unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy_with(sqlx::postgres::PgConnectOptions::new_without_pgpass());
        let store = brain_storage::PgStore::new(pool.clone());
        let error = refresh_git_knowledge(&source, &pinned, directory.path(), &store, &pool)
            .await
            .unwrap_err();
        let status = refresh_failure(&error);
        assert_eq!(status["code"], "ENTITY_PROFILE_REFRESH_FAILED");
        assert_eq!(status["source_id"], "fixture");
        assert_eq!(status["commit"], commit);
        assert_eq!(status["step"], "JSONL-Validierung");
        assert_eq!(status["document_path"], "stats.json");
        assert_eq!(status["validation"][0]["line"], 1);
        assert_eq!(
            status["validation"][0]["cause"],
            "Nichtleerer Text ohne Steuerzeichen erforderlich"
        );
        assert!(!status.to_string().contains("privater"));
        let database_error =
            anyhow::Error::new(sqlx::Error::Protocol("privater SQL-Nutzwert".into()));
        assert!(!refresh_failure(&database_error)
            .to_string()
            .contains("Nutzwert"));
        pool.close().await;
    }

    #[test]
    fn refresh_failure_preserves_safe_nested_port_classes() {
        for (port, class, sqlstate) in [
            (
                brain_contracts::PortError::Unavailable("postgres_query_cancelled".into()),
                "query_cancelled",
                Some("57014"),
            ),
            (
                brain_contracts::PortError::Unavailable("postgres_lock_not_available".into()),
                "lock_not_available",
                Some("55P03"),
            ),
            (
                brain_contracts::PortError::InvalidResponse(
                    "SELECT privater_Nutzwert FROM geheime_Daten".into(),
                ),
                "invalid_response",
                None,
            ),
            (
                brain_contracts::PortError::Unavailable("privater_Nutzwert".into()),
                "unavailable",
                None,
            ),
        ] {
            let error = anyhow::Error::new(port).context("privater_Nutzwert im Kontext");
            let status = refresh_failure(&error);
            assert_eq!(status["error_class"], class);
            assert_eq!(
                status.get("sqlstate").and_then(serde_json::Value::as_str),
                sqlstate
            );
            assert!(!status.to_string().contains("privater_Nutzwert"));
            assert!(!status.to_string().contains("SELECT"));
        }
        let error = anyhow::Error::new(dbrain_sources::SourcesError::EntityDerivation {
            entity_key: "brain.entities:270510".into(),
            step: "patch_story",
            source: brain_storage::StorageError::Database(sqlx::Error::Protocol(
                "SELECT privater_Nutzwert FROM geheime_Daten".into(),
            )),
        })
        .context("privater_Nutzwert");
        let status = refresh_failure(&error);
        assert_eq!(status["entity_key"], "brain.entities:270510");
        assert_eq!(status["step"], "patch_story");
        assert_eq!(status["error_class"], "database_error");
        assert!(!status.to_string().contains("privater_Nutzwert"));
        assert!(!status.to_string().contains("SELECT"));
    }

    #[tokio::test]
    async fn derivation_timeout_keeps_safe_sqlstate_and_step() {
        let pg = ScratchPg::start();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55441)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL statement_timeout='1ms'")
            .execute(&mut *tx)
            .await
            .unwrap();
        let database = sqlx::query("SELECT pg_sleep(0.02)")
            .execute(&mut *tx)
            .await
            .unwrap_err();
        let error = anyhow::Error::new(dbrain_sources::SourcesError::EntityDerivation {
            entity_key: "brain.entities:270510".into(),
            step: "patch_story",
            source: brain_storage::StorageError::Database(database),
        })
        .context("privater_Nutzwert");
        let status = refresh_failure(&error);
        assert_eq!(status["step"], "patch_story");
        assert_eq!(status["error_class"], "query_cancelled");
        assert_eq!(status["sqlstate"], "57014");
        assert!(!status.to_string().contains("privater_Nutzwert"));
        assert!(!status.to_string().contains("pg_sleep"));
        tx.rollback().await.unwrap();
        pool.close().await;
    }

    #[tokio::test]
    #[ignore = "Lokaler Beleg mit den vorgegebenen Originalpins"]
    async fn pinned_original_import() {
        let pg = ScratchPg::start();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55441)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let store = brain_storage::PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(
            &std::fs::read("/etc/deadlock-brain/maintenance-runtime.json").unwrap(),
        )
        .unwrap();
        for (index, commit) in [
            (1, "e0b9830a7f69726c933da90446e669ad276e73f8"),
            (0, "46c3fd0cfbf2108f48123e1dddd59e416db1b7ee"),
        ] {
            let mut source: super::super::runtime_config::EntityProfileSource =
                serde_json::from_value(value["entity_profile_sources"][index].clone()).unwrap();
            source.canonical_raw_dir = None;
            let repo = source.extraction.root.clone();
            let pinned = dbrain_sources::git_source::PinnedRepository::open(&repo, commit).unwrap();
            let result = refresh_git_knowledge(&source, &pinned, &repo, &store, &pool).await;
            match result {
                Ok(status) => eprintln!(
                    "commit={} documents={} facts={}",
                    status["commit"], status["documents"], status["facts"]
                ),
                Err(error) => panic!("{}", refresh_failure(&error)),
            }
            let revisions: Vec<String> = sqlx::query_scalar("SELECT record_json->'metadata'->>'wiki-spielwissen.original_revision' FROM brain.source_record_heads WHERE source_id=$1")
                .bind(&source.extraction.source_id).fetch_all(&pool).await.unwrap();
            assert!(!revisions.is_empty());
            assert!(revisions.iter().all(|revision| revision == commit));
        }
        pool.close().await;
    }

    #[tokio::test]
    async fn existing_patch_import_and_parser_repeat_without_deleting_history() {
        let pg = ScratchPg::start();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55441)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let owner = brain_storage::PgStore::new(pool.clone());
        owner.migrate_core().await.unwrap();
        sqlx::raw_sql("CREATE SCHEMA patchnotes; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN;")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("../../../dbrain-sources/src/wiki_scratch.sql"))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql("CREATE TABLE brain.entity_snapshots(id bigserial PRIMARY KEY,legacy_sqlite_id bigint,source text NOT NULL,entity_type text NOT NULL,external_id text NOT NULL,canonical_name text,payload_hash text NOT NULL,payload jsonb NOT NULL,fetched_at timestamptz NOT NULL,source_document_id bigint REFERENCES brain.source_documents(id),UNIQUE(source,entity_type,external_id,payload_hash));
            CREATE TABLE brain.entities(id bigserial PRIMARY KEY,entity_type text NOT NULL,canonical_name text NOT NULL);
            CREATE TABLE brain.entity_aliases(entity_id bigint,alias text);
            CREATE TABLE brain.patch_events(id bigserial PRIMARY KEY,patch_snapshot_id bigint,legacy_patch_snapshot_id bigint,patch_external_id text,patch_title text,patch_url text,source_kind text,posted_at timestamptz,line_index bigint,section text,entity_type text,entity_name text,subject text,change_type text,raw_line text,normalized_line text,old_value text,new_value text,confidence double precision,metadata jsonb,event_hash text UNIQUE,created_at timestamptz);
            CREATE VIEW brain.patch_changes AS SELECT patch_title,posted_at::date AS patch_date,entity_type,entity_name,NULL::text AS ability_name,NULL::text AS stat_name,old_value,new_value,change_type,NULL::text AS numeric_direction,raw_line,patch_url,confidence FROM brain.patch_events;
            CREATE TABLE patchnotes.changelog_posts(id bigint PRIMARY KEY,title text,url text,posted_at timestamptz,raw_content text,translated_content text);
            INSERT INTO brain.entities(entity_type,canonical_name) VALUES('hero','Warden');
            INSERT INTO patchnotes.changelog_posts VALUES(1,'09-16-2026 Update','https://forums.playdeadlock.com/threads/fixture1','2026-09-16T12:00:00Z','Warden\n- Health increased from 500 to 550',NULL);")
            .execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!(
            "../../../../../scripts/migrations/2026-10-04-brain-entity-derived-receipts-v1.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        for _ in 0..2 {
            let grants = std::process::Command::new("/usr/lib/postgresql/16/bin/psql")
                .arg("-X")
                .arg("-h")
                .arg(pg.directory.path().join("socket"))
                .args(["-p", "55441", "-U", "brain_core_test", "-d", "postgres"])
                .args(["-v", "db=postgres", "-f"])
                .arg(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../../ops/brain-postgres/grants.sql"
                ))
                .output()
                .unwrap();
            assert!(
                grants.status.success(),
                "{}",
                String::from_utf8_lossy(&grants.stderr)
            );
        }
        let ingest_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55441)
                    .username("brain_ingest")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let role: String = sqlx::query_scalar("SELECT current_user::text")
            .fetch_one(&ingest_pool)
            .await
            .unwrap();
        assert_eq!(role, "brain_ingest");
        let raw = pg.directory.path().join("raw");
        let initial = refresh_patch_history(&ingest_pool, &raw).await.unwrap();
        assert!(initial["parsed"]["events_inserted"].as_i64().unwrap() > 0);
        assert_eq!(initial["parsed"]["deleted_before_parse"], 0);
        let first: Vec<(i64, String)> =
            sqlx::query_as("SELECT id,event_hash FROM brain.patch_events ORDER BY id")
                .fetch_all(&pool)
                .await
                .unwrap();
        let repeated = refresh_patch_history(&ingest_pool, &raw).await.unwrap();
        assert_eq!(repeated["parsed"]["events_inserted"], 0);
        assert_eq!(
            sqlx::query_as::<_, (i64, String)>(
                "SELECT id,event_hash FROM brain.patch_events ORDER BY id"
            )
            .fetch_all(&pool)
            .await
            .unwrap(),
            first
        );
        sqlx::query("INSERT INTO patchnotes.changelog_posts VALUES(2,'09-30-2026 Update','https://forums.playdeadlock.com/threads/fixture2','2026-09-30T12:00:00Z','Warden\n- Health increased from 550 to 600',NULL)")
            .execute(&pool).await.unwrap();
        let updated = refresh_patch_history(&ingest_pool, &raw).await.unwrap();
        assert!(updated["parsed"]["events_inserted"].as_i64().unwrap() > 0);
        assert_eq!(updated["parsed"]["deleted_before_parse"], 0);
        let historical: Vec<(i64,String)> = sqlx::query_as("SELECT id,event_hash FROM brain.patch_events WHERE posted_at::date='2026-09-16' ORDER BY id")
            .fetch_all(&pool).await.unwrap();
        assert_eq!(historical, first);
        assert_eq!(
            refresh_patch_history(&ingest_pool, &raw).await.unwrap()["parsed"]["events_inserted"],
            0
        );
        let event: (String, String, String) = sqlx::query_as("SELECT entity_name,old_value,new_value FROM brain.patch_events WHERE posted_at::date='2026-09-30'")
            .fetch_one(&ingest_pool).await.unwrap();
        assert_eq!(event, ("Warden".into(), "550".into(), "600".into()));
        let ingest = brain_storage::PgStore::new(ingest_pool.clone());
        let entity = EntityIdentity {
            entity_key: "hero:Warden".into(),
            kind: EntityKind::Hero,
            name: "Warden".into(),
            aliases: Vec::new(),
            identity_evidence: vec!["fixture".into()],
        };
        let story = ingest.entity_patch_story(&entity).await.unwrap();
        assert_eq!(story.len(), 2);
        assert_eq!(story[1].new_value, serde_json::json!("600"));
        let profile = EntityProfile {
            contract_version: ENTITY_PROFILE_VERSION.into(),
            entity,
            patch: Some("2026-09-30".into()),
            source_state: Vec::new(),
            facts: Vec::new(),
            context: Vec::new(),
            conflicts: Vec::new(),
            patch_story: story,
            unknowns: Vec::new(),
        };
        let rendered = crate::entity_profile_render::render_entity_profile(&profile).unwrap();
        use brain_storage::entity_profile::derivation::{
            derived_policy, GitDocumentReceipt, GIT_DOCUMENT_CONTRACT,
        };
        let receipt = GitDocumentReceipt {
            contract_version: GIT_DOCUMENT_CONTRACT.into(),
            entity_key: profile.entity.entity_key.clone(),
            original_release_id: "fixture-release".into(),
            document_sha256: crate::digest(rendered.brain_document.as_bytes()),
            fact_pins: Vec::new(),
        };
        let (record, private) =
            entity_profile_document(&profile, &receipt, derived_policy()).unwrap();
        let stored = ingest
            .persist_entity_document(record, &private)
            .await
            .unwrap();
        assert_eq!(stored.content, rendered.brain_document);
        assert!(stored.content.contains("600"));
        for table in ["source_documents", "entity_snapshots", "patch_events"] {
            for operation in ["UPDATE", "DELETE"] {
                let allowed: bool =
                    sqlx::query_scalar("SELECT has_table_privilege(current_user,$1,$2)")
                        .bind(format!("brain.{table}"))
                        .bind(operation)
                        .fetch_one(&ingest_pool)
                        .await
                        .unwrap();
                assert!(!allowed);
            }
            assert!(sqlx::query(&format!("UPDATE brain.{table} SET id=id"))
                .execute(&ingest_pool)
                .await
                .is_err());
            assert!(sqlx::query(&format!("DELETE FROM brain.{table}"))
                .execute(&ingest_pool)
                .await
                .is_err());
        }
        let completed: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM brain.source_runs WHERE status='ok' AND finished_at IS NOT NULL",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(completed, 4);
        assert!(sqlx::query("UPDATE brain.source_runs SET source='fremd'")
            .execute(&ingest_pool)
            .await
            .is_err());
        ingest_pool.close().await;
        pool.close().await;
    }

    #[tokio::test]
    async fn compact_renderer_document_storage_is_atomic_and_idempotent() {
        use brain_storage::entity_profile::derivation::{
            derived_policy, GitDocumentReceipt, GIT_DOCUMENT_CONTRACT,
        };
        let pg = ScratchPg::start();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55441)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let store = brain_storage::PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        sqlx::raw_sql(include_str!(
            "../../../../../scripts/migrations/2026-10-04-brain-entity-derived-receipts-v1.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("ALTER TABLE brain.entity_derived_receipts_v1 ADD CHECK(receipt_json NOT LIKE '%sperren%')")
            .execute(&pool).await.unwrap();
        let mut profile = EntityProfile {
            contract_version: ENTITY_PROFILE_VERSION.into(),
            entity: EntityIdentity {
                entity_key: "hero:fixture".into(),
                kind: EntityKind::Hero,
                name: "Fixture".into(),
                aliases: Vec::new(),
                identity_evidence: vec!["fixture".into()],
            },
            patch: None,
            source_state: Vec::new(),
            facts: Vec::new(),
            context: Vec::new(),
            conflicts: Vec::new(),
            patch_story: Vec::new(),
            unknowns: vec!["Patchstand unbekannt".into()],
        };
        let document = |profile: &EntityProfile, original_release_id: &str| {
            let rendered = crate::entity_profile_render::render_entity_profile(profile).unwrap();
            let receipt = GitDocumentReceipt {
                contract_version: GIT_DOCUMENT_CONTRACT.into(),
                entity_key: profile.entity.entity_key.clone(),
                original_release_id: original_release_id.into(),
                document_sha256: crate::digest(rendered.brain_document.as_bytes()),
                fact_pins: Vec::new(),
            };
            let (record, private) =
                entity_profile_document(profile, &receipt, derived_policy()).unwrap();
            assert_eq!(record.content, rendered.brain_document);
            assert_eq!(record.metadata.len(), 3);
            assert_eq!(
                brain_contracts::source::origin_from_record(&record)
                    .unwrap()
                    .policy,
                derived_policy()
            );
            let mut wrong = receipt.clone();
            wrong.document_sha256 = "0".repeat(64);
            assert!(entity_profile_document(profile, &wrong, derived_policy()).is_err());
            let mut wrong_policy = derived_policy();
            wrong_policy.provider_egress_allowed = false;
            assert!(entity_profile_document(profile, &receipt, wrong_policy).is_err());
            (record, private)
        };
        let (original, receipt) = document(&profile, "original-r1");
        let initial = store
            .persist_entity_document(original.clone(), &receipt)
            .await
            .unwrap();
        assert_eq!(initial.content, original.content);
        assert_eq!(initial.revision, 1);
        assert!(initial
            .metadata
            .values()
            .all(|value| !value.contains("original-r1")));
        assert_eq!(
            store
                .persist_entity_document(original, &receipt)
                .await
                .unwrap(),
            initial
        );
        let (same, refreshed_receipt) = document(&profile, "original-r2");
        assert_eq!(
            store
                .persist_entity_document(same, &refreshed_receipt)
                .await
                .unwrap(),
            initial
        );
        profile
            .unknowns
            .push("Neuer gespeicherter Quellenhinweis".into());
        let (changed, changed_receipt) = document(&profile, "original-r2");
        let updated = store
            .persist_entity_document(changed, &changed_receipt)
            .await
            .unwrap();
        assert_eq!(updated.revision, 2);
        assert_ne!(initial.content, updated.content);
        profile
            .patch_story
            .push(brain_contracts::entity_profile::PatchStoryChange {
                patch_date: "2026-09-16".into(),
                patch_title: Some("Prüfpatch".into()),
                entity_type: Some("hero".into()),
                entity_name: Some("Fixture".into()),
                ability_name: None,
                stat_name: Some("MaxHealth".into()),
                old_value: serde_json::json!(30),
                new_value: serde_json::json!(42),
                change_type: Some("buff".into()),
                numeric_direction: Some("increase".into()),
                confidence: serde_json::json!(1),
                provenance: brain_contracts::entity_profile::PatchStoryProvenance {
                    relation: "brain.patch_changes".into(),
                    source_url: None,
                    evidence_ref: "fixture:patch:16-09".into(),
                },
                original_line: brain_contracts::entity_profile::RestrictedPatchLine {
                    text: None,
                    redistribution_allowed: false,
                },
                additional_fields: Default::default(),
            });
        let (story_document, story_receipt) = document(&profile, "original-r2");
        let story_updated = store
            .persist_entity_document(story_document.clone(), &story_receipt)
            .await
            .unwrap();
        assert_eq!(story_updated.revision, 3);
        let content: serde_json::Value = serde_json::from_str(&story_updated.content).unwrap();
        assert_eq!(content["patch_story"][0]["patch_date"], "2026-09-16");
        assert_eq!(content["patch_story"][0]["stat_name"], "MaxHealth");
        assert_eq!(content["patch_story"][0]["new_value"], 42);
        assert_eq!(
            store
                .persist_entity_document(story_document, &story_receipt)
                .await
                .unwrap(),
            story_updated
        );
        profile.unknowns.push("Weitere Speicherprobe".into());
        let (failed, failed_receipt) = document(&profile, "sperren");
        assert!(store
            .persist_entity_document(failed, &failed_receipt)
            .await
            .is_err());
        let records: Vec<serde_json::Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id='git-game-facts-derived' ORDER BY revision")
            .fetch_all(&pool).await.unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0]["content"], initial.content);
        assert_eq!(records[1]["content"], updated.content);
        assert_eq!(records[2]["content"], story_updated.content);
        let head: i64 = sqlx::query_scalar("SELECT revision FROM brain.source_record_heads WHERE source_id='git-game-facts-derived'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(head, 3);
        let receipts: i64 =
            sqlx::query_scalar("SELECT count(*) FROM brain.entity_derived_receipts_v1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(receipts, 3);
        let directory = tempfile::tempdir().unwrap();
        let artifacts = Artifacts::open(&directory.path().join("artifacts")).unwrap();
        let mut removed_profile = profile.clone();
        removed_profile.entity.entity_key = "hero:removed".into();
        let (removed_document, removed_receipt) = document(&removed_profile, "original-r2");
        let mut removed = store
            .persist_entity_document(removed_document.clone(), &removed_receipt)
            .await
            .unwrap();
        let mut foreign_document = story_updated.clone();
        let mut foreign_origin =
            brain_contracts::source::origin_from_record(&foreign_document).unwrap();
        foreign_document.source_id = "foreign-profiles".into();
        foreign_origin.identity.source_id = foreign_document.source_id.clone();
        foreign_origin.bind_record(&mut foreign_document).unwrap();
        store.apply(&foreign_document).await.unwrap();
        let base = brain_contracts::CorpusRelease {
            release_id: "profiles-base".into(),
            knowledge_version: "profiles-base".into(),
            patch: "unbekannt".into(),
            created_at_epoch: 1,
            source_revisions: std::collections::BTreeMap::from([(
                "git-game-facts-derived".into(),
                std::collections::BTreeMap::from([
                    (story_updated.logical_id.clone(), story_updated.revision),
                    (removed.logical_id.clone(), removed.revision),
                ]),
            )]),
        };
        let mut base = base;
        base.source_revisions.insert(
            foreign_document.source_id.clone(),
            std::collections::BTreeMap::from([(
                foreign_document.logical_id.clone(),
                foreign_document.revision,
            )]),
        );
        store.publish_release(&base).await.unwrap();
        let rendered =
            crate::entity_profile_render::render_entity_profile(&removed_profile).unwrap();
        render_and_export_profiles(&artifacts, &[removed_profile.clone()], directory.path())
            .unwrap();
        let own_html = directory.path().join(&rendered.public_relative_path);
        let foreign_html = directory.path().join("site/entities/foreign.html");
        std::fs::write(&foreign_html, "Fremder Snapshot").unwrap();
        assert!(retire_git_profile(
            &store,
            &mut removed,
            std::slice::from_ref(&rendered),
            directory.path()
        )
        .await
        .unwrap());
        assert!(!own_html.exists());
        assert_eq!(
            std::fs::read_to_string(&foreign_html).unwrap(),
            "Fremder Snapshot"
        );
        assert!(!retire_git_profile(
            &store,
            &mut removed,
            std::slice::from_ref(&rendered),
            directory.path()
        )
        .await
        .unwrap());
        profile
            .unknowns
            .push("Aktualisierung der erhaltenen Entität".into());
        let (changed, changed_receipt) = document(&profile, "original-r3");
        let changed = store
            .persist_entity_document(changed, &changed_receipt)
            .await
            .unwrap();
        let sources = vec!["git-game-facts-derived".into()];
        let published = publish_refreshed_sources(&store, &pool, &base.release_id, &sources)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            published.candidate.source_revisions["git-game-facts-derived"],
            std::collections::BTreeMap::from([(changed.logical_id.clone(), changed.revision)])
        );
        assert!(store
            .snapshot(&published.candidate.release_id)
            .await
            .unwrap()
            .revisions
            .contains(&foreign_document));
        let mut forbidden_tombstone = foreign_document.clone();
        forbidden_tombstone.tombstone = true;
        assert!(brain_storage::PgStore::prepare_imported_release(
            &store.snapshot(&base.release_id).await.unwrap(),
            &[foreign_document.source_id.clone()],
            vec![forbidden_tombstone],
            "foreign-removal",
            "foreign-removal",
            2
        )
        .is_err());
        let restored = store
            .persist_entity_document(removed_document, &removed_receipt)
            .await
            .unwrap();
        assert_eq!(restored.revision, 3);
        assert!(!restored.tombstone);
        render_and_export_profiles(&artifacts, &[removed_profile], directory.path()).unwrap();
        assert!(own_html.exists());
        let restored_release =
            publish_refreshed_sources(&store, &pool, &published.candidate.release_id, &sources)
                .await
                .unwrap()
                .unwrap();
        assert_eq!(
            restored_release.candidate.source_revisions["git-game-facts-derived"].len(),
            2
        );
        let foreign_page = std::fs::read(&own_html).unwrap();
        std::fs::write(&own_html, "Fremder Seiteninhalt").unwrap();
        let mut restored = restored;
        retire_git_profile(
            &store,
            &mut restored,
            std::slice::from_ref(&rendered),
            directory.path(),
        )
        .await
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(&own_html).unwrap(),
            "Fremder Seiteninhalt"
        );
        assert!(!foreign_page.is_empty());
        assert!(store
            .snapshot(&base.release_id)
            .await
            .unwrap()
            .revisions
            .iter()
            .any(|record| record.logical_id == "hero:removed" && record.revision == 1));
        pool.close().await;
    }

    #[test]
    fn repeating_profile_preserves_artifacts_and_patch_update_creates_new_outputs() {
        let directory = tempfile::tempdir().unwrap();
        let artifacts = Artifacts::open(&directory.path().join("artifacts")).unwrap();
        let mut profile = EntityProfile {
            contract_version: ENTITY_PROFILE_VERSION.into(),
            entity: EntityIdentity {
                entity_key: "hero/fixture".into(),
                kind: EntityKind::Hero,
                name: "Fixture".into(),
                aliases: Vec::new(),
                identity_evidence: vec!["fixture".into()],
            },
            patch: None,
            source_state: vec!["fixture/revision/1".into()],
            facts: Vec::new(),
            context: Vec::new(),
            conflicts: Vec::new(),
            patch_story: Vec::new(),
            unknowns: vec!["Patchstand unbekannt".into()],
        };
        let render = |profile: &EntityProfile| {
            Ok((
                serde_json::to_string(profile)?,
                format!("<p>{:?}</p>", profile.patch),
                PathBuf::from("site/entities/hero/fixture.html"),
            ))
        };
        let initial = stage_profiles(&artifacts, &[profile.clone()], render).unwrap();
        assert_eq!(
            stage_profiles(&artifacts, &[profile.clone()], render).unwrap(),
            initial
        );
        profile.patch = Some("fixture-patch".into());
        let updated = stage_profiles(&artifacts, &[profile], render).unwrap();
        assert_ne!(updated[0].brain_document_ref, initial[0].brain_document_ref);
        assert_ne!(updated[0].public_html_ref, initial[0].public_html_ref);
        assert!(artifacts.read(&initial[0].brain_document_ref).is_ok());
        assert!(artifacts.read(&updated[0].brain_document_ref).is_ok());

        let exported = render_and_export_profiles(
            &artifacts,
            &[EntityProfile {
                patch: Some("2026-09-16".into()),
                ..serde_json::from_slice(&artifacts.read(&initial[0].brain_document_ref).unwrap())
                    .unwrap()
            }],
            directory.path(),
        )
        .unwrap();
        let page = directory.path().join(&exported[0].public_relative_path);
        assert!(std::fs::read_to_string(&page)
            .unwrap()
            .contains("2026-09-16"));
        let mut changed: EntityProfile =
            serde_json::from_slice(&artifacts.read(&initial[0].brain_document_ref).unwrap())
                .unwrap();
        changed.patch = Some("2026-09-30".into());
        let refreshed =
            render_and_export_profiles(&artifacts, &[changed.clone()], directory.path()).unwrap();
        assert_ne!(
            exported[0].brain_document_ref,
            refreshed[0].brain_document_ref
        );
        assert_ne!(exported[0].public_html_ref, refreshed[0].public_html_ref);
        assert_eq!(
            render_and_export_profiles(&artifacts, &[changed], directory.path()).unwrap(),
            refreshed
        );
        let html = std::fs::read_to_string(page).unwrap();
        assert!(html.contains("2026-09-30"));
        assert!(!html.contains("2026-09-16"));
        assert!(artifacts.read(&exported[0].public_html_ref).is_ok());
    }
}
