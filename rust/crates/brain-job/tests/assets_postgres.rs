use brain_job::{assets::storage::Store, store::Counts};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

async fn pool(scratch: &scratch_pg::ScratchPg, user: &str, database: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(scratch.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username(user)
                .database(database)
                .ssl_mode(PgSslMode::Disable)
                .disable_statement_logging(),
        )
        .await
        .unwrap()
}

async fn publish_old_extractor(pool: &PgPool, locator: &str, hash: &[u8]) -> (i64, i64, i64) {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-assets','brain-assets','p2-old') RETURNING id")
        .fetch_one(&mut *tx).await.unwrap();
    let (source, base): (i64, String) = sqlx::query_as(
        "SELECT id,last_complete_revision FROM raw.source WHERE locator=$1 FOR UPDATE",
    )
    .bind(locator)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let (object, target): (i64, String) = sqlx::query_as("SELECT id,revision FROM raw.object WHERE source_id=$1 AND external_id='items/en.json' AND content_sha256=$2")
        .bind(source).bind(hash).fetch_one(&mut *tx).await.unwrap();
    let raw_run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,'brain-assets','p2-old') RETURNING id")
        .bind(source).bind(job).bind(base).bind(&target).fetch_one(&mut *tx).await.unwrap();
    let revision: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,'brain-assets','p2-old') RETURNING id")
        .bind(source).bind(raw_run).bind(&target).fetch_one(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES ($1,'items/en.json',$2,'upsert','succeeded')")
        .bind(raw_run).bind(object).execute(&mut *tx).await.unwrap();
    for table in ["fact", "alias", "relation"] {
        sqlx::query(&format!("UPDATE canon.{table} a SET valid_to_revision_id=$2 FROM canon.receipt r WHERE a.source_id=$1 AND a.source_path='items/en.json' AND a.valid_to_revision_id IS NULL AND r.id=a.receipt_id AND r.extractor='brain-assets'"))
            .bind(source).bind(revision).execute(&mut *tx).await.unwrap();
    }
    let proof: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,'json_pointer','/0/properties/AbilityCooldown/value',$4,'brain-assets','p2-old') RETURNING id")
        .bind(object).bind(source).bind(revision).bind(Sha256::digest(b"24.0").to_vec()).fetch_one(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) SELECT id,'ability.AbilityCooldown',$1,'items/en.json',24.0,$2,$3 FROM canon.entity WHERE kind='ability' AND key='viscous_goo_grenade'")
        .bind(source).bind(proof).bind(revision).execute(&mut *tx).await.unwrap();
    sqlx::query(
        "UPDATE raw.run SET status='succeeded',last_completed_path='items/en.json' WHERE id=$1",
    )
    .bind(raw_run)
    .execute(&mut *tx)
    .await
    .unwrap();
    sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
        .bind(source)
        .bind(target)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE canon.source_head SET published_revision_id=$2 WHERE source_id=$1")
        .bind(source)
        .bind(revision)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE ops.job_run SET status='succeeded',finished_at=clock_timestamp() WHERE id=$1",
    )
    .bind(job)
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    (revision, raw_run, object)
}

