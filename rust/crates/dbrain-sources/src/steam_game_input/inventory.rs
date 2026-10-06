use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{filesystem, invalid, GameFileOptions, SteamGameInputOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryFileKind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryFile {
    pub path: String,
    pub kind: InventoryFileKind,
    pub size: u64,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SteamProvenance {
    pub source: String,
    pub source_layout: String,
    pub app_id: u32,
    pub build_id: u64,
    pub depot_id: u32,
    pub manifest_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DepotInventory {
    pub app_id: u32,
    pub build_id: u64,
    pub depot_id: u32,
    pub manifest_id: String,
    pub manifest_sha256: String,
    pub root: String,
    pub observed_at: String,
    pub branch: String,
    pub platform: String,
    pub languages: Vec<String>,
    pub status: String,
    pub expected_bytes: u64,
    pub files: Vec<InventoryFile>,
    pub provenance: SteamProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadLimits {
    pub download_bytes: u64,
    pub holding_bytes: u64,
    pub reserve_bytes: u64,
    pub metadata_reserve_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadInventory {
    pub app_id: u32,
    pub build_id: u64,
    pub observed_at: String,
    pub branch: String,
    pub platform: String,
    pub languages: Vec<String>,
    pub status: String,
    pub expected_bytes: u64,
    pub depots: Vec<DepotInventory>,
    pub limits: DownloadLimits,
}

pub(super) struct ValidatedInventory {
    pub download: DownloadInventory,
    pub depot: DepotInventory,
    pub paths: BTreeMap<String, InventoryFileKind>,
    pub regular_files: usize,
}

pub(super) fn decode_hash<const N: usize>(text: Option<&str>) -> io::Result<[u8; N]> {
    let text = text.ok_or_else(|| invalid("Pflicht-Hash fehlt im Dateibeleg"))?;
    if text.len() != N * 2
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid(
            "Hash muss vollständiges kleingeschriebenes Hex sein",
        ));
    }
    let mut bytes = [0_u8; N];
    hex::decode_to_slice(text, &mut bytes).map_err(|_| invalid("Ungültiger Hash"))?;
    Ok(bytes)
}

pub(super) mod hash_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(hash: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode(hash))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 32], D::Error> {
        let text = String::deserialize(deserializer)?;
        super::decode_hash(Some(&text)).map_err(serde::de::Error::custom)
    }
}

pub(super) fn load(
    options: &SteamGameInputOptions,
    extraction: &GameFileOptions,
) -> io::Result<ValidatedInventory> {
    let download: DownloadInventory = read_json(
        &options.inventory_path,
        Some(options.inventory_sha256),
        options.limits.max_inventory_bytes,
    )?;
    let depot: DepotInventory = read_json(
        &options.depot_inventory_path,
        Some(options.depot_inventory_sha256),
        options.limits.max_inventory_bytes,
    )?;
    if options.app_id == 0
        || options.build_id == 0
        || options.depot_id == 0
        || !canonical_identifier(&options.manifest_id)
        || download.app_id != options.app_id
        || download.build_id != options.build_id
        || download.status != "complete"
        || download.depots.is_empty()
        || download.expected_bytes == 0
        || download.expected_bytes > options.limits.max_total_bytes
        || download.limits.download_bytes != 96 * 1024 * 1024 * 1024
        || download.limits.holding_bytes != 192 * 1024 * 1024 * 1024
        || download.limits.reserve_bytes != 16 * 1024 * 1024 * 1024
        || download.limits.metadata_reserve_bytes == 0
    {
        return Err(invalid(
            "Gesamtinventar oder erwartete Steam-Identität ist ungültig",
        ));
    }
    validate_time(&download.observed_at)?;
    let mut ids = BTreeSet::new();
    let mut roots = BTreeSet::new();
    let mut total = 0_u64;
    let mut entries = 0_usize;
    let mut selected = None;
    for candidate in &download.depots {
        if !ids.insert(candidate.depot_id)
            || !roots.insert(candidate.root.clone())
            || candidate.app_id != download.app_id
            || candidate.build_id != download.build_id
            || candidate.observed_at != download.observed_at
            || candidate.branch != download.branch
            || candidate.platform != download.platform
            || candidate.languages != download.languages
        {
            return Err(invalid(
                "Widersprüchliche oder doppelte Depotbelege im Gesamtinventar",
            ));
        }
        entries = entries
            .checked_add(candidate.files.len())
            .ok_or_else(|| invalid("Inventaranzahl übergelaufen"))?;
        if entries > options.limits.max_entries {
            return Err(invalid("Gesamtinventar überschreitet die Eintragsgrenze"));
        }
        let (paths, count) = validate_depot(candidate, options)?;
        total = total
            .checked_add(candidate.expected_bytes)
            .ok_or_else(|| invalid("Gesamtgröße übergelaufen"))?;
        if candidate.depot_id == options.depot_id {
            if candidate != &depot {
                return Err(invalid(
                    "Separater Depotbeleg widerspricht dem Gesamtinventar",
                ));
            }
            selected = Some((paths, count));
        }
    }
    if total != download.expected_bytes
        || depot.app_id != options.app_id
        || depot.build_id != options.build_id
        || depot.depot_id != options.depot_id
        || depot.manifest_id != options.manifest_id
        || Path::new(&depot.root) != options.root
    {
        return Err(invalid(
            "Depotidentität oder vollständige Downloadsumme widerspricht der Vorgabe",
        ));
    }
    validate_root(&options.root)?;
    if extraction.root != options.root
        || extraction.app_id != options.app_id
        || extraction.build_id.as_deref() != Some(options.build_id.to_string().as_str())
        || extraction.depot_id != Some(options.depot_id)
        || extraction.manifest_id.as_deref() != Some(options.manifest_id.as_str())
        || extraction.source_revision.is_some()
        || extraction.observed_at != depot.observed_at
        || extraction.provenance != serde_json::to_value(&depot.provenance)?
    {
        return Err(invalid(
            "Extraktoroptionen widersprechen der gebundenen Steam-Herkunft",
        ));
    }
    let (paths, regular_files) =
        selected.ok_or_else(|| invalid("Depot fehlt im Gesamtinventar"))?;
    Ok(ValidatedInventory {
        download,
        depot,
        paths,
        regular_files,
    })
}

pub(super) fn read_json<T: serde::de::DeserializeOwned>(
    path: &Path,
    expected_sha256: Option<[u8; 32]>,
    max_bytes: u64,
) -> io::Result<T> {
    use std::io::Read;
    let (mut file, identity) = filesystem::open_explicit_file(path)?;
    if identity.bytes > max_bytes {
        return Err(invalid(
            "Inventardatei überschreitet die konfigurierte Grenze",
        ));
    }
    let mut bytes = Vec::new();
    (&mut file).take(max_bytes + 1).read_to_end(&mut bytes)?;
    filesystem::check_file(&file, &identity)?;
    let actual_sha256: [u8; 32] = Sha256::digest(&bytes).into();
    if bytes.len() as u64 != identity.bytes
        || expected_sha256.is_some_and(|expected| actual_sha256 != expected)
    {
        return Err(invalid(
            "Inventardatei widerspricht Größe oder gepinntem SHA-256",
        ));
    }
    serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("Ungültiger D-Inventarbeleg: {error}")))
}

