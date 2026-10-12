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
        2,
        include_str!(
            "../../../../../scripts/migrations/2026-10-12-brain-gamefacts-p3-timers-v1.sql"
        ),
    ),
];

fn invalid() -> PortError {
    PortError::InvalidResponse("Spielwertschema fehlt, ist verändert oder nicht kompatibel".into())
}

fn ordered() -> Result<(), PortError> {
    for (index, (version, _)) in MIGRATIONS.iter().enumerate() {
        if i32::try_from(index + 1).ok() != Some(*version) {
            return Err(invalid());
        }
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

async fn check(connection: &mut PgConnection) -> Result<(), PortError> {
    ordered()?;
    let expected: Vec<_> = MIGRATIONS
        .iter()
        .map(|(v, s)| (*v, sql_and_checksum(s).1))
        .collect();
    if recorded(connection).await? != expected {
        return Err(invalid());
    }
    sqlx::query("SELECT o.bytes,o.content_sha256,r.status,r.finished,j.detail,e.key,i.receipt_id,a.alias_norm,f.value_bool,f.valid_to_run,l.to_entity,p.revision,ce.key,ca.alias_norm,cf.stat_key,cr.kind FROM raw.object o,ops.ingest_run r,ops.ingest_rejection j,canon.entity e,canon.entity_identity i,canon.alias a,canon.fact f,canon.relation l,serve.published_receipt p,serve.current_entity ce,serve.current_alias ca,serve.current_fact cf,serve.current_relation cr LIMIT 0")
        .fetch_all(&mut *connection).await.map_err(|_| invalid())?;
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
        AND NOT has_column_privilege('brain_ingest','canon.fact','value_num','UPDATE')
        AND NOT has_table_privilege('brain_tool','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_table_privilege('brain_service','canon.fact','INSERT,UPDATE,DELETE,TRUNCATE')
        AND NOT has_schema_privilege('brain_tool','raw','USAGE')
        AND NOT has_schema_privilege('brain_tool','ops','USAGE')")
        .fetch_one(connection).await.map_err(|_| invalid())?;
    if !valid {
        return Err(invalid());
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
        check(&mut tx).await?;
        tx.commit().await.map_err(|_| invalid())
    }

    pub async fn migrate_gamefacts(&self) -> Result<(), PortError> {
        ordered()?;
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
        if existing.len() > MIGRATIONS.len() {
            return Err(invalid());
        }
        for (index, (version, template)) in MIGRATIONS.iter().enumerate() {
            let (sql, checksum) = sql_and_checksum(template);
            if let Some(saved) = existing.get(index) {
                if saved != &(*version, checksum) {
                    return Err(invalid());
                }
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
        check(&mut tx).await?;
        tx.commit().await.map_err(|_| invalid())
    }
}
