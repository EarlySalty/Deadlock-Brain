use brain_contracts::{
    source::{origin_from_record, SourceRevision},
    AnswerProfile, AnswerStatus, CorpusSnapshot, Principal, PublicAnswerResponse, Query,
    SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use brain_serve::{Config, Secrets};
use brain_storage::LocalPgReader;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::Command,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, &'static str>;
const DOCS_COMMIT: &str = "4b072aee3127564def674f4b53d9be5d1d42cfdf";
const AUTHORIZATION: &str = "workspace-request:2026-10-02:reviewed-existing-public-doc-corrections";
const PAGES: [&str; 5] = [
    "public/discord-server/paten.html",
    "public/twitch-bot/chat-moderation.html",
    "public/discord-server/dm-concierge.html",
    "public/discord-server/voice-features.html",
    "public/twitch-bot/auto-raid.html",
];
const MAX_BODY: u64 = 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    class: String,
    scenario: String,
    expected: String,
    checks: Vec<String>,
    source_path: String,
    source_commit: String,
    source_sha256: String,
    source_heading: String,
    synthetic: bool,
}

#[derive(Serialize)]
struct Observation {
    http_status: Option<u16>,
    elapsed_ms: u128,
    error: Option<&'static str>,
    response: Option<PublicAnswerResponse>,
    usage: Option<Value>,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn require(ok: bool, code: &'static str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(code)
    }
}

fn headings(html: &str) -> Result<Vec<String>> {
    let mut found = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("<h") {
        rest = &rest[start + 2..];
        let Some(level) = rest.chars().next() else {
            break;
        };
        if level != '2' && level != '3' {
            continue;
        }
        let end_open = rest.find('>').ok_or("invalid_heading")?;
        let close = format!("</h{level}>");
        let end = rest.find(&close).ok_or("invalid_heading")?;
        require(end > end_open, "invalid_heading")?;
        let text = rest[end_open + 1..end].trim();
        require(
            !text.is_empty() && !text.contains(['<', '&']) && text.len() < 256,
            "unsupported_heading_markup",
        )?;
        found.push(text.to_owned());
        rest = &rest[end + close.len()..];
    }
    Ok(found)
}

fn corpus(repo: &Path) -> Result<Vec<Case>> {
    let mut cases = Vec::new();
    let mut topic_count = 0;
    for page in PAGES {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo)
            .arg("show")
            .arg(format!("{DOCS_COMMIT}:{page}"))
            .output()
            .map_err(|_| "docs_git_unavailable")?;
        require(output.status.success(), "approved_docs_revision_missing")?;
        let hash = digest(&output.stdout);
        let html = String::from_utf8(output.stdout).map_err(|_| "invalid_docs_utf8")?;
        for heading in headings(&html)? {
            topic_count += 1;
            let original = page == PAGES[0]
                || (page == PAGES[1] && heading == "Wie kann ich eine Maßnahme prüfen lassen?");
            let mut questions = vec![
                (format!("Was muss ich zu „{heading}“ wissen?"), true),
                (
                    format!("Wie funktioniert „{heading}“ laut der Hilfe?"),
                    true,
                ),
                (format!("Kannst du „{heading}“ kurz erklären?"), true),
                (
                    format!("Welche Hinweise gibt die Hilfe zu „{heading}“?"),
                    true,
                ),
            ];
            if original {
                questions.insert(0, (heading.clone(), false));
            }
            for (question, synthetic) in questions {
                cases.push(Case {
                    id: format!("Q4-{:03}", cases.len() + 1),
                    class: "public".into(),
                    scenario: question,
                    expected: "unlabelled".into(),
                    checks: vec!["source_scope".into(), "citation_support".into()],
                    source_path: page.into(),
                    source_commit: DOCS_COMMIT.into(),
                    source_sha256: hash.clone(),
                    source_heading: heading.clone(),
                    synthetic,
                });
            }
        }
    }
    require(
        topic_count == 24 && cases.len() == 103,
        "approved_corpus_shape_changed",
    )?;
    require(
        cases
            .iter()
            .map(|c| &c.scenario)
            .collect::<BTreeSet<_>>()
            .len()
            == cases.len(),
        "duplicate_question",
    )?;
    Ok(cases)
}

