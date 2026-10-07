use std::collections::BTreeSet;

pub use brain_contracts::{AnswerProfile, Principal, PublicAnswerResponse, Query};
use brain_contracts::{AnswerStatus, PublicCitation, PUBLIC_API_VERSION};
use serde_json::{json, Value};

#[path = "../src/bot_tasks.rs"]
mod bot_tasks;

use bot_tasks::*;

fn request() -> BotTaskRequest {
    BotTaskRequest {
        contract_version: BOT_TASK_VERSION.into(),
        request_id: "guide-42".into(),
        platform: BotPlatform::Discord,
        audience: BotAudience::PublicDiscord {
            guild_id: 123,
            channel_id: 456,
        },
        task: BotTask::PublicGuide {
            question: "Wie finde ich die Serverregeln?".into(),
        },
    }
}

fn principal() -> Principal {
    Principal {
        actor_id: "discord-guide".into(),
        channel: "discord".into(),
        scopes: BTreeSet::from(["public-guide".into()]),
        provider_egress: BTreeSet::from(["public-guide".into()]),
    }
}

fn consumer() -> RegisteredBotConsumer {
    RegisteredBotConsumer {
        actor_id: "discord-guide".into(),
        principal_channel: "discord".into(),
        platform: BotPlatform::Discord,
        allowed_audiences: BTreeSet::from([request().audience]),
        capabilities: BTreeSet::from([BotCapability::PublicGuideEndAnswer]),
        guide_scope: "public-guide".into(),
    }
}

fn context<'a>(
    principal: &'a Principal,
    consumer: &'a RegisteredBotConsumer,
    request: &'a BotTaskRequest,
) -> TrustedBotTaskContext<'a> {
    TrustedBotTaskContext {
        principal,
        consumer,
        request_id: &request.request_id,
        audience: &request.audience,
        conversation_id: "public-channel-456",
        input_provenance: VerifiedInputProvenance::ApprovedPublic,
        processing: ActualProcessing::RemoteInference,
        request_allowed: true,
        delivery_allowed: true,
        guide_implementation: GuideImplementation::Approved,
    }
}

fn response() -> BotTaskResponse {
    let request = request();
    BotTaskResponse {
        contract_version: BOT_TASK_VERSION.into(),
        request_id: request.request_id.clone(),
        platform: request.platform,
        audience: request.audience,
        result: BotTaskResult::PublicGuide {
            answer: PublicAnswerResponse {
                contract_version: PUBLIC_API_VERSION.into(),
                request_id: request.request_id,
                knowledge_release: "public-release-7".into(),
                status: AnswerStatus::Answered,
                text: "Die Regeln stehen im Regelkanal.".into(),
                citations: vec![PublicCitation {
                    citation_id: "rules-7".into(),
                    label: "Serverregeln".into(),
                }],
            },
        },
    }
}

fn wire_request() -> Value {
    serde_json::to_value(request()).unwrap()
}

#[test]
fn public_guide_wire_roundtrip_preserves_discriminants() {
    let wire = wire_request();
    assert_eq!(wire["task"]["kind"], "public_guide");
    assert_eq!(wire["audience"]["kind"], "public_discord");
    let decoded: BotTaskRequest = serde_json::from_value(wire).unwrap();
    assert_eq!(decoded, request());
    assert_eq!(decoded.validate(), Ok(()));
    let response = response();
    let wire = serde_json::to_value(&response).unwrap();
    assert_eq!(wire["result"]["kind"], "public_guide");
    assert_eq!(
        serde_json::from_value::<BotTaskResponse>(wire).unwrap(),
        response
    );
    assert_eq!(response.validate_for(&decoded), Ok(()));
}

#[test]
fn wire_never_accepts_authority_or_provider_overrides() {
    for field in [
        "principal",
        "consumer",
        "scopes",
        "model",
        "provider",
        "endpoint",
        "address",
        "credential",
        "api_key",
        "processing",
        "input_provenance",
        "request_allowed",
    ] {
        let mut wire = wire_request();
        wire[field] = json!(true);
        assert!(
            serde_json::from_value::<BotTaskRequest>(wire).is_err(),
            "{field}"
        );
    }
}

