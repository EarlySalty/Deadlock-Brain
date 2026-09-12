use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use serde_json::Value;

use crate::{
    AuthorBuild, CoreLayoutBand, CoreLayoutIndex, CoreLayoutStats, MetaIndex, MetaRow, MetaSupport,
    ReasonerConfig, ReasonerError, Result,
};

pub(crate) const BASE_SLOTS_PER_CATEGORY: usize = 4;
const FALLBACK_BASE_SLOT_CAPACITY: usize = 3 * BASE_SLOTS_PER_CATEGORY;

pub(crate) fn snapshot_flex_slots(snapshot: &Value) -> Option<usize> {
    let slots = snapshot.get("item_slot_info")?;
    ["weapon", "vitality", "spirit"]
        .into_iter()
        .try_fold(0usize, |total, category| {
            let tiers = slots
                .get(category)?
                .get("max_purchases_for_tier")?
                .as_array()?;
            let capacity = usize::try_from(tiers.first()?.as_u64()?).ok()?;
            if capacity < BASE_SLOTS_PER_CATEGORY
                || tiers
                    .iter()
                    .any(|tier| tier.as_u64() != Some(capacity as u64))
            {
                return None;
            }
            total.checked_add(capacity - BASE_SLOTS_PER_CATEGORY)
        })
}

#[derive(Debug, Clone)]
pub struct AuthorBuildSource {
    pub hero_id: i64,
    pub author: String,
    pub weight: f64,
    pub details: Value,
}

#[derive(Debug, Clone)]
pub struct AuthorBuildLayoutSource {
    pub hero_id: i64,
    pub build_id: i64,
    pub version: i64,
    pub details: Value,
}

#[derive(Debug, Clone)]
pub struct MetaIndexWithSources {
    pub index: MetaIndex,
    pub author_builds: Vec<AuthorBuildSource>,
    pub hero_ability_orders: BTreeMap<i64, Vec<crate::AbilityStep>>,
    pub core_layouts: CoreLayoutIndex,
}

impl MetaIndexWithSources {
    pub fn ability_order(&self, hero_id: i64) -> (Vec<crate::AbilityStep>, crate::Evidence) {
        let mut authors = self
            .author_builds
            .iter()
            .filter(|source| source.hero_id == hero_id && source.weight.is_finite())
            .collect::<Vec<_>>();
        authors.sort_by(|left, right| {
            right
                .weight
                .total_cmp(&left.weight)
                .then_with(|| left.author.cmp(&right.author))
                .then_with(|| left.details.to_string().cmp(&right.details.to_string()))
        });
        for source in authors {
            if let Some(order) = author_ability_order(&source.details) {
                return (
                    order,
                    crate::Evidence {
                        kind: crate::EvidenceKind::Author,
                        detail: format!(
                            "Skill-Order: Autoren-Build {} (Gewicht {})",
                            source.author, source.weight
                        ),
                    },
                );
            }
        }
        if let Some(order) = self
            .hero_ability_orders
            .get(&hero_id)
            .filter(|order| !order.is_empty())
        {
            return (
                order.clone(),
                crate::Evidence {
                    kind: crate::EvidenceKind::Meta,
                    detail: "Skill-Order: brain.hero_ability_orders".to_string(),
                },
            );
        }
        (
            Vec::new(),
            crate::Evidence {
                kind: crate::EvidenceKind::Author,
                detail: "Skill-Order: keine Quelle".to_string(),
            },
        )
    }
}

fn category_mod_ids(category: &Value) -> impl Iterator<Item = i64> + '_ {
    category
        .get("mods")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| {
            item.get("abilityId")
                .or_else(|| item.get("ability_id"))
                .and_then(|value| value.as_i64().or_else(|| value.as_str()?.parse().ok()))
                .filter(|id| *id != 0)
        })
}

