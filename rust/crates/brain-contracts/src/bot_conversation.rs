use crate::bot_tasks::{BotCapability, BotTaskError, TaskUsage};
use crate::{PublicArtifact, PublicCitation};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

pub const MAX_TASK_TEXT_CHARS: usize = 4000;
pub const MAX_LABEL_CHARS: usize = 200;
pub const MAX_DIALOG_TURNS: usize = 20;
pub const MAX_DIALOG_CHARS: usize = 12_000;
pub const MAX_CHAT_LINES: usize = 40;
pub const MAX_CHAT_LINE_CHARS: usize = 500;
pub const MAX_LIST_ITEMS: usize = 20;
pub const MAX_LIST_ITEM_CHARS: usize = 500;
pub const MAX_DOCUMENT_CHARS: usize = 24_000;
pub const MAX_ANALYSIS_BYTES: usize = 40_000;
pub const MAX_READABLE_CHANNELS: usize = 1000;
pub const MAX_REPLY_BYTES: usize = 8 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialogTurn {
    pub role: DialogRole,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatLine {
    pub author: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamContext {
    pub streamer_login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub deadlock_live: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatPersona {
    #[default]
    Veteran,
    Rookie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandardReplyKind {
    Greeting,
    ReleaseDate,
    Thanks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplyLanguage {
    #[default]
    De,
    En,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PromoPitchRequest {
    Reactive {
        target_login: String,
        message: String,
    },
    Targeted {
        target_login: String,
        target_messages: Vec<String>,
    },
    Partner {
        target_login: String,
        target_messages: Vec<String>,
    },
    ChannelPromo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "capability", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConversationTask {
    DiscordConcierge {
        question: String,
        #[serde(default)]
        history: Vec<DialogTurn>,
        #[serde(default)]
        readable_channel_ids: BTreeSet<u64>,
        #[serde(default)]
        member_facts: Vec<String>,
    },
    DiscordFaq {
        question: String,
        #[serde(default)]
        history: Vec<DialogTurn>,
    },
    DiscordPassiveHelp {
        message: String,
        #[serde(default)]
        recent_messages: Vec<ChatLine>,
    },
    DiscordVoiceHint {
        message: String,
    },
    DiscordInviteLounge {
        message: String,
    },
    TwitchChat {
        stream: StreamContext,
        author: String,
        message: String,
        #[serde(default)]
        recent_messages: Vec<ChatLine>,
        #[serde(default)]
        persona: ChatPersona,
        #[serde(default)]
        talkative: bool,
    },
    TwitchPromoPitch {
        stream: StreamContext,
        pitch: PromoPitchRequest,
        #[serde(default)]
        recent_messages: Vec<ChatLine>,
        #[serde(default)]
        examples: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        invite_url: Option<String>,
    },
    TwitchInviteReply {
        stream: StreamContext,
        author: String,
        message: String,
        #[serde(default)]
        recent_messages: Vec<ChatLine>,
        is_newcomer: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        invite_url: Option<String>,
    },
    TwitchLfgReply {
        stream: StreamContext,
        author: String,
        message: String,
        #[serde(default)]
        recent_messages: Vec<ChatLine>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        invite_url: Option<String>,
    },
    TwitchStandardReply {
        stream: StreamContext,
        reply: StandardReplyKind,
        author: String,
        message: String,
    },
    TwitchTip {
        slug: String,
        source_text: String,
        #[serde(default)]
        language: ReplyLanguage,
    },
    TwitchDashboardAssistant {
        streamer_name: String,
        page: String,
        question: String,
        #[serde(default)]
        history: Vec<DialogTurn>,
        knowledge: String,
        streamer_data: String,
        #[serde(default)]
        language: ReplyLanguage,
    },
    TwitchSelfExplainer {
        question: String,
        #[serde(default)]
        history: Vec<DialogTurn>,
        documents: String,
        #[serde(default)]
        language: ReplyLanguage,
    },
    TwitchAnalysisChat {
        streamer_name: String,
        question: String,
        #[serde(default)]
        history: Vec<DialogTurn>,
        analysis: serde_json::Value,
        #[serde(default)]
        language: ReplyLanguage,
    },
}

impl ConversationTask {
    pub fn audit_question(&self) -> Option<&str> {
        match self {
            Self::DiscordConcierge { question, .. }
            | Self::DiscordFaq { question, .. }
            | Self::TwitchDashboardAssistant { question, .. }
            | Self::TwitchSelfExplainer { question, .. }
            | Self::TwitchAnalysisChat { question, .. } => Some(question),
            _ => None,
        }
    }

    pub fn capability(&self) -> BotCapability {
        match self {
            Self::DiscordConcierge { .. } => BotCapability::DiscordConcierge,
            Self::DiscordFaq { .. } => BotCapability::DiscordFaq,
            Self::DiscordPassiveHelp { .. } => BotCapability::DiscordPassiveHelp,
            Self::DiscordVoiceHint { .. } => BotCapability::DiscordVoiceHint,
            Self::DiscordInviteLounge { .. } => BotCapability::DiscordInviteLounge,
            Self::TwitchChat { .. } => BotCapability::TwitchChat,
            Self::TwitchPromoPitch { .. } => BotCapability::TwitchPromoPitch,
            Self::TwitchInviteReply { .. } => BotCapability::TwitchInviteReply,
            Self::TwitchLfgReply { .. } => BotCapability::TwitchLfgReply,
            Self::TwitchStandardReply { .. } => BotCapability::TwitchStandardReply,
            Self::TwitchTip { .. } => BotCapability::TwitchTip,
            Self::TwitchDashboardAssistant { .. } => BotCapability::TwitchDashboardAssistant,
            Self::TwitchSelfExplainer { .. } => BotCapability::TwitchSelfExplainer,
            Self::TwitchAnalysisChat { .. } => BotCapability::TwitchAnalysisChat,
        }
    }

    pub fn readable_channel_ids(&self) -> Option<&BTreeSet<u64>> {
        match self {
            Self::DiscordConcierge {
                readable_channel_ids,
                ..
            } if !readable_channel_ids.is_empty() => Some(readable_channel_ids),
            _ => None,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), BotTaskError> {
        let valid = match self {
            Self::DiscordConcierge {
                question,
                history,
                readable_channel_ids,
                member_facts,
            } => {
                text(question)
                    && dialog(history)
                    && list(member_facts)
                    && readable_channel_ids.len() <= MAX_READABLE_CHANNELS
                    && !readable_channel_ids.contains(&0)
            }
            Self::DiscordFaq { question, history } => text(question) && dialog(history),
            Self::DiscordPassiveHelp {
                message,
                recent_messages,
            } => text(message) && chat(recent_messages),
            Self::DiscordVoiceHint { message } | Self::DiscordInviteLounge { message } => {
                text(message)
            }
            Self::TwitchChat {
                stream,
                author,
                message,
                recent_messages,
                ..
            }
            | Self::TwitchInviteReply {
                stream,
                author,
                message,
                recent_messages,
                invite_url: None,
                ..
            }
            | Self::TwitchLfgReply {
                stream,
                author,
                message,
                recent_messages,
                invite_url: None,
            } => stream.valid() && label(author) && text(message) && chat(recent_messages),
            Self::TwitchInviteReply {
                stream,
                author,
                message,
                recent_messages,
                invite_url: Some(url),
                ..
            }
            | Self::TwitchLfgReply {
                stream,
                author,
                message,
                recent_messages,
                invite_url: Some(url),
            } => {
                stream.valid()
                    && label(author)
                    && text(message)
                    && chat(recent_messages)
                    && link(url)
            }
            Self::TwitchPromoPitch {
                stream,
                pitch,
                recent_messages,
                examples,
                invite_url,
            } => {
                stream.valid()
                    && pitch.valid()
                    && chat(recent_messages)
                    && list(examples)
                    && invite_url.as_deref().is_none_or(link)
            }
            Self::TwitchStandardReply {
                stream,
                author,
                message,
                ..
            } => stream.valid() && label(author) && text(message),
            Self::TwitchTip {
                slug, source_text, ..
            } => label(slug) && text(source_text),
            Self::TwitchDashboardAssistant {
                streamer_name,
                page,
                question,
                history,
                knowledge,
                streamer_data,
                ..
            } => {
                label(streamer_name)
                    && label(page)
                    && text(question)
                    && dialog(history)
                    && document(knowledge)
                    && document(streamer_data)
            }
            Self::TwitchSelfExplainer {
                question,
                history,
                documents,
                ..
            } => text(question) && dialog(history) && document(documents),
            Self::TwitchAnalysisChat {
                streamer_name,
                question,
                history,
                analysis,
                ..
            } => {
                label(streamer_name)
                    && text(question)
                    && dialog(history)
                    && analysis.is_object()
                    && serde_json::to_string(analysis)
                        .is_ok_and(|encoded| encoded.len() <= MAX_ANALYSIS_BYTES)
            }
        };
        if valid {
            Ok(())
        } else {
            Err(BotTaskError::InvalidConversation)
        }
    }
}

impl StreamContext {
    fn valid(&self) -> bool {
        label(&self.streamer_login)
            && self.game.as_deref().is_none_or(label)
            && self.title.as_deref().is_none_or(|value| {
                value.chars().count() <= MAX_LIST_ITEM_CHARS && !value.chars().any(char::is_control)
            })
    }
}

impl PromoPitchRequest {
    fn valid(&self) -> bool {
        match self {
            Self::Reactive {
                target_login,
                message,
            } => label(target_login) && text(message),
            Self::Targeted {
                target_login,
                target_messages,
            }
            | Self::Partner {
                target_login,
                target_messages,
            } => label(target_login) && !target_messages.is_empty() && list(target_messages),
            Self::ChannelPromo => true,
        }
    }
}

fn text(value: &str) -> bool {
    !value.trim().is_empty() && value.chars().count() <= MAX_TASK_TEXT_CHARS
}

fn label(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().count() <= MAX_LABEL_CHARS
        && !value.chars().any(char::is_control)
}

fn document(value: &str) -> bool {
    !value.trim().is_empty() && value.chars().count() <= MAX_DOCUMENT_CHARS
}

fn dialog(turns: &[DialogTurn]) -> bool {
    turns.len() <= MAX_DIALOG_TURNS
        && turns.iter().all(|turn| !turn.text.trim().is_empty())
        && turns
            .iter()
            .map(|turn| turn.text.chars().count())
            .sum::<usize>()
            <= MAX_DIALOG_CHARS
}

fn chat(lines: &[ChatLine]) -> bool {
    lines.len() <= MAX_CHAT_LINES
        && lines.iter().all(|line| {
            label(&line.author)
                && !line.text.trim().is_empty()
                && line.text.chars().count() <= MAX_CHAT_LINE_CHARS
        })
}

fn list(items: &[String]) -> bool {
    items.len() <= MAX_LIST_ITEMS
        && items
            .iter()
            .all(|item| !item.trim().is_empty() && item.chars().count() <= MAX_LIST_ITEM_CHARS)
}

fn link(value: &str) -> bool {
    value.starts_with("https://")
        && value.len() <= MAX_LABEL_CHARS
        && !value.chars().any(|c| c.is_whitespace() || c.is_control())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SilentReason {
    NoOccasion,
    InsufficientEvidence,
    OutputRejected,
}

impl SilentReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoOccasion => "no_occasion",
            Self::InsufficientEvidence => "insufficient_evidence",
            Self::OutputRejected => "output_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PitchOccasion {
    NoMates,
    GameUnpopular,
    TooTryhard,
    SoloQueue,
    NewPlayer,
    WantsHelp,
    RankedCompetitive,
    BuildMeta,
    Coaching,
    PatchnotesNews,
    ScamProtection,
    NewcomerInterest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReplySignal {
    PateRequest,
    PitchOccasion { occasion: PitchOccasion },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BotReply {
    Answered {
        text: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        signals: Vec<ReplySignal>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        citations: Vec<PublicCitation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        artifacts: Vec<PublicArtifact>,
    },
    Silent {
        reason: SilentReason,
    },
}

impl BotReply {
    pub fn answered(text: impl Into<String>) -> Self {
        Self::Answered {
            text: text.into(),
            signals: Vec::new(),
            citations: Vec::new(),
            artifacts: Vec::new(),
        }
    }

    pub fn outcome(&self) -> &'static str {
        match self {
            Self::Answered { .. } => "answered",
            Self::Silent { reason } => reason.as_str(),
        }
    }

    pub(crate) fn valid(&self) -> bool {
        match self {
            Self::Silent { .. } => true,
            Self::Answered {
                text,
                citations,
                artifacts,
                ..
            } => {
                let unique_citations = citations
                    .iter()
                    .map(|citation| citation.citation_id.as_str())
                    .collect::<HashSet<_>>()
                    .len()
                    == citations.len();
                !text.trim().is_empty()
                    && text.len() <= MAX_REPLY_BYTES
                    && unique_citations
                    && artifacts.iter().all(|artifact| artifact.validate().is_ok())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedReply {
    pub text: String,
    pub model: String,
    pub usage: TaskUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationAudit {
    pub capability: BotCapability,
    pub actor_id: String,
    pub channel: String,
    pub question: Option<String>,
    pub status: &'static str,
    pub answer_text: Option<String>,
    pub reason: Option<&'static str>,
    pub usage: TaskUsage,
    pub latency_ms: u64,
}

pub trait ConversationAuditPort: Send + Sync {
    fn record(&self, audit: ConversationAudit);
}
