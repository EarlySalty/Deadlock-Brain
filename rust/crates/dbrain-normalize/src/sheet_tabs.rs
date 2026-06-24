use std::collections::BTreeMap;

use rusqlite::{params, params_from_iter, types::Value as SqlValue, Connection};
use serde_json::{json, Value};

use crate::util::{
    build_hero_index, json_string, normalize_alias, parse_bool, parse_float, parse_int,
    payload_values, row_number, title_case, value_to_string, SHEET_SOURCE,
};
use crate::Result;

const DEDICATED_TABS: &[&str] = &[
    "Heroes",
    "Hero meta ranking",
    "Boons/AP",
    "raw_hero_data",
    "raw_items_and_abilities",
    "shopBonuses",
];

const RANKING_COLUMN_MAP: &[(&str, &str)] = &[
    ("carry", "carry"),
    ("crowd control", "crowd_control"),
    ("disengage", "disengage"),
    ("early", "early"),
    ("engage", "engage"),
    ("frontline", "frontline"),
    ("late", "late"),
    ("mid", "mid"),
    ("mid contest", "mid_contest"),
    ("mobility", "mobility"),
    ("nuke (phys)", "nuke_phys"),
    ("nuke (spirit)", "nuke_spirit"),
    ("pick", "pick"),
    ("poke", "poke"),
    ("support", "support"),
    ("wave clear", "wave_clear"),
    ("sustain dps (physical)", "sustain_dps_phys"),
    ("sustain dps (spirit)", "sustain_dps_spirit"),
    ("average rank", "average_rank"),
];

const RAW_HERO_COLUMN_MAP: &[(&str, &str)] = &[
    ("move speed", "move_speed"),
    ("sprint speed", "sprint_speed"),
    ("crouch speed", "crouch_speed"),
    ("move accel", "move_accel"),
    ("light melee dmg", "light_melee_dmg"),
    ("heavy melee dmg", "heavy_melee_dmg"),
    ("max hp", "max_hp"),
    ("base stamina", "base_stamina"),
    ("stam regen per second", "stam_regen"),
    ("hp regen", "hp_regen"),
    ("base health", "base_health"),
    ("gun growth", "gun_growth"),
    ("alt gun growth", "alt_gun_growth"),
    ("hp per boon", "hp_per_boon"),
    ("melee gain", "melee_gain"),
    ("spirit per boon", "spirit_per_boon"),
];

const HEROES_STATS_MAP: &[(&str, &str)] = &[
    ("Base HP", "base_hp"),
    ("Base Move Speed", "base_move_speed"),
    ("Base Sprint", "base_sprint"),
    ("Base Stamina", "base_stamina"),
    ("Base Regen", "base_regen"),
    ("Base Ammo", "base_ammo"),
    ("Pellets", "pellets"),
    ("Alt Fire Pellets", "alt_fire_pellets"),
    ("Base Bullet Dmg", "base_bullet_dmg"),
    ("Base Fire Rate", "base_fire_rate"),
    ("Base DPS", "base_dps"),
    ("Max Gun DPS", "max_gun_dps"),
    ("Max Gun Damage", "max_gun_damage"),
    ("DPM", "dpm"),
    ("Max DPM", "max_dpm"),
    ("Falloff Range Min", "falloff_range_min"),
    ("Falloff Range Max", "falloff_range_max"),
    ("HP Gain", "hp_gain"),
    ("Dmg Gain", "dmg_gain"),
    ("Spirit Gain", "spirit_gain"),
    ("Spirit Bonus", "spirit_bonus"),
    ("Spirit Bonus 2", "spirit_bonus_2"),
    ("Spirit Ratio", "spirit_ratio"),
    ("Spirit Ratio 2", "spirit_ratio_2"),
    ("Spirit Scaling", "spirit_scaling"),
    ("Spirit Scaling 2", "spirit_scaling_2"),
    ("aggregate growth %", "aggregate_growth_pct"),
    ("dps % growth increase", "dps_growth_pct"),
    ("hp % growth increase", "hp_growth_pct"),
    ("melee ratio", "melee_ratio"),
    ("total bullet ratio", "total_bullet_ratio"),
    ("total spirit ratio", "total_spirit_ratio"),
    ("Max Level HP", "max_level_hp"),
];

#[derive(Debug, Clone)]
struct SheetRow {
    id: i64,
    canonical_name: Option<String>,
    payload_hash: String,
    payload_json: String,
}

