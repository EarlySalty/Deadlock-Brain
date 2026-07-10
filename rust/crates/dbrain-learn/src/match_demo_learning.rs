use std::collections::{BTreeMap, BTreeSet};

use deadlock_brain_core::minimax::{
    extract_minimax_text, minimax_usage_summary, ChatCompletionRequest, ChatMessage, MiniMaxClient,
    MiniMaxConfig,
};
use serde_json::{json, Map, Value};
use sqlx::PgPool;

use crate::{
    build_optimizer::{asset_payload_by_id, hero_name_from_id, item_summary, load_entity_payload},
    util::{
        compact_json, execute_sql, get, prompt_text_from_request, query_json_rows, query_one_json,
        stable_hash_text, value_to_string, SqlParam,
    },
    LearnError, Result,
};

const TOP_LEVEL_SECTIONS: [&str; 10] = [
    "metadata",
    "phases",
    "economy_build",
    "combat",
    "macro",
    "strong_decisions",
    "mistakes",
    "turning_points",
    "hypotheses",
    "data_gaps",
];
const PHASES: [&str; 4] = ["lane", "transition", "midgame", "late_game"];
const DECISION_TEXT_FIELDS: [&str; 6] = [
    "observed_state",
    "action",
    "effects",
    "interpreted_intent",
    "evaluation",
    "alternative",
];
const DECISION_FIELDS: [&str; 10] = [
    "tick",
    "time_seconds",
    "observed_state",
    "action",
    "effects",
    "interpreted_intent",
    "evaluation",
    "alternative",
    "confidence",
    "evidence_ids",
];
const FIGHT_WINDOW_GAP_TICKS: u64 = 900;
const REVIEW_LABELS: [&str; 4] = [
    "falsch",
    "unbelegt",
    "Kontext fehlt",
    "wichtige Entscheidung uebersehen",
];
pub const DEMO_REPORT_PROMPT_VERSION: &str = "mo_full_report_de_v2";
const DEMO_EVIDENCE_QUERY_VERSION: &str = "mo_full_report_v1";

#[derive(Debug, Clone)]
pub struct DemoAnalyzeMatchOptions {
    pub account_id: String,
    pub match_id: String,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub include_request: bool,
}

struct DemoContextParts {
    account_id: String,
    match_id: String,
    hero_name: String,
    player_match: Value,
    match_metadata: Value,
    demo_evidence: Value,
    patch: Option<Value>,
    hero_mechanics: Value,
    asset_resolution: Value,
    correction_rules: Vec<Value>,
}

struct BuiltDemoContext {
    context: Value,
    evidence_ids: BTreeSet<String>,
}

pub async fn demo_analyze_match(pool: &PgPool, options: DemoAnalyzeMatchOptions) -> Result<Value> {
    let account_id = required_numeric_id(&options.account_id, "account_id")?;
    let match_id = required_numeric_id(&options.match_id, "match_id")?;
    let built = load_demo_context(pool, &account_id, &match_id).await?;
    let request = build_demo_report_request(&built.context, &options.config)?;
    let prompt_text = prompt_text_from_request(&request);
    let endpoint = format!(
        "{}/chat/completions",
        options.config.base_url.trim_end_matches('/')
    );
    if options.dry_run {
        let note = save_demo_report_note(
            pool,
            &built.context,
            &prompt_text,
            None,
            None,
            Some(&options.config.model),
            "context_ready",
        )
        .await?;
        let mut result = json!({
            "dry_run": true,
            "account_id": account_id,
            "match_id": match_id,
            "model": options.config.model,
            "endpoint": endpoint,
            "api_key_present": options.config.api_key_present(),
            "evidence_count": built.evidence_ids.len(),
            "raw_evidence_count": built.context["evidence_registry"]["raw_count"],
            "derived_evidence_count": built.context["evidence_registry"]["derived_count"],
            "patch_provenance": built.context["report_metadata"]["patch"].clone(),
            "note": note,
            "context": built.context
        });
        if options.include_request {
            let result = result.as_object_mut().ok_or_else(|| {
                LearnError::InvalidInput("Dry-Run-Ergebnis ist kein Objekt.".to_string())
            })?;
            result.insert("request".to_string(), serde_json::to_value(&request)?);
        }
        return Ok(result);
    }

    let config = options.config.clone();
    let model_request = request.clone();
    let response = tokio::task::spawn_blocking(move || {
        let client = MiniMaxClient::new(config)?;
        client.chat(&model_request)
    })
    .await
    .map_err(|error| {
        LearnError::InvalidInput(format!("Fireworks-Worker abgebrochen: {error}"))
    })??;
    let raw_report = extract_minimax_text(&response);
    if raw_report.trim().is_empty() {
        return Err(LearnError::EmptyMiniMaxResponse);
    }
    let report = parse_and_validate_model_report(
        &raw_report,
        &built.context["report_metadata"],
        &built.evidence_ids,
    )?;
    let rendered_report = render_demo_report(&report)?;
    let note = save_demo_report_note(
        pool,
        &built.context,
        &prompt_text,
        Some(&rendered_report),
        Some(&report),
        Some(&options.config.model),
        "calibration_pending",
    )
    .await?;
    let mut result = json!({
        "dry_run": false,
        "account_id": account_id,
        "match_id": match_id,
        "model": options.config.model,
        "endpoint": endpoint,
        "evidence_count": built.evidence_ids.len(),
        "raw_evidence_count": built.context["evidence_registry"]["raw_count"],
        "derived_evidence_count": built.context["evidence_registry"]["derived_count"],
        "patch_provenance": built.context["report_metadata"]["patch"].clone(),
        "note": note,
        "report": report,
        "rendered_report": rendered_report,
        "provider_metadata": minimax_usage_summary(&response)
    });
    if options.include_request {
        let result = result.as_object_mut().ok_or_else(|| {
            LearnError::InvalidInput("Analyse-Ergebnis ist kein Objekt.".to_string())
        })?;
        result.insert("request".to_string(), serde_json::to_value(&request)?);
    }
    Ok(result)
}

async fn load_demo_context(
    pool: &PgPool,
    account_id: &str,
    match_id: &str,
) -> Result<BuiltDemoContext> {
    let player_match = latest_deadlock_snapshot(
        pool,
        "deadlock_api_player_match",
        &format!("{account_id}:{match_id}"),
    )
    .await?;
    let match_metadata =
        latest_deadlock_snapshot(pool, "deadlock_api_match_metadata", match_id).await?;
    let demo_evidence = latest_deadlock_snapshot(
        pool,
        "deadlock_api_demo_evidence",
        &format!("{account_id}:{match_id}:{DEMO_EVIDENCE_QUERY_VERSION}"),
    )
    .await?;
    let target_player = match_metadata
        .get("players")
        .and_then(Value::as_array)
        .and_then(|players| {
            players.iter().find(|player| {
                player.get("account_id").map(value_text).as_deref() == Some(account_id)
            })
        })
        .ok_or_else(|| {
            LearnError::InvalidInput("Zielspieler fehlt in Match-Metadaten.".to_string())
        })?;
    let hero_id = target_player
        .get("hero_id")
        .map(value_text)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| LearnError::InvalidInput("Zielspieler hat keine hero_id.".to_string()))?;
    let hero_name = hero_name_from_id(pool, Some(&hero_id))
        .await?
        .unwrap_or_else(|| format!("Hero {hero_id}"));
    let patch = load_active_patch(pool, match_metadata.get("start_time")).await?;
    let hero_mechanics = load_entity_payload(pool, &hero_name, "hero")
        .await?
        .map(|value| compact_json(&value, 5, 50))
        .unwrap_or_else(|| json!({"hero_id": hero_id, "data_gap": "Hero asset missing"}));
    let asset_resolution =
        resolve_demo_assets(pool, account_id, &match_metadata, &demo_evidence).await?;
    let correction_rules = load_correction_rules(pool).await?;
    build_demo_context_from_parts(DemoContextParts {
        account_id: account_id.to_string(),
        match_id: match_id.to_string(),
        hero_name,
        player_match,
        match_metadata,
        demo_evidence,
        patch,
        hero_mechanics,
        asset_resolution,
        correction_rules,
    })
}

async fn latest_deadlock_snapshot(
    pool: &PgPool,
    entity_type: &str,
    external_id: &str,
) -> Result<Value> {
    let row = query_one_json(
        pool,
        r#"
        SELECT payload::text AS payload_json
        FROM brain.entity_snapshots
        WHERE source='deadlock_api' AND entity_type=$1 AND external_id=$2
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        "#,
        &[
            SqlParam::Text(entity_type.to_string()),
            SqlParam::Text(external_id.to_string()),
        ],
    )
    .await?
    .ok_or_else(|| {
        LearnError::InvalidInput(format!("Snapshot {entity_type}:{external_id} fehlt."))
    })?;
    let raw = get(&row, "payload_json")
        .and_then(Value::as_str)
        .ok_or_else(|| LearnError::InvalidInput("Snapshot-Payload fehlt.".to_string()))?;
    let payload: Value = serde_json::from_str(raw)?;
    if !payload.is_object() {
        return invalid(format!(
            "Snapshot {entity_type}:{external_id} ist kein Objekt."
        ));
    }
    Ok(payload)
}

async fn load_active_patch(pool: &PgPool, start_time: Option<&Value>) -> Result<Option<Value>> {
    let Some(start_time) = start_time.filter(|value| !value.is_null()).map(value_text) else {
        return Ok(None);
    };
    query_one_json(
        pool,
        r#"
        SELECT patch_external_id, patch_title, posted_at::text AS posted_at
        FROM brain.patch_events
        WHERE posted_at IS NOT NULL
          AND posted_at <= CASE
            WHEN $1 ~ '^[0-9]+$' THEN to_timestamp($1::double precision)
            ELSE $1::timestamptz
          END
        ORDER BY posted_at DESC, id DESC
        LIMIT 1
        "#,
        &[SqlParam::Text(start_time)],
    )
    .await
}

async fn resolve_demo_assets(
    pool: &PgPool,
    account_id: &str,
    match_metadata: &Value,
    demo_evidence: &Value,
) -> Result<Value> {
    let (ids, item_ids) = collect_demo_asset_ids(account_id, match_metadata, demo_evidence);
    let mut assets = BTreeMap::new();
    for id in ids.into_iter().take(150) {
        if let Some(payload) = asset_payload_by_id(pool, Some(&id)).await? {
            let is_item = item_ids.contains(&id);
            assets.insert(
                id.clone(),
                compact_asset_for_context(&payload, &id, is_item),
            );
        }
    }
    Ok(serde_json::to_value(assets)?)
}

