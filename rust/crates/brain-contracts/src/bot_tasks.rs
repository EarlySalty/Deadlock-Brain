use std::collections::BTreeSet;

use crate::{AnswerProfile, Principal, PublicAnswerResponse, Query};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const BOT_TASK_VERSION: &str = "brain.bot_tasks.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotPlatform {
    Discord,
    Twitch,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotAudience {
    PublicDiscord { guild_id: u64, channel_id: u64 },
    PublicTwitch { channel_id: String },
}

impl BotAudience {
    pub fn platform(&self) -> BotPlatform {
        match self {
            Self::PublicDiscord { .. } => BotPlatform::Discord,
            Self::PublicTwitch { .. } => BotPlatform::Twitch,
        }
    }

    fn validate(&self) -> Result<(), BotTaskError> {
        match self {
            Self::PublicDiscord {
                guild_id,
                channel_id,
            } if *guild_id > 0 && *channel_id > 0 => Ok(()),
            Self::PublicTwitch { channel_id }
                if !channel_id.is_empty()
                    && channel_id.len() <= 20
                    && channel_id.bytes().all(|b| b.is_ascii_digit())
                    && channel_id.parse::<u64>().is_ok_and(|id| id > 0)
                    && !channel_id.starts_with('0') =>
            {
                Ok(())
            }
            _ => Err(BotTaskError::InvalidBinding),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotCapability {
    PublicGuideEndAnswer,
    TwitchTitleDraft,
    PersonalHelp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityAvailability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotTask {
    PublicGuide { question: String },
    CapabilityStatus { capability: BotCapability },
}

impl BotTask {
    pub fn capability(&self) -> BotCapability {
        match self {
            Self::PublicGuide { .. } => BotCapability::PublicGuideEndAnswer,
            Self::CapabilityStatus { capability } => *capability,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotTaskRequest {
    pub contract_version: String,
    pub request_id: String,
    pub platform: BotPlatform,
    pub audience: BotAudience,
    pub task: BotTask,
}

impl BotTaskRequest {
    pub fn validate(&self) -> Result<(), BotTaskError> {
        if self.contract_version != BOT_TASK_VERSION {
            return Err(BotTaskError::VersionMismatch);
        }
        if !stable_id(&self.request_id) {
            return Err(BotTaskError::InvalidRequestId);
        }
        self.audience.validate()?;
        if self.platform != self.audience.platform() {
            return Err(BotTaskError::InvalidBinding);
        }
        if let BotTask::PublicGuide { question } = &self.task {
            if self.platform != BotPlatform::Discord {
                return Err(BotTaskError::InvalidBinding);
            }
            if question.trim().is_empty() || question.len() > 32_768 {
                return Err(BotTaskError::InvalidQuestion);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskUnavailableReason {
    ImplementationNotApproved,
    ProviderUnavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotTaskResult {
    PublicGuide {
        answer: PublicAnswerResponse,
    },
    CapabilityStatus {
        capability: BotCapability,
        availability: CapabilityAvailability,
    },
    Unavailable {
        capability: BotCapability,
        reason: TaskUnavailableReason,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotTaskResponse {
    pub contract_version: String,
    pub request_id: String,
    pub platform: BotPlatform,
    pub audience: BotAudience,
    pub result: BotTaskResult,
}

impl BotTaskResponse {
    pub fn validate_for(&self, request: &BotTaskRequest) -> Result<(), BotTaskError> {
        request.validate()?;
        if self.contract_version != BOT_TASK_VERSION {
            return Err(BotTaskError::VersionMismatch);
        }
        if self.request_id != request.request_id {
            return Err(BotTaskError::InvalidRequestId);
        }
        if self.platform != request.platform || self.audience != request.audience {
            return Err(BotTaskError::InvalidBinding);
        }
        match (&request.task, &self.result) {
            (BotTask::PublicGuide { .. }, BotTaskResult::PublicGuide { answer }) => answer
                .validate(&request.request_id)
                .map_err(|_| BotTaskError::InvalidAnswer),
            (
                BotTask::PublicGuide { .. },
                BotTaskResult::Unavailable {
                    capability: BotCapability::PublicGuideEndAnswer,
                    ..
                },
            ) => Ok(()),
            (
                BotTask::CapabilityStatus {
                    capability: expected,
                },
                BotTaskResult::CapabilityStatus {
                    capability,
                    availability,
                },
            ) if capability == expected
                && ((*capability == BotCapability::PublicGuideEndAnswer
                    && request.platform == BotPlatform::Discord)
                    || *availability == CapabilityAvailability::Unavailable) =>
            {
                Ok(())
            }
            _ => Err(BotTaskError::ResultMismatch),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideImplementation {
    #[default]
    NotApproved,
    Approved,
}

pub fn capability_availability(
    capability: BotCapability,
    platform: BotPlatform,
    guide: GuideImplementation,
) -> CapabilityAvailability {
    match (capability, platform, guide) {
        (
            BotCapability::PublicGuideEndAnswer,
            BotPlatform::Discord,
            GuideImplementation::Approved,
        ) => CapabilityAvailability::Available,
        _ => CapabilityAvailability::Unavailable,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifiedInputProvenance {
    ApprovedPublic,
    Private,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActualProcessing {
    LocalInference,
    RemoteInference,
    LoopbackRemoteProxy,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredBotConsumer {
    pub actor_id: String,
    pub principal_channel: String,
    pub platform: BotPlatform,
    pub allowed_audiences: BTreeSet<BotAudience>,
    pub capabilities: BTreeSet<BotCapability>,
    pub guide_scope: String,
}

#[derive(Debug)]
pub struct TrustedBotTaskContext<'a> {
    pub principal: &'a Principal,
    pub consumer: &'a RegisteredBotConsumer,
    pub request_id: &'a str,
    pub audience: &'a BotAudience,
    pub conversation_id: &'a str,
    pub input_provenance: VerifiedInputProvenance,
    pub processing: ActualProcessing,
    pub request_allowed: bool,
    pub delivery_allowed: bool,
    pub guide_implementation: GuideImplementation,
}

#[derive(Debug)]
pub struct AuthorizedPublicGuide {
    query: Query,
    audience: BotAudience,
    processing: ActualProcessing,
}

impl AuthorizedPublicGuide {
    pub fn query(&self) -> &Query {
        &self.query
    }

    pub fn audience(&self) -> &BotAudience {
        &self.audience
    }

    pub fn processing(&self) -> ActualProcessing {
        self.processing
    }
}

pub fn authorize_public_guide(
    request: &BotTaskRequest,
    context: &TrustedBotTaskContext<'_>,
) -> Result<AuthorizedPublicGuide, BotTaskError> {
    request.validate()?;
    let BotTask::PublicGuide { question } = &request.task else {
        return Err(BotTaskError::ResultMismatch);
    };
    let consumer = context.consumer;
    if !stable_id(&consumer.actor_id)
        || !stable_id(&consumer.principal_channel)
        || !stable_id(&consumer.guide_scope)
        || context.principal.actor_id != consumer.actor_id
        || context.principal.channel != consumer.principal_channel
        || consumer.platform != request.platform
        || !consumer.allowed_audiences.contains(&request.audience)
        || !consumer
            .capabilities
            .contains(&BotCapability::PublicGuideEndAnswer)
        || !context.principal.scopes.contains(&consumer.guide_scope)
        || context.request_id != request.request_id
        || context.audience != &request.audience
        || !context.request_allowed
        || !context.delivery_allowed
    {
        return Err(BotTaskError::PermissionDenied);
    }
    if context.input_provenance != VerifiedInputProvenance::ApprovedPublic {
        return Err(BotTaskError::UnapprovedInput);
    }
    match context.processing {
        ActualProcessing::Unknown => return Err(BotTaskError::ProcessingUnverified),
        ActualProcessing::RemoteInference | ActualProcessing::LoopbackRemoteProxy
            if !context
                .principal
                .provider_egress
                .contains(&consumer.guide_scope) =>
        {
            return Err(BotTaskError::PermissionDenied);
        }
        _ => {}
    }
    if context.guide_implementation != GuideImplementation::Approved {
        return Err(BotTaskError::Unavailable);
    }
    let query = Query {
        request_id: request.request_id.clone(),
        conversation_id: context.conversation_id.to_owned(),
        text: question.clone(),
        domain: None,
        requested_scopes: BTreeSet::from([consumer.guide_scope.clone()]),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    };
    query.validate().map_err(|_| BotTaskError::InvalidBinding)?;
    Ok(AuthorizedPublicGuide {
        query,
        audience: request.audience.clone(),
        processing: context.processing,
    })
}

fn stable_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 512 && id.trim() == id && !id.chars().any(char::is_control)
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum BotTaskError {
    #[error("Unbekannte Aufgabenvertragsversion")]
    VersionMismatch,
    #[error("Ungültige Anfragekennung")]
    InvalidRequestId,
    #[error("Ungültige Plattform- oder Zielbindung")]
    InvalidBinding,
    #[error("Ungültige öffentliche Frage")]
    InvalidQuestion,
    #[error("Ergebnisart passt nicht zur Aufgabe")]
    ResultMismatch,
    #[error("Ungültige öffentliche Antwort")]
    InvalidAnswer,
    #[error("Anfrage oder Versand nicht freigegeben")]
    PermissionDenied,
    #[error("Öffentliche Herkunft nicht bestätigt")]
    UnapprovedInput,
    #[error("Tatsächliche Modellverarbeitung nicht bestätigt")]
    ProcessingUnverified,
    #[error("Aufgabenimplementierung nicht freigegeben")]
    Unavailable,
}
