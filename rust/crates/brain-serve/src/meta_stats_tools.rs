use brain_contracts::{tools::GetMetaStatsRequest, PortError, ToolDefinition, ToolName};
use serde_json::{json, Value};
use sqlx::PgConnection;

pub(super) const QUERY: &str = include_str!("meta_stats_query.sql");
const IDENTITY: &str = include_str!("meta_stats_identity.sql");

pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::GetMetaStats,
        description: "Liest gespeicherte Metastatistiken eines Helden (kind hero) oder Items (kind item) aus stündlichen Snapshots der Deadlock-API-Analysen, ohne Live-Abruf. game_id ist die numerische Deadlock-ID (hero_id bzw. item_id); alternativ entity mit der Entitäts-ID aus find_entity. Liefert den neuesten Snapshot und, falls vorhanden, Vergleichsstände von vor etwa 24 Stunden, 7 und 30 Tagen, jeweils mit Snapshot-Zeit, Bedingungen und Beleg. snapshot_at ist die UTC-Erfassungsstunde des Snapshots; neuester Stand und Vergleichsstände richten sich nach dieser Erfassungszeit. Jeder Snapshot fasst Matches im Modus normal ab Beginn des Erfassungstags (UTC) minus 30 Tage bis zur Erfassungsstunde zusammen, angegeben in condition als matches_since und matches_until. Fehlt der Held oder das Item im neuesten Snapshot, wird der neueste ältere Snapshot aus den letzten 1000 Erfassungsstunden geliefert; seine snapshot_at zeigt das. Siegquote ist wins geteilt durch matches. Snapshot-Zeit immer nennen. Fehlende Snapshots nicht ergänzen und keine aktuellen Werte behaupten, die nicht geliefert wurden.".into(),
        input_schema: json!({
            "type":"object","additionalProperties":false,"required":["kind"],
            "properties":{
                "kind":{"type":"string","enum":["hero","item"]},
                "game_id":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "entity":{"type":"integer","minimum":1,"maximum":i64::MAX}
            }
        }),
    }
}

async fn identity(
    connection: &mut PgConnection,
    request: &GetMetaStatsRequest,
) -> Result<Option<(u64, Option<Value>)>, PortError> {
    if let Some(id) = request.game_id {
        return Ok(Some((id, None)));
    }
    let Some(entity) = request.entity else {
        return Ok(None);
    };
    let fact = sqlx::query_scalar::<_, Value>(IDENTITY)
        .bind(entity as i64)
        .bind(request.kind.as_str())
        .fetch_optional(connection)
        .await
        .map_err(super::database_error)?;
    Ok(fact.and_then(|fact| {
        let id = fact.get("value")?.as_str()?.parse().ok()?;
        Some((id, Some(fact)))
    }))
}

pub(super) async fn read(
    connection: &mut PgConnection,
    request: &GetMetaStatsRequest,
) -> Result<Option<Value>, PortError> {
    let Some((game_id, identity)) = identity(&mut *connection, request).await? else {
        return Ok(None);
    };
    let rows = sqlx::query_scalar::<_, Value>(QUERY)
        .bind(request.kind.stat_set())
        .bind(format!("{}_{game_id}", request.kind.as_str()))
        .fetch_all(connection)
        .await
        .map_err(super::database_error)?;
    if rows.is_empty() {
        return Ok(None);
    }
    let mut result = json!({
        "kind":request.kind.as_str(),"game_id":game_id,"entity":request.entity,
        "facts":rows,"truncated":false,"omitted_count":0
    });
    if let (Some(identity), Some(object)) = (identity, result.as_object_mut()) {
        object.insert("identity".into(), identity);
    }
    Ok(Some(result))
}
