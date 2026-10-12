use crate::{memory_repository::validate_release, pg_jobs::database_error, PgStore};
use brain_contracts::{store::STORE_VERSION, CorpusRelease, PortError, SourceRecordV2};
use sqlx::{Connection, PgConnection, Row};
use std::collections::BTreeSet;

pub const CORE_SCHEMA_VERSION: i32 = 2;
pub(crate) const RESPONSE_AUDIT_SHAPE_PROBE: &str = "SELECT count(*)=11 FROM (VALUES
    ('audit_id','int8'),('recorded_at','timestamptz'),('request_id','text'),('model','text'),
    ('check_json','jsonb'),('raw_output','bytea'),('raw_output_complete','bool'),
    ('source_ids','jsonb'),('evidence_ids','jsonb'),('disposition','text'),('identifiers_redacted','bool')) expected(name,type)
    JOIN pg_attribute a ON a.attrelid=to_regclass('brain.response_deviations_v1') AND a.attname=expected.name
    WHERE a.atttypid=expected.type::regtype AND a.attnotnull AND NOT a.attisdropped";

const GAMEFACTS_MIGRATIONS: &[(i32, &str)] = &[(
    1,
    include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"),
)];
const GAMEFACTS_V1_MIN_SERIALIZE_TRIGGERS: i64 = 14;
const GAMEFACTS_V1_SHAPE_PROBE: &str = "SELECT s.id,s.last_complete_revision,o.id,o.content_sha256,o.body,
    r.id,r.base_revision,r.target_revision,r.extractor_version,ro.run_id,ro.path,ro.parse_status,
    v.id,v.source_revision,h.published_revision_id,p.id,c.id,c.raw_object_id,c.value_sha256,
    e.id,e.identity_receipt_id,d.key,d.section,d.unit,f.id,f.receipt_id,f.valid_from_revision_id,f.valid_to_revision_id,
    a.alias_norm,a.receipt_id,l.slot,l.receipt_id,j.id,j.counts,k.epoch,t.question_sha256,
    hp.entity_id,hp.identity,hp.stats,hp.weapon,hp.abilities,cf.receipt_id,ca.alias_norm,cr.slot
    FROM raw.source s,raw.object o,raw.run r,raw.run_object ro,canon.revision v,canon.source_head h,
    canon.patch p,canon.receipt c,canon.entity e,canon.stat d,canon.fact f,canon.alias a,canon.relation l,
    ops.job_run j,ops.job_lease k,ops.answer_trace t,serve.hero_profile hp,serve.current_fact cf,
    serve.current_alias ca,serve.current_relation cr LIMIT 0";
const GAMEFACTS_ROLE_BOOTSTRAP: &str = "DO $roles$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_tool') THEN
            CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
        END IF;
        IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_tool' AND NOT rolcanlogin
            AND NOT rolsuper AND NOT rolcreatedb AND NOT rolcreaterole AND NOT rolreplication AND NOT rolbypassrls)
            OR EXISTS (SELECT 1 FROM pg_auth_members m JOIN pg_roles r ON r.oid=m.member WHERE r.rolname='brain_tool') THEN
            RAISE EXCEPTION 'Werkzeugrolle besitzt unzulässige Rechte';
        END IF;
        GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE;
    END
    $roles$; SET LOCAL ROLE brain_migrate";

const RESPONSE_AUDIT: &str =
    include_str!("../../../../scripts/migrations/2026-10-08-brain-response-audit-v1.sql");
const V1: &str = include_str!("../../../../scripts/migrations/2026-09-24-brain-contract-v1.sql");
const V2: &str = include_str!("../../../../scripts/migrations/2026-09-25-brain-core-jobs-v2.sql");
const VERSION: &str =
    include_str!("../../../../scripts/migrations/2026-09-26-brain-core-compatibility-v2.sql");
const READ_HEADERS: &str =
    include_str!("../../../../scripts/migrations/2026-10-06-brain-source-read-headers-v1.sql");
const READ_HEADER_WRITER: &str = include_str!(
    "../../../../scripts/migrations/2026-10-06-brain-source-read-header-writer-v1.sql"
);
const PATCH_STORY_INLINE: &str =
    include_str!("../../../../scripts/migrations/2026-10-07-brain-patch-story-inline-v1.sql");
