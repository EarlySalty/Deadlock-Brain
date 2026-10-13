use super::*;
use brain_contracts::{
    AnswerProviderPort, AnswerStatus, Budget, Principal, ProviderAnswer, ProviderTurn,
    RetrievalPort, ToolConversation, ToolMessage,
};
use brain_kernel::AnswerKernelPort;
use brain_storage::gamefacts::{
    Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt,
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, sync::Arc};

use crate::deadlock_data::scratch_pg;

struct Delegate;

impl ToolExecutionPort for Delegate {
    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(Vec::new())
    }

    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        _: &str,
        _: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        panic!("canonical tools must never execute the legacy delegate")
    }

    fn validate_dependencies(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        assert!(dependencies.is_empty());
        Ok(())
    }
}

fn query() -> Query {
    serde_json::from_value(json!({"request_id":"pg-hero", "conversation_id":"pg-hero", "text":"Baba Steckbrief", "profile":"explain"})).unwrap()
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "pg-hero".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 30_000,
        request_deadline: None,
        discord: None,
        budget: Budget {
            max_network_rounds: 8,
            max_input_tokens: 32_000,
            max_output_tokens: 4_000,
            ..Budget::default()
        },
    }
    .with_request_deadline()
    .into_owned()
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

async fn publish(admin: &PgPool, name: &str, key: &str, revision: &str, large: bool) -> i64 {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("fixture").await.unwrap();
    let values = if large {
        vec![Value::String("x".repeat(2_000)); 40]
    } else {
        vec![json!(780)]
    };
    let bytes = serde_json::to_vec(&json!({"name":name,"values":values})).unwrap();
    let raw = store
        .capture(
            run,
            "fixture",
            revision,
            &format!("{key}.json"),
            bytes.clone(),
        )
        .await
        .unwrap();
    let receipt = |kind: &str, locator: &str, token: &[u8]| Receipt {
        locator_kind: kind.into(),
        locator: locator.into(),
        value_sha256: Sha256::digest(token).into(),
    };
    let name_receipt = receipt("json_pointer", "/name", &serde_json::to_vec(name).unwrap());
    let mut facts = vec![Fact {
        entity_key: key.into(),
        stat_key: "name_en".into(),
        value: FactValue::Text(name.into()),
        receipt: name_receipt.clone(),
    }];
    for (index, value) in values.iter().enumerate() {
        facts.push(Fact {
            entity_key: key.into(),
            stat_key: if large {
                format!("stat_{index:02}")
            } else {
                "health".into()
            },
            value: if large {
                FactValue::Text(value.as_str().unwrap().into())
            } else {
                FactValue::Number(780.0)
            },
            receipt: receipt(
                "json_pointer",
                &format!("/values/{index}"),
                &serde_json::to_vec(value).unwrap(),
            ),
        });
    }
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: receipt("raw_bytes", "", &bytes),
        }],
        aliases: vec![Alias {
            entity_key: key.into(),
            alias: name.into(),
            lang: "en".into(),
            receipt: name_receipt,
        }],
        facts,
        ..FactSet::default()
    };
    store.apply(run, &raw, "fixture-v1", &set).await.unwrap();
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    sqlx::query_scalar("SELECT entity_id FROM serve.entity_identity WHERE key=$1")
        .bind(key)
        .fetch_one(admin)
        .await
        .unwrap()
}

struct NoRetrieval;
impl RetrievalPort for NoRetrieval {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        panic!("hero explanation must not force legacy pre-retrieval")
    }
}

