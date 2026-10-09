use brain_contracts::entity_profile::{
    EntityIdentity, EntityKind, PatchValidity, ProfileSourceKind,
};
use brain_storage::entity_profile::{assemble_profile, project_entity_facts, validity_contains};
use dbrain_sources::knowledge_contract::{sha256_content, validate_knowledge_jsonl_str};
use dbrain_sources::knowledge_import::{prepare_validated_knowledge, ImportPolicy};
use serde_json::{json, Value};

fn record() -> brain_contracts::SourceRecordV2 {
    let document = json!({
        "contract_version":"wiki-spielwissen-v1","source_kind":"wiki","source_id":"fixture",
        "document_id":"wiki:fixture:page:123","source_locator":"https://example.org/Test","title":"Test","language":"und","revision":"456",
        "observed_at":"2026-10-03T12:00:00Z","content_sha256":sha256_content("Beleg"),"content":"Beleg","evidence_status":"source_statement",
        "license":{"name":"unverified","url":null,"attribution":"Fixture","redistribution_allowed":false},"metadata":{"unknown":"erhalten"},
        "facts":[{"fact_id":"health","subject":"hero:test","predicate":"health","value":serde_json::from_str::<Value>("1.2345678901234567890123456789e+19").unwrap(),"unit":"hp","evidence_status":"extracted_value","source_span":"Test:health","qualifiers":{"condition":"unknown"}}]
    });
    let input = validate_knowledge_jsonl_str(&document.to_string()).unwrap();
    let policy: ImportPolicy = serde_json::from_value(json!({"sources":{"fixture":{
        "internal_read_allowed":true,"raw_retention_allowed":true,"publication_allowed":false,"provider_egress_allowed":false,"authorization_ref":"fixture-grant","provenance_evidence_ref":"fixture-origin","allowed_scopes":[]
    }}})).unwrap();
    prepare_validated_knowledge(&input, &policy, "fixture-v1")
        .unwrap()
        .records()[0]
        .record
        .clone()
}
fn entity(kind: EntityKind) -> EntityIdentity {
    EntityIdentity {
        entity_key: "test".into(),
        kind,
        name: "Test".into(),
        aliases: vec![],
        identity_evidence: vec!["fixture:identity".into()],
    }
}

fn extracted_fact(
    extension: &str,
    number: &str,
) -> brain_contracts::entity_profile::EntityProfileFact {
    let content = if extension == "json" {
        format!("{{\"health\":{number}}}")
    } else {
        format!("{{ health = {number} }}")
    };
    let document = extracted_game_document(extension, &content);
    let fact_id = document["facts"][0]["fact_id"].as_str().unwrap().to_owned();
    project_entity_facts(&prepared_game_document(&document), &[fact_id])
        .unwrap()
        .remove(0)
}

fn extracted_game_document(extension: &str, content: &str) -> Value {
    extracted_game_document_from_source("fixture", extension, content)
}

fn extracted_game_document_from_source(source_id: &str, extension: &str, content: &str) -> Value {
    use dbrain_sources::game_files::{extract_game_files, GameFileOptions};

    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join(format!("health.{extension}")), content).unwrap();
    let options = GameFileOptions {
        root: root.path().into(),
        app_id: 1422450,
        source_id: source_id.into(),
        observed_at: "2026-10-03T00:00:00Z".into(),
        build_id: None,
        manifest_id: None,
        source_revision: Some("fixture-revision".into()),
        depot_id: None,
        language: "und".into(),
        attribution: "Testdaten".into(),
        license_name: "fixture".into(),
        license_url: None,
        provenance: json!({"fixture":true}),
        max_file_bytes: 1024 * 1024,
    };
    let mut output = Vec::new();
    extract_game_files(&options, &mut output).unwrap();
    serde_json::from_slice(&output).unwrap()
}

fn prepared_game_document(document: &Value) -> brain_contracts::SourceRecordV2 {
    let input = validate_knowledge_jsonl_str(&document.to_string()).unwrap();
    let source = document["source_id"].as_str().unwrap();
    let policy: ImportPolicy = serde_json::from_value(json!({"sources":{source:{
        "internal_read_allowed":true,"raw_retention_allowed":true,"publication_allowed":false,"provider_egress_allowed":false,"authorization_ref":"fixture-grant","provenance_evidence_ref":"fixture-origin","allowed_scopes":[]
    }}})).unwrap();
    let prepared = prepare_validated_knowledge(&input, &policy, "fixture-v1").unwrap();
    prepared.records()[0].record.clone()
}

fn kv1_documents(legacy: bool) -> Vec<Value> {
    [
        "a { damage 10 damage 11 } a { damage 20 } b { damage 30 } \"a/b~\" { damage 40 }",
        "a { damage 12 damage 11 } a { damage 20 } b { damage 30 } \"a/b~\" { damage 40 }",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, content)| {
        let mut document =
            extracted_game_document_from_source(&format!("kv1-{index}-{legacy}"), "kv1", content);
        if legacy {
            for fact in document["facts"].as_array_mut().unwrap() {
                fact["qualifiers"]
                    .as_object_mut()
                    .unwrap()
                    .remove("source_pointer");
            }
        }
        document
    })
    .collect()
}

