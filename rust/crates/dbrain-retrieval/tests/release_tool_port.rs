use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    store::AnswerPurpose,
    tools::{
        BuildPlanRequest, ToolCall, ToolExecutionPort, ToolLanguage, ToolName, ToolPlaystyle,
        ToolValidationPurpose,
    },
    value::Observed,
    AuthorizedContext, CorpusRelease, PortError, Query, RequestDeadline, RetrievalPort,
    SourceRecordV2, SourceVisibility,
};
use brain_storage::{LocalPgReader, PgStore};
use dbrain_retrieval::{ReleaseRetriever, ReleaseToolExecutionPort};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[path = "../../brain-storage/tests/mirror_game_context.rs"]
mod mirror_fixture;

use mirror_fixture::scratch_pg;

fn context() -> (Query, AuthorizedContext, Arc<Mutex<Instant>>) {
    let query: Query = serde_json::from_value(json!({
        "request_id":"fixture-request", "conversation_id":"fixture-conversation",
        "text":"Öffentliches Serverwissen", "profile":"explain"
    }))
    .unwrap();
    let mut context: AuthorizedContext = serde_json::from_value(json!({
        "principal":{"actor_id":"fixture","channel":"fixture","scopes":[],"provider_egress":["public"]},
        "conversation_id":"fixture-conversation","knowledge_release":"fixture-release","deadline_ms":3600000,
        "budget":{"max_network_rounds":4,"max_input_tokens":100000,"max_output_tokens":2000,"max_cost_micros":50000}
    }))
    .unwrap();
    let clock = Arc::new(Mutex::new(Instant::now()));
    let now = clock.clone();
    context.request_deadline = Some(RequestDeadline::after_with_clock(
        Duration::from_secs(3600),
        move || *now.lock().unwrap(),
    ));
    (query, context, clock)
}

