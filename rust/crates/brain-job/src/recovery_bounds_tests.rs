use super::*;

#[tokio::test]
async fn wiki_numeric_targets_fail_closed_before_corrected_snapshot_reruns() {
    for value in ["1e64", "1e-64", "0e1073741824"] {
        for introduced in [false, true] {
            recovery(Source::ALL[0], TargetFailure::Numeric(value), introduced).await;
        }
    }
}

#[tokio::test]
async fn gametracking_numeric_targets_fail_closed_before_corrected_snapshot_reruns() {
    for value in ["1e64", "1e-64", "0e1073741824"] {
        for introduced in [false, true] {
            recovery(Source::ALL[1], TargetFailure::Numeric(value), introduced).await;
        }
    }
}

fn numeric_documents(value: &str) -> Vec<(Source, &'static str, String, String)> {
    vec![
        (
            Source::ALL[0],
            "data/json/hero-data.json",
            format!(r#"{{"hero_test":{{"MaxHealth":{value}}}}}"#),
            value.into(),
        ),
        (
            Source::ALL[0],
            "data/json/ability-data.json",
            format!(r#"{{"ability_test":{{"AbilityCooldown":{{"Value":{value}}}}}}}"#),
            value.into(),
        ),
        (
            Source::ALL[0],
            "data/json/ability-data.json",
            format!(r#"{{"ability_test":{{"Upgrades":[{{"AbilityCooldown":{value}}}]}}}}"#),
            value.into(),
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/heroes.vdata",
            format!(r#"hero_test {{ m_mapStartingStats {{ EMaxHealth "{value}" }} }}"#),
            value.into(),
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            format!(
                r#"ability_test {{ m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties {{ AbilityCooldown {{ m_strValue "{value}" }} }} }}"#
            ),
            value.into(),
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/heroes.vdata",
            format!(
                r#"<!-- kv3 encoding:text --> {{ hero_test = {{ m_mapStartingStats = {{ EMaxHealth = {value} }} }} }}"#
            ),
            value.into(),
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            format!(
                r#"<!-- kv3 encoding:text --> {{ ability_test = {{ m_eAbilityType = "EAbilityType_Signature" m_mapAbilityProperties = {{ AbilityCooldown = {{ m_strValue = "{value}" }} }} }} }}"#
            ),
            format!("\"{value}\""),
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            format!(
                r#"<!-- kv3 encoding:text --> {{ ability_test = {{ m_eAbilityType = "EAbilityType_Signature" m_vecAbilityUpgrades = [ {{ m_vecPropertyUpgrades = [ {{ m_strPropertyName = "AbilityCooldown" m_strBonus = "{value}" }} ] }} ] }} }}"#
            ),
            format!("\"{value}\""),
        ),
    ]
}

#[tokio::test]
async fn numeric_storage_acceptance_matches_postgres_text_without_changing_lexemes_or_hashes() {
    let pg = ScratchPg::start();
    let pool = pg.pool("brain_test_dba").await;
    let mut values: Vec<String> = [
        "1e63",
        "1e64",
        "-1e62",
        "-1e63",
        "1e-62",
        "1e-63",
        "1e-64",
        "-1e-61",
        "-1e-62",
        "0e-62",
        "-0e-62",
        "0e-63",
        "0.000e-59",
        "0.000e-60",
        "1.2300e2",
        "1.2300e3",
        "0.0001e67",
        "0.0001e68",
        "0.00100e-57",
        "0.00100e-58",
        "0e131071",
        "0e131072",
        "0e131073",
        "0e1073741822",
        "0e1073741823",
        "0e1073741824",
        "0e2147483647",
        "0e2147483648",
        "0e999999999999999999999999",
        "0e-1073741823",
        "0e-16383",
        "0e-16384",
        "1e131071",
        "1e131072",
        "1e-16383",
        "1e-16384",
        "-0",
        "780.0",
        "26.0",
        "-12.0",
        "1e2",
        "0.50",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    for exponent in [-64, -63, -62, -61, -60, -59, 60, 61, 62, 63, 64, 65, 67, 68] {
        for mantissa in ["0", "-0", "1", "-1", "12.00", "-0.00100", "0.0001"] {
            values.push(format!("{mantissa}E{exponent:+}"));
        }
    }
    for length in [62, 63, 64, 65] {
        values.extend([
            "1".repeat(length),
            format!("-{}", "1".repeat(length)),
            format!("0.{}", "0".repeat(length)),
            format!("-0.{}", "0".repeat(length)),
        ]);
    }
    for value in values {
        let stored: Result<(String, i32), sqlx::Error> = sqlx::query_as(
            "SELECT left(n::text,80),length(n::text) FROM (SELECT $1::text::numeric n) q",
        )
        .bind(&value)
        .fetch_one(&pool)
        .await;
        let (representation, length, sqlstate) = match stored {
            Ok((representation, length)) => (Some(representation), Some(length), None),
            Err(error) => {
                let code = error
                    .as_database_error()
                    .unwrap()
                    .code()
                    .unwrap()
                    .to_string();
                assert_eq!(code, "22003", "{value}: {error}");
                (None, None, Some(code))
            }
        };
        let supported = value.len() <= 64 && length.is_some_and(|length| length <= 64);
        for (case, (source, path, document, lexeme)) in
            numeric_documents(&value).into_iter().enumerate()
        {
            let extracted = extract::extract(source, path, document.as_bytes());
            assert_eq!(
                extracted.is_ok(),
                supported,
                "{value} case={case} postgres={representation:?} length={length:?}"
            );
            let hash = if supported {
                let facts = extracted.unwrap();
                assert_eq!(facts.len(), 1, "{value} case={case}");
                assert!(
                    matches!(&facts[0].value, extract::Atom::Number(number) if number == &value)
                );
                assert_eq!(facts[0].hash, Sha256::digest(lexeme.as_bytes()).to_vec());
                assert_eq!(extract::VERSION, "p1-v1");
                extract::prove(source, path, document.as_bytes(), &facts).unwrap();
                Some(
                    facts[0]
                        .hash
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect::<String>(),
                )
            } else {
                None
            };
            println!(
                "numeric-boundary {}",
                serde_json::json!({"value":value,"case":case,"source":source.key,"supported":supported,"postgres_prefix":representation,"postgres_length":length,"sqlstate":sqlstate,"source_lexeme":lexeme,"value_sha256":hash,"extractor_version":extract::VERSION})
            );
        }
    }
    pool.close().await;
}

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
