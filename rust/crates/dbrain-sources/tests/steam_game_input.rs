use std::{
    fs,
    io::Read,
    os::unix::fs::{symlink, PermissionsExt},
    path::PathBuf,
};

use dbrain_sources::{
    game_files::GameFileOptions,
    steam_game_input::{prepare_steam_game_input, SteamGameInputLimits, SteamGameInputOptions},
};
use serde_json::{json, Value};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

struct Fixture {
    base: TempDir,
    options: SteamGameInputOptions,
    extraction: GameFileOptions,
    download: Value,
    depot: Value,
}

impl Fixture {
    fn new() -> Self {
        let base = tempfile::tempdir().unwrap();
        fs::set_permissions(base.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let root = base.path().join("depot");
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("scripts")).unwrap();
        let text = b"\"hero\" { \"damage\" \"12\" }\n";
        let asset = b"opaque asset";
        fs::write(root.join("scripts/hero.txt"), text).unwrap();
        fs::write(root.join("asset.bin"), asset).unwrap();
        let provenance = json!({"source":"steam","source_layout":"resource_paths",
            "app_id":1422450,"build_id":123,"depot_id":1422451,"manifest_id":"456"});
        let depot = json!({"app_id":1422450,"build_id":123,"depot_id":1422451,
            "manifest_id":"456","manifest_sha256":"11".repeat(32),"root":root,
            "observed_at":"2026-10-03T12:00:00.000Z","branch":"public","platform":"linux",
            "languages":["english","german"],"status":"complete",
            "expected_bytes":text.len()+asset.len(),"provenance":provenance,
            "files":[{"path":"scripts","kind":"directory","size":0},
                file_entry("scripts/hero.txt",text),file_entry("asset.bin",asset)]});
        let download = json!({"app_id":1422450,"build_id":123,
            "observed_at":"2026-10-03T12:00:00.000Z","branch":"public","platform":"linux",
            "languages":["english","german"],"status":"complete",
            "expected_bytes":text.len()+asset.len(),"depots":[depot.clone()],
            "limits":{"download_bytes":96_u64*1024*1024*1024,
                "holding_bytes":192_u64*1024*1024*1024,"reserve_bytes":16_u64*1024*1024*1024,
                "metadata_reserve_bytes":1024*1024}});
        let options = SteamGameInputOptions {
            inventory_path: base.path().join("download.json"),
            inventory_sha256: [0; 32],
            depot_inventory_path: base.path().join("depot.json"),
            depot_inventory_sha256: [0; 32],
            root: root.clone(),
            app_id: 1422450,
            build_id: 123,
            depot_id: 1422451,
            manifest_id: "456".into(),
            snapshot_directory: base.path().to_owned(),
            limits: SteamGameInputLimits {
                max_inventory_bytes: 1024 * 1024,
                max_total_bytes: 1024 * 1024,
                max_file_bytes: 1024 * 1024,
                max_entries: 100,
                max_path_bytes: 4096,
                max_depth: 64,
                max_open_files: 200,
                fd_reserve: 144,
                reserve_bytes: 0,
                max_output_bytes: 1024 * 1024,
            },
        };
        let extraction = GameFileOptions {
            root,
            app_id: 1422450,
            source_id: "steam-deadlock".into(),
            observed_at: "2026-10-03T12:00:00.000Z".into(),
            build_id: Some("123".into()),
            manifest_id: Some("456".into()),
            source_revision: None,
            depot_id: Some(1422451),
            language: "en".into(),
            attribution: "Valve".into(),
            license_name: "unverified".into(),
            license_url: None,
            provenance,
            max_file_bytes: 1024 * 1024,
        };
        let mut fixture = Self {
            base,
            options,
            extraction,
            download,
            depot,
        };
        fixture.repin();
        fixture
    }

    fn repin(&mut self) {
        self.download["depots"][0] = self.depot.clone();
        self.write_pins();
    }

    fn write_pins(&mut self) {
        let total = serde_json::to_vec(&self.download).unwrap();
        let depot = serde_json::to_vec(&self.depot).unwrap();
        fs::write(&self.options.inventory_path, &total).unwrap();
        fs::write(&self.options.depot_inventory_path, &depot).unwrap();
        self.options.inventory_sha256 = Sha256::digest(total).into();
        self.options.depot_inventory_sha256 = Sha256::digest(depot).into();
    }

    fn output(&self) -> PathBuf {
        self.base.path().join("artifact")
    }

    fn fails(&self) {
        assert!(prepare_steam_game_input(&self.options, self.extraction.clone()).is_err());
        assert!(!self.output().exists());
        assert!(fs::read_dir(self.base.path()).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".steam-input-")));
    }
}

