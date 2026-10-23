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

const RELEASE_ID: &str = "p6-process-r1";
const READONLY_PASSWORD: &str = "synthetic-unused-trust-password";

async fn provider(State(calls): State<Arc<AtomicUsize>>, Json(body): Json<Value>) -> Json<Value> {
    calls.fetch_add(1, Ordering::SeqCst);
    if body.to_string().contains("DrainProbe") {
        tokio::time::sleep(Duration::from_millis(1500)).await;
    }
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

fn hero_query(id: &str, hero: &str, scope: &str) -> Value {
    json!({"request_id":id,"conversation_id":id,"text":format!("Wie viel Leben hat {hero}?"),"profile":"fact","requested_scopes":[scope]})
}

async fn post_answer(address: String, token: &'static str, query: Value) -> (u16, String) {
    tokio::task::spawn_blocking(move || {
        common::request(&address, "POST", "/v1/answer", Some(token), Some(query))
    })
    .await
    .unwrap()
}

async fn database_pool(options: &PgConnectOptions, database: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone().database(database))
        .await
        .unwrap()
}

fn assert_secret_free(log: &str, credentials: &[(&str, &str)]) {
    for (_, secret) in credentials {
        assert!(
            !log.contains(secret),
            "service log leaked a credential value"
        );
    }
}

fn assert_startup_refused(config: &Value, credentials: &[(&str, &str)], database: &str) {
    let mut refused_config = config.clone();
    refused_config["postgres"]["database"] = json!(database);
    let mut refused = Service::spawn(&refused_config, credentials);
    let status = refused.wait(Duration::from_secs(8));
    let log = refused.log();
    assert_eq!(status.code(), Some(1), "{log}");
    assert!(log.contains("core_schema_incompatible"), "{log}");
    assert!(!log.contains("listening"));
    assert_secret_free(&log, credentials);
}

async fn assert_incompatible_schemas_are_refused_untouched(
    options: &PgConnectOptions,
    admin: &sqlx::PgPool,
    config: &Value,
    credentials: &[(&str, &str)],
) {
    for database in ["brain_schema_test", "brain_empty_schema_test"] {
        sqlx::raw_sql(&format!("CREATE DATABASE {database}"))
            .execute(admin)
            .await
            .unwrap();
    }
    let incompatible = database_pool(options, "brain_schema_test").await;
    sqlx::raw_sql(
        "CREATE SCHEMA brain; \
         CREATE TABLE brain.core_schema_version(singleton boolean PRIMARY KEY, schema_version integer NOT NULL, store_contract text NOT NULL); \
         INSERT INTO brain.core_schema_version(singleton, schema_version, store_contract) VALUES(true, 99, 'fixture-contract')",
    )
    .execute(&incompatible)
    .await
    .unwrap();
    tokio::task::block_in_place(|| {
        assert_startup_refused(config, credentials, "brain_schema_test")
    });
    let schema_version: i32 =
        sqlx::query_scalar("SELECT schema_version FROM brain.core_schema_version WHERE singleton")
            .fetch_one(&incompatible)
            .await
            .unwrap();
    assert_eq!(schema_version, 99);
    let schema_relations: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname='brain' AND c.relkind IN ('r','p','v','m','S')",
    )
    .fetch_one(&incompatible)
    .await
    .unwrap();
    assert_eq!(schema_relations, 1);
    incompatible.close().await;
    tokio::task::block_in_place(|| {
        assert_startup_refused(config, credentials, "brain_empty_schema_test")
    });
    let empty = database_pool(options, "brain_empty_schema_test").await;
    let empty_schema: bool = sqlx::query_scalar("SELECT to_regnamespace('brain') IS NOT NULL")
        .fetch_one(&empty)
        .await
        .unwrap();
    assert!(!empty_schema);
    empty.close().await;
}

async fn assert_scope_and_token_denials(address: &str, calls: &AtomicUsize) {
    let before = calls.load(Ordering::SeqCst);
    let (status, body) = post_answer(
        address.to_owned(),
        API_TOKEN,
        hero_query("scope-denied", "NonexistentHero987654", "docs.internal"),
    )
    .await;
    assert_eq!(status, 403, "{body}");
    assert!(body.contains("forbidden"), "{body}");
    let (status, body) = post_answer(
        address.to_owned(),
        "synthetic-unknown-token",
        hero_query("token-denied", "NonexistentHero987654", "docs.public"),
    )
    .await;
    assert_eq!(status, 401, "{body}");
    assert_eq!(calls.load(Ordering::SeqCst), before);
}

