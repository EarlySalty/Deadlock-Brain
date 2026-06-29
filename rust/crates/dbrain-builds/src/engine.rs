use std::{cmp::Ordering, collections::HashMap, env};

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    error::BuildEngineError,
    schema,
    util::{
        clamp_unit, normalize_name, now_epoch_seconds, winrate_pp, BRACKET_BADGE_80,
        PATCH_TAG_CURRENT,
    },
    BuildContext, BuildPath, BuildPathSummary, BuildPhase, FitFlag, ItemDossier,
};

const DEFAULT_MIN_MATCHES: i64 = 500;
const DEFAULT_MIN_PREVALENCE_BUILDS: i64 = 30;
const MAX_ITEMS_PER_PATH: usize = 18;
const MAX_ITEMS_PER_PHASE: usize = 6;

pub(crate) fn build_context(
    conn: &Connection,
    hero_query: &str,
    playstyle: Option<&str>,
) -> Result<BuildContext> {
    schema::ensure_schema(conn)?;
    let requested_playstyle = normalize_playstyle(playstyle)?;
    let hero_id = resolve_hero_id(conn, hero_query)?;
    let hero = load_hero(conn, hero_id)?;
    let rows = load_item_rows(conn, hero_id)?;
    if rows.is_empty() {
        return Err(BuildEngineError::MissingBuildData(hero_id).into());
    }

    let gates = SampleGates::from_env();
    let max_builds = rows
        .iter()
        .map(|row| row.prevalence_builds)
        .max()
        .unwrap_or(1)
        .max(1);
    let mut scored = rows
        .into_iter()
        .map(|row| ScoredItem {
            score: composite_score(&row, max_builds),
            confidence: confidence(&row, &gates).to_string(),
            row,
        })
        .collect::<Vec<_>>();

    scored.sort_by(compare_scored_items);
    let labels = ["spirit", "weapon", "tank"];
    let mut paths = labels
        .iter()
        .map(|label| build_path_candidate(label, &hero.archetype, &scored))
        .collect::<Vec<_>>();
    paths.sort_by(compare_paths);

    let primary_label = requested_playstyle.as_deref().unwrap_or_else(|| {
        paths
            .first()
            .map(|path| path.label.as_str())
            .unwrap_or("spirit")
    });
    let primary = paths
        .iter()
        .find(|path| path.label == primary_label)
        .cloned()
        .or_else(|| paths.first().cloned())
        .ok_or(BuildEngineError::MissingBuildData(hero_id))?;

    let selected_items = select_path_items(primary.label.as_str(), &scored);
    let name_lookup = item_name_lookup(&selected_items);
    let primary_path = BuildPath {
        label: primary.label.clone(),
        winrate: primary.winrate,
        sample_matches: primary.sample_matches,
        phases: build_phases(
            conn,
            hero_id,
            &hero.archetype,
            &primary.label,
            &selected_items,
            &name_lookup,
        )?,
    };
    let alternative_paths = paths
        .into_iter()
        .filter(|path| path.label != primary_path.label)
        .map(|path| BuildPathSummary {
            label: path.label,
            winrate: path.winrate,
            sample_matches: path.sample_matches,
        })
        .collect();

    Ok(BuildContext {
        hero_id,
        hero_name: hero.name,
        hero_archetype: hero.archetype,
        playstyle: requested_playstyle,
        primary_path,
        alternative_paths,
        ability_order: load_ability_order(conn, hero_id)?,
        generated_at: now_epoch_seconds()?,
    })
}

pub(crate) fn resolve_hero_id(conn: &Connection, hero_query: &str) -> Result<i64> {
    schema::ensure_schema(conn)?;
    if let Ok(hero_id) = hero_query.trim().parse::<i64>() {
        let found: Option<i64> = conn
            .query_row(
                "SELECT hero_id FROM hero_catalog WHERE hero_id=?1",
                [hero_id],
                |row| row.get(0),
            )
            .optional()?;
        return found.ok_or_else(|| BuildEngineError::HeroNotFound(hero_query.to_string()).into());
    }

    let query_norm = normalize_name(hero_query);
    if query_norm.is_empty() {
        return Err(BuildEngineError::HeroNotFound(hero_query.to_string()).into());
    }
    if let Some(hero_id) = resolve_from_entity_aliases(conn, &query_norm)? {
        return Ok(hero_id);
    }

    let mut statement = conn.prepare("SELECT hero_id, name FROM hero_catalog ORDER BY hero_id")?;
    let heroes = statement
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if let Some((hero_id, _)) = heroes
        .iter()
        .find(|(_, name)| normalize_name(name) == query_norm)
    {
        return Ok(*hero_id);
    }
    if let Some((hero_id, _)) = heroes
        .iter()
        .find(|(_, name)| normalize_name(name).starts_with(&query_norm))
    {
        return Ok(*hero_id);
    }
    if let Some((hero_id, _)) = heroes
        .iter()
        .find(|(_, name)| normalize_name(name).contains(&query_norm))
    {
        return Ok(*hero_id);
    }

    Err(BuildEngineError::HeroNotFound(hero_query.to_string()).into())
}

