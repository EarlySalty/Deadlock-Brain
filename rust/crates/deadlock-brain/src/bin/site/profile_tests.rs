use super::*;
use brain_contracts::{
    entity_profile::{EntityIdentity, EntityKind},
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    CorpusRelease, SourceRecordV2, SourceVisibility,
};
use brain_storage::{compare_artifact::compare_sha256, PgStore};
use std::collections::{BTreeMap, BTreeSet};

fn source() -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: "public-game-fixture".into(),
        logical_id: "test-hero".into(),
        revision: 1,
        content_hash: compare_sha256(b"fixture original"),
        content: "fixture original".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Git {
            commit: "a".repeat(40),
        },
        raw_sha256: record.content_hash.clone(),
        locator: "git:fixture/test-hero".into(),
        parser_revision: "fixture-v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::known("1".into()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            authorization_ref: Observed::known("fixture-public".into()),
            license: Observed::known("fixture".into()),
            publication_allowed: true,
            provider_egress_allowed: false,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record.metadata.insert(brain_storage::source_versions::DOCUMENT_METADATA_KEY.into(), json!({
        "document_id":record.logical_id,"source_id":record.source_id,"content_sha256":record.content_hash,"content":record.content,
        "source_kind":"game_file","revision":"a".repeat(40),"observed_at":"2026-10-08","license":{"redistribution_allowed":true},"metadata":{},
        "facts":[{"fact_id":"health","subject":"hero:Testheld","predicate":"Health","value":501,"unit":"HP","qualifiers":{},"evidence_status":"structural_match","source_span":null}]
    }).to_string());
    record
}

#[tokio::test]
async fn public_profiles_use_the_isolated_role_and_recheck_source_grants() {
    let pg = Postgres::new();
    pg.roles();
    let owner = pg.pool("brain_migrate").await;
    let store = PgStore::new(owner.clone());
    store.migrate_core().await.unwrap();
    store.migrate_entity_profiles().await.unwrap();
    TestStore::new(owner.clone())
        .migrate_site_comments()
        .await
        .unwrap();
    store.migrate_site_profiles().await.unwrap();
    store.migrate_site_profiles().await.unwrap();
    pg.grants();
    let record = source();
    store.apply(&record).await.unwrap();
    let identity = EntityIdentity {
        entity_key: "hero_test".into(),
        kind: EntityKind::Hero,
        name: "Testheld".into(),
        aliases: vec![],
        identity_evidence: vec!["fixture".into()],
    };
    store
        .store_entity_fact_bindings(&identity, &record, &["health".into()])
        .await
        .unwrap();
    let release = CorpusRelease {
        release_id: "profile-release".into(),
        knowledge_version: "fixture".into(),
        patch: "unknown".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            record.source_id.clone(),
            BTreeMap::from([(record.logical_id.clone(), 1)]),
        )]),
    };
    sqlx::query("INSERT INTO brain.corpus_releases_v1(release_id,knowledge_version,patch,release_json) VALUES($1,$2,$3,$4)")
        .bind(&release.release_id).bind(&release.knowledge_version).bind(&release.patch).bind(serde_json::to_value(&release).unwrap()).execute(&owner).await.unwrap();
    let pool = pg.pool("brain_site").await;
    assert!(sqlx::query("SELECT * FROM brain.entity_profile_facts_v1")
        .fetch_all(&pool)
        .await
        .is_err());
    assert!(sqlx::query("SELECT * FROM brain.source_record_revisions")
        .fetch_all(&pool)
        .await
        .is_err());
    let root = fixtures();
    let router = super::super::router_with_release(
        root.path(),
        pool.clone(),
        Some(release.release_id.clone()),
    )
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let path = format!(
        "/site/steckbriefe/hero/{}",
        hex::encode(identity.entity_key.as_bytes())
    );
    let client = Client::new();
    let index = client
        .get(format!("{url}/site/steckbriefe"))
        .send()
        .await
        .unwrap();
    assert_eq!(index.status(), StatusCode::OK);
    assert_eq!(index.headers()[header::CACHE_CONTROL], "no-store");
    assert!(index
        .text()
        .await
        .unwrap()
        .contains(&format!("/brain{path}")));
    let page = client.get(format!("{url}{path}")).send().await.unwrap();
    assert_eq!(page.status(), StatusCode::OK);
    assert_eq!(
        page.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );
    let page = page.text().await.unwrap();
    assert!(page.contains("<code>501</code>"));
    assert!(page.contains("Patchstand: Unbekannt"));
    assert!(client
        .head(format!("{url}{path}"))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        client
            .get(format!(
                "{url}/site/steckbriefe/item/{}",
                hex::encode(identity.entity_key.as_bytes())
            ))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    let mut revoked = record.clone();
    revoked.revision = 2;
    let mut origin = brain_contracts::source::origin_from_record(&revoked).unwrap();
    origin.policy.publication_allowed = false;
    origin.bind_record(&mut revoked).unwrap();
    store.apply(&revoked).await.unwrap();
    assert_eq!(
        client
            .get(format!("{url}{path}"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    let index = client
        .get(format!("{url}/site/steckbriefe"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(!index.contains("Testheld"));
    assert!(PgStore::new(pool)
        .read_site_profiles(&release.release_id, None)
        .await
        .unwrap()
        .is_empty());
    handle.abort();
}
