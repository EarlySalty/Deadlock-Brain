use brain_contracts::{
    external::{ExternalSourceIr, Provenance, SourceRevision, Validation},
    source::Versioned,
    tools::{GameContextResolver, ToolLanguage},
    value::{Observed, UnknownReason},
    AuthorizedContext, PortError, Query, RequestDeadline, SourceVisibility,
};
use brain_storage::{
    asset_mirror::{mirrored_asset_key, mirrored_asset_languages, MIRRORED_ASSET_KINDS},
    entity_profile::MirroredGameContextReader,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[path = "support/scratch_pg.rs"]
mod scratch_pg;

fn authorized_context() -> (AuthorizedContext, Arc<Mutex<Instant>>) {
    let clock = Arc::new(Mutex::new(Instant::now()));
    let now = clock.clone();
    let deadline =
        RequestDeadline::after_with_clock(Duration::from_secs(3600), move || *now.lock().unwrap());
    let mut context: AuthorizedContext = serde_json::from_value(json!({
        "principal":{"actor_id":"fixture","channel":"test","scopes":[],"provider_egress":[]},
        "conversation_id":"fixture","knowledge_release":"fixture","deadline_ms":3600000,
        "budget":{"max_network_rounds":4,"max_input_tokens":12000,"max_output_tokens":2000,"max_cost_micros":50000}
    })).unwrap();
    context.request_deadline = Some(deadline);
    (context, clock)
}

fn query() -> Query {
    serde_json::from_value(json!({"request_id":"fixture", "conversation_id":"fixture", "text":"Öffentliche Spielwerte"})).unwrap()
}

async fn pool(pg: &scratch_pg::ScratchPg) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(pg.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let (address, user, directory): (Option<String>, String, String) = sqlx::query_as(
        "SELECT inet_server_addr()::text,current_user::text,current_setting('data_directory')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(address.is_none());
    assert_eq!(user, "brain_core_test");
    assert_eq!(std::path::Path::new(&directory), pg.directory.join("data"));
    sqlx::raw_sql("CREATE SCHEMA brain;
        CREATE TABLE brain.source_runs(id bigserial PRIMARY KEY,source text,status text,summary jsonb,
            started_at timestamptz DEFAULT '2026-10-07T00:00:00Z',finished_at timestamptz DEFAULT '2026-10-07T00:01:00Z');
        CREATE TABLE brain.source_documents(id bigserial PRIMARY KEY,source text,metadata jsonb,
            url text,content_hash text NOT NULL,fetched_at timestamptz DEFAULT '2026-10-07T00:00:30Z',fixture_raw bytea);")
        .execute(&pool).await.unwrap();
    pool
}

// Öffentliche Testformen, keine Produktionsoriginale. Der Receiptleser und Postgres
// laufen wirklich; diese Probe ersetzt keinen Importer- oder Produktionsnachweis.
async fn document(pool: &PgPool, payload: Value, url: String, adapter: Value) -> (i64, String) {
    let raw = serde_json::to_vec(&payload).unwrap();
    let hash = format!("{:x}", Sha256::digest(&raw));
    let provenance = Provenance {
        source: "deadlock_assets_api".into(),
        locator: url.clone(),
        source_revision: SourceRevision::Http {
            body_sha256: hash.clone(),
            etag: None,
            last_modified: None,
        },
        parser_revision: "fixture-parser-v1".into(),
        parser_family: "fixture-assets".into(),
        raw_sha256: hash.clone(),
        schema_sha256: None,
        observed_at: 100,
        origin_artifacts: BTreeSet::new(),
        derivation_family: None,
        publication_authorized: false,
        provider_egress_authorized: false,
    };
    let validation = Validation::Validated {
        extra_fields: Vec::new(),
    };
    let ir = ExternalSourceIr {
        payload: Observed::known(payload),
        provenance: provenance.clone(),
        validation: validation.clone(),
        transport: json!({"status":200}),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        field_provenance: BTreeMap::new(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        license: Observed::unknown(UnknownReason::NotPresent),
    };
    ir.origin_artifact().validate().unwrap();
    let metadata = json!({"source_ir_version":1,"adapter":adapter,"provenance":provenance,
        "validation":validation,"contract":Versioned::new(ir)});
    let id: i64 = sqlx::query_scalar("INSERT INTO brain.source_documents(source,metadata,url,content_hash,fixture_raw) VALUES('deadlock_assets_api',$1,$2,$3,$4) RETURNING id")
        .bind(metadata).bind(url).bind(&hash).bind(&raw).fetch_one(pool).await.unwrap();
    (id, hash)
}

async fn seed(pool: &PgPool, marker: &str) -> i64 {
    let (manifest, manifest_hash) = document(
        pool,
        json!({"client_version":6759}),
        "https://example.org/fixture/manifest".into(),
        json!({"role":"client_manifest","client_version":6759}),
    )
    .await;
    let mut endpoints = serde_json::Map::new();
    for kind in MIRRORED_ASSET_KINDS {
        for language in mirrored_asset_languages(kind).unwrap() {
            let language = (!language.is_empty()).then_some(*language);
            let key = mirrored_asset_key(kind, language).unwrap();
            let payload = if *kind == "generic_data" {
                json!({"observation":marker,"zero":0})
            } else {
                json!([{"id":7,"name":"Prüfdaten","observation":marker}])
            };
            let url = format!("https://example.org/fixture/{key}?client_version=6759");
            let (id, hash) = document(
                pool,
                payload,
                url,
                json!({"kind":kind,"language":language,"client_version":6759}),
            )
            .await;
            endpoints.insert(key, json!({"source_document_id":id,"raw_sha256":hash}));
        }
    }
    assert_eq!(endpoints.len(), 13);
    sqlx::query_scalar("INSERT INTO brain.source_runs(source,status,summary) VALUES('assets','ok',$1) RETURNING id")
        .bind(json!({"client_version":6759,"manifest_document_id":manifest,"manifest_raw_sha256":manifest_hash,
            "parser_revision":"fixture-parser-v1","mirrored_at":100,"checked_at":100,
            "endpoints":endpoints,"mirror_complete":true,"reused_local_mirror":false}))
        .fetch_one(pool).await.unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_receipt_reader_pins_one_run_and_rejects_drift() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = pool(&pg).await;
    let first_run = seed(&pool, "first").await;
    let reader = MirroredGameContextReader::new(
        pool.clone(),
        tokio::runtime::Handle::current(),
        ToolLanguage::German,
    )
    .unwrap();
    let (context, _) = authorized_context();
    let query = query();
    let pin = reader.resolve(&query, &context).unwrap().unwrap();
    let bundle = reader.read_pinned(&context, &pin).unwrap();
    assert_eq!(pin.language, ToolLanguage::German);
    assert_eq!(pin.client_version, 6759);
    assert!(reader.validate(&query, &context, Some(&pin)).is_ok());
    for kind in MIRRORED_ASSET_KINDS {
        for language in mirrored_asset_languages(kind).unwrap() {
            let asset = bundle
                .asset(kind, (!language.is_empty()).then_some(*language))
                .unwrap();
            assert_eq!(asset.receipt.source_run_id, first_run);
            assert_eq!(asset.receipt.parser_revision, "fixture-parser-v1");
            assert!(!asset.receipt.endpoint.provenance.provider_egress_authorized);
            assert!(!asset.receipt.endpoint.provenance.publication_authorized);
            let raw: Vec<u8> =
                sqlx::query_scalar("SELECT fixture_raw FROM brain.source_documents WHERE id=$1")
                    .bind(asset.receipt.endpoint.source_document_id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&raw)),
                asset.receipt.endpoint.raw_sha256
            );
            assert_eq!(
                serde_json::from_slice::<Value>(&raw).unwrap(),
                asset.payload
            );
        }
    }
    let mut foreign = pin.clone();
    foreign.language = ToolLanguage::English;
    assert!(reader.read_pinned(&context, &foreign).is_err());
    foreign = pin.clone();
    foreign.mechanic_revision = "not-a-receipt".into();
    assert!(reader.read_pinned(&context, &foreign).is_err());
    assert!(reader.validate(&query, &context, None).is_err());

    let second_run = seed(&pool, "second").await;
    let second_pin = reader.resolve(&query, &context).unwrap().unwrap();
    assert_ne!(pin, second_pin);
    assert!(reader.validate(&query, &context, Some(&pin)).is_err());
    assert_eq!(
        reader
            .read_pinned(&context, &pin)
            .unwrap()
            .asset("items", Some("english"))
            .unwrap()
            .receipt
            .source_run_id,
        first_run
    );
    assert_eq!(
        reader
            .read_pinned(&context, &second_pin)
            .unwrap()
            .asset("items", Some("english"))
            .unwrap()
            .receipt
            .source_run_id,
        second_run
    );

    // Eine Payloadänderung wird auch bei unverändertem Originalhash nicht als
    // dieselbe Bindung akzeptiert. Die Importerprüfung ist ein eigener Beweis.
    sqlx::query("UPDATE brain.source_documents SET metadata=jsonb_set(metadata,'{contract,data,payload,value,0,observation}','\"changed\"') WHERE id=(SELECT (summary->'endpoints'->'items/english'->>'source_document_id')::bigint FROM brain.source_runs WHERE id=$1)")
        .bind(second_run).execute(&pool).await.unwrap();
    assert!(reader.read_pinned(&context, &second_pin).is_err());
    assert!(reader
        .validate(&query, &context, Some(&second_pin))
        .is_err());
    let changed_pin = reader.resolve(&query, &context).unwrap().unwrap();
    sqlx::query("UPDATE brain.source_documents SET content_hash='broken' WHERE id=(SELECT (summary->'endpoints'->'modifiers'->>'source_document_id')::bigint FROM brain.source_runs WHERE id=$1)")
        .bind(second_run).execute(&pool).await.unwrap();
    assert!(reader.read_pinned(&context, &changed_pin).is_err());
    assert!(reader.resolve(&query, &context).is_err());
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn original_controlled_lifetime_is_required_before_any_read() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = pool(&pg).await;
    seed(&pool, "bound").await;
    let reader = MirroredGameContextReader::new(
        pool.clone(),
        tokio::runtime::Handle::current(),
        ToolLanguage::German,
    )
    .unwrap();
    let (context, clock) = authorized_context();
    let pin = reader.resolve(&query(), &context).unwrap().unwrap();
    let mut missing = context.clone();
    missing.request_deadline = None;
    assert!(matches!(
        reader.resolve(&query(), &missing),
        Err(PortError::Unavailable(_))
    ));
    let original = context.request_deadline.as_ref().unwrap();
    let clone = context.clone();
    original.cancel();
    assert_eq!(
        reader.resolve(&query(), &clone).unwrap_err(),
        PortError::BudgetExceeded
    );
    assert!(matches!(
        reader.read_pinned(&clone, &pin),
        Err(PortError::BudgetExceeded)
    ));
    assert_eq!(
        reader.validate(&query(), &clone, Some(&pin)).unwrap_err(),
        PortError::BudgetExceeded
    );
    let (expired, _) = authorized_context();
    // Diese Frist bekommt dieselbe kontrollierte Uhr, keine echte Wartezeit.
    let now = clock.clone();
    let mut expired = expired;
    expired.request_deadline = Some(RequestDeadline::after_with_clock(
        Duration::from_secs(1),
        move || *now.lock().unwrap(),
    ));
    *clock.lock().unwrap() += Duration::from_secs(2);
    assert_eq!(
        reader.resolve(&query(), &expired).unwrap_err(),
        PortError::BudgetExceeded
    );
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_finishes_the_caller_while_a_real_database_read_is_locked() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = pool(&pg).await;
    seed(&pool, "pending").await;
    let reader = MirroredGameContextReader::new(
        pool.clone(),
        tokio::runtime::Handle::current(),
        ToolLanguage::German,
    )
    .unwrap();
    let (context, _) = authorized_context();
    let cancellation = context.request_deadline.as_ref().unwrap().clone();
    let mut lock = pool.begin().await.unwrap();
    sqlx::raw_sql("LOCK TABLE brain.source_runs IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *lock)
        .await
        .unwrap();
    let pending = tokio::task::spawn_blocking(move || reader.resolve(&query(), &context));
    loop {
        let waiting: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=current_database() AND wait_event_type='Lock' AND query LIKE '%sr.summary%')")
            .fetch_one(&pool).await.unwrap();
        if waiting {
            break;
        }
        assert!(
            !pending.is_finished(),
            "Der echte Spiegelaufruf muss an der eigenen Test-Sperre warten"
        );
        tokio::task::yield_now().await;
    }
    cancellation.cancel();
    assert_eq!(
        pending.await.unwrap().unwrap_err(),
        PortError::BudgetExceeded
    );
    lock.rollback().await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn current_thread_runtime_is_rejected_without_creating_another_runtime() {
    let pool = PgPoolOptions::new().connect_lazy_with(
        PgConnectOptions::new_without_pgpass()
            .host("/nonexistent-mirror-fixture")
            .database("postgres"),
    );
    assert!(MirroredGameContextReader::new(
        pool,
        tokio::runtime::Handle::current(),
        ToolLanguage::German
    )
    .is_err());
}