fn resolve_from_entity_aliases(conn: &Connection, query_norm: &str) -> Result<Option<i64>> {
    let has_entities: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('entities', 'entity_aliases')",
        [],
        |row| row.get(0),
    )?;
    if has_entities < 2 {
        return Ok(None);
    }

    let mut statement = conn.prepare(
        r#"
        SELECT e.primary_external_id, e.canonical_name
        FROM entity_aliases a
        JOIN entities e ON e.id = a.entity_id
        WHERE a.alias_norm=?1 AND e.entity_type='hero'
        ORDER BY e.id
        LIMIT 5
        "#,
    )?;
    let rows = statement
        .query_map([query_norm], |row| {
            Ok((row.get::<_, Option<String>>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    for (external_id, canonical_name) in rows {
        if let Some(hero_id) = external_id.and_then(|id| id.parse::<i64>().ok()) {
            let exists: Option<i64> = conn
                .query_row(
                    "SELECT hero_id FROM hero_catalog WHERE hero_id=?1",
                    [hero_id],
                    |row| row.get(0),
                )
                .optional()?;
            if exists.is_some() {
                return Ok(Some(hero_id));
            }
        }
        let canonical_norm = normalize_name(&canonical_name);
        if let Some(hero_id) = conn
            .query_row(
                "SELECT hero_id FROM hero_catalog WHERE lower(name)=lower(?1)",
                [canonical_name],
                |row| row.get(0),
            )
            .optional()?
        {
            return Ok(Some(hero_id));
        }
        if canonical_norm == query_norm {
            continue;
        }
    }
    Ok(None)
}

#[derive(Debug, Clone)]
struct HeroRow {
    name: String,
    archetype: String,
}

fn load_hero(conn: &Connection, hero_id: i64) -> Result<HeroRow> {
    conn.query_row(
        "SELECT name, archetype FROM hero_catalog WHERE hero_id=?1",
        [hero_id],
        |row| {
            Ok(HeroRow {
                name: row.get(0)?,
                archetype: row.get(1)?,
            })
        },
    )
    .optional()?
    .ok_or_else(|| BuildEngineError::HeroNotFound(hero_id.to_string()).into())
}

#[derive(Debug, Clone)]
struct ItemRow {
    item_id: i64,
    name: String,
    slot_type: String,
    tier: i64,
    defense_kind: Vec<String>,
    damage_axis: String,
    prevalence_builds: i64,
    wins: i64,
    losses: i64,
    matches: i64,
    avg_buy_time_relative: Option<f64>,
    lift_pp: Option<f64>,
}

fn load_item_rows(conn: &Connection, hero_id: i64) -> Result<Vec<ItemRow>> {
    let mut statement = conn.prepare(
        r#"
        SELECT
          i.item_id, i.name, i.slot_type, i.tier, i.defense_kind_json, i.damage_axis,
          s.prevalence_builds, s.wins, s.losses, s.matches, s.avg_buy_time_relative, s.lift_pp
        FROM hero_item_stats s
        JOIN item_catalog i ON i.item_id=s.item_id
        WHERE s.hero_id=?1 AND s.bracket=?2 AND s.patch_tag=?3
        ORDER BY s.prevalence_builds DESC, s.matches DESC, i.item_id ASC
        "#,
    )?;
    let rows = statement
        .query_map(
            params![hero_id, BRACKET_BADGE_80, PATCH_TAG_CURRENT],
            |row| {
                let defense_json: String = row.get(4)?;
                let defense_kind =
                    serde_json::from_str::<Vec<String>>(&defense_json).unwrap_or_default();
                Ok(ItemRow {
                    item_id: row.get(0)?,
                    name: row.get(1)?,
                    slot_type: row.get(2)?,
                    tier: row.get(3)?,
                    defense_kind,
                    damage_axis: row.get(5)?,
                    prevalence_builds: row.get(6)?,
                    wins: row.get(7)?,
                    losses: row.get(8)?,
                    matches: row.get(9)?,
                    avg_buy_time_relative: row.get(10)?,
                    lift_pp: row.get(11)?,
                })
            },
        )?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows)
}

#[derive(Debug, Clone)]
struct ScoredItem {
    row: ItemRow,
    score: f64,
    confidence: String,
}

#[derive(Debug, Clone)]
struct PathCandidate {
    label: String,
    score: f64,
    winrate: Option<f64>,
    sample_matches: i64,
}

#[derive(Debug, Clone, Copy)]
struct SampleGates {
    min_matches: i64,
    min_prevalence_builds: i64,
}

impl SampleGates {
    fn from_env() -> Self {
        Self {
            min_matches: env_i64("DBRAIN_BUILDS_MIN_MATCHES", DEFAULT_MIN_MATCHES),
            min_prevalence_builds: env_i64(
                "DBRAIN_BUILDS_MIN_PREVALENCE_BUILDS",
                DEFAULT_MIN_PREVALENCE_BUILDS,
            ),
        }
    }
}

fn env_i64(name: &str, default: i64) -> i64 {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .filter(|value| *value >= 0)
        .unwrap_or(default)
}

fn confidence(row: &ItemRow, gates: &SampleGates) -> &'static str {
    if row.matches < gates.min_matches || row.prevalence_builds < gates.min_prevalence_builds {
        "low"
    } else if row.matches < gates.min_matches * 4
        || row.prevalence_builds < gates.min_prevalence_builds * 4
        || row.lift_pp.is_none()
    {
        "medium"
    } else {
        "high"
    }
}

fn composite_score(row: &ItemRow, max_builds: i64) -> f64 {
    let prevalence =
        (row.prevalence_builds.max(0) as f64 + 1.0).ln() / (max_builds.max(1) as f64 + 1.0).ln();
    let winrate = winrate_pp(row.wins, row.losses, row.matches)
        .map(|value| clamp_unit((value - 47.0) / 14.0))
        .unwrap_or(0.35);
    let lift = row
        .lift_pp
        .map(|value| clamp_unit((value + 3.0) / 8.0))
        .unwrap_or(0.45);
    (prevalence * 0.50) + (winrate * 0.30) + (lift * 0.20)
}

fn build_path_candidate(label: &str, hero_archetype: &str, scored: &[ScoredItem]) -> PathCandidate {
    let items = select_path_items(label, scored);
    let score = items.iter().map(|item| item.score).sum::<f64>();
    let wins = items.iter().map(|item| item.row.wins).sum::<i64>();
    let losses = items.iter().map(|item| item.row.losses).sum::<i64>();
    let matches = items.iter().map(|item| item.row.matches).sum::<i64>();
    let archetype_bonus = if label == "tank" && matches!(hero_archetype, "tank" | "bruiser") {
        0.10
    } else {
        0.0
    };
    PathCandidate {
        label: label.to_string(),
        score: score + archetype_bonus,
        winrate: winrate_pp(wins, losses, matches),
        sample_matches: matches,
    }
}

fn select_path_items<'a>(label: &str, scored: &'a [ScoredItem]) -> Vec<&'a ScoredItem> {
    let mut phase_counts = HashMap::<&'static str, usize>::new();
    let mut selected = Vec::new();
    for item in scored {
        if item.confidence == "low" || !fits_path(label, &item.row) {
            continue;
        }
        let phase = buy_phase(&item.row);
        let count = phase_counts.entry(phase).or_insert(0);
        if *count >= MAX_ITEMS_PER_PHASE {
            continue;
        }
        selected.push(item);
        *count += 1;
        if selected.len() >= MAX_ITEMS_PER_PATH {
            break;
        }
    }
    selected
}

