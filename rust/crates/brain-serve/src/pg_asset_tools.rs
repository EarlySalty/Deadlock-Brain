use brain_contracts::tools::{ToolDefinition, ToolName, ToolRequest, ToolResult, ToolSubrequest};
use brain_contracts::PortError;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[path = "pg_asset_queries.rs"]
mod queries;
pub(crate) use queries::{attach_successor, read};

pub const MAX_RESULT_BYTES: usize = 16_384;

pub fn canonical(name: ToolName) -> bool {
    matches!(
        name,
        ToolName::GetItem | ToolName::GetAbility | ToolName::Compare
    )
}

pub fn historical(request: &ToolRequest) -> bool {
    let patch = match request.subrequest() {
        ToolSubrequest::GetItem(request) => &request.patch,
        ToolSubrequest::GetAbility(request) => &request.patch,
        ToolSubrequest::Compare(request) => &request.patch,
        _ => return false,
    };
    patch.as_deref().is_some_and(|patch| patch != "current")
}

pub fn definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: ToolName::GetItem,
            description: "Liest die veröffentlichte Itemkarte aus kanonischen Spielwerten. item ist die positive kanonische Datenbank-ID aus find_entity mit kind item, keine öffentliche Spiel-ID. Werte, Bedingungen, Einheiten und Quellenbelege unverändert beachten. Fehlender Patch oder current bedeutet beobachteter Quellenstand; historische Patches sind nicht verfügbar. Fehlende oder ausgelassene Fakten nicht ergänzen. lineage nennt frühere Namen mit Patch und Quelle, den Status removed samt Nachfolger und die letzten Patchnotiz-Zeilen. Bei removed mit Nachfolger enthält successor_item die aktuelle Itemkarte des Nachfolgers mit Werten, Kosten, Stufe und Historie; diese Werte für die Bewertung verwenden. Fragt jemand, ob das alte Item gut ist oder sich auf einem Helden lohnt, gib mit diesen Werten und den Heldendaten eine klare, begründete Einschätzung zum Nachfolger; fehlende Metastatistiken verhindern diese Einschätzung nicht, sie werden nur als fehlend benannt. Immer den aktuellen Shopnamen verwenden und alte Namen nur als früheren Namen nennen.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["item"],"properties":{
                "item":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
        ToolDefinition {
            name: ToolName::GetAbility,
            description: "Liest eine veröffentlichte Fähigkeitskarte. Genau eine Auswahl verwenden: ability als kanonische Datenbank-ID aus find_entity oder hero als kanonische Helden-ID zusammen mit slot eins bis vier, wobei vier die ultimative Fähigkeit ist. Keine öffentlichen Spiel-IDs verwenden. Mehrdeutige Slots werden nicht geraten. Werte, Upgrades, Bedingungen und Quellenbelege unverändert beachten. Fehlender Patch oder current bedeutet beobachteter Quellenstand; historische Patches sind nicht verfügbar. history.changes enthält datierte Patchnotiz-Zeilen zu dieser Fähigkeit, neueste zuerst. Nennt der Nutzer einen früheren Wert oder fragt nach einer Regel wie Heilanteil oder Ausdehnzeit, die nicht in den Werten steht, diese Zeilen mit Datum nennen: bei from X to Y galt X bis released_at und Y gilt seitdem, solange keine neuere Zeile es ändert. Fehlende oder ausgelassene Fakten nicht ergänzen.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{
                "ability":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "hero":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "slot":{"type":"integer","minimum":1,"maximum":4},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
        ToolDefinition {
            name: ToolName::Compare,
            description: "Vergleicht zwei bis vier verschiedene Helden oder Items anhand ein bis zwölf verschiedener kanonischer Statistikschlüssel. entities enthält positive kanonische Datenbank-IDs aus find_entity, keine öffentlichen Spiel-IDs. stats enthält vorhandene Schlüssel aus kanonischen Karten, keine geschätzten Metriken. Bedingungen und Einheiten sind Teil jedes Werts. Alle Identitäten bleiben erhalten; cells kennzeichnet fehlende oder ausgelassene Werte, die nicht ergänzt werden dürfen. Fehlender Patch oder current bedeutet beobachteter Quellenstand; historische Patches sind nicht verfügbar.".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"required":["entities","stats"],"properties":{
                "entities":{"type":"array","minItems":2,"maxItems":4,"items":{"type":"integer","minimum":1,"maximum":i64::MAX}},
                "stats":{"type":"array","minItems":1,"maxItems":12,"items":{"type":"string","minLength":1,"maxLength":128}},
                "patch":{"type":"string","minLength":1,"maxLength":128}}}),
        },
    ]
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Unvollständige kanonische Spielwerte".into())
}

