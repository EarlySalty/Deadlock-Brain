use std::{collections::BTreeMap, path::Path};

use serde_json::Value;
use sqlx::Row;

mod ai_roles;
pub mod ability_interactions;
pub mod backtest;
pub mod composer;
pub mod combat;
pub mod inventory;
pub use data::{enrich_frozen_models,refresh_ability_derived,ability_damage_units};
mod data;
pub mod hero;
pub mod item;
pub mod item_interactions;
pub mod mechanics;
pub mod meta;
pub mod patch;
pub mod publish;
mod types;

#[cfg(test)]
mod fix_tests;

pub use ai_roles::{
    build_critic_request, build_hero_analyst_request, build_item_analyst_request,
    build_meta_analyst_request, build_patch_analyst_request, parse_critic_response,
    parse_hero_analyst_response, parse_item_analyst_response, parse_meta_analyst_response,
    parse_patch_analyst_response, run_critic, run_hero_analyst, run_item_analyst, run_meta_analyst,
    run_patch_analyst, CriticResponse, HeroAnalystResponse, ItemAnalystResponse,
    MetaAnalystResponse, PatchAnalystResponse,
};
pub use data::load_hero_abilities;
pub use data::{
    load_author_builds, load_author_source_rows, load_claims, load_hero_model,
    load_hero_stat_values, load_item_models, load_meta_rows, load_patch_events,
    load_patch_events_for_snapshots, load_synergies,
};
pub use types::*;

#[derive(Clone, Copy, Debug)]
pub struct ReasonerOptions<'a> {
    pub seed_path: Option<&'a Path>,
    pub persist: bool,
}

impl Default for ReasonerOptions<'_> {
    fn default() -> Self {
        Self {
            seed_path: None,
            persist: true,
        }
    }
}

pub async fn reason_build(ctx: &ReasonerCtx, hero: &str) -> Result<BuildObject> {
    reason_build_with_seed_path(ctx, hero, None).await
}

pub async fn reason_build_with_seed_path(
    ctx: &ReasonerCtx,
    hero: &str,
    seed_path: Option<&Path>,
) -> Result<BuildObject> {
    reason_build_with_options(
        ctx,
        hero,
        ReasonerOptions {
            seed_path,
            ..Default::default()
        },
    )
    .await
}

pub async fn reason_build_with_options(
    ctx: &ReasonerCtx,
    hero: &str,
    options: ReasonerOptions<'_>,
) -> Result<BuildObject> {
    let seed_path = options.seed_path;
    let ctx = effective_context(ctx).await?;
    let (mut hero_model, mut items, meta, snapshots) =
        load_reasoning_inputs(&ctx, hero, seed_path).await?;
    let events =
        data::load_patch_events_for_snapshots(&ctx, hero_model.hero_id, &snapshots).await?;
    let mut deltas = patch::compute_patch_delta_with_snapshots(&hero_model, &events, &snapshots);
    patch::apply_scored_patch_delta(
        &mut hero_model,
        &mut items,
        &mut deltas,
        &meta.index,
        &ctx.config,
    );
    let mut scored = item::score_items(&hero_model, &items, &meta.index, &[], &ctx.config);
    finish_scores(&mut scored);
    let mut build = composer::compose_build_with_sources(
        &hero_model,
        &scored,
        &deltas,
        &ctx.config,
        &[],
        &meta,
    );
    if ctx.config.use_ai {
        build = enrich_build(
            &ctx,
            &hero_model,
            &scored,
            &deltas,
            &events,
            &meta.index,
            build,
        );
        if let Some(client) = ctx.ai.as_ref() {
            if let Ok(critic) = ai_roles::run_critic(client, &build) {
                if critic.verdict == "recompose" {
                    build = composer::compose_build_with_sources(
                        &hero_model,
                        &scored,
                        &deltas,
                        &ctx.config,
                        &critic.issues,
                        &meta,
                    );
                    build = enrich_build(
                        &ctx,
                        &hero_model,
                        &scored,
                        &deltas,
                        &events,
                        &meta.index,
                        build,
                    );
                    if !critic.issues.is_empty() {
                        build.rationale = append_text(
                            &build.rationale,
                            &format!("Offene Kritikpunkte: {}", critic.issues.join("; ")),
                        );
                    }
                }
            }
        }
    }
    if meta.author_builds.is_empty() {
        build.confidence = Confidence::Low;
        build.rationale = append_text(&build.rationale, &format!("Für diesen Helden fehlen Builds aktiver beobachteter Autoren. Die Kaufkurve ist ein Behelf aus {} beobachteten Builds anderer Helden; ein eigener Autorenvergleich ist nicht möglich.",meta.core_layouts.overall.source_builds));
    }
    if options.persist {
        persist_build(&ctx, &build, &scored).await?;
    }
    Ok(build)
}

