use brain_contracts::{
    canonical_receipt::CanonicalReceipt, tools::canonical_patch_history_definition,
    AuthorizedContext, PinnedGameContext, PortError, Principal, Query, ToolCall, ToolExecution,
    ToolExecutionPort, ToolName, ToolRequest, ToolValidationPurpose,
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
use std::collections::{BTreeMap, BTreeSet};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

#[path = "../src/pg_patch_tools.rs"]
mod p4_unit_contracts;

const PRE_FACTOR_CHANGES: &str = "SELECT jsonb_build_object('id',c.change_id,'entity_id',c.entity_id,
    'stat_key',c.stat_key,'old',c.old_value,'new',c.new_value,'change_kind',c.change_kind,
    'line_text',c.line_text,'subject',c.subject,'statement_index',c.statement_index,
    'context_index',c.context_index,'stat_label',c.stat_label,'patch',p.label,'released_at',p.released_at,
    'citation',c.receipt_id::text,'head_citation',p.official_receipt_id::text,
    'label_citation',p.label_receipt_id::text,'released_at_citation',p.released_at_receipt_id::text,
    'url_citation',p.url_receipt_id::text,
    'binding_citation',c.binding_receipt_id::text,'entity_citation',c.entity_receipt_id::text,
    'stat_citation',c.stat_receipt_id::text,'source_revision',c.source_revision)
    FROM serve.p4_patch_change c JOIN serve.p4_patch_catalog p ON p.patch_id=c.patch_id
    WHERE c.entity_id=$1 AND ($2::bigint IS NULL OR c.patch_id=$2)
        AND ($3::text IS NULL OR p.released_at >= $3::timestamptz)
        AND ($4::text IS NULL OR p.released_at <= $4::timestamptz)
    ORDER BY p.released_at DESC,p.patch_id DESC,c.change_id DESC LIMIT 20";

