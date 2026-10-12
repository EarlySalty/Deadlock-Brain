use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let root = PathBuf::from(format!("/tmp/brain-gamefacts-p4.{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::create_dir(root.join("socket")).unwrap();
        let scratch = Self { root };
        let initialized = Command::new("/usr/lib/postgresql/16/bin/initdb")
            .args([
                "--auth=trust",
                "--no-locale",
                "--username=brain_test_dba",
                "--encoding=UTF8",
                "--pgdata",
            ])
            .arg(scratch.root.join("data"))
            .output()
            .unwrap();
        assert!(
            initialized.status.success(),
            "{}",
            String::from_utf8_lossy(&initialized.stderr)
        );
        let started = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("--pgdata")
            .arg(scratch.root.join("data"))
            .arg("--log")
            .arg(scratch.root.join("postgres.log"))
            .args(["--wait", "--options"])
            .arg(format!(
                "-k {} -p 55479 -h '' -c fsync=off",
                scratch.root.join("socket").display()
            ))
            .arg("start")
            .output()
            .unwrap();
        assert!(
            started.status.success(),
            "{}",
            String::from_utf8_lossy(&started.stderr)
        );
        scratch
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(self.root.join("socket").to_str().unwrap())
                    .port(55479)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(sqlx::postgres::PgSslMode::Disable),
            )
            .await
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
        if stopped.is_ok_and(|result| result.status.success()) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

struct Revision {
    source: i64,
    run: i64,
    revision: i64,
    receipt: i64,
    content_receipt: i64,
    mapping_receipt: i64,
    name: String,
    path: String,
    official: bool,
}

async fn revision(
    ingest: &PgPool,
    source: Option<i64>,
    name: &str,
    path: &str,
    body: Value,
) -> Revision {
    let official = body.get("raw_content").is_some();
    let mut tx = ingest.begin().await.unwrap();
    let source = match source {
        Some(id) => id,
        None => {
            sqlx::query_scalar("INSERT INTO raw.source(kind,locator) VALUES ($1,$2) RETURNING id")
                .bind(if official { "patchnotes" } else { "git" })
                .bind(if official {
                    "postgres://patchnotes-feed/public.patchnotes".into()
                } else {
                    format!("synthetic://git/{name}")
                })
                .fetch_one(&mut *tx)
                .await
                .unwrap()
        }
    };
    let base: Option<String> =
        sqlx::query_scalar("SELECT last_complete_revision FROM raw.source WHERE id=$1")
            .bind(source)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('p4-contract','json','v1') RETURNING id")
        .fetch_one(&mut *tx).await.unwrap();
    let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,'json','v1') RETURNING id")
        .bind(source).bind(job).bind(base).bind(name).fetch_one(&mut *tx).await.unwrap();
    let object: i64 = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,sha256(convert_to($4,'UTF8')),'application/json',convert_to($4,'UTF8')) RETURNING id")
        .bind(source).bind(path).bind(name).bind(body.to_string()).fetch_one(&mut *tx).await.unwrap();
    sqlx::query("INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES ($1,$2,$3,'upsert','succeeded')")
        .bind(run).bind(path).bind(object).execute(&mut *tx).await.unwrap();
    let revision: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,'json','v1') RETURNING id")
        .bind(source).bind(run).bind(name).fetch_one(&mut *tx).await.unwrap();
    let mut receipts = Vec::new();
    for field in if official {
        ["title", "raw_content", "title"]
    } else {
        ["health", "health", "patch_url"]
    } {
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) SELECT $1,$2,$3,'json_pointer','/'||$4,sha256(convert_to((convert_from(body,'UTF8')::json->$4)::text,'UTF8')),'json','v1' FROM raw.object WHERE id=$1 RETURNING id")
            .bind(object).bind(source).bind(revision).bind(field).fetch_one(&mut *tx).await.unwrap();
        receipts.push(receipt);
    }
    tx.commit().await.unwrap();
    Revision {
        source,
        run,
        revision,
        receipt: receipts[0],
        content_receipt: receipts[1],
        mapping_receipt: receipts[2],
        name: name.into(),
        path: path.into(),
        official,
    }
}

