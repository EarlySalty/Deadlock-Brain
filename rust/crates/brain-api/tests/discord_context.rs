use brain_api::{router, ApiService};
use brain_client::AsyncBrainClient;
use brain_contracts::{
    discord_task::{
        DiscordAnswerCapability, DiscordAnswerTask, DiscordContextResolver, DiscordReference,
    },
    AnswerProfile, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort, Principal, Query,
    SourceRecordV2, SourceVisibility,
};
use brain_kernel::Kernel;
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
async fn both_routes_resolve_only_public_subjects_before_real_provider_transport() {
    let store = store().await;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let recorded = requests.clone();
    let provider_server = std::thread::spawn(move || {
        for (subject, question) in [
            ("Paten", "Wie mache ich das? \nGesprächsthema: Paten"),
            ("Coaching", "Wie mache ich das? \nGesprächsthema: Coaching"),
            (
                "Sprachkanäle",
                "Wie mache ich das? \nGesprächsthema: Sprachkanäle",
            ),
            (
                "Paten",
                "Wie kann ich das verbessern?\nGesprächsthema: Paten",
            ),
            ("Abrams", "Welche Items passen zu Abrams?"),
            (
                "Abrams",
                "Okay ja ne Idee für ein Spirit build\nGesprächsthema: Abrams",
            ),
            ("Abrams", "Abrams\nGesprächsthema: Abrams Spirit-Build"),
            ("Seelen", "Wie bekomme ich mehr Seelen?"),
            ("Abrams", "Warum?\nGesprächsthema: Abrams"),
            (
                "Paten",
                "Kann ich das mit Coaching kombinieren?\nGesprächsthema: Paten",
            ),
            (
                "Paten",
                "Kann ich das mit Coaching kombinieren?\nGesprächsthema: Paten",
            ),
            (
                "Paten",
                "Kann ich das mit Coaching kombinieren?\nGesprächsthema: Paten",
            ),
            ("Paten", "Warum?\nGesprächsthema: Coaching Paten"),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            assert!(request.starts_with("POST /v1/messages "));
            let data: Value =
                serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
            let model_input: Value =
                serde_json::from_str(data["messages"][0]["content"].as_str().unwrap()).unwrap();
            assert_eq!(model_input["query"], question);
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
            let ids: Vec<Value> = evidence.iter().map(|item| item["id"].clone()).collect();
            assert!(!ids.is_empty());
            let answer =
                json!({"text":format!("{subject} helfen dir hier."), "cited_evidence_ids":ids})
                    .to_string();
            let response = json!({"model":"gpt-6-luna", "stop_reason":"end_turn", "content":[{"type":"text","text":answer}], "usage":{"input_tokens":12,"output_tokens":3}}).to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response.len(),
                response
            )
            .unwrap();
            recorded.lock().unwrap().push(request);
        }
    });
    let api = tokio::task::spawn_blocking(move || {
        let retrieval = ReleaseRetriever::new(store, 8);
        ApiService::new(
            PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture",
                "bot",
                "discord",
                BTreeSet::from(["bot.public".into()]),
                BTreeSet::from(["public".into()]),
            )])),
            Kernel::new(
                retrieval.clone(),
                CodexSubscriptionProvider::new(ProviderConfig::codex_subscription(format!(
                    "http://{address}/v1"
                )))
                .unwrap(),
            ),
            "context-release",
            5000,
            Budget::default(),
        )
        .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]))
        .with_discord_context_resolver(retrieval)
    })
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(api)).await.unwrap() });
    let client = AsyncBrainClient::new_local(&endpoint, "fixture", Duration::from_secs(5)).unwrap();
    for (id, capability, prior) in [
        (
            "concierge-followup",
            DiscordAnswerCapability::Concierge,
            "Wie finde ich einen Paten?",
        ),
        (
            "faq-followup",
            DiscordAnswerCapability::Faq,
            "Wo gibt es Coaching?",
        ),
        (
            "faq-umlaut",
            DiscordAnswerCapability::Faq,
            "Wie öffne ich Sprachkanäle?",
        ),
    ] {
        let task = DiscordAnswerTask {
            capability,
            channel_id: 123456789012345678,
        };
        let mut prior = format!("{prior} PRIVATE_CANARY_äöüß 76561197960265839");
        prior.push_str(&"ä".repeat(4000 - prior.chars().count()));
        let history = vec![prior];
        let response = client
            .answer_discord_task_with_history(
                &query(id, "Wie mache ich das? <@76561197960265839>"),
                42,
                &task,
                &history,
            )
            .await
            .unwrap();
        assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    }
    let response = client
        .answer_discord_task_with_history(
            &query("pure-followups", "Wie kann ich das verbessern?"),
            42,
            &DiscordAnswerTask {
                capability: DiscordAnswerCapability::Concierge,
                channel_id: 10,
            },
            &["Paten?", "Wie mache ich das?", "Warum?", "Erzähl mir mehr"].map(str::to_owned),
        )
        .await
        .unwrap();
    assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    let first = "Welche Items passen zu Abrams?";
    let second = "Okay ja ne Idee für ein Spirit build";
    for (id, text, history) in [
        ("direct-first", first, vec![]),
        (
            "direct-second",
            second,
            vec![format!("{first} PRIVATE_CANARY_äöüß <@76561197960265839>")],
        ),
        (
            "direct-third",
            "Abrams",
            vec![
                format!("{first} PRIVATE_CANARY_äöüß <@76561197960265839>"),
                second.to_owned(),
            ],
        ),
        (
            "direct-independent-after-build",
            "Wie bekomme ich mehr Seelen?",
            vec![
                format!("{first} PRIVATE_CANARY_äöüß <@76561197960265839>"),
                second.to_owned(),
            ],
        ),
        (
            "direct-followup-after-build",
            "Warum?",
            vec![
                format!("{first} PRIVATE_CANARY_äöüß <@76561197960265839>"),
                second.to_owned(),
            ],
        ),
    ] {
        let response = client
            .answer_for_discord_with_history(&query(id, text), 42, &history)
            .await
            .unwrap();
        assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    }
    let mixed_history =
        vec!["Wie finde ich einen Paten? PRIVATE_CANARY_äöüß <@76561197960265839>".into()];
    let mixed = "Kann ich das mit Coaching kombinieren?";
    for capability in [
        DiscordAnswerCapability::Concierge,
        DiscordAnswerCapability::Faq,
    ] {
        let response = client
            .answer_discord_task_with_history(
                &query("mixed-task", mixed),
                42,
                &DiscordAnswerTask {
                    capability,
                    channel_id: 123456789012345678,
                },
                &mixed_history,
            )
            .await
            .unwrap();
        assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    }
    let response = client
        .answer_for_discord_with_history(&query("mixed-direct", mixed), 42, &mixed_history)
        .await
        .unwrap();
    assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    let mut chained_history = mixed_history;
    chained_history.push(mixed.into());
    let response = client
        .answer_for_discord_with_history(&query("mixed-chain", "Warum?"), 42, &chained_history)
        .await
        .unwrap();
    assert_eq!(response.status, AnswerStatus::Answered, "{}", response.text);
    for history in [
        vec![first, "Und wie spiele ich PRIVATE_CANARY_äöüß?"],
        vec![first, "Und wie bekomme ich Rollen?"],
        vec!["Paten?"],
        vec![],
    ] {
        let response = client
            .answer_for_discord_with_history(
                &query("direct-unknown", second),
                42,
                &history.into_iter().map(str::to_owned).collect::<Vec<_>>(),
            )
            .await
            .unwrap();
        assert_eq!(response.status, AnswerStatus::InsufficientEvidence);
        assert!(response.citations.is_empty());
    }
    for (id, history) in [
        ("missing", vec![]),
        ("unknown", vec!["PRIVATE_CANARY_äöüß".into()]),
        ("ambiguous", vec!["Paten oder Coaching?".into()]),
        ("private-source", vec!["Was sind Rollen?".into()]),
        (
            "changed-topic",
            vec!["Paten?".into(), "Unbekanntes Privatproblem".into()],
        ),
        (
            "private-followup-topic",
            vec!["Paten?".into(), "Und wie bekomme ich Rollen?".into()],
        ),
        (
            "unknown-followup-topic",
            vec![
                "Paten?".into(),
                "Und wie finde ich PRIVATE_CANARY_äöüß?".into(),
            ],
        ),
        (
            "unknown-pronoun-topic",
            vec![
                "Paten?".into(),
                "Wie kann ich das Privatproblem verbessern?".into(),
            ],
        ),
    ] {
        let response = client
            .answer_discord_task_with_history(
                &query(id, "Wie mache ich das?"),
                42,
                &DiscordAnswerTask {
                    capability: DiscordAnswerCapability::Faq,
                    channel_id: 10,
                },
                &history,
            )
            .await
            .unwrap();
        assert_eq!(response.status, AnswerStatus::InsufficientEvidence);
        assert!(response.citations.is_empty());
        assert!(!response.text.is_empty());
    }
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    for body in [
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":[""]}),
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":[{"role":"assistant", "content":"PRIVATE_CANARY_äöüß"}]}),
        json!({"query":query("invalid-context", "Wie mache ich das?"), "user_questions":["Paten"], "resolved_subject":"PRIVATE_CANARY_äöüß"}),
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
    assert_eq!(requests.lock().unwrap().len(), 13);
    provider_server.join().unwrap();
    server.abort();
}

