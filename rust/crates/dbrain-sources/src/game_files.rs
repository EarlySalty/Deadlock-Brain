use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "game_files/anchored.rs"]
mod anchored;
#[path = "game_files/budget.rs"]
mod budget;
#[path = "game_files/json_text.rs"]
mod json_text;
#[path = "game_files/kv.rs"]
mod kv;
#[path = "game_files/kv3.rs"]
mod kv3;
#[path = "game_files/vpk.rs"]
mod vpk;

pub const EXTRACTOR_VERSION: &str = "game-files-v2";
const MAX_VPK_TREE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFileOptions {
    pub root: PathBuf,
    pub app_id: u32,
    pub source_id: String,
    pub observed_at: String,
    pub build_id: Option<String>,
    pub manifest_id: Option<String>,
    pub source_revision: Option<String>,
    pub depot_id: Option<u32>,
    pub language: String,
    pub attribution: String,
    pub license_name: String,
    pub license_url: Option<String>,
    pub provenance: Value,
    pub max_file_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFileInventoryEntry {
    pub relative_path: String,
    pub container_path: Option<String>,
    pub bytes: u64,
    pub original_sha256: Option<String>,
    pub category: String,
    pub disposition: String,
    pub reason: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameFileInventory {
    pub extractor_version: String,
    pub files: Vec<GameFileInventoryEntry>,
    pub documents: u64,
    pub facts: u64,
    pub unknown_revisions: u64,
    pub categories: BTreeMap<String, u64>,
    pub gaps: Vec<String>,
}

/// Writes source documents only. The caller owns the output artifact and database import.
pub fn extract_game_files(
    options: &GameFileOptions,
    output: &mut impl Write,
) -> io::Result<GameFileInventory> {
    validate_options(options)?;
    let root = anchored::root(&options.root)?;
    let mut inventory = GameFileInventory {
        extractor_version: EXTRACTOR_VERSION.to_owned(),
        ..Default::default()
    };
    visit_directory(&root, Path::new(""), options, output, &mut inventory)?;
    output.flush()?;
    Ok(inventory)
}

fn validate_options(options: &GameFileOptions) -> io::Result<()> {
    if options.app_id == 0
        || options.source_id.trim().is_empty()
        || options.attribution.trim().is_empty()
        || options.license_name.trim().is_empty()
        || options.language.trim().is_empty()
        || options.max_file_bytes == 0
        || options.max_file_bytes >= usize::MAX as u64
        || !options.provenance.is_object()
    {
        return Err(invalid(
            "Unvollständige Herkunft oder ungültige Extraktionsgrenze",
        ));
    }
    // C performs the complete RFC 3339 validation before import.
    if !options.observed_at.ends_with('Z') || !options.observed_at.contains('T') {
        return Err(invalid("observed_at muss eine UTC-Zeit nach RFC 3339 sein"));
    }
    for revision in [
        &options.build_id,
        &options.manifest_id,
        &options.source_revision,
    ]
    .into_iter()
    .flatten()
    {
        if revision.trim().is_empty() {
            return Err(invalid("Eine angegebene Revision darf nicht leer sein"));
        }
    }
    if let Some(layout) = options.provenance.get("source_layout") {
        if !matches!(
            layout.as_str(),
            Some("resource_paths" | "gametracking-citadel-pak01-dir" | "gametracking-pak01-dir")
        ) {
            return Err(invalid("Unbekanntes Spieldatei-Quellenlayout"));
        }
    }
    Ok(())
}

fn visit_directory(
    directory: &File,
    relative_directory: &Path,
    options: &GameFileOptions,
    output: &mut impl Write,
    inventory: &mut GameFileInventory,
) -> io::Result<()> {
    let anchored_path = anchored::location(directory)?;
    let mut paths = fs::read_dir(&anchored_path)?
        .map(|entry| entry.map(|entry| entry.file_name()))
        .collect::<io::Result<Vec<_>>>()?;
    paths.sort();
    for name in paths {
        let path = anchored_path.join(&name);
        let relative = normalize_relative(&relative_directory.join(&name))?;
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            inventory.files.push(entry(
                &relative,
                None,
                0,
                "excluded",
                "skipped",
                Some("Symlink wird nicht verfolgt"),
            ));
            continue;
        }
        if metadata.is_dir() {
            if path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with('.'))
            {
                continue;
            }
            let child = anchored::directory(&path)?;
            visit_directory(&child, Path::new(&relative), options, output, inventory)?;
            continue;
        }
        if !metadata.is_file() {
            continue;
        }
        let category = classify(&relative);
        if is_sensitive_name(&relative) {
            inventory.files.push(entry(
                &relative,
                None,
                metadata.len(),
                "excluded",
                "skipped",
                Some("Zugangsdaten und lokale Konfiguration ausgeschlossen"),
            ));
            continue;
        }
        if relative.to_ascii_lowercase().ends_with("_dir.vpk") {
            match vpk::read_directory(&path, MAX_VPK_TREE_BYTES) {
                Ok(archive) => {
                    // Preflight every retained virtual path before emitting any resource.
                    // The archive prefix can amplify a tiny entry just like a KV parent key.
                    let mut path_budget = budget::Budget::new();
                    let path_check = archive.entries.iter().try_for_each(|resource| {
                        path_budget
                            .expanded(&[relative.len(), resource.path.as_os_str().len()], 1536)
                    });
                    if let Err(reason) = path_check {
                        inventory.files.push(entry(
                            &relative,
                            None,
                            metadata.len(),
                            "package_directory",
                            "gap",
                            Some(reason),
                        ));
                        inventory.gaps.push(format!("{relative}: {reason}"));
                        continue;
                    }
                    let original_sha256 = hash_file(archive.header.original_file()?)?;
                    let mut item = entry(
                        &relative,
                        None,
                        metadata.len(),
                        "package_directory",
                        "inventoried",
                        None,
                    );
                    item.original_sha256 = Some(original_sha256.clone());
                    inventory.files.push(item);
                    for resource in archive.entries {
                        let parent = Path::new(&relative)
                            .parent()
                            .unwrap_or_else(|| Path::new(""));
                        let virtual_path = normalize_relative(&parent.join(&resource.path))?;
                        let resource_category = classify(&virtual_path);
                        let mut item = entry(
                            &virtual_path,
                            Some(&relative),
                            resource.total_bytes(),
                            resource_category,
                            "inventoried",
                            None,
                        );
                        if is_sensitive_name(&virtual_path) {
                            item.category = "excluded".to_owned();
                            item.disposition = "skipped".to_owned();
                            item.reason = Some(
                                "Zugangsdaten und lokale Konfiguration ausgeschlossen".to_owned(),
                            );
                            inventory.files.push(item);
                            continue;
                        }
                        if resource_category == "binary_asset" {
                            item.reason = Some(
                                "Binäres Asset; kein Beleg ausgeführter Spiellogik".to_owned(),
                            );
                            inventory.files.push(item);
                            continue;
                        }
                        if resource.total_bytes() > options.max_file_bytes {
                            item.disposition = "gap".to_owned();
                            item.reason = Some(
                                "Datei überschreitet die konfigurierte Größenbegrenzung".to_owned(),
                            );
                            inventory
                                .gaps
                                .push(format!("{relative}:{virtual_path}: Größenbegrenzung"));
                            inventory.files.push(item);
                            continue;
                        }
                        match vpk::read_resource(&path, &archive.header, &resource) {
                            Ok(bytes) => {
                                let extraction = json!({
                                    "method": "vpk-directory-and-payload",
                                    "version": EXTRACTOR_VERSION,
                                    "container_path": relative,
                                    "container_sha256": original_sha256,
                                    "archive_index": resource.archive_index,
                                    "archive_offset": resource.offset,
                                    "archive_length": resource.length,
                                    "preload_bytes": resource.preload.len(),
                                    "crc32_verified": true
                                });
                                write_document(
                                    options,
                                    &virtual_path,
                                    &bytes,
                                    extraction,
                                    &mut item,
                                    output,
                                    inventory,
                                )?;
                            }
                            Err(error) => {
                                item.disposition = "gap".to_owned();
                                item.reason = Some(error.to_string());
                                inventory
                                    .gaps
                                    .push(format!("{relative}:{virtual_path}: {error}"));
                            }
                        }
                        inventory.files.push(item);
                    }
                }
                Err(error) => {
                    inventory.files.push(entry(
                        &relative,
                        None,
                        metadata.len(),
                        "package_directory",
                        "gap",
                        Some(&error.to_string()),
                    ));
                    inventory.gaps.push(format!("{relative}: {error}"));
                }
            }
            continue;
        }
        let mut item = entry(
            &relative,
            None,
            metadata.len(),
            category,
            "inventoried",
            None,
        );
        if category == "binary_asset" {
            item.reason = Some("Binäres Asset; kein Beleg ausgeführter Spiellogik".to_owned());
            inventory.files.push(item);
            continue;
        }
        if metadata.len() > options.max_file_bytes {
            item.disposition = "gap".to_owned();
            item.reason = Some("Datei überschreitet die konfigurierte Größenbegrenzung".to_owned());
            inventory.gaps.push(format!("{relative}: Größenbegrenzung"));
            inventory.files.push(item);
            continue;
        }
        let mut bytes = Vec::new();
        open_regular(&path)?
            .take(options.max_file_bytes + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > options.max_file_bytes {
            item.disposition = "gap".to_owned();
            item.reason =
                Some("Datei wuchs während der Extraktion über die Größenbegrenzung".to_owned());
            inventory
                .gaps
                .push(format!("{relative}: Datei während Extraktion geändert"));
        } else {
            write_document(
                options,
                &relative,
                &bytes,
                json!({"method": "loose-file", "version": EXTRACTOR_VERSION}),
                &mut item,
                output,
                inventory,
            )?;
        }
        inventory.files.push(item);
    }
    Ok(())
}

fn entry(
    path: &str,
    container: Option<&str>,
    bytes: u64,
    category: &str,
    disposition: &str,
    reason: Option<&str>,
) -> GameFileInventoryEntry {
    GameFileInventoryEntry {
        relative_path: path.to_owned(),
        container_path: container.map(str::to_owned),
        bytes,
        original_sha256: None,
        category: category.to_owned(),
        disposition: disposition.to_owned(),
        reason: reason.map(str::to_owned),
    }
}

fn write_document(
    options: &GameFileOptions,
    path: &str,
    bytes: &[u8],
    extraction: Value,
    item: &mut GameFileInventoryEntry,
    output: &mut impl Write,
    inventory: &mut GameFileInventory,
) -> io::Result<()> {
    let original_hash = sha256(bytes);
    item.original_sha256 = Some(original_hash.clone());
    let content = match decode_text(bytes) {
        Ok(content) => content,
        Err(reason) => {
            item.disposition = "gap".to_owned();
            item.reason = Some(reason.to_owned());
            inventory.gaps.push(format!("{path}: {reason}"));
            return Ok(());
        }
    };
    let content_hash = sha256(content.as_bytes());
    let revision = options
        .manifest_id
        .as_ref()
        .map(|id| format!("manifest:{id}"))
        .or_else(|| options.build_id.as_ref().map(|id| format!("build:{id}")))
        .or_else(|| options.source_revision.clone())
        .unwrap_or_else(|| format!("unknown:{content_hash}"));
    let canonical_path =
        canonical_resource_path(options, path, extraction["method"] == "loose-file");
    let (facts, parse_status) = extract_facts(&canonical_path, &content);
    let facts_count = facts.len() as u64;
    let mut document = json!({
        "contract_version": "wiki-spielwissen-v1",
        "source_kind": "game_file",
        "source_id": options.source_id,
        "document_id": format!("game:{}:{canonical_path}", options.app_id),
        "source_locator": path,
        "title": path,
        "language": options.language,
        "revision": revision,
        "observed_at": options.observed_at,
        "content_sha256": content_hash,
        "content": content,
        "evidence_status": "extracted_value",
        "license": {
            "name": options.license_name,
            "url": options.license_url,
            "attribution": options.attribution,
            "redistribution_allowed": false
        },
        "metadata": {
            "app_id": options.app_id,
            "build_id": options.build_id,
            "manifest_id": options.manifest_id,
            "source_revision": options.source_revision,
            "depot_id": options.depot_id,
            "version_status": if options.build_id.is_some() || options.manifest_id.is_some() { "declared" } else if options.source_revision.is_some() { "source_revision_only" } else { "unknown" },
            "relative_path": canonical_path,
            "original_relative_path": path,
            "source_layout": options.provenance.get("source_layout").and_then(Value::as_str).unwrap_or("resource_paths"),
            "original_sha256": original_hash,
            "original_bytes": bytes.len(),
            "extraction": extraction,
            "category": item.category,
            "category_basis": "path_and_format_only",
            "parse_status": parse_status,
            "provenance": options.provenance,
            "gameplay_execution_verified": false
        }
    });
    // Move the already bounded values; json!(facts) would duplicate the entire fact tree.
    document["facts"] = Value::Array(facts);
    serde_json::to_writer(&mut *output, &document).map_err(invalid)?;
    output.write_all(b"\n")?;
    item.disposition = "extracted".to_owned();
    inventory.documents += 1;
    inventory.facts += facts_count;
    if options.build_id.is_none()
        && options.manifest_id.is_none()
        && options.source_revision.is_none()
    {
        inventory.unknown_revisions += 1;
    }
    *inventory
        .categories
        .entry(item.category.clone())
        .or_default() += 1;
    Ok(())
}

fn canonical_resource_path<'a>(
    options: &GameFileOptions,
    path: &'a str,
    loose: bool,
) -> std::borrow::Cow<'a, str> {
    let layout = options.provenance["source_layout"].as_str();
    if loose
        && matches!(
            layout,
            Some("gametracking-citadel-pak01-dir" | "gametracking-pak01-dir")
        )
    {
        for (presentation, resource_root) in [
            ("game/citadel/pak01_dir/", "game/citadel/"),
            ("game/core/pak01_dir/", "game/core/"),
        ] {
            if let Some(resource) = path.strip_prefix(presentation) {
                return format!("{resource_root}{resource}").into();
            }
        }
    }
    path.into()
}

