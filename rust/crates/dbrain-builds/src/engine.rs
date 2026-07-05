use std::{cmp::Ordering, collections::HashMap, env};

use anyhow::Result;
use sqlx::PgPool;

use crate::{
    error::BuildEngineError,
    util::{
        clamp_unit, normalize_name, now_epoch_seconds, winrate_pp, BRACKET_BADGE_80,
        PATCH_TAG_CURRENT,
    },
    BuildContext, BuildPath, BuildPathSummary, BuildPhase, ItemDossier,
};

const DEFAULT_MIN_MATCHES: i64 = 500;
const DEFAULT_MIN_PREVALENCE_BUILDS: i64 = 30;
const MAX_ITEMS_PER_PATH: usize = 18;
const MAX_ITEMS_PER_PHASE: usize = 6;

pub(crate) async fn build_context(
    pool: &PgPool,
    hero_query: &str,
    playstyle: Option<&str>,
) -> Result<BuildContext> {
    let requested_playstyle = normalize_playstyle(playstyle)?;
    let hero_id = resolve_hero_id(pool, hero_query).await?;
    let hero = load_hero(pool, hero_id).await?;
    let rows = load_item_rows(pool, hero_id).await?;
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
        phases: build_phases(pool, hero_id, &selected_items, &name_lookup).await?,
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
        hero_base_health: hero.base_health,
        playstyle: requested_playstyle,
        primary_path,
        alternative_paths,
        ability_order: load_ability_order(pool, hero_id).await?,
        generated_at: now_epoch_seconds()?,
    })
}

pub(crate) async fn resolve_hero_id(pool: &PgPool, hero_query: &str) -> Result<i64> {
    if let Ok(hero_id) = hero_query.trim().parse::<i64>() {
        let found: Option<i64> = sqlx::query_scalar!(
            "SELECT hero_id FROM brain.hero_catalog WHERE hero_id=$1",
            hero_id,
        )
        .fetch_optional(pool)
        .await?;
        return found.ok_or_else(|| BuildEngineError::HeroNotFound(hero_query.to_string()).into());
    }

    let query_norm = normalize_name(hero_query);
    if query_norm.is_empty() {
        return Err(BuildEngineError::HeroNotFound(hero_query.to_string()).into());
    }
    if let Some(hero_id) = resolve_from_entity_aliases(pool, &query_norm).await? {
        return Ok(hero_id);
    }

    let heroes = sqlx::query!("SELECT hero_id, name FROM brain.hero_catalog ORDER BY hero_id")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| (row.hero_id, row.name))
        .collect::<Vec<(i64, String)>>();

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

async fn resolve_from_entity_aliases(pool: &PgPool, query_norm: &str) -> Result<Option<i64>> {
    let rows = sqlx::query!(
        r#"
        SELECT e.primary_external_id AS "primary_external_id?", e.canonical_name AS "canonical_name!"
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id = a.entity_id
        WHERE a.alias_norm=$1 AND e.entity_type='hero'
        ORDER BY e.id
        LIMIT 5
        "#,
        query_norm,
    )
    .fetch_all(pool)
    .await?;
    for row in rows {
        if let Some(hero_id) = row
            .primary_external_id
            .and_then(|id| id.parse::<i64>().ok())
        {
            let exists: Option<i64> = sqlx::query_scalar!(
                "SELECT hero_id FROM brain.hero_catalog WHERE hero_id=$1",
                hero_id,
            )
            .fetch_optional(pool)
            .await?;
            if exists.is_some() {
                return Ok(Some(hero_id));
            }
        }
        let canonical_norm = normalize_name(&row.canonical_name);
        if let Some(hero_id) = sqlx::query_scalar!(
            "SELECT hero_id FROM brain.hero_catalog WHERE lower(name)=lower($1)",
            row.canonical_name,
        )
        .fetch_optional(pool)
        .await?
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
    base_health: Option<f64>,
}

async fn load_hero(pool: &PgPool, hero_id: i64) -> Result<HeroRow> {
    let row = sqlx::query!(
        "SELECT name, archetype, base_health FROM brain.hero_catalog WHERE hero_id=$1",
        hero_id,
    )
    .fetch_optional(pool)
    .await?;
    row.map(|row| HeroRow {
        name: row.name,
        archetype: row.archetype,
        base_health: Some(row.base_health as f64),
    })
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

async fn load_item_rows(pool: &PgPool, hero_id: i64) -> Result<Vec<ItemRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT
          i.item_id AS "item_id!", i.name AS "name!", i.slot_type AS "slot_type!",
          i.tier AS "tier!", i.defense_kind::text AS "defense_kind_json!",
          i.damage_axis AS "damage_axis!",
          s.prevalence_builds AS "prevalence_builds!", s.wins AS "wins!",
          s.losses AS "losses!", s.matches AS "matches!",
          s.avg_buy_time_relative AS "avg_buy_time_relative?", s.lift_pp AS "lift_pp?"
        FROM brain.hero_item_stats s
        JOIN brain.item_catalog i ON i.item_id=s.item_id
        WHERE s.hero_id=$1 AND s.bracket=$2 AND s.patch_tag=$3
        ORDER BY s.prevalence_builds DESC, s.matches DESC, i.item_id ASC
        "#,
        hero_id,
        BRACKET_BADGE_80,
        PATCH_TAG_CURRENT,
    )
    .fetch_all(pool)
    .await?;
    let rows = rows
        .into_iter()
        .map(|row| {
            let defense_kind =
                serde_json::from_str::<Vec<String>>(&row.defense_kind_json).unwrap_or_default();
            ItemRow {
                item_id: row.item_id,
                name: row.name,
                slot_type: row.slot_type,
                tier: row.tier,
                defense_kind,
                damage_axis: row.damage_axis,
                prevalence_builds: row.prevalence_builds,
                wins: row.wins,
                losses: row.losses,
                matches: row.matches,
                avg_buy_time_relative: row.avg_buy_time_relative,
                lift_pp: row.lift_pp,
            }
        })
        .collect();
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

async fn build_phases(
    pool: &PgPool,
    hero_id: i64,
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
            dossiers.push(build_dossier(pool, hero_id, item, name_lookup).await?);
        }
        phases.push(BuildPhase {
            phase: phase.to_string(),
            items: dossiers,
        });
    }
    Ok(phases)
}