#[tokio::test]
async fn local_reference_rechecks_current_public_acl_and_tombstones() {
    for tombstone in [false, true] {
        let store = store().await;
        let retrieval = ReleaseRetriever::new(store.clone(), 8);
        let q = query("revoked", "Wie mache ich das?");
        let history = vec![
            "Coaching?".into(),
            "Und wie finde ich einen Paten? PRIVATE_CANARY_äöüß".into(),
        ];
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            DiscordReference::Subject("Paten".into())
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
            DiscordReference::Clarification
        );
    }
}

#[tokio::test]
async fn local_reference_reuses_public_hero_names_without_numeric_identifiers() {
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
    let q = query("hero-followup", "Und seine Ult?");
    let mut context = context(&q);
    context.knowledge_release = "game-release".into();
    assert_eq!(
        retrieval
            .resolve(
                &q,
                &context,
                &["Was macht der Wächter? PRIVATE_CANARY_äöüß".into()]
            )
            .unwrap(),
        DiscordReference::Subject("Warden".into())
    );
    for text in ["Und seine Ult?", "Wie kann ich sie spielen?"] {
        let q = query("hero-followup", text);
        assert_eq!(
            retrieval
                .resolve(
                    &q,
                    &context,
                    &["Was macht der Wächter?", "Wie spielt er?", "Warum?"].map(str::to_owned),
                )
                .unwrap(),
            DiscordReference::Subject("Warden".into())
        );
    }
    assert_eq!(
        retrieval
            .resolve(&q, &context, &["Was kostet 25?".into()])
            .unwrap(),
        DiscordReference::Clarification
    );
}

