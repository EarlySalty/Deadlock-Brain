use super::*;
use brain_contracts::bot_tasks::{
    BotCapability, BotTaskResponse, BotTaskResult, CapabilityAvailability, TaskFailure,
};
use brain_contracts::{
    AnswerResponse, AnswerStatus, AuthorizedContext, Budget, DiscordRequestContext, Principal,
    PublicAnswerResponse, Query, Usage,
};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_providers::{GenerationEndpoint, TextGenerationProvider, TwitchTitleGenerator};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    net::TcpListener,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::Mutex,
};

type Seen = (Query, Principal, String, Option<DiscordRequestContext>);

#[derive(Clone, Default)]
struct RecordingKernel(Arc<Mutex<Vec<Seen>>>);

impl AnswerKernelPort for RecordingKernel {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.0.lock().unwrap().push((
            query.clone(),
            context.principal.clone(),
            context.conversation_id.clone(),
            context.discord.clone(),
        ));
        AnswerResponse {
            contract_version: brain_contracts::CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: AnswerStatus::InsufficientEvidence,
            text: "Dazu habe ich keine Belege.".into(),
            citations: Vec::new(),
            usage: Usage::default(),
        }
    }

    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer(query, context)
    }
}

fn grant(token: &str, actor: &str, channel: &str, scope: &str, egress: &[&str]) -> AuthGrant {
    AuthGrant::from_secret(
        token,
        actor,
        channel,
        BTreeSet::from([scope.into()]),
        egress.iter().map(|value| (*value).to_owned()).collect(),
    )
}

fn service(kernel: RecordingKernel) -> ApiService<RecordingKernel> {
    ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![
            grant("bot-token", "bot", "discord", "bot.public", &["public"]),
            grant("other-token", "other", "discord", "bot.public", &["public"]),
            grant(
                "twitch-token",
                "twitch-bot",
                "twitch",
                "bot.public",
                &["public"],
            ),
            grant(
                "second-token",
                "second-brain",
                "internal",
                "second_brain.internal",
                &[],
            ),
        ])),
        kernel,
        "fixture-release",
        5000,
        Budget::default(),
    )
    .with_discord_consumers(BTreeSet::from([
        ("bot".into(), "discord".into()),
        ("other".into(), "discord".into()),
    ]))
    .with_bot_task_consumers(BTreeMap::from([
        (
            ("bot".into(), "discord".into()),
            BTreeSet::from([BotCapability::DiscordConcierge, BotCapability::DiscordFaq]),
        ),
        (
            ("twitch-bot".into(), "twitch".into()),
            BTreeSet::from([BotCapability::TwitchTitle]),
        ),
        (
            ("other".into(), "discord".into()),
            BTreeSet::from([BotCapability::DiscordFaq]),
        ),
    ]))
}

fn state(service: ApiService<RecordingKernel>) -> Arc<HttpState<RecordingKernel>> {
    Arc::new(HttpState {
        service,
        slots: Arc::new(Semaphore::new(64)),
    })
}

async fn send(
    state: &Arc<HttpState<RecordingKernel>>,
    route: Route,
    token: &str,
    body: String,
    headers: &[(&str, &str)],
) -> (ApiResponse, String) {
    let mut request = Request::builder()
        .method("POST")
        .uri(route.path())
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json");
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let trace = Arc::new(RequestTrace::new(route.path()));
    let response = dispatch_response(
        state.clone(),
        request.body(axum::body::Body::from(body)).unwrap(),
        route,
        trace.clone(),
    )
    .await;
    let mut log = Vec::new();
    trace.write(&response, &mut log).unwrap();
    (response, String::from_utf8(log).unwrap())
}

fn discord_task(request_id: &str, capability: &str, question: &str, history: &[&str]) -> Value {
    json!({
        "contract_version": "brain.bot_tasks.v1",
        "request_id": request_id,
        "identity": {"kind": "discord", "guild_id": 7, "channel_id": 10, "user_id": 42},
        "task": {"kind": "discord_answer", "capability": capability, "question": question, "user_questions": history}
    })
}

