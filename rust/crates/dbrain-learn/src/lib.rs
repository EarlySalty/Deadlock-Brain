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
pub use build_optimizer::{build_hero_build_context, build_suggest, BuildSuggestOptions};
pub use error::{LearnError, Result};
pub use match_coaching::build_match_coaching_context;
pub use player_decision_learning::{
    build_minimax_player_match_decision_request, build_player_match_decision_context,
    player_analyze_match, player_analyze_next, player_list_matches, player_match_context,
    PlayerAnalyzeMatchOptions, PlayerAnalyzeNextOptions, PLAYER_DECISION_PROMPT_VERSION,
};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use deadlock_brain_core::{config::Settings, minimax::MiniMaxConfig};
    use sqlx::postgres::{PgPool, PgPoolOptions};

    use super::*;

    /// Reiner Logik-Test ohne Datenbank: der Insights-Parser findet den letzten
    /// gefencten Modell-JSON-Block.
    #[test]
    fn extract_insights_reads_fenced_minimax_json_at_end() {
        let text = r#"
        # Analyse

        Vorheriger JSON-Block, der nicht relevant ist:
        {"status":"draft"}

        ```json
        {
          "verdict": "ok",
          "insights": {
            "hero_job": "spirit_poke",
            "build_variant": "cooldown_spirit",
            "core_items": ["Improved Spirit", "Superior Cooldown"],
            "situational_items": ["Reactive Barrier"],
            "avoid_or_question": ["Burst Fire"],
            "timing_rules": ["Cooldown nach erstem Core"],
            "scoring_hints": ["Spirit vor Weapon"]
          }
        }
        ```

        Nachsatz: nicht jeder Block mit {Klammern} ist JSON.
        "#;

        let insights = crate::util::extract_insights(text);

        assert_eq!(insights["hero_job"], "spirit_poke");
        assert_eq!(insights["core_items"][0], "Improved Spirit");
        assert_eq!(insights["core_items"][1], "Superior Cooldown");
    }

    /// Wegwerf-Postgres aus `DEADLOCK_CENTRAL_DSN`. `None` (Test-Skip), wenn die
    /// Variable nicht gesetzt ist — genau wie im bereits portierten `dbrain-enrich`.
    async fn test_pool() -> Option<PgPool> {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
        PgPoolOptions::new()
            .max_connections(2)
            .connect(&dsn)
            .await
            .ok()
    }

    fn test_minimax_config() -> MiniMaxConfig {
        let settings = Settings {
            project_root: PathBuf::from("/tmp/dbrain-learn-test"),
            data_dir: PathBuf::from("/tmp/dbrain-learn-test/data"),
            raw_dir: PathBuf::from("/tmp/dbrain-learn-test/raw"),
            cache_dir: PathBuf::from("/tmp/dbrain-learn-test/cache"),
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

    /// Paritaet: `learn_list_builds` liefert exakt so viele Zeilen wie die Tabelle
    /// `brain.learned_builds` enthaelt (Live-Read gegen echtes Schema).
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn list_builds_matches_learned_builds_count() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let db_count: i64 = sqlx::query_scalar("SELECT count(*)::int8 FROM brain.learned_builds")
            .fetch_one(&pool)
            .await
            .expect("count learned_builds");
        let listed = learn_list_builds(&pool, None, 100_000)
            .await
            .expect("learn_list_builds");
        assert_eq!(listed.len() as i64, db_count);
        assert!(db_count > 0, "scratch PG sollte learned_builds enthalten");
        // Jede Zeile hat die dekodierte item_names-Liste.
        assert!(listed.iter().all(|row| row.get("item_names").is_some()));
    }

    /// Voller Read-Pfad: `build_hero_build_context` fuer einen echten Hero fuehrt
    /// alle `load_*`/Review-/Entity-Queries gegen die Scratch-PG aus.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn build_hero_build_context_for_real_hero() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let hero: Option<String> = sqlx::query_scalar(
            "SELECT canonical_name FROM brain.entities WHERE entity_type='hero' ORDER BY canonical_name LIMIT 1",
        )
        .fetch_optional(&pool)
        .await
        .expect("hero lookup");
        let Some(hero) = hero else {
            return; // keine Heroes in der Scratch-PG -> nichts zu pruefen
        };
        let context = build_hero_build_context(&pool, &hero, &[], 40)
            .await
            .expect("build context");
        assert_eq!(context["hero"]["name"].as_str(), Some(hero.as_str()));
        assert!(context.get("build").and_then(|b| b.as_object()).is_some());
        assert!(context
            .get("top_items")
            .and_then(|t| t.as_array())
            .is_some());
    }

    /// Statlocker-Player-Match-Query laeuft ohne Fehler.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn player_list_matches_runs() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let rows = player_list_matches(&pool, None, 10)
            .await
            .expect("player_list_matches");
        // Jede Zeile hat account_id und match_id.
        assert!(rows.iter().all(|row| row.get("account_id").is_some()));
    }

    /// Insert-/JSONB-/ON-CONFLICT-Pfad fuer `build_learning_notes`: Dry-Run gegen
    /// einen echten `learned_build`, danach Aufraeumen der Testnotiz.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn analyze_build_dry_run_inserts_note() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let build_id: Option<i64> =
            sqlx::query_scalar("SELECT id FROM brain.learned_builds ORDER BY id LIMIT 1")
                .fetch_optional(&pool)
                .await
                .expect("learned_build id");
        let Some(build_id) = build_id else {
            return;
        };
        let result = learn_analyze_build(
            &pool,
            LearnAnalyzeBuildOptions {
                build_id,
                config: test_minimax_config(),
                dry_run: true,
                include_request: true,
            },
        )
        .await
        .expect("dry run");

        assert_eq!(result["dry_run"], true);
        assert_eq!(result["note"]["status"], "context_ready");
        assert!(result.get("request").is_some());

        if let Some(note_id) = result["note"]["id"].as_i64() {
            sqlx::query("DELETE FROM brain.build_learning_notes WHERE id=$1")
                .bind(note_id)
                .execute(&pool)
                .await
                .expect("cleanup note");
        }
    }

    /// Insert-/JSONB-/ON-CONFLICT-Pfad fuer `player_match_decision_notes`: Dry-Run
    /// gegen ein echtes Statlocker-Player-Match, danach Aufraeumen der Testnotiz.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn analyze_match_dry_run_inserts_note() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let external: Option<String> = sqlx::query_scalar(
            "SELECT external_id FROM brain.entity_snapshots \
             WHERE source='statlocker' AND entity_type='statlocker_player_match' \
             ORDER BY fetched_at DESC LIMIT 1",
        )
        .fetch_optional(&pool)
        .await
        .expect("player match lookup");
        let Some(external) = external else {
            return;
        };
        let Some((account_id, match_id)) = external.split_once(':') else {
            return;
        };

        let result = player_analyze_match(
            &pool,
            PlayerAnalyzeMatchOptions {
                account_id: account_id.to_string(),
                match_id: match_id.to_string(),
                config: test_minimax_config(),
                dry_run: true,
                include_request: true,
            },
        )
        .await
        .expect("player dry run");

        assert_eq!(result["dry_run"], true);
        assert!(result.get("request").is_some());

        if let Some(note_id) = result["note"]["id"].as_i64() {
            sqlx::query("DELETE FROM brain.player_match_decision_notes WHERE id=$1")
                .bind(note_id)
                .execute(&pool)
                .await
                .expect("cleanup note");
        }
    }
}
