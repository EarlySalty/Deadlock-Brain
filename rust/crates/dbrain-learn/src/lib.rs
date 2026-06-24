#![forbid(unsafe_code)]

//! Lern-Crate fuer Build-, Player- und YouTube-Analyse-Workflows.

pub use deadlock_brain_core as core;

mod build_learning;
mod build_optimizer;
mod error;
mod match_coaching;
mod player_decision_learning;
mod util;

pub use build_learning::{
    build_learning_context, build_minimax_build_learning_request, learn_analyze_build,
    learn_analyze_next, learn_import_steam_builds, learn_list_builds, LearnAnalyzeBuildOptions,
    LearnAnalyzeNextOptions, LearnImportSteamBuildsOptions, BUILD_LEARNING_PROMPT_VERSION,
};
pub use build_optimizer::{build_suggest, build_hero_build_context, BuildSuggestOptions};
pub use error::{LearnError, Result};
pub use match_coaching::build_match_coaching_context;
pub use player_decision_learning::{
    build_minimax_player_match_decision_request, build_player_match_decision_context,
    player_analyze_match, player_analyze_next, player_list_matches, player_match_context,
    PlayerAnalyzeMatchOptions, PlayerAnalyzeNextOptions, PLAYER_DECISION_PROMPT_VERSION,
};

#[cfg(test)]
mod tests {
    use std::path::Path;

    use deadlock_brain_core::{config::Settings, minimax::MiniMaxConfig};
    use rusqlite::Connection;
    use serde_json::json;

    use super::*;