fn twitch_task(request_id: &str, prompt: &str, temperature: f64, max_tokens: u32) -> Value {
    json!({
        "contract_version": "brain.bot_tasks.v1",
        "request_id": request_id,
        "identity": {"kind": "twitch", "broadcaster_id": "123456"},
        "task": {"kind": "twitch_title", "purpose": "title", "prompt": prompt, "temperature": temperature, "max_tokens": max_tokens}
    })
}

fn status_task(capability: &str) -> String {
    json!({
        "contract_version": "brain.bot_tasks.v1",
        "request_id": format!("status-{capability}"),
        "identity": if capability == "twitch_title" {
            json!({"kind": "twitch", "broadcaster_id": "123456"})
        } else {
            json!({"kind": "discord", "channel_id": 10, "user_id": 42})
        },
        "task": {"kind": "capability_status", "capability": capability}
    })
    .to_string()
}

fn parsed(response: &ApiResponse) -> BotTaskResponse {
    assert_eq!(response.status, 200, "{}", response.body);
    serde_json::from_str(&response.body).unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn discord_answer_task_reuses_the_header_path_exactly() {
    let kernel = RecordingKernel::default();
    let state = state(service(kernel.clone()));
    for (capability, history) in [("concierge", vec![]), ("faq", vec!["Wie geht Coaching?"])] {
        let header_id = format!("header-{capability}");
        let task_id = format!("task-{capability}");
        let query = json!({
            "request_id": header_id,
            "conversation_id": header_id,
            "text": "Wo finde ich die Serverregeln?",
            "requested_scopes": ["bot.public"],
        });
        let body = if history.is_empty() {
            query
        } else {
            json!({"query": query, "user_questions": history})
        };
        let task_header = json!({"capability": capability, "channel_id": 10}).to_string();
        let (header, _) = send(
            &state,
            Route::Answer,
            "bot-token",
            body.to_string(),
            &[
                ("x-discord-user-id", "42"),
                ("x-discord-read-access", "disabled"),
                ("x-discord-answer-task", &task_header),
            ],
        )
        .await;
        assert_eq!(header.status, 200, "{}", header.body);
        let (task, log) = send(
            &state,
            Route::Task,
            "bot-token",
            discord_task(
                &task_id,
                capability,
                "Wo finde ich die Serverregeln?",
                &history,
            )
            .to_string(),
            &[],
        )
        .await;
        let BotTaskResult::DiscordAnswer { answer } = parsed(&task).result else {
            panic!("unexpected task result");
        };
        let header_answer: PublicAnswerResponse = serde_json::from_str(&header.body).unwrap();
        assert_eq!(answer.request_id, task_id);
        assert_eq!(answer.status, header_answer.status);
        assert_eq!(answer.text, header_answer.text);
        let seen = kernel.0.lock().unwrap();
        let (header_query, header_principal, header_conversation, header_discord) =
            &seen[seen.len() - 2];
        let (task_query, task_principal, task_conversation, task_discord) = &seen[seen.len() - 1];
        let mut normalized = task_query.clone();
        normalized.request_id = header_query.request_id.clone();
        normalized.conversation_id = header_query.conversation_id.clone();
        assert_eq!(&normalized, header_query);
        assert_eq!(task_principal, header_principal);
        assert_eq!(task_conversation, &task_id);
        assert_eq!(header_conversation, &header_id);
        assert_eq!(task_discord, header_discord);
        assert!(task_discord.is_none());
        assert_eq!(
            serde_json::to_value(&task_query.answer_context).unwrap()["purpose"],
            json!(format!("bot_task:{capability}"))
        );
        let event: Value = serde_json::from_str(log.trim()).unwrap();
        assert_eq!(event["event"], "bot_task");
        assert_eq!(event["kind"], "discord_answer");
        assert_eq!(event["outcome"], "insufficient_evidence");
        assert!(!log.contains("Serverregeln") && !log.contains("Coaching"));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn rights_identity_and_bounds_fail_with_typed_statuses() {
    let kernel = RecordingKernel::default();
    let state = state(service(kernel.clone()));
    let mut wrong_identity = discord_task("wrong-identity", "faq", "Frage", &[]);
    wrong_identity["identity"] = json!({"kind": "twitch", "broadcaster_id": "1"});
    let mut unknown_field = twitch_task("unknown", "Titel", 0.5, 32);
    unknown_field["task"]["model"] = json!("override");
    for (token, body, status) in [
        ("bot-token", wrong_identity, 400),
        ("bot-token", discord_task("blank", "faq", " \n", &[]), 400),
        (
            "bot-token",
            discord_task("long", "faq", &"ä".repeat(4001), &[]),
            400,
        ),
        ("twitch-token", unknown_field, 400),
        (
            "twitch-token",
            twitch_task("oversized", &"x".repeat(32_001), 0.5, 32),
            400,
        ),
        ("twitch-token", twitch_task("hot", "Titel", 2.5, 32), 400),
        ("twitch-token", twitch_task("cold", "Titel", -0.1, 32), 400),
        ("twitch-token", twitch_task("zero", "Titel", 0.5, 0), 400),
        (
            "other-token",
            discord_task("other", "concierge", "Frage", &[]),
            403,
        ),
        (
            "twitch-token",
            discord_task("platform", "faq", "Frage", &[]),
            403,
        ),
        (
            "bot-token",
            twitch_task("discord-title", "Titel", 0.5, 32),
            403,
        ),
        (
            "second-token",
            discord_task("second", "faq", "Frage", &[]),
            401,
        ),
        (
            "unknown-token",
            discord_task("unknown", "faq", "Frage", &[]),
            401,
        ),
    ] {
        let (response, log) = send(&state, Route::Task, token, body.to_string(), &[]).await;
        assert_eq!(response.status, status, "{token} {body}");
        assert!(!log.contains("Titel") && !log.contains("Frage"));
    }
    let (response, _) = send(&state, Route::Task, "bot-token", "{".into(), &[]).await;
    assert_eq!(response.status, 400);
    let (response, _) = send(
        &state,
        Route::Task,
        "bot-token",
        json!({"pad": "x".repeat(70_000)}).to_string(),
        &[],
    )
    .await;
    assert_eq!(response.status, 413);
    let (response, _) = send(
        &state,
        Route::Task,
        "bot-token",
        discord_task("forged", "faq", "Frage", &[]).to_string(),
        &[("x-discord-read-access", "disabled")],
    )
    .await;
    assert_eq!(response.status, 400);
    assert!(kernel.0.lock().unwrap().is_empty());
    for (token, capability, available) in [
        ("bot-token", "discord_faq", true),
        ("other-token", "discord_concierge", false),
        ("twitch-token", "twitch_title", false),
        ("twitch-token", "discord_faq", false),
    ] {
        let (response, _) = send(&state, Route::Task, token, status_task(capability), &[]).await;
        let BotTaskResult::CapabilityStatus { availability, .. } = parsed(&response).result else {
            panic!("unexpected status result");
        };
        assert_eq!(
            availability == CapabilityAvailability::Available,
            available,
            "{token} {capability}"
        );
    }
    let (response, _) = send(
        &state,
        Route::Task,
        "twitch-token",
        twitch_task("no-provider", "Titel", 0.5, 32).to_string(),
        &[],
    )
    .await;
    assert_eq!(
        parsed(&response).result,
        BotTaskResult::Failed {
            capability: BotCapability::TwitchTitle,
            failure: TaskFailure::ProviderUnavailable,
        }
    );
}

fn protected_dir() -> tempfile::TempDir {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|path| {
            path.ancestors().all(|ancestor| {
                std::fs::symlink_metadata(ancestor).is_ok_and(|meta| {
                    meta.is_dir() && [0, 1000].contains(&meta.uid()) && meta.mode() & 0o022 == 0
                })
            })
        })
        .unwrap();
    let dir = tempfile::Builder::new()
        .prefix(".brain-task-test-")
        .tempdir_in(base)
        .unwrap();
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    dir
}

fn write_selection(path: &Path, model: &str) {
    let checked = (chrono::Utc::now() - chrono::Duration::minutes(1)).to_rfc3339();
    std::fs::write(
        path,
        json!({"schema_version":1,"provider":"fireworks","model":model,"release":null,"probed_at":checked,"checked_at":checked}).to_string(),
    )
    .unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644)).unwrap();
}