#[test]
fn guide_input_never_accepts_history_profiles_or_provider_overrides() {
    for field in [
        "history",
        "profile",
        "user_id",
        "dm",
        "context",
        "query",
        "domain",
        "model",
        "provider",
        "endpoint",
        "credential",
        "requested_scopes",
    ] {
        let mut wire = wire_request();
        wire["task"][field] = json!(true);
        assert!(
            serde_json::from_value::<BotTaskRequest>(wire).is_err(),
            "{field}"
        );
    }
}

#[test]
fn nested_audience_and_result_reject_unknown_fields() {
    let mut wire = wire_request();
    wire["audience"]["user_id"] = json!(123);
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    for target in ["result", "answer"] {
        let mut wire = serde_json::to_value(response()).unwrap();
        if target == "result" {
            wire["result"]["model"] = json!(true);
        } else {
            wire["result"]["answer"]["history"] = json!(true);
        }
        assert!(serde_json::from_value::<BotTaskResponse>(wire).is_err());
    }
}

#[test]
fn unknown_missing_and_reduced_title_tasks_are_rejected() {
    for kind in [
        "twitch_title",
        "personal_help",
        "public_guide_draft",
        "unknown",
    ] {
        let mut wire = wire_request();
        wire["task"]["kind"] = json!(kind);
        assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    }
    let mut wire = wire_request();
    wire["task"].as_object_mut().unwrap().remove("kind");
    assert!(serde_json::from_value::<BotTaskRequest>(wire).is_err());
    let mut wire = serde_json::to_value(response()).unwrap();
    wire["result"]["kind"] = json!("twitch_title");
    assert!(serde_json::from_value::<BotTaskResponse>(wire).is_err());
}

#[test]
fn versions_and_request_ids_fail_closed() {
    let mut request = request();
    request.contract_version = "brain.bot_tasks.v2".into();
    assert_eq!(request.validate(), Err(BotTaskError::VersionMismatch));
    request.contract_version = BOT_TASK_VERSION.into();
    for id in ["", " ", " guide-42", "guide-42 ", "guide\n42"] {
        request.request_id = id.into();
        assert_eq!(request.validate(), Err(BotTaskError::InvalidRequestId));
    }
    request.request_id = "x".repeat(513);
    assert_eq!(request.validate(), Err(BotTaskError::InvalidRequestId));
}

