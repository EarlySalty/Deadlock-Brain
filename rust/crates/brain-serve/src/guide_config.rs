//! Konfiguration und Quellfreigaben für den begrenzten Guide-Betrieb.
use serde::Deserialize;
use std::collections::BTreeSet;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "name")]
    pub display_name: String,
    pub guild_id: String,
    #[serde(default)]
    pub allowed_users: BTreeSet<String>,
    #[serde(default)]
    pub private_dm_egress: bool,
    #[serde(default)]
    pub memory_retention_seconds: Option<i64>,
    #[serde(default = "inactivity")]
    pub conversation_inactivity_seconds: i64,
    pub moderator_channel_id: String,
    pub faq_channel_id: String,
    pub ticket_channel_id: String,
    pub mates_channel_id: String,
    pub voice_channel_id: String,
    pub coaching_channel_id: String,
    pub streamer_channel_id: String,
    pub team_channel_id: String,
    #[serde(default)]
    pub allowed_knowledge_sources: BTreeSet<String>,
    #[serde(default)]
    pub approved_discord_channels: BTreeSet<String>,
    #[serde(default)]
    pub approved_rule_channels: BTreeSet<String>,
    #[serde(default = "event_retention")]
    pub technical_event_retention_seconds: i64,
    #[serde(default = "source_age")]
    pub server_snapshot_max_age_seconds: i64,
}
fn event_retention() -> i64 {
    86400
}
fn source_age() -> i64 {
    300
}
fn name() -> String {
    "Serverguide".into()
}
fn inactivity() -> i64 {
    600
}
impl GuideConfig {
    pub fn valid(&self) -> bool {
        [
            &self.guild_id,
            &self.moderator_channel_id,
            &self.faq_channel_id,
            &self.ticket_channel_id,
            &self.mates_channel_id,
            &self.voice_channel_id,
            &self.coaching_channel_id,
            &self.streamer_channel_id,
            &self.team_channel_id,
        ]
        .iter()
        .all(|id| brain_contracts::guide::snowflake(id))
            && !self.display_name.trim().is_empty()
            && self.display_name.len() <= 60
            && !self.display_name.chars().any(char::is_control)
            && !self.display_name.to_lowercase().contains("brain")
            && self
                .allowed_users
                .iter()
                .all(|id| brain_contracts::guide::snowflake(id))
            && (!self.enabled
                || (!self.allowed_users.is_empty() && !self.allowed_knowledge_sources.is_empty()))
            && (60..=3600).contains(&self.conversation_inactivity_seconds)
            && self
                .memory_retention_seconds
                .is_none_or(|v| (60..=31_536_000).contains(&v))
            && self
                .allowed_knowledge_sources
                .iter()
                .all(|s| !s.trim().is_empty() && s.len() <= 128 && !s.chars().any(char::is_control))
            && self
                .approved_discord_channels
                .iter()
                .all(|s| brain_contracts::guide::snowflake(s))
            && !self
                .approved_discord_channels
                .contains(&self.moderator_channel_id)
            && self
                .approved_rule_channels
                .is_subset(&self.approved_discord_channels)
            && !self
                .approved_rule_channels
                .contains(&self.moderator_channel_id)
            && !self
                .approved_rule_channels
                .contains(&self.ticket_channel_id)
            && (60..=3600).contains(&self.server_snapshot_max_age_seconds)
            && (3600..=604800).contains(&self.technical_event_retention_seconds)
    }
}
