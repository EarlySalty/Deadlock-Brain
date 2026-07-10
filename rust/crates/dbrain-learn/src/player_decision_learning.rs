use std::{collections::BTreeMap, thread, time::Duration};

use deadlock_brain_core::minimax::{
    extract_minimax_text, minimax_usage_summary, ChatCompletionRequest, ChatMessage, MiniMaxClient,
    MiniMaxConfig,
};
use rusqlite::Connection;
use serde_json::{json, Map, Value};

use crate::{
    build_optimizer::{
        asset_payload_by_id, build_hero_build_context, hero_name_from_id, item_summary,
        key_item_properties, specific_item_purpose,
    },
    util::{
        as_array, clamp_i64, compact_json, ensure_schema, extract_insights, get, get_any,
        get_string, int_or_zero, json_loads, now_epoch_seconds, numeric_value,
        prompt_text_from_request, query_json_rows, query_one_json, stable_hash_text,
        value_to_non_empty_string, value_to_string,
    },
    LearnError, Result,
};

pub const PLAYER_DECISION_PROMPT_VERSION: &str = "player_match_decision_de_v2";

#[derive(Debug, Clone)]
pub struct PlayerAnalyzeMatchOptions {
    pub account_id: String,
    pub match_id: String,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub include_request: bool,
}

#[derive(Debug, Clone)]
pub struct PlayerAnalyzeNextOptions {
    pub account_id: Option<String>,
    pub limit: i64,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub delay_seconds: f64,
}

pub fn player_list_matches(
    conn: &Connection,
    account_id: Option<&str>,
    limit: i64,
) -> Result<Vec<Value>> {
    let bounded_limit = clamp_i64(limit, 1, 500);
    let rows = if let Some(account_id) = account_id.filter(|value| !value.trim().is_empty()) {
        let like = format!("{account_id}:%");
        query_json_rows(
            conn,
            r#"
            SELECT external_id, canonical_name, payload_json, fetched_at
            FROM entity_snapshots
            WHERE source='statlocker' AND entity_type='statlocker_player_match'
              AND external_id LIKE ?1
            ORDER BY fetched_at DESC, id DESC LIMIT ?2
            "#,
            &[&like, &bounded_limit],
        )?
    } else {
        query_json_rows(
            conn,
            r#"
            SELECT external_id, canonical_name, payload_json, fetched_at
            FROM entity_snapshots
            WHERE source='statlocker' AND entity_type='statlocker_player_match'
            ORDER BY fetched_at DESC, id DESC LIMIT ?1
            "#,
            &[&bounded_limit],
        )?
    };
    Ok(rows
        .into_iter()
        .map(|row| {
            let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
            let source = get(&payload, "_deadlock_brain").cloned().unwrap_or_else(|| json!({}));
            let external = get_string(&row, "external_id").unwrap_or_default();
            let (account, match_id) = split_external_match_id(&external);
            json!({
                "account_id": get_string(&source, "account_id").unwrap_or(account),
                "match_id": get_string(&source, "match_id").unwrap_or(match_id),
                "hero_id": get(&source, "hero_id").cloned().or_else(|| hero_id_from_payload(&payload).map(Value::String)).unwrap_or(Value::Null),
                "fetched_at": get(&row, "fetched_at").cloned().unwrap_or(Value::Null),
                "summary": compact_match_row(&payload),
            })
        })
        .collect())
}

pub fn player_match_context(conn: &Connection, account_id: &str, match_id: &str) -> Result<Value> {
    build_player_match_decision_context(conn, account_id, match_id)
}

pub fn build_player_match_decision_context(
    conn: &Connection,
    account_id: &str,
    match_id: &str,
) -> Result<Value> {
    let safe_account_id = account_id.trim();
    let safe_match_id = match_id.trim();
    if safe_account_id.is_empty() || safe_match_id.is_empty() {
        return Err(LearnError::InvalidInput(
            "account_id und match_id sind erforderlich.".to_string(),
        ));
    }

    let player_match = latest_snapshot_payload(
        conn,
        "statlocker_player_match",
        &format!("{safe_account_id}:{safe_match_id}"),
    )?;
    if player_match.is_null() || player_match.as_object().map(Map::is_empty).unwrap_or(true) {
        return Err(LearnError::InvalidInput(format!(
            "Kein Statlocker Player-Match fuer {safe_account_id}:{safe_match_id} gefunden."
        )));
    }
    let profile = latest_snapshot_payload(conn, "statlocker_player_profile", safe_account_id)?;
    let match_detail = latest_snapshot_payload(conn, "statlocker_match_detail", safe_match_id)?;
    let deadlock_api_match = latest_deadlock_api_match_metadata(conn, safe_match_id)?;

    let hero_id = hero_id_from_payload(&player_match)
        .or_else(|| hero_id_from_match_detail(&match_detail, safe_account_id))
        .or_else(|| hero_id_from_deadlock_api_match(&deadlock_api_match, safe_account_id));
    let hero_name = hero_name_from_id(conn, hero_id.as_deref())?;
    let build_analysis = if let Some(hero_id) = hero_id.as_deref() {
        latest_snapshot_payload(
            conn,
            "statlocker_player_build_analysis",
            &format!("{safe_account_id}:{hero_id}"),
        )?
    } else {
        json!({})
    };
    let deterministic_context = if let Some(hero_name) = hero_name.as_deref() {
        match build_hero_build_context(conn, hero_name, &[], 60) {
            Ok(context) => context,
            Err(error) => json!({"error": error.to_string(), "hero_name": hero_name}),
        }
    } else {
        json!({})
    };

    Ok(json!({
        "context_kind": "player_match_decision_learning",
        "prompt_version": PLAYER_DECISION_PROMPT_VERSION,
        "account_id": safe_account_id,
        "match_id": safe_match_id,
        "hero_id": hero_id,
        "hero_name": hero_name,
        "player_profile": compact_profile(&profile),
        "player_match": compact_match_row(&player_match),
        "match_detail": compact_match_detail(&match_detail, safe_account_id),
        "deadlock_api_match": compact_deadlock_api_match(conn, &deadlock_api_match, safe_account_id)?,
        "player_build_analysis": compact_json(&build_analysis, 4, 25),
        "deterministic_build_brain": compact_hero_context(&deterministic_context),
        "learning_goal": {
            "source_assumption": "Deadlock API match metadata is treated as actual match evidence. Statlocker player_build_analysis is aggregate behavior across matches and must not be described as this exact match unless also present in the Deadlock API item timeline.",
            "focus": [
                "item order and timing",
                "specific item purpose for this hero and this game state",
                "hero ability function and scaling",
                "shop-bonus routes by Weapon/Vitality/Spirit spend",
                "standard core versus adaptation",
                "hero job and build variant",
                "game-state reasons for buys",
                "reusable coaching/build rules",
            ],
            "caution": "Do not invent winrates or hidden match data. Do not infer item sales unless sold_time_s is explicitly present and > 0. Mark facts as belegt, wahrscheinlich or unsicher.",
        },
    }))
}

