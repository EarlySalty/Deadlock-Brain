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
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

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
        if !matches!(mode.as_str(), "partition" | "validate" | "import" | "publish") {
            return Err("Unbekannter Unterbefehl".into());
        }
        let mut parsed = Self { mode, values: BTreeMap::new(), sources: Vec::new() };
        while let Some(key) = args.next() {
            let allowed = match parsed.mode.as_str() {
                "partition" => ["--input", "--output-dir", "--report"].as_slice(),
                "validate" => ["--input", "--policy", "--parser-revision", "--report"].as_slice(),
                "import" => ["--input", "--policy", "--parser-revision", "--report", "--infisical-config"].as_slice(),
                "publish" => ["--base-release", "--release-id", "--knowledge-version", "--source", "--report", "--infisical-config"].as_slice(),
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
            _ => unreachable!(),
        }
        if matches!(parsed.mode.as_str(), "import" | "publish") {
            parsed.path("--infisical-config")?;
        }
        Ok(Some(parsed))
    }

    fn required(&self, key: &str) -> Result<&str, String> {
        self.values.get(key).map(String::as_str).ok_or_else(|| format!("Pflichtoption fehlt: {key}"))
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
    let input = File::open(args.path("--input")?).map_err(|_| "Wissenseingabe kann nicht geöffnet werden")?;
    let input_bytes = input.metadata().map_err(|_| "Eingabegröße kann nicht geprüft werden")?.len();
    let validated = validate_knowledge_jsonl(BufReader::new(input)).map_err(|error| error.to_string())?;
    let policy: ImportPolicy = serde_json::from_reader(BufReader::new(
        File::open(args.path("--policy")?).map_err(|_| "Importrechte können nicht geöffnet werden")?,
    )).map_err(|_| "Importrechte entsprechen nicht dem Schema")?;
    let prepared = prepare_validated_knowledge(&validated, &policy, args.required("--parser-revision")?)
        .map_err(|error| error.to_string())?;
    let proof = InputProof {
        input_bytes,
        documents: validated.documents().len(),
        facts: validated.documents().iter().map(|record| record.document.facts.len()).sum(),
        duplicate_lines: validated.duplicates().len(),
        input_conflicts: validated.conflicts().len(),
        unknown_revisions: validated.unknown_revision_count(),
        prepared_records: prepared.records().len(),
        content_bytes: prepared.records().iter().map(|record| record.record.content.len()).sum(),
        largest_content_bytes: prepared.records().iter().map(|record| record.record.content.len()).max().unwrap_or(0),
    };
    Ok((prepared, proof))
}

fn report_preflight(path: &Path) -> Result<(), String> {
    if std::fs::symlink_metadata(path).is_ok() {
        return Err("Berichtsdatei existiert bereits; vorhandene Nachweise werden nicht überschrieben".into());
    }
    if !path.parent().is_some_and(Path::is_dir) {
        return Err("Berichtsordner muss bereits vorhanden sein".into());
    }
    Ok(())
}

fn write_report(path: &Path, value: &Value) -> Result<(), String> {
    let parent = path.parent().ok_or("Berichtsordner fehlt")?;
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|_| "Bericht kann nicht vorbereitet werden")?;
    serde_json::to_writer_pretty(file.as_file_mut(), value).map_err(|_| "Bericht kann nicht serialisiert werden")?;
    file.as_file_mut().write_all(b"\n").map_err(|_| "Bericht kann nicht geschrieben werden")?;
    file.as_file().sync_all().map_err(|_| "Bericht kann nicht dauerhaft gespeichert werden")?;
    file.persist_noclobber(path).map_err(|_| "Bericht kann nicht ohne Überschreiben veröffentlicht werden")?;
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

#[derive(Default)]
struct SourceCounts<'a> {
    documents: BTreeSet<&'a str>,
    versions: BTreeSet<(&'a str, &'a str)>,
    document_lines: usize,
    facts: usize,
}

