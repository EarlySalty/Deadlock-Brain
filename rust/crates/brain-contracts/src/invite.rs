use crate::{
    AnswerProfile, AuthorizedContext, DiscordRequestContext, Evidence, EvidenceKind, PortError,
    Query, SourceVisibility,
};
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
    let lower = query.text.to_lowercase();
    let text = lower
        .split_once("? chatkontext:")
        .map_or(lower.as_str(), |(question, _)| question)
        .trim()
        .trim_end_matches(['?', '.', '!'])
        .trim();
    if text.contains(['?', '.', '!', '\n', ';', ':', '"', '\'']) {
        return false;
    }
    let words: Vec<_> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect();
    let text = words.join(" ");
    if [
        "bin ich eingeladen",
        "bin ich schon eingeladen",
        "wurde ich eingeladen",
        "wurde ich schon eingeladen",
        "hat der invite bot mich schon eingeladen",
        "hat der invitebot mich schon eingeladen",
        "was i invited",
        "have i been invited",
        "am i invited",
        "did you send my invite",
        "how long has my invite been pending",
        "wie kann ich prüfen ob ich eingeladen bin",
        "wann bekomme ich deadlock",
        "wann bekomm ich deadlock",
        "habe ich zugang zu deadlock",
        "hab ich zugang zu deadlock",
        "habe ich schon zugang zu deadlock",
        "hab ich schon zugang zu deadlock",
    ]
    .contains(&text.as_str())
    {
        return true;
    }
    for subject in [
        "meine einladung",
        "meine deadlock einladung",
        "mein invite",
        "mein deadlock invite",
        "my invite",
        "my deadlock invite",
    ] {
        if [
            "wo ist",
            "wo bleibt",
            "where is",
            "wann bekomme ich",
            "wann erhalte ich",
        ]
        .iter()
        .any(|prefix| text == format!("{prefix} {subject}"))
        {
            return true;
        }
        for state in [
            "verschickt",
            "versandt",
            "versendet",
            "gesendet",
            "angekommen",
            "ausstehend",
            "abgelehnt",
            "sent",
            "pending",
            "missing",
            "rejected",
        ] {
            if ["ist", "wurde", "is", "was"].iter().any(|prefix| {
                text == format!("{prefix} {subject} {state}")
                    || text == format!("{prefix} {subject} schon {state}")
                    || text == format!("{prefix} {subject} noch {state}")
            }) {
                return true;
            }
        }
    }
    for subject in [
        "mein einladungsstatus",
        "mein invite status",
        "mein eigener einladungsstatus",
        "mein eigener invite status",
        "mein eigener deadlock invite status",
        "mein eigener deadlock einladungsstatus",
        "mein deadlock einladungsstatus",
        "der status meiner einladung",
        "my invite status",
        "my invitation status",
    ] {
        if ["wie ist", "was ist", "wie steht", "what is", "how is"]
            .iter()
            .any(|prefix| text == format!("{prefix} {subject}"))
        {
            return true;
        }
    }
    for subject in ["meinen invite status", "meinen einladungsstatus"] {
        if ["kann ich", "wie kann ich"]
            .iter()
            .any(|prefix| text == format!("{prefix} {subject} sehen"))
        {
            return true;
        }
    }
    false
}

pub fn project_query(query: &Query) -> Query {
    let mut projected = query.clone();
    if requested(query) {
        projected.text = QUESTION.into();
        projected.answer_context = None;
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
    if item.validate().is_err()
        || item.source_id != SOURCE
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

pub fn authorize<'a>(
    query: &Query,
    context: &'a AuthorizedContext,
    provider: bool,
) -> Result<&'a DiscordRequestContext, PortError> {
    let request = context.discord.as_ref().ok_or_else(invalid)?;
    if !requested(query)
        || query.validate().is_err()
        || query.conversation_id != context.conversation_id
        || !query.requested_scopes.contains("bot.public")
        || !context.principal.scopes.contains("bot.public")
        || request.user_id.is_none_or(|id| id == 0)
        || request.request_id != query.request_id
        || !request.scope.starts_with("discord.request:")
        || request.scope == "discord.request:unbound"
        || !context.principal.scopes.contains(&request.scope)
        || (provider
            && (!context.principal.provider_egress.contains("public")
                || !context
                    .principal
                    .provider_egress
                    .contains("discord_request")))
    {
        return Err(invalid());
    }
    Ok(request)
}