const PATCH_STORY_ABILITY_PUSHDOWN: &str = include_str!(
    "../../../../scripts/migrations/2026-10-09-brain-patch-story-ability-pushdown-v1.sql"
);
const ENTITY_MIGRATIONS: [(&str, &str); 5] = [
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
    (
        include_str!("../../../../scripts/migrations/2026-10-09-brain-entity-lookup-names-v1.sql"),
        "SELECT CASE WHEN EXISTS(SELECT 1 FROM pg_attribute WHERE attrelid=to_regclass('brain.entity_profile_entities_v1') AND attname='lookup_names' AND NOT attisdropped) THEN 1 ELSE 0 END",
    ),
];
const ENTITY_SHAPE_PROBE: &str = "SELECT e.entity_key,e.identity_json,e.lookup_names,
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

pub(crate) const SHAPE_PROBE: &str = "SELECT r.source_id,r.logical_id,r.revision,r.content_hash,r.tombstone,r.record_json,r.created_at,r.read_header_json,r.original_header_json,
    h.source_id,h.logical_id,h.revision,h.content_hash,h.tombstone,h.record_json,h.updated_at,h.read_header_json,
    c.release_id,c.knowledge_version,c.patch,c.release_json,c.created_at,
    j.source_id,j.owner,j.fence,j.lease_until,j.state,j.updated_at,
    p.source_id,p.configuration,p.generation,p.checkpoint_json,p.batch_json,p.updated_at,
    o.conversation_id,o.actor_id,o.created_at
    FROM brain.source_record_revisions r, brain.source_record_heads h, brain.corpus_releases_v1 c,
         brain.source_jobs_v1 j, brain.source_checkpoints_v1 p, brain.conversation_owners_v1 o
    LIMIT 0";

pub(crate) const READ_HEADER_CONTRACT_PROBE: &str = "SELECT
    (SELECT count(*)=3 FROM (VALUES
        ('source_record_revisions','read_header_json','source_read_header_v1'),
        ('source_record_revisions','original_header_json','source_original_header_v1'),
        ('source_record_heads','read_header_json','source_read_header_v1')) expected(relation,column_name,function_name)
    JOIN pg_namespace n ON n.nspname='brain'
    JOIN pg_class c ON c.relnamespace=n.oid AND c.relname=expected.relation
    JOIN pg_attribute a ON a.attrelid=c.oid AND a.attname=expected.column_name AND NOT a.attisdropped
    JOIN pg_attrdef d ON d.adrelid=c.oid AND d.adnum=a.attnum
    WHERE a.attgenerated='s' AND a.atttypid='jsonb'::regtype
      AND pg_get_expr(d.adbin,d.adrelid) IN
          (expected.function_name || '(record_json)', 'brain.' || expected.function_name || '(record_json)')
      AND EXISTS(SELECT 1 FROM pg_depend dependency
          JOIN pg_proc function ON function.oid=dependency.refobjid
          JOIN pg_namespace function_namespace ON function_namespace.oid=function.pronamespace
          WHERE dependency.classid='pg_attrdef'::regclass AND dependency.objid=d.oid
            AND dependency.refclassid='pg_proc'::regclass
            AND function_namespace.nspname='brain' AND function.proname=expected.function_name
            AND function.pronargs=1 AND function.proargtypes[0]='jsonb'::regtype))
    AND (SELECT count(*)=2 FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace
        WHERE n.nspname='brain' AND p.proname IN ('source_read_header_v1','source_original_header_v1')
          AND p.pronargs=1 AND p.proargtypes[0]='jsonb'::regtype
          AND p.prorettype='jsonb'::regtype AND p.provolatile='i' AND NOT p.prosecdef)";

async fn check_read_headers(connection: &mut PgConnection) -> Result<(), PortError> {
    let valid: bool = sqlx::query_scalar(READ_HEADER_CONTRACT_PROBE)
        .fetch_one(connection)
        .await
        .map_err(compatibility_error)?;
    if !valid {
        return Err(invalid(
            "Gespeicherte Leseheader erfüllen den Schemavertrag nicht",
        ));
    }
    Ok(())
}

