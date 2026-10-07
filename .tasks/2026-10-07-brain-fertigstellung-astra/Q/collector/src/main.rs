use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, &'static str>;

fn valid_version(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
}

fn private_directory(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.permissions().mode() & 0o777 != 0o700 {
                return Err("private_directory_not_restricted");
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            DirBuilder::new()
                .mode(0o700)
                .create(path)
                .map_err(|_| "private_directory_create_failed")?;
        }
        Err(_) => return Err("private_directory_unavailable"),
    }
    Ok(())
}

fn root(version: &str) -> Result<PathBuf> {
    if !valid_version(version) {
        return Err("invalid_version");
    }
    let q = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("missing_q_root")?;
    let private = q.join("private");
    private_directory(&private)?;
    let version = private.join(version);
    private_directory(&version)?;
    Ok(version)
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|_| "snapshot_exists_or_write_denied")?;
    file.write_all(bytes).map_err(|_| "snapshot_write_failed")?;
    file.sync_all().map_err(|_| "snapshot_sync_failed")
}

fn response_kind(text: &str) -> &'static str {
    let lower = text.to_lowercase();
    let has = |words: &[&str]| words.iter().any(|word| lower.contains(word));
    if has(&["patch", "geändert", "nerf", "buff"]) {
        "patch_original_source"
    } else if has(&["coach", "besser spielen", "pate", "patenschaft"]) {
        "coaching_or_community_help"
    } else if has(&[
        "wer bist",
        "was bist",
        "was kannst",
        "dein modell",
        "welches modell",
    ]) {
        "self_description_without_internals"
    } else if has(&["server", "voice", "kanal", "discord", "lounge", "invite"]) {
        "server_or_own_status"
    } else if has(&[
        "pocket", "haze", "spirit", "item", "held", "build", "damage", "schaden", "magic",
        "counter", "konter", "deadlock",
    ]) {
        "game_original_source"
    } else {
        "unlabelled"
    }
}

fn question_candidate(text: &str) -> bool {
    let lower = text.trim().to_lowercase();
    let lower = if let Some(mention) = lower.strip_prefix("<@") {
        if let Some((id, rest)) = mention.split_once('>') {
            let id = id.strip_prefix('!').unwrap_or(id);
            if !id.is_empty() && id.bytes().all(|c| c.is_ascii_digit()) {
                rest.trim_start()
            } else {
                lower.as_str()
            }
        } else {
            lower.as_str()
        }
    } else {
        lower.as_str()
    };
    !lower.is_empty()
        && (lower.contains('?')
            || [
                "wie ", "was ", "wo ", "wer ", "warum ", "kann ", "gibt ", "welche ", "scal",
                "skal",
            ]
            .iter()
            .any(|prefix| lower.starts_with(prefix)))
}

fn postgres(database: &str, sql: &str) -> Result<Vec<Value>> {
    let output = Command::new("psql")
        .args([
            "-X",
            "--no-password",
            "-h",
            "/var/run/postgresql",
            "-U",
            "nathanael",
            "-d",
            database,
            "-At",
            "-c",
            sql,
        ])
        .env(
            "PGOPTIONS",
            "-c default_transaction_read_only=on -c statement_timeout=15000 -c lock_timeout=1000",
        )
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| "existing_postgres_reader_start_failed")?;
    if !output.status.success() {
        return Err("existing_postgres_reader_query_failed");
    }
    let value: Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| "existing_postgres_reader_invalid_json")?;
    value.as_array().cloned().ok_or("source_not_array")
}