fn collect_demo_asset_ids(
    account_id: &str,
    match_metadata: &Value,
    demo_evidence: &Value,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut ids = BTreeSet::new();
    let mut item_ids = BTreeSet::new();
    if let Some(players) = match_metadata.get("players").and_then(Value::as_array) {
        for player in players.iter().filter(|player| {
            player.get("account_id").map(value_text).as_deref() == Some(account_id)
        }) {
            if let Some(items) = player.get("items").and_then(Value::as_array) {
                for item in items {
                    for key in ["item_id", "upgrade_id"] {
                        if let Some(id) = item
                            .get(key)
                            .and_then(Value::as_u64)
                            .filter(|id| *id > 1_000)
                            .map(|id| id.to_string())
                        {
                            ids.insert(id.clone());
                            item_ids.insert(id);
                        }
                    }
                    if let Some(id) = item
                        .get("imbued_ability_id")
                        .and_then(Value::as_u64)
                        .filter(|id| *id > 1_000)
                        .map(|id| id.to_string())
                    {
                        ids.insert(id);
                    }
                }
            }
        }
    }
    if let Some(rows) = demo_evidence.get("rows").and_then(Value::as_array) {
        for row in rows {
            if let Some(id) = row
                .get("ability_id")
                .and_then(Value::as_u64)
                .filter(|id| *id > 1_000)
                .map(|id| id.to_string())
            {
                ids.insert(id.clone());
                if row.get("event_type").and_then(Value::as_str)
                    == Some("ItemPurchaseNotificationEvent")
                {
                    item_ids.insert(id);
                }
            }
        }
    }
    (ids, item_ids)
}

fn compact_asset_for_context(payload: &Value, id: &str, is_item: bool) -> Value {
    if !is_item {
        return json!({
            "id": id,
            "name": payload.get("name").cloned().unwrap_or(Value::Null),
            "class_name": payload.get("class_name").cloned().unwrap_or(Value::Null),
            "type": payload.get("type").cloned().unwrap_or(Value::Null),
            "hero": payload.get("hero").cloned().unwrap_or(Value::Null)
        });
    }
    let mut summary = item_summary(payload);
    if let Some(summary) = summary.as_object_mut() {
        summary.insert("id".to_string(), json!(id));
        summary.insert(
            "type".to_string(),
            payload.get("type").cloned().unwrap_or(Value::Null),
        );
        summary.remove("activation");
        summary.remove("upgrades");
        if let Some(description) = summary.get_mut("description") {
            if let Some(text) = description.as_str() {
                *description = Value::String(truncate_chars(text, 600));
            }
        }
        if let Some(properties) = summary.get_mut("properties").and_then(Value::as_array_mut) {
            properties.sort_by_key(|property| {
                !property
                    .get("important")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            });
            properties.truncate(10);
            for property in properties {
                if let Some(property) = property.as_object_mut() {
                    property.retain(|key, _| {
                        matches!(
                            key.as_str(),
                            "name" | "label" | "value" | "prefix" | "postfix" | "important"
                        )
                    });
                }
            }
        }
    }
    summary
}

fn truncate_chars(value: &str, limit: usize) -> String {
    let mut chars = value.chars();
    let shortened = chars.by_ref().take(limit).collect::<String>();
    if chars.next().is_some() {
        format!("{shortened}...")
    } else {
        shortened
    }
}

async fn load_correction_rules(pool: &PgPool) -> Result<Vec<Value>> {
    let rows = query_json_rows(
        pool,
        r#"
        SELECT payload::text AS payload_json
        FROM brain.entity_snapshots
        WHERE source='human_review' AND entity_type='player_match_report_review'
        ORDER BY fetched_at DESC, id DESC
        LIMIT 20
        "#,
        &[],
    )
    .await?;
    let mut rules = Vec::new();
    for row in rows {
        if let Some(raw) = get(&row, "payload_json").and_then(Value::as_str) {
            rules.push(compact_json(&serde_json::from_str(raw)?, 4, 30));
        }
    }
    Ok(rules)
}

async fn save_demo_report_note(
    pool: &PgPool,
    context: &Value,
    prompt_text: &str,
    rendered_report: Option<&str>,
    report: Option<&Value>,
    model: Option<&str>,
    status: &str,
) -> Result<Value> {
    let context_hash = demo_note_context_hash(context, report)?;
    let account_id = value_to_string(&context["account_id"]);
    let match_id = value_to_string(&context["match_id"]);
    let hero_value = &context["match_metadata"]["target_player"]["hero_id"];
    let hero_id = (!hero_value.is_null()).then(|| value_to_string(hero_value));
    let hero_name = context["report_metadata"]["hero"]
        .as_str()
        .map(str::to_string);
    let empty_report = json!({});
    let insights = serde_json::to_string(report.unwrap_or(&empty_report))?;
    execute_sql(
        pool,
        r#"
        INSERT INTO brain.player_match_decision_notes(
          account_id, match_id, hero_id, hero_name, context_hash, prompt_version,
          prompt_text, result_text, insights, model, status, created_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9::text::jsonb, $10, $11, now(), now())
        ON CONFLICT (account_id, match_id, context_hash, prompt_version, COALESCE(model, ''), status)
        DO UPDATE SET prompt_text=excluded.prompt_text, result_text=excluded.result_text,
          insights=excluded.insights, hero_id=excluded.hero_id, hero_name=excluded.hero_name,
          updated_at=excluded.updated_at
        "#,
        &[
            SqlParam::Text(account_id.clone()),
            SqlParam::Text(match_id.clone()),
            SqlParam::TextOpt(hero_id),
            SqlParam::TextOpt(hero_name),
            SqlParam::Text(context_hash.clone()),
            SqlParam::Text(DEMO_REPORT_PROMPT_VERSION.to_string()),
            SqlParam::Text(prompt_text.to_string()),
            SqlParam::TextOpt(rendered_report.map(str::to_string)),
            SqlParam::Text(insights),
            SqlParam::TextOpt(model.map(str::to_string)),
            SqlParam::Text(status.to_string()),
        ],
    )
    .await?;
    Ok(query_one_json(
        pool,
        r#"
        SELECT id, account_id, match_id, hero_id, hero_name, model, status,
               extract(epoch from updated_at)::int8 AS updated_at
        FROM brain.player_match_decision_notes
        WHERE account_id=$1 AND match_id=$2 AND context_hash=$3 AND prompt_version=$4
          AND COALESCE(model, '')=COALESCE($5, '') AND status=$6
        ORDER BY id DESC LIMIT 1
        "#,
        &[
            SqlParam::Text(account_id),
            SqlParam::Text(match_id),
            SqlParam::Text(context_hash.clone()),
            SqlParam::Text(DEMO_REPORT_PROMPT_VERSION.to_string()),
            SqlParam::TextOpt(model.map(str::to_string)),
            SqlParam::Text(status.to_string()),
        ],
    )
    .await?
    .unwrap_or_else(|| json!({"context_hash": context_hash, "status": status})))
}

fn demo_note_context_hash(context: &Value, report: Option<&Value>) -> Result<String> {
    let payload = match report {
        Some(report) => json!({"context": context, "validated_report": report}),
        None => context.clone(),
    };
    Ok(stable_hash_text(&serde_json::to_string(&payload)?))
}

fn required_numeric_id(value: &str, name: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return invalid(format!("{name} muss eine numerische ID sein."));
    }
    Ok(value.to_string())
}

struct FightWindow {
    start_tick: u64,
    end_tick: u64,
    damage_dealt: f64,
    damage_taken: f64,
    kills: u64,
    deaths: u64,
    interrupted_abilities: Vec<Value>,
    important_abilities: Vec<Value>,
    stamina_events: Vec<Value>,
    involved_entity_ids: BTreeSet<u64>,
    ability_ids: BTreeSet<u64>,
    evidence_ids: Vec<String>,
}

impl FightWindow {
    fn new(tick: u64) -> Self {
        Self {
            start_tick: tick,
            end_tick: tick,
            damage_dealt: 0.0,
            damage_taken: 0.0,
            kills: 0,
            deaths: 0,
            interrupted_abilities: Vec::new(),
            important_abilities: Vec::new(),
            stamina_events: Vec::new(),
            involved_entity_ids: BTreeSet::new(),
            ability_ids: BTreeSet::new(),
            evidence_ids: Vec::new(),
        }
    }

    fn push(&mut self, row: &Value, target_entity: u64) -> Result<()> {
        let row = object(row, "target_combat row")?;
        let tick = field(row, "tick", "target_combat row")?
            .as_u64()
            .ok_or_else(|| LearnError::InvalidInput("target_combat tick fehlt.".to_string()))?;
        self.end_tick = tick;
        let attacker = row.get("attacker_entity").and_then(Value::as_u64);
        let victim = row.get("victim_entity").and_then(Value::as_u64);
        let damage = row.get("damage").and_then(Value::as_f64).unwrap_or(0.0);
        if attacker == Some(target_entity) {
            self.damage_dealt += damage;
        }
        if victim == Some(target_entity) {
            self.damage_taken += damage;
        }
        match row.get("event_type").and_then(Value::as_str).unwrap_or_default() {
            "HeroKilledEvent" => {
                if attacker == Some(target_entity) {
                    self.kills += 1;
                }
                if victim == Some(target_entity) {
                    self.deaths += 1;
                }
            }
            "AbilityInterruptedEvent" => self.interrupted_abilities.push(json!({
                "tick": tick,
                "ability_id": row.get("ability_id").cloned().unwrap_or(Value::Null),
                "hero_id_interrupter": row.get("hero_id_interrupter").cloned().unwrap_or(Value::Null),
                "evidence_id": row.get("evidence_id").cloned().unwrap_or(Value::Null)
            })),
            "ImportantAbilityUsedEvent" => self.important_abilities.push(json!({
                "tick": tick,
                "ability_name": row.get("ability_name").cloned().unwrap_or(Value::Null),
                "evidence_id": row.get("evidence_id").cloned().unwrap_or(Value::Null)
            })),
            "StaminaConsumedEvent" => self.stamina_events.push(json!({
                "tick": tick,
                "stamina_before": row.get("stamina_before").cloned().unwrap_or(Value::Null),
                "stamina_after": row.get("stamina_after").cloned().unwrap_or(Value::Null),
                "stamina_drained": row.get("stamina_drained").cloned().unwrap_or(Value::Null),
                "evidence_id": row.get("evidence_id").cloned().unwrap_or(Value::Null)
            })),
            _ => {}
        }
        for key in [
            "attacker_entity",
            "victim_entity",
            "inflictor_entity",
            "ability_entity",
        ] {
            if let Some(entity) = row.get(key).and_then(Value::as_u64) {
                self.involved_entity_ids.insert(entity);
            }
        }
        if let Some(ability) = row.get("ability_id").and_then(Value::as_u64) {
            self.ability_ids.insert(ability);
        }
        let evidence_id = text(
            field(row, "evidence_id", "target_combat row")?,
            "target_combat.evidence_id",
        )?;
        self.evidence_ids.push(evidence_id.to_string());
        Ok(())
    }

    fn into_value(self, match_id: &str, index: usize) -> Value {
        json!({
            "evidence_id": format!("fight_window:{match_id}:{index:06}"),
            "start_tick": self.start_tick,
            "end_tick": self.end_tick,
            "damage_dealt": self.damage_dealt,
            "damage_taken": self.damage_taken,
            "kills": self.kills,
            "deaths": self.deaths,
            "interrupted_abilities": self.interrupted_abilities,
            "important_abilities": self.important_abilities,
            "stamina_events": self.stamina_events,
            "involved_entity_ids": self.involved_entity_ids,
            "ability_ids": self.ability_ids,
            "evidence_ids": self.evidence_ids
        })
    }
}

