pub mod limits;
mod migrations;
pub mod receipts;
mod types;
pub use types::*;

use sha2::{Digest, Sha256};
use sqlx::{Connection, PgConnection, PgPool, Row};

async fn capture(
    pool: &PgPool,
    run: i64,
    source: &str,
    revision: &str,
    path: &str,
    bytes: Vec<u8>,
) -> Result<RawObject, sqlx::Error> {
    let content_sha256: [u8; 32] = Sha256::digest(&bytes).into();
    let mut digest = Sha256::new();
    digest.update(source.len().to_be_bytes());
    digest.update(source.as_bytes());
    digest.update(path.as_bytes());
    let source_path_sha256 = digest.finalize().to_vec();
    let inserted = sqlx::query("INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(source_path_sha256,content_sha256) DO NOTHING RETURNING id")
        .bind(source).bind(revision).bind(path).bind(&source_path_sha256).bind(content_sha256.as_slice()).bind(&bytes).bind(run).fetch_optional(pool).await?;
    if let Some(row) = inserted {
        return Ok(RawObject {
            id: row.try_get("id")?,
            source: source.into(),
            revision: revision.into(),
            path: path.into(),
            content_sha256,
            bytes,
        });
    }
    let row = sqlx::query("SELECT id,source,revision,path FROM raw.object WHERE source_path_sha256=$1 AND content_sha256=$2")
        .bind(source_path_sha256).bind(content_sha256.as_slice()).fetch_one(pool).await?;
    if row.try_get::<String, _>("source")? != source || row.try_get::<String, _>("path")? != path {
        return Err(sqlx::Error::Protocol(
            "Rohobjekt-Schlüssel kollidiert".into(),
        ));
    }
    Ok(RawObject {
        id: row.try_get("id")?,
        source: row.try_get("source")?,
        revision: row.try_get("revision")?,
        path: row.try_get("path")?,
        content_sha256,
        bytes,
    })
}

async fn receipt(
    connection: &mut PgConnection,
    run: i64,
    raw: i64,
    extractor: &str,
    value: &Receipt,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,$3,$4,$5,$6) RETURNING id")
        .bind(raw).bind(run).bind(extractor).bind(&value.locator_kind).bind(&value.locator).bind(value.value_sha256.as_slice()).fetch_one(connection).await
}

async fn entity(connection: &mut PgConnection, key: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
        .bind(limits::normalize_key(key))
        .fetch_one(connection)
        .await
}

async fn write(
    connection: &mut PgConnection,
    run: i64,
    raw: i64,
    extractor: &str,
    set: &FactSet,
    bytes: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(742110026113::bigint)")
        .execute(&mut *connection)
        .await?;
    if set.entities.is_empty()
        && set.aliases.is_empty()
        && set.facts.is_empty()
        && set.relations.is_empty()
    {
        let value = Receipt {
            locator_kind: "raw_bytes".into(),
            locator: String::new(),
            value_sha256: Sha256::digest(bytes).into(),
        };
        receipt(connection, run, raw, extractor, &value).await?;
    }
    for row in &set.entities {
        let receipt = receipt(connection, run, raw, extractor, &row.receipt).await?;
        sqlx::query("INSERT INTO canon.entity(kind,key,receipt_id) VALUES($1,$2,$3) ON CONFLICT(key) DO NOTHING")
            .bind(limits::normalize_key(&row.kind)).bind(limits::normalize_key(&row.key)).bind(receipt).execute(&mut *connection).await?;
        let id: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1 AND kind=$2")
            .bind(limits::normalize_key(&row.key))
            .bind(limits::normalize_key(&row.kind))
            .fetch_one(&mut *connection)
            .await?;
        sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
            .bind(id)
            .bind(receipt)
            .execute(&mut *connection)
            .await?;
    }
    for row in &set.aliases {
        let id = entity(connection, &row.entity_key).await?;
        let receipt = receipt(connection, run, raw, extractor, &row.receipt).await?;
        sqlx::query(
            "INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) VALUES($1,$2,$3,$4)",
        )
        .bind(id)
        .bind(limits::normalize_alias(&row.alias))
        .bind(limits::normalize_key(&row.lang))
        .bind(receipt)
        .execute(&mut *connection)
        .await?;
    }
    for row in &set.facts {
        let id = entity(connection, &row.entity_key).await?;
        let key = limits::normalize_key(&row.stat_key);
        let receipt = receipt(connection, run, raw, extractor, &row.receipt).await?;
        let (num, text, boolean) = match &row.value {
            FactValue::Number(n) => (Some(*n), None, None),
            FactValue::Text(t) => (None, Some(t.as_str()), None),
            FactValue::Bool(b) => (None, None, Some(*b)),
        };
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,value_text,value_bool,valid_from_run,receipt_id) VALUES($1,$2,$3,$4,$5,$6,$7)").bind(id).bind(key).bind(num).bind(text).bind(boolean).bind(run).bind(receipt).execute(&mut *connection).await?;
    }
    for row in &set.relations {
        let from = entity(connection, &row.from_key).await?;
        let to = entity(connection, &row.to_key).await?;
        let receipt = receipt(connection, run, raw, extractor, &row.receipt).await?;
        sqlx::query(
            "INSERT INTO canon.relation(from_entity,kind,to_entity,receipt_id) VALUES($1,$2,$3,$4)",
        )
        .bind(from)
        .bind(limits::normalize_key(&row.kind))
        .bind(to)
        .bind(receipt)
        .execute(&mut *connection)
        .await?;
    }
    Ok(())
}

