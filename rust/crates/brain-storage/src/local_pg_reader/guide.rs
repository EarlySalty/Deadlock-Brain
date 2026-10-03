//! Profile ausschließlich im Brain. Revisionen verhindern spätere Wiederanlage.
use super::{invalid, LocalPgReader};
use brain_contracts::{guide::*, PortError, RequestDeadline};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct GuideSnapshot {
    pub profile: ProfileSnapshot,
    pub history: Vec<GuideHistory>,
    pub conversation: Option<GuideConversation>,
}
impl LocalPgReader {
    /// Nur ein konkretes Anliegen für eine kurze Zustimmung vormerken, kein Profilverlauf.
    pub fn guide_feedback_draft(
        &self,
        turn: &GuideTurn,
        now: i64,
        ttl: i64,
        save: bool,
        deadline: &RequestDeadline,
    ) -> Result<Option<(String, String)>, PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        let uid = turn
            .user_id
            .parse::<i64>()
            .map_err(|_| invalid("Mitgliedskennung ist ungültig"))?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(uid ^ i64::MIN)])?;
        let blocked = tx
            .query_opt(
                "SELECT opted_out,deleted_at IS NOT NULL FROM core.user_privacy WHERE user_id=$1",
                &[&uid],
            )?
            .is_some_and(|r| r.get::<_, bool>(0) || r.get::<_, bool>(1));
        let subject = tx.query_opt("SELECT deleted,globally_opted_out FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2 FOR UPDATE", &[&turn.guild_id,&turn.user_id])?;
        if blocked || subject.is_none_or(|r| r.get::<_, bool>(0) || r.get::<_, bool>(1)) {
            tx.commit()?;
            return Ok(None);
        }
        if save {
            let text: String = turn.content.chars().take(3500).collect();
            tx.execute("INSERT INTO brain.guide_feedback_drafts(guild_id,user_id,channel_id,message_id,text,expires_at) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(guild_id,user_id,channel_id) DO UPDATE SET message_id=EXCLUDED.message_id,text=EXCLUDED.text,expires_at=EXCLUDED.expires_at", &[&turn.guild_id,&turn.user_id,&turn.channel_id,&turn.message_id,&text,&(now+ttl)])?;
            tx.commit()?;
            return Ok(None);
        }
        let row=tx.query_opt("DELETE FROM brain.guide_feedback_drafts WHERE guild_id=$1 AND user_id=$2 AND channel_id=$3 AND expires_at>$4 RETURNING message_id,text", &[&turn.guild_id,&turn.user_id,&turn.channel_id,&now])?;
        tx.commit()?;
        Ok(row.map(|r| (r.get(0), r.get(1))))
    }

    pub fn check_guide_schema(&self) -> Result<(), PortError> {
        let mut client = self.pool.acquire()?;
        client.query_schema("SELECT s.guild_id,s.user_id,s.epoch,s.min_event_id,s.profile_json,s.history_json,c.request_id,o.delivery_id,o.expires_at,v.version FROM brain.guide_subjects s,brain.guide_turn_claims c,brain.guide_feedback_outbox o,brain.guide_schema_version v LIMIT 0")?;
        let row = client.query_one("SELECT version FROM brain.guide_schema_version", &[])?;
        if row.get::<_, i32>(0) != 1 {
            return Err(invalid("Serverguide-Schema ist nicht kompatibel"));
        }
        // Ein globaler Opt-out darf niemals wegen fehlender Leserechte übergangen werden.
        client
            .query_schema("SELECT user_id,opted_out,deleted_at FROM core.user_privacy LIMIT 0")?;
        Ok(())
    }
    pub fn guide_claim(
        &self,
        turn: &GuideTurn,
        deadline: &RequestDeadline,
        now: i64,
        memory_allowed: bool,
    ) -> Result<Option<GuideSnapshot>, PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        let uid = turn
            .user_id
            .parse::<i64>()
            .map_err(|_| invalid("Mitgliedskennung ist ungültig"))?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(uid ^ i64::MIN)])?;
        tx.execute("INSERT INTO brain.guide_subjects(guild_id,user_id) VALUES($1,$2) ON CONFLICT DO NOTHING", &[&turn.guild_id,&turn.user_id])?;
        let row = tx.query_one("SELECT epoch,memory_enabled,contact_enabled,globally_opted_out,deleted,min_event_id FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2 FOR UPDATE", &[&turn.guild_id,&turn.user_id])?;
        let min_event_id: i64 = row.get(5);
        if min_event_id > 0
            && turn
                .message_id
                .parse::<i64>()
                .ok()
                .is_none_or(|id| id < min_event_id)
        {
            tx.commit()?;
            return Ok(None);
        }
        let mut profile = ProfileSnapshot {
            epoch: row.get(0),
            memory_enabled: row.get(1),
            contact_enabled: row.get(2),
            globally_opted_out: row.get(3),
            deleted: row.get(4),
            fields: BTreeMap::new(),
        };
        if let Some(global) = tx.query_opt(
            "SELECT opted_out,deleted_at IS NOT NULL,CASE WHEN reason='user_opt_in' THEN GREATEST(0,((floor(extract(epoch from updated_at)*1000)::bigint+1)-1420070400000)*4194304) ELSE 0 END FROM core.user_privacy WHERE user_id=$1",
            &[&uid],
        )? {
            profile.globally_opted_out |= global.get::<_, bool>(0);
            profile.deleted |= global.get::<_, bool>(1);
            let global_min_event_id: i64 = global.get(2);
            if global_min_event_id > 0 {
                tx.execute("UPDATE brain.guide_subjects SET min_event_id=GREATEST(min_event_id,$3) WHERE guild_id=$1 AND user_id=$2",&[&turn.guild_id,&turn.user_id,&global_min_event_id])?;
                if turn.message_id.parse::<i64>().ok().is_none_or(|id| id < global_min_event_id) {
                    tx.commit()?;
                    return Ok(None);
                }
            }
        }
        if profile.globally_opted_out || profile.deleted {
            profile.memory_enabled = false;
            tx.execute("UPDATE brain.guide_subjects SET globally_opted_out=$3,deleted=$4,memory_enabled=false,profile_json='{}',history_json='[]' WHERE guild_id=$1 AND user_id=$2", &[&turn.guild_id,&turn.user_id,&profile.globally_opted_out,&profile.deleted])?;
        }
        let claimed=tx.execute("INSERT INTO brain.guide_turn_claims(guild_id,user_id,request_id,state) VALUES($1,$2,$3,'claimed') ON CONFLICT DO NOTHING", &[&turn.guild_id,&turn.user_id,&turn.request_id])?;
        if claimed == 0 {
            tx.commit()?;
            return Ok(None);
        }
        let mut history = vec![];
        // Öffentliche Züge lesen weder Profildaten noch DM-Verlauf, auch bei gleichem Mitglied.
        if turn.surface == Surface::Dm
            && memory_allowed
            && profile.memory_enabled
            && !profile.globally_opted_out
            && !profile.deleted
        {
            let private=tx.query_one("SELECT profile_json,history_json FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2", &[&turn.guild_id,&turn.user_id])?;
            profile.fields = serde_json::from_value(private.get::<_, Value>(0))
                .map_err(|_| invalid("Gespeichertes Profil ist ungültig"))?;
            history = serde_json::from_value(private.get::<_, Value>(1))
                .map_err(|_| invalid("Gespeicherter Kontext ist ungültig"))?;
            profile.fields.retain(|_, v| v.expires_at > now);
            history.retain(|v: &GuideHistory| v.expires_at > now);
            if !profile.memory_enabled {
                history.clear();
            }
        }
        let conversation=match &turn.conversation_id {
            Some(id)=>tx.query_opt("SELECT state_json FROM brain.guide_conversations WHERE guild_id=$1 AND user_id=$2 AND conversation_id=$3 AND surface=$4 AND expires_at>$5", &[&turn.guild_id,&turn.user_id,id,&if turn.surface==Surface::Dm{"dm"}else{"public"},&now])?.map(|r|serde_json::from_value(r.get::<_,Value>(0)).map_err(|_|invalid("Unterhaltung ist ungültig"))).transpose()?,
            None=>None,
        };
        tx.commit()?;
        Ok(Some(GuideSnapshot {
            profile,
            history,
            conversation,
        }))
    }
    pub fn guide_control(
        &self,
        turn: &GuideTurn,
        control: &ProfileControl,
        now: i64,
        retention: Option<i64>,
        deadline: &RequestDeadline,
    ) -> Result<ProfileSnapshot, PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        let uid = turn
            .user_id
            .parse::<i64>()
            .map_err(|_| invalid("Mitgliedskennung ist ungültig"))?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(uid ^ i64::MIN)])?;
        let row=tx.query_one("SELECT epoch,memory_enabled,contact_enabled,globally_opted_out,deleted,profile_json FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2 FOR UPDATE", &[&turn.guild_id,&turn.user_id])?;
        let mut profile = ProfileSnapshot {
            epoch: row.get(0),
            memory_enabled: row.get(1),
            contact_enabled: row.get(2),
            globally_opted_out: row.get(3),
            deleted: row.get(4),
            fields: serde_json::from_value(row.get::<_, Value>(5))
                .map_err(|_| invalid("Profil ist ungültig"))?,
        };
        profile.fields.retain(|_, v| v.expires_at > now);
        if let Some(global) = tx.query_opt(
            "SELECT opted_out,deleted_at IS NOT NULL FROM core.user_privacy WHERE user_id=$1",
            &[&uid],
        )? {
            profile.globally_opted_out |= global.get::<_, bool>(0);
            profile.deleted |= global.get::<_, bool>(1);
        }
        if profile.globally_opted_out || profile.deleted {
            profile.memory_enabled = false;
            profile.fields.clear();
        }
        match control {
            ProfileControl::View => {}
            ProfileControl::Forget => {
                profile.memory_enabled = false;
                profile.deleted = true;
                profile.fields.clear();
            }
            ProfileControl::Memory { enabled } => {
                if *enabled
                    && (retention.is_none() || profile.globally_opted_out || profile.deleted)
                {
                    return Err(invalid("Erinnerung kann derzeit nicht aktiviert werden"));
                }
                profile.memory_enabled = *enabled;
                if !enabled {
                    profile.fields.clear();
                }
            }
            ProfileControl::Correct { field, value } => {
                if !profile.memory_enabled || profile.deleted || profile.globally_opted_out {
                    return Err(invalid("Erinnerung ist ausgeschaltet"));
                }
                let ttl =
                    retention.ok_or_else(|| invalid("Aufbewahrung ist noch nicht festgelegt"))?;
                profile.fields.insert(
                    *field,
                    ProfileValue {
                        value: value.clone(),
                        origin_message_id: turn.message_id.clone(),
                        updated_at: now,
                        expires_at: now + ttl,
                        explicitly_stated: true,
                    },
                );
            }
            ProfileControl::Remove { field } => {
                profile.fields.remove(field);
            }
        }
        if !matches!(control, ProfileControl::View) {
            profile.epoch += 1;
            let fields = serde_json::to_value(&profile.fields)
                .map_err(|_| invalid("Profil konnte nicht gespeichert werden"))?;
            tx.execute("UPDATE brain.guide_subjects SET epoch=$3,memory_enabled=$4,deleted=$5,profile_json=$6,updated_at=now() WHERE guild_id=$1 AND user_id=$2", &[&turn.guild_id,&turn.user_id,&profile.epoch,&profile.memory_enabled,&profile.deleted,&fields])?;
            if !profile.memory_enabled || profile.deleted {
                tx.execute("UPDATE brain.guide_subjects SET history_json='[]' WHERE guild_id=$1 AND user_id=$2", &[&turn.guild_id,&turn.user_id])?;
                tx.execute(
                    "DELETE FROM brain.guide_conversations WHERE guild_id=$1 AND user_id=$2",
                    &[&turn.guild_id, &turn.user_id],
                )?;
                tx.execute(
                    "DELETE FROM brain.guide_feedback_drafts WHERE guild_id=$1 AND user_id=$2",
                    &[&turn.guild_id, &turn.user_id],
                )?;
                tx.execute("UPDATE brain.guide_feedback_outbox SET text=NULL,state='failed' WHERE guild_id=$1 AND user_id=$2 AND state='pending'", &[&turn.guild_id,&turn.user_id])?;
            }
        }
        tx.commit()?;
        Ok(profile)
    }
    pub fn guide_finish(
        &self,
        turn: &GuideTurn,
        epoch: i64,
        conversation: Option<&GuideConversation>,
        history: &[GuideHistory],
        feedback: Option<(&str, &str, &str)>,
        deadline: &RequestDeadline,
    ) -> Result<bool, PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        let uid = turn
            .user_id
            .parse::<i64>()
            .map_err(|_| invalid("Mitgliedskennung ist ungültig"))?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(uid ^ i64::MIN)])?;
        let row=tx.query_one("SELECT epoch,memory_enabled,globally_opted_out,deleted FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2 FOR UPDATE", &[&turn.guild_id,&turn.user_id])?;
        if row.get::<_, i64>(0) != epoch {
            tx.commit()?;
            return Ok(false);
        }
        let global = tx.query_opt(
            "SELECT opted_out,deleted_at IS NOT NULL FROM core.user_privacy WHERE user_id=$1",
            &[&uid],
        )?;
        let blocked = global.is_some_and(|r| r.get::<_, bool>(0) || r.get::<_, bool>(1));
        let memory =
            row.get::<_, bool>(1) && !row.get::<_, bool>(2) && !row.get::<_, bool>(3) && !blocked;
        if blocked && !row.get::<_, bool>(2) && !row.get::<_, bool>(3) {
            tx.commit()?;
            return Ok(false);
        }
        if turn.surface == Surface::Dm && memory {
            let json =
                serde_json::to_value(history).map_err(|_| invalid("Kontext ist ungültig"))?;
            tx.execute("UPDATE brain.guide_subjects SET history_json=$3,updated_at=now() WHERE guild_id=$1 AND user_id=$2", &[&turn.guild_id,&turn.user_id,&json])?;
        }
        if let Some(conv) = conversation.filter(|_| !row.get::<_, bool>(3)) {
            let json =
                serde_json::to_value(conv).map_err(|_| invalid("Unterhaltung ist ungültig"))?;
            tx.execute("INSERT INTO brain.guide_conversations(guild_id,user_id,conversation_id,surface,state_json,expires_at) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(guild_id,user_id,conversation_id) DO UPDATE SET state_json=EXCLUDED.state_json,expires_at=EXCLUDED.expires_at", &[&turn.guild_id,&turn.user_id,&conv.id,&if conv.surface==Surface::Dm{"dm"}else{"public"},&json,&conv.expires_at])?;
        }
        if let Some((id, destination, text)) = feedback {
            tx.execute("INSERT INTO brain.guide_feedback_outbox(guild_id,user_id,delivery_id,destination_channel_id,text,state) VALUES($1,$2,$3,$4,$5,'pending') ON CONFLICT DO NOTHING", &[&turn.guild_id,&turn.user_id,&id,&destination,&text])?;
        }
        tx.execute("UPDATE brain.guide_turn_claims SET state='finished' WHERE guild_id=$1 AND user_id=$2 AND request_id=$3", &[&turn.guild_id,&turn.user_id,&turn.request_id])?;
        tx.commit()?;
        Ok(true)
    }
    pub fn guide_action_result(
        &self,
        action: &ActionResult,
        deadline: &RequestDeadline,
    ) -> Result<Option<i64>, PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        let uid = action
            .user_id
            .parse::<i64>()
            .map_err(|_| invalid("Mitgliedskennung ist ungültig"))?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(uid ^ i64::MIN)])?;
        let Some(subject) = tx.query_opt("SELECT epoch,deleted FROM brain.guide_subjects WHERE guild_id=$1 AND user_id=$2 FOR UPDATE", &[&action.guild_id,&action.user_id])? else {tx.commit()?;return Ok(None);};
        let epoch: i64 = subject.get(0);
        if subject.get::<_, bool>(1) {
            tx.commit()?;
            return Ok(None);
        }
        if let Some(conversation) = action.delivery_id.strip_prefix("reply:") {
            if action.success {
                let bot_message = action
                    .reply_message_id
                    .as_ref()
                    .ok_or_else(|| invalid("Antwortnachweis fehlt"))?;
                let row=tx.query_opt("SELECT state_json FROM brain.guide_conversations WHERE guild_id=$1 AND user_id=$2 AND conversation_id=$3 FOR UPDATE", &[&action.guild_id,&action.user_id,&conversation])?;
                if let Some(row) = row {
                    let mut conv: GuideConversation =
                        serde_json::from_value(row.get::<_, Value>(0))
                            .map_err(|_| invalid("Unterhaltung ist ungültig"))?;
                    conv.last_bot_message_id = Some(bot_message.clone());
                    let json = serde_json::to_value(conv)
                        .map_err(|_| invalid("Unterhaltung ist ungültig"))?;
                    tx.execute("UPDATE brain.guide_conversations SET state_json=$4 WHERE guild_id=$1 AND user_id=$2 AND conversation_id=$3", &[&action.guild_id,&action.user_id,&conversation,&json])?;
                }
            }
            tx.commit()?;
            return Ok(None);
        }
        let row=tx.query_opt("SELECT state FROM brain.guide_feedback_outbox WHERE guild_id=$1 AND user_id=$2 AND delivery_id=$3 FOR UPDATE", &[&action.guild_id,&action.user_id,&action.delivery_id])?;
        let Some(row) = row else {
            tx.commit()?;
            return Ok(None);
        };
        if row.get::<_, String>(0) != "pending" {
            tx.commit()?;
            return Ok(None);
        }
        if action.success && action.sent_message_id.is_none() {
            return Err(invalid("Zustellnachweis fehlt"));
        }
        tx.execute("UPDATE brain.guide_feedback_outbox SET state=$4,text=NULL,discord_message_id=$5 WHERE guild_id=$1 AND user_id=$2 AND delivery_id=$3", &[&action.guild_id,&action.user_id,&action.delivery_id,&if action.success{"sent"}else{"failed"},&action.sent_message_id])?;
        tx.commit()?;
        Ok(Some(epoch))
    }
}