pub fn build_minimax_player_match_decision_request(
    context: &Value,
    config: &MiniMaxConfig,
) -> Result<ChatCompletionRequest> {
    let compact = compact_json(context, 6, 40);
    let prompt = format!(
        "Analysiere dieses Deadlock Player-Match als Trainingsbeispiel fuer ein Coaching- und Build-Brain.\n\
Schreibe Deutsch, aber lasse alle Hero-, Item-, Ability-, Stat- und Map-/Mode-Namen exakt auf Englisch.\n\
Nutze nur den Kontext. Trenne harte Match-Fakten strikt von Aggregatmustern und Vermutungen.\n\
Prioritaet der Evidenz: 1) deadlock_api_match.actual_item_timeline und Match-Stats, \
2) Statlocker Player-/Build-Aggregate, 3) deterministic_build_brain als Regel-/Mechanik-Hilfe.\n\
Wenn sich Quellen widersprechen, zaehlt die Deadlock API fuer dieses konkrete Match.\n\
Item-ID-Mapping aus der Assets API ist verbindlich. Verwechsle z.B. Extra Stamina nicht mit Warp Stone.\n\
Beschreibe einen Sale nur, wenn `sold_time_s` im echten Match-Itemevent > 0 ist.\n\n\
Aufgaben:\n\
1. Hero-Verstaendnis: Was macht der Hero, welche Abilities tragen den Gameplan, welche Stats/Cooldowns/Actives skalieren ihn?\n\
2. Item-Verstaendnis: Fuer jedes wichtige Core-/T4-/Active-/verkaufte Item: welches konkrete Problem loest es hier fuer diesen Hero?\n\
3. Erklaere besonders Cooldown-Items sauber: Superior Cooldown = Ability-Fokus, Transcendent Cooldown = Abilities und Items; \
bewerte, ob der Hero und die gekauften Actives genug Cooldown-gebundenen Wert haben.\n\
4. Shop-Oekonomie: Welche Weapon/Vitality/Spirit Route entsteht, wann wird ungefaehr der 4800-Shop-Bonus erreicht, \
und ob die Route Lane/Core/Spike sinnvoll verbindet.\n\
5. Was ist belegt, was wahrscheinlich, was unsicher? Nutze diese Woerter explizit.\n\
6. Was wirkt wie Standard-Core, was wie situative Adaptation gegen Teamcomp/Game-State?\n\
7. Welche Build-Regeln soll das Brain daraus lernen, ohne sie blind auf jeden Hero zu kopieren?\n\
8. Gib am Ende ein kompaktes JSON-Feld `insights` mit keys: \
actual_match_facts, hero_job, hero_ability_scaling_rules, build_variant, item_purpose_rules, \
shop_bonus_rules, item_timing_rules, adaptation_rules, uncertain_inferences, coaching_takeaways, data_gaps.\n\n\
Kontext JSON:\n{}",
        serde_json::to_string(&compact)?
    );
    Ok(ChatCompletionRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage::system(
                "Du bist ein strenger Deadlock Match- und Build-Analyst fuer einen deutschen Discord. \
Du wandelst beobachtete High-Skill-Matches in wiederverwendbare, datenbasierte Regeln um. \
Keine erfundenen Zahlen, keine ungekennzeichneten Vermutungen.",
            ),
            ChatMessage::user(prompt),
        ],
        max_completion_tokens: config.max_completion_tokens,
        temperature: config.temperature,
        top_p: config.top_p,
        stream: false,
    })
}

pub fn player_analyze_match(
    conn: &Connection,
    options: PlayerAnalyzeMatchOptions,
) -> Result<Value> {
    run_single_player_match_analysis(
        conn,
        &options.account_id,
        &options.match_id,
        &options.config,
        options.dry_run,
        options.include_request,
    )
}

pub fn player_analyze_next(conn: &Connection, options: PlayerAnalyzeNextOptions) -> Result<Value> {
    let targets = list_pending_player_match_decision_targets(
        conn,
        options.account_id.as_deref(),
        options.limit,
        Some(&options.config.model),
    )?;
    if !options.dry_run && !options.config.api_key_present() {
        return Err(LearnError::InvalidInput(
            "MiniMax API key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.".to_string(),
        ));
    }
    let mut results = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        let account_id = get_string(target, "account_id").unwrap_or_default();
        let match_id = get_string(target, "match_id").unwrap_or_default();
        match run_single_player_match_analysis(
            conn,
            &account_id,
            &match_id,
            &options.config,
            options.dry_run,
            false,
        ) {
            Ok(result) => {
                let note = get(&result, "note").cloned().unwrap_or_else(|| json!({}));
                results.push(json!({
                    "account_id": get(target, "account_id").cloned().unwrap_or(Value::Null),
                    "match_id": get(target, "match_id").cloned().unwrap_or(Value::Null),
                    "hero_id": get(&result, "hero_id").cloned().or_else(|| get(target, "hero_id").cloned()).unwrap_or(Value::Null),
                    "hero_name": get(&result, "hero_name").cloned().unwrap_or(Value::Null),
                    "status": get(&note, "status")
                        .cloned()
                        .unwrap_or_else(|| if options.dry_run { json!("context_ready") } else { json!("analysis_ready") }),
                    "note_id": get(&note, "id").cloned().unwrap_or(Value::Null),
                    "provider_metadata": get(&result, "provider_metadata").cloned().unwrap_or(Value::Null),
                }));
            }
            Err(error) => {
                results.push(json!({
                    "account_id": get(target, "account_id").cloned().unwrap_or(Value::Null),
                    "match_id": get(target, "match_id").cloned().unwrap_or(Value::Null),
                    "hero_id": get(target, "hero_id").cloned().unwrap_or(Value::Null),
                    "status": "error",
                    "error": error.to_string(),
                }));
                if !options.dry_run {
                    break;
                }
            }
        }
        if !options.dry_run && index + 1 < targets.len() && options.delay_seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(options.delay_seconds));
        }
    }
    Ok(json!({
        "dry_run": options.dry_run,
        "account_id": options.account_id,
        "limit": options.limit,
        "model": options.config.model,
        "api_key_present": options.config.api_key_present(),
        "pending_selected": targets.len(),
        "results": results,
    }))
}

