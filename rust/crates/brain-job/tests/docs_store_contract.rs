use brain_job::*;

use anyhow::{ensure, Context, Result};
use brain_storage::{gamefacts::receipts, PgStore};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgConnectOptions, postgres::PgPoolOptions, postgres::PgSslMode, PgPool, Row};
use std::collections::BTreeSet;

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const REVISION: &str = "400231af7e8196e145111e4150c2e24c2ab79fb5";
const GOOD: &str = "public/deadlock-helden/billy.html";
const COLLISION: &str = "public/poison_collision.html";
const OVERSIZED: &str = "public/poison_oversized.html";
const INTERNAL: &str = "internal/deadlock-helden/billy.html";
const DOCS_MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-13-brain-gamefacts-p5-docs-v1.sql");

fn billy() -> Result<String> {
    let declaration = include_str!("docs_extract_contract.rs")
        .lines()
        .find(|line| line.starts_with("const BILLY: &str = "))
        .context("Frozen Billy fixture is missing")?;
    let (_, literal) = declaration
        .split_once(" = ")
        .context("Billy fixture declaration")?;
    serde_json::from_str(
        literal
            .strip_suffix(';')
            .context("Billy fixture terminator")?,
    )
    .context("Frozen Billy fixture literal")
}

fn input(path: &str, bytes: Vec<u8>) -> Result<InputObject> {
    Ok(InputObject {
        revision: REVISION.into(),
        path: path.into(),
        bytes,
    })
}

fn objects() -> Result<Vec<Result<InputObject>>> {
    let source = billy()?;
    let collision = source.replace("</main>", "<section id=\" WER-IST-BILLY-UND-WIE-GEWINNT-MAN-MIT-IHM \"><h2>Kollision</h2><p>Andere ID.</p></section></main>");
    let oversized = source.replace(
        "<h1>Billy</h1>",
        &format!("<h1>{}</h1>", "x".repeat(limits::TEXT_BYTES + 1)),
    );
    Ok(vec![
        input(GOOD, source.into_bytes()),
        input(COLLISION, collision.into_bytes()),
        input(OVERSIZED, oversized.into_bytes()),
    ])
}

async fn count(pool: &PgPool, sql: &str) -> Result<i64> {
    Ok(sqlx::query_scalar(sql).fetch_one(pool).await?)
}

async fn run_status(pool: &PgPool, id: i64, expected: (i64, i64, i64)) -> Result<()> {
    let row: (String, i64, i64, i64, bool) = sqlx::query_as("SELECT status,accepted,rejected,ignored,finished IS NOT NULL FROM ops.ingest_run WHERE id=$1")
        .bind(id).fetch_one(pool).await?;
    ensure!(
        row.0 == "succeeded" && row.4 && (row.1, row.2, row.3) == expected,
        "Unexpected ingest run status/counts"
    );
    Ok(())
}

async fn snapshot(pool: &PgPool) -> Result<(i64, i64, i64, i64)> {
    Ok(sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM canon.entity_identity)")
        .fetch_one(pool).await?)
}

async fn setup(pg: &scratch_pg::ScratchPg) -> Result<(PgPool, PgPool, PgPool)> {
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .map_err(|_| anyhow::anyhow!("Disposable database connection failed"))?;
    ensure!(
        count(
            &admin,
            "SELECT count(*) FROM pg_namespace WHERE nspname IN('raw','canon','serve','ops')"
        )
        .await?
            == 0,
        "Disposable fixture must be empty"
    );
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate;")
        .execute(&admin).await?;
    let schema = PgStore::new(admin.clone());
    schema.migrate_gamefacts().await?;
    schema.check_gamefacts_schema().await?;
    sqlx::raw_sql(DOCS_MIGRATION).execute(&admin).await?;
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options.clone())
        .await?;
    let reader = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_tool").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await?;
    Ok((admin, writer, reader))
}

