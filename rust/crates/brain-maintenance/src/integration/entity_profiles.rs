use super::artifacts::Artifacts;
use anyhow::{ensure, Result};
use brain_contracts::entity_profile::EntityProfile;
use std::{
    collections::BTreeSet,
    path::{Component, Path, PathBuf},
};

pub async fn refresh_git_knowledge(
    source: &super::runtime_config::EntityProfileSource,
    pinned: &dbrain_sources::git_source::PinnedRepository,
    repository_path: &Path,
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
) -> Result<serde_json::Value> {
    use dbrain_sources::{game_files, knowledge_contract, knowledge_import};
    let staged = tempfile::tempdir()?;
    let mut files = std::collections::BTreeMap::new();
    for prefix in &source.paths {
        for blob in pinned.files(prefix)? {
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
        std::fs::write(target, pinned.read_blob(path)?)?;
    }
    let mut options = source.extraction.clone();
    options.root = staged.path().to_owned();
    options.source_revision = Some(pinned.commit().to_owned());
    options.observed_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    options.provenance["git_commit"] = serde_json::json!(pinned.commit());
    let mut output = tempfile::tempfile()?;
    let inventory = game_files::extract_game_files(&options, &mut output)?;
    ensure!(
        inventory.gaps.is_empty(),
        "Spielquellenextraktion meldet offene Lücken"
    );
    use std::io::{Seek, SeekFrom};
    output.seek(SeekFrom::Start(0))?;
    let input = knowledge_contract::validate_knowledge_jsonl(std::io::BufReader::new(output))?;
    let policy: knowledge_import::ImportPolicy = serde_json::from_slice(
        &super::runtime_config::read_bounded(&source.import_policy, 256 * 1024)?,
    )?;
    let prepared = knowledge_import::prepare_validated_knowledge(
        &input,
        &policy,
        game_files::EXTRACTOR_VERSION,
    )?;
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
    let imported = knowledge_import::import_prepared_knowledge(store, prepared).await?;
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
        pins.entry(head.source_id.clone())
            .or_default()
            .insert(head.logical_id.clone(), head.revision);
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
                .filter(|record| sources.contains(&record.source_id))
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
        actual.len() == expected_heads.len()
            && expected_heads
                .iter()
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
    let runtime_path = runtime
        .loaded_from
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("entity_profile_runtime_path"))?;
    let executable = std::env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("entity_profile_binary_directory"))?
        .join("brain-candidate-activate");
    let serve_bytes = super::runtime_config::read_bounded(&runtime.serve_config, 65536)?;
    let mut args = vec![
        "--config".into(),
        runtime_path.to_string_lossy().into_owned(),
        "--target".into(),
        "entity-profiles-internal".into(),
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
    let active = super::activation::ActivationTarget::SecondBrainInternal.release(&serve)?;
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
        sqlx::raw_sql("CREATE SCHEMA brain; CREATE SCHEMA patchnotes;")
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
            CREATE TABLE patchnotes.changelog_posts(id bigint PRIMARY KEY,title text,url text,posted_at timestamptz,raw_content text,translated_content text);
            INSERT INTO brain.entities(entity_type,canonical_name) VALUES('hero','Warden');
            INSERT INTO patchnotes.changelog_posts VALUES(1,'09-16-2026 Update','https://forums.playdeadlock.com/threads/fixture1','2026-09-16T12:00:00Z','Warden\n- Health increased from 500 to 550',NULL);")
            .execute(&pool).await.unwrap();
        let raw = pg.directory.path().join("raw");
        let initial = refresh_patch_history(&pool, &raw).await.unwrap();
        assert!(initial["parsed"]["events_inserted"].as_i64().unwrap() > 0);
        assert_eq!(initial["parsed"]["deleted_before_parse"], 0);
        let first: Vec<(i64, String)> =
            sqlx::query_as("SELECT id,event_hash FROM brain.patch_events ORDER BY id")
                .fetch_all(&pool)
                .await
                .unwrap();
        let repeated = refresh_patch_history(&pool, &raw).await.unwrap();
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
        let updated = refresh_patch_history(&pool, &raw).await.unwrap();
        assert!(updated["parsed"]["events_inserted"].as_i64().unwrap() > 0);
        assert_eq!(updated["parsed"]["deleted_before_parse"], 0);
        let historical: Vec<(i64,String)> = sqlx::query_as("SELECT id,event_hash FROM brain.patch_events WHERE posted_at::date='2026-09-16' ORDER BY id")
            .fetch_all(&pool).await.unwrap();
        assert_eq!(historical, first);
        assert_eq!(
            refresh_patch_history(&pool, &raw).await.unwrap()["parsed"]["events_inserted"],
            0
        );
        pool.close().await;
    }

    #[tokio::test]
    async fn compact_renderer_document_storage_is_atomic_and_idempotent() {
        use brain_contracts::{
            source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
            value::{Observed, UnknownReason},
            SourceRecordV2, SourceVisibility,
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
        sqlx::raw_sql("CREATE TABLE brain.entity_derived_receipts_v1(derived_source_id text NOT NULL,derived_logical_id text NOT NULL,derived_revision bigint NOT NULL,receipt_json text NOT NULL CHECK(receipt_json NOT LIKE '%sperren%'),PRIMARY KEY(derived_source_id,derived_logical_id,derived_revision),FOREIGN KEY(derived_source_id,derived_logical_id,derived_revision) REFERENCES brain.source_record_revisions(source_id,logical_id,revision))")
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
            let hash = crate::digest(rendered.brain_document.as_bytes());
            let receipt = serde_json::json!({
                "contract_version":"git-entity-document-v1",
                "entity_key":profile.entity.entity_key,
                "original_release_id":original_release_id,
                "document_sha256":hash,
                "fact_pins":[{"fixture":"Private Speicherprobe"}],
            })
            .to_string();
            let policy = SourcePolicy {
                visibility: SourceVisibility::Public,
                allowed_scopes: Default::default(),
                authorization_ref: Observed::known("fixture".into()),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: true,
                provider_egress_allowed: false,
                raw_retention_allowed: false,
            };
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
                        "git-entity-document-v1".into(),
                    ),
                    (
                        "brain.entity_projection.receipt_sha256".into(),
                        crate::digest(receipt.as_bytes()),
                    ),
                ]),
            };
            OriginArtifact {
                identity: SourceIdentity {
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                },
                source_revision: SourceRevision::Api {
                    api_version: "git-entity-document-v1".into(),
                    original_revision: Some(hash.clone()),
                },
                raw_sha256: hash,
                locator: "git-game-facts-derived".into(),
                parser_revision: ENTITY_PROFILE_VERSION.into(),
                parser_family: "entity-profile-render".into(),
                schema_version: Observed::known(ENTITY_PROFILE_VERSION.into()),
                schema_sha256: Observed::unknown(UnknownReason::NotPresent),
                retrieved_at: Observed::unknown(UnknownReason::NotPresent),
                source_time: Observed::unknown(UnknownReason::NotPresent),
                language: Observed::known("de".into()),
                origin_artifacts: Default::default(),
                derivation_family: Observed::known("git-entity-document-v1".into()),
                policy,
                validity: GameValidity::unknown(),
            }
            .bind_record(&mut record)
            .unwrap();
            (record, receipt)
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
            .all(|value| !value.contains("Private Speicherprobe")));
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
        profile.unknowns.push("Weitere Speicherprobe".into());
        let (failed, failed_receipt) = document(&profile, "sperren");
        assert!(store
            .persist_entity_document(failed, &failed_receipt)
            .await
            .is_err());
        let records: Vec<serde_json::Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id='git-game-facts-derived' ORDER BY revision")
            .fetch_all(&pool).await.unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["content"], initial.content);
        assert_eq!(records[1]["content"], updated.content);
        let head: i64 = sqlx::query_scalar("SELECT revision FROM brain.source_record_heads WHERE source_id='git-game-facts-derived'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(head, 2);
        let receipts: i64 =
            sqlx::query_scalar("SELECT count(*) FROM brain.entity_derived_receipts_v1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(receipts, 2);
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
