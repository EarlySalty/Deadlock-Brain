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

pub fn needs_reference(text: &str) -> bool {
    let words = crate::lexical::terms(text);
    let deictic = words.iter().any(|word| {
        matches!(
            word.as_str(),
            "dafuer"
                | "dort"
                | "dabei"
                | "damit"
                | "darueber"
                | "dazu"
                | "davon"
                | "dahin"
                | "daraus"
                | "dieser"
                | "diese"
                | "dieses"
                | "diesem"
                | "diesen"
                | "dessen"
                | "er"
                | "ihn"
                | "ihm"
                | "ihnen"
                | "ihre"
                | "ihren"
                | "ihrem"
                | "seine"
                | "seinen"
                | "seiner"
                | "deren"
        )
    });
    let pronoun = words
        .iter()
        .enumerate()
        .any(|(index, word)| match word.as_str() {
            "das" => words.get(index + 1).is_none_or(|next| {
                matches!(
                    next.as_str(),
                    "genau"
                        | "genauer"
                        | "weiter"
                        | "mit"
                        | "erklaeren"
                        | "machen"
                        | "finden"
                        | "nutzen"
                        | "starten"
                        | "einstellen"
                        | "denn"
                        | "auch"
                        | "funktioniert"
                        | "geht"
                        | "kostet"
                        | "verbessern"
                        | "spielen"
                )
            }),
            "es" | "sie" => words.get(index + 1).is_none_or(|next| {
                matches!(
                    next.as_str(),
                    "weiter" | "genau" | "genauer" | "nochmal" | "spielen" | "verbessern"
                )
            }),
            _ => false,
        });
    let continuation = words
        .first()
        .is_some_and(|word| matches!(word.as_str(), "und" | "auch"))
        && words.get(1).is_some_and(|word| {
            matches!(
                word.as_str(),
                "wie" | "wo" | "was" | "die" | "der" | "den" | "welche"
            )
        });
    let elaboration = words.first().is_some_and(|word| {
        matches!(
            word.as_str(),
            "erzaehl" | "erzaehle" | "erklaer" | "erklaere"
        )
    }) && words
        .last()
        .is_some_and(|word| matches!(word.as_str(), "mehr" | "genauer" | "nochmal" | "weiter"));
    let short = words.len() == 1
        && words.first().is_some_and(|word| {
            matches!(
                word.as_str(),
                "mehr" | "weiter" | "genauer" | "warum" | "wieso" | "wie" | "wo"
            )
        });
    deictic || pronoun || continuation || elaboration || short
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordReference {
    Independent,
    Subject(String),
    Clarification,
}

pub trait DiscordContextResolver: Send + Sync {
    fn resolve(
        &self,
        query: &crate::Query,
        context: &crate::AuthorizedContext,
        user_questions: &[String],
    ) -> Result<DiscordReference, crate::PortError>;
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
    fn followups_are_distinct_from_self_contained_questions() {
        for text in [
            "Wie mache ich das?",
            "Wie kann ich sie spielen?",
            "Wie kann ich das verbessern?",
            "Und seine Ult?",
            "Was kostet es?",
            "Wie geht es weiter?",
            "Wie geht das mit der Anmeldung?",
            "Was ist das genau?",
            "Wie spielt er?",
            "Wo finde ich sie?",
            "Wie melde ich mich dafür an?",
            "Erzähl mir mehr",
            "Warum?",
        ] {
            assert!(needs_reference(text), "{text}");
        }
        for text in [
            "Gibt es Coaching?",
            "Was ist das beste Item?",
            "Wie bekomme ich mehr Seelen?",
            "Wo finde ich einen Paten?",
        ] {
            assert!(!needs_reference(text), "{text}");
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