fn fits_path(label: &str, row: &ItemRow) -> bool {
    match label {
        "spirit" => {
            row.slot_type != "weapon"
                && matches!(row.damage_axis.as_str(), "spirit" | "hybrid" | "utility")
        }
        "weapon" => {
            matches!(row.damage_axis.as_str(), "weapon" | "hybrid")
                || (row.slot_type == "vitality" && row.damage_axis == "utility")
        }
        "tank" => row.slot_type == "vitality" || !row.defense_kind.is_empty(),
        _ => false,
    }
}

fn buy_phase(row: &ItemRow) -> &'static str {
    if let Some(relative) = row.avg_buy_time_relative {
        if relative < 33.0 {
            "early"
        } else if relative < 66.0 {
            "mid"
        } else {
            "late"
        }
    } else if row.tier <= 2 {
        "early"
    } else if row.tier == 3 {
        "mid"
    } else {
        "late"
    }
}

fn build_phases(
    conn: &Connection,
    hero_id: i64,
    hero_archetype: &str,
    path_label: &str,
    selected_items: &[&ScoredItem],
    name_lookup: &HashMap<i64, String>,
) -> Result<Vec<BuildPhase>> {
    let mut phases = Vec::new();
    for phase in ["early", "mid", "late"] {
        let mut phase_items = selected_items
            .iter()
            .copied()
            .filter(|item| buy_phase(&item.row) == phase)
            .collect::<Vec<_>>();
        phase_items.sort_by(compare_buy_order);
        let mut dossiers = Vec::new();
        for item in phase_items {
            dossiers.push(build_dossier(
                conn,
                hero_id,
                hero_archetype,
                path_label,
                item,
                name_lookup,
            )?);
        }
        phases.push(BuildPhase {
            phase: phase.to_string(),
            items: dossiers,
        });
    }
    Ok(phases)
}

