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
const ENTITY_MIGRATIONS: [(&str, &str); 4] = [
    (
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profiles-v1.sql"),
        "SELECT CASE WHEN to_regclass('brain.entity_profile_entities_v1') IS NULL AND to_regclass('brain.entity_profile_facts_v1') IS NULL THEN 0 WHEN to_regclass('brain.entity_profile_entities_v1') IS NOT NULL AND to_regclass('brain.entity_profile_facts_v1') IS NOT NULL THEN 1 ELSE 2 END",
    ),
    (
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-profile-binding-identity-v1.sql"),
        "SELECT CASE WHEN EXISTS(SELECT 1 FROM pg_attribute WHERE attrelid=to_regclass('brain.entity_profile_facts_v1') AND attname='binding_identity_json' AND NOT attisdropped) THEN 1 ELSE 0 END",
    ),
    (
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-semantic-projection-v1.sql"),
        "SELECT CASE WHEN to_regclass('brain.entity_semantic_projections_v1') IS NULL AND to_regclass('brain.entity_patch_intervals_v1') IS NULL THEN 0 WHEN to_regclass('brain.entity_semantic_projections_v1') IS NOT NULL AND to_regclass('brain.entity_patch_intervals_v1') IS NOT NULL THEN 1 ELSE 2 END",
    ),
    (
        include_str!("../../../../scripts/migrations/2026-10-04-brain-entity-derived-receipts-v1.sql"),
        "SELECT CASE WHEN to_regclass('brain.entity_derived_receipts_v1') IS NULL THEN 0 ELSE 1 END",
    ),
];
const ENTITY_SHAPE_PROBE: &str = "SELECT e.entity_key,e.identity_json,
    f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id,f.fact_json,f.binding_identity_json,
    s.entity_key,s.source_id,s.logical_id,s.revision,s.fact_id,s.relative_pointer,
    s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit,
    i.entity_key,i.source_id,i.logical_id,i.revision,i.fact_id,i.current_patch_fact_id,i.interval_json,
    d.derived_source_id,d.derived_logical_id,d.derived_revision,d.receipt_json
    FROM brain.entity_profile_entities_v1 e,brain.entity_profile_facts_v1 f,
         brain.entity_semantic_projections_v1 s,brain.entity_patch_intervals_v1 i,
         brain.entity_derived_receipts_v1 d LIMIT 0";

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
    pub async fn check_entity_profile_schema(&self) -> Result<(), PortError> {
        self.check_core_schema().await?;
        let mut tx = self.pool.begin().await.map_err(compatibility_error)?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(compatibility_error)?;
        sqlx::query(ENTITY_SHAPE_PROBE)
            .fetch_all(&mut *tx)
            .await
            .map_err(|_| invalid("Spielprofilschema fehlt oder ist nicht lesbar"))?;
        tx.commit().await.map_err(compatibility_error)
    }

    pub async fn migrate_entity_profiles(&self) -> Result<(), PortError> {
        self.check_core_schema().await?;
        let mut tx = self.pool.begin().await.map_err(migration_error)?;
        sqlx::raw_sql(
            "SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms';
             SELECT pg_advisory_xact_lock(742110026112::bigint)",
        )
        .execute(&mut *tx)
        .await
        .map_err(migration_error)?;
        check_version(&mut tx).await?;
        let owner: bool = sqlx::query_scalar(
            "SELECT pg_has_role(current_user,nspowner,'USAGE') FROM pg_namespace WHERE nspname='brain'",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(migration_error)?;
        if !owner {
            return Err(invalid(
                "Spielprofilmigration benötigt den vorhandenen Schemaowner",
            ));
        }
        for (migration, state_query) in ENTITY_MIGRATIONS {
            match sqlx::query_scalar::<_, i32>(state_query)
                .fetch_one(&mut *tx)
                .await
                .map_err(migration_error)?
            {
                0 => {
                    sqlx::raw_sql(body(migration)?)
                        .execute(&mut *tx)
                        .await
                        .map_err(migration_error)?;
                }
                1 => {}
                _ => return Err(invalid("Spielprofilschema ist nur teilweise vorhanden")),
            }
        }
        sqlx::query(ENTITY_SHAPE_PROBE)
            .fetch_all(&mut *tx)
            .await
            .map_err(migration_error)?;
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
        for (migration, _) in ENTITY_MIGRATIONS {
            assert!(body(migration).is_ok());
        }
    }
}
