use super::{verified_capture, write_new, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::process::Command;

fn owner() -> Result<u32> {
    let output = Command::new("id")
        .arg("-u")
        .output()
        .map_err(|_| "owner_unavailable")?;
    if !output.status.success() {
        return Err("owner_unavailable");
    }
    std::str::from_utf8(&output.stdout)
        .map_err(|_| "owner_invalid")?
        .trim()
        .parse()
        .map_err(|_| "owner_invalid")
}

fn restricted(path: &Path, directory: bool, uid: u32) -> Result<()> {
    if !path.is_absolute()
        || fs::canonicalize(path).map_err(|_| "private_path_unavailable")? != path
    {
        return Err("private_path_not_canonical");
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| "private_path_unavailable")?;
    let mode = if directory { 0o700 } else { 0o600 };
    if metadata.uid() != uid
        || metadata.permissions().mode() & 0o777 != mode
        || if directory {
            !metadata.is_dir()
        } else {
            !metadata.is_file()
        }
    {
        return Err("private_path_not_restricted");
    }
    Ok(())
}

fn capture(root: &Path, source: &str, uid: u32) -> Result<(Value, String)> {
    restricted(&root.join(format!("{source}.json")), false, uid)?;
    restricted(&root.join(format!("{source}.sha256")), false, uid)?;
    verified_capture(root, source)
}

fn normalized(text: &str) -> String {
    let mut result = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("<@") {
        result.push_str(&rest[..start]);
        let mention = &rest[start + 2..];
        if let Some(end) = mention.find('>') {
            let id = mention[..end].trim_start_matches('!');
            if !id.is_empty() && id.bytes().all(|c| c.is_ascii_digit()) {
                result.push(' ');
                rest = &mention[end + 1..];
                continue;
            }
        }
        result.push_str("<@");
        rest = mention;
    }
    result.push_str(rest);
    result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn candidates(root: &Path) -> Result<(Value, Vec<Value>, usize)> {
    let uid = owner()?;
    restricted(root, true, uid)?;
    let (plan, _) = capture(root, "partial-plan-v1", uid)?;
    if plan["schema"] != "brain.q.partial-plan.v1" {
        return Err("partial_plan_schema_invalid");
    }
    let hashes = plan["source_hashes"]
        .as_object()
        .ok_or("plan_sources_missing")?;
    let mut sources = std::collections::BTreeMap::new();
    for (source, expected) in hashes {
        if !["botlogs", "dm", "twitch", "twitch-brain"].contains(&source.as_str()) {
            return Err("plan_source_invalid");
        }
        let (data, actual) = capture(root, source, uid)?;
        if expected.as_str() != Some(actual.as_str()) {
            return Err("plan_source_digest_mismatch");
        }
        sources.insert(source.clone(), data);
    }
    let mut output = Vec::new();
    let mut texts = BTreeSet::new();
    let mut duplicates = 0;
    let mut ids = BTreeSet::new();
    for case in plan["cases"].as_array().ok_or("plan_cases_missing")? {
        let id = case["case_id"].as_str().ok_or("case_id_missing")?;
        if !ids.insert(id) {
            return Err("case_id_duplicate");
        }
        let source = case["source"].as_str().ok_or("case_source_missing")?;
        let row = case["source_row"].as_u64().ok_or("case_row_invalid")?;
        let row = usize::try_from(row).map_err(|_| "case_row_invalid")?;
        let data = sources.get(source).ok_or("case_source_not_bound")?;
        if case["source_sha256"] != hashes[source] {
            return Err("case_source_digest_mismatch");
        }
        let original = data["original_rows"]
            .as_array()
            .and_then(|rows| rows.get(row))
            .and_then(|record| record["original"].as_str())
            .ok_or("case_original_missing")?;
        let duplicate = !texts.insert(normalized(original));
        duplicates += usize::from(duplicate);
        output.push(json!({
            "case_id":id, "source":source, "source_row":row,
            "source_sha256":hashes[source], "original":original,
            "duplicate_candidate":duplicate,
            "provisional_response_kind":case["expected_response_kind"],
            "authenticity":"pending", "privacy":"pending", "accepted_gold":false,
            "provider_question":null, "expected_response_kind":null,
            "original_facts":[], "original_fact_evidence":[],
            "transport":"pending", "review_evidence_sha256":null
        }));
    }
    Ok((
        json!({"source_hashes":hashes,"source_version":plan["version"]}),
        output,
        duplicates,
    ))
}

fn inventory(
    root: &Path,
    uid: u32,
) -> Result<(std::collections::BTreeMap<String, String>, usize, usize)> {
    restricted(root, true, uid)?;
    let mut files = std::collections::BTreeMap::new();
    let mut directories = vec![root.to_path_buf()];
    let mut directory_count = 0;
    let mut bindings = 0;
    while let Some(directory) = directories.pop() {
        restricted(&directory, true, uid)?;
        directory_count += 1;
        for entry in fs::read_dir(&directory).map_err(|_| "private_inventory_failed")? {
            let path = entry.map_err(|_| "private_inventory_failed")?.path();
            let metadata = fs::symlink_metadata(&path).map_err(|_| "private_inventory_failed")?;
            if metadata.is_dir() {
                directories.push(path);
            } else {
                restricted(&path, false, uid)?;
                let bytes = fs::read(&path).map_err(|_| "private_inventory_failed")?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| "private_inventory_failed")?
                    .to_str()
                    .ok_or("private_inventory_failed")?
                    .to_owned();
                files.insert(relative, format!("{:x}", Sha256::digest(&bytes)));
                if path
                    .extension()
                    .is_some_and(|extension| extension == "sha256")
                {
                    let data = fs::read(path.with_extension("json"))
                        .map_err(|_| "private_binding_failed")?;
                    let expected =
                        std::str::from_utf8(&bytes).map_err(|_| "private_binding_failed")?;
                    if expected != format!("{:x}", Sha256::digest(data)) {
                        return Err("private_binding_failed");
                    }
                    bindings += 1;
                }
            }
        }
    }
    Ok((files, directory_count, bindings))
}

pub(super) fn compare(left: &Path, right: &Path) -> Result<()> {
    let uid = owner()?;
    let a = inventory(left, uid)?;
    let b = inventory(right, uid)?;
    if a != b {
        return Err("private_copies_differ");
    }
    let bytes = serde_json::to_vec(&a.0).map_err(|_| "private_inventory_encode_failed")?;
    println!(
        "{}",
        json!({"probe_kind":"private_copy_integrity",
        "files_per_copy":a.0.len(),"directories_per_copy":a.1,"digest_bindings_per_copy":a.2,
        "paths_and_bytes_identical":true,"owner_and_permissions_verified":true,
        "inventory_algorithm":"sha256_of_sorted_json_relative_path_to_sha256_map",
        "inventory_sha256":format!("{:x}",Sha256::digest(bytes)),"model_requests":0})
    );
    Ok(())
}

pub(super) fn audit(root: &Path) -> Result<()> {
    let (binding, cases, duplicates) = candidates(root)?;
    println!(
        "{}",
        json!({"probe_kind":"local_provenance_and_duplicate_candidates",
        "verified_sources":binding["source_hashes"].as_object().map_or(0, |v| v.len()),
        "candidate_cases":cases.len(), "normalized_duplicate_candidates":duplicates,
        "unique_normalized_candidates":cases.len()-duplicates,
        "accepted_gold_cases":0,"model_requests":0})
    );
    Ok(())
}

pub(super) fn prepare(root: &Path, destination: &Path) -> Result<()> {
    let uid = owner()?;
    restricted(destination, true, uid)?;
    let (binding, cases, duplicates) = candidates(root)?;
    let count = cases.len();
    let review = json!({"schema":"brain.q.local-review.v1", "binding":binding,
        "source_root":root, "cases":cases, "model_runs":0});
    let bytes = serde_json::to_vec_pretty(&review).map_err(|_| "review_encode_failed")?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    write_new(&destination.join("review-v1.json"), &bytes)?;
    println!(
        "{}",
        json!({"probe_kind":"local_review_preparation", "review_sha256":digest,
        "candidate_cases":count,"normalized_duplicate_candidates":duplicates,
        "accepted_gold_cases":0,"model_requests":0})
    );
    Ok(())
}

fn valid_digest(value: &Value) -> bool {
    value.as_str().is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    })
}