fn extract_facts(path: &str, content: &str) -> (Vec<Value>, String) {
    let trimmed = content.trim_start_matches('\u{feff}').trim_start();
    if path.to_ascii_lowercase().ends_with(".json") {
        return match json_text::parse(trimmed) {
            Ok(leaves) => match leaf_facts(path, leaves, "json", "file.json_value") {
                Ok(facts) => (facts, "json".to_owned()),
                Err(reason) => (Vec::new(), reason.to_owned()),
            },
            Err(reason) => (Vec::new(), reason.to_owned()),
        };
    }
    if trimmed.starts_with("<!-- kv3") || path.to_ascii_lowercase().ends_with(".kv3") {
        return match kv3::parse(trimmed) {
            Ok(leaves) => match leaf_facts(path, leaves, "kv3", "file.kv3_value") {
                Ok(facts) => (facts, "kv3_text_values".to_owned()),
                Err(reason) => (Vec::new(), reason.to_owned()),
            },
            Err(reason) => (Vec::new(), format!("kv3_text_preserved: {reason}")),
        };
    }
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(
        extension.as_str(),
        "txt" | "kv" | "res" | "vdf" | "vdata" | "vdata_inc" | "gi" | "kv1"
    ) {
        return match kv::parse(trimmed) {
            Ok(entries) => {
                let mut budget = budget::Budget::new();
                let facts: Result<Vec<Value>, &'static str> = entries.into_iter().map(|entry| {
                    budget.expanded(&[path.len(), entry.pointer.len(), entry.value.len(), entry.key.len(), entry.conditions.iter().map(String::len).sum()], 1536 + entry.conditions.len() * 128)?;
                    Ok(json!({
                        "fact_id": format!("kv:{}", entry.pointer),
                        "subject": format!("game_file:{path}"),
                        "predicate": "file.kv_value",
                        "value": entry.value,
                        "unit": null,
                        "evidence_status": "extracted_value",
                        "source_span": format!("{path}:{}", entry.pointer),
                        "qualifiers": {"source_key": entry.key, "occurrence": entry.occurrence, "conditions": entry.conditions, "string_encoding": "source_escape_bytes_preserved", "unit_status": "unknown", "gameplay_binding": "uninterpreted"}
                    }))
                }).collect();
                match facts {
                    Ok(facts) => (facts, "kv1_lossless_entries".to_owned()),
                    Err(reason) => (Vec::new(), reason.to_owned()),
                }
            }
            Err(reason) => (
                Vec::new(),
                format!("text_preserved_without_semantic_parser: {reason}"),
            ),
        };
    }
    (
        Vec::new(),
        "text_preserved_without_semantic_parser".to_owned(),
    )
}

