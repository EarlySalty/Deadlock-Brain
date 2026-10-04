use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{util::form_urlencode, Result, SourcesError};

use super::{
    check_api, merge_inventory_page, merge_page, WikiInventoryPage, CONTRACT_VERSION, SOURCE_ID,
};

fn official_origin() -> String {
    "https://deadlock.wiki".into()
}
fn siteinfo_hash_representation() -> String {
    "serde_json_compact".into()
}

pub(super) fn source_id_for_origin(origin: &str) -> Result<&'static str> {
    let domain = origin
        .strip_prefix("https://")
        .or_else(|| origin.strip_prefix("http://"))
        .ok_or_else(|| {
            SourcesError::invalid_input("Wiki-Quellenadresse benötigt HTTP oder HTTPS")
        })?;
    match domain {
        "deadlock.wiki" => Ok(SOURCE_ID),
        "deadlocked.wiki" => Ok("deadlocked-wiki"),
        "deadlockwiki.org" => Ok("deadlockwiki-org"),
        "deadlock.miraheze.org" => Ok("deadlock-miraheze-org"),
        "deadlockwiki.miraheze.org" => Ok("deadlockwiki-miraheze-org"),
        _ => Err(SourcesError::invalid_input(
            "Nicht belegte Offline-Wiki-Quelle",
        )),
    }
}

pub(super) fn source_origin_from_url(url: &str) -> Result<&str> {
    if url.chars().any(|c| c.is_control() || c.is_whitespace()) || url.contains(['#', '\\']) {
        return Err(SourcesError::invalid_input("Ungültige Wiki-Quellenadresse"));
    }
    let scheme = if url.starts_with("https://") {
        8
    } else if url.starts_with("http://") {
        7
    } else {
        return Err(SourcesError::invalid_input("Wiki-Quellenherkunft fehlt"));
    };
    let end = url[scheme..]
        .find('/')
        .map(|index| scheme + index)
        .unwrap_or(url.len());
    let origin = &url[..end];
    source_id_for_origin(origin)?;
    Ok(origin)
}

fn article_base_from_url(base: &str) -> Result<String> {
    let origin = source_origin_from_url(base)?;
    if let Some((script, _)) = base.split_once("?title=") {
        return Ok(format!("{script}?title="));
    }
    if base.contains('?') {
        return Err(SourcesError::invalid_input("Unbekannter XML-Artikelpfad"));
    }
    base.rsplit_once('/')
        .filter(|(prefix, _)| prefix.len() >= origin.len())
        .map(|(prefix, _)| format!("{prefix}/"))
        .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne Artikelpfad"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSourceContext {
    #[serde(default = "official_origin")]
    pub source_origin: String,
    #[serde(default)]
    pub source_article_base: Option<String>,
    #[serde(default = "siteinfo_hash_representation")]
    pub source_siteinfo_hash_representation: String,
    pub language: String,
    pub namespaces: BTreeMap<i64, String>,
    pub license: Value,
    pub statistics: Value,
    pub source_siteinfo_sha256: String,
    #[serde(default)]
    pub license_observed_at: Option<String>,
    #[serde(default)]
    pub source_fetched_at: Option<String>,
    #[serde(default)]
    pub source_capture_sha256: Option<String>,
    #[serde(default)]
    pub historical_capture: bool,
}

