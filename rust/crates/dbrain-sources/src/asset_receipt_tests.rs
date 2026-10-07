use super::*;
use brain_storage::asset_mirror::{
    latest_mirrored_client_version, load_mirrored_assets, load_mirrored_assets_for_run,
    load_mirrored_assets_with_receipt,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

fn transport(url: String, raw: Vec<u8>) -> SourceHttpResponse {
    SourceHttpResponse {
        url,
        status: 200,
        content: raw,
        headers: std::collections::BTreeMap::from([(
            "content-type".into(),
            "application/json".into(),
        )]),
        observed_at: 100,
        attempts: 1,
    }
}

async fn scratch_pool(pg: &scratch_pg::ScratchPg) -> sqlx::PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(pg.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    let identity: (Option<String>, String, String) = sqlx::query_as(
        "SELECT inet_server_addr()::text, current_user::text, current_setting('data_directory')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(identity.0, None);
    assert_eq!(identity.1, "brain_core_test");
    assert_eq!(Path::new(&identity.2), pg.directory.join("data"));
    sqlx::raw_sql("CREATE SCHEMA brain")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("wiki_scratch.sql"))
        .execute(&pool)
        .await
        .unwrap();
    pool
}

async fn persist_run(store: &SourceStore<'_>, marker: &str) -> (i64, Value) {
    let version = 6759;
    let run = store.begin_run("assets").await.unwrap();
    let manifest = SourceIr::from_http(
        SOURCE,
        PARSER_REVISION,
        transport(
            format!("{BASE_URL}/v1/assets/steam-info"),
            b" {\"client_version\":6759,\"server_version\":6759}\n".to_vec(),
        ),
    )
    .unwrap();
    let manifest_id = store
        .persist_ir(
            "steam_info/6759",
            "Versionsmanifest",
            &manifest,
            &json!({"role":"client_manifest","client_version":version,"not_patch_mapping":true}),
        )
        .await
        .unwrap();
    let mut endpoints = Map::new();
    let mut count = 0;
    for (kind, endpoint) in resolve_kinds(&[]).unwrap() {
        for language in mirrored_asset_languages(&kind).unwrap() {
            let payload = if kind == "generic_data" {
                json!({"item_price_per_tier":[800,1600,3200,6400],"hero_kill_gold_share_frac":0.1,
                    "zero":0.0,"negative":-0.25,"observation":marker})
            } else {
                json!([{"id":0,"class_name":"test_entity","name":"Prüfdaten",
                    "properties":{"Damage":"40"},"observation":marker}])
            };
            let mut raw = b" \n".to_vec();
            raw.extend(serde_json::to_vec(&payload).unwrap());
            raw.extend(b"\n");
            let ir = prepare_assets(
                &kind,
                transport(asset_url(&kind, endpoint, version, language), raw),
                None,
            )
            .unwrap();
            assert!(
                !ir.is_quarantined(),
                "{kind}/{language}: {:?}",
                ir.validation()
            );
            let language = (!language.is_empty()).then_some(*language);
            let key = mirrored_asset_key(&kind, language).unwrap();
            let document_id = store.persist_ir(&format!("{version}/{key}"), &key, &ir,
                &json!({"kind":kind,"endpoint":endpoint,"client_version":version,"language":language}),
            ).await.unwrap();
            let snapshots = asset_entities(&kind, ir.payload().unwrap()).unwrap().len();
            count += snapshots;
            endpoints.insert(key, json!({"source_document_id":document_id,"snapshots":snapshots,
                "url":ir.provenance().locator,"raw_sha256":ir.provenance().raw_sha256,"validation":ir.validation()}));
        }
    }
    let summary = json!({"client_version":version,"manifest_document_id":manifest_id,
        "manifest_raw_sha256":manifest.provenance().raw_sha256,"parser_revision":PARSER_REVISION,
        "mirrored_at":100,"checked_at":100,"snapshots":count,"endpoints":endpoints,
        "mirror_complete":true,"reused_local_mirror":false});
    store.finish_run(run, "ok", &summary).await.unwrap();
    (run, summary)
}

async fn original_documents(pool: &sqlx::PgPool) -> Vec<(i64, String, Value)> {
    sqlx::query_as(
        "SELECT id, raw_path, to_jsonb(d)-'raw_path' FROM brain.source_documents d ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[tokio::test]
async fn fresh_mirror_moves_verified_originals_without_changing_receipts() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = scratch_pool(&pg).await;
    let old_dir = tempfile::tempdir().unwrap();
    let new_dir = tempfile::tempdir().unwrap();
    let old_store = SourceStore::new(&pool, old_dir.path()).unwrap();
    let (old_run, summary) = persist_run(&old_store, "original").await;
    let before = original_documents(&pool).await;
    let originals: Vec<_> = before
        .iter()
        .map(|(_, path, _)| std::fs::read(path).unwrap())
        .collect();
    let store = SourceStore::new(&pool, new_dir.path()).unwrap();
    let new_run = store.begin_run("assets").await.unwrap();
    let reused = reuse_mirror_if_fresh(&store, summary.clone(), &resolve_kinds(&[]).unwrap(), 101)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reused["reused_local_mirror"], true);
    assert_eq!(reused["checked_at"], 101);
    assert_eq!(reused["mirrored_at"], summary["mirrored_at"]);
    assert_eq!(reused["endpoints"], summary["endpoints"]);
    store.finish_run(new_run, "ok", &reused).await.unwrap();
    let after = original_documents(&pool).await;
    assert_eq!(before.len(), after.len());
    for (((old_id, _, old_document), (new_id, path, document)), raw) in
        before.iter().zip(&after).zip(&originals)
    {
        assert_eq!(old_id, new_id);
        assert_eq!(old_document, document);
        assert!(Path::new(path).starts_with(new_dir.path()));
        assert_eq!(std::fs::read(path).unwrap(), *raw);
        assert_eq!(
            crate::store::stable_hash_bytes(raw),
            document["content_hash"]
        );
    }
    old_dir.close().unwrap();
    let loaded = load_mirrored_assets_with_receipt(&pool, 6759, "items", Some("english"))
        .await
        .unwrap();
    assert_eq!(loaded.receipt.source_run_id, old_run);
    assert_eq!(loaded.payload[0]["observation"], "original");
    assert!(
        load_mirrored_assets_for_run(&pool, old_run, 6759, "items", Some("english"))
            .await
            .is_ok()
    );
    assert!(
        reuse_mirror_if_fresh(&store, reused, &resolve_kinds(&[]).unwrap(), 102)
            .await
            .unwrap()
            .is_some()
    );
    assert_eq!(original_documents(&pool).await, after);
    pool.close().await;
}

#[tokio::test]
async fn identical_ir_repoints_only_raw_path_to_verified_new_originals() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = scratch_pool(&pg).await;
    let old_dir = tempfile::tempdir().unwrap();
    let new_dir = tempfile::tempdir().unwrap();
    let old_store = SourceStore::new(&pool, old_dir.path()).unwrap();
    let raw = b" \n[{\"id\":0,\"name\":\"Original\"}]\n";
    let ir = prepare_assets(
        "items",
        transport(
            asset_url("items", ENDPOINTS[0].1, 6759, "english"),
            raw.to_vec(),
        ),
        None,
    )
    .unwrap();
    let id = old_store
        .persist_ir(
            "6759/items/english",
            "Originaltitel",
            &ir,
            &json!({"kind":"items"}),
        )
        .await
        .unwrap();
    let before = original_documents(&pool).await;
    old_dir.close().unwrap();
    let store = SourceStore::new(&pool, new_dir.path()).unwrap();
    let mut later_response = transport(
        asset_url("items", ENDPOINTS[0].1, 6759, "english"),
        raw.to_vec(),
    );
    later_response.observed_at = 200;
    let repeated = prepare_assets("items", later_response, None).unwrap();
    let reused_id = store
        .persist_ir(
            "6759/items/english",
            "Anderer Titel",
            &repeated,
            &json!({"changed":true}),
        )
        .await
        .unwrap();
    assert_eq!(reused_id, id);
    let after = original_documents(&pool).await;
    assert_eq!(after.len(), 1);
    assert_eq!(before[0].0, after[0].0);
    assert_eq!(before[0].2, after[0].2);
    assert!(Path::new(&after[0].1).starts_with(new_dir.path()));
    assert_eq!(std::fs::read(&after[0].1).unwrap(), raw);
    assert!(store
        .ensure_original(id, SOURCE, ir.provenance().raw_sha256.as_str())
        .await
        .unwrap());
    pool.close().await;
}

