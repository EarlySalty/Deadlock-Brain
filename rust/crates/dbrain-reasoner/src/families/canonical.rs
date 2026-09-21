//! One identity normalization for discovery, conditioning and held-out data.
//! Conflicting imports without source-version evidence are quarantined, not
//! resolved by arrival order or by whichever payload would pass a patch gate.
use super::{BuildObservation, ObservationSource};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct CanonicalObservations {
    pub rows: Vec<BuildObservation>,
    pub duplicate_rows: usize,
    pub conflicting_keys: BTreeSet<String>,
}

fn same_evidence(left: &BuildObservation, right: &BuildObservation) -> bool {
    left.source == right.source
        && left.participant == right.participant
        && left.items == right.items
        && left.imbues == right.imbues
        && left.skill_order == right.skill_order
        && left.buy_times_s == right.buy_times_s
        && left.sold_times_s == right.sold_times_s
        && left.observed_at == right.observed_at
        && left.won == right.won
}

pub(super) fn observations(input: &[BuildObservation]) -> CanonicalObservations {
    let mut groups = BTreeMap::<&str, Vec<&BuildObservation>>::new();
    for row in input {
        groups.entry(&row.key).or_default().push(row);
    }
    let duplicate_rows = input.len() - groups.len();
    let mut rows = Vec::with_capacity(groups.len());
    let mut conflicting_keys = BTreeSet::new();
    for (key, group) in groups {
        let first = group[0];
        if group.iter().any(|row| !same_evidence(first, row)) {
            conflicting_keys.insert(key.to_string());
            continue;
        }
        let mut row = first.clone();
        // Diagnostics cannot break a numeric tie, but none may disappear when
        // the same match arrived through more than one import path.
        row.warnings = group
            .iter()
            .flat_map(|row| row.warnings.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        rows.push(row);
    }
    // This is also the centroid summation order in every consumer. Sorting by
    // complete numerical evidence prevents input/SQL order changing floats.
    rows.sort_by(|a, b| {
        (a.source == ObservationSource::Author)
            .cmp(&(b.source == ObservationSource::Author))
            .then_with(|| b.items.len().cmp(&a.items.len()))
            .then_with(|| a.items.cmp(&b.items))
            .then_with(|| a.imbues.cmp(&b.imbues))
            .then_with(|| super::order_key(&a.skill_order).cmp(&super::order_key(&b.skill_order)))
            .then_with(|| a.key.cmp(&b.key))
    });
    CanonicalObservations {
        rows,
        duplicate_rows,
        conflicting_keys,
    }
}
