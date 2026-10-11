#![recursion_limit = "256"]

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
#[path = "../../../../rust/crates/dbrain-sources/src/error.rs"]
mod error;
pub use error::{Result, SourcesError};
#[path = "../../../../rust/crates/dbrain-sources/src/util.rs"]
mod util;
mod wiki_inventory;
use wiki_inventory::{WikiInventoryOptions, WikiSourceContext};
const ROOT: &str = "/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a";
const OUTPUT: &str = "normalized-fix6";
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
#[link(name = "libzstd.so.1", kind = "dylib", modifiers = "+verbatim")]
unsafe extern "C" {
    fn ZSTD_decompress(
        dst: *mut std::ffi::c_void,
        capacity: usize,
        src: *const std::ffi::c_void,
        size: usize,
    ) -> usize;
    fn ZSTD_isError(code: usize) -> u32;
    fn ZSTD_getErrorName(code: usize) -> *const std::ffi::c_char;
    fn ZSTD_getFrameContentSize(src: *const std::ffi::c_void, size: usize) -> u64;
}
fn decompress(bytes: &[u8]) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    let frame_size = unsafe { ZSTD_getFrameContentSize(bytes.as_ptr().cast(), bytes.len()) };
    let limit = 512usize * 1024 * 1024;
    let capacity = if frame_size == u64::MAX {
        limit
    } else {
        usize::try_from(frame_size)?
    };
    if capacity > limit || frame_size == u64::MAX - 1 {
        return Err("zstd invalid frame or decompression size exceeds 512MiB".into());
    }
    let mut output = vec![0u8; capacity];
    let size = unsafe {
        ZSTD_decompress(
            output.as_mut_ptr().cast(),
            output.len(),
            bytes.as_ptr().cast(),
            bytes.len(),
        )
    };
    if unsafe { ZSTD_isError(size) } != 0 {
        let message =
            unsafe { std::ffi::CStr::from_ptr(ZSTD_getErrorName(size)) }.to_string_lossy();
        return Err(format!("zstd: {message}").into());
    }
    output.truncate(size);
    Ok(output)
}
fn save_new(path: &Path, value: &Value) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(&serde_json::to_vec_pretty(value)?)?;
    file.sync_all()?;
    Ok(())
}
fn verify_fact_values(doc: &Value) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    let facts = doc["facts"].as_array().ok_or("missing facts")?;
    let metadata = &doc["metadata"];
    let structured = metadata["content_representation"] == "revision_slot"
        && metadata["namespace_name"].as_str().is_some_and(|name| {
            name.eq_ignore_ascii_case("data") || name.eq_ignore_ascii_case("bucket")
        })
        && metadata["content_model"].as_str().is_some_and(|model| {
            model.eq_ignore_ascii_case("json") || model.eq_ignore_ascii_case("JsonConfig.Json")
        });
    if !structured {
        if !facts.is_empty() {
            return Err("facts without structured original content".into());
        }
        return Ok(0);
    }
    let original: Value = serde_json::from_str(doc["content"].as_str().ok_or("missing content")?)?;
    let mut pending = vec![&original];
    let mut leaves = 0usize;
    while let Some(value) = pending.pop() {
        match value {
            Value::Object(values) => pending.extend(values.values()),
            Value::Array(values) => pending.extend(values),
            _ => leaves += 1,
        }
    }
    let mut pointers = BTreeSet::new();
    let mut fact_ids = BTreeSet::new();
    let mut numbers = 0;
    for fact in facts {
        let pointer = fact["qualifiers"]["json_pointer"]
            .as_str()
            .ok_or("missing JSON pointer")?;
        let value = original
            .pointer(pointer)
            .ok_or("unknown original JSON pointer")?;
        if value.is_array()
            || value.is_object()
            || !pointers.insert(pointer)
            || !fact_ids.insert(fact["fact_id"].as_str().ok_or("missing fact id")?)
            || fact["fact_id"] != format!("json:{}", hash(pointer.as_bytes()))
            || fact["subject"] != doc["document_id"]
            || fact["predicate"] != "wiki.data.value"
            || fact["source_span"]
                != format!(
                    "revision:{}:json{pointer}",
                    doc["revision"].as_str().ok_or("missing revision")?
                )
            || fact.get("value") != Some(value)
        {
            return Err(format!("original fact value/identity mismatch at {pointer}").into());
        }
        if let Some(number) = value.as_number() {
            let actual = fact["value"]
                .as_number()
                .ok_or("JSON number changed type")?;
            if actual.as_str() != number.as_str() {
                return Err(format!("numeric representation mismatch at {pointer}").into());
            }
            numbers += 1;
        }
    }
    if leaves != facts.len() {
        return Err("original leaves missing from facts".into());
    }
    Ok(numbers)
}
fn verify(path: &Path) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let mut keys = BTreeSet::new();
    let mut facts = 0usize;
    let mut numbers = 0usize;
    let mut pages = BTreeSet::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let doc: Value = serde_json::from_str(&line?)?;
        let content = doc["content"].as_str().ok_or("missing content")?;
        if doc["content_sha256"] != hash(content.as_bytes())
            || doc["contract_version"] != "wiki-spielwissen-v1"
            || doc["license"]["redistribution_allowed"] != false
        {
            return Err("contract/hash/license mismatch".into());
        }
        let key = (
            doc["document_id"].as_str().ok_or("missing id")?.to_owned(),
            doc["revision"]
                .as_str()
                .ok_or("missing revision")?
                .to_owned(),
        );
        pages.insert(key.0.clone());
        if !keys.insert(key) {
            return Err("duplicate document revision".into());
        }
        facts += doc["facts"].as_array().ok_or("missing facts")?.len();
        numbers += verify_fact_values(&doc)?;
    }
    Ok(
        json!({"documents": keys.len(), "pages_with_content": pages.len(), "facts": facts, "exact_fact_value_matches": facts, "numeric_fact_value_matches": numbers, "jsonl_sha256": hash(&fs::read(path)?)}),
    )
}
fn verify_original(
    path: &Path,
    xml: &str,
    context: &WikiSourceContext,
) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let tree = roxmltree::Document::parse(xml)?;
    let text = |node: roxmltree::Node<'_, '_>, name: &str| {
        node.children()
            .find(|n| n.has_tag_name(name))
            .and_then(|n| n.text())
            .unwrap_or("")
            .to_owned()
    };
    let mut expected = BTreeMap::new();
    let mut duplicates = 0usize;
    let source_id = context.source_id()?;
    for page in tree
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("page"))
    {
        let id = text(page, "id");
        for rev in page.children().filter(|n| n.has_tag_name("revision")) {
            if let Some(content) = rev.children().find(|n| n.has_tag_name("text")) {
                if content.attribute("deleted").is_some() {
                    continue;
                }
                let key = (format!("wiki:{source_id}:page:{id}"), text(rev, "id"));
                let original = content.text().unwrap_or("");
                let digest = hash(original.as_bytes());
                if let Some((previous, previous_text)) =
                    expected.insert(key, (digest.clone(), original))
                {
                    if previous != digest || previous_text != original {
                        return Err("original XML has conflicting version content".into());
                    }
                    duplicates += 1;
                }
            }
        }
    }
    let expected_count = expected.len();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let doc: Value = serde_json::from_str(&line?)?;
        let key = (
            doc["document_id"]
                .as_str()
                .ok_or("document id missing")?
                .to_owned(),
            doc["revision"]
                .as_str()
                .ok_or("revision missing")?
                .to_owned(),
        );
        let (expected_hash, expected_text) =
            expected.remove(&key).ok_or("unknown source version")?;
        if Some(expected_hash.as_str()) != doc["content_sha256"].as_str()
            || Some(expected_text) != doc["content"].as_str()
        {
            return Err(format!("source version/hash/text mismatch: {:?}", key).into());
        }
        if doc["source_id"] != source_id
            || !doc["source_locator"]
                .as_str()
                .is_some_and(|url| url.starts_with(&format!("{}/", context.source_origin)))
        {
            return Err("source identity/locator mismatch".into());
        }
    }
    if !expected.is_empty() {
        return Err(format!(
            "{} original XML versions missing from JSONL",
            expected.len()
        )
        .into());
    }
    Ok(
        json!({"exact_original_revision_hash_matches": expected_count, "identical_duplicate_revision_blocks": duplicates, "missing_original_versions": 0, "source_identity_checked": true}),
    )
}
fn xml_inventory(xml: &str) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let tree = roxmltree::Document::parse(xml)?;
    let mut pages = BTreeSet::new();
    let mut revisions = BTreeSet::new();
    let mut ns: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut page_blocks = 0usize;
    let mut rev_blocks = 0usize;
    let mut hidden = 0usize;
    let mut missing = 0usize;
    let mut ns_schema = BTreeMap::new();
    let text = |node: roxmltree::Node<'_, '_>, name: &str| {
        node.children()
            .find(|n| n.has_tag_name(name))
            .and_then(|n| n.text())
            .unwrap_or("")
            .to_owned()
    };
    for node in tree.descendants().filter(|n| n.has_tag_name("namespace")) {
        ns_schema.insert(
            node.attribute("key").unwrap_or("").to_owned(),
            node.text().unwrap_or("").to_owned(),
        );
    }
    for page in tree
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("page"))
    {
        page_blocks += 1;
        let id = text(page, "id");
        pages.insert(id.clone());
        ns.entry(text(page, "ns")).or_default().insert(id);
        for rev in page.children().filter(|n| n.has_tag_name("revision")) {
            rev_blocks += 1;
            revisions.insert(text(rev, "id"));
            match rev.children().find(|n| n.has_tag_name("text")) {
                Some(n) if n.attribute("deleted").is_some() => hidden += 1,
                None => missing += 1,
                _ => (),
            }
        }
    }
    let counts: BTreeMap<_, _> = ns
        .into_iter()
        .map(|(id, values)| (id, values.len()))
        .collect();
    Ok(
        json!({"page_blocks": page_blocks, "revision_blocks": rev_blocks, "distinct_page_ids": pages.len(), "distinct_revision_ids": revisions.len(), "namespace_id_counts": counts, "namespace_schema": ns_schema, "suppressed_text_blocks": hidden, "missing_text_blocks": missing, "coverage_scope": "actual_historical_xml_ids_not_current_wiki", "inventory_complete": false}),
    )
}
fn dump(
    name: &str,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let archive = Path::new(ROOT).join("dump-evidence").join(name);
    let compressed_hash = hash(&fs::read(&archive)?);
    let decompressed = decompress(&fs::read(&archive)?)?;
    let xml_hash = hash(&decompressed);
    let size = decompressed.len();
    let xml = String::from_utf8(decompressed)?;
    let mut evidence = xml_inventory(&xml)?;
    evidence["archive"] = json!(archive);
    evidence["compressed_sha256"] = json!(compressed_hash);
    evidence["xml_sha256"] = json!(xml_hash);
    evidence["xml_bytes"] = json!(size);
    let mut ctx = if name == "deadlock.wiki-20250415-history.xml.zst" {
        context.clone()
    } else {
        WikiSourceContext::from_mediawiki_export(&xml)?
    };
    ctx.source_capture_sha256 = Some(compressed_hash.clone());
    ctx.historical_capture = true;
    if name != "deadlock.wiki-20250415-history.xml.zst" {
        let metadata_path = Path::new(ROOT).join("dump-evidence").join(format!(
            "wiki-{}.metadata.json",
            name.trim_end_matches("-history.xml.zst")
        ));
        let metadata_bytes = fs::read(&metadata_path)?;
        let metadata: Value = serde_json::from_slice(&metadata_bytes)?;
        let url = metadata
            .pointer("/metadata/licenseurl")
            .and_then(Value::as_str);
        let rights = metadata.pointer("/metadata/rights").and_then(Value::as_str);
        let name = rights.or(match url {
            Some("https://creativecommons.org/licenses/by-nc-sa/4.0/") => Some("CC BY-NC-SA 4.0"),
            Some("https://creativecommons.org/licenses/by-sa/4.0/") => Some("CC BY-SA 4.0"),
            _ => None,
        });
        if let Some(name) = name {
            ctx.license = json!({"name": name, "url": url, "attribution": format!("{} contributors", ctx.source_origin), "redistribution_allowed": false});
            ctx.license_observed_at = Some(options.observed_at.clone());
        }
        evidence["license_evidence_path"] = json!(metadata_path);
        evidence["license_evidence_sha256"] = json!(hash(&metadata_bytes));
        evidence["license_evidence_scope"] =
            json!("same_archive_metadata_not_individual_revision_rights");
    }
    evidence["source_id"] = json!(ctx.source_id()?);
    evidence["source_origin"] = json!(ctx.source_origin);
    evidence["source_article_base"] = json!(ctx.source_article_base);
    let target = Path::new(ROOT)
        .join(OUTPUT)
        .join(name.trim_end_matches(".xml.zst"));
    if target.exists() {
        return Err(format!("output already exists: {}", target.display()).into());
    }
    fs::create_dir_all(&target)?;
    if name == "deadlock.wiki-20250415-history.xml.zst"
        && (compressed_hash != "bea69e14f1ec35b8bbfe157b3cf9cf362f888e2321e9aa5a46a03c1670120825"
            || xml_hash != "4a64288c2776b46a3722054aa4fe7f61cae167b1f5e3d4125c94080c37ab240e"
            || size != 177037503)
    {
        return Err("main archive integrity mismatch".into());
    }
    match wiki_inventory::write_wiki_export_capture(&target, &xml, &ctx, options) {
        Ok(report) => {
            let before = verify(&report.documents_path)?;
            evidence["original_revision_verification"] =
                verify_original(&report.documents_path, &xml, &ctx)?;
            let again = wiki_inventory::write_wiki_export_capture(&target, &xml, &ctx, options)?;
            let after = verify(&again.documents_path)?;
            if before != after {
                return Err("repeat changed JSONL".into());
            }
            if before["documents"] != evidence["distinct_revision_ids"]
                || before["pages_with_content"] != evidence["distinct_page_ids"]
                || serde_json::to_value(&report.namespace_counts)?
                    != evidence["namespace_id_counts"]
            {
                return Err("full historical XML denominator mismatch".into());
            }
            evidence["normalization"] = before;
            evidence["repeat_identical"] = json!(true);
            evidence["report"] = serde_json::to_value(report)?;
        }
        Err(error) => {
            evidence["normalization_error"] = json!(error.to_string());
        }
    }
    evidence["classification"] = json!(if name.starts_with("deadlockwiki.miraheze.org") {
        "no_proven_game_knowledge_corpus"
    } else {
        "historical_wiki_text_corpus"
    });
    save_new(&target.join("run-evidence.json"), &evidence)?;
    Ok(evidence)
}
fn verify_api_original(
    path: &Path,
    inputs: &[(Value, WikiSourceContext)],
) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    let mut expected = BTreeMap::new();
    for (payload, context) in inputs {
        let source_id = context.source_id()?;
        for page in payload
            .pointer("/query/pages")
            .and_then(Value::as_object)
            .ok_or("API original pages missing")?
            .values()
        {
            for revision in page["revisions"]
                .as_array()
                .ok_or("API original revisions missing")?
            {
                let slot = revision.pointer("/slots/main").unwrap_or(revision);
                let content = slot
                    .get("content")
                    .or_else(|| slot.get("*"))
                    .and_then(Value::as_str)
                    .ok_or("API original text missing")?;
                let key = (
                    format!("wiki:{source_id}:page:{}", page["pageid"]),
                    revision["revid"].to_string(),
                );
                if expected
                    .insert(key, content)
                    .is_some_and(|previous| previous != content)
                {
                    return Err("conflicting API original versions".into());
                }
            }
        }
    }
    let count = expected.len();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let document: Value = serde_json::from_str(&line?)?;
        let key = (
            document["document_id"]
                .as_str()
                .ok_or("missing id")?
                .to_owned(),
            document["revision"]
                .as_str()
                .ok_or("missing revision")?
                .to_owned(),
        );
        let original = expected
            .remove(&key)
            .ok_or("unexpected API output identity")?;
        if document["content"] != original
            || document["content_sha256"] != hash(original.as_bytes())
        {
            return Err("API original text mismatch".into());
        }
    }
    if !expected.is_empty() {
        return Err("API original versions missing".into());
    }
    Ok(count)
}
fn legacy(
    options: &WikiInventoryOptions,
) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let site: Value = serde_json::from_slice(&fs::read(
        Path::new(ROOT).join("source-evidence/wiki-siteinfo.json"),
    )?)?;
    let mut base = WikiSourceContext::from_siteinfo(&site)?;
    base.historical_capture = true;
    base.license_observed_at = Some("2026-10-03T03:18:42Z".into());
    let mut sidecars = BTreeMap::new();
    for entry in fs::read_dir(Path::new(ROOT).join("cache-provenance"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "json") {
            let sidecar: Value = serde_json::from_slice(&fs::read(&path)?)?;
            let payload_path = PathBuf::from(path.to_string_lossy().trim_end_matches(".json"));
            let payload: Value = serde_json::from_slice(&fs::read(&payload_path)?)?;
            let pages = payload
                .pointer("/query/pages")
                .and_then(Value::as_object)
                .ok_or("sidecar payload lacks pages")?;
            let fetched = sidecar["fetched_at"]
                .as_i64()
                .ok_or("sidecar fetched_at missing")?;
            let fetched = chrono::DateTime::from_timestamp(fetched, 0)
                .ok_or("sidecar fetched_at out of range")?
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            for page in pages.values() {
                sidecars.insert(
                    (
                        page["pageid"].to_string(),
                        page["revisions"][0]["revid"].to_string(),
                    ),
                    fetched.clone(),
                );
            }
        }
    }
    let target = Path::new(ROOT).join(OUTPUT).join("legacy-2026-captures");
    if target.exists() {
        return Err("legacy output already exists".into());
    }
    let mut inputs = Vec::new();
    for entry in fs::read_dir(Path::new(ROOT).join("legacy-raw"))? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(&path)?;
        let payload: Value = serde_json::from_slice(&bytes)?;
        let page = payload
            .pointer("/query/pages")
            .and_then(Value::as_object)
            .and_then(|p| p.values().next())
            .ok_or("raw lacks page")?;
        let key = (
            page["pageid"].to_string(),
            page["revisions"][0]["revid"].to_string(),
        );
        let mut context = base.clone();
        context.source_fetched_at = Some(sidecars.get(&key).ok_or("unmatched sidecar")?.clone());
        context.source_capture_sha256 = Some(hash(&bytes));
        inputs.push((payload, context));
    }
    for (payload, ctx) in &inputs {
        wiki_inventory::write_wiki_api_capture(&target, payload, ctx, options)?;
    }
    let before = verify(&target.join("documents.jsonl"))?;
    for (payload, ctx) in &inputs {
        wiki_inventory::write_wiki_api_capture(&target, payload, ctx, options)?;
    }
    let after = verify(&target.join("documents.jsonl"))?;
    if before != after {
        return Err("legacy repeat changed JSONL".into());
    }
    let exact_original_versions = verify_api_original(&target.join("documents.jsonl"), &inputs)?;
    let evidence = json!({"normalization": before, "exact_original_api_versions": exact_original_versions, "input_captures": inputs.len(), "repeat_identical": true, "report": wiki_inventory::read_wiki_inventory_report(&target, options.max_total_bytes)?});
    save_new(&target.join("run-evidence.json"), &evidence)?;
    Ok(evidence)
}
fn precision_check() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let archive = Path::new(ROOT).join("dump-evidence/deadlock.wiki-20250415-history.xml.zst");
    let compressed = fs::read(&archive)?;
    if hash(&compressed) != "bea69e14f1ec35b8bbfe157b3cf9cf362f888e2321e9aa5a46a03c1670120825" {
        return Err("precision archive hash mismatch".into());
    }
    let xml = String::from_utf8(decompress(&compressed)?)?;
    if hash(xml.as_bytes()) != "4a64288c2776b46a3722054aa4fe7f61cae167b1f5e3d4125c94080c37ab240e" {
        return Err("precision XML hash mismatch".into());
    }
    let tree = roxmltree::Document::parse(&xml)?;
    fn child<'a, 'i>(node: roxmltree::Node<'a, 'i>, name: &str) -> Option<roxmltree::Node<'a, 'i>> {
        node.children().find(|node| node.has_tag_name(name))
    }
    let root = tree.root_element();
    let siteinfo = child(root, "siteinfo").ok_or("original siteinfo missing")?;
    let page = root
        .children()
        .find(|node| {
            node.has_tag_name("page")
                && child(*node, "id").and_then(|node| node.text()) == Some("2880")
        })
        .ok_or("original page 2880 missing")?;
    let revision = page
        .children()
        .find(|node| {
            node.has_tag_name("revision")
                && child(*node, "id").and_then(|node| node.text()) == Some("18108")
        })
        .ok_or("original revision 18108 missing")?;
    let content = child(revision, "text")
        .and_then(|node| node.text())
        .ok_or("original text missing")?;
    let page_headers = ["title", "ns", "id"]
        .into_iter()
        .map(|name| {
            child(page, name)
                .map(|node| &xml[node.range()])
                .ok_or("original page header missing")
        })
        .collect::<std::result::Result<Vec<_>, _>>()?
        .join("");
    let root_xml = &xml[root.range()];
    let root_open = &root_xml[..=root_xml.find('>').ok_or("root opening missing")?];
    let selection = format!(
        "{root_open}{}<page>{page_headers}{}</page></mediawiki>",
        &xml[siteinfo.range()],
        &xml[revision.range()]
    );
    let context = WikiSourceContext::from_mediawiki_export(&selection)?;
    let target = Path::new("/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/bereiche/a/fix6-final-precision-spool");
    if target.exists() {
        return Err("precision proof output already exists".into());
    }
    let options = WikiInventoryOptions {
        observed_at: "2026-10-03T11:23:00Z".into(),
        ..Default::default()
    };
    let report = wiki_inventory::write_wiki_export_capture(target, &selection, &context, &options)?;
    let before = fs::read(&report.documents_path)?;
    let document: Value = serde_json::from_slice(&before)?;
    if document["content"] != content
        || document["content_sha256"] != hash(content.as_bytes())
        || document["document_id"] != "wiki:deadlock-wiki:page:2880"
        || document["revision"] != "18108"
    {
        return Err("precision original/identity mismatch".into());
    }
    let values = verify(&report.documents_path)?;
    let old_path =
        Path::new(ROOT).join("normalized/deadlock.wiki-20250415-history/documents.jsonl");
    let mut old = None;
    for line in BufReader::new(fs::File::open(old_path)?).lines() {
        let candidate: Value = serde_json::from_str(&line?)?;
        if candidate["document_id"] == document["document_id"]
            && candidate["revision"] == document["revision"]
        {
            old = Some(candidate);
            break;
        }
    }
    let old = old.ok_or("old precision example missing")?;
    if old["content"] != content || old["content_sha256"] != document["content_sha256"] {
        return Err("old example does not match original text".into());
    }
    let mut examples = Vec::new();
    for (pointer, expected, previous) in [
        (
            "/hero_atlas/FalloffStartRange",
            "20.000010800000002",
            "20.0000108",
        ),
        ("/hero_ghost/DPS", "55.555555555555564", "55.55555555555557"),
    ] {
        let find = |doc: &Value| {
            doc["facts"]
                .as_array()
                .unwrap()
                .iter()
                .find(|fact| fact["qualifiers"]["json_pointer"] == pointer)
                .map(|fact| fact["value"].clone())
        };
        let actual = find(&document).ok_or("new precision fact missing")?;
        let previous_actual = find(&old).ok_or("old precision fact missing")?;
        if actual.as_number().map(serde_json::Number::as_str) != Some(expected)
            || previous_actual.as_number().map(serde_json::Number::as_str) != Some(previous)
            || actual == previous_actual
        {
            return Err("precision correction not proven".into());
        }
        examples.push(json!({"pointer": pointer, "original_and_corrected": actual, "previous": previous_actual}));
    }
    wiki_inventory::write_wiki_export_capture(target, &selection, &context, &options)?;
    if before != fs::read(&report.documents_path)? {
        return Err("precision spool repeat changed JSONL".into());
    }
    let republished = wiki_inventory::read_wiki_inventory_report(target, options.max_total_bytes)?;
    if before != fs::read(&republished.documents_path)? {
        return Err("precision spool reread changed JSONL".into());
    }
    println!(
        "{}",
        serde_json::to_string(
            &json!({"serde_json_version": "1.0.151", "features": ["arbitrary_precision"], "examples": examples, "verification": values, "exact_original_text": true, "repeat_and_republication_identical": true})
        )?
    );
    Ok(())
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--precision-check") {
        return precision_check();
    }
    let observed = std::env::args()
        .nth(1)
        .ok_or("UTC observation argument required")?;
    let site: Value = serde_json::from_slice(&fs::read(
        Path::new(ROOT).join("dump-evidence/siteinfo.json"),
    )?)?;
    let mut context = WikiSourceContext::from_siteinfo(&site)?;
    context.historical_capture = true;
    context.license_observed_at = Some(observed.clone());
    let options = WikiInventoryOptions {
        observed_at: observed,
        max_total_bytes: 2 * 1024 * 1024 * 1024,
        max_response_bytes: 64 * 1024 * 1024,
        ..Default::default()
    };
    let mut results = Vec::new();
    for name in [
        "deadlock.wiki-20250415-history.xml.zst",
        "deadlocked.wiki-20241107-history.xml.zst",
        "deadlockwiki.org_mw-20260130-history.xml.zst",
        "deadlock.miraheze.org_w-20240616-history.xml.zst",
        "deadlockwiki.miraheze.org_w-20231203-history.xml.zst",
    ] {
        let value = match dump(name, &context, &options) {
            Ok(v) => v,
            Err(e) => json!({"archive_name": name, "run_error": e.to_string()}),
        };
        println!("{}", serde_json::to_string(&value)?);
        results.push(value);
    }
    let value = match legacy(&options) {
        Ok(v) => v,
        Err(e) => json!({"source": "legacy", "run_error": e.to_string()}),
    };
    println!("{}", serde_json::to_string(&value)?);
    results.push(value);
    let mut global_keys = BTreeSet::new();
    for result in &results {
        if let Some(path) = result
            .pointer("/report/documents_path")
            .and_then(Value::as_str)
        {
            for line in BufReader::new(fs::File::open(path)?).lines() {
                let doc: Value = serde_json::from_str(&line?)?;
                let id = doc["document_id"].as_str().ok_or("missing global id")?;
                let source = doc["source_id"].as_str().ok_or("missing global source")?;
                let revision = doc["revision"].as_str().ok_or("missing global revision")?;
                if !id.starts_with(&format!("wiki:{source}:"))
                    || !global_keys.insert((id.to_owned(), revision.to_owned()))
                {
                    return Err("global source identity or document revision collision".into());
                }
            }
        }
    }
    save_new(
        &Path::new(ROOT)
            .join(OUTPUT)
            .join("harness-global-identities.json"),
        &json!({"unique_document_revisions": global_keys.len(), "duplicate_document_revisions": 0}),
    )?;
    save_new(
        &Path::new(ROOT)
            .join(OUTPUT)
            .join("harness-run-summary.json"),
        &json!(results),
    )?;
    if results
        .iter()
        .any(|v| v.get("run_error").is_some() || v.get("normalization_error").is_some())
    {
        std::process::exit(2);
    }
    Ok(())
}

