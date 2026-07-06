use std::collections::BTreeMap;

use serde_json::{json, Value};
use sqlx::PgPool;

use crate::util::{
    bind_params, build_hero_index, json_string, normalize_alias, parse_bool, parse_float, parse_int,
    payload_values, row_number, title_case, value_to_string, SqlParam, SHEET_SOURCE,
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
    legacy_snapshot_id: i64,
    canonical_name: Option<String>,
    payload_hash: String,
    payload_json: String,
}

pub async fn normalize_sheet_tabs(pool: &PgPool, rebuild: bool) -> Result<Value> {
    if rebuild {
        sqlx::query!("DELETE FROM brain.sheet_hero_rankings").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_boons_ap").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_raw_heroes").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_heroes_stats").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_items").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_shop_bonuses").execute(pool).await?;
        sqlx::query!("DELETE FROM brain.sheet_tab_rows").execute(pool).await?;
    }

    let hero_index = build_hero_index(pool, false).await?;
    Ok(json!({
        "sheet_hero_rankings": normalize_hero_rankings(pool, &hero_index).await?,
        "sheet_boons_ap": normalize_boons_ap(pool).await?,
        "sheet_raw_heroes": normalize_raw_heroes(pool, &hero_index).await?,
        "sheet_heroes_stats": normalize_heroes_stats(pool, &hero_index).await?,
        "sheet_items": normalize_items(pool).await?,
        "sheet_shop_bonuses": normalize_shop_bonuses(pool).await?,
        "sheet_tab_rows": normalize_freeform_tabs(pool).await?,
    }))
}

async fn normalize_heroes_stats(pool: &PgPool, hero_index: &HeroIndex) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("Heroes"), false).await?;
    let mut inserted = 0_i64;
    let mut columns = vec![
        "snapshot_id",
        "legacy_snapshot_id",
        "entity_id",
        "hero_name",
        "alt_fire_type",
        "hero_labs",
    ];
    columns.extend(HEROES_STATS_MAP.iter().map(|(_, column)| *column));
    columns.push("payload_hash");
    let sql = upsert_sql("sheet_heroes_stats", &columns);

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
        let mut params = vec![
            SqlParam::Int(row.id),
            SqlParam::Int(row.legacy_snapshot_id),
            SqlParam::IntOpt(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlParam::Text(hero_name.clone()),
            SqlParam::TextOpt(nonempty(value_to_string(values.get("Alt Fire Type/Dmg")))),
            SqlParam::TextOpt(nonempty(value_to_string(values.get("Hero Labs")))),
        ];
        for (label, _) in HEROES_STATS_MAP {
            params.push(SqlParam::FloatOpt(parse_float(&value_to_string(values.get(*label)))));
        }
        params.push(SqlParam::Text(row.payload_hash.clone()));
        bind_params(sqlx::query(&sql), &params).execute(pool).await?;
        inserted += 1;
    }

    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

async fn normalize_items(pool: &PgPool) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("raw_items_and_abilities"), false).await?;
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
        sqlx::query!(
            r#"
            INSERT INTO brain.sheet_items(
              snapshot_id, legacy_snapshot_id, item_id, code_name, game_name, canonical_name,
              created_at, updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, now(), now())
            ON CONFLICT (snapshot_id) DO UPDATE SET
              item_id = EXCLUDED.item_id, code_name = EXCLUDED.code_name,
              game_name = EXCLUDED.game_name, canonical_name = EXCLUDED.canonical_name,
              updated_at = now()
            "#,
            row.id,
            row.legacy_snapshot_id,
            item_id,
            code_name,
            game_name,
            canonical,
        )
        .execute(pool)
        .await?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

