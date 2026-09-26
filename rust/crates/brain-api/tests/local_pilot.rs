#[path = "support/domain_fixture.rs"]
mod domain_fixture;
use axum::{
    body::Bytes,
    extract::State,
    http::{header, StatusCode},
    routing::post,
    Router,
};
use brain_client::BrainClient;
use brain_contracts::{
    AnswerProfile, AnswerStatus, Budget, CorpusRelease, DocumentStorePort, PublicAnswerResponse,
    Query, SnapshotReadPort, SourceBatch, SourceVisibility,
};
use brain_ingestion::FileConnector;
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_providers::{OpenAiCompatibleProvider, ProviderConfig};
use brain_storage::{LocalPgReader, PgStore};
use dbrain_retrieval::ReleaseRetriever;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

const PORT: u16 = 55439;
const USER: &str = "brain_core_test";
const PATCH: &str = "pilot-20260925";
const PUBLIC_TOKEN: &str = "pilot-public-token";
const INTERNAL_TOKEN: &str = "pilot-internal-token";

struct Env {
    socket: String,
    root: PathBuf,
    database: String,
    report: PathBuf,
}

fn env() -> Env {
    let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
    assert!(
        socket.ends_with("/.core-test-pg"),
        "refusing non-scratch socket"
    );
    let database = std::env::var("BRAIN_PILOT_DATABASE").expect("pilot database required");
    assert!(
        database.starts_with("pilot_"),
        "refusing non-pilot database"
    );
    Env {
        socket,
        root: std::env::var("BRAIN_PILOT_ROOT")
            .expect("pilot root required")
            .into(),
        database,
        report: std::env::var("BRAIN_PILOT_REPORT")
            .expect("report path required")
            .into(),
    }
}

async fn store(env: &Env) -> PgStore {
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    let options = PgConnectOptions::new()
        .host(&env.socket)
        .port(PORT)
        .username(USER)
        .database(&env.database);
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(address.is_none(), "pilot cluster must not use TCP");
    let store = PgStore::new(pool);
    store.migrate_core().await.unwrap();
    store
}

fn connectors(env: &Env) -> [FileConnector; 2] {
    [
        FileConnector::new(
            env.root.join("public"),
            "pilot-game-wiki",
            SourceVisibility::Public,
            BTreeSet::new(),
        ),
        FileConnector::new(
            env.root.join("internal"),
            "pilot-internal-docs",
            SourceVisibility::Private,
            BTreeSet::from(["docs.internal".into()]),
        ),
    ]
}

async fn ingest(store: &PgStore, env: &Env, owner: &str) -> (Vec<SourceBatch>, Vec<bool>) {
    let mut batches = Vec::new();
    let mut replayed = Vec::new();
    for connector in connectors(env) {
        let source = batch_source(&connector);
        let previous = store.checkpoint(&source).await.unwrap();
        let batch = connector.prepare_batch(previous.as_ref()).unwrap();
        let lease = store.claim(&source, owner, 30_000).await.unwrap();
        let receipt = connector.commit_batch(store, &batch, &lease).await.unwrap();
        replayed.push(receipt.replayed);
        batches.push(batch);
    }
    // Synthetic domain adapter fixture, explicitly separate from the approved
    // real documents. It follows the same transactional PostgreSQL release path.
    let previous = store.checkpoint(domain_fixture::SOURCE).await.unwrap();
    if let Some(checkpoint) = &previous {
        assert_eq!(checkpoint.configuration, "c6-synthetic-domain-v1");
    }
    let generation = previous
        .as_ref()
        .map_or(0, |checkpoint| checkpoint.generation);
    let batch = SourceBatch {
        expected_generation: generation,
        checkpoint: brain_contracts::SourceCheckpoint {
            source_id: domain_fixture::SOURCE.into(),
            configuration: "c6-synthetic-domain-v1".into(),
            generation: generation.checked_add(1).unwrap(),
            state: json!({"fixture":"synthetic-domain-adapter-not-game-stats-v1"}),
        },
        // Like FileConnector, each new job advances its checkpoint. Reusing an
        // old committed batch with a NEW lease would only replay the receipt,
        // leaving that new lease active. Unchanged sources need no new records.
        records: if previous.is_none() {
            domain_fixture::records("pilot-r1", PATCH, 1, "500")
        } else {
            Vec::new()
        },
    };
    let lease = store
        .claim(domain_fixture::SOURCE, owner, 30_000)
        .await
        .unwrap();
    let receipt = store.commit(&batch, &lease).await.unwrap();
    replayed.push(receipt.replayed);
    batches.push(batch);
    (batches, replayed)
}