fn discord(pages: usize) -> Result<(Vec<Value>, Option<&'static str>)> {
    let mut rows = Vec::new();
    let mut before: Option<String> = None;
    let mut seen = BTreeSet::new();
    for page in 0..pages {
        let mut arguments =
            json!({"channel_id":"1374364800817303632", "max":5, "save_to_file":false});
        if let Some(before) = &before {
            arguments["before"] = json!(before);
        }
        let request = json!({"jsonrpc":"2.0", "id":page+1, "method":"tools/call", "params":{"name":"read_messages", "arguments":arguments}});
        let mut child = Command::new("python3")
            .arg("/home/nathanael/Documents/tools/dl-bot-mcp-stdio.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "existing_discord_adapter_start_failed")?;
        let mut input = child.stdin.take().ok_or("adapter_stdin_missing")?;
        writeln!(input, "{request}").map_err(|_| "adapter_request_failed")?;
        drop(input);
        let output = child
            .wait_with_output()
            .map_err(|_| "adapter_wait_failed")?;
        if !output.status.success() {
            return Ok((rows, Some("adapter_failed")));
        }
        let parsed = (|| -> Option<Vec<Value>> {
            let rpc: Value = serde_json::from_slice(&output.stdout).ok()?;
            if rpc.get("error").is_some() || rpc["result"]["isError"] == true {
                return None;
            }
            let text = rpc["result"]["content"]
                .as_array()?
                .iter()
                .find(|item| item["type"] == "text")?["text"]
                .as_str()?;
            let data: Value = serde_json::from_str(text).ok()?;
            if data["channel_id"] != "1374364800817303632" {
                return None;
            }
            data["messages"].as_array().cloned()
        })();
        let Some(messages) = parsed else {
            return Ok((rows, Some("inline_page_unavailable")));
        };
        if messages.is_empty() {
            break;
        }
        let cursor = messages
            .iter()
            .filter_map(|message| message["id"].as_str()?.parse::<u64>().ok())
            .min()
            .ok_or("invalid_source_cursor")?
            .to_string();
        if before.as_deref() == Some(&cursor) {
            return Err("source_cursor_not_advanced");
        }
        before = Some(cursor);
        let exhausted = messages.len() < 5;
        for message in messages {
            let id = message["id"].as_str().ok_or("message_id_missing")?;
            if seen.insert(id.to_owned()) && message["author"]["bot"] == false {
                rows.push(json!({
                    "source_ref": {"channel_id":"1374364800817303632", "message_id":id, "timestamp":message["timestamp"], "author_id":message["author"]["id"]},
                    "original":message["content"],
                    "provenance":"discord_human_message",
                    "sample_authenticity":"needs_check_not_all_human_posts_are_unscripted_questions"
                }));
            }
        }
        if exhausted {
            break;
        }
    }
    Ok((rows, None))
}

fn public_assets(source: &str) -> Result<Vec<Value>> {
    let url = match source {
        "public-heroes" => "https://api.deadlock-api.com/v1/assets/heroes?only_active=true",
        "public-items" => "https://api.deadlock-api.com/v1/assets/items",
        "public-patches" => "https://api.steampowered.com/ISteamNews/GetNewsForApp/v2/?appid=1422450&count=500&maxlength=0",
        _ => return Err("invalid_public_source"),
    };
    let output = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--fail",
            "--max-time",
            "30",
            url,
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| "public_source_reader_start_failed")?;
    if !output.status.success() {
        return Err("public_source_request_failed");
    }
    public_rows(source, &output.stdout)
}

fn public_rows(source: &str, bytes: &[u8]) -> Result<Vec<Value>> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| "public_source_invalid_json")?;
    if source == "public-patches" {
        if value["appnews"]["appid"] != 1_422_450 {
            return Err("public_patch_app_mismatch");
        }
        let rows = value["appnews"]["newsitems"]
            .as_array()
            .ok_or("public_patch_news_missing")?;
        Ok(rows
            .iter()
            .filter(|row| row["feedname"] == "steam_community_announcements")
            .cloned()
            .collect())
    } else {
        value.as_array().cloned().ok_or("public_source_not_array")
    }
}

