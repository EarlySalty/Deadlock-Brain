#![forbid(unsafe_code)]
mod gold;
mod matching;
mod report;
mod score;

use anyhow::{bail, Context, Result};
use brain_client::{BrainClient, Query};
use clap::Parser;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const REPORT_SCHEMA: &str = "brain.e1-eval-report.v1";

#[derive(Parser)]
#[command(
    name = "brain-eval",
    about = "Scores a brain-serve endpoint against the E1 gold set"
)]
struct Args {
    #[arg(long)]
    gold: PathBuf,
    #[arg(long, default_value = "http://127.0.0.1:8788")]
    endpoint: String,
    #[arg(long, default_value = "BRAIN_EVAL_TOKEN")]
    token_env: String,
    #[arg(long = "scope", default_values_t = vec!["bot.public".to_owned()])]
    scopes: Vec<String>,
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,
    #[arg(long)]
    limit: Option<usize>,
    #[arg(long)]
    validate_only: bool,
}

#[derive(Serialize)]
struct Report<'a> {
    schema: &'static str,
    gold_schema: &'a str,
    gold_version: u32,
    gold_sha256: &'a str,
    endpoint: &'a str,
    scopes: &'a [String],
    started_unix: u64,
    duration_ms: u128,
    summary: report::Summary,
    cases: Vec<score::CaseResult>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let loaded = gold::load(&args.gold)?;
    if args.validate_only {
        println!(
            "gold ok: {} cases, sha256 {}",
            loaded.gold.cases.len(),
            loaded.sha256
        );
        return Ok(());
    }
    let Some(out) = args.out.as_ref() else {
        bail!("--out is required unless --validate-only is set");
    };
    let client = client(&args)?;
    let started_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let started = Instant::now();
    let cases = take_cases(&loaded.gold.cases, args.limit);
    let results = run_cases(&client, cases, &args.scopes, started_unix);
    let summary = report::summarize(&results);
    let text = report::render_text(&summary);
    let report = Report {
        schema: REPORT_SCHEMA,
        gold_schema: &loaded.gold.schema,
        gold_version: loaded.gold.version,
        gold_sha256: &loaded.sha256,
        endpoint: &args.endpoint,
        scopes: &args.scopes,
        started_unix,
        duration_ms: started.elapsed().as_millis(),
        summary,
        cases: results,
    };
    write_report(out, &report)?;
    println!("{text}");
    Ok(())
}

fn client(args: &Args) -> Result<BrainClient> {
    let token = std::env::var(&args.token_env)
        .with_context(|| format!("environment variable {} is not set", args.token_env))?;
    if args.timeout_secs == 0 {
        bail!("--timeout-secs must be positive");
    }
    BrainClient::new(
        &args.endpoint,
        &token,
        Duration::from_secs(args.timeout_secs),
    )
    .map_err(|e| anyhow::anyhow!("brain client setup failed: {e}"))
}

fn take_cases(cases: &[gold::GoldCase], limit: Option<usize>) -> &[gold::GoldCase] {
    &cases[..limit.unwrap_or(cases.len()).min(cases.len())]
}

fn run_cases(
    client: &BrainClient,
    cases: &[gold::GoldCase],
    scopes: &[String],
    run_id: u64,
) -> Vec<score::CaseResult> {
    cases
        .iter()
        .enumerate()
        .map(|(index, case)| {
            let result = run_case(client, case, scopes, run_id);
            eprintln!(
                "[{}/{}] {} {}",
                index + 1,
                cases.len(),
                case.id,
                outcome_label(&result)
            );
            result
        })
        .collect()
}

fn run_case(
    client: &BrainClient,
    case: &gold::GoldCase,
    scopes: &[String],
    run_id: u64,
) -> score::CaseResult {
    let query = build_query(case, scopes, run_id);
    let started = Instant::now();
    let answer = client.answer(&query);
    let latency_ms = started.elapsed().as_millis();
    match answer {
        Ok(response) => score::score_response(case, &response, latency_ms),
        Err(error) => score::score_error(case, error.to_string(), latency_ms),
    }
}

fn build_query(case: &gold::GoldCase, scopes: &[String], run_id: u64) -> Query {
    Query {
        request_id: format!("e1-eval-{run_id}-{}", case.id),
        conversation_id: format!("e1-eval-{run_id}-{}", case.id),
        text: case.question.clone(),
        answer_context: None,
        domain: None,
        requested_scopes: scopes.iter().cloned().collect::<BTreeSet<_>>(),
        profile: case.profile.clone(),
        patch: None,
        mode: None,
    }
}

fn outcome_label(result: &score::CaseResult) -> String {
    match (&result.status, &result.error) {
        (Some(status), _) => format!(
            "{status:?} ok={} facts={}/{} citations={}",
            result.outcome_ok, result.facts_matched, result.facts_total, result.citation_count
        ),
        (None, Some(error)) => format!("error {error}"),
        (None, None) => "no result".to_owned(),
    }
}

fn write_report(path: &PathBuf, report: &Report<'_>) -> Result<()> {
    let mut json = serde_json::to_string_pretty(report).context("serialize report")?;
    json.push('\n');
    std::fs::write(path, json).with_context(|| format!("write {}", path.display()))
}
