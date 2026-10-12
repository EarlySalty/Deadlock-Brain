use brain_storage::PgStore;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Duration,
};

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let root = PathBuf::from(format!("/tmp/brain-gamefacts.{}", std::process::id()));
        std::fs::create_dir(&root).expect("new private scratch directory");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::create_dir(root.join("socket")).unwrap();
        let scratch = Self { root };
        success(
            Command::new("/usr/lib/postgresql/16/bin/initdb")
                .args([
                    "--auth=trust",
                    "--no-locale",
                    "--username=brain_test_dba",
                    "--pgdata",
                ])
                .arg(scratch.root.join("data"))
                .output()
                .unwrap(),
        );
        success(
            Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                .arg("--pgdata")
                .arg(scratch.root.join("data"))
                .arg("--log")
                .arg(scratch.root.join("postgres.log"))
                .args(["--wait", "--options"])
                .arg(format!(
                    "-k {} -p 55467 -h '' -c fsync=off",
                    scratch.root.join("socket").display()
                ))
                .arg("start")
                .output()
                .unwrap(),
        );
        scratch
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(self.root.join("socket").to_str().unwrap())
                    .port(55467)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(sqlx::postgres::PgSslMode::Disable),
            )
            .await
            .unwrap()
    }

    fn cli(&self, command: &str, user: &str) -> Output {
        let config = self.root.join(format!("{user}.json"));
        std::fs::write(
            &config,
            serde_json::to_vec(&json!({"socket":self.root.join("socket"),"port":55467,
                "database":"brain_gamefacts","user":user}))
            .unwrap(),
        )
        .unwrap();
        Command::new(env!("CARGO_BIN_EXE_brain-migrate"))
            .env_clear()
            .args([command, "--config"])
            .arg(config)
            .output()
            .unwrap()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let stopped = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("--pgdata")
            .arg(self.root.join("data"))
            .args(["--wait", "--mode=immediate", "stop"])
            .output();
        if stopped.is_ok_and(|output| output.status.success()) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

fn success(output: Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

async fn error_code(pool: &PgPool, sql: &str, expected: &str) {
    let error = sqlx::raw_sql(sql).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some(expected)
    );
}

async fn wait_blocked(observer: &PgPool, pid: i32, blocker: i32) {
    tokio::time::timeout(Duration::from_secs(8), async {
        loop {
            let blocked: bool = sqlx::query_scalar("SELECT $2 = ANY(pg_blocking_pids($1))")
                .bind(pid)
                .bind(blocker)
                .fetch_one(observer)
                .await
                .unwrap();
            if blocked {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("a real PostgreSQL lock wait must be observed");
}

struct Pending {
    source: i64,
    run: i64,
    revision: i64,
    entity: i64,
    receipt: i64,
}

async fn pending_source(ingest: &PgPool, key: &str) -> Pending {
    let mut tx = ingest.begin().await.unwrap();
    let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('contract','kv','v1') RETURNING id")
        .fetch_one(&mut *tx).await.unwrap();
    let source: i64 =
        sqlx::query_scalar("INSERT INTO raw.source(kind,locator) VALUES ('git',$1) RETURNING id")
            .bind(format!("synthetic://{key}"))
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES ($1,$2,'first','kv','v1') RETURNING id")
        .bind(source).bind(job).fetch_one(&mut *tx).await.unwrap();
    let object: i64 = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,'hero.kv','first',decode(repeat('01',32),'hex'),'text/plain','contract') RETURNING id")
        .bind(source).fetch_one(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES ($1,'hero.kv',$2,'upsert','succeeded')")
        .bind(run).bind(object).execute(&mut *tx).await.unwrap();
    let revision: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,'first','kv','v1') RETURNING id")
        .bind(source).bind(run).fetch_one(&mut *tx).await.unwrap();
    let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,'kv_path','identity',decode(repeat('02',32),'hex'),'kv','v1') RETURNING id")
        .bind(object).bind(source).bind(revision).fetch_one(&mut *tx).await.unwrap();
    let entity: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero',$1,$2) RETURNING id",
    )
    .bind(key)
    .bind(receipt)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES ($1,'health',$2,'hero.kv',200,$3,$4)")
        .bind(entity).bind(source).bind(receipt).bind(revision).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    Pending {
        source,
        run,
        revision,
        entity,
        receipt,
    }
}

async fn publish(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, pending: &Pending) {
    sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
        .bind(pending.run)
        .execute(&mut **tx)
        .await
        .unwrap();
    sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2)")
        .bind(pending.source)
        .bind(pending.revision)
        .execute(&mut **tx)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.source SET last_complete_revision='first' WHERE id=$1")
        .bind(pending.source)
        .execute(&mut **tx)
        .await
        .unwrap();
}

async fn overlapping_publishers(ingest: &PgPool, owner: &PgPool) {
    let first = pending_source(ingest, "overlap-a").await;
    let second = pending_source(ingest, "overlap-b").await;
    let mut a = ingest.begin().await.unwrap();
    let a_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *a)
        .await
        .unwrap();
    publish(&mut a, &first).await;
    let mut b = ingest.begin().await.unwrap();
    let b_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *b)
        .await
        .unwrap();
    let b_entity = second.entity;
    let waiting = tokio::spawn(async move {
        publish(&mut b, &second).await;
        b.commit().await.unwrap();
    });
    wait_blocked(owner, b_pid, a_pid).await;
    a.commit().await.unwrap();
    tokio::time::timeout(Duration::from_secs(8), waiting)
        .await
        .unwrap()
        .unwrap();
    let projected: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.hero_profile WHERE entity_id=ANY($1)")
            .bind(vec![first.entity, b_entity])
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(projected, 2);
}