async fn normalize_shop_bonuses(pool: &PgPool) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("shopBonuses"), false).await?;
    let mut inserted = 0_i64;
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let Some(souls_cost) = parse_int(&first_nonempty([
            value_to_string(values.get("Column 1")),
            value_to_string(values.get("Column 7")),
        ]))
        .filter(|value| *value > 0) else {
            continue;
        };
        let weapon = parse_int(&value_to_string(values.get("Weapon")));
        let spirit = parse_int(&value_to_string(values.get("Spirit")));
        let vitality = parse_int(&value_to_string(values.get("Vitality")));
        if weapon.is_none() && spirit.is_none() && vitality.is_none() {
            continue;
        }
        let inc_pct = parse_float(&value_to_string(values.get("inc from previous")));
        sqlx::query!(
            r#"
            INSERT INTO brain.sheet_shop_bonuses(
              snapshot_id, legacy_snapshot_id, souls_cost, weapon, spirit, vitality,
              inc_from_prev_pct, created_at, updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, now(), now())
            ON CONFLICT (snapshot_id) DO UPDATE SET
              souls_cost = EXCLUDED.souls_cost, weapon = EXCLUDED.weapon,
              spirit = EXCLUDED.spirit, vitality = EXCLUDED.vitality,
              inc_from_prev_pct = EXCLUDED.inc_from_prev_pct, updated_at = now()
            "#,
            row.id,
            row.legacy_snapshot_id,
            souls_cost,
            weapon,
            spirit,
            vitality,
            inc_pct,
        )
        .execute(pool)
        .await?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

async fn normalize_freeform_tabs(pool: &PgPool) -> Result<Value> {
    let rows = load_sheet_rows(pool, None, true).await?;
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
        sqlx::query!(
            r#"
            INSERT INTO brain.sheet_tab_rows(
              snapshot_id, legacy_snapshot_id, tab_name, gid, row_number, canonical_name,
              row_data, created_at, updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7::text::jsonb, now(), now())
            ON CONFLICT (snapshot_id) DO UPDATE SET
              tab_name = EXCLUDED.tab_name, gid = EXCLUDED.gid, row_number = EXCLUDED.row_number,
              canonical_name = EXCLUDED.canonical_name, row_data = EXCLUDED.row_data,
              updated_at = now()
            "#,
            row.id,
            row.legacy_snapshot_id,
            tab_name,
            gid,
            row_number(payload.get("row_number")),
            canonical_name,
            row_json,
        )
        .execute(pool)
        .await?;
        *counts_by_tab.entry(tab_name).or_default() += 1;
    }
    let total: i64 = counts_by_tab.values().sum();
    Ok(json!({"total": total, "by_tab": counts_by_tab}))
}

