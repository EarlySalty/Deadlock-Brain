use super::*;
use crate::deadlock_data::scratch_pg;
use brain_contracts::{Budget, Principal};
use brain_job::{ingest, registry::EXTRACTORS, GameFactsStore, InputObject};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[path = "pg_asset_slot_tests.rs"]
mod slot_tests;

struct NoLegacy;
impl ToolExecutionPort for NoLegacy {
    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(Vec::new())
    }
    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        _: &str,
        _: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        panic!("P2 must execute through the PostgreSQL executor")
    }
    fn validate_dependencies(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        assert!(dependencies.is_empty());
        Ok(())
    }
}
fn query_context() -> (Query, AuthorizedContext) {
    let query = serde_json::from_value(json!({"request_id":"p2-executor","conversation_id":"p2-executor","text":"Toxic Bullets","profile":"explain"})).expect("query");
    let context = AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "p2-executor".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 30_000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    }
    .with_request_deadline()
    .into_owned();
    (query, context)
}
fn call(name: ToolName, arguments: Value) -> (ToolCall, ToolRequest) {
    let call = ToolCall {
        id: "\"".repeat(512),
        name,
        arguments,
    };
    let request = call.validate(&definitions()).expect("typed P2 call");
    (call, request)
}
async fn database(options: &PgConnectOptions) -> (PgPool, PgPool) {
    let admin = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .expect("scratch DB");
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&admin).await.expect("scratch roles");
    brain_storage::PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .expect("migrations");
    let pool = tool_pool(
        options.clone().username("brain_readonly"),
        Duration::from_secs(2),
    );
    let mut connection = pool.acquire().await.expect("real tool connection");
    assert_session(&mut connection)
        .await
        .expect("readonly 500ms session");
    drop(connection);
    (admin, pool)
}
fn options(pg: &scratch_pg::ScratchPg) -> PgConnectOptions {
    PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
}

async fn assert_item_card_matches_view(admin: &PgPool, pool: &PgPool, id: i64) {
    let (call, request) = call(ToolName::GetItem, json!({"item":id}));
    let card = sqlx::query_scalar::<_, Value>(
        "SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1",
    )
    .bind(id)
    .fetch_one(admin)
    .await
    .expect("published reference card");
    let expected = crate::pg_asset_tools::bounded_card_result(&call.id, &request, card)
        .expect("reference bounds")
        .expect("published reference selection");
    let mut connection = pool.begin().await.expect("readonly contract transaction");
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *connection)
        .await
        .expect("same-snapshot card reads");
    let actual = crate::pg_asset_tools::read(&mut connection, &call.id, &request)
        .await
        .expect("bounded card fragments")
        .expect("published fragmented selection");
    assert_eq!(actual, expected.result);
    connection.commit().await.expect("readonly contract commit");
}