#[tokio::test]
async fn direct_build_history_preserves_independent_questions_and_safe_followups() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    let history = [
        "Welche Items passen zu Abrams?",
        "Okay ja ne Idee für ein Spirit build",
    ]
    .map(str::to_owned);
    for (text, expected) in [
        (
            "Wie bekomme ich mehr Seelen?",
            DiscordReference::Independent,
        ),
        ("Wie bekomme ich mehr Seelen", DiscordReference::Independent),
        (
            "Gibt es mehr Seelen im Dschungel?",
            DiscordReference::Independent,
        ),
        (
            "Kann ich mehr Seelen farmen?",
            DiscordReference::Independent,
        ),
        ("Warum?", DiscordReference::Subject("Abrams".into())),
        ("Und seine Ult?", DiscordReference::Subject("Abrams".into())),
        (
            "Okay ja ne Idee für ein Spirit build",
            DiscordReference::Subject("Abrams".into()),
        ),
        ("Unbekannt", DiscordReference::Clarification),
        ("Unbekannter Held", DiscordReference::Clarification),
        ("PRIVATE_CANARY_äöüß", DiscordReference::Clarification),
        (
            "Und wie bekomme ich Rollen?",
            DiscordReference::Clarification,
        ),
        (
            "Wie kann ich das verbessern mit Privatproblem?",
            DiscordReference::Clarification,
        ),
    ] {
        let mut q = query("direct-after-build", text);
        q.answer_context = Some(brain_contracts::AnswerContext::Discord(
            brain_contracts::DiscordAnswerContext {
                purpose: Some("bot_context:direct".into()),
                ..Default::default()
            },
        ));
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            expected,
            "{text}",
        );
        if text == "Warum?" {
            for barrier in ["Und wie bekomme ich Rollen?", "Unbekannter Held"] {
                assert_eq!(
                    retrieval
                        .resolve(
                            &q,
                            &context(&q),
                            &[history[0].clone(), barrier.into(), history[1].clone()],
                        )
                        .unwrap(),
                    DiscordReference::Clarification,
                    "{barrier}",
                );
            }
        }
    }
}