fn compact_demo_evidence(rows: &[Value], target_entity: u64, match_id: &str) -> Result<Value> {
    let mut player_state = Vec::new();
    let mut combat = Vec::new();
    let mut economy_objectives = Vec::new();
    for row in rows {
        match row.get("query_name").and_then(Value::as_str) {
            Some("player_state") => player_state.push(compact_player_state(row)),
            Some("target_combat") => combat.push(row),
            Some("economy_objectives") => economy_objectives.push(without_source_envelope(row)),
            Some(other) => return invalid(format!("Unbekannter Demo-Query-Typ {other}.")),
            None => return invalid("Demo-Evidenzzeile ohne query_name."),
        }
    }
    player_state.sort_by_key(row_tick);
    economy_objectives.sort_by_key(row_tick);
    combat.sort_by_key(|row| row_tick(row));

    let mut windows = Vec::new();
    let mut current: Option<FightWindow> = None;
    for row in combat {
        let tick = row_tick(row);
        if current
            .as_ref()
            .is_some_and(|window| tick.saturating_sub(window.end_tick) > FIGHT_WINDOW_GAP_TICKS)
        {
            if let Some(window) = current.take() {
                let index = windows.len() + 1;
                windows.push(window.into_value(match_id, index));
            }
        }
        let window = current.get_or_insert_with(|| FightWindow::new(tick));
        window.push(row, target_entity)?;
    }
    if let Some(window) = current {
        let index = windows.len() + 1;
        windows.push(window.into_value(match_id, index));
    }
    Ok(json!({
        "player_state": player_state,
        "fight_windows": windows,
        "economy_objectives": economy_objectives
    }))
}

fn row_tick(row: &Value) -> u64 {
    row.get("tick").and_then(Value::as_u64).unwrap_or(u64::MAX)
}

fn compact_player_state(row: &Value) -> Value {
    let mut compact = Map::new();
    for key in [
        "evidence_id",
        "tick",
        "pawn_entity_index",
        "assigned_lane",
        "hero_id",
        "health",
        "health_max",
        "level",
        "net_worth",
        "kills",
        "assists",
        "deaths",
        "last_hits",
        "denies",
        "hero_damage",
        "objective_damage",
        "cell_x",
        "cell_y",
        "cell_z",
        "upgrades",
    ] {
        if let Some(value) = row.get(key) {
            compact.insert(key.to_string(), value.clone());
        }
    }
    Value::Object(compact)
}

fn without_source_envelope(row: &Value) -> Value {
    let mut compact = row.as_object().cloned().unwrap_or_default();
    for key in ["query_name", "query_version", "match_id"] {
        compact.remove(key);
    }
    Value::Object(compact)
}

fn build_demo_context_from_parts(parts: DemoContextParts) -> Result<BuiltDemoContext> {
    let rows = parts
        .demo_evidence
        .get("rows")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            LearnError::InvalidInput("Demo-Snapshot enthaelt keine rows.".to_string())
        })?;
    let mut evidence_ids = BTreeSet::new();
    for row in rows {
        let id = row
            .get("evidence_id")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| LearnError::InvalidInput("Demo-Zeile ohne evidence_id.".to_string()))?;
        if !evidence_ids.insert(id.to_string()) {
            return invalid(format!("Doppelte Demo-Evidenz-ID {id}."));
        }
    }
    let target_entity = rows
        .iter()
        .find(|row| row.get("query_name").and_then(Value::as_str) == Some("player_state"))
        .and_then(|row| row.get("pawn_entity_index"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            LearnError::InvalidInput("Demo-Snapshot enthaelt keine Ziel-Pawn-Entity.".to_string())
        })?;
    let report_metadata = deterministic_report_metadata(
        &parts.account_id,
        &parts.match_id,
        &parts.hero_name,
        &parts.match_metadata,
        parts.patch.as_ref(),
    )?;
    let raw_evidence_count = evidence_ids.len();
    let evidence = compact_demo_evidence(rows, target_entity, &parts.match_id)?;
    if let Some(windows) = evidence.get("fight_windows").and_then(Value::as_array) {
        for window in windows {
            let id = window
                .get("evidence_id")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    LearnError::InvalidInput("Fightfenster ohne evidence_id.".to_string())
                })?;
            if !evidence_ids.insert(id.to_string()) {
                return invalid(format!("Doppelte abgeleitete Evidenz-ID {id}."));
            }
        }
    }
    let match_metadata = compact_match_metadata(&parts.match_metadata, &parts.account_id)?;
    let context = json!({
        "context_kind": "mo_match_demo_full_report",
        "prompt_version": DEMO_REPORT_PROMPT_VERSION,
        "account_id": parts.account_id,
        "match_id": parts.match_id,
        "report_metadata": report_metadata,
        "player_match": parts.player_match,
        "match_metadata": match_metadata,
        "target_pawn_entity": target_entity,
        "evidence": evidence,
        "evidence_registry": {
            "count": evidence_ids.len(),
            "raw_count": raw_evidence_count,
            "derived_count": evidence_ids.len() - raw_evidence_count,
            "query_version": DEMO_EVIDENCE_QUERY_VERSION
        },
        "hero_mechanics": parts.hero_mechanics,
        "asset_resolution": parts.asset_resolution,
        "prior_correction_rules": parts.correction_rules,
        "analysis_constraints": {
            "facts": "Only deterministic metadata and cited evidence rows are match facts.",
            "interpretation": "Intent must remain explicitly interpreted, never observed fact.",
            "evaluation": "Evaluate decisions independently of the final match result.",
            "promotion": "Report remains calibration_pending and cannot create active knowledge."
        }
    });
    Ok(BuiltDemoContext {
        context,
        evidence_ids,
    })
}