fn record(revision: u64, provider: bool, publication: bool, tombstone: bool) -> SourceRecordV2 {
    let content = "Der Fixture-Server bietet öffentliche Sprechstunden am Freitag.";
    let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
    let mut record = SourceRecordV2 {
        source_id: "fixture-server".into(),
        logical_id: "sprechstunden".into(),
        revision,
        content_hash: hash.clone(),
        content: content.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: hash,
        locator: "https://example.org/fixture/sprechstunden".into(),
        parser_revision: "fixture-parser-v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::known("fixture-v1".into()),
        schema_sha256: brain_contracts::source::observed_option(None),
        retrieved_at: brain_contracts::source::observed_option(None),
        source_time: brain_contracts::source::observed_option(None),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: brain_contracts::source::observed_option(None),
        policy: SourcePolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            authorization_ref: Observed::known("fixture-operator".into()),
            license: brain_contracts::source::observed_option(None),
            publication_allowed: publication,
            provider_egress_allowed: provider,
            raw_retention_allowed: false,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record
}

async fn setup(
    pg: &scratch_pg::ScratchPg,
    provider: bool,
) -> (PgStore, ReleaseRetriever<LocalPgReader>, sqlx::PgPool) {
    let socket = pg.directory.join("socket");
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(socket.to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let (address, user, directory): (Option<String>, String, String) = sqlx::query_as(
        "SELECT inet_server_addr()::text,current_user::text,current_setting('data_directory')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(address.is_none());
    assert_eq!(user, "brain_core_test");
    assert_eq!(std::path::Path::new(&directory), pg.directory.join("data"));
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    let original = record(1, provider, true, false);
    store.apply(&original).await.unwrap();
    store
        .publish_release(&CorpusRelease {
            release_id: "fixture-release".into(),
            knowledge_version: "fixture-v1".into(),
            patch: "fixture-patch".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([(
                original.source_id.clone(),
                BTreeMap::from([(original.logical_id.clone(), original.revision)]),
            )]),
        })
        .await
        .unwrap();
    let reader = LocalPgReader::new(socket, 55439, "brain_core_test", "postgres").unwrap();
    (store, ReleaseRetriever::new(reader, 6), pool)
}

fn call() -> ToolCall {
    ToolCall {
        id: "fixture-tool".into(),
        name: ToolName::ServerKnowledge,
        arguments: json!({"question":"Wann sind öffentliche Sprechstunden am Fixture-Server?"}),
    }
}

fn test_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
}

#[test]
fn real_release_port_completes_the_kernel_tool_loop_from_an_empty_first_turn() {
    use brain_contracts::{
        tools::GameContextResolver, AnswerProviderPort, Evidence, ModelBlock, PinnedGameContext,
        ProviderAnswer, ProviderFinishReason, ProviderTurn, ToolConversation, ToolDefinition,
        Usage,
    };
    use brain_kernel::{AnswerKernelPort, Kernel};

    struct KnowledgeResolver;
    impl GameContextResolver for KnowledgeResolver {
        fn resolve(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
        ) -> Result<Option<PinnedGameContext>, PortError> {
            Ok(None)
        }

        fn validate(
            &self,
            _query: &Query,
            context: &AuthorizedContext,
            game: Option<&PinnedGameContext>,
        ) -> Result<(), PortError> {
            assert!(game.is_none());
            context.check_deadline()
        }
    }

    struct Provider;
    impl AnswerProviderPort for Provider {
        fn answer(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            _evidence: &[Evidence],
        ) -> Result<ProviderAnswer, PortError> {
            panic!("Werkzeugloop darf nicht in den Textpfad wechseln")
        }

        fn answer_turn(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            evidence: &[Evidence],
            tools: &[ToolDefinition],
            conversation: &ToolConversation,
        ) -> Result<ProviderTurn, PortError> {
            let usage = Usage {
                input_tokens: 20,
                output_tokens: 10,
                network_rounds: 1,
                ..Usage::default()
            };
            if conversation.messages.is_empty() {
                assert!(evidence.is_empty());
                call().validate(tools)?;
                return Ok(ProviderTurn::ToolCalls {
                    blocks: vec![ModelBlock::ToolUse { call: call() }],
                    finish_reason: ProviderFinishReason::ToolUse,
                    usage,
                });
            }
            assert_eq!(conversation.messages.len(), 2);
            assert!(!evidence.is_empty());
            Ok(ProviderAnswer {
                text: "Die Sprechstunde ist am Freitag.".into(),
                cited_evidence_ids: evidence
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect(),
                usage,
            }
            .into())
        }
    }

    struct Knowledge(ReleaseRetriever<LocalPgReader>);
    impl RetrievalPort for Knowledge {
        fn retrieve(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            self.0.retrieve(query, context)
        }

        fn retrieve_with_usage(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<(Vec<Evidence>, Usage), PortError> {
            Ok((
                self.retrieve(query, context)?,
                Usage {
                    network_rounds: 1,
                    input_tokens: 12,
                    ..Usage::default()
                },
            ))
        }

        fn validate_evidence(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
            provider: bool,
        ) -> Result<(), PortError> {
            self.0.validate_evidence(query, context, evidence, provider)
        }

        fn validate_publication(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
        ) -> Result<(), PortError> {
            self.0.validate_publication(query, context, evidence)
        }
    }

    let pg = scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let (_store, retrieval, pool) = runtime.block_on(setup(&pg, true));
    let port = ReleaseToolExecutionPort::new(retrieval.clone())
        .with_knowledge_retrieval(Knowledge(retrieval.clone()));
    let kernel = Kernel::new(retrieval, Provider).with_tools(port, KnowledgeResolver, "fixture");
    let (query, mut context, _) = context();
    context.deadline_ms = 60000;
    let outcome = kernel.answer_for_publication_accounted(&query, &context);
    assert_eq!(
        outcome.value.status,
        brain_contracts::AnswerStatus::Answered
    );
    assert_eq!(outcome.accounting.observed.network_rounds, 3);
    assert!(!outcome.accounting.unaccounted);
    assert!(!outcome.value.citations.is_empty());
    runtime.block_on(pool.close());
}

#[test]
fn charged_knowledge_usage_survives_every_post_retrieval_rejection() {
    use brain_contracts::{Evidence, Usage, UsageAccounting};

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum KnowledgeResultCase {
        Success,
        Empty,
        Private,
        Validation,
        Deadline,
        UnreportedFailure,
        Duplicate,
    }

    struct ChargedKnowledge {
        retrieval: ReleaseRetriever<LocalPgReader>,
        mode: KnowledgeResultCase,
        clock: Arc<Mutex<Instant>>,
    }

    impl RetrievalPort for ChargedKnowledge {
        fn retrieve(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            self.retrieval.retrieve(query, context)
        }

        fn retrieve_with_usage(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<(Vec<Evidence>, Usage), PortError> {
            let mut evidence = self.retrieve(query, context)?;
            assert!(!evidence.is_empty());
            match self.mode {
                KnowledgeResultCase::Empty => evidence.clear(),
                KnowledgeResultCase::Private => evidence[0].visibility = SourceVisibility::Private,
                KnowledgeResultCase::UnreportedFailure => {
                    return Err(PortError::Unavailable(
                        "Retrievalfehler ohne Verbrauchsmeldung".into(),
                    ))
                }
                KnowledgeResultCase::Duplicate => evidence.push(evidence[0].clone()),
                _ => {}
            }
            Ok((
                evidence,
                Usage {
                    network_rounds: 1,
                    input_tokens: 12,
                    output_tokens: 4,
                    cost_micros: 7,
                    ..Usage::default()
                },
            ))
        }

        fn validate_evidence(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
            provider: bool,
        ) -> Result<(), PortError> {
            self.retrieval
                .validate_evidence(query, context, evidence, provider)?;
            match self.mode {
                KnowledgeResultCase::Validation => Err(PortError::PermissionDenied(
                    "Wissensfreigabe nach Abruf abgelehnt".into(),
                )),
                KnowledgeResultCase::Deadline => {
                    *self.clock.lock().unwrap() += Duration::from_secs(3601);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }

    let pg = scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let (_store, retrieval, pool) = runtime.block_on(setup(&pg, true));
    let expected = UsageAccounting::observed(Usage {
        network_rounds: 1,
        input_tokens: 12,
        output_tokens: 4,
        cost_micros: 7,
        ..Usage::default()
    });
    for mode in [
        KnowledgeResultCase::Success,
        KnowledgeResultCase::Empty,
        KnowledgeResultCase::Private,
        KnowledgeResultCase::Validation,
        KnowledgeResultCase::Deadline,
        KnowledgeResultCase::UnreportedFailure,
        KnowledgeResultCase::Duplicate,
    ] {
        let (query, context, clock) = context();
        let port = ReleaseToolExecutionPort::new(retrieval.clone()).with_knowledge_retrieval(
            ChargedKnowledge {
                retrieval: retrieval.clone(),
                mode,
                clock,
            },
        );
        let call = call();
        let request = call
            .validate(&port.definitions(&query, &context, None).unwrap())
            .unwrap();
        let mut foreign = context.clone();
        foreign.conversation_id = "foreign".into();
        let preflight = port
            .execute_accounted(&query, &foreign, None, &call.id, &request)
            .unwrap_err();
        assert_eq!(
            preflight.accounting.as_deref(),
            Some(&UsageAccounting::default())
        );
        let result = port.execute_accounted(&query, &context, None, &call.id, &request);
        if mode == KnowledgeResultCase::Success {
            let success = result.unwrap();
            assert_eq!(success.accounting, expected);
            assert_eq!(success.value.usage, expected.observed);
        } else {
            let failure = result.unwrap_err();
            if mode == KnowledgeResultCase::UnreportedFailure {
                assert!(failure.accounting.is_none());
            } else {
                assert_eq!(failure.accounting.as_deref(), Some(&expected));
            }
            if mode == KnowledgeResultCase::Deadline {
                assert_eq!(failure.error, PortError::BudgetExceeded);
            }
        }
    }
    runtime.block_on(pool.close());
}

#[test]
fn actual_port_preserves_request_receipt_and_current_purpose_permissions() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let (store, retrieval, pool) = runtime.block_on(setup(&pg, true));
    let port = ReleaseToolExecutionPort::new(retrieval);
    let (query, context, _) = context();
    let definitions = port.definitions(&query, &context, None).unwrap();
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].name, ToolName::ServerKnowledge);
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        port.validate_dependencies(&query, &context, None, &[], purpose)
            .unwrap();
    }
    let call = call();
    let request = call.validate(&definitions).unwrap();
    let accounted = port
        .execute_accounted(&query, &context, None, &call.id, &request)
        .unwrap();
    let execution = accounted.value;
    execution.validate_for(&call, &request, None).unwrap();
    assert!(!execution.result.is_error);
    assert!(!execution.result.evidence_ids.is_empty());
    let matches = execution.result.result["matches"].as_array().unwrap();
    assert!(!matches.is_empty());
    for item in matches {
        let fields = item.as_object().unwrap();
        assert_eq!(fields.len(), 2);
        assert!(fields.contains_key("evidence_id"));
        assert!(fields.contains_key("content"));
    }
    assert_eq!(execution.usage.network_rounds, 0);
    assert_eq!(execution.dependencies.len(), 1);
    assert!(execution.dependencies[0].game_context.is_none());
    for evidence in &execution.dependencies[0].evidence {
        evidence.validate().unwrap();
        assert_eq!(evidence.source_id, "fixture-server");
        assert_eq!(evidence.revision, 1);
        assert_eq!(evidence.provenance.as_ref().unwrap().document.revision, 1);
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        port.validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .unwrap();
    }
    runtime
        .block_on(store.apply(&record(2, true, false, false)))
        .unwrap();
    port.validate_dependencies(
        &query,
        &context,
        None,
        &execution.dependencies,
        ToolValidationPurpose::Provider,
    )
    .unwrap();
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            None,
            &execution.dependencies,
            ToolValidationPurpose::Publication,
        )
        .is_err());
    runtime
        .block_on(store.apply(&record(3, false, true, false)))
        .unwrap();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Cache,
    ] {
        assert!(port
            .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .is_err());
    }
    port.validate_dependencies(
        &query,
        &context,
        None,
        &execution.dependencies,
        ToolValidationPurpose::Publication,
    )
    .unwrap();
    assert!(port
        .execute(&query, &context, None, &call.id, &request)
        .is_err());
    runtime
        .block_on(store.apply(&record(4, true, true, false)))
        .unwrap();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        port.validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .unwrap();
    }
    runtime
        .block_on(store.apply(&record(5, true, true, true)))
        .unwrap();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(port
            .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .is_err());
    }
    assert!(port
        .execute(&query, &context, None, &call.id, &request)
        .is_err());
    runtime.block_on(pool.close());
}

