#[path = "../src/docs_tools.rs"]
mod docs_tools;
#[path = "../src/internal_docs.rs"]
mod internal_docs;
#[path = "../src/pg_docs.rs"]
mod pg_docs;
#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

use brain_contracts::{
    AnswerProfile, AuthorizedContext, Budget, Principal, Query, ToolCall, ToolDefinition,
    ToolEvidenceDependency, ToolName, ToolValidationPurpose,
};
use pg_docs::DocsReader;
use sqlx::{postgres::PgConnectOptions, PgPool, Row};
use std::collections::BTreeSet;

const MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-13-brain-gamefacts-p5-docs-v1.sql");

async fn setup(pg: &scratch_pg::ScratchPg) -> PgPool {
    let pool = PgPool::connect_with(
        PgConnectOptions::new()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username("brain_core_test")
            .database("postgres"),
    )
    .await
    .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate; CREATE ROLE brain_ingest; CREATE ROLE brain_service; CREATE ROLE brain_tool; CREATE ROLE brain_readonly LOGIN; GRANT brain_tool TO brain_readonly WITH INHERIT FALSE, SET TRUE;")
        .execute(&pool).await.unwrap();
    let base = brain_storage::gamefacts::limits::render_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .replace("{{HERO_PROFILE}}", "");
    sqlx::raw_sql(&base).execute(&pool).await.unwrap();
    sqlx::raw_sql(MIGRATION).execute(&pool).await.unwrap();
    sqlx::raw_sql("CREATE TABLE canon.test_quarantine(receipt_id bigint PRIMARY KEY); CREATE OR REPLACE VIEW serve.published_receipt AS SELECT r.*,o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256 FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded' JOIN raw.object o ON o.id=r.raw_object_id WHERE NOT EXISTS(SELECT 1 FROM canon.test_quarantine q WHERE q.receipt_id=r.id)")
        .execute(&pool).await.unwrap();
    pool
}

async fn receipt(pool: &PgPool, raw: i64, run: i64, section: bool) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'docs',$3,$4,$5) RETURNING id")
        .bind(raw).bind(run)
        .bind(if section { "byte_ranges" } else { "bytes" })
        .bind(if section { "[[0,4]]" } else { "0:4" })
        .bind(vec![1_u8;32]).fetch_one(pool).await.unwrap()
}

async fn fact(
    pool: &PgPool,
    entity: i64,
    raw: i64,
    run: i64,
    key: &str,
    value: &str,
    section: bool,
) -> i64 {
    let receipt = receipt(pool, raw, run, section).await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_text,valid_from_run,receipt_id) VALUES($1,$2,$3,$4,$5)")
        .bind(entity).bind(key).bind(value).bind(run).bind(receipt).execute(pool).await.unwrap();
    receipt
}

async fn document(
    pool: &PgPool,
    path: &str,
    version: u8,
    prior: Option<i64>,
    complete: bool,
) -> (i64, i64) {
    let run: i64 = sqlx::query_scalar("INSERT INTO ops.ingest_run(source,status) VALUES('deadlock-docs','succeeded') RETURNING id")
        .fetch_one(pool).await.unwrap();
    let raw: i64 = sqlx::query_scalar("INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES('deadlock-docs',$1,$2,$3,$4,$5,$6) RETURNING id")
        .bind(format!("{version:040x}")).bind(path).bind(vec![version;32]).bind(vec![version;32]).bind(b"test".to_vec()).bind(run)
        .fetch_one(pool).await.unwrap();
    let identity = receipt(pool, raw, run, false).await;
    let entity = if let Some(entity) = prior {
        entity
    } else {
        sqlx::query_scalar(
            "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('doc_page',$1,$2) RETURNING id",
        )
        .bind(format!("doc_{version}"))
        .bind(identity)
        .fetch_one(pool)
        .await
        .unwrap()
    };
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(entity)
        .bind(identity)
        .execute(pool)
        .await
        .unwrap();
    for (key, value) in [
        ("doc_path", path),
        ("doc_title", "Baba erklärt"),
        ("doc_lang", "de"),
    ] {
        fact(pool, entity, raw, run, key, value, false).await;
    }
    let count = receipt(pool, raw, run, false).await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'doc_section_count',1,$2,$3)")
        .bind(entity).bind(run).bind(count).execute(pool).await.unwrap();
    fact(pool,entity,raw,run,"doc_intro_meta",&serde_json::json!({"ordinal":0,"locator":"intro","heading_path":["Baba"],"line_start":1,"line_end":3,"chunks":2}).to_string(),true).await;
    let body_receipt = fact(
        pool,
        entity,
        raw,
        run,
        "doc_intro_body_0",
        "Baba bewegt ",
        true,
    )
    .await;
    if complete {
        fact(
            pool,
            entity,
            raw,
            run,
            "doc_intro_body_1",
            "sich schnell.",
            true,
        )
        .await;
    }
    (entity, body_receipt)
}

