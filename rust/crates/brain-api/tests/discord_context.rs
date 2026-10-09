use brain_api::{router, ApiService};
use brain_client::AsyncBrainClient;
use brain_contracts::{
    discord_task::{
        DiscordAnswerCapability, DiscordAnswerTask, DiscordContextProjection,
        DiscordContextResolver, PUBLIC_CONTEXT_PREFIX,
    },
    AnswerProfile, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort, Principal, Query,
    SourceRecordV2, SourceVisibility,
};
use brain_kernel::{AnswerKernelPort, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_providers::{CodexSubscriptionProvider, ProviderConfig};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    time::Duration,
};

fn record(id: &str, text: &str, private: bool) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "docs-public".into(),
        logical_id: id.into(),
        revision: 1,
        content_hash: format!("{:x}", Sha256::digest(text.as_bytes())),
        content: text.into(),
        visibility: if private {
            SourceVisibility::Private
        } else {
            SourceVisibility::Public
        },
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}

async fn store() -> MemoryRepository {
    let store = MemoryRepository::default();
    for record in [
        record(
            "paten",
            "Paten helfen beim Einstieg. Für Paten meldest du dich beim Concierge.",
            false,
        ),
        record(
            "coaching",
            "Coaching hilft beim Spielen. Für Coaching meldest du dich im Coaching-Bereich.",
            false,
        ),
        record(
            "voice",
            "Sprachkanäle werden über den Sprachkanal-Button geöffnet.",
            false,
        ),
        record(
            "seelen",
            "Mehr Seelen bekommst du durch das Besiegen von Troopern und das Sichern ihrer Seelen.",
            false,
        ),
        record("private", "Rollen PRIVATE_SOURCE_CANARY", true),
    ] {
        store.apply_record(record).unwrap();
    }
    let mut hero = record(
        "asset/hero/1/summary",
        "Hero: Abrams\nAbrams kann Spirit-Builds mit öffentlichen Items spielen.",
        false,
    );
    hero.source_id = "deadlock-assets-heroes".into();
    hero.metadata = BTreeMap::from([
        ("name".into(), "Abrams".into()),
        ("kind".into(), "fact".into()),
    ]);
    store.apply_record(hero).unwrap();
    let release = store
        .release_from_heads("context-release", "v1", "p1")
        .unwrap();
    store.publish(&release).await.unwrap();
    store
}

fn query(id: &str, text: &str) -> Query {
    Query {
        request_id: id.into(),
        conversation_id: id.into(),
        text: text.into(),
        answer_context: None,
        domain: None,
        requested_scopes: BTreeSet::from(["bot.public".into()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}

fn context(q: &Query) -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "bot".into(),
            channel: "discord".into(),
            scopes: q.requested_scopes.clone(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: q.conversation_id.clone(),
        knowledge_release: "context-release".into(),
        deadline_ms: 2000,
        request_deadline: None,
        budget: Budget::default(),
        discord: None,
    }
}

fn projection(turns: &[&[&str]]) -> DiscordContextProjection {
    DiscordContextProjection {
        turns: turns
            .iter()
            .map(|turn| turn.iter().map(|subject| (*subject).into()).collect())
            .collect(),
    }
}

fn api<K: AnswerKernelPort>(kernel: K) -> ApiService<K> {
    ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture",
            "bot",
            "discord",
            BTreeSet::from(["bot.public".into()]),
            BTreeSet::from(["public".into()]),
        )])),
        kernel,
        "context-release",
        5000,
        Budget::default(),
    )
    .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]))
}

