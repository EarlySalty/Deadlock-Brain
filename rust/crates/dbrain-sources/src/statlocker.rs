use std::{collections::BTreeSet, path::Path, thread, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Map, Value};
use sqlx::PgPool;

use crate::{
    store::{complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    util::{form_urlencode, python_or_string, quote_path, value_to_python_string},
    Result, SourcesError,
};

pub const SOURCE: &str = "statlocker";
pub const BASE_URL: &str = "https://statlocker.gg";
pub const DEFAULT_RANKS: &str = "rank_10,rank_11";

#[derive(Debug, Clone)]
pub struct PullStatlockerOptions {
    pub kinds: Vec<String>,
    pub patch: Option<String>,
    pub hero: String,
    pub min_sample_size: u64,
    pub rank: String,
    pub leaderboard_page: u64,
    pub leaderboard_page_size: u64,
    pub account_id: Option<String>,
    pub match_id: Option<String>,
    pub hero_id: Option<String>,
    pub players_from_leaderboard: u64,
    pub matches_per_player: u64,
    pub include_match_details: bool,
    pub include_build_analysis: bool,
    pub game_mode: Option<String>,
    pub delay_seconds: f64,
    pub cache_ttl_seconds: u64,
}

impl Default for PullStatlockerOptions {
    fn default() -> Self {
        Self {
            kinds: Vec::new(),
            patch: None,
            hero: "all".to_string(),
            min_sample_size: 500,
            rank: DEFAULT_RANKS.to_string(),
            leaderboard_page: 1,
            leaderboard_page_size: 100,
            account_id: None,
            match_id: None,
            hero_id: None,
            players_from_leaderboard: 5,
            matches_per_player: 6,
            include_match_details: false,
            include_build_analysis: false,
            game_mode: None,
            delay_seconds: 1.0,
            cache_ttl_seconds: 21_600,
        }
    }
}

pub async fn pull_statlocker(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullStatlockerOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("statlocker").await?;
    let outcome = pull_statlocker_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_statlocker_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullStatlockerOptions,
) -> Result<Value> {
    let selected = if options.kinds.is_empty() {
        vec![
            "wpa-patches".to_string(),
            "wpa-items".to_string(),
            "leaderboard".to_string(),
        ]
    } else {
        options.kinds.clone()
    };

    let mut endpoints = Map::new();
    let mut total_snapshots = 0usize;
    let mut latest_patch = options.patch.clone();

    if contains_kind(&selected, "wpa-patches") || contains_kind(&selected, "wpa-items") {
        let (patches_summary, latest) =
            pull_wpa_patches(store, http, options.cache_ttl_seconds).await?;
        total_snapshots += snapshots_from_summary(&patches_summary);
        endpoints.insert("wpa-patches".to_string(), patches_summary);
        if latest_patch.as_ref().map(|value| value.is_empty()).unwrap_or(true) {
            latest_patch = latest;
        }
    }

    if contains_kind(&selected, "wpa-items") {
        let patch = latest_patch.as_deref().unwrap_or_default();
        if patch.is_empty() {
            return Err(SourcesError::invalid_input("Kein Statlocker WPA-Patch gefunden."));
        }
        let items_summary = pull_wpa_items(
            store,
            http,
            &normalize_patch(patch),
            &normalize_hero(&options.hero),
            options.min_sample_size,
            &options.rank,
            options.cache_ttl_seconds,
        )
        .await?;
        total_snapshots += snapshots_from_summary(&items_summary);
        endpoints.insert("wpa-items".to_string(), items_summary);
    }

    if contains_kind(&selected, "leaderboard") {
        let leaderboard_summary = pull_leaderboard(
            store,
            http,
            options.leaderboard_page,
            options.leaderboard_page_size,
            options.cache_ttl_seconds,
        )
        .await?;
        total_snapshots += snapshots_from_summary(&leaderboard_summary);
        endpoints.insert("leaderboard".to_string(), leaderboard_summary);
    }

    if contains_kind(&selected, "player-profile") {
        let Some(account_id) = truthy_option(options.account_id.as_deref()) else {
            return Err(SourcesError::invalid_input(
                "--account-id ist fuer player-profile erforderlich.",
            ));
        };
        let profile_summary =
            pull_player_profile(store, http, account_id, options.cache_ttl_seconds).await?;
        total_snapshots += snapshots_from_summary(&profile_summary);
        endpoints.insert("player-profile".to_string(), profile_summary);
    }

    if contains_kind(&selected, "player-matches") {
        let Some(account_id) = truthy_option(options.account_id.as_deref()) else {
            return Err(SourcesError::invalid_input(
                "--account-id ist fuer player-matches erforderlich.",
            ));
        };
        let matches = pull_player_matches(
            store,
            http,
            account_id,
            options.matches_per_player,
            options.game_mode.as_deref(),
            options.cache_ttl_seconds,
        )
        .await?;
        total_snapshots += snapshots_from_summary(&matches.summary);
        endpoints.insert("player-matches".to_string(), without_match_rows(&matches.summary));
        if options.include_match_details {
            let details_summary = pull_match_details_for_rows(
                store,
                http,
                &matches.match_rows,
                options.cache_ttl_seconds,
                options.delay_seconds,
            )
            .await?;
            total_snapshots += snapshots_from_summary(&details_summary);
            endpoints.insert("match-detail".to_string(), details_summary);
        }
        if options.include_build_analysis {
            let build_summary = pull_build_analysis_for_rows(
                store,
                http,
                &matches.match_rows,
                options.cache_ttl_seconds,
                options.delay_seconds,
            )
            .await?;
            total_snapshots += snapshots_from_summary(&build_summary);
            endpoints.insert("player-build-analysis".to_string(), build_summary);
        }
    }

    if contains_kind(&selected, "match-detail") {
        let Some(match_id) = truthy_option(options.match_id.as_deref()) else {
            return Err(SourcesError::invalid_input(
                "--match-id ist fuer match-detail erforderlich.",
            ));
        };
        let detail_summary =
            pull_match_detail(store, http, match_id, options.cache_ttl_seconds).await?;
        total_snapshots += snapshots_from_summary(&detail_summary);
        endpoints.insert("match-detail".to_string(), detail_summary);
    }

    if contains_kind(&selected, "player-build-analysis") {
        let missing_account_or_hero =
            truthy_option(options.account_id.as_deref()).is_none()
                || truthy_option(options.hero_id.as_deref()).is_none();
        if missing_account_or_hero {
            return Err(SourcesError::invalid_input(
                "--account-id und --hero-id sind fuer player-build-analysis erforderlich.",
            ));
        }
        let account_id = options.account_id.as_deref().unwrap_or_default();
        let hero_id = options.hero_id.as_deref().unwrap_or_default();
        let build_summary =
            pull_player_build_analysis(store, http, account_id, hero_id, options.cache_ttl_seconds)
                .await?;
        total_snapshots += snapshots_from_summary(&build_summary);
        endpoints.insert("player-build-analysis".to_string(), build_summary);
    }

    if contains_kind(&selected, "leaderboard-player-matches") {
        let player_summary = pull_leaderboard_player_matches(store, http, options).await?;
        total_snapshots += snapshots_from_summary(&player_summary);
        endpoints.insert("leaderboard-player-matches".to_string(), player_summary);
    }

    let known = [
        "wpa-patches",
        "wpa-items",
        "leaderboard",
        "player-profile",
        "player-matches",
        "match-detail",
        "player-build-analysis",
        "leaderboard-player-matches",
    ];
    let unknown = selected
        .iter()
        .filter(|kind| !known.iter().any(|candidate| candidate == &kind.as_str()))
        .cloned()
        .collect::<BTreeSet<_>>();
    if !unknown.is_empty() {
        return Err(SourcesError::invalid_input(format!(
            "Unbekannte Statlocker-Kinds: {}",
            unknown.into_iter().collect::<Vec<_>>().join(", ")
        )));
    }

    Ok(json!({
        "endpoints": endpoints,
        "snapshots": total_snapshots,
    }))
}

async fn pull_wpa_patches(
    store: &SourceStore<'_>,
    http: &HttpClient,
    cache_ttl_seconds: u64,
) -> Result<(Value, Option<String>)> {
    let url = format!("{BASE_URL}/api/info/wpa-patches");
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/vision/wpa"),
        cache_ttl_seconds,
    )?;
    let rows = payload.as_array().cloned().unwrap_or_default();
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, "wpa-patches", &raw, "json")?;
    let metadata = json!({ "cache_ttl_seconds": cache_ttl_seconds });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: "wpa-patches",
            title: Some("Statlocker WPA patches"),
            url: Some(&url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::new();
    let mut latest_patch = None;
    for row in &rows {
        let Value::Object(object) = row else {
            continue;
        };
        let patch_id = python_or_string(object.get("minorPatchId"))
            .unwrap_or_default()
            .trim()
            .to_string();
        if patch_id.is_empty() {
            continue;
        }
        let external_id = format!("patch_{patch_id}");
        if latest_patch.is_none() {
            latest_patch = Some(external_id.clone());
        }
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "statlocker_wpa_patch".to_string(),
            external_id: external_id.clone(),
            canonical_name: Some(external_id),
            payload: with_source_metadata(row, json!({ "source_url": &url })),
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok((
        json!({
            "url": url,
            "patches": rows.len(),
            "snapshots": count,
            "latest_patch": &latest_patch,
        }),
        latest_patch,
    ))
}

async fn pull_wpa_items(
    store: &SourceStore<'_>,
    http: &HttpClient,
    patch: &str,
    hero: &str,
    min_sample_size: u64,
    rank: &str,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let safe_min_sample_size = min_sample_size.max(1);
    let params = [
        ("hero", hero.to_string()),
        ("tier", "all".to_string()),
        ("rank", rank.to_string()),
        ("category", "all".to_string()),
        ("gameState", "all".to_string()),
        ("purchaseTime", "all".to_string()),
        ("teamComp", "Average Comp".to_string()),
        ("buildType", "all".to_string()),
        ("patch", patch.to_string()),
        ("minSampleSize", safe_min_sample_size.to_string()),
        ("searchTerm", String::new()),
        ("sortBy", "wpa".to_string()),
    ];
    let url = format!(
        "{BASE_URL}/api/info/wpa-filtered-items?{}",
        form_urlencode(&params)
    );
    let referer = format!("{BASE_URL}/vision/wpa?min={min_sample_size}&mode=items-heroes&patch={patch}");
    let payload = get_statlocker_json(http, &url, &referer, cache_ttl_seconds)?;
    let raw = json_bytes(&payload)?;
    let external_id = format!("wpa-items:{patch}:{hero}:min{min_sample_size}:rank{rank}");
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Statlocker WPA items {patch} {hero}");
    let metadata = json!({
        "patch": patch,
        "hero": hero,
        "min_sample_size": min_sample_size,
        "rank": rank,
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

    let items = payload
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut snapshots = Vec::new();
    for item in &items {
        let Value::Object(object) = item else {
            continue;
        };
        let item_name = python_or_string(object.get("item"))
            .unwrap_or_default()
            .trim()
            .to_string();
        if item_name.is_empty() {
            continue;
        }
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "statlocker_wpa_item".to_string(),
            external_id: format!("{patch}:{hero}:{item_name}"),
            canonical_name: Some(item_name.clone()),
            payload: with_source_metadata(
                item,
                json!({
                    "source_url": &url,
                    "patch": patch,
                    "requested_hero": hero,
                    "min_sample_size": min_sample_size,
                    "rank": rank,
                    "signal_kind": "item_wpa",
                }),
            ),
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "patch": patch,
        "hero": hero,
        "items": items.len(),
        "snapshots": count,
    }))
}

async fn pull_leaderboard(
    store: &SourceStore<'_>,
    http: &HttpClient,
    page: u64,
    page_size: u64,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let safe_page = page.max(1);
    let safe_page_size = page_size.clamp(1, 100);
    let query = form_urlencode(&[
        ("page", safe_page.to_string()),
        ("pageSize", safe_page_size.to_string()),
        ("version", "2".to_string()),
    ]);
    let url = format!("{BASE_URL}/api/leaderboard/get-pp-rankings/?{query}");
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/statlocker-leaderboard"),
        cache_ttl_seconds,
    )?;
    let raw = json_bytes(&payload)?;
    let external_id = format!("leaderboard:global:page{safe_page}:size{safe_page_size}:v2");
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Statlocker leaderboard page {safe_page}");
    let metadata = json!({ "page": safe_page, "page_size": safe_page_size, "version": 2 });
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

    let rows = payload
        .get("data")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut snapshots = Vec::new();
    for row in &rows {
        let Value::Object(object) = row else {
            continue;
        };
        let account_id = python_or_string(object.get("accountId"))
            .unwrap_or_default()
            .trim()
            .to_string();
        if account_id.is_empty() {
            continue;
        }
        let canonical_name =
            python_or_string(object.get("name")).unwrap_or_else(|| account_id.clone());
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "statlocker_leaderboard_player".to_string(),
            external_id: account_id,
            canonical_name: Some(canonical_name),
            payload: with_source_metadata(
                row,
                json!({ "source_url": &url, "leaderboard_page": safe_page }),
            ),
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "page": payload.get("page").cloned().unwrap_or(json!(safe_page)),
        "total_count": payload.get("totalCount").cloned().unwrap_or(Value::Null),
        "players": rows.len(),
        "snapshots": count,
    }))
}

