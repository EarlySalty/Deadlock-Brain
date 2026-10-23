use super::*;
#[path = "timer_pg_tests.rs"]
mod timers;
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

use crate::{assets_fixture, scratch_pg};

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
    let values = if large {
        vec![Value::String("x".repeat(2_000)); 40]
    } else {
        vec![json!(780)]
    };
    let entity = ingest_fixture(admin, name, key, revision, values, true).await;
    playable_hero(admin, key).await;
    entity
}

async fn playable_hero(admin: &PgPool, key: &str) {
    if key.starts_with("hero_") {
        assets_fixture::playable(admin, key).await;
    }
}

async fn ingest_fixture(
    admin: &PgPool,
    name: &str,
    key: &str,
    revision: &str,
    values: Vec<Value>,
    succeeded: bool,
) -> i64 {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("fixture").await.unwrap();
    let large = values.len() > 1;
    let bytes =
        serde_json::to_vec(&json!({"name":name,"values":values,"revision":revision})).unwrap();
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
                FactValue::Number(value.as_f64().unwrap())
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
            succeeded,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
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
        let names: BTreeSet<_> = tools.iter().map(|tool| tool.name).collect();
        assert_eq!(names.len(), tools.len());
        assert!(BTreeSet::from([
            ToolName::FindEntity,
            ToolName::GetHero,
            ToolName::ServerKnowledge,
        ])
        .is_subset(&names));
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
                    claims: Vec::new(),
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

fn coherence_set(
    key: &str,
    alias: &str,
    raw: &brain_storage::gamefacts::RawObject,
    full: bool,
) -> FactSet {
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: raw.content_sha256,
    };
    let values = if full {
        vec![
            ("name_en", FactValue::Text("Previous".into())),
            ("name_de", FactValue::Text("Vorher".into())),
            ("health", FactValue::Number(100.0)),
            ("armor", FactValue::Number(10.0)),
            ("weapon_damage", FactValue::Number(20.0)),
        ]
    } else {
        vec![
            ("name_en", FactValue::Text("Current".into())),
            ("health", FactValue::Number(200.0)),
        ]
    };
    FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: receipt.clone(),
        }],
        aliases: vec![Alias {
            entity_key: key.into(),
            alias: alias.into(),
            lang: "en".into(),
            receipt: receipt.clone(),
        }],
        facts: values
            .into_iter()
            .map(|(stat_key, value)| Fact {
                entity_key: key.into(),
                stat_key: stat_key.into(),
                value,
                receipt: receipt.clone(),
            })
            .collect(),
        ..FactSet::default()
    }
}

