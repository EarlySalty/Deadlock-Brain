use std::collections::HashMap;

use anyhow::Result;
use sqlx::postgres::PgPool;
use sqlx::Row;

use crate::BUCKET_ALL;

#[derive(Debug, Clone)]
pub struct ItemStat {
    pub item_id: i64,
    pub prevalence_raw: f64,
    pub prevalence_weighted: f64,
    pub median_position: f64,
    pub median_buy_time_s: f64,
    pub sell_rate: f64,
    pub is_staple: bool,
    pub next_item_id: Option<i64>,
    pub next_item_share: f64,
}

#[derive(Debug, Clone, Default)]
struct BucketData {
    items: Vec<ItemStat>,
    ability_order: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct PopulationIndex {
    pub hero_id: i64,
    buckets: HashMap<String, BucketData>,
    imbue: HashMap<i64, i64>,
}

impl PopulationIndex {
    pub async fn load(pool: &PgPool, hero_id: i64) -> Result<Self> {
        let mut buckets: HashMap<String, BucketData> = HashMap::new();

        let item_rows = sqlx::query(
            r#"
            SELECT bucket, item_id, prevalence_raw, prevalence_weighted, median_position,
                   median_buy_time_s, sell_rate, is_staple, next_item_id, next_item_share
            FROM brain.population_item_stats
            WHERE hero_id = $1
            "#,
        )
        .bind(hero_id)
        .fetch_all(pool)
        .await?;
        for row in item_rows {
            let bucket: String = row.get("bucket");
            buckets.entry(bucket).or_default().items.push(ItemStat {
                item_id: row.get("item_id"),
                prevalence_raw: row.get("prevalence_raw"),
                prevalence_weighted: row.get("prevalence_weighted"),
                median_position: row.get("median_position"),
                median_buy_time_s: row.get("median_buy_time_s"),
                sell_rate: row.get("sell_rate"),
                is_staple: row.get("is_staple"),
                next_item_id: row.get("next_item_id"),
                next_item_share: row.get("next_item_share"),
            });
        }

        let ability_rows = sqlx::query(
            r#"
            SELECT bucket, position, ability_id
            FROM brain.population_ability_order
            WHERE hero_id = $1
            ORDER BY bucket, position
            "#,
        )
        .bind(hero_id)
        .fetch_all(pool)
        .await?;
        for row in ability_rows {
            let bucket: String = row.get("bucket");
            buckets
                .entry(bucket)
                .or_default()
                .ability_order
                .push(row.get("ability_id"));
        }

        let imbue_rows = sqlx::query(
            r#"
            SELECT item_id, target_ability_id
            FROM brain.population_imbue_stats
            WHERE hero_id = $1 AND bucket = $2
            "#,
        )
        .bind(hero_id)
        .bind(BUCKET_ALL)
        .fetch_all(pool)
        .await?;
        let imbue = imbue_rows
            .into_iter()
            .map(|row| {
                (
                    row.get::<i64, _>("item_id"),
                    row.get::<i64, _>("target_ability_id"),
                )
            })
            .collect();

        Ok(Self {
            hero_id,
            buckets,
            imbue,
        })
    }

    fn bucket(&self, bucket: &str) -> Option<&BucketData> {
        self.buckets.get(bucket)
    }

    pub fn buckets(&self) -> Vec<String> {
        let mut names: Vec<String> = self.buckets.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn staples(&self, bucket: &str) -> Vec<i64> {
        let Some(data) = self.bucket(bucket) else {
            return Vec::new();
        };
        let mut staples: Vec<&ItemStat> = data.items.iter().filter(|item| item.is_staple).collect();
        staples.sort_by(|a, b| b.prevalence_raw.total_cmp(&a.prevalence_raw));
        staples.into_iter().map(|item| item.item_id).collect()
    }

    pub fn item(&self, item_id: i64) -> Option<&ItemStat> {
        self.bucket(BUCKET_ALL)?
            .items
            .iter()
            .find(|item| item.item_id == item_id)
    }

    pub fn prevalence(&self, item_id: i64) -> f64 {
        self.item(item_id)
            .map(|item| item.prevalence_weighted)
            .unwrap_or(0.0)
    }

    pub fn median_position(&self, item_id: i64) -> Option<f64> {
        self.item(item_id).map(|item| item.median_position)
    }

    pub fn imbue_target(&self, item_id: i64) -> Option<i64> {
        self.imbue.get(&item_id).copied()
    }

    pub fn ability_order(&self) -> Vec<i64> {
        self.bucket(BUCKET_ALL)
            .map(|data| data.ability_order.clone())
            .unwrap_or_default()
    }

    pub fn next_after(&self, item_id: i64) -> Option<i64> {
        self.item(item_id).and_then(|item| item.next_item_id)
    }

    pub fn population_positions(&self, bucket: &str) -> Vec<(i64, f64)> {
        self.bucket(bucket)
            .map(|data| {
                data.items
                    .iter()
                    .map(|item| (item.item_id, item.median_position))
                    .collect()
            })
            .unwrap_or_default()
    }
}
