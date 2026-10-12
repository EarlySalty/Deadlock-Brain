use super::super::{tool_pool, PgTools};
use super::{definition, QUERY};
use brain_contracts::{
    AuthorizedContext, Budget, PinnedGameContext, PortError, Principal, Query, ToolCall,
    ToolExecution, ToolExecutionPort, ToolName, ToolRequest,
};
use serde_json::{json, Value};
use sqlx::postgres::PgConnectOptions;
use std::{collections::BTreeSet, path::PathBuf, time::Duration};

struct ScratchPg {
    directory: PathBuf,
    _root: tempfile::TempDir,
}
impl ScratchPg {
    fn start() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-p3-timer-pg-")
            .tempdir()
            .unwrap();
        let directory = root.path().to_owned();
        std::fs::create_dir(directory.join("socket")).unwrap();
        let instance = Self {
            directory,
            _root: root,
        };
        for (program, arguments) in [
            (
                "initdb",
                vec![
                    "-D".into(),
                    instance.directory.join("data").into_os_string(),
                    "-A".into(),
                    "trust".into(),
                    "-U".into(),
                    "brain_core_test".into(),
                    "--no-locale".into(),
                    "--encoding=UTF8".into(),
                ],
            ),
            (
                "pg_ctl",
                vec![
                    "-D".into(),
                    instance.directory.join("data").into_os_string(),
                    "-l".into(),
                    instance.directory.join("postgres.log").into_os_string(),
                    "-o".into(),
                    format!(
                        "-k {} -p 55439 -c listen_addresses=''",
                        instance.directory.join("socket").display()
                    )
                    .into(),
                    "-w".into(),
                    "start".into(),
                ],
            ),
        ] {
            let output =
                std::process::Command::new(format!("/usr/lib/postgresql/16/bin/{program}"))
                    .args(arguments)
                    .output()
                    .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        instance
    }
    fn options(&self, user: &str) -> PgConnectOptions {
        PgConnectOptions::new_without_pgpass()
            .host(self.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username(user)
            .database("postgres")
    }
}
impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output();
    }
}

struct NoLegacy;
impl ToolExecutionPort for NoLegacy {
    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        _: &str,
        _: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        panic!("Timer query must not reach legacy tools");
    }
}

const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','fixture','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://timers');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'commit-a','fixture','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (1,'misc-data.json','commit-a',sha256(convert_to('{\"box\":{\"spawn\":180}}','UTF8')),'application/json',convert_to('{\"box\":{\"spawn\":180}}','UTF8'));
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (1,'misc-data.json',1,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'commit-a','fixture','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'json_pointer','/box/spawn',sha256(convert_to('180','UTF8')),'fixture','v1' FROM generate_series(1,40);
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('timer','soul_box',1);
    INSERT INTO canon.entity(kind,key,identity_receipt_id) SELECT 'hero','fixture_'||n,1 FROM generate_series(1,2000) n;
    INSERT INTO canon.stat(key,section,unit,value_type) VALUES ('timer.first_spawn_seconds','stats','s','number'),('fixture.background','stats','s','number');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,condition,receipt_id,valid_from_revision_id)
        SELECT 1,'timer.first_spawn_seconds',1,'misc-data.json',180,jsonb_build_object('source_entity',repeat('x',500)||n),n,1 FROM generate_series(1,40) n;
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        SELECT id,'fixture.background',1,'misc-data.json',180,1,1 FROM canon.entity WHERE kind='hero';
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision='commit-a' WHERE id=1;
    COMMIT; ANALYZE canon.entity; ANALYZE canon.fact;";

fn indexed_timer_lookup(plan: &Value) -> bool {
    (plan["Relation Name"] == "entity"
        && matches!(
            plan["Node Type"].as_str(),
            Some("Index Scan" | "Index Only Scan")
        )
        && matches!(
            plan["Index Name"].as_str(),
            Some("timer_key_idx" | "entity_kind_key_key")
        )
        && plan["Index Cond"]
            .as_str()
            .is_some_and(|condition| condition.contains("key")))
        || plan["Plans"]
            .as_array()
            .is_some_and(|children| children.iter().any(indexed_timer_lookup))
}

