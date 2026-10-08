use brain_client::{AnswerProfile, AsyncBrainClient, Query};
use brain_serve::{config::ProviderKind, Config};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::Path,
    process::Command,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GoldSet {
    schema: String,
    version: String,
    source_snapshot_sha256: String,
    anonymization: String,
    cases: Vec<GoldCase>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GoldCase {
    case: String,
    topic: String,
    question: String,
    source: String,
    expected: Vec<String>,
    evidence: Vec<String>,
    required_any: Vec<Vec<String>>,
    forbidden: Vec<String>,
    requires_citations: bool,
}

#[derive(Serialize)]
struct Checks {
    required_groups: Vec<bool>,
    forbidden_terms_absent: bool,
    citations_present_if_required: bool,
    lexical_pass: bool,
    semantic_verdict: Option<bool>,
}

fn parse(bytes: &[u8]) -> Result<GoldSet> {
    let set: GoldSet = serde_json::from_slice(bytes).map_err(|_| "gold_json_invalid")?;
    let mut questions = BTreeSet::new();
    let mut cases = BTreeSet::new();
    let mut topics = BTreeSet::new();
    if set.schema != "brain.community-gold.v1"
        || set.version.trim().is_empty()
        || set.source_snapshot_sha256.len() != 64
        || !set
            .source_snapshot_sha256
            .bytes()
            .all(|b| b.is_ascii_hexdigit())
        || set.anonymization.trim().is_empty()
        || set.cases.len() < 30
    {
        return Err("gold_set_incomplete");
    }
    for case in &set.cases {
        let question = case
            .question
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if !cases.insert(&case.case)
            || !questions.insert(question)
            || case.question.trim().is_empty()
            || case.question.contains(['@', '<', '>'])
            || case.question.contains("[entfernt]")
            || case
                .question
                .split(|c: char| !c.is_ascii_digit())
                .any(|s| s.len() > 8)
            || !["builds", "items", "heroes", "patch", "server", "invite"]
                .contains(&case.topic.as_str())
            || case.source.trim().is_empty()
            || case.expected.is_empty()
            || case.evidence.is_empty()
            || case
                .expected
                .iter()
                .chain(&case.evidence)
                .any(|s| s.trim().is_empty())
            || case.required_any.is_empty()
            || case
                .required_any
                .iter()
                .any(|group| group.is_empty() || group.iter().any(|s| s.trim().is_empty()))
            || case.forbidden.iter().any(|s| s.trim().is_empty())
        {
            return Err("gold_case_invalid");
        }
        topics.insert(case.topic.as_str());
    }
    if topics.len() != 6 {
        return Err("gold_topic_coverage_incomplete");
    }
    Ok(set)
}

fn checks(case: &GoldCase, text: &str, citations: usize) -> Checks {
    let lower = text.to_lowercase();
    let required_groups: Vec<_> = case
        .required_any
        .iter()
        .map(|group| {
            group
                .iter()
                .any(|term| lower.contains(&term.to_lowercase()))
        })
        .collect();
    let forbidden_terms_absent = case
        .forbidden
        .iter()
        .all(|term| !lower.contains(&term.to_lowercase()));
    let citations_present_if_required = !case.requires_citations || citations > 0;
    let lexical_pass = required_groups.iter().all(|matched| *matched)
        && forbidden_terms_absent
        && citations_present_if_required;
    Checks {
        required_groups,
        forbidden_terms_absent,
        citations_present_if_required,
        lexical_pass,
        semantic_verdict: None,
    }
}

fn process_binding(config_path: &str) -> Result<Value> {
    let output = Command::new("systemctl")
        .args([
            "--user",
            "show",
            "brain-serve",
            "--property=MainPID",
            "--value",
        ])
        .output()
        .map_err(|_| "live_process_unavailable")?;
    if !output.status.success() {
        return Err("live_process_unavailable");
    }
    let pid: u32 = std::str::from_utf8(&output.stdout)
        .map_err(|_| "live_pid_invalid")?
        .trim()
        .parse()
        .map_err(|_| "live_pid_invalid")?;
    if pid == 0 {
        return Err("live_pid_invalid");
    }
    let cmdline =
        fs::read(format!("/proc/{pid}/cmdline")).map_err(|_| "live_command_unavailable")?;
    let arguments: Vec<_> = cmdline.split(|byte| *byte == 0).collect();
    if !arguments
        .windows(2)
        .any(|pair| pair[0] == b"--config" && pair[1] == config_path.as_bytes())
    {
        return Err("config_is_not_the_live_service_config");
    }
    let exe = fs::read_link(format!("/proc/{pid}/exe")).map_err(|_| "live_exe_unavailable")?;
    let release_root = Path::new("/opt/deadlock-brain/maintenance-releases");
    let relative = exe
        .strip_prefix(release_root)
        .map_err(|_| "live_release_path_invalid")?;
    let parts: Vec<_> = relative.components().collect();
    if parts.len() != 2 || parts[1].as_os_str() != "brain-serve" {
        return Err("live_release_path_invalid");
    }
    let sha = parts[0]
        .as_os_str()
        .to_str()
        .ok_or("live_release_sha_invalid")?;
    if sha.len() != 40 || !sha.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("live_release_sha_invalid");
    }
    Ok(json!({"pid":pid,"sha":sha,"exe":exe}))
}

