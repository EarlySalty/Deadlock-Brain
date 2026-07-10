use std::{
    path::Path,
    time::{Duration, Instant},
};

use deadlock_brain_core::{
    http::{HttpClient, HttpGetOptions, RetryPolicy},
    CoreError,
};
use serde_json::{json, Map, Value};

use crate::{
    store::{
        complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore,
    },
    util::{form_urlencode, python_or_string},
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_api";
pub const BASE_URL: &str = "https://api.deadlock-api.com";
pub const DEMO_QUERY_VERSION: &str = "mo_full_report_v1";
const STEAM_ID64_ACCOUNT_BASE: u64 = 76_561_197_960_265_728;
const DEMO_QUERY_COUNT: usize = 3;

#[derive(Debug, Clone)]
pub struct PullMatchMetadataOptions {
    pub match_ids: Vec<String>,
    pub account_ids: Vec<String>,
    pub hero_ids: Vec<String>,
    pub include_player_items: bool,
    pub include_player_info: bool,
    pub include_player_stats: bool,
    pub include_player_death_details: bool,
    pub include_objectives: bool,
    pub cache_ttl_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct PullPlayerMatchHistoryOptions {
    pub account_id: String,
    pub hero_id: Option<u32>,
    pub cache_ttl_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemoJobState {
    Queued,
    Running,
    Done,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoJobStatus {
    pub state: DemoJobState,
    pub result_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoQuery {
    pub name: &'static str,
    pub sql: String,
}

#[derive(Debug, Clone)]
pub struct PullDemoEvidenceOptions {
    pub account_id: String,
    pub match_id: String,
    pub hero_id: u32,
    pub steam_id64: u64,
    pub player_slot: u32,
    pub poll_interval_seconds: u64,
    pub timeout_seconds: u64,
}

impl Default for PullMatchMetadataOptions {
    fn default() -> Self {
        Self {
            match_ids: Vec::new(),
            account_ids: Vec::new(),
            hero_ids: Vec::new(),
            include_player_items: true,
            include_player_info: true,
            include_player_stats: true,
            include_player_death_details: true,
            include_objectives: true,
            cache_ttl_seconds: 21_600,
        }
    }
}

impl Default for PullPlayerMatchHistoryOptions {
    fn default() -> Self {
        Self {
            account_id: String::new(),
            hero_id: Some(18),
            cache_ttl_seconds: 21_600,
        }
    }
}

pub async fn pull_match_metadata(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullMatchMetadataOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("deadlock-api").await?;
    let outcome = pull_match_metadata_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

pub async fn pull_player_match_history(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullPlayerMatchHistoryOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("deadlock-api").await?;
    let outcome = pull_player_match_history_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

pub async fn pull_demo_evidence(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullDemoEvidenceOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("deadlock-api").await?;
    let outcome = pull_demo_evidence_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_match_metadata_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullMatchMetadataOptions,
) -> Result<Value> {
    let safe_match_ids = safe_ids(&options.match_ids);
    if safe_match_ids.is_empty() {
        return Err(SourcesError::invalid_input("match_ids fehlt."));
    }

    let mut params = vec![
        ("match_ids", safe_match_ids.join(",")),
        ("include_info", "true".to_string()),
        (
            "include_player_items",
            options.include_player_items.to_string(),
        ),
        (
            "include_player_info",
            options.include_player_info.to_string(),
        ),
        (
            "include_player_stats",
            options.include_player_stats.to_string(),
        ),
        (
            "include_player_death_details",
            options.include_player_death_details.to_string(),
        ),
        ("include_objectives", options.include_objectives.to_string()),
        ("limit", safe_match_ids.len().to_string()),
    ];
    let safe_account_ids = safe_ids(&options.account_ids);
    if !safe_account_ids.is_empty() {
        params.push(("account_ids", safe_account_ids.join(",")));
    }
    let safe_hero_ids = safe_ids(&options.hero_ids);
    if !safe_hero_ids.is_empty() {
        params.push(("hero_ids", safe_hero_ids.join(",")));
    }

    let url = format!("{BASE_URL}/v1/matches/metadata?{}", form_urlencode(&params));
    let payload = get_deadlock_api_json(http, &url, options.cache_ttl_seconds)?;
    let rows = payload.as_array().cloned().unwrap_or_default();
    let target_player_slot =
        if safe_match_ids.len() == 1 && safe_account_ids.len() == 1 && safe_hero_ids.len() == 1 {
            metadata_player_slot(
                &rows,
                &safe_match_ids[0],
                &safe_account_ids[0],
                &safe_hero_ids[0],
            )?
        } else {
            None
        };
    let external_id = format!("match-metadata:{}", safe_match_ids.join(","));
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Deadlock API match metadata {}", safe_match_ids.join(","));
    let metadata = json!({
        "match_ids": safe_match_ids,
        "account_ids": safe_account_ids,
        "hero_ids": safe_hero_ids,
        "cache_ttl_seconds": options.cache_ttl_seconds,
        "include_player_items": options.include_player_items,
        "include_player_stats": options.include_player_stats,
        "include_player_death_details": options.include_player_death_details,
        "include_objectives": options.include_objectives,
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &external_id,
            title: Some(&title),
            url: Some(&url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::new();
    for row in &rows {
        let Value::Object(object) = row else {
            continue;
        };
        let match_id = python_or_string(object.get("match_id"))
            .or_else(|| python_or_string(object.get("matchId")))
            .unwrap_or_default()
            .trim()
            .to_string();
        if match_id.is_empty() {
            continue;
        }
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "deadlock_api_match_metadata".to_string(),
            external_id: match_id.clone(),
            canonical_name: Some(match_id.clone()),
            payload: with_source_metadata(row, json!({ "source_url": url, "match_id": match_id })),
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "matches": rows.len(),
        "snapshots": count,
        "target_player_slot": target_player_slot,
    }))
}

async fn pull_demo_evidence_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullDemoEvidenceOptions,
) -> Result<Value> {
    let account_id = safe_numeric_id(&options.account_id, "account_id")?;
    validate_demo_identity(&account_id, options.steam_id64)?;
    let match_id = safe_numeric_id(&options.match_id, "match_id")?;
    let match_id_number = parse_u64_id(&match_id, "match_id")?;
    if options.timeout_seconds == 0 {
        return Err(SourcesError::invalid_input(
            "timeout_seconds muss groesser als 0 sein.",
        ));
    }

    let mut all_rows = Vec::new();
    let mut query_summaries = Vec::new();
    let mut source_document_ids = Vec::new();
    for query in demo_query_bundle(options.hero_id, options.steam_id64, options.player_slot) {
        let job_id = submit_demo_query(http, match_id_number, &query.sql)?;
        let status = poll_demo_query(
            http,
            &job_id,
            options.poll_interval_seconds.max(1),
            options.timeout_seconds,
        )
        .await?;
        let result_url = status.result_url.ok_or_else(|| {
            SourcesError::invalid_input(format!(
                "Deadlock API Demo-Job {job_id} ist done ohne result_url."
            ))
        })?;
        let ndjson = download_demo_result(http, &result_url)?;
        let raw_path = store.write_raw(
            SOURCE,
            &format!(
                "demo-evidence:{account_id}:{match_id}:{DEMO_QUERY_VERSION}:{}",
                query.name
            ),
            ndjson.as_bytes(),
            "ndjson",
        )?;
        let metadata = json!({
            "account_id": account_id,
            "match_id": match_id,
            "hero_id": options.hero_id,
            "steam_id64": options.steam_id64,
            "query_name": query.name,
            "query_version": DEMO_QUERY_VERSION,
            "job_id": job_id,
            "result_url": result_url,
        });
        let title = format!("Deadlock API demo evidence {match_id} {}", query.name);
        let document_id = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: &format!(
                    "demo-evidence:{account_id}:{match_id}:{DEMO_QUERY_VERSION}:{}",
                    query.name
                ),
                title: Some(&title),
                url: Some(&result_url),
                content_type: "application/x-ndjson",
                raw_path: &raw_path,
                content: ndjson.as_bytes(),
                metadata: &metadata,
            })
            .await?;
        source_document_ids.push(document_id);

        let rows = normalize_demo_rows(query.name, &ndjson, &match_id)?;
        validate_demo_query_rows(query.name, &rows)?;
        query_summaries.push(json!({
            "name": query.name,
            "job_id": job_id,
            "result_url": result_url,
            "rows": rows.len(),
        }));
        all_rows.extend(rows);
    }

    validate_complete_demo_batch(&query_summaries, &source_document_ids)?;
    let external_id = format!("{account_id}:{match_id}:{DEMO_QUERY_VERSION}");
    let snapshot = EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "deadlock_api_demo_evidence".to_string(),
        external_id: external_id.clone(),
        canonical_name: Some(match_id.clone()),
        payload: json!({
            "account_id": account_id,
            "match_id": match_id,
            "hero_id": options.hero_id,
            "steam_id64": options.steam_id64,
            "query_version": DEMO_QUERY_VERSION,
            "queries": query_summaries,
            "source_document_ids": source_document_ids,
            "rows": all_rows,
        }),
    };
    store
        .upsert_entity_snapshot(&snapshot, source_document_ids.first().copied())
        .await?;

    Ok(json!({
        "account_id": account_id,
        "match_id": match_id,
        "hero_id": options.hero_id,
        "steam_id64": options.steam_id64,
        "query_version": DEMO_QUERY_VERSION,
        "queries": query_summaries,
        "rows": all_rows.len(),
        "snapshot_external_id": external_id,
    }))
}

async fn pull_player_match_history_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullPlayerMatchHistoryOptions,
) -> Result<Value> {
    let account_id = safe_numeric_id(&options.account_id, "account_id")?;
    let url = format!("{BASE_URL}/v1/players/{account_id}/match-history");
    let payload = get_deadlock_api_json(http, &url, options.cache_ttl_seconds)?;
    let rows = match_history_rows(&payload)?;
    let filtered = filter_match_history(rows, options.hero_id)?;
    let external_id = format!("player-match-history:{account_id}");
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Deadlock API player match history {account_id}");
    let metadata = json!({
        "account_id": account_id,
        "hero_id": options.hero_id,
        "cache_ttl_seconds": options.cache_ttl_seconds,
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &external_id,
            title: Some(&title),
            url: Some(&url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::new();
    for (index, row) in filtered.iter().enumerate() {
        let (_, hero_id, match_id) = match_history_fields(row, index)?;
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "deadlock_api_player_match".to_string(),
            external_id: format!("{account_id}:{match_id}"),
            canonical_name: Some(match_id.clone()),
            payload: with_source_metadata(
                row,
                json!({
                    "account_id": account_id,
                    "match_id": match_id,
                    "hero_id": hero_id,
                    "source_url": url,
                }),
            ),
        });
    }
    let stored_rows = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "account_id": account_id,
        "hero_id": options.hero_id,
        "api_rows": rows.len(),
        "stored_rows": stored_rows,
    }))
}

fn submit_demo_query(http: &HttpClient, match_id: u64, sql: &str) -> Result<String> {
    let url = format!("{BASE_URL}/v1/matches/demo/query");
    let body = json!({
        "match_id": match_id,
        "query": sql,
        "format": "ndjson",
    });
    let result = demo_post_json(http, &url, &body)?;
    let payload: Value = serde_json::from_str(&result)?;
    text_field(&payload, &["job_id", "id"])
        .filter(|value| !value.is_empty())
        .ok_or_else(|| SourcesError::invalid_input("Deadlock API Demo-Submit ohne job_id."))
}

async fn poll_demo_query(
    http: &HttpClient,
    job_id: &str,
    poll_interval_seconds: u64,
    timeout_seconds: u64,
) -> Result<DemoJobStatus> {
    let url = format!("{BASE_URL}/v1/matches/demo/query/{job_id}");
    poll_demo_query_url(http, job_id, &url, poll_interval_seconds, timeout_seconds).await
}

async fn poll_demo_query_url(
    http: &HttpClient,
    job_id: &str,
    url: &str,
    poll_interval_seconds: u64,
    timeout_seconds: u64,
) -> Result<DemoJobStatus> {
    let started = Instant::now();
    loop {
        let raw = demo_get_text(http, url, Duration::from_secs(60), "application/json")?;
        let status = parse_demo_job_status(&raw)?;
        match status.state {
            DemoJobState::Done => return Ok(status),
            DemoJobState::Failed => {
                return Err(SourcesError::invalid_input(format!(
                    "Deadlock API Demo-Job {job_id} fehlgeschlagen: {}",
                    status
                        .error
                        .unwrap_or_else(|| "ohne Fehlertext".to_string())
                )));
            }
            DemoJobState::Queued | DemoJobState::Running => {
                if started.elapsed() >= Duration::from_secs(timeout_seconds) {
                    return Err(SourcesError::invalid_input(format!(
                        "Deadlock API Demo-Job {job_id} Timeout nach {timeout_seconds}s."
                    )));
                }
                wait_for_demo_poll(Duration::from_secs(poll_interval_seconds)).await;
            }
        }
    }
}

async fn wait_for_demo_poll(duration: Duration) {
    tokio::time::sleep(duration).await;
}

fn download_demo_result(http: &HttpClient, result_url: &str) -> Result<String> {
    demo_get_text(
        http,
        result_url,
        Duration::from_secs(180),
        "application/x-ndjson",
    )
}

fn parse_demo_job_status(raw: &str) -> Result<DemoJobStatus> {
    let payload: Value = serde_json::from_str(raw)?;
    let status = text_field(&payload, &["status", "state"])
        .ok_or_else(|| SourcesError::invalid_input("Deadlock API Demo-Status ohne status."))?;
    let state = match status.trim().to_ascii_lowercase().as_str() {
        "queued" => DemoJobState::Queued,
        "running" => DemoJobState::Running,
        "done" => DemoJobState::Done,
        "failed" => DemoJobState::Failed,
        other => {
            return Err(SourcesError::invalid_input(format!(
                "Unbekannter Deadlock API Demo-Status: {other}."
            )));
        }
    };
    Ok(DemoJobStatus {
        state,
        result_url: text_field(&payload, &["result_url", "resultUrl", "url"]),
        error: text_field(&payload, &["error", "message"]),
    })
}

fn normalize_demo_rows(query_name: &str, raw: &str, match_id: &str) -> Result<Vec<Value>> {
    let mut rows = Vec::new();
    for (line_index, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut row = serde_json::from_str::<Value>(line).map_err(|error| {
            SourcesError::invalid_input(format!(
                "Deadlock API Demo-NDJSON {query_name} Zeile {} ist ungueltig: {error}",
                line_index + 1
            ))
        })?;
        let object = row.as_object_mut().ok_or_else(|| {
            SourcesError::invalid_input(format!(
                "Deadlock API Demo-NDJSON {query_name} Zeile {} ist kein JSON-Objekt.",
                line_index + 1
            ))
        })?;
        let evidence_index = rows.len() + 1;
        object.insert(
            "evidence_id".to_string(),
            json!(format!("{query_name}:{match_id}:{evidence_index:06}")),
        );
        object.insert("query_name".to_string(), json!(query_name));
        object.insert("query_version".to_string(), json!(DEMO_QUERY_VERSION));
        object.insert("match_id".to_string(), json!(match_id));
        rows.push(row);
    }
    Ok(rows)
}

fn validate_demo_query_rows(query_name: &str, rows: &[Value]) -> Result<()> {
    if query_name == "player_state" && rows.is_empty() {
        return Err(SourcesError::invalid_input(
            "Deadlock API Demo-Query player_state lieferte keine Zielspieler-Daten.",
        ));
    }
    Ok(())
}

fn validate_complete_demo_batch(
    query_summaries: &[Value],
    source_document_ids: &[i64],
) -> Result<()> {
    if query_summaries.len() != DEMO_QUERY_COUNT || source_document_ids.len() != DEMO_QUERY_COUNT {
        return Err(SourcesError::invariant(
            "Unvollstaendige Demo-Evidenz darf nicht als Snapshot gespeichert werden.",
        ));
    }
    Ok(())
}

fn demo_query_bundle(hero_id: u32, steam_id64: u64, player_slot: u32) -> Vec<DemoQuery> {
    vec![
        DemoQuery {
            name: "player_state",
            sql: player_state_query(hero_id, steam_id64),
        },
        DemoQuery {
            name: "target_combat",
            sql: target_combat_query(hero_id, steam_id64),
        },
        DemoQuery {
            name: "economy_objectives",
            sql: economy_objectives_query(hero_id, steam_id64, player_slot),
        },
    ]
}

fn player_state_query(hero_id: u32, steam_id64: u64) -> String {
    format!(
        r#"
WITH sampled AS (
  SELECT
    tick AS sample_tick,
    entity_index,
    "m_steamID" AS steam_id,
    "m_hHeroPawn" AS hero_pawn,
    "m_nAssignedLane" AS assigned_lane,
    "m_PlayerDataGlobal__m_nHeroID" AS hero_id,
    "m_PlayerDataGlobal__m_iHealth" AS health,
    "m_PlayerDataGlobal__m_iHealthMax" AS health_max,
    "m_PlayerDataGlobal__m_iLevel" AS level,
    "m_PlayerDataGlobal__m_iGoldNetWorth" AS net_worth,
    "m_PlayerDataGlobal__m_iPlayerKills" AS kills,
    "m_PlayerDataGlobal__m_iPlayerAssists" AS assists,
    "m_PlayerDataGlobal__m_iDeaths" AS deaths,
    "m_PlayerDataGlobal__m_iLastHits" AS last_hits,
    "m_PlayerDataGlobal__m_iDenies" AS denies,
    "m_PlayerDataGlobal__m_iHeroDamage" AS hero_damage,
    "m_PlayerDataGlobal__m_iObjectiveDamage" AS objective_damage,
    "m_PlayerDataGlobal__m_vecUpgrades" AS upgrades,
    "m_PlayerDataGlobal__m_tHeldItem" AS held_item,
    "m_PlayerDataGlobal__m_vecAbilityUpgradeState__m_ItemID" AS ability_upgrade_item_ids,
    "m_PlayerDataGlobal__m_vecAbilityUpgradeState__m_nUpgradeInfo" AS ability_upgrade_info,
    ROW_NUMBER() OVER (
      PARTITION BY CAST(tick / 1800 AS BIGINT)
      ORDER BY tick DESC
    ) AS sample_rank
  FROM CCitadelPlayerController
  WHERE entity_index IN (
    SELECT entity_index
    FROM CCitadelPlayerController
    WHERE "m_steamID" = {steam_id64}
      AND "m_PlayerDataGlobal__m_nHeroID" = {hero_id}
  )
)
SELECT
  CAST('player_state' AS VARCHAR) AS evidence_kind,
  CAST(sampled.sample_tick AS BIGINT) AS tick,
  CAST(sampled.entity_index AS BIGINT) AS controller_entity_index,
  CAST(sampled.steam_id AS BIGINT) AS steam_id,
  CAST(sampled.hero_pawn AS BIGINT) AS hero_pawn,
  CAST(sampled.hero_pawn AS BIGINT) % 16384 AS pawn_entity_index,
  CAST(sampled.assigned_lane AS BIGINT) AS assigned_lane,
  CAST(sampled.hero_id AS BIGINT) AS hero_id,
  CAST(sampled.health AS BIGINT) AS health,
  CAST(sampled.health_max AS BIGINT) AS health_max,
  CAST(sampled.level AS BIGINT) AS level,
  CAST(sampled.net_worth AS BIGINT) AS net_worth,
  CAST(sampled.kills AS BIGINT) AS kills,
  CAST(sampled.assists AS BIGINT) AS assists,
  CAST(sampled.deaths AS BIGINT) AS deaths,
  CAST(sampled.last_hits AS BIGINT) AS last_hits,
  CAST(sampled.denies AS BIGINT) AS denies,
  CAST(sampled.hero_damage AS BIGINT) AS hero_damage,
  CAST(sampled.objective_damage AS BIGINT) AS objective_damage,
  sampled.upgrades,
  sampled.held_item,
  sampled.ability_upgrade_item_ids,
  sampled.ability_upgrade_info,
  CAST(pawn."m_iHealth" AS BIGINT) AS pawn_health,
  CAST(pawn."m_iMaxHealth" AS BIGINT) AS pawn_health_max,
  CAST(pawn."m_nLevel" AS BIGINT) AS pawn_level,
  CAST(pawn."CBodyComponent__m_cellX" AS BIGINT) AS cell_x,
  CAST(pawn."CBodyComponent__m_cellY" AS BIGINT) AS cell_y,
  CAST(pawn."CBodyComponent__m_cellZ" AS BIGINT) AS cell_z,
  pawn."m_CCitadelAbilityComponent__m_vecAbilities" AS pawn_abilities,
  pawn."m_vecFullSellPriceItems" AS pawn_full_sell_items,
  pawn."m_vecFullSellPriceAbilityUpgrades__m_strAbilityUpgrade" AS pawn_full_sell_ability_upgrades
FROM sampled
LEFT JOIN CCitadelPlayerPawn pawn
  ON pawn.tick = sampled.sample_tick
 AND pawn.entity_index = CAST(sampled.hero_pawn AS BIGINT) % 16384
WHERE sampled.sample_rank = 1
ORDER BY sampled.sample_tick
"#
    )
}

fn target_combat_query(hero_id: u32, steam_id64: u64) -> String {
    format!(
        r#"
WITH target_pawns AS (
  SELECT CAST("m_hHeroPawn" AS BIGINT) % 16384 AS pawn_entity_index
  FROM CCitadelPlayerController
  WHERE "m_steamID" = {steam_id64}
    AND "m_PlayerDataGlobal__m_nHeroID" = {hero_id}
)
SELECT
  CAST('DamageEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(entindex_attacker AS BIGINT) AS attacker_entity,
  CAST(entindex_victim AS BIGINT) AS victim_entity,
  CAST(entindex_inflictor AS BIGINT) AS inflictor_entity,
  CAST(entindex_ability AS BIGINT) AS ability_entity,
  CAST(ability_id AS BIGINT) AS ability_id,
  CAST(NULL AS VARCHAR) AS ability_name,
  CAST(damage AS DOUBLE) AS damage,
  CAST(pre_damage AS DOUBLE) AS pre_damage,
  CAST(health_lost AS DOUBLE) AS health_lost,
  CAST(victim_health_new AS DOUBLE) AS victim_health_new,
  CAST(NULL AS DOUBLE) AS stamina_before,
  CAST(NULL AS DOUBLE) AS stamina_after,
  CAST(NULL AS DOUBLE) AS stamina_drained,
  CAST(NULL AS BIGINT) AS hero_id_interrupter
FROM DamageEvent
WHERE entindex_attacker IN (SELECT pawn_entity_index FROM target_pawns)
   OR entindex_victim IN (SELECT pawn_entity_index FROM target_pawns)
UNION ALL
SELECT
  CAST('HeroKilledEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(entindex_attacker AS BIGINT) AS attacker_entity,
  CAST(entindex_victim AS BIGINT) AS victim_entity,
  CAST(entindex_inflictor AS BIGINT) AS inflictor_entity,
  CAST(NULL AS BIGINT) AS ability_entity,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS VARCHAR) AS ability_name,
  CAST(NULL AS DOUBLE) AS damage,
  CAST(NULL AS DOUBLE) AS pre_damage,
  CAST(NULL AS DOUBLE) AS health_lost,
  CAST(NULL AS DOUBLE) AS victim_health_new,
  CAST(NULL AS DOUBLE) AS stamina_before,
  CAST(NULL AS DOUBLE) AS stamina_after,
  CAST(NULL AS DOUBLE) AS stamina_drained,
  CAST(NULL AS BIGINT) AS hero_id_interrupter
FROM HeroKilledEvent
WHERE entindex_attacker IN (SELECT pawn_entity_index FROM target_pawns)
   OR entindex_victim IN (SELECT pawn_entity_index FROM target_pawns)
UNION ALL
SELECT
  CAST('AbilityInterruptedEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(entindex_interrupter AS BIGINT) AS attacker_entity,
  CAST(entindex_victim AS BIGINT) AS victim_entity,
  CAST(NULL AS BIGINT) AS inflictor_entity,
  CAST(NULL AS BIGINT) AS ability_entity,
  CAST(ability_id_interrupted AS BIGINT) AS ability_id,
  CAST(NULL AS VARCHAR) AS ability_name,
  CAST(NULL AS DOUBLE) AS damage,
  CAST(NULL AS DOUBLE) AS pre_damage,
  CAST(NULL AS DOUBLE) AS health_lost,
  CAST(NULL AS DOUBLE) AS victim_health_new,
  CAST(NULL AS DOUBLE) AS stamina_before,
  CAST(NULL AS DOUBLE) AS stamina_after,
  CAST(NULL AS DOUBLE) AS stamina_drained,
  CAST(hero_id_interrupter AS BIGINT) AS hero_id_interrupter
FROM AbilityInterruptedEvent
WHERE entindex_interrupter IN (SELECT pawn_entity_index FROM target_pawns)
   OR entindex_victim IN (SELECT pawn_entity_index FROM target_pawns)
UNION ALL
SELECT
  CAST('ImportantAbilityUsedEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(caster AS BIGINT) AS attacker_entity,
  CAST(NULL AS BIGINT) AS victim_entity,
  CAST(NULL AS BIGINT) AS inflictor_entity,
  CAST(player AS BIGINT) AS ability_entity,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(ability_name AS VARCHAR) AS ability_name,
  CAST(NULL AS DOUBLE) AS damage,
  CAST(NULL AS DOUBLE) AS pre_damage,
  CAST(NULL AS DOUBLE) AS health_lost,
  CAST(NULL AS DOUBLE) AS victim_health_new,
  CAST(NULL AS DOUBLE) AS stamina_before,
  CAST(NULL AS DOUBLE) AS stamina_after,
  CAST(NULL AS DOUBLE) AS stamina_drained,
  CAST(NULL AS BIGINT) AS hero_id_interrupter
FROM ImportantAbilityUsedEvent
WHERE caster IN (SELECT pawn_entity_index FROM target_pawns)
   OR player IN (SELECT pawn_entity_index FROM target_pawns)
UNION ALL
SELECT
  CAST('StaminaConsumedEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS attacker_entity,
  CAST(entindex_target AS BIGINT) AS victim_entity,
  CAST(NULL AS BIGINT) AS inflictor_entity,
  CAST(NULL AS BIGINT) AS ability_entity,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS VARCHAR) AS ability_name,
  CAST(NULL AS DOUBLE) AS damage,
  CAST(NULL AS DOUBLE) AS pre_damage,
  CAST(NULL AS DOUBLE) AS health_lost,
  CAST(NULL AS DOUBLE) AS victim_health_new,
  CAST(stamina_before AS DOUBLE) AS stamina_before,
  CAST(stamina_after AS DOUBLE) AS stamina_after,
  CAST(drained AS DOUBLE) AS stamina_drained,
  CAST(NULL AS BIGINT) AS hero_id_interrupter
FROM StaminaConsumedEvent
WHERE entindex_target IN (SELECT pawn_entity_index FROM target_pawns)
ORDER BY tick
"#
    )
}

fn economy_objectives_query(hero_id: u32, steam_id64: u64, player_slot: u32) -> String {
    format!(
        r#"
WITH target AS (
  SELECT CAST("m_hHeroPawn" AS BIGINT) % 16384 AS pawn_entity_index
  FROM CCitadelPlayerController
  WHERE "m_steamID" = {steam_id64}
    AND "m_PlayerDataGlobal__m_nHeroID" = {hero_id}
)
SELECT
  CAST('ItemPurchaseNotificationEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(userid AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(ability_id AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(sell AS BOOLEAN) AS sell,
  CAST(quickbuy AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM ItemPurchaseNotificationEvent
WHERE userid = {player_slot}
UNION ALL
SELECT
  CAST('AbilitiesChangedEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS user_id,
  CAST(purchaser_player_slot AS BIGINT) AS player_slot,
  CAST(ability_id AS BIGINT) AS ability_id,
  CAST(change AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM AbilitiesChangedEvent
WHERE purchaser_player_slot = {player_slot}
UNION ALL
SELECT
  CAST('CurrencyChangedEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(userid AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(ability_id AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(currency_type AS BIGINT) AS currency_type,
  CAST(currency_source AS BIGINT) AS currency_source,
  CAST(delta AS BIGINT) AS currency_delta,
  CAST(new_value AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM CurrencyChangedEvent
WHERE userid = {player_slot}
UNION ALL
SELECT
  CAST('ObjectiveMaskEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(objective_mask_team0 AS BIGINT) AS objective_mask_team0,
  CAST(objective_mask_team1 AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM ObjectiveMaskEvent
UNION ALL
SELECT
  CAST('BossKilledEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(objective_team AS BIGINT) AS objective_team,
  CAST(objective_mask_change AS BIGINT) AS objective_mask_change,
  CAST(entity_killed AS BIGINT) AS entity_killed,
  CAST(entity_killer AS BIGINT) AS entity_killer,
  CAST(gametime AS DOUBLE) AS gametime
FROM BossKilledEvent
UNION ALL
SELECT
  CAST('GameOverEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(NULL AS BIGINT) AS xp,
  CAST(NULL AS BIGINT) AS gold,
  CAST(NULL AS BIGINT) AS winner,
  CAST(winning_team AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM GameOverEvent
UNION ALL
SELECT
  CAST('TeamRewardsEvent' AS VARCHAR) AS event_type,
  CAST(tick AS BIGINT) AS tick,
  CAST(NULL AS BIGINT) AS user_id,
  CAST(NULL AS BIGINT) AS player_slot,
  CAST(NULL AS BIGINT) AS ability_id,
  CAST(NULL AS BIGINT) AS ability_change,
  CAST(NULL AS BOOLEAN) AS sell,
  CAST(NULL AS BOOLEAN) AS quickbuy,
  CAST(NULL AS BIGINT) AS currency_type,
  CAST(NULL AS BIGINT) AS currency_source,
  CAST(NULL AS BIGINT) AS currency_delta,
  CAST(NULL AS BIGINT) AS currency_new_value,
  CAST(NULL AS BIGINT) AS objective_mask_team0,
  CAST(NULL AS BIGINT) AS objective_mask_team1,
  CAST(xp AS BIGINT) AS xp,
  CAST(gold AS BIGINT) AS gold,
  CAST(winner AS BIGINT) AS winner,
  CAST(NULL AS BIGINT) AS winning_team,
  CAST(NULL AS BIGINT) AS objective_team,
  CAST(NULL AS BIGINT) AS objective_mask_change,
  CAST(NULL AS BIGINT) AS entity_killed,
  CAST(NULL AS BIGINT) AS entity_killer,
  CAST(NULL AS DOUBLE) AS gametime
FROM TeamRewardsEvent
ORDER BY tick
"#
    )
}

fn demo_post_json(http: &HttpClient, url: &str, body: &Value) -> Result<String> {
    match http.post_json(
        url,
        body,
        HttpGetOptions {
            timeout: Duration::from_secs(60),
            headers: demo_headers("application/json"),
            retry: RetryPolicy {
                attempts: 1,
                backoff: Duration::from_millis(0),
            },
            ..HttpGetOptions::default()
        },
    ) {
        Ok(result) => Ok(result.text()),
        Err(CoreError::HttpStatus { status, .. }) if status.as_u16() == 404 => Err(
            SourcesError::invalid_input("Deadlock API Demo ist nicht verfuegbar."),
        ),
        Err(error) => Err(error.into()),
    }
}

fn demo_get_text(
    http: &HttpClient,
    url: &str,
    timeout: Duration,
    accept: &'static str,
) -> Result<String> {
    match http.get(
        url,
        HttpGetOptions {
            timeout,
            headers: demo_headers(accept),
            ..HttpGetOptions::default()
        },
    ) {
        Ok(result) => Ok(result.text()),
        Err(CoreError::HttpStatus { status, .. }) if status.as_u16() == 404 => Err(
            SourcesError::invalid_input("Deadlock API Demo ist nicht verfuegbar."),
        ),
        Err(error) => Err(error.into()),
    }
}

fn demo_headers(accept: &'static str) -> Vec<(String, String)> {
    vec![
        ("Accept".to_string(), accept.to_string()),
        (
            "Referer".to_string(),
            "https://deadlock-api.com/".to_string(),
        ),
    ]
}

fn get_deadlock_api_json(http: &HttpClient, url: &str, cache_ttl_seconds: u64) -> Result<Value> {
    let result = http.get(
        url,
        HttpGetOptions {
            cache_ttl_seconds: Some(cache_ttl_seconds),
            timeout: Duration::from_secs(60),
            headers: vec![
                ("Accept".to_string(), "application/json".to_string()),
                (
                    "Referer".to_string(),
                    "https://deadlock-api.com/".to_string(),
                ),
            ],
            ..HttpGetOptions::default()
        },
    )?;
    Ok(serde_json::from_str(&result.text())?)
}

fn safe_ids(values: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for value in values {
        let raw = value.trim();
        if !raw.is_empty() && !result.iter().any(|existing| existing == raw) {
            result.push(raw.to_string());
        }
    }
    result
}

fn safe_numeric_id(value: &str, name: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(SourcesError::invalid_input(format!(
            "{name} muss eine numerische ID sein."
        )));
    }
    Ok(trimmed.to_string())
}

fn parse_u64_id(value: &str, name: &str) -> Result<u64> {
    value.parse::<u64>().map_err(|error| {
        SourcesError::invalid_input(format!("{name} ist keine gueltige u64-ID: {error}"))
    })
}

fn validate_demo_identity(account_id: &str, steam_id64: u64) -> Result<()> {
    let account_id = parse_u64_id(account_id, "account_id")?;
    let expected = STEAM_ID64_ACCOUNT_BASE
        .checked_add(account_id)
        .ok_or_else(|| SourcesError::invalid_input("account_id ist zu gross fuer steam_id64."))?;
    if steam_id64 != expected {
        return Err(SourcesError::invalid_input(
            "account_id und steam_id64 gehoeren nicht zum selben Spieler.",
        ));
    }
    Ok(())
}

fn text_field(value: &Value, keys: &[&str]) -> Option<String> {
    let object = value.as_object()?;
    for key in keys {
        let Some(raw) = object.get(*key) else {
            continue;
        };
        let text = match raw {
            Value::String(text) => text.trim().to_string(),
            Value::Null => continue,
            other => other.to_string(),
        };
        if !text.is_empty() {
            return Some(text);
        }
    }
    None
}

fn metadata_player_slot(
    rows: &[Value],
    match_id: &str,
    account_id: &str,
    hero_id: &str,
) -> Result<Option<u32>> {
    for row in rows {
        let Some(object) = row.as_object() else {
            continue;
        };
        if python_or_string(object.get("match_id")).as_deref() != Some(match_id) {
            continue;
        }
        let Some(players) = object.get("players").and_then(Value::as_array) else {
            continue;
        };
        for player in players {
            let Some(player) = player.as_object() else {
                continue;
            };
            if python_or_string(player.get("account_id")).as_deref() != Some(account_id)
                || python_or_string(player.get("hero_id")).as_deref() != Some(hero_id)
            {
                continue;
            }
            let slot = python_or_string(player.get("player_slot"))
                .ok_or_else(|| {
                    SourcesError::invalid_input(
                        "Deadlock API Zielspieler-Metadaten ohne player_slot.",
                    )
                })?
                .parse::<u32>()
                .map_err(|error| {
                    SourcesError::invalid_input(format!(
                        "Deadlock API player_slot ist ungueltig: {error}"
                    ))
                })?;
            return Ok(Some(slot));
        }
    }
    Ok(None)
}

fn match_history_rows(payload: &Value) -> Result<&[Value]> {
    payload.as_array().map(Vec::as_slice).ok_or_else(|| {
        SourcesError::invalid_input("Deadlock API match-history ist kein JSON-Array.")
    })
}

fn filter_match_history(rows: &[Value], hero_id: Option<u32>) -> Result<Vec<Value>> {
    let mut filtered = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let (_, row_hero_id, _) = match_history_fields(row, index)?;
        if hero_id.is_none_or(|expected| row_hero_id == expected) {
            filtered.push(row.clone());
        }
    }
    Ok(filtered)
}

fn match_history_fields(row: &Value, index: usize) -> Result<(&Map<String, Value>, u32, String)> {
    let row_number = index + 1;
    let object = row.as_object().ok_or_else(|| {
        SourcesError::invalid_input(format!(
            "Deadlock API match-history Zeile {row_number} ist kein JSON-Objekt."
        ))
    })?;
    let hero_id = history_hero_id(object).ok_or_else(|| {
        SourcesError::invalid_input(format!(
            "Deadlock API match-history Zeile {row_number} hat keine gueltige hero_id."
        ))
    })?;
    let match_id = history_match_id(object).ok_or_else(|| {
        SourcesError::invalid_input(format!(
            "Deadlock API match-history Zeile {row_number} hat keine nicht-leere match_id."
        ))
    })?;
    Ok((object, hero_id, match_id))
}

fn history_hero_id(object: &Map<String, Value>) -> Option<u32> {
    for key in ["hero_id", "heroId", "player_hero_id", "playerHeroId"] {
        let Some(value) = object.get(key) else {
            continue;
        };
        if let Some(id) = value.as_u64().and_then(|id| u32::try_from(id).ok()) {
            return Some(id);
        }
        if let Some(id) = value
            .as_str()
            .and_then(|text| text.trim().parse::<u32>().ok())
        {
            return Some(id);
        }
    }
    None
}

fn history_match_id(object: &Map<String, Value>) -> Option<String> {
    python_or_string(object.get("match_id"))
        .or_else(|| python_or_string(object.get("matchId")))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn with_source_metadata(payload: &Value, metadata: Value) -> Value {
    let mut copied = payload.as_object().cloned().unwrap_or_else(|| {
        let mut object = serde_json::Map::new();
        object.insert("value".to_string(), payload.clone());
        object
    });
    copied.insert("_deadlock_brain".to_string(), metadata);
    Value::Object(copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_every_requested_hero_match() {
        let payload = json!([
            {
                "match_id": 111,
                "hero_id": 18,
                "match_result": "won",
                "start_time": 1700000000,
                "team_abandoned": false
            },
            {
                "match_id": 222,
                "hero_id": 7,
                "match_result": "lost",
                "start_time": 1700000300,
                "team_abandoned": false
            },
            {
                "match_id": 333,
                "hero_id": 18,
                "match_result": "lost",
                "start_time": 1700000600,
                "team_abandoned": true
            }
        ]);

        let filtered =
            filter_match_history(match_history_rows(&payload).unwrap(), Some(18)).unwrap();

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0]["match_id"], json!(111));
        assert_eq!(filtered[0]["match_result"], json!("won"));
        assert_eq!(filtered[0]["start_time"], json!(1700000000));
        assert_eq!(filtered[0]["team_abandoned"], json!(false));
        assert_eq!(filtered[1]["match_id"], json!(333));
        assert_eq!(filtered[1]["match_result"], json!("lost"));
        assert_eq!(filtered[1]["start_time"], json!(1700000600));
        assert_eq!(filtered[1]["team_abandoned"], json!(true));
    }

    #[test]
    fn rejects_non_object_match_history_row() {
        let payload = json!([
            {
                "match_id": 111,
                "hero_id": 18
            },
            "not an object"
        ]);

        let error =
            filter_match_history(match_history_rows(&payload).unwrap(), Some(18)).unwrap_err();

        assert!(error.to_string().contains("Zeile 2"));
        assert!(error.to_string().contains("JSON-Objekt"));
    }

    #[test]
    fn rejects_match_history_row_without_valid_hero_id() {
        for payload in [
            json!([{ "match_id": 111 }]),
            json!([{ "match_id": 111, "hero_id": "   " }]),
            json!([{ "match_id": 111, "hero_id": -1 }]),
        ] {
            let error =
                filter_match_history(match_history_rows(&payload).unwrap(), Some(18)).unwrap_err();

            assert!(error.to_string().contains("hero_id"));
        }
    }

    #[test]
    fn rejects_match_history_row_without_non_empty_match_id() {
        for payload in [
            json!([{ "hero_id": 18 }]),
            json!([{ "match_id": "", "hero_id": 18 }]),
            json!([{ "match_id": "   ", "hero_id": 18 }]),
        ] {
            let error =
                filter_match_history(match_history_rows(&payload).unwrap(), Some(18)).unwrap_err();

            assert!(error.to_string().contains("match_id"));
        }
    }

    #[test]
    fn rejects_non_array_match_history() {
        let error = match_history_rows(&json!({"error": "schema changed"})).unwrap_err();

        assert!(error
            .to_string()
            .contains("match-history ist kein JSON-Array"));
    }

    #[test]
    fn demo_parse_job_status_accepts_done_with_result_url() {
        let status =
            parse_demo_job_status(r#"{"status":"done","result_url":"https://x/result.ndjson"}"#)
                .unwrap();

        assert_eq!(status.state, DemoJobState::Done);
        assert_eq!(
            status.result_url.as_deref(),
            Some("https://x/result.ndjson")
        );
        assert_eq!(status.error, None);
    }

    #[test]
    fn demo_parse_job_status_preserves_failed_error() {
        let status =
            parse_demo_job_status(r#"{"status":"failed","error":"demo unavailable"}"#).unwrap();

        assert_eq!(status.state, DemoJobState::Failed);
        assert_eq!(status.error.as_deref(), Some("demo unavailable"));
    }

    #[test]
    fn demo_normalize_rows_adds_versioned_evidence_id() {
        let rows = normalize_demo_rows("combat", "{\"tick\":10}\n", "92242282").unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["evidence_id"], "combat:92242282:000001");
        assert_eq!(rows[0]["query_name"], "combat");
        assert_eq!(rows[0]["query_version"], DEMO_QUERY_VERSION);
        assert_eq!(rows[0]["match_id"], "92242282");
        assert_eq!(rows[0]["tick"], 10);
    }

    #[test]
    fn demo_normalize_rows_rejects_malformed_ndjson_line() {
        let error = normalize_demo_rows("combat", "{\"tick\":10}\nnot-json\n", "92242282")
            .unwrap_err()
            .to_string();

        assert!(error.contains("NDJSON"));
        assert!(error.contains("Zeile 2"));
    }

    #[test]
    fn demo_query_bundle_has_three_quoted_queries() {
        let queries = demo_query_bundle(18, 76561198242034120, 8);

        assert_eq!(queries.len(), 3);
        assert_eq!(queries[0].name, "player_state");
        assert_eq!(queries[1].name, "target_combat");
        assert_eq!(queries[2].name, "economy_objectives");
        assert!(queries.iter().all(|query| query.sql.contains('"')));
        assert!(queries
            .iter()
            .all(|query| query.sql.contains("76561198242034120")));
        assert!(queries.iter().all(|query| query.sql.contains("18")));
        assert!(queries[0].sql.contains("ROW_NUMBER() OVER"));
        assert!(queries[0]
            .sql
            .contains("PARTITION BY CAST(tick / 1800 AS BIGINT)"));
        assert!(!queries[1].sql.contains("ROW_NUMBER() OVER"));
        assert!(!queries[2].sql.contains("ROW_NUMBER() OVER"));
    }

    #[test]
    fn demo_player_state_uses_an_unambiguous_join_tick() {
        let state = demo_query_bundle(18, 76561198242034120, 8)
            .into_iter()
            .find(|query| query.name == "player_state")
            .expect("player state query");

        assert!(state.sql.contains("tick AS sample_tick"));
        assert!(state.sql.contains("pawn.tick = sampled.sample_tick"));
        assert!(!state.sql.contains("sampled.tick"));
    }

    #[test]
    fn demo_target_queries_use_source2_14_bit_pawn_entity_index() {
        let queries = demo_query_bundle(18, 76561198242034120, 8);

        let combat = queries
            .iter()
            .find(|query| query.name == "target_combat")
            .expect("target combat query");
        assert!(combat
            .sql
            .contains("CAST(\"m_hHeroPawn\" AS BIGINT) % 16384"));
        assert!(!combat.sql.contains("% 32768"));

        let state = queries
            .iter()
            .find(|query| query.name == "player_state")
            .expect("player state query");
        assert!(state
            .sql
            .contains("CAST(sampled.hero_pawn AS BIGINT) % 16384"));
    }

    #[test]
    fn demo_economy_query_filters_user_events_by_lobby_player_slot() {
        let queries = demo_query_bundle(18, 76561198242034120, 8);
        let economy = queries
            .iter()
            .find(|query| query.name == "economy_objectives")
            .expect("economy objectives query");

        assert_eq!(economy.sql.matches("userid = 8").count(), 2);
        assert!(!economy.sql.contains("userid = 281768392"));
    }

    #[test]
    fn demo_economy_query_uses_the_metadata_player_slot_literal() {
        let economy = economy_objectives_query(18, 76561198242034120, 8);

        assert!(economy.contains("WHERE userid = 8"));
        assert!(economy.contains("WHERE purchaser_player_slot = 8"));
        assert!(!economy.contains("m_unLobbyPlayerSlot"));
        assert!(!economy.contains("OR entindex_victim"));
    }

    #[test]
    fn demo_metadata_resolves_the_target_player_slot() {
        let rows = vec![json!({
            "match_id": 92685682,
            "players": [{
                "account_id": 281768392,
                "hero_id": 18,
                "player_slot": 8
            }]
        })];

        assert_eq!(
            metadata_player_slot(&rows, "92685682", "281768392", "18").unwrap(),
            Some(8)
        );
    }

    #[test]
    fn demo_identity_rejects_mismatched_account_and_steam_ids() {
        let error = validate_demo_identity("123", 76561198242034120)
            .unwrap_err()
            .to_string();

        assert!(error.contains("account_id und steam_id64"));
    }

    #[test]
    fn demo_player_state_must_not_be_empty() {
        let error = validate_demo_query_rows("player_state", &[])
            .unwrap_err()
            .to_string();

        assert!(error.contains("player_state"));
        assert!(validate_demo_query_rows("target_combat", &[]).is_ok());
    }

    #[tokio::test]
    async fn demo_poll_wait_yields_to_the_async_runtime() {
        use std::{cell::Cell, rc::Rc};

        let other_task_ran = Rc::new(Cell::new(false));
        let marker = Rc::clone(&other_task_ran);
        tokio::join!(
            async {
                wait_for_demo_poll(Duration::from_millis(10)).await;
                assert!(other_task_ran.get());
            },
            async {
                tokio::task::yield_now().await;
                marker.set(true);
            }
        );
    }

    #[test]
    fn demo_404_is_visible_and_an_incomplete_batch_cannot_be_snapshotted() {
        use std::{io::Write, net::TcpListener, thread};

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/result", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let bytes_read = std::io::Read::read(&mut stream, &mut request).unwrap();
            assert!(bytes_read > 0);
            stream
                .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
                .unwrap();
        });
        let cache = tempfile::tempdir().unwrap();
        let http = HttpClient::new("demo-test", cache.path()).unwrap();

        let error = demo_get_text(&http, &url, Duration::from_secs(1), "application/json")
            .unwrap_err()
            .to_string();
        server.join().unwrap();

        assert!(error.contains("nicht verfuegbar"));
        assert!(validate_complete_demo_batch(&[], &[]).is_err());
    }

    #[test]
    fn demo_failed_and_timeout_jobs_are_visible() {
        use std::{io::Write, net::TcpListener, thread};

        fn status_server(body: &'static str) -> (String, thread::JoinHandle<()>) {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!("http://{}/status", listener.local_addr().unwrap());
            let server = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0_u8; 1024];
                let bytes_read = std::io::Read::read(&mut stream, &mut request).unwrap();
                assert!(bytes_read > 0);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            });
            (url, server)
        }

        let cache = tempfile::tempdir().unwrap();
        let http = HttpClient::new("demo-test", cache.path()).unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (failed_url, failed_server) =
            status_server(r#"{"status":"failed","error":"demo gone"}"#);
        let failed = runtime
            .block_on(poll_demo_query_url(&http, "failed-job", &failed_url, 1, 10))
            .unwrap_err()
            .to_string();
        failed_server.join().unwrap();
        assert!(failed.contains("demo gone"));

        let (running_url, running_server) = status_server(r#"{"status":"running"}"#);
        let timeout = runtime
            .block_on(poll_demo_query_url(&http, "slow-job", &running_url, 1, 0))
            .unwrap_err()
            .to_string();
        running_server.join().unwrap();
        assert!(timeout.contains("Timeout"));
    }
}
