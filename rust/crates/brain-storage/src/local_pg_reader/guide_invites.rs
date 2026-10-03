//! Konkrete Einladungsaufträge bleiben vom Profil getrennt und widerrufbar.
use super::{invalid, LocalPgReader};
use brain_contracts::{guide::*, PortError, RequestDeadline};

pub struct GuideInviteGrant {
    pub action_id: String,
    pub target: i64,
    pub ttl_seconds: i64,
    pub conversation: GuideConversation,
}
impl LocalPgReader {
    pub fn guide_create_invite_grant(
        &self, turn: &GuideTurn, epoch: i64, grant: &GuideInviteGrant, deadline: &RequestDeadline,
    ) -> Result<Option<InviteCorrelation>, PortError> {
        let action = grant.action_id.as_str();
        let target = grant.target;
        let ttl = grant.ttl_seconds;
        let conversation = &grant.conversation;
        if conversation.user_id != turn.user_id || conversation.channel_id != turn.channel_id
            || conversation.thread_id != turn.thread_id || conversation.surface != turn.surface
            || conversation.last_user_message_id != turn.message_id {
            return Err(invalid("Einladungsunterhaltung stimmt nicht mit dem Auftrag überein"));
        }
        if !(1..=3600).contains(&ttl) || target <= 0 { return Err(invalid("Ungültiger Einladungsauftrag")); }
        let actor: i64 = turn.user_id.parse().map_err(|_| invalid("Ungültiges Mitglied"))?;
        let guild: i64 = turn.guild_id.parse().map_err(|_| invalid("Ungültiger Server"))?;
        let channel: i64 = turn.channel_id.parse().map_err(|_| invalid("Ungültiger Kanal"))?;
        let message: i64 = turn.message_id.parse().map_err(|_| invalid("Ungültige Nachricht"))?;
        let thread = turn.thread_id.as_ref().map(|id| id.parse::<i64>()).transpose().map_err(|_| invalid("Ungültiger Thread"))?;
        let surface = if turn.surface == Surface::Dm { "dm" } else { "public" };
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(actor ^ i64::MIN)])?;
        let valid = tx.query_opt("SELECT c.request_id FROM brain.guide_turn_claims c JOIN brain.guide_subjects s USING(guild_id,user_id) WHERE c.guild_id=$1 AND c.user_id=$2 AND c.request_id=$3 AND c.state='claimed' AND c.subject_epoch=$4 AND s.epoch=$4 AND c.turn_sequence=s.turn_sequence AND c.source_surface=$6 AND c.source_channel_id=$7 AND c.source_message_id=$8 AND c.source_thread_id IS NOT DISTINCT FROM $9 AND c.source_event_type='message' AND NOT s.deleted AND NOT s.globally_opted_out AND NOT EXISTS(SELECT 1 FROM core.user_privacy p WHERE p.user_id=$5 AND (p.opted_out OR p.deleted_at IS NOT NULL))", &[&turn.guild_id,&turn.user_id,&turn.request_id,&epoch,&actor,&surface,&turn.channel_id,&turn.message_id,&turn.thread_id])?.is_some();
        if !valid { tx.commit()?; return Ok(None); }
        let inserted = tx.execute("INSERT INTO brain.guide_action_grants(action_id,turn_id,actor_id,guild_id,source_channel_id,source_thread_id,source_event_type,source_surface,message_id,privacy_epoch,kind,steam_id64,expires_at) VALUES($1,$2,$3,$4,$5,$6,'message',$7,$8,$9,'deadlock_access_invite',$10,clock_timestamp()+make_interval(secs=>$11::bigint::double precision)) ON CONFLICT DO NOTHING", &[&action,&turn.request_id,&actor,&guild,&channel,&thread,&surface,&message,&epoch,&target,&ttl])?;
        if inserted == 0 { tx.commit()?; return Ok(None); }
        let json=serde_json::to_value(conversation).map_err(|_|invalid("Einladungsunterhaltung ist ungültig"))?;
        tx.execute("INSERT INTO brain.guide_conversations(guild_id,user_id,conversation_id,surface,state_json,expires_at,latest_request_id) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(guild_id,user_id,conversation_id) DO UPDATE SET state_json=EXCLUDED.state_json,expires_at=EXCLUDED.expires_at,latest_request_id=EXCLUDED.latest_request_id", &[&turn.guild_id,&turn.user_id,&conversation.id,&surface,&json,&conversation.expires_at,&turn.request_id])?;
        tx.execute("UPDATE brain.guide_turn_claims SET state='finished',conversation_id=$4 WHERE guild_id=$1 AND user_id=$2 AND request_id=$3", &[&turn.guild_id,&turn.user_id,&turn.request_id,&conversation.id])?;
        tx.commit()?;
        Ok(Some(InviteCorrelation {action_id:action.into(),turn_id:turn.request_id.clone(),actor_id:actor,guild_id:guild,channel_id:channel,source_thread_id:thread,source_event_type:"message".into(),message_id:message,privacy_epoch:epoch}))
    }

    pub fn guide_invite_state(
        &self, turn: &GuideTurn, epoch: i64, action: &str, cancel: bool,
        deadline: &RequestDeadline,
    ) -> Result<Option<InviteStatus>, PortError> {
        let actor: i64 = turn.user_id.parse().map_err(|_| invalid("Ungültiges Mitglied"))?;
        let guild: i64 = turn.guild_id.parse().map_err(|_| invalid("Ungültiger Server"))?;
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false,false)?;
        tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(actor ^ i64::MIN)])?;
        let permitted = tx.query_opt("SELECT epoch FROM brain.guide_subjects s WHERE guild_id=$1 AND user_id=$2 AND epoch=$3 AND NOT deleted AND NOT globally_opted_out AND NOT EXISTS(SELECT 1 FROM core.user_privacy p WHERE p.user_id=$4 AND (p.opted_out OR p.deleted_at IS NOT NULL))", &[&turn.guild_id,&turn.user_id,&epoch,&actor])?.is_some();
        if !permitted { tx.commit()?; return Ok(None); }
        let Some(row) = tx.query_opt("SELECT status,source_surface,expires_at>clock_timestamp(),revoked_at IS NULL FROM brain.guide_action_grants WHERE action_id=$1 AND actor_id=$2 AND guild_id=$3 AND privacy_epoch=$4", &[&action,&actor,&guild,&epoch])? else {tx.commit()?;return Ok(None);};
        if turn.surface == Surface::Public && row.get::<_,String>(1) != "public" {tx.commit()?;return Ok(None);}
        let mut status: String = row.get(0);
        if cancel || !row.get::<_,bool>(2) {
            let terminal = matches!(status.as_str(),"invite_sent"|"already_has_access"|"unknown");
            if !terminal { status=if cancel {"cancelled"}else{"expired"}.into(); }
            tx.execute("UPDATE brain.guide_action_grants SET revoked_at=COALESCE(revoked_at,clock_timestamp()),steam_id64=NULL,turn_id=NULL,source_channel_id=NULL,source_thread_id=NULL,source_event_type=NULL,status=$2,updated_at=clock_timestamp() WHERE action_id=$1", &[&action,&status])?;
        }
        let result = serde_json::from_value(serde_json::Value::String(status)).map_err(|_| invalid("Ungültiger Einladungsstatus"))?;
        tx.commit()?;
        Ok(Some(result))
    }
}
