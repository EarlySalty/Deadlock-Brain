use super::*;

#[test]
fn one_account_cannot_cross_holdout_partitions_by_changing_source_role() {
    for account in 0..1000 {
        let mut player = observations(account, 1, &[1, 2, 3]).remove(0);
        player.participant = account.to_string();
        let mut author = player.clone();
        author.source = ObservationSource::Author;
        author.key = format!("author:{account}");
        assert_eq!(
            is_holdout(&player),
            is_holdout(&author),
            "account {account} occurs as both match player and build author"
        );
    }
}

#[test]
fn distinct_families_with_the_same_six_common_roots_have_unique_ids() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    rows.extend(observations(
        12,
        12,
        &[1, 2, 3, 4, 5, 6, 21, 22, 23, 24, 25, 26, 27, 28],
    ));
    let result = discover(&rows);
    let families = published(&result);
    assert_eq!(
        families.len(),
        2,
        "the observed mechanical families must remain separate"
    );
    assert_ne!(
        families[0].id, families[1].id,
        "different plans must never overwrite one another in variant_scores or holdout assignment"
    );
}

#[test]
fn conflicting_duplicate_imports_have_order_independent_patch_evidence() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    let mut corrected = rows.clone();
    for row in &mut corrected {
        row.observed_at = Some(3000);
        row.won = Some(false);
    }
    rows.extend(corrected);
    let policy = FamilyPolicy {
        patch_started_at: Some(2000),
        ..policy()
    };
    let first = detect_families(&rows, &catalog(), &PopulationPrior::default(), &policy);
    rows.reverse();
    let second = detect_families(&rows, &catalog(), &PopulationPrior::default(), &policy);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
}

#[test]
fn conflicting_duplicate_imports_have_order_independent_holdout_priors() {
    let mut rows = observations(0, 1, &[1, 2, 3]);
    let mut corrected = rows[0].clone();
    corrected.items = vec![11, 12, 13, 14];
    rows.push(corrected);
    let first = observed_population(&rows);
    rows.reverse();
    let second = observed_population(&rows);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
}

#[test]
fn conditioning_cannot_reintroduce_duplicate_matches_into_win_statistics() {
    let rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    let duplicated = rows.iter().cloned().cycle().take(120).collect::<Vec<_>>();
    let cfg = crate::ReasonerConfig::default();
    let input = crate::meta::MetaIndexWithSources {
        index: crate::meta::build_meta_index(&[], &[], &[], &cfg),
        author_builds: Vec::new(),
        hero_ability_orders: BTreeMap::new(),
        core_layouts: crate::CoreLayoutIndex::default(),
        combinations: BTreeMap::new(),
        population: PopulationPrior::default(),
        observations: duplicated.clone(),
        family: None,
    };
    let policy = FamilyPolicy {
        patch_started_at: Some(500),
        ..policy()
    };
    let discovery = detect_families(&duplicated, &catalog(), &input.population, &policy);
    let family = &discovery.families[0];
    assert_eq!(family.player_matches, 12);
    let context = conditioned_meta(&input, family, &cfg);
    assert_eq!(
        context.observations.len(),
        12,
        "deduplication must survive family conditioning"
    );
    assert_eq!(context.population.prevalence(1), 1.0);
}

#[test]
fn duplicate_current_rows_cannot_evict_the_historical_cohort() {
    let mut rows = observations(0, 100, &[1, 2, 3, 4, 5, 6]);
    let mut current = observations(200, 1, &[1, 2, 3, 4, 5, 6]).remove(0);
    current.observed_at = Some(3000);
    rows.extend(std::iter::repeat_n(current, 100));
    let policy = FamilyPolicy {
        patch_started_at: Some(2000),
        ..Default::default()
    };
    let result = detect_families(&rows, &catalog(), &PopulationPrior::default(), &policy);
    assert_eq!(result.duplicate_observations, 99);
    assert_eq!(result.historical_observations_excluded, 0);
    assert_eq!(result.input_observations, 101);
    assert_eq!(result.families[0].post_patch_player_matches, Some(1));
}

#[test]
fn repeated_matches_cannot_inflate_holdout_prevalence() {
    let first = observations(0, 1, &[1, 2, 3]).remove(0);
    let mut rows = std::iter::repeat_n(first, 9).collect::<Vec<_>>();
    rows.extend(observations(1, 1, &[11, 12, 13]));
    assert_eq!(observed_population(&rows).prevalence(1), 0.5);
}

#[test]
fn author_fallback_keys_preserve_different_bindings_and_skill_orders() {
    let source = |target| crate::meta::AuthorBuildSource {
        hero_id: 777,
        author: "same author".into(),
        weight: 1.0,
        details: json!({"modCategories":[{"name":"Core","mods":[{"abilityId":1,"imbue":target},{"abilityId":2},{"abilityId":3}]}]}),
    };
    let first = author_observation(&source(101));
    let second = author_observation(&source(102));
    assert_eq!(first.items, second.items);
    assert_ne!(first.key, second.key);
}

#[test]
fn historical_matches_are_not_current_patch_evidence() {
    let rows = observations(0, 100, &[1, 2, 3, 4, 5, 6]);
    let policy = FamilyPolicy {
        patch_started_at: Some(2000),
        ..Default::default()
    };
    let result = detect_families(&rows, &catalog(), &PopulationPrior::default(), &policy);
    let family = &result.families[0];
    assert!(family.eligible_for_planning);
    assert_eq!(family.post_patch_player_matches, Some(0));
    assert!(family
        .limitations
        .iter()
        .any(|note| note.contains("historische Priors")));
}