impl WikiSourceContext {
    pub fn from_siteinfo(payload: &Value) -> Result<Self> {
        check_api(payload)?;
        let query = payload
            .get("query")
            .and_then(Value::as_object)
            .ok_or_else(|| SourcesError::invalid_input("Wiki-siteinfo fehlt"))?;
        let rights = query
            .get("rightsinfo")
            .and_then(Value::as_object)
            .ok_or_else(|| SourcesError::invalid_input("Wiki-Lizenznachweis fehlt"))?;
        let name = text(rights.get("text"))
            .ok_or_else(|| SourcesError::invalid_input("Wiki-Lizenzname fehlt"))?;
        let url = text(rights.get("url"))
            .ok_or_else(|| SourcesError::invalid_input("Wiki-Lizenzadresse fehlt"))?;
        if !url.starts_with("https://") || url.chars().any(char::is_control) {
            return Err(SourcesError::invalid_input("Ungültige Wiki-Lizenzadresse"));
        }
        let server = payload
            .pointer("/query/general/server")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                SourcesError::invalid_input("siteinfo ohne belegte Wiki-Quellenadresse")
            })?;
        let server = if server.starts_with("//") {
            format!("https:{server}")
        } else {
            server.into()
        };
        let source_origin = source_origin_from_url(&server)?.to_string();
        if server != source_origin {
            return Err(SourcesError::invalid_input(
                "siteinfo-Server enthält einen Pfad",
            ));
        }
        let source_article_base = match payload.pointer("/query/general/articlepath") {
            Some(Value::String(path)) => {
                let prefix = path
                    .strip_suffix("$1")
                    .filter(|prefix| prefix.starts_with('/') && !prefix.starts_with("//"))
                    .ok_or_else(|| {
                        SourcesError::invalid_input("Ungültiger siteinfo-Artikelpfad")
                    })?;
                Some(format!("{source_origin}{prefix}"))
            }
            None if source_id_for_origin(&source_origin)? == SOURCE_ID => None,
            _ => {
                return Err(SourcesError::invalid_input(
                    "siteinfo ohne belegten Artikelpfad",
                ));
            }
        };
        let namespace_values = query
            .get("namespaces")
            .ok_or_else(|| SourcesError::invalid_input("Wiki-Namensraumverzeichnis fehlt"))?;
        let mut namespaces = BTreeMap::new();
        match namespace_values {
            Value::Object(values) => {
                for (key, value) in values {
                    let id = value
                        .get("id")
                        .and_then(Value::as_i64)
                        .or_else(|| key.parse::<i64>().ok())
                        .ok_or_else(|| {
                            SourcesError::invalid_input("Ungültige Wiki-Namensraum-ID")
                        })?;
                    add_namespace(&mut namespaces, id, value)?;
                }
            }
            Value::Array(values) => {
                for value in values {
                    let id = value
                        .get("id")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| SourcesError::invalid_input("Wiki-Namensraum-ID fehlt"))?;
                    add_namespace(&mut namespaces, id, value)?;
                }
            }
            _ => {
                return Err(SourcesError::invalid_input(
                    "Ungültiges Wiki-Namensraumverzeichnis",
                ));
            }
        }
        if !namespaces.contains_key(&0) {
            return Err(SourcesError::invalid_input("Wiki-Hauptnamensraum fehlt"));
        }
        let language = payload
            .pointer("/query/general/lang")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .unwrap_or("und");
        let context = Self {
            source_article_base,
            source_siteinfo_hash_representation: siteinfo_hash_representation(),
            license: json!({
                "name": name,
                "url": url,
                "attribution": if source_id_for_origin(&source_origin)? == SOURCE_ID {
                    "Deadlock Wiki contributors".to_string()
                } else { format!("{source_origin} contributors") },
                "redistribution_allowed": false,
            }),
            source_origin,
            language: language.into(),
            namespaces,
            statistics: query
                .get("statistics")
                .cloned()
                .unwrap_or_else(|| json!({})),
            source_siteinfo_sha256: sha256(&serde_json::to_vec(payload)?),
            license_observed_at: None,
            source_fetched_at: None,
            source_capture_sha256: None,
            historical_capture: false,
        };
        context.validate()?;
        Ok(context)
    }

    pub fn from_mediawiki_export(xml: &str) -> Result<Self> {
        let export = roxmltree::Document::parse(xml).map_err(|error| {
            SourcesError::invalid_input(format!("Ungültiger MediaWiki-Export: {error}"))
        })?;
        let root = export.root_element();
        if root.tag_name().name() != "mediawiki" {
            return Err(SourcesError::invalid_input("Kein MediaWiki-Export"));
        }
        let siteinfo = root
            .children()
            .find(|node| node.has_tag_name("siteinfo"))
            .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne Quellenherkunft"))?;
        let base = child_text(siteinfo, "base")
            .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne Quellenbasis"))?;
        let source_origin = source_origin_from_url(base)?.to_string();
        let article_base = article_base_from_url(base)?;
        let namespace_list = siteinfo
            .children()
            .find(|node| node.has_tag_name("namespaces"))
            .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne Namensräume"))?;
        let mut namespaces = BTreeMap::new();
        for namespace in namespace_list
            .children()
            .filter(|node| node.has_tag_name("namespace"))
        {
            let id = namespace
                .attribute("key")
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or_else(|| SourcesError::invalid_input("Ungültige XML-Namensraum-ID"))?;
            if namespaces
                .insert(id, namespace.text().unwrap_or_default().into())
                .is_some()
            {
                return Err(SourcesError::invalid_input("Doppelte XML-Namensraum-ID"));
            }
        }
        let context = Self {
            source_origin: source_origin.clone(),
            source_article_base: Some(article_base),
            source_siteinfo_hash_representation: "mediawiki_export_siteinfo_xml_utf8".into(),
            language: root
                .attribute(("http://www.w3.org/XML/1998/namespace", "lang"))
                .filter(|language| !language.is_empty())
                .unwrap_or("und")
                .into(),
            namespaces,
            license: json!({"name": "unverified", "url": null,
                "attribution": format!("{source_origin} contributors"), "redistribution_allowed": false}),
            statistics: json!({}),
            source_siteinfo_sha256: sha256(xml[siteinfo.range()].as_bytes()),
            license_observed_at: None,
            source_fetched_at: None,
            source_capture_sha256: None,
            historical_capture: true,
        };
        context.validate()?;
        Ok(context)
    }

    pub fn source_id(&self) -> Result<&'static str> {
        source_id_for_origin(&self.source_origin)
    }

    pub(super) fn validate(&self) -> Result<()> {
        self.source_id()?;
        if let Some(base) = &self.source_article_base {
            if source_origin_from_url(base)? != self.source_origin
                || !(base.ends_with('/') && !base.contains('?') || base.ends_with("?title="))
            {
                return Err(SourcesError::invalid_input(
                    "Artikelpfad und Wiki-Quelle widersprechen sich",
                ));
            }
        }
        if !matches!(
            self.source_siteinfo_hash_representation.as_str(),
            "serde_json_compact" | "mediawiki_export_siteinfo_xml_utf8"
        ) {
            return Err(SourcesError::invalid_input(
                "Unbekannte Wiki-Quellenhashdarstellung",
            ));
        }
        for timestamp in [&self.license_observed_at, &self.source_fetched_at]
            .into_iter()
            .flatten()
        {
            validate_observed_at(timestamp)?;
        }
        if self.source_capture_sha256.as_ref().is_some_and(|hash| {
            hash.len() != 64
                || !hash
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        }) {
            return Err(SourcesError::invalid_input(
                "Ungültiger Wiki-Aufzeichnungshash",
            ));
        }
        if self.language.is_empty()
            || !self.namespaces.contains_key(&0)
            || self.source_siteinfo_sha256.len() != 64
            || !self
                .source_siteinfo_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
            || self.license.get("name").and_then(Value::as_str).is_none()
            || self
                .license
                .get("attribution")
                .and_then(Value::as_str)
                .is_none()
            || self
                .license
                .get("redistribution_allowed")
                .and_then(Value::as_bool)
                != Some(false)
            || !self
                .license
                .get("url")
                .is_some_and(|value| value.is_null() || value.is_string())
        {
            return Err(SourcesError::invalid_input(
                "Ungültiger Wiki-Quellenkontext",
            ));
        }
        Ok(())
    }
}