fn batch_source(connector: &FileConnector) -> String {
    connector.prepare_batch(None).unwrap().checkpoint.source_id
}

fn release(id: &str, previous: Option<&CorpusRelease>, batches: &[SourceBatch]) -> CorpusRelease {
    let mut pins: BTreeMap<String, BTreeMap<String, u64>> = previous
        .map(|r| r.source_revisions.clone())
        .unwrap_or_default();
    for batch in batches {
        for record in &batch.records {
            let source = pins.entry(record.source_id.clone()).or_default();
            if record.tombstone {
                source.remove(&record.logical_id);
            } else {
                source.insert(record.logical_id.clone(), record.revision);
            }
        }
    }
    pins.retain(|_, docs| !docs.is_empty());
    CorpusRelease {
        release_id: id.into(),
        knowledge_version: "pilot-knowledge-v1".into(),
        patch: PATCH.into(),
        created_at_epoch: 1_790_000_000,
        source_revisions: pins,
    }
}

fn snapshot_digest(reader: &LocalPgReader, release_id: &str) -> (String, usize) {
    let snapshot = reader.read_snapshot(release_id).unwrap();
    let mut hasher = Sha256::new();
    for record in &snapshot.revisions {
        hasher.update(record.source_id.as_bytes());
        hasher.update([0]);
        hasher.update(record.logical_id.as_bytes());
        hasher.update([0]);
        hasher.update(record.revision.to_le_bytes());
        hasher.update(record.content_hash.as_bytes());
        hasher.update([0]);
    }
    (format!("{:x}", hasher.finalize()), snapshot.revisions.len())
}

fn reader(env: &Env) -> LocalPgReader {
    LocalPgReader::new(&env.socket, PORT, USER, &env.database).unwrap()
}

fn write_report(env: &Env, phase: &str, value: Value) {
    let path = env.report.join(format!("{phase}.json"));
    std::fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "local pilot: BRAIN_CORE_TEST_PG_SOCKET, BRAIN_PILOT_DATABASE, BRAIN_PILOT_ROOT, BRAIN_PILOT_REPORT"]
