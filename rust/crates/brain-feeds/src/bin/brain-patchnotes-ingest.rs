#![forbid(unsafe_code)]

use brain_contracts::{CorpusRelease, DocumentStorePort, PortError, SourceBatch, SourceVisibility};
use brain_feeds::{
    patchnotes::{parse_feed, prepare_batch, MAX_FEED_BYTES, PARSER_REVISION, SOURCE},
    FeedPolicy,
};
use brain_ingestion::document_set::{current_pins, DocumentSetCheckpoint};
use brain_storage::PgStore;
use dbrain_sources::external::sha256;
use reqwest::{header, Client, StatusCode, Url};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Row,
};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    net::IpAddr,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

const DEFAULT_URL: &str = "http://127.0.0.1:8791/v1/brain/patchnotes/v1";
const PROVIDER: &str = "deadlock-patchnotes-bot";
const API_KEY: &str = "BRAIN_FEED_API_KEY";
const APPLY_BLOCKED: &str = "Schreiben ist gesperrt: Der gemeinsame Laufzeit- und Aktivierungsvertrag mit dem Brain-Dienst ist noch nicht angebunden.";
const USAGE: &str = "Aufruf: brain-patchnotes-ingest preview --infisical-config <absoluter-pfad> [--url <loopback-feed-url>] | validate --feed-file <pfad> | stage-candidate --config <absoluter-pfad>. apply bleibt gesperrt; stage-candidate speichert einen Kandidaten ohne Aktivierung.";
const ACTIVATION_TARGET: &str = "standard";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CandidateConfig {
    infisical_config: PathBuf,
    url: String,
    base_release_id: String,
    base_release_sha256: String,
    owner: String,
    policy: FeedPolicy,
}

enum Input {
    Http { infisical_config: PathBuf, url: Url },
    File(PathBuf),
    Candidate(PathBuf),
}

fn bounded_identifier(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 512 && !value.chars().any(char::is_control)
}

fn validate_candidate_config(config: &CandidateConfig) -> Result<(), &'static str> {
    if !config.infisical_config.is_absolute()
        || !bounded_identifier(&config.base_release_id)
        || !bounded_identifier(&config.owner)
        || config.base_release_sha256.len() != 64
        || !config
            .base_release_sha256
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        || config.policy.visibility != SourceVisibility::Public
        || config.policy.allowed_scopes.is_empty()
        || config.policy.allowed_scopes.len() > 64
        || !config
            .policy
            .allowed_scopes
            .iter()
            .all(|scope| bounded_identifier(scope))
    {
        return Err(
            "Kandidatenbasis, Quellenverantwortlicher oder öffentliche Quellenrechte fehlen.",
        );
    }
    endpoint(&config.url)?;
    Ok(())
}

fn read_json_config(path: &Path) -> Result<Value, &'static str> {
    if !path.is_absolute() {
        return Err("Konfigurationspfad muss absolut sein.");
    }
    let file = File::open(path).map_err(|_| "Konfiguration ist nicht lesbar.")?;
    if !file
        .metadata()
        .map_err(|_| "Konfiguration ist nicht prüfbar.")?
        .is_file()
    {
        return Err("Konfiguration muss eine reguläre Datei sein.");
    }
    let mut bytes = Vec::new();
    file.take(65_537)
        .read_to_end(&mut bytes)
        .map_err(|_| "Konfiguration wurde unterbrochen.")?;
    if bytes.len() > 65_536 {
        return Err("Konfiguration überschreitet die erlaubte Größe.");
    }
    serde_json::from_slice(&bytes).map_err(|_| "Konfiguration ist ungültig.")
}

fn candidate_config(path: &Path) -> Result<CandidateConfig, &'static str> {
    let config: CandidateConfig = serde_json::from_value(read_json_config(path)?)
        .map_err(|_| "Kandidatenkonfiguration ist ungültig.")?;
    validate_candidate_config(&config)?;
    Ok(config)
}

fn require_fd_access(config: &Value) -> Result<(), &'static str> {
    if config.get("credential_fd").and_then(Value::as_i64) != Some(5) {
        return Err("Infisical-Zugang benötigt den bestehenden Dateideskriptor 5.");
    }
    Ok(())
}

async fn infrastructure(path: &Path) -> Result<Vec<(String, Zeroizing<String>)>, &'static str> {
    if [
        "INFISICAL_TOKEN_FILE",
        "CREDENTIALS_DIRECTORY",
        "PGPASSWORD",
        "DATABASE_URL",
        "DEADLOCK_CENTRAL_DSN",
        "PGOPTIONS",
        "PGSERVICE",
        "PGSERVICEFILE",
        "PGPASSFILE",
        "PGHOST",
        "PGHOSTADDR",
        "PGPORT",
        "PGDATABASE",
        "PGUSER",
        "PGSSLMODE",
        "PGSSLROOTCERT",
        "PGSSLCERT",
        "PGSSLKEY",
    ]
    .iter()
    .any(|name| std::env::var_os(name).is_some())
    {
        return Err("Dateibasierte oder geerbte Datenbankzugänge sind nicht zulässig.");
    }
    require_fd_access(&read_json_config(path)?)?;
    deadlock_brain_core::pg::infisical_environment(path)
        .await
        .map_err(|_| "Bestehender Infisical-Zugang konnte nicht geladen werden.")
}

