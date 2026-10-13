use brain_contracts::{
    tools::canonical_patch_history_definition, AuthorizedContext, PortError, Principal, Query,
    ToolCall, ToolExecution, ToolName,
};
use brain_serve::pg_patch_tools::{execute_patch_history, validate_patch_dependency};
use brain_storage::{
    gamefacts::{
        receipts, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject,
        Receipt,
    },
    PgStore,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::collections::BTreeSet;

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

struct Database {
    admin: PgPool,
    reader: PgPool,
    writer: PgPool,
    store: GameFactsStore,
    log: std::path::PathBuf,
}

impl Database {
    async fn start(pg: &scratch_pg::ScratchPg) -> Self {
        let options = PgConnectOptions::new_without_pgpass()
            .host(&pg.directory.join("socket").to_string_lossy())
            .port(55439)
            .username("brain_core_test")
            .database("postgres")
            .password("")
            .ssl_mode(PgSslMode::Disable);
        let admin = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.clone())
            .await
            .unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
            .execute(&admin).await.unwrap();
        let schema = PgStore::new(admin.clone());
        schema.migrate_gamefacts().await.unwrap();
        schema.check_gamefacts_schema().await.unwrap();
        schema.migrate_gamefacts().await.unwrap();
        schema.check_gamefacts_schema().await.unwrap();
        let versions: Vec<i32> =
            sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
                .fetch_all(&admin)
                .await
                .unwrap();
        assert_eq!(versions, [1, 4]);
        let reader = PgPoolOptions::new()
            .max_connections(2)
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET log_statement='all'")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::query("SET ROLE brain_tool")
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect_with(options.clone())
            .await
            .unwrap();
        let writer = PgPoolOptions::new()
            .max_connections(2)
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET ROLE brain_ingest")
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect_with(options)
            .await
            .unwrap();
        Self {
            admin,
            reader,
            store: GameFactsStore::new(writer.clone()),
            log: pg.directory.join("postgres.log"),
            writer,
        }
    }

    async fn publish(
        &self,
        source: &str,
        path: &str,
        revision: &str,
        mut value: Value,
        hero: bool,
    ) -> RawObject {
        let changes = value.as_object_mut().unwrap().remove("changes");
        let run = self.store.begin_run(source).await.unwrap();
        let raw = self
            .store
            .capture(
                run,
                source,
                revision,
                path,
                serde_json::to_vec(&value).unwrap(),
            )
            .await
            .unwrap();
        let stored: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE id=$1")
            .bind(raw.id)
            .fetch_one(&self.admin)
            .await
            .unwrap();
        assert_eq!(stored, serde_json::to_vec(&value).unwrap());
        let set = if hero {
            hero_facts(&raw, &value)
        } else {
            patch_facts(&raw, &value, changes.as_ref().unwrap())
        };
        self.store
            .apply(run, &raw, if hero { "hero" } else { "patchnotes" }, &set)
            .await
            .unwrap();
        self.store
            .finish_run(
                run,
                true,
                Counts {
                    accepted: 1,
                    rejected: 0,
                    ignored: 0,
                },
            )
            .await
            .unwrap();
        let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id=$1")
            .bind(run)
            .fetch_one(&self.admin)
            .await
            .unwrap();
        assert_eq!(status, "succeeded");
        raw
    }

    async fn hero_id(&self, key: &str) -> i64 {
        sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
            .bind(key)
            .fetch_one(&self.reader)
            .await
            .unwrap()
    }

    async fn execute(&self, arguments: Value) -> ToolExecution {
        let id = "\"".repeat(512);
        let request = ToolCall {
            id: id.clone(),
            name: ToolName::PatchHistory,
            arguments,
        }
        .validate(&[canonical_patch_history_definition()])
        .unwrap();
        let (query, context) = query_context();
        match execute_patch_history(&self.reader, &query, &context, &id, &request).await {
            Ok(execution) => execution,
            Err(error) => {
                let log = std::fs::read_to_string(&self.log).unwrap();
                let errors = log
                    .lines()
                    .filter(|line| line.contains("ERROR:"))
                    .collect::<Vec<_>>();
                panic!(
                    "Request {:?}: {error:?}, scratch SQL errors: {errors:?}",
                    request.arguments()
                );
            }
        }
    }

    async fn quarantine(&self, definition: &str, hidden: &[i64]) {
        let ids = hidden
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let filter = if hidden.is_empty() {
            "true".into()
        } else {
            format!("p.id NOT IN ({ids})")
        };
        let mut tx = self.admin.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE brain_migrate")
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::raw_sql(&format!("CREATE OR REPLACE VIEW serve.published_receipt AS SELECT p.* FROM ({}) p WHERE {filter}", definition.trim_end_matches(';')))
            .execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
    }

    async fn close(&self) {
        self.reader.close().await;
        self.writer.close().await;
        self.admin.close().await;
    }
}

