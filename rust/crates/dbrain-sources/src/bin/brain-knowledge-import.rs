#![forbid(unsafe_code)]

use brain_contracts::{CorpusRelease, SourceRecordV2};
use brain_storage::PgStore;
use dbrain_sources::knowledge_contract::validate_knowledge_jsonl;
use dbrain_sources::knowledge_import::{
    import_prepared_knowledge, prepare_validated_knowledge, ImportPolicy, PreparedKnowledgeImport,
};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "brain-knowledge-import/public_game.rs"]
mod public_game;
#[path = "brain-knowledge-import/runtime.rs"]
mod runtime;

#[derive(Debug)]
struct Arguments {
    mode: String,
    values: BTreeMap<String, String>,
    sources: Vec<String>,
}

impl Arguments {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>, String> {
        let mut args = args.into_iter();
        let Some(mode) = args.next() else {
            return Err("Unterbefehl fehlt: partition, validate, import oder publish".into());
        };
        if matches!(mode.as_str(), "--help" | "-h") {
            return Ok(None);
        }
        if !matches!(
            mode.as_str(),
            "partition"
                | "validate"
                | "import"
                | "publish"
                | "inspect-game"
                | "reauthorize-game"
                | "extract-game"
                | "export-legacy-game"
        ) {
            return Err("Unbekannter Unterbefehl".into());
        }
        let mut parsed = Self {
            mode,
            values: BTreeMap::new(),
            sources: Vec::new(),
        };
        while let Some(key) = args.next() {
            let allowed = match parsed.mode.as_str() {
                "partition" => ["--input", "--output-dir", "--report"].as_slice(),
                "validate" => [
                    "--input",
                    "--policy",
                    "--parser-revision",
                    "--report",
                    "--authorization-ref",
                ]
                .as_slice(),
                "import" => [
                    "--input",
                    "--policy",
                    "--parser-revision",
                    "--report",
                    "--infisical-config",
                    "--runtime-config",
                ]
                .as_slice(),
                "publish" => [
                    "--base-release",
                    "--release-id",
                    "--knowledge-version",
                    "--source",
                    "--report",
                    "--infisical-config",
                    "--runtime-config",
                ]
                .as_slice(),
                "inspect-game" => [
                    "--source",
                    "--report",
                    "--runtime-config",
                    "--infisical-config",
                ]
                .as_slice(),
                "reauthorize-game" => [
                    "--source",
                    "--authorization-ref",
                    "--provider-egress-ref",
                    "--report",
                    "--runtime-config",
                    "--infisical-config",
                ]
                .as_slice(),
                "export-legacy-game" => [
                    "--source",
                    "--output",
                    "--report",
                    "--runtime-config",
                    "--infisical-config",
                ]
                .as_slice(),
                "extract-game" => [
                    "--source",
                    "--revision",
                    "--output",
                    "--report",
                    "--runtime-config",
                ]
                .as_slice(),
                _ => unreachable!(),
            };
            if !allowed.contains(&key.as_str()) {
                return Err("Unbekannte Option für diesen Unterbefehl".into());
            }
            let value = args.next().ok_or("Optionswert fehlt")?;
            if value.is_empty() || value.starts_with("--") || value.chars().any(char::is_control) {
                return Err("Ungültiger Optionswert".into());
            }
            if key == "--source" {
                if parsed.sources.contains(&value) {
                    return Err("Quelle mehrfach angegeben".into());
                }
                parsed.sources.push(value);
            } else if parsed.values.insert(key, value).is_some() {
                return Err("Option mehrfach angegeben".into());
            }
        }
        parsed.path("--report")?;
        match parsed.mode.as_str() {
            "partition" => {
                parsed.path("--input")?;
                parsed.path("--output-dir")?;
            }
            "validate" | "import" => {
                parsed.path("--input")?;
                parsed.path("--policy")?;
                parsed.required("--parser-revision")?;
            }
            "publish" => {
                for key in ["--base-release", "--release-id", "--knowledge-version"] {
                    parsed.required(key)?;
                }
                if parsed.sources.is_empty() {
                    return Err("Mindestens eine ausdrücklich benannte Quelle erforderlich".into());
                }
                if parsed.required("--base-release")? == parsed.required("--release-id")? {
                    return Err("Neuer Release muss eine eigene Kennung erhalten".into());
                }
            }
            "inspect-game" | "reauthorize-game" | "extract-game" | "export-legacy-game" => {
                dbrain_sources::knowledge_import::public_game::validate_sources(&parsed.sources)?;
                if parsed.mode == "export-legacy-game" {
                    if parsed.sources.len() != 1
                        || !["legacy-entities", "legacy-patchnotes"]
                            .contains(&parsed.sources[0].as_str())
                    {
                        return Err("Legacy-Export erfordert genau eine Legacy-Spielquelle".into());
                    }
                    parsed.path("--output")?;
                }
                if parsed.mode == "reauthorize-game" {
                    parsed.required("--authorization-ref")?;
                }
                if parsed.mode == "extract-game" {
                    if parsed.sources.len() != 1
                        || !dbrain_sources::knowledge_import::public_game::SOURCES[..2]
                            .contains(&parsed.sources[0].as_str())
                    {
                        return Err(
                            "Extraktion erfordert genau eine gepinnte Git-Spielquelle".into()
                        );
                    }
                    parsed.path("--output")?;
                    parsed.path("--runtime-config")?;
                    dbrain_sources::git_source::validate_commit(parsed.required("--revision")?)
                        .map_err(|_| "Vollständiger Git-Commit erforderlich")?;
                }
            }
            _ => unreachable!(),
        }
        if matches!(
            parsed.mode.as_str(),
            "import" | "publish" | "inspect-game" | "reauthorize-game" | "export-legacy-game"
        ) {
            parsed.path("--runtime-config")?;
            if parsed.values.contains_key("--infisical-config") {
                parsed.path("--infisical-config")?;
            }
        }
        Ok(Some(parsed))
    }

    fn required(&self, key: &str) -> Result<&str, String> {
        self.values
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| format!("Pflichtoption fehlt: {key}"))
    }

    fn path(&self, key: &str) -> Result<PathBuf, String> {
        let path = PathBuf::from(self.required(key)?);
        if !path.is_absolute() {
            return Err(format!("Absoluter Pfad erforderlich: {key}"));
        }
        Ok(path)
    }
}

#[derive(Serialize)]
struct InputProof {
    input_bytes: u64,
    documents: usize,
    facts: usize,
    duplicate_lines: usize,
    input_conflicts: usize,
    unknown_revisions: usize,
    prepared_records: usize,
    content_bytes: usize,
    largest_content_bytes: usize,
}

