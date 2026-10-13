use crate::PgStore;
use brain_contracts::PortError;
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Row};

pub const MIGRATIONS: &[(i32, &str)] = &[
    (
        1,
        include_str!("../../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"),
    ),
    (
        7,
        include_str!(
            "../../../../../scripts/migrations/2026-10-13-brain-gamefacts-run-local-v7.sql"
        ),
    ),
    (
        8,
        include_str!(
            "../../../../../scripts/migrations/2026-10-13-brain-gamefacts-fact-integrity-v8.sql"
        ),
    ),
    (
        9,
        include_str!(
            "../../../../../scripts/migrations/2026-10-13-brain-gamefacts-application-cap-v9.sql"
        ),
    ),
    (
        10,
        include_str!(
            "../../../../../scripts/migrations/2026-10-13-brain-gamefacts-published-inserts-v10.sql"
        ),
    ),
];

fn invalid() -> PortError {
    PortError::InvalidResponse("Spielwertschema fehlt, ist verändert oder nicht kompatibel".into())
}

fn ordered(migrations: &[(i32, &str)]) -> Result<(), PortError> {
    let mut previous = 0;
    for (version, _) in migrations {
        if *version <= previous {
            return Err(invalid());
        }
        previous = *version;
    }
    Ok(())
}

async fn recorded(connection: &mut PgConnection) -> Result<Vec<(i32, Vec<u8>)>, PortError> {
    let rows =
        sqlx::query("SELECT version,checksum FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(connection)
            .await
            .map_err(|_| invalid())?;
    rows.into_iter()
        .map(|r| {
            Ok((
                r.try_get("version").map_err(|_| invalid())?,
                r.try_get("checksum").map_err(|_| invalid())?,
            ))
        })
        .collect()
}

fn sql_and_checksum(template: &str) -> (String, Vec<u8>) {
    let sql = super::limits::render_sql(template)
        .replace("{{HERO_PROFILE}}", include_str!("hero_profile.sql"));
    let checksum = Sha256::digest(sql.as_bytes()).to_vec();
    (sql, checksum)
}

async fn check(connection: &mut PgConnection, migrations: &[(i32, &str)]) -> Result<(), PortError> {
    ordered(migrations)?;
    let expected: Vec<_> = migrations
        .iter()
        .map(|(v, s)| (*v, sql_and_checksum(s).1))
        .collect();
    if recorded(connection).await? != expected {
        return Err(invalid());
    }
    sqlx::query("SELECT o.bytes,o.content_sha256,r.status,r.finished,j.detail,e.key,i.receipt_id,a.alias_norm,f.value_bool,f.valid_to_run,l.to_entity FROM raw.object o,ops.ingest_run r,ops.ingest_rejection j,canon.entity e,canon.entity_identity i,canon.alias a,canon.fact f,canon.relation l LIMIT 0")
        .fetch_all(&mut *connection).await.map_err(|_| invalid())?;
    let shape: bool = sqlx::query_scalar(
        "SELECT NOT EXISTS (
        SELECT 1 FROM (VALUES
            ('published_receipt','revision','text'),
            ('current_entity','key','canon.key'),
            ('current_alias','alias_norm','canon.short_text'),
            ('current_fact','stat_key','canon.key'),
            ('current_relation','kind','canon.key')
        ) AS required(relation,column_name,type_name)
        WHERE NOT EXISTS (
            SELECT 1 FROM pg_catalog.pg_namespace n
            JOIN pg_catalog.pg_class c ON c.relnamespace=n.oid
            JOIN pg_catalog.pg_attribute a ON a.attrelid=c.oid
            WHERE n.nspname='serve' AND c.relname=required.relation AND c.relkind='v'
                AND a.attname=required.column_name AND a.attnum>0 AND NOT a.attisdropped
                AND a.atttypid=pg_catalog.to_regtype(required.type_name)
        )
    )",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|_| invalid())?;
    if !shape {
        return Err(invalid());
    }
    let valid: bool = sqlx::query_scalar("SELECT
        (SELECT count(*)=4 FROM pg_namespace n JOIN pg_roles r ON r.oid=n.nspowner WHERE n.nspname IN('raw','canon','serve','ops') AND r.rolname='brain_migrate')
        AND EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_tool' AND NOT rolcanlogin AND NOT rolsuper AND NOT rolcreatedb AND NOT rolcreaterole AND NOT rolreplication AND NOT rolbypassrls)
        AND EXISTS(SELECT 1 FROM pg_auth_members m JOIN pg_roles r ON r.oid=m.roleid JOIN pg_roles u ON u.oid=m.member WHERE r.rolname='brain_tool' AND u.rolname='brain_readonly' AND NOT m.inherit_option AND m.set_option AND NOT m.admin_option)
        AND NOT pg_has_role('brain_readonly','brain_tool','USAGE')
        AND has_column_privilege('brain_service','ops.answer_trace','question_sha256','INSERT')
        AND NOT has_table_privilege('brain_service','ops.answer_trace','SELECT,UPDATE,DELETE,TRUNCATE')
        AND NOT pg_has_role('brain_ingest','brain_migrate','MEMBER')
        AND NOT has_schema_privilege('brain_ingest','canon','CREATE')
        AND NOT has_table_privilege('brain_ingest','raw.object','UPDATE,DELETE,TRUNCATE')
        AND NOT has_table_privilege('brain_ingest','canon.receipt','UPDATE,DELETE,TRUNCATE')
        AND NOT has_any_column_privilege('brain_ingest','canon.fact','UPDATE')
        AND NOT has_table_privilege('brain_tool','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_table_privilege('brain_service','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_schema_privilege('brain_tool','raw','USAGE')
        AND NOT has_schema_privilege('brain_tool','ops','USAGE')")
        .fetch_one(&mut *connection).await.map_err(|_| invalid())?;
    if !valid {
        return Err(invalid());
    }
    if migrations.iter().any(|(version, _)| *version == 7) {
        let protected: bool = sqlx::query_scalar("SELECT EXISTS(
            SELECT 1 FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner
            WHERE p.oid=to_regprocedure('canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint)')
                AND r.rolname='brain_migrate' AND p.prosecdef
                AND p.proconfig=ARRAY['search_path=pg_catalog']
                AND has_function_privilege('brain_ingest',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_tool',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_service',p.oid,'EXECUTE')
        ) AND EXISTS(
            SELECT 1 FROM pg_trigger t WHERE t.tgrelid='ops.ingest_run'::regclass
                AND t.tgname='keep_published_run' AND t.tgenabled='O'
                AND t.tgfoid=to_regprocedure('ops.keep_published_run()')
        )")
            .fetch_one(&mut *connection).await.map_err(|_| invalid())?;
        if !protected {
            return Err(invalid());
        }
    }
    if migrations.iter().any(|(version, _)| *version == 8) {
        let protected: bool = sqlx::query_scalar(
            "SELECT EXISTS(
            SELECT 1 FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner
            WHERE p.oid=to_regprocedure('canon.guard_fact_integrity()')
                AND r.rolname='brain_migrate' AND p.prosecdef
                AND p.prorettype='pg_catalog.trigger'::regtype
                AND p.proconfig=ARRAY['search_path=pg_catalog']
                AND NOT has_function_privilege('brain_ingest',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_tool',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_service',p.oid,'EXECUTE')
        ) AND EXISTS(
            SELECT 1 FROM pg_trigger t WHERE t.tgrelid='canon.fact'::regclass
                AND t.tgname='guard_fact_integrity' AND t.tgenabled='O'
                AND t.tgfoid=to_regprocedure('canon.guard_fact_integrity()')
                AND t.tgtype=23 AND t.tgattr=''::int2vector AND t.tgqual IS NULL
                AND NOT t.tgisinternal AND t.tgconstraint=0
        )",
        )
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| invalid())?;
        if !protected {
            return Err(invalid());
        }
    }
    if let Some((_, template)) = migrations.iter().find(|(version, _)| *version == 9) {
        let (sql, _) = sql_and_checksum(template);
        let body = sql.split("$rows$").nth(1).ok_or_else(invalid)?;
        let protected: bool = sqlx::query_scalar(
            "SELECT EXISTS(
            SELECT 1 FROM pg_attribute a JOIN pg_attrdef d ON d.adrelid=a.attrelid AND d.adnum=a.attnum
            WHERE a.attrelid='canon.receipt'::regclass AND a.attname='application_xid'
                AND a.atttypid='pg_catalog.xid8'::regtype AND a.attnotnull AND NOT a.attisdropped
                AND a.attgenerated='' AND a.attidentity=''
                AND pg_get_expr(d.adbin,d.adrelid)='pg_current_xact_id()'
                AND NOT has_column_privilege('brain_ingest',a.attrelid,a.attname,'INSERT,UPDATE')
        ) AND EXISTS(
            SELECT 1 FROM pg_proc p JOIN pg_roles r ON r.oid=p.proowner
            WHERE p.oid=to_regprocedure('canon.check_object_rows()')
                AND r.rolname='brain_migrate' AND NOT p.prosecdef
                AND p.prorettype='pg_catalog.trigger'::regtype AND p.prosrc=$1
                AND p.proconfig=ARRAY['search_path=pg_catalog']
                AND NOT has_function_privilege('brain_ingest',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_tool',p.oid,'EXECUTE')
                AND NOT has_function_privilege('brain_service',p.oid,'EXECUTE')
        ) AND EXISTS(
            SELECT 1 FROM pg_trigger t WHERE t.tgrelid='canon.receipt'::regclass
                AND t.tgname='object_rows' AND t.tgenabled='O'
                AND t.tgfoid=to_regprocedure('canon.check_object_rows()')
                AND t.tgtype=5 AND t.tgattr=''::int2vector AND t.tgqual IS NULL
                AND NOT t.tgisinternal AND t.tgdeferrable AND t.tginitdeferred
        ) AND EXISTS(
            SELECT 1 FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid
            JOIN pg_namespace n ON n.oid=c.relnamespace JOIN pg_am am ON am.oid=c.relam
            WHERE i.indrelid='canon.receipt'::regclass AND n.nspname='canon'
                AND c.relname='receipt_object_application' AND am.amname='btree'
                AND i.indisvalid AND i.indisready AND i.indislive AND i.indnkeyatts=4
                AND i.indnatts=4 AND i.indpred IS NULL AND i.indexprs IS NULL
                AND ARRAY(SELECT a.attname::text FROM unnest(i.indkey) WITH ORDINALITY k(num,pos)
                    JOIN pg_attribute a ON a.attrelid=i.indrelid AND a.attnum=k.num ORDER BY k.pos)
                    =ARRAY['raw_object_id','ingest_run_id','extractor','application_xid']
        )",
        )
        .bind(body)
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| invalid())?;
        if !protected {
            return Err(invalid());
        }
    }
    if let Some((_, template)) = migrations.iter().find(|(version, _)| *version == 10) {
        let (sql, _) = sql_and_checksum(template);
        let receipt_body = sql.split("$receipt$").nth(1).ok_or_else(invalid)?;
        let reference_body = sql.split("$reference$").nth(1).ok_or_else(invalid)?;
        let protected: bool = sqlx::query_scalar(
            "SELECT NOT EXISTS (
            SELECT 1 FROM (VALUES
                ('canon.guard_receipt_insert()', $1::text),
                ('canon.guard_reference_insert()', $2::text)
            ) AS required(signature,body)
            WHERE NOT EXISTS (
                SELECT 1 FROM pg_catalog.pg_proc p
                JOIN pg_catalog.pg_roles r ON r.oid=p.proowner
                JOIN pg_catalog.pg_language l ON l.oid=p.prolang
                WHERE p.oid=pg_catalog.to_regprocedure(required.signature)
                    AND r.rolname='brain_migrate' AND l.lanname='plpgsql' AND p.prosecdef
                    AND p.prokind='f' AND p.pronargs=0 AND p.pronargdefaults=0
                    AND p.proallargtypes IS NULL AND p.provariadic=0 AND NOT p.proretset
                    AND p.prorettype='pg_catalog.trigger'::regtype AND p.prosrc=required.body
                    AND p.proconfig=ARRAY['search_path=pg_catalog']
                    AND NOT EXISTS (
                        SELECT 1 FROM pg_catalog.aclexplode(COALESCE(p.proacl,pg_catalog.acldefault('f',p.proowner))) a
                        WHERE a.grantee=0 AND a.privilege_type='EXECUTE'
                    )
                    AND NOT has_function_privilege('brain_ingest',p.oid,'EXECUTE')
                    AND NOT has_function_privilege('brain_tool',p.oid,'EXECUTE')
                    AND NOT has_function_privilege('brain_service',p.oid,'EXECUTE')
                    AND NOT has_function_privilege('brain_readonly',p.oid,'EXECUTE')
            )
        ) AND NOT EXISTS (
            SELECT 1 FROM (VALUES
                ('canon.receipt','canon.guard_receipt_insert()'),
                ('canon.entity','canon.guard_reference_insert()'),
                ('canon.entity_identity','canon.guard_reference_insert()'),
                ('canon.alias','canon.guard_reference_insert()'),
                ('canon.relation','canon.guard_reference_insert()')
            ) AS required(relation,signature)
            WHERE NOT EXISTS (
                SELECT 1 FROM pg_catalog.pg_trigger t
                WHERE t.tgrelid=pg_catalog.to_regclass(required.relation)
                    AND t.tgname='guard_published_insert' AND t.tgenabled='O'
                    AND t.tgfoid=pg_catalog.to_regprocedure(required.signature)
                    AND t.tgtype=7 AND t.tgattr=''::int2vector AND t.tgqual IS NULL
                    AND NOT t.tgisinternal AND t.tgconstraint=0
            )
        )",
        )
        .bind(receipt_body)
        .bind(reference_body)
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| invalid())?;
        if !protected {
            return Err(invalid());
        }
    }
    Ok(())
}

