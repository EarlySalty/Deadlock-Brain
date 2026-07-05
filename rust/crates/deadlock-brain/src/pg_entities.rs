//! PG-gestuetzter `entities`-Lesebefehl (Phase-2-Tracer).
//!
//! Dies ist der EINE Befehl, der von rusqlite auf sqlx/PgPool umgestellt wurde.
//! Er liest `brain.entities` + `brain.entity_aliases` aus der zentralen Postgres.
//! Alle anderen Befehle laufen weiterhin ueber rusqlite (`db::open_connection`).
//!
//! Suchsemantik (identisch zur SQLite-Referenz fuers Paritaets-Diffing):
//! Ein Entity passt, wenn `canonical_name` ODER einer seiner Aliase den `--query`
//! als case-insensitiven Teilstring enthaelt; `--type` filtert zusaetzlich exakt
//! auf `entity_type`.

use anyhow::Result;
use deadlock_brain_core::pg;
use serde_json::{json, Value};

use crate::EntitiesArgs;

struct EntityAliasRow {
    entity_type: String,
    canonical_name: String,
    source: String,
    matched_alias: Option<String>,
}

/// Async-Einstieg: wird vom Top-Level-Tokio-Runtime (`main.rs`) getrieben.
/// Ein eigener Per-Befehl-Runtime ist nicht mehr noetig.
pub async fn run(args: EntitiesArgs) -> Result<()> {
    let result = search_entities(&args).await?;
    crate::print_json(&result)
}

async fn search_entities(args: &EntitiesArgs) -> Result<Value> {
    let pool = pg::pg_pool().await?;
    let pattern = format!("%{}%", args.query);

    // Eine Zeile je (Entity, passender-oder-NULL Alias): der Alias-LIKE steht im
    // JOIN, damit rein ueber canonical_name gefundene Entities genau eine Zeile
    // mit NULL-Alias liefern. WHERE vereint Canonical- und Alias-Treffer.
    //
    // `query_as!` ist compile-geprueft (Offline-Cache in `.sqlx/`). Die
    // Nullbarkeit ist explizit annotiert: `!` = NOT NULL, `?` = NULL-bar
    // (a.alias stammt aus dem LEFT-JOIN und ist optional).
    let rows = sqlx::query_as!(
        EntityAliasRow,
        r#"SELECT
               e.entity_type    AS "entity_type!",
               e.canonical_name AS "canonical_name!",
               e.source         AS "source!",
               a.alias          AS "matched_alias?"
           FROM brain.entities e
           LEFT JOIN brain.entity_aliases a
             ON a.entity_id = e.id
            AND lower(a.alias) LIKE lower($2)
           WHERE ($1::text IS NULL OR e.entity_type = $1)
             AND (lower(e.canonical_name) LIKE lower($2) OR a.alias IS NOT NULL)
           ORDER BY e.entity_type, e.canonical_name, a.alias"#,
        args.entity_type.as_deref(),
        pattern.as_str(),
    )
    .fetch_all(&pool)
    .await?;

    pool.close().await;

    // Zeilen sind nach (entity_type, canonical_name) sortiert -> konsekutiv gruppieren.
    let mut entities: Vec<Value> = Vec::new();
    let mut current: Option<(String, String, String, Vec<String>)> = None;
    for row in rows {
        let changed = match &current {
            Some((etype, name, _, _)) => etype != &row.entity_type || name != &row.canonical_name,
            None => true,
        };
        if changed {
            if let Some((etype, name, source, aliases)) = current.take() {
                entities.push(entity_json(etype, name, source, aliases));
            }
            current = Some((
                row.entity_type.clone(),
                row.canonical_name.clone(),
                row.source.clone(),
                Vec::new(),
            ));
        }
        if let (Some(alias), Some((_, _, _, aliases))) = (row.matched_alias, current.as_mut()) {
            if !aliases.contains(&alias) {
                aliases.push(alias);
            }
        }
    }
    if let Some((etype, name, source, aliases)) = current.take() {
        entities.push(entity_json(etype, name, source, aliases));
    }

    Ok(json!({
        "query": args.query,
        "type": args.entity_type,
        "count": entities.len(),
        "entities": entities,
    }))
}

fn entity_json(entity_type: String, canonical_name: String, source: String, aliases: Vec<String>) -> Value {
    json!({
        "entity_type": entity_type,
        "canonical_name": canonical_name,
        "source": source,
        "matched_aliases": aliases,
    })
}