fn collect_plan_indexes(value: &Value, indexes: &mut BTreeSet<String>) {
    match value {
        Value::Object(fields) => {
            if let Some(Value::String(index)) = fields.get("Index Name") {
                indexes.insert(index.clone());
            }
            for child in fields.values() {
                collect_plan_indexes(child, indexes);
            }
        }
        Value::Array(children) => {
            for child in children {
                collect_plan_indexes(child, indexes);
            }
        }
        _ => {}
    }
}

fn query_context() -> (Query, AuthorizedContext) {
    (
        Query {
            request_id: "p4-test".into(),
            conversation_id: "p4-test".into(),
            text: "Patchänderungen".into(),
            answer_context: None,
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: Default::default(),
            patch: None,
            mode: None,
        },
        AuthorizedContext {
            principal: Principal {
                actor_id: "test".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            discord: None,
            conversation_id: "p4-test".into(),
            knowledge_release: "test".into(),
            deadline_ms: 1000,
            budget: Default::default(),
            request_deadline: None,
        },
    )
}

fn receipt(raw: &RawObject, pointer: Option<&str>) -> Receipt {
    let mut receipt = Receipt {
        locator_kind: if pointer.is_some() {
            "json_pointer"
        } else {
            "raw_bytes"
        }
        .into(),
        locator: pointer.unwrap_or_default().into(),
        value_sha256: [0; 32],
    };
    receipt.value_sha256 =
        Sha256::digest(receipts::value_bytes(&raw.bytes, &receipt).unwrap()).into();
    receipt
}

fn fact(key: &str, stat: &str, value: FactValue, receipt: Receipt) -> Fact {
    Fact {
        entity_key: key.into(),
        stat_key: stat.into(),
        value,
        receipt,
    }
}

fn hero_facts(raw: &RawObject, value: &Value) -> FactSet {
    let key = value["key"].as_str().unwrap();
    FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: receipt(raw, None),
        }],
        aliases: value["aliases"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, alias)| Alias {
                entity_key: key.into(),
                alias: alias.as_str().unwrap().into(),
                lang: "en".into(),
                receipt: receipt(raw, Some(&format!("/aliases/{index}"))),
            })
            .collect(),
        facts: value
            .get("health")
            .map(|health| {
                fact(
                    key,
                    "health",
                    FactValue::Number(health.as_f64().unwrap()),
                    receipt(raw, Some("/health")),
                )
            })
            .into_iter()
            .collect(),
        relations: Vec::new(),
    }
}

fn patch_facts(raw: &RawObject, value: &Value, changes: &Value) -> FactSet {
    let key = format!("patch_{}", value["id"].as_i64().unwrap());
    let mut set = FactSet {
        entities: vec![Entity {
            kind: "patch".into(),
            key: key.clone(),
            receipt: receipt(raw, None),
        }],
        ..FactSet::default()
    };
    for (stat, field) in [
        ("label", "title"),
        ("released_at", "posted_at"),
        ("official_url", "url"),
    ] {
        set.facts.push(fact(
            &key,
            stat,
            FactValue::Text(value[field].as_str().unwrap().into()),
            receipt(raw, Some(&format!("/{field}"))),
        ));
    }
    for change in changes.as_array().unwrap() {
        set.facts.push(fact(
            &key,
            &format!(
                "patch_change_r{}_{}",
                hex::encode(Sha256::digest(
                    change["reference"].as_str().unwrap().as_bytes()
                )),
                change["statement_index"].as_u64().unwrap()
            ),
            FactValue::Text(change.to_string()),
            receipt(raw, Some("/raw_content")),
        ));
    }
    set
}

