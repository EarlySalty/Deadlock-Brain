use brain_contracts::entity_profile::{EntityIdentity, EntityKind};
use brain_storage::entity_profile::{
    assemble_profile, project_entity_facts,
    semantic::{project_semantic_fact, semantic_projection},
};
use dbrain_sources::{
    entity_binding::{bind_stored_document, CatalogEntity},
    knowledge_contract::{sha256_content, validate_knowledge_jsonl_str},
    knowledge_import::{prepare_validated_knowledge, ImportPolicy},
};
use serde_json::{json, Value};

fn prepared(document: &Value) -> brain_contracts::SourceRecordV2 {
    let input = validate_knowledge_jsonl_str(&document.to_string()).unwrap();
    let source = document["source_id"].as_str().unwrap();
    let policy: ImportPolicy = serde_json::from_value(json!({"sources":{source:{
        "internal_read_allowed":true,"raw_retention_allowed":true,"publication_allowed":false,
        "provider_egress_allowed":false,"authorization_ref":"fixture-grant","provenance_evidence_ref":"fixture-origin","allowed_scopes":[]
    }}})).unwrap();
    prepare_validated_knowledge(&input, &policy, "fixture-v1")
        .unwrap()
        .records()[0]
        .record
        .clone()
}

fn document(kind: &str, number: Value, variant: &str) -> Value {
    let document_id = if kind == "wiki" {
        "wiki:wiki:page:123"
    } else {
        "game:1422450:health.json"
    };
    let locator = if kind == "wiki" {
        "https://example.org/Test"
    } else {
        "health.json"
    };
    json!({"contract_version":"wiki-spielwissen-v1","source_kind":kind,"source_id":kind,
        "document_id":document_id,"source_locator":locator,"title":"Test","language":"und","revision":"456",
        "observed_at":"2026-10-03T12:00:00Z","content_sha256":sha256_content("Beleg"),"content":"Beleg","evidence_status":"extracted_value",
        "license":{"name":"unverified","url":null,"attribution":"Testdaten","redistribution_allowed":false},"metadata":{},
        "facts":[{"fact_id":"health","subject":"game_file:test","predicate":"file.json_value","value":number,"unit":"hp","evidence_status":"extracted_value",
            "source_span":"/hero_test/MaxHealth","qualifiers":{"source_pointer":"/hero_test/MaxHealth","source_lexeme":"erhalten","numeric_representation":"source_numeric_lexeme","gameplay_binding":"uninterpreted","variant":variant}}]})
}

fn identity(kind: EntityKind, key: &str, name: &str) -> EntityIdentity {
    EntityIdentity {
        entity_key: key.into(),
        kind,
        name: name.into(),
        aliases: vec![],
        identity_evidence: vec![format!("fixture:identity:{key}")],
    }
}

#[test]
fn canonical_numeric_conflicts_keep_originals_variants_and_exponents() {
    let originals = [
        project_entity_facts(
            &prepared(&document("game_file", json!("1.200e+19"), "normal")),
            &["health".into()],
        )
        .unwrap()
        .remove(0),
        project_entity_facts(
            &prepared(&document("wiki", json!("1.300e+19"), "normal")),
            &["health".into()],
        )
        .unwrap()
        .remove(0),
    ];
    let projected: Vec<_> = originals
        .iter()
        .map(|fact| {
            project_semantic_fact(
                fact,
                &semantic_projection(fact, "/MaxHealth").unwrap().unwrap(),
            )
            .unwrap()
        })
        .collect();
    assert_eq!(projected[0].value, "1.200e+19");
    assert_eq!(projected[0].predicate, "max_health");
    assert_eq!(originals[0].predicate, "file.json_value");
    assert_eq!(originals[0].qualifiers["source_lexeme"], "erhalten");
    assert!(!projected[0].qualifiers.contains_key("source_pointer"));
    assert_eq!(
        assemble_profile(
            identity(EntityKind::Hero, "hero_test", "Test"),
            None,
            projected.clone(),
            vec![]
        )
        .conflicts
        .len(),
        1
    );
    let mut variants = projected;
    variants[1]
        .qualifiers
        .insert("variant".into(), json!("verstärkt"));
    assert!(assemble_profile(
        identity(EntityKind::Hero, "hero_test", "Test"),
        None,
        variants,
        vec![]
    )
    .conflicts
    .is_empty());
    let mut projection = semantic_projection(&originals[0], "/MaxHealth")
        .unwrap()
        .unwrap();
    projection.predicate = "damage".into();
    assert!(project_semantic_fact(&originals[0], &projection).is_err());
    assert!(semantic_projection(&originals[0], "/Damage").is_err());
}

