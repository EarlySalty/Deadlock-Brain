use super::*;
use brain_contracts::{
    invite::{self, InviteStatus, SelfInviteStatus},
    AnswerProviderPort, AnswerStatus, Budget, DiscordRequestContext, Principal, ProviderAnswer,
};
use brain_kernel::{AnswerKernelPort, Kernel};
use brain_providers::{CodexSubscriptionProvider, OpenAiCompatibleProvider, ProviderConfig};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

struct NoOtherSource;
impl RetrievalPort for NoOtherSource {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        panic!("Die private Statusfrage darf keinen weiteren Retriever erreichen")
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        panic!("Statusbelege dürfen keine andere Quellenprüfung erreichen")
    }
    fn validate_publication(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<(), PortError> {
        panic!("Statusbelege dürfen keine andere Publikationsprüfung erreichen")
    }
}

fn query() -> Query {
    serde_json::from_value(json!({
        "request_id":"r-Geheimtext", "conversation_id":"c-Geheimtext",
        "text":"Einladungsstatus für Fremdname und Steamcode 123456, Chatkontext Geheimtext?",
        "requested_scopes":["bot.public"]
    }))
    .unwrap()
}

fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: Some(DiscordRequestContext {
            user_id: Some(42),
            request_id: query().request_id,
            scope: "discord.request:test".into(),
        }),
        principal: Principal {
            actor_id: "test-consumer".into(),
            channel: "discord".into(),
            scopes: BTreeSet::from(["bot.public".into(), "discord.request:test".into()]),
            provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
        },
        conversation_id: query().conversation_id,
        knowledge_release: "test-release".into(),
        deadline_ms: 10000,
        budget: Budget::default(),
        request_deadline: None,
    }
}

fn http_request(stream: &mut TcpStream) -> (String, Value) {
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    let mut reader = BufReader::new(stream);
    let mut headers = String::new();
    let mut length = 0;
    loop {
        let mut line = String::new();
        assert_ne!(reader.read_line(&mut line).unwrap(), 0);
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
            length = value.trim().parse().unwrap();
        }
        headers.push_str(&line);
    }
    let mut bytes = vec![0; length];
    reader.read_exact(&mut bytes).unwrap();
    (headers, serde_json::from_slice(&bytes).unwrap())
}

fn reply(stream: &mut TcpStream, body: Value) {
    let body = body.to_string();
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
}

fn rpc(status: Value) -> Value {
    json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":status.to_string()}]}})
}

fn live(listener: &TcpListener) -> Arc<DiscordLive> {
    let mut adapter = DiscordLive::new("test-access".into()).unwrap();
    adapter.endpoint = format!("http://{}/mcp/public", listener.local_addr().unwrap());
    Arc::new(adapter)
}

struct ActualProvider(Arc<dyn AnswerProviderPort>);
impl AnswerProviderPort for ActualProvider {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.0.answer(query, context, evidence)
    }
}

