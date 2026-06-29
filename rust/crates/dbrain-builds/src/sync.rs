use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use serde_json::Value;

use crate::{
    api::DeadlockApiClient,
    classify::{classify_item, ClassifiedItem},
    error::BuildEngineError,
    schema,
    util::{
        json_string, now_epoch_seconds, value_as_i64, value_f64, value_i64, value_string, winrate,
        BRACKET_BADGE_80, PATCH_TAG_CURRENT,
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

pub fn sync_build_data(
    conn: &Connection,
    options: BuildDataSyncOptions,
) -> Result<BuildDataSyncSummary> {
    schema::ensure_schema(conn)?;
    let api = DeadlockApiClient::new(&options.user_agent)?;

    let items = api.items()?;
    let item_catalog_rows = upsert_item_catalog(conn, &items)?;
    let heroes = api.heroes()?;
    let hero_catalog_rows = upsert_hero_catalog(conn, &heroes)?;
    let hero_ids = resolve_sync_hero_ids(conn, &options.hero)?;

    let mut hero_summaries = Vec::new();
    for hero_id in hero_ids {
        let summary = sync_one_hero(conn, &api, hero_id, &options)?;
        hero_summaries.push(summary);
    }

    Ok(BuildDataSyncSummary {
        item_catalog_rows,
        hero_catalog_rows,
        heroes: hero_summaries,
    })
}

fn sync_one_hero(
    conn: &Connection,
    api: &DeadlockApiClient,
    hero_id: i64,
    options: &BuildDataSyncOptions,
) -> Result<HeroBuildDataSyncSummary> {
    let hero_name = hero_name(conn, hero_id)?;
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
    let catalog_ids = item_catalog_ids(conn)?;
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
        conn,
        hero_id,
        &prevalence,
        &item_stats,
        &catalog_ids,
        &lift_map,
    )?;

    let ability_payload = api.ability_order_stats(
        hero_id,
        options.min_average_badge,
        options.min_ability_matches,
    )?;
    analytics_pause(options);
    let ability_order_rows = upsert_ability_order(conn, hero_id, &ability_payload)?;

    let synergy_payload = api.item_permutation_stats(hero_id)?;
    analytics_pause(options);
    let synergy_rows = upsert_synergies(
        conn,
        hero_id,
        &synergy_payload,
        &catalog_ids,
        options.synergy_limit,
    )?;

    Ok(HeroBuildDataSyncSummary {
        hero_id,
        hero_name,
        item_stat_rows,
        lift_rows: lift_map.len(),
        ability_order_rows,
        synergy_rows,
    })
}

pub(crate) fn upsert_item_catalog(conn: &Connection, payload: &Value) -> Result<usize> {
    schema::ensure_schema(conn)?;
    let now = now_epoch_seconds()?;
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

        conn.execute(
            r#"
            INSERT INTO item_catalog(
              item_id, name, slot_type, tier, defense_kind_json, damage_axis, properties_json, updated_at
            )
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(item_id) DO UPDATE SET
              name=excluded.name,
              slot_type=excluded.slot_type,
              tier=excluded.tier,
              defense_kind_json=excluded.defense_kind_json,
              damage_axis=excluded.damage_axis,
              properties_json=excluded.properties_json,
              updated_at=excluded.updated_at
            "#,
            params![
                item_id,
                name,
                slot_type,
                tier,
                json_string(&defense_kind)?,
                damage_axis,
                json_string(&properties)?,
                now,
            ],
        )?;
        count += 1;
    }

    Ok(count)
}

