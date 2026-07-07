use std::{collections::BTreeMap, thread, time::Duration};

use deadlock_brain_core::minimax::{
    extract_minimax_text, minimax_usage_summary, ChatCompletionRequest, ChatMessage, MiniMaxClient,
    MiniMaxConfig,
};
use serde_json::{json, Map, Value};
use sqlx::PgPool;

use crate::{
    build_optimizer::build_hero_build_context,
    util::{
        clamp_i64, execute_sql, extract_insights, get, get_any, get_string, int_or_none,
        int_or_zero, json_loads, prompt_text_from_request, query_json_rows, query_one_json,
        stable_hash_text, value_to_string, SqlParam,
    },
    LearnError, Result,
};

pub const BUILD_LEARNING_PROMPT_VERSION: &str = "build_learning_de_v1";

#[derive(Debug, Clone)]
pub struct LearnImportSteamBuildsOptions {
    pub hero: Option<String>,
    pub language: i64,
    pub limit_per_hero: i64,
}

impl Default for LearnImportSteamBuildsOptions {
    fn default() -> Self {
        Self {
            hero: None,
            language: 0,
            limit_per_hero: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LearnAnalyzeBuildOptions {
    pub build_id: i64,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub include_request: bool,
}

#[derive(Debug, Clone)]
pub struct LearnAnalyzeNextOptions {
    pub hero: Option<String>,
    pub limit: i64,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub delay_seconds: f64,
}

pub async fn learn_import_steam_builds(
    pool: &PgPool,
    options: LearnImportSteamBuildsOptions,
) -> Result<Value> {
    // Der Steam-GC-Build-Quelltisch `hero_build_sources` gehoert dem Steam-Bot
    // und liegt in der zentralen Postgres (search_path). Er ist kein `brain.*`-Objekt.
    if !steam_source_available(pool).await? {
        return Ok(json!({
            "source": "central_postgres.hero_build_sources",
            "imported": 0,
            "updated": 0,
            "skipped": 0,
            "error": "hero_build_sources_missing",
        }));
    }

    let hero_map = hero_id_name_map(pool).await?;
    let item_map = item_id_name_map(pool).await?;
    let rows = load_steam_build_rows(
        pool,
        &hero_map,
        options.hero.as_deref(),
        options.language,
        options.limit_per_hero,
    )
    .await?;
    let mut imported = 0;
    let mut updated = 0;
    let mut skipped = 0;
    for row in &rows {
        let source_build_id = get(row, "hero_build_id")
            .map(value_to_string)
            .unwrap_or_default()
            .trim()
            .to_string();
        if source_build_id.is_empty() {
            skipped += 1;
            continue;
        }
        let hero_id = int_or_none(get(row, "hero_id"));
        let hero_name = hero_id
            .and_then(|id| hero_map.get(&id).cloned())
            .or_else(|| hero_id.map(|id| format!("hero_id:{id}")));
        let details = json_loads(get(row, "details_json").and_then(Value::as_str), json!({}));
        let tags = json_loads(get(row, "tags_json").and_then(Value::as_str), json!([]));
        let item_names = extract_item_names(&details, &item_map);
        let ability_order = extract_ability_order(&details, &item_map);
        let source_rank = int_or_zero(get(row, "source_rank"));
        let (quality_tier, quality_score) = quality_from_rank(source_rank);
        let before = query_one_json(
            pool,
            "SELECT id FROM brain.learned_builds WHERE source=$1 AND source_build_id=$2",
            &[
                SqlParam::Text("steam_gc".to_string()),
                SqlParam::Text(source_build_id.clone()),
            ],
        )
        .await?;
        let tags_value = if tags.is_array() {
            tags.clone()
        } else {
            json!([])
        };
        let tags_json = serde_json::to_string(&tags_value)?;
        let item_names_json = serde_json::to_string(&item_names)?;
        let ability_order_json = serde_json::to_string(&ability_order)?;
        let metadata_json = serde_json::to_string(&json!({
            "origin_build_id": get(row, "origin_build_id").cloned().unwrap_or(Value::Null),
            "version": get(row, "version").cloned().unwrap_or(Value::Null),
            "publish_ts": get(row, "publish_ts").cloned().unwrap_or(Value::Null),
            "last_updated_ts": get(row, "last_updated_ts").cloned().unwrap_or(Value::Null),
            "fetched_at": get(row, "fetched_at").cloned().unwrap_or(Value::Null),
            "last_seen_at": get(row, "last_seen_at").cloned().unwrap_or(Value::Null),
        }))?;
        let details_json = get(row, "details_json")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .unwrap_or("{}")
            .to_string();
        let language = int_or_none(get(row, "language"));
        let name = get(row, "name").and_then(Value::as_str).map(str::to_string);
        let author_account_id = get(row, "author_account_id")
            .filter(|value| !value.is_null())
            .map(value_to_string)
            .filter(|value| !value.is_empty());
        let description = get(row, "description")
            .and_then(Value::as_str)
            .map(str::to_string);
        execute_sql(
            pool,
            r#"
            INSERT INTO brain.learned_builds(
              source, source_build_id, hero_id, hero_name, language, source_rank,
              quality_tier, quality_score, name, author_account_id, description,
              tags, details, item_names, ability_order,
              source_metadata, imported_at, updated_at
            )
            VALUES(
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
              $12::text::jsonb, $13::text::jsonb, $14::text::jsonb, $15::text::jsonb,
              $16::text::jsonb, now(), now()
            )
            ON CONFLICT(source, source_build_id) DO UPDATE SET
              hero_id=excluded.hero_id,
              hero_name=excluded.hero_name,
              language=excluded.language,
              source_rank=excluded.source_rank,
              quality_tier=excluded.quality_tier,
              quality_score=excluded.quality_score,
              name=excluded.name,
              author_account_id=excluded.author_account_id,
              description=excluded.description,
              tags=excluded.tags,
              details=excluded.details,
              item_names=excluded.item_names,
              ability_order=excluded.ability_order,
              source_metadata=excluded.source_metadata,
              updated_at=excluded.updated_at
            "#,
            &[
                SqlParam::Text("steam_gc".to_string()),
                SqlParam::Text(source_build_id.clone()),
                SqlParam::IntOpt(hero_id),
                SqlParam::TextOpt(hero_name.clone()),
                SqlParam::IntOpt(language),
                SqlParam::Int(source_rank),
                SqlParam::Text(quality_tier.to_string()),
                SqlParam::Float(quality_score),
                SqlParam::TextOpt(name.clone()),
                SqlParam::TextOpt(author_account_id.clone()),
                SqlParam::TextOpt(description.clone()),
                SqlParam::Text(tags_json),
                SqlParam::Text(details_json),
                SqlParam::Text(item_names_json),
                SqlParam::Text(ability_order_json),
                SqlParam::Text(metadata_json),
            ],
        )
        .await?;
        if before.is_some() {
            updated += 1;
        } else {
            imported += 1;
        }
    }
    Ok(json!({
        "source": "central_postgres.hero_build_sources",
        "language": options.language,
        "limit_per_hero": options.limit_per_hero,
        "rows_seen": rows.len(),
        "imported": imported,
        "updated": updated,
        "skipped": skipped,
    }))
}

async fn steam_source_available(pool: &PgPool) -> Result<bool> {
    let row = query_one_json(
        pool,
        "SELECT (to_regclass('hero_build_sources') IS NOT NULL) AS present",
        &[],
    )
    .await?;
    Ok(row
        .and_then(|row| get(&row, "present").and_then(Value::as_bool))
        .unwrap_or(false))
}

pub async fn learn_list_builds(
    pool: &PgPool,
    hero: Option<&str>,
    limit: i64,
) -> Result<Vec<Value>> {
    let bounded_limit = clamp_i64(limit, 1, 500);
    let rows = if let Some(hero) = hero.filter(|value| !value.trim().is_empty()) {
        let like = format!("%{hero}%");
        query_json_rows(
            pool,
            r#"
            SELECT id, source, source_build_id, hero_name, language, source_rank,
                   quality_tier, quality_score, name, author_account_id,
                   extract(epoch from updated_at)::int8 AS updated_at,
                   item_names::text AS item_names_json
            FROM brain.learned_builds
            WHERE hero_name LIKE $1
            ORDER BY hero_name, quality_score DESC, source_rank ASC, updated_at DESC LIMIT $2
            "#,
            &[SqlParam::Text(like), SqlParam::Int(bounded_limit)],
        )
        .await?
    } else {
        query_json_rows(
            pool,
            r#"
            SELECT id, source, source_build_id, hero_name, language, source_rank,
                   quality_tier, quality_score, name, author_account_id,
                   extract(epoch from updated_at)::int8 AS updated_at,
                   item_names::text AS item_names_json
            FROM brain.learned_builds
            ORDER BY hero_name, quality_score DESC, source_rank ASC, updated_at DESC LIMIT $1
            "#,
            &[SqlParam::Int(bounded_limit)],
        )
        .await?
    };
    Ok(rows
        .into_iter()
        .map(|mut row| {
            let items = json_loads(
                get(&row, "item_names_json").and_then(Value::as_str),
                json!([]),
            );
            if let Some(object) = row.as_object_mut() {
                object.insert("item_names".to_string(), items);
            }
            row
        })
        .collect())
}

pub(crate) async fn list_pending_build_learning_targets(
    pool: &PgPool,
    hero: Option<&str>,
    limit: i64,
    model: Option<&str>,
) -> Result<Vec<Value>> {
    let bounded_limit = clamp_i64(limit, 1, 100);
    let rows = if let Some(hero) = hero.filter(|value| !value.trim().is_empty()) {
        let like = format!("%{hero}%");
        query_json_rows(
            pool,
            r#"
            SELECT id, source, source_build_id, hero_name, source_rank,
                   quality_tier, quality_score, name, item_names::text AS item_names_json
            FROM brain.learned_builds lb
            WHERE NOT EXISTS (
              SELECT 1
              FROM brain.build_learning_notes n
              WHERE n.learned_build_id = lb.id
                AND n.status = 'analysis_ready'
                AND n.prompt_version = $1
                AND COALESCE(n.model, '') = COALESCE($2, '')
            )
              AND lb.hero_name LIKE $3
            ORDER BY lb.quality_score DESC, lb.source_rank ASC, lb.updated_at DESC, lb.id ASC
            LIMIT $4
            "#,
            &[
                SqlParam::Text(BUILD_LEARNING_PROMPT_VERSION.to_string()),
                SqlParam::TextOpt(model.map(str::to_string)),
                SqlParam::Text(like),
                SqlParam::Int(bounded_limit),
            ],
        )
        .await?
    } else {
        query_json_rows(
            pool,
            r#"
            SELECT id, source, source_build_id, hero_name, source_rank,
                   quality_tier, quality_score, name, item_names::text AS item_names_json
            FROM brain.learned_builds lb
            WHERE NOT EXISTS (
              SELECT 1
              FROM brain.build_learning_notes n
              WHERE n.learned_build_id = lb.id
                AND n.status = 'analysis_ready'
                AND n.prompt_version = $1
                AND COALESCE(n.model, '') = COALESCE($2, '')
            )
            ORDER BY lb.quality_score DESC, lb.source_rank ASC, lb.updated_at DESC, lb.id ASC
            LIMIT $3
            "#,
            &[
                SqlParam::Text(BUILD_LEARNING_PROMPT_VERSION.to_string()),
                SqlParam::TextOpt(model.map(str::to_string)),
                SqlParam::Int(bounded_limit),
            ],
        )
        .await?
    };
    Ok(rows
        .into_iter()
        .map(|mut row| {
            let items = json_loads(
                get(&row, "item_names_json").and_then(Value::as_str),
                json!([]),
            );
            if let Some(object) = row.as_object_mut() {
                object.insert("item_names".to_string(), items);
            }
            row
        })
        .collect())
}

pub async fn build_learning_context(pool: &PgPool, learned_build_id: i64) -> Result<Value> {
    let row = query_one_json(
        pool,
        r#"
        SELECT id, source, source_build_id, hero_id, hero_name, language, source_rank,
               quality_tier, quality_score, name, author_account_id, description,
               extract(epoch from imported_at)::int8 AS imported_at,
               extract(epoch from updated_at)::int8 AS updated_at,
               tags::text AS tags_json,
               details::text AS details_json,
               item_names::text AS item_names_json,
               ability_order::text AS ability_order_json,
               source_metadata::text AS source_metadata_json
        FROM brain.learned_builds WHERE id=$1
        "#,
        &[SqlParam::Int(learned_build_id)],
    )
    .await?;
    let Some(row) = row else {
        return Err(LearnError::InvalidInput(format!(
            "Kein learned_build mit id={learned_build_id} gefunden."
        )));
    };
    let mut build = row.clone();
    let tags = json_loads(get(&row, "tags_json").and_then(Value::as_str), json!([]));
    let details = json_loads(get(&row, "details_json").and_then(Value::as_str), json!({}));
    let item_names = json_loads(
        get(&row, "item_names_json").and_then(Value::as_str),
        json!([]),
    );
    let ability_order = json_loads(
        get(&row, "ability_order_json").and_then(Value::as_str),
        json!([]),
    );
    let source_metadata = json_loads(
        get(&row, "source_metadata_json").and_then(Value::as_str),
        json!({}),
    );
    let item_map = item_id_name_map(pool).await?;
    if let Some(object) = build.as_object_mut() {
        object.remove("tags_json");
        object.remove("details_json");
        object.remove("item_names_json");
        object.remove("ability_order_json");
        object.remove("source_metadata_json");
        object.insert("tags".to_string(), tags);
        object.insert("details".to_string(), details.clone());
        object.insert("item_names".to_string(), item_names);
        object.insert("ability_order".to_string(), ability_order);
        object.insert("source_metadata".to_string(), source_metadata);
        object.insert(
            "item_categories".to_string(),
            extract_item_categories(&details, &item_map),
        );
    }
    let hero_name = get_string(&build, "hero_name").unwrap_or_default();
    let hero_context = if !hero_name.is_empty() && !hero_name.starts_with("hero_id:") {
        build_hero_build_context(pool, &hero_name, &[], 80).await?
    } else {
        json!({})
    };
    let sibling_builds = sibling_build_summaries(pool, &hero_name, learned_build_id).await?;
    Ok(json!({
        "context_kind": "build_learning",
        "prompt_version": BUILD_LEARNING_PROMPT_VERSION,
        "build": build,
        "hero_context": hero_context,
        "nearby_top_builds": sibling_builds,
        "learning_goal": {
            "top_build_assumption": "Steam/GC top builds are treated as weak positive labels: top 1-3 usually good, top 4-10 often usable, lower ranks noisy.",
            "output_goal": "Extract reusable build principles, item timing, hero job, build variants and suspicious choices.",
        },
    }))
}

pub fn build_minimax_build_learning_request(
    context: &Value,
    config: &MiniMaxConfig,
) -> Result<ChatCompletionRequest> {
    let compact = compact_learning_context(context);
    let prompt = format!(
        "Analysiere diesen Deadlock Hero-Build als Trainingsbeispiel fuer ein Build-Brain.\n\
Schreibe Deutsch, aber lasse alle Hero-, Item-, Ability- und Stat-Namen exakt auf Englisch.\n\
Bewerte nicht nach erfundenen Winrates. Nutze nur den Kontext.\n\n\
Aufgaben:\n\
1. Ist der Build wahrscheinlich gut, situativ gut, mittel oder schlecht? Warum?\n\
2. Welche Hero-Aufgabe und Build-Variante erkennt man?\n\
3. Welche Items sind Lane/Core/Late/Situational und warum genau?\n\
4. Welche Kaufreihenfolge/Timing-Logik laesst sich ableiten?\n\
5. Welche allgemeinen Regeln soll das Brain fuer diesen Hero lernen?\n\
6. Welche Teile sind unsicher oder koennten nur Popularitaetsrauschen sein?\n\
7. Gib am Ende ein kompaktes JSON-Feld `insights` mit keys: \
hero_job, build_variant, core_items, situational_items, avoid_or_question, timing_rules, scoring_hints.\n\n\
Kontext JSON:\n{}",
        serde_json::to_string(&compact)?
    );
    Ok(ChatCompletionRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage::system(
                "Du bist ein strenger Deadlock Build-Analyst fuer ein deutsches Coaching-Brain. \
Du arbeitest datenbasiert, markierst Unsicherheit und verwandelst einzelne Builds in wiederverwendbare Regeln.",
            ),
            ChatMessage::user(prompt),
        ],
        max_completion_tokens: config.max_completion_tokens,
        temperature: config.temperature,
        top_p: config.top_p,
        stream: false,
    })
}

pub async fn learn_analyze_build(
    pool: &PgPool,
    options: LearnAnalyzeBuildOptions,
) -> Result<Value> {
    run_single_build_learning_analysis(
        pool,
        options.build_id,
        &options.config,
        options.dry_run,
        options.include_request,
    )
    .await
}

pub async fn learn_analyze_next(pool: &PgPool, options: LearnAnalyzeNextOptions) -> Result<Value> {
    let targets = list_pending_build_learning_targets(
        pool,
        options.hero.as_deref(),
        options.limit,
        Some(&options.config.model),
    )
    .await?;
    if !options.dry_run && !options.config.api_key_present() {
        return Err(LearnError::InvalidInput(
            "MiniMax API key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.".to_string(),
        ));
    }
    let mut results = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        match int_or_none(get(target, "id")) {
            Some(build_id) => match run_single_build_learning_analysis(
                pool,
                build_id,
                &options.config,
                options.dry_run,
                false,
            )
            .await
            {
                Ok(result) => {
                    let note = get(&result, "note").cloned().unwrap_or_else(|| json!({}));
                    results.push(json!({
                        "build_id": get(target, "id").cloned().unwrap_or(Value::Null),
                        "hero_name": get(target, "hero_name").cloned().unwrap_or(Value::Null),
                        "source_rank": get(target, "source_rank").cloned().unwrap_or(Value::Null),
                        "quality_tier": get(target, "quality_tier").cloned().unwrap_or(Value::Null),
                        "name": get(target, "name").cloned().unwrap_or(Value::Null),
                        "status": get(&note, "status")
                            .cloned()
                            .unwrap_or_else(|| if options.dry_run { json!("context_ready") } else { json!("analysis_ready") }),
                        "note_id": get(&note, "id").cloned().unwrap_or(Value::Null),
                        "provider_metadata": get(&result, "provider_metadata").cloned().unwrap_or(Value::Null),
                    }));
                }
                Err(error) => {
                    results.push(json!({
                        "build_id": get(target, "id").cloned().unwrap_or(Value::Null),
                        "hero_name": get(target, "hero_name").cloned().unwrap_or(Value::Null),
                        "source_rank": get(target, "source_rank").cloned().unwrap_or(Value::Null),
                        "name": get(target, "name").cloned().unwrap_or(Value::Null),
                        "status": "error",
                        "error": error.to_string(),
                    }));
                    if !options.dry_run {
                        break;
                    }
                }
            },
            None => results.push(json!({
                "build_id": Value::Null,
                "status": "error",
                "error": "missing build id",
            })),
        }
        if !options.dry_run && index + 1 < targets.len() && options.delay_seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(options.delay_seconds));
        }
    }
    Ok(json!({
        "dry_run": options.dry_run,
        "hero": options.hero,
        "limit": options.limit,
        "model": options.config.model,
        "api_key_present": options.config.api_key_present(),
        "pending_selected": targets.len(),
        "results": results,
    }))
}