fn change(index: u64, reference: &str, stat: &str, statement: &str) -> Value {
    let explicit = statement
        .split_once("from ")
        .and_then(|(_, tail)| tail.split_once(" to "));
    let old = explicit.map(|(old, _)| old);
    let new = explicit
        .and_then(|(_, new)| new.split_whitespace().next())
        .map(|new| new.trim_end_matches('.'));
    json!({"reference":reference,"subject":reference,"statement_index":index,"context_index":null,
        "stat_label":stat,"old":old,"new":new,"line_text":statement})
}

fn patch(id: i64, label: &str, date: &str, changes: Vec<Value>) -> Value {
    json!({"id":id,"title":label,"posted_at":date,
        "url":format!("https://store.steampowered.com/news/app/1422450/view/{id}"),
        "raw_content":changes.iter().map(|change| change["line_text"].as_str().unwrap()).collect::<Vec<_>>().join("\n"),
        "changes":changes})
}

fn rows(execution: &ToolExecution) -> &[Value] {
    assert!(!execution.result.is_error, "{:?}", execution.result.result);
    execution.result.result["changes"].as_array().unwrap()
}

fn error(execution: &ToolExecution, code: &str) {
    assert!(execution.result.is_error);
    assert_eq!(execution.result.result["error"], code);
    assert!(execution.dependencies.is_empty());
    assert!(execution.result.evidence_ids.is_empty());
}

fn verify_receipts(execution: &ToolExecution, objects: &[RawObject]) {
    let mut citations = BTreeSet::new();
    for patch in [
        &execution.result.result["current_official_patch"],
        &execution.result.result["range"]["since"],
        &execution.result.result["range"]["until"],
    ] {
        if !patch.is_null() {
            for field in [
                "head_citation",
                "label_citation",
                "released_at_citation",
                "url_citation",
            ] {
                citations.insert(patch[field].as_str().unwrap());
            }
        }
    }
    for row in rows(execution) {
        for field in [
            "citation",
            "head_citation",
            "label_citation",
            "released_at_citation",
            "url_citation",
            "binding_citation",
            "entity_citation",
            "stat_citation",
        ] {
            if !row[field].is_null() {
                citations.insert(row[field].as_str().unwrap());
            }
        }
    }
    assert_eq!(
        citations,
        execution
            .result
            .evidence_ids
            .iter()
            .map(String::as_str)
            .collect()
    );
    assert_eq!(execution.dependencies.len(), 1);
    assert_eq!(execution.dependencies[0].evidence.len(), citations.len());
    for evidence in &execution.dependencies[0].evidence {
        let content: Value = serde_json::from_str(&evidence.content).unwrap();
        let raw = objects
            .iter()
            .find(|raw| content["raw_object_id"] == raw.id)
            .unwrap();
        assert_eq!(content["source"], raw.source);
        assert_eq!(content["source_revision"], raw.revision);
        assert_eq!(content["path"], raw.path);
        assert_eq!(content["content_sha256"], hex::encode(raw.content_sha256));
        assert_eq!(content["citation"], evidence.citation);
        assert_eq!(content["ingest_run_id"].as_u64(), Some(evidence.revision));
        let receipt = Receipt {
            locator_kind: content["locator_kind"].as_str().unwrap().into(),
            locator: content["locator"].as_str().unwrap().into(),
            value_sha256: [0; 32],
        };
        assert_eq!(
            content["value_sha256"],
            hex::encode(Sha256::digest(
                receipts::value_bytes(&raw.bytes, &receipt).unwrap()
            ))
        );
        assert_eq!(content["patch"], Value::Null);
        assert!(evidence.patch.is_none());
    }
    assert!(serde_json::to_vec(execution).unwrap().len() < 16_384);
}

