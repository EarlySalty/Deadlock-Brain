use super::*;
use crate::service::answer_trace::{
    PgTraceWriter, TraceRecord, TraceSink, TracedKernel, TracedTools,
};
use brain_contracts::{
    AnswerProfile, AnswerProviderPort, AnswerStatus, Budget, ModelBlock, Principal, ProviderAnswer,
    ProviderFinishReason, ProviderTurn, RetrievalPort, ToolConversation,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, GameContextResolver, Kernel};
use std::sync::{Arc, Mutex};

struct ScratchPg {
    directory: std::path::PathBuf,
    _root: tempfile::TempDir,
}

impl ScratchPg {
    fn start() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-p1-tools-")
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

fn query() -> Query {
    serde_json::from_value(
        json!({"request_id":"pg-tools", "conversation_id":"pg-tools",
        "text":"Baba Steckbrief", "profile":"explain"}),
    )
    .unwrap()
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "pg-tools".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10_000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    }
}

fn request(name: ToolName, arguments: Value) -> ToolRequest {
    ToolCall {
        id: "fixture".into(),
        name,
        arguments,
    }
    .validate(&definitions())
    .unwrap()
}

#[derive(Default)]
struct Delegate(Arc<Mutex<Vec<String>>>);

impl ToolExecutionPort for Delegate {
    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(vec![ToolDefinition {
            name: ToolName::ServerKnowledge,
            description: "Fixture".into(),
            input_schema: json!({"type":"object","properties":{"question":{"type":"string"}},"required":["question"],"additionalProperties":false}),
        }])
    }

    fn required_calls(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        self.0.lock().unwrap().push("required".into());
        Ok(Vec::new())
    }

    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        self.0.lock().unwrap().push("execute".into());
        Ok(error_execution(id, request.name(), "fixture", "Fixture"))
    }

    fn execute_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        self.0.lock().unwrap().push("accounted".into());
        Ok(Accounted {
            value: self.execute(query, context, pin, id, request).unwrap(),
            accounting: UsageAccounting {
                observed: Usage {
                    cost_micros: 7,
                    ..Default::default()
                },
                reserved: Usage {
                    input_tokens: 8,
                    ..Default::default()
                },
                unaccounted: true,
            },
        })
    }

    fn validate_dependencies(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        assert!(dependencies
            .iter()
            .all(|dependency| !canonical(dependency.request.name())));
        self.0.lock().unwrap().push("dependencies".into());
        Ok(())
    }

    fn validate_build_plan(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &PinnedGameContext,
        _: &BuildPlanRequest,
        _: &ToolExecution,
        _: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        self.0.lock().unwrap().push("build".into());
        Err(PortError::PermissionDenied("Fixture".into()))
    }
}

struct NoSnapshot;

impl RetrievalPort for NoSnapshot {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        panic!("Explain tools must not read a snapshot");
    }
}

impl GameContextResolver for NoSnapshot {
    fn resolve(
        &self,
        _: &Query,
        _: &AuthorizedContext,
    ) -> Result<Option<PinnedGameContext>, PortError> {
        Ok(None)
    }
    fn validate(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<(), PortError> {
        assert!(pin.is_none());
        Ok(())
    }
}

struct FixtureProvider;

impl AnswerProviderPort for FixtureProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("Tool conversation required");
    }

    fn answer_turn(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> Result<ProviderTurn, PortError> {
        let call = match conversation.messages.len() {
            0 => ToolCall {
                id: "find".into(),
                name: ToolName::FindEntity,
                arguments: json!({"text":"Baba Jäger","kind":"hero"}),
            },
            2 => ToolCall {
                id: "hero".into(),
                name: ToolName::GetHero,
                arguments: json!({"hero":1,"sections":["stats"]}),
            },
            4 => {
                return Ok(ProviderAnswer {
                    text: "Synthetic grounded answer".into(),
                    cited_evidence_ids: vec!["5".into()],
                    usage: Usage {
                        network_rounds: 1,
                        input_tokens: 10,
                        output_tokens: 3,
                        ..Usage::default()
                    },
                }
                .into())
            }
            _ => panic!("Unexpected conversation"),
        };
        Ok(ProviderTurn::ToolCalls {
            blocks: vec![ModelBlock::ToolUse { call }],
            finish_reason: ProviderFinishReason::ToolUse,
            usage: Usage {
                network_rounds: 1,
                input_tokens: 10,
                output_tokens: 3,
                ..Usage::default()
            },
        })
    }
}