fn official_body(id: i64, title: &str, date: &str, content: &str) -> Value {
    json!({"id":id,"title":title,"url":format!("https://forums.playdeadlock.com/threads/{id}"),"posted_at":date,"raw_content":content})
}

fn game_body(name: &str, patch: i64) -> Value {
    json!({"health":100,"source_revision":name,"patch_url":format!("https://forums.playdeadlock.com/threads/{patch}")})
}

async fn publish(ingest: &PgPool, revision: &Revision) {
    let mut tx = ingest.begin().await.unwrap();
    sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
        .bind(revision.run)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2) ON CONFLICT(source_id) DO UPDATE SET published_revision_id=EXCLUDED.published_revision_id")
        .bind(revision.source).bind(revision.revision).execute(&mut *tx).await.unwrap();
    sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
        .bind(revision.source)
        .bind(&revision.name)
        .execute(&mut *tx)
        .await
        .unwrap();
    if revision.official {
        sqlx::query("SELECT canon.publish_patch_origin($1)")
            .bind(revision.receipt)
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    tx.commit().await.unwrap();
}

async fn stage(ingest: &PgPool, revision: &Revision) -> i64 {
    sqlx::query_scalar("SELECT canon.stage_patch_origin($1)")
        .bind(revision.receipt)
        .fetch_one(ingest)
        .await
        .unwrap()
}

async fn change(ingest: &PgPool, patch: i64, entity: i64, receipt: i64, line: &str) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.patch_change(patch_id,entity_id,stat_key,\"old\",\"new\",change_kind,line_text,receipt_id) VALUES ($1,$2,'health','100'::jsonb,'110'::jsonb,'numeric',$3,$4) RETURNING id")
        .bind(patch).bind(entity).bind(line).bind(receipt).fetch_one(ingest).await.unwrap()
}

async fn mapping(ingest: &PgPool, revision: &Revision, patch: i64) {
    sqlx::query("INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id) VALUES ($1,$2,$3,$4)")
        .bind(revision.source).bind(&revision.name).bind(patch).bind(revision.mapping_receipt).execute(ingest).await.unwrap();
}

async fn fact(ingest: &PgPool, revision: &Revision, entity: i64) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES ($1,'health',$2,$3,100,$4,$5) RETURNING id")
        .bind(entity).bind(revision.source).bind(&revision.path).bind(revision.receipt).bind(revision.revision).fetch_one(ingest).await.unwrap()
}

async fn rejected(pool: &PgPool, sql: &str, expected: &str) {
    let error = sqlx::raw_sql(sql).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some(expected),
        "{error}"
    );
}