#[test]
fn alle_statuswerte_durchlaufen_den_kernel_und_beide_echten_providertransporte() {
    for subscription in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let source = live(&listener);
        let config = if subscription {
            ProviderConfig::codex_subscription(&endpoint)
        } else {
            let mut config = ProviderConfig::new("test-access", &endpoint, "fixture-model");
            config.retry_attempts = 1;
            config
        };
        let expected_model = config.model.clone();
        let provider: Arc<dyn AnswerProviderPort> = if subscription {
            Arc::new(CodexSubscriptionProvider::new(config).unwrap())
        } else {
            Arc::new(OpenAiCompatibleProvider::new(config).unwrap())
        };
        let statuses = [
            "sent",
            "pending",
            "friendship_missing",
            "already_has_game",
            "error",
            "unknown",
            "unavailable",
        ];
        let server = std::thread::spawn(move || {
            for status in statuses {
                let at = if status == "sent" {
                    json!("2026-01-01T12:00:00Z")
                } else {
                    json!(null)
                };
                let minimal = json!({"status":status,"at":at});
                let (mut stream, _) = listener.accept().unwrap();
                let (headers, request) = http_request(&mut stream);
                assert!(headers.starts_with("POST /mcp/public "));
                assert!(headers.contains("x-discord-user-id: 42\r\n"));
                assert!(headers.contains("x-discord-request-id: r-Geheimtext\r\n"));
                assert!(headers.contains("authorization: Bearer test-access\r\n"));
                assert_eq!(
                    request["params"],
                    json!({"name":"self_invite_status","arguments":{}})
                );
                for private in ["Fremdname", "123456", "Chatkontext"] {
                    assert!(!request.to_string().contains(private));
                }
                reply(&mut stream, rpc(minimal.clone()));
                drop(stream);
                let (mut stream, _) = listener.accept().unwrap();
                let (headers, payload) = http_request(&mut stream);
                assert!(headers.starts_with(if subscription {
                    "POST /messages "
                } else {
                    "POST /chat/completions "
                }));
                for private in [
                    "Fremdname",
                    "123456",
                    "Geheimtext",
                    "x-discord",
                    "discord.request:test",
                ] {
                    assert!(!headers.contains(private));
                    assert!(!payload.to_string().contains(private));
                }
                assert_eq!(payload["max_tokens"], Budget::default().max_output_tokens);
                assert_eq!(payload["model"], expected_model);
                let user = &payload["messages"][if subscription { 0 } else { 1 }];
                let supplied: Value =
                    serde_json::from_str(user["content"].as_str().unwrap()).unwrap();
                assert_eq!(
                    supplied,
                    json!({"query":invite::QUESTION,"evidence":[{"id":invite::EVIDENCE_ID,"citation":invite::CITATION,"content":minimal}]})
                );
                let grounded = json!({"text":format!("Status {status}"),"cited_evidence_ids":[invite::EVIDENCE_ID]}).to_string();
                let response = if subscription {
                    json!({"model":expected_model,"content":[{"type":"text","text":grounded}],"usage":{"input_tokens":100,"output_tokens":10}})
                } else {
                    json!({"model":"fixture-model","choices":[{"message":{"content":grounded}}],"usage":{"prompt_tokens":100,"completion_tokens":10}})
                };
                reply(&mut stream, response);
            }
        });
        let kernel = Kernel::new(
            DiscordRetriever::new(NoOtherSource, Some(source)),
            ActualProvider(provider),
        );
        for (index, status) in statuses.into_iter().enumerate() {
            let mut request = query();
            match index % 4 {
                0 => {
                    request.text =
                        "Bin ich eingeladen? Chatkontext Fremdname und Steamcode 123456, Geheimtext"
                            .into();
                }
                1 => {
                    request.text =
                        "How long has my invite been pending? Fremdname, Steamcode 123456, Geheimtext"
                            .into();
                }
                2 => {
                    request.text =
                        "Habe ich schon Zugang zu Deadlock? Fremdname, Steamcode 123456, Geheimtext"
                            .into();
                }
                _ => {}
            }
            let response = kernel.answer_for_publication(&request, &context());
            assert_eq!(response.status, AnswerStatus::Answered);
            assert_eq!(response.text, format!("Status {status}"));
            assert_eq!(response.usage.network_rounds, 2);
            assert_eq!(response.citations.len(), 1);
            assert_eq!(response.citations[0].citation, invite::CITATION);
        }
        server.join().unwrap();
    }
}