async fn pull_player_profile(
    store: &SourceStore<'_>,
    http: &HttpClient,
    account_id: &str,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let safe_account_id = safe_required_id(account_id, "account_id")?;
    let url = format!(
        "{BASE_URL}/api/profile/steam-profile/{}",
        quote_path(&safe_account_id)
    );
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/profile/{safe_account_id}"),
        cache_ttl_seconds,
    )?;
    let document_id = store_json_document(
        store,
        &format!("player-profile:{safe_account_id}"),
        &format!("Statlocker player profile {safe_account_id}"),
        &url,
        &payload,
        &json!({ "account_id": &safe_account_id, "cache_ttl_seconds": cache_ttl_seconds }),
    )
    .await?;
    let profile_name = first_string(
        &payload,
        &["name", "personaName", "personaname", "steamName", "displayName"],
    )
    .unwrap_or_else(|| safe_account_id.clone());
    let snapshots = [EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "statlocker_player_profile".to_string(),
        external_id: safe_account_id.clone(),
        canonical_name: Some(profile_name),
        payload: with_source_metadata(
            &payload,
            json!({ "source_url": &url, "account_id": &safe_account_id }),
        ),
    }];
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({ "url": url, "account_id": safe_account_id, "snapshots": count }))
}

async fn pull_player_matches(
    store: &SourceStore<'_>,
    http: &HttpClient,
    account_id: &str,
    limit: u64,
    game_mode: Option<&str>,
    cache_ttl_seconds: u64,
) -> Result<PlayerMatchesResult> {
    let safe_account_id = safe_required_id(account_id, "account_id")?;
    let safe_limit = limit.clamp(1, 50);
    let mode_value = game_mode_value(game_mode);
    let mut params = vec![
        ("offset", "0".to_string()),
        ("limit", safe_limit.to_string()),
    ];
    if let Some(mode_value) = &mode_value {
        params.push(("gameMode", mode_value.clone()));
    }
    let query = form_urlencode(&params);
    let url = format!(
        "{BASE_URL}/api/profile/data/matches/{}/concise?{query}",
        quote_path(&safe_account_id)
    );
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/profile/{safe_account_id}"),
        cache_ttl_seconds,
    )?;
    let rows = extract_rows(&payload);
    let external_id = format!(
        "player-matches:{safe_account_id}:limit{safe_limit}:{}",
        mode_value.as_deref().unwrap_or("all")
    );
    let document_id = store_json_document(
        store,
        &external_id,
        &format!("Statlocker player matches {safe_account_id}"),
        &url,
        &payload,
        &json!({ "account_id": &safe_account_id, "limit": safe_limit, "game_mode": game_mode }),
    )
    .await?;

    let mut snapshots = Vec::new();
    let mut match_rows = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        if !row.is_object() {
            continue;
        }
        let match_id = match_id_from_payload(row).unwrap_or_else(|| format!("row{index}"));
        let hero_id = hero_id_from_payload(row);
        let row_payload = with_source_metadata(
            row,
            json!({
                "source_url": &url,
                "account_id": &safe_account_id,
                "match_id": &match_id,
                "hero_id": &hero_id,
                "signal_kind": "player_match",
            }),
        );
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "statlocker_player_match".to_string(),
            external_id: format!("{safe_account_id}:{match_id}"),
            canonical_name: Some(format!("{safe_account_id}:{match_id}")),
            payload: row_payload.clone(),
        });
        match_rows.push(MatchRow {
            account_id: safe_account_id.clone(),
            match_id,
            hero_id,
            payload: row_payload,
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    let match_row_values = match_rows
        .iter()
        .map(MatchRow::to_value)
        .collect::<Vec<_>>();
    Ok(PlayerMatchesResult {
        summary: json!({
            "url": url,
            "account_id": safe_account_id,
            "matches": match_rows.len(),
            "snapshots": count,
            "match_rows": match_row_values,
        }),
        match_rows,
    })
}

async fn pull_match_detail(
    store: &SourceStore<'_>,
    http: &HttpClient,
    match_id: &str,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let safe_match_id = safe_required_id(match_id, "match_id")?;
    let url = format!("{BASE_URL}/api/match/{}", quote_path(&safe_match_id));
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/match/{safe_match_id}"),
        cache_ttl_seconds,
    )?;
    let document_id = store_json_document(
        store,
        &format!("match-detail:{safe_match_id}"),
        &format!("Statlocker match detail {safe_match_id}"),
        &url,
        &payload,
        &json!({ "match_id": &safe_match_id, "cache_ttl_seconds": cache_ttl_seconds }),
    )
    .await?;
    let snapshots = [EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "statlocker_match_detail".to_string(),
        external_id: safe_match_id.clone(),
        canonical_name: Some(safe_match_id.clone()),
        payload: with_source_metadata(
            &payload,
            json!({ "source_url": &url, "match_id": &safe_match_id }),
        ),
    }];
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({ "url": url, "match_id": safe_match_id, "snapshots": count }))
}

