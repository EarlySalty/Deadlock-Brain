//! Explicit owner-only migration and DDL-free service preflight.
//! The historical SQL files stay immutable; one outer transaction owns the upgrade.
use crate::{memory_repository::validate_release, pg_jobs::database_error, PgStore};
use brain_contracts::{store::STORE_VERSION, CorpusRelease, PortError, SourceRecordV2};
use sqlx::{PgConnection, Row};

pub const CORE_SCHEMA_VERSION: i32 = 2;
const V1: &str = include_str!("../../../../scripts/migrations/2026-09-24-brain-contract-v1.sql");
const V2: &str = include_str!("../../../../scripts/migrations/2026-09-25-brain-core-jobs-v2.sql");
const VERSION: &str =
    include_str!("../../../../scripts/migrations/2026-09-26-brain-core-compatibility-v2.sql");

fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn migration_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable(
        "core migration failed or lock timed out; use brain-migrate with the migration owner, never the service role".into(),
    )
}
fn compatibility_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable(
        "core schema missing, unreadable or incompatible; run brain-migrate separately before service startup".into(),
    )
}

// Not a general SQL parser: accept only our embedded, single-transaction files.
// Never send their COMMIT through an outer sqlx transaction (partial upgrades!).
fn body(sql: &str) -> Result<&str, PortError> {
    let (_, rest) = sql
        .split_once("BEGIN;")
        .ok_or_else(|| invalid("migration has no BEGIN wrapper"))?;
    let body = rest
        .trim()
        .strip_suffix("COMMIT;")
        .ok_or_else(|| invalid("migration has no COMMIT wrapper"))?;
    if body.contains("BEGIN;") || body.contains("COMMIT;") || body.contains("ROLLBACK;") {
        return Err(invalid("unexpected migration transaction control"));
    }
    Ok(body)
}

async fn check_version(connection: &mut PgConnection) -> Result<(), PortError> {
    let rows = sqlx::query("SELECT schema_version, store_contract FROM brain.core_schema_version")
        .fetch_all(&mut *connection)
        .await
        .map_err(compatibility_error)?;
    if rows.len() != 1
        || rows[0]
            .try_get::<i32, _>("schema_version")
            .map_err(compatibility_error)?
            != CORE_SCHEMA_VERSION
        || rows[0]
            .try_get::<String, _>("store_contract")
            .map_err(compatibility_error)?
            != STORE_VERSION
    {
        return Err(invalid(
            "unsupported core schema/store version; do not downgrade or start an older binary",
        ));
    }
    Ok(())
}

// Parse/plan only. Requires SELECT, never CREATE/ALTER, and reads no corpus rows.
pub(crate) const SHAPE_PROBE: &str = "SELECT r.source_id,r.logical_id,r.revision,r.content_hash,r.tombstone,r.record_json,r.created_at,
    h.source_id,h.logical_id,h.revision,h.content_hash,h.tombstone,h.record_json,h.updated_at,
    c.release_id,c.knowledge_version,c.patch,c.release_json,c.created_at,
    j.source_id,j.owner,j.fence,j.lease_until,j.state,j.updated_at,
    p.source_id,p.configuration,p.generation,p.checkpoint_json,p.batch_json,p.updated_at,
    o.conversation_id,o.actor_id,o.created_at
    FROM brain.source_record_revisions r, brain.source_record_heads h, brain.corpus_releases_v1 c,
         brain.source_jobs_v1 j, brain.source_checkpoints_v1 p, brain.conversation_owners_v1 o
    LIMIT 0";