struct ScratchPg {
    directory: tempfile::TempDir,
}
impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.path().join("data"))
            .args(["-m", "fast", "-w", "stop"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
}

#[tokio::test]
#[ignore = "Externe Datenprobe: benötigt private eingefrorene Git-JSONL und lokale PostgreSQL-16-Werkzeuge"]
async fn actual_git_documents_bind_all_kinds_idempotently_to_immutable_originals() {
    let pg = ScratchPg {
        directory: tempfile::tempdir().unwrap(),
    };
    let data = pg.directory.path().join("data");
    let socket = pg.directory.path().join("socket");
    std::fs::create_dir(&socket).unwrap();
    assert!(
        std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
            .arg("-D")
            .arg(&data)
            .args(["-A", "trust", "-U", "fixture", "--no-locale"])
            .stdout(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    );
    assert!(
        std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(&data)
            .arg("-l")
            .arg(pg.directory.path().join("postgres.log"))
            .args([
                "-o",
                &format!("-k {} -p 15449 -c listen_addresses=''", socket.display()),
                "-w",
                "start"
            ])
            .stdout(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(15449)
                .username("fixture")
                .database("postgres"),
        )
        .await
        .unwrap();
    let store = brain_storage::PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    for migration in [include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profiles-v1.sql"),include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profile-binding-identity-v1.sql"),include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-semantic-projection-v1.sql"),include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-derived-receipts-v1.sql")] {
        sqlx::raw_sql(migration).execute(&pool).await.unwrap();
    }
    let catalog: Vec<_> = [
        (EntityKind::Hero, "hero_inferno", "Infernus"),
        (
            EntityKind::Ability,
            "ability_incendiary_projectile",
            "Napalm",
        ),
        (EntityKind::Item, "upgrade_clip_size", "Extended Magazine"),
    ]
    .into_iter()
    .map(|(kind, key, name)| CatalogEntity {
        identity: identity(kind, key, name),
        identifiers: vec![key.into()],
    })
    .collect();
    sqlx::raw_sql("CREATE TABLE brain.patch_changes(entity_type text,entity_name text,ability_name text,stat_name text,patch_date text,old_value text,new_value text,unit text,variant text); INSERT INTO brain.patch_changes VALUES('hero','Test',NULL,'Max Health','2026-09-01','100','110','hp','normal'),('hero','Test',NULL,'Max Health','2026-09-16','110','120','hp','normal'),('item','Test',NULL,'Max Health','2026-09-16','900','999','hp','normal')").execute(&pool).await.unwrap();
    let mut interval_document = document("game_file", json!("120"), "normal");
    let mut patch_fact = interval_document["facts"][0].clone();
    patch_fact["fact_id"] = json!("current_patch");
    patch_fact["predicate"] = json!("current_patch");
    patch_fact["value"] = json!("2026-09-30");
    patch_fact["unit"] = Value::Null;
    patch_fact["qualifiers"] = json!({"source_pointer":"/hero_test/CurrentPatch"});
    interval_document["facts"]
        .as_array_mut()
        .unwrap()
        .push(patch_fact);
    let interval_record = prepared(&interval_document);
    store.apply(&interval_record).await.unwrap();
    let interval_entity = identity(EntityKind::Hero, "hero_test", "Test");
    let interval_catalog = vec![CatalogEntity {
        identity: interval_entity.clone(),
        identifiers: vec!["hero_test".into()],
    }];
    bind_stored_document(
        &store,
        &interval_record.source_id,
        &interval_record.logical_id,
        interval_record.revision,
        &interval_catalog,
    )
    .await
    .unwrap();
    assert_eq!(
        store
            .store_entity_patch_intervals("hero_test", &interval_record, "health", "current_patch")
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .store_entity_patch_intervals("hero_test", &interval_record, "health", "current_patch")
            .await
            .unwrap(),
        0
    );
    let encoded: String = sqlx::query_scalar(
        "SELECT interval_json FROM brain.entity_patch_intervals_v1 WHERE entity_key='hero_test'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let intervals: brain_storage::entity_profile::intervals::IntervalProjection =
        serde_json::from_str(&encoded).unwrap();
    assert_eq!(intervals.intervals.len(), 2);
    let originals =
        project_entity_facts(&interval_record, &["health".into(), "current_patch".into()]).unwrap();
    let semantic = semantic_projection(&originals[0], "/MaxHealth")
        .unwrap()
        .unwrap();
    let story = store.entity_patch_story(&interval_entity).await.unwrap();
    let previous = brain_storage::entity_profile::intervals::project_interval_fact(
        &interval_entity,
        &originals[0],
        &originals[1],
        &semantic,
        &intervals,
        &story,
        "2026-09-15",
    )
    .unwrap()
    .unwrap();
    assert_eq!(previous.value, "110");
    assert_eq!(
        previous.provenance.document_metadata["derived_interval_evidence"]["patch_evidence"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        brain_storage::entity_profile::intervals::project_interval_fact(
            &interval_entity,
            &originals[0],
            &originals[1],
            &semantic,
            &intervals,
            &story,
            "2026-08-31"
        )
        .unwrap()
        .is_none()
    );
    let mut missing = originals[1].clone();
    missing.predicate = "file.json_value".into();
    assert!(
        brain_storage::entity_profile::intervals::derive_patch_intervals(
            &interval_entity,
            &originals[0],
            &missing,
            &semantic,
            &story
        )
        .is_err()
    );
    sqlx::query("UPDATE brain.patch_changes SET new_value='121' WHERE entity_type='hero' AND patch_date='2026-09-16'").execute(&pool).await.unwrap();
    let changed_story = store.entity_patch_story(&interval_entity).await.unwrap();
    assert!(
        brain_storage::entity_profile::intervals::project_interval_fact(
            &interval_entity,
            &originals[0],
            &originals[1],
            &semantic,
            &intervals,
            &changed_story,
            "2026-09-15"
        )
        .is_err()
    );
    use std::io::BufRead;
    let file=std::fs::File::open("/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/run-CrJzHxm4/deadlock-data-0d46cdecfccf77adec16aac01af6d30173e0ebb8.jsonl").unwrap();
    let mut count = 0;
    let mut bound = 0;
    let mut numeric = 0;
    let mut actual_records = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let document: Value = serde_json::from_str(&line.unwrap()).unwrap();
        if !matches!(
            document["title"].as_str(),
            Some(
                "data/json/hero-data.json"
                    | "data/json/ability-data.json"
                    | "data/json/item-data.json"
            )
        ) {
            continue;
        }
        let record = prepared(&document);
        store.apply(&record).await.unwrap();
        let first = bind_stored_document(
            &store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        let retry = bind_stored_document(
            &store,
            &record.source_id,
            &record.logical_id,
            record.revision,
            &catalog,
        )
        .await
        .unwrap();
        assert_eq!(first.bound_facts, retry.bound_facts);
        assert_eq!(retry.inserted_bindings, 0);
        assert_eq!(retry.inserted_projections, 0);
        assert!(first.inserted_projections > 0);
        assert!(!first.entity_keys.is_empty());
        let originals:Vec<(String,String)>=sqlx::query_as("SELECT fact_id,fact_json FROM brain.entity_profile_facts_v1 WHERE source_id=$1 AND logical_id=$2").bind(&record.source_id).bind(&record.logical_id).fetch_all(&pool).await.unwrap();
        let ids: Vec<_> = originals.iter().map(|(id, _)| id.clone()).collect();
        let projected = project_entity_facts(&record, &ids).unwrap();
        for ((_, encoded), original) in originals.iter().zip(projected) {
            assert_eq!(
                serde_json::from_str::<brain_contracts::entity_profile::EntityProfileFact>(encoded)
                    .unwrap(),
                original
            );
        }
        let mut changed = record.clone();
        let mut changed_document = document.clone();
        changed_document["facts"][0]["value"] = json!("verändert");
        changed.metadata.insert(
            brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(),
            changed_document.to_string(),
        );
        let fact = project_entity_facts(&record, &ids[..1]).unwrap().remove(0);
        assert!(store
            .store_entity_fact_bindings(&catalog[0].identity, &changed, &[fact.fact_id])
            .await
            .is_err());
        bound += first.bound_facts;
        numeric += first.inserted_projections;
        count += 1;
        actual_records.push(record);
    }
    assert_eq!(count, 3);
    let actual_release = brain_contracts::CorpusRelease {
        release_id: "actual-frozen-git-release".into(),
        knowledge_version: "fixture-v1".into(),
        patch: "unknown".into(),
        created_at_epoch: 1,
        source_revisions: std::collections::BTreeMap::from([(
            actual_records[0].source_id.clone(),
            actual_records
                .iter()
                .map(|record| (record.logical_id.clone(), record.revision))
                .collect(),
        )]),
    };
    store.publish_release(&actual_release).await.unwrap();
    let actual_operator = brain_contracts::Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: actual_records
            .iter()
            .flat_map(|record| record.allowed_scopes.iter().cloned())
            .collect(),
        provider_egress: Default::default(),
    };
    let actual_commit = "0d46cdecfccf77adec16aac01af6d30173e0ebb8";
    let actual_repository = dbrain_sources::git_source::PinnedRepository::open(
        std::path::Path::new("/home/nathanael/repos/Deadlock-Brain/data/external/deadlock-data"),
        actual_commit,
    )
    .unwrap();
    let actual_repositories = std::collections::BTreeMap::from([(
        (actual_records[0].source_id.clone(), actual_commit.into()),
        actual_repository,
    )]);
    let mut actual_profiles = Vec::new();
    for entry in &catalog {
        let derived = dbrain_sources::entity_binding::derivation::derive_git_entity_profile(
            &store,
            &actual_release.release_id,
            &actual_operator,
            &entry.identity.entity_key,
            &actual_repositories,
        )
        .await
        .unwrap();
        let compact =
            brain_storage::entity_profile::compact::compact_document(derived.profile()).unwrap();
        assert!(!compact.contains("semantic_scope"));
        assert!(!compact.contains("source_pointer"));
        for predicate in ["max_health", "bonus_clip_size_percent"] {
            if let Some(fact) = derived
                .profile()
                .facts
                .iter()
                .find(|fact| fact.predicate == predicate)
            {
                println!(
                    "Bereinigter eingefrorener Originalwert: {} {}={}; erhaltene Zahlen: {}",
                    derived.profile().entity.name,
                    predicate,
                    fact.value,
                    derived.profile().facts.len()
                );
            }
        }
        let document = derived_document(derived.profile(), derived.receipt());
        store.apply(&document).await.unwrap();
        sqlx::query("INSERT INTO brain.entity_derived_receipts_v1(derived_source_id,derived_logical_id,derived_revision,receipt_json) VALUES($1,$2,$3,$4)")
            .bind(&document.source_id).bind(&document.logical_id).bind(document.revision as i64).bind(serde_json::to_string(derived.receipt()).unwrap()).execute(&pool).await.unwrap();
        let encoded: String = sqlx::query_scalar("SELECT receipt_json FROM brain.entity_derived_receipts_v1 WHERE derived_source_id=$1 AND derived_logical_id=$2 AND derived_revision=$3")
            .bind(&document.source_id).bind(&document.logical_id).bind(document.revision as i64).fetch_one(&pool).await.unwrap();
        let stored_receipt: brain_storage::entity_profile::derivation::GitDocumentReceipt =
            serde_json::from_str(&encoded).unwrap();
        assert_eq!(&stored_receipt, derived.receipt());
        let snapshot = store.snapshot(&actual_release.release_id).await.unwrap();
        let mut actual_blobs = Vec::new();
        let mut actual_seen = std::collections::BTreeSet::new();
        for binding in derived.original_pins() {
            if !actual_seen.insert((
                binding.source_id.clone(),
                binding.logical_id.clone(),
                binding.store_revision,
            )) {
                continue;
            }
            let record = snapshot
                .revisions
                .iter()
                .find(|record| {
                    record.source_id == binding.source_id
                        && record.logical_id == binding.logical_id
                        && record.revision == binding.store_revision
                })
                .unwrap();
            let (commit, repository, path) =
                brain_storage::entity_profile::derivation::git_document_identity(record).unwrap();
            let pinned = &actual_repositories[&(record.source_id.clone(), commit.clone())];
            pinned
                .require_origin(&[repository.as_str(), &format!("{repository}.git")])
                .unwrap();
            actual_blobs.push(brain_storage::entity_profile::derivation::GitBlobEvidence {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
                store_revision: record.revision,
                git_commit: commit,
                repository_url: repository,
                bytes: pinned.read_blob(&path).unwrap(),
            });
        }
        let story = store.entity_patch_story(&entry.identity).await.unwrap();
        brain_storage::entity_profile::derivation::verify_git_document_receipt(
            &document,
            &stored_receipt,
            &snapshot,
            &actual_operator,
            derived.original_pins(),
            &actual_blobs,
            &story,
        )
        .unwrap();
        actual_profiles.push(derived.profile().clone());
    }
    std::fs::write(
        "/tmp/brain-a3-f1-real-profiles.json",
        serde_json::to_vec(&actual_profiles).unwrap(),
    )
    .unwrap();
    let repository = pg.directory.path().join("repository");
    std::fs::create_dir(&repository).unwrap();
    std::fs::write(
        repository.join("health.json"),
        "{\"hero_test\":{\"MaxHealth\":120}}",
    )
    .unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "fixture@example.org"],
        vec!["config", "user.name", "Testdaten"],
        vec![
            "remote",
            "add",
            "origin",
            "https://github.com/deadlock-wiki/deadlock-data",
        ],
        vec!["add", "health.json"],
        vec!["commit", "-qm", "Testdaten"],
    ] {
        assert!(std::process::Command::new("git")
            .arg("-C")
            .arg(&repository)
            .args(args)
            .status()
            .unwrap()
            .success());
    }
    let commit = String::from_utf8(
        std::process::Command::new("git")
            .arg("-C")
            .arg(&repository)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    let options = dbrain_sources::game_files::GameFileOptions {
        root: repository.clone(),
        app_id: 1422450,
        source_id: "derived-fixture".into(),
        observed_at: "2026-10-04T00:00:00Z".into(),
        build_id: None,
        manifest_id: None,
        source_revision: Some(commit.clone()),
        depot_id: None,
        language: "und".into(),
        attribution: "Testdaten".into(),
        license_name: "fixture".into(),
        license_url: None,
        provenance: json!({"repository_url":"https://github.com/deadlock-wiki/deadlock-data"}),
        max_file_bytes: 1024 * 1024,
    };
    let mut extracted = Vec::new();
    dbrain_sources::game_files::extract_game_files(&options, &mut extracted).unwrap();
    let document: Value = serde_json::from_slice(&extracted).unwrap();
    let mut derived_record = prepared(&document);
    let mut origin = brain_contracts::source::origin_from_record(&derived_record).unwrap();
    derived_record
        .allowed_scopes
        .insert("source.review:derived-fixture".into());
    origin.policy.allowed_scopes = derived_record.allowed_scopes.clone();
    origin.bind_record(&mut derived_record).unwrap();
    store.apply(&derived_record).await.unwrap();
    bind_stored_document(
        &store,
        &derived_record.source_id,
        &derived_record.logical_id,
        derived_record.revision,
        &interval_catalog,
    )
    .await
    .unwrap();
    let release = brain_contracts::CorpusRelease {
        release_id: "derived-fixture-release".into(),
        knowledge_version: "fixture-v1".into(),
        patch: "unknown".into(),
        created_at_epoch: 1,
        source_revisions: std::collections::BTreeMap::from([(
            derived_record.source_id.clone(),
            std::collections::BTreeMap::from([(
                derived_record.logical_id.clone(),
                derived_record.revision,
            )]),
        )]),
    };
    store.publish_release(&release).await.unwrap();
    let mut operator = brain_contracts::Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: derived_record.allowed_scopes.clone(),
        provider_egress: Default::default(),
    };
    let pinned = dbrain_sources::git_source::PinnedRepository::open(&repository, &commit).unwrap();
    let repositories = std::collections::BTreeMap::from([(
        (derived_record.source_id.clone(), commit.clone()),
        pinned,
    )]);
    let derived = dbrain_sources::entity_binding::derivation::derive_git_entity_profile(
        &store,
        &release.release_id,
        &operator,
        "hero_test",
        &repositories,
    )
    .await
    .unwrap();
    assert_eq!(derived.profile().facts.len(), 1);
    assert_eq!(derived.profile().entity.name, "Test");
    assert!(derived.profile().entity.aliases.is_empty());
    assert_eq!(
        derived.policy().visibility,
        brain_contracts::SourceVisibility::Public
    );
    assert!(derived.policy().provider_egress_allowed);
    assert_eq!(
        derived.original_pins()[0].original_fact.predicate,
        "file.json_value"
    );
    assert_eq!(
        derived.original_pins()[0].binding_identity.entity_key,
        "hero_test"
    );
    assert!(!derived.profile().facts[0]
        .qualifiers
        .contains_key("source_pointer"));
    operator.scopes.clear();
    assert!(
        dbrain_sources::entity_binding::derivation::derive_git_entity_profile(
            &store,
            &release.release_id,
            &operator,
            "hero_test",
            &repositories
        )
        .await
        .is_err()
    );
    let health:String=sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1 WHERE entity_key='hero_inferno' AND fact_id='json:/hero_inferno/MaxHealth'").fetch_one(&pool).await.unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&health).unwrap()["value"],
        "830.0"
    );
    println!("Echte eingefrorene Git-Dokumente: {count}, Entitäten: 1 Held, 1 Fähigkeit, 1 Item; Originalfakten: {bound}, numerische Projektionen: {numeric}; Infernus MaxHealth=830. Aktueller Patchanker fehlt.");
    pool.close().await;
}