fn prepare(args: &Arguments) -> Result<(PreparedKnowledgeImport, InputProof), String> {
    let input = File::open(args.path("--input")?)
        .map_err(|_| "Wissenseingabe kann nicht geöffnet werden")?;
    if !input
        .metadata()
        .map_err(|_| "Eingabegröße kann nicht geprüft werden")?
        .is_file()
    {
        return Err("Reguläre Wissenseingabe erforderlich".into());
    }
    let mut bytes = Vec::new();
    input
        .take(partitions::SINGLETON_MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Wissenseingabe kann nicht begrenzt gelesen werden")?;
    if bytes.len() > partitions::SINGLETON_MAX_BYTES {
        return Err(
            "Importeingabe überschreitet 128 MiB; vorhandenen Partitionspfad verwenden".into(),
        );
    }
    let input_bytes = bytes.len() as u64;
    let validated = validate_knowledge_jsonl(Cursor::new(bytes.as_slice()))
        .map_err(|error| error.to_string())?;
    drop(bytes);
    let policy: ImportPolicy = serde_json::from_reader(BufReader::new(
        File::open(args.path("--policy")?)
            .map_err(|_| "Importrechte können nicht geöffnet werden")?,
    ))
    .map_err(|_| "Importrechte entsprechen nicht dem Schema")?;
    let prepared =
        prepare_validated_knowledge(&validated, &policy, args.required("--parser-revision")?)
            .map_err(|error| error.to_string())?;
    let proof = InputProof {
        input_bytes,
        documents: validated.documents().len(),
        facts: validated
            .documents()
            .iter()
            .map(|record| record.document.facts.len())
            .sum(),
        duplicate_lines: validated.duplicates().len(),
        input_conflicts: validated.conflicts().len(),
        unknown_revisions: validated.unknown_revision_count(),
        prepared_records: prepared.records().len(),
        content_bytes: prepared
            .records()
            .iter()
            .map(|record| record.record.content.len())
            .sum(),
        largest_content_bytes: prepared
            .records()
            .iter()
            .map(|record| record.record.content.len())
            .max()
            .unwrap_or(0),
    };
    Ok((prepared, proof))
}

fn public_game_projection_proof(
    prepared: &PreparedKnowledgeImport,
    authorization_ref: &str,
) -> Result<Value, String> {
    let mut projected_bytes = 0usize;
    let mut facts = 0usize;
    let mut documents = Vec::new();
    let mut selection = dbrain_sources::knowledge_import::public_game::GameSelection::default();
    for prepared_record in prepared.records() {
        let record = &prepared_record.record;
        let admitted = dbrain_sources::knowledge_import::public_game::select_game_heads(
            std::slice::from_ref(record),
            authorization_ref,
            None,
        )?;
        if admitted.selected.is_empty() {
            selection.excluded.extend(admitted.excluded);
            continue;
        }
        selection.selected.extend(admitted.selected);
        let next_revision = record
            .revision
            .checked_add(1)
            .ok_or("Speicherrevision ist zu groß für die Offline-Probe")?;
        let authorized = dbrain_sources::knowledge_import::public_game::authorize_game_record(
            record,
            authorization_ref,
            None,
            next_revision,
        )?;
        let projection = dbrain_retrieval::knowledge_projection::project_knowledge(&authorized)
            .map_err(|_| "Öffentliche Faktenprojektion hat ihren Vertrag nicht bestanden")?
            .ok_or("Öffentliche Faktenprojektion fehlt")?;
        if projection.raw_byte_end != 0 || projection.facts.is_empty() {
            return Err("Offline-Probe enthält keine reine Faktenprojektion".into());
        }
        projected_bytes = projected_bytes
            .checked_add(projection.text.len())
            .ok_or("Projektionsgröße ist zu groß")?;
        facts = facts
            .checked_add(projection.facts.len())
            .ok_or("Faktenzahl ist zu groß")?;
        documents.push(json!({
            "source_id": record.source_id,
            "logical_id": record.logical_id,
            "raw_sha256": projection.raw_sha256,
            "semantic_sha256": projection.semantic_sha256,
            "projected_bytes": projection.text.len(),
            "facts": projection.facts.len(),
        }));
    }
    Ok(json!({
        "simulation_only": true,
        "projection_version": dbrain_retrieval::knowledge_projection::PUBLIC_GAME_FACTS_PROJECTION_VERSION,
        "documents": documents.len(),
        "facts": facts,
        "projected_bytes": projected_bytes,
        "records": documents,
        "selection": selection.report(), "checked_records": prepared.records().len(),
        "originals_changed": false,
    }))
}

fn report_preflight(path: &Path) -> Result<(), String> {
    if std::fs::symlink_metadata(path).is_ok() {
        return Err(
            "Berichtsdatei existiert bereits; vorhandene Nachweise werden nicht überschrieben"
                .into(),
        );
    }
    if !path.parent().is_some_and(Path::is_dir) {
        return Err("Berichtsordner muss bereits vorhanden sein".into());
    }
    Ok(())
}

fn write_report(path: &Path, value: &Value) -> Result<(), String> {
    let parent = path.parent().ok_or("Berichtsordner fehlt")?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Bericht kann nicht vorbereitet werden")?;
    serde_json::to_writer_pretty(file.as_file_mut(), value)
        .map_err(|_| "Bericht kann nicht serialisiert werden")?;
    file.as_file_mut()
        .write_all(b"\n")
        .map_err(|_| "Bericht kann nicht geschrieben werden")?;
    file.as_file()
        .sync_all()
        .map_err(|_| "Bericht kann nicht dauerhaft gespeichert werden")?;
    file.persist_noclobber(path)
        .map_err(|_| "Bericht kann nicht ohne Überschreiben veröffentlicht werden")?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "Berichtsordner kann nicht dauerhaft gespeichert werden")?;
    Ok(())
}

const PARTITION_MAX_DOCUMENT_LINES: usize = 1_000;
const PARTITION_MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Serialize)]
struct PartitionRange {
    input_byte_start: usize,
    input_byte_end_exclusive: usize,
    first_line: usize,
    last_line: usize,
    document_lines: usize,
}

#[derive(Serialize)]
struct PartitionSourceProof {
    document_lines: usize,
    distinct_documents: usize,
    distinct_versions: usize,
    facts: usize,
}

#[path = "brain-knowledge-import/partition.rs"]
mod partitions;
use partitions::partition;

#[cfg(test)]
fn partition_ranges(
    input: &[u8],
    max_lines: usize,
    max_bytes: usize,
) -> Result<Vec<PartitionRange>, String> {
    if max_lines == 0 || max_bytes == 0 {
        return Err("Partitionsgrenzen müssen größer als null sein".into());
    }
    let mut ranges = Vec::new();
    for (index, line) in input.split_inclusive(|byte| *byte == b'\n').enumerate() {
        partitions::append_range(&mut ranges, line.len(), index + 1, max_lines, max_bytes)?;
    }
    Ok(ranges)
}

#[cfg(test)]
fn write_partition(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("Partitionsordner fehlt")?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Partition kann nicht vorbereitet werden")?;
    file.as_file_mut()
        .write_all(bytes)
        .map_err(|_| "Partition kann nicht geschrieben werden")?;
    file.as_file()
        .sync_all()
        .map_err(|_| "Partition kann nicht dauerhaft gespeichert werden")?;
    file.persist_noclobber(path)
        .map_err(|_| "Partition kann nicht ohne Überschreiben veröffentlicht werden")?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "Partitionsordner kann nicht dauerhaft gespeichert werden")?;
    Ok(())
}

fn partition_failure(report: &Path, manifest: &mut Value, error: String) -> String {
    manifest["partitioned"] = json!(false);
    manifest["complete"] = json!(false);
    manifest["error"] = json!(error);
    let evidence = match write_report(report, manifest) {
        Ok(()) => "Fehlerbericht wurde gespeichert".to_owned(),
        Err(report_error) => {
            format!("Fehlerbericht konnte nicht gespeichert werden: {report_error}")
        }
    };
    format!(
        "{error}; {evidence}; eigener Ausgabeordner: {}; bestätigte Partitionen: {}",
        manifest["output_dir"].as_str().unwrap_or(""),
        manifest["persisted_partitions"]
            .as_array()
            .map_or(0, Vec::len),
    )
}

async fn pool(args: &Arguments) -> Result<sqlx::PgPool, String> {
    let infisical = args.values.get("--infisical-config").map(PathBuf::from);
    runtime::connect(&args.path("--runtime-config")?, infisical.as_deref()).await
}