async fn pull_player_build_analysis(
    store: &SourceStore<'_>,
    http: &HttpClient,
    account_id: &str,
    hero_id: &str,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let safe_account_id = safe_required_id(account_id, "account_id")?;
    let safe_hero_id = safe_required_id(hero_id, "hero_id")?;
    let url = format!(
        "{BASE_URL}/api/info/player-build-analysis/{}/{}",
        quote_path(&safe_account_id),
        quote_path(&safe_hero_id)
    );
    let payload = get_statlocker_json(
        http,
        &url,
        &format!("{BASE_URL}/profile/{safe_account_id}"),
        cache_ttl_seconds,
    )?;
    let document_id = store_json_document(
        store,
        &format!("player-build-analysis:{safe_account_id}:{safe_hero_id}"),
        &format!("Statlocker player build analysis {safe_account_id} hero {safe_hero_id}"),
        &url,
        &payload,
        &json!({
            "account_id": &safe_account_id,
            "hero_id": &safe_hero_id,
            "cache_ttl_seconds": cache_ttl_seconds,
        }),
    )
    .await?;
    let snapshots = [EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "statlocker_player_build_analysis".to_string(),
        external_id: format!("{safe_account_id}:{safe_hero_id}"),
        canonical_name: Some(format!("{safe_account_id}:{safe_hero_id}")),
        payload: with_source_metadata(
            &payload,
            json!({
                "source_url": &url,
                "account_id": &safe_account_id,
                "hero_id": &safe_hero_id,
            }),
        ),
    }];
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "account_id": safe_account_id,
        "hero_id": safe_hero_id,
        "snapshots": count,
    }))
}