#[test]
fn real_pg_timer_tool_is_bounded_indexed_receipted_and_scope_guarded() {
    let pg = ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let admin = runtime.block_on(async {
        let pool = sqlx::postgres::PgPoolOptions::new().max_connections(1)
            .connect_with(pg.options("brain_core_test")).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN;
            CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; CREATE ROLE brain_tool NOLOGIN;
            GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE;
            GRANT CREATE ON DATABASE postgres TO brain_migrate;").execute(&pool).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        sqlx::query("SET ROLE brain_migrate").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"))
            .execute(&mut *connection).await.unwrap();
        sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES(1)")
            .execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p3-timers-v1.sql"))
            .execute(&mut *connection).await.unwrap();
        sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES(2)")
            .execute(&mut *connection).await.unwrap();
        sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(SEED).execute(&mut *connection).await.unwrap();
        drop(connection);
        pool
    });
    let pool = {
        let _entered = runtime.enter();
        tool_pool(pg.options("brain_readonly"), Duration::from_secs(2))
    };
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let query: Query =
        serde_json::from_value(json!({"request_id":"timer-pg","conversation_id":"timer-pg",
        "text":"Wann spawnen Seelenboxen?","profile":"explain"}))
        .unwrap();
    let authorization = AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: query.conversation_id.clone(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    };
    let request = |arguments| {
        ToolCall {
            id: "fixture".into(),
            name: ToolName::GetTimer,
            arguments,
        }
        .validate(&[definition()])
        .unwrap()
    };
    let timer = request(json!({"timer":"soul_box"}));
    let execution = tools
        .execute(
            &query,
            &authorization.with_request_deadline(),
            None,
            "timer",
            &timer,
        )
        .unwrap();
    assert!(!execution.result.is_error);
    assert!(serde_json::to_vec(&execution.result).unwrap().len() <= 16384);
    let facts = execution.result.result["facts"].as_array().unwrap();
    assert!(!facts.is_empty() && facts.len() <= 32);
    assert_eq!(execution.result.result["truncated"], true);
    assert!(facts.iter().all(|fact| fact["value"] == 180
        && fact["unit"] == "s"
        && fact["patch"].is_null()
        && fact["source_revision"] == "commit-a"
        && fact["citation"].is_string()
        && fact["condition"].is_object()));
    assert_eq!(execution.dependencies[0].evidence.len(), facts.len());
    assert!(execution.dependencies[0].evidence.iter().all(
        |evidence| serde_json::from_str::<Value>(&evidence.content).unwrap()["raw_object_id"] == 1
    ));
    for arguments in [
        json!({"timer":"urn"}),
        json!({"timer":"soul_box","patch":"historical"}),
    ] {
        let missing = tools
            .execute(
                &query,
                &authorization.with_request_deadline(),
                None,
                "missing",
                &request(arguments),
            )
            .unwrap();
        assert!(
            missing.result.is_error
                && missing.result.evidence_ids.is_empty()
                && missing.dependencies.is_empty()
        );
    }
    let mut denied = authorization.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        tools.execute(&query, &denied, None, "denied", &timer),
        Err(PortError::PermissionDenied(_))
    ));
    let mut wrong_conversation = authorization.clone();
    wrong_conversation.conversation_id = "another".into();
    assert!(matches!(
        tools.execute(&query, &wrong_conversation, None, "denied", &timer),
        Err(PortError::PermissionDenied(_))
    ));
    runtime.block_on(async {
        let mut connection = pool.acquire().await.unwrap();
        let plan: Value = sqlx::query_scalar(&format!("EXPLAIN (FORMAT JSON) {QUERY}"))
            .bind("soul_box")
            .fetch_one(&mut *connection)
            .await
            .unwrap();
        assert!(indexed_timer_lookup(&plan[0]["Plan"]), "{plan}");
        assert!(sqlx::query("SELECT body FROM raw.object LIMIT 1")
            .execute(&mut *connection)
            .await
            .is_err());
        drop(connection);
        pool.close().await;
        admin.close().await;
    });
}
