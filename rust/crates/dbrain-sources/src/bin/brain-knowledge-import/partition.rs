use super::{
    partition_failure, report_preflight, write_report, PartitionRange, PartitionSourceProof,
    PARTITION_MAX_BYTES, PARTITION_MAX_DOCUMENT_LINES,
};
use dbrain_sources::knowledge_contract::{
    validate_knowledge_jsonl_seekable, KnowledgeDocument, KnowledgeStreamLimits,
    KnowledgeValidationError,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub(super) const SINGLETON_MAX_BYTES: usize = 128 * 1024 * 1024;
const INPUT_MAX_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const INPUT_MAX_LINES: usize = 100_000;
const INDEX_MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(Default)]
struct SourceCounts {
    documents: BTreeSet<String>,
    versions: BTreeSet<(String, String)>,
    document_lines: usize,
    facts: usize,
}

fn observe(counts: &mut BTreeMap<String, SourceCounts>, document: &KnowledgeDocument) {
    let count = counts.entry(document.source_id.clone()).or_default();
    count.documents.insert(document.document_id.clone());
    count
        .versions
        .insert((document.document_id.clone(), document.revision.clone()));
    count.document_lines += 1;
    count.facts += document.facts.len();
}

fn proof(counts: BTreeMap<String, SourceCounts>) -> BTreeMap<String, PartitionSourceProof> {
    counts
        .into_iter()
        .map(|(source, count)| {
            (
                source,
                PartitionSourceProof {
                    document_lines: count.document_lines,
                    distinct_documents: count.documents.len(),
                    distinct_versions: count.versions.len(),
                    facts: count.facts,
                },
            )
        })
        .collect()
}

pub(super) fn append_range(
    ranges: &mut Vec<PartitionRange>,
    length: usize,
    line: usize,
    max_lines: usize,
    max_bytes: usize,
) -> Result<usize, String> {
    if max_lines == 0 || max_bytes == 0 {
        return Err("Partitionsgrenzen müssen größer als null sein".into());
    }
    if length > SINGLETON_MAX_BYTES {
        return Err(format!("Zeile {line} überschreitet die begrenzte Singletongröße von {SINGLETON_MAX_BYTES} Bytes"));
    }
    let new = ranges.last().is_none_or(|range| {
        range.document_lines == max_lines
            || length
                > max_bytes.saturating_sub(range.input_byte_end_exclusive - range.input_byte_start)
            || range.input_byte_end_exclusive - range.input_byte_start > max_bytes
    });
    if new {
        let start = ranges
            .last()
            .map_or(0, |range| range.input_byte_end_exclusive);
        ranges.push(PartitionRange {
            input_byte_start: start,
            input_byte_end_exclusive: start,
            first_line: line,
            last_line: line,
            document_lines: 0,
        });
    }
    let index = ranges.len() - 1;
    let range = &mut ranges[index];
    range.input_byte_end_exclusive += length;
    range.last_line = line;
    range.document_lines += 1;
    Ok(index)
}

fn snapshot(
    input_path: &Path,
    parent: &Path,
) -> Result<(tempfile::NamedTempFile, usize, String), String> {
    let mut input =
        File::open(input_path).map_err(|_| "Wissenseingabe kann nicht geöffnet werden")?;
    let before = input
        .metadata()
        .map_err(|_| "Eingabeidentität kann nicht geprüft werden")?;
    if !before.is_file() || before.len() > INPUT_MAX_BYTES {
        return Err("Reguläre Eingabedatei bis höchstens 2 GiB erforderlich".into());
    }
    let mut saved = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Private Eingabesicherung kann nicht vorbereitet werden")?;
    let mut buffer = [0u8; 64 * 1024];
    let mut hash = Sha256::new();
    let mut total = 0u64;
    loop {
        let size = input
            .read(&mut buffer)
            .map_err(|_| "Eingabesicherung kann nicht gelesen werden")?;
        if size == 0 {
            break;
        }
        total = total
            .checked_add(size as u64)
            .filter(|size| *size <= INPUT_MAX_BYTES)
            .ok_or("Eingabe überschreitet die Grenze von 2 GiB")?;
        saved
            .as_file_mut()
            .write_all(&buffer[..size])
            .map_err(|_| "Private Eingabesicherung kann nicht geschrieben werden")?;
        hash.update(&buffer[..size]);
    }
    let after = input
        .metadata()
        .map_err(|_| "Eingabeidentität kann nicht erneut geprüft werden")?;
    if before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.len() != after.len()
        || total != after.len()
        || before.mtime() != after.mtime()
        || before.mtime_nsec() != after.mtime_nsec()
        || before.ctime() != after.ctime()
        || before.ctime_nsec() != after.ctime_nsec()
    {
        return Err(
            "Eingabe wurde während der privaten Sicherung verändert; keine Ausgabe erstellt".into(),
        );
    }
    Ok((saved, total as usize, format!("{:x}", hash.finalize())))
}