impl PgStore {
    /// Separate additive Guide-Migration, ausschließlich über den Owner-Migrator.
    pub async fn migrate_guide(&self) -> Result<(), PortError> {
        let sql = include_str!("../../../../scripts/migrations/2026-10-03-serverguide-v1.sql");
        let mut tx = self.pool.begin().await.map_err(migration_error)?;
        sqlx::raw_sql(
            "SET LOCAL lock_timeout='5000ms'; SELECT pg_advisory_xact_lock(742110026113::bigint)",
        )
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        let present = sqlx::query("SELECT to_regclass('brain.guide_subjects') IS NOT NULL,to_regclass('brain.guide_schema_version') IS NOT NULL")
            .fetch_one(&mut *tx).await.map_err(migration_error)?;
        let existed: bool = present.try_get(0).map_err(migration_error)?;
        if existed != present.try_get::<bool, _>(1).map_err(migration_error)? {
            return Err(invalid("Serverguide-Schema ist unvollständig"));
        }
        if !existed {
            sqlx::raw_sql(body(sql)?)
                .execute(&mut *tx)
                .await
                .map_err(migration_error)?;
        }
        sqlx::raw_sql(body(include_str!(
            "../../../../scripts/migrations/2026-10-03-serverguide-v2.sql"
        ))?)
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        sqlx::raw_sql(body(include_str!(
            "../../../../scripts/migrations/2026-10-03-serverguide-v3.sql"
        ))?)
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        sqlx::raw_sql(body(include_str!(
            "../../../../scripts/migrations/2026-10-03-serverguide-v4.sql"
        ))?)
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        if !existed {
            sqlx::query("UPDATE brain.guide_subjects SET legacy_import_eligible=true WHERE epoch=0 AND turn_sequence=0 AND NOT memory_enabled AND NOT contact_enabled AND profile_json='{}'::jsonb AND history_json='[]'::jsonb")
                .execute(&mut *tx).await.map_err(migration_error)?;
        }
        tx.commit().await.map_err(migration_error)
    }

    pub async fn check_guide(&self) -> Result<(), PortError> {
        sqlx::query("SELECT s.min_event_id,s.turn_sequence,s.legacy_import_eligible,c.turn_sequence,c.subject_epoch,c.reply_message_id,c.conversation_id,v.latest_request_id,o.expires_at FROM brain.guide_subjects s,brain.guide_turn_claims c,brain.guide_conversations v,brain.guide_feedback_outbox o LIMIT 0")
            .execute(&self.pool)
            .await
            .map_err(compatibility_error)?;
        sqlx::query("SELECT a.action_id,a.source_surface,a.steam_id64,a.friend_task_id,a.invite_task_id,a.friend_dispatch_reserved,a.invite_dispatch_reserved,c.source_channel_id,c.source_thread_id,c.source_message_id,c.source_event_type FROM brain.guide_action_grants a,brain.guide_turn_claims c LIMIT 0")
            .execute(&self.pool).await.map_err(compatibility_error)?;
        let rows = sqlx::query("SELECT version FROM brain.guide_schema_version")
            .fetch_all(&self.pool)
            .await
            .map_err(compatibility_error)?;
        if rows.len() != 1
            || rows[0]
                .try_get::<i32, _>("version")
                .map_err(compatibility_error)?
                != 1
        {
            return Err(invalid("Serverguide-Schema ist nicht kompatibel"));
        }
        Ok(())
    }

