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
use sha2::Digest;
use sqlx::{postgres::PgConnectOptions, PgConnection, PgPool, Row};
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
    sqlx::raw_sql("CREATE TABLE canon.test_quarantine(receipt_id bigint PRIMARY KEY); ALTER TABLE canon.test_quarantine OWNER TO brain_migrate; CREATE OR REPLACE VIEW serve.published_receipt AS SELECT r.id,r.raw_object_id,r.ingest_run_id,r.extractor,r.locator_kind,r.locator,r.value_sha256,o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256 FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded' JOIN raw.object o ON o.id=r.raw_object_id WHERE NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id) AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection rejected WHERE rejected.raw_object_id=r.raw_object_id) AND ((NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine WHERE cause='audited') AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection)) OR serve.receipt_is_published(r.id)) AND NOT EXISTS(SELECT 1 FROM canon.test_quarantine q WHERE q.receipt_id=r.id)")
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
    store.check_gamefacts_schema().await.unwrap();
    let before: Vec<(i32, Vec<u8>)> = sqlx::query_as(versions).fetch_all(&pool).await.unwrap();
    let docs_checksum = &before.iter().find(|(version, _)| *version == 5).unwrap().1;
    let rendered = brain_storage::gamefacts::limits::render_sql(include_str!(
        "../../../../scripts/migrations/2026-10-13-brain-gamefacts-p5-docs-v1.sql"
    ));
    assert_eq!(*docs_checksum, Sha256::digest(rendered.as_bytes()).to_vec());
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

async fn receipt(connection: &mut PgConnection, raw: i64, run: i64, section: bool) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'docs',$3,$4,$5) RETURNING id")
        .bind(raw).bind(run)
        .bind(if section { "byte_ranges" } else { "bytes" })
        .bind(if section { "[[0,4]]" } else { "0:4" })
        .bind(vec![1_u8;32]).fetch_one(connection).await.unwrap()
}

async fn fact(
    connection: &mut PgConnection,
    entity: i64,
    raw: i64,
    run: i64,
    key: &str,
    value: &str,
    section: bool,
) -> i64 {
    let receipt = receipt(connection, raw, run, section).await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_text,valid_from_run,receipt_id) VALUES($1,$2,$3,$4,$5)")
        .bind(entity).bind(key).bind(value).bind(run).bind(receipt).execute(connection).await.unwrap();
    receipt
}

struct DocumentContent<'a> {
    title: &'a str,
    lang: &'a str,
    heading_path: &'a [&'a str],
    body: [&'a str; 2],
}

impl Default for DocumentContent<'_> {
    fn default() -> Self {
        Self {
            title: "Baba erklärt",
            lang: "de",
            heading_path: &["Baba"],
            body: ["Baba bewegt ", "sich schnell."],
        }
    }
}

struct StagedDocument {
    page: i64,
    body_receipt: i64,
    identity_receipt: i64,
    run: i64,
}

async fn stage_document(
    connection: &mut PgConnection,
    path: &str,
    version: u16,
    prior: Option<i64>,
    complete: bool,
    content: DocumentContent<'_>,
) -> StagedDocument {
    let run: i64 = sqlx::query_scalar(
        "INSERT INTO ops.ingest_run(source,status) VALUES('deadlock-docs','failed') RETURNING id",
    )
    .fetch_one(&mut *connection)
    .await
    .unwrap();
    let raw: i64 = sqlx::query_scalar("INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES('deadlock-docs',$1,$2,$3,$4,$5,$6) RETURNING id")
        .bind(format!("{version:040x}")).bind(path).bind(sha2::Sha256::digest(format!("{path}:{version}").as_bytes()).to_vec()).bind(sha2::Sha256::digest(version.to_string().as_bytes()).to_vec()).bind(b"test".to_vec()).bind(run)
        .fetch_one(&mut *connection).await.unwrap();
    let identity = receipt(connection, raw, run, false).await;
    let entity = if let Some(entity) = prior {
        entity
    } else {
        sqlx::query_scalar(
            "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('doc_page',$1,$2) RETURNING id",
        )
        .bind(format!("doc_{version}"))
        .bind(identity)
        .fetch_one(&mut *connection)
        .await
        .unwrap()
    };
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(entity)
        .bind(identity)
        .execute(&mut *connection)
        .await
        .unwrap();
    for (key, value) in [
        ("doc_path", path),
        ("doc_title", content.title),
        ("doc_lang", content.lang),
    ] {
        fact(connection, entity, raw, run, key, value, false).await;
    }
    let count = receipt(connection, raw, run, false).await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'doc_section_count',1,$2,$3)")
        .bind(entity).bind(run).bind(count).execute(&mut *connection).await.unwrap();
    fact(connection,entity,raw,run,"doc_intro_meta",&serde_json::json!({"ordinal":0,"locator":"intro","heading_path":content.heading_path,"line_start":1,"line_end":3,"chunks":2}).to_string(),true).await;
    let body_receipt = fact(
        connection,
        entity,
        raw,
        run,
        "doc_intro_body_0",
        content.body[0],
        true,
    )
    .await;
    if complete {
        fact(
            connection,
            entity,
            raw,
            run,
            "doc_intro_body_1",
            content.body[1],
            true,
        )
        .await;
    }
    StagedDocument {
        page: entity,
        body_receipt,
        identity_receipt: identity,
        run,
    }
}