fn new_output(path: &Path) -> Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|_| "output_not_new_or_unwritable")
}

fn write_json(file: &mut std::fs::File, value: &impl Serialize) -> Result<()> {
    serde_json::to_writer(&mut *file, value).map_err(|_| "output_write_failed")?;
    file.write_all(b"\n").map_err(|_| "output_write_failed")?;
    file.flush().map_err(|_| "output_write_failed")
}

fn equal_configs(a: &Config, b: &Config, secret_name: &str) -> Result<()> {
    let grants = |config: &Config| -> Result<_> {
        let grant = config
            .credentials
            .iter()
            .find(|c| c.token_env == secret_name)
            .ok_or("client_secret_not_in_config")?;
        require(
            grant.scopes == BTreeSet::from(["bot.public".into()])
                && grant.provider_egress == BTreeSet::from(["public".into()]),
            "client_not_public_only",
        )?;
        let release = grant.release.as_ref().unwrap_or(&config.release);
        require(
            release.id == config.release.id
                && release.knowledge_version == config.release.knowledge_version,
            "client_release_override",
        )?;
        Ok((grant.actor_id.clone(), grant.channel.clone()))
    };
    require(grants(a)? == grants(b)?, "client_grants_differ")?;
    require(
        a.bind.ip().is_loopback()
            && b.bind.ip().is_loopback()
            && a.bind != b.bind
            && a.bind.port() != 0
            && b.bind.port() != 0,
        "endpoints_not_distinct_loopback",
    )?;
    require(
        a.release.id == b.release.id && a.release.knowledge_version == b.release.knowledge_version,
        "release_mismatch",
    )?;
    require(
        a.postgres.socket_dir == b.postgres.socket_dir
            && a.postgres.port == b.postgres.port
            && a.postgres.database == b.postgres.database
            && a.postgres.username == b.postgres.username
            && a.postgres.auth == b.postgres.auth
            && a.postgres.password_env == b.postgres.password_env,
        "database_mismatch",
    )?;
    require(
        a.provider.model == b.provider.model
            && a.provider.base_url == b.provider.base_url
            && a.provider.api_key_env == b.provider.api_key_env
            && a.provider.retry_attempts == b.provider.retry_attempts
            && a.provider.retry_backoff_ms == b.provider.retry_backoff_ms
            && a.provider.max_response_bytes == b.provider.max_response_bytes,
        "provider_mismatch",
    )?;
    require(
        a.provider
            .pricing
            .map(|p| (p.input_micros_per_token, p.output_micros_per_token))
            == b.provider
                .pricing
                .map(|p| (p.input_micros_per_token, p.output_micros_per_token)),
        "pricing_mismatch",
    )?;
    require(
        a.budgets.max_network_rounds == b.budgets.max_network_rounds
            && a.budgets.max_input_tokens == b.budgets.max_input_tokens
            && a.budgets.max_output_tokens == b.budgets.max_output_tokens
            && a.budgets.max_cost_micros == b.budgets.max_cost_micros,
        "budget_mismatch",
    )?;
    require(
        a.timeouts.provider_ms == b.timeouts.provider_ms
            && a.timeouts.request_ms == b.timeouts.request_ms
            && a.retrieval.limit == b.retrieval.limit
            && a.kernel.cache_entries == b.kernel.cache_entries
            && a.kernel.cache_ttl_ms == b.kernel.cache_ttl_ms,
        "timeouts_retrieval_or_cache_not_comparable",
    )?;
    Ok(())
}