#[test]
fn p2_executor_preserves_qualifier_receipts_complete_grid_and_revalidation() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let (admin, pool) = runtime.block_on(database(&options(&pg)));
    let inputs = (0..4).map(|index| {
        let properties: serde_json::Map<String,Value> = (0..12).map(|stat| (format!("Stat{stat}"),json!({"value":stat+1}))).collect();
        Ok(InputObject {revision:"p2-fixture".into(),path:format!("items/en/{index}.json"),bytes:serde_json::to_vec(&json!({
            "id":index+1,"class_name":format!("upgrade_fixture_{index}"),"type":"upgrade","name":format!("Item {index}"),"cost":3200,"properties":properties
        })).expect("fixture bytes")})
    });
    let report = runtime
        .block_on(ingest(
            &GameFactsStore::new(admin.clone()),
            "assets",
            inputs,
            EXTRACTORS,
        ))
        .expect("real ingestion");
    assert_eq!(report.counts.accepted, 4);
    assert_eq!(report.counts.rejected, 0);
    let ids: Vec<i64> = runtime
        .block_on(
            sqlx::query_scalar("SELECT id FROM serve.current_entity ORDER BY id LIMIT 4")
                .fetch_all(&admin),
        )
        .expect("ids");
    let ability = br#"{"id":5,"class_name":"ability_fixture","type":"ability","name":"Dagger","properties":{"Damage":{"value":"1.2300e+2","postfix":"%","scale_function":{"class_name":"scale_function_tech_damage","stat_scale":0.25}}}}"#.to_vec();
    let accepted = runtime
        .block_on(ingest(
            &GameFactsStore::new(admin.clone()),
            "assets",
            [Ok(InputObject {
                revision: "p2-ability".into(),
                path: "items/en/4.json".into(),
                bytes: ability,
            })],
            EXTRACTORS,
        ))
        .expect("ability ingestion");
    assert_eq!(accepted.counts.accepted, 1);
    let ability_id: i64 = runtime
        .block_on(
            sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key='ability_fixture'")
                .fetch_one(&admin),
        )
        .expect("ability id");
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let plan: Value = runtime.block_on(sqlx::query_scalar("EXPLAIN (ANALYZE, FORMAT JSON) SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1").bind(ids[0]).fetch_one(&admin)).expect("own scratch plan");
    println!(
        "P2 item planning_ms={}, execution_ms={}",
        plan[0]["Planning Time"], plan[0]["Execution Time"]
    );
    let (query, context) = query_context();
    let stats: Vec<String> = (0..12).map(|stat| format!("item_stat{stat}")).collect();
    for (name, args) in [
        (ToolName::GetItem, json!({"item":ids[0]})),
        (ToolName::GetAbility, json!({"ability":ability_id})),
        (ToolName::Compare, json!({"entities":ids,"stats":stats})),
    ] {
        let (call, request) = call(name, args);
        let execution = tools
            .execute(&query, &context, None, &call.id, &request)
            .unwrap_or_else(|error| panic!("actual P2 {name:?}: {error}"));
        assert!(!execution.result.is_error);
        if name == ToolName::GetItem {
            runtime.block_on(assert_item_card_matches_view(&admin, &pool, ids[0]));
        }
        execution
            .validate_for(&call, &request, None)
            .expect("canonical dependency binding");
        assert!(
            serde_json::to_vec(&execution.result)
                .expect("ToolResult")
                .len()
                < MAX_BYTES
        );
        assert!(execution.result.evidence_ids.len() <= MAX_EVIDENCE);
        if name == ToolName::Compare {
            assert_eq!(
                execution.result.result["entities"].as_array().map(Vec::len),
                Some(4)
            );
            assert_eq!(
                execution.result.result["cells"].as_array().map(Vec::len),
                Some(48)
            );
            let facts = execution.result.result["stats"]["facts"]
                .as_array()
                .expect("comparison facts");
            for cell in execution.result.result["cells"]
                .as_array()
                .expect("comparison cells")
            {
                let shown = facts
                    .iter()
                    .filter(|fact| {
                        fact["entity_id"] == cell["entity_id"]
                            && fact["stat_key"] == cell["stat_key"]
                    })
                    .count() as u64;
                let total = cell["total_count"].as_u64().expect("cell total");
                let omitted = cell["omitted_count"].as_u64().expect("cell omission");
                assert_eq!(shown + omitted, total);
                assert_eq!(
                    cell["status"],
                    if total == 0 {
                        "missing"
                    } else if shown == 0 {
                        "omitted"
                    } else if omitted > 0 {
                        "partial"
                    } else {
                        "available"
                    }
                );
            }
        }
        for evidence in &execution.dependencies[0].evidence {
            assert_eq!(evidence.source_id, "canon.source/assets");
            let metadata = serde_json::to_value(
                brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(evidence)
                    .expect("complete shared receipt format"),
            )
            .expect("canonical metadata");
            assert_eq!(metadata["patch"], Value::Null);
            let bytes: Vec<u8> = runtime
                .block_on(
                    sqlx::query_scalar("SELECT bytes FROM raw.object WHERE id=$1")
                        .bind(metadata["raw_object_id"].as_i64().expect("object id"))
                        .fetch_one(&admin),
                )
                .expect("own scratch raw");
            let receipt = brain_storage::gamefacts::Receipt {
                locator_kind: metadata["locator_kind"].as_str().expect("kind").into(),
                locator: metadata["locator"].as_str().expect("locator").into(),
                value_sha256: hex::decode(metadata["value_sha256"].as_str().expect("hash"))
                    .expect("hex")
                    .try_into()
                    .expect("SHA256"),
            };
            let lexeme = brain_storage::gamefacts::receipts::value_bytes(&bytes, &receipt)
                .expect("original lexeme");
            assert_eq!(
                <[u8; 32]>::from(Sha256::digest(lexeme)),
                receipt.value_sha256
            );
        }
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
            ToolValidationPurpose::Cache,
        ] {
            tools
                .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
                .expect("real revalidation");
        }
        if name == ToolName::GetAbility {
            let fact = execution.result.result["facts"]["facts"]
                .as_array()
                .expect("facts")
                .iter()
                .find(|fact| fact["stat_key"] == "ability_damage")
                .expect("damage");
            assert_eq!(fact["value"].as_f64(), Some(123.0));
            assert_eq!(fact["unit"], "%");
            for qualifier in fact["qualifiers"].as_array().expect("qualifiers") {
                assert!(execution
                    .result
                    .evidence_ids
                    .iter()
                    .any(|id| qualifier["citation"] == *id));
            }
        }
    }
    for (name, args) in [
        (
            ToolName::GetItem,
            json!({"item":ids[0],"patch":"unpublished"}),
        ),
        (ToolName::GetAbility, json!({"hero":ids[0],"slot":1})),
        (
            ToolName::Compare,
            json!({"entities":[ids[0],i64::MAX],"stats":["item_cost"]}),
        ),
    ] {
        let (call, request) = call(name, args);
        assert!(
            tools
                .execute(&query, &context, None, &call.id, &request)
                .expect("fail closed")
                .result
                .is_error
        );
    }
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}

