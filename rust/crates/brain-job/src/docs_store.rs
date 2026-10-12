use crate::{
    docs::SOURCE,
    docs_extract::{self, MAX_BYTES},
    extract::{self, Assertion, Atom},
    git::Blob,
    store::{Counts, Run},
};
use anyhow::{ensure, Context, Result};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::{future::Future, time::Duration};

const LEASE_KEY: &str = "brain-job:ingest-docs";

pub struct DocsStore {
    pub pool: PgPool,
    pub job: i64,
    epoch: i64,
    seconds: i32,
}

impl DocsStore {
    pub async fn start(pool: PgPool, seconds: u32) -> Result<Self> {
        ensure!((30..=900).contains(&seconds), "docs_lease_duration");
        ensure!(
            sqlx::query_scalar::<_, bool>("SELECT to_regclass('docs.current_section') IS NOT NULL")
                .fetch_one(&pool)
                .await?,
            "docs_schema_unavailable"
        );
        let seconds = i32::try_from(seconds).context("docs_lease_duration")?;
        let (job, epoch) = retry(|| async {
            let mut tx = transaction(&pool).await?;
            let job = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-docs',$1,$2) RETURNING id")
                .bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
            let epoch = sqlx::query_scalar("INSERT INTO ops.job_lease(lease_key,run_id,epoch,expires_at) VALUES ($1,$2,1,clock_timestamp()+make_interval(secs=>$3)) ON CONFLICT(lease_key) DO UPDATE SET run_id=EXCLUDED.run_id,epoch=ops.job_lease.epoch+1,expires_at=EXCLUDED.expires_at WHERE ops.job_lease.expires_at<clock_timestamp() RETURNING epoch")
                .bind(LEASE_KEY).bind(job).bind(seconds).fetch_optional(&mut *tx).await?.context("docs_lease_busy")?;
            tx.commit().await?;
            Ok((job, epoch))
        }).await?;
        Ok(Self {
            pool,
            job,
            epoch,
            seconds,
        })
    }

    async fn renew_tx(&self, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        let changed = sqlx::query("UPDATE ops.job_lease SET expires_at=clock_timestamp()+make_interval(secs=>$4) WHERE lease_key=$1 AND run_id=$2 AND epoch=$3 AND expires_at>clock_timestamp()")
            .bind(LEASE_KEY).bind(self.job).bind(self.epoch).bind(self.seconds).execute(&mut **tx).await?.rows_affected();
        ensure!(changed == 1, "docs_lease_lost");
        Ok(())
    }

    pub async fn renew(&self) -> Result<()> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn during<T>(&self, future: impl Future<Output = Result<T>>) -> Result<T> {
        tokio::pin!(future);
        let mut interval =
            tokio::time::interval(Duration::from_secs(u64::try_from(self.seconds / 3)?));
        interval.tick().await;
        loop {
            tokio::select! {
                result = &mut future => return result,
                _ = interval.tick() => self.renew().await?,
            }
        }
    }