pub fn normalize_sheet_tabs(conn: &Connection, rebuild: bool) -> Result<Value> {
    if rebuild {
        for table in [
            "sheet_hero_rankings",
            "sheet_boons_ap",
            "sheet_raw_heroes",
            "sheet_heroes_stats",
            "sheet_items",
            "sheet_shop_bonuses",
            "sheet_tab_rows",
        ] {
            let sql = format!("DELETE FROM {table}");
            let _ = conn.execute(&sql, [])?;
        }
    }

    let hero_index = build_hero_index(conn, false)?;
    Ok(json!({
        "sheet_hero_rankings": normalize_hero_rankings(conn, &hero_index)?,
        "sheet_boons_ap": normalize_boons_ap(conn)?,
        "sheet_raw_heroes": normalize_raw_heroes(conn, &hero_index)?,
        "sheet_heroes_stats": normalize_heroes_stats(conn, &hero_index)?,
        "sheet_items": normalize_items(conn)?,
        "sheet_shop_bonuses": normalize_shop_bonuses(conn)?,
        "sheet_tab_rows": normalize_freeform_tabs(conn)?,
    }))
}

fn normalize_heroes_stats(conn: &Connection, hero_index: &BTreeMapLike) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("Heroes"), false)?;
    let mut inserted = 0_i64;
    let mut columns = vec![
        "snapshot_id",
        "entity_id",
        "hero_name",
        "alt_fire_type",
        "hero_labs",
    ];
    columns.extend(HEROES_STATS_MAP.iter().map(|(_, column)| *column));
    columns.extend(["payload_hash", "created_at", "updated_at"]);
    let update_columns = columns
        .iter()
        .copied()
        .filter(|column| *column != "snapshot_id" && *column != "created_at")
        .collect::<Vec<_>>();
    let sql = upsert_sql("sheet_heroes_stats", &columns, &update_columns);

    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let hero_name = value_to_string(values.get("Hero Name")).trim().to_string();
        if hero_name.is_empty() || matches!(hero_name.to_lowercase().as_str(), "hero name" | "hero labs") {
            continue;
        }
        let now = crate::util::now()?;
        let mut sql_values = vec![
            SqlValue::Integer(row.id),
            opt_i64(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlValue::Text(hero_name.clone()),
            opt_string(nonempty(value_to_string(values.get("Alt Fire Type/Dmg")))),
            opt_string(nonempty(value_to_string(values.get("Hero Labs")))),
        ];
        for (label, _) in HEROES_STATS_MAP {
            sql_values.push(opt_f64(parse_float(&value_to_string(values.get(*label)))));
        }
        sql_values.push(SqlValue::Text(row.payload_hash.clone()));
        sql_values.push(SqlValue::Integer(now));
        sql_values.push(SqlValue::Integer(now));
        conn.execute(&sql, params_from_iter(sql_values))?;
        inserted += 1;
    }

    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

fn normalize_items(conn: &Connection) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("raw_items_and_abilities"), false)?;
    let mut inserted = 0_i64;
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let code_name = value_to_string(values.get("code name")).trim().to_string();
        let game_name = value_to_string(values.get("game name")).trim().to_string();
        let item_id = parse_int(&first_nonempty([
            value_to_string(values.get("id")),
            value_to_string(values.get("hero")),
        ]));
        if code_name.is_empty() && game_name.is_empty() {
            continue;
        }
        let canonical = if game_name.is_empty() { code_name.clone() } else { game_name.clone() };
        let now = crate::util::now()?;
        conn.execute(
            r#"
            INSERT INTO sheet_items(snapshot_id, item_id, code_name, game_name, canonical_name, created_at, updated_at)
            VALUES(?1,?2,?3,?4,?5,?6,?7)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              item_id=excluded.item_id, code_name=excluded.code_name,
              game_name=excluded.game_name, canonical_name=excluded.canonical_name,
              updated_at=excluded.updated_at
            "#,
            params![row.id, item_id, code_name, game_name, canonical, now, now],
        )?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

