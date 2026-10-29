use brain_contracts::{
    tools::{ListPatchesRequest, PatchKind, PATCH_LIST_MAX_OFFSET, PATCH_LIST_PAGE},
    PortError, ToolDefinition, ToolName,
};
use serde_json::{json, Value};
use sqlx::PgConnection;

const QUERY: &str = "SELECT jsonb_build_object(
        'ledger_id',l.ledger_id,'label',l.label,'kind',l.kind,
        'released_at',to_char(l.released_at AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'),
        'official_url',l.source_url,'posts',l.posts,
        'citation',l.released_at_receipt_id::text,'source_revision',l.source_revision,'patch',NULL)
    FROM serve.patch_ledger l
    WHERE ($1::text IS NULL OR (l.released_at AT TIME ZONE 'UTC')::date >= $1::text::date)
        AND ($2::text IS NULL OR (l.released_at AT TIME ZONE 'UTC')::date <= $2::text::date)
        AND ($3::text IS NULL OR l.kind=$3::text)
    ORDER BY l.released_at,l.ledger_id";

pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::ListPatches,
        description: "Listet alle Deadlock-Patches aus dem Patch-Ledger chronologisch vom ältesten zum neuesten: große Updates (major), Gameplay-Patches (gameplay) und Hotfixes (hotfix). Jede Zeile hat ledger_id, label (Titel), kind, released_at (UTC-Veröffentlichung), official_url und posts (Zahl der zusammengeführten Quellenbeiträge). Doppelte Beiträge sind bereits zusammengeführt. since und until filtern nach UTC-Datum JJJJ-MM-TT (beide inklusiv), kind nach Art. Für die neuesten Patches latest mit der gewünschten Anzahl angeben; die Zeilen bleiben chronologisch. Ohne latest kommen bis zu 25 Patches ab offset; weitere Seiten mit next_offset abrufen. total nennt die Zahl aller passenden Patches. Für Änderungen an einem Helden, Item oder einer Fähigkeit patch_history verwenden.".into(),
        input_schema: json!({
            "type":"object","additionalProperties":false,
            "properties":{
                "since":{"type":"string","minLength":10,"maxLength":10},
                "until":{"type":"string","minLength":10,"maxLength":10},
                "kind":{"type":"string","enum":PatchKind::ALL.map(PatchKind::as_str)},
                "latest":{"type":"integer","minimum":1,"maximum":PATCH_LIST_PAGE},
                "offset":{"type":"integer","minimum":0,"maximum":PATCH_LIST_MAX_OFFSET}
            }
        }),
    }
}

fn page(rows: Vec<Value>, request: &ListPatchesRequest) -> Value {
    let total = rows.len();
    let page = PATCH_LIST_PAGE as usize;
    let start = match request.latest {
        Some(latest) => total.saturating_sub(latest as usize),
        None => (request.offset.unwrap_or(0) as usize).min(total),
    };
    let end = match request.latest {
        Some(_) => total,
        None => start.saturating_add(page).min(total),
    };
    let patches: Vec<Value> = rows.into_iter().skip(start).take(end - start).collect();
    let next = (end < total).then_some(end);
    json!({
        "order":"oldest_first","total":total,"offset":start,"latest":request.latest,
        "next_offset":next,"truncated":next.is_some(),"omitted_count":total - patches.len(),
        "patches":patches
    })
}

pub(super) async fn read(
    connection: &mut PgConnection,
    request: &ListPatchesRequest,
) -> Result<Option<Value>, PortError> {
    let rows = sqlx::query_scalar::<_, Value>(QUERY)
        .bind(request.since.as_deref())
        .bind(request.until.as_deref())
        .bind(request.kind.map(PatchKind::as_str))
        .fetch_all(&mut *connection)
        .await
        .map_err(super::database_error)?;
    let value = page(rows, request);
    let empty = value["patches"].as_array().is_none_or(Vec::is_empty);
    Ok((!empty).then_some(value))
}

pub(super) fn trim(object: &mut serde_json::Map<String, Value>) -> bool {
    let newest_first = object.get("latest").is_some_and(Value::is_u64);
    let Some(patches) = object.get_mut("patches").and_then(Value::as_array_mut) else {
        return false;
    };
    if patches.len() <= 1 {
        return false;
    }
    if newest_first {
        patches.remove(0);
    } else {
        patches.pop();
    }
    let shown = patches.len() as u64;
    let offset = object.get("offset").and_then(Value::as_u64).unwrap_or(0);
    let total = object.get("total").and_then(Value::as_u64).unwrap_or(0);
    if newest_first {
        object.insert("offset".into(), json!(offset + 1));
    } else {
        object.insert("next_offset".into(), json!(offset + shown));
    }
    object.insert("omitted_count".into(), json!(total.saturating_sub(shown)));
    object.insert("truncated".into(), json!(true));
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(count: u64) -> Vec<Value> {
        (1..=count).map(|id| json!({"ledger_id":id})).collect()
    }

    fn ids(value: &Value) -> Vec<u64> {
        value["patches"]
            .as_array()
            .map(|rows| {
                rows.iter()
                    .filter_map(|row| row["ledger_id"].as_u64())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn request(latest: Option<u32>, offset: Option<u32>) -> ListPatchesRequest {
        ListPatchesRequest {
            since: None,
            until: None,
            kind: None,
            latest,
            offset,
        }
    }

    #[test]
    fn pages_stay_oldest_first() {
        let first = page(rows(50), &request(None, None));
        assert_eq!(ids(&first), (1..=25).collect::<Vec<_>>());
        assert_eq!(first["next_offset"], 25);
        let last = page(rows(50), &request(None, Some(40)));
        assert_eq!(ids(&last), (41..=50).collect::<Vec<_>>());
        assert_eq!(last["next_offset"], Value::Null);
        let latest = page(rows(50), &request(Some(3), None));
        assert_eq!(ids(&latest), [48, 49, 50]);
        assert_eq!(latest["truncated"], false);
    }

    #[test]
    fn trimming_moves_next_offset_back() {
        let mut value = page(rows(50), &request(None, Some(10)));
        let object = value.as_object_mut().map(trim);
        assert_eq!(object, Some(true));
        assert_eq!(value["next_offset"], 34);
        assert_eq!(ids(&value).last(), Some(&34));
    }

    #[test]
    fn latest_trimming_keeps_the_newest_patch() {
        let mut value = page(rows(50), &request(Some(3), None));
        let object = value.as_object_mut().map(trim);
        assert_eq!(object, Some(true));
        assert_eq!(ids(&value), [49, 50]);
        assert_eq!(value["offset"], 48);
        assert_eq!(value["next_offset"], Value::Null);
        assert_eq!(value["omitted_count"], 48);
    }

    #[test]
    fn single_row_is_never_trimmed_to_empty() {
        let mut value = page(rows(1), &request(None, None));
        let object = value.as_object_mut().map(trim);
        assert_eq!(object, Some(false));
        assert_eq!(ids(&value), [1]);
    }

    #[test]
    fn offset_beyond_total_gives_empty_page() {
        let value = page(rows(5), &request(None, Some(40)));
        assert!(ids(&value).is_empty());
        assert_eq!(value["offset"], 5);
        assert_eq!(value["next_offset"], Value::Null);
    }
}
