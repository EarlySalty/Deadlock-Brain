use crate::{AnswerProfile, Evidence, EvidenceKind, PortError, Query, SourceVisibility};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeSet;

pub const SOURCE: &str = "discord.self-invite.v1";
pub const EVIDENCE_ID: &str = "self-invite-status";
pub const QUESTION: &str = "Wie ist mein eigener Deadlock-Einladungsstatus?";
pub const CITATION: &str = "Eigener Einladungsstatus";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    Sent,
    Pending,
    FriendshipMissing,
    AlreadyHasGame,
    Error,
    Unknown,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelfInviteStatus {
    pub status: InviteStatus,
    #[serde(deserialize_with = "timestamp")]
    pub at: Option<String>,
}

fn timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let at = Option::<String>::deserialize(deserializer)?;
    if let Some(value) = &at {
        chrono::DateTime::parse_from_rfc3339(value).map_err(serde::de::Error::custom)?;
    }
    Ok(at)
}

pub fn requested(query: &Query) -> bool {
    let text = query.text.to_lowercase();
    let invite = [
        "einlad",
        "invite",
        "beta-zugang",
        "betazugang",
        "zugang zu deadlock",
    ]
    .iter()
    .any(|term| text.contains(term));
    let procedural = [
        "wie bekomme",
        "wie bekomm",
        "wie bekommt",
        "wie erhalte",
        "wie erhält",
        "wie kann",
        "wie funktioniert",
        "wie werde",
        "wie wird",
        "how can",
        "how do",
        "how to",
    ]
    .iter()
    .any(|term| text.trim_start().starts_with(term));
    let explicit_status = [
        "status",
        "schon",
        "noch nicht",
        "verschick",
        "versand",
        "versandt",
        "gesendet",
        "aussteh",
        "pending",
        "fehler",
        "failed",
        "warte",
        "waiting",
        "sent",
        "missing",
    ]
    .iter()
    .any(|term| text.contains(term));
    if procedural
        && !explicit_status
        && !text.contains("ob ")
        && !text.contains("whether")
        && !text.contains("if my")
    {
        return false;
    }
    let status = [
        "status",
        "wann",
        "wo bleibt",
        "wo ist",
        "where is",
        "schon",
        "noch",
        "verschick",
        "versand",
        "versandt",
        "gesendet",
        "erhalt",
        "bekommen",
        "ankomm",
        "angekomm",
        "zugeschickt",
        "raus",
        "abgelehnt",
        "received",
        "failed",
        "when",
        "did ",
        "has ",
        "have ",
        "aussteh",
        "pending",
        "freundschaft",
        "fehler",
        "warte",
        "warten",
        "waiting",
        "sent",
        "missing",
        "meine einladung",
        "mein invite",
        "my invite",
        "eingeladen",
        "invited",
        "rejected",
        "angenommen",
    ]
    .iter()
    .any(|term| text.contains(term));
    (invite && status)
        || (text.contains("deadlock")
            && [
                "bekomme ich",
                "bekomm ich",
                "habe ich zugang",
                "hab ich zugang",
            ]
            .iter()
            .any(|term| text.contains(term)))
}

pub fn project_query(query: &Query) -> Query {
    let mut projected = query.clone();
    if requested(query) {
        projected.text = QUESTION.into();
        projected.domain = None;
        projected.profile = AnswerProfile::Explain;
        projected.patch = None;
        projected.mode = None;
    }
    projected
}

pub fn status_evidence(status: &SelfInviteStatus, scope: String) -> Result<Evidence, PortError> {
    let content = serde_json::to_string(status).map_err(|_| invalid())?;
    serde_json::from_str::<SelfInviteStatus>(&content).map_err(|_| invalid())?;
    Ok(Evidence {
        evidence_id: EVIDENCE_ID.into(),
        source_id: SOURCE.into(),
        logical_id: EVIDENCE_ID.into(),
        revision: 1,
        kind: EvidenceKind::Fact,
        content,
        citation: CITATION.into(),
        visibility: SourceVisibility::RequestScoped,
        allowed_scopes: BTreeSet::from([scope]),
        score: 100.0,
        provenance: None,
        patch: None,
    })
}

pub fn projection(query: &Query, evidence: &[Evidence]) -> Result<SelfInviteStatus, PortError> {
    if !requested(query) || evidence.len() != 1 {
        return Err(invalid());
    }
    let item = &evidence[0];
    if item.source_id != SOURCE
        || item.evidence_id != EVIDENCE_ID
        || item.logical_id != EVIDENCE_ID
        || item.revision != 1
        || item.kind != EvidenceKind::Fact
        || item.citation != CITATION
        || item.visibility != SourceVisibility::RequestScoped
        || item.allowed_scopes.len() != 1
        || item.provenance.is_some()
        || item.patch.is_some()
    {
        return Err(invalid());
    }
    serde_json::from_str(&item.content).map_err(|_| invalid())
}

