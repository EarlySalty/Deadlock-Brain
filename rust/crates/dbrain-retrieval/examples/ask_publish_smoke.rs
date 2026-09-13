use std::{collections::BTreeSet, path::Path};

use dbrain_reasoner::{BuildObject, ReasonerConfig, ReasonerCtx, ReasonerOptions};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).ok_or("Ausgabepfad fehlt")?;
    let pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&pool)
        .await?;
    assert_eq!(read_only, "on");
    let started_at = chrono::Utc::now();
    let ask = dbrain_retrieval::ask_context(
        &pool,
        "Build für Warden",
        &dbrain_retrieval::AskContextOptions {
            limit_events: 30,
            include_unverified: false,
            max_claims: 20,
            game_wiki_dir: None,
        },
    )
    .await?;
    assert_eq!(ask["retrieval_meta"]["route"], "build_reasoner");
    assert_eq!(ask["build_context_schema"], "reasoner_build_v1");
    let text = ask["result_text"].as_str().ok_or("Antworttext fehlt")?;
    assert!(!text.trim().is_empty());
    assert_eq!(ask["validation"]["text"], text);
    assert_eq!(ask["validation"]["violations"], json!([]));
    let prompt = ask["prompt"].as_str().ok_or("Prompt fehlt")?;
    let prompt_build: serde_json::Value = serde_json::from_str(
        prompt
            .split_once("BUILD_CONTEXT_JSON:\n")
            .ok_or("Build im Prompt fehlt")?
            .1,
    )?;
    assert_eq!(prompt_build["schema"], "reasoner_prompt_v1");
    assert_eq!(prompt_build["hero_id"], ask["build_context"]["hero_id"]);
    assert_eq!(
        prompt_build["ability_order"],
        ask["build_context"]["ability_order"]
    );
    let prompt_items = prompt_build["core"]
        .as_array()
        .ok_or("Kern im Prompt fehlt")?
        .iter()
        .chain(
            prompt_build["situations"]
                .as_array()
                .ok_or("Situationen im Prompt fehlen")?
                .iter()
                .flat_map(|block| block["items"].as_array().into_iter().flatten()),
        );
    let complete_items = ask["build_context"]["core"]
        .as_array()
        .ok_or("Vollständiger Kern fehlt")?
        .iter()
        .chain(
            ask["build_context"]["situations"]
                .as_array()
                .ok_or("Vollständige Situationen fehlen")?
                .iter()
                .flat_map(|block| block["items"].as_array().into_iter().flatten()),
        );
    for (compact, complete) in prompt_items.zip(complete_items) {
        for field in [
            "item_id",
            "name",
            "buy_phase",
            "confidence",
            "imbue_target",
            "sell_priority",
        ] {
            assert_eq!(compact[field], complete[field]);
        }
        assert!(compact.get("sources").is_none());
        if complete["why"]
            .as_str()
            .is_some_and(|why| why.contains("Verkaufe "))
        {
            assert!(compact["why"]
                .as_str()
                .is_some_and(|why| why.contains("Verkaufe ")));
        }
    }
    let build: BuildObject = serde_json::from_value(ask["build_context"].clone())?;
    let ctx = ReasonerCtx {
        pool: pool.clone(),
        ai: None,
        config: ReasonerConfig {
            use_ai: false,
            patch_tag: build.patch_tag.clone(),
            ..Default::default()
        },
    };
    let direct = dbrain_reasoner::reason_build_with_options(
        &ctx,
        &build.hero_name,
        ReasonerOptions {
            seed_path: None,
            persist: false,
        },
    )
    .await?;
    assert_eq!(serde_json::to_value(&direct)?, ask["build_context"]);
    let hero = dbrain_reasoner::load_hero_model(&ctx, &build.hero_name).await?;
    let ability_ids = hero
        .abilities
        .iter()
        .filter_map(|ability| u32::try_from(ability.ability_id).ok().filter(|id| *id > 0))
        .collect::<BTreeSet<_>>();
    let payload = dbrain_reasoner::publish::to_publish_payload(&build);
    assert!(u32::try_from(payload.hero_id)? > 0);
    assert_eq!(payload.hero_id, build.hero_id);
    assert_eq!(payload.language, 1);
    assert!(!payload.name.trim().is_empty());
    assert_eq!(payload.description, build.rationale);
    assert_eq!(payload.mod_categories.len(), build.situations.len() + 1);
    let groups = std::iter::once(build.core.as_slice())
        .chain(build.situations.iter().map(|block| block.items.as_slice()));
    let mut categories = Vec::new();
    let mut imbue_count = 0;
    let mut sale_count = 0;
    for (category, items) in payload.mod_categories.iter().zip(groups) {
        assert!(category
            .width
            .is_some_and(|value| value.is_finite() && value > 0.0 && value <= f32::MAX as f64));
        assert!(category
            .height
            .is_some_and(|value| value.is_finite() && value > 0.0 && value <= f32::MAX as f64));
        assert_eq!(category.mods.len(), items.len());
        assert!(category
            .description
            .as_deref()
            .is_none_or(
                |description| !description.contains('\n') && description.chars().count() <= 400
            ));
        for (published, item) in category.mods.iter().zip(items) {
            assert!(u32::try_from(published.ability_id)? > 0);
            assert_eq!(published.ability_id, item.item_id);
            assert_eq!(published.annotation, item.why);
            assert!(!published.annotation.trim().is_empty());
            assert_eq!(published.imbue, item.imbue_target);
            assert_eq!(published.sell_priority, item.sell_priority);
            if let Some(imbue) = published.imbue {
                assert!(ability_ids.contains(&u32::try_from(imbue)?));
                imbue_count += 1;
            }
            sale_count += usize::from(published.sell_priority.is_some());
        }
        categories.push(json!({"name":category.name,"items":category.mods.len(),"width":category.width,"height":category.height,"optional":category.optional}));
    }
    assert_eq!(
        payload.ability_order.as_ref().map_or(0, Vec::len),
        build.ability_order.len()
    );
    for (published, step) in payload
        .ability_order
        .iter()
        .flatten()
        .zip(&build.ability_order)
    {
        assert!(ability_ids.contains(&u32::try_from(published.ability_id)?));
        i32::try_from(published.currency_type)?;
        i32::try_from(published.delta)?;
        assert_eq!(published.ability_id, step.ability_id);
        assert_eq!(published.currency_type, step.currency_type);
        assert_eq!(published.delta, step.delta);
    }
    let evidence = json!({
        "started_at":started_at.to_rfc3339(),"finished_at":chrono::Utc::now().to_rfc3339(),"read_only":read_only,
        "route":ask["retrieval_meta"]["route"],"direct_reasoner_identical":true,
        "result_text_chars":text.chars().count(),"prompt_chars":prompt.chars().count(),
        "core_items":build.core.len(),"categories":categories,"imbue_items":imbue_count,
        "items_with_sell_priority":sale_count,"ability_steps":build.ability_order.len(),
        "upload_performed":false,"publish_consumer_contract":"steam-core/src/task/handlers/builds/publish_original.rs",
        "ask":ask,"publish_payload":payload,
    });
    std::fs::write(Path::new(&output), serde_json::to_vec_pretty(&evidence)?)?;
    println!("Ask und Publish geprüft: {} Kern-Items, {} Kategorien, {imbue_count} Imbue-Ziele, {sale_count} Verkaufsprioritäten; kein Upload.", build.core.len(), build.situations.len() + 1);
    Ok(())
}
