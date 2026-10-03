use brain_contracts::{
    source::{origin_from_record, GameValidity},
    CorpusRelease, SourceVisibility,
};
use brain_feeds::{
    sheet_core::{self, SheetSource, SheetTabBody},
    source_sync::{candidate_release, SourceRights},
    youtube_core::{self, ExistingVideo, Retention, YoutubeSource},
    FeedPolicy,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

fn rights() -> SourceRights {
    SourceRights {
        ingestion_allowed: true,
        authorization_ref: Some("decision:test-only".into()),
        license: None,
        history_retention_months: 12,
        policy: FeedPolicy {
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::from(["source.review".into()]),
            provider_egress_allowed: false,
            publication_allowed: false,
            raw_retention_allowed: true,
        },
    }
}
fn sheet() -> SheetSource {
    SheetSource {
        sheet_id: "fixture".into(),
        gids: BTreeSet::from(["0".into(), "42".into()]),
        rights: rights(),
    }
}
fn tabs() -> Vec<SheetTabBody> {
    vec![
        SheetTabBody {
            gid: "0".into(),
            bytes: b"Hero,Value\nHaze,1\n".to_vec(),
            complete: true,
        },
        SheetTabBody {
            gid: "42".into(),
            bytes: b"Item,Value\nTest,2\n".to_vec(),
            complete: true,
        },
    ]
}
fn youtube() -> YoutubeSource {
    YoutubeSource {
        channel_id: "UCabcdefghijklmnopqrstuv".into(),
        retention: Retention::TranscriptAndMetadata,
        rights: rights(),
    }
}
fn video() -> ExistingVideo {
    ExistingVideo {
        video_id: "abcdefghijk".into(),
        channel_id: youtube().channel_id,
        title: "Fixture".into(),
        description: None,
        published_at: Some(1),
        language: Some("de".into()),
        transcript: Some("Beleg".into()),
        transcript_kind: Some("existing-caption".into()),
        transcript_hash: Some(hex::encode(Sha256::digest(b"Beleg"))),
    }
}

#[test]
fn sheet_is_idempotent_and_never_promotes_untyped_data() {
    let first = sheet_core::prepare_batch(&sheet(), &tabs(), 1, None).unwrap();
    assert_eq!(first.records.len(), 2);
    assert!(first
        .records
        .iter()
        .all(|r| r.metadata["kind"] == "prose" && r.valid_from.is_none()));
    assert_eq!(
        origin_from_record(&first.records[0]).unwrap().validity,
        GameValidity::unknown()
    );
    assert!(
        sheet_core::prepare_batch(&sheet(), &tabs(), 2, Some(&first.checkpoint))
            .unwrap()
            .records
            .is_empty()
    );
}

#[test]
fn incomplete_sheet_or_youtube_never_prepares_tombstones() {
    let first = sheet_core::prepare_batch(&sheet(), &tabs(), 1, None).unwrap();
    assert!(sheet_core::prepare_batch(&sheet(), &tabs()[..1], 2, Some(&first.checkpoint)).is_err());
    let mut partial = tabs();
    partial[0].complete = false;
    assert!(sheet_core::prepare_batch(&sheet(), &partial, 2, Some(&first.checkpoint)).is_err());
    partial[0].complete = true;
    partial[0].bytes = b"Hero,Value\nHaze,\"unterbrochen".to_vec();
    assert!(sheet_core::prepare_batch(&sheet(), &partial, 2, Some(&first.checkpoint)).is_err());
    let first = youtube_core::prepare_batch(&youtube(), &[video()], true, 1, None).unwrap();
    assert!(
        youtube_core::prepare_batch(&youtube(), &[video()], false, 2, Some(&first.checkpoint))
            .is_err()
    );
    assert!(
        youtube_core::prepare_batch(&youtube(), &[], true, 2, Some(&first.checkpoint)).is_err()
    );
}

#[test]
fn rights_change_revises_unchanged_content_and_missing_rights_fail_closed() {
    let first = youtube_core::prepare_batch(&youtube(), &[video()], true, 1, None).unwrap();
    assert!(
        youtube_core::prepare_batch(&youtube(), &[video()], true, 2, Some(&first.checkpoint))
            .unwrap()
            .records
            .is_empty()
    );
    let mut changed = youtube();
    changed.rights.policy.allowed_scopes = BTreeSet::from(["restricted.review".into()]);
    let update =
        youtube_core::prepare_batch(&changed, &[video()], true, 2, Some(&first.checkpoint))
            .unwrap();
    assert_eq!(update.records[0].revision, 2);
    assert_eq!(
        update.records[0].content_hash,
        first.records[0].content_hash
    );
    let origin = origin_from_record(&update.records[0]).unwrap();
    assert!(!origin.policy.provider_egress_allowed && !origin.policy.publication_allowed);
    assert!(matches!(
        origin.policy.license,
        brain_contracts::value::Observed::Unknown { .. }
    ));
    for kind in ["authorization", "ingestion", "retention"] {
        let mut denied = youtube();
        match kind {
            "authorization" => denied.rights.authorization_ref = None,
            "ingestion" => denied.rights.ingestion_allowed = false,
            _ => denied.retention = Retention::Disabled,
        }
        assert!(
            youtube_core::prepare_batch(&denied, &[video()], true, 2, Some(&first.checkpoint))
                .is_err()
        );
    }
}

#[test]
fn metadata_only_does_not_retain_transcripts_or_accept_bad_transcript_hashes() {
    let mut source = youtube();
    source.retention = Retention::MetadataOnly;
    let batch = youtube_core::prepare_batch(&source, &[video()], true, 1, None).unwrap();
    assert!(!batch.records[0].content.contains("Beleg"));
    let mut bad = video();
    bad.transcript_hash = Some("0".repeat(64));
    assert!(youtube_core::prepare_batch(&youtube(), &[bad], true, 1, None).is_err());
}

#[test]
fn configured_rights_cannot_weaken_the_operator_decision() {
    for field in [
        "public",
        "private",
        "publication",
        "egress",
        "license",
        "history",
        "wildcard",
        "scope",
        "raw",
    ] {
        let mut changed = rights();
        match field {
            "public" => changed.policy.visibility = SourceVisibility::Public,
            "private" => changed.policy.visibility = SourceVisibility::Private,
            "publication" => changed.policy.publication_allowed = true,
            "egress" => changed.policy.provider_egress_allowed = true,
            "license" => changed.license = Some("invented".into()),
            "history" => changed.history_retention_months = 13,
            "wildcard" => changed.policy.allowed_scopes = BTreeSet::from(["*".into()]),
            "scope" => changed.policy.allowed_scopes.clear(),
            _ => changed.policy.raw_retention_allowed = false,
        }
        assert!(changed.validate().is_err(), "{field}");
    }
    let mut omitted = serde_json::to_value(rights()).unwrap();
    omitted
        .as_object_mut()
        .unwrap()
        .remove("history_retention_months");
    assert!(serde_json::from_value::<SourceRights>(omitted)
        .unwrap()
        .validate()
        .is_err());
    assert!(brain_feeds::source_sync::require_history_retention().is_err());
}

#[test]
fn removed_sheet_rows_leave_the_active_document() {
    let first = sheet_core::prepare_batch(&sheet(), &tabs(), 1, None).unwrap();
    let mut changed = tabs();
    changed[0].bytes = b"Hero,Value\n".to_vec();
    let update = sheet_core::prepare_batch(&sheet(), &changed, 2, Some(&first.checkpoint)).unwrap();
    assert_eq!(update.records.len(), 1);
    assert_eq!(update.records[0].logical_id, first.records[0].logical_id);
    assert_eq!(update.records[0].revision, 2);
    assert!(!update.records[0].content.contains("Haze"));
}

#[test]
fn removed_videos_are_tombstoned_when_another_video_proves_a_nonempty_complete_set() {
    let source = youtube();
    let mut second = video();
    second.video_id = "lmnopqrstuv".into();
    let first =
        youtube_core::prepare_batch(&source, &[video(), second.clone()], true, 1, None).unwrap();
    let update =
        youtube_core::prepare_batch(&source, &[second], true, 2, Some(&first.checkpoint)).unwrap();
    assert_eq!(update.records.len(), 1);
    assert!(update.records[0].tombstone);
    assert!(update.records[0].content.is_empty());
    let base = CorpusRelease {
        release_id: "base".into(),
        knowledge_version: "v1".into(),
        patch: "patch".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::new(),
    };
    let release = candidate_release(&base, &[update], 2).unwrap();
    assert!(!release.source_revisions[&source.source_id()].contains_key("video/abcdefghijk"));
}

#[test]
fn release_preserves_all_foreign_pins_and_patch() {
    let base = CorpusRelease {
        release_id: "explicit-base".into(),
        knowledge_version: "v1".into(),
        patch: "real-patch".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "foreign-source".into(),
            BTreeMap::from([("foreign-document".into(), 7)]),
        )]),
    };
    let batches = vec![
        sheet_core::prepare_batch(&sheet(), &tabs(), 1, None).unwrap(),
        youtube_core::prepare_batch(&youtube(), &[video()], true, 1, None).unwrap(),
    ];
    let release = candidate_release(&base, &batches, 2).unwrap();
    assert_eq!(
        release.source_revisions["foreign-source"],
        base.source_revisions["foreign-source"]
    );
    assert_eq!(release.patch, base.patch);
    assert_ne!(release.release_id, base.release_id);
    assert_eq!(
        release.release_id,
        candidate_release(&base, &batches, 3).unwrap().release_id
    );
}
