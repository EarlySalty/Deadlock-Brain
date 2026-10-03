use std::{collections::VecDeque, fs};

use serde_json::{json, Value};
use tempfile::tempdir;

use super::*;

fn site() -> Value {
    json!({"query": {
        "general": {"server": "https://deadlock.wiki", "lang": "en"},
        "rightsinfo": {"text": "Creative Commons Attribution-NonCommercial-ShareAlike", "url": "https://creativecommons.org/licenses/by-nc-sa/4.0/"},
        "namespaces": {
            "0": {"id": 0, "name": ""},
            "10": {"id": 10, "name": "Template", "canonical": "Template"}
        },
        "statistics": {"pages": 123456, "articles": 939}
    }})
}

fn context() -> WikiSourceContext {
    WikiSourceContext::from_siteinfo(&site()).unwrap()
}

fn options() -> WikiInventoryOptions {
    WikiInventoryOptions {
        enabled: true,
        access_policy_reviewed: true,
        observed_at: "2026-10-03T03:18:42Z".into(),
        ..Default::default()
    }
}

fn page(id: i64, ns: i64, revision: i64, content: &str) -> Value {
    json!({
        "pageid": id, "ns": ns, "title": format!("Page {id}"),
        "revisions": [{
            "revid": revision, "timestamp": "2026-04-28T16:22:50Z",
            "slots": {"main": {"contentmodel": "wikitext", "*": content}}
        }]
    })
}

fn capture(page: Value) -> Value {
    json!({"batchcomplete": true, "query": {"pages": [page]}})
}

fn inventory_page(report: &WikiInventoryReport) -> Value {
    let inventory: Value =
        serde_json::from_slice(&fs::read(&report.inventory_path).unwrap()).unwrap();
    inventory["pages"][0].clone()
}

fn export(domain: &str, blocks: &str) -> String {
    format!(
        r#"<mediawiki xmlns="http://www.mediawiki.org/xml/export-0.11/" xml:lang="en"><siteinfo><base>https://{domain}/wiki/Main_Page</base><wikiid>deadlock</wikiid><namespaces><namespace key="0"/><namespace key="10">Template</namespace></namespaces></siteinfo>{blocks}</mediawiki>"#
    )
}

fn xml_page(revision: i64, text: &str) -> String {
    format!(
        r#"<page><title>Example</title><ns>0</ns><id>1</id><revision><id>{revision}</id><timestamp>2025-01-01T00:00:00Z</timestamp><contributor><username>Actual author</username><id>9</id></contributor><model>wikitext</model>{text}</revision></page>"#
    )
}

fn spool_snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, directory: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &path, files);
            } else if entry.file_name() != "inventory.lock" {
                files.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

fn published_document(report: &WikiInventoryReport) -> Value {
    serde_json::from_str(
        fs::read_to_string(&report.documents_path)
            .unwrap()
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap()
}

fn write_authored_sync_capture(
    root: &Path,
    api: bool,
    text: &str,
    observed_at: &str,
) -> crate::Result<WikiInventoryReport> {
    let mut opts = options();
    opts.observed_at = observed_at.into();
    if api {
        let mut authored = page(1, 0, 101, text);
        authored["revisions"][0]["user"] = json!("Actual author");
        write_wiki_api_capture(root, &capture(authored), &context(), &opts)
    } else {
        let xml = export(
            "deadlock.wiki",
            &xml_page(101, &format!("<text>{text}</text>")),
        );
        let ctx = WikiSourceContext::from_mediawiki_export(&xml)?;
        write_wiki_export_capture(root, &xml, &ctx, &opts)
    }
}

fn document_storage_key(document: &Value) -> String {
    normalize::sha256(
        &serde_json::to_vec(&(&document["document_id"], &document["revision"])).unwrap(),
    )
}

fn supplement_entry(previous: &Value, incoming: &Value) -> Value {
    let metadata = &incoming["metadata"];
    let assertion = json!({
        "revision_author": metadata["revision_author"],
        "revision_contributor": metadata["revision_contributor"],
        "attribution_url": metadata["attribution_url"],
        "license": incoming["license"],
        "source_locator": incoming["source_locator"],
        "source_origin": metadata["source_origin"],
        "source_siteinfo_sha256": metadata["source_siteinfo_sha256"],
        "source_siteinfo_hash_representation": metadata["source_siteinfo_hash_representation"],
        "source_capture_sha256": metadata["source_capture_sha256"],
        "source_fetched_at": metadata["source_fetched_at"],
        "license_observed_at": metadata["license_observed_at"],
        "provenance_capture_format": metadata["provenance_capture_format"],
    });
    json!({
        "document_id": previous["document_id"],
        "revision": previous["revision"],
        "content_sha256": previous["content_sha256"],
        "assertion_sha256": normalize::sha256(&serde_json::to_vec(&assertion).unwrap()),
        "assertion": assertion,
        "observed_at": incoming["observed_at"],
    })
}

fn stored_record_bytes(root: &Path) -> usize {
    ["documents", "conflicts", "provenance"]
        .into_iter()
        .map(|directory| root.join(directory))
        .filter(|directory| directory.exists())
        .flat_map(|directory| fs::read_dir(directory).unwrap())
        .map(|entry| {
            let entry = entry.unwrap();
            assert_eq!(entry.path().extension().unwrap(), "json");
            usize::try_from(entry.metadata().unwrap().len()).unwrap() + 1
        })
        .sum()
}

#[test]
fn api_conflict_then_xml_keeps_separate_authors_captures_and_first_observations() {
    let dir = tempdir().unwrap();
    let mut first = page(1, 0, 101, "A");
    first["revisions"][0]["user"] = json!("Original author");
    let report =
        write_wiki_api_capture(dir.path(), &capture(first), &context(), &options()).unwrap();
    let original = published_document(&report);
    let originals = spool_snapshot(&dir.path().join("documents"));
    let publication = fs::read(&report.documents_path).unwrap();
    let mut later = options();
    later.observed_at = "2026-10-04T01:00:00Z".into();
    assert!(write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "B")),
        &context(),
        &later
    )
    .unwrap_err()
    .to_string()
    .contains("Konflikt erhalten"));
    let key = format!(
        "{}-{}",
        document_storage_key(&original),
        normalize::sha256(b"B")
    );
    let target = dir.path().join("conflicts").join(format!("{key}.json"));
    let first_conflict = fs::read(&target).unwrap();
    let saved: Value = serde_json::from_slice(&first_conflict).unwrap();
    assert!(saved["metadata"]["revision_author"].is_null());
    assert_eq!(saved["observed_at"], later.observed_at);
    let provenance = dir.path().join("provenance").join(format!("{key}.json"));
    let xml = export("deadlock.wiki", &xml_page(101, "<text>B</text>"));
    let mut ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    ctx.source_capture_sha256 = Some(normalize::sha256(xml.as_bytes()));
    ctx.source_fetched_at = Some("2026-05-02T14:59:46Z".into());
    ctx.license_observed_at = Some("2026-10-03T03:18:42Z".into());
    later.observed_at = "2026-10-05T01:00:00Z".into();
    assert!(write_wiki_export_capture(dir.path(), &xml, &ctx, &later)
        .unwrap_err()
        .to_string()
        .contains("Konflikt erhalten"));
    let entries: Value = serde_json::from_slice(&fs::read(&provenance).unwrap()).unwrap();
    assert_eq!(entries.as_array().unwrap().len(), 1);
    let incoming = normalize_mediawiki_export(&xml, &ctx, &later.observed_at)
        .unwrap()
        .documents
        .remove(0);
    assert_eq!(entries[0], supplement_entry(&saved, &incoming));
    assert_eq!(entries[0]["content_sha256"], normalize::sha256(b"B"));
    assert_eq!(entries[0]["assertion"]["revision_author"], "Actual author");
    assert_eq!(entries[0]["assertion"]["revision_contributor"]["id"], "9");
    assert_eq!(
        entries[0]["assertion"]["source_capture_sha256"],
        ctx.source_capture_sha256.clone().unwrap()
    );
    assert_eq!(entries[0]["assertion"]["license"]["name"], "unverified");
    assert_eq!(
        entries[0]["assertion"]["license"]["redistribution_allowed"],
        false
    );
    let before = spool_snapshot(dir.path());
    later.observed_at = "2026-10-06T01:00:00Z".into();
    assert!(write_wiki_export_capture(dir.path(), &xml, &ctx, &later)
        .unwrap_err()
        .to_string()
        .contains("Konflikt erhalten"));
    assert_eq!(before, spool_snapshot(dir.path()));
    let other_xml = xml
        .replace("Actual author", "Other author")
        .replace("<id>9</id>", "<id>10</id>");
    ctx.source_capture_sha256 = Some(normalize::sha256(other_xml.as_bytes()));
    assert!(
        write_wiki_export_capture(dir.path(), &other_xml, &ctx, &later)
            .unwrap_err()
            .to_string()
            .contains("Konflikt erhalten")
    );
    let additional: Value = serde_json::from_slice(&fs::read(&provenance).unwrap()).unwrap();
    assert_eq!(additional.as_array().unwrap().len(), 2);
    assert_eq!(additional[0], entries[0]);
    assert_eq!(
        additional[1]["assertion"]["revision_author"],
        "Other author"
    );
    assert_eq!(
        additional[1]["assertion"]["revision_contributor"]["id"],
        "10"
    );
    assert_eq!(additional[1]["content_sha256"], saved["content_sha256"]);
    assert_eq!(first_conflict, fs::read(&target).unwrap());
    assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
    assert!(!dir
        .path()
        .join("provenance")
        .join(format!("{}.json", document_storage_key(&original)))
        .exists());
    let reopened = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    assert_eq!(publication, fs::read(&reopened.documents_path).unwrap());
    assert_eq!(published_document(&reopened), original);
    assert_eq!(saved["license"]["redistribution_allowed"], false);
}

