use std::collections::HashMap;

use serde_json::Value;

use crate::catalog::Catalog;

#[derive(Debug, Clone)]
pub struct PlayerMatchRow {
    pub match_id: i64,
    pub account_id: i64,
    pub hero_id: i64,
    pub team: i16,
    pub won: bool,
    pub average_badge: Option<i32>,
    pub duration_s: Option<i32>,
    pub start_time: Option<String>,
    pub items: Vec<i64>,
    pub buy_times_s: Vec<i32>,
    pub sold_times_s: Vec<Option<i32>>,
    pub net_worth_rank: Vec<i16>,
    pub imbue_targets: Vec<i64>,
    pub ability_points: Vec<i64>,
    pub ability_times_s: Vec<i32>,
}

struct Purchase {
    item_id: i64,
    time_s: i32,
    sold_time_s: Option<i32>,
    imbue_target: i64,
}

pub fn player_won(
    player: &Value,
    winning_team: Option<&str>,
    match_outcome: Option<&str>,
) -> Option<bool> {
    match player.get("player_match_outcome").and_then(Value::as_str) {
        Some("Win") => return Some(true),
        Some("Loss") | Some("Penalized") | Some("PenalizedParty") => return Some(false),
        Some("Invalid") | Some("NotScored") => return None,
        _ => {}
    }
    let team = player.get("team").and_then(Value::as_str)?;
    let winner = winning_team?;
    if winner.is_empty() || winner == "Spectator" {
        return None;
    }
    if !matches!(match_outcome, None | Some("TeamWin")) {
        return None;
    }
    Some(team == winner)
}

pub fn match_to_rows(match_value: &Value, catalog: &Catalog) -> Vec<PlayerMatchRow> {
    let Some(players) = match_value.get("players").and_then(Value::as_array) else {
        return Vec::new();
    };
    let Some(match_id) = v_i64(match_value, "match_id") else {
        return Vec::new();
    };
    let winning_team = match_value.get("winning_team").and_then(Value::as_str);
    let match_outcome = match_value.get("match_outcome").and_then(Value::as_str);
    let average_badge = v_i64(match_value, "average_badge")
        .filter(|badge| *badge > 0)
        .map(|badge| badge as i32);
    let duration_s = v_i64(match_value, "duration_s").map(|value| value as i32);
    let start_time = match_value
        .get("start_time")
        .and_then(Value::as_str)
        .map(|text| text.to_string());

    let curves: Vec<Vec<(f64, f64)>> = players.iter().map(networth_series).collect();

    let mut rows = Vec::new();
    for (index, player) in players.iter().enumerate() {
        let Some(won) = player_won(player, winning_team, match_outcome) else {
            continue;
        };
        let Some(team) = player
            .get("team")
            .and_then(Value::as_str)
            .and_then(parse_team)
        else {
            continue;
        };
        let Some(account_id) = v_i64(player, "account_id") else {
            continue;
        };
        let Some(hero_id) = v_i64(player, "hero_id") else {
            continue;
        };
        let purchases = clean_purchases(player, catalog);
        if purchases.is_empty() {
            continue;
        }
        let abilities = clean_ability_points(player, catalog);

        let mut items = Vec::with_capacity(purchases.len());
        let mut buy_times_s = Vec::with_capacity(purchases.len());
        let mut sold_times_s = Vec::with_capacity(purchases.len());
        let mut net_worth_rank = Vec::with_capacity(purchases.len());
        let mut imbue_targets = Vec::with_capacity(purchases.len());
        for purchase in &purchases {
            items.push(purchase.item_id);
            buy_times_s.push(purchase.time_s);
            sold_times_s.push(purchase.sold_time_s);
            imbue_targets.push(purchase.imbue_target);
            net_worth_rank.push(networth_quintile(&curves, index, purchase.time_s as f64));
        }

        let ability_points = abilities.iter().map(|(id, _)| *id).collect();
        let ability_times_s = abilities.iter().map(|(_, time)| *time).collect();

        rows.push(PlayerMatchRow {
            match_id,
            account_id,
            hero_id,
            team,
            won,
            average_badge,
            duration_s,
            start_time: start_time.clone(),
            items,
            buy_times_s,
            sold_times_s,
            net_worth_rank,
            imbue_targets,
            ability_points,
            ability_times_s,
        });
    }
    rows
}