async fn run(args: &[String]) -> Result<()> {
    if args.len() == 2 && args[0] == "validate" {
        let set = parse(&fs::read(&args[1]).map_err(|_| "gold_unavailable")?)?;
        println!(
            "{}",
            json!({"version":set.version,"cases":set.cases.len(),"topics":6,"semantic_review_required":true})
        );
        return Ok(());
    }
    if args.len() != 6 || args[0] != "run" {
        return Err(
            "usage_validate_gold_or_run_gold_output_runtime_config_infisical_config_expected_model",
        );
    }
    let bytes = fs::read(&args[1]).map_err(|_| "gold_unavailable")?;
    let set = parse(&bytes)?;
    let config_bytes = fs::read(&args[3]).map_err(|_| "runtime_config_unavailable")?;
    let config = Config::parse(&config_bytes).map_err(|_| "runtime_config_invalid")?;
    if !matches!(config.provider.kind, ProviderKind::CodexSubscription)
        || config.provider.model != args[5]
        || !config.bind.ip().is_loopback()
    {
        return Err("requested_subscription_model_not_configured");
    }
    let grant = config
        .credentials
        .iter()
        .find(|grant| {
            grant.actor_id == "docs-client"
                && grant.channel == "docs"
                && grant.scopes == BTreeSet::from(["bot.public".to_owned()])
        })
        .ok_or("public_service_grant_missing")?;
    let output_path = Path::new(&args[2]);
    let parent = output_path
        .parent()
        .ok_or("private_output_parent_missing")?;
    let metadata = fs::metadata(parent).map_err(|_| "private_output_parent_missing")?;
    let uid = fs::metadata("/proc/self")
        .map_err(|_| "own_uid_unavailable")?
        .uid();
    if fs::canonicalize(parent).map_err(|_| "private_output_parent_missing")? != parent
        || !metadata.is_dir()
        || metadata.uid() != uid
        || metadata.permissions().mode() & 0o777 != 0o700
    {
        return Err("private_output_parent_not_restricted");
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(output_path)
        .map_err(|_| "output_exists_or_unavailable")?;
    let binding = process_binding(&args[3])?;
    let secrets = dl_token_secrets::values(Path::new(&args[4]))
        .await
        .map_err(|_| "existing_secret_loader_failed")?;
    let token = secrets
        .iter()
        .find(|(key, _)| key == &grant.token_env)
        .map(|(_, value)| value.as_str())
        .ok_or("public_service_credential_missing")?;
    let client = AsyncBrainClient::new(
        &format!("http://{}", config.bind),
        token,
        Duration::from_millis(config.timeouts.request_ms),
    )
    .map_err(|_| "existing_brain_client_invalid")?;
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "clock_invalid")?
        .as_nanos();
    let header = json!({"schema":"brain.community-gold-run.v1","version":set.version,"gold_sha256":format!("{:x}",Sha256::digest(&bytes)),"runtime_config_sha256":format!("{:x}",Sha256::digest(&config_bytes)),"provider_kind":"codex_subscription","configured_model":config.provider.model,"live_process":binding,"transport":"public_docs_service_http_not_discord_delivery","cases":set.cases.len(),"started_at_unix_ns":run_id});
    writeln!(output, "{header}").map_err(|_| "output_write_failed")?;
    output.sync_all().map_err(|_| "output_sync_failed")?;
    let mut answered = 0;
    let mut lexical_passes = 0;
    for case in &set.cases {
        if process_binding(&args[3])? != binding
            || fs::read(&args[3]).map_err(|_| "runtime_config_unavailable")? != config_bytes
        {
            return Err("live_binding_changed_during_run");
        }
        let query = Query {
            domain: None,
            request_id: format!("gold-{run_id}-{}", case.case),
            conversation_id: format!("gold-{run_id}-{}", case.case),
            text: case.question.clone(),
            requested_scopes: grant.scopes.clone(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        let started = Instant::now();
        let result = client.answer(&query).await;
        let elapsed_ms = started.elapsed().as_millis();
        let record = match result {
            Ok(answer) => {
                let rubric = checks(case, &answer.text, answer.citations.len());
                answered += usize::from(answer.status == brain_client::AnswerStatus::Answered);
                lexical_passes += usize::from(rubric.lexical_pass);
                json!({"case":case.case,"topic":case.topic,"elapsed_ms":elapsed_ms,"answer":answer,"checks":rubric})
            }
            Err(error) => {
                json!({"case":case.case,"topic":case.topic,"elapsed_ms":elapsed_ms,"transport_error":error.to_string(),"semantic_verdict":null})
            }
        };
        writeln!(output, "{record}").map_err(|_| "output_write_failed")?;
        output.sync_all().map_err(|_| "output_sync_failed")?;
        println!(
            "{}",
            json!({"case":case.case,"elapsed_ms":elapsed_ms,"answered":answered,"lexical_passes":lexical_passes})
        );
    }
    if process_binding(&args[3])? != binding
        || fs::read(&args[3]).map_err(|_| "runtime_config_unavailable")? != config_bytes
    {
        return Err("live_binding_changed_during_run");
    }
    let footer = json!({"completed_cases":set.cases.len(),"answered":answered,"lexical_passes":lexical_passes,"semantic_review_required":true,"live_process_unchanged":true});
    writeln!(output, "{footer}").map_err(|_| "output_write_failed")?;
    output.sync_all().map_err(|_| "output_sync_failed")?;
    println!("{footer}");
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let Err(code) = run(&args).await {
        eprintln!("{}", json!({"error_code":code}));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const GOLD: &[u8] =
        include_bytes!("../../../../architecture/migration/evals/community-gold.json");

    #[test]
    fn real_set_has_all_six_topics_and_unique_anonymized_questions() {
        let set = parse(GOLD).unwrap();
        assert!(set.cases.len() >= 30);
        let mut invalid: Value = serde_json::from_slice(GOLD).unwrap();
        invalid["cases"][0]["question"] = json!("<@123456789012345678> Frage?");
        assert!(parse(&serde_json::to_vec(&invalid).unwrap()).is_err());
        invalid = serde_json::from_slice(GOLD).unwrap();
        invalid["cases"][1]["question"] = invalid["cases"][0]["question"].clone();
        assert!(parse(&serde_json::to_vec(&invalid).unwrap()).is_err());
    }

    #[test]
    fn lexical_success_never_claims_semantic_acceptance() {
        let set = parse(GOLD).unwrap();
        let case = &set.cases[0];
        let text = case
            .required_any
            .iter()
            .map(|group| group[0].as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let result = checks(case, &text, 1);
        assert!(result.lexical_pass);
        assert_eq!(result.semantic_verdict, None);
        assert!(!checks(case, "", 0).lexical_pass);
    }
}