fn core_categories(details: &Value) -> Vec<&Value> {
    let Some(categories) = details
        .get("modCategories")
        .or_else(|| details.get("mod_categories"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    let named = categories
        .iter()
        .filter(|category| {
            let name = category
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            [
                "core", "kern", "standard", "main", "primary", "basis", "default",
            ]
            .iter()
            .any(|marker| name.contains(marker))
        })
        .collect::<Vec<_>>();
    if !named.is_empty() {
        return named;
    }
    categories
        .iter()
        .max_by_key(|category| category_mod_ids(category).count())
        .into_iter()
        .collect()
}

fn quantile(values: &[usize], fraction: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let position = (sorted.len() - 1) as f64 * fraction;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    sorted[lower] as f64 + (sorted[upper] - sorted[lower]) as f64 * position.fract()
}

fn layout_stats(counts: &[BTreeMap<i64, usize>]) -> CoreLayoutStats {
    if counts.is_empty() {
        return CoreLayoutStats::default();
    }
    let totals = counts
        .iter()
        .map(|counts| counts.values().sum())
        .collect::<Vec<_>>();
    let mut bands = BTreeMap::new();
    for tier in 1..=5 {
        let values = counts
            .iter()
            .map(|counts| counts.get(&tier).copied().unwrap_or(0))
            .collect::<Vec<_>>();
        let median = quantile(&values, 0.5);
        bands.insert(
            tier,
            CoreLayoutBand {
                tier,
                median,
                lower_quartile: quantile(&values, 0.25),
                upper_quartile: quantile(&values, 0.75),
                target: 0,
            },
        );
    }
    let total_median = quantile(&totals, 0.5);
    let total_target = total_median.round() as usize;
    let median_sum: f64 = bands.values().map(|band| band.median).sum();
    let weights = bands
        .values()
        .map(|band| {
            let weight = if median_sum > 0.0 {
                band.median
            } else {
                counts
                    .iter()
                    .map(|counts| counts.get(&band.tier).copied().unwrap_or(0) as f64)
                    .sum()
            };
            (band.tier, weight)
        })
        .collect::<Vec<_>>();
    let weight_sum: f64 = weights.iter().map(|(_, weight)| weight).sum();
    let mut remainders = Vec::new();
    for (tier, weight) in weights {
        let quota = if weight_sum > 0.0 {
            weight / weight_sum * total_target as f64
        } else {
            0.0
        };
        bands.get_mut(&tier).unwrap().target = quota.floor() as usize;
        remainders.push((tier, quota.fract()));
    }
    remainders.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.cmp(&right.0))
    });
    let assigned: usize = bands.values().map(|band| band.target).sum();
    for (tier, _) in remainders
        .into_iter()
        .take(total_target.saturating_sub(assigned))
    {
        bands.get_mut(&tier).unwrap().target += 1;
    }
    CoreLayoutStats {
        source_builds: counts.len(),
        total_median,
        total_lower_quartile: quantile(&totals, 0.25),
        total_upper_quartile: quantile(&totals, 0.75),
        flex_slots: total_target.saturating_sub(FALLBACK_BASE_SLOT_CAPACITY),
        bands,
    }
}

pub fn derive_core_layouts(
    sources: &[AuthorBuildLayoutSource],
    item_tiers: &BTreeMap<i64, i64>,
) -> CoreLayoutIndex {
    let mut by_hero_counts = BTreeMap::<i64, Vec<BTreeMap<i64, usize>>>::new();
    for source in sources {
        let mut counts = BTreeMap::new();
        for category in core_categories(&source.details) {
            for item_id in category_mod_ids(category) {
                if let Some(tier) = item_tiers.get(&item_id) {
                    *counts.entry(*tier).or_insert(0) += 1;
                }
            }
        }
        if !counts.is_empty() {
            by_hero_counts
                .entry(source.hero_id)
                .or_default()
                .push(counts);
        }
    }
    let overall_counts = by_hero_counts
        .values()
        .flat_map(|counts| counts.iter().cloned())
        .collect::<Vec<_>>();
    CoreLayoutIndex {
        by_hero: by_hero_counts
            .into_iter()
            .map(|(hero_id, counts)| (hero_id, layout_stats(&counts)))
            .collect(),
        overall: layout_stats(&overall_counts),
    }
}