fn attest(pid: &str, sha: &str, config: &Path) -> Result<Value> {
    require(
        pid.parse::<u32>().is_ok_and(|p| p > 1)
            && sha.len() == 40
            && sha.bytes().all(|b| b.is_ascii_hexdigit()),
        "invalid_process_identity",
    )?;
    let executable =
        fs::read_link(format!("/proc/{pid}/exe")).map_err(|_| "process_executable_unavailable")?;
    require(
        executable.file_name().is_some_and(|n| n == "brain-serve")
            && executable
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|n| n == sha),
        "process_commit_not_attested",
    )?;
    let cmdline =
        fs::read(format!("/proc/{pid}/cmdline")).map_err(|_| "process_arguments_unavailable")?;
    let args: Vec<_> = cmdline
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .collect();
    require(
        args.windows(2)
            .any(|pair| pair[0] == b"--config" && pair[1] == config.as_os_str().as_encoded_bytes()),
        "process_config_not_attested",
    )?;
    let mut image =
        fs::File::open(format!("/proc/{pid}/exe")).map_err(|_| "process_executable_unavailable")?;
    let mut hasher = Sha256::new();
    let mut bytes = [0_u8; 64 * 1024];
    loop {
        let n = image
            .read(&mut bytes)
            .map_err(|_| "process_executable_unavailable")?;
        if n == 0 {
            break;
        }
        hasher.update(&bytes[..n]);
    }
    Ok(
        json!({"pid":pid, "commit":sha, "binary_sha256":format!("{:x}",hasher.finalize()), "config_sha256":digest(&fs::read(config).map_err(|_| "config_unavailable")?)}),
    )
}

fn policy(record: &SourceRecordV2) -> Result<()> {
    let origin = origin_from_record(record).map_err(|_| "missing_or_invalid_origin")?;
    require(
        !record.tombstone
            && record.visibility == SourceVisibility::Public
            && record.allowed_scopes == BTreeSet::from(["bot.public".into()])
            && origin.policy.publication_allowed
            && origin.policy.provider_egress_allowed
            && matches!(&origin.policy.authorization_ref, brain_contracts::value::Observed::Known { value } if value == AUTHORIZATION),
        "source_egress_not_explicitly_approved",
    )
}

fn validate_snapshot(
    snapshot: &CorpusSnapshot,
    cases: &[Case],
    config: &Config,
    secret_name: &str,
) -> Result<Value> {
    let grant = config
        .credentials
        .iter()
        .find(|c| c.token_env == secret_name)
        .ok_or("client_secret_not_in_config")?;
    let principal = Principal {
        actor_id: grant.actor_id.clone(),
        channel: grant.channel.clone(),
        scopes: grant.scopes.clone(),
        provider_egress: grant.provider_egress.clone(),
    };
    require(
        snapshot.release.release_id == config.release.id
            && snapshot.release.knowledge_version == config.release.knowledge_version,
        "snapshot_release_mismatch",
    )?;
    let records = snapshot
        .authorized(&principal, true)
        .map_err(|_| "snapshot_authorization_failed")?;
    require(!records.is_empty(), "empty_public_provider_corpus")?;
    let public = snapshot
        .authorized_for_publication(&principal)
        .map_err(|_| "snapshot_publication_failed")?;
    let mut pins = Vec::new();
    for record in &records {
        policy(record)?;
        require(public.iter().any(|p| p == record), "source_not_publishable")?;
    }
    for page in PAGES {
        let matching: Vec<_> = records
            .iter()
            .filter(|record| {
                record.logical_id.ends_with(page)
                    || origin_from_record(record).is_ok_and(|o| o.locator.ends_with(page))
            })
            .collect();
        require(
            matching.len() == 1,
            "approved_faq_source_missing_or_ambiguous",
        )?;
        let record = matching[0];
        let head = snapshot
            .heads
            .iter()
            .find(|h| h.source_id == record.source_id && h.logical_id == record.logical_id)
            .ok_or("faq_head_missing")?;
        policy(head)?;
        require(
            record.revision == head.revision && record.content_hash == head.content_hash,
            "faq_pin_not_current",
        )?;
        let origin = origin_from_record(record).map_err(|_| "faq_origin_missing")?;
        require(
            matches!(&origin.source_revision, SourceRevision::Git { commit } if commit == DOCS_COMMIT),
            "faq_docs_commit_mismatch",
        )?;
        require(
            cases
                .iter()
                .filter(|c| c.source_path == page)
                .all(|c| c.source_sha256 == origin.raw_sha256),
            "faq_raw_artifact_mismatch",
        )?;
        require(
            cases
                .iter()
                .filter(|c| c.source_path == page)
                .all(|c| record.content.contains(&c.source_heading)),
            "faq_heading_not_in_release",
        )?;
        pins.push(json!({"page":page,"source_id":record.source_id,"logical_id":record.logical_id,"revision":record.revision,"content_hash":record.content_hash,"origin":origin}));
    }
    Ok(
        json!({"release":snapshot.release,"release_sha256":digest(&serde_json::to_vec(&snapshot.release).map_err(|_| "release_encoding_failed")?),"approved_faq_pins":pins,"provider_visible_records":records.len()}),
    )
}