async fn pilot_phase_ingest() {
    let env = env();
    let store = store(&env).await;
    let started = Instant::now();
    let (batches, replayed) = ingest(&store, &env, "pilot-ingest").await;
    let r1 = release("pilot-r1", None, &batches);
    store.publish(&r1).await.unwrap();
    let reader = reader(&env);
    let (digest, documents) =
        tokio::task::spawn_blocking(move || snapshot_digest(&reader, "pilot-r1"))
            .await
            .unwrap();
    write_report(
        &env,
        "ingest",
        json!({
            "records": batches.iter().map(|b| b.records.len()).sum::<usize>(),
            "replayed": replayed,
            "release": "pilot-r1",
            "snapshot_digest": digest,
            "documents": documents,
            "elapsed_ms": started.elapsed().as_millis() as u64,
        }),
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "local pilot: run after pilot_phase_ingest and a forced PostgreSQL restart"]
async fn pilot_phase_after_restart() {
    let env = env();
    let store = store(&env).await;
    let reader_for_digest = reader(&env);
    let (digest, documents) =
        tokio::task::spawn_blocking(move || snapshot_digest(&reader_for_digest, "pilot-r1"))
            .await
            .unwrap();
    let (again, replayed) = ingest(&store, &env, "pilot-after-restart").await;
    let unchanged_records: usize = again
        .iter()
        .zip(&replayed)
        .filter(|(_, replayed)| !**replayed)
        .map(|(b, _)| b.records.len())
        .sum();

    let calls = Arc::new(AtomicUsize::new(0));
    let fail = Arc::new(AtomicBool::new(false));
    let sent: Sent = Arc::new(Mutex::new(Vec::new()));
    let files = pilot_files(&env);
    let provider_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let provider_address = provider_listener.local_addr().unwrap();
    let provider_router = Router::new()
        .route("/chat/completions", post(provider))
        .with_state((calls.clone(), fail.clone(), sent.clone()));
    let (provider_stop, provider_stopped) = tokio::sync::oneshot::channel::<()>();
    let provider_task = tokio::spawn(async move {
        axum::serve(provider_listener, provider_router)
            .with_graceful_shutdown(async {
                let _ = provider_stopped.await;
            })
            .await
            .unwrap();
    });
    let transport = tokio::task::spawn_blocking(move || {
        OpenAiCompatibleProvider::new(ProviderConfig::new(
            "pilot-loopback-provider",
            format!("http://{provider_address}"),
            "pilot-loopback-model",
        ))
        .unwrap()
    })
    .await
    .unwrap();
    let kernel = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(reader(&env), retrieval_limit()),
            transport,
        ),
        8,
        Duration::from_millis(1),
    );
    let credentials = CredentialRegistry::new(vec![
        AuthGrant::from_secret(
            PUBLIC_TOKEN,
            "pilot-public",
            "pilot",
            BTreeSet::from(["docs.public".into()]),
            BTreeSet::from(["public".into()]),
        ),
        AuthGrant::from_secret(
            INTERNAL_TOKEN,
            "pilot-operator",
            "pilot",
            BTreeSet::from(["docs.public".into(), "docs.internal".into()]),
            BTreeSet::from(["public".into(), "private".into()]),
        ),
    ]);
    let service = brain_api::ApiService::new(
        PolicyEngine::new(credentials),
        kernel,
        "pilot-r1",
        8000,
        budget(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let api_task = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(service))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });

    let exact_value = exact_number(&env);
    let mut cases = Vec::new();
    let run = |cases: &mut Vec<Value>, name: &str, token: &str, q: Query, check: Check| {
        let address = address.clone();
        let token = token.to_string();
        sent.lock().unwrap().clear();
        let started = Instant::now();
        let calls_before = calls.load(Ordering::SeqCst);
        let domain_case = matches!(check, Check::Domain(_, _));
        let outcome = std::thread::spawn(move || {
            BrainClient::new(&address, &token, Duration::from_secs(10))
                .unwrap()
                .answer(&q)
        })
        .join()
        .unwrap();
        let elapsed = started.elapsed().as_millis() as u64;
        let egress: Vec<String> = sent
            .lock()
            .unwrap()
            .iter()
            .flatten()
            .map(|h| {
                files
                    .get(h)
                    .cloned()
                    .unwrap_or_else(|| format!("unknown:{h}"))
            })
            .collect();
        let (status, result) = evaluate(&outcome, &egress, &env, check);
        let provider_calls = calls.load(Ordering::SeqCst) - calls_before;
        let passed = result && (!domain_case || provider_calls == 0);
        cases.push(json!({"case": name, "status": status, "passed": passed, "elapsed_ms": elapsed, "provider_egress": egress, "provider_calls": provider_calls}));
    };

    run(
        &mut cases,
        "public_question",
        PUBLIC_TOKEN,
        query("p1", "Abrams Siphon Life", &["docs.public"], None, None),
        Check::AnsweredFrom("public/"),
    );
    run(
        &mut cases,
        "internal_question",
        INTERNAL_TOKEN,
        query(
            "i1",
            "match demo learning",
            &["docs.public", "docs.internal"],
            None,
            None,
        ),
        Check::AnsweredFrom("internal/"),
    );
    run(
        &mut cases,
        "internal_question_public_token_scope_request",
        PUBLIC_TOKEN,
        query(
            "i2",
            "match demo learning",
            &["docs.public", "docs.internal"],
            None,
            None,
        ),
        Check::Rejected,
    );
    run(
        &mut cases,
        "internal_question_public_scope_no_leak",
        PUBLIC_TOKEN,
        query("i3", "match demo learning", &["docs.public"], None, None),
        Check::NoSource("internal/"),
    );
    run(
        &mut cases,
        "exact_number",
        PUBLIC_TOKEN,
        query(
            "n1",
            "Abrams BonusMaxHealthPerHero Max Health",
            &["docs.public"],
            None,
            None,
        ),
        Check::CitationContains(exact_value.clone()),
    );
    run(
        &mut cases,
        "alias_en",
        PUBLIC_TOKEN,
        query("a1", "Lady Geist", &["docs.public"], None, None),
        Check::CitationLogical("lady-geist.md"),
    );
    run(
        &mut cases,
        "alias_de_lowercase",
        PUBLIC_TOKEN,
        query("a2", "geist", &["docs.public"], None, None),
        Check::CitationLogical("lady-geist.md"),
    );
    run(
        &mut cases,
        "unknown_entity",
        PUBLIC_TOKEN,
        query("u1", "Zzyzxqv", &["docs.public"], None, None),
        Check::NotAnswered,
    );
    run(
        &mut cases,
        "missing_evidence",
        PUBLIC_TOKEN,
        query(
            "m1",
            "Turnierregeln Preisgeld Anmeldeschluss",
            &["docs.public"],
            None,
            None,
        ),
        Check::NotAnswered,
    );
    run(
        &mut cases,
        "wrong_patch",
        PUBLIC_TOKEN,
        query(
            "w1",
            "Abrams",
            &["docs.public"],
            Some("patch-does-not-exist"),
            None,
        ),
        Check::NotAnswered,
    );
    run(
        &mut cases,
        "wrong_mode",
        PUBLIC_TOKEN,
        query("w2", "Abrams", &["docs.public"], None, Some("street_brawl")),
        Check::NotAnswered,
    );
    run(
        &mut cases,
        "invalid_token",
        "pilot-invalid-token",
        query("x1", "Abrams", &["docs.public"], None, None),
        Check::Rejected,
    );
    for (name, items, status, reason) in [
        (
            "legal_build",
            vec!["101", "102", "103"],
            AnswerStatus::Answered,
            "Build legal",
        ),
        (
            "illegal_build",
            vec!["101", "101"],
            AnswerStatus::BuildRejected,
            "bereits im Inventar",
        ),
    ] {
        let mut q =
            domain_fixture::query(&domain_fixture::build("Fixture Hero", "en", &items), PATCH);
        q.request_id = name.into();
        q.conversation_id = format!("pilot-{name}");
        run(
            &mut cases,
            name,
            PUBLIC_TOKEN,
            q,
            Check::Domain(status, reason),
        );
    }
    fail.store(true, Ordering::SeqCst);
    run(
        &mut cases,
        "provider_error",
        PUBLIC_TOKEN,
        query("e1", "Wraith", &["docs.public"], None, None),
        Check::Status(AnswerStatus::ProviderError),
    );
    fail.store(false, Ordering::SeqCst);

    let load = load(&address);
    let revoke_reader = reader(&env);
    let mut revoked = tokio::task::spawn_blocking(move || {
        revoke_reader
            .read_snapshot("pilot-r1")
            .unwrap()
            .heads
            .into_iter()
            .find(|r| r.logical_id.ends_with("warden.md"))
            .unwrap()
    })
    .await
    .unwrap();
    revoked.revision += 1;
    revoked.visibility = SourceVisibility::Private;
    revoked.allowed_scopes = BTreeSet::from(["docs.internal".into()]);
    store.apply(&revoked).await.unwrap();
    run(
        &mut cases,
        "acl_revoke_live_release",
        PUBLIC_TOKEN,
        query("r1", "Warden", &["docs.public"], None, None),
        Check::NoLogical("warden.md"),
    );

    let removed = env.root.join("public/vindicta.md");
    let parked = env.root.join("vindicta.md.parked");
    std::fs::rename(&removed, &parked).unwrap();
    let (deleted, _) = ingest(&store, &env, "pilot-delete").await;
    std::fs::rename(&parked, &removed).unwrap();
    let tombstones: usize = deleted
        .iter()
        .flat_map(|b| &b.records)
        .filter(|r| r.tombstone)
        .count();
    run(
        &mut cases,
        "delete_tombstone_live_release",
        PUBLIC_TOKEN,
        query("d1", "Vindicta", &["docs.public"], None, None),
        Check::NoLogical("vindicta.md"),
    );

    let calls_total = calls.load(Ordering::SeqCst);
    stop.send(()).unwrap();
    api_task.await.unwrap();
    provider_stop.send(()).unwrap();
    provider_task.await.unwrap();
    let failed: Vec<_> = cases
        .iter()
        .filter(|c| c["passed"] != json!(true))
        .map(|c| c["case"].clone())
        .collect();
    write_report(
        &env,
        &format!(
            "after_restart_{}",
            std::env::var("BRAIN_PILOT_VARIANT").unwrap_or_else(|_| "default".into())
        ),
        json!({
            "retrieval_limit": retrieval_limit(),
            "budget_max_input_tokens": budget().max_input_tokens,
            "snapshot_digest_after_restart": digest,
            "documents": documents,
            "reingest_replayed": replayed,
            "reingest_records": unchanged_records,
            "delete_tombstones": tombstones,
            "provider_calls": calls_total,
            "load": load,
            "cases": cases,
            "failed": failed,
        }),
    );
    assert!(failed.is_empty(), "pilot cases failed: {failed:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "local pilot: empty rebuild into a fresh pilot database"]
async fn pilot_phase_empty_rebuild() {
    let env = env();
    let store = store(&env).await;
    let (batches, _) = ingest(&store, &env, "pilot-rebuild").await;
    let r1 = release("pilot-r1", None, &batches);
    store.publish(&r1).await.unwrap();
    let reader = reader(&env);
    let (digest, documents) =
        tokio::task::spawn_blocking(move || snapshot_digest(&reader, "pilot-r1"))
            .await
            .unwrap();
    write_report(
        &env,
        "rebuild",
        json!({"snapshot_digest": digest, "documents": documents}),
    );
}

fn load(address: &str) -> Value {
    if std::env::var("BRAIN_PILOT_VARIANT").as_deref() != Ok("diagnostic") {
        return Value::Null;
    }
    let requests: usize = match std::env::var("BRAIN_PILOT_LOAD_REQUESTS") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => return Value::Null,
    };
    let workers: usize = std::env::var("BRAIN_PILOT_LOAD_WORKERS")
        .map(|v| v.parse().unwrap())
        .unwrap_or(8);
    let texts = [
        "Abrams",
        "Haze",
        "Wraith",
        "Lady Geist",
        "Warden",
        "Vindicta",
    ];
    let started = Instant::now();
    let handles: Vec<_> = (0..workers)
        .map(|worker| {
            let address = address.to_string();
            std::thread::spawn(move || {
                let client =
                    BrainClient::new(&address, PUBLIC_TOKEN, Duration::from_secs(10)).unwrap();
                let mut samples = Vec::new();
                let mut statuses: BTreeMap<String, usize> = BTreeMap::new();
                for i in (worker..requests).step_by(workers) {
                    let q = query(
                        &format!("load-{i}"),
                        texts[i % texts.len()],
                        &["docs.public"],
                        None,
                        None,
                    );
                    let t = Instant::now();
                    let key = match client.answer(&q) {
                        Ok(r) => serde_json::to_value(r.status)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        Err(_) => "client_error".into(),
                    };
                    samples.push(t.elapsed().as_micros() as u64);
                    *statuses.entry(key).or_default() += 1;
                }
                (samples, statuses)
            })
        })
        .collect();
    let mut samples = Vec::new();
    let mut statuses: BTreeMap<String, usize> = BTreeMap::new();
    for handle in handles {
        let (s, st) = handle.join().unwrap();
        samples.extend(s);
        for (k, v) in st {
            *statuses.entry(k).or_default() += v;
        }
    }
    let wall = started.elapsed();
    let process = process_usage();
    samples.sort_unstable();
    let pct = |p: f64| {
        samples[((samples.len() as f64 * p).ceil() as usize).saturating_sub(1)] as f64 / 1000.0
    };
    json!({
        "requests": samples.len(),
        "workers": workers,
        "wall_ms": wall.as_millis() as u64,
        "throughput_rps": samples.len() as f64 / wall.as_secs_f64(),
        "p50_ms": pct(0.50),
        "p95_ms": pct(0.95),
        "p99_ms": pct(0.99),
        "max_ms": *samples.last().unwrap() as f64 / 1000.0,
        "statuses": statuses,
        "process_after_load": process,
    })
}