fn author_ability_order(details: &Value) -> Option<Vec<crate::AbilityStep>> {
    let order = details
        .get("abilityOrder")
        .or_else(|| details.get("ability_order"))?;
    let changes = order
        .get("currencyChanges")
        .or_else(|| order.get("currency_changes"))
        .unwrap_or(order)
        .as_array()?;
    let steps = changes
        .iter()
        .map(|change| {
            let integer = |camel: &str, snake: &str| {
                let value = change.get(camel).or_else(|| change.get(snake))?;
                value.as_i64().or_else(|| value.as_str()?.parse().ok())
            };
            let ability_id = integer("abilityId", "ability_id")?;
            (ability_id > 0).then(|| {
                Some(crate::AbilityStep {
                    ability_id,
                    currency_type: integer("currencyType", "currency_type")?,
                    delta: integer("delta", "delta")?,
                })
            })?
        })
        .collect::<Option<Vec<_>>>()?;
    (!steps.is_empty()).then_some(steps)
}

fn item_ids(value: &Value, output: &mut BTreeSet<i64>) {
    match value {
        Value::Array(values) => values.iter().for_each(|value| item_ids(value, output)),
        Value::Object(object) => {
            for (key, value) in object {
                if ["item_id", "itemId", "ability_id", "abilityId", "id"].contains(&key.as_str()) {
                    if let Some(id) = value.as_i64().or_else(|| value.as_str()?.parse().ok()) {
                        if id != 0 {
                            output.insert(id);
                        }
                    }
                }
                item_ids(value, output);
            }
        }
        _ => {}
    }
}

fn claim_hits(claims: &[Value], item_id: i64) -> i64 {
    claims
        .iter()
        .filter(|claim| {
            let mut ids = BTreeSet::new();
            item_ids(claim, &mut ids);
            ids.contains(&item_id)
        })
        .count() as i64
}

fn author_hits(authors: &[AuthorBuild], item_id: i64) -> i64 {
    authors
        .iter()
        .filter(|author| {
            author.core_item_ids.contains(&item_id) || author.buy_order.contains(&item_id)
        })
        .count() as i64
}

pub fn build_meta_index(
    rows: &[MetaRow],
    authors: &[AuthorBuild],
    claims: &[Value],
    cfg: &ReasonerConfig,
) -> MetaIndex {
    let mut grouped = BTreeMap::<i64, Vec<&MetaRow>>::new();
    for row in rows {
        if !cfg.patch_tag.is_empty() && !row.patch_tag.is_empty() && row.patch_tag != cfg.patch_tag
        {
            continue;
        }
        grouped.entry(row.item_id).or_default().push(row);
    }
    let max_prevalence = grouped
        .values()
        .flat_map(|values| values.iter().map(|row| row.prevalence_builds))
        .max()
        .unwrap_or(0) as f64;
    let mut by_item = BTreeMap::new();
    let mut sample_ok = BTreeSet::new();
    for (item_id, item_rows) in grouped {
        let prevalence_raw: i64 = item_rows
            .iter()
            .map(|row| row.prevalence_builds.max(0))
            .sum();
        let wins: i64 = item_rows.iter().map(|row| row.wins.max(0)).sum();
        let matches: i64 = item_rows.iter().map(|row| row.matches.max(0)).sum();
        let sample = prevalence_raw >= cfg.min_prevalence_builds && matches >= cfg.min_matches;
        if sample {
            sample_ok.insert(item_id);
        }
        let damp = if sample { 1.0 } else { 0.5 };
        let prevalence = if max_prevalence > 0.0 {
            (prevalence_raw as f64 / max_prevalence).min(1.0)
        } else {
            0.0
        };
        let winrate_pp = (matches > 0).then(|| wins as f64 / matches as f64 * 100.0 * damp);
        let lift_values = item_rows.iter().filter_map(|row| row.lift_pp);
        let lift_count = lift_values.clone().count();
        let lift_sum: f64 = lift_values.sum();
        let lift_pp = (lift_count > 0).then(|| lift_sum / lift_count as f64 * damp);
        let buy_values = item_rows.iter().filter_map(|row| row.avg_buy_time_relative);
        let buy_count = buy_values.clone().count();
        let buy_sum: f64 = buy_values.sum();
        by_item.insert(
            item_id,
            MetaSupport {
                prevalence: prevalence * damp,
                winrate_pp,
                lift_pp,
                author_hits: author_hits(authors, item_id),
                claim_hits: claim_hits(claims, item_id),
                avg_buy_time_relative: (buy_count > 0).then(|| buy_sum / buy_count as f64),
            },
        );
    }
    MetaIndex { by_item, sample_ok }
}