fn snapshot(version: &str, source: &str, pages: usize) -> Result<()> {
    let root = root(version)?;
    let output_path = root.join(format!("{source}.json"));
    if output_path.exists() {
        return Err("version_source_already_frozen");
    }
    let (rows, collection_error) = match source {
        "botlogs" => discord(pages)?,
        "dm" => (postgres("deadlock", "SELECT COALESCE(json_agg(q), '[]'::json) FROM (SELECT json_build_object('row_id',c.id,'user_id',c.user_id,'guild_id',c.guild_id,'timestamp',c.created_at) AS source_ref, c.content AS original, 'bot.concierge_conversations.user' AS provenance FROM bot.concierge_conversations c WHERE c.role='user' AND NOT EXISTS (SELECT 1 FROM core.user_privacy p WHERE p.user_id=c.user_id AND p.opted_out) ORDER BY c.created_at DESC,c.id DESC LIMIT 1000) q")?, None),
        "twitch" => (postgres("twitch_analytics", "SELECT COALESCE(json_agg(q), '[]'::json) FROM (SELECT json_build_object('row_id',id,'message_id',message_id,'channel',streamer_login,'chatter_id',chatter_id,'timestamp',message_ts) AS source_ref, content AS original, 'public.twitch_chat_messages' AS provenance, 'needs_human_origin_check' AS sample_authenticity FROM public.twitch_chat_messages WHERE NOT is_command AND content IS NOT NULL AND message_id ~ '^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$' ORDER BY message_ts DESC,id DESC LIMIT 1000) q")?, None),
        "twitch-brain" => (postgres("twitch_analytics", "SELECT COALESCE(json_agg(q), '[]'::json) FROM (SELECT json_build_object('row_id',id,'message_id',message_id,'broadcaster_user_id',broadcaster_user_id,'chatter_user_id',chatter_user_id,'timestamp',created_at) AS source_ref, question AS original, 'public.tb_chat_brain_answers.question' AS provenance, 'needs_original_chat_and_authenticity_check' AS sample_authenticity FROM public.tb_chat_brain_answers WHERE message_id ~ '^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$' ORDER BY created_at DESC,id DESC LIMIT 1000) q")?, None),
        "public-heroes" | "public-items" | "public-patches" => (public_assets(source)?, None),
        _ => return Err("invalid_source"),
    };
    let mut candidates = Vec::new();
    let mut categories = BTreeMap::<String, usize>::new();
    for (index, row) in rows.iter().enumerate() {
        let Some(text) = row["original"].as_str() else {
            continue;
        };
        if question_candidate(text) {
            let kind = response_kind(text);
            *categories.entry(kind.into()).or_default() += 1;
            candidates.push(json!({"source_row":index, "provisional_response_kind":kind, "label_status":"pending_original_source_and_authenticity_check", "expected_original_facts":[]}));
        }
    }
    let count = candidates.len();
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "clock_invalid")?
        .as_secs();
    let capture = json!({"schema":"brain.q.private-source.v1", "version":version, "source":source, "captured_at_unix":epoch, "collection_error":collection_error, "original_rows":rows, "candidate_expectations":candidates, "accepted_gold_cases":0, "model_requests":0});
    let bytes = serde_json::to_vec_pretty(&capture).map_err(|_| "snapshot_encode_failed")?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    write_new(&output_path, &bytes)?;
    write_new(&root.join(format!("{source}.sha256")), hash.as_bytes())?;
    println!(
        "{}",
        json!({"version":version,"source":source,"sha256":hash,"original_rows":rows.len(),"question_candidates":count,"provisional_categories":categories,"accepted_gold_cases":0,"collection_error":collection_error,"model_requests":0})
    );
    Ok(())
}

fn verified_capture(root: &Path, source: &str) -> Result<(Value, String)> {
    let path = root.join(format!("{source}.json"));
    let metadata = fs::symlink_metadata(&path).map_err(|_| "source_snapshot_missing")?;
    if !metadata.is_file() || metadata.permissions().mode() & 0o777 != 0o600 {
        return Err("source_snapshot_not_restricted");
    }
    let bytes = fs::read(path).map_err(|_| "source_snapshot_read_failed")?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    let expected = fs::read_to_string(root.join(format!("{source}.sha256")))
        .map_err(|_| "source_digest_missing")?;
    if expected != digest {
        return Err("source_digest_mismatch");
    }
    let value = serde_json::from_slice(&bytes).map_err(|_| "source_snapshot_invalid_json")?;
    Ok((value, digest))
}

fn plan(version: &str) -> Result<()> {
    let root = root(version)?;
    let path = root.join("partial-plan-v1.json");
    if path.exists() {
        return Err("partial_plan_already_frozen");
    }
    let mut cases = Vec::new();
    let mut sources = BTreeMap::new();
    for source in ["botlogs", "dm", "twitch", "twitch-brain"] {
        if !root.join(format!("{source}.json")).exists() {
            continue;
        }
        let (capture, digest) = verified_capture(&root, source)?;
        sources.insert(source, digest.clone());
        let rows = capture["original_rows"]
            .as_array()
            .ok_or("source_rows_missing")?;
        for (row, record) in rows.iter().enumerate() {
            let Some(text) = record["original"].as_str() else {
                continue;
            };
            if question_candidate(text) {
                cases.push(json!({"case_id":format!("{source}-{row:04}"), "source":source, "source_sha256":digest, "source_row":row, "expected_response_kind":response_kind(text), "authenticity_check":"pending", "original_fact_check":"pending", "accepted_gold":false}));
            }
        }
    }
    let count = cases.len();
    let value = json!({"schema":"brain.q.partial-plan.v1", "version":version, "source_hashes":sources, "cases":cases, "accepted_gold_cases":0, "model_runs":0, "private_replay_allowed":false});
    let bytes = serde_json::to_vec_pretty(&value).map_err(|_| "plan_encode_failed")?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    write_new(&path, &bytes)?;
    write_new(&root.join("partial-plan-v1.sha256"), hash.as_bytes())?;
    println!(
        "{}",
        json!({"version":version,"partial_plan_sha256":hash,"candidate_cases":count,"source_count":sources.len(),"accepted_gold_cases":0,"model_requests":0})
    );
    Ok(())
}