async fn pull_leaderboard_player_matches(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullStatlockerOptions,
) -> Result<Value> {
    let players = latest_leaderboard_accounts(store.pool(), options.players_from_leaderboard).await?;
    let mut snapshots = 0usize;
    let mut results = Vec::new();
    for (index, player) in players.iter().enumerate() {
        if player.account_id.is_empty() {
            continue;
        }
        let mut player_result = Map::new();
        player_result.insert("account_id".to_string(), json!(&player.account_id));
        player_result.insert(
            "name".to_string(),
            player
                .name
                .as_ref()
                .map(|value| json!(value))
                .unwrap_or(Value::Null),
        );
        let profile_summary =
            pull_player_profile(store, http, &player.account_id, options.cache_ttl_seconds).await?;
        snapshots += snapshots_from_summary(&profile_summary);
        player_result.insert("profile".to_string(), profile_summary);
        sleep_delay(options.delay_seconds);
        let matches = pull_player_matches(
            store,
            http,
            &player.account_id,
            options.matches_per_player,
            options.game_mode.as_deref(),
            options.cache_ttl_seconds,
        )
        .await?;
        snapshots += snapshots_from_summary(&matches.summary);
        player_result.insert("matches".to_string(), without_match_rows(&matches.summary));
        if options.include_match_details {
            sleep_delay(options.delay_seconds);
            let details_summary = pull_match_details_for_rows(
                store,
                http,
                &matches.match_rows,
                options.cache_ttl_seconds,
                options.delay_seconds,
            )
            .await?;
            snapshots += snapshots_from_summary(&details_summary);
            player_result.insert("match_details".to_string(), details_summary);
        }
        if options.include_build_analysis {
            sleep_delay(options.delay_seconds);
            let build_summary = pull_build_analysis_for_rows(
                store,
                http,
                &matches.match_rows,
                options.cache_ttl_seconds,
                options.delay_seconds,
            )
            .await?;
            snapshots += snapshots_from_summary(&build_summary);
            player_result.insert("build_analysis".to_string(), build_summary);
        }
        results.push(Value::Object(player_result));
        if index + 1 < players.len() {
            sleep_delay(options.delay_seconds);
        }
    }
    Ok(json!({
        "players_selected": players.len(),
        "matches_per_player": options.matches_per_player.clamp(1, 50),
        "include_match_details": options.include_match_details,
        "include_build_analysis": options.include_build_analysis,
        "snapshots": snapshots,
        "players": results,
    }))
}

