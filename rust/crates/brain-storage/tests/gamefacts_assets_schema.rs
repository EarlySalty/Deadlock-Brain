use brain_contracts::{ToolName, ToolResult};
use brain_storage::PgStore;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let root = PathBuf::from(format!(
            "/tmp/brain-assets-schema.{}.{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir(&root).unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::create_dir(root.join("socket")).unwrap();
        for (binary, args) in [
            (
                "initdb",
                vec!["--auth=trust", "--no-locale", "--username=brain_test_dba"],
            ),
            ("pg_ctl", vec!["--wait", "start"]),
        ] {
            let mut command = Command::new(format!("/usr/lib/postgresql/16/bin/{binary}"));
            command.arg("--pgdata").arg(root.join("data")).args(args);
            if binary == "pg_ctl" {
                command.arg("--log").arg(root.join("postgres.log"));
                command.arg("--options").arg(format!(
                    "-k {} -p 55468 -h '' -c fsync=off",
                    root.join("socket").display()
                ));
            }
            let output = command.output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Self { root }
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(self.root.join("socket").to_str().unwrap())
                    .port(55468)
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
        if stopped.is_ok_and(|output| output.status.success()) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

const P1: &str = include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql");
const P2: &str =
    include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p2-assets-v1.sql");
const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles','hero-ability-atoms','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://gamefiles');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'game-a','hero-ability-atoms','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (1,'hero.kv','game-a',decode(repeat('01',32),'hex'),'text/plain','synthetic');
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (1,'hero.kv',1,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'game-a','hero-ability-atoms','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'kv_path','field-'||n,decode(repeat('02',32),'hex'),'hero-ability-atoms','v1' FROM generate_series(1,10) n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','baba',1),('ability','baba_1',2),('ability','baba_2',3);
    INSERT INTO canon.stat(key,section,value_type) VALUES ('damage','ability','number'),('name_en','identity','text'),('levels','ability','json');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES (2,'damage',1,'hero.kv',25,4,1);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES (2,'name_en',1,'hero.kv','Game ability',5,1);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_json,receipt_id,valid_from_revision_id) VALUES (2,'levels',1,'hero.kv','[1,2,3]',6,1);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,1,7,1,'hero.kv',1);
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision='game-a' WHERE id=1;
    COMMIT";

const ASSETS: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-assets','brain-assets','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('assets','synthetic://assets');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (2,2,'assets-a','brain-assets','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (2,'abilities.json','assets-a',decode(repeat('03',32),'hex'),'application/json','synthetic');
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (2,'abilities.json',2,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (2,2,'assets-a','brain-assets','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 2,2,2,'json_pointer','/'||n,decode(repeat('04',32),'hex'),'brain-assets','v1' FROM generate_series(1,10) n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('ability','hidden_ability',16);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id) VALUES (2,'damage',2,'abilities.json',999,11,2);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES (2,'name_en',2,'abilities.json','Assets Fähigkeit',12,2);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_json,receipt_id,valid_from_revision_id) VALUES (2,'levels',2,'abilities.json','[99]',13,2);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,receipt_id,valid_from_revision_id) VALUES (2,'damage',2,'abilities.json','{\"level\":2}',80,14,2);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',3,1,15,2,'abilities.json',2);
    COMMIT";

async fn snapshot(owner: &PgPool) -> Value {
    sqlx::query_scalar(
        "SELECT jsonb_build_object(
        'facts',(SELECT jsonb_agg(to_jsonb(f) ORDER BY id) FROM canon.fact f),
        'relations',(SELECT jsonb_agg(to_jsonb(r) ORDER BY id) FROM canon.relation r),
        'receipts',(SELECT jsonb_agg(to_jsonb(p) ORDER BY id) FROM canon.receipt p),
        'entities',(SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e),
        'heads',(SELECT jsonb_agg(to_jsonb(h) ORDER BY source_id) FROM canon.source_head h))",
    )
    .fetch_one(owner)
    .await
    .unwrap()
}

async fn rejected(pool: &PgPool, sql: &str, code: &str) {
    let error = sqlx::raw_sql(sql).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some(code)
    );
}

async fn relation_contracts(ingest: &PgPool) {
    for slot in ["NULL", "0", "5"] {
        rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,{slot},15,2,'abilities.json',2)"), "23514").await;
    }
    for (from, to) in [(2, 1), (1, 1), (2, 3)] {
        rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ({from},'has_ability',{to},2,15,2,'abilities.json',2)"), "P0001").await;
    }
    for kind in ["component_of", "upgrades_to"] {
        for (from, to) in [(1, 2), (2, 3), (1, 1)] {
            rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ({from},'{kind}',{to},15,2,'abilities.json',2)"), "P0001").await;
        }
    }
    rejected(ingest, "INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,2,2,'abilities.json',2)", "P0001").await;
    rejected(ingest, "INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,2,7,2,'abilities.json',2)", "P0001").await;
    rejected(ingest, "INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,2,99999,2,'abilities.json',2)", "P0001").await;
    rejected(ingest, "INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES (1,'has_ability',2,1,15,2,'abilities.json',2)", "23505").await;
    rejected(
        ingest,
        "UPDATE canon.fact SET value_num=200 WHERE id=1",
        "42501",
    )
    .await;
    rejected(ingest, "DELETE FROM canon.receipt WHERE id=1", "42501").await;
    rejected(ingest, "INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id) VALUES (3,'damage',2,'abilities.json','wrong type',15,2)", "P0001").await;
    item_relation_contracts(ingest).await;
}

async fn item_relation_contracts(ingest: &PgPool) {
    let items: Vec<i64> = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('item','component',16),('item','upgrade',17) RETURNING id")
        .fetch_all(ingest).await.unwrap();
    let from = items[0];
    let to = items[1];
    for kind in ["component_of", "upgrades_to"] {
        let edge = format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ({from},'{kind}',{to},NULL,15,2,'abilities.json',2)");
        sqlx::raw_sql(&edge).execute(ingest).await.unwrap();
        rejected(ingest, &edge, "23505").await;
        rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ({from},'{kind}',{to},1,15,2,'abilities.json',2)"), "23514").await;
        for endpoint in [1, 2, from] {
            rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,receipt_id,source_id,source_path,valid_from_revision_id) VALUES ({from},'{kind}',{endpoint},15,2,'abilities.json',2)"), "P0001").await;
        }
        rejected(ingest, &format!("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,source_id,source_path,valid_from_revision_id) VALUES ({to},'{kind}',{from},2,'abilities.json',2)"), "P0001").await;
    }
}

async fn serving_contracts(owner: &PgPool, ingest: &PgPool) {
    let hidden: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.ability_card WHERE key='hidden_ability'")
            .fetch_one(owner)
            .await
            .unwrap();
    assert_eq!(hidden, 0);
    let before: String = sqlx::query_scalar(
        "SELECT value_text FROM serve.selected_fact_p2 WHERE entity_id=2 AND stat_key='name_en'",
    )
    .fetch_one(owner)
    .await
    .unwrap();
    assert_eq!(before, "Game ability");
    sqlx::raw_sql(
        "BEGIN; UPDATE raw.run SET status='succeeded' WHERE id=2;
        INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (2,2);
        UPDATE raw.source SET last_complete_revision='assets-a' WHERE id=2; COMMIT",
    )
    .execute(ingest)
    .await
    .unwrap();
    let selected: Vec<Value> = sqlx::query_scalar("SELECT serve.atomic_fact(entity_id,stat_key,value_num,value_text,value_json,unit,condition,receipt_id,source_revision) FROM serve.selected_fact_p2 WHERE entity_id=2 ORDER BY stat_key,condition")
        .fetch_all(owner).await.unwrap();
    assert_eq!(selected.len(), 4);
    assert_eq!(selected[0]["value"], 25);
    assert_eq!(selected[0]["citation"], "4");
    assert_eq!(selected[1]["value"], 80);
    assert_eq!(selected[1]["condition"], json!({"level": 2}));
    assert_eq!(selected[1]["citation"], "14");
    assert_eq!(selected[2]["value"], json!([1, 2, 3]));
    assert_eq!(selected[2]["citation"], "6");
    assert_eq!(selected[3]["value"], "Assets Fähigkeit");
    assert_eq!(selected[3]["citation"], "12");
    let retained: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.fact WHERE entity_id=2")
        .fetch_one(owner)
        .await
        .unwrap();
    assert_eq!(retained, 7);
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=2")
        .fetch_one(owner)
        .await
        .unwrap();
    assert_eq!(card["identity"]["citation"], "2");
    assert_eq!(card["facts"]["facts"], Value::Array(selected));
    assert_eq!(card["links"]["links"].as_array().unwrap().len(), 1);
    assert_eq!(card["links"]["links"][0]["citation"], "7");
    assert_eq!(card["links"]["links"][0]["identity"]["citation"], "1");
    assert_eq!(card["links"]["links"][0]["condition"], json!({}));
    assert!(card["links"]["links"][0]["patch"].is_null());
    let other: Value = sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=3")
        .fetch_one(owner)
        .await
        .unwrap();
    assert_eq!(other["links"]["links"], json!([]));
    let published: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.ability_card WHERE key='hidden_ability'")
            .fetch_one(owner)
            .await
            .unwrap();
    let item_cards: Vec<Value> =
        sqlx::query_scalar("SELECT card FROM serve.item_card ORDER BY key")
            .fetch_all(owner)
            .await
            .unwrap();
    assert_eq!(item_cards.len(), 2);
    for item in item_cards {
        assert_eq!(item["links"]["links"].as_array().unwrap().len(), 2);
        for link in item["links"]["links"].as_array().unwrap() {
            assert_eq!(link["citation"], "15");
            assert!(link["identity"]["citation"].is_string());
            assert!(link["patch"].is_null());
        }
    }
    assert_eq!(published, 1);
    let same_shape: bool = sqlx::query_scalar("SELECT NOT EXISTS (
        SELECT attname,atttypid,attnum FROM pg_attribute WHERE attrelid='serve.current_fact'::regclass AND attnum>0
        EXCEPT SELECT attname,atttypid,attnum FROM pg_attribute WHERE attrelid='serve.selected_fact_p2'::regclass AND attnum>0)")
        .fetch_one(owner).await.unwrap();
    assert!(same_shape);
    let forbidden_lineage: i64 = sqlx::query_scalar(
        "WITH RECURSIVE lineage(oid) AS (
        SELECT unnest(ARRAY['serve.item_card'::regclass,'serve.ability_card'::regclass]::oid[])
        UNION SELECT d.refobjid FROM lineage l JOIN pg_rewrite rw ON rw.ev_class=l.oid
        JOIN pg_depend d ON d.classid='pg_rewrite'::regclass AND d.objid=rw.oid
            AND d.refclassid='pg_class'::regclass AND d.refobjid<>l.oid)
        SELECT count(*) FROM lineage l JOIN pg_class c ON c.oid=l.oid
        JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname NOT IN ('canon','serve')",
    )
    .fetch_one(owner)
    .await
    .unwrap();
    assert_eq!(forbidden_lineage, 0);
}

