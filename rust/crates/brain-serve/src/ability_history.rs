use serde_json::{json, Value};
use sqlx::PgConnection;

const QUERY: &str = include_str!("ability_patch_history.sql");
const LIMIT: i64 = 12;
const CANDIDATE_CAP: i64 = 2000;
const ATTEMPTS: u8 = 2;
const TIMEOUT_SQLSTATE: &str = "57014";
const MEANING: &str = "Datierte Patchnotiz-Zeilen zu dieser Fähigkeit, neueste zuerst. Bei from X to Y galt X bis released_at und Y gilt ab released_at, solange keine neuere Zeile es ändert. Regeln, die nur hier stehen, gelten ab released_at. Bei truncated fehlen ältere Zeilen; bei available false war die Historie nicht abrufbar, dann keine Aussage über frühere Werte treffen.";

pub(super) async fn attach(connection: &mut PgConnection, card: &mut Value, ability: i64) {
    card["history"] = match lines(connection, ability).await {
        Some(result) => section(&result),
        None => unavailable(),
    };
}

fn section(result: &Value) -> Value {
    let rows = result["lines"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let changes: Vec<Value> = rows.iter().filter_map(change).collect();
    let total = result["total"].as_u64().unwrap_or(0);
    let omitted = total.saturating_sub(changes.len() as u64);
    let lower_bound = result["lower_bound"].as_bool().unwrap_or(false);
    json!({
        "meaning":MEANING,
        "available":true,
        "changes":changes,
        "truncated":omitted > 0,
        "omitted_count":omitted,
        "omitted_count_is_lower_bound":lower_bound
    })
}

fn unavailable() -> Value {
    json!({
        "meaning":MEANING,
        "available":false,
        "reason":"patch_history_unavailable",
        "changes":[],
        "truncated":false,
        "omitted_count":0,
        "omitted_count_is_lower_bound":false
    })
}

async fn lines(connection: &mut PgConnection, ability: i64) -> Option<Value> {
    for attempt in 1..=ATTEMPTS {
        match attempt_lines(connection, ability, attempt).await {
            Ok(result) => return Some(result),
            Err(error) if attempt < ATTEMPTS && timed_out(&error) => continue,
            Err(_) => return None,
        }
    }
    None
}

fn timed_out(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|error| error.code())
        .is_some_and(|code| code == TIMEOUT_SQLSTATE)
}

async fn attempt_lines(
    connection: &mut PgConnection,
    ability: i64,
    attempt: u8,
) -> Result<Value, sqlx::Error> {
    let mut savepoint = sqlx::Connection::begin(&mut *connection).await?;
    let started = std::time::Instant::now();
    let history = match sqlx::query_scalar::<_, Value>(QUERY)
        .bind(ability)
        .bind(LIMIT)
        .bind(CANDIDATE_CAP)
        .fetch_one(&mut *savepoint)
        .await
    {
        Ok(result) => savepoint.commit().await.map(|_| result),
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
            "total": history.as_ref().map_or(0, |result| result["total"].as_u64().unwrap_or(0)),
            "elapsed_ms": started.elapsed().as_millis(),
        })
    );
    history
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