pub(crate) fn list_pending_player_match_decision_targets(
    conn: &Connection,
    account_id: Option<&str>,
    limit: i64,
    model: Option<&str>,
) -> Result<Vec<Value>> {
    ensure_schema(conn)?;
    let bounded_limit = clamp_i64(limit, 1, 100);
    let rows = if let Some(account_id) = account_id.filter(|value| !value.trim().is_empty()) {
        let like = format!("{account_id}:%");
        query_json_rows(
            conn,
            r#"
            SELECT pm.external_id, pm.payload_json, pm.fetched_at
            FROM entity_snapshots pm
            WHERE pm.source='statlocker'
              AND pm.entity_type='statlocker_player_match'
              AND NOT EXISTS (
                SELECT 1
                FROM player_match_decision_notes n
                WHERE n.account_id = substr(pm.external_id, 1, instr(pm.external_id, ':') - 1)
                  AND n.match_id = substr(pm.external_id, instr(pm.external_id, ':') + 1)
                  AND n.status = 'analysis_ready'
                  AND n.prompt_version = ?1
                  AND COALESCE(n.model, '') = COALESCE(?2, '')
              )
              AND pm.external_id LIKE ?3
            ORDER BY pm.fetched_at DESC, pm.id DESC LIMIT ?4
            "#,
            &[
                &PLAYER_DECISION_PROMPT_VERSION,
                &model,
                &like,
                &bounded_limit,
            ],
        )?
    } else {
        query_json_rows(
            conn,
            r#"
            SELECT pm.external_id, pm.payload_json, pm.fetched_at
            FROM entity_snapshots pm
            WHERE pm.source='statlocker'
              AND pm.entity_type='statlocker_player_match'
              AND NOT EXISTS (
                SELECT 1
                FROM player_match_decision_notes n
                WHERE n.account_id = substr(pm.external_id, 1, instr(pm.external_id, ':') - 1)
                  AND n.match_id = substr(pm.external_id, instr(pm.external_id, ':') + 1)
                  AND n.status = 'analysis_ready'
                  AND n.prompt_version = ?1
                  AND COALESCE(n.model, '') = COALESCE(?2, '')
              )
            ORDER BY pm.fetched_at DESC, pm.id DESC LIMIT ?3
            "#,
            &[&PLAYER_DECISION_PROMPT_VERSION, &model, &bounded_limit],
        )?
    };
    let mut targets = Vec::new();
    for row in rows {
        let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        let source = get(&payload, "_deadlock_brain")
            .cloned()
            .unwrap_or_else(|| json!({}));
        let external = get_string(&row, "external_id").unwrap_or_default();
        let (account, match_id) = split_external_match_id(&external);
        let account_id_value = get_string(&source, "account_id").unwrap_or(account);
        let match_id_value = get_string(&source, "match_id").unwrap_or(match_id);
        if account_id_value.is_empty() || match_id_value.is_empty() {
            continue;
        }
        let hero_id = get(&source, "hero_id")
            .map(value_to_string)
            .filter(|value| !value.is_empty())
            .or_else(|| hero_id_from_payload(&payload));
        targets.push(json!({
            "account_id": account_id_value,
            "match_id": match_id_value,
            "hero_id": hero_id,
            "fetched_at": get(&row, "fetched_at").cloned().unwrap_or(Value::Null),
            "summary": compact_match_row(&payload),
        }));
    }
    Ok(targets)
}