fn build_dossier(
    conn: &Connection,
    hero_id: i64,
    hero_archetype: &str,
    path_label: &str,
    item: &ScoredItem,
    name_lookup: &HashMap<i64, String>,
) -> Result<ItemDossier> {
    let row = &item.row;
    let buy_phase = buy_phase(row).to_string();
    Ok(ItemDossier {
        item_id: row.item_id,
        name: row.name.clone(),
        slot_type: row.slot_type.clone(),
        tier: row.tier,
        defense_kind: row.defense_kind.clone(),
        damage_axis: row.damage_axis.clone(),
        prevalence_builds: row.prevalence_builds,
        winrate: winrate_pp(row.wins, row.losses, row.matches),
        sample_matches: row.matches,
        lift_pp: row.lift_pp,
        buy_phase,
        synergy_with: synergy_names(conn, hero_id, row.item_id, name_lookup)?,
        fit_flags: fit_flags(hero_archetype, path_label, row, &item.confidence),
        confidence: item.confidence.clone(),
    })
}

fn fit_flags(
    hero_archetype: &str,
    path_label: &str,
    row: &ItemRow,
    confidence: &str,
) -> Vec<FitFlag> {
    let mut flags = Vec::new();
    if matches!(hero_archetype, "tank" | "bruiser")
        && row.defense_kind.iter().any(|kind| kind == "flat_shield")
    {
        flags.push(FitFlag {
            code: "flat_shield_on_tank".to_string(),
            severity: "warning".to_string(),
            message_de: "Platzhalter".to_string(),
        });
    }
    if matches!(path_label, "weapon" | "spirit")
        && row.damage_axis != path_label
        && row.damage_axis != "hybrid"
    {
        flags.push(FitFlag {
            code: "axis_mismatch".to_string(),
            severity: "info".to_string(),
            message_de: "Platzhalter".to_string(),
        });
    }
    if confidence == "low" {
        flags.push(FitFlag {
            code: "low_sample".to_string(),
            severity: "warning".to_string(),
            message_de: "Platzhalter".to_string(),
        });
    }
    flags
}

