use brain_contracts::{
    canonical_receipt::CanonicalReceipt,
    tools::{canonical_patch_history_definition, BuildPlanRequest},
    Accounted, AuthorizedContext, PinnedGameContext, PortError, PortFailure, Principal, Query,
    ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolName,
    ToolRequest, ToolResult, ToolValidationPurpose, Usage, UsageAccounting,
};
use brain_serve::pg_patch_port::PgPatchToolPort;
use brain_storage::{
    gamefacts::{
        receipts, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject,
        Receipt,
    },
    PgStore,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
pub(crate) mod scratch_pg;

#[path = "../src/pg_tools.rs"]
mod pg_tools;

#[path = "../src/answer_trace.rs"]
mod answer_trace;

mod deadlock_data {
    pub(crate) use crate::scratch_pg;
}

type Validation = (
    Option<PinnedGameContext>,
    Vec<ToolEvidenceDependency>,
    ToolValidationPurpose,
);

#[derive(Clone, Default)]
struct Delegate {
    validations: Arc<Mutex<Vec<Validation>>>,
    calls: Arc<Mutex<Vec<String>>>,
}

fn knowledge_definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::ServerKnowledge,
        description: "Wissen".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["question"],
            "properties":{"question":{"type":"string"}}}),
    }
}

fn legacy_definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::PatchHistory,
        description: "Legacy".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["entity"],
            "properties":{"entity":{"type":"object","additionalProperties":false,"required":["kind","id"],
                "properties":{"kind":{"type":"string","enum":["hero"]},"id":{"type":"integer","minimum":1}}}}}),
    }
}

impl ToolExecutionPort for Delegate {
    fn required_calls(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        Ok(vec![ToolCall {
            id: "required".into(),
            name: ToolName::ServerKnowledge,
            arguments: json!({"question":"Wissen"}),
        }])
    }

    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(vec![knowledge_definition(), legacy_definition()])
    }

    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        self.calls.lock().unwrap().push(format!("execute:{id}"));
        Ok(ToolExecution {
            result: ToolResult {
                call_id: id.into(),
                name: request.name(),
                result: json!({"delegate":true}),
                evidence_ids: Vec::new(),
                is_error: true,
            },
            dependencies: Vec::new(),
            usage: Usage::default(),
        })
    }

    fn execute_accounted(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        id: &str,
        _: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        self.calls.lock().unwrap().push(format!("accounted:{id}"));
        Err(PortFailure::accounted(
            PortError::BudgetExceeded,
            UsageAccounting::default(),
        ))
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
        self.calls.lock().unwrap().push("build".into());
        Err(PortError::BudgetExceeded)
    }

    fn validate_dependencies(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        assert!(dependencies.iter().all(|dependency| !matches!(
            dependency.request.subrequest(),
            brain_contracts::tools::ToolSubrequest::CanonicalPatchHistory(_)
        )));
        self.validations
            .lock()
            .unwrap()
            .push((pin.cloned(), dependencies.to_vec(), purpose));
        if dependencies.is_empty() {
            Err(PortError::BudgetExceeded)
        } else {
            Ok(())
        }
    }
}