fn text(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase()
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn seed_build(value: &Value, items: &[crate::ItemModel]) -> AuthorBuild {
    let mut core_item_ids = Vec::new();
    let mut buy_order = Vec::new();
    let author = value
        .get("author")
        .and_then(Value::as_str)
        .unwrap_or("Seed-Build")
        .to_string();
    let patch_tag = value
        .get("patch_tag")
        .and_then(Value::as_str)
        .map(str::to_string);
    for block in value
        .get("bloecke")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let block_name = text(block.get("name"));
        for item in block
            .get("items")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let name = item.get("name").and_then(Value::as_str).unwrap_or_default();
            let normalized = normalize(name);
            if let Some(model) = items
                .iter()
                .find(|model| normalize(&model.name) == normalized)
            {
                buy_order.push(model.item_id);
                if block_name.contains("core") {
                    core_item_ids.push(model.item_id);
                }
            }
        }
    }
    AuthorBuild {
        author,
        version: 0,
        published_at: None,
        last_updated_at: None,
        patch_tag,
        core_item_ids,
        buy_order,
    }
}

pub fn load_seed_builds(path: &Path, items: &[crate::ItemModel]) -> Result<Vec<AuthorBuild>> {
    let mut paths = Vec::new();
    if path.is_file() {
        paths.push(path.to_path_buf());
    } else if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|error| ReasonerError::Data(format!("Seed-Verzeichnis: {error}")))?;
        for entry in entries {
            let path = entry
                .map_err(|error| ReasonerError::Data(format!("Seed-Datei: {error}")))?
                .path();
            if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
                paths.push(path);
            }
        }
        paths.sort();
    } else {
        return Err(ReasonerError::Data(format!(
            "Seed-Pfad fehlt: {}",
            path.display()
        )));
    }
    paths
        .into_iter()
        .map(|path| {
            let text = fs::read_to_string(&path).map_err(|error| {
                ReasonerError::Data(format!("Seed-Datei {}: {error}", path.display()))
            })?;
            let value: Value = serde_json::from_str(&text).map_err(|error| {
                ReasonerError::Data(format!("Seed-Datei {}: {error}", path.display()))
            })?;
            Ok(seed_build(&value, items))
        })
        .collect()
}

