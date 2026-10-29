use brain_job::{
    answer_audit,
    config::Config,
    freshness, ingest_then, probe_guard, questions,
    registry::EXTRACTORS,
    sources::{self, SOURCES},
    GameFactsStore,
};
use brain_storage::PgStore;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::ExitCode,
};

const USAGE: &str =
    "Aufruf: brain-job --config <brain-job.json> ingest <source> [--input <git-repository>] | brain-job --config <brain-job.json> audit-receipts | brain-job --config <brain-job.json> freshness [--since <date>] | brain-job --config <brain-job.json> freshness-history <git-repository> | brain-job --config <brain-job.json> questions [--since <7d|24h|date>] [--status <not_answered|status>] [--limit <n>] | brain-job --config <brain-job.json> answer-probe [--url <http://127.0.0.1:8788>] | brain-job --config <brain-job.json> answer-audit [--since <24h|date>] [--trace <id>] [--limit <n>] [--record]";
const FRESHNESS_SINCE: &str = "2026-08-01";
const QUESTIONS_SINCE: &str = "7d";
const QUESTIONS_LIMIT: i64 = 50;
const AUDIT_SINCE: &str = "24h";
const AUDIT_LIMIT: i64 = 5000;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            match error.downcast_ref::<Usage>() {
                Some(usage) => eprintln!("brain-job: {usage}. {USAGE}"),
                None => eprintln!(
                    "brain-job: Verarbeitung fehlgeschlagen: {}",
                    redact(&format!("{error:#}"))
                ),
            }
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug)]
struct Usage(&'static str);

impl std::fmt::Display for Usage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for Usage {}

fn usage(reason: &'static str) -> anyhow::Error {
    Usage(reason).into()
}

fn redact(message: &str) -> String {
    message
        .split_inclusive(char::is_whitespace)
        .map(|word| {
            let lower = word.to_ascii_lowercase();
            let credential = word.contains("://") && word.contains('@');
            let secret = ["password=", "passwd=", "token=", "secret=", "dsn="]
                .iter()
                .any(|key| lower.contains(key));
            match (credential || secret, word.ends_with(char::is_whitespace)) {
                (false, _) => word.to_owned(),
                (true, true) => "[redacted] ".to_owned(),
                (true, false) => "[redacted]".to_owned(),
            }
        })
        .collect()
}

async fn run() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("{USAGE}");
        return Ok(());
    }
    let audit_config = match args.as_slice() {
        [flag, config, command] if flag == "--config" && command == "audit-receipts" => {
            Some(config)
        }
        [command, flag, config] if command == "audit-receipts" && flag == "--config" => {
            Some(config)
        }
        _ => None,
    };
    if let Some(config) = audit_config {
        return audit(Path::new(config)).await;
    }
    if let Some(result) = freshness_command(&args).await {
        return result;
    }
    if let [flag, config, command, rest @ ..] = args.as_slice() {
        if flag == "--config" && command == "questions" {
            return questions_command(Path::new(config), rest).await;
        }
        if flag == "--config" && command == "answer-probe" {
            return probe_command(Path::new(config), rest).await;
        }
        if flag == "--config" && command == "answer-audit" {
            return answer_audit_command(Path::new(config), rest).await;
        }
    }
    let (config_path, source, input) = match args.as_slice() {
        [flag, config, command, source] if flag == "--config" && command == "ingest" => {
            (config, source, None)
        }
        [flag, config, command, source, input_flag, input]
            if flag == "--config" && command == "ingest" && input_flag == "--input" =>
        {
            (config, source, Some(input))
        }
        [command, source, flag, config, input_flag, input]
            if command == "ingest" && flag == "--config" && input_flag == "--input" =>
        {
            (config, source, Some(input))
        }
        _ => return Err(usage("Ungültiger Aufruf")),
    };
    let source = source.to_str().ok_or_else(|| usage("Ungültige Quelle"))?;
    let loader = SOURCES
        .iter()
        .find(|(name, _)| *name == source)
        .map(|(_, loader)| *loader)
        .ok_or_else(|| usage("Unbekannte Quelle"))?;
    let config = Config::load(Path::new(config_path))?;
    let (objects, repository) = match input {
        Some(repository) => (
            loader(Path::new(repository), false),
            PathBuf::from(repository),
        ),
        None => {
            let cache = config
                .source_cache
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("Quellen-Cache fehlt"))?;
            (sources::fetch(source, cache), cache.join(source))
        }
    };
    let objects = prefetched(source, objects);
    let pool = config.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        let (objects, revision, observed) = tracked(source, objects);
        let recorded = std::sync::atomic::AtomicU64::new(0);
        let (watched, writer, counter) = (&observed, &pool, &recorded);
        let report = ingest_then(
            &GameFactsStore::new(pool.clone()),
            source,
            objects,
            EXTRACTORS,
            |run_id| async move {
                if let Some(observed) = watched {
                    let (paths, digests) = observed.take();
                    let changed =
                        freshness::record_patch_observations(writer, run_id, &paths, &digests)
                            .await?;
                    counter.store(changed, std::sync::atomic::Ordering::Relaxed);
                }
                Ok(())
            },
        )
        .await?;
        println!(
            "run={} accepted={} rejected={} ignored={}",
            report.run_id, report.counts.accepted, report.counts.rejected, report.counts.ignored
        );
        if let Some(revision) = revision {
            freshness::record_ingest(&pool, report.run_id, &repository, &revision).await?;
            println!("revision={revision}");
        }
        if observed.is_some() {
            println!(
                "patch_observations={}",
                recorded.load(std::sync::atomic::Ordering::Relaxed)
            );
            let changed = brain_job::patch_ledger::refresh(&pool, report.run_id).await?;
            println!("patch_ledger_changed={changed}");
        }
        if source == brain_job::item_lineage::SOURCE {
            match brain_job::item_lineage::log_alias_conflicts(&pool).await {
                Ok(conflicts) => println!("alias_conflicts={conflicts}"),
                Err(error) => eprintln!(
                    "{}",
                    serde_json::json!({"event":"item_lineage_alias_conflicts_unavailable","error":error.to_string()})
                ),
            }
        }
        anyhow::Ok(())
    }
    .await;
    pool.close().await;
    result
}