#[test]
fn same_spool_install_sync_failure_counts_original_and_conflict_at_exact_budget() {
    let document = |id, text| {
        normalize_api_response(
            &capture(page(id, 0, 101, text)),
            &context(),
            &options().observed_at,
        )
        .unwrap()
        .documents
        .remove(0)
    };
    let original = document(1, "A");
    let conflict = document(1, "B");
    let next = document(2, "C");
    for is_conflict in [false, true] {
        for room_for_next in [false, true] {
            let dir = tempdir().unwrap();
            let installed = if is_conflict { &conflict } else { &original };
            let budget = serde_json::to_vec(&original).unwrap().len()
                + 1
                + if is_conflict {
                    serde_json::to_vec(&conflict).unwrap().len() + 1
                } else {
                    0
                }
                + if room_for_next {
                    serde_json::to_vec(&next).unwrap().len() + 1
                } else {
                    0
                };
            let store = storage::WikiSpool::open(dir.path(), budget).unwrap();
            let mut state = store.read_checkpoint().unwrap();
            store
                .bind_source(&mut state, Some(&context().source_origin))
                .unwrap();
            if is_conflict {
                store.persist_document(&original).unwrap();
            }
            let target = if is_conflict {
                dir.path().join("conflicts").join(format!(
                    "{}-{}.json",
                    document_storage_key(&original),
                    normalize::sha256(b"B")
                ))
            } else {
                dir.path()
                    .join("documents")
                    .join(format!("{}.json", document_storage_key(&original)))
            };
            let (_, attempts) = storage::with_parent_sync_failures(&target, 1, || {
                let error = store.persist_document(installed).unwrap_err().to_string();
                assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
                assert!(!error.contains("Konflikt erhalten"));
                assert_eq!(
                    fs::read(&target).unwrap(),
                    serde_json::to_vec(installed).unwrap()
                );
                let retry = store.persist_document(installed);
                if is_conflict {
                    assert!(retry.unwrap_err().to_string().contains("Konflikt erhalten"));
                } else {
                    retry.unwrap();
                }
            });
            assert_eq!(attempts, 2);
            let before = spool_snapshot(dir.path());
            if room_for_next {
                store.persist_document(&next).unwrap();
            } else {
                assert!(store
                    .persist_document(&next)
                    .unwrap_err()
                    .to_string()
                    .contains("Wiki-Gesamtgrenze erreicht"));
                assert_eq!(before, spool_snapshot(dir.path()));
            }
            assert_eq!(stored_record_bytes(dir.path()), budget);
            let full = spool_snapshot(dir.path());
            assert!(store
                .persist_document(&document(3, "D"))
                .unwrap_err()
                .to_string()
                .contains("Wiki-Gesamtgrenze erreicht"));
            assert_eq!(full, spool_snapshot(dir.path()));
            drop(store);
            assert!(storage::WikiSpool::open(dir.path(), budget - 1).is_err());
            drop(storage::WikiSpool::open(dir.path(), budget).unwrap());
        }
    }
}

#[test]
fn same_spool_provenance_replacement_sync_failure_retains_exact_shared_budget() {
    let original = normalize_api_response(
        &capture(page(1, 0, 101, "A")),
        &context(),
        &options().observed_at,
    )
    .unwrap()
    .documents
    .remove(0);
    for is_conflict in [false, true] {
        let dir = tempdir().unwrap();
        let mut base = original.clone();
        if is_conflict {
            base["content"] = json!("B");
            base["content_sha256"] = json!(normalize::sha256(b"B"));
        }
        let key = if is_conflict {
            format!(
                "{}-{}",
                document_storage_key(&original),
                normalize::sha256(b"B")
            )
        } else {
            document_storage_key(&original)
        };
        let target = dir.path().join("provenance").join(format!("{key}.json"));
        let mut first = base.clone();
        first["metadata"]["revision_author"] = json!("First additional author");
        {
            let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
            let mut state = store.read_checkpoint().unwrap();
            store
                .bind_source(&mut state, Some(&context().source_origin))
                .unwrap();
            store.persist_document(&original).unwrap();
            if is_conflict {
                assert!(store
                    .persist_document(&base)
                    .unwrap_err()
                    .to_string()
                    .contains("Konflikt erhalten"));
            }
            let result = store.persist_document(&first);
            if is_conflict {
                assert!(result
                    .unwrap_err()
                    .to_string()
                    .contains("Konflikt erhalten"));
            } else {
                result.unwrap();
            }
        }
        let originals = spool_snapshot(&dir.path().join("documents"));
        let conflicts = if is_conflict {
            Some(spool_snapshot(&dir.path().join("conflicts")))
        } else {
            None
        };
        let mut second = base.clone();
        second["metadata"]["revision_author"] =
            json!("Second additional author with a different claim");
        second["license"]["redistribution_allowed"] = json!(true);
        second["observed_at"] = json!("2026-10-04T01:00:00Z");
        let mut expected: Vec<Value> = serde_json::from_slice(&fs::read(&target).unwrap()).unwrap();
        assert_eq!(expected, vec![supplement_entry(&base, &first)]);
        expected.push(supplement_entry(&base, &second));
        let expected_bytes = serde_json::to_vec(&expected).unwrap();
        let budget = stored_record_bytes(dir.path()) - fs::read(&target).unwrap().len()
            + expected_bytes.len();
        let store = storage::WikiSpool::open(dir.path(), budget).unwrap();
        let mut state = store.read_checkpoint().unwrap();
        store
            .bind_source(&mut state, Some(&context().source_origin))
            .unwrap();
        let (_, attempts) = storage::with_parent_sync_failures_after(&target, 1, 2, || {
            let error = store.persist_document(&second).unwrap_err().to_string();
            assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
            assert!(!error.contains("Konflikt erhalten"));
            assert_eq!(fs::read(&target).unwrap(), expected_bytes);
            let before = spool_snapshot(dir.path());
            let error = store.persist_document(&base).unwrap_err().to_string();
            assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
            assert!(!error.contains("Konflikt erhalten"));
            assert_eq!(before, spool_snapshot(dir.path()));
            second["observed_at"] = json!("2026-10-05T01:00:00Z");
            let retry = store.persist_document(&second);
            if is_conflict {
                assert!(retry.unwrap_err().to_string().contains("Konflikt erhalten"));
            } else {
                retry.unwrap();
            }
        });
        assert_eq!(attempts, 4);
        assert_eq!(stored_record_bytes(dir.path()), budget);
        assert_eq!(fs::read(&target).unwrap(), expected_bytes);
        let full = spool_snapshot(dir.path());
        let mut third = second.clone();
        third["metadata"]["revision_author"] = json!("Third additional author");
        assert!(store
            .persist_document(&third)
            .unwrap_err()
            .to_string()
            .contains("Wiki-Gesamtgrenze für Herkunftsergänzung erreicht"));
        let mut next = original.clone();
        next["revision"] = json!("102");
        assert!(store
            .persist_document(&next)
            .unwrap_err()
            .to_string()
            .contains("Wiki-Gesamtgrenze erreicht"));
        assert_eq!(full, spool_snapshot(dir.path()));
        assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
        if let Some(conflicts) = conflicts {
            assert_eq!(conflicts, spool_snapshot(&dir.path().join("conflicts")));
        }
        drop(store);
        assert!(storage::WikiSpool::open(dir.path(), budget - 1).is_err());
        drop(storage::WikiSpool::open(dir.path(), budget).unwrap());
    }
}

