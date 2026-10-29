use anyhow::Context;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::{collections::HashMap, time::Duration};

pub const PROBE_SET: &str = include_str!("../probes/answer-probes.json");
pub const TOKEN_SECRET: &str = "DISCORD_BRAIN_CLIENT_TOKEN";
pub const DEFAULT_URL: &str = "http://127.0.0.1:8788";
const PROBE_USER_ID: &str = "100000000000000001";
const REQUEST_PREFIX: &str = "probe-guard-";
const PAUSE: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(620);
const ANSWER_CHARS: usize = 8000;

#[derive(Deserialize, Clone)]
pub struct Probe {
    pub id: String,
    pub kind: String,
    pub topic: String,
    pub turns: Vec<String>,
    pub expect: Vec<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Grade {
    Pass,
    Partial,
    Fail,
    Unavailable,
}

impl Grade {
    pub fn as_str(self) -> &'static str {
        match self {
            Grade::Pass => "pass",
            Grade::Partial => "partial",
            Grade::Fail => "fail",
            Grade::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pass" => Some(Grade::Pass),
            "partial" => Some(Grade::Partial),
            "fail" => Some(Grade::Fail),
            "unavailable" => Some(Grade::Unavailable),
            _ => None,
        }
    }

    fn rank(self) -> Option<u8> {
        match self {
            Grade::Pass => Some(2),
            Grade::Partial => Some(1),
            Grade::Fail => Some(0),
            Grade::Unavailable => None,
        }
    }
}

pub struct Outcome {
    pub probe: Probe,
    pub status: String,
    pub http: u16,
    pub answer: String,
    pub latency_ms: u64,
    pub grade: Grade,
    pub hits: usize,
}

pub fn probes() -> anyhow::Result<Vec<Probe>> {
    let probes: Vec<Probe> = serde_json::from_str(PROBE_SET)?;
    anyhow::ensure!(
        !probes.is_empty()
            && probes
                .iter()
                .all(|probe| !probe.turns.is_empty() && !probe.expect.is_empty()),
        "Ungültiger Prüfsatz"
    );
    Ok(probes)
}

