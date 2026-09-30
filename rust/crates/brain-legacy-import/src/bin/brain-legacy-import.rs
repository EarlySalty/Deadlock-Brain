use brain_contracts::{DocumentStorePort, SourceCheckpoint, SourceRecordV2};
use brain_ingestion::document_set::DocumentSetCheckpoint;
use brain_legacy_import::{
    ENTITIES_SOURCE, ImportContext, PATCHNOTES_SOURCE, ReleaseConfig, SourcePolicyConfig,
    cutover::CutoverBinding, entity_documents, patch_documents, pg::read_legacy, prepare_batch,
    release_from_checkpoints, snapshot_digest,
};
use brain_storage::PgStore;
use serde::Deserialize;
use serde_json::json;
use sqlx::{
    ConnectOptions, Connection, Row,
    postgres::{PgConnectOptions, PgConnection, PgPoolOptions},
};
use std::{collections::BTreeMap, path::PathBuf, time::Duration};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Endpoint {
    socket: String,
    port: u16,
    database: String,
    username: String,
    auth_env: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    legacy: Endpoint,
    target: Endpoint,
    snapshot_label: String,
    snapshot_epoch: i64,
    owner: String,
    release: ReleaseConfig,
    sources: BTreeMap<String, SourcePolicyConfig>,
    production_binding: Option<CutoverBinding>,
    report: PathBuf,
}

#[derive(Clone, Copy)]
struct Destination<'a> {
    socket: &'a str,
    port: u16,
    database: &'a str,
    legacy_role: &'a str,
    target_role: &'a str,
    legacy_auth: &'a str,
    target_auth: &'a str,
    require_auth: bool,
}

const PRODUCTION: Destination<'static> = Destination {
    socket: "/run/deadlock-brain-postgresql",
    port: 5446,
    database: "brain",
    legacy_role: "brain_readonly",
    target_role: "brain_ingest",
    legacy_auth: "BRAIN_LEGACY_READ_AUTH",
    target_auth: "BRAIN_TARGET_INGEST_AUTH",
    require_auth: true,
};

fn route(config: &Config, destination: Destination<'_>) -> Result<bool, String> {
    if config.target.database.starts_with("brain_pilot") {
        if config.legacy.database == config.target.database || config.production_binding.is_some() {
            return Err("pilot archive and target must be different databases".into());
        }
        return Ok(false);
    }
    if config.target.database != destination.database
        || config.legacy.database != destination.database
        || config.legacy.socket != destination.socket
        || config.target.socket != destination.socket
        || config.legacy.port != destination.port
        || config.target.port != destination.port
        || config.legacy.username != destination.legacy_role
        || config.target.username != destination.target_role
        || config.production_binding.is_none()
        || !config.report.is_absolute()
        || config.owner.trim().is_empty()
        || config.release.id_prefix.trim().is_empty()
        || config.release.knowledge_version.trim().is_empty()
        || config.release.patch.trim().is_empty()
    {
        return Err("target requires the exact approved archive-to-core binding".into());
    }
    if destination.require_auth
        && (config.legacy.auth_env.as_deref() != Some(destination.legacy_auth)
            || config.target.auth_env.as_deref() != Some(destination.target_auth)
            || destination.legacy_auth == destination.target_auth)
    {
        return Err("cutover requires the existing separate secret references".into());
    }
    Ok(true)
}

const IDENTITY_SQL: &str = "SELECT current_database() AS database, current_user AS username,
    inet_server_addr()::text AS address,
    (SELECT oid::bigint FROM pg_database WHERE datname=current_database()) AS database_oid,
    (SELECT oid::bigint FROM pg_namespace WHERE nspname='brain_legacy') AS archive_oid,
    (SELECT oid::bigint FROM pg_namespace WHERE nspname='brain') AS core_oid";

struct Identity {
    database: String,
    username: String,
    address: Option<String>,
    database_oid: i64,
    archive_oid: Option<i64>,
    core_oid: Option<i64>,
}

impl Identity {
    fn read(row: &sqlx::postgres::PgRow) -> Result<Self, String> {
        Ok(Self {
            database: row
                .try_get("database")
                .map_err(|e: sqlx::Error| e.to_string())?,
            username: row
                .try_get("username")
                .map_err(|e: sqlx::Error| e.to_string())?,
            address: row
                .try_get("address")
                .map_err(|e: sqlx::Error| e.to_string())?,
            database_oid: row
                .try_get("database_oid")
                .map_err(|e: sqlx::Error| e.to_string())?,
            archive_oid: row
                .try_get("archive_oid")
                .map_err(|e: sqlx::Error| e.to_string())?,
            core_oid: row
                .try_get("core_oid")
                .map_err(|e: sqlx::Error| e.to_string())?,
        })
    }

