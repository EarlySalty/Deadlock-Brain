//! Holdout assignment never refits clusters on the withheld observations.
use super::features::{centroid, features, similarity, upgrade_roots};
use super::{BuildObservation, FamilyDiscovery, ObservationSource};
use crate::{ItemModel, PopulationItem, PopulationPrior};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeldOutAssignment {
    pub observation_key: String,
    pub family_id: Option<String>,
    pub similarity: Option<f64>,
    #[serde(default)]
    pub exclusion_reason: Option<String>,
}

pub fn assign_holdout(
    discovery: &FamilyDiscovery,
    training: &[BuildObservation],
    holdout: &[BuildObservation],
    items: &[ItemModel],
) -> Vec<HeldOutAssignment> {
    let training = super::canonical::observations(training).rows;
    let holdout = super::canonical::observations(holdout);
    let roots = upgrade_roots(items);
    let catalog = items
        .iter()
        .filter(|item| item.shopable && !item.disabled)
        .map(|item| (item.item_id, item))
        .collect();
    let training_features = training
        .iter()
        .map(|row| features(row, &catalog, &roots))
        .collect::<Vec<_>>();
    let centers = discovery
        .families
        .iter()
        .filter(|family| family.eligible_for_planning)
        .filter_map(|family| {
            let members = training
                .iter()
                .enumerate()
                .filter(|(_, row)| family.member_keys.contains(&row.key))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            (!members.is_empty())
                .then(|| (family.id.clone(), centroid(&members, &training_features)))
        })
        .collect::<Vec<_>>();
    let mut output = holdout
        .rows
        .iter()
        .map(|row| {
            let value = features(row, &catalog, &roots);
            let nearest = centers
                .iter()
                .map(|(id, center)| (id, similarity(&value, center)))
                .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(a.0)));
            let family_id = nearest
                .filter(|(_, score)| {
                    value.roots.len() >= 3 && *score >= discovery.policy.join_similarity
                })
                .map(|(id, _)| id.clone());
            let exclusion_reason = family_id.is_none().then(|| {
                if value.roots.len() < 3 {
                    "Zu wenige bekannte Itemwurzeln für eine Familienzuordnung."
                } else {
                    "Keine ausreichend ähnliche, im Training belegte Familie."
                }
                .to_string()
            });
            HeldOutAssignment {
                observation_key: row.key.clone(),
                family_id,
                similarity: nearest.map(|(_, score)| score),
                exclusion_reason,
            }
        })
        .collect::<Vec<_>>();
    // Rejected identities remain visible once, rather than disappearing from
    // the assignment report or inflating an unrelated family's sample size.
    output.extend(holdout.conflicting_keys.into_iter().map(|observation_key| {
        HeldOutAssignment {
            observation_key,
            family_id: None,
            similarity: None,
            exclusion_reason: Some(
                "Widersprüchliche Importe derselben Beobachtung; ohne Quellversion ausgeschlossen."
                    .into(),
            ),
        }
    }));
    output.sort_by(|a, b| a.observation_key.cmp(&b.observation_key));
    output
}

/// Unweighted observed player purchase frequencies for independent evaluation.
/// Authors never count as player-matches; absence of player data stays empty.
pub fn observed_population(rows: &[BuildObservation]) -> PopulationPrior {
    let canonical = super::canonical::observations(rows);
    let players = canonical
        .rows
        .iter()
        .filter(|row| row.source == ObservationSource::Player)
        .collect::<Vec<_>>();
    if players.is_empty() {
        return PopulationPrior::default();
    }
    let mut positions = BTreeMap::<i64, Vec<f64>>::new();
    for row in &players {
        let mut seen = std::collections::BTreeSet::new();
        for (index, id) in row
            .items
            .iter()
            .enumerate()
            .filter(|(_, id)| seen.insert(**id))
        {
            positions.entry(*id).or_default().push(index as f64 + 1.0);
        }
    }
    PopulationPrior::from_items(positions.into_iter().map(|(item_id, mut positions)| {
        positions.sort_by(f64::total_cmp);
        let count = positions.len();
        let prevalence = count as f64 / players.len() as f64;
        let median = if count.is_multiple_of(2) {
            (positions[count / 2 - 1] + positions[count / 2]) / 2.0
        } else {
            positions[count / 2]
        };
        PopulationItem {
            item_id,
            prevalence,
            median_position: Some(median),
            is_staple: prevalence >= dbrain_population::STAPLE_THRESHOLD,
        }
    }))
}

/// Entire participants are assigned to one side; matches by the same person
/// cannot leak between training and validation. This is not an external review.
pub fn is_holdout(row: &BuildObservation) -> bool {
    // Production player/author observations both use the same account ID.
    // The role describes evidence, not a different person. Keep the existing
    // player partition unchanged and align author records to that partition.
    let hash = super::identity("player:".bytes().chain(row.participant.bytes()));
    u64::from_str_radix(hash.trim_start_matches("family-"), 16)
        .is_ok_and(|value| value.is_multiple_of(5))
}
