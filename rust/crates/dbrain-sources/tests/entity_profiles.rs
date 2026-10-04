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
    use dbrain_sources::game_files::{extract_game_files, GameFileOptions};

    let root = tempfile::tempdir().unwrap();
    let content = if extension == "json" {
        format!("{{\"health\":{number}}}")
    } else {
        format!("{{ health = {number} }}")
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
    let inventory = extract_game_files(&options, &mut output).unwrap();
    assert_eq!(inventory.facts, 1);
    let document: Value = serde_json::from_slice(&output).unwrap();
    let fact_id = document["facts"][0]["fact_id"].as_str().unwrap().to_owned();
    let input = validate_knowledge_jsonl_str(std::str::from_utf8(&output).unwrap()).unwrap();
    let policy: ImportPolicy = serde_json::from_value(json!({"sources":{"fixture":{
        "internal_read_allowed":true,"raw_retention_allowed":true,"publication_allowed":false,"provider_egress_allowed":false,"authorization_ref":"fixture-grant","provenance_evidence_ref":"fixture-origin","allowed_scopes":[]
    }}})).unwrap();
    let prepared = prepare_validated_knowledge(&input, &policy, "fixture-v1").unwrap();
    project_entity_facts(&prepared.records()[0].record, &[fact_id])
        .unwrap()
        .remove(0)
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
}
#[test]
fn validity_is_open_or_exclusive_and_unknown_is_not_backdated() {
    let mut validity = PatchValidity::Known {
        from_patch: "2026-09-16".into(),
        to_patch_exclusive: None,
        evidence_ref: "patch-fixture".into(),
    };
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
#[tokio::test]
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
        .max_connections(1)
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
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-04-brain-entity-profiles-v1.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-04-brain-entity-profile-binding-identity-v1.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
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
    assert_eq!(visible.entity, entity);
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
        entity
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
        assert_eq!(actual.entity, identity);
        assert_eq!(actual.facts, project_entity_facts(&record, &ids).unwrap());
    }
    pool.close().await;
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
            assert_eq!(change.original_line.text.as_deref(), Some("Beleg"));
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
    let mut documents = Vec::new();
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