#[tokio::test]
async fn direct_build_reference_rechecks_public_hero_and_rejects_private_intents() {
    let store = store().await;
    let retrieval = ReleaseRetriever::new(store.clone(), 8);
    let mut q = query("direct-hero", "Abrams");
    q.answer_context = Some(brain_contracts::AnswerContext::Discord(
        brain_contracts::DiscordAnswerContext {
            purpose: Some("bot_context:direct".into()),
            ..Default::default()
        },
    ));
    let history = [
        "Welche Items passen zu Abrams?",
        "Okay ja ne Idee für ein Spirit build",
    ]
    .map(str::to_owned);
    assert_eq!(
        retrieval.resolve(&q, &context(&q), &history).unwrap(),
        DiscordReference::Subject("Abrams Spirit-Build".into())
    );
    assert_eq!(
        retrieval
            .resolve(
                &q,
                &context(&q),
                &["Spirit build PRIVATE_CANARY_äöüß".into()]
            )
            .unwrap(),
        DiscordReference::Independent
    );
    q.text = "Unbekannt".into();
    assert_eq!(
        retrieval.resolve(&q, &context(&q), &history).unwrap(),
        DiscordReference::Clarification
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
        for text in ["Abrams", "Warum?", "Und seine Ult?"] {
            q.text = text.into();
            assert_eq!(
                retrieval.resolve(&q, &context(&q), &history).unwrap(),
                DiscordReference::Clarification,
                "{text}, tombstone={tombstone}",
            );
        }
        q.text = "Wie bekomme ich mehr Seelen?".into();
        assert_eq!(
            retrieval.resolve(&q, &context(&q), &history).unwrap(),
            DiscordReference::Independent,
        );
        q.text = "Okay ja ne Idee für ein Spirit build".into();
        assert_eq!(
            retrieval
                .resolve(&q, &context(&q), &[history[0].clone()])
                .unwrap(),
            DiscordReference::Clarification
        );
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

impl brain_kernel::AnswerKernelPort for NeverKernel {
    fn answer(&self, _: &Query, _: &AuthorizedContext) -> brain_contracts::AnswerResponse {
        panic!("Ungeprüfter Bezug darf den Kernel nicht erreichen")
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
async fn newer_unsupported_topics_stop_both_routes_before_the_kernel() {
    let store = store().await;
    let mut revoked = record(
        "coaching",
        "Coaching hilft beim Spielen. Für Coaching meldest du dich im Coaching-Bereich.",
        true,
    );
    revoked.revision = 2;
    store.apply_record(revoked).unwrap();
    let api = ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture",
            "bot",
            "discord",
            BTreeSet::from(["bot.public".into()]),
            BTreeSet::from(["public".into()]),
        )])),
        NeverKernel,
        "context-release",
        5000,
        Budget::default(),
    )
    .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]))
    .with_discord_context_resolver(ReleaseRetriever::new(store, 8));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(api)).await.unwrap() });
    let client = AsyncBrainClient::new_local(&endpoint, "fixture", Duration::from_secs(5)).unwrap();
    for capability in [
        Some(DiscordAnswerCapability::Faq),
        Some(DiscordAnswerCapability::Concierge),
        None,
    ] {
        for (current, history) in [
            (
                "Wie mache ich das?",
                vec!["Paten?", "Und wie bekomme ich Rollen?"],
            ),
            (
                "Wie mache ich das?",
                vec!["Paten?", "Und wie finde ich PRIVATE_CANARY_äöüß?"],
            ),
            (
                "Wie mache ich das?",
                vec!["Paten?", "Und wie finde ich Coaching?"],
            ),
            (
                "Wie kann ich das verbessern?",
                vec!["Paten?", "Und wie bekomme ich Rollen?", "Warum?"],
            ),
            (
                "Wie kann ich sie spielen?",
                vec!["Paten?", "Und wie spiele ich Unbekannt?"],
            ),
            ("Und wie bekomme ich Rollen?", vec!["Paten?"]),
            (
                "Wie mache ich das?",
                vec!["Paten?", "Und wie bekomme ich Ult?"],
            ),
            (
                "Wie mache ich das?",
                vec!["Paten?", "Und wie finde ich <#123456789012345678>?"],
            ),
            (
                "Wie kann ich das verbessern mit Privatproblem?",
                vec!["Paten?"],
            ),
            (
                "Kann ich das mit Paten kombinieren?",
                vec!["Sprachkanäle?", "Coaching?"],
            ),
            (
                "Kann ich das mit Paten kombinieren?",
                vec!["Sprachkanäle?", "PRIVATE_CANARY_äöüß", "Warum?"],
            ),
            (
                "Kann ich das mit Paten kombinieren?",
                vec!["Sprachkanäle?", "Wie bekomme ich Rollen?"],
            ),
            (
                "Warum?",
                vec![
                    "Sprachkanäle?",
                    "Coaching?",
                    "Kann ich das mit Paten kombinieren?",
                ],
            ),
        ] {
            let history = history.into_iter().map(str::to_owned).collect::<Vec<_>>();
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
            assert_eq!(
                response.status,
                AnswerStatus::InsufficientEvidence,
                "{current}"
            );
            assert!(response.citations.is_empty());
            assert!(!response.text.is_empty());
            assert!(!response.text.contains("PRIVATE_CANARY_äöüß"));
        }
    }
    server.abort();
}

