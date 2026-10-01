use anyhow::{bail, Context, Result};
use dbrain_session_store::{connect_with_values, load, save, valid_account, valid_state, LIMIT};
use serde::Deserialize;
use serde_json::Value;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    os::unix::fs::FileTypeExt,
    path::PathBuf,
};
#[derive(Deserialize)]
struct Config {
    account_id: String,
    infisical_config: PathBuf,
    #[serde(default = "key_name")]
    key_secret: String,
}
fn key_name() -> String {
    "DB_MASTER_KEY_V1".into()
}
fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().context("operation missing")?;
    let config_path = args.next().context("config missing")?;
    let fd: u32 = if mode == "status" {
        0
    } else {
        args.next().context("private pipe missing")?.parse()?
    };
    if !matches!(mode.as_str(), "get" | "put" | "status")
        || (mode != "status" && fd < 3)
        || args.next().is_some()
    {
        bail!("invalid invocation");
    }
    let config: Config = serde_json::from_slice(&std::fs::read(config_path)?)?;
    if !valid_account(&config.account_id) {
        bail!("invalid account");
    }
    let runtime = tokio::runtime::Runtime::new()?;
    let values = runtime.block_on(dl_token_secrets::values(&config.infisical_config))?;
    let (mut client, cipher) = runtime.block_on(connect_with_values(
        &config.infisical_config,
        &config.key_secret,
        &values,
    ))?;
    if mode == "status" {
        let row=client.query_opt("SELECT revision,revoked_at IS NOT NULL FROM core.browser_credentials WHERE provider='gemini-browser' AND account_id=$1",&[&config.account_id])?;
        let status = match row {
            Some(row) => {
                serde_json::json!({"stored":true,"revision":row.get::<_,i64>(0),"revoked":row.get::<_,bool>(1)})
            }
            None => serde_json::json!({"stored":false}),
        };
        println!("{}", serde_json::to_string(&status)?);
        return Ok(());
    }
    let mut pipe = OpenOptions::new()
        .read(mode == "put")
        .write(mode == "get")
        .open(format!("/proc/self/fd/{fd}"))?;
    if !pipe.metadata()?.file_type().is_fifo() {
        bail!("private pipe required");
    }
    if mode == "get" {
        let raw = serde_json::to_vec(&load(&mut client, &cipher, &config.account_id)?)?;
        if raw.len() > LIMIT {
            bail!("state too large");
        }
        pipe.write_all(&raw)?;
    } else {
        let mut raw = Vec::new();
        pipe.take((LIMIT + 1) as u64).read_to_end(&mut raw)?;
        if raw.len() > LIMIT {
            bail!("state too large");
        }
        let input: Value = serde_json::from_slice(&raw)?;
        let revision = input
            .get("revision")
            .and_then(Value::as_i64)
            .filter(|v| *v >= -1)
            .context("revision missing")?;
        let state = input
            .get("state")
            .filter(|v| valid_state(v))
            .context("invalid state")?;
        save(&mut client, &cipher, &config.account_id, revision, state)?;
    }
    Ok(())
}
fn main() {
    if run().is_err() {
        eprintln!("Browser-Sitzungsspeicher fehlgeschlagen. Konto, Datenbank, Schlüssel und Revision prüfen.");
        std::process::exit(1);
    }
}
