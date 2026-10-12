use brain_contracts::{tools::GetTimerRequest, PortError, ToolDefinition, ToolName};
use serde_json::{json, Value};
use sqlx::PgConnection;

pub(super) const QUERY: &str =
    "SELECT serve.atomic_fact(entity_id,stat_key,value_num,value_text,value_json,
    unit,condition,receipt_id,source_revision) || jsonb_build_object('timer',key)
    FROM serve.timer WHERE key=$1 ORDER BY stat_key,source_id,id LIMIT 33";

pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::GetTimer,
        description: "Liest belegte Kartenzeiten für urn (Urne), soul_box (Seelenboxen), mid_boss (Midboss) oder rejuvenator (Rejuvenator) aus der Datenbank. Für diese Kartenzeiten get_timer statt deadlock_data oder game_rules verwenden. Liefert einzelne Werte mit Einheit, Bedingungen, Quellenstand und Beleg. Fehlender Patch oder current bedeutet aktueller beobachteter Quellenstand, nicht offizieller Patch. Andere Patches werden ohne Belege abgewiesen. Fehlende Werte nicht ergänzen; unterschiedliche Bedingungen und Quellen nicht vermischen. Bei fehlenden Belegen keine Spielzeit behaupten.".into(),
        input_schema: json!({
            "type":"object", "additionalProperties":false, "required":["timer"],
            "properties":{
                "timer":{"type":"string","enum":["urn","soul_box","mid_boss","rejuvenator"]},
                "patch":{"type":"string","minLength":1,"maxLength":128}
            }
        }),
    }
}

pub(super) async fn read(
    connection: &mut PgConnection,
    request: &GetTimerRequest,
) -> Result<Option<Value>, PortError> {
    let rows = sqlx::query_scalar::<_, Value>(QUERY)
        .bind(request.timer.as_str())
        .fetch_all(connection)
        .await
        .map_err(super::database_error)?;
    if rows.is_empty() {
        return Ok(None);
    }
    let omitted = rows.len().saturating_sub(32);
    Ok(Some(json!({
        "timer":request.timer.as_str(),
        "facts":rows.into_iter().take(32).collect::<Vec<_>>(),
        "truncated":omitted>0,
        "omitted_count":omitted,
        "omitted_count_is_lower_bound":omitted>0
    })))
}

#[cfg(test)]
#[path = "timer_tools_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "timer_pg_tests.rs"]
mod pg_tests;
