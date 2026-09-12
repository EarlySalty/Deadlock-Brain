use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use serde_json::Value;

use crate::{AuthorBuild, MetaIndex, MetaRow, MetaSupport, ReasonerConfig, ReasonerError, Result};

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
            prevalence_raw as f64 / max_prevalence
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
}