struct Database {
    admin: PgPool,
    reader: PgPool,
    writer: PgPool,
    store: GameFactsStore,
    log: std::path::PathBuf,
    tool_calls: std::sync::atomic::AtomicU64,
    tool_peak_us: std::sync::atomic::AtomicU64,
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
        assert_eq!(versions, [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 13, 23, 28, 31]);
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
            tool_calls: Default::default(),
            tool_peak_us: Default::default(),
            writer,
        }
    }

    async fn publish(
        &self,
        source: &str,
        path: &str,
        revision: &str,
        value: Value,
        hero: bool,
    ) -> RawObject {
        self.publish_with_factset((source, path, revision), value, hero, |_| {})
            .await
    }

    async fn publish_with_factset(
        &self,
        (source, path, revision): (&str, &str, &str),
        mut value: Value,
        hero: bool,
        prepare: impl FnOnce(&mut FactSet),
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
        let mut set = if hero {
            hero_facts(&raw, &value)
        } else {
            patch_facts(&raw, &value, changes.as_ref().unwrap())
        };
        prepare(&mut set);
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

    async fn assert_change_projection_equivalence(&self, entity: i64) {
        let mut tx = self.admin.begin().await.unwrap();
        sqlx::raw_sql(
            "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY; SET LOCAL ROLE brain_tool",
        )
        .execute(&mut *tx)
        .await
        .unwrap();
        let (since, until, patch): (Option<String>, Option<String>, Option<i64>) = sqlx::query_as("SELECT min(released_at)::text,max(released_at)::text,(array_agg(patch_id ORDER BY released_at DESC,patch_id DESC))[1] FROM serve.p4_patch_catalog")
            .fetch_one(&mut *tx).await.unwrap();
        let current = changes_sql();
        let scenarios = [
            (false, None, None, None),
            (true, patch, None, None),
            (true, None, since, until),
        ];
        let mut counts = Vec::new();
        for (bounded, patch, since, until) in scenarios {
            let mut results = Vec::new();
            for sql in [PRE_FACTOR_CHANGES, current.as_str()] {
                let sql = if bounded {
                    sql
                } else {
                    sql.strip_suffix(" LIMIT 20").unwrap()
                };
                let rows: Vec<Value> = sqlx::query_scalar(sql)
                    .bind(entity)
                    .bind(patch)
                    .bind(&since)
                    .bind(&until)
                    .fetch_all(&mut *tx)
                    .await
                    .unwrap();
                results.push(rows);
            }
            assert_eq!(results[0], results[1]);
            assert_eq!(
                serde_json::to_vec(&results[0]).unwrap(),
                serde_json::to_vec(&results[1]).unwrap()
            );
            counts.push(results[0].len());
        }
        tx.rollback().await.unwrap();
        println!("P4_SQL_EQUIVALENCE entity={entity} full_before_limit/current/range_rows={counts:?}; complete original/factored rows and numeric citation IDs byte-identical within one brain_tool read-only repeatable-read snapshot");
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
        let started = std::time::Instant::now();
        let execution = execute_patch_history(&self.reader, &query, &context, &id, &request)
            .await
            .unwrap();
        self.tool_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.tool_peak_us.fetch_max(
            started.elapsed().as_micros().try_into().unwrap(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.assert_change_projection_equivalence(request.arguments()["entity"].as_i64().unwrap())
            .await;
        execution
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
        let calls = self.tool_calls.load(std::sync::atomic::Ordering::Relaxed);
        let peak_us = self.tool_peak_us.load(std::sync::atomic::Ordering::Relaxed);
        println!(
            "P4_RUNTIME_TIMING_PROOF actual complete tool calls={calls}, peak_wall_us={peak_us}, pool/transaction/SQL/fetch/format/bounding/seal/validation included, unchanged 500ms aggregate and statement timeout"
        );
        self.reader.close().await;
        self.writer.close().await;
        self.admin.close().await;
    }
}

fn changes_sql() -> String {
    include_str!("../src/pg_patch_tools.rs")
        .split_once("const CHANGES: &str = \"")
        .unwrap()
        .1
        .split_once("\";")
        .unwrap()
        .0
        .replace("\\\"", "\"")
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

fn raw_receipt_id(execution: &ToolExecution, citation: &Value) -> i64 {
    let id = citation.as_str().unwrap();
    let evidence = execution.dependencies[0]
        .evidence
        .iter()
        .find(|evidence| evidence.evidence_id == id)
        .unwrap();
    CanonicalReceipt::from_evidence(evidence)
        .unwrap()
        .raw_receipt_id
}

fn projected_execution(
    bounded: &ToolExecution,
    parts: &[&ToolExecution],
    total: usize,
) -> ToolExecution {
    let mut execution = bounded.clone();
    let changes = parts
        .iter()
        .flat_map(|part| rows(part).iter().cloned())
        .collect::<Vec<_>>();
    let omitted = total - changes.len();
    execution.result.result["changes"] = json!(changes);
    execution.result.result["omitted_count"] = json!(omitted);
    execution.result.result["truncated"] = json!(omitted > 0);
    let evidence = std::iter::once(bounded)
        .chain(parts.iter().copied())
        .flat_map(|part| &part.dependencies[0].evidence)
        .map(|evidence| {
            let receipt = CanonicalReceipt::from_evidence(evidence).unwrap();
            (receipt.raw_receipt_id.to_string(), evidence.clone())
        })
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect::<Vec<_>>();
    execution.result.evidence_ids = evidence
        .iter()
        .map(|evidence| evidence.evidence_id.clone())
        .collect();
    execution.dependencies[0].request = bounded.dependencies[0].request.without_canonical_result();
    execution.dependencies[0].evidence = evidence;
    execution.dependencies[0] = execution.dependencies[0]
        .clone()
        .with_canonical_result(&execution.result)
        .unwrap();
    execution
}

fn execution_bytes(execution: &ToolExecution) -> usize {
    let request = execution.dependencies[0].request.without_canonical_result();
    let call = ToolCall {
        id: execution.result.call_id.clone(),
        name: request.name(),
        arguments: request.arguments().clone(),
    };
    execution.validate_for(&call, &request, None).unwrap();
    assert_eq!(execution.result.call_id, "\"".repeat(512));
    serde_json::to_vec(execution).unwrap().len()
}

fn verify_receipts(execution: &ToolExecution, objects: &[RawObject]) {
    verify_receipt_integrity(execution, objects);
    assert!(execution_bytes(execution) < 16_384);
}

fn verify_receipt_integrity(execution: &ToolExecution, objects: &[RawObject]) {
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
        assert!(content.get("content_sha256").is_none());
        assert_eq!(
            <[u8; 32]>::from(Sha256::digest(&raw.bytes)),
            raw.content_sha256
        );
        assert_eq!(evidence.source_id, format!("canon.source/{}", raw.source));
        let metadata = CanonicalReceipt::from_evidence(evidence).unwrap();
        assert_eq!(evidence.citation, evidence.evidence_id);
        assert_eq!(content["raw_receipt_id"], metadata.raw_receipt_id);
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
        .publish_with_factset(
            ("deadlock_patchnotes_db", "10", "opaque-r1-not-a-patch"),
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
            |set| {
                let corrupted: Vec<_> = set
                    .facts
                    .iter_mut()
                    .filter(|fact| fact.stat_key.ends_with("_5"))
                    .collect();
                assert_eq!(corrupted.len(), 1);
                for fact in corrupted {
                    fact.stat_key = format!(
                        "patch_change_r{}_5",
                        hex::encode(Sha256::digest(b"infernus"))
                    );
                }
            },
        )
        .await;
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
    let plan: Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {}",
        changes_sql()
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
    let lexical_id = raw_receipt_id(&execution, &first_rows[0]["citation"]);
    db.quarantine(&published_definition, &[lexical_id]).await;
    let narrowed = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&narrowed).len(), 1);
    assert_eq!(rows(&narrowed)[0]["line_text"], line);
    assert!(!narrowed
        .result
        .evidence_ids
        .contains(&first_rows[0]["citation"].as_str().unwrap().to_owned()));
    for field in [
        "binding_citation",
        "entity_citation",
        "head_citation",
        "label_citation",
        "released_at_citation",
        "url_citation",
    ] {
        let hidden = raw_receipt_id(&execution, &first_rows[0][field]);
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
    let hidden_stat = raw_receipt_id(&execution, &first_rows[0]["stat_citation"]);
    db.quarantine(&published_definition, &[hidden_stat]).await;
    let no_stat = db.execute(json!({"entity":id})).await;
    assert_eq!(rows(&no_stat).len(), 2);
    assert!(rows(&no_stat)
        .iter()
        .all(|row| row["stat_key"].is_null() && row["stat_citation"].is_null()));
    assert!(!no_stat
        .result
        .evidence_ids
        .contains(&first_rows[0]["stat_citation"].as_str().unwrap().to_owned()));
    db.quarantine(&published_definition, &[]).await;
    assert_eq!(db.execute(json!({"entity":id})).await, execution);
    let (query, context) = query_context();
    validate_patch_dependency(&db.reader, &query, &context, &execution.dependencies[0])
        .await
        .unwrap();
    let request = execution.dependencies[0].request.without_canonical_result();
    assert!(matches!(
        execute_patch_history(&db.admin, &query, &context, "patch-call", &request).await,
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
    let complete = projected_execution(&bounded, &[&execution], 2);
    assert_eq!(rows(&complete), rows(&execution));
    verify_receipt_integrity(
        &complete,
        &[
            baba.clone(),
            infernus.clone(),
            first.clone(),
            second.clone(),
        ],
    );
    let complete_bytes = execution_bytes(&complete);
    let bounded_bytes = execution_bytes(&bounded);
    assert!(complete_bytes >= 16_384);
    assert_eq!(rows(&bounded), &rows(&execution)[..1]);
    assert_eq!(bounded.result.result["omitted_count"], 1);
    assert_eq!(bounded.result.result["truncated"], true);
    assert_eq!(bounded.result.result["omitted_count_is_lower_bound"], false);
    println!(
        "P4_PROJECTION_BOUND_PROOF two original statements independently visible with exact original-byte receipts; full sealed two-row execution={complete_bytes} bytes, final one-row execution={bounded_bytes} bytes, omitted_count=1, limit=16384, worst_call_id=512 escaped quotes"
    );
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
        execute_patch_history(&db.reader, &query, &context, "timeout-proof", &request).await,
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
    for (index, date) in ["not-a-date", "2026-99-99T12:00:00Z"]
        .into_iter()
        .enumerate()
    {
        db.publish(
            "deadlock_patchnotes_db",
            "10",
            &format!("invalid-date-{index}"),
            patch(
                10,
                "invalid-date",
                date,
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
        error(
            &db.execute(json!({"entity":id})).await,
            "official_patch_unavailable",
        );
    }
    println!("P4_HEAD_PROOF mismatched raw/run/extractor hidden, complete statement removal and invalid dates hidden through real GameFactsStore and brain_tool queries");
    db.close().await;
}

struct NoOtherTools;

impl ToolExecutionPort for NoOtherTools {
    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        _: &str,
        _: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        panic!("canonical patch execution must not reach the delegate")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn quarantined_omitted_middle_post_invalidates_every_validation_purpose() {
    let pg = scratch_pg::ScratchPg::start();
    let db = Database::start(&pg).await;
    let hero = db
        .publish(
            "heroes",
            "baba",
            "hero",
            json!({"key":"hero_baba","aliases":["Baba"],"health":500}),
            true,
        )
        .await;
    let entity = db.hero_id("hero_baba").await;
    let mut objects = vec![hero];
    for index in 1..=3 {
        objects.push(
            db.publish(
                "deadlock_patchnotes_db",
                &index.to_string(),
                &format!("r{index}"),
                patch(
                    index,
                    &format!("p{index}"),
                    &format!("2026-10-0{index}T12:00:00Z"),
                    vec![change(
                        1,
                        "baba",
                        "health",
                        &format!("Baba: health changed from {index} to {}", index + 1),
                    )],
                ),
                false,
            )
            .await,
        );
    }
    let projection = sqlx::query_scalar::<_, Value>(&changes_sql())
        .bind(entity)
        .bind(None::<i64>)
        .bind(Some("2026-10-01T12:00:00Z"))
        .bind(Some("2026-10-03T12:00:00Z"))
        .fetch_all(&db.reader)
        .await
        .unwrap();
    assert_eq!(projection.len(), 3);
    assert_eq!(
        projection
            .iter()
            .map(|row| &row["patch"])
            .collect::<Vec<_>>(),
        [&json!("p3"), &json!("p2"), &json!("p1")]
    );
    let mut parts = Vec::new();
    for projected in &projection {
        let label = &projected["patch"];
        let part = db
            .execute(json!({"entity":entity,"since":label,"until":label}))
            .await;
        assert_eq!(rows(&part).len(), 1);
        let row = &rows(&part)[0];
        for field in [
            "id",
            "entity_id",
            "stat_key",
            "old",
            "new",
            "line_text",
            "patch",
            "statement_index",
            "source_revision",
        ] {
            assert_eq!(row[field], projected[field]);
        }
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
            assert_eq!(
                raw_receipt_id(&part, &row[field]).to_string(),
                projected[field].as_str().unwrap()
            );
        }
        verify_receipts(&part, &objects);
        parts.push(part);
    }
    let all = db
        .execute(json!({"entity":entity,"since":"p1","until":"p3"}))
        .await;
    let complete = projected_execution(&all, &[&parts[0], &parts[1], &parts[2]], 3);
    let prefix = projected_execution(&all, &[&parts[0], &parts[1]], 3);
    assert_eq!(rows(&complete).len(), 3);
    verify_receipt_integrity(&complete, &objects);
    verify_receipt_integrity(&prefix, &objects);
    let complete_bytes = execution_bytes(&complete);
    let prefix_bytes = execution_bytes(&prefix);
    let bounded_bytes = execution_bytes(&all);
    assert!(complete_bytes >= 16_384);
    assert!(prefix_bytes >= 16_384);
    assert_eq!(rows(&all), rows(&parts[0]));
    assert_eq!(all.result.result["omitted_count"], 2);
    assert_eq!(all.result.result["truncated"], true);
    assert_eq!(all.result.result["omitted_count_is_lower_bound"], false);
    verify_receipts(&all, &objects);
    println!(
        "P4_THREE_POST_BOUND_PROOF real CHANGES projection=3 rows, independent bounded requests verify every original-byte receipt; full sealed three-row execution={complete_bytes} bytes, two-row prefix={prefix_bytes} bytes, final one-row execution={bounded_bytes} bytes, limit=16384, worst_call_id=512 escaped quotes"
    );
    let middle = &rows(&parts[1])[0];
    assert_eq!(middle["patch"], "p2");
    let hidden = raw_receipt_id(&parts[1], &middle["citation"]);
    let arguments = json!({"entity":entity,"since":"p1","until":"p3","limit":1});
    let stale = db.execute(arguments.clone()).await;
    assert_eq!(rows(&stale).len(), 1);
    assert_eq!(rows(&stale)[0]["patch"], "p3");
    assert_eq!(stale.result.result["omitted_count"], 2);
    assert_eq!(rows(&stale), rows(&all));
    assert_eq!(stale.result.evidence_ids, all.result.evidence_ids);
    assert_eq!(stale.dependencies[0].evidence, all.dependencies[0].evidence);
    verify_receipts(&stale, &objects);
    assert!(!stale
        .result
        .evidence_ids
        .contains(&middle["citation"].as_str().unwrap().to_owned()));
    let port = brain_serve::pg_patch_port::PgPatchToolPort::new(
        NoOtherTools,
        db.reader.clone(),
        tokio::runtime::Handle::current(),
    )
    .unwrap();
    let (query, context) = query_context();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        port.validate_dependencies(&query, &context, None, &stale.dependencies, purpose)
            .unwrap();
    }
    let definition: String =
        sqlx::query_scalar("SELECT pg_get_viewdef('serve.published_receipt'::regclass, true)")
            .fetch_one(&db.admin)
            .await
            .unwrap();
    db.quarantine(&definition, &[hidden]).await;
    let fresh = db.execute(arguments).await;
    assert_eq!(rows(&fresh), rows(&stale));
    assert_eq!(fresh.result.evidence_ids, stale.result.evidence_ids);
    assert_eq!(
        fresh.dependencies[0].evidence,
        stale.dependencies[0].evidence
    );
    assert_eq!(fresh.result.result["omitted_count"], 1);
    assert_eq!(
        fresh.result.result["current_official_patch"],
        stale.result.result["current_official_patch"]
    );
    assert_eq!(fresh.result.result["range"], stale.result.result["range"]);
    assert_eq!(
        fresh.dependencies[0].request.without_canonical_result(),
        stale.dependencies[0].request.without_canonical_result()
    );
    assert_ne!(fresh.dependencies[0].request, stale.dependencies[0].request);
    let mut result = fresh.result.result.clone();
    result["omitted_count"] = json!(2);
    assert_eq!(result, stale.result.result);
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(matches!(
            port.validate_dependencies(&query, &context, None, &stale.dependencies, purpose),
            Err(PortError::Unavailable(_))
        ));
        port.validate_dependencies(&query, &context, None, &fresh.dependencies, purpose)
            .unwrap();
    }
    db.quarantine(&definition, &[]).await;
    assert_eq!(
        db.execute(json!({"entity":entity,"since":"p1","until":"p3","limit":1}))
            .await,
        stale
    );
    println!("P4_OMISSION_SEAL_PROOF three posts/runs, only omitted middle receipt quarantined, selected row/evidence unchanged, omitted_count 2->1, stale seal rejected for Provider/Publication/Cache");
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