fn read_request(stream: &mut std::net::TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; 4096];
        let count = stream.read(&mut buffer).unwrap();
        assert!(count > 0);
        bytes.extend_from_slice(&buffer[..count]);
        if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&bytes[..end]);
            let length: usize = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .map(|length| length.trim().parse().unwrap())
                })
                .unwrap();
            if bytes.len() >= end + 4 + length {
                return String::from_utf8(bytes).unwrap();
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn all_routes_always_send_ordered_public_projection_before_real_provider_transport() {
    let maximum_question = format!("Coaching?{}", "ä".repeat(3991));
    let cases = vec![
        (
            "Wie mache ich das? <@76561197960265839>",
            vec!["Wie finde ich einen Paten? PRIVATE_CANARY_äöüß 123456789012345678"],
            json!([["Paten"]]),
        ),
        (
            "Wie kann ich das verbessern?",
            vec!["Paten?", "Wie mache ich das?", "Warum?", "Erzähl mir mehr"],
            json!([["Paten"], [], [], []]),
        ),
        (
            "Wie kann ich sie spielen?",
            vec!["Welche Items passen zu Abrams? PRIVATE_CANARY_äöüß"],
            json!([["Abrams"]]),
        ),
        (
            "Abrams",
            vec![
                "Welche Items passen zu Abrams? PRIVATE_CANARY_äöüß",
                "Okay ja ne Idee für ein Spirit build",
            ],
            json!([["Abrams"], ["Build", "Spirit-Build"]]),
        ),
        (
            "Warum?",
            vec![
                "Welche Items passen zu Abrams? PRIVATE_CANARY_äöüß",
                "Okay ja ne Idee für ein Spirit build",
            ],
            json!([["Abrams"], ["Build", "Spirit-Build"]]),
        ),
        (
            "Wie bekomme ich mehr Seelen?",
            vec![
                "Welche Items passen zu Abrams?",
                "Okay ja ne Idee für ein Spirit build",
            ],
            json!([["Abrams"], ["Build", "Spirit-Build"]]),
        ),
        (
            "Kann ich das mit Coaching kombinieren?",
            vec!["Wie finde ich einen Paten? PRIVATE_CANARY_äöüß"],
            json!([["Paten"]]),
        ),
        (
            "Warum?",
            vec!["Paten?", "Kann ich das mit Coaching kombinieren?"],
            json!([["Paten"], ["Coaching"]]),
        ),
        ("Gibt es auch Coaching?", vec![], json!([])),
        (
            "Gibt es auch Coaching?",
            vec!["Paten?", "PRIVATE_CANARY_äöüß Fremdnutzer"],
            json!([["Paten"], []]),
        ),
        (
            "Kann ich es auch mit Coaching kombinieren?",
            vec!["Paten?"],
            json!([["Paten"]]),
        ),
        (
            "Warum?",
            vec![
                "Paten?",
                "Wie bekomme ich Rollen? PRIVATE_CANARY_äöüß",
                "Warum?",
            ],
            json!([["Paten"], [], []]),
        ),
        (
            "Gibt es gleichzeitig Coaching und Paten?",
            vec!["Wie öffne ich Sprachkanäle?"],
            json!([["Sprachkanäle"]]),
        ),
        ("Welche Items passen zu Abrams?", vec![], json!([])),
        (
            maximum_question.as_str(),
            vec!["Paten?"],
            json!([["Paten"]]),
        ),
    ];
    let expected_count = cases.len() * 3;
    let expected: Vec<_> = (0..3)
        .flat_map(|_| {
            cases.iter().map(|(text, _, turns)| {
                (
                    brain_contracts::discord_task::without_platform_ids(text),
                    turns.clone(),
                )
            })
        })
        .collect();
    let store = store().await;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let recorded = requests.clone();
    let provider_server = std::thread::spawn(move || {
        for (question, turns) in expected {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            assert!(request.starts_with("POST /v1/messages "));
            let data: Value =
                serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
            let model_input: Value =
                serde_json::from_str(data["messages"][0]["content"].as_str().unwrap()).unwrap();
            assert_eq!(model_input["query"], question);
            assert_eq!(model_input["public_conversation_context"]["turns"], turns);
            assert_eq!(
                model_input["public_conversation_context"]["private_content_omitted"],
                true
            );
            for forbidden in [
                "PRIVATE_CANARY_äöüß",
                "76561197960265839",
                "123456789012345678",
                "PRIVATE_SOURCE_CANARY",
                "user_questions",
                "assistant",
                "Fremdnutzer",
            ] {
                assert!(
                    !request.contains(forbidden),
                    "{forbidden} in Providerrequest"
                );
            }
            let evidence = model_input["evidence"].as_array().unwrap();
            if question == "Kann ich das mit Coaching kombinieren?" {
                for subject in ["Paten", "Coaching"] {
                    assert!(evidence
                        .iter()
                        .any(|item| item["content"].as_str().unwrap().contains(subject)));
                }
            }
            let ids: Vec<Value> = evidence.iter().map(|item| item["id"].clone()).collect();
            assert!(!ids.is_empty());
            let answer =
                json!({"text":"Hier ist die Antwort auf deine Frage.", "cited_evidence_ids":ids})
                    .to_string();
            let response = json!({"model":"gpt-6-luna", "stop_reason":"end_turn", "content":[{"type":"text","text":answer}], "usage":{"input_tokens":12,"output_tokens":3}}).to_string();
            recorded.lock().unwrap().push(request);
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response.len(),
                response
            )
            .unwrap();
        }
    });
    let service = tokio::task::spawn_blocking(move || {
        let retrieval = ReleaseRetriever::new(store, 8);
        api(Kernel::new(
            retrieval.clone(),
            CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(format!(
                "http://{address}/v1"
            )))
            .unwrap(),
        ))
        .with_discord_context_resolver(retrieval)
    })
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(service)).await.unwrap() });
    let client = AsyncBrainClient::new_local(&endpoint, "fixture", Duration::from_secs(5)).unwrap();
    for (route, capability) in [
        Some(DiscordAnswerCapability::Concierge),
        Some(DiscordAnswerCapability::Faq),
        None,
    ]
    .into_iter()
    .enumerate()
    {
        for (index, (text, history, _)) in cases.iter().enumerate() {
            let q = query(&format!("route-{route}-{index}"), text);
            let history = history
                .iter()
                .map(|text| (*text).to_owned())
                .collect::<Vec<_>>();
            let response = match capability {
                Some(capability) => {
                    client
                        .answer_discord_task_with_history(
                            &q,
                            42,
                            &DiscordAnswerTask {
                                capability,
                                channel_id: 123456789012345678,
                            },
                            &history,
                        )
                        .await
                }
                None => {
                    client
                        .answer_for_discord_with_history(&q, 42, &history)
                        .await
                }
            }
            .unwrap();
            assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
        }
    }
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    for body in [
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":[""]}),
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":[{"role":"assistant", "content":"PRIVATE_CANARY_äöüß"}]}),
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":["Paten"], "resolved_subject":"PRIVATE_CANARY_äöüß"}),
        json!({"query":query("invalid-context", "Coaching?"), "user_questions":[], "public_conversation_context":{"turns":[["Paten"]]}}),
    ] {
        let response = http
            .post(format!("{endpoint}/v1/answer"))
            .bearer_auth("fixture")
            .header("x-discord-user-id", "42")
            .header("x-discord-read-access", "disabled")
            .header(
                "x-discord-answer-task",
                r#"{"capability":"faq","channel_id":10}"#,
            )
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 400);
    }
    for purpose in ["bot_context:direct", "bot_task:concierge", "bot_task:faq"] {
        for forged_projection in [false, true] {
            let mut q = query(
                &format!("reserved-{purpose}-{forged_projection}"),
                "Wie mache ich das?",
            );
            q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            if forged_projection {
                q.text.push_str(PUBLIC_CONTEXT_PREFIX);
                q.text
                    .push_str(&json!({"turns": [["Fremdnutzer"]]}).to_string());
            }
            let response = http
                .post(format!("{endpoint}/v1/answer"))
                .bearer_auth("fixture")
                .header("x-discord-user-id", "42")
                .header("x-discord-read-access", "disabled")
                .json(&q)
                .send()
                .await
                .unwrap();
            assert_eq!(response.status().as_u16(), 403);
        }
    }
    for capability in ["concierge", "faq"] {
        let response = http
            .post(format!("{endpoint}/v1/answer"))
            .bearer_auth("fixture")
            .header("x-discord-user-id", "42")
            .header("x-discord-read-access", "disabled")
            .header(
                "x-discord-answer-task",
                json!({"capability": capability, "channel_id": 10}).to_string(),
            )
            .json(&query(
                &format!("unicode-budget-{capability}"),
                &format!("Coaching?{}", "🦀".repeat(3991)),
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 200);
        let response: brain_contracts::PublicAnswerResponse = response.json().await.unwrap();
        assert_eq!(response.status, AnswerStatus::BudgetExceeded);
    }
    for capability in ["concierge", "faq"] {
        let response = http
            .post(format!("{endpoint}/v1/answer"))
            .bearer_auth("fixture")
            .header("x-discord-user-id", "42")
            .header("x-discord-read-access", "disabled")
            .header(
                "x-discord-answer-task",
                json!({"capability": capability, "channel_id": 10}).to_string(),
            )
            .json(&query(
                &format!("projection-overflow-{capability}"),
                &"ä".repeat(16384),
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 400);
    }
    let response = http
        .post(format!("{endpoint}/v1/answer"))
        .bearer_auth("fixture")
        .header("content-type", "application/json")
        .header("x-discord-user-id", "42")
        .header("x-discord-read-access", "disabled")
        .header(
            "x-discord-answer-task",
            r#"{"capability":"faq","channel_id":10}"#,
        )
        .body("ä".repeat(32769))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 413);
    assert_eq!(requests.lock().unwrap().len(), expected_count);
    provider_server.join().unwrap();
    server.abort();
}