#[derive(Serialize)]
struct PartitionSourceProof {
    document_lines: usize,
    distinct_documents: usize,
    distinct_versions: usize,
    facts: usize,
}

fn source_counts(
    records: &[dbrain_sources::knowledge_contract::LocatedKnowledgeDocument],
) -> BTreeMap<String, PartitionSourceProof> {
    let mut counts: BTreeMap<&str, SourceCounts<'_>> = BTreeMap::new();
    for record in records {
        let document = &record.document;
        let count = counts.entry(&document.source_id).or_default();
        count.documents.insert(&document.document_id);
        count.versions.insert((&document.document_id, &document.revision));
        count.document_lines += 1;
        count.facts += document.facts.len();
    }
    counts.into_iter().map(|(source, count)| {
        (source.to_owned(), PartitionSourceProof {
            document_lines: count.document_lines,
            distinct_documents: count.documents.len(),
            distinct_versions: count.versions.len(),
            facts: count.facts,
        })
    }).collect()
}

fn partition_ranges(
    input: &[u8], max_lines: usize, max_bytes: usize,
) -> Result<Vec<PartitionRange>, String> {
    if max_lines == 0 || max_bytes == 0 {
        return Err("Partitionsgrenzen müssen größer als null sein".into());
    }
    if input.is_empty() {
        return Ok(Vec::new());
    }
    let mut ranges = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut first_line = 1;
    let mut document_lines = 0;
    for (index, line) in input.split_inclusive(|byte| *byte == b'\n').enumerate() {
        if line.len() > max_bytes {
            return Err(format!(
                "Zeile {} umfasst {} UTF-8-JSONL-Bytes und überschreitet die Partitionsgrenze von {max_bytes} Bytes",
                index + 1, line.len(),
            ));
        }
        if document_lines == max_lines || line.len() > max_bytes - (end - start) {
            ranges.push(PartitionRange {
                input_byte_start: start, input_byte_end_exclusive: end,
                first_line, last_line: index, document_lines,
            });
            start = end;
            first_line = index + 1;
            document_lines = 0;
        }
        end += line.len();
        document_lines += 1;
    }
    if document_lines != 0 {
        ranges.push(PartitionRange {
            input_byte_start: start, input_byte_end_exclusive: end,
            first_line, last_line: first_line + document_lines - 1, document_lines,
        });
    }
    Ok(ranges)
}

fn write_partition(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("Partitionsordner fehlt")?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Partition kann nicht vorbereitet werden")?;
    file.as_file_mut().write_all(bytes).map_err(|_| "Partition kann nicht geschrieben werden")?;
    file.as_file().sync_all().map_err(|_| "Partition kann nicht dauerhaft gespeichert werden")?;
    file.persist_noclobber(path)
        .map_err(|_| "Partition kann nicht ohne Überschreiben veröffentlicht werden")?;
    Ok(())
}

fn partition_failure(report: &Path, manifest: &mut Value, error: String) -> String {
    manifest["partitioned"] = json!(false);
    manifest["complete"] = json!(false);
    manifest["error"] = json!(error);
    let evidence = match write_report(report, manifest) {
        Ok(()) => "Fehlerbericht wurde gespeichert".to_owned(),
        Err(report_error) => format!("Fehlerbericht konnte nicht gespeichert werden: {report_error}"),
    };
    format!("{error}; {evidence}; eigener Ausgabeordner: {}; bestätigte Partitionen: {}",
        manifest["output_dir"].as_str().unwrap_or(""),
        manifest["persisted_partitions"].as_array().map_or(0, Vec::len),
    )
}

