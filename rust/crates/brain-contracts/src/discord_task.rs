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

pub use crate::provider_input::discord_display_text as without_platform_ids;

#[cfg(test)]
mod tests {
    use super::*;

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
