use super::*;

async fn retained_identity_revision(pool: &PgPool, revision: i64, hero: i64, ability: i64) {
    let hero_receipt = metadata_receipt(pool, revision, "hero_kept.MaxHealth").await;
    let ability_receipt = metadata_receipt(pool, revision, "ability_kept.AbilityCooldown").await;
    let link_receipt = metadata_receipt(pool, revision, "hero_kept.BoundAbilities.1").await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        VALUES ($1,'stats.max_health',1,'hero.kv',301,$3,$5),($2,'ability.damage',1,'hero.kv',21,$4,$5)")
        .bind(hero).bind(ability).bind(hero_receipt).bind(ability_receipt).bind(revision)
        .execute(pool).await.unwrap();
    sqlx::query("INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES ($1,'kept hero','en',$3,1,'hero.kv',$5),($2,'kept ability','en',$4,1,'hero.kv',$5)")
        .bind(hero).bind(ability).bind(hero_receipt).bind(ability_receipt).bind(revision)
        .execute(pool).await.unwrap();
    sqlx::query("INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
        VALUES ($1,'has_ability',$2,1,$3,1,'hero.kv',$4)")
        .bind(hero).bind(ability).bind(link_receipt).bind(revision)
        .execute(pool).await.unwrap();
}

async fn assert_published_execution(pool: &PgPool, execution: &ToolExecution) {
    assert!(!execution.result.is_error);
    assert!(serde_json::to_vec(&execution.result).unwrap().len() <= MAX_BYTES);
    let mut pairs = BTreeMap::new();
    citations(&execution.result.result, &mut pairs).unwrap();
    assert_eq!(pairs.len(), execution.result.evidence_ids.len());
    for evidence in &execution.dependencies[0].evidence {
        let id = evidence.evidence_id.parse::<i64>().unwrap();
        let (revision_id, source_revision, status, published): (i64, String, String, bool) =
            sqlx::query_as("SELECT p.revision_id,v.source_revision,r.status,h.published_revision_id>=v.id
                FROM canon.receipt p JOIN canon.revision v ON v.id=p.revision_id
                JOIN raw.run r ON r.id=v.raw_run_id JOIN canon.source_head h ON h.source_id=p.source_id
                WHERE p.id=$1")
                .bind(id).fetch_one(pool).await.unwrap();
        assert_eq!(status, "succeeded");
        assert!(published);
        assert_eq!(pairs.get(&id), Some(&source_revision));
        assert_eq!(evidence.revision, revision_id as u64);
        let content: Value = serde_json::from_str(&evidence.content).unwrap();
        assert_eq!(content["citation"], evidence.evidence_id);
        assert_eq!(content["source_revision"], source_revision);
    }
}

#[test]
fn real_pg_recovered_hero_and_ability_use_only_succeeded_identity_receipts() {
    let pg = ScratchPg::start();
    let executor = tokio::runtime::Builder::new_multi_thread()
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
    let admin = executor.block_on(async {
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
        drop(connection);
        pool
    });
    let pool = {
        let _entered = executor.enter();
        tool_pool(options("brain_readonly"), Duration::from_secs(2))
    };
    let tools = PgTools::new(Delegate::default(), pool.clone(), executor.handle().clone());
    let query = query();
    let context = context();
    let (failed_revision, hero, ability, failed_hero_receipt, failed_ability_receipt) = executor
        .block_on(async {
            let revision = metadata_revision(&admin, "failed-identities").await;
            let hero_receipt = metadata_receipt(&admin, revision, "hero_kept").await;
            let ability_receipt = metadata_receipt(&admin, revision, "ability_kept").await;
            let hero: i64 = sqlx::query_scalar(
                "INSERT INTO canon.entity(kind,key,identity_receipt_id)
            VALUES ('hero','hero_kept',$1) RETURNING id",
            )
            .bind(hero_receipt)
            .fetch_one(&admin)
            .await
            .unwrap();
            let ability: i64 = sqlx::query_scalar(
                "INSERT INTO canon.entity(kind,key,identity_receipt_id)
            VALUES ('ability','ability_kept',$1) RETURNING id",
            )
            .bind(ability_receipt)
            .fetch_one(&admin)
            .await
            .unwrap();
            retained_identity_revision(&admin, revision, hero, ability).await;
            (revision, hero, ability, hero_receipt, ability_receipt)
        });
    let find = request(
        ToolName::FindEntity,
        json!({"text":"Kept Hero","kind":"hero"}),
    );
    assert!(
        tools
            .execute(&query, &context, None, "unpublished", &find)
            .unwrap()
            .result
            .is_error
    );
    let audit: Value = executor
        .block_on(
            sqlx::query_scalar(
                "SELECT jsonb_build_object(
        'entities',(SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e),
        'receipts',(SELECT jsonb_agg(to_jsonb(p) ORDER BY id) FROM canon.receipt p))",
            )
            .fetch_one(&admin),
        )
        .unwrap();
    executor.block_on(async {
        sqlx::query("INSERT INTO ops.job_lease(lease_key,run_id,epoch,expires_at)
            SELECT 'brain-job:ingest-gamefiles',r.job_run_id,1,clock_timestamp()+interval '1 hour'
            FROM canon.revision v JOIN raw.run r ON r.id=v.raw_run_id WHERE v.id=$1")
            .bind(failed_revision).execute(&admin).await.unwrap();
        sqlx::query("UPDATE raw.run SET status='failed' WHERE id=(SELECT raw_run_id FROM canon.revision WHERE id=$1)")
            .bind(failed_revision).execute(&admin).await.unwrap();
        let corrected = metadata_revision(&admin, "corrected-identities").await;
        for table in ["fact", "alias", "relation"] {
            sqlx::query(&format!("UPDATE canon.{table} SET valid_to_revision_id=$1 WHERE valid_from_revision_id=$2"))
                .bind(corrected).bind(failed_revision).execute(&admin).await.unwrap();
        }
        retained_identity_revision(&admin, corrected, hero, ability).await;
        publish_metadata_revision(&admin, corrected).await;
        let unchanged: Value = sqlx::query_scalar("SELECT jsonb_build_object(
            'entities',(SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e),
            'receipts',(SELECT jsonb_agg(to_jsonb(p) ORDER BY id) FROM canon.receipt p WHERE revision_id<=$1))")
            .bind(failed_revision).fetch_one(&admin).await.unwrap();
        assert_eq!(unchanged, audit);
        let status: String = sqlx::query_scalar("SELECT r.status FROM canon.revision v JOIN raw.run r ON r.id=v.raw_run_id WHERE v.id=$1")
            .bind(failed_revision).fetch_one(&admin).await.unwrap();
        assert_eq!(status, "failed");
    });
    for (text, kind, entity, expected_match) in [
        ("Kept Hero", "hero", hero, "exact"),
        ("Kept Her", "hero", hero, "suggestion"),
        ("Kept Ability", "ability", ability, "exact"),
        ("Kept Abilit", "ability", ability, "suggestion"),
    ] {
        let find = request(ToolName::FindEntity, json!({"text":text,"kind":kind}));
        let execution = tools
            .execute(&query, &context, None, "recovered-find", &find)
            .unwrap();
        assert_eq!(execution.result.result["match"], expected_match);
        assert_eq!(execution.result.result["entities"][0]["entity_id"], entity);
        assert_eq!(
            execution.result.result["entities"][0]["identity"]["source_revision"],
            "corrected-identities"
        );
        executor.block_on(assert_published_execution(&admin, &execution));
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
            ToolValidationPurpose::Cache,
        ] {
            tools
                .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
                .unwrap();
        }
    }
    let hero_request = request(ToolName::GetHero, json!({"hero":hero}));
    let profile = tools
        .execute(&query, &context, None, "recovered-profile", &hero_request)
        .unwrap();
    assert_eq!(profile.result.result["identity"]["key"], "hero_kept");
    assert_eq!(profile.result.result["stats"]["facts"][0]["value"], 301);
    let link = &profile.result.result["abilities"]["links"][0];
    assert_eq!(link["key"], "ability_kept");
    assert_eq!(link["facts"][0]["value"], 21);
    assert_eq!(link["identity"]["source_revision"], "corrected-identities");
    executor.block_on(assert_published_execution(&admin, &profile));
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &profile.dependencies, purpose)
            .unwrap();
    }
    executor.block_on(async {
        let mut connection = pool.acquire().await.unwrap();
        let failed_rows = sqlx::query(RECEIPTS)
            .bind(vec![failed_hero_receipt, failed_ability_receipt])
            .fetch_all(&mut *connection)
            .await
            .unwrap();
        assert!(failed_rows.is_empty());
        for (id, revision) in [
            (failed_hero_receipt.to_string(), "failed-identities"),
            (failed_ability_receipt.to_string(), "corrected-identities"),
            (
                profile.result.result["identity"]["citation"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                "failed-identities",
            ),
        ] {
            let result = bounded_execution(
                &mut connection,
                "invalid-proof",
                &hero_request,
                json!({"identity":{"citation":id,"source_revision":revision,"patch":null}}),
            )
            .await;
            assert!(matches!(result, Err(PortError::InvalidResponse(_))));
        }
        drop(connection);
        pool.close().await;
        admin.close().await;
    });
}