async fn check_response_audit(connection: &mut PgConnection) -> Result<(), PortError> {
    sqlx::query("SELECT audit_id,recorded_at,request_id,model,check_json,raw_output,raw_output_complete,source_ids,evidence_ids,disposition,identifiers_redacted FROM brain.response_deviations_v1 LIMIT 0")
        .fetch_all(&mut *connection).await.map_err(compatibility_error)?;
    let valid: bool = sqlx::query_scalar(RESPONSE_AUDIT_SHAPE_PROBE)
        .fetch_one(connection)
        .await
        .map_err(compatibility_error)?;
    if !valid {
        return Err(invalid("incompatible response audit schema"));
    }
    Ok(())
}

fn gamefacts_listed_versions(migrations: &[(i32, &str)]) -> Result<BTreeSet<i32>, PortError> {
    let versions: BTreeSet<_> = migrations.iter().map(|(version, _)| *version).collect();
    if versions.is_empty() || versions.len() != migrations.len() || versions.iter().any(|v| *v <= 0)
    {
        return Err(invalid(
            "Spielwertmigrationen benötigen eindeutige positive Versionen",
        ));
    }
    Ok(versions)
}

async fn gamefacts_recorded_versions(
    connection: &mut PgConnection,
) -> Result<BTreeSet<i32>, PortError> {
    let versions: Vec<i32> = sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version")
        .fetch_all(connection)
        .await
        .map_err(|_| invalid("Spielwertschema fehlt oder ist nicht lesbar"))?;
    Ok(versions.into_iter().collect())
}

async fn check_gamefacts(
    connection: &mut PgConnection,
    migrations: &[(i32, &str)],
) -> Result<(), PortError> {
    if gamefacts_recorded_versions(connection).await? != gamefacts_listed_versions(migrations)? {
        return Err(invalid(
            "Spielwertschema besitzt fehlende oder unbekannte Versionen",
        ));
    }
    check_gamefacts_v1(connection).await
}

async fn check_gamefacts_v1(connection: &mut PgConnection) -> Result<(), PortError> {
    sqlx::query(GAMEFACTS_V1_SHAPE_PROBE)
        .fetch_all(&mut *connection)
        .await
        .map_err(|_| invalid("Spielwertschema erfüllt den Spaltenvertrag nicht"))?;
    let valid: bool = sqlx::query_scalar(
        "SELECT
        (SELECT count(*)=5 FROM pg_namespace n JOIN pg_roles r ON r.oid=n.nspowner
            WHERE n.nspname IN ('raw','canon','docs','serve','ops') AND r.rolname='brain_migrate')
        AND EXISTS (SELECT 1 FROM pg_index WHERE indrelid='serve.hero_profile'::regclass
            AND indisunique AND indisvalid AND indpred IS NULL AND indexprs IS NULL AND indnatts=1)
        AND EXISTS (SELECT 1 FROM pg_class c JOIN pg_roles r ON r.oid=c.relowner
            WHERE c.oid='serve.hero_profile'::regclass AND c.relispopulated AND r.rolname='brain_migrate')
        AND EXISTS (SELECT 1 FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner
            WHERE p.oid='serve.refresh_hero_profile()'::regprocedure AND p.prosecdef
            AND p.proconfig=ARRAY['search_path=pg_catalog, pg_temp'] AND r.rolname='brain_migrate')
        AND EXISTS (SELECT 1 FROM pg_auth_members m JOIN pg_roles tool ON tool.oid=m.roleid
            JOIN pg_roles reader ON reader.oid=m.member WHERE tool.rolname='brain_tool'
            AND reader.rolname='brain_readonly' AND m.set_option AND NOT m.inherit_option AND NOT m.admin_option)
        AND EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_tool' AND NOT rolcanlogin
            AND NOT rolsuper AND NOT rolcreatedb AND NOT rolcreaterole AND NOT rolreplication AND NOT rolbypassrls)
        AND NOT EXISTS (SELECT 1 FROM pg_auth_members m JOIN pg_roles r ON r.oid=m.member WHERE r.rolname='brain_tool')
        AND NOT pg_has_role('brain_ingest','brain_migrate','MEMBER')
        AND NOT has_schema_privilege('brain_ingest','canon','CREATE')
        AND NOT has_table_privilege('brain_service','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_table_privilege('brain_tool','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_schema_privilege('brain_tool','raw','USAGE')
        AND NOT has_schema_privilege('brain_tool','ops','USAGE')
        AND NOT has_function_privilege('brain_tool','serve.refresh_hero_profile()','EXECUTE')
        AND has_function_privilege('brain_ingest','serve.refresh_hero_profile()','EXECUTE')
        AND NOT has_column_privilege('brain_ingest','canon.fact','value_num','UPDATE')
        AND NOT EXISTS (SELECT 1 FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid
            JOIN pg_namespace n ON n.oid=c.relnamespace
            WHERE n.nspname IN ('raw','canon','ops') AND a.attidentity <> ''
                AND has_column_privilege('brain_ingest',c.oid,a.attnum,'INSERT'))
        AND NOT has_column_privilege('brain_service','ops.answer_trace','id','INSERT')
        AND NOT has_table_privilege('brain_service','ops.answer_trace','SELECT,UPDATE,DELETE,TRUNCATE')
        AND has_column_privilege('brain_service','ops.answer_trace','question_sha256','INSERT')
        AND (SELECT count(*)=4 FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid
            WHERE c.relname IN ('fact_source_history_idx','alias_source_history_idx','relation_source_history_idx','relation_from_entity_idx')
                AND i.indisvalid AND i.indpred IS NULL AND i.indexprs IS NULL)
        AND (SELECT count(*) >= $1 FROM pg_trigger t WHERE t.tgfoid='ops.serialize_gamefacts_write()'::regprocedure
            AND t.tgenabled='O' AND t.tgtype & 1 = 0 AND t.tgtype & 2 = 2)
        AND NOT has_table_privilege('brain_ingest','raw.object','UPDATE,DELETE,TRUNCATE')
        AND NOT has_table_privilege('brain_ingest','canon.receipt','UPDATE,DELETE,TRUNCATE')",
    )
    .bind(GAMEFACTS_V1_MIN_SERIALIZE_TRIGGERS)
    .fetch_one(connection)
    .await
    .map_err(|_| invalid("Spielwertschema erfüllt den Rollenvertrag nicht"))?;
    if !valid {
        return Err(invalid(
            "Spielwertschema erfüllt den Eigentums- oder Rollenvertrag nicht",
        ));
    }
    Ok(())
}

