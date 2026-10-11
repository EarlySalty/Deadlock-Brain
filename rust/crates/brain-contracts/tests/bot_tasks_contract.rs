use brain_contracts::bot_tasks::*;
use brain_contracts::discord_task::DiscordAnswerCapability;
use brain_contracts::{AnswerStatus, PublicAnswerResponse, PublicCitation, PUBLIC_API_VERSION};
use serde_json::{json, Value};

fn discord_request() -> BotTaskRequest {
    BotTaskRequest {
        contract_version: BOT_TASK_VERSION.into(),
        request_id: "task-42".into(),
        identity: BotIdentity::Discord {
            guild_id: Some(1),
            channel_id: 2,
            user_id: 3,
        },
        task: BotTask::DiscordAnswer {
            capability: DiscordAnswerCapability::Faq,
            question: "Wie finde ich die Serverregeln?".into(),
            user_questions: vec!["Hallo".into()],
        },
    }
}

fn twitch_request() -> BotTaskRequest {
    BotTaskRequest {
        contract_version: BOT_TASK_VERSION.into(),
        request_id: "title-7".into(),
        identity: BotIdentity::Twitch {
            broadcaster_id: "123456".into(),
        },
        task: BotTask::TwitchTitle {
            purpose: TwitchTitlePurpose::Title,
            prompt: "Schreibe einen Titel".into(),
            temperature: 0.7,
            max_tokens: 64,
        },
    }
}

fn answer(request_id: &str) -> PublicAnswerResponse {
    PublicAnswerResponse {
        contract_version: PUBLIC_API_VERSION.into(),
        request_id: request_id.into(),
        knowledge_release: "release".into(),
        status: AnswerStatus::Answered,
        text: "Antwort".into(),
        citations: vec![PublicCitation {
            citation_id: "cite-1".into(),
            label: "Beleg 1".into(),
        }],
        artifacts: Vec::new(),
    }
}

#[test]
fn wire_roundtrip_uses_kind_tags_and_numbers() {
    let wire = json!({
        "contract_version": BOT_TASK_VERSION,
        "request_id": "title-7",
        "identity": {"kind": "twitch", "broadcaster_id": "123456"},
        "task": {"kind": "twitch_title", "purpose": "title", "prompt": "Schreibe einen Titel", "temperature": 0.7, "max_tokens": 64}
    });
    let parsed: BotTaskRequest = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(parsed, twitch_request());
    parsed.validate().unwrap();
    let discord: BotTaskRequest = serde_json::from_value(json!({
        "contract_version": BOT_TASK_VERSION,
        "request_id": "task-42",
        "identity": {"kind": "discord", "guild_id": 1, "channel_id": 2, "user_id": 3},
        "task": {"kind": "discord_answer", "capability": "faq", "question": "Wie finde ich die Serverregeln?", "user_questions": ["Hallo"]}
    }))
    .unwrap();
    assert_eq!(discord, discord_request());
    let encoded = serde_json::to_value(discord_request()).unwrap();
    assert_eq!(encoded["identity"]["kind"], "discord");
    assert_eq!(encoded["task"]["kind"], "discord_answer");
    let status: Value = serde_json::to_value(BotTask::CapabilityStatus {
        capability: BotCapability::TwitchTitle,
    })
    .unwrap();
    assert_eq!(
        status,
        json!({"kind": "capability_status", "capability": "twitch_title"})
    );
}

#[test]
fn unknown_fields_and_kinds_are_rejected() {
    let mut wire = serde_json::to_value(twitch_request()).unwrap();
    wire["task"]["model"] = json!("override");
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    let mut wire = serde_json::to_value(twitch_request()).unwrap();
    wire["identity"]["actor_id"] = json!("admin");
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    let mut wire = serde_json::to_value(twitch_request()).unwrap();
    wire["principal"] = json!({"actor_id": "admin"});
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    let mut wire = serde_json::to_value(twitch_request()).unwrap();
    wire["task"]["kind"] = json!("public_guide");
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
}

