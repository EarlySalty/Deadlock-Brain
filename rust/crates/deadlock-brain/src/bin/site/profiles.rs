use anyhow::{ensure, Context, Result};
use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
};
use brain_contracts::canonical_receipt::{format_receipt, CanonicalReceipt};
use serde_json::Value;
use sqlx::{PgConnection, Row};
use std::collections::{BTreeMap, BTreeSet};

use super::{cache, hero, Site};

pub(super) fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub(super) fn document(title: &str, body: &str) -> String {
    format!("<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{} · Deadlock</title><style>body{{overflow-wrap:break-word;font:1rem/1.6 system-ui,sans-serif;color:#20252b;background:#f7f5ef;margin:0}}main{{max-width:70rem;margin:auto;padding:clamp(1rem,4vw,3rem)}}a{{color:#254a80}}table{{border-collapse:collapse;width:100%}}th,td{{padding:.7rem;text-align:left;border-bottom:1px solid #bfc3c8;vertical-align:top;overflow-wrap:normal;word-break:normal;hyphens:manual}}.scroll{{overflow-x:auto}}@media (max-width:40rem){{.scroll table,.scroll tbody,.scroll tr,.scroll th,.scroll td{{display:block;width:auto}}.scroll thead{{position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap}}.scroll tr{{border:1px solid #bfc3c8;border-radius:.4rem;background:#fff;margin:0 0 1rem;padding:.4rem 0}}.scroll th,.scroll td{{border-bottom:0;padding:.3rem .8rem;overflow-wrap:anywhere}}.scroll td[data-label]::before{{content:attr(data-label);display:block;font-size:.85rem;font-weight:600;color:#4a525b}}}}code,pre{{white-space:pre-wrap;overflow-wrap:anywhere}}li{{margin:.5rem 0}}article{{margin:2rem 0}}:focus-visible{{outline:3px solid #254a80;outline-offset:3px}}</style></head><body><main><nav><a href=\"/brain/site/steckbriefe\">Steckbriefe</a></nav>{body}</main></body></html>", escape(title))
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

struct Source {
    title: String,
    url: Option<String>,
}

fn language_name(code: &str) -> Option<(&'static str, &'static str)> {
    match code {
        "de" => Some(("deutsch", "german")),
        "en" => Some(("englisch", "english")),
        _ => None,
    }
}

fn commit(revision: &str) -> Option<&str> {
    (revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then_some(revision)
}

fn github(repository: &str, revision: &str, path: &str) -> String {
    match commit(revision) {
        Some(commit) if !path.is_empty() => {
            format!("https://github.com/{repository}/blob/{commit}/{path}")
        }
        _ => format!("https://github.com/{repository}"),
    }
}

fn asset_source(path: &str) -> Source {
    let mut parts = path.split('/');
    let (kind, language) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let subject = match kind {
        "items" => "Item-Daten",
        "heroes" => "Helden-Daten",
        _ => "Spieldaten",
    };
    let endpoint = match kind {
        "items" | "heroes" => format!("{}/v1/assets/{kind}", dbrain_sources::assets_api::BASE_URL),
        _ => dbrain_sources::assets_api::BASE_URL.to_owned(),
    };
    match language_name(language) {
        Some((german, query)) => Source {
            title: format!("{subject} der Deadlock-API ({german})"),
            url: Some(format!("{endpoint}?language={query}")),
        },
        None => Source {
            title: format!("{subject} der Deadlock-API"),
            url: Some(endpoint),
        },
    }
}

fn describe(source: &str, revision: &str, path: &str, patch: Option<&Patch>) -> Source {
    match source {
        "assets" => asset_source(path),
        "deadlock-data" => Source {
            title: "Spieldaten-Export des Deadlock-Wikis".into(),
            url: Some(github("deadlock-wiki/deadlock-data", revision, path)),
        },
        "gametracking" => Source {
            title: "Spieldateien aus GameTracking-Deadlock".into(),
            url: Some(github(
                "SteamTracking/GameTracking-Deadlock",
                revision,
                path,
            )),
        },
        "deadlock-api-meta" => Source {
            title: "Spielstatistiken der Deadlock-API".into(),
            url: Some("https://deadlock-api.com".into()),
        },
        "deadlock_patchnotes_db" => Source {
            title: match patch.map(|patch| patch.label.as_str()) {
                Some(label) if !label.is_empty() => format!("Offizielle Patchnotes: {label}"),
                _ => "Offizielle Patchnotes".into(),
            },
            url: patch.and_then(|patch| patch.url.clone()),
        },
        "deadlock-docs" => Source {
            title: "Deadlock-Dokumentation".into(),
            url: None,
        },
        _ => Source {
            title: "Weitere Quelldaten".into(),
            url: None,
        },
    }
}

fn revision_label(revision: &str) -> String {
    let short = |hash: &str| hash.chars().take(12).collect::<String>();
    match revision.strip_prefix("sha256:") {
        Some(hash) => format!("Prüfsumme {}", short(hash)),
        None if commit(revision).is_some() => format!("Version {}", short(revision)),
        None => format!("Stand {revision}"),
    }
}

fn web_link(url: &str) -> Option<&str> {
    (url.starts_with("https://") && !url.contains(char::is_whitespace)).then_some(url)
}

fn receipt_line(id: i64, evidence_id: &str, revision: &str, source: &Source) -> String {
    let title = escape(&source.title);
    let named = match source.url.as_deref().and_then(web_link) {
        Some(url) => format!("<a href=\"{}\">{title}</a>", escape(url)),
        None => title,
    };
    format!(
        "<li id=\"beleg-{id}\" data-beleg=\"{}\">Beleg {id}: {named}, <span title=\"{}\">{}</span></li>",
        escape(evidence_id),
        escape(revision),
        escape(&revision_label(revision))
    )
}

struct Patch {
    label: String,
    url: Option<String>,
}

async fn patches(connection: &mut PgConnection, objects: &[i64]) -> Result<BTreeMap<i64, Patch>> {
    let mut found = BTreeMap::new();
    if objects.is_empty() {
        return Ok(found);
    }
    let rows = sqlx::query("SELECT raw_object_id,max(value_text::text) FILTER (WHERE stat_key::text='label') AS label,max(value_text::text) FILTER (WHERE stat_key::text='official_url') AS url FROM serve.current_fact WHERE source='deadlock_patchnotes_db' AND stat_key::text IN ('label','official_url') AND raw_object_id=ANY($1::bigint[]) GROUP BY raw_object_id LIMIT 512")
        .bind(objects)
        .fetch_all(connection)
        .await?;
    for row in rows {
        let label: Option<String> = row.try_get("label")?;
        found.insert(
            row.try_get("raw_object_id")?,
            Patch {
                label: label
                    .map(|label| label.trim().to_owned())
                    .unwrap_or_default(),
                url: row.try_get("url")?,
            },
        );
    }
    Ok(found)
}

pub(super) async fn receipts(connection: &mut PgConnection, value: &Value) -> Result<String> {
    let mut ids = BTreeSet::new();
    citation_ids(value, &mut ids)?;
    ensure!(!ids.is_empty(), "citations_missing");
    let rows = sqlx::query("SELECT id,raw_object_id,ingest_run_id,source,source_revision,path,locator_kind,locator,encode(value_sha256,'hex') AS value_sha256,extractor FROM serve.published_receipt WHERE id=ANY($1::bigint[]) ORDER BY id LIMIT 512")
        .bind(ids.iter().copied().collect::<Vec<_>>()).fetch_all(&mut *connection).await?;
    ensure!(rows.len() == ids.len(), "unpublished_citation");
    let mut receipts = Vec::new();
    for row in rows {
        let id: i64 = row.try_get("id")?;
        receipts.push(CanonicalReceipt {
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
        });
    }
    let objects = receipts
        .iter()
        .filter(|receipt| receipt.source == "deadlock_patchnotes_db")
        .map(|receipt| receipt.raw_object_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let patches = patches(connection, &objects).await?;
    let mut html = String::from("<h2 id=\"quellen\">Quellen</h2><ol class=\"quellen\">");
    for receipt in &receipts {
        let evidence = format_receipt(receipt)?;
        let source = describe(
            &receipt.source,
            &receipt.source_revision,
            &receipt.path,
            patches.get(&receipt.raw_object_id),
        );
        html.push_str(&receipt_line(
            receipt.raw_receipt_id,
            &evidence.evidence_id,
            &receipt.source_revision,
            &source,
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

fn stat_label(stat_key: &str) -> Option<&'static str> {
    Some(match stat_key {
        "max_health" => "Maximale Lebenspunkte",
        "base_health_regen" => "Lebensregeneration",
        "max_move_speed" => "Bewegungstempo",
        "sprint_speed" => "Sprinttempo",
        "stamina" => "Ausdauer",
        "light_melee_damage" => "Leichter Nahkampfschaden",
        "heavy_melee_damage" => "Schwerer Nahkampfschaden",
        "type" => "Typ",
        "weapon_bullet_damage" => "Schaden pro Kugel",
        "weapon_rounds_per_second" => "Schüsse pro Sekunde",
        "weapon_clip_size" => "Max. Munition",
        "weapon_reload_time" => "Nachladezeit",
        "item_cost" => "Kosten",
        "item_description_desc" => "Beschreibung",
        "item_description_desc2" => "Weitere Beschreibung",
        "item_description_active" => "Aktive Wirkung",
        "item_description_passive" => "Passive Wirkung",
        _ => return None,
    })
}

fn scale_name(scale_type: &str) -> Option<&'static str> {
    Some(match scale_type {
        "ETechPower" => "Magiekraft",
        "ETechDuration" => "Magiedauer",
        "ETechRange" => "Magiereichweite",
        "ETechRadius" => "Magieradius",
        "ETechCooldown" => "Fähigkeitsabklingzeit",
        "EItemCooldown" => "Item-Abklingzeit",
        "ETechCooldownBetweenChargeUses" => "Abklingzeit zwischen Aufladungen",
        "EMaxChargesIncrease" => "zusätzlichen Aufladungen",
        "EHealingOutput" => "Heilungsleistung",
        "EMeleeRange" => "Nahkampfreichweite",
        "ELevelUpBoons" => "Stufenboni",
        "ELightMeleeDamage" => "leichtem Nahkampfschaden",
        "EHeavyMeleeDamage" => "schwerem Nahkampfschaden",
        "EBuildUpRate" => "Aufbaurate",
        "EBaseWeaponDamageIncrease" | "EWeaponDamageScale" => "Waffenschaden",
        "EWeaponPower" => "Waffenkraft",
        "EBulletDamage" => "Kugelschaden",
        "EChannelDuration" => "Kanalisierungsdauer",
        _ => return None,
    })
}

fn conditional_text(text: &str) -> &str {
    match text {
        "against NPCs" => "gegen NSC",
        "after proc" => "nach Auslösung",
        "within Range" => "im Nahbereich",
        "beyond Range" => "außer Reichweite",
        other => other,
    }
}

fn condition_note(key: &str, value: &Value, condition: &Value) -> Option<String> {
    let text = value
        .as_str()
        .map(str::trim)
        .filter(|text| !text.is_empty());
    Some(match key {
        "lang" => match text? {
            "de" => "Deutscher Spieltext".into(),
            "en" => "Englischer Spieltext".into(),
            _ => return None,
        },
        _ if numbered(key, "usage_flag_") => match text? {
            "ConditionallyApplied" => "Gilt nur unter Bedingungen".into(),
            "ConditionallyEnemyApplied" => "Wirkt unter Bedingungen auf Gegner".into(),
            "IntrinsicallyProvidedInAbility" => "Fester Teil der Fähigkeit".into(),
            _ => return None,
        },
        "conditional" => format!("Bedingung: {}", conditional_text(text?)),
        "scale_type" => {
            let name = scale_name(text?)?;
            match condition["scale_value"]
                .as_str()
                .map(str::trim)
                .filter(|v| !v.is_empty())
            {
                Some(factor) => format!("Skaliert mit {name} (×{factor})"),
                None => format!("Wird von {name} beeinflusst"),
            }
        }
        "upgrade_tier" => format!("Upgrade-Stufe {}", value.as_u64()?),
        "operation" if text? == "delta" => "Bonus auf den Grundwert".into(),
        _ => return None,
    })
}

fn numbered(key: &str, prefix: &str) -> bool {
    key.strip_prefix(prefix)
        .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|byte| byte.is_ascii_digit()))
}

fn display_unit(text: &str) -> Option<&'static str> {
    Some(match text {
        "EDisplayUnit_Meters" => "Meter",
        "EDisplayUnit_MetersPerSecond" => "Meter pro Sekunde",
        "EDisplayUnit_Normal" => "ohne Einheit",
        _ => return None,
    })
}

fn detail(key: &str, value: &Value) -> Option<String> {
    let text = value.as_str().map(str::trim)?;
    Some(match key {
        "display_units" => format!("Anzeigeeinheit: {}", display_unit(text)?),
        _ if numbered(key, "scale_stat_") => {
            format!("Weitere Skalierungsgröße: {}", scale_name(text)?)
        }
        _ => return None,
    })
}

fn qualifier_citation<'a>(fact: &'a Value, key: &str) -> Option<&'a str> {
    fact["qualifiers"]
        .as_array()?
        .iter()
        .find(|qualifier| qualifier["key"] == key)?["citation"]
        .as_str()
}

