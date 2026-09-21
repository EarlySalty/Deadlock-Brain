use super::{author_observation, BuildFamily, ObservationSource};
use crate::{meta, MetaRow, PopulationItem, PopulationPrior, ReasonerConfig};
use std::collections::{BTreeMap, BTreeSet};

/// Recompute every statistical prior from the assigned player cohort. Global
/// hero prevalence remains in FamilyItem for context, not as a competing prior.
pub fn conditioned_meta(
    input: &meta::MetaIndexWithSources,
    family: &BuildFamily,
    cfg: &ReasonerConfig,
) -> meta::MetaIndexWithSources {
    let mut result = input.clone();
    result.observations = super::canonical::observations(&input.observations).rows;
    result
        .observations
        .retain(|row| family.member_keys.contains(&row.key));
    result
        .author_builds
        .retain(|source| family.member_keys.contains(&author_observation(source).key));
    result.family = Some(family.clone());
    let players = result
        .observations
        .iter()
        .filter(|row| row.source == ObservationSource::Player)
        .collect::<Vec<_>>();
    result.population = if players.is_empty() {
        PopulationPrior::default()
    } else {
        PopulationPrior::from_items(family.items.iter().map(|item| PopulationItem {
            item_id: item.item_id,
            prevalence: item.prevalence,
            median_position: item.median_position,
            is_staple: item.staple,
        }))
    };
    result.population = result
        .population
        .with_imbue_targets(family.reliable_imbues());
    let mut singles = BTreeMap::<i64, (i64, i64)>::new();
    let mut pairs = BTreeMap::<(i64, i64), (i64, i64)>::new();
    for row in &players {
        // A historical prior is useful, but its wins are not current-patch wins.
        if !family
            .patch_started_at
            .zip(row.observed_at)
            .is_some_and(|(start, time)| time >= start)
        {
            continue;
        }
        let Some(won) = row.won else {
            continue;
        };
        let ids = row
            .items
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        for (index, id) in ids.iter().enumerate() {
            let count = singles.entry(*id).or_default();
            count.0 += 1;
            count.1 += i64::from(won);
            for other in &ids[index + 1..] {
                let count = pairs.entry((*id, *other)).or_default();
                count.0 += 1;
                count.1 += i64::from(won);
            }
        }
    }
    let rows = family
        .items
        .iter()
        .map(|item| {
            let (matches, wins) = singles.get(&item.item_id).copied().unwrap_or_default();
            MetaRow {
                item_id: item.item_id,
                patch_tag: cfg.patch_tag.clone(),
                prevalence_builds: matches,
                wins,
                losses: matches - wins,
                matches,
                avg_buy_time_relative: None,
                lift_pp: None,
            }
        })
        .collect::<Vec<_>>();
    // No hero-global claims or win-rate lifts leak into a family. Count each
    // independent author once, regardless of how many versions they submitted.
    result.index = meta::build_meta_index(&rows, &[], &[], cfg);
    for (id, support) in &mut result.index.by_item {
        support.author_hits = result
            .author_builds
            .iter()
            .filter(|source| meta::core_item_ids(&source.details).contains(id))
            .map(|source| &source.author)
            .collect::<BTreeSet<_>>()
            .len() as i64;
    }
    let pair_rows=pairs.into_iter().map(|((left,right),(matches,wins))| serde_json::json!({
        "item_id":left,"with_item_id":right,"matches":matches,"wins":wins,"losses":matches-wins,"patch_tag":cfg.patch_tag
    })).collect::<Vec<_>>();
    result.combinations = meta::combination_support(&pair_rows, &rows, cfg);
    result
}

impl BuildFamily {
    pub fn reliable_imbues(&self) -> BTreeMap<i64, i64> {
        self.imbues
            .iter()
            .filter(|e| {
                e.observations >= dbrain_population::IMBUE_THIN_OBSERVATIONS as usize
                    && e.share >= dbrain_population::STAPLE_THRESHOLD
            })
            .map(|e| (e.item_id, e.ability_id))
            .collect()
    }
}
