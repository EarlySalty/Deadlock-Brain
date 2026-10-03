//! Begrenzte öffentliche Discordquelle im vorhandenen kanonischen Wissensspeicher.
use super::*;
use brain_contracts::SourceRecordV2;
use serde_json::Value;
use std::collections::BTreeMap;

impl GuideRuntime {
    pub(super) fn sync_snapshot(
        &self,
        mut snapshot: ServerSnapshot,
        deadline: &RequestDeadline,
    ) -> Result<(), PortError> {
        let current = now();
        if snapshot.guild_id != self.config.guild_id
            || !self.config.enabled
            || snapshot.revision == 0
            || snapshot.revision > i64::MAX as u64
            || snapshot.observed_at > current + 30
            || snapshot.observed_at < current - self.config.server_snapshot_max_age_seconds
            || snapshot.channels.len() > 100
            || snapshot.roles.len() > 100
            || snapshot.rules.len() > 100
        {
            return Err(PortError::PermissionDenied(
                "Serverquelle ist nicht freigegeben".into(),
            ));
        }
        let mut channels = BTreeSet::new();
        for channel in &snapshot.channels {
            if !snowflake(&channel.id)
                || !self.config.approved_discord_channels.contains(&channel.id)
                || !channels.insert(&channel.id)
                || channel.name.len() > 100
                || channel.name.chars().any(char::is_control)
                || channel.kind.len() > 32
            {
                return Err(PortError::PermissionDenied(
                    "Kanalquelle ist nicht freigegeben".into(),
                ));
            }
        }
        let mut messages = BTreeSet::new();
        for rule in &mut snapshot.rules {
            if snapshot
                .channels
                .iter()
                .any(|c| c.id == rule.channel_id && (c.deleted || !c.public_readable))
                || rule.deleted
            {
                rule.text.clear();
                rule.deleted = true;
            }
            if !snowflake(&rule.channel_id)
                || !snowflake(&rule.message_id)
                || !self
                    .config
                    .approved_rule_channels
                    .contains(&rule.channel_id)
                || !snapshot.channels.iter().any(|c| c.id == rule.channel_id)
                || !messages.insert(&rule.message_id)
                || rule.text.len() > 16000
                || rule.updated_at > current + 30
            {
                return Err(PortError::PermissionDenied(
                    "Regelquelle ist nicht freigegeben".into(),
                ));
            }
        }
        for role in &snapshot.roles {
            if !snowflake(&role.id)
                || !role.public
                || role.name.len() > 100
                || role.name.chars().any(char::is_control)
            {
                return Err(PortError::PermissionDenied(
                    "Rollenquelle ist nicht freigegeben".into(),
                ));
            }
        }
        // Widerrufene Metadaten und gelöschte Texte bleiben auch im kanonischen
        // öffentlichen Snapshot nicht als alte Inhalte erhalten.
        for channel in &mut snapshot.channels {
            if channel.deleted || !channel.public_readable {
                channel.name.clear();
                channel.kind.clear();
            }
        }
        for rule in &mut snapshot.rules {
            if rule.deleted {
                rule.text.clear();
            }
        }
        let content = serde_json::to_string(&snapshot)
            .map_err(|_| PortError::InvalidResponse("Serverquelle ist ungültig".into()))?;
        if content.len() > 100000 {
            return Err(PortError::InvalidResponse(
                "Serverquelle ist zu groß".into(),
            ));
        }
        let record = SourceRecordV2 {
            source_id: format!("guide-discord:{}", snapshot.guild_id),
            logical_id: "server".into(),
            revision: snapshot.revision,
            content_hash: format!("{:x}", Sha256::digest(content.as_bytes())),
            content,
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            tombstone: false,
            valid_from: Some(snapshot.observed_at.to_string()),
            valid_to: Some(
                (snapshot.observed_at + self.config.server_snapshot_max_age_seconds).to_string(),
            ),
            metadata: BTreeMap::from([
                ("source_class".into(), "server_documentation".into()),
                ("observed_at".into(), snapshot.observed_at.to_string()),
                ("kind".into(), "prose".into()),
                (
                    "upstream".into(),
                    "Discord v10: freigegebene öffentliche Kanäle und Regeln".into(),
                ),
            ]),
        };
        self.reader.guide_sync_server(&snapshot, &record, deadline)
    }
    pub(super) fn current_server_evidence(
        &self,
        deadline: &RequestDeadline,
    ) -> Result<Option<Evidence>, PortError> {
        let Some(record) = self
            .reader
            .guide_server_record(&self.config.guild_id, deadline)?
        else {
            return Ok(None);
        };
        let snapshot: ServerSnapshot = serde_json::from_str(&record.content)
            .map_err(|_| PortError::InvalidResponse("Serverquelle ist ungültig".into()))?;
        if snapshot.observed_at < now() - self.config.server_snapshot_max_age_seconds {
            return Ok(None);
        }
        let available: BTreeSet<_> = snapshot
            .channels
            .iter()
            .filter(|c| {
                c.public_readable
                    && !c.deleted
                    && self.config.approved_discord_channels.contains(&c.id)
            })
            .map(|c| c.id.clone())
            .collect();
        let mut text=String::from("Aktueller geprüfter Serverstand. Nur hier aufgeführte Wege sind derzeit verifiziert:\n");
        for (name, id) in [
            ("Mitspieler", &self.config.mates_channel_id),
            ("Voice-Runden", &self.config.voice_channel_id),
            ("Coaching", &self.config.coaching_channel_id),
            ("Streamer", &self.config.streamer_channel_id),
            ("Mitarbeit und Ideen", &self.config.team_channel_id),
            ("Allgemeine Hilfe", &self.config.faq_channel_id),
            ("Ticket erstellen", &self.config.ticket_channel_id),
        ] {
            if available.contains(id) {
                text.push_str(&format!("{name}: <#{id}>\n"));
            }
        }
        for role in &snapshot.roles {
            if role.public {
                text.push_str(&format!("Öffentliche Ansprechpartnerrolle: {} (keine Verfügbarkeits- oder Sicherheitsgarantie).\n",role.name));
            }
        }
        for rule in &snapshot.rules {
            if !rule.deleted
                && available.contains(&rule.channel_id)
                && self
                    .config
                    .approved_rule_channels
                    .contains(&rule.channel_id)
            {
                text.push_str(&format!(
                    "Öffentliche Regel aus <#{}>, Nachricht {}, Stand {}: {}\n",
                    rule.channel_id, rule.message_id, rule.updated_at, rule.text
                ));
            }
        }
        Ok(Some(Evidence {
            evidence_id: format!("discord-server-{}", record.revision),
            source_id: record.source_id,
            logical_id: record.logical_id,
            revision: record.revision,
            kind: EvidenceKind::Rule,
            content: text,
            citation: format!("Öffentlicher Discordstand {}", snapshot.observed_at),
            visibility: SourceVisibility::Public,
            allowed_scopes: record.allowed_scopes,
            score: 1.0,
            provenance: None,
            patch: None,
        }))
    }
    pub(super) fn verify_server_evidence(
        &self,
        evidence: &[Evidence],
        deadline: &RequestDeadline,
    ) -> Result<(), PortError> {
        for item in evidence
            .iter()
            .filter(|e| e.source_id.starts_with("guide-discord:"))
        {
            let current = self.current_server_evidence(deadline)?.ok_or_else(|| {
                PortError::PermissionDenied("Serverquelle ist veraltet oder widerrufen".into())
            })?;
            if current.revision != item.revision || current.content != item.content {
                return Err(PortError::PermissionDenied(
                    "Serverquelle wurde geändert".into(),
                ));
            }
        }
        Ok(())
    }
}
pub(super) async fn snapshot_handler(
    State(runtime): State<Arc<GuideRuntime>>,
    headers: HeaderMap,
    Json(snapshot): Json<ServerSnapshot>,
) -> Result<Json<Value>, StatusCode> {
    runtime.authorized(&headers)?;
    let deadline = RequestDeadline::after(Duration::from_millis(runtime.deadline_ms));
    tokio::task::spawn_blocking(move || runtime.sync_snapshot(snapshot, &deadline))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(
        json!({"contract_version":GUIDE_VERSION,"status":"accepted"}),
    ))
}