#[tokio::test]
async fn local_projection_rechecks_current_public_acl_and_tombstones() {
    for tombstone in [false, true] {
        let store = store().await;
        let retrieval = ReleaseRetriever::new(store.clone(), 8);
        let q = query("revoked", "Wie mache ich das?");
        let history = ["Coaching?", "Gibt es auch Paten? PRIVATE_CANARY_äöüß"].map(str::to_owned);
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            projection(&[&["Coaching"], &["Paten"]])
        );
        let mut revoked = record(
            "paten",
            "Paten helfen beim Einstieg. Für Paten meldest du dich beim Concierge.",
            !tombstone,
        );
        revoked.revision = 2;
        revoked.tombstone = tombstone;
        store.apply_record(revoked).unwrap();
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            projection(&[&["Coaching"], &[]])
        );
    }
}

#[tokio::test]
async fn local_projection_reuses_public_hero_names_without_numeric_identifiers() {
    let store = store().await;
    let mut hero = record("asset/hero/25/health", "Hero: Warden\nhealth: 500", false);
    hero.source_id = "deadlock-assets-heroes".into();
    hero.metadata = BTreeMap::from([
        ("kind".into(), "fact".into()),
        ("name".into(), "Warden".into()),
        ("aliases_de".into(), "Wächter".into()),
    ]);
    store.apply_record(hero).unwrap();
    let release = store
        .release_from_heads("game-release", "v1", "p1")
        .unwrap();
    store.publish(&release).await.unwrap();
    let retrieval = ReleaseRetriever::new(store, 8);
    for text in [
        "Und seine Ult?",
        "Wie kann ich sie spielen?",
        "Gibt es auch Coaching?",
    ] {
        let q = query("hero-followup", text);
        let mut authorized = context(&q);
        authorized.knowledge_release = "game-release".into();
        let history = [
            "Was macht der Wächter? PRIVATE_CANARY_äöüß",
            "Wie spielt er?",
            "Warum?",
        ]
        .map(str::to_owned);
        assert_eq!(
            retrieval.resolve(&q, &authorized, &history).unwrap(),
            projection(&[&["Warden"], &[], &[]])
        );
        assert_eq!(
            retrieval
                .resolve(&q, &authorized, &["Was kostet 25?".into()])
                .unwrap(),
            projection(&[&[]])
        );
    }
}

