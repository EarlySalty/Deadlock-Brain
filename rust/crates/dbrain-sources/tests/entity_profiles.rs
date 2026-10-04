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
        "numeric_representation".into(),
        json!("source_numeric_lexeme"),
    );
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
    let mut record = record();
    let mut origin = brain_contracts::source::origin_from_record(&record).unwrap();
    record.allowed_scopes.insert("fixture:read".into());
    origin.policy.allowed_scopes = record.allowed_scopes.clone();
    origin.bind_record(&mut record).unwrap();
    store.apply(&record).await.unwrap();
    let entity = entity(EntityKind::Hero);
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
    sqlx::raw_sql("CREATE TABLE brain.patch_changes(patch_date text, entity_name text, ability_name text, stat_name text)").execute(&pool).await.unwrap();
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
        .unwrap()
        .unwrap();
    assert!(hidden.facts.is_empty());
    principal.scopes = record.allowed_scopes.clone();
    let visible = store
        .read_entity_profile(&entity.entity_key, &release.release_id, &principal, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(visible.facts.len(), 1);
    assert_eq!(
        visible.facts[0].value.to_string(),
        "1.2345678901234567890123456789e+19"
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
        .unwrap()
        .unwrap();
    assert!(hidden.facts.is_empty());
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
    pool.close().await;
}