fn endpoint(value: &str) -> Result<Url, &'static str> {
    let url = Url::parse(value).map_err(|_| "Feed-Adresse ist ungültig.")?;
    let loopback = url
        .host_str()
        .and_then(|host| host.trim_matches(['[', ']']).parse::<IpAddr>().ok())
        .is_some_and(|ip| ip.is_loopback());
    if !loopback
        || !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.port_or_known_default().is_none_or(|port| port == 0)
        || url.path() != "/v1/brain/patchnotes/v1"
    {
        return Err(
            "Feed-Adresse muss eine lokale IP-Adresse mit dem vorgesehenen Feed-Pfad verwenden.",
        );
    }
    Ok(url)
}

fn arguments(args: &[String]) -> Result<Input, &'static str> {
    if args.iter().any(|arg| arg == "--apply" || arg == "apply") {
        return Err(APPLY_BLOCKED);
    }
    match args {
        [action, flag, path] if action == "validate" && flag == "--feed-file" => {
            Ok(Input::File(PathBuf::from(path)))
        }
        [action, flag, path, rest @ ..] if action == "preview" && flag == "--infisical-config" => {
            let path = PathBuf::from(path);
            if !path.is_absolute() {
                return Err("Infisical-Konfigurationspfad muss absolut sein.");
            }
            let url = match rest {
                [] => endpoint(DEFAULT_URL)?,
                [flag, value] if flag == "--url" => endpoint(value)?,
                _ => return Err(USAGE),
            };
            Ok(Input::Http {
                infisical_config: path,
                url,
            })
        }
        [action, flag, path] if action == "stage-candidate" && flag == "--config" => {
            let path = PathBuf::from(path);
            if !path.is_absolute() {
                return Err("Konfigurationspfad muss absolut sein.");
            }
            Ok(Input::Candidate(path))
        }
        _ => Err(USAGE),
    }
}

fn read_feed_file(path: &Path) -> Result<Vec<u8>, &'static str> {
    if !std::fs::metadata(path)
        .map_err(|_| "Feed-Datei ist nicht prüfbar.")?
        .is_file()
    {
        return Err("Feed muss eine reguläre Datei sein.");
    }
    let file = File::open(path).map_err(|_| "Feed-Datei ist nicht lesbar.")?;
    if !file
        .metadata()
        .map_err(|_| "Feed-Datei ist nicht prüfbar.")?
        .is_file()
    {
        return Err("Feed muss eine reguläre Datei sein.");
    }
    let mut bytes = Vec::new();
    file.take(MAX_FEED_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Feed-Datei konnte nicht vollständig gelesen werden.")?;
    if bytes.len() > MAX_FEED_BYTES {
        return Err("Feed überschreitet die erlaubte Größe.");
    }
    Ok(bytes)
}

fn authorization(key: &str) -> Result<header::HeaderValue, &'static str> {
    if key.is_empty() || key.len() > 4096 || !key.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err("Feed-Zugang aus Infisical ist ungültig.");
    }
    let value = Zeroizing::new(format!("Bearer {key}"));
    let mut header = header::HeaderValue::from_str(&value)
        .map_err(|_| "Feed-Zugang aus Infisical ist ungültig.")?;
    header.set_sensitive(true);
    Ok(header)
}

fn http_client() -> Result<Client, &'static str> {
    Client::builder()
        .no_proxy()
        .no_gzip()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .user_agent("brain-patchnotes-ingest/1")
        .build()
        .map_err(|_| "HTTP-Client konnte nicht gestartet werden.")
}

async fn fetch_feed(client: &Client, url: Url, key: &str) -> Result<Vec<u8>, &'static str> {
    let url = endpoint(url.as_str())?;
    let reply = client
        .get(url)
        .header(header::AUTHORIZATION, authorization(key)?)
        .header(header::ACCEPT, "application/json")
        .header(header::ACCEPT_ENCODING, "identity")
        .send()
        .await
        .map_err(|_| "Feed-Abruf ist fehlgeschlagen oder hat die Zeitgrenze überschritten.")?;
    match reply.status() {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            return Err("Feed-Zugriff wurde abgelehnt.");
        }
        status if status.is_redirection() => return Err("Feed-Weiterleitung wurde abgelehnt."),
        _ => return Err("Feed-Dienst hat keinen vollständigen Export geliefert."),
    }
    let media_type = reply
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());
    if !media_type.is_some_and(|value| {
        value
            .split(';')
            .next()
            .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("application/json"))
    }) {
        return Err("Feed-Antwort hat nicht das erwartete JSON-Format.");
    }
    if reply
        .headers()
        .get(header::CONTENT_ENCODING)
        .is_some_and(|value| value != "identity")
    {
        return Err("Komprimierte Feed-Antwort wurde abgelehnt.");
    }
    if reply
        .content_length()
        .is_some_and(|length| length > MAX_FEED_BYTES as u64)
    {
        return Err("Feed überschreitet die erlaubte Größe.");
    }
    read_body(reply, MAX_FEED_BYTES).await
}