async fn prepare_gamefacts_migration(connection: &mut PgConnection) -> Result<(), PortError> {
    sqlx::raw_sql("SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms'; SELECT pg_advisory_xact_lock(742110026112::bigint)")
        .execute(&mut *connection).await.map_err(migration_error)?;
    let is_dba: bool =
        sqlx::query_scalar("SELECT rolsuper FROM pg_roles WHERE rolname=current_user")
            .fetch_one(&mut *connection)
            .await
            .map_err(migration_error)?;
    if is_dba {
        sqlx::raw_sql(GAMEFACTS_ROLE_BOOTSTRAP)
            .execute(&mut *connection)
            .await
            .map_err(|_| invalid("Werkzeugrolle konnte nicht sicher vorbereitet werden"))?;
    }
    let owner: bool = sqlx::query_scalar("SELECT current_user='brain_migrate'")
        .fetch_one(connection)
        .await
        .map_err(migration_error)?;
    if !owner {
        return Err(PortError::PermissionDenied(
            "Spielwertmigration benötigt brain_migrate oder den lokalen DBA".into(),
        ));
    }
    Ok(())
}

async fn apply_gamefacts_migrations(
    connection: &mut PgConnection,
    migrations: &[(i32, &str)],
) -> Result<(), PortError> {
    let expected = gamefacts_listed_versions(migrations)?;
    for (index, (version, sql)) in migrations.iter().enumerate() {
        let mut tx = connection.begin().await.map_err(migration_error)?;
        prepare_gamefacts_migration(&mut tx).await?;
        let installed: bool =
            sqlx::query_scalar("SELECT to_regclass('ops.gamefacts_schema_version') IS NOT NULL")
                .fetch_one(&mut *tx)
                .await
                .map_err(migration_error)?;
        let recorded = if installed {
            gamefacts_recorded_versions(&mut tx).await?
        } else {
            BTreeSet::new()
        };
        if !recorded.is_subset(&expected) {
            return Err(invalid(
                "Spielwertschema besitzt unbekannte Versionen; eine ältere Programmversion darf es nicht zurücksetzen",
            ));
        }
        if !recorded.contains(version) {
            sqlx::raw_sql(body(sql)?)
                .execute(&mut *tx)
                .await
                .map_err(|_| invalid("Spielwertmigration ist fehlgeschlagen; die Transaktion wurde nicht veröffentlicht"))?;
            sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES ($1)")
                .bind(version)
                .execute(&mut *tx)
                .await
                .map_err(migration_error)?;
        }
        if index + 1 == migrations.len() {
            check_gamefacts(&mut tx, migrations).await?;
        } else {
            check_gamefacts_v1(&mut tx).await?;
        }
        tx.commit().await.map_err(migration_error)?;
    }
    Ok(())
}

