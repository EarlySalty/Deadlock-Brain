use std::{
    collections::{BTreeMap, HashSet},
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    path::{Component, Path},
    time::Instant,
};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_LINE_BYTES: usize = 256 * 1024 * 1024;

fn invalid(message: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}

fn text<'a>(value: &'a Value, key: &str) -> io::Result<&'a str> {
    value[key]
        .as_str()
        .filter(|text| !text.is_empty())
        .ok_or_else(|| invalid(format!("Fehlendes Textfeld: {key}")))
}

fn hex_hash(value: &Value, key: &str) -> io::Result<()> {
    let hash = text(value, key)?;
    if hash.len() != 64
        || !hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid(format!("Ungültiger SHA-256: {key}")));
    }
    Ok(())
}

fn sha_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let count = file.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        digest.update(&bytes[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn next_line(reader: &mut impl BufRead, line: &mut Vec<u8>) -> io::Result<bool> {
    line.clear();
    loop {
        let (count, finished, eof) = {
            let bytes = reader.fill_buf()?;
            if bytes.is_empty() {
                (0, false, true)
            } else {
                let count = bytes
                    .iter()
                    .position(|b| *b == b'\n')
                    .map_or(bytes.len(), |n| n + 1);
                if line
                    .len()
                    .checked_add(count)
                    .is_none_or(|n| n > MAX_LINE_BYTES)
                {
                    return Err(invalid(
                        "JSONL-Zeile überschreitet die Prüfgrenze von 256 MiB",
                    ));
                }
                line.extend_from_slice(&bytes[..count]);
                (count, bytes[count - 1] == b'\n', false)
            }
        };
        reader.consume(count);
        if eof {
            return Ok(!line.is_empty());
        }
        if finished {
            line.pop();
            return Ok(true);
        }
    }
}

fn decode(bytes: &[u8]) -> io::Result<String> {
    if bytes.starts_with(&[0xff, 0xfe]) || bytes.starts_with(&[0xfe, 0xff]) {
        if !(bytes.len() - 2).is_multiple_of(2) {
            return Err(invalid("Ungültiges UTF-16"));
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
        return String::from_utf16(&units).map_err(invalid);
    }
    String::from_utf8(bytes.to_vec()).map_err(invalid)
}

fn utc_timestamp(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
        || !bytes
            .iter()
            .enumerate()
            .all(|(i, b)| matches!(i, 4 | 7 | 10 | 13 | 16 | 19) || b.is_ascii_digit())
    {
        return false;
    }
    let part = |start: usize, end: usize| text[start..end].parse::<u32>().ok();
    let Some(year) = part(0, 4) else { return false };
    let Some(month) = part(5, 7) else {
        return false;
    };
    let Some(day) = part(8, 10) else { return false };
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || year % 4 == 0 && year % 100 != 0 => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
        && part(11, 13).is_some_and(|n| n <= 23)
        && part(14, 16).is_some_and(|n| n <= 59)
        && part(17, 19).is_some_and(|n| n <= 59)
}

fn decimal(text: &str) -> Option<(bool, String, i64)> {
    let (negative, text) = text.strip_prefix('-').map_or((false, text), |s| (true, s));
    let (base, exponent) = match text.split_once(['e', 'E']) {
        Some((base, exponent)) => (base, exponent.parse::<i64>().ok()?),
        None => (text, 0),
    };
    let (whole, fraction) = base.split_once('.').unwrap_or((base, ""));
    if whole.is_empty()
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let digits = format!("{whole}{fraction}");
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return Some((negative, "0".to_owned(), 0));
    }
    let trimmed = digits.trim_end_matches('0');
    let trailing = i64::try_from(digits.len() - trimmed.len()).ok()?;
    let fraction = i64::try_from(fraction.len()).ok()?;
    let exponent = exponent.checked_sub(fraction)?.checked_add(trailing)?;
    Some((negative, trimmed.to_owned(), exponent))
}

fn status(value: &Value, key: &str) -> io::Result<()> {
    if !matches!(
        text(value, key)?,
        "extracted_value" | "source_statement" | "hypothesis"
    ) {
        return Err(invalid(format!("Ungültiger Belegstatus: {key}")));
    }
    Ok(())
}

fn lexical_number_consistency(fact: &Value) -> io::Result<bool> {
    if fact["qualifiers"]["numeric_representation"] != "source_numeric_lexeme" {
        return Ok(false);
    }
    match (
        fact["value"].as_str(),
        fact["qualifiers"]["source_lexeme"].as_str(),
    ) {
        (Some(value), Some(lexeme)) if value == lexeme => Ok(true),
        _ => Err(invalid(
            "Numerischer Quelltext stimmt nicht mit dem Zeichenkettenwert überein",
        )),
    }
}

fn report_resources(started: Instant, success: bool) -> io::Result<()> {
    let reader = BufReader::new(File::open("/proc/self/status")?);
    let mut peak_kib = None;
    for line in reader.lines() {
        let line = line?;
        if let Some(value) = line.strip_prefix("VmHWM:") {
            let mut fields = value.split_whitespace();
            let peak = fields
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(|| io::Error::other("Ungültiger VmHWM-Wert"))?;
            if fields.next() != Some("kB") || fields.next().is_some() {
                return Err(io::Error::other("Ungültige VmHWM-Einheit"));
            }
            peak_kib = Some(peak);
            break;
        }
    }
    let peak_kib = peak_kib.ok_or_else(|| io::Error::other("VmHWM fehlt im Prozessstatus"))?;
    let mut stderr = io::stderr().lock();
    writeln!(
        stderr,
        "RESOURCE {}",
        json!({
            "duration_seconds": started.elapsed().as_secs_f64(),
            "peak_rss_kib": peak_kib,
            "pid": std::process::id(),
            "success": success,
            "scope": "linux_process_vmhwm"
        })
    )?;
    stderr.flush()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let result = run();
    let measurement = report_resources(started, result.is_ok());
    match (result, measurement) {
        (Err(error), Err(measurement)) => {
            Err(format!("{error}; Ressourcenmessung fehlgeschlagen: {measurement}").into())
        }
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error.into()),
        (Ok(()), Ok(())) => Ok(()),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("Aufruf: validate-jsonl OPTIONS.json DOCUMENTS.jsonl INVENTORY.json".into());
    }
    let options: Value = serde_json::from_reader(File::open(&args[1])?)?;
    let inventory: Value = serde_json::from_reader(File::open(&args[3])?)?;
    let root = fs::canonicalize(text(&options, "root")?)?;
    let observed = text(&options, "observed_at")?;
    if !utc_timestamp(observed) {
        return Err(invalid("Prüfharness erwartet eine gültige sekundengenaue UTC-Zeit").into());
    }
    let expected_revision = text(&options, "source_revision")?;
    let expected_source = text(&options, "source_id")?;
    let expected_app = options["app_id"]
        .as_u64()
        .ok_or_else(|| invalid("Fehlende App-ID"))?;
    let max_source_bytes = options["max_file_bytes"]
        .as_u64()
        .ok_or_else(|| invalid("Fehlende Eingabegrenze"))?;
    let mut reader = BufReader::new(File::open(&args[2])?);
    let mut line = Vec::new();
    let mut documents = 0u64;
    let mut facts_count = 0u64;
    let mut lexical_numbers_checked = 0u64;
    let mut largest_line = 0usize;
    let mut largest_content = 0usize;
    let mut ids = HashSet::new();
    let mut parsers = BTreeMap::<String, u64>::new();
    let mut categories = BTreeMap::<String, u64>::new();
    let mut substantive = BTreeMap::<String, u64>::new();
    while next_line(&mut reader, &mut line)? {
        if line.is_empty() {
            return Err(invalid("Leere JSONL-Zeile").into());
        }
        largest_line = largest_line.max(line.len());
        let document: Value = serde_json::from_slice(&line)?;
        if document["contract_version"] != "wiki-spielwissen-v1"
            || document["source_kind"] != "game_file"
            || document["source_id"] != expected_source
            || document["revision"] != expected_revision
            || document["observed_at"] != observed
        {
            return Err(invalid("Vertrags-/Quellen-/Versionsabweichung").into());
        }
        let id = text(&document, "document_id")?;
        if !ids.insert(id.to_owned()) {
            return Err(invalid(format!("Doppelte Dokument-ID: {id}")).into());
        }
        text(&document, "title")?;
        text(&document, "language")?;
        status(&document, "evidence_status")?;
        let content = document["content"]
            .as_str()
            .ok_or_else(|| invalid("Kein UTF-8-Inhalt"))?;
        largest_content = largest_content.max(content.len());
        hex_hash(&document, "content_sha256")?;
        if format!("{:x}", Sha256::digest(content.as_bytes())) != document["content_sha256"] {
            return Err(invalid(format!("Inhaltshash stimmt nicht: {id}")).into());
        }
        let license = &document["license"];
        text(license, "name")?;
        text(license, "attribution")?;
        if !license["url"].is_null() && !license["url"].is_string()
            || license["redistribution_allowed"] != false
        {
            return Err(invalid("Ungültige Lizenzfelder oder Veröffentlichungsfreigabe").into());
        }
        let metadata = &document["metadata"];
        if !metadata.is_object()
            || metadata["app_id"] != expected_app
            || !metadata["build_id"].is_null()
            || !metadata["manifest_id"].is_null()
            || !metadata["depot_id"].is_null()
            || metadata["source_revision"] != expected_revision
            || metadata["gameplay_execution_verified"] != false
            || !metadata["extraction"].is_object()
        {
            return Err(invalid(format!("Ungültige Spielherkunft: {id}")).into());
        }
        text(&metadata["extraction"], "method")?;
        text(&metadata["extraction"], "version")?;
        let relative = text(metadata, "relative_path")?;
        if id != format!("game:{expected_app}:{relative}") {
            return Err(invalid(format!(
                "ID ist nicht an kanonischen Dateipfad gebunden: {id}"
            ))
            .into());
        }
        let locator = text(&document, "source_locator")?;
        let locator_path = Path::new(locator);
        if !locator_path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
        {
            return Err(invalid("Locator ist kein sicherer relativer Quellpfad").into());
        }
        let original_path = fs::canonicalize(root.join(locator_path))?;
        if !original_path.starts_with(&root) || !original_path.is_file() {
            return Err(invalid("Originalpfad verlässt den Quellenroot").into());
        }
        if fs::metadata(&original_path)?.len() > max_source_bytes {
            return Err(
                invalid("Originaldatei überschreitet die konfigurierte Eingabegrenze").into(),
            );
        }
        hex_hash(metadata, "original_sha256")?;
        if sha_file(&original_path)? != metadata["original_sha256"] {
            return Err(invalid(format!("Originalhash stimmt nicht: {id}")).into());
        }
        if decode(&fs::read(&original_path)?)? != content {
            return Err(invalid(format!(
                "Quelldokument wurde verändert oder abgeschnitten: {id}"
            ))
            .into());
        }
        let parser = text(metadata, "parse_status")?;
        *parsers.entry(parser.to_owned()).or_default() += 1;
        let category = text(metadata, "category")?;
        *categories.entry(category.to_owned()).or_default() += 1;
        let facts = document["facts"]
            .as_array()
            .ok_or_else(|| invalid("Fakten sind keine Liste"))?;
        let mut fact_ids = HashSet::new();
        for fact in facts {
            let fact_id = text(fact, "fact_id")?;
            if !fact_ids.insert(fact_id) {
                return Err(invalid(format!("Doppelte Fakten-ID in {id}: {fact_id}")).into());
            }
            text(fact, "subject")?;
            text(fact, "predicate")?;
            status(fact, "evidence_status")?;
            if fact.get("value").is_none()
                || !fact
                    .get("unit")
                    .is_some_and(|v| v.is_null() || v.is_string())
                || !fact
                    .get("source_span")
                    .is_some_and(|v| v.is_null() || v.is_string())
                || !fact["qualifiers"].is_object()
            {
                return Err(invalid(format!("Unvollständiger Fakt: {id}:{fact_id}")).into());
            }
            lexical_numbers_checked += u64::from(
                lexical_number_consistency(fact)
                    .map_err(|error| invalid(format!("{id}:{fact_id}: {error}")))?,
            );
            if let (Some(number), Some(lexeme)) = (
                fact["value"].as_number(),
                fact["qualifiers"]["source_lexeme"].as_str(),
            ) {
                if decimal(lexeme).is_some() && decimal(lexeme) != decimal(&number.to_string()) {
                    return Err(invalid(format!("Gerundeter Zahlenwert: {id}:{fact_id}")).into());
                }
            }
        }
        if locator.ends_with("abilities.vdata")
            || locator.ends_with("heroes.vdata")
            || locator.ends_with("generic_data.vdata")
            || locator.ends_with("hero-data.json")
            || locator.ends_with("ability-data.json")
            || locator.ends_with("npc-data.json")
        {
            substantive.insert(locator.to_owned(), facts.len() as u64);
        }
        facts_count += facts.len() as u64;
        documents += 1;
    }
    let extracted = inventory["files"]
        .as_array()
        .ok_or_else(|| invalid("Inventar ist keine Dateiliste"))?
        .iter()
        .filter(|entry| entry["disposition"] == "extracted")
        .count() as u64;
    if inventory["documents"] != documents
        || inventory["facts"] != facts_count
        || extracted != documents
    {
        return Err(invalid(
            "Inventarzahlen stimmen nicht mit dem vollständigen JSONL-Lauf überein",
        )
        .into());
    }
    let mut stdout = io::stdout().lock();
    writeln!(
        stdout,
        "{}",
        serde_json::to_string_pretty(&json!({
            "source_id": expected_source,
            "revision": expected_revision,
            "documents": documents,
            "facts": facts_count,
            "source_numeric_lexemes_checked": lexical_numbers_checked,
            "inventoried_files": inventory["files"].as_array().map(Vec::len),
            "unknown_revisions": inventory["unknown_revisions"],
            "gaps": inventory["gaps"],
            "max_jsonl_line_bytes": largest_line,
            "max_content_utf8_bytes": largest_content,
            "source_and_content_hashes_verified": true,
            "complete_source_text_verified": true,
            "duplicate_document_ids": 0,
            "duplicate_fact_ids": 0,
            "parse_statuses": parsers,
            "categories": categories,
            "substantive_file_facts": substantive
        }))?
    )?;
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn decimal_comparison_does_not_round() {
        assert_eq!(decimal("1.2500e2"), decimal("125"));
        assert_ne!(decimal("0.100000000000000005"), decimal("0.1"));
        assert_eq!(
            decimal("18446744073709551616"),
            decimal("184467440737095516160e-1")
        );
    }

    #[test]
    fn marked_lexical_numbers_require_exact_string_consistency() {
        for lexeme in [
            "340282346638528859811704183484516925440.0",
            "18446744073709551616",
            "0.12345678901234567890123456789",
            "1e999",
            "-0",
        ] {
            let mut fact = json!({
                "value": lexeme,
                "qualifiers": {
                    "source_lexeme": lexeme,
                    "numeric_representation": "source_numeric_lexeme"
                }
            });
            assert!(lexical_number_consistency(&fact).unwrap());
            fact["value"] = json!("rounded");
            assert!(lexical_number_consistency(&fact).is_err());
            for wrong in [json!(12), Value::Null, json!(false), json!([])] {
                fact["value"] = wrong;
                assert!(lexical_number_consistency(&fact).is_err());
            }
            fact["value"] = json!(lexeme);
            fact["qualifiers"]
                .as_object_mut()
                .unwrap()
                .remove("source_lexeme");
            assert!(lexical_number_consistency(&fact).is_err());
            fact["qualifiers"]["source_lexeme"] = json!(12);
            assert!(lexical_number_consistency(&fact).is_err());
        }
        let ordinary = json!({"value": "source string", "qualifiers": {}});
        assert!(!lexical_number_consistency(&ordinary).unwrap());
        let integer = json!({"value": 12, "qualifiers": {"source_lexeme": "12", "numeric_representation": "exact_integer"}});
        assert!(!lexical_number_consistency(&integer).unwrap());
    }

    #[test]
    fn source_decoding_is_strict() {
        assert_eq!(decode(&[255, 254, 65, 0]).unwrap(), "A");
        assert!(decode(&[255, 254, 65]).is_err());
        assert!(decode(&[255, 254, 0, 216]).is_err());
    }

    #[test]
    fn utc_dates_are_checked_against_calendar() {
        assert!(utc_timestamp("2026-10-03T03:37:23Z"));
        assert!(!utc_timestamp("2026-02-29T03:37:23Z"));
        assert!(utc_timestamp("2024-02-29T03:37:23Z"));
        assert!(!utc_timestamp("2026-10-03T24:37:23Z"));
    }

    #[test]
    fn final_line_does_not_need_newline() {
        let mut reader = Cursor::new(b"first\nlast");
        let mut bytes = Vec::new();
        assert!(next_line(&mut reader, &mut bytes).unwrap());
        assert_eq!(bytes, b"first");
        assert!(next_line(&mut reader, &mut bytes).unwrap());
        assert_eq!(bytes, b"last");
        assert!(!next_line(&mut reader, &mut bytes).unwrap());
    }
}