async fn audit(path: &Path) -> anyhow::Result<()> {
    let pool = Config::load(path)?.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        let purged = questions::purge(&pool).await;
        if let Ok(count) = &purged {
            println!("answer_trace_purged={count}");
        }
        let report = brain_job::receipt_audit::run(&pool).await?;
        println!(
            "run={} sampled={} verified_receipts={} rejected_receipts={} rejected_objects={}",
            report.run_id,
            report.sampled,
            report.verified_receipts,
            report.rejected_receipts,
            report.rejected_objects
        );
        purged?;
        anyhow::Ok(())
    }
    .await;
    pool.close().await;
    result
}

fn prefetched(source: &str, objects: brain_job::InputObjects) -> brain_job::InputObjects {
    if source != brain_job::meta_source::SOURCE {
        return objects;
    }
    Box::new(objects.collect::<Vec<_>>().into_iter())
}

fn tracked(
    source: &str,
    objects: brain_job::InputObjects,
) -> (
    brain_job::InputObjects,
    Option<String>,
    Option<freshness::Observed>,
) {
    if source == brain_job::patch_extract::SOURCE {
        let observed = freshness::Observed::default();
        return (observed.tap(objects), None, Some(observed));
    }
    if source != freshness::SOURCE {
        return (objects, None, None);
    }
    let objects: Vec<_> = objects.collect();
    let revision = objects
        .iter()
        .find_map(|object| object.as_ref().ok().map(|object| object.revision.clone()));
    (Box::new(objects.into_iter()), revision, None)
}

async fn freshness_command(args: &[OsString]) -> Option<anyhow::Result<()>> {
    let (config, command, rest) = match args {
        [flag, config, command, rest @ ..] if flag == "--config" => (config, command, rest),
        _ => return None,
    };
    let action = match (command.to_str(), rest) {
        (Some("freshness"), []) => Freshness::Report(FRESHNESS_SINCE.to_owned()),
        (Some("freshness"), [flag, since]) if flag == "--since" => {
            Freshness::Report(since.to_str()?.to_owned())
        }
        (Some("freshness-history"), [repository]) => Freshness::History(PathBuf::from(repository)),
        _ => return None,
    };
    Some(freshness(Path::new(config), action).await)
}