fn compact_match_metadata(metadata: &Value, account_id: &str) -> Result<Value> {
    let metadata = object(metadata, "match metadata")?;
    let players = metadata
        .get("players")
        .and_then(Value::as_array)
        .ok_or_else(|| LearnError::InvalidInput("Match-Metadaten ohne players.".to_string()))?;
    let target_player = players
        .iter()
        .find(|player| player.get("account_id").map(value_text).as_deref() == Some(account_id))
        .cloned()
        .ok_or_else(|| LearnError::InvalidInput("Zielspieler fehlt in players.".to_string()))?;
    let roster = players
        .iter()
        .map(|player| {
            json!({
                "account_id": player.get("account_id").cloned().unwrap_or(Value::Null),
                "hero_id": player.get("hero_id").cloned().unwrap_or(Value::Null),
                "team": player.get("team").cloned().unwrap_or(Value::Null),
                "player_slot": player.get("player_slot").cloned().unwrap_or(Value::Null),
                "assigned_lane": player.get("assigned_lane").cloned().unwrap_or(Value::Null),
                "kills": player.get("kills").cloned().unwrap_or(Value::Null),
                "deaths": player.get("deaths").cloned().unwrap_or(Value::Null),
                "assists": player.get("assists").cloned().unwrap_or(Value::Null),
                "net_worth": player.get("net_worth").cloned().unwrap_or(Value::Null)
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "match_id": metadata.get("match_id").cloned().unwrap_or(Value::Null),
        "start_time": metadata.get("start_time").cloned().unwrap_or(Value::Null),
        "duration_s": metadata.get("duration_s").cloned().unwrap_or(Value::Null),
        "match_outcome": metadata.get("match_outcome").cloned().unwrap_or(Value::Null),
        "winning_team": metadata.get("winning_team").cloned().unwrap_or(Value::Null),
        "game_mode": metadata.get("game_mode").cloned().unwrap_or(Value::Null),
        "match_mode": metadata.get("match_mode").cloned().unwrap_or(Value::Null),
        "target_player": target_player,
        "roster": roster,
        "objectives": metadata.get("objectives").cloned().unwrap_or(Value::Null)
    }))
}

fn deterministic_report_metadata(
    account_id: &str,
    match_id: &str,
    hero_name: &str,
    match_metadata: &Value,
    patch: Option<&Value>,
) -> Result<Value> {
    let metadata = object(match_metadata, "match metadata")?;
    let metadata_match_id = metadata.get("match_id").map(value_text).unwrap_or_default();
    if metadata_match_id != match_id {
        return invalid(format!(
            "Match-Metadaten gehoeren zu {metadata_match_id} statt {match_id}."
        ));
    }
    let player = metadata
        .get("players")
        .and_then(Value::as_array)
        .and_then(|players| {
            players.iter().find(|player| {
                player.get("account_id").map(value_text).as_deref() == Some(account_id)
            })
        })
        .ok_or_else(|| {
            LearnError::InvalidInput(format!(
                "Zielspieler {account_id} fehlt in Match-Metadaten {match_id}."
            ))
        })?;
    let winning_team = metadata.get("winning_team").map(value_text);
    let player_team = player.get("team").map(value_text);
    let result = match (winning_team.as_deref(), player_team.as_deref()) {
        (Some(winner), Some(team)) if winner == team => Value::String("win".to_string()),
        (Some(_), Some(_)) => Value::String("loss".to_string()),
        _ => Value::Null,
    };
    let lane = player
        .get("assigned_lane")
        .filter(|value| !value.is_null())
        .map(value_text)
        .map(Value::String)
        .unwrap_or(Value::Null);
    let patch = patch.and_then(Value::as_object);
    let patch_metadata = match patch {
        Some(patch) => json!({
            "id": patch.get("patch_external_id").cloned().unwrap_or(Value::Null),
            "title": patch.get("patch_title").cloned().unwrap_or(Value::Null),
            "posted_at": patch.get("posted_at").cloned().unwrap_or(Value::Null),
            "provenance": "derived_from_patch_timeline"
        }),
        None => json!({
            "id": Value::Null,
            "title": Value::Null,
            "posted_at": Value::Null,
            "provenance": "unknown"
        }),
    };
    let mut data_gaps = vec![json!(
        "Rolle ist in den Match-Metadaten nicht explizit enthalten."
    )];
    if result.is_null() {
        data_gaps.push(json!(
            "Ergebnis konnte nicht aus Team und Siegerteam bestimmt werden."
        ));
    }
    if lane.is_null() {
        data_gaps.push(json!("Lane fehlt in den Zielspieler-Metadaten."));
    }
    Ok(json!({
        "match_id": match_id,
        "account_id": account_id,
        "hero": hero_name,
        "patch": patch_metadata,
        "result": result,
        "duration_seconds": metadata.get("duration_s").cloned().unwrap_or(Value::Null),
        "lane": lane,
        "data_gaps": data_gaps
    }))
}

fn build_demo_report_request(
    context: &Value,
    config: &MiniMaxConfig,
) -> Result<ChatCompletionRequest> {
    let report_shape = json!({
        "metadata": context.get("report_metadata").cloned().unwrap_or(Value::Null),
        "phases": {
            "lane": {"reached": false, "decisions": []},
            "transition": {"reached": false, "decisions": []},
            "midgame": {"reached": false, "decisions": []},
            "late_game": {"reached": false, "decisions": []}
        },
        "economy_build": {"decisions": []},
        "combat": {"decisions": []},
        "macro": {"decisions": []},
        "strong_decisions": [],
        "mistakes": [],
        "turning_points": [],
        "hypotheses": [],
        "data_gaps": []
    });
    let decision_shape = json!({
        "tick": 0,
        "time_seconds": 0.0,
        "observed_state": "...",
        "action": "...",
        "effects": "...",
        "interpreted_intent": "...",
        "evaluation": "...",
        "alternative": "...",
        "confidence": 0.0,
        "evidence_ids": ["EXISTIERENDE_EVIDENZ_ID"]
    });
    let hypothesis_shape = json!({
        "claim": "...",
        "confidence": 0.0,
        "evidence_ids": ["EXISTIERENDE_EVIDENZ_ID"],
        "validation_needed": "..."
    });
    let report_shape = serde_json::to_string(&report_shape)?;
    let decision_shape = serde_json::to_string(&decision_shape)?;
    let hypothesis_shape = serde_json::to_string(&hypothesis_shape)?;
    let context = serde_json::to_string(context)?;
    let prompt = format!(
        "Erzeuge den vollstaendigen Mo-&-Krill-Matchreport aus dem folgenden Evidenzkontext.\n\
Antworte ausschliesslich mit einem einzelnen JSON-Objekt, ohne Markdown-Fence und ohne Begleittext.\n\
Nutze exakt dieses Formgeruest und fuelle alle belegten Entscheidungen ein; leere Arrays sind nur erlaubt, wenn keine passende Evidenz vorliegt:\n{report_shape}\n\
Jede Entscheidung hat exakt diese Form; ersetze die Beispielwerte und nutze mindestens eine tatsaechlich vorhandene evidence_id:\n{decision_shape}\n\
Jede Hypothese hat diese Form:\n{hypothesis_shape}\n\
Uebernimm `report_metadata` unveraendert als `metadata`. \
Der Vollreport braucht mindestens zwei Economy-/Build-, drei Combat-, zwei Macro-, eine starke, eine fehlerhafte und eine Wendepunkt-Entscheidung; \
jede erreichte Phase braucht mindestens eine Entscheidung und der gesamte Report mindestens sieben unterschiedliche Kernentscheidungen. Wiederhole dieselbe Entscheidung nie innerhalb eines Abschnitts. \
Wenn dieselbe Entscheidung sowohl zu einer Phase als auch zu einem Fachabschnitt gehoert, verwende exakt denselben Tick und dieselben Evidenz-IDs; das System fuehrt beide Vorkommen unter einer ID zusammen. \
Objective-Ereignisse in `economy_objectives` sind global und beweisen keine Beteiligung des Zielspielers. \
Nutze Item- und Ability-Wirkungen nur, wenn sie in `asset_resolution` oder `hero_mechanics` stehen; nenne sonst die fehlende Mechanik in `data_gaps`. \
Belege Aussagen ueber aggregierten Schaden, Kills oder Tode eines Fight-Fensters mit dessen `fight_window:*`-Evidenz-ID; die darunterliegenden Roh-IDs bleiben zur Nachpruefung erhalten. \
Trenne Beobachtung, Interpretation und Bewertung. Das Matchergebnis ist kein Qualitaetsurteil. \
Unbelegte Aussagen gehoeren ausschliesslich in data_gaps. Hypothesen muessen als solche markiert \
sein. \
Schreibe die Inhalte auf Deutsch, lasse Hero-, Item-, Ability-, Stat- und Map-Namen auf Englisch.\n\n\
Evidenzkontext:\n{context}"
    );
    Ok(ChatCompletionRequest::new(
        vec![
            ChatMessage::system(
                "Du bist ein strenger Deadlock-Demoanalyst. Deine Antwort ist ein einzelnes JSON-Objekt. \
Du erfindest keine Ereignisse, IDs, Mechaniken oder Absichten und belegst jede Entscheidung mit den bereitgestellten Evidenz-IDs.",
            ),
            ChatMessage::user(prompt),
        ],
        config,
    ))
}

fn parse_and_validate_model_report(
    raw: &str,
    deterministic_metadata: &Value,
    evidence_ids: &BTreeSet<String>,
) -> Result<Value> {
    let mut report: Value = serde_json::from_str(raw.trim()).map_err(|error| {
        LearnError::InvalidInput(format!(
            "Fireworks lieferte keinen einzelnen gueltigen JSON-Report: {error}"
        ))
    })?;
    if report.get("metadata") != Some(deterministic_metadata) {
        return invalid("Der Modellreport hat den deterministischen Matchkopf veraendert.");
    }
    validate_demo_report(&report, evidence_ids)?;
    validate_full_demo_report_coverage(&report)?;
    assign_decision_ids(&mut report)?;
    Ok(report)
}

fn assign_decision_ids(report: &mut Value) -> Result<()> {
    let mut next_id = 1_u64;
    let mut canonical = BTreeMap::<String, Value>::new();
    for pointer in [
        "/phases/lane/decisions",
        "/phases/transition/decisions",
        "/phases/midgame/decisions",
        "/phases/late_game/decisions",
        "/economy_build/decisions",
        "/combat/decisions",
        "/macro/decisions",
        "/strong_decisions",
        "/mistakes",
        "/turning_points",
    ] {
        let decisions = report
            .pointer_mut(pointer)
            .and_then(Value::as_array_mut)
            .ok_or_else(|| {
                LearnError::InvalidInput(format!("Reportpfad {pointer} fehlt bei der ID-Vergabe."))
            })?;
        for decision in decisions {
            let fingerprint = decision_fingerprint(decision, pointer)?;
            if let Some(existing) = canonical.get(&fingerprint) {
                *decision = existing.clone();
                continue;
            }
            let decision_object = decision.as_object_mut().ok_or_else(|| {
                LearnError::InvalidInput(format!("Entscheidung an {pointer} ist kein Objekt."))
            })?;
            decision_object.insert("decision_id".to_string(), json!(format!("D{next_id}")));
            next_id += 1;
            canonical.insert(fingerprint, decision.clone());
        }
    }
    Ok(())
}

pub fn validate_demo_report(report: &Value, evidence_ids: &BTreeSet<String>) -> Result<()> {
    let report = object(report, "report")?;
    let actual_sections = report.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected_sections = TOP_LEVEL_SECTIONS.into_iter().collect::<BTreeSet<_>>();
    if actual_sections != expected_sections {
        return invalid("Report hat fehlende oder unbekannte Hauptabschnitte.");
    }

    validate_metadata(field(report, "metadata", "report")?)?;
    let phases = object(field(report, "phases", "report")?, "phases")?;
    let actual_phases = phases.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected_phases = PHASES.into_iter().collect::<BTreeSet<_>>();
    if actual_phases != expected_phases {
        return invalid("phases muss lane, transition, midgame und late_game enthalten.");
    }
    for phase in PHASES {
        let value = field(phases, phase, "phases")?;
        let phase_object = object(value, &format!("phases.{phase}"))?;
        if field(phase_object, "reached", phase)?.as_bool().is_none() {
            return invalid(format!("phases.{phase}.reached muss boolean sein."));
        }
        validate_decisions(
            field(phase_object, "decisions", phase)?,
            evidence_ids,
            &format!("phases.{phase}.decisions"),
        )?;
    }

    for section in ["economy_build", "combat", "macro"] {
        let section_object = object(field(report, section, "report")?, section)?;
        validate_decisions(
            field(section_object, "decisions", section)?,
            evidence_ids,
            &format!("{section}.decisions"),
        )?;
    }
    for section in ["strong_decisions", "mistakes", "turning_points"] {
        validate_decisions(field(report, section, "report")?, evidence_ids, section)?;
    }
    validate_hypotheses(field(report, "hypotheses", "report")?, evidence_ids)?;
    validate_string_array(field(report, "data_gaps", "report")?, "data_gaps")?;
    Ok(())
}

fn validate_full_demo_report_coverage(report: &Value) -> Result<()> {
    let requirements = [
        ("/economy_build/decisions", 2_usize),
        ("/combat/decisions", 3),
        ("/macro/decisions", 2),
        ("/strong_decisions", 1),
        ("/mistakes", 1),
        ("/turning_points", 1),
    ];
    let mut unique_fingerprints = BTreeSet::new();
    for phase in PHASES {
        let reached_pointer = format!("/phases/{phase}/reached");
        let decisions_pointer = format!("/phases/{phase}/decisions");
        let reached = report
            .pointer(&reached_pointer)
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                LearnError::InvalidInput(format!("Reportpfad {reached_pointer} fehlt."))
            })?;
        let decisions = report
            .pointer(&decisions_pointer)
            .and_then(Value::as_array)
            .ok_or_else(|| {
                LearnError::InvalidInput(format!("Reportpfad {decisions_pointer} fehlt."))
            })?;
        if reached && decisions.is_empty() {
            return invalid(format!(
                "Erreichte Phase {phase} braucht mindestens eine Entscheidung."
            ));
        }
        if !reached && !decisions.is_empty() {
            return invalid(format!(
                "Nicht erreichte Phase {phase} darf keine Entscheidungen enthalten."
            ));
        }
        register_decision_fingerprints(decisions, &mut unique_fingerprints, &decisions_pointer)?;
    }
    for (pointer, minimum) in requirements {
        let decisions = report
            .pointer(pointer)
            .and_then(Value::as_array)
            .ok_or_else(|| LearnError::InvalidInput(format!("Reportpfad {pointer} fehlt.")))?;
        if decisions.len() < minimum {
            return invalid(format!(
                "Vollreport braucht an {pointer} mindestens {minimum} Entscheidungen."
            ));
        }
        register_decision_fingerprints(decisions, &mut unique_fingerprints, pointer)?;
    }
    if unique_fingerprints.len() < 7 {
        return invalid(format!(
            "Vollreport braucht mindestens 7 unterschiedliche Entscheidungen, vorhanden: {}.",
            unique_fingerprints.len()
        ));
    }
    Ok(())
}

fn register_decision_fingerprints(
    decisions: &[Value],
    fingerprints: &mut BTreeSet<String>,
    path: &str,
) -> Result<()> {
    let mut local_fingerprints = BTreeSet::new();
    for (index, decision) in decisions.iter().enumerate() {
        let fingerprint = decision_fingerprint(decision, &format!("{path}[{index}]"))?;
        if !local_fingerprints.insert(fingerprint.clone()) {
            return invalid(format!(
                "Vollreport enthaelt im selben Abschnitt dieselbe Entscheidung mehrfach; Duplikat an {path}[{index}]."
            ));
        }
        fingerprints.insert(fingerprint);
    }
    Ok(())
}

fn decision_fingerprint(decision: &Value, path: &str) -> Result<String> {
    let mut normalized = object(decision, path)?.clone();
    normalized.remove("decision_id");
    let mut evidence = array(field(&normalized, "evidence_ids", path)?, path)?
        .iter()
        .map(|value| text(value, &format!("{path}.evidence_ids")).map(str::to_string))
        .collect::<Result<Vec<_>>>()?;
    evidence.sort();
    normalized.insert("evidence_ids".to_string(), json!(evidence));
    Ok(serde_json::to_string(&normalized)?)
}