async fn run_single_build_learning_analysis(
    pool: &PgPool,
    build_id: i64,
    config: &MiniMaxConfig,
    dry_run: bool,
    include_request: bool,
) -> Result<Value> {
    let context = build_learning_context(pool, build_id).await?;
    let request = build_minimax_build_learning_request(&context, config)?;
    let prompt_text = prompt_text_from_request(&request);
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    if dry_run {
        let note = save_build_learning_note(
            pool,
            &context,
            &prompt_text,
            None,
            Some(&config.model),
            "context_ready",
        )
        .await?;
        let mut result = json!({
            "dry_run": true,
            "build_id": build_id,
            "hero_name": get(&context, "build").and_then(|build| get(build, "hero_name")).cloned().unwrap_or(Value::Null),
            "model": config.model,
            "endpoint": endpoint,
            "api_key_present": config.api_key_present(),
            "note": note,
        });
        if include_request {
            if let Some(object) = result.as_object_mut() {
                object.insert("request".to_string(), serde_json::to_value(&request)?);
            }
        }
        return Ok(result);
    }

    let client = MiniMaxClient::new(config.clone())?;
    let response = client.chat(&request)?;
    let result_text = extract_minimax_text(&response);
    if result_text.is_empty() {
        return Err(LearnError::EmptyMiniMaxResponse);
    }
    let note = save_build_learning_note(
        pool,
        &context,
        &prompt_text,
        Some(&result_text),
        Some(&config.model),
        "analysis_ready",
    )
    .await?;
    Ok(json!({
        "build_id": build_id,
        "hero_name": get(&context, "build").and_then(|build| get(build, "hero_name")).cloned().unwrap_or(Value::Null),
        "model": config.model,
        "endpoint": endpoint,
        "note": note,
        "result_text": result_text,
        "provider_metadata": minimax_usage_summary(&response),
    }))
}

