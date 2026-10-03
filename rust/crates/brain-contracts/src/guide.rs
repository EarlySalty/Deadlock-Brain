//! Typisierter Serverguide-Vertrag. Discord-Inhalte bleiben Daten.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const GUIDE_VERSION: &str = "guide.v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    Dm,
    Public,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Addressed {
    Dm,
    Mention,
    Reply,
    Followup,
    Command,
    TourButton,
    Control,
    CommunityQuestion,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Message,
    TourStart,
    TourStep,
    ProfileControl,
    FeedbackSubmit,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTurn {
    pub request_id: String,
    pub guild_id: String,
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub reply_to_message_id: Option<String>,
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub bot_user_id: Option<String>,
    pub surface: Surface,
    pub addressed: Addressed,
    pub event: Event,
    pub content: String,
    #[serde(default)]
    pub control: Option<ProfileControl>,
    #[serde(default)]
    pub domain: Option<GuideDomainCommand>,
    #[serde(default)]
    pub access_invite: Option<AccessInviteCommand>,
    #[serde(default)]
    pub human_helped: bool,
}
impl GuideTurn {
    pub fn valid(&self) -> bool {
        !self.request_id.is_empty()
            && self.request_id.len() <= 160
            && self
                .request_id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
            && [
                &self.guild_id,
                &self.user_id,
                &self.channel_id,
                &self.message_id,
            ]
            .iter()
            .all(|v| snowflake(v))
            && [
                &self.thread_id,
                &self.reply_to_message_id,
                &self.bot_user_id,
            ]
            .iter()
            .all(|v| v.as_ref().is_none_or(|v| snowflake(v)))
            && self.conversation_id.as_ref().is_none_or(|v| {
                !v.is_empty()
                    && v.len() <= 160
                    && v.bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
            })
            && !self.content.trim().is_empty()
            && self.content.len() <= 16000
            && !(self.surface == Surface::Public
                && matches!(self.addressed, Addressed::Dm | Addressed::Control))
            && self.control.as_ref().is_none_or(ProfileControl::valid)
            && self.domain.as_ref().is_none_or(|domain| {
                domain.valid()
                    && self.addressed == Addressed::Command
                    && self.event == Event::Message
                    && self.control.is_none()
                    && self.access_invite.is_none()
            })
            && self.access_invite.as_ref().is_none_or(|command| {
                command.valid()
                    && self.event == Event::Message
                    && self.control.is_none()
                    && self.domain.is_none()
                    && self.request_id.len() <= 128
            })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AccessInviteCommand {
    AwaitCode,
    Request { friend_code: String },
    Status { action_id: String },
    Cancel { action_id: String },
}
impl AccessInviteCommand {
    pub fn valid(&self) -> bool {
        match self {
            Self::AwaitCode => true,
            Self::Request { friend_code } => {
                !friend_code.trim().is_empty()
                    && friend_code.len() <= 32
                    && !friend_code.chars().any(char::is_control)
            }
            Self::Status { action_id } | Self::Cancel { action_id } => {
                !action_id.is_empty()
                    && action_id.len() <= 128
                    && action_id
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
            }
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InviteCorrelation {
    pub action_id: String,
    pub turn_id: String,
    pub actor_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub source_thread_id: Option<i64>,
    pub source_event_type: String,
    pub message_id: i64,
    pub privacy_epoch: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    Queued,
    FriendRequestSent,
    WaitingForAcceptance,
    InviteSent,
    AlreadyHasAccess,
    Failed,
    Unknown,
    Expired,
    Cancelled,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideDomainCommand {
    pub request: crate::domain::DomainRequest,
    pub patch: String,
    pub mode: String,
}
impl GuideDomainCommand {
    pub fn valid(&self) -> bool {
        !matches!(self.request, crate::domain::DomainRequest::Card { .. })
            && self.request.validate().is_ok()
            && [&self.patch, &self.mode].iter().all(|value| {
                !value.trim().is_empty()
                    && value.len() <= 512
                    && !value.chars().any(char::is_control)
            })
    }
}
pub fn snowflake(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 20
        && value.bytes().all(|c| c.is_ascii_digit())
        && value
            .parse::<u64>()
            .is_ok_and(|n| n > 0 && n <= i64::MAX as u64)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProfileControl {
    View,
    Forget,
    Memory { enabled: bool },
    Correct { field: ProfileField, value: String },
    Remove { field: ProfileField },
}
impl ProfileControl {
    pub fn valid(&self) -> bool {
        match self {
            Self::Correct { value, .. } => {
                !value.trim().is_empty()
                    && value.len() <= 500
                    && !value.chars().any(char::is_control)
            }
            _ => true,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileField {
    GameInterests,
    CurrentGoals,
    PlayTimes,
    Communication,
    AnswerLength,
    ExplainedFlows,
    OpenConcern,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileValue {
    pub value: String,
    pub origin_message_id: String,
    pub updated_at: i64,
    pub expires_at: i64,
    pub explicitly_stated: bool,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileSnapshot {
    pub memory_enabled: bool,
    pub contact_enabled: bool,
    pub globally_opted_out: bool,
    pub deleted: bool,
    pub epoch: i64,
    pub fields: BTreeMap<ProfileField, ProfileValue>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideStatus {
    Reply,
    Silent,
    Unavailable,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum GuideAction {
    Feedback {
        delivery_id: String,
        destination_channel_id: String,
        text: String,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideResult {
    #[serde(default)]
    pub privacy_epoch: Option<i64>,
    pub contract_version: String,
    pub request_id: String,
    pub status: GuideStatus,
    pub reply: Option<String>,
    pub conversation_id: Option<String>,
    pub expires_at: Option<i64>,
    pub actions: Vec<GuideAction>,
    pub profile: Option<ProfileSnapshot>,
    pub memory_notice: Option<String>,
    pub contact_proactive: bool,
    #[serde(default)]
    pub control_result: Option<String>,
}
impl GuideResult {
    pub fn silent(request_id: &str) -> Self {
        Self {
            privacy_epoch: None,
            contract_version: GUIDE_VERSION.into(),
            request_id: request_id.into(),
            status: GuideStatus::Silent,
            reply: None,
            conversation_id: None,
            expires_at: None,
            actions: vec![],
            profile: None,
            memory_notice: None,
            contact_proactive: false,
            control_result: None,
        }
    }
    pub fn reply(request_id: &str, reply: String) -> Self {
        Self {
            status: GuideStatus::Reply,
            reply: Some(reply),
            ..Self::silent(request_id)
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionResult {
    pub request_id: String,
    pub guild_id: String,
    pub user_id: String,
    pub delivery_id: String,
    pub success: bool,
    #[serde(default)]
    pub sent_message_id: Option<String>,
    #[serde(default)]
    pub reply_message_id: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideConversation {
    pub id: String,
    pub channel_id: String,
    pub thread_id: Option<String>,
    pub user_id: String,
    pub surface: Surface,
    pub last_user_message_id: String,
    pub last_bot_message_id: Option<String>,
    pub expires_at: i64,
    pub closed: bool,
    #[serde(default)]
    pub pending_access_invite: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideHistory {
    pub role: String,
    pub content: String,
    pub message_id: String,
    pub expires_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerSnapshot {
    pub guild_id: String,
    pub revision: u64,
    pub observed_at: i64,
    pub channels: Vec<ServerChannel>,
    pub roles: Vec<ServerRole>,
    pub rules: Vec<ServerRule>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerChannel {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub public_readable: bool,
    #[serde(default)]
    pub deleted: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerRole {
    pub id: String,
    pub name: String,
    pub public: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerRule {
    pub channel_id: String,
    pub message_id: String,
    pub text: String,
    pub updated_at: i64,
    #[serde(default)]
    pub deleted: bool,
}