pub fn validate_demo_report_review(report: &Value, review: &Value) -> Result<Value> {
    object(report, "report")?;
    let mut known_decisions = BTreeMap::<String, Value>::new();
    for pointer in [
        "/phases/lane/decisions",
        "/phases/transition/decisions",
        "/phases/midgame/decisions",
        "/phases/late_game/decisions",
        "/economy_build/decisions",
        "/combat/decisions",
        "/macro/decisions",
        "/strong_decisions",
        "/mistakes",
        "/turning_points",
    ] {
        let decisions = report
            .pointer(pointer)
            .and_then(Value::as_array)
            .ok_or_else(|| LearnError::InvalidInput(format!("Reportpfad {pointer} fehlt.")))?;
        for decision in decisions {
            let decision = object(decision, pointer)?;
            let id = text(field(decision, "decision_id", pointer)?, "decision_id")?;
            if let Some(existing) = known_decisions.get(id) {
                if existing != &Value::Object(decision.clone()) {
                    return invalid(format!(
                        "decision_id {id} hat widerspruechliche Inhalte im Report."
                    ));
                }
            } else {
                known_decisions.insert(id.to_string(), Value::Object(decision.clone()));
            }
        }
    }
    if known_decisions.is_empty() {
        return invalid("Der Report enthaelt keine pruefbaren Entscheidungen.");
    }

    let review_object = object(review, "review")?;
    let reviewed_values = array(
        field(review_object, "reviewed_decision_ids", "review")?,
        "review.reviewed_decision_ids",
    )?;
    if reviewed_values.is_empty() {
        return invalid("reviewed_decision_ids darf nicht leer sein.");
    }
    let mut reviewed_ids = BTreeSet::new();
    for value in reviewed_values {
        let id = text(value, "review.reviewed_decision_ids")?;
        if !known_decisions.contains_key(id) {
            return invalid(format!("Review verweist auf unbekannte decision_id {id}."));
        }
        if !reviewed_ids.insert(id.to_string()) {
            return invalid(format!(
                "decision_id {id} wurde mehrfach als geprueft angegeben."
            ));
        }
    }
    let expected_ids = known_decisions.keys().cloned().collect::<BTreeSet<_>>();
    if reviewed_ids != expected_ids {
        let missing = expected_ids
            .difference(&reviewed_ids)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        return invalid(format!(
            "Review ist unvollstaendig; nicht gepruefte Entscheidungen: {missing}."
        ));
    }

    let corrections = array(
        field(review_object, "corrections", "review")?,
        "review.corrections",
    )?;
    let mut corrected_ids = BTreeSet::new();
    let mut critical_error = false;
    for (index, correction) in corrections.iter().enumerate() {
        let path = format!("review.corrections[{index}]");
        let correction = object(correction, &path)?;
        let decision_id = text(
            field(correction, "decision_id", &path)?,
            &format!("{path}.decision_id"),
        )?;
        if !reviewed_ids.contains(decision_id) {
            return invalid(format!(
                "Korrektur {decision_id} gehoert nicht zu den geprueften Entscheidungen."
            ));
        }
        if !corrected_ids.insert(decision_id.to_string()) {
            return invalid(format!(
                "Doppelte Korrektur fuer decision_id {decision_id}."
            ));
        }
        let label = text(field(correction, "label", &path)?, &format!("{path}.label"))?;
        if !REVIEW_LABELS.contains(&label) {
            return invalid(format!("Unbekanntes Korrekturlabel {label}."));
        }
        text(
            field(correction, "comment", &path)?,
            &format!("{path}.comment"),
        )?;
        let replacement_changes_decision = correction
            .get("replacement")
            .filter(|value| !value.is_null())
            .map(|value| text(value, &format!("{path}.replacement")))
            .transpose()?
            .is_some();
        critical_error |= matches!(
            label,
            "falsch" | "unbelegt" | "wichtige Entscheidung uebersehen"
        ) || (label == "Kontext fehlt" && replacement_changes_decision);
    }

    let mut validated = review_object.clone();
    validated.insert(
        "reviewed_decision_count".to_string(),
        json!(reviewed_ids.len()),
    );
    validated.insert("correction_count".to_string(), json!(corrected_ids.len()));
    validated.insert(
        "correction_rate".to_string(),
        json!(corrected_ids.len() as f64 / reviewed_ids.len() as f64),
    );
    validated.insert("critical_error".to_string(), json!(critical_error));
    Ok(Value::Object(validated))
}

pub async fn save_demo_report_review(pool: &PgPool, note_id: i64, review: &Value) -> Result<Value> {
    if note_id <= 0 {
        return invalid("note_id muss positiv sein.");
    }
    let note = query_one_json(
        pool,
        r#"
        SELECT id, account_id, match_id, prompt_version, context_hash,
               insights::text AS report_json
        FROM brain.player_match_decision_notes
        WHERE id=$1 AND prompt_version LIKE 'mo_full_report_de_v%' AND status='calibration_pending'
        "#,
        &[SqlParam::Int(note_id)],
    )
    .await?
    .ok_or_else(|| {
        LearnError::InvalidInput(format!(
            "Kalibrierungsreport {note_id} wurde nicht gefunden."
        ))
    })?;
    let report_raw = get(&note, "report_json")
        .and_then(Value::as_str)
        .ok_or_else(|| LearnError::InvalidInput("Gespeicherter Report fehlt.".to_string()))?;
    let report: Value = serde_json::from_str(report_raw)?;
    let validated_review = validate_demo_report_review(&report, review)?;
    let report_hash = stable_hash_text(&serde_json::to_string(&report)?);
    let mut payload = object(&validated_review, "validated review")?.clone();
    payload.insert("note_id".to_string(), json!(note_id));
    payload.insert(
        "account_id".to_string(),
        note.get("account_id").cloned().unwrap_or(Value::Null),
    );
    payload.insert(
        "match_id".to_string(),
        note.get("match_id").cloned().unwrap_or(Value::Null),
    );
    payload.insert(
        "report_prompt_version".to_string(),
        note.get("prompt_version").cloned().unwrap_or(Value::Null),
    );
    payload.insert(
        "note_context_hash".to_string(),
        note.get("context_hash").cloned().unwrap_or(Value::Null),
    );
    payload.insert("report_hash".to_string(), json!(report_hash));
    let payload = Value::Object(payload);
    let payload_json = serde_json::to_string(&payload)?;
    let payload_hash = stable_hash_text(&payload_json);
    let external_id = format!("{note_id}:{payload_hash}");
    execute_sql(
        pool,
        r#"
        INSERT INTO brain.entity_snapshots(
          source, entity_type, external_id, canonical_name, payload_hash,
          payload, fetched_at, source_document_id
        )
        VALUES('human_review', 'player_match_report_review', $1, $2, $3, $4::text::jsonb, now(), NULL)
        ON CONFLICT (source, entity_type, external_id, payload_hash) DO NOTHING
        "#,
        &[
            SqlParam::Text(external_id.clone()),
            SqlParam::Text(value_to_string(&payload["account_id"])),
            SqlParam::Text(payload_hash.clone()),
            SqlParam::Text(payload_json),
        ],
    )
    .await?;
    let snapshot = query_one_json(
        pool,
        r#"
        SELECT id AS snapshot_id, extract(epoch from fetched_at)::int8 AS reviewed_at
        FROM brain.entity_snapshots
        WHERE source='human_review' AND entity_type='player_match_report_review'
          AND external_id=$1 AND payload_hash=$2
        ORDER BY id DESC LIMIT 1
        "#,
        &[
            SqlParam::Text(external_id.clone()),
            SqlParam::Text(payload_hash),
        ],
    )
    .await?
    .ok_or_else(|| {
        LearnError::InvalidInput("Review-Snapshot fehlt nach dem Speichern.".to_string())
    })?;
    Ok(json!({
        "snapshot_id": snapshot["snapshot_id"],
        "reviewed_at": snapshot["reviewed_at"],
        "external_id": external_id,
        "payload": payload
    }))
}

pub async fn demo_calibration_status(pool: &PgPool, account_id: &str) -> Result<Value> {
    let account_id = required_numeric_id(account_id, "account_id")?;
    let rows = query_json_rows(
        pool,
        r#"
        WITH latest AS (
          SELECT DISTINCT ON (s.payload->>'note_id')
            s.id, s.payload::text AS payload_json, s.fetched_at
          FROM brain.entity_snapshots s
          JOIN brain.player_match_decision_notes n
            ON n.id::text=s.payload->>'note_id'
           AND n.context_hash=s.payload->>'note_context_hash'
           AND n.status='calibration_pending'
          WHERE s.source='human_review'
            AND s.entity_type='player_match_report_review'
            AND s.payload->>'account_id'=$1
          ORDER BY s.payload->>'note_id', s.fetched_at DESC, s.id DESC
        )
        SELECT id AS snapshot_id, payload_json,
               extract(epoch from fetched_at)::int8 AS reviewed_at
        FROM latest
        ORDER BY fetched_at, id
        "#,
        &[SqlParam::Text(account_id.clone())],
    )
    .await?;
    let mut reviews = Vec::with_capacity(rows.len());
    for row in rows {
        let payload_raw = get(&row, "payload_json")
            .and_then(Value::as_str)
            .ok_or_else(|| LearnError::InvalidInput("Review-Payload fehlt.".to_string()))?;
        let mut payload: Value = serde_json::from_str(payload_raw)?;
        let payload_object = payload.as_object_mut().ok_or_else(|| {
            LearnError::InvalidInput("Review-Payload ist kein Objekt.".to_string())
        })?;
        payload_object.insert(
            "snapshot_id".to_string(),
            row.get("snapshot_id").cloned().unwrap_or(Value::Null),
        );
        payload_object.insert(
            "reviewed_at".to_string(),
            row.get("reviewed_at").cloned().unwrap_or(Value::Null),
        );
        reviews.push(payload);
    }
    calculate_demo_calibration_status(&account_id, &reviews)
}

fn calculate_demo_calibration_status(account_id: &str, reviews: &[Value]) -> Result<Value> {
    let mut seen_note_ids = BTreeSet::new();
    for (index, review) in reviews.iter().enumerate() {
        let review = object(review, &format!("reviews[{index}]"))?;
        let note_id = field(review, "note_id", &format!("reviews[{index}]"))?
            .as_i64()
            .filter(|value| *value > 0)
            .ok_or_else(|| {
                LearnError::InvalidInput(format!("reviews[{index}].note_id ist ungueltig."))
            })?;
        if !seen_note_ids.insert(note_id) {
            return invalid(format!("Report {note_id} hat mehrere aktuelle Reviews."));
        }
        if field(review, "critical_error", &format!("reviews[{index}]"))?
            .as_bool()
            .is_none()
        {
            return invalid(format!(
                "reviews[{index}].critical_error muss boolean sein."
            ));
        }
        let rate = number(
            field(review, "correction_rate", &format!("reviews[{index}]"))?,
            &format!("reviews[{index}].correction_rate"),
        )?;
        if rate > 1.0 {
            return invalid(format!(
                "reviews[{index}].correction_rate muss zwischen 0 und 1 liegen."
            ));
        }
    }
    let qualifying_recent_reports = reviews
        .iter()
        .rev()
        .take(3)
        .take_while(|review| {
            review.get("critical_error").and_then(Value::as_bool) == Some(false)
                && review
                    .get("correction_rate")
                    .and_then(Value::as_f64)
                    .is_some_and(|rate| rate < 0.10)
        })
        .count();
    let passed = reviews.len() >= 5 && qualifying_recent_reports >= 3;
    let recent_reviews = reviews
        .iter()
        .rev()
        .take(3)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    Ok(json!({
        "account_id": account_id,
        "reviewed_report_count": reviews.len(),
        "minimum_reviewed_reports": 5,
        "required_clean_recent_reports": 3,
        "qualifying_recent_reports": qualifying_recent_reports,
        "passed": passed,
        "recent_reviews": recent_reviews
    }))
}

