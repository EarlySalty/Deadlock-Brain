use brain_contracts::entity_profile::{EntityIdentity, EntityKind};
use dbrain_sources::entity_binding::{bind_document, CatalogEntity};
use dbrain_sources::knowledge_contract::{KnowledgeDocument, KnowledgeFact};
use serde_json::json;

fn catalog() -> Vec<CatalogEntity> {
    [
        (EntityKind::Hero, "hero_test"),
        (EntityKind::Ability, "ability_test"),
        (EntityKind::Item, "upgrade_test"),
    ]
    .into_iter()
    .map(|(kind, id)| CatalogEntity {
        identity: EntityIdentity {
            entity_key: id.into(),
            kind,
            name: id.into(),
            aliases: vec![],
            identity_evidence: vec![format!("brain.entity_aliases:{id}")],
        },
        identifiers: vec![id.into()],
    })
    .collect()
}

fn document(facts: Vec<KnowledgeFact>) -> KnowledgeDocument {
    serde_json::from_value(json!({
        "contract_version":"wiki-spielwissen-v1","source_kind":"game_file","source_id":"test",
        "document_id":"test:document","source_locator":"https://example.org","title":"hero_test",
        "language":"und","revision":"git:test","observed_at":"2026-10-03T00:00:00Z",
        "content_sha256":"0".repeat(64),"content":"","evidence_status":"extracted_value",
        "license":{"name":"unverified","url":null,"attribution":"Quelle","redistribution_allowed":false},
        "metadata":{},"facts":facts
    })).unwrap()
}

fn fact(path: &str, value: serde_json::Value) -> KnowledgeFact {
    serde_json::from_value(json!({"fact_id":path,"subject":"game_file:test","predicate":"file.json_value",
        "value":value,"unit":null,"evidence_status":"extracted_value","source_span":path,
        "qualifiers":{"source_pointer":path,"numeric_representation":"source_numeric_lexeme","condition":"erhalten"}})).unwrap()
}

#[test]
fn all_kinds_bind_without_changing_source_facts_and_repeat_identically() {
    let catalog = catalog();
    let document = document(
        catalog
            .iter()
            .map(|entry| {
                fact(
                    &format!("/{}/Health", entry.identifiers[0]),
                    json!("1.200e+19"),
                )
            })
            .collect(),
    );
    let first = bind_document(&document, &catalog);
    assert_eq!(first.bindings.len(), 3);
    assert!(first.unresolved_fact_ids.is_empty());
    assert_eq!(first.bindings, bind_document(&document, &catalog).bindings);
    for (binding, source) in first.bindings.iter().zip(&document.facts) {
        assert_eq!(binding.fact, *source);
        assert_eq!(binding.revision, document.revision);
        assert_eq!(binding.relative_pointer, "/Health");
    }
}

#[test]
fn ambiguous_ids_and_title_only_matches_remain_open() {
    let mut catalog = catalog();
    catalog[1].identifiers.push("hero_test".into());
    let document = document(vec![
        fact("/hero_test/Health", json!(10)),
        fact("/Health", json!(20)),
    ]);
    let result = bind_document(&document, &catalog);
    assert!(result.bindings.is_empty());
    assert_eq!(result.ambiguous_fact_ids, vec!["/hero_test/Health"]);
    assert_eq!(result.unresolved_fact_ids, vec!["/Health"]);
}

#[test]
fn explicit_class_evidence_binds_unnamed_object_and_keeps_nested_variants() {
    let document = document(vec![
        fact("/0/class_name", json!("ability_test")),
        fact("/0/Variants/1/Damage", json!("3.140")),
    ]);
    let result = bind_document(&document, &catalog());
    assert_eq!(result.bindings.len(), 2);
    assert_eq!(result.bindings[1].relative_pointer, "/Variants/1/Damage");
    assert_eq!(result.bindings[1].fact.qualifiers["condition"], "erhalten");
}

#[test]
fn conflicting_object_identity_is_not_overruled_by_its_path() {
    let document = document(vec![
        fact("/hero_test/class_name", json!("ability_test")),
        fact("/hero_test/Health", json!(10)),
    ]);
    let result = bind_document(&document, &catalog());
    assert!(result.bindings.is_empty());
    assert_eq!(result.ambiguous_fact_ids.len(), 2);
}

#[test]
fn numeric_string_keeps_git_priority_against_wiki_with_different_source_qualifiers() {
    use dbrain_sources::entity_binding::binding_conflicts;
    use dbrain_sources::knowledge_contract::KnowledgeSourceKind;
    let git = document(vec![fact("/hero_test/Health", json!("1.200e+19"))]);
    let mut wiki = document(vec![fact("/hero_test/Health", json!(10))]);
    wiki.source_kind = KnowledgeSourceKind::Wiki;
    wiki.source_id = "wiki".into();
    wiki.facts[0].qualifiers.remove("source_pointer");
    wiki.facts[0]
        .qualifiers
        .insert("json_pointer".into(), json!("/hero_test/Health"));
    wiki.facts[0].qualifiers.remove("numeric_representation");
    let mut bindings = bind_document(&git, &catalog()).bindings;
    bindings.extend(bind_document(&wiki, &catalog()).bindings);
    let conflicts = binding_conflicts(&bindings);
    assert_eq!(conflicts.len(), 1);
    assert!(conflicts[0]
        .preferred_fact_reference
        .as_ref()
        .unwrap()
        .starts_with("test:"));
    assert_eq!(bindings[0].fact.value, "1.200e+19");
    assert_eq!(bindings[1].fact.value, 10);
}