#[test]
fn normale_fragen_behalten_den_inneren_retriever_ohne_statusabruf() {
    struct Ordinary(Query, Arc<std::sync::atomic::AtomicUsize>);
    impl RetrievalPort for Ordinary {
        fn retrieve(
            &self,
            request: &Query,
            _: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            assert_eq!(request, &self.0);
            self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Vec::new())
        }
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let source = live(&listener);
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let texts = [
        "Wie kann ich eine Einladung verschicken?",
        "Wie wird eine Einladung verschickt?",
        "Welche FPS bekomme ich in Deadlock?",
        "Welche FPS bekomm ich in Deadlock?",
        "Wie funktioniert der Invite-Bot?",
        "Wo kann ich eine Einladung verschicken?",
        "Warum kann ich keine Einladung verschicken?",
        "Erkläre mir, wie ich eine Einladung verschicken kann.",
        "Sind Einladungen noch verfügbar?",
        "Hat der Invite-Bot noch offene Aufgaben?",
        "How long has the Invite-Bot been online?",
        "How long is an invite valid?",
        "What does 'how long has my invite been pending' mean?",
        "Habe ich schon Zugang zu Deadlock-Wiki?",
    ];
    for text in texts {
        let mut request = query();
        request.text = text.into();
        let adapter = DiscordRetriever::new(
            Ordinary(request.clone(), calls.clone()),
            Some(source.clone()),
        );
        let (items, usage) = adapter.retrieve_with_usage(&request, &context()).unwrap();
        assert!(items.is_empty());
        assert_eq!(usage.network_rounds, 0);
        assert!(adapter.observations.lock().unwrap().is_empty());
    }
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), texts.len());
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn fehlende_identitaet_hat_keinen_anonymen_netzwerkfallback() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, Some(live(&listener)));
    for user_id in [None, Some(0)] {
        let mut context = context();
        context.discord.as_mut().unwrap().user_id = user_id;
        let (items, usage) = adapter.retrieve_with_usage(&query(), &context).unwrap();
        assert_eq!(
            invite::projection(&query(), &items).unwrap(),
            SelfInviteStatus {
                status: InviteStatus::Unknown,
                at: None
            }
        );
        assert_eq!(usage.network_rounds, 0);
        adapter
            .validate_publication(&query(), &context, &items)
            .unwrap();
    }
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn transportfehler_bleiben_unavailable_statt_erfolg_oder_ablehnung() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, Some(live(&listener)));
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        http_request(&mut stream);
        write!(
            stream,
            "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
    });
    let (items, usage) = adapter.retrieve_with_usage(&query(), &context()).unwrap();
    assert_eq!(
        invite::projection(&query(), &items).unwrap(),
        SelfInviteStatus {
            status: InviteStatus::Unavailable,
            at: None
        }
    );
    assert_eq!(usage.network_rounds, 1);
    adapter
        .validate_evidence(&query(), &context(), &items, true)
        .unwrap();
    server.join().unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, None);
    let (items, usage) = adapter.retrieve_with_usage(&query(), &context()).unwrap();
    assert_eq!(
        invite::projection(&query(), &items).unwrap().status,
        InviteStatus::Unavailable
    );
    assert_eq!(usage.network_rounds, 0);
}

#[test]
fn widerspruechliche_rpc_antworten_belegen_keinen_erfolgsstatus() {
    let valid = rpc(json!({"status":"sent","at":null}));
    let mut with_error = valid.clone();
    with_error["error"] = json!({"code":-32603,"message":"Geheimtext"});
    let mut wrong_id = valid.clone();
    wrong_id["id"] = json!(99);
    let mut wrong_version = valid.clone();
    wrong_version["jsonrpc"] = json!("1.0");
    let mut tool_error = valid;
    tool_error["result"]["isError"] = json!(true);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, Some(live(&listener)));
    let server = std::thread::spawn(move || {
        for response in [with_error, wrong_id, wrong_version, tool_error] {
            let (mut stream, _) = listener.accept().unwrap();
            http_request(&mut stream);
            reply(&mut stream, response);
        }
    });
    for _ in 0..4 {
        let (items, usage) = adapter.retrieve_with_usage(&query(), &context()).unwrap();
        assert_eq!(usage.network_rounds, 1);
        assert_eq!(
            invite::projection(&query(), &items).unwrap().status,
            InviteStatus::Unavailable
        );
    }
    server.join().unwrap();
}

#[test]
fn fremdfelder_unbekannte_enums_und_rohfehler_werden_abgelehnt() {
    let values = [
        json!({"status":"success","at":null}),
        json!({"status":"sent","at":null,"steam_id":"fremd"}),
        json!({"status":"error","at":null,"error":"Geheimtext"}),
        json!({"status":"sent","at":"Fremdname"}),
    ];
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, Some(live(&listener)));
    let server = std::thread::spawn(move || {
        for value in values {
            let (mut stream, _) = listener.accept().unwrap();
            http_request(&mut stream);
            reply(&mut stream, rpc(value));
        }
    });
    for _ in 0..4 {
        assert!(matches!(
            adapter.retrieve_with_usage(&query(), &context()),
            Err(PortError::PermissionDenied(_))
        ));
    }
    server.join().unwrap();
    assert!(adapter.observations.lock().unwrap().is_empty());
}