async fn overlapping_completion_and_next_run(ingest: &PgPool, owner: &PgPool) {
    let pending = pending_source(ingest, "completion-order").await;
    let mut completion = ingest.begin().await.unwrap();
    let completion_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *completion)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
        .bind(pending.run)
        .execute(&mut *completion)
        .await
        .unwrap();
    let mut next = ingest.begin().await.unwrap();
    let next_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *next)
        .await
        .unwrap();
    let source = pending.source;
    let next_task = tokio::spawn(async move {
        let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,1,'first','second','kv','v1') RETURNING id")
            .bind(source).fetch_one(&mut *next).await.unwrap();
        next.commit().await.unwrap();
        run
    });
    wait_blocked(owner, next_pid, completion_pid).await;
    sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2)")
        .bind(pending.source)
        .bind(pending.revision)
        .execute(&mut *completion)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.source SET last_complete_revision='first' WHERE id=$1")
        .bind(pending.source)
        .execute(&mut *completion)
        .await
        .unwrap();
    completion.commit().await.unwrap();
    let next_run = tokio::time::timeout(Duration::from_secs(8), next_task)
        .await
        .unwrap()
        .unwrap();
    assert!(next_run > pending.run);
}

async fn overlapping_explicit_refresh(ingest: &PgPool, owner: &PgPool) {
    let pending = pending_source(ingest, "explicit-refresh").await;
    let mut publication = ingest.begin().await.unwrap();
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *publication)
        .await
        .unwrap();
    publish(&mut publication, &pending).await;
    let mut refresh = ingest.acquire().await.unwrap();
    let refresh_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *refresh)
        .await
        .unwrap();
    let waiting = tokio::spawn(async move {
        sqlx::query("SELECT serve.refresh_hero_profile()")
            .execute(&mut *refresh)
            .await
            .unwrap();
    });
    wait_blocked(owner, refresh_pid, pid).await;
    publication.commit().await.unwrap();
    tokio::time::timeout(Duration::from_secs(8), waiting)
        .await
        .unwrap()
        .unwrap();
    let projected: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.hero_profile WHERE entity_id=$1")
            .bind(pending.entity)
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(projected, 1);
}

