use super::features::{family_label, Features};
use super::{
    BuildFamily, BuildObservation, FamilyItem, ImbueEvidence, ObservationSource, PairEvidence,
};
use crate::{AbilityStep, PopulationPrior};
use std::collections::{BTreeMap, BTreeSet};

fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let n = values.len();
    Some(if n.is_multiple_of(2) {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    } else {
        values[n / 2]
    })
}

fn order_consensus(rows: &[&BuildObservation]) -> (Vec<AbilityStep>, Option<f64>) {
    let mut groups = BTreeMap::<Vec<(i64, i64, i64)>, Vec<&BuildObservation>>::new();
    for row in rows.iter().filter(|o| o.skill_order.len() >= 8) {
        groups
            .entry(super::order_key(&row.skill_order[..8]))
            .or_default()
            .push(row);
    }
    let total: usize = groups.values().map(Vec::len).sum();
    let Some((_, group)) = groups
        .iter()
        .max_by(|a, b| a.1.len().cmp(&b.1.len()).then_with(|| b.0.cmp(a.0)))
    else {
        return (Vec::new(), None);
    };
    let share = group.len() as f64 / total as f64;
    // Select an observed coherent sequence, never synthesize incompatible
    // per-position modes. Support is explicitly the eight-step prefix support.
    let mut orders: BTreeMap<_, usize> = BTreeMap::new();
    for row in group {
        *orders
            .entry(super::order_key(&row.skill_order))
            .or_default() += 1;
    }
    let key = orders
        .iter()
        .max_by(|a, b| {
            a.1.cmp(b.1)
                .then_with(|| a.0.len().cmp(&b.0.len()))
                .then_with(|| b.0.cmp(a.0))
        })
        .map(|(key, _)| key);
    let order = key
        .into_iter()
        .flatten()
        .map(|(ability_id, currency_type, delta)| AbilityStep {
            ability_id: *ability_id,
            currency_type: *currency_type,
            delta: *delta,
        })
        .collect();
    (order, Some(share))
}

