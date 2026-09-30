use super::*;
use brain_contracts::{source::origin_from_record, DocumentStorePort, Principal, SnapshotReadPort};
use brain_storage::MemoryRepository;

fn context() -> ImportContext {
    ImportContext {
        snapshot_label: "legacy-import-test".into(),
        snapshot_epoch: 1_790_000_000,
        schema_sha256: "a".repeat(64),
    }
}

fn public_policy() -> SourcePolicyConfig {
    SourcePolicyConfig {
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["game.public".into()]),
        provider_egress_allowed: false,
        publication_allowed: false,
        raw_retention_allowed: true,
        authorization_ref: None,
    }
}

fn line(id: i64, patch: &str, index: i64, raw: &str) -> PatchLineRow {
    PatchLineRow {
        event_id: id,
        patch_external_id: patch.into(),
        patch_title: Some(format!("Update {patch}")),
        patch_url: Some(format!("https://example.invalid/{patch}")),
        posted_at_epoch: Some(1_700_000_000),
        posted_at_iso: Some("2023-11-14T22:13:20Z".into()),
        source_kind: Some("steam".into()),
        line_index: Some(index),
        section: None,
        entity_name: Some("Abrams".into()),
        raw_line: Some(raw.into()),
        language: Some("en".into()),
        stat_name: None,
        old_value: None,
        new_value: None,
        unit: None,
    }
}

fn entity(id: i64, name: &str) -> EntityRow {
    EntityRow {
        entity_id: id,
        entity_type: "hero".into(),
        canonical_name: name.into(),
        primary_external_id: Some(format!("hero_{id}")),
        source: Some("deadlock_assets_api".into()),
        metadata: serde_json::json!({"z": 1, "a": "x", "n": null}),
        aliases: vec![("Bull".into(), Some("canonical".into()))],
    }
}

async fn commit(repo: &MemoryRepository, batch: &SourceBatch, owner: &str) -> bool {
    let lease = repo
        .claim(&batch.checkpoint.source_id, owner, 30_000)
        .await
        .unwrap();
    repo.commit(batch, &lease).await.unwrap().replayed
}

#[test]
fn patch_documents_are_ordered_enriched_and_keep_unknowns() {
    let mut enriched = line(2, "p1", 1, "Bullet damage increased");
    enriched.stat_name = Some("Bullet damage".into());
    enriched.old_value = Some("5".into());
    enriched.new_value = Some("6".into());
    let mut control = line(3, "p1", 2, "Line\nwith break");
    control.entity_name = None;
    let source = patch_documents(&[control, line(1, "p1", 0, "Abrams: first"), enriched]).unwrap();
    assert_eq!(source.documents.len(), 1);
    let doc = &source.documents[0];
    assert_eq!(doc.logical_id, "patch/p1");
    let body: Vec<_> = doc
        .content
        .lines()
        .filter(|l| l.starts_with("- "))
        .collect();
    assert_eq!(
        body,
        vec![
            "- Abrams: first",
            "- Abrams: Bullet damage increased (change: Bullet damage 5 -> 6)",
            "- Line with break",
        ]
    );
    assert_eq!(doc.patch, Observed::known("p1".to_string()));
    assert_eq!(doc.legacy_rows, 3);
    assert_eq!(doc.kind, "prose");
    assert_eq!(
        doc.source_time,
        Observed::known(SourceTimestamp::UnixSeconds(1_700_000_000))
    );
}

#[test]
fn conflicting_patch_headers_stay_unknown() {
    let mut other = line(2, "p1", 1, "second");
    other.posted_at_epoch = Some(1_700_000_001);
    other.patch_url = Some("https://example.invalid/mirror".into());
    let doc = &patch_documents(&[line(1, "p1", 0, "first"), other])
        .unwrap()
        .documents[0];
    assert_eq!(doc.source_time, Observed::unknown(UnknownReason::Unmapped));
    assert!(doc.locator.starts_with("brain_legacy.patch_events#"));
    assert_eq!(doc.origin_artifacts.len(), 2);
    assert_eq!(doc.content.matches("Patch: ").count(), 2);
}

