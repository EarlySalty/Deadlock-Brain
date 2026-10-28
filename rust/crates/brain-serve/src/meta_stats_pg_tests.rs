use super::*;

fn snapshot_bytes(wins: u64, with_second: bool) -> Vec<u8> {
    let second = if with_second {
        ",{\"hero_id\":2,\"bucket\":0,\"wins\":7,\"matches\":20}"
    } else {
        ""
    };
    format!("[{{\"hero_id\":1,\"bucket\":0,\"wins\":{wins},\"matches\":1000}}{second}]")
        .into_bytes()
}

fn receipt(kind: &str, locator: String, value: &[u8]) -> Receipt {
    Receipt {
        locator_kind: kind.into(),
        locator,
        value_sha256: Sha256::digest(value).into(),
    }
}

async fn publish_snapshot(admin: &PgPool, revision: &str, wins: u64, with_second: bool) {
    publish_capture(
        admin,
        ("hero-stats.json", "meta_hero_stats", "hero"),
        revision,
        snapshot_bytes(wins, with_second),
    )
    .await;
}

async fn publish_capture(
    admin: &PgPool,
    (path, entity, prefix): (&str, &str, &str),
    revision: &str,
    bytes: Vec<u8>,
) -> i64 {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("deadlock-api-meta").await.unwrap();
    let raw = store
        .capture(run, "deadlock-api-meta", revision, path, bytes.clone())
        .await
        .unwrap();
    let rows: Vec<&serde_json::value::RawValue> = serde_json::from_slice(&bytes).unwrap();
    let set = FactSet {
        entities: vec![Entity {
            kind: "meta_stats".into(),
            key: entity.into(),
            receipt: receipt("raw_bytes", String::new(), &bytes),
        }],
        facts: rows
            .iter()
            .enumerate()
            .map(|(index, row)| Fact {
                entity_key: entity.into(),
                stat_key: format!("{prefix}_{}", index + 1),
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
    run
}

async fn hour_before(admin: &PgPool, back: &str) -> String {
    sqlx::query_scalar(
        "SELECT to_char(date_trunc('hour',now(),'UTC') AT TIME ZONE 'UTC'-$1::interval,'YYYY-MM-DD\"T\"HH24:00:00\"Z\"')",
    )
    .bind(back)
    .fetch_one(admin)
    .await
    .unwrap()
}

fn repeated_capture_ranks_by_its_own_capture_hour(
    runtime: &tokio::runtime::Runtime,
    admin: &PgPool,
    tools: &PgTools<Delegate>,
) {
    let item = ("item-stats.json", "meta_item_stats", "item");
    let first = br#"[{"item_id":1,"wins":40,"matches":100}]"#.to_vec();
    let second = br#"[{"item_id":1,"wins":60,"matches":100}]"#.to_vec();
    let (old, middle, repeated_hour) = runtime.block_on(async {
        let old = hour_before(admin, "10 days").await;
        let middle = hour_before(admin, "3 days").await;
        let fresh = hour_before(admin, "0 hours").await;
        publish_capture(admin, item, &old, first.clone()).await;
        publish_capture(admin, item, &middle, second).await;
        let repeated = publish_capture(admin, item, &fresh, first).await;
        let repeated_hour: String = sqlx::query_scalar(
            "SELECT to_char(date_trunc('hour',started,'UTC') AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:00:00\"Z\"') FROM ops.ingest_run WHERE id=$1",
        )
        .bind(repeated)
        .fetch_one(admin)
        .await
        .unwrap();
        (old, middle, repeated_hour)
    });
    let execution = tools
        .execute(
            &query(),
            &context(),
            None,
            "meta",
            &request(ToolName::GetMetaStats, json!({"kind":"item","game_id":1})),
        )
        .unwrap();
    assert!(!execution.result.is_error, "{}", execution.result.result);
    let facts = execution.result.result["facts"].as_array().unwrap();
    let points: Vec<_> = facts
        .iter()
        .map(|fact| {
            (
                fact["snapshot"].as_str().unwrap(),
                fact["value"]["wins"].as_u64().unwrap(),
                fact["snapshot_at"].as_str().unwrap(),
                fact["source_revision"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        points,
        [
            ("latest", 40, repeated_hour.as_str(), old.as_str()),
            ("24h", 60, middle.as_str(), middle.as_str()),
            ("7d", 40, old.as_str(), old.as_str()),
        ]
    );
    assert_eq!(facts[0]["condition"]["matches_until"], repeated_hour);
    assert_eq!(facts[0]["condition"]["min_matches"], 20);
}

const OLDER_HISTORY: &str = r#"DO $history$
DECLARE run bigint; object bigint; cited bigint; stats bigint;
BEGIN
    SELECT entity_id INTO stats FROM serve.meta_stat_set WHERE stat_set='meta_hero_stats';
    FOR hour IN 1001..10000 LOOP
        INSERT INTO ops.ingest_run(source) VALUES('deadlock-api-meta') RETURNING id INTO run;
        INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id)
        VALUES('deadlock-api-meta',
            to_char((timestamptz '2026-10-06 05:00:00+00'-make_interval(hours=>hour)) AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:00:00"Z"'),
            'hero-stats.json',sha256('history'::bytea),sha256(hour::text::bytea),'\x00'::bytea,run)
        RETURNING id INTO object;
        INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256)
        VALUES(object,run,'v1:10:meta_stats:1:1','json_pointer','/0',sha256('history'::bytea)) RETURNING id INTO cited;
        INSERT INTO canon.fact(entity_id,stat_key,value_text,valid_from_run,receipt_id)
        VALUES(stats,'hero_1','{"hero_id":1,"bucket":0,"wins":1,"matches":2}',run,cited);
        UPDATE ops.ingest_run SET status='succeeded' WHERE id=run;
    END LOOP;
END
$history$"#;

async fn publish_hero(admin: &PgPool, key: &str) -> i64 {
    let store = GameFactsStore::new(admin.clone());
    let run = store.begin_run("fixture").await.unwrap();
    let bytes = format!("{{\"class_name\":\"{key}\"}}").into_bytes();
    let raw = store
        .capture(
            run,
            "fixture",
            "assets-r1",
            &format!("{key}.json"),
            bytes.clone(),
        )
        .await
        .unwrap();
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: key.into(),
            receipt: receipt("raw_bytes", String::new(), &bytes),
        }],
        ..FactSet::default()
    };
    store.apply(run, &raw, "fixture_v1", &set).await.unwrap();
    store
        .finish_run(run, true, Counts::default())
        .await
        .unwrap();
    sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
        .bind(key)
        .fetch_one(admin)
        .await
        .unwrap()
}

async fn assets_hero(admin: &PgPool, key: &str, game_id: u64, flags: Value) -> i64 {
    let entity = publish_hero(admin, key).await;
    let mut flags = flags;
    flags["id"] = json!(game_id);
    crate::assets_fixture::flagged(admin, key, flags).await;
    entity
}

fn playable_flags() -> Value {
    json!({"player_selectable":true,"disabled":false,"in_development":false})
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
        for (wins, revision) in [
            (502, "2026-09-28T05:00:00Z"),
            (505, "2026-10-06T05:00:00Z"),
            (500, "2026-08-27T05:00:00Z"),
            (504, "2026-10-06T04:00:00Z"),
            (501, "2026-09-05T05:00:00Z"),
            (503, "2026-10-05T04:00:00Z"),
        ] {
            publish_snapshot(&admin, revision, wins, wins != 505).await;
        }
        sqlx::raw_sql(OLDER_HISTORY).execute(&admin).await.unwrap();
        assets_hero(&admin, "hero_meta_first", 1, playable_flags()).await;
        let entity = assets_hero(&admin, "hero_meta_fixture", 2, playable_flags()).await;
        let pool = tool_pool(options.clone().username("brain_readonly"), Duration::from_secs(2), 1);
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
        assert_eq!(fact["snapshot_at"], fact["source_revision"]);
        assert_eq!(fact["condition"]["game_mode"], "normal");
        assert!(fact["condition"].get("min_matches").is_none());
        assert!(fact["condition"]["matches_since"]
            .as_str()
            .unwrap()
            .ends_with("T00:00:00Z"));
        assert_eq!(fact["condition"]["matches_until"], fact["source_revision"]);
    }
    let times: Vec<_> = facts
        .iter()
        .map(|fact| fact["snapshot_at"].as_str().unwrap())
        .collect();
    assert_eq!(
        times,
        [
            "2026-10-06T05:00:00Z",
            "2026-10-05T04:00:00Z",
            "2026-09-28T05:00:00Z",
            "2026-09-05T05:00:00Z"
        ]
    );
    assert_eq!(
        facts[0]["condition"]["matches_since"],
        "2026-09-06T00:00:00Z"
    );
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
    let second_latest = &by_entity.result.result["facts"][0];
    assert_eq!(second_latest["snapshot"], "latest");
    assert_eq!(second_latest["value"]["hero_id"], 2);
    assert!(
        second_latest["snapshot_at"].as_str().unwrap() < facts[0]["snapshot_at"].as_str().unwrap()
    );
    assert_eq!(second_latest["source_revision"], "2026-10-06T04:00:00Z");

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

    repeated_capture_ranks_by_its_own_capture_hour(&runtime, &admin, &tools);

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
        plan.iter().any(|line| line.contains("fact_history"))
            && plan
                .iter()
                .any(|line| line.contains("meta_stat_capture_order")),
        "{plan:#?}"
    );
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}

