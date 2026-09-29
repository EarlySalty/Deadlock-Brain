use brain_contracts::{DocumentStorePort, SourceBatch, SourceVisibility};
use brain_feeds::{
    deadlock_match::{
        commit_match_batch, demo_source_id, match_metadata_url, match_release_from_batches,
        match_source_id, prepare_match_metadata_batch, prepare_revoke_demo_batch,
        prepare_revoke_match_batch, MatchScope,
    },
    FeedPolicy,
};
use brain_storage::PgStore;
use dbrain_sources::{external::sha256, schema_watch::OpenApiSnapshot};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::BTreeSet, fs::File, io::Read, os::unix::fs::PermissionsExt, path::Path,
    time::Duration,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    socket_dir: String,
    port: u16,
    database: String,
    username: String,
    owner: String,
    account_id: String,
    match_id: String,
    base_release_id: String,
    release_id: String,
    created_at_epoch: i64,
    schema_sha256: String,
    expected_raw_sha256: Option<String>,
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte))
}

fn load_config(path: &Path) -> Result<Config, String> {
    let metadata = path.metadata().map_err(|_| "config unreadable")?;
    if !metadata.is_file() || metadata.permissions().mode() & 0o077 != 0 {
        return Err("config requires a private regular file".into());
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|_| "config unreadable")?
        .take(16_385)
        .read_to_end(&mut bytes)
        .map_err(|_| "config unreadable")?;
    if bytes.len() > 16_384 {
        return Err("config exceeds 16 KiB".into());
    }
    let config: Config = serde_json::from_slice(&bytes).map_err(|_| "config invalid")?;
    if !Path::new(&config.socket_dir).is_absolute()
        || config.port == 0
        || ![
            &config.database,
            &config.username,
            &config.owner,
            &config.base_release_id,
            &config.release_id,
        ]
        .iter()
        .all(|value| identifier(value))
        || config.base_release_id == config.release_id
        || config.created_at_epoch <= 0
        || config
            .expected_raw_sha256
            .as_ref()
            .is_some_and(|hash| hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err("config fields outside policy".into());
    }
    let schema = OpenApiSnapshot::pinned().map_err(|_| "pinned schema unavailable")?;
    if config.schema_sha256 != schema.schema_sha256 {
        return Err("schema hash does not match reviewed OpenAPI".into());
    }
    Ok(config)
}

fn verify_raw_hash(expected: Option<&str>, bytes: &[u8]) -> Result<String, String> {
    let digest = sha256(bytes);
    if expected.is_some_and(|value| value != digest) {
        return Err("match body hash differs from configured expectation".into());
    }
    Ok(digest)
}