#[test]
fn entity_documents_sort_metadata_and_reject_duplicates() {
    let source = entity_documents(&[entity(2, "Abrams")]).unwrap();
    let doc = &source.documents[0];
    assert_eq!(doc.logical_id, "entity/hero/Abrams");
    assert_eq!(doc.kind, "fact");
    assert!(doc.content.contains("Aliases: Bull\n"));
    assert!(doc.content.find("a: x").unwrap() < doc.content.find("z: 1").unwrap());
    assert!(!doc.content.contains("n: "));
    assert_eq!(doc.patch, Observed::unknown(UnknownReason::NotPresent));
    assert!(entity_documents(&[entity(1, "Abrams"), entity(2, "Abrams")]).is_err());
    let mut duplicate_external = entity(2, "Warden");
    duplicate_external.primary_external_id = Some("hero_1".into());
    assert!(entity_documents(&[entity(1, "Abrams"), duplicate_external]).is_err());
    let mut spoofed = entity(3, "Seven");
    spoofed.metadata = serde_json::json!({"External ID": "hero_25"});
    assert!(entity_documents(&[spoofed]).is_err());
    let mut spoofed_source = entity(3, "Seven");
    spoofed_source.metadata = serde_json::json!({"Source": "other"});
    assert!(entity_documents(&[spoofed_source]).is_err());
    let mut spoofed_alias = entity(3, "Seven");
    spoofed_alias.metadata = serde_json::json!({"Aliases:": "Warden"});
    assert!(entity_documents(&[spoofed_alias]).is_err());
    let mut spaced_alias = entity(3, "Seven");
    spaced_alias.metadata = serde_json::json!({"Aliases :": "Warden"});
    assert!(entity_documents(&[spaced_alias]).is_err());
    let mut double_colon_alias = entity(3, "Seven");
    double_colon_alias.metadata = serde_json::json!({"Aliases::": "Warden"});
    assert!(entity_documents(&[double_colon_alias]).is_err());
    let mut spoofed_external = entity(3, "Seven");
    spoofed_external.metadata = serde_json::json!({"External\tID": "hero_25"});
    assert!(entity_documents(&[spoofed_external]).is_err());
    let mut spoofed_name = entity(3, "Seven");
    spoofed_name.metadata = serde_json::json!({"hero": "Warden"});
    let imported = entity_documents(&[spoofed_name]).unwrap();
    assert!(imported.documents[0].content.starts_with("hero: Seven\n"));
    assert!(!imported.documents[0].content.contains("hero: Warden"));
    let mut spoofed_colon_name = entity(3, "Seven");
    spoofed_colon_name.metadata = serde_json::json!({"hero :": "Warden"});
    assert!(
        !entity_documents(&[spoofed_colon_name]).unwrap().documents[0]
            .content
            .contains("hero :")
    );
    let mut unrelated_colon = entity(3, "Seven");
    unrelated_colon.metadata = serde_json::json!({"other: hero": "Warden"});
    assert!(!entity_documents(&[unrelated_colon]).unwrap().documents[0]
        .content
        .contains("other: hero"));
    let mut historical = entity(3, "Seven");
    historical.metadata = serde_json::json!({"hero": 25});
    assert!(!entity_documents(&[historical]).unwrap().documents[0]
        .content
        .contains("hero: 25"));
    let batch = prepare_batch(&source, &public_policy(), &context(), None).unwrap();
    assert_eq!(
        batch.records[0]
            .metadata
            .get("entity_source")
            .map(String::as_str),
        Some("deadlock_assets_api")
    );
    assert_eq!(
        batch.records[0]
            .metadata
            .get("entity_external_id")
            .map(String::as_str),
        Some("hero_2")
    );
}

#[test]
fn changed_parser_configuration_reissues_unchanged_entity_with_structured_identity() {
    let source = entity_documents(&[entity(25, "Warden")]).unwrap();
    let first = prepare_batch(&source, &public_policy(), &context(), None).unwrap();
    let mut old_checkpoint = first.checkpoint;
    old_checkpoint.configuration = "previous-parser-configuration".into();
    old_checkpoint.state["configuration"] =
        serde_json::Value::String(old_checkpoint.configuration.clone());
    let refreshed =
        prepare_batch(&source, &public_policy(), &context(), Some(&old_checkpoint)).unwrap();
    assert_eq!(refreshed.records.len(), 1);
    assert_eq!(refreshed.records[0].revision, 2);
    assert_eq!(
        refreshed.records[0]
            .metadata
            .get("entity_external_id")
            .map(String::as_str),
        Some("hero_25")
    );
}

