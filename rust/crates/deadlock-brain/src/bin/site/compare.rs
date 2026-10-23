use anyhow::{Context, Result};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::Response,
};
use brain_contracts::{ToolCall, ToolName};
use serde::Deserialize;
use serde_json::{json, Value};

use super::{artifacts, profiles, Site};

const COMPARE: &str = include_str!("../../../../brain-serve/src/pg_asset_compare.sql");

#[derive(Default, Deserialize)]
pub(super) struct Selection {
    entities: Option<String>,
    stats: Option<String>,
}

pub(super) async fn page(
    State(site): State<Site>,
    Path(id): Path<String>,
    Query(query): Query<Selection>,
    method: Method,
    headers: HeaderMap,
) -> Response {
    if query.entities.is_none() && brain_storage::compare_artifact::valid_compare_id(&id) {
        return artifacts::page(site, id, method, &headers).await;
    }
    let raw = query.entities.as_deref().unwrap_or(&id);
    let Ok(entities) = raw
        .split(',')
        .map(str::parse::<i64>)
        .collect::<std::result::Result<Vec<_>, _>>()
    else {
        return profiles::missing(method);
    };
    let stats: Vec<_> = query
        .stats
        .as_deref()
        .unwrap_or("health,move_speed")
        .split(',')
        .collect();
    let call = ToolCall {
        id: "site-compare".into(),
        name: ToolName::Compare,
        arguments: json!({"entities":entities,"stats":stats}),
    };
    let Ok(request) = call.validate(&brain_serve::pg_asset_tools::definitions()) else {
        return profiles::missing(method);
    };
    let result: Result<Option<String>> = async {
        let mut tx = profiles::transaction(&site).await?;
        let value: Value = sqlx::query_scalar(COMPARE).bind(&entities).bind(&stats).fetch_one(&mut *tx).await?;
        let Some(result) = brain_serve::pg_asset_tools::bounded_comparison_result("site-compare", &request, value)? else { return Ok(None); };
        let value = result.result;
        let mut body = String::from("<h1>Vergleich</h1><p>Aktuell beobachteter Quellenstand. Der offizielle Patchstand ist unbekannt. Bedingungen und Einheiten gehören zum jeweiligen Wert.</p>");
        for identity in value["entities"].as_array().context("entities")? {
            let id = &identity["entity_id"];
            if let Some(name) = profiles::display_name(identity) {
                body.push_str(&format!("<h2>{}</h2>", profiles::escape(name)));
            }
            let facts: Vec<_> = value["stats"]["facts"].as_array().context("facts")?.iter().filter(|fact| fact["entity_id"] == *id).cloned().collect();
            body.push_str(&profiles::fact_rows(&json!({"facts":facts,"truncated":value["stats"]["truncated"]})));
            for cell in value["cells"].as_array().context("cells")?.iter().filter(|cell| cell["entity_id"] == *id) {
                let message = match cell["status"].as_str() {
                    Some("missing") => Some("kein belegter Wert verfügbar"),
                    Some("omitted" | "partial") => Some("nicht alle belegten Werte werden angezeigt"),
                    _ => None,
                };
                if let Some(message) = message {
                    body.push_str(&format!("<p>{}: {message}.</p>", profiles::escape(cell["stat_key"].as_str().context("stat_key")?)));
                }
            }
        }
        body.push_str(&profiles::receipts(&mut tx, &value).await?);
        tx.commit().await?;
        Ok(Some(profiles::document("Vergleich", &body)))
    }.await;
    match result {
        Ok(Some(html)) => profiles::response(StatusCode::OK, html, method),
        Ok(None) => profiles::missing(method),
        Err(_) => {
            eprintln!("site_comparison_unavailable");
            profiles::unavailable(method)
        }
    }
}