async fn publish(args: &Arguments, store: &PgStore, pool: &sqlx::PgPool) -> Result<Value, String> {
    let base = store
        .snapshot(args.required("--base-release")?)
        .await
        .map_err(|_| "Basisrelease kann nicht vollständig gelesen werden")?;
    let heads = if args.sources.iter().any(|source| {
        dbrain_sources::knowledge_import::public_game::SOURCES.contains(&source.as_str())
    }) {
        dbrain_sources::knowledge_import::public_game::read_game_heads(pool, &args.sources).await?
    } else {
        let rows: Vec<Value> = sqlx::query_scalar(
            "SELECT record_json FROM brain.source_record_heads WHERE source_id = ANY($1) ORDER BY source_id,logical_id",
        ).bind(&args.sources).fetch_all(pool).await
            .map_err(|_| "Ausdrücklich benannte Quellköpfe können nicht gelesen werden")?;
        rows.into_iter()
            .map(|row| {
                serde_json::from_value(row)
                    .map_err(|_| "Gespeicherter Quellkopf ist ungültig".to_owned())
            })
            .collect::<Result<Vec<SourceRecordV2>, String>>()?
    };
    let PreparedPublication {
        release,
        expected_heads,
        source_counts: added,
        largest_content_bytes,
        selection,
    } = prepare_publication(args, &base, heads)?;
    let release = store
        .imported_release_for_retry(&release)
        .await
        .map_err(|_| {
            "Bestehender Release hat eine abweichende Identität oder ungültige Erstellungszeit"
        })?;
    let pinned_records = base
        .revisions
        .into_iter()
        .filter(|record| !args.sources.contains(&record.source_id))
        .chain(
            expected_heads
                .iter()
                .filter(|record| {
                    args.sources.contains(&record.source_id)
                        && release.source_revisions[&record.source_id].get(&record.logical_id)
                            == Some(&record.revision)
                })
                .cloned(),
        )
        .collect();
    let index_proof = dbrain_retrieval::preflight_release_index(release.clone(), pinned_records)
        .map_err(|_| "Vollständiger gepinnter Release verletzt die bestehende Index-/Projektionsgrenze oder seinen Herkunftsvertrag; keine Veröffentlichung")?;
    let document_count: usize = release.source_revisions.values().map(BTreeMap::len).sum();
    let previous_pins = base
        .release
        .source_revisions
        .values()
        .map(BTreeMap::len)
        .sum::<usize>();
    let committed_count = if let Some(selection) = &selection {
        store
            .publish_selected_imported_heads_checked(
                args.required("--base-release")?,
                &args.sources,
                &release,
                &expected_heads,
                &selection.selected,
            )
            .await
    } else {
        store
            .publish_imported_heads_checked(
                args.required("--base-release")?,
                &args.sources,
                &release,
                &expected_heads,
            )
            .await
    }
    .map_err(|_| {
        "Releaseveröffentlichung fehlgeschlagen; Kopfstand oder Rechte müssen erneut geprüft werden"
    })?;
    let release = store
        .imported_release_for_retry(&release)
        .await
        .map_err(|_| "Veröffentlichter Retry-Release entspricht nicht dem geprüften Manifest")?;
    let verified = store
        .snapshot(&release.release_id)
        .await
        .map_err(|_| "Veröffentlichter Release kann nicht nachgelesen werden")?;
    let verified_heads: BTreeMap<_, _> = verified
        .heads
        .iter()
        .map(|record| {
            (
                (record.source_id.clone(), record.logical_id.clone()),
                record,
            )
        })
        .collect();
    let release_heads: Vec<_> = expected_heads
        .iter()
        .filter(|record| {
            release.source_revisions[&record.source_id].contains_key(&record.logical_id)
        })
        .collect();
    if verified.release != release
        || verified.revisions.len() != document_count
        || committed_count != document_count
        || verified_heads.len() != release_heads.len()
        || release_heads.iter().any(|record| {
            verified_heads
                .get(&(record.source_id.clone(), record.logical_id.clone()))
                .copied()
                != Some(*record)
        })
    {
        return Err(
            "Nachgelesener Release entspricht nicht dem geprüften Manifest und Kopfstand".into(),
        );
    }
    let base_release_sha256 = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&base.release)
                .map_err(|_| "Basisrelease kann nicht gebunden werden")?
        )
    );
    let candidate_release_sha256 = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&verified.release)
                .map_err(|_| "Kandidatenrelease kann nicht gebunden werden")?
        )
    );
    Ok(json!({
        "published": true, "activated": false, "release_id": release.release_id,
        "knowledge_version": release.knowledge_version, "base_release": base.release.release_id,
        "base_release_sha256": base_release_sha256, "candidate_release_sha256": candidate_release_sha256,
        "previous_pins": previous_pins, "documents": document_count,
        "selected_source_heads": added, "largest_content_bytes": largest_content_bytes,
        "selection": selection.as_ref().map(|selection| selection.report()), "checked_heads": expected_heads.len(),
        "release_index": index_proof,
        "activation_required": "Der bestehende Brain-Dienst muss diesen Release ausdrücklich auswählen."
    }))
}

struct PreparedPublication {
    release: CorpusRelease,
    expected_heads: Vec<SourceRecordV2>,
    source_counts: BTreeMap<String, usize>,
    largest_content_bytes: usize,
    selection: Option<dbrain_sources::knowledge_import::public_game::GameSelection>,
}

fn prepare_publication(
    args: &Arguments,
    base: &brain_contracts::CorpusSnapshot,
    heads: Vec<SourceRecordV2>,
) -> Result<PreparedPublication, String> {
    let created_at_epoch = i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Systemzeit ist ungültig")?
            .as_secs(),
    )
    .map_err(|_| "Systemzeit ist zu groß")?;
    let mut added: BTreeMap<String, usize> = args
        .sources
        .iter()
        .map(|source| (source.clone(), 0))
        .collect();
    let mut largest_content_bytes = 0;
    for record in &heads {
        largest_content_bytes = largest_content_bytes.max(record.content.len());
        *added
            .get_mut(&record.source_id)
            .ok_or("Unerwartete Quelle beim Releaseaufbau")? += 1;
    }
    let selection = if args.sources.iter().any(|source| {
        dbrain_sources::knowledge_import::public_game::SOURCES.contains(&source.as_str())
    }) {
        dbrain_sources::knowledge_import::public_game::validate_sources(&args.sources)?;
        Some(dbrain_sources::knowledge_import::public_game::select_game_publication(&heads)?)
    } else {
        None
    };
    let (release, expected_heads) = if let Some(selection) = &selection {
        PgStore::prepare_selected_imported_release(
            base,
            &args.sources,
            heads,
            &selection.selected,
            args.required("--release-id")?,
            args.required("--knowledge-version")?,
            created_at_epoch,
        )
    } else {
        PgStore::prepare_imported_release(
            base,
            &args.sources,
            heads,
            args.required("--release-id")?,
            args.required("--knowledge-version")?,
            created_at_epoch,
        )
    }
    .map_err(|_| {
        "Release verletzt den bestehenden Vertrag oder enthält ungültige Quellköpfe".to_owned()
    })?;
    Ok(PreparedPublication {
        release,
        expected_heads,
        source_counts: added,
        largest_content_bytes,
        selection,
    })
}