fn run_single_player_match_analysis(
    conn: &Connection,
    account_id: &str,
    match_id: &str,
    config: &MiniMaxConfig,
    dry_run: bool,
    include_request: bool,
) -> Result<Value> {
    let context = build_player_match_decision_context(conn, account_id, match_id)?;
    let request = build_minimax_player_match_decision_request(&context, config)?;
    let prompt_text = prompt_text_from_request(&request);
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    if dry_run {
        let note = save_player_match_decision_note(
            conn,
            &context,
            &prompt_text,
            None,
            Some(&config.model),
            "context_ready",
        )?;
        let mut result = json!({
            "dry_run": true,
            "account_id": account_id,
            "match_id": match_id,
            "hero_id": get(&context, "hero_id").cloned().unwrap_or(Value::Null),
            "hero_name": get(&context, "hero_name").cloned().unwrap_or(Value::Null),
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
    let note = save_player_match_decision_note(
        conn,
        &context,
        &prompt_text,
        Some(&result_text),
        Some(&config.model),
        "analysis_ready",
    )?;
    Ok(json!({
        "account_id": account_id,
        "match_id": match_id,
        "hero_id": get(&context, "hero_id").cloned().unwrap_or(Value::Null),
        "hero_name": get(&context, "hero_name").cloned().unwrap_or(Value::Null),
        "model": config.model,
        "endpoint": endpoint,
        "note": note,
        "result_text": result_text,
        "provider_metadata": minimax_usage_summary(&response),
    }))
}

fn save_player_match_decision_note(
    conn: &Connection,
    context: &Value,
    prompt_text: &str,
    result_text: Option<&str>,
    model: Option<&str>,
    status: &str,
) -> Result<Value> {
    ensure_schema(conn)?;
    let context_json = serde_json::to_string(context)?;
    let context_hash = stable_hash_text(&context_json);
    let now = now_epoch_seconds()?;
    let hero_id = get(context, "hero_id")
        .map(value_to_string)
        .filter(|value| !value.is_empty());
    let hero_name = get_string(context, "hero_name");
    let insights_json = serde_json::to_string(&extract_insights(result_text.unwrap_or("")))?;
    let account_id = get_string(context, "account_id").unwrap_or_default();
    let match_id = get_string(context, "match_id").unwrap_or_default();
    conn.execute(
        r#"
        INSERT INTO player_match_decision_notes(
          account_id, match_id, hero_id, hero_name, context_hash, prompt_version,
          prompt_text, result_text, insights_json, model, status, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          insights_json=excluded.insights_json,
          hero_id=excluded.hero_id,
          hero_name=excluded.hero_name,
          updated_at=excluded.updated_at
        "#,
        (
            account_id.as_str(),
            match_id.as_str(),
            hero_id.as_deref(),
            hero_name.as_deref(),
            context_hash.as_str(),
            PLAYER_DECISION_PROMPT_VERSION,
            prompt_text,
            result_text,
            insights_json.as_str(),
            model,
            status,
            now,
            now,
        ),
    )?;
    Ok(query_one_json(
        conn,
        r#"
        SELECT id, account_id, match_id, hero_id, hero_name, model, status, updated_at
        FROM player_match_decision_notes
        WHERE account_id=?1 AND match_id=?2 AND context_hash=?3 AND prompt_version=?4
          AND COALESCE(model, '')=COALESCE(?5, '') AND status=?6
        ORDER BY id DESC LIMIT 1
        "#,
        &[
            &account_id,
            &match_id,
            &context_hash,
            &PLAYER_DECISION_PROMPT_VERSION,
            &model,
            &status,
        ],
    )?
    .unwrap_or_else(|| json!({"context_hash": context_hash, "status": status})))
}

fn latest_snapshot_payload(
    conn: &Connection,
    entity_type: &str,
    external_id: &str,
) -> Result<Value> {
    let row = query_one_json(
        conn,
        r#"
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='statlocker' AND entity_type=?1 AND external_id=?2
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        "#,
        &[&entity_type, &external_id],
    )?;
    Ok(row
        .and_then(|row| {
            get(&row, "payload_json")
                .and_then(Value::as_str)
                .map(|raw| json_loads(Some(raw), json!({})))
        })
        .unwrap_or_else(|| json!({})))
}

fn latest_deadlock_api_match_metadata(conn: &Connection, match_id: &str) -> Result<Value> {
    let row = query_one_json(
        conn,
        r#"
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_api' AND entity_type='deadlock_api_match_metadata' AND external_id=?1
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        "#,
        &[&match_id],
    )?;
    Ok(row
        .and_then(|row| {
            get(&row, "payload_json")
                .and_then(Value::as_str)
                .map(|raw| json_loads(Some(raw), json!({})))
        })
        .unwrap_or_else(|| json!({})))
}

fn hero_id_from_deadlock_api_match(payload: &Value, account_id: &str) -> Option<String> {
    let player = deadlock_api_player(payload, account_id);
    get_any(&player, &["hero_id", "heroId"])
        .map(value_to_string)
        .filter(|value| !value.trim().is_empty())
}

fn compact_deadlock_api_match(
    conn: &Connection,
    payload: &Value,
    account_id: &str,
) -> Result<Value> {
    if !payload.is_object() {
        return Ok(json!({}));
    }
    let player = deadlock_api_player(payload, account_id);
    let players = get(payload, "players")
        .and_then(as_array)
        .cloned()
        .unwrap_or_default();
    let mut team_roster = Vec::new();
    for row in &players {
        team_roster.push(compact_deadlock_api_player_roster_row(conn, row)?);
    }
    let mut compact = json!({
        "evidence_layer": "actual_match_facts_from_deadlock_api",
        "match_id": get_any(payload, &["match_id", "matchId"]).cloned().unwrap_or(Value::Null),
        "start_time": get_any(payload, &["start_time", "startTime"]).cloned().unwrap_or(Value::Null),
        "duration_s": get_any(payload, &["duration_s", "duration"]).cloned().unwrap_or(Value::Null),
        "winning_team": get_any(payload, &["winning_team", "winningTeam"]).cloned().unwrap_or(Value::Null),
        "match_outcome": get_any(payload, &["match_outcome", "matchOutcome"]).cloned().unwrap_or(Value::Null),
        "match_mode": get_any(payload, &["match_mode", "matchMode"]).cloned().unwrap_or(Value::Null),
        "game_mode": get_any(payload, &["game_mode", "gameMode"]).cloned().unwrap_or(Value::Null),
        "average_badges": {
            "team0": get(payload, "average_badge_team0").cloned().unwrap_or(Value::Null),
            "team1": get(payload, "average_badge_team1").cloned().unwrap_or(Value::Null),
        },
        "teams": team_roster,
        "data_quality_notes": [
            "actual_item_timeline kommt aus Deadlock API match metadata und ist fuer dieses Match priorisiert",
            "player_build_analysis ist Statlocker-Aggregat ueber mehrere Matches und darf nicht als exakte Timeline behandelt werden",
            "upgrade_id/imbued_ability_id werden soweit moeglich ueber Assets API aufgeloest; unbekannte IDs bleiben unsicher",
        ],
    });
    if player.is_object() && !player.as_object().map(Map::is_empty).unwrap_or(true) {
        if let Some(object) = compact.as_object_mut() {
            object.insert(
                "player".to_string(),
                compact_deadlock_api_player(conn, &player, payload)?,
            );
        }
    }
    Ok(compact_json(&compact, 6, 80))
}

fn deadlock_api_player(payload: &Value, account_id: &str) -> Value {
    for row in get(payload, "players")
        .and_then(as_array)
        .into_iter()
        .flatten()
    {
        let row_account = get_any(row, &["account_id", "accountId"]).map(value_to_string);
        if row_account.as_deref() == Some(account_id) {
            return row.clone();
        }
    }
    json!({})
}

fn compact_deadlock_api_player_roster_row(conn: &Connection, row: &Value) -> Result<Value> {
    let hero_id = get_any(row, &["hero_id", "heroId"]).map(value_to_string);
    Ok(json!({
        "account_id": get_any(row, &["account_id", "accountId"]).cloned().unwrap_or(Value::Null),
        "team": get(row, "team").cloned().unwrap_or(Value::Null),
        "player_slot": get_any(row, &["player_slot", "playerSlot"]).cloned().unwrap_or(Value::Null),
        "assigned_lane": get_any(row, &["assigned_lane", "assignedLane"]).cloned().unwrap_or(Value::Null),
        "hero_id": hero_id,
        "hero_name": hero_name_from_id(conn, hero_id.as_deref())?,
        "kills": get(row, "kills").cloned().unwrap_or(Value::Null),
        "deaths": get(row, "deaths").cloned().unwrap_or(Value::Null),
        "assists": get(row, "assists").cloned().unwrap_or(Value::Null),
        "net_worth": get_any(row, &["net_worth", "netWorth"]).cloned().unwrap_or(Value::Null),
    }))
}

fn compact_deadlock_api_player(
    conn: &Connection,
    player: &Value,
    match_payload: &Value,
) -> Result<Value> {
    let hero_id = get_any(player, &["hero_id", "heroId"]).map(value_to_string);
    let raw_events = get(player, "items")
        .and_then(as_array)
        .cloned()
        .unwrap_or_default();
    let mut all_events = Vec::new();
    for item in raw_events {
        all_events.push(compact_actual_item_event(conn, &item)?);
    }
    let mut item_timeline = all_events
        .iter()
        .filter(|event| is_public_shop_item_event(event))
        .cloned()
        .collect::<Vec<_>>();
    let mut ability_events = all_events
        .into_iter()
        .filter(|event| !is_public_shop_item_event(event))
        .collect::<Vec<_>>();
    item_timeline.sort_by(|left, right| {
        numeric_value(get(left, "game_time_s"))
            .partial_cmp(&numeric_value(get(right, "game_time_s")))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    ability_events.sort_by(|left, right| {
        numeric_value(get(left, "game_time_s"))
            .partial_cmp(&numeric_value(get(right, "game_time_s")))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(json!({
        "account_id": get_any(player, &["account_id", "accountId"]).cloned().unwrap_or(Value::Null),
        "hero_id": hero_id,
        "hero_name": hero_name_from_id(conn, hero_id.as_deref())?,
        "team": get(player, "team").cloned().unwrap_or(Value::Null),
        "player_slot": get_any(player, &["player_slot", "playerSlot"]).cloned().unwrap_or(Value::Null),
        "assigned_lane": get_any(player, &["assigned_lane", "assignedLane"]).cloned().unwrap_or(Value::Null),
        "scoreline": {
            "kills": get(player, "kills").cloned().unwrap_or(Value::Null),
            "deaths": get(player, "deaths").cloned().unwrap_or(Value::Null),
            "assists": get(player, "assists").cloned().unwrap_or(Value::Null),
            "net_worth": get_any(player, &["net_worth", "netWorth"]).cloned().unwrap_or(Value::Null),
        },
        "enemy_heroes": enemy_heroes(conn, match_payload, get(player, "team"))?,
        "ally_heroes": ally_heroes(conn, match_payload, get(player, "team"), get_any(player, &["account_id", "accountId"]))?,
        "actual_item_timeline": item_timeline,
        "ability_or_non_shop_events": ability_events.into_iter().take(30).collect::<Vec<_>>(),
        "shop_bonus_actual_spend_by_slot_approx": shop_bonus_progress(&item_timeline),
        "stat_checkpoints": stat_checkpoints(get(player, "stats")),
        "death_details": compact_death_details(conn, get_any(player, &["death_details", "deathDetails"]), match_payload)?,
        "objective_events": compact_objectives(get(match_payload, "objectives")),
    }))
}

fn compact_actual_item_event(conn: &Connection, row: &Value) -> Result<Value> {
    let item_id = get_any(row, &["item_id", "itemId"]).map(value_to_string);
    let upgrade_id = get_any(row, &["upgrade_id", "upgradeId"]).map(value_to_string);
    let imbued_id = get_any(row, &["imbued_ability_id", "imbuedAbilityId"]).map(value_to_string);
    let payload = asset_payload_by_id(conn, item_id.as_deref())?.unwrap_or_else(|| json!({}));
    let upgrade_payload =
        asset_payload_by_id(conn, upgrade_id.as_deref())?.unwrap_or_else(|| json!({}));
    let item = item_mechanics(&payload, item_id.as_deref());
    let upgrade = if upgrade_payload.is_object()
        && !upgrade_payload
            .as_object()
            .map(Map::is_empty)
            .unwrap_or(true)
    {
        item_mechanics(&upgrade_payload, upgrade_id.as_deref())
    } else {
        json!({})
    };
    let mut event = json!({
        "evidence": "deadlock_api_match_metadata.players.items",
        "game_time_s": get_any(row, &["game_time_s", "gameTimeS"]).cloned().unwrap_or(Value::Null),
        "item_id": item_id,
        "item_name": get(&item, "name").cloned().unwrap_or(Value::Null),
        "slot": get(&item, "slot").cloned().unwrap_or(Value::Null),
        "tier": get(&item, "tier").cloned().unwrap_or(Value::Null),
        "cost": get(&item, "cost").cloned().unwrap_or(Value::Null),
        "is_active": get(&item, "is_active").cloned().unwrap_or(Value::Null),
        "archetypes": get(&item, "archetypes").cloned().unwrap_or(Value::Null),
        "specific_purpose": specific_item_purpose(&item),
        "description": get(&item, "description").cloned().unwrap_or(Value::Null),
        "key_properties": get(&item, "key_properties").cloned().unwrap_or(Value::Null),
        "upgrade_id": upgrade_id,
        "upgrade_name": get(&upgrade, "name").cloned().unwrap_or(Value::Null),
        "imbued_ability_id": imbued_id,
        "imbued_ability_name": ability_name_from_id(conn, imbued_id.as_deref())?,
        "sold_time_s": get_any(row, &["sold_time_s", "soldTimeS"]).cloned().unwrap_or(Value::Null),
        "flags": get(row, "flags").cloned().unwrap_or(Value::Null),
    });
    if numeric_value(get(&event, "sold_time_s")) <= 0.0 {
        if let Some(object) = event.as_object_mut() {
            object.remove("sold_time_s");
        }
    }
    Ok(remove_empty_fields(event))
}

fn item_mechanics(payload: &Value, item_id: Option<&str>) -> Value {
    if !payload.is_object() || payload.as_object().map(Map::is_empty).unwrap_or(true) {
        return json!({"name": item_id.map(|id| format!("unknown:{id}"))});
    }
    let item = item_summary(payload);
    json!({
        "name": get(&item, "name").cloned().unwrap_or(Value::Null),
        "class_name": get(&item, "class_name").cloned().unwrap_or(Value::Null),
        "slot": get(&item, "slot").cloned().unwrap_or(Value::Null),
        "tier": get(&item, "tier").cloned().unwrap_or(Value::Null),
        "cost": get(&item, "cost").cloned().unwrap_or(Value::Null),
        "is_active": get(&item, "is_active").cloned().unwrap_or(Value::Null),
        "activation": get(&item, "activation").cloned().unwrap_or(Value::Null),
        "description": get(&item, "description").cloned().unwrap_or(Value::Null),
        "archetypes": get(&item, "archetypes").cloned().unwrap_or(Value::Null),
        "key_properties": key_item_properties(get(&item, "properties")),
    })
}

fn ability_name_from_id(conn: &Connection, ability_id: Option<&str>) -> Result<Option<String>> {
    Ok(asset_payload_by_id(conn, ability_id)?.and_then(|payload| get_string(&payload, "name")))
}

fn is_public_shop_item_event(event: &Value) -> bool {
    let slot = get_string(event, "slot").unwrap_or_default();
    let cost = numeric_value(get(event, "cost"));
    matches!(slot.as_str(), "weapon" | "vitality" | "spirit") && cost > 0.0
}

fn shop_bonus_progress(item_timeline: &[Value]) -> Value {
    let mut gross_spend_by_slot: Map<String, Value> = Map::new();
    let mut purchases_by_slot: Map<String, Value> = Map::new();
    let mut running: BTreeMap<String, i64> = BTreeMap::new();
    let mut first_4800_gross = Map::new();
    for event in item_timeline {
        let slot = get_string(event, "slot").unwrap_or_default();
        let cost = numeric_value(get(event, "cost")) as i64;
        if !matches!(slot.as_str(), "weapon" | "vitality" | "spirit") || cost <= 0 {
            continue;
        }
        let current = running.get(&slot).copied().unwrap_or(0) + cost;
        running.insert(slot.clone(), current);
        gross_spend_by_slot.insert(slot.clone(), json!(current));
        let purchase = json!({
            "time_s": get(event, "game_time_s").cloned().unwrap_or(Value::Null),
            "item": get(event, "item_name").cloned().unwrap_or(Value::Null),
            "cost": cost,
            "sold_time_s": get(event, "sold_time_s").cloned().unwrap_or(Value::Null),
            "gross_running_spend": current,
        });
        if let Some(rows) = purchases_by_slot
            .entry(slot.clone())
            .or_insert_with(|| json!([]))
            .as_array_mut()
        {
            rows.push(purchase);
        }
        if !first_4800_gross.contains_key(&slot) && current >= 4800 {
            first_4800_gross.insert(
                slot,
                get(event, "game_time_s").cloned().unwrap_or(Value::Null),
            );
        }
    }
    let current_value = shop_current_value_progress(item_timeline);
    json!({
        "note": "gross spend is useful for purchase tempo; current value subtracts explicit sold_time_s events and is safer for shop-bonus state. Upgrade/component interpretation can still be uncertain.",
        "target_spike_per_slot": 4800,
        "gross_spend_by_slot": Value::Object(gross_spend_by_slot),
        "first_4800_time_s_gross": Value::Object(first_4800_gross),
        "current_value_by_slot_after_sales_approx": get(&current_value, "current_value_by_slot").cloned().unwrap_or(Value::Null),
        "first_4800_time_s_current_value_approx": get(&current_value, "first_4800_time_s").cloned().unwrap_or(Value::Null),
        "current_value_events": get(&current_value, "events").cloned().unwrap_or(Value::Null),
        "purchases_by_slot": Value::Object(purchases_by_slot),
    })
}

fn shop_current_value_progress(item_timeline: &[Value]) -> Value {
    let mut events = Vec::new();
    for item in item_timeline {
        let slot = get_string(item, "slot").unwrap_or_default();
        let cost = numeric_value(get(item, "cost")) as i64;
        if !matches!(slot.as_str(), "weapon" | "vitality" | "spirit") || cost <= 0 {
            continue;
        }
        events.push(json!({
            "time_s": numeric_value(get(item, "game_time_s")),
            "sort": 1,
            "slot": slot,
            "delta": cost,
            "item": get(item, "item_name").cloned().unwrap_or(Value::Null),
            "event": "buy",
        }));
        let sold_time = numeric_value(get(item, "sold_time_s"));
        if sold_time > 0.0 {
            events.push(json!({
                "time_s": sold_time,
                "sort": 0,
                "slot": slot,
                "delta": -cost,
                "item": get(item, "item_name").cloned().unwrap_or(Value::Null),
                "event": "sold",
            }));
        }
    }
    events.sort_by(|left, right| {
        numeric_value(get(left, "time_s"))
            .partial_cmp(&numeric_value(get(right, "time_s")))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| int_or_zero(get(left, "sort")).cmp(&int_or_zero(get(right, "sort"))))
    });
    let mut current_by_slot: Map<String, Value> = Map::new();
    let mut first_4800 = Map::new();
    let mut compact_events = Vec::new();
    for event in events {
        let slot = get_string(&event, "slot").unwrap_or_default();
        let current = (current_by_slot
            .get(&slot)
            .and_then(Value::as_i64)
            .unwrap_or(0)
            + int_or_zero(get(&event, "delta")))
        .max(0);
        current_by_slot.insert(slot.clone(), json!(current));
        if !first_4800.contains_key(&slot) && current >= 4800 {
            first_4800.insert(
                slot.clone(),
                get(&event, "time_s").cloned().unwrap_or(Value::Null),
            );
        }
        compact_events.push(json!({
            "time_s": get(&event, "time_s").cloned().unwrap_or(Value::Null),
            "slot": slot,
            "event": get(&event, "event").cloned().unwrap_or(Value::Null),
            "item": get(&event, "item").cloned().unwrap_or(Value::Null),
            "delta": get(&event, "delta").cloned().unwrap_or(Value::Null),
            "current_value": current,
        }));
    }
    json!({
        "current_value_by_slot": Value::Object(current_by_slot),
        "first_4800_time_s": Value::Object(first_4800),
        "events": compact_events.into_iter().take(80).collect::<Vec<_>>(),
    })
}

fn enemy_heroes(conn: &Connection, match_payload: &Value, team: Option<&Value>) -> Result<Value> {
    let mut rows = Vec::new();
    for row in get(match_payload, "players")
        .and_then(as_array)
        .into_iter()
        .flatten()
    {
        if get(row, "team") != team {
            rows.push(compact_deadlock_api_player_roster_row(conn, row)?);
        }
    }
    Ok(Value::Array(rows))
}

fn ally_heroes(
    conn: &Connection,
    match_payload: &Value,
    team: Option<&Value>,
    account_id: Option<&Value>,
) -> Result<Value> {
    let mut rows = Vec::new();
    let account_text = account_id.map(value_to_string).unwrap_or_default();
    for row in get(match_payload, "players")
        .and_then(as_array)
        .into_iter()
        .flatten()
    {
        if get(row, "team") == team
            && get_any(row, &["account_id", "accountId"])
                .map(value_to_string)
                .unwrap_or_default()
                != account_text
        {
            rows.push(compact_deadlock_api_player_roster_row(conn, row)?);
        }
    }
    Ok(Value::Array(rows))
}

fn stat_checkpoints(stats: Option<&Value>) -> Value {
    let rows = stats.and_then(as_array).cloned().unwrap_or_default();
    if rows.is_empty() {
        return json!([]);
    }
    let last = rows.len().saturating_sub(1);
    let mut indexes = vec![
        0,
        rows.len() / 4,
        rows.len() / 2,
        (rows.len() * 3) / 4,
        last,
    ];
    indexes.sort_unstable();
    indexes.dedup();
    let keys = [
        "time_stamp_s",
        "timeStampS",
        "net_worth",
        "netWorth",
        "player_damage",
        "playerDamage",
        "player_damage_taken",
        "playerDamageTaken",
        "tech_power",
        "techPower",
        "weapon_power",
        "weaponPower",
        "max_health",
        "maxHealth",
        "kills",
        "deaths",
        "assists",
    ];
    Value::Array(
        indexes
            .into_iter()
            .filter_map(|index| rows.get(index))
            .map(|row| {
                let mut object = Map::new();
                for key in keys {
                    if let Some(value) = get(row, key) {
                        object.insert(key.to_string(), value.clone());
                    }
                }
                Value::Object(object)
            })
            .collect(),
    )
}

fn compact_death_details(
    conn: &Connection,
    death_details: Option<&Value>,
    match_payload: &Value,
) -> Result<Value> {
    let death_rows = death_details
        .and_then(as_array)
        .cloned()
        .unwrap_or_default();
    let mut players_by_slot: Map<String, Value> = Map::new();
    for row in get(match_payload, "players")
        .and_then(as_array)
        .into_iter()
        .flatten()
    {
        let slot = get_any(row, &["player_slot", "playerSlot"])
            .map(value_to_string)
            .unwrap_or_default();
        players_by_slot.insert(slot, row.clone());
    }
    let mut result = Vec::new();
    for row in death_rows.iter().take(12) {
        let killer_slot = get_any(
            row,
            &["killer_player_slot", "killerPlayerSlot", "killer_slot"],
        )
        .map(value_to_string);
        let killer = killer_slot
            .as_ref()
            .and_then(|slot| players_by_slot.get(slot));
        let killer_hero_id = killer
            .and_then(|killer| get_any(killer, &["hero_id", "heroId"]))
            .map(value_to_string);
        result.push(json!({
            "game_time_s": get_any(row, &["game_time_s", "gameTimeS"]).cloned().unwrap_or(Value::Null),
            "killer_player_slot": killer_slot,
            "killer_hero_name": hero_name_from_id(conn, killer_hero_id.as_deref())?,
            "time_to_kill_s": get_any(row, &["time_to_kill_s", "timeToKillS"]).cloned().unwrap_or(Value::Null),
            "damage_taken": get_any(row, &["damage_taken", "damageTaken"]).cloned().unwrap_or(Value::Null),
            "raw": compact_json(row, 2, 8),
        }));
    }
    Ok(Value::Array(result))
}

fn compact_objectives(objectives: Option<&Value>) -> Value {
    let keys = [
        "game_time_s",
        "gameTimeS",
        "team",
        "objective_id",
        "objectiveId",
        "destroyed",
        "damage",
        "player_slot",
        "playerSlot",
    ];
    Value::Array(
        objectives
            .and_then(as_array)
            .into_iter()
            .flatten()
            .take(20)
            .map(|row| {
                let mut object = Map::new();
                for key in keys {
                    if let Some(value) = get(row, key) {
                        object.insert(key.to_string(), value.clone());
                    }
                }
                Value::Object(object)
            })
            .collect(),
    )
}

fn hero_id_from_payload(payload: &Value) -> Option<String> {
    for key in [
        "hero_id",
        "heroId",
        "player_hero_id",
        "playerHeroId",
        "hero",
    ] {
        if let Some(value) = get(payload, key).and_then(value_to_non_empty_string) {
            return Some(value);
        }
    }
    get(payload, "_deadlock_brain")
        .and_then(|source| get(source, "hero_id"))
        .and_then(value_to_non_empty_string)
}

fn hero_id_from_match_detail(payload: &Value, account_id: &str) -> Option<String> {
    for row in walk_dicts(payload) {
        let row_account =
            get_any(row, &["accountId", "account_id", "playerAccountId"]).map(value_to_string);
        if row_account.as_deref() != Some(account_id) {
            continue;
        }
        if let Some(hero_id) = hero_id_from_payload(row) {
            return Some(hero_id);
        }
    }
    None
}

fn compact_profile(payload: &Value) -> Value {
    if !payload.is_object() {
        return json!({});
    }
    object_pick(
        payload,
        &[
            "accountId",
            "account_id",
            "name",
            "personaName",
            "rank",
            "rankedRank",
            "badgeLevel",
            "leaderboardRank",
        ],
    )
}

fn compact_match_row(payload: &Value) -> Value {
    if !payload.is_object() {
        return json!({});
    }
    let mut compact = object_pick(
        payload,
        &[
            "match_id",
            "matchId",
            "startTime",
            "start_time",
            "duration",
            "duration_s",
            "hero_id",
            "heroId",
            "netWorth",
            "net_worth",
            "kills",
            "deaths",
            "assists",
            "playerScore",
            "player_score",
            "matchResult",
            "match_result",
            "won",
            "items",
            "itemBuild",
            "item_build",
            "abilityOrder",
            "ability_order",
        ],
    );
    if let Some(source) = get(payload, "_deadlock_brain") {
        if let Some(object) = compact.as_object_mut() {
            object.insert("_deadlock_brain".to_string(), source.clone());
        }
    }
    compact_json(&compact, 4, 30)
}

fn compact_match_detail(payload: &Value, account_id: &str) -> Value {
    if !payload.is_object() {
        return json!({});
    }
    let mut compact = object_pick(
        payload,
        &[
            "match_id",
            "matchId",
            "duration",
            "duration_s",
            "startTime",
            "start_time",
            "winningTeam",
            "winning_team",
        ],
    );
    let player_rows = walk_dicts(payload)
        .into_iter()
        .filter(|row| {
            get_any(row, &["accountId", "account_id", "playerAccountId"])
                .map(value_to_string)
                .as_deref()
                == Some(account_id)
        })
        .map(|row| compact_json(row, 3, 30))
        .take(4)
        .collect::<Vec<_>>();
    if !player_rows.is_empty() {
        if let Some(object) = compact.as_object_mut() {
            object.insert("matched_player_rows".to_string(), Value::Array(player_rows));
        }
    }
    compact
}

fn compact_hero_context(context: &Value) -> Value {
    let hero = get(context, "hero").cloned().unwrap_or_else(|| json!({}));
    let build = get(context, "build").cloned().unwrap_or_else(|| json!({}));
    json!({
        "hero": {
            "name": get(&hero, "name").cloned().unwrap_or(Value::Null),
            "hero_type": get(&hero, "hero_type").cloned().unwrap_or(Value::Null),
            "role": get(&hero, "role").cloned().unwrap_or(Value::Null),
            "playstyle": get(&hero, "playstyle").cloned().unwrap_or(Value::Null),
            "inferred_gameplan": get(&hero, "inferred_gameplan").cloned().unwrap_or(Value::Null),
            "abilities": get(&hero, "abilities").cloned().unwrap_or(Value::Null),
            "sheet_hints": get(&hero, "sheet_hints").cloned().unwrap_or(Value::Null),
        },
        "build": {
            "plan": get(&build, "plan").cloned().unwrap_or(Value::Null),
            "lane_decision_model": get(&build, "lane_decision_model").cloned().unwrap_or(Value::Null),
            "early": get(&build, "early").cloned().unwrap_or(Value::Null),
            "core": get(&build, "core").cloned().unwrap_or(Value::Null),
            "late": get(&build, "late").cloned().unwrap_or(Value::Null),
            "situational": get(&build, "situational").cloned().unwrap_or(Value::Null),
            "shop_routes_to_4800": get(&build, "shop_routes_to_4800").cloned().unwrap_or(Value::Null),
        },
        "statlocker_signals": get(context, "statlocker_signals").cloned().unwrap_or(Value::Null),
        "review_signals": get(context, "review_signals").cloned().unwrap_or(Value::Null),
        "error": get(context, "error").cloned().unwrap_or(Value::Null),
    })
}

fn walk_dicts(value: &Value) -> Vec<&Value> {
    let mut rows = Vec::new();
    walk_dicts_inner(value, &mut rows);
    rows
}

fn walk_dicts_inner<'a>(value: &'a Value, rows: &mut Vec<&'a Value>) {
    if value.is_object() {
        rows.push(value);
        if let Some(object) = value.as_object() {
            for item in object.values() {
                walk_dicts_inner(item, rows);
            }
        }
    } else if let Some(array) = value.as_array() {
        for item in array {
            walk_dicts_inner(item, rows);
        }
    }
}

fn object_pick(payload: &Value, keys: &[&str]) -> Value {
    let mut object = Map::new();
    for key in keys {
        if let Some(value) = get(payload, key) {
            object.insert((*key).to_string(), value.clone());
        }
    }
    Value::Object(object)
}

fn split_external_match_id(external: &str) -> (String, String) {
    if let Some((account, match_id)) = external.split_once(':') {
        (account.to_string(), match_id.to_string())
    } else {
        (external.to_string(), String::new())
    }
}

fn remove_empty_fields(value: Value) -> Value {
    let Some(object) = value.as_object() else {
        return value;
    };
    Value::Object(
        object
            .iter()
            .filter(|(_, value)| match value {
                Value::Null => false,
                Value::String(text) => !text.is_empty(),
                Value::Array(rows) => !rows.is_empty(),
                Value::Object(map) => !map.is_empty(),
                Value::Bool(_) | Value::Number(_) => true,
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    )
}
