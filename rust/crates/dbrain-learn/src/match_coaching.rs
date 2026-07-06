use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{build_optimizer::build_hero_build_context, Result};

pub async fn build_match_coaching_context(
    pool: &PgPool,
    match_id: &str,
    query: &str,
) -> Result<Value> {
    let base_context = match build_hero_build_context(pool, query, &[], 20).await {
        Ok(context) => context,
        Err(error) => json!({"error": error.to_string(), "query": query}),
    };
    Ok(json!({
        "query": query,
        "match_id": match_id,
        "match_data": {
            "status": "simulated",
            "timeline_events": [
                {"time_min": 5, "event": "First Blood (Died)", "enemy": "Infernus"},
                {"time_min": 10, "event": "Networth Difference: -1500", "cause": "Missed farm"},
                {"time_min": 15, "event": "Enemy Infernus bought Toxic Bullets"},
                {"time_min": 25, "event": "Player bought Debuff Reducer (Late)"}
            ],
            "heuristics": [
                "Warning: Debuff Reducer was bought 10 minutes after Infernus started snowballing.",
                "Warning: High death rate in early laning phase against Infernus."
            ]
        },
        "entity_knowledge": base_context,
    }))
}