#[test]
fn conflict_provenance_final_sync_is_retried_before_public_retention_or_report() {
    let dir = tempdir().unwrap();
    let report = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "A")),
        &context(),
        &options(),
    )
    .unwrap();
    let original = published_document(&report);
    assert!(write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "B")),
        &context(),
        &options()
    )
    .unwrap_err()
    .to_string()
    .contains("Konflikt erhalten"));
    let target = dir.path().join("provenance").join(format!(
        "{}-{}.json",
        document_storage_key(&original),
        normalize::sha256(b"B")
    ));
    let xml = export("deadlock.wiki", &xml_page(101, "<text>B</text>"));
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    let mut later = options();
    later.observed_at = "2026-10-04T01:00:00Z".into();
    let publication = fs::read(&report.documents_path).unwrap();
    let (_, attempts) = storage::with_parent_sync_failures(&target, 2, || {
        let error = write_wiki_export_capture(dir.path(), &xml, &ctx, &later)
            .unwrap_err()
            .to_string();
        assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
        assert!(!error.contains("Konflikt erhalten"));
        assert!(target.is_file());
        let saved = spool_snapshot(dir.path());
        let error = read_wiki_inventory_report(dir.path(), options().max_total_bytes)
            .unwrap_err()
            .to_string();
        assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
        assert_eq!(saved, spool_snapshot(dir.path()));
        later.observed_at = "2026-10-05T01:00:00Z".into();
        assert!(write_wiki_export_capture(dir.path(), &xml, &ctx, &later)
            .unwrap_err()
            .to_string()
            .contains("Konflikt erhalten"));
        assert_eq!(saved, spool_snapshot(dir.path()));
        let reopened = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
        assert_eq!(publication, fs::read(reopened.documents_path).unwrap());
    });
    assert_eq!(attempts, 5);
    let entries: Value = serde_json::from_slice(&fs::read(&target).unwrap()).unwrap();
    assert_eq!(entries.as_array().unwrap().len(), 1);
    assert_eq!(entries[0]["observed_at"], "2026-10-04T01:00:00Z");
}

#[test]
fn provenance_final_parent_sync_is_retried_for_api_and_xml() {
    for api in [false, true] {
        let dir = tempdir().unwrap();
        let first = write_wiki_api_capture(
            dir.path(),
            &capture(page(1, 0, 101, "A")),
            &context(),
            &options(),
        )
        .unwrap();
        let original = published_document(&first);
        let originals = spool_snapshot(&dir.path().join("documents"));
        let target = dir
            .path()
            .join("provenance")
            .join(format!("{}.json", document_storage_key(&original)));
        let (report, attempts) = storage::with_parent_sync_failures(&target, 2, || {
            let error = write_authored_sync_capture(dir.path(), api, "A", "2026-10-04T01:00:00Z")
                .unwrap_err();
            assert!(error
                .to_string()
                .contains("Injizierter abschließender Elternsyncfehler"));
            let entries: Value = serde_json::from_slice(&fs::read(&target).unwrap()).unwrap();
            assert_eq!(entries.as_array().unwrap().len(), 1);
            assert_eq!(entries[0]["assertion"]["revision_author"], "Actual author");
            assert_eq!(entries[0]["observed_at"], "2026-10-04T01:00:00Z");
            let saved = spool_snapshot(dir.path());
            let error = write_authored_sync_capture(dir.path(), api, "A", "2026-10-05T01:00:00Z")
                .unwrap_err();
            assert!(error
                .to_string()
                .contains("Injizierter abschließender Elternsyncfehler"));
            assert_eq!(saved, spool_snapshot(dir.path()));
            let report =
                write_authored_sync_capture(dir.path(), api, "A", "2026-10-06T01:00:00Z").unwrap();
            assert_eq!(
                fs::read(&target).unwrap(),
                serde_json::to_vec(&entries).unwrap()
            );
            report
        });
        assert_eq!(attempts, 4);
        let document = published_document(&report);
        assert_eq!(document["content"], original["content"]);
        assert_eq!(document["observed_at"], original["observed_at"]);
        assert_eq!(document["metadata"]["revision_author"], "Actual author");
        assert_eq!(document["license"]["name"], original["license"]["name"]);
        assert_eq!(document["license"]["redistribution_allowed"], false);
        assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
    }
}

#[test]
fn conflict_final_parent_sync_is_retried_without_a_false_retained_confirmation() {
    for api in [false, true] {
        let dir = tempdir().unwrap();
        let first =
            write_authored_sync_capture(dir.path(), api, "A", &options().observed_at).unwrap();
        let original = published_document(&first);
        let mut conflict = original.clone();
        conflict["content"] = json!("B");
        conflict["content_sha256"] = json!(normalize::sha256(b"B"));
        let target = dir.path().join("conflicts").join(format!(
            "{}-{}.json",
            document_storage_key(&original),
            normalize::sha256(b"B")
        ));
        {
            let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
            let mut state = store.read_checkpoint().unwrap();
            store
                .bind_source(&mut state, Some(&context().source_origin))
                .unwrap();
            let (_, attempts) = storage::with_parent_sync_failures(&target, 2, || {
                let error = store.persist_document(&conflict).unwrap_err().to_string();
                assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
                assert!(!error.contains("Konflikt erhalten"));
                assert_eq!(
                    fs::read(&target).unwrap(),
                    serde_json::to_vec(&conflict).unwrap()
                );
                let saved = spool_snapshot(dir.path());
                let error = store.persist_document(&conflict).unwrap_err().to_string();
                assert!(error.contains("Injizierter abschließender Elternsyncfehler"));
                assert!(!error.contains("Konflikt erhalten"));
                assert_eq!(saved, spool_snapshot(dir.path()));
                let error = store.persist_document(&conflict).unwrap_err().to_string();
                assert!(error.contains("Konflikt erhalten"));
                assert!(!error.contains("Injizierter abschließender Elternsyncfehler"));
                assert_eq!(saved, spool_snapshot(dir.path()));
            });
            assert_eq!(attempts, 3);
        }
        let resumed =
            write_authored_sync_capture(dir.path(), api, "A", &options().observed_at).unwrap();
        assert_eq!(published_document(&resumed), original);
        assert_eq!(
            fs::read_dir(dir.path().join("conflicts")).unwrap().count(),
            1
        );
    }
}

#[test]
fn original_and_source_binding_final_parent_sync_are_retried_for_api_and_xml() {
    for api in [false, true] {
        for binding in [false, true] {
            let dir = tempdir().unwrap();
            let key = normalize::sha256(
                &serde_json::to_vec(&("wiki:deadlock-wiki:page:1", "101")).unwrap(),
            );
            let target = if binding {
                dir.path().join("source.json")
            } else {
                dir.path().join("documents").join(format!("{key}.json"))
            };
            let (report, attempts) = storage::with_parent_sync_failures(&target, 2, || {
                let error =
                    write_authored_sync_capture(dir.path(), api, "A", &options().observed_at)
                        .unwrap_err();
                assert!(error
                    .to_string()
                    .contains("Injizierter abschließender Elternsyncfehler"));
                assert!(target.is_file());
                assert!(!dir.path().join("checkpoint.json").exists());
                assert!(!dir.path().join("documents.jsonl").exists());
                let saved = spool_snapshot(dir.path());
                let error =
                    write_authored_sync_capture(dir.path(), api, "A", "2026-10-04T01:00:00Z")
                        .unwrap_err();
                assert!(error
                    .to_string()
                    .contains("Injizierter abschließender Elternsyncfehler"));
                assert_eq!(saved, spool_snapshot(dir.path()));
                write_authored_sync_capture(dir.path(), api, "A", "2026-10-05T01:00:00Z").unwrap()
            });
            assert_eq!(attempts, if binding { 3 } else { 5 });
            assert_eq!(report.documents, 1);
            let document = published_document(&report);
            assert_eq!(document["content"], "A");
            if !binding {
                assert_eq!(document["observed_at"], options().observed_at);
            }
        }
    }
}

#[test]
fn publication_retries_final_parent_sync_of_visible_provenance() {
    let dir = tempdir().unwrap();
    let first = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "A")),
        &context(),
        &options(),
    )
    .unwrap();
    let original = published_document(&first);
    let target = dir
        .path()
        .join("provenance")
        .join(format!("{}.json", document_storage_key(&original)));
    let (report, attempts) = storage::with_parent_sync_failures(&target, 2, || {
        let error =
            write_authored_sync_capture(dir.path(), true, "A", "2026-10-04T01:00:00Z").unwrap_err();
        assert!(error
            .to_string()
            .contains("Injizierter abschließender Elternsyncfehler"));
        assert!(target.is_file());
        let publication = fs::read(&first.documents_path).unwrap();
        let error = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap_err();
        assert!(error
            .to_string()
            .contains("Injizierter abschließender Elternsyncfehler"));
        assert_eq!(publication, fs::read(&first.documents_path).unwrap());
        read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap()
    });
    assert_eq!(attempts, 3);
    let document = published_document(&report);
    assert_eq!(document["metadata"]["revision_author"], "Actual author");
    assert_eq!(document["observed_at"], original["observed_at"]);
    assert_eq!(document["content"], original["content"]);
}

