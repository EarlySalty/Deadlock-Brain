//! Browserzustand nur verschlüsselt in Postgres. Der Klartext wird ausschließlich
//! über einen ausdrücklich geerbten Pipe-FD ausgetauscht, niemals über Dateien
//! oder stdout. Die Kryptooperationen liegen im Rust-Helfer; der vorhandene
//! geschützte Launcher liefert den Schlüssel, ohne ihn in Dateien abzulegen.
use anyhow::{bail, Context, Result};
use postgres::{Client, NoTls};
use serde_json::{json, Value};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    os::unix::fs::FileTypeExt,
    str::FromStr,
};
use tb_crypto::FieldCipher;
const LIMIT: usize = 1024 * 1024;

fn valid_account(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
fn aad(account: &str) -> String {
    format!("core.browser_credentials|storage_state|gemini-browser|{account}|1")
}
fn valid_state(state: &Value) -> bool {
    state.is_object()
        && state.get("cookies").is_some_and(Value::is_array)
        && state.get("origins").is_some_and(Value::is_array)
}
fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().context("operation missing")?;
    if !matches!(mode.as_str(), "get" | "put") || args.next().is_some() {
        bail!("invalid operation");
    }
    let account = std::env::var("GEMINI_ACCOUNT_ID").context("account missing")?;
    if !valid_account(&account) {
        bail!("invalid account");
    }
    let fd = std::env::var("GEMINI_SESSION_FD")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .filter(|fd| *fd >= 3)
        .context("private pipe missing")?;
    let mut pipe = OpenOptions::new()
        .read(mode == "put")
        .write(mode == "get")
        .open(format!("/proc/self/fd/{fd}"))?;
    if !pipe.metadata()?.file_type().is_fifo() {
        bail!("private pipe required");
    }
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").context("database bootstrap missing")?;
    let mut options = postgres::Config::from_str(&dsn)
        .map_err(|_| anyhow::anyhow!("invalid database configuration"))?;
    if options.get_hosts().is_empty() || options.get_hosts().iter().any(|h|matches!(h,postgres::config::Host::Tcp(host) if !matches!(host.as_str(),"localhost"|"127.0.0.1"|"::1"))) {bail!("local database required");}
    options.connect_timeout(std::time::Duration::from_secs(10));
    let mut client = options
        .connect(NoTls)
        .map_err(|_| anyhow::anyhow!("database unavailable"))?;
    client.batch_execute("SET statement_timeout='15s'; SET lock_timeout='5s'")?;
    let cipher = FieldCipher::from_env().map_err(|_| anyhow::anyhow!("key unavailable"))?;
    if mode == "get" {
        let result = load(&mut client, &cipher, &account)?;
        let raw = serde_json::to_vec(&result)?;
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
        let input: Value =
            serde_json::from_slice(&raw).map_err(|_| anyhow::anyhow!("invalid state envelope"))?;
        let revision = input
            .get("revision")
            .and_then(Value::as_i64)
            .filter(|v| *v >= -1)
            .context("revision missing")?;
        let state = input
            .get("state")
            .filter(|v| valid_state(v))
            .context("invalid state")?;
        save(&mut client, &cipher, &account, revision, state)?;
    }
    Ok(())
}

fn load(client: &mut Client, cipher: &FieldCipher, account: &str) -> Result<Value> {
    let row=client.query_opt("SELECT state_enc,revision,revoked_at IS NOT NULL FROM core.browser_credentials WHERE provider='gemini-browser' AND account_id=$1",&[&account])?;
    let Some(row) = row else {
        return Ok(json!({"revision":-1,"state":{"cookies":[],"origins":[]}}));
    };
    if row.get::<_, bool>(2) {
        bail!("state revoked; explicit reauthorization required");
    }
    let blob: Vec<u8> = row.get(0);
    let raw = cipher
        .decrypt_field(&blob, &aad(account))
        .map_err(|_| anyhow::anyhow!("state decrypt failed"))?;
    let state: Value =
        serde_json::from_str(&raw).map_err(|_| anyhow::anyhow!("stored state invalid"))?;
    if !valid_state(&state) {
        bail!("stored state invalid");
    }
    Ok(json!({"revision":row.get::<_,i64>(1),"state":state}))
}
fn save(
    client: &mut Client,
    cipher: &FieldCipher,
    account: &str,
    revision: i64,
    state: &Value,
) -> Result<()> {
    if !valid_account(account) || !valid_state(state) {
        bail!("invalid state");
    }
    let raw = serde_json::to_string(state)?;
    if raw.len() > LIMIT {
        bail!("state too large");
    }
    let blob = cipher
        .encrypt_field(&raw, &aad(account))
        .map_err(|_| anyhow::anyhow!("state encrypt failed"))?;
    // Vor dem Write Rücklesbarkeit nachweisen. Keine Rohkopie für Rollback anlegen.
    if cipher
        .decrypt_field(&blob, &aad(account))
        .map_err(|_| anyhow::anyhow!("state verify failed"))?
        != raw
    {
        bail!("state verify failed");
    }
    let changed = if revision == -1 {
        client.execute("INSERT INTO core.browser_credentials(provider,account_id,state_enc) VALUES('gemini-browser',$1,$2) ON CONFLICT(provider,account_id) DO NOTHING",&[&account,&blob])?
    } else {
        client.execute("UPDATE core.browser_credentials SET state_enc=$1,revision=revision+1,updated_at=now() WHERE provider='gemini-browser' AND account_id=$2 AND revision=$3 AND revoked_at IS NULL",&[&blob,&account,&revision])?
    };
    if changed != 1 {
        bail!("state changed or revoked; do not overwrite");
    }
    Ok(())
}
fn main() {
    if run().is_err() {
        // Fehlerketten können Provider-/DB-Werte enthalten. Keine weiterreichen.
        eprintln!("Browser-Sitzungsspeicher fehlgeschlagen. Konto, Datenbank, Schlüssel und Revision prüfen. Kein Datei-Fallback.");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod database_tests;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_account_and_structured_state_without_echoing_data() {
        assert!(valid_account("demo-account-1"));
        assert!(!valid_account("a|b"));
        assert!(!valid_account(""));
        assert!(valid_state(&json!({"cookies":[],"origins":[]})));
        assert!(!valid_state(
            &json!({"cookies":"not-an-array","origins":[]})
        ));
        let cipher = FieldCipher::from_hex_key(&"11".repeat(32), "v1").unwrap();
        let blob = cipher
            .encrypt_field("synthetic-state", &aad("first"))
            .unwrap();
        assert!(cipher.decrypt_field(&blob, &aad("second")).is_err());
    }
}
