use super::{BuildObservation, ObservationSource};
use crate::{meta::AuthorBuildSource, ReasonerError, Result};
use serde_json::Value;
use sqlx::Row;
use std::collections::{BTreeMap, BTreeSet};

fn integer(v: Option<&Value>) -> Option<i64> {
    v.and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
}

pub fn author_observation(source: &AuthorBuildSource) -> BuildObservation {
    let items = crate::meta::core_item_ids(&source.details);
    let mut imbues = BTreeMap::new();
    for category in source
        .details
        .get("modCategories")
        .or_else(|| source.details.get("mod_categories"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        for item in category
            .get("mods")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let id = integer(item.get("abilityId").or_else(|| item.get("ability_id")));
            let target = integer(item.get("imbue"));
            if let (Some(id), Some(target)) = (id, target.filter(|v| *v > 0)) {
                if items.contains(&id) {
                    imbues.insert(id, target);
                }
            }
        }
    }
    let skill_order = crate::meta::author_ability_order(&source.details).unwrap_or_default();
    // Imported sources without a build ID still distinguish mechanical plans.
    // Display names, category labels and author priority cannot change the key.
    let signature = serde_json::to_vec(&(source.hero_id, &items, &imbues, &skill_order))
        .expect("integer-only observation signature is serializable");
    let source_key = source
        .details
        .get("_reasoner_build_id")
        .and_then(Value::as_i64)
        .map(|id| id.to_string())
        .unwrap_or_else(|| super::identity(signature));
    BuildObservation {
        key: format!("author:{}:{source_key}", source.author),
        participant: source.author.clone(),
        source: ObservationSource::Author,
        items,
        imbues,
        skill_order,
        buy_times_s: BTreeMap::new(),
        sold_times_s: BTreeMap::new(),
        observed_at: integer(source.details.get("_reasoner_updated_at")),
        won: None,
        warnings: vec![
            "Autoren-Kaufposition stammt aus der Kategorienreihenfolge; keine gemessene Kaufzeit."
                .into(),
        ],
    }
}