    /// Kontrollierter einmaliger Inhaltsimport. Ohne ausdrücklich konfigurierte Frist kein Lesen.
    pub async fn import_guide_legacy(&self, retention: Option<i64>) -> Result<(), PortError> {
        let ttl = retention
            .filter(|v| (60..=31_536_000).contains(v))
            .ok_or_else(|| {
                invalid("Guide-Aufbewahrung fehlt; private Inhaltsmigration bleibt aus")
            })?;
        let mut tx = self.pool.begin().await.map_err(migration_error)?;
        let present: bool =
            sqlx::query_scalar("SELECT to_regclass('bot.concierge_profiles') IS NOT NULL")
                .fetch_one(&mut *tx)
                .await
                .map_err(migration_error)?;
        if !present {
            return Err(invalid("Concierge-Quelltabellen fehlen"));
        }
        let ids=sqlx::query("SELECT p.user_id,p.guild_id FROM bot.concierge_profiles p WHERE NOT p.opted_out AND p.forgot_at IS NULL AND NOT EXISTS(SELECT 1 FROM core.user_privacy g WHERE g.user_id=p.user_id AND (g.opted_out OR g.deleted_at IS NOT NULL OR g.reason='user_opt_in')) AND NOT EXISTS(SELECT 1 FROM brain.guide_legacy_imports i WHERE i.user_id=p.user_id::text AND i.guild_id=p.guild_id::text)")
            .fetch_all(&mut *tx).await.map_err(migration_error)?;
        for row in ids {
            let user: i64 = row.try_get("user_id").map_err(migration_error)?;
            let guild: i64 = row.try_get("guild_id").map_err(migration_error)?;
            sqlx::query("SELECT pg_advisory_xact_lock($1)")
                .bind(user ^ i64::MIN)
                .execute(&mut *tx)
                .await
                .map_err(migration_error)?;
            let blocked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM core.user_privacy WHERE user_id=$1 AND (opted_out OR deleted_at IS NOT NULL OR reason='user_opt_in')) OR EXISTS(SELECT 1 FROM brain.guide_subjects WHERE user_id=$1::bigint::text AND guild_id=$2::bigint::text AND (deleted OR globally_opted_out OR min_event_id>0))")
                .bind(user).bind(guild).fetch_one(&mut *tx).await.map_err(migration_error)?;
            if blocked {
                continue;
            }
            sqlx::query("INSERT INTO brain.guide_subjects(guild_id,user_id,legacy_import_eligible) VALUES($1::bigint::text,$2::bigint::text,true) ON CONFLICT DO NOTHING").bind(guild).bind(user).execute(&mut *tx).await.map_err(migration_error)?;
            let pristine: bool = sqlx::query_scalar("SELECT legacy_import_eligible AND epoch=0 AND turn_sequence=0 AND NOT memory_enabled AND NOT contact_enabled AND NOT deleted AND NOT globally_opted_out AND min_event_id=0 AND profile_json='{}'::jsonb AND history_json='[]'::jsonb AND NOT EXISTS(SELECT 1 FROM brain.guide_turn_claims c WHERE c.guild_id=s.guild_id AND c.user_id=s.user_id) AND NOT EXISTS(SELECT 1 FROM brain.guide_conversations c WHERE c.guild_id=s.guild_id AND c.user_id=s.user_id) AND NOT EXISTS(SELECT 1 FROM brain.guide_feedback_outbox c WHERE c.guild_id=s.guild_id AND c.user_id=s.user_id) AND NOT EXISTS(SELECT 1 FROM brain.guide_feedback_drafts c WHERE c.guild_id=s.guild_id AND c.user_id=s.user_id) AND NOT EXISTS(SELECT 1 FROM brain.guide_action_grants a WHERE a.actor_id=$1 AND a.guild_id=$2) FROM brain.guide_subjects s WHERE user_id=$1::bigint::text AND guild_id=$2::bigint::text FOR UPDATE")
                .bind(user).bind(guild).fetch_one(&mut *tx).await.map_err(migration_error)?;
            if !pristine {
                sqlx::query("INSERT INTO brain.guide_legacy_imports(guild_id,user_id) VALUES($1::bigint::text,$2::bigint::text) ON CONFLICT DO NOTHING").bind(guild).bind(user).execute(&mut *tx).await.map_err(migration_error)?;
                continue;
            }
            // Nur das bekannte Spielzeitfeld. Keine Persönlichkeits-/Rang-/Patenanalyse übernehmen.
            sqlx::query("UPDATE brain.guide_subjects s SET memory_enabled=false,contact_enabled=false,epoch=epoch+1,turn_sequence=turn_sequence+1,legacy_import_eligible=false,profile_json=CASE WHEN p.play_times IS NOT NULL AND p.updated_at+make_interval(secs=>$3::bigint::double precision)>now() THEN jsonb_build_object('play_times',jsonb_build_object('value',left(p.play_times,500),'origin_message_id','legacy-concierge','updated_at',extract(epoch from p.updated_at)::bigint,'expires_at',extract(epoch from p.updated_at)::bigint+$3,'explicitly_stated',false)) ELSE '{}'::jsonb END, history_json=COALESCE((SELECT jsonb_agg(z.entry ORDER BY z.created_at) FROM (SELECT c.created_at,jsonb_build_object('role',c.role,'content',left(c.content,2000),'message_id','legacy-concierge','expires_at',extract(epoch from c.created_at)::bigint+$3) entry FROM bot.concierge_conversations c WHERE c.user_id=$2 AND c.guild_id=$1 AND c.role IN ('user','assistant') AND c.created_at+make_interval(secs=>$3::bigint::double precision)>now() ORDER BY c.created_at DESC LIMIT 8) z),'[]'::jsonb) FROM bot.concierge_profiles p WHERE p.guild_id=$1 AND p.user_id=$2 AND NOT p.opted_out AND p.forgot_at IS NULL AND s.guild_id=$1::bigint::text AND s.user_id=$2::bigint::text AND s.legacy_import_eligible AND s.epoch=0 AND s.turn_sequence=0 AND s.profile_json='{}'::jsonb AND s.history_json='[]'::jsonb AND NOT s.memory_enabled AND NOT s.contact_enabled AND NOT s.deleted AND NOT s.globally_opted_out AND s.min_event_id=0")
                .bind(guild).bind(user).bind(ttl).execute(&mut *tx).await.map_err(migration_error)?;
            sqlx::query("INSERT INTO brain.guide_legacy_imports(guild_id,user_id) VALUES($1::bigint::text,$2::bigint::text) ON CONFLICT DO NOTHING").bind(guild).bind(user).execute(&mut *tx).await.map_err(migration_error)?;
        }
        tx.commit().await.map_err(migration_error)
    }

    /// Service startup preflight. SELECT-only; no automatic migration or repair.
    /// Call before admitting traffic/jobs, including when the schema already exists.
    pub async fn check_core_schema(&self) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(compatibility_error)?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(compatibility_error)?;
        check_version(&mut tx).await?;
        sqlx::query(SHAPE_PROBE)
            .fetch_all(&mut *tx)
            .await
            .map_err(compatibility_error)?;
        tx.commit().await.map_err(compatibility_error)
    }

    /// Explicit privileged maintenance operation. Never called by constructors/read paths.
    /// Stop and externally fence every writer before backup and this call; the locks below
    /// protect the transaction, not the post-COMMIT cutover from old/uncooperative binaries.
    pub async fn migrate_core(&self) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(migration_error)?;
        sqlx::raw_sql(
            "SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms';
            SELECT pg_advisory_xact_lock(742110026112::bigint)",
        )
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        let marked: bool =
            sqlx::query_scalar("SELECT to_regclass('brain.core_schema_version') IS NOT NULL")
                .fetch_one(&mut *tx)
                .await
                .map_err(migration_error)?;
        if marked {
            check_version(&mut tx).await?;
        }
        for migration in [V1, V2] {
            sqlx::raw_sql(body(migration)?)
                .execute(&mut *tx)
                .await
                .map_err(migration_error)?;
        }
        sqlx::raw_sql(
            "LOCK TABLE brain.source_record_revisions, brain.source_record_heads,
            brain.corpus_releases_v1, brain.source_jobs_v1, brain.source_checkpoints_v1,
            brain.conversation_owners_v1 IN ACCESS EXCLUSIVE MODE",
        )
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        validate_records(&mut tx).await?;
        validate_releases(&mut tx).await?;
        // No source record, head, release, checkpoint, lease, or ACL is rewritten.
        // In particular, source-wide legacy release maxima cannot be guessed into document pins.
        sqlx::raw_sql(body(VERSION)?)
            .execute(&mut *tx)
            .await
            .map_err(migration_error)?;
        if !marked {
            sqlx::query("INSERT INTO brain.core_schema_version(singleton,schema_version,store_contract) VALUES(true,$1,$2)")
                .bind(CORE_SCHEMA_VERSION).bind(STORE_VERSION)
                .execute(&mut *tx).await.map_err(migration_error)?;
        }
        check_version(&mut tx).await?;
        sqlx::query(SHAPE_PROBE)
            .fetch_all(&mut *tx)
            .await
            .map_err(migration_error)?;
        tx.commit().await.map_err(migration_error)
    }
}

