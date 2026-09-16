use std::collections::{HashMap, HashSet};

use anyhow::Result;
use sqlx::postgres::PgPool;

use crate::catalog::Catalog;
use crate::clean::neigung_bucket;
use crate::db::AggRow;
use crate::{
    ABILITY_ORDER_PREFIX, BADGE_CENTER, BADGE_HALFWIDTH, BUCKET_ALL, IMBUE_SPLIT_THRESHOLD,
    IMBUE_THIN_OBSERVATIONS, NEIGUNG_MIN_SHARE, STAPLE_THRESHOLD, WIN_WEIGHT,
};

pub struct ItemStat {
    pub item_id: i64,
    pub buyers: i32,
    pub prevalence_raw: f64,
    pub prevalence_weighted: f64,
    pub median_position: f64,
    pub median_buy_time_s: f64,
    pub sell_rate: f64,
    pub is_staple: bool,
    pub next_item_id: Option<i64>,
    pub next_item_share: f64,
}

pub struct ImbueStat {
    pub item_id: i64,
    pub target_ability_id: i64,
    pub target_count: i32,
    pub total_imbues: i32,
    pub is_split: bool,
    pub is_thin: bool,
}

pub struct AbilityPosition {
    pub position: i16,
    pub ability_id: i64,
    pub followers: i32,
}

pub struct BucketAgg {
    pub bucket: String,
    pub players_raw: i32,
    pub players_weighted: f64,
    pub items: Vec<ItemStat>,
    pub imbue: Vec<ImbueStat>,
    pub ability: Vec<AbilityPosition>,
    pub ability_followers: i32,
    pub ability_players: i32,
}

pub async fn rebuild_hero(
    pool: &PgPool,
    hero_id: i64,
    rows: &[AggRow],
    catalog: &Catalog,
) -> Result<Vec<BucketAgg>> {
    let buckets = compute_hero(rows, catalog);
    write_hero(pool, hero_id, &buckets).await?;
    Ok(buckets)
}

pub fn compute_hero(rows: &[AggRow], catalog: &Catalog) -> Vec<BucketAgg> {
    if rows.is_empty() {
        return Vec::new();
    }
    let neigungen: Vec<Option<String>> = rows
        .iter()
        .map(|row| neigung_bucket(&row.items, catalog))
        .collect();

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for neigung in neigungen.iter().flatten() {
        *counts.entry(neigung.as_str()).or_insert(0) += 1;
    }
    let total = rows.len() as f64;

    let mut buckets = Vec::new();
    let all_rows: Vec<&AggRow> = rows.iter().collect();
    buckets.push(compute_bucket(BUCKET_ALL, &all_rows, catalog));

    for slot in ["weapon", "spirit", "vitality"] {
        let share = *counts.get(slot).unwrap_or(&0) as f64 / total;
        if share < NEIGUNG_MIN_SHARE {
            continue;
        }
        let subset: Vec<&AggRow> = rows
            .iter()
            .zip(neigungen.iter())
            .filter(|(_, neigung)| neigung.as_deref() == Some(slot))
            .map(|(row, _)| row)
            .collect();
        if subset.is_empty() {
            continue;
        }
        buckets.push(compute_bucket(slot, &subset, catalog));
    }
    buckets
}

fn row_weight(row: &AggRow) -> f64 {
    let badge_weight = match row.average_badge {
        Some(badge) => {
            let delta = (badge as f64 - BADGE_CENTER) / BADGE_HALFWIDTH;
            (-(delta * delta)).exp()
        }
        None => 1.0,
    };
    badge_weight * if row.won { WIN_WEIGHT } else { 1.0 }
}

