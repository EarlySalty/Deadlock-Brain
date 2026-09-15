use anyhow::{anyhow, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;

use crate::clean::PlayerMatchRow;

pub const DSN_ENV: &str = "POPULATION_DB_DSN";
const MIGRATION: &str = include_str!("../../../../scripts/migrations/2026-09-16-population.sql");

pub async fn connect() -> Result<PgPool> {
    let dsn = std::env::var(DSN_ENV)
        .map_err(|_| anyhow!("{DSN_ENV} ist nicht gesetzt; die DSN wird nie ausgegeben."))?;
    PgPoolOptions::new()
        .max_connections(4)
        .connect(&dsn)
        .await
        .map_err(|_| {
            anyhow!("Verbindung zur Populations-Datenbank fehlgeschlagen (DSN aus {DSN_ENV}).")
        })
}

pub async fn ensure_schema(pool: &PgPool) -> Result<()> {
    sqlx::raw_sql(MIGRATION)
        .execute(pool)
        .await
        .map_err(|error| anyhow!("Migration der Populations-Tabellen fehlgeschlagen: {error}"))?;
    Ok(())
}

pub async fn insert_rows(pool: &PgPool, rows: &[PlayerMatchRow]) -> Result<(usize, usize)> {
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut tx = pool.begin().await?;
    for row in rows {
        let affected = sqlx::query(
            r#"
            INSERT INTO brain.population_player_matches(
              match_id, account_id, hero_id, team, won, average_badge, duration_s,
              start_time, items, buy_times_s, sold_times_s, net_worth_rank,
              imbue_targets, ability_points, ability_times_s
            )
            VALUES($1,$2,$3,$4,$5,$6,$7,$8::text::timestamptz,$9,$10,$11,$12,$13,$14,$15)
            ON CONFLICT (match_id, account_id) DO NOTHING
            "#,
        )
        .bind(row.match_id)
        .bind(row.account_id)
        .bind(row.hero_id)
        .bind(row.team)
        .bind(row.won)
        .bind(row.average_badge)
        .bind(row.duration_s)
        .bind(row.start_time.as_deref())
        .bind(&row.items)
        .bind(&row.buy_times_s)
        .bind(&row.sold_times_s)
        .bind(&row.net_worth_rank)
        .bind(&row.imbue_targets)
        .bind(&row.ability_points)
        .bind(&row.ability_times_s)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if affected > 0 {
            inserted += 1;
        } else {
            skipped += 1;
        }
    }
    tx.commit().await?;
    Ok((inserted, skipped))
}

pub struct SyncRun {
    pub requested_matches: i32,
    pub hero_filter: Option<i64>,
    pub since_unix: Option<i64>,
    pub window_low_match_id: Option<i64>,
    pub window_high_match_id: Option<i64>,
    pub matches_seen: i32,
    pub player_matches_inserted: i32,
    pub player_matches_skipped: i32,
    pub duration_ms: i64,
}

pub async fn record_sync_run(pool: &PgPool, run: &SyncRun) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO brain.population_sync_runs(
          finished_at, requested_matches, hero_filter, since_unix,
          window_low_match_id, window_high_match_id, matches_seen,
          player_matches_inserted, player_matches_skipped, duration_ms
        )
        VALUES(now(),$1,$2,$3,$4,$5,$6,$7,$8,$9)
        "#,
    )
    .bind(run.requested_matches)
    .bind(run.hero_filter)
    .bind(run.since_unix)
    .bind(run.window_low_match_id)
    .bind(run.window_high_match_id)
    .bind(run.matches_seen)
    .bind(run.player_matches_inserted)
    .bind(run.player_matches_skipped)
    .bind(run.duration_ms)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AggRow {
    pub items: Vec<i64>,
    pub buy_times_s: Vec<i32>,
    pub sold_times_s: Vec<Option<i32>>,
    pub imbue_targets: Vec<i64>,
    pub ability_points: Vec<i64>,
    pub average_badge: Option<i32>,
    pub won: bool,
}

pub async fn distinct_hero_ids(pool: &PgPool) -> Result<Vec<i64>> {
    let rows = sqlx::query(
        "SELECT DISTINCT hero_id FROM brain.population_player_matches ORDER BY hero_id",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| row.get::<i64, _>("hero_id"))
        .collect())
}

pub async fn load_hero_rows(pool: &PgPool, hero_id: i64) -> Result<Vec<AggRow>> {
    let rows = sqlx::query(
        r#"
        SELECT items, buy_times_s, sold_times_s, imbue_targets, ability_points,
               average_badge, won
        FROM brain.population_player_matches
        WHERE hero_id = $1
        "#,
    )
    .bind(hero_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| AggRow {
            items: row.get("items"),
            buy_times_s: row.get("buy_times_s"),
            sold_times_s: row.get("sold_times_s"),
            imbue_targets: row.get("imbue_targets"),
            ability_points: row.get("ability_points"),
            average_badge: row.get("average_badge"),
            won: row.get("won"),
        })
        .collect())
}