fn positive_id(value: &Value) -> Result<u64, PortError> {
    value
        .as_u64()
        .filter(|id| *id > 0 && *id <= i64::MAX as u64)
        .ok_or_else(invalid)
}

fn receipt(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    positive_id(&value["entity_id"])?;
    row_receipt(value, ids)
}

fn row_receipt(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    let row = value.as_object().ok_or_else(invalid)?;
    let citation = row
        .get("citation")
        .and_then(Value::as_str)
        .filter(|citation| {
            !citation.trim().is_empty()
                && citation.len() <= 512
                && !citation.chars().any(char::is_control)
        })
        .ok_or_else(invalid)?;
    if row.get("patch") != Some(&Value::Null)
        || !row
            .get("source_revision")
            .and_then(Value::as_str)
            .is_some_and(|revision| {
                !revision.trim().is_empty() && !revision.chars().any(char::is_control)
            })
    {
        return Err(invalid());
    }
    ids.insert(citation.into());
    Ok(())
}

fn fact_fields(value: &Value) -> Result<(), PortError> {
    if !value["stat_key"]
        .as_str()
        .is_some_and(|key| !key.is_empty())
        || value.get("value").is_none()
        || !value["condition"].is_object()
    {
        return Err(invalid());
    }
    Ok(())
}

fn qualifier_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    if let Some(qualifiers) = value.get("qualifiers") {
        for qualifier in qualifiers.as_array().ok_or_else(invalid)? {
            row_receipt(qualifier, ids)?;
            if !qualifier["key"].as_str().is_some_and(|key| !key.is_empty())
                || qualifier.get("value").is_none()
            {
                return Err(invalid());
            }
        }
    }
    Ok(())
}

fn receipted_fact(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    receipt(value, ids)?;
    qualifier_receipts(value, ids)?;
    fact_fields(value)
}

fn identity_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    receipt(value, ids)?;
    let entity = positive_id(&value["entity_id"])?;
    if !value["key"].as_str().is_some_and(|key| !key.is_empty())
        || !matches!(value["kind"].as_str(), Some("hero" | "item" | "ability"))
    {
        return Err(invalid());
    }
    let facts = match value.get("facts") {
        Some(facts) => facts.as_array().ok_or_else(invalid)?.as_slice(),
        None => &[],
    };
    for fact in facts {
        row_receipt(fact, ids)?;
        fact_fields(fact)?;
        if fact
            .get("entity_id")
            .is_some_and(|id| id.as_u64() != Some(entity))
        {
            return Err(invalid());
        }
    }
    for key in ["name_en", "name_de"] {
        if let Some(name) = value.get(key).filter(|name| !name.is_null()) {
            if !name.is_string()
                || !facts
                    .iter()
                    .any(|fact| fact["stat_key"] == key && fact["value"] == *name)
            {
                return Err(invalid());
            }
        }
    }
    Ok(())
}

fn section_receipts(value: &Value, key: &str, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    let rows = value[key].as_array().ok_or_else(invalid)?;
    let omitted = value["omitted_count"].as_u64().ok_or_else(invalid)?;
    if omitted.checked_add(rows.len() as u64).is_none()
        || value["truncated"].as_bool() != Some(omitted > 0)
    {
        return Err(invalid());
    }
    for row in rows {
        if key == "facts" {
            receipted_fact(row, ids)?;
        } else {
            receipt(row, ids)?;
            qualifier_receipts(row, ids)?;
            identity_receipts(&row["identity"], ids)?;
            if row["entity_id"] != row["identity"]["entity_id"] {
                return Err(invalid());
            }
        }
    }
    Ok(())
}

fn lineage_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    let Some(lineage) = value.get("lineage") else {
        return Ok(());
    };
    if let Some(removed) = lineage.get("removed").filter(|removed| !removed.is_null()) {
        row_receipt(removed, ids)?;
    }
    for name in lineage["former_names"].as_array().ok_or_else(invalid)? {
        row_receipt(name, ids)?;
    }
    for predecessor in lineage["predecessors"].as_array().ok_or_else(invalid)? {
        row_receipt(&predecessor["removed"], ids)?;
    }
    let history = &lineage["history"];
    let omitted = history["omitted_count"].as_u64().ok_or_else(invalid)?;
    if history["truncated"].as_bool() != Some(omitted > 0) {
        return Err(invalid());
    }
    for change in history["changes"].as_array().ok_or_else(invalid)? {
        row_receipt(change, ids)?;
    }
    Ok(())
}

