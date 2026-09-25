#![forbid(unsafe_code)]

//! Offline inventory checks only. This is neither a replay decoder nor a Brain
//! contract, rights authority, file-hash verifier, or implementation/release gate.
//! Reports deliberately contain no caller-controlled cell values or raw locators.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const MAX_MANIFEST_BYTES: usize = 262_144;
pub const MIN_GOLDEN_MATCHES: usize = 10;
pub const CORPUS_HEADER: &str = "case_id\tprovenance\tmatch_key\tpatch\tmode\tsha256\tparser_revision\tschema_revision\trights\tstorage\tfile_status\tsplit\tplayed_at_epoch\tavailable_at_epoch\treference_status";
pub const CAPABILITY_HEADER: &str =
    "case_id\tfield\tstatus\tunit\ttime_basis\treference_key\tderivation_key";
pub const REQUIRED_FIELDS: [&str; 9] = [
    "match_id",
    "patch_mode",
    "teams_heroes",
    "duration_timeline",
    "items",
    "ability_levels",
    "kda",
    "positions",
    "objectives",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditError {
    pub table: &'static str,
    pub line: usize,
    pub code: &'static str,
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.table, self.line, self.code)
    }
}
impl std::error::Error for AuditError {}

fn err(table: &'static str, line: usize, code: &'static str) -> AuditError {
    AuditError { table, line, code }
}

fn table<'a>(
    raw: &'a str,
    header: &str,
    name: &'static str,
) -> Result<Vec<Vec<&'a str>>, AuditError> {
    if raw.len() > MAX_MANIFEST_BYTES {
        return Err(err(name, 0, "manifest_too_large"));
    }
    let mut lines = raw.lines();
    if lines.next() != Some(header) {
        return Err(err(name, 1, "invalid_header"));
    }
    let columns = header.split('\t').count();
    let mut result = Vec::new();
    for (index, line) in lines.enumerate() {
        let row: Vec<_> = line.split('\t').collect();
        if row.len() != columns || row.iter().any(|value| value.is_empty()) {
            return Err(err(name, index + 2, "invalid_columns"));
        }
        if row.iter().any(|value| !token(value)) {
            return Err(err(name, index + 2, "invalid_token"));
        }
        result.push(row);
    }
    Ok(result)
}

