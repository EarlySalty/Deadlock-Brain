use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::game_files::{
    extract_game_files_from_handles, BoundGameFile, BoundGameFiles, GameFileInventory,
    GameFileOptions,
};

#[path = "steam_game_input/filesystem.rs"]
mod filesystem;
#[path = "steam_game_input/inventory.rs"]
mod inventory;

pub use inventory::{DepotInventory, DownloadInventory, InventoryFile, InventoryFileKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SteamGameInputLimits {
    pub max_inventory_bytes: u64,
    pub max_total_bytes: u64,
    pub max_file_bytes: u64,
    pub max_entries: usize,
    pub max_path_bytes: usize,
    pub max_depth: usize,
    pub max_open_files: usize,
    pub fd_reserve: usize,
    pub reserve_bytes: u64,
    pub max_output_bytes: u64,
}

impl SteamGameInputLimits {
    fn validate(&self) -> io::Result<()> {
        if self.max_inventory_bytes == 0
            || self.max_inventory_bytes > 256 * 1024 * 1024
            || self.max_total_bytes == 0
            || self.max_total_bytes > 96 * 1024 * 1024 * 1024
            || self.max_file_bytes == 0
            || self.max_file_bytes > self.max_total_bytes
            || self.max_entries == 0
            || self.max_entries > 300_000
            || self.max_path_bytes == 0
            || self.max_path_bytes > 4096
            || self.max_depth == 0
            || self.max_depth > 64
            || self.max_open_files == 0
            || self.fd_reserve < self.max_depth.saturating_mul(2).saturating_add(16)
            || self.max_output_bytes == 0
        {
            return Err(invalid("Ungültige Grenzen für den Steam-Dateieingang"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SteamGameInputOptions {
    pub inventory_path: PathBuf,
    #[serde(with = "inventory::hash_hex")]
    pub inventory_sha256: [u8; 32],
    pub depot_inventory_path: PathBuf,
    #[serde(with = "inventory::hash_hex")]
    pub depot_inventory_sha256: [u8; 32],
    pub root: PathBuf,
    pub app_id: u32,
    pub build_id: u64,
    pub depot_id: u32,
    pub manifest_id: String,
    pub snapshot_directory: PathBuf,
    pub limits: SteamGameInputLimits,
}

impl SteamGameInputOptions {
    pub fn from_file(path: &Path, max_bytes: u64) -> io::Result<Self> {
        if max_bytes == 0 || max_bytes > 256 * 1024 * 1024 {
            return Err(invalid(
                "Ungültige Größenbegrenzung für die normale Optionsdatei",
            ));
        }
        let options: Self = inventory::read_json(path, None, max_bytes)?;
        options.limits.validate()?;
        Ok(options)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGameInputEvidence {
    pub inventory_sha256: [u8; 32],
    pub depot_inventory_sha256: [u8; 32],
    pub manifest_transport_sha256: [u8; 32],
    pub download_inventory: DownloadInventory,
    pub depot_inventory: DepotInventory,
    pub copied_bytes: u64,
    pub copied_files: usize,
}

pub struct PreparedSteamGameInput {
    input: BoundGameFiles,
    evidence: SteamGameInputEvidence,
    limits: SteamGameInputLimits,
}

pub struct SteamGameExtraction {
    pub directory: PathBuf,
    pub jsonl_path: PathBuf,
    pub evidence_path: PathBuf,
    pub extractor_inventory: GameFileInventory,
}

impl PreparedSteamGameInput {
    pub fn evidence(&self) -> &SteamGameInputEvidence {
        &self.evidence
    }

    pub fn extract_to(self, directory: &Path) -> io::Result<SteamGameExtraction> {
        self.extract_with(directory, extract_game_files_from_handles)
    }

    fn extract_with(
        self,
        directory: &Path,
        extractor: impl FnOnce(
            BoundGameFiles,
            &mut LimitedOutput<File>,
        ) -> io::Result<GameFileInventory>,
    ) -> io::Result<SteamGameExtraction> {
        let staging = filesystem::PrivateDirectory::for_destination(directory)?;
        filesystem::require_space(
            staging.handle(),
            self.limits.max_output_bytes,
            self.limits.reserve_bytes,
        )?;
        let mut output = LimitedOutput {
            writer: staging.create_file("documents.jsonl")?,
            remaining: self.limits.max_output_bytes,
        };
        let extractor_inventory = extractor(self.input, &mut output)?;
        output.flush()?;
        filesystem::finish_private_file(&output.writer)?;
        let remaining = output.remaining;
        drop(output);
        let mut evidence = LimitedOutput {
            writer: staging.create_file("evidence.json")?,
            remaining,
        };
        serde_json::to_writer(
            &mut evidence,
            &ExtractionEvidence {
                input: &self.evidence,
                extraction: &extractor_inventory,
            },
        )?;
        evidence.write_all(b"\n")?;
        evidence.flush()?;
        filesystem::finish_private_file(&evidence.writer)?;
        drop(evidence);
        staging.publish(directory)?;
        Ok(SteamGameExtraction {
            directory: directory.to_owned(),
            jsonl_path: directory.join("documents.jsonl"),
            evidence_path: directory.join("evidence.json"),
            extractor_inventory,
        })
    }
}

#[derive(Serialize)]
struct ExtractionEvidence<'a> {
    input: &'a SteamGameInputEvidence,
    extraction: &'a GameFileInventory,
}

struct LimitedOutput<W> {
    writer: W,
    remaining: u64,
}

impl<W: Write> Write for LimitedOutput<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.remaining {
            return Err(invalid(
                "Extraktionsausgabe überschreitet die konfigurierte Grenze",
            ));
        }
        let count = self.writer.write(bytes)?;
        self.remaining -= count as u64;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub fn prepare_steam_game_input(
    options: &SteamGameInputOptions,
    extraction: GameFileOptions,
) -> io::Result<PreparedSteamGameInput> {
    options.limits.validate()?;
    let validated = inventory::load(options, &extraction)?;
    filesystem::require_descriptors(validated.regular_files, &options.limits)?;
    let root = filesystem::open_root(&options.root)?;
    let before = filesystem::observe_tree(&root, &validated.paths, &options.limits)?;
    for entry in &validated.depot.files {
        if entry.kind == InventoryFileKind::File
            && before
                .get(&entry.path)
                .is_none_or(|actual| actual.bytes != entry.size)
        {
            return Err(invalid(
                "Tatsächliche Dateigröße widerspricht dem vollständigen Inventar",
            ));
        }
    }
    let snapshots = filesystem::PrivateDirectory::new(&options.snapshot_directory)?;
    filesystem::require_space(
        snapshots.handle(),
        validated.depot.expected_bytes,
        options.limits.reserve_bytes,
    )?;
    let mut files = Vec::with_capacity(validated.regular_files);
    let mut copied_bytes = 0_u64;
    for entry in &validated.depot.files {
        if entry.kind != InventoryFileKind::File {
            continue;
        }
        filesystem::require_space(
            snapshots.handle(),
            validated
                .depot
                .expected_bytes
                .checked_sub(copied_bytes)
                .ok_or_else(|| invalid("Kopierte Bytes überschreiten die Depotsumme"))?,
            options.limits.reserve_bytes,
        )?;
        let expected = before
            .get(&entry.path)
            .ok_or_else(|| invalid("Dateibeleg fehlt"))?;
        let mut original = filesystem::open_bound_file(&root, &entry.path, expected)?;
        let mut snapshot = snapshots.create_file("snapshot")?;
        let expected_sha1 = inventory::decode_hash::<20>(entry.sha1.as_deref())?;
        let expected_sha256 = inventory::decode_hash::<32>(entry.sha256.as_deref())?;
        copy_verified(
            &mut original,
            &mut snapshot,
            entry.size,
            expected_sha1,
            expected_sha256,
        )?;
        filesystem::check_file(&original, expected)?;
        let readonly = snapshots.detach_readonly(snapshot, "snapshot")?;
        copied_bytes = copied_bytes
            .checked_add(entry.size)
            .ok_or_else(|| invalid("Kopierte Gesamtgröße übergelaufen"))?;
        files.push(BoundGameFile {
            relative_path: entry.path.clone(),
            file: readonly,
            expected_bytes: entry.size,
            steam_sha1: expected_sha1,
            expected_sha256,
        });
    }
    let after = filesystem::observe_tree(&root, &validated.paths, &options.limits)?;
    filesystem::check_root_binding(&root, &options.root)?;
    if before != after || copied_bytes != validated.depot.expected_bytes {
        return Err(invalid(
            "Depotbestand wurde während der Lesesicherung verändert",
        ));
    }
    let manifest_transport_sha256 =
        inventory::decode_hash::<32>(Some(&validated.depot.manifest_sha256))?;
    Ok(PreparedSteamGameInput {
        evidence: SteamGameInputEvidence {
            inventory_sha256: options.inventory_sha256,
            depot_inventory_sha256: options.depot_inventory_sha256,
            manifest_transport_sha256,
            download_inventory: validated.download,
            depot_inventory: validated.depot,
            copied_bytes,
            copied_files: files.len(),
        },
        input: BoundGameFiles {
            options: extraction,
            manifest_transport_sha256,
            files,
        },
        limits: options.limits.clone(),
    })
}

fn copy_verified(
    original: &mut File,
    snapshot: &mut File,
    expected_bytes: u64,
    expected_sha1: [u8; 20],
    expected_sha256: [u8; 32],
) -> io::Result<()> {
    use std::io::Read;
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut copied = 0_u64;
    let mut buffer = [0_u8; 128 * 1024];
    loop {
        let count = original.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        copied = copied
            .checked_add(count as u64)
            .ok_or_else(|| invalid("Dateigröße übergelaufen"))?;
        if copied > expected_bytes {
            return Err(invalid("Datei überschreitet die inventarisierte Größe"));
        }
        sha1.update(&buffer[..count]);
        sha256.update(&buffer[..count]);
        snapshot.write_all(&buffer[..count])?;
    }
    let actual_sha1: [u8; 20] = sha1.finalize().into();
    let actual_sha256: [u8; 32] = sha256.finalize().into();
    if copied != expected_bytes || actual_sha1 != expected_sha1 || actual_sha256 != expected_sha256
    {
        return Err(invalid(
            "Kopierte Bytes widersprechen Größe, Steam-SHA-1 oder SHA-256",
        ));
    }
    snapshot.sync_all()?;
    Ok(())
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[cfg(test)]
#[path = "steam_game_input/output_tests.rs"]
mod output_tests;
