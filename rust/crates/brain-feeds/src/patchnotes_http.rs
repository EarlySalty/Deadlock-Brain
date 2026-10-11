use crate::patchnotes::{parse_feed, MAX_FEED_BYTES};
use reqwest::{header, Client, StatusCode, Url};
use std::{net::IpAddr, time::Duration};
use zeroize::Zeroizing;

pub const DEFAULT_URL: &str = "http://127.0.0.1:8791/v1/brain/patchnotes/v1";
const PROVIDER: &str = "deadlock-patchnotes-bot";

pub fn endpoint(value: &str) -> Result<Url, &'static str> {
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

pub fn authorization(key: &str) -> Result<header::HeaderValue, &'static str> {
    if key.is_empty() || key.len() > 4096 || !key.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err("Feed-Zugang aus Infisical ist ungültig.");
    }
    let value = Zeroizing::new(format!("Bearer {key}"));
    let mut header = header::HeaderValue::from_str(&value)
        .map_err(|_| "Feed-Zugang aus Infisical ist ungültig.")?;
    header.set_sensitive(true);
    Ok(header)
}

pub fn http_client() -> Result<Client, &'static str> {
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

pub async fn fetch_feed(client: &Client, url: Url, key: &str) -> Result<Vec<u8>, &'static str> {
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

pub async fn read_body(
    mut reply: reqwest::Response,
    maximum: usize,
) -> Result<Vec<u8>, &'static str> {
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

pub fn validated_feed(
    bytes: &[u8],
) -> Result<brain_contracts::feeds::PatchnotesFeedV1, &'static str> {
    let feed = parse_feed(bytes)
        .map_err(|_| "Feed ist leer, unvollständig oder verletzt den Patchnotes-Vertrag.")?;
    if feed.provider != PROVIDER {
        return Err("Feed stammt nicht vom vorgesehenen Patchnotes-Dienst.");
    }
    Ok(feed)
}