fn leaf_facts(
    path: &str,
    leaves: Vec<kv3::Leaf>,
    kind: &str,
    predicate: &str,
) -> Result<Vec<Value>, &'static str> {
    let mut budget = budget::Budget::new();
    leaves.into_iter().map(|leaf| {
        budget.expanded(&[path.len(), leaf.pointer.len(), leaf.lexeme.len(), leaf.flags.iter().map(String::len).sum()], 1536 + leaf.flags.len() * 128)?;
        Ok(json!({
            "fact_id": format!("{kind}:{}", leaf.pointer),
            "subject": format!("game_file:{path}"),
            "predicate": predicate,
            "value": leaf.value,
            "unit": null,
            "evidence_status": "extracted_value",
            "source_span": format!("{path}:{}", leaf.pointer),
            "qualifiers": {"source_pointer": leaf.pointer, "json_pointer": if kind == "json" { Some(&leaf.pointer) } else { None }, "type_flags": leaf.flags, "source_lexeme": leaf.lexeme, "numeric_representation": leaf.numeric_representation, "unit_status": "unknown", "gameplay_binding": "uninterpreted", "references_resolved": false}
        }))
    }).collect()
}

fn classify(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    let extension = Path::new(&lower)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if matches!(
        extension,
        "vdata_c"
            | "vmap_c"
            | "vmdl_c"
            | "vmat_c"
            | "vtex_c"
            | "vsnd_c"
            | "vpcf_c"
            | "vpk"
            | "dll"
            | "so"
            | "exe"
    ) {
        return "binary_asset";
    }
    if !matches!(
        extension,
        "txt"
            | "json"
            | "kv"
            | "kv1"
            | "kv3"
            | "vdf"
            | "vdata"
            | "vdata_inc"
            | "vpulse"
            | "gi"
            | "inf"
            | "res"
            | "cfg"
            | "ini"
            | "vmap"
            | "xml"
            | "csv"
            | "vjs"
            | "lua"
            | "nut"
    ) {
        return "binary_asset";
    }
    if lower.contains("localization")
        || lower.contains("localisation")
        || lower.contains("resource/") && matches!(extension, "txt" | "res")
    {
        "localization"
    } else if lower.contains("abilit") {
        "abilities"
    } else if lower.contains("item") || lower.contains("upgrade") {
        "items"
    } else if lower.contains("hero") {
        "heroes"
    } else if lower.contains("npc") || lower.contains("unit") {
        "units"
    } else if lower.contains("map") {
        "map_metadata"
    } else if lower.contains("movement") {
        "movement"
    } else if lower.contains("econom") || lower.contains("shop") {
        "economy"
    } else if lower.contains("damage") {
        "damage"
    } else if lower.contains("script") || extension == "vdata" {
        "gameplay_data"
    } else {
        "supporting_text"
    }
}

