use brain_api::{router, ApiService};
use brain_contracts::bot_conversation::GeneratedReply;
use brain_contracts::bot_tasks::{
    BotCapability, ConversationPort, ConversationPrompt, TaskFailure, TaskUsage, BOT_TASK_VERSION,
};
use brain_contracts::{
    AnswerStatus, AuthorizedContext, Budget, Evidence, EvidenceKind, Query, RequestDeadline,
    SourceVisibility,
};
use brain_kernel::AnswerKernelPort;
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

type SeenReads = Arc<Mutex<Vec<Option<BTreeSet<u64>>>>>;

struct ScopeKernel {
    seen: SeenReads,
    status: AnswerStatus,
}

impl AnswerKernelPort for ScopeKernel {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> brain_contracts::AnswerResponse {
        let readable = context
            .discord
            .as_ref()
            .filter(|discord| discord.allow_discord_reads)
            .and_then(|discord| discord.readable_channels.clone());
        self.seen.lock().unwrap().extend([readable]);
        brain_contracts::AnswerResponse {
            contract_version: brain_contracts::CONTRACT_VERSION.into(),
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release.clone(),
            status: self.status,
            text: "Antwort aus dem Kern.".into(),
            citations: vec![live_evidence()],
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

fn live_evidence() -> Evidence {
    Evidence {
        evidence_id: "discord-live-7".into(),
        source_id: "discord_live".into(),
        logical_id: "7".into(),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: "Patchnotes aus dem Kanal".into(),
        citation: "discord:7".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: None,
        patch: None,
    }
}

struct FixedPort(&'static str);

impl ConversationPort for FixedPort {
    fn available(&self) -> bool {
        true
    }

    fn generate(
        &self,
        _: &ConversationPrompt,
        _: &RequestDeadline,
    ) -> Result<GeneratedReply, TaskFailure> {
        Ok(GeneratedReply {
            text: self.0.into(),
            model: "fixture".into(),
            usage: TaskUsage::default(),
        })
    }
}

fn service(status: AnswerStatus, seen: SeenReads, reply: &'static str) -> ApiService<ScopeKernel> {
    ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture",
            "bot",
            "discord",
            BTreeSet::from(["bot.public".into()]),
            BTreeSet::from(["public".into()]),
        )])),
        ScopeKernel { seen, status },
        "context-release",
        5000,
        Budget::default(),
    )
    .with_discord_consumers(BTreeSet::from([("bot".into(), "discord".into())]))
    .with_bot_task_consumers(BTreeMap::from([(
        ("bot".into(), "discord".into()),
        BTreeSet::from([
            BotCapability::DiscordConcierge,
            BotCapability::DiscordFaq,
            BotCapability::DiscordInviteLounge,
        ]),
    )]))
    .with_conversation(Arc::new(FixedPort(reply)))
}

async fn post(service: ApiService<ScopeKernel>, guild: Option<u64>, task: Value) -> (u16, Value) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1/task", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router(service)).await.unwrap() });
    let mut identity = json!({"kind": "discord", "channel_id": 5, "user_id": 42});
    if let Some(guild) = guild {
        identity["guild_id"] = json!(guild);
    }
    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth("fixture")
        .json(&json!({
            "contract_version": BOT_TASK_VERSION,
            "request_id": "conversation-fixture",
            "identity": identity,
            "task": task,
        }))
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    let body = response.json().await.unwrap_or(Value::Null);
    server.abort();
    (status, body)
}

fn concierge(readable: &[u64]) -> Value {
    json!({
        "kind": "conversation",
        "capability": "discord_concierge",
        "question": "Was steht in den Patchnotes?",
        "readable_channel_ids": readable,
    })
}

#[tokio::test]
async fn direct_message_concierge_reads_only_the_sent_channels() {
    let seen = SeenReads::default();
    let (status, body) = post(
        service(AnswerStatus::Answered, seen.clone(), "<silent>"),
        None,
        concierge(&[7, 9]),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["result"]["reply"]["text"], "Antwort aus dem Kern.");
    assert_eq!(
        *seen.lock().unwrap(),
        vec![Some(BTreeSet::from([7_u64, 9]))]
    );
}

#[tokio::test]
async fn concierge_without_a_scope_never_reads_live_channels() {
    let seen = SeenReads::default();
    let (status, body) = post(
        service(AnswerStatus::Answered, seen.clone(), "<silent>"),
        None,
        concierge(&[]),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(*seen.lock().unwrap(), vec![None]);
}

#[tokio::test]
async fn scoped_reads_from_a_guild_channel_are_rejected() {
    let seen = SeenReads::default();
    let (status, _) = post(
        service(AnswerStatus::Answered, seen.clone(), "<silent>"),
        Some(11),
        concierge(&[7]),
    )
    .await;
    assert_eq!(status, 400);
    assert!(seen.lock().unwrap().is_empty());
}

#[tokio::test]
async fn missing_evidence_stays_silent_when_the_model_declines() {
    let (status, body) = post(
        service(
            AnswerStatus::InsufficientEvidence,
            SeenReads::default(),
            "<silent>",
        ),
        None,
        concierge(&[7]),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["result"]["reply"]["kind"], "silent");
    assert_eq!(body["result"]["reply"]["reason"], "insufficient_evidence");
}

#[tokio::test]
async fn smalltalk_is_answered_by_the_concierge_persona() {
    let (status, body) = post(
        service(
            AnswerStatus::InsufficientEvidence,
            SeenReads::default(),
            "Mir geht es gut, danke!",
        ),
        None,
        concierge(&[]),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["result"]["reply"]["text"], "Mir geht es gut, danke!");
}

#[tokio::test]
async fn fallback_rejects_invented_links() {
    let (status, body) = post(
        service(
            AnswerStatus::InsufficientEvidence,
            SeenReads::default(),
            "Schau auf https://example.invalid/x",
        ),
        None,
        concierge(&[]),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["result"]["reply"]["reason"], "output_rejected");
}

#[tokio::test]
async fn capability_status_lists_the_consumer_platform() {
    let (status, body) = post(
        service(AnswerStatus::Answered, SeenReads::default(), "<silent>"),
        None,
        json!({"kind": "capability_status", "capability": "discord_concierge"}),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    let states = body["result"]["capabilities"].as_array().unwrap();
    let state = |name: &str| {
        states
            .iter()
            .find(|state| state["capability"] == name)
            .map(|state| state["availability"].clone())
    };
    assert_eq!(state("discord_invite_lounge"), Some(json!("available")));
    assert_eq!(state("discord_voice_hint"), Some(json!("unavailable")));
    assert_eq!(state("twitch_chat"), None);
    assert_eq!(states.len(), 5);
}