fn ability_history_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    let Some(history) = value.get("history") else {
        return Ok(());
    };
    let omitted = history["omitted_count"].as_u64().ok_or_else(invalid)?;
    if history["truncated"].as_bool() != Some(omitted > 0) {
        return Err(invalid());
    }
    for change in history["changes"].as_array().ok_or_else(invalid)? {
        row_receipt(change, ids)?;
        if !change["line_text"].is_string() || !change["released_at"].is_string() {
            return Err(invalid());
        }
    }
    Ok(())
}

fn successor_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    let Some(successor) = value.get("successor_item") else {
        return Ok(());
    };
    let expected = &value["lineage"]["removed"]["successor"]["entity_id"];
    if value["lineage"]["status"] != "removed"
        || successor.get("successor_item").is_some()
        || successor["identity"]["kind"] != "item"
        || successor["identity"]["entity_id"] != *expected
    {
        return Err(invalid());
    }
    entity_receipts(successor, ids)
}

fn entity_receipts(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    identity_receipts(&value["identity"], ids)?;
    lineage_receipts(value, ids)?;
    let entity = positive_id(&value["identity"]["entity_id"])?;
    section_receipts(&value["facts"], "facts", ids)?;
    section_receipts(&value["links"], "links", ids)?;
    if value["facts"]["facts"]
        .as_array()
        .ok_or_else(invalid)?
        .iter()
        .any(|row| row["entity_id"].as_u64() != Some(entity))
    {
        return Err(invalid());
    }
    Ok(())
}

fn card_receipts(value: &Value) -> Result<BTreeSet<String>, PortError> {
    let mut ids = BTreeSet::new();
    entity_receipts(value, &mut ids)?;
    successor_receipts(value, &mut ids)?;
    ability_history_receipts(value, &mut ids)?;
    if let Some(abilities) = value.get("abilities") {
        section_receipts(abilities, "links", &mut ids)?;
        if abilities["links"]
            .as_array()
            .ok_or_else(invalid)?
            .is_empty()
        {
            return Err(invalid());
        }
    }
    Ok(ids)
}

fn selection_link(row: &Value, request: &ToolRequest) -> bool {
    let ToolSubrequest::GetAbility(request) = request.subrequest() else {
        return false;
    };
    let (Some(hero), Some(slot)) = (request.hero, request.slot) else {
        return false;
    };
    row["kind"] == "has_ability"
        && row["from_entity_id"].as_u64() == Some(hero)
        && row["slot"].as_u64() == Some(u64::from(slot))
}

fn card_matches(value: &Value, request: &ToolRequest) -> bool {
    let entity = value["identity"]["entity_id"].as_u64();
    match request.subrequest() {
        ToolSubrequest::GetItem(request) => {
            value["identity"]["kind"] == "item" && entity == Some(request.item)
        }
        ToolSubrequest::GetAbility(ability) if value["identity"]["kind"] == "ability" => {
            if let Some(ability) = ability.ability {
                return entity == Some(ability);
            }
            let Some(rows) = value["abilities"]["links"].as_array() else {
                return false;
            };
            let matching: Vec<_> = rows
                .iter()
                .filter(|row| selection_link(row, request))
                .collect();
            if matching.len() != 1 {
                return false;
            }
            let row = matching[0];
            let qualifiers = row["qualifiers"].as_array();
            entity.is_some()
                && row["to_entity_id"].as_u64() == entity
                && row["entity_id"].as_u64() == ability.hero
                && row["identity"]["kind"] == "hero"
                && qualifiers.is_some_and(|qualifiers| {
                    let slots: Vec<_> = qualifiers.iter().filter(|q| q["key"] == "slot").collect();
                    slots.len() == 1 && slots[0]["value"] == row["slot"]
                })
        }
        _ => false,
    }
}