fn synergy_names(
    conn: &Connection,
    hero_id: i64,
    item_id: i64,
    name_lookup: &HashMap<i64, String>,
) -> Result<Vec<String>> {
    let mut statement = conn.prepare(
        r#"
        SELECT with_item_id
        FROM hero_item_synergies
        WHERE hero_id=?1 AND item_id=?2 AND patch_tag=?3
        ORDER BY matches DESC, with_item_id ASC
        LIMIT 8
        "#,
    )?;
    let ids = statement
        .query_map(params![hero_id, item_id, PATCH_TAG_CURRENT], |row| {
            row.get::<_, i64>(0)
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(ids
        .into_iter()
        .filter_map(|id| name_lookup.get(&id).cloned())
        .take(3)
        .collect())
}

fn item_name_lookup(selected_items: &[&ScoredItem]) -> HashMap<i64, String> {
    selected_items
        .iter()
        .map(|item| (item.row.item_id, item.row.name.clone()))
        .collect()
}

fn load_ability_order(conn: &Connection, hero_id: i64) -> Result<Option<Vec<i64>>> {
    let abilities_json: Option<String> = conn
        .query_row(
            r#"
            SELECT abilities_json
            FROM hero_ability_orders
            WHERE hero_id=?1 AND bracket=?2 AND patch_tag=?3
            "#,
            params![hero_id, BRACKET_BADGE_80, PATCH_TAG_CURRENT],
            |row| row.get(0),
        )
        .optional()?;
    abilities_json
        .map(|json| serde_json::from_str::<Vec<i64>>(&json).map_err(Into::into))
        .transpose()
}

fn normalize_playstyle(playstyle: Option<&str>) -> Result<Option<String>> {
    let Some(playstyle) = playstyle else {
        return Ok(None);
    };
    let normalized = playstyle.trim().to_ascii_lowercase();
    if matches!(normalized.as_str(), "weapon" | "spirit" | "tank") {
        Ok(Some(normalized))
    } else {
        Err(BuildEngineError::InvalidPlaystyle(playstyle.to_string()).into())
    }
}

fn compare_scored_items(left: &ScoredItem, right: &ScoredItem) -> Ordering {
    right
        .score
        .total_cmp(&left.score)
        .then_with(|| right.row.prevalence_builds.cmp(&left.row.prevalence_builds))
        .then_with(|| right.row.matches.cmp(&left.row.matches))
        .then_with(|| left.row.item_id.cmp(&right.row.item_id))
}

fn compare_paths(left: &PathCandidate, right: &PathCandidate) -> Ordering {
    right
        .score
        .total_cmp(&left.score)
        .then_with(|| right.sample_matches.cmp(&left.sample_matches))
        .then_with(|| left.label.cmp(&right.label))
}

fn compare_buy_order(left: &&ScoredItem, right: &&ScoredItem) -> Ordering {
    left.row
        .avg_buy_time_relative
        .unwrap_or(phase_fallback_time(&left.row))
        .total_cmp(
            &right
                .row
                .avg_buy_time_relative
                .unwrap_or(phase_fallback_time(&right.row)),
        )
        .then_with(|| compare_scored_items(left, right))
}

fn phase_fallback_time(row: &ItemRow) -> f64 {
    match buy_phase(row) {
        "early" => 20.0,
        "mid" => 50.0,
        _ => 80.0,
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;
    use crate::sync::sync_fixture_payloads;

    #[test]
    fn mo_primary_path_is_deterministic_spirit_vitality() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        sync_fixture_payloads(&conn).expect("sync fixtures");

        let context = build_context(&conn, "Mo", None).expect("build context");

        assert_eq!(context.hero_id, 18);
        assert_eq!(context.hero_name, "Mo & Krill");
        assert_ne!(context.primary_path.label, "weapon");
        let primary_items = context
            .primary_path
            .phases
            .iter()
            .flat_map(|phase| phase.items.iter())
            .collect::<Vec<_>>();
        assert!(!primary_items.is_empty());
        assert!(primary_items
            .iter()
            .all(|item| matches!(item.slot_type.as_str(), "spirit" | "vitality")));
        assert!(!primary_items
            .iter()
            .any(|item| item.name == "Weapon Shielding"));
        assert!(!primary_items.iter().any(|item| item.name == "Lucky Shot"));
    }

    #[test]
    fn emitted_items_exist_in_catalog() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        sync_fixture_payloads(&conn).expect("sync fixtures");
        let context = build_context(&conn, "Mo", None).expect("build context");

        for item in context
            .primary_path
            .phases
            .iter()
            .flat_map(|phase| phase.items.iter())
        {
            let exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM item_catalog WHERE item_id=?1",
                    [item.item_id],
                    |row| row.get(0),
                )
                .expect("catalog lookup");
            assert_eq!(exists, 1, "missing item {}", item.item_id);
        }
    }

    #[test]
    fn playstyle_selects_requested_path() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        sync_fixture_payloads(&conn).expect("sync fixtures");
        let context = build_context(&conn, "Mo", Some("tank")).expect("build context");
        assert_eq!(context.playstyle.as_deref(), Some("tank"));
        assert_eq!(context.primary_path.label, "tank");
    }

    #[test]
    fn fit_flag_messages_are_placeholders() {
        let row = ItemRow {
            item_id: 1,
            name: "Shield".to_string(),
            slot_type: "vitality".to_string(),
            tier: 2,
            defense_kind: vec!["flat_shield".to_string()],
            damage_axis: "utility".to_string(),
            prevalence_builds: 1,
            wins: 1,
            losses: 1,
            matches: 2,
            avg_buy_time_relative: Some(10.0),
            lift_pp: None,
        };
        let flags = fit_flags("tank", "spirit", &row, "low");
        assert!(flags.iter().all(|flag| flag.message_de == "Platzhalter"));
    }

    #[test]
    fn json_contract_serializes() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        sync_fixture_payloads(&conn).expect("sync fixtures");
        let context = build_context(&conn, "Mo", None).expect("build context");
        let json = crate::util::json_string(&context).expect("serialize context");
        assert!(json.contains("\"hero_id\":18"));
    }
}
