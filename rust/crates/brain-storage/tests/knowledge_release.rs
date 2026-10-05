use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    CorpusRelease, Principal, SourceRecordV2, SourceVisibility,
};
use brain_storage::PgStore;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Postgres, Transaction,
};
use std::collections::{BTreeMap, BTreeSet};

fn record(source: &str, id: &str, revision: u64) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: source.into(),
        logical_id: id.into(),
        revision,
        content_hash: {
            use sha2::{Digest, Sha256};
            format!("{:x}", Sha256::digest("Überliefertes Wissen".as_bytes()))
        },
        content: "Überliefertes Wissen".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from(["knowledge:read".into()]),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: source.into(),
            logical_id: id.into(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: "fixture/document".into(),
        parser_revision: "fixture-v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(1)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: Observed::known("fixture-internal-grant".into()),
            license: Observed::unknown(UnknownReason::NotPresent),
            publication_allowed: false,
            provider_egress_allowed: false,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record
}

fn release(id: &str, records: &[SourceRecordV2]) -> CorpusRelease {
    let mut source_revisions: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    for record in records {
        source_revisions
            .entry(record.source_id.clone())
            .or_default()
            .insert(record.logical_id.clone(), record.revision);
    }
    CorpusRelease {
        release_id: id.into(),
        knowledge_version: "fixture-v1".into(),
        patch: "fixture-patch".into(),
        created_at_epoch: 1,
        source_revisions,
    }
}

async fn write_record(tx: &mut Transaction<'_, Postgres>, record: &SourceRecordV2) {
    let value = serde_json::to_value(record).unwrap();
    sqlx::query("INSERT INTO brain.source_record_revisions(source_id,logical_id,revision,content_hash,tombstone,record_json) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(source_id,logical_id,revision) DO UPDATE SET record_json=EXCLUDED.record_json")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
        .bind(&record.content_hash).bind(record.tombstone).bind(&value).execute(&mut **tx).await.unwrap();
    sqlx::query("INSERT INTO brain.source_record_heads(source_id,logical_id,revision,content_hash,tombstone,record_json) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(source_id,logical_id) DO UPDATE SET revision=EXCLUDED.revision,content_hash=EXCLUDED.content_hash,tombstone=EXCLUDED.tombstone,record_json=EXCLUDED.record_json")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
        .bind(&record.content_hash).bind(record.tombstone).bind(value).execute(&mut **tx).await.unwrap();
}

struct ScratchPg {
    directory: std::path::PathBuf,
}