struct NoPin;
impl brain_contracts::tools::GameContextResolver for NoPin {
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

struct ThirdAdapter(CanonicalReceipt);

impl ToolExecutionPort for ThirdAdapter {
    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(vec![ToolDefinition {
            name: ToolName::ServerKnowledge,
            description: "Independent shared-receipt adapter fixture".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["question"],"properties":{"question":{"type":"string","minLength":1}}}),
        }])
    }

    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        assert_eq!(request.name(), ToolName::ServerKnowledge);
        let evidence = format_receipt(&self.0)?;
        Ok(ToolExecution {
            result: ToolResult {
                call_id: id.into(),
                name: request.name(),
                result: json!({"citation": evidence.evidence_id}),
                evidence_ids: vec![evidence.evidence_id.clone()],
                is_error: false,
            },
            dependencies: vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: None,
                evidence: vec![evidence],
            }],
            usage: Usage::default(),
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
        for dependency in dependencies {
            assert_eq!(dependency.request.name(), ToolName::ServerKnowledge);
            assert_eq!(dependency.evidence, vec![format_receipt(&self.0)?]);
        }
        Ok(())
    }
}

struct ScriptedProvider;
impl AnswerProviderPort for ScriptedProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("tool turn required")
    }
    fn answer_turn(
        &self,
        query: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> Result<ProviderTurn, PortError> {
        assert_eq!(tools.len(), 3);
        let call = match conversation.messages.last() {
            None => ToolCall {
                id: "find".into(),
                name: ToolName::FindEntity,
                arguments: json!({"text":query.text.split_whitespace().next().unwrap(),"kind":"hero"}),
            },
            Some(ToolMessage::ToolResults { results })
                if results[0].name == ToolName::FindEntity =>
            {
                assert_eq!(results[0].result["resolved"], true);
                ToolCall {
                    id: "hero".into(),
                    name: ToolName::GetHero,
                    arguments: json!({"hero":results[0].result["entities"][0]["entity_id"]}),
                }
            }
            Some(ToolMessage::ToolResults { results }) if results[0].name == ToolName::GetHero => {
                assert!(!results[0].is_error);
                assert!(!results[0].result["stats"]["facts"]
                    .as_array()
                    .unwrap()
                    .is_empty());
                ToolCall {
                    id: "third".into(),
                    name: ToolName::ServerKnowledge,
                    arguments: json!({"question":query.text}),
                }
            }
            Some(ToolMessage::ToolResults { results }) => {
                assert_eq!(results[0].name, ToolName::ServerKnowledge);
                let cited_evidence_ids: BTreeSet<_> = conversation
                    .messages
                    .iter()
                    .flat_map(|message| match message {
                        ToolMessage::ToolResults { results } => results
                            .iter()
                            .flat_map(|result| result.evidence_ids.clone())
                            .collect::<Vec<_>>(),
                        _ => Vec::new(),
                    })
                    .collect();
                return Ok(ProviderTurn::from(ProviderAnswer {
                    text: "Steckbrief aus veröffentlichten Spielwerten".into(),
                    cited_evidence_ids: cited_evidence_ids.into_iter().collect(),
                    usage: Usage {
                        network_rounds: 1,
                        ..Usage::default()
                    },
                }));
            }
            _ => panic!("unexpected conversation"),
        };
        Ok(ProviderTurn::ToolCalls {
            blocks: vec![brain_contracts::ModelBlock::ToolUse { call }],
            finish_reason: brain_contracts::ProviderFinishReason::ToolUse,
            usage: Usage {
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}

#[test]
fn trimming_keeps_identity_name_receipts_and_reports_omissions() {
    let identity = json!({"name_en":"Baba","facts":[{"stat_key":"name_en","citation":"1"}]});
    let mut profile = json!({"identity":identity,"stats":{"facts":[{"citation":"2"}],"omitted_count":8,"truncated":true}});
    assert!(trim_row(&mut profile));
    assert_eq!(profile["identity"], identity);
    assert_eq!(profile["stats"]["omitted_count"], 9);
    assert_eq!(profile["stats"]["truncated"], true);
    assert!(!trim_row(&mut profile));
}

#[test]
fn cited_rows_and_truncation_preserve_identity_and_strict_output_bounds() {
    let fact = json!({"stat_key":"health","value":780,"citation":"42","source_revision":"r1","patch":null,"condition":{}});
    let mut ids = BTreeMap::new();
    citations(&json!({"stats":{"facts":[fact.clone()]}}), &mut ids).unwrap();
    assert_eq!(ids, BTreeMap::from([(42, "r1".into())]));
    for (field, invalid_value) in [
        ("citation", json!("0")),
        ("citation", json!(42)),
        ("source_revision", json!("")),
        ("patch", json!("historical")),
        ("condition", Value::Null),
    ] {
        let mut invalid_fact = fact.clone();
        invalid_fact[field] = invalid_value;
        assert!(citations(&invalid_fact, &mut BTreeMap::new()).is_err());
    }
    let mut conflicting = fact.clone();
    conflicting["source_revision"] = json!("r2");
    assert!(citations(&json!([fact.clone(), conflicting]), &mut BTreeMap::new()).is_err());
    let identity = json!({"name_en":"Baba","facts":[fact.clone()]});
    let mut value =
        json!({"identity":identity,"stats":{"facts":[fact],"truncated":true,"omitted_count":8}});
    assert!(trim_row(&mut value));
    assert_eq!(value["identity"], identity);
    assert_eq!(value["stats"]["omitted_count"], 9);
    assert!(!trim_row(&mut value));
    let mut result = ToolResult {
        call_id: "bound".into(),
        name: ToolName::GetHero,
        result: json!(""),
        evidence_ids: Vec::new(),
        is_error: false,
    };
    let mut worst = result.clone();
    worst.call_id = "\"".repeat(512);
    let overhead = serde_json::to_vec(&worst).unwrap().len();
    result.result = json!("x".repeat(MAX_BYTES - overhead - 1));
    assert!(fits(&result, MAX_EVIDENCE).unwrap());
    result.result = json!("x".repeat(MAX_BYTES - overhead));
    assert!(!fits(&result, MAX_EVIDENCE).unwrap());
    result.result = json!({});
    assert!(!fits(&result, MAX_EVIDENCE + 1).unwrap());
}

#[test]
fn hash_citation_expansion_is_measured_before_result_binding() {
    let receipt = CanonicalReceipt {
        raw_receipt_id: 42,
        raw_object_id: 7,
        ingest_run_id: 3,
        source: "fixture".into(),
        source_revision: "r1".into(),
        path: "hero.json".into(),
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: "ab".repeat(32),
        extractor: "fixture-v1".into(),
        patch: None,
    };
    let evidence = format_receipt(&receipt).unwrap();
    let mut result = ToolResult {
        call_id: "boundary".into(),
        name: ToolName::GetHero,
        result: json!({"identity":{"citation":"42","source_revision":"r1","patch":null},"padding":""}),
        evidence_ids: vec!["42".into()],
        is_error: false,
    };
    let mut worst = result.clone();
    worst.call_id = "\"".repeat(512);
    let overhead = serde_json::to_vec(&worst).unwrap().len();
    result.result["padding"] = json!("x".repeat(MAX_BYTES - overhead - 1));
    assert!(fits(&result, 1).unwrap());
    replace_citations(
        &mut result.result,
        &BTreeMap::from([(42, evidence.clone())]),
    )
    .unwrap();
    result.evidence_ids = vec![evidence.evidence_id];
    assert!(!fits(&result, 1).unwrap());
}

#[test]
fn pg_executor_proves_receipts_bounds_authorization_revalidation_and_runtime_wiring() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("");
    let (admin, pool, baba, abrams, large) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2).connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        let baba = publish(&admin,"Baba","hero_baba","r1",false).await;
        let abrams = publish(&admin,"Abrams","hero_abrams","r1",false).await;
        let large = publish(&admin,"Large","hero_large","r1",true).await;
        let pool = tool_pool(options.clone().username("brain_readonly"),Duration::from_secs(2));
        let mut connection = pool.acquire().await.unwrap();
        assert_session(&mut connection).await.unwrap();
        for sql in ["SELECT * FROM canon.fact","SELECT * FROM raw.object","SELECT * FROM ops.ingest_run","DELETE FROM serve.current_fact"] {
            assert!(sqlx::query(sql).execute(&mut *connection).await.is_err(),"{sql}");
        }
        sqlx::query("SET SESSION statement_timeout='501ms'").execute(&mut *connection).await.unwrap();
        drop(connection);
        let mut clean = pool.acquire().await.unwrap();
        assert_session(&mut clean).await.unwrap();
        drop(clean);
        (admin,pool,baba,abrams,large)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let query = query();
    let context = context();
    assert!(tools
        .required_calls(&query, &context, None)
        .unwrap()
        .is_empty());
    let find = tools
        .execute(
            &query,
            &context,
            None,
            "find",
            &request(
                ToolName::FindEntity,
                json!({"text":"  BABA  ","kind":"hero"}),
            ),
        )
        .unwrap();
    assert_eq!(find.result.result["resolved"], true);
    assert_eq!(find.result.result["entities"][0]["entity_id"], baba);
    let fuzzy = tools
        .execute(
            &query,
            &context,
            None,
            "fuzzy",
            &request(ToolName::FindEntity, json!({"text":"Babaa"})),
        )
        .unwrap();
    assert_eq!(fuzzy.result.result["match"], "suggestion");
    assert_eq!(fuzzy.result.result["resolved"], false);
    let hero_request = request(ToolName::GetHero, json!({"hero":baba}));
    let hero = tools
        .execute(&query, &context, None, "hero", &hero_request)
        .unwrap();
    assert_eq!(hero.result.result["identity"]["name_en"], "Baba");
    assert_eq!(hero.result.result["abilities"]["available"], false);
    assert_eq!(hero.result.result["stats"]["facts"][0]["value"], 780.0);
    assert!(serde_json::to_vec(&hero.result).unwrap().len() < MAX_BYTES);
    let shared = find.dependencies[0]
        .evidence
        .iter()
        .find(|evidence| hero.dependencies[0].evidence.contains(evidence))
        .unwrap();
    assert!(find.result.evidence_ids.contains(
        &find.result.result["entities"][0]["citation"]
            .as_str()
            .unwrap()
            .to_owned()
    ));
    assert!(shared
        .evidence_id
        .starts_with(brain_contracts::canonical_receipt::RECEIPT_ID_PREFIX));
    let shared_metadata = CanonicalReceipt::from_evidence(shared).unwrap();
    let third_metadata = runtime.block_on(async {
        let row = sqlx::query(RECEIPTS)
            .bind(vec![shared_metadata.raw_receipt_id])
            .fetch_one(&admin)
            .await
            .unwrap();
        receipt_metadata(row).unwrap()
    });
    assert_eq!(format_receipt(&third_metadata).unwrap(), *shared);
    for evidence in &hero.dependencies[0].evidence {
        let metadata: Value = serde_json::from_str(&evidence.content).unwrap();
        assert_eq!(metadata["patch"], Value::Null);
        assert_eq!(metadata["source_revision"], "r1");
        let raw = runtime.block_on(async {
            sqlx::query_scalar::<_, Vec<u8>>("SELECT bytes FROM raw.object WHERE id=$1")
                .bind(metadata["raw_object_id"].as_i64().unwrap())
                .fetch_one(&admin)
                .await
                .unwrap()
        });
        let receipt = Receipt {
            locator_kind: metadata["locator_kind"].as_str().unwrap().into(),
            locator: metadata["locator"].as_str().unwrap().into(),
            value_sha256: hex::decode(metadata["value_sha256"].as_str().unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
        };
        let exact = brain_storage::gamefacts::receipts::value_bytes(&raw, &receipt).unwrap();
        assert_eq!(
            <[u8; 32]>::from(Sha256::digest(exact)),
            receipt.value_sha256
        );
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &hero.dependencies, purpose)
            .unwrap();
    }
    let bounded = tools
        .execute(
            &query,
            &context,
            None,
            "large",
            &request(ToolName::GetHero, json!({"hero":large})),
        )
        .unwrap();
    assert!(serde_json::to_vec(&bounded.result).unwrap().len() < MAX_BYTES);
    assert_eq!(bounded.result.result["stats"]["truncated"], true);
    assert!(
        bounded.result.result["stats"]["omitted_count"]
            .as_u64()
            .unwrap()
            > 8
    );
    let sections = tools
        .execute(
            &query,
            &context,
            None,
            "sections",
            &request(
                ToolName::GetHero,
                json!({"hero":abrams,"sections":["weapon"]}),
            ),
        )
        .unwrap();
    assert!(sections.result.result.get("stats").is_none());
    assert!(sections.result.result.get("abilities").is_none());
    for (name, args) in [
        (ToolName::GetHero, json!({"hero":i64::MAX})),
        (ToolName::FindEntity, json!({"text":"zzzzzznotpresent"})),
        (ToolName::GetHero, json!({"hero":baba,"patch":"2026-10-01"})),
    ] {
        let missing = tools
            .execute(&query, &context, None, "missing", &request(name, args))
            .unwrap();
        assert!(missing.result.is_error);
        assert!(missing.dependencies.is_empty() && missing.result.evidence_ids.is_empty());
    }
    let mut historic = query.clone();
    historic.patch = Some("2026-10-01".into());
    assert!(
        tools
            .execute(&historic, &context, None, "history", &hero_request)
            .unwrap()
            .result
            .is_error
    );
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        tools.execute(&query, &denied, None, "denied", &hero_request),
        Err(PortError::PermissionDenied(_))
    ));
    denied = context.clone();
    denied.conversation_id = "other".into();
    assert!(tools
        .execute(&query, &denied, None, "denied", &hero_request)
        .is_err());
    runtime.block_on(async {
        sqlx::query("UPDATE canon.receipt SET extractor=$1 WHERE id=$2")
            .bind(format!("{}.changed", shared_metadata.extractor))
            .bind(shared_metadata.raw_receipt_id)
            .execute(&admin)
            .await
            .unwrap();
    });
    let changed_receipt = tools
        .execute(&query, &context, None, "changed-receipt", &hero_request)
        .unwrap();
    assert!(!changed_receipt
        .result
        .evidence_ids
        .contains(&shared.evidence_id));
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &hero.dependencies,
            ToolValidationPurpose::Cache
        )
        .is_err());
    runtime.block_on(async {
        sqlx::query("UPDATE canon.receipt SET extractor=$1 WHERE id=$2")
            .bind(&shared_metadata.extractor)
            .bind(shared_metadata.raw_receipt_id)
            .execute(&admin)
            .await
            .unwrap();
        sqlx::query("UPDATE canon.fact SET value_num=781 WHERE entity_id=$1 AND stat_key='health'")
            .bind(baba)
            .execute(&admin)
            .await
            .unwrap();
    });
    let changed_fact = tools
        .execute(&query, &context, None, "changed-fact", &hero_request)
        .unwrap();
    assert_eq!(changed_fact.result.evidence_ids, hero.result.evidence_ids);
    assert_eq!(
        changed_fact.dependencies[0].evidence,
        hero.dependencies[0].evidence
    );
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &hero.dependencies,
            ToolValidationPurpose::Publication
        )
        .is_err());
    runtime.block_on(async {
        sqlx::query("UPDATE canon.fact SET value_num=780 WHERE entity_id=$1 AND stat_key='health'")
            .bind(baba)
            .execute(&admin)
            .await
            .unwrap();
    });
    tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &hero.dependencies,
            ToolValidationPurpose::Cache,
        )
        .unwrap();
    let tools = PgTools::new(
        ThirdAdapter(third_metadata),
        pool.clone(),
        runtime.handle().clone(),
    );
    let traced = super::super::answer_trace::TracedTools(tools);
    let kernel = brain_kernel::Kernel::new(NoRetrieval, ScriptedProvider)
        .with_tools(traced, NoPin, "fixture");
    let kernel = brain_kernel::CachedKernel::new(kernel, 8, Duration::from_secs(30));
    assert_eq!(
        kernel.answer_for_publication(&query, &context).status,
        AnswerStatus::BudgetExceeded
    );
    let mut adapter_context = context.clone();
    adapter_context.budget.max_input_tokens = 128_000;
    let writer = runtime.block_on(async {
        super::super::answer_trace::PgTraceWriter::new(
            options.username("brain_service"),
            runtime.handle().clone(),
            Duration::from_secs(2),
        )
    });
    let kernel = super::super::answer_trace::TracedKernel::new(kernel, Some(Arc::new(writer)));
    for name in ["Baba", "Abrams"] {
        let mut question = query.clone();
        question.text = format!("{name} Steckbrief");
        let answer = kernel.answer_for_publication(&question, &adapter_context);
        assert_eq!(answer.status, AnswerStatus::Answered, "{answer:?}");
        assert!(!answer.citations.is_empty());
        question.request_id = format!("warm-{name}");
        let warm = kernel.answer_for_publication(&question, &adapter_context);
        assert_eq!(warm.status, AnswerStatus::Answered, "{warm:?}");
        assert_eq!(warm.citations, answer.citations);
    }
    runtime.block_on(async {
        let rows: Vec<(Value, Value)> =
            sqlx::query_as("SELECT tool_calls,citation_ids FROM ops.answer_trace ORDER BY id")
                .fetch_all(&admin)
                .await
                .unwrap();
        assert_eq!(rows.len(), 4);
        for (index, (calls, citations)) in rows.iter().enumerate() {
            assert_eq!(
                *calls,
                if index % 2 == 0 {
                    json!(["find_entity", "get_hero", "server_knowledge"])
                } else {
                    json!([])
                }
            );
            let ids = citations.as_array().unwrap();
            assert!(!ids.is_empty());
            assert!(ids.iter().all(|id| id
                .as_str()
                .unwrap()
                .parse::<i64>()
                .is_ok_and(|id| id > 0)));
            assert!(ids.contains(&json!(shared_metadata.raw_receipt_id.to_string())));
        }
        let trace_ids = |value: &Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|id| id.as_str().unwrap().to_owned())
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(trace_ids(&rows[0].1), trace_ids(&rows[1].1));
        assert_eq!(trace_ids(&rows[2].1), trace_ids(&rows[3].1));
        publish(&admin, "Baba", "hero_baba", "r2", false).await;
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    runtime.block_on(publish(&admin, "Baba", "hero_baba_twin", "r3", false));
    let ambiguous = tools
        .execute(
            &query,
            &context,
            None,
            "ambiguous",
            &request(ToolName::FindEntity, json!({"text":"Baba","limit":1})),
        )
        .unwrap();
    assert_eq!(ambiguous.result.result["ambiguous"], true);
    assert_eq!(ambiguous.result.result["resolved"], false);
    assert_eq!(ambiguous.result.result["truncated"], true);
    assert_eq!(
        ambiguous.result.result["entities"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &hero.dependencies,
            ToolValidationPurpose::Cache
        )
        .is_err());
    runtime.block_on(async {
        sqlx::query("UPDATE ops.ingest_run SET status='failed' WHERE source='fixture'")
            .execute(&admin)
            .await
            .unwrap();
    });
    assert!(
        tools
            .execute(&query, &context, None, "unpublished", &hero_request)
            .unwrap()
            .result
            .is_error
    );
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