#[test]
fn superseded_hero_objects_hide_their_aliases_and_profile_values() {
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
    let (admin, writer, pool, entities) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2)
            .connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
            .execute(&admin).await.unwrap();
        let migrator = brain_storage::PgStore::new(admin.clone());
        migrator.migrate_gamefacts().await.unwrap();
        let writer = sqlx::postgres::PgPoolOptions::new().max_connections(1)
            .connect_with(options.clone().username("brain_ingest")).await.unwrap();
        let roles: (String,String,bool) = sqlx::query_as("SELECT session_user::text,current_user::text,(SELECT rolsuper FROM pg_roles WHERE rolname=current_user)")
            .fetch_one(&writer).await.unwrap();
        assert_eq!(roles, ("brain_ingest".into(),"brain_ingest".into(),false));
        let store = GameFactsStore::new(writer.clone());
        let mut entities = Vec::new();
        for dimension in ["raw", "run", "extractor"] {
            let key = format!("hero_coherence_{dimension}");
            let retired = "obsolete".to_owned();
            let active = "current".to_owned();
            let run = store.begin_run("coherence").await.unwrap();
            let raw = store.capture(run,"coherence","previous",&format!("{key}.json"),br#"{"name":"Previous","health":100,"armor":10,"weapon_damage":20}"#.to_vec()).await.unwrap();
            store.apply(run,&raw,"previous",&coherence_set(&key,&retired,&raw,true)).await.unwrap();
            let (current_run,current_raw,extractor) = match dimension {
                "raw" => (run,store.capture(run,"coherence","current",&format!("{key}.json"),br#"{"name":"Current","health":200}"#.to_vec()).await.unwrap(),"previous"),
                "run" => {
                    store.finish_run(run,true,Counts::default()).await.unwrap();
                    (store.begin_run("coherence").await.unwrap(),raw.clone(),"previous")
                }
                "extractor" => (run,raw.clone(),"current"),
                _ => unreachable!(),
            };
            store.apply(current_run,&current_raw,extractor,&coherence_set(&key,&active,&current_raw,false)).await.unwrap();
            store.finish_run(current_run,true,Counts::default()).await.unwrap();
            let entity: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
                .bind(&key).fetch_one(&admin).await.unwrap();
            let historical: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.current_fact WHERE entity_id=$1")
                .bind(entity).fetch_one(&admin).await.unwrap();
            assert_eq!(historical,5);
            playable_hero(&admin,&key).await;
            entities.push((entity,retired,active));
        }
        brain_storage::PgStore::new(writer.clone()).check_gamefacts_schema().await.unwrap();
        let pool = tool_pool(options.username("brain_readonly"),Duration::from_secs(2));
        (admin,writer,pool,entities)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    for (entity, retired, active) in entities {
        let profile = tools
            .execute(
                &query(),
                &context(),
                None,
                "coherent",
                &request(ToolName::GetHero, json!({"hero":entity})),
            )
            .unwrap();
        assert_eq!(profile.result.result["identity"]["name_en"], "Current");
        assert_eq!(profile.result.result["identity"]["name_de"], Value::Null);
        assert_eq!(
            profile.result.result["identity"]["facts"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            profile.result.result["stats"]["facts"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            profile.result.result["stats"]["facts"][0]["stat_key"],
            "health"
        );
        assert_eq!(profile.result.result["stats"]["facts"][0]["value"], 200.0);
        assert_eq!(profile.result.result["weapon"]["facts"], json!([]));
        runtime.block_on(async {
            let mut connection = pool.begin().await.unwrap();
            sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
                .execute(&mut *connection)
                .await
                .unwrap();
            assert_session(&mut connection).await.unwrap();
            for sql in [EXACT, FUZZY] {
                let stale = sqlx::query(sql)
                    .bind(&retired)
                    .bind(Some("hero"))
                    .bind(5_i64)
                    .fetch_all(&mut *connection)
                    .await
                    .unwrap();
                assert!(stale
                    .iter()
                    .all(|row| row.get::<i64, _>("entity_id") != entity));
                let current = sqlx::query(sql)
                    .bind(&active)
                    .bind(Some("hero"))
                    .bind(5_i64)
                    .fetch_all(&mut *connection)
                    .await
                    .unwrap();
                assert!(current
                    .iter()
                    .any(|row| row.get::<i64, _>("entity_id") == entity));
            }
            connection.commit().await.unwrap();
        });
    }
    runtime.block_on(async {
        pool.close().await;
        writer.close().await;
        admin.close().await;
    });
}

fn language_set(
    key: &str,
    alias: &str,
    lang: &str,
    raw: &brain_storage::gamefacts::RawObject,
    identity: bool,
) -> FactSet {
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: raw.content_sha256,
    };
    FactSet {
        entities: if identity {
            vec![Entity {
                kind: "item".into(),
                key: key.into(),
                receipt: receipt.clone(),
            }]
        } else {
            Vec::new()
        },
        aliases: vec![Alias {
            entity_key: key.into(),
            alias: alias.into(),
            lang: lang.into(),
            receipt: receipt.clone(),
        }],
        facts: vec![Fact {
            entity_key: key.into(),
            stat_key: format!("name_{lang}"),
            value: FactValue::Text(alias.into()),
            receipt,
        }],
        ..FactSet::default()
    }
}

fn found_entities(tools: &PgTools<Delegate>, text: &str) -> Vec<i64> {
    let found = tools
        .execute(
            &query(),
            &context(),
            None,
            "language",
            &request(ToolName::FindEntity, json!({"text":text,"kind":"item"})),
        )
        .unwrap();
    if found.result.is_error {
        assert_eq!(found.result.result["error"], "entity_not_found");
        return Vec::new();
    }
    found.result.result["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entity| entity["entity_id"].as_i64().unwrap())
        .collect()
}

#[test]
fn item_aliases_from_the_current_english_file_resolve_to_the_german_identity() {
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
    let key = "upgrade_language_coherence";
    let (admin, writer, pool, entity) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2)
            .connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate")
            .execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        let writer = sqlx::postgres::PgPoolOptions::new().max_connections(1)
            .connect_with(options.clone().username("brain_ingest")).await.unwrap();
        let store = GameFactsStore::new(writer.clone());
        let run = store.begin_run("assets").await.unwrap();
        let german = store.capture(run,"assets","r1",&format!("items/de/{key}.json"),br#"{"name":"Kugelhagel"}"#.to_vec()).await.unwrap();
        store.apply(run,&german,"assets",&language_set(key,"Kugelhagel","de",&german,true)).await.unwrap();
        let english = store.capture(run,"assets","r1",&format!("items/en/{key}.json"),br#"{"name":"Bullet Storm"}"#.to_vec()).await.unwrap();
        store.apply(run,&english,"assets",&language_set(key,"Bullet Storm","en",&english,false)).await.unwrap();
        store.finish_run(run,true,Counts::default()).await.unwrap();
        let entity: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
            .bind(key).fetch_one(&admin).await.unwrap();
        let receipts: (i64,i64) = sqlx::query_as("SELECT i.raw_object_id,a.raw_object_id FROM serve.current_entity i JOIN serve.current_alias a ON a.entity_id=i.id AND a.lang='en' WHERE i.id=$1")
            .bind(entity).fetch_one(&admin).await.unwrap();
        assert_eq!(receipts,(german.id,english.id));
        let pool = tool_pool(options.username("brain_readonly"),Duration::from_secs(2));
        (admin,writer,pool,entity)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    assert_eq!(found_entities(&tools, "Bullet Storm"), vec![entity]);
    assert_eq!(found_entities(&tools, "Bullet Stor"), vec![entity]);
    assert_eq!(found_entities(&tools, "Kugelhagel"), vec![entity]);
    runtime.block_on(async {
        let store = GameFactsStore::new(writer.clone());
        let run = store.begin_run("assets").await.unwrap();
        let english = store
            .capture(
                run,
                "assets",
                "r2",
                &format!("items/en/{key}.json"),
                br#"{"name":"Mystic Burst"}"#.to_vec(),
            )
            .await
            .unwrap();
        store
            .apply(
                run,
                &english,
                "assets",
                &language_set(key, "Mystic Burst", "en", &english, false),
            )
            .await
            .unwrap();
        store
            .finish_run(run, true, Counts::default())
            .await
            .unwrap();
    });
    assert_eq!(found_entities(&tools, "Bullet Storm"), Vec::<i64>::new());
    assert_eq!(found_entities(&tools, "Mystic Burst"), vec![entity]);
    assert_eq!(found_entities(&tools, "Kugelhagel"), vec![entity]);
    runtime.block_on(async {
        pool.close().await;
        writer.close().await;
        admin.close().await;
    });
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
        publish(&admin,"DocsOnly","docs_only","r1",false).await;
        sqlx::query("UPDATE canon.entity SET kind='doc_page' WHERE key='docs_only'").execute(&admin).await.unwrap();
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
    for text in ["DocsOnly", "DocsOnlu"] {
        let document = tools
            .execute(
                &query,
                &context,
                None,
                "doc-excluded",
                &request(ToolName::FindEntity, json!({"text":text})),
            )
            .unwrap();
        assert!(document.result.is_error);
        assert_eq!(document.result.result["error"], "entity_not_found");
    }
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
    });
    let mut changed_fact = hero.clone();
    changed_fact.result.result["stats"]["facts"][0]["value"] = json!(781.0);
    changed_fact.dependencies[0] = changed_fact.dependencies[0]
        .clone()
        .with_canonical_result(&changed_fact.result)
        .unwrap();
    assert_eq!(changed_fact.result.evidence_ids, hero.result.evidence_ids);
    assert_eq!(
        changed_fact.dependencies[0].evidence,
        hero.dependencies[0].evidence
    );
    assert_eq!(
        changed_fact.dependencies[0]
            .request
            .without_canonical_result(),
        hero.dependencies[0].request.without_canonical_result()
    );
    assert_ne!(
        changed_fact.dependencies[0].request,
        hero.dependencies[0].request
    );
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &changed_fact.dependencies,
            ToolValidationPurpose::Publication
        )
        .is_err());
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
    let mut tight = context.clone();
    tight.budget.max_input_tokens = 8_000;
    assert_eq!(
        kernel.answer_for_publication(&query, &tight).status,
        AnswerStatus::BudgetExceeded
    );
    let adapter_context = context.clone();
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
        assert_eq!(
            ingest_fixture(&admin, "Baba", "hero_baba", "r2", vec![json!(781)], true).await,
            baba
        );
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let fresh = tools
        .execute(&query, &context, None, "fresh", &hero_request)
        .unwrap();
    assert_eq!(fresh.result.result["stats"]["facts"][0]["value"], 781.0);
    assert_eq!(
        fresh.result.result["stats"]["facts"][0]["source_revision"],
        "r2"
    );
    assert_ne!(fresh.result.evidence_ids, hero.result.evidence_ids);
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(tools
            .validate_dependencies(&query, &context, None, &hero.dependencies, purpose)
            .is_err());
        tools
            .validate_dependencies(&query, &context, None, &fresh.dependencies, purpose)
            .unwrap();
    }
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
    let unpublished = runtime.block_on(async {
        let entity = ingest_fixture(
            &admin,
            "Unpublished",
            "hero_unpublished",
            "failed",
            vec![json!(780)],
            false,
        )
        .await;
        let (facts, failed_runs, published_receipts, profiles): (i64, i64, i64, i64) =
            sqlx::query_as("SELECT (SELECT count(*) FROM canon.fact WHERE entity_id=$1), (SELECT count(DISTINCT r.ingest_run_id) FROM canon.fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN ops.ingest_run run ON run.id=r.ingest_run_id WHERE f.entity_id=$1 AND run.status='failed'), (SELECT count(*) FROM canon.fact f JOIN serve.published_receipt r ON r.id=f.receipt_id WHERE f.entity_id=$1), (SELECT count(*) FROM serve.hero_profile WHERE entity_id=$1)")
                .bind(entity)
                .fetch_one(&admin)
                .await
                .unwrap();
        assert_eq!((facts, failed_runs, published_receipts, profiles), (2, 1, 0, 0));
        eprintln!("PostgreSQL fixture proof: facts={facts}, failed_runs={failed_runs}, published_receipts={published_receipts}, profiles={profiles}");
        entity
    });
    for unpublished_request in [
        request(ToolName::GetHero, json!({"hero":unpublished})),
        request(
            ToolName::FindEntity,
            json!({"text":"Unpublished","kind":"hero"}),
        ),
    ] {
        let execution = tools
            .execute(&query, &context, None, "unpublished", &unpublished_request)
            .unwrap();
        assert!(execution.result.is_error);
        assert!(execution.result.evidence_ids.is_empty());
        assert!(execution.dependencies.is_empty());
    }
    tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &fresh.dependencies,
            ToolValidationPurpose::Cache,
        )
        .unwrap();
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}