    #[test]
    fn imports_steam_builds_and_lists_item_names() {
        let temp = tempfile::tempdir().expect("tempdir");
        let conn = test_conn(temp.path());
        seed_assets(&conn);
        let steam_path = temp.path().join("steam.sqlite3");
        seed_steam_db(&steam_path);

        let result = learn_import_steam_builds(
            &conn,
            LearnImportSteamBuildsOptions {
                steam_db_path: Some(steam_path),
                hero: Some("Test".to_string()),
                language: 0,
                limit_per_hero: 10,
            },
        )
        .expect("import builds");

        assert_eq!(result["imported"], 1);
        let rows = learn_list_builds(&conn, Some("TestHero"), 25).expect("list builds");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["item_names"][0], "Extra Stamina");
    }

    #[test]
    fn build_suggest_returns_deterministic_build() {
        let temp = tempfile::tempdir().expect("tempdir");
        let conn = test_conn(temp.path());
        seed_assets(&conn);

        let result = build_suggest(&conn, BuildSuggestOptions::new("TestHero")).expect("suggest");

        assert_eq!(result["hero"]["name"], "TestHero");
        assert!(result["build"]["early"].as_array().expect("early array").len() > 0);
    }

    #[test]
    fn learn_analyze_build_dry_run_stores_context_without_api_call() {
        let temp = tempfile::tempdir().expect("tempdir");
        let conn = test_conn(temp.path());
        seed_assets(&conn);
        seed_learned_build(&conn);
        let config = test_minimax_config(temp.path());

        let result = learn_analyze_build(
            &conn,
            LearnAnalyzeBuildOptions {
                build_id: 1,
                config,
                dry_run: true,
                include_request: true,
            },
        )
        .expect("dry run");

        assert_eq!(result["dry_run"], true);
        assert_eq!(result["note"]["status"], "context_ready");
        assert!(result.get("request").is_some());
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM build_learning_notes", [], |row| row.get(0))
            .expect("note count");
        assert_eq!(count, 1);
    }

    #[test]
    fn player_analyze_match_dry_run_stores_context_without_api_call() {
        let temp = tempfile::tempdir().expect("tempdir");
        let conn = test_conn(temp.path());
        seed_assets(&conn);
        seed_player_match(&conn);
        let config = test_minimax_config(temp.path());

        let result = player_analyze_match(
            &conn,
            PlayerAnalyzeMatchOptions {
                account_id: "acc1".to_string(),
                match_id: "m1".to_string(),
                config,
                dry_run: true,
                include_request: true,
            },
        )
        .expect("player dry run");

        assert_eq!(result["dry_run"], true);
        assert_eq!(result["hero_name"], "TestHero");
        assert!(result.get("request").is_some());
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM player_match_decision_notes", [], |row| row.get(0))
            .expect("note count");
        assert_eq!(count, 1);
    }

    fn test_conn(temp: &Path) -> Connection {
        deadlock_brain_core::db::open_connection(Some(temp.join("brain.sqlite3"))).expect("open conn")
    }

    fn test_minimax_config(temp: &Path) -> MiniMaxConfig {
        let settings = Settings {
            project_root: temp.to_path_buf(),
            data_dir: temp.join("data"),
            raw_dir: temp.join("raw"),
            cache_dir: temp.join("cache"),
            db_path: temp.join("brain.sqlite3"),
            central_deadlock_db_path: temp.join("steam.sqlite3"),
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
            minimax_max_completion_tokens: 256,
            minimax_temperature: 0.2,
            minimax_top_p: 0.9,
            minimax_use_token_plan: false,
        };
        MiniMaxConfig::from_settings(&settings)
    }

    fn seed_assets(conn: &Connection) {
        let hero = json!({
            "id": 1,
            "name": "TestHero",
            "hero_type": "Brawler",
            "gun_tag": "gun",
            "description": {"role": "Test", "playstyle": "Test playstyle"},
            "items": {"signature1": "ability_test"},
            "item_draft_bucketing": {"item_extra_stamina": {"bucket": "Good"}},
            "cost_bonuses": {},
            "purchase_bonuses": {},
            "starting_stats": {"max_health": 600}
        });
        insert_snapshot(conn, "deadlock_assets_api", "hero", "1", "TestHero", &hero);
        let ability = json!({
            "id": 10,
            "name": "Test Stun",
            "class_name": "ability_test",
            "description": {"desc": "Stun and dash with weapon damage"},
            "properties": {
                "AbilityCooldown": {"value": 20, "label": "Cooldown", "disable_value": 1, "provided_property_type": "ETechCooldown"},
                "Damage": {"value": 100, "label": "Damage", "disable_value": 1, "scale_function": {"scaling_stats": ["ETechPower"]}}
            },
            "upgrades": []
        });
        insert_snapshot(conn, "deadlock_assets_api", "item_or_ability", "10", "Test Stun", &ability);
        let item = json!({
            "id": 100,
            "name": "Extra Stamina",
            "class_name": "item_extra_stamina",
            "item_slot_type": "vitality",
            "item_tier": 1,
            "cost": 800,
            "shopable": true,
            "disabled": false,
            "is_active_item": false,
            "description": {"desc": "Stamina and move speed for lane"},
            "properties": {
                "Stamina": {"value": 1, "label": "Stamina", "disable_value": 1},
                "MoveSpeed": {"value": 1, "label": "Move Speed", "disable_value": 1}
            }
        });
        insert_snapshot(conn, "deadlock_assets_api", "item_or_ability", "100", "Extra Stamina", &item);
    }

    fn insert_snapshot(
        conn: &Connection,
        source: &str,
        entity_type: &str,
        external_id: &str,
        canonical_name: &str,
        payload: &serde_json::Value,
    ) {
        conn.execute(
            "INSERT INTO entity_snapshots(source, entity_type, external_id, canonical_name, payload_hash, payload_json, fetched_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                source,
                entity_type,
                external_id,
                canonical_name,
                format!("{source}:{entity_type}:{external_id}"),
                payload.to_string(),
                1_i64,
            ),
        )
        .expect("insert snapshot");
    }

    fn seed_steam_db(path: &Path) {
        let conn = Connection::open(path).expect("open steam");
        conn.execute_batch(
            "CREATE TABLE hero_build_sources(
               hero_build_id TEXT, hero_id INTEGER, language INTEGER, details_json TEXT, tags_json TEXT,
               origin_build_id TEXT, version INTEGER, publish_ts INTEGER, last_updated_ts INTEGER,
               fetched_at INTEGER, last_seen_at INTEGER, name TEXT, author_account_id TEXT, description TEXT
             );",
        )
        .expect("create steam schema");
        let details = json!({
            "mod_categories": [{"name": "Core", "mods": [{"ability_id": 100, "annotation": "buy early"}]}],
            "ability_order": {"currency_changes": [{"ability_id": 10, "currency_type": "ap", "delta": 1}]}
        });
        conn.execute(
            "INSERT INTO hero_build_sources VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            (
                "b1",
                1_i64,
                0_i64,
                details.to_string(),
                json!(["tag"]).to_string(),
                "origin",
                1_i64,
                100_i64,
                200_i64,
                300_i64,
                400_i64,
                "Test Build",
                "42",
                "desc",
            ),
        )
        .expect("insert steam row");
    }

    fn seed_learned_build(conn: &Connection) {
        conn.execute(
            "INSERT INTO learned_builds(
               source, source_build_id, hero_id, hero_name, language, source_rank,
               quality_tier, quality_score, name, tags_json, details_json, item_names_json,
               ability_order_json, source_metadata_json, imported_at, updated_at
             )
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            (
                "steam_gc",
                "b1",
                1_i64,
                "TestHero",
                0_i64,
                1_i64,
                "likely_good",
                0.9_f64,
                "Test Build",
                "[]",
                json!({"mod_categories": [{"name": "Core", "mods": [{"ability_id": 100}]}]}).to_string(),
                json!(["Extra Stamina"]).to_string(),
                "[]",
                "{}",
                1_i64,
                1_i64,
            ),
        )
        .expect("insert learned build");
    }

    fn seed_player_match(conn: &Connection) {
        let player_match = json!({
            "hero_id": 1,
            "kills": 4,
            "deaths": 2,
            "assists": 8,
            "won": true,
            "_deadlock_brain": {"account_id": "acc1", "match_id": "m1", "hero_id": "1"}
        });
        insert_snapshot(conn, "statlocker", "statlocker_player_match", "acc1:m1", "acc1:m1", &player_match);
        let api_match = json!({
            "match_id": "m1",
            "players": [{
                "account_id": "acc1",
                "hero_id": 1,
                "team": 0,
                "kills": 4,
                "deaths": 2,
                "assists": 8,
                "items": [{"game_time_s": 300, "item_id": 100}]
            }]
        });
        insert_snapshot(conn, "deadlock_api", "deadlock_api_match_metadata", "m1", "m1", &api_match);
    }
}