fn beleg_link(fact: &Value, key: &str) -> String {
    qualifier_citation(fact, key)
        .map(|citation| format!(" <a href=\"#beleg-{0}\">Beleg {0}</a>", escape(citation)))
        .unwrap_or_default()
}

fn condition_notes(fact: &Value) -> String {
    let condition = &fact["condition"];
    let Some(fields) = condition.as_object() else {
        return String::new();
    };
    let mut notes = Vec::new();
    let mut details = Vec::new();
    for (key, value) in fields {
        if let Some(note) = condition_note(key, value, condition) {
            let extra = if key == "scale_type" {
                beleg_link(fact, "scale_value")
            } else {
                String::new()
            };
            notes.push(format!(
                "<p>{}{}{extra}</p>",
                escape(&note),
                beleg_link(fact, key)
            ));
        } else if let Some(text) = detail(key, value) {
            details.push(format!("{}{}", escape(&text), beleg_link(fact, key)));
        }
    }
    if !details.is_empty() {
        notes.push(format!(
            "<p>Weitere Quellangaben: {}</p>",
            details.join("; ")
        ));
    }
    notes.concat()
}

fn fact_label(fact: &Value) -> &str {
    let stat_key = fact["stat_key"].as_str();
    fact["label"]
        .as_str()
        .map(str::trim)
        .filter(|label| !label.is_empty() && !key_shaped(label))
        .map(|label| hero::german_label(label).unwrap_or(label))
        .or_else(|| stat_key.and_then(stat_label))
        .or_else(|| stat_key.filter(|key| !key_shaped(key)))
        .unwrap_or("Wert")
}

