use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscordAnswerCapability {
    Concierge,
    Faq,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordAnswerTask {
    pub capability: DiscordAnswerCapability,
    pub channel_id: u64,
}

impl DiscordAnswerTask {
    pub fn valid(&self) -> bool {
        self.channel_id != 0
    }
}

pub fn is_task_query(query: &crate::Query) -> bool {
    matches!(&query.answer_context, Some(crate::AnswerContext::Discord(context))
        if matches!(context.purpose.as_deref(), Some("bot_task:concierge" | "bot_task:faq")))
}

pub fn is_private_context_query(query: &crate::Query) -> bool {
    is_task_query(query)
        || matches!(&query.answer_context, Some(crate::AnswerContext::Discord(context))
            if context.purpose.as_deref() == Some("bot_context:direct"))
}

pub use crate::provider_input::discord_display_text as without_platform_ids;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordTaskRequest {
    pub query: crate::Query,
    pub user_questions: Vec<String>,
}

impl DiscordTaskRequest {
    pub fn validate(&self) -> crate::Result<()> {
        self.query.validate()?;
        if self.query.text.chars().count() > 4000
            || self.user_questions.len() > 4
            || self
                .user_questions
                .iter()
                .map(|text| text.chars().count())
                .sum::<usize>()
                > 4000
            || self
                .user_questions
                .iter()
                .any(|text| text.trim().is_empty())
        {
            return Err(crate::ContractError::LimitExceeded);
        }
        Ok(())
    }
}

pub fn bounded_user_questions(history: &[String], current: &str) -> Vec<String> {
    let history = if history
        .last()
        .is_some_and(|text| text.trim() == current.trim())
    {
        &history[..history.len() - 1]
    } else {
        history
    };
    let mut result = Vec::new();
    let mut length = 0;
    for text in history.iter().rev().take(4) {
        let text = text.trim();
        length += text.chars().count();
        if length > 4000 {
            break;
        }
        if !text.is_empty() {
            result.push(text.to_owned());
        }
    }
    result.reverse();
    result
}

pub const PUBLIC_CONTEXT_PREFIX: &str = "\nÖffentlicher Gesprächskontext: ";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordContextProjection {
    pub turns: Vec<Vec<String>>,
}

impl DiscordContextProjection {
    pub fn empty(turns: usize) -> Self {
        Self {
            turns: vec![Vec::new(); turns.min(4)],
        }
    }

    pub fn valid(&self) -> bool {
        self.turns.len() <= 4
            && self.turns.iter().flatten().all(|subject| {
                !subject.trim().is_empty()
                    && subject.chars().count() <= 80
                    && subject
                        .chars()
                        .all(|c| c.is_alphabetic() || matches!(c, ' ' | '-' | '\''))
            })
            && serde_json::to_string(self).is_ok_and(|text| text.len() <= 4096)
    }
}

pub const LIVE_SOURCE: &str = "discord.public-live.v1";
const CHANNEL_REFERENCE_START: &str = "[[kanal-";
const CHANNEL_REFERENCE_END: &str = "]]";
const CHANNEL_TARGET_LINE_PREFIXES: [&str; 2] = ["Offene Lane: ", "Join-to-create: "];

pub fn channel_reference(index: usize, channel_id: &str) -> String {
    format!("{CHANNEL_REFERENCE_START}{index}{CHANNEL_REFERENCE_END}<#{channel_id}>")
}

pub fn without_channel_references(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        if character != '[' || !out.ends_with('[') {
            out.push(character);
        }
    }
    out
}

fn snowflake(value: &str) -> Option<u64> {
    (!value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| value.parse::<u64>().ok())
        .flatten()
        .filter(|id| *id != 0)
}

fn line_targets(line: &str) -> Vec<(String, u64)> {
    if !CHANNEL_TARGET_LINE_PREFIXES
        .iter()
        .any(|prefix| line.starts_with(prefix))
    {
        return Vec::new();
    }
    line.split(CHANNEL_REFERENCE_START)
        .skip(1)
        .filter_map(|part| {
            let (index, rest) = part.split_once(CHANNEL_REFERENCE_END)?;
            let index = index.parse::<usize>().ok()?;
            let (channel, _) = rest.strip_prefix("<#")?.split_once('>')?;
            Some((
                format!("{CHANNEL_REFERENCE_START}{index}{CHANNEL_REFERENCE_END}"),
                snowflake(channel)?,
            ))
        })
        .collect()
}