fn comparison_layout(value: &Value) -> Result<BTreeSet<(u64, String)>, PortError> {
    let identities = value["entities"].as_array().ok_or_else(invalid)?;
    let cells = value["cells"].as_array().ok_or_else(invalid)?;
    if !(2..=4).contains(&identities.len()) || !(2..=48).contains(&cells.len()) {
        return Err(invalid());
    }
    let mut entities = BTreeSet::new();
    for identity in identities {
        if !entities.insert(positive_id(&identity["entity_id"])?)
            || !matches!(identity["kind"].as_str(), Some("hero" | "item"))
        {
            return Err(invalid());
        }
    }
    let mut keys = BTreeSet::new();
    let mut grid = BTreeSet::new();
    for cell in cells {
        let entity = positive_id(&cell["entity_id"])?;
        let key = cell["stat_key"].as_str().ok_or_else(invalid)?;
        if !entities.contains(&entity)
            || key.is_empty()
            || key.len() > 128
            || !key
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_alphabetic())
            || !key
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.'))
            || cell["total_count"].as_u64().is_none()
            || !grid.insert((entity, key.to_owned()))
        {
            return Err(invalid());
        }
        keys.insert(key);
    }
    if !(1..=12).contains(&keys.len()) || grid.len() != entities.len() * keys.len() {
        return Err(invalid());
    }
    Ok(grid)
}

fn update_cells(value: &mut Value) -> Result<(), PortError> {
    let grid = comparison_layout(value)?;
    let facts = value["stats"]["facts"].as_array().ok_or_else(invalid)?;
    let mut counts = BTreeMap::<(u64, String), u64>::new();
    for fact in facts {
        let entity = positive_id(&fact["entity_id"])?;
        let stat = fact["stat_key"].as_str().ok_or_else(invalid)?;
        let cell = (entity, stat.to_owned());
        if !grid.contains(&cell) {
            return Err(invalid());
        }
        let count = counts.entry(cell).or_default();
        *count = count.checked_add(1).ok_or_else(invalid)?;
    }
    let cells = value["cells"].as_array_mut().ok_or_else(invalid)?;
    let mut omitted_total = 0_u64;
    for cell in cells {
        let entity = positive_id(&cell["entity_id"])?;
        let stat = cell["stat_key"].as_str().ok_or_else(invalid)?;
        let total = cell["total_count"].as_u64().ok_or_else(invalid)?;
        let count = counts.get(&(entity, stat.into())).copied().unwrap_or(0);
        let omitted = total.checked_sub(count).ok_or_else(invalid)?;
        omitted_total = omitted_total.checked_add(omitted).ok_or_else(invalid)?;
        let row = cell.as_object_mut().ok_or_else(invalid)?;
        row.insert("omitted_count".into(), json!(omitted));
        row.insert(
            "status".into(),
            json!(if total == 0 {
                "missing"
            } else if count == 0 {
                "omitted"
            } else if omitted > 0 {
                "partial"
            } else {
                "available"
            }),
        );
    }
    let stats = value["stats"].as_object_mut().ok_or_else(invalid)?;
    stats.insert("omitted_count".into(), json!(omitted_total));
    stats.insert("truncated".into(), json!(omitted_total > 0));
    Ok(())
}

fn comparison_receipts(value: &Value) -> Result<BTreeSet<String>, PortError> {
    let mut ids = BTreeSet::new();
    for identity in value["entities"].as_array().ok_or_else(invalid)? {
        identity_receipts(identity, &mut ids)?;
    }
    section_receipts(&value["stats"], "facts", &mut ids)?;
    Ok(ids)
}

fn trim_section(
    value: &mut Value,
    key: &str,
    protected: impl Fn(&Value) -> bool,
) -> Result<bool, PortError> {
    let omitted = value["omitted_count"].as_u64().ok_or_else(invalid)?;
    let rows = value[key].as_array_mut().ok_or_else(invalid)?;
    let Some(index) = rows.iter().rposition(|row| !protected(row)) else {
        return Ok(false);
    };
    let omitted = omitted.checked_add(1).ok_or_else(invalid)?;
    rows.remove(index);
    let section = value.as_object_mut().ok_or_else(invalid)?;
    section.insert("omitted_count".into(), json!(omitted));
    section.insert("truncated".into(), json!(true));
    Ok(true)
}

fn trim_history(value: &mut Value) -> Result<bool, PortError> {
    match value
        .get_mut("lineage")
        .and_then(|lineage| lineage.get_mut("history"))
    {
        Some(history) => trim_section(history, "changes", |_| false),
        None => Ok(false),
    }
}

fn trim_successor(value: &mut Value, section: &str) -> Result<bool, PortError> {
    match value.get_mut("successor_item") {
        Some(successor) if section == "changes" => trim_history(successor),
        Some(successor) => trim_section(&mut successor[section], section, |_| false),
        None => Ok(false),
    }
}