#[tokio::test]
async fn current_public_topic_does_not_need_an_older_reference() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    for (text, expected) in [
        ("Und wie bekomme ich Paten?", DiscordReference::Independent),
        (
            "Und wie bekomme ich Rollen?",
            DiscordReference::Clarification,
        ),
        (
            "Und wie finde ich Unbekannt?",
            DiscordReference::Clarification,
        ),
    ] {
        let q = query("current-topic", text);
        assert_eq!(
            retrieval
                .resolve(&q, &context(&q), &["Paten?".into()])
                .unwrap(),
            expected,
            "{text}",
        );
    }
}

#[tokio::test]
async fn mixed_public_topics_preserve_only_the_needed_local_reference() {
    let retrieval = ReleaseRetriever::new(store().await, 8);
    for purpose in ["bot_context:direct", "bot_task:concierge", "bot_task:faq"] {
        for (text, prior, expected) in [
            (
                "Kann ich das mit Coaching kombinieren?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Kann ich es mit Coaching verbinden?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Wie kann ich sie mit Coaching vergleichen?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Kann ich das statt Coaching nutzen?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Wie spielt Abrams seine Ult?",
                "Wo gibt es Coaching?",
                DiscordReference::Independent,
            ),
            (
                "Kann ich das mit Abrams seiner Ult kombinieren?",
                "Wo gibt es Coaching?",
                DiscordReference::Subject("Coaching".into()),
            ),
            (
                "Wie kann ich das für Sprachkanäle nutzen?",
                "Wo gibt es Coaching?",
                DiscordReference::Subject("Coaching".into()),
            ),
            (
                "Kann ich es für Coaching nutzen?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Passt das zu Coaching?",
                "Wie finde ich einen Paten?",
                DiscordReference::Subject("Paten".into()),
            ),
            (
                "Was ist das für ein Coaching?",
                "Wie finde ich einen Paten?",
                DiscordReference::Independent,
            ),
            (
                "Kann ich das mit Coaching und Paten kombinieren?",
                "Wie öffne ich Sprachkanäle?",
                DiscordReference::Subject("Sprachkanäle".into()),
            ),
            (
                "Gibt es Coaching?",
                "Wie finde ich einen Paten?",
                DiscordReference::Independent,
            ),
            (
                "Und wie bekomme ich Paten?",
                "Wo gibt es Coaching?",
                DiscordReference::Independent,
            ),
            (
                "Was ist das Coaching?",
                "Wie finde ich einen Paten?",
                DiscordReference::Independent,
            ),
            (
                "Coaching oder Paten?",
                "Wie öffne ich Sprachkanäle?",
                DiscordReference::Independent,
            ),
            (
                "Kann ich das mit Coaching und Privatproblem kombinieren?",
                "Wie finde ich einen Paten?",
                DiscordReference::Clarification,
            ),
        ] {
            let mut q = query("mixed-reference", text);
            q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            assert_eq!(
                retrieval
                    .resolve(&q, &context(&q), &[prior.into()])
                    .unwrap(),
                expected,
                "{purpose}: {text}",
            );
        }
    }
}