#[test]
fn enough_current_matches_exclude_older_competing_family() {
    let mut rows = observations(0, 100, &[1, 2, 3, 4, 5, 6]);
    let mut current = observations(100, 100, &[11, 12, 13, 14, 15, 16]);
    for row in &mut current {
        row.observed_at = Some(3000);
    }
    rows.extend(current);
    let policy = FamilyPolicy {
        patch_started_at: Some(2000),
        ..Default::default()
    };
    let result = detect_families(&rows, &catalog(), &PopulationPrior::default(), &policy);
    assert_eq!(result.historical_observations_excluded, 100);
    assert_eq!(result.duplicate_observations, 0);
    assert_eq!(result.families.len(), 1);
    assert_eq!(result.families[0].post_patch_player_matches, Some(100));
    assert!(result.families[0]
        .items
        .iter()
        .all(|item| item.item_id >= 11));
}

#[test]
fn author_count_cannot_rescue_a_thin_player_cluster() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 2, &[11, 12, 13, 14, 15, 16]));
    let mut authors = observations(100, 8, &[11, 12, 13, 14, 15, 16]);
    for row in &mut authors {
        row.source = ObservationSource::Author;
    }
    rows.extend(authors);
    let result = discover(&rows);
    let thin = result
        .families
        .iter()
        .find(|f| f.player_matches == 2)
        .unwrap();
    assert_eq!(thin.distinct_authors, 8);
    assert!(!thin.eligible_for_planning);
}

#[test]
fn held_out_family_is_not_created_by_validation_data() {
    let training = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    let discovery = discover(&training);
    let mut withheld = observations(100, 5, &[1, 2, 3, 4, 5, 6]);
    withheld.extend(observations(200, 30, &[11, 12, 13, 14, 15, 16]));
    let assignments = assign_holdout(&discovery, &training, &withheld, &catalog());
    assert_eq!(
        assignments
            .iter()
            .filter(|row| row.family_id.is_some())
            .count(),
        5
    );
    assert_eq!(
        assignments
            .iter()
            .filter(|row| row.family_id.is_none())
            .count(),
        30
    );
    assert_eq!(discovery.families.len(), 1);
}

#[test]
fn all_matches_of_one_player_stay_on_the_same_side() {
    let mut rows = observations(0, 20, &[1, 2, 3, 4, 5, 6]);
    for row in &mut rows {
        row.participant = "one person".into();
    }
    assert!(rows
        .iter()
        .all(|row| is_holdout(row) == is_holdout(&rows[0])));
}

#[test]
fn unknown_global_prevalence_and_sale_frequency_are_not_zero() {
    let result = discover(&observations(0, 12, &[1, 2, 3, 4, 5, 6]));
    assert!(result.families[0]
        .items
        .iter()
        .all(|item| item.global_prevalence.is_none() && item.sell_rate.is_none()));
}

#[test]
fn held_out_population_excludes_authors_and_deduplicates_purchases() {
    let mut rows = observations(0, 4, &[1, 1, 1, 2, 3]);
    let mut author = observations(10, 1, &[11, 12, 13]).remove(0);
    author.source = ObservationSource::Author;
    rows.push(author);
    let population = observed_population(&rows);
    assert_eq!(population.prevalence(1), 1.0);
    assert_eq!(population.prevalence(11), 0.0);
}

#[test]
fn unrelated_patch_timestamp_cannot_relabel_the_cohort() {
    let config = crate::ReasonerConfig {
        patch_tag: "patch-b".into(),
        ..Default::default()
    };
    let events = vec![json!({"patch_external_id":"patch-a","posted_at_epoch":1234.0})];
    assert_eq!(
        FamilyPolicy::for_patch(&events, &config).patch_started_at,
        None
    );
}

fn unpublished_build() -> crate::BuildObject {
    let family = discover(&observations(0, 12, &[1, 2, 3, 4, 5, 6]))
        .families
        .remove(0);
    crate::BuildObject {
        hero_id: 700,
        hero_name: "Synthetic".into(),
        patch_tag: "test".into(),
        name: "Test".into(),
        core: Vec::new(),
        situations: Vec::new(),
        ability_order: Vec::new(),
        confidence: crate::Confidence::Low,
        rationale: String::new(),
        family: Some(family),
        variants: Vec::new(),
        family_discovery: None,
    }
}

#[tokio::test]
async fn missing_post_patch_evidence_blocks_before_any_database_connection() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgresql://127.0.0.1:1/unreachable")
        .unwrap();
    let error = crate::publish::enqueue_publish_task(&pool, &unpublished_build())
        .await
        .unwrap_err();
    assert!(
        matches!(error,crate::ReasonerError::Data(note) if note.contains("Nach-Patch-Stichprobe"))
    );
}

#[test]
fn legacy_single_build_publish_cannot_silently_drop_variants() {
    let mut build = unpublished_build();
    build.variants.push(unpublished_build());
    let error = crate::publish::validate_publish_input(&build).unwrap_err();
    assert!(error
        .to_string()
        .contains("Varianten nicht still verwerfen"));
}