#[tokio::test]
async fn real_current_projection_and_composed_patch_history() {
    let pg = scratch_pg::ScratchPg::start();
    let db = Database::start(&pg).await;
    let empty = db.execute(json!({"entity":1})).await;
    error(&empty, "official_patch_unavailable");
    let baba = db
        .publish(
            "heroes",
            "baba",
            "hero-revision-α",
            json!({"key":"hero_baba","aliases":["Baba","shared"],"health":500}),
            true,
        )
        .await;
    let infernus = db
        .publish(
            "heroes",
            "infernus",
            "hero-revision-β",
            json!({"key":"hero_infernus","aliases":["Infernus","shared"],"health":600}),
            true,
        )
        .await;
    let id = db.hero_id("hero_baba").await;
    let line = "Baba: Health increased from 123 to 150.\n  when nearby, äöüß stay intact";
    let first = db
        .publish(
            "deadlock_patchnotes_db",
            "10",
            "opaque-r1-not-a-patch",
            patch(
                10,
                "p1",
                "2026-10-01T12:00:00Z",
                vec![
                    change(1, "baba", "Health", line),
                    change(
                        2,
                        "baba",
                        "health",
                        "Baba: health increased from 123 to 150",
                    ),
                    change(
                        3,
                        "unknown",
                        "health",
                        "Unknown: health increased from 123 to 150",
                    ),
                    change(
                        4,
                        "shared",
                        "health",
                        "Shared: health increased from 123 to 150",
                    ),
                    change(5, "baba", "health", "Baba: health changed from 150 to 160"),
                ],
            ),
            false,
        )
        .await;
    let modified =
        sqlx::query("UPDATE canon.fact SET stat_key=$1 WHERE receipt_id IN (SELECT id FROM canon.receipt WHERE raw_object_id=$2) AND stat_key=$3")
            .bind(format!(
                "patch_change_r{}_5",
                hex::encode(Sha256::digest(b"infernus"))
            ))
            .bind(first.id)
            .bind(format!(
                "patch_change_r{}_5",
                hex::encode(Sha256::digest(b"baba"))
            ))
            .execute(&db.admin)
            .await
            .unwrap();
    assert_eq!(modified.rows_affected(), 1);
    let infernus_id = db.hero_id("hero_infernus").await;
    error(
        &db.execute(json!({"entity":infernus_id})).await,
        "no_patch_changes",
    );
    let execution = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&execution).len(), 2);
    assert_eq!(rows(&execution)[0]["stat_key"], "health");
    assert_eq!(rows(&execution)[0]["old"], "123");
    assert_eq!(rows(&execution)[1]["stat_key"], Value::Null);
    assert_eq!(rows(&execution)[1]["line_text"], line);
    verify_receipts(&execution, &[baba.clone(), infernus.clone(), first.clone()]);
    let changes_sql = include_str!("../src/pg_patch_tools.rs")
        .split_once("const CHANGES: &str = \"")
        .unwrap()
        .1
        .split_once("\";")
        .unwrap()
        .0;
    let plan: Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {changes_sql}"
    ))
    .bind(id)
    .bind(None::<i64>)
    .bind(Some("2026-10-01T12:00:00Z"))
    .bind(Some("2026-10-01T12:00:00Z"))
    .fetch_one(&db.reader)
    .await
    .unwrap();
    assert_eq!(plan[0]["Plan"]["Actual Rows"], 2);
    let mut indexes = BTreeSet::new();
    collect_plan_indexes(&plan, &mut indexes);
    println!(
        "P4_INDEX_DIAGNOSTIC real CHANGES query, 2 fixture rows, execution_ms={}, planning_ms={}, indexes={indexes:?}; dynamic binding.entity_id has no stored (entity_id,patch_id) index",
        plan[0]["Execution Time"], plan[0]["Planning Time"]
    );
    let published_definition: String =
        sqlx::query_scalar("SELECT pg_get_viewdef('serve.published_receipt'::regclass, true)")
            .fetch_one(&db.admin)
            .await
            .unwrap();
    let first_rows = rows(&execution);
    let lexical_id = first_rows[0]["citation"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    db.quarantine(&published_definition, &[lexical_id]).await;
    let narrowed = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&narrowed).len(), 1);
    assert_eq!(rows(&narrowed)[0]["line_text"], line);
    assert!(!narrowed
        .result
        .evidence_ids
        .contains(&lexical_id.to_string()));
    for field in [
        "binding_citation",
        "entity_citation",
        "head_citation",
        "label_citation",
        "released_at_citation",
        "url_citation",
    ] {
        let hidden = first_rows[0][field]
            .as_str()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        db.quarantine(&published_definition, &[hidden]).await;
        let count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM serve.p4_patch_receipt WHERE receipt_id=$1")
                .bind(hidden)
                .fetch_one(&db.reader)
                .await
                .unwrap();
        assert_eq!(count, 0);
        error(
            &db.execute(json!({"entity":id})).await,
            if matches!(field, "binding_citation" | "entity_citation") {
                "no_patch_changes"
            } else {
                "official_patch_unavailable"
            },
        );
    }
    let hidden_stat = first_rows[0]["stat_citation"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    db.quarantine(&published_definition, &[hidden_stat]).await;
    let no_stat = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&no_stat).len(), 2);
    assert!(rows(&no_stat)
        .iter()
        .all(|row| row["stat_key"].is_null() && row["stat_citation"].is_null()));
    assert!(!no_stat
        .result
        .evidence_ids
        .contains(&hidden_stat.to_string()));
    db.quarantine(&published_definition, &[]).await;
    assert_eq!(db.execute(json!({"entity":id})).await, execution);
    let (query, context) = query_context();
    validate_patch_dependency(&db.reader, &query, &context, &execution.dependencies[0])
        .await
        .unwrap();
    let request = &execution.dependencies[0].request;
    assert!(matches!(
        execute_patch_history(&db.admin, &query, &context, "patch-call", request).await,
        Err(PortError::PermissionDenied(_))
    ));
    assert!(sqlx::query("SELECT bytes FROM raw.object")
        .fetch_all(&db.reader)
        .await
        .is_err());
    assert!(sqlx::query("SELECT * FROM canon.fact")
        .fetch_all(&db.reader)
        .await
        .is_err());
    assert!(sqlx::query("DELETE FROM serve.p4_patch_catalog")
        .execute(&db.reader)
        .await
        .is_err());
    for view in ["p4_patch_catalog", "p4_patch_change", "p4_patch_receipt"] {
        let definition: String =
            sqlx::query_scalar("SELECT pg_get_viewdef(('serve.' || $1)::regclass, true)")
                .bind(view)
                .fetch_one(&db.admin)
                .await
                .unwrap();
        assert!(!definition.contains("canon.") && !definition.contains("raw."));
        assert!(definition.contains("published_receipt"));
    }
    let second = db
        .publish(
            "deadlock_patchnotes_db",
            "20",
            "opaque-r2",
            patch(
                20,
                "p2",
                "2026-10-02T12:00:00Z",
                vec![change(
                    1,
                    "infernus",
                    "health",
                    "Infernus: health increased from 123 to 150",
                )],
            ),
            false,
        )
        .await;
    error(&db.execute(json!({"entity":id})).await, "no_patch_changes");
    let bounded = db
        .execute(json!({"entity":id,"since":"p1","until":"p2"}))
        .await;
    assert_eq!(rows(&bounded).len(), 2);
    assert_eq!(
        bounded.result.result["current_official_patch"]["label"],
        "p2"
    );
    verify_receipts(
        &bounded,
        &[
            baba.clone(),
            infernus.clone(),
            first.clone(),
            second.clone(),
        ],
    );
    let same = db.execute(json!({"entity":id,"since":"2026-10-01T12:00:00Z","until":"2026-10-01T12:00:00Z","limit":1})).await;
    assert_eq!(rows(&same).len(), 1);
    assert_eq!(same.result.result["omitted_count"], 1);
    assert_eq!(same.result.result["truncated"], true);
    error(
        &db.execute(json!({"entity":id,"since":"p2","until":"p1"}))
            .await,
        "patch_range_reversed",
    );
    for label in ["missing", "p1' OR true --", "2026-99-99T12:00:00Z"] {
        error(
            &db.execute(json!({"entity":id,"since":label})).await,
            "patch_not_found",
        );
    }
    let revised = db
        .publish(
            "deadlock_patchnotes_db",
            "10",
            "opaque-revision-changed",
            patch(
                10,
                "p1-revised",
                "2026-10-01T12:00:00Z",
                vec![change(
                    1,
                    "baba",
                    "health",
                    "Baba: health changed from 123 to 175",
                )],
            ),
            false,
        )
        .await;
    let current_revision = db
        .execute(json!({"entity":id,"since":"p1-revised","until":"p1-revised"}))
        .await;
    assert_eq!(rows(&current_revision).len(), 1);
    assert_eq!(
        rows(&current_revision)[0]["source_revision"],
        "opaque-revision-changed"
    );
    verify_receipts(
        &current_revision,
        &[baba.clone(), revised.clone(), second.clone()],
    );
    assert!(
        validate_patch_dependency(&db.reader, &query, &context, &bounded.dependencies[0])
            .await
            .is_err()
    );
    error(
        &db.execute(json!({"entity":id,"since":"p1"})).await,
        "patch_not_found",
    );
    db.publish(
        "heroes",
        "baba",
        "hero-revision-renamed",
        json!({"key":"hero_baba","aliases":["Baba renamed","shared"],"health":500}),
        true,
    )
    .await;
    error(
        &db.execute(json!({"entity":id,"since":"p1-revised"})).await,
        "no_patch_changes",
    );
    let without_health = db
        .publish(
            "heroes",
            "baba",
            "hero-health-removed",
            json!({"key":"hero_baba","aliases":["Baba"]}),
            true,
        )
        .await;
    let absent_stat = db.execute(json!({"entity":id,"since":"p1-revised"})).await;
    assert_eq!(rows(&absent_stat).len(), 1);
    assert_eq!(rows(&absent_stat)[0]["stat_key"], Value::Null);
    assert_eq!(rows(&absent_stat)[0]["stat_citation"], Value::Null);
    verify_receipts(
        &absent_stat,
        &[without_health, revised.clone(), second.clone()],
    );
    let renamed = db
        .publish(
            "heroes",
            "baba",
            "hero-revision-restored",
            json!({"key":"hero_baba","aliases":["Baba"],"health":501}),
            true,
        )
        .await;
    let restored = db.execute(json!({"entity":id,"since":"p1-revised"})).await;
    assert_eq!(rows(&restored).len(), 1);
    verify_receipts(
        &restored,
        &[renamed.clone(), revised.clone(), second.clone()],
    );
    let ambiguous = db
        .publish(
            "deadlock_patchnotes_db",
            "30",
            "label-duplicate",
            patch(
                30,
                "p2",
                "2026-10-02T12:00:00Z",
                vec![change(
                    1,
                    "baba",
                    "health",
                    "Baba: health changed from 123 to 150",
                )],
            ),
            false,
        )
        .await;
    for bound in ["since", "until"] {
        for label in ["p2", "2026-10-02T12:00:00Z"] {
            let mut arguments = json!({"entity":id});
            arguments[bound] = json!(label);
            error(&db.execute(arguments).await, "patch_ambiguous");
        }
    }
    let long = "Baba: health changed, vollständige Zeile äöüß ".repeat(28);
    let budget = db
        .publish(
            "deadlock_patchnotes_db",
            "40",
            "large-response",
            patch(
                40,
                "large",
                "2026-10-04T12:00:00Z",
                (1..=20)
                    .map(|index| change(index, "baba", "health", &long))
                    .collect(),
            ),
            false,
        )
        .await;
    let limited = db.execute(json!({"entity":id})).await;
    assert!(!rows(&limited).is_empty() && rows(&limited).len() < 20);
    assert_eq!(limited.result.result["truncated"], true);
    assert_eq!(
        limited.result.result["omitted_count"].as_u64().unwrap() as usize,
        20 - rows(&limited).len()
    );
    assert!(rows(&limited).iter().all(|row| row["line_text"] == long));
    verify_receipts(&limited, &[renamed, revised, second, ambiguous, budget]);
    let failed_run = db.store.begin_run("deadlock_patchnotes_db").await.unwrap();
    let mut hidden_value = patch(
        40,
        "hidden",
        "2026-10-05T12:00:00Z",
        vec![change(
            1,
            "baba",
            "health",
            "Baba: health changed from 1 to 2",
        )],
    );
    let hidden_changes = hidden_value
        .as_object_mut()
        .unwrap()
        .remove("changes")
        .unwrap();
    let hidden = db
        .store
        .capture(
            failed_run,
            "deadlock_patchnotes_db",
            "failed-revision",
            "40",
            serde_json::to_vec(&hidden_value).unwrap(),
        )
        .await
        .unwrap();
    db.store
        .apply(
            failed_run,
            &hidden,
            "patchnotes",
            &patch_facts(&hidden, &hidden_value, &hidden_changes),
        )
        .await
        .unwrap();
    db.store
        .finish_run(
            failed_run,
            false,
            Counts {
                accepted: 1,
                rejected: 0,
                ignored: 0,
            },
        )
        .await
        .unwrap();
    let still_current = db.execute(json!({"entity":id})).await;
    assert_eq!(
        still_current.result.result["current_official_patch"]["label"],
        "large"
    );
    assert!(rows(&still_current)
        .iter()
        .all(|row| row["source_revision"] == "large-response"));
    let catalog_definition: String =
        sqlx::query_scalar("SELECT pg_get_viewdef('serve.p4_patch_catalog'::regclass, true)")
            .fetch_one(&db.admin)
            .await
            .unwrap();
    let mut guard = db.admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *guard)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("CREATE OR REPLACE VIEW serve.p4_patch_catalog AS SELECT p.* FROM ({}) p WHERE current_user='brain_tool' AND current_setting('transaction_read_only')='on' AND current_setting('statement_timeout')='500ms' AND current_setting('transaction_isolation')='repeatable read'", catalog_definition.trim_end_matches(';')))
        .execute(&mut *guard).await.unwrap();
    guard.commit().await.unwrap();
    assert_eq!(db.execute(json!({"entity":id})).await, still_current);
    let mut slow = db.admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *slow)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("CREATE OR REPLACE VIEW serve.p4_patch_catalog AS SELECT p.* FROM ({}) p CROSS JOIN (SELECT pg_sleep(1)) wait", catalog_definition.trim_end_matches(';')))
        .execute(&mut *slow).await.unwrap();
    slow.commit().await.unwrap();
    assert!(matches!(
        execute_patch_history(&db.reader, &query, &context, "timeout-proof", request).await,
        Err(PortError::Unavailable(_))
    ));
    let mut restore = db.admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *restore)
        .await
        .unwrap();
    sqlx::raw_sql(&format!(
        "CREATE OR REPLACE VIEW serve.p4_patch_catalog AS {}",
        catalog_definition.trim_end_matches(';')
    ))
    .execute(&mut *restore)
    .await
    .unwrap();
    restore.commit().await.unwrap();
    assert_eq!(db.execute(json!({"entity":id})).await, still_current);
    let log = std::fs::read_to_string(&db.log).unwrap();
    assert!(log.contains("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY"));
    assert!(log.contains("SET LOCAL statement_timeout = '500ms'"));
    println!("P4_DB_PROOF frozen-v1 migration, additive brain_migrate views, brain_tool/read-only/500ms execution, unique aliases, superseded raw/alias/stat suppression, inclusive bounds, exact complete receipts, response budget <16384, failed-run isolation, quarantine propagation, serve-only reads verified");
    db.close().await;
}