fn query_context() -> (Query, AuthorizedContext) {
    (
        Query {
            request_id: "port-test".into(),
            conversation_id: "port-test".into(),
            text: "Patchänderungen".into(),
            answer_context: None,
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: Default::default(),
            patch: None,
            mode: None,
        },
        AuthorizedContext {
            principal: Principal {
                actor_id: "test".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            discord: None,
            conversation_id: "port-test".into(),
            knowledge_release: "test".into(),
            deadline_ms: 1000,
            budget: Default::default(),
            request_deadline: None,
        },
    )
}

fn request(name: ToolName, arguments: Value, definition: ToolDefinition) -> ToolRequest {
    ToolCall {
        id: "patch-call".into(),
        name,
        arguments,
    }
    .validate(&[definition])
    .unwrap()
}

fn receipt(raw: &RawObject, pointer: Option<&str>) -> Receipt {
    let mut value = Receipt {
        locator_kind: if pointer.is_some() {
            "json_pointer"
        } else {
            "raw_bytes"
        }
        .into(),
        locator: pointer.unwrap_or_default().into(),
        value_sha256: [0; 32],
    };
    value.value_sha256 = Sha256::digest(receipts::value_bytes(&raw.bytes, &value).unwrap()).into();
    value
}

async fn publish(store: &GameFactsStore, source: &str, value: Value, hero: bool) -> RawObject {
    let run = store.begin_run(source).await.unwrap();
    let raw = store
        .capture(
            run,
            source,
            "revision-1",
            if hero { "object.json" } else { "287" },
            serde_json::to_vec(&value).unwrap(),
        )
        .await
        .unwrap();
    let key = if hero {
        "hero_baba".into()
    } else {
        "patch_287".into()
    };
    let mut set = FactSet {
        entities: vec![Entity {
            kind: if hero { "hero" } else { "patch" }.into(),
            key,
            receipt: receipt(&raw, None),
        }],
        ..FactSet::default()
    };
    if hero {
        set.aliases.push(Alias {
            entity_key: "hero_baba".into(),
            alias: "baba".into(),
            lang: "en".into(),
            receipt: receipt(&raw, Some("/alias")),
        });
        set.facts.push(Fact {
            entity_key: "hero_baba".into(),
            stat_key: "health".into(),
            value: FactValue::Number(650.0),
            receipt: receipt(&raw, Some("/health")),
        });
    } else {
        for (stat, field) in [
            ("label", "title"),
            ("released_at", "posted_at"),
            ("official_url", "url"),
        ] {
            set.facts.push(Fact {
                entity_key: "patch_287".into(),
                stat_key: stat.into(),
                value: FactValue::Text(value[field].as_str().unwrap().into()),
                receipt: receipt(&raw, Some(&format!("/{field}"))),
            });
        }
        set.facts.push(Fact { entity_key: "patch_287".into(), stat_key: format!("patch_change_r{:x}_0", Sha256::digest(b"baba")), value: FactValue::Text(json!({"reference":"baba","subject":"Baba","statement_index":0,"context_index":null,"stat_label":"health","old":"600","new":"650","line_text":value["raw_content"]}).to_string()), receipt: receipt(&raw, Some("/raw_content")) });
    }
    store
        .apply(
            run,
            &raw,
            if hero { "hero" } else { "v1:10:patchnotes:1:2" },
            &set,
        )
        .await
        .unwrap();
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                rejected: 0,
                ignored: 0,
            },
        )
        .await
        .unwrap();
    raw
}