fn unit_text(unit: &str) -> &str {
    match unit.trim() {
        "souls" => "Seelen",
        other => other,
    }
}

fn fact_value(fact: &Value) -> String {
    let unit = fact["unit"]
        .as_str()
        .map(unit_text)
        .filter(|unit| !unit.is_empty())
        .map(|unit| format!(" {}", escape(unit)))
        .unwrap_or_default();
    match &fact["value"] {
        Value::String(text) => format!("{}{unit}", hero::markup(text)),
        other => format!("{}{unit}", escape(&other.to_string())),
    }
}

pub(super) fn fact_rows(section: &Value) -> String {
    let mut html = String::from("<div class=\"scroll\"><table><thead><tr><th scope=\"col\">Wert</th><th scope=\"col\">Angabe</th><th scope=\"col\">Bedingung und Beleg</th></tr></thead><tbody>");
    let rows = section["facts"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    for fact in rows {
        let citation = fact["citation"].as_str().unwrap_or("");
        html.push_str(&format!(
            "<tr><th scope=\"row\">{}</th><td data-label=\"Angabe\">{}</td><td data-label=\"Bedingung und Beleg\"><a href=\"#beleg-{}\">Beleg {}</a>{}",
            escape(fact_label(fact)),
            fact_value(fact),
            escape(citation),
            escape(citation),
            condition_notes(fact)
        ));
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

impl Outcome {
    fn rendered(&self) -> Option<cache::Rendered> {
        match self {
            Outcome::Page(html) => Some(cache::Rendered::Page(html.as_str().into())),
            Outcome::Redirect(location) => {
                Some(cache::Rendered::Redirect(location.as_str().into()))
            }
            Outcome::Missing => None,
        }
    }
}

impl From<cache::Rendered> for Outcome {
    fn from(rendered: cache::Rendered) -> Self {
        match rendered {
            cache::Rendered::Page(html) => Outcome::Page(html.to_string()),
            cache::Rendered::Redirect(location) => Outcome::Redirect(location.to_string()),
        }
    }
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

async fn cached_outcome(site: &Site, kind: &str, key: &str) -> Result<Outcome> {
    let mut tx = transaction(site).await?;
    let generation = cache::generation(&mut tx).await?;
    let cache_key = format!("{kind}/{key}");
    if let Some(hit) = site.pages.get(&generation, &cache_key) {
        tx.commit().await?;
        return Ok(hit.into());
    }
    let outcome = match kind {
        "hero" => hero_page(&mut tx, key).await?,
        "item" => item_page(&mut tx, key).await?,
        "ability" => ability_redirect(&mut tx, key).await?,
        _ => Outcome::Missing,
    };
    tx.commit().await?;
    if let Some(rendered) = outcome.rendered() {
        site.pages.put(&generation, cache_key, rendered);
    }
    Ok(outcome)
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
    if !matches!(kind.as_str(), "hero" | "item" | "ability") {
        return missing(method);
    }
    match cached_outcome(&site, &kind, &key).await {
        Ok(Outcome::Page(html)) => response(StatusCode::OK, html, method),
        Ok(Outcome::Redirect(location)) => (
            StatusCode::TEMPORARY_REDIRECT,
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

const INDEX_KEY: &str = "index";

pub(super) async fn index(State(site): State<Site>, method: Method) -> Response {
    let result: Result<String> = async {
        let mut tx = transaction(&site).await?;
        let generation = cache::generation(&mut tx).await?;
        if let Some(cache::Rendered::Page(html)) = site.pages.get(&generation, INDEX_KEY) {
            tx.commit().await?;
            return Ok(html.to_string());
        }
        let heroes = sqlx::query_scalar::<_, Value>("SELECT identity FROM serve.entity_identity WHERE kind='hero' AND serve.hero_is_playable(entity_id) ORDER BY key LIMIT 256")
            .fetch_all(&mut *tx)
            .await?;
        let items = sqlx::query_scalar::<_, Value>(
            "SELECT identity FROM serve.entity_identity WHERE kind='item' ORDER BY key LIMIT 1024",
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
        let html = document("Steckbriefe", &body);
        site.pages.put(
            &generation,
            INDEX_KEY.to_owned(),
            cache::Rendered::Page(html.as_str().into()),
        );
        Ok(html)
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

    #[test]
    fn conditions_render_as_german_text_without_json() {
        assert_eq!(condition_notes(&json!({"condition": {}})), "");
        let html = condition_notes(&json!({
            "condition": {
                "scale_type": "ETechPower",
                "scale_value": "0.5",
                "scale_class": "scale_function_tech_damage",
                "display_units": "EDisplayUnit_Meters",
                "usage_flag_1": "Mystery",
                "custom": {"a": [1, true]}
            },
            "qualifiers": [
                {"key": "scale_type", "citation": "7"},
                {"key": "scale_value", "citation": "8"},
                {"key": "usage_flag_1", "citation": "9"}
            ]
        }));
        assert!(html.contains("Skaliert mit Magiekraft (×0.5) <a href=\"#beleg-7\">Beleg 7</a> <a href=\"#beleg-8\">Beleg 8</a>"));
        assert!(html.contains("Anzeigeeinheit: Meter"));
        for hidden in [
            "scale_function",
            "Mystery",
            "custom",
            "usage_flag",
            "Skalierungsfaktor",
            "Beleg 9",
        ] {
            assert!(!html.contains(hidden), "{hidden}");
        }
        assert!(!html.contains('{'));
    }

    #[test]
    fn item_values_render_game_markup_safely() {
        let html = fact_rows(&json!({"facts": [
            {"stat_key": "item_description_desc", "citation": "3", "value": "Gain <span class=\"highlight\">Barrier</span> on <svg width=\"128\"><path d=\"M6 7\" fill=\"white\"/></svg> <span class=\"inline-attribute-label SpiritDamage\" style=\"color: #EC9719;\">spirit damage</span>.<br><br>Then <script>x</script>&amp; more <span>plain</span>"},
            {"stat_key": "item_cost", "citation": "4", "value": "800", "unit": "souls"},
            {"stat_key": "item_secret_key", "citation": "5", "value": 1, "unit": " m"},
            {"label": "Weapon Damage", "stat_key": "item_x", "citation": "6", "value": "7"}
        ]}));
        assert!(html.contains("<th scope=\"row\">Beschreibung</th><td data-label=\"Angabe\">Gain <strong>Barrier</strong> on <strong>spirit damage</strong>.<br><br>Then x&amp; more plain</td>"), "{html}");
        assert!(
            html.contains("<th scope=\"row\">Kosten</th><td data-label=\"Angabe\">800 Seelen</td>")
        );
        assert!(html.contains("<th scope=\"row\">Wert</th><td data-label=\"Angabe\">1 m</td>"));
        assert!(html.contains("<th scope=\"row\">Waffenschaden</th>"));
        for hidden in ["svg", "span", "script", "item_", "souls", "#EC9719"] {
            assert!(!html.contains(hidden), "{hidden}");
        }
    }

    #[test]
    fn receipts_render_as_readable_german_lines() {
        let patch = Patch {
            label: "10-24-2025 Update".into(),
            url: Some("https://forums.playdeadlock.com/threads/x.1/".into()),
        };
        let line = receipt_line(
            7,
            "gamefacts.receipt.v1:sha256:ab",
            "sha256:5646654812345678",
            &describe(
                "deadlock_patchnotes_db",
                "sha256:5646654812345678",
                "241",
                Some(&patch),
            ),
        );
        assert_eq!(line, "<li id=\"beleg-7\" data-beleg=\"gamefacts.receipt.v1:sha256:ab\">Beleg 7: <a href=\"https://forums.playdeadlock.com/threads/x.1/\">Offizielle Patchnotes: 10-24-2025 Update</a>, <span title=\"sha256:5646654812345678\">Prüfsumme 564665481234</span></li>");
        let asset = describe("assets", "sha256:aa", "items/de/649.json", None);
        assert_eq!(asset.title, "Item-Daten der Deadlock-API (deutsch)");
        assert_eq!(
            asset.url.as_deref(),
            Some("https://api.deadlock-api.com/v1/assets/items?language=german")
        );
        let wiki = describe(
            "deadlock-data",
            "8d53e1a606e5e207258d534351c6296550bcc850",
            "data/json/hero-data.json",
            None,
        );
        assert_eq!(wiki.url.as_deref(), Some("https://github.com/deadlock-wiki/deadlock-data/blob/8d53e1a606e5e207258d534351c6296550bcc850/data/json/hero-data.json"));
        let unknown = receipt_line(9, "e", "r", &describe("raw_source_name", "r", "p_q", None));
        assert!(!unknown.contains("raw_source_name") && !unknown.contains("<a "));
        let hostile = Source {
            title: "x".into(),
            url: Some("javascript:alert(1)".into()),
        };
        assert!(!receipt_line(1, "e", "r", &hostile).contains("javascript"));
    }
}