async fn replay(pool: &PgPool) -> Result<()> {
    let rows = sqlx::query("SELECT r.locator_kind::text,r.locator::text,r.value_sha256,o.bytes FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path=$1 ORDER BY r.id")
        .bind(GOOD).fetch_all(pool).await?;
    ensure!(rows.len() == 17, "Unexpected persisted receipt count");
    let original = billy()?.into_bytes();
    let mut locators = BTreeSet::new();
    let mut ranged = 0;
    for row in rows {
        let bytes: Vec<u8> = row.try_get("bytes")?;
        ensure!(bytes == original, "Captured original bytes changed");
        let hash: Vec<u8> = row.try_get("value_sha256")?;
        let receipt = Receipt {
            locator_kind: row.try_get("locator_kind")?,
            locator: row.try_get("locator")?,
            value_sha256: hash
                .try_into()
                .map_err(|_| anyhow::anyhow!("Receipt hash width"))?,
        };
        let value = receipts::value_bytes(&bytes, &receipt)
            .map_err(|r| anyhow::anyhow!("Receipt replay rejected: {}", r.code))?;
        let expected: [u8; 32] = Sha256::digest(value).into();
        ensure!(
            expected == receipt.value_sha256,
            "Persisted original-byte hash mismatch"
        );
        if receipt.locator_kind == "byte_ranges" {
            ranged += 1;
            locators.insert(receipt.locator);
        } else {
            ensure!(
                receipt.locator_kind == "raw_bytes" && receipt.locator.is_empty(),
                "Invalid page receipt"
            );
        }
    }
    ensure!(
        ranged == 12 && locators.len() == 6 && locators.iter().any(|l| l.contains(',')),
        "Original section range coverage is incomplete"
    );
    Ok(())
}