fn write_range(path: &Path, input: &mut File, range: &PartitionRange) -> Result<(), String> {
    let parent = path.parent().ok_or("Partitionsordner fehlt")?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Partition kann nicht vorbereitet werden")?;
    input
        .seek(SeekFrom::Start(range.input_byte_start as u64))
        .map_err(|_| "Geprüfter Partitionsbereich kann nicht gelesen werden")?;
    let length = (range.input_byte_end_exclusive - range.input_byte_start) as u64;
    let copied = std::io::copy(&mut input.take(length), file.as_file_mut())
        .map_err(|_| "Partition kann nicht geschrieben werden")?;
    if copied != length {
        return Err("Private Eingabesicherung wurde verkürzt".into());
    }
    file.as_file()
        .sync_all()
        .map_err(|_| "Partition kann nicht dauerhaft gespeichert werden")?;
    file.persist_noclobber(path)
        .map_err(|_| "Partition kann nicht ohne Überschreiben veröffentlicht werden")?;
    Ok(())
}

pub(super) fn partition(
    input_path: &Path,
    output_dir: &Path,
    report: &Path,
) -> Result<Value, String> {
    if report == output_dir {
        return Err("Berichtspfad darf nicht dem Ausgabeordner entsprechen".into());
    }
    match std::fs::symlink_metadata(output_dir) {
        Ok(_) => {
            return Err("Ausgabeordner existiert bereits; ein neuer Ordner ist erforderlich".into())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err("Ausgabeordner kann nicht geprüft werden".into()),
    }
    let parent = output_dir
        .parent()
        .filter(|path| path.is_dir())
        .ok_or("Übergeordneter Ausgabeordner muss bereits vorhanden sein")?;
    if report.parent() != Some(output_dir) {
        report_preflight(report)?;
    }
    let (mut saved, input_bytes, input_sha256) = snapshot(input_path, parent)?;
    let mut ranges = Vec::new();
    let mut partitions: Vec<BTreeMap<String, SourceCounts>> = Vec::new();
    let mut hashes: Vec<Sha256> = Vec::new();
    let mut sources = BTreeMap::new();
    let mut checked_hash = Sha256::new();
    let mut unknown_revisions = 0usize;
    let validated = validate_knowledge_jsonl_seekable(
        BufReader::new(saved.as_file_mut()),
        KnowledgeStreamLimits {
            max_line_bytes: SINGLETON_MAX_BYTES,
            max_input_bytes: INPUT_MAX_BYTES,
            max_document_lines: INPUT_MAX_LINES,
            max_index_bytes: INDEX_MAX_BYTES,
        },
        |record, line, bytes| {
            let index = append_range(
                &mut ranges,
                bytes.len(),
                line.line,
                PARTITION_MAX_DOCUMENT_LINES,
                PARTITION_MAX_BYTES,
            )
            .map_err(|message| KnowledgeValidationError {
                line: line.line,
                field: "$".into(),
                message,
            })?;
            if index == partitions.len() {
                partitions.push(BTreeMap::new());
                hashes.push(Sha256::new());
            }
            observe(&mut partitions[index], &record.document);
            observe(&mut sources, &record.document);
            hashes[index].update(bytes);
            checked_hash.update(bytes);
            unknown_revisions += usize::from(record.document.revision_is_unknown());
            Ok(())
        },
    )
    .map_err(|error| error.to_string())?;
    if let Some(conflict) = validated.conflicts.first() {
        return Err(format!("Eingabe enthält {} Konflikte; erster Konflikt zwischen Zeilen {} und {}; keine Ausgabe erstellt",
            validated.conflicts.len(), conflict.first_line, conflict.line));
    }
    if validated.input_bytes != input_bytes as u64
        || format!("{:x}", checked_hash.finalize()) != input_sha256
        || ranges
            .iter()
            .map(|range| range.document_lines)
            .sum::<usize>()
            != validated.document_lines
    {
        return Err(
            "Private Eingabesicherung entspricht nicht dem vollständig geprüften Byte-/Zeilenstand"
                .into(),
        );
    }
    let mut plans = Vec::with_capacity(ranges.len());
    for (index, ((range, counts), hash)) in ranges.iter().zip(partitions).zip(hashes).enumerate() {
        let path = output_dir.join(format!("part-{:06}.jsonl", index + 1));
        if path == report {
            return Err("Berichtspfad darf keine Partitionsdatei belegen".into());
        }
        plans.push(json!({
            "index": index + 1, "path": path, "range": range,
            "bytes": range.input_byte_end_exclusive - range.input_byte_start,
            "singleton": range.input_byte_end_exclusive - range.input_byte_start > PARTITION_MAX_BYTES,
            "sha256": format!("{:x}", hash.finalize()), "sources": proof(counts),
        }));
    }
    let sources = proof(sources);
    let mut manifest = json!({
        "mode": "partition", "partitioned": false, "complete": false,
        "imported": false, "published": false, "activated": false,
        "input": {
            "path": input_path, "bytes": input_bytes, "sha256": input_sha256,
            "document_lines": validated.document_lines,
            "distinct_documents": sources.values().map(|count| count.distinct_documents).sum::<usize>(),
            "distinct_versions": sources.values().map(|count| count.distinct_versions).sum::<usize>(),
            "source_count": sources.len(),
            "facts": sources.values().map(|count| count.facts).sum::<usize>(),
            "sources": sources, "duplicate_lines": validated.duplicate_lines.len(),
            "input_conflicts": 0, "unknown_revision_lines": unknown_revisions,
        },
        "limits": {
            "document_lines": PARTITION_MAX_DOCUMENT_LINES, "bytes": PARTITION_MAX_BYTES,
            "singleton_bytes": SINGLETON_MAX_BYTES, "input_bytes": INPUT_MAX_BYTES,
            "input_document_lines": INPUT_MAX_LINES, "index_bytes": INDEX_MAX_BYTES,
        },
        "validation_index_bytes": validated.index_bytes,
        "output_dir": output_dir, "output_dir_created_by_this_attempt": true,
        "planned_partitions": plans, "persisted_partitions": [], "report": report,
    });
    std::fs::create_dir(output_dir).map_err(|_| {
        "Neuer Ausgabeordner kann nicht angelegt werden; keine eigene Ausgabe bestätigt"
    })?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "Übergeordneter Ausgabeordner kann nicht dauerhaft gespeichert werden")?;
    for (index, range) in ranges.iter().enumerate() {
        let path = output_dir.join(format!("part-{:06}.jsonl", index + 1));
        if let Err(error) = write_range(&path, saved.as_file_mut(), range) {
            return Err(partition_failure(
                report,
                &mut manifest,
                format!("Partition {}: {error}", index + 1),
            ));
        }
        let value = manifest["planned_partitions"][index].clone();
        manifest["persisted_partitions"]
            .as_array_mut()
            .ok_or("Partitionsnachweis ist ungültig")?
            .push(value);
    }
    if File::open(output_dir)
        .and_then(|directory| directory.sync_all())
        .is_err()
    {
        return Err(partition_failure(
            report,
            &mut manifest,
            "Ausgabeordner kann nicht dauerhaft gespeichert werden".into(),
        ));
    }
    manifest["partitioned"] = json!(true);
    manifest["complete"] = json!(true);
    if let Err(error) = write_report(report, &manifest) {
        return Err(partition_failure(report, &mut manifest, error));
    }
    Ok(manifest)
}
