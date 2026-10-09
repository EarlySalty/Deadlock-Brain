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
}