/// Bounded, read-only load. IDs make the ORDER BY total; no random SQL sampling.
/// The time range is relative to the newest ingested match, not wall-clock time,
/// so replay inputs remain stable. Its actual range is reported by each family.
pub async fn load_player_observations(
    pool: &sqlx::PgPool,
    hero_id: i64,
) -> Result<Vec<BuildObservation>> {
    let present: bool =
        sqlx::query_scalar("SELECT to_regclass('brain.population_player_matches') IS NOT NULL")
            .fetch_one(pool)
            .await
            .map_err(ReasonerError::Db)?;
    if !present {
        return Ok(Vec::new());
    }
    let rows = sqlx::query("SELECT match_id,account_id,items,buy_times_s,sold_times_s,imbue_targets,ability_points,won,EXTRACT(EPOCH FROM start_time)::bigint AS observed_at FROM brain.population_player_matches WHERE hero_id=$1 AND (start_time >= (SELECT max(start_time)-interval '14 days' FROM brain.population_player_matches WHERE hero_id=$1) OR start_time IS NULL) ORDER BY start_time DESC NULLS LAST,match_id DESC,account_id LIMIT 8001")
        .bind(hero_id).fetch_all(pool).await.map_err(ReasonerError::Db)?;
    let limited = rows.len() > 8000;
    let mut output = Vec::new();
    for row in rows.into_iter().take(8000) {
        let ids: Vec<i64> = row.try_get("items").map_err(ReasonerError::Db)?;
        let buys: Vec<i32> = row.try_get("buy_times_s").map_err(ReasonerError::Db)?;
        let sold: Vec<Option<i32>> = row.try_get("sold_times_s").map_err(ReasonerError::Db)?;
        let targets: Vec<i64> = row.try_get("imbue_targets").map_err(ReasonerError::Db)?;
        let points: Vec<i64> = row.try_get("ability_points").map_err(ReasonerError::Db)?;
        let mut warnings = Vec::new();
        if limited {
            warnings
                .push("Populationsfenster auf die neuesten 8000 Spieler-Matches begrenzt.".into());
        }
        let times_ok = ids.len() == buys.len() && ids.len() == sold.len();
        let targets_ok = ids.len() == targets.len();
        if !times_ok {
            warnings
                .push("Kauf-/Verkaufszeiten nicht ausgerichtet; Zeiten bleiben unbekannt.".into());
        }
        if !targets_ok {
            warnings.push("Imbue-Array nicht ausgerichtet; keine Zielzuordnung geraten.".into());
        }
        let mut buy_times_s = BTreeMap::new();
        let mut sold_times_s = BTreeMap::new();
        let mut imbues = BTreeMap::new();
        let mut conflicts = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut items = Vec::new();
        let mut indices: Vec<_> = (0..ids.len()).collect();
        if times_ok {
            indices.sort_by_key(|i| (buys[*i], *i));
        }
        for i in indices {
            let id = ids[i];
            if id <= 0 {
                continue;
            }
            if seen.insert(id) {
                items.push(id);
            }
            if times_ok {
                buy_times_s.entry(id).or_insert(i64::from(buys[i]));
                if let Some(time) = sold[i] {
                    sold_times_s.insert(id, i64::from(time));
                }
            }
            if targets_ok
                && targets[i] > 0
                && imbues
                    .insert(id, targets[i])
                    .is_some_and(|old| old != targets[i])
            {
                conflicts.insert(id);
            }
        }
        for id in conflicts {
            imbues.remove(&id);
            warnings.push(format!(
                "Item {id}: mehrere Imbue-Ziele im selben Match; keine eindeutige Bindung."
            ));
        }
        let skill_values: Vec<_> = points.into_iter().map(Value::from).collect();
        let skill_order = match crate::fallback_ability_order(&skill_values) {
            Ok(order) => order,
            Err(error) => {
                warnings.push(error.to_string());
                Vec::new()
            }
        };
        let account: i64 = row.try_get("account_id").map_err(ReasonerError::Db)?;
        let match_id: i64 = row.try_get("match_id").map_err(ReasonerError::Db)?;
        output.push(BuildObservation {
            key: format!("match:{match_id}:{account}"),
            participant: account.to_string(),
            source: ObservationSource::Player,
            items,
            imbues,
            skill_order,
            buy_times_s,
            sold_times_s,
            observed_at: row.try_get("observed_at").map_err(ReasonerError::Db)?,
            won: Some(row.try_get("won").map_err(ReasonerError::Db)?),
            warnings,
        });
    }
    Ok(output)
}

/// Family discovery is not restricted to watched authors. The watchlist remains
/// an attribution/priority signal, never proof that other playstyles do not exist.
pub async fn load_family_author_sources(
    pool: &sqlx::PgPool,
    hero_id: i64,
) -> Result<Vec<AuthorBuildSource>> {
    let rows = sqlx::query("SELECT h.author_account_id::text AS author, COALESCE(w.priority,0)::double precision AS weight, h.details || jsonb_build_object('_reasoner_build_id',h.hero_build_id,'_reasoner_updated_at',EXTRACT(EPOCH FROM COALESCE(h.last_updated_at,h.published_at))::bigint) AS details FROM (SELECT DISTINCT ON(hero_build_id) * FROM tierlist.hero_build_sources WHERE hero_id=$1 ORDER BY hero_build_id,version DESC NULLS LAST,fetched_at DESC NULLS LAST) h LEFT JOIN tierlist.watched_build_authors w ON w.author_account_id=h.author_account_id AND w.is_active IS TRUE ORDER BY h.hero_build_id")
        .bind(hero_id).fetch_all(pool).await.map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            Ok(AuthorBuildSource {
                hero_id,
                author: row.try_get("author").map_err(ReasonerError::Db)?,
                weight: row.try_get("weight").map_err(ReasonerError::Db)?,
                details: row.try_get("details").map_err(ReasonerError::Db)?,
            })
        })
        .collect()
}