fn partition(input_path: &Path, output_dir: &Path, report: &Path) -> Result<Value, String> {
    if report == output_dir {
        return Err("Berichtspfad darf nicht dem Ausgabeordner entsprechen".into());
    }
    match std::fs::symlink_metadata(output_dir) {
        Ok(_) => return Err("Ausgabeordner existiert bereits; ein neuer Ordner ist erforderlich".into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err("Ausgabeordner kann nicht geprüft werden".into()),
    }
    if !output_dir.parent().is_some_and(Path::is_dir) {
        return Err("Übergeordneter Ausgabeordner muss bereits vorhanden sein".into());
    }
    if report.parent() != Some(output_dir) {
        report_preflight(report)?;
    }
    let input = std::fs::read(input_path).map_err(|_| "Wissenseingabe kann nicht gelesen werden")?;
    let validated = validate_knowledge_jsonl(Cursor::new(input.as_slice()))
        .map_err(|error| error.to_string())?;
    if let Some(conflict) = validated.conflicts().first() {
        return Err(format!(
            "Eingabe enthält {} Konflikte; erster Konflikt zwischen Zeilen {} und {}; keine Ausgabe erstellt",
            validated.conflicts().len(), conflict.first_line, conflict.line,
        ));
    }
    let ranges = partition_ranges(&input, PARTITION_MAX_DOCUMENT_LINES, PARTITION_MAX_BYTES)?;
    if ranges.iter().map(|range| range.document_lines).sum::<usize>() != validated.documents().len() {
        return Err("Partitionszeilen entsprechen nicht den geprüften Dokumentzeilen; keine Ausgabe erstellt".into());
    }
    let mut plans = Vec::with_capacity(ranges.len());
    for (index, range) in ranges.iter().enumerate() {
        let path = output_dir.join(format!("part-{:06}.jsonl", index + 1));
        if path == report {
            return Err("Berichtspfad darf keine Partitionsdatei belegen".into());
        }
        let bytes = &input[range.input_byte_start..range.input_byte_end_exclusive];
        plans.push(json!({
            "index": index + 1,
            "path": path,
            "range": range,
            "bytes": bytes.len(),
            "sha256": format!("{:x}", Sha256::digest(bytes)),
            "sources": source_counts(&validated.documents()[range.first_line - 1..range.last_line]),
        }));
    }
    let sources = source_counts(validated.documents());
    let distinct_documents: usize = sources.values().map(|count| count.distinct_documents).sum();
    let distinct_versions: usize = sources.values().map(|count| count.distinct_versions).sum();
    let mut manifest = json!({
        "mode": "partition",
        "partitioned": false,
        "complete": false,
        "imported": false,
        "published": false,
        "activated": false,
        "input": {
            "path": input_path,
            "bytes": input.len(),
            "sha256": format!("{:x}", Sha256::digest(&input)),
            "document_lines": validated.documents().len(),
            "distinct_documents": distinct_documents,
            "distinct_versions": distinct_versions,
            "source_count": sources.len(),
            "sources": sources,
            "facts": validated.documents().iter().map(|record| record.document.facts.len()).sum::<usize>(),
            "duplicate_lines": validated.duplicates().len(),
            "input_conflicts": 0,
            "unknown_revision_lines": validated.unknown_revision_count(),
        },
        "limits": {"document_lines": PARTITION_MAX_DOCUMENT_LINES, "bytes": PARTITION_MAX_BYTES},
        "output_dir": output_dir,
        "output_dir_created_by_this_attempt": true,
        "planned_partitions": plans,
        "persisted_partitions": [],
        "report": report,
    });
    std::fs::create_dir(output_dir)
        .map_err(|_| "Neuer Ausgabeordner kann nicht angelegt werden; keine eigene Ausgabe bestätigt")?;
    for (index, range) in ranges.iter().enumerate() {
        let path = output_dir.join(format!("part-{:06}.jsonl", index + 1));
        let bytes = &input[range.input_byte_start..range.input_byte_end_exclusive];
        if let Err(error) = write_partition(&path, bytes) {
            return Err(partition_failure(report, &mut manifest, format!("Partition {}: {error}", index + 1)));
        }
        let proof = manifest["planned_partitions"][index].clone();
        if let Some(persisted) = manifest["persisted_partitions"].as_array_mut() {
            persisted.push(proof);
        } else {
            return Err(partition_failure(report, &mut manifest, "Partitionsnachweis ist ungültig".into()));
        }
    }
    if File::open(output_dir).and_then(|directory| directory.sync_all()).is_err() {
        return Err(partition_failure(report, &mut manifest, "Ausgabeordner kann nicht dauerhaft gespeichert werden".into()));
    }
    manifest["partitioned"] = json!(true);
    manifest["complete"] = json!(true);
    if let Err(error) = write_report(report, &manifest) {
        return Err(partition_failure(report, &mut manifest, error));
    }
    Ok(manifest)
}

async fn pool(args: &Arguments) -> Result<sqlx::PgPool, String> {
    dbrain_sources::core::pg::pg_pool_from_config(&args.path("--infisical-config")?, false)
        .await.map_err(|_| "Datenbankverbindung über Infisical fehlgeschlagen".into())
}

async fn publish(args: &Arguments, store: &PgStore, pool: &sqlx::PgPool) -> Result<Value, String> {
    let base = store.snapshot(args.required("--base-release")?).await
        .map_err(|_| "Basisrelease kann nicht vollständig gelesen werden")?;
    let rows: Vec<Value> = sqlx::query_scalar(
        "SELECT record_json FROM brain.source_record_heads WHERE source_id = ANY($1) ORDER BY source_id,logical_id",
    ).bind(&args.sources).fetch_all(pool).await.map_err(|_| "Ausdrücklich benannte Quellköpfe können nicht gelesen werden")?;
    let heads = rows.into_iter().map(|row| serde_json::from_value(row)
        .map_err(|_| "Gespeicherter Quellkopf ist ungültig".to_owned()))
        .collect::<Result<Vec<SourceRecordV2>, String>>()?;
    let PreparedPublication { release, expected_heads, source_counts: added, largest_content_bytes } =
        prepare_publication(args, &base, heads)?;
    let document_count: usize = release.source_revisions.values().map(BTreeMap::len).sum();
    let previous_pins = base.release.source_revisions.values().map(BTreeMap::len).sum::<usize>();
    let committed_count = store.publish_imported_heads_checked(
        args.required("--base-release")?, &args.sources, &release, &expected_heads,
    ).await.map_err(|_| "Releaseveröffentlichung fehlgeschlagen; Kopfstand oder Rechte müssen erneut geprüft werden")?;
    let verified = store.snapshot(&release.release_id).await
        .map_err(|_| "Veröffentlichter Release kann nicht nachgelesen werden")?;
    let verified_heads: BTreeMap<_, _> = verified.heads.iter()
        .map(|record| ((record.source_id.clone(), record.logical_id.clone()), record)).collect();
    if verified.release != release || verified.revisions.len() != document_count
        || committed_count != document_count || verified_heads.len() != expected_heads.len()
        || expected_heads.iter().any(|record| {
            verified_heads.get(&(record.source_id.clone(), record.logical_id.clone())).copied() != Some(record)
        })
    {
        return Err("Nachgelesener Release entspricht nicht dem geprüften Manifest und Kopfstand".into());
    }
    Ok(json!({
        "published": true,
        "activated": false,
        "release_id": release.release_id,
        "knowledge_version": release.knowledge_version,
        "base_release": base.release.release_id,
        "previous_pins": previous_pins,
        "documents": document_count,
        "selected_source_heads": added,
        "largest_content_bytes": largest_content_bytes,
        "activation_required": "Der bestehende Brain-Dienst muss diesen Release ausdrücklich auswählen."
    }))
}

struct PreparedPublication {
    release: CorpusRelease,
    expected_heads: Vec<SourceRecordV2>,
    source_counts: BTreeMap<String, usize>,
    largest_content_bytes: usize,
}

fn prepare_publication(
    args: &Arguments,
    base: &brain_contracts::CorpusSnapshot,
    heads: Vec<SourceRecordV2>,
) -> Result<PreparedPublication, String> {
    let mut expected: BTreeMap<(String, String), SourceRecordV2> = base.heads.iter()
        .filter(|record| !args.sources.contains(&record.source_id))
        .map(|record| ((record.source_id.clone(), record.logical_id.clone()), record.clone())).collect();
    let mut release = CorpusRelease {
        release_id: args.required("--release-id")?.to_owned(),
        knowledge_version: args.required("--knowledge-version")?.to_owned(),
        patch: base.release.patch.clone(),
        created_at_epoch: i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|_| "Systemzeit ist ungültig")?.as_secs()).map_err(|_| "Systemzeit ist zu groß")?,
        source_revisions: base.release.source_revisions.clone(),
    };
    for source in &args.sources {
        release.source_revisions.remove(source);
    }
    let mut added: BTreeMap<String, usize> = args.sources.iter().map(|source| (source.clone(), 0)).collect();
    let mut largest_content_bytes = 0;
    for record in heads {
        if record.tombstone {
            return Err("Benannte Quelle enthält einen zurückgezogenen Kopf; Release bleibt unverändert".into());
        }
        record.validate().map_err(|_| "Gespeicherter Quellkopf verletzt den Vertrag")?;
        brain_contracts::source::origin_from_record(&record).map_err(|_| "Importierter Quellkopf hat keinen gültigen Herkunftsnachweis")?;
        largest_content_bytes = largest_content_bytes.max(record.content.len());
        *added.get_mut(&record.source_id).ok_or("Unerwartete Quelle beim Releaseaufbau")? += 1;
        release.source_revisions.entry(record.source_id.clone()).or_default()
            .insert(record.logical_id.clone(), record.revision);
        if expected.insert((record.source_id.clone(), record.logical_id.clone()), record).is_some() {
            return Err("Gespeicherte Quelle enthält einen doppelten Kopf".into());
        }
    }
    if added.values().any(|count| *count == 0) {
        return Err("Mindestens eine benannte Quelle hat keine gespeicherten Dokumente".into());
    }
    let document_count: usize = release.source_revisions.values().map(BTreeMap::len).sum();
    if document_count > 10_000 {
        return Err("Release überschreitet die bestehende Grenze von 10.000 Dokumenten".into());
    }
    brain_storage::validate_release(&release).map_err(|_| "Release verletzt den bestehenden Vertrag".to_owned())?;
    Ok(PreparedPublication {
        release, expected_heads: expected.into_values().collect(), source_counts: added,
        largest_content_bytes,
    })
}