async fn write_object(
    pool: &PgPool,
    run: i64,
    raw: i64,
    extractor: &str,
    set: &FactSet,
) -> Result<(), Rejection> {
    limits::validate(set)?;
    let mut tx = pool.begin().await.map_err(sql_rejection)?;
    let bytes: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE id=$1")
        .bind(raw)
        .fetch_one(&mut *tx)
        .await
        .map_err(sql_rejection)?;
    if let Err(rejection) = receipts::verify(&bytes, set) {
        tx.rollback().await.map_err(sql_rejection)?;
        return Err(rejection);
    }
    if let Err(error) = write(&mut tx, run, raw, extractor, set, &bytes).await {
        tx.rollback().await.map_err(sql_rejection)?;
        return Err(sql_rejection(error));
    }
    tx.commit().await.map_err(sql_rejection)
}

fn sql_rejection(error: sqlx::Error) -> Rejection {
    let code = error
        .as_database_error()
        .and_then(|e| e.code())
        .map(|c| c.into_owned())
        .unwrap_or_else(|| "unavailable".into());
    Rejection::new(
        "sql_write",
        format!("Kanonischer Schreibvorgang fehlgeschlagen ({code})"),
    )
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    pub accepted: i64,
    pub rejected: i64,
    pub ignored: i64,
}

#[derive(Clone)]
pub struct GameFactsStore {
    pool: PgPool,
}

impl GameFactsStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn lock_run(&self) -> Result<PgConnection, sqlx::Error> {
        let mut connection = PgConnection::connect_with(&self.pool.connect_options()).await?;
        sqlx::query("SELECT pg_advisory_lock(742110026114::bigint)")
            .execute(&mut connection)
            .await?;
        Ok(connection)
    }

    pub async fn begin_run(&self, source: &str) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("INSERT INTO ops.ingest_run(source) VALUES($1) RETURNING id")
            .bind(source)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn finish_run(
        &self,
        run: i64,
        succeeded: bool,
        counts: Counts,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE ops.ingest_run SET finished=clock_timestamp(),status=$2,accepted=$3,rejected=$4,ignored=$5 WHERE id=$1")
            .bind(run).bind(if succeeded { "succeeded" } else { "failed" })
            .bind(counts.accepted).bind(counts.rejected).bind(counts.ignored).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn capture(
        &self,
        run: i64,
        source: &str,
        revision: &str,
        path: &str,
        bytes: Vec<u8>,
    ) -> Result<RawObject, sqlx::Error> {
        capture(&self.pool, run, source, revision, path, bytes).await
    }

    pub async fn was_applied(&self, raw: i64, extractor: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM canon.receipt receipt JOIN ops.ingest_run run ON run.id=receipt.ingest_run_id WHERE receipt.raw_object_id=$1 AND receipt.extractor=$2 AND run.status='succeeded')")
            .bind(raw).bind(extractor).fetch_one(&self.pool).await
    }

    pub async fn apply(
        &self,
        run: i64,
        raw: &RawObject,
        extractor: &str,
        set: &FactSet,
    ) -> Result<(), Rejection> {
        write_object(&self.pool, run, raw.id, extractor, set).await
    }

    pub async fn reject(
        &self,
        run: i64,
        raw: i64,
        extractor: &str,
        value: &Rejection,
    ) -> Result<(), sqlx::Error> {
        let rejection = Rejection::new(value.code, value.detail.clone());
        sqlx::query("INSERT INTO ops.ingest_rejection(run_id,raw_object_id,extractor,code,detail) VALUES($1,$2,$3,$4,$5)")
            .bind(run).bind(raw).bind(extractor).bind(rejection.code).bind(rejection.detail).execute(&self.pool).await?;
        Ok(())
    }
}