async fn bounded_cards(owner: &PgPool, ingest: &PgPool) {
    sqlx::raw_sql("BEGIN;
        INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles','hero-ability-atoms','v1');
        INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) VALUES (1,3,'game-a','game-b','hero-ability-atoms','v1');
        INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (3,'hero.kv',1,'upsert','succeeded');
        INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,3,'game-b','hero-ability-atoms','v1');
        INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (1,1,3,'kv_path','bulk',decode(repeat('06',32),'hex'),'hero-ability-atoms','v1');
        INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_json,receipt_id,valid_from_revision_id)
            SELECT 3,'levels',1,'hero.kv',jsonb_build_object('ordinal',n,'large',repeat('c',700)),to_jsonb(repeat('v',700)),21,3 FROM generate_series(1,30) n;
        UPDATE raw.run SET status='succeeded' WHERE id=3;
        UPDATE canon.source_head SET published_revision_id=3 WHERE source_id=1;
        UPDATE raw.source SET last_complete_revision='game-b' WHERE id=1; COMMIT")
        .execute(ingest).await.unwrap();
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=3")
        .fetch_one(owner)
        .await
        .unwrap();
    let returned = card["facts"]["facts"].as_array().unwrap().len();
    assert!(returned > 0 && returned < 30);
    assert_eq!(card["facts"]["omitted_count"], 30 - returned);
    assert_eq!(card["facts"]["truncated"], true);
    assert!(serde_json::to_vec(&card).unwrap().len() < 16 * 1024);
    let size: i32 =
        sqlx::query_scalar("SELECT max(octet_length(card::text)) FROM serve.ability_card")
            .fetch_one(owner)
            .await
            .unwrap();
    assert!(size < 16 * 1024);
}