async fn validate_records(connection: &mut PgConnection) -> Result<(), PortError> {
    // Bounded keyset pages, not a full in-memory copy of the corpus during maintenance.
    let mut cursor: Option<(String, String, i64)> = None;
    loop {
        let rows = sqlx::query(
            "SELECT source_id,logical_id,revision,content_hash,tombstone,record_json
            FROM brain.source_record_revisions
            WHERE $1::text IS NULL OR (source_id,logical_id,revision)>($1,$2,$3)
            ORDER BY source_id,logical_id,revision LIMIT 256",
        )
        .bind(cursor.as_ref().map(|c| &c.0))
        .bind(cursor.as_ref().map(|c| &c.1))
        .bind(cursor.as_ref().map(|c| c.2))
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
        if rows.is_empty() {
            break;
        }
        for row in rows {
            let source: String = row.try_get("source_id").map_err(database_error)?;
            let logical: String = row.try_get("logical_id").map_err(database_error)?;
            let revision: i64 = row.try_get("revision").map_err(database_error)?;
            let record: SourceRecordV2 = serde_json::from_value(
                row.try_get("record_json").map_err(database_error)?,
            )
            .map_err(|_| {
                invalid("stored record is not compatible; upgrade aborted without rewriting data")
            })?;
            record
                .validate()
                .map_err(|_| invalid("stored record fails current contract; upgrade aborted"))?;
            if record.source_id != source
                || record.logical_id != logical
                || record.revision != revision as u64
                || record.content_hash
                    != row
                        .try_get::<String, _>("content_hash")
                        .map_err(database_error)?
                || record.tombstone
                    != row
                        .try_get::<bool, _>("tombstone")
                        .map_err(database_error)?
            {
                return Err(invalid(
                    "stored record columns and JSON disagree; upgrade aborted",
                ));
            }
            cursor = Some((source, logical, revision));
        }
    }
    let inconsistent: bool = sqlx::query_scalar("SELECT EXISTS (
        SELECT 1 FROM brain.source_record_heads h LEFT JOIN brain.source_record_revisions r
          USING(source_id,logical_id,revision)
        WHERE r.source_id IS NULL OR h.record_json IS DISTINCT FROM r.record_json
           OR h.content_hash IS DISTINCT FROM r.content_hash OR h.tombstone IS DISTINCT FROM r.tombstone
    )")
        .fetch_one(connection).await.map_err(database_error)?;
    if inconsistent {
        return Err(invalid("head/history mismatch; upgrade aborted"));
    }
    Ok(())
}

