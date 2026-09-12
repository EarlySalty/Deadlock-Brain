use std::{collections::BTreeMap, path::Path};

use serde_json::Value;
use sqlx::Row;

mod ai_roles;
pub mod backtest;
pub mod composer;
mod data;
pub mod hero;
pub mod item;
pub mod mechanics;
pub mod meta;
pub mod patch;
pub mod publish;
mod types;

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
    load_author_builds, load_claims, load_hero_model, load_hero_stat_values, load_item_models,
    load_meta_rows, load_patch_events, load_synergies,
};
pub use types::*;

pub async fn reason_build(ctx: &ReasonerCtx, hero: &str) -> Result<BuildObject> {
    reason_build_with_seed_path(ctx, hero, None).await
}

pub async fn reason_build_with_seed_path(
    ctx: &ReasonerCtx,
    hero: &str,
    seed_path: Option<&Path>,
) -> Result<BuildObject> {
    let ctx = effective_context(ctx).await?;
    let (mut hero_model, mut items, meta) = load_reasoning_inputs(&ctx, hero, seed_path).await?;
    let events = load_patch_events(&ctx, hero_model.hero_id).await?;
    let deltas = patch::compute_patch_delta(&hero_model, &events);
    patch::apply_patch_delta(&mut hero_model, &mut items, &deltas);
    let scored = item::score_items(&hero_model, &items, &meta.index, &deltas, &ctx.config);
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
    Ok(build)
}

pub async fn reason_patch_impact(ctx: &ReasonerCtx, hero: &str) -> Result<PatchImpactReport> {
    let ctx = effective_context(ctx).await?;
    let hero_model = hero::load_built_hero_model(&ctx, hero).await?;
    let events = load_patch_events(&ctx, hero_model.hero_id).await?;
    let deltas = patch::compute_patch_delta(&hero_model, &events);
    let mut patched_hero = hero_model.clone();
    let before_items = load_item_models(&ctx).await?;
    let mut after_items = before_items.clone();
    patch::apply_patch_delta(&mut patched_hero, &mut after_items, &deltas);
    let rows = load_meta_rows(&ctx, hero_model.hero_id).await?;
    let claims = load_claims(&ctx, hero_model.hero_id).await?;
    let meta = meta::build_meta_index(&rows, &[], &claims, &ctx.config);
    let before = item::score_items(&hero_model, &before_items, &meta, &[], &ctx.config);
    let after = item::score_items(&patched_hero, &after_items, &meta, &deltas, &ctx.config);
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
        "{} Patch-Deltas, {} Items verschoben",
        deltas.len(),
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
    Ok(PatchImpactReport {
        hero_id: hero_model.hero_id,
        hero_name: hero_model.name,
        deltas,
        shifted_items,
        summary,
    })
}

pub async fn reason_backtest(ctx: &ReasonerCtx, filter: BacktestFilter) -> Result<BacktestReport> {
    reason_backtest_with_seed_path(ctx, filter, None).await
}

