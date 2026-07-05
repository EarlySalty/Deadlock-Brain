use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    api::DeadlockApiClient,
    classify::{classify_item, ClassifiedItem},
    error::BuildEngineError,
    util::{
        json_string, value_as_i64, value_f64, value_i64, value_string, winrate, BRACKET_BADGE_80,
        PATCH_TAG_CURRENT,
    },
};

const DEFAULT_MIN_AVERAGE_BADGE: i64 = 80;
const DEFAULT_MIN_ABILITY_MATCHES: i64 = 200;
const DEFAULT_ANALYTICS_DELAY_MS: u64 = 325;
const DEFAULT_LIFT_ITEM_LIMIT: usize = 16;
const DEFAULT_SYNERGY_LIMIT: usize = 1_500;

#[derive(Debug, Clone)]
pub struct BuildDataSyncOptions {
    pub hero: String,
    pub user_agent: String,
    pub min_average_badge: i64,
    pub min_ability_matches: i64,
    pub analytics_delay_ms: u64,
    pub lift_item_limit: usize,
    pub synergy_limit: usize,
}

impl BuildDataSyncOptions {
    pub fn new(hero: impl Into<String>, user_agent: impl Into<String>) -> Self {
        Self {
            hero: hero.into(),
            user_agent: user_agent.into(),
            min_average_badge: DEFAULT_MIN_AVERAGE_BADGE,
            min_ability_matches: DEFAULT_MIN_ABILITY_MATCHES,
            analytics_delay_ms: DEFAULT_ANALYTICS_DELAY_MS,
            lift_item_limit: DEFAULT_LIFT_ITEM_LIMIT,
            synergy_limit: DEFAULT_SYNERGY_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BuildDataSyncSummary {
    pub item_catalog_rows: usize,
    pub hero_catalog_rows: usize,
    pub heroes: Vec<HeroBuildDataSyncSummary>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HeroBuildDataSyncSummary {
    pub hero_id: i64,
    pub hero_name: String,
    pub item_stat_rows: usize,
    pub lift_rows: usize,
    pub ability_order_rows: usize,
    pub synergy_rows: usize,
}

pub async fn sync_build_data(
    pool: &PgPool,
    options: BuildDataSyncOptions,
) -> Result<BuildDataSyncSummary> {
    let api = DeadlockApiClient::new(&options.user_agent)?;

    let items = api.items()?;
    let item_catalog_rows = upsert_item_catalog(pool, &items).await?;
    let heroes = api.heroes()?;
    let hero_catalog_rows = upsert_hero_catalog(pool, &heroes).await?;
    let hero_ids = resolve_sync_hero_ids(pool, &options.hero).await?;

    let mut hero_summaries = Vec::new();
    for hero_id in hero_ids {
        let summary = sync_one_hero(pool, &api, hero_id, &options).await?;
        hero_summaries.push(summary);
    }

    Ok(BuildDataSyncSummary {
        item_catalog_rows,
        hero_catalog_rows,
        heroes: hero_summaries,
    })
}

async fn sync_one_hero(
    pool: &PgPool,
    api: &DeadlockApiClient,
    hero_id: i64,
    options: &BuildDataSyncOptions,
) -> Result<HeroBuildDataSyncSummary> {
    let hero_name = hero_name(pool, hero_id).await?;
    let prevalence_payload = api.build_item_stats(hero_id)?;
    analytics_pause(options);
    let item_stats_payload = api.item_stats(hero_id, options.min_average_badge)?;
    analytics_pause(options);
    let baseline_payload = api.hero_stats(hero_id, options.min_average_badge)?;
    analytics_pause(options);

    let baseline = stat_line_for_hero(&baseline_payload, hero_id)?;
    let baseline_wr = baseline
        .winrate()
        .ok_or(BuildEngineError::MissingHeroStats(hero_id))?;
    let prevalence = parse_prevalence(&prevalence_payload)?;
    let item_stats = parse_item_stats(&item_stats_payload)?;
    let catalog_ids = item_catalog_ids(pool).await?;
    let lift_ids = lift_candidate_ids(
        &prevalence,
        &item_stats,
        &catalog_ids,
        options.lift_item_limit,
    );
    let mut lift_map = HashMap::new();
    for item_id in lift_ids {
        let payload = api.hero_stats_with_item(hero_id, item_id, options.min_average_badge)?;
        analytics_pause(options);
        if let Ok(with_item) = stat_line_for_hero(&payload, hero_id) {
            if let Some(with_item_wr) = with_item.winrate() {
                lift_map.insert(item_id, (with_item_wr - baseline_wr) * 100.0);
            }
        }
    }

    let item_stat_rows = upsert_hero_item_stats(
        pool,
        hero_id,
        &prevalence,
        &item_stats,
        &catalog_ids,
        &lift_map,
    )
    .await?;

    let ability_payload = api.ability_order_stats(
        hero_id,
        options.min_average_badge,
        options.min_ability_matches,
    )?;
    analytics_pause(options);
    let ability_order_rows = upsert_ability_order(pool, hero_id, &ability_payload).await?;

    let synergy_payload = api.item_permutation_stats(hero_id)?;
    analytics_pause(options);
    let synergy_rows = upsert_synergies(
        pool,
        hero_id,
        &synergy_payload,
        &catalog_ids,
        options.synergy_limit,
    )
    .await?;

    Ok(HeroBuildDataSyncSummary {
        hero_id,
        hero_name,
        item_stat_rows,
        lift_rows: lift_map.len(),
        ability_order_rows,
        synergy_rows,
    })
}

pub(crate) async fn upsert_item_catalog(pool: &PgPool, payload: &Value) -> Result<usize> {
    let items = payload
        .as_array()
        .ok_or_else(|| anyhow!("Item-Katalog ist kein JSON-Array"))?;
    let mut count = 0usize;

    for item in items {
        if value_string(item, "type").as_deref() != Some("upgrade") {
            continue;
        }
        let Some(slot_type) = value_string(item, "item_slot_type") else {
            continue;
        };
        if !matches!(slot_type.as_str(), "weapon" | "vitality" | "spirit") {
            continue;
        }
        let Some(item_id) = value_i64(item, "id") else {
            continue;
        };
        let Some(name) = value_string(item, "name") else {
            continue;
        };
        let tier = value_i64(item, "item_tier")
            .or_else(|| value_i64(item, "tier"))
            .unwrap_or(0);
        let properties = item.get("properties").cloned().unwrap_or(Value::Null);
        let ClassifiedItem {
            defense_kind,
            damage_axis,
        } = classify_item(item);

        sqlx::query!(
            r#"
            INSERT INTO brain.item_catalog(
              item_id, name, slot_type, tier, defense_kind, damage_axis, properties, updated_at
            )
            VALUES($1, $2, $3, $4, $5::text::jsonb, $6, $7::text::jsonb, now())
            ON CONFLICT(item_id) DO UPDATE SET
              name=excluded.name,
              slot_type=excluded.slot_type,
              tier=excluded.tier,
              defense_kind=excluded.defense_kind,
              damage_axis=excluded.damage_axis,
              properties=excluded.properties,
              updated_at=excluded.updated_at
            "#,
            item_id,
            name,
            slot_type,
            tier,
            json_string(&defense_kind)?,
            damage_axis,
            json_string(&properties)?,
        )
        .execute(pool)
        .await?;
        count += 1;
    }

    Ok(count)
}

pub(crate) async fn upsert_hero_catalog(pool: &PgPool, payload: &Value) -> Result<usize> {
    let heroes = payload
        .as_array()
        .ok_or_else(|| anyhow!("Hero-Katalog ist kein JSON-Array"))?;
    let mut count = 0usize;

    for hero in heroes {
        let Some(hero_id) = value_i64(hero, "id") else {
            continue;
        };
        let Some(name) = value_string(hero, "name") else {
            continue;
        };
        let base_health = base_health(hero).unwrap_or(0);
        let archetype = derive_archetype(base_health);
        sqlx::query!(
            r#"
            INSERT INTO brain.hero_catalog(hero_id, name, base_health, archetype, stats, updated_at)
            VALUES($1, $2, $3, $4, $5::text::jsonb, now())
            ON CONFLICT(hero_id) DO UPDATE SET
              name=excluded.name,
              base_health=excluded.base_health,
              archetype=excluded.archetype,
              stats=excluded.stats,
              updated_at=excluded.updated_at
            "#,
            hero_id,
            name,
            base_health,
            archetype,
            json_string(hero)?,
        )
        .execute(pool)
        .await?;
        count += 1;
    }

    Ok(count)
}

fn base_health(hero: &Value) -> Option<i64> {
    hero.get("starting_stats")
        .and_then(|stats| stats.get("max_health"))
        .and_then(|max_health| max_health.get("value"))
        .and_then(value_as_i64)
}

fn derive_archetype(base_health: i64) -> &'static str {
    // Live starting-health spread is currently roughly 500-930. >=850 is a
    // frontline tank, >=760 is a bruiser, lower values are treated as squishy.
    if base_health >= 850 {
        "tank"
    } else if base_health >= 760 {
        "bruiser"
    } else {
        "squishy"
    }
}

async fn resolve_sync_hero_ids(pool: &PgPool, hero: &str) -> Result<Vec<i64>> {
    if hero.trim().eq_ignore_ascii_case("all") {
        let ids = sqlx::query_scalar!("SELECT hero_id FROM brain.hero_catalog ORDER BY hero_id")
            .fetch_all(pool)
            .await?;
        return Ok(ids);
    }

    if let Ok(hero_id) = hero.trim().parse::<i64>() {
        let exists: Option<i64> = sqlx::query_scalar!(
            "SELECT hero_id FROM brain.hero_catalog WHERE hero_id=$1",
            hero_id,
        )
        .fetch_optional(pool)
        .await?;
        return exists
            .map(|id| vec![id])
            .ok_or_else(|| BuildEngineError::HeroNotFound(hero.to_string()).into());
    }

    crate::engine::resolve_hero_id(pool, hero)
        .await
        .map(|id| vec![id])
}

async fn hero_name(pool: &PgPool, hero_id: i64) -> Result<String> {
    let name: Option<String> = sqlx::query_scalar!(
        "SELECT name FROM brain.hero_catalog WHERE hero_id=$1",
        hero_id,
    )
    .fetch_optional(pool)
    .await?;
    name.ok_or_else(|| BuildEngineError::HeroNotFound(hero_id.to_string()).into())
}

#[derive(Debug, Clone, Copy)]
struct StatLine {
    wins: i64,
    losses: i64,
    matches: i64,
}

impl StatLine {
    fn winrate(self) -> Option<f64> {
        winrate(self.wins, self.losses, self.matches)
    }
}

#[derive(Debug, Clone, Copy)]
struct ItemStatLine {
    wins: i64,
    losses: i64,
    matches: i64,
    players: i64,
    avg_buy_time_relative: Option<f64>,
}

fn stat_line_for_hero(payload: &Value, hero_id: i64) -> Result<StatLine> {
    match payload {
        Value::Array(rows) => rows
            .iter()
            .find(|row| value_i64(row, "hero_id") == Some(hero_id))
            .map(stat_line_from_value)
            .transpose()?
            .ok_or_else(|| BuildEngineError::MissingHeroStats(hero_id).into()),
        Value::Object(_) if value_i64(payload, "hero_id") == Some(hero_id) => {
            stat_line_from_value(payload)
        }
        _ => Err(BuildEngineError::MissingHeroStats(hero_id).into()),
    }
}

fn stat_line_from_value(value: &Value) -> Result<StatLine> {
    Ok(StatLine {
        wins: value_i64(value, "wins").unwrap_or(0),
        losses: value_i64(value, "losses").unwrap_or(0),
        matches: value_i64(value, "matches").unwrap_or(0),
    })
}

fn parse_prevalence(payload: &Value) -> Result<BTreeMap<i64, i64>> {
    let rows = payload
        .as_array()
        .ok_or_else(|| anyhow!("Build-Item-Stats sind kein JSON-Array"))?;
    let mut map = BTreeMap::new();
    for row in rows {
        if let Some(item_id) = value_i64(row, "item_id") {
            map.insert(item_id, value_i64(row, "builds").unwrap_or(0));
        }
    }
    Ok(map)
}

fn parse_item_stats(payload: &Value) -> Result<BTreeMap<i64, ItemStatLine>> {
    let rows = payload
        .as_array()
        .ok_or_else(|| anyhow!("Item-Stats sind kein JSON-Array"))?;
    let mut map = BTreeMap::new();
    for row in rows {
        if let Some(item_id) = value_i64(row, "item_id") {
            map.insert(
                item_id,
                ItemStatLine {
                    wins: value_i64(row, "wins").unwrap_or(0),
                    losses: value_i64(row, "losses").unwrap_or(0),
                    matches: value_i64(row, "matches").unwrap_or(0),
                    players: value_i64(row, "players").unwrap_or(0),
                    avg_buy_time_relative: value_f64(row, "avg_buy_time_relative"),
                },
            );
        }
    }
    Ok(map)
}

async fn item_catalog_ids(pool: &PgPool) -> Result<BTreeSet<i64>> {
    let ids = sqlx::query_scalar!("SELECT item_id FROM brain.item_catalog")
        .fetch_all(pool)
        .await?;
    Ok(ids.into_iter().collect())
}

fn lift_candidate_ids(
    prevalence: &BTreeMap<i64, i64>,
    item_stats: &BTreeMap<i64, ItemStatLine>,
    catalog_ids: &BTreeSet<i64>,
    limit: usize,
) -> Vec<i64> {
    let mut candidates = BTreeSet::new();
    candidates.extend(prevalence.keys().copied());
    candidates.extend(item_stats.keys().copied());

    let mut scored = candidates
        .into_iter()
        .filter(|item_id| catalog_ids.contains(item_id))
        .filter_map(|item_id| {
            let builds = prevalence.get(&item_id).copied().unwrap_or(0);
            let matches = item_stats
                .get(&item_id)
                .map(|line| line.matches)
                .unwrap_or(0);
            (builds >= 30 && matches >= 500).then_some((item_id, builds, matches))
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| right.2.cmp(&left.2))
            .then_with(|| left.0.cmp(&right.0))
    });
    scored
        .into_iter()
        .take(limit)
        .map(|(item_id, _, _)| item_id)
        .collect()
}

async fn upsert_hero_item_stats(
    pool: &PgPool,
    hero_id: i64,
    prevalence: &BTreeMap<i64, i64>,
    item_stats: &BTreeMap<i64, ItemStatLine>,
    catalog_ids: &BTreeSet<i64>,
    lift_map: &HashMap<i64, f64>,
) -> Result<usize> {
    let mut ids = BTreeSet::new();
    ids.extend(prevalence.keys().copied());
    ids.extend(item_stats.keys().copied());
    let mut count = 0usize;

    for item_id in ids {
        if !catalog_ids.contains(&item_id) {
            continue;
        }
        let line = item_stats.get(&item_id).copied().unwrap_or(ItemStatLine {
            wins: 0,
            losses: 0,
            matches: 0,
            players: 0,
            avg_buy_time_relative: None,
        });
        sqlx::query!(
            r#"
            INSERT INTO brain.hero_item_stats(
              hero_id, item_id, bracket, prevalence_builds, wins, losses, matches, players,
              avg_buy_time_relative, lift_pp, patch_tag, updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now())
            ON CONFLICT(hero_id, item_id, bracket, patch_tag) DO UPDATE SET
              prevalence_builds=excluded.prevalence_builds,
              wins=excluded.wins,
              losses=excluded.losses,
              matches=excluded.matches,
              players=excluded.players,
              avg_buy_time_relative=excluded.avg_buy_time_relative,
              lift_pp=excluded.lift_pp,
              updated_at=excluded.updated_at
            "#,
            hero_id,
            item_id,
            BRACKET_BADGE_80,
            prevalence.get(&item_id).copied().unwrap_or(0),
            line.wins,
            line.losses,
            line.matches,
            line.players,
            line.avg_buy_time_relative,
            lift_map.get(&item_id).copied(),
            PATCH_TAG_CURRENT,
        )
        .execute(pool)
        .await?;
        count += 1;
    }

    Ok(count)
}

async fn upsert_ability_order(pool: &PgPool, hero_id: i64, payload: &Value) -> Result<usize> {
    let Some(best) = payload.as_array().and_then(|rows| {
        rows.iter()
            .max_by_key(|row| value_i64(row, "matches").unwrap_or(0))
    }) else {
        return Ok(0);
    };
    let abilities = best
        .get("abilities")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    if abilities.is_empty() {
        return Ok(0);
    }

    sqlx::query!(
        r#"
        INSERT INTO brain.hero_ability_orders(
          hero_id, bracket, abilities, wins, losses, matches, players, patch_tag, updated_at
        )
        VALUES($1, $2, $3::text::jsonb, $4, $5, $6, $7, $8, now())
        ON CONFLICT(hero_id, bracket, patch_tag) DO UPDATE SET
          abilities=excluded.abilities,
          wins=excluded.wins,
          losses=excluded.losses,
          matches=excluded.matches,
          players=excluded.players,
          updated_at=excluded.updated_at
        "#,
        hero_id,
        BRACKET_BADGE_80,
        json_string(&abilities)?,
        value_i64(best, "wins").unwrap_or(0),
        value_i64(best, "losses").unwrap_or(0),
        value_i64(best, "matches").unwrap_or(0),
        value_i64(best, "players").unwrap_or(0),
        PATCH_TAG_CURRENT,
    )
    .execute(pool)
    .await?;
    Ok(1)
}

async fn upsert_synergies(
    pool: &PgPool,
    hero_id: i64,
    payload: &Value,
    catalog_ids: &BTreeSet<i64>,
    limit: usize,
) -> Result<usize> {
    let Some(rows) = payload.as_array() else {
        return Ok(0);
    };
    let mut count = 0usize;
    for row in rows.iter().take(limit) {
        let Some(item_ids) = row.get("item_ids").and_then(Value::as_array) else {
            continue;
        };
        if item_ids.len() != 2 {
            continue;
        }
        let Some(first) = item_ids.first().and_then(value_as_i64) else {
            continue;
        };
        let Some(second) = item_ids.get(1).and_then(value_as_i64) else {
            continue;
        };
        if !catalog_ids.contains(&first) || !catalog_ids.contains(&second) {
            continue;
        }
        for (item_id, with_item_id) in [(first, second), (second, first)] {
            sqlx::query!(
                r#"
                INSERT INTO brain.hero_item_synergies(
                  hero_id, item_id, with_item_id, wins, losses, matches, patch_tag, updated_at
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, now())
                ON CONFLICT(hero_id, item_id, with_item_id, patch_tag) DO UPDATE SET
                  wins=excluded.wins,
                  losses=excluded.losses,
                  matches=excluded.matches,
                  updated_at=excluded.updated_at
                "#,
                hero_id,
                item_id,
                with_item_id,
                value_i64(row, "wins").unwrap_or(0),
                value_i64(row, "losses").unwrap_or(0),
                value_i64(row, "matches").unwrap_or(0),
                PATCH_TAG_CURRENT,
            )
            .execute(pool)
            .await?;
            count += 1;
        }
    }
    Ok(count)
}

fn analytics_pause(options: &BuildDataSyncOptions) {
    if options.analytics_delay_ms > 0 {
        thread::sleep(Duration::from_millis(options.analytics_delay_ms));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Schreib-/Lese-Roundtrip gegen die Scratch-Postgres. Synthetische Hoch-IDs,
    // die nicht mit echten Daten kollidieren, plus Aufraeumen am Anfang und Ende.
    const TEST_HERO_ID: i64 = 990018;
    const TEST_ITEM_ID: i64 = 990001;

    async fn cleanup(pool: &PgPool) {
        // Reihenfolge wegen Fremdschluesseln: erst abhaengige Tabellen, dann Kataloge.
        for sql in [
            "DELETE FROM brain.hero_item_stats WHERE hero_id=$1",
            "DELETE FROM brain.hero_ability_orders WHERE hero_id=$1",
            "DELETE FROM brain.hero_item_synergies WHERE hero_id=$1",
            "DELETE FROM brain.hero_catalog WHERE hero_id=$1",
        ] {
            sqlx::query(sql)
                .bind(TEST_HERO_ID)
                .execute(pool)
                .await
                .expect("cleanup hero");
        }
        sqlx::query("DELETE FROM brain.item_catalog WHERE item_id=$1")
            .bind(TEST_ITEM_ID)
            .execute(pool)
            .await
            .expect("cleanup item");
    }

    #[tokio::test]
    #[ignore = "benoetigt Scratch-Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn sync_upsert_roundtrip_writes_and_reads_back() {
        let Some(pool) = crate::util::test_pool().await else {
            eprintln!("DEADLOCK_CENTRAL_DSN nicht gesetzt; Test uebersprungen.");
            return;
        };
        cleanup(&pool).await;

        // Item-Katalog (API-Shape: type=upgrade, item_slot_type in {weapon,vitality,spirit}).
        let items = json!([{
            "id": TEST_ITEM_ID,
            "type": "upgrade",
            "name": "Roundtrip Testitem",
            "item_slot_type": "spirit",
            "item_tier": 2,
            "properties": {}
        }]);
        let item_rows = upsert_item_catalog(&pool, &items)
            .await
            .expect("upsert items");
        assert_eq!(item_rows, 1);

        // Hero-Katalog (API-Shape mit starting_stats.max_health.value=900 -> archetype "tank").
        let heroes = json!([{
            "id": TEST_HERO_ID,
            "name": "Roundtrip Testheld",
            "starting_stats": {"max_health": {"value": 900}}
        }]);
        let hero_rows = upsert_hero_catalog(&pool, &heroes)
            .await
            .expect("upsert heroes");
        assert_eq!(hero_rows, 1);

        // Hero-Item-Stats-Schreibpfad.
        let mut prevalence = BTreeMap::new();
        prevalence.insert(TEST_ITEM_ID, 42_i64);
        let mut item_stats = BTreeMap::new();
        item_stats.insert(
            TEST_ITEM_ID,
            ItemStatLine {
                wins: 700,
                losses: 300,
                matches: 1000,
                players: 1200,
                avg_buy_time_relative: Some(25.0),
            },
        );
        let mut catalog_ids = BTreeSet::new();
        catalog_ids.insert(TEST_ITEM_ID);
        let mut lift_map = HashMap::new();
        lift_map.insert(TEST_ITEM_ID, 3.5_f64);
        let stat_rows = upsert_hero_item_stats(
            &pool,
            TEST_HERO_ID,
            &prevalence,
            &item_stats,
            &catalog_ids,
            &lift_map,
        )
        .await
        .expect("upsert stats");
        assert_eq!(stat_rows, 1);

        // Lesepfad: zurueckgelesene Werte muessen den geschriebenen entsprechen.
        let read: (i64, i64, i64, Option<f64>, Option<f64>) = sqlx::query_as(
            "SELECT prevalence_builds, wins, matches, avg_buy_time_relative, lift_pp \
             FROM brain.hero_item_stats \
             WHERE hero_id=$1 AND item_id=$2 AND bracket='badge80' AND patch_tag='current'",
        )
        .bind(TEST_HERO_ID)
        .bind(TEST_ITEM_ID)
        .fetch_one(&pool)
        .await
        .expect("read back stats");
        assert_eq!(read.0, 42);
        assert_eq!(read.1, 700);
        assert_eq!(read.2, 1000);
        assert_eq!(read.3, Some(25.0));
        assert_eq!(read.4, Some(3.5));

        // Archetyp-Ableitung wurde persistiert (900 base_health -> "tank").
        let archetype: String =
            sqlx::query_scalar("SELECT archetype FROM brain.hero_catalog WHERE hero_id=$1")
                .bind(TEST_HERO_ID)
                .fetch_one(&pool)
                .await
                .expect("read archetype");
        assert_eq!(archetype, "tank");

        cleanup(&pool).await;
    }
}