async fn postgres_refresh_snapshot_mechanics(owner: &PgPool) {
    sqlx::raw_sql("CREATE TABLE docs.snapshot_probe(id integer PRIMARY KEY,value integer NOT NULL);
        INSERT INTO docs.snapshot_probe VALUES (1,0),(2,0);
        CREATE MATERIALIZED VIEW docs.snapshot_projection AS SELECT * FROM docs.snapshot_probe;
        CREATE UNIQUE INDEX snapshot_projection_id ON docs.snapshot_projection(id);
        CREATE FUNCTION docs.legacy_refresh() RETURNS void LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog,pg_temp
        AS $$BEGIN REFRESH MATERIALIZED VIEW CONCURRENTLY docs.snapshot_projection; END$$")
        .execute(owner).await.unwrap();
    let mut a = owner.begin().await.unwrap();
    let a_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *a)
        .await
        .unwrap();
    sqlx::raw_sql(
        "UPDATE docs.snapshot_probe SET value=1 WHERE id=1; SELECT docs.legacy_refresh()",
    )
    .execute(&mut *a)
    .await
    .unwrap();
    let mut b = owner.begin().await.unwrap();
    let b_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *b)
        .await
        .unwrap();
    sqlx::query("UPDATE docs.snapshot_probe SET value=2 WHERE id=2")
        .execute(&mut *b)
        .await
        .unwrap();
    let waiting = tokio::spawn(async move {
        sqlx::query("SELECT docs.legacy_refresh()")
            .execute(&mut *b)
            .await
            .unwrap();
        b.commit().await.unwrap();
    });
    wait_blocked(owner, b_pid, a_pid).await;
    a.commit().await.unwrap();
    tokio::time::timeout(Duration::from_secs(8), waiting)
        .await
        .unwrap()
        .unwrap();
    let values: Vec<i32> =
        sqlx::query_scalar("SELECT value FROM docs.snapshot_projection ORDER BY id")
            .fetch_all(owner)
            .await
            .unwrap();
    assert_eq!(values, [0, 2]);
    sqlx::raw_sql("DROP FUNCTION docs.legacy_refresh(); DROP MATERIALIZED VIEW docs.snapshot_projection; DROP TABLE docs.snapshot_probe")
        .execute(owner).await.unwrap();
}

async fn eligible_ability_limit(ingest: &PgPool, owner: &PgPool) {
    let hidden = pending_source(ingest, "hidden-identities").await;
    let hero = pending_source(ingest, "eligible-hero").await;
    let extra = pending_source(ingest, "eligible-extra").await;
    let mut tx = ingest.begin().await.unwrap();
    let abilities: Vec<i64> = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id) SELECT 'ability','hidden_'||n,$1 FROM generate_series(1,4) n RETURNING id")
        .bind(hidden.receipt).fetch_all(&mut *tx).await.unwrap();
    for (n, ability) in abilities.iter().enumerate() {
        sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,'has_ability',$2,$3,$4,$5,'hero.kv',$6)")
            .bind(hero.entity).bind(ability).bind(i16::try_from(n+1).unwrap()).bind(hero.receipt).bind(hero.source).bind(hero.revision)
            .execute(&mut *tx).await.unwrap();
    }
    sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ($1,'has_ability',2,4,$2,$3,'hero.kv',$4)")
        .bind(hero.entity).bind(extra.receipt).bind(extra.source).bind(extra.revision).execute(&mut *tx).await.unwrap();
    publish(&mut tx, &hero).await;
    publish(&mut tx, &extra).await;
    tx.commit().await.unwrap();
    let visible: Value =
        sqlx::query_scalar("SELECT abilities FROM serve.hero_profile WHERE entity_id=$1")
            .bind(hero.entity)
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(visible["links"].as_array().unwrap().len(), 1);
    assert_eq!(visible["links"][0]["key"], "baba_1");
    assert_eq!(visible["omitted_count"], 0);
    assert_eq!(visible["truncated"], false);
    let mut tx = ingest.begin().await.unwrap();
    publish(&mut tx, &hidden).await;
    tx.commit().await.unwrap();
    let visible: Value =
        sqlx::query_scalar("SELECT abilities FROM serve.hero_profile WHERE entity_id=$1")
            .bind(hero.entity)
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(visible["links"].as_array().unwrap().len(), 4);
    assert_eq!(visible["omitted_count"], 1);
    assert_eq!(visible["truncated"], true);
    for (n, ability) in abilities.iter().enumerate() {
        assert_eq!(visible["links"][n]["entity_id"], *ability);
    }
}