#[test]
fn platforms_and_stable_audiences_must_match() {
    let mut request = request();
    request.platform = BotPlatform::Twitch;
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    request.platform = BotPlatform::Discord;
    request.audience = BotAudience::PublicDiscord {
        guild_id: 0,
        channel_id: 456,
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    request.audience = BotAudience::PublicDiscord {
        guild_id: 123,
        channel_id: 0,
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    request.platform = BotPlatform::Twitch;
    request.task = BotTask::CapabilityStatus {
        capability: BotCapability::TwitchTitleDraft,
    };
    for id in ["", "0", "0123", "name", "18446744073709551616"] {
        request.audience = BotAudience::PublicTwitch {
            channel_id: id.into(),
        };
        assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
    }
    request.audience = BotAudience::PublicTwitch {
        channel_id: "123".into(),
    };
    assert_eq!(request.validate(), Ok(()));
    request.task = BotTask::PublicGuide {
        question: "Regeln?".into(),
    };
    assert_eq!(request.validate(), Err(BotTaskError::InvalidBinding));
}

#[test]
fn empty_and_oversized_public_questions_are_rejected() {
    let mut request = request();
    for question in ["".into(), " \n".into(), "x".repeat(32_769)] {
        request.task = BotTask::PublicGuide { question };
        assert_eq!(request.validate(), Err(BotTaskError::InvalidQuestion));
    }
}

#[test]
fn trusted_binding_produces_existing_query_without_control_protocol() {
    let request = request();
    let principal = principal();
    let consumer = consumer();
    let authorized =
        authorize_public_guide(&request, &context(&principal, &consumer, &request)).unwrap();
    let BotTask::PublicGuide { question } = &request.task else {
        panic!()
    };
    assert_eq!(authorized.query().text, *question);
    assert_eq!(authorized.query().request_id, request.request_id);
    assert_eq!(authorized.query().conversation_id, "public-channel-456");
    assert_eq!(
        authorized.query().requested_scopes,
        BTreeSet::from([consumer.guide_scope])
    );
    assert_eq!(authorized.query().profile, AnswerProfile::Explain);
    assert!(authorized.query().domain.is_none());
    assert!(authorized.query().patch.is_none());
    assert!(authorized.query().mode.is_none());
    assert_eq!(authorized.audience(), &request.audience);
    assert_eq!(authorized.processing(), ActualProcessing::RemoteInference);
    assert_eq!(
        request.task.capability(),
        BotCapability::PublicGuideEndAnswer
    );
}

#[test]
fn public_label_never_proves_provenance_even_for_local_processing() {
    let request = request();
    let principal = principal();
    let consumer = consumer();
    for processing in [
        ActualProcessing::LocalInference,
        ActualProcessing::RemoteInference,
        ActualProcessing::LoopbackRemoteProxy,
    ] {
        for provenance in [
            VerifiedInputProvenance::Private,
            VerifiedInputProvenance::Unknown,
        ] {
            let mut context = context(&principal, &consumer, &request);
            context.processing = processing;
            context.input_provenance = provenance;
            assert_eq!(
                authorize_public_guide(&request, &context).unwrap_err(),
                BotTaskError::UnapprovedInput
            );
        }
    }
}

#[test]
fn loopback_proxy_requires_remote_egress_permission() {
    let request = request();
    let mut principal = principal();
    principal.provider_egress.clear();
    let consumer = consumer();
    for processing in [
        ActualProcessing::RemoteInference,
        ActualProcessing::LoopbackRemoteProxy,
    ] {
        let mut context = context(&principal, &consumer, &request);
        context.processing = processing;
        assert_eq!(
            authorize_public_guide(&request, &context).unwrap_err(),
            BotTaskError::PermissionDenied
        );
    }
    let mut context = context(&principal, &consumer, &request);
    context.processing = ActualProcessing::LocalInference;
    assert!(authorize_public_guide(&request, &context).is_ok());
    context.processing = ActualProcessing::Unknown;
    assert_eq!(
        authorize_public_guide(&request, &context).unwrap_err(),
        BotTaskError::ProcessingUnverified
    );
}

#[test]
fn mismatched_principal_registration_scope_and_capability_are_denied() {
    let request = request();
    for case in 0..8 {
        let mut principal = principal();
        let mut consumer = consumer();
        match case {
            0 => principal.actor_id = "other-service".into(),
            1 => principal.channel = "twitch".into(),
            2 => principal.scopes.clear(),
            3 => consumer.allowed_audiences.clear(),
            4 => consumer.capabilities.clear(),
            5 => consumer.platform = BotPlatform::Twitch,
            6 => consumer.guide_scope = " ".into(),
            7 => consumer.actor_id.clear(),
            _ => unreachable!(),
        }
        assert_eq!(
            authorize_public_guide(&request, &context(&principal, &consumer, &request))
                .unwrap_err(),
            BotTaskError::PermissionDenied
        );
    }
}

#[test]
fn request_and_delivery_rights_are_bound_and_rechecked() {
    let request = request();
    let principal = principal();
    let consumer = consumer();
    for case in 0..5 {
        let other_audience = BotAudience::PublicDiscord {
            guild_id: 123,
            channel_id: 789,
        };
        let mut context = context(&principal, &consumer, &request);
        match case {
            0 => context.request_allowed = false,
            1 => context.delivery_allowed = false,
            2 => context.request_id = "another-request",
            3 => context.audience = &other_audience,
            4 => context.conversation_id = "",
            _ => unreachable!(),
        }
        let expected = if case == 4 {
            BotTaskError::InvalidBinding
        } else {
            BotTaskError::PermissionDenied
        };
        assert_eq!(
            authorize_public_guide(&request, &context).unwrap_err(),
            expected
        );
    }
    let mut context = context(&principal, &consumer, &request);
    assert!(authorize_public_guide(&request, &context).is_ok());
    context.delivery_allowed = false;
    assert_eq!(
        authorize_public_guide(&request, &context).unwrap_err(),
        BotTaskError::PermissionDenied
    );
}

#[test]
fn unapproved_implementation_is_explicitly_unavailable() {
    let request = request();
    let principal = principal();
    let consumer = consumer();
    let mut context = context(&principal, &consumer, &request);
    context.guide_implementation = GuideImplementation::default();
    assert_eq!(
        authorize_public_guide(&request, &context).unwrap_err(),
        BotTaskError::Unavailable
    );
    assert_eq!(
        capability_availability(
            BotCapability::PublicGuideEndAnswer,
            BotPlatform::Discord,
            GuideImplementation::default()
        ),
        CapabilityAvailability::Unavailable
    );
    assert_eq!(
        capability_availability(
            BotCapability::PublicGuideEndAnswer,
            BotPlatform::Discord,
            GuideImplementation::Approved
        ),
        CapabilityAvailability::Available
    );
    for capability in [BotCapability::TwitchTitleDraft, BotCapability::PersonalHelp] {
        assert_eq!(
            capability_availability(
                capability,
                BotPlatform::Discord,
                GuideImplementation::Approved
            ),
            CapabilityAvailability::Unavailable
        );
    }
}

#[test]
fn capability_availability_requires_discord_and_approved_guide() {
    for platform in [BotPlatform::Discord, BotPlatform::Twitch] {
        for guide in [
            GuideImplementation::NotApproved,
            GuideImplementation::Approved,
        ] {
            for capability in [
                BotCapability::PublicGuideEndAnswer,
                BotCapability::TwitchTitleDraft,
                BotCapability::PersonalHelp,
            ] {
                let expected = match (platform, guide, capability) {
                    (
                        BotPlatform::Discord,
                        GuideImplementation::Approved,
                        BotCapability::PublicGuideEndAnswer,
                    ) => CapabilityAvailability::Available,
                    _ => CapabilityAvailability::Unavailable,
                };
                assert_eq!(
                    capability_availability(capability, platform, guide),
                    expected,
                    "{platform:?} {guide:?} {capability:?}"
                );
            }
        }
    }
}

#[test]
fn capability_status_responses_reject_available_outside_discord_guide() {
    for platform in [BotPlatform::Discord, BotPlatform::Twitch] {
        for capability in [
            BotCapability::PublicGuideEndAnswer,
            BotCapability::TwitchTitleDraft,
            BotCapability::PersonalHelp,
        ] {
            let mut request = request();
            request.platform = platform;
            request.audience = match platform {
                BotPlatform::Discord => request.audience,
                BotPlatform::Twitch => BotAudience::PublicTwitch {
                    channel_id: "123".into(),
                },
            };
            request.task = BotTask::CapabilityStatus { capability };
            assert_eq!(request.validate(), Ok(()));
            for availability in [
                CapabilityAvailability::Available,
                CapabilityAvailability::Unavailable,
            ] {
                let response = BotTaskResponse {
                    contract_version: request.contract_version.clone(),
                    request_id: request.request_id.clone(),
                    platform,
                    audience: request.audience.clone(),
                    result: BotTaskResult::CapabilityStatus {
                        capability,
                        availability,
                    },
                };
                let wire = serde_json::to_value(&response).unwrap();
                let response: BotTaskResponse = serde_json::from_value(wire).unwrap();
                let expected = if availability == CapabilityAvailability::Unavailable
                    || (platform == BotPlatform::Discord
                        && capability == BotCapability::PublicGuideEndAnswer)
                {
                    Ok(())
                } else {
                    Err(BotTaskError::ResultMismatch)
                };
                assert_eq!(
                    response.validate_for(&request),
                    expected,
                    "{platform:?} {capability:?} {availability:?}"
                );
            }
        }
    }
}

#[test]
fn result_envelope_and_nested_answer_are_strictly_bound() {
    let request = request();
    for case in 0..7 {
        let mut response = response();
        let expected = match case {
            0 => {
                response.contract_version = "brain.bot_tasks.v2".into();
                BotTaskError::VersionMismatch
            }
            1 => {
                response.request_id = "other-request".into();
                BotTaskError::InvalidRequestId
            }
            2 => {
                response.platform = BotPlatform::Twitch;
                BotTaskError::InvalidBinding
            }
            3 => {
                response.audience = BotAudience::PublicDiscord {
                    guild_id: 123,
                    channel_id: 789,
                };
                BotTaskError::InvalidBinding
            }
            4..=6 => {
                let BotTaskResult::PublicGuide { answer } = &mut response.result else {
                    panic!()
                };
                match case {
                    4 => answer.request_id = "other-request".into(),
                    5 => answer.contract_version = "brain.public.v2".into(),
                    6 => answer.citations.clear(),
                    _ => unreachable!(),
                }
                BotTaskError::InvalidAnswer
            }
            _ => unreachable!(),
        };
        assert_eq!(response.validate_for(&request), Err(expected));
    }
}

#[test]
fn result_kind_cannot_impersonate_success_or_another_capability() {
    let request = request();
    let mut response = response();
    response.result = BotTaskResult::CapabilityStatus {
        capability: BotCapability::PublicGuideEndAnswer,
        availability: CapabilityAvailability::Available,
    };
    assert_eq!(
        response.validate_for(&request),
        Err(BotTaskError::ResultMismatch)
    );
    response.result = BotTaskResult::Unavailable {
        capability: BotCapability::TwitchTitleDraft,
        reason: TaskUnavailableReason::ImplementationNotApproved,
    };
    assert_eq!(
        response.validate_for(&request),
        Err(BotTaskError::ResultMismatch)
    );
    for reason in [
        TaskUnavailableReason::ImplementationNotApproved,
        TaskUnavailableReason::ProviderUnavailable,
    ] {
        response.result = BotTaskResult::Unavailable {
            capability: BotCapability::PublicGuideEndAnswer,
            reason,
        };
        assert_eq!(response.validate_for(&request), Ok(()));
    }
}

#[test]
fn reserved_capabilities_have_status_but_no_success_schema() {
    for capability in [BotCapability::TwitchTitleDraft, BotCapability::PersonalHelp] {
        let mut request = request();
        request.task = BotTask::CapabilityStatus { capability };
        assert_eq!(request.task.capability(), capability);
        let answer = match response().result {
            BotTaskResult::PublicGuide { answer } => answer,
            _ => panic!(),
        };
        let mut response = response();
        response.result = BotTaskResult::CapabilityStatus {
            capability,
            availability: CapabilityAvailability::Unavailable,
        };
        assert_eq!(response.validate_for(&request), Ok(()));
        response.result = BotTaskResult::CapabilityStatus {
            capability,
            availability: CapabilityAvailability::Available,
        };
        assert_eq!(
            response.validate_for(&request),
            Err(BotTaskError::ResultMismatch)
        );
        response.result = BotTaskResult::PublicGuide { answer };
        assert_eq!(
            response.validate_for(&request),
            Err(BotTaskError::ResultMismatch)
        );
    }
}

#[test]
fn status_request_is_not_a_model_task() {
    let mut request = request();
    request.task = BotTask::CapabilityStatus {
        capability: BotCapability::PublicGuideEndAnswer,
    };
    let principal = principal();
    let consumer = consumer();
    assert_eq!(
        authorize_public_guide(&request, &context(&principal, &consumer, &request)).unwrap_err(),
        BotTaskError::ResultMismatch
    );
}