#[test]
fn conflict_budget_is_counted_once_in_memory_and_after_reopening() {
    let dir = tempdir().unwrap();
    let document = |text| {
        normalize_api_response(
            &capture(page(1, 0, 101, text)),
            &context(),
            &options().observed_at,
        )
        .unwrap()
        .documents
        .remove(0)
    };
    let original = document("A");
    let conflict = document("B");
    let rejected = document("C");
    let budget = serde_json::to_vec(&original).unwrap().len()
        + serde_json::to_vec(&conflict).unwrap().len()
        + 2;
    let conflict_error = |error: SourcesError| {
        assert!(error.to_string().contains("Konflikt erhalten"));
    };
    {
        let store = storage::WikiSpool::open(dir.path(), budget).unwrap();
        let mut state = store.read_checkpoint().unwrap();
        store
            .bind_source(&mut state, Some(&context().source_origin))
            .unwrap();
        store.persist_document(&original).unwrap();
        conflict_error(store.persist_document(&conflict).unwrap_err());
        let saved = spool_snapshot(dir.path());
        conflict_error(store.persist_document(&conflict).unwrap_err());
        let error = store.persist_document(&rejected).unwrap_err().to_string();
        assert!(error.contains("Konflikt nicht gespeichert"));
        assert!(error.contains("unvollständig"));
        assert_eq!(saved, spool_snapshot(dir.path()));
    }
    // One byte below the actual document + conflict budget must refuse opening.
    assert!(storage::WikiSpool::open(dir.path(), budget - 1).is_err());
    let store = storage::WikiSpool::open(dir.path(), budget).unwrap();
    let mut state = store.read_checkpoint().unwrap();
    store
        .bind_source(&mut state, Some(&context().source_origin))
        .unwrap();
    let saved = spool_snapshot(dir.path());
    conflict_error(store.persist_document(&conflict).unwrap_err());
    assert!(store
        .persist_document(&rejected)
        .unwrap_err()
        .to_string()
        .contains("Konflikt nicht gespeichert"));
    let mut new_revision = original.clone();
    new_revision["revision"] = json!("102");
    assert!(store.persist_document(&new_revision).is_err());
    store.persist_document(&original).unwrap();
    assert_eq!(saved, spool_snapshot(dir.path()));
    assert_eq!(
        fs::read_dir(dir.path().join("conflicts")).unwrap().count(),
        1
    );
    let report = store.publish(&state).unwrap();
    assert_eq!(report.documents, 1);
    assert_eq!(published_document(&report)["content"], "A");
}

#[test]
fn oversized_enriched_conflict_is_rejected_without_poisoning_original_resume() {
    let dir = tempdir().unwrap();
    let ctx = context();
    let oversized_capture = capture(page(1, 0, 101, &"x".repeat(4096)));
    let enriched = normalize_api_response(&oversized_capture, &ctx, &options().observed_at)
        .unwrap()
        .documents
        .remove(0);
    let mut bounded = options();
    bounded.max_response_bytes = serde_json::to_vec(&oversized_capture).unwrap().len();
    bounded.max_total_bytes = serde_json::to_vec(&enriched).unwrap().len() - 1;
    assert!(bounded.max_response_bytes < bounded.max_total_bytes);
    let original_capture = capture(page(1, 0, 101, "A"));
    let first = write_wiki_api_capture(dir.path(), &original_capture, &ctx, &bounded).unwrap();
    let original = published_document(&first);
    let before = spool_snapshot(dir.path());
    let error = write_wiki_api_capture(dir.path(), &oversized_capture, &ctx, &bounded)
        .unwrap_err()
        .to_string();
    assert!(error.contains("Konflikt nicht gespeichert"));
    assert!(!error.contains("Konflikt erhalten"));
    assert_eq!(before, spool_snapshot(dir.path()));
    assert!(!dir.path().join("conflicts").exists());
    let resumed = write_wiki_api_capture(dir.path(), &original_capture, &ctx, &bounded).unwrap();
    assert_eq!(resumed.documents, 1);
    assert_eq!(published_document(&resumed), original);
    let reopened = read_wiki_inventory_report(dir.path(), bounded.max_total_bytes).unwrap();
    assert_eq!(published_document(&reopened), original);
}

#[test]
fn directory_durability_syncs_leaf_and_ancestors_and_propagates_failure() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("new-spool");
    let leaf = root.join("provenance");
    let mut attempted = Vec::new();
    let error = storage::create_dir_all_with_sync(&leaf, |directory| {
        attempted.push(directory.to_path_buf());
        if directory == root {
            return Err(SourcesError::invalid_input(
                "injected directory sync failure",
            ));
        }
        fs::File::open(directory)?.sync_all()?;
        Ok(())
    })
    .unwrap_err();
    assert!(error
        .to_string()
        .contains("injected directory sync failure"));
    assert_eq!(attempted, vec![leaf.clone(), root]);
    assert!(leaf.is_dir());
    // Retry must sync already created entries too. This checks syscall routing,
    // not persistence under a real power loss or kernel crash.
    let mut synced = Vec::new();
    storage::create_dir_all_with_sync(&leaf, |directory| {
        fs::File::open(directory)?.sync_all()?;
        synced.push(directory.to_path_buf());
        Ok(())
    })
    .unwrap();
    assert_eq!(
        synced,
        leaf.ancestors().map(Path::to_path_buf).collect::<Vec<_>>()
    );
}

#[test]
fn new_spool_and_first_supplement_and_conflict_survive_normal_reopen_without_checkpoint() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("new").join("spool");
    let original = normalize_api_response(
        &capture(page(1, 0, 101, "A")),
        &context(),
        &options().observed_at,
    )
    .unwrap()
    .documents
    .remove(0);
    let mut supplement = original.clone();
    supplement["metadata"]["revision_author"] = json!("Additional author");
    let mut conflict = original.clone();
    conflict["content"] = json!("B");
    conflict["content_sha256"] = json!(normalize::sha256(b"B"));
    {
        let store = storage::WikiSpool::open(&root, options().max_total_bytes).unwrap();
        let mut state = store.read_checkpoint().unwrap();
        store
            .bind_source(&mut state, Some(&context().source_origin))
            .unwrap();
        store.persist_document(&original).unwrap();
        let originals = spool_snapshot(&root.join("documents"));
        store.persist_document(&supplement).unwrap();
        assert_eq!(originals, spool_snapshot(&root.join("documents")));
        assert!(store
            .persist_document(&conflict)
            .unwrap_err()
            .to_string()
            .contains("Konflikt erhalten"));
        assert!(!root.join("checkpoint.json").exists());
    }
    let before = spool_snapshot(&root);
    let store = storage::WikiSpool::open(&root, options().max_total_bytes).unwrap();
    let mut state = store.read_checkpoint().unwrap();
    store
        .bind_source(&mut state, Some(&context().source_origin))
        .unwrap();
    store.persist_document(&supplement).unwrap();
    assert!(store.persist_document(&conflict).is_err());
    assert_eq!(before, spool_snapshot(&root));
    let report = store.publish(&state).unwrap();
    let published = published_document(&report);
    assert_eq!(published["content"], "A");
    assert_eq!(published["observed_at"], original["observed_at"]);
    assert_eq!(
        published["metadata"]["revision_author"],
        "Additional author"
    );
    assert_eq!(fs::read_dir(root.join("conflicts")).unwrap().count(), 1);
    assert_eq!(fs::read_dir(root.join("provenance")).unwrap().count(), 1);
}

#[test]
fn failed_first_offline_batch_binds_source_before_its_first_document() {
    for api in [false, true] {
        let dir = tempdir().unwrap();
        let xml = export(
            "deadlocked.wiki",
            &format!(
                "{}{}",
                xml_page(101, "<text>A</text>"),
                xml_page(101, "<text>B</text>")
            ),
        );
        let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        let error = if api {
            write_wiki_api_capture(
                dir.path(),
                &json!({"query": {"pages": [page(1, 0, 101, "A"), page(1, 0, 101, "B")]}}),
                &ctx,
                &options(),
            )
            .unwrap_err()
        } else {
            write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap_err()
        };
        assert!(error.to_string().contains("Konflikt"));
        assert!(!dir.path().join("checkpoint.json").exists());
        let binding: Value =
            serde_json::from_slice(&fs::read(dir.path().join("source.json")).unwrap()).unwrap();
        assert_eq!(binding["source_origin"], "https://deadlocked.wiki");
        let before = spool_snapshot(dir.path());
        assert!(write_wiki_api_capture(
            dir.path(),
            &capture(page(2, 0, 102, "C")),
            &context(),
            &options()
        )
        .is_err());
        let official = export("deadlock.wiki", &xml_page(103, "<text>C</text>"));
        assert!(write_wiki_export_capture(dir.path(), &official, &context(), &options()).is_err());
        assert!(
            collect_wiki_inventory_with_transport(dir.path(), &options(), |_| panic!(
                "orphaned foreign spool reached network"
            ))
            .is_err()
        );
        assert_eq!(before, spool_snapshot(dir.path()));
        let recovered =
            write_wiki_api_capture(dir.path(), &capture(page(1, 0, 101, "A")), &ctx, &options())
                .unwrap();
        assert_eq!(recovered.source_id, "deadlocked-wiki");
        assert_eq!(recovered.documents, 1);
    }
}

#[test]
fn legacy_orphans_and_interrupted_writes_are_checked_without_a_checkpoint_default() {
    for legacy in [false, true] {
        let dir = tempdir().unwrap();
        let xml = export("deadlockwiki.org", &xml_page(101, "<text>Original</text>"));
        let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        {
            let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
            let mut state = store.read_checkpoint().unwrap();
            store
                .bind_source(&mut state, Some(&ctx.source_origin))
                .unwrap();
            let document = normalize_mediawiki_export(&xml, &ctx, &options().observed_at)
                .unwrap()
                .documents
                .remove(0);
            store.persist_document(&document).unwrap();
            // Simulate interruption before checkpoint and publication.
        }
        if legacy {
            fs::remove_file(dir.path().join("source.json")).unwrap();
        }
        let before = spool_snapshot(dir.path());
        assert!(
            collect_wiki_inventory_with_transport(dir.path(), &options(), |_| panic!(
                "legacy orphan reached network"
            ))
            .is_err()
        );
        assert!(write_wiki_api_capture(
            dir.path(),
            &capture(page(2, 0, 102, "wrong")),
            &context(),
            &options()
        )
        .is_err());
        assert_eq!(before, spool_snapshot(dir.path()));
        let report = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
        assert_eq!(report.source_id, "deadlockwiki-org");
        assert_eq!(published_document(&report)["content"], "Original");
    }
}

