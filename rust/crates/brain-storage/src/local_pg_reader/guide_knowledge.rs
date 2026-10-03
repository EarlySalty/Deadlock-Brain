//! Öffentlicher Discordstand wird in den vorhandenen kanonischen Quelltabellen geführt.
use super::{invalid, LocalPgReader};
use brain_contracts::{guide::ServerSnapshot, PortError, RequestDeadline, SourceRecordV2};
use serde_json::Value;
impl LocalPgReader {
    pub fn guide_sync_server(
        &self,
        snapshot: &ServerSnapshot,
        record: &SourceRecordV2,
        deadline: &RequestDeadline,
    ) -> Result<(), PortError> {
        let revision =
            i64::try_from(snapshot.revision).map_err(|_| invalid("Quellrevision ist ungültig"))?;
        let json =
            serde_json::to_value(record).map_err(|_| invalid("Quellvertrag ist ungültig"))?;
        let mut client = self.pool.acquire_until(Some(deadline))?;
        client.query_one(
            "SELECT brain.guide_set_server_record($1,$2,$3)",
            &[&snapshot.guild_id, &revision, &json],
        )?;
        Ok(())
    }
    pub fn guide_server_record(
        &self,
        guild: &str,
        deadline: &RequestDeadline,
    ) -> Result<Option<SourceRecordV2>, PortError> {
        let source = format!("guide-discord:{guild}");
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let row=client.query_opt("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id='server' AND NOT tombstone AND record_json->>'visibility'='public' AND record_json->'metadata'->>'source_class'='server_documentation'",&[&source])?;
        row.map(|r| {
            serde_json::from_value(r.get::<_, Value>(0))
                .map_err(|_| invalid("Serverquelle ist ungültig"))
        })
        .transpose()
    }
    pub fn guide_cleanup(
        &self,
        now: i64,
        event_retention: i64,
        deadline: &RequestDeadline,
    ) -> Result<(), PortError> {
        let mut client = self.pool.acquire_until(Some(deadline))?;
        let mut tx = client.transaction(false, false)?;
        // Derselbe Mitgliedsschutz wie bei Korrektur, Import und globaler Löschung.
        // Ein fester Ablauf verhindert neue Profilwerte nach einer parallelen Löschung.
        for row in tx.query(
            "SELECT actor_id FROM (SELECT user_id::bigint AS actor_id FROM brain.guide_subjects UNION SELECT actor_id FROM brain.guide_action_grants) subjects ORDER BY actor_id",
            &[],
        )? {
            let user: i64 = row.get(0);
            tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&(user ^ i64::MIN)])?;
        }
        tx.execute(
            "DELETE FROM brain.guide_conversations WHERE expires_at<=$1",
            &[&now],
        )?;
        tx.execute(
            "DELETE FROM brain.guide_feedback_drafts WHERE expires_at<=$1",
            &[&now],
        )?;
        tx.execute("DELETE FROM brain.guide_turn_claims WHERE created_at<to_timestamp($1::double precision)",&[&((now-event_retention) as f64)])?;
        tx.execute("UPDATE brain.guide_feedback_outbox SET text=NULL,state='failed' WHERE state='pending' AND expires_at<=to_timestamp($1::double precision)",&[&(now as f64)])?;
        tx.execute("DELETE FROM brain.guide_feedback_outbox WHERE state<>'pending' AND created_at<to_timestamp($1::double precision)",&[&((now-event_retention) as f64)])?;
        tx.execute("UPDATE brain.guide_subjects s SET profile_json=COALESCE((SELECT jsonb_object_agg(f.key,f.value) FROM jsonb_each(s.profile_json) f WHERE (f.value->>'expires_at')::bigint>$1),'{}'::jsonb),history_json=COALESCE((SELECT jsonb_agg(v.value) FROM jsonb_array_elements(s.history_json) v WHERE (v.value->>'expires_at')::bigint>$1),'[]'::jsonb) WHERE EXISTS(SELECT 1 FROM jsonb_each(s.profile_json) f WHERE (f.value->>'expires_at')::bigint<=$1) OR EXISTS(SELECT 1 FROM jsonb_array_elements(s.history_json) v WHERE (v.value->>'expires_at')::bigint<=$1)",&[&now])?;
        tx.execute("UPDATE brain.guide_action_grants SET revoked_at=COALESCE(revoked_at,clock_timestamp()),steam_id64=NULL,turn_id=NULL,source_channel_id=NULL,source_thread_id=NULL,source_event_type=NULL,status=CASE WHEN status IN ('invite_sent','already_has_access','unknown') THEN status ELSE 'expired' END,updated_at=clock_timestamp() WHERE expires_at<=clock_timestamp() AND (steam_id64 IS NOT NULL OR turn_id IS NOT NULL)", &[])?;
        tx.commit()
    }
}