#[tokio::test]
async fn mixed_reference_chains_stop_at_unknown_private_and_revoked_topics() {
    for tombstone in [false, true] {
        let store = store().await;
        let retrieval = ReleaseRetriever::new(store.clone(), 8);
        for purpose in ["bot_context:direct", "bot_task:concierge", "bot_task:faq"] {
            let mut q = query("mixed-chain", "Warum?");
            q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                brain_contracts::DiscordAnswerContext {
                    purpose: Some(purpose.into()),
                    ..Default::default()
                },
            ));
            assert_eq!(
                retrieval
                    .resolve(
                        &q,
                        &context(&q),
                        &["Paten?", "Kann ich das mit Coaching kombinieren?", "Warum?"]
                            .map(str::to_owned),
                    )
                    .unwrap(),
                DiscordReference::Subject("Coaching Paten".into()),
            );
            assert_eq!(
                retrieval
                    .resolve(
                        &q,
                        &context(&q),
                        &[
                            "Sprachkanäle?",
                            "Kann ich das mit Coaching und Paten kombinieren?"
                        ]
                        .map(str::to_owned),
                    )
                    .unwrap(),
                DiscordReference::Subject("Coaching Paten Sprachkanäle".into()),
            );
            for barrier in ["PRIVATE_CANARY_äöüß", "Wie bekomme ich Rollen?"] {
                for text in ["Warum?", "Kann ich das mit Coaching kombinieren?"] {
                    q.text = text.into();
                    assert_eq!(
                        retrieval
                            .resolve(
                                &q,
                                &context(&q),
                                &["Paten?", barrier, "Warum?"].map(str::to_owned),
                            )
                            .unwrap(),
                        DiscordReference::Clarification,
                        "{purpose}: {text}, {barrier}",
                    );
                }
                q.text = "Warum?".into();
                assert_eq!(
                    retrieval
                        .resolve(
                            &q,
                            &context(&q),
                            &["Paten?", barrier, "Kann ich das mit Coaching kombinieren?"]
                                .map(str::to_owned),
                        )
                        .unwrap(),
                    DiscordReference::Clarification,
                );
            }
        }
        let mut revoked = record(
            "paten",
            "Paten helfen beim Einstieg. Für Paten meldest du dich beim Concierge.",
            !tombstone,
        );
        revoked.revision = 2;
        revoked.tombstone = tombstone;
        store.apply_record(revoked).unwrap();
        for purpose in ["bot_context:direct", "bot_task:concierge", "bot_task:faq"] {
            for (text, history) in [
                (
                    "Kann ich das mit Coaching kombinieren?",
                    vec!["Wie öffne ich Sprachkanäle?", "Wie finde ich einen Paten?"],
                ),
                (
                    "Warum?",
                    vec!["Paten?", "Kann ich das mit Coaching kombinieren?"],
                ),
            ] {
                let mut q = query("mixed-revoked", text);
                q.answer_context = Some(brain_contracts::AnswerContext::Discord(
                    brain_contracts::DiscordAnswerContext {
                        purpose: Some(purpose.into()),
                        ..Default::default()
                    },
                ));
                assert_eq!(
                    retrieval
                        .resolve(
                            &q,
                            &context(&q),
                            &history.into_iter().map(str::to_owned).collect::<Vec<_>>(),
                        )
                        .unwrap(),
                    DiscordReference::Clarification,
                    "{purpose}: {text}, tombstone={tombstone}",
                );
            }
        }
    }
}

