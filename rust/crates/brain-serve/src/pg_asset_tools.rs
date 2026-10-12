use super::*;
use brain_contracts::tools::{CompareRequest, GetAbilityRequest};
use sqlx::Connection;
use std::collections::BTreeMap;

const ITEM: &str = "SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1";
const ABILITY: &str = "SELECT card FROM serve.ability_card WHERE entity_id=$1 LIMIT 1";
const SLOT: &str = "SELECT DISTINCT r.to_entity_id FROM serve.selected_relation_p2 r
    JOIN canon.entity h ON h.id=r.from_entity_id AND h.kind='hero'
    JOIN canon.entity a ON a.id=r.to_entity_id AND a.kind='ability'
    JOIN canon.receipt p ON p.id=h.identity_receipt_id
    JOIN canon.source_head v ON v.source_id=p.source_id AND v.published_revision_id>=p.revision_id
    WHERE r.from_entity_id=$1 AND r.kind='has_ability' AND r.slot=$2
    ORDER BY r.to_entity_id LIMIT 2";
const SLOT_LINKS: &str = "SELECT jsonb_build_object('entity_id',r.to_entity_id,
    'from_entity_id',r.from_entity_id,'kind',r.kind,'slot',r.slot,
    'citation',r.receipt_id::text,'patch',NULL,'source_revision',r.source_revision,
    'identity',jsonb_build_object('entity_id',h.id,'key',h.key,'citation',p.id::text,
        'patch',NULL,'source_revision',v.source_revision)) AS link, count(*) OVER () AS total
    FROM serve.selected_relation_p2 r JOIN canon.entity h ON h.id=r.from_entity_id AND h.kind='hero'
    JOIN canon.receipt p ON p.id=h.identity_receipt_id JOIN canon.revision v ON v.id=p.revision_id
    JOIN canon.source_head s ON s.source_id=p.source_id AND s.published_revision_id>=p.revision_id
    WHERE r.from_entity_id=$1 AND r.slot=$2 AND r.kind='has_ability' AND r.to_entity_id=$3
    ORDER BY r.source_id,r.id LIMIT 16";
const IDENTITIES: &str = "SELECT e.id, jsonb_build_object('entity_id',e.id,'kind',e.kind,
    'key',e.key,'status',e.status,'citation',p.id::text,'patch',NULL,
    'source_revision',v.source_revision) AS identity
    FROM canon.entity e JOIN canon.receipt p ON p.id=e.identity_receipt_id
    JOIN canon.revision v ON v.id=p.revision_id
    JOIN canon.source_head h ON h.source_id=p.source_id AND h.published_revision_id>=p.revision_id
    WHERE e.id=ANY($1::bigint[]) AND e.kind IN ('hero','item') ORDER BY e.id LIMIT 4";
const STATS: &str = "SELECT key FROM canon.stat WHERE key=ANY($1::text[]) ORDER BY key LIMIT 12";
const COMPARE: &str = "SELECT grid.entity_id,grid.stat_key,f.total,
    CASE WHEN f.id IS NULL THEN NULL ELSE jsonb_build_object('entity_id',f.entity_id,
        'stat_key',f.stat_key,'value',coalesce(to_jsonb(f.value_num),to_jsonb(f.value_text),f.value_json),
        'unit',f.unit,'condition',f.condition,'citation',f.receipt_id::text,
        'patch',NULL,'source_revision',f.source_revision) END AS fact
    FROM (SELECT e.id AS entity_id,s.key AS stat_key FROM canon.entity e CROSS JOIN canon.stat s
        WHERE e.id=ANY($1::bigint[]) AND e.kind IN ('hero','item') AND s.key=ANY($2::text[])) grid
    LEFT JOIN LATERAL (SELECT f.*, count(*) OVER () AS total FROM serve.selected_fact_p2 f
        WHERE f.entity_id=grid.entity_id AND f.stat_key=grid.stat_key
        ORDER BY f.condition,f.id LIMIT 3) f ON true
    ORDER BY grid.entity_id,grid.stat_key,f.condition,f.id LIMIT 144";

pub(super) fn canonical(name: ToolName) -> bool {
    matches!(
        name,
        ToolName::GetItem | ToolName::GetAbility | ToolName::Compare
    )
}

