use super::*;

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const SEED: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','brain-gamefiles','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://gamefiles');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'gamefiles-a','brain-gamefiles','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (1,'hero.kv','gamefiles-a',decode(repeat('01',32),'hex'),'text/plain',convert_to('synthetic','UTF8'));
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (1,'hero.kv',1,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'gamefiles-a','brain-gamefiles','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'kv_path','field-'||n,decode(repeat('02',32),'hex'),'brain-gamefiles','v1' FROM generate_series(1,5) n;
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','hero_fixture',1),('ability','ability_gamefiles',2),('ability','ability_assets',3);
    INSERT INTO canon.stat(key,section,unit,value_type) VALUES ('ability.damage','ability','damage','number');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,condition,receipt_id,valid_from_revision_id)
        VALUES (2,'ability.damage',1,'hero.kv',25,'{\"rank\":3,\"citation\":\"source-condition\"}',4,1);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES (1,'has_ability',2,1,5,1,'hero.kv',1);
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision='gamefiles-a' WHERE id=1;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','brain-assets','v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://assets');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (2,2,'assets-a','brain-assets','v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        VALUES (2,'abilities.json','assets-a',decode(repeat('03',32),'hex'),'application/json',convert_to('{}','UTF8'));
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES (2,'abilities.json',2,'upsert','succeeded');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (2,2,'assets-a','brain-assets','v1');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 2,2,2,'json_pointer','/field-'||n,decode(repeat('04',32),'hex'),'brain-assets','v1' FROM generate_series(1,2) n;
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,condition,receipt_id,valid_from_revision_id)
        VALUES (3,'ability.damage',2,'abilities.json',99,'{}',6,2);
    INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES (1,'has_ability',3,1,7,2,'abilities.json',2);
    UPDATE raw.run SET status='succeeded' WHERE id=2;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (2,2);
    UPDATE raw.source SET last_complete_revision='assets-a' WHERE id=2;
    COMMIT;";

#[test]
fn real_p2_ability_cards_use_selected_slots_and_published_receipts() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let options = |user: &str| {
        PgConnectOptions::new_without_pgpass()
            .host(pg.directory.join("socket").to_str().unwrap())
            .port(55439)
            .username(user)
            .database("postgres")
    };
    let admin = runtime.block_on(async {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(options("brain_core_test"))
            .await
            .unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN;
            CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN;
            CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
            GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE;
            GRANT CREATE ON DATABASE postgres TO brain_migrate;")
            .execute(&pool).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        sqlx::query("SET ROLE brain_migrate").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"))
            .execute(&mut *connection).await.unwrap();
        sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(SEED).execute(&mut *connection).await.unwrap();
        sqlx::query("SET ROLE brain_migrate").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p2-assets-v1.sql"))
            .execute(&mut *connection).await.unwrap();
        sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
        let targets: Vec<i64> = sqlx::query_scalar("SELECT to_entity_id FROM serve.current_relation WHERE from_entity_id=1 AND slot=1 ORDER BY to_entity_id")
            .fetch_all(&mut *connection).await.unwrap();
        assert_eq!(targets, vec![2, 3]);
        let receipts: Vec<i64> = sqlx::query_scalar("SELECT receipt_id FROM canon.relation ORDER BY receipt_id")
            .fetch_all(&mut *connection).await.unwrap();
        assert_eq!(receipts, vec![5, 7]);
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt WHERE id IN (5,7)")
            .fetch_one(&mut *connection).await.unwrap();
        assert_eq!(count, 2);
        let producer: Value = sqlx::query_scalar(ABILITY).bind(2_i64).fetch_one(&mut *connection).await.unwrap();
        assert_eq!(producer["facts"]["facts"][0]["value"], 25);
        assert_eq!(producer["facts"]["omitted_count"], 0);
        assert_eq!(producer["links"]["links"][0]["citation"], "5");
        assert_eq!(bounded_value(ToolName::GetAbility, producer.clone()).unwrap(), Some(producer));
        eprintln!("P2_POSTGRES_ABILITY_PRODUCER_CONSUMER_RAN; POSITIVE_ITEM_SQL_PENDING_P1_KIND_CHECKPOINT");
        drop(connection);
        pool
    });
    let pool = {
        let _entered = runtime.enter();
        tool_pool(options("brain_readonly"), Duration::from_secs(2))
    };
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let query: Query = serde_json::from_value(json!({"request_id":"p2-pg","conversation_id":"p2-pg","text":"Fähigkeit","profile":"explain"})).unwrap();
    let context = AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: query.conversation_id.clone(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    };
    for (arguments, expected_ids) in [
        (json!({"ability":2}), vec!["1", "2", "4", "5"]),
        (json!({"hero":1,"slot":1}), vec!["1", "2", "4", "5"]),
        (json!({"ability":3}), vec!["3", "6"]),
    ] {
        let request = request(ToolName::GetAbility, arguments).unwrap();
        let result = tools
            .execute(&query, &context, None, "\"".repeat(512).as_str(), &request)
            .unwrap();
        assert!(!result.result.is_error);
        assert_eq!(result.result.evidence_ids, expected_ids);
        assert_eq!(result.dependencies.len(), 1);
        assert_eq!(result.dependencies[0].evidence.len(), expected_ids.len());
        assert!(serde_json::to_vec(&result.result).unwrap().len() <= MAX_BYTES);
        if result.result.result.get("abilities").is_some() {
            assert_eq!(result.result.result["identity"]["entity_id"], 2);
            assert_eq!(
                result.result.result["abilities"]["links"][0]["citation"],
                "5"
            );
            assert_eq!(
                result.result.result["abilities"]["links"][0]["source_revision"],
                "gamefiles-a"
            );
            assert_eq!(result.result.result["abilities"]["omitted_count"], 0);
        }
        tools
            .validate_dependencies(
                &query,
                &context,
                None,
                &result.dependencies,
                ToolValidationPurpose::Publication,
            )
            .unwrap();
    }
    for arguments in [json!({"ability":999}), json!({"hero":1,"slot":2})] {
        let request = request(ToolName::GetAbility, arguments).unwrap();
        let missing = tools
            .execute(&query, &context, None, "missing", &request)
            .unwrap();
        assert!(missing.result.is_error);
        assert!(missing.result.evidence_ids.is_empty() && missing.dependencies.is_empty());
    }
    runtime.block_on(async {
        let mut connection = pool.acquire().await.unwrap();
        assert_session(&mut connection).await.unwrap();
        assert!(sqlx::query("SELECT body FROM raw.object")
            .execute(&mut *connection)
            .await
            .is_err());
        drop(connection);
        pool.close().await;
        admin.close().await;
    });
}