async fn serialized_publication_refresh(owner: &PgPool, ingest: &PgPool) {
    sqlx::raw_sql("BEGIN;
        INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-assets','brain-assets','v1');
        INSERT INTO raw.source(kind,locator) VALUES ('assets','synthetic://concurrent-assets');
        INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (3,4,'concurrent-a','brain-assets','v1');
        INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
            VALUES (3,'concurrent.json','concurrent-a',decode(repeat('07',32),'hex'),'application/json','synthetic');
        INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (4,'concurrent.json',3,'upsert','succeeded');
        INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (3,4,'concurrent-a','brain-assets','v1');
        INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (3,3,4,'json_pointer','/identity',decode(repeat('08',32),'hex'),'brain-assets','v1');
        INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('ability','concurrent_ability',22);
        COMMIT")
        .execute(ingest).await.unwrap();
    let mut publication = ingest.begin().await.unwrap();
    let publisher: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *publication)
        .await
        .unwrap();
    sqlx::raw_sql(
        "UPDATE raw.run SET status='succeeded' WHERE id=4;
        INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (3,4);
        UPDATE raw.source SET last_complete_revision='concurrent-a' WHERE id=3",
    )
    .execute(&mut *publication)
    .await
    .unwrap();
    let mut refresh = ingest.acquire().await.unwrap();
    let refresher: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *refresh)
        .await
        .unwrap();
    let waiting = tokio::spawn(async move {
        sqlx::query("SELECT serve.refresh_assets_p2()")
            .execute(&mut *refresh)
            .await
            .unwrap();
    });
    tokio::time::timeout(std::time::Duration::from_secs(8), async {
        loop {
            let blocked: bool = sqlx::query_scalar("SELECT $2=ANY(pg_blocking_pids($1))")
                .bind(refresher)
                .bind(publisher)
                .fetch_one(owner)
                .await
                .unwrap();
            if blocked {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the refresh must wait for the shared publication lock");
    publication.commit().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(8), waiting)
        .await
        .unwrap()
        .unwrap();
    let published: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM serve.ability_card WHERE key='concurrent_ability'",
    )
    .fetch_one(owner)
    .await
    .unwrap();
    assert_eq!(published, 1);
}

#[tokio::test]
async fn p1_upgrade_priority_receipts_bounds_and_roles_on_real_postgres() {
    let scratch = Scratch::new();
    let admin = scratch.pool("postgres", "brain_test_dba").await;
    sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
        GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE")
        .execute(&admin).await.unwrap();
    sqlx::query("CREATE DATABASE brain_assets OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    let owner = scratch.pool("brain_assets", "brain_migrate").await;
    let ingest = scratch.pool("brain_assets", "brain_ingest").await;
    sqlx::raw_sql(P1).execute(&owner).await.unwrap();
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES (1)")
        .execute(&owner)
        .await
        .unwrap();
    sqlx::raw_sql(SEED).execute(&ingest).await.unwrap();
    let before = snapshot(&owner).await;
    let original_index: String =
        sqlx::query_scalar("SELECT pg_get_indexdef('canon.relation_active_slot_idx'::regclass)")
            .fetch_one(&owner)
            .await
            .unwrap();
    let store = PgStore::new(owner.clone());
    assert!(store.check_gamefacts_schema().await.is_err());
    store.migrate_gamefacts().await.unwrap();
    store.check_gamefacts_schema().await.unwrap();
    assert_eq!(snapshot(&owner).await, before);
    store.migrate_gamefacts().await.unwrap();
    assert_eq!(snapshot(&owner).await, before);
    let upgraded_index: String =
        sqlx::query_scalar("SELECT pg_get_indexdef('canon.relation_active_slot_idx'::regclass)")
            .fetch_one(&owner)
            .await
            .unwrap();
    assert_eq!(original_index, upgraded_index);
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=2")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(card["facts"]["facts"].as_array().unwrap().len(), 3);
    sqlx::raw_sql(ASSETS).execute(&ingest).await.unwrap();
    relation_contracts(&ingest).await;
    serving_contracts(&owner, &ingest).await;
    bounded_cards(&owner, &ingest).await;
    serialized_publication_refresh(&owner, &ingest).await;
    let reader = scratch.pool("brain_assets", "brain_readonly").await;
    let mut tool = reader.acquire().await.unwrap();
    sqlx::query("SET ROLE brain_tool")
        .execute(&mut *tool)
        .await
        .unwrap();
    sqlx::query("SELECT entity_id,key,card FROM serve.item_card UNION ALL SELECT entity_id,key,card FROM serve.ability_card")
        .fetch_all(&mut *tool).await.unwrap();
    for sql in [
        "SELECT serve.refresh_assets_p2()",
        "SELECT body FROM raw.object",
        "SELECT * FROM ops.job_run",
        "CREATE TABLE serve.forbidden(id integer)",
    ] {
        let error = sqlx::raw_sql(sql).execute(&mut *tool).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
    }
    drop(tool);
    sqlx::query("SELECT serve.refresh_assets_p2()")
        .execute(&ingest)
        .await
        .unwrap();
    for isolation in ["REPEATABLE READ", "SERIALIZABLE"] {
        let mut tx = ingest.begin().await.unwrap();
        sqlx::raw_sql(&format!("SET TRANSACTION ISOLATION LEVEL {isolation}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        let error = sqlx::query("SELECT serve.refresh_assets_p2()")
            .execute(&mut *tx)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("0A000")
        );
        tx.rollback().await.unwrap();
    }
    store.check_gamefacts_schema().await.unwrap();
    let versions: Vec<i32> =
        sqlx::query_scalar("SELECT version FROM ops.gamefacts_schema_version ORDER BY version")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(versions, [1, 3]);
    reader.close().await;
    ingest.close().await;
    owner.close().await;
    admin.close().await;
}

async fn public_fixture_database(scratch: &Scratch) -> PgPool {
    let admin = scratch.pool("postgres", "brain_test_dba").await;
    sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
        CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS")
        .execute(&admin).await.unwrap();
    sqlx::query("CREATE DATABASE brain_public OWNER brain_migrate")
        .execute(&admin)
        .await
        .unwrap();
    admin.close().await;
    let owner = scratch.pool("brain_public", "brain_migrate").await;
    sqlx::raw_sql(P1).execute(&owner).await.unwrap();
    sqlx::raw_sql(P2).execute(&owner).await.unwrap();
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES (1),(3)")
        .execute(&owner)
        .await
        .unwrap();
    sqlx::raw_sql("BEGIN;
        INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-assets','brain-assets','v1');
        INSERT INTO raw.source(kind,locator) VALUES ('assets','public-fixture://client6774');
        INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version)
            VALUES (1,1,'31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596','brain-assets','v1');
        INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
            VALUES (1,'items-english.json','31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596',decode(repeat('01',32),'hex'),'application/json','projection fixture, not lexical receipt audit'),
                (1,'heroes-english.json','31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596',decode(repeat('03',32),'hex'),'application/json','projection fixture, not lexical receipt audit');
        INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
            VALUES (1,'items-english.json',1,'upsert','succeeded'), (1,'heroes-english.json',2,'upsert','succeeded');
        INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
            VALUES (1,1,'31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596','brain-assets','v1');
        SELECT setval('canon.receipt_id_seq',100000,false);
        SELECT setval('canon.entity_id_seq',1000,false); COMMIT")
        .execute(&owner).await.unwrap();
    owner
}

fn public_records() -> Vec<Value> {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/public_assets_card_6774.json")).unwrap();
    assert_eq!(fixture["client_version"], 6774);
    let records = fixture["records"].as_array().unwrap().clone();
    for record in &records {
        let prefix = format!("/{}", record["source_position"].as_u64().unwrap());
        assert_eq!(record["key"], record["source_record"]["class_name"]);
        for fact in record["facts"].as_array().unwrap() {
            let pointer = fact["locator"]
                .as_str()
                .unwrap()
                .strip_prefix(&prefix)
                .unwrap();
            assert_eq!(
                record["source_record"].pointer(pointer).unwrap(),
                &fact["value"]
            );
            for (key, locator) in fact["condition"].as_object().unwrap() {
                if let Some(qualifier) = key.strip_suffix("_locator") {
                    let pointer = locator.as_str().unwrap().strip_prefix(&prefix).unwrap();
                    let value = record["source_record"].pointer(pointer).unwrap();
                    let expected = value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string());
                    assert_eq!(fact["condition"][qualifier], expected);
                }
            }
            if let Some(tier) = fact["condition"].get("upgrade_tier") {
                let parent = pointer.strip_suffix("/bonus").unwrap();
                let property = record["source_record"]
                    .pointer(&format!("{parent}/name"))
                    .unwrap();
                assert_eq!(
                    fact["stat"],
                    format!("ability.{}", property.as_str().unwrap())
                );
                assert_eq!(
                    pointer.split('/').nth(2).unwrap().parse::<u64>().unwrap() + 1,
                    tier.as_u64().unwrap()
                );
            }
        }
    }
    records
}

async fn insert_public_record(owner: &PgPool, record: &Value) -> i64 {
    let mut tx = owner.begin().await.unwrap();
    let mut receipts = Vec::new();
    for fact in record["facts"].as_array().unwrap() {
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (1,1,1,'json_pointer',$1,decode(repeat('02',32),'hex'),'brain-assets','v1') RETURNING id")
            .bind(fact["locator"].as_str().unwrap()).fetch_one(&mut *tx).await.unwrap();
        receipts.push(receipt);
    }
    let identity_index = record["facts"]
        .as_array()
        .unwrap()
        .iter()
        .position(|f| f["stat"] == "identity.key")
        .unwrap();
    let entity: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ($1,$2,$3) RETURNING id",
    )
    .bind(record["kind"].as_str().unwrap())
    .bind(record["key"].as_str().unwrap())
    .bind(receipts[identity_index])
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    for (fact, receipt) in record["facts"].as_array().unwrap().iter().zip(receipts) {
        let numeric = fact["numeric"].as_bool().unwrap();
        sqlx::query(
            "INSERT INTO canon.stat(key,section,unit,value_type,display_order)
            VALUES ($1,$2,$3,$4,$5) ON CONFLICT(key) DO NOTHING",
        )
        .bind(fact["stat"].as_str().unwrap())
        .bind(fact["section"].as_str().unwrap())
        .bind(fact["unit"].as_str())
        .bind(if numeric { "number" } else { "text" })
        .bind(fact["order"].as_i64().unwrap() as i32)
        .execute(&mut *tx)
        .await
        .unwrap();
        let value = fact["value"]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| fact["value"].to_string());
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,value_text,receipt_id,valid_from_revision_id)
            VALUES ($1,$2,1,'items-english.json',$3,$4::text::numeric,$5,$6,1)")
            .bind(entity).bind(fact["stat"].as_str().unwrap()).bind(&fact["condition"])
            .bind(numeric.then_some(value.clone())).bind((!numeric).then_some(value))
            .bind(receipt).execute(&mut *tx).await.unwrap();
    }
    tx.commit().await.unwrap();
    entity
}