pub fn render_demo_report(report: &Value) -> Result<String> {
    let report = object(report, "report")?;
    let metadata = object(field(report, "metadata", "report")?, "metadata")?;
    let match_id = text(
        field(metadata, "match_id", "metadata")?,
        "metadata.match_id",
    )?;
    let account_id = text(
        field(metadata, "account_id", "metadata")?,
        "metadata.account_id",
    )?;
    let hero = text(field(metadata, "hero", "metadata")?, "metadata.hero")?;
    let mut rendered_decision_ids = BTreeSet::new();
    let mut output = format!(
        "# {hero} Matchreport {match_id}\n\n## Matchkopf\n\n- Spieler: `{account_id}`\n- Match: `{match_id}`\n"
    );
    push_optional_metadata(&mut output, metadata, "result", "Ergebnis");
    push_optional_metadata(
        &mut output,
        metadata,
        "duration_seconds",
        "Dauer (Sekunden)",
    );
    push_optional_metadata(&mut output, metadata, "lane", "Lane");
    if let Some(patch) = metadata.get("patch").and_then(Value::as_object) {
        let patch_id = patch
            .get("id")
            .map(value_text)
            .unwrap_or_else(|| "unknown".to_string());
        let provenance = patch
            .get("provenance")
            .map(value_text)
            .unwrap_or_else(|| "unknown".to_string());
        output.push_str(&format!("- Patch: {patch_id} ({provenance})\n"));
    }
    let metadata_gaps = array(
        field(metadata, "data_gaps", "metadata")?,
        "metadata.data_gaps",
    )?;
    if !metadata_gaps.is_empty() {
        output.push_str("\n### Datenluecken im Matchkopf\n");
        for gap in metadata_gaps {
            output.push_str(&format!("\n- {}\n", value_text(gap)));
        }
    }

    output.push_str("\n## Phasen\n");
    let phases = object(field(report, "phases", "report")?, "phases")?;
    for (key, title) in [
        ("lane", "Lane"),
        ("transition", "Uebergang"),
        ("midgame", "Midgame"),
        ("late_game", "Late Game"),
    ] {
        let phase = object(field(phases, key, "phases")?, key)?;
        output.push_str(&format!("\n### {title}\n"));
        if !phase
            .get("reached")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            output.push_str("\nNicht erreicht.\n");
        }
        render_decisions(
            &mut output,
            field(phase, "decisions", key)?,
            &mut rendered_decision_ids,
        )?;
    }

    for (key, title) in [
        ("economy_build", "Oekonomie und Build"),
        ("combat", "Kaempfe"),
        ("macro", "Makro"),
    ] {
        output.push_str(&format!("\n## {title}\n"));
        let section = object(field(report, key, "report")?, key)?;
        render_decisions(
            &mut output,
            field(section, "decisions", key)?,
            &mut rendered_decision_ids,
        )?;
    }
    for (key, title) in [
        ("strong_decisions", "Starke Entscheidungen"),
        ("mistakes", "Fehler"),
        ("turning_points", "Wendepunkte"),
    ] {
        output.push_str(&format!("\n## {title}\n"));
        render_decisions(
            &mut output,
            field(report, key, "report")?,
            &mut rendered_decision_ids,
        )?;
    }

    output.push_str("\n## Hypothesen\n");
    for hypothesis in array(field(report, "hypotheses", "report")?, "hypotheses")? {
        let hypothesis = object(hypothesis, "hypothesis")?;
        output.push_str(&format!(
            "\n- {} (Sicherheit: {})\n  - Weiter pruefen: {}\n  - Evidenz: {}\n",
            value_text(field(hypothesis, "claim", "hypothesis")?),
            value_text(field(hypothesis, "confidence", "hypothesis")?),
            value_text(field(hypothesis, "validation_needed", "hypothesis")?),
            render_evidence(field(hypothesis, "evidence_ids", "hypothesis")?)?
        ));
    }
    output.push_str("\n## Datenluecken\n");
    for gap in array(field(report, "data_gaps", "report")?, "data_gaps")? {
        output.push_str(&format!("\n- {}\n", value_text(gap)));
    }
    Ok(output)
}

fn validate_metadata(value: &Value) -> Result<()> {
    let metadata = object(value, "metadata")?;
    for key in ["match_id", "account_id", "hero"] {
        text(
            field(metadata, key, "metadata")?,
            &format!("metadata.{key}"),
        )?;
    }
    let patch = object(field(metadata, "patch", "metadata")?, "metadata.patch")?;
    let patch_id = field(patch, "id", "metadata.patch")?;
    if !patch_id.is_null() {
        text(patch_id, "metadata.patch.id")?;
    }
    text(
        field(patch, "provenance", "metadata.patch")?,
        "metadata.patch.provenance",
    )?;
    validate_string_array(
        field(metadata, "data_gaps", "metadata")?,
        "metadata.data_gaps",
    )
}

fn validate_decisions(value: &Value, evidence_ids: &BTreeSet<String>, path: &str) -> Result<()> {
    for (index, decision) in array(value, path)?.iter().enumerate() {
        let path = format!("{path}[{index}]");
        let decision = object(decision, &path)?;
        let actual_fields = decision.keys().map(String::as_str).collect::<BTreeSet<_>>();
        let expected_fields = DECISION_FIELDS.into_iter().collect::<BTreeSet<_>>();
        if actual_fields != expected_fields {
            return invalid(format!("{path} hat fehlende oder unbekannte Felder."));
        }
        if field(decision, "tick", &path)?.as_u64().is_none() {
            return invalid(format!(
                "{path}.tick muss eine nichtnegative Ganzzahl sein."
            ));
        }
        number(
            field(decision, "time_seconds", &path)?,
            &format!("{path}.time_seconds"),
        )?;
        for key in DECISION_TEXT_FIELDS {
            text(field(decision, key, &path)?, &format!("{path}.{key}"))?;
        }
        confidence(field(decision, "confidence", &path)?, &path)?;
        validate_evidence_ids(field(decision, "evidence_ids", &path)?, evidence_ids, &path)?;
    }
    Ok(())
}

fn validate_hypotheses(value: &Value, evidence_ids: &BTreeSet<String>) -> Result<()> {
    for (index, hypothesis) in array(value, "hypotheses")?.iter().enumerate() {
        let path = format!("hypotheses[{index}]");
        let hypothesis = object(hypothesis, &path)?;
        text(field(hypothesis, "claim", &path)?, &format!("{path}.claim"))?;
        text(
            field(hypothesis, "validation_needed", &path)?,
            &format!("{path}.validation_needed"),
        )?;
        confidence(field(hypothesis, "confidence", &path)?, &path)?;
        validate_evidence_ids(
            field(hypothesis, "evidence_ids", &path)?,
            evidence_ids,
            &path,
        )?;
    }
    Ok(())
}

fn validate_evidence_ids(value: &Value, known: &BTreeSet<String>, path: &str) -> Result<()> {
    let ids = array(value, &format!("{path}.evidence_ids"))?;
    if ids.is_empty() {
        return invalid(format!("{path}.evidence_ids darf nicht leer sein."));
    }
    for id in ids {
        let id = text(id, &format!("{path}.evidence_ids"))?;
        if !known.contains(id) {
            return invalid(format!("{path} verweist auf unbekannte Evidenz-ID {id}."));
        }
    }
    Ok(())
}

fn validate_string_array(value: &Value, path: &str) -> Result<()> {
    for item in array(value, path)? {
        text(item, path)?;
    }
    Ok(())
}

fn render_decisions(
    output: &mut String,
    value: &Value,
    rendered_decision_ids: &mut BTreeSet<String>,
) -> Result<()> {
    let decisions = array(value, "decisions")?;
    if decisions.is_empty() {
        output.push_str("\nKeine belegte Entscheidung.\n");
    }
    for decision in decisions {
        let decision = object(decision, "decision")?;
        let decision_id = decision.get("decision_id").and_then(Value::as_str);
        if let Some(decision_id) = decision_id {
            if !rendered_decision_ids.insert(decision_id.to_string()) {
                output.push_str(&format!("\n- Siehe {decision_id}.\n"));
                continue;
            }
        }
        let decision_id = decision_id.map(|id| format!("{id} - ")).unwrap_or_default();
        output.push_str(&format!(
            "\n### {decision_id}{}s / Tick {}\n\n- Beobachtung: {}\n- Aktion: {}\n- Wirkung: {}\n- Interpretation: {}\n- Bewertung: {}\n- Alternative: {}\n- Sicherheit: {}\n- Evidenz: {}\n",
            value_text(field(decision, "time_seconds", "decision")?),
            value_text(field(decision, "tick", "decision")?),
            value_text(field(decision, "observed_state", "decision")?),
            value_text(field(decision, "action", "decision")?),
            value_text(field(decision, "effects", "decision")?),
            value_text(field(decision, "interpreted_intent", "decision")?),
            value_text(field(decision, "evaluation", "decision")?),
            value_text(field(decision, "alternative", "decision")?),
            value_text(field(decision, "confidence", "decision")?),
            render_evidence(field(decision, "evidence_ids", "decision")?)?
        ));
    }
    Ok(())
}

fn render_evidence(value: &Value) -> Result<String> {
    Ok(array(value, "evidence_ids")?
        .iter()
        .map(|value| format!("`{}`", value_text(value)))
        .collect::<Vec<_>>()
        .join(", "))
}

fn push_optional_metadata(
    output: &mut String,
    metadata: &Map<String, Value>,
    key: &str,
    label: &str,
) {
    if let Some(value) = metadata.get(key).filter(|value| !value.is_null()) {
        output.push_str(&format!("- {label}: {}\n", value_text(value)));
    }
}

fn field<'a>(object: &'a Map<String, Value>, key: &str, path: &str) -> Result<&'a Value> {
    object
        .get(key)
        .ok_or_else(|| LearnError::InvalidInput(format!("{path}.{key} fehlt.")))
}

fn object<'a>(value: &'a Value, path: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| LearnError::InvalidInput(format!("{path} muss ein Objekt sein.")))
}

fn array<'a>(value: &'a Value, path: &str) -> Result<&'a Vec<Value>> {
    value
        .as_array()
        .ok_or_else(|| LearnError::InvalidInput(format!("{path} muss ein Array sein.")))
}

fn text<'a>(value: &'a Value, path: &str) -> Result<&'a str> {
    value
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| LearnError::InvalidInput(format!("{path} muss nichtleerer Text sein.")))
}

fn number(value: &Value, path: &str) -> Result<f64> {
    let value = value
        .as_f64()
        .filter(|value| value.is_finite() && *value >= 0.0)
        .ok_or_else(|| {
            LearnError::InvalidInput(format!("{path} muss eine nichtnegative Zahl sein."))
        })?;
    Ok(value)
}

fn confidence(value: &Value, path: &str) -> Result<()> {
    let value = number(value, &format!("{path}.confidence"))?;
    if value > 1.0 {
        return invalid(format!("{path}.confidence muss zwischen 0 und 1 liegen."));
    }
    Ok(())
}