struct InvalidProjection(String);

impl DiscordContextResolver for InvalidProjection {
    fn resolve(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[String],
    ) -> Result<DiscordReference, brain_contracts::PortError> {
        Ok(DiscordReference::Subject(self.0.clone()))
    }
}

#[tokio::test]
async fn invalid_projection_and_missing_local_release_fail_without_kernel_fallback() {
    for projection in [
        None,
        Some(String::new()),
        Some("<@42>".into()),
        Some("ä".repeat(81)),
    ] {
        let mut api = ApiService::new(
            PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture",
                "bot",
                "discord",
                BTreeSet::from(["bot.public".into()]),
                BTreeSet::from(["public".into()]),
            )])),
            NeverKernel,
            "context-release",
            5000,
            Budget::default(),
        )
        .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]));
        let unavailable = projection.is_none();
        api = match projection {
            Some(subject) => api.with_discord_context_resolver(InvalidProjection(subject)),
            None => api.with_discord_context_resolver(ReleaseRetriever::new(
                MemoryRepository::default(),
                8,
            )),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, router(api)).await.unwrap() });
        for capability in ["faq", "concierge"] {
            let response = reqwest::Client::builder().no_proxy().build().unwrap()
                .post(format!("{endpoint}/v1/answer")).bearer_auth("fixture")
                .header("x-discord-user-id", "42").header("x-discord-read-access", "disabled")
                .header("x-discord-answer-task", json!({"capability":capability, "channel_id":10}).to_string())
                .json(&json!({"query":query(capability, "Wie mache ich das?"), "user_questions":["Paten PRIVATE_CANARY_äöüß"]}))
                .send().await.unwrap();
            assert_eq!(
                response.status().as_u16(),
                if unavailable { 503 } else { 200 }
            );
            let body: Value = response.json().await.unwrap();
            if unavailable {
                assert_eq!(body["error"]["code"], "context_unavailable");
            } else {
                assert_eq!(body["status"], "insufficient_evidence");
                assert_eq!(body["citations"], json!([]));
            }
            assert!(!body.to_string().contains("PRIVATE_CANARY_äöüß"));
        }
        server.abort();
    }
}

#[tokio::test]
async fn direct_history_requires_trusted_identity_and_server_owned_projection() {
    let api = ApiService::new(
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
    let server = tokio::spawn(async move { axum::serve(listener, router(api)).await.unwrap() });
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
        .headers(headers)
        .header("x-discord-read-access", "disabled")
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