pub(crate) fn trim_item_card(value: &mut Value) -> Result<bool, PortError> {
    if trim_history(value)? || trim_successor(value, "changes")? {
        return Ok(true);
    }
    if trim_section(&mut value["links"], "links", |_| false)? || trim_successor(value, "links")? {
        return Ok(true);
    }
    if value.get("successor_item").is_some()
        && trim_section(&mut value["facts"], "facts", |_| false)?
    {
        return Ok(true);
    }
    trim_successor(value, "facts")
}

fn trim_card(value: &mut Value, request: &ToolRequest) -> Result<bool, PortError> {
    if request.name() == ToolName::GetItem {
        return trim_item_card(value);
    }
    if trim_history(value)? {
        return Ok(true);
    }
    if let Some(history) = value.get_mut("history") {
        if trim_section(history, "changes", |_| false)? {
            return Ok(true);
        }
    }
    if trim_section(&mut value["facts"], "facts", |_| false)? {
        return Ok(true);
    }
    if trim_section(&mut value["links"], "links", |_| false)? {
        return Ok(true);
    }
    if let Some(abilities) = value.get_mut("abilities") {
        if abilities["links"].as_array().ok_or_else(invalid)?.len() > 1 {
            return trim_section(abilities, "links", |row| selection_link(row, request));
        }
    }
    Ok(false)
}

pub(crate) fn trim_execution(value: &mut Value, request: &ToolRequest) -> Result<bool, PortError> {
    match request.name() {
        ToolName::Compare => {
            let trimmed = trim_section(&mut value["stats"], "facts", |_| false)?;
            update_cells(value)?;
            Ok(trimmed)
        }
        ToolName::GetItem | ToolName::GetAbility => trim_card(value, request),
        _ => Err(invalid()),
    }
}

pub fn bounded_card_result(
    call_id: &str,
    request: &ToolRequest,
    value: Value,
) -> Result<Option<ToolResult>, PortError> {
    if !matches!(request.name(), ToolName::GetItem | ToolName::GetAbility) {
        return Err(invalid());
    }
    if historical(request) || !card_matches(&value, request) {
        return Ok(None);
    }
    bounded_result(call_id, request, value, false)
}

pub fn bounded_comparison_result(
    call_id: &str,
    request: &ToolRequest,
    mut value: Value,
) -> Result<Option<ToolResult>, PortError> {
    let ToolSubrequest::Compare(compare) = request.subrequest() else {
        return Err(invalid());
    };
    let expected = compare
        .entities
        .iter()
        .flat_map(|entity| compare.stats.iter().map(|key| (*entity, key.clone())))
        .collect::<BTreeSet<_>>();
    if historical(request)
        || comparison_layout(&value).ok().as_ref() != Some(&expected)
        || update_cells(&mut value).is_err()
    {
        return Ok(None);
    }
    bounded_result(call_id, request, value, true)
}

fn bounded_result(
    call_id: &str,
    request: &ToolRequest,
    value: Value,
    comparison: bool,
) -> Result<Option<ToolResult>, PortError> {
    if call_id.trim().is_empty() || call_id.len() > 512 || call_id.chars().any(char::is_control) {
        return Err(PortError::InvalidResponse(
            "Ungültige Werkzeugaufruf-ID".into(),
        ));
    }
    let mut result = ToolResult {
        call_id: call_id.into(),
        name: request.name(),
        result: value,
        evidence_ids: Vec::new(),
        is_error: false,
    };
    loop {
        let receipts = if comparison {
            comparison_receipts(&result.result)
        } else {
            card_receipts(&result.result)
        };
        let Ok(receipts) = receipts else {
            return Ok(None);
        };
        result.evidence_ids = receipts.into_iter().collect();
        let mut worst = result.clone();
        worst.call_id = "\"".repeat(512);
        let bytes = serde_json::to_vec(&worst).map_err(|_| invalid())?.len();
        if bytes < MAX_RESULT_BYTES && result.evidence_ids.len() <= 100 {
            return Ok(Some(result));
        }
        if comparison {
            if !trim_section(&mut result.result["stats"], "facts", |_| false)? {
                return Ok(None);
            }
            update_cells(&mut result.result)?;
        } else if !trim_card(&mut result.result, request)? {
            return Ok(None);
        }
    }
}

#[cfg(test)]
#[path = "pg_asset_tools_tests.rs"]
mod tests;