#[tokio::test]
async fn incoherent_heads_and_removed_statements_are_hidden() {
    let pg = scratch_pg::ScratchPg::start();
    let db = Database::start(&pg).await;
    db.publish(
        "heroes",
        "baba",
        "hero",
        json!({"key":"hero_baba","aliases":["Baba"],"health":500}),
        true,
    )
    .await;
    let id = db.hero_id("hero_baba").await;
    for mismatch in ["extractor", "run", "raw"] {
        let source = "deadlock_patchnotes_db";
        let mut value = patch(
            10,
            mismatch,
            "2026-10-01T12:00:00Z",
            vec![change(
                1,
                "baba",
                "health",
                "Baba: health increased from 123 to 150",
            )],
        );
        let changes = value.as_object_mut().unwrap().remove("changes").unwrap();
        let head_run = db.store.begin_run(source).await.unwrap();
        let head_raw = db
            .store
            .capture(
                head_run,
                source,
                mismatch,
                "10",
                serde_json::to_vec(&value).unwrap(),
            )
            .await
            .unwrap();
        let mut head = patch_facts(&head_raw, &value, &changes);
        head.facts.clear();
        db.store
            .apply(head_run, &head_raw, "patchnotes", &head)
            .await
            .unwrap();
        let facts_run = if mismatch == "run" {
            db.store
                .finish_run(
                    head_run,
                    true,
                    Counts {
                        accepted: 1,
                        rejected: 0,
                        ignored: 0,
                    },
                )
                .await
                .unwrap();
            db.store.begin_run(source).await.unwrap()
        } else {
            head_run
        };
        let facts_raw = if mismatch == "raw" {
            value["title"] = json!("different raw identity");
            db.store
                .capture(
                    facts_run,
                    source,
                    "other-raw",
                    "10",
                    serde_json::to_vec(&value).unwrap(),
                )
                .await
                .unwrap()
        } else {
            head_raw
        };
        let mut facts = patch_facts(&facts_raw, &value, &changes);
        facts.entities.clear();
        db.store
            .apply(
                facts_run,
                &facts_raw,
                if mismatch == "extractor" {
                    "different_extractor"
                } else {
                    "patchnotes"
                },
                &facts,
            )
            .await
            .unwrap();
        db.store
            .finish_run(
                facts_run,
                true,
                Counts {
                    accepted: if facts_run == head_run { 2 } else { 1 },
                    rejected: 0,
                    ignored: 0,
                },
            )
            .await
            .unwrap();
        let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id=$1")
            .bind(facts_run)
            .fetch_one(&db.admin)
            .await
            .unwrap();
        assert_eq!(status, "succeeded");
        error(
            &db.execute(json!({"entity":id})).await,
            "official_patch_unavailable",
        );
    }
    db.publish(
        "deadlock_patchnotes_db",
        "10",
        "complete",
        patch(
            10,
            "complete",
            "2026-10-01T12:00:00Z",
            vec![change(
                1,
                "baba",
                "health",
                "Baba: health increased from 123 to 150",
            )],
        ),
        false,
    )
    .await;
    assert_eq!(rows(&db.execute(json!({"entity":id})).await).len(), 1);
    db.publish(
        "deadlock_patchnotes_db",
        "10",
        "removed",
        patch(10, "removed", "2026-10-01T12:00:00Z", vec![]),
        false,
    )
    .await;
    error(&db.execute(json!({"entity":id})).await, "no_patch_changes");
    let changes: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.p4_patch_change")
        .fetch_one(&db.reader)
        .await
        .unwrap();
    assert_eq!(changes, 0);
    println!("P4_HEAD_PROOF mismatched raw/run/extractor hidden, complete statement removal hidden through real GameFactsStore and brain_tool queries");
    db.close().await;
}

#[tokio::test]
async fn escaped_unicode_reference_keeps_exact_alias_and_receipt_binding() {
    let pg = scratch_pg::ScratchPg::start();
    let db = Database::start(&pg).await;
    let reference = "held \"ä\" \\";
    let hero = db
        .publish(
            "heroes",
            "escaped",
            "hero-escaped",
            json!({"key":"hero_escaped","aliases":["Held \"Ä\" \\"],"health":500}),
            true,
        )
        .await;
    let id = db.hero_id("hero_escaped").await;
    let patch = db
        .publish(
            "deadlock_patchnotes_db",
            "10",
            "patch-escaped",
            patch(
                10,
                "escaped",
                "2026-10-01T12:00:00Z",
                vec![change(
                    1,
                    reference,
                    "health",
                    "Held: health changed from 400 to 500",
                )],
            ),
            false,
        )
        .await;
    let execution = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&execution).len(), 1);
    assert_eq!(rows(&execution)[0]["stat_key"], "health");
    assert_eq!(rows(&execution)[0]["subject"], reference);
    verify_receipts(&execution, &[hero, patch]);
    db.close().await;
}