async fn read_body(mut reply: reqwest::Response, maximum: usize) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();
    while let Some(chunk) = reply
        .chunk()
        .await
        .map_err(|_| "Feed-Antwort wurde unterbrochen.")?
    {
        if chunk.len() > maximum.saturating_sub(bytes.len()) {
            return Err("Feed überschreitet die erlaubte Größe.");
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

fn validated_feed(bytes: &[u8]) -> Result<brain_contracts::feeds::PatchnotesFeedV1, &'static str> {
    let feed = parse_feed(bytes)
        .map_err(|_| "Feed ist leer, unvollständig oder verletzt den Patchnotes-Vertrag.")?;
    if feed.provider != PROVIDER {
        return Err("Feed stammt nicht vom vorgesehenen Patchnotes-Dienst.");
    }
    Ok(feed)
}

fn validate(bytes: &[u8], input: &str) -> Result<Value, &'static str> {
    let feed = validated_feed(bytes)?;
    let policy = FeedPolicy {
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["game.public".into()]),
        provider_egress_allowed: false,
        publication_allowed: false,
        raw_retention_allowed: true,
    };
    let batch = prepare_batch(&feed, &policy, None)
        .map_err(|_| "Feed konnte nicht für den bestehenden Brain-Import vorbereitet werden.")?;
    Ok(json!({
        "mode": "preview",
        "input": input,
        "validated": true,
        "contract_version": feed.contract_version,
        "provider": feed.provider,
        "source_id": SOURCE,
        "parser_revision": PARSER_REVISION,
        "export_revision": feed.export_revision,
        "exported_at": feed.exported_at,
        "feed_bytes": bytes.len(),
        "posts": feed.posts.len(),
        "prepared_records_without_checkpoint": batch.records.len(),
        "previous_checkpoint_loaded": false,
        "store_access": false,
        "applied": false,
        "activated": false,
        "apply_available": false,
        "apply_blocked_reason": APPLY_BLOCKED,
    }))
}

fn release_hash(release: &CorpusRelease) -> Result<String, &'static str> {
    Ok(sha256(
        &serde_json::to_vec(release).map_err(|_| "Release ist nicht serialisierbar.")?,
    ))
}

fn verify_base(base: &CorpusRelease, config: &CandidateConfig) -> Result<(), &'static str> {
    if base.release_id != config.base_release_id
        || release_hash(base)? != config.base_release_sha256
    {
        return Err("Basisrelease entspricht nicht dem freigegebenen Stand.");
    }
    Ok(())
}

fn prepare_candidate(
    base: &CorpusRelease,
    batch: &SourceBatch,
    epoch: i64,
) -> Result<Option<CorpusRelease>, &'static str> {
    batch.validate().map_err(|_| "Quellenbatch ist ungültig.")?;
    if batch.checkpoint.source_id != SOURCE || epoch <= 0 {
        return Err("Kandidatenquelle oder Systemzeit ist ungültig.");
    }
    let state: DocumentSetCheckpoint = serde_json::from_value(batch.checkpoint.state.clone())
        .map_err(|_| "Quellencheckpoint ist ungültig.")?;
    let pins = current_pins(&batch.checkpoint).map_err(|_| "Quellenrevisionen sind ungültig.")?;
    if pins.is_empty()
        || state.documents.iter().any(|(id, state)| {
            !bounded_identifier(id) || state.revision == 0 || state.revision > i64::MAX as u64
        })
        || base.source_revisions.get(SOURCE).is_some_and(|old| {
            old.iter().any(|(id, revision)| {
                state
                    .documents
                    .get(id)
                    .is_none_or(|document| document.revision < *revision)
            })
        })
    {
        return Err(
            "Quellencheckpoint ist leer, unvollständig oder älter als die gebundene Basis.",
        );
    }
    let mut proposed = base.source_revisions.clone();
    proposed.insert(SOURCE.into(), pins);
    if batch.records.is_empty() && proposed == base.source_revisions {
        return Ok(None);
    }
    if proposed == base.source_revisions {
        return Err("Geänderte Quellenrecords passen nicht zu den Basisrevisionen.");
    }
    let identity = sha256(
        &serde_json::to_vec(&(release_hash(base)?, &proposed))
            .map_err(|_| "Kandidatenrevisionen sind nicht serialisierbar.")?,
    );
    Ok(Some(CorpusRelease {
        release_id: format!("patchnotes-{identity}"),
        knowledge_version: format!("patchnotes-{identity}"),
        patch: base.patch.clone(),
        created_at_epoch: epoch,
        source_revisions: proposed,
    }))
}