async fn revision_and_isolation_boundaries(ingest: &PgPool, owner: &PgPool) {
    let mut tx = ingest.begin().await.unwrap();
    let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES (1,1,'commit-b','commit-c','kv','v1') RETURNING id")
        .fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    for id in [0, 1] {
        let sql = format!("INSERT INTO canon.revision(id,source_id,raw_run_id,source_revision,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES ({id},1,{run},'commit-c','kv','v1')");
        error_code(ingest, &sql, "42501").await;
        error_code(owner, &sql, "P0001").await;
    }
    for isolation in ["REPEATABLE READ", "SERIALIZABLE"] {
        let mut tx = ingest.begin().await.unwrap();
        sqlx::raw_sql(&format!("SET TRANSACTION ISOLATION LEVEL {isolation}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let error = sqlx::query("UPDATE raw.run SET status='partial' WHERE id=$1")
            .bind(run)
            .execute(&mut *tx)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("0A000")
        );
        tx.rollback().await.unwrap();
        let mut tx = ingest.begin().await.unwrap();
        sqlx::raw_sql(&format!("SET TRANSACTION ISOLATION LEVEL {isolation}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let error = sqlx::query("SELECT serve.refresh_hero_profile()")
            .execute(&mut *tx)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("0A000")
        );
        tx.rollback().await.unwrap();
    }
    let privileged: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname IN ('raw','canon','ops') AND a.attidentity<>'' AND has_column_privilege('brain_ingest',c.oid,a.attnum,'INSERT')")
        .fetch_one(owner).await.unwrap();
    assert_eq!(privileged, 0);
}

async fn full_history_index_plans(ingest: &PgPool, owner: &PgPool) {
    let bulk = pending_source(ingest, "history-index-bulk").await;
    let mut tx = ingest.begin().await.unwrap();
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,receipt_id,valid_from_revision_id) SELECT $1,'health',$2,'hero.kv',jsonb_build_object('ordinal',n),n,$3,$4 FROM generate_series(1,6000) n")
        .bind(bulk.entity).bind(bulk.source).bind(bulk.receipt).bind(bulk.revision).execute(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id) SELECT $1,'bulk_'||n,'en',$2,$3,'hero.kv',$4 FROM generate_series(1,6000) n")
        .bind(bulk.entity).bind(bulk.receipt).bind(bulk.source).bind(bulk.revision).execute(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO canon.entity(kind,key,identity_receipt_id) SELECT 'hero','bulk_'||n,$1 FROM generate_series(1,6000) n")
        .bind(bulk.receipt).execute(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) SELECT id,'has_ability',2,1,$1,$2,'hero.kv',$3 FROM canon.entity WHERE key LIKE 'bulk_%'")
        .bind(bulk.receipt).bind(bulk.source).bind(bulk.revision).execute(&mut *tx).await.unwrap();
    let run: i64 = sqlx::query_scalar(
        "SELECT id FROM raw.run WHERE source_id=1 AND target_revision='commit-c'",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let revision: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,$1,'commit-c','kv','v1') RETURNING id")
        .bind(run).fetch_one(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO raw.run_object(run_id,path,action,parse_status) VALUES ($1,'hero.kv','delete','succeeded')")
        .bind(run).execute(&mut *tx).await.unwrap();
    for table in ["alias", "relation"] {
        sqlx::query(&format!(
            "UPDATE canon.{table} SET valid_to_revision_id=$1 WHERE id=1"
        ))
        .bind(revision)
        .execute(&mut *tx)
        .await
        .unwrap();
    }
    tx.commit().await.unwrap();
    sqlx::raw_sql("ANALYZE canon.fact; ANALYZE canon.alias; ANALYZE canon.relation")
        .execute(owner)
        .await
        .unwrap();
    for table in ["fact", "alias", "relation"] {
        let plan: Value = sqlx::query_scalar(&format!("EXPLAIN (ANALYZE, FORMAT JSON) SELECT source_path,valid_from_revision_id,valid_to_revision_id,receipt_id FROM canon.{table} WHERE source_id=1"))
            .fetch_one(owner).await.unwrap();
        assert!(
            plan.to_string()
                .contains(&format!("{table}_source_history_idx")),
            "{plan}"
        );
        assert!(plan[0]["Plan"]["Actual Rows"].as_u64().unwrap() > 0);
    }
    let plan: Value = sqlx::query_scalar("EXPLAIN (ANALYZE, FORMAT JSON) SELECT to_entity_id FROM canon.relation WHERE from_entity_id=1")
        .fetch_one(owner).await.unwrap();
    assert!(
        plan.to_string().contains("relation_from_entity_idx"),
        "{plan}"
    );
    let profiles: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.hero_profile WHERE key LIKE 'bulk_%'")
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(profiles, 0);
}

async fn overlapping_object_and_completion(ingest: &PgPool, owner: &PgPool) {
    let pending = pending_source(ingest, "object-order").await;
    let mut object = ingest.begin().await.unwrap();
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *object)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.run_object SET parse_status='pending' WHERE run_id=$1")
        .bind(pending.run)
        .execute(&mut *object)
        .await
        .unwrap();
    sqlx::query("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) SELECT raw_object_id,source_id,revision_id,locator_kind,'concurrent',value_sha256,extractor,extractor_version FROM canon.receipt WHERE id=$1")
        .bind(pending.receipt).execute(&mut *object).await.unwrap();
    let mut completion = ingest.begin().await.unwrap();
    let completion_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *completion)
        .await
        .unwrap();
    let run = pending.run;
    let waiting = tokio::spawn(async move {
        publish(&mut completion, &pending).await;
        completion.commit().await.unwrap();
    });
    wait_blocked(owner, completion_pid, pid).await;
    sqlx::query("UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=$1")
        .bind(run)
        .execute(&mut *object)
        .await
        .unwrap();
    object.commit().await.unwrap();
    tokio::time::timeout(Duration::from_secs(8), waiting)
        .await
        .unwrap()
        .unwrap();
    error_code(
        ingest,
        &format!("UPDATE raw.run_object SET parse_status='pending' WHERE run_id={run}"),
        "P0001",
    )
    .await;
}

const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles','kv','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://gamefiles');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'commit-a','kv','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (1,'hero.kv','commit-a',decode(repeat('01',32),'hex'),'text/plain',convert_to('synthetic-a','UTF8'));
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (1,'hero.kv',1,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'commit-a','kv','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'kv_path','field-'||n,decode(repeat('02',32),'hex'),'kv','v1' FROM generate_series(1,40) n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','baba',1),('ability','baba_1',2);
    INSERT INTO canon.stat(key,section,unit,value_type) VALUES ('health','stats','unknown-game-unit','number'),('name_en','identity',NULL,'text'),('damage','ability','damage','number');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES (1,'health',1,'hero.kv',100,3,1),(2,'damage',1,'hero.kv',25,4,1);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES (1,'name_en',1,'hero.kv','Baba',5,1);
    INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'baba','en',5,1,'hero.kv',1),(2,'baba','en',6,1,'hero.kv',1);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,1,7,1,'hero.kv',1);
    INSERT INTO canon.stat(key,section,unit,value_type,display_order) SELECT 'stat_'||n,'stats','units','number',n FROM generate_series(1,25) n;
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        SELECT 1,'stat_'||n,1,'hero.kv',n,8+n,1 FROM generate_series(1,25) n;
    COMMIT;";