#[test]
fn hero_lookups_follow_published_assets_playability() {
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
    let (admin, pool, ids) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2)
            .connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
            .execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        let fixture = |name: &'static str, key: &'static str| {
            let admin = admin.clone();
            async move { ingest_fixture(&admin, name, key, "r1", vec![json!(780)], true).await }
        };
        let baba = fixture("Baba", "hero_baba").await;
        assets_fixture::flagged(&admin, "hero_baba", json!({"player_selectable":true,"disabled":false,"in_development":true})).await;
        let disabled = fixture("Verborgen", "hero_hidden_fixture").await;
        assets_fixture::flagged(&admin, "hero_hidden_fixture", json!({"player_selectable":false,"disabled":true,"in_development":false})).await;
        let dummy = fixture("Zielpuppe", "hero_target_dummy").await;
        let item = fixture("Klinge", "upgrade_fixture_blade").await;
        let ability = fixture("Funke", "ability_fixture_spark").await;
        for (key, kind) in [("upgrade_fixture_blade", "item"), ("ability_fixture_spark", "ability")] {
            sqlx::query("UPDATE canon.entity SET kind=$2 WHERE key=$1").bind(key).bind(kind).execute(&admin).await.unwrap();
        }
        let pool = tool_pool(options.clone().username("brain_readonly"), Duration::from_secs(2));
        (admin, pool, [baba, disabled, dummy, item, ability])
    });
    let [baba, disabled, dummy, item, ability] = ids;
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let query = query();
    let context = context();
    let execute = |name: ToolName, arguments: Value| {
        tools
            .execute(
                &query,
                &context,
                None,
                "playable",
                &request(name, arguments),
            )
            .unwrap()
            .result
    };
    let found = execute(ToolName::FindEntity, json!({"text":"Baba","kind":"hero"}));
    assert_eq!(found.result["resolved"], true);
    assert_eq!(found.result["entities"][0]["entity_id"], baba);
    let profile = execute(ToolName::GetHero, json!({"hero":baba}));
    assert!(!profile.is_error);
    assert_eq!(profile.result["identity"]["name_en"], "Baba");
    for (text, hidden) in [("Verborgen", disabled), ("Zielpuppe", dummy)] {
        for arguments in [
            json!({"text":text}),
            json!({"text":text,"kind":"hero"}),
            json!({"text":format!("{text}x")}),
        ] {
            let missing = execute(ToolName::FindEntity, arguments);
            assert!(missing.is_error, "{text}");
            assert_eq!(missing.result["error"], "entity_not_found", "{text}");
        }
        let profile = execute(ToolName::GetHero, json!({"hero":hidden}));
        assert!(profile.is_error, "{text}");
        assert!(profile.evidence_ids.is_empty(), "{text}");
    }
    for (text, kind, expected) in [("Klinge", "item", item), ("Funke", "ability", ability)] {
        let found = execute(ToolName::FindEntity, json!({"text":text,"kind":kind}));
        assert_eq!(found.result["resolved"], true, "{text}");
        assert_eq!(found.result["entities"][0]["entity_id"], expected, "{text}");
    }
    runtime.block_on(async {
        let playable: Vec<bool> = sqlx::query_scalar("SELECT serve.hero_is_playable(id) FROM unnest($1::bigint[]) WITH ORDINALITY ids(id,position) ORDER BY position")
            .bind(ids.to_vec()).fetch_all(&pool).await.unwrap();
        assert_eq!(playable, [true, false, false, false, false]);
        let compared: Value = sqlx::query_scalar(include_str!("pg_asset_compare.sql"))
            .bind(vec![baba, disabled, dummy, item])
            .bind(Vec::<String>::new())
            .fetch_one(&pool)
            .await
            .unwrap();
        let shown: Vec<i64> = compared["entities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|identity| identity["entity_id"].as_i64().unwrap())
            .collect();
        assert_eq!(shown, [baba, item]);
        pool.close().await;
        admin.close().await;
    });
}