#[test]
fn mixed_legacy_orphans_fail_before_binding_or_writing() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("documents")).unwrap();
    for (index, domain) in ["deadlock.wiki", "deadlocked.wiki"].iter().enumerate() {
        let xml = export(domain, &xml_page(101, "<text>Original</text>"));
        let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        let doc = normalize_mediawiki_export(&xml, &ctx, &options().observed_at)
            .unwrap()
            .documents
            .remove(0);
        fs::write(
            dir.path().join("documents").join(format!("{index}.json")),
            serde_json::to_vec(&doc).unwrap(),
        )
        .unwrap();
    }
    let before = spool_snapshot(dir.path());
    assert!(write_wiki_api_capture(
        dir.path(),
        &capture(page(3, 0, 103, "new")),
        &context(),
        &options()
    )
    .is_err());
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| panic!(
            "mixed sources reached network"
        ))
        .is_err()
    );
    assert_eq!(before, spool_snapshot(dir.path()));
    assert!(!dir.path().join("source.json").exists());
}

#[test]
fn api_then_xml_enriches_durable_authorship_without_changing_original_revision() {
    let dir = tempdir().unwrap();
    let first = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "Original & exact")),
        &context(),
        &options(),
    )
    .unwrap();
    let original = published_document(&first);
    let originals = spool_snapshot(&dir.path().join("documents"));
    let xml = export(
        "deadlock.wiki",
        &xml_page(101, "<text>Original &amp; exact</text>"),
    );
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    let mut later = options();
    later.observed_at = "2026-10-04T01:00:00Z".into();
    let report = write_wiki_export_capture(dir.path(), &xml, &ctx, &later).unwrap();
    let document = published_document(&report);
    assert_eq!(document["content"], original["content"]);
    assert_eq!(document["content_sha256"], original["content_sha256"]);
    assert_eq!(document["observed_at"], original["observed_at"]);
    assert_eq!(document["revision"], "101");
    assert_eq!(document["metadata"]["revision_author"], "Actual author");
    assert_eq!(document["metadata"]["revision_contributor"]["id"], "9");
    assert_eq!(document["license"]["name"], original["license"]["name"]);
    assert_eq!(document["license"]["url"], original["license"]["url"]);
    assert_eq!(document["license"]["redistribution_allowed"], false);
    assert!(document["license"]["attribution"]
        .as_str()
        .unwrap()
        .contains("Actual author"));
    assert!(document["license"]["attribution"]
        .as_str()
        .unwrap()
        .contains(original["license"]["attribution"].as_str().unwrap()));
    assert!(document["metadata"]["provenance_original"]["revision_author"].is_null());
    assert_eq!(
        document["metadata"]["provenance_supplements"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        document["metadata"]["provenance_supplements"][0]["observed_at"],
        later.observed_at
    );
    assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
    let provenance = spool_snapshot(&dir.path().join("provenance"));
    let bytes = fs::read(&report.documents_path).unwrap();
    later.observed_at = "2026-10-05T01:00:00Z".into();
    let repeated = write_wiki_export_capture(dir.path(), &xml, &ctx, &later).unwrap();
    assert_eq!(bytes, fs::read(&repeated.documents_path).unwrap());
    assert_eq!(provenance, spool_snapshot(&dir.path().join("provenance")));
    let reopened = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    assert_eq!(bytes, fs::read(reopened.documents_path).unwrap());
}

#[test]
fn contradictory_authorship_is_preserved_without_selecting_a_winner_or_granting_rights() {
    let dir = tempdir().unwrap();
    let mut first = page(1, 0, 101, "Original");
    first["revisions"][0]["user"] = json!("First author");
    write_wiki_api_capture(dir.path(), &capture(first), &context(), &options()).unwrap();
    let originals = spool_snapshot(&dir.path().join("documents"));
    let xml = export("deadlock.wiki", &xml_page(101, "<text>Original</text>"));
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    let report = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
    let doc = published_document(&report);
    assert_eq!(doc["metadata"]["revision_author"], "First author");
    assert!(doc["metadata"]["revision_contributor"].is_null());
    assert_eq!(
        doc["metadata"]["provenance_conflicts"]["revision_author"],
        true
    );
    assert_eq!(
        doc["metadata"]["provenance_supplements"][0]["assertion"]["revision_author"],
        "Actual author"
    );
    assert_eq!(doc["license"]["redistribution_allowed"], false);
    assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
    let bytes = fs::read(&report.documents_path).unwrap();
    let repeated = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
    assert_eq!(bytes, fs::read(repeated.documents_path).unwrap());
}

#[test]
fn interrupted_provenance_step_is_recoverable_and_cannot_upgrade_original_rights() {
    let dir = tempdir().unwrap();
    let first = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "Original")),
        &context(),
        &options(),
    )
    .unwrap();
    let original = published_document(&first);
    let originals = spool_snapshot(&dir.path().join("documents"));
    let xml = export("deadlock.wiki", &xml_page(101, "<text>Original</text>"));
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    let mut incoming = normalize_mediawiki_export(&xml, &ctx, "2026-10-04T01:00:00Z")
        .unwrap()
        .documents
        .remove(0);
    incoming["license"]["name"] = json!("Later claim of unrestricted rights");
    incoming["license"]["redistribution_allowed"] = json!(true);
    {
        let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
        let mut state = store.read_checkpoint().unwrap();
        store
            .bind_source(&mut state, Some(&ctx.source_origin))
            .unwrap();
        store.persist_document(&incoming).unwrap();
        // Stop immediately after the durable supplement, before publication/checkpoint.
    }
    let report = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    let document = published_document(&report);
    assert_eq!(document["metadata"]["revision_author"], "Actual author");
    assert_eq!(document["license"]["name"], original["license"]["name"]);
    assert_eq!(document["license"]["redistribution_allowed"], false);
    assert_eq!(
        document["metadata"]["provenance_supplements"][0]["assertion"]["license"]
            ["redistribution_allowed"],
        true
    );
    assert_eq!(document["observed_at"], original["observed_at"]);
    assert_eq!(originals, spool_snapshot(&dir.path().join("documents")));
    let before = spool_snapshot(&dir.path().join("provenance"));
    {
        let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
        let mut state = store.read_checkpoint().unwrap();
        store
            .bind_source(&mut state, Some(&ctx.source_origin))
            .unwrap();
        incoming["observed_at"] = json!("2026-10-05T01:00:00Z");
        store.persist_document(&incoming).unwrap();
    }
    assert_eq!(before, spool_snapshot(&dir.path().join("provenance")));
}

#[test]
fn unbound_spool_refuses_direct_document_and_checkpoint_effects() {
    let dir = tempdir().unwrap();
    let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
    let document = normalize_api_response(
        &capture(page(1, 0, 101, "Original")),
        &context(),
        &options().observed_at,
    )
    .unwrap()
    .documents
    .remove(0);
    assert!(store.persist_document(&document).is_err());
    assert!(store.write_checkpoint(&Checkpoint::default()).is_err());
    assert_eq!(
        fs::read_dir(dir.path().join("documents")).unwrap().count(),
        0
    );
    assert!(!dir.path().join("checkpoint.json").exists());
}

#[test]
fn missing_and_hidden_revisions_select_latest_identity_independently_of_readable_text() {
    for hidden in [false, true] {
        for reversed in [false, true] {
            let mut payload = page(1, 0, 101, "old original text");
            let old = payload["revisions"][0].clone();
            let newer = if hidden {
                json!({"revid": 102, "slots": {"main": {"texthidden": true, "content": "never expose"}}})
            } else {
                json!({"revid": 102})
            };
            payload["revisions"] = if reversed {
                json!([newer, old])
            } else {
                json!([old, newer])
            };
            let out = normalize_api_response(&capture(payload), &context(), &options().observed_at)
                .unwrap();
            assert_eq!(out.documents.len(), 1);
            assert_eq!(out.documents[0]["revision"], "101");
            assert_eq!(out.documents[0]["content"], "old original text");
            assert_eq!(out.pages[0].revision.as_deref(), Some("102"));
            assert_eq!(out.pages[0].available_revision.as_deref(), Some("101"));
            assert!(!out.pages[0].content_available);
        }
    }
    let out = normalize_api_response(
        &capture(json!({"pageid": 1, "ns": 0, "title": "Example",
        "revisions": [{"revid": 102}]})),
        &context(),
        &options().observed_at,
    )
    .unwrap();
    assert_eq!(out.pages[0].revision.as_deref(), Some("102"));
    assert!(out.pages[0].available_revision.is_none());
    assert!(out.documents.is_empty());
    let invalid = capture(json!({"pageid": 1, "ns": 0, "title": "Example",
        "revisions": [{"revid": "invalid", "texthidden": true}]}));
    assert!(normalize_api_response(&invalid, &context(), &options().observed_at).is_err());
}