    pub async fn source(&self) -> Result<i64> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            sqlx::query("INSERT INTO raw.source(kind,locator) VALUES ('git',$1) ON CONFLICT(locator) DO NOTHING")
                .bind(SOURCE.url()).execute(&mut *tx).await?;
            let id = sqlx::query_scalar("SELECT id FROM raw.source WHERE locator=$1 AND enabled")
                .bind(SOURCE.url()).fetch_one(&mut *tx).await?;
            tx.commit().await?;
            Ok(id)
        }).await
    }

    async fn full(&self, source: i64, tx: &mut Transaction<'_, Postgres>) -> Result<bool> {
        let version = sqlx::query_scalar::<_, String>("SELECT v.extractor_version FROM canon.source_head h JOIN canon.revision v ON v.id=h.published_revision_id WHERE h.source_id=$1")
            .bind(source).fetch_optional(&mut **tx).await?;
        Ok(version.as_deref() != Some(extract::VERSION))
    }

    pub async fn pending(&self, source: i64) -> Result<Option<Run>> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            let row = sqlx::query("SELECT r.id,r.base_revision,r.target_revision,r.extractor,r.extractor_version,v.id AS revision_id FROM raw.run r JOIN canon.revision v ON v.raw_run_id=r.id WHERE r.source_id=$1 AND r.status<>'succeeded' FOR UPDATE OF r")
                .bind(source).fetch_optional(&mut *tx).await?;
            let Some(row) = row else { tx.commit().await?; return Ok(None) };
            ensure!(row.get::<String, _>("extractor") == extract::EXTRACTOR && row.get::<String, _>("extractor_version") == extract::VERSION, "docs_resume_extractor_unavailable");
            let id = row.get("id");
            sqlx::query("UPDATE raw.run SET job_run_id=$2,status='running' WHERE id=$1")
                .bind(id).bind(self.job).execute(&mut *tx).await?;
            let run = Run {
                id, source, revision: row.get("revision_id"), base: row.get("base_revision"),
                target: row.get("target_revision"), full: self.full(source, &mut tx).await?,
            };
            tx.commit().await?;
            Ok(Some(run))
        }).await
    }

    pub async fn begin_run(&self, source: i64, target: &str) -> Result<Option<Run>> {
        ensure!(
            target.len() == 40 && crate::git::valid_oid(target),
            "docs_revision"
        );
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            let base: Option<String> = sqlx::query_scalar("SELECT last_complete_revision FROM raw.source WHERE id=$1 AND locator=$2 AND enabled FOR UPDATE")
                .bind(source).bind(SOURCE.url()).fetch_one(&mut *tx).await?;
            let full = self.full(source, &mut tx).await?;
            if base.as_deref() == Some(target) && !full { tx.commit().await?; return Ok(None) }
            let id = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id")
                .bind(source).bind(self.job).bind(&base).bind(target).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
            let revision = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5) RETURNING id")
                .bind(source).bind(id).bind(target).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut *tx).await?;
            tx.commit().await?;
            Ok(Some(Run { id, source, revision, base, target: target.into(), full }))
        }).await
    }

    pub async fn backfill_run(&self, source: i64) -> Result<Option<Run>> {
        let target: Option<String> = sqlx::query_scalar("SELECT s.last_complete_revision FROM raw.source s JOIN canon.source_head h ON h.source_id=s.id JOIN canon.revision v ON v.id=h.published_revision_id WHERE s.id=$1 AND s.locator=$2 AND s.enabled AND v.extractor_version<>$3")
            .bind(source).bind(SOURCE.url()).bind(extract::VERSION).fetch_optional(&self.pool).await?.flatten();
        match target {
            Some(target) => self.begin_run(source, &target).await,
            None => Ok(None),
        }
    }

    pub async fn completed(&self, run: &Run, path: &str) -> Result<bool> {
        Ok(sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM raw.run_object WHERE run_id=$1 AND path=$2 AND parse_status='succeeded')")
            .bind(run.id).bind(path).fetch_one(&self.pool).await?)
    }

    pub async fn backfill_blob(&self, run: &Run, path: &str) -> Result<Blob> {
        let row = sqlx::query("SELECT o.content_sha256,octet_length(o.body) AS size FROM docs.page p JOIN raw.object o ON o.id=p.raw_object_id JOIN canon.source_head h ON h.source_id=p.source_id WHERE p.source_id=$1 AND p.path=$2 AND p.valid_from_revision_id<=h.published_revision_id AND (p.valid_to_revision_id IS NULL OR p.valid_to_revision_id>h.published_revision_id)")
            .bind(run.source).bind(path).fetch_one(&self.pool).await?;
        ensure!(
            usize::try_from(row.get::<i32, _>("size")).is_ok_and(|size| size <= MAX_BYTES),
            "docs_backfill_size"
        );
        let bytes = sqlx::query_scalar("SELECT o.body FROM docs.page p JOIN raw.object o ON o.id=p.raw_object_id JOIN canon.source_head h ON h.source_id=p.source_id WHERE p.source_id=$1 AND p.path=$2 AND p.valid_from_revision_id<=h.published_revision_id AND (p.valid_to_revision_id IS NULL OR p.valid_to_revision_id>h.published_revision_id)")
            .bind(run.source).bind(path).fetch_one(&self.pool).await?;
        Ok(Blob {
            bytes,
            hash: row.get("content_sha256"),
        })
    }

    pub async fn object(
        &self,
        run: &Run,
        path: &str,
        blob: Option<Blob>,
        counts: &mut Counts,
    ) -> Result<()> {
        ensure!(docs_extract::supported(path), "docs_path");
        let inserted = retry(|| self.object_once(run, path, blob.as_ref())).await?;
        if blob.is_none() {
            counts.paths_deleted += 1;
        } else if inserted {
            counts.raw_inserted += 1;
        } else {
            counts.raw_reused += 1;
        }
        counts.objects_completed += 1;
        Ok(())
    }

    async fn object_once(&self, run: &Run, path: &str, blob: Option<&Blob>) -> Result<bool> {
        let mut tx = transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let mut inserted = false;
        let object = if let Some(blob) = blob {
            ensure!(
                blob.bytes.len() <= MAX_BYTES
                    && Sha256::digest(&blob.bytes).as_slice() == blob.hash,
                "docs_raw_proof"
            );
            let id: Option<i64> = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,$4,'text/html',$5) ON CONFLICT(source_id,external_id,content_sha256) DO NOTHING RETURNING id")
                .bind(run.source).bind(path).bind(&run.target).bind(&blob.hash).bind(&blob.bytes).fetch_optional(&mut *tx).await?;
            inserted = id.is_some();
            Some(match id {
                Some(id) => id,
                None => sqlx::query_scalar("SELECT id FROM raw.object WHERE source_id=$1 AND external_id=$2 AND content_sha256=$3")
                    .bind(run.source).bind(path).bind(&blob.hash).fetch_one(&mut *tx).await?,
            })
        } else {
            None
        };
        sqlx::query(
            "INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES ($1,$2,$3,$4)",
        )
        .bind(run.id)
        .bind(path)
        .bind(object)
        .bind(if object.is_some() { "upsert" } else { "delete" })
        .execute(&mut *tx)
        .await?;
        sqlx::query("UPDATE docs.page SET valid_to_revision_id=$3 WHERE source_id=$1 AND path=$2 AND valid_to_revision_id IS NULL")
            .bind(run.source).bind(path).bind(run.revision).execute(&mut *tx).await?;
        if let Some(object) = object {
            let bytes = stored_bytes(&mut tx, object, blob.context("docs_blob")?).await?;
            let assertions = extract::extract(SOURCE, path, &bytes)?;
            extract::prove(SOURCE, path, &bytes, &assertions)?;
            let first = docs_extract::metadata(assertions.first().context("docs_empty_page")?)?;
            let page: i64 = sqlx::query_scalar("INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id")
                .bind(run.source).bind(path).bind(&first.title).bind(&first.lang).bind(object).bind(run.revision).fetch_one(&mut *tx).await?;
            let aliases: Vec<(String, i64)> = sqlx::query_as("SELECT DISTINCT a.alias_norm,a.entity_id FROM serve.current_alias a JOIN canon.entity e ON e.id=a.entity_id JOIN canon.receipt c ON c.id=e.identity_receipt_id JOIN canon.source_head h ON h.source_id=c.source_id AND h.published_revision_id>=c.revision_id ORDER BY a.alias_norm,a.entity_id LIMIT 10001")
                .fetch_all(&mut *tx).await?;
            ensure!(aliases.len() <= 10000, "docs_alias_limit");
            for (index, assertion) in assertions.iter().enumerate() {
                if index % 100 == 0 {
                    self.renew_tx(&mut tx).await?;
                }
                self.section(&mut tx, run, (path, page, object), assertion, &aliases)
                    .await?;
            }
            let reread = stored_bytes(&mut tx, object, blob.context("docs_blob")?).await?;
            extract::prove(SOURCE, path, &reread, &assertions)?;
        }
        sqlx::query(
            "UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=$1 AND path=$2",
        )
        .bind(run.id)
        .bind(path)
        .execute(&mut *tx)
        .await?;
        sqlx::query("UPDATE raw.run SET last_completed_path=$2 WHERE id=$1")
            .bind(run.id)
            .bind(path)
            .execute(&mut *tx)
            .await?;
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok(inserted)
    }

    async fn section(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        run: &Run,
        page: (&str, i64, i64),
        assertion: &Assertion,
        aliases: &[(String, i64)],
    ) -> Result<()> {
        let (path, page, object) = page;
        let metadata = docs_extract::metadata(assertion)?;
        let Atom::Text(body) = &assertion.value else {
            anyhow::bail!("docs_body_type")
        };
        let entities = docs_extract::linked_entities(body, aliases)?;
        let url = docs_extract::section_url(path, &run.target, &metadata)?;
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id")
            .bind(object).bind(run.source).bind(run.revision).bind(assertion.locator_kind).bind(&assertion.locator).bind(&assertion.hash).bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut **tx).await?;
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,heading_path,body,entity_ids,url,receipt_id) VALUES ($1,$2,$3,$4,$5,$6,$7)")
            .bind(page).bind(metadata.ordinal).bind(&metadata.heading_path).bind(body).bind(entities).bind(url).bind(receipt).execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn publish(&self, run: &Run) -> Result<()> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
                .bind(run.source).bind(&run.target).execute(&mut *tx).await?;
            sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
                .bind(run.id).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2) ON CONFLICT(source_id) DO UPDATE SET published_revision_id=EXCLUDED.published_revision_id")
                .bind(run.source).bind(run.revision).execute(&mut *tx).await?;
            self.renew_tx(&mut tx).await?;
            tx.commit().await?;
            Ok(())
        }).await
    }

    pub async fn finish(&self, counts: &Counts, failed: bool) -> Result<()> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            if failed {
                sqlx::query("UPDATE raw.run SET status='partial' WHERE job_run_id=$1 AND status='running'")
                    .bind(self.job).execute(&mut *tx).await?;
            }
            sqlx::query("UPDATE ops.job_run SET finished_at=clock_timestamp(),status=$2,counts=$3,error_code=$4 WHERE id=$1")
                .bind(self.job).bind(if failed { "failed" } else { "succeeded" }).bind(counts.value()).bind(if failed { Some("docs_ingest_failed") } else { None }).execute(&mut *tx).await?;
            sqlx::query("UPDATE ops.job_lease SET expires_at=clock_timestamp() WHERE lease_key=$1 AND run_id=$2 AND epoch=$3")
                .bind(LEASE_KEY).bind(self.job).bind(self.epoch).execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(())
        }).await
    }
}

