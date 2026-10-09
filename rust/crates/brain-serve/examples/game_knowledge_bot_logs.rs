use serde_json::{json, Value};
use std::{fs, path::Path, time::Duration};

type Result<T> = std::result::Result<T, &'static str>;

async fn mcp(
    client: &reqwest::Client,
    credential: &str,
    name: &str,
    arguments: Value,
) -> Result<Value> {
    let response = client
        .post("http://127.0.0.1:8890/mcp")
        .bearer_auth(credential)
        .json(&json!({"jsonrpc":"2.0","id":1,"method":"tools/call",
            "params":{"name":name,"arguments":arguments}}))
        .send()
        .await
        .map_err(|_| "mcp_unavailable")?
        .error_for_status()
        .map_err(|_| "mcp_request_failed")?;
    let value: Value = response.json().await.map_err(|_| "mcp_invalid")?;
    if value.get("error").is_some() || value["result"]["isError"] == true {
        return Err("mcp_tool_failed");
    }
    serde_json::from_str(
        value["result"]["content"][0]["text"]
            .as_str()
            .ok_or("mcp_text_missing")?,
    )
    .map_err(|_| "mcp_data_invalid")
}

fn channels(value: &Value, matches: &mut Vec<String>) {
    if value["name"] == "bot-logs" {
        if let Some(id) = value["id"].as_str() {
            matches.push(id.to_owned());
        }
    }
    match value {
        Value::Array(values) => values.iter().for_each(|value| channels(value, matches)),
        Value::Object(values) => values.values().for_each(|value| channels(value, matches)),
        _ => {}
    }
}

async fn run(args: &[String]) -> Result<()> {
    if args.len() != 3 || args.iter().take(2).any(|arg| !Path::new(arg).is_absolute()) {
        return Err("usage_after_jsonl_infisical_config_expected_release");
    }
    let input = fs::read_to_string(&args[0]).map_err(|_| "answers_unavailable")?;
    if input.len() > 1024 * 1024 {
        return Err("answer_input_too_large");
    }
    let rows: Vec<Value> = input
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| "answer_input_invalid")?;
    if rows.len() != 20
        || rows.iter().any(|row| {
            !row["response"].is_null() && row["response"]["knowledge_release"] != args[2]
        })
    {
        return Err("answer_release_or_count_mismatch");
    }
    let mut contents = Vec::new();
    for index in [0, 3, 14, 15] {
        let row = &rows[index];
        if row["response"]["status"] != "answered"
            || row["response"]["citations"]
                .as_array()
                .is_none_or(Vec::is_empty)
        {
            return Err("four_grounded_answers_required");
        }
        let question = row["question"].as_str().ok_or("question_missing")?;
        let text = row["response"]["text"].as_str().ok_or("answer_missing")?;
        let content = format!("**S2-Test: {question}**\n{text}");
        if content.chars().count() > 2000
            || content.contains("<@")
            || content.contains("@everyone")
            || content.contains("@here")
        {
            return Err("safe_complete_discord_message_required");
        }
        contents.push((index + 1, content));
    }
    let secrets = dl_token_secrets::values(Path::new(&args[1]))
        .await
        .map_err(|_| "secrets_unavailable")?;
    let credential = secrets
        .iter()
        .find(|(name, _)| name == "TWITCH_INTERNAL_API_TOKEN")
        .map(|(_, value)| value.as_str())
        .ok_or("mcp_credential_unavailable")?;
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| "mcp_client_invalid")?;
    let listed = mcp(&client, credential, "list_channels", json!({})).await?;
    let mut matches = Vec::new();
    channels(&listed, &mut matches);
    matches.sort();
    matches.dedup();
    let [channel_id] = matches.as_slice() else {
        return Err("unique_bot_logs_channel_required");
    };
    if !channel_id.bytes().all(|byte| byte.is_ascii_digit())
        || channel_id.parse::<u64>().ok().is_none_or(|id| id == 0)
    {
        return Err("channel_id_invalid");
    }
    let channel = mcp(
        &client,
        credential,
        "api_call",
        json!({"method":"GET","path":format!("/channels/{channel_id}")}),
    )
    .await?;
    let guild_id = channel["guild_id"].as_str().ok_or("guild_missing")?;
    if channel["id"] != *channel_id
        || channel["name"] != "bot-logs"
        || channel["type"] != 0
        || !guild_id.bytes().all(|byte| byte.is_ascii_digit())
        || guild_id.parse::<u64>().ok().is_none_or(|id| id == 0)
    {
        return Err("bot_logs_destination_changed");
    }
    for (case, content) in contents {
        let sent = mcp(
            &client,
            credential,
            "send_message",
            json!({"channel_id":channel_id,"content":content}),
        )
        .await?;
        let message_id = sent["message_id"]
            .as_str()
            .ok_or("sent_message_id_missing")?;
        if !message_id.bytes().all(|byte| byte.is_ascii_digit())
            || message_id.parse::<u64>().ok().is_none_or(|id| id == 0)
            || sent["sent"] != true
            || sent["channel_id"] != *channel_id
        {
            return Err("sent_message_binding_invalid");
        }
        let verified = mcp(
            &client,
            credential,
            "api_call",
            json!({"method":"GET",
            "path":format!("/channels/{channel_id}/messages/{message_id}")}),
        )
        .await?;
        if verified["id"] != message_id
            || verified["content"] != content
            || verified["author"]["bot"] != true
            || verified["channel_id"] != *channel_id
        {
            return Err("posted_answer_readback_mismatch");
        }
        println!(
            "{}",
            json!({"case":case,"channel_id":channel_id,"message_id":message_id,
            "content_verified":true,"release":args[2],
            "url":format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}")})
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    std::panic::set_hook(Box::new(|_| eprintln!("bot_logs_probe_panic_redacted")));
    match run(&std::env::args().skip(1).collect::<Vec<_>>()).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