pub fn build_meta_index_with_seed_path(
    rows: &[MetaRow],
    authors: &[AuthorBuild],
    claims: &[Value],
    cfg: &ReasonerConfig,
    seed_path: &Path,
    items: &[crate::ItemModel],
) -> Result<MetaIndex> {
    let mut all_authors = authors.to_vec();
    all_authors.extend(load_seed_builds(seed_path, items)?);
    Ok(build_meta_index(rows, &all_authors, claims, cfg))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> ReasonerConfig {
        ReasonerConfig {
            min_matches: 100,
            min_prevalence_builds: 5,
            ..ReasonerConfig::default()
        }
    }

    #[test]
    fn dampens_meta_support_below_both_sample_thresholds() {
        let rows = vec![
            MetaRow {
                item_id: 1,
                patch_tag: "current".to_string(),
                prevalence_builds: 100,
                wins: 60,
                losses: 40,
                matches: 100,
                avg_buy_time_relative: Some(0.2),
                lift_pp: Some(4.0),
            },
            MetaRow {
                item_id: 2,
                patch_tag: "current".to_string(),
                prevalence_builds: 2,
                wins: 2,
                losses: 0,
                matches: 2,
                avg_buy_time_relative: None,
                lift_pp: Some(8.0),
            },
        ];
        let index = build_meta_index(&rows, &[], &[], &cfg());
        assert!(index.sample_ok.contains(&1));
        assert!(!index.sample_ok.contains(&2));
        assert_eq!(index.by_item[&1].winrate_pp, Some(60.0));
        assert_eq!(index.by_item[&2].winrate_pp, Some(50.0));
        assert_eq!(index.by_item[&2].lift_pp, Some(4.0));
        assert_eq!(index.by_item[&2].prevalence, 0.01);
    }

    #[test]
    fn caps_grouped_prevalence_before_sample_damping() {
        let row = MetaRow {
            item_id: 42,
            patch_tag: "current".to_string(),
            prevalence_builds: 10,
            wins: 5,
            losses: 5,
            matches: 10,
            avg_buy_time_relative: None,
            lift_pp: None,
        };
        let rows = [row.clone(), row];
        let mut config = cfg();
        config.min_matches = 10;
        assert_eq!(
            build_meta_index(&rows, &[], &[], &config).by_item[&42].prevalence,
            1.0
        );
        config.min_matches = 100;
        assert_eq!(
            build_meta_index(&rows, &[], &[], &config).by_item[&42].prevalence,
            0.5
        );
    }

    #[test]
    fn counts_author_and_claim_hits_once_per_build_or_claim() {
        let rows = vec![MetaRow {
            item_id: 42,
            patch_tag: "current".to_string(),
            prevalence_builds: 10,
            wins: 5,
            losses: 5,
            matches: 10,
            avg_buy_time_relative: None,
            lift_pp: None,
        }];
        let authors = vec![AuthorBuild {
            author: "A".to_string(),
            version: 1,
            published_at: None,
            last_updated_at: None,
            patch_tag: None,
            core_item_ids: vec![42, 42],
            buy_order: vec![42],
        }];
        let claims = vec![serde_json::json!({"item_id": 42, "mentions": [{"item_id": 42}]})];
        let support = &build_meta_index(&rows, &authors, &claims, &cfg()).by_item[&42];
        assert_eq!(support.author_hits, 1);
        assert_eq!(support.claim_hits, 1);
    }

    #[test]
    fn derives_core_layout_from_named_categories_and_falls_back_to_largest() {
        let source = |hero_id, build_id, categories| AuthorBuildLayoutSource {
            hero_id,
            build_id,
            version: 1,
            details: serde_json::json!({"modCategories": categories}),
        };
        let id = |item_id| serde_json::json!({"abilityId": item_id});
        let sources = vec![
            source(
                25,
                1,
                vec![
                    serde_json::json!({"name":"Early", "mods":[id(1),id(2),id(3),id(4)]}),
                    serde_json::json!({"name":"Core Items", "mods":[id(5),id(6)]}),
                    serde_json::json!({"name":"Core Damage", "mods":[id(7)]}),
                ],
            ),
            source(
                25,
                2,
                vec![
                    serde_json::json!({"name":"First", "mods":[id(1),id(2)]}),
                    serde_json::json!({"name":"Options", "mods":[id(3)]}),
                ],
            ),
            source(
                99,
                3,
                vec![serde_json::json!({"name":"Standard", "mods":[id(1),id(2),id(3)]})],
            ),
        ];
        let tiers = BTreeMap::from([(1, 1), (2, 2), (3, 3), (4, 4), (5, 4), (6, 2), (7, 4)]);
        let layouts = derive_core_layouts(&sources, &tiers);
        let warden = layouts.for_hero(25);
        assert_eq!(warden.source_builds, 2);
        assert_eq!(warden.target_for_tier(1), 1);
        assert_eq!(warden.target_for_tier(2), 1);
        assert_eq!(warden.target_for_tier(4), 1);
        assert_eq!(warden.total_target(), 3);
        assert_eq!(layouts.overall.source_builds, 3);
        assert_eq!(layouts.overall.target_for_tier(1), 2);
        assert_eq!(layouts.overall.target_for_tier(2), 1);
    }

    #[test]
    fn apportions_warden_band_medians_to_rounded_total() {
        let counts = [[0, 0, 0, 11], [0, 1, 2, 2], [0, 1, 2, 8], [0, 5, 5, 0]].map(|bands| {
            bands
                .into_iter()
                .enumerate()
                .map(|(i, n)| (i as i64 + 1, n))
                .collect()
        });
        let layout = layout_stats(&counts);
        assert_eq!(layout.total_median, 10.5);
        assert_eq!(
            layout.bands.values().map(|b| b.median).collect::<Vec<_>>(),
            [0.0, 1.0, 2.0, 5.0, 0.0]
        );
        assert_eq!(layout.total_target(), 11);
        assert_eq!(
            (1..=5)
                .map(|tier| layout.target_for_tier(tier))
                .collect::<Vec<_>>(),
            [0, 1, 3, 7, 0]
        );
    }

    #[test]
    fn apportionment_handles_zero_medians_and_totals_below_band_sum() {
        for counts in [
            vec![
                BTreeMap::from([(1, 2)]),
                BTreeMap::from([(2, 2)]),
                BTreeMap::from([(3, 2)]),
            ],
            vec![
                BTreeMap::from([(1, 2), (2, 2)]),
                BTreeMap::from([(2, 2), (3, 2)]),
                BTreeMap::from([(1, 2), (3, 2)]),
            ],
        ] {
            let layout = layout_stats(&counts);
            assert_eq!(layout.total_target(), layout.total_median.round() as usize);
        }
    }

    #[test]
    fn snapshot_capacity_overrides_target_and_requires_unambiguous_tiers() {
        let snapshot = serde_json::json!({"item_slot_info": {
            "weapon": {"max_purchases_for_tier": [6, 6, 6]},
            "vitality": {"max_purchases_for_tier": [6, 6, 6]},
            "spirit": {"max_purchases_for_tier": [6, 6, 6]}
        }});
        assert_eq!(snapshot_flex_slots(&snapshot), Some(6));
        let mut missing = snapshot.clone();
        missing["item_slot_info"]
            .as_object_mut()
            .unwrap()
            .remove("spirit");
        assert_eq!(snapshot_flex_slots(&missing), None);
        let mut gated = snapshot.clone();
        gated["item_slot_info"]["weapon"]["max_purchases_for_tier"] = serde_json::json!([4, 5, 6]);
        assert_eq!(snapshot_flex_slots(&gated), None);
        for invalid in [
            serde_json::json!([]),
            serde_json::json!([-1]),
            serde_json::json!(["six"]),
        ] {
            let mut malformed = snapshot.clone();
            malformed["item_slot_info"]["spirit"]["max_purchases_for_tier"] = invalid;
            assert_eq!(snapshot_flex_slots(&malformed), None);
        }
    }

    #[test]
    fn fallback_flex_uses_corrected_total_capacity() {
        let counts = [[0, 0, 0, 15], [0, 1, 2, 2], [0, 1, 2, 12], [0, 7, 7, 0]].map(|bands| {
            bands
                .into_iter()
                .enumerate()
                .map(|(i, n)| (i as i64 + 1, n))
                .collect()
        });
        let layout = layout_stats(&counts);
        assert_eq!(layout.total_median, 14.5);
        assert_eq!(layout.total_target(), 15);
        assert_eq!(layout.flex_slots, 3);
    }

    #[test]
    fn reports_interquartiles_and_flex_budget_from_target_bands() {
        let source = |build_id, count| AuthorBuildLayoutSource {
            hero_id: 25,
            build_id,
            version: 1,
            details: serde_json::json!({"modCategories":[{"name":"Core","mods":
                (1..=count).map(|item_id| serde_json::json!({"abilityId":item_id})).collect::<Vec<_>>()
            }]}),
        };
        let layouts = derive_core_layouts(
            &[source(1, 1), source(2, 3), source(3, 5)],
            &BTreeMap::from([(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)]),
        );
        let band = &layouts.overall.bands[&1];
        assert_eq!(band.median, 3.0);
        assert_eq!(band.lower_quartile, 2.0);
        assert_eq!(band.upper_quartile, 4.0);
        assert_eq!(layouts.overall.total_median, 3.0);
        assert_eq!(layouts.overall.flex_slots, 0);
    }
}
