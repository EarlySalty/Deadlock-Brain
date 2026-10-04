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
    let mut proposed = release(&format!("{prefix}-published"), &imported);
    proposed
        .source_revisions
        .extend(base.source_revisions.clone());
    let expected: Vec<_> = imported
        .iter()
        .cloned()
        .chain([newer_old.clone()])
        .collect();
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
    pool.close().await;
}