pub fn probe_set_sha256() -> String {
    Sha256::digest(PROBE_SET.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn normalize(answer: &str) -> String {
    let chars: Vec<char> = answer.chars().collect();
    let mut out = String::with_capacity(answer.len());
    for (index, &c) in chars.iter().enumerate() {
        let before = index > 0 && chars[index - 1].is_ascii_digit();
        let group = || {
            chars[index + 1..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .count()
                == 3
                && chars
                    .get(index + 4)
                    .is_none_or(|next| !next.is_ascii_digit())
        };
        match c {
            ' ' | '\u{a0}' | '\u{202f}' if before && group() => {}
            '.' if before
                && group()
                && chars.get(index + 4) != Some(&'.')
                && !chars
                    .get(index + 4)
                    .is_some_and(|next| next.is_ascii_digit()) => {}
            ',' if before && chars.get(index + 1).is_some_and(|c| c.is_ascii_digit()) => {
                out.push('.')
            }
            _ => out.push(c),
        }
    }
    out
}

fn numeric(alternative: &str) -> bool {
    !alternative.is_empty() && alternative.chars().all(|c| c.is_ascii_digit() || c == '.')
}

pub fn hits(text: &str, alternative: &str) -> bool {
    if !numeric(alternative) {
        return text.to_lowercase().contains(&alternative.to_lowercase());
    }
    text.match_indices(alternative).any(|(start, matched)| {
        let before = text[..start].chars().next_back();
        let mut after = text[start + matched.len()..].chars();
        let next = after.next();
        let following = after.next();
        let boundary_before = !before.is_some_and(|c| c.is_ascii_digit() || c == '.');
        let boundary_after = !next.is_some_and(|c| c.is_ascii_digit())
            && !(next == Some('.') && following.is_some_and(|c| c.is_ascii_digit()));
        boundary_before && boundary_after
    })
}

pub fn grade(probe: &Probe, status: &str, answer: &str) -> (Grade, usize) {
    if status != "answered" {
        let silent = status == "insufficient_evidence";
        return (
            if silent {
                Grade::Fail
            } else {
                Grade::Unavailable
            },
            0,
        );
    }
    let text = normalize(answer);
    let hit = probe
        .expect
        .iter()
        .filter(|group| group.iter().any(|alternative| hits(&text, alternative)))
        .count();
    let grade = match hit {
        0 => Grade::Fail,
        n if n == probe.expect.len() => Grade::Pass,
        _ => Grade::Partial,
    };
    (grade, hit)
}

fn query(request_id: &str, text: &str, context: bool) -> Value {
    let mut body = json!({
        "request_id": request_id,
        "conversation_id": request_id,
        "text": text,
        "requested_scopes": ["bot.public"],
        "profile": "explain",
    });
    if context {
        body["answer_context"] = json!({
            "platform": "discord",
            "is_thread": false,
            "is_direct_message": false,
            "input_kind": "mention",
        });
    }
    body
}

fn payload(request_id: &str, text: &str, history: &[String]) -> Value {
    if history.is_empty() {
        return query(request_id, text, true);
    }
    let recent = &history[history.len().saturating_sub(4)..];
    json!({"query": query(request_id, text, false), "user_questions": recent})
}

struct Reply {
    status: String,
    http: u16,
    answer: String,
    latency_ms: u64,
}

fn status_label(value: &str) -> String {
    let clean: String = value
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .take(64)
        .collect();
    match clean.chars().next() {
        Some(first) if first.is_ascii_lowercase() => clean,
        _ => "unknown".into(),
    }
}

async fn ask(client: &reqwest::Client, url: &str, token: &str, body: &Value) -> Reply {
    let started = std::time::Instant::now();
    let response = client
        .post(format!("{url}/v1/answer"))
        .bearer_auth(token)
        .header("x-discord-user-id", PROBE_USER_ID)
        .header("x-discord-read-access", "disabled")
        .json(body)
        .send()
        .await;
    let (status, http, answer) = match response {
        Err(_) => ("client_error".to_owned(), 0, String::new()),
        Ok(response) => {
            let http = response.status().as_u16();
            match (
                response.status().is_success(),
                response.json::<Value>().await,
            ) {
                (true, Ok(data)) => (
                    status_label(data["status"].as_str().unwrap_or("missing")),
                    http,
                    data["text"].as_str().unwrap_or_default().to_owned(),
                ),
                (true, Err(_)) => ("invalid_response".to_owned(), http, String::new()),
                (false, _) => (format!("http_{http}"), http, String::new()),
            }
        }
    };
    Reply {
        status,
        http,
        answer: answer.chars().take(ANSWER_CHARS).collect(),
        latency_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    }
}

async fn run_probe(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    run_id: i64,
    probe: &Probe,
) -> Outcome {
    let mut history: Vec<String> = Vec::new();
    let mut last = None;
    for (index, text) in probe.turns.iter().enumerate() {
        let request_id = format!("{REQUEST_PREFIX}{run_id}-{}-{}", probe.id, index + 1);
        let reply = ask(client, url, token, &payload(&request_id, text, &history)).await;
        history.push(text.clone());
        last = Some(reply);
        tokio::time::sleep(PAUSE).await;
    }
    let reply = last.unwrap_or(Reply {
        status: "client_error".into(),
        http: 0,
        answer: String::new(),
        latency_ms: 0,
    });
    let (grade, hits) = grade(probe, &reply.status, &reply.answer);
    Outcome {
        probe: probe.clone(),
        status: reply.status,
        http: reply.http,
        answer: reply.answer,
        latency_ms: reply.latency_ms,
        grade,
        hits,
    }
}

pub struct Baseline {
    pub run_id: i64,
    pub unavailable: i64,
    pub grades: HashMap<String, Grade>,
}

pub fn regressed(baseline: Option<Grade>, current: Grade) -> bool {
    match (baseline.and_then(Grade::rank), current.rank()) {
        (Some(before), Some(now)) => now < before,
        _ => false,
    }
}

pub fn unavailable_spike(probes: usize, unavailable: i64, baseline: Option<i64>) -> bool {
    let floor = i64::try_from(probes / 10).unwrap_or(i64::MAX).max(3);
    match baseline {
        Some(before) => unavailable > floor.max(before.saturating_mul(2)),
        None => unavailable > floor,
    }
}

async fn baseline(pool: &PgPool, set: &str) -> anyhow::Result<Option<Baseline>> {
    let run: Option<(i64, Option<i32>)> = sqlx::query_as(
        "SELECT id,unavailable FROM ops.answer_probe_run
         WHERE status='ok' AND probe_set_sha256=$1 ORDER BY id DESC LIMIT 1",
    )
    .bind(set)
    .fetch_optional(pool)
    .await?;
    let Some((run_id, unavailable)) = run else {
        return Ok(None);
    };
    let rows: Vec<(String, String)> =
        sqlx::query_as("SELECT probe_id,grade FROM ops.answer_probe_result WHERE run_id=$1")
            .bind(run_id)
            .fetch_all(pool)
            .await?;
    Ok(Some(Baseline {
        run_id,
        unavailable: i64::from(unavailable.unwrap_or(0)),
        grades: rows
            .into_iter()
            .filter_map(|(id, grade)| Grade::parse(&grade).map(|grade| (id, grade)))
            .collect(),
    }))
}

fn release() -> Option<String> {
    let path = std::fs::canonicalize("/opt/deadlock-brain/current").ok()?;
    let name = path.file_name()?.to_str()?.to_owned();
    let valid = (7..=64).contains(&name.len())
        && name
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase());
    valid.then_some(name)
}

async fn store(
    pool: &PgPool,
    run_id: i64,
    outcome: &Outcome,
    baseline: Option<Grade>,
) -> anyhow::Result<bool> {
    let regressed = regressed(baseline, outcome.grade);
    sqlx::query(
        "INSERT INTO ops.answer_probe_result
         (run_id,probe_id,kind,topic,final_status,http,grade,hit_groups,total_groups,latency_ms,answer_text,baseline_grade,regressed)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
    )
    .bind(run_id)
    .bind(&outcome.probe.id)
    .bind(&outcome.probe.kind)
    .bind(outcome.probe.topic.chars().take(300).collect::<String>())
    .bind(&outcome.status)
    .bind(i32::from(outcome.http))
    .bind(outcome.grade.as_str())
    .bind(i32::try_from(outcome.hits).unwrap_or(i32::MAX))
    .bind(i32::try_from(outcome.probe.expect.len()).unwrap_or(i32::MAX))
    .bind(i32::try_from(outcome.latency_ms).unwrap_or(i32::MAX))
    .bind((!outcome.answer.is_empty()).then_some(&outcome.answer))
    .bind(baseline.map(Grade::as_str))
    .bind(regressed)
    .execute(pool)
    .await?;
    Ok(regressed)
}

pub async fn run(pool: &PgPool, url: &str, token: &str) -> anyhow::Result<i64> {
    let probes = probes()?;
    let set = probe_set_sha256();
    let baseline = baseline(pool, &set).await?;
    let run_id: i64 = sqlx::query_scalar(
        "INSERT INTO ops.answer_probe_run (release,probe_set_sha256,probes,baseline_run,status)
         VALUES ($1,$2,$3,$4,'running') RETURNING id",
    )
    .bind(release())
    .bind(&set)
    .bind(i32::try_from(probes.len()).unwrap_or(i32::MAX))
    .bind(baseline.as_ref().map(|baseline| baseline.run_id))
    .fetch_one(pool)
    .await?;
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("HTTP-Client konnte nicht gestartet werden")?;
    let mut counts: HashMap<&'static str, i64> = HashMap::new();
    let mut regressions = Vec::new();
    for probe in &probes {
        let outcome = run_probe(&client, url, token, run_id, probe).await;
        let before = baseline
            .as_ref()
            .and_then(|baseline| baseline.grades.get(&probe.id).copied());
        if store(pool, run_id, &outcome, before).await? {
            regressions.push(probe.id.clone());
        }
        *counts.entry(outcome.grade.as_str()).or_default() += 1;
        println!(
            "probe={} status={} grade={} latency_ms={}",
            probe.id,
            outcome.status,
            outcome.grade.as_str(),
            outcome.latency_ms
        );
    }
    let count = |grade: Grade| counts.get(grade.as_str()).copied().unwrap_or(0);
    let unavailable = count(Grade::Unavailable);
    let spike = unavailable_spike(
        probes.len(),
        unavailable,
        baseline.as_ref().map(|baseline| baseline.unavailable),
    );
    let status = match (regressions.is_empty(), spike) {
        (_, true) => "degraded",
        (false, false) => "regressed",
        (true, false) => "ok",
    };
    let to_i32 = |value: i64| i32::try_from(value).unwrap_or(i32::MAX);
    sqlx::query(
        "UPDATE ops.answer_probe_run SET finished_at=clock_timestamp(),passed=$2,partial=$3,failed=$4,
         unavailable=$5,regressions=$6,unavailable_spike=$7,status=$8 WHERE id=$1",
    )
    .bind(run_id)
    .bind(to_i32(count(Grade::Pass)))
    .bind(to_i32(count(Grade::Partial)))
    .bind(to_i32(count(Grade::Fail)))
    .bind(to_i32(unavailable))
    .bind(i32::try_from(regressions.len()).unwrap_or(i32::MAX))
    .bind(spike)
    .bind(status)
    .execute(pool)
    .await?;
    println!(
        "probe_run={run_id} status={status} pass={} partial={} fail={} unavailable={unavailable} regressions={} baseline={}",
        count(Grade::Pass),
        count(Grade::Partial),
        count(Grade::Fail),
        if regressions.is_empty() { "-".to_owned() } else { regressions.join(",") },
        baseline.map_or_else(|| "-".to_owned(), |baseline| baseline.run_id.to_string()),
    );
    Ok(run_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn probe(expect: &[&[&str]]) -> Probe {
        Probe {
            id: "F06".into(),
            kind: "followup".into(),
            topic: "t".into(),
            turns: vec!["q".into()],
            expect: expect
                .iter()
                .map(|group| group.iter().map(|alt| (*alt).to_owned()).collect())
                .collect(),
        }
    }

    #[test]
    fn grading_matches_the_python_probe_harness() {
        assert_eq!(
            normalize("12.000 Seelen, 0,5 und 6 000"),
            "12000 Seelen, 0.5 und 6000"
        );
        assert_eq!(normalize("21.11.2025"), "21.11.2025");
        assert!(hits("liegt bei 40 %", "40"));
        assert!(!hits("liegt bei 140 %", "40"));
        assert!(!hits("liegt bei 40.5", "40"));
        assert!(hits("am 21.11.2025.", "21.11.2025"));
        let f06 = probe(&[&["40"], &["2025-11-21", "21.11.2025"], &["50%", "50 %"]]);
        assert_eq!(
            grade(&f06, "answered", "Seit 21.11.2025 40, jetzt 50 %").0,
            Grade::Pass
        );
        assert_eq!(grade(&f06, "answered", "Es sind 40").0, Grade::Partial);
        assert_eq!(grade(&f06, "answered", "keine Ahnung").0, Grade::Fail);
        assert_eq!(grade(&f06, "insufficient_evidence", "").0, Grade::Fail);
        assert_eq!(grade(&f06, "unavailable", "").0, Grade::Unavailable);
    }

    #[test]
    fn regressions_compare_against_the_last_good_grade() {
        assert!(regressed(Some(Grade::Pass), Grade::Partial));
        assert!(regressed(Some(Grade::Partial), Grade::Fail));
        assert!(!regressed(Some(Grade::Fail), Grade::Pass));
        assert!(!regressed(Some(Grade::Pass), Grade::Unavailable));
        assert!(!regressed(None, Grade::Fail));
        assert!(unavailable_spike(59, 6, None));
        assert!(!unavailable_spike(59, 5, None));
        assert!(!unavailable_spike(59, 4, Some(2)));
        assert!(unavailable_spike(59, 9, Some(4)));
    }

    #[test]
    fn shipped_probe_set_is_valid() {
        assert_eq!(probes().map(|probes| probes.len()).ok(), Some(59));
        assert_eq!(probe_set_sha256().len(), 64);
    }
}
