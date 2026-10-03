use brain_contracts::DocumentStorePort;
use brain_feeds::{
    sheet_core::{self, SheetSource, SheetTabBody},
    source_sync::candidate_release,
    youtube_core::{self, YoutubeSource},
};
use brain_storage::PgStore;
use deadlock_brain_core::pg::infisical_environment;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool, Row,
};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    infisical_config: PathBuf,
    base_release_id: String,
    base_release_sha256: String,
    owner: String,
    #[serde(default = "http_timeout")]
    http_timeout_seconds: u64,
    #[serde(default)]
    sheets: Vec<SheetSource>,
    #[serde(default)]
    youtube: Vec<YoutubeSource>,
    #[serde(default)]
    youtube_metadata_api_key_secret: Option<String>,
}

fn http_timeout() -> u64 {
    90
}

fn read_config(path: &Path) -> Result<Config> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|_| "Konfiguration ist nicht lesbar.")?
        .take(65537)
        .read_to_end(&mut bytes)
        .map_err(|_| "Konfiguration ist nicht lesbar.")?;
    if bytes.len() > 65536 {
        return Err("Konfiguration ist zu groß.");
    }
    let config: Config =
        serde_json::from_slice(&bytes).map_err(|_| "Konfiguration ist ungültig.")?;
    if !config.infisical_config.is_absolute()
        || config.base_release_id.trim().is_empty()
        || config.owner.trim().is_empty()
        || config.http_timeout_seconds == 0
        || config.http_timeout_seconds > 300
        || config.base_release_sha256.len() != 64
        || !config
            .base_release_sha256
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("Konfigurationsfelder sind nicht zulässig.");
    }
    let mut sources = BTreeSet::new();
    for source in &config.sheets {
        source.validate().map_err(|_| "Tabellenrechte fehlen.")?;
        if source.rights.policy.allowed_scopes != BTreeSet::from(["second_brain.internal".into()]) {
            return Err("Tabellenkandidat benötigt die interne Second-Brain-Bindung.");
        }
        if !sources.insert(source.source_id()) {
            return Err("Doppelte Quelle.");
        }
    }
    for source in &config.youtube {
        source.validate().map_err(|_| "YouTube-Rechte fehlen.")?;
        if source.rights.policy.allowed_scopes != BTreeSet::from(["second_brain.internal".into()]) {
            return Err("YouTube-Kandidat benötigt die interne Second-Brain-Bindung.");
        }
        if !sources.insert(source.source_id()) {
            return Err("Doppelte Quelle.");
        }
    }
    if sources.is_empty() {
        return Err("Keine freigegebenen Quellen konfiguriert.");
    }
    if !config.youtube.is_empty() {
        metadata_secret_name(&config)?;
    }
    Ok(config)
}

fn metadata_secret_name(config: &Config) -> Result<&str> {
    config
        .youtube_metadata_api_key_secret
        .as_deref()
        .filter(|name| {
            !name.is_empty()
                && name.len() <= 128
                && name
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
                && !name.contains("GEMINI")
        })
        .ok_or("Expliziter YouTube-Metadatenzugang über Infisical fehlt.")
}

async fn pool(password: &str, readonly: bool) -> Result<PgPool> {
    let user = if readonly {
        "brain_readonly"
    } else {
        "brain_ingest"
    };
    let mut options = PgConnectOptions::new_without_pgpass()
        .host("/run/deadlock-brain-postgresql")
        .port(5446)
        .database("brain")
        .username(user)
        .password(password)
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-source-sync");
    if readonly {
        options = options.options([("default_transaction_read_only", "on")]);
    }
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| "Brain-Datenbankverbindung ist fehlgeschlagen.")?;
    let row = sqlx::query("SELECT current_database() AS database,current_user AS username,inet_server_addr()::text AS address,current_setting('port') AS port").fetch_one(&pool).await.map_err(|_| "Datenbankidentität fehlt.")?;
    if row.try_get::<String, _>("database").ok().as_deref() != Some("brain")
        || row.try_get::<String, _>("username").ok().as_deref() != Some(user)
        || row.try_get::<Option<String>, _>("address").ok() != Some(None)
        || row.try_get::<String, _>("port").ok().as_deref() != Some("5446")
    {
        return Err("Datenbankidentität ist nicht zulässig.");
    }
    Ok(pool)
}

fn google_url(url: &reqwest::Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port_or_known_default() == Some(443)
        && url.host_str().is_some_and(|host| {
            host == "docs.google.com" || host.ends_with(".googleusercontent.com")
        })
}