async fn stored_bytes(
    tx: &mut Transaction<'_, Postgres>,
    object: i64,
    blob: &Blob,
) -> Result<Vec<u8>> {
    let size: i32 = sqlx::query_scalar("SELECT octet_length(body) FROM raw.object WHERE id=$1")
        .bind(object)
        .fetch_one(&mut **tx)
        .await?;
    ensure!(
        usize::try_from(size).ok() == Some(blob.bytes.len()) && blob.bytes.len() <= MAX_BYTES,
        "docs_stored_size"
    );
    let bytes: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=$1")
        .bind(object)
        .fetch_one(&mut **tx)
        .await?;
    ensure!(
        Sha256::digest(&bytes).as_slice() == blob.hash,
        "docs_stored_hash"
    );
    Ok(bytes)
}

async fn transaction(pool: &PgPool) -> Result<Transaction<'_, Postgres>> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

async fn retry<T, F, Fut>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut retries = 0;
    loop {
        match operation().await {
            Err(error)
                if retries < 2
                    && error
                        .downcast_ref::<sqlx::Error>()
                        .and_then(sqlx::Error::as_database_error)
                        .and_then(|error| error.code())
                        .is_some_and(|code| {
                            matches!(code.as_ref(), "40001" | "40P01" | "55P03")
                        }) =>
            {
                retries += 1;
                tokio::time::sleep(Duration::from_millis(50 * retries)).await;
            }
            result => return result,
        }
    }
}