#[cfg(test)]
mod value_verification_tests {
    use super::*;

    fn document() -> Value {
        let pointer = "/DPS";
        json!({
            "document_id": "wiki:deadlock-wiki:page:2880",
            "revision": "18108",
            "content": "{\"DPS\":55.555555555555564}",
            "metadata": {"content_representation": "revision_slot", "namespace_name": "Data", "content_model": "json"},
            "facts": [{
                "fact_id": format!("json:{}", hash(pointer.as_bytes())),
                "subject": "wiki:deadlock-wiki:page:2880",
                "predicate": "wiki.data.value",
                "source_span": "revision:18108:json/DPS",
                "value": serde_json::from_str::<Value>("55.555555555555564").unwrap(),
                "qualifiers": {"json_pointer": pointer}
            }]
        })
    }

    #[test]
    fn original_number_is_verified_without_float_conversion() {
        assert_eq!(verify_fact_values(&document()).unwrap(), 1);
    }

    #[test]
    fn rounded_or_string_number_is_rejected() {
        for wrong in [
            serde_json::from_str::<Value>("55.55555555555557").unwrap(),
            json!("55.555555555555564"),
        ] {
            let mut doc = document();
            doc["facts"][0]["value"] = wrong;
            assert!(verify_fact_values(&doc).is_err());
        }
    }

    #[test]
    fn missing_duplicate_or_wrong_identity_is_rejected() {
        let mut missing = document();
        missing["facts"] = json!([]);
        assert!(verify_fact_values(&missing).is_err());
        let mut duplicate = document();
        let fact = duplicate["facts"][0].clone();
        duplicate["facts"].as_array_mut().unwrap().push(fact);
        assert!(verify_fact_values(&duplicate).is_err());
        let mut wrong = document();
        wrong["facts"][0]["subject"] = json!("wiki:deadlock-wiki:page:1");
        assert!(verify_fact_values(&wrong).is_err());
    }
}