// Opaque identifiers only, not URLs, local paths, emails, player names or prose.
fn token(value: &str) -> bool {
    value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-.:".contains(&b))
}
fn hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn unknown_or_hex(value: &str, len: usize) -> bool {
    value == "-" || hex(value, len)
}
fn epoch(value: &str, line: usize) -> Result<Option<u64>, AuditError> {
    if value == "-" {
        return Ok(None);
    }
    if !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(err("corpus", line, "invalid_epoch"));
    }
    value
        .parse::<u64>()
        .ok()
        .filter(|v| *v <= 253_402_300_799)
        .map(Some)
        .ok_or_else(|| err("corpus", line, "invalid_epoch"))
}
fn choice(
    value: &str,
    allowed: &[&str],
    table: &'static str,
    line: usize,
) -> Result<(), AuditError> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(err(table, line, "invalid_enum"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub manifest_rows: usize,
    pub real_matches: usize,
    pub complete_matches: usize,
    pub synthetic_cases: usize,
    pub repeated_match_rows: usize,
    pub missing_capabilities: usize,
    pub blockers: BTreeSet<&'static str>,
}
impl Report {
    pub fn metadata_complete(&self) -> bool {
        self.blockers.is_empty()
    }
    pub fn json(&self) -> String {
        let blockers = self
            .blockers
            .iter()
            .map(|v| format!("\"{v}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"audit_version\":\"s14.inventory.v1\",\"status\":\"{}\",\"implementation_authorized\":false,\"integration_verified\":false,\"manifest_rows\":{},\"real_matches\":{},\"complete_matches\":{},\"synthetic_cases\":{},\"repeated_match_rows\":{},\"missing_capabilities\":{},\"blockers\":[{}]}}",
            if self.metadata_complete() { "metadata_complete" } else { "blocked" },
            self.manifest_rows, self.real_matches, self.complete_matches,
            self.synthetic_cases, self.repeated_match_rows, self.missing_capabilities, blockers
        )
    }
}

/// Validate self-reported metadata; never opens replays or verifies the truth of
/// approvals/reference results. Even a complete manifest cannot authorize work.
/// Cutoff must be fixed by S10 before corpus selection, not chosen after tuning.
pub fn audit(corpus: &str, capabilities: &str, cutoff: Option<u64>) -> Result<Report, AuditError> {
    if cutoff.is_some_and(|v| v > 253_402_300_799) {
        return Err(err("config", 0, "invalid_cutoff"));
    }
    let rows = table(corpus, CORPUS_HEADER, "corpus")?;
    let caps = table(capabilities, CAPABILITY_HEADER, "capabilities")?;
    let mut report = Report {
        manifest_rows: rows.len(),
        real_matches: 0,
        complete_matches: 0,
        synthetic_cases: 0,
        repeated_match_rows: 0,
        missing_capabilities: 0,
        blockers: BTreeSet::new(),
    };
    if rows.is_empty() {
        report.blockers.insert("corpus_empty");
    }
    if cutoff.is_none() {
        report.blockers.insert("holdout_cutoff_missing");
    }
    let mut ids = BTreeMap::new();
    let mut matches: BTreeMap<&str, (&str, &str, &str)> = BTreeMap::new();
    let mut hashes = BTreeMap::new();
    let mut ready_cases = BTreeSet::new();
    let mut development = false;
    let mut holdout = false;
    for (index, row) in rows.iter().enumerate() {
        let line = index + 2;
        if row[0] == "-" || ids.insert(row[0], index).is_some() {
            return Err(err("corpus", line, "invalid_or_duplicate_case_id"));
        }
        choice(row[1], &["real", "synthetic"], "corpus", line)?;
        choice(row[8], &["approved", "unknown", "denied"], "corpus", line)?;
        choice(row[9], &["restricted", "public", "unknown"], "corpus", line)?;
        choice(
            row[10],
            &["verified", "unverified", "missing", "corrupt"],
            "corpus",
            line,
        )?;
        choice(
            row[11],
            &["development", "holdout", "unassigned"],
            "corpus",
            line,
        )?;
        choice(
            row[14],
            &["independent", "correlated", "missing", "unverified"],
            "corpus",
            line,
        )?;
        if !unknown_or_hex(row[5], 64) || !unknown_or_hex(row[6], 40) || !unknown_or_hex(row[7], 40)
        {
            return Err(err("corpus", line, "invalid_hash_or_revision"));
        }
        let played = epoch(row[12], line)?;
        let available = epoch(row[13], line)?;
        if row[1] == "synthetic" {
            report.synthetic_cases += 1;
            // Synthetic reference records are valid test design, never real evidence.
            continue;
        }
        let mut ready = true;
        let mut require = |ok: bool, code| {
            if !ok {
                ready = false;
                report.blockers.insert(code);
            }
        };
        require(row[8] == "approved", "rights_not_approved");
        // Public exception requires a separately approved future policy change.
        require(row[9] == "restricted", "raw_storage_not_restricted");
        require(row[10] == "verified", "raw_file_not_verified");
        require(
            row[2..8].iter().all(|v| *v != "-"),
            "identity_patch_or_revision_unknown",
        );
        require(row[14] == "independent", "independent_reference_missing");
        require(
            played.is_some() && available.is_some(),
            "time_provenance_missing",
        );
        require(
            !matches!((played, available), (Some(p), Some(a)) if a < p),
            "invalid_availability_order",
        );
        require(row[11] != "unassigned", "split_unassigned");
        match (row[11], cutoff) {
            ("development", Some(c)) => require(
                available.is_some_and(|a| a <= c),
                "development_future_leakage",
            ),
            ("holdout", Some(c)) => require(played.is_some_and(|p| p > c), "holdout_not_future"),
            _ => {}
        }
        if row[2] != "-" {
            if let Some((patch, mode, split)) = matches.get_mut(row[2]) {
                report.repeated_match_rows += 1;
                if (*patch != row[3] && *patch != "-" && row[3] != "-")
                    || (*mode != row[4] && *mode != "-" && row[4] != "-")
                {
                    return Err(err("corpus", line, "match_identity_conflict"));
                }
                if *patch == "-" {
                    *patch = row[3];
                }
                if *mode == "-" {
                    *mode = row[4];
                }
                if *split != row[11] {
                    report.blockers.insert("match_split_leakage");
                    ready = false;
                }
            } else {
                matches.insert(row[2], (row[3], row[4], row[11]));
            }
        }
        if row[5] != "-" && row[2] != "-" {
            if let Some(old_match) = hashes.insert(row[5], row[2]) {
                if old_match != row[2] {
                    return Err(err("corpus", line, "raw_hash_identity_conflict"));
                }
            }
        }
        if ready {
            ready_cases.insert(row[0]);
        }
    }
    let mut coverage = BTreeMap::new();
    for (index, row) in caps.iter().enumerate() {
        let line = index + 2;
        if !ids.contains_key(row[0]) {
            return Err(err("capabilities", line, "unknown_case"));
        }
        if !REQUIRED_FIELDS.contains(&row[1]) {
            return Err(err("capabilities", line, "unknown_field"));
        }
        choice(
            row[2],
            &["supported", "derived", "unavailable", "unverified"],
            "capabilities",
            line,
        )?;
        choice(
            row[4],
            &["verified", "unknown", "not_applicable"],
            "capabilities",
            line,
        )?;
        let temporal = !["match_id", "patch_mode", "teams_heroes"].contains(&row[1]);
        let complete = ["supported", "derived"].contains(&row[2])
            && !["-", "unknown"].contains(&row[3])
            && row[5] != "-"
            && (row[2] != "derived" || row[6] != "-")
            && (row[4] == "verified" || (!temporal && row[4] == "not_applicable"));
        if coverage.insert((row[0], row[1]), complete).is_some() {
            return Err(err("capabilities", line, "duplicate_capability"));
        }
    }
    let mut complete_matches = BTreeSet::new();
    for row in &rows {
        if row[1] != "real" {
            continue;
        }
        let mut all = true;
        for field in REQUIRED_FIELDS {
            if coverage.get(&(row[0], field)) != Some(&true) {
                report.missing_capabilities += 1;
                all = false;
            }
        }
        if all && ready_cases.contains(row[0]) {
            complete_matches.insert(row[2]);
            development |= row[11] == "development";
            holdout |= row[11] == "holdout";
        }
    }
    report.real_matches = matches.len();
    report.complete_matches = complete_matches.len();
    if report.missing_capabilities > 0 {
        report.blockers.insert("mandatory_capabilities_unproven");
    }
    if report.complete_matches < MIN_GOLDEN_MATCHES {
        report.blockers.insert("golden_corpus_below_minimum");
    }
    if !development || !holdout {
        report.blockers.insert("both_holdout_partitions_required");
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_matches_are_not_reported_as_duplicates() {
        let (mut c, p) = fixture();
        for i in 0..10 {
            c = c.replace(&format!("match-{i}"), "-");
        }
        let r = audit(&c, &p, Some(1000)).unwrap();
        assert_eq!(r.real_matches, 0);
        assert_eq!(r.repeated_match_rows, 0);
        assert!(!r.metadata_complete());
    }

    #[test]
    fn initially_unknown_patch_cannot_hide_later_conflict() {
        let (c, p) = fixture();
        let mut c = c.replacen("patch-a", "-", 1);
        c.push_str(&corpus_row(0).replace("case-0", "second-feed"));
        c.push_str(
            &corpus_row(0)
                .replace("case-0", "third-feed")
                .replace("patch-a", "patch-b"),
        );
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap_err().code,
            "match_identity_conflict"
        );
    }

    #[test]
    fn partial_parser_versions_cannot_be_combined_into_complete_match() {
        let (mut c, p) = fixture();
        c.push_str(
            &corpus_row(0)
                .replace("case-0", "new-parser")
                .replace(&"a".repeat(40), &"c".repeat(40)),
        );
        let mut p = p
            .lines()
            .filter(|line| !line.starts_with("case-0\titems\t"))
            .collect::<Vec<_>>()
            .join("\n");
        p.push_str("\nnew-parser\titems\tsupported\titem-id\tverified\tref\t-\n");
        let r = audit(&c, &p, Some(1000)).unwrap();
        assert_eq!(r.complete_matches, 9);
        assert!(!r.metadata_complete());
    }

    fn corpus_row(i: usize) -> String {
        let split = if i < 5 { "development" } else { "holdout" };
        let epoch = if i < 5 { 100 } else { 2000 };
        format!("case-{i}\treal\tmatch-{i}\tpatch-a\tnormal\t{i:064x}\t{}\t{}\tapproved\trestricted\tverified\t{split}\t{epoch}\t{epoch}\tindependent\n", "a".repeat(40), "b".repeat(40))
    }
    fn fixture() -> (String, String) {
        let mut corpus = format!("{CORPUS_HEADER}\n");
        let mut caps = format!("{CAPABILITY_HEADER}\n");
        for i in 0..10 {
            corpus.push_str(&corpus_row(i));
            for field in REQUIRED_FIELDS {
                caps.push_str(&format!(
                    "case-{i}\t{field}\tsupported\tdeclared-unit\tverified\tcurated-ref\t-\n"
                ));
            }
        }
        (corpus, caps)
    }
    fn blocked(c: &str, p: &str, code: &str) {
        assert!(
            audit(c, p, Some(1000)).unwrap().blockers.contains(code),
            "expected {code}"
        );
    }
    #[test]
    fn complete_metadata_never_authorizes_integration() {
        let (c, p) = fixture();
        let r = audit(&c, &p, Some(1000)).unwrap();
        assert!(r.metadata_complete());
        assert_eq!(r.complete_matches, 10);
        assert!(r.json().contains("\"implementation_authorized\":false"));
        assert!(r.json().contains("\"integration_verified\":false"));
    }
    #[test]
    fn empty_corpus_is_blocked_not_vacuously_green() {
        blocked(CORPUS_HEADER, CAPABILITY_HEADER, "corpus_empty");
    }
    #[test]
    fn cutoff_is_required() {
        let (c, p) = fixture();
        assert!(audit(&c, &p, None)
            .unwrap()
            .blockers
            .contains("holdout_cutoff_missing"));
    }
    #[test]
    fn synthetic_never_counts_as_real() {
        let (c, p) = fixture();
        let r = audit(&c.replace("\treal\t", "\tsynthetic\t"), &p, Some(1000)).unwrap();
        assert_eq!(r.real_matches, 0);
        assert_eq!(r.complete_matches, 0);
        assert_eq!(r.synthetic_cases, 10);
        assert!(!r.metadata_complete());
    }
    #[test]
    fn unknown_rights_block() {
        let (c, p) = fixture();
        blocked(&c.replace("approved", "unknown"), &p, "rights_not_approved");
    }
    #[test]
    fn denied_rights_block() {
        let (c, p) = fixture();
        blocked(&c.replace("approved", "denied"), &p, "rights_not_approved");
    }
    #[test]
    fn public_raw_storage_blocks() {
        let (c, p) = fixture();
        blocked(
            &c.replace("restricted", "public"),
            &p,
            "raw_storage_not_restricted",
        );
    }
    #[test]
    fn corrupted_files_block() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\tverified\t", "\tcorrupt\t"),
            &p,
            "raw_file_not_verified",
        );
    }
    #[test]
    fn missing_files_block() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\tverified\t", "\tmissing\t"),
            &p,
            "raw_file_not_verified",
        );
    }
    #[test]
    fn missing_patch_is_not_guessed() {
        let (c, p) = fixture();
        blocked(
            &c.replace("patch-a", "-"),
            &p,
            "identity_patch_or_revision_unknown",
        );
    }
    #[test]
    fn missing_parser_pin_blocks() {
        let (c, p) = fixture();
        blocked(
            &c.replace(&"a".repeat(40), "-"),
            &p,
            "identity_patch_or_revision_unknown",
        );
    }
    #[test]
    fn correlated_reference_is_not_independent() {
        let (c, p) = fixture();
        blocked(
            &c.replace("independent", "correlated"),
            &p,
            "independent_reference_missing",
        );
    }
    #[test]
    fn absent_capabilities_block() {
        let (c, _) = fixture();
        blocked(&c, CAPABILITY_HEADER, "mandatory_capabilities_unproven");
    }
    #[test]
    fn unverified_is_not_supported() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("supported", "unverified"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn unavailable_is_not_zero() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("supported", "unavailable"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn derived_requires_algorithm_reference() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("supported", "derived"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn derived_with_provenance_is_metadata_complete() {
        let (c, p) = fixture();
        assert!(audit(
            &c,
            &p.replace("supported", "derived")
                .replace("\t-\n", "\talgorithm-v1\n"),
            Some(1000)
        )
        .unwrap()
        .metadata_complete());
    }
    #[test]
    fn reference_locator_is_required() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("curated-ref", "-"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn unknown_units_are_not_guessed() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("declared-unit", "unknown"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn timeline_requires_time_basis() {
        let (c, p) = fixture();
        blocked(
            &c,
            &p.replace("\tverified\t", "\tnot_applicable\t"),
            "mandatory_capabilities_unproven",
        );
    }
    #[test]
    fn duplicate_matches_do_not_expand_population() {
        let (mut c, mut p) = fixture();
        c.push_str(&corpus_row(0).replace("case-0", "second-feed"));
        let extra = p
            .lines()
            .filter(|l| l.starts_with("case-0\t"))
            .map(|l| format!("{}\n", l.replace("case-0", "second-feed")))
            .collect::<String>();
        p.push_str(&extra);
        let r = audit(&c, &p, Some(1000)).unwrap();
        assert_eq!(r.real_matches, 10);
        assert_eq!(r.complete_matches, 10);
        assert_eq!(r.repeated_match_rows, 1);
    }
    #[test]
    fn parser_upgrade_does_not_create_new_match() {
        let (c, p) = fixture();
        let c = c
            .replace("match-1", "match-0")
            .replace(&"a".repeat(40), &"c".repeat(40));
        let r = audit(&c, &p, Some(1000)).unwrap();
        assert_eq!(r.real_matches, 9);
        assert!(!r.metadata_complete());
    }
    #[test]
    fn match_cannot_cross_holdout_boundary() {
        let (c, p) = fixture();
        blocked(&c.replace("match-5", "match-0"), &p, "match_split_leakage");
    }
    #[test]
    fn knowledge_available_after_cutoff_is_leakage() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\t100\t100\t", "\t100\t2001\t"),
            &p,
            "development_future_leakage",
        );
    }
    #[test]
    fn holdout_must_be_future_match() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\t2000\t2000\t", "\t100\t100\t"),
            &p,
            "holdout_not_future",
        );
    }
    #[test]
    fn availability_cannot_precede_match() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\t100\t100\t", "\t100\t99\t"),
            &p,
            "invalid_availability_order",
        );
    }
    #[test]
    fn unknown_time_is_not_epoch_zero() {
        let (c, p) = fixture();
        blocked(
            &c.replace("\t100\t100\t", "\t-\t-\t"),
            &p,
            "time_provenance_missing",
        );
    }
    #[test]
    fn raw_hash_conflicts_rejected() {
        let (c, p) = fixture();
        let c = c.replace(&format!("{:064x}", 1), &format!("{:064x}", 0));
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap_err().code,
            "raw_hash_identity_conflict"
        );
    }
    #[test]
    fn conflicting_patch_rejected() {
        let (mut c, p) = fixture();
        c.push_str(
            &corpus_row(0)
                .replace("case-0", "other")
                .replace("patch-a", "patch-b"),
        );
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap_err().code,
            "match_identity_conflict"
        );
    }
    #[test]
    fn duplicate_case_rejected() {
        let (mut c, p) = fixture();
        c.push_str(&corpus_row(0));
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap_err().code,
            "invalid_or_duplicate_case_id"
        );
    }
    #[test]
    fn duplicate_capability_rejected() {
        let (c, mut p) = fixture();
        p.push_str("case-0\titems\tsupported\titem-id\tverified\tref\t-\n");
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap_err().code,
            "duplicate_capability"
        );
    }
    #[test]
    fn unknown_case_rejected() {
        let (c, p) = fixture();
        assert_eq!(
            audit(&c, &p.replace("case-0", "absent"), Some(1000))
                .unwrap_err()
                .code,
            "unknown_case"
        );
    }
    #[test]
    fn unknown_field_rejected() {
        let (c, p) = fixture();
        assert_eq!(
            audit(
                &c,
                &p.replace("\titems\t", "\tinvented-event\t"),
                Some(1000)
            )
            .unwrap_err()
            .code,
            "unknown_field"
        );
    }
    #[test]
    fn invalid_enum_rejected() {
        let (c, p) = fixture();
        assert_eq!(
            audit(&c.replace("approved", "APPROVED"), &p, Some(1000))
                .unwrap_err()
                .code,
            "invalid_enum"
        );
    }
    #[test]
    fn invalid_hash_rejected() {
        let (c, p) = fixture();
        assert_eq!(
            audit(&c.replace(&"a".repeat(40), "main"), &p, Some(1000))
                .unwrap_err()
                .code,
            "invalid_hash_or_revision"
        );
    }
    #[test]
    fn numeric_overflow_rejected() {
        let (c, p) = fixture();
        assert_eq!(
            audit(
                &c.replace("\t100\t", "\t99999999999999999999999\t"),
                &p,
                Some(1000)
            )
            .unwrap_err()
            .code,
            "invalid_epoch"
        );
    }
    #[test]
    fn wrong_header_rejected() {
        assert_eq!(
            audit("case_id", CAPABILITY_HEADER, None).unwrap_err().code,
            "invalid_header"
        );
    }
    #[test]
    fn malformed_columns_rejected() {
        assert_eq!(
            audit(&format!("{CORPUS_HEADER}\nbad\n"), CAPABILITY_HEADER, None)
                .unwrap_err()
                .code,
            "invalid_columns"
        );
    }
    #[test]
    fn oversized_manifest_rejected() {
        assert_eq!(
            audit(&"x".repeat(MAX_MANIFEST_BYTES + 1), CAPABILITY_HEADER, None)
                .unwrap_err()
                .code,
            "manifest_too_large"
        );
    }
    #[test]
    fn errors_do_not_echo_secret_values() {
        let (c, p) = fixture();
        let e = audit(
            &c.replace("case-0", "https://host/?secret=sentinel"),
            &p,
            None,
        )
        .unwrap_err();
        assert!(!e.to_string().contains("sentinel"));
    }
    #[test]
    fn report_contains_no_match_ids_or_hashes() {
        let (c, p) = fixture();
        let out = audit(&c, &p, Some(1000)).unwrap().json();
        assert!(!out.contains("match-0"));
        assert!(!out.contains(&"a".repeat(40)));
    }
    #[test]
    fn deterministic_report_under_row_reordering() {
        let (c, p) = fixture();
        let reversed = |s: &str| {
            let mut lines = s.lines();
            let header = lines.next().unwrap();
            format!(
                "{header}\n{}\n",
                lines
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };
        assert_eq!(
            audit(&c, &p, Some(1000)).unwrap(),
            audit(&reversed(&c), &reversed(&p), Some(1000)).unwrap()
        );
    }
    #[test]
    fn crlf_is_supported() {
        let (c, p) = fixture();
        assert!(audit(
            &c.replace('\n', "\r\n"),
            &p.replace('\n', "\r\n"),
            Some(1000)
        )
        .unwrap()
        .metadata_complete());
    }
}