impl PgStore {
    pub async fn check_gamefacts_schema(&self) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(compatibility_error)?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(compatibility_error)?;
        check_gamefacts(&mut tx, GAMEFACTS_MIGRATIONS).await?;
        tx.commit().await.map_err(compatibility_error)
    }

    pub async fn migrate_gamefacts(&self) -> Result<(), PortError> {
        let mut connection = self.pool.acquire().await.map_err(migration_error)?;
        apply_gamefacts_migrations(&mut connection, GAMEFACTS_MIGRATIONS).await
    }

    pub async fn check_response_audit_schema(&self) -> Result<(), PortError> {
        let mut connection = self.pool.acquire().await.map_err(compatibility_error)?;
        check_response_audit(&mut connection).await
    }

    pub async fn migrate_response_audit(&self) -> Result<(), PortError> {
        self.check_core_schema().await?;
        let mut tx = self.pool.begin().await.map_err(migration_error)?;
        sqlx::raw_sql("SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms'; SELECT pg_advisory_xact_lock(742110026112::bigint)")
            .execute(&mut *tx).await.map_err(migration_error)?;
        let owner: bool = sqlx::query_scalar("SELECT pg_has_role(current_user,nspowner,'USAGE') FROM pg_namespace WHERE nspname='brain'")
            .fetch_one(&mut *tx).await.map_err(migration_error)?;
        if !owner {
            return Err(PortError::PermissionDenied(
                "response audit migration requires schema owner".into(),
            ));
        }
        sqlx::raw_sql(body(RESPONSE_AUDIT)?)
            .execute(&mut *tx)
            .await
            .map_err(migration_error)?;
        check_response_audit(&mut tx).await?;
        tx.commit().await.map_err(migration_error)
    }

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
        check_read_headers(&mut tx).await?;
        tx.commit().await.map_err(compatibility_error)
    }

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
        for migration in [V1, V2, READ_HEADERS, READ_HEADER_WRITER, RESPONSE_AUDIT] {
            sqlx::raw_sql(body(migration)?)
                .execute(&mut *tx)
                .await
                .map_err(migration_error)?;
        }
        let patch_tables: bool = sqlx::query_scalar("SELECT to_regclass('brain.patch_events') IS NOT NULL AND to_regclass('brain.patch_event_enrichments') IS NOT NULL AND to_regclass('patchnotes.changelog_posts') IS NOT NULL")
            .fetch_one(&mut *tx).await.map_err(migration_error)?;
        if patch_tables {
            for migration in [PATCH_STORY_INLINE, PATCH_STORY_ABILITY_PUSHDOWN] {
                sqlx::raw_sql(body(migration)?)
                    .execute(&mut *tx)
                    .await
                    .map_err(migration_error)?;
            }
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
        check_read_headers(&mut tx).await?;
        check_response_audit(&mut tx).await?;
        tx.commit().await.map_err(migration_error)
    }
}

async fn validate_records(connection: &mut PgConnection) -> Result<(), PortError> {
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
        for sql in [PATCH_STORY_INLINE, PATCH_STORY_ABILITY_PUSHDOWN] {
            assert!(body(sql)
                .unwrap()
                .contains("CREATE OR REPLACE VIEW brain.patch_changes"));
        }
        assert!(body("BEGIN; SELECT 1; COMMIT; SELECT 2;").is_err());
        assert!(body("BEGIN; COMMIT; BEGIN; COMMIT;").is_err());
        for (migration, _) in ENTITY_MIGRATIONS {
            assert!(body(migration).is_ok());
        }
        for (_, migration) in GAMEFACTS_MIGRATIONS {
            assert!(body(migration).is_ok());
        }
    }
}

#[cfg(test)]
#[path = "../tests/support/gamefacts_migrations.rs"]
mod gamefacts_migration_tests;