#[tokio::test]
async fn direct_build_history_is_projected_without_classifying_the_current_question() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    let history = [
        "Welche Items passen zu Abrams?",
        "Okay ja ne Idee für ein Spirit build",
    ]
    .map(str::to_owned);
    for text in [
        "Abrams",
        "Warum?",
        "Wie bekomme ich mehr Seelen?",
        "Gibt es mehr Seelen im Dschungel?",
        "Kann ich mehr Seelen farmen?",
        "Und seine Ult?",
        "Okay ja ne Idee für ein Spirit build",
        "Unbekannter Held",
        "Wie kann ich das verbessern mit Privatproblem?",
    ] {
        let q = query("direct-after-build", text);
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            projection(&[&["Abrams"], &["Build", "Spirit-Build"]]),
            "{text}"
        );
        for barrier in ["Und wie bekomme ich Rollen?", "Unbekannter Held"] {
            assert_eq!(
                retrieval
                    .resolve(
                        &q,
                        &context(&q),
                        &[history[0].clone(), barrier.into(), history[1].clone()]
                    )
                    .unwrap(),
                projection(&[&["Abrams"], &[], &["Build", "Spirit-Build"]])
            );
        }
    }
}

#[tokio::test]
async fn direct_build_projection_rechecks_public_hero_and_omits_private_content() {
    let store = store().await;
    let retrieval = ReleaseRetriever::new(store.clone(), 8);
    let q = query("direct-hero", "Abrams");
    let history = [
        "Welche Items passen zu Abrams?",
        "Okay ja ne Idee für ein Spirit build",
    ]
    .map(str::to_owned);
    assert_eq!(
        retrieval.resolve(&q, &context(&q), &history).unwrap(),
        projection(&[&["Abrams"], &["Build", "Spirit-Build"]])
    );
    assert_eq!(
        retrieval
            .resolve(
                &q,
                &context(&q),
                &["Spirit build PRIVATE_CANARY_äöüß".into()]
            )
            .unwrap(),
        projection(&[&["Build", "Spirit-Build"]])
    );
    for tombstone in [false, true] {
        let mut revoked = record(
            "asset/hero/1/summary",
            "Hero: Abrams\nAbrams kann Spirit-Builds mit öffentlichen Items spielen.",
            !tombstone,
        );
        revoked.source_id = "deadlock-assets-heroes".into();
        revoked.revision = if tombstone { 3 } else { 2 };
        revoked.tombstone = tombstone;
        revoked.metadata = BTreeMap::from([
            ("name".into(), "Abrams".into()),
            ("kind".into(), "fact".into()),
        ]);
        store.apply_record(revoked).unwrap();
        for text in [
            "Abrams",
            "Warum?",
            "Und seine Ult?",
            "Wie bekomme ich mehr Seelen?",
        ] {
            let q = query("revoked-build", text);
            assert_eq!(
                retrieval.resolve(&q, &context(&q), &history).unwrap(),
                projection(&[&[], &[]]),
                "{text}"
            );
        }
    }
}