#[test]
fn canonical_metadata_does_not_interpret_source_value_or_condition_keys() {
    assert_eq!(normalize("  BABA\u{2003}  JÄGER  "), "baba jäger");
    let value = json!({"identity":{"citation":"1","patch":null,"source_revision":"a"},
        "stats":{"facts":[{"stat_key":"stats.any","value":{"citation":"source-value"},
        "condition":{"citation":"source-condition"},"citation":"2","patch":null,"source_revision":"a"}]}});
    let mut ids = BTreeSet::new();
    citations(&value, &mut ids).unwrap();
    assert_eq!(ids, BTreeSet::from([1, 2]));
    let mut changed = value;
    changed["stats"]["facts"][0]["condition"] = Value::Null;
    assert!(citations(&changed, &mut ids).is_err());
}

#[test]
fn complete_rows_are_trimmed_without_changing_values_or_conditions() {
    let first = json!({"stat_key":"stats.first","value":"x".repeat(512),"condition":{"rank":3},
        "citation":"1","patch":null,"source_revision":"a"});
    let mut value =
        json!({"stats":{"facts":[first.clone(),first.clone()],"omitted_count":4,"truncated":true}});
    assert!(trim_row(&mut value));
    assert_eq!(value["stats"]["facts"], json!([first]));
    assert_eq!(value["stats"]["omitted_count"], 5);
    let result = ToolResult {
        call_id: "\"".repeat(512),
        name: ToolName::GetHero,
        result: json!({"value":"x".repeat(MAX_BYTES-200)}),
        evidence_ids: vec!["9".repeat(19); 100],
        is_error: false,
    };
    assert!(!fits(&result, 100).unwrap());
    assert!(!fits(
        &ToolResult {
            result: json!({}),
            ..result
        },
        101
    )
    .unwrap());
}

const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','fixture','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://tools');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'commit-a','fixture','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (1,'hero.kv','commit-a',decode(repeat('01',32),'hex'),'text/plain',convert_to('synthetic','UTF8'));
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (1,'hero.kv',1,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'commit-a','fixture','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'kv_path','field-'||n,decode(repeat('02',32),'hex'),'fixture','v1' FROM generate_series(1,120) n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','hero_baba',1),('hero','other',2),('ability','baba_1',3);
    INSERT INTO canon.stat(key,section,unit,value_type) VALUES ('identity.name','identity',NULL,'text'),('stats.max_health','stats','unknown-game-unit','number'),('ability.damage','ability','damage','number');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES (1,'identity.name',1,'hero.kv','Baba',4,1);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,condition,receipt_id,valid_from_revision_id)
        VALUES (1,'stats.max_health',1,'hero.kv',100,'{\"rank\":3,\"citation\":\"source-condition\"}',5,1),(3,'ability.damage',1,'hero.kv',25,'{}',6,1);
    INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES (1,'baba','en',4,1,'hero.kv',1),(1,'baba','de',4,1,'hero.kv',1),(2,'baba','en',7,1,'hero.kv',1),(1,'baba jäger','de',8,1,'hero.kv',1);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',3,1,9,1,'hero.kv',1);
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision='commit-a' WHERE id=1;
    COMMIT;";