async fn database(pg: &scratch_pg::ScratchPg) -> (PgPool, PgPool, PgPool) {
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    let schema = PgStore::new(admin.clone());
    schema.migrate_gamefacts().await.unwrap();
    schema.check_gamefacts_schema().await.unwrap();
    schema.migrate_gamefacts().await.unwrap();
    schema.check_gamefacts_schema().await.unwrap();
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(versions, [1, 4, 7]);
    let reader = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_tool")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options.clone())
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    (admin, reader, writer)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn adapter_executes_real_pg_and_revalidates_without_legacy_pin_or_retrieval() {
    let pg = scratch_pg::ScratchPg::start();
    let (admin, reader, writer) = database(&pg).await;
    let store = GameFactsStore::new(writer.clone());
    let hero = publish(
        &store,
        "gamefiles",
        json!({"alias":"baba","health":650}),
        true,
    )
    .await;
    let patch = publish(&store, "deadlock_patchnotes_db", json!({"title":"Patch 287","posted_at":"2026-10-10T12:00:00Z","url":"https://store.steampowered.com/news/app/1422450/view/287","raw_content":"Baba health increased from 600 to 650"}), false).await;
    let entity: i64 =
        sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key='hero_baba'")
            .fetch_one(&reader)
            .await
            .unwrap();
    let delegate = Delegate::default();
    let port = Arc::new(
        PgPatchToolPort::new(
            delegate.clone(),
            reader.clone(),
            tokio::runtime::Handle::current(),
        )
        .unwrap(),
    );
    let (query, context) = query_context();
    let definitions = port.definitions(&query, &context, None).unwrap();
    brain_contracts::tools::validate_definitions(&definitions).unwrap();
    assert_eq!(
        definitions,
        [knowledge_definition(), canonical_patch_history_definition()]
    );
    let call = ToolCall {
        id: "patch-call".into(),
        name: ToolName::PatchHistory,
        arguments: json!({"entity":entity}),
    };
    let request = call.validate(&definitions).unwrap();
    let execution = {
        let port = port.clone();
        let query = query.clone();
        let context = context.clone();
        let request = request.clone();
        tokio::task::spawn_blocking(move || {
            port.execute(&query, &context, None, "patch-call", &request)
        })
        .await
        .unwrap()
        .unwrap()
    };
    execution.validate_for(&call, &request, None).unwrap();
    assert!(!execution.result.is_error, "{:?}", execution.result);
    assert_eq!(execution.result.result["changes"][0]["old"], "600");
    assert_eq!(execution.result.result["changes"][0]["new"], "650");
    assert!(execution.dependencies[0].game_context.is_none());
    assert!(serde_json::to_vec(&execution).unwrap().len() < 16_384);
    for evidence in &execution.dependencies[0].evidence {
        let content: Value = serde_json::from_str(&evidence.content).unwrap();
        let raw = [&hero, &patch]
            .into_iter()
            .find(|raw| content["raw_object_id"] == raw.id)
            .unwrap();
        let locator = Receipt {
            locator_kind: content["locator_kind"].as_str().unwrap().into(),
            locator: content["locator"].as_str().unwrap().into(),
            value_sha256: [0; 32],
        };
        assert_eq!(
            content["value_sha256"],
            hex::encode(Sha256::digest(
                receipts::value_bytes(&raw.bytes, &locator).unwrap()
            ))
        );
        assert!(content.get("content_sha256").is_none());
        let content_hash: String = sqlx::query_scalar(
            "SELECT receipt->>'content_sha256' FROM serve.p4_patch_receipt WHERE receipt_id=$1",
        )
        .bind(
            CanonicalReceipt::from_evidence(evidence)
                .unwrap()
                .raw_receipt_id,
        )
        .fetch_one(&reader)
        .await
        .unwrap();
        assert_eq!(content_hash, hex::encode(raw.content_sha256));
        assert_eq!(evidence.source_id, format!("canon.source/{}", raw.source));
        assert!(evidence.patch.is_none());
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        port.validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .unwrap();
    }
    assert!(delegate.validations.lock().unwrap().is_empty());
    assert!(delegate.calls.lock().unwrap().is_empty());
    let accounted = port
        .execute_accounted(&query, &context, None, "accounted", &request)
        .unwrap();
    assert_eq!(
        accounted.accounting,
        UsageAccounting::observed(accounted.value.usage.clone())
    );
    let knowledge = ToolEvidenceDependency {
        request: request_for_knowledge(),
        game_context: None,
        evidence: Vec::new(),
    };
    let mut mixed = execution.dependencies.clone();
    mixed.push(knowledge.clone());
    port.validate_dependencies(
        &query,
        &context,
        None,
        &mixed,
        ToolValidationPurpose::Publication,
    )
    .unwrap();
    assert_eq!(
        *delegate.validations.lock().unwrap(),
        [(None, vec![knowledge], ToolValidationPurpose::Publication)]
    );
    let mut tampered = execution.dependencies.clone();
    tampered[0].evidence[0].content.push(' ');
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            None,
            &tampered,
            ToolValidationPurpose::Cache
        )
        .is_err());
    let mut pinned = execution.dependencies.clone();
    pinned[0].game_context = Some(pin());
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            None,
            &pinned,
            ToolValidationPurpose::Provider
        )
        .is_err());
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        port.execute(&query, &denied, None, "denied", &request),
        Err(PortError::PermissionDenied(_))
    ));
    let mut foreign = query.clone();
    foreign.conversation_id = "foreign".into();
    assert!(matches!(
        port.execute(&foreign, &context, None, "foreign", &request),
        Err(PortError::PermissionDenied(_))
    ));
    let wrong_role = PgPatchToolPort::new(
        Delegate::default(),
        admin.clone(),
        tokio::runtime::Handle::current(),
    )
    .unwrap();
    assert!(matches!(
        wrong_role.execute(&query, &context, None, "wrong-role", &request),
        Err(PortError::PermissionDenied(_))
    ));
    assert!(sqlx::query("UPDATE ops.ingest_run SET status='failed' WHERE id=(SELECT max(id) FROM ops.ingest_run WHERE source='deadlock_patchnotes_db')").execute(&admin).await.is_err());
    let definition: String =
        sqlx::query_scalar("SELECT pg_get_viewdef('serve.published_receipt'::regclass, true)")
            .fetch_one(&admin)
            .await
            .unwrap();
    let mut quarantine = admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *quarantine)
        .await
        .unwrap();
    sqlx::raw_sql(&format!(
        "CREATE OR REPLACE VIEW serve.published_receipt AS SELECT p.* FROM ({}) p WHERE p.source <> 'deadlock_patchnotes_db'",
        definition.trim_end_matches(';')
    ))
    .execute(&mut *quarantine)
    .await
    .unwrap();
    quarantine.commit().await.unwrap();
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            None,
            &execution.dependencies,
            ToolValidationPurpose::Publication
        )
        .is_err());
    reader.close().await;
    writer.close().await;
    admin.close().await;
}