#[tokio::test]
async fn private_history_transport_is_loopback_only_and_bounds_unicode() {
    let task = DiscordAnswerTask {
        capability: DiscordAnswerCapability::Faq,
        channel_id: 10,
    };
    for endpoint in [
        "https://example.invalid",
        "https://localhost.example.invalid",
        "https://127.0.0.1.example.invalid",
    ] {
        let client = AsyncBrainClient::new(endpoint, "fixture", Duration::from_secs(1)).unwrap();
        assert!(matches!(
            client
                .answer_discord_task_with_history(
                    &query("nonlocal", "Frage"),
                    42,
                    &task,
                    &["Privat".into()]
                )
                .await,
            Err(brain_client::ClientError::InvalidBaseUrl)
        ));
        assert!(matches!(
            client
                .answer_for_discord_with_history(
                    &query("nonlocal-direct", "Frage"),
                    42,
                    &["Privat".into()]
                )
                .await,
            Err(brain_client::ClientError::InvalidBaseUrl)
        ));
    }
    assert!(matches!(
        AsyncBrainClient::new(
            "http://127.0.0.1.example.invalid",
            "fixture",
            Duration::from_secs(1)
        ),
        Err(brain_client::ClientError::InvalidBaseUrl)
    ));
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client = AsyncBrainClient::new_local(
        &format!("http://{}", listener.local_addr().unwrap()),
        "fixture",
        Duration::from_secs(1),
    )
    .unwrap();
    for history in [
        vec!["äöüß".repeat(1001)],
        vec!["x".into(); 5],
        vec![String::new()],
    ] {
        assert!(client
            .answer_discord_task_with_history(&query("invalid", "Frage"), 42, &task, &history)
            .await
            .is_err());
        assert!(client
            .answer_for_discord_with_history(&query("invalid-direct", "Frage"), 42, &history)
            .await
            .is_err());
    }
    listener.set_nonblocking(true).unwrap();
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

struct NeverKernel;

impl AnswerKernelPort for NeverKernel {
    fn answer(&self, _: &Query, _: &AuthorizedContext) -> brain_contracts::AnswerResponse {
        panic!("Ungeprüfte Identität darf den Kernel nicht erreichen")
    }
    fn answer_for_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        self.answer(query, context)
    }
}

struct RecordingKernel(Arc<Mutex<Vec<Query>>>);

impl AnswerKernelPort for RecordingKernel {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        assert!(context.discord.is_none());
        assert_eq!(
            context.principal.scopes,
            BTreeSet::from(["bot.public".into()])
        );
        assert_eq!(
            context.principal.provider_egress,
            BTreeSet::from(["public".into()])
        );
        self.0.lock().unwrap().push(query.clone());
        brain_contracts::AnswerResponse {
            contract_version: brain_contracts::CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: AnswerStatus::InsufficientEvidence,
            text: "Diese Frage erreicht den Kernel.".into(),
            citations: Vec::new(),
            usage: Default::default(),
        }
    }
    fn answer_for_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        self.answer(query, context)
    }
}