async fn save_build_learning_note(
    pool: &PgPool,
    context: &Value,
    prompt_text: &str,
    result_text: Option<&str>,
    model: Option<&str>,
    status: &str,
) -> Result<Value> {
    let context_json = serde_json::to_string(context)?;
    let context_hash = stable_hash_text(&context_json);
    let build = get(context, "build").cloned().unwrap_or_else(|| json!({}));
    let build_id = int_or_none(get(&build, "id"));
    let hero_name = get_string(&build, "hero_name");
    let source = get_string(&build, "source").unwrap_or_else(|| "unknown".to_string());
    let insights_json = serde_json::to_string(&extract_insights(result_text.unwrap_or("")))?;
    execute_sql(
        pool,
        r#"
        INSERT INTO brain.build_learning_notes(
          learned_build_id, hero_name, source, context_hash, prompt_version,
          prompt_text, result_text, insights, model, status, created_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8::text::jsonb, $9, $10, now(), now())
        ON CONFLICT (learned_build_id, context_hash, prompt_version, COALESCE(model, ''), status)
        DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          insights=excluded.insights,
          updated_at=excluded.updated_at
        "#,
        &[
            SqlParam::IntOpt(build_id),
            SqlParam::TextOpt(hero_name.clone()),
            SqlParam::Text(source),
            SqlParam::Text(context_hash.clone()),
            SqlParam::Text(BUILD_LEARNING_PROMPT_VERSION.to_string()),
            SqlParam::Text(prompt_text.to_string()),
            SqlParam::TextOpt(result_text.map(str::to_string)),
            SqlParam::Text(insights_json),
            SqlParam::TextOpt(model.map(str::to_string)),
            SqlParam::Text(status.to_string()),
        ],
    )
    .await?;
    Ok(query_one_json(
        pool,
        r#"
        SELECT id, learned_build_id, hero_name, source, model, status,
               extract(epoch from updated_at)::int8 AS updated_at
        FROM brain.build_learning_notes
        WHERE context_hash=$1 AND prompt_version=$2 AND COALESCE(model, '')=COALESCE($3, '') AND status=$4
        ORDER BY id DESC LIMIT 1
        "#,
        &[
            SqlParam::Text(context_hash.clone()),
            SqlParam::Text(BUILD_LEARNING_PROMPT_VERSION.to_string()),
            SqlParam::TextOpt(model.map(str::to_string)),
            SqlParam::Text(status.to_string()),
        ],
    )
    .await?
    .unwrap_or_else(|| json!({"context_hash": context_hash, "status": status})))
}