fn add_namespace(namespaces: &mut BTreeMap<i64, String>, id: i64, value: &Value) -> Result<()> {
    let name = value
        .get("canonical")
        .and_then(Value::as_str)
        .or_else(|| value.get("name").and_then(Value::as_str))
        .or_else(|| value.get("*").and_then(Value::as_str))
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Namensraumname fehlt"))?;
    if namespaces.insert(id, name.into()).is_some() {
        return Err(SourcesError::invalid_input("Doppelte Wiki-Namensraum-ID"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WikiGap {
    pub document_id: Option<String>,
    pub title: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct WikiNormalization {
    pub documents: Vec<Value>,
    pub pages: Vec<WikiInventoryPage>,
    pub gaps: Vec<WikiGap>,
}

pub fn normalize_api_response(
    payload: &Value,
    context: &WikiSourceContext,
    observed_at: &str,
) -> Result<WikiNormalization> {
    validate_observed_at(observed_at)?;
    context.validate()?;
    check_api(payload)?;
    let page_values = match payload.pointer("/query/pages") {
        Some(Value::Array(values)) => values.iter().collect::<Vec<_>>(),
        Some(Value::Object(values)) => values.values().collect(),
        None if payload.get("batchcomplete").is_some() || payload.get("continue").is_some() => {
            return Ok(WikiNormalization::default());
        }
        _ => {
            return Err(SourcesError::invalid_input(
                "Wiki-Seitenliste fehlt oder ist ungültig",
            ));
        }
    };
    let mut out = WikiNormalization::default();
    for page in page_values {
        normalize_page(page, context, observed_at, &mut out)?;
    }
    if let Some(redirects) = payload.pointer("/query/redirects") {
        let redirects = redirects
            .as_array()
            .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Redirectzuordnungen"))?;
        for redirect in redirects {
            let from = text(redirect.get("from"))
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Redirectquelle fehlt"))?;
            let to = text(redirect.get("to"))
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Redirectziel fehlt"))?;
            for document in &mut out.documents {
                if document["title"].as_str() == Some(to) {
                    let aliases = document["metadata"]["query_redirect_mappings"]
                        .as_array_mut()
                        .ok_or_else(|| SourcesError::invariant("Wiki-Redirectliste fehlt"))?;
                    aliases.push(json!({"from": from, "to": to}));
                }
            }
            let missing = WikiGap {
                document_id: None,
                title: Some(from.into()),
                reason: "redirect_source_page_and_revision_unavailable".into(),
            };
            if !out.gaps.contains(&missing) {
                out.gaps.push(missing);
            }
        }
    }
    Ok(out)
}

fn normalize_page(
    page: &Value,
    context: &WikiSourceContext,
    observed_at: &str,
    out: &mut WikiNormalization,
) -> Result<()> {
    let title = page
        .get("title")
        .and_then(Value::as_str)
        .filter(|title| !title.trim().is_empty() && !title.chars().any(char::is_control))
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Seitentitel fehlt oder ist ungültig"))?;
    let page_id = page
        .get("pageid")
        .and_then(Value::as_i64)
        .filter(|id| *id > 0);
    let namespace_id = match page.get("ns") {
        Some(Value::Null) | None => None,
        Some(value) => Some(
            value
                .as_i64()
                .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Namensraum-ID"))?,
        ),
    };
    let source_id = context.source_id()?;
    let locator = page_locator(context, page_id, title);
    let document_id = page_id
        .map(|id| format!("wiki:{source_id}:page:{id}"))
        .unwrap_or_else(|| format!("wiki:{source_id}:url:{}", sha256(locator.as_bytes())));
    if page_id.is_none() {
        gap(out, &document_id, title, "page_id_unknown");
    }
    if namespace_id.is_none() {
        gap(out, &document_id, title, "namespace_unknown");
    }
    let mut inventory_page = WikiInventoryPage {
        page_id,
        namespace_id,
        title: title.into(),
        document_id: document_id.clone(),
        revision: None,
        page_kind: page_kind(namespace_id, context, flag(page.get("redirect"))).into(),
        content_available: false,
        available_revision: None,
    };
    if flag(page.get("missing")) || flag(page.get("invalid")) {
        gap(out, &document_id, title, "page_missing_or_invalid");
        out.pages.push(inventory_page);
        return Ok(());
    }
    let revisions = match page.get("revisions") {
        Some(Value::Array(revisions)) => revisions.iter().collect::<Vec<_>>(),
        None => vec![&Value::Null],
        _ => return Err(SourcesError::invalid_input("Ungültige Wiki-Revisionsliste")),
    };
    if revisions.is_empty() {
        gap(out, &document_id, title, "revision_unavailable");
        out.pages.push(inventory_page);
        return Ok(());
    }
    let page_identity = inventory_page.clone();
    for revision in revisions {
        let revision_id = match revision.get("revid") {
            Some(Value::Null) | None => None,
            Some(value) => Some(
                value
                    .as_i64()
                    .filter(|id| *id > 0)
                    .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Revisions-ID"))?,
            ),
        };
        let mut candidate = page_identity.clone();
        candidate.revision = revision_id.map(|id| id.to_string());
        merge_inventory_page(&mut inventory_page, candidate.clone());
        if revision_id.is_none() {
            gap(out, &document_id, title, "revision_id_unknown");
        }
        let slot = revision.pointer("/slots/main");
        let raw = slot
            .and_then(|slot| slot.get("content").or_else(|| slot.get("*")))
            .or_else(|| revision.get("content").or_else(|| revision.get("*")));
        let suppressed = flag(revision.get("texthidden"))
            || flag(revision.get("contenthidden"))
            || slot.is_some_and(|slot| {
                flag(slot.get("texthidden")) || flag(slot.get("contenthidden"))
            });
        let (content, representation) = if suppressed {
            gap(out, &document_id, title, "revision_content_suppressed");
            continue;
        } else if let Some(content) = raw.and_then(Value::as_str) {
            (content, "revision_slot")
        } else if let Some(content) = page.get("extract").and_then(Value::as_str) {
            gap(
                out,
                &document_id,
                title,
                "raw_revision_text_unavailable_rendered_extract_only",
            );
            (content, "rendered_extract")
        } else {
            gap(out, &document_id, title, "revision_content_unavailable");
            continue;
        };
        let content_hash = sha256(content.as_bytes());
        let revision_key = if representation == "rendered_extract" {
            format!(
                "rendered_extract:{}:{content_hash}",
                revision_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "unknown".into())
            )
        } else {
            revision_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| format!("unknown:{content_hash}"))
        };
        let revision_timestamp = revision.get("timestamp").and_then(Value::as_str);
        if revision_timestamp.is_none() {
            gap(out, &document_id, title, "revision_timestamp_unknown");
        }
        let namespace_name = namespace_id
            .and_then(|id| context.namespaces.get(&id))
            .map(String::as_str);
        let content_model = slot
            .and_then(|slot| slot.get("contentmodel"))
            .or_else(|| revision.get("contentmodel"))
            .and_then(Value::as_str);
        let dependencies = extract_dependencies(content)?;
        let redirect_target = page
            .get("redirect_target")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| dependencies.redirect_target.clone());
        let kind = page_kind(
            namespace_id,
            context,
            flag(page.get("redirect")) || redirect_target.is_some(),
        );
        let historical = context.historical_capture
            || namespace_name.is_some_and(|name| name.eq_ignore_ascii_case("update"))
            || is_historical(title)
            || dependencies
                .categories
                .iter()
                .any(|category| is_historical(category));
        let historical_sections = history_sections(content)?;
        let author = revision.get("user").and_then(Value::as_str);
        let source_attribution = context.license["attribution"]
            .as_str()
            .ok_or_else(|| SourcesError::invariant("Wiki-Quellenattribution fehlt"))?;
        let attribution = author
            .map(|author| format!("{source_attribution}; revision author: {author}"))
            .unwrap_or_else(|| source_attribution.into());
        let mut license = context.license.clone();
        license["attribution"] = json!(attribution);
        let mut facts = Vec::new();
        if representation == "revision_slot"
            && namespace_name.is_some_and(|name| {
                name.eq_ignore_ascii_case("data") || name.eq_ignore_ascii_case("bucket")
            })
            && content_model.is_some_and(|model| {
                model.eq_ignore_ascii_case("json") || model.eq_ignore_ascii_case("JsonConfig.Json")
            })
        {
            match crate::knowledge_contract::parse_unique_json(content) {
                Ok(data) => flatten_data(
                    &data,
                    "",
                    &document_id,
                    &revision_key,
                    historical,
                    &mut facts,
                    0,
                )?,
                Err(_) => gap(out, &document_id, title, "data_json_invalid_or_ambiguous"),
            }
        }
        let document = json!({
            "contract_version": CONTRACT_VERSION,
            "source_kind": "wiki",
            "source_id": source_id,
            "document_id": document_id,
            "source_locator": locator,
            "title": title,
            "language": page.get("pagelanguage").and_then(Value::as_str).unwrap_or(&context.language),
            "revision": revision_key,
            "observed_at": observed_at,
            "content_sha256": content_hash,
            "content": content,
            "evidence_status": "source_statement",
            "license": license,
            "metadata": {
                "page_id": page_id,
                "namespace_id": namespace_id,
                "namespace_name": namespace_name,
                "page_kind": kind,
                "revision_id": revision_id,
                "revision_timestamp": revision_timestamp,
                "revision_url": revision_id.map(|id| format!("{locator}&oldid={id}")),
                "revision_author": author,
                "revision_contributor": revision.get("contributor"),
                "provenance_capture_format": "mediawiki_api_json",
                "attribution_url": format!("{}&action=history", locator),
                "source_origin": context.source_origin,
                "source_siteinfo_sha256": context.source_siteinfo_sha256,
                "source_siteinfo_hash_representation": context.source_siteinfo_hash_representation,
                "license_observed_at": context.license_observed_at,
                "source_fetched_at": context.source_fetched_at,
                "source_capture_sha256": context.source_capture_sha256,
                "historical_capture": context.historical_capture,
                "license_revision_verified": false,
                "query_redirect_mappings": [],
                "redirect_source_revisions_pinned": false,
                "content_model": content_model,
                "content_representation": representation,
                "rendered_extract": page.get("extract").and_then(Value::as_str),
                "rendered_extract_sha256": page.get("extract").and_then(Value::as_str).map(|extract| sha256(extract.as_bytes())),
                "rendered_extract_observation_only": true,
                "historical": historical,
                "historical_sections": historical_sections,
                "includes_historical_content": historical || !historical_sections.is_empty(),
                "game_patch_verified": false,
                "temporal_scope": if historical { "historical" } else { "source_revision_not_verified_against_current_game" },
                "redirect_target": redirect_target,
                "template_dependencies": dependencies.templates,
                "module_dependencies": dependencies.modules,
                "categories": dependencies.categories,
                "dependencies_expanded": false,
                "rendered_template_revisions_pinned": false,
                "content_is_untrusted_data": true,
            },
            "facts": facts,
        });
        candidate.content_available = representation == "revision_slot";
        candidate.page_kind = kind.into();
        candidate.revision = Some(revision_key.clone());
        candidate.available_revision = candidate.content_available.then_some(revision_key);
        merge_inventory_page(&mut inventory_page, candidate);
        out.documents.push(document);
    }
    out.pages.push(inventory_page);
    Ok(())
}

pub fn normalize_mediawiki_export(
    xml: &str,
    context: &WikiSourceContext,
    observed_at: &str,
) -> Result<WikiNormalization> {
    validate_observed_at(observed_at)?;
    context.validate()?;
    let export = roxmltree::Document::parse(xml).map_err(|error| {
        SourcesError::invalid_input(format!("Ungültiger MediaWiki-Export: {error}"))
    })?;
    let root = export.root_element();
    if root.tag_name().name() != "mediawiki" {
        return Err(SourcesError::invalid_input("Kein MediaWiki-Export"));
    }
    let base = root
        .children()
        .find(|node| node.has_tag_name("siteinfo"))
        .and_then(|node| child_text(node, "base"))
        .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne belegte Quellenbasis"))?;
    if source_origin_from_url(base)? != context.source_origin {
        return Err(SourcesError::invalid_input(
            "MediaWiki-Export und Quellenkontext haben verschiedene Herkunft",
        ));
    }
    let mut export_context = context.clone();
    export_context.source_article_base = Some(article_base_from_url(base)?);
    export_context.validate()?;
    let mut out = WikiNormalization::default();
    for page in root.children().filter(|node| node.has_tag_name("page")) {
        let title = child_text(page, "title")
            .ok_or_else(|| SourcesError::invalid_input("MediaWiki-Export ohne Seitentitel"))?;
        let id = child_text(page, "id").and_then(|value| value.parse::<i64>().ok());
        let ns = child_text(page, "ns").and_then(|value| value.parse::<i64>().ok());
        let mut revisions = Vec::new();
        for revision in page.children().filter(|node| node.has_tag_name("revision")) {
            let contributor = revision
                .children()
                .find(|node| node.has_tag_name("contributor"));
            let author = contributor
                .and_then(|node| child_text(node, "username"))
                .or_else(|| contributor.and_then(|node| child_text(node, "ip")));
            let contributor_metadata = contributor.map(|node| {
                json!({
                    "username": child_text(node, "username"),
                    "id": child_text(node, "id"),
                    "ip": child_text(node, "ip"),
                    "deleted": node.attribute("deleted"),
                })
            });
            let text = revision.children().find(|node| node.has_tag_name("text"));
            let mut row = json!({
                "revid": child_text(revision, "id").map(|value| value.parse::<i64>())
                    .transpose().map_err(|_| SourcesError::invalid_input("Ungültige XML-Revisions-ID"))?,
                "timestamp": child_text(revision, "timestamp"),
                "user": author,
                "contributor": contributor_metadata,
                "slots": { "main": {
                    "contentmodel": child_text(revision, "model"),
                }},
            });
            if let Some(text) = text {
                if text.attribute("deleted").is_some() {
                    row["slots"]["main"]["texthidden"] = json!(true);
                } else {
                    row["slots"]["main"]["content"] = json!(text.text().unwrap_or_default());
                }
            }
            revisions.push(row);
        }
        let mut row = json!({"pageid": id, "ns": ns, "title": title, "revisions": revisions});
        if let Some(redirect) = page.children().find(|node| node.has_tag_name("redirect")) {
            row["redirect"] = json!(true);
            row["redirect_target"] = json!(redirect.attribute("title"));
        }
        normalize_page(&row, &export_context, observed_at, &mut out)?;
    }
    if out.pages.is_empty() {
        return Err(SourcesError::invalid_input(
            "MediaWiki-Export enthält keine Seiten",
        ));
    }
    for document in &mut out.documents {
        document["metadata"]["provenance_capture_format"] = json!("mediawiki_export_xml");
    }
    let mut pages = BTreeMap::new();
    for page in std::mem::take(&mut out.pages) {
        merge_page(&mut pages, page);
    }
    out.pages = pages.into_values().collect();
    Ok(out)
}

fn child_text<'a, 'input>(node: roxmltree::Node<'a, 'input>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|child| child.has_tag_name(name))
        .and_then(|child| child.text())
}

#[derive(Default)]
struct Dependencies {
    templates: Vec<String>,
    modules: Vec<String>,
    categories: Vec<String>,
    redirect_target: Option<String>,
}

fn extract_dependencies(content: &str) -> Result<Dependencies> {
    let template = Regex::new(r"\{\{\s*([^{}|\n]+)")?;
    let category = Regex::new(r"(?i)\[\[Category:([^\]|]+)")?;
    let redirect = Regex::new(r"(?i)^\s*#redirect\s*\[\[([^\]|]+)")?;
    let mut templates = BTreeSet::new();
    let mut modules = BTreeSet::new();
    for captures in template.captures_iter(content) {
        let name = captures[1].trim();
        if let Some(module) = name.strip_prefix("#invoke:") {
            modules.insert(format!("Module:{}", module.trim()));
        } else if !name.starts_with('#') && !name.is_empty() {
            templates.insert(if name.contains(':') {
                name.into()
            } else {
                format!("Template:{name}")
            });
        }
    }
    Ok(Dependencies {
        templates: templates.into_iter().collect(),
        modules: modules.into_iter().collect(),
        categories: category
            .captures_iter(content)
            .map(|captures| captures[1].trim().to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        redirect_target: redirect
            .captures(content)
            .map(|captures| captures[1].trim().into()),
    })
}

fn history_sections(content: &str) -> Result<Vec<String>> {
    Ok(Regex::new(r"(?m)^={2,6}\s*(.*?)\s*={2,6}\s*$")?
        .captures_iter(content)
        .filter_map(|captures| {
            let heading = captures[1].trim();
            is_historical(heading).then(|| heading.to_string())
        })
        .collect())
}

fn is_historical(text: &str) -> bool {
    let lower = text.to_lowercase().replace('_', " ");
    [
        "update history",
        "version history",
        "patch history",
        "patchnotes",
        "patch notes",
        "changelog",
        "removed",
        "deprecated",
        "obsolete",
        "historical",
        "/history",
        "updates/",
        "update:",
    ]
    .iter()
    .any(|word| lower.contains(word))
}

fn page_kind(namespace: Option<i64>, context: &WikiSourceContext, redirect: bool) -> &'static str {
    if redirect {
        return "redirect";
    }
    match namespace
        .and_then(|id| context.namespaces.get(&id))
        .map(|name| name.to_ascii_lowercase())
        .as_deref()
    {
        Some("") => "article",
        Some("template") => "template",
        Some("category") => "category",
        Some("module") => "module",
        Some("data") => "data",
        Some("bucket") => "bucket_data",
        Some("update") => "historical_update",
        Some("file") | Some("image") => "file_description",
        Some(_) => "other_namespace",
        None => "unknown_namespace",
    }
}

fn flatten_data(
    value: &Value,
    pointer: &str,
    document_id: &str,
    revision: &str,
    historical: bool,
    facts: &mut Vec<Value>,
    depth: usize,
) -> Result<()> {
    if depth > 32 || facts.len() > 10_000 {
        return Err(SourcesError::invalid_input(
            "Wiki-Data-Struktur überschreitet die Faktengrenzen",
        ));
    }
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                flatten_data(
                    value,
                    &format!("{pointer}/{escaped}"),
                    document_id,
                    revision,
                    historical,
                    facts,
                    depth + 1,
                )?;
            }
        }
        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                flatten_data(
                    value,
                    &format!("{pointer}/{index}"),
                    document_id,
                    revision,
                    historical,
                    facts,
                    depth + 1,
                )?;
            }
        }
        _ => facts.push(json!({
            "fact_id": format!("json:{}", sha256(pointer.as_bytes())),
            "subject": document_id,
            "predicate": "wiki.data.value",
            "value": value,
            "unit": null,
            "evidence_status": "extracted_value",
            "source_span": format!("revision:{revision}:json{pointer}"),
            "qualifiers": {
                "json_pointer": pointer,
                "historical": historical,
                "game_patch_verified": false,
                "unit_inferred": false,
            },
        })),
    }
    Ok(())
}