pub(super) fn historical(request: &ToolRequest) -> bool {
    let patch = match request.subrequest() {
        ToolSubrequest::GetItem(request) => &request.patch,
        ToolSubrequest::GetAbility(request) => &request.patch,
        ToolSubrequest::Compare(request) => &request.patch,
        _ => return false,
    };
    patch.as_deref().is_some_and(|patch| patch != "current")
}

pub(super) fn definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: ToolName::GetItem,
            description: "Liest die veröffentlichte Itemkarte aus kanonischen Spielwerten. item ist die positive kanonische Datenbank-ID aus find_entity mit kind item, keine ältere Spiel-ID. Werte, Bedingungen, Einheiten und Quellenbelege unverändert beachten. Fehlender Patch oder current bedeutet beobachteter Quellenstand; historische Patches sind nicht verfügbar. Fehlende oder abgeschnittene Fakten nicht ergänzen.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["item"],"properties":{
                "item":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
        ToolDefinition {
            name: ToolName::GetAbility,
            description: "Liest eine veröffentlichte Fähigkeitskarte. Genau eine Auswahl verwenden: ability als kanonische Datenbank-ID aus find_entity oder hero als kanonische Helden-ID zusammen mit slot eins bis vier. Keine älteren Spiel-IDs verwenden. Mehrdeutige Slots werden nicht geraten. Werte, Upgrades, Bedingungen und Quellenbelege unverändert beachten. Fehlender Patch oder current bedeutet beobachteter Quellenstand; historische Patches sind nicht verfügbar. Fehlende und abgeschnittene Fakten nicht ergänzen.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{
                "ability":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "hero":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "slot":{"type":"integer","minimum":1,"maximum":4},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
        ToolDefinition {
            name: ToolName::Compare,
            description: "Vergleicht zwei bis vier verschiedene Helden oder Items anhand ein bis zwölf verschiedener kanonischer Statistikschlüssel. entities enthält positive kanonische Datenbank-IDs aus find_entity, keine älteren Spiel-IDs. stats enthält vorhandene Schlüssel aus kanonischen Karten, keine geschätzten Metriken. Bedingungen und Einheiten sind Teil jedes Werts. Alle Identitäten bleiben erhalten; cells kennzeichnet fehlende oder ausgelassene Werte, die nicht ergänzt werden dürfen. Fehlender Patch oder current bedeutet beobachteter Quellenstand, historische Patches sind nicht verfügbar.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["entities","stats"],"properties":{
                "entities":{"type":"array","minItems":2,"maxItems":4,"items":{"type":"integer","minimum":1,"maximum":i64::MAX}},
                "stats":{"type":"array","minItems":1,"maxItems":12,"items":{"type":"string","minLength":1,"maxLength":128}},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
    ]
}

pub(super) async fn execute(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
) -> Result<ToolExecution, PortError> {
    let mut transaction = connection.begin().await.map_err(database_error)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;
    let connection = &mut *transaction;
    let value = match request.subrequest() {
        ToolSubrequest::GetItem(item) => sqlx::query_scalar::<_, Value>(ITEM)
            .bind(item.item as i64)
            .fetch_optional(&mut *connection)
            .await
            .map_err(database_error)?,
        ToolSubrequest::GetAbility(ability) => ability_card(connection, ability).await?,
        ToolSubrequest::Compare(compare) => comparison(connection, compare).await?,
        _ => return Err(invalid()),
    };
    let Some(value) = value else {
        return Ok(error_execution(
            id,
            request.name(),
            "canonical_data_unavailable",
            "Keine eindeutigen veröffentlichten Spielwerte für diese Auswahl verfügbar.",
        ));
    };
    let Some(value) = bounded_value(request.name(), value)? else {
        return Ok(error_execution(
            id,
            request.name(),
            "canonical_data_unavailable",
            "Die Spielwerte sind unvollständig belegt oder überschreiten die Ausgabegrenze.",
        ));
    };
    let execution = bounded_execution(connection, id, request, value).await?;
    transaction.commit().await.map_err(database_error)?;
    Ok(execution)
}