#[test]
fn kv1_profiles_compare_full_original_scope_including_frozen_documents() {
    for legacy in [false, true] {
        let mut combined = Vec::new();
        for document in kv1_documents(legacy) {
            let record = prepared_game_document(&document);
            let ids: Vec<String> = document["facts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|fact| fact["fact_id"].as_str().unwrap().into())
                .collect();
            let facts = project_entity_facts(&record, &ids).unwrap();
            assert_eq!(facts.len(), 5);
            for (fact, original) in facts.iter().zip(document["facts"].as_array().unwrap()) {
                assert_eq!(fact.subject, "game_file:health.kv1");
                assert_eq!(fact.value, original["value"]);
                assert_eq!(
                    serde_json::to_value(&fact.qualifiers).unwrap(),
                    original["qualifiers"]
                );
                assert_eq!(
                    fact.provenance.source_span.as_deref(),
                    original["source_span"].as_str()
                );
            }
            let profile = assemble_profile(entity(EntityKind::Hero), None, facts.clone(), vec![]);
            assert!(profile.conflicts.is_empty());
            assert_eq!(profile.facts, facts);
            combined.extend(facts);
        }
        let profile = assemble_profile(entity(EntityKind::Hero), None, combined.clone(), vec![]);
        assert_eq!(profile.conflicts.len(), 1);
        assert_eq!(profile.conflicts[0].fact_ids.len(), 2);
        assert!(profile.conflicts[0]
            .fact_ids
            .iter()
            .all(|id| id.ends_with("kv:/a/0/damage/0")));
        assert_eq!(profile.facts, combined);
    }
}

#[test]
fn kv1_missing_original_scope_is_visible_and_caller_pointer_cannot_replace_it() {
    for span in [
        Value::Null,
        json!("health.kv1:damage"),
        json!("other.kv1:/a/0/damage/0"),
    ] {
        let documents = kv1_documents(true);
        let mut facts = Vec::new();
        for mut document in documents {
            document["facts"][0]["source_span"] = span.clone();
            let record = prepared_game_document(&document);
            facts.extend(project_entity_facts(&record, &["kv:/a/0/damage/0".into()]).unwrap());
        }
        for caller_pointer in [None, Some("/a/0/damage/0")] {
            let mut unbound = facts.clone();
            if let Some(pointer) = caller_pointer {
                for fact in &mut unbound {
                    fact.qualifiers
                        .insert("source_pointer".into(), json!(pointer));
                }
            }
            let profile = assemble_profile(entity(EntityKind::Hero), None, unbound.clone(), vec![]);
            assert!(profile.conflicts.is_empty());
            assert!(profile
                .unknowns
                .iter()
                .any(|gap| gap.contains("KV1-Feldpfad")));
            assert_eq!(profile.facts, unbound);
        }
    }
}

#[test]
fn json_and_kv3_array_elements_keep_distinct_comparison_scopes() {
    for (extension, content, count) in [
        ("json", r#"{"damage":[10,20]}"#, 2),
        ("kv3", "{ damage = [10,20] damage = 30 }", 3),
    ] {
        let document = extracted_game_document(extension, content);
        let ids: Vec<String> = document["facts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|fact| fact["fact_id"].as_str().unwrap().into())
            .collect();
        let facts = project_entity_facts(&prepared_game_document(&document), &ids).unwrap();
        assert_eq!(facts.len(), count);
        let profile = assemble_profile(entity(EntityKind::Hero), None, facts.clone(), vec![]);
        assert!(profile.conflicts.is_empty());
        assert_eq!(profile.facts, facts);
    }
}

#[test]
fn extractor_lexemes_do_not_hide_conflicts_or_merge_semantic_qualifiers() {
    for extension in ["json", "kv3"] {
        for (left, right) in [
            ("12", "13"),
            ("12", "340282346638528859811704183484516925440.0"),
        ] {
            let facts = vec![
                extracted_fact(extension, left),
                extracted_fact(extension, right),
            ];
            assert_eq!(facts[0].qualifiers["source_lexeme"], left);
            assert_eq!(facts[1].qualifiers["source_lexeme"], right);
            let profile = assemble_profile(entity(EntityKind::Hero), None, facts.clone(), vec![]);
            assert_eq!(profile.conflicts.len(), 1);
            assert_eq!(profile.conflicts[0].fact_ids.len(), 2);
            assert_eq!(profile.facts, facts);

            for key in [
                "variant",
                "condition",
                "level",
                "source_pointer",
                "type_flags",
            ] {
                let mut separated = facts.clone();
                separated[0].qualifiers.insert(key.into(), json!("normal"));
                separated[1]
                    .qualifiers
                    .insert(key.into(), json!("enhanced"));
                let profile =
                    assemble_profile(entity(EntityKind::Hero), None, separated.clone(), vec![]);
                assert!(profile.conflicts.is_empty(), "{extension}:{key}");
                assert_eq!(profile.facts, separated);
            }
            let mut separated = facts.clone();
            separated[0].unit = Some("hp".into());
            separated[1].unit = Some("percent".into());
            assert!(
                assemble_profile(entity(EntityKind::Hero), None, separated, vec![])
                    .conflicts
                    .is_empty()
            );
        }
    }
}
#[test]
fn generic_fields_keep_subject_and_document_binding_in_comparison() {
    for predicate in [
        "file.kv_value",
        "file.json_value",
        "file.kv3_value",
        "wiki.data.value",
    ] {
        let mut health = extracted_fact("json", "100");
        health.predicate = predicate.into();
        health
            .qualifiers
            .insert("source_pointer".into(), json!("/value"));
        if predicate == "file.kv_value" {
            health.fact_id = "kv:/value/0".into();
            health.provenance.source_span = Some("health.kv1:/value/0".into());
            health.subject = "game_file:health.kv1".into();
            health
                .provenance
                .document_metadata
                .insert("relative_path".into(), json!("health.kv1"));
            health
                .provenance
                .document_metadata
                .insert("parse_status".into(), json!("kv1_lossless_entries"));
            health
                .qualifiers
                .insert("source_pointer".into(), json!("/value/0"));
            health
                .qualifiers
                .insert("source_key".into(), json!("value"));
            health.qualifiers.insert("occurrence".into(), json!(0));
        }
        let mut damage = health.clone();
        damage.value = json!("20");
        for change_subject in [false, true] {
            let mut separate = damage.clone();
            if change_subject {
                separate.subject = "game_file:damage.kv1".into();
                if predicate == "file.kv_value" {
                    separate.provenance.source_span = Some("damage.kv1:/value/0".into());
                    separate
                        .provenance
                        .document_metadata
                        .insert("relative_path".into(), json!("damage.kv1"));
                }
            } else {
                separate.provenance.origin.identity.logical_id = "damage-document".into();
            }
            let facts = vec![health.clone(), separate];
            let profile = assemble_profile(entity(EntityKind::Hero), None, facts.clone(), vec![]);
            assert!(profile.conflicts.is_empty(), "{predicate}:{change_subject}");
            assert_eq!(profile.facts, facts);
        }
        damage.provenance.origin.identity.source_id = "second-evidence".into();
        assert_eq!(
            assemble_profile(entity(EntityKind::Hero), None, vec![health, damage], vec![])
                .conflicts
                .len(),
            1
        );
    }
}

#[test]
fn values_units_qualifiers_and_rights_remain_lossless() {
    let facts = project_entity_facts(&record(), &["health".into()]).unwrap();
    assert_eq!(
        facts[0].value.to_string(),
        "1.2345678901234567890123456789e+19"
    );
    assert_eq!(facts[0].unit.as_deref(), Some("hp"));
    assert_eq!(facts[0].qualifiers["condition"], "unknown");
    assert_eq!(facts[0].provenance.license["redistribution_allowed"], false);
    assert!(!facts[0].provenance.origin.policy.publication_allowed);
    assert!(matches!(facts[0].validity, PatchValidity::Unknown { .. }));
}
#[test]
fn all_three_entity_kinds_keep_their_binding() {
    for kind in [EntityKind::Hero, EntityKind::Ability, EntityKind::Item] {
        let profile = assemble_profile(
            entity(kind),
            None,
            project_entity_facts(&record(), &["health".into()]).unwrap(),
            vec![],
        );
        assert_eq!(profile.entity.kind, kind);
        assert!(profile.patch.is_none());
        assert!(!profile.unknowns.is_empty());
    }
}
#[test]
fn conflicting_sources_keep_both_values_and_git_preference() {
    let mut facts = project_entity_facts(&record(), &["health".into()]).unwrap();
    let mut git = facts[0].clone();
    git.value = json!("123");
    facts[0].qualifiers.insert(
        "source_lexeme".into(),
        json!("1.2345678901234567890123456789e+19"),
    );
    git.qualifiers.insert("source_lexeme".into(), json!("123"));
    git.qualifiers.insert(
        "numeric_representation".into(),
        json!("source_numeric_lexeme"),
    );
    git.provenance.source_kind = ProfileSourceKind::GameFile;
    git.provenance.origin.identity.source_id = "git-fixture".into();
    facts.push(git);
    let profile = assemble_profile(entity(EntityKind::Hero), None, facts, vec![]);
    assert_eq!(profile.facts.len(), 2);
    assert_eq!(profile.conflicts.len(), 1);
    assert!(profile.conflicts[0]
        .preferred_fact_id
        .as_ref()
        .unwrap()
        .starts_with("git-fixture:"));
    let wiki = profile.facts[0].clone();
    let git = profile.facts[1].clone();
    for value in [json!("123"), json!("125")] {
        let mut other_git = git.clone();
        other_git.value = value.clone();
        other_git.provenance.origin.identity.source_id = "other-git-fixture".into();
        let mut facts = vec![wiki.clone(), other_git, git.clone()];
        let mut preferred = None;
        for _ in 0..2 {
            let profile = assemble_profile(entity(EntityKind::Hero), None, facts.clone(), vec![]);
            assert_eq!(profile.facts, facts);
            assert_eq!(profile.conflicts.len(), 1);
            assert_eq!(profile.conflicts[0].fact_ids.len(), 3);
            if value == git.value {
                assert!(profile.conflicts[0]
                    .preferred_fact_id
                    .as_ref()
                    .unwrap()
                    .starts_with("git-fixture:"));
            } else {
                assert!(profile.conflicts[0].preferred_fact_id.is_none());
            }
            if let Some(previous) = &preferred {
                assert_eq!(previous, &profile.conflicts[0].preferred_fact_id);
            }
            preferred = Some(profile.conflicts[0].preferred_fact_id.clone());
            facts.reverse();
        }
    }
}
#[test]
fn validity_is_open_or_exclusive_and_unknown_is_not_backdated() {
    let mut validity = PatchValidity::Known {
        from_patch: "2026-09-16".into(),
        to_patch_exclusive: None,
        through_patch_inclusive: None,
        evidence_ref: "patch-fixture".into(),
    };
    let serialized = serde_json::to_value(&validity).unwrap();
    assert!(serialized.get("through_patch_inclusive").is_none());
    assert_eq!(
        serde_json::from_value::<PatchValidity>(serialized).unwrap(),
        validity
    );
    assert!(!validity_contains(&validity, "2026-09-15"));
    assert!(validity_contains(&validity, "2026-10-04"));
    if let PatchValidity::Known {
        to_patch_exclusive, ..
    } = &mut validity
    {
        *to_patch_exclusive = Some("2026-09-30".into());
    }
    assert!(validity_contains(&validity, "2026-09-29"));
    assert!(!validity_contains(&validity, "2026-09-30"));
    if let PatchValidity::Known {
        to_patch_exclusive,
        through_patch_inclusive,
        ..
    } = &mut validity
    {
        *to_patch_exclusive = None;
        *through_patch_inclusive = Some("2026-09-30".into());
    }
    assert!(validity_contains(&validity, "2026-09-30"));
    assert!(!validity_contains(&validity, "2026-10-01"));
    assert!(!validity_contains(
        &PatchValidity::Unknown {
            reason: "fehlt".into()
        },
        "2026-09-16"
    ));
}
#[test]
fn ambiguous_missing_and_duplicate_fact_bindings_fail() {
    assert!(project_entity_facts(&record(), &["missing".into()]).is_err());
    assert!(project_entity_facts(&record(), &["health".into(), "health".into()]).is_err());
}

struct ScratchPg {
    directory: tempfile::TempDir,
}
impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .args(["-D"])
            .arg(self.directory.path().join("data"))
            .args(["-m", "fast", "-w", "stop"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn isolated_import_is_idempotent_and_preserves_revision_binding() {
    let pg = ScratchPg {
        directory: tempfile::tempdir().unwrap(),
    };
    let data = pg.directory.path().join("data");
    let socket = pg.directory.path().join("socket");
    std::fs::create_dir(&socket).unwrap();
    assert!(
        std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
            .args(["-D"])
            .arg(&data)
            .args(["-A", "trust", "-U", "fixture", "--no-locale"])
            .stdout(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    );
    let options = format!("-k {} -p 15446 -c listen_addresses=''", socket.display());
    assert!(
        std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .args(["-D"])
            .arg(&data)
            .args(["-l"])
            .arg(pg.directory.path().join("postgres.log"))
            .args(["-o", &options, "-w", "start"])
            .stdout(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(15446)
                .username("fixture")
                .database("postgres"),
        )
        .await
        .unwrap();
    let store = brain_storage::PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    store.migrate_entity_profiles().await.unwrap();
    store.migrate_entity_profiles().await.unwrap();
    store.check_entity_profile_schema().await.unwrap();
    let mut record = record();
    let mut origin = brain_contracts::source::origin_from_record(&record).unwrap();
    record.allowed_scopes.insert("fixture:read".into());
    origin.policy.allowed_scopes = record.allowed_scopes.clone();
    origin.bind_record(&mut record).unwrap();
    store.apply(&record).await.unwrap();
    let mut entity = entity(EntityKind::Hero);
    entity.name = "Vertraulicher Name".into();
    entity.aliases = vec!["Vertraulicher Alias".into()];
    entity.identity_evidence = vec!["vertraulicher Beleg".into()];
    let mut manipulated = record.clone();
    let document_key = brain_storage::source_versions::DOCUMENT_METADATA_KEY;
    let mut document: Value = serde_json::from_str(&manipulated.metadata[document_key]).unwrap();
    document["facts"][0]["value"] = json!(999999);
    manipulated
        .metadata
        .insert(document_key.into(), document.to_string());
    assert_eq!(manipulated.content, record.content);
    assert_eq!(manipulated.content_hash, record.content_hash);
    assert_ne!(
        serde_json::to_value(&manipulated).unwrap(),
        serde_json::to_value(&record).unwrap()
    );
    let original: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
        .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_one(&pool).await.unwrap();
    assert_eq!(original, serde_json::to_value(&record).unwrap());
    assert!(store
        .store_entity_fact_bindings(&entity, &manipulated, &["health".into()])
        .await
        .is_err());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.entity_profile_facts_v1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.entity_profile_entities_v1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(
        store
            .store_entity_fact_bindings(&entity, &record, &["health".into()])
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .store_entity_fact_bindings(&entity, &record, &["health".into()])
            .await
            .unwrap(),
        0
    );
    let names: Vec<String> = sqlx::query_scalar(
        "SELECT lookup_names FROM brain.entity_profile_entities_v1 WHERE entity_key=$1",
    )
    .bind(&entity.entity_key)
    .fetch_one(&pool)
    .await
    .unwrap();
    let mut expected = vec![entity.name.clone(), entity.aliases[0].clone()];
    expected.sort();
    assert_eq!(names, expected);
    let stored: String = sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1")
        .fetch_one(&pool)
        .await
        .unwrap();
    let stored: Value = serde_json::from_str(&stored).unwrap();
    assert_eq!(
        stored["value"].to_string(),
        "1.2345678901234567890123456789e+19"
    );
    let mut conflict = entity.clone();
    conflict.kind = EntityKind::Item;
    assert!(store
        .store_entity_fact_bindings(&conflict, &record, &["health".into()])
        .await
        .is_err());
    sqlx::raw_sql("CREATE TABLE brain.patch_changes(patch_date text, entity_type text, entity_name text, ability_name text, stat_name text, old_value text, new_value text, raw_line text, patch_url text)").execute(&pool).await.unwrap();
    let release = brain_contracts::CorpusRelease {
        release_id: "fixture-binding-release".into(),
        knowledge_version: "k1".into(),
        patch: "p1".into(),
        created_at_epoch: 1,
        source_revisions: std::collections::BTreeMap::from([(
            record.source_id.clone(),
            std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
        )]),
    };
    store.publish_release(&release).await.unwrap();
    let mut principal = brain_contracts::Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: Default::default(),
        provider_egress: Default::default(),
    };
    let hidden = store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap();
    assert!(hidden.is_none());
    principal.scopes = record.allowed_scopes.clone();
    let visible = store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(visible.facts.len(), 1);
    assert_eq!(
        visible.entity,
        brain_storage::entity_profile::consumer_entity_identity(&entity)
    );
    entity_kinds_are_consistent_across_bindings(&store, &pool, &principal).await;
    patch_stories_follow_entity_kind(&store, &pool, &record, &release, &principal).await;
    assert_eq!(
        visible.facts[0].value.to_string(),
        "1.2345678901234567890123456789e+19"
    );
    sqlx::query(
        "UPDATE brain.entity_profile_facts_v1 SET binding_identity_json=NULL WHERE entity_key=$1",
    )
    .bind(&entity.entity_key)
    .execute(&pool)
    .await
    .unwrap();
    assert!(store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .is_err());
    assert_eq!(
        store
            .store_entity_fact_bindings(&entity, &record, &["health".into()])
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
            .await
            .unwrap()
            .unwrap()
            .entity,
        brain_storage::entity_profile::consumer_entity_identity(&entity)
    );
    let mut restricted = record.clone();
    restricted.revision += 1;
    restricted.allowed_scopes = std::collections::BTreeSet::from(["fixture:restricted".into()]);
    origin.policy.allowed_scopes = restricted.allowed_scopes.clone();
    origin.bind_record(&mut restricted).unwrap();
    store.apply(&restricted).await.unwrap();
    let hidden = store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap();
    assert!(hidden.is_none());
    principal.scopes.extend(restricted.allowed_scopes.clone());
    let visible = store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(visible.facts.len(), 1);
    assert_eq!(
        visible.facts[0].provenance.origin.policy.allowed_scopes,
        record.allowed_scopes
    );
    let mut public = record.clone();
    let mut public_origin = brain_contracts::source::origin_from_record(&public).unwrap();
    public.source_id = "public-fixture".into();
    public.allowed_scopes = std::collections::BTreeSet::from(["fixture:public".into()]);
    public_origin.identity.source_id = public.source_id.clone();
    public_origin.policy.allowed_scopes = public.allowed_scopes.clone();
    public_origin.bind_record(&mut public).unwrap();
    let mut document: Value = serde_json::from_str(&public.metadata[document_key]).unwrap();
    document["source_id"] = json!(public.source_id);
    document["title"] = json!("Öffentlicher Name");
    public
        .metadata
        .insert(document_key.into(), document.to_string());
    store.apply(&public).await.unwrap();
    let mut public_entity = entity.clone();
    public_entity.name = "Öffentlicher Name".into();
    public_entity.aliases.clear();
    public_entity.identity_evidence = vec!["public-fixture:identity".into()];
    store
        .store_entity_fact_bindings(&public_entity, &public, &["health".into()])
        .await
        .unwrap();
    let mut mixed = release.clone();
    mixed.release_id = "mixed-release".into();
    mixed.source_revisions.insert(
        public.source_id.clone(),
        std::collections::BTreeMap::from([(public.logical_id.clone(), public.revision)]),
    );
    store.publish_release(&mixed).await.unwrap();
    principal.scopes = public.allowed_scopes.clone();
    let visible = store
        .read_entity_profile(&entity.entity_key, &mixed.release_id, &principal, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(visible.facts.len(), 1);
    assert_eq!(visible.entity.name, "Öffentlicher Name");
    assert!(visible.entity.aliases.is_empty());
    assert!(visible
        .entity
        .identity_evidence
        .iter()
        .all(|evidence| evidence.starts_with("public-fixture:")));
    let mut unpinned = mixed.clone();
    unpinned.release_id = "unpinned-release".into();
    unpinned.source_revisions.clear();
    store.publish_release(&unpinned).await.unwrap();
    assert!(store
        .read_entity_profile(&entity.entity_key, &unpinned.release_id, &principal, None)
        .await
        .unwrap()
        .is_none());
    public.revision += 1;
    public.tombstone = true;
    store.apply(&public).await.unwrap();
    assert!(store
        .read_entity_profile(&entity.entity_key, &mixed.release_id, &principal, None)
        .await
        .unwrap()
        .is_none());
    let mut shared_kv1_identity = entity.clone();
    shared_kv1_identity.entity_key = "kv1-shared".into();
    let mut shared_kv1_release = release.clone();
    shared_kv1_release.release_id = "kv1-shared-release".into();
    shared_kv1_release.source_revisions.clear();
    let mut shared_kv1_facts = Vec::new();
    let mut shared_kv1_principal = principal.clone();
    shared_kv1_principal.scopes.clear();
    for (index, record) in extractor_records().into_iter().enumerate() {
        let document: Value = serde_json::from_str(&record.metadata[document_key]).unwrap();
        let ids: Vec<String> = document["facts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| {
                let subject = f["subject"].as_str().unwrap();
                assert!(subject.starts_with("game_file:") || subject.starts_with("wiki:"));
                f["fact_id"].as_str().unwrap().into()
            })
            .collect();
        assert!(!ids.is_empty());
        let identity = EntityIdentity {
            entity_key: format!("extractor-{index}"),
            kind: [EntityKind::Hero, EntityKind::Ability, EntityKind::Item][index % 3],
            name: format!("Belegter Name {index}"),
            aliases: vec![format!("Belegter Alias {index}")],
            identity_evidence: vec![format!("{}:binding", record.logical_id)],
        };
        store.apply(&record).await.unwrap();
        assert_eq!(
            store
                .store_entity_fact_bindings(&identity, &record, &ids)
                .await
                .unwrap(),
            ids.len()
        );
        let release = brain_contracts::CorpusRelease {
            release_id: format!("extractor-release-{index}"),
            source_revisions: std::collections::BTreeMap::from([(
                record.source_id.clone(),
                std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
            )]),
            ..release.clone()
        };
        store.publish_release(&release).await.unwrap();
        principal.scopes.clear();
        assert!(store
            .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
            .await
            .unwrap()
            .is_none());
        principal.scopes = record.allowed_scopes.clone();
        let actual = store
            .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            actual.entity,
            brain_storage::entity_profile::consumer_entity_identity(&identity)
        );
        let expected = project_entity_facts(&record, &ids).unwrap();
        assert_eq!(actual.facts.len(), expected.len());
        for fact in expected {
            assert!(actual.facts.contains(&fact));
        }
        assert!(actual.conflicts.is_empty());
        if record.source_id.starts_with("kv1-") {
            shared_kv1_principal
                .scopes
                .extend(record.allowed_scopes.clone());
            store
                .store_entity_fact_bindings(&shared_kv1_identity, &record, &ids)
                .await
                .unwrap();
            shared_kv1_release.source_revisions.insert(
                record.source_id.clone(),
                std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
            );
            shared_kv1_facts.extend(actual.facts);
            let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
                .bind(&record.source_id).bind(&record.logical_id).bind(i64::try_from(record.revision).unwrap())
                .fetch_one(&pool).await.unwrap();
            assert_eq!(stored, serde_json::to_value(&record).unwrap());
        }
    }
    store.publish_release(&shared_kv1_release).await.unwrap();
    let shared = store
        .read_entity_profile(
            &shared_kv1_identity.entity_key,
            &shared_kv1_release.release_id,
            &shared_kv1_principal,
            None,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(shared.facts.len(), 20);
    assert_eq!(shared.conflicts.len(), 1);
    assert_eq!(shared.conflicts[0].fact_ids.len(), 4);
    assert!(shared.conflicts[0]
        .fact_ids
        .iter()
        .all(|id| id.ends_with("kv:/a/0/damage/0")));
    for fact in shared_kv1_facts {
        assert!(shared.facts.contains(&fact));
    }
    original_alias_bindings_drive_history_without_consumer_leaks(&store, &pool).await;
    private_einheiten_bleiben_bei_gespeicherter_neuableitung_in_den_originalpins(&store, &pool)
        .await;
    gitblobfelder_werden_im_normalen_ableitungs_und_abrufweg_geprüft(&store, &pool, &socket).await;
    sqlx::raw_sql(
        "CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service; CREATE ROLE brain_readonly",
    )
    .execute(&pool)
    .await
    .unwrap();
    let grants = include_str!("../../../../ops/brain-postgres/grants.sql")
        .lines()
        .filter(|line| !line.starts_with('\\'))
        .collect::<Vec<_>>()
        .join("\n");
    sqlx::raw_sql(&grants).execute(&pool).await.unwrap();
    sqlx::raw_sql("GRANT SELECT,INSERT,UPDATE ON brain.entity_profile_entities_v1,brain.entity_profile_facts_v1 TO brain_ingest; GRANT SELECT,INSERT ON brain.entity_semantic_projections_v1 TO brain_ingest")
        .execute(&pool)
        .await
        .unwrap();
    let ingest_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(15446)
                .username("brain_ingest")
                .database("postgres"),
        )
        .await
        .unwrap();
    let raw_rights: (bool, bool, bool, bool) = sqlx::query_as(
        "SELECT has_table_privilege(current_user,'brain.source_record_revisions','SELECT'),has_table_privilege(current_user,'brain.source_record_revisions','INSERT'),has_table_privilege(current_user,'brain.source_record_revisions','UPDATE'),has_table_privilege(current_user,'brain.source_record_revisions','DELETE')",
    )
    .fetch_one(&ingest_pool)
    .await
    .unwrap();
    assert_eq!(raw_rights, (true, true, false, false));
    unicode_bindings_remain_verifiable_after_import(
        &brain_storage::PgStore::new(ingest_pool.clone()),
        &pool,
    )
    .await;
    ingest_pool.close().await;
    pool.close().await;
}

async fn original_alias_bindings_drive_history_without_consumer_leaks(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
) {
    use brain_storage::entity_profile::{
        consumer_entity_identity,
        semantic::{project_semantic_fact, semantic_projection},
    };
    use dbrain_sources::entity_binding::{bind_document, bind_stored_document, CatalogEntity};
    store.migrate_entity_profiles().await.unwrap();
    let document = extracted_game_document_from_source(
        "alias-original",
        "json",
        r#"{"hero_test":{"MaxHealth":120,"Variants":[{"Damage":20},{"Damage":30}]}}"#,
    );
    let mut record = prepared_game_document(&document);
    let mut origin = brain_contracts::source::origin_from_record(&record).unwrap();
    record
        .allowed_scopes
        .insert("fixture:alias-original".into());
    origin.policy.allowed_scopes = record.allowed_scopes.clone();
    origin.bind_record(&mut record).unwrap();
    store.apply(&record).await.unwrap();
    let mut identity = entity(EntityKind::Hero);
    identity.entity_key = "alias-original-hero".into();
    identity.name = "Testheld".into();
    identity.aliases = vec!["Historischer Privatname".into()];
    let catalog = vec![CatalogEntity {
        identity: identity.clone(),
        identifiers: vec!["hero_test".into()],
    }];
    let knowledge = serde_json::from_value(document).unwrap();
    let coverage = bind_document(&knowledge, &catalog);
    let mut previous = identity.clone();
    previous.aliases.clear();
    previous.identity_evidence = coverage
        .bindings
        .iter()
        .flat_map(|binding| binding.identity_evidence.clone())
        .collect();
    previous.identity_evidence.sort();
    previous.identity_evidence.dedup();
    let ids: Vec<_> = coverage
        .bindings
        .iter()
        .map(|binding| binding.fact.fact_id.clone())
        .collect();
    store
        .store_entity_fact_bindings(&previous, &record, &ids)
        .await
        .unwrap();
    for _ in 0..2 {
        bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
    }
    let mut catalog = catalog;
    catalog[0]
        .identity
        .aliases
        .push("Zusätzlicher Katalogalias".into());
    catalog[0]
        .identity
        .identity_evidence
        .push("brain.entity_aliases:200".into());
    bind_stored_document(
        store,
        &record.source_id,
        &record.logical_id,
        record.revision,
        &catalog,
    )
    .await
    .unwrap();
    let rebound = store
        .stored_entity_binding_identity(&identity.entity_key, &record, "json:/hero_test/MaxHealth")
        .await
        .unwrap();
    assert!(rebound
        .identity_evidence
        .contains(&"brain.entity_aliases:200".into()));
    let mut wrong = rebound.clone();
    wrong.kind = EntityKind::Item;
    assert!(store
        .store_entity_fact_bindings(&wrong, &record, &ids)
        .await
        .is_err());
    let mut wrong = rebound.clone();
    wrong.identity_evidence.remove(0);
    assert!(store
        .store_entity_fact_bindings(&wrong, &record, &ids)
        .await
        .is_err());
    let mut wrong_record = record.clone();
    wrong_record.content.push('x');
    assert!(store
        .store_entity_fact_bindings(&rebound, &wrong_record, &ids)
        .await
        .is_err());
    let health_id = "json:/hero_test/MaxHealth";
    let stored = store
        .stored_entity_binding_identity(&identity.entity_key, &record, health_id)
        .await
        .unwrap();
    assert!(stored.aliases.contains(&"Historischer Privatname".into()));
    assert!(stored.aliases.contains(&"hero_test".into()));
    assert_eq!(
        consumer_entity_identity(&stored).aliases,
        Vec::<String>::new()
    );
    assert!(consumer_entity_identity(&stored)
        .identity_evidence
        .is_empty());
    sqlx::query("INSERT INTO brain.patch_changes(patch_date,entity_type,entity_name,stat_name,old_value,new_value,raw_line,patch_url) VALUES('2026-09-01','hero','Historischer Privatname','Max Health','100','110','Historischer Privatname','https://example.org/patch'),('2026-09-16','hero','Historischer Privatname','Max Health','110','120','Historischer Privatname','https://example.org/patch')").execute(pool).await.unwrap();
    assert_eq!(store.entity_patch_story(&stored).await.unwrap().len(), 2);
    let variant_id = "json:/hero_test/Variants/1/Damage";
    let fact = project_entity_facts(&record, &[variant_id.into()])
        .unwrap()
        .remove(0);
    let mut forged = semantic_projection(&fact, "/Variants/1/Damage", &record, &stored)
        .unwrap()
        .unwrap();
    forged.relative_pointer = "/Damage".into();
    forged.qualifiers.remove("semantic_scope");
    assert!(project_semantic_fact(&fact, &forged, &record, &stored).is_err());
    assert!(store
        .store_entity_semantic_projection(&identity.entity_key, &record, variant_id, &forged)
        .await
        .is_err());
    let release = brain_contracts::CorpusRelease {
        release_id: "alias-original-release".into(),
        knowledge_version: "fixture".into(),
        patch: "unknown".into(),
        created_at_epoch: 1,
        source_revisions: std::collections::BTreeMap::from([(
            record.source_id.clone(),
            std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
        )]),
    };
    store.publish_release(&release).await.unwrap();
    let mut principal = brain_contracts::Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: record.allowed_scopes.clone(),
        provider_egress: Default::default(),
    };
    let current = store
        .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        current
            .facts
            .iter()
            .find(|fact| fact.predicate == "max_health")
            .unwrap()
            .value,
        json!(120)
    );
    assert_eq!(current.patch_story.len(), 2);
    let profile = store
        .read_entity_profile(
            &identity.entity_key,
            &release.release_id,
            &principal,
            Some("2026-09-15"),
        )
        .await
        .unwrap()
        .unwrap();
    assert!(profile.facts.is_empty());
    assert!(profile
        .unknowns
        .iter()
        .any(|unknown| unknown.contains("Historische Fakten")));
    assert_eq!(profile.patch_story.len(), 1);
    let consumer = serde_json::to_string(&profile.entity).unwrap();
    assert!(!consumer.contains("Historischer Privatname"));
    let story = serde_json::to_string(&profile.patch_story).unwrap();
    assert!(!story.contains("Historischer Privatname"));
    assert_eq!(
        profile.patch_story[0].entity_name.as_deref(),
        Some("Testheld")
    );
    assert!(!serde_json::to_string(&profile)
        .unwrap()
        .contains("Historischer Privatname"));
    let changes = store
        .read_entity_profile(
            &identity.entity_key,
            &release.release_id,
            &principal,
            Some("2026-09-16"),
        )
        .await
        .unwrap()
        .unwrap();
    assert!(changes.facts.is_empty());
    let change = changes
        .patch_story
        .iter()
        .find(|change| change.patch_date == "2026-09-16")
        .unwrap();
    assert_eq!(change.entity_name.as_deref(), Some("Testheld"));
    assert_eq!(change.stat_name.as_deref(), Some("Max Health"));
    assert_eq!(change.old_value, "110");
    assert_eq!(change.new_value, "120");
    assert_eq!(change.provenance.relation, "brain.patch_changes");
    principal.scopes.clear();
    assert!(store
        .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .is_none());
    principal.scopes = record.allowed_scopes.clone();
    let mut revoked = record.clone();
    revoked.revision += 1;
    revoked
        .allowed_scopes
        .insert("fixture:alias-revoked".into());
    origin.policy.allowed_scopes = revoked.allowed_scopes.clone();
    origin.bind_record(&mut revoked).unwrap();
    store.apply(&revoked).await.unwrap();
    assert!(store
        .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .is_none());
    principal.scopes = revoked.allowed_scopes.clone();
    assert!(store
        .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .is_some());
    revoked.revision += 1;
    revoked.tombstone = true;
    store.apply(&revoked).await.unwrap();
    assert!(store
        .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .is_none());
}

async fn private_einheiten_bleiben_bei_gespeicherter_neuableitung_in_den_originalpins(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
) {
    use brain_storage::entity_profile::{
        compact::compact_document,
        derivation::{
            derive_git_profile, derived_policy, receipt_sha256, verify_git_document_receipt,
            GitBlobEvidence, GitDocumentReceipt, GIT_DOCUMENT_CONTRACT,
        },
    };
    use dbrain_sources::entity_binding::{bind_stored_document, CatalogEntity};

    store.migrate_entity_profiles().await.unwrap();
    for (index, unit) in [
        "/private/health.json",
        "private/health.json",
        "../health.json",
    ]
    .into_iter()
    .enumerate()
    {
        let mut document = extracted_game_document_from_source(
            &format!("unit-original-{index}"),
            "json",
            r#"{"hero_test":{"Health":998877,"Damage":25,"Variants":[{"class_name":"nested_test","Bonus":30}]}}"#,
        );
        let commit = "a".repeat(40);
        document["revision"] = json!(format!("git:{commit}"));
        document["metadata"]["source_revision"] = json!(format!("git:{commit}"));
        document["metadata"]["provenance"]["repository_url"] =
            json!("https://github.com/deadlock-wiki/deadlock-data");
        for fact in document["facts"].as_array_mut().unwrap() {
            if fact["fact_id"] == "json:/hero_test/Variants/0/Bonus" {
                fact["qualifiers"]["condition"] = json!("bei Treffer");
            }
            fact["unit"] = if fact["value"] == json!(998877) {
                json!(unit)
            } else {
                json!("HP")
            };
        }
        let mut record = prepared_game_document(&document);
        let mut original_origin = brain_contracts::source::origin_from_record(&record).unwrap();
        record.allowed_scopes.insert("fixture:unit-original".into());
        original_origin.policy.allowed_scopes = record.allowed_scopes.clone();
        original_origin.bind_record(&mut record).unwrap();
        store.apply(&record).await.unwrap();
        let mut identity = entity(EntityKind::Hero);
        identity.entity_key = format!("unit-hero-{index}");
        let mut catalog = vec![CatalogEntity {
            identity: identity.clone(),
            identifiers: vec!["hero_test".into()],
        }];
        bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        let release = brain_contracts::CorpusRelease {
            release_id: format!("unit-release-{index}"),
            knowledge_version: "fixture".into(),
            patch: "unknown".into(),
            created_at_epoch: 1,
            source_revisions: std::collections::BTreeMap::from([(
                record.source_id.clone(),
                std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
            )]),
        };
        store.publish_release(&release).await.unwrap();
        let principal = brain_contracts::Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: record.allowed_scopes.clone(),
            provider_egress: Default::default(),
        };
        let snapshot = store.snapshot(&release.release_id).await.unwrap();
        let bindings = store
            .stored_git_entity_bindings(&identity.entity_key, &record)
            .await
            .unwrap();
        assert_eq!(bindings.len(), 4);
        let originals = serde_json::to_value(
            bindings
                .iter()
                .map(|binding| &binding.original_fact)
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let blobs = [GitBlobEvidence {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
            git_commit: commit,
            repository_url: "https://github.com/deadlock-wiki/deadlock-data".into(),
            bytes: record.content.as_bytes().to_vec(),
        }];
        let (profile, receipt) = derive_git_profile(
            &identity.entity_key,
            &snapshot,
            &principal,
            &bindings,
            &blobs,
            &[],
        )
        .unwrap();
        assert_eq!(receipt.fact_pins.len(), 3);
        assert!(receipt
            .fact_pins
            .iter()
            .any(|pin| pin.semantic_projection.unit.as_deref() == Some(unit)));
        assert_eq!(profile.facts.len(), 1);
        assert_eq!(profile.facts[0].value, 25);
        assert_eq!(profile.facts[0].unit.as_deref(), Some("HP"));
        let content = compact_document(&profile).unwrap();
        assert!(!content.contains(unit));
        assert!(!content.contains("998877"));
        assert!(content.contains("öffentliche Beschreibung"));
        let mut derived = record.clone();
        derived.source_id = "git-game-facts-derived".into();
        derived.logical_id = identity.entity_key.clone();
        derived.content = content;
        derived.content_hash = receipt.document_sha256.clone();
        derived.visibility = brain_contracts::SourceVisibility::Public;
        derived.allowed_scopes.clear();
        derived.metadata = std::collections::BTreeMap::from([
            (
                "brain.entity_projection.contract".into(),
                GIT_DOCUMENT_CONTRACT.into(),
            ),
            (
                "brain.entity_projection.receipt_sha256".into(),
                receipt_sha256(&receipt).unwrap(),
            ),
        ]);
        let mut origin = brain_contracts::source::origin_from_record(&record).unwrap();
        origin.identity.source_id = derived.source_id.clone();
        origin.identity.logical_id = derived.logical_id.clone();
        origin.raw_sha256 = derived.content_hash.clone();
        origin.locator = "git-game-facts-derived".into();
        origin.origin_artifacts.clear();
        origin.policy = derived_policy();
        origin.bind_record(&mut derived).unwrap();
        let saved = store
            .persist_entity_document(derived, &serde_json::to_string(&receipt).unwrap())
            .await
            .unwrap();
        let stored_record: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&saved.source_id).bind(&saved.logical_id).bind(saved.revision as i64).fetch_one(pool).await.unwrap();
        let saved: brain_contracts::SourceRecordV2 = serde_json::from_value(stored_record).unwrap();
        let stored_receipt: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
            .bind(&saved.source_id).bind(&saved.logical_id).bind(saved.revision as i64).fetch_one(pool).await.unwrap();
        let stored_receipt: GitDocumentReceipt = serde_json::from_str(&stored_receipt).unwrap();
        let before = store
            .read_entity_profile(&identity.entity_key, &release.release_id, &principal, None)
            .await
            .unwrap()
            .unwrap();
        catalog[0].identifiers.push("nested_test".into());
        catalog[0]
            .identity
            .aliases
            .push("Zusätzlicher Katalogalias".into());
        catalog[0]
            .identity
            .identity_evidence
            .push("brain.entity_aliases:201".into());
        for _ in 0..2 {
            let result = bind_stored_document(
                store,
                &record.source_id,
                &record.logical_id,
                record.revision,
                &catalog,
            )
            .await
            .unwrap();
            assert_eq!(result.inserted_bindings, 0);
            assert_eq!(result.inserted_projections, 0);
            assert_eq!(
                store
                    .read_entity_profile(
                        &identity.entity_key,
                        &release.release_id,
                        &principal,
                        None
                    )
                    .await
                    .unwrap()
                    .unwrap(),
                before
            );
        }
        let fresh_snapshot = store.snapshot(&release.release_id).await.unwrap();
        let fresh_bindings = store
            .stored_git_entity_bindings(&identity.entity_key, &record)
            .await
            .unwrap();
        for invalid_case in 0..3 {
            let mut wrong = catalog.clone();
            match invalid_case {
                0 => wrong[0].identity.kind = EntityKind::Item,
                1 => wrong[0].identity.name = "Fremde Entität".into(),
                _ => {
                    wrong[0].identity.identity_evidence.remove(0);
                }
            };
            assert!(bind_stored_document(
                store,
                &record.source_id,
                &record.logical_id,
                record.revision,
                &wrong
            )
            .await
            .is_err());
            let unchanged = store
                .stored_git_entity_bindings(&identity.entity_key, &record)
                .await
                .unwrap();
            for (before, after) in fresh_bindings.iter().zip(unchanged) {
                assert_eq!(before.original_fact, after.original_fact);
                assert_eq!(before.binding_identity, after.binding_identity);
                assert_eq!(before.semantic_projection, after.semantic_projection);
            }
        }
        for pin in &stored_receipt.fact_pins {
            let fresh = fresh_bindings
                .iter()
                .find(|binding| binding.original_fact.fact_id == pin.fact_id)
                .unwrap();
            assert_eq!(
                fresh.semantic_projection.as_ref(),
                Some(&pin.semantic_projection)
            );
            assert!(fresh
                .binding_identity
                .aliases
                .contains(&"nested_test".into()));
        }
        let nested = fresh_bindings
            .iter()
            .find(|binding| binding.original_fact.fact_id == "json:/hero_test/Variants/0/Bonus")
            .unwrap();
        let projection = nested.semantic_projection.as_ref().unwrap();
        assert_eq!(projection.relative_pointer, "/Variants/0/Bonus");
        assert_eq!(projection.qualifiers["semantic_scope"], "Variants/0");
        assert_eq!(projection.qualifiers["condition"], "bei Treffer");
        assert_eq!(
            serde_json::to_value(
                fresh_bindings
                    .iter()
                    .map(|binding| &binding.original_fact)
                    .collect::<Vec<_>>()
            )
            .unwrap(),
            originals
        );
        let revalidated = verify_git_document_receipt(
            &saved,
            &stored_receipt,
            &fresh_snapshot,
            &principal,
            &fresh_bindings,
            &blobs,
            &[],
        )
        .unwrap();
        assert_eq!(revalidated, profile);
        assert_eq!(compact_document(&revalidated).unwrap(), saved.content);
        let repeated = derive_git_profile(
            &identity.entity_key,
            &fresh_snapshot,
            &principal,
            &fresh_bindings,
            &blobs,
            &[],
        )
        .unwrap();
        assert_eq!(repeated.0, profile);
        assert_eq!(repeated.1.document_sha256, stored_receipt.document_sha256);
        let mut invalid_bindings = fresh_bindings.clone();
        invalid_bindings[0].binding_identity.kind = EntityKind::Item;
        assert!(verify_git_document_receipt(
            &saved,
            &stored_receipt,
            &fresh_snapshot,
            &principal,
            &invalid_bindings,
            &blobs,
            &[]
        )
        .is_err());
        let mut invalid_bindings = fresh_bindings.clone();
        for binding in &mut invalid_bindings {
            binding
                .binding_identity
                .identity_evidence
                .retain(|evidence| evidence != &identity.identity_evidence[0]);
        }
        assert!(verify_git_document_receipt(
            &saved,
            &stored_receipt,
            &fresh_snapshot,
            &principal,
            &invalid_bindings,
            &blobs,
            &[]
        )
        .is_err());
        assert_eq!(
            store
                .persist_entity_document(
                    saved.clone(),
                    &serde_json::to_string(&stored_receipt).unwrap()
                )
                .await
                .unwrap(),
            saved
        );
    }
}

async fn unicode_bindings_remain_verifiable_after_import(
    store: &brain_storage::PgStore,
    owner: &sqlx::PgPool,
) {
    use brain_storage::entity_profile::semantic::project_semantic_fact;
    use dbrain_sources::entity_binding::{bind_stored_document, CatalogEntity};

    for (index, content) in [
        r#"{"äther":{"Health":120}}"#,
        r#"{"0":{"CLASS_NAME":"äTHER","Health":125}}"#,
    ]
    .into_iter()
    .enumerate()
    {
        let document =
            extracted_game_document_from_source(&format!("unicode-{index}"), "json", content);
        let record = prepared_game_document(&document);
        store.apply(&record).await.unwrap();
        let mut identity = entity(EntityKind::Hero);
        identity.entity_key = format!("unicode-{index}");
        identity.name = "Äther".into();
        let catalog = [CatalogEntity {
            identity: identity.clone(),
            identifiers: vec!["Äther".into()],
        }];
        let first = bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        assert_eq!(first.bound_facts, index + 1);
        assert_eq!(first.inserted_bindings, index + 1);
        assert_eq!(first.inserted_projections, 1);
        let retry = bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        assert_eq!(retry.bound_facts, first.bound_facts);
        assert_eq!(retry.inserted_bindings, 0);
        assert_eq!(retry.inserted_projections, 0);
        let verify = || {
            dbrain_sources::entity_binding::verify_stored_document_bindings(
                store,
                &record.source_id,
                &record.logical_id,
                record.revision,
                &catalog,
            )
        };
        let checked = verify().await.unwrap();
        assert_eq!(checked.bound_facts, first.bound_facts);
        assert_eq!(checked.inserted_bindings, 0);
        assert_eq!(checked.inserted_projections, 0);
        sqlx::query("DELETE FROM brain.entity_semantic_projections_v1 WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).execute(owner).await.unwrap();
        assert!(verify().await.is_err());
        bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        let saved:Vec<(String,String)> = sqlx::query_as("SELECT fact_id,fact_json FROM brain.entity_profile_facts_v1 WHERE source_id=$1 AND logical_id=$2 AND revision=$3 ORDER BY fact_id")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_all(owner).await.unwrap();
        let corrupted = if index == 0 {
            let mut fact: serde_json::Value = serde_json::from_str(&saved[0].1).unwrap();
            fact["value"] = serde_json::json!(999);
            fact.to_string()
        } else {
            saved[1].1.clone()
        };
        sqlx::query("UPDATE brain.entity_profile_facts_v1 SET fact_json=$5 WHERE source_id=$1 AND logical_id=$2 AND revision=$3 AND fact_id=$4")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).bind(&saved[0].0).bind(&corrupted).execute(owner).await.unwrap();
        assert!(verify().await.is_err());
        sqlx::query("UPDATE brain.entity_profile_facts_v1 SET fact_json=$5 WHERE source_id=$1 AND logical_id=$2 AND revision=$3 AND fact_id=$4")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).bind(&saved[0].0).bind(&saved[0].1).execute(owner).await.unwrap();
        assert!(verify().await.is_ok());
        sqlx::query("DELETE FROM brain.entity_semantic_projections_v1 WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
            .execute(owner).await.unwrap();
        sqlx::query("DELETE FROM brain.entity_profile_facts_v1 WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64)
            .execute(owner).await.unwrap();
        assert!(verify().await.is_err());
        bind_stored_document(
            store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        assert!(verify().await.is_ok());
        let bindings = store
            .stored_git_entity_bindings(&identity.entity_key, &record)
            .await
            .unwrap();
        let health = bindings
            .iter()
            .find(|binding| binding.semantic_projection.is_some())
            .unwrap();
        let projected = project_semantic_fact(
            &health.original_fact,
            health.semantic_projection.as_ref().unwrap(),
            &record,
            &health.binding_identity,
        )
        .unwrap();
        assert_eq!(projected.predicate, "health");
        assert_eq!(projected.value, json!(if index == 0 { 120 } else { 125 }));
        assert_eq!(health.binding_identity.name, "Äther");
    }
}

async fn entity_kinds_are_consistent_across_bindings(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    principal: &brain_contracts::Principal,
) {
    let document_key = brain_storage::source_versions::DOCUMENT_METADATA_KEY;
    let mut document: Value = serde_json::from_str(&record().metadata[document_key]).unwrap();
    document["source_id"] = json!("kind-fixture");
    document["document_id"] = json!("wiki:kind-fixture:page:123");
    let mut other_fact = document["facts"][0].clone();
    other_fact["fact_id"] = json!("damage");
    document["facts"].as_array_mut().unwrap().push(other_fact);
    let first = prepared_game_document(&document);
    store.apply(&first).await.unwrap();
    let mut candidates = vec![(first.clone(), "damage")];
    for (field, value) in [
        ("source_id", "other-source"),
        ("document_id", "wiki:kind-fixture:page:789"),
        ("revision", "457"),
    ] {
        let mut next = document.clone();
        next[field] = json!(value);
        if field == "source_id" {
            next["document_id"] = json!("wiki:other-source:page:123");
        }
        let mut record = prepared_game_document(&next);
        if field == "revision" {
            record.revision = first.revision + 1;
        }
        store.apply(&record).await.unwrap();
        candidates.push((record, "health"));
    }
    let mut reader = principal.clone();
    reader.scopes.extend(first.allowed_scopes.clone());
    for (record, _) in &candidates {
        reader.scopes.extend(record.allowed_scopes.clone());
    }
    for (index, (second, fact_id)) in candidates.iter().enumerate() {
        let mut hero = entity(EntityKind::Hero);
        hero.entity_key = format!("kind-consistency-{index}");
        let mut item = hero.clone();
        item.kind = EntityKind::Item;
        store
            .store_entity_fact_bindings(&hero, &first, &["health".into()])
            .await
            .unwrap();
        assert!(store
            .store_entity_fact_bindings(&item, second, &[(*fact_id).into()])
            .await
            .is_err());
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM brain.entity_profile_facts_v1 WHERE entity_key=$1",
        )
        .bind(&hero.entity_key)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(count, 1);
        let mut compatible = hero.clone();
        compatible.name = "Weiterer belegter Name".into();
        store
            .store_entity_fact_bindings(&compatible, second, &[(*fact_id).into()])
            .await
            .unwrap();
        let mut revisions = std::collections::BTreeMap::new();
        for record in [&first, second] {
            revisions
                .entry(record.source_id.clone())
                .or_insert_with(std::collections::BTreeMap::new)
                .insert(record.logical_id.clone(), record.revision);
        }
        let release = brain_contracts::CorpusRelease {
            release_id: format!("kind-consistency-{index}"),
            knowledge_version: "k1".into(),
            patch: "p1".into(),
            created_at_epoch: 1,
            source_revisions: revisions,
        };
        store.publish_release(&release).await.unwrap();
        let profile = store
            .read_entity_profile(&hero.entity_key, &release.release_id, &reader, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(profile.entity.kind, EntityKind::Hero);
        assert_eq!(
            profile.facts.len(),
            if index == 3 { 1 } else { 2 },
            "Fall {index}"
        );
    }
    for (index, same_kind) in [false, true].into_iter().enumerate() {
        let mut hero = entity(EntityKind::Hero);
        hero.entity_key = format!("concurrent-kind-{index}");
        let mut other = hero.clone();
        other.kind = if same_kind {
            EntityKind::Hero
        } else {
            EntityKind::Item
        };
        let first_ids = ["health".into()];
        let second_ids = ["damage".into()];
        let (left, right) = tokio::join!(
            store.store_entity_fact_bindings(&hero, &first, &first_ids),
            store.store_entity_fact_bindings(&other, &first, &second_ids),
        );
        assert_eq!(
            usize::from(left.is_ok()) + usize::from(right.is_ok()),
            if same_kind { 2 } else { 1 }
        );
        let identities: Vec<Value> = sqlx::query_scalar(
            "SELECT binding_identity_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1",
        )
        .bind(&hero.entity_key)
        .fetch_all(pool)
        .await
        .unwrap();
        assert_eq!(identities.len(), if same_kind { 2 } else { 1 });
        assert!(identities
            .iter()
            .all(|identity| identity["kind"] == identities[0]["kind"]));
    }
}

async fn patch_stories_follow_entity_kind(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    record: &brain_contracts::SourceRecordV2,
    release: &brain_contracts::CorpusRelease,
    principal: &brain_contracts::Principal,
) {
    sqlx::raw_sql(
        "INSERT INTO brain.patch_changes VALUES
        ('2026-09-15','hero','Gleicher Name',NULL,'hero_name','100','110','Beleg','https://example.org/patch'),
        ('2026-09-16','hero','gleicher alias',NULL,'hero_alias','100','120','Beleg','https://example.org/patch'),
        ('2026-09-16','hero','Gleicher Name','Zugehörige Fähigkeit','hero_ability','10','12','Beleg','https://example.org/patch'),
        ('2026-09-16','hero','Gleicher Alias','Fähigkeitsalias','hero_ability_alias','10','13','Beleg','https://example.org/patch'),
        ('2026-09-15','item','Gleicher Name',NULL,'item_name','100','110','Beleg','https://example.org/patch'),
        ('2026-09-16','item','gleicher alias',NULL,'item_alias','100','120','Beleg','https://example.org/patch'),
        ('2026-09-15','ability','GLEICHER NAME',NULL,'ability_name','100','110','Beleg','https://example.org/patch'),
        ('2026-09-16','ability','Gleicher Alias',NULL,'ability_alias','100','120','Beleg','https://example.org/patch'),
        ('2026-09-16','hero','Anderer Held','Gleicher Name','ability_on_hero','10','12','Beleg','https://example.org/patch'),
        ('2026-09-16','hero','Anderer Held','gleicher alias','ability_alias_on_hero','10','13','Beleg','https://example.org/patch'),
        ('2026-09-16','item','Anderes Item','Gleicher Name','foreign_item_ability',NULL,NULL,NULL,NULL),
        ('2026-09-16','item','Anderes Item','Gleicher Alias','foreign_item_ability_alias',NULL,NULL,NULL,NULL),
        ('2026-09-16','general','Gleicher Name','Gleicher Alias','foreign_general',NULL,NULL,NULL,NULL),
        ('2026-09-16','ability_internal','Gleicher Name','Gleicher Alias','foreign_internal',NULL,NULL,NULL,NULL),
        ('2026-09-16','item_special','Gleicher Alias',NULL,'foreign_special',NULL,NULL,NULL,NULL)",
    )
    .execute(pool)
    .await
    .unwrap();
    for (kind, expected) in [
        (
            EntityKind::Hero,
            vec![
                "hero_name",
                "hero_ability",
                "hero_ability_alias",
                "hero_alias",
            ],
        ),
        (
            EntityKind::Ability,
            vec![
                "ability_name",
                "ability_alias",
                "ability_alias_on_hero",
                "ability_on_hero",
            ],
        ),
        (EntityKind::Item, vec!["item_name", "item_alias"]),
    ] {
        let identity = EntityIdentity {
            entity_key: format!("history-{kind:?}"),
            kind,
            name: "Gleicher Name".into(),
            aliases: vec!["Gleicher Alias".into()],
            identity_evidence: vec!["fixture:history".into()],
        };
        store
            .store_entity_fact_bindings(&identity, record, &["health".into()])
            .await
            .unwrap();
        let current = store
            .read_entity_profile(&identity.entity_key, &release.release_id, principal, None)
            .await
            .unwrap()
            .unwrap();
        let stats: Vec<_> = current
            .patch_story
            .iter()
            .map(|change| change.stat_name.as_deref().unwrap())
            .collect();
        assert_eq!(stats, expected, "{kind:?}");
        assert_eq!(current.facts.len(), 1);
        for change in &current.patch_story {
            assert_eq!(change.provenance.relation, "brain.patch_changes");
            assert_eq!(
                change.provenance.source_url.as_deref(),
                Some("https://example.org/patch")
            );
            assert!(change
                .provenance
                .evidence_ref
                .starts_with("brain.patch_changes:"));
            assert_eq!(
                change.old_value,
                if change.ability_name.is_some() {
                    "10"
                } else {
                    "100"
                }
            );
            assert!(change.original_line.text.is_none());
            assert!(!change.original_line.redistribution_allowed);
        }
        let historical = store
            .read_entity_profile(
                &identity.entity_key,
                &release.release_id,
                principal,
                Some("2026-09-15"),
            )
            .await
            .unwrap()
            .unwrap();
        assert!(historical.facts.is_empty());
        assert!(!historical.unknowns.is_empty());
        assert_eq!(historical.patch_story.len(), 1);
        assert_eq!(historical.patch_story[0], current.patch_story[0]);
        let denied = brain_contracts::Principal {
            scopes: Default::default(),
            ..principal.clone()
        };
        assert!(store
            .read_entity_profile(&identity.entity_key, &release.release_id, &denied, None)
            .await
            .unwrap()
            .is_none());
    }
    let ability = EntityIdentity {
        entity_key: "associated-ability".into(),
        kind: EntityKind::Ability,
        name: "Zugehörige Fähigkeit".into(),
        aliases: vec!["Fähigkeitsalias".into()],
        identity_evidence: vec!["fixture:history".into()],
    };
    let story = store.entity_patch_story(&ability).await.unwrap();
    assert_eq!(
        story
            .iter()
            .map(|change| change.stat_name.as_deref().unwrap())
            .collect::<Vec<_>>(),
        vec!["hero_ability", "hero_ability_alias"]
    );
}

fn extractor_records() -> Vec<brain_contracts::SourceRecordV2> {
    use dbrain_sources::game_files::{extract_game_files, GameFileOptions};
    use dbrain_sources::wiki_inventory::{normalize_api_response, WikiSourceContext};
    let mut documents = kv1_documents(false);
    documents.extend(kv1_documents(true));
    for extension in ["txt", "json", "kv3"] {
        let root = tempfile::tempdir().unwrap();
        let content = match extension {
            "txt" => r#""hero" { "health" "12" }"#,
            "json" => r#"{"health":12}"#,
            _ => "{ health = 12 }",
        };
        std::fs::write(root.path().join(format!("health.{extension}")), content).unwrap();
        let options = GameFileOptions {
            root: root.path().into(),
            app_id: 1422450,
            source_id: "fixture".into(),
            observed_at: "2026-10-03T00:00:00Z".into(),
            build_id: None,
            manifest_id: None,
            source_revision: Some("fixture-revision".into()),
            depot_id: None,
            language: "und".into(),
            attribution: "Testdaten".into(),
            license_name: "fixture".into(),
            license_url: None,
            provenance: json!({"fixture":true}),
            max_file_bytes: 1024 * 1024,
        };
        let mut output = Vec::new();
        assert_eq!(extract_game_files(&options, &mut output).unwrap().facts, 1);
        documents.push(serde_json::from_slice::<Value>(&output).unwrap());
    }
    for (namespace, id) in [("Data", 3500), ("Bucket", 3501)] {
        let xml = format!("<mediawiki><siteinfo><base>https://deadlock.wiki/{namespace}:Heroes</base><namespaces><namespace key=\"0\"></namespace><namespace key=\"{id}\">{namespace}</namespace></namespaces></siteinfo></mediawiki>");
        let context = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        let payload = json!({"query":{"pages":[{"pageid":id,"ns":id,"title":format!("{namespace}:Heroes"),"revisions":[{"revid":18108,"slots":{"main":{"contentmodel":"json","content":"{\"health\":12}"}}}]}]}});
        documents.extend(
            normalize_api_response(&payload, &context, "2026-10-04T12:00:00Z")
                .unwrap()
                .documents,
        );
    }
    documents.into_iter().map(|document| {
        let source = document["source_id"].as_str().unwrap();
        let policy: ImportPolicy = serde_json::from_value(json!({"sources":{source:{"internal_read_allowed":true,"raw_retention_allowed":true,"publication_allowed":false,"provider_egress_allowed":false,"authorization_ref":"fixture-grant","provenance_evidence_ref":"fixture-origin","allowed_scopes":["fixture:extractor"]}}})).unwrap();
        let input = validate_knowledge_jsonl_str(&document.to_string()).unwrap();
        prepare_validated_knowledge(&input,&policy,"fixture-v1").unwrap().records()[0].record.clone()
    }).collect()
}

async fn gitblobfelder_werden_im_normalen_ableitungs_und_abrufweg_geprüft(
    store: &brain_storage::PgStore,
    pool: &sqlx::PgPool,
    socket: &std::path::Path,
) {
    use brain_contracts::store::SnapshotReadPort;
    use brain_storage::entity_profile::{
        compact::compact_document,
        derivation::{
            derive_git_profile, derived_policy, receipt_sha256, sha256,
            verify_git_document_receipt, GitBlobEvidence, GIT_DOCUMENT_CONTRACT,
        },
    };
    use dbrain_sources::{
        entity_binding::derivation::derive_git_entity_profile,
        entity_binding::{bind_stored_document, CatalogEntity},
        git_source::PinnedRepository,
    };
    use std::collections::{BTreeMap, BTreeSet};

    let directory = tempfile::tempdir().unwrap();
    let formats = [
        ("json", r#"{"hero_test":{"MaxHealth":120,"Damage":999,"Speed":1.234567890123456789e+19,"Armor":9007199254740993}}"#),
        ("kv3", "{ hero_test = { MaxHealth = 120 Damage = 999 Speed = 1.234567890123456789e+19 Armor = 9007199254740993 } }"),
        ("kv1", "hero_test { MaxHealth 120 Damage 999 Speed 1.234567890123456789e+19 Armor 9007199254740993 }")
    ];
    for (extension, content) in formats {
        std::fs::write(
            directory.path().join(format!("health.{extension}")),
            content,
        )
        .unwrap();
    }
    let upstream = "https://github.com/deadlock-wiki/deadlock-data";
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "fixture@example.org"],
        vec!["config", "user.name", "Testdaten"],
        vec!["remote", "add", "origin", upstream],
        vec!["add", "."],
        vec!["commit", "-qm", "Testdaten"],
    ] {
        assert!(std::process::Command::new("git")
            .arg("-C")
            .arg(directory.path())
            .args(args)
            .status()
            .unwrap()
            .success());
    }
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(directory.path())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let commit = String::from_utf8(output.stdout).unwrap().trim().to_owned();
    let pinned = PinnedRepository::open(directory.path(), &commit).unwrap();
    for (extension, content) in formats {
        let mut clean = None;
        for case in 0..5 {
            if extension == "kv1" && case == 4 {
                continue;
            }
            let source_id = format!("blobfield-{extension}-{case}");
            let mut document = extracted_game_document_from_source(&source_id, extension, content);
            document["revision"] = json!(format!("git:{commit}"));
            document["metadata"]["source_revision"] = json!(format!("git:{commit}"));
            document["metadata"]["provenance"]["repository_url"] = json!(upstream);
            let facts = document["facts"].as_array_mut().unwrap();
            let health = facts
                .iter_mut()
                .find(|fact| fact["fact_id"].as_str().unwrap().contains("MaxHealth"))
                .unwrap();
            match case {
                1 | 3 => {
                    health["value"] = if extension == "kv1" {
                        json!(if case == 1 { "777" } else { "999" })
                    } else {
                        json!(if case == 1 { 777 } else { 999 })
                    };
                    if extension != "kv1" {
                        health["qualifiers"]["source_lexeme"] =
                            json!(if case == 1 { "777" } else { "999" });
                    }
                }
                2 => {
                    let pointer = health["qualifiers"]["source_pointer"]
                        .as_str()
                        .unwrap()
                        .replace("MaxHealth", "Damage");
                    health["qualifiers"]["source_pointer"] = json!(pointer);
                    if extension == "json" {
                        health["qualifiers"]["json_pointer"] = json!(pointer);
                    }
                }
                4 => health["qualifiers"]["source_lexeme"] = json!("0120"),
                _ => {}
            }
            let record = prepared_game_document(&document);
            assert_eq!(
                pinned.read_blob(&format!("health.{extension}")).unwrap(),
                record.content.as_bytes()
            );
            store.apply(&record).await.unwrap();
            let mut identity = entity(EntityKind::Hero);
            identity.entity_key = format!("blobfield-{extension}");
            identity.name = format!("Blobheld {extension}");
            let catalog = [CatalogEntity {
                identity: identity.clone(),
                identifiers: vec!["hero_test".into()],
            }];
            bind_stored_document(
                store,
                &record.source_id,
                &record.logical_id,
                record.revision,
                &catalog,
            )
            .await
            .unwrap();
            let release = brain_contracts::CorpusRelease {
                release_id: source_id.clone(),
                knowledge_version: "fixture".into(),
                patch: "unknown".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    record.source_id.clone(),
                    BTreeMap::from([(record.logical_id.clone(), record.revision)]),
                )]),
            };
            store.publish_release(&release).await.unwrap();
            let operator = brain_contracts::Principal {
                actor_id: "fixture".into(),
                channel: "test".into(),
                scopes: record.allowed_scopes.clone(),
                provider_egress: BTreeSet::new(),
            };
            let repositories =
                BTreeMap::from([((source_id.clone(), commit.clone()), pinned.clone())]);
            let snapshot = store.snapshot(&release.release_id).await.unwrap();
            let bindings = store
                .stored_git_entity_bindings(&identity.entity_key, &record)
                .await
                .unwrap();
            assert_eq!(bindings.len(), 4);
            let blobs = [GitBlobEvidence {
                source_id: source_id.clone(),
                logical_id: record.logical_id.clone(),
                store_revision: record.revision,
                git_commit: commit.clone(),
                repository_url: upstream.into(),
                bytes: pinned.read_blob(&format!("health.{extension}")).unwrap(),
            }];
            if extension != "json" {
                let result = derive_git_profile(
                    &identity.entity_key,
                    &snapshot,
                    &operator,
                    &bindings,
                    &blobs,
                    &[],
                );
                if case == 0 {
                    assert!(result.is_ok());
                    let profile = store
                        .read_entity_profile(
                            &identity.entity_key,
                            &release.release_id,
                            &operator,
                            None,
                        )
                        .await
                        .unwrap()
                        .unwrap();
                    assert!(profile
                        .facts
                        .iter()
                        .any(|fact| fact.value == "1.234567890123456789e+19"));
                    assert!(profile
                        .facts
                        .iter()
                        .any(|fact| fact.value == json!(9007199254740993_u64)
                            || fact.value == "9007199254740993"));
                } else {
                    assert!(result.is_err(), "{extension}/{case}");
                }
                continue;
            }
            let result = derive_git_entity_profile(
                store,
                &release.release_id,
                &operator,
                &identity.entity_key,
                &repositories,
            )
            .await;
            if case == 0 {
                let verified = result.unwrap();
                assert!(verified
                    .profile()
                    .facts
                    .iter()
                    .any(|fact| fact.value == "1.234567890123456789e+19"
                        && fact.qualifiers["numeric_representation"] == "source_numeric_lexeme"));
                assert!(verified
                    .profile()
                    .facts
                    .iter()
                    .any(|fact| fact.value == json!(9007199254740993_u64)));
                clean = Some((verified.profile().clone(), verified.receipt().clone()));
            } else {
                assert!(result.is_err(), "{extension}/{case}");
            }
            if case == 2 || case == 4 {
                continue;
            }
            let (mut profile, mut receipt) = clean.clone().unwrap();
            if case != 0 {
                profile
                    .facts
                    .iter_mut()
                    .find(|fact| fact.predicate == "max_health")
                    .unwrap()
                    .value = json!(if case == 1 { 777 } else { 999 });
            }
            receipt.original_release_id = release.release_id.clone();
            for pin in &mut receipt.fact_pins {
                let binding = bindings
                    .iter()
                    .find(|binding| binding.original_fact.fact_id == pin.fact_id)
                    .unwrap();
                pin.source_id = source_id.clone();
                pin.binding_identity = binding.binding_identity.clone();
                pin.semantic_projection = binding.semantic_projection.clone().unwrap();
            }
            let mut derived = record.clone();
            derived.source_id = "git-game-facts-derived".into();
            derived.logical_id = identity.entity_key.clone();
            derived.content = compact_document(&profile).unwrap();
            derived.content_hash = sha256(derived.content.as_bytes());
            receipt.document_sha256 = derived.content_hash.clone();
            derived.visibility = brain_contracts::SourceVisibility::Public;
            derived.allowed_scopes.clear();
            derived.metadata = BTreeMap::from([
                (
                    "brain.entity_projection.contract".into(),
                    GIT_DOCUMENT_CONTRACT.into(),
                ),
                (
                    "brain.entity_projection.receipt_sha256".into(),
                    receipt_sha256(&receipt).unwrap(),
                ),
            ]);
            let mut origin = brain_contracts::source::origin_from_record(&record).unwrap();
            origin.identity.source_id = derived.source_id.clone();
            origin.identity.logical_id = derived.logical_id.clone();
            origin.raw_sha256 = derived.content_hash.clone();
            origin.locator = "git-game-facts-derived".into();
            origin.origin_artifacts.clear();
            origin.policy = derived_policy();
            origin.bind_record(&mut derived).unwrap();
            let saved = store
                .persist_entity_document(derived, &serde_json::to_string(&receipt).unwrap())
                .await
                .unwrap();
            let receipt_json: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
                .bind(&saved.source_id).bind(&saved.logical_id).bind(saved.revision as i64).fetch_one(pool).await.unwrap();
            let stored_receipt = serde_json::from_str(&receipt_json).unwrap();
            let checked = verify_git_document_receipt(
                &saved,
                &stored_receipt,
                &snapshot,
                &operator,
                &bindings,
                &blobs,
                &[],
            );
            if case == 0 {
                assert_eq!(checked.unwrap(), profile);
            } else {
                assert!(checked.is_err());
            }
            let mut public_release = release.clone();
            public_release.release_id = format!("{source_id}-derived");
            public_release.source_revisions.insert(
                saved.source_id.clone(),
                BTreeMap::from([(saved.logical_id.clone(), saved.revision)]),
            );
            store.publish_release(&public_release).await.unwrap();
            let operator_copy = operator.clone();
            let blob_copy = blobs[0].clone();
            let reader = tokio::task::block_in_place(|| {
                brain_storage::LocalPgReader::new(socket, 15446, "fixture", "postgres")
                    .unwrap()
                    .with_entity_profile_access(
                        move || Ok(operator_copy.clone()),
                        move |_| Ok(blob_copy.clone()),
                    )
                    .with_entity_profile_model_consumers(BTreeSet::from([(
                        "fixture".into(),
                        "test".into(),
                    )]))
                    .unwrap()
            });
            let query = brain_contracts::Query {
                answer_context: None,
                request_id: "blobfield".into(),
                conversation_id: "blobfield".into(),
                text: format!("Welche Gesundheit hat {}?", identity.name),
                requested_scopes: BTreeSet::new(),
                profile: Default::default(),
                patch: None,
                mode: None,
                domain: None,
            };
            let mut consumer = operator.clone();
            consumer.scopes = BTreeSet::from(["bot.public".into()]);
            consumer.provider_egress = BTreeSet::from(["public".into()]);
            let context = brain_contracts::AuthorizedContext {
                discord: None,
                request_deadline: None,
                principal: consumer,
                conversation_id: query.conversation_id.clone(),
                knowledge_release: public_release.release_id,
                deadline_ms: 30000,
                budget: Default::default(),
            };
            assert_eq!(
                store
                    .snapshot(&context.knowledge_release)
                    .await
                    .unwrap()
                    .authorized(&context.principal, false)
                    .unwrap()
                    .len(),
                1,
                "{saved:?}"
            );
            let evidence = tokio::task::block_in_place(|| {
                reader.read_entity_evidence(
                    &query,
                    &context,
                    None,
                    false,
                    brain_contracts::store::AnswerPurpose::InternalRead,
                )
            });
            if case == 0 {
                assert!(!evidence.unwrap().unwrap().is_empty());
            } else {
                assert!(evidence.is_err());
            }
        }
    }
}