#[tokio::test]
async fn raw_reversion_dedup_rollback_publication_and_source_separation_use_postgres() {
    let scratch = scratch_pg::ScratchPg::start();
    let admin = pool(&scratch, "brain_core_test", "postgres").await;
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(
        std::path::PathBuf::from(directory),
        scratch.directory.join("data")
    );
    sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER; CREATE ROLE brain_ingest LOGIN NOSUPERUSER; CREATE ROLE brain_service LOGIN NOSUPERUSER; CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;").execute(&admin).await.unwrap();
    sqlx::query("CREATE DATABASE assets_test OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let owner = pool(&scratch, "brain_migrate", "assets_test").await;
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .execute(&owner)
    .await
    .unwrap();
    sqlx::query("ALTER ROLE brain_ingest SET default_transaction_isolation='repeatable read'")
        .execute(&admin)
        .await
        .unwrap();
    let pool = pool(&scratch, "brain_ingest", "assets_test").await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    assert!(Store::start(pool.clone(), 300).await.is_err());
    let mut counts = Counts::default();
    let a = br#"[{"id":3247040238,"class_name":"viscous_goo_grenade","type":"ability","name":"Splatter","properties":{"AbilityCooldown":{"value":"26.0"}}}]"#;
    let b = br#"[{"id":3247040238,"class_name":"viscous_goo_grenade","type":"ability","name":"Splatter changed","properties":{"AbilityCooldown":{"value":24.0}}}]"#;
    let en = "https://api.deadlock-api.com/v1/assets/items?language=english";
    let de = "https://api.deadlock-api.com/v1/assets/items?language=german";
    store.object(en, "en", a, &mut counts).await.unwrap();
    let first: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.fact")
        .fetch_one(&pool)
        .await
        .unwrap();
    store.object(en, "en", a, &mut counts).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.fact")
            .fetch_one(&pool)
            .await
            .unwrap(),
        first
    );
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.sources_unchanged, 1);
    store.object(de, "de", a, &mut counts).await.unwrap();
    let gamefiles = brain_job::store::Store::start(pool.clone(), 300)
        .await
        .unwrap();
    let source = brain_job::git::Source::ALL[0];
    let source_id = gamefiles.source(source).await.unwrap();
    let run = gamefiles
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let bytes = br#"{"viscous_goo_grenade":{"AbilityCooldown":26.0}}"#;
    let mut gamefiles_counts = Counts::default();
    gamefiles
        .object(
            &run,
            source,
            "data/json/ability-data.json",
            Some(brain_job::git::Blob {
                bytes: bytes.to_vec(),
                hash: Sha256::digest(bytes).to_vec(),
            }),
            &mut gamefiles_counts,
        )
        .await
        .unwrap();
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&gamefiles_counts, false).await.unwrap();
    store.object(en, "en", b, &mut counts).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_fact WHERE stat_key='ability.AbilityCooldown'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        3
    );
    store.object(en, "en", a, &mut counts).await.unwrap();
    assert_eq!(counts.raw_inserted, 3);
    assert_eq!(counts.raw_reused, 1);
    let actual: (String, String, Vec<u8>, String) = sqlx::query_as("SELECT f.value_num::text,r.locator,r.value_sha256,r.extractor FROM serve.current_fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.source s ON s.id=f.source_id WHERE f.stat_key='ability.AbilityCooldown' AND s.locator=$1").bind(en).fetch_one(&pool).await.unwrap();
    assert_eq!(
        actual,
        (
            "26.0".into(),
            "/0/properties/AbilityCooldown/value".into(),
            Sha256::digest(br#""26.0""#).to_vec(),
            "brain-assets".into()
        )
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_alias WHERE alias_norm='splatter changed'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    let before: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap();
    assert!(store
        .object(
            en,
            "en",
            br#"[{"id":1,"class_name":"bad","type":"ability","name":"a","name":"b"}]"#,
            &mut counts
        )
        .await
        .is_err());
    assert_eq!(sqlx::query_as::<_, (i64,i64,i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap(), before);
    store.finish(&counts, 0, false).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM ops.job_run WHERE job='ingest-assets' AND status='succeeded'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        1
    );
    let hash = Sha256::digest(b).to_vec();
    let (old_revision, old_run, current_object) = publish_old_extractor(&pool, en, &hash).await;
    let unchanged_sources: Vec<(i64, Option<String>, Option<String>)> = sqlx::query_as("SELECT f.id,f.value_num::text,f.value_text FROM serve.current_fact f JOIN raw.source s ON s.id=f.source_id WHERE s.locator<>$1 ORDER BY f.id")
        .bind(en).fetch_all(&pool).await.unwrap();
    let raw_before: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&pool)
        .await
        .unwrap();
    let replay = Store::start(pool.clone(), 300).await.unwrap();
    let mut replay_counts = Counts::default();
    assert_eq!(
        replay.replay(en, "en", &mut replay_counts).await.unwrap(),
        0
    );
    assert_eq!(replay_counts.raw_inserted, 0);
    assert_eq!(replay_counts.raw_reused, 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        raw_before
    );
    let (new_revision, version, object, target, value, receipt_version, receipt_hash): (i64, String, i64, String, String, String, Vec<u8>) = sqlx::query_as("SELECT v.id,v.extractor_version,r.raw_object_id,v.source_revision,f.value_num::text,r.extractor_version,r.value_sha256 FROM canon.source_head h JOIN canon.revision v ON v.id=h.published_revision_id JOIN serve.current_fact f ON f.source_id=h.source_id JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.source s ON s.id=h.source_id WHERE s.locator=$1 AND f.stat_key='ability.AbilityCooldown'")
        .bind(en).fetch_one(&pool).await.unwrap();
    assert!(new_revision > old_revision);
    assert_eq!(version, brain_job::assets::VERSION);
    assert_eq!(object, current_object);
    assert_eq!(target, hex::encode(&hash));
    assert_eq!(value, "24.0");
    assert_eq!(receipt_version, brain_job::assets::VERSION);
    assert_eq!(receipt_hash, Sha256::digest(b"24.0").to_vec());
    let rebuilt: (i64, bool) = sqlx::query_as("SELECT count(*),bool_and(r.extractor_version=$2 AND r.raw_object_id=$3 AND r.revision_id=$4) FROM serve.current_fact f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.source s ON s.id=f.source_id WHERE s.locator=$1")
        .bind(en).bind(brain_job::assets::VERSION).bind(current_object).bind(new_revision).fetch_one(&pool).await.unwrap();
    assert_eq!(rebuilt, (4, true));
    assert_eq!(sqlx::query_scalar::<_, String>("SELECT f.value_text FROM serve.current_fact f JOIN raw.source s ON s.id=f.source_id WHERE s.locator=$1 AND f.stat_key='identity.name'").bind(en).fetch_one(&pool).await.unwrap(), "Splatter changed");
    assert_eq!(
        sqlx::query_as::<_, (String, String)>(
            "SELECT status,extractor_version FROM raw.run WHERE id=$1"
        )
        .bind(old_run)
        .fetch_one(&pool)
        .await
        .unwrap(),
        ("succeeded".into(), "p2-old".into())
    );
    assert_eq!(sqlx::query_as::<_, (i64, Option<String>, Option<String>)>("SELECT f.id,f.value_num::text,f.value_text FROM serve.current_fact f JOIN raw.source s ON s.id=f.source_id WHERE s.locator<>$1 ORDER BY f.id").bind(en).fetch_all(&pool).await.unwrap(), unchanged_sources);
    let repeated_before: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(
        replay.replay(en, "en", &mut replay_counts).await.unwrap(),
        0
    );
    replay
        .object(en, "en", b, &mut replay_counts)
        .await
        .unwrap();
    assert_eq!(replay_counts.raw_reused, 1);
    assert_eq!(replay_counts.sources_unchanged, 1);
    assert_eq!(sqlx::query_as::<_, (i64, i64, i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)").fetch_one(&pool).await.unwrap(), repeated_before);
    let unavailable: anyhow::Result<()> = replay
        .during(async { anyhow::bail!("assets_refresh_unavailable") })
        .await;
    assert!(unavailable.is_err());
    replay
        .finish(&replay_counts, 0, unavailable.is_err())
        .await
        .unwrap();
    assert_eq!(sqlx::query_as::<_, (String, i64, i64)>("SELECT status,(counts->>'raw_inserted')::bigint,(counts->>'raw_reused')::bigint FROM ops.job_run WHERE id=$1").bind(replay.job).fetch_one(&pool).await.unwrap(), ("failed".into(), 0, 1));
    assert_eq!(sqlx::query_scalar::<_, i64>("SELECT published_revision_id FROM canon.source_head h JOIN raw.source s ON s.id=h.source_id WHERE s.locator=$1").bind(en).fetch_one(&pool).await.unwrap(), new_revision);
    publish_old_extractor(&pool, en, &hash).await;
    let fenced = Store::start(pool.clone(), 300).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    let pending: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) SELECT s.id,r.job_run_id,s.last_complete_revision,s.last_complete_revision,'brain-assets','p2-old' FROM raw.source s JOIN canon.source_head h ON h.source_id=s.id JOIN canon.revision v ON v.id=h.published_revision_id JOIN raw.run r ON r.id=v.raw_run_id WHERE s.locator=$1 RETURNING id")
        .bind(en).fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    let before: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)")
        .fetch_one(&pool).await.unwrap();
    assert!(fenced.replay(en, "en", &mut replay_counts).await.is_err());
    assert_eq!(sqlx::query_as::<_, (i64,i64,i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap(), before);
    assert_eq!(
        sqlx::query_as::<_, (String, String)>(
            "SELECT status,extractor_version FROM raw.run WHERE id=$1"
        )
        .bind(pending)
        .fetch_one(&pool)
        .await
        .unwrap(),
        ("running".into(), "p2-old".into())
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE ops.job_lease SET epoch=epoch+1 WHERE lease_key='brain-job:ingest-assets'")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let lost = fenced
        .replay(en, "en", &mut replay_counts)
        .await
        .unwrap_err();
    assert_eq!(lost.to_string(), "assets_lease_lost");
    assert!(fenced.object(en, "en", b, &mut counts).await.is_err());
    assert_eq!(sqlx::query_as::<_, (i64,i64,i64)>("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.receipt),(SELECT count(*) FROM raw.run)").fetch_one(&pool).await.unwrap(), before);
    pool.close().await;
    owner.close().await;
    admin.close().await;
}