fn story_change(
    date: &str,
    old: i64,
    new: i64,
) -> brain_contracts::entity_profile::PatchStoryChange {
    use brain_contracts::entity_profile::{
        PatchStoryChange, PatchStoryProvenance, RestrictedPatchLine,
    };
    PatchStoryChange {
        patch_date: date.into(),
        patch_title: Some("Testpatch".into()),
        entity_type: Some("hero".into()),
        entity_name: Some("Test".into()),
        ability_name: None,
        stat_name: Some("Max Health".into()),
        old_value: json!(old),
        new_value: json!(new),
        change_type: Some("changed".into()),
        numeric_direction: Some("increase".into()),
        confidence: json!(1),
        provenance: PatchStoryProvenance {
            relation: "brain.patch_changes".into(),
            source_url: None,
            evidence_ref: format!("brain.patch_changes:{date}:hero_test"),
        },
        original_line: RestrictedPatchLine {
            text: Some("Privater Originaltext".into()),
            redistribution_allowed: false,
        },
        additional_fields: serde_json::from_value(json!({"unit":"hp","variant":"normal"})).unwrap(),
    }
}

#[test]
fn interval_chain_ignores_unrelated_rows_but_rechecks_participating_evidence() {
    use brain_storage::entity_profile::intervals::{derive_patch_intervals, project_interval_fact};
    let record = prepared(&document("game_file", json!(120), "normal"));
    let anchor = project_entity_facts(&record, &["health".into()])
        .unwrap()
        .remove(0);
    let mut patch = anchor.clone();
    patch.fact_id = "patch".into();
    patch.predicate = "current_patch".into();
    patch.value = json!("2026-09-30");
    let semantic = semantic_projection(&anchor, "/MaxHealth").unwrap().unwrap();
    let entity = identity(EntityKind::Hero, "hero_test", "Test");
    let mut story = vec![
        story_change("2026-09-01", 100, 110),
        story_change("2026-09-16", 110, 120),
    ];
    let stored = derive_patch_intervals(&entity, &anchor, &patch, &semantic, &story).unwrap();
    story.push(story_change("2026-10-01", 120, 999));
    for (key, value) in [
        ("unit", json!("seconds")),
        ("variant", json!("verstärkt")),
        ("condition", json!("Im Sprung")),
        ("level", json!(2)),
        ("semantic_scope", json!("Variants/1")),
    ] {
        let mut other = story_change("2026-09-16", 900, 999);
        other.additional_fields.insert(key.into(), value);
        story.push(other);
    }
    let mut item = story_change("2026-09-16", 900, 999);
    item.entity_type = Some("item".into());
    story.push(item);
    assert_eq!(
        derive_patch_intervals(&entity, &anchor, &patch, &semantic, &story).unwrap(),
        stored
    );
    assert_eq!(
        project_interval_fact(
            &entity,
            &anchor,
            &patch,
            &semantic,
            &stored,
            &story,
            "2026-09-15"
        )
        .unwrap()
        .unwrap()
        .value,
        json!(110)
    );
    assert!(project_interval_fact(
        &entity,
        &anchor,
        &patch,
        &semantic,
        &stored,
        &story,
        "2026-08-31"
    )
    .unwrap()
    .is_none());
    story[1].new_value = json!(121);
    assert!(project_interval_fact(
        &entity,
        &anchor,
        &patch,
        &semantic,
        &stored,
        &story,
        "2026-09-15"
    )
    .is_err());
    patch.predicate = "file.json_value".into();
    assert!(derive_patch_intervals(&entity, &anchor, &patch, &semantic, &story).is_err());
}