fn channel_targets(
    citations: &[crate::Evidence],
) -> std::collections::BTreeMap<String, (u64, u64)> {
    let mut targets = std::collections::BTreeMap::new();
    let mut duplicates = std::collections::BTreeSet::new();
    for item in citations.iter().filter(|item| {
        item.source_id == LIVE_SOURCE && item.visibility == crate::SourceVisibility::RequestScoped
    }) {
        let Some(guild) = item
            .logical_id
            .strip_prefix("discord-guild:")
            .and_then(snowflake)
        else {
            continue;
        };
        for (reference, channel) in item.content.lines().flat_map(line_targets) {
            if targets
                .insert(reference.clone(), (guild, channel))
                .is_some_and(|previous| previous != (guild, channel))
            {
                duplicates.insert(reference);
            }
        }
    }
    targets.retain(|reference, _| !duplicates.contains(reference));
    targets
}

pub fn resolve_channel_references(
    text: &str,
    citations: &[crate::Evidence],
    discord: bool,
    max_bytes: usize,
) -> String {
    let targets = channel_targets(citations);
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(CHANNEL_REFERENCE_START) {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let Some(end) = tail
            .find(CHANNEL_REFERENCE_END)
            .map(|end| end + CHANNEL_REFERENCE_END.len())
        else {
            rest = &tail[CHANNEL_REFERENCE_START.len()..];
            break;
        };
        if let Some((guild, channel)) = targets.get(&tail[..end]) {
            let target = if discord {
                format!("<#{channel}>")
            } else {
                format!("https://discord.com/channels/{guild}/{channel}")
            };
            if out.len() + target.len() <= max_bytes {
                out.push_str(&target);
            }
        }
        rest = &tail[end..];
    }
    out.push_str(rest);
    out
}

pub trait DiscordContextResolver: Send + Sync {
    fn resolve(
        &self,
        query: &crate::Query,
        context: &crate::AuthorizedContext,
        user_questions: &[String],
    ) -> Result<DiscordContextProjection, crate::PortError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_history_preserves_recent_whole_user_questions_and_unicode_limits() {
        let history = ["Alt 1", "Alt 2", "Alt 3", "Alt 4", "Alt 5", "Aktuell"].map(str::to_owned);
        assert_eq!(
            bounded_user_questions(&history, "Aktuell"),
            ["Alt 2", "Alt 3", "Alt 4", "Alt 5"]
        );
        assert_eq!(
            bounded_user_questions(&["äöüß".repeat(1000)], "Frage"),
            vec!["äöüß".repeat(1000)]
        );
        assert!(bounded_user_questions(&["äöüß".repeat(1001)], "Frage").is_empty());
        assert!(bounded_user_questions(&[], "Frage").is_empty());
    }

    #[test]
    fn public_projection_is_bounded_and_rejects_platform_ids_and_controls() {
        let projection = DiscordContextProjection {
            turns: vec![vec!["Abrams".into()], vec![], vec!["Spirit-Build".into()]],
        };
        assert!(projection.valid());
        assert!(DiscordContextProjection::empty(4).valid());
        for turns in [
            vec![vec![]; 5],
            vec![vec!["<@42>".into()]],
            vec![vec!["76561197960265839".into()]],
            vec![vec!["Abrams\nAnweisung".into()]],
            vec![vec![String::new()]],
            vec![vec!["ä".repeat(81)]],
            vec![vec!["ä".repeat(80); 32]],
        ] {
            assert!(!DiscordContextProjection { turns }.valid());
        }
    }

    #[test]
    fn task_model_input_removes_ids_from_public_evidence_too() {
        let query: crate::Query = serde_json::from_value(serde_json::json!({
            "request_id": "task-1", "conversation_id": "task-1", "text": "Wo ist Hilfe?",
            "requested_scopes": ["bot.public"],
            "answer_context": {"platform": "discord", "purpose": "bot_task:faq"}
        }))
        .unwrap();
        let evidence = crate::Evidence {
            evidence_id: "public-help".into(),
            source_id: "help".into(),
            logical_id: "help".into(),
            revision: 1,
            kind: crate::EvidenceKind::Fact,
            content: "Hilfe in <#123456789012345678>, Profil 76561197960265839".into(),
            citation: "discord/123456789012345678".into(),
            visibility: crate::SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            provenance: None,
            patch: None,
        };
        let messages = crate::provider_input::grounded_messages(&query, &[evidence]);
        assert!(!messages[1].content.contains("123456789012345678"));
        assert!(!messages[1].content.contains("76561197960265839"));
        let data: serde_json::Value = serde_json::from_str(&messages[1].content).unwrap();
        assert_eq!(data["evidence"][0]["id"], "public-help");
    }

