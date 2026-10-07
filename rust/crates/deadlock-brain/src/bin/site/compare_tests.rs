use super::*;
use brain_contracts::{
    CorpusRelease, CorpusSnapshot, DocumentHead, DocumentRevision, PortError, Principal,
    SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use brain_maintenance::{
    compare_artifact::{prepare_compare_artifact, publish_compare_artifact, render_stored_compare},
    hero_compare_render::{
        BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue, HeroCompareInput,
        HeroCompareSeries, PublicationStatus, VersionBinding,
    },
};
use brain_storage::compare_artifact::{
    compare_dependency, compare_sha256, CompareArtifactBody, CompareCalculationVerifier,
};
use std::collections::{BTreeMap, BTreeSet};

struct Fixture {
    record: SourceRecordV2,
    release: CorpusRelease,
}
impl SnapshotReadPort for Fixture {
    fn read_snapshot(&self, _: &str) -> std::result::Result<CorpusSnapshot, PortError> {
        Ok(CorpusSnapshot {
            release: self.release.clone(),
            revisions: vec![self.record.clone()],
            heads: vec![self.record.clone()],
        })
    }
    fn read_heads(
        &self,
        _: &[DocumentRevision],
    ) -> std::result::Result<Vec<DocumentHead>, PortError> {
        Ok(vec![DocumentHead::from(&self.record)])
    }
}
struct FixtureVerifier(CompareArtifactBody);
impl CompareCalculationVerifier for FixtureVerifier {
    fn verify(&self, body: &CompareArtifactBody) -> std::result::Result<(), PortError> {
        if body != &self.0 {
            return Err(PortError::InvalidResponse("fixture mismatch".into()));
        }
        Ok(())
    }
}

#[tokio::test]
async fn public_compare_http_has_no_cache_and_rechecks_revocation() {
    let pg = Postgres::new();
    pg.roles();
    let owner = pg.pool("brain_migrate").await;
    let store = brain_storage::PgStore::new(owner.clone());
    store.migrate_core().await.unwrap();
    TestStore::new(owner.clone())
        .migrate_site_comments()
        .await
        .unwrap();
    store.migrate_compare_artifacts().await.unwrap();
    pg.grants();
    let record = SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: "mechanism".into(),
        revision: 1,
        content_hash: compare_sha256(b"fixture"),
        content: "fixture".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    let release = CorpusRelease {
        release_id: "fixture-release".into(),
        knowledge_version: "fixture-client".into(),
        patch: "fixture".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "fixture".into(),
            BTreeMap::from([("mechanism".into(), 1)]),
        )]),
    };
    store.apply(&record).await.unwrap();
    sqlx::query("INSERT INTO brain.corpus_releases_v1(release_id,knowledge_version,patch,release_json) VALUES($1,$2,$3,$4)").bind(&release.release_id).bind(&release.knowledge_version).bind(&release.patch).bind(serde_json::to_value(&release).unwrap()).execute(&owner).await.unwrap();
    let binding = CompareBinding {
        result_id: "fixture".into(),
        version: VersionBinding {
            snapshot_id: "fixture-release".into(),
            client_version: "fixture-client".into(),
        },
        conditions: vec!["Erfundene Strukturprobe ohne Spielaussage".into()],
    };
    let hero = |id: &str, name: &str, value: f64| HeroCompareSeries {
        hero_id: id.into(),
        hero_name: name.into(),
        publication: PublicationStatus::PublicApproved,
        binding: binding.clone(),
        metric: CompareMetric::BaseDps,
        source_ids: vec!["fixture/mechanism".into()],
        values: vec![BoonValue {
            boon: 0,
            value: DisplayValue::Quantified(value),
        }],
    };
    let heroes = [hero("a", "Testheld A", 10.0), hero("b", "Testheld B", 15.0)];
    let input = HeroCompareInput {
        sources: vec![CompareSource {
            source_id: "fixture/mechanism".into(),
            evidence: "Erfundene Testwerte".into(),
            version: binding.version.clone(),
            publication: PublicationStatus::PublicApproved,
        }],
        binding,
        publication: PublicationStatus::PublicApproved,
        metric: CompareMetric::BaseDps,
        valid_boon_states: vec![0],
        heroes,
    };
    let principal = Principal {
        actor_id: "fixture".into(),
        channel: "fixture".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    let document = compare_dependency(&record).unwrap().document;
    let fixture = Fixture {
        record: record.clone(),
        release,
    };
    let artifact = prepare_compare_artifact(
        &fixture,
        &principal,
        &input,
        &[document.clone()],
        json!({"fixture":true}),
        "fixture-mechanism".into(),
    )
    .unwrap();
    store.save_pending_compare(&artifact).await.unwrap();
    let pool = pg.pool("brain_site").await;
    let root = fixtures();
    let (url, handle) = server(root.path(), pool.clone()).await;
    let client = Client::new();
    let path = format!("/compare/{}", artifact.id());
    assert_eq!(
        client
            .get(format!("{url}{path}"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    let publication = publish_compare_artifact(
        &store,
        &fixture,
        &principal,
        &artifact,
        &FixtureVerifier(artifact.body().clone()),
    )
    .await
    .unwrap();
    for (path, expected) in [
        (&publication.html_path, &artifact.body().html),
        (&publication.svg_path, &artifact.body().svg),
    ] {
        let reply = client.get(format!("{url}{path}")).send().await.unwrap();
        assert_eq!(reply.status(), StatusCode::OK);
        assert_eq!(reply.headers()["cache-control"], "no-store");
        assert_eq!(reply.headers()["x-content-type-options"], "nosniff");
        assert_eq!(reply.text().await.unwrap(), *expected);
        let reply = client.head(format!("{url}{path}")).send().await.unwrap();
        assert_eq!(reply.status(), StatusCode::OK);
        assert_eq!(
            reply.headers()["content-length"],
            expected.len().to_string()
        );
        assert!(reply.bytes().await.unwrap().is_empty());
    }
    for value in [-0.0, 1e18, 1e-100, 1.2345678901234567] {
        let mut numeric_input = input.clone();
        numeric_input.heroes[0].values[0].value = DisplayValue::Quantified(value);
        numeric_input.heroes[1].values[0].value = DisplayValue::Quantified(value);
        let calculation = json!({
            "fixture": true,
            "nested": {"values": [value, u64::MAX, i64::MIN]},
            "precise": serde_json::from_str::<serde_json::Value>(
                "12345678901234567890.12345678901234567890"
            ).unwrap()
        });
        let numeric = prepare_compare_artifact(
            &fixture,
            &principal,
            &numeric_input,
            std::slice::from_ref(&document),
            calculation.clone(),
            "fixture-mechanism".into(),
        )
        .unwrap();
        store.save_pending_compare(&numeric).await.unwrap();
        store.save_pending_compare(&numeric).await.unwrap();
        let normalized: serde_json::Value = sqlx::query_scalar(
            "SELECT body_json FROM brain.compare_artifacts_v1 WHERE artifact_id=$1",
        )
        .bind(numeric.id())
        .fetch_one(&owner)
        .await
        .unwrap();
        if value == 0.0 || value == 1e18 || value == 1e-100 {
            assert_ne!(
                normalized["calculation"]["nested"]["values"][0],
                calculation["nested"]["values"][0]
            );
        }
        let publication = publish_compare_artifact(
            &store,
            &fixture,
            &principal,
            &numeric,
            &FixtureVerifier(numeric.body().clone()),
        )
        .await
        .unwrap();
        assert_eq!(publication.artifact_id, numeric.id());
        let restored = brain_storage::PgStore::new(pool.clone())
            .read_public_compare(numeric.id(), &principal)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(restored, numeric);
        assert_eq!(restored.body().calculation, calculation);
        let rendered = render_stored_compare(&restored).unwrap();
        assert_eq!(rendered.html.as_bytes(), numeric.body().html.as_bytes());
        assert_eq!(rendered.svg.as_bytes(), numeric.body().svg.as_bytes());
        for (path, expected) in [
            (&publication.html_path, &numeric.body().html),
            (&publication.svg_path, &numeric.body().svg),
        ] {
            let reply = client.get(format!("{url}{path}")).send().await.unwrap();
            assert_eq!(reply.status(), StatusCode::OK);
            assert_eq!(reply.bytes().await.unwrap().as_ref(), expected.as_bytes());
        }
    }
    let mut revoked = record;
    revoked.revision = 2;
    revoked.visibility = SourceVisibility::Private;
    revoked.allowed_scopes.insert("private".into());
    store.apply(&revoked).await.unwrap();
    for path in [&publication.html_path, &publication.svg_path] {
        let reply = client
            .get(format!("{url}{path}"))
            .header("If-None-Match", artifact.id())
            .send()
            .await
            .unwrap();
        assert_ne!(reply.status(), StatusCode::OK);
        assert_ne!(reply.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(reply.headers()["cache-control"], "no-store");
    }
    assert!(
        sqlx::query("SELECT record_json FROM brain.source_record_heads")
            .execute(&pool)
            .await
            .is_err()
    );
    handle.abort();
    let _ = handle.await;
    pool.close().await;
    owner.close().await;
}
