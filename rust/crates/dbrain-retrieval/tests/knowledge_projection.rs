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