#[tokio::test]
async fn newer_unsupported_topics_preserve_gaps_and_reach_all_routes_without_preclassification() {
    let store = store().await;
    let mut revoked = record(
        "coaching",
        "Coaching hilft beim Spielen. Für Coaching meldest du dich im Coaching-Bereich.",
        true,
    );
    revoked.revision = 2;
    store.apply_record(revoked).unwrap();
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let service = api(RecordingKernel(recorded.clone()))
        .with_discord_context_resolver(ReleaseRetriever::new(store, 8));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(service)).await.unwrap() });
    let client = AsyncBrainClient::new_local(&endpoint, "fixture", Duration::from_secs(5)).unwrap();
    let cases = [
        (
            "Wie mache ich das?",
            vec!["Paten?", "Und wie bekomme ich Rollen?"],
            json!([["Paten"], []]),
        ),
        (
            "Wie mache ich das?",
            vec!["Paten?", "Und wie finde ich PRIVATE_CANARY_äöüß?"],
            json!([["Paten"], []]),
        ),
        (
            "Wie mache ich das?",
            vec!["Paten?", "Und wie finde ich Coaching?"],
            json!([["Paten"], []]),
        ),
        (
            "Wie kann ich das verbessern?",
            vec!["Paten?", "Und wie bekomme ich Rollen?", "Warum?"],
            json!([["Paten"], [], []]),
        ),
        (
            "Wie kann ich sie spielen?",
            vec!["Paten?", "Und wie spiele ich Unbekannt?"],
            json!([["Paten"], []]),
        ),
        (
            "Und wie bekomme ich Rollen?",
            vec!["Paten?"],
            json!([["Paten"]]),
        ),
        (
            "Wie mache ich das?",
            vec!["Paten?", "Und wie bekomme ich Ult?"],
            json!([["Paten"], []]),
        ),
        (
            "Wie mache ich das?",
            vec!["Paten?", "Und wie finde ich <#123456789012345678>?"],
            json!([["Paten"], []]),
        ),
        (
            "Wie kann ich das verbessern mit Privatproblem?",
            vec!["Paten?"],
            json!([["Paten"]]),
        ),
        (
            "Kann ich das mit Paten kombinieren?",
            vec!["Sprachkanäle?", "Coaching?"],
            json!([["Sprachkanäle"], []]),
        ),
        (
            "Kann ich das mit Paten kombinieren?",
            vec!["Sprachkanäle?", "PRIVATE_CANARY_äöüß", "Warum?"],
            json!([["Sprachkanäle"], [], []]),
        ),
        (
            "Warum?",
            vec![
                "Sprachkanäle?",
                "Coaching?",
                "Kann ich das mit Paten kombinieren?",
            ],
            json!([["Sprachkanäle"], [], ["Paten"]]),
        ),
        ("Gibt es auch Coaching?", vec![], json!([])),
        ("Wie kann ich sie spielen?", vec![], json!([])),
        ("Gibt es auch dafür Coaching?", vec![], json!([])),
    ];
    for capability in [
        Some(DiscordAnswerCapability::Faq),
        Some(DiscordAnswerCapability::Concierge),
        None,
    ] {
        for (current, history, turns) in &cases {
            let history = history
                .iter()
                .map(|text| (*text).to_owned())
                .collect::<Vec<_>>();
            let q = query("unsupported-topic", current);
            let response = match capability {
                Some(capability) => {
                    client
                        .answer_discord_task_with_history(
                            &q,
                            42,
                            &DiscordAnswerTask {
                                capability,
                                channel_id: 10,
                            },
                            &history,
                        )
                        .await
                }
                None => {
                    client
                        .answer_for_discord_with_history(&q, 42, &history)
                        .await
                }
            }
            .unwrap();
            assert_eq!(response.status, AnswerStatus::InsufficientEvidence);
            let received = recorded.lock().unwrap().last().unwrap().clone();
            let model_input: Value = serde_json::from_str(
                &brain_contracts::provider_input::grounded_messages(&received, &[])[1].content,
            )
            .unwrap();
            assert_eq!(model_input["query"], *current);
            assert_eq!(model_input["public_conversation_context"]["turns"], *turns);
            assert!(!received.text.contains("PRIVATE_CANARY_äöüß"));
        }
    }
    assert_eq!(recorded.lock().unwrap().len(), cases.len() * 3);
    server.abort();
}

#[tokio::test]
async fn current_public_topic_never_changes_the_history_projection() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    for text in [
        "Und wie bekomme ich Paten?",
        "Und wie bekomme ich Rollen?",
        "Und wie finde ich Unbekannt?",
        "Gibt es auch Coaching?",
    ] {
        let q = query("current-topic", text);
        assert_eq!(
            retrieval
                .resolve(&q, &context(&q), &["Paten?".into()])
                .unwrap(),
            projection(&[&["Paten"]])
        );
    }
}