async fn validate_releases(connection: &mut PgConnection) -> Result<(), PortError> {
    let mut cursor: Option<String> = None;
    loop {
        let rows = sqlx::query(
            "SELECT release_id,knowledge_version,patch,release_json
            FROM brain.corpus_releases_v1 WHERE $1::text IS NULL OR release_id>$1
            ORDER BY release_id LIMIT 64",
        )
        .bind(&cursor)
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
        if rows.is_empty() {
            break;
        }
        for row in rows {
            let release: CorpusRelease = serde_json::from_value(row.try_get("release_json").map_err(database_error)?)
                .map_err(|_| invalid("legacy or invalid release pins; explicit verified document manifest required, upgrade aborted"))?;
            validate_release(&release)?;
            let id: String = row.try_get("release_id").map_err(database_error)?;
            if release.release_id != id
                || release.knowledge_version
                    != row
                        .try_get::<String, _>("knowledge_version")
                        .map_err(database_error)?
                || release.patch != row.try_get::<String, _>("patch").map_err(database_error)?
            {
                return Err(invalid(
                    "release columns and JSON disagree; upgrade aborted",
                ));
            }
            let pins = serde_json::to_value(&release.source_revisions)
                .map_err(|_| invalid("invalid release pins"))?;
            let missing: bool = sqlx::query_scalar("SELECT EXISTS (
                SELECT 1 FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d
                LEFT JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint
                LEFT JOIN brain.source_record_heads h ON h.source_id=s.key AND h.logical_id=d.key
                WHERE r.source_id IS NULL OR h.source_id IS NULL OR h.revision<r.revision
            )")
                .bind(pins).fetch_one(&mut *connection).await.map_err(database_error)?;
            if missing {
                return Err(invalid(
                    "release revision or current ACL missing; upgrade aborted",
                ));
            }
            cursor = Some(id);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn embedded_migrations_have_exactly_one_outer_transaction() {
        for sql in [V1, V2, VERSION] {
            let body = body(sql).unwrap();
            assert!(!body.contains("BEGIN;"));
            assert!(!body.contains("COMMIT;"));
            assert!(body.contains("CREATE TABLE"));
        }
        assert!(body("BEGIN; SELECT 1; COMMIT; SELECT 2;").is_err());
        assert!(body("BEGIN; COMMIT; BEGIN; COMMIT;").is_err());
    }
}