#[tokio::test]
async fn missing_or_modified_originals_cannot_reuse_a_fresh_mirror() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = scratch_pool(&pg).await;
    let old_dir = tempfile::tempdir().unwrap();
    let new_dir = tempfile::tempdir().unwrap();
    let old_store = SourceStore::new(&pool, old_dir.path()).unwrap();
    let (_, summary) = persist_run(&old_store, "original").await;
    let before = original_documents(&pool).await;
    let id = summary["endpoints"]["items/english"]["source_document_id"]
        .as_i64()
        .unwrap();
    let (_, original_path, original_document) =
        before.iter().find(|(row_id, _, _)| *row_id == id).unwrap();
    let raw = std::fs::read(original_path).unwrap();
    let store = SourceStore::new(&pool, new_dir.path()).unwrap();
    for replacement in [None, Some(b"[]".as_slice())] {
        if let Some(replacement) = replacement {
            std::fs::write(original_path, replacement).unwrap();
        } else {
            std::fs::remove_file(original_path).unwrap();
        }
        assert!(
            reuse_mirror_if_fresh(&store, summary.clone(), &resolve_kinds(&[]).unwrap(), 101)
                .await
                .unwrap()
                .is_none()
        );
        let after = original_documents(&pool).await;
        let (_, path, document) = after.iter().find(|(row_id, _, _)| *row_id == id).unwrap();
        assert_eq!(path, original_path);
        assert_eq!(document, original_document);
    }
    std::fs::write(original_path, &raw).unwrap();
    assert!(
        reuse_mirror_if_fresh(&store, summary.clone(), &resolve_kinds(&[]).unwrap(), 102)
            .await
            .unwrap()
            .is_some()
    );
    let after = original_documents(&pool).await;
    let (_, path, _) = after.iter().find(|(row_id, _, _)| *row_id == id).unwrap();
    std::fs::write(path, b"tampered").unwrap();
    assert!(
        reuse_mirror_if_fresh(&store, summary.clone(), &resolve_kinds(&[]).unwrap(), 103)
            .await
            .unwrap()
            .is_none()
    );
    std::fs::write(path, &raw).unwrap();
    let mut wrong_hash = summary.clone();
    wrong_hash["endpoints"]["items/english"]["raw_sha256"] = json!("wrong");
    assert!(
        reuse_mirror_if_fresh(&store, wrong_hash, &resolve_kinds(&[]).unwrap(), 103)
            .await
            .unwrap()
            .is_none()
    );
    let mut missing_hash = summary.clone();
    missing_hash["endpoints"]["items/english"]
        .as_object_mut()
        .unwrap()
        .remove("raw_sha256");
    assert!(
        reuse_mirror_if_fresh(&store, missing_hash, &resolve_kinds(&[]).unwrap(), 103)
            .await
            .unwrap()
            .is_none()
    );
    assert!(!store
        .ensure_original(
            id,
            "wrong_source",
            crate::store::stable_hash_bytes(&raw).as_str()
        )
        .await
        .unwrap());
    std::fs::remove_file(path).unwrap();
    std::os::unix::fs::symlink(original_path, path).unwrap();
    assert!(
        reuse_mirror_if_fresh(&store, summary, &resolve_kinds(&[]).unwrap(), 103)
            .await
            .is_err()
    );
    assert!(store
        .ensure_original(id, SOURCE, crate::store::stable_hash_bytes(&raw).as_str())
        .await
        .is_err());
    assert_eq!(std::fs::read(original_path).unwrap(), raw);
    pool.close().await;
}