fn reader(config: &Config, password: Option<String>) -> Result<LocalPgReader> {
    let pg = &config.postgres;
    let t = &config.timeouts;
    LocalPgReader::new(&pg.socket_dir, pg.port, &pg.username, &pg.database)
        .and_then(|r| {
            r.with_pool_options(
                password,
                Duration::from_millis(t.postgres_connect_ms),
                Duration::from_millis(t.postgres_statement_ms),
                Duration::from_millis(t.postgres_lock_ms),
                1,
                Duration::from_millis(t.postgres_pool_wait_ms),
            )
        })
        .map_err(|_| "postgres_reader_unavailable")
}

fn client(config: &Config) -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_millis(config.timeouts.request_ms))
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .no_gzip()
        .build()
        .map_err(|_| "http_client_unavailable")
}

fn body(response: reqwest::blocking::Response) -> Result<Vec<u8>> {
    require(
        !response.content_length().is_some_and(|n| n > MAX_BODY),
        "response_too_large",
    )?;
    require(
        response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                v.split(';')
                    .next()
                    .is_some_and(|v| v.trim().eq_ignore_ascii_case("application/json"))
            }),
        "response_not_json",
    )?;
    let mut bytes = Vec::new();
    response
        .take(MAX_BODY + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "response_read_failed")?;
    require(bytes.len() as u64 <= MAX_BODY, "response_too_large")?;
    Ok(bytes)
}

fn ready(client: &Client, config: &Config) -> Result<()> {
    let response = client
        .get(format!("http://{}/readyz", config.bind))
        .send()
        .map_err(|_| "ready_transport_failed")?;
    require(response.status().is_success(), "service_not_ready")?;
    let value: Value =
        serde_json::from_slice(&body(response)?).map_err(|_| "ready_invalid_json")?;
    require(
        value["status"] == "ready"
            && value["release_id"] == config.release.id
            && value["knowledge_version"] == config.release.knowledge_version,
        "live_release_mismatch",
    )
}

fn answer(client: &Client, config: &Config, token: &str, query: &Query) -> Observation {
    let start = Instant::now();
    let mut status = None;
    let result = (|| {
        let response = client
            .post(format!("http://{}/v1/answer", config.bind))
            .bearer_auth(token)
            .json(query)
            .send()
            .map_err(|_| "answer_transport_failed")?;
        status = Some(response.status().as_u16());
        require(response.status().is_success(), "answer_http_error")?;
        let answer: PublicAnswerResponse =
            serde_json::from_slice(&body(response)?).map_err(|_| "answer_contract_invalid")?;
        answer
            .validate(&query.request_id)
            .map_err(|_| "answer_contract_invalid")?;
        require(
            answer.knowledge_release == config.release.id,
            "answer_release_mismatch",
        )?;
        Ok(answer)
    })();
    match result {
        Ok(response) => Observation {
            http_status: status,
            elapsed_ms: start.elapsed().as_millis(),
            error: None,
            response: Some(response),
            usage: None,
        },
        Err(error) => Observation {
            http_status: status,
            elapsed_ms: start.elapsed().as_millis(),
            error: Some(error),
            response: None,
            usage: None,
        },
    }
}

