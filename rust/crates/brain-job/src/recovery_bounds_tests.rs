use super::*;

#[tokio::test]
async fn corrected_wiki_target_recovers_a_source_identity_extraction_limit() {
    let pg = ScratchPg::start();
    let (admin, pool) = database(&pg).await;
    let upstream = Upstream::start(Source::ALL[0]).await;
    let source = upstream.source;
    let state = tempfile::tempdir().unwrap();
    let repository = Repository::open(state.path(), source, 30).await.unwrap();
    let hero_path = crate::IDENTITY_PATHS[0];
    let names_path = crate::LOCALIZATION_PATHS[0];
    let hero = br#"{"hero_old":{"MaxHealth":780}}"#;
    let names = br#"{"hero_old":"Old Hero"}"#;
    upstream.file(hero_path, Some(hero));
    upstream.file(names_path, Some(names));
    let initial = upstream.commit(None);
    let mut store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    let expected = current(&pool).await;
    store.finish(&counts, false).await.unwrap();
    let mut oversized = serde_json::Map::new();
    oversized.insert("hero_old".into(), serde_json::json!({"MaxHealth":900}));
    for index in 0..10000 {
        oversized.insert(
            format!("hero_bad_{index}"),
            serde_json::json!({"MaxHealth":1}),
        );
    }
    upstream.file(hero_path, Some(&serde_json::to_vec(&oversized).unwrap()));
    let bad = upstream.commit(Some(&initial));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    let error = ingest_source(
        &store,
        &repository,
        Source::ALL[0],
        1024 * 1024,
        &mut counts,
        repository.fetch(source),
    )
    .await
    .unwrap_err();
    assert!(error.downcast_ref::<InvalidTarget>().is_some(), "{error:#}");
    assert_eq!(error.to_string(), "identity_limit");
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.objects_completed, 1);
    assert_eq!(current(&pool).await, expected);
    let receipts: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt")
        .fetch_one(&pool)
        .await
        .unwrap();
    let identities: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.entity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(identities, 10001);
    store.finish(&counts, true).await.unwrap();
    upstream.file(hero_path, Some(hero));
    let corrected = upstream.commit(Some(&bad));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    ingest_source(
        &store,
        &repository,
        Source::ALL[0],
        1024 * 1024,
        &mut counts,
        repository.fetch(source),
    )
    .await
    .unwrap();
    assert_eq!(counts.sources_completed, 1);
    assert_eq!(counts.objects_completed, 2);
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.raw_reused, 2);
    assert_eq!(current(&pool).await, expected);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.entity")
            .fetch_one(&pool)
            .await
            .unwrap(),
        identities
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap(),
        receipts + 2
    );
    let profiles: Vec<String> =
        sqlx::query_scalar("SELECT key FROM serve.hero_profile ORDER BY key")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(profiles, vec!["hero_old"]);
    let status: String = sqlx::query_scalar("SELECT status FROM raw.run WHERE target_revision=$1")
        .bind(&bad)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "failed");
    verify_proofs(&pool, source).await;
    println!(
        "identity-limit recovered target={corrected} identities_preserved={identities}: {}",
        counts.value()
    );
    store.finish(&counts, false).await.unwrap();
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.objects_completed, 0);
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.sources_unchanged, 1);
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
    admin.close().await;
}