async fn assert_warm_schema_marker_is_enforced(pool: &sqlx::PgPool, address: &str) {
    let supported: i32 =
        sqlx::query_scalar("SELECT schema_version FROM brain.core_schema_version WHERE singleton")
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("UPDATE brain.core_schema_version SET schema_version=99 WHERE singleton")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(common::get(address, "/readyz").0, 503);
    assert_eq!(common::get(address, "/healthz").0, 200);
    sqlx::query("UPDATE brain.core_schema_version SET schema_version=$1 WHERE singleton")
        .bind(supported)
        .execute(pool)
        .await
        .unwrap();
    common::assert_ready(address);
}

async fn assert_database_outage_recovers(admin: &sqlx::PgPool, address: &str, calls: &AtomicUsize) {
    sqlx::query("ALTER DATABASE postgres ALLOW_CONNECTIONS false")
        .execute(admin)
        .await
        .unwrap();
    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='postgres' AND pid<>pg_backend_pid()",
    )
    .execute(admin)
    .await
    .unwrap();
    assert_eq!(common::get(address, "/readyz").0, 503);
    assert_eq!(common::get(address, "/healthz").0, 200);
    sqlx::query("ALTER DATABASE postgres ALLOW_CONNECTIONS true")
        .execute(admin)
        .await
        .unwrap();
    common::assert_ready(address);
    let before = calls.load(Ordering::SeqCst);
    let (status, body) = post_answer(
        address.to_owned(),
        API_TOKEN,
        hero_query("db-recovered", "NonexistentHero987654", "docs.public"),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        serde_json::from_str::<Value>(&body).unwrap()["status"],
        "insufficient_evidence"
    );
    assert_eq!(calls.load(Ordering::SeqCst), before + 2);
}

async fn assert_in_flight_answer_drains(service: &mut Service, address: &str, calls: &AtomicUsize) {
    let before = calls.load(Ordering::SeqCst);
    let in_flight = tokio::spawn(post_answer(
        address.to_owned(),
        API_TOKEN,
        hero_query("drain", "DrainProbe", "docs.public"),
    ));
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while calls.load(Ordering::SeqCst) == before {
        assert!(
            std::time::Instant::now() < deadline,
            "drain request never reached the provider"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    service.signal("TERM");
    let (status, body) = in_flight.await.unwrap();
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        serde_json::from_str::<Value>(&body).unwrap()["request_id"],
        "drain"
    );
    let exit = tokio::task::block_in_place(|| service.wait(Duration::from_secs(15)));
    let log = service.log();
    assert!(exit.success(), "{log}");
    assert!(
        log.contains("shutdown_started") && log.contains("stopped"),
        "{log}"
    );
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
        .connect_with(options.clone())
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
    config["release"] = json!({"id":RELEASE_ID,"knowledge_version":"p6-process-knowledge"});
    let mut credentials = common::credentials();
    credentials.push(("BRAIN_PG_READONLY_PASSWORD", READONLY_PASSWORD));
    let admin = database_pool(&options, "template1").await;
    assert_incompatible_schemas_are_refused_untouched(&options, &admin, &config, &credentials)
        .await;
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let mut service = Service::spawn(&config, &credentials);
    let address = service.address();
    common::assert_ready(&address);
    let (_, body) = common::get(&address, "/readyz");
    assert_eq!(
        serde_json::from_str::<Value>(&body).unwrap()["serving_contract"],
        "brain.tools-only.v2"
    );
    let ready = serde_json::from_str::<Value>(&body).unwrap();
    for field in ["release_id", "knowledge_version", "release_bindings_sha256"] {
        assert!(ready[field].as_str().is_some_and(|value| !value.is_empty()));
    }
    assert_eq!(ready["release_id"], RELEASE_ID);
    let result = post_answer(
        address.clone(),
        API_TOKEN,
        hero_query("unknown-hero", "NonexistentHero987654", "docs.public"),
    )
    .await;
    assert_eq!(result.0, 200, "{}", result.1);
    let answer: Value = serde_json::from_str(&result.1).unwrap();
    assert_eq!(answer["status"], "insufficient_evidence");
    assert_eq!(answer["knowledge_release"], RELEASE_ID);
    assert!(answer["citations"].as_array().unwrap().is_empty());
    assert!(answer.get("claims").is_none());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(common::get(&service.address(), "/readyz").0, 200);
    assert_scope_and_token_denials(&address, &calls).await;
    assert_warm_schema_marker_is_enforced(&pool, &address).await;
    assert_database_outage_recovers(&admin, &address, &calls).await;
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
    assert_in_flight_answer_drains(&mut service, &address, &calls).await;
    assert_secret_free(&service.log(), &credentials);
    let _ = stop.send(());
    task.await.unwrap();
    pool.close().await;
    admin.close().await;
}