fn reuse_candidate(
    proposed: CorpusRelease,
    existing: Option<CorpusRelease>,
) -> Result<CorpusRelease, &'static str> {
    match existing {
        None => Ok(proposed),
        Some(existing)
            if existing.release_id == proposed.release_id
                && existing.knowledge_version == proposed.knowledge_version
                && existing.patch == proposed.patch
                && existing.source_revisions == proposed.source_revisions =>
        {
            Ok(existing)
        }
        Some(_) => Err("Kandidatenkennung kollidiert mit einem anderen gespeicherten Stand."),
    }
}

fn candidate_report(
    config: &CandidateConfig,
    base: &CorpusRelease,
    candidate: Option<&CorpusRelease>,
    batch: &SourceBatch,
    receipt: &brain_contracts::BatchReceipt,
    documents: Option<usize>,
    reused: bool,
) -> Result<Value, &'static str> {
    Ok(json!({
        "mode": "stage-candidate",
        "status": if candidate.is_some() { "candidate_published" } else { "base_matches_feed" },
        "base_release_id": base.release_id,
        "base_release_sha256": config.base_release_sha256,
        "base_knowledge_version": base.knowledge_version,
        "candidate_release_id": candidate.map(|release| &release.release_id),
        "candidate_release_sha256": candidate.map(release_hash).transpose()?,
        "knowledge_version": candidate.map(|release| &release.knowledge_version),
        "candidate_status": if candidate.is_none() { "not_needed" } else if reused { "reused" } else { "published" },
        "allowed_changed_sources": [SOURCE],
        "sources": [{
            "source_id": SOURCE,
            "expected_generation": batch.expected_generation,
            "generation": receipt.generation,
            "replayed": receipt.replayed,
            "records": batch.records.len(),
            "tombstones": batch.records.iter().filter(|record| record.tombstone).count(),
            "live_documents": current_pins(&batch.checkpoint).map_err(|_| "Quellenrevisionen sind ungültig.")?.len(),
            "pins_sha256": sha256(&serde_json::to_vec(&current_pins(&batch.checkpoint).map_err(|_| "Quellenrevisionen sind ungültig.")?).map_err(|_| "Quellenrevisionen sind nicht serialisierbar.")?),
        }],
        "documents": documents,
        "activation_target": ACTIVATION_TARGET,
        "activation_performed": false,
        "reader_state": "not_checked",
        "base_binding": "provided_by_activation_adapter",
    }))
}

async fn stage_candidate(config: CandidateConfig) -> Result<Value, &'static str> {
    validate_candidate_config(&config)?;
    let secrets = infrastructure(&config.infisical_config).await?;
    let secret = |name: &str| {
        secrets
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    };
    let key = secret(API_KEY).ok_or("Feed-Zugang fehlt in Infisical.")?;
    let password = secret("BRAIN_PG_INGEST_PASSWORD")
        .filter(|value| !value.is_empty())
        .ok_or("Brain-Schreibzugang fehlt in Infisical.")?;
    let bytes = fetch_feed(&http_client()?, endpoint(&config.url)?, key).await?;
    let feed = validated_feed(&bytes)?;
    let initial = prepare_batch(&feed, &config.policy, None)
        .map_err(|_| "Feed konnte nicht für den bestehenden Brain-Import vorbereitet werden.")?;
    let options = PgConnectOptions::new_without_pgpass()
        .host("/run/deadlock-brain-postgresql")
        .port(5446)
        .database("brain")
        .username("brain_ingest")
        .password(password)
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-patchnotes-ingest");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| "Brain-Datenbankverbindung ist fehlgeschlagen.")?;
    let row = sqlx::query("SELECT current_database() AS database,current_user AS username,inet_server_addr()::text AS address,current_setting('port') AS port")
        .fetch_one(&pool).await.map_err(|_| "Datenbankidentität fehlt.")?;
    if row.try_get::<String, _>("database").ok().as_deref() != Some("brain")
        || row.try_get::<String, _>("username").ok().as_deref() != Some("brain_ingest")
        || row.try_get::<Option<String>, _>("address").ok() != Some(None)
        || row.try_get::<String, _>("port").ok().as_deref() != Some("5446")
    {
        return Err("Datenbankidentität ist nicht zulässig.");
    }
    let store = PgStore::new(pool);
    store
        .check_core_schema()
        .await
        .map_err(|_| "Brain-Kernschema fehlt.")?;
    let base = store
        .snapshot(&config.base_release_id)
        .await
        .map_err(|_| "Gebundenes Basisrelease ist nicht vollständig lesbar.")?
        .release;
    verify_base(&base, &config)?;
    let epoch = i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Systemzeit fehlt.")?
            .as_secs(),
    )
    .map_err(|_| "Systemzeit ist ungültig.")?;
    let lease = store
        .claim(SOURCE, &config.owner, 60_000)
        .await
        .map_err(|_| "Patchnotes-Quellensperre ist nicht verfügbar.")?;
    let previous = store
        .checkpoint(SOURCE)
        .await
        .map_err(|_| "Patchnotes-Checkpoint ist nicht lesbar.")?;
    let batch = match previous.as_ref() {
        None => initial,
        Some(checkpoint) => prepare_batch(&feed, &config.policy, Some(checkpoint))
            .map_err(|_| "Feed passt nicht zum bestehenden Quellencheckpoint.")?,
    };
    let Some(proposed) = prepare_candidate(&base, &batch, epoch)? else {
        let receipt = store
            .commit(&batch, &lease)
            .await
            .map_err(|_| "Unveränderter Quellencheckpoint konnte nicht bestätigt werden.")?;
        return candidate_report(&config, &base, None, &batch, &receipt, None, false);
    };
    let existing = match store.snapshot(&proposed.release_id).await {
        Ok(snapshot) => Some(snapshot.release),
        Err(PortError::InvalidResponse(message)) if message == "unknown release" => None,
        Err(_) => {
            return Err("Vorhandener Kandidatenstand konnte nicht zuverlässig geprüft werden.")
        }
    };
    let reused = existing.is_some();
    let release = reuse_candidate(proposed, existing)?;
    let (receipts, documents) = store
        .commit_batches_and_publish(&[(&batch, &lease)], &release)
        .await
        .map_err(|_| "Atomarer Quellencommit und Kandidatenpublikation sind fehlgeschlagen.")?;
    let [receipt] = receipts.as_slice() else {
        return Err("Bestätigung der Kandidatenpublikation ist unvollständig.");
    };
    candidate_report(
        &config,
        &base,
        Some(&release),
        &batch,
        receipt,
        Some(documents),
        reused,
    )
}