fn file_entry(path: &str, bytes: &[u8]) -> Value {
    json!({"path":path,"kind":"file","size":bytes.len(),
        "sha1":hex::encode(Sha1::digest(bytes)),"sha256":hex::encode(Sha256::digest(bytes))})
}

#[test]
fn complete_inventory_and_asset_handles_survive_original_mutation() {
    let fixture = Fixture::new();
    let prepared = prepare_steam_game_input(&fixture.options, fixture.extraction.clone()).unwrap();
    assert_eq!(prepared.evidence().copied_files, 2);
    assert_eq!(prepared.evidence().depot_inventory.files.len(), 3);
    assert_eq!(prepared.evidence().download_inventory.depots.len(), 1);
    fs::write(fixture.options.root.join("scripts/hero.txt"), b"changed").unwrap();
    fs::write(fixture.options.root.join("asset.bin"), b"changed").unwrap();
    fs::rename(&fixture.options.root, fixture.base.path().join("old-depot")).unwrap();
    symlink(fixture.base.path().join("old-depot"), &fixture.options.root).unwrap();
    let result = prepared.extract_to(&fixture.output()).unwrap();
    let mut jsonl = String::new();
    fs::File::open(result.jsonl_path)
        .unwrap()
        .read_to_string(&mut jsonl)
        .unwrap();
    assert!(jsonl.contains("damage"));
    assert!(!jsonl.contains("changed"));
    assert_eq!(result.extractor_inventory.documents, 1);
    let evidence: Value = serde_json::from_slice(&fs::read(result.evidence_path).unwrap()).unwrap();
    assert_eq!(evidence["input"]["copied_files"], 2);
    assert_eq!(
        evidence["input"]["depot_inventory"]["files"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        fs::metadata(result.directory).unwrap().permissions().mode() & 0o777,
        0o700
    );
}

#[test]
fn both_pinned_inventory_hashes_are_mandatory() {
    let mut fixture = Fixture::new();
    fixture.options.inventory_sha256[0] ^= 1;
    fixture.fails();
    fixture.write_pins();
    fixture.options.depot_inventory_sha256[0] ^= 1;
    fixture.fails();
}

#[test]
fn partial_depot_proof_cannot_replace_embedded_full_proof() {
    let mut fixture = Fixture::new();
    fixture.depot["files"].as_array_mut().unwrap().pop();
    fixture.write_pins();
    fixture.fails();
}

#[test]
fn every_expected_identity_and_extraction_origin_must_match() {
    let mut fixture = Fixture::new();
    fixture.options.build_id += 1;
    fixture.fails();
    fixture.options.build_id -= 1;
    fixture.options.manifest_id = "0456".into();
    fixture.fails();
    fixture.options.manifest_id = "456".into();
    fixture.extraction.provenance["source"] = json!("git");
    fixture.fails();
}

#[test]
fn steam_sha1_and_sha256_are_independently_checked_on_copied_bytes() {
    let mut fixture = Fixture::new();
    fixture.depot["files"][1]["sha1"] = json!("00".repeat(20));
    fixture.repin();
    fixture.fails();
    let original = fs::read(fixture.options.root.join("scripts/hero.txt")).unwrap();
    fixture.depot["files"][1] = file_entry("scripts/hero.txt", &original);
    fixture.depot["files"][1]["sha256"] = json!("00".repeat(32));
    fixture.repin();
    fixture.fails();
}

#[test]
fn missing_truncated_and_extended_files_are_rejected() {
    let fixture = Fixture::new();
    let asset = fixture.options.root.join("asset.bin");
    fs::write(&asset, b"short").unwrap();
    fixture.fails();
    fs::write(&asset, b"much longer than the original asset").unwrap();
    fixture.fails();
    fs::remove_file(&asset).unwrap();
    fixture.fails();
}

#[test]
fn extras_are_not_opened_as_manifest_files_or_deleted() {
    let fixture = Fixture::new();
    let extra = fixture.options.root.join("foreign.bin");
    fs::write(&extra, b"foreign").unwrap();
    fixture.fails();
    assert_eq!(fs::read(extra).unwrap(), b"foreign");
}

#[test]
fn symlinks_hardlinks_and_directory_type_changes_fail_closed() {
    let fixture = Fixture::new();
    let asset = fixture.options.root.join("asset.bin");
    fs::rename(&asset, fixture.base.path().join("asset-original")).unwrap();
    symlink(fixture.base.path().join("asset-original"), &asset).unwrap();
    fixture.fails();
    fs::remove_file(&asset).unwrap();
    fs::hard_link(fixture.base.path().join("asset-original"), &asset).unwrap();
    fixture.fails();
    fs::remove_file(&asset).unwrap();
    fs::rename(fixture.base.path().join("asset-original"), &asset).unwrap();
    fs::rename(
        fixture.options.root.join("scripts"),
        fixture.base.path().join("scripts-original"),
    )
    .unwrap();
    symlink(
        fixture.base.path().join("scripts-original"),
        fixture.options.root.join("scripts"),
    )
    .unwrap();
    fixture.fails();
}

#[test]
fn duplicate_traversal_and_file_parent_paths_are_rejected() {
    for path in [
        "scripts/../asset.bin",
        "scripts//hero.txt",
        "scripts/./hero.txt",
        "/asset.bin",
        "scripts\\hero.txt",
        "scripts:hero.txt",
        ".download-temp",
    ] {
        let mut fixture = Fixture::new();
        fixture.depot["files"][1]["path"] = json!(path);
        fixture.repin();
        fixture.fails();
    }
    let mut fixture = Fixture::new();
    fixture.depot["files"][2] = fixture.depot["files"][1].clone();
    fixture.repin();
    fixture.fails();
}

#[test]
fn configured_disk_fd_entry_and_byte_limits_are_preflighted() {
    let mut fixture = Fixture::new();
    fixture.options.limits.reserve_bytes = u64::MAX;
    fixture.fails();
    fixture.options.limits.reserve_bytes = 0;
    fixture.options.limits.max_open_files = 1;
    fixture.fails();
    fixture.options.limits.max_open_files = 200;
    fixture.options.limits.max_entries = 1;
    fixture.fails();
    fixture.options.limits.max_entries = 100;
    fixture.options.limits.max_file_bytes = 1;
    fixture.fails();
}

#[test]
fn missing_bound_vpk_companion_discards_preceding_text_output() {
    let mut fixture = Fixture::new();
    let mut tree = b"txt\0scripts\0items\0".to_vec();
    tree.extend(0_u32.to_le_bytes());
    tree.extend(0_u16.to_le_bytes());
    tree.extend(0_u16.to_le_bytes());
    tree.extend(0_u32.to_le_bytes());
    tree.extend(4_u32.to_le_bytes());
    tree.extend(0xffff_u16.to_le_bytes());
    tree.extend([0, 0, 0]);
    let mut vpk = Vec::new();
    vpk.extend(0x55aa1234_u32.to_le_bytes());
    vpk.extend(1_u32.to_le_bytes());
    vpk.extend((tree.len() as u32).to_le_bytes());
    vpk.extend(tree);
    fs::write(fixture.options.root.join("pak01_dir.vpk"), &vpk).unwrap();
    fixture.depot["files"]
        .as_array_mut()
        .unwrap()
        .push(file_entry("pak01_dir.vpk", &vpk));
    let bytes = fixture.depot["expected_bytes"].as_u64().unwrap() + vpk.len() as u64;
    fixture.depot["expected_bytes"] = json!(bytes);
    fixture.download["expected_bytes"] = json!(bytes);
    fixture.repin();
    let prepared = prepare_steam_game_input(&fixture.options, fixture.extraction.clone()).unwrap();
    assert!(prepared.extract_to(&fixture.output()).is_err());
    assert!(!fixture.output().exists());
}

#[test]
fn normal_options_loader_is_bounded_and_accepts_only_strict_hex_pins() {
    let fixture = Fixture::new();
    let path = fixture.base.path().join("options.json");
    let mut value = serde_json::to_value(&fixture.options).unwrap();
    assert_eq!(value["inventory_sha256"].as_str().unwrap().len(), 64);
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    let loaded = SteamGameInputOptions::from_file(&path, 1024 * 1024).unwrap();
    assert_eq!(loaded.inventory_sha256, fixture.options.inventory_sha256);
    assert!(SteamGameInputOptions::from_file(&path, 1).is_err());
    value["inventory_sha256"] = json!("AA".repeat(32));
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(SteamGameInputOptions::from_file(&path, 1024 * 1024).is_err());
    value["inventory_sha256"] = json!(hex::encode(fixture.options.inventory_sha256));
    value["unknown_option"] = json!(true);
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(SteamGameInputOptions::from_file(&path, 1024 * 1024).is_err());
}

#[test]
fn artifact_is_never_overwritten_and_partial_output_is_not_published() {
    let fixture = Fixture::new();
    fs::create_dir(fixture.output()).unwrap();
    fs::write(fixture.output().join("sentinel"), b"keep").unwrap();
    let prepared = prepare_steam_game_input(&fixture.options, fixture.extraction.clone()).unwrap();
    assert!(prepared.extract_to(&fixture.output()).is_err());
    assert_eq!(
        fs::read(fixture.output().join("sentinel")).unwrap(),
        b"keep"
    );
    fs::remove_dir_all(fixture.output()).unwrap();
    let mut options = fixture.options.clone();
    options.limits.max_output_bytes = 1;
    let prepared = prepare_steam_game_input(&options, fixture.extraction.clone()).unwrap();
    assert!(prepared.extract_to(&fixture.output()).is_err());
    assert!(!fixture.output().exists());
}