fn read_body(stream: &mut std::net::TcpStream) -> Value {
    let mut bytes = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        let count = stream.read(&mut buffer).unwrap();
        assert!(count > 0);
        bytes.extend_from_slice(&buffer[..count]);
        let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") else {
            continue;
        };
        let head = String::from_utf8_lossy(&bytes[..end]).to_ascii_lowercase();
        assert!(head.starts_with("post /inference/v1/chat/completions "));
        assert!(head.contains("authorization: bearer fixture-title-key"));
        let length: usize = head
            .lines()
            .find_map(|line| line.strip_prefix("content-length:"))
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        while bytes.len() < end + 4 + length {
            let count = stream.read(&mut buffer).unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&buffer[..count]);
        }
        return serde_json::from_slice(&bytes[end + 4..end + 4 + length]).unwrap();
    }
}

fn upstream(responses: Vec<(u16, String)>) -> (String, std::thread::JoinHandle<Vec<Value>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}/inference/v1", listener.local_addr().unwrap());
    let handle = std::thread::spawn(move || {
        let mut bodies = Vec::new();
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            bodies.push(read_body(&mut stream));
            let reason = if status == 200 {
                "OK"
            } else {
                "Too Many Requests"
            };
            let reply = format!(
                "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nRetry-After: 0\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).unwrap();
        }
        bodies
    });
    (base, handle)
}