pub fn execute(args: Vec<String>) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("prepare") if args.len() == 3 => {
            let cases = corpus(Path::new(&args[1]))?;
            write_json(&mut new_output(Path::new(&args[2]))?, &json!({"schema_version":1,"cases":cases,"gold_labels":false}))
        }
        Some("run") if args.len() == 12 => run(&args),
        _ => Err("usage_prepare_DOCS_REPO_OUT_or_run_DOCS_REPO_BASE_CONFIG_CAND_CONFIG_INFISICAL_CLIENT_SECRET_BASE_PID_BASE_SHA_CAND_PID_CAND_SHA_OUT"),
    }
}

fn run(args: &[String]) -> Result<()> {
    require(
        std::env::var_os("PGOPTIONS").is_none(),
        "ambient_postgres_options",
    )?;
    let cases = corpus(Path::new(&args[1]))?;
    let base = Config::load(Path::new(&args[2])).map_err(|_| "baseline_config_invalid")?;
    let candidate = Config::load(Path::new(&args[3])).map_err(|_| "candidate_config_invalid")?;
    equal_configs(&base, &candidate, &args[5])?;
    let base_identity = attest(&args[6], &args[7], Path::new(&args[2]))?;
    let candidate_identity = attest(&args[8], &args[9], Path::new(&args[3]))?;
    require(
        args[6] != args[8] && args[7] != args[9],
        "not_distinct_implementations",
    )?;
    require(
        args[10] == "--execute-approved-public",
        "explicit_execution_flag_required",
    )?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| "secret_runtime_failed")?;
    let values: BTreeMap<_, _> = runtime
        .block_on(async {
            tokio::time::timeout(
                Duration::from_millis(base.timeouts.startup_ms),
                dl_token_secrets::values(Path::new(&args[4])),
            )
            .await
        })
        .map_err(|_| "secret_startup_timeout")?
        .map_err(|_| "secret_source_unavailable")?
        .into_iter()
        .collect();
    let lookup = |name: &str| values.get(name).map(|v| v.as_str().to_owned());
    let _ = Secrets::load(&base, lookup).map_err(|_| "baseline_secrets_invalid")?;
    let _ = Secrets::load(&candidate, lookup).map_err(|_| "candidate_secrets_invalid")?;
    let token = lookup(&args[5]).ok_or("client_secret_missing")?;
    let password = base.postgres.password_env.as_deref().and_then(lookup);
    let reader = reader(&base, password)?;
    reader
        .check_core_schema()
        .map_err(|_| "core_schema_invalid")?;
    reader
        .check_permissions()
        .map_err(|_| "reader_permissions_invalid")?;
    let snapshot = reader
        .read_snapshot(&base.release.id)
        .map_err(|_| "snapshot_unavailable")?;
    let initial = validate_snapshot(&snapshot, &cases, &base, &args[5])?;
    let baseline_http = client(&base)?;
    let candidate_http = client(&candidate)?;
    ready(&baseline_http, &base)?;
    ready(&candidate_http, &candidate)?;
    let mut file = new_output(Path::new(&args[11]))?;
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "clock_invalid")?
        .as_nanos();
    write_json(
        &mut file,
        &json!({"type":"metadata","schema_version":1,"run_id":run_id.to_string(),"corpus_sha256":digest(&serde_json::to_vec(&cases).map_err(|_| "corpus_encoding_failed")?),"cases":cases.len(),"real_faq_questions":7,"synthetic_questions":96,"baseline":base_identity,"candidate":candidate_identity,"snapshot":initial,"model":base.provider.model,"provider_endpoint":base.provider.base_url,"request_timeout_ms":base.timeouts.request_ms,"provider_timeout_ms":base.timeouts.provider_ms,"budget":brain_contracts::Budget::from(&base.budgets),"limits":["Öffentliche FAQ, kein aufgezeichneter Verkehr und keine unabhängigen Goldlabels.","Öffentliche API liefert Belegkennungen, aber keine Nutzungs- oder Kostenwerte. usage bleibt null.","Konfigurationsdatei und Prozesspfad sind geprüft. Modell und Budget der bereits laufenden Prozesse sind ohne separate Laufzeitnachweise nicht bewiesen.","Ein möglicher Cachetreffer ist im öffentlichen Vertrag nicht sichtbar. Antworten belegen deshalb nicht allein einen Anbieteraufruf.","Antwortvergleich ist deskriptiv und setzt PROVIDER_SHADOW_PASSED nicht automatisch."]}),
    )?;
    let mut counts = [0_usize; 2];
    let mut failures = [0_usize; 2];
    let mut same_status = 0;
    for (index, case) in cases.iter().enumerate() {
        let fresh = reader
            .read_snapshot(&base.release.id)
            .map_err(|_| "snapshot_unavailable")?;
        require(
            validate_snapshot(&fresh, &cases, &base, &args[5])? == initial,
            "source_policy_or_pins_changed",
        )?;
        require(
            attest(&args[6], &args[7], Path::new(&args[2]))? == base_identity
                && attest(&args[8], &args[9], Path::new(&args[3]))? == candidate_identity,
            "process_or_config_changed",
        )?;
        let query = |arm: &str| Query {
            request_id: format!("q4-{run_id}-{}-{arm}", case.id),
            conversation_id: format!("q4-{run_id}-{}-{arm}", case.id),
            text: case.scenario.clone(),
            requested_scopes: BTreeSet::from(["bot.public".into()]),
            profile: AnswerProfile::Explain,
            domain: None,
            patch: None,
            mode: None,
        };
        let baseline;
        let candidate_result;
        if index % 2 == 0 {
            baseline = answer(&baseline_http, &base, &token, &query("base"));
            candidate_result = answer(&candidate_http, &candidate, &token, &query("candidate"));
        } else {
            candidate_result = answer(&candidate_http, &candidate, &token, &query("candidate"));
            baseline = answer(&baseline_http, &base, &token, &query("base"));
        }
        for (arm, observation) in [&baseline, &candidate_result].into_iter().enumerate() {
            if observation
                .response
                .as_ref()
                .is_some_and(|r| r.status == AnswerStatus::Answered)
            {
                counts[arm] += 1;
            }
            if observation.error.is_some() {
                failures[arm] += 1;
            }
        }
        if let (Some(a), Some(b)) = (&baseline.response, &candidate_result.response) {
            if a.status == b.status {
                same_status += 1;
            }
        }
        write_json(
            &mut file,
            &json!({"type":"case","run_id":run_id.to_string(),"case":case,"baseline":baseline,"candidate":candidate_result}),
        )?;
    }
    write_json(
        &mut file,
        &json!({"type":"summary","complete":true,"cases":cases.len(),"baseline_answered":counts[0],"candidate_answered":counts[1],"baseline_transport_or_contract_errors":failures[0],"candidate_transport_or_contract_errors":failures[1],"same_status":same_status,"provider_shadow_passed":null,"judgment":"Unbewerteter Vergleich. Echte Anbieteraufrufe und Antwortqualität müssen separat belegt werden."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_supported_headings() {
        assert_eq!(
            headings("<h1>Ignore</h1><h2>A</h2><h3 id=\"x\">B</h3>").unwrap(),
            ["A", "B"]
        );
        assert!(headings("<h2>A &amp; B</h2>").is_err());
        assert!(headings("<h2><b>A</b></h2>").is_err());
    }

    #[test]
    fn output_does_not_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("result.json");
        new_output(&path).unwrap();
        assert!(new_output(&path).is_err());
    }

    #[test]
    fn observations_have_no_fake_usage() {
        let observation = Observation {
            http_status: None,
            elapsed_ms: 1,
            error: Some("answer_transport_failed"),
            response: None,
            usage: None,
        };
        let value = serde_json::to_value(observation).unwrap();
        assert!(value["usage"].is_null());
        assert!(value["response"].is_null());
    }
}