#[test]
fn real_pg_tools_roles_validity_bounded_results_and_trace_writer() {
    let pg = ScratchPg::start();
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let options = |user: &str| {
        PgConnectOptions::new_without_pgpass()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username(user)
            .database("postgres")
    };
    let admin=executor.block_on(async {
        let pool=sqlx::postgres::PgPoolOptions::new().max_connections(1).connect_with(options("brain_core_test")).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN;
            CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN;
            CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
            GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE;
            GRANT CREATE ON DATABASE postgres TO brain_migrate;").execute(&pool).await.unwrap();
        let mut connection=pool.acquire().await.unwrap();
        sqlx::query("SET ROLE brain_migrate").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql")).execute(&mut *connection).await.unwrap();
        sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES(1)").execute(&mut *connection).await.unwrap();
        sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(SEED).execute(&mut *connection).await.unwrap();
        drop(connection);
        pool
    });
    let pool = {
        let _entered = executor.enter();
        tool_pool(options("brain_readonly"), Duration::from_secs(2))
    };
    let delegate = Delegate::default();
    let delegated = delegate.0.clone();
    let tools = PgTools::new(delegate, pool.clone(), executor.handle().clone());
    let authorization = context();
    let context = authorization.with_request_deadline();
    let query = query();
    assert_eq!(query.profile, AnswerProfile::Explain);
    let find = request(
        ToolName::FindEntity,
        json!({"text":"Baba","kind":"hero","limit":1}),
    );
    let found = tools
        .execute(&query, &context, None, "find", &find)
        .unwrap();
    assert_eq!(found.result.result["match"], "exact");
    assert_eq!(found.result.result["ambiguous"], true);
    assert_eq!(found.result.result["resolved"], false);
    assert_eq!(found.result.result["entities"].as_array().unwrap().len(), 1);
    assert_eq!(found.result.result["omitted_count"], 1);
    let exact = request(
        ToolName::FindEntity,
        json!({"text":" BABA  JÄGER ","kind":"hero"}),
    );
    let exact = tools
        .execute(&query, &context, None, "exact", &exact)
        .unwrap();
    assert_eq!(exact.result.result["resolved"], true);
    let fuzzy = request(
        ToolName::FindEntity,
        json!({"text":"baba jäge","kind":"hero"}),
    );
    let suggestion = tools
        .execute(&query, &context, None, "fuzzy", &fuzzy)
        .unwrap();
    assert_eq!(suggestion.result.result["match"], "suggestion");
    assert_eq!(suggestion.result.result["resolved"], false);
    let hero = request(ToolName::GetHero, json!({"hero":1,"sections":["stats"]}));
    let profile = tools
        .execute(&query, &context, None, "hero", &hero)
        .unwrap();
    assert!(profile.result.result.get("abilities").is_none());
    assert!(profile.result.result.get("weapon").is_none());
    assert_eq!(
        profile.result.result["stats"]["facts"][0]["condition"],
        json!({"rank":3,"citation":"source-condition"})
    );
    assert_eq!(
        profile.result.result["stats"]["facts"][0]["source_revision"],
        "commit-a"
    );
    assert!(profile.result.result["stats"]["facts"][0]["patch"].is_null());
    for evidence in &found.dependencies[0].evidence {
        assert_eq!(
            profile.dependencies[0]
                .evidence
                .iter()
                .find(|item| item.evidence_id == evidence.evidence_id),
            Some(evidence)
        );
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &profile.dependencies, purpose)
            .unwrap();
    }
    let historical = request(ToolName::GetHero, json!({"hero":1,"patch":"old"}));
    let error = tools
        .execute(&query, &context, None, "historical", &historical)
        .unwrap();
    assert!(
        error.result.is_error
            && error.result.evidence_ids.is_empty()
            && error.dependencies.is_empty()
    );
    let mut old_query = query.clone();
    old_query.patch = Some("old".into());
    assert!(
        tools
            .execute(&old_query, &context, None, "old-query", &hero)
            .unwrap()
            .result
            .is_error
    );
    let definitions = tools.definitions(&query, &context, None).unwrap();
    assert_eq!(definitions.len(), 3);
    tools.required_calls(&query, &context, None).unwrap();
    let old = ToolCall {
        id: "old".into(),
        name: ToolName::ServerKnowledge,
        arguments: json!({"question":"fixture"}),
    }
    .validate(&definitions)
    .unwrap();
    let accounted = tools
        .execute_accounted(&query, &context, None, "old", &old)
        .unwrap();
    assert_eq!(accounted.accounting.observed.cost_micros, 7);
    assert_eq!(accounted.accounting.reserved.input_tokens, 8);
    assert!(accounted.accounting.unaccounted);
    assert!(delegated.lock().unwrap().contains(&"accounted".into()));
    let pin = PinnedGameContext {
        client_version: 1,
        language: brain_contracts::tools::ToolLanguage::German,
        mechanic_revision: "fixture".into(),
    };
    let build = BuildPlanRequest {
        hero_id: 1,
        playstyle: brain_contracts::tools::ToolPlaystyle::Weapon,
        budget: None,
        imbues: vec![],
    };
    assert!(tools
        .validate_build_plan(
            &query,
            &context,
            &pin,
            &build,
            &accounted.value,
            brain_contracts::store::AnswerPurpose::ExternalPublication
        )
        .is_err());
    assert!(delegated.lock().unwrap().contains(&"build".into()));
    executor.block_on(async {
        let mut connection = pool.acquire().await.unwrap();
        assert_session(&mut connection).await.unwrap();
        assert!(sqlx::query("SELECT body FROM raw.object LIMIT 1")
            .execute(&mut *connection)
            .await
            .is_err());
        assert!(sqlx::query("SELECT id FROM ops.answer_trace LIMIT 1")
            .execute(&mut *connection)
            .await
            .is_err());
        sqlx::query("RESET ROLE")
            .execute(&mut *connection)
            .await
            .unwrap();
        assert!(assert_session(&mut connection).await.is_err());
        drop(connection);
        let mut clean = pool.acquire().await.unwrap();
        assert_session(&mut clean).await.unwrap();
    });
    let writer = {
        let _entered = executor.enter();
        PgTraceWriter::new(
            options("brain_service"),
            executor.handle().clone(),
            Duration::from_secs(2),
        )
    };
    let trace = |tools: usize, citations: usize| TraceRecord {
        question_sha256: [1; 32],
        tool_calls: vec!["get_hero".into(); tools],
        citation_ids: (1..=citations).map(|id| id.to_string()).collect(),
        status: "answered",
        tokens: json!({"input_tokens":3}),
        latency_ms: 1,
    };
    writer.write(trace(2, 3)).unwrap();
    assert!(writer.write(trace(9, 3)).is_err());
    assert!(writer.write(trace(2, 101)).is_err());
    let mut invalid_counts = trace(2, 3);
    invalid_counts.tokens = json!({"input_tokens":u64::MAX});
    assert!(matches!(
        writer.write(invalid_counts),
        Err(PortError::InvalidResponse(_))
    ));
    executor.block_on(async {
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM ops.answer_trace").fetch_one(&admin).await.unwrap();assert_eq!(count,1);
        let mut large=json!({"identity":{"citation":"1","patch":null,"source_revision":"commit-a"},
            "stats":{"facts":[],"truncated":false,"omitted_count":0}});
        large["stats"]["facts"]=json!((1..=120).map(|id| json!({"stat_key":"stats.large","value":"x".repeat(512),"condition":{"rank":3},"citation":id.to_string(),"patch":null,"source_revision":"commit-a"})).collect::<Vec<_>>());
        let mut connection=pool.acquire().await.unwrap();
        let bounded=bounded_execution(&mut connection,"\"".repeat(512).as_str(),&hero,large).await.unwrap();
        assert!(serde_json::to_vec(&bounded.result).unwrap().len()<=MAX_BYTES);
        assert!(bounded.dependencies[0].evidence.len()<=MAX_EVIDENCE);
        assert!(bounded.result.result["stats"]["truncated"].as_bool().unwrap());
        let rows=bounded.result.result["stats"]["facts"].as_array().unwrap();
        assert_eq!(bounded.result.result["stats"]["omitted_count"].as_u64().unwrap() as usize,120-rows.len());
        assert!(rows.iter().all(|row| row["value"].as_str().unwrap().len()==512 && row["condition"]==json!({"rank":3})));
        drop(connection);
    });
    let kernel = Kernel::new(NoSnapshot, FixtureProvider).with_tools(
        TracedTools(tools),
        NoSnapshot,
        "fixture",
    );
    let kernel = TracedKernel::new(
        CachedKernel::new(kernel, 1, Duration::from_secs(30)),
        Some(Arc::new(writer)),
    );
    let mut kernel_context = context.clone().into_owned();
    kernel_context.budget.max_input_tokens = 64_000;
    let answered = kernel.answer_for_publication_accounted(&query, &kernel_context);
    assert_eq!(answered.value.status, AnswerStatus::Answered);
    assert_eq!(answered.value.citations[0].evidence_id, "5");
    assert_eq!(answered.accounting.observed.network_rounds, 3);
    assert_eq!(answered.accounting.observed.input_tokens, 30);
    assert_eq!(answered.accounting.observed.output_tokens, 9);
    assert!(answered.accounting.reserved.input_tokens > 0);
    assert!(!answered.accounting.unaccounted);
    executor.block_on(async {
        let (calls, citations): (Value, Value) = sqlx::query_as(
            "SELECT tool_calls,citation_ids FROM ops.answer_trace ORDER BY id DESC LIMIT 1",
        )
        .fetch_one(&admin)
        .await
        .unwrap();
        assert_eq!(calls, json!(["find_entity", "get_hero"]));
        assert_eq!(citations, json!(["1", "8", "4", "5"]));
    });
    let mut cached_query = query.clone();
    cached_query.request_id = "cached".into();
    let cached = kernel.answer_for_publication_accounted(&cached_query, &kernel_context);
    assert_eq!(cached.value.status, AnswerStatus::Answered);
    assert_eq!(cached.accounting, UsageAccounting::default());
    executor.block_on(async {
        let (calls, citations): (Value, Value) = sqlx::query_as(
            "SELECT tool_calls,citation_ids FROM ops.answer_trace ORDER BY id DESC LIMIT 1",
        ).fetch_one(&admin).await.unwrap();
        assert_eq!(calls, json!([]));
        assert_eq!(citations, json!(["5"]));
        sqlx::raw_sql("BEGIN;
            INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','fixture','v1');
            INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES (1,2,'commit-a','commit-b','fixture','v1');
            INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES (1,'hero.kv','commit-b',decode(repeat('03',32),'hex'),'text/plain',convert_to('synthetic-b','UTF8'));
            INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES (2,'hero.kv',2,'upsert');
            INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,2,'commit-b','fixture','v1');
            INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) SELECT 2,1,2,'kv_path','new-'||n,decode(repeat('04',32),'hex'),'fixture','v1' FROM generate_series(1,2) n;
            UPDATE canon.fact SET valid_to_revision_id=2 WHERE entity_id=1 AND stat_key='stats.max_health';
            INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,condition,receipt_id,valid_from_revision_id) VALUES (1,'stats.max_health',1,'hero.kv',120,'{\"rank\":3,\"citation\":\"source-condition\"}',121,2);
            INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'unpublishedonly','en',122,1,'hero.kv',2);
            UPDATE raw.run SET status='partial' WHERE id=2; COMMIT;")
            .execute(&admin).await.unwrap();
    });
    let updated_tools = PgTools::new(Delegate::default(), pool.clone(), executor.handle().clone());
    let hidden = request(
        ToolName::FindEntity,
        json!({"text":"unpublishedonly","kind":"hero"}),
    );
    assert!(
        updated_tools
            .execute(&query, &context, None, "hidden", &hidden)
            .unwrap()
            .result
            .is_error
    );
    updated_tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &profile.dependencies,
            ToolValidationPurpose::Publication,
        )
        .unwrap();
    executor.block_on(async {
        sqlx::raw_sql(
            "BEGIN; UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=2;
            UPDATE raw.run SET status='succeeded' WHERE id=2;
            UPDATE canon.source_head SET published_revision_id=2 WHERE source_id=1;
            UPDATE raw.source SET last_complete_revision='commit-b' WHERE id=1; COMMIT",
        )
        .execute(&admin)
        .await
        .unwrap();
    });
    assert_eq!(
        updated_tools
            .execute(&query, &context, None, "published", &hidden)
            .unwrap()
            .result
            .result["match"],
        "exact"
    );
    assert!(updated_tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &profile.dependencies,
            ToolValidationPurpose::Publication
        )
        .is_err());
    executor.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
