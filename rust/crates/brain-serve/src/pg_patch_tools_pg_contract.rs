use super::super::*;

struct ScratchPg {
    directory: tempfile::TempDir,
}

impl ScratchPg {
    fn start() -> Self {
        let directory = tempfile::Builder::new()
            .prefix("brain-p4-tools-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(directory.path().join("socket")).unwrap();
        let instance = Self { directory };
        let initialized = std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
            .arg("-D")
            .arg(instance.directory.path().join("data"))
            .args([
                "-A",
                "trust",
                "-U",
                "brain_patch_test",
                "--no-locale",
                "--encoding=UTF8",
            ])
            .output()
            .unwrap();
        assert!(
            initialized.status.success(),
            "{}",
            String::from_utf8_lossy(&initialized.stderr)
        );
        let started = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(instance.directory.path().join("data"))
            .arg("-l")
            .arg(instance.directory.path().join("postgres.log"))
            .arg("-o")
            .arg(format!(
                "-k {} -p 55449 -c listen_addresses=''",
                instance.directory.path().join("socket").display()
            ))
            .args(["-w", "start"])
            .output()
            .unwrap();
        assert!(
            started.status.success(),
            "{}",
            String::from_utf8_lossy(&started.stderr)
        );
        instance
    }

    async fn connection(&self, database: &str) -> PgConnection {
        PgConnection::connect_with(
            &sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(self.directory.path().join("socket").to_str().unwrap())
                .port(55449)
                .username("brain_patch_test")
                .database(database)
                .ssl_mode(sqlx::postgres::PgSslMode::Disable),
        )
        .await
        .unwrap()
    }
}

impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.path().join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output();
    }
}

