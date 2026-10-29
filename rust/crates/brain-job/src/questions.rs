use sqlx::PgPool;

const HISTORY_CHARS: usize = 160;
const ANSWER_CHARS: usize = 600;

const QUESTIONS: &str =
    "SELECT to_char(recorded_at AT TIME ZONE 'Europe/Berlin','YYYY-MM-DD HH24:MI:SS'),
        coalesce(source,'-'),coalesce(route,'-'),status,
        coalesce(not_answered_stage||'/'||not_answered_reason,not_answered_reason,'-'),
        latency_ms,provider_rounds,coalesce(question,'-'),
        ARRAY(SELECT jsonb_array_elements_text(history)),answer_text
    FROM ops.answer_trace
    WHERE recorded_at >= $1::timestamptz
        AND ($2::text IS NULL
            OR ($2='not_answered' AND status<>'answered')
            OR status=$2)
    ORDER BY recorded_at DESC, id DESC
    LIMIT $3";

type Row = (
    String,
    String,
    String,
    String,
    String,
    i32,
    Option<i32>,
    String,
    Vec<String>,
    Option<String>,
);

pub struct Filter {
    pub since: chrono::DateTime<chrono::Utc>,
    pub status: Option<String>,
    pub limit: i64,
}

pub fn parse_since(
    value: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
    let minutes = match value.char_indices().last() {
        Some((index, unit @ ('d' | 'h' | 'm'))) => {
            let count: i64 = value[..index].parse()?;
            let factor = match unit {
                'd' => 1440,
                'h' => 60,
                _ => 1,
            };
            Some(
                count
                    .checked_mul(factor)
                    .filter(|total| (1..=52_560_000).contains(total)),
            )
        }
        _ => None,
    };
    if let Some(minutes) = minutes {
        let minutes = minutes.ok_or_else(|| anyhow::anyhow!("Ungültiger Zeitraum"))?;
        return Ok(now - chrono::Duration::minutes(minutes));
    }
    let date = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")?;
    date.and_hms_opt(0, 0, 0)
        .map(|start| start.and_utc())
        .ok_or_else(|| anyhow::anyhow!("Ungültiges Datum"))
}

fn printable(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if c.is_control() {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

fn short(text: &str, limit: usize) -> String {
    let flat = printable(&text.split_whitespace().collect::<Vec<_>>().join(" "));
    if flat.chars().count() <= limit {
        return flat;
    }
    let mut cut: String = flat.chars().take(limit).collect();
    cut.push('…');
    cut
}

pub async fn report(pool: &PgPool, filter: &Filter) -> anyhow::Result<()> {
    let rows: Vec<Row> = sqlx::query_as(QUESTIONS)
        .bind(filter.since.to_rfc3339())
        .bind(filter.status.as_deref())
        .bind(filter.limit)
        .fetch_all(pool)
        .await?;
    for (time, source, route, status, reason, latency, rounds, question, history, answer) in &rows {
        let rounds = rounds.map_or_else(|| "-".into(), |value| value.to_string());
        let [source, route, status, reason] =
            [source, route, status, reason].map(|text| printable(text));
        println!("{time}  {source}  {route}  {status}  reason={reason}  latency_ms={latency}  rounds={rounds}");
        println!("  question: {}", short(question, 4000));
        if !history.is_empty() {
            let turns: Vec<_> = history
                .iter()
                .map(|turn| short(turn, HISTORY_CHARS))
                .collect();
            println!("  history:  {}", turns.join(" | "));
        }
        if let Some(answer) = answer {
            println!("  answer:   {}", short(answer, ANSWER_CHARS));
        }
        println!();
    }
    println!("rows={}", rows.len());
    Ok(())
}

pub async fn purge(pool: &PgPool) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar("SELECT ops.purge_answer_trace()")
        .fetch_one(pool)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printed_fields_escape_terminal_controls() {
        assert_eq!(
            short("a\x1b[2J\x1b[H  b\n\0c", 100),
            "a\\u{1b}[2J\\u{1b}[H b \\u{0}c"
        );
        assert_eq!(short("abcdef", 3), "abc…");
        assert_eq!(printable("ok"), "ok");
    }
}
