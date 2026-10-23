use anyhow::{ensure, Context, Result};
use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
};
use brain_contracts::canonical_receipt::{format_receipt, CanonicalReceipt};
use serde_json::Value;
use sqlx::{PgConnection, Row};
use std::collections::BTreeSet;

use super::{hero, Site};

pub(super) fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub(super) fn document(title: &str, body: &str) -> String {
    format!("<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{} · Deadlock</title><style>body{{overflow-wrap:anywhere;font:1rem/1.6 system-ui,sans-serif;color:#20252b;background:#f7f5ef;margin:0}}main{{max-width:70rem;margin:auto;padding:clamp(1rem,4vw,3rem)}}a{{color:#254a80}}table{{border-collapse:collapse;width:100%}}th,td{{padding:.7rem;text-align:left;border-bottom:1px solid #bfc3c8;vertical-align:top;overflow-wrap:anywhere}}.scroll{{overflow-x:auto}}code,pre{{white-space:pre-wrap;overflow-wrap:anywhere}}li{{margin:.5rem 0}}article{{margin:2rem 0}}:focus-visible{{outline:3px solid #254a80;outline-offset:3px}}</style></head><body><main><nav><a href=\"/brain/site/steckbriefe\">Steckbriefe</a></nav>{body}</main></body></html>", escape(title))
}