fn compute_bucket(bucket: &str, rows: &[&AggRow], catalog: &Catalog) -> BucketAgg {
    let n = rows.len();
    let total_weight: f64 = rows.iter().map(|row| row_weight(row)).sum();

    let mut buyers: HashMap<i64, Vec<usize>> = HashMap::new();
    for (row_index, row) in rows.iter().enumerate() {
        let mut seen: HashSet<i64> = HashSet::new();
        for item in &row.items {
            if seen.insert(*item) {
                buyers.entry(*item).or_default().push(row_index);
            }
        }
    }

    let mut items = Vec::new();
    for (&item_id, buyer_rows) in &buyers {
        let buyers_count = buyer_rows.len();
        let mut positions = Vec::with_capacity(buyers_count);
        let mut buy_times = Vec::with_capacity(buyers_count);
        let mut sold = 0usize;
        let mut weighted_buyers = 0.0f64;
        let mut next_counts: HashMap<i64, usize> = HashMap::new();
        let mut next_total = 0usize;
        for &row_index in buyer_rows {
            let row = rows[row_index];
            if let Some(position) = row.items.iter().position(|value| *value == item_id) {
                positions.push(position as f64);
                if let Some(time) = row.buy_times_s.get(position) {
                    buy_times.push(*time as f64);
                }
                if matches!(row.sold_times_s.get(position), Some(Some(_))) {
                    sold += 1;
                }
                if let Some(next) = row.items.get(position + 1) {
                    *next_counts.entry(*next).or_insert(0) += 1;
                    next_total += 1;
                }
            }
            weighted_buyers += row_weight(row);
        }
        let prevalence_raw = buyers_count as f64 / n as f64;
        let prevalence_weighted = if total_weight > 0.0 {
            weighted_buyers / total_weight
        } else {
            0.0
        };
        let (next_item_id, next_item_share) = next_counts
            .iter()
            .max_by_key(|(_, count)| **count)
            .map(|(next, count)| (Some(*next), *count as f64 / next_total as f64))
            .unwrap_or((None, 0.0));
        items.push(ItemStat {
            item_id,
            buyers: buyers_count as i32,
            prevalence_raw,
            prevalence_weighted,
            median_position: median(&mut positions),
            median_buy_time_s: median(&mut buy_times),
            sell_rate: sold as f64 / buyers_count as f64,
            is_staple: prevalence_raw >= STAPLE_THRESHOLD,
            next_item_id,
            next_item_share,
        });
    }
    items.sort_by(|a, b| {
        b.is_staple
            .cmp(&a.is_staple)
            .then(b.prevalence_raw.total_cmp(&a.prevalence_raw))
            .then(a.item_id.cmp(&b.item_id))
    });

    let imbue = compute_imbue(rows, catalog);
    let (ability, ability_followers, ability_players) = compute_ability_order(rows);

    BucketAgg {
        bucket: bucket.to_string(),
        players_raw: n as i32,
        players_weighted: total_weight,
        items,
        imbue,
        ability,
        ability_followers,
        ability_players,
    }
}

fn compute_imbue(rows: &[&AggRow], catalog: &Catalog) -> Vec<ImbueStat> {
    let mut targets: HashMap<i64, HashMap<i64, usize>> = HashMap::new();
    for row in rows {
        for (index, item) in row.items.iter().enumerate() {
            if !catalog.imbueable(*item) {
                continue;
            }
            let Some(target) = row.imbue_targets.get(index).copied() else {
                continue;
            };
            if target == 0 {
                continue;
            }
            *targets.entry(*item).or_default().entry(target).or_insert(0) += 1;
        }
    }
    let mut out = Vec::new();
    for (item_id, distribution) in targets {
        let total: usize = distribution.values().sum();
        if total == 0 {
            continue;
        }
        let (target, count) = distribution
            .iter()
            .max_by_key(|(_, count)| **count)
            .map(|(target, count)| (*target, *count))
            .unwrap();
        out.push(ImbueStat {
            item_id,
            target_ability_id: target,
            target_count: count as i32,
            total_imbues: total as i32,
            is_split: (count as f64 / total as f64) < IMBUE_SPLIT_THRESHOLD,
            is_thin: (total as i64) < IMBUE_THIN_OBSERVATIONS,
        });
    }
    out.sort_by(|a, b| {
        b.total_imbues
            .cmp(&a.total_imbues)
            .then(a.item_id.cmp(&b.item_id))
    });
    out
}