async fn corpus(store: &GameFactsStore, admin: &PgPool) -> Result<()> {
    let Some(repository) = std::env::var_os("BRAIN_DOCS_CORPUS_REPOSITORY") else {
        return Ok(());
    };
    let repository = std::path::PathBuf::from(repository);
    let loader = sources::SOURCES
        .iter()
        .find(|(name, _)| *name == "deadlock-docs")
        .context("Registered Docs source is missing")?
        .1;
    let (mut documents, mut bytes, mut largest) = (0, 0, 0);
    let streamed = loader(&repository, false).map(|object| {
        let object = object?;
        ensure!(object.revision == REVISION, "Corpus revision changed");
        documents += 1;
        bytes += object.bytes.len();
        largest = largest.max(object.bytes.len());
        Ok(object)
    });
    let first = ingest(store, "deadlock-docs", streamed, registry::EXTRACTORS).await?;
    ensure!(documents > 0, "Corpus is empty");
    run_status(
        admin,
        first.run_id,
        (
            first.counts.accepted as i64,
            first.counts.rejected as i64,
            first.counts.ignored as i64,
        ),
    )
    .await?;
    let before = snapshot(admin).await?;
    let second = ingest(
        store,
        "deadlock-docs",
        loader(&repository, false),
        registry::EXTRACTORS,
    )
    .await?;
    ensure!(
        second.counts.accepted == 0
            && second.counts.rejected == first.counts.rejected
            && second.counts.ignored == first.counts.accepted + first.counts.ignored,
        "Corpus repeat counts"
    );
    run_status(
        admin,
        second.run_id,
        (
            0,
            second.counts.rejected as i64,
            second.counts.ignored as i64,
        ),
    )
    .await?;
    ensure!(
        snapshot(admin).await? == before,
        "Corpus repeat created raw/canonical rows"
    );
    let reasons: Vec<(String,i64)> = sqlx::query_as("SELECT code,count(*) FROM ops.ingest_rejection WHERE run_id=$1 GROUP BY code ORDER BY code")
        .bind(first.run_id).fetch_all(admin).await?;
    println!("DOCS_CORPUS revision={REVISION} documents={documents} bytes={bytes} largest={largest} first={}/{}/{} second={}/{}/{} snapshot={before:?} rejection_counts={reasons:?}",
        first.counts.accepted, first.counts.rejected, first.counts.ignored, second.counts.accepted, second.counts.rejected, second.counts.ignored);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn actual_docs_ingest_poison_idempotence_receipts_and_scopes() -> Result<()> {
    let pg = scratch_pg::ScratchPg::start();
    let (admin, writer, reader) = setup(&pg).await?;
    let store = GameFactsStore::new(writer.clone());
    let first = ingest(&store, "deadlock-docs", objects()?, registry::EXTRACTORS).await?;
    ensure!(
        (
            first.counts.accepted,
            first.counts.rejected,
            first.counts.ignored
        ) == (1, 2, 0),
        "First run counts"
    );
    run_status(&admin, first.run_id, (1, 2, 0)).await?;
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(first.run_id)
            .fetch_all(&admin)
            .await?;
    ensure!(
        reasons == ["key_collision", "value_too_large"],
        "Missing concrete poison rejections"
    );
    let before = snapshot(&admin).await?;
    ensure!(
        before == (3, 16, 17, 1),
        "Unexpected first canonical snapshot"
    );
    ensure!(count(&admin, "SELECT count(*) FROM canon.fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path LIKE 'public/poison_%'").await? == 0, "Poison facts were partially stored");
    ensure!(count(&admin, "SELECT count(*) FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path LIKE 'public/poison_%'").await? == 0, "Poison receipts were partially stored");
    ensure!(
        count(&admin, "SELECT count(*) FROM canon.alias").await? == 0
            && count(&admin, "SELECT count(*) FROM canon.relation").await? == 0,
        "Document carriers leaked aliases or relations"
    );
    ensure!(
        count(&admin, "SELECT count(*) FROM canon.entity").await? == 1
            && count(
                &admin,
                "SELECT count(*) FROM canon.entity WHERE kind='doc_page'"
            )
            .await?
                == 1,
        "Poison entities were partially stored or document kind changed"
    );
    let public: (String, String, String, i32, String) =
        sqlx::query_as("SELECT path,title,lang,section_count,scope FROM serve.doc_page")
            .fetch_one(&reader)
            .await?;
    ensure!(
        public == (GOOD.into(), "Billy".into(), "de".into(), 6, "public".into()),
        "Public metadata projection mismatch"
    );
    ensure!(
        count(
            &reader,
            "SELECT count(*) FROM serve.doc_section WHERE scope='public'"
        )
        .await?
            == 6,
        "Public sections are not fully published"
    );
    replay(&admin).await?;
    let second = ingest(&store, "deadlock-docs", objects()?, registry::EXTRACTORS).await?;
    ensure!(
        (
            second.counts.accepted,
            second.counts.rejected,
            second.counts.ignored
        ) == (0, 2, 1),
        "Second identical run counts"
    );
    run_status(&admin, second.run_id, (0, 2, 1)).await?;
    ensure!(
        snapshot(&admin).await? == before,
        "Second identical run created raw/canonical rows"
    );
    let private = ingest(
        &store,
        "deadlock-docs",
        [input(INTERNAL, billy()?.into_bytes())],
        registry::EXTRACTORS,
    )
    .await?;
    ensure!(
        (
            private.counts.accepted,
            private.counts.rejected,
            private.counts.ignored
        ) == (1, 0, 0),
        "Internal scope ingest counts"
    );
    run_status(&admin, private.run_id, (1, 0, 0)).await?;
    let scopes: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT path,scope,count(*) FROM serve.doc_section GROUP BY path,scope ORDER BY path",
    )
    .fetch_all(&reader)
    .await?;
    ensure!(
        scopes
            == [
                (INTERNAL.into(), "internal".into(), 6),
                (GOOD.into(), "public".into(), 6)
            ],
        "Scope metadata or section publication mismatch"
    );
    ensure!(
        snapshot(&admin).await? == (4, 32, 34, 2),
        "Internal scope carrier snapshot mismatch"
    );
    println!("DOCS_STORE_PROOF first=1/2/0 second=0/2/1 internal=1/0/0 raw=4 facts=32 receipts=34 sections=12 ranged_receipts=12 distinct_ranges=6 poison_facts=0");
    corpus(&store, &admin).await?;
    let status = std::fs::read_to_string("/proc/self/status")?;
    let peak = status
        .lines()
        .find(|line| line.starts_with("VmHWM:"))
        .context("Process peak RSS is unavailable")?;
    println!("DOCS_PROCESS_PEAK {peak}");
    reader.close().await;
    writer.close().await;
    admin.close().await;
    Ok(())
}