#[tokio::test]
async fn mixed_public_topics_are_projected_without_selecting_a_reference() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    let history = [
        "Wie finde ich einen Paten?",
        "Kann ich das mit Coaching kombinieren?",
    ]
    .map(str::to_owned);
    for purpose in ["bot_context:direct", "bot_task:concierge", "bot_task:faq"] {
        for text in [
            "Kann ich das mit Coaching kombinieren?",
            "Kann ich es mit Coaching verbinden?",
            "Wie kann ich sie mit Coaching vergleichen?",
            "Was ist das für ein Coaching?",
            "Kann ich das statt Coaching nutzen?",
            "Kann ich das mit Coaching und Privatproblem kombinieren?",
            "Warum?",
        ] {
            let mut q = query("mixed-reference", text);
            q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            assert_eq!(
                retrieval.resolve(&q, &context(&q), &history).unwrap(),
                projection(&[&["Paten"], &["Coaching"]])
            );
            assert_eq!(
                retrieval
                    .resolve(&q, &context(&q), &["Paten oder Coaching?".into()])
                    .unwrap(),
                projection(&[&["Coaching", "Paten"]])
            );
        }
    }
}

#[tokio::test]
async fn existential_questions_and_real_references_use_the_same_ordered_projection() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    for text in [
        "Gibt es auch Coaching?",
        "Gibt es zusätzlich Coaching?",
        "Wo gibt es auch Coaching?",
        "Gibt es auch noch Coaching?",
        "Gibt es für Anfänger auch Coaching?",
        "Kann ich es auch mit Coaching kombinieren?",
        "Gibt es auch dafür Coaching?",
        "Gibt es das auch?",
        "Gibt es auch?",
    ] {
        let q = query("existential-current", text);
        for (history, expected) in [
            (vec![], projection(&[])),
            (vec!["Paten?"], projection(&[&["Paten"]])),
            (
                vec!["Paten?", "PRIVATE_CANARY_äöüß"],
                projection(&[&["Paten"], &[]]),
            ),
            (
                vec!["Paten?", "Wie bekomme ich Rollen?"],
                projection(&[&["Paten"], &[]]),
            ),
        ] {
            assert_eq!(
                retrieval
                    .resolve(
                        &q,
                        &context(&q),
                        &history.into_iter().map(str::to_owned).collect::<Vec<_>>()
                    )
                    .unwrap(),
                expected,
                "{text}"
            );
        }
    }
}

#[tokio::test]
async fn mixed_history_keeps_unknown_private_and_revoked_turns_in_place() {
    for tombstone in [false, true] {
        let store = store().await;
        let retrieval = ReleaseRetriever::new(store.clone(), 8);
        let q = query("mixed-chain", "Warum?");
        for barrier in ["PRIVATE_CANARY_äöüß", "Wie bekomme ich Rollen?"] {
            let history =
                ["Paten?", barrier, "Kann ich das mit Coaching kombinieren?"].map(str::to_owned);
            assert_eq!(
                retrieval.resolve(&q, &context(&q), &history).unwrap(),
                projection(&[&["Paten"], &[], &["Coaching"]])
            );
        }
        let mut revoked = record(
            "paten",
            "Paten helfen beim Einstieg. Für Paten meldest du dich beim Concierge.",
            !tombstone,
        );
        revoked.revision = 2;
        revoked.tombstone = tombstone;
        store.apply_record(revoked).unwrap();
        assert_eq!(
            retrieval
                .resolve(
                    &q,
                    &context(&q),
                    &[
                        "Sprachkanäle?",
                        "Wie finde ich einen Paten?",
                        "Kann ich das mit Coaching kombinieren?"
                    ]
                    .map(str::to_owned)
                )
                .unwrap(),
            projection(&[&["Sprachkanäle"], &[], &["Coaching"]])
        );
    }
}

struct InvalidProjection(DiscordContextProjection);

impl DiscordContextResolver for InvalidProjection {
    fn resolve(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[String],
    ) -> Result<DiscordContextProjection, brain_contracts::PortError> {
        Ok(self.0.clone())
    }
}