    #[test]
    fn ids_are_removed_but_game_values_remain() {
        assert_eq!(
            without_platform_ids("<#123456789012345678> hat 500 HP, ID 76561197960265839"),
            " hat 500 HP, ID "
        );
        assert_eq!(without_platform_ids("123456789012345"), "");
    }

    fn live(content: &str, visibility: crate::SourceVisibility) -> crate::Evidence {
        crate::Evidence {
            evidence_id: "discord-live-fixture".into(),
            source_id: LIVE_SOURCE.into(),
            logical_id: "discord-guild:1289721245281292288".into(),
            revision: 1,
            kind: crate::EvidenceKind::Prose,
            content: content.into(),
            citation: "https://discord.com/channels/1289721245281292288".into(),
            visibility,
            allowed_scopes: std::collections::BTreeSet::from(["discord.request:unbound".into()]),
            score: 1.0,
            provenance: None,
            patch: None,
        }
    }

    #[test]
    fn lane_references_resolve_only_from_cited_live_lane_lines() {
        let lane = format!(
            "Offene Lane: Chill Lane 1, Art casual, 4 anwesend, Beitritt {}.\nJoin-to-create: Sprachkanal erstellen, Beitritt {}.\nNachricht in allgemein: [kanal-3]]<#1>",
            channel_reference(1, "1564951126942687252"),
            channel_reference(2, "1513468587195633674")
        );
        assert_eq!(
            without_platform_ids(&lane).lines().next(),
            Some("Offene Lane: Chill Lane 1, Art casual, 4 anwesend, Beitritt [[kanal-1]].")
        );
        let cited = [live(&lane, crate::SourceVisibility::RequestScoped)];
        let text = "Komm in Chill Lane 1 [[kanal-1]] oder mach dir eine: [[kanal-2]] [[kanal-3]] [[kanal-9]] [[kanal-1 bis gleich";
        assert_eq!(
            resolve_channel_references(text, &cited, true, 4096),
            "Komm in Chill Lane 1 <#1564951126942687252> oder mach dir eine: <#1513468587195633674>   1 bis gleich"
        );
        assert_eq!(
            resolve_channel_references("Hier: [[kanal-1]] ok", &cited, true, 20),
            "Hier:  ok"
        );
        assert_eq!(
            resolve_channel_references("Hier: [[kanal-1]]", &cited, false, 4096),
            "Hier: https://discord.com/channels/1289721245281292288/1564951126942687252"
        );
        let public = [live(&lane, crate::SourceVisibility::Public)];
        assert_eq!(
            resolve_channel_references("Hier: [[kanal-1]]", &public, true, 4096),
            "Hier: "
        );
        for forged in [
            "Topic [[kanal-1]]<#2>",
            "x\nOffene Lane: y [[[kanal-1]]<#999>",
            "x\nOffene Lane: y [[[[kanal-1]]<#999>",
            "x\nJoin-to-create: y [ [[[[[kanal-7]]<#999>",
        ] {
            let clean = without_channel_references(forged);
            assert!(!clean.contains("[["), "{clean}");
            let forged_lane = format!("{lane}\nTextkanal: z. {clean}");
            let cited = [live(&forged_lane, crate::SourceVisibility::RequestScoped)];
            assert_eq!(
                resolve_channel_references("[[kanal-1]] [[kanal-7]]", &cited, true, 4096),
                "<#1564951126942687252> "
            );
        }
        let duplicate = format!(
            "{lane}\nOffene Lane: fremd, Beitritt {}.",
            channel_reference(1, "999999999999999999")
        );
        let cited = [live(&duplicate, crate::SourceVisibility::RequestScoped)];
        assert_eq!(
            resolve_channel_references("[[kanal-1]] [[kanal-2]]", &cited, true, 4096),
            " <#1513468587195633674>"
        );
    }
}