async fn tabs(source: &SheetSource, timeout: u64) -> Result<Vec<SheetTabBody>> {
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(timeout))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() >= 5 || !google_url(attempt.url()) {
                attempt.error("Nicht freigegebene Tabellenweiterleitung.")
            } else {
                attempt.follow()
            }
        }))
        .build()
        .map_err(|_| "Tabellenclient fehlt.")?;
    let mut tabs = Vec::new();
    for gid in &source.gids {
        let mut response = client
            .get(dbrain_sources::google_sheet::sheet_csv_url(
                &source.sheet_id,
                gid,
            ))
            .header("Accept", "text/csv")
            .send()
            .await
            .map_err(|_| "Tabellenabruf wurde unterbrochen.")?;
        if response.status() != reqwest::StatusCode::OK
            || !google_url(response.url())
            || !response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|s| {
                    s.split(';')
                        .next()
                        .is_some_and(|t| matches!(t.trim(), "text/csv" | "application/csv"))
                })
        {
            return Err("Tabellenantwort ist kein vollständiges CSV.");
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "Tabellenantwort wurde unterbrochen.")?
        {
            if bytes.len().saturating_add(chunk.len())
                > brain_ingestion::document_set::MAX_DOCUMENT_BYTES
            {
                return Err("Tabellenantwort ist zu groß.");
            }
            bytes.extend_from_slice(&chunk);
        }
        tabs.push(SheetTabBody {
            gid: gid.clone(),
            bytes,
            complete: true,
        });
    }
    Ok(tabs)
}