fn validate_depot(
    depot: &DepotInventory,
    options: &SteamGameInputOptions,
) -> io::Result<(BTreeMap<String, InventoryFileKind>, usize)> {
    validate_time(&depot.observed_at)?;
    validate_root(Path::new(&depot.root))?;
    decode_hash::<32>(Some(&depot.manifest_sha256))?;
    let provenance = &depot.provenance;
    if depot.status != "complete"
        || depot.app_id == 0
        || depot.build_id == 0
        || depot.depot_id == 0
        || !canonical_identifier(&depot.manifest_id)
        || provenance.source != "steam"
        || provenance.source_layout != "resource_paths"
        || provenance.app_id != depot.app_id
        || provenance.build_id != depot.build_id
        || provenance.depot_id != depot.depot_id
        || provenance.manifest_id != depot.manifest_id
    {
        return Err(invalid(
            "Unvollständiger oder widersprüchlicher Steam-Depotbeleg",
        ));
    }
    let mut paths = BTreeMap::new();
    let mut explicit = BTreeSet::new();
    let mut bytes = 0_u64;
    let mut regular = 0_usize;
    for entry in &depot.files {
        validate_relative(&entry.path, &options.limits)?;
        if !explicit.insert(entry.path.as_str()) {
            return Err(invalid("Doppelter relativer Inventarpfad"));
        }
        match entry.kind {
            InventoryFileKind::File => {
                decode_hash::<20>(entry.sha1.as_deref())?;
                decode_hash::<32>(entry.sha256.as_deref())?;
                if entry.size > options.limits.max_file_bytes {
                    return Err(invalid(
                        "Inventarisierte Datei überschreitet die Dateigrößengrenze",
                    ));
                }
                bytes = bytes
                    .checked_add(entry.size)
                    .ok_or_else(|| invalid("Depotgröße übergelaufen"))?;
                regular += 1;
            }
            InventoryFileKind::Directory => {
                if entry.size != 0 || entry.sha1.is_some() || entry.sha256.is_some() {
                    return Err(invalid(
                        "Verzeichnisbeleg enthält Dateigröße oder Dateihashes",
                    ));
                }
            }
        }
        insert_path(
            &mut paths,
            &entry.path,
            entry.kind,
            options.limits.max_entries,
        )?;
        let mut parent = entry.path.as_str();
        while let Some((next, _)) = parent.rsplit_once('/') {
            insert_path(
                &mut paths,
                next,
                InventoryFileKind::Directory,
                options.limits.max_entries,
            )?;
            parent = next;
        }
    }
    if bytes != depot.expected_bytes {
        return Err(invalid(
            "Vollständige Dateisumme widerspricht dem Depotbeleg",
        ));
    }
    Ok((paths, regular))
}

