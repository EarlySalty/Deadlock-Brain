//! Explicit operator entrypoint; no production config or DSN is ever loaded.
use dbrain_s12_wiki_probe::{
    analyze,
    capture::CaptureOptions,
    knowledge::{self, MappingProfile},
    model::MAX_INPUT_BYTES,
    parse_json, sha256,
};
use dbrain_sources::{
    core::http::HttpClient,
    wiki_capture_io::{capture_with_http, WikiCaptureAccess},
    wiki_runtime::{plan_release, ReleaseRequest, ScratchWikiStore},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

const USAGE: &str = "brain-wiki-pilot (C5; no production database)\n\n  capture CONFIG.json OUTPUT_DIR --allow-network\n  plan CAPTURE.json MAPPING.json RELEASE_REQUEST.json OUTPUT_DIR\n  stage CAPTURE.json MAPPING.json RELEASE_REQUEST.json SCRATCH_SOCKET OUTPUT_DIR\n  delta BEFORE.json BEFORE_MAPPING.json AFTER.json AFTER_MAPPING.json OUTPUT_DIR\n\nCapture has no implicit scopes. Every HTTP call shares the configured budgets.\nStage requires the dedicated C5 scratch role/database, Unix socket and data-directory marker.\nFacts require an operator review of exact source revision/hash and dependency pins.\n";
fn input(path: &str) -> Result<Vec<u8>, String> {
    let meta = fs::symlink_metadata(path).map_err(|_| "cannot inspect input")?;
    if !meta.is_file() || meta.len() > MAX_INPUT_BYTES as u64 {
        return Err("input must be a bounded regular file, not a symlink/device".into());
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|_| "cannot open input")?
        .take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "cannot read input")?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input exceeded limit".into());
    }
    Ok(bytes)
}
fn json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    serde_json::from_value(parse_json(&input(path)?)?)
        .map_err(|_| "invalid JSON input schema".into())
}
fn directory(path: &str) -> Result<(), String> {
    use std::os::unix::fs::DirBuilderExt;
    if !Path::new(path).exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)
            .map_err(|_| "cannot create output directory")?;
    }
    let meta = fs::symlink_metadata(path).map_err(|_| "cannot inspect output directory")?;
    if !meta.is_dir() {
        return Err("output must be a real directory".into());
    }
    Ok(())
}
fn output(dir: &str, name: &str, value: &impl Serialize) -> Result<(), String> {
    use std::os::unix::fs::OpenOptionsExt;
    directory(dir)?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| "output encoding failed")?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("output exceeds limit".into());
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(Path::new(dir).join(name))
        .map_err(|_| "output already exists or cannot be created; nothing overwritten")?;
    f.write_all(&bytes)
        .and_then(|_| f.write_all(b"\n"))
        .and_then(|_| f.sync_all())
        .map_err(|_| "cannot persist output".into())
}
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--help"] || args == ["-h"] {
        print!("{USAGE}");
        return Ok(());
    }
    match args[0].as_str() {
        "capture" if args.len() == 4 && args[3] == "--allow-network" => {
            let mut options: CaptureOptions = json(&args[1])?;
            if options.retrieved_at == 0 {
                options.retrieved_at =
                    dbrain_sources::core::now_epoch_seconds().map_err(|_| "clock unavailable")?;
            }
            options.validate()?;
            if !options.policy.raw_retention_allowed
                || options
                    .policy
                    .source_license
                    .as_deref()
                    .is_none_or(|s| s.trim().is_empty())
            {
                return Err("persisting capture requires explicit raw retention approval and recorded source license".into());
            }
            if Path::new(&args[2]).exists() {
                return Err("capture output directory must be new".into());
            }
            directory(&args[2])?;
            let cache = Path::new(&args[2])
                .parent()
                .unwrap_or(Path::new("."))
                .join(".wiki-c5-http");
            let http = HttpClient::new("Deadlock-Brain-C5-bounded-pilot/1.0", cache)
                .map_err(|_| "cannot initialize shared HTTP transport")?;
            let capture = capture_with_http(
                &http,
                &WikiCaptureAccess {
                    enabled: true,
                    min_delay_seconds: 5.0,
                    max_response_bytes: options.max_total_bytes.min(4 * 1024 * 1024),
                },
                &options,
            )
            .map_err(|e| e.to_string())?;
            let bytes = serde_json::to_vec(&capture).map_err(|_| "capture encoding failed")?;
            let report = analyze(&bytes)?;
            let mut entities = BTreeMap::new();
            for hero in &options.scope.heroes {
                let binding = capture
                    .hero_bindings
                    .iter()
                    .find(|b| {
                        b.locale == hero.locale
                            && report.pages.iter().any(|p| {
                                p.page_id == b.hero_page_id
                                    && p.title.replace('_', " ") == hero.title.replace('_', " ")
                            })
                    })
                    .ok_or("hero normalization requires explicit mapping review")?;
                entities.insert(binding.hero_page_id, hero.entity_id.clone());
            }
            let mapping = MappingProfile {
                version: "c5-operator-mapping-v1".into(),
                review_ref: "unreviewed-empty-mapping; edit before adding fields".into(),
                source_key: options.source_key,
                entities,
                fields: Vec::new(),
            };
            let ir = knowledge::extract(&bytes, &mapping)?;
            let request = ReleaseRequest {
                release_id: format!("wiki-c5-{}", &sha256(&bytes)[..20]),
                knowledge_version: "wiki-c5-review-only".into(),
                validity: brain_contracts::domain::Validity {
                    patch: "pilot-unverified".into(),
                    mode: "pilot-review".into(),
                },
                created_at_epoch: capture.retrieved_at,
                review: None,
            };
            output(&args[2], "capture.json", &capture)?;
            output(&args[2], "analysis.json", &report)?;
            output(&args[2], "mapping.json", &mapping)?;
            output(&args[2], "ir.json", ir.contract())?;
            output(&args[2], "release-request.json", &request)?;
            println!("captured_pages={} raw_revisions={} source_fields={} provider_calls=0; review mapping/IR before facts",capture.pages.len(),ir.sources().len(),ir.fields().len());
        }
        "plan" if args.len() == 5 => {
            let ir = knowledge::extract(&input(&args[1])?, &json(&args[2])?)?;
            let request: ReleaseRequest = json(&args[3])?;
            let plan = plan_release(&ir, &request).map_err(|e| e.to_string())?;
            output(&args[4], "ir.json", ir.contract())?;
            output(&args[4], "plan.json", &plan)?;
            println!(
                "ir_records={} fact_records={} published=false",
                plan.ir_records.len(),
                plan.fact_records.len()
            );
        }
        "stage" if args.len() == 6 => {
            let ir = knowledge::extract(&input(&args[1])?, &json(&args[2])?)?;
            let request: ReleaseRequest = json(&args[3])?;
            // Validate before any DB connection. The scratch handle independently
            // checks the actual server before it permits migrations/staging.
            plan_release(&ir, &request).map_err(|e| e.to_string())?;
            directory(&args[5])?;
            if Path::new(&args[5]).join("receipt.json").exists() {
                return Err("receipt already exists; use a new output directory".into());
            }
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| "cannot initialize runtime")?;
            let receipt = runtime
                .block_on(async {
                    let store = ScratchWikiStore::connect(
                        Path::new(&args[4]),
                        &Path::new(&args[5]).join("raw"),
                    )
                    .await?;
                    store.migrate().await?;
                    store.stage(&ir, &request).await
                })
                .map_err(|e| e.to_string())?;
            output(&args[5], "receipt.json", &receipt)?;
            output(&args[5], "ir.json", ir.contract())?;
            println!(
                "scratch_release_written=true fact_records={} provider_calls=0",
                receipt["fact_records"]
            );
        }
        "delta" if args.len() == 6 => {
            let before = knowledge::extract(&input(&args[1])?, &json(&args[2])?)?;
            let after = knowledge::extract(&input(&args[3])?, &json(&args[4])?)?;
            output(
                &args[5],
                "delta.json",
                &knowledge::compare_ir(&before, &after)?,
            )?;
        }
        _ => return Err("invalid arguments; use --help (network is never implicit)".into()),
    }
    Ok(())
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("C5 wiki pilot: {error}");
            ExitCode::from(2)
        }
    }
}
