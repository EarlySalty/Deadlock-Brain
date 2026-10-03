//! Einladungen verwenden ausschließlich den bestehenden Steam-Dienst.
use super::*;
use brain_storage::GuideInviteGrant;
use serde::{de::DeserializeOwned, Deserialize};
use std::io::Read;
fn bounded_response<T: DeserializeOwned>(
    response: reqwest::blocking::Response,
) -> Result<T, PortError> {
    let response = response
        .error_for_status()
        .map_err(|_| PortError::Unavailable("Einladungsdienst nicht erreichbar".into()))?;
    let mut bytes = Vec::new();
    response
        .take(4097)
        .read_to_end(&mut bytes)
        .map_err(|_| PortError::Unavailable("Einladungsantwort fehlt".into()))?;
    if bytes.len() > 4096 {
        return Err(PortError::InvalidResponse(
            "Einladungsantwort ist zu groß".into(),
        ));
    }
    serde_json::from_slice(&bytes)
        .map_err(|_| PortError::InvalidResponse("Ungültige Einladungsantwort".into()))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NormalizedCode {
    account_id: u32,
    steam_id64: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DispatchResult {
    action_id: String,
    turn_id: String,
    status: InviteStatus,
}

pub(super) fn command(text: &str) -> Option<AccessInviteCommand> {
    let trimmed = text.trim();
    let lower = trimmed.to_lowercase();
    for prefix in ["einladen ", "bitte lade mich ein: ", "bitte lade mich ein "] {
        if lower.starts_with(prefix) {
            let code = trimmed.get(prefix.len()..)?.trim();
            if code.split_whitespace().count() == 1 {
                return Some(AccessInviteCommand::Request {
                    friend_code: code.into(),
                });
            }
        }
    }
    if [
        "kann mich jemand einladen",
        "ich möchte eine deadlock-einladung",
        "ich möchte deadlock spielen",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
        && !["nicht", "kein", "für jemand", "für meinen", "für meine"]
            .iter()
            .any(|word| lower.contains(word))
    {
        if let Some(index) = lower.find("mein freundescode:") {
            let code = trimmed.get(index + "mein freundescode:".len()..)?.trim();
            if code.split_whitespace().count() == 1 {
                return Some(AccessInviteCommand::Request {
                    friend_code: code.into(),
                });
            }
        }
    }
    for (prefix, cancel) in [("einladungsstatus ", false), ("einladung abbrechen ", true)] {
        if lower.starts_with(prefix) {
            let action_id = trimmed.get(prefix.len()..)?.trim().to_owned();
            return Some(if cancel {
                AccessInviteCommand::Cancel { action_id }
            } else {
                AccessInviteCommand::Status { action_id }
            });
        }
    }
    if matches!(
        lower.trim_end_matches(['?', '.', '!']),
        "kann mich jemand einladen" | "ich möchte eine deadlock-einladung"
    ) {
        return Some(AccessInviteCommand::AwaitCode);
    }
    None
}

pub(super) fn code(text: &str) -> Option<AccessInviteCommand> {
    let trimmed = text.trim();
    let lower = trimmed.to_lowercase();
    for prefix in ["mein freundescode:", "freundescode:"] {
        if lower.starts_with(prefix) {
            let friend_code = trimmed.get(prefix.len()..)?.trim();
            if friend_code.split_whitespace().count() == 1 {
                return Some(AccessInviteCommand::Request {
                    friend_code: friend_code.into(),
                });
            }
        }
    }
    None
}

impl GuideRuntime {
    pub(super) fn access_invite(
        &self,
        turn: &GuideTurn,
        epoch: i64,
        command: AccessInviteCommand,
        deadline: &RequestDeadline,
    ) -> Result<GuideResult, PortError> {
        let mut conversation = GuideConversation {
            id: turn
                .conversation_id
                .clone()
                .unwrap_or_else(|| format!("guide:{}", hex_action(turn))),
            channel_id: turn.channel_id.clone(),
            thread_id: turn.thread_id.clone(),
            user_id: turn.user_id.clone(),
            surface: turn.surface,
            last_user_message_id: turn.message_id.clone(),
            last_bot_message_id: None,
            expires_at: now() + self.config.conversation_inactivity_seconds,
            closed: false,
            pending_access_invite: false,
        };
        if !self.config.access_invites_enabled
            || !command.valid()
            || (turn.surface == Surface::Public
                && (turn.guild_id != "1289721245281292288"
                    || turn.channel_id != "1426220702054355077"
                    || turn.thread_id.is_some()))
        {
            if !self
                .reader
                .guide_finish(turn, epoch, Some(&conversation), &[], None, deadline)?
            {
                return Ok(GuideResult::silent(&turn.request_id));
            }
            let mut result = GuideResult::reply(
                &turn.request_id,
                "Spieleinladungen über mich sind noch nicht freigeschaltet.".into(),
            );
            result.conversation_id = Some(conversation.id);
            return Ok(result);
        }
        match command {
            AccessInviteCommand::AwaitCode => {
                conversation.pending_access_invite = true;
                if !self.reader.guide_finish(
                    turn,
                    epoch,
                    Some(&conversation),
                    &[],
                    None,
                    deadline,
                )? {
                    return Ok(GuideResult::silent(&turn.request_id));
                }
                let mut result=GuideResult::reply(&turn.request_id,"Schick mir deinen eigenen Steam-Freundescode als „Mein Freundescode: …“, dann kann ich die Einladung für dich anstoßen.".into());
                result.conversation_id = Some(conversation.id);
                Ok(result)
            }
            AccessInviteCommand::Request { friend_code } => {
                let base =
                    self.config.steam_service_url.as_deref().ok_or_else(|| {
                        PortError::InvalidResponse("Einladungsdienst fehlt".into())
                    })?;
                let token = self
                    .peer_token
                    .as_deref()
                    .ok_or_else(|| PortError::PermissionDenied("Einladungszugang fehlt".into()))?;
                let http = reqwest::blocking::Client::builder()
                    .redirect(reqwest::redirect::Policy::none())
                    .timeout(deadline.remaining()?)
                    .build()
                    .map_err(|_| {
                        PortError::InvalidResponse("Einladungsdienst nicht erreichbar".into())
                    })?;
                let normalized: NormalizedCode = http
                    .post(format!(
                        "{}/internal/guide/access-invite/normalize",
                        base.trim_end_matches('/')
                    ))
                    .bearer_auth(token)
                    .json(&json!({"friend_code":friend_code}))
                    .send()
                    .map_err(|_| {
                        PortError::Unavailable("Freundescode konnte nicht geprüft werden".into())
                    })
                    .and_then(bounded_response)?;
                let target = normalized
                    .steam_id64
                    .parse::<i64>()
                    .map_err(|_| PortError::InvalidResponse("Ungültiges Steamziel".into()))?;
                if normalized.account_id == 0
                    || target != 76561197960265728_i64 + i64::from(normalized.account_id)
                {
                    return Err(PortError::InvalidResponse(
                        "Widersprüchliches Steamziel".into(),
                    ));
                }
                deadline.check()?;
                let action = hex_action(turn);
                let ttl = self
                    .config
                    .access_invite_ttl_seconds
                    .ok_or_else(|| PortError::InvalidResponse("Einladungsablauf fehlt".into()))?;
                let Some(correlation) = self.reader.guide_create_invite_grant(
                    turn,
                    epoch,
                    &GuideInviteGrant {
                        action_id: action.clone(),
                        target,
                        ttl_seconds: ttl as i64,
                        conversation: conversation.clone(),
                    },
                    deadline,
                )?
                else {
                    return Ok(GuideResult::silent(&turn.request_id));
                };
                // Kein Mitgliedslock liegt während dieses HTTP-Aufrufs auf dem Brain-Pfad.
                let sent = match deadline.remaining() {
                    Ok(remaining) => http
                        .post(format!(
                            "{}/internal/guide/access-invite",
                            base.trim_end_matches('/')
                        ))
                        .timeout(remaining)
                        .bearer_auth(token)
                        .json(&json!({"correlation":correlation,"friend_code":friend_code}))
                        .send()
                        .map_err(|_| {
                            PortError::Unavailable("Einladungsübergabe ist nicht bestätigt".into())
                        })
                        .and_then(bounded_response::<DispatchResult>),
                    Err(error) => Err(error),
                };
                let confirmed = sent.is_ok_and(|response| {
                    response.action_id == action
                        && response.turn_id == turn.request_id
                        && matches!(
                            response.status,
                            InviteStatus::Queued
                                | InviteStatus::FriendRequestSent
                                | InviteStatus::WaitingForAcceptance
                                | InviteStatus::InviteSent
                                | InviteStatus::AlreadyHasAccess
                        )
                });
                let unconfirmed="Die Übergabe ist nicht bestätigt. Ich starte keinen zweiten Versand. Du kannst den Auftragsstatus prüfen.";
                let text = if confirmed {
                    match self
                        .reader
                        .guide_invite_state(turn, epoch, &action, false, deadline)
                    {
                        Ok(Some(status)) => status_text(status),
                        Ok(None) => "Der Auftrag ist nicht mehr freigegeben.",
                        Err(_) => unconfirmed,
                    }
                } else {
                    unconfirmed
                };
                let mut result =
                    GuideResult::reply(&turn.request_id, format!("{text}\nAuftrag: `{action}`"));
                result.conversation_id = Some(conversation.id);
                Ok(result)
            }
            AccessInviteCommand::Status { action_id }
            | AccessInviteCommand::Cancel { action_id } => {
                let cancel = matches!(
                    turn.access_invite.as_ref(),
                    Some(AccessInviteCommand::Cancel { .. })
                ) || command_is_cancel(&turn.content);
                let status = self
                    .reader
                    .guide_invite_state(turn, epoch, &action_id, cancel, deadline)?;
                if !self.reader.guide_finish(
                    turn,
                    epoch,
                    Some(&conversation),
                    &[],
                    None,
                    deadline,
                )? {
                    return Ok(GuideResult::silent(&turn.request_id));
                }
                let mut result = GuideResult::reply(
                    &turn.request_id,
                    status
                        .map(status_text)
                        .unwrap_or("Für diesen Auftrag kann ich dir hier keinen Status zeigen.")
                        .into(),
                );
                result.conversation_id = Some(conversation.id);
                Ok(result)
            }
        }
    }
}
fn command_is_cancel(text: &str) -> bool {
    text.trim()
        .to_lowercase()
        .starts_with("einladung abbrechen ")
}
fn hex_action(turn: &GuideTurn) -> String {
    format!(
        "invite:{:x}",
        Sha256::digest(
            format!("{}:{}:{}", turn.guild_id, turn.user_id, turn.message_id).as_bytes()
        )
    )
}
fn status_text(status: InviteStatus) -> &'static str {
    match status {
        InviteStatus::Queued=>"Dein Einladungsauftrag ist angenommen.",
        InviteStatus::FriendRequestSent|InviteStatus::WaitingForAcceptance=>"Die Freundschaftsanfrage ist versendet. Die Spieleinladung wartet auf die Annahme.",
        InviteStatus::InviteSent=>"Die Spieleinladung ist versendet.",
        InviteStatus::AlreadyHasAccess=>"Für dieses Steamkonto ist der Spielzugang bereits bestätigt.",
        InviteStatus::Failed=>"Der Einladungsversuch ist fehlgeschlagen.",
        InviteStatus::Unknown=>"Ob der Versand erfolgt ist, lässt sich gerade nicht bestätigen. Ich versende nicht erneut.",
        InviteStatus::Expired=>"Der Einladungsauftrag ist abgelaufen.",
        InviteStatus::Cancelled=>"Der Auftrag ist abgebrochen. Eine schon versendete Anfrage wird dadurch nicht zurückgenommen.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nur_konkreter_eigener_auftrag_erzeugt_eine_einladung() {
        assert!(
            matches!(command("Bitte lade mich ein: ABCD"),Some(AccessInviteCommand::Request{friend_code}) if friend_code=="ABCD")
        );
        assert!(matches!(
            command("Kann mich jemand einladen?"),
            Some(AccessInviteCommand::AwaitCode)
        ));
        assert!(
            matches!(code("Mein Freundescode: ABCD"),Some(AccessInviteCommand::Request{friend_code}) if friend_code=="ABCD")
        );
        assert!(code("ABCD").is_none());
        assert!(code("Mein Freundescode: ABCD für jemand anderen").is_none());
        assert!(matches!(
            command("Kann mich jemand einladen? Mein Freundescode: ABCD"),
            Some(AccessInviteCommand::Request { .. })
        ));
        for text in [
            "Lade ihn ein ABCD",
            "Bitte lade mich nicht ein: ABCD",
            "Bitte lade mich ein: ABCD für jemand anderen",
            "Hat jemand einen Code?",
        ] {
            assert!(command(text).is_none());
        }
        assert!(matches!(
            command("Einladung abbrechen invite:123"),
            Some(AccessInviteCommand::Cancel { .. })
        ));
    }
}