fn insert_path(
    paths: &mut BTreeMap<String, InventoryFileKind>,
    path: &str,
    kind: InventoryFileKind,
    limit: usize,
) -> io::Result<()> {
    if let Some(previous) = paths.get(path) {
        if previous != &kind {
            return Err(invalid("Datei-/Verzeichniskonflikt im Inventar"));
        }
    } else {
        if paths.len() >= limit {
            return Err(invalid(
                "Inventar samt Elternverzeichnissen überschreitet die Eintragsgrenze",
            ));
        }
        paths.insert(path.to_owned(), kind);
    }
    Ok(())
}

pub(super) fn validate_relative(
    path: &str,
    limits: &super::SteamGameInputLimits,
) -> io::Result<()> {
    if path.is_empty()
        || path.len() > limits.max_path_bytes
        || path.contains(['\\', '\0', ':'])
        || path.split('/').any(|part| {
            part.is_empty() || part == "." || part == ".." || part.starts_with(".download-")
        })
        || path.split('/').count() > limits.max_depth
        || Path::new(path)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(invalid(
            "Nicht normalisierter oder zu langer relativer Inventarpfad",
        ));
    }
    Ok(())
}

fn validate_root(path: &Path) -> io::Result<()> {
    if !path.is_absolute()
        || path == Path::new("/")
        || path
            .components()
            .any(|part| !matches!(part, Component::RootDir | Component::Normal(_)))
        || path.to_str().is_none_or(|text| {
            text.split('/')
                .skip(1)
                .any(|part| part.is_empty() || part == "." || part == "..")
        })
    {
        return Err(invalid(
            "Root muss ein normalisierter absoluter Verzeichnispfad sein",
        ));
    }
    Ok(())
}

fn canonical_identifier(value: &str) -> bool {
    value
        .parse::<u64>()
        .is_ok_and(|id| id > 0 && id.to_string() == value)
}

fn validate_time(value: &str) -> io::Result<()> {
    if !value.ends_with('Z') || chrono::DateTime::parse_from_rfc3339(value).is_err() {
        return Err(invalid("Inventarzeit muss eine belegte UTC-Zeit sein"));
    }
    Ok(())
}