const AUDIT_FILLER: &str = include_str!("../tests/fixtures/audit_filler.sql");

fn patch_sql(name: &str) -> String {
    include_str!("pg_patch_tools.rs")
        .split_once(&format!("const {name}: &str = \""))
        .unwrap()
        .1
        .split_once("\";")
        .unwrap()
        .0
        .replace("\\\"", "\"")
}

async fn alias_receipt(admin: &PgPool, key: &str) -> i64 {
    sqlx::query_scalar("SELECT a.receipt_id FROM canon.alias a JOIN canon.entity e ON e.id=a.entity_id WHERE e.key=$1 ORDER BY a.receipt_id DESC LIMIT 1")
        .bind(key).fetch_one(admin).await.unwrap()
}

async fn audit(admin: &PgPool, receipt: i64, rejected: bool) {
    let run: i64 = sqlx::query_scalar("INSERT INTO ops.receipt_audit_run(sampled,rejected_receipts,rejected_objects) VALUES(1,1,1) RETURNING id")
        .fetch_one(admin).await.unwrap();
    let statement = if rejected {
        "INSERT INTO ops.receipt_audit_rejection(run_id,raw_object_id,code,detail) SELECT $1,raw_object_id,'hash_mismatch','test' FROM canon.receipt WHERE id=$2"
    } else {
        "INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome) VALUES($1,$2,'locator_not_found')"
    };
    sqlx::query(statement)
        .bind(run)
        .bind(receipt)
        .execute(admin)
        .await
        .unwrap();
}

