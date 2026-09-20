//! Read-only inventory of the inputs required for family-conditioned builds.
use serde_json::{json, Value};
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let read_only: String = sqlx::query_scalar("SHOW transaction_read_only")
        .fetch_one(&pool)
        .await?;
    if read_only != "on" {
        return Err("read-only connection required".into());
    }
    let mut result = json!({"read_only": read_only});
    for (key, query) in [
        ("population", "SELECT jsonb_build_object('hero_id',p.hero_id,'name',max(h.name),'matches',count(*),'players',count(DISTINCT account_id),'first',min(start_time),'last',max(start_time),'with_imbues',count(*) FILTER(WHERE EXISTS(SELECT 1 FROM unnest(imbue_targets) AS v WHERE v>0)),'with_skills',count(*) FILTER(WHERE cardinality(ability_points)>0)) AS row FROM brain.population_player_matches p LEFT JOIN brain.hero_catalog h ON h.hero_id=p.hero_id GROUP BY p.hero_id ORDER BY p.hero_id"),
        ("snapshots", "SELECT jsonb_build_object('source',source,'entity_type',entity_type,'count',count(*),'oldest',min(fetched_at),'newest',max(fetched_at)) AS row FROM brain.entity_snapshots GROUP BY source,entity_type ORDER BY source,entity_type"),
        ("sources", "SELECT jsonb_build_object('hero_id',s.hero_id,'name',max(h.name),'builds',count(DISTINCT hero_build_id),'authors',count(DISTINCT s.author_account_id),'latest',max(s.last_updated_at)) AS row FROM tierlist.hero_build_sources s JOIN tierlist.watched_build_authors w ON w.author_account_id=s.author_account_id AND w.is_active LEFT JOIN brain.hero_catalog h ON h.hero_id=s.hero_id GROUP BY s.hero_id ORDER BY s.hero_id"),
        ("sample", "SELECT jsonb_build_object('hero_id',p.hero_id,'items',items,'buy_times_s',buy_times_s,'sold_times_s',sold_times_s,'imbue_targets',imbue_targets,'ability_points',ability_points,'ability_times_s',ability_times_s) AS row FROM brain.population_player_matches p JOIN brain.hero_catalog h ON h.hero_id=p.hero_id WHERE h.name='Viscous' ORDER BY start_time DESC NULLS LAST,match_id,account_id LIMIT 2"),
    ] {
        let rows = sqlx::query(query).fetch_all(&pool).await?;
        result[key] = Value::Array(rows.iter().map(|r| r.get::<Value,_>("row")).collect());
    }
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
