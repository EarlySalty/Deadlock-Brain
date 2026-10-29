use serde_json::{json, Value};
use sqlx::PgConnection;

const QUERY: &str = include_str!("ability_patch_history.sql");
const LIMIT: i64 = 12;
const ATTEMPTS: u8 = 2;
const MEANING: &str = "Datierte Patchnotiz-Zeilen zu dieser Fähigkeit, neueste zuerst. Bei from X to Y galt X bis released_at und Y gilt ab released_at, solange keine neuere Zeile es ändert. Regeln, die nur hier stehen, gelten ab released_at.";

pub(super) async fn attach(connection: &mut PgConnection, card: &mut Value, ability: i64) {
    let Some(lines) = lines(connection, ability).await else {
        return;
    };
    let changes: Vec<Value> = lines.iter().filter_map(change).collect();
    if !changes.is_empty() {
        card["history"] = section(changes);
    }
}

fn section(changes: Vec<Value>) -> Value {
    json!({"meaning":MEANING,"changes":changes,"truncated":false,"omitted_count":0})
}

async fn lines(connection: &mut PgConnection, ability: i64) -> Option<Vec<Value>> {
    for attempt in 1..=ATTEMPTS {
        if let Some(rows) = attempt_lines(connection, ability, attempt).await {
            return Some(rows);
        }
    }
    None
}

async fn attempt_lines(
    connection: &mut PgConnection,
    ability: i64,
    attempt: u8,
) -> Option<Vec<Value>> {
    let mut savepoint = sqlx::Connection::begin(&mut *connection).await.ok()?;
    let started = std::time::Instant::now();
    let history = match sqlx::query_scalar::<_, Value>(QUERY)
        .bind(ability)
        .bind(LIMIT)
        .fetch_all(&mut *savepoint)
        .await
    {
        Ok(rows) => savepoint.commit().await.map(|_| rows),
        Err(error) => {
            let _ = savepoint.rollback().await;
            Err(error)
        }
    };
    eprintln!(
        "{}",
        json!({
            "event": "brain_ability_patch_history",
            "ok": history.is_ok(),
            "attempt": attempt,
            "rows": history.as_ref().map_or(0, Vec::len),
            "elapsed_ms": started.elapsed().as_millis(),
        })
    );
    history.ok()
}

fn change(line: &Value) -> Option<Value> {
    let text = line["line_text"].as_str().filter(|text| !text.is_empty())?;
    let citation = line["citation"].as_str().filter(|id| !id.is_empty())?;
    let revision = line["source_revision"]
        .as_str()
        .filter(|revision| !revision.is_empty())?;
    let released_at = line["released_at"]
        .as_str()
        .filter(|released| !released.is_empty())?;
    Some(json!({
        "patch_label":line["patch"].as_str().map(str::trim),
        "released_at":released_at,
        "line_text":text,
        "citation":citation,
        "source_revision":revision,
        "patch":null
    }))
}