#[tokio::test]
async fn invalid_projection_and_missing_local_release_do_not_block_the_current_question() {
    for candidate in [
        None,
        Some(String::new()),
        Some("<@42>".into()),
        Some("ä".repeat(81)),
    ] {
        let recorded = Arc::new(Mutex::new(Vec::new()));
        let service = api(RecordingKernel(recorded.clone()));
        let service = match candidate {
            Some(subject) => {
                service.with_discord_context_resolver(InvalidProjection(projection(&[&[&subject]])))
            }
            None => service.with_discord_context_resolver(ReleaseRetriever::new(
                MemoryRepository::default(),
                8,
            )),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server =
            tokio::spawn(async move { axum::serve(listener, router(service)).await.unwrap() });
        for capability in ["faq", "concierge"] {
            let response = reqwest::Client::builder().no_proxy().build().unwrap()
                .post(format!("{endpoint}/v1/answer")).bearer_auth("fixture")
                .header("x-discord-user-id", "42").header("x-discord-read-access", "disabled")
                .header("x-discord-answer-task", json!({"capability":capability, "channel_id":10}).to_string())
                .json(&json!({"query":query(capability, "Gibt es auch Coaching?"), "user_questions":["Paten PRIVATE_CANARY_äöüß"]})).send().await.unwrap();
            assert_eq!(response.status().as_u16(), 200);
        }
        let recorded = recorded.lock().unwrap();
        assert_eq!(recorded.len(), 2);
        for received in recorded.iter() {
            let (current, encoded) = received.text.rsplit_once(PUBLIC_CONTEXT_PREFIX).unwrap();
            assert_eq!(current, "Gibt es auch Coaching?");
            assert_eq!(
                serde_json::from_str::<DiscordContextProjection>(encoded).unwrap(),
                projection(&[&[]])
            );
        }
        server.abort();
    }
}

#[tokio::test]
async fn direct_history_requires_trusted_identity_and_server_owned_projection() {
    let service = ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![
            AuthGrant::from_secret(
                "fixture",
                "bot",
                "discord",
                BTreeSet::from(["bot.public".into()]),
                BTreeSet::from(["public".into()]),
            ),
            AuthGrant::from_secret(
                "outsider",
                "other",
                "web",
                BTreeSet::from(["bot.public".into()]),
                BTreeSet::from(["public".into()]),
            ),
        ])),
        NeverKernel,
        "context-release",
        5000,
        Budget::default(),
    )
    .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(service)).await.unwrap() });
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    for (token, identity, reads, injected) in [
        ("outsider", "42", "disabled", false),
        ("fixture", "0", "disabled", false),
        ("fixture", "42", "enabled", false),
        ("fixture", "42", "disabled", true),
    ] {
        let mut q = query("direct-forbidden", "Abrams");
        if injected {
            q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some("bot_context:direct".into()),
                    ..Default::default()
                },
            ));
        }
        let response = http
            .post(format!("{endpoint}/v1/answer"))
            .bearer_auth(token)
            .header("x-discord-user-id", identity)
            .header("x-discord-read-access", reads)
            .json(&json!({"query": q, "user_questions": ["PRIVATE_CANARY_äöüß"]}))
            .send()
            .await
            .unwrap();
        assert_eq!(
            response.status().as_u16(),
            if reads == "enabled" { 400 } else { 403 }
        );
        assert!(!response
            .text()
            .await
            .unwrap()
            .contains("PRIVATE_CANARY_äöüß"));
    }
    let response = http.post(format!("{endpoint}/v1/answer")).bearer_auth("fixture")
        .header("x-discord-read-access", "disabled")
        .json(&json!({"query": query("missing-identity", "Abrams"), "user_questions": ["PRIVATE_CANARY_äöüß"]})).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 403);
    let response = http.post(format!("{endpoint}/v1/answer")).bearer_auth("fixture")
        .header("x-discord-user-id", "42")
        .json(&json!({"query": query("missing-read-restriction", "Abrams"), "user_questions": ["PRIVATE_CANARY_äöüß"]})).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 403);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("x-discord-user-id", "42".parse().unwrap());
    headers.append("x-discord-user-id", "43".parse().unwrap());
    let response = http.post(format!("{endpoint}/v1/answer")).bearer_auth("fixture")
        .headers(headers).header("x-discord-read-access", "disabled")
        .json(&json!({"query": query("duplicate-identity", "Abrams"), "user_questions": ["PRIVATE_CANARY_äöüß"]})).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 400);
    server.abort();
}

#[tokio::test]
async fn private_history_never_follows_redirects() {
    let target = TcpListener::bind("127.0.0.1:0").unwrap();
    let location = format!("http://{}/v1/answer", target.local_addr().unwrap());
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let request = read_request(&mut stream);
        assert!(request.contains("PRIVATE_CANARY_äöüß"));
        assert!(request.contains("x-discord-read-access: disabled"));
        write!(stream, "HTTP/1.1 307 Temporary Redirect\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
    });
    let client = AsyncBrainClient::new_local(&endpoint, "fixture", Duration::from_secs(2)).unwrap();
    assert!(client
        .answer_discord_task_with_history(
            &query("redirect", "Frage"),
            42,
            &DiscordAnswerTask {
                capability: DiscordAnswerCapability::Concierge,
                channel_id: 10
            },
            &["PRIVATE_CANARY_äöüß".into()]
        )
        .await
        .is_err());
    server.join().unwrap();
    target.set_nonblocking(true).unwrap();
    assert!(
        matches!(target.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}