fn public_plan_metrics(plan: &Value) {
    if let Some(node) = plan.get("Node Type") {
        let elapsed = plan["Actual Total Time"].as_f64().unwrap_or_default();
        if elapsed >= 10.0 {
            println!(
                "public plan node={node}, relation={}, ms={elapsed}, loops={}, rows={}",
                plan["Relation Name"], plan["Actual Loops"], plan["Actual Rows"]
            );
        }
    }
    for child in plan["Plans"].as_array().into_iter().flatten() {
        public_plan_metrics(child);
    }
}

#[test]
#[ignore = "explicit public Assets ingestion and P2 executor probe in isolated PostgreSQL"]
fn public_assets_source_reaches_the_actual_item_executor() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let (admin, pool) = runtime.block_on(database(&options(&pg)));
    let cache = tempfile::tempdir().expect("own public data cache");
    let store = GameFactsStore::new(admin.clone());
    let report = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), true),
            EXTRACTORS,
        ))
        .expect("public Assets run");
    println!(
        "public Assets accepted={}, rejected={}, ignored={}",
        report.counts.accepted, report.counts.rejected, report.counts.ignored
    );
    let counts = "SELECT (SELECT count(*) FROM raw.object), (SELECT count(*) FROM canon.fact)";
    let before: (i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("before counts");
    for language in ["en", "de"] {
        let parts: Vec<Vec<u8>> = runtime.block_on(sqlx::query_scalar(
            "SELECT bytes FROM raw.object WHERE source='assets' AND path LIKE $1 ORDER BY length(split_part(path,'/',4)),split_part(path,'/',4)")
            .bind(format!("items/{language}/%/%.response")).fetch_all(&admin)).expect("archived public response");
        assert!(!parts.is_empty());
        let bytes: Vec<u8> = parts.into_iter().flatten().collect();
        std::fs::write(cache.path().join(format!("{language}.json")), bytes)
            .expect("own public snapshot");
    }
    let parts: Vec<Vec<u8>> = runtime.block_on(sqlx::query_scalar(
        "SELECT bytes FROM raw.object WHERE source='assets' AND path LIKE 'heroes/en/%/%.response' ORDER BY length(split_part(path,'/',4)),split_part(path,'/',4)")
        .fetch_all(&admin)).expect("archived public heroes response");
    assert!(!parts.is_empty());
    std::fs::write(
        cache.path().join("heroes-en.json"),
        parts.into_iter().flatten().collect::<Vec<_>>(),
    )
    .expect("own public heroes snapshot");
    let repeated = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("unchanged actual public snapshot");
    assert_eq!(repeated.counts.accepted, 0);
    assert_eq!(repeated.counts.rejected, report.counts.rejected);
    let after: (i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("after counts");
    assert_eq!(after, before);
    println!(
        "unchanged public snapshot added_raw={}, added_facts={}",
        after.0 - before.0,
        after.1 - before.1
    );
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let (query, context) = query_context();
    let id: i64 = runtime
        .block_on(
            sqlx::query_scalar(
                "SELECT id FROM serve.current_entity WHERE key='upgrade_toxic_bullets'",
            )
            .fetch_one(&admin),
        )
        .expect("real Toxic Bullets");
    let (call, request) = call(ToolName::GetItem, json!({"item":id}));
    let executed = tools.execute(&query, &context, None, &call.id, &request);
    if executed.is_err() {
        let plan: Value = runtime.block_on(sqlx::query_scalar(
            "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1")
            .bind(id).fetch_one(&admin)).expect("failed public item plan");
        println!(
            "public item planning_ms={}, execution_ms={}, jit_ms={}",
            plan[0]["Planning Time"], plan[0]["Execution Time"], plan[0]["JIT"]["Timing"]["Total"]
        );
        public_plan_metrics(&plan[0]["Plan"]);
        let retried = runtime.block_on(
            sqlx::query_scalar::<_, Value>(
                "SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1",
            )
            .bind(id)
            .fetch_one(&pool),
        );
        match retried {
            Ok(_) => println!("post-failure readonly query succeeded"),
            Err(error) => println!(
                "post-failure readonly SQLSTATE={:?}",
                error
                    .as_database_error()
                    .and_then(|error| error.code())
                    .as_deref()
            ),
        }
    }
    let execution = executed.expect("real item executor");
    assert!(
        !execution.result.is_error,
        "public item error={}",
        execution.result.result["error"]
    );
    runtime.block_on(assert_item_card_matches_view(&admin, &pool, id));
    let facts = execution.result.result["facts"]["facts"]
        .as_array()
        .expect("facts");
    assert!(facts
        .iter()
        .any(|fact| fact["stat_key"] == "item_cost" && fact["value"].as_f64().is_some()));
    assert!(facts
        .iter()
        .any(|fact| fact["stat_key"] == "item_dothealthpercent"));
    assert!(facts
        .iter()
        .any(|fact| fact["stat_key"] == "item_healampreceivepenaltypercent"));
    println!(
        "real Toxic Bullets ToolResult bytes={}, receipt_count={}",
        serde_json::to_vec(&execution.result)
            .expect("result bytes")
            .len(),
        execution.result.evidence_ids.len()
    );
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