pub async fn reason_patch_impact(ctx: &ReasonerCtx, hero: &str) -> Result<PatchImpactReport> {
    reason_patch_impact_with_options(ctx, hero, ReasonerOptions::default()).await
}

pub async fn reason_patch_impact_with_options(
    ctx: &ReasonerCtx,
    hero: &str,
    options: ReasonerOptions<'_>,
) -> Result<PatchImpactReport> {
    let ctx = effective_context(ctx).await?;
    let (hero_model, mut snapshots) = data::load_hero_model_with_snapshots(&ctx, hero).await?;
    let (before_items, item_snapshots) = data::load_item_models_with_snapshots(&ctx).await?;
    snapshots.extend(item_snapshots);
    let events =
        data::load_patch_events_for_snapshots(&ctx, hero_model.hero_id, &snapshots).await?;
    let mut deltas = patch::compute_patch_delta_with_snapshots(&hero_model, &events, &snapshots);
    let mut patched_hero = hero_model.clone();
    let mut after_items = before_items.clone();
    let rows = load_meta_rows(&ctx, hero_model.hero_id).await?;
    let claims = load_claims(&ctx, hero_model.hero_id).await?;
    let meta = meta::build_meta_index(&rows, &[], &claims, &ctx.config);
    patch::apply_scored_patch_delta(
        &mut patched_hero,
        &mut after_items,
        &mut deltas,
        &meta,
        &ctx.config,
    );
    let before = item::score_items(&hero_model, &before_items, &meta, &[], &ctx.config);
    let after = item::score_items(&patched_hero, &after_items, &meta, &[], &ctx.config);
    let before_by_id = before
        .iter()
        .map(|item| (item.item.item_id, item.score.total))
        .collect::<BTreeMap<_, _>>();
    let mut shifted_items = after
        .iter()
        .filter_map(|item| {
            let previous = before_by_id
                .get(&item.item.item_id)
                .copied()
                .unwrap_or_default();
            let shift = item.score.total - previous;
            (shift.abs() > f64::EPSILON).then_some((item.item.item_id, shift))
        })
        .collect::<Vec<_>>();
    shifted_items.sort_by(|left, right| {
        right
            .1
            .abs()
            .total_cmp(&left.1.abs())
            .then_with(|| left.0.cmp(&right.0))
    });
    shifted_items.truncate(20);
    let mut summary = format!(
        "{} deduplizierte Patch-Belege, {} angewendet, {} nicht anwendbar, {} Items verschoben",
        deltas.len(),
        deltas
            .iter()
            .filter(|delta| delta.application.is_some())
            .count(),
        deltas
            .iter()
            .filter(|delta| delta.application.is_none())
            .count(),
        shifted_items.len()
    );
    if ctx.config.use_ai {
        if let Some(client) = ctx.ai.as_ref() {
            if let Ok(response) = ai_roles::run_patch_analyst(client, &deltas, &events) {
                let notes = response
                    .notes
                    .into_iter()
                    .map(|note| format!("{}: {}", note.label, note.rationale))
                    .collect::<Vec<_>>();
                if !notes.is_empty() {
                    summary = append_text(&summary, &notes.join("; "));
                }
            }
        }
    }
    let report = PatchImpactReport {
        hero_id: hero_model.hero_id,
        hero_name: hero_model.name,
        deltas,
        shifted_items,
        summary,
    };
    if options.persist {
        persist_patch_impact(&ctx, &report).await?;
    }
    Ok(report)
}

pub async fn reason_backtest(ctx: &ReasonerCtx, filter: BacktestFilter) -> Result<BacktestReport> {
    reason_backtest_with_seed_path(ctx, filter, None).await
}

pub async fn reason_backtest_with_seed_path(
    ctx: &ReasonerCtx,
    filter: BacktestFilter,
    seed_path: Option<&Path>,
) -> Result<BacktestReport> {
    reason_backtest_with_options(
        ctx,
        filter,
        ReasonerOptions {
            seed_path,
            ..Default::default()
        },
    )
    .await
}