type Explain<'q> = sqlx::query::QueryScalar<'q, sqlx::Postgres, Value, sqlx::postgres::PgArguments>;

async fn explained_ms(
    pool: &PgPool,
    sql: &str,
    bind: impl FnOnce(Explain<'_>) -> Explain<'_>,
) -> f64 {
    let mut connection = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *connection)
        .await
        .unwrap();
    assert_session(&mut connection).await.unwrap();
    let explain = format!("EXPLAIN (ANALYZE,FORMAT JSON) {sql}");
    let plan = bind(sqlx::query_scalar(&explain))
        .fetch_one(&mut *connection)
        .await
        .unwrap();
    connection.commit().await.unwrap();
    plan[0]["Planning Time"].as_f64().unwrap() + plan[0]["Execution Time"].as_f64().unwrap()
}

async fn lookup_entities(pool: &PgPool, sql: &str, alias: &str) -> Vec<(i64, String)> {
    let mut connection = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *connection)
        .await
        .unwrap();
    let rows = sqlx::query(sql)
        .bind(alias)
        .bind(Some("hero"))
        .bind(6_i64)
        .fetch_all(&mut *connection)
        .await
        .unwrap();
    connection.commit().await.unwrap();
    rows.iter()
        .map(|row| (row.get("entity_id"), row.get("alias_norm")))
        .collect()
}

