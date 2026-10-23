use super::*;

pub(super) async fn publish_timer(admin: &PgPool, revision: &str, large: bool) {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("timer_fixture").await.unwrap();
    let count = if large { 40 } else { 2 };
    let bytes = format!("{{\"values\":[{}]}}", vec!["120.00"; count].join(",")).into_bytes();
    let raw = store
        .capture(
            run,
            "timer_fixture",
            revision,
            "generic.json",
            bytes.clone(),
        )
        .await
        .unwrap();
    let key = "timer_rejuvenator__timer_fixture_rejuvparams";
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(&bytes).into(),
    };
    let condition = json!({"source_entity":"RejuvParams","flags":if large { vec!["x".repeat(2000)] } else { Vec::new() }});
    let set = FactSet {
        entities: vec![Entity {
            kind: "timer".into(),
            key: key.into(),
            receipt: receipt.clone(),
        }],
        aliases: vec![Alias {
            entity_key: key.into(),
            alias: "rejuvenator".into(),
            lang: "und".into(),
            receipt,
        }],
        facts: (0..count)
            .map(|index| Fact {
                entity_key: key.into(),
                stat_key: format!("timer_duration_{index:02}"),
                value: FactValue::Text(
                    json!({"value":"120.00","unit":"s","condition":condition}).to_string(),
                ),
                receipt: Receipt {
                    locator_kind: "json_pointer".into(),
                    locator: format!("/values/{index}"),
                    value_sha256: Sha256::digest(b"120.00").into(),
                },
            })
            .collect(),
        ..FactSet::default()
    };
    store
        .apply(run, &raw, "timer_fixture_v1", &set)
        .await
        .unwrap();
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
}

#[test]
fn timer_pg_executor_preserves_values_receipts_bounds_and_revalidation() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("");
    let (admin, pool) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2).connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        publish_timer(&admin, "timer-r1", false).await;
        let pool = tool_pool(options.username("brain_readonly"), Duration::from_secs(2));
        let mut connection = pool.acquire().await.unwrap();
        assert_session(&mut connection).await.unwrap();
        let timeout = sqlx::query("SELECT pg_sleep(1)").execute(&mut *connection).await.unwrap_err();
        assert_eq!(timeout.as_database_error().unwrap().code().as_deref(), Some("57014"));
        drop(connection);
        (admin, pool)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let query = query();
    let context = context();
    let timer_request = request(ToolName::GetTimer, json!({"timer":"rejuvenator"}));
    let execution = tools
        .execute(&query, &context, None, "timer", &timer_request)
        .unwrap();
    assert!(!execution.result.is_error);
    assert_eq!(
        execution.result.result["facts"].as_array().unwrap().len(),
        2
    );
    assert!(serde_json::to_vec(&execution.result).unwrap().len() < MAX_BYTES);
    for fact in execution.result.result["facts"].as_array().unwrap() {
        assert_eq!(fact["value"], "120.00");
        assert_eq!(fact["unit"], "s");
        assert_eq!(fact["condition"]["source_entity"], "RejuvParams");
        assert_eq!(fact["source_revision"], "timer-r1");
        assert!(fact["patch"].is_null());
    }
    for evidence in &execution.dependencies[0].evidence {
        let canonical =
            brain_contracts::canonical_receipt::CanonicalReceipt::from_evidence(evidence).unwrap();
        assert_eq!(
            brain_contracts::canonical_receipt::format_receipt(&canonical).unwrap(),
            *evidence
        );
        assert!(execution
            .result
            .evidence_ids
            .contains(&evidence.evidence_id));
        assert!(execution.result.result["facts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|fact| fact["citation"].as_str() == Some(evidence.evidence_id.as_str())));
        let metadata: Value = serde_json::from_str(&evidence.content).unwrap();
        let raw = runtime
            .block_on(
                sqlx::query_scalar::<_, Vec<u8>>("SELECT bytes FROM raw.object WHERE id=$1")
                    .bind(metadata["raw_object_id"].as_i64().unwrap())
                    .fetch_one(&admin),
            )
            .unwrap();
        let receipt = Receipt {
            locator_kind: metadata["locator_kind"].as_str().unwrap().into(),
            locator: metadata["locator"].as_str().unwrap().into(),
            value_sha256: hex::decode(metadata["value_sha256"].as_str().unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
        };
        assert_eq!(
            brain_storage::gamefacts::receipts::value_bytes(&raw, &receipt).unwrap(),
            b"120.00"
        );
    }
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
            .unwrap();
    }
    for args in [
        json!({"timer":"urn"}),
        json!({"timer":"rejuvenator","patch":"old"}),
    ] {
        let missing = tools
            .execute(
                &query,
                &context,
                None,
                "missing",
                &request(ToolName::GetTimer, args),
            )
            .unwrap();
        assert!(missing.result.is_error);
        assert!(missing.result.evidence_ids.is_empty() && missing.dependencies.is_empty());
    }
    let mut history = query.clone();
    history.patch = Some("old".into());
    assert!(
        tools
            .execute(&history, &context, None, "history", &timer_request)
            .unwrap()
            .result
            .is_error
    );
    let mut denied = context.clone();
    denied.principal.provider_egress.clear();
    assert!(matches!(
        tools.execute(&query, &denied, None, "denied", &timer_request),
        Err(PortError::PermissionDenied(_))
    ));
    runtime.block_on(publish_timer(&admin, "timer-r2", true));
    assert!(tools
        .validate_dependencies(
            &query,
            &context,
            None,
            &execution.dependencies,
            ToolValidationPurpose::Cache
        )
        .is_err());
    let bounded = tools
        .execute(&query, &context, None, "bounded", &timer_request)
        .unwrap();
    assert!(!bounded.result.is_error);
    assert!(serde_json::to_vec(&bounded.result).unwrap().len() < MAX_BYTES);
    assert_eq!(bounded.result.result["truncated"], true);
    assert!(bounded.result.result["omitted_count"].as_u64().unwrap() > 0);
    let facts = bounded.result.result["facts"].as_array().unwrap();
    assert!(facts.len() <= 32 && !facts.is_empty());
    assert!(facts
        .iter()
        .all(|f| f["value"] == "120.00"
            && f["condition"]["flags"][0].as_str().unwrap().len() == 2000));
    println!(
        "P3_TIMER_TOOL_RESULT_BYTES={} P3_TIMER_EXECUTION_BYTES={}",
        serde_json::to_vec(&bounded.result).unwrap().len(),
        serde_json::to_vec(&bounded).unwrap().len()
    );
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
