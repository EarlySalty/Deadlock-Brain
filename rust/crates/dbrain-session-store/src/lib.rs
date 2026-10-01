use anyhow::{bail, Context, Result};
use postgres::{Client, NoTls};
use serde_json::{json, Value};
use std::{io::Read, path::Path, str::FromStr};
use tb_crypto::FieldCipher;
pub const LIMIT: usize = 1024 * 1024;
/// State bytes plus the fixed revision/state envelope and an i64 revision.
pub const ENVELOPE_LIMIT: usize = LIMIT + 128;
/// The largest revision is 20 bytes; JSON punctuation/field names are 22 bytes.
/// 128 bytes also permits bounded surrounding whitespace on private input.
pub fn encode_envelope(envelope: &Value) -> Result<Vec<u8>> {
    let raw = serde_json::to_vec(envelope)?;
    if raw.len() > ENVELOPE_LIMIT {
        bail!("envelope too large");
    }
    Ok(raw)
}
pub fn decode_envelope(input: impl std::io::Read) -> Result<(i64, Value)> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Envelope {
        revision: i64,
        state: Value,
    }
    let mut raw = Vec::new();
    input
        .take((ENVELOPE_LIMIT + 1) as u64)
        .read_to_end(&mut raw)?;
    if raw.len() > ENVELOPE_LIMIT {
        bail!("envelope too large");
    }
    let envelope: Envelope = serde_json::from_slice(&raw)?;
    if envelope.revision < -1 || !valid_state(&envelope.state) {
        bail!("invalid envelope");
    }
    if serde_json::to_vec(&envelope.state)?.len() > LIMIT {
        bail!("state too large");
    }
    Ok((envelope.revision, envelope.state))
}
pub fn valid_account(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
fn aad(account: &str) -> String {
    format!("core.browser_credentials|storage_state|gemini-browser|{account}|1")
}
pub fn valid_state(state: &Value) -> bool {
    state.is_object()
        && state.get("cookies").is_some_and(Value::is_array)
        && state.get("origins").is_some_and(Value::is_array)
}
pub fn load(client: &mut Client, cipher: &FieldCipher, account: &str) -> Result<Value> {
    let row=client.query_opt("SELECT state_enc,revision,revoked_at IS NOT NULL FROM core.browser_credentials WHERE provider='gemini-browser' AND account_id=$1",&[&account])?;
    let Some(row) = row else {
        return Ok(json!({"revision":-1,"state":{"cookies":[],"origins":[]}}));
    };
    if row.get::<_, bool>(2) {
        bail!("state revoked; explicit reauthorization required");
    }
    let blob: Vec<u8> = row.get(0);
    if blob.len() > LIMIT + 1024 {
        bail!("stored state too large");
    }
    let raw = cipher
        .decrypt_field(&blob, &aad(account))
        .map_err(|_| anyhow::anyhow!("state decrypt failed"))?;
    if raw.len() > LIMIT {
        bail!("stored state too large");
    }
    let state: Value =
        serde_json::from_str(&raw).map_err(|_| anyhow::anyhow!("stored state invalid"))?;
    if !valid_state(&state) {
        bail!("stored state invalid");
    }
    Ok(json!({"revision":row.get::<_,i64>(1),"state":state}))
}
pub fn save(
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

pub async fn connect_with_values(
    config: &Path,
    key_name: &str,
    values: &[(String, zeroize::Zeroizing<String>)],
) -> Result<(Client, FieldCipher)> {
    let config_bytes = std::fs::read(config)?;
    let config_json: Value = serde_json::from_slice(&config_bytes)?;
    let dsn_name = config_json
        .get("database_secret")
        .and_then(Value::as_str)
        .context("database secret name missing")?;
    let dsn = values
        .iter()
        .find(|(k, _)| k == dsn_name)
        .map(|(_, v)| v)
        .context("database bootstrap missing")?;
    let key = values
        .iter()
        .find(|(k, _)| k == key_name)
        .map(|(_, v)| v)
        .context("key missing")?;
    let cipher = FieldCipher::from_hex_key(key.trim(), "v1")
        .map_err(|_| anyhow::anyhow!("key unavailable"))?;
    let mut options = postgres::Config::from_str(dsn)
        .map_err(|_| anyhow::anyhow!("invalid database configuration"))?;
    if options.get_hosts().is_empty() || options.get_hosts().iter().any(|h|matches!(h,postgres::config::Host::Tcp(host) if !matches!(host.as_str(),"localhost"|"127.0.0.1"|"::1"))) { bail!("local database required"); }
    options.connect_timeout(std::time::Duration::from_secs(10));
    let mut client = tokio::task::spawn_blocking(move || options.connect(NoTls))
        .await
        .map_err(|_| anyhow::anyhow!("database unavailable"))?
        .map_err(|_| anyhow::anyhow!("database unavailable"))?;
    tokio::task::spawn_blocking(move || {
        client.batch_execute("SET statement_timeout='15s'; SET lock_timeout='5s'")?;
        Ok((client, cipher))
    })
    .await?
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