fn value_text(value: &Value) -> String {
    value
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| value.to_string())
}

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(LearnError::InvalidInput(message.into()))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, path::PathBuf};

    use deadlock_brain_core::{config::Settings, minimax::MiniMaxConfig};
    use serde_json::{json, Value};

    use super::{
        build_demo_context_from_parts, build_demo_report_request,
        calculate_demo_calibration_status, collect_demo_asset_ids, compact_asset_for_context,
        compact_demo_evidence, demo_note_context_hash, deterministic_report_metadata,
        parse_and_validate_model_report, render_demo_report, validate_demo_report,
        validate_demo_report_review, validate_full_demo_report_coverage, DemoContextParts,
    };

    const EVIDENCE_ID: &str = "target_combat:92685682:000001";

    fn decision() -> Value {
        json!({
            "tick": 12345,
            "time_seconds": 205.75,
            "observed_state": "Mo steht mit halbem Leben vor einem Teamfight.",
            "action": "Mo startet Burrow.",
            "effects": "Mo erreicht die gegnerische Backline.",
            "interpreted_intent": "Wahrscheinlich soll die gegnerische Carry-Position gebunden werden.",
            "evaluation": "Die Initiierung erzeugt Raum, beginnt aber ohne direkten Follow-up-Kill.",
            "alternative": "Auf den eigenen Cooldown-Vorteil warten und mit dem Team starten.",
            "confidence": 0.8,
            "evidence_ids": [EVIDENCE_ID]
        })
    }

    fn valid_report() -> Value {
        json!({
            "metadata": {
                "match_id": "92685682",
                "account_id": "281768392",
                "hero": "Mo & Krill",
                "patch": {
                    "id": "patch_1",
                    "provenance": "derived_from_patch_timeline"
                },
                "result": "loss",
                "duration_seconds": 2520,
                "lane": "Blue",
                "data_gaps": ["Rolle ist nicht explizit enthalten."]
            },
            "phases": {
                "lane": {"reached": true, "decisions": [decision()]},
                "transition": {"reached": true, "decisions": []},
                "midgame": {"reached": true, "decisions": []},
                "late_game": {"reached": false, "decisions": []}
            },
            "economy_build": {"decisions": []},
            "combat": {"decisions": []},
            "macro": {"decisions": []},
            "strong_decisions": [],
            "mistakes": [],
            "turning_points": [],
            "hypotheses": [{
                "claim": "Der Spieler priorisiert Backline-Druck.",
                "confidence": 0.5,
                "evidence_ids": [EVIDENCE_ID],
                "validation_needed": "In weiteren Matches pruefen."
            }],
            "data_gaps": ["Exakte Teamkommunikation ist nicht beobachtbar."]
        })
    }

    fn comprehensive_report() -> Value {
        let mut report = valid_report();
        report["phases"]["late_game"]["reached"] = json!(true);
        let mut sequence = 1_u64;
        for pointer in [
            "/phases/lane/decisions",
            "/phases/transition/decisions",
            "/phases/midgame/decisions",
            "/phases/late_game/decisions",
            "/economy_build/decisions",
            "/combat/decisions",
            "/macro/decisions",
            "/strong_decisions",
            "/mistakes",
            "/turning_points",
        ] {
            let minimum = match pointer {
                "/economy_build/decisions" | "/macro/decisions" => 2,
                "/combat/decisions" => 3,
                _ => 1,
            };
            let decisions = report
                .pointer_mut(pointer)
                .and_then(Value::as_array_mut)
                .unwrap();
            decisions.clear();
            for _ in 0..minimum {
                let mut value = decision();
                value["tick"] = json!(sequence * 1000);
                value["time_seconds"] = json!(sequence as f64 * 15.625);
                value["action"] = json!(format!("Aktion {sequence}"));
                decisions.push(value);
                sequence += 1;
            }
        }
        report
    }

    fn evidence_ids() -> BTreeSet<String> {
        [EVIDENCE_ID.to_string()].into_iter().collect()
    }

    fn report_with_decision_ids() -> Value {
        parse_and_validate_model_report(
            &serde_json::to_string(&comprehensive_report()).unwrap(),
            &comprehensive_report()["metadata"],
            &evidence_ids(),
        )
        .unwrap()
    }

    fn all_decision_ids(report: &Value) -> Vec<String> {
        let mut ids = BTreeSet::new();
        for pointer in [
            "/phases/lane/decisions",
            "/phases/transition/decisions",
            "/phases/midgame/decisions",
            "/phases/late_game/decisions",
            "/economy_build/decisions",
            "/combat/decisions",
            "/macro/decisions",
            "/strong_decisions",
            "/mistakes",
            "/turning_points",
        ] {
            for decision in report.pointer(pointer).and_then(Value::as_array).unwrap() {
                ids.insert(decision["decision_id"].as_str().unwrap().to_string());
            }
        }
        ids.into_iter().collect()
    }

    fn model_config() -> MiniMaxConfig {
        MiniMaxConfig::from_settings(&Settings {
            project_root: PathBuf::from("/tmp/demo-report-test"),
            data_dir: PathBuf::from("/tmp/demo-report-test/data"),
            raw_dir: PathBuf::from("/tmp/demo-report-test/raw"),
            cache_dir: PathBuf::from("/tmp/demo-report-test/cache"),
            user_agent: "test".to_string(),
            sheet_id: "sheet".to_string(),
            sheet_gid: "0".to_string(),
            wiki_enabled: false,
            wiki_min_delay_seconds: 0.0,
            wiki_cache_ttl_seconds: 0,
            minimax_api_key: None,
            minimax_base_url: "http://127.0.0.1:9".to_string(),
            minimax_model: "test-model".to_string(),
            minimax_timeout_seconds: 1,
            minimax_max_completion_tokens: 512,
            minimax_temperature: 0.2,
            minimax_top_p: 0.9,
            minimax_use_token_plan: false,
        })
    }

    #[test]
    fn validates_a_complete_report_with_known_evidence() {
        validate_demo_report(&valid_report(), &evidence_ids()).unwrap();
    }

    #[test]
    fn rejects_invented_evidence_and_merged_interpretation_evaluation() {
        let mut invented = valid_report();
        invented["phases"]["lane"]["decisions"][0]["evidence_ids"] =
            json!(["target_combat:92685682:999999"]);
        assert!(validate_demo_report(&invented, &evidence_ids()).is_err());

        let mut merged = valid_report();
        merged["phases"]["lane"]["decisions"][0]
            .as_object_mut()
            .unwrap()
            .remove("evaluation");
        assert!(validate_demo_report(&merged, &evidence_ids()).is_err());
    }

    #[test]
    fn renders_the_full_decision_with_evidence() {
        let rendered = render_demo_report(&valid_report()).unwrap();

        assert!(rendered.contains("92685682"));
        assert!(rendered.contains("205.75"));
        assert!(rendered.contains("Die Initiierung erzeugt Raum"));
        assert!(rendered.contains("Auf den eigenen Cooldown-Vorteil warten"));
        assert!(rendered.contains(EVIDENCE_ID));
        assert!(rendered.contains("Rolle ist nicht explizit enthalten"));
    }

    #[test]
    fn compacts_combat_into_900_tick_windows_without_dropping_other_rows() {
        let rows = vec![
            json!({
                "query_name": "player_state",
                "query_version": "mo_full_report_v1",
                "tick": 90,
                "pawn_entity_index": 85,
                "evidence_id": "player_state:92685682:000001"
            }),
            json!({
                "query_name": "target_combat",
                "event_type": "DamageEvent",
                "tick": 100,
                "attacker_entity": 85,
                "victim_entity": 7,
                "damage": 100.0,
                "ability_id": 42,
                "evidence_id": "target_combat:92685682:000001"
            }),
            json!({
                "query_name": "target_combat",
                "event_type": "HeroKilledEvent",
                "tick": 500,
                "attacker_entity": 85,
                "victim_entity": 7,
                "evidence_id": "target_combat:92685682:000002"
            }),
            json!({
                "query_name": "target_combat",
                "event_type": "DamageEvent",
                "tick": 1500,
                "attacker_entity": 7,
                "victim_entity": 85,
                "damage": 40.0,
                "evidence_id": "target_combat:92685682:000003"
            }),
            json!({
                "query_name": "economy_objectives",
                "event_type": "ItemPurchaseNotificationEvent",
                "tick": 200,
                "ability_id": 99,
                "evidence_id": "economy_objectives:92685682:000001"
            }),
        ];

        let compact = compact_demo_evidence(&rows, 85, "92685682").unwrap();

        assert_eq!(compact["player_state"].as_array().unwrap().len(), 1);
        assert_eq!(
            compact["player_state"][0]["evidence_id"],
            "player_state:92685682:000001"
        );
        assert!(compact["player_state"][0].get("query_version").is_none());
        assert_eq!(compact["economy_objectives"].as_array().unwrap().len(), 1);
        assert_eq!(compact["fight_windows"].as_array().unwrap().len(), 2);
        assert_eq!(compact["fight_windows"][0]["start_tick"], 100);
        assert_eq!(compact["fight_windows"][0]["end_tick"], 500);
        assert_eq!(compact["fight_windows"][0]["damage_dealt"], 100.0);
        assert_eq!(compact["fight_windows"][0]["kills"], 1);
        assert_eq!(
            compact["fight_windows"][0]["evidence_ids"],
            json!([
                "target_combat:92685682:000001",
                "target_combat:92685682:000002"
            ])
        );
        assert_eq!(compact["fight_windows"][1]["damage_taken"], 40.0);
        assert_eq!(
            compact["fight_windows"][0]["evidence_id"],
            "fight_window:92685682:000001"
        );
    }

    #[test]
    fn rejects_sparse_or_duplicate_full_reports() {
        assert!(validate_full_demo_report_coverage(&valid_report()).is_err());

        let report = comprehensive_report();
        validate_full_demo_report_coverage(&report).unwrap();

        let mut cross_section_reference = report.clone();
        cross_section_reference["mistakes"][0] = report["strong_decisions"][0].clone();
        validate_full_demo_report_coverage(&cross_section_reference).unwrap();

        let mut duplicate_in_section = report;
        let duplicate = duplicate_in_section["combat"]["decisions"][0].clone();
        duplicate_in_section["combat"]["decisions"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
        assert!(validate_full_demo_report_coverage(&duplicate_in_section).is_err());
    }

    #[test]
    fn rejects_decision_fields_outside_the_report_contract() {
        let mut report = comprehensive_report();
        report["combat"]["decisions"][0]["nonce"] = json!("coverage-bypass");

        assert!(validate_demo_report(&report, &evidence_ids()).is_err());
    }

    #[test]
    fn canonicalizes_cross_section_decisions_and_renders_references() {
        let mut report = comprehensive_report();
        report["economy_build"]["decisions"][0] = report["phases"]["lane"]["decisions"][0].clone();

        let parsed = parse_and_validate_model_report(
            &serde_json::to_string(&report).unwrap(),
            &report["metadata"],
            &evidence_ids(),
        )
        .unwrap();

        assert_eq!(
            parsed["phases"]["lane"]["decisions"][0],
            parsed["economy_build"]["decisions"][0]
        );
        assert_eq!(
            parsed["phases"]["lane"]["decisions"][0]["decision_id"],
            "D1"
        );
        let rendered = render_demo_report(&parsed).unwrap();
        assert_eq!(rendered.matches("### D1 -").count(), 1);
        assert!(rendered.contains("Siehe D1"));
    }

    #[test]
    fn preserves_distinct_assertions_on_the_same_evidence_window() {
        let mut report = comprehensive_report();
        report["economy_build"]["decisions"][0]["tick"] =
            report["phases"]["lane"]["decisions"][0]["tick"].clone();
        report["economy_build"]["decisions"][0]["evidence_ids"] =
            report["phases"]["lane"]["decisions"][0]["evidence_ids"].clone();
        report["economy_build"]["decisions"][0]["evaluation"] =
            json!("Eigenstaendige zweite Bewertung.");

        let parsed = parse_and_validate_model_report(
            &serde_json::to_string(&report).unwrap(),
            &report["metadata"],
            &evidence_ids(),
        )
        .unwrap();

        assert_ne!(
            parsed["phases"]["lane"]["decisions"][0]["decision_id"],
            parsed["economy_build"]["decisions"][0]["decision_id"]
        );
        assert_eq!(
            parsed["economy_build"]["decisions"][0]["evaluation"],
            "Eigenstaendige zweite Bewertung."
        );
    }

    #[test]
    fn final_note_hash_is_bound_to_the_validated_report() {
        let context = json!({"match_id": "92685682", "prompt_version": "v2"});
        let mut changed_report = comprehensive_report();
        changed_report["mistakes"][0]["evaluation"] = json!("Geaenderte Bewertung.");

        let first = demo_note_context_hash(&context, Some(&comprehensive_report())).unwrap();
        let repeated = demo_note_context_hash(&context, Some(&comprehensive_report())).unwrap();
        let changed = demo_note_context_hash(&context, Some(&changed_report)).unwrap();
        let dry_run = demo_note_context_hash(&context, None).unwrap();

        assert_eq!(first, repeated);
        assert_ne!(first, changed);
        assert_ne!(first, dry_run);
    }

    #[test]
    fn classifies_demo_purchase_ability_ids_as_items() {
        let metadata = json!({
            "players": [{"account_id": 281768392, "items": []}]
        });
        let evidence = json!({
            "rows": [{
                "query_name": "economy_objectives",
                "event_type": "ItemPurchaseNotificationEvent",
                "ability_id": 1763073141
            }, {
                "query_name": "target_combat",
                "event_type": "ImportantAbilityUsedEvent",
                "ability_id": 1234567
            }]
        });

        let (ids, item_ids) = collect_demo_asset_ids("281768392", &metadata, &evidence);

        assert!(ids.contains("1763073141"));
        assert!(ids.contains("1234567"));
        assert!(item_ids.contains("1763073141"));
        assert!(!item_ids.contains("1234567"));
    }

    #[test]
    fn builds_a_json_only_evidence_bound_request() {
        let context = json!({
            "report_metadata": valid_report()["metadata"].clone(),
            "evidence": {"fight_windows": [{"evidence_ids": [EVIDENCE_ID]}]}
        });

        let request = build_demo_report_request(&context, &model_config()).unwrap();

        assert_eq!(request.messages.len(), 2);
        assert!(request.messages[0]
            .content
            .contains("einzelnes JSON-Objekt"));
        assert!(request.messages[1].content.contains(EVIDENCE_ID));
        assert!(request.messages[1].content.contains("interpreted_intent"));
        assert!(request.messages[1]
            .content
            .contains(r#""economy_build":{"decisions":[]}"#));
        assert!(request.messages[1]
            .content
            .contains(r#""strong_decisions":[]"#));
    }

    #[test]
    fn rejects_fenced_json_and_changed_deterministic_metadata() {
        let metadata = valid_report()["metadata"].clone();
        let raw = serde_json::to_string(&valid_report()).unwrap();
        assert!(parse_and_validate_model_report(
            &format!("```json\n{raw}\n```"),
            &metadata,
            &evidence_ids()
        )
        .is_err());

        let mut changed = valid_report();
        changed["metadata"]["match_id"] = json!("invented");
        assert!(parse_and_validate_model_report(
            &serde_json::to_string(&changed).unwrap(),
            &metadata,
            &evidence_ids()
        )
        .is_err());
    }

    #[test]
    fn assigns_stable_decision_ids_after_model_validation() {
        let report = report_with_decision_ids();

        assert_eq!(
            report["phases"]["lane"]["decisions"][0]["decision_id"],
            "D1"
        );
        assert!(render_demo_report(&report).unwrap().contains("D1"));
    }

    #[test]
    fn review_accepts_known_decisions_and_calculates_quality_fields() {
        let report = report_with_decision_ids();
        let reviewed_decision_ids = all_decision_ids(&report);
        let review = json!({
            "reviewed_decision_ids": reviewed_decision_ids,
            "corrections": [{
                "decision_id": "D1",
                "label": "falsch",
                "comment": "Die Alternative ignoriert den gegnerischen Cooldown.",
                "replacement": "Rueckzug bis der Cooldown abgelaufen ist."
            }]
        });

        let validated = validate_demo_report_review(&report, &review).unwrap();

        assert_eq!(
            validated["reviewed_decision_count"],
            all_decision_ids(&report).len()
        );
        assert_eq!(validated["correction_count"], 1);
        assert_eq!(
            validated["correction_rate"],
            1.0 / all_decision_ids(&report).len() as f64
        );
        assert_eq!(validated["critical_error"], true);
    }

    #[test]
    fn review_rejects_unknown_or_unreviewed_decisions() {
        let unknown = json!({
            "reviewed_decision_ids": ["D99"],
            "corrections": []
        });
        assert!(validate_demo_report_review(&report_with_decision_ids(), &unknown).is_err());

        let unreviewed = json!({
            "reviewed_decision_ids": ["D1"],
            "corrections": [{
                "decision_id": "D2",
                "label": "unbelegt",
                "comment": "Kein Beleg."
            }]
        });
        assert!(validate_demo_report_review(&report_with_decision_ids(), &unreviewed).is_err());
    }

    #[test]
    fn review_rejects_partial_report_coverage() {
        let partial = json!({
            "reviewed_decision_ids": ["D1"],
            "corrections": []
        });

        assert!(validate_demo_report_review(&report_with_decision_ids(), &partial).is_err());
    }

    #[test]
    fn review_rejects_empty_duplicates_bad_labels_and_empty_comments() {
        let report = report_with_decision_ids();
        let reviewed_decision_ids = all_decision_ids(&report);
        let empty = json!({"reviewed_decision_ids": [], "corrections": []});
        assert!(validate_demo_report_review(&report, &empty).is_err());

        let duplicate = json!({
            "reviewed_decision_ids": reviewed_decision_ids,
            "corrections": [
                {"decision_id": "D1", "label": "unbelegt", "comment": "A"},
                {"decision_id": "D1", "label": "Kontext fehlt", "comment": "B"}
            ]
        });
        assert!(validate_demo_report_review(&report, &duplicate).is_err());

        let bad_label = json!({
            "reviewed_decision_ids": all_decision_ids(&report),
            "corrections": [{"decision_id": "D1", "label": "unklar", "comment": "A"}]
        });
        assert!(validate_demo_report_review(&report, &bad_label).is_err());

        let empty_comment = json!({
            "reviewed_decision_ids": all_decision_ids(&report),
            "corrections": [{"decision_id": "D1", "label": "unbelegt", "comment": "  "}]
        });
        assert!(validate_demo_report_review(&report, &empty_comment).is_err());
    }

    #[test]
    fn review_gate_requires_five_reports_and_three_clean_recent_reports() {
        let clean = |note_id| {
            json!({
                "note_id": note_id,
                "critical_error": false,
                "correction_rate": 0.0
            })
        };
        let four = vec![clean(1), clean(2), clean(3), clean(4)];
        assert_eq!(
            calculate_demo_calibration_status("281768392", &four).unwrap()["passed"],
            false
        );

        let five_with_clean_tail = vec![
            json!({"note_id": 1, "critical_error": true, "correction_rate": 1.0}),
            clean(2),
            clean(3),
            clean(4),
            clean(5),
        ];
        let passed = calculate_demo_calibration_status("281768392", &five_with_clean_tail).unwrap();
        assert_eq!(passed["passed"], true);
        assert_eq!(passed["reviewed_report_count"], 5);
        assert_eq!(passed["qualifying_recent_reports"], 3);

        let mut threshold_is_not_below_ten_percent = five_with_clean_tail;
        threshold_is_not_below_ten_percent[4]["correction_rate"] = json!(0.10);
        assert_eq!(
            calculate_demo_calibration_status("281768392", &threshold_is_not_below_ten_percent)
                .unwrap()["passed"],
            false
        );
    }

    #[test]
    fn builds_the_match_header_only_from_deterministic_metadata() {
        let metadata = json!({
            "match_id": 92685682,
            "duration_s": 2314,
            "winning_team": "Team1",
            "players": [{
                "account_id": 281768392,
                "hero_id": 18,
                "team": "Team1",
                "assigned_lane": 4
            }]
        });

        let header =
            deterministic_report_metadata("281768392", "92685682", "Mo & Krill", &metadata, None)
                .unwrap();

        assert_eq!(header["result"], "win");
        assert_eq!(header["duration_seconds"], 2314);
        assert_eq!(header["lane"], "4");
        assert!(header["patch"]["id"].is_null());
        assert_eq!(header["patch"]["provenance"], "unknown");
        assert!(
            deterministic_report_metadata("999", "92685682", "Mo & Krill", &metadata, None)
                .is_err()
        );
    }

    #[test]
    fn builds_a_compact_context_and_complete_evidence_registry() {
        let match_metadata = json!({
            "match_id": 92685682,
            "duration_s": 1200,
            "winning_team": "Team1",
            "players": [{
                "account_id": 281768392,
                "hero_id": 18,
                "team": "Team1",
                "assigned_lane": 4
            }]
        });
        let demo_evidence = json!({
            "rows": [
                {
                    "query_name": "player_state",
                    "tick": 100,
                    "pawn_entity_index": 85,
                    "evidence_id": "player_state:92685682:000001"
                },
                {
                    "query_name": "target_combat",
                    "event_type": "DamageEvent",
                    "tick": 200,
                    "attacker_entity": 85,
                    "victim_entity": 7,
                    "damage": 10.0,
                    "evidence_id": "target_combat:92685682:000001"
                },
                {
                    "query_name": "economy_objectives",
                    "event_type": "ItemPurchaseNotificationEvent",
                    "tick": 300,
                    "evidence_id": "economy_objectives:92685682:000001"
                }
            ]
        });

        let built = build_demo_context_from_parts(DemoContextParts {
            account_id: "281768392".to_string(),
            match_id: "92685682".to_string(),
            hero_name: "Mo & Krill".to_string(),
            player_match: json!({"match_id": 92685682}),
            match_metadata,
            demo_evidence,
            patch: None,
            hero_mechanics: json!({"name": "Mo & Krill"}),
            asset_resolution: json!({"42": {"name": "Burrow"}}),
            correction_rules: vec![json!({"rule": "Keine Absicht als Fakt."})],
        })
        .unwrap();

        assert_eq!(built.evidence_ids.len(), 4);
        assert_eq!(built.context["evidence_registry"]["raw_count"], 3);
        assert_eq!(built.context["evidence_registry"]["derived_count"], 1);
        assert_eq!(
            built.context["evidence"]["fight_windows"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(built.context["evidence"].get("target_combat").is_none());
        assert_eq!(built.context["report_metadata"]["result"], "win");
        assert_eq!(
            built.context["prior_correction_rules"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn strips_heavy_asset_media_and_limits_item_details() {
        let payload = json!({
            "id": 42,
            "name": "Test Item",
            "type": "item",
            "class_name": "item_test",
            "image": "https://large.example/image.png",
            "description": {"desc": "A".repeat(2000)},
            "properties": {
                "Damage": {"label": "Damage", "value": "10"}
            }
        });

        let item = compact_asset_for_context(&payload, "42", true);
        let ability = compact_asset_for_context(&payload, "42", false);

        assert!(item.get("image").is_none());
        assert!(item["description"].as_str().unwrap().len() <= 603);
        assert!(ability.get("description").is_none());
        assert!(ability.get("properties").is_none());
        assert_eq!(ability["name"], "Test Item");
    }
}
