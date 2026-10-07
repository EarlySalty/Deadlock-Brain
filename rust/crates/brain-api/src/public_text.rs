use brain_contracts::{AnswerResponse, AnswerStatus, Query};

pub const COACHING_CHANNEL_ID: u64 = 1494373349944459355;
pub const COACHING_URL: &str = "https://deutsche-deadlock-community.de/coaching";
pub const COACHING_REFERENCE: &str = "[[coaching]]";

pub(super) fn prepare(answer: &mut AnswerResponse, query: &Query, discord: bool) {
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
        answer.text = if twitch_bot {
            "Dazu habe ich gerade keine gesicherten Infos. Frag bitte Nani im Discord."
        } else {
            "Dazu habe ich gerade keine gesicherten Infos. Frag mich was zum Spiel: Held, Item, Build oder Mechanik."
        }
        .into();
    } else if matches!(
        answer.status,
        AnswerStatus::Answered | AnswerStatus::BuildRejected
    ) {
        let destination = if discord {
            format!("<#{}>", COACHING_CHANNEL_ID)
        } else {
            format!("Discord oder {COACHING_URL}")
        };
        answer.text = answer.text.replace(COACHING_REFERENCE, &destination);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{AnswerProfile, Usage, CONTRACT_VERSION};
    use std::collections::BTreeSet;

    fn query(text: &str) -> Query {
        Query {
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
                assert!(response.text.contains(COACHING_URL));
                assert!(!response.text.contains("<#"));
            }
        }
    }
}