fn clean_purchases(player: &Value, catalog: &Catalog) -> Vec<Purchase> {
    let Some(entries) = player.get("items").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut purchases: Vec<Purchase> = entries
        .iter()
        .filter_map(|entry| {
            let item_id = v_i64(entry, "item_id")?;
            if !catalog.is_purchase(item_id) || catalog.is_tier5(item_id) {
                return None;
            }
            let time_s = v_i64(entry, "game_time_s").unwrap_or(0) as i32;
            let sold = v_i64(entry, "sold_time_s").unwrap_or(0);
            let sold_time_s = if sold > 0 { Some(sold as i32) } else { None };
            let imbue_target = if catalog.imbueable(item_id) {
                v_i64(entry, "imbued_ability_id").unwrap_or(0)
            } else {
                0
            };
            Some(Purchase {
                item_id,
                time_s,
                sold_time_s,
                imbue_target,
            })
        })
        .collect();
    purchases.sort_by_key(|purchase| purchase.time_s);
    purchases
}

fn clean_ability_points(player: &Value, catalog: &Catalog) -> Vec<(i64, i32)> {
    let Some(entries) = player.get("items").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut abilities: Vec<(i64, i32)> = entries
        .iter()
        .filter_map(|entry| {
            let item_id = v_i64(entry, "item_id")?;
            if !catalog.is_ability(item_id) {
                return None;
            }
            let time_s = v_i64(entry, "game_time_s").unwrap_or(0) as i32;
            Some((item_id, time_s))
        })
        .collect();
    abilities.sort_by_key(|(_, time)| *time);
    abilities
}

fn networth_series(player: &Value) -> Vec<(f64, f64)> {
    let mut series: Vec<(f64, f64)> = player
        .get("stats")
        .and_then(Value::as_array)
        .map(|stats| {
            stats
                .iter()
                .filter_map(|sample| {
                    let time = sample.get("time_stamp_s").and_then(Value::as_f64)?;
                    let worth = sample.get("net_worth").and_then(Value::as_f64)?;
                    Some((time, worth))
                })
                .collect()
        })
        .unwrap_or_default();
    series.sort_by(|a, b| a.0.total_cmp(&b.0));
    if series.first().map(|(time, _)| *time > 0.0).unwrap_or(true) {
        series.insert(0, (0.0, 0.0));
    }
    series
}

fn interpolate(series: &[(f64, f64)], time: f64) -> f64 {
    if series.is_empty() {
        return 0.0;
    }
    if time <= series[0].0 {
        return series[0].1;
    }
    if time >= series[series.len() - 1].0 {
        return series[series.len() - 1].1;
    }
    for window in series.windows(2) {
        let (t0, w0) = window[0];
        let (t1, w1) = window[1];
        if time <= t1 {
            if (t1 - t0).abs() < f64::EPSILON {
                return w1;
            }
            let ratio = (time - t0) / (t1 - t0);
            return w0 + ratio * (w1 - w0);
        }
    }
    series[series.len() - 1].1
}

fn networth_quintile(curves: &[Vec<(f64, f64)>], player_index: usize, time: f64) -> i16 {
    let total = curves.len();
    if total <= 1 {
        return 0;
    }
    let mine = interpolate(&curves[player_index], time);
    let below = curves
        .iter()
        .enumerate()
        .filter(|(index, curve)| *index != player_index && interpolate(curve, time) < mine)
        .count();
    let rank = below as f64 / (total - 1) as f64;
    ((rank * 5.0).floor() as i64).clamp(0, 4) as i16
}

