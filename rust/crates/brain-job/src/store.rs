use crate::{
    extract::{self, Assertion, Atom},
    git::{Blob, Source},
};
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    time::Duration,
};

const LEASE_KEY: &str = "brain-job:ingest-gamefiles";

#[derive(Default)]
pub struct Counts {
    pub raw_inserted: u64,
    pub raw_reused: u64,
    pub objects_completed: u64,
    pub objects_skipped: u64,
    pub paths_deleted: u64,
    pub sources_completed: u64,
    pub sources_unchanged: u64,
}
impl Counts {
    pub fn value(&self) -> Value {
        json!({"raw_inserted": self.raw_inserted, "raw_reused": self.raw_reused, "objects_completed": self.objects_completed, "objects_skipped": self.objects_skipped, "paths_deleted": self.paths_deleted, "sources_completed": self.sources_completed, "sources_unchanged": self.sources_unchanged})
    }
}

pub struct Store {
    pub pool: PgPool,
    pub job: i64,
    epoch: i64,
    seconds: i32,
}
pub struct Run {
    pub id: i64,
    pub source: i64,
    pub revision: i64,
    pub base: Option<String>,
    pub target: String,
    pub full: bool,
}

impl Store {
    pub async fn start(pool: PgPool, seconds: u32) -> Result<Self> {
        ensure!((30..=900).contains(&seconds), "lease_duration");
        ensure!(
            sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM ops.gamefacts_schema_version WHERE version=1)"
            )
            .fetch_one(&pool)
            .await?,
            "schema_version"
        );
        let seconds = i32::try_from(seconds).context("lease_duration")?;
        let (job, epoch) = retry_transaction(|| Self::start_once(&pool, seconds)).await?;
        Ok(Self {
            pool,
            job,
            epoch,
            seconds,
        })
    }

    async fn start_once(pool: &PgPool, seconds: i32) -> Result<(i64, i64)> {
        let mut tx = write_transaction(pool).await?;
        let job = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles',$1,$2) RETURNING id")
            .bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
        let epoch = sqlx::query_scalar("INSERT INTO ops.job_lease(lease_key,run_id,epoch,expires_at) VALUES ($1,$2,1,clock_timestamp()+make_interval(secs=>$3)) ON CONFLICT(lease_key) DO UPDATE SET run_id=EXCLUDED.run_id,epoch=ops.job_lease.epoch+1,expires_at=EXCLUDED.expires_at WHERE ops.job_lease.expires_at < clock_timestamp() RETURNING epoch")
            .bind(LEASE_KEY).bind(job).bind(seconds).fetch_optional(&mut *tx).await?.context("lease_busy")?;
        tx.commit().await?;
        Ok((job, epoch))
    }

    async fn renew_tx(&self, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        let changed = sqlx::query("UPDATE ops.job_lease SET expires_at=clock_timestamp()+make_interval(secs=>$4) WHERE lease_key=$1 AND run_id=$2 AND epoch=$3 AND expires_at>clock_timestamp()")
            .bind(LEASE_KEY).bind(self.job).bind(self.epoch).bind(self.seconds).execute(&mut **tx).await?.rows_affected();
        ensure!(changed == 1, "lease_lost");
        Ok(())
    }

    pub async fn renew(&self) -> Result<()> {
        retry_transaction(|| self.renew_once()).await
    }

    async fn renew_once(&self) -> Result<()> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn during<T>(&self, future: impl Future<Output = Result<T>>) -> Result<T> {
        tokio::pin!(future);
        let mut interval = tokio::time::interval(Duration::from_secs(
            u64::try_from(self.seconds / 3).context("lease_duration")?,
        ));
        interval.tick().await;
        loop {
            tokio::select! {
                result = &mut future => return result,
                _ = interval.tick() => self.renew().await?,
            }
        }
    }

    pub async fn source(&self, source: Source) -> Result<i64> {
        retry_transaction(|| self.source_once(source)).await
    }

    async fn source_once(&self, source: Source) -> Result<i64> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        sqlx::query("INSERT INTO raw.source(kind,locator) VALUES ('git',$1) ON CONFLICT(locator) DO NOTHING").bind(source.url()).execute(&mut *tx).await?;
        let id = sqlx::query_scalar("SELECT id FROM raw.source WHERE locator=$1 AND enabled")
            .bind(source.url())
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(id)
    }

    pub async fn pending(&self, source: i64) -> Result<Option<Run>> {
        retry_transaction(|| self.pending_once(source)).await
    }

    async fn pending_once(&self, source: i64) -> Result<Option<Run>> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let row = sqlx::query("SELECT r.id,r.base_revision,r.target_revision,r.extractor,r.extractor_version,v.id AS revision_id FROM raw.run r JOIN canon.revision v ON v.raw_run_id=r.id WHERE r.source_id=$1 AND r.status IN ('running','partial') FOR UPDATE OF r")
            .bind(source).fetch_optional(&mut *tx).await?;
        let Some(row) = row else {
            tx.commit().await?;
            return Ok(None);
        };
        ensure!(
            row.get::<String, _>("extractor") == extract::EXTRACTOR
                && row.get::<String, _>("extractor_version") == extract::VERSION,
            "resume_extractor_unavailable"
        );
        let id = row.get("id");
        sqlx::query("UPDATE raw.run SET job_run_id=$2,status='running' WHERE id=$1")
            .bind(id)
            .bind(self.job)
            .execute(&mut *tx)
            .await?;
        let full = self.needs_full_tx(source, &mut tx).await?;
        let run = Run {
            id,
            source,
            revision: row.get("revision_id"),
            base: row.get("base_revision"),
            target: row.get("target_revision"),
            full,
        };
        tx.commit().await?;
        Ok(Some(run))
    }

    async fn needs_full_tx(&self, source: i64, tx: &mut Transaction<'_, Postgres>) -> Result<bool> {
        let version = sqlx::query_scalar::<_, String>("SELECT v.extractor_version FROM canon.source_head h JOIN canon.revision v ON v.id=h.published_revision_id WHERE h.source_id=$1")
            .bind(source).fetch_optional(&mut **tx).await?;
        let failures: Vec<(String, String)> = sqlx::query_as(
            "SELECT DISTINCT r.extractor,r.extractor_version FROM raw.run r
            JOIN canon.revision v ON v.raw_run_id=r.id
            LEFT JOIN canon.source_head h ON h.source_id=r.source_id
            WHERE r.source_id=$1 AND r.status='failed'
            AND v.id>coalesce(h.published_revision_id,0)",
        )
        .bind(source)
        .fetch_all(&mut **tx)
        .await?;
        ensure!(
            failures
                .iter()
                .all(|(extractor, version)| extractor == extract::EXTRACTOR
                    && version == extract::VERSION),
            "resume_extractor_unavailable"
        );
        Ok(version.as_deref() != Some(extract::VERSION) || !failures.is_empty())
    }

    pub async fn fail(&self, run: &Run) -> Result<()> {
        retry_transaction(|| self.fail_once(run)).await
    }

    async fn fail_once(&self, run: &Run) -> Result<()> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let changed = sqlx::query(
            "UPDATE raw.run SET status='failed' WHERE id=$1 AND job_run_id=$2 AND status='running'",
        )
        .bind(run.id)
        .bind(self.job)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        ensure!(changed == 1, "run_not_owned");
        tx.commit().await?;
        Ok(())
    }

    pub async fn active_paths(&self, run: &Run) -> Result<BTreeSet<String>> {
        let paths: Vec<String> = sqlx::query_scalar(
            "WITH paths AS (
                SELECT source_path,receipt_id FROM canon.fact WHERE source_id=$1 AND valid_to_revision_id IS NULL
                UNION
                SELECT source_path,receipt_id FROM canon.alias WHERE source_id=$1 AND valid_to_revision_id IS NULL
                UNION
                SELECT source_path,receipt_id FROM canon.relation WHERE source_id=$1 AND valid_to_revision_id IS NULL
            ) SELECT DISTINCT a.source_path FROM paths a
            JOIN canon.receipt p ON p.id=a.receipt_id WHERE p.extractor=$2",
        )
        .bind(run.source)
        .bind(extract::EXTRACTOR)
        .fetch_all(&self.pool)
        .await?;
        Ok(paths.into_iter().collect())
    }

    pub async fn begin_run(&self, source: i64, target: &str) -> Result<Option<Run>> {
        retry_transaction(|| self.begin_run_once(source, target)).await
    }

    async fn begin_run_once(&self, source: i64, target: &str) -> Result<Option<Run>> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let base: Option<String> = sqlx::query_scalar(
            "SELECT last_complete_revision FROM raw.source WHERE id=$1 FOR UPDATE",
        )
        .bind(source)
        .fetch_one(&mut *tx)
        .await?;
        let full = self.needs_full_tx(source, &mut tx).await?;
        if base.as_deref() == Some(target) && !full {
            tx.commit().await?;
            return Ok(None);
        }
        let id = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id")
            .bind(source).bind(self.job).bind(&base).bind(target).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
        let revision = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5) RETURNING id")
            .bind(source).bind(id).bind(target).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(Some(Run {
            id,
            source,
            revision,
            base,
            target: target.to_owned(),
            full,
        }))
    }

    pub async fn completed(&self, run: &Run, path: &str) -> Result<bool> {
        Ok(sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM raw.run_object WHERE run_id=$1 AND path=$2 AND parse_status IN ('succeeded','skipped'))")
            .bind(run.id).bind(path).fetch_one(&self.pool).await?)
    }

    pub async fn object(
        &self,
        run: &Run,
        source: Source,
        path: &str,
        blob: Option<Blob>,
        counts: &mut Counts,
    ) -> Result<()> {
        let (inserted, had_raw, skipped) =
            retry_transaction(|| self.object_once(run, source, path, blob.as_ref())).await?;
        if inserted {
            counts.raw_inserted += 1;
        } else if had_raw {
            counts.raw_reused += 1;
        } else {
            counts.paths_deleted += 1;
        }
        counts.objects_completed += 1;
        if skipped {
            counts.objects_skipped += 1;
        }
        Ok(())
    }

    async fn object_once(
        &self,
        run: &Run,
        source: Source,
        path: &str,
        blob: Option<&Blob>,
    ) -> Result<(bool, bool, bool)> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let mut inserted = false;
        let raw = if let Some(blob) = blob {
            ensure!(
                blob.bytes.len() <= 32 * 1024 * 1024
                    && Sha256::digest(&blob.bytes).as_slice() == blob.hash,
                "raw_blob_proof"
            );
            let id = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT(source_id,external_id,content_sha256) DO NOTHING RETURNING id")
                .bind(run.source).bind(path).bind(&run.target).bind(&blob.hash).bind(media_type(path)).bind(&blob.bytes).fetch_optional(&mut *tx).await?;
            inserted = id.is_some();
            let id: i64 = match id {
                Some(id) => id,
                None => sqlx::query_scalar("SELECT id FROM raw.object WHERE source_id=$1 AND external_id=$2 AND content_sha256=$3")
                    .bind(run.source).bind(path).bind(&blob.hash).fetch_one(&mut *tx).await?,
            };
            Some((id, blob.hash.as_slice(), blob.bytes.len()))
        } else {
            None
        };
        sqlx::query(
            "INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES ($1,$2,$3,$4)",
        )
        .bind(run.id)
        .bind(path)
        .bind(raw.as_ref().map(|(id, _, _)| *id))
        .bind(if raw.is_some() { "upsert" } else { "delete" })
        .execute(&mut *tx)
        .await?;
        let had_raw = raw.is_some();
        let extracted = if let Some((id, hash, size)) = &raw {
            let stored_size: i32 =
                sqlx::query_scalar("SELECT octet_length(body) FROM raw.object WHERE id=$1")
                    .bind(id)
                    .fetch_one(&mut *tx)
                    .await?;
            ensure!(
                usize::try_from(stored_size).ok() == Some(*size),
                "stored_size_mismatch"
            );
            let bytes: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=$1")
                .bind(id)
                .fetch_one(&mut *tx)
                .await?;
            ensure!(
                Sha256::digest(&bytes).as_slice() == *hash,
                "stored_hash_mismatch"
            );
            if extract::supported(source, path) {
                let identities = if path.starts_with("data/localizations/") {
                    self.identity_keys(&mut tx, run).await?
                } else {
                    BTreeMap::new()
                };
                let assertions =
                    extract::extract_with_identities(source, path, &bytes, &identities)
                        .context(crate::InvalidTarget)?;
                extract::prove(source, path, &bytes, &assertions)?;
                Some(assertions)
            } else {
                None
            }
        } else {
            Some(Vec::new())
        };
        if let Some(assertions) = &extracted {
            self.close_path(&mut tx, run, path).await?;
            if let Some((object, _, _)) = &raw {
                for (index, assertion) in assertions.iter().enumerate() {
                    if index % 100 == 0 {
                        self.renew_tx(&mut tx).await?;
                    }
                    self.assertion(&mut tx, run, path, *object, assertion)
                        .await?;
                }
            }
        }
        let parse_status = if extracted.is_some() {
            "succeeded"
        } else {
            "skipped"
        };
        sqlx::query("UPDATE raw.run_object SET parse_status=$3 WHERE run_id=$1 AND path=$2")
            .bind(run.id)
            .bind(path)
            .bind(parse_status)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE raw.run SET last_completed_path=$2 WHERE id=$1")
            .bind(run.id)
            .bind(path)
            .execute(&mut *tx)
            .await?;
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok((inserted, had_raw, extracted.is_none()))
    }

    async fn identity_keys(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
    ) -> Result<BTreeMap<String, &'static str>> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "WITH evidence AS (
                SELECT entity_id,receipt_id,source_id,source_path,valid_from_revision_id,valid_to_revision_id
                FROM canon.fact WHERE source_id=$1
                UNION ALL
                SELECT from_entity_id,receipt_id,source_id,source_path,valid_from_revision_id,valid_to_revision_id
                FROM canon.relation WHERE source_id=$1
            )
            SELECT DISTINCT e.key,e.kind FROM canon.entity e
            JOIN evidence a ON a.entity_id=e.id
            JOIN canon.receipt r ON r.id=a.receipt_id
            JOIN raw.object o ON o.id=r.raw_object_id
            LEFT JOIN canon.source_head h ON h.source_id=a.source_id
            WHERE r.extractor=$2 AND r.extractor_version=$3
            AND ((e.kind='hero' AND a.source_path='data/json/hero-data.json')
                OR (e.kind='ability' AND a.source_path='data/json/ability-data.json'))
            AND o.external_id=a.source_path
            AND a.valid_from_revision_id<=$4
            AND (a.valid_to_revision_id IS NULL OR a.valid_to_revision_id>$4)
            AND (r.revision_id=$4 OR r.revision_id<=h.published_revision_id)
            ORDER BY e.key LIMIT 10001",
        )
        .bind(run.source)
        .bind(extract::EXTRACTOR)
        .bind(extract::VERSION)
        .bind(run.revision)
        .fetch_all(&mut **tx)
        .await?;
        if rows.len() > 10000 {
            return Err(anyhow::Error::new(crate::InvalidTarget).context("identity_limit"));
        }
        rows.into_iter()
            .map(|(key, kind)| {
                let kind = match kind.as_str() {
                    "hero" => "hero",
                    "ability" => "ability",
                    _ => anyhow::bail!("identity_kind"),
                };
                Ok((key, kind))
            })
            .collect()
    }

    async fn close_path(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        path: &str,
    ) -> Result<()> {
        for table in ["fact", "alias", "relation"] {
            sqlx::query(&format!("UPDATE canon.{table} a SET valid_to_revision_id=$3 FROM canon.receipt p WHERE a.source_id=$1 AND a.source_path=$2 AND a.valid_to_revision_id IS NULL AND p.id=a.receipt_id AND p.extractor=$4"))
                .bind(run.source).bind(path).bind(run.revision).bind(extract::EXTRACTOR).execute(&mut **tx).await?;
        }
        Ok(())
    }

    async fn receipt(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        object: i64,
        proof: &Assertion,
    ) -> Result<i64> {
        Ok(sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id")
            .bind(object).bind(run.source).bind(run.revision).bind(proof.locator_kind).bind(&proof.locator).bind(&proof.hash).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut **tx).await?)
    }

    async fn ensure_entity(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kind: &str,
        key: &str,
        receipt: i64,
    ) -> Result<i64> {
        sqlx::query("INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ($1,$2,$3) ON CONFLICT(kind,key) DO NOTHING").bind(kind).bind(key).bind(receipt).execute(&mut **tx).await?;
        Ok(
            sqlx::query_scalar("SELECT id FROM canon.entity WHERE kind=$1 AND key=$2")
                .bind(kind)
                .bind(key)
                .fetch_one(&mut **tx)
                .await?,
        )
    }

    async fn assertion(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        path: &str,
        object: i64,
        assertion: &Assertion,
    ) -> Result<()> {
        let existing = if path.starts_with("data/localizations/") {
            let id = sqlx::query_scalar::<_, i64>(
                "SELECT id FROM canon.entity WHERE kind=$1 AND key=$2",
            )
            .bind(assertion.kind)
            .bind(&assertion.entity)
            .fetch_optional(&mut **tx)
            .await?;
            let Some(id) = id else {
                return Ok(());
            };
            Some(id)
        } else {
            None
        };
        let receipt = self.receipt(tx, run, object, assertion).await?;
        let id = match existing {
            Some(id) => id,
            None => {
                self.ensure_entity(tx, assertion.kind, &assertion.entity, receipt)
                    .await?
            }
        };
        let alias = assertion
            .entity
            .strip_prefix("hero_")
            .or_else(|| assertion.entity.strip_prefix("ability_"))
            .unwrap_or(&assertion.entity);
        self.alias(tx, run, path, id, receipt, (alias, "und"))
            .await?;
        if let Some((text, lang)) = &assertion.alias {
            self.alias(tx, run, path, id, receipt, (text, lang)).await?;
        }
        if let Some((ability, slot)) = &assertion.relation {
            let target = self.ensure_entity(tx, "ability", ability, receipt).await?;
            sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,'has_ability',$2,$3,$4,$5,$6,$7)")
                .bind(id).bind(target).bind(slot).bind(receipt).bind(run.source).bind(path).bind(run.revision).execute(&mut **tx).await?;
        } else {
            self.fact(tx, run, path, receipt, id, assertion).await?;
        }
        Ok(())
    }

    async fn alias(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        path: &str,
        entity: i64,
        receipt: i64,
        alias: (&str, &str),
    ) -> Result<()> {
        let (text, lang) = alias;
        let normalized = text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if normalized.is_empty() || normalized.len() > 256 {
            return Ok(());
        }
        sqlx::query("INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(source_id,source_path,entity_id,alias_norm,lang) WHERE valid_to_revision_id IS NULL DO NOTHING")
            .bind(entity).bind(normalized).bind(lang).bind(receipt).bind(run.source).bind(path).bind(run.revision).execute(&mut **tx).await?;
        Ok(())
    }

    async fn fact(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        path: &str,
        receipt: i64,
        entity: i64,
        fact: &Assertion,
    ) -> Result<()> {
        let (number, text, value_type) = match &fact.value {
            Atom::Number(n) => (Some(n.as_str()), None, "number"),
            Atom::Text(s) => (None, Some(s.as_str()), "text"),
        };
        sqlx::query("INSERT INTO canon.stat(key,section,unit,value_type,display_order) VALUES ($1,$2,$3,$4,$5) ON CONFLICT(key) DO NOTHING")
            .bind(&fact.stat).bind(fact.section).bind(fact.unit).bind(value_type).bind(fact.order).execute(&mut **tx).await?;
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,value_text,receipt_id,valid_from_revision_id) VALUES ($1,$2,$3,$4,$5,$6::text::numeric,$7,$8,$9)")
            .bind(entity).bind(&fact.stat).bind(run.source).bind(path).bind(&fact.condition).bind(number).bind(text).bind(receipt).bind(run.revision).execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn publish(&self, run: &Run) -> Result<()> {
        retry_transaction(|| self.publish_once(run)).await
    }

    async fn publish_once(&self, run: &Run) -> Result<()> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
            .bind(run.source)
            .bind(&run.target)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
            .bind(run.id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2) ON CONFLICT(source_id) DO UPDATE SET published_revision_id=EXCLUDED.published_revision_id")
            .bind(run.source).bind(run.revision).execute(&mut *tx).await?;
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn finish(&self, counts: &Counts, failed: bool) -> Result<()> {
        retry_transaction(|| self.finish_once(counts, failed)).await
    }

    async fn finish_once(&self, counts: &Counts, failed: bool) -> Result<()> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        if failed {
            sqlx::query(
                "UPDATE raw.run SET status='partial' WHERE job_run_id=$1 AND status='running'",
            )
            .bind(self.job)
            .execute(&mut *tx)
            .await?;
        }
        sqlx::query("UPDATE ops.job_run SET finished_at=clock_timestamp(),status=$2,counts=$3,error_code=$4 WHERE id=$1")
            .bind(self.job).bind(if failed { "failed" } else { "succeeded" }).bind(counts.value()).bind(if failed { Some("ingest_failed") } else { None }).execute(&mut *tx).await?;
        sqlx::query("UPDATE ops.job_lease SET expires_at=clock_timestamp() WHERE lease_key=$1 AND run_id=$2 AND epoch=$3")
            .bind(LEASE_KEY).bind(self.job).bind(self.epoch).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

async fn write_transaction(pool: &PgPool) -> Result<Transaction<'_, Postgres>> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

fn retryable(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<sqlx::Error>()
        .and_then(sqlx::Error::as_database_error)
        .and_then(|error| error.code())
        .is_some_and(|code| matches!(code.as_ref(), "40001" | "40P01" | "55P03"))
}

async fn retry_transaction<T, F, Fut>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut retries = 0;
    loop {
        match operation().await {
            Err(error) if retries < 2 && retryable(&error) => {
                retries += 1;
                tokio::time::sleep(Duration::from_millis(50 * retries)).await;
            }
            result => return result,
        }
    }
}

fn media_type(path: &str) -> &'static str {
    if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".vdata") || path.ends_with(".txt") || path.ends_with(".vdata_inc") {
        "text/plain"
    } else {
        "application/octet-stream"
    }
}