fn normalize_shop_bonuses(conn: &Connection) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("shopBonuses"), false)?;
    let mut inserted = 0_i64;
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let souls = parse_int(&first_nonempty([
            value_to_string(values.get("Column 1")),
            value_to_string(values.get("Column 7")),
        ]));
        if souls.is_none_or(|value| value <= 0) {
            continue;
        }
        let weapon = parse_int(&value_to_string(values.get("Weapon")));
        let spirit = parse_int(&value_to_string(values.get("Spirit")));
        let vitality = parse_int(&value_to_string(values.get("Vitality")));
        if weapon.is_none() && spirit.is_none() && vitality.is_none() {
            continue;
        }
        let inc_pct = parse_float(&value_to_string(values.get("inc from previous")));
        let now = crate::util::now()?;
        conn.execute(
            r#"
            INSERT INTO sheet_shop_bonuses(snapshot_id, souls_cost, weapon, spirit, vitality, inc_from_prev_pct, created_at, updated_at)
            VALUES(?1,?2,?3,?4,?5,?6,?7,?8)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              souls_cost=excluded.souls_cost, weapon=excluded.weapon,
              spirit=excluded.spirit, vitality=excluded.vitality,
              inc_from_prev_pct=excluded.inc_from_prev_pct, updated_at=excluded.updated_at
            "#,
            params![row.id, souls, weapon, spirit, vitality, inc_pct, now, now],
        )?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

fn normalize_freeform_tabs(conn: &Connection) -> Result<Value> {
    let rows = load_sheet_rows(conn, None, true)?;
    let mut counts_by_tab: BTreeMap<String, i64> = BTreeMap::new();
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let tab_name = value_to_string(payload.get("sheet_name")).trim().to_string();
        if tab_name.is_empty() {
            continue;
        }
        let gid = value_to_string(payload.get("gid")).trim().to_string();
        let canonical_name = row
            .canonical_name
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string);
        let row_json = payload
            .get("values")
            .and_then(Value::as_object)
            .map(|values| json_string(&Value::Object(values.clone())))
            .transpose()?
            .unwrap_or_else(|| "{}".to_string());
        let now = crate::util::now()?;
        conn.execute(
            r#"
            INSERT INTO sheet_tab_rows(snapshot_id, tab_name, gid, row_number, canonical_name, row_json, created_at, updated_at)
            VALUES(?1,?2,?3,?4,?5,?6,?7,?8)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              tab_name=excluded.tab_name, gid=excluded.gid, row_number=excluded.row_number,
              canonical_name=excluded.canonical_name, row_json=excluded.row_json,
              updated_at=excluded.updated_at
            "#,
            params![
                row.id,
                tab_name,
                gid,
                row_number(payload.get("row_number")),
                canonical_name,
                row_json,
                now,
                now,
            ],
        )?;
        *counts_by_tab.entry(tab_name).or_default() += 1;
    }
    let total: i64 = counts_by_tab.values().sum();
    Ok(json!({"total": total, "by_tab": counts_by_tab}))
}

fn normalize_hero_rankings(conn: &Connection, hero_index: &BTreeMapLike) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("Hero meta ranking"), false)?;
    let mut inserted = 0_i64;
    let mut columns = vec!["snapshot_id", "entity_id", "hero_name"];
    columns.extend(RANKING_COLUMN_MAP.iter().map(|(_, column)| *column));
    columns.extend(["payload_hash", "created_at", "updated_at"]);
    let update_columns = columns
        .iter()
        .copied()
        .filter(|column| *column != "snapshot_id" && *column != "created_at")
        .collect::<Vec<_>>();
    let sql = upsert_sql("sheet_hero_rankings", &columns, &update_columns);

    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let hero_name = first_nonempty([
            value_to_string(values.get("hero name")),
            value_to_string(values.get("Hero Name")),
        ]);
        if hero_name.is_empty() {
            continue;
        }
        let now = crate::util::now()?;
        let mut sql_values = vec![
            SqlValue::Integer(row.id),
            opt_i64(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlValue::Text(hero_name.clone()),
        ];
        for (label, _) in RANKING_COLUMN_MAP {
            let title = title_case(label);
            sql_values.push(opt_f64(parse_float(&first_nonempty([
                value_to_string(values.get(*label)),
                value_to_string(values.get(title.as_str())),
            ]))));
        }
        sql_values.push(SqlValue::Text(row.payload_hash.clone()));
        sql_values.push(SqlValue::Integer(now));
        sql_values.push(SqlValue::Integer(now));
        conn.execute(&sql, params_from_iter(sql_values))?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

fn normalize_boons_ap(conn: &Connection) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("Boons/AP"), false)?;
    let mut inserted = 0_i64;
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let souls = parse_int(&value_to_string(values.get("Souls")));
        if souls.is_none_or(|value| value <= 0) {
            continue;
        }
        let boons = parse_int(&value_to_string(values.get("Boons")));
        let ap = parse_int(&value_to_string(values.get("AP")));
        let note = nonempty(value_to_string(values.get("Column 4")));
        let now = crate::util::now()?;
        conn.execute(
            r#"
            INSERT INTO sheet_boons_ap(snapshot_id, souls, boons, ap, note, created_at, updated_at)
            VALUES(?1,?2,?3,?4,?5,?6,?7)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              souls=excluded.souls, boons=excluded.boons, ap=excluded.ap,
              note=excluded.note, updated_at=excluded.updated_at
            "#,
            params![row.id, souls, boons, ap, note, now, now],
        )?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