async fn insert_public_hero_link(owner: &PgPool, record: &Value, ability: i64) {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/public_assets_card_6774.json")).unwrap();
    let link = fixture["hero_links"]
        .as_array()
        .unwrap()
        .iter()
        .find(|link| link["ability"] == record["key"])
        .unwrap();
    let mut tx = owner.begin().await.unwrap();
    let mut receipts = Vec::new();
    for locator in ["identity_locator", "locator"] {
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (2,1,1,'json_pointer',$1,decode(repeat('04',32),'hex'),'brain-assets','v1') RETURNING id")
            .bind(link[locator].as_str().unwrap()).fetch_one(&mut *tx).await.unwrap();
        receipts.push(receipt);
    }
    let hero: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero',$1,$2) RETURNING id",
    )
    .bind(link["key"].as_str().unwrap())
    .bind(receipts[0])
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES ($1,'has_ability',$2,$3,$4,1,'heroes-english.json',1)")
        .bind(hero).bind(ability).bind(link["slot"].as_i64().unwrap() as i16).bind(receipts[1])
        .execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
}

fn essential_public_fact(fact: &Value) -> bool {
    let stat = fact["stat"].as_str().unwrap();
    stat == "item.cost"
        || stat.contains(".description_")
        || fact["condition"].get("upgrade_tier").is_some()
        || (fact["section"] != "identity"
            && (!fact["numeric"].as_bool().unwrap()
                || fact["value"]
                    .as_f64()
                    .unwrap_or_else(|| fact["value"].as_str().unwrap().parse::<f64>().unwrap())
                    != 0.0))
}

