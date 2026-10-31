pub use crate::bot_conversation::*;
use crate::{discord_task::DiscordAnswerCapability, PublicAnswerResponse, RequestDeadline};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const BOT_TASK_VERSION: &str = "brain.bot_tasks.v1";
pub const MAX_DISCORD_QUESTION_CHARS: usize = 4000;
pub const MAX_DISCORD_USER_QUESTIONS: usize = 4;
pub const MAX_TWITCH_PROMPT_CHARS: usize = 32_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotPlatform {
    Discord,
    Twitch,
}

impl BotPlatform {
    pub fn channel(self) -> &'static str {
        match self {
            Self::Discord => "discord",
            Self::Twitch => "twitch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotIdentity {
    Discord {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        guild_id: Option<u64>,
        channel_id: u64,
        user_id: u64,
    },
    Twitch {
        broadcaster_id: String,
    },
}

impl BotIdentity {
    pub fn platform(&self) -> BotPlatform {
        match self {
            Self::Discord { .. } => BotPlatform::Discord,
            Self::Twitch { .. } => BotPlatform::Twitch,
        }
    }

    fn valid(&self) -> bool {
        match self {
            Self::Discord {
                guild_id,
                channel_id,
                user_id,
            } => guild_id.is_none_or(|id| id > 0) && *channel_id > 0 && *user_id > 0,
            Self::Twitch { broadcaster_id } => {
                !broadcaster_id.is_empty()
                    && broadcaster_id.len() <= 20
                    && broadcaster_id.bytes().all(|b| b.is_ascii_digit())
                    && !broadcaster_id.starts_with('0')
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotCapability {
    DiscordConcierge,
    DiscordFaq,
    DiscordPassiveHelp,
    DiscordVoiceHint,
    DiscordInviteLounge,
    TwitchTitle,
    TwitchChat,
    TwitchPromoPitch,
    TwitchInviteReply,
    TwitchLfgReply,
    TwitchStandardReply,
    TwitchTip,
    TwitchDashboardAssistant,
    TwitchSelfExplainer,
    TwitchAnalysisChat,
}

impl BotCapability {
    pub const ALL: [Self; 15] = [
        Self::DiscordConcierge,
        Self::DiscordFaq,
        Self::DiscordPassiveHelp,
        Self::DiscordVoiceHint,
        Self::DiscordInviteLounge,
        Self::TwitchTitle,
        Self::TwitchChat,
        Self::TwitchPromoPitch,
        Self::TwitchInviteReply,
        Self::TwitchLfgReply,
        Self::TwitchStandardReply,
        Self::TwitchTip,
        Self::TwitchDashboardAssistant,
        Self::TwitchSelfExplainer,
        Self::TwitchAnalysisChat,
    ];

    pub fn platform(self) -> BotPlatform {
        match self {
            Self::DiscordConcierge
            | Self::DiscordFaq
            | Self::DiscordPassiveHelp
            | Self::DiscordVoiceHint
            | Self::DiscordInviteLounge => BotPlatform::Discord,
            Self::TwitchTitle
            | Self::TwitchChat
            | Self::TwitchPromoPitch
            | Self::TwitchInviteReply
            | Self::TwitchLfgReply
            | Self::TwitchStandardReply
            | Self::TwitchTip
            | Self::TwitchDashboardAssistant
            | Self::TwitchSelfExplainer
            | Self::TwitchAnalysisChat => BotPlatform::Twitch,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DiscordConcierge => "discord_concierge",
            Self::DiscordFaq => "discord_faq",
            Self::DiscordPassiveHelp => "discord_passive_help",
            Self::DiscordVoiceHint => "discord_voice_hint",
            Self::DiscordInviteLounge => "discord_invite_lounge",
            Self::TwitchTitle => "twitch_title",
            Self::TwitchChat => "twitch_chat",
            Self::TwitchPromoPitch => "twitch_promo_pitch",
            Self::TwitchInviteReply => "twitch_invite_reply",
            Self::TwitchLfgReply => "twitch_lfg_reply",
            Self::TwitchStandardReply => "twitch_standard_reply",
            Self::TwitchTip => "twitch_tip",
            Self::TwitchDashboardAssistant => "twitch_dashboard_assistant",
            Self::TwitchSelfExplainer => "twitch_self_explainer",
            Self::TwitchAnalysisChat => "twitch_analysis_chat",
        }
    }
}

impl From<DiscordAnswerCapability> for BotCapability {
    fn from(value: DiscordAnswerCapability) -> Self {
        match value {
            DiscordAnswerCapability::Concierge => Self::DiscordConcierge,
            DiscordAnswerCapability::Faq => Self::DiscordFaq,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TwitchTitlePurpose {
    Title,
    Insight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotTask {
    DiscordAnswer {
        capability: DiscordAnswerCapability,
        question: String,
        #[serde(default)]
        user_questions: Vec<String>,
    },
    TwitchTitle {
        purpose: TwitchTitlePurpose,
        prompt: String,
        temperature: f32,
        max_tokens: u32,
    },
    CapabilityStatus {
        capability: BotCapability,
    },
    Conversation(ConversationTask),
}

impl BotTask {
    pub fn capability(&self) -> BotCapability {
        match self {
            Self::DiscordAnswer { capability, .. } => (*capability).into(),
            Self::TwitchTitle { .. } => BotCapability::TwitchTitle,
            Self::CapabilityStatus { capability } => *capability,
            Self::Conversation(task) => task.capability(),
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::DiscordAnswer { .. } => "discord_answer",
            Self::TwitchTitle { .. } => "twitch_title",
            Self::CapabilityStatus { .. } => "capability_status",
            Self::Conversation(_) => "conversation",
        }
    }

    fn validate(&self) -> Result<(), BotTaskError> {
        match self {
            Self::DiscordAnswer {
                question,
                user_questions,
                ..
            } if question.trim().is_empty()
                || question.chars().count() > MAX_DISCORD_QUESTION_CHARS
                || user_questions.len() > MAX_DISCORD_USER_QUESTIONS
                || user_questions
                    .iter()
                    .map(|text| text.chars().count())
                    .sum::<usize>()
                    > MAX_DISCORD_QUESTION_CHARS
                || user_questions.iter().any(|text| text.trim().is_empty()) =>
            {
                Err(BotTaskError::InvalidQuestion)
            }
            Self::TwitchTitle { prompt, .. }
                if prompt.trim().is_empty() || prompt.chars().count() > MAX_TWITCH_PROMPT_CHARS =>
            {
                Err(BotTaskError::InvalidPrompt)
            }
            Self::TwitchTitle {
                temperature,
                max_tokens,
                ..
            } if !temperature.is_finite()
                || !(0.0..=2.0).contains(temperature)
                || *max_tokens == 0 =>
            {
                Err(BotTaskError::InvalidGeneration)
            }
            Self::Conversation(task) => task.validate(),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotTaskRequest {
    pub contract_version: String,
    pub request_id: String,
    pub identity: BotIdentity,
    pub task: BotTask,
}

impl BotTaskRequest {
    pub fn scoped_reads(&self) -> Option<&std::collections::BTreeSet<u64>> {
        match &self.task {
            BotTask::Conversation(task) => task.readable_channel_ids(),
            _ => None,
        }
    }

    fn direct_message(&self) -> bool {
        matches!(self.identity, BotIdentity::Discord { guild_id: None, .. })
    }

    pub fn validate(&self) -> Result<(), BotTaskError> {
        if self.contract_version != BOT_TASK_VERSION {
            return Err(BotTaskError::VersionMismatch);
        }
        if !stable_id(&self.request_id) {
            return Err(BotTaskError::InvalidRequestId);
        }
        if !self.identity.valid() || self.identity.platform() != self.task.capability().platform() {
            return Err(BotTaskError::InvalidBinding);
        }
        if self.scoped_reads().is_some() && !self.direct_message() {
            return Err(BotTaskError::InvalidBinding);
        }
        self.task.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityAvailability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityState {
    pub capability: BotCapability,
    pub availability: CapabilityAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskFailure {
    ProviderUnavailable,
    ProviderRateLimited,
    Timeout,
    InvalidOutput,
}

impl TaskFailure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderRateLimited => "provider_rate_limited",
            Self::Timeout => "timeout",
            Self::InvalidOutput => "invalid_output",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotTaskResult {
    DiscordAnswer {
        answer: PublicAnswerResponse,
    },
    TwitchTitle {
        text: String,
        model: String,
        usage: TaskUsage,
    },
    CapabilityStatus {
        capability: BotCapability,
        availability: CapabilityAvailability,
        #[serde(default)]
        capabilities: Vec<CapabilityState>,
    },
    Conversation {
        capability: BotCapability,
        reply: BotReply,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(default)]
        usage: TaskUsage,
    },
    Failed {
        capability: BotCapability,
        failure: TaskFailure,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotTaskResponse {
    pub contract_version: String,
    pub request_id: String,
    pub result: BotTaskResult,
}

impl BotTaskResponse {
    pub fn new(request: &BotTaskRequest, result: BotTaskResult) -> Self {
        Self {
            contract_version: BOT_TASK_VERSION.into(),
            request_id: request.request_id.clone(),
            result,
        }
    }

    pub fn validate_for(&self, request: &BotTaskRequest) -> Result<(), BotTaskError> {
        request.validate()?;
        if self.contract_version != BOT_TASK_VERSION {
            return Err(BotTaskError::VersionMismatch);
        }
        if self.request_id != request.request_id {
            return Err(BotTaskError::InvalidRequestId);
        }
        match (&request.task, &self.result) {
            (BotTask::DiscordAnswer { .. }, BotTaskResult::DiscordAnswer { answer }) => answer
                .validate(&request.request_id)
                .map_err(|_| BotTaskError::InvalidAnswer),
            (BotTask::TwitchTitle { .. }, BotTaskResult::TwitchTitle { text, model, .. })
                if !text.trim().is_empty() && !model.trim().is_empty() =>
            {
                Ok(())
            }
            (
                BotTask::CapabilityStatus {
                    capability: expected,
                },
                BotTaskResult::CapabilityStatus { capability, .. },
            ) if capability == expected => Ok(()),
            (
                BotTask::Conversation(task),
                BotTaskResult::Conversation {
                    capability, reply, ..
                },
            ) if *capability == task.capability() && reply.valid() => Ok(()),
            (task, BotTaskResult::Failed { capability, .. })
                if *capability == task.capability() =>
            {
                Ok(())
            }
            _ => Err(BotTaskError::ResultMismatch),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwitchTitleInput<'a> {
    pub purpose: TwitchTitlePurpose,
    pub prompt: &'a str,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedTitle {
    pub text: String,
    pub model: String,
    pub usage: TaskUsage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConversationPrompt {
    pub system: String,
    pub user: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

pub trait ConversationPort: Send + Sync {
    fn available(&self) -> bool;
    fn generate(
        &self,
        prompt: &ConversationPrompt,
        deadline: &RequestDeadline,
    ) -> Result<GeneratedReply, TaskFailure>;
}

pub trait TwitchTitlePort: Send + Sync {
    fn max_output_tokens(&self) -> u32;
    fn available(&self) -> bool;
    fn generate(
        &self,
        input: &TwitchTitleInput<'_>,
        deadline: &RequestDeadline,
    ) -> Result<GeneratedTitle, TaskFailure>;
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
    #[error("Ungültige Plattformbindung")]
    InvalidBinding,
    #[error("Ungültige Frage")]
    InvalidQuestion,
    #[error("Ungültige Vorgabe")]
    InvalidPrompt,
    #[error("Ungültige Generierungsparameter")]
    InvalidGeneration,
    #[error("Ergebnisart passt nicht zur Aufgabe")]
    ResultMismatch,
    #[error("Ungültige Gesprächsaufgabe")]
    InvalidConversation,
    #[error("Ungültige öffentliche Antwort")]
    InvalidAnswer,
}
