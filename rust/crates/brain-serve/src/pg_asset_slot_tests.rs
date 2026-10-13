use super::*;
use brain_contracts::canonical_receipt::CanonicalReceipt;

#[path = "../../brain-job/tests/fixtures/p2-slots/capture.rs"]
mod captures;

fn cache(heroes: &[u8]) -> tempfile::TempDir {
    let cache = tempfile::tempdir().expect("own source cache");
    let items = std::env::var_os("BRAIN_P2_ITEMS_CAPTURE")
        .map(|path| std::fs::read(path).expect("explicit public Items capture"))
        .unwrap_or_else(|| captures::read("items-en.json.gz"));
    std::fs::write(cache.path().join("en.json"), items).expect("captured abilities");
    std::fs::write(cache.path().join("de.json"), b"[]").expect("independent locale");
    std::fs::write(cache.path().join("heroes-en.json"), heroes).expect("captured heroes");
    cache
}

#[test]
fn captured_hero_slots_reach_the_real_executor_and_revalidate_fail_closed() {
    let pg = scratch_pg::ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let (admin, pool) = runtime.block_on(database(&options(&pg)));
    let store = GameFactsStore::new(admin.clone());
    let heroes = runtime
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
        .expect("genuine P1 hero extraction");
    assert_eq!(heroes.counts.accepted, 1);
    assert_eq!(heroes.counts.rejected, 0);
    let hero_bytes = captures::read("heroes-en.json.gz");
    let cache = cache(&hero_bytes);
    let report = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("actual captured source ingestion");
    let item_rows: Vec<Value> = serde_json::from_slice(
        &std::fs::read(cache.path().join("en.json")).expect("public Items snapshot"),
    )
    .expect("captured Items records");
    let supported = item_rows
        .iter()
        .filter(|row| matches!(row["type"].as_str(), Some("ability" | "upgrade")))
        .count() as i64;
    assert_eq!(report.counts.accepted, supported + 42);
    assert_eq!(report.counts.rejected, 0);
    let counts = "SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.relation)";
    let before: (i64, i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("counts");
    assert_eq!(before.2, 164);
    let replay = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("unchanged replay");
    assert_eq!(replay.counts.accepted, 0);
    assert_eq!(replay.counts.rejected, 0);
    let after: (i64, i64, i64) = runtime
        .block_on(sqlx::query_as(counts).fetch_one(&admin))
        .expect("counts");
    assert_eq!(before, after);
    let tools = PgTools::new(NoLegacy, pool.clone(), runtime.handle().clone());
    let captured: Vec<Value> = serde_json::from_slice(&hero_bytes).expect("captured heroes");
    let mut selected = None;
    for hero in &captured {
        let hero_id: i64 = runtime
            .block_on(
                sqlx::query_scalar(
                    "SELECT id FROM serve.current_entity WHERE key=$1 AND kind='hero'",
                )
                .bind(hero["class_name"].as_str().expect("source ID"))
                .fetch_one(&admin),
            )
            .expect("P1 published identity");
        for slot in 1..=4 {
            let ability_key = hero["items"][format!("signature{slot}")]
                .as_str()
                .expect("explicit class");
            let expected: i64 = runtime
                .block_on(
                    sqlx::query_scalar(
                        "SELECT id FROM serve.current_entity WHERE key=$1 AND kind='ability'",
                    )
                    .bind(ability_key)
                    .fetch_one(&admin),
                )
                .expect("P2 published identity");
            let (query, context) = query_context();
            let (call, request) = call(ToolName::GetAbility, json!({"hero":hero_id,"slot":slot}));
            let execution = tools.execute(&query, &context, None, &call.id, &request)
                .unwrap_or_else(|error| {
                    let plan: Value = runtime.block_on(sqlx::query_scalar(
                        "EXPLAIN (ANALYZE, FORMAT JSON) SELECT link FROM serve.p2_hero_slot WHERE from_entity_id=$1 AND slot=$2 LIMIT 2")
                        .bind(hero_id).bind(slot).fetch_one(&admin)).expect("failed slot plan");
                    println!("slot planning_ms={}, execution_ms={}", plan[0]["Planning Time"],plan[0]["Execution Time"]);
                    public_plan_metrics(&plan[0]["Plan"]);
                    panic!("actual 500ms readonly executor: {error}");
                });
            assert!(
                !execution.result.is_error,
                "{} slot {}",
                hero["class_name"], slot
            );
            assert_eq!(execution.result.result["identity"]["entity_id"], expected);
            execution
                .validate_for(&call, &request, None)
                .expect("canonical dependency resolver");
            assert!(
                serde_json::to_vec(&execution.result)
                    .expect("bounded result")
                    .len()
                    < MAX_BYTES
            );
            assert!(execution.result.evidence_ids.len() <= MAX_EVIDENCE);
            let link = &execution.result.result["abilities"]["links"][0];
            let qualifier = &link["qualifiers"][0];
            assert_eq!(qualifier["key"], "slot");
            assert_eq!(qualifier["value"], slot);
            for row in [link, qualifier] {
                let citation = row["citation"].as_str().expect("canonical evidence ID");
                assert!(execution
                    .result
                    .evidence_ids
                    .iter()
                    .any(|id| id == citation));
                let evidence = execution.dependencies[0]
                    .evidence
                    .iter()
                    .find(|e| e.evidence_id == citation)
                    .expect("canonical evidence");
                let receipt =
                    CanonicalReceipt::from_evidence(evidence).expect("shared canonical formatter");
                let metadata = serde_json::to_value(receipt).expect("canonical metadata");
                assert_eq!(metadata["locator"], format!("/items/signature{slot}"));
                let bytes: Vec<u8> = runtime
                    .block_on(
                        sqlx::query_scalar("SELECT bytes FROM raw.object WHERE id=$1")
                            .bind(metadata["raw_object_id"].as_i64().expect("raw ID"))
                            .fetch_one(&admin),
                    )
                    .expect("own raw bytes");
                let raw_receipt = brain_storage::gamefacts::Receipt {
                    locator_kind: "json_pointer".into(),
                    locator: metadata["locator"].as_str().expect("pointer").into(),
                    value_sha256: hex::decode(metadata["value_sha256"].as_str().expect("hash"))
                        .expect("hex")
                        .try_into()
                        .expect("SHA256"),
                };
                let value = brain_storage::gamefacts::receipts::value_bytes(&bytes, &raw_receipt)
                    .expect("original lexeme");
                assert_eq!(
                    <[u8; 32]>::from(Sha256::digest(&value)),
                    raw_receipt.value_sha256
                );
                assert_eq!(
                    serde_json::from_slice::<String>(&value).expect("ability source ID"),
                    ability_key
                );
            }
            if selected.is_none() {
                for purpose in [
                    ToolValidationPurpose::Provider,
                    ToolValidationPurpose::Publication,
                    ToolValidationPurpose::Cache,
                ] {
                    tools
                        .validate_dependencies(
                            &query,
                            &context,
                            None,
                            &execution.dependencies,
                            purpose,
                        )
                        .expect("published selection revalidation");
                }
                selected = Some((hero_id, execution));
            }
        }
    }
    let (hero_id, old) = selected.expect("164 real selections");
    let (query, context) = query_context();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(&query, &context, None, &old.dependencies, purpose)
            .expect("unchanged mapping revalidates with a fresh deadline");
    }
    let mut changed = captured.clone();
    changed[0]["items"]["signature1"] = captured[1]["items"]["signature1"].clone();
    std::fs::write(
        cache.path().join("heroes-en.json"),
        serde_json::to_vec(&changed).expect("changed snapshot"),
    )
    .expect("source replacement");
    let changed_report = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("mapping change");
    assert_eq!(changed_report.counts.rejected, 0);
    let (query, context) = query_context();
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(tools
            .validate_dependencies(&query, &context, None, &old.dependencies, purpose)
            .is_err());
    }
    let (call, request) = call(ToolName::GetAbility, json!({"hero":hero_id,"slot":1}));
    assert!(
        !tools
            .execute(&query, &context, None, &call.id, &request)
            .expect("changed selection")
            .result
            .is_error
    );
    let mut unpublished = changed.clone();
    unpublished[0]["name"] = json!("unpublished capture");
    std::fs::write(
        cache.path().join("heroes-en.json"),
        serde_json::to_vec(&unpublished).expect("unpublished snapshot"),
    )
    .expect("source replacement");
    let failed = runtime.block_on(ingest(
        &store,
        "assets",
        brain_job::assets::source::load(cache.path(), false)
            .chain([Err(
                std::io::Error::other("transport fixture failure").into()
            )]),
        EXTRACTORS,
    ));
    assert!(failed.is_err());
    let (query, context) = query_context();
    assert!(
        tools
            .execute(&query, &context, None, &call.id, &request)
            .expect("unpublished fails closed")
            .result
            .is_error
    );
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(tools
            .validate_dependencies(&query, &context, None, &old.dependencies, purpose)
            .is_err());
    }
    changed[0]["items"]["signature1"] = json!("ability_missing_reference");
    changed[1]["items"]["signature4"] = Value::Null;
    changed[2]["items"]["signature4"] = changed[2]["items"]["signature1"].clone();
    changed[3]["class_name"] = json!("hero_missing_reference");
    changed[4]["items"]["signature4"] = json!("x".repeat(brain_job::limits::KEY_BYTES + 1));
    let mut poisoned_bytes = serde_json::to_vec(&changed).expect("poisoned snapshot");
    poisoned_bytes.pop();
    poisoned_bytes.extend_from_slice(
        br#",{"class_name":"hero_duplicate_key","CLASS_NAME":"hero_collision"}]"#,
    );
    std::fs::write(cache.path().join("heroes-en.json"), poisoned_bytes)
        .expect("source replacement");
    let poisoned = runtime
        .block_on(ingest(
            &store,
            "assets",
            brain_job::assets::source::load(cache.path(), false),
            EXTRACTORS,
        ))
        .expect("isolated poison ingestion");
    assert_eq!(poisoned.counts.rejected, 6);
    assert_eq!(poisoned.counts.accepted, 37);
    let (query, context) = query_context();
    let reason: String = runtime.block_on(sqlx::query_scalar("SELECT reason FROM ops.p2_hero_slot_rejection WHERE code='sql_write' ORDER BY id DESC LIMIT 1").fetch_one(&admin)).expect("readable per-record reference reason");
    assert!(!reason.is_empty());
    assert!(
        tools
            .execute(&query, &context, None, &call.id, &request)
            .expect("unresolved fails closed")
            .result
            .is_error
    );
    let invented: i64 = runtime.block_on(sqlx::query_scalar(
        "SELECT count(*) FROM canon.entity WHERE key IN('hero_missing_reference','ability_missing_reference')")
        .fetch_one(&admin)).expect("no invented identities");
    assert_eq!(invented, 0);
    let healthy: i64 = runtime
        .block_on(
            sqlx::query_scalar("SELECT id FROM serve.current_entity WHERE key=$1")
                .bind(
                    captured[5]["class_name"]
                        .as_str()
                        .expect("healthy source ID"),
                )
                .fetch_one(&admin),
        )
        .expect("healthy P1 identity");
    let (healthy_call, healthy_request) =
        super::call(ToolName::GetAbility, json!({"hero":healthy,"slot":4}));
    assert!(
        !tools
            .execute(&query, &context, None, &healthy_call.id, &healthy_request)
            .expect("healthy peer after poison")
            .result
            .is_error
    );
    let status: String = runtime
        .block_on(
            sqlx::query_scalar("SELECT status FROM ops.ingest_run ORDER BY id DESC LIMIT 1")
                .fetch_one(&admin),
        )
        .expect("finished run");
    assert_eq!(status, "succeeded");
    println!("P2 captured heroes=41, mappings=164, actual selections=164, unchanged replay adds=0");
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
}
