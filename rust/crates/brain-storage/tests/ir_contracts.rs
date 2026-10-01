use brain_contracts::{
    source::{
        origin_from_record, GameValidity, OriginArtifact, SourceIdentity, SourcePolicy,
        SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    CorpusRelease, DocumentStorePort, Principal, SnapshotReadPort, SourceRecordV2,
    SourceVisibility,
};
use brain_storage::MemoryRepository;
use std::collections::{BTreeMap, BTreeSet};

fn source(revision: u64, scope: &str, egress: bool) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: "ir-fixture".into(),
        logical_id: "raw:1".into(),
        revision,
        content_hash: "a".repeat(64),
        content: "synthetic private raw".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from([scope.into()]),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    let origin = OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Git {
            commit: "b".repeat(40),
        },
        raw_sha256: record.content_hash.clone(),
        locator: "git:repo/field.json".into(),
        parser_revision: "fixture-parser:2".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::known("fixture:1".into()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(123)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: Observed::known("operator:1".into()),
            license: Observed::unknown(UnknownReason::NotPresent),
            publication_allowed: false,
            provider_egress_allowed: egress,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    };
    origin.bind_record(&mut record).unwrap();
    record
}
fn principal(scopes: &[&str]) -> Principal {
    Principal {
        actor_id: "fixture-reader".into(),
        channel: "test".into(),
        scopes: scopes.iter().map(|s| (*s).into()).collect(),
        provider_egress: BTreeSet::from(["internal".into()]),
    }
}
async fn publish(store: &MemoryRepository) {
    store
        .publish(&CorpusRelease {
            release_id: "ir-release".into(),
            knowledge_version: "ir-fixture:1".into(),
            patch: "p1".into(),
            created_at_epoch: 123,
            source_revisions: BTreeMap::from([(
                "ir-fixture".into(),
                BTreeMap::from([("raw:1".into(), 1)]),
            )]),
        })
        .await
        .unwrap();
}
#[tokio::test]
async fn actual_store_retains_common_origin_and_denies_unapproved_provider_egress() {
    let store = MemoryRepository::default();
    let original = source(1, "review", false);
    store.apply_record(original.clone()).unwrap();
    publish(&store).await;
    let snapshot = store.read_snapshot("ir-release").unwrap();
    assert_eq!(snapshot.revisions, vec![original.clone()]);
    let local = snapshot.authorized(&principal(&["review"]), false).unwrap();
    assert_eq!(
        origin_from_record(&local[0]).unwrap(),
        origin_from_record(&original).unwrap()
    );
    assert!(snapshot
        .authorized(&principal(&[]), false)
        .unwrap()
        .is_empty());
    assert!(snapshot
        .authorized(&principal(&["review"]), true)
        .unwrap()
        .is_empty());
}
#[tokio::test]
async fn old_release_cannot_restore_revoked_egress_or_scopes() {
    let store = MemoryRepository::default();
    store.apply_record(source(1, "review", true)).unwrap();
    publish(&store).await;
    assert_eq!(
        store
            .read_snapshot("ir-release")
            .unwrap()
            .authorized(&principal(&["review"]), true)
            .unwrap()
            .len(),
        1
    );
    store.apply_record(source(2, "restricted", false)).unwrap();
    let snapshot = store.read_snapshot("ir-release").unwrap();
    assert!(snapshot
        .authorized(&principal(&["review"]), false)
        .unwrap()
        .is_empty());
    assert!(snapshot
        .authorized(&principal(&["review", "restricted"]), true)
        .unwrap()
        .is_empty());
    let local = snapshot
        .authorized(&principal(&["review", "restricted"]), false)
        .unwrap();
    let origin = origin_from_record(&local[0]).unwrap();
    assert_eq!(
        origin.policy.allowed_scopes,
        BTreeSet::from(["review".into(), "restricted".into()])
    );
    assert!(!origin.policy.provider_egress_allowed);
    assert!(matches!(origin.validity.patch, Observed::Unknown { .. }));
    assert_eq!(local[0].revision, 1);
}

#[tokio::test]
async fn legacy_head_cannot_supply_missing_rights_for_versioned_origin() {
    let store = MemoryRepository::default();
    store.apply_record(source(1, "review", true)).unwrap();
    publish(&store).await;
    let mut legacy_head = source(2, "review", true);
    legacy_head.metadata.clear();
    store.apply_record(legacy_head).unwrap();
    let snapshot = store.read_snapshot("ir-release").unwrap();
    let local = snapshot.authorized(&principal(&["review"]), false).unwrap();
    assert!(
        !origin_from_record(&local[0])
            .unwrap()
            .policy
            .provider_egress_allowed
    );
    assert!(snapshot
        .authorized(&principal(&["review"]), true)
        .unwrap()
        .is_empty());
}
