use brain_contracts::{
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
mod scratch_pg;

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
            "object.json",
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
        set.facts.push(Fact { entity_key: "patch_287".into(), stat_key: "patch_change_0".into(), value: FactValue::Text(json!({"reference":"baba","subject":"Baba","statement_index":0,"context_index":null,"stat_label":"health","old":"600","new":"650","line_text":value["raw_content"]}).to_string()), receipt: receipt(&raw, Some("/raw_content")) });
    }
    store
        .apply(run, &raw, if hero { "hero" } else { "patchnotes" }, &set)
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
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let mut transaction = admin.begin().await.unwrap();
    sqlx::query("SET LOCAL ROLE brain_migrate")
        .execute(&mut *transaction)
        .await
        .unwrap();
    let projection =
        include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p4-patches-v2.sql")
            .trim()
            .strip_prefix("BEGIN;")
            .unwrap()
            .trim()
            .strip_suffix("COMMIT;")
            .unwrap();
    sqlx::raw_sql(projection)
        .execute(&mut *transaction)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
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
        assert_eq!(content["content_sha256"], hex::encode(raw.content_sha256));
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
    sqlx::query("UPDATE ops.ingest_run SET status='failed' WHERE id=(SELECT max(id) FROM ops.ingest_run WHERE source='deadlock_patchnotes_db')").execute(&admin).await.unwrap();
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