async fn visible(pool: &PgPool, id: i64) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM serve.fact_patch_evidence WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn guarded_receipts(
    ingest: &PgPool,
    owner: &PgPool,
    official: &Revision,
    game: &Revision,
    patch: i64,
    entity: i64,
) {
    rejected(ingest, &format!("INSERT INTO canon.patch_change(patch_id,entity_id,change_kind,line_text,receipt_id) VALUES ({patch},{entity},'other','x',{})", game.receipt), "P0001").await;
    rejected(ingest, &format!("INSERT INTO canon.patch_change(patch_id,entity_id,change_kind,line_text,receipt_id) VALUES ({patch},{entity},'other','x',{})", official.receipt), "P0001").await;
    rejected(ingest, &format!("INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id) VALUES ({},'wrong',{patch},{})", game.source, game.mapping_receipt), "P0001").await;
    rejected(ingest, &format!("INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id) VALUES ({},'{}',{patch},{})", game.source, game.name, official.receipt), "P0001").await;
    rejected(ingest, &format!("INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id) VALUES ({},'{}',{patch},{})", game.source, game.name, game.receipt), "P0001").await;
    rejected(owner, &format!("INSERT INTO canon.patch_origin(source_id,external_id,patch_id,receipt_id) VALUES ({},'wrong',{patch},{})", official.source, official.receipt), "P0001").await;
    let false_receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) SELECT raw_object_id,source_id,revision_id,locator_kind,locator,decode(repeat('00',32),'hex'),extractor,extractor_version FROM canon.receipt WHERE id=$1 RETURNING id")
        .bind(official.receipt).fetch_one(ingest).await.unwrap();
    rejected(
        ingest,
        &format!("SELECT canon.stage_patch_origin({false_receipt})"),
        "P0001",
    )
    .await;
    rejected(
        ingest,
        &format!("SELECT canon.stage_patch_origin({})", game.receipt),
        "P0001",
    )
    .await;
    rejected(
        ingest,
        &format!("SELECT canon.publish_patch_origin({})", official.receipt),
        "P0001",
    )
    .await;
    for (column, value) in [
        ("line_text", "repeat('x',513)"),
        ("\"old\"", "'{\"nested\":{\"x\":1}}'::jsonb"),
        ("\"new\"", "to_jsonb(repeat('x',1025))"),
    ] {
        let line = if column == "line_text" {
            value
        } else {
            "'bounded'"
        };
        let old = if column == "\"old\"" { value } else { "NULL" };
        let new = if column == "\"new\"" { value } else { "NULL" };
        rejected(ingest, &format!("INSERT INTO canon.patch_change(patch_id,entity_id,\"old\",\"new\",change_kind,line_text,receipt_id) VALUES ({patch},{entity},{old},{new},'other',{line},{})", official.content_receipt), "23514").await;
    }
    for sql in [
        "UPDATE canon.patch SET label='forged'",
        "DELETE FROM canon.patch_origin",
        "UPDATE canon.patch_origin SET receipt_id=1",
        "INSERT INTO canon.patch(label,released_at,raw_object_id) VALUES ('x','2026-10-01Z',1)",
        "UPDATE canon.patch_change SET line_text='x'",
        "UPDATE canon.source_revision_patch SET patch_id=1",
        "UPDATE canon.fact SET valid_from_patch=1",
        "SELECT canon.guard_patch_p4_change()",
        "SELECT canon.patch_p4_anchor(1)",
    ] {
        rejected(ingest, sql, "42501").await;
    }
    rejected(
        owner,
        "UPDATE canon.patch_change SET line_text='x'",
        "P0001",
    )
    .await;
    let triggers: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_trigger WHERE tgfoid='ops.serialize_gamefacts_write()'::regprocedure").fetch_one(owner).await.unwrap();
    assert_eq!(triggers, 14);
    let mut tx = ingest.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *tx)
        .await
        .unwrap();
    let error = sqlx::query("SELECT canon.stage_patch_origin($1)")
        .bind(official.receipt)
        .execute(&mut *tx)
        .await
        .unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some("0A000")
    );
    tx.rollback().await.unwrap();
}

async fn bounded_index_plan(owner: &PgPool, patch: i64, entity: i64) {
    sqlx::query("ANALYZE canon.patch_change")
        .execute(owner)
        .await
        .unwrap();
    let plan: Value = sqlx::query_scalar("EXPLAIN (ANALYZE,FORMAT JSON) SELECT id,patch_id,\"old\",\"new\",line_text,receipt_id FROM serve.published_patch_change WHERE entity_id=$1 AND patch_id=$2 ORDER BY patch_id DESC,id DESC LIMIT 20")
        .bind(entity).bind(patch).fetch_one(owner).await.unwrap();
    assert_eq!(plan[0]["Plan"]["Node Type"], "Limit");
    assert_eq!(plan[0]["Plan"]["Actual Rows"], 20);
    assert!(
        plan.to_string().contains("patch_change_entity_patch_idx"),
        "{plan}"
    );
    assert!(plan[0]["Execution Time"].as_f64().unwrap() < 5.0, "{plan}");
    println!(
        "indexed_limit_20_execution_ms={}",
        plan[0]["Execution Time"]
    );
    let rows = sqlx::query("SELECT id FROM serve.published_patch_change WHERE entity_id=$1 AND patch_id=$2 ORDER BY patch_id DESC,id DESC LIMIT 20").bind(entity).bind(patch).fetch_all(owner).await.unwrap();
    assert_eq!(rows.len(), 20);
    assert!(rows
        .windows(2)
        .all(|rows| rows[0].get::<i64, _>("id") > rows[1].get::<i64, _>("id")));
}