pub fn validate_projection(
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
    provider: bool,
) -> Result<SelfInviteStatus, PortError> {
    let request = authorize(query, context, provider)?;
    let status = projection(query, evidence)?;
    if evidence[0].allowed_scopes != BTreeSet::from([request.scope.clone()]) {
        return Err(invalid());
    }
    Ok(status)
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
    fn allgemeine_fremde_und_gemischte_fragen_bleiben_unveraendert() {
        for text in [
            "Wann sind Einladungen wieder verfügbar?",
            "When does an invite expire?",
            "When does my invite expire?",
            "Wie kann ich eine Einladung verschicken?",
            "Wie bekomme ich eine Einladung?",
            "Wie werde ich zu Deadlock eingeladen?",
            "Welche FPS bekomme ich in Deadlock?",
            "Wie funktioniert der Invite-Bot?",
            "Wie ist ein Einladungsstatus aufgebaut?",
            "Wie kann ich meinen Invite-Status ändern?",
            "Was kann ich tun, wenn meine Einladung noch aussteht?",
            "Hat Fremdname die Einladung erhalten?",
            "Did Fremdname get an invite?",
            "Wo bleibt der Invite für Fremdname?",
            "Ist der Invite schon verschickt?",
            "Bin ich eingeladen? Und wie verschicke ich eine Einladung?",
            "Bin ich eingeladen und ist Fremdname eingeladen?",
            "Bin ich eingeladen?\nWelche FPS bekomme ich?",
            "Was kann Abrams? Chatkontext: Ist meine Einladung verschickt?",
            "What does 'how long has my invite been pending' mean?",
            "Habe ich Zugang zu Deadlock-Server?",
            "Wann bekomme ich Deadlock-Skins?",
            "Was ist mein invite status in einem Bot?",
            "Ist die Gemeinschaftseinladung verschickt?",
            "Bin ich eingeladen? Chatkontextfrei: zweite Frage",
        ] {
            let ordinary = query(text);
            assert!(!requested(&ordinary), "{text}");
            assert_eq!(project_query(&ordinary), ordinary);
        }
    }

    #[test]
    fn eindeutiger_eigener_status_wird_projektiert() {
        for text in [
            QUESTION,
            "Bin ich eingeladen?",
            "BIN ICH SCHON EINGELADEN?",
            "Ist meine Einladung verschickt?",
            "Wo bleibt meine Einladung?",
            "Where is my invite?",
            "Was I invited?",
            "How long has my invite been pending?",
            "Wie ist mein Invite-Status?",
            "Kann ich meinen Invite-Status sehen?",
            "Habe ich schon Zugang zu Deadlock?",
            "Wann bekomme ich Deadlock?",
            "Hat der Invite-Bot mich schon eingeladen?",
            "Bin ich eingeladen? Chatkontext: privater Zusatz",
        ] {
            let mut raw = query(text);
            raw.profile = AnswerProfile::Build;
            raw.patch = Some("Privatname".into());
            raw.mode = Some("Geheimtext".into());
            assert!(requested(&raw), "{text}");
            let clean = project_query(&raw);
            assert_eq!(clean.text, QUESTION);
            assert_eq!(clean.profile, AnswerProfile::Explain);
            assert!(clean.domain.is_none() && clean.patch.is_none() && clean.mode.is_none());
            assert_eq!(clean.request_id, raw.request_id);
            assert_eq!(clean.conversation_id, raw.conversation_id);
        }
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
        for raw in [
            r#"{"status":"unknown","status":"sent","at":null}"#,
            r#"{"status":"sent","at":null,"at":"2026-01-01T12:00:00Z"}"#,
        ] {
            assert!(serde_json::from_str::<SelfInviteStatus>(raw).is_err());
        }
    }

    #[test]
    fn eigener_status_ist_an_person_request_scope_und_providerrecht_gebunden() {
        let mut query = query(QUESTION);
        query.requested_scopes.insert("bot.public".into());
        let context = AuthorizedContext {
            discord: Some(DiscordRequestContext {
                user_id: Some(42),
                request_id: query.request_id.clone(),
                scope: "discord.request:fixture".into(),
                allow_discord_reads: false,
            }),
            principal: crate::Principal {
                actor_id: "fixture".into(),
                channel: "discord".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:fixture".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: query.conversation_id.clone(),
            knowledge_release: "fixture".into(),
            deadline_ms: 1000,
            budget: crate::Budget::default(),
            request_deadline: None,
        };
        let item = status_evidence(
            &SelfInviteStatus {
                status: InviteStatus::Unknown,
                at: None,
            },
            "discord.request:fixture".into(),
        )
        .unwrap();
        let items = vec![item];
        assert!(validate_projection(&query, &context, &items, true).is_ok());
        for field in 0..6 {
            let mut changed = context.clone();
            match field {
                0 => changed.discord.as_mut().unwrap().user_id = None,
                1 => changed.discord.as_mut().unwrap().request_id = "other".into(),
                2 => changed.discord.as_mut().unwrap().scope = "discord.request:other".into(),
                3 => changed.principal.scopes.clear(),
                4 => changed.principal.provider_egress.clear(),
                _ => changed.conversation_id = "other".into(),
            }
            assert!(validate_projection(&query, &changed, &items, true).is_err());
        }
        let mut mixed = items.clone();
        mixed.push(items[0].clone());
        assert!(validate_projection(&query, &context, &mixed, false).is_err());
        query.text = "Wie funktioniert der Invite-Bot?".into();
        assert!(validate_projection(&query, &context, &items, false).is_err());
    }

    #[test]
    fn beide_providerformate_enthalten_nur_die_eigene_enum_und_zeit() {
        use crate::provider_input::{grounded_turn_payload, ToolWireFormat};
        let mut query = query("Bin ich eingeladen? Chatkontext: private Kennung 76561197960265839");
        query.answer_context = Some(crate::AnswerContext::Discord(crate::DiscordAnswerContext {
            purpose: Some("private Ortskennung".into()),
            ..Default::default()
        }));
        assert!(project_query(&query).answer_context.is_none());
        let item = status_evidence(
            &SelfInviteStatus {
                status: InviteStatus::Pending,
                at: Some("2026-10-02T12:00:00Z".into()),
            },
            "discord.request:private-kennung".into(),
        )
        .unwrap();
        for format in [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible] {
            let payload = grounded_turn_payload(
                &query,
                std::slice::from_ref(&item),
                &[],
                &crate::ToolConversation::default(),
                format,
            )
            .unwrap();
            let raw = payload.to_string();
            for private in [
                "private Kennung",
                "private Ortskennung",
                "76561197960265839",
                "discord.request:",
            ] {
                assert!(!raw.contains(private));
            }
            let messages = payload["messages"].as_array().unwrap();
            let data: serde_json::Value =
                serde_json::from_str(messages.last().unwrap()["content"].as_str().unwrap())
                    .unwrap();
            assert_eq!(data["query"], QUESTION);
            assert!(data.get("answer_context").is_none());
            assert_eq!(data["evidence"].as_array().unwrap().len(), 1);
            let projected: SelfInviteStatus =
                serde_json::from_str(data["evidence"][0]["content"].as_str().unwrap()).unwrap();
            assert_eq!(projected.status, InviteStatus::Pending);
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
