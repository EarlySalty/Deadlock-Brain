use serde_json::json;

#[path = "../../brain-storage/tests/mirror_game_context.rs"]
mod mirror;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn final_publication_validator_uses_released_only_verified_hero_data() {
    let pg = mirror::scratch_pg::ScratchPg::start();
    let pool = mirror::pool(&pg).await;
    mirror::seed_with_heroes(
        &pool,
        "hero-publication-status",
        true,
        json!([
            {"id":88,"name":"Baba","class_name":"hero_baba","player_selectable":true,
                "disabled":false,"in_development":true},
            {"id":56,"name":"Bomber","class_name":"hero_bomber","player_selectable":false,
                "disabled":true,"in_development":true},
            {"id":68,"name":"Boho","class_name":"hero_boho","player_selectable":false,
                "disabled":true,"in_development":true},
            {"id":99,"name":"Labs Fixture","class_name":"hero_labs_fixture",
                "player_selectable":true,"disabled":false,"in_hero_labs":true},
            {"id":83,"name":"hero_testhero","class_name":"hero_testhero",
                "player_selectable":true,"disabled":false}
        ]),
    )
    .await;
    let mut payload = json!({"hero_id":88,"mod_categories":[{"mods":[{"ability_id":8}]}]});
    dbrain_reasoner::publish::validate_publish_items(&pool, &payload)
        .await
        .unwrap();
    for id in [56, 68, 99, 83, 404] {
        payload["hero_id"] = json!(id);
        assert!(
            dbrain_reasoner::publish::validate_publish_items(&pool, &payload)
                .await
                .is_err(),
            "hero {id}"
        );
    }
    pool.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "explicit read-only final-validator probe against the current production game mirror"]
async fn live_final_publication_validator_reads_current_production_catalog() {
    let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
        .host("/var/run/postgresql")
        .port(5432)
        .username("nathanael")
        .database("deadlock")
        .options([
            ("default_transaction_read_only", "on"),
            ("statement_timeout", "10000"),
        ]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(read_only, "on");
    let version = brain_storage::asset_mirror::latest_mirrored_client_version(&pool)
        .await
        .unwrap();
    let heroes = brain_storage::asset_mirror::load_mirrored_assets_with_receipt(
        &pool,
        version,
        "heroes_all",
        Some("english"),
    )
    .await
    .unwrap();
    let items = brain_storage::asset_mirror::load_mirrored_assets_with_receipt(
        &pool,
        version,
        "items",
        Some("english"),
    )
    .await
    .unwrap();
    assert_eq!(heroes.receipt.source_run_id, items.receipt.source_run_id);
    let item = items
        .payload
        .as_array()
        .unwrap()
        .iter()
        .find(|item| brain_contracts::game_mode::GameMode::Normal.allows_item(item))
        .unwrap();
    for name in ["Warden", "Baba"] {
        let hero = heroes
            .payload
            .as_array()
            .unwrap()
            .iter()
            .find(|hero| hero["name"].as_str() == Some(name))
            .unwrap();
        let payload =
            json!({"hero_id":hero["id"],"mod_categories":[{"mods":[{"ability_id":item["id"]}]}]});
        let result = dbrain_reasoner::publish::validate_publish_items(&pool, &payload).await;
        println!(
            "{}",
            json!({"event":"hero_publication_read_only_probe","hero":name,
            "hero_id":hero["id"],"item_id":item["id"],"client_version":version,
            "heroes_run":heroes.receipt.source_run_id,"items_run":items.receipt.source_run_id,
            "database_read_only":read_only,"allowed":result.is_ok()})
        );
        result.unwrap();
    }
    pool.close().await;
}