async fn pull_match_details_for_rows(
    store: &SourceStore<'_>,
    http: &HttpClient,
    rows: &[MatchRow],
    cache_ttl_seconds: u64,
    delay_seconds: f64,
) -> Result<Value> {
    let match_ids = unique_match_ids(rows);
    let mut snapshots = 0usize;
    let mut pulled = Vec::new();
    for (index, match_id) in match_ids.iter().enumerate() {
        let detail = pull_match_detail(store, http, match_id, cache_ttl_seconds).await?;
        snapshots += snapshots_from_summary(&detail);
        pulled.push(json!({ "match_id": match_id, "snapshots": detail.get("snapshots").cloned().unwrap_or(Value::Null) }));
        if index + 1 < match_ids.len() {
            sleep_delay(delay_seconds);
        }
    }
    Ok(json!({ "matches": pulled.len(), "snapshots": snapshots, "pulled": pulled }))
}

async fn pull_build_analysis_for_rows(
    store: &SourceStore<'_>,
    http: &HttpClient,
    rows: &[MatchRow],
    cache_ttl_seconds: u64,
    delay_seconds: f64,
) -> Result<Value> {
    let pairs = rows
        .iter()
        .filter(|row| !row.account_id.is_empty())
        .filter_map(|row| {
            row.hero_id
                .as_ref()
                .filter(|hero_id| !hero_id.is_empty())
                .map(|hero_id| (row.account_id.clone(), hero_id.clone()))
        })
        .collect::<BTreeSet<_>>();
    let mut snapshots = 0usize;
    let mut pulled = Vec::new();
    for (index, (account_id, hero_id)) in pairs.iter().enumerate() {
        let build =
            pull_player_build_analysis(store, http, account_id, hero_id, cache_ttl_seconds).await?;
        snapshots += snapshots_from_summary(&build);
        pulled.push(json!({
            "account_id": account_id,
            "hero_id": hero_id,
            "snapshots": build.get("snapshots").cloned().unwrap_or(Value::Null),
        }));
        if index + 1 < pairs.len() {
            sleep_delay(delay_seconds);
        }
    }
    Ok(json!({ "pairs": pulled.len(), "snapshots": snapshots, "pulled": pulled }))
}