pub async fn reason_backtest_with_options(
    ctx: &ReasonerCtx,
    filter: BacktestFilter,
    options: ReasonerOptions<'_>,
) -> Result<BacktestReport> {
    let seed_path = options.seed_path;
    let mut ctx = ctx.clone();
    if let Some(tag) = filter.patch_tag.as_ref() {
        ctx.config.patch_tag = tag.clone();
    }
    let ctx = effective_context(&ctx).await?;
    let mut run_config = ctx.config.clone();
    let heroes = if let Some(hero) = filter.hero {
        vec![load_hero_model(&ctx, &hero).await?]
    } else {
        load_all_hero_models(&ctx).await?
    };
    run_config.use_ai = false;
    let deterministic_ctx = ReasonerCtx {
        pool: ctx.pool.clone(),
        ai: None,
        config: run_config,
    };
    let mut reports = Vec::new();
    for hero_model in heroes {
        let build =
            reason_build_with_options(&deterministic_ctx, &hero_model.name, options).await?;
        let mut authors = load_author_builds(&ctx, hero_model.hero_id).await?;
        if let Some(path) = seed_path {
            let items = load_item_models(&ctx).await?;
            authors.extend(meta::load_seed_builds_for_hero(
                path,
                &items,
                &hero_model.name,
            )?);
        }
        if let Some(tag) = filter.patch_tag.as_deref() {
            authors.retain(|author| author.patch_tag.as_deref() == Some(tag));
        }
        reports.push(backtest::backtest_hero_with_build(
            hero_model.hero_id,
            &hero_model.name,
            &build,
            &authors,
        ));
    }
    let report = BacktestReport { heroes: reports };
    if options.persist {
        persist_backtest(&deterministic_ctx, &report).await?;
    }
    Ok(report)
}