async fn publish(connection: &mut PgConnection, run: i64) {
    let unpublished: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE ingest_run_id=$1")
            .bind(run)
            .fetch_one(&mut *connection)
            .await
            .unwrap();
    assert_eq!(unpublished, 0);
    let applications: i64 = sqlx::query_scalar(
        "SELECT count(DISTINCT application_xid) FROM canon.receipt WHERE ingest_run_id=$1",
    )
    .bind(run)
    .fetch_one(&mut *connection)
    .await
    .unwrap();
    assert_eq!(applications, 1);
    let result = sqlx::query("UPDATE ops.ingest_run SET status='succeeded',finished=clock_timestamp() WHERE id=$1 AND status='failed'")
        .bind(run).execute(connection).await.unwrap();
    assert_eq!(result.rows_affected(), 1);
}

async fn document_with_content(
    pool: &PgPool,
    path: &str,
    version: u16,
    prior: Option<i64>,
    complete: bool,
    content: DocumentContent<'_>,
) -> (i64, i64) {
    let mut tx = pool.begin().await.unwrap();
    let staged = stage_document(&mut tx, path, version, prior, complete, content).await;
    publish(&mut tx, staged.run).await;
    tx.commit().await.unwrap();
    (staged.page, staged.body_receipt)
}

async fn document(
    pool: &PgPool,
    path: &str,
    version: u16,
    prior: Option<i64>,
    complete: bool,
) -> (i64, i64) {
    document_with_content(
        pool,
        path,
        version,
        prior,
        complete,
        DocumentContent::default(),
    )
    .await
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
    let canonical =
        brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(&evidence[0]).unwrap();
    assert_eq!(canonical.raw_receipt_id, published_receipt);
    assert_eq!(canonical.source, "deadlock-docs");
    assert_eq!(
        execution.result.evidence_ids,
        vec![evidence[0].evidence_id.clone()]
    );
    assert_eq!(
        execution.result.result["sections"][0]["citation"],
        evidence[0].evidence_id
    );
    assert!(execution.result.result["sections"][0]["url"]
        .as_str()
        .unwrap()
        .contains("/public/baba.html#L1-L3"));
    assert_eq!(
        evidence[0].logical_id,
        format!("receipt/{published_receipt}")
    );
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
    document(&pool, "public/baba.html", 1, None, true).await;
    document(&pool, "internal/baba.html", 2, None, true).await;
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let original = reader.retrieve(&query, &ctx).unwrap();
    let (internal_query, internal_ctx) = context(true);
    let adapter = pg_docs::InternalDocsReader::new(reader.clone());
    let internal =
        brain_contracts::RetrievalPort::retrieve(&adapter, &internal_query, &internal_ctx).unwrap();
    assert_eq!(original.len(), 1);
    assert_eq!(internal.len(), 1);
    pool.close().await;
    drop(pg);
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
        let pg = scratch_pg::ScratchPg::start();
        let pool = setup(&pg).await;
        let (page, _) = document_with_content(
            &pool,
            "public/baba.html",
            1,
            None,
            true,
            DocumentContent {
                lang: &lang,
                ..DocumentContent::default()
            },
        )
        .await;
        document_with_content(
            &pool,
            "internal/baba.html",
            2,
            None,
            true,
            DocumentContent {
                lang: &lang,
                ..DocumentContent::default()
            },
        )
        .await;
        let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
        let adapter = pg_docs::InternalDocsReader::new(reader.clone());
        let tools = pg_docs::DocsTools::new(None, reader.clone(), definition.clone()).unwrap();
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
        pool.close().await;
    }
    for lang in [
        String::new(),
        " \u{2003}\u{00a0}".into(),
        "de\n".into(),
        "de\u{0085}".into(),
        "de\u{009f}".into(),
    ] {
        let pg = scratch_pg::ScratchPg::start();
        let pool = setup(&pg).await;
        document_with_content(
            &pool,
            "public/baba.html",
            1,
            None,
            true,
            DocumentContent {
                lang: &lang,
                ..DocumentContent::default()
            },
        )
        .await;
        let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
        assert!(reader.retrieve(&query, &ctx).unwrap().is_empty());
        pool.close().await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn search_terms_can_span_title_headings_and_body_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document_with_content(
        &pool,
        "public/baba.html",
        1,
        None,
        true,
        DocumentContent {
            heading_path: &[],
            body: ["Early ", "Game"],
            ..DocumentContent::default()
        },
    )
    .await;
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
    document_with_content(
        &pool,
        "public/phrases.html",
        1,
        None,
        true,
        DocumentContent {
            title: "Document",
            heading_path: &[],
            body: ["alpha ", "omega und Häuser"],
            ..DocumentContent::default()
        },
    )
    .await;
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
        document_with_content(
            &pool,
            path,
            version,
            None,
            true,
            DocumentContent {
                title: "Document",
                heading_path: &[],
                body: [
                    "alpha ",
                    if version % 2 == 0 {
                        "omega alpha und Häuser"
                    } else {
                        "omega und Häuser"
                    },
                ],
                ..DocumentContent::default()
            },
        )
        .await;
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
                    .map(|item| {
                        if internal {
                            item.evidence_id.clone()
                        } else {
                            brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(
                                item,
                            )
                            .unwrap()
                            .raw_receipt_id
                            .to_string()
                        }
                    })
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
            assert_eq!(
                execution.result.evidence_ids,
                evidence
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect::<Vec<_>>(),
                "{text}"
            );
            for dependency in &execution.dependencies {
                assert_eq!(dependency.request.without_canonical_result(), request);
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
async fn reader_preserves_hidden_websearch_operator_boundaries() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let original = "alpha or² omega beta alpha";
    let compact = "alpha or² omega beta";
    for language in ["pg_catalog.simple", "pg_catalog.german"] {
        let result: (bool, bool, f32, f32) = sqlx::query_as(
            "SELECT to_tsvector($1::regconfig,$2) @@ websearch_to_tsquery($1::regconfig,$3),
                to_tsvector($1::regconfig,$2) @@ websearch_to_tsquery($1::regconfig,$4),
                ts_rank_cd(to_tsvector($1::regconfig,$2),websearch_to_tsquery($1::regconfig,$3)),
                ts_rank_cd(to_tsvector($1::regconfig,$2),websearch_to_tsquery($1::regconfig,$4))",
        )
        .bind(language)
        .bind("² omega beta")
        .bind(original)
        .bind(compact)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(result, (false, true, 0.0, 0.1), "{language}");
        for token in [
            "or2", "OR2", "or2x", "2or", "or123", "alpha2", "2alpha", "123", "1e2",
        ] {
            let text = format!("alpha {token} omega beta alpha");
            let plain: bool = sqlx::query_scalar(
                "SELECT websearch_to_tsquery($1::regconfig,$2)=plainto_tsquery($1::regconfig,$2)",
            )
            .bind(language)
            .bind(&text)
            .fetch_one(&pool)
            .await
            .unwrap();
            assert!(plain, "{language}: {text}");
        }
    }
    for (version, path, body) in [
        (1, "public/negative.html", "² omega beta"),
        (2, "public/positive.html", "alpha ² omega beta"),
        (3, "internal/negative.html", "² omega beta"),
        (4, "internal/positive.html", "alpha ² omega beta"),
    ] {
        document_with_content(
            &pool,
            path,
            version,
            None,
            true,
            DocumentContent {
                title: "Document",
                heading_path: &[],
                body: [body, " "],
                ..DocumentContent::default()
            },
        )
        .await;
    }
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let definition = ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte.".into(),
        input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
    };
    for internal in [false, true] {
        let (mut query, ctx) = context(internal);
        query.text = original.into();
        let rows = sqlx::query(include_str!("../src/pg_docs/search.sql"))
            .bind(if internal { "internal" } else { "public" })
            .bind(original)
            .bind(None::<i64>)
            .bind(6_i64)
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        let expected = vec![rows[0].get::<i64, _>("receipt_id").to_string()];
        let evidence = reader.retrieve(&query, &ctx).unwrap();
        assert_eq!(
            evidence
                .iter()
                .map(|item| {
                    if internal {
                        item.evidence_id.clone()
                    } else {
                        brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(item)
                            .unwrap()
                            .raw_receipt_id
                            .to_string()
                    }
                })
                .collect::<Vec<_>>(),
            expected
        );
        reader
            .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
            .unwrap();
        let request = ToolCall {
            id: "boundaries".into(),
            name: ToolName::SearchDocs,
            arguments: serde_json::json!({"query":original}),
        }
        .validate(std::slice::from_ref(&definition))
        .unwrap();
        let execution = reader.search(&query, &ctx, "boundaries", &request).unwrap();
        assert_eq!(
            execution.result.evidence_ids,
            evidence
                .iter()
                .map(|item| item.evidence_id.clone())
                .collect::<Vec<_>>()
        );
        assert_eq!(execution.dependencies.len(), 1);
        assert_eq!(
            execution.dependencies[0].request.without_canonical_result(),
            request
        );
        reader
            .validate_dependency(
                &query,
                &ctx,
                &execution.dependencies[0],
                ToolValidationPurpose::Cache,
            )
            .unwrap();
        let mut compact_query = query.clone();
        compact_query.text = compact.into();
        let broadened = reader.retrieve(&compact_query, &ctx).unwrap();
        assert_eq!(broadened.len(), 2);
        assert!(reader
            .validate_evidence(&query, &ctx, &broadened, ToolValidationPurpose::Cache)
            .is_err());
        let broadened_dependency = ToolEvidenceDependency {
            request,
            game_context: None,
            evidence: broadened,
        };
        assert!(reader
            .validate_dependency(
                &query,
                &ctx,
                &broadened_dependency,
                ToolValidationPurpose::Cache
            )
            .is_err());
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
    let mut tx = pool.begin().await.unwrap();
    let staged = stage_document(
        &mut tx,
        "public/baba.html",
        2,
        Some(page),
        true,
        DocumentContent::default(),
    )
    .await;
    let receipt = staged.identity_receipt;
    let game: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','baba',$1) RETURNING id",
    )
    .bind(receipt)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(game)
        .bind(receipt)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) VALUES($1,'baba','de',$2)",
    )
    .bind(game)
    .bind(receipt)
    .execute(&mut *tx)
    .await
    .unwrap();
    publish(&mut tx, staged.run).await;
    tx.commit().await.unwrap();
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
    let (_, current_body_receipt) = document_with_content(
        &pool,
        "public/baba.html",
        2,
        Some(page),
        true,
        DocumentContent {
            body: ["Baba bleibt ", "sich schnell."],
            ..DocumentContent::default()
        },
    )
    .await;
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
    let mut probe = pool.begin().await.unwrap();
    sqlx::raw_sql(
        "SET LOCAL enable_seqscan=off; SET LOCAL jit=off; DROP INDEX canon.fact_patch_reference",
    )
    .execute(&mut *probe)
    .await
    .unwrap();
    let canonical_plan: serde_json::Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE,BUFFERS,FORMAT JSON) {canonical_sql}"
    ))
    .fetch_one(&mut *probe)
    .await
    .unwrap();
    probe.rollback().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql("SET LOCAL enable_seqscan=off; SET LOCAL jit=off")
        .execute(&mut *tx)
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
            "enable_seqscan":"off","jit":"off","canonical_without_index":"fact_patch_reference","canonical_sql":canonical_sql,
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