pub(super) fn summarize(
    members: &[usize],
    all: &[BuildObservation],
    center: &Features,
    global: &PopulationPrior,
    total_players: usize,
) -> BuildFamily {
    let rows: Vec<_> = members.iter().map(|i| &all[*i]).collect();
    let players: Vec<_> = rows
        .iter()
        .copied()
        .filter(|o| o.source == ObservationSource::Player)
        .collect();
    let authors: Vec<_> = rows
        .iter()
        .copied()
        .filter(|o| o.source == ObservationSource::Author)
        .collect();
    let population = if players.is_empty() {
        &authors
    } else {
        &players
    };
    let n = population.len();
    let mut counts = BTreeMap::<i64, usize>::new();
    let mut positions = BTreeMap::<i64, Vec<f64>>::new();
    let mut times = BTreeMap::<i64, Vec<f64>>::new();
    let mut sales = BTreeMap::<i64, usize>::new();
    let mut imbue_counts = BTreeMap::<(i64, i64), usize>::new();
    let mut pairs = BTreeMap::<(i64, i64), usize>::new();
    for row in population {
        let mut seen = BTreeSet::new();
        for (pos, id) in row
            .items
            .iter()
            .enumerate()
            .filter(|(_, id)| seen.insert(**id))
        {
            *counts.entry(*id).or_default() += 1;
            positions.entry(*id).or_default().push(pos as f64 + 1.0);
            if let Some(time) = row.buy_times_s.get(id) {
                times.entry(*id).or_default().push(*time as f64);
            }
            if row.sold_times_s.contains_key(id) {
                *sales.entry(*id).or_default() += 1;
            }
            if let Some(target) = row.imbues.get(id) {
                *imbue_counts.entry((*id, *target)).or_default() += 1;
            }
        }
        let ids: Vec<_> = seen.into_iter().collect();
        for (i, a) in ids.iter().enumerate() {
            for b in &ids[i + 1..] {
                *pairs.entry((*a, *b)).or_default() += 1;
            }
        }
    }
    let items = counts
        .iter()
        .map(|(id, count)| FamilyItem {
            item_id: *id,
            observations: *count,
            prevalence: *count as f64 / n as f64,
            global_prevalence: (!global.is_empty()).then(|| global.prevalence(*id)),
            median_position: median(positions.remove(id).unwrap_or_default()),
            median_buy_time_s: median(times.remove(id).unwrap_or_default()),
            sell_rate: {
                let observed = players
                    .iter()
                    .filter(|row| row.buy_times_s.contains_key(id))
                    .count();
                (observed > 0).then(|| *sales.get(id).unwrap_or(&0) as f64 / observed as f64)
            },
            staple: *count as f64 / n as f64 >= dbrain_population::STAPLE_THRESHOLD,
        })
        .collect::<Vec<_>>();
    let mut pair_evidence = pairs
        .into_iter()
        .filter(|(_, count)| *count >= 3 && *count as f64 / n as f64 >= 0.25)
        .map(|((left, right), observations)| PairEvidence {
            left,
            right,
            observations,
            prevalence: observations as f64 / n as f64,
        })
        .collect::<Vec<_>>();
    pair_evidence.sort_by(|a, b| {
        b.observations
            .cmp(&a.observations)
            .then_with(|| (a.left, a.right).cmp(&(b.left, b.right)))
    });
    pair_evidence.truncate(40);
    let mut imbue_totals = BTreeMap::<i64, usize>::new();
    for ((id, _), count) in &imbue_counts {
        *imbue_totals.entry(*id).or_default() += count;
    }
    let imbues = imbue_counts
        .into_iter()
        .map(|((item_id, ability_id), observations)| ImbueEvidence {
            item_id,
            ability_id,
            observations,
            share: observations as f64 / imbue_totals[&item_id] as f64,
        })
        .collect();
    let (skill_order, skill_order_support) = order_consensus(population);
    // A six-root prefix is not a family identity: disjoint late-game plans can
    // share all six early items. Fingerprint every mechanical feature, with
    // separate domains for roots/axes/focus/bindings. Quantization applies ONLY
    // to the identifier (one part per million), never to scores or clustering;
    // equivalent cohort proportions do not acquire new IDs from float noise.
    let quantize = |value: f64| (value * 1_000_000.0).round() as i64;
    let signature = serde_json::to_vec(&(
        "family-centroid-v2",
        center
            .roots
            .iter()
            .map(|(id, value)| (*id, quantize(*value)))
            .collect::<Vec<_>>(),
        center
            .axes
            .iter()
            .map(|(axis, value)| (*axis, quantize(*value)))
            .collect::<Vec<_>>(),
        center
            .focus
            .iter()
            .map(|(id, value)| (*id, quantize(*value)))
            .collect::<Vec<_>>(),
        center
            .imbues
            .iter()
            .map(|((id, target), value)| (*id, *target, quantize(*value)))
            .collect::<Vec<_>>(),
    ))
    .expect("integer mechanical signature is serializable");
    let id = super::identity(signature);
    let mut limitations = rows
        .iter()
        .flat_map(|r| r.warnings.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if players.is_empty() {
        limitations.push("Keine zugeordneten Spieler-Matches; Häufigkeiten sind Autoren-Support, keine Spieler-Pickrate.".into());
    }
    if skill_order_support.is_none_or(|s| s < 0.50) {
        limitations.push("Keine stabile familientypische Acht-Schritt-Skill-Signatur; gemeinsame Skillorder bleibt Fallback.".into());
    }
    BuildFamily {
        id,
        label: family_label(&center.axes),
        player_matches: players.len(),
        patch_started_at: None,
        post_patch_player_matches: None,
        distinct_players: players
            .iter()
            .map(|o| &o.participant)
            .collect::<BTreeSet<_>>()
            .len(),
        source_builds: authors.len(),
        distinct_authors: authors
            .iter()
            .map(|o| &o.participant)
            .collect::<BTreeSet<_>>()
            .len(),
        population_share: (total_players > 0).then(|| players.len() as f64 / total_players as f64),
        cohesion: 0.0,
        axes: center.axes.clone(),
        items,
        pairs: pair_evidence,
        skill_order,
        skill_order_support,
        imbues,
        first_observed_at: population.iter().filter_map(|o| o.observed_at).min(),
        last_observed_at: population.iter().filter_map(|o| o.observed_at).max(),
        eligible_for_planning: false,
        limitations,
        member_keys: rows.iter().map(|o| o.key.clone()).collect(),
    }
}