impl ScratchPg {
    fn start() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("brain-release-pg-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let instance = Self { directory };
        let data = instance.directory.join("data");
        let socket = instance.directory.join("socket");
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
        let options = format!("-k {} -p 55439 -c listen_addresses=''", socket.display());
        assert!(
            std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                .arg("-D")
                .arg(&data)
                .arg("-l")
                .arg(instance.directory.join("postgres.log"))
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
            .arg(self.directory.join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

#[tokio::test]
async fn imported_heads_preserve_base_and_block_preparation_commit_races() {
    let pg = ScratchPg::start();
    let socket = pg.directory.join("socket");
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(address.is_none());
    let user: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(user, "brain_core_test");
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    let prefix = format!(
        "knowledge-release-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let old = record(&format!("{prefix}-old"), "old", 1);
    store.apply(&old).await.unwrap();
    let base = release(&format!("{prefix}-base"), std::slice::from_ref(&old));
    store.publish_release(&base).await.unwrap();
    let newer_old = record(&old.source_id, "old", 2);
    store.apply(&newer_old).await.unwrap();
    let selected = ["wiki", "game-tracking", "game-data"].map(|name| format!("{prefix}-{name}"));
    let imported: Vec<_> = selected
        .iter()
        .map(|source| record(source, "document", 1))
        .collect();
    for record in &imported {
        store.apply(record).await.unwrap();
    }
    let base_snapshot = store.snapshot(&base.release_id).await.unwrap();
    let (proposed, expected) = PgStore::prepare_imported_release(
        &base_snapshot,
        &selected,
        imported.clone(),
        &format!("{prefix}-published"),
        "fixture-v1",
        1,
    )
    .unwrap();
    assert_eq!(
        store
            .publish_imported_heads_checked(&base.release_id, &selected, &proposed, &expected)
            .await
            .unwrap(),
        4
    );
    assert_eq!(
        store
            .publish_imported_heads_checked(&base.release_id, &selected, &proposed, &expected)
            .await
            .unwrap(),
        4
    );
    let mut retry = proposed.clone();
    retry.created_at_epoch = 123_456;
    assert_eq!(
        store.imported_release_for_retry(&retry).await.unwrap(),
        proposed
    );
    assert_eq!(
        store
            .publish_imported_heads_checked(&base.release_id, &selected, &retry, &expected)
            .await
            .unwrap(),
        4
    );
    assert_eq!(
        store.snapshot(&proposed.release_id).await.unwrap().release,
        proposed
    );
    for change in ["knowledge_version", "patch", "pins"] {
        let mut changed = retry.clone();
        match change {
            "knowledge_version" => changed.knowledge_version = "changed".into(),
            "patch" => changed.patch = "changed".into(),
            _ => {
                changed.source_revisions.remove(&old.source_id);
            }
        }
        assert!(store.imported_release_for_retry(&changed).await.is_err());
        assert!(store
            .publish_imported_heads_checked(&base.release_id, &selected, &changed, &expected)
            .await
            .is_err());
    }
    let mut first = proposed.clone();
    first.release_id = format!("{prefix}-concurrent-retry");
    first.created_at_epoch = 10;
    let mut second = first.clone();
    second.created_at_epoch = 20;
    assert_eq!(
        store.imported_release_for_retry(&first).await.unwrap(),
        first
    );
    assert_eq!(
        store.imported_release_for_retry(&second).await.unwrap(),
        second
    );
    let (one, two) = tokio::join!(
        store.publish_imported_heads_checked(&base.release_id, &selected, &first, &expected),
        store.publish_imported_heads_checked(&base.release_id, &selected, &second, &expected),
    );
    assert_eq!(one.unwrap(), 4);
    assert_eq!(two.unwrap(), 4);
    let concurrent = store.snapshot(&first.release_id).await.unwrap().release;
    assert!([10, 20].contains(&concurrent.created_at_epoch));
    let mut normalized = first.clone();
    normalized.created_at_epoch = concurrent.created_at_epoch;
    assert_eq!(concurrent, normalized);

    let snapshot = store.snapshot(&proposed.release_id).await.unwrap();
    assert_eq!(snapshot.release, proposed);
    assert!(snapshot.revisions.contains(&old));
    assert!(snapshot.heads.contains(&newer_old));
    let principal = Principal {
        actor_id: "fixture".into(),
        channel: "internal".into(),
        scopes: BTreeSet::from(["knowledge:read".into()]),
        provider_egress: BTreeSet::new(),
    };
    assert_eq!(snapshot.authorized(&principal, false).unwrap().len(), 4);
    assert!(snapshot
        .authorized_for_publication(&principal)
        .unwrap()
        .is_empty());
    assert!(snapshot.authorized(&principal, true).unwrap().is_empty());
    assert!(store
        .commit_batches_and_publish_checked(&[], &proposed, &expected)
        .await
        .is_err());
    let mut changed = proposed.clone();
    changed.release_id = format!("{prefix}-changed-base");
    changed
        .source_revisions
        .get_mut(&old.source_id)
        .unwrap()
        .insert("old".into(), 2);
    assert!(store
        .publish_imported_heads_checked(&base.release_id, &selected, &changed, &expected)
        .await
        .is_err());
    let mut conflict = proposed.clone();
    conflict.knowledge_version = "conflict".into();
    assert!(store
        .publish_imported_heads_checked(&base.release_id, &selected, &conflict, &expected)
        .await
        .is_err());
    let mut missing_origin = expected.clone();
    missing_origin[0].metadata.clear();
    assert!(store
        .publish_imported_heads_checked(&base.release_id, &selected, &proposed, &missing_origin)
        .await
        .is_err());

    for race in [
        "revision",
        "acl",
        "added",
        "removed",
        "tombstone",
        "preserved-acl",
    ] {
        let source = format!("{prefix}-{race}");
        let head = record(&source, "document", 1);
        store.apply(&head).await.unwrap();
        let mut target = release(
            &format!("{prefix}-race-{race}"),
            std::slice::from_ref(&head),
        );
        target
            .source_revisions
            .extend(base.source_revisions.clone());
        let heads = vec![head.clone(), newer_old.clone()];
        let locked_source = if race == "preserved-acl" {
            &old.source_id
        } else {
            &source
        };
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(format!("core-source:{locked_source}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let worker_store = store.clone();
        let worker_base = base.release_id.clone();
        let worker_target = target.clone();
        let worker_source = source.clone();
        let publishing = tokio::spawn(async move {
            worker_store
                .publish_imported_heads_checked(
                    &worker_base,
                    &[worker_source],
                    &worker_target,
                    &heads,
                )
                .await
        });
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        assert!(!publishing.is_finished());
        match race {
            "removed" => {
                sqlx::query("DELETE FROM brain.source_record_heads WHERE source_id=$1")
                    .bind(&source)
                    .execute(&mut *tx)
                    .await
                    .unwrap();
            }
            "acl" | "preserved-acl" => {
                let mut restricted = if race == "preserved-acl" {
                    newer_old.clone()
                } else {
                    head.clone()
                };
                restricted
                    .allowed_scopes
                    .insert("knowledge:restricted".into());
                let mut origin =
                    brain_contracts::source::origin_from_record(if race == "preserved-acl" {
                        &newer_old
                    } else {
                        &head
                    })
                    .unwrap();
                origin.policy.allowed_scopes = restricted.allowed_scopes.clone();
                origin.bind_record(&mut restricted).unwrap();
                write_record(&mut tx, &restricted).await;
            }
            _ => {
                let mut changed = if race == "added" {
                    record(&source, "new-document", 1)
                } else {
                    record(&source, "document", 2)
                };
                changed.tombstone = race == "tombstone";
                write_record(&mut tx, &changed).await;
            }
        }
        tx.commit().await.unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(3), publishing)
                .await
                .unwrap()
                .unwrap()
                .is_err()
        );
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM brain.corpus_releases_v1 WHERE release_id=$1)",
        )
        .bind(&target.release_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(!exists, "{race} must not publish");
        if race == "preserved-acl" {
            let mut tx = pool.begin().await.unwrap();
            write_record(&mut tx, &newer_old).await;
            tx.commit().await.unwrap();
        }
    }
    let mut restricted = imported[0].clone();
    let mut origin = brain_contracts::source::origin_from_record(&restricted).unwrap();
    restricted
        .allowed_scopes
        .insert("knowledge:restricted".into());
    origin.policy.allowed_scopes = restricted.allowed_scopes.clone();
    origin.bind_record(&mut restricted).unwrap();
    let mut tx = pool.begin().await.unwrap();
    write_record(&mut tx, &restricted).await;
    tx.commit().await.unwrap();
    assert!(store
        .publish_imported_heads_checked(&base.release_id, &selected, &retry, &expected)
        .await
        .is_err());
    assert_eq!(
        store.snapshot(&proposed.release_id).await.unwrap().release,
        proposed
    );
    sqlx::query("UPDATE brain.corpus_releases_v1 SET knowledge_version='fixture-corrupt' WHERE release_id=$1")
        .bind(&concurrent.release_id).execute(&pool).await.unwrap();
    assert!(store.imported_release_for_retry(&first).await.is_err());
    for kind in ["game_file", "wiki"] {
        let mut raw = record(&format!("{prefix}-raw-{kind}"), "original", 1);
        raw.metadata.insert(
            brain_storage::source_versions::ORIGINAL_VERSION_KEY.into(),
            "1".into(),
        );
        raw.metadata.insert(brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(), serde_json::json!({
            "contract_version":"wiki-spielwissen-v1", "source_kind":kind, "source_id":raw.source_id,
            "document_id":raw.logical_id, "revision":"1", "content":raw.content, "content_sha256":raw.content_hash
        }).to_string());
        store.apply(&raw).await.unwrap();
        let raw_base = release(
            &format!("{prefix}-raw-base-{kind}"),
            std::slice::from_ref(&raw),
        );
        store.publish_release(&raw_base).await.unwrap();
        let mut withdrawn = raw.clone();
        withdrawn.revision += 1;
        withdrawn.tombstone = true;
        store.apply(&withdrawn).await.unwrap();
        let selected = vec![raw.source_id.clone()];
        let snapshot = store.snapshot(&raw_base.release_id).await.unwrap();
        let (candidate, expected) = PgStore::prepare_imported_release(
            &snapshot,
            &selected,
            vec![withdrawn.clone()],
            &format!("{prefix}-raw-retired-{kind}"),
            "raw-retired",
            2,
        )
        .unwrap();
        assert!(candidate.source_revisions[&raw.source_id].is_empty());
        let mut altered = withdrawn.clone();
        let mut origin = brain_contracts::source::origin_from_record(&altered).unwrap();
        altered.allowed_scopes.insert("knowledge:restricted".into());
        origin.policy.allowed_scopes = altered.allowed_scopes.clone();
        origin.bind_record(&mut altered).unwrap();
        let mut tx = pool.begin().await.unwrap();
        write_record(&mut tx, &altered).await;
        tx.commit().await.unwrap();
        let (_, altered_expected) = PgStore::prepare_imported_release(
            &snapshot,
            &selected,
            vec![altered],
            &candidate.release_id,
            &candidate.knowledge_version,
            2,
        )
        .unwrap();
        assert!(store
            .publish_imported_heads_checked(
                &raw_base.release_id,
                &selected,
                &candidate,
                &altered_expected
            )
            .await
            .is_err());
        let mut tx = pool.begin().await.unwrap();
        write_record(&mut tx, &withdrawn).await;
        tx.commit().await.unwrap();
        assert_eq!(
            store
                .publish_imported_heads_checked(
                    &raw_base.release_id,
                    &selected,
                    &candidate,
                    &expected
                )
                .await
                .unwrap(),
            0
        );
        assert!(store
            .snapshot(&candidate.release_id)
            .await
            .unwrap()
            .revisions
            .is_empty());
        assert!(store
            .snapshot(&raw_base.release_id)
            .await
            .unwrap()
            .revisions
            .contains(&raw));
        let mut unproven = raw.clone();
        unproven.revision = 3;
        let mut tx = pool.begin().await.unwrap();
        write_record(&mut tx, &unproven).await;
        tx.commit().await.unwrap();
        unproven.revision = 4;
        unproven.tombstone = true;
        store.apply(&unproven).await.unwrap();
        assert!(store
            .publish_imported_heads_checked(
                &raw_base.release_id,
                &selected,
                &candidate,
                &[unproven]
            )
            .await
            .is_err());
    }
    pool.close().await;
}

#[tokio::test]
async fn spielprofilmigration_ist_explizit_wiederholbar_und_erhaelt_tabellenrechte() {
    use brain_contracts::entity_profile::{EntityIdentity, EntityKind};
    let pg = ScratchPg::start();
    let socket = pg.directory.join("socket");
    let connect = |role: &str| {
        PgPoolOptions::new().max_connections(1).connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(55439)
                .username(role)
                .database("postgres")
                .password(""),
        )
    };
    let pool = connect("brain_core_test").await.unwrap();
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    sqlx::raw_sql("CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN")
        .execute(&pool).await.unwrap();
    let config_path = pg.directory.join("migration.json");
    std::fs::write(
        &config_path,
        serde_json::to_vec(&serde_json::json!({
            "socket": socket, "port":55439, "database":"postgres", "user":"brain_core_test"
        }))
        .unwrap(),
    )
    .unwrap();
    let invoke = |mode: &str| {
        std::process::Command::new(env!("CARGO_BIN_EXE_brain-migrate"))
            .arg(mode)
            .arg("--config")
            .arg(&config_path)
            .output()
            .unwrap()
    };
    assert!(!invoke("check-entity-profiles").status.success());
    let exists: bool =
        sqlx::query_scalar("SELECT to_regclass('brain.entity_profile_entities_v1') IS NOT NULL")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(!exists);
    let mut original = record("migration-game", "original", 1);
    original.metadata.insert(
        brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(),
        serde_json::json!({
            "document_id":original.logical_id,"source_id":original.source_id,
            "content_sha256":original.content_hash,"content":original.content,
            "source_kind":"game_file","revision":"fixture-v1","observed_at":"2026-10-04",
            "license":null,"metadata":{},
            "facts":[{"fact_id":"health","subject":"hero:Migrationsheld","predicate":"Health",
                "value":830,"unit":null,"qualifiers":{},"evidence_status":"structural_match","source_span":null}]
        }).to_string(),
    );
    store.apply(&original).await.unwrap();
    for _ in 0..2 {
        let output = invoke("up-entity-profiles");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(invoke("check-entity-profiles").status.success());
        let rights: (bool, bool, bool, bool) = sqlx::query_as(
            "SELECT has_table_privilege('brain_ingest','brain.entity_semantic_projections_v1','SELECT'),has_table_privilege('brain_ingest','brain.entity_semantic_projections_v1','INSERT'),has_table_privilege('brain_ingest','brain.entity_semantic_projections_v1','UPDATE'),has_table_privilege('brain_ingest','brain.entity_patch_intervals_v1','UPDATE')",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(rights, (true, true, false, true));
    }
    let grants = include_str!("../../../../ops/brain-postgres/grants.sql")
        .lines()
        .filter(|line| !line.starts_with('\\'))
        .collect::<Vec<_>>()
        .join("\n");
    for _ in 0..2 {
        sqlx::raw_sql(&grants).execute(&pool).await.unwrap();
    }
    let ingest_pool = connect("brain_ingest").await.unwrap();
    let ingest = PgStore::new(ingest_pool.clone());
    ingest.check_entity_profile_schema().await.unwrap();
    assert!(ingest.migrate_entity_profiles().await.is_err());
    let raw_rights: (bool, bool, bool) = sqlx::query_as(
        "SELECT has_table_privilege(current_user,'brain.source_record_revisions','SELECT'),has_table_privilege(current_user,'brain.source_record_revisions','INSERT'),has_table_privilege(current_user,'brain.source_record_revisions','UPDATE')",
    )
    .fetch_one(&ingest_pool)
    .await
    .unwrap();
    assert_eq!(raw_rights, (true, true, false));
    let entity = EntityIdentity {
        entity_key: "migration-hero".into(),
        kind: EntityKind::Hero,
        name: "Migrationsheld".into(),
        aliases: Vec::new(),
        identity_evidence: vec!["fixture-v1".into()],
    };
    assert_eq!(
        ingest
            .store_entity_fact_bindings(&entity, &original, &["health".into()])
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        ingest
            .store_entity_fact_bindings(&entity, &original, &["health".into()])
            .await
            .unwrap(),
        0
    );
    sqlx::query(
        "UPDATE brain.entity_profile_facts_v1 SET binding_identity_json=NULL WHERE entity_key=$1",
    )
    .bind(&entity.entity_key)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        ingest
            .store_entity_fact_bindings(&entity, &original, &["health".into()])
            .await
            .unwrap(),
        1
    );
    sqlx::query("INSERT INTO brain.entity_semantic_projections_v1(entity_key,source_id,logical_id,revision,fact_id,relative_pointer,semantic_predicate,semantic_qualifiers_json) VALUES($1,$2,$3,1,'health','/hero/Health','Health','{}') ON CONFLICT DO NOTHING")
        .bind(&entity.entity_key).bind(&original.source_id).bind(&original.logical_id).execute(&ingest_pool).await.unwrap();
    for value in ["first", "second"] {
        sqlx::query("INSERT INTO brain.entity_patch_intervals_v1(entity_key,source_id,logical_id,revision,fact_id,current_patch_fact_id,interval_json) VALUES($1,$2,$3,1,'health','patch',$4) ON CONFLICT(entity_key,source_id,logical_id,revision,fact_id) DO UPDATE SET interval_json=EXCLUDED.interval_json")
            .bind(&entity.entity_key).bind(&original.source_id).bind(&original.logical_id).bind(value).execute(&ingest_pool).await.unwrap();
    }
    sqlx::query("INSERT INTO brain.entity_derived_receipts_v1(derived_source_id,derived_logical_id,derived_revision,receipt_json) VALUES($1,$2,1,'{}')")
        .bind(&original.source_id).bind(&original.logical_id).execute(&ingest_pool).await.unwrap();
    for table in [
        "entity_profile_entities_v1",
        "entity_profile_facts_v1",
        "entity_semantic_projections_v1",
        "entity_patch_intervals_v1",
        "entity_derived_receipts_v1",
    ] {
        let delete: bool =
            sqlx::query_scalar("SELECT has_table_privilege(current_user,$1,'DELETE')")
                .bind(format!("brain.{table}"))
                .fetch_one(&ingest_pool)
                .await
                .unwrap();
        assert!(!delete);
    }
    for table in [
        "entity_semantic_projections_v1",
        "entity_derived_receipts_v1",
    ] {
        let update: bool =
            sqlx::query_scalar("SELECT has_table_privilege(current_user,$1,'UPDATE')")
                .bind(format!("brain.{table}"))
                .fetch_one(&ingest_pool)
                .await
                .unwrap();
        assert!(!update);
    }
    for role in ["brain_service", "brain_readonly"] {
        let role_pool = connect(role).await.unwrap();
        let reader = PgStore::new(role_pool.clone());
        reader.check_entity_profile_schema().await.unwrap();
        assert!(reader.migrate_entity_profiles().await.is_err());
        assert!(sqlx::query("DELETE FROM brain.entity_derived_receipts_v1")
            .execute(&role_pool)
            .await
            .is_err());
        let receipts: i64 =
            sqlx::query_scalar("SELECT count(*) FROM brain.entity_derived_receipts_v1")
                .fetch_one(&role_pool)
                .await
                .unwrap();
        assert_eq!(receipts, 1);
        role_pool.close().await;
    }
    assert!(invoke("up-entity-profiles").status.success());
    let unchanged: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=1")
        .bind(&original.source_id).bind(&original.logical_id).fetch_one(&pool).await.unwrap();
    assert_eq!(unchanged, serde_json::to_value(original).unwrap());
    let retained: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.entity_profile_facts_v1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(retained, 1);
    ingest_pool.close().await;
    pool.close().await;
}