async fn ability_card(
    connection: &mut PgConnection,
    request: &GetAbilityRequest,
) -> Result<Option<Value>, PortError> {
    let ability = if let Some(ability) = request.ability {
        ability as i64
    } else {
        let (Some(hero), Some(slot)) = (request.hero, request.slot) else {
            return Err(invalid());
        };
        let candidates = sqlx::query_scalar::<_, i64>(SLOT)
            .bind(hero as i64)
            .bind(i16::from(slot))
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?;
        let [ability] = candidates.as_slice() else {
            return Ok(None);
        };
        *ability
    };
    let Some(mut card) = sqlx::query_scalar::<_, Value>(ABILITY)
        .bind(ability)
        .fetch_optional(&mut *connection)
        .await
        .map_err(database_error)?
    else {
        return Ok(None);
    };
    if let (Some(hero), Some(slot)) = (request.hero, request.slot) {
        let rows = sqlx::query(SLOT_LINKS)
            .bind(hero as i64)
            .bind(i16::from(slot))
            .bind(ability)
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?;
        if rows.is_empty() {
            return Ok(None);
        }
        let total = rows[0].try_get::<i64, _>("total").map_err(database_error)?;
        let omitted = total.saturating_sub(rows.len() as i64);
        let links = rows
            .into_iter()
            .map(|row| row.try_get::<Value, _>("link").map_err(database_error))
            .collect::<Result<Vec<_>, _>>()?;
        let Some(card) = card.as_object_mut() else {
            return Ok(None);
        };
        card.insert(
            "abilities".into(),
            json!({"links":links,"truncated":omitted>0,"omitted_count":omitted}),
        );
    }
    Ok(Some(card))
}

async fn comparison(
    connection: &mut PgConnection,
    request: &CompareRequest,
) -> Result<Option<Value>, PortError> {
    let ids = request
        .entities
        .iter()
        .map(|id| *id as i64)
        .collect::<Vec<_>>();
    let keys = sqlx::query_scalar::<_, String>(STATS)
        .bind(&request.stats)
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
    if keys.len() != request.stats.len() {
        return Ok(None);
    }
    let rows = sqlx::query(IDENTITIES)
        .bind(&ids)
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
    if rows.len() != ids.len() {
        return Ok(None);
    }
    let mut identities = BTreeMap::new();
    for row in rows {
        identities.insert(
            row.try_get::<i64, _>("id").map_err(database_error)?,
            row.try_get::<Value, _>("identity")
                .map_err(database_error)?,
        );
    }
    let entities = ids
        .iter()
        .map(|id| identities.remove(id).ok_or_else(invalid))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = sqlx::query(COMPARE)
        .bind(&ids)
        .bind(&request.stats)
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
    let mut facts = Vec::new();
    let mut totals = BTreeMap::new();
    for row in rows {
        let entity = row.try_get::<i64, _>("entity_id").map_err(database_error)?;
        let stat = row
            .try_get::<String, _>("stat_key")
            .map_err(database_error)?;
        let total = row
            .try_get::<Option<i64>, _>("total")
            .map_err(database_error)?
            .unwrap_or(0);
        totals.insert((entity, stat), total);
        if let Some(fact) = row
            .try_get::<Option<Value>, _>("fact")
            .map_err(database_error)?
        {
            facts.push(fact);
        }
    }
    let mut cells = Vec::new();
    for entity in &ids {
        for stat in &request.stats {
            let Some(total) = totals.get(&(*entity, stat.clone())) else {
                return Ok(None);
            };
            cells.push(json!({"entity_id":entity,"stat_key":stat,"total_count":total}));
        }
    }
    let mut value = json!({"entities":entities,"stats":{"facts":facts},"cells":cells});
    update_cells(&mut value)?;
    Ok(Some(value))
}

fn validate_receipted_row(value: &Value) -> Result<(), PortError> {
    let object = value.as_object().ok_or_else(invalid)?;
    if !object.contains_key("citation")
        || !object
            .get("entity_id")
            .and_then(Value::as_i64)
            .is_some_and(|id| id > 0)
    {
        return Err(invalid());
    }
    let mut ids = BTreeSet::new();
    citations(value, &mut ids)?;
    Ok(())
}

fn validate_section(value: &Value, key: &str) -> Result<(), PortError> {
    let rows = value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(invalid)?;
    let omitted = value["omitted_count"].as_u64().ok_or_else(invalid)?;
    if omitted.checked_add(rows.len() as u64).is_none()
        || value["truncated"].as_bool() != Some(omitted > 0)
    {
        return Err(invalid());
    }
    for row in rows {
        validate_receipted_row(row)?;
        if key == "facts" {
            if !row
                .get("stat_key")
                .and_then(Value::as_str)
                .is_some_and(|key| !key.is_empty())
                || row.get("value").is_none()
            {
                return Err(invalid());
            }
        } else {
            validate_receipted_row(&row["identity"])?;
        }
    }
    Ok(())
}

