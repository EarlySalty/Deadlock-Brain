use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::BTreeSet, path::Path, time::Duration};

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DataJob {
    AssetsBuildData,
    SheetSync,
}

impl DataJob {
    fn id(self) -> &'static str {
        match self {
            Self::AssetsBuildData => "assets_build_data",
            Self::SheetSync => "sheet_sync",
        }
    }

    fn commands(self) -> &'static [&'static [&'static str]] {
        match self {
            Self::AssetsBuildData => &[
                &["pull", "assets"],
                &["pull", "build-data", "--hero", "all"],
            ],
            Self::SheetSync => &[&["refresh-sheet"]],
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataJobConfig {
    pub job: DataJob,
    pub interval_ms: u64,
    pub timeout_ms: u64,
}

pub fn validate(jobs: &[DataJobConfig]) -> Result<()> {
    let mut seen = BTreeSet::new();
    ensure!(jobs.len() <= 2, "data_job_count");
    for job in jobs {
        ensure!(
            seen.insert(job.job)
                && (3_600_000..=86_400_000).contains(&job.interval_ms)
                && (60_000..=3_600_000).contains(&job.timeout_ms)
                && job.timeout_ms < job.interval_ms,
            "data_job_schedule"
        );
    }
    Ok(())
}

pub async fn run_due(
    pool: &sqlx::PgPool,
    jobs: &[DataJobConfig],
    infisical_config: &Path,
    retry_delay_ms: u64,
) -> Result<Vec<Value>> {
    run_due_with(pool, jobs, retry_delay_ms, |job| async move {
        let mut results = Vec::new();
        for arguments in job.job.commands() {
            let mut args = vec![
                "--config".into(),
                infisical_config
                    .to_str()
                    .context("infisical_config_path")?
                    .into(),
                "--".into(),
                "/opt/deadlock-brain/current/bin/deadlock-brain".into(),
            ];
            args.extend(arguments.iter().map(|argument| (*argument).to_owned()));
            let output = crate::process::run(
                Path::new("/opt/deadlock-brain/current/bin/deadlock-brain-secret-exec"),
                &args,
                Path::new("/"),
                &[],
                job.timeout_ms,
                1024 * 1024,
            )
            .await?;
            let summary = result_summary(arguments, &output)?;
            results.push(json!({"command":arguments,"output_sha256":crate::digest(&output),"summary":summary}));
        }
        Ok(json!({"commands":results}))
    })
    .await
}

fn result_summary(arguments: &[&str], output: &[u8]) -> Result<Value> {
    let value: Value =
        serde_json::from_slice(output).map_err(|_| anyhow::anyhow!("data_job_output_invalid"))?;
    match arguments {
        ["pull", "assets"] => {
            ensure!(
                value["mirror_complete"] == true
                    && value["client_version"]
                        .as_i64()
                        .is_some_and(|version| version > 0),
                "assets_mirror_incomplete"
            );
            Ok(json!({"client_version":value["client_version"],"mirror_complete":true}))
        }
        ["pull", "build-data", "--hero", "all"] => {
            let heroes = value["heroes"]
                .as_array()
                .context("build_data_heroes_missing")?;
            ensure!(
                !heroes.is_empty()
                    && value["item_catalog_rows"]
                        .as_u64()
                        .is_some_and(|rows| rows > 0)
                    && value["hero_catalog_rows"]
                        .as_u64()
                        .is_some_and(|rows| rows > 0),
                "build_data_incomplete"
            );
            Ok(
                json!({"client_version":value["client_version"],"heroes":heroes.len(),"item_catalog_rows":value["item_catalog_rows"],"hero_catalog_rows":value["hero_catalog_rows"]}),
            )
        }
        ["refresh-sheet"] => {
            ensure!(
                value["pull"]["tabs"].as_u64().is_some_and(|tabs| tabs > 0)
                    && value["pull"]["rows"].as_u64().is_some_and(|rows| rows > 0)
                    && [
                        "normalize_entities",
                        "normalize_sheet_stats",
                        "normalize_sheet_tabs"
                    ]
                    .iter()
                    .all(|key| value[*key].is_object()),
                "sheet_refresh_incomplete"
            );
            Ok(json!({"tabs":value["pull"]["tabs"],"rows":value["pull"]["rows"]}))
        }
        _ => anyhow::bail!("data_job_command_unregistered"),
    }
}

async fn run_due_with<'a, F, Fut>(
    pool: &sqlx::PgPool,
    jobs: &'a [DataJobConfig],
    retry_delay_ms: u64,
    dispatch: F,
) -> Result<Vec<Value>>
where
    F: Fn(&'a DataJobConfig) -> Fut,
    Fut: std::future::Future<Output = Result<Value>>,
{
    validate(jobs)?;
    let mut outcomes = Vec::new();
    for job in jobs {
        let id = job.job.id();
        sqlx::query(
            "INSERT INTO brain.maintenance_data_jobs_v1(id) VALUES($1) ON CONFLICT DO NOTHING",
        )
        .bind(id)
        .execute(pool)
        .await?;
        let fence: Option<i64> = sqlx::query_scalar("UPDATE brain.maintenance_data_jobs_v1 SET fence=fence+1,lease_until=clock_timestamp()+($2::bigint*interval '1 millisecond'),last_started_at=clock_timestamp(),updated_at=now() WHERE id=$1 AND available_at<=clock_timestamp() AND (lease_until IS NULL OR lease_until<=clock_timestamp()) AND fence<9223372036854775807 RETURNING fence")
            .bind(id).bind(i64::try_from(job.timeout_ms + 60_000)?).fetch_optional(pool).await?;
        let Some(fence) = fence else {
            outcomes.push(json!({"job":id,"status":"not_due_or_leased"}));
            continue;
        };
        let result =
            tokio::time::timeout(Duration::from_millis(job.timeout_ms), dispatch(job)).await;
        let (status, error, result, delay) = match result {
            Ok(Ok(result)) => ("success", None, Some(result), job.interval_ms),
            Ok(Err(_)) => ("failed", Some("DATA_JOB_FAILED"), None, retry_delay_ms),
            Err(_) => ("failed", Some("DATA_JOB_TIMEOUT"), None, retry_delay_ms),
        };
        let changed = sqlx::query("UPDATE brain.maintenance_data_jobs_v1 SET lease_until=NULL,available_at=clock_timestamp()+($3::bigint*interval '1 millisecond'),last_finished_at=clock_timestamp(),last_success_at=CASE WHEN $4::text IS NULL THEN clock_timestamp() ELSE last_success_at END,last_error_code=$4,result_json=$5,updated_at=now() WHERE id=$1 AND fence=$2 AND lease_until>clock_timestamp()")
            .bind(id).bind(fence).bind(i64::try_from(delay)?).bind(error).bind(result)
            .execute(pool).await?;
        ensure!(changed.rows_affected() == 1, "data_job_lease_expired");
        outcomes.push(json!({"job":id,"status":status,"error_code":error}));
    }
    Ok(outcomes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn incomplete_outputs_do_not_count_as_success() {
        assert!(result_summary(
            &["pull", "assets"],
            br#"{"client_version":6774,"mirror_complete":false}"#
        )
        .is_err());
        assert!(result_summary(
            &["pull", "build-data", "--hero", "all"],
            br#"{"heroes":[],"item_catalog_rows":1,"hero_catalog_rows":1}"#
        )
        .is_err());
        assert!(result_summary(&["refresh-sheet"], br#"{"pull":{"tabs":0,"rows":0}}"#).is_err());
        assert!(result_summary(
            &["pull", "assets"],
            br#"{"client_version":6774,"mirror_complete":true}"#
        )
        .is_ok());
    }

    #[test]
    fn schedules_are_bounded_unique_and_data_only() {
        let job = || DataJobConfig {
            job: DataJob::SheetSync,
            interval_ms: 14_400_000,
            timeout_ms: 600_000,
        };
        assert!(validate(&[job()]).is_ok());
        assert!(validate(&[job(), job()]).is_err());
        let mut invalid = job();
        invalid.timeout_ms = invalid.interval_ms;
        assert!(validate(&[invalid]).is_err());
        assert_eq!(DataJob::SheetSync.commands(), &[&["refresh-sheet"]]);
    }

    #[tokio::test]
    #[ignore = "requires isolated PostgreSQL Unix socket: BRAIN_MAINTENANCE_TEST_PG_SOCKET"]
    async fn postgres_non_rebuild_entities_upsert_and_preserve_ids_and_aliases() -> Result<()> {
        let socket = std::env::var("BRAIN_MAINTENANCE_TEST_PG_SOCKET")?;
        ensure!(socket.starts_with("/tmp/") && socket.ends_with("/.maintenance-test-pg"));
        let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
            .host(&socket)
            .port(55447)
            .username("brain_maintenance_test")
            .database("postgres")
            .password("");
        let owner = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(options.clone())
            .await?;
        let identity: (String, String) =
            sqlx::query_as("SELECT current_user::text,current_setting('data_directory')")
                .fetch_one(&owner)
                .await?;
        ensure!(identity == ("brain_maintenance_test".into(), socket));
        let fixture = tempfile::tempdir()?;
        let database = format!(
            "o1_entities_{}",
            &crate::digest(fixture.path().as_os_str().as_encoded_bytes())[..16]
        );
        sqlx::query(&format!("CREATE DATABASE {database} TEMPLATE template0"))
            .execute(&owner)
            .await?;
        let result = async {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect_with(options.database(&database))
                .await?;
            let result = non_rebuild_entity_contract(&pool).await;
            pool.close().await;
            result
        }
        .await;
        sqlx::query(&format!("DROP DATABASE {database}"))
            .execute(&owner)
            .await?;
        owner.close().await;
        result
    }

    async fn non_rebuild_entity_contract(pool: &sqlx::PgPool) -> Result<()> {
        sqlx::raw_sql(
            "CREATE SCHEMA brain;
            CREATE TABLE brain.source_documents(id bigserial PRIMARY KEY,external_id text NOT NULL);
            CREATE TABLE brain.entity_snapshots(
                id bigserial PRIMARY KEY,source text NOT NULL,entity_type text NOT NULL,
                external_id text NOT NULL,canonical_name text,payload jsonb NOT NULL,
                source_document_id bigint REFERENCES brain.source_documents(id));
            CREATE TABLE brain.entities(
                id bigserial PRIMARY KEY,entity_type text NOT NULL,canonical_name text NOT NULL,
                primary_external_id text,source text,first_snapshot_id bigint,
                metadata jsonb NOT NULL DEFAULT '{}',created_at timestamptz NOT NULL DEFAULT now(),
                updated_at timestamptz NOT NULL DEFAULT now(),UNIQUE(entity_type,canonical_name));
            CREATE TABLE brain.entity_aliases(
                entity_id bigint NOT NULL REFERENCES brain.entities(id) ON DELETE CASCADE,
                alias text NOT NULL,alias_norm text NOT NULL,alias_kind text NOT NULL,
                source text,external_id text,snapshot_id bigint,created_at timestamptz NOT NULL,
                UNIQUE(entity_id,alias_norm,alias_kind));
            INSERT INTO brain.entities(entity_type,canonical_name) VALUES
                ('hero','Existing Hero'),('hero','Unrelated Hero');
            INSERT INTO brain.entity_aliases(entity_id,alias,alias_norm,alias_kind,created_at)
                SELECT id,'Preserved Alias','preserved alias','manual',now()
                FROM brain.entities WHERE canonical_name='Existing Hero';",
        )
        .execute(pool)
        .await?;
        let before: Vec<(String, String, i64)> = sqlx::query_as(
            "SELECT entity_type,canonical_name,id FROM brain.entities ORDER BY entity_type,canonical_name",
        )
        .fetch_all(pool)
        .await?;
        for (kind, external_id, name, payload) in [
            (
                "hero",
                "101",
                "Existing Snapshot",
                json!({"id":101,"name":"Existing Hero","class_name":"hero_existing","player_selectable":true,"disabled":false}),
            ),
            (
                "hero",
                "102",
                "New Hero Snapshot",
                json!({"id":102,"name":"New Hero","class_name":"hero_new","player_selectable":true,"disabled":false}),
            ),
            (
                "item",
                "201",
                "New Item Snapshot",
                json!({"id":201,"name":"New Item","class_name":"upgrade_new","type":"upgrade","shopable":true,"disabled":false,"item_slot_type":"weapon","item_tier":1,"cost":500}),
            ),
        ] {
            sqlx::query("INSERT INTO brain.entity_snapshots(source,entity_type,external_id,canonical_name,payload) VALUES('deadlock_assets_api',$1,$2,$3,$4)")
                .bind(kind).bind(external_id).bind(name).bind(payload).execute(pool).await?;
        }
        let first = dbrain_normalize::normalize_entities(pool, false).await?;
        ensure!(first["deleted"] == json!({"aliases":0,"entities":0}));
        ensure!(first["by_type"] == json!({"hero":2,"item":1}));
        let after_first: Vec<(String, String, i64)> = sqlx::query_as(
            "SELECT entity_type,canonical_name,id FROM brain.entities ORDER BY entity_type,canonical_name",
        )
        .fetch_all(pool)
        .await?;
        ensure!(after_first.len() == 4 && before.iter().all(|row| after_first.contains(row)));
        ensure!(after_first
            .iter()
            .any(|(kind, name, _)| kind == "hero" && name == "New Hero"));
        ensure!(after_first
            .iter()
            .any(|(kind, name, _)| kind == "item" && name == "New Item"));
        let aliases_before: Vec<(i64, String, String)> = sqlx::query_as(
            "SELECT entity_id,alias,alias_kind FROM brain.entity_aliases ORDER BY entity_id,alias,alias_kind",
        )
        .fetch_all(pool)
        .await?;
        sqlx::query("UPDATE brain.entity_snapshots SET canonical_name='Added Snapshot Alias' WHERE external_id='101'")
            .execute(pool).await?;
        let second = dbrain_normalize::normalize_entities(pool, false).await?;
        ensure!(second["deleted"] == json!({"aliases":0,"entities":0}));
        let after_second: Vec<(String, String, i64)> = sqlx::query_as(
            "SELECT entity_type,canonical_name,id FROM brain.entities ORDER BY entity_type,canonical_name",
        )
        .fetch_all(pool)
        .await?;
        ensure!(after_first == after_second);
        let aliases_after: Vec<(i64, String, String)> = sqlx::query_as(
            "SELECT entity_id,alias,alias_kind FROM brain.entity_aliases ORDER BY entity_id,alias,alias_kind",
        )
        .fetch_all(pool)
        .await?;
        ensure!(aliases_before.iter().all(|row| aliases_after.contains(row)));
        let existing_id = before
            .iter()
            .find(|(_, name, _)| name == "Existing Hero")
            .context("existing_entity_missing")?
            .2;
        ensure!(aliases_after.contains(&(existing_id, "Preserved Alias".into(), "manual".into())));
        ensure!(aliases_after.contains(&(
            existing_id,
            "Existing Snapshot".into(),
            "snapshot_name".into()
        )));
        ensure!(aliases_after.contains(&(
            existing_id,
            "Added Snapshot Alias".into(),
            "snapshot_name".into()
        )));
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires isolated PostgreSQL Unix socket: BRAIN_MAINTENANCE_TEST_PG_SOCKET"]
    async fn postgres_schedule_fences_concurrent_runs_and_retains_last_success() {
        let socket = std::env::var("BRAIN_MAINTENANCE_TEST_PG_SOCKET").unwrap();
        assert!(socket.ends_with("/.maintenance-test-pg"));
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(&socket)
                    .port(55447)
                    .username("brain_maintenance_test")
                    .database("postgres")
                    .password(""),
            )
            .await
            .unwrap();
        sqlx::query("TRUNCATE brain.maintenance_data_jobs_v1")
            .execute(&pool)
            .await
            .unwrap();
        let jobs = [DataJobConfig {
            job: DataJob::SheetSync,
            interval_ms: 14_400_000,
            timeout_ms: 600_000,
        }];
        let calls = AtomicUsize::new(0);
        let dispatch = |_| async {
            calls.fetch_add(1, Ordering::SeqCst);
            tokio::task::yield_now().await;
            Ok(json!({"synthetic":true}))
        };
        let (left, right) = tokio::join!(
            run_due_with(&pool, &jobs, 3_600_000, dispatch),
            run_due_with(&pool, &jobs, 3_600_000, dispatch)
        );
        let outcomes: Vec<_> = left.unwrap().into_iter().chain(right.unwrap()).collect();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter(|value| value["status"] == "success")
                .count(),
            1
        );
        assert_eq!(
            outcomes
                .iter()
                .filter(|value| value["status"] == "not_due_or_leased")
                .count(),
            1
        );
        let success: bool = sqlx::query_scalar("SELECT last_success_at IS NOT NULL AND lease_until IS NULL AND available_at>clock_timestamp() FROM brain.maintenance_data_jobs_v1 WHERE id='sheet_sync'").fetch_one(&pool).await.unwrap();
        assert!(success);
        sqlx::query("UPDATE brain.maintenance_data_jobs_v1 SET available_at='2000-01-01' WHERE id='sheet_sync'").execute(&pool).await.unwrap();
        let failed = run_due_with(&pool, &jobs, 3_600_000, |_| async {
            anyhow::bail!("synthetic_failure")
        })
        .await
        .unwrap();
        assert_eq!(failed[0]["status"], "failed");
        let retained: bool = sqlx::query_scalar("SELECT last_success_at IS NOT NULL AND last_error_code='DATA_JOB_FAILED' AND available_at>clock_timestamp() FROM brain.maintenance_data_jobs_v1 WHERE id='sheet_sync'").fetch_one(&pool).await.unwrap();
        assert!(retained);
    }
}
