use sqlx::{postgres::PgPool, Row};

pub const STAPLE_THRESHOLD: f64 = 0.70;
pub const IMBUE_THIN_OBSERVATIONS: i64 = 30;
const BUCKET_ALL: &str = "all";

struct ItemStat {
    item_id: i64,
    prevalence_raw: f64,
    prevalence_weighted: f64,
    median_position: f64,
    is_staple: bool,
}

pub(crate) struct PopulationIndex {
    items: Vec<ItemStat>,
}

impl PopulationIndex {
    pub(crate) async fn load(pool: &PgPool, hero_id: i64) -> sqlx::Result<Self> {
        let rows = sqlx::query(
            "SELECT item_id, prevalence_raw, prevalence_weighted, median_position, is_staple \
             FROM brain.population_item_stats WHERE hero_id = $1 AND bucket = $2",
        )
        .bind(hero_id)
        .bind(BUCKET_ALL)
        .fetch_all(pool)
        .await?;
        let items = rows
            .iter()
            .map(|row| {
                Ok(ItemStat {
                    item_id: row.try_get("item_id")?,
                    prevalence_raw: row.try_get("prevalence_raw")?,
                    prevalence_weighted: row.try_get("prevalence_weighted")?,
                    median_position: row.try_get("median_position")?,
                    is_staple: row.try_get("is_staple")?,
                })
            })
            .collect::<sqlx::Result<Vec<_>>>()?;
        Ok(Self { items })
    }

    pub(crate) fn staples(&self) -> Vec<i64> {
        let mut staples: Vec<&ItemStat> = self.items.iter().filter(|item| item.is_staple).collect();
        staples.sort_by(|a, b| b.prevalence_raw.total_cmp(&a.prevalence_raw));
        staples.into_iter().map(|item| item.item_id).collect()
    }

    pub(crate) fn prevalence(&self, item_id: i64) -> f64 {
        self.items
            .iter()
            .find(|item| item.item_id == item_id)
            .map(|item| item.prevalence_weighted)
            .unwrap_or(0.0)
    }

    pub(crate) fn population_positions(&self) -> Vec<(i64, f64)> {
        self.items
            .iter()
            .map(|item| (item.item_id, item.median_position))
            .collect()
    }
}