    fn verify(
        &self,
        destination: Destination<'_>,
        role: &str,
        binding: &CutoverBinding,
    ) -> Result<(), String> {
        if self.database != destination.database
            || self.username != role
            || self.address.is_some()
            || self.database_oid != binding.database_oid
            || self.archive_oid != Some(binding.archive_schema_oid)
            || self.core_oid != Some(binding.core_schema_oid)
        {
            return Err("connected database, role or schema differs from cutover approval".into());
        }
        Ok(())
    }
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

fn verify_checkpoint(
    previous: Option<&SourceCheckpoint>,
    heads: &BTreeMap<String, SourceRecordV2>,
) -> Result<(), String> {
    match previous {
        None if heads.is_empty() => Ok(()),
        Some(checkpoint) => {
            let state: DocumentSetCheckpoint = serde_json::from_value(checkpoint.state.clone())
                .map_err(|_| "invalid existing cutover checkpoint")?;
            if state.configuration != checkpoint.configuration
                || state.documents.len() != heads.len()
            {
                return Err("cutover target heads disagree with checkpoint".into());
            }
            for (id, document) in state.documents {
                let head = heads
                    .get(&id)
                    .ok_or("cutover checkpoint refers to a missing head")?;
                if head.revision != document.revision
                    || head.content_hash != document.content_hash
                    || head.tombstone != document.tombstone
                {
                    return Err("cutover head changed after its checkpoint".into());
                }
            }
            Ok(())
        }
        None => Err("uncheckpointed cutover target heads exist".into()),
    }
}

async fn target_heads(
    pool: &sqlx::PgPool,
    binding: &CutoverBinding,
    sources: &[brain_legacy_import::LegacySource; 2],
    policies: &BTreeMap<String, SourcePolicyConfig>,
    context: &ImportContext,
) -> Result<BTreeMap<String, BTreeMap<String, SourceRecordV2>>, String> {
    let rows: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT record_json FROM brain.source_record_heads WHERE source_id IN ($1,$2)",
    )
    .bind(ENTITIES_SOURCE)
    .bind(PATCHNOTES_SOURCE)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("cutover target heads: {e}"))?;
    let mut heads: BTreeMap<String, BTreeMap<String, SourceRecordV2>> = BTreeMap::new();
    for value in rows {
        let record: SourceRecordV2 =
            serde_json::from_value(value).map_err(|_| "invalid cutover target head")?;
        binding
            .verify_head(&record, sources, policies, context)
            .map_err(|e| e.to_string())?;
        let source = heads.entry(record.source_id.clone()).or_default();
        if source.insert(record.logical_id.clone(), record).is_some() {
            return Err("duplicate cutover target head".into());
        }
    }
    Ok(heads)
}

async fn run(config: Config) -> Result<serde_json::Value, String> {
    run_with_destination(config, PRODUCTION).await
}