pub(super) fn check(path: &Path) -> Result<()> {
    let uid = owner()?;
    restricted(path.parent().ok_or("review_parent_missing")?, true, uid)?;
    restricted(path, false, uid)?;
    let bytes = fs::read(path).map_err(|_| "review_unavailable")?;
    let review: Value = serde_json::from_slice(&bytes).map_err(|_| "review_invalid_json")?;
    if review["schema"] != "brain.q.local-review.v1" {
        return Err("review_schema_invalid");
    }
    let root = Path::new(
        review["source_root"]
            .as_str()
            .ok_or("review_source_missing")?,
    );
    let (binding, originals, _) = candidates(root)?;
    if binding != review["binding"] {
        return Err("review_source_binding_changed");
    }
    let mut ids = BTreeSet::new();
    let mut texts = BTreeSet::new();
    let mut sources = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for case in review["cases"]
        .as_array()
        .ok_or("review_cases_missing")?
        .iter()
        .filter(|case| case["accepted_gold"] == true)
    {
        let original = originals
            .iter()
            .find(|v| v["case_id"] == case["case_id"])
            .ok_or("review_case_not_in_source")?;
        for field in ["source", "source_row", "source_sha256", "original"] {
            if case[field] != original[field] {
                return Err("review_original_changed");
            }
        }
        let id = case["case_id"].as_str().ok_or("review_id_missing")?;
        let question = case["provider_question"]
            .as_str()
            .ok_or("review_question_missing")?;
        let kind = case["expected_response_kind"]
            .as_str()
            .ok_or("review_kind_missing")?;
        if !ids.insert(id) || !texts.insert(normalized(case["original"].as_str().unwrap_or(""))) {
            return Err("review_accepted_duplicate");
        }
        if case["authenticity"] != "locally_verified_real_question"
            || case["privacy"] != "locally_verified_minimal_no_foreign_person_data"
            || !valid_digest(&case["review_evidence_sha256"])
            || question.trim().is_empty()
            || question.contains("<@")
            || question.contains("<#")
            || question
                .split(|c: char| !c.is_ascii_digit())
                .any(|s| s.len() >= 17)
            || ![
                "game_original_source",
                "patch_original_source",
                "server_or_own_status",
                "coaching_or_community_help",
                "self_description_without_internals",
                "honest_unknown_or_nonsense",
            ]
            .contains(&kind)
            || !case["original_facts"]
                .as_array()
                .is_some_and(|v| !v.is_empty())
            || !case["original_fact_evidence"]
                .as_array()
                .is_some_and(|v| !v.is_empty() && v.iter().all(valid_digest))
            || ![
                "discord_mention",
                "discord_dm",
                "discord_thread",
                "twitch_chat",
            ]
            .contains(&case["transport"].as_str().unwrap_or(""))
        {
            return Err("review_accepted_case_incomplete");
        }
        sources.insert(case["source"].as_str().ok_or("review_source_invalid")?);
        kinds.insert(kind);
    }
    if ids.len() < 30
        || !sources.contains("botlogs")
        || !sources.contains("dm")
        || !(sources.contains("twitch") || sources.contains("twitch-brain"))
        || kinds.len() < 6
    {
        return Err("review_gold_coverage_incomplete");
    }
    println!(
        "{}",
        json!({"probe_kind":"local_review_structure_only",
        "review_sha256":format!("{:x}", Sha256::digest(bytes)),
        "accepted_gold_cases":ids.len(),"covered_response_kinds":kinds.len(),
        "semantic_review_not_performed_by_validator":true,
        "live_execution_authorized":false,"model_requests":0})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::DirBuilderExt;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fixture(std::path::PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "brain-q-acceptance-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::DirBuilder::new().mode(0o700).create(&path).unwrap();
            Self(fs::canonicalize(path).unwrap())
        }

        fn snapshot(&self, name: &str, value: &Value) -> String {
            let bytes = serde_json::to_vec(value).unwrap();
            let digest = format!("{:x}", Sha256::digest(&bytes));
            write_new(&self.0.join(format!("{name}.json")), &bytes).unwrap();
            write_new(&self.0.join(format!("{name}.sha256")), digest.as_bytes()).unwrap();
            digest
        }

        fn plan(&self) {
            let digest = self.snapshot(
                "botlogs",
                &json!({"original_rows":[
                    {"original":"<@123> Haze? <@123> Pocket?"},
                    {"original":"Haze? Pocket?"}
                ]}),
            );
            self.snapshot("partial-plan-v1", &json!({
                "schema":"brain.q.partial-plan.v1", "version":"fixture",
                "source_hashes":{"botlogs":digest},"cases":[
                    {"case_id":"botlogs-0000","source":"botlogs","source_row":0,"source_sha256":digest},
                    {"case_id":"botlogs-0001","source":"botlogs","source_row":1,"source_sha256":digest}
                ]
            }));
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn actual_filesystem_rejects_open_permissions_and_symlinks() {
        let fixture = Fixture::new();
        let path = fixture.0.join("original.json");
        write_new(&path, b"{}").unwrap();
        let uid = owner().unwrap();
        assert!(restricted(&path, false, uid).is_ok());
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(
            restricted(&path, false, uid),
            Err("private_path_not_restricted")
        );
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        let link = fixture.0.join("link.json");
        std::os::unix::fs::symlink(&path, &link).unwrap();
        assert_eq!(
            restricted(&link, false, uid),
            Err("private_path_not_canonical")
        );
    }

    #[test]
    fn existing_snapshot_hashes_and_original_messages_are_bound() {
        let fixture = Fixture::new();
        fixture.plan();
        let (_, cases, duplicates) = candidates(&fixture.0).unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(duplicates, 1);
        assert_eq!(cases[0]["accepted_gold"], false);
        assert_eq!(cases[0]["original"], "<@123> Haze? <@123> Pocket?");
        fs::write(fixture.0.join("botlogs.json"), b"{}").unwrap();
        assert_eq!(
            candidates(&fixture.0).unwrap_err(),
            "source_digest_mismatch"
        );
    }

    #[test]
    fn preparation_never_accepts_gold_or_overwrites_local_review() {
        let source = Fixture::new();
        let destination = Fixture::new();
        source.plan();
        prepare(&source.0, &destination.0).unwrap();
        let path = destination.0.join("review-v1.json");
        let before = fs::read(&path).unwrap();
        assert_eq!(check(&path), Err("review_gold_coverage_incomplete"));
        assert_eq!(
            prepare(&source.0, &destination.0),
            Err("snapshot_exists_or_write_denied")
        );
        assert_eq!(fs::read(path).unwrap(), before);
    }

    #[test]
    fn backup_comparison_rejects_byte_and_digest_drift() {
        let a = Fixture::new();
        let b = Fixture::new();
        a.snapshot("public-items", &json!([]));
        b.snapshot("public-items", &json!([]));
        assert!(compare(&a.0, &b.0).is_ok());
        fs::write(b.0.join("public-items.json"), b"[1]").unwrap();
        assert_eq!(compare(&a.0, &b.0), Err("private_binding_failed"));
    }

    #[test]
    fn duplicate_normalization_preserves_multi_question_message() {
        assert_eq!(normalized("<@123> Haze? <@!123> Pocket?"), "haze? pocket?");
        assert_ne!(normalized("Haze? Pocket?"), normalized("Haze?"));
        assert_eq!(normalized("  Haze?\n"), normalized("haze?"));
        assert_eq!(normalized("<@person> Haze?"), "<@person> haze?");
    }

    #[test]
    fn evidence_digests_reject_missing_or_short_values() {
        assert!(!valid_digest(&Value::Null));
        assert!(!valid_digest(&json!("0")));
        assert!(!valid_digest(&json!("A".repeat(64))));
        assert!(valid_digest(&json!("a".repeat(64))));
    }
}