#[test]
fn internal_read_is_not_a_model_grant_or_an_empty_tool_success() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let (_store, retrieval, pool) = runtime.block_on(setup(&pg, false));
    let (query, context, _) = context();
    let mut subquery = query.clone();
    subquery.text = call().arguments["question"].as_str().unwrap().into();
    assert!(!retrieval.retrieve(&subquery, &context).unwrap().is_empty());
    let port = ReleaseToolExecutionPort::new(retrieval);
    let call = call();
    let request = call
        .validate(&port.definitions(&query, &context, None).unwrap())
        .unwrap();
    assert!(port
        .execute(&query, &context, None, &call.id, &request)
        .is_err());
    runtime.block_on(pool.close());
}

#[test]
fn original_lifetime_request_binding_and_f_boundary_remain_fail_closed() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let (_store, retrieval, pool) = runtime.block_on(setup(&pg, true));
    let port = ReleaseToolExecutionPort::new(retrieval);
    let (query, context, clock) = context();
    let call = call();
    let request = call
        .validate(&port.definitions(&query, &context, None).unwrap())
        .unwrap();
    let mut missing = context.clone();
    missing.request_deadline = None;
    assert!(port.definitions(&query, &missing, None).is_err());
    let mut foreign = context.clone();
    foreign.conversation_id = "other-conversation".into();
    assert!(port
        .execute(&query, &foreign, None, &call.id, &request)
        .is_err());
    let pin = brain_contracts::PinnedGameContext {
        client_version: 6759,
        language: ToolLanguage::German,
        mechanic_revision: "fixture-pin".into(),
    };
    let execution = port
        .execute(&query, &context, None, &call.id, &request)
        .unwrap();
    assert!(matches!(
        port.validate_build_plan(
            &query,
            &context,
            &pin,
            &BuildPlanRequest {
                hero_id: 7,
                playstyle: ToolPlaystyle::Spirit,
                budget: Some(1000),
                imbues: vec![],
            },
            &execution,
            AnswerPurpose::InternalRead,
        ),
        Err(PortError::Unavailable(_))
    ));
    *clock.lock().unwrap() += Duration::from_secs(3601);
    assert_eq!(
        port.execute(&query, &context, None, &call.id, &request)
            .unwrap_err(),
        PortError::BudgetExceeded
    );
    assert_eq!(
        port.validate_dependencies(
            &query,
            &context,
            None,
            &execution.dependencies,
            ToolValidationPurpose::Cache,
        )
        .unwrap_err(),
        PortError::BudgetExceeded
    );
    runtime.block_on(pool.close());
}