fn invalid() -> PortError {
    PortError::PermissionDenied("Eigener Einladungsstatus ist nicht sicher belegt.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn query(text: &str) -> Query {
        serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":text})).unwrap()
    }

    #[test]
    fn nur_enum_und_pflichtzeitpunkt_sind_zulaessig() {
        for status in [
            "sent",
            "pending",
            "friendship_missing",
            "already_has_game",
            "error",
            "unknown",
            "unavailable",
        ] {
            for at in [json!(null), json!("2026-10-06T23:25:00+02:00")] {
                let value = json!({"status":status,"at":at});
                let parsed: SelfInviteStatus = serde_json::from_value(value.clone()).unwrap();
                assert_eq!(serde_json::to_value(parsed).unwrap(), value);
            }
        }
        for value in [
            json!({"status":"accepted","at":null}),
            json!({"status":"sent"}),
            json!({"status":"sent","at":"privater Freitext"}),
            json!({"status":"sent","at":"2026-02-30T12:00:00Z"}),
            json!({"status":"sent","at":null,"steam_id":"fremd"}),
            json!({"status":"error","at":null,"error":"privat"}),
        ] {
            assert!(serde_json::from_value::<SelfInviteStatus>(value).is_err());
        }
    }

    #[test]
    fn doppelte_transportfelder_sind_uneindeutig() {
        for raw in [
            r#"{"status":"unknown","status":"sent","at":null}"#,
            r#"{"status":"sent","at":null,"at":"2026-01-01T12:00:00Z"}"#,
        ] {
            assert!(serde_json::from_str::<SelfInviteStatus>(raw).is_err());
        }
    }

    #[test]
    fn personenwahl_und_freie_parameter_bleiben_intern() {
        let mut raw = query("Invite-Status für Fremdname, Steamcode 123456, Kontext Geheimtext?");
        raw.profile = AnswerProfile::Build;
        raw.patch = Some("Privatname".into());
        raw.mode = Some("Geheimtext".into());
        let clean = project_query(&raw);
        assert_eq!(clean.text, QUESTION);
        assert_eq!(clean.profile, AnswerProfile::Explain);
        assert!(clean.domain.is_none() && clean.patch.is_none() && clean.mode.is_none());
        assert_eq!(clean.request_id, raw.request_id);
        assert_eq!(clean.conversation_id, raw.conversation_id);
        for text in [
            "Was kann Abrams?",
            "Wie bekommt man eine Einladung?",
            "Wie bekomme ich eine Einladung?",
            "Wie erhalte ich eine Einladung?",
            "Wie kann ich eine Einladung erhalten?",
            "How do I get an invite?",
            "Wie funktioniert der Invite-Bot?",
            "Wie werde ich zu Deadlock eingeladen?",
            "Wie wird man eingeladen?",
        ] {
            let ordinary = query(text);
            assert_eq!(project_query(&ordinary), ordinary);
        }
        for text in [
            "Wann bekomme ich meine Einladung?",
            "Wo bleibt der Invite für Fremdname?",
            "Where is my invite?",
            "Ist der Invite schon verschickt?",
            "Wie ist mein Invite-Status?",
            "Wie kann ich meinen Invite-Status sehen?",
            "Wie bekomme ich den Status meiner gesendeten Einladung?",
            "Wann bekomme ich Deadlock?",
            "Ich warte auf eine Einladung.",
            "Hat Fremdname die Einladung erhalten, Steamcode 123456?",
            "Did Fremdname get an invite?",
            "Bin ich eingeladen?",
            "Bin ich eingeladen? Chatkontext Fremdname: Wie kann ich eine Einladung erhalten?",
            "Wurde Fremdname eingeladen, Steamcode 123456?",
            "Was I invited?",
            "Wie kann ich prüfen, ob ich eingeladen bin?",
        ] {
            assert!(requested(&query(text)));
        }
    }

    #[test]
    fn keine_gemischten_oder_gefaelschten_providerbelege() {
        let query = query(QUESTION);
        let evidence = status_evidence(
            &SelfInviteStatus {
                status: InviteStatus::Pending,
                at: None,
            },
            "request-scope".into(),
        )
        .unwrap();
        assert!(projection(&query, std::slice::from_ref(&evidence)).is_ok());
        assert!(projection(&query, &[evidence.clone(), evidence.clone()]).is_err());
        let mut changed = evidence.clone();
        changed.citation = "Fremdname".into();
        assert!(projection(&query, &[changed]).is_err());
        let mut changed = evidence;
        changed.content = json!({"status":"pending","at":null,"discord_id":"privat"}).to_string();
        assert!(projection(&query, &[changed]).is_err());
    }
}
