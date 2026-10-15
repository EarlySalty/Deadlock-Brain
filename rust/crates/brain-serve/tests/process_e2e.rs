mod common;
#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

use axum::{extract::State, routing::post, Json, Router};
use common::{Service, API_TOKEN};
use serde_json::{json, Value};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

async fn provider(State(calls): State<Arc<AtomicUsize>>, Json(body): Json<Value>) -> Json<Value> {
    calls.fetch_add(1, Ordering::SeqCst);
    let has_result = body["messages"]
        .as_array()
        .unwrap()
        .iter()
        .any(|message| message["role"] == "tool");
    let message = if has_result {
        json!({"content":json!({"text":"","cited_evidence_ids":[],"claims":[]}).to_string()})
    } else {
        let tools = body["tools"].as_array().unwrap();
        let names: Vec<_> = tools
            .iter()
            .map(|tool| tool["function"]["name"].as_str().unwrap())
            .collect();
        for name in [
            "entity_profile",
            "deadlock_data",
            "game_rules",
            "server_knowledge",
            "entity_find",
            "hero_compare",
        ] {
            assert!(!names.contains(&name));
        }
        for name in [
            "find_entity",
            "get_hero",
            "get_ability",
            "get_item",
            "get_timer",
            "compare",
        ] {
            assert!(names.contains(&name));
        }
        json!({"content":null,"tool_calls":[{"id":"find","type":"function","function":{"name":"find_entity","arguments":json!({"text":"NonexistentHero987654","kind":"hero"}).to_string()}}]})
    };
    Json(
        json!({"model":body["model"],"choices":[{"finish_reason":if has_result {"stop"} else {"tool_calls"}, "message":message}],"usage":{"prompt_tokens":1,"completion_tokens":1}}),
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn tools_only_process_needs_no_corpus_release_and_refuses_unknown_heroes() {
    let pg = scratch_pg::ScratchPg::start();
    let socket = pg.directory.join("socket");
    let options = PgConnectOptions::new_without_pgpass()
        .host(&socket.to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&pool).await.unwrap();
    let store = brain_storage::PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    store.migrate_gamefacts().await.unwrap();
    let release_count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.corpus_releases_v1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(release_count, 0);
    let calls = Arc::new(AtomicUsize::new(0));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let provider_address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let app = Router::new()
        .route("/chat/completions", post(provider))
        .with_state(calls.clone());
    let task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let mut config = common::config();
    config["postgres"]["socket_dir"] = json!(socket);
    config["postgres"]["port"] = json!(55439);
    config["postgres"]["username"] = json!("brain_core_test");
    config["postgres"]["database"] = json!("postgres");
    config["provider"]["base_url"] = json!(format!("http://{provider_address}"));
    config["deadlock_api"] = json!({});
    let mut credentials = common::credentials();
    credentials.push((
        "BRAIN_PG_READONLY_PASSWORD",
        "synthetic-unused-trust-password",
    ));
    let mut service = Service::spawn(&config, &credentials);
    let address = service.address();
    common::assert_ready(&address);
    let (_, body) = common::get(&address, "/readyz");
    assert_eq!(
        serde_json::from_str::<Value>(&body).unwrap()["serving_contract"],
        "brain.tools-only.v2"
    );
    let query = json!({"request_id":"unknown-hero","conversation_id":"unknown-hero","text":"Wie viel Leben hat NonexistentHero987654?","profile":"fact","requested_scopes":["docs.public"]});
    let result = tokio::task::spawn_blocking(move || {
        common::request(&address, "POST", "/v1/answer", Some(API_TOKEN), Some(query))
    })
    .await
    .unwrap();
    assert_eq!(result.0, 200, "{}", result.1);
    let answer: Value = serde_json::from_str(&result.1).unwrap();
    assert_eq!(answer["status"], "insufficient_evidence");
    assert!(answer["citations"].as_array().unwrap().is_empty());
    assert!(answer.get("claims").is_none());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(common::get(&service.address(), "/readyz").0, 200);
    if let Some(binary) = std::env::var_os("BRAIN_EVAL_BINARY") {
        let endpoint = service.address();
        let gold = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../brain-eval/gold/e1-gold-v1.json");
        let out = std::env::var_os("BRAIN_EVAL_OUT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| pg.directory.join("e1-scratch.json"));
        let status = tokio::task::spawn_blocking(move || {
            std::process::Command::new(binary)
                .args([
                    "--endpoint",
                    &endpoint,
                    "--scope",
                    "docs.public",
                    "--token-env",
                    "P6_EVAL_SYNTHETIC_TOKEN",
                    "--timeout-secs",
                    "10",
                ])
                .arg("--gold")
                .arg(gold)
                .arg("--out")
                .arg(out)
                .env("P6_EVAL_SYNTHETIC_TOKEN", API_TOKEN)
                .status()
                .unwrap()
        })
        .await
        .unwrap();
        assert!(status.success());
    }
    service.signal("TERM");
    assert!(service.wait(Duration::from_secs(15)).success());
    let _ = stop.send(());
    task.await.unwrap();
    pool.close().await;
}