async fn run(input: Input) -> Result<Value, &'static str> {
    match input {
        Input::Candidate(path) => stage_candidate(candidate_config(&path)?).await,
        Input::File(path) => validate(&read_feed_file(&path)?, "file"),
        Input::Http {
            infisical_config,
            url,
        } => {
            let values = infrastructure(&infisical_config).await?;
            let key = values
                .into_iter()
                .find_map(|(name, value)| (name == API_KEY).then_some(value))
                .ok_or("Feed-Zugang fehlt in Infisical.")?;
            let bytes = fetch_feed(&http_client()?, url, &key).await?;
            validate(&bytes, "authenticated_http")
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        println!("{USAGE}");
        return;
    }
    let result = arguments(&args).and_then(|input| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| "Laufzeit konnte nicht gestartet werden.")?;
        runtime.block_on(run(input))
    });
    match result {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("brain-patchnotes-ingest: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
    };

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/patchnotes_feed.json");

    fn server(response: String) -> (Url, thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = endpoint(&format!(
            "http://{}/v1/brain/patchnotes/v1",
            listener.local_addr().unwrap()
        ))
        .unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request = String::new();
            loop {
                let mut line = String::new();
                assert!(reader.read_line(&mut line).unwrap() > 0);
                request.push_str(&line);
                if line == "\r\n" {
                    break;
                }
            }
            let _ = stream.write_all(response.as_bytes());
            request
        });
        (url, handle)
    }

    fn response(body: &[u8]) -> String {
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), std::str::from_utf8(body).unwrap())
    }

    #[test]
    fn endpoints_require_literal_loopback_and_contract_path() {
        assert!(endpoint(DEFAULT_URL).is_ok());
        assert!(endpoint("http://[::1]:8791/v1/brain/patchnotes/v1").is_ok());
        for url in [
            "http://localhost:8791/v1/brain/patchnotes/v1",
            "https://example.invalid/v1/brain/patchnotes/v1",
            "http://127.0.0.1:0/v1/brain/patchnotes/v1",
            "http://secret@127.0.0.1:8791/v1/brain/patchnotes/v1",
            "http://127.0.0.1:8791/v1/brain/patchnotes/v1?key=x",
            "http://127.0.0.1:8791/v1/brain/patchnotes/v1#x",
            "http://127.0.0.1:8791/healthz",
        ] {
            assert!(endpoint(url).is_err(), "{url}");
        }
    }

    #[test]
    fn apply_is_closed_before_configuration_or_network_access() {
        for args in [
            vec!["apply"],
            vec!["preview", "--apply"],
            vec!["validate", "--feed-file", "missing", "--apply"],
        ] {
            assert!(matches!(
                arguments(&args.into_iter().map(String::from).collect::<Vec<_>>()),
                Err(APPLY_BLOCKED)
            ));
        }
        assert!(arguments(&[
            "preview".into(),
            "--infisical-config".into(),
            "relative.json".into()
        ])
        .is_err());
        assert!(arguments(&[
            "preview".into(),
            "--infisical-config".into(),
            "/config/infisical.json".into()
        ])
        .is_ok());
    }

    #[test]
    fn validation_reuses_parser_and_batch_without_claiming_store_changes() {
        let report = validate(FIXTURE, "file").unwrap();
        assert_eq!(report["posts"], 2);
        assert_eq!(report["prepared_records_without_checkpoint"], 2);
        assert_eq!(report["store_access"], false);
        assert_eq!(report["activated"], false);
        assert_eq!(report["previous_checkpoint_loaded"], false);
        assert_eq!(report["apply_available"], false);
    }

    #[test]
    fn invalid_empty_tampered_or_foreign_feed_fails_closed() {
        assert!(validate(b"{}", "file").is_err());
        assert!(validate(&FIXTURE[..FIXTURE.len() / 2], "file").is_err());
        let original: Value = serde_json::from_slice(FIXTURE).unwrap();
        for (field, value) in [
            ("posts", json!([])),
            ("provider", json!("other")),
            ("contract_version", json!("brain.feed.patchnotes.v2")),
        ] {
            let mut feed = original.clone();
            feed[field] = value;
            assert!(validate(&serde_json::to_vec(&feed).unwrap(), "file").is_err());
        }
        let mut feed = original;
        feed["posts"][0]["raw_text"] = json!("tampered");
        assert!(validate(&serde_json::to_vec(&feed).unwrap(), "file").is_err());
    }

    #[tokio::test]
    async fn http_fetch_authenticates_and_validates_real_contract_fixture() {
        let (url, handle) = server(response(FIXTURE));
        let bytes = fetch_feed(&http_client().unwrap(), url, "synthetic-fixture-key")
            .await
            .unwrap();
        let request = handle.join().unwrap().to_ascii_lowercase();
        assert!(request.starts_with("get /v1/brain/patchnotes/v1 "));
        assert!(request.contains("authorization: bearer synthetic-fixture-key\r\n"));
        assert!(request.contains("accept-encoding: identity\r\n"));
        assert_eq!(validate(&bytes, "authenticated_http").unwrap()["posts"], 2);
    }

    #[tokio::test]
    async fn http_rejects_redirect_partial_unauthorized_and_declared_oversize() {
        for reply in [
            "HTTP/1.1 302 Found\r\nLocation: http://example.invalid/\r\nContent-Length: 0\r\n\r\n"
                .into(),
            "HTTP/1.1 206 Partial Content\r\nContent-Length: 0\r\n\r\n".into(),
            "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\n\r\n".into(),
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
                MAX_FEED_BYTES + 1
            ),
        ] {
            let (url, handle) = server(reply);
            assert!(
                fetch_feed(&http_client().unwrap(), url, "synthetic-fixture-key")
                    .await
                    .is_err()
            );
            handle.join().unwrap();
        }
    }

    #[tokio::test]
    async fn streamed_body_limit_and_truncation_fail_closed() {
        let (url, handle) = server("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n5\r\n12345\r\n0\r\n\r\n".into());
        let reply = http_client().unwrap().get(url).send().await.unwrap();
        assert!(read_body(reply, 4).await.is_err());
        handle.join().unwrap();
        let (url, handle) = server("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 100\r\nConnection: close\r\n\r\n{}".into());
        assert!(
            fetch_feed(&http_client().unwrap(), url, "synthetic-fixture-key")
                .await
                .is_err()
        );
        handle.join().unwrap();
    }

    fn base() -> CorpusRelease {
        CorpusRelease {
            release_id: "fixture-base".into(),
            knowledge_version: "fixture-knowledge".into(),
            patch: "fixture-patch".into(),
            created_at_epoch: 1_700_000_000,
            source_revisions: std::collections::BTreeMap::from([
                (
                    "wiki".into(),
                    std::collections::BTreeMap::from([("hero/a".into(), 7)]),
                ),
                (
                    "builds".into(),
                    std::collections::BTreeMap::from([("build/a".into(), 3)]),
                ),
            ]),
        }
    }

    fn config_value(base: &CorpusRelease) -> Value {
        json!({
            "infisical_config": "/fixture/infisical.json",
            "url": DEFAULT_URL,
            "base_release_id": base.release_id,
            "base_release_sha256": release_hash(base).unwrap(),
            "owner": "fixture-owner",
            "policy": {
                "visibility": "public",
                "allowed_scopes": ["fixture.public"],
                "provider_egress_allowed": false,
                "publication_allowed": false,
                "raw_retention_allowed": true,
            },
        })
    }

    fn config(value: Value) -> Result<CandidateConfig, &'static str> {
        let config = serde_json::from_value(value).map_err(|_| "fixture invalid")?;
        validate_candidate_config(&config)?;
        Ok(config)
    }

    #[test]
    fn candidate_mode_is_explicit_and_requires_absolute_configuration() {
        assert!(matches!(
            arguments(&[
                "stage-candidate".into(),
                "--config".into(),
                "/fixture/config.json".into()
            ]),
            Ok(Input::Candidate(_))
        ));
        assert!(arguments(&[
            "stage-candidate".into(),
            "--config".into(),
            "relative.json".into()
        ])
        .is_err());
        assert!(arguments(&["stage-candidate".into()]).is_err());
        assert!(matches!(
            arguments(&[
                "stage-candidate".into(),
                "--config".into(),
                "/missing".into(),
                "--apply".into()
            ]),
            Err(APPLY_BLOCKED)
        ));
    }

    #[test]
    fn candidate_binding_never_defaults_base_owner_or_scope() {
        let original = config_value(&base());
        assert!(config(original.clone()).is_ok());
        for field in [
            "base_release_id",
            "base_release_sha256",
            "owner",
            "policy",
            "url",
            "infisical_config",
        ] {
            let mut value = original.clone();
            value.as_object_mut().unwrap().remove(field);
            assert!(config(value).is_err(), "{field}");
        }
        for (field, invalid) in [
            ("base_release_id", json!("")),
            ("owner", json!("\n")),
            ("base_release_sha256", json!("G".repeat(64))),
            ("base_release_sha256", json!("a".repeat(63))),
            ("infisical_config", json!("relative")),
            (
                "url",
                json!("http://example.invalid/v1/brain/patchnotes/v1"),
            ),
        ] {
            let mut value = original.clone();
            value[field] = invalid;
            assert!(config(value).is_err(), "{field}");
        }
        for (field, invalid) in [
            ("visibility", json!("internal")),
            ("allowed_scopes", json!([])),
            ("allowed_scopes", json!(["\n"])),
        ] {
            let mut value = original.clone();
            value["policy"][field] = invalid;
            assert!(config(value).is_err(), "{field}");
        }
        let mut value = original;
        value["writer_dsn"] = json!("forbidden");
        assert!(config(value).is_err());
    }

    #[test]
    fn infrastructure_requires_exact_fd_five() {
        assert!(require_fd_access(&json!({"credential_fd":5})).is_ok());
        for value in [
            json!({}),
            json!({"credential_fd":4}),
            json!({"credential_fd":"5"}),
            json!({"credential_fd":null}),
        ] {
            assert!(require_fd_access(&value).is_err());
        }
    }

    #[test]
    fn supplied_base_hash_covers_full_serialized_release() {
        let base = base();
        let config = config(config_value(&base)).unwrap();
        verify_base(&base, &config).unwrap();
        let mut changed = base.clone();
        changed.created_at_epoch += 1;
        assert!(verify_base(&changed, &config).is_err());
        changed = base.clone();
        changed
            .source_revisions
            .get_mut("wiki")
            .unwrap()
            .insert("hero/a".into(), 8);
        assert!(verify_base(&changed, &config).is_err());
        changed = base;
        changed.release_id = "wrong-base".into();
        assert!(verify_base(&changed, &config).is_err());
    }

    #[test]
    fn candidate_preserves_foreign_pins_patch_and_uses_explicit_policy() {
        let base = base();
        let config = config(config_value(&base)).unwrap();
        let batch = prepare_batch(&validated_feed(FIXTURE).unwrap(), &config.policy, None).unwrap();
        assert!(batch
            .records
            .iter()
            .all(|record| record.allowed_scopes == BTreeSet::from(["fixture.public".into()])));
        let release = prepare_candidate(&base, &batch, 1_790_000_000)
            .unwrap()
            .unwrap();
        assert_ne!(release.release_id, base.release_id);
        assert_ne!(release.knowledge_version, base.knowledge_version);
        assert_eq!(release.patch, base.patch);
        for source in ["wiki", "builds"] {
            assert_eq!(
                release.source_revisions[source],
                base.source_revisions[source]
            );
        }
        assert_eq!(
            release.source_revisions[SOURCE],
            current_pins(&batch.checkpoint).unwrap()
        );
        assert_eq!(release.source_revisions.len(), 3);
    }

    #[tokio::test]
    async fn committed_but_unactivated_checkpoint_still_produces_pending_candidate() {
        let repo = brain_storage::MemoryRepository::default();
        let base = base();
        let config = config(config_value(&base)).unwrap();
        let feed = validated_feed(FIXTURE).unwrap();
        let batch = prepare_batch(&feed, &config.policy, None).unwrap();
        let candidate = prepare_candidate(&base, &batch, 1_790_000_000)
            .unwrap()
            .unwrap();
        let lease = repo.claim(SOURCE, "fixture", 60_000).await.unwrap();
        let receipt = repo.commit(&batch, &lease).await.unwrap();
        assert!(repo.commit(&batch, &lease).await.unwrap().replayed);
        let checkpoint = repo.checkpoint(SOURCE).await.unwrap().unwrap();
        let retry_batch = prepare_batch(&feed, &config.policy, Some(&checkpoint)).unwrap();
        assert!(retry_batch.records.is_empty());
        let proposed = prepare_candidate(&base, &retry_batch, 1_790_000_001)
            .unwrap()
            .unwrap();
        assert_eq!(proposed.release_id, candidate.release_id);
        let recovered = reuse_candidate(proposed, Some(candidate.clone())).unwrap();
        assert_eq!(recovered, candidate);
        let report = candidate_report(
            &config,
            &base,
            Some(&recovered),
            &retry_batch,
            &receipt,
            Some(4),
            true,
        )
        .unwrap();
        assert_eq!(report["candidate_status"], "reused");
        assert_eq!(report["activation_target"], "standard");
        assert_eq!(report["activation_performed"], false);
        assert_eq!(report["reader_state"], "not_checked");
        assert_eq!(
            report["candidate_release_sha256"],
            release_hash(&candidate).unwrap()
        );
        assert!(prepare_candidate(&candidate, &retry_batch, 1_790_000_002)
            .unwrap()
            .is_none());
        let active_config = config_value(&candidate);
        let active_config = super::tests::config(active_config).unwrap();
        let no_change = candidate_report(
            &active_config,
            &candidate,
            None,
            &retry_batch,
            &receipt,
            None,
            false,
        )
        .unwrap();
        assert_eq!(no_change["status"], "base_matches_feed");
        assert!(no_change["candidate_release_id"].is_null());
        assert!(no_change["candidate_release_sha256"].is_null());
        assert_eq!(no_change["candidate_status"], "not_needed");
        assert_eq!(no_change["activation_performed"], false);
    }

    #[test]
    fn removal_uses_tombstone_checkpoint_pins_and_preserves_other_sources() {
        let base = base();
        let config = config(config_value(&base)).unwrap();
        let mut feed = validated_feed(FIXTURE).unwrap();
        let first = prepare_batch(&feed, &config.policy, None).unwrap();
        let base = prepare_candidate(&base, &first, 1_790_000_000)
            .unwrap()
            .unwrap();
        let removed = feed.posts.pop().unwrap();
        feed.export_revision = "fixture-removal".into();
        let batch = prepare_batch(&feed, &config.policy, Some(&first.checkpoint)).unwrap();
        let tombstone = batch
            .records
            .iter()
            .find(|record| record.tombstone)
            .unwrap();
        assert_eq!(tombstone.logical_id, format!("post/{}", removed.post_id));
        let next = prepare_candidate(&base, &batch, 1_790_000_001)
            .unwrap()
            .unwrap();
        assert_eq!(next.source_revisions[SOURCE].len(), 1);
        assert!(!next.source_revisions[SOURCE].contains_key(&tombstone.logical_id));
        assert_eq!(next.source_revisions["wiki"], base.source_revisions["wiki"]);
        assert_eq!(
            next.source_revisions["builds"],
            base.source_revisions["builds"]
        );
        let retry = prepare_batch(&feed, &config.policy, Some(&batch.checkpoint)).unwrap();
        assert!(retry.records.is_empty());
        assert_eq!(
            prepare_candidate(&base, &retry, 1_790_000_002)
                .unwrap()
                .unwrap()
                .release_id,
            next.release_id
        );
    }

    #[test]
    fn inconsistent_checkpoints_and_candidate_collisions_fail_closed() {
        let base = base();
        let config = config(config_value(&base)).unwrap();
        let batch = prepare_batch(&validated_feed(FIXTURE).unwrap(), &config.policy, None).unwrap();
        let proposed = prepare_candidate(&base, &batch, 1_790_000_000)
            .unwrap()
            .unwrap();
        let mut wrong = proposed.clone();
        wrong
            .source_revisions
            .get_mut("wiki")
            .unwrap()
            .insert("hero/a".into(), 8);
        assert!(reuse_candidate(proposed.clone(), Some(wrong)).is_err());
        let mut ahead = base.clone();
        ahead.source_revisions.insert(
            SOURCE.into(),
            std::collections::BTreeMap::from([("post/missing".into(), 1)]),
        );
        assert!(prepare_candidate(&ahead, &batch, 1_790_000_001).is_err());
        ahead.source_revisions.insert(
            SOURCE.into(),
            current_pins(&batch.checkpoint)
                .unwrap()
                .into_iter()
                .map(|(id, rev)| (id, rev + 1))
                .collect(),
        );
        assert!(prepare_candidate(&ahead, &batch, 1_790_000_001).is_err());
        let mut wrong_source = batch.clone();
        wrong_source.checkpoint.source_id = "other".into();
        assert!(prepare_candidate(&base, &wrong_source, 1_790_000_001).is_err());
        assert!(prepare_candidate(&base, &batch, 0).is_err());
    }

    #[test]
    fn authorization_is_sensitive_and_rejects_invalid_values() {
        assert!(authorization("synthetic-fixture-key")
            .unwrap()
            .is_sensitive());
        for key in ["", "x\r\ny", "has space", "ä"] {
            assert!(authorization(key).is_err());
        }
        assert!(authorization(&"x".repeat(4097)).is_err());
    }
}
