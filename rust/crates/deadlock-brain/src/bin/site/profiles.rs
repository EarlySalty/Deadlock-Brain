use anyhow::{ensure, Context, Result};
use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
};
use brain_contracts::entity_profile::EntityKind;
use brain_maintenance::entity_profile_render::render_entity_profile;
use brain_storage::PgStore;
use serde::Deserialize;
use std::path::Path as FilePath;

use super::Site;

#[derive(Deserialize)]
struct ServeBinding {
    release: ReleaseBinding,
}

#[derive(Deserialize)]
struct ReleaseBinding {
    id: String,
}

pub(super) async fn release_from_config(path: &FilePath) -> Result<String> {
    ensure!(
        path.is_absolute(),
        "Die Dienstkonfiguration muss absolut sein."
    );
    let config: ServeBinding = serde_json::from_slice(&tokio::fs::read(path).await?)
        .context("Der Quellenstand der Brain-Site fehlt.")?;
    ensure!(
        !config.release.id.is_empty() && config.release.id.len() <= 160,
        "Der Quellenstand ist ungültig."
    );
    Ok(config.release.id)
}

async fn current_release(site: &Site) -> Result<String> {
    let path = site
        .serve_config
        .as_deref()
        .context("Der Quellenstand der Brain-Site fehlt.")?;
    release_from_config(path).await
}

fn kind(value: EntityKind) -> &'static str {
    match value {
        EntityKind::Hero => "hero",
        EntityKind::Ability => "ability",
        EntityKind::Item => "item",
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn response(status: StatusCode, html: String, method: Method) -> Response {
    let length = html.len().to_string();
    (
        status,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8".to_owned()),
            (header::CACHE_CONTROL, "no-store".to_owned()),
            (header::CONTENT_LENGTH, length),
        ],
        if method == Method::HEAD {
            String::new()
        } else {
            html
        },
    )
        .into_response()
}

fn unavailable(method: Method) -> Response {
    response(
        StatusCode::SERVICE_UNAVAILABLE,
        "Die Steckbriefe sind gerade nicht verfügbar. <a href=\"/brain/site/\">Zur Brain-Site</a>"
            .into(),
        method,
    )
}

pub(super) async fn page(
    State(site): State<Site>,
    Path((requested_kind, id)): Path<(String, String)>,
    method: Method,
) -> Response {
    let Some(key) = decode_id(&id) else {
        return response(
            StatusCode::NOT_FOUND,
            "Dieser Steckbrief ist nicht verfügbar.".into(),
            method,
        );
    };
    if !matches!(requested_kind.as_str(), "hero" | "ability" | "item") {
        return response(
            StatusCode::NOT_FOUND,
            "Dieser Steckbrief ist nicht verfügbar.".into(),
            method,
        );
    }
    let Ok(release) = current_release(&site).await else {
        return unavailable(method);
    };
    let store = PgStore::new(site.pool);
    let result: Result<Option<String>> = async {
        let profiles = store.read_site_profiles(&release, Some(&key)).await?;
        match profiles.as_slice() {
            [profile] if kind(profile.entity.kind) == requested_kind => {
                Ok(Some(render_entity_profile(profile)?.public_html))
            }
            _ => Ok(None),
        }
    }
    .await;
    match result {
        Ok(Some(html)) => response(StatusCode::OK, html, method),
        Ok(None) => response(
            StatusCode::NOT_FOUND,
            "Dieser Steckbrief ist nicht verfügbar.".into(),
            method,
        ),
        Err(_) => unavailable(method),
    }
}

fn decode_id(id: &str) -> Option<String> {
    if id.is_empty()
        || id.len() > 200
        || !id.len().is_multiple_of(2)
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return None;
    }
    let key = String::from_utf8(hex::decode(id).ok()?).ok()?;
    (!key.chars().any(char::is_control)).then_some(key)
}

pub(super) async fn index(State(site): State<Site>, method: Method) -> Response {
    let Ok(release) = current_release(&site).await else {
        return unavailable(method);
    };
    let result: Result<String> = async {
        let profiles = PgStore::new(site.pool).read_site_profiles(&release, None).await?;
        let mut html = String::from("<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>Steckbriefe · Deadlock</title></head><body><main><nav><a href=\"/brain/site/\">Zur Brain-Site</a></nav><h1>Steckbriefe</h1><p>Helden, Fähigkeiten und Items mit belegten Werten.</p><ul>");
        for profile in &profiles {
            render_entity_profile(profile)?;
            html.push_str(&format!("<li><a href=\"/brain/site/steckbriefe/{}/{}\">{}</a></li>", kind(profile.entity.kind), hex::encode(profile.entity.entity_key.as_bytes()), escape(&profile.entity.name)));
        }
        html.push_str("</ul>");
        if profiles.is_empty() {
            html.push_str("<p>Für diesen Datenstand sind noch keine öffentlichen Steckbriefe verfügbar.</p>");
        }
        html.push_str("<p>Ein unbekannter Patchstand bleibt als unbekannt gekennzeichnet.</p></main></body></html>");
        Ok(html)
    }.await;
    match result {
        Ok(html) => response(StatusCode::OK, html, method),
        Err(_) => unavailable(method),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_ids_are_fixed_encoded_game_keys() {
        assert_eq!(
            decode_id(&hex::encode("hero_haze")),
            Some("hero_haze".into())
        );
        for id in ["", "a", "AA", "2f../", "00", "ff"] {
            assert_eq!(decode_id(id), None, "{id}");
        }
        assert_eq!(escape("<script>\"&"), "&lt;script&gt;&quot;&amp;");
    }
}