#[tokio::test]
async fn standalone_p4_receipts_corrections_snapshots_and_bounded_history() {
    let scratch = Scratch::new();
    let admin = scratch.pool("postgres", "brain_test_dba").await;
    let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(directory, scratch.root.join("data").to_str().unwrap());
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert!(address.is_none());
    sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
        GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE")
        .execute(&admin).await.unwrap();
    sqlx::query("CREATE DATABASE brain_p4 OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let owner = scratch.pool("brain_p4", "brain_migrate").await;
    let ingest = scratch.pool("brain_p4", "brain_ingest").await;
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .execute(&owner)
    .await
    .unwrap();
    let legacy = revision(&ingest, None, "legacy", "hero.json", game_body("legacy", 1)).await;
    let entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','baba',$1) RETURNING id").bind(legacy.receipt).fetch_one(&ingest).await.unwrap();
    sqlx::query(
        "INSERT INTO canon.stat(key,section,value_type) VALUES ('health','stats','number')",
    )
    .execute(&ingest)
    .await
    .unwrap();
    let legacy_fact = fact(&ingest, &legacy, entity).await;
    publish(&ingest, &legacy).await;
    let assertion_guard: String =
        sqlx::query_scalar("SELECT pg_get_functiondef('canon.guard_assertion()'::regprocedure)")
            .fetch_one(&owner)
            .await
            .unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-p4-patches-v1.sql"
    ))
    .execute(&owner)
    .await
    .unwrap();
    let unchanged_guard: String =
        sqlx::query_scalar("SELECT pg_get_functiondef('canon.guard_assertion()'::regprocedure)")
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(assertion_guard, unchanged_guard);
    let original_content = std::iter::once("Health increased".to_owned())
        .chain((1..=6000).map(|n| format!("Other health {n}")))
        .collect::<Vec<_>>()
        .join("\n");
    let official = revision(
        &ingest,
        None,
        "latest",
        "1",
        official_body(1, "Latest", "2026-10-12T00:00:00Z", &original_content),
    )
    .await;
    let latest = stage(&ingest, &official).await;
    let original_claim = change(
        &ingest,
        latest,
        entity,
        official.content_receipt,
        "Health increased",
    )
    .await;
    let other_entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','other',$1) RETURNING id")
        .bind(official.receipt).fetch_one(&ingest).await.unwrap();
    sqlx::query("INSERT INTO canon.patch_change(patch_id,entity_id,change_kind,line_text,receipt_id) SELECT $1,$2,'numeric','Other health '||n,$3 FROM generate_series(1,6000) n")
        .bind(latest).bind(other_entity).bind(official.content_receipt).execute(&ingest).await.unwrap();
    assert!(
        sqlx::query_scalar::<_, i64>("SELECT id FROM serve.current_patch")
            .fetch_optional(&owner)
            .await
            .unwrap()
            .is_none()
    );
    guarded_receipts(&ingest, &owner, &official, &legacy, latest, entity).await;
    publish(&ingest, &official).await;
    let older = revision(
        &ingest,
        Some(official.source),
        "older",
        "2",
        official_body(2, "Latest", "2026-09-01T00:00:00Z", "Other change"),
    )
    .await;
    let older_patch = stage(&ingest, &older).await;
    assert!(older_patch > latest);
    rejected(&ingest, &format!("INSERT INTO canon.patch_change(patch_id,entity_id,change_kind,line_text,receipt_id) VALUES ({latest},{entity},'other','x',{})", older.content_receipt), "P0001").await;
    publish(&ingest, &older).await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT id FROM serve.current_patch")
            .fetch_one(&owner)
            .await
            .unwrap(),
        latest
    );
    mapping(&ingest, &legacy, latest).await;
    rejected(
        &owner,
        "UPDATE canon.source_revision_patch SET patch_id=1",
        "P0001",
    )
    .await;
    assert!(sqlx::query_scalar::<_, Option<i64>>(
        "SELECT valid_from_patch FROM canon.fact WHERE id=$1"
    )
    .bind(legacy_fact)
    .fetch_one(&owner)
    .await
    .unwrap()
    .is_none());
    assert_eq!(visible(&owner, legacy_fact).await, 1);
    let unknown_snapshot: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM serve.fact_patch_snapshot WHERE id=$1 AND snapshot_patch_id=$2",
    )
    .bind(legacy_fact)
    .bind(older_patch)
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(unknown_snapshot, 0);
    let known_snapshot: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM serve.fact_patch_snapshot WHERE id=$1 AND snapshot_patch_id=$2",
    )
    .bind(legacy_fact)
    .bind(latest)
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(known_snapshot, 1);
    let content = (1..=6000)
        .map(|n| format!("Health corrected {n}"))
        .collect::<Vec<_>>()
        .join("\n");
    let corrected = revision(
        &ingest,
        Some(official.source),
        "corrected",
        "1",
        official_body(1, "Corrected", "2026-10-11T00:00:00Z", &content),
    )
    .await;
    assert_eq!(stage(&ingest, &corrected).await, latest);
    sqlx::query("INSERT INTO canon.patch_change(patch_id,entity_id,stat_key,\"old\",\"new\",change_kind,line_text,receipt_id) SELECT $1,$2,'health','100'::jsonb,'105'::jsonb,'numeric','Health corrected '||n,$3 FROM generate_series(1,6000) n")
        .bind(latest).bind(entity).bind(corrected.content_receipt).execute(&ingest).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.published_patch_change WHERE patch_id=$1"
        )
        .bind(latest)
        .fetch_one(&owner)
        .await
        .unwrap(),
        6001
    );
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT label FROM canon.patch WHERE id=$1")
            .bind(latest)
            .fetch_one(&owner)
            .await
            .unwrap(),
        "Latest"
    );
    publish(&ingest, &corrected).await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.patch_change WHERE id=$1")
            .bind(original_claim)
            .fetch_one(&owner)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.published_patch_change WHERE id=$1"
        )
        .bind(original_claim)
        .fetch_one(&owner)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.published_patch_change WHERE patch_id=$1"
        )
        .bind(latest)
        .fetch_one(&owner)
        .await
        .unwrap(),
        6000
    );
    rejected(
        &ingest,
        &format!("SELECT canon.publish_patch_origin({})", official.receipt),
        "P0001",
    )
    .await;
    rejected(
        &owner,
        &format!("UPDATE canon.patch SET label='forged' WHERE id={latest}"),
        "P0001",
    )
    .await;
    bounded_index_plan(&owner, latest, entity).await;
    let unmapped = revision(
        &ingest,
        None,
        "unknown",
        "hero.json",
        game_body("unknown", 999),
    )
    .await;
    let unmapped_fact = fact(&ingest, &unmapped, entity).await;
    rejected(&ingest, &format!("INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id) VALUES ({},'{}',{latest},{})", unmapped.source, unmapped.name, unmapped.mapping_receipt), "P0001").await;
    publish(&ingest, &unmapped).await;
    let endpoints: (Option<i64>, Option<i64>) =
        sqlx::query_as("SELECT valid_from_patch,valid_to_patch FROM canon.fact WHERE id=$1")
            .bind(unmapped_fact)
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(endpoints, (None, None));
    assert_eq!(visible(&owner, unmapped_fact).await, 0);
    let future = revision(
        &ingest,
        Some(legacy.source),
        "future",
        "hero.json",
        game_body("future", 1),
    )
    .await;
    mapping(&ingest, &future, latest).await;
    sqlx::query("UPDATE canon.fact SET valid_to_revision_id=$1 WHERE id=$2")
        .bind(future.revision)
        .bind(legacy_fact)
        .execute(&ingest)
        .await
        .unwrap();
    let future_fact = fact(&ingest, &future, entity).await;
    let endpoints: (Option<i64>, Option<i64>) =
        sqlx::query_as("SELECT valid_from_patch,valid_to_patch FROM canon.fact WHERE id=$1")
            .bind(legacy_fact)
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(endpoints, (None, Some(latest)));
    assert_eq!(visible(&owner, legacy_fact).await, 0);
    assert_eq!(visible(&owner, future_fact).await, 0);
    rejected(&ingest, &format!("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES ({entity},'health',{},'hero.json','wrong',{},{})", future.source, future.receipt, future.revision), "P0001").await;
    publish(&ingest, &future).await;
    assert_eq!(
        sqlx::query_scalar::<_, Option<i64>>("SELECT valid_from_patch FROM canon.fact WHERE id=$1")
            .bind(future_fact)
            .fetch_one(&owner)
            .await
            .unwrap(),
        Some(latest)
    );
    assert_eq!(visible(&owner, legacy_fact).await, 1);
    assert_eq!(visible(&owner, future_fact).await, 1);
    let snapshot_facts: Vec<i64> = sqlx::query_scalar("SELECT id FROM serve.fact_patch_snapshot WHERE source_id=$1 AND snapshot_patch_id=$2 ORDER BY id")
        .bind(legacy.source).bind(latest).fetch_all(&owner).await.unwrap();
    assert_eq!(snapshot_facts, vec![future_fact]);
    let closing = revision(
        &ingest,
        Some(legacy.source),
        "unmapped-close",
        "hero.json",
        game_body("unmapped-close", 999),
    )
    .await;
    sqlx::query("UPDATE canon.fact SET valid_to_revision_id=$1 WHERE id=$2")
        .bind(closing.revision)
        .bind(future_fact)
        .execute(&ingest)
        .await
        .unwrap();
    rejected(
        &ingest,
        &format!(
            "UPDATE canon.fact SET valid_to_revision_id={} WHERE id={legacy_fact}",
            closing.revision
        ),
        "P0001",
    )
    .await;
    publish(&ingest, &closing).await;
    assert_eq!(visible(&owner, future_fact).await, 0);
    assert!(sqlx::query_scalar::<_, Option<i64>>(
        "SELECT valid_to_patch FROM canon.fact WHERE id=$1"
    )
    .bind(future_fact)
    .fetch_one(&owner)
    .await
    .unwrap()
    .is_none());
    let reader = scratch.pool("brain_p4", "brain_readonly").await;
    let mut tool = reader.acquire().await.unwrap();
    sqlx::raw_sql(
        "SET SESSION ROLE brain_tool; BEGIN READ ONLY; SET LOCAL statement_timeout='500ms'",
    )
    .execute(&mut *tool)
    .await
    .unwrap();
    let rows = sqlx::query("SELECT id FROM serve.published_patch_change WHERE entity_id=$1 AND patch_id=$2 ORDER BY patch_id DESC,id DESC LIMIT 20").bind(entity).bind(latest).fetch_all(&mut *tool).await.unwrap();
    assert_eq!(rows.len(), 20);
    sqlx::query("COMMIT").execute(&mut *tool).await.unwrap();
    for sql in [
        "SELECT * FROM raw.object",
        "SELECT * FROM ops.job_run",
        "SELECT * FROM canon.patch_change",
        "SELECT * FROM canon.patch_origin",
        "SELECT * FROM canon.source_revision_patch",
        "UPDATE canon.patch_change SET line_text='bad'",
        "SELECT canon.publish_patch_origin(1)",
    ] {
        let error = sqlx::raw_sql(sql).execute(&mut *tool).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
    }
    let writable: bool = sqlx::query_scalar("SELECT has_table_privilege(current_user,'canon.patch_change','INSERT,UPDATE,DELETE') OR has_table_privilege(current_user,'canon.patch_origin','INSERT,UPDATE,DELETE') OR has_table_privilege(current_user,'canon.source_revision_patch','INSERT,UPDATE,DELETE')").fetch_one(&mut *tool).await.unwrap();
    assert!(!writable);
    drop(tool);
    let service = scratch.pool("brain_p4", "brain_service").await;
    rejected(&service, "SELECT * FROM canon.patch_change", "42501").await;
    let mut invalid_body = official_body(
        1,
        "Undated correction",
        "2026-10-12T00:00:00Z",
        "Unprovable correction",
    );
    invalid_body["posted_at"] = Value::Null;
    let mut invalid = revision(&ingest, Some(official.source), "undated", "1", invalid_body).await;
    rejected(
        &ingest,
        &format!("SELECT canon.stage_patch_origin({})", invalid.receipt),
        "P0001",
    )
    .await;
    invalid.official = false;
    publish(&ingest, &invalid).await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.published_patch_change WHERE patch_id=$1"
        )
        .bind(latest)
        .fetch_one(&owner)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.patch_change WHERE patch_id=$1")
            .bind(latest)
            .fetch_one(&owner)
            .await
            .unwrap(),
        12001
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT id FROM serve.current_patch")
            .fetch_one(&owner)
            .await
            .unwrap(),
        older_patch
    );
    service.close().await;
    reader.close().await;
    ingest.close().await;
    owner.close().await;
    admin.close().await;
}