pub(super) fn response(status: StatusCode, html: String, method: Method) -> Response {
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

pub(super) fn missing(method: Method) -> Response {
    response(StatusCode::NOT_FOUND, document("Seite nicht verfügbar", "<h1>Diese Seite ist nicht verfügbar.</h1><p>Die aktuellen belegten Werte findest du in den Steckbriefen.</p>"), method)
}

pub(super) fn unavailable(method: Method) -> Response {
    response(StatusCode::SERVICE_UNAVAILABLE, document("Gerade nicht verfügbar", "<h1>Die Spielwerte sind gerade nicht verfügbar.</h1><p>Bitte versuche es später erneut.</p>"), method)
}

pub(super) async fn transaction(site: &Site) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
    let mut tx = site.pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SET LOCAL statement_timeout='500ms'")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

pub(super) fn decode_id(id: &str) -> Option<String> {
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

pub(super) fn citation_ids(value: &Value, ids: &mut BTreeSet<i64>) -> Result<()> {
    match value {
        Value::Object(fields) => {
            if let Some(citation) = fields.get("citation") {
                let id = citation
                    .as_str()
                    .and_then(|id| id.parse::<i64>().ok())
                    .filter(|id| *id > 0)
                    .context("invalid_citation")?;
                ids.insert(id);
            }
            for field in fields.values() {
                citation_ids(field, ids)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                citation_ids(value, ids)?;
            }
        }
        _ => {}
    }
    ensure!(ids.len() <= 512, "citation_limit");
    Ok(())
}

pub(super) async fn receipts(connection: &mut PgConnection, value: &Value) -> Result<String> {
    let mut ids = BTreeSet::new();
    citation_ids(value, &mut ids)?;
    ensure!(!ids.is_empty(), "citations_missing");
    let rows = sqlx::query("SELECT id,raw_object_id,ingest_run_id,source,source_revision,path,locator_kind,locator,encode(value_sha256,'hex') AS value_sha256,extractor FROM serve.published_receipt WHERE id=ANY($1::bigint[]) ORDER BY id LIMIT 512")
        .bind(ids.iter().copied().collect::<Vec<_>>()).fetch_all(connection).await?;
    ensure!(rows.len() == ids.len(), "unpublished_citation");
    let mut html = String::from("<h2 id=\"quellen\">Quellen</h2><ol>");
    for row in rows {
        let id: i64 = row.try_get("id")?;
        let receipt = CanonicalReceipt {
            raw_receipt_id: id,
            raw_object_id: row.try_get("raw_object_id")?,
            ingest_run_id: row.try_get("ingest_run_id")?,
            source: row.try_get("source")?,
            source_revision: row.try_get("source_revision")?,
            path: row.try_get("path")?,
            locator_kind: row.try_get("locator_kind")?,
            locator: row.try_get("locator")?,
            value_sha256: row.try_get("value_sha256")?,
            extractor: row.try_get("extractor")?,
            patch: None,
        };
        let evidence = format_receipt(&receipt)?;
        html.push_str(&format!(
            "<li id=\"beleg-{id}\"><code>{}</code><p>{}</p></li>",
            escape(&evidence.evidence_id),
            escape(&evidence.content)
        ));
    }
    html.push_str("</ol>");
    Ok(html)
}

fn key_shaped(name: &str) -> bool {
    name.contains('_')
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

pub(super) fn display_name(identity: &Value) -> Option<&str> {
    let key = identity["key"].as_str().unwrap_or("");
    ["name_de", "name_en"]
        .iter()
        .filter_map(|field| identity[*field].as_str())
        .map(str::trim)
        .find(|name| !name.is_empty() && *name != key && !key_shaped(name))
}

pub(super) fn name(identity: &Value) -> &str {
    display_name(identity).unwrap_or("Unbekannt")
}

pub(super) fn fact_rows(section: &Value) -> String {
    let mut html = String::from("<div class=\"scroll\"><table><thead><tr><th scope=\"col\">Wert</th><th scope=\"col\">Angabe</th><th scope=\"col\">Bedingung und Beleg</th></tr></thead><tbody>");
    let rows = section["facts"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    for fact in rows {
        let label = fact["label"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| fact["stat_key"].as_str())
            .unwrap_or("Wert");
        let value = fact["value"]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| fact["value"].to_string());
        let citation = fact["citation"].as_str().unwrap_or("");
        let unit = fact["unit"]
            .as_str()
            .filter(|unit| !unit.is_empty())
            .map(|unit| format!(" {unit}"))
            .unwrap_or_default();
        html.push_str(&format!("<tr><th scope=\"row\">{}</th><td>{}{}</td><td><code>{}</code> <a href=\"#beleg-{}\">Beleg {}</a>", escape(label), escape(&value), escape(&unit), escape(&fact["condition"].to_string()), escape(citation), escape(citation)));
        if let Some(qualifiers) = fact["qualifiers"].as_array() {
            for qualifier in qualifiers {
                html.push_str(&format!(
                    "<p><code>{}: {}</code> <a href=\"#beleg-{}\">Beleg</a></p>",
                    escape(qualifier["key"].as_str().unwrap_or("Bedingung")),
                    escape(&qualifier["value"].to_string()),
                    escape(qualifier["citation"].as_str().unwrap_or(""))
                ));
            }
        }
        html.push_str("</td></tr>");
    }
    html.push_str("</tbody></table></div>");
    if rows.is_empty() {
        html.push_str("<p>Für diesen Abschnitt sind keine belegten Werte verfügbar.</p>");
    }
    if section["truncated"] == true {
        html.push_str("<p>Die Ausgabe ist gekürzt. Weitere Werte werden hier nicht angezeigt.</p>");
    }
    html
}

async fn load_profile(
    connection: &mut PgConnection,
    kind: &str,
    key: &str,
) -> Result<Option<Value>> {
    let sql = match kind {
        "hero" => "SELECT p.profile FROM (SELECT h.entity_id,jsonb_build_object('identity',h.identity,'stats',h.stats,'weapon',h.weapon) AS profile FROM serve.hero_profile h WHERE h.entity_id=(SELECT serve.site_entity_id($1)) OFFSET 0) p WHERE serve.hero_is_playable(p.entity_id) LIMIT 1",
        "item" => "SELECT card FROM serve.site_entity_card($1) WHERE kind='item' LIMIT 1",
        _ => return Ok(None),
    };
    Ok(sqlx::query_scalar(sql)
        .bind(key)
        .fetch_optional(connection)
        .await?)
}

enum Outcome {
    Page(String),
    Redirect(String),
    Missing,
}

const INTRO: &str =
    "<p>Aktuell beobachteter Quellenstand. Der offizielle Patchstand ist unbekannt.</p>";

async fn hero_page(connection: &mut PgConnection, key: &str) -> Result<Outcome> {
    let Some(profile) = load_profile(connection, "hero", key).await? else {
        return Ok(Outcome::Missing);
    };
    let Some(title) = display_name(&profile["identity"]) else {
        return Ok(Outcome::Missing);
    };
    let abilities = hero::abilities(connection, key).await?;
    let changes = hero::changes(connection, key).await?;
    let mut shown = Vec::new();
    let mut body = format!("<h1>{}</h1>{INTRO}", escape(title));
    body.push_str(&hero::render(&profile, &abilities, &changes, &mut shown)?);
    body.push_str(&receipts(connection, &Value::Array(shown)).await?);
    Ok(Outcome::Page(document(title, &body)))
}

fn link_items(links: &[Value]) -> String {
    let mut html = String::new();
    for link in links {
        let identity = &link["identity"];
        if let (Some(kind), Some(key), Some(title)) = (
            identity["kind"].as_str(),
            identity["key"].as_str(),
            display_name(identity),
        ) {
            html.push_str(&format!(
                "<li><a href=\"/brain/site/steckbriefe/{}/{}\">{}</a></li>",
                escape(kind),
                hex::encode(key),
                escape(title)
            ));
        }
    }
    html
}

async fn item_page(connection: &mut PgConnection, key: &str) -> Result<Outcome> {
    let Some(card) = load_profile(connection, "item", key).await? else {
        return Ok(Outcome::Missing);
    };
    let Some(title) = display_name(&card["identity"]) else {
        return Ok(Outcome::Missing);
    };
    let mut body = format!(
        "<h1>{}</h1>{INTRO}<h2>Eigenschaften</h2>{}",
        escape(title),
        fact_rows(&card["facts"])
    );
    let links = link_items(
        card["links"]["links"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or(&[]),
    );
    if !links.is_empty() {
        body.push_str(&format!("<h2>Verknüpfungen</h2><ul>{links}</ul>"));
    }
    body.push_str(&receipts(connection, &card).await?);
    Ok(Outcome::Page(document(title, &body)))
}

async fn ability_redirect(connection: &mut PgConnection, key: &str) -> Result<Outcome> {
    Ok(match hero::owner(connection, key).await? {
        Some(hero) => Outcome::Redirect(format!(
            "/brain/site/steckbriefe/hero/{}#{}",
            hex::encode(hero),
            hero::anchor(key)
        )),
        None => Outcome::Missing,
    })
}

pub(super) async fn page(
    State(site): State<Site>,
    Path((kind, id)): Path<(String, String)>,
    method: Method,
) -> Response {
    let id = id.strip_suffix(".html").unwrap_or(&id);
    let Some(key) = decode_id(id) else {
        return missing(method);
    };
    let result: Result<Outcome> = async {
        let mut tx = transaction(&site).await?;
        let outcome = match kind.as_str() {
            "hero" => hero_page(&mut tx, &key).await?,
            "item" => item_page(&mut tx, &key).await?,
            "ability" => ability_redirect(&mut tx, &key).await?,
            _ => Outcome::Missing,
        };
        tx.commit().await?;
        Ok(outcome)
    }
    .await;
    match result {
        Ok(Outcome::Page(html)) => response(StatusCode::OK, html, method),
        Ok(Outcome::Redirect(location)) => (
            StatusCode::MOVED_PERMANENTLY,
            [(header::LOCATION, location)],
        )
            .into_response(),
        Ok(Outcome::Missing) => missing(method),
        Err(_) => {
            eprintln!("site_profile_unavailable");
            unavailable(method)
        }
    }
}

fn index_list(identities: &[Value]) -> Result<(usize, String)> {
    let mut entries = Vec::new();
    for identity in identities {
        if let Some(title) = display_name(identity) {
            let kind = identity["kind"].as_str().context("kind")?;
            let key = identity["key"].as_str().context("key")?;
            entries.push((
                title.to_lowercase(),
                format!(
                    "<li><a href=\"/brain/site/steckbriefe/{}/{}\">{}</a></li>",
                    escape(kind),
                    hex::encode(key),
                    escape(title)
                ),
            ));
        }
    }
    entries.sort();
    let count = entries.len();
    let html = entries.into_iter().map(|(_, entry)| entry).collect();
    Ok((count, html))
}

pub(super) async fn index(State(site): State<Site>, method: Method) -> Response {
    let result: Result<String> = async {
        let mut tx = transaction(&site).await?;
        let heroes = sqlx::query_scalar::<_, Value>("SELECT identity FROM serve.entity_identity WHERE kind='hero' AND serve.hero_is_playable(entity_id) LIMIT 256")
            .fetch_all(&mut *tx)
            .await?;
        let items = sqlx::query_scalar::<_, Value>(
            "SELECT identity FROM serve.entity_identity WHERE kind='item' LIMIT 1024",
        )
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;
        let (hero_count, hero_list) = index_list(&heroes)?;
        let (item_count, item_list) = index_list(&items)?;
        let mut body = String::from("<h1>Steckbriefe</h1><p>Ein Steckbrief pro Held mit Grundwerten, allen Fähigkeiten, Upgrades und Patchverlauf. Darunter folgen die Items.</p>");
        body.push_str(&format!(
            "<h2 id=\"helden\">Helden ({hero_count})</h2><ul>{hero_list}</ul>"
        ));
        body.push_str(&format!(
            "<h2 id=\"items\">Items ({item_count})</h2><ul>{item_list}</ul>"
        ));
        if hero_count + item_count == 0 {
            body.push_str("<p>Es sind noch keine veröffentlichten Steckbriefe verfügbar.</p>");
        }
        Ok(document("Steckbriefe", &body))
    }
    .await;
    match result {
        Ok(html) => response(StatusCode::OK, html, method),
        Err(_) => {
            eprintln!("site_index_unavailable");
            unavailable(method)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn ids_and_html_are_untrusted() {
        assert_eq!(
            decode_id(&hex::encode("hero_haze")),
            Some("hero_haze".into())
        );
        for id in ["", "a", "AA", "2f../", "00", "ff"] {
            assert_eq!(decode_id(id), None);
        }
        assert_eq!(escape("<script>\"&"), "&lt;script&gt;&quot;&amp;");
        for (identity, shown) in [
            (
                json!({"key":"ability_boho_ability02","name_de":"ability_boho_ability02"}),
                None,
            ),
            (
                json!({"key":"upgrade_toughness_3","name_de":"upgrade_toughness_3"}),
                None,
            ),
            (
                json!({"key":"ability_x","name_de":" ","name_en":"Sleep Dagger"}),
                Some("Sleep Dagger"),
            ),
            (
                json!({"key":"hero_krill","name_en":"Mo & Krill"}),
                Some("Mo & Krill"),
            ),
        ] {
            assert_eq!(display_name(&identity), shown);
        }
    }
}