#[test]
fn variant_qualifiers_and_same_source_conflicts_do_not_get_arbitrary_preferences() {
    use dbrain_sources::entity_binding::binding_conflicts;
    let first = document(vec![fact("/hero_test/Health", json!("10"))]);
    let mut second = document(vec![fact("/hero_test/Health", json!("20"))]);
    second.revision = "git:other".into();
    let mut bindings = bind_document(&first, &catalog()).bindings;
    bindings.extend(bind_document(&second, &catalog()).bindings);
    assert!(binding_conflicts(&bindings)[0]
        .preferred_fact_reference
        .is_none());
    bindings[1]
        .fact
        .qualifiers
        .insert("variant".into(), json!("empowered"));
    assert!(binding_conflicts(&bindings).is_empty());
}

#[test]
fn wiki_object_alias_is_evidence_but_numeric_array_offset_is_not() {
    use dbrain_sources::knowledge_contract::KnowledgeSourceKind;
    let mut catalog = catalog();
    catalog[0].identity.aliases.push("Testheld".into());
    catalog[0].identifiers.push("0".into());
    let mut wiki = document(vec![
        fact("/Testheld/Health", json!(10)),
        fact("/0/Health", json!(20)),
    ]);
    wiki.source_kind = KnowledgeSourceKind::Wiki;
    let result = bind_document(&wiki, &catalog);
    assert_eq!(result.bindings.len(), 1);
    assert_eq!(result.unresolved_fact_ids, vec!["/0/Health"]);
}

#[tokio::test]
#[ignore = "Benötigt vorhandenes Infisical-Credential auf FD 5 und echte Eingaben"]
async fn real_catalog_and_git_documents_report_coverage() {
    use dbrain_sources::entity_binding::load_entity_catalog;
    use dbrain_sources::knowledge_contract::{
        validate_knowledge_jsonl_seekable, KnowledgeStreamLimits,
    };
    use std::{
        collections::{BTreeMap, BTreeSet},
        fs::File,
        io::BufReader,
        path::Path,
    };
    let pool = dbrain_sources::core::pg::pg_pool_from_config(
        Path::new("/etc/deadlock-brain/infisical.json"),
        true,
    )
    .await
    .expect("DB-Verbindung");
    let catalog = load_entity_catalog(&pool).await.expect("Entitätskatalog");
    let mut counts = BTreeMap::new();
    for entry in &catalog {
        *counts
            .entry(format!("{:?}", entry.identity.kind))
            .or_insert(0usize) += 1;
    }
    println!("Katalog: {counts:?}");
    let root = "/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/run-CrJzHxm4";
    for file in [
        "deadlock-data-0d46cdecfccf77adec16aac01af6d30173e0ebb8.jsonl",
        "gametracking-4c6431ccdb816d2911bbbaa4335169cc34140386.jsonl",
    ] {
        let input = BufReader::new(File::open(format!("{root}/{file}")).unwrap());
        let mut bound = BTreeMap::new();
        let mut unresolved = 0;
        let mut ambiguous = 0;
        let mut examples = BTreeMap::new();
        let mut entities: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        validate_knowledge_jsonl_seekable(
            input,
            KnowledgeStreamLimits {
                max_line_bytes: 128 * 1024 * 1024,
                max_input_bytes: 2 * 1024 * 1024 * 1024,
                max_document_lines: 100_000,
                max_index_bytes: 64 * 1024 * 1024,
            },
            |located, _, _| {
                let result = bind_document(&located.document, &catalog);
                unresolved += result.unresolved_fact_ids.len();
                ambiguous += result.ambiguous_fact_ids.len();
                for binding in result.bindings {
                    let entry = catalog
                        .iter()
                        .find(|entry| entry.identity.entity_key == binding.entity_key)
                        .unwrap();
                    let kind = format!("{:?}", entry.identity.kind);
                    entities
                        .entry(kind.clone())
                        .or_default()
                        .insert(binding.entity_key.clone());
                    *bound.entry(kind.clone()).or_insert(0usize) += 1;
                    examples.entry(kind).or_insert_with(|| {
                        format!(
                            "{}:{}:{}",
                            entry.identity.name, located.document.document_id, binding.fact.fact_id
                        )
                    });
                }
                Ok(())
            },
        )
        .expect("Streamprüfung");
        let entity_counts: BTreeMap<_, _> = entities
            .into_iter()
            .map(|(kind, keys)| (kind, keys.len()))
            .collect();
        println!("{file}: Entitäten={entity_counts:?}, zugeordnet={bound:?}, offen={unresolved}, mehrdeutig={ambiguous}, Beispiele={examples:?}");
    }
    pool.close().await;
}