#[test]
fn rendered_extract_is_not_available_revision_text() {
    let mut payload = page(1, 0, 102, "unused");
    payload["revisions"][0]["slots"]["main"]
        .as_object_mut()
        .unwrap()
        .remove("*");
    payload["extract"] = json!("unversioned rendering");
    let out =
        normalize_api_response(&capture(payload), &context(), &options().observed_at).unwrap();
    assert_eq!(out.documents.len(), 1);
    assert_eq!(out.pages[0].revision.as_deref(), Some("102"));
    assert!(!out.pages[0].content_available);
    assert!(out.pages[0].available_revision.is_none());
}

#[test]
fn newer_then_older_captures_do_not_roll_back_checkpoint_or_inventory() {
    for hidden_latest in [false, true] {
        let dir = tempdir().unwrap();
        let mut latest = page(1, 0, 102, "new original text");
        if hidden_latest {
            latest["revisions"][0]["slots"]["main"]["texthidden"] = json!(true);
        }
        write_wiki_api_capture(dir.path(), &capture(latest), &context(), &options()).unwrap();
        let report = write_wiki_api_capture(
            dir.path(),
            &capture(page(1, 0, 101, "old original text")),
            &context(),
            &options(),
        )
        .unwrap();
        let latest = inventory_page(&report);
        assert_eq!(latest["revision"], "102");
        assert_eq!(latest["content_available"], !hidden_latest);
        assert_eq!(
            latest["available_revision"],
            if hidden_latest { "101" } else { "102" }
        );
        assert_eq!(report.documents, if hidden_latest { 1 } else { 2 });
        let store = storage::WikiSpool::open(dir.path(), options().max_total_bytes).unwrap();
        let state = store.read_checkpoint().unwrap();
        assert_eq!(
            state.pages["wiki:deadlock-wiki:page:1"].revision.as_deref(),
            Some("102")
        );
    }
}

#[test]
fn network_batches_use_the_same_revision_merge_as_offline_captures() {
    let dir = tempdir().unwrap();
    let mut latest = capture(page(1, 0, 102, "new"));
    latest["continue"] = json!({"continue": "||", "gapcontinue": "next"});
    let mut responses = VecDeque::from(vec![
        site(),
        latest,
        capture(page(1, 0, 101, "old")),
        json!({"batchcomplete": true}),
    ]);
    let report = collect_wiki_inventory_with_transport(dir.path(), &options(), |_| {
        Ok(responses.pop_front().unwrap())
    })
    .unwrap();
    assert_eq!(report.documents, 2);
    assert_eq!(report.inventory_pages, 1);
    assert_eq!(inventory_page(&report)["revision"], "102");
    assert!(report.content_complete);
}

#[test]
fn repeated_xml_page_blocks_preserve_history_in_both_orders() {
    for hidden in [false, true] {
        let latest = xml_page(
            102,
            if hidden {
                r#"<text deleted="deleted"/>"#
            } else {
                "<text>new original text</text>"
            },
        );
        let old = xml_page(101, "<text>old &amp; original text</text>");
        for reversed in [false, true] {
            let blocks = if reversed {
                format!("{old}{latest}")
            } else {
                format!("{latest}{old}")
            };
            let xml = export("deadlock.wiki", &blocks);
            let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
            let normalized =
                normalize_mediawiki_export(&xml, &ctx, &options().observed_at).unwrap();
            assert_eq!(normalized.pages.len(), 1);
            assert_eq!(normalized.pages[0].revision.as_deref(), Some("102"));
            assert_eq!(
                normalized.pages[0].available_revision.as_deref(),
                Some(if hidden { "101" } else { "102" })
            );
            assert_eq!(normalized.pages[0].content_available, !hidden);
            let dir = tempdir().unwrap();
            let report = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
            assert_eq!(report.documents, if hidden { 1 } else { 2 });
            assert_eq!(inventory_page(&report)["revision"], "102");
            let bytes = fs::read(&report.documents_path).unwrap();
            let again = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
            assert_eq!(bytes, fs::read(&again.documents_path).unwrap());
            assert!(!again.inventory_complete);
        }
    }
}

#[test]
fn siteinfo_article_paths_are_used_for_offline_api_provenance() {
    for path in ["/wiki/$1", "/mw/index.php?title=$1"] {
        let mut site = site();
        site["query"]["general"]["server"] = json!("https://deadlockwiki.org");
        site["query"]["general"]["articlepath"] = json!(path);
        let ctx = WikiSourceContext::from_siteinfo(&site).unwrap();
        let out = normalize_api_response(
            &capture(page(1, 0, 101, "original")),
            &ctx,
            &options().observed_at,
        )
        .unwrap();
        let expected = format!("https://deadlockwiki.org{}", path.trim_end_matches("$1"));
        for address in [
            &out.documents[0]["source_locator"],
            &out.documents[0]["metadata"]["revision_url"],
            &out.documents[0]["metadata"]["attribution_url"],
        ] {
            assert!(address.as_str().unwrap().starts_with(&expected));
        }
        assert_eq!(out.documents[0]["source_id"], "deadlockwiki-org");
    }
    // Even a compatible legacy official context must not erase the XML article path.
    let xml = export("deadlock.wiki", &xml_page(101, "<text>original</text>"));
    let out = normalize_mediawiki_export(&xml, &context(), &options().observed_at).unwrap();
    assert!(out.documents[0]["source_locator"]
        .as_str()
        .unwrap()
        .starts_with("https://deadlock.wiki/wiki/"));
}

#[test]
fn xml_query_style_article_path_and_original_http_scheme_are_preserved() {
    let xml = export("deadlockwiki.org", &xml_page(101, "<text>original</text>")).replace(
        "https://deadlockwiki.org/wiki/Main_Page",
        "http://deadlockwiki.org/mw/index.php?title=Main_Page",
    );
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    assert_eq!(ctx.source_origin, "http://deadlockwiki.org");
    let out = normalize_mediawiki_export(&xml, &ctx, &options().observed_at).unwrap();
    assert_eq!(
        out.documents[0]["source_locator"],
        "http://deadlockwiki.org/mw/index.php?title=Example&curid=1"
    );
    assert_eq!(
        out.documents[0]["metadata"]["revision_url"],
        "http://deadlockwiki.org/mw/index.php?title=Example&curid=1&oldid=101"
    );
}

#[test]
fn historical_domains_with_identical_ids_remain_separate_sources() {
    let mut ids = BTreeSet::new();
    for (domain, source) in [
        ("deadlock.wiki", "deadlock-wiki"),
        ("deadlocked.wiki", "deadlocked-wiki"),
        ("deadlockwiki.org", "deadlockwiki-org"),
        ("deadlock.miraheze.org", "deadlock-miraheze-org"),
        ("deadlockwiki.miraheze.org", "deadlockwiki-miraheze-org"),
    ] {
        let xml = export(domain, &xml_page(101, "<text>original &amp; exact</text>"));
        let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        assert_eq!(ctx.source_id().unwrap(), source);
        assert_eq!(ctx.license["name"], "unverified");
        assert!(ctx.license["url"].is_null());
        let out = normalize_mediawiki_export(&xml, &ctx, &options().observed_at).unwrap();
        let document = &out.documents[0];
        assert_eq!(document["source_id"], source);
        assert_eq!(document["document_id"], format!("wiki:{source}:page:1"));
        assert!(ids.insert(document["document_id"].as_str().unwrap().to_string()));
        for locator in [
            &document["source_locator"],
            &document["metadata"]["revision_url"],
            &document["metadata"]["attribution_url"],
        ] {
            assert!(locator
                .as_str()
                .unwrap()
                .starts_with(&format!("https://{domain}/wiki/")));
        }
        assert_eq!(document["content"], "original & exact");
        assert_eq!(
            document["content_sha256"],
            normalize::sha256(b"original & exact")
        );
        assert_eq!(document["metadata"]["revision_contributor"]["id"], "9");
        assert_eq!(document["metadata"]["historical_capture"], true);
        let dir = tempdir().unwrap();
        let report = write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
        assert_eq!(report.source_id, source);
        assert_eq!(
            read_wiki_inventory_report(dir.path(), options().max_total_bytes)
                .unwrap()
                .source_id,
            source
        );
        if domain != "deadlock.wiki" {
            assert!(
                collect_wiki_inventory_with_transport(dir.path(), &options(), |_| panic!(
                    "foreign source caused network access"
                ))
                .is_err()
            );
        }
    }
    assert_eq!(ids.len(), 5);
}

