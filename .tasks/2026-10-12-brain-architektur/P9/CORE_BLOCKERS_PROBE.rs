use anyhow::{ensure, Context};
use brain_job::{Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, RawObject, Receipt};
use brain_storage::{gamefacts::receipts, PgStore};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

fn set(raw: &RawObject, key: &str, value: f64, facts: usize) -> anyhow::Result<FactSet> {
    let root = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(&raw.bytes).into(),
    };
    let mut scalar = Receipt {
        locator_kind: "json_pointer".into(),
        locator: "/x".into(),
        value_sha256: [0; 32],
    };
    let token = receipts::value_bytes(&raw.bytes, &scalar)
        .map_err(|r| anyhow::anyhow!("core token rejection: {}", r.code))?;
    scalar.value_sha256 = Sha256::digest(token).into();
    Ok(FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: root,
        }],
        facts: (0..facts)
            .map(|i| Fact {
                entity_key: key.into(),
                stat_key: format!("stat_{i}"),
                value: FactValue::Number(value),
                receipt: scalar.clone(),
            })
            .collect(),
        ..FactSet::default()
    })
}

async fn apply(
    store: &GameFactsStore,
    run: i64,
    raw: &RawObject,
    key: &str,
    value: f64,
    facts: usize,
) -> anyhow::Result<()> {
    store
        .apply(run, raw, "probe", &set(raw, key, value, facts)?)
        .await
        .map_err(|r| anyhow::anyhow!("application rejection: {}", r.code))
}

async fn guards(admin: &PgPool, writer: &PgPool, store: &GameFactsStore) -> anyhow::Result<()> {
    let published = store.begin_run("probe").await?;
    let a = store
        .capture(published, "probe", "r1", "original", br#"{"x":1}"#.to_vec())
        .await?;
    apply(store, published, &a, "hero_guard_probe", 1.0, 1).await?;
    store
        .finish_run(
            published,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await?;
    let (entity, receipt): (i64, i64) =
        sqlx::query_as("SELECT entity_id,receipt_id FROM canon.fact WHERE valid_from_run=$1")
            .bind(published)
            .fetch_one(writer)
            .await?;
    let pending = store.begin_run("probe").await?;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'injected',999,$2,$3)")
        .bind(entity).bind(pending).bind(receipt).execute(writer).await?;
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.current_fact WHERE entity_id=$1 AND stat_key='injected' AND value_num=999")
        .bind(entity).fetch_one(admin).await?;
    ensure!(visible == 1, "failed-run injection did not reproduce");
    let b = store
        .capture(pending, "probe", "r2", "changed", br#"{"x":2}"#.to_vec())
        .await?;
    apply(store, pending, &b, "hero_guard_probe", 2.0, 1).await?;
    let pending_receipt: i64 = sqlx::query_scalar(
        "SELECT receipt_id FROM canon.fact WHERE valid_from_run=$1 AND stat_key='stat_0'",
    )
    .bind(pending)
    .fetch_one(writer)
    .await?;
    sqlx::query("SELECT canon.write_run_fact($1,'injected',2,NULL,NULL,$2,$3)")
        .bind(entity)
        .bind(pending)
        .bind(pending_receipt)
        .execute(writer)
        .await?;
    let visible: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM serve.current_fact WHERE entity_id=$1 AND stat_key='injected'",
    )
    .bind(entity)
    .fetch_one(admin)
    .await?;
    ensure!(
        visible == 0,
        "injected publication withdrawal did not reproduce"
    );
    let before: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE ingest_run_id=$1")
            .bind(published)
            .fetch_one(admin)
            .await?;
    store
        .apply(published, &a, "probe-empty", &FactSet::default())
        .await
        .map_err(|r| anyhow::anyhow!("empty finalized application rejected: {}", r.code))?;
    let after: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE ingest_run_id=$1")
            .bind(published)
            .fetch_one(admin)
            .await?;
    ensure!(
        after == before + 1,
        "finalized empty application did not add a receipt"
    );
    println!("publication_guard_probe: failed_run_insert_visible=true; withdrawal=true; finalized_empty_application_accepted=true");
    Ok(())
}

async fn ambiguous(admin: &PgPool, writer: &PgPool, store: &GameFactsStore) -> anyhow::Result<()> {
    let run = store.begin_run("probe").await?;
    let raw = store
        .capture(
            run,
            "probe",
            "r1",
            "ambiguous",
            br#"{"x":1,"x":2}"#.to_vec(),
        )
        .await?;
    apply(store, run, &raw, "hero_collision_probe", 2.0, 1).await?;
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await?;
    let report = brain_job::receipt_audit::run(writer).await?;
    let rejected: i64 = sqlx::query_scalar("SELECT count(*) FROM ops.receipt_audit_rejection WHERE run_id=$1 AND raw_object_id=$2 AND code='locator_collision'")
        .bind(report.run_id).bind(raw.id).fetch_one(admin).await?;
    let published: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE raw_object_id=$1")
            .bind(raw.id)
            .fetch_one(admin)
            .await?;
    ensure!(
        rejected == 1 && published == 0,
        "duplicate-key audit divergence did not reproduce"
    );
    println!(
        "duplicate_json_probe: core_accepted=true; audit_object_rejected=true; serving_hidden=true"
    );
    Ok(())
}

async fn replay(admin: &PgPool, store: &GameFactsStore) -> anyhow::Result<()> {
    let run = store.begin_run("probe").await?;
    let a = store
        .capture(run, "probe", "r1", "replay", br#"{"x":1}"#.to_vec())
        .await?;
    let b = store
        .capture(run, "probe", "r2", "replay", br#"{"x":2}"#.to_vec())
        .await?;
    apply(store, run, &a, "hero_replay_probe", 1.0, 4095).await?;
    apply(store, run, &b, "hero_replay_probe", 2.0, 4095).await?;
    let rejection = match store
        .apply(run, &a, "probe", &set(&a, "hero_replay_probe", 1.0, 4095)?)
        .await
    {
        Ok(()) => anyhow::bail!("same-run cumulative receipt cap did not reproduce"),
        Err(rejection) => rejection,
    };
    ensure!(
        rejection.code == "sql_write" && rejection.detail.contains("23514"),
        "unexpected replay rejection"
    );
    store.reject(run, a.id, "probe", &rejection).await?;
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 2,
                rejected: 1,
                ignored: 0,
            },
        )
        .await?;
    let (count, min, max): (i64, f64, f64) = sqlx::query_as("SELECT count(*),min(value_num),max(value_num) FROM serve.current_fact WHERE entity_id=(SELECT id FROM canon.entity WHERE key='hero_replay_probe')")
        .fetch_one(admin).await?;
    ensure!(
        count == 4095 && min == 2.0 && max == 2.0,
        "replay did not retain B publication"
    );
    println!("same_run_replay_probe: rows_per_application=4096; final_A_sqlstate=23514; published_B=true");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pg = scratch_pg::ScratchPg::start();
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
        .await?;
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate")
        .execute(&admin).await?;
    PgStore::new(admin.clone()).migrate_gamefacts().await?;
    let writer = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.username("brain_ingest"))
        .await?;
    let store = GameFactsStore::new(writer.clone());
    guards(&admin, &writer, &store)
        .await
        .context("publication probe")?;
    ambiguous(&admin, &writer, &store)
        .await
        .context("duplicate JSON probe")?;
    replay(&admin, &store)
        .await
        .context("same-run replay probe")?;
    writer.close().await;
    admin.close().await;
    Ok(())
}