async fn hero(admin: &PgPool, alias: &str, key: &str, revision: &str) -> i64 {
    let entity = ingest_fixture(admin, alias, key, revision, vec![json!(780)], true).await;
    playable_hero(admin, key).await;
    entity
}

#[test]
fn audited_publication_keeps_gamefacts_lookups_within_statement_budget() {
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
    runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2)
            .connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
            .execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        let visible = hero(&admin, "auditsichtbar", "hero_audit_visible", "r1").await;
        let rejected = hero(&admin, "auditabgelehnt", "hero_audit_rejected", "r1").await;
        let audited = hero(&admin, "auditgeprueft", "hero_audit_audited", "r1").await;
        let superseded = hero(&admin, "auditalt", "hero_audit_superseded", "r1").await;
        hero(&admin, "auditneu", "hero_audit_superseded", "r2").await;
        let shadowed = hero(&admin, "auditschatten", "hero_audit_shadowed", "r1").await;
        hero(&admin, "auditschatten", "hero_audit_shadowed", "r2").await;
        let mut roster = Vec::new();
        for n in 0..67 {
            roster.extend([hero(&admin, &format!("kaderheld {n:02}"), &format!("hero_roster_{n:02}"), "r1").await]);
        }
        timers::publish_timer(&admin, "timer-r1", false).await;
        sqlx::raw_sql(AUDIT_FILLER).execute(&admin).await.unwrap();
        audit(&admin, alias_receipt(&admin, "hero_audit_rejected").await, true).await;
        audit(&admin, alias_receipt(&admin, "hero_audit_audited").await, false).await;
        audit(&admin, alias_receipt(&admin, "hero_audit_shadowed").await, false).await;
        let boundary: (i64, i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM ops.receipt_audit),(SELECT count(*) FROM ops.receipt_audit_rejection),(SELECT count(*) FROM canon.receipt_quarantine WHERE cause='audited'),(SELECT count(*) FROM canon.receipt_quarantine WHERE cause='shadowed')")
            .fetch_one(&admin).await.unwrap();
        assert!(boundary.0 >= 5000 && boundary.1 >= 1000 && boundary.2 >= 1000 && boundary.3 >= 1, "{boundary:?}");
        sqlx::raw_sql("ANALYZE").execute(&admin).await.unwrap();
        let pool = tool_pool(options.username("brain_readonly"), Duration::from_secs(2));
        for sql in [EXACT, FUZZY] {
            assert!(lookup_entities(&pool, sql, "auditsichtbar").await.contains(&(visible, "auditsichtbar".into())));
            assert!(lookup_entities(&pool, sql, "auditneu").await.contains(&(superseded, "auditneu".into())));
            for (alias, hidden) in [("auditabgelehnt", rejected), ("auditgeprueft", audited), ("auditalt", superseded), ("auditschatten", shadowed)] {
                assert!(!lookup_entities(&pool, sql, alias).await.contains(&(hidden, alias.into())), "{alias}");
            }
        }
        let timer_sql = include_str!("timer_query.sql").trim().trim_end_matches(';').to_owned();
        let timer: Vec<Value> = sqlx::query_scalar(&timer_sql).bind("rejuvenator").fetch_all(&pool).await.unwrap();
        assert_eq!(timer.len(), 2);
        let current = patch_sql("CURRENT");
        let resolve = patch_sql("RESOLVE");
        let changes = patch_sql("CHANGES");
        let timings = [
            ("exact", explained_ms(&pool, EXACT, |query| query.bind("auditsichtbar").bind(None::<String>).bind(6_i64)).await),
            ("fuzzy", explained_ms(&pool, FUZZY, |query| query.bind("auditsichtbar").bind(None::<String>).bind(6_i64)).await),
            ("timer", explained_ms(&pool, &timer_sql, |query| query.bind("rejuvenator")).await),
            ("patch_current", explained_ms(&pool, &current, |query| query).await),
            ("patch_resolve", explained_ms(&pool, &resolve, |query| query.bind("2026-01-01")).await),
            ("patch_changes", explained_ms(&pool, &changes, |query| query.bind(visible).bind(None::<i64>).bind(None::<String>).bind(None::<String>)).await),
            ("fuzzy_roster", explained_ms(&pool, FUZZY, |query| query.bind("kaderheld").bind(Some("hero")).bind(6_i64)).await),
            ("hero_roster", explained_ms(&pool, hero_sql(&GetHeroRequest { hero: roster[10] as u64, patch: None, sections: None }), |query| query.bind(roster[10])).await),
        ];
        eprintln!("GAMEFACTS_AUDITED_PUBLICATION_PROOF audit_rows={} rejections={} audited={} shadowed={} planning_plus_execution_ms={timings:?}", boundary.0, boundary.1, boundary.2, boundary.3);
        for (name, elapsed) in timings {
            assert!(elapsed < 500.0, "{name} planning_plus_execution_ms={elapsed}");
        }
        pool.close().await;
        admin.close().await;
    });
}

#[path = "meta_stats_pg_tests.rs"]
mod meta_stats;