async fn effective_context(ctx: &ReasonerCtx) -> Result<ReasonerCtx> {
    if ctx.config.patch_tag != "current" {
        return Ok(ctx.clone());
    }
    let patch_tag = dbrain_builds::latest_patch_tag(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut config = ctx.config.clone();
    config.patch_tag = patch_tag;
    Ok(ReasonerCtx {
        pool: ctx.pool.clone(),
        ai: ctx.ai.clone(),
        config,
    })
}

const UPSERT_BUILD: &str = "INSERT INTO brain.reasoner_builds (hero_id, patch_tag, hero_name, build, confidence, used_ai) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (hero_id, patch_tag) DO UPDATE SET hero_name=EXCLUDED.hero_name, build=EXCLUDED.build, confidence=EXCLUDED.confidence, used_ai=EXCLUDED.used_ai";

const UPSERT_SCORE: &str = "INSERT INTO brain.reasoner_item_scores (hero_id, patch_tag, item_id, combat_value, per_slot_value, per_soul_value, purchase_bonus, condition_factor, active_value, passive_value, meta_support, total, confidence, buy_phase) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) ON CONFLICT (hero_id, patch_tag, item_id) DO UPDATE SET combat_value=EXCLUDED.combat_value, per_slot_value=EXCLUDED.per_slot_value, per_soul_value=EXCLUDED.per_soul_value, purchase_bonus=EXCLUDED.purchase_bonus, condition_factor=EXCLUDED.condition_factor, active_value=EXCLUDED.active_value, passive_value=EXCLUDED.passive_value, meta_support=EXCLUDED.meta_support, total=EXCLUDED.total, confidence=EXCLUDED.confidence, buy_phase=EXCLUDED.buy_phase";

const UPSERT_DELTA: &str = "INSERT INTO brain.reasoner_patch_deltas (hero_id, patch_tag, target_kind, target_id, mechanic, sign, magnitude, note) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (hero_id, patch_tag, target_kind, target_id, mechanic) DO UPDATE SET sign=EXCLUDED.sign, magnitude=EXCLUDED.magnitude, note=EXCLUDED.note";

const UPSERT_BACKTEST: &str = "INSERT INTO brain.reasoner_backtests (hero_id, patch_tag, author, core_coverage, order_proximity, switch_detected, detail) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (hero_id, patch_tag, author) DO UPDATE SET core_coverage=EXCLUDED.core_coverage, order_proximity=EXCLUDED.order_proximity, switch_detected=EXCLUDED.switch_detected, detail=EXCLUDED.detail";

fn persistence_json(value: &impl serde::Serialize) -> Result<Value> {
    serde_json::to_value(value)
        .map_err(|error| ReasonerError::Data(format!("Reasoner-Persistenz: {error}")))
}

async fn persist_build(
    ctx: &ReasonerCtx,
    build: &BuildObject,
    scored: &[ScoredItem],
) -> Result<()> {
    let mut tx = ctx.pool.begin().await.map_err(ReasonerError::Db)?;
    sqlx::query(UPSERT_BUILD)
        .bind(build.hero_id)
        .bind(&build.patch_tag)
        .bind(&build.hero_name)
        .bind(persistence_json(build)?)
        .bind(format!("{:?}", build.confidence))
        .bind(ctx.config.use_ai && ctx.ai.is_some())
        .execute(&mut *tx)
        .await
        .map_err(ReasonerError::Db)?;
    for item in scored {
        sqlx::query(UPSERT_SCORE)
            .bind(build.hero_id)
            .bind(&build.patch_tag)
            .bind(item.item.item_id)
            .bind(item.score.combat_value)
            .bind(item.score.per_slot_value)
            .bind(item.score.per_soul_value)
            .bind(item.score.purchase_bonus_value)
            .bind(item.score.condition_factor)
            .bind(item.score.active_value)
            .bind(item.score.passive_value)
            .bind(item.score.meta_support)
            .bind(item.score.total)
            .bind(format!("{:?}", item.confidence))
            .bind(format!("{:?}", item.buy_phase))
            .execute(&mut *tx)
            .await
            .map_err(ReasonerError::Db)?;
    }
    tx.commit().await.map_err(ReasonerError::Db)
}

async fn persist_patch_impact(ctx: &ReasonerCtx, report: &PatchImpactReport) -> Result<()> {
    let mut tx = ctx.pool.begin().await.map_err(ReasonerError::Db)?;
    for delta in &report.deltas {
        let (kind, id) = match delta.target {
            DeltaTarget::Hero(id) => ("hero", id),
            DeltaTarget::Item(id) => ("item", id),
            DeltaTarget::Ability(id) => ("ability", id),
        };
        sqlx::query(UPSERT_DELTA)
            .bind(report.hero_id)
            .bind(&ctx.config.patch_tag)
            .bind(kind)
            .bind(id)
            .bind(&delta.mechanic)
            .bind(i16::from(delta.sign))
            .bind(delta.magnitude)
            .bind(&delta.note)
            .execute(&mut *tx)
            .await
            .map_err(ReasonerError::Db)?;
    }
    tx.commit().await.map_err(ReasonerError::Db)
}

async fn persist_backtest(ctx: &ReasonerCtx, report: &BacktestReport) -> Result<()> {
    let mut tx = ctx.pool.begin().await.map_err(ReasonerError::Db)?;
    for hero in &report.heroes {
        let mut authors = BTreeMap::<&str, Vec<&BacktestMetrics>>::new();
        for (author, metrics) in &hero.per_author {
            authors.entry(author).or_default().push(metrics);
        }
        if authors.is_empty() {
            authors.insert("", vec![&hero.aggregate]);
        }
        for (author, comparisons) in authors {
            let core_coverage = comparisons
                .iter()
                .map(|metrics| metrics.core_coverage)
                .sum::<f64>()
                / comparisons.len() as f64;
            let orders = comparisons
                .iter()
                .filter_map(|metrics| metrics.order_proximity)
                .collect::<Vec<_>>();
            let switches = comparisons
                .iter()
                .filter_map(|metrics| metrics.switch_detected)
                .collect::<Vec<_>>();
            let order_proximity =
                (!orders.is_empty()).then(|| orders.iter().sum::<f64>() / orders.len() as f64);
            let switch_detected =
                (!switches.is_empty()).then(|| switches.iter().any(|value| *value));
            sqlx::query(UPSERT_BACKTEST)
                .bind(hero.hero_id)
                .bind(&ctx.config.patch_tag)
                .bind(author)
                .bind(core_coverage)
                .bind(order_proximity)
                .bind(switch_detected)
                .bind(persistence_json(hero)?)
                .execute(&mut *tx)
                .await
                .map_err(ReasonerError::Db)?;
        }
    }
    tx.commit().await.map_err(ReasonerError::Db)
}

pub async fn load_reasoning_inputs(
    ctx: &ReasonerCtx,
    hero: &str,
    _seed_path: Option<&Path>,
) -> Result<(
    HeroModel,
    Vec<ItemModel>,
    meta::MetaIndexWithSources,
    Vec<PatchSnapshot>,
)> {
    let (hero_model, mut snapshots) = data::load_hero_model_with_snapshots(ctx, hero).await?;
    let (items, item_snapshots) = data::load_item_models_with_snapshots(ctx).await?;
    snapshots.extend(item_snapshots);
    let items = items
        .iter()
        .map(item::build_item_model)
        .collect::<Result<Vec<_>>>()?;
    let mut core_layouts = data::load_core_layouts(ctx).await?;
    let slot_snapshot: Option<Value> = sqlx::query_scalar("SELECT payload FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='hero' AND payload->>'id'=$1 ORDER BY fetched_at DESC, id DESC LIMIT 1")
        .bind(hero_model.hero_id.to_string())
        .fetch_optional(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    if let Some(flex_slots) = slot_snapshot.as_ref().and_then(meta::snapshot_flex_slots) {
        let mut layout = core_layouts.for_hero(hero_model.hero_id).clone();
        layout.flex_slots = flex_slots;
        core_layouts.by_hero.insert(hero_model.hero_id, layout);
    }
    let rows = load_meta_rows(ctx, hero_model.hero_id).await?;
    let combinations = meta::combination_support(
        &load_synergies(ctx, hero_model.hero_id).await?,
        &rows,
        &ctx.config,
    );
    let authors = load_author_builds(ctx, hero_model.hero_id).await?;
    let claims = load_claims(ctx, hero_model.hero_id).await?;
    let index = meta::build_meta_index(&rows, &authors, &claims, &ctx.config);
    let author_builds = load_author_sources(ctx, hero_model.hero_id).await?;
    let hero_ability_orders = load_hero_ability_orders(ctx, hero_model.hero_id).await?;
    Ok((
        hero_model,
        items,
        meta::MetaIndexWithSources {
            index,
            author_builds,
            hero_ability_orders,
            core_layouts,
            combinations,
        },
        snapshots,
    ))
}

fn finish_scores(scored: &mut [ScoredItem]) {
    for item in scored {
        if !item.score.total.is_finite() || item.score.total <= 0.0 {
            item.confidence = Confidence::Low;
        }
    }
}

async fn load_all_hero_models(ctx: &ReasonerCtx) -> Result<Vec<HeroModel>> {
    let rows = sqlx::query("SELECT hero_id, name FROM brain.hero_catalog ORDER BY hero_id")
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut heroes = Vec::new();
    for row in rows {
        let name: String = row.try_get("name").map_err(ReasonerError::Db)?;
        heroes.push(load_hero_model(ctx, &name).await?);
    }
    Ok(heroes)
}

async fn load_author_sources(
    ctx: &ReasonerCtx,
    hero_id: i64,
) -> Result<Vec<meta::AuthorBuildSource>> {
    let rows = data::load_author_source_rows(ctx, Some(hero_id)).await?;
    rows.into_iter()
        .map(|value| {
            Ok(meta::AuthorBuildSource {
                hero_id: value["hero_id"].as_i64().unwrap_or(hero_id),
                author: value["author"].as_str().unwrap_or("unbekannt").to_string(),
                weight: value["weight"].as_f64().unwrap_or_default(),
                details: value.get("details").cloned().unwrap_or(Value::Null),
            })
        })
        .collect()
}

async fn load_hero_ability_orders(
    ctx: &ReasonerCtx,
    hero_id: i64,
) -> Result<BTreeMap<i64, Vec<AbilityStep>>> {
    let result = sqlx::query_scalar::<_, String>(
        "SELECT abilities::text FROM brain.hero_ability_orders WHERE hero_id=$1 ORDER BY matches DESC, updated_at DESC LIMIT 1",
    )
    .bind(hero_id)
    .fetch_optional(&ctx.pool)
    .await;
    let Some(text) = result.map_err(ReasonerError::Db)? else {
        return Ok(BTreeMap::new());
    };
    let values: Vec<Value> = serde_json::from_str(&text)
        .map_err(|error| ReasonerError::Data(format!("Skill-Order: {error}")))?;
    let order = fallback_ability_order(&values)?;
    Ok([(hero_id, order)].into_iter().collect())
}

fn fallback_ability_order(values: &[Value]) -> Result<Vec<AbilityStep>> {
    let mut levels = BTreeMap::<i64, usize>::new();
    values
        .iter()
        .map(|value| {
            let step = if value.is_object() {
                serde_json::from_value::<AbilityStep>(value.clone())
                    .map_err(|error| ReasonerError::Data(format!("Skill-Order: {error}")))?
            } else {
                let ability_id = value.as_i64().ok_or_else(|| {
                    ReasonerError::Data("Skill-Order: ungültige Ability-ID".into())
                })?;
                let level = levels.get(&ability_id).copied().unwrap_or_default();
                let (currency_type, delta) = [(2, -1), (1, -1), (1, -2), (1, -5)]
                    .get(level)
                    .copied()
                    .ok_or_else(|| {
                        ReasonerError::Data(format!(
                            "Skill-Order: mehr als vier Schritte für Ability {ability_id}"
                        ))
                    })?;
                AbilityStep {
                    ability_id,
                    currency_type,
                    delta,
                }
            };
            if step.ability_id <= 0 {
                return Err(ReasonerError::Data(
                    "Skill-Order: ungültige Ability-ID".into(),
                ));
            }
            *levels.entry(step.ability_id).or_default() += 1;
            Ok(step)
        })
        .collect()
}

fn enrich_build(
    ctx: &ReasonerCtx,
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    events: &[Value],
    meta: &MetaIndex,
    mut build: BuildObject,
) -> BuildObject {
    let Some(client) = ctx.ai.as_ref() else {
        return build;
    };
    let mut rationale = Vec::new();
    if let Ok(response) = ai_roles::run_hero_analyst(client, hero, &hero.damage_plan) {
        if !response.playstyle.trim().is_empty() {
            rationale.push(format!("Spielstil: {}", response.playstyle.trim()));
        }
        let roles = response
            .ability_roles
            .into_iter()
            .filter(|note| note.ability_id > 0 && !note.role.trim().is_empty())
            .map(|note| format!("Ability {}: {}", note.ability_id, note.role.trim()))
            .collect::<Vec<_>>();
        if !roles.is_empty() {
            rationale.push(format!("Fähigkeitsrollen: {}", roles.join(", ")));
        }
    }
    let build_ids = build_item_ids(&build);
    let foreign_names = scored
        .iter()
        .filter(|item| !build_ids.contains(&item.item.item_id))
        .map(|item| normalize_text(&item.item.name))
        .filter(|name| name.len() >= 5)
        .collect::<Vec<_>>();
    let selected = scored
        .iter()
        .filter(|item| build_ids.contains(&item.item.item_id))
        .cloned()
        .collect::<Vec<_>>();
    if let Ok(response) = ai_roles::run_item_analyst(client, &selected) {
        for note in response.items {
            if !build_ids.contains(&note.item_id) {
                continue;
            }
            let text = append_text(&note.why, &note.condition_note);
            let normalized = normalize_text(&text);
            if foreign_names
                .iter()
                .any(|name| normalized.contains(name.as_str()))
            {
                continue;
            }
            if let Some(item) = find_build_item_mut(&mut build, note.item_id) {
                item.why = text;
            }
        }
    }
    if let Ok(response) = ai_roles::run_patch_analyst(client, deltas, events) {
        let notes = response
            .notes
            .into_iter()
            .map(|note| format!("{}: {}", note.label, note.rationale))
            .filter(|note| !note.trim().is_empty())
            .collect::<Vec<_>>();
        if !notes.is_empty() {
            rationale.push(format!("Patch: {}", notes.join("; ")));
        }
    }
    if let Ok(response) = ai_roles::run_meta_analyst(client, meta) {
        if !response.summary.trim().is_empty() {
            rationale.push(format!("Meta: {}", response.summary.trim()));
        }
    }
    if !rationale.is_empty() {
        build.rationale = append_text(&build.rationale, &rationale.join(" "));
    }
    build
}

fn build_item_ids(build: &BuildObject) -> std::collections::BTreeSet<i64> {
    build
        .core
        .iter()
        .chain(build.situations.iter().flat_map(|block| block.items.iter()))
        .map(|item| item.item_id)
        .collect()
}

fn find_build_item_mut(build: &mut BuildObject, item_id: i64) -> Option<&mut BuildItem> {
    build
        .core
        .iter_mut()
        .chain(
            build
                .situations
                .iter_mut()
                .flat_map(|block| block.items.iter_mut()),
        )
        .find(|item| item.item_id == item_id)
}

fn normalize_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn append_text(left: &str, right: &str) -> String {
    match (left.trim(), right.trim()) {
        ("", right) => right.to_string(),
        (left, "") => left.to_string(),
        (left, right) => format!("{left} {right}"),
    }
}