fn is_sensitive_name(path: &str) -> bool {
    path.split('/').any(|part| {
        let part = part.to_ascii_lowercase();
        part == ".env"
            || part.starts_with(".env.")
            || part == "loginusers.vdf"
            || part == "config.vdf"
            || part == "ssfn"
            || part.starts_with("ssfn")
            || part.contains("credential")
            || part.contains("secret")
            || part.ends_with(".pem")
    })
}

fn decode_text(bytes: &[u8]) -> Result<String, &'static str> {
    if bytes.starts_with(&[0xff, 0xfe]) || bytes.starts_with(&[0xfe, 0xff]) {
        if !(bytes.len() - 2).is_multiple_of(2) {
            return Err("Ungültige UTF-16-Bytefolge");
        }
        let little = bytes[0] == 0xff;
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|pair| {
                if little {
                    u16::from_le_bytes([pair[0], pair[1]])
                } else {
                    u16::from_be_bytes([pair[0], pair[1]])
                }
            })
            .collect();
        let text = String::from_utf16(&units).map_err(|_| "Ungültige UTF-16-Zeichenfolge")?;
        if text.contains('\0') {
            return Err("Binärdaten mit NUL-Zeichen");
        }
        return Ok(text);
    }
    let text =
        std::str::from_utf8(bytes).map_err(|_| "Keine unterstützte UTF-8-/UTF-16-Textdatei")?;
    if text.contains('\0') {
        return Err("Binärdaten mit NUL-Zeichen");
    }
    Ok(text.to_owned())
}