async fn mirror_records(store: &PgStore, pool: &sqlx::PgPool) -> Vec<SourceRecordV2> {
    let rows: Vec<(serde_json::Value, Vec<u8>)> =
        sqlx::query_as("SELECT metadata,fixture_raw FROM brain.source_documents ORDER BY id")
            .fetch_all(pool)
            .await
            .unwrap();
    let mut records = Vec::new();
    let mut pins = BTreeMap::<String, BTreeMap<String, u64>>::new();
    for (metadata, bytes) in rows {
        let ir: brain_contracts::source::Versioned<brain_contracts::external::ExternalSourceIr> =
            serde_json::from_value(metadata["contract"].clone()).unwrap();
        let origin = ir.data.origin_artifact();
        let mut record = SourceRecordV2 {
            source_id: origin.identity.source_id.clone(),
            logical_id: origin.identity.logical_id.clone(),
            revision: 1,
            content_hash: format!("{:x}", Sha256::digest(&bytes)),
            content: String::from_utf8(bytes).unwrap(),
            visibility: ir.data.visibility,
            allowed_scopes: ir.data.allowed_scopes,
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        origin.bind_record(&mut record).unwrap();
        store.apply(&record).await.unwrap();
        pins.entry(record.source_id.clone())
            .or_default()
            .insert(record.logical_id.clone(), record.revision);
        records.push(record);
    }
    store
        .publish_release(&CorpusRelease {
            release_id: "fixture-release".into(),
            knowledge_version: "fixture-v1".into(),
            patch: "fixture-patch".into(),
            created_at_epoch: 1,
            source_revisions: pins,
        })
        .await
        .unwrap();
    records
}

#[test]
fn mirrored_identity_and_raw_profiles_require_real_receipts_and_canonical_grants() {
    use brain_contracts::tools::GameContextResolver;
    use brain_storage::entity_profile::MirroredGameContextReader;
    let pg = mirror_fixture::scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let pool = runtime.block_on(mirror_fixture::pool(&pg));
    runtime.block_on(mirror_fixture::seed_with_grants(&pool, "first", true));
    let store = PgStore::new(pool.clone());
    runtime.block_on(store.migrate_core()).unwrap();
    let records = runtime.block_on(mirror_records(&store, &pool));
    let canonical = LocalPgReader::new(
        pg.directory.join("socket"),
        55439,
        "brain_core_test",
        "postgres",
    )
    .unwrap();
    let reader = MirroredGameContextReader::new(
        pool.clone(),
        runtime.handle().clone(),
        ToolLanguage::German,
    )
    .unwrap();
    let port = ReleaseToolExecutionPort::new(ReleaseRetriever::new(canonical, 6))
        .with_mirrored_entities(reader.clone());
    let (query, context, clock) = context();
    let pin = reader.resolve(&query, &context).unwrap().unwrap();
    let definitions = port.definitions(&query, &context, Some(&pin)).unwrap();
    assert_eq!(definitions.len(), 3);
    assert!(!definitions
        .iter()
        .any(|definition| definition.name == ToolName::BuildPlan));
    let find_schema = definitions
        .iter()
        .find(|definition| definition.name == ToolName::EntityFind)
        .unwrap();
    assert_eq!(
        find_schema.input_schema["properties"]["language"]["enum"],
        json!(["german"])
    );
    let foreign_language = ToolCall {
        id: "foreign-language".into(),
        name: ToolName::EntityFind,
        arguments: json!({"query":"Prüfdaten","kind":"hero","language":"english"}),
    };
    assert!(foreign_language.validate(&definitions).is_err());
    let mut saved = None;
    for (kind, id, name) in [
        ("hero", 7, "Prüfdaten"),
        ("item", 8, "Prüfitem"),
        ("ability", 9, "Prüffähigkeit"),
    ] {
        let call = ToolCall {
            id: "find".into(),
            name: ToolName::EntityFind,
            arguments: json!({"query":name,"kind":kind,"language":"german"}),
        };
        let request = call.validate(&definitions).unwrap();
        let execution = port
            .execute(&query, &context, Some(&pin), &call.id, &request)
            .unwrap();
        assert_eq!(
            execution.result.result["matches"],
            json!([{"kind":kind,"id":id,"name":name}])
        );
        assert_eq!(execution.dependencies[0].evidence.len(), 2);
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Cache,
            ToolValidationPurpose::Publication,
        ] {
            port.validate_dependencies(
                &query,
                &context,
                Some(&pin),
                &execution.dependencies,
                purpose,
            )
            .unwrap();
        }
        saved = Some(execution);
    }
    let profile = ToolCall {
        id: "profile".into(),
        name: ToolName::EntityProfile,
        arguments: json!({"entity":{"kind":"item","id":8},"fields":["zero","missing","observation"]}),
    };
    let request = profile.validate(&definitions).unwrap();
    let execution = port
        .execute(&query, &context, Some(&pin), &profile.id, &request)
        .unwrap();
    assert_eq!(
        execution.result.result["fields"]["zero"],
        json!({"state":"known","value":0})
    );
    assert_eq!(
        execution.result.result["fields"]["missing"]["state"],
        "unknown"
    );
    assert_eq!(
        execution.result.result["fields"]["observation"]["value"],
        "first"
    );
    let mut altered = execution.dependencies.clone();
    altered[0].evidence[0].content = "Erfundene Spielwerte".into();
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            Some(&pin),
            &altered,
            ToolValidationPurpose::Provider
        )
        .is_err());
    altered = execution.dependencies.clone();
    altered[0].evidence.pop();
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            Some(&pin),
            &altered,
            ToolValidationPurpose::Cache
        )
        .is_err());
    let mut foreign = pin.clone();
    foreign.language = ToolLanguage::English;
    assert!(port
        .execute(&query, &context, Some(&foreign), &profile.id, &request)
        .is_err());
    let mut forbidden = profile.clone();
    forbidden.arguments["scenario"] = json!({"progression":{"kind":"boons","value":0}});
    assert!(forbidden.validate(&definitions).is_err());
    let missing = ToolCall {
        id: "missing".into(),
        name: ToolName::EntityFind,
        arguments: json!({"query":"Nichtvorhanden","language":"german"}),
    };
    let absent = port
        .execute(
            &query,
            &context,
            Some(&pin),
            &missing.id,
            &missing.validate(&definitions).unwrap(),
        )
        .unwrap();
    assert_eq!(absent.result.result["matches"], json!([]));
    assert_eq!(absent.dependencies[0].evidence.len(), 3);
    let document_id = reader
        .read_pinned(&context, &pin)
        .unwrap()
        .asset("items", Some("german"))
        .unwrap()
        .receipt
        .endpoint
        .source_document_id;
    let metadata: serde_json::Value = runtime
        .block_on(
            sqlx::query_scalar("SELECT metadata FROM brain.source_documents WHERE id=$1")
                .bind(document_id)
                .fetch_one(&pool),
        )
        .unwrap();
    let mut forged = metadata.clone();
    forged["contract"]["data"]["payload"]["value"][0]["zero"] = json!(999);
    runtime
        .block_on(
            sqlx::query("UPDATE brain.source_documents SET metadata=$1 WHERE id=$2")
                .bind(forged)
                .bind(document_id)
                .execute(&pool),
        )
        .unwrap();
    let forged_pin = reader.resolve(&query, &context).unwrap().unwrap();
    assert_ne!(forged_pin, pin);
    let find = ToolCall {
        id: "forged-find".into(),
        name: ToolName::EntityFind,
        arguments: json!({"query":"Prüfitem","kind":"item","language":"german"}),
    };
    for tool_call in [&find, &profile] {
        let forged_request = tool_call
            .validate(
                &port
                    .definitions(&query, &context, Some(&forged_pin))
                    .unwrap(),
            )
            .unwrap();
        assert!(matches!(
            port.execute(
                &query,
                &context,
                Some(&forged_pin),
                &tool_call.id,
                &forged_request,
            ),
            Err(PortError::PermissionDenied(_))
        ));
    }
    runtime
        .block_on(
            sqlx::query("UPDATE brain.source_documents SET metadata=$1 WHERE id=$2")
                .bind(metadata)
                .bind(document_id)
                .execute(&pool),
        )
        .unwrap();
    reader.validate(&query, &context, Some(&pin)).unwrap();
    let original = records
        .iter()
        .find(|record| record.logical_id.contains("items/german"))
        .unwrap();
    let mut denied = original.clone();
    denied.revision = 2;
    let mut origin = brain_contracts::source::origin_from_record(&denied).unwrap();
    origin.policy.publication_allowed = false;
    origin.bind_record(&mut denied).unwrap();
    runtime.block_on(store.apply(&denied)).unwrap();
    let saved = saved.unwrap();
    port.validate_dependencies(
        &query,
        &context,
        Some(&pin),
        &saved.dependencies,
        ToolValidationPurpose::Provider,
    )
    .unwrap();
    assert!(port
        .validate_dependencies(
            &query,
            &context,
            Some(&pin),
            &saved.dependencies,
            ToolValidationPurpose::Publication
        )
        .is_err());
    denied.revision = 3;
    origin.policy.provider_egress_allowed = false;
    origin.bind_record(&mut denied).unwrap();
    runtime.block_on(store.apply(&denied)).unwrap();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Cache,
    ] {
        assert!(port
            .validate_dependencies(&query, &context, Some(&pin), &saved.dependencies, purpose)
            .is_err());
    }
    assert!(port
        .execute(&query, &context, Some(&pin), &profile.id, &request)
        .is_err());
    *clock.lock().unwrap() += Duration::from_secs(3601);
    assert_eq!(
        port.validate_dependencies(
            &query,
            &context,
            Some(&pin),
            &saved.dependencies,
            ToolValidationPurpose::Cache
        )
        .unwrap_err(),
        PortError::BudgetExceeded
    );
    runtime.block_on(pool.close());
}