fn compute_ability_order(rows: &[&AggRow]) -> (Vec<AbilityPosition>, i32, i32) {
    let mut lengths: Vec<usize> = rows.iter().map(|row| row.ability_points.len()).collect();
    if lengths.is_empty() {
        return (Vec::new(), 0, 0);
    }
    lengths.sort_unstable();
    let p50 = lengths[lengths.len() / 2];
    let order_len = p50.min(ABILITY_ORDER_PREFIX);
    if order_len == 0 {
        return (Vec::new(), 0, 0);
    }

    let mut modal = Vec::with_capacity(order_len);
    let mut positions = Vec::with_capacity(order_len);
    for position in 0..order_len {
        let mut counts: HashMap<i64, usize> = HashMap::new();
        for row in rows {
            if let Some(ability) = row.ability_points.get(position) {
                *counts.entry(*ability).or_insert(0) += 1;
            }
        }
        let Some((ability, _)) = counts
            .into_iter()
            .max_by_key(|(ability, count)| (*count, *ability))
        else {
            break;
        };
        modal.push(ability);
        positions.push(AbilityPosition {
            position: position as i16,
            ability_id: ability,
            followers: 0,
        });
    }

    let candidates: Vec<&&AggRow> = rows
        .iter()
        .filter(|row| row.ability_points.len() >= modal.len())
        .collect();
    let followers = candidates
        .iter()
        .filter(|row| row.ability_points[..modal.len()] == modal[..])
        .count();
    for entry in &mut positions {
        entry.followers = followers as i32;
    }
    (positions, followers as i32, candidates.len() as i32)
}