#[tokio::test]
async fn real_postgres_gamefacts_receipts_publication_and_roles() {
    let scratch = Scratch::new();
    let admin = scratch.pool("postgres", "brain_test_dba").await;
    let identity = sqlx::query("SELECT current_setting('data_directory') AS directory, current_setting('listen_addresses') AS listeners, inet_server_addr()::text AS address")
        .fetch_one(&admin).await.unwrap();
    assert_eq!(
        identity.get::<String, _>("directory"),
        scratch.root.join("data").to_str().unwrap()
    );
    assert_eq!(identity.get::<String, _>("listeners"), "");
    assert!(identity.get::<Option<String>, _>("address").is_none());
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE",
    )
    .execute(&admin)
    .await
    .unwrap();
    sqlx::query("CREATE DATABASE brain_gamefacts OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    assert!(!scratch
        .cli("check-gamefacts", "brain_migrate")
        .status
        .success());
    success(scratch.cli("up-gamefacts", "brain_test_dba"));
    success(scratch.cli("up-gamefacts", "brain_migrate"));
    success(scratch.cli("up-gamefacts", "brain_test_dba"));
    success(scratch.cli("check-gamefacts", "brain_migrate"));
    assert!(!scratch.cli("up-gamefacts", "brain_ingest").status.success());
    let owner = scratch.pool("brain_gamefacts", "brain_migrate").await;
    let ingest = scratch.pool("brain_gamefacts", "brain_ingest").await;
    sqlx::raw_sql(SEED).execute(&ingest).await.unwrap();
    let current: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.current_fact")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(current, 0);
    let proof_columns: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_attribute WHERE attrelid IN ('canon.fact'::regclass,'canon.alias'::regclass,'canon.relation'::regclass) AND attname='receipt_id' AND attnotnull")
        .fetch_one(&owner).await.unwrap();
    assert_eq!(proof_columns, 3);
    error_code(&ingest, "INSERT INTO canon.receipt(id,raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,2,1,'kv_path','field',decode(repeat('02',32),'hex'),'kv','v1')", "42501").await;
    error_code(&owner, "INSERT INTO canon.receipt(id,raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,2,1,'kv_path','field',decode(repeat('02',32),'hex'),'kv','v1')", "23503").await;
    error_code(&ingest, "INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,valid_from_revision_id) VALUES (1,'health',1,'hero.kv',1,1)", "P0001").await;
    error_code(&ingest, "INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,2,1,'hero.kv',1)", "P0001").await;
    error_code(&ingest, "UPDATE raw.object SET body='' WHERE id=1", "42501").await;
    error_code(&ingest, "DELETE FROM canon.receipt WHERE id=3", "42501").await;
    error_code(
        &ingest,
        "UPDATE canon.fact SET value_num=999 WHERE id=1",
        "42501",
    )
    .await;
    error_code(
        &ingest,
        "UPDATE raw.run SET target_revision='commit-z' WHERE id=1",
        "42501",
    )
    .await;
    error_code(&ingest, "INSERT INTO raw.run(id,source_id,job_run_id,target_revision,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,1,'commit-z','kv','v1')", "42501").await;
    error_code(&owner, "INSERT INTO raw.run(id,source_id,job_run_id,target_revision,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,1,'commit-z','kv','v1')", "23505").await;
    error_code(
        &ingest,
        "INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1)",
        "P0001",
    )
    .await;
    sqlx::raw_sql(
        "BEGIN; UPDATE raw.run SET status='succeeded' WHERE id=1;
        INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
        UPDATE raw.source SET last_complete_revision='commit-a' WHERE id=1; COMMIT",
    )
    .execute(&ingest)
    .await
    .unwrap();
    let profile: Value =
        sqlx::query_scalar("SELECT to_jsonb(h) FROM serve.hero_profile h WHERE entity_id=1")
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(profile["stats"]["facts"].as_array().unwrap().len(), 20);
    assert_eq!(profile["stats"]["omitted_count"], 6);
    assert_eq!(profile["stats"]["truncated"], true);
    assert_eq!(profile["stats"]["facts"][0]["citation"], "3");
    assert_eq!(profile["stats"]["facts"][0]["source_revision"], "commit-a");
    assert!(profile["stats"]["facts"][0]["patch"].is_null());
    assert_eq!(profile["stats"]["facts"][0]["unit"], "unknown-game-unit");
    assert_eq!(profile["abilities"]["links"][0]["citation"], "7");
    assert_eq!(
        profile["abilities"]["links"][0]["identity"]["citation"],
        "2"
    );
    assert_eq!(profile["abilities"]["links"][0]["facts"][0]["value"], 25);
    let reader = scratch.pool("brain_gamefacts", "brain_readonly").await;
    let mut tool = reader.acquire().await.unwrap();
    sqlx::query("SET SESSION ROLE brain_tool")
        .execute(&mut *tool)
        .await
        .unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.hero_profile")
        .fetch_one(&mut *tool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    for sql in [
        "INSERT INTO canon.stat(key,section,value_type) VALUES ('forbidden','stats','number')",
        "UPDATE canon.fact SET valid_to_revision_id=1",
        "SELECT body FROM raw.object",
        "SELECT * FROM ops.answer_trace",
        "SELECT serve.refresh_hero_profile()",
        "CREATE TABLE serve.forbidden(id bigint)",
    ] {
        let error = sqlx::raw_sql(sql).execute(&mut *tool).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
    }
    drop(tool);
    sqlx::raw_sql("BEGIN;
        INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles','kv','v1');
        INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES (1,2,'commit-a','commit-b','kv','v1');
        INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES (1,'hero.kv','commit-b',decode(repeat('03',32),'hex'),'text/plain',convert_to('synthetic-b','UTF8'));
        INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES (2,'hero.kv',2,'upsert');
        INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,2,'commit-b','kv','v1');
        INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (2,1,2,'kv_path','health',decode(repeat('04',32),'hex'),'kv','v1');
        UPDATE canon.fact SET valid_to_revision_id=2 WHERE id=1;
        INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES (1,'health',1,'hero.kv',120,41,2);
        UPDATE raw.run SET status='partial' WHERE id=2; COMMIT")
        .execute(&ingest).await.unwrap();
    let health: String = sqlx::query_scalar(
        "SELECT value_num::text FROM serve.current_fact WHERE entity_id=1 AND stat_key='health'",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(health, "100");
    sqlx::query("SELECT serve.refresh_hero_profile()")
        .execute(&ingest)
        .await
        .unwrap();
    let unpublished_value: Value = sqlx::query_scalar(
        "SELECT stats->'facts'->0->'value' FROM serve.hero_profile WHERE entity_id=1",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(unpublished_value, 100);
    error_code(
        &ingest,
        "UPDATE raw.run SET status='succeeded' WHERE id=2",
        "P0001",
    )
    .await;
    error_code(&ingest, "INSERT INTO raw.run(id,source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,2,'commit-a','commit-c','kv','v1')", "42501").await;
    error_code(&owner, "INSERT INTO raw.run(id,source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) OVERRIDING SYSTEM VALUE VALUES (1000,1,2,'commit-a','commit-c','kv','v1')", "23505").await;
    sqlx::raw_sql(
        "BEGIN; UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=2;
        UPDATE raw.run SET status='succeeded' WHERE id=2;
        UPDATE canon.source_head SET published_revision_id=2 WHERE source_id=1;
        UPDATE raw.source SET last_complete_revision='commit-b' WHERE id=1; COMMIT",
    )
    .execute(&ingest)
    .await
    .unwrap();
    let health: String = sqlx::query_scalar(
        "SELECT value_num::text FROM serve.current_fact WHERE entity_id=1 AND stat_key='health'",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(health, "120");
    let value: Value = sqlx::query_scalar(
        "SELECT stats->'facts'->0->'value' FROM serve.hero_profile WHERE entity_id=1",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(value, 120);
    error_code(&owner, "DELETE FROM canon.receipt WHERE id=41", "23503").await;
    error_code(&owner, "DELETE FROM raw.object WHERE id=2", "23503").await;
    let service = scratch.pool("brain_gamefacts", "brain_service").await;
    sqlx::query("INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms)
        VALUES (decode(repeat('05',32),'hex'),'[\"find_entity\",\"get_hero\"]','[\"41\"]','answered','{\"input\":12,\"output\":9}',1)")
        .execute(&service).await.unwrap();
    error_code(
        &service,
        "INSERT INTO canon.stat(key,section,value_type) VALUES ('forbidden','stats','number')",
        "42501",
    )
    .await;
    error_code(&service, "INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms) VALUES (decode(repeat('05',32),'hex'),'[{\"question\":\"forbidden\"}]','[]','answered','{}',1)", "23514").await;
    error_code(&service, "SELECT id FROM ops.answer_trace", "42501").await;
    error_code(&service, "INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms) VALUES (decode(repeat('05',32),'hex'),'[]','[]','answered','{}',1) RETURNING id", "42501").await;
    error_code(&service, "INSERT INTO ops.answer_trace(id,question_sha256,tool_calls,citation_ids,status,tokens,latency_ms) OVERRIDING SYSTEM VALUE VALUES (0,decode(repeat('05',32),'hex'),'[]','[]','answered','{}',1)", "42501").await;
    let owner_store = PgStore::new(owner.clone());
    owner_store.migrate_gamefacts().await.unwrap();
    owner_store.check_gamefacts_schema().await.unwrap();
    let patches: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.patch")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(patches, 0);
    postgres_refresh_snapshot_mechanics(&owner).await;
    overlapping_publishers(&ingest, &owner).await;
    overlapping_explicit_refresh(&ingest, &owner).await;
    overlapping_completion_and_next_run(&ingest, &owner).await;
    overlapping_object_and_completion(&ingest, &owner).await;
    eligible_ability_limit(&ingest, &owner).await;
    revision_and_isolation_boundaries(&ingest, &owner).await;
    full_history_index_plans(&ingest, &owner).await;
    owner_store.check_gamefacts_schema().await.unwrap();
    service.close().await;
    reader.close().await;
    ingest.close().await;
    owner.close().await;
    admin.close().await;
    assert!(scratch.root.starts_with(Path::new("/tmp")));
}