async fn assert_public_card(owner: &PgPool, record: &Value, entity: i64) {
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.asset_card_p2 WHERE entity_id=$1")
        .bind(entity)
        .fetch_one(owner)
        .await
        .unwrap();
    let stored: Value = sqlx::query_scalar("SELECT card FROM serve.item_card WHERE entity_id=$1 UNION ALL SELECT card FROM serve.ability_card WHERE entity_id=$1")
        .bind(entity).fetch_one(owner).await.unwrap();
    assert_eq!(card, stored);
    let rows = card["facts"]["facts"].as_array().unwrap();
    let mut evidence = std::collections::BTreeSet::new();
    evidence.insert(card["identity"]["citation"].as_str().unwrap().to_owned());
    for fact in record["facts"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| essential_public_fact(f))
    {
        let returned = rows
            .iter()
            .find(|row| row["stat_key"] == fact["stat"] && row["condition"] == fact["condition"])
            .unwrap_or_else(|| {
                panic!(
                    "essential fact lost: {} {} {}",
                    record["key"], fact["stat"], fact["condition"]
                )
            });
        if fact["numeric"] == true {
            let expected = fact["value"]
                .as_f64()
                .unwrap_or_else(|| fact["value"].as_str().unwrap().parse().unwrap());
            assert_eq!(returned["value"].as_f64().unwrap(), expected);
        } else {
            assert_eq!(returned["value"], fact["value"]);
        }
        assert_eq!(
            returned["source_revision"],
            "31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596"
        );
        assert!(returned["patch"].is_null());
        let citation: i64 = returned["citation"].as_str().unwrap().parse().unwrap();
        let locator: String = sqlx::query_scalar("SELECT locator FROM canon.receipt WHERE id=$1")
            .bind(citation)
            .fetch_one(owner)
            .await
            .unwrap();
        assert_eq!(locator, fact["locator"].as_str().unwrap());
    }
    for row in rows {
        evidence.insert(row["citation"].as_str().unwrap().to_owned());
    }
    let links = card["links"]["links"].as_array().unwrap();
    assert_eq!(links.len(), usize::from(record["kind"] == "ability"));
    for link in links {
        evidence.insert(link["citation"].as_str().unwrap().to_owned());
        evidence.insert(link["identity"]["citation"].as_str().unwrap().to_owned());
        assert!(link["patch"].is_null());
        assert_eq!(
            link["source_revision"],
            "31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596"
        );
    }
    let mapped = record["facts"].as_array().unwrap().len();
    assert_eq!(card["facts"]["omitted_count"], mapped - rows.len());
    assert_eq!(card["facts"]["truncated"], mapped > rows.len());
    assert!(rows.len() > 12);
    let pg_bytes: i32 = sqlx::query_scalar(
        "SELECT octet_length(card::text) FROM serve.asset_card_p2 WHERE entity_id=$1",
    )
    .bind(entity)
    .fetch_one(owner)
    .await
    .unwrap();
    assert!(pg_bytes <= 15000);
    let returned_count = rows.len();
    let result = ToolResult {
        call_id: "\"".repeat(512),
        name: ToolName::DamageCalculate,
        result: json!({"card": card}),
        evidence_ids: evidence.into_iter().collect(),
        is_error: false,
    };
    let envelope_bytes = serde_json::to_vec(&result).unwrap().len();
    assert!(envelope_bytes <= 16 * 1024);
    println!("PUBLIC_CARD_PROJECTION: key={} mapped={mapped} returned={returned_count} pg_bytes={pg_bytes} ToolResult_bytes={envelope_bytes}; synthetic receipts, no ingestion or lexical hash audit, conservative existing tool name", record["key"]);
}

async fn publish_public_fixture(owner: &PgPool) {
    sqlx::raw_sql(
        "BEGIN; UPDATE raw.run SET status='succeeded' WHERE id=1;
        INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
        UPDATE raw.source SET last_complete_revision='31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596' WHERE id=1; COMMIT",
    )
    .execute(owner)
    .await
    .unwrap();
}

async fn insert_qualified_fact(
    owner: &PgPool,
    entity: i64,
    source: i64,
    stat: &str,
    value: &Value,
    condition: &Value,
    locator: &str,
) -> i64 {
    let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT o.id,v.source_id,v.id,'json_pointer',$2,decode(repeat('05',32),'hex'),v.extractor,v.extractor_version
        FROM canon.revision v JOIN raw.object o ON o.source_id=v.source_id
        WHERE v.source_id=$1 AND o.external_id<> 'heroes-english.json' RETURNING id")
        .bind(source).bind(locator).fetch_one(owner).await.unwrap();
    let value_type = if value.is_number() {
        "number"
    } else if value.is_string() {
        "text"
    } else {
        "json"
    };
    sqlx::query("INSERT INTO canon.stat(key,section,value_type) VALUES ($1,'ability',$2) ON CONFLICT(key) DO NOTHING")
        .bind(stat).bind(value_type).execute(owner).await.unwrap();
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,condition,value_num,value_text,value_json,receipt_id,valid_from_revision_id)
        SELECT $1,$2,p.source_id,o.external_id,$3,$4::text::numeric,$5,$6,p.id,p.revision_id
        FROM canon.receipt p JOIN raw.object o ON o.id=p.raw_object_id WHERE p.id=$7")
        .bind(entity).bind(stat).bind(condition)
        .bind(value.is_number().then(|| value.to_string())).bind(value.as_str())
        .bind((value_type == "json").then_some(value)).bind(receipt)
        .execute(owner).await.unwrap();
    receipt
}