fn median(values: &mut [f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let mid = values.len() / 2;
    if values.len() % 2 == 1 {
        values[mid]
    } else {
        (values[mid - 1] + values[mid]) / 2.0
    }
}

async fn write_hero(pool: &PgPool, hero_id: i64, buckets: &[BucketAgg]) -> Result<()> {
    let mut tx = pool.begin().await?;
    for table in [
        "population_hero_buckets",
        "population_item_stats",
        "population_imbue_stats",
        "population_ability_order",
    ] {
        sqlx::query(&format!("DELETE FROM brain.{table} WHERE hero_id = $1"))
            .bind(hero_id)
            .execute(&mut *tx)
            .await?;
    }
    for bucket in buckets {
        sqlx::query(
            r#"
            INSERT INTO brain.population_hero_buckets(
              hero_id, bucket, players_raw, players_weighted,
              ability_order_followers, ability_order_players, updated_at
            )
            VALUES($1,$2,$3,$4,$5,$6, now())
            "#,
        )
        .bind(hero_id)
        .bind(&bucket.bucket)
        .bind(bucket.players_raw)
        .bind(bucket.players_weighted)
        .bind(bucket.ability_followers)
        .bind(bucket.ability_players)
        .execute(&mut *tx)
        .await?;

        for item in &bucket.items {
            sqlx::query(
                r#"
                INSERT INTO brain.population_item_stats(
                  hero_id, bucket, item_id, buyers, prevalence_raw, prevalence_weighted,
                  median_position, median_buy_time_s, sell_rate, is_staple,
                  next_item_id, next_item_share, updated_at
                )
                VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12, now())
                "#,
            )
            .bind(hero_id)
            .bind(&bucket.bucket)
            .bind(item.item_id)
            .bind(item.buyers)
            .bind(item.prevalence_raw)
            .bind(item.prevalence_weighted)
            .bind(item.median_position)
            .bind(item.median_buy_time_s)
            .bind(item.sell_rate)
            .bind(item.is_staple)
            .bind(item.next_item_id)
            .bind(item.next_item_share)
            .execute(&mut *tx)
            .await?;
        }

        for imbue in &bucket.imbue {
            sqlx::query(
                r#"
                INSERT INTO brain.population_imbue_stats(
                  hero_id, bucket, item_id, target_ability_id, target_count,
                  total_imbues, is_split, is_thin, updated_at
                )
                VALUES($1,$2,$3,$4,$5,$6,$7,$8, now())
                "#,
            )
            .bind(hero_id)
            .bind(&bucket.bucket)
            .bind(imbue.item_id)
            .bind(imbue.target_ability_id)
            .bind(imbue.target_count)
            .bind(imbue.total_imbues)
            .bind(imbue.is_split)
            .bind(imbue.is_thin)
            .execute(&mut *tx)
            .await?;
        }

        for entry in &bucket.ability {
            sqlx::query(
                r#"
                INSERT INTO brain.population_ability_order(
                  hero_id, bucket, position, ability_id, followers, updated_at
                )
                VALUES($1,$2,$3,$4,$5, now())
                "#,
            )
            .bind(hero_id)
            .bind(&bucket.bucket)
            .bind(entry.position)
            .bind(entry.ability_id)
            .bind(entry.followers)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::AggRow;
    use serde_json::json;

    fn catalog() -> Catalog {
        let items = json!([
            {"id": 100, "type": "upgrade", "name": "W1", "item_slot_type": "weapon", "item_tier": 1, "cost": 800},
            {"id": 101, "type": "upgrade", "name": "W2", "item_slot_type": "weapon", "item_tier": 2, "cost": 1600},
            {"id": 200, "type": "upgrade", "name": "S1", "item_slot_type": "spirit", "item_tier": 2, "cost": 1600, "imbue": "imbue_active"}
        ]);
        Catalog::from_payloads(&items, &json!([])).unwrap()
    }

    fn row(
        items: Vec<i64>,
        targets: Vec<i64>,
        abilities: Vec<i64>,
        badge: i32,
        won: bool,
    ) -> AggRow {
        let buy_times_s = items
            .iter()
            .enumerate()
            .map(|(i, _)| (i as i32 + 1) * 100)
            .collect();
        let sold_times_s = items.iter().map(|_| None).collect();
        AggRow {
            items,
            buy_times_s,
            sold_times_s,
            imbue_targets: targets,
            ability_points: abilities,
            average_badge: Some(badge),
            won,
        }
    }

    #[test]
    fn staple_flag_triggers_at_seventy_percent() {
        let rows: Vec<AggRow> = (0..10)
            .map(|i| {
                let items = if i < 8 { vec![100, 200] } else { vec![101] };
                let targets = vec![0; items.len()];
                row(items, targets, vec![900], 80, i % 2 == 0)
            })
            .collect();
        let buckets = compute_hero(&rows, &catalog());
        let all = buckets.iter().find(|b| b.bucket == "all").unwrap();
        let staple = all.items.iter().find(|s| s.item_id == 100).unwrap();
        assert!(staple.is_staple);
        let rare = all.items.iter().find(|s| s.item_id == 101).unwrap();
        assert!(!rare.is_staple);
    }

    #[test]
    fn imbue_mode_and_flags() {
        let mut rows = Vec::new();
        for _ in 0..4 {
            rows.push(row(vec![200], vec![777], vec![900], 80, true));
        }
        for _ in 0..2 {
            rows.push(row(vec![200], vec![555], vec![900], 80, true));
        }
        let buckets = compute_hero(&rows, &catalog());
        let all = buckets.iter().find(|b| b.bucket == "all").unwrap();
        let imbue = all.imbue.iter().find(|i| i.item_id == 200).unwrap();
        assert_eq!(imbue.target_ability_id, 777);
        assert_eq!(imbue.target_count, 4);
        assert_eq!(imbue.total_imbues, 6);
        assert!(!imbue.is_split);
        assert!(imbue.is_thin);
    }

    #[test]
    fn ability_order_mode_and_followers() {
        let mut rows = Vec::new();
        for _ in 0..7 {
            rows.push(row(vec![100], vec![0], vec![900, 901, 900, 901], 80, true));
        }
        for _ in 0..3 {
            rows.push(row(vec![100], vec![0], vec![901, 900, 901, 900], 80, true));
        }
        let buckets = compute_hero(&rows, &catalog());
        let all = buckets.iter().find(|b| b.bucket == "all").unwrap();
        assert_eq!(all.ability.first().unwrap().ability_id, 900);
        assert_eq!(all.ability_followers, 7);
        assert_eq!(all.ability_players, 10);
    }

    #[test]
    fn winners_and_high_badge_weigh_more_than_raw() {
        let rows = vec![
            row(vec![100], vec![0], vec![900], 80, true),
            row(vec![100], vec![0], vec![900], 20, false),
        ];
        let buckets = compute_hero(&rows, &catalog());
        let all = buckets.iter().find(|b| b.bucket == "all").unwrap();
        let stat = all.items.iter().find(|s| s.item_id == 100).unwrap();
        assert!((stat.prevalence_raw - 1.0).abs() < 1e-9);
        assert!((stat.prevalence_weighted - 1.0).abs() < 1e-9);
        assert!(all.players_weighted < 2.0);
    }

    #[test]
    fn duplicate_item_in_a_row_counts_the_player_once() {
        let rows = vec![row(vec![100, 100], vec![0, 0], vec![900], 80, true)];
        let buckets = compute_hero(&rows, &catalog());
        let all = buckets.iter().find(|b| b.bucket == "all").unwrap();
        let stat = all.items.iter().find(|s| s.item_id == 100).unwrap();
        assert_eq!(stat.buyers, 1);
        assert!(stat.prevalence_raw <= 1.0);
        assert!((stat.prevalence_raw - 1.0).abs() < 1e-9);
    }
}