fn get_statlocker_json(
    http: &HttpClient,
    url: &str,
    referer: &str,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let result = http.get(
        url,
        HttpGetOptions {
            cache_ttl_seconds: Some(cache_ttl_seconds),
            timeout: Duration::from_secs(30),
            headers: vec![
                ("Accept".to_string(), "application/json".to_string()),
                ("Origin".to_string(), BASE_URL.to_string()),
                ("Referer".to_string(), referer.to_string()),
            ],
            ..HttpGetOptions::default()
        },
    )?;
    Ok(serde_json::from_str(&result.text())?)
}

async fn store_json_document(
    store: &SourceStore<'_>,
    external_id: &str,
    title: &str,
    url: &str,
    payload: &Value,
    metadata: &Value,
) -> Result<i64> {
    let raw = json_bytes(payload)?;
    let raw_path = store.write_raw(SOURCE, external_id, &raw, "json")?;
    store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id,
            title: Some(title),
            url: Some(url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata,
        })
        .await
}

/// Neueste Leaderboard-Spieler je `external_id` aus `brain.entity_snapshots`.
/// PG-Port des frueheren SQLite-`json_extract`-Reads: `DISTINCT ON` waehlt pro
/// Spieler den juengsten Snapshot; die Sortierung nach Rang/Seite toleriert
/// nicht-numerische Werte (regex-Guard) statt an einem CAST zu scheitern.
async fn latest_leaderboard_accounts(
    pool: &PgPool,
    limit: u64,
) -> Result<Vec<LeaderboardPlayer>> {
    let safe_limit = limit.clamp(1, 50) as i64;
    let players = sqlx::query!(
        r#"
        SELECT latest.external_id AS "external_id!",
               latest.canonical_name AS "canonical_name?"
        FROM (
            SELECT DISTINCT ON (external_id)
                   external_id, canonical_name, payload, fetched_at
            FROM brain.entity_snapshots
            WHERE source = $1 AND entity_type = 'statlocker_leaderboard_player'
            ORDER BY external_id, fetched_at DESC
        ) latest
        ORDER BY
            CASE WHEN latest.payload->>'rank' ~ '^-?[0-9]+$'
                 THEN (latest.payload->>'rank')::int ELSE 2147483647 END ASC,
            CASE WHEN latest.payload->>'leaderboard_page' ~ '^-?[0-9]+$'
                 THEN (latest.payload->>'leaderboard_page')::int ELSE 2147483647 END ASC,
            latest.fetched_at DESC
        LIMIT $2
        "#,
        SOURCE,
        safe_limit,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| LeaderboardPlayer {
        account_id: row.external_id,
        name: row.canonical_name,
    })
    .collect();
    Ok(players)
}

