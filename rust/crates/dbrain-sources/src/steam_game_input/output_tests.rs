use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::MetadataExt,
};

use super::*;
use serde_json::json;

fn empty_prepared(parent: &Path) -> PreparedSteamGameInput {
    let options = GameFileOptions {
        root: parent.to_owned(),
        app_id: 1422450,
        source_id: "test-steam".into(),
        observed_at: "2026-10-03T12:00:00Z".into(),
        build_id: Some("123".into()),
        manifest_id: Some("456".into()),
        source_revision: None,
        depot_id: Some(1422451),
        language: "en".into(),
        attribution: "Valve".into(),
        license_name: "unverified".into(),
        license_url: None,
        provenance: json!({}),
        max_file_bytes: 1024,
    };
    let depot: DepotInventory = serde_json::from_value(json!({
        "app_id":1422450,"build_id":123,"depot_id":1422451,"manifest_id":"456",
        "manifest_sha256":"11".repeat(32),"root":parent,"observed_at":options.observed_at,
        "branch":"public","platform":"linux","languages":["english"],
        "status":"complete","expected_bytes":0,"files":[],
        "provenance":{"source":"steam","source_layout":"resource_paths",
            "app_id":1422450,"build_id":123,"depot_id":1422451,"manifest_id":"456"}
    }))
    .unwrap();
    let download: DownloadInventory = serde_json::from_value(json!({
        "app_id":1422450,"build_id":123,"observed_at":options.observed_at,
        "branch":"public","platform":"linux","languages":["english"],
        "status":"complete","expected_bytes":0,"depots":[depot],
        "limits":{"download_bytes":96_u64*1024*1024*1024,"holding_bytes":192_u64*1024*1024*1024,
            "reserve_bytes":16_u64*1024*1024*1024,"metadata_reserve_bytes":1024}
    }))
    .unwrap();
    PreparedSteamGameInput {
        input: BoundGameFiles {
            options,
            manifest_transport_sha256: [17; 32],
            files: Vec::new(),
        },
        evidence: SteamGameInputEvidence {
            inventory_sha256: [0; 32],
            depot_inventory_sha256: [0; 32],
            manifest_transport_sha256: [17; 32],
            download_inventory: download,
            depot_inventory: depot,
            copied_bytes: 0,
            copied_files: 0,
        },
        limits: SteamGameInputLimits {
            max_inventory_bytes: 1024,
            max_total_bytes: 1024,
            max_file_bytes: 1024,
            max_entries: 100,
            max_path_bytes: 4096,
            max_depth: 64,
            max_open_files: 200,
            fd_reserve: 144,
            reserve_bytes: 0,
            max_output_bytes: 1024,
        },
    }
}

#[test]
fn extractor_error_discards_written_prefix_and_never_publishes_evidence() {
    let parent = tempfile::tempdir().unwrap();
    let destination = parent.path().join("artifact");
    let result = empty_prepared(parent.path()).extract_with(&destination, |_, writer| {
        writer.write_all(b"partial jsonl\n")?;
        Err(io::Error::other("Extraktorfehler nach Teiloutput"))
    });
    assert!(result.is_err());
    assert!(!destination.exists());
    assert_eq!(fs::read_dir(parent.path()).unwrap().count(), 0);
}

#[test]
fn readonly_snapshot_is_unlinked_and_has_no_write_handle() {
    let parent = tempfile::tempdir().unwrap();
    let staging = filesystem::PrivateDirectory::new(parent.path()).unwrap();
    let mut writer = staging.create_file("snapshot").unwrap();
    writer.write_all(b"private bytes").unwrap();
    let mut readonly = staging.detach_readonly(writer, "snapshot").unwrap();
    assert_eq!(readonly.metadata().unwrap().nlink(), 0);
    assert!(readonly.write_all(b"overwrite").is_err());
    drop(staging);
    let mut bytes = Vec::new();
    readonly.read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, b"private bytes");
}

#[test]
fn held_original_identity_rejects_same_size_path_replacement() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("original");
    fs::write(&path, b"original").unwrap();
    let (mut held, identity) = filesystem::open_explicit_file(&path).unwrap();
    fs::rename(&path, parent.path().join("old")).unwrap();
    fs::write(&path, b"replaced").unwrap();
    let root = filesystem::open_root(parent.path()).unwrap();
    assert!(filesystem::open_bound_file(&root, "original", &identity).is_err());
    let mut bytes = Vec::new();
    held.read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, b"original");
}

#[test]
fn same_inode_mutation_and_truncation_do_not_create_snapshot_success() {
    for replacement in [b"mutated!".as_slice(), b"short".as_slice()] {
        let parent = tempfile::tempdir().unwrap();
        let original_path = parent.path().join("original");
        fs::write(&original_path, b"original").unwrap();
        let (mut original, identity) = filesystem::open_explicit_file(&original_path).unwrap();
        fs::write(&original_path, replacement).unwrap();
        let mut snapshot = tempfile::tempfile_in(parent.path()).unwrap();
        assert!(copy_verified(
            &mut original,
            &mut snapshot,
            8,
            Sha1::digest(b"original").into(),
            Sha256::digest(b"original").into()
        )
        .is_err());
        assert!(filesystem::check_file(&original, &identity).is_err());
    }
}
