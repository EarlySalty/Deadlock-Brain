use brain_client::AsyncBrainClient;
use brain_contracts::Query;
use brain_serve::Config;
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
    time::Duration,
};

type Result<T> = std::result::Result<T, &'static str>;

async fn run(args: &[String]) -> Result<()> {
    if args.len() != 5 {
        return Err(
            "usage_input_jsonl_new_output_jsonl_serve_config_infisical_config_expected_release",
        );
    }
    for path in &args[..4] {
        if !Path::new(path).is_absolute() {
            return Err("absolute_paths_required");
        }
    }
    let input = fs::File::open(&args[0]).map_err(|_| "questions_unavailable")?;
    if input.metadata().map_err(|_| "questions_unavailable")?.len() > 1024 * 1024 {
        return Err("question_input_too_large");
    }
    let mut questions = Vec::new();
    for line in BufReader::new(input).lines() {
        let value: Value = serde_json::from_str(&line.map_err(|_| "question_read_failed")?)
            .map_err(|_| "question_json_invalid")?;
        let question = value["question"]
            .as_str()
            .filter(|text| !text.trim().is_empty() && text.len() <= 8192)
            .ok_or("question_missing")?;
        questions.push(question.to_owned());
    }
    if questions.len() != 20 {
        return Err("exactly_twenty_questions_required");
    }
    let config = Config::parse(&fs::read(&args[2]).map_err(|_| "serve_config_unavailable")?)
        .map_err(|_| "serve_config_invalid")?;
    if !config.bind.ip().is_loopback() {
        return Err("local_service_required");
    }
    let grant = config
        .credentials
        .iter()
        .find(|grant| grant.actor_id == "dl-bot" && grant.channel == "discord")
        .ok_or("public_bot_grant_missing")?;
    let release = grant.release.as_ref().unwrap_or(&config.release);
    if release.id != args[4] || !grant.scopes.contains("bot.public") {
        return Err("unexpected_release_or_scope");
    }
    let secrets = dl_token_secrets::values(Path::new(&args[3]))
        .await
        .map_err(|_| "secrets_unavailable")?;
    let credential = secrets
        .iter()
        .find(|(name, _)| name == &grant.token_env)
        .map(|(_, value)| value.as_str())
        .ok_or("bot_credential_unavailable")?;
    let client = AsyncBrainClient::new_local(
        &format!("http://{}", config.bind),
        credential,
        Duration::from_millis(config.timeouts.request_ms),
    )
    .map_err(|_| "local_client_invalid")?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&args[1])
        .map_err(|_| "new_output_required")?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| "clock_invalid")?
        .as_millis();
    let mut answered = 0;
    let mut cited = 0;
    let mut transport_errors = 0;
    for (index, question) in questions.into_iter().enumerate() {
        let query: Query = serde_json::from_value(json!({
            "request_id": format!("synthetic-s2-after-{stamp}-{}", index + 1),
            "conversation_id": "synthetic-s2-game-knowledge",
            "text": question, "profile": "explain", "requested_scopes": ["bot.public"]
        }))
        .map_err(|_| "query_invalid")?;
        let row = match client
            .answer_for_discord_with_read_access(&query, 0, false)
            .await
        {
            Ok(answer) => {
                if answer.knowledge_release != args[4] {
                    return Err("live_release_mismatch");
                }
                answered += usize::from(answer.status == brain_contracts::AnswerStatus::Answered);
                cited += usize::from(!answer.citations.is_empty());
                json!({"question": question, "response": answer})
            }
            Err(_) => {
                transport_errors += 1;
                json!({"question": question, "response": null,
                    "request_id": query.request_id, "expected_release": args[4],
                    "error": "live_answer_failed"})
            }
        };
        serde_json::to_writer(&mut output, &row).map_err(|_| "output_write_failed")?;
        output.write_all(b"\n").map_err(|_| "output_write_failed")?;
        output.sync_all().map_err(|_| "output_sync_failed")?;
        println!(
            "{}",
            json!({"case": index + 1, "request_id": query.request_id,
            "answered": answered, "cited": cited, "transport_errors": transport_errors,
            "release": args[4]})
        );
    }
    println!(
        "{}",
        json!({"cases": 20, "answered": answered, "cited": cited,
        "transport_errors": transport_errors, "synthetic_requests": true, "discord_reads": false,
        "semantic_validation_required": true, "output": args[1]})
    );
    Ok(())
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    std::panic::set_hook(Box::new(|_| eprintln!("probe_panic_redacted")));
    match run(&std::env::args().skip(1).collect::<Vec<_>>()).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