pub(crate) fn upsert_hero_catalog(conn: &Connection, payload: &Value) -> Result<usize> {
    schema::ensure_schema(conn)?;
    let now = now_epoch_seconds()?;
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
        conn.execute(
            r#"
            INSERT INTO hero_catalog(hero_id, name, base_health, archetype, stats_json, updated_at)
            VALUES(?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(hero_id) DO UPDATE SET
              name=excluded.name,
              base_health=excluded.base_health,
              archetype=excluded.archetype,
              stats_json=excluded.stats_json,
              updated_at=excluded.updated_at
            "#,
            params![
                hero_id,
                name,
                base_health,
                archetype,
                json_string(hero)?,
                now,
            ],
        )?;
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

fn resolve_sync_hero_ids(conn: &Connection, hero: &str) -> Result<Vec<i64>> {
    if hero.trim().eq_ignore_ascii_case("all") {
        let mut statement = conn.prepare("SELECT hero_id FROM hero_catalog ORDER BY hero_id")?;
        let ids = statement
            .query_map([], |row| row.get::<_, i64>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        return Ok(ids);
    }

    if let Ok(hero_id) = hero.trim().parse::<i64>() {
        let exists: Option<i64> = conn
            .query_row(
                "SELECT hero_id FROM hero_catalog WHERE hero_id=?1",
                [hero_id],
                |row| row.get(0),
            )
            .optional()?;
        return exists
            .map(|id| vec![id])
            .ok_or_else(|| BuildEngineError::HeroNotFound(hero.to_string()).into());
    }

    crate::engine::resolve_hero_id(conn, hero).map(|id| vec![id])
}

fn hero_name(conn: &Connection, hero_id: i64) -> Result<String> {
    conn.query_row(
        "SELECT name FROM hero_catalog WHERE hero_id=?1",
        [hero_id],
        |row| row.get(0),
    )
    .optional()?
    .ok_or_else(|| BuildEngineError::HeroNotFound(hero_id.to_string()).into())
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

fn item_catalog_ids(conn: &Connection) -> Result<BTreeSet<i64>> {
    let mut statement = conn.prepare("SELECT item_id FROM item_catalog")?;
    let ids = statement
        .query_map([], |row| row.get::<_, i64>(0))?
        .collect::<std::result::Result<BTreeSet<_>, _>>()?;
    Ok(ids)
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

fn upsert_hero_item_stats(
    conn: &Connection,
    hero_id: i64,
    prevalence: &BTreeMap<i64, i64>,
    item_stats: &BTreeMap<i64, ItemStatLine>,
    catalog_ids: &BTreeSet<i64>,
    lift_map: &HashMap<i64, f64>,
) -> Result<usize> {
    let now = now_epoch_seconds()?;
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
        conn.execute(
            r#"
            INSERT INTO hero_item_stats(
              hero_id, item_id, bracket, prevalence_builds, wins, losses, matches, players,
              avg_buy_time_relative, lift_pp, patch_tag, updated_at
            )
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
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
            params![
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
                now,
            ],
        )?;
        count += 1;
    }

    Ok(count)
}

fn upsert_ability_order(conn: &Connection, hero_id: i64, payload: &Value) -> Result<usize> {
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

    let now = now_epoch_seconds()?;
    conn.execute(
        r#"
        INSERT INTO hero_ability_orders(
          hero_id, bracket, abilities_json, wins, losses, matches, players, patch_tag, updated_at
        )
        VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ON CONFLICT(hero_id, bracket, patch_tag) DO UPDATE SET
          abilities_json=excluded.abilities_json,
          wins=excluded.wins,
          losses=excluded.losses,
          matches=excluded.matches,
          players=excluded.players,
          updated_at=excluded.updated_at
        "#,
        params![
            hero_id,
            BRACKET_BADGE_80,
            json_string(&abilities)?,
            value_i64(best, "wins").unwrap_or(0),
            value_i64(best, "losses").unwrap_or(0),
            value_i64(best, "matches").unwrap_or(0),
            value_i64(best, "players").unwrap_or(0),
            PATCH_TAG_CURRENT,
            now,
        ],
    )?;
    Ok(1)
}

fn upsert_synergies(
    conn: &Connection,
    hero_id: i64,
    payload: &Value,
    catalog_ids: &BTreeSet<i64>,
    limit: usize,
) -> Result<usize> {
    let Some(rows) = payload.as_array() else {
        return Ok(0);
    };
    let now = now_epoch_seconds()?;
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
            conn.execute(
                r#"
                INSERT INTO hero_item_synergies(
                  hero_id, item_id, with_item_id, wins, losses, matches, patch_tag, updated_at
                )
                VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ON CONFLICT(hero_id, item_id, with_item_id, patch_tag) DO UPDATE SET
                  wins=excluded.wins,
                  losses=excluded.losses,
                  matches=excluded.matches,
                  updated_at=excluded.updated_at
                "#,
                params![
                    hero_id,
                    item_id,
                    with_item_id,
                    value_i64(row, "wins").unwrap_or(0),
                    value_i64(row, "losses").unwrap_or(0),
                    value_i64(row, "matches").unwrap_or(0),
                    PATCH_TAG_CURRENT,
                    now,
                ],
            )?;
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
pub(crate) fn sync_fixture_payloads(conn: &Connection) -> Result<()> {
    schema::ensure_schema(conn)?;
    upsert_item_catalog(
        conn,
        &crate::util::load_fixture("mo_item_catalog_subset.json"),
    )?;
    upsert_hero_catalog(conn, &crate::util::load_fixture("mo_hero_catalog.json"))?;
    let prevalence = parse_prevalence(&crate::util::load_fixture("mo_build_item_stats.json"))?;
    let item_stats = parse_item_stats(&crate::util::load_fixture("mo_item_stats.json"))?;
    let catalog_ids = item_catalog_ids(conn)?;
    upsert_hero_item_stats(
        conn,
        18,
        &prevalence,
        &item_stats,
        &catalog_ids,
        &HashMap::new(),
    )?;
    upsert_ability_order(
        conn,
        18,
        &crate::util::load_fixture("mo_ability_order_stats.json"),
    )?;
    upsert_synergies(
        conn,
        18,
        &crate::util::load_fixture("mo_item_permutation_stats.json"),
        &catalog_ids,
        200,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn fixtures_sync_into_expected_tables() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        sync_fixture_payloads(&conn).expect("sync fixtures");

        let items: i64 = conn
            .query_row("SELECT COUNT(*) FROM item_catalog", [], |row| row.get(0))
            .expect("count items");
        let stats: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM hero_item_stats WHERE hero_id=18",
                [],
                |row| row.get(0),
            )
            .expect("count stats");
        assert!(items > 10);
        assert!(stats > 10);
    }

    #[test]
    fn mo_catalog_derives_tank_archetype() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        upsert_hero_catalog(&conn, &crate::util::load_fixture("mo_hero_catalog.json"))
            .expect("upsert heroes");
        let archetype: String = conn
            .query_row(
                "SELECT archetype FROM hero_catalog WHERE hero_id=18",
                [],
                |row| row.get(0),
            )
            .expect("archetype");
        assert_eq!(archetype, "tank");
    }
}