async fn execute(args: Arguments) -> Result<bool, String> {
    let report = args.path("--report")?;
    if args.mode == "partition" {
        let manifest = partition(&args.path("--input")?, &args.path("--output-dir")?, &report)?;
        println!("{}", json!({
            "partitioned": true, "complete": true,
            "imported": false, "published": false, "activated": false,
            "partitions": manifest["persisted_partitions"].as_array().map_or(0, Vec::len),
            "report": report,
        }));
        return Ok(true);
    }
    report_preflight(&report)?;
    match args.mode.as_str() {
        "validate" => {
            let (prepared, proof) = prepare(&args)?;
            let complete = proof.input_conflicts == 0 && prepared.skipped_reasons().is_empty();
            let value = json!({"validated": true, "complete": complete, "input": proof,
                "rights": prepared.rights(), "skipped_reasons": prepared.skipped_reasons()});
            write_report(&report, &value)?;
            println!("{}", json!({"validated": true, "complete": complete, "report": report}));
            Ok(complete)
        }
        "import" => {
            let (prepared, proof) = prepare(&args)?;
            let pool = pool(&args).await?;
            let store = PgStore::new(pool.clone());
            store.check_core_schema().await.map_err(|_| "Bestehendes Core-Schema ist nicht lesbar oder kompatibel; keine automatische Migration")?;
            let summary = import_prepared_knowledge(&store, prepared).await
                .map_err(|_| "Atomarer Wissensimport fehlgeschlagen; keine vollständige Übernahme bestätigt")?;
            let complete = summary.complete;
            println!("{}", json!({"committed": summary.storage.committed, "complete": complete,
                "inserted": summary.storage.inserted, "updated": summary.storage.updated,
                "unchanged": summary.storage.unchanged, "conflicts": summary.storage.conflicts.len(), "report": report}));
            write_report(&report, &json!({"input": proof, "result": summary}))?;
            pool.close().await;
            Ok(complete)
        }
        "publish" => {
            let pool = pool(&args).await?;
            let store = PgStore::new(pool.clone());
            store.check_core_schema().await.map_err(|_| "Bestehendes Core-Schema ist nicht lesbar oder kompatibel")?;
            let value = publish(&args, &store, &pool).await?;
            println!("{}", json!({"published": true, "activated": false,
                "release_id": value["release_id"], "report": report}));
            write_report(&report, &value)?;
            pool.close().await;
            Ok(true)
        }
        _ => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|_| eprintln!("{}", json!({"error": "panic_redacted"}))));
    match Arguments::parse(std::env::args().skip(1)) {
        Ok(None) => {
            println!("brain-knowledge-import partition --input /pfad/daten.jsonl --output-dir /pfad/neue-partitionen --report /pfad/neuer-bericht.json\nbrain-knowledge-import validate --input /pfad/daten.jsonl --policy /pfad/rechte.json --parser-revision <stand> --report /pfad/neuer-bericht.json\nbrain-knowledge-import import <dieselben Optionen> --infisical-config /pfad/infisical.json\nbrain-knowledge-import publish --base-release <id> --release-id <neue-id> --knowledge-version <stand> --source <quelle> [--source <weitere-quelle>] --infisical-config /pfad/infisical.json --report /pfad/neuer-bericht.json\n\nPartitionierung prüft die gesamte Eingabe vor der Ausgabe und erhält ihre exakten Bytes. Ausgabeordner müssen neu sein, ihre Elternordner bereits vorhanden. Der Bericht darf direkt im neuen Ausgabeordner liegen, sonst muss sein Elternordner vorhanden sein. Jede Partition umfasst höchstens 1.000 Dokumentzeilen und 64 MiB. Partitionierung importiert, veröffentlicht und aktiviert keine Daten und benötigt weder Importrechte noch Infisical. Import und Veröffentlichung aktivieren keinen Dienst. Bestehende Berichte werden nicht überschrieben. Geheimnisse bleiben im vorhandenen Infisical-Verfahren.");
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
    fn help_has_no_operational_effect() {
        assert!(args(&["--help"]).unwrap().is_none());
    }

    #[test]
    fn validate_requires_absolute_paths_and_explicit_parser() {
        assert!(args(&["validate", "--input", "relative", "--policy", "/policy", "--report", "/report", "--parser-revision", "v1"]).is_err());
        assert!(args(&["validate", "--input", "/input", "--policy", "/policy", "--report", "/report"]).is_err());
        assert!(args(&["validate", "--input", "/input", "--policy", "/policy", "--report", "/report", "--parser-revision", "v1"]).is_ok());
    }

    #[test]
    fn publication_requires_new_identity_and_named_sources() {
        assert!(args(&["publish", "--base-release", "r1", "--release-id", "r1", "--knowledge-version", "k1", "--source", "wiki", "--report", "/report", "--infisical-config", "/infisical"]).is_err());
        assert!(args(&["publish", "--base-release", "r1", "--release-id", "r2", "--knowledge-version", "k1", "--report", "/report", "--infisical-config", "/infisical"]).is_err());
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
        })).unwrap()
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
            assert!(bytes.len() <= PARTITION_MAX_BYTES);
            assert!(proof["range"]["document_lines"].as_u64().unwrap() <= 1_000);
            assert_eq!(proof["bytes"], json!(bytes.len()));
            assert_eq!(proof["sha256"], json!(format!("{:x}", Sha256::digest(&bytes))));
            assert_eq!(validate_knowledge_jsonl(Cursor::new(bytes.as_slice())).unwrap().documents().len(),
                proof["range"]["document_lines"].as_u64().unwrap() as usize);
            retained.extend(bytes);
        }
        retained
    }

    #[test]
    fn partition_requires_its_three_absolute_paths_without_operational_options() {
        assert!(args(&["partition", "--input", "/input", "--output-dir", "/output", "--report", "/report"]).is_ok());
        for missing in ["--input", "--output-dir", "--report"] {
            let values: Vec<&str> = [
                ("--input", "/input"), ("--output-dir", "/output"), ("--report", "/report"),
            ].into_iter().filter(|(key, _)| *key != missing)
                .flat_map(|(key, value)| [key, value]).collect();
            let mut command = vec!["partition"];
            command.extend(values);
            assert!(args(&command).is_err());
        }
        assert!(args(&["partition", "--input", "relative", "--output-dir", "/output", "--report", "/report"]).is_err());
        assert!(args(&["partition", "--input", "/input", "--output-dir", "relative", "--report", "/report"]).is_err());
        assert!(args(&["partition", "--input", "/input", "--output-dir", "/output", "--report", "relative"]).is_err());
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
        assert_eq!(manifest["persisted_partitions"].as_array().unwrap().len(), 11);
        assert_eq!(manifest["partitioned"], json!(true));
        for field in ["imported", "published", "activated"] {
            assert_eq!(manifest[field], json!(false));
        }
        assert_eq!(retained_bytes(&manifest), input);
        assert_eq!(manifest["input"]["sha256"], json!(format!("{:x}", Sha256::digest(&input))));
        let persisted: Value = serde_json::from_reader(File::open(report).unwrap()).unwrap();
        assert_eq!(persisted, manifest);
    }

    #[test]
    fn partition_count_and_exact_byte_boundaries_are_inclusive() {
        let input = historical_input(1_000);
        let ranges = partition_ranges(&input, PARTITION_MAX_DOCUMENT_LINES, PARTITION_MAX_BYTES).unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].document_lines, 1_000);
        assert_eq!(ranges[0].input_byte_end_exclusive, input.len());
        let input = historical_input(1_001);
        let ranges = partition_ranges(&input, PARTITION_MAX_DOCUMENT_LINES, PARTITION_MAX_BYTES).unwrap();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[1].first_line, 1_001);
        assert_eq!(ranges[1].last_line, 1_001);
        assert_eq!(ranges[1].document_lines, 1);
        assert_eq!(ranges[0].input_byte_end_exclusive, ranges[1].input_byte_start);
        let ranges = partition_ranges(b"abc\ndef\nx", 1_000, 8).unwrap();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].input_byte_end_exclusive, 8);
        assert_eq!(ranges[0].document_lines, 2);
        assert_eq!(ranges[1].input_byte_start, 8);
        let mut bytes = vec![b'x'; PARTITION_MAX_BYTES];
        assert_eq!(partition_ranges(&bytes, 1_000, PARTITION_MAX_BYTES).unwrap().len(), 1);
        bytes.push(b'\n');
        assert!(partition_ranges(&bytes, 1_000, PARTITION_MAX_BYTES).unwrap_err().contains("Zeile 1"));
        assert!(partition_ranges(b"a", 0, 1).is_err());
        assert!(partition_ranges(b"a", 1, 0).is_err());
    }

    #[test]
    fn partition_retains_whitespace_duplicates_facts_and_observation_times() {
        let directory = tempfile::tempdir().unwrap();
        let input_path = directory.path().join("history.jsonl");
        let mut input = b" \t".to_vec();
        input.extend(document_line(1, 1, "Überlieferung\nOriginal", "2020-01-02T03:04:05Z"));
        input.extend(b" \r\n\t");
        input.extend(document_line(1, 1, "Überlieferung\nOriginal", "2021-02-03T04:05:06+00:00"));
        input.extend(b"  ");
        std::fs::write(&input_path, &input).unwrap();
        let manifest = partition(&input_path, &directory.path().join("parts"), &directory.path().join("report.json")).unwrap();
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
        conflict.extend(document_line(1, 1, "Anderes Wissen", "2020-01-02T03:04:05Z"));
        std::fs::write(&input_path, conflict).unwrap();
        assert!(partition(&input_path, &output, &report).unwrap_err().contains("Konflikte"));
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
        for field in ["partitioned", "complete", "imported", "published", "activated"] {
            assert_eq!(persisted[field], json!(false));
        }
        let error = partition_failure(&report, &mut manifest, "Zweiter Fehler".into());
        assert!(error.contains("Fehlerbericht konnte nicht gespeichert werden"));
        let unchanged: Value = serde_json::from_reader(File::open(report).unwrap()).unwrap();
        assert_eq!(persisted, unchanged);
    }

    fn publication_args(sources: &[&str]) -> Arguments {
        let mut values = vec!["publish", "--base-release", "base", "--release-id", "next",
            "--knowledge-version", "k2", "--report", "/report", "--infisical-config", "/infisical"];
        for source in sources { values.extend(["--source", *source]); }
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
        prepare_validated_knowledge(&validated, &policy, "fixture-v1").unwrap().records()[0].record.clone()
    }

    fn publication_base() -> brain_contracts::CorpusSnapshot {
        let mut old = imported_head();
        old.source_id = "legacy".into();
        old.logical_id = "old-document".into();
        old.metadata.clear();
        old.revision = 9;
        let release = CorpusRelease {
            release_id: "base".into(), knowledge_version: "k1".into(), patch: "p1".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([
                ("legacy".into(), BTreeMap::from([("old-document".into(), 2)])),
                ("deadlock-wiki".into(), BTreeMap::from([("removed-document".into(), 1)])),
            ]),
        };
        let mut revisions = old.clone();
        revisions.revision = 2;
        brain_contracts::CorpusSnapshot { release, heads: vec![old], revisions: vec![revisions] }
    }

    #[test]
    fn publication_replaces_selected_pins_and_preserves_old_pins_and_current_rights() {
        let args = publication_args(&["deadlock-wiki"]);
        let base = publication_base();
        let head = imported_head();
        let prepared = prepare_publication(&args, &base, vec![head.clone()]).unwrap();
        assert_eq!(prepared.release.source_revisions["legacy"], base.release.source_revisions["legacy"]);
        assert_eq!(prepared.release.source_revisions["deadlock-wiki"],
            BTreeMap::from([(head.logical_id.clone(), head.revision)]));
        assert!(prepared.expected_heads.contains(&base.heads[0]));
        assert!(prepared.expected_heads.contains(&head));
        let origin = brain_contracts::source::origin_from_record(&head).unwrap();
        assert!(!origin.policy.publication_allowed);
        assert!(!origin.policy.provider_egress_allowed);
        assert_eq!(prepared.largest_content_bytes, head.content.len());
    }

    #[test]
    fn publication_rejects_empty_duplicate_tombstone_and_invalid_origin_heads() {
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
        assert_eq!(prepare_publication(&args, &base, heads.clone()).unwrap().source_counts.len(), 3);
        base.release.source_revisions.insert("legacy-many".into(),
            (0..9_997).map(|id| (format!("document-{id}"), 1)).collect());
        assert!(prepare_publication(&args, &base, heads.clone()).is_err());
        base.release.source_revisions.get_mut("legacy-many").unwrap().remove("document-0");
        assert_eq!(prepare_publication(&args, &base, heads).unwrap().release.source_revisions
            .values().map(BTreeMap::len).sum::<usize>(), 10_000);
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