fn process_usage() -> Value {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let field = |name: &str| {
        status
            .lines()
            .find(|l| l.starts_with(name))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
    };
    let stat = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
    let fields: Vec<&str> = stat
        .rsplit_once(')')
        .map(|(_, rest)| rest.split_whitespace().collect())
        .unwrap_or_default();
    let ticks = |i: usize| fields.get(i).and_then(|v| v.parse::<u64>().ok());
    let io = std::fs::read_to_string("/proc/self/io").unwrap_or_default();
    let io_field = |name: &str| {
        io.lines()
            .find(|l| l.starts_with(name))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
    };
    json!({
        "vm_hwm_kib": field("VmHWM:"),
        "vm_rss_kib": field("VmRSS:"),
        "threads": field("Threads:"),
        "utime_ticks": ticks(11),
        "stime_ticks": ticks(12),
        "clock_ticks_per_second": 100,
        "read_bytes": io_field("read_bytes:"),
        "write_bytes": io_field("write_bytes:"),
    })
}

fn retrieval_limit() -> usize {
    std::env::var("BRAIN_PILOT_RETRIEVAL_LIMIT")
        .map(|v| v.parse().unwrap())
        .unwrap_or(6)
}

fn budget() -> Budget {
    let mut budget = Budget::default();
    if let Ok(tokens) = std::env::var("BRAIN_PILOT_MAX_INPUT_TOKENS") {
        budget.max_input_tokens = tokens.parse().unwrap();
    }
    budget
}