async fn load_steam_build_rows(
    pool: &PgPool,
    hero_map: &BTreeMap<i64, String>,
    hero: Option<&str>,
    language: i64,
    limit_per_hero: i64,
) -> Result<Vec<Value>> {
    let hero_ids = if let Some(hero) = hero.filter(|value| !value.trim().is_empty()) {
        let hero_key = hero.to_lowercase();
        let ids = hero_map
            .iter()
            .filter_map(|(hero_id, name)| {
                if name.to_lowercase().contains(&hero_key) {
                    Some(*hero_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        ids
    } else {
        Vec::new()
    };
    let bounded_limit = clamp_i64(limit_per_hero, 1, 50);
    if hero_ids.is_empty() {
        return query_json_rows(
            pool,
            r#"
            SELECT *
            FROM (
              SELECT hbs.*,
                     ROW_NUMBER() OVER (
                       PARTITION BY hbs.hero_id, hbs.language
                       ORDER BY COALESCE(hbs.last_seen_at, hbs.fetched_at, hbs.last_updated_ts, hbs.publish_ts, 0) DESC,
                                COALESCE(hbs.publish_ts, 0) DESC,
                                hbs.hero_build_id ASC
                     ) AS source_rank
              FROM hero_build_sources hbs
              WHERE hbs.language = $1
            ) ranked
            WHERE source_rank <= $2
            ORDER BY hero_id ASC, source_rank ASC
            "#,
            &[SqlParam::Int(language), SqlParam::Int(bounded_limit)],
        )
        .await;
    }
    query_json_rows(
        pool,
        r#"
        SELECT *
        FROM (
          SELECT hbs.*,
                 ROW_NUMBER() OVER (
                   PARTITION BY hbs.hero_id, hbs.language
                   ORDER BY COALESCE(hbs.last_seen_at, hbs.fetched_at, hbs.last_updated_ts, hbs.publish_ts, 0) DESC,
                            COALESCE(hbs.publish_ts, 0) DESC,
                            hbs.hero_build_id ASC
                 ) AS source_rank
          FROM hero_build_sources hbs
          WHERE hbs.language = $1 AND hbs.hero_id = ANY($2)
        ) ranked
        WHERE source_rank <= $3
        ORDER BY hero_id ASC, source_rank ASC
        "#,
        &[
            SqlParam::Int(language),
            SqlParam::IntArray(hero_ids),
            SqlParam::Int(bounded_limit),
        ],
    )
    .await
}

async fn hero_id_name_map(pool: &PgPool) -> Result<BTreeMap<i64, String>> {
    let rows = query_json_rows(
        pool,
        r#"
        SELECT payload::text AS payload_json
        FROM brain.entity_snapshots
        WHERE source='deadlock_assets_api' AND entity_type='hero'
        "#,
        &[],
    )
    .await?;
    let mut result = BTreeMap::new();
    for row in rows {
        let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        let hero_id = int_or_none(get(&payload, "id"));
        let name = get_string(&payload, "name").unwrap_or_default();
        if let Some(hero_id) = hero_id {
            if !name.is_empty() && get(&payload, "disabled").and_then(Value::as_bool) != Some(true)
            {
                result.insert(hero_id, name);
            }
        }
    }
    Ok(result)
}

async fn item_id_name_map(pool: &PgPool) -> Result<BTreeMap<i64, String>> {
    let rows = query_json_rows(
        pool,
        r#"
        SELECT payload::text AS payload_json
        FROM brain.entity_snapshots
        WHERE source='deadlock_assets_api' AND entity_type='item_or_ability'
        "#,
        &[],
    )
    .await?;
    let mut result = BTreeMap::new();
    for row in rows {
        let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        let item_id = int_or_none(get(&payload, "id"));
        let name = get_string(&payload, "name").unwrap_or_default();
        if let Some(item_id) = item_id {
            if !name.is_empty() {
                result.insert(item_id, name);
            }
        }
    }
    Ok(result)
}

fn extract_item_names(details: &Value, item_map: &BTreeMap<i64, String>) -> Vec<String> {
    let mut names = Vec::new();
    for category in get_any(details, &["mod_categories", "modCategories"])
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        for item in get(category, "mods")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let ability_id = int_or_none(get_any(item, &["ability_id", "abilityId"]));
            if let Some(name) = ability_id.and_then(|id| item_map.get(&id)) {
                if !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
    }
    names
}

fn extract_ability_order(details: &Value, item_map: &BTreeMap<i64, String>) -> Value {
    let ability_order = get_any(details, &["ability_order", "abilityOrder"])
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut result = Vec::new();
    for change in get_any(&ability_order, &["currency_changes", "currencyChanges"])
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let ability_id = int_or_none(get_any(change, &["ability_id", "abilityId"]));
        result.push(json!({
            "ability_id": ability_id,
            "ability_name": ability_id.and_then(|id| item_map.get(&id).cloned()),
            "currency_type": get_any(change, &["currency_type", "currencyType"]).cloned().unwrap_or(Value::Null),
            "delta": get(change, "delta").cloned().unwrap_or(Value::Null),
            "annotation": get(change, "annotation").cloned().unwrap_or(Value::Null),
        }));
    }
    Value::Array(result)
}

fn extract_item_categories(details: &Value, item_map: &BTreeMap<i64, String>) -> Value {
    let mut rows = Vec::new();
    for category in get_any(details, &["mod_categories", "modCategories"])
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let mut mods = Vec::new();
        for item in get(category, "mods")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let ability_id = int_or_none(get_any(item, &["ability_id", "abilityId"]));
            let item_name = ability_id.and_then(|id| item_map.get(&id).cloned());
            if let Some(item_name) = item_name {
                mods.push(json!({
                    "item": item_name,
                    "annotation": get(item, "annotation").cloned().unwrap_or(Value::Null),
                    "sell_priority": get_any(item, &["sell_priority", "sellPriority"]).cloned().unwrap_or(Value::Null),
                }));
            }
        }
        if !mods.is_empty() {
            rows.push(json!({
                "name": get(category, "name").cloned().unwrap_or(Value::Null),
                "description": get(category, "description").cloned().unwrap_or(Value::Null),
                "optional": get(category, "optional").and_then(Value::as_bool).unwrap_or(false),
                "items": mods,
            }));
        }
    }
    Value::Array(rows)
}

fn quality_from_rank(rank: i64) -> (&'static str, f64) {
    if rank <= 0 {
        ("unknown", 0.5)
    } else if rank <= 3 {
        ("likely_good", 0.9)
    } else if rank <= 10 {
        ("usable_noisy", 0.7)
    } else {
        ("low_confidence", 0.35)
    }
}

async fn sibling_build_summaries(pool: &PgPool, hero_name: &str, exclude_id: i64) -> Result<Value> {
    if hero_name.is_empty() {
        return Ok(json!([]));
    }
    let rows = query_json_rows(
        pool,
        r#"
        SELECT id, source_build_id, source_rank, quality_tier, quality_score, name,
               item_names::text AS item_names_json
        FROM brain.learned_builds
        WHERE hero_name=$1 AND id != $2
        ORDER BY quality_score DESC, source_rank ASC
        LIMIT 8
        "#,
        &[
            SqlParam::Text(hero_name.to_string()),
            SqlParam::Int(exclude_id),
        ],
    )
    .await?;
    Ok(Value::Array(
        rows.into_iter()
            .map(|mut row| {
                let item_names = json_loads(
                    get(&row, "item_names_json").and_then(Value::as_str),
                    json!([]),
                );
                if let Some(object) = row.as_object_mut() {
                    object.insert("item_names".to_string(), item_names);
                }
                row
            })
            .collect(),
    ))
}

fn compact_learning_context(context: &Value) -> Value {
    let hero_context = get(context, "hero_context")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let hero = get(&hero_context, "hero")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let build = get(&hero_context, "build")
        .cloned()
        .unwrap_or_else(|| json!({}));
    json!({
        "build": compact_build_for_model(get(context, "build")),
        "nearby_top_builds": get(context, "nearby_top_builds").cloned().unwrap_or(Value::Null),
        "learning_goal": get(context, "learning_goal").cloned().unwrap_or(Value::Null),
        "hero_understanding": {
            "name": get(&hero, "name").cloned().unwrap_or(Value::Null),
            "hero_type": get(&hero, "hero_type").cloned().unwrap_or(Value::Null),
            "role": get(&hero, "role").cloned().unwrap_or(Value::Null),
            "playstyle": get(&hero, "playstyle").cloned().unwrap_or(Value::Null),
            "inferred_gameplan": get(&hero, "inferred_gameplan").cloned().unwrap_or(Value::Null),
            "abilities": get(&hero, "abilities").cloned().unwrap_or(Value::Null),
            "sheet_hints": get(&hero, "sheet_hints").cloned().unwrap_or(Value::Null),
        },
        "deterministic_build_brain": {
            "plan": get(&build, "plan").cloned().unwrap_or(Value::Null),
            "early": get(&build, "early").cloned().unwrap_or(Value::Null),
            "core": get(&build, "core").cloned().unwrap_or(Value::Null),
            "late": get(&build, "late").cloned().unwrap_or(Value::Null),
            "situational": get(&build, "situational").cloned().unwrap_or(Value::Null),
            "shop_routes_to_4800": get(&build, "shop_routes_to_4800").cloned().unwrap_or(Value::Null),
        },
        "review_signals": get(&hero_context, "review_signals").cloned().unwrap_or(Value::Null),
    })
}

fn compact_build_for_model(build: Option<&Value>) -> Value {
    let Some(build) = build else {
        return json!({});
    };
    let keys = [
        "id",
        "source",
        "source_build_id",
        "hero_name",
        "language",
        "source_rank",
        "quality_tier",
        "quality_score",
        "name",
        "description",
        "tags",
        "item_names",
        "item_categories",
        "ability_order",
        "source_metadata",
    ];
    let mut object = Map::new();
    for key in keys {
        object.insert(
            key.to_string(),
            get(build, key).cloned().unwrap_or(Value::Null),
        );
    }
    Value::Object(object)
}
