//! Deterministic, data-derived playstyles in front of the existing planner.
//! Clustering features describe mechanics; they never invent numerical scores.
use crate::{AbilityStep, ItemModel, PopulationPrior};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
mod canonical;
mod conditioned;
mod data;
mod evidence;
mod features;
mod holdout;
pub use conditioned::conditioned_meta;
pub use holdout::{assign_holdout, is_holdout, observed_population, HeldOutAssignment};
#[cfg(test)]
mod tests;
pub use data::{author_observation, load_family_author_sources, load_player_observations};
pub use features::item_axes;
use features::{centroid, core_centroid, features, merge_similarity, similarity, upgrade_roots};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MechanicAxis {
    Weapon,
    Spirit,
    Melee,
    Support,
    Defense,
    Mobility,
    Sustain,
    Control,
    Cooldown,
    Duration,
    Charges,
    Range,
    Active,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationSource {
    Player,
    Author,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildObservation {
    pub key: String,
    pub participant: String,
    pub source: ObservationSource,
    pub items: Vec<i64>,
    pub imbues: BTreeMap<i64, i64>,
    pub skill_order: Vec<AbilityStep>,
    pub buy_times_s: BTreeMap<i64, i64>,
    pub sold_times_s: BTreeMap<i64, i64>,
    pub observed_at: Option<i64>,
    pub won: Option<bool>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FamilyPolicy {
    pub min_matches: usize,
    pub min_players: usize,
    pub min_authors: usize,
    pub min_share: f64,
    pub join_similarity: f64,
    pub merge_similarity: f64,
    pub max_prototypes: usize,
    #[serde(default)]
    pub patch_started_at: Option<i64>,
}
impl Default for FamilyPolicy {
    fn default() -> Self {
        Self {
            min_matches: 100,
            min_players: 20,
            min_authors: 3,
            min_share: 0.03,
            join_similarity: 0.55,
            merge_similarity: 0.68,
            max_prototypes: 64,
            patch_started_at: None,
        }
    }
}
impl FamilyPolicy {
    pub fn for_patch(events: &[serde_json::Value], cfg: &crate::ReasonerConfig) -> Self {
        let patch_started_at = events
            .iter()
            .filter(|event| {
                ["patch_external_id", "patch_url"]
                    .iter()
                    .any(|key| event[*key].as_str() == Some(cfg.patch_tag.as_str()))
            })
            .filter_map(|event| event["posted_at_epoch"].as_f64())
            .filter(|value| value.is_finite() && *value > 0.0 && *value < i64::MAX as f64)
            .map(|value| value as i64)
            .min();
        Self {
            patch_started_at,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FamilyItem {
    pub item_id: i64,
    pub observations: usize,
    pub prevalence: f64,
    pub global_prevalence: Option<f64>,
    pub median_position: Option<f64>,
    pub median_buy_time_s: Option<f64>,
    pub sell_rate: Option<f64>,
    pub staple: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImbueEvidence {
    pub item_id: i64,
    pub ability_id: i64,
    pub observations: usize,
    pub share: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairEvidence {
    pub left: i64,
    pub right: i64,
    pub observations: usize,
    pub prevalence: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildFamily {
    pub id: String,
    pub label: String,
    pub player_matches: usize,
    #[serde(default)]
    pub patch_started_at: Option<i64>,
    #[serde(default)]
    pub post_patch_player_matches: Option<usize>,
    pub distinct_players: usize,
    pub source_builds: usize,
    pub distinct_authors: usize,
    pub population_share: Option<f64>,
    pub cohesion: f64,
    pub axes: BTreeMap<MechanicAxis, f64>,
    pub items: Vec<FamilyItem>,
    pub pairs: Vec<PairEvidence>,
    pub skill_order: Vec<AbilityStep>,
    pub skill_order_support: Option<f64>,
    pub imbues: Vec<ImbueEvidence>,
    pub first_observed_at: Option<i64>,
    pub last_observed_at: Option<i64>,
    /// Statistical support admits planning, never authorizes publication.
    pub eligible_for_planning: bool,
    pub limitations: Vec<String>,
    #[serde(skip)]
    pub member_keys: BTreeSet<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FamilyDiscovery {
    pub policy: FamilyPolicy,
    pub input_observations: usize,
    pub duplicate_observations: usize,
    /// Unique identities excluded because imported evidence disagrees.
    #[serde(default)]
    pub conflicting_observations: usize,
    pub historical_observations_excluded: usize,
    pub unassigned_observations: usize,
    pub families: Vec<BuildFamily>,
    pub limitations: Vec<String>,
}

pub fn detect_families(
    observations: &[BuildObservation],
    items: &[ItemModel],
    global: &PopulationPrior,
    policy: &FamilyPolicy,
) -> FamilyDiscovery {
    let catalog: BTreeMap<_, _> = items
        .iter()
        .filter(|i| i.shopable && !i.disabled)
        .map(|i| (i.item_id, i))
        .collect();
    let roots = upgrade_roots(items);
    let canonical = canonical::observations(observations);
    let duplicate_observations = canonical.duplicate_rows;
    let conflicting_observations = canonical.conflicting_keys.len();
    let mut input = canonical.rows;
    let before_patch_filter = input.len();
    // Freshness support is counted after identity deduplication. A repeated
    // import of one new match cannot discard a well-supported older cohort.
    if let Some(start) = policy.patch_started_at {
        let current = input
            .iter()
            .filter(|row| {
                row.source == ObservationSource::Player
                    && row.observed_at.is_some_and(|time| time >= start)
            })
            .count();
        if current >= policy.min_matches {
            input.retain(|row| {
                row.source == ObservationSource::Author
                    || row.observed_at.is_some_and(|time| time >= start)
            });
        }
    }
    let historical_observations_excluded = before_patch_filter - input.len();
    let values: Vec<_> = input
        .iter()
        .map(|o| features(o, &catalog, &roots))
        .collect();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut centers = Vec::new();
    let mut unassigned = 0;
    for (i, f) in values.iter().enumerate() {
        if f.roots.len() < 3 {
            unassigned += 1;
            continue;
        }
        let nearest = centers
            .iter()
            .enumerate()
            .map(|(j, c)| (j, similarity(f, c)))
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)));
        match nearest.filter(|(_, s)| *s >= policy.join_similarity) {
            Some((j, _)) => {
                groups[j].push(i);
                if groups[j].len().is_power_of_two() {
                    centers[j] = centroid(&groups[j], &values);
                }
            }
            None if groups.len() < policy.max_prototypes => {
                groups.push(vec![i]);
                centers.push(f.clone());
            }
            None => unassigned += 1,
        }
    }
    centers = groups.iter().map(|g| centroid(g, &values)).collect();
    // Online prototypes are only a bounded initialization. Reassign against the
    // complete cohort centroids so the first long match cannot own a family and
    // a row rejected before a centroid matured can still find its real cohort.
    for _ in 0..3 {
        let mut reassigned = vec![Vec::new(); centers.len()];
        unassigned = 0;
        for (i, f) in values.iter().enumerate() {
            let nearest = centers
                .iter()
                .enumerate()
                .map(|(j, c)| (j, similarity(f, c)))
                .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)));
            match nearest.filter(|(_, s)| f.roots.len() >= 3 && *s >= policy.join_similarity) {
                Some((j, _)) => reassigned[j].push(i),
                None => unassigned += 1,
            }
        }
        reassigned.retain(|group| !group.is_empty());
        if reassigned == groups {
            break;
        }
        groups = reassigned;
        centers = groups.iter().map(|g| centroid(g, &values)).collect();
    }
    let mut core_centers = groups
        .iter()
        .map(|g| core_centroid(g, &values))
        .collect::<Vec<_>>();
    loop {
        let mut best: Option<(usize, usize, f64)> = None;
        for a in 0..groups.len() {
            for b in a + 1..groups.len() {
                let score =
                    merge_similarity(&centers[a], &centers[b], &core_centers[a], &core_centers[b]);
                if score >= policy.merge_similarity
                    && best.as_ref().is_none_or(|(_, _, s)| score > *s)
                {
                    best = Some((a, b, score));
                }
            }
        }
        let Some((a, b, _)) = best else {
            break;
        };
        let moved = groups.remove(b);
        groups[a].extend(moved);
        groups[a].sort_unstable();
        centers.remove(b);
        centers[a] = centroid(&groups[a], &values);
        core_centers.remove(b);
        core_centers[a] = core_centroid(&groups[a], &values);
    }
    let total_players = input
        .iter()
        .filter(|o| o.source == ObservationSource::Player)
        .count();
    let mut families = groups.iter().zip(&centers).map(|(members,center)| {
        let mut family = evidence::summarize(members,&input,center,global,total_players);
        family.patch_started_at = policy.patch_started_at;
        family.post_patch_player_matches = policy.patch_started_at.map(|start| members.iter().filter(|i| input[**i].source == ObservationSource::Player && input[**i].observed_at.is_some_and(|time| time >= start)).count());
        match family.post_patch_player_matches {
            Some(count) if count < policy.min_matches => family.limitations.push(format!("Nur {count} belegte Spieler-Matches nach dem Patch; Familienhäufigkeiten sind überwiegend historische Priors, kein aktueller Patch-Beweis.")),
            None => family.limitations.push("Patchzeitpunkt nicht belegt; Aktualität der Spieler-Matches bleibt unbekannt.".into()),
            _ => {},
        }
        family.cohesion = members.iter().map(|i| similarity(&values[*i],center)).sum::<f64>() / members.len() as f64;
        // Source-count evidence must not masquerade as a statistically supported
        // player cohort. An author-only fallback is possible only when no player
        // population exists for this hero at all.
        let supported = if total_players > 0 {
            family.player_matches >= policy.min_matches && family.distinct_players >= policy.min_players
        } else {
            family.distinct_authors >= policy.min_authors
        };
        let large_enough = family.population_share.is_none_or(|share| share >= policy.min_share);
        family.eligible_for_planning = supported && large_enough && family.cohesion >= 0.40;
        if !supported { family.limitations.push("Stichprobe oder Zahl unabhängiger Quellen reicht nicht für einen veröffentlichten Playstyle.".into()); }
        if !large_enough { family.limitations.push("Zu geringer Populationsanteil ohne unabhängigen Autorenbeleg.".into()); }
        if family.cohesion < 0.40 { family.limitations.push("Interne Item-/Mechanikkohärenz zu gering.".into()); }
        family
    }).collect::<Vec<_>>();
    families.sort_by(|a, b| {
        b.player_matches
            .cmp(&a.player_matches)
            .then_with(|| b.distinct_authors.cmp(&a.distinct_authors))
            .then_with(|| a.id.cmp(&b.id))
    });
    let mut limitations = vec!["Begrenztes deterministisches Prototyp-Clustering; keine vorgegebene Anzahl von Playstyles. Namenssignale haben kein numerisches Gewicht.".into(),
        "Quellenkategorien kennzeichnen mögliche Kernkäufe, beweisen aber allein keine Familie. Populationsanteile beziehen sich auf den geladenen Datenzeitraum, nicht alle Deadlock-Spieler.".into()];
    if conflicting_observations > 0 {
        limitations.push(format!("{conflicting_observations} widersprüchlich importierte Beobachtungsidentitäten ausgeschlossen; ohne belegte Quellversion wird kein Payload bevorzugt."));
    }
    FamilyDiscovery {
        policy: policy.clone(),
        input_observations: input.len(),
        duplicate_observations,
        conflicting_observations,
        historical_observations_excluded,
        unassigned_observations: unassigned,
        families,
        limitations,
    }
}

pub(super) fn order_key(order: &[AbilityStep]) -> Vec<(i64, i64, i64)> {
    order
        .iter()
        .map(|s| (s.ability_id, s.currency_type, s.delta))
        .collect()
}

/// Stable non-cryptographic identifier. Only numeric mechanics enter this hash.
pub(super) fn identity(bytes: impl IntoIterator<Item = u8>) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for b in bytes {
        hash = (hash ^ u64::from(b)).wrapping_mul(0x100000001b3);
    }
    format!("family-{hash:016x}")
}
