use super::*;

#[test]
fn published_complete_responses_restore_replayed_slots_and_bind_reused_chunks() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let (admin, pool) = runtime.block_on(database(&options(&pg)));
    let store = GameFactsStore::new(admin.clone());
    runtime
        .block_on(ingest(
            &store,
            "deadlock-data",
            [Ok(InputObject {
                revision: "public-core-hero-capture".into(),
                path: "data/json/hero-data.json".into(),
                bytes: captures::read("hero-data.json.gz"),
            })],
            EXTRACTORS,
        ))
        .expect("published heroes");
    let a = captures::read("heroes-en.json.gz");
    let mut changed: Vec<Value> = serde_json::from_slice(&a).expect("captured heroes");
    changed[0]["items"]["signature1"] = changed[1]["items"]["signature1"].clone();
    let b = serde_json::to_vec(&changed).expect("changed complete response");
    let cache = cache(&a);
    let hero_key = changed[0]["class_name"]
        .as_str()
        .expect("hero source identity");
    let hero_id: i64 = runtime
        .block_on(
            sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
                .bind(hero_key)
                .fetch_one(&admin),
        )
        .expect("published hero identity");
    let original: Vec<Value> = serde_json::from_slice(&a).expect("original source heroes");
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let padded = |leading: u8, suffix: &[u8]| {
        let mut response = vec![b' '; brain_job::limits::RAW_BYTES];
        response[0] = leading;
        response.extend_from_slice(suffix);
        response
    };
    for (label, bytes) in [
        ("exact original A", a.clone()),
        ("changed B", b.clone()),
        ("exact original A replay", a.clone()),
        ("A", padded(b' ', &a)),
        ("B", padded(b' ', &b)),
        ("A replay", padded(b' ', &a)),
        ("other leading chunk", padded(b'\n', &b)),
        ("shared chunks", padded(b'\n', &a)),
    ] {
        std::fs::write(cache.path().join("heroes-en.json"), &bytes).expect("complete response");
        let report = runtime
            .block_on(ingest(
                &store,
                "assets",
                brain_job::assets::source::load(cache.path(), false),
                EXTRACTORS,
            ))
            .expect("successful publication");
        assert_eq!(report.counts.rejected, 0);
        let expected = format!("sha256:{}", hex::encode(Sha256::digest(&bytes)));
        let revision: String = runtime
            .block_on(
                sqlx::query_scalar("SELECT revision FROM serve.p2_hero_slot_source")
                    .fetch_one(&admin),
            )
            .expect("published complete-response anchor");
        assert_eq!(revision, expected, "{label}: source revision");
        let total: i64 = runtime
            .block_on(
                sqlx::query_scalar("SELECT count(*) FROM serve.p2_hero_slot").fetch_one(&admin),
            )
            .expect("published slots");
        assert_eq!(total, 164, "{label}: restored mappings");
        let expected_hero = if bytes.ends_with(&b) {
            &changed[0]
        } else {
            &original[0]
        };
        let expected_ability: i64 = runtime
            .block_on(
                sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
                    .bind(
                        expected_hero["items"]["signature1"]
                            .as_str()
                            .expect("explicit source reference"),
                    )
                    .fetch_one(&admin),
            )
            .expect("published ability identity");
        let (query, context) = query_context();
        let (call, request) = call(ToolName::GetAbility, json!({"hero":hero_id,"slot":1}));
        let execution = tools
            .execute(&query, &context, None, &call.id, &request)
            .unwrap_or_else(|error| {
                panic!("{label}: actual 500ms readonly replay selector: {error}")
            });
        assert!(!execution.result.is_error, "{label}: visible selection");
        assert_eq!(
            execution.result.result["identity"]["entity_id"],
            expected_ability
        );
        assert_eq!(
            execution.result.result["abilities"]["links"][0]["source_revision"],
            expected
        );
        let original_ability: i64 = runtime
            .block_on(
                sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
                    .bind(
                        original[0]["items"]["signature1"]
                            .as_str()
                            .expect("original ability key"),
                    )
                    .fetch_one(&admin),
            )
            .expect("original published ability");
        let (direct_call, direct_request) =
            self::call(ToolName::GetAbility, json!({"ability":original_ability}));
        let direct = tools
            .execute(&query, &context, None, &direct_call.id, &direct_request)
            .expect("direct card after mapping replacement");
        assert!(!direct.result.is_error);
        let linked = direct.result.result["links"]["links"]
            .as_array()
            .expect("direct links")
            .iter()
            .any(|link| link["kind"] == "has_ability" && link["from_entity_id"] == hero_id);
        assert_eq!(
            linked,
            !bytes.ends_with(&b),
            "{label}: current direct hero link"
        );
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
            ToolValidationPurpose::Cache,
        ] {
            tools
                .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
                .expect("restored published dependencies");
        }
    }
    let counts = "SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.relation)";
    let before: (i64, i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("published counts");
    let replay = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("unchanged final response replay");
    assert_eq!(replay.counts.accepted, 0);
    assert_eq!(replay.counts.rejected, 0);
    let after: (i64, i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("replayed counts");
    assert_eq!(before, after);
    let source_before: String = runtime
        .block_on(
            sqlx::query_scalar("SELECT revision FROM serve.p2_hero_slot_source").fetch_one(&admin),
        )
        .expect("last valid source");
    for malformed in [b"[not-json".as_slice(), b"{}".as_slice()] {
        std::fs::write(cache.path().join("heroes-en.json"), malformed)
            .expect("malformed replacement");
        let report = runtime
            .block_on(ingest(
                &store,
                "assets",
                brain_job::assets::source::load(cache.path(), false),
                EXTRACTORS,
            ))
            .expect("quarantined replacement");
        assert_eq!(report.counts.accepted, 0);
        assert_eq!(report.counts.rejected, 1);
        let source: String = runtime
            .block_on(
                sqlx::query_scalar("SELECT revision FROM serve.p2_hero_slot_source")
                    .fetch_one(&admin),
            )
            .expect("retained source");
        assert_eq!(source, source_before);
        let total: i64 = runtime
            .block_on(
                sqlx::query_scalar("SELECT count(*) FROM serve.p2_hero_slot").fetch_one(&admin),
            )
            .expect("retained mappings");
        assert_eq!(total, 164);
        let (query, context) = query_context();
        let (call, request) = call(ToolName::GetAbility, json!({"hero":hero_id,"slot":1}));
        let execution = tools
            .execute(&query, &context, None, &call.id, &request)
            .expect("retained live selector");
        assert!(!execution.result.is_error);
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
            ToolValidationPurpose::Cache,
        ] {
            tools
                .validate_dependencies(&query, &context, None, &execution.dependencies, purpose)
                .expect("retained published dependencies");
        }
    }
    std::fs::write(cache.path().join("heroes-en.json"), b"[]").expect("published empty source");
    runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("empty response publication");
    let total: i64 = runtime
        .block_on(sqlx::query_scalar("SELECT count(*) FROM serve.p2_hero_slot").fetch_one(&admin))
        .expect("empty mapping selection");
    assert_eq!(total, 0);
    let retired_ability: i64 = runtime
        .block_on(
            sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
                .bind(
                    original[0]["items"]["signature1"]
                        .as_str()
                        .expect("original ability key"),
                )
                .fetch_one(&admin),
        )
        .expect("published ability remains");
    let (query, context) = query_context();
    let (call, request) = call(ToolName::GetAbility, json!({"ability":retired_ability}));
    let direct = tools
        .execute(&query, &context, None, &call.id, &request)
        .expect("direct ability card after empty source");
    assert!(!direct.result.is_error);
    assert_eq!(direct.result.result["links"]["omitted_count"], 0);
    assert_eq!(direct.result.result["links"]["truncated"], false);
    assert!(
        direct.result.result["links"]["links"]
            .as_array()
            .expect("card links")
            .iter()
            .all(|link| link["kind"] != "has_ability"),
        "direct card must not expose retired hero links"
    );
    let peers = serde_json::to_vec(&changed[1..]).expect("removed hero response");
    std::fs::write(cache.path().join("heroes-en.json"), peers).expect("removed hero");
    runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("removal publication");
    let total: i64 = runtime
        .block_on(sqlx::query_scalar("SELECT count(*) FROM serve.p2_hero_slot").fetch_one(&admin))
        .expect("surviving mapping selection");
    assert_eq!(total, 160);
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
