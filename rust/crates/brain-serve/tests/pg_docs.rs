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

async fn setup(pg: &scratch_pg::ScratchPg) -> PgPool {
    let pool = PgPool::connect_with(
        PgConnectOptions::new_without_pgpass()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username("brain_core_test")
            .database("postgres"),
    )
    .await
    .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
        .execute(&pool).await.unwrap();
    brain_storage::PgStore::new(pool.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TABLE canon.test_quarantine(receipt_id bigint PRIMARY KEY); ALTER TABLE canon.test_quarantine OWNER TO brain_migrate; CREATE OR REPLACE VIEW serve.published_receipt AS SELECT r.*,o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256 FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded' JOIN raw.object o ON o.id=r.raw_object_id WHERE NOT EXISTS(SELECT 1 FROM canon.test_quarantine q WHERE q.receipt_id=r.id)")
        .execute(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn registered_docs_migration_is_recorded_and_idempotent() {
    use sha2::{Digest, Sha256};

    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let store = brain_storage::PgStore::new(pool.clone());
    let versions = "SELECT version,checksum FROM ops.gamefacts_schema_version ORDER BY version";
    let before: Vec<(i32, Vec<u8>)> = sqlx::query_as(versions).fetch_all(&pool).await.unwrap();
    assert_eq!(
        before.iter().map(|(v, _)| *v).collect::<Vec<_>>(),
        [1, 5, 7]
    );
    let rendered = brain_storage::gamefacts::limits::render_sql(include_str!(
        "../../../../scripts/migrations/2026-10-13-brain-gamefacts-p5-docs-v1.sql"
    ));
    assert_eq!(before[1].1, Sha256::digest(rendered.as_bytes()).to_vec());
    let objects = "SELECT to_regclass('serve.doc_page')::oid::bigint,to_regclass('serve.doc_section')::oid::bigint,to_regprocedure('serve.doc_meta(text)')::oid::bigint,to_regprocedure('serve.doc_tsv(text)')::oid::bigint,to_regclass('canon.doc_body_fts')::oid::bigint";
    let installed: (i64, i64, i64, i64, i64) =
        sqlx::query_as(objects).fetch_one(&pool).await.unwrap();
    store.check_gamefacts_schema().await.unwrap();
    store.migrate_gamefacts().await.unwrap();
    let after: Vec<(i32, Vec<u8>)> = sqlx::query_as(versions).fetch_all(&pool).await.unwrap();
    assert_eq!(before, after);
    let unchanged: (i64, i64, i64, i64, i64) =
        sqlx::query_as(objects).fetch_one(&pool).await.unwrap();
    assert_eq!(installed, unchanged);
    store.check_gamefacts_schema().await.unwrap();
    pool.close().await;
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
    let (public_page, receipt) = document(&pool, "public/baba.html", 1, None, true).await;
    let (internal_page, internal_receipt) =
        document(&pool, "internal/baba.html", 2, None, true).await;
    let reader_pool = PgPool::connect_with(
        PgConnectOptions::new_without_pgpass()
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
    let published_receipt: i64 =
        sqlx::query_scalar("SELECT receipt_id FROM serve.doc_section WHERE page_id=$1")
            .bind(public_page)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(published_receipt > 0);
    assert_eq!(evidence[0].evidence_id, published_receipt.to_string());
    assert_eq!(
        execution.result.evidence_ids,
        vec![published_receipt.to_string()]
    );
    assert_eq!(evidence[0].source_id, format!("docs.public/{public_page}"));
    assert_eq!(
        evidence[0].logical_id,
        format!("receipt/{published_receipt}")
    );
    assert_eq!(
        brain_contracts::PublicCitation::from_evidence(&evidence[0], 0).label,
        evidence[0].citation
    );
    assert!(evidence[0].citation.contains("/public/baba.html#L1-L3"));
    assert!(evidence[0].patch.is_none());
    assert!(serde_json::to_vec(&evidence).unwrap().len() <= 16_384);
    reader
        .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
        .unwrap();
    let (internal_query, internal_ctx) = context(true);
    let internal = reader.retrieve(&internal_query, &internal_ctx).unwrap();
    assert_eq!(internal.len(), 1);
    assert!(internal[0].evidence_id.parse::<i64>().unwrap() > 0);
    assert_eq!(
        internal[0].source_id,
        format!("docs.internal/{internal_page}")
    );
    assert_eq!(
        internal[0].logical_id,
        format!("receipt/{}", internal[0].evidence_id)
    );
    assert_eq!(internal[0].citation, internal[0].logical_id);
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
    brain_contracts::RetrievalPort::validate_evidence(
        &adapter,
        &internal_query,
        &internal_ctx,
        &internal,
        false,
    )
    .unwrap();
    assert!(brain_contracts::RetrievalPort::validate_evidence(
        &adapter,
        &internal_query,
        &internal_ctx,
        &internal,
        true,
    )
    .is_err());
    assert!(brain_contracts::RetrievalPort::validate_publication(
        &adapter,
        &internal_query,
        &internal_ctx,
        &internal,
    )
    .is_err());
    sqlx::query("INSERT INTO canon.test_quarantine VALUES($1)")
        .bind(internal_receipt)
        .execute(&pool)
        .await
        .unwrap();
    assert!(brain_contracts::RetrievalPort::validate_evidence(
        &adapter,
        &internal_query,
        &internal_ctx,
        &internal,
        false,
    )
    .is_err());
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
async fn internal_reader_uses_query_bounds_and_authorizes_empty_searches() {
    use brain_contracts::{tools::SearchDocsRequest, RetrievalPort};

    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document(&pool, "public/baba.html", 1, None, true).await;
    let (page, _) = document(&pool, "internal/baba.html", 2, None, true).await;
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let adapter = pg_docs::InternalDocsReader::new(reader.clone());
    let (mut query, ctx) = context(true);
    let original = adapter.retrieve(&query, &ctx).unwrap();
    assert_eq!(original.len(), 1);
    assert_eq!(original[0].source_id, format!("docs.internal/{page}"));
    for text in [
        format!("{}Baba", "Baba ".repeat(128)),
        format!("{}erklärt", "erklärt ".repeat(64)),
        format!("{}   ", "Baba ".repeat(6553)),
    ] {
        query.text = text;
        query.validate().unwrap();
        assert!(
            internal_docs::internal_search_text(&query).unwrap().len()
                > SearchDocsRequest::MAX_QUERY_BYTES
        );
        assert!(query.text.len() <= 32_768);
        let evidence = adapter.retrieve(&query, &ctx).unwrap();
        assert_eq!(evidence, original);
        assert!(evidence.len() <= SearchDocsRequest::MAX_RESULTS);
        assert!(serde_json::to_vec(&evidence).unwrap().len() <= 16_384);
        adapter
            .validate_evidence(&query, &ctx, &evidence, false)
            .unwrap();
        assert!(adapter
            .validate_evidence(&query, &ctx, &evidence, true)
            .is_err());
        assert!(adapter
            .validate_publication(&query, &ctx, &evidence)
            .is_err());
        query.text.push_str(" p5suffixnichtvorhanden");
        if query.text.len() <= 32_768 {
            assert!(adapter.retrieve(&query, &ctx).unwrap().is_empty());
            assert!(adapter
                .validate_evidence(&query, &ctx, &evidence, false)
                .is_err());
        }
    }
    query.text = "\0\u{0001}\u{007f}\t\r\n \u{2003}".into();
    query.validate().unwrap();
    assert!(adapter.retrieve(&query, &ctx).unwrap().is_empty());
    adapter.validate_evidence(&query, &ctx, &[], false).unwrap();
    let mut denied = ctx.clone();
    denied.principal.provider_egress.insert("public".into());
    assert!(matches!(
        adapter.retrieve(&query, &denied),
        Err(brain_contracts::PortError::PermissionDenied(_))
    ));
    assert!(matches!(
        reader.retrieve(&query, &denied),
        Err(brain_contracts::PortError::PermissionDenied(_))
    ));
    assert!(adapter
        .validate_evidence(&query, &denied, &[], false)
        .is_err());
    for text in [" ".into(), "x".repeat(32_769), "ä".repeat(16_385)] {
        query.text = text;
        assert!(adapter.retrieve(&query, &ctx).is_err());
        assert!(adapter.validate_evidence(&query, &ctx, &[], false).is_err());
    }
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte.".into(),
        input_schema: SearchDocsRequest::input_schema(),
    };
    for text in ["B".repeat(513), "ä".repeat(257), "\u{0001}".into()] {
        let arguments = serde_json::json!({"query":text});
        assert!(SearchDocsRequest::parse(&arguments).is_err());
        assert!(ToolCall {
            id: "bounded".into(),
            name: ToolName::SearchDocs,
            arguments
        }
        .validate(std::slice::from_ref(&definition))
        .is_err());
    }
    let arguments = serde_json::json!({"query":format!("{}  ", "Baba ".repeat(102))});
    assert_eq!(arguments["query"].as_str().unwrap().len(), 512);
    let request = ToolCall {
        id: "bounded".into(),
        name: ToolName::SearchDocs,
        arguments,
    }
    .validate(std::slice::from_ref(&definition))
    .unwrap();
    let tools = pg_docs::DocsTools::new(None, reader, definition).unwrap();
    let (public_query, public_ctx) = context(false);
    let execution = brain_contracts::ToolExecutionPort::execute(
        &tools,
        &public_query,
        &public_ctx,
        None,
        "bounded",
        &request,
    )
    .unwrap();
    assert_eq!(execution.dependencies[0].evidence.len(), 1);
    assert_eq!(
        execution.dependencies[0].evidence[0].visibility,
        brain_contracts::SourceVisibility::Public
    );
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn serving_preserves_extractor_language_values_and_receipts() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (page, _) = document(&pool, "public/baba.html", 1, None, true).await;
    document(&pool, "internal/baba.html", 2, None, true).await;
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let original = reader.retrieve(&query, &ctx).unwrap();
    let (internal_query, internal_ctx) = context(true);
    let adapter = pg_docs::InternalDocsReader::new(reader.clone());
    let internal =
        brain_contracts::RetrievalPort::retrieve(&adapter, &internal_query, &internal_ctx).unwrap();
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte.".into(),
        input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
    };
    let request = ToolCall {
        id: "language".into(),
        name: ToolName::SearchDocs,
        arguments: serde_json::json!({"query":"Baba"}),
    }
    .validate(std::slice::from_ref(&definition))
    .unwrap();
    let tools = pg_docs::DocsTools::new(None, reader.clone(), definition).unwrap();
    for lang in [
        "de-DE-1996".into(),
        "zh-Hant-TW".into(),
        "en-Latn-US-variant".into(),
        " deutsch ".into(),
        "日本語".into(),
        "a".repeat(brain_storage::gamefacts::limits::TEXT_BYTES),
        "ä".repeat(brain_storage::gamefacts::limits::TEXT_BYTES / 2),
    ] {
        assert!(!lang.trim().is_empty());
        assert!(!lang.chars().any(char::is_control));
        assert!(lang.len() <= brain_storage::gamefacts::limits::TEXT_BYTES);
        sqlx::query("UPDATE canon.fact SET value_text=$1 WHERE stat_key='doc_lang'")
            .bind(&lang)
            .execute(&pool)
            .await
            .unwrap();
        let projected: String =
            sqlx::query_scalar("SELECT lang FROM serve.doc_page WHERE page_id=$1")
                .bind(page)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(projected, lang);
        let section_lang: String =
            sqlx::query_scalar("SELECT lang FROM serve.doc_section WHERE page_id=$1")
                .bind(page)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(section_lang, lang);
        assert_eq!(reader.retrieve(&query, &ctx).unwrap(), original);
        reader
            .validate_evidence(&query, &ctx, &original, ToolValidationPurpose::Cache)
            .unwrap();
        assert_eq!(
            brain_contracts::RetrievalPort::retrieve(&adapter, &internal_query, &internal_ctx)
                .unwrap(),
            internal
        );
        let execution = brain_contracts::ToolExecutionPort::execute(
            &tools, &query, &ctx, None, "language", &request,
        )
        .unwrap();
        assert_eq!(execution.dependencies[0].evidence, original);
    }
    for lang in [
        String::new(),
        " \u{2003}\u{00a0}".into(),
        "de\n".into(),
        "de\u{0085}".into(),
        "de\u{009f}".into(),
    ] {
        sqlx::query("UPDATE canon.fact SET value_text=$1 WHERE stat_key='doc_lang'")
            .bind(lang)
            .execute(&pool)
            .await
            .unwrap();
        assert!(reader.retrieve(&query, &ctx).unwrap().is_empty());
    }
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn search_terms_can_span_title_headings_and_body_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document(&pool, "public/baba.html", 1, None, true).await;
    sqlx::raw_sql("UPDATE canon.fact SET value_text=CASE stat_key::text WHEN 'doc_intro_body_0' THEN 'Early ' WHEN 'doc_intro_body_1' THEN 'Game' WHEN 'doc_intro_meta' THEN '{\"ordinal\":0,\"locator\":\"intro\",\"heading_path\":[],\"line_start\":1,\"line_end\":3,\"chunks\":2}' ELSE value_text END")
        .execute(&pool).await.unwrap();
    for query in ["Baba Early Game", "Baba", "\"Early Game\"", "Early Game"] {
        let rows = sqlx::query(include_str!("../src/pg_docs/search.sql"))
            .bind("public")
            .bind(query)
            .bind(None::<i64>)
            .bind(6_i64)
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1, "{query}");
    }
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn search_keeps_language_vectors_and_phrase_positions_independent() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document(&pool, "public/phrases.html", 1, None, true).await;
    sqlx::raw_sql("UPDATE canon.fact SET value_text=CASE stat_key::text WHEN 'doc_title' THEN 'Document' WHEN 'doc_intro_body_0' THEN 'alpha ' WHEN 'doc_intro_body_1' THEN 'omega und Häuser' WHEN 'doc_intro_meta' THEN '{\"ordinal\":0,\"locator\":\"intro\",\"heading_path\":[],\"line_start\":1,\"line_end\":3,\"chunks\":2}' ELSE value_text END")
        .execute(&pool).await.unwrap();
    let matches: (bool, bool, bool, bool, bool, bool) = sqlx::query_as(
        "SELECT tsv @@ websearch_to_tsquery('pg_catalog.simple','\"omega alpha\"'),
            tsv_german @@ websearch_to_tsquery('pg_catalog.german','\"omega alpha\"'),
            tsv @@ websearch_to_tsquery('pg_catalog.simple','und'),
            tsv_german @@ websearch_to_tsquery('pg_catalog.german','und'),
            tsv @@ websearch_to_tsquery('pg_catalog.simple','Haus'),
            tsv_german @@ websearch_to_tsquery('pg_catalog.german','Haus')
        FROM serve.doc_section",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(matches, (false, false, true, false, false, true));
    let reverse: bool = sqlx::query_scalar(
        "SELECT serve.doc_tsv('alpha omega') @@ websearch_to_tsquery('pg_catalog.simple','\"omega alpha\"')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(!reverse);
    for (query, expected) in [
        ("\"omega alpha\"", 0),
        ("\"alpha omega\"", 1),
        ("und", 1),
        ("Haus", 1),
    ] {
        let rows = sqlx::query(include_str!("../src/pg_docs/search.sql"))
            .bind("public")
            .bind(query)
            .bind(None::<i64>)
            .bind(6_i64)
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(rows.len(), expected, "{query}");
    }
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn reader_compaction_preserves_websearch_membership_and_order() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    for (version, path) in [
        (1, "public/first.html"),
        (2, "public/second.html"),
        (3, "internal/first.html"),
        (4, "internal/second.html"),
    ] {
        let (page, _) = document(&pool, path, version, None, true).await;
        sqlx::query("UPDATE canon.fact SET value_text=CASE stat_key::text WHEN 'doc_title' THEN 'Document' WHEN 'doc_intro_body_0' THEN 'alpha ' WHEN 'doc_intro_body_1' THEN $2 WHEN 'doc_intro_meta' THEN '{\"ordinal\":0,\"locator\":\"intro\",\"heading_path\":[],\"line_start\":1,\"line_end\":3,\"chunks\":2}' ELSE value_text END WHERE entity_id=$1")
            .bind(page).bind(if version % 2 == 0 { "omega alpha und Häuser" } else { "omega und Häuser" })
            .execute(&pool).await.unwrap();
    }
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte.".into(),
        input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
    };
    for internal in [false, true] {
        let (mut query, ctx) = context(internal);
        for text in [
            "alpha alpha",
            "alpha omega alpha",
            "und und alpha",
            "Haus Häuser Haus",
            "alpha alpha p5suffixnichtvorhanden",
            "\"alpha omega\"",
            "\"omega alpha\"",
            "\"alpha alpha\"",
            "alpha -omega",
            "alpha -p5suffixnichtvorhanden",
            "alpha OR p5suffixnichtvorhanden",
            "p5suffixnichtvorhanden or alpha",
            "alpha Or alpha",
            "alpha-alpha alpha-alpha",
        ] {
            query.text = text.into();
            let rows = sqlx::query(include_str!("../src/pg_docs/search.sql"))
                .bind(if internal { "internal" } else { "public" })
                .bind(text)
                .bind(None::<i64>)
                .bind(6_i64)
                .fetch_all(&pool)
                .await
                .unwrap();
            let expected: Vec<_> = rows
                .iter()
                .map(|row| row.get::<i64, _>("receipt_id").to_string())
                .collect();
            let evidence = reader.retrieve(&query, &ctx).unwrap();
            assert_eq!(
                evidence
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect::<Vec<_>>(),
                expected,
                "{text}"
            );
            reader
                .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
                .unwrap();
            let request = ToolCall {
                id: "compaction".into(),
                name: ToolName::SearchDocs,
                arguments: serde_json::json!({"query":text}),
            }
            .validate(std::slice::from_ref(&definition))
            .unwrap();
            let execution = reader.search(&query, &ctx, "compaction", &request).unwrap();
            assert_eq!(execution.result.evidence_ids, expected, "{text}");
            for dependency in &execution.dependencies {
                assert_eq!(dependency.request, request);
                reader
                    .validate_dependency(&query, &ctx, dependency, ToolValidationPurpose::Cache)
                    .unwrap();
            }
        }
    }
    for (original, compact) in [
        ("alpha alpha", "alpha"),
        ("alpha omega alpha", "alpha omega"),
        ("und und alpha", "und alpha"),
        ("Haus Häuser Haus", "Haus Häuser"),
    ] {
        let same: bool = sqlx::query_scalar("SELECT bool_and(
            (tsv @@ websearch_to_tsquery('pg_catalog.simple',$1))=(tsv @@ websearch_to_tsquery('pg_catalog.simple',$2))
            AND (tsv_german @@ websearch_to_tsquery('pg_catalog.german',$1))=(tsv_german @@ websearch_to_tsquery('pg_catalog.german',$2))
            AND ts_rank_cd(tsv,websearch_to_tsquery('pg_catalog.simple',$1))=ts_rank_cd(tsv,websearch_to_tsquery('pg_catalog.simple',$2))
            AND ts_rank_cd(tsv_german,websearch_to_tsquery('pg_catalog.german',$1))=ts_rank_cd(tsv_german,websearch_to_tsquery('pg_catalog.german',$2))) FROM serve.doc_section")
            .bind(original).bind(compact).fetch_one(&pool).await.unwrap();
        assert!(same, "{original}");
    }
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

fn plan_matches<F>(value: &serde_json::Value, predicate: F) -> bool
where
    F: Fn(&serde_json::Value) -> bool + Copy,
{
    predicate(value)
        || match value {
            serde_json::Value::Object(fields) => {
                fields.values().any(|child| plan_matches(child, predicate))
            }
            serde_json::Value::Array(values) => {
                values.iter().any(|child| plan_matches(child, predicate))
            }
            _ => false,
        }
}

#[tokio::test(flavor = "multi_thread")]
async fn current_fact_winner_selection_blocks_source_gin_predicate_pushdown() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (page, _) = document(&pool, "public/baba.html", 1, None, true).await;
    let (_, current_body_receipt) = document(&pool, "public/baba.html", 2, Some(page), true).await;
    sqlx::query("UPDATE canon.fact SET value_text='Baba bleibt ' WHERE entity_id=$1 AND stat_key='doc_intro_body_0' AND receipt_id=$2")
        .bind(page).bind(current_body_receipt).execute(&pool).await.unwrap();
    let canonical_sql = "SELECT entity_id,stat_key FROM canon.fact WHERE stat_key::text ~ '^doc_(intro|section_.+)_body_[0-9]+$' AND serve.doc_tsv(value_text::text) @@ websearch_to_tsquery('pg_catalog.simple','bewegt')";
    let published_sql = "SELECT entity_id,stat_key FROM serve.current_fact WHERE stat_key::text ~ '^doc_(intro|section_.+)_body_[0-9]+$' AND serve.doc_tsv(value_text::text) @@ websearch_to_tsquery('pg_catalog.simple','bewegt')";
    assert_eq!(
        sqlx::query(canonical_sql)
            .fetch_all(&pool)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(sqlx::query(published_sql)
        .fetch_all(&pool)
        .await
        .unwrap()
        .is_empty());
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql("SET LOCAL enable_seqscan=off; SET LOCAL jit=off")
        .execute(&mut *tx)
        .await
        .unwrap();
    let canonical_plan: serde_json::Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE,BUFFERS,FORMAT JSON) {canonical_sql}"
    ))
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let published_plan: serde_json::Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE,BUFFERS,FORMAT JSON) {published_sql}"
    ))
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let section_sql = include_str!("../src/pg_docs/search.sql");
    let section_plan: serde_json::Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE,BUFFERS,FORMAT JSON) {section_sql}"
    ))
    .bind("public")
    .bind("Baba")
    .bind(None::<i64>)
    .bind(6_i64)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let indexed_fts = |node: &serde_json::Value| {
        node["Index Name"] == "doc_body_fts"
            && node["Index Cond"]
                .as_str()
                .is_some_and(|condition| condition.contains("@@"))
    };
    assert!(plan_matches(&canonical_plan, indexed_fts));
    assert!(!plan_matches(&published_plan, indexed_fts));
    assert!(!plan_matches(&section_plan, indexed_fts));
    assert!(plan_matches(&published_plan, |node| {
        node["Node Type"] == "Subquery Scan"
            && node["Filter"]
                .as_str()
                .is_some_and(|filter| filter.contains("@@"))
            && plan_matches(&node["Plans"], |child| child["Node Type"] == "Unique")
    }));
    eprintln!(
        "FTS_PROOF {}",
        serde_json::json!({
            "enable_seqscan":"off","jit":"off","canonical_sql":canonical_sql,
            "published_sql":published_sql,"section_sql":section_sql,
            "section_parameters":["public","Baba",null,6],
            "canonical_plan":canonical_plan,"published_plan":published_plan,"section_plan":section_plan,
        })
    );
    tx.commit().await.unwrap();
    sqlx::query("INSERT INTO canon.test_quarantine VALUES($1)")
        .bind(current_body_receipt)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        sqlx::query(published_sql)
            .fetch_all(&pool)
            .await
            .unwrap()
            .len(),
        1
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.doc_section")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let mut tx = pool.begin().await.unwrap();
    let error =
        sqlx::raw_sql("CREATE INDEX scratch_doc_section_fts ON serve.doc_section USING gin(tsv)")
            .execute(&mut *tx)
            .await
            .unwrap_err();
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref(),
        Some("42809")
    );
    tx.rollback().await.unwrap();
    pool.close().await;
}
