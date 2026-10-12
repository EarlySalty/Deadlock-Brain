use super::{
    extraction::{self, ItemEdge},
    EXTRACTOR, VERSION,
};
use crate::{
    extract::{Assertion, Atom},
    store::Counts,
};
use anyhow::{ensure, Context, Result};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Transaction};
use std::{future::Future, time::Duration};

const LEASE: &str = "brain-job:ingest-assets";

pub struct Store {
    pub pool: PgPool,
    pub job: i64,
    epoch: i64,
    seconds: i32,
}

struct Run {
    source: i64,
    revision: i64,
    path: String,
    object: i64,
}

impl Store {
    pub async fn start(pool: PgPool, seconds: u32) -> Result<Self> {
        ensure!((30..=900).contains(&seconds), "assets_lease_duration");
        let seconds = i32::try_from(seconds)?;
        let (job, epoch) = retry(|| async {
            let mut tx = transaction(&pool).await?;
            let job = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-assets',$1,$2) RETURNING id")
                .bind(EXTRACTOR).bind(VERSION).fetch_one(&mut *tx).await?;
            let epoch = sqlx::query_scalar("INSERT INTO ops.job_lease(lease_key,run_id,epoch,expires_at) VALUES ($1,$2,1,clock_timestamp()+make_interval(secs=>$3)) ON CONFLICT(lease_key) DO UPDATE SET run_id=EXCLUDED.run_id,epoch=ops.job_lease.epoch+1,expires_at=EXCLUDED.expires_at WHERE ops.job_lease.expires_at<clock_timestamp() RETURNING epoch")
                .bind(LEASE).bind(job).bind(seconds).fetch_optional(&mut *tx).await?.context("assets_lease_busy")?;
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
            .bind(LEASE).bind(self.job).bind(self.epoch).bind(self.seconds).execute(&mut **tx).await?.rows_affected();
        ensure!(changed == 1, "assets_lease_lost");
        Ok(())
    }

    pub async fn during<T>(&self, future: impl Future<Output = Result<T>>) -> Result<T> {
        tokio::pin!(future);
        let mut interval =
            tokio::time::interval(Duration::from_secs(u64::try_from(self.seconds / 3)?));
        interval.tick().await;
        loop {
            tokio::select! {
                result = &mut future => return result,
                _ = interval.tick() => {
                    retry(|| async {
                        let mut tx = transaction(&self.pool).await?;
                        self.renew_tx(&mut tx).await?;
                        tx.commit().await?;
                        Ok(())
                    }).await?;
                }
            }
        }
    }

    pub async fn replay(
        &self,
        locator: &str,
        language: &'static str,
        counts: &mut Counts,
    ) -> Result<u64> {
        let result = retry(|| self.object_once(locator, language, None)).await?;
        Ok(completed(counts, result))
    }

    pub async fn object(
        &self,
        locator: &str,
        language: &'static str,
        bytes: &[u8],
        counts: &mut Counts,
    ) -> Result<u64> {
        let result = retry(|| self.object_once(locator, language, Some(bytes))).await?;
        if result.is_none() {
            counts.sources_unchanged += 1;
        }
        Ok(completed(counts, result))
    }

    async fn object_once(
        &self,
        locator: &str,
        language: &'static str,
        bytes: Option<&[u8]>,
    ) -> Result<Option<(bool, u64)>> {
        ensure!(
            matches!(language, "en" | "de")
                && locator.len() <= 2048
                && bytes.is_none_or(|bytes| bytes.len() <= super::MAX_BYTES),
            "assets_object_input"
        );
        let mut tx = transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        if bytes.is_some() {
            sqlx::query("INSERT INTO raw.source(kind,locator) VALUES ('assets_api',$1) ON CONFLICT(locator) DO NOTHING").bind(locator).execute(&mut *tx).await?;
        }
        let source: Option<(i64, Option<String>)> = sqlx::query_as("SELECT id,last_complete_revision FROM raw.source WHERE locator=$1 AND kind='assets_api' AND enabled FOR UPDATE")
            .bind(locator).fetch_optional(&mut *tx).await?;
        let Some((source, base)) = source else {
            ensure!(bytes.is_none(), "assets_source_unavailable");
            tx.commit().await?;
            return Ok(None);
        };
        let head: Option<(String, String, i64)> = sqlx::query_as("SELECT v.extractor_version,v.source_revision,v.raw_run_id FROM canon.source_head h JOIN canon.revision v ON v.id=h.published_revision_id WHERE h.source_id=$1 AND v.extractor=$2")
            .bind(source).bind(EXTRACTOR).fetch_optional(&mut *tx).await?;
        let path = format!("items/{language}.json");
        let replay: Option<(i64, Vec<u8>, Vec<u8>)> = if bytes.is_none() {
            let Some((version, revision, raw_run)) = &head else {
                tx.commit().await?;
                return Ok(None);
            };
            if version == VERSION {
                tx.commit().await?;
                return Ok(None);
            }
            ensure!(base.as_ref() == Some(revision), "assets_replay_head");
            Some(sqlx::query_as("SELECT o.id,o.body,o.content_sha256 FROM raw.run r JOIN raw.run_object ro ON ro.run_id=r.id JOIN raw.object o ON o.id=ro.object_id WHERE r.id=$1 AND r.source_id=$2 AND r.status='succeeded' AND ro.path=$3 AND ro.action='upsert' AND ro.parse_status='succeeded' AND o.source_id=$2 AND o.external_id=$3 AND o.media_type='application/json'")
                .bind(raw_run).bind(source).bind(&path).fetch_one(&mut *tx).await?)
        } else {
            None
        };
        let bytes = bytes
            .or_else(|| replay.as_ref().map(|(_, body, _)| body.as_slice()))
            .context("assets_replay_bytes")?;
        ensure!(bytes.len() <= super::MAX_BYTES, "assets_payload_limit");
        let hash = Sha256::digest(bytes).to_vec();
        let target = hex::encode(&hash);
        if let Some((_, _, stored_hash)) = &replay {
            ensure!(
                stored_hash == &hash && base.as_ref() == Some(&target),
                "assets_replay_hash"
            );
        }
        if base.as_ref() == Some(&target)
            && head
                .as_ref()
                .is_some_and(|(version, _, _)| version == VERSION)
        {
            tx.commit().await?;
            return Ok(None);
        }
        let raw_run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id")
            .bind(source).bind(self.job).bind(base).bind(&target).bind(EXTRACTOR).bind(VERSION).fetch_one(&mut *tx).await?;
        let revision = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5) RETURNING id")
            .bind(source).bind(raw_run).bind(&target).bind(EXTRACTOR).bind(VERSION).fetch_one(&mut *tx).await?;
        let inserted: Option<i64> = if replay.is_some() {
            None
        } else {
            sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,$4,'application/json',$5) ON CONFLICT(source_id,external_id,content_sha256) DO NOTHING RETURNING id")
                .bind(source).bind(&path).bind(&target).bind(&hash).bind(bytes).fetch_optional(&mut *tx).await?
        };
        let object = match (replay.as_ref(), inserted) {
            (Some((id, _, _)), _) => *id,
            (None, Some(id)) => id,
            (None, None) => sqlx::query_scalar("SELECT id FROM raw.object WHERE source_id=$1 AND external_id=$2 AND content_sha256=$3")
                .bind(source).bind(&path).bind(&hash).fetch_one(&mut *tx).await?,
        };
        sqlx::query(
            "INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES ($1,$2,$3,'upsert')",
        )
        .bind(raw_run)
        .bind(&path)
        .bind(object)
        .execute(&mut *tx)
        .await?;
        let stored: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=$1")
            .bind(object)
            .fetch_one(&mut *tx)
            .await?;
        ensure!(
            stored == bytes && Sha256::digest(&stored).as_slice() == hash.as_slice(),
            "assets_stored_bytes"
        );
        let extracted = extraction::extract(&stored, language)?;
        extraction::prove(&stored, &extracted)?;
        let run = Run {
            source,
            revision,
            path,
            object,
        };
        for table in ["fact", "alias", "relation"] {
            sqlx::query(&format!("UPDATE canon.{table} a SET valid_to_revision_id=$3 FROM canon.receipt r WHERE a.source_id=$1 AND a.source_path=$2 AND a.valid_to_revision_id IS NULL AND r.id=a.receipt_id AND r.extractor=$4"))
                .bind(source).bind(&run.path).bind(revision).bind(EXTRACTOR).execute(&mut *tx).await?;
        }
        for (position, fact) in extracted.facts.iter().enumerate() {
            if position % 100 == 0 {
                self.renew_tx(&mut tx).await?;
            }
            write_fact(&mut tx, &run, fact).await?;
        }
        for edge in &extracted.edges {
            write_edge(&mut tx, &run, edge).await?;
        }
        let verified: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=$1")
            .bind(object)
            .fetch_one(&mut *tx)
            .await?;
        ensure!(verified == stored, "assets_commit_bytes");
        extraction::prove(&verified, &extracted)?;
        sqlx::query(
            "UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=$1 AND path=$2",
        )
        .bind(raw_run)
        .bind(&run.path)
        .execute(&mut *tx)
        .await?;
        sqlx::query("UPDATE raw.run SET status='succeeded',last_completed_path=$2 WHERE id=$1")
            .bind(raw_run)
            .bind(&run.path)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
            .bind(source)
            .bind(&target)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2) ON CONFLICT(source_id) DO UPDATE SET published_revision_id=EXCLUDED.published_revision_id")
            .bind(source).bind(revision).execute(&mut *tx).await?;
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok(Some((inserted.is_some(), extracted.unmapped)))
    }

    pub async fn finish(&self, counts: &Counts, unmapped: u64, failed: bool) -> Result<()> {
        retry(|| async {
            let mut tx = transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            let mut value = counts.value();
            value["unmapped_values"] = unmapped.into();
            sqlx::query("UPDATE ops.job_run SET finished_at=clock_timestamp(),status=$2,counts=$3,error_code=$4 WHERE id=$1")
                .bind(self.job).bind(if failed { "failed" } else { "succeeded" }).bind(value).bind(if failed { Some("assets_ingest_failed") } else { None }).execute(&mut *tx).await?;
            sqlx::query("UPDATE ops.job_lease SET expires_at=clock_timestamp() WHERE lease_key=$1 AND run_id=$2 AND epoch=$3")
                .bind(LEASE).bind(self.job).bind(self.epoch).execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(())
        }).await
    }
}