impl PgStore {
    pub async fn check_gamefacts_schema(&self) -> Result<(), PortError> {
        let mut tx = self.pool.begin().await.map_err(|_| invalid())?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(|_| invalid())?;
        check(&mut tx, MIGRATIONS).await?;
        tx.commit().await.map_err(|_| invalid())
    }

    pub async fn migrate_gamefacts(&self) -> Result<(), PortError> {
        self.migrate_gamefacts_registered(MIGRATIONS).await
    }

    async fn migrate_gamefacts_registered(
        &self,
        migrations: &[(i32, &str)],
    ) -> Result<(), PortError> {
        ordered(migrations)?;
        let mut tx = self.pool.begin().await.map_err(|_| invalid())?;
        sqlx::raw_sql("SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms'; SELECT pg_advisory_xact_lock(742110026112::bigint)").execute(&mut *tx).await.map_err(|_| invalid())?;
        let dba: bool =
            sqlx::query_scalar("SELECT rolsuper FROM pg_roles WHERE rolname=current_user")
                .fetch_one(&mut *tx)
                .await
                .map_err(|_| invalid())?;
        if dba {
            sqlx::raw_sql("DO $roles$ BEGIN IF NOT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_tool') THEN CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS; END IF; END $roles$; GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE; SET LOCAL ROLE brain_migrate")
                .execute(&mut *tx).await.map_err(|_| invalid())?;
        }
        let owner: bool = sqlx::query_scalar("SELECT current_user='brain_migrate'")
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| invalid())?;
        if !owner {
            return Err(PortError::PermissionDenied(
                "Migration benötigt brain_migrate oder den lokalen DBA".into(),
            ));
        }
        let installed: bool =
            sqlx::query_scalar("SELECT to_regclass('ops.gamefacts_schema_version') IS NOT NULL")
                .fetch_one(&mut *tx)
                .await
                .map_err(|_| invalid())?;
        let existing = if installed {
            recorded(&mut tx).await?
        } else {
            Vec::new()
        };
        for (version, saved_checksum) in &existing {
            let (_, template) = migrations
                .iter()
                .find(|(v, _)| v == version)
                .ok_or_else(invalid)?;
            if *saved_checksum != sql_and_checksum(template).1 {
                return Err(invalid());
            }
        }
        for (version, template) in migrations {
            let (sql, checksum) = sql_and_checksum(template);
            if existing.binary_search_by_key(version, |(v, _)| *v).is_ok() {
                continue;
            }
            let body = sql
                .trim()
                .strip_prefix("BEGIN;")
                .and_then(|s| s.trim().strip_suffix("COMMIT;"))
                .ok_or_else(invalid)?;
            if body.contains("BEGIN;") || body.contains("COMMIT;") || body.contains("ROLLBACK;") {
                return Err(invalid());
            }
            sqlx::raw_sql(body)
                .execute(&mut *tx)
                .await
                .map_err(|_| invalid())?;
            sqlx::query("INSERT INTO ops.gamefacts_schema_version(version,checksum) VALUES($1,$2)")
                .bind(version)
                .bind(checksum)
                .execute(&mut *tx)
                .await
                .map_err(|_| invalid())?;
        }
        check(&mut tx, migrations).await?;
        tx.commit().await.map_err(|_| invalid())
    }
}

#[cfg(test)]
#[path = "migrations_tests.rs"]
mod tests;