fn exact_number(env: &Env) -> Vec<String> {
    let text = std::fs::read_to_string(env.root.join("public/abrams.md")).unwrap();
    let at = text.find("\"Key\": \"BonusMaxHealthPerHero\"").unwrap();
    let rest = &text[at..];
    let value_at = rest.find("\"Value\": ").unwrap() + "\"Value\": ".len();
    let value: String = rest[value_at..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    assert!(!value.is_empty());
    vec![
        "\"Key\": \"BonusMaxHealthPerHero\"".into(),
        format!("\"Value\": {value}"),
    ]
}

enum Check {
    AnsweredFrom(&'static str),
    NoSource(&'static str),
    NoLogical(&'static str),
    CitationLogical(&'static str),
    CitationContains(Vec<String>),
    NotAnswered,
    Rejected,
    Status(AnswerStatus),
    Domain(AnswerStatus, &'static str),
}

fn evaluate(
    outcome: &Result<PublicAnswerResponse, brain_client::ClientError>,
    egress: &[String],
    env: &Env,
    check: Check,
) -> (String, bool) {
    let response = match outcome {
        Ok(r) => r,
        Err(_) => return ("rejected".into(), matches!(check, Check::Rejected)),
    };
    let status = serde_json::to_value(response.status)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let answered = response.status == AnswerStatus::Answered && !response.citations.is_empty();
    let first = egress.first();
    let passed = match check {
        Check::AnsweredFrom(dir) => answered && first.is_some_and(|f| f.starts_with(dir)),
        Check::NoSource(dir) => egress.iter().all(|f| !f.starts_with(dir)),
        Check::NoLogical(name) => egress.iter().all(|f| !f.ends_with(name)),
        Check::CitationLogical(name) => answered && first.is_some_and(|f| f.ends_with(name)),
        Check::CitationContains(needles) => {
            answered
                && first.is_some_and(|f| {
                    let text = std::fs::read_to_string(env.root.join(f)).unwrap_or_default();
                    needles.iter().all(|n| text.contains(n.as_str()))
                })
        }
        Check::NotAnswered => !answered,
        Check::Rejected => false,
        Check::Status(expected) => response.status == expected,
        Check::Domain(expected, reason) => {
            response.status == expected
                && egress.is_empty()
                && response.text.contains(reason)
                && !response.citations.is_empty()
                && response.text.contains(&response.citations[0].label)
        }
    };
    (status, passed)
}

type Sent = Arc<Mutex<Vec<Vec<String>>>>;

fn pilot_files(env: &Env) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    for dir in ["public", "internal"] {
        for entry in std::fs::read_dir(env.root.join(dir)).unwrap() {
            let path = entry.unwrap().path();
            let bytes = std::fs::read(&path).unwrap();
            let name = format!("{dir}/{}", path.file_name().unwrap().to_string_lossy());
            files.insert(format!("{:x}", Sha256::digest(&bytes)), name);
        }
    }
    files
}

fn query(id: &str, text: &str, scopes: &[&str], patch: Option<&str>, mode: Option<&str>) -> Query {
    Query {
        domain: None,
        request_id: id.into(),
        conversation_id: format!("pilot-conversation-{id}"),
        text: text.into(),
        requested_scopes: scopes.iter().map(|s| s.to_string()).collect(),
        profile: AnswerProfile::Explain,
        patch: patch.map(Into::into),
        mode: mode.map(Into::into),
    }
}

async fn provider(
    State((calls, fail, sent)): State<(Arc<AtomicUsize>, Arc<AtomicBool>, Sent)>,
    body: Bytes,
) -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    calls.fetch_add(1, Ordering::SeqCst);
    let request: Value = serde_json::from_slice(&body).unwrap();
    let input: Value =
        serde_json::from_str(request["messages"][1]["content"].as_str().unwrap()).unwrap();
    sent.lock().unwrap().push(
        input["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                format!(
                    "{:x}",
                    Sha256::digest(e["content"].as_str().unwrap().as_bytes())
                )
            })
            .collect(),
    );
    if fail.load(Ordering::SeqCst) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            "{}".into(),
        );
    }
    let id = input["evidence"][0]["id"].as_str().unwrap();
    let answer =
        json!({"text": "Pilotantwort aus der ersten Evidenz.", "cited_evidence_ids": [id]})
            .to_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        json!({"model":"pilot-loopback-model","choices":[{"message":{"content":answer}}],"usage":{"prompt_tokens":16,"completion_tokens":10}}).to_string(),
    )
}