const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('p4-contract','json','v1');
    INSERT INTO raw.source(kind,locator) VALUES
        ('patchnotes','postgres://patchnotes-feed/public.patchnotes'),('git','synthetic://p4-game');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version)
        VALUES (1,1,'official','json','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        SELECT 1,n::text,'official',sha256(convert_to(body,'UTF8')),'application/json',convert_to(body,'UTF8')
        FROM (VALUES
            (1,'{\"id\":1,\"title\":\"Opening\",\"url\":\"https://example.invalid/p4/opening\",\"posted_at\":\"2026-10-01T00:00:00Z\",\"raw_content\":\"Contract fixture\"}'),
            (2,'{\"id\":2,\"title\":\"Snapshot\",\"url\":\"https://example.invalid/p4/snapshot\",\"posted_at\":\"2026-10-02T00:00:00Z\",\"raw_content\":\"Contract fixture\"}')
        ) payload(n,body) ORDER BY n;
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
        VALUES (1,'1',1,'upsert','succeeded'),(1,'2',2,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
        VALUES (1,1,'official','json','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT id,1,1,'json_pointer','/title',sha256(convert_to((convert_from(body,'UTF8')::json->'title')::text,'UTF8')),'json','v1'
        FROM raw.object WHERE source_id=1 ORDER BY id;
    SELECT canon.stage_patch_origin(1);
    SELECT canon.stage_patch_origin(2);
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision='official' WHERE id=1;
    SELECT canon.publish_patch_origin(1);
    SELECT canon.publish_patch_origin(2);
    COMMIT;
    BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('p4-contract','json','v1');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version)
        VALUES (2,2,'opening','json','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        SELECT 2,'hero.json','opening',sha256(convert_to(body,'UTF8')),'application/json',convert_to(body,'UTF8')
        FROM (VALUES ('{\"hero_name\":\"Baba\",\"ability_name\":\"Original\",\"health\":100,\"damage\":25,\"ability\":\"p4_original\",\"patch_url\":\"https://example.invalid/p4/opening\",\"replacement_hero_name\":\"Neuer Held\",\"replacement_ability_name\":\"Neue Fähigkeit\",\"replacement_health\":120,\"replacement_damage\":30,\"replacement_ability\":\"p4_replacement\"}')) payload(body);
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
        VALUES (2,'hero.json',3,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
        VALUES (2,2,'opening','json','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 3,2,2,'json_pointer','/'||field,sha256(convert_to((convert_from(o.body,'UTF8')::json->field)::text,'UTF8')),'json','v1'
        FROM raw.object o CROSS JOIN (VALUES
            (1,'hero_name'),(2,'ability_name'),(3,'health'),(4,'damage'),(5,'ability'),(6,'patch_url')
        ) proof(n,field) WHERE o.id=3 ORDER BY n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id)
        VALUES ('hero','p4_baba',3),('ability','p4_original',4);
    INSERT INTO canon.stat(key,section,value_type)
        VALUES ('health','stats','number'),('damage','ability','number');
    INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id)
        VALUES (2,'opening',1,8);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        VALUES (1,'health',2,'hero.json',100,5,2),(2,'damage',2,'hero.json',25,6,2);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES (1,'has_ability',2,1,7,2,'hero.json',2);
    UPDATE raw.run SET status='succeeded' WHERE id=2;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (2,2);
    UPDATE raw.source SET last_complete_revision='opening' WHERE id=2;
    COMMIT;
    BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('p4-contract','json','v1');
    INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version)
        VALUES (2,3,'opening','snapshot','json','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        SELECT 2,'hero.json','snapshot',sha256(convert_to(body,'UTF8')),'application/json',convert_to(body,'UTF8')
        FROM (VALUES ('{\"patch_url\":\"https://example.invalid/p4/snapshot\"}')) payload(body);
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
        VALUES (3,'hero.json',4,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
        VALUES (2,3,'snapshot','json','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 4,2,3,'json_pointer','/patch_url',sha256(convert_to((convert_from(body,'UTF8')::json->'patch_url')::text,'UTF8')),'json','v1'
        FROM raw.object WHERE id=4;
    INSERT INTO canon.source_revision_patch(source_id,source_revision,patch_id,receipt_id)
        VALUES (2,'snapshot',2,9);
    UPDATE raw.run SET status='succeeded' WHERE id=3;
    UPDATE canon.source_head SET published_revision_id=3 WHERE source_id=2;
    UPDATE raw.source SET last_complete_revision='snapshot' WHERE id=2;
    COMMIT;";

const PENDING: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('p4-contract','json','v2');
    INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version)
        VALUES (2,4,'snapshot','closing-upstream','json','v2');
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
        VALUES (4,'hero.json',3,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
        SELECT source_id,id,target_revision,extractor,extractor_version FROM raw.run WHERE id=4;
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 3,2,4,'json_pointer','/'||field,sha256(convert_to((convert_from(o.body,'UTF8')::json->field)::text,'UTF8')),'json','v2'
        FROM raw.object o CROSS JOIN (VALUES
            (1,'replacement_hero_name'),(2,'replacement_ability_name'),(3,'replacement_health'),
            (4,'replacement_damage'),(5,'replacement_ability')
        ) proof(n,field) WHERE o.id=3 ORDER BY n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id)
        VALUES ('hero','p4_new_hero',10),('ability','p4_replacement',11);
    UPDATE canon.fact SET valid_to_revision_id=4 WHERE id IN (1,2);
    UPDATE canon.relation SET valid_to_revision_id=4 WHERE id=1;
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        VALUES (1,'health',2,'hero.json',120,12,4),(4,'damage',2,'hero.json',30,13,4),
            (3,'health',2,'hero.json',120,12,4);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES (1,'has_ability',4,1,14,2,'hero.json',4);
    COMMIT;";

async fn profile(connection: &mut PgConnection, label: &str) -> ToolExecution {
    let call = ToolCall {
        id: "historical".into(),
        name: ToolName::GetHero,
        arguments: json!({"hero":1,"patch":label}),
    };
    let request = call.validate(&[historical_definition()]).unwrap();
    let ToolSubrequest::GetHero(hero) = request.subrequest() else {
        panic!("Expected a historical hero request");
    };
    sqlx::query("SET ROLE brain_tool")
        .execute(&mut *connection)
        .await
        .unwrap();
    let mut transaction = connection.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *transaction)
        .await
        .unwrap();
    let PatchResolution::Found(patch) = resolve_patch(&mut transaction, label).await.unwrap() else {
        panic!("Expected a published fixture patch");
    };
    let execution = historical_execution(&mut transaction, &call.id, &request, hero, patch)
        .await
        .unwrap();
    execution.validate_for(&call, &request, None).unwrap();
    transaction.commit().await.unwrap();
    sqlx::query("SET ROLE brain_ingest")
        .execute(connection)
        .await
        .unwrap();
    execution
}

#[tokio::test]
async fn real_pg_pending_closures_preserve_historical_profile_and_receipts() {
    let pg = ScratchPg::start();
    let mut admin = pg.connection("postgres").await;
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN;
        CREATE ROLE brain_service NOLOGIN;
        CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;",
    )
    .execute(&mut admin)
    .await
    .unwrap();
    for database in ["brain_patch_known", "brain_patch_unknown"] {
        sqlx::query(&format!("CREATE DATABASE {database} OWNER brain_migrate"))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    let migration = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../scripts/migrations/2026-10-12-brain-gamefacts-p4-patches-v1.sql"),
    )
    .expect("Integrate the P4 schema before running this PostgreSQL contract");
    for (database, upstream) in [
        ("brain_patch_known", "opening"),
        ("brain_patch_unknown", "unmapped-close"),
    ] {
        let mut connection = pg.connection(database).await;
        sqlx::query("SET ROLE brain_migrate")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
        ))
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::raw_sql(&migration)
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query("SET ROLE brain_ingest")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::raw_sql(SEED)
            .execute(&mut connection)
            .await
            .unwrap();
        let before = profile(&mut connection, "Snapshot").await;
        assert!(!before.result.is_error);
        assert_eq!(before.result.result["stats"]["facts"][0]["value"], 100);
        assert_eq!(before.result.result["stats"]["facts"][0]["citation"], "5");
        assert_eq!(
            before.result.result["stats"]["facts"][0]["patch_mapping"]["patch"],
            "Opening"
        );
        assert_eq!(
            before.result.result["stats"]["facts"][0]["snapshot"]["patch"],
            "Snapshot"
        );
        let opening_before = profile(&mut connection, "Opening").await;
        assert!(!opening_before.result.is_error);
        let ability = &before.result.result["abilities"]["links"][0];
        assert_eq!(ability["entity_id"], 2);
        assert_eq!(ability["citation"], "7");
        assert_eq!(ability["facts"][0]["value"], 25);
        assert_eq!(ability["facts"][0]["citation"], "6");
        assert!(ability["closing_mapping"].is_null());
        assert!(ability["facts"][0]["closing_mapping"].is_null());
        assert!(before.result.result["stats"]["facts"][0]["closing_mapping"].is_null());
        assert!(!before.dependencies[0].evidence.is_empty());
        for evidence in &before.dependencies[0].evidence {
            assert!(evidence.patch.is_none());
            let content: Value = serde_json::from_str(&evidence.content).unwrap();
            assert!(content["patch"].is_null());
        }
        sqlx::raw_sql(&PENDING.replace("closing-upstream", upstream))
            .execute(&mut connection)
            .await
            .unwrap();
        for status in ["running", "partial", "failed"] {
            sqlx::query("UPDATE raw.run SET status=$1 WHERE id=4")
                .bind(status)
                .execute(&mut connection)
                .await
                .unwrap();
            let pending = profile(&mut connection, "Snapshot").await;
            assert_eq!(pending.result, before.result);
            assert_eq!(pending.dependencies, before.dependencies);
            let opening_pending = profile(&mut connection, "Opening").await;
            assert_eq!(opening_pending.result, opening_before.result);
            assert_eq!(opening_pending.dependencies, opening_before.dependencies);
            for patch in [1, 2] {
                for entity in [3, 4] {
                    assert!(historical_identity(&mut connection, entity, patch)
                        .await
                        .unwrap()
                        .is_none());
                    assert!(
                        historical_facts(&mut connection, entity, patch, &["stats", "ability"])
                            .await
                            .unwrap()
                            .is_empty()
                    );
                }
            }
        }
        sqlx::raw_sql(
            "BEGIN; UPDATE raw.run SET status='succeeded' WHERE id=4;
            UPDATE canon.source_head SET published_revision_id=4 WHERE source_id=2;
            UPDATE raw.source SET last_complete_revision=(SELECT source_revision FROM canon.revision WHERE id=4) WHERE id=2;
            COMMIT;",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        let published = profile(&mut connection, "Snapshot").await;
        assert!(published.result.is_error);
        assert_eq!(
            published.result.result["error"],
            "historical_patch_unavailable"
        );
        assert!(published.result.evidence_ids.is_empty());
        assert!(published.dependencies.is_empty());
        if upstream == "opening" {
            let refreshed = profile(&mut connection, "Opening").await;
            assert!(!refreshed.result.is_error);
            assert_eq!(refreshed.result.result["stats"]["facts"][0]["value"], 120);
            assert_eq!(
                refreshed.result.result["stats"]["facts"][0]["citation"],
                "12"
            );
            assert_eq!(
                refreshed.result.result["abilities"]["links"][0]["entity_id"],
                4
            );
            assert_eq!(
                refreshed.result.result["abilities"]["links"][0]["citation"],
                "14"
            );
            let mapping: (i64, i64, i64) = sqlx::query_as(
                "SELECT evidence_revision_id,receipt_id,patch_receipt_id
                FROM serve.published_source_revision_patch WHERE source_id=2 AND source_revision='opening'",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(mapping, (4, 8, 1));
        } else {
            let unknown_close = profile(&mut connection, "Opening").await;
            assert!(unknown_close.result.is_error);
            assert!(unknown_close.result.evidence_ids.is_empty());
            assert!(unknown_close.dependencies.is_empty());
        }
        connection.close().await.unwrap();
    }
    admin.close().await.unwrap();
}