fn title_service(base: &str, selection: PathBuf, retries: u32) -> ApiService<RecordingKernel> {
    let provider = TextGenerationProvider::new(GenerationEndpoint::new(
        "fixture-title-key",
        base,
        Duration::from_secs(5),
        retries,
        64 * 1024,
    ))
    .unwrap();
    service(RecordingKernel::default()).with_twitch_title(Arc::new(TwitchTitleGenerator::new(
        provider, selection, 256,
    )))
}

const PROMPT: &str = "PRIVATE_STREAMER_PROMPT_äöü Schreibe einen Titel";
const MODEL: &str = "accounts/fireworks/models/deepseek-v4p1-flash";

#[tokio::test(flavor = "multi_thread")]
async fn twitch_title_uses_the_selected_model_without_reasoning_and_keeps_prompts_out_of_logs() {
    let dir = protected_dir();
    let selection = dir.path().join("selection.json");
    write_selection(&selection, MODEL);
    let completion = json!({
        "model": MODEL,
        "choices": [{"message": {"role": "assistant", "content": " Neuer Titel ", "reasoning_content": "PRIVATE_REASONING"}}],
        "usage": {"prompt_tokens": 12, "completion_tokens": 5}
    })
    .to_string();
    let (base, upstream) = upstream(vec![(200, completion)]);
    let state = state(title_service(&base, selection, 2));
    let (response, log) = send(
        &state,
        Route::Task,
        "twitch-token",
        twitch_task("title-1", PROMPT, 0.7, 64).to_string(),
        &[],
    )
    .await;
    assert!(!response.body.contains("PRIVATE_REASONING"));
    let BotTaskResult::TwitchTitle { text, model, usage } = parsed(&response).result else {
        panic!("unexpected title result");
    };
    assert_eq!(text, "Neuer Titel");
    assert_eq!(model, MODEL);
    assert_eq!(
        (usage.input_tokens, usage.output_tokens),
        (Some(12), Some(5))
    );
    let bodies = upstream.join().unwrap();
    assert_eq!(bodies.len(), 1);
    assert_eq!(bodies[0]["model"], MODEL);
    assert_eq!(bodies[0]["reasoning_effort"], "none");
    assert_eq!(bodies[0]["stream"], false);
    assert_eq!(bodies[0]["max_tokens"], 64);
    assert_eq!(
        bodies[0]["messages"],
        json!([{"role": "user", "content": PROMPT}])
    );
    let event: Value = serde_json::from_str(log.trim()).unwrap();
    assert_eq!(event["kind"], "twitch_title");
    assert_eq!(event["purpose"], "title");
    assert_eq!(event["model"], MODEL);
    assert_eq!(event["input_tokens"], 12);
    assert_eq!(event["output_tokens"], 5);
    assert_eq!(event["outcome"], "succeeded");
    assert!(!log.contains("PRIVATE_STREAMER_PROMPT") && !log.contains("Neuer Titel"));
    let (response, _) = send(
        &state,
        Route::Task,
        "twitch-token",
        twitch_task("too-many-tokens", PROMPT, 0.7, 257).to_string(),
        &[],
    )
    .await;
    assert_eq!(response.status, 400);
}