async fn qualified_rows(owner: &PgPool, entity: i64, stat: &str) -> Vec<Value> {
    sqlx::query_scalar("SELECT serve.atomic_fact(entity_id,stat_key,value_num,value_text,value_json,unit,condition,receipt_id,source_revision)
        FROM serve.selected_fact_p2 WHERE entity_id=$1 AND stat_key=$2 ORDER BY condition,receipt_id")
        .bind(entity).bind(stat).fetch_all(owner).await.unwrap()
}

#[tokio::test]
async fn qualified_source_semantics_receipts_and_ambiguous_entries_on_real_postgres() {
    let scratch = Scratch::new();
    let owner = public_fixture_database(&scratch).await;
    sqlx::raw_sql("BEGIN;
        INSERT INTO ops.job_run(job,extractor,extractor_version)
            VALUES ('ingest-gamefiles','hero-ability-atoms','p1-v2'),('ingest-assets','brain-assets','v1');
        INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://qualified-gamefiles'),('assets','synthetic://qualified-assets');
        INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version)
            VALUES (2,2,'qualified-game','hero-ability-atoms','p1-v2'),(3,3,'qualified-assets','brain-assets','v1');
        INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
            VALUES (2,'abilities.vdata','qualified-game',decode(repeat('06',32),'hex'),'text/plain','qualified serving contract, not lexical receipt audit'),
                (3,'items-german.json','qualified-assets',decode(repeat('07',32),'hex'),'application/json','qualified serving contract, not lexical receipt audit');
        INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
            VALUES (2,'abilities.vdata',3,'upsert','succeeded'),(3,'items-german.json',4,'upsert','succeeded');
        INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
            VALUES (2,2,'qualified-game','hero-ability-atoms','p1-v2'),(3,3,'qualified-assets','brain-assets','v1');
        INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
            VALUES (3,2,2,'kv_pointer','/qualified_ability/0/m_eAbilityType/0',decode(repeat('08',32),'hex'),'hero-ability-atoms','p1-v2');
        INSERT INTO canon.entity(kind,key,identity_receipt_id)
            SELECT 'ability','qualified_ability',id FROM canon.receipt WHERE source_id=2;
        COMMIT")
        .execute(&owner).await.unwrap();
    let entity: i64 =
        sqlx::query_scalar("SELECT id FROM canon.entity WHERE key='qualified_ability'")
            .fetch_one(&owner)
            .await
            .unwrap();
    let upgrade = json!({"upgrade_tier":2,"operation":"delta"});
    let game_upgrade = json!({"upgrade_tier":2,"operation":"delta","upgrade_property":"AbilityCooldown",
        "upgrade_property_locator":"/qualified_ability/0/m_vecAbilityUpgrades/0/[1]/m_vecPropertyUpgrades/0/[0]/m_StrPropertyNAme/0"});
    let game_scale = json!({"scale_class":"CScale","scale_type":"Spirit","scale_value":"1.5","scale_stat_0":"Spirit",
        "scale_locator":"/qualified_ability/0/m_mapAbilityProperties/0/Damage/0/m_subclassScaleFunction/0"});
    let assets_scale = json!({"scale_class":"CScale","scale_type":"Spirit","scale_value":"1.5","scale_stat_0":"Spirit",
        "scale_class_locator":"/0/properties/Damage/scale_function/class_name",
        "scale_type_locator":"/0/properties/Damage/scale_function/specific_stat_scale_type",
        "scale_value_locator":"/0/properties/Damage/scale_function/stat_scale",
        "scale_stat_0_locator":"/0/properties/Damage/scale_function/scaling_stats/0"});
    let winners = [
        (
            "ability.AbilityCooldown",
            json!(-12),
            upgrade.clone(),
            game_upgrade,
            2,
        ),
        ("ability.Damage", json!(25), assets_scale, game_scale, 2),
        (
            "ability.DeadlockScale",
            json!(26),
            json!({"scale_type":"cooldown","scale_value":"1","scale_type_locator":"/0/properties/DeadlockScale/scale_function/specific_stat_scale_type","scale_value_locator":"/0/properties/DeadlockScale/scale_function/stat_scale"}),
            json!({"scale_type":"cooldown","scale_value":"1","scale_type_locator":"/qualified_ability/DeadlockScale/Scale/Type","scale_value_locator":"/qualified_ability/DeadlockScale/Scale/Value"}),
            2,
        ),
        (
            "ability.QualifiedJson",
            json!([1, 2, 3]),
            json!({"scale_type":"Spirit","scale_type_locator":"/0/properties/QualifiedJson/scale_function/specific_stat_scale_type"}),
            json!({"scale_type":"Spirit","scale_locator":"/qualified_ability/0/m_mapAbilityProperties/0/QualifiedJson/0/m_subclassScaleFunction/0"}),
            2,
        ),
        (
            "identity.name",
            json!("Assets Fähigkeit"),
            json!({"lang":"en"}),
            json!({"lang":"en"}),
            1,
        ),
    ];
    let mut expected_winners = Vec::new();
    for (stat, value, assets_condition, game_condition, source) in winners {
        let other_value = if value.is_number() {
            json!(999)
        } else if value.is_string() {
            json!("Game ability")
        } else {
            json!([99])
        };
        let assets_value = if source == 1 { &value } else { &other_value };
        let game_value = if source == 2 { &value } else { &other_value };
        let assets = insert_qualified_fact(
            &owner,
            entity,
            1,
            stat,
            assets_value,
            &assets_condition,
            "/0/value",
        )
        .await;
        let game = insert_qualified_fact(
            &owner,
            entity,
            2,
            stat,
            game_value,
            &game_condition,
            "/qualified_ability/0/value/0",
        )
        .await;
        let (receipt, condition) = if source == 1 {
            (assets, assets_condition)
        } else {
            (game, game_condition)
        };
        expected_winners.push((stat.to_owned(), value, condition, receipt));
    }
    for key in [
        "scale_class_locator",
        "scale_type_locator",
        "scale_value_locator",
        "scale_stat_0_locator",
        "scale_stat_1_locator",
        "scale_stat_2_locator",
        "scale_stat_3_locator",
        "usage_flag_0_locator",
        "usage_flag_1_locator",
        "usage_flag_2_locator",
        "usage_flag_3_locator",
    ] {
        let label = key.strip_suffix("_locator").unwrap();
        let stat = format!("ability.Known_{label}");
        let first = json!({label:"known",key:"/0/properties/value"});
        let second = json!({label:"known",key:"/9/properties/value"});
        let receipt =
            insert_qualified_fact(&owner, entity, 1, &stat, &json!(10), &first, "/0/value").await;
        insert_qualified_fact(&owner, entity, 3, &stat, &json!(99), &second, "/9/value").await;
        expected_winners.push((stat, json!(10), first, receipt));
    }
    let distinct = [
        (
            "tiers",
            upgrade.clone(),
            json!({"upgrade_tier":3,"operation":"delta"}),
        ),
        (
            "tier_types",
            upgrade.clone(),
            json!({"upgrade_tier":"2","operation":"delta"}),
        ),
        (
            "operations",
            upgrade.clone(),
            json!({"upgrade_tier":2,"operation":"set"}),
        ),
        ("languages", json!({"lang":"en"}), json!({"lang":"de"})),
        (
            "duplicate_entries",
            json!({"upgrade_tier":2,"operation":"delta","upgrade_entry":0}),
            json!({"upgrade_tier":2,"operation":"delta","upgrade_entry":1}),
        ),
        (
            "missing_entry",
            upgrade.clone(),
            json!({"upgrade_tier":2,"operation":"delta","upgrade_entry":0}),
        ),
        (
            "scale_classes",
            json!({"scale_class":"CScale"}),
            json!({"scale_class":"OtherScale"}),
        ),
        (
            "scale_types",
            json!({"scale_type":"Spirit"}),
            json!({"scale_type":"Health"}),
        ),
        (
            "scale_values",
            json!({"scale_value":"1"}),
            json!({"scale_value":"2"}),
        ),
        (
            "scale_numeric_lexemes",
            json!({"scale_value":"1"}),
            json!({"scale_value":"1.0"}),
        ),
        (
            "scale_value_types",
            json!({"scale_value":"1"}),
            json!({"scale_value":1}),
        ),
        (
            "scale_stats",
            json!({"scale_stat_0":"Spirit"}),
            json!({"scale_stat_0":"Health"}),
        ),
        (
            "scale_stat_positions",
            json!({"scale_stat_0":"Spirit"}),
            json!({"scale_stat_1":"Spirit"}),
        ),
        (
            "scale_missing_class",
            json!({"scale_type":"Spirit"}),
            json!({"scale_type":"Spirit","scale_class":"CScale"}),
        ),
        (
            "hero_scales",
            json!({"scale":"LevelScaling"}),
            json!({"scale":"SpiritScaling"}),
        ),
        (
            "usage_flags",
            json!({"usage_flag_0":"Imbued"}),
            json!({"usage_flag_0":"All"}),
        ),
        (
            "usage_positions",
            json!({"usage_flag_0":"Imbued"}),
            json!({"usage_flag_1":"Imbued"}),
        ),
        ("usage_missing", json!({}), json!({"usage_flag_0":"Imbued"})),
        (
            "source_flags",
            json!({"source_flags":"Imbued"}),
            json!({"usage_flag_0":"Imbued"}),
        ),
        (
            "unknown_metadata",
            json!({"future_qualifier":"one"}),
            json!({"future_qualifier":"two"}),
        ),
        (
            "unknown_locators",
            json!({"audit_locator":"/one"}),
            json!({"audit_locator":"/two"}),
        ),
        (
            "orphan_locators",
            json!({"scale_type_locator":"/one"}),
            json!({"scale_type_locator":"/two"}),
        ),
        (
            "malformed_locators",
            json!({"scale_type":"Spirit","scale_type_locator":1}),
            json!({"scale_type":"Spirit","scale_type_locator":2}),
        ),
        (
            "nonpointer_locators",
            json!({"scale_type":"Spirit","scale_type_locator":"one"}),
            json!({"scale_type":"Spirit","scale_type_locator":"two"}),
        ),
        (
            "null_companions",
            json!({"scale_type":null,"scale_type_locator":"/one"}),
            json!({"scale_type":null,"scale_type_locator":"/two"}),
        ),
        (
            "unknown_scale_index",
            json!({"scale_stat_4":"Spirit","scale_stat_4_locator":"/one"}),
            json!({"scale_stat_4":"Spirit","scale_stat_4_locator":"/two"}),
        ),
        (
            "inconsistent_property",
            upgrade.clone(),
            json!({"upgrade_tier":2,"operation":"delta","upgrade_property":"OtherStat","upgrade_property_locator":"/other/name"}),
        ),
        (
            "orphan_upgrade_locator",
            upgrade.clone(),
            json!({"upgrade_tier":2,"operation":"delta","upgrade_property_locator":"/other/name"}),
        ),
        (
            "orphan_scale_locator",
            json!({"scale_locator":"/one"}),
            json!({"scale_locator":"/two"}),
        ),
    ];
    let mut expected_distinct = Vec::new();
    for (name, first, second) in distinct {
        let stat = format!("ability.Distinct_{name}");
        let assets =
            insert_qualified_fact(&owner, entity, 1, &stat, &json!(999), &first, "/0/value").await;
        let game = insert_qualified_fact(
            &owner,
            entity,
            2,
            &stat,
            &json!(25),
            &second,
            "/qualified_ability/0/value/0",
        )
        .await;
        expected_distinct.push((stat, vec![(first, assets), (second, game)]));
    }
    let mut duplicate_upgrade = Vec::new();
    for entry in 0..2 {
        let condition = json!({"upgrade_tier":2,"operation":"delta","upgrade_property":"DuplicateUpgrade",
            "upgrade_property_locator":format!("/qualified_ability/0/m_vecAbilityUpgrades/0/[1]/m_vecPropertyUpgrades/0/[{entry}]/m_strPropertyName/0")});
        let receipt = insert_qualified_fact(
            &owner,
            entity,
            2,
            "ability.DuplicateUpgrade",
            &json!(-10 - entry),
            &condition,
            &format!("/duplicate/{entry}/bonus"),
        )
        .await;
        duplicate_upgrade.push((condition, receipt));
    }
    let receipt = insert_qualified_fact(
        &owner,
        entity,
        1,
        "ability.DuplicateUpgrade",
        &json!(-99),
        &upgrade,
        "/0/upgrades/1/property_upgrades/0/bonus",
    )
    .await;
    duplicate_upgrade.push((upgrade.clone(), receipt));
    expected_distinct.push(("ability.DuplicateUpgrade".into(), duplicate_upgrade));
    let mut duplicate_scale = Vec::new();
    for (source, locator) in [(1, "/one"), (2, "/two"), (2, "/three"), (3, "/four")] {
        let condition = json!({"scale_type":"Spirit","scale_type_locator":locator});
        let receipt = insert_qualified_fact(
            &owner,
            entity,
            source,
            "ability.DuplicateScale",
            &json!(source),
            &condition,
            locator,
        )
        .await;
        duplicate_scale.push((condition, receipt));
    }
    expected_distinct.push(("ability.DuplicateScale".into(), duplicate_scale));
    let other_entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id)
        SELECT 'ability','qualified_other',identity_receipt_id FROM canon.entity WHERE id=$1 RETURNING id")
        .bind(entity).fetch_one(&owner).await.unwrap();
    let other_receipt = insert_qualified_fact(
        &owner,
        other_entity,
        2,
        "ability.AbilityCooldown",
        &json!(7),
        &upgrade,
        "/other/value",
    )
    .await;
    let original = snapshot(&owner).await;
    assert!(qualified_rows(&owner, entity, "ability.AbilityCooldown")
        .await
        .is_empty());
    publish_public_fixture(&owner).await;
    let unpublished = qualified_rows(&owner, entity, "ability.AbilityCooldown").await;
    assert_eq!(unpublished.len(), 1);
    assert_eq!(unpublished[0]["value"], 999);
    sqlx::raw_sql("BEGIN; UPDATE raw.run SET status='succeeded' WHERE id IN (2,3);
        INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (2,2),(3,3);
        UPDATE raw.source SET last_complete_revision=CASE id WHEN 2 THEN 'qualified-game' ELSE 'qualified-assets' END WHERE id IN (2,3); COMMIT")
        .execute(&owner).await.unwrap();
    for (stat, value, condition, receipt) in &expected_winners {
        let rows = qualified_rows(&owner, entity, stat).await;
        assert_eq!(
            rows.len(),
            1,
            "equivalent conditions did not reconcile: {stat}"
        );
        assert_eq!(rows[0]["value"], *value, "wrong source priority: {stat}");
        assert_eq!(
            rows[0]["condition"], *condition,
            "winner condition changed: {stat}"
        );
        assert_eq!(
            rows[0]["citation"],
            receipt.to_string(),
            "winner receipt changed: {stat}"
        );
        assert!(rows[0]["patch"].is_null());
    }
    for (stat, originals) in &expected_distinct {
        let rows = qualified_rows(&owner, entity, stat).await;
        assert_eq!(
            rows.len(),
            originals.len(),
            "genuine or ambiguous qualifiers collapsed: {stat}"
        );
        for (condition, receipt) in originals {
            assert!(
                rows.iter()
                    .any(|row| row["condition"] == *condition
                        && row["citation"] == receipt.to_string()),
                "original assertion lost: {stat} {condition}"
            );
        }
    }
    let other_rows = qualified_rows(&owner, other_entity, "ability.AbilityCooldown").await;
    assert_eq!(other_rows.len(), 1);
    assert_eq!(other_rows[0]["entity_id"], other_entity);
    assert_eq!(other_rows[0]["value"], 7);
    assert_eq!(other_rows[0]["citation"], other_receipt.to_string());
    let selected_before: Vec<Value> =
        sqlx::query_scalar("SELECT to_jsonb(f) FROM serve.selected_fact_p2 f ORDER BY id")
            .fetch_all(&owner)
            .await
            .unwrap();
    sqlx::query("SELECT serve.refresh_assets_p2()")
        .execute(&owner)
        .await
        .unwrap();
    let selected_after: Vec<Value> =
        sqlx::query_scalar("SELECT to_jsonb(f) FROM serve.selected_fact_p2 f ORDER BY id")
            .fetch_all(&owner)
            .await
            .unwrap();
    assert_eq!(selected_before, selected_after);
    let retained = snapshot(&owner).await;
    for field in ["facts", "receipts", "entities", "relations"] {
        assert_eq!(
            original[field], retained[field],
            "serving rewrote stored {field}"
        );
    }
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=$1")
        .bind(entity)
        .fetch_one(&owner)
        .await
        .unwrap();
    assert!(serde_json::to_vec(&card).unwrap().len() < 16 * 1024);
    let upgrade_winner = &expected_winners[0];
    assert!(card["facts"]["facts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row["stat_key"] == upgrade_winner.0
            && row["condition"] == upgrade_winner.2
            && row["citation"] == upgrade_winner.3.to_string()));
    let tool_pool = scratch.pool("brain_public", "brain_test_dba").await;
    let mut tool = tool_pool.acquire().await.unwrap();
    sqlx::query("SET ROLE brain_tool")
        .execute(&mut *tool)
        .await
        .unwrap();
    sqlx::query("SELECT condition FROM serve.selected_fact_p2 WHERE entity_id=$1 AND stat_key='ability.AbilityCooldown'")
        .bind(entity).fetch_all(&mut *tool).await.unwrap();
    sqlx::query("RESET ROLE").execute(&mut *tool).await.unwrap();
    drop(tool);
    tool_pool.close().await;
    println!("QUALIFIED_SOURCE_CONTRACT: {} reconciled groups, {} distinct or ambiguous groups; synthetic canon assertions and receipts, no extraction or lexical hash audit", expected_winners.len(), expected_distinct.len());
    owner.close().await;
}

#[tokio::test]
async fn public_ability_values_descriptions_and_all_upgrade_tiers_survive_real_projection() {
    let scratch = Scratch::new();
    let owner = public_fixture_database(&scratch).await;
    let mut inserted = Vec::new();
    for record in public_records()
        .into_iter()
        .filter(|r| r["kind"] == "ability")
    {
        let entity = insert_public_record(&owner, &record).await;
        insert_public_hero_link(&owner, &record, entity).await;
        inserted.push((record, entity));
    }
    publish_public_fixture(&owner).await;
    for (record, entity) in inserted {
        assert_public_card(&owner, &record, entity).await;
    }
    owner.close().await;
}

#[tokio::test]
async fn public_item_cost_and_effects_survive_real_p1_kinds_and_p2_projection() {
    let scratch = Scratch::new();
    let owner = public_fixture_database(&scratch).await;
    let record = public_records()
        .into_iter()
        .find(|r| r["key"] == "upgrade_toxic_bullets")
        .unwrap();
    assert_eq!(record["source_record"]["cost"], 3200);
    assert_eq!(
        record["source_record"]["properties"]["DotHealthPercent"]["value"],
        "1.9"
    );
    assert_eq!(
        record["source_record"]["properties"]["HealAmpReceivePenaltyPercent"]["value"],
        "-35"
    );
    let entity = insert_public_record(&owner, &record).await;
    publish_public_fixture(&owner).await;
    assert_public_card(&owner, &record, entity).await;
    owner.close().await;
}