fn parse_team(team: &str) -> Option<i16> {
    match team {
        "Team0" => Some(0),
        "Team1" => Some(1),
        _ => None,
    }
}

fn v_i64(value: &Value, key: &str) -> Option<i64> {
    value
        .get(key)
        .and_then(|inner| inner.as_i64().or_else(|| inner.as_f64().map(|f| f as i64)))
}

pub fn neigung_bucket(items: &[i64], catalog: &Catalog) -> Option<String> {
    let mut souls: HashMap<&str, i64> = HashMap::new();
    for item in items {
        if let Some(slot) = catalog.slot_type(*item) {
            *souls.entry(slot).or_insert(0) += catalog.cost(*item);
        }
    }
    let total: i64 = souls.values().sum();
    if total <= 0 {
        return None;
    }
    ["weapon", "spirit", "vitality"]
        .into_iter()
        .map(|slot| (slot, *souls.get(slot).unwrap_or(&0)))
        .max_by_key(|(_, sum)| *sum)
        .filter(|(_, sum)| *sum > 0)
        .map(|(slot, _)| slot.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn catalog() -> Catalog {
        let items = json!([
            {"id": 100, "type": "upgrade", "name": "Cheap Weapon", "item_slot_type": "weapon", "item_tier": 1, "cost": 800},
            {"id": 101, "type": "upgrade", "name": "Weapon Two", "item_slot_type": "weapon", "item_tier": 2, "cost": 1600},
            {"id": 200, "type": "upgrade", "name": "Spirit Imbue", "item_slot_type": "spirit", "item_tier": 2, "cost": 1600, "imbue": "imbue_active"},
            {"id": 300, "type": "upgrade", "name": "Endgame", "item_slot_type": "vitality", "item_tier": 5, "cost": 9999},
            {"id": 900, "type": "ability", "name": "ability_one"},
            {"id": 901, "type": "ability", "name": "ability_two"}
        ]);
        let heroes = json!([{"id": 8, "name": "Warden"}]);
        Catalog::from_payloads(&items, &heroes).unwrap()
    }

    fn two_player_match() -> Value {
        json!({
            "match_id": 42,
            "winning_team": "Team1",
            "match_outcome": "TeamWin",
            "average_badge": 90,
            "duration_s": 1500,
            "start_time": "2026-09-15 22:54:48",
            "players": [
                {
                    "account_id": 111,
                    "hero_id": 8,
                    "player_slot": 1,
                    "team": "Team0",
                    "net_worth": 20000,
                    "stats": [{"time_stamp_s": 600, "net_worth": 8000}, {"time_stamp_s": 1200, "net_worth": 16000}],
                    "items": [
                        {"item_id": 101, "game_time_s": 500, "sold_time_s": 700, "imbued_ability_id": 0},
                        {"item_id": 100, "game_time_s": 120, "sold_time_s": 0, "imbued_ability_id": 0},
                        {"item_id": 300, "game_time_s": 1400, "sold_time_s": 0, "imbued_ability_id": 0},
                        {"item_id": 200, "game_time_s": 800, "sold_time_s": 0, "imbued_ability_id": 555},
                        {"item_id": 900, "game_time_s": 5, "upgrade_info": 65537},
                        {"item_id": 901, "game_time_s": 60, "upgrade_info": 65537},
                        {"item_id": 900, "game_time_s": 200, "upgrade_info": 196609}
                    ]
                },
                {
                    "account_id": 222,
                    "hero_id": 8,
                    "player_slot": 2,
                    "team": "Team1",
                    "net_worth": 30000,
                    "stats": [{"time_stamp_s": 600, "net_worth": 12000}, {"time_stamp_s": 1200, "net_worth": 24000}],
                    "items": [
                        {"item_id": 100, "game_time_s": 100, "sold_time_s": 0, "imbued_ability_id": 0}
                    ]
                }
            ]
        })
    }

    #[test]
    fn purchases_are_sorted_and_ability_points_split_out() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        let player = rows.iter().find(|row| row.account_id == 111).unwrap();
        assert_eq!(player.items, vec![100, 101, 200]);
        assert_eq!(player.buy_times_s, vec![120, 500, 800]);
        assert_eq!(player.ability_points, vec![900, 901, 900]);
        assert_eq!(player.ability_times_s, vec![5, 60, 200]);
    }

    #[test]
    fn tier_five_items_are_dropped() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        let player = rows.iter().find(|row| row.account_id == 111).unwrap();
        assert!(!player.items.contains(&300));
    }

    #[test]
    fn won_falls_back_to_winning_team() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        let loser = rows.iter().find(|row| row.account_id == 111).unwrap();
        let winner = rows.iter().find(|row| row.account_id == 222).unwrap();
        assert!(!loser.won);
        assert!(winner.won);
    }

    #[test]
    fn imbue_target_only_on_imbueable_items() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        let player = rows.iter().find(|row| row.account_id == 111).unwrap();
        let spirit_index = player.items.iter().position(|item| *item == 200).unwrap();
        assert_eq!(player.imbue_targets[spirit_index], 555);
        let weapon_index = player.items.iter().position(|item| *item == 100).unwrap();
        assert_eq!(player.imbue_targets[weapon_index], 0);
    }

    #[test]
    fn sold_times_are_nullable() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        let player = rows.iter().find(|row| row.account_id == 111).unwrap();
        let weapon_two_index = player.items.iter().position(|item| *item == 101).unwrap();
        assert_eq!(player.sold_times_s[weapon_two_index], Some(700));
        let weapon_index = player.items.iter().position(|item| *item == 100).unwrap();
        assert_eq!(player.sold_times_s[weapon_index], None);
    }

    #[test]
    fn net_worth_rank_stays_in_quintile_range() {
        let rows = match_to_rows(&two_player_match(), &catalog());
        for row in &rows {
            for rank in &row.net_worth_rank {
                assert!((0..=4).contains(rank));
            }
        }
    }

    #[test]
    fn invalid_outcome_drops_player() {
        let mut value = two_player_match();
        value["players"][0]["player_match_outcome"] = json!("NotScored");
        let rows = match_to_rows(&value, &catalog());
        assert!(rows.iter().all(|row| row.account_id != 111));
    }

    #[test]
    fn neigung_follows_largest_soul_share() {
        let catalog = catalog();
        assert_eq!(
            neigung_bucket(&[100, 101], &catalog).as_deref(),
            Some("weapon")
        );
        assert_eq!(
            neigung_bucket(&[200, 200, 100], &catalog).as_deref(),
            Some("spirit")
        );
        assert_eq!(neigung_bucket(&[], &catalog), None);
    }

    fn frozen_catalog() -> Catalog {
        let raw: Value =
            serde_json::from_str(include_str!("../testdata/catalog_sample.json")).unwrap();
        Catalog::from_payloads(&raw["items"], &raw["heroes"]).unwrap()
    }

    #[test]
    fn frozen_real_payload_parses_into_rows() {
        let matches: Value =
            serde_json::from_str(include_str!("../testdata/metadata_sample.json")).unwrap();
        let catalog = frozen_catalog();
        let rows: Vec<PlayerMatchRow> = matches
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|value| match_to_rows(value, &catalog))
            .collect();
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|row| !row.items.is_empty()));
        assert!(rows
            .iter()
            .all(|row| row.items.len() == row.net_worth_rank.len()));
        assert!(rows.iter().any(|row| !row.ability_points.is_empty()));
        assert!(rows
            .iter()
            .any(|row| row.imbue_targets.iter().any(|target| *target != 0)));
        for row in &rows {
            for rank in &row.net_worth_rank {
                assert!((0..=4).contains(rank));
            }
        }
    }
}