fn verify(version: &str) -> Result<()> {
    let root = root(version)?;
    let (plan, hash) = verified_capture(&root, "partial-plan-v1")?;
    let sources = plan["source_hashes"]
        .as_object()
        .ok_or("plan_sources_missing")?;
    for (source, expected) in sources {
        if !["botlogs", "dm", "twitch", "twitch-brain"].contains(&source.as_str()) {
            return Err("plan_source_invalid");
        }
        let (_, actual) = verified_capture(&root, source)?;
        if expected.as_str() != Some(&actual) {
            return Err("plan_source_digest_mismatch");
        }
    }
    println!(
        "{}",
        json!({"version":version,"partial_plan_sha256":hash,"verified_sources":sources.len(),"candidate_cases":plan["cases"].as_array().ok_or("plan_cases_missing")?.len(),"accepted_gold_cases":0,"private_replay_allowed":false,"probe_kind":"local_file_integrity_only","model_requests":0})
    );
    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "plan" {
        return plan(&args[1]);
    }
    if args.len() == 2 && args[0] == "verify" {
        return verify(&args[1]);
    }
    if args.len() < 2 || args.len() > 3 {
        return Err("usage_source_version_optional_pages");
    }
    if ![
        "botlogs",
        "dm",
        "twitch",
        "twitch-brain",
        "public-heroes",
        "public-items",
        "public-patches",
    ]
    .contains(&args[0].as_str())
    {
        return Err("invalid_source");
    }
    let pages = args
        .get(2)
        .map(|v| v.parse::<usize>().map_err(|_| "invalid_pages"))
        .transpose()?
        .unwrap_or(40);
    if !(1..=400).contains(&pages) {
        return Err("invalid_pages");
    }
    snapshot(&args[1], &args[0], pages)
}

fn main() {
    if let Err(code) = run() {
        eprintln!("{}", json!({"error_code":code,"model_requests":0}));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_cannot_escape_private_directory() {
        assert!(valid_version("q-partial-v1-20261007"));
        for value in ["", "..", "../x", "a/b", "a.b", "a b"] {
            assert!(!valid_version(value));
        }
    }

    #[test]
    fn heuristic_does_not_invent_unknown_kind() {
        assert_eq!(response_kind("unbekannter Sachverhalt?"), "unlabelled");
        assert_eq!(
            response_kind("Wie countert man Pocket?"),
            "game_original_source"
        );
    }

    #[test]
    fn public_patch_rows_exclude_third_party_news_without_gold_labelling() {
        let payload = json!({"appnews":{"appid":1422450,"newsitems":[
            {"feedname":"steam_community_announcements","title":"fixture"},
            {"feedname":"external_news","title":"fixture"}
        ]}});
        let bytes = serde_json::to_vec(&payload).unwrap();
        let rows = public_rows("public-patches", &bytes).unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].get("accepted_gold").is_none());
        assert_eq!(public_rows("public-items", b"[]").unwrap().len(), 0);
    }

    #[test]
    fn public_patch_rows_reject_wrong_app_and_missing_news() {
        assert_eq!(
            public_rows(
                "public-patches",
                br#"{"appnews":{"appid":1,"newsitems":[]}}"#
            ),
            Err("public_patch_app_mismatch")
        );
        assert_eq!(
            public_rows("public-patches", br#"{"appnews":{"appid":1422450}}"#),
            Err("public_patch_news_missing")
        );
    }

    #[test]
    fn non_question_is_not_replaced_with_synthetic_question() {
        assert!(!question_candidate("Danke"));
        assert!(!question_candidate(""));
        assert!(question_candidate("Kann ich besser spielen?"));
        assert!(question_candidate("<@123> wie countert man Pocket"));
        assert!(question_candidate(
            "<@!123> scalled haze auch mit Magic dmg"
        ));
        assert!(!question_candidate("<@no-id> wie geht es"));
    }
}