fn completed(counts: &mut Counts, result: Option<(bool, u64)>) -> u64 {
    let Some((inserted, unmapped)) = result else {
        return 0;
    };
    if inserted {
        counts.raw_inserted += 1;
    } else {
        counts.raw_reused += 1;
    }
    counts.objects_completed += 1;
    counts.sources_completed += 1;
    unmapped
}

async fn receipt(tx: &mut Transaction<'_, Postgres>, run: &Run, proof: &Assertion) -> Result<i64> {
    Ok(sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id")
        .bind(run.object).bind(run.source).bind(run.revision).bind(proof.locator_kind).bind(&proof.locator).bind(&proof.hash).bind(EXTRACTOR).bind(VERSION).fetch_one(&mut **tx).await?)
}

async fn entity(
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

async fn write_fact(tx: &mut Transaction<'_, Postgres>, run: &Run, fact: &Assertion) -> Result<()> {
    let proof = receipt(tx, run, fact).await?;
    let id = entity(tx, fact.kind, &fact.entity, proof).await?;
    if let Some((name, language)) = &fact.alias {
        let alias = name
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if !alias.is_empty() && alias.len() <= 256 {
            sqlx::query("INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(source_id,source_path,entity_id,alias_norm,lang) WHERE valid_to_revision_id IS NULL DO NOTHING")
                .bind(id).bind(alias).bind(language).bind(proof).bind(run.source).bind(&run.path).bind(run.revision).execute(&mut **tx).await?;
        }
    }
    let (number, text, value_type) = match &fact.value {
        Atom::Number(value) => (Some(value.as_str()), None, "number"),
        Atom::Text(value) => (None, Some(value.as_str()), "text"),
    };
    sqlx::query("INSERT INTO canon.stat(key,section,unit,value_type,display_order) VALUES ($1,$2,$3,$4,$5) ON CONFLICT(key) DO NOTHING")
        .bind(&fact.stat).bind(fact.section).bind(fact.unit).bind(value_type).bind(fact.order).execute(&mut **tx).await?;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,value_text,receipt_id,valid_from_revision_id) VALUES ($1,$2,$3,$4,$5,$6::text::numeric,$7,$8,$9)")
        .bind(id).bind(&fact.stat).bind(run.source).bind(&run.path).bind(&fact.condition).bind(number).bind(text).bind(proof).bind(run.revision).execute(&mut **tx).await?;
    Ok(())
}

async fn write_edge(tx: &mut Transaction<'_, Postgres>, run: &Run, edge: &ItemEdge) -> Result<()> {
    let proof = receipt(tx, run, &edge.proof).await?;
    let from = entity(tx, "item", &edge.from, proof).await?;
    let to = entity(tx, "item", &edge.to, proof).await?;
    sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,$2,$3,NULL,$4,$5,$6,$7)")
        .bind(from).bind(edge.kind).bind(to).bind(proof).bind(run.source).bind(&run.path).bind(run.revision).execute(&mut **tx).await?;
    Ok(())
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