struct EmptyDelegate;

impl ToolExecutionPort for EmptyDelegate {
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
        panic!("all three canonical tools must execute PostgreSQL")
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

struct NoRetrieval;

impl brain_contracts::RetrievalPort for NoRetrieval {
    fn retrieve(
        &self,
        _: &Query,
        _: &AuthorizedContext,
    ) -> Result<Vec<brain_contracts::Evidence>, PortError> {
        panic!("canonical tool loop must not retrieve legacy evidence")
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

struct CombinedProvider {
    expected: std::collections::BTreeMap<String, brain_contracts::Evidence>,
}

impl brain_contracts::AnswerProviderPort for CombinedProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[brain_contracts::Evidence],
    ) -> Result<brain_contracts::ProviderAnswer, PortError> {
        panic!("combined proof requires the real kernel tool loop")
    }

    fn answer_turn(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[brain_contracts::Evidence],
        definitions: &[ToolDefinition],
        conversation: &brain_contracts::ToolConversation,
    ) -> Result<brain_contracts::ProviderTurn, PortError> {
        use brain_contracts::{
            ModelBlock, ProviderAnswer, ProviderFinishReason, ProviderTurn, ToolMessage,
        };
        assert_eq!(definitions.len(), 3);
        let call = match conversation.messages.last() {
            None => ToolCall {
                id: "find".into(),
                name: ToolName::FindEntity,
                arguments: json!({"text":"Baba","kind":"hero"}),
            },
            Some(ToolMessage::ToolResults { results })
                if results[0].name == ToolName::FindEntity =>
            {
                assert!(!results[0].is_error);
                assert_eq!(results[0].result["resolved"], true);
                ToolCall {
                    id: "hero".into(),
                    name: ToolName::GetHero,
                    arguments: json!({"hero":results[0].result["entities"][0]["entity_id"]}),
                }
            }
            Some(ToolMessage::ToolResults { results }) if results[0].name == ToolName::GetHero => {
                assert!(!results[0].is_error);
                ToolCall {
                    id: "patch".into(),
                    name: ToolName::PatchHistory,
                    arguments: json!({"entity":results[0].result["identity"]["entity_id"]}),
                }
            }
            Some(ToolMessage::ToolResults { results })
                if results[0].name == ToolName::PatchHistory =>
            {
                assert!(!results[0].is_error);
                assert_eq!(results[0].result["changes"][0]["old"], "600");
                assert_eq!(results[0].result["changes"][0]["new"], "650");
                let seen = evidence
                    .iter()
                    .map(|item| (item.evidence_id.clone(), item.clone()))
                    .collect::<std::collections::BTreeMap<_, _>>();
                assert_eq!(seen.len(), evidence.len());
                assert_eq!(seen, self.expected);
                return Ok(ProviderTurn::from(ProviderAnswer {
                    text: "Babas Gesundheit beträgt 650. Die offizielle Patchzeile nennt eine Erhöhung von 600 auf 650.".into(),
                    cited_evidence_ids: self.expected.keys().cloned().collect(),
                    usage: Usage { network_rounds: 1, ..Usage::default() },
                }));
            }
            _ => panic!("unexpected combined tool conversation"),
        };
        Ok(ProviderTurn::ToolCalls {
            blocks: vec![ModelBlock::ToolUse { call }],
            finish_reason: ProviderFinishReason::ToolUse,
            usage: Usage {
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}

fn combined_input_budget(
    query: &Query,
    definitions: &[ToolDefinition],
    executions: &[&ToolExecution],
) -> u32 {
    use brain_contracts::{
        provider_input::{grounded_turn_input_ceiling, ToolWireFormat},
        ModelBlock, ToolConversation, ToolMessage,
    };
    let mut conversation = ToolConversation::default();
    let mut evidence = std::collections::BTreeMap::new();
    let ceiling = |conversation: &ToolConversation, evidence: Vec<brain_contracts::Evidence>| {
        [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible]
            .into_iter()
            .map(|format| {
                grounded_turn_input_ceiling(query, &evidence, definitions, conversation, format)
                    .unwrap()
            })
            .max()
            .unwrap()
    };
    let mut total = ceiling(&conversation, Vec::new());
    for execution in executions {
        conversation.messages.push(ToolMessage::Assistant {
            blocks: vec![ModelBlock::ToolUse {
                call: ToolCall {
                    id: execution.result.call_id.clone(),
                    name: execution.result.name,
                    arguments: execution.dependencies[0].request.arguments().clone(),
                },
            }],
        });
        conversation.messages.push(ToolMessage::ToolResults {
            results: vec![execution.result.clone()],
        });
        for item in &execution.dependencies[0].evidence {
            evidence.insert(item.evidence_id.clone(), item.clone());
        }
        total += ceiling(&conversation, evidence.values().cloned().collect());
    }
    u32::try_from(total).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_owner_and_patch_receipts_match_byte_for_byte_in_the_kernel_loop() {
    use brain_kernel::AnswerKernelPort;
    let pg = scratch_pg::ScratchPg::start();
    let (admin, reader, writer) = database(&pg).await;
    let store = GameFactsStore::new(writer.clone());
    publish(
        &store,
        "gamefiles",
        json!({"alias":"baba","health":650}),
        true,
    )
    .await;
    publish(&store, "deadlock_patchnotes_db", json!({"title":"Patch 287","posted_at":"2026-10-10T12:00:00Z","url":"https://store.steampowered.com/news/app/1422450/view/287","raw_content":"Baba health increased from 600 to 650"}), false).await;
    sqlx::raw_sql("ALTER ROLE brain_readonly LOGIN; ALTER ROLE brain_service LOGIN")
        .execute(&admin)
        .await
        .unwrap();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_readonly")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let pool = pg_tools::tool_pool(options.clone(), std::time::Duration::from_secs(2));
    let tools = pg_tools::PgTools::new(
        PgPatchToolPort::new(
            EmptyDelegate,
            pool.clone(),
            tokio::runtime::Handle::current(),
        )
        .unwrap(),
        pool.clone(),
        tokio::runtime::Handle::current(),
    );
    let (mut query, mut context) = query_context();
    query.text = "Baba Steckbrief und Patchänderungen".into();
    query.profile = brain_contracts::AnswerProfile::Explain;
    context.deadline_ms = 30_000;
    context.budget.max_network_rounds = 8;
    let definitions = tools.definitions(&query, &context, None).unwrap();
    let execute = |name, arguments, id: &str| {
        let call = ToolCall {
            id: id.into(),
            name,
            arguments,
        };
        let request = call.validate(&definitions).unwrap();
        let execution = tools.execute(&query, &context, None, id, &request).unwrap();
        execution.validate_for(&call, &request, None).unwrap();
        assert!(!execution.result.is_error, "{:?}", execution.result);
        execution
    };
    let find = execute(
        ToolName::FindEntity,
        json!({"text":"Baba","kind":"hero"}),
        "find",
    );
    let entity = find.result.result["entities"][0]["entity_id"]
        .as_i64()
        .unwrap();
    let hero = execute(ToolName::GetHero, json!({"hero":entity}), "hero");
    let patch = execute(ToolName::PatchHistory, json!({"entity":entity}), "patch");
    for (field, owner) in [
        ("binding_citation", &find),
        ("entity_citation", &find),
        ("entity_citation", &hero),
        ("stat_citation", &hero),
    ] {
        let id = patch.result.result["changes"][0][field].as_str().unwrap();
        let actual = patch.dependencies[0]
            .evidence
            .iter()
            .find(|item| item.evidence_id == id)
            .unwrap();
        let expected = owner.dependencies[0]
            .evidence
            .iter()
            .find(|item| item.evidence_id == id)
            .unwrap();
        assert_eq!(actual, expected, "{field}");
        assert_eq!(actual.content.as_bytes(), expected.content.as_bytes());
        assert_eq!(
            serde_json::to_vec(actual).unwrap(),
            serde_json::to_vec(expected).unwrap()
        );
    }
    let mut expected = std::collections::BTreeMap::new();
    let mut dependencies = Vec::new();
    for execution in [&find, &hero, &patch] {
        dependencies.extend(execution.dependencies.clone());
        for item in &execution.dependencies[0].evidence {
            if let Some(previous) = expected.insert(item.evidence_id.clone(), item.clone()) {
                assert_eq!(previous, *item);
            }
        }
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &dependencies, purpose)
            .unwrap();
    }
    let count = expected.len();
    let expected_numeric_ids = expected
        .values()
        .map(|evidence| {
            CanonicalReceipt::from_evidence(evidence)
                .unwrap()
                .raw_receipt_id
                .to_string()
        })
        .collect::<BTreeSet<_>>();
    let input_budget = combined_input_budget(&query, &definitions, &[&find, &hero, &patch]);
    context.budget.max_input_tokens = input_budget;
    let kernel = brain_kernel::Kernel::new(NoRetrieval, CombinedProvider { expected }).with_tools(
        answer_trace::TracedTools(tools),
        NoPin,
        "fixture",
    );
    let kernel = brain_kernel::CachedKernel::new(kernel, 8, std::time::Duration::from_secs(30));
    let trace_writer = answer_trace::PgTraceWriter::new(
        options.username("brain_service"),
        tokio::runtime::Handle::current(),
        std::time::Duration::from_secs(2),
    );
    let kernel = answer_trace::TracedKernel::new(kernel, Some(Arc::new(trace_writer)));
    let accounted = kernel.answer_for_publication_accounted(&query, &context);
    let answer = accounted.value;
    assert_eq!(
        accounted.accounting.charged().input_tokens,
        u64::from(input_budget)
    );
    assert_eq!(
        answer.status,
        brain_contracts::AnswerStatus::Answered,
        "{answer:?}"
    );
    assert_eq!(answer.citations.len(), count);
    query.request_id = "warm-port-test".into();
    let warm = kernel.answer_for_publication_accounted(&query, &context);
    assert_eq!(
        warm.value.status,
        brain_contracts::AnswerStatus::Answered,
        "{warm:?}"
    );
    assert_eq!(warm.value.citations, answer.citations);
    let traces: Vec<(Value, Value)> =
        sqlx::query_as("SELECT tool_calls,citation_ids FROM ops.answer_trace ORDER BY id")
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(traces.len(), 2);
    assert_eq!(
        traces[0].0,
        json!(["find_entity", "get_hero", "patch_history"])
    );
    assert_eq!(traces[1].0, json!([]));
    for (_, citations) in traces {
        let numeric_ids = citations
            .as_array()
            .unwrap()
            .iter()
            .map(|id| id.as_str().unwrap().to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(numeric_ids, expected_numeric_ids);
        assert!(numeric_ids
            .iter()
            .all(|id| id.parse::<i64>().is_ok_and(|id| id > 0)));
    }
    println!("P4_OWNER_RECEIPT_PROOF actual pg_tools.rs FindEntity/GetHero + PgPatchToolPort, real scratch PostgreSQL, alias/entity/stat Evidence byte equality, real three-tool kernel publication loop, all validation purposes, exact_input_budget={input_budget}, canonical hash IDs and cold/warm numeric PostgreSQL traces");
    pool.close().await;
    reader.close().await;
    writer.close().await;
    admin.close().await;
}

fn request_for_knowledge() -> ToolRequest {
    request(
        ToolName::ServerKnowledge,
        json!({"question":"Wissen"}),
        knowledge_definition(),
    )
}

fn pin() -> PinnedGameContext {
    PinnedGameContext {
        client_version: 1,
        language: brain_contracts::tools::ToolLanguage::English,
        mechanic_revision: "fixture".into(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn adapter_preserves_delegate_contract_and_only_advertises_to_authorized_contexts() {
    let delegate = Delegate::default();
    let pool = PgPoolOptions::new().connect_lazy_with(PgConnectOptions::new_without_pgpass());
    let port =
        PgPatchToolPort::new(delegate.clone(), pool, tokio::runtime::Handle::current()).unwrap();
    let (query, context) = query_context();
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert_eq!(
        port.definitions(&query, &denied, None).unwrap(),
        delegate.definitions(&query, &denied, None).unwrap()
    );
    let mut foreign = query.clone();
    foreign.conversation_id = "foreign".into();
    assert_eq!(
        port.definitions(&foreign, &context, None).unwrap(),
        delegate.definitions(&foreign, &context, None).unwrap()
    );
    assert_eq!(
        port.required_calls(&query, &context, None).unwrap(),
        delegate.required_calls(&query, &context, None).unwrap()
    );
    let legacy = request(
        ToolName::PatchHistory,
        json!({"entity":{"kind":"hero","id":1}}),
        legacy_definition(),
    );
    for request in [legacy, request_for_knowledge()] {
        let execution = port
            .execute(&query, &context, Some(&pin()), "delegated", &request)
            .unwrap();
        assert_eq!(execution.result.result, json!({"delegate":true}));
        assert_eq!(
            port.validate_build_plan(
                &query,
                &context,
                &pin(),
                &BuildPlanRequest {
                    hero_id: 1,
                    playstyle: brain_contracts::tools::ToolPlaystyle::Weapon,
                    budget: None,
                    imbues: Vec::new(),
                },
                &execution,
                brain_contracts::store::AnswerPurpose::ExternalPublication,
            ),
            Err(PortError::BudgetExceeded)
        );
        let failure = port
            .execute_accounted(&query, &context, Some(&pin()), "delegated", &request)
            .unwrap_err();
        assert_eq!(failure.error, PortError::BudgetExceeded);
        assert_eq!(
            failure.accounting.as_deref(),
            Some(&UsageAccounting::default())
        );
        let dependency = ToolEvidenceDependency {
            request,
            game_context: Some(pin()),
            evidence: Vec::new(),
        };
        port.validate_dependencies(
            &query,
            &context,
            Some(&pin()),
            std::slice::from_ref(&dependency),
            ToolValidationPurpose::Cache,
        )
        .unwrap();
        assert_eq!(
            delegate.validations.lock().unwrap().last(),
            Some(&(Some(pin()), vec![dependency], ToolValidationPurpose::Cache))
        );
    }
    assert_eq!(
        port.validate_dependencies(&query, &context, None, &[], ToolValidationPurpose::Provider),
        Err(PortError::BudgetExceeded)
    );
    assert_eq!(
        *delegate.calls.lock().unwrap(),
        [
            "execute:delegated",
            "build",
            "accounted:delegated",
            "execute:delegated",
            "build",
            "accounted:delegated"
        ]
    );
}