fn context(internal: bool) -> (Query, AuthorizedContext) {
    let query = Query {
        request_id: "docs-test".into(),
        conversation_id: "docs-test".into(),
        text: "Baba".into(),
        answer_context: None,
        domain: None,
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    };
    let context = AuthorizedContext {
        discord: None,
        principal: Principal {
            actor_id: if internal {
                "second-brain"
            } else {
                "docs-client"
            }
            .into(),
            channel: if internal { "internal" } else { "web" }.into(),
            scopes: BTreeSet::from([if internal {
                "second_brain.internal"
            } else {
                "docs.public"
            }
            .into()]),
            provider_egress: if internal {
                BTreeSet::new()
            } else {
                BTreeSet::from(["public".into()])
            },
        },
        conversation_id: "docs-test".into(),
        knowledge_release: "test".into(),
        deadline_ms: 5_000,
        budget: Budget::default(),
        request_deadline: None,
    };
    (query, context)
}

#[tokio::test(flavor = "multi_thread")]
async fn published_projection_rejects_mixed_revisions_and_incomplete_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (entity, _) = document(&pool, "public/baba.html", 1, None, true).await;
    let body: String = sqlx::query_scalar("SELECT body FROM serve.doc_section")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(body, "Baba bewegt sich schnell.");
    document(&pool, "public/baba.html", 2, Some(entity), false).await;
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.doc_section")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let meta: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT serve.doc_meta('{\"chunks\":\"oops\"}')")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(meta.is_none());
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn reader_is_scoped_bounded_and_revalidates_published_state() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (_, receipt) = document(&pool, "public/baba.html", 1, None, true).await;
    document(&pool, "internal/baba.html", 2, None, true).await;
    let reader_pool = PgPool::connect_with(
        PgConnectOptions::new()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username("brain_readonly")
            .database("postgres"),
    )
    .await
    .unwrap();
    let reader = DocsReader::new(reader_pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let evidence = reader.retrieve(&query, &ctx).unwrap();
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte.".into(),
        input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
    };
    let tools = pg_docs::DocsTools::new(None, reader.clone(), definition.clone()).unwrap();
    let request = ToolCall {
        id: "docs-execution".into(),
        name: ToolName::SearchDocs,
        arguments: serde_json::json!({"query":"Baba","limit":5}),
    }
    .validate(&[definition])
    .unwrap();
    let execution = brain_contracts::ToolExecutionPort::execute(
        &tools,
        &query,
        &ctx,
        None,
        "docs-execution",
        &request,
    )
    .unwrap();
    assert_eq!(
        execution.result.result["sections"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(serde_json::to_vec(&execution).unwrap().len() <= 16_384);
    reader
        .validate_dependency(
            &query,
            &ctx,
            &execution.dependencies[0],
            ToolValidationPurpose::Cache,
        )
        .unwrap();
    brain_contracts::ToolExecutionPort::validate_dependencies(
        &tools,
        &query,
        &ctx,
        None,
        &execution.dependencies,
        ToolValidationPurpose::Cache,
    )
    .unwrap();
    assert_eq!(evidence.len(), 1);
    assert!(evidence[0].citation.contains("/public/baba.html#L1-L3"));
    assert!(evidence[0].patch.is_none());
    assert!(serde_json::to_vec(&evidence).unwrap().len() <= 16_384);
    reader
        .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
        .unwrap();
    let (internal_query, internal_ctx) = context(true);
    let internal = reader.retrieve(&internal_query, &internal_ctx).unwrap();
    assert_eq!(internal.len(), 1);
    assert!(internal[0].citation.starts_with("receipt/"));
    assert!(
        serde_json::from_str::<serde_json::Value>(&internal[0].content)
            .unwrap()
            .get("url")
            .is_none()
    );
    assert!(reader
        .validate_evidence(
            &internal_query,
            &internal_ctx,
            &internal,
            ToolValidationPurpose::Provider
        )
        .is_err());
    let adapter = pg_docs::InternalDocsReader::new(reader.clone());
    assert_eq!(
        brain_contracts::RetrievalPort::retrieve(&adapter, &internal_query, &internal_ctx).unwrap(),
        internal
    );
    assert!(brain_contracts::RetrievalPort::retrieve(&adapter, &query, &ctx).is_err());
    let definition = ToolDefinition {
        name: ToolName::ServerKnowledge,
        description: "test".into(),
        input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
    };
    assert!(pg_docs::DocsTools::new(None, reader.clone(), definition).is_err());
    let wrong_request=ToolCall {id:"test".into(),name:ToolName::ServerKnowledge,arguments:serde_json::json!({"question":"Baba"})}
        .validate(&[ToolDefinition {name:ToolName::ServerKnowledge,description:"test".into(),input_schema:serde_json::json!({"type":"object","additionalProperties":false,"required":["question"],"properties":{"question":{"type":"string"}}})}]).unwrap();
    assert!(reader.search(&query, &ctx, "test", &wrong_request).is_err());
    let dependency = ToolEvidenceDependency {
        request: wrong_request,
        game_context: None,
        evidence: evidence.clone(),
    };
    assert!(reader
        .validate_dependency(&query, &ctx, &dependency, ToolValidationPurpose::Cache)
        .is_err());
    sqlx::query("INSERT INTO canon.test_quarantine VALUES($1)")
        .bind(receipt)
        .execute(&pool)
        .await
        .unwrap();
    assert!(reader.retrieve(&query, &ctx).unwrap().is_empty());
    assert!(brain_contracts::ToolExecutionPort::validate_dependencies(
        &tools,
        &query,
        &ctx,
        None,
        &execution.dependencies,
        ToolValidationPurpose::Cache
    )
    .is_err());
    assert!(reader
        .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
        .is_err());
    reader_pool.close().await;
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn search_terms_can_span_title_headings_and_body_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document(&pool, "public/baba.html", 1, None, true).await;
    sqlx::raw_sql("UPDATE canon.fact SET value_text=CASE stat_key::text WHEN 'doc_intro_body_0' THEN 'Early ' WHEN 'doc_intro_body_1' THEN 'Game' WHEN 'doc_intro_meta' THEN '{\"ordinal\":0,\"locator\":\"intro\",\"heading_path\":[],\"line_start\":1,\"line_end\":3,\"chunks\":2}' ELSE value_text END")
        .execute(&pool).await.unwrap();
    let rows = sqlx::query(include_str!("../src/pg_docs/search.sql"))
        .bind("public")
        .bind("Baba Early Game")
        .bind(None::<i64>)
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn entity_filter_excludes_doc_carriers_and_explain_uses_real_postgres() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (page, _) = document(&pool, "public/baba.html", 1, None, true).await;
    let sql = include_str!("../src/pg_docs/search.sql");
    let rows = sqlx::query(sql)
        .bind("public")
        .bind("Baba")
        .bind(Some(page))
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert!(rows.is_empty());
    let receipt: i64 = sqlx::query_scalar("SELECT receipt_id FROM canon.entity WHERE id=$1")
        .bind(page)
        .fetch_one(&pool)
        .await
        .unwrap();
    let game: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','baba',$1) RETURNING id",
    )
    .bind(receipt)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(game)
        .bind(receipt)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) VALUES($1,'baba','de',$2)",
    )
    .bind(game)
    .bind(receipt)
    .execute(&pool)
    .await
    .unwrap();
    let rows = sqlx::query(sql)
        .bind("public")
        .bind("Baba")
        .bind(Some(game))
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    let plans = sqlx::query(&format!("EXPLAIN (ANALYZE,BUFFERS,FORMAT TEXT) {sql}"))
        .bind("public")
        .bind("Baba")
        .bind(None::<i64>)
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap();
    for row in plans {
        eprintln!("{}", row.get::<String, _>(0));
    }
    let allowed:bool=sqlx::query_scalar("SELECT has_table_privilege('brain_tool','serve.doc_section','SELECT') AND NOT has_table_privilege('brain_tool','canon.fact','SELECT') AND NOT has_schema_privilege('brain_tool','raw','USAGE')")
        .fetch_one(&pool).await.unwrap();
    assert!(allowed);
    pool.close().await;
}