enum Freshness {
    Report(String),
    History(PathBuf),
}

async fn freshness(path: &Path, action: Freshness) -> anyhow::Result<()> {
    let pool = Config::load(path)?.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        match action {
            Freshness::Report(since) => freshness::report(&pool, &since).await,
            Freshness::History(repository) => {
                let (commits, runs) = freshness::backfill(&pool, &repository).await?;
                println!("commits={commits} runs={runs}");
                Ok(())
            }
        }
    }
    .await;
    pool.close().await;
    result
}

fn questions_filter(rest: &[OsString]) -> anyhow::Result<questions::Filter> {
    let mut since = QUESTIONS_SINCE.to_owned();
    let mut status = None;
    let mut limit = QUESTIONS_LIMIT;
    for pair in rest.chunks(2) {
        let [flag, value] = pair else {
            return Err(usage("Ungültiger Aufruf"));
        };
        let value = value.to_str().ok_or_else(|| usage("Ungültiger Wert"))?;
        match flag.to_str() {
            Some("--since") => since = value.to_owned(),
            Some("--status") => status = Some(value.to_owned()),
            Some("--limit") => {
                limit = value
                    .parse::<i64>()
                    .map_err(|_| usage("Ungültiges Limit"))?
                    .clamp(1, 1000)
            }
            _ => return Err(usage("Ungültiger Aufruf")),
        }
    }
    Ok(questions::Filter {
        since: questions::parse_since(&since, chrono::Utc::now())?,
        status,
        limit,
    })
}

async fn questions_command(path: &Path, rest: &[OsString]) -> anyhow::Result<()> {
    let filter = questions_filter(rest)?;
    let pool = Config::load(path)?.pool().await?;
    let result = questions::report(&pool, &filter).await;
    pool.close().await;
    result
}

async fn probe_command(path: &Path, rest: &[OsString]) -> anyhow::Result<()> {
    let url = match rest {
        [] => probe_guard::DEFAULT_URL.to_owned(),
        [flag, url] if flag == "--url" => url
            .to_str()
            .filter(|url| {
                url.starts_with("http://127.0.0.1:") || url.starts_with("http://localhost:")
            })
            .ok_or_else(|| usage("Nur lokale Brain-Adressen"))?
            .trim_end_matches('/')
            .to_owned(),
        _ => return Err(usage("Ungültiger Aufruf")),
    };
    let config = Config::load(path)?;
    let token = config.secret(probe_guard::TOKEN_SECRET).await?;
    let pool = config.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        probe_guard::run(&pool, &url, &token).await.map(|_| ())
    }
    .await;
    pool.close().await;
    result
}

fn audit_options(rest: &[OsString]) -> anyhow::Result<answer_audit::Options> {
    let mut since = AUDIT_SINCE.to_owned();
    let mut trace = None;
    let mut limit = AUDIT_LIMIT;
    let mut record = false;
    let mut args = rest.iter();
    while let Some(flag) = args.next() {
        if flag == "--record" {
            record = true;
            continue;
        }
        let value = args
            .next()
            .and_then(|value| value.to_str())
            .ok_or_else(|| usage("Ungültiger Wert"))?;
        match flag.to_str() {
            Some("--since") => since = value.to_owned(),
            Some("--trace") => {
                trace = Some(
                    value
                        .parse::<i64>()
                        .map_err(|_| usage("Ungültige Trace-ID"))?,
                )
            }
            Some("--limit") => {
                limit = value
                    .parse::<i64>()
                    .map_err(|_| usage("Ungültiges Limit"))?
                    .clamp(1, 50_000)
            }
            _ => return Err(usage("Ungültiger Aufruf")),
        }
    }
    if record && trace.is_some() {
        return Err(usage("--record gilt nur für Tagesläufe"));
    }
    Ok(answer_audit::Options {
        since: questions::parse_since(&since, chrono::Utc::now())?,
        trace,
        record,
        limit,
    })
}

async fn answer_audit_command(path: &Path, rest: &[OsString]) -> anyhow::Result<()> {
    let options = audit_options(rest)?;
    let pool = Config::load(path)?.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        answer_audit::run(&pool, &options).await
    }
    .await;
    pool.close().await;
    result
}