async fn run(config: Config) -> Result<serde_json::Value> {
    if [
        "INFISICAL_TOKEN_FILE",
        "CREDENTIALS_DIRECTORY",
        "PGPASSWORD",
        "DATABASE_URL",
        "DEADLOCK_CENTRAL_DSN",
        "PGOPTIONS",
        "PGSERVICE",
        "PGPASSFILE",
    ]
    .iter()
    .any(|name| std::env::var_os(name).is_some())
    {
        return Err("Dateibasierte oder geerbte Zugangsdaten sind nicht zulässig.");
    }
    brain_feeds::source_sync::require_history_retention().map_err(|_| "Quellenschreiben ist gesperrt, bis der Kernspeicher die Historie nach zwölf Monaten zuverlässig entfernt.")?;
    let secret_config: serde_json::Value = serde_json::from_reader(
        File::open(&config.infisical_config).map_err(|_| "Infisical-Konfiguration fehlt.")?,
    )
    .map_err(|_| "Infisical-Konfiguration ist ungültig.")?;
    if secret_config["credential_fd"] != 5 {
        return Err("Infisical benötigt Credential-FD 5.");
    }
    let secrets = infisical_environment(&config.infisical_config)
        .await
        .map_err(|_| "Infrastrukturzugang über Infisical fehlt.")?;
    let password = secrets
        .iter()
        .find(|(name, _)| name == "BRAIN_PG_INGEST_PASSWORD")
        .map(|(_, value)| value.as_str())
        .ok_or("Brain-Schreibzugang fehlt.")?;
    let target = pool(password, false).await?;
    let store = PgStore::new(target);
    store
        .check_core_schema()
        .await
        .map_err(|_| "Brain-Kernschema fehlt.")?;
    let base = store
        .snapshot(&config.base_release_id)
        .await
        .map_err(|_| "Konfiguriertes Basisrelease fehlt.")?
        .release;
    let hash = hex::encode(Sha256::digest(
        serde_json::to_vec(&base).map_err(|_| "Basisrelease ist ungültig.")?,
    ));
    if hash != config.base_release_sha256 {
        return Err("Basisrelease entspricht nicht der freigegebenen Konfiguration.");
    }
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Systemzeit fehlt.")?
        .as_secs() as i64;
    let mut batches = Vec::new();
    let mut youtube_metadata_checks = Vec::new();
    for source in &config.sheets {
        let bodies = tabs(source, config.http_timeout_seconds).await?;
        let previous = store
            .checkpoint(&source.source_id())
            .await
            .map_err(|_| "Tabellencheckpoint fehlt.")?;
        batches.push(
            sheet_core::prepare_batch(source, &bodies, epoch, previous.as_ref())
                .map_err(|_| "Tabellenbatch wurde abgewiesen.")?,
        );
    }
    if !config.youtube.is_empty() {
        let secret_name = metadata_secret_name(&config)?;
        let api_key = secrets
            .iter()
            .find(|(name, _)| name == secret_name)
            .map(|(_, value)| value.as_str())
            .filter(|key| !key.trim().is_empty())
            .ok_or("Konfigurierter YouTube-Metadatenzugang ist in Infisical nicht verfügbar.")?;
        let password = secrets
            .iter()
            .find(|(name, _)| name == "BRAIN_PG_READONLY_PASSWORD")
            .map(|(_, value)| value.as_str())
            .ok_or("YouTube-Lesezugang fehlt.")?;
        let readonly = pool(password, true).await?;
        for source in &config.youtube {
            let videos = youtube_core::read_existing(&readonly, source)
                .await
                .map_err(|_| "Vollständiger YouTube-Bestand fehlt.")?;
            let previous = store
                .checkpoint(&source.source_id())
                .await
                .map_err(|_| "YouTube-Checkpoint fehlt.")?;
            let verified = youtube_core::metadata::fetch_complete(
                source,
                &videos,
                previous.as_ref(),
                api_key,
                config.http_timeout_seconds,
            )
            .await
            .map_err(|_| {
                "Vollständige frische YouTube-Metadatenprüfung fehlgeschlagen. Kein Quellencommit."
            })?;
            youtube_metadata_checks
                .push(json!({"source_id":source.source_id(),"proof":verified.proof()}));
            batches.push(
                youtube_core::prepare_fresh_batch(source, &verified, epoch, previous.as_ref())
                    .map_err(|_| "YouTube-Batch wurde abgewiesen.")?,
            );
        }
        readonly.close().await;
    }
    let mut release =
        candidate_release(&base, &batches, epoch).map_err(|_| "Kandidatenrelease ist ungültig.")?;
    if batches.iter().all(|batch| batch.records.is_empty())
        && release.source_revisions == base.source_revisions
    {
        return Ok(
            json!({"status":"unverändert","base_release_id":base.release_id,"youtube_metadata_checks":youtube_metadata_checks,"activation_performed":false}),
        );
    }
    if let Ok(existing) = store.snapshot(&release.release_id).await {
        if existing.release.source_revisions != release.source_revisions
            || existing.release.patch != release.patch
            || existing.release.knowledge_version != release.knowledge_version
        {
            return Err("Kandidatenrelease kollidiert mit bestehendem Release.");
        }
        release = existing.release;
    }
    let mut leases = Vec::new();
    for batch in &batches {
        leases.push(
            store
                .claim(&batch.checkpoint.source_id, &config.owner, 60000)
                .await
                .map_err(|_| "Quellensperre fehlt.")?,
        );
    }
    let pairs: Vec<_> = batches.iter().zip(&leases).collect();
    let (receipts, documents) = store
        .commit_batches_and_publish(&pairs, &release)
        .await
        .map_err(|_| "Atomarer Quellencommit ist fehlgeschlagen.")?;
    let sources: Vec<_> = batches.iter().zip(receipts).map(|(batch,receipt)| json!({"source_id":batch.checkpoint.source_id,"generation":receipt.generation,"records":batch.records.len(),"replayed":receipt.replayed})).collect();
    let candidate_sha256 = hex::encode(Sha256::digest(
        serde_json::to_vec(&release).map_err(|_| "Kandidatenrelease ist ungültig.")?,
    ));
    Ok(
        json!({"status":"Kandidat gespeichert","base_release_id":base.release_id,"base_release_sha256":config.base_release_sha256,"candidate_release_id":release.release_id,"candidate_release_sha256":candidate_sha256,"knowledge_version":release.knowledge_version,"activation_target":"second_brain_internal","youtube_metadata_checks":youtube_metadata_checks,"sources":sources,"documents":documents,"activation_performed":false}),
    )
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = (|| {
        let [_, flag, path] = args.as_slice() else {
            return Err("Aufruf: brain-source-sync --config DATEI");
        };
        if flag != "--config" {
            return Err("Aufruf: brain-source-sync --config DATEI");
        }
        let config = read_config(Path::new(path))?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| "Rust-Laufzeit fehlt.")?;
        runtime.block_on(run(config))
    })();
    match result {
        Ok(value) => println!("{value}"),
        Err(message) => {
            eprintln!("brain-source-sync: {message}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(scopes: Vec<&str>, youtube: bool) -> serde_json::Value {
        let rights = json!({"ingestion_allowed":true,"authorization_ref":"fixture","license":null,"history_retention_months":12,
            "policy":{"visibility":"internal","allowed_scopes":scopes,"provider_egress_allowed":false,"publication_allowed":false,"raw_retention_allowed":true}});
        json!({"infisical_config":"/fixture/infisical.json","base_release_id":"explicit-internal-base","base_release_sha256":"a".repeat(64),"owner":"fixture",
            "sheets":[{"sheet_id":"fixture","gids":["0"],"rights":rights}],
            "youtube":if youtube { vec![json!({"channel_id":"UCabcdefghijklmnopqrstuv","retention":"metadata_only","rights":rights})] } else { vec![] },
            "youtube_metadata_api_key_secret":null})
    }

    fn parsed(value: serde_json::Value) -> Result<Config> {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), serde_json::to_vec(&value).unwrap()).unwrap();
        read_config(file.path())
    }

    #[test]
    fn writer_requires_internal_scope_and_explicit_metadata_credential_reference() {
        assert!(parsed(config(vec!["second_brain.internal"], false)).is_ok());
        assert!(parsed(config(vec!["docs.public"], false)).is_err());
        assert!(parsed(config(vec!["second_brain.internal", "docs.public"], false)).is_err());
        assert!(parsed(config(vec!["second_brain.internal"], true)).is_err());
        let mut value = config(vec!["second_brain.internal"], true);
        for denied in ["GEMINI_API_KEY", "", "lowercase_secret", "ABC\n"] {
            value["youtube_metadata_api_key_secret"] = json!(denied);
            assert!(parsed(value.clone()).is_err());
        }
        value["youtube_metadata_api_key_secret"] = json!("FIXTURE_METADATA_CREDENTIAL");
        assert!(parsed(value).is_ok());
    }
}