async fn normalize_hero_rankings(pool: &PgPool, hero_index: &HeroIndex) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("Hero meta ranking"), false).await?;
    let mut inserted = 0_i64;
    let mut columns = vec!["snapshot_id", "legacy_snapshot_id", "entity_id", "hero_name"];
    columns.extend(RANKING_COLUMN_MAP.iter().map(|(_, column)| *column));
    columns.push("payload_hash");
    let sql = upsert_sql("sheet_hero_rankings", &columns);

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
        let mut params = vec![
            SqlParam::Int(row.id),
            SqlParam::Int(row.legacy_snapshot_id),
            SqlParam::IntOpt(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlParam::Text(hero_name.clone()),
        ];
        for (label, _) in RANKING_COLUMN_MAP {
            let title = title_case(label);
            params.push(SqlParam::FloatOpt(parse_float(&first_nonempty([
                value_to_string(values.get(*label)),
                value_to_string(values.get(title.as_str())),
            ]))));
        }
        params.push(SqlParam::Text(row.payload_hash.clone()));
        bind_params(sqlx::query(&sql), &params).execute(pool).await?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

async fn normalize_boons_ap(pool: &PgPool) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("Boons/AP"), false).await?;
    let mut inserted = 0_i64;
    for row in &rows {
        let Some(payload) = parse_payload(&row.payload_json) else {
            continue;
        };
        let Some(values) = payload_values(&payload) else {
            continue;
        };
        let Some(souls) = parse_int(&value_to_string(values.get("Souls"))).filter(|value| *value > 0)
        else {
            continue;
        };
        let boons = parse_int(&value_to_string(values.get("Boons")));
        let ap = parse_int(&value_to_string(values.get("AP")));
        let note = nonempty(value_to_string(values.get("Column 4")));
        sqlx::query!(
            r#"
            INSERT INTO brain.sheet_boons_ap(
              snapshot_id, legacy_snapshot_id, souls, boons, ap, note, created_at, updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, now(), now())
            ON CONFLICT (snapshot_id) DO UPDATE SET
              souls = EXCLUDED.souls, boons = EXCLUDED.boons, ap = EXCLUDED.ap,
              note = EXCLUDED.note, updated_at = now()
            "#,
            row.id,
            row.legacy_snapshot_id,
            souls,
            boons,
            ap,
            note,
        )
        .execute(pool)
        .await?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

async fn normalize_raw_heroes(pool: &PgPool, hero_index: &HeroIndex) -> Result<Value> {
    let rows = load_sheet_rows(pool, Some("raw_hero_data"), false).await?;
    let mut inserted = 0_i64;
    let mut columns = vec![
        "snapshot_id",
        "legacy_snapshot_id",
        "entity_id",
        "hero_name",
        "hero_id",
        "disabled",
    ];
    columns.extend(RAW_HERO_COLUMN_MAP.iter().map(|(_, column)| *column));
    columns.push("payload_hash");
    let sql = upsert_sql("sheet_raw_heroes", &columns);

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
        let disabled = parse_bool(&first_nonempty([
            value_to_string(values.get("disabled")),
            "FALSE".to_string(),
        ]));
        let mut params = vec![
            SqlParam::Int(row.id),
            SqlParam::Int(row.legacy_snapshot_id),
            SqlParam::IntOpt(hero_index.get(&normalize_alias(&hero_name)).copied()),
            SqlParam::Text(hero_name.clone()),
            SqlParam::IntOpt(parse_int(&value_to_string(values.get("id")))),
            SqlParam::Int(if disabled { 1 } else { 0 }),
        ];
        for (label, _) in RAW_HERO_COLUMN_MAP {
            params.push(SqlParam::FloatOpt(parse_float(&value_to_string(values.get(*label)))));
        }
        params.push(SqlParam::Text(row.payload_hash.clone()));
        bind_params(sqlx::query(&sql), &params).execute(pool).await?;
        inserted += 1;
    }
    Ok(json!({"snapshots": rows.len(), "inserted": inserted}))
}

type HeroIndex = std::collections::HashMap<String, i64>;

async fn load_sheet_rows(pool: &PgPool, sheet: Option<&str>, freeform: bool) -> Result<Vec<SheetRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!", COALESCE(legacy_sqlite_id, id) AS "legacy_snapshot_id!",
               canonical_name AS "canonical_name?", payload_hash AS "payload_hash!",
               payload::text AS "payload_json!"
        FROM brain.entity_snapshots
        WHERE source = $1
        ORDER BY id
        "#,
        SHEET_SOURCE,
    )
    .fetch_all(pool)
    .await?;
    let mut out = Vec::new();
    for row in rows {
        let row = SheetRow {
            id: row.id,
            legacy_snapshot_id: row.legacy_snapshot_id,
            canonical_name: row.canonical_name,
            payload_hash: row.payload_hash,
            payload_json: row.payload_json,
        };
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

/// Baut das Upsert-SQL fuer die Sheet-Tabellen mit fester Konflikt-Spalte
/// `snapshot_id`. Die Spaltenliste stammt aus compile-festen Konstanten-Maps,
/// wird aber zur Laufzeit zusammengesetzt (variable Statistik-Spalten), daher
/// `sqlx::query` + [`bind_params`] statt der `query!`-Makro-Form.
fn upsert_sql(table: &str, bound_columns: &[&str]) -> String {
    let placeholders = (1..=bound_columns.len())
        .map(|index| format!("${index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut update_set = bound_columns
        .iter()
        .filter(|column| **column != "snapshot_id")
        .map(|column| format!("{column} = EXCLUDED.{column}"))
        .collect::<Vec<_>>();
    update_set.push("updated_at = now()".to_string());
    format!(
        "INSERT INTO brain.{table}({}, created_at, updated_at) \
         VALUES({}, now(), now()) \
         ON CONFLICT (snapshot_id) DO UPDATE SET {}",
        bound_columns.join(", "),
        placeholders,
        update_set.join(", "),
    )
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