#[test]
fn global_contracts_preserve_payloads_and_reject_missing_identity_or_empty_data() {
    for kind in ["npc_units", "misc_entities", "modifiers"] {
        let url = asset_url(
            kind,
            endpoint_path(kind).unwrap(),
            6759,
            if kind == "modifiers" { "" } else { "german" },
        );
        assert!(url.contains("client_version=6759"));
        assert_eq!(url.contains("language="), kind != "modifiers");
        for raw in [
            b"[]".as_slice(),
            b"{}",
            b"[{\"class_name\":\"test\"}]",
            b"[{\"id\":false,\"class_name\":\"test\"}]",
        ] {
            assert!(
                prepare_assets(kind, transport(url.clone(), raw.to_vec()), None)
                    .unwrap()
                    .is_quarantined()
            );
        }
    }
    for raw in [b"{}".as_slice(), b"[]", b"null"] {
        assert!(prepare_assets(
            "generic_data",
            transport(
                "https://api.deadlock-api.com/v1/assets/generic-data".into(),
                raw.to_vec()
            ),
            None
        )
        .unwrap()
        .is_quarantined());
    }
    let payload = json!({"item_price_per_tier":[800,1600],"hero_kill_gold_share_frac":0.1,"zero":0.0,"negative":-0.25});
    let ir = prepare_assets(
        "generic_data",
        transport(
            asset_url(
                "generic_data",
                endpoint_path("generic_data").unwrap(),
                6759,
                "german",
            ),
            serde_json::to_vec(&payload).unwrap(),
        ),
        None,
    )
    .unwrap();
    assert_eq!(
        asset_entities("generic_data", ir.payload().unwrap()).unwrap()[0].payload,
        payload
    );
}