fn gap(out: &mut WikiNormalization, id: &str, title: &str, reason: &str) {
    let gap = WikiGap {
        document_id: Some(id.into()),
        title: Some(title.into()),
        reason: reason.into(),
    };
    if !out.gaps.contains(&gap) {
        out.gaps.push(gap);
    }
}

fn page_locator(context: &WikiSourceContext, id: Option<i64>, title: &str) -> String {
    if let Some(base) = &context.source_article_base {
        let encoded = form_urlencode(&[("title", title.replace(' ', "_"))]);
        let title = encoded.strip_prefix("title=").unwrap_or(&encoded);
        let separator = if base.contains('?') { '&' } else { '?' };
        return match id {
            Some(id) => format!("{base}{title}{separator}curid={id}"),
            None if base.contains('?') => format!("{base}{title}"),
            None => format!("{base}{title}?title={title}"),
        };
    }
    id.map(|id| format!("{}/index.php?curid={id}", context.source_origin))
        .unwrap_or_else(|| {
            format!(
                "{}/index.php?{}",
                context.source_origin,
                form_urlencode(&[("title", title.into())])
            )
        })
}

fn flag(value: Option<&Value>) -> bool {
    value.is_some_and(|value| value.as_bool().unwrap_or(!value.is_null()))
}