struct PublicAlias;
impl brain_contracts::RetrievalPort for PublicAlias {
    fn retrieve(
        &self,
        _: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<brain_contracts::Evidence>, brain_contracts::PortError> {
        assert!(context.principal.scopes.contains("docs.public"));
        Ok(Vec::new())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn operator_socket_reaches_only_internal_docs_without_egress_and_keeps_alias_mode() {
    use brain_contracts::{RequestDeadline, RetrievalPort};
    use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    use std::time::Duration;
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    document(&pool, "public/baba.html", 1, None, true).await;
    document(&pool, "internal/baba.html", 2, None, true).await;
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let retrieval = pg_docs::OperatorDocsRetrieval::new(PublicAlias, Some(reader.clone()));
    let (mut query, ctx) = context(true);
    query
        .requested_scopes
        .insert("second_brain.internal".into());
    let evidence = retrieval.retrieve(&query, &ctx).unwrap();
    assert_eq!(evidence.len(), 1);
    assert_eq!(
        evidence[0].visibility,
        brain_contracts::SourceVisibility::Internal
    );
    assert!(retrieval
        .validate_evidence(&query, &ctx, &evidence, true)
        .is_err());
    assert!(retrieval
        .validate_publication(&query, &ctx, &evidence)
        .is_err());
    let tools = pg_docs::DocsTools::new(
        None,
        reader,
        ToolDefinition {
            name: ToolName::SearchDocs,
            description: "Dokumentensuche".into(),
            input_schema: brain_contracts::tools::SearchDocsRequest::input_schema(),
        },
    )
    .unwrap();
    assert!(
        brain_contracts::ToolExecutionPort::definitions(&tools, &query, &ctx, None)
            .unwrap()
            .is_empty()
    );
    let policy = || {
        PolicyEngine::new(CredentialRegistry::new(vec![
            AuthGrant::from_secret(
                "internal-test-token",
                "second-brain",
                "internal",
                BTreeSet::from(["second_brain.internal".into()]),
                BTreeSet::new(),
            ),
            AuthGrant::from_secret(
                "public-docs-token",
                "docs-client",
                "docs",
                BTreeSet::from(["docs.public".into()]),
                BTreeSet::from(["public".into()]),
            ),
            AuthGrant::from_secret(
                "egress-test-token",
                "second-brain",
                "internal",
                BTreeSet::from(["second_brain.internal".into()]),
                BTreeSet::from(["public".into()]),
            ),
        ]))
    };
    let alias = brain_api::internal::InternalApiService::new(
        policy(),
        pg_docs::OperatorDocsRetrieval::new(PublicAlias, None),
        "test".into(),
        5000,
        Budget::default(),
    )
    .with_public_docs_release("test".into());
    assert_eq!(
        alias
            .handle_query(
                Some("Bearer internal-test-token"),
                &serde_json::to_vec(&query).unwrap(),
                RequestDeadline::after(Duration::from_secs(5))
            )
            .status,
        200
    );
    let service = brain_api::internal::InternalApiService::new(
        policy(),
        retrieval,
        "test".into(),
        5000,
        Budget::default(),
    );
    let directory = tempfile::tempdir().unwrap();
    std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let path = directory.path().join("operator.sock");
    let listener = tokio::net::UnixListener::bind(&path).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let owner = std::fs::metadata(&path).unwrap().uid();
    let server =
        tokio::spawn(
            async move { axum::serve(listener, brain_api::internal::router(service)).await },
        );
    let client = brain_client::AsyncInternalBrainClient::new(
        &path,
        owner,
        "internal-test-token",
        Duration::from_secs(5),
    )
    .unwrap();
    let answer = client.query(&query).await.unwrap();
    assert_eq!(answer.status, brain_client::InternalStatus::Answered);
    assert_eq!(answer.excerpts.len(), 1);
    for token in ["public-docs-token", "egress-test-token"] {
        let client = brain_client::AsyncInternalBrainClient::new(
            &path,
            owner,
            token,
            Duration::from_secs(5),
        )
        .unwrap();
        assert!(
            matches!(client.query(&query).await,Err(brain_client::ClientError::HttpStatus{status,..}) if status==reqwest::StatusCode::FORBIDDEN)
        );
    }
    server.abort();
    let _ = server.await;
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn owned_writer_updates_only_docs_and_never_grants_projection_access() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let mut tx = pool.begin().await.unwrap();
    let staged = stage_document(
        &mut tx,
        "public/writer.html",
        1,
        None,
        true,
        DocumentContent::default(),
    )
    .await;
    let raw: i64 = sqlx::query_scalar("SELECT raw_object_id FROM canon.receipt WHERE id=$1")
        .bind(staged.body_receipt)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let hero: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','writer_hero',$1) RETURNING id").bind(staged.identity_receipt).fetch_one(&mut *tx).await.unwrap();
    for (key, value) in [
        ("doc_title", "Hero"),
        (
            "doc_intro_meta",
            r#"{"ordinal":0,"locator":"intro","heading_path":[],"line_start":1,"line_end":3,"chunks":1}"#,
        ),
        ("doc_intro_body_0", "not a document"),
    ] {
        fact(&mut tx, hero, raw, staged.run, key, value, true).await;
    }
    let hero_rows: i64 =
        sqlx::query_scalar("SELECT count(*) FROM canon.doc_search WHERE entity_id=$1")
            .bind(hero)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    assert_eq!(hero_rows, 0);
    sqlx::query(
        "SELECT canon.write_run_fact($1,'doc_intro_body_0',NULL,'Baba ersetzt ',NULL,$2,$3)",
    )
    .bind(staged.page)
    .bind(staged.run)
    .bind(staged.body_receipt)
    .execute(&mut *tx)
    .await
    .unwrap();
    let body: String = sqlx::query_scalar("SELECT body FROM canon.doc_search WHERE entity_id=$1")
        .bind(staged.page)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(body, "Baba ersetzt sich schnell.");
    for role in ["brain_tool", "brain_service", "brain_ingest"] {
        let permitted: bool = sqlx::query_scalar(
            "SELECT has_table_privilege($1,'canon.doc_search','SELECT,INSERT,UPDATE,DELETE')",
        )
        .bind(role)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        assert!(!permitted, "{role}");
    }
    publish(&mut tx, staged.run).await;
    tx.commit().await.unwrap();
    let body: String = sqlx::query_scalar("SELECT body FROM serve.doc_section WHERE page_id=$1")
        .bind(staged.page)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(body, "Baba ersetzt sich schnell.");
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn real_search_uses_owned_gin_naturally_within_statement_budget() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let filler = "Lane Gegner Position Heilung Bewegung Rotation Farm Team ".repeat(80);
    for version in 1..=400 {
        let body = if version == 400 {
            format!("{filler} Zielsprungnadel")
        } else {
            filler.clone()
        };
        document_with_content(
            &pool,
            &format!("public/guide{version}.html"),
            version,
            None,
            true,
            DocumentContent {
                body: [&body, " Endabschnitt"],
                ..DocumentContent::default()
            },
        )
        .await;
    }
    sqlx::raw_sql("VACUUM (ANALYZE) canon.doc_search")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("ANALYZE canon.fact; ANALYZE canon.receipt; ANALYZE canon.entity; ANALYZE raw.object; ANALYZE ops.ingest_run").execute(&pool).await.unwrap();
    let search = include_str!("../src/pg_docs/search.sql");
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql(pg_docs::READ_TRANSACTION)
        .execute(&mut *tx)
        .await
        .unwrap();
    let plan: serde_json::Value =
        sqlx::query_scalar(&format!("EXPLAIN (ANALYZE,BUFFERS,FORMAT JSON) {search}"))
            .bind("public")
            .bind("Zielsprungnadel")
            .bind(None::<i64>)
            .bind(5_i64)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let indexed = plan_matches(&plan, |node| {
        if node["Relation Name"] == "doc_search" {
            eprintln!(
                "DOCS_SCAN_PROOF {}",
                serde_json::json!({
                    "node":node["Node Type"],"index":node["Index Name"],
                    "filter":node["Filter"],"index_cond":node["Index Cond"],
                    "rows":node["Actual Rows"],"cost":node["Total Cost"]
                })
            );
        }
        matches!(
            node["Index Name"].as_str(),
            Some("doc_search_simple_gin" | "doc_search_german_gin")
        ) && node["Index Cond"]
            .as_str()
            .is_some_and(|condition| condition.contains("@@"))
    });
    assert!(indexed, "execution_ms={}", plan[0]["Execution Time"]);
    assert_eq!(plan[0]["Plan"]["Actual Rows"], 1);
    assert!(plan[0]["Execution Time"].as_f64().unwrap() < 500.0);
    eprintln!(
        "DOCS_INDEX_PROOF documents=400 execution_ms={} planning_ms={}",
        plan[0]["Execution Time"], plan[0]["Planning Time"]
    );
    let broad: serde_json::Value =
        sqlx::query_scalar(&format!("EXPLAIN (ANALYZE,FORMAT JSON) {search}"))
            .bind("public")
            .bind("Lane Rotation")
            .bind(None::<i64>)
            .bind(5_i64)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    assert_eq!(broad[0]["Plan"]["Actual Rows"], 5);
    assert!(
        broad[0]["Execution Time"].as_f64().unwrap() < 500.0,
        "broad execution_ms={}",
        broad[0]["Execution Time"]
    );
    tx.commit().await.unwrap();
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn docs_ingestion_keeps_poison_isolated_and_unchanged_replay_has_zero_growth() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    sqlx::raw_sql("ALTER ROLE brain_ingest LOGIN")
        .execute(&pool)
        .await
        .unwrap();
    let writer = PgPool::connect_with(
        PgConnectOptions::new_without_pgpass()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username("brain_ingest")
            .database("postgres"),
    )
    .await
    .unwrap();
    let store = brain_job::GameFactsStore::new(writer.clone());
    let objects = || {
        [
        ("public/poison.html","<html><title>Baba</title><body>Kein Hauptbereich</body></html>"),
        ("public/baba.html","<!doctype html>\n<html lang=\"de\"><head><title>Baba</title></head><body><main><h1>Baba</h1><p>Baba bewegt sich früh auf der Lane.</p></main></body></html>"),
        ("internal/baba.html","<!doctype html>\n<html lang=\"de\"><head><title>Baba intern</title></head><body><main><h1>Baba</h1><p>Baba bleibt ein interner Hinweis.</p></main></body></html>"),
    ].into_iter().map(|(path,html)| Ok(brain_job::InputObject{revision:"a".repeat(40),path:path.into(),bytes:html.as_bytes().to_vec()}))
    };
    let extractors: &[&dyn brain_job::Extractor] = &[&brain_job::docs::DocsExtractor];
    let first = brain_job::ingest(&store, "deadlock-docs", objects(), extractors)
        .await
        .unwrap();
    assert_eq!(
        (
            first.counts.accepted,
            first.counts.rejected,
            first.counts.ignored
        ),
        (2, 1, 0)
    );
    let growth = "SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.entity),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM canon.doc_search)";
    let before: (i64, i64, i64, i64, i64) = sqlx::query_as(growth).fetch_one(&pool).await.unwrap();
    assert_eq!(before.0, 3);
    assert_eq!(before.1, 2);
    assert_eq!(before.4, 2);
    let replay = brain_job::ingest(&store, "deadlock-docs", objects(), extractors)
        .await
        .unwrap();
    assert_eq!(
        (
            replay.counts.accepted,
            replay.counts.rejected,
            replay.counts.ignored
        ),
        (0, 1, 2)
    );
    let after: (i64, i64, i64, i64, i64) = sqlx::query_as(growth).fetch_one(&pool).await.unwrap();
    assert_eq!(before, after);
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let evidence = reader.retrieve(&query, &ctx).unwrap();
    assert_eq!(evidence.len(), 1);
    let receipt =
        brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(&evidence[0]).unwrap();
    let published: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM serve.published_receipt WHERE id=$1 AND path='public/baba.html')").bind(receipt.raw_receipt_id).fetch_one(&pool).await.unwrap();
    assert!(published);
    writer.close().await;
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn p9_audit_quarantine_hides_indexed_docs_and_never_revives_older_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let (page, _) = document(&pool, "public/audited.html", 1, None, true).await;
    let (_, bad_body) = document(&pool, "public/audited.html", 2, Some(page), true).await;
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let original = reader.retrieve(&query, &ctx).unwrap();
    assert_eq!(original.len(), 1);
    let audit: i64 = sqlx::query_scalar(
        "INSERT INTO ops.receipt_audit_run(sampled,rejected_receipts) VALUES(1,1) RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome) VALUES($1,$2,'locator_not_found')").bind(audit).bind(bad_body).execute(&pool).await.unwrap();
    let owned_rows: i64 =
        sqlx::query_scalar("SELECT count(*) FROM canon.doc_search WHERE entity_id=$1")
            .bind(page)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(owned_rows, 2);
    assert!(reader.retrieve(&query, &ctx).unwrap().is_empty());
    assert!(reader
        .validate_evidence(&query, &ctx, &original, ToolValidationPurpose::Cache)
        .is_err());
    let sections: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.doc_section WHERE page_id=$1")
            .bind(page)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(sections, 0);
    pool.close().await;
}

const AUDIT_FILLER: &str = include_str!("fixtures/audit_filler.sql");

#[tokio::test(flavor = "multi_thread")]
async fn audited_publication_keeps_multi_page_docs_search_within_statement_budget() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    let mut public = Vec::new();
    for version in 1..=400 {
        let path = format!("public/guide{version}.html");
        public.push(document(&pool, &path, version, None, true).await);
    }
    let (internal, _) = document(&pool, "internal/geheim.html", 401, None, true).await;
    let (shadowed_page, _) = document(&pool, "public/shadowed.html", 402, None, true).await;
    let (_, shadowing_body) = document(
        &pool,
        "public/shadowed.html",
        403,
        Some(shadowed_page),
        true,
    )
    .await;
    sqlx::raw_sql(AUDIT_FILLER).execute(&pool).await.unwrap();
    let audit: i64 = sqlx::query_scalar(
        "INSERT INTO ops.receipt_audit_run(sampled,rejected_receipts,rejected_objects) VALUES(2,2,1) RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let (audited_page, audited_body) = public[0];
    let (rejected_page, rejected_body) = public[1];
    for body in [audited_body, shadowing_body] {
        sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome) VALUES($1,$2,'locator_not_found')")
            .bind(audit).bind(body).execute(&pool).await.unwrap();
    }
    sqlx::query("INSERT INTO ops.receipt_audit_rejection(run_id,raw_object_id,code,detail) SELECT $1,raw_object_id,'hash_mismatch','test' FROM canon.receipt WHERE id=$2")
        .bind(audit).bind(rejected_body).execute(&pool).await.unwrap();
    let boundary: (i64, i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM ops.receipt_audit),(SELECT count(*) FROM ops.receipt_audit_rejection),(SELECT count(*) FROM canon.receipt_quarantine WHERE cause='audited'),(SELECT count(*) FROM canon.receipt_quarantine WHERE cause='shadowed')")
        .fetch_one(&pool).await.unwrap();
    assert!(boundary.0 >= 5000 && boundary.1 >= 1000 && boundary.2 >= 1000 && boundary.3 >= 1);
    sqlx::raw_sql("VACUUM (ANALYZE) canon.doc_search")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("ANALYZE canon.fact; ANALYZE canon.receipt; ANALYZE canon.entity; ANALYZE canon.entity_identity; ANALYZE raw.object; ANALYZE ops.ingest_run; ANALYZE canon.receipt_quarantine; ANALYZE ops.receipt_audit_rejection")
        .execute(&pool).await.unwrap();
    let diverging: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt r WHERE serve.receipt_is_published(r.id)<>EXISTS(SELECT 1 FROM serve.published_receipt p WHERE p.id=r.id)")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(diverging, 0);
    let pages: Vec<i64> = sqlx::query_scalar("SELECT page_id FROM serve.doc_page ORDER BY page_id")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert!(!pages.contains(&rejected_page));
    assert!(pages.contains(&internal) && pages.contains(&public[2].0));
    let hidden_sections: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.doc_section WHERE page_id=ANY($1)")
            .bind(vec![audited_page, rejected_page, shadowed_page])
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(hidden_sections, 0);
    let search = include_str!("../src/pg_docs/search.sql");
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql(pg_docs::READ_TRANSACTION)
        .execute(&mut *tx)
        .await
        .unwrap();
    let plan: serde_json::Value =
        sqlx::query_scalar(&format!("EXPLAIN (ANALYZE,FORMAT JSON) {search}"))
            .bind("public")
            .bind("Baba bewegt")
            .bind(None::<i64>)
            .bind(5_i64)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let execution = plan[0]["Execution Time"].as_f64().unwrap();
    eprintln!(
        "DOCS_AUDITED_PUBLICATION_PROOF audit_rows={} rejections={} audited={} shadowed={} sections=5 execution_ms={execution} planning_ms={}",
        boundary.0, boundary.1, boundary.2, boundary.3, plan[0]["Planning Time"]
    );
    assert_eq!(plan[0]["Plan"]["Actual Rows"], 5);
    assert!(execution < 500.0, "execution_ms={execution}");
    tx.commit().await.unwrap();
    let rows = sqlx::query(search)
        .bind("public")
        .bind("Baba bewegt")
        .bind(None::<i64>)
        .bind(500_i64)
        .fetch_all(&pool)
        .await
        .unwrap();
    let paths: BTreeSet<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("path"))
        .collect();
    assert!(paths.len() >= 2);
    for hidden in [
        "public/guide1.html",
        "public/guide2.html",
        "public/shadowed.html",
        "internal/geheim.html",
    ] {
        assert!(!paths.contains(hidden), "{hidden}");
    }
    let reader = DocsReader::new(pool.clone(), tokio::runtime::Handle::current());
    let (query, ctx) = context(false);
    let evidence = reader.retrieve(&query, &ctx).unwrap();
    assert!(evidence.len() >= 2);
    reader
        .validate_evidence(&query, &ctx, &evidence, ToolValidationPurpose::Cache)
        .unwrap();
    for item in &evidence {
        let receipt =
            brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(item).unwrap();
        assert!(receipt.path.starts_with("public/guide"));
        assert!(!["public/guide1.html", "public/guide2.html"].contains(&receipt.path.as_str()));
    }
    pool.close().await;
}

