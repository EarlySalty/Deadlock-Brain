use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision,
        ORIGIN_METADATA_KEY,
    },
    value::Observed,
    *,
};
use brain_storage::{
    source_versions::{DOCUMENT_METADATA_KEY, ORIGINAL_VERSION_KEY},
    MemoryRepository,
};
use dbrain_retrieval::{
    knowledge_projection::{project_knowledge, KNOWLEDGE_BYTE_BASIS, KNOWLEDGE_CHUNKER_VERSION},
    ReleaseRetriever,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

fn record(content: &str, status: &str) -> SourceRecordV2 {
    let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
    let document = json!({
        "contract_version": "wiki-spielwissen-v1",
        "source_kind": "wiki", "source_id": "fixture", "document_id": "wiki:fixture:page:123",
        "source_locator": "https://example.org/Page", "title": "Page", "language": "de",
        "revision": "7", "observed_at": "2026-10-03T12:00:00Z",
        "content_sha256": hash, "content": content, "evidence_status": "source_statement",
        "license": {"name": "CC-BY-SA", "url": null, "attribution": "Fixture", "redistribution_allowed": true},
        "metadata": {},
        "facts": [{"fact_id": "cooldown-ä", "subject": "hero:Wächter", "predicate": "ability.cooldown",
            "value": 12.5, "unit": "seconds", "evidence_status": status,
            "source_span": "Page:section/key", "qualifiers": {"mode": "unknown", "nested": {"b": 2, "a": 1}}}]
    });
    let mut record = SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: "wiki:fixture:page:123".into(),
        revision: 7,
        content_hash: hash.clone(),
        content: content.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([
            (DOCUMENT_METADATA_KEY.into(), document.to_string()),
            (ORIGINAL_VERSION_KEY.into(), "7".into()),
            (
                "wiki-spielwissen.provenance_evidence_ref".into(),
                "evidence:fixture".into(),
            ),
        ]),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Wiki {
            page_id: 123,
            revision_id: 7,
        },
        raw_sha256: hash,
        locator: "https://example.org/Page".into(),
        parser_revision: "fixture-v1".into(),
        parser_family: "dbrain-sources/wiki-spielwissen".into(),
        schema_version: Observed::known("wiki-spielwissen-v1".into()),
        schema_sha256: source::observed_option(None),
        retrieved_at: source::observed_option(None),
        source_time: source::observed_option(None),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::from(["fixture:wiki:fixture:page:123:7".into()]),
        derivation_family: Observed::known("wiki-spielwissen-v1".into()),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: Observed::known("operator:fixture".into()),
            license: Observed::known("CC-BY-SA".into()),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record
}

fn query(text: &str) -> Query {
    Query {
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: text.into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: Some("p1".into()),
        mode: None,
        domain: None,
    }
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        request_deadline: None,
        principal: Principal {
            actor_id: "tester".into(),
            channel: "fixture".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into(), "internal".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 30000,
        budget: Budget::default(),
    }
}

async fn published(record: SourceRecordV2) -> MemoryRepository {
    let store = MemoryRepository::default();
    store.apply_record(record).unwrap();
    let release = store.release_from_heads("r1", "knowledge1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}

#[test]
fn projection_is_deterministic_and_keeps_exact_source_and_fact_fields() {
    let original = record("Nur Rohquelle äöü\nEnde", "extracted_value");
    let before = original.clone();
    let projection = project_knowledge(&original).unwrap().unwrap();
    assert_eq!(projection, project_knowledge(&original).unwrap().unwrap());
    assert_eq!(original, before);
    assert_eq!(
        &projection.text[..projection.raw_byte_end],
        original.content
    );
    assert_eq!(projection.raw_sha256, original.content_hash);
    assert_ne!(projection.raw_sha256, projection.semantic_sha256);
    let fact = &projection.facts[0];
    let projected: Value =
        serde_json::from_str(&projection.text[fact.byte_start..fact.byte_end]).unwrap();
    let stored: Value = serde_json::from_str(&original.metadata[DOCUMENT_METADATA_KEY]).unwrap();
    assert_eq!(projected, stored["facts"][0]);
    assert_eq!(projected["source_span"], "Page:section/key");
    assert_eq!(projected["qualifiers"]["nested"]["a"], 1);
    assert_eq!(projected["unit"], "seconds");
}

#[tokio::test]
async fn facts_only_terms_values_and_hypotheses_use_the_canonical_release_path() {
    for status in ["extracted_value", "hypothesis"] {
        let original = record("OriginalMarker äöü", status);
        assert!(!original.content.contains("cooldown"));
        assert!(!original.content.contains("12.5"));
        let projection = project_knowledge(&original).unwrap().unwrap();
        let retriever = ReleaseRetriever::new(published(original.clone()).await, 6);
        let q = query("Wächter cooldown 12.5");
        let c = context();
        let hits = retriever.retrieve(&q, &c).unwrap();
        assert!(!hits.is_empty());
        for hit in &hits {
            let p = hit.provenance.as_ref().unwrap();
            assert_eq!(hit.kind, EvidenceKind::Prose);
            assert_eq!(p.document.content_hash, original.content_hash);
            assert_eq!(p.chunker_version, KNOWLEDGE_CHUNKER_VERSION);
            assert_eq!(p.metadata["byte_basis"], KNOWLEDGE_BYTE_BASIS);
            assert_eq!(
                p.metadata["knowledge_semantic_sha256"],
                projection.semantic_sha256
            );
            assert_eq!(p.metadata["evidence_status"], status);
            assert_eq!(p.metadata["fact_id"], "cooldown-ä");
            assert!(!p.metadata.contains_key(DOCUMENT_METADATA_KEY));
            assert_eq!(hit.content, projection.text[p.byte_start..p.byte_end]);
            assert!(hit.content.contains(status));
            assert_eq!(
                p.metadata[ORIGIN_METADATA_KEY],
                original.metadata[ORIGIN_METADATA_KEY]
            );
        }
        retriever.validate_evidence(&q, &c, &hits, true).unwrap();
        retriever.validate_publication(&q, &c, &hits).unwrap();
        let mut fact_query = q.clone();
        fact_query.profile = AnswerProfile::Fact;
        let fact_hits = retriever.retrieve(&fact_query, &c).unwrap();
        assert!(!fact_hits.is_empty());
        assert!(fact_hits.iter().all(|hit| hit.kind == EvidenceKind::Prose));
        retriever
            .validate_evidence(&fact_query, &c, &fact_hits, false)
            .unwrap();
        let raw = retriever.retrieve(&query("OriginalMarker"), &c).unwrap();
        assert!(!raw.is_empty());
        assert_eq!(
            raw[0].provenance.as_ref().unwrap().document,
            hits[0].provenance.as_ref().unwrap().document
        );
        assert_eq!(
            raw[0].provenance.as_ref().unwrap().metadata[ORIGIN_METADATA_KEY],
            hits[0].provenance.as_ref().unwrap().metadata[ORIGIN_METADATA_KEY]
        );
        let mut forged = hits.clone();
        forged[0]
            .provenance
            .as_mut()
            .unwrap()
            .metadata
            .insert("knowledge_semantic_sha256".into(), "0".repeat(64));
        assert!(retriever.validate_evidence(&q, &c, &forged, true).is_err());
        let mut forged = hits.clone();
        forged[0]
            .provenance
            .as_mut()
            .unwrap()
            .metadata
            .insert("byte_basis".into(), "raw-utf8".into());
        assert!(retriever.validate_evidence(&q, &c, &forged, true).is_err());
        let fresh = ReleaseRetriever::new(published(original).await, 6);
        fresh.validate_evidence(&q, &c, &hits, true).unwrap();
    }
}

#[test]
fn inconsistent_document_or_missing_origin_fails_closed() {
    let original = record("Rohquelle", "hypothesis");
    for key in [
        "source_id",
        "document_id",
        "content_sha256",
        "content",
        "revision",
        "source_locator",
    ] {
        let mut wrong = original.clone();
        let mut document: Value =
            serde_json::from_str(&wrong.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document[key] = json!("wrong");
        wrong
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(project_knowledge(&wrong).is_err(), "{key}");
    }
    for key in [
        ORIGIN_METADATA_KEY,
        ORIGINAL_VERSION_KEY,
        "wiki-spielwissen.provenance_evidence_ref",
    ] {
        let mut wrong = original.clone();
        wrong.metadata.remove(key);
        assert!(project_knowledge(&wrong).is_err(), "{key}");
    }
    for key in [
        "identity",
        "raw_sha256",
        "source_revision",
        "policy",
        "origin_artifacts",
    ] {
        let mut wrong = original.clone();
        let mut origin: Value = serde_json::from_str(&wrong.metadata[ORIGIN_METADATA_KEY]).unwrap();
        origin["data"][key] = json!(null);
        wrong
            .metadata
            .insert(ORIGIN_METADATA_KEY.into(), origin.to_string());
        assert!(project_knowledge(&wrong).is_err(), "{key}");
    }
    for key in [
        "value",
        "unit",
        "source_span",
        "qualifiers",
        "evidence_status",
    ] {
        let mut wrong = original.clone();
        let mut document: Value =
            serde_json::from_str(&wrong.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["facts"][0].as_object_mut().unwrap().remove(key);
        wrong
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(project_knowledge(&wrong).is_err(), "{key}");
    }
    let mut wrong = original.clone();
    wrong.content.push('!');
    assert!(project_knowledge(&wrong).is_err());
    let mut wrong = original;
    wrong
        .metadata
        .insert(DOCUMENT_METADATA_KEY.into(), "{".into());
    assert!(project_knowledge(&wrong).is_err());
}

#[tokio::test]
async fn fact_changes_bind_new_chunk_ids_without_changing_raw_hash() {
    let original = record("OriginalMarker", "hypothesis");
    let q = query("cooldown");
    let c = context();
    let retriever = ReleaseRetriever::new(published(original.clone()).await, 6);
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert!(!hits.is_empty());
    let mut changed = original.clone();
    let mut document: Value =
        serde_json::from_str(&changed.metadata[DOCUMENT_METADATA_KEY]).unwrap();
    document["facts"][0]["value"] = json!(15.5);
    changed
        .metadata
        .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
    assert_eq!(changed.content_hash, original.content_hash);
    assert_eq!(changed.content, original.content);
    let changed_retriever = ReleaseRetriever::new(published(changed).await, 6);
    let new_hits = changed_retriever.retrieve(&q, &c).unwrap();
    assert!(!new_hits.is_empty());
    assert_ne!(hits[0].evidence_id, new_hits[0].evidence_id);
    assert!(changed_retriever
        .validate_evidence(&q, &c, &hits, false)
        .is_err());
}

#[tokio::test]
async fn license_and_current_acl_still_gate_facts() {
    let mut original = record("Rohquelle", "hypothesis");
    let mut document: Value =
        serde_json::from_str(&original.metadata[DOCUMENT_METADATA_KEY]).unwrap();
    document["license"]["redistribution_allowed"] = json!(false);
    original
        .metadata
        .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
    assert!(project_knowledge(&original).is_err());
    let mut origin = source::origin_from_record(&original).unwrap();
    original.visibility = SourceVisibility::Internal;
    original
        .allowed_scopes
        .insert("source.review:fixture".into());
    origin.policy.visibility = original.visibility;
    origin.policy.allowed_scopes = original.allowed_scopes.clone();
    origin.policy.publication_allowed = false;
    origin.policy.provider_egress_allowed = false;
    origin.bind_record(&mut original).unwrap();
    let store = published(original).await;
    let retriever = ReleaseRetriever::new(store, 6);
    let q = query("cooldown 12.5");
    let mut c = context();
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    c.principal.scopes.insert("source.review:fixture".into());
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert!(!hits.is_empty());
    retriever.validate_evidence(&q, &c, &hits, false).unwrap();
    assert!(retriever.validate_evidence(&q, &c, &hits, true).is_err());
    assert!(retriever.validate_publication(&q, &c, &hits).is_err());
}

#[tokio::test]
async fn large_utf8_source_keeps_tail_and_facts_retrievable() {
    let mut content = "Original äöü und UTF-8 mit Leerzeichen.\n".repeat(205_000);
    content.push_str("LetzterOriginalmarker äöü\n");
    assert!(content.len() >= 7_500_000);
    let original = record(&content, "hypothesis");
    let projection = project_knowledge(&original).unwrap().unwrap();
    assert_eq!(&projection.text[..projection.raw_byte_end], content);
    let retriever = ReleaseRetriever::new(published(original).await, 6);
    let c = context();
    for term in ["LetzterOriginalmarker", "cooldown 12.5"] {
        let q = query(term);
        let hits = retriever.retrieve(&q, &c).unwrap();
        assert!(!hits.is_empty(), "{term}");
        for hit in &hits {
            let p = hit.provenance.as_ref().unwrap();
            assert!(projection.text.is_char_boundary(p.byte_start));
            assert!(projection.text.is_char_boundary(p.byte_end));
            assert_eq!(hit.content, projection.text[p.byte_start..p.byte_end]);
        }
        retriever.validate_evidence(&q, &c, &hits, true).unwrap();
    }
}

#[tokio::test]
async fn legacy_without_document_metadata_is_unchanged() {
    let mut original = record("LegacyMarker äöü", "hypothesis");
    original.metadata.remove(DOCUMENT_METADATA_KEY);
    assert_eq!(project_knowledge(&original).unwrap(), None);
    let retriever = ReleaseRetriever::new(published(original.clone()).await, 6);
    let hits = retriever
        .retrieve(&query("LegacyMarker"), &context())
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].content, original.content);
    assert_eq!(
        hits[0].provenance.as_ref().unwrap().metadata,
        original.metadata
    );
    assert_eq!(
        hits[0].provenance.as_ref().unwrap().chunker_version,
        "utf8-window-v1-1024-overlap192"
    );
}

struct ScratchPg {
    directory: std::path::PathBuf,
}

impl ScratchPg {
    fn start() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "brain-entity-query-pg-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&directory).unwrap();
        let instance = Self { directory };
        let data = instance.directory.join("data");
        let socket = instance.directory.join("socket");
        std::fs::create_dir(&socket).unwrap();
        assert!(
            std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
                .arg("-D")
                .arg(&data)
                .args(["-A", "trust", "-U", "brain_core_test", "--no-locale"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .unwrap()
                .success()
        );
        let options = format!("-k {} -p 55440 -c listen_addresses=''", socket.display());
        assert!(
            std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                .arg("-D")
                .arg(&data)
                .arg("-l")
                .arg(instance.directory.join("postgres.log"))
                .args(["-o", &options, "-w", "start"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .unwrap()
                .success()
        );
        instance
    }
}

impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn normal_texts_read_live_entity_facts_counts_and_patch_history() {
    use brain_contracts::entity_profile::{EntityIdentity, EntityKind};
    use brain_storage::{LocalPgReader, PgStore};
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    let pg = ScratchPg::start();
    let socket = pg.directory.join("socket");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(55440)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let store = PgStore::new(pool.clone());
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
    sqlx::raw_sql("CREATE TABLE brain.entities(entity_type text); INSERT INTO brain.entities VALUES('hero'); CREATE TABLE brain.patch_changes(patch_date text,entity_type text,entity_name text,ability_name text,stat_name text,old_value text,new_value text,change_type text,confidence double precision,raw_line text); INSERT INTO brain.patch_changes VALUES('2026-09-16','hero','Wächter',NULL,'cooldown','18','12.5','decrease',1,'Gesperrter Originaltext'),('2025-09-16','item','Wächter',NULL,'Fremde Änderung','777','778','increase',1,'Gesperrter Originaltext'),('2024-09-16','hero','Anderer Held','Wächter','Fremde Änderung','777','778','increase',1,'Gesperrter Originaltext')")
        .execute(&pool).await.unwrap();
    let mut hero = record("Gespeicherter Heldenbeleg", "extracted_value");
    let mut document: Value = serde_json::from_str(&hero.metadata[DOCUMENT_METADATA_KEY]).unwrap();
    document["facts"][0]["subject"] = json!("game_file:technische-werte.json");
    hero.metadata
        .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
    store.apply(&hero).await.unwrap();
    let identity = EntityIdentity {
        entity_key: "hero:waechter".into(),
        kind: EntityKind::Hero,
        name: "Wächter".into(),
        aliases: vec!["Warden".into()],
        identity_evidence: vec!["fixture:catalog:1".into()],
    };
    assert_eq!(
        store
            .store_entity_fact_bindings(&identity, &hero, &["cooldown-ä".into()])
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .store_entity_fact_bindings(&identity, &hero, &["cooldown-ä".into()])
            .await
            .unwrap(),
        0
    );
    let mut item = record("Gespeicherter Itembeleg", "extracted_value");
    let mut origin = source::origin_from_record(&item).unwrap();
    item.logical_id = "wiki:fixture:page:456".into();
    origin.identity.logical_id = item.logical_id.clone();
    origin.source_revision = SourceRevision::Wiki {
        page_id: 456,
        revision_id: 7,
    };
    let mut document: Value = serde_json::from_str(&item.metadata[DOCUMENT_METADATA_KEY]).unwrap();
    document["document_id"] = json!(item.logical_id);
    document["facts"][0]["subject"] = json!("item:Extended Magazine");
    document["facts"][0]["predicate"] = json!("clip_size");
    document["facts"][0]["value"] = json!(22);
    item.metadata
        .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
    origin.bind_record(&mut item).unwrap();
    store.apply(&item).await.unwrap();
    let identity = EntityIdentity {
        entity_key: "item:extended-magazine".into(),
        kind: EntityKind::Item,
        name: "Extended Magazine".into(),
        aliases: vec![],
        identity_evidence: vec!["fixture:catalog:2".into()],
    };
    store
        .store_entity_fact_bindings(&identity, &item, &["cooldown-ä".into()])
        .await
        .unwrap();
    sqlx::query("UPDATE brain.entity_profile_entities_v1 SET identity_json=jsonb_set(jsonb_set(identity_json,'{name}','\"Gesperrter globaler Name\"'::jsonb),'{aliases}','[\"Privater Alias\"]'::jsonb) WHERE entity_key='hero:waechter'")
        .execute(&pool).await.unwrap();
    sqlx::query("UPDATE brain.patch_changes SET entity_name='Warden' WHERE entity_type='hero' AND entity_name='Wächter' AND patch_date='2026-09-16'")
        .execute(&pool).await.unwrap();
    let release = CorpusRelease {
        release_id: "r1".into(),
        knowledge_version: "knowledge1".into(),
        patch: "p1".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "fixture".into(),
            BTreeMap::from([(hero.logical_id.clone(), 7), (item.logical_id.clone(), 7)]),
        )]),
    };
    store.publish_release(&release).await.unwrap();
    let reader = tokio::task::block_in_place(|| {
        LocalPgReader::new(&socket, 55440, "brain_core_test", "postgres").unwrap()
    });
    let retriever = ReleaseRetriever::new(reader.clone(), 10);
    let context = context();
    for (text, expected) in [
        (
            "Welche Abklingzeit ist für den Helden Wächter gespeichert?",
            "12.5",
        ),
        ("Wie viele Helden gibt es?", "1 Helden"),
        ("Was macht das Item Extended Magazine?", "22"),
        ("Was änderte sich bei Wächter im Patch vom 16.09.?", "18"),
    ] {
        let mut query = query(text);
        query.patch = None;
        let evidence =
            tokio::task::block_in_place(|| retriever.retrieve(&query, &context)).unwrap();
        assert!(!evidence.is_empty(), "{text}");
        assert!(
            evidence.iter().any(|item| item.content.contains(expected)),
            "{text}"
        );
        assert!(evidence
            .iter()
            .all(|item| item.evidence_id.starts_with("entity-profile:")));
        assert!(evidence
            .iter()
            .all(|item| !item.content.contains("Gesperrter Originaltext")));
        assert!(evidence
            .iter()
            .all(|item| !item.content.contains("Fremde Änderung")));
        assert!(evidence.iter().all(|item| {
            !item.content.contains("Gesperrter globaler Name")
                && !item.content.contains("Privater Alias")
                && !item.content.contains("Warden")
                && !item.citation.contains("Warden")
                && !item.content.contains("game_file:technische-werte.json")
        }));
        tokio::task::block_in_place(|| {
            retriever.validate_evidence(&query, &context, &evidence, false)
        })
        .unwrap();
    }
    let mut previous_value = query("Welche Abklingzeit hatte Wächter?");
    previous_value.patch = Some("2026-09-16".into());
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&previous_value, &context))
            .unwrap()
            .is_empty()
    );
    let mut historical = query("Was änderte sich bei Wächter im Patch vom 16.09.?");
    historical.patch = None;
    let previous =
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context)).unwrap();
    sqlx::query("UPDATE brain.patch_changes SET new_value='13' WHERE patch_date='2026-09-16'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(tokio::task::block_in_place(|| retriever.validate_evidence(
        &historical,
        &context,
        &previous,
        false
    ))
    .is_err());
    let current =
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context)).unwrap();
    assert!(current.iter().any(|item| item.content.contains("13")));
    for text in [
        "Was änderte sich bei Privater Alias im Patch vom 16.09.?",
        "Welche Abklingzeit hat Warden?",
        "Was änderte sich bei Warden im Patch vom 16.09.?",
    ] {
        let mut forbidden_alias = query(text);
        forbidden_alias.patch = None;
        assert!(
            tokio::task::block_in_place(|| retriever.retrieve(&forbidden_alias, &context))
                .unwrap()
                .is_empty(),
            "{text}"
        );
    }
    let mut scoped = hero.clone();
    let mut origin = source::origin_from_record(&scoped).unwrap();
    scoped.revision = 8;
    origin.source_revision = SourceRevision::Wiki {
        page_id: 123,
        revision_id: 8,
    };
    origin.policy.visibility = SourceVisibility::Internal;
    origin.policy.allowed_scopes = BTreeSet::from(["source.review:fixture".into()]);
    scoped.visibility = origin.policy.visibility;
    scoped.allowed_scopes = origin.policy.allowed_scopes.clone();
    origin.bind_record(&mut scoped).unwrap();
    store.apply(&scoped).await.unwrap();
    assert_eq!(
        tokio::task::block_in_place(|| reader.read_entity_evidence(
            &historical,
            &context,
            Some("%-09-16"),
            false,
            brain_contracts::store::AnswerPurpose::InternalRead,
        ))
        .unwrap(),
        Some(Vec::new())
    );
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context))
            .unwrap()
            .is_empty()
    );
    assert!(tokio::task::block_in_place(|| retriever.validate_evidence(
        &historical,
        &context,
        &current,
        false
    ))
    .is_err());
    let mut restricted = hero.clone();
    restricted.tombstone = true;
    restricted.revision = 9;
    store.apply(&restricted).await.unwrap();
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context))
            .unwrap()
            .is_empty()
    );
    assert!(tokio::task::block_in_place(|| retriever.validate_evidence(
        &historical,
        &context,
        &current,
        false
    ))
    .is_err());
    tokio::task::block_in_place(|| drop((retriever, reader)));
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn normal_texts_read_stored_compact_documents_with_fresh_original_proofs() {
    use brain_contracts::entity_profile::{EntityIdentity, EntityKind, ENTITY_PROFILE_VERSION};
    use brain_storage::{
        entity_profile::{
            compact::compact_document,
            derivation::{
                derive_git_profile, derived_policy, git_document_identity, receipt_sha256,
                GitBlobEvidence, GIT_DOCUMENT_CONTRACT,
            },
            project_entity_facts,
            semantic::semantic_projection,
        },
        LocalPgReader, PgStore,
    };
    use std::sync::{Arc, Mutex};
    let pg = ScratchPg::start();
    let socket = pg.directory.join("socket");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(55440)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    for migration in [
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profiles-v1.sql"),
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profile-binding-identity-v1.sql"),
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-semantic-projection-v1.sql"),
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-derived-receipts-v1.sql"),
    ] { sqlx::raw_sql(migration).execute(&pool).await.unwrap(); }
    sqlx::raw_sql("CREATE TABLE brain.patch_changes(patch_date text,entity_type text,entity_name text,ability_name text,stat_name text,old_value text,new_value text,change_type text,confidence double precision,raw_line text); INSERT INTO brain.patch_changes VALUES('2026-09-16','hero','hero_test',NULL,'MaxHealth','800','830','increase',1,'Gesperrte Originalzeile'),('2025-09-16','item','Wächter',NULL,'Fremdes Feld','777','778','increase',1,'Gesperrte Originalzeile')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO brain.patch_changes VALUES('2026-09-01','hero','hero_test',NULL,'MaxHealth','780','800','increase',1,'Gesperrte Originalzeile')")
        .execute(&pool).await.unwrap();
    let identities = [
        EntityIdentity {
            entity_key: "hero:fixture".into(),
            kind: EntityKind::Hero,
            name: "Wächter".into(),
            aliases: Vec::new(),
            identity_evidence: vec!["fixture:hero".into()],
        },
        EntityIdentity {
            entity_key: "item:fixture".into(),
            kind: EntityKind::Item,
            name: "Extended Magazine".into(),
            aliases: Vec::new(),
            identity_evidence: vec!["fixture:item".into()],
        },
    ];
    let mut originals = Vec::new();
    for (identity, identifier, stat, value) in [
        (&identities[0], "hero_test", "MaxHealth", 830),
        (&identities[1], "item_test", "BonusClipSizePercent", 30),
    ] {
        let mut identity = identity.clone();
        let content = if identity.kind == EntityKind::Hero {
            json!({identifier:{stat:value,"CurrentPatch":"2026-09-16"}}).to_string()
        } else {
            json!({identifier:{stat:value}}).to_string()
        };
        let mut original = record(&content, "extracted_value");
        let mut origin = source::origin_from_record(&original).unwrap();
        original.logical_id = format!("game:fixture:{identifier}.json");
        original.visibility = SourceVisibility::Internal;
        original.allowed_scopes = BTreeSet::from(["raw:fixture".into()]);
        origin.identity.logical_id = original.logical_id.clone();
        origin.source_revision = SourceRevision::Api {
            api_version: "wiki-spielwissen-v1".into(),
            original_revision: Some("a".repeat(40)),
        };
        origin.policy.visibility = original.visibility;
        origin.policy.allowed_scopes = original.allowed_scopes.clone();
        origin.policy.provider_egress_allowed = false;
        origin.policy.publication_allowed = false;
        let mut document: Value =
            serde_json::from_str(&original.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["source_kind"] = json!("game_file");
        document["document_id"] = json!(original.logical_id);
        document["revision"] = json!("a".repeat(40));
        document["metadata"] = json!({"source_revision":"a".repeat(40),"original_relative_path":format!("{identifier}.json"),"original_sha256":original.content_hash,"provenance":{"repository_url":"https://github.com/deadlock-wiki/deadlock-data"}});
        document["facts"] = json!([{"fact_id":"stat","subject":"game_file:private-fixture.json","predicate":"file.json_value","value":value,"unit":null,"evidence_status":"extracted_value","source_span":format!("/{identifier}/{stat}"),"qualifiers":{"source_pointer":format!("/{identifier}/{stat}")}}]);
        let mut fact_ids = vec!["stat".into()];
        if identity.kind == EntityKind::Hero {
            document["facts"].as_array_mut().unwrap().push(json!({"fact_id":"current_patch","subject":"game_file:private-fixture.json","predicate":"current_patch","value":"2026-09-16","unit":null,"evidence_status":"extracted_value","source_span":format!("/{identifier}/CurrentPatch"),"qualifiers":{"source_pointer":format!("/{identifier}/CurrentPatch")}}));
            fact_ids.push("current_patch".into());
        }
        original
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        original
            .metadata
            .insert(ORIGINAL_VERSION_KEY.into(), "a".repeat(40));
        origin.bind_record(&mut original).unwrap();
        identity.aliases.push(identifier.into());
        identity.identity_evidence = vec![format!(
            "{}:{}:{}:/{identifier}",
            original.source_id,
            original.logical_id,
            "a".repeat(40)
        )];
        store.apply(&original).await.unwrap();
        store
            .store_entity_fact_bindings(&identity, &original, &fact_ids)
            .await
            .unwrap();
        let fact = project_entity_facts(&original, &["stat".into()])
            .unwrap()
            .remove(0);
        let projection = semantic_projection(&fact, &format!("/{stat}"), &original, &identity)
            .unwrap()
            .unwrap();
        store
            .store_entity_semantic_projections(
                &identity.entity_key,
                &original,
                &[("stat".into(), projection)],
            )
            .await
            .unwrap();
        originals.push(original);
    }
    let original_release = CorpusRelease {
        release_id: "original-fixture".into(),
        knowledge_version: "original-fixture-v1".into(),
        patch: "unbekannt".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "fixture".into(),
            originals
                .iter()
                .map(|record| (record.logical_id.clone(), record.revision))
                .collect(),
        )]),
    };
    store.publish_release(&original_release).await.unwrap();
    let operator = Principal {
        actor_id: "unix:fixture".into(),
        channel: "local-operator".into(),
        scopes: BTreeSet::from(["raw:fixture".into()]),
        provider_egress: Default::default(),
    };
    let snapshot = store.snapshot(&original_release.release_id).await.unwrap();
    let blob = |record: &SourceRecordV2| {
        let (commit, repository, path) = git_document_identity(record).unwrap();
        assert!(path.ends_with(".json"));
        GitBlobEvidence {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            store_revision: record.revision,
            git_commit: commit,
            repository_url: repository,
            bytes: record.content.as_bytes().to_vec(),
        }
    };
    let mut documents = Vec::new();
    for (identity, original) in identities.iter().zip(&originals) {
        let bindings = store
            .stored_git_entity_bindings(&identity.entity_key, original)
            .await
            .unwrap();
        let story = store
            .entity_patch_story(&bindings[0].binding_identity)
            .await
            .unwrap();
        let (profile, receipt) = derive_git_profile(
            &identity.entity_key,
            &snapshot,
            &operator,
            &bindings,
            &[blob(original)],
            &story,
        )
        .unwrap();
        let content = compact_document(&profile).unwrap();
        let mut document = SourceRecordV2 {
            source_id: "git-game-facts-derived".into(),
            logical_id: identity.entity_key.clone(),
            revision: 1,
            content_hash: receipt.document_sha256.clone(),
            content,
            visibility: SourceVisibility::Public,
            allowed_scopes: Default::default(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::from([
                (
                    "brain.entity_projection.contract".into(),
                    GIT_DOCUMENT_CONTRACT.into(),
                ),
                (
                    "brain.entity_projection.receipt_sha256".into(),
                    receipt_sha256(&receipt).unwrap(),
                ),
            ]),
        };
        let mut origin = source::origin_from_record(original).unwrap();
        origin.identity = SourceIdentity {
            source_id: document.source_id.clone(),
            logical_id: document.logical_id.clone(),
        };
        origin.source_revision = SourceRevision::Api {
            api_version: GIT_DOCUMENT_CONTRACT.into(),
            original_revision: Some(document.content_hash.clone()),
        };
        origin.raw_sha256 = document.content_hash.clone();
        origin.locator = document.source_id.clone();
        origin.parser_family = "entity-profile-render".into();
        origin.parser_revision = ENTITY_PROFILE_VERSION.into();
        origin.schema_version = Observed::known(ENTITY_PROFILE_VERSION.into());
        origin.origin_artifacts.clear();
        origin.derivation_family = Observed::known(GIT_DOCUMENT_CONTRACT.into());
        origin.policy = derived_policy();
        origin.bind_record(&mut document).unwrap();
        documents.push(
            store
                .persist_entity_document(document, &serde_json::to_string(&receipt).unwrap())
                .await
                .unwrap(),
        );
    }
    let mut release = original_release.clone();
    release.release_id = "r1".into();
    release.knowledge_version = "knowledge1".into();
    release.source_revisions.insert(
        "git-game-facts-derived".into(),
        documents
            .iter()
            .map(|record| (record.logical_id.clone(), record.revision))
            .collect(),
    );
    store.publish_release(&release).await.unwrap();
    let scopes = Arc::new(Mutex::new(operator.scopes.clone()));
    let fresh_scopes = scopes.clone();
    let reader = tokio::task::block_in_place(|| {
        LocalPgReader::new(&socket, 55440, "brain_core_test", "postgres")
            .unwrap()
            .with_entity_profile_access(
                move || {
                    Ok(Principal {
                        actor_id: "unix:fixture".into(),
                        channel: "local-operator".into(),
                        scopes: fresh_scopes.lock().unwrap().clone(),
                        provider_egress: Default::default(),
                    })
                },
                move |record| {
                    let (commit, repository, _path) = git_document_identity(record)
                        .map_err(|_| PortError::InvalidResponse("Fixture-Gitbeleg fehlt".into()))?;
                    Ok(GitBlobEvidence {
                        source_id: record.source_id.clone(),
                        logical_id: record.logical_id.clone(),
                        store_revision: record.revision,
                        git_commit: commit,
                        repository_url: repository,
                        bytes: record.content.as_bytes().to_vec(),
                    })
                },
            )
    });
    let retriever = ReleaseRetriever::new(reader.clone(), 10);
    let context = context();
    for (text, expected) in [
        ("Wie viel MaxHealth ist für Wächter gespeichert?", "830"),
        ("Was macht Extended Magazine?", "30"),
    ] {
        let mut query = query(text);
        query.patch = None;
        let evidence =
            tokio::task::block_in_place(|| retriever.retrieve(&query, &context)).unwrap();
        assert_eq!(evidence.len(), 1);
        assert!(evidence[0]
            .evidence_id
            .starts_with("entity-profile:document:"));
        assert!(evidence[0].content.contains(expected));
        assert!(documents
            .iter()
            .any(|document| document.content == evidence[0].content));
        for forbidden in [
            "private-fixture.json",
            "raw:fixture",
            "original-fixture",
            "Gesperrte Originalzeile",
            "hero_test",
            "item_test",
        ] {
            assert!(!evidence[0].content.contains(forbidden));
        }
        tokio::task::block_in_place(|| {
            retriever.validate_evidence(&query, &context, &evidence, true)
        })
        .unwrap();
    }
    let mut count = query("Wie viele Helden sind gespeichert?");
    count.patch = None;
    let counted = tokio::task::block_in_place(|| retriever.retrieve(&count, &context)).unwrap();
    assert!(counted[0].content.contains("1 Helden"));
    for patch in ["2026-08-31", "2026-09-15", "2026-09-16", "2026-09-17"] {
        let mut previous_value = query("Wie viel MaxHealth war für Wächter gespeichert?");
        previous_value.patch = Some(patch.into());
        assert!(
            tokio::task::block_in_place(|| retriever.retrieve(&previous_value, &context))
                .unwrap()
                .is_empty()
        );
    }
    let mut missing_date = query("Was änderte sich bei Wächter im Patch vom 15.09.?");
    missing_date.patch = None;
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&missing_date, &context))
            .unwrap()
            .is_empty()
    );
    let mut historical = query("Was änderte sich bei Wächter im Patch vom 16.09.?");
    historical.patch = None;
    let previous =
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context)).unwrap();
    assert!(previous.iter().any(|item| item.content.contains("800")));
    assert!(previous
        .iter()
        .all(|item| item.source_id == "brain.patch_changes"
            && !item.content.contains("Originalzeile")));
    sqlx::query("UPDATE brain.patch_changes SET new_value='831' WHERE patch_date='2026-09-16'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(tokio::task::block_in_place(|| retriever.validate_evidence(
        &historical,
        &context,
        &previous,
        true
    ))
    .is_err());
    sqlx::query("UPDATE brain.patch_changes SET new_value='830' WHERE patch_date='2026-09-16'")
        .execute(&pool)
        .await
        .unwrap();
    scopes.lock().unwrap().clear();
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&historical, &context))
            .unwrap()
            .is_empty()
    );
    assert!(tokio::task::block_in_place(|| retriever.validate_evidence(
        &historical,
        &context,
        &previous,
        true
    ))
    .is_err());
    let mut numeric = query("Welche gespeicherten Zahlen sind 830?");
    numeric.patch = None;
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&numeric, &context))
            .unwrap()
            .is_empty()
    );
    let mut current = query("Wie viel MaxHealth hat Wächter?");
    current.patch = None;
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&current, &context))
            .unwrap()
            .is_empty()
    );
    scopes.lock().unwrap().insert("raw:fixture".into());
    let mut tombstone = originals[0].clone();
    tombstone.revision += 1;
    tombstone.tombstone = true;
    store.apply(&tombstone).await.unwrap();
    assert!(
        tokio::task::block_in_place(|| retriever.retrieve(&current, &context))
            .unwrap()
            .is_empty()
    );
    tokio::task::block_in_place(|| drop((retriever, reader)));
    pool.close().await;
}
