use brain_contracts::{AnswerResponse, AnswerStatus, Query};

pub const COACHING_CHANNEL_ID: u64 = 1494373349944459355;
pub const COACHING_URL: &str = "https://deutsche-deadlock-community.de/coaching";
pub const COACHING_REFERENCE: &str = "[[coaching]]";

fn insufficient_text(english: bool, twitch_bot: bool, text: &str) -> String {
    let no_value = text == brain_contracts::answer_contract::NO_AUTHORIZED_VALUE;
    match (english, no_value, twitch_bot) {
        (false, true, _) => text,
        (true, true, _) => {
            "I have no verified value for that in the released game data. I don't guess numbers."
        }
        (true, false, true) => {
            "I don't have verified info on that right now. Please ask Nani on Discord."
        }
        (true, false, false) => {
            "I don't have verified info on that right now. Ask me about the game: heroes, items, builds or mechanics."
        }
        (false, false, true) => {
            "Dazu habe ich gerade keine gesicherten Infos. Frag bitte Nani im Discord."
        }
        (false, false, false) => {
            "Dazu habe ich gerade keine gesicherten Infos. Frag mich was zum Spiel: Held, Item, Build oder Mechanik."
        }
    }
    .into()
}

pub(super) fn prepare(answer: &mut AnswerResponse, query: &Query, discord: bool) {
    let english = brain_contracts::language::english(
        &brain_contracts::provider_input::current_question(query),
    );
    if matches!(
        answer.status,
        AnswerStatus::Answered | AnswerStatus::Unverified | AnswerStatus::BuildRejected
    ) {
        answer.text = brain_contracts::provider_input::discord_display_text(&answer.text);
    }
    if answer.status == AnswerStatus::Unverified {
        answer.text = format!(
            "{}{}",
            brain_contracts::public_api::UNVERIFIED_PREFIX,
            answer.text.trim()
        );
    }
    if answer.status == AnswerStatus::InsufficientEvidence {
        let question = query.text.to_lowercase();
        let twitch_bot = [
            "twitch-bot",
            "twitch bot",
            "bot auf twitch",
            "twitch-chatbefehl",
        ]
        .iter()
        .any(|term| question.contains(term));
        answer.text = insufficient_text(english, twitch_bot, &answer.text);
    } else if matches!(
        answer.status,
        AnswerStatus::Answered | AnswerStatus::Unverified | AnswerStatus::BuildRejected
    ) {
        let destination = if discord {
            format!("<#{}>", COACHING_CHANNEL_ID)
        } else {
            format!("Discord ({COACHING_URL})")
        };
        answer.text = answer.text.replace(COACHING_REFERENCE, &destination);
        answer.text = brain_contracts::discord_task::resolve_channel_references(
            &answer.text,
            &answer.citations,
            discord,
        );
        if answer.text.len() > brain_contracts::public_api::MAX_PUBLIC_ANSWER_TEXT_BYTES {
            let mut end = brain_contracts::public_api::MAX_PUBLIC_ANSWER_TEXT_BYTES;
            while !answer.text.is_char_boundary(end) {
                end -= 1;
            }
            answer.text.truncate(end);
        }
    } else if english {
        answer.text =
            "I couldn't answer that safely right now. Please try again in a moment.".into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{AnswerProfile, Usage, CONTRACT_VERSION};
    use std::collections::BTreeSet;

    fn query(text: &str) -> Query {
        Query {
            answer_context: None,
            request_id: "request".into(),
            conversation_id: "conversation".into(),
            text: text.into(),
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        }
    }

    fn answer(status: AnswerStatus, text: &str) -> AnswerResponse {
        AnswerResponse {
            contract_version: CONTRACT_VERSION.into(),
            request_id: "request".into(),
            knowledge_release: "release".into(),
            status,
            text: text.into(),
            citations: Vec::new(),
            usage: Usage::default(),
        }
    }

    #[test]
    fn unverified_text_is_delivered_with_legacy_wire_status_and_platform_owned_link() {
        let mut response = answer(
            AnswerStatus::Unverified,
            "Hilfreiche Antwort <@123456789012345678> [[coaching]]",
        );
        prepare(&mut response, &query("Pocket"), true);
        assert_eq!(response.status, AnswerStatus::Unverified);
        assert!(response
            .text
            .starts_with(brain_contracts::public_api::UNVERIFIED_PREFIX));
        assert!(!response.text.contains("<@"));
        assert!(!response.text.contains(COACHING_REFERENCE));
        let wire = super::super::answer_response(&response);
        assert_eq!(wire.status, 200);
        let public: brain_contracts::PublicAnswerResponse =
            serde_json::from_str(&wire.body).unwrap();
        assert_eq!(public.status, AnswerStatus::InsufficientEvidence);
        assert_eq!(public.text, response.text);
        assert!(public.citations.is_empty());
    }

    #[test]
    fn unverified_prefix_and_link_expansion_preserve_the_public_text_bound() {
        let limit = brain_contracts::public_api::MAX_PUBLIC_ANSWER_TEXT_BYTES;
        let text = format!(
            "{COACHING_REFERENCE}{}",
            "ü".repeat((limit - COACHING_REFERENCE.len()) / 2)
        );
        assert!(text.len() <= limit);
        for discord in [true, false] {
            let mut response = answer(AnswerStatus::Unverified, &text);
            prepare(&mut response, &query("Pocket"), discord);
            assert!(response
                .text
                .starts_with(brain_contracts::public_api::UNVERIFIED_PREFIX));
            assert!(response.text.len() <= limit);
            assert!(response.text.ends_with('ü'));
            assert!(!response.text.contains(COACHING_REFERENCE));
            let wire = super::super::answer_response(&response);
            assert_eq!(wire.status, 200);
            let public: brain_contracts::PublicAnswerResponse =
                serde_json::from_str(&wire.body).unwrap();
            public.validate("request").unwrap();
        }
    }

    #[test]
    fn deterministic_game_refusal_survives_public_text_mapping() {
        for discord in [true, false] {
            let mut response = answer(
                AnswerStatus::InsufficientEvidence,
                brain_contracts::answer_contract::NO_AUTHORIZED_VALUE,
            );
            prepare(&mut response, &query("Wie viel Leben hat Abrams?"), discord);
            let wire = super::super::answer_response(&response);
            assert_eq!(wire.status, 200);
            let public: brain_contracts::PublicAnswerResponse =
                serde_json::from_str(&wire.body).unwrap();
            assert_eq!(public.status, AnswerStatus::InsufficientEvidence);
            assert_eq!(
                public.text,
                brain_contracts::answer_contract::NO_AUTHORIZED_VALUE
            );
            assert!(public.citations.is_empty());
        }
    }

    #[test]
    fn spielfragen_verweisen_bei_wissensluecken_nicht_an_nani() {
        for question in [
            "wie countert man Pocket",
            "scalled haze auch mit Magic dmg?",
            "Bot, wie spiele ich Haze im Twitch-Stream?",
        ] {
            let mut response = answer(AnswerStatus::InsufficientEvidence, "Interner Hinweis");
            prepare(&mut response, &query(question), true);
            assert!(response.text.contains("Frag mich was zum Spiel"));
            assert!(!response.text.contains("Nani"));
            assert_eq!(response.status, AnswerStatus::InsufficientEvidence);
        }
    }

    #[test]
    fn nur_ausdrueckliche_twitch_bot_fragen_bekommen_den_nani_verweis() {
        let mut response = answer(AnswerStatus::InsufficientEvidence, "Interner Hinweis");
        prepare(
            &mut response,
            &query("Wie richte ich den Twitch-Bot ein?"),
            false,
        );
        assert!(response.text.contains("Nani im Discord"));
        let mut error = answer(AnswerStatus::Unavailable, "Dienst nicht verfügbar");
        prepare(
            &mut error,
            &query("Wie richte ich den Twitch-Bot ein?"),
            false,
        );
        assert_eq!(error.text, "Dienst nicht verfügbar");
        assert_eq!(error.status, AnswerStatus::Unavailable);
    }

    #[test]
    fn fixed_texts_follow_the_question_language() {
        for question in [
            "What changed in the latest patch?",
            "What changed for Baba in the latest patch?",
            "How much health does Seven have?",
            "What counters Infernus?",
            "What is Pocket's ultimate ability?",
        ] {
            assert!(brain_contracts::language::english(question), "{question}");
            let mut response = answer(AnswerStatus::InsufficientEvidence, "Interner Hinweis");
            prepare(&mut response, &query(question), false);
            assert!(response.text.starts_with("I don't have verified info"));
            let mut value = answer(
                AnswerStatus::InsufficientEvidence,
                brain_contracts::answer_contract::NO_AUTHORIZED_VALUE,
            );
            prepare(&mut value, &query(question), false);
            assert!(value.text.starts_with("I have no verified value"));
            let mut error = answer(
                AnswerStatus::Unavailable,
                "Werkzeugantwort konnte nicht sicher abgeschlossen werden.",
            );
            prepare(&mut error, &query(question), false);
            assert!(error.text.starts_with("I couldn't answer that"));
            assert_eq!(error.status, AnswerStatus::Unavailable);
        }
        for question in [
            "Was wurde im neuesten Patch geändert?",
            "Was macht das Item Unstoppable?",
            "Welche Items helfen gegen Haze?",
            "Wie schnell läuft Lash?",
            "Haze?",
        ] {
            assert!(!brain_contracts::language::english(question), "{question}");
        }
    }

    #[test]
    fn coaching_ziel_kommt_von_der_plattform_nicht_vom_modell() {
        for discord in [true, false] {
            let mut response = answer(
                AnswerStatus::Answered,
                "Für Coaching schau in [[coaching]]. Sag mir Bescheid, wenn du einen Paten willst.",
            );
            prepare(
                &mut response,
                &query("ich bin schlecht in dem Game was soll ich tun"),
                discord,
            );
            assert!(!response.text.contains(COACHING_REFERENCE));
            assert!(!response.text.contains("dem Concierge"));
            if discord {
                assert!(response.text.contains("<#1494373349944459355>"));
                assert!(!response.text.contains(COACHING_URL));
            } else {
                assert!(response.text.contains(&format!("Discord ({COACHING_URL})")));
                assert!(!response.text.contains("Discord oder"));
                assert!(!response.text.contains("<#"));
            }
        }
    }
}