struct ReceiptFixture {
    _directories: Vec<tempfile::TempDir>,
    snapshot: brain_contracts::CorpusSnapshot,
    operator: brain_contracts::Principal,
    bindings: Vec<brain_storage::entity_profile::derivation::StoredGitBinding>,
    blobs: Vec<brain_storage::entity_profile::derivation::GitBlobEvidence>,
}

fn receipt_fixture() -> ReceiptFixture {
    use brain_storage::entity_profile::derivation::{GitBlobEvidence, StoredGitBinding};
    let mut directories = Vec::new();
    let mut records = Vec::new();
    let mut bindings = Vec::new();
    let mut blobs = Vec::new();
    for (source, upstream, number) in [
        (
            "deadlock-data",
            "https://github.com/deadlock-wiki/deadlock-data",
            120,
        ),
        (
            "GameTracking",
            "https://github.com/SteamTracking/GameTracking-Deadlock",
            125,
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(
            directory.path().join("health.json"),
            format!("{{\"hero_test\":{{\"MaxHealth\":{number}}}}}"),
        )
        .unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.email", "fixture@example.org"],
            vec!["config", "user.name", "Testdaten"],
            vec!["remote", "add", "origin", upstream],
            vec!["add", "health.json"],
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
        let commit = String::from_utf8(
            std::process::Command::new("git")
                .arg("-C")
                .arg(directory.path())
                .args(["rev-parse", "HEAD"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_owned();
        let options = dbrain_sources::game_files::GameFileOptions {
            root: directory.path().into(),
            app_id: 1422450,
            source_id: source.into(),
            observed_at: "2026-10-04T00:00:00Z".into(),
            build_id: None,
            manifest_id: None,
            source_revision: Some(commit.clone()),
            depot_id: None,
            language: "und".into(),
            attribution: "Testdaten".into(),
            license_name: "fixture".into(),
            license_url: None,
            provenance: json!({"repository_url":upstream}),
            max_file_bytes: 1024 * 1024,
        };
        let mut extracted = Vec::new();
        dbrain_sources::game_files::extract_game_files(&options, &mut extracted).unwrap();
        let doc: Value = serde_json::from_slice(&extracted).unwrap();
        let record = prepared(&doc);
        let fact = project_entity_facts(&record, &["json:/hero_test/MaxHealth".into()])
            .unwrap()
            .remove(0);
        let projection = semantic_projection(&fact, "/MaxHealth").unwrap().unwrap();
        let pinned =
            dbrain_sources::git_source::PinnedRepository::open(directory.path(), &commit).unwrap();
        pinned.require_origin(&[upstream]).unwrap();
        blobs.push(GitBlobEvidence {
            source_id: source.into(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
            git_commit: commit,
            repository_url: upstream.into(),
            bytes: pinned.read_blob("health.json").unwrap(),
        });
        bindings.push(StoredGitBinding {
            source_id: source.into(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
            original_fact: fact,
            binding_identity: identity(EntityKind::Hero, "hero_test", "Test"),
            semantic_projection: Some(projection),
        });
        records.push(record);
        directories.push(directory);
    }
    let revisions = records
        .iter()
        .map(|record| {
            (
                record.source_id.clone(),
                std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
            )
        })
        .collect();
    let snapshot = brain_contracts::CorpusSnapshot {
        release: brain_contracts::CorpusRelease {
            release_id: "fixture-release".into(),
            knowledge_version: "fixture-v1".into(),
            patch: "unknown".into(),
            created_at_epoch: 1,
            source_revisions: revisions,
        },
        revisions: records.clone(),
        heads: records,
    };
    let operator = brain_contracts::Principal {
        actor_id: "fixture".into(),
        channel: "test".into(),
        scopes: snapshot
            .revisions
            .iter()
            .flat_map(|record| record.allowed_scopes.iter().cloned())
            .collect(),
        provider_egress: Default::default(),
    };
    ReceiptFixture {
        _directories: directories,
        snapshot,
        operator,
        bindings,
        blobs,
    }
}

fn derived_document(
    profile: &brain_contracts::entity_profile::EntityProfile,
    receipt: &brain_storage::entity_profile::derivation::GitDocumentReceipt,
) -> brain_contracts::SourceRecordV2 {
    use brain_storage::entity_profile::{
        compact::compact_document,
        derivation::{derived_policy, receipt_sha256, GIT_DOCUMENT_CONTRACT},
    };
    let mut record = brain_contracts::SourceRecordV2 {
        source_id: "git-game-facts-derived".into(),
        logical_id: profile.entity.entity_key.clone(),
        revision: 1,
        content_hash: receipt.document_sha256.clone(),
        content: compact_document(profile).unwrap(),
        visibility: brain_contracts::SourceVisibility::Public,
        allowed_scopes: Default::default(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: std::collections::BTreeMap::from([
            (
                "brain.entity_projection.contract".into(),
                GIT_DOCUMENT_CONTRACT.into(),
            ),
            (
                "brain.entity_projection.receipt_sha256".into(),
                receipt_sha256(receipt).unwrap(),
            ),
        ]),
    };
    let mut origin = profile.facts[0].provenance.origin.clone();
    origin.raw_sha256 = record.content_hash.clone();
    origin.policy = derived_policy();
    origin.bind_record(&mut record).unwrap();
    record
}

#[test]
fn document_receipt_reconstructs_both_git_sources_story_and_exact_output() {
    use brain_storage::entity_profile::derivation::{
        derive_git_profile, verify_git_document_receipt,
    };
    let fixture = receipt_fixture();
    let story = vec![story_change("2026-09-16", 110, 120)];
    let (profile, receipt) = derive_git_profile(
        "hero_test",
        &fixture.snapshot,
        &fixture.operator,
        &fixture.bindings,
        &fixture.blobs,
        &story,
    )
    .unwrap();
    assert_eq!(profile.facts.len(), 2);
    assert_eq!(receipt.fact_pins.len(), 2);
    assert_eq!(profile.patch_story.len(), 1);
    let record = derived_document(&profile, &receipt);
    assert!(!record.content.contains("Privater Originaltext"));
    assert!(!record.content.contains("original_relative_path"));
    assert!(profile
        .facts
        .iter()
        .all(|fact| fact.provenance.origin.policy.provider_egress_allowed));
    assert_eq!(
        verify_git_document_receipt(
            &record,
            &receipt,
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &fixture.blobs,
            &story
        )
        .unwrap(),
        profile
    );
    let mut reordered = fixture.bindings.clone();
    reordered.reverse();
    assert_eq!(
        verify_git_document_receipt(
            &record,
            &receipt,
            &fixture.snapshot,
            &fixture.operator,
            &reordered,
            &fixture.blobs,
            &story
        )
        .unwrap(),
        profile
    );
    let mut changed_story = story.clone();
    let mut variant_story = story[0].clone();
    variant_story.new_value = json!(150);
    variant_story
        .additional_fields
        .insert("variant".into(), json!("verstärkt"));
    let mut multiple_story = vec![story[0].clone(), variant_story];
    let (multiple_profile, multiple_receipt) = derive_git_profile(
        "hero_test",
        &fixture.snapshot,
        &fixture.operator,
        &fixture.bindings,
        &fixture.blobs,
        &multiple_story,
    )
    .unwrap();
    let multiple_document = derived_document(&multiple_profile, &multiple_receipt);
    multiple_story.reverse();
    assert_eq!(
        verify_git_document_receipt(
            &multiple_document,
            &multiple_receipt,
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &fixture.blobs,
            &multiple_story
        )
        .unwrap(),
        multiple_profile
    );
    assert!(multiple_document.content.contains("verstärkt"));
    changed_story[0].new_value = json!(121);
    assert!(verify_git_document_receipt(
        &record,
        &receipt,
        &fixture.snapshot,
        &fixture.operator,
        &fixture.bindings,
        &fixture.blobs,
        &changed_story
    )
    .is_err());
    for index in 0..2 {
        let mut missing = fixture.blobs.clone();
        missing.remove(index);
        assert!(derive_git_profile(
            "hero_test",
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &missing,
            &story
        )
        .is_err());
        let mut wrong = fixture.blobs.clone();
        wrong[index].git_commit = "a".repeat(40);
        assert!(derive_git_profile(
            "hero_test",
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &wrong,
            &story
        )
        .is_err());
        wrong = fixture.blobs.clone();
        wrong[index].bytes.push(b'x');
        assert!(derive_git_profile(
            "hero_test",
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &wrong,
            &story
        )
        .is_err());
        wrong = fixture.blobs.clone();
        wrong[index].repository_url = "https://example.org/privat".into();
        assert!(derive_git_profile(
            "hero_test",
            &fixture.snapshot,
            &fixture.operator,
            &fixture.bindings,
            &wrong,
            &story
        )
        .is_err());
    }
}

#[test]
fn document_receipt_rejects_stale_rights_forged_bindings_and_marker_only_claims() {
    use brain_storage::entity_profile::derivation::{
        derive_git_profile, verify_git_document_receipt,
    };
    let fixture = receipt_fixture();
    let (profile, receipt) = derive_git_profile(
        "hero_test",
        &fixture.snapshot,
        &fixture.operator,
        &fixture.bindings,
        &fixture.blobs,
        &[],
    )
    .unwrap();
    let record = derived_document(&profile, &receipt);
    let verify =
        |record: &brain_contracts::SourceRecordV2,
         snapshot: &brain_contracts::CorpusSnapshot,
         bindings: &[brain_storage::entity_profile::derivation::StoredGitBinding]| {
            verify_git_document_receipt(
                record,
                &receipt,
                snapshot,
                &fixture.operator,
                bindings,
                &fixture.blobs,
                &[],
            )
        };
    let mut snapshot = fixture.snapshot.clone();
    let mut revoked_operator = fixture.operator.clone();
    revoked_operator.scopes.clear();
    assert!(verify_git_document_receipt(
        &record,
        &receipt,
        &snapshot,
        &revoked_operator,
        &fixture.bindings,
        &fixture.blobs,
        &[]
    )
    .is_err());
    snapshot.heads[0].tombstone = true;
    assert!(verify(&record, &snapshot, &fixture.bindings).is_err());
    snapshot = fixture.snapshot.clone();
    let mut origin = brain_contracts::source::origin_from_record(&snapshot.heads[0]).unwrap();
    origin
        .policy
        .allowed_scopes
        .insert("source.review:entzogen".into());
    snapshot.heads[0].allowed_scopes = origin.policy.allowed_scopes.clone();
    origin.bind_record(&mut snapshot.heads[0]).unwrap();
    assert!(verify(&record, &snapshot, &fixture.bindings).is_err());
    let mut changed = record.clone();
    changed.content.push(' ');
    assert!(verify(&changed, &fixture.snapshot, &fixture.bindings).is_err());
    changed = record.clone();
    changed.metadata.insert(
        "private_receipt".into(),
        serde_json::to_string(&receipt).unwrap(),
    );
    assert!(verify(&changed, &fixture.snapshot, &fixture.bindings).is_err());
    changed = record.clone();
    changed
        .metadata
        .remove(brain_contracts::source::ORIGIN_METADATA_KEY);
    assert!(verify(&changed, &fixture.snapshot, &fixture.bindings).is_err());
    let mut bindings = fixture.bindings.clone();
    bindings[0].original_fact.value = json!(999);
    assert!(verify(&record, &fixture.snapshot, &bindings).is_err());
    bindings = fixture.bindings.clone();
    bindings[0].semantic_projection.as_mut().unwrap().predicate = "damage".into();
    assert!(verify(&record, &fixture.snapshot, &bindings).is_err());
    bindings = fixture.bindings.clone();
    bindings[0].binding_identity.name = "Falscher Name".into();
    assert!(verify(&record, &fixture.snapshot, &bindings).is_err());
    assert!(verify(&record, &fixture.snapshot, &[]).is_err());
}