#[test]
fn foreign_context_cannot_relabel_an_export_or_mix_spool_sources() {
    let xml = export(
        "deadlocked.wiki",
        &xml_page(101, "<text>old archive</text>"),
    );
    let ctx = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
    assert!(normalize_mediawiki_export(&xml, &context(), &options().observed_at).is_err());
    let dir = tempdir().unwrap();
    write_wiki_export_capture(dir.path(), &xml, &ctx, &options()).unwrap();
    let before = fs::read(dir.path().join("documents.jsonl")).unwrap();
    assert!(write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "different source")),
        &context(),
        &options()
    )
    .is_err());
    assert_eq!(
        before,
        fs::read(dir.path().join("documents.jsonl")).unwrap()
    );
    let mut foreign_site = site();
    foreign_site["query"]["general"]["server"] = json!("https://deadlocked.wiki");
    let dir = tempdir().unwrap();
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| Ok(foreign_site.clone()))
            .is_err()
    );
    for base in [
        "https://deadlock.wiki.evil.test/wiki/Main_Page",
        "https://deadlock.wiki@evil.test/wiki/Main_Page",
        "https://deadlocked.wiki:443/wiki/Main_Page",
    ] {
        let invalid = xml.replace("https://deadlocked.wiki/wiki/Main_Page", base);
        assert!(WikiSourceContext::from_mediawiki_export(&invalid).is_err());
    }
    let missing = xml.replace("<base>https://deadlocked.wiki/wiki/Main_Page</base>", "");
    assert!(WikiSourceContext::from_mediawiki_export(&missing).is_err());
    assert!(normalize_mediawiki_export(&missing, &ctx, &options().observed_at).is_err());
    let mut missing_site = site();
    missing_site["query"]["general"]
        .as_object_mut()
        .unwrap()
        .remove("server");
    assert!(WikiSourceContext::from_siteinfo(&missing_site).is_err());
}

#[test]
fn checkpoint_reads_are_bounded_before_parsing_and_during_streaming() {
    use std::io::{Cursor, Read};
    let exact = vec![b'x'; 16];
    assert_eq!(
        storage::read_checkpoint_bytes(Cursor::new(&exact), 16).unwrap(),
        exact
    );
    let mut endless = std::io::repeat(b'x');
    assert!(storage::read_checkpoint_bytes(&mut endless, 16).is_err());
    // A counting reader proves a growing/unbounded stream consumes at most limit + 1.
    struct Counter {
        consumed: usize,
    }
    impl Read for Counter {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            buffer.fill(b'x');
            self.consumed += buffer.len();
            Ok(buffer.len())
        }
    }
    let mut counter = Counter { consumed: 0 };
    assert!(storage::read_checkpoint_bytes(&mut counter, 16).is_err());
    assert_eq!(counter.consumed, 17);
    let dir = tempdir().unwrap();
    let file = fs::File::create(dir.path().join("checkpoint.json")).unwrap();
    file.set_len(1024 * 1024 * 1024).unwrap(); // sparse file, not a large memory fixture
    let store = storage::WikiSpool::open(dir.path(), 1024).unwrap();
    assert!(store
        .read_checkpoint()
        .unwrap_err()
        .to_string()
        .contains("Größenlimit"));
}

#[test]
fn old_checkpoint_fields_remain_readable_and_merge_correctly() {
    let mut ctx = serde_json::to_value(context()).unwrap();
    for field in [
        "source_origin",
        "source_article_base",
        "source_siteinfo_hash_representation",
    ] {
        ctx.as_object_mut().unwrap().remove(field);
    }
    let restored: WikiSourceContext = serde_json::from_value(ctx.clone()).unwrap();
    assert_eq!(restored.source_origin, "https://deadlock.wiki");
    let mut previous = serde_json::to_value(
        normalize_api_response(
            &capture(page(1, 0, 102, "new")),
            &context(),
            &options().observed_at,
        )
        .unwrap()
        .pages[0]
            .clone(),
    )
    .unwrap();
    previous
        .as_object_mut()
        .unwrap()
        .remove("available_revision");
    let checkpoint = json!({"version": CONTRACT_VERSION, "source_id": SOURCE_ID, "context": ctx,
        "namespaces": {}, "pages": {"wiki:deadlock-wiki:page:1": previous}, "gaps": [],
        "access_block": null, "scope": "offline_partial_source_documents"});
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("checkpoint.json"),
        serde_json::to_vec(&checkpoint).unwrap(),
    )
    .unwrap();
    let report = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "old")),
        &context(),
        &options(),
    )
    .unwrap();
    assert_eq!(inventory_page(&report)["revision"], "102");
    assert_eq!(inventory_page(&report)["available_revision"], "102");
}

#[test]
fn original_v1_slot_redirect_mapping_and_distinct_observation_times_survive() {
    let mut context = context();
    context.license_observed_at = Some("2026-10-03T03:18:42Z".into());
    context.source_fetched_at = Some("2026-05-02T14:59:46Z".into());
    context.historical_capture = true;
    let mut page = page(
        947,
        0,
        66280,
        "{{Stats}}\n== Update history ==\nOld value 999",
    );
    page["title"] = json!("Stats");
    page["extract"] = json!("Historical rendered value 35%");
    let payload = json!({"batchcomplete": "", "query": {
        "pages": {"947": page}, "redirects": [{"from": "Statistics", "to": "Stats"}]
    }});
    let normalized = normalize_api_response(&payload, &context, "2026-10-03T04:00:00Z").unwrap();
    assert_eq!(normalized.documents.len(), 1);
    let document = &normalized.documents[0];
    assert_eq!(document["document_id"], "wiki:deadlock-wiki:page:947");
    assert_eq!(document["revision"], "66280");
    assert_eq!(
        document["content_sha256"],
        normalize::sha256(document["content"].as_str().unwrap().as_bytes())
    );
    assert_eq!(document["observed_at"], "2026-10-03T04:00:00Z");
    assert_eq!(
        document["metadata"]["source_fetched_at"],
        "2026-05-02T14:59:46Z"
    );
    assert_eq!(
        document["metadata"]["revision_timestamp"],
        "2026-04-28T16:22:50Z"
    );
    assert_eq!(
        document["metadata"]["license_observed_at"],
        "2026-10-03T03:18:42Z"
    );
    assert_eq!(document["metadata"]["historical"], true);
    assert_eq!(
        document["metadata"]["query_redirect_mappings"][0]["from"],
        "Statistics"
    );
    assert_eq!(
        document["metadata"]["rendered_extract"],
        "Historical rendered value 35%"
    );
    assert_eq!(
        document["metadata"]["rendered_template_revisions_pinned"],
        false
    );
    assert!(document["facts"].as_array().unwrap().is_empty());
    assert!(normalized
        .gaps
        .iter()
        .any(|gap| gap.reason == "redirect_source_page_and_revision_unavailable"));
}

#[test]
fn traversal_includes_every_real_namespace_and_resumes_after_a_failed_request() {
    let dir = tempdir().unwrap();
    let mut first = capture(page(1, 0, 101, "first"));
    first["continue"] = json!({"continue": "||", "gapcontinue": "Page 2"});
    let mut responses = VecDeque::from(vec![
        Ok(site()),
        Ok(first),
        Err(SourcesError::invalid_input("Wiki-API-Fehler: maxlag")),
    ]);
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| responses
            .pop_front()
            .unwrap())
        .is_err()
    );
    let partial = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    assert_eq!(partial.documents, 1);
    assert!(!partial.inventory_complete);
    let mut responses = VecDeque::from(vec![
        capture(page(2, 0, 102, "second")),
        json!({"batchcomplete": true}),
    ]);
    let mut requests = Vec::new();
    let report = collect_wiki_inventory_with_transport(dir.path(), &options(), |params| {
        requests.push(params.to_vec());
        Ok(responses.pop_front().unwrap())
    })
    .unwrap();
    assert_eq!(report.documents, 2);
    assert_eq!(report.inventory_pages, 2);
    assert!(report.inventory_complete);
    assert!(report.content_complete);
    assert_eq!(report.namespaces_completed, vec![0, 10]);
    assert!(requests[0].contains(&("gapcontinue".into(), "Page 2".into())));
    assert!(requests[0].contains(&("continue".into(), "||".into())));
    assert!(requests[0].contains(&("gapfilterredir".into(), "all".into())));
    assert!(requests[1].contains(&("gapnamespace".into(), "10".into())));
    let bytes = fs::read(&report.documents_path).unwrap();
    let repeated = collect_wiki_inventory_with_transport(dir.path(), &options(), |_| {
        panic!("completed collection requested network")
    })
    .unwrap();
    assert_eq!(repeated.documents, 2);
    assert_eq!(bytes, fs::read(repeated.documents_path).unwrap());
}

#[test]
fn offline_repetition_keeps_first_observation_and_new_revisions_preserve_history() {
    let dir = tempdir().unwrap();
    let first = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "first")),
        &context(),
        &options(),
    )
    .unwrap();
    let bytes = fs::read(&first.documents_path).unwrap();
    let mut later = options();
    later.observed_at = "2026-10-04T01:00:00Z".into();
    let repeated = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "first")),
        &context(),
        &later,
    )
    .unwrap();
    assert_eq!(repeated.documents, 1);
    assert_eq!(bytes, fs::read(&repeated.documents_path).unwrap());
    assert!(!repeated.inventory_complete);
    let new = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 102, "second")),
        &context(),
        &later,
    )
    .unwrap();
    assert_eq!(new.documents, 2);
    assert_eq!(new.inventory_pages, 1);
    assert!(!new.content_complete);
}