async fn build_dossier(
    pool: &PgPool,
    hero_id: i64,
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
        synergy_with: synergy_names(pool, hero_id, row.item_id, name_lookup).await?,
        confidence: item.confidence.clone(),
    })
}

async fn synergy_names(
    pool: &PgPool,
    hero_id: i64,
    item_id: i64,
    name_lookup: &HashMap<i64, String>,
) -> Result<Vec<String>> {
    let ids = sqlx::query_scalar!(
        r#"
        SELECT with_item_id
        FROM brain.hero_item_synergies
        WHERE hero_id=$1 AND item_id=$2 AND patch_tag=$3
        ORDER BY matches DESC, with_item_id ASC
        LIMIT 8
        "#,
        hero_id,
        item_id,
        PATCH_TAG_CURRENT,
    )
    .fetch_all(pool)
    .await?;
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

async fn load_ability_order(pool: &PgPool, hero_id: i64) -> Result<Option<Vec<i64>>> {
    let abilities_json: Option<String> = sqlx::query_scalar!(
        r#"
        SELECT abilities::text AS "abilities_json!"
        FROM brain.hero_ability_orders
        WHERE hero_id=$1 AND bracket=$2 AND patch_tag=$3
        "#,
        hero_id,
        BRACKET_BADGE_80,
        PATCH_TAG_CURRENT,
    )
    .fetch_optional(pool)
    .await?;
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
    use super::*;

    // Bekannter Held in der Scratch-Postgres: hero_id 18 = "Mo & Krill" (tank),
    // 156 hero_item_stats-Zeilen (bracket badge80, patch current).
    const MO_HERO_ID: i64 = 18;

    #[tokio::test]
    #[ignore = "benoetigt Scratch-Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn build_context_for_known_hero_yields_typed_evidence() {
        let Some(pool) = crate::util::test_pool().await else {
            eprintln!("DEADLOCK_CENTRAL_DSN nicht gesetzt; Test uebersprungen.");
            return;
        };

        let context = build_context(&pool, "18", None)
            .await
            .expect("build context");

        assert_eq!(context.hero_id, MO_HERO_ID);
        assert_eq!(context.hero_name, "Mo & Krill");
        assert_eq!(context.hero_base_health, Some(930.0));

        let primary_items = context
            .primary_path
            .phases
            .iter()
            .flat_map(|phase| phase.items.iter())
            .collect::<Vec<_>>();
        assert!(
            !primary_items.is_empty(),
            "primary path muss Evidenz enthalten"
        );
        for item in &primary_items {
            assert!(!item.slot_type.trim().is_empty());
            assert!(!item.damage_axis.trim().is_empty());
            assert!(matches!(item.confidence.as_str(), "medium" | "high"));
            assert!(item.prevalence_builds >= 30);
            assert!(item.sample_matches >= 500);
        }

        let json = crate::util::json_string(&context).expect("serialize context");
        assert!(json.contains("\"hero_id\":18"));
    }

    #[tokio::test]
    #[ignore = "benoetigt Scratch-Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn playstyle_selects_requested_path() {
        let Some(pool) = crate::util::test_pool().await else {
            return;
        };
        let context = build_context(&pool, "Mo & Krill", Some("tank"))
            .await
            .expect("build context");
        assert_eq!(context.playstyle.as_deref(), Some("tank"));
        assert_eq!(context.primary_path.label, "tank");
    }

    #[tokio::test]
    #[ignore = "benoetigt Scratch-Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn resolve_hero_id_matches_numeric_and_name() {
        let Some(pool) = crate::util::test_pool().await else {
            return;
        };
        assert_eq!(
            resolve_hero_id(&pool, "18").await.expect("numeric"),
            MO_HERO_ID
        );
        assert_eq!(
            resolve_hero_id(&pool, "Mo & Krill").await.expect("name"),
            MO_HERO_ID
        );
    }

    // Paritaets-Nachweis: load_item_rows liefert exakt so viele Evidenz-Zeilen wie
    // der direkte SQL-Join (hero_item_stats JOIN item_catalog) fuer denselben Held.
    #[tokio::test]
    #[ignore = "benoetigt Scratch-Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn load_item_rows_matches_direct_sql_count() {
        let Some(pool) = crate::util::test_pool().await else {
            return;
        };
        let rows = load_item_rows(&pool, MO_HERO_ID).await.expect("load rows");
        let direct: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM brain.hero_item_stats s \
             JOIN brain.item_catalog i ON i.item_id=s.item_id \
             WHERE s.hero_id=$1 AND s.bracket='badge80' AND s.patch_tag='current'",
        )
        .bind(MO_HERO_ID)
        .fetch_one(&pool)
        .await
        .expect("direct count");
        assert_eq!(rows.len() as i64, direct);
        assert_eq!(direct, 156);
    }
}
