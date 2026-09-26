use brain_contracts::{DocumentStorePort, SourceCheckpoint};
use brain_legacy_import::{
    entity_documents, patch_documents, pg::read_legacy, prepare_batch, release_from_checkpoints,
    snapshot_digest, ImportContext, ReleaseConfig, SourcePolicyConfig, ENTITIES_SOURCE,
    PATCHNOTES_SOURCE,
};
use brain_storage::PgStore;
use serde::Deserialize;
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Connection,
};
use std::{collections::BTreeMap, path::PathBuf, time::Duration};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Endpoint {
    socket: String,
    port: u16,
    database: String,
    username: String,
    auth_env: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    legacy: Endpoint,
    target: Endpoint,
    snapshot_label: String,
    snapshot_epoch: i64,
    owner: String,
    release: ReleaseConfig,
    sources: BTreeMap<String, SourcePolicyConfig>,
    report: PathBuf,
}

fn options(endpoint: &Endpoint) -> Result<PgConnectOptions, String> {
    if !endpoint.socket.starts_with('/') {
        return Err("only Unix sockets are allowed".into());
    }
    let mut options = PgConnectOptions::new_without_pgpass()
        .host(&endpoint.socket)
        .port(endpoint.port)
        .username(&endpoint.username)
        .database(&endpoint.database)
        .application_name("brain-legacy-import");
    if let Some(name) = &endpoint.auth_env {
        let value = std::env::var(name).map_err(|_| format!("{name} missing"))?;
        options = options.password(&value);
    }
    Ok(options)
}

async fn run(config: Config) -> Result<serde_json::Value, String> {
    if !config.target.database.starts_with("brain_pilot") {
        return Err("target must be a brain_pilot* database".into());
    }
    if config.legacy.database == config.target.database {
        return Err("legacy archive and target must be different databases".into());
    }
    let expected: Vec<&str> = vec![ENTITIES_SOURCE, PATCHNOTES_SOURCE];
    if config
        .sources
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>()
        != expected
    {
        return Err(format!("sources must be exactly {expected:?}"));
    }
    let mut legacy = options(&config.legacy)?
        .connect()
        .await
        .map_err(|e| format!("legacy connect: {e}"))?;
    let read = read_legacy(&mut legacy).await.map_err(|e| e.to_string())?;
    legacy.close().await.map_err(|e| e.to_string())?;
    let context = ImportContext {
        snapshot_label: config.snapshot_label.clone(),
        snapshot_epoch: config.snapshot_epoch,
        schema_sha256: read.schema_sha256.clone(),
    };
    let sources = [
        entity_documents(&read.entities).map_err(|e| e.to_string())?,
        patch_documents(&read.patch_lines).map_err(|e| e.to_string())?,
    ];
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options(&config.target)?)
        .await
        .map_err(|e| format!("target connect: {e}"))?;
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;
    if address.is_some() {
        return Err("target must not use TCP".into());
    }
    let store = PgStore::new(pool);
    store
        .check_core_schema()
        .await
        .map_err(|e| format!("core schema: {e:?}"))?;
    let mut batches = Vec::new();
    let mut checkpoints: Vec<SourceCheckpoint> = Vec::new();
    for source in &sources {
        let policy = &config.sources[source.source_id];
        let previous = store
            .checkpoint(source.source_id)
            .await
            .map_err(|e| format!("{e:?}"))?;
        let batch = prepare_batch(source, policy, &context, previous.as_ref())
            .map_err(|e| e.to_string())?;
        let lease = store
            .claim(source.source_id, &config.owner, 60_000)
            .await
            .map_err(|e| format!("{e:?}"))?;
        let receipt = store
            .commit(&batch, &lease)
            .await
            .map_err(|e| format!("{e:?}"))?;
        let inserted = batch.records.iter().filter(|r| !r.tombstone).count();
        let tombstoned = batch.records.len() - inserted;
        batches.push(json!({
            "source_id": source.source_id,
            "documents": source.documents.len(),
            "legacy_rows": source.documents.iter().map(|d| d.legacy_rows).sum::<usize>(),
            "changed_records": inserted,
            "tombstones": tombstoned,
            "generation": receipt.generation,
            "replayed": receipt.replayed,
        }));
        checkpoints.push(batch.checkpoint);
    }
    let release = release_from_checkpoints(&checkpoints, &config.release, config.snapshot_epoch)
        .map_err(|e| e.to_string())?;
    store
        .publish(&release)
        .await
        .map_err(|e| format!("{e:?}"))?;
    let snapshot = store
        .snapshot(&release.release_id)
        .await
        .map_err(|e| format!("{e:?}"))?;
    let documents: usize = release.source_revisions.values().map(BTreeMap::len).sum();
    if snapshot.revisions.len() != documents {
        return Err("snapshot does not match release pins".into());
    }
    Ok(json!({
        "snapshot_label": config.snapshot_label,
        "schema_sha256": read.schema_sha256,
        "legacy_table_counts": read.table_counts,
        "sources": batches,
        "release_id": release.release_id,
        "knowledge_version": release.knowledge_version,
        "release_patch": release.patch,
        "release_documents": documents,
        "snapshot_digest": snapshot_digest(&snapshot.revisions),
    }))
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = match args.as_slice() {
        [_, flag, path] if flag == "--config" => path.clone(),
        _ => {
            eprintln!("usage: brain-legacy-import --config <file>");
            std::process::exit(64);
        }
    };
    let config: Config = match std::fs::read(&path)
        .map_err(|e| e.to_string())
        .and_then(|b| serde_json::from_slice(&b).map_err(|e| e.to_string()))
    {
        Ok(config) => config,
        Err(error) => {
            eprintln!("config: {error}");
            std::process::exit(64);
        }
    };
    let report = config.report.clone();
    match run(config).await {
        Ok(value) => {
            let text = serde_json::to_string_pretty(&value).unwrap();
            if let Err(error) = std::fs::write(&report, &text) {
                eprintln!("report: {error}");
                std::process::exit(1);
            }
            println!("{text}");
        }
        Err(error) => {
            eprintln!("legacy import failed: {error}");
            std::process::exit(1);
        }
    }
}