#[test]
fn same_revision_different_content_is_retained_as_conflict_without_overwrite() {
    let dir = tempdir().unwrap();
    let first = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "first")),
        &context(),
        &options(),
    )
    .unwrap();
    let bytes = fs::read(&first.documents_path).unwrap();
    let error = write_wiki_api_capture(
        dir.path(),
        &capture(page(1, 0, 101, "changed")),
        &context(),
        &options(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("Konflikt"));
    assert_eq!(bytes, fs::read(first.documents_path).unwrap());
    assert_eq!(
        fs::read_dir(dir.path().join("conflicts")).unwrap().count(),
        1
    );
}

#[test]
fn source_namespace_names_distinguish_updates_json_data_and_bucket() {
    let mut site = site();
    site["query"]["namespaces"]["3000"] = json!({"id": 3000, "canonical": "Update"});
    site["query"]["namespaces"]["3002"] = json!({"id": 3002, "canonical": "Data"});
    site["query"]["namespaces"]["9592"] = json!({"id": 9592, "canonical": "Bucket"});
    let context = WikiSourceContext::from_siteinfo(&site).unwrap();
    let mut data = page(
        2,
        3002,
        102,
        r#"{"cooldown":{"value":12,"unit":"seconds"},"enabled":true}"#,
    );
    data["revisions"][0]["slots"]["main"]["contentmodel"] = json!("json");
    let normalized =
        normalize_api_response(&capture(data), &context, &options().observed_at).unwrap();
    assert_eq!(normalized.documents[0]["metadata"]["page_kind"], "data");
    assert_eq!(
        normalized.documents[0]["facts"].as_array().unwrap().len(),
        3
    );
    for fact in normalized.documents[0]["facts"].as_array().unwrap() {
        assert_eq!(fact["predicate"], "wiki.data.value");
        assert!(fact["unit"].is_null());
        assert_eq!(fact["qualifiers"]["unit_inferred"], false);
    }
    let update = normalize_api_response(
        &capture(page(3, 3000, 103, "Historical patch")),
        &context,
        &options().observed_at,
    )
    .unwrap();
    assert_eq!(
        update.documents[0]["metadata"]["page_kind"],
        "historical_update"
    );
    assert_eq!(update.documents[0]["metadata"]["historical"], true);
    let mut bucket = page(4, 9592, 104, "[1,2]");
    bucket["revisions"][0]["slots"]["main"]["contentmodel"] = json!("json");
    let bucket =
        normalize_api_response(&capture(bucket), &context, &options().observed_at).unwrap();
    assert_eq!(bucket.documents[0]["metadata"]["page_kind"], "bucket_data");
    assert_eq!(bucket.documents[0]["facts"].as_array().unwrap().len(), 2);
}

#[test]
fn redirects_templates_modules_and_categories_stay_unexpanded() {
    let mut redirect = page(1, 0, 101, "#REDIRECT [[Target]]");
    redirect["redirect"] = json!(true);
    let redirect =
        normalize_api_response(&capture(redirect), &context(), &options().observed_at).unwrap();
    assert_eq!(redirect.documents[0]["metadata"]["page_kind"], "redirect");
    assert_eq!(
        redirect.documents[0]["metadata"]["redirect_target"],
        "Target"
    );
    let normalized = normalize_api_response(
        &capture(page(
            2,
            0,
            102,
            "{{Stats}} {{#invoke:HeroDataArrays|f}} [[Category:Historical]]",
        )),
        &context(),
        &options().observed_at,
    )
    .unwrap();
    let metadata = &normalized.documents[0]["metadata"];
    assert_eq!(metadata["template_dependencies"][0], "Template:Stats");
    assert_eq!(metadata["module_dependencies"][0], "Module:HeroDataArrays");
    assert_eq!(metadata["categories"][0], "Historical");
    assert_eq!(metadata["dependencies_expanded"], false);
    assert_eq!(metadata["historical"], true);
}

#[test]
fn suppressed_content_and_unknown_revisions_remain_explicit() {
    let mut suppressed = page(1, 0, 101, "suppressed secret");
    suppressed["revisions"][0]["slots"]["main"]["texthidden"] = json!(true);
    suppressed["extract"] = json!("must not leak");
    let normalized =
        normalize_api_response(&capture(suppressed), &context(), &options().observed_at).unwrap();
    assert!(normalized.documents.is_empty());
    assert!(!normalized.pages[0].content_available);
    assert_eq!(normalized.pages[0].revision.as_deref(), Some("101"));
    assert!(normalized.pages[0].available_revision.is_none());
    assert_eq!(normalized.gaps[0].reason, "revision_content_suppressed");
    let mut unknown = page(2, 0, 102, "known text, unknown revision");
    unknown["revisions"][0]
        .as_object_mut()
        .unwrap()
        .remove("revid");
    let normalized =
        normalize_api_response(&capture(unknown), &context(), &options().observed_at).unwrap();
    assert!(normalized.documents[0]["revision"]
        .as_str()
        .unwrap()
        .starts_with("unknown:"));
    assert!(normalized
        .gaps
        .iter()
        .any(|gap| gap.reason == "revision_id_unknown"));
    let mut invalid = page(3, 0, 103, "text");
    invalid["revisions"][0]["revid"] = json!("known-but-invalid");
    assert!(normalize_api_response(&capture(invalid), &context(), &options().observed_at).is_err());
}

#[test]
fn xml_export_preserves_each_revision_and_does_not_claim_global_completeness() {
    let xml = r#"<mediawiki xmlns="http://www.mediawiki.org/xml/export-0.11/"><siteinfo><base>https://deadlock.wiki/Main_Page</base></siteinfo><page><title>Hero</title><ns>0</ns><id>1</id><revision><id>101</id><timestamp>2026-04-28T16:22:50Z</timestamp><contributor><username>Contributor</username></contributor><model>wikitext</model><text>Old &amp; preserved</text></revision><revision><id>102</id><timestamp>2026-04-29T16:22:50Z</timestamp><model>wikitext</model><text>New</text></revision></page></mediawiki>"#;
    let dir = tempdir().unwrap();
    let report = write_wiki_export_capture(dir.path(), xml, &context(), &options()).unwrap();
    assert_eq!(report.documents, 2);
    assert_eq!(report.inventory_pages, 1);
    assert!(!report.inventory_complete);
    let bytes = fs::read_to_string(report.documents_path).unwrap();
    assert!(bytes.contains("Old & preserved"));
    assert!(bytes.contains("revision author: Contributor"));
    assert!(normalize_mediawiki_export("<!DOCTYPE mediawiki [<!ENTITY external SYSTEM 'file:///etc/passwd'>]><mediawiki>&external;</mediawiki>", &context(), &options().observed_at).is_err());
}

#[test]
fn disabled_access_missing_license_warnings_and_bad_times_fail_closed() {
    let dir = tempdir().unwrap();
    assert!(collect_wiki_inventory_with_transport(
        dir.path(),
        &WikiInventoryOptions::default(),
        |_| panic!("network called")
    )
    .is_err());
    let mut no_review = options();
    no_review.access_policy_reviewed = false;
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &no_review, |_| panic!("network called"))
            .is_err()
    );
    let mut site = site();
    site["query"].as_object_mut().unwrap().remove("rightsinfo");
    assert!(WikiSourceContext::from_siteinfo(&site).is_err());
    assert!(normalize_api_response(
        &json!({"warnings": {"x": "bad"}}),
        &context(),
        &options().observed_at
    )
    .is_err());
    for invalid in [
        "2026-02-30T00:00:00Z",
        "2026-10-03T24:00:00Z",
        "2026-10-03T00:00:00+00:00",
        "",
        "2026-10-03T00:00:00.Z",
    ] {
        assert!(normalize::validate_observed_at(invalid).is_err());
    }
}

#[test]
fn repeated_or_overriding_continuations_are_rejected_before_advancing() {
    let dir = tempdir().unwrap();
    let mut first = capture(page(1, 0, 101, "first"));
    first["continue"] = json!({"continue": "||", "gapcontinue": "same"});
    let mut second = capture(page(2, 0, 102, "second"));
    second["continue"] = first["continue"].clone();
    let mut responses = VecDeque::from(vec![site(), first, second]);
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| Ok(responses
            .pop_front()
            .unwrap()))
        .is_err()
    );
    let report = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    assert_eq!(report.documents, 1);
    assert!(!report.inventory_complete);
    assert!(continuation_pairs(&json!({"continue": "||", "action": "edit"})).is_err());
}

#[test]
fn explicit_api_access_denial_is_latched_until_a_reviewed_reset() {
    let dir = tempdir().unwrap();
    let mut responses =
        VecDeque::from(vec![site(), json!({"error": {"code": "permissiondenied"}})]);
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| Ok(responses
            .pop_front()
            .unwrap()))
        .is_err()
    );
    let report = read_wiki_inventory_report(dir.path(), options().max_total_bytes).unwrap();
    assert!(report.access_block.is_some());
    assert!(
        collect_wiki_inventory_with_transport(dir.path(), &options(), |_| panic!(
            "access denial retried"
        ))
        .is_err()
    );
}