fn extract_rows(payload: &Value) -> Vec<Value> {
    if let Value::Array(rows) = payload {
        return rows.clone();
    }
    let Some(object) = payload.as_object() else {
        return Vec::new();
    };
    for key in [
        "matchHistory",
        "match_history",
        "data",
        "matches",
        "rows",
        "results",
        "items",
    ] {
        if let Some(rows) = object.get(key).and_then(Value::as_array) {
            return rows.clone();
        }
    }
    Vec::new()
}

fn match_id_from_payload(payload: &Value) -> Option<String> {
    let object = payload.as_object()?;
    for key in ["match_id", "matchId", "id", "match"] {
        if let Some(value) = object.get(key) {
            let raw = value_to_python_string(value).trim().to_string();
            if !raw.is_empty() && !value.is_null() {
                return Some(raw);
            }
        }
    }
    object
        .get("_deadlock_brain")
        .and_then(Value::as_object)
        .and_then(|metadata| metadata.get("match_id"))
        .map(value_to_python_string)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn hero_id_from_payload(payload: &Value) -> Option<String> {
    let object = payload.as_object()?;
    for key in ["hero_id", "heroId", "player_hero_id", "playerHeroId", "hero"] {
        if let Some(value) = object.get(key) {
            let raw = value_to_python_string(value).trim().to_string();
            if !raw.is_empty() && !value.is_null() {
                return Some(raw);
            }
        }
    }
    object
        .get("_deadlock_brain")
        .and_then(Value::as_object)
        .and_then(|metadata| metadata.get("hero_id"))
        .map(value_to_python_string)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn unique_match_ids(rows: &[MatchRow]) -> Vec<String> {
    let mut result = Vec::new();
    for row in rows {
        let match_id = match_id_from_payload(&row.payload).unwrap_or_else(|| row.match_id.clone());
        if !match_id.is_empty() && !result.iter().any(|existing| existing == &match_id) {
            result.push(match_id);
        }
    }
    result
}

fn game_mode_value(game_mode: Option<&str>) -> Option<String> {
    let raw = game_mode.unwrap_or_default().trim().to_lowercase();
    if raw.is_empty() || raw == "all" {
        None
    } else if matches!(raw.as_str(), "standard" | "normal" | "ranked") {
        Some("1".to_string())
    } else if matches!(raw.as_str(), "brawl" | "street-brawl" | "street_brawl") {
        Some("4".to_string())
    } else {
        Some(raw)
    }
}

fn safe_required_id(value: &str, label: &str) -> Result<String> {
    let raw = value.trim();
    if raw.is_empty() {
        return Err(SourcesError::invalid_input(format!("{label} fehlt.")));
    }
    Ok(raw.to_string())
}

fn first_string(payload: &Value, keys: &[&str]) -> Option<String> {
    let object = payload.as_object()?;
    for key in keys {
        let Some(value) = object.get(*key) else {
            continue;
        };
        if value.is_null() {
            continue;
        }
        let raw = value_to_python_string(value).trim().to_string();
        if !raw.is_empty() {
            return Some(raw);
        }
    }
    None
}

fn without_match_rows(summary: &Value) -> Value {
    let Some(object) = summary.as_object() else {
        return summary.clone();
    };
    let mut copied = object.clone();
    copied.remove("match_rows");
    Value::Object(copied)
}

fn normalize_patch(patch: &str) -> String {
    let raw = patch.trim();
    if raw.is_empty() || raw.starts_with("patch_") {
        raw.to_string()
    } else {
        format!("patch_{raw}")
    }
}

fn normalize_hero(hero: &str) -> String {
    let raw = hero.trim();
    if raw.is_empty() || raw.eq_ignore_ascii_case("all") {
        "all".to_string()
    } else {
        raw.replace(" & ", "_and_")
            .replace('&', "and")
            .replace(' ', "_")
    }
}

fn with_source_metadata(payload: &Value, metadata: Value) -> Value {
    let mut copied = payload.as_object().cloned().unwrap_or_else(|| {
        let mut object = Map::new();
        object.insert("value".to_string(), payload.clone());
        object
    });
    copied.insert("_deadlock_brain".to_string(), metadata);
    Value::Object(copied)
}

fn contains_kind(selected: &[String], kind: &str) -> bool {
    selected.iter().any(|selected| selected == kind)
}

fn snapshots_from_summary(summary: &Value) -> usize {
    summary
        .get("snapshots")
        .and_then(Value::as_u64)
        .map(|value| value as usize)
        .unwrap_or(0)
}

fn truthy_option(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.is_empty())
}

fn sleep_delay(delay_seconds: f64) {
    if delay_seconds.is_finite() && delay_seconds > 0.0 {
        thread::sleep(Duration::from_secs_f64(delay_seconds));
    }
}

#[derive(Debug, Clone)]
struct PlayerMatchesResult {
    summary: Value,
    match_rows: Vec<MatchRow>,
}

#[derive(Debug, Clone)]
struct MatchRow {
    account_id: String,
    match_id: String,
    hero_id: Option<String>,
    payload: Value,
}

impl MatchRow {
    fn to_value(&self) -> Value {
        json!({
            "account_id": &self.account_id,
            "match_id": &self.match_id,
            "hero_id": &self.hero_id,
            "payload": &self.payload,
        })
    }
}

#[derive(Debug, Clone)]
struct LeaderboardPlayer {
    account_id: String,
    name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pure-Logic-Ports: die frueheren End-to-End-Tests schrieben mit
    // realer `statlocker`-Quelle in `entity_snapshots`/`source_documents`.
    // Gegen die geteilte Scratch-PG mit echten Daten waere das destruktiv und
    // wuerde Zaehler veraendern. Die DB-Schreibpfade decken `store.rs`
    // (Round-Trip) und die compile-geprueften `query!` in dieser Datei ab; hier
    // bleibt die reine Ableitungs-/Normalisierungslogik.

    #[test]
    fn normalize_patch_prefixes_only_when_needed() {
        assert_eq!(normalize_patch("129989"), "patch_129989");
        assert_eq!(normalize_patch("patch_5"), "patch_5");
        assert_eq!(normalize_patch("   "), "");
    }

    #[test]
    fn normalize_hero_handles_all_and_separators() {
        assert_eq!(normalize_hero("All"), "all");
        assert_eq!(normalize_hero(""), "all");
        assert_eq!(normalize_hero("Grey Talon"), "Grey_Talon");
        assert_eq!(normalize_hero("Mo & Krill"), "Mo_and_Krill");
    }

    #[test]
    fn game_mode_value_maps_known_modes() {
        assert_eq!(game_mode_value(Some("all")), None);
        assert_eq!(game_mode_value(None), None);
        assert_eq!(game_mode_value(Some("ranked")), Some("1".to_string()));
        assert_eq!(game_mode_value(Some("brawl")), Some("4".to_string()));
        assert_eq!(game_mode_value(Some("custom")), Some("custom".to_string()));
    }

    #[test]
    fn extract_rows_reads_arrays_and_keyed_objects() {
        assert_eq!(extract_rows(&json!([{"a": 1}])).len(), 1);
        assert_eq!(
            extract_rows(&json!({"matchHistory": [{"a": 1}, {"b": 2}]})).len(),
            2
        );
        assert!(extract_rows(&json!({"nope": 1})).is_empty());
    }

    #[test]
    fn match_and_hero_id_from_payload_prefer_direct_keys() {
        let row = json!({"matchId": "m1", "heroId": "h1"});
        assert_eq!(match_id_from_payload(&row).as_deref(), Some("m1"));
        assert_eq!(hero_id_from_payload(&row).as_deref(), Some("h1"));
        let meta = json!({"_deadlock_brain": {"match_id": "m9"}});
        assert_eq!(match_id_from_payload(&meta).as_deref(), Some("m9"));
    }
}