fn normalize_raw_heroes(conn: &Connection, hero_index: &BTreeMapLike) -> Result<Value> {
    let rows = load_sheet_rows(conn, Some("raw_hero_data"), false)?;
    let mut inserted = 0_i64;
    let mut columns = vec!["snapshot_id", "entity_id", "hero_name", "hero_id", "disabled"];
    columns.extend(RAW_HERO_COLUMN_MAP.iter().map(|(_, column)| *column));
    columns.extend(["payload_hash", "created_at", "updated_at"]);
    let update_columns = columns
        .iter()
        .copied()
        .filter(|column| *column != "snapshot_id" && *column != "created_at")
        .collect::<Vec<_>>();
    let sql = upsert_sql("sheet_raw_heroes", &columns, &update_columns);

    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let hero_name = value_to_string(values.get("name")).trim().to_string();
        if hero_name.is_empty() {
            continue;
        }
        let now = crate::util::now()?;
        let mut sql_values = vec![
            SqlValue::Integer(row.id),
            opt_i64(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlValue::Text(hero_name.clone()),
            opt_i64(parse_int(&value_to_string(values.get("id")))),
            SqlValue::Integer(if parse_bool(&first_nonempty([
                value_to_string(values.get("disabled")),
                "FALSE".to_string(),
            ])) { 1 } else { 0 }),
        ];
        for (label, _) in RAW_HERO_COLUMN_MAP {
            sql_values.push(opt_f64(parse_float(&value_to_string(values.get(*label)))));
        }
        sql_values.push(SqlValue::Text(row.payload_hash.clone()));
        sql_values.push(SqlValue::Integer(now));
        sql_values.push(SqlValue::Integer(now));
        conn.execute(&sql, params_from_iter(sql_values))?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

type BTreeMapLike = std::collections::HashMap<String, i64>;

fn load_sheet_rows(conn: &Connection, sheet: Option<&str>, freeform: bool) -> Result<Vec<SheetRow>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, canonical_name, payload_hash, payload_json
        FROM entity_snapshots
        WHERE source=?1
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([SHEET_SOURCE], |row| {
        Ok(SheetRow {
            id: row.get("id")?,
            canonical_name: row.get("canonical_name")?,
            payload_hash: row.get("payload_hash")?,
            payload_json: row.get("payload_json")?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        let row = row?;
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let name = value_to_string(payload.get("sheet_name")).trim().to_string();
        let include = if freeform {
            !name.is_empty() && !DEDICATED_TABS.contains(&name.as_str())
        } else {
            sheet == Some(name.as_str())
        };
        if include {
            out.push(row);
        }
    }
    Ok(out)
}

fn parse_payload(payload_json: &str) -> Option<Value> {
    serde_json::from_str::<Value>(payload_json).ok().filter(Value::is_object)
}

fn upsert_sql(table: &str, columns: &[&str], update_columns: &[&str]) -> String {
    let placeholders = (1..=columns.len())
        .map(|index| format!("?{index}"))
        .collect::<Vec<_>>()
        .join(",");
    let update_set = update_columns
        .iter()
        .map(|column| format!("{column}=excluded.{column}"))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "INSERT INTO {table}({}) VALUES({}) ON CONFLICT(snapshot_id) DO UPDATE SET {}",
        columns.join(","),
        placeholders,
        update_set,
    )
}

fn opt_i64(value: Option<i64>) -> SqlValue {
    value.map(SqlValue::Integer).unwrap_or(SqlValue::Null)
}

fn opt_f64(value: Option<f64>) -> SqlValue {
    value.map(SqlValue::Real).unwrap_or(SqlValue::Null)
}

fn opt_string(value: Option<String>) -> SqlValue {
    value.map(SqlValue::Text).unwrap_or(SqlValue::Null)
}

fn nonempty(value: String) -> Option<String> {
    let text = value.trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn first_nonempty<const N: usize>(values: [String; N]) -> String {
    values
        .into_iter()
        .map(|value| value.trim().to_string())
        .find(|value| !value.is_empty())
        .unwrap_or_default()
}