fn text(value: Option<&Value>) -> Option<&str> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn newer_revision(new: &str, previous: Option<&str>) -> bool {
    match (
        new.parse::<u64>(),
        previous.and_then(|value| value.parse::<u64>().ok()),
    ) {
        (Ok(new), Some(previous)) => new >= previous,
        (Ok(_), None) => true,
        (Err(_), Some(_)) => false,
        (Err(_), None) => true,
    }
}

pub(crate) fn validate_observed_at(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    let valid_shape = bytes.len() >= 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes.last() == Some(&b'Z')
        && [0..4, 5..7, 8..10, 11..13, 14..16, 17..19]
            .iter()
            .all(|range| bytes[range.clone()].iter().all(u8::is_ascii_digit))
        && (bytes.len() == 20
            || (bytes.len() > 21
                && bytes[19] == b'.'
                && bytes[20..bytes.len() - 1].iter().all(u8::is_ascii_digit)));
    if !valid_shape {
        return Err(SourcesError::invalid_input(
            "Wiki-Beobachtungszeit muss eine UTC-Zeit nach RFC 3339 sein",
        ));
    }
    let number = |start: usize, end: usize| value[start..end].parse::<u32>().unwrap_or(u32::MAX);
    let year = number(0, 4);
    let month = number(5, 7);
    let day = number(8, 10);
    let days = match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0,
    };
    if day == 0 || day > days || number(11, 13) > 23 || number(14, 16) > 59 || number(17, 19) > 59 {
        return Err(SourcesError::invalid_input(
            "Ungültige Wiki-Beobachtungszeit",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod precision_tests {
    use super::*;

    fn context() -> WikiSourceContext {
        WikiSourceContext::from_mediawiki_export("<mediawiki><siteinfo><base>https://deadlock.wiki/Data:Heroes</base><namespaces><namespace key=\"0\"></namespace><namespace key=\"3500\">Data</namespace></namespaces></siteinfo></mediawiki>").unwrap()
    }

    fn payload(content: &str) -> Value {
        json!({"query":{"pages":[{"pageid":2880,"ns":3500,"title":"Data:Heroes","revisions":[{"revid":18108,"slots":{"main":{"contentmodel":"json","content":content}}}]}]}})
    }

    #[test]
    fn ambiguous_data_retains_original_without_extracted_facts() {
        for content in [
            r#"{"value":1.2300,"value":2}"#,
            r#"{"nested":[{"value":1,"\u0076alue":2}]}"#,
        ] {
            let normalized =
                normalize_api_response(&payload(content), &context(), "2026-10-04T12:00:00Z")
                    .unwrap();
            assert_eq!(normalized.documents[0]["content"], content);
            assert_eq!(
                normalized.documents[0]["content_sha256"],
                sha256(content.as_bytes())
            );
            assert!(normalized.documents[0]["facts"]
                .as_array()
                .unwrap()
                .is_empty());
            assert!(normalized
                .gaps
                .iter()
                .any(|gap| gap.reason == "data_json_invalid_or_ambiguous"));
        }
    }

    #[test]
    fn api_decode_rejects_duplicate_keys_and_preserves_numbers() {
        for text in [
            r#"{"query":{},"query":{}}"#,
            r#"{"query":{"pages":[{"revid":1,"revid":2}]}}"#,
        ] {
            assert!(super::super::decode_api_response(text.as_bytes()).is_err());
        }
        let text = r#"{"query":{"value":20.000010800000002,"large":18446744073709551616001}}"#;
        let value = super::super::decode_api_response(text.as_bytes()).unwrap();
        assert_eq!(value["query"]["value"].to_string(), "20.000010800000002");
        assert_eq!(
            value["query"]["large"].to_string(),
            "18446744073709551616001"
        );
    }

    #[test]
    fn extracts_and_raw_revisions_survive_retry_including_frozen_legacy() {
        let context = context();
        let observed_at = "2026-10-04T12:00:00Z";
        let raw = normalize_api_response(&payload(r#"{"value":1.2300}"#), &context, observed_at)
            .unwrap()
            .documents
            .remove(0);
        let mut rendered = payload("");
        rendered["query"]["pages"][0]["revisions"][0]["slots"] = Value::Null;
        rendered["query"]["pages"][0]["extract"] = json!("Gerenderter Wert");
        let extract = normalize_api_response(&rendered, &context, observed_at)
            .unwrap()
            .documents
            .remove(0);
        assert_ne!(extract["revision"], raw["revision"]);
        for legacy in [false, true] {
            let root = tempfile::tempdir().unwrap();
            let store = super::super::storage::WikiSpool::open(root.path(), 1024 * 1024).unwrap();
            let mut state = super::super::Checkpoint {
                context: Some(context.clone()),
                ..Default::default()
            };
            store
                .bind_source(&mut state, Some(&context.source_origin))
                .unwrap();
            let mut frozen = extract.clone();
            if legacy {
                frozen["revision"] = raw["revision"].clone();
            }
            store.persist_document(&frozen).unwrap();
            let path = std::fs::read_dir(root.path().join("documents"))
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .path();
            let original_bytes = std::fs::read(&path).unwrap();
            for _ in 0..2 {
                store.persist_document(&raw).unwrap();
                store.persist_document(&frozen).unwrap();
                let report = store.publish(&state).unwrap();
                let text = std::fs::read_to_string(report.documents_path).unwrap();
                let documents: Vec<Value> = text
                    .lines()
                    .map(|line| serde_json::from_str(line).unwrap())
                    .collect();
                assert_eq!(documents.len(), 2);
                assert!(documents.iter().any(|document| document == &raw));
                assert!(documents
                    .iter()
                    .any(|document| document["revision"] == extract["revision"]
                        && document["content"] == extract["content"]));
                assert_eq!(std::fs::read(&path).unwrap(), original_bytes);
            }
        }
    }

    #[test]
    fn data_numbers_survive_normalization_spool_and_republication() {
        let content = r#"{"hero_atlas":{"FalloffStartRange":20.000010800000002},"hero_ghost":{"DPS":55.555555555555564},"long":0.123456789012345678901234567890,"integer":184467440737095516160,"trailing":1.2300,"exponent":1.2500E003,"negative_zero":-0}"#;
        let xml = format!(
            "<mediawiki><siteinfo><base>https://deadlock.wiki/Data:Heroes</base><namespaces><namespace key=\"0\"></namespace><namespace key=\"3500\">Data</namespace></namespaces></siteinfo><page><title>Data:Heroes</title><ns>3500</ns><id>2880</id><revision><id>18108</id><timestamp>2025-04-15T00:00:00Z</timestamp><model>json</model><text>{content}</text></revision></page></mediawiki>"
        );
        let context = WikiSourceContext::from_mediawiki_export(&xml).unwrap();
        let options = super::super::WikiInventoryOptions {
            observed_at: "2026-10-03T11:23:00Z".into(),
            ..Default::default()
        };
        let root = tempfile::tempdir().unwrap();
        let expected: Value = serde_json::from_str(content).unwrap();
        let expected_numbers = [
            ("/hero_atlas/FalloffStartRange", "20.000010800000002"),
            ("/hero_ghost/DPS", "55.555555555555564"),
            ("/long", "0.123456789012345678901234567890"),
            ("/integer", "184467440737095516160"),
            ("/trailing", "1.2300"),
            ("/exponent", "1.2500e+003"),
            ("/negative_zero", "0"),
        ];
        for _ in 0..2 {
            let report =
                super::super::write_wiki_export_capture(root.path(), &xml, &context, &options)
                    .unwrap();
            let bytes = std::fs::read(&report.documents_path).unwrap();
            let document: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(document["content"], content);
            assert_eq!(document["content_sha256"], sha256(content.as_bytes()));
            assert_eq!(
                document["facts"].as_array().unwrap().len(),
                expected_numbers.len()
            );
            let roundtrip: Value = serde_json::from_value(document.clone()).unwrap();
            assert_eq!(roundtrip, document);
            for (pointer, representation) in expected_numbers {
                let fact = document["facts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|fact| fact["qualifiers"]["json_pointer"] == pointer)
                    .unwrap();
                assert_eq!(&fact["value"], expected.pointer(pointer).unwrap());
                let number = fact["value"].as_number().unwrap();
                assert_eq!(number.as_str(), representation);
                assert_eq!(serde_json::to_string(number).unwrap(), representation);
            }
        }
    }
}
