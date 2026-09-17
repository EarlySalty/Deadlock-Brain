use super::*;
use crate::{AbilityStep, PopulationItem};
use serde_json::json;
mod regression_tests;

fn item(id: i64, property: &str) -> ItemModel {
    serde_json::from_value(json!({"item_id":id,"name":format!("item {id}"),
        "class_name":format!("class_{id}"),"slot":"Spirit","tier":2,"cost":1600,
        "is_active":false,"shopable":true,"disabled":false,"damage_axis":"Spirit",
        "defense_kind":[],"properties":{property:10.0},"passive_properties":{},
        "condition":"None","proc_cooldown":null,"imbueable":true}))
    .unwrap()
}
fn catalog() -> Vec<ItemModel> {
    (1..=30)
        .map(|id| {
            item(
                id,
                if id <= 10 {
                    "TechPower"
                } else if id <= 20 {
                    "MeleeDamage"
                } else {
                    "BonusFireRate"
                },
            )
        })
        .collect()
}
fn observations(start: usize, n: usize, ids: &[i64]) -> Vec<BuildObservation> {
    (start..start + n)
        .map(|i| BuildObservation {
            key: format!("match:{i}"),
            participant: format!("player:{i}"),
            source: ObservationSource::Player,
            items: ids.to_vec(),
            imbues: BTreeMap::new(),
            skill_order: Vec::new(),
            buy_times_s: BTreeMap::new(),
            sold_times_s: BTreeMap::new(),
            observed_at: Some(1000),
            won: Some(true),
            warnings: Vec::new(),
        })
        .collect()
}
fn policy() -> FamilyPolicy {
    FamilyPolicy {
        min_matches: 6,
        min_players: 3,
        ..Default::default()
    }
}
fn discover(rows: &[BuildObservation]) -> FamilyDiscovery {
    detect_families(rows, &catalog(), &PopulationPrior::default(), &policy())
}
fn published(d: &FamilyDiscovery) -> Vec<&BuildFamily> {
    d.families
        .iter()
        .filter(|f| f.eligible_for_planning)
        .collect()
}
fn order(primary: i64, secondary: i64) -> Vec<AbilityStep> {
    [
        primary, secondary, primary, primary, primary, secondary, secondary, secondary,
    ]
    .iter()
    .map(|id| AbilityStep {
        ability_id: *id,
        currency_type: 1,
        delta: -1,
    })
    .collect()
}

