use brain_contracts::{
    answer_contract::{is_original_bound, NO_SUPPORTED_ANSWER},
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    tools::{GameContextResolver, ToolLanguage},
    value::Observed,
    *,
};
use brain_kernel::{AnswerKernelPort, Kernel};
use brain_storage::{entity_profile::MirroredGameContextReader, MemoryRepository};
use dbrain_retrieval::{ReleaseRetriever, ReleaseToolExecutionPort};
use std::{
    collections::{BTreeMap, BTreeSet},
    result::Result,
};

#[path = "../../brain-storage/tests/mirror_game_context.rs"]
mod mirror_fixture;

const ORIGINAL: &str = "Seelenurne\nSoul Urn\nHalte Abstand und nutze Deckung.";

async fn corpus() -> MemoryRepository {
    let mut record = SourceRecordV2 {
        source_id: "localization-original".into(),
        logical_id: "localization/german.txt".into(),
        revision: 1,
        content_hash: format!("{:x}", <sha2::Sha256 as sha2::Digest>::digest(ORIGINAL)),
        content: ORIGINAL.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
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
            api_version: "localization-v1".into(),
            original_revision: Some("frozen-original".into()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: "https://example.org/localization/german.txt".into(),
        parser_revision: "original-text-v1".into(),
        parser_family: "localization".into(),
        schema_version: Observed::known("localization-v1".into()),
        schema_sha256: source::observed_option(None),
        retrieved_at: source::observed_option(None),
        source_time: source::observed_option(None),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: source::observed_option(None),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: BTreeSet::new(),
            authorization_ref: Observed::known("test-operator".into()),
            license: source::observed_option(None),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    let store = MemoryRepository::default();
    store.apply_record(record).unwrap();
    let release = store.release_from_heads("r1", "k1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}

fn query(text: &str) -> Query {
    serde_json::from_value(serde_json::json!({
        "request_id":"name-contract", "conversation_id":"c1", "text":text
    }))
    .unwrap()
}

fn context() -> AuthorizedContext {
    let context: AuthorizedContext = serde_json::from_value(serde_json::json!({
        "principal":{"actor_id":"test-operator","channel":"test","scopes":[],"provider_egress":["public"]},
        "conversation_id":"c1","knowledge_release":"r1","deadline_ms":60000,
        "budget":{"max_network_rounds":4,"max_input_tokens":100000,"max_output_tokens":2000,"max_cost_micros":50000}
    }))
    .unwrap();
    context.with_request_deadline().into_owned()
}

async fn mirror_corpus(pool: &sqlx::PgPool) -> MemoryRepository {
    let rows: Vec<(serde_json::Value, Vec<u8>)> =
        sqlx::query_as("SELECT metadata,fixture_raw FROM brain.source_documents ORDER BY id")
            .fetch_all(pool)
            .await
            .unwrap();
    let store = MemoryRepository::default();
    for (metadata, bytes) in rows {
        let ir: source::Versioned<external::ExternalSourceIr> =
            serde_json::from_value(metadata["contract"].clone()).unwrap();
        let origin = ir.data.origin_artifact();
        let mut record = SourceRecordV2 {
            source_id: origin.identity.source_id.clone(),
            logical_id: origin.identity.logical_id.clone(),
            revision: 1,
            content_hash: format!("{:x}", <sha2::Sha256 as sha2::Digest>::digest(&bytes)),
            content: String::from_utf8(bytes).unwrap(),
            visibility: ir.data.visibility,
            allowed_scopes: ir.data.allowed_scopes,
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        origin.bind_record(&mut record).unwrap();
        store.apply_record(record).unwrap();
    }
    let release = store.release_from_heads("r1", "k1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}

#[test]
fn real_mirror_names_are_supported_but_unknown_field_keys_and_raw_json_are_not() {
    let pg = mirror_fixture::scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let pool = runtime.block_on(mirror_fixture::pool(&pg));
    runtime.block_on(mirror_fixture::seed_with_grants(
        &pool,
        "private-metadata-marker",
        true,
    ));
    let resolver = MirroredGameContextReader::new(
        pool.clone(),
        runtime.handle().clone(),
        ToolLanguage::German,
    )
    .unwrap();
    let store = runtime.block_on(mirror_corpus(&pool));
    let request = query("Prüfitem");
    let context = context();
    let pin = resolver.resolve(&request, &context).unwrap().unwrap();
    let port = ReleaseToolExecutionPort::new(ReleaseRetriever::new(store, 6))
        .with_mirrored_entities(resolver);
    let definitions = port.definitions(&request, &context, Some(&pin)).unwrap();
    for call in [
        ToolCall {
            id: "name-find".into(),
            name: ToolName::EntityFind,
            arguments: serde_json::json!({"query":"Prüfitem", "language":"german"}),
        },
        ToolCall {
            id: "name-fields".into(),
            name: ToolName::EntityProfile,
            arguments: serde_json::json!({"entity":{"kind":"item", "id":8}, "fields":["name","Riftwalker"]}),
        },
    ] {
        let subrequest = call.validate(&definitions).unwrap();
        let execution = port
            .execute(&request, &context, Some(&pin), &call.id, &subrequest)
            .unwrap();
        assert!(!execution.result.is_error);
        port.validate_dependencies(
            &request,
            &context,
            Some(&pin),
            &execution.dependencies,
            ToolValidationPurpose::Provider,
        )
        .unwrap();
        let evidence: Vec<_> = execution
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.evidence.clone())
            .collect();
        let ids = execution.result.evidence_ids.clone();
        for (text, supported) in [
            ("Prüfitem", true),
            ("Riftwalker", false),
            ("private-metadata-marker", false),
        ] {
            assert_eq!(
                is_original_bound(
                    &request,
                    &evidence,
                    &ProviderAnswer {
                        text: text.into(),
                        cited_evidence_ids: ids.clone(),
                        usage: Usage::default()
                    }
                ),
                supported
            );
        }
        let raw = ProviderAnswer {
            text: execution.result.result.to_string(),
            cited_evidence_ids: ids,
            usage: Usage::default(),
        };
        assert!(!is_original_bound(&request, &evidence, &raw));
    }
    runtime.block_on(pool.close());
}

#[derive(Clone, Copy)]
enum Citations {
    Known,
    Missing,
    Foreign,
    Duplicate,
}

struct Generation {
    text: &'static str,
    citations: Citations,
    use_tool: bool,
}

impl Generation {
    fn final_answer(&self, evidence: &[Evidence]) -> ProviderAnswer {
        let ids = match self.citations {
            Citations::Known => evidence
                .iter()
                .take(1)
                .map(|item| item.evidence_id.clone())
                .collect(),
            Citations::Missing => Vec::new(),
            Citations::Foreign => vec!["foreign-proof".into()],
            Citations::Duplicate => evidence
                .iter()
                .take(1)
                .flat_map(|item| [item.evidence_id.clone(), item.evidence_id.clone()])
                .collect(),
        };
        ProviderAnswer {
            text: self.text.into(),
            cited_evidence_ids: ids,
            usage: Usage {
                input_tokens: 1,
                output_tokens: 1,
                network_rounds: 1,
                ..Usage::default()
            },
        }
    }
}

impl AnswerProviderPort for Generation {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        assert!(!self.use_tool);
        Ok(self.final_answer(evidence))
    }

    fn answer_turn(
        &self,
        query: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
        definitions: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> Result<ProviderTurn, PortError> {
        assert!(self.use_tool);
        if conversation.messages.is_empty() {
            let call = ToolCall {
                id: "source-read".into(),
                name: ToolName::ServerKnowledge,
                arguments: serde_json::json!({"question":query.text}),
            };
            call.validate(definitions)?;
            Ok(ProviderTurn::ToolCalls {
                blocks: vec![ModelBlock::ToolUse { call }],
                finish_reason: ProviderFinishReason::ToolUse,
                usage: Usage {
                    input_tokens: 1,
                    output_tokens: 1,
                    network_rounds: 1,
                    ..Usage::default()
                },
            })
        } else {
            Ok(self.final_answer(evidence).into())
        }
    }
}

#[test]
fn original_bound_contract_covers_direct_and_real_tool_final_paths_without_name_declarations() {
    let pg = mirror_fixture::scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let pool = runtime.block_on(mirror_fixture::pool(&pg));
    runtime.block_on(mirror_fixture::seed_with_grants(
        &pool,
        "actual-resolver",
        true,
    ));
    let resolver = MirroredGameContextReader::new(
        pool.clone(),
        runtime.handle().clone(),
        ToolLanguage::German,
    )
    .unwrap();
    let store = runtime.block_on(corpus());
    for quality in [false, true] {
        for use_tool in [false, true] {
            for (text, citations, accepted) in [
                ("Seelenurne", Citations::Known, true),
                ("Soul Urn", Citations::Known, true),
                ("Halte Abstand und nutze Deckung.", Citations::Known, true),
                ("Falls du mit Rift den Riftwalker meinst, kenne ich keinen bestätigten Spielwert.", Citations::Known, false),
                ("Riftwalker", Citations::Missing, false),
                ("Riftwalker", Citations::Foreign, false),
                ("Riftwalker", Citations::Duplicate, false),
                ("Seelenurne und Riftwalker", Citations::Known, false),
                ("Seelenurne", Citations::Foreign, false),
                ("Seelenurne", Citations::Duplicate, false),
                ("Seelenurne", Citations::Missing, false),
                ("Seelenurne erscheint nach zehn Minuten.", Citations::Known, false),
                ("Nimm dir Zeit und spiel ruhig.", Citations::Missing, false),
                (NO_SUPPORTED_ANSWER, Citations::Missing, false),
                ("", Citations::Missing, false),
            ] {
                let retrieval = ReleaseRetriever::new(store.clone(), 6);
                let request = query("Seelenurne");
                let context = context();
                let originals = retrieval.retrieve(&request, &context).unwrap();
                assert!(!originals.is_empty());
                retrieval.validate_evidence(&request, &context, &originals, true).unwrap();
                let mut kernel = Kernel::new(retrieval.clone(), Generation { text, citations, use_tool }).with_quality_filters(quality);
                if use_tool {
                    kernel = kernel.with_tools(ReleaseToolExecutionPort::new(retrieval), resolver.clone(), "test-generation");
                }
                let answer = kernel.answer(&request, &context);
                assert_eq!(answer.status, if accepted { AnswerStatus::Answered } else { AnswerStatus::Unverified }, "quality={quality}, tool={use_tool}, text={text}");
                if accepted {
                    assert_eq!(answer.text, text);
                    assert!(!answer.citations.is_empty());
                    assert!(is_original_bound(&request, &answer.citations, &ProviderAnswer { text: answer.text, cited_evidence_ids: answer.citations.iter().map(|item| item.evidence_id.clone()).collect(), usage: answer.usage }));
                } else {
                    assert_eq!(answer.text, NO_SUPPORTED_ANSWER);
                    assert!(answer.citations.is_empty());
                }
            }
        }
    }
    runtime.block_on(pool.close());
}

#[test]
fn absent_object_uses_safe_nameless_text_in_direct_and_real_tool_paths() {
    let pg = mirror_fixture::scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let pool = runtime.block_on(mirror_fixture::pool(&pg));
    runtime.block_on(mirror_fixture::seed_with_grants(
        &pool,
        "actual-resolver",
        true,
    ));
    let resolver = MirroredGameContextReader::new(
        pool.clone(),
        runtime.handle().clone(),
        ToolLanguage::German,
    )
    .unwrap();
    let store = runtime.block_on(corpus());
    for quality in [false, true] {
        for use_tool in [false, true] {
            let retrieval = ReleaseRetriever::new(store.clone(), 6);
            let request = query("wann kommt das Rift wann respawnt das, was sind die Timer");
            let context = context();
            assert!(retrieval.retrieve(&request, &context).unwrap().is_empty());
            let mut kernel = Kernel::new(
                retrieval.clone(),
                Generation {
                    text: "Falls du mit Rift den Riftwalker meinst",
                    citations: Citations::Missing,
                    use_tool,
                },
            )
            .with_quality_filters(quality);
            if use_tool {
                kernel = kernel.with_tools(
                    ReleaseToolExecutionPort::new(retrieval),
                    resolver.clone(),
                    "test-generation",
                );
            }
            let answer = kernel.answer(&request, &context);
            assert_eq!(answer.status, AnswerStatus::Unverified);
            assert_eq!(answer.text, NO_SUPPORTED_ANSWER);
            assert!(answer.citations.is_empty());
        }
    }
    runtime.block_on(pool.close());
}