#[test]
fn meta_stats_treat_hidden_heroes_as_unknown() {
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
    let (admin, pool, visible, hidden) = runtime.block_on(async {
        let admin = sqlx::postgres::PgPoolOptions::new().max_connections(2).connect_with(options.clone()).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
        brain_storage::PgStore::new(admin.clone()).migrate_gamefacts().await.unwrap();
        publish_snapshot(&admin, "2026-10-06T05:00:00Z", 505, true).await;
        let visible = assets_hero(&admin, "hero_meta_visible", 1, playable_flags()).await;
        let hidden = assets_hero(
            &admin,
            "hero_meta_hidden",
            2,
            json!({"player_selectable":false,"disabled":true,"in_development":false}),
        )
        .await;
        for (game_id, key, restriction, playable) in [
            (3, "hero_meta_prerelease", json!({"prerelease_only":true}), false),
            (4, "hero_meta_assigned", json!({"assigned_players_only":null}), false),
            (5, "hero_meta_labs", json!({"in_hero_labs":true}), false),
            (6, "hero_meta_labs_legacy", json!({"InHeroLabs":"false"}), false),
            (7, "hero_meta_unrestricted", json!({"prerelease_only":false,"assigned_players_only":false,"in_hero_labs":false}), true),
        ] {
            let mut flags = playable_flags();
            for (field, value) in restriction.as_object().unwrap() {
                flags[field] = value.clone();
            }
            let entity = assets_hero(&admin, key, game_id, flags).await;
            let served: bool = sqlx::query_scalar("SELECT serve.hero_is_playable($1)").bind(entity).fetch_one(&admin).await.unwrap();
            let by_game_id: bool = sqlx::query_scalar("SELECT serve.hero_game_id_is_playable($1)").bind(game_id.to_string()).fetch_one(&admin).await.unwrap();
            let identity: Option<Value> = sqlx::query_scalar("SELECT serve.hero_game_identity($1)").bind(entity).fetch_one(&admin).await.unwrap();
            assert_eq!((served, by_game_id, identity.is_some()), (playable, playable, playable), "{key}");
        }
        let identity_facts: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.fact WHERE stat_key='identity_game_id'").fetch_one(&admin).await.unwrap();
        assert_eq!(identity_facts, 0);
        let pool = tool_pool(options.clone().username("brain_readonly"), Duration::from_secs(2), 1);
        (admin, pool, visible, hidden)
    });
    let tools = PgTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let execute = |arguments: Value| {
        tools
            .execute(
                &query(),
                &context(),
                None,
                "meta",
                &request(ToolName::GetMetaStats, arguments),
            )
            .unwrap()
            .result
    };
    for arguments in [
        json!({"kind":"hero","game_id":1}),
        json!({"kind":"hero","entity":visible}),
    ] {
        let found = execute(arguments.clone());
        assert!(!found.is_error, "{arguments} {}", found.result);
        assert_eq!(found.result["facts"][0]["value"]["hero_id"], 1);
    }
    for arguments in [
        json!({"kind":"hero","game_id":2}),
        json!({"kind":"hero","entity":hidden}),
    ] {
        let missing = execute(arguments.clone());
        assert!(missing.is_error, "{arguments}");
        assert_eq!(
            missing.result["error"], "meta_stats_not_found",
            "{arguments}"
        );
        assert!(missing.evidence_ids.is_empty(), "{arguments}");
    }
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
