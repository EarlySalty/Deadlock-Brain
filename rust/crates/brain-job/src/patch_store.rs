use crate::{
    extract::{self, Assertion, Atom},
    patch_extract::{self, AliasBinding, AliasIndex, KnownEntity, StatBinding},
    patchnotes::{Post, MAX_ENVELOPE_BYTES, SOURCE_LOCATOR},
};
use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use serde_json::{json, value::RawValue, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::{future::Future, time::Duration};

const LEASE_KEY: &str = "brain-job:ingest-patchnotes";

#[derive(Default)]
pub struct Counts {
    pub raw_inserted: u64,
    pub raw_reused: u64,
    pub objects_completed: u64,
    pub objects_skipped: u64,
    pub objects_oversized: u64,
    pub metadata_missing: u64,
    pub changes_inserted: u64,
    pub changes_unresolved: u64,
    pub sources_completed: u64,
}

impl Counts {
    pub fn value(&self) -> Value {
        json!({
            "raw_inserted": self.raw_inserted,
            "raw_reused": self.raw_reused,
            "objects_completed": self.objects_completed,
            "objects_skipped": self.objects_skipped,
            "objects_oversized": self.objects_oversized,
            "metadata_missing": self.metadata_missing,
            "changes_inserted": self.changes_inserted,
            "changes_unresolved": self.changes_unresolved,
            "sources_completed": self.sources_completed,
        })
    }

    pub fn partial(&self) -> bool {
        self.objects_skipped > 0
            || self.objects_oversized > 0
            || self.metadata_missing > 0
            || self.changes_unresolved > 0
    }
}

pub struct PatchStore {
    pub pool: PgPool,
    pub job: i64,
    pub source: i64,
    epoch: i64,
    seconds: i32,
}

impl PatchStore {
    pub async fn start(pool: PgPool, seconds: u32) -> Result<Self> {
        ensure!(
            (30..=900).contains(&seconds),
            "Ungültige Patchimport-Leasedauer."
        );
        let ready: bool = sqlx::query_scalar(
            "SELECT current_user='brain_ingest' AND session_user='brain_ingest'
                AND to_regprocedure('canon.stage_patch_origin(bigint)') IS NOT NULL
                AND to_regprocedure('canon.publish_patch_origin(bigint)') IS NOT NULL
                AND to_regclass('serve.published_patch_change') IS NOT NULL",
        )
        .fetch_one(&pool)
        .await?;
        ensure!(
            ready,
            "Patchimport benötigt das veröffentlichte Patchschema."
        );
        let seconds = i32::try_from(seconds).context("Ungültige Patchimport-Leasedauer.")?;
        let (job, epoch, source) = retry_transaction(|| Self::start_once(&pool, seconds)).await?;
        Ok(Self {
            pool,
            job,
            source,
            epoch,
            seconds,
        })
    }

    async fn start_once(pool: &PgPool, seconds: i32) -> Result<(i64, i64, i64)> {
        let mut tx = write_transaction(pool).await?;
        let job = sqlx::query_scalar(
            "INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-patchnotes',$1,$2) RETURNING id",
        )
        .bind(extract::EXTRACTOR)
        .bind(extract::VERSION)
        .fetch_one(&mut *tx)
        .await?;
        let epoch = sqlx::query_scalar(
            "INSERT INTO ops.job_lease(lease_key,run_id,epoch,expires_at)
            VALUES ($1,$2,1,clock_timestamp()+make_interval(secs=>$3))
            ON CONFLICT(lease_key) DO UPDATE SET run_id=EXCLUDED.run_id,
                epoch=ops.job_lease.epoch+1,expires_at=EXCLUDED.expires_at
            WHERE ops.job_lease.expires_at<clock_timestamp() RETURNING epoch",
        )
        .bind(LEASE_KEY)
        .bind(job)
        .bind(seconds)
        .fetch_optional(&mut *tx)
        .await?
        .context("Ein anderer Patchimport hält die Lease.")?;
        sqlx::query(
            "INSERT INTO raw.source(kind,locator) VALUES ('patchnotes',$1) ON CONFLICT(locator) DO NOTHING",
        )
        .bind(SOURCE_LOCATOR)
        .execute(&mut *tx)
        .await?;
        let source = sqlx::query_scalar(
            "SELECT id FROM raw.source WHERE locator=$1 AND kind='patchnotes' AND enabled",
        )
        .bind(SOURCE_LOCATOR)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok((job, epoch, source))
    }

    async fn renew_tx(&self, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        let changed = sqlx::query(
            "UPDATE ops.job_lease SET expires_at=clock_timestamp()+make_interval(secs=>$4)
            WHERE lease_key=$1 AND run_id=$2 AND epoch=$3 AND expires_at>clock_timestamp()",
        )
        .bind(LEASE_KEY)
        .bind(self.job)
        .bind(self.epoch)
        .bind(self.seconds)
        .execute(&mut **tx)
        .await?
        .rows_affected();
        ensure!(changed == 1, "Patchimport-Lease ist nicht mehr gültig.");
        Ok(())
    }

    pub async fn renew(&self) -> Result<()> {
        retry_transaction(|| async {
            let mut tx = write_transaction(&self.pool).await?;
            self.renew_tx(&mut tx).await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn during<T>(&self, future: impl Future<Output = Result<T>>) -> Result<T> {
        tokio::pin!(future);
        let mut interval = tokio::time::interval(Duration::from_secs(
            u64::try_from(self.seconds / 3).context("Ungültige Patchimport-Leasedauer.")?,
        ));
        interval.tick().await;
        loop {
            tokio::select! {
                result = &mut future => return result,
                _ = interval.tick() => self.renew().await?,
            }
        }
    }

    pub async fn object(&self, post: &Post, counts: &mut Counts) -> Result<()> {
        ensure!(post.id > 0, "Patchbeitrag besitzt eine ungültige Kennung.");
        let outcome = retry_transaction(|| self.object_once(post)).await?;
        counts.raw_inserted += u64::from(outcome.inserted);
        counts.raw_reused += u64::from(outcome.reused);
        counts.objects_completed += 1;
        counts.objects_skipped += u64::from(outcome.skipped);
        counts.objects_oversized += u64::from(post.bytes.is_none());
        counts.metadata_missing += u64::from(outcome.metadata_missing);
        counts.changes_inserted += outcome.changes;
        counts.changes_unresolved += outcome.unresolved;
        Ok(())
    }

    async fn object_once(&self, post: &Post) -> Result<Outcome> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let path = post.id.to_string();
        let catalog = Self::catalog(&mut tx).await?;
        let hash = post
            .bytes
            .as_ref()
            .map(|bytes| Sha256::digest(bytes).to_vec());
        let target = hash.as_ref().map_or_else(
            || format!("post:{}:oversized:job:{}", post.id, self.job),
            |hash| {
                format!(
                    "post:{}:sha256:{}:catalog:{}",
                    post.id,
                    hex::encode(hash),
                    catalog.fingerprint
                )
            },
        );
        if let Some(hash) = &hash {
            ensure!(
                post.bytes
                    .as_ref()
                    .is_some_and(|bytes| bytes.len() <= MAX_ENVELOPE_BYTES),
                "Patchrohobjekt überschreitet die Größenbegrenzung."
            );
            let identity: PostIdentity =
                serde_json::from_slice(post.bytes.as_deref().context("Patchrohobjekt fehlt.")?)
                    .map_err(|_| anyhow::anyhow!("Patchrohkennung konnte nicht gelesen werden."))?;
            ensure!(
                identity.id == post.id,
                "Patchrohobjekt gehört zu einem anderen Beitrag."
            );
            let previous = sqlx::query(
                "SELECT o.id,o.content_sha256,r.extractor,r.extractor_version,v.source_revision,ro.parse_status
                FROM raw.run r JOIN raw.run_object ro ON ro.run_id=r.id
                JOIN canon.revision v ON v.raw_run_id=r.id
                JOIN canon.source_head h ON h.source_id=r.source_id AND h.published_revision_id>=v.id
                LEFT JOIN raw.object o ON o.id=ro.object_id
                WHERE r.source_id=$1 AND ro.path=$2 AND r.status='succeeded'
                ORDER BY r.id DESC LIMIT 1",
            )
            .bind(self.source)
            .bind(&path)
            .fetch_optional(&mut *tx)
            .await?;
            if let Some(previous) = previous {
                if previous
                    .get::<Option<Vec<u8>>, _>("content_sha256")
                    .as_ref()
                    == Some(hash)
                    && previous.get::<String, _>("extractor") == extract::EXTRACTOR
                    && previous.get::<String, _>("extractor_version") == extract::VERSION
                    && previous.get::<String, _>("source_revision") == target
                {
                    let object: i64 = previous.get("id");
                    let input = post.bytes.as_deref().context("Patchrohobjekt fehlt.")?;
                    Self::stored_bytes(&mut tx, object, input, hash).await?;
                    let skipped = previous.get::<String, _>("parse_status") == "skipped";
                    tx.commit().await?;
                    return Ok(Outcome {
                        reused: true,
                        skipped,
                        ..Outcome::default()
                    });
                }
            }
        }
        let base: Option<String> = sqlx::query_scalar(
            "SELECT last_complete_revision FROM raw.source WHERE id=$1 AND enabled FOR UPDATE",
        )
        .bind(self.source)
        .fetch_one(&mut *tx)
        .await?;
        let run: i64 = sqlx::query_scalar(
            "INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version)
            VALUES ($1,$2,$3,$4,$5,$6) RETURNING id",
        )
        .bind(self.source)
        .bind(self.job)
        .bind(base)
        .bind(&target)
        .bind(extract::EXTRACTOR)
        .bind(extract::VERSION)
        .fetch_one(&mut *tx)
        .await?;
        let revision: i64 = sqlx::query_scalar(
            "INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
            VALUES ($1,$2,$3,$4,$5) RETURNING id",
        )
        .bind(self.source)
        .bind(run)
        .bind(&target)
        .bind(extract::EXTRACTOR)
        .bind(extract::VERSION)
        .fetch_one(&mut *tx)
        .await?;
        let mut outcome = Outcome::default();
        let object = if let (Some(bytes), Some(hash)) = (&post.bytes, &hash) {
            let inserted: Option<i64> = sqlx::query_scalar(
                "INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
                VALUES ($1,$2,$3,$4,'application/json',$5)
                ON CONFLICT(source_id,external_id,content_sha256) DO NOTHING RETURNING id",
            )
            .bind(self.source)
            .bind(&path)
            .bind(&target)
            .bind(hash)
            .bind(bytes)
            .fetch_optional(&mut *tx)
            .await?;
            outcome.inserted = inserted.is_some();
            outcome.reused = inserted.is_none();
            Some(match inserted {
                Some(id) => id,
                None => sqlx::query_scalar(
                    "SELECT id FROM raw.object WHERE source_id=$1 AND external_id=$2 AND content_sha256=$3",
                )
                .bind(self.source)
                .bind(&path)
                .bind(hash)
                .fetch_one(&mut *tx)
                .await?,
            })
        } else {
            outcome.skipped = true;
            None
        };
        sqlx::query(
            "INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES ($1,$2,$3,$4)",
        )
        .bind(run)
        .bind(&path)
        .bind(object)
        .bind(if object.is_some() { "upsert" } else { "delete" })
        .execute(&mut *tx)
        .await?;
        let mut origin_receipt = None;
        if let (Some(object), Some(hash), Some(input)) = (object, &hash, &post.bytes) {
            let stored = Self::stored_bytes(&mut tx, object, input, hash).await?;
            match patch_extract::parse_post(&stored) {
                Ok(parsed) => {
                    ensure!(
                        parsed.id() == post.id,
                        "Patchrohobjekt gehört zu einem anderen Beitrag."
                    );
                    let extracted = patch_extract::extract(&parsed, &catalog.index)?;
                    outcome.metadata_missing = extracted
                        .skipped
                        .iter()
                        .any(|skip| skip.statement_index.is_none());
                    outcome.unresolved = u64::try_from(
                        extracted
                            .skipped
                            .iter()
                            .filter(|skip| {
                                skip.statement_index.is_some() && skip.reason != "not_a_change"
                            })
                            .count(),
                    )
                    .context("Zu viele übersprungene Patchänderungen.")?;
                    outcome.skipped = outcome.metadata_missing || outcome.unresolved > 0;
                    let assertions: Vec<_> = extracted
                        .changes
                        .into_iter()
                        .map(|change| change.assertion)
                        .collect();
                    patch_extract::prove(&parsed, &catalog.index, &assertions)?;
                    if !outcome.metadata_missing && parsed.title().len() <= 256 {
                        let title: TitleProof<'_> =
                            serde_json::from_slice(&stored).map_err(|_| {
                                anyhow::anyhow!("Patchtitelbeleg konnte nicht gelesen werden.")
                            })?;
                        let receipt = self
                            .receipt(
                                &mut tx,
                                object,
                                revision,
                                "/title",
                                &Sha256::digest(title.title.get().as_bytes()),
                            )
                            .await?;
                        let patch: i64 = sqlx::query_scalar("SELECT canon.stage_patch_origin($1)")
                            .bind(receipt)
                            .fetch_one(&mut *tx)
                            .await?;
                        origin_receipt = Some(receipt);
                        for (index, assertion) in assertions.iter().enumerate() {
                            if index % 100 == 0 {
                                self.renew_tx(&mut tx).await?;
                            }
                            self.change(&mut tx, object, revision, patch, assertion)
                                .await?;
                            outcome.changes += 1;
                        }
                    } else {
                        outcome.skipped = true;
                        outcome.metadata_missing = true;
                    }
                }
                Err(_) => {
                    outcome.skipped = true;
                    outcome.metadata_missing = true;
                }
            }
        }
        sqlx::query("UPDATE raw.run_object SET parse_status=$3 WHERE run_id=$1 AND path=$2")
            .bind(run)
            .bind(&path)
            .bind(if outcome.skipped {
                "skipped"
            } else {
                "succeeded"
            })
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE raw.run SET last_completed_path=$2,status='succeeded' WHERE id=$1")
            .bind(run)
            .bind(&path)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
            .bind(self.source)
            .bind(&target)
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2)
            ON CONFLICT(source_id) DO UPDATE SET published_revision_id=EXCLUDED.published_revision_id",
        )
        .bind(self.source).bind(revision).execute(&mut *tx).await?;
        if let Some(receipt) = origin_receipt {
            sqlx::query("SELECT canon.publish_patch_origin($1)")
                .bind(receipt)
                .execute(&mut *tx)
                .await?;
        }
        self.renew_tx(&mut tx).await?;
        tx.commit().await?;
        Ok(outcome)
    }

    async fn stored_bytes(
        tx: &mut Transaction<'_, Postgres>,
        object: i64,
        input: &[u8],
        hash: &[u8],
    ) -> Result<Vec<u8>> {
        let size: i32 = sqlx::query_scalar("SELECT octet_length(body) FROM raw.object WHERE id=$1")
            .bind(object)
            .fetch_one(&mut **tx)
            .await?;
        ensure!(
            usize::try_from(size).ok() == Some(input.len()),
            "Gespeicherte Patchrohgröße stimmt nicht überein."
        );
        let stored: Vec<u8> = sqlx::query_scalar("SELECT body FROM raw.object WHERE id=$1")
            .bind(object)
            .fetch_one(&mut **tx)
            .await?;
        ensure!(
            stored == input && Sha256::digest(&stored).as_slice() == hash,
            "Gespeicherter Patchrohbeleg stimmt nicht überein."
        );
        Ok(stored)
    }

    async fn catalog(tx: &mut Transaction<'_, Postgres>) -> Result<Catalog> {
        let mut rows: Vec<CatalogRow> = sqlx::query_as(
            "WITH known AS (
                SELECT e.id,e.key,e.kind FROM canon.entity e
                JOIN canon.receipt proof ON proof.id=e.identity_receipt_id
                JOIN canon.source_head head ON head.source_id=proof.source_id AND head.published_revision_id>=proof.revision_id
                WHERE e.kind IN ('hero','ability','item','timer')
            ), names AS (
                SELECT e.id,e.key AS alias FROM known e
                UNION SELECT a.entity_id,a.alias_norm FROM serve.current_alias a JOIN known e ON e.id=a.entity_id
                UNION SELECT f.entity_id,f.value_text FROM serve.current_fact f JOIN known e ON e.id=f.entity_id
                    WHERE f.stat_key='identity.name' AND f.value_text IS NOT NULL
            ), aliases AS (
                SELECT DISTINCT names.alias,e.key,e.kind,owner.key AS scope
                FROM names JOIN known e ON e.id=names.id
                LEFT JOIN serve.current_relation relation ON relation.to_entity_id=e.id AND e.kind='ability' AND relation.kind='has_ability'
                LEFT JOIN known owner ON owner.id=relation.from_entity_id
                WHERE octet_length(names.alias) BETWEEN 1 AND 128 AND names.alias !~ '[[:cntrl:]]'
                ORDER BY names.alias,e.key,e.kind,owner.key LIMIT 10001
            ), stats AS (
                SELECT DISTINCT e.key,label.value AS label,s.key AS stat FROM serve.current_fact f
                JOIN known e ON e.id=f.entity_id JOIN canon.stat s ON s.key=f.stat_key
                CROSS JOIN LATERAL (VALUES(s.label_en),(s.label_de),(s.key)) label(value)
                WHERE label.value IS NOT NULL AND octet_length(label.value) BETWEEN 1 AND 128
                    AND label.value !~ '[[:cntrl:]]'
                ORDER BY e.key,label.value,s.key LIMIT 10001
            ) SELECT 'alias',alias,key,kind,scope FROM aliases
            UNION ALL SELECT 'stat',label,key,stat,NULL FROM stats",
        )
        .fetch_all(&mut **tx)
        .await?;
        ensure!(
            rows.iter().filter(|row| row.0 == "alias").count() <= 10000,
            "Veröffentlichter Patchentitätskatalog überschreitet die Begrenzung."
        );
        ensure!(
            rows.iter().filter(|row| row.0 == "stat").count() <= 10000,
            "Veröffentlichter Patchstatistikkatalog überschreitet die Begrenzung."
        );
        rows.sort_unstable();
        let bytes = serde_json::to_vec(&("patch-catalog-v1", &rows))
            .context("Patchkatalog konnte nicht eindeutig kodiert werden.")?;
        let fingerprint = hex::encode(Sha256::digest(&bytes));
        let mut aliases = Vec::new();
        let mut stats = Vec::new();
        for (entry, label, key, kind_or_stat, scope) in rows {
            match entry.as_str() {
                "alias" => {
                    let kind = match kind_or_stat.as_str() {
                        "hero" => "hero",
                        "ability" => "ability",
                        "item" => "item",
                        "timer" => "timer",
                        _ => anyhow::bail!("Patchkatalog enthält eine ungültige Entitätsart."),
                    };
                    aliases.push(AliasBinding {
                        alias: label,
                        entity: KnownEntity { key, kind },
                        scope,
                    });
                }
                "stat" => stats.push(StatBinding {
                    entity: key,
                    label,
                    stat: kind_or_stat,
                }),
                _ => anyhow::bail!("Patchkatalog enthält eine ungültige Belegart."),
            }
        }
        Ok(Catalog {
            index: AliasIndex::new(aliases, stats)?,
            fingerprint,
        })
    }

    async fn receipt(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        object: i64,
        revision: i64,
        locator: &str,
        hash: &[u8],
    ) -> Result<i64> {
        Ok(sqlx::query_scalar(
            "INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES ($1,$2,$3,'json_pointer',$4,$5,$6,$7) RETURNING id",
        )
        .bind(object).bind(self.source).bind(revision).bind(locator).bind(hash)
        .bind(extract::EXTRACTOR).bind(extract::VERSION).fetch_one(&mut **tx).await?)
    }

    async fn change(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        object: i64,
        revision: i64,
        patch: i64,
        assertion: &Assertion,
    ) -> Result<()> {
        let entity: i64 =
            sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1 AND kind=$2")
                .bind(&assertion.entity)
                .bind(assertion.kind)
                .fetch_one(&mut **tx)
                .await?;
        let Atom::Text(line) = &assertion.value else {
            anyhow::bail!("Patchänderung benötigt einen unveränderten Textbeleg.");
        };
        ensure!(
            assertion.locator_kind == "json_pointer" && assertion.locator == "/raw_content",
            "Patchänderung benötigt den unveränderten Rohtextbeleg."
        );
        let receipt = self
            .receipt(tx, object, revision, &assertion.locator, &assertion.hash)
            .await?;
        let old = assertion
            .condition
            .get("old_value")
            .filter(|value| !value.is_null());
        let new = assertion
            .condition
            .get("new_value")
            .filter(|value| !value.is_null());
        let stat = assertion.condition.get("stat").and_then(Value::as_str);
        sqlx::query(
            "INSERT INTO canon.patch_change(patch_id,entity_id,stat_key,\"old\",\"new\",change_kind,line_text,receipt_id)
            VALUES ($1,$2,$3,$4,$5,'statement',$6,$7)",
        )
        .bind(patch).bind(entity).bind(stat).bind(old).bind(new).bind(line).bind(receipt)
        .execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn finish(&self, counts: &Counts, failed: bool) -> Result<()> {
        retry_transaction(|| self.finish_once(counts, failed)).await
    }

    async fn finish_once(&self, counts: &Counts, failed: bool) -> Result<()> {
        let mut tx = write_transaction(&self.pool).await?;
        self.renew_tx(&mut tx).await?;
        let (status, error_code) = if failed {
            ("failed", Some("patch_ingest_failed"))
        } else if counts.partial() {
            ("partial", Some("patch_data_gaps"))
        } else {
            ("succeeded", None)
        };
        sqlx::query(
            "UPDATE ops.job_run SET finished_at=clock_timestamp(),status=$2,counts=$3,error_code=$4 WHERE id=$1",
        )
        .bind(self.job)
        .bind(status)
        .bind(counts.value())
        .bind(error_code)
        .execute(&mut *tx)
        .await?;
        let released = sqlx::query(
            "UPDATE ops.job_lease SET expires_at=clock_timestamp() WHERE lease_key=$1 AND run_id=$2 AND epoch=$3",
        )
        .bind(LEASE_KEY)
        .bind(self.job)
        .bind(self.epoch)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        ensure!(
            released == 1,
            "Patchimport-Lease konnte nicht freigegeben werden."
        );
        tx.commit().await?;
        Ok(())
    }
}

type CatalogRow = (String, String, String, String, Option<String>);

struct Catalog {
    index: AliasIndex,
    fingerprint: String,
}

#[derive(Default)]
struct Outcome {
    inserted: bool,
    reused: bool,
    skipped: bool,
    metadata_missing: bool,
    changes: u64,
    unresolved: u64,
}

#[derive(Deserialize)]
struct PostIdentity {
    id: i64,
}

#[derive(Deserialize)]
struct TitleProof<'a> {
    #[serde(borrow)]
    title: &'a RawValue,
}

async fn write_transaction(pool: &PgPool) -> Result<Transaction<'_, Postgres>> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

async fn retry_transaction<T, F, Fut>(mut operation: F) -> Result<T>
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