async fn run(config: Config, action: &str, http: &HttpClient) -> Result<serde_json::Value, String> {
    let scope = MatchScope {
        account_id: config.account_id,
        match_id: config.match_id,
    };
    let source_id = match_source_id(&scope).map_err(|_| "invalid match identity")?;
    let policy = FeedPolicy {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::from([format!("account:{}", scope.account_id)]),
        provider_egress_allowed: false,
        publication_allowed: false,
        raw_retention_allowed: false,
    };
    if std::env::var_os("PGPASSWORD").is_some() || std::env::var_os("DATABASE_URL").is_some() {
        return Err("ambient database credentials are forbidden".into());
    }
    let options = PgConnectOptions::new_without_pgpass()
        .host(&config.socket_dir)
        .port(config.port)
        .username(&config.username)
        .database(&config.database)
        .application_name("brain-match-ingest");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .map_err(|_| "Postgres connection failed")?;
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .map_err(|_| "Postgres transport check failed")?;
    if address.is_some() {
        return Err("Postgres must use a Unix socket".into());
    }
    let store = PgStore::new(pool);
    store
        .check_core_schema()
        .await
        .map_err(|_| "Brain core schema unavailable")?;
    let base = store
        .snapshot(&config.base_release_id)
        .await
        .map_err(|_| "base release unavailable")?
        .release;
    let mut batches: Vec<SourceBatch> = Vec::new();
    let observed_sha256 = match action {
        "ingest" => {
            let url = match_metadata_url(&scope).map_err(|_| "invalid match URL")?;
            let response = http
                .get_bounded(
                    &url,
                    SourceHttpOptions {
                        max_bytes: 1024 * 1024,
                        attempts: 2,
                        request_timeout: Duration::from_secs(8),
                        total_timeout: Duration::from_secs(16),
                        backoff: Duration::from_millis(200),
                        max_retry_wait: Duration::from_secs(2),
                        headers: vec![("Accept".into(), "application/json".into())],
                    },
                )
                .map_err(|_| "bounded Deadlock API request failed")?;
            let digest = verify_raw_hash(config.expected_raw_sha256.as_deref(), &response.content)?;
            let previous = store
                .checkpoint(&source_id)
                .await
                .map_err(|_| "match checkpoint unavailable")?;
            batches.push(
                prepare_match_metadata_batch(&scope, response, &policy, previous.as_ref())
                    .map_err(|_| "match response rejected by contract")?,
            );
            Some(digest)
        }
        "revoke" => {
            if config.expected_raw_sha256.is_some() {
                return Err("revoke must not pin an HTTP body".into());
            }
            if let Some(previous) = store
                .checkpoint(&source_id)
                .await
                .map_err(|_| "match checkpoint unavailable")?
            {
                batches.push(
                    prepare_revoke_match_batch(&scope, &policy, &previous)
                        .map_err(|_| "match revoke rejected by contract")?,
                );
            }
            let demo_id = demo_source_id(&scope).map_err(|_| "invalid demo identity")?;
            if let Some(previous) = store
                .checkpoint(&demo_id)
                .await
                .map_err(|_| "demo checkpoint unavailable")?
            {
                batches.push(
                    prepare_revoke_demo_batch(&scope, &policy, &previous)
                        .map_err(|_| "demo revoke rejected by contract")?,
                );
            }
            if batches.is_empty() {
                return Err("no match evidence exists to revoke".into());
            }
            None
        }
        _ => return Err("action must be ingest or revoke".into()),
    };
    let mut changes = Vec::new();
    for batch in &batches {
        let receipt = commit_match_batch(&store, batch, &config.owner)
            .await
            .map_err(|_| "match batch commit failed")?;
        changes.push(json!({
            "source_id": batch.checkpoint.source_id,
            "generation": receipt.generation,
            "replayed": receipt.replayed,
            "records": batch.records.len(),
            "tombstones": batch.records.iter().filter(|r| r.tombstone).count(),
        }));
    }
    let release =
        match_release_from_batches(&base, &batches, &config.release_id, config.created_at_epoch)
            .map_err(|_| "match release rejected by contract")?;
    store
        .publish(&release)
        .await
        .map_err(|_| "match release publish failed")?;
    let readback = store
        .snapshot(&config.release_id)
        .await
        .map_err(|_| "match release readback failed")?;
    if readback.release != release
        || readback.revisions.len()
            != release
                .source_revisions
                .values()
                .map(std::collections::BTreeMap::len)
                .sum::<usize>()
    {
        return Err("match release readback mismatch".into());
    }
    Ok(json!({
        "release_id": release.release_id,
        "knowledge_version": release.knowledge_version,
        "patch": release.patch,
        "account_id": scope.account_id,
        "match_id": scope.match_id,
        "observed_raw_sha256": observed_sha256,
        "sources": changes,
        "release_documents": readback.revisions.len(),
    }))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let [_, action, flag, path] = args.as_slice() else {
        eprintln!("usage: brain-match-ingest <ingest|revoke> --config <private-json>");
        std::process::exit(64);
    };
    if flag != "--config" || !["ingest", "revoke"].contains(&action.as_str()) {
        eprintln!("usage: brain-match-ingest <ingest|revoke> --config <private-json>");
        std::process::exit(64);
    }
    let result = (|| {
        let config = load_config(Path::new(path))?;
        let cache = tempfile::tempdir().map_err(|_| "temporary HTTP directory unavailable")?;
        let http = HttpClient::new("brain-match-ingest/1", cache.path())
            .map_err(|_| "HTTP client unavailable")?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| "runtime unavailable")?;
        let result = runtime.block_on(run(config, action, &http));
        drop(runtime);
        result
    })();
    match result {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("brain-match-ingest: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_match_hash_must_equal_received_raw_bytes() {
        let body = b"fixture-match-response";
        let digest = sha256(body);
        assert_eq!(verify_raw_hash(Some(&digest), body).unwrap(), digest);
        assert!(verify_raw_hash(Some(&"0".repeat(64)), body).is_err());
        assert_eq!(verify_raw_hash(None, body).unwrap(), digest);
    }
}