async fn search_paths(pool: &PgPool, scope: &str, query: &str, entity: Option<i64>) -> Vec<String> {
    sqlx::query(include_str!("../src/pg_docs/search.sql"))
        .bind(scope)
        .bind(query)
        .bind(entity)
        .bind(6_i64)
        .fetch_all(pool)
        .await
        .unwrap()
        .iter()
        .map(|row| row.get::<String, _>("path"))
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn search_falls_back_to_partial_matches_and_bare_commands() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    for (version, path, title, body) in [
        (
            1,
            "public/steam.html",
            "Steam verknüpfen",
            ["Nutze /checkrank ", "im Kanal."],
        ),
        (
            2,
            "public/discord.html",
            "Discord Server",
            ["Rollen ", "und Kanäle."],
        ),
        (
            3,
            "internal/steam.html",
            "Steam verknüpfen Discord",
            ["Nur ", "Team."],
        ),
    ] {
        document_with_content(
            &pool,
            path,
            version,
            None,
            true,
            DocumentContent {
                title,
                heading_path: &[],
                body,
                ..DocumentContent::default()
            },
        )
        .await;
    }
    for (query, expected) in [
        ("Steam Discord verknüpfen Rang", vec!["public/steam.html"]),
        ("checkrank", vec!["public/steam.html"]),
        ("/checkrank", vec!["public/steam.html"]),
        ("Steam Kanal", vec!["public/steam.html"]),
        ("Discord Steam -verknüpfen", vec![]),
        ("Rang Team", vec![]),
        ("Steam-Kanal verknüpfen Rang", vec!["public/steam.html"]),
        ("Discord-Server Rollen", vec!["public/discord.html"]),
        ("Steam -Kanal Rang", vec![]),
    ] {
        assert_eq!(
            search_paths(&pool, "public", query, None).await,
            expected,
            "{query}"
        );
    }
    assert_eq!(
        search_paths(&pool, "internal", "Steam Discord", None).await,
        vec!["internal/steam.html"]
    );
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn search_keeps_published_runs_and_filters_entities_before_the_limit() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = setup(&pg).await;
    for version in 1..=13 {
        document_with_content(
            &pool,
            &format!("public/guide-{version}.html"),
            version,
            None,
            true,
            DocumentContent {
                title: "Guide Guide",
                heading_path: &[],
                body: ["Guide ", "Guide Guide."],
                ..DocumentContent::default()
            },
        )
        .await;
    }
    let mut tx = pool.begin().await.unwrap();
    let staged = stage_document(
        &mut tx,
        "public/baba.html",
        14,
        None,
        true,
        DocumentContent {
            title: "Baba",
            heading_path: &[],
            body: ["Guide ", "für Baba."],
            ..DocumentContent::default()
        },
    )
    .await;
    let game: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('hero','baba',$1) RETURNING id",
    )
    .bind(staged.identity_receipt)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(game)
        .bind(staged.identity_receipt)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) VALUES($1,'baba','de',$2)",
    )
    .bind(game)
    .bind(staged.identity_receipt)
    .execute(&mut *tx)
    .await
    .unwrap();
    publish(&mut tx, staged.run).await;
    tx.commit().await.unwrap();
    assert_eq!(
        search_paths(&pool, "public", "Guide", Some(game)).await,
        vec!["public/baba.html"]
    );
    let counts: Vec<i64> = sqlx::query(include_str!("../src/pg_docs/search.sql"))
        .bind("public")
        .bind("Guide")
        .bind(None::<i64>)
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|row| row.get::<i64, _>("bounded_matches"))
        .collect();
    assert_eq!(counts, vec![7; 6]);
    let mut tx = pool.begin().await.unwrap();
    stage_document(
        &mut tx,
        "public/baba.html",
        15,
        Some(staged.page),
        true,
        DocumentContent {
            title: "Baba",
            heading_path: &[],
            body: ["Entwurf ", "ohne Freigabe."],
            ..DocumentContent::default()
        },
    )
    .await;
    tx.commit().await.unwrap();
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.doc_search WHERE entity_id=$1")
        .bind(staged.page)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows, 2);
    let runs: Vec<i64> = sqlx::query(include_str!("../src/pg_docs/search.sql"))
        .bind("public")
        .bind("Baba")
        .bind(None::<i64>)
        .bind(6_i64)
        .fetch_all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|row| row.get::<i64, _>("valid_from_run"))
        .collect();
    assert_eq!(runs, vec![staged.run]);
    assert!(search_paths(&pool, "public", "Entwurf", None)
        .await
        .is_empty());
    pool.close().await;
}