#[test]
fn public_mirror_originals_without_canonical_registration_are_not_tool_grants() {
    use brain_contracts::tools::GameContextResolver;
    use brain_storage::entity_profile::MirroredGameContextReader;
    let pg = mirror_fixture::scratch_pg::ScratchPg::start();
    let runtime = test_runtime();
    let pool = runtime.block_on(mirror_fixture::pool(&pg));
    runtime.block_on(mirror_fixture::seed_with_grants(
        &pool,
        "unregistered",
        true,
    ));
    let store = PgStore::new(pool.clone());
    runtime.block_on(store.migrate_core()).unwrap();
    let original = record(1, true, true, false);
    runtime.block_on(store.apply(&original)).unwrap();
    runtime
        .block_on(store.publish_release(&CorpusRelease {
            release_id: "fixture-release".into(),
            knowledge_version: "fixture-v1".into(),
            patch: "fixture-patch".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([(
                original.source_id.clone(),
                BTreeMap::from([(original.logical_id.clone(), 1)]),
            )]),
        }))
        .unwrap();
    let canonical = LocalPgReader::new(
        pg.directory.join("socket"),
        55439,
        "brain_core_test",
        "postgres",
    )
    .unwrap();
    let reader = MirroredGameContextReader::new(
        pool.clone(),
        runtime.handle().clone(),
        ToolLanguage::German,
    )
    .unwrap();
    let port = ReleaseToolExecutionPort::new(ReleaseRetriever::new(canonical, 6))
        .with_mirrored_entities(reader.clone());
    let (query, context, _) = context();
    let pin = reader.resolve(&query, &context).unwrap().unwrap();
    let call = ToolCall {
        id: "find".into(),
        name: ToolName::EntityFind,
        arguments: json!({"query":"Prüfdaten","kind":"hero","language":"german"}),
    };
    let request = call
        .validate(&port.definitions(&query, &context, Some(&pin)).unwrap())
        .unwrap();
    assert!(matches!(
        port.execute(&query, &context, Some(&pin), &call.id, &request),
        Err(PortError::PermissionDenied(_))
    ));
    runtime.block_on(pool.close());
}