fn validate_receipts(name: ToolName, value: &Value) -> Result<(), PortError> {
    if name == ToolName::Compare {
        let entities = value["entities"].as_array().ok_or_else(invalid)?;
        for entity in entities {
            validate_receipted_row(entity)?;
        }
        validate_section(&value["stats"], "facts")?;
    } else {
        validate_receipted_row(&value["identity"])?;
        validate_section(&value["facts"], "facts")?;
        validate_section(&value["links"], "links")?;
        if let Some(abilities) = value.get("abilities") {
            validate_section(abilities, "links")?;
        }
    }
    Ok(())
}

fn update_cells(value: &mut Value) -> Result<(), PortError> {
    let facts = value["stats"]["facts"].as_array().ok_or_else(invalid)?;
    let mut counts = BTreeMap::<(u64, String), usize>::new();
    for fact in facts {
        let entity = fact["entity_id"].as_u64().ok_or_else(invalid)?;
        let stat = fact["stat_key"].as_str().ok_or_else(invalid)?;
        *counts.entry((entity, stat.into())).or_default() += 1;
    }
    let cells = value["cells"].as_array_mut().ok_or_else(invalid)?;
    let mut omitted_total = 0;
    for cell in cells {
        let entity = cell["entity_id"].as_u64().ok_or_else(invalid)?;
        let stat = cell["stat_key"].as_str().ok_or_else(invalid)?;
        let total = cell["total_count"].as_u64().ok_or_else(invalid)?;
        let count = counts.get(&(entity, stat.into())).copied().unwrap_or(0) as u64;
        if count > total {
            return Err(invalid());
        }
        let omitted = total - count;
        omitted_total += omitted;
        cell["omitted_count"] = json!(omitted);
        cell["status"] = json!(if total == 0 {
            "missing"
        } else if count == 0 {
            "omitted"
        } else if omitted > 0 {
            "partial"
        } else {
            "available"
        });
    }
    value["stats"]["omitted_count"] = json!(omitted_total);
    value["stats"]["truncated"] = json!(omitted_total > 0);
    Ok(())
}

fn trim_card(value: &mut Value) -> bool {
    for key in ["abilities", "facts", "links"] {
        if let Some(section) = value.get_mut(key).filter(|field| field.is_object()) {
            if trim_card(section) {
                return true;
            }
        }
    }
    for key in ["facts", "links"] {
        if let Some(rows) = value.get_mut(key).and_then(Value::as_array_mut) {
            if rows.pop().is_some() {
                let omitted = value["omitted_count"].as_u64().unwrap_or(0);
                value["omitted_count"] = json!(omitted.saturating_add(1));
                value["truncated"] = json!(true);
                return true;
            }
        }
    }
    false
}

fn bounded_value(name: ToolName, mut value: Value) -> Result<Option<Value>, PortError> {
    if validate_receipts(name, &value).is_err() {
        return Ok(None);
    }
    if name == ToolName::Compare {
        let Some(entities) = value["entities"].as_array() else {
            return Ok(None);
        };
        if !(2..=4).contains(&entities.len()) {
            return Ok(None);
        }
    } else if !value["identity"].is_object() || value["identity"].get("citation").is_none() {
        return Ok(None);
    }
    loop {
        let mut ids = BTreeSet::new();
        citations(&value, &mut ids)?;
        let result = ToolResult {
            call_id: String::new(),
            name,
            result: value.clone(),
            evidence_ids: ids.iter().map(ToString::to_string).collect(),
            is_error: false,
        };
        if !ids.is_empty() && fits(&result, ids.len())? {
            return Ok(Some(value));
        }
        if name == ToolName::Compare {
            if !trim_card(&mut value["stats"]) {
                return Ok(None);
            }
            update_cells(&mut value)?;
        } else if !trim_card(&mut value) {
            return Ok(None);
        }
    }
}

#[cfg(test)]
#[path = "pg_asset_tools_tests.rs"]
mod tests;