fn normalize_relative(path: &Path) -> io::Result<String> {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_str().ok_or_else(|| invalid("Nicht-UTF-8-Pfad"))?;
                if part.contains('\\') || part.contains(':') || part.is_empty() {
                    return Err(invalid("Ungültiger relativer Spieldateipfad"));
                }
                components.push(part);
            }
            Component::CurDir => {}
            _ => return Err(invalid("Spieldateipfad verlässt die Quelle")),
        }
    }
    if components.is_empty() {
        return Err(invalid("Leerer Spieldateipfad"));
    }
    Ok(components.join("/"))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hash_file(mut file: File) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn open_regular(path: &Path) -> io::Result<File> {
    anchored::regular(path)
}

fn invalid(message: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(root: PathBuf) -> GameFileOptions {
        GameFileOptions {
            root,
            app_id: 1422450,
            source_id: "fixture".into(),
            observed_at: "2026-10-03T00:00:00Z".into(),
            build_id: Some("fixture".into()),
            manifest_id: None,
            source_revision: None,
            depot_id: None,
            language: "und".into(),
            attribution: "Testdaten".into(),
            license_name: "fixture".into(),
            license_url: None,
            provenance: json!({"fixture": true}),
            max_file_bytes: 1024 * 1024,
        }
    }

    #[test]
    fn amplified_paths_abort_without_partial_facts() {
        let key = "a".repeat(100_000);
        for (path, input) in [
            ("a.kv", format!("\"{key}\" {{ {} }}", "x 1 ".repeat(1000))),
            (
                "a.kv3",
                format!("{{ \"{key}\" = {{ {} }} }}", "x = 1 ".repeat(1000)),
            ),
            (
                "a.json",
                format!("{{\"{key}\":[{}]}}", vec!["1"; 1000].join(",")),
            ),
        ] {
            let (facts, status) = extract_facts(path, &input);
            assert!(facts.is_empty());
            assert!(status.contains(budget::EXCEEDED), "{status}");
        }
    }

    #[test]
    fn declared_layout_only_maps_exact_gametracking_prefix() {
        let mut o = options(PathBuf::new());
        let p = "game/citadel/pak01_dir/scripts/abilities.vdata";
        assert_eq!(canonical_resource_path(&o, p, true), p);
        o.provenance["source_layout"] = json!("gametracking-citadel-pak01-dir");
        assert_eq!(
            canonical_resource_path(&o, p, true),
            "game/citadel/scripts/abilities.vdata"
        );
        assert_eq!(canonical_resource_path(&o, p, false), p);
        let core = "game/core/pak01_dir/scripts/movement.vdata";
        assert_eq!(
            canonical_resource_path(&o, core, true),
            "game/core/scripts/movement.vdata"
        );
        o.provenance["source_layout"] = json!("gametracking-pak01-dir");
        assert_eq!(
            canonical_resource_path(&o, core, true),
            "game/core/scripts/movement.vdata"
        );
        let unrelated = "other/pak01_dir/x.txt";
        assert_eq!(canonical_resource_path(&o, unrelated, true), unrelated);
    }

    #[test]
    fn source_large_decimal_is_a_qualified_lexeme_in_kv3_and_json() {
        let number = "340282346638528859811704183484516925440.0";
        for (path, input) in [
            ("x.kv3", format!("{{ value = {number} }}")),
            ("x.json", format!("{{\"value\":{number}}}")),
        ] {
            let (facts, _) = extract_facts(path, &input);
            assert_eq!(facts.len(), 1);
            assert_eq!(facts[0]["value"], number);
            assert_eq!(facts[0]["qualifiers"]["source_lexeme"], number);
            assert_eq!(
                facts[0]["qualifiers"]["numeric_representation"],
                "source_numeric_lexeme"
            );
        }
    }

    #[test]
    fn duplicate_json_keeps_whole_raw_text_with_explicit_status() {
        let temp = tempfile::tempdir().unwrap();
        let text = r#"{"damage":12,"damage":13}"#;
        fs::write(temp.path().join("items.json"), text).unwrap();
        let mut out = Vec::new();
        extract_game_files(&options(temp.path().to_owned()), &mut out).unwrap();
        let document: Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(document["content"], text);
        assert_eq!(document["facts"], json!([]));
        assert_eq!(
            document["metadata"]["parse_status"],
            "duplicate_json_keys_preserved_as_text"
        );
    }

    #[test]
    fn raw_json_values_keep_units_unknown_and_hash_exact_content() {
        let temp = tempfile::tempdir().unwrap();
        let content = "{\"hero\":{\"cooldown\":12,\"enabled\":true}}\n";
        fs::write(temp.path().join("heroes.json"), content).unwrap();
        let mut out = Vec::new();
        let report = extract_game_files(&options(temp.path().to_owned()), &mut out).unwrap();
        let document: Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(report.documents, 1);
        assert_eq!(report.facts, 2);
        assert_eq!(document["content_sha256"], sha256(content.as_bytes()));
        assert_eq!(document["document_id"], "game:1422450:heroes.json");
        assert!(document["facts"][0]["unit"].is_null());
        assert_eq!(document["metadata"]["gameplay_execution_verified"], false);
    }

    #[test]
    fn kv_duplicates_and_conditionals_are_retained() {
        let (facts, status) = extract_facts(
            "scripts/items.txt",
            "\"root\" { \"damage\" \"12\" [$WIN32] \"damage\" \"13\" }",
        );
        assert_eq!(status, "kv1_lossless_entries");
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0]["value"], "12");
        assert_eq!(facts[1]["value"], "13");
        assert_ne!(facts[0]["fact_id"], facts[1]["fact_id"]);
        assert_eq!(facts[0]["qualifiers"]["conditions"][0], "$WIN32");
    }

    #[test]
    fn compiled_binary_and_kv3_semantics_are_not_invented() {
        assert_eq!(classify("scripts/heroes/hero.vdata_c"), "binary_asset");
        let (facts, status) = extract_facts(
            "scripts/hero.vdata",
            "<!-- kv3 encoding:text:version{} format:generic:version{} -->\n{ x = 12 }",
        );
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0]["value"], 12);
        assert!(facts[0]["unit"].is_null());
        assert_eq!(facts[0]["qualifiers"]["gameplay_binding"], "uninterpreted");
        assert_eq!(status, "kv3_text_values");
        assert!(decode_text(b"\x00binary").is_err());
    }

    #[test]
    fn unknown_revision_uses_content_hash_and_utf16_keeps_original_hash() {
        let temp = tempfile::tempdir().unwrap();
        let text = "\"lang\" { \"Text\" \"Grüße\" }";
        let mut bytes = vec![0xff, 0xfe];
        for unit in text.encode_utf16() {
            bytes.extend(unit.to_le_bytes());
        }
        fs::write(temp.path().join("localization.txt"), &bytes).unwrap();
        let mut settings = options(temp.path().to_owned());
        settings.build_id = None;
        let mut out = Vec::new();
        let report = extract_game_files(&settings, &mut out).unwrap();
        let document: Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(report.unknown_revisions, 1);
        assert_eq!(
            document["revision"],
            format!("unknown:{}", sha256(text.as_bytes()))
        );
        assert_eq!(document["metadata"]["original_sha256"], sha256(&bytes));
        assert_eq!(document["content"], text);
    }

    #[test]
    fn traversal_and_symlinks_are_not_followed() {
        assert!(normalize_relative(Path::new("../secret.txt")).is_err());
        assert!(normalize_relative(Path::new("/absolute.txt")).is_err());
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join(".env"), "PRIVATE=1").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("/etc/passwd", temp.path().join("resource.txt")).unwrap();
        let mut out = Vec::new();
        let report = extract_game_files(&options(temp.path().to_owned()), &mut out).unwrap();
        assert_eq!(report.documents, 0);
        assert!(out.is_empty());
        assert!(report
            .files
            .iter()
            .all(|file| file.disposition == "skipped"));
    }
}