#[tokio::test(flavor = "multi_thread")]
async fn twitch_title_failures_are_typed_and_never_fall_back() {
    let dir = protected_dir();
    let selection = dir.path().join("selection.json");
    let rate_limited = json!({"error": {"message": "slow down"}}).to_string();
    let (base, upstream) = upstream(vec![(429, rate_limited.clone()), (429, rate_limited)]);
    let state = state(title_service(&base, selection.clone(), 1));
    let (response, _) = send(
        &state,
        Route::Task,
        "twitch-token",
        status_task("twitch_title"),
        &[],
    )
    .await;
    assert_eq!(
        parsed(&response).result,
        BotTaskResult::CapabilityStatus {
            capability: BotCapability::TwitchTitle,
            availability: CapabilityAvailability::Unavailable,
        }
    );
    let (response, log) = send(
        &state,
        Route::Task,
        "twitch-token",
        twitch_task("missing-selection", PROMPT, 0.7, 64).to_string(),
        &[],
    )
    .await;
    assert_eq!(
        parsed(&response).result,
        BotTaskResult::Failed {
            capability: BotCapability::TwitchTitle,
            failure: TaskFailure::ProviderUnavailable,
        }
    );
    assert!(!log.contains("PRIVATE_STREAMER_PROMPT"));
    write_selection(&selection, MODEL);
    let (response, _) = send(
        &state,
        Route::Task,
        "twitch-token",
        status_task("twitch_title"),
        &[],
    )
    .await;
    assert_eq!(
        parsed(&response).result,
        BotTaskResult::CapabilityStatus {
            capability: BotCapability::TwitchTitle,
            availability: CapabilityAvailability::Available,
        }
    );
    let (response, log) = send(
        &state,
        Route::Task,
        "twitch-token",
        twitch_task("rate-limited", PROMPT, 0.7, 64).to_string(),
        &[],
    )
    .await;
    assert_eq!(
        parsed(&response).result,
        BotTaskResult::Failed {
            capability: BotCapability::TwitchTitle,
            failure: TaskFailure::ProviderRateLimited,
        }
    );
    let event: Value = serde_json::from_str(log.trim()).unwrap();
    assert_eq!(event["outcome"], "provider_rate_limited");
    assert_eq!(event["model"], Value::Null);
    assert!(!log.contains("PRIVATE_STREAMER_PROMPT"));
    let bodies = upstream.join().unwrap();
    assert_eq!(bodies.len(), 2);
    assert!(bodies.iter().all(|body| body["model"] == MODEL));
}