pub async fn reason_backtest_with_seed_path(
    ctx: &ReasonerCtx,
    filter: BacktestFilter,
    seed_path: Option<&Path>,
) -> Result<BacktestReport> {
    let ctx = effective_context(ctx).await?;
    let mut run_config = ctx.config.clone();
    if let Some(tag) = filter.patch_tag.as_ref() {
        run_config.patch_tag = tag.clone();
    }
    let heroes = if let Some(hero) = filter.hero {
        vec![hero::load_built_hero_model(&ctx, &hero).await?]
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
            reason_build_with_seed_path(&deterministic_ctx, &hero_model.name, seed_path).await?;
        let mut authors = load_author_builds(&ctx, hero_model.hero_id).await?;
        if let Some(path) = seed_path {
            let items = load_item_models(&ctx).await?;
            authors.extend(meta::load_seed_builds(path, &items)?);
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
    Ok(BacktestReport { heroes: reports })
}

async fn effective_context(ctx: &ReasonerCtx) -> Result<ReasonerCtx> {
    if ctx.config.patch_tag != "current" {
        return Ok(ctx.clone());
    }
    let patch_tag = sqlx::query_scalar::<_, String>(
        "SELECT COALESCE(NULLIF(pe.patch_external_id, ''), to_char(pe.posted_at::date, 'YYYY-MM-DD')) FROM brain.patch_events pe WHERE pe.patch_external_id IS NOT NULL OR pe.posted_at IS NOT NULL ORDER BY pe.posted_at DESC NULLS LAST, pe.id DESC LIMIT 1",
    )
    .fetch_optional(&ctx.pool)
    .await
    .map_err(ReasonerError::Db)?;
    let Some(patch_tag) = patch_tag else {
        return Ok(ctx.clone());
    };
    let mut config = ctx.config.clone();
    config.patch_tag = patch_tag;
    Ok(ReasonerCtx {
        pool: ctx.pool.clone(),
        ai: ctx.ai.clone(),
        config,
    })
}

async fn load_reasoning_inputs(
    ctx: &ReasonerCtx,
    hero: &str,
    seed_path: Option<&Path>,
) -> Result<(HeroModel, Vec<ItemModel>, meta::MetaIndexWithSources)> {
    let hero_model = hero::load_built_hero_model(ctx, hero).await?;
    let items = load_item_models(ctx).await?;
    let items = items
        .iter()
        .map(item::build_item_model)
        .collect::<Result<Vec<_>>>()?;
    let rows = load_meta_rows(ctx, hero_model.hero_id).await?;
    let authors = load_author_builds(ctx, hero_model.hero_id).await?;
    let claims = load_claims(ctx, hero_model.hero_id).await?;
    let index = if let Some(path) = seed_path {
        meta::build_meta_index_with_seed_path(&rows, &authors, &claims, &ctx.config, path, &items)?
    } else {
        meta::build_meta_index(&rows, &authors, &claims, &ctx.config)
    };
    let author_builds = load_author_sources(ctx, hero_model.hero_id).await?;
    let hero_ability_orders = load_hero_ability_orders(ctx, hero_model.hero_id).await?;
    Ok((
        hero_model,
        items,
        meta::MetaIndexWithSources {
            index,
            author_builds,
            hero_ability_orders,
        },
    ))
}

async fn load_all_hero_models(ctx: &ReasonerCtx) -> Result<Vec<HeroModel>> {
    let rows = sqlx::query("SELECT hero_id, name FROM brain.hero_catalog ORDER BY hero_id")
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut heroes = Vec::new();
    for row in rows {
        let name: String = row.try_get("name").map_err(ReasonerError::Db)?;
        heroes.push(hero::load_built_hero_model(ctx, &name).await?);
    }
    Ok(heroes)
}

async fn load_author_sources(
    ctx: &ReasonerCtx,
    hero_id: i64,
) -> Result<Vec<meta::AuthorBuildSource>> {
    let query = "SELECT jsonb_build_object('hero_id', hbs.hero_id, 'author', COALESCE(hbs.author_account_id::text, 'unbekannt'), 'weight', COALESCE(wba.priority, 0)::double precision, 'details', hbs.details)::text AS row_json FROM tierlist.hero_build_sources hbs LEFT JOIN tierlist.watched_build_authors wba ON wba.author_account_id=hbs.author_account_id WHERE hbs.hero_id=$1 ORDER BY COALESCE(hbs.last_updated_at, hbs.published_at) DESC NULLS LAST, hbs.version DESC NULLS LAST";
    let rows = sqlx::query(query)
        .bind(hero_id)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            let text: String = row.try_get("row_json").map_err(ReasonerError::Db)?;
            let value: Value = serde_json::from_str(&text)
                .map_err(|error| ReasonerError::Data(format!("Autoren-Quelle: {error}")))?;
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
    let order = values
        .iter()
        .filter_map(|value| value.as_i64())
        .filter(|ability_id| *ability_id > 0)
        .map(|ability_id| AbilityStep {
            ability_id,
            currency_type: 0,
            delta: 1,
        })
        .collect::<Vec<_>>();
    Ok([(hero_id, order)].into_iter().collect())
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
