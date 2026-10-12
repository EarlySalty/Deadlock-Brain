use super::*;

#[tokio::test]
async fn corrected_wiki_snapshot_rebuilds_unchanged_names_and_failed_identity_projections() {
    let pg = ScratchPg::start();
    let (admin, pool) = database(&pg).await;
    let upstream = Upstream::start(Source::ALL[0]).await;
    let source = upstream.source;
    let state = tempfile::tempdir().unwrap();
    let repository = Repository::open(state.path(), source, 30).await.unwrap();
    let hero_path = crate::IDENTITY_PATHS[0];
    let ability_path = crate::IDENTITY_PATHS[1];
    let hero = br#"{"hero_old":{"MaxHealth":780.0,"BoundAbilities":{"1":{"Key":"ability_old"}}}}"#;
    let ability = br#"{"ability_old":{"AbilityCooldown":26.0}}"#;
    let english = br#"{"hero_old":"Old Hero","hero_removed":"Removed Hero","hero_kept":"Kept Hero","ability_old":"Old Ability","ability_new":"New Ability","ability_kept":"Kept Ability"}"#;
    let german = r#"{"hero_old":"Alte Heldin","hero_removed":"Entfernte Heldin","hero_kept":"Gebliebene Heldin","ability_old":"Alte Fähigkeit","ability_new":"Neue Fähigkeit","ability_kept":"Gebliebene Fähigkeit"}"#.as_bytes();
    upstream.file(hero_path, Some(hero));
    upstream.file(ability_path, Some(ability));
    upstream.file(crate::LOCALIZATION_PATHS[0], Some(english));
    upstream.file(crate::LOCALIZATION_PATHS[1], Some(german));
    let initial = upstream.commit(None);
    let mut store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    let original = current(&pool).await;
    store.finish(&counts, false).await.unwrap();
    upstream.file(hero_path, Some(br#"{"hero_old":{"MaxHealth":900,"BoundAbilities":{"1":{"Key":"ability_new"}}},"hero_removed":{"MaxHealth":100},"hero_kept":{"MaxHealth":300,"BoundAbilities":{"1":{"Key":"ability_kept"}}}}"#));
    upstream.file(
        ability_path,
        Some(br#"{"ability_new":{"AbilityCooldown":99},"ability_kept":{"AbilityCooldown":20}}"#),
    );
    upstream.file(crate::LOCALIZATION_PATHS[1], Some(b"{"));
    let bad = upstream.commit(Some(&initial));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    let error = source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap_err();
    assert!(error.downcast_ref::<InvalidTarget>().is_some(), "{error:#}");
    println!("partial localization failure: {error:#}");
    assert_eq!(counts.raw_inserted, 2);
    assert_eq!(counts.raw_reused, 1);
    assert_eq!(counts.objects_completed, 3);
    assert_eq!(current(&pool).await, original);
    let max_raw: i64 = sqlx::query_scalar("SELECT max(id) FROM raw.object")
        .fetch_one(&pool)
        .await
        .unwrap();
    let max_receipt: i64 = sqlx::query_scalar("SELECT max(id) FROM canon.receipt")
        .fetch_one(&pool)
        .await
        .unwrap();
    let raw = snapshot(&pool, "raw.object", max_raw).await;
    let receipts = snapshot(&pool, "canon.receipt", max_receipt).await;
    let entities: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e")
            .fetch_one(&pool)
            .await
            .unwrap();
    store.finish(&counts, true).await.unwrap();
    upstream.file(hero_path, Some(br#"{"hero_old":{"MaxHealth":780.0,"BoundAbilities":{"1":{"Key":"ability_old"}}},"hero_kept":{"MaxHealth":301,"BoundAbilities":{"1":{"Key":"ability_kept"}}}}"#));
    upstream.file(
        ability_path,
        Some(br#"{"ability_old":{"AbilityCooldown":26.0},"ability_kept":{"AbilityCooldown":21}}"#),
    );
    upstream.file(crate::LOCALIZATION_PATHS[1], Some(german));
    let corrected = upstream.commit(Some(&bad));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.raw_inserted, 2);
    assert_eq!(counts.raw_reused, 2);
    assert_eq!(counts.objects_completed, 4);
    let values: Vec<(String,String,String)> = sqlx::query_as("SELECT e.key,f.stat_key,f.value_num::text FROM serve.current_fact f
        JOIN canon.entity e ON e.id=f.entity_id WHERE f.value_num IS NOT NULL ORDER BY e.key,f.stat_key")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(
        values,
        vec![
            (
                "ability_kept".into(),
                "ability.AbilityCooldown".into(),
                "21".into()
            ),
            (
                "ability_old".into(),
                "ability.AbilityCooldown".into(),
                "26.0".into()
            ),
            ("hero_kept".into(), "stats.max_health".into(), "301".into()),
            ("hero_old".into(), "stats.max_health".into(), "780.0".into()),
        ]
    );
    let links: Vec<(String,String)> = sqlx::query_as("SELECT h.key,a.key FROM serve.current_relation r
        JOIN canon.entity h ON h.id=r.from_entity_id JOIN canon.entity a ON a.id=r.to_entity_id ORDER BY h.key")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(
        links,
        vec![
            ("hero_kept".into(), "ability_kept".into()),
            ("hero_old".into(), "ability_old".into())
        ]
    );
    let aliases = super::super::localized_aliases(&pool).await;
    assert_eq!(aliases.len(), 8);
    assert!(aliases.iter().all(|(key, _, _)| [
        "hero_old",
        "hero_kept",
        "ability_old",
        "ability_kept"
    ]
    .contains(&key.as_str())));
    let profiles: Vec<(String, Value, Value)> =
        sqlx::query_as("SELECT key,identity,abilities FROM serve.hero_profile ORDER BY key")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(profiles.len(), 2);
    assert_eq!(profiles[0].0, "hero_kept");
    assert_eq!(profiles[0].1["source_revision"], corrected);
    assert_eq!(
        profiles[0].2["links"][0]["identity"]["source_revision"],
        corrected
    );
    assert_eq!(profiles[1].0, "hero_old");
    assert_eq!(profiles[1].1["source_revision"], initial);
    let entities_after: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(entities_after, entities);
    assert_eq!(snapshot(&pool, "raw.object", max_raw).await, raw);
    assert_eq!(
        snapshot(&pool, "canon.receipt", max_receipt).await,
        receipts
    );
    verify_proofs(&pool, source).await;
    println!(
        "corrected localization and identity projection: {}",
        counts.value()
    );
    store.finish(&counts, false).await.unwrap();
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.objects_completed, 0);
    assert_eq!(counts.sources_unchanged, 1);
    println!(
        "unchanged localization and identity projection: {}",
        counts.value()
    );
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
    admin.close().await;
}

#[tokio::test]
async fn both_sources_resume_same_target_after_a_transient_database_interruption() {
    for original_source in Source::ALL {
        let pg = ScratchPg::start();
        let (admin, pool) = database(&pg).await;
        let upstream = Upstream::start(original_source).await;
        let source = upstream.source;
        let state = tempfile::tempdir().unwrap();
        let repository = Repository::open(state.path(), source, 30).await.unwrap();
        let (hero_path, ability_path, hero, ability) = if source.key == "deadlock-data" {
            (
                "data/json/hero-data.json",
                "data/json/ability-data.json",
                br#"{"hero_old":{"MaxHealth":780.0}}"#.as_slice(),
                br#"{"ability_old":{"AbilityCooldown":26.0}}"#.as_slice(),
            )
        } else {
            ("game/citadel/pak01_dir/scripts/heroes.vdata", "game/citadel/pak01_dir/scripts/abilities.vdata",
             br#"hero_old { m_mapStartingStats { EMaxHealth "780.0" } }"#.as_slice(),
             br#"ability_old { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "26.0" } } }"#.as_slice())
        };
        upstream.file(hero_path, Some(hero));
        upstream.file(ability_path, Some(ability));
        let target = upstream.commit(None);
        sqlx::raw_sql(&format!("CREATE FUNCTION ops.test_interruption() RETURNS trigger LANGUAGE plpgsql
            SET search_path=pg_catalog,pg_temp AS $fixture$
            BEGIN IF NEW.path='{hero_path}' THEN RAISE EXCEPTION 'interrupted' USING ERRCODE='23514'; END IF;
            RETURN NEW; END $fixture$;
            CREATE TRIGGER test_interruption BEFORE INSERT ON raw.run_object FOR EACH ROW EXECUTE FUNCTION ops.test_interruption()"))
            .execute(&admin).await.unwrap();
        let mut store = Store::start(pool.clone(), 300).await.unwrap();
        let mut counts = Counts::default();
        let error = source_ingest(&store, &repository, source, &mut counts)
            .await
            .unwrap_err();
        assert!(error.downcast_ref::<InvalidTarget>().is_none());
        assert_eq!(
            error
                .downcast_ref::<sqlx::Error>()
                .and_then(sqlx::Error::as_database_error)
                .and_then(|error| error.code())
                .as_deref(),
            Some("23514"),
            "{error:#}"
        );
        assert_eq!(counts.objects_completed, 1);
        assert_eq!(counts.raw_inserted, 1);
        let run: i64 = sqlx::query_scalar("SELECT id FROM raw.run WHERE target_revision=$1")
            .bind(&target)
            .fetch_one(&pool)
            .await
            .unwrap();
        let raw = snapshot(&pool, "raw.object", i64::MAX).await;
        let receipts = snapshot(&pool, "canon.receipt", i64::MAX).await;
        store.finish(&counts, true).await.unwrap();
        sqlx::query("DROP TRIGGER test_interruption ON raw.run_object")
            .execute(&admin)
            .await
            .unwrap();
        store = Store::start(pool.clone(), 300).await.unwrap();
        counts = Counts::default();
        source_ingest(&store, &repository, source, &mut counts)
            .await
            .unwrap();
        assert_eq!(counts.objects_completed, 1);
        assert_eq!(counts.sources_completed, 1);
        assert_eq!(counts.sources_unchanged, 1);
        let same: i64 = sqlx::query_scalar(
            "SELECT id FROM raw.run WHERE target_revision=$1 AND status='succeeded'",
        )
        .bind(&target)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(same, run);
        assert_eq!(snapshot(&pool, "raw.object", 1).await, raw);
        assert_eq!(snapshot(&pool, "canon.receipt", 1).await, receipts);
        let failures: i64 =
            sqlx::query_scalar("SELECT count(*) FROM raw.run WHERE status='failed'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(failures, 0);
        println!(
            "transient same-target resume {}: {}",
            source.key,
            counts.value()
        );
        store.finish(&counts, false).await.unwrap();
        pool.close().await;
        admin.close().await;
    }
}