#[test]
fn identitaet_scope_budget_und_requestbindung_bleiben_verbindlich() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let adapter = DiscordRetriever::new(NoOtherSource, Some(live(&listener)));
    let original = context();
    for context in [
        AuthorizedContext {
            discord: None,
            ..original.clone()
        },
        AuthorizedContext {
            principal: Principal {
                scopes: BTreeSet::new(),
                ..original.principal.clone()
            },
            ..original.clone()
        },
        AuthorizedContext {
            discord: Some(DiscordRequestContext {
                request_id: "falsch".into(),
                ..original.discord.clone().unwrap()
            }),
            ..original.clone()
        },
    ] {
        assert!(matches!(
            adapter.retrieve_with_usage(&query(), &context),
            Err(PortError::PermissionDenied(_))
        ));
    }
    let mut exhausted = original;
    exhausted.budget.max_network_rounds = 0;
    assert!(matches!(
        adapter.retrieve_with_usage(&query(), &exhausted),
        Err(PortError::BudgetExceeded)
    ));
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn beobachtung_bleibt_eigen_requestgebunden_und_nicht_manipulierbar() {
    let adapter = DiscordRetriever::new(NoOtherSource, None);
    let (items, _) = adapter.retrieve_with_usage(&query(), &context()).unwrap();
    for provider in [false, true] {
        adapter
            .validate_evidence(&query(), &context(), &items, provider)
            .unwrap();
    }
    let mut changed = items.clone();
    changed[0].content = json!({"status":"sent","at":null}).to_string();
    assert!(adapter
        .validate_publication(&query(), &context(), &changed)
        .is_err());
    let mut other = context();
    other.discord.as_mut().unwrap().user_id = Some(99);
    assert!(adapter
        .validate_publication(&query(), &other, &items)
        .is_err());
    let mut denied = context();
    denied.principal.provider_egress.clear();
    assert!(adapter
        .validate_evidence(&query(), &denied, &items, true)
        .is_err());
    let mut other_query = query();
    other_query.text.push_str(" weiterer Chatkontext");
    assert!(adapter
        .validate_publication(&other_query, &context(), &items)
        .is_err());
    adapter
        .observations
        .lock()
        .unwrap()
        .get_mut(&observation_key(&query(), &context()).unwrap())
        .unwrap()
        .0 = Instant::now();
    assert!(adapter
        .validate_publication(&query(), &context(), &items)
        .is_err());
}

#[test]
fn statusvalidierung_lehnt_gemischte_quellen_vor_jedem_inneren_adapter_ab() {
    let adapter = DiscordRetriever::new(NoOtherSource, None);
    let (items, _) = adapter.retrieve_with_usage(&query(), &context()).unwrap();
    let mut ordinary = items[0].clone();
    ordinary.source_id = "docs.public".into();
    ordinary.visibility = SourceVisibility::Public;
    ordinary.allowed_scopes.clear();
    let mut mixed = items;
    mixed.push(ordinary);
    for provider in [false, true] {
        assert!(adapter
            .validate_evidence(&query(), &context(), &mixed, provider)
            .is_err());
    }
    assert!(adapter
        .validate_publication(&query(), &context(), &mixed)
        .is_err());
}

#[test]
fn provider_lehnt_fremde_und_gemischte_evidenz_ohne_netzwerk_ab() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let provider = OpenAiCompatibleProvider::new(ProviderConfig::new(
        "test-access",
        format!("http://{}", listener.local_addr().unwrap()),
        "fixture-model",
    ))
    .unwrap();
    let item = invite::status_evidence(
        &SelfInviteStatus {
            status: InviteStatus::Pending,
            at: None,
        },
        context().discord.unwrap().scope,
    )
    .unwrap();
    let mut foreign = item.clone();
    foreign.citation = "Fremdname".into();
    for items in [vec![], vec![foreign], vec![item.clone(), item.clone()]] {
        assert!(provider.answer(&query(), &context(), &items).is_err());
    }
    let mut denied = context();
    denied.principal.provider_egress.remove("discord_request");
    assert!(provider.answer(&query(), &denied, &[item]).is_err());
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