async fn execute(args: Arguments) -> Result<bool, String> {
    let report = args.path("--report")?;
    if args.mode == "partition" {
        let manifest = partition(&args.path("--input")?, &args.path("--output-dir")?, &report)?;
        println!(
            "{}",
            json!({
                "partitioned": true, "complete": true,
                "imported": false, "published": false, "activated": false,
                "partitions": manifest["persisted_partitions"].as_array().map_or(0, Vec::len),
                "report": report,
            })
        );
        return Ok(true);
    }
    report_preflight(&report)?;
    if args.mode == "extract-game" {
        let value = public_game::extract(&args)?;
        write_report(&report, &value)?;
        println!(
            "{}",
            json!({"extracted": true, "imported": false, "report": report})
        );
        return Ok(true);
    }
    if args.mode == "export-legacy-game" {
        let output = args.path("--output")?;
        report_preflight(&output)?;
        if output == report {
            return Err("Ausgabe und Bericht müssen getrennte Dateien sein".into());
        }
        let pool = pool(&args).await?;
        let mut file =
            tempfile::NamedTempFile::new_in(output.parent().ok_or("Ausgabeordner fehlt")?)
                .map_err(|_| "Legacy-Ausgabe kann nicht vorbereitet werden")?;
        let export = {
            let mut writer = std::io::BufWriter::new(file.as_file_mut());
            let export = dbrain_sources::knowledge_import::legacy_game::export_legacy_game(
                &pool,
                &args.sources[0],
                &mut writer,
            )
            .await?;
            writer
                .flush()
                .map_err(|_| "Legacy-Ausgabe kann nicht geschrieben werden")?;
            export
        };
        if export.documents == 0 {
            write_report(
                &report,
                &json!({"exported": false, "complete": false, "source": args.sources[0],
                "documents": 0, "selection": export, "output_persisted": false,
                "imported": false, "published": false, "activated": false}),
            )?;
            pool.close().await;
            return Err("Keine bestätigten öffentlichen Spielfakten ausgewählt; Ausschlüsse stehen im Bericht, keine Ausgabe veröffentlicht".into());
        }
        file.as_file()
            .sync_all()
            .map_err(|_| "Legacy-Ausgabe kann nicht gespeichert werden")?;
        file.persist_noclobber(&output).map_err(|_| {
            "Legacy-Ausgabe existiert bereits oder kann nicht veröffentlicht werden"
        })?;
        File::open(output.parent().ok_or("Ausgabeordner fehlt")?)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| "Ausgabeordner kann nicht gespeichert werden")?;
        write_report(
            &report,
            &json!({"exported": true, "source": args.sources[0], "documents": export.documents, "selection": export, "output": output, "imported": false, "published": false, "activated": false}),
        )?;
        pool.close().await;
        println!(
            "{}",
            json!({"exported": true, "documents": export.documents, "report": report})
        );
        return Ok(true);
    }
    if matches!(args.mode.as_str(), "inspect-game" | "reauthorize-game") {
        let pool = pool(&args).await?;
        let store = PgStore::new(pool.clone());
        store
            .check_core_schema()
            .await
            .map_err(|_| "Bestehendes Core-Schema ist nicht kompatibel")?;
        let value = public_game::inspect_or_authorize(&args, &pool).await?;
        write_report(&report, &value)?;
        pool.close().await;
        println!(
            "{}",
            json!({"complete": true, "report": report, "published": false, "activated": false})
        );
        return Ok(true);
    }
    match args.mode.as_str() {
        "validate" => {
            let (prepared, proof) = prepare(&args)?;
            let complete = proof.input_conflicts == 0 && prepared.skipped_reasons().is_empty();
            let mut value = json!({"validated": true, "complete": complete, "input": proof,
                "rights": prepared.rights(), "skipped_reasons": prepared.skipped_reasons()});
            if let Some(authorization_ref) = args.values.get("--authorization-ref") {
                value["public_game_factual_projection"] =
                    public_game_projection_proof(&prepared, authorization_ref)?;
            }
            write_report(&report, &value)?;
            println!(
                "{}",
                json!({"validated": true, "complete": complete, "report": report})
            );
            Ok(complete)
        }
        "import" => {
            let (prepared, proof) = prepare(&args)?;
            let pool = pool(&args).await?;
            let store = PgStore::new(pool.clone());
            store.check_core_schema().await.map_err(|_| "Bestehendes Core-Schema ist nicht lesbar oder kompatibel; keine automatische Migration")?;
            let summary = import_prepared_knowledge(&store, prepared)
                .await
                .map_err(|_| {
                    "Atomarer Wissensimport fehlgeschlagen; keine vollständige Übernahme bestätigt"
                })?;
            let complete = summary.complete;
            println!(
                "{}",
                json!({"committed": summary.storage.committed, "complete": complete,
                "inserted": summary.storage.inserted, "updated": summary.storage.updated,
                "unchanged": summary.storage.unchanged, "conflicts": summary.storage.conflicts.len(), "report": report})
            );
            write_report(&report, &json!({"input": proof, "result": summary}))?;
            pool.close().await;
            Ok(complete)
        }
        "publish" => {
            let pool = pool(&args).await?;
            let store = PgStore::new(pool.clone());
            store
                .check_core_schema()
                .await
                .map_err(|_| "Bestehendes Core-Schema ist nicht lesbar oder kompatibel")?;
            let value = publish(&args, &store, &pool).await?;
            println!(
                "{}",
                json!({"published": true, "activated": false,
                "release_id": value["release_id"], "report": report})
            );
            write_report(&report, &value)?;
            pool.close().await;
            Ok(true)
        }
        _ => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|_| {
        eprintln!("{}", json!({"error": "panic_redacted"}))
    }));
    match Arguments::parse(std::env::args().skip(1)) {
        Ok(None) => {
            println!(
                "brain-knowledge-import extract-game --source <git-spielquelle> --revision <vollständiger-commit> --runtime-config /etc/deadlock-brain/maintenance-runtime.json --output /pfad/neue-spieldaten.jsonl --report /pfad/neuer-bericht.json\nbrain-knowledge-import export-legacy-game --source <legacy-entities|legacy-patchnotes> --runtime-config /etc/deadlock-brain/maintenance-runtime.json --output /pfad/neue-legacy-daten.jsonl --report /pfad/neuer-bericht.json\nbrain-knowledge-import inspect-game --source <spielquelle> [--source <weitere-spielquelle>] --runtime-config /etc/deadlock-brain/maintenance-runtime.json --report /pfad/neuer-bericht.json\nbrain-knowledge-import reauthorize-game --source <spielquelle> [--source <weitere-spielquelle>] --authorization-ref <ausdrücklicher-freigabenachweis> [--provider-egress-ref <gesonderter-weitergabenachweis>] --runtime-config /etc/deadlock-brain/maintenance-runtime.json --report /pfad/neuer-bericht.json\n\nSpielquellenfreigaben gelten ausschließlich für öffentliche sachliche Spieldaten mit geprüftem Ursprung. Ursprüngliche Lizenzerklärungen bleiben erhalten; daraus entsteht keine Lizenz zur Weitergabe von Rohassets. Die Freigabe schreibt nur monotone Rechteversionen für genau benannte Quellen. Ohne gesonderten Nachweis bleibt die Weitergabe an Modellanbieter gesperrt. Veröffentlichung und Aktivierung bleiben getrennte Schritte. Extraktion liest ausschließlich gepinnte Git-Objekte mit geprüfter Repository-Herkunft. Der Legacy-Export übernimmt nur vorhandene Spielentitäten, deutsche und englische Lokalisierungen sowie offizielle Patchnotes, keine Community- oder Nutzerdaten."
            );
            println!(
                "brain-knowledge-import partition --input /pfad/daten.jsonl --output-dir /pfad/neue-partitionen --report /pfad/neuer-bericht.json\nbrain-knowledge-import validate --input /pfad/daten.jsonl --policy /pfad/rechte.json --parser-revision <stand> --report /pfad/neuer-bericht.json [--authorization-ref <freigabenachweis>]\nbrain-knowledge-import import <dieselben Optionen> --runtime-config /etc/deadlock-brain/maintenance-runtime.json\nbrain-knowledge-import publish --base-release <id> --release-id <neue-id> --knowledge-version <stand> --source <quelle> [--source <weitere-quelle>] --runtime-config /etc/deadlock-brain/maintenance-runtime.json --report /pfad/neuer-bericht.json\n\nPartitionierung prüft die gesamte Eingabe vor der Ausgabe und erhält ihre exakten Bytes. Ausgabeordner müssen neu sein, ihre Elternordner bereits vorhanden. Der Bericht darf direkt im neuen Ausgabeordner liegen, sonst muss sein Elternordner vorhanden sein. Normale Partitionen umfassen höchstens 1.000 Dokumentzeilen und 64 MiB. Größere Einzeldokumente bis 128 MiB erhalten eine eigene Partition. Jede Eingabe ist auf 100.000 historische Dokumentzeilen, 2 GiB Sicherungsbytes und einen 64-MiB-Identitätsindex begrenzt; der Release bleibt getrennt auf 10.000 aktive Pins, 256 MiB Indextext und 500.000 Chunks begrenzt. Import und Validate lesen höchstens 128 MiB je Eingabe. Validate misst mit --authorization-ref zusätzlich die öffentlichen Faktenprojektionen im Speicher und meldet deren genaue UTF-8-Bytes; ohne diese Option bleibt die zusätzliche Probe aus. Diese Probe ändert keine gespeicherten Rechte oder Daten. Import und Veröffentlichung verwenden ausschließlich das dedizierte Ziel und den Infisical-Verweis der normalen Runtime-Konfiguration. Jede Datenbankverbindung bestätigt Rolle, Datenbank, lokalen Socket und Port vor der Verarbeitung. Ein optionaler --infisical-config-Pfad muss mit der Runtime übereinstimmen. Partitionierung importiert, veröffentlicht und aktiviert keine Daten und benötigt weder Importrechte noch Infisical. Import und Veröffentlichung aktivieren keinen Dienst. Bestehende Berichte werden nicht überschrieben. Geheimnisse bleiben im vorhandenen Infisical-Verfahren."
            );
            ExitCode::SUCCESS
        }
        Ok(Some(args)) => match execute(args).await {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::from(2),
            Err(error) => {
                eprintln!("{}", json!({"complete": false, "error": error}));
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("{}", json!({"complete": false, "error": error}));
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Result<Option<Arguments>, String> {
        Arguments::parse(values.iter().map(|value| value.to_string()))
    }

    #[test]
    fn public_game_modes_require_exact_sources_and_explicit_evidence() {
        let common = [
            "--source",
            "deadlock-wiki-deadlock-data",
            "--report",
            "/report",
            "--runtime-config",
            "/runtime",
        ];
        let mut inspect = vec!["inspect-game"];
        inspect.extend(common);
        assert!(args(&inspect).is_ok());
        let mut authorize = vec!["reauthorize-game"];
        authorize.extend(common);
        assert!(args(&authorize).is_err());
        authorize.extend(["--authorization-ref", "operator:explicit-public-facts"]);
        assert!(args(&authorize).is_ok());
        authorize.extend(["--source", "member-data"]);
        assert!(args(&authorize).is_err());
        let mut extract = vec!["extract-game"];
        extract.extend(common);
        extract.extend(["--output", "/output", "--revision", "HEAD"]);
        assert!(args(&extract).is_err());
        extract.pop();
        extract.push("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert!(args(&extract).is_ok());
        let mut export = vec!["export-legacy-game"];
        export.extend(common);
        export.extend(["--output", "/output"]);
        assert!(args(&export).is_err());
        export[2] = "legacy-entities";
        assert!(args(&export).is_ok());
    }

    #[test]
    fn help_has_no_operational_effect() {
        assert!(args(&["--help"]).unwrap().is_none());
    }

    #[test]
    fn validate_requires_absolute_paths_and_explicit_parser() {
        assert!(args(&[
            "validate",
            "--input",
            "relative",
            "--policy",
            "/policy",
            "--report",
            "/report",
            "--parser-revision",
            "v1"
        ])
        .is_err());
        assert!(args(&[
            "validate", "--input", "/input", "--policy", "/policy", "--report", "/report"
        ])
        .is_err());
        assert!(args(&[
            "validate",
            "--input",
            "/input",
            "--policy",
            "/policy",
            "--report",
            "/report",
            "--parser-revision",
            "v1"
        ])
        .is_ok());
    }

    #[test]
    fn validate_factual_projection_requires_the_explicit_offline_option() {
        let mut values = vec![
            "validate",
            "--input",
            "/input",
            "--policy",
            "/policy",
            "--parser-revision",
            "v1",
            "--report",
            "/report",
        ];
        let parsed = args(&values).unwrap().unwrap();
        assert!(!parsed.values.contains_key("--authorization-ref"));
        values.extend(["--authorization-ref", "operator:public-game-facts"]);
        let parsed = args(&values).unwrap().unwrap();
        assert_eq!(
            parsed.required("--authorization-ref").unwrap(),
            "operator:public-game-facts"
        );
        values[0] = "import";
        values.extend(["--runtime-config", "/runtime"]);
        assert!(args(&values).is_err());
    }

    fn prepared_game_fixture() -> PreparedKnowledgeImport {
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(directory.path().join("data/json")).unwrap();
        std::fs::write(
            directory.path().join("data/json/value.json"),
            "{\"Urn\":12}",
        )
        .unwrap();
        std::fs::write(directory.path().join("data/json/empty.json"), "{}").unwrap();
        let commit = "a".repeat(40);
        let options = dbrain_sources::game_files::GameFileOptions {
            root: directory.path().into(),
            app_id: 1422450,
            source_id: "deadlock-wiki-deadlock-data".into(),
            observed_at: "2026-10-09T12:00:00Z".into(),
            build_id: None,
            manifest_id: None,
            source_revision: Some(commit.clone()),
            depot_id: None,
            language: "und".into(),
            attribution: "Valve game data".into(),
            license_name: "unverified".into(),
            license_url: None,
            provenance: json!({"repository_url": "https://github.com/deadlock-wiki/deadlock-data", "git_commit": commit}),
            max_file_bytes: 8388608,
        };
        let mut bytes = Vec::new();
        dbrain_sources::game_files::extract_game_files(&options, &mut bytes).unwrap();
        let input = validate_knowledge_jsonl(Cursor::new(bytes)).unwrap();
        let policy: ImportPolicy = serde_json::from_value(json!({"sources": {"deadlock-wiki-deadlock-data": {
            "internal_read_allowed": true, "raw_retention_allowed": true, "publication_allowed": false,
            "provider_egress_allowed": false, "authorization_ref": "operator:original",
            "provenance_evidence_ref": "evidence:original", "allowed_scopes": ["internal_docs"]
        }}})).unwrap();
        prepare_validated_knowledge(&input, &policy, "game-files-v2").unwrap()
    }

    #[test]
    fn offline_simulation_and_publication_account_for_preserved_excluded_originals() {
        let prepared = prepared_game_fixture();
        let proof = public_game_projection_proof(&prepared, "operator:public").unwrap();
        assert_eq!(proof["documents"], 1);
        assert_eq!(proof["facts"], 1);
        assert_eq!(proof["checked_records"], 2);
        assert_eq!(proof["selection"]["excluded_live"], 1);
        assert_eq!(proof["selection"]["excluded_tombstones"], 0);
        assert_eq!(proof["originals_changed"], false);
        let mut heads: Vec<_> = prepared
            .records()
            .iter()
            .map(|record| record.record.clone())
            .collect();
        let args = publication_args(&["deadlock-wiki-deadlock-data"]);
        let base = publication_base();
        let private = prepare_publication(&args, &base, heads.clone()).unwrap();
        assert!(private.release.source_revisions["deadlock-wiki-deadlock-data"].is_empty());
        for record in &mut heads {
            if record.content != "{}" {
                *record = dbrain_sources::knowledge_import::public_game::authorize_game_record(
                    record,
                    "operator:public",
                    None,
                    2,
                )
                .unwrap();
            }
        }
        let publication = prepare_publication(&args, &base, heads.clone()).unwrap();
        assert_eq!(
            publication.release.source_revisions["deadlock-wiki-deadlock-data"].len(),
            1
        );
        assert!(heads
            .iter()
            .all(|record| publication.expected_heads.contains(record)));
        assert_eq!(
            publication.release.source_revisions["legacy"],
            base.release.source_revisions["legacy"]
        );
        assert_eq!(publication.selection.unwrap().report()["excluded_live"], 1);
    }

    #[test]
    fn publication_requires_new_identity_and_named_sources() {
        assert!(args(&[
            "publish",
            "--base-release",
            "r1",
            "--release-id",
            "r1",
            "--knowledge-version",
            "k1",
            "--source",
            "wiki",
            "--report",
            "/report",
            "--infisical-config",
            "/infisical"
        ])
        .is_err());
        assert!(args(&[
            "publish",
            "--base-release",
            "r1",
            "--release-id",
            "r2",
            "--knowledge-version",
            "k1",
            "--report",
            "/report",
            "--infisical-config",
            "/infisical"
        ])
        .is_err());
    }

    #[test]
    fn operational_modes_require_normal_runtime_without_a_central_dsn_fallback() {
        let mut values = vec![
            "import",
            "--input",
            "/input",
            "--policy",
            "/policy",
            "--parser-revision",
            "v1",
            "--report",
            "/report",
        ];
        assert!(args(&values).is_err());
        values.extend([
            "--runtime-config",
            "/etc/deadlock-brain/maintenance-runtime.json",
        ]);
        assert!(args(&values).is_ok());
        values.extend(["--infisical-config", "/etc/deadlock-brain/infisical.json"]);
        assert!(args(&values).is_ok());
        assert!(args(&["validate", "--runtime-config", "/runtime"]).is_err());
        assert!(args(&["partition", "--runtime-config", "/runtime"]).is_err());
    }

    #[test]
    fn duplicates_and_unrelated_options_are_rejected() {
        assert!(args(&["validate", "--input", "/input", "--input", "/other"]).is_err());
        assert!(args(&["validate", "--source", "wiki"]).is_err());
    }

    fn document_line(page: usize, revision: usize, content: &str, observed_at: &str) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "contract_version": "wiki-spielwissen-v1",
            "source_kind": "wiki",
            "source_id": "deadlock-wiki",
            "document_id": format!("wiki:deadlock-wiki:page:{page}"),
            "source_locator": format!("https://deadlock.wiki/Example_{page}"),
            "title": "Überlieferung",
            "language": "de",
            "revision": revision.to_string(),
            "observed_at": observed_at,
            "content_sha256": format!("{:x}", Sha256::digest(content.as_bytes())),
            "content": content,
            "evidence_status": "source_statement",
            "license": {
                "name": "unverified", "url": null,
                "attribution": "Wiki", "redistribution_allowed": false,
            },
            "metadata": {"original": "äöüß"},
            "facts": [{
                "fact_id": "cooldown", "subject": "hero:example",
                "predicate": "ability.cooldown", "value": 12,
                "unit": "seconds", "evidence_status": "extracted_value",
                "source_span": "section/key", "qualifiers": {"level": 1},
            }],
        }))
        .unwrap()
    }

    fn historical_input(versions: usize) -> Vec<u8> {
        let mut input = Vec::new();
        for revision in 1..=versions {
            input.extend(document_line(1, revision, "Wissen", "2025-01-02T03:04:05Z"));
            if revision != versions {
                input.push(b'\n');
            }
        }
        input
    }

    fn retained_bytes(manifest: &Value) -> Vec<u8> {
        let mut retained = Vec::new();
        for proof in manifest["persisted_partitions"].as_array().unwrap() {
            let bytes = std::fs::read(proof["path"].as_str().unwrap()).unwrap();
            assert!(bytes.len() <= partitions::SINGLETON_MAX_BYTES);
            if bytes.len() > PARTITION_MAX_BYTES {
                assert_eq!(proof["range"]["document_lines"], json!(1));
                assert_eq!(proof["singleton"], json!(true));
            }
            assert!(proof["range"]["document_lines"].as_u64().unwrap() <= 1_000);
            assert_eq!(proof["bytes"], json!(bytes.len()));
            assert_eq!(
                proof["sha256"],
                json!(format!("{:x}", Sha256::digest(&bytes)))
            );
            assert_eq!(
                validate_knowledge_jsonl(Cursor::new(bytes.as_slice()))
                    .unwrap()
                    .documents()
                    .len(),
                proof["range"]["document_lines"].as_u64().unwrap() as usize
            );
            retained.extend(bytes);
        }
        retained
    }

    #[test]
    fn partition_requires_its_three_absolute_paths_without_operational_options() {
        assert!(args(&[
            "partition",
            "--input",
            "/input",
            "--output-dir",
            "/output",
            "--report",
            "/report"
        ])
        .is_ok());
        for missing in ["--input", "--output-dir", "--report"] {
            let values: Vec<&str> = [
                ("--input", "/input"),
                ("--output-dir", "/output"),
                ("--report", "/report"),
            ]
            .into_iter()
            .filter(|(key, _)| *key != missing)
            .flat_map(|(key, value)| [key, value])
            .collect();
            let mut command = vec!["partition"];
            command.extend(values);
            assert!(args(&command).is_err());
        }
        assert!(args(&[
            "partition",
            "--input",
            "relative",
            "--output-dir",
            "/output",
            "--report",
            "/report"
        ])
        .is_err());
        assert!(args(&[
            "partition",
            "--input",
            "/input",
            "--output-dir",
            "relative",
            "--report",
            "/report"
        ])
        .is_err());
        assert!(args(&[
            "partition",
            "--input",
            "/input",
            "--output-dir",
            "/output",
            "--report",
            "relative"
        ])
        .is_err());
        assert!(args(&["partition", "--policy", "/policy"]).is_err());
        assert!(args(&["partition", "--infisical-config", "/infisical"]).is_err());
    }

    #[test]
    fn partition_preserves_more_than_ten_thousand_versions_of_one_document() {
        let directory = tempfile::tempdir().unwrap();
        let input_path = directory.path().join("history.jsonl");
        let output = directory.path().join("parts");
        let report = output.join("manifest.json");
        let input = historical_input(10_001);
        std::fs::write(&input_path, &input).unwrap();
        let manifest = partition(&input_path, &output, &report).unwrap();
        assert_eq!(manifest["input"]["distinct_versions"], json!(10_001));
        assert_eq!(manifest["input"]["distinct_documents"], json!(1));
        assert_eq!(manifest["input"]["source_count"], json!(1));
        assert_eq!(manifest["input"]["facts"], json!(10_001));
        assert_eq!(
            manifest["persisted_partitions"].as_array().unwrap().len(),
            11
        );
        assert_eq!(manifest["partitioned"], json!(true));
        for field in ["imported", "published", "activated"] {
            assert_eq!(manifest[field], json!(false));
        }
        assert_eq!(retained_bytes(&manifest), input);
        assert_eq!(
            manifest["input"]["sha256"],
            json!(format!("{:x}", Sha256::digest(&input)))
        );
        let persisted: Value = serde_json::from_reader(File::open(report).unwrap()).unwrap();
        assert_eq!(persisted, manifest);
    }

    #[test]
    fn partition_count_and_exact_byte_boundaries_are_inclusive() {
        let input = historical_input(1_000);
        let ranges =
            partition_ranges(&input, PARTITION_MAX_DOCUMENT_LINES, PARTITION_MAX_BYTES).unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].document_lines, 1_000);
        assert_eq!(ranges[0].input_byte_end_exclusive, input.len());
        let input = historical_input(1_001);
        let ranges =
            partition_ranges(&input, PARTITION_MAX_DOCUMENT_LINES, PARTITION_MAX_BYTES).unwrap();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[1].first_line, 1_001);
        assert_eq!(ranges[1].last_line, 1_001);
        assert_eq!(ranges[1].document_lines, 1);
        assert_eq!(
            ranges[0].input_byte_end_exclusive,
            ranges[1].input_byte_start
        );
        let ranges = partition_ranges(b"abc\ndef\nx", 1_000, 8).unwrap();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].input_byte_end_exclusive, 8);
        assert_eq!(ranges[0].document_lines, 2);
        assert_eq!(ranges[1].input_byte_start, 8);
        let mut bytes = vec![b'x'; PARTITION_MAX_BYTES];
        assert_eq!(
            partition_ranges(&bytes, 1_000, PARTITION_MAX_BYTES)
                .unwrap()
                .len(),
            1
        );
        bytes.push(b'\n');
        let singleton = partition_ranges(&bytes, 1_000, PARTITION_MAX_BYTES).unwrap();
        assert_eq!(singleton.len(), 1);
        assert_eq!(singleton[0].document_lines, 1);
        let isolated = partition_ranges(b"abc\nabcdefghijkl\ndef", 1_000, 8).unwrap();
        assert_eq!(isolated.len(), 3);
        assert_eq!(isolated[1].document_lines, 1);
        assert_eq!(
            isolated[1].input_byte_start,
            isolated[0].input_byte_end_exclusive
        );
        assert_eq!(
            isolated[1].input_byte_end_exclusive,
            isolated[2].input_byte_start
        );
        assert!(partitions::append_range(
            &mut Vec::new(),
            partitions::SINGLETON_MAX_BYTES + 1,
            1,
            1_000,
            PARTITION_MAX_BYTES
        )
        .unwrap_err()
        .contains("Zeile 1"));
        assert!(partition_ranges(b"a", 0, 1).is_err());
        assert!(partition_ranges(b"a", 1, 0).is_err());
    }

    #[test]
    fn partition_retains_whitespace_duplicates_facts_and_observation_times() {
        let directory = tempfile::tempdir().unwrap();
        let input_path = directory.path().join("history.jsonl");
        let mut input = b" \t".to_vec();
        input.extend(document_line(
            1,
            1,
            "Überlieferung\nOriginal",
            "2020-01-02T03:04:05Z",
        ));
        input.extend(b" \r\n\t");
        input.extend(document_line(
            1,
            1,
            "Überlieferung\nOriginal",
            "2021-02-03T04:05:06+00:00",
        ));
        input.extend(b"  ");
        std::fs::write(&input_path, &input).unwrap();
        let manifest = partition(
            &input_path,
            &directory.path().join("parts"),
            &directory.path().join("report.json"),
        )
        .unwrap();
        assert_eq!(manifest["input"]["document_lines"], json!(2));
        assert_eq!(manifest["input"]["distinct_versions"], json!(1));
        assert_eq!(manifest["input"]["duplicate_lines"], json!(1));
        assert_eq!(manifest["input"]["facts"], json!(2));
        assert_eq!(retained_bytes(&manifest), input);
    }

    #[test]
    fn partition_rejects_conflicts_and_invalid_lines_before_output() {
        let directory = tempfile::tempdir().unwrap();
        let input_path = directory.path().join("history.jsonl");
        let output = directory.path().join("parts");
        let report = directory.path().join("report.json");
        let valid = document_line(1, 1, "Wissen", "2020-01-02T03:04:05Z");
        let mut conflict = valid.clone();
        conflict.push(b'\n');
        conflict.extend(document_line(
            1,
            1,
            "Anderes Wissen",
            "2020-01-02T03:04:05Z",
        ));
        std::fs::write(&input_path, conflict).unwrap();
        assert!(partition(&input_path, &output, &report)
            .unwrap_err()
            .contains("Konflikte"));
        assert!(!output.exists());
        assert!(!report.exists());
        for suffix in [b"\n\n".as_slice(), b"\n{}", b"\n\xff"] {
            let mut invalid = valid.clone();
            invalid.extend(suffix);
            std::fs::write(&input_path, invalid).unwrap();
            assert!(partition(&input_path, &output, &report).is_err());
            assert!(!output.exists());
            assert!(!report.exists());
        }
    }

    #[test]
    fn partition_refuses_existing_outputs_and_report_collisions() {
        let directory = tempfile::tempdir().unwrap();
        let input_path = directory.path().join("history.jsonl");
        let output = directory.path().join("parts");
        let report = directory.path().join("report.json");
        std::fs::write(&input_path, historical_input(1)).unwrap();
        std::fs::create_dir(&output).unwrap();
        let sentinel = output.join("part-000001.jsonl");
        std::fs::write(&sentinel, b"vorhanden").unwrap();
        assert!(partition(&input_path, &output, &report).is_err());
        assert_eq!(std::fs::read(sentinel).unwrap(), b"vorhanden");
        let fresh = directory.path().join("fresh");
        write_report(&report, &json!({"prior": true})).unwrap();
        assert!(partition(&input_path, &fresh, &report).is_err());
        assert!(!fresh.exists());
        assert!(partition(&input_path, &fresh, &fresh.join("part-000001.jsonl")).is_err());
        assert!(!fresh.exists());
        let manifest = partition(&input_path, &fresh, &fresh.join("manifest.json")).unwrap();
        assert_eq!(manifest["partitioned"], json!(true));
        assert!(partition(&input_path, &fresh, &directory.path().join("other.json")).is_err());
    }

    #[test]
    fn partition_write_and_failure_reports_do_not_claim_import_or_overwrite() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("part-000001.jsonl");
        write_partition(&path, b"original").unwrap();
        assert!(write_partition(&path, b"ersetzt").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"original");
        let report = directory.path().join("failure.json");
        let mut manifest = json!({
            "partitioned": false, "complete": false, "imported": false,
            "published": false, "activated": false,
            "output_dir": directory.path(), "output_dir_created_by_this_attempt": true,
            "persisted_partitions": [{"path": path}],
        });
        let error = partition_failure(&report, &mut manifest, "Schreibfehler".into());
        assert!(error.contains("bestätigte Partitionen: 1"));
        let persisted: Value = serde_json::from_reader(File::open(&report).unwrap()).unwrap();
        for field in [
            "partitioned",
            "complete",
            "imported",
            "published",
            "activated",
        ] {
            assert_eq!(persisted[field], json!(false));
        }
        let error = partition_failure(&report, &mut manifest, "Zweiter Fehler".into());
        assert!(error.contains("Fehlerbericht konnte nicht gespeichert werden"));
        let unchanged: Value = serde_json::from_reader(File::open(report).unwrap()).unwrap();
        assert_eq!(persisted, unchanged);
    }

    fn publication_args(sources: &[&str]) -> Arguments {
        let mut values = vec![
            "publish",
            "--base-release",
            "base",
            "--release-id",
            "next",
            "--knowledge-version",
            "k2",
            "--report",
            "/report",
            "--runtime-config",
            "/runtime",
            "--infisical-config",
            "/infisical",
        ];
        for source in sources {
            values.extend(["--source", *source]);
        }
        args(&values).unwrap().unwrap()
    }

    fn imported_head() -> SourceRecordV2 {
        let input = document_line(1, 1, "Wissen", "2020-01-02T03:04:05Z");
        let validated = validate_knowledge_jsonl(Cursor::new(input)).unwrap();
        let policy: ImportPolicy = serde_json::from_value(json!({"sources": {"deadlock-wiki": {
            "internal_read_allowed": true, "raw_retention_allowed": true,
            "publication_allowed": false, "provider_egress_allowed": false,
            "authorization_ref": "fixture-internal-grant", "provenance_evidence_ref": "fixture-origin",
            "allowed_scopes": ["knowledge:read"],
        }}})).unwrap();
        prepare_validated_knowledge(&validated, &policy, "fixture-v1")
            .unwrap()
            .records()[0]
            .record
            .clone()
    }

    fn publication_base() -> brain_contracts::CorpusSnapshot {
        let mut old = imported_head();
        old.source_id = "legacy".into();
        old.logical_id = "old-document".into();
        old.metadata.clear();
        old.revision = 9;
        let release = CorpusRelease {
            release_id: "base".into(),
            knowledge_version: "k1".into(),
            patch: "p1".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([
                (
                    "legacy".into(),
                    BTreeMap::from([("old-document".into(), 2)]),
                ),
                (
                    "deadlock-wiki".into(),
                    BTreeMap::from([("removed-document".into(), 1)]),
                ),
            ]),
        };
        let mut revisions = old.clone();
        revisions.revision = 2;
        brain_contracts::CorpusSnapshot {
            release,
            heads: vec![old],
            revisions: vec![revisions],
        }
    }

    #[test]
    fn publication_replaces_selected_pins_and_preserves_old_pins_and_current_rights() {
        let args = publication_args(&["deadlock-wiki"]);
        let base = publication_base();
        let head = imported_head();
        let prepared = prepare_publication(&args, &base, vec![head.clone()]).unwrap();
        assert_eq!(
            prepared.release.source_revisions["legacy"],
            base.release.source_revisions["legacy"]
        );
        assert_eq!(
            prepared.release.source_revisions["deadlock-wiki"],
            BTreeMap::from([(head.logical_id.clone(), head.revision)])
        );
        assert!(prepared.expected_heads.contains(&base.heads[0]));
        assert!(prepared.expected_heads.contains(&head));
        let origin = brain_contracts::source::origin_from_record(&head).unwrap();
        assert!(!origin.policy.publication_allowed);
        assert!(!origin.policy.provider_egress_allowed);
        assert_eq!(prepared.largest_content_bytes, head.content.len());
    }

    struct ScratchPg {
        directory: tempfile::TempDir,
    }

    impl ScratchPg {
        fn start() -> Self {
            let instance = Self {
                directory: tempfile::tempdir().unwrap(),
            };
            let data = instance.directory.path().join("data");
            let socket = instance.directory.path().join("socket");
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
            let options = format!("-k {} -p 55443 -c listen_addresses=''", socket.display());
            assert!(
                std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                    .arg("-D")
                    .arg(data)
                    .arg("-l")
                    .arg(instance.directory.path().join("postgres.log"))
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
                .arg(self.directory.path().join("data"))
                .args(["-m", "immediate", "-w", "stop"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }

    #[tokio::test]
    async fn publication_rejects_invalid_heads_and_publishes_git_and_wiki_withdrawals() {
        let args = publication_args(&["deadlock-wiki"]);
        let base = publication_base();
        let head = imported_head();
        assert!(prepare_publication(&args, &base, vec![]).is_err());
        assert!(prepare_publication(&args, &base, vec![head.clone(), head.clone()]).is_err());
        let mut tombstone = head.clone();
        tombstone.tombstone = true;
        assert!(prepare_publication(&args, &base, vec![tombstone]).is_err());
        let mut invalid = head;
        invalid.metadata.clear();
        assert!(prepare_publication(&args, &base, vec![invalid]).is_err());

        let pg = ScratchPg::start();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(pg.directory.path().join("socket").to_str().unwrap())
                    .port(55443)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let store = PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        for kind in ["wiki", "game_file"] {
            let mut original = imported_head();
            let mut encoded: Value = serde_json::from_str(
                &original.metadata[brain_storage::source_versions::DOCUMENT_METADATA_KEY],
            )
            .unwrap();
            encoded["source_kind"] = json!(kind);
            original.metadata.insert(
                brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(),
                serde_json::to_string(&encoded).unwrap(),
            );
            original.revision = if kind == "wiki" { 1 } else { 4 };
            store.apply(&original).await.unwrap();
            let mut kept = original.clone();
            kept.source_id = format!("kept-{kind}");
            encoded["source_id"] = json!(kept.source_id);
            kept.metadata.insert(
                brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(),
                serde_json::to_string(&encoded).unwrap(),
            );
            let mut origin = brain_contracts::source::origin_from_record(&original).unwrap();
            origin.identity.source_id = kept.source_id.clone();
            origin.bind_record(&mut kept).unwrap();
            store.apply(&kept).await.unwrap();
            let base = CorpusRelease {
                release_id: format!("base-{kind}"),
                knowledge_version: format!("base-{kind}"),
                patch: "fixture".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([
                    (
                        original.source_id.clone(),
                        BTreeMap::from([(original.logical_id.clone(), original.revision)]),
                    ),
                    (
                        kept.source_id.clone(),
                        BTreeMap::from([(kept.logical_id.clone(), kept.revision)]),
                    ),
                ]),
            };
            store.publish_release(&base).await.unwrap();
            let mut withdrawn = original.clone();
            withdrawn.revision += 1;
            withdrawn.tombstone = true;
            store.apply(&withdrawn).await.unwrap();
            let mut args = publication_args(&[&original.source_id]);
            args.values
                .insert("--base-release".into(), base.release_id.clone());
            args.values
                .insert("--release-id".into(), format!("withdrawn-{kind}"));
            args.values
                .insert("--knowledge-version".into(), format!("withdrawn-{kind}"));
            for _ in 0..2 {
                let proof = publish(&args, &store, &pool).await.unwrap();
                assert_eq!(proof["published"], true);
                assert_eq!(proof["documents"], 1);
                let snapshot = store
                    .snapshot(args.required("--release-id").unwrap())
                    .await
                    .unwrap();
                assert!(snapshot.release.source_revisions[&original.source_id].is_empty());
                assert_eq!(snapshot.revisions, vec![kept.clone()]);
                assert_eq!(snapshot.heads, vec![kept.clone()]);
                assert_eq!(
                    proof["base_release_sha256"],
                    format!("{:x}", Sha256::digest(serde_json::to_vec(&base).unwrap()))
                );
                assert_eq!(
                    proof["candidate_release_sha256"],
                    format!(
                        "{:x}",
                        Sha256::digest(serde_json::to_vec(&snapshot.release).unwrap())
                    )
                );
            }
            let mut forged = withdrawn.clone();
            forged.revision += 1;
            store.apply(&forged).await.unwrap();
            args.values
                .insert("--release-id".into(), format!("unproven-{kind}"));
            args.values
                .insert("--knowledge-version".into(), format!("unproven-{kind}"));
            assert!(publish(&args, &store, &pool).await.is_err());
            assert!(store
                .snapshot(args.required("--release-id").unwrap())
                .await
                .is_err());
        }
        pool.close().await;
    }

    #[test]
    fn publication_accepts_three_sources_but_keeps_the_ten_thousand_document_limit() {
        let mut heads = Vec::new();
        for source in ["deadlock-wiki", "game-tracking", "game-data"] {
            let mut head = imported_head();
            let mut origin = brain_contracts::source::origin_from_record(&head).unwrap();
            head.source_id = source.into();
            origin.identity.source_id = source.into();
            origin.bind_record(&mut head).unwrap();
            heads.push(head);
        }
        let args = publication_args(&["deadlock-wiki", "game-tracking", "game-data"]);
        let mut base = publication_base();
        assert_eq!(
            prepare_publication(&args, &base, heads.clone())
                .unwrap()
                .source_counts
                .len(),
            3
        );
        base.release.source_revisions.insert(
            "legacy-many".into(),
            (0..9_997).map(|id| (format!("document-{id}"), 1)).collect(),
        );
        assert!(prepare_publication(&args, &base, heads.clone()).is_err());
        base.release
            .source_revisions
            .get_mut("legacy-many")
            .unwrap()
            .remove("document-0");
        assert_eq!(
            prepare_publication(&args, &base, heads)
                .unwrap()
                .release
                .source_revisions
                .values()
                .map(BTreeMap::len)
                .sum::<usize>(),
            10_000
        );
    }

    #[test]
    fn report_never_overwrites_prior_evidence() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("proof.json");
        write_report(&path, &json!({"first": true})).unwrap();
        assert!(report_preflight(&path).is_err());
        assert!(write_report(&path, &json!({"first": false})).is_err());
        let value: Value = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        assert_eq!(value, json!({"first": true}));
    }
}