#[tokio::test]
async fn receipt_binds_actual_run_manifest_original_hash_and_independent_modifiers() {
    let pg = scratch_pg::ScratchPg::start();
    let pool = scratch_pool(&pg).await;
    let raw_dir = tempfile::tempdir().unwrap();
    let store = SourceStore::new(&pool, raw_dir.path()).unwrap();
    let (first_run, first_summary) = persist_run(&store, "first").await;
    assert_eq!(first_summary["endpoints"].as_object().unwrap().len(), 13);
    assert_eq!(latest_mirrored_client_version(&pool).await.unwrap(), 6759);
    for kind in MIRRORED_ASSET_KINDS {
        for language in mirrored_asset_languages(kind).unwrap() {
            let language_arg = (!language.is_empty()).then_some(*language);
            let loaded = load_mirrored_assets_with_receipt(&pool, 6759, kind, language_arg)
                .await
                .unwrap();
            let key = mirrored_asset_key(kind, language_arg).unwrap();
            assert_eq!(loaded.receipt.source_run_id, first_run);
            assert_eq!(loaded.receipt.client_version, 6759);
            assert_eq!(loaded.receipt.kind, *kind);
            assert_eq!(loaded.receipt.language.as_deref(), language_arg);
            assert_eq!(loaded.receipt.parser_revision, PARSER_REVISION);
            assert_eq!(
                loaded.receipt.endpoint.source_document_id,
                first_summary["endpoints"][&key]["source_document_id"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                loaded.receipt.manifest.source_document_id,
                first_summary["manifest_document_id"].as_i64().unwrap()
            );
            assert_eq!(
                loaded.receipt.endpoint.raw_sha256,
                first_summary["endpoints"][&key]["raw_sha256"]
                    .as_str()
                    .unwrap()
            );
            assert_ne!(
                loaded.receipt.endpoint.raw_sha256,
                crate::external::sha256(&serde_json::to_vec(&loaded.payload).unwrap())
            );
            assert!(!loaded.receipt.endpoint.provenance.publication_authorized);
            assert!(
                !loaded
                    .receipt
                    .endpoint
                    .provenance
                    .provider_egress_authorized
            );
            assert!(!loaded.receipt.endpoint.fetched_at.is_empty());
            assert!(!loaded.receipt.run_started_at.is_empty());
            assert!(!loaded.receipt.run_finished_at.is_empty());
            assert_eq!(
                load_mirrored_assets(&pool, 6759, kind, language)
                    .await
                    .unwrap(),
                loaded.payload
            );
        }
    }
    assert!(
        load_mirrored_assets_with_receipt(&pool, 6759, "modifiers", Some("english"))
            .await
            .is_err()
    );
    let (second_run, _) = persist_run(&store, "second").await;
    let latest = load_mirrored_assets_with_receipt(&pool, 6759, "items", Some("english"))
        .await
        .unwrap();
    assert_eq!(latest.receipt.source_run_id, second_run);
    assert_eq!(latest.payload[0]["observation"], "second");
    let pinned = load_mirrored_assets_for_run(&pool, first_run, 6759, "items", Some("english"))
        .await
        .unwrap();
    assert_eq!(pinned.receipt.source_run_id, first_run);
    assert_eq!(pinned.payload[0]["observation"], "first");
    assert!(
        load_mirrored_assets_for_run(&pool, first_run, 6757, "items", Some("english"))
            .await
            .is_err()
    );
    sqlx::query(
        "UPDATE brain.source_runs SET summary=jsonb_set(summary, \
        '{endpoints,items/english,raw_sha256}', '\"wrong\"'::jsonb) WHERE id=$1",
    )
    .bind(second_run)
    .execute(&pool)
    .await
    .unwrap();
    assert!(
        load_mirrored_assets_with_receipt(&pool, 6759, "items", Some("english"))
            .await
            .is_err()
    );
    assert!(
        load_mirrored_assets_for_run(&pool, first_run, 6759, "items", Some("english"))
            .await
            .is_ok()
    );
    pool.close().await;
}