#[test]
fn one_dominant_family_is_not_artificially_split() {
    let rows = observations(0, 100, &[1, 2, 3, 4, 5, 6]);
    let d = detect_families(
        &rows,
        &catalog(),
        &PopulationPrior::default(),
        &FamilyPolicy::default(),
    );
    assert_eq!(published(&d).len(), 1);
    assert_eq!(published(&d)[0].player_matches, 100);
}
#[test]
fn disjoint_two_families_are_not_averaged() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[11, 12, 13, 14, 15, 16]));
    let d = discover(&rows);
    let families = published(&d);
    assert_eq!(families.len(), 2);
    assert!(families
        .iter()
        .all(|f| f.items.len() == 6 && f.items.iter().all(|i| i.staple)));
}
#[test]
fn three_mechanically_distinct_families_remain_distinct() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[11, 12, 13, 14, 15, 16]));
    rows.extend(observations(24, 12, &[21, 22, 23, 24, 25, 26]));
    assert_eq!(published(&discover(&rows)).len(), 3);
}
#[test]
fn thin_cluster_is_visible_but_not_publishable() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 2, &[11, 12, 13, 14, 15, 16]));
    let d = discover(&rows);
    assert_eq!(d.families.len(), 2);
    assert_eq!(published(&d).len(), 1);
    assert!(d
        .families
        .iter()
        .any(|f| f.player_matches == 2 && !f.limitations.is_empty()));
}
#[test]
fn nearly_identical_item_sets_share_a_family() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[1, 2, 3, 4, 5, 7]));
    assert_eq!(published(&discover(&rows)).len(), 1);
}
#[test]
fn reliable_imbue_disagreement_separates_otherwise_identical_items() {
    let mut rows = observations(0, 24, &[1, 2, 3, 4, 5, 6]);
    for (i, row) in rows.iter_mut().enumerate() {
        row.imbues.insert(1, if i < 12 { 101 } else { 102 });
    }
    let d = discover(&rows);
    assert_eq!(published(&d).len(), 2);
    assert!(published(&d)
        .iter()
        .all(|f| f.imbues.len() == 1 && f.imbues[0].share == 1.0));
}
#[test]
fn skill_focus_preserves_family_specific_observed_orders() {
    let mut rows = observations(0, 24, &[1, 2, 3, 4, 5, 6]);
    for (i, row) in rows.iter_mut().enumerate() {
        if i < 12 {
            row.items = vec![1, 2, 3, 4, 7, 8];
            row.skill_order = order(101, 102);
        } else {
            row.items = vec![1, 2, 3, 5, 9, 10];
            row.skill_order = order(102, 101);
        }
    }
    let d = discover(&rows);
    let families = published(&d);
    assert_eq!(families.len(), 2);
    assert_ne!(families[0].skill_order, families[1].skill_order);
    assert!(families.iter().all(|f| f.skill_order_support == Some(1.0)));
}
#[test]
fn globally_rare_item_can_be_a_family_staple_and_global_staple_can_be_absent() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[11, 12, 13, 14, 15, 16]));
    let global = PopulationPrior::from_items([
        PopulationItem {
            item_id: 1,
            prevalence: 0.45,
            median_position: Some(1.0),
            is_staple: false,
        },
        PopulationItem {
            item_id: 11,
            prevalence: 0.8,
            median_position: Some(1.0),
            is_staple: true,
        },
    ]);
    let d = detect_families(&rows, &catalog(), &global, &policy());
    let family = d
        .families
        .iter()
        .find(|f| f.items.iter().any(|i| i.item_id == 1))
        .unwrap();
    let local = family.items.iter().find(|i| i.item_id == 1).unwrap();
    assert!(local.staple);
    assert_eq!(local.prevalence, 1.0);
    assert_eq!(local.global_prevalence, Some(0.45));
    assert!(!family.items.iter().any(|i| i.item_id == 11));
}
#[test]
fn renaming_and_input_permutations_cannot_change_numeric_discovery() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[11, 12, 13, 14, 15, 16]));
    let first = serde_json::to_vec(&discover(&rows)).unwrap();
    rows.reverse();
    let second = serde_json::to_vec(&discover(&rows)).unwrap();
    let mut items = catalog();
    for item in &mut items {
        item.name = "completely renamed".into();
        item.description = "irrelevant".into();
    }
    let third = serde_json::to_vec(&detect_families(
        &rows,
        &items,
        &PopulationPrior::default(),
        &policy(),
    ))
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(second, third);
}
#[test]
fn repeated_rows_do_not_inflate_sample_size() {
    let rows = observations(0, 5, &[1, 2, 3, 4, 5, 6]);
    let repeated = rows.iter().cloned().cycle().take(50).collect::<Vec<_>>();
    let d = discover(&repeated);
    assert_eq!(d.duplicate_observations, 45);
    assert!(published(&d).is_empty());
}
#[test]
fn one_prolific_player_is_not_independent_population_support() {
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    for row in &mut rows {
        row.participant = "same player".into();
    }
    let d = discover(&rows);
    assert!(published(&d).is_empty());
    assert_eq!(d.families[0].distinct_players, 1);
}
#[test]
fn missing_skills_and_imbues_are_unknown_not_zero_quality_observations() {
    let d = discover(&observations(0, 12, &[1, 2, 3, 4, 5, 6]));
    assert_eq!(d.families[0].skill_order_support, None);
    assert!(d.families[0].imbues.is_empty());
    assert_eq!(published(&d).len(), 1);
}
#[test]
fn upgrade_components_normalize_without_duplicate_identity_weight() {
    let mut items = catalog();
    items[6].component_items = vec![items[0].class_name.clone()];
    let mut rows = observations(0, 12, &[1, 2, 3, 4, 5, 6]);
    rows.extend(observations(12, 12, &[7, 2, 3, 4, 5, 6]));
    let d = detect_families(&rows, &items, &PopulationPrior::default(), &policy());
    assert_eq!(published(&d).len(), 1);
}
