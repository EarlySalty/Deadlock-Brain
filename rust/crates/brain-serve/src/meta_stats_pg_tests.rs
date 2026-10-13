use super::*;

fn snapshot_bytes(wins: u64) -> Vec<u8> {
    format!(
        "[{{\"hero_id\":1,\"bucket\":0,\"wins\":{wins},\"matches\":1000}},{{\"hero_id\":2,\"bucket\":0,\"wins\":7,\"matches\":20}}]"
    )
    .into_bytes()
}

fn receipt(kind: &str, locator: String, value: &[u8]) -> Receipt {
    Receipt {
        locator_kind: kind.into(),
        locator,
        value_sha256: Sha256::digest(value).into(),
    }
}

async fn publish_snapshot(admin: &PgPool, revision: &str, wins: u64, hours_ago: i64) {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("deadlock-api-meta").await.unwrap();
    let bytes = snapshot_bytes(wins);
    let raw = store
        .capture(
            run,
            "deadlock-api-meta",
            revision,
            "hero-stats.json",
            bytes.clone(),
        )
        .await
        .unwrap();
    let rows: Vec<&serde_json::value::RawValue> = serde_json::from_slice(&bytes).unwrap();
    let set = FactSet {
        entities: vec![Entity {
            kind: "meta_stats".into(),
            key: "meta_hero_stats".into(),
            receipt: receipt("raw_bytes", String::new(), &bytes),
        }],
        facts: rows
            .iter()
            .enumerate()
            .map(|(index, row)| Fact {
                entity_key: "meta_hero_stats".into(),
                stat_key: format!("hero_{}", index + 1),
                value: FactValue::Text(row.get().into()),
                receipt: receipt("json_pointer", format!("/{index}"), row.get().as_bytes()),
            })
            .collect(),
        ..FactSet::default()
    };
    store
        .apply(run, &raw, "v1:10:meta_stats:1:1", &set)
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
    sqlx::query(
        "UPDATE ops.ingest_run SET started=clock_timestamp()-make_interval(hours=>$2::int) WHERE id=$1",
    )
    .bind(run)
    .bind(hours_ago as i32)
    .execute(admin)
    .await
    .unwrap();
}

async fn publish_identity(admin: &PgPool) {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("fixture").await.unwrap();
    let bytes = br#"{"id":2}"#.to_vec();
    let raw = store
        .capture(run, "fixture", "assets-r1", "heroes.json", bytes.clone())
        .await
        .unwrap();
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: "hero_meta_fixture".into(),
            receipt: receipt("raw_bytes", String::new(), &bytes),
        }],
        facts: vec![Fact {
            entity_key: "hero_meta_fixture".into(),
            stat_key: "identity_game_id".into(),
            value: FactValue::Text("2".into()),
            receipt: receipt("json_pointer", "/id".into(), b"2"),
        }],
        ..FactSet::default()
    };
    store.apply(run, &raw, "fixture_v1", &set).await.unwrap();
    store
        .finish_run(run, true, Counts::default())
        .await
        .unwrap();
}

#[test]
fn meta_stats_tool_reads_cited_snapshots_without_live_calls() {
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
    let (admin, pool, entity) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2).connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        for (index, hours_ago) in [24 * 40, 24 * 31, 24 * 8, 25, 1, 0].into_iter().enumerate() {
            let revision = format!("2026-10-{:02}T{:02}:00:00Z", index + 1, index);
            publish_snapshot(&admin, &revision, 500 + index as u64, hours_ago).await;
        }
        publish_identity(&admin).await;
        let entity: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key='hero_meta_fixture'")
            .fetch_one(&admin).await.unwrap();
        let pool = tool_pool(options.clone().username("brain_readonly"), Duration::from_secs(2));
        (admin, pool, entity)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let execution = tools
        .execute(
            &query(),
            &context(),
            None,
            "meta",
            &request(ToolName::GetMetaStats, json!({"kind":"hero","game_id":1})),
        )
        .unwrap();
    assert!(!execution.result.is_error, "{}", execution.result.result);
    let facts = execution.result.result["facts"].as_array().unwrap();
    let snapshots: Vec<_> = facts
        .iter()
        .map(|fact| fact["snapshot"].as_str().unwrap())
        .collect();
    assert_eq!(snapshots, ["latest", "24h", "7d", "30d"]);
    let wins: Vec<_> = facts
        .iter()
        .map(|fact| fact["value"]["wins"].as_u64().unwrap())
        .collect();
    assert_eq!(wins, [505, 503, 502, 501]);
    for fact in facts {
        assert!(fact["patch"].is_null());
        assert!(fact["snapshot_at"].as_str().unwrap().ends_with('Z'));
        assert!(fact["source_revision"]
            .as_str()
            .unwrap()
            .starts_with("2026-10-"));
        assert_eq!(fact["condition"]["game_mode"], "normal");
    }
    assert_eq!(execution.dependencies[0].evidence.len(), 4);
    assert!(serde_json::to_vec(&execution.result).unwrap().len() < MAX_BYTES);

    let by_entity = tools
        .execute(
            &query(),
            &context(),
            None,
            "meta",
            &request(
                ToolName::GetMetaStats,
                json!({"kind":"hero","entity":entity}),
            ),
        )
        .unwrap();
    assert!(!by_entity.result.is_error, "{}", by_entity.result.result);
    assert_eq!(by_entity.result.result["game_id"], 2);
    assert_eq!(by_entity.result.result["identity"]["value"], "2");
    assert_eq!(by_entity.dependencies[0].evidence.len(), 5);

    for arguments in [
        json!({"kind":"hero","game_id":999}),
        json!({"kind":"item","game_id":1}),
        json!({"kind":"item","entity":entity}),
    ] {
        let missing = tools
            .execute(
                &query(),
                &context(),
                None,
                "meta",
                &request(ToolName::GetMetaStats, arguments),
            )
            .unwrap();
        assert!(missing.result.is_error);
        assert_eq!(missing.result.result["error"], "meta_stats_not_found");
    }

    let plan: Vec<String> = runtime.block_on(async {
        let mut connection = admin.acquire().await.unwrap();
        sqlx::query("SET enable_seqscan=off")
            .execute(&mut *connection)
            .await
            .unwrap();
        sqlx::query_scalar(&format!("EXPLAIN {}", meta_stats_tools::QUERY))
            .bind("meta_hero_stats")
            .bind("hero_1")
            .fetch_all(&mut *connection)
            .await
            .unwrap()
    });
    assert!(
        plan.iter().any(|line| line.contains("fact_history")),
        "{plan:#?}"
    );
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