#[test]
fn identity_must_match_task_platform() {
    let mut request = discord_request();
    request.identity = BotIdentity::Twitch {
        broadcaster_id: "1".into(),
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    let mut request = twitch_request();
    request.identity = BotIdentity::Discord {
        guild_id: None,
        channel_id: 1,
        user_id: 1,
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    request.task = BotTask::CapabilityStatus {
        capability: BotCapability::TwitchTitle,
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    request.task = BotTask::CapabilityStatus {
        capability: BotCapability::DiscordConcierge,
    };
    request.validate().unwrap();
}

#[test]
fn identities_are_strict() {
    for identity in [
        BotIdentity::Discord {
            guild_id: Some(0),
            channel_id: 1,
            user_id: 1,
        },
        BotIdentity::Discord {
            guild_id: None,
            channel_id: 0,
            user_id: 1,
        },
        BotIdentity::Discord {
            guild_id: None,
            channel_id: 1,
            user_id: 0,
        },
    ] {
        let mut request = discord_request();
        request.identity = identity;
        assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    }
    for broadcaster_id in ["", "0123", "12a", "123456789012345678901", " 12"] {
        let mut request = twitch_request();
        request.identity = BotIdentity::Twitch {
            broadcaster_id: broadcaster_id.into(),
        };
        assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    }
    let mut request = twitch_request();
    request.identity = BotIdentity::Twitch {
        broadcaster_id: "12345678901234567890".into(),
    };
    request.validate().unwrap();
}

#[test]
fn versions_and_request_ids_fail_closed() {
    let mut request = twitch_request();
    request.contract_version = "brain.bot_tasks.v2".into();
    assert_eq!(request.validate(), Err(BotTaskError::VersionMismatch));
    for id in ["", " id", "id\n", &"x".repeat(513)] {
        let mut request = twitch_request();
        request.request_id = id.into();
        assert_eq!(request.validate(), Err(BotTaskError::InvalidRequestId));
    }
}

#[test]
fn discord_questions_keep_existing_limits() {
    let set = |question: String, user_questions: Vec<String>| {
        let mut request = discord_request();
        request.task = BotTask::DiscordAnswer {
            capability: DiscordAnswerCapability::Concierge,
            question,
            user_questions,
        };
        request.validate()
    };
    assert!(set("ä".repeat(4000), vec![]).is_ok());
    assert!(set("ä".repeat(2000), vec!["ö".repeat(4000)]).is_ok());
    for (question, history) in [
        (" ".to_string(), vec![]),
        ("ä".repeat(4001), vec![]),
        ("Frage".into(), vec!["a".into(); 5]),
        ("Frage".into(), vec!["ö".repeat(2001), "ö".repeat(2000)]),
        ("Frage".into(), vec![" ".into()]),
    ] {
        assert_eq!(set(question, history), Err(BotTaskError::InvalidQuestion));
    }
}

#[test]
fn twitch_prompt_and_generation_parameters_are_bounded() {
    let set = |prompt: String, temperature: f32, max_tokens: u32| {
        let mut request = twitch_request();
        request.task = BotTask::TwitchTitle {
            purpose: TwitchTitlePurpose::Insight,
            prompt,
            temperature,
            max_tokens,
        };
        request.validate()
    };
    assert!(set("ä".repeat(32_000), 0.0, 1).is_ok());
    assert!(set("Titel".into(), 2.0, u32::MAX).is_ok());
    assert_eq!(set("\n ".into(), 1.0, 1), Err(BotTaskError::InvalidPrompt));
    assert_eq!(
        set("ä".repeat(32_001), 1.0, 1),
        Err(BotTaskError::InvalidPrompt)
    );
    for (temperature, max_tokens) in [
        (-0.1, 10),
        (2.01, 10),
        (f32::NAN, 10),
        (f32::INFINITY, 10),
        (1.0, 0),
    ] {
        assert_eq!(
            set("Titel".into(), temperature, max_tokens),
            Err(BotTaskError::InvalidGeneration)
        );
    }
}

#[test]
fn responses_are_bound_to_request_and_task_kind() {
    let discord = discord_request();
    let ok = BotTaskResponse::new(
        &discord,
        BotTaskResult::DiscordAnswer {
            answer: answer("task-42"),
        },
    );
    ok.validate_for(&discord).unwrap();
    let mut foreign = ok.clone();
    foreign.request_id = "other".into();
    assert_eq!(
        foreign.validate_for(&discord),
        Err(BotTaskError::InvalidRequestId)
    );
    let mut version = ok.clone();
    version.contract_version = "brain.v1".into();
    assert_eq!(
        version.validate_for(&discord),
        Err(BotTaskError::VersionMismatch)
    );
    let nested = BotTaskResponse::new(
        &discord,
        BotTaskResult::DiscordAnswer {
            answer: answer("other"),
        },
    );
    assert_eq!(
        nested.validate_for(&discord),
        Err(BotTaskError::InvalidAnswer)
    );
    let twitch = twitch_request();
    let title = BotTaskResponse::new(
        &twitch,
        BotTaskResult::TwitchTitle {
            text: "Titel".into(),
            model: "fixture-model".into(),
            usage: TaskUsage {
                input_tokens: Some(1),
                output_tokens: None,
            },
        },
    );
    title.validate_for(&twitch).unwrap();
    assert_eq!(
        title.validate_for(&discord),
        Err(BotTaskError::InvalidRequestId)
    );
    let mut swapped = title.clone();
    swapped.request_id = discord.request_id.clone();
    assert_eq!(
        swapped.validate_for(&discord),
        Err(BotTaskError::ResultMismatch)
    );
    let failed = BotTaskResponse::new(
        &twitch,
        BotTaskResult::Failed {
            capability: BotCapability::TwitchTitle,
            failure: TaskFailure::ProviderRateLimited,
        },
    );
    failed.validate_for(&twitch).unwrap();
    let wrong_failure = BotTaskResponse::new(
        &twitch,
        BotTaskResult::Failed {
            capability: BotCapability::DiscordFaq,
            failure: TaskFailure::Timeout,
        },
    );
    assert_eq!(
        wrong_failure.validate_for(&twitch),
        Err(BotTaskError::ResultMismatch)
    );
    let empty = BotTaskResponse::new(
        &twitch,
        BotTaskResult::TwitchTitle {
            text: " ".into(),
            model: "fixture-model".into(),
            usage: TaskUsage::default(),
        },
    );
    assert_eq!(
        empty.validate_for(&twitch),
        Err(BotTaskError::ResultMismatch)
    );
    let mut status_request = twitch_request();
    status_request.task = BotTask::CapabilityStatus {
        capability: BotCapability::TwitchTitle,
    };
    BotTaskResponse::new(
        &status_request,
        BotTaskResult::CapabilityStatus {
            capability: BotCapability::TwitchTitle,
            availability: CapabilityAvailability::Unavailable,
        },
    )
    .validate_for(&status_request)
    .unwrap();
}

#[test]
fn result_wire_rejects_unknown_fields() {
    let wire = json!({
        "contract_version": BOT_TASK_VERSION,
        "request_id": "title-7",
        "result": {"kind": "twitch_title", "text": "Titel", "model": "m", "usage": {"input_tokens": 3, "output_tokens": null}}
    });
    let parsed: BotTaskResponse = serde_json::from_value(wire.clone()).unwrap();
    parsed.validate_for(&twitch_request()).unwrap();
    let mut extra = wire.clone();
    extra["result"]["reasoning"] = json!("hidden");
    assert!(serde_json::from_value::<BotTaskResponse>(extra).is_err());
    let mut failure = wire;
    failure["result"] =
        json!({"kind": "failed", "capability": "twitch_title", "failure": "approved"});
    assert!(serde_json::from_value::<BotTaskResponse>(failure).is_err());
}