async fn run_with_destination(
    config: Config,
    destination: Destination<'_>,
) -> Result<serde_json::Value, String> {
    let production = route(&config, destination)?;
    let binding = if production {
        config.production_binding.as_ref()
    } else {
        None
    };
    if production
        && (!config.report.parent().is_some_and(|parent| parent.is_dir())
            || config.report.file_name().is_none())
    {
        return Err("cutover requires an existing report directory".into());
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
    if let Some(binding) = binding {
        let row = sqlx::query(IDENTITY_SQL)
            .fetch_one(&mut legacy)
            .await
            .map_err(|e| format!("legacy identity: {e}"))?;
        Identity::read(&row)?.verify(destination, destination.legacy_role, binding)?;
    }
    let read = read_legacy(&mut legacy).await.map_err(|e| e.to_string())?;
    legacy.close().await.map_err(|e| e.to_string())?;
    let context = ImportContext {
        snapshot_label: config.snapshot_label.clone(),
        snapshot_epoch: config.snapshot_epoch,
        schema_sha256: read.schema_sha256.clone(),
    };
    let mut sources = [
        entity_documents(&read.entities).map_err(|e| e.to_string())?,
        patch_documents(&read.patch_lines).map_err(|e| e.to_string())?,
    ];
    if let Some(binding) = binding {
        binding
            .verify_sources(
                &mut sources,
                &read,
                &config.snapshot_label,
                config.snapshot_epoch,
                &config.sources,
            )
            .map_err(|e| e.to_string())?;
    }
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
    if let Some(binding) = binding {
        let row = sqlx::query(IDENTITY_SQL)
            .fetch_one(&pool)
            .await
            .map_err(|e| format!("target identity: {e}"))?;
        Identity::read(&row)?.verify(destination, destination.target_role, binding)?;
    }
    let store = PgStore::new(pool.clone());
    store
        .check_core_schema()
        .await
        .map_err(|e| format!("core schema: {e:?}"))?;
    let current_heads = if let Some(binding) = binding {
        target_heads(&pool, binding, &sources, &config.sources, &context).await?
    } else {
        BTreeMap::new()
    };
    let mut prepared = Vec::new();
    for source in &sources {
        let policy = &config.sources[source.source_id];
        let previous = store
            .checkpoint(source.source_id)
            .await
            .map_err(|e| format!("{e:?}"))?;
        if binding.is_some() {
            let empty = BTreeMap::new();
            verify_checkpoint(
                previous.as_ref(),
                current_heads.get(source.source_id).unwrap_or(&empty),
            )?;
        }
        prepared.push(
            prepare_batch(source, policy, &context, previous.as_ref())
                .map_err(|e| e.to_string())?,
        );
    }
    let checkpoints: Vec<SourceCheckpoint> = prepared
        .iter()
        .map(|batch| batch.checkpoint.clone())
        .collect();
    let release = release_from_checkpoints(&checkpoints, &config.release, config.snapshot_epoch)
        .map_err(|e| e.to_string())?;
    let documents: usize = release.source_revisions.values().map(BTreeMap::len).sum();
    if let Some(binding) = binding {
        let approved: usize = binding.active_ids.values().map(|ids| ids.len()).sum();
        if documents != approved {
            return Err("cutover release pins differ from approved inventory".into());
        }
    }
    let mut batches = Vec::new();
    for (source, batch) in sources.iter().zip(prepared.iter()) {
        let lease = store
            .claim(source.source_id, &config.owner, 60_000)
            .await
            .map_err(|e| format!("{e:?}"))?;
        let receipt = store
            .commit(batch, &lease)
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
    }
    if let Some(binding) = binding {
        let final_heads = target_heads(&pool, binding, &sources, &config.sources, &context).await?;
        for checkpoint in &checkpoints {
            let stored = store
                .checkpoint(&checkpoint.source_id)
                .await
                .map_err(|e| format!("{e:?}"))?;
            if stored.as_ref() != Some(checkpoint) {
                return Err("cutover checkpoint changed before release".into());
            }
            let empty = BTreeMap::new();
            verify_checkpoint(
                stored.as_ref(),
                final_heads.get(&checkpoint.source_id).unwrap_or(&empty),
            )?;
        }
    }
    store
        .publish(&release)
        .await
        .map_err(|e| format!("{e:?}"))?;
    let snapshot = store
        .snapshot(&release.release_id)
        .await
        .map_err(|e| format!("{e:?}"))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        serde_json::from_value(json!({
            "legacy": {
                "socket": PRODUCTION.socket,
                "port": PRODUCTION.port,
                "database": "brain",
                "username": "brain_readonly",
                "auth_env": "BRAIN_LEGACY_READ_AUTH"
            },
            "target": {
                "socket": PRODUCTION.socket,
                "port": PRODUCTION.port,
                "database": "brain",
                "username": "brain_ingest",
                "auth_env": "BRAIN_TARGET_INGEST_AUTH"
            },
            "snapshot_label": "approved-fixture",
            "snapshot_epoch": 1790391478,
            "owner": "fixture",
            "release": {"id_prefix": "fixture", "knowledge_version": "v1", "patch": "p1"},
            "sources": {},
            "production_binding": {
                "approval_ref": format!("sha256:{}", "a".repeat(64)),
                "database_oid": 1,
                "archive_schema_oid": 2,
                "core_schema_oid": 3,
                "schema_sha256": "a".repeat(64),
                "snapshot_sha256": "b".repeat(64),
                "policy_sha256": "c".repeat(64),
                "table_counts": {},
                "active_ids": {},
                "revoked_ids": {},
                "tombstone_ids": {}
            },
            "report": "/tmp/cutover-fixture.json"
        }))
        .unwrap()
    }

    #[test]
    fn production_route_requires_explicit_exact_endpoint_and_distinct_roles() {
        let mut approved = config();
        assert!(route(&approved, PRODUCTION).unwrap());
        approved.production_binding = None;
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.target.socket = "/tmp/wrong-cluster".into();
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.target.database = "brain_other".into();
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.legacy.username = "brain_ingest".into();
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.target.username = "brain_service".into();
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.target.port = 55439;
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.legacy.auth_env = Some("OTHER_SECRET".into());
        assert!(route(&approved, PRODUCTION).is_err());
        let mut approved = config();
        approved.target.database = "brain_pilot_legacy".into();
        assert!(route(&approved, PRODUCTION).is_err());
        approved.production_binding = None;
        assert!(!route(&approved, PRODUCTION).unwrap());
        approved.legacy.database = approved.target.database.clone();
        assert!(route(&approved, PRODUCTION).is_err());
    }

    #[test]
    fn connected_identity_must_match_approved_database_and_schemas() {
        let mut identity = Identity {
            database: "brain".into(),
            username: "brain_readonly".into(),
            address: None,
            database_oid: 1,
            archive_oid: Some(2),
            core_oid: Some(3),
        };
        let approved = config();
        let binding = approved.production_binding.as_ref().unwrap();
        assert!(
            identity
                .verify(PRODUCTION, "brain_readonly", binding)
                .is_ok()
        );
        identity.archive_oid = Some(4);
        assert!(
            identity
                .verify(PRODUCTION, "brain_readonly", binding)
                .is_err()
        );
        identity.archive_oid = Some(2);
        identity.address = Some("127.0.0.1".into());
        assert!(
            identity
                .verify(PRODUCTION, "brain_readonly", binding)
                .is_err()
        );
        identity.address = None;
        identity.username = "brain_service".into();
        assert!(
            identity
                .verify(PRODUCTION, "brain_readonly", binding)
                .is_err()
        );
    }

    #[tokio::test]
    #[ignore = "requires a disposable brain_schema_test database from the isolated brain-serve harness"]
    async fn same_database_archive_to_core_requires_bound_private_snapshot() {
        use brain_contracts::{SourceVisibility, source::origin_from_record};
        use brain_legacy_import::{
            cutover::snapshot_sha256, entity_documents, patch_documents, pg::LEGACY_TABLES,
        };
        use std::{collections::BTreeSet, path::Path};

        let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
        assert!(socket.ends_with("/.core-test-pg") && Path::new(&socket).is_absolute());
        let database = "brain_schema_test";
        let fixture = Destination {
            socket: &socket,
            port: 55439,
            database,
            legacy_role: "brain_core_test",
            target_role: "brain_core_test",
            legacy_auth: "",
            target_auth: "",
            require_auth: false,
        };
        let mut config = config();
        for endpoint in [&mut config.legacy, &mut config.target] {
            endpoint.socket = socket.clone();
            endpoint.port = fixture.port;
            endpoint.database = database.into();
            endpoint.username = "brain_core_test".into();
            endpoint.auth_env = None;
        }
        config.report = Path::new(&socket)
            .parent()
            .unwrap()
            .join("cutover-fixture.json");
        let pool = PgPoolOptions::new()
            .connect_with(options(&config.target).unwrap())
            .await
            .unwrap();
        PgStore::new(pool.clone()).migrate_core().await.unwrap();
        for statement in [
            "CREATE SCHEMA brain_legacy",
            "CREATE TABLE brain_legacy.entities (id bigint PRIMARY KEY, entity_type text NOT NULL, canonical_name text NOT NULL, primary_external_id text, source text, metadata jsonb)",
            "CREATE TABLE brain_legacy.entity_aliases (id bigint PRIMARY KEY, entity_id bigint NOT NULL, alias text NOT NULL, alias_kind text)",
            "CREATE TABLE brain_legacy.patch_events (id bigint PRIMARY KEY, patch_external_id text, patch_title text, patch_url text, posted_at timestamptz, source_kind text, line_index bigint, section text, entity_name text, raw_line text, metadata jsonb)",
            "CREATE TABLE brain_legacy.patch_event_enrichments (id bigint PRIMARY KEY, patch_event_id bigint NOT NULL, stat_name text, old_value text, new_value text, unit text)",
            "INSERT INTO brain_legacy.entities VALUES (25, 'hero', 'Warden', 'hero_25', 'deadlock_assets_api', '{}')",
            "INSERT INTO brain_legacy.patch_events VALUES (1, 'p1', 'Update p1', 'https://example.invalid/p1', to_timestamp(1700000000), 'steam', 0, NULL, 'Warden', 'Warden: first', '{}')",
            "INSERT INTO brain_legacy.patch_events VALUES (2, 'p2', 'Update p2', 'https://example.invalid/p2', to_timestamp(1700000001), 'steam', 0, NULL, 'Warden', 'Warden: second', '{}')",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }
        let mut source = options(&config.legacy).unwrap().connect().await.unwrap();
        let identity = Identity::read(
            &sqlx::query(IDENTITY_SQL)
                .fetch_one(&mut source)
                .await
                .unwrap(),
        )
        .unwrap();
        let read = read_legacy(&mut source).await.unwrap();
        source.close().await.unwrap();
        assert_eq!(read.table_counts.len(), LEGACY_TABLES.len());
        let sources = [
            entity_documents(&read.entities).unwrap(),
            patch_documents(&read.patch_lines).unwrap(),
        ];
        let approval = format!("sha256:{}", "a".repeat(64));
        config.sources = BTreeMap::from([
            (
                ENTITIES_SOURCE.into(),
                SourcePolicyConfig {
                    visibility: SourceVisibility::Public,
                    allowed_scopes: BTreeSet::from(["game.public".into()]),
                    provider_egress_allowed: false,
                    publication_allowed: false,
                    raw_retention_allowed: true,
                    authorization_ref: Some(approval.clone()),
                },
            ),
            (
                PATCHNOTES_SOURCE.into(),
                SourcePolicyConfig {
                    visibility: SourceVisibility::Private,
                    allowed_scopes: BTreeSet::from(["brain.legacy.review".into()]),
                    provider_egress_allowed: false,
                    publication_allowed: false,
                    raw_retention_allowed: true,
                    authorization_ref: Some(approval.clone()),
                },
            ),
        ]);
        let active_ids = sources
            .iter()
            .map(|source| {
                (
                    source.source_id.into(),
                    source
                        .documents
                        .iter()
                        .map(|d| d.logical_id.clone())
                        .collect(),
                )
            })
            .collect();
        let empty_ids = BTreeMap::from([
            (ENTITIES_SOURCE.into(), BTreeSet::new()),
            (PATCHNOTES_SOURCE.into(), BTreeSet::new()),
        ]);
        let mut binding = CutoverBinding {
            approval_ref: approval,
            database_oid: identity.database_oid,
            archive_schema_oid: identity.archive_oid.unwrap(),
            core_schema_oid: identity.core_oid.unwrap(),
            schema_sha256: read.schema_sha256.clone(),
            snapshot_sha256: snapshot_sha256(
                &sources,
                &read,
                &config.snapshot_label,
                config.snapshot_epoch,
            )
            .unwrap(),
            policy_sha256: "0".repeat(64),
            table_counts: read.table_counts.clone(),
            active_ids,
            revoked_ids: empty_ids.clone(),
            tombstone_ids: empty_ids,
        };
        binding.policy_sha256 = binding.policy_sha256(&config.sources).unwrap();
        config.production_binding = Some(binding);
        let mut unbound = config.clone();
        unbound.production_binding = None;
        assert!(run_with_destination(unbound, fixture).await.is_err());
        let mut wrong_schema = config.clone();
        wrong_schema
            .production_binding
            .as_mut()
            .unwrap()
            .archive_schema_oid += 1;
        assert!(run_with_destination(wrong_schema, fixture).await.is_err());
        let empty: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.source_record_heads")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(empty, 0);
        let first = run_with_destination(config.clone(), fixture).await.unwrap();
        assert_eq!(first["release_documents"], 3);
        let private_head: serde_json::Value = sqlx::query_scalar(
            "SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2",
        )
        .bind(PATCHNOTES_SOURCE)
        .bind("patch/p1")
        .fetch_one(&pool)
        .await
        .unwrap();
        let private_head: SourceRecordV2 = serde_json::from_value(private_head).unwrap();
        assert_eq!(private_head.visibility, SourceVisibility::Private);
        assert!(
            !origin_from_record(&private_head)
                .unwrap()
                .policy
                .publication_allowed
        );
        let repeated = run_with_destination(config.clone(), fixture).await.unwrap();
        assert_eq!(first["release_id"], repeated["release_id"]);
        assert!(
            repeated["sources"]
                .as_array()
                .unwrap()
                .iter()
                .all(|source| source["changed_records"] == 0)
        );
        sqlx::query("INSERT INTO brain_legacy.patch_events VALUES (3, 'p3', 'Update p3', 'https://example.invalid/p3', to_timestamp(1700000002), 'steam', 0, NULL, 'Warden', 'new', '{}')")
            .execute(&pool)
            .await
            .unwrap();
        assert!(run_with_destination(config, fixture).await.is_err());
        let releases: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.corpus_releases_v1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(releases, 1);
        pool.close().await;
    }
}