#[tokio::test]
async fn reimport_is_idempotent_updates_revise_and_removals_tombstone() {
    let repo = MemoryRepository::default();
    let policy = public_policy();
    let first = patch_documents(&[line(1, "p1", 0, "a"), line(2, "p2", 0, "b")]).unwrap();
    let batch = prepare_batch(&first, &policy, &context(), None).unwrap();
    assert_eq!(batch.records.len(), 2);
    for record in &batch.records {
        let origin = origin_from_record(record).unwrap();
        assert_eq!(origin.raw_sha256, record.content_hash);
        assert_eq!(
            origin.policy.license,
            Observed::unknown(UnknownReason::NotPresent)
        );
        assert_eq!(
            origin.validity.mode,
            Observed::unknown(UnknownReason::NotPresent)
        );
        assert!(!record.metadata.contains_key("patch"));
    }
    let lease = repo.claim(PATCHNOTES_SOURCE, "one", 30_000).await.unwrap();
    assert!(!repo.commit(&batch, &lease).await.unwrap().replayed);
    assert!(repo.commit(&batch, &lease).await.unwrap().replayed);

    let cp = repo.checkpoint(PATCHNOTES_SOURCE).await.unwrap().unwrap();
    let again = prepare_batch(&first, &policy, &context(), Some(&cp)).unwrap();
    assert!(again.records.is_empty());
    commit(&repo, &again, "two").await;

    let cp = repo.checkpoint(PATCHNOTES_SOURCE).await.unwrap().unwrap();
    let changed = patch_documents(&[line(1, "p1", 0, "a2")]).unwrap();
    let update = prepare_batch(&changed, &policy, &context(), Some(&cp)).unwrap();
    let by_id: BTreeMap<_, _> = update
        .records
        .iter()
        .map(|r| (r.logical_id.as_str(), r))
        .collect();
    assert_eq!(by_id["patch/p1"].revision, 2);
    assert!(!by_id["patch/p1"].tombstone);
    assert_eq!(by_id["patch/p2"].revision, 2);
    assert!(by_id["patch/p2"].tombstone);
    commit(&repo, &update, "three").await;

    let cp = repo.checkpoint(PATCHNOTES_SOURCE).await.unwrap().unwrap();
    let release = release_from_checkpoints(
        std::slice::from_ref(&cp),
        &ReleaseConfig {
            id_prefix: "legacy-test".into(),
            knowledge_version: "kv".into(),
            patch: "legacy-archive-test".into(),
        },
        1,
    )
    .unwrap();
    assert_eq!(release.source_revisions[PATCHNOTES_SOURCE].len(), 1);
    repo.publish(&release).await.unwrap();
    repo.publish(&release).await.unwrap();
    let snapshot = repo.read_snapshot(&release.release_id).unwrap();
    let digest = snapshot_digest(&snapshot.revisions);
    assert_eq!(digest, snapshot_digest(&snapshot.revisions));
    let principal = |scopes: &[&str]| Principal {
        actor_id: "a".into(),
        channel: "test".into(),
        scopes: scopes.iter().map(|s| s.to_string()).collect(),
        provider_egress: BTreeSet::from(["public".into()]),
    };
    assert_eq!(
        snapshot
            .authorized(&principal(&["game.public"]), false)
            .unwrap()
            .len(),
        1
    );
    assert!(snapshot
        .authorized(&principal(&["docs.public"]), false)
        .unwrap()
        .is_empty());
    assert!(snapshot
        .authorized(&principal(&["game.public"]), true)
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn policy_change_revises_unchanged_content_and_restricts_heads() {
    let repo = MemoryRepository::default();
    let docs = entity_documents(&[entity(1, "Abrams")]).unwrap();
    let batch = prepare_batch(&docs, &public_policy(), &context(), None).unwrap();
    commit(&repo, &batch, "one").await;
    let cp = repo.checkpoint(ENTITIES_SOURCE).await.unwrap().unwrap();
    let private = SourcePolicyConfig {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::from(["brain.legacy.review".into()]),
        ..public_policy()
    };
    let revoked = prepare_batch(&docs, &private, &context(), Some(&cp)).unwrap();
    assert_eq!(revoked.records.len(), 1);
    assert_eq!(revoked.records[0].revision, 2);
    assert_eq!(revoked.records[0].content, batch.records[0].content);
    assert_eq!(revoked.records[0].visibility, SourceVisibility::Private);
    let no_scope = SourcePolicyConfig {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::new(),
        ..public_policy()
    };
    assert!(prepare_batch(&docs, &no_scope, &context(), Some(&cp)).is_err());
}

#[test]
fn empty_read_and_foreign_checkpoints_fail_closed() {
    let empty = LegacySource {
        source_id: PATCHNOTES_SOURCE,
        documents: Vec::new(),
    };
    assert!(prepare_batch(&empty, &public_policy(), &context(), None).is_err());
    let docs = patch_documents(&[line(1, "p1", 0, "a")]).unwrap();
    let batch = prepare_batch(&docs, &public_policy(), &context(), None).unwrap();
    let mut foreign = batch.checkpoint.clone();
    foreign.source_id = ENTITIES_SOURCE.into();
    assert!(prepare_batch(&docs, &public_policy(), &context(), Some(&foreign)).is_err());
    let mut tampered = batch.checkpoint;
    tampered.configuration = "other".into();
    assert!(prepare_batch(&docs, &public_policy(), &context(), Some(&tampered)).is_err());
}

#[test]
fn release_id_is_deterministic_and_content_addressed() {
    let docs = patch_documents(&[line(1, "p1", 0, "a")]).unwrap();
    let batch = prepare_batch(&docs, &public_policy(), &context(), None).unwrap();
    let config = ReleaseConfig {
        id_prefix: "legacy-core".into(),
        knowledge_version: "kv".into(),
        patch: "legacy-archive".into(),
    };
    let a = release_from_checkpoints(std::slice::from_ref(&batch.checkpoint), &config, 5).unwrap();
    let b = release_from_checkpoints(std::slice::from_ref(&batch.checkpoint), &config, 5).unwrap();
    assert_eq!(a, b);
    let other = patch_documents(&[line(1, "p1", 0, "changed")]).unwrap();
    let changed = prepare_batch(
        &other,
        &public_policy(),
        &context(),
        Some(&batch.checkpoint),
    )
    .unwrap();
    let c = release_from_checkpoints(&[changed.checkpoint], &config, 5).unwrap();
    assert_ne!(a.release_id, c.release_id);
}

async fn scratch_snapshot(
    reader: &brain_storage::LocalPgReader,
    release: &str,
) -> brain_contracts::CorpusSnapshot {
    let reader = reader.clone();
    let release = release.to_owned();
    tokio::task::spawn_blocking(move || reader.read_snapshot(&release).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
#[ignore = "scripts/test_brain_serve.sh: synthetic legacy records in disposable peer-auth PostgreSQL"]
async fn scratch_import_release_tombstone_and_revoke() {
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

    let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
    assert!(socket.ends_with("/.core-test-pg") && std::path::Path::new(&socket).is_absolute());
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(&socket)
                .port(55439)
                .username("brain_core_test")
                .database("brain_legacy_test"),
        )
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(address.is_none());
    let store = brain_storage::PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    let patches = patch_documents(&[
        line(1, "p1", 0, "Abrams: first"),
        line(2, "p2", 0, "Warden: second"),
    ])
    .unwrap();
    let entities = entity_documents(&[entity(25, "Warden")]).unwrap();
    let patches_first = prepare_batch(&patches, &public_policy(), &context(), None).unwrap();
    let entities_first = prepare_batch(&entities, &public_policy(), &context(), None).unwrap();
    for batch in [&patches_first, &entities_first] {
        let lease = store
            .claim(&batch.checkpoint.source_id, "initial", 30_000)
            .await
            .unwrap();
        assert!(!store.commit(batch, &lease).await.unwrap().replayed);
        assert!(store.commit(batch, &lease).await.unwrap().replayed);
    }
    let patch_checkpoint = store.checkpoint(PATCHNOTES_SOURCE).await.unwrap().unwrap();
    let entity_checkpoint = store.checkpoint(ENTITIES_SOURCE).await.unwrap().unwrap();
    let same = prepare_batch(
        &patches,
        &public_policy(),
        &context(),
        Some(&patch_checkpoint),
    )
    .unwrap();
    assert!(same.records.is_empty());
    let config = ReleaseConfig {
        id_prefix: "legacy-fixture".into(),
        knowledge_version: "v1".into(),
        patch: "p1".into(),
    };
    let r1 = release_from_checkpoints(
        &[patch_checkpoint.clone(), entity_checkpoint.clone()],
        &config,
        1,
    )
    .unwrap();
    store.publish(&r1).await.unwrap();
    let reader =
        brain_storage::LocalPgReader::new(&socket, 55439, "brain_core_test", "brain_legacy_test")
            .unwrap();
    let snapshot = scratch_snapshot(&reader, &r1.release_id).await;
    assert_eq!(snapshot.revisions.len(), 3);
    let game = Principal {
        actor_id: "game-fixture".into(),
        channel: "test".into(),
        scopes: BTreeSet::from(["game.public".into()]),
        provider_egress: BTreeSet::new(),
    };
    assert_eq!(snapshot.authorized(&game, false).unwrap().len(), 3);
    let docs = Principal {
        scopes: BTreeSet::from(["docs.public".into()]),
        ..game.clone()
    };
    assert!(snapshot.authorized(&docs, false).unwrap().is_empty());
    let remaining = patch_documents(&[line(1, "p1", 0, "Abrams: first")]).unwrap();
    let removed = prepare_batch(
        &remaining,
        &public_policy(),
        &context(),
        Some(&patch_checkpoint),
    )
    .unwrap();
    assert_eq!(
        removed
            .records
            .iter()
            .filter(|record| record.tombstone)
            .count(),
        1
    );
    let lease = store
        .claim(PATCHNOTES_SOURCE, "removed", 30_000)
        .await
        .unwrap();
    store.commit(&removed, &lease).await.unwrap();
    let patch_checkpoint = store.checkpoint(PATCHNOTES_SOURCE).await.unwrap().unwrap();
    let r2 = release_from_checkpoints(
        &[patch_checkpoint.clone(), entity_checkpoint.clone()],
        &config,
        2,
    )
    .unwrap();
    store.publish(&r2).await.unwrap();
    assert_eq!(
        scratch_snapshot(&reader, &r2.release_id)
            .await
            .revisions
            .len(),
        2
    );
    let tombstones: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2 AND tombstone AND revision=2",
    )
    .bind(PATCHNOTES_SOURCE)
    .bind("patch/p2")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(tombstones, 1);
    let unchanged = prepare_batch(
        &remaining,
        &public_policy(),
        &context(),
        Some(&patch_checkpoint),
    )
    .unwrap();
    assert!(unchanged.records.is_empty());
    let restricted = SourcePolicyConfig {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::from(["brain.legacy.review".into()]),
        ..public_policy()
    };
    let revoke =
        prepare_batch(&entities, &restricted, &context(), Some(&entity_checkpoint)).unwrap();
    assert_eq!(revoke.records.len(), 1);
    let lease = store
        .claim(ENTITIES_SOURCE, "revoke", 30_000)
        .await
        .unwrap();
    store.commit(&revoke, &lease).await.unwrap();
    let current = scratch_snapshot(&reader, &r2.release_id).await;
    assert_eq!(current.authorized(&game, false).unwrap().len(), 1);
    assert!(current.authorized(&docs, false).unwrap().is_empty());
    let pinned = scratch_snapshot(&reader, &r1.release_id).await;
    assert_eq!(pinned.authorized(&game, false).unwrap().len(), 1);
    tokio::task::spawn_blocking(move || drop(reader))
        .await
        .unwrap();
    pool.close().await;
}

fn cutover_fixture() -> (
    pg::LegacyRead,
    [LegacySource; 2],
    BTreeMap<String, SourcePolicyConfig>,
    cutover::CutoverBinding,
) {
    let patches = vec![line(1, "p1", 0, "first"), line(2, "p2", 0, "second")];
    let entities = vec![entity(25, "Warden")];
    let read = pg::LegacyRead {
        patch_lines: patches.clone(),
        entities: entities.clone(),
        schema_sha256: "a".repeat(64),
        table_counts: pg::LEGACY_TABLES
            .into_iter()
            .map(|table| (table.to_string(), 1))
            .collect(),
    };
    let sources = [
        entity_documents(&entities).unwrap(),
        patch_documents(&patches).unwrap(),
    ];
    let approval = format!("sha256:{}", "b".repeat(64));
    let mut entity_policy = public_policy();
    entity_policy.authorization_ref = Some(approval.clone());
    let mut patch_policy = public_policy();
    patch_policy.visibility = SourceVisibility::Private;
    patch_policy.allowed_scopes = BTreeSet::from(["brain.legacy.review".into()]);
    patch_policy.authorization_ref = Some(approval.clone());
    let policies = BTreeMap::from([
        (ENTITIES_SOURCE.to_string(), entity_policy),
        (PATCHNOTES_SOURCE.to_string(), patch_policy),
    ]);
    let active_ids = sources
        .iter()
        .map(|source| {
            (
                source.source_id.to_string(),
                source
                    .documents
                    .iter()
                    .map(|d| d.logical_id.clone())
                    .collect(),
            )
        })
        .collect();
    let empty_ids = BTreeMap::from([
        (ENTITIES_SOURCE.to_string(), BTreeSet::new()),
        (PATCHNOTES_SOURCE.to_string(), BTreeSet::new()),
    ]);
    let mut binding = cutover::CutoverBinding {
        approval_ref: approval,
        database_oid: 1,
        archive_schema_oid: 2,
        core_schema_oid: 3,
        schema_sha256: read.schema_sha256.clone(),
        snapshot_sha256: cutover::snapshot_sha256(
            &sources,
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
        )
        .unwrap(),
        policy_sha256: "0".repeat(64),
        table_counts: read.table_counts.clone(),
        active_ids,
        revoked_ids: empty_ids.clone(),
        tombstone_ids: empty_ids,
    };
    binding.policy_sha256 = binding.policy_sha256(&policies).unwrap();
    (read, sources, policies, binding)
}

#[test]
fn cutover_requires_complete_private_policy_and_exact_snapshot() {
    let (read, mut sources, policies, binding) = cutover_fixture();
    let mut missing_baseline = binding.clone();
    missing_baseline.policy_sha256.clear();
    assert!(missing_baseline
        .verify_sources(
            &mut sources.clone(),
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &policies,
        )
        .is_err());
    let mut wrong_snapshot = binding.clone();
    wrong_snapshot.snapshot_sha256 = "0".repeat(64);
    assert!(wrong_snapshot
        .verify_sources(
            &mut sources.clone(),
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &policies,
        )
        .is_err());
    let mut changed_read = pg::LegacyRead {
        patch_lines: read.patch_lines.clone(),
        entities: read.entities.clone(),
        schema_sha256: read.schema_sha256.clone(),
        table_counts: read.table_counts.clone(),
    };
    changed_read.entities[0].metadata = serde_json::json!({"hero": "unused"});
    assert!(binding
        .verify_sources(
            &mut sources.clone(),
            &changed_read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &policies,
        )
        .is_err());
    let mut exposed = policies.clone();
    exposed.get_mut(PATCHNOTES_SOURCE).unwrap().visibility = SourceVisibility::Public;
    assert!(binding
        .verify_sources(
            &mut sources.clone(),
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &exposed,
        )
        .is_err());
    binding
        .verify_sources(
            &mut sources,
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &policies,
        )
        .unwrap();
    let batch = prepare_batch(&sources[1], &policies[PATCHNOTES_SOURCE], &context(), None).unwrap();
    let origin = origin_from_record(&batch.records[0]).unwrap();
    assert_eq!(
        origin.policy.authorization_ref,
        Observed::known(binding.approval_ref)
    );
    assert_eq!(batch.records[0].visibility, SourceVisibility::Private);
}

#[test]
fn cutover_excludes_approved_tombstones_and_rejects_resurrection() {
    let (read, mut sources, policies, mut binding) = cutover_fixture();
    binding
        .active_ids
        .get_mut(PATCHNOTES_SOURCE)
        .unwrap()
        .remove("patch/p2");
    binding
        .tombstone_ids
        .get_mut(PATCHNOTES_SOURCE)
        .unwrap()
        .insert("patch/p2".into());
    binding.policy_sha256 = binding.policy_sha256(&policies).unwrap();
    binding
        .verify_sources(
            &mut sources,
            &read,
            &context().snapshot_label,
            context().snapshot_epoch,
            &policies,
        )
        .unwrap();
    let batch = prepare_batch(&sources[1], &policies[PATCHNOTES_SOURCE], &context(), None).unwrap();
    assert_eq!(batch.records.len(), 1);
    assert_eq!(batch.records[0].logical_id, "patch/p1");
    binding
        .verify_head(&batch.records[0], &sources, &policies, &context())
        .unwrap();
    let mut resurrected = batch.records[0].clone();
    resurrected.logical_id = "patch/p2".into();
    assert!(binding
        .verify_head(&resurrected, &sources, &policies, &context())
        .is_err());
    let mut revoked_scope = batch.records[0].clone();
    revoked_scope.visibility = SourceVisibility::Public;
    assert!(binding
        .verify_head(&revoked_scope, &sources, &policies, &context())
        .is_err());
    let mut changed_origin = batch.records[0].clone();
    changed_origin
        .metadata
        .insert("locator".into(), "other".into());
    assert!(binding
        .verify_head(&changed_origin, &sources, &policies, &context())
        .is_err());
    let mut changed_content = batch.records[0].clone();
    changed_content.content.push_str("different");
    assert!(binding
        .verify_head(&changed_content, &sources, &policies, &context())
        .is_err());
}
