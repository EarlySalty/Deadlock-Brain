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
