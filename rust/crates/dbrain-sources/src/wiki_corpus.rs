//! Vollständiger Textimport des Artikel-Namensraums. Nur ein erfolgreicher
//! Source-Run veröffentlicht die Mitgliederliste; Teilimporte bleiben unsichtbar.
use std::{collections::BTreeSet, fs::OpenOptions, path::Path};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions, RetryPolicy};
use scraper::{ElementRef, Html, Node};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    store::{complete_run, json_bytes, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    util::form_urlencode,
    wiki::{WikiRateLimiter, DEFAULT_API_URL, SOURCE},
    Result, SourcesError,
};

pub const RUN_SOURCE: &str = "deadlock_wiki_corpus";

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WikiCorpusOptions {
    pub enabled: bool,
    pub min_delay_seconds: f64,
    pub cache_ttl_seconds: u64,
    pub max_pages: usize,
    pub max_response_bytes: usize,
    pub max_total_bytes: usize,
}

impl Default for WikiCorpusOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            min_delay_seconds: 5.0,
            cache_ttl_seconds: 604_800,
            max_pages: 20_000,
            max_response_bytes: 4 * 1024 * 1024,
            max_total_bytes: 256 * 1024 * 1024,
        }
    }
}

impl WikiCorpusOptions {
    fn validate(&self) -> Result<()> {
        if !self.enabled {
            return Err(SourcesError::invalid_input(
                "Wiki-Netzwerkzugriff ist deaktiviert",
            ));
        }
        if !self.min_delay_seconds.is_finite()
            || self.min_delay_seconds < 5.0
            || self.max_pages == 0
            || self.max_response_bytes == 0
            || self.max_total_bytes < self.max_response_bytes
        {
            return Err(SourcesError::invalid_input(
                "Ungültige Wiki-Grenzen; mindestens fünf Sekunden Abrufabstand sind erforderlich",
            ));
        }
        Ok(())
    }
}

/// Nutzt den vorhandenen Pool, HTTP-Cache und SourceStore, keinen zweiten Dienst.
pub async fn pull_wiki_corpus_with_pool(
    pool: &PgPool,
    raw_dir: &Path,
    http: &HttpClient,
    options: &WikiCorpusOptions,
) -> Result<Value> {
    options.validate()?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(http.cache_dir().join("wiki-corpus.lock"))?;
    lock.try_lock().map_err(|error| {
        SourcesError::invalid_input(format!("Wiki-Import läuft bereits: {error}"))
    })?;
    let store = SourceStore::new(pool, raw_dir)?;
    let run_id = store.begin_run(RUN_SOURCE).await?;
    let outcome = async {
        let worker_http = http.clone();
        let worker_options = options.clone();
        let worker_lock = lock.try_clone()?;
        let pages = tokio::task::spawn_blocking(move || {
        // Netzwerk und Rate-Limit-Schlaf dürfen keinen Tokio-Worker blockieren.
        // Der Lock bleibt bei Abbruch des Aufrufers bis zum Worker-Ende gehalten.
        let http = worker_http;
        let options = worker_options;
        let _lock = worker_lock;
        let limiter = WikiRateLimiter {
            state_path: http.cache_dir().join("wiki_last_request.txt"),
            min_delay_seconds: options.min_delay_seconds,
        };
        collect_corpus(&options, |params| {
            let revision_with_template_stamp = params.iter().any(|(key, _)| *key == "oldid");
            let url = format!("{DEFAULT_API_URL}?{}", form_urlencode(params));
            let cached = if revision_with_template_stamp && options.cache_ttl_seconds > 0 {
                http.cached_result(&url, options.cache_ttl_seconds)?
                    .filter(|response| serde_json::from_slice::<Value>(&response.content)
                        .is_ok_and(|value| api_ok(&value).is_ok()))
            } else { None };
            let response = if let Some(response) = cached { response } else {
                limiter.wait()?;
                // Discovery muss neue, umbenannte und entfernte Seiten sehen.
                // API-Fehler aus einem früheren HTTP-200 werden nicht wiederverwendet.
                http.get(&url, HttpGetOptions {
                    cache_ttl_seconds: Some(0),
                    retry: RetryPolicy { attempts: 1, ..Default::default() },
                    ..Default::default()
                })?
            };
            if response.content.len() > options.max_response_bytes {
                return Err(SourcesError::invalid_input("Wiki-Antwort überschreitet das Größenlimit"));
            }
            Ok(serde_json::from_slice(&response.content)?)
        })
        }).await.map_err(|error| SourcesError::invariant(format!("Wiki-Abruf-Worker fehlgeschlagen: {error}")))??;
        let mut snapshot_ids = Vec::with_capacity(pages.len());
        for page in &pages {
            let external_id = page["_wiki"]["page_id"].as_i64()
                .ok_or_else(|| SourcesError::invariant("Wiki-Seiten-ID fehlt"))?.to_string();
            let title = page["title"].as_str().unwrap_or_default();
            let raw = json_bytes(page)?;
            let path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
            let document_id = store.upsert_source_document(SourceDocumentInput {
                source: SOURCE, external_id: &external_id, title: Some(title),
                url: page["_wiki"]["revision_url"].as_str(), content_type: "application/json",
                raw_path: &path, content: &raw, metadata: &page["_wiki"],
            }).await?;
            snapshot_ids.push(store.upsert_entity_snapshot_id(&EntitySnapshotInput {
                source: SOURCE.into(), entity_type: "wiki_page".into(),
                external_id, canonical_name: Some(title.into()), payload: page.clone(),
            }, Some(document_id)).await?);
        }
        Ok(json!({
            "schema_version": 1, "scope": "main_namespace_nonredirect_articles",
            "complete": true, "pages": pages.len(), "snapshot_ids": snapshot_ids,
            "license": pages.first().map(|page| &page["_wiki"]["license"]),
            "limitations": ["Text einschließlich gerenderter Vorlagen und Tabellen; keine Medien-Dateien.", "Ein Wiki-Revisionsdatum bestätigt keinen aktuellen Spiel-Patch."]
        }))
    }.await;
    complete_run(&store, run_id, outcome).await
}

/// Der Transport ist austauschbar für deterministische, netzwerkfreie Vertragstests.
fn collect_corpus(
    options: &WikiCorpusOptions,
    mut get: impl FnMut(&[(&str, String)]) -> Result<Value>,
) -> Result<Vec<Value>> {
    options.validate()?;
    let site = get(&[
        ("action", "query".into()),
        ("format", "json".into()),
        ("formatversion", "2".into()),
        ("meta", "siteinfo".into()),
        ("siprop", "rightsinfo".into()),
        ("maxlag", "5".into()),
    ])?;
    api_ok(&site)?;
    let license = site
        .pointer("/query/rightsinfo")
        .filter(|rights| nonempty(rights.get("text")) && nonempty(rights.get("url")))
        .ok_or_else(|| {
            SourcesError::invalid_input("Wiki-Lizenzangaben fehlen; kein unattribuierter Import")
        })?
        .clone();
    let mut continuation = None::<Value>;
    let mut cursors = BTreeSet::new();
    let mut ids = BTreeSet::new();
    let mut pages = Vec::new();
    let mut total_bytes = 0usize;
    loop {
        let mut params = vec![
            ("action", "query".into()),
            ("format", "json".into()),
            ("formatversion", "2".into()),
            ("generator", "allpages".into()),
            ("gapnamespace", "0".into()),
            ("gapfilterredir", "nonredirects".into()),
            ("gaplimit", "50".into()),
            ("prop", "revisions|info".into()),
            ("rvprop", "ids|timestamp".into()),
            ("maxlag", "5".into()),
        ];
        if let Some(cursor) = &continuation {
            for key in ["continue", "gapcontinue"] {
                if let Some(value) = cursor[key].as_str() {
                    params.push((key, value.into()));
                }
            }
        }
        let listing = get(&params)?;
        api_ok(&listing)?;
        let empty = Vec::new();
        let batch = match listing.pointer("/query/pages") {
            Some(value) => value
                .as_array()
                .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Seitenliste"))?,
            None if listing.get("continue").is_some()
                || (!pages.is_empty() && listing["batchcomplete"] == true) =>
            {
                &empty
            }
            None => return Err(SourcesError::invalid_input("Wiki-Seitenliste fehlt")),
        };
        for page in batch {
            let id = positive_id(page.get("pageid"))?;
            let title = page["title"]
                .as_str()
                .filter(|s| {
                    !s.trim().is_empty()
                        && !s
                            .chars()
                            .any(|ch| ch.is_control() || matches!(ch, '<' | '>'))
                })
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Titel fehlt"))?;
            if page["ns"].as_i64() != Some(0) || page.get("missing").is_some() || !ids.insert(id) {
                return Err(SourcesError::invalid_input(
                    "Ungültige oder doppelte Wiki-Seite",
                ));
            }
            if ids.len() > options.max_pages {
                return Err(SourcesError::invalid_input(
                    "Wiki-Seitenlimit erreicht; kein vollständiger Import",
                ));
            }
            let revision = &page["revisions"][0];
            let revid = positive_id(revision.get("revid"))?;
            let timestamp = revision["timestamp"]
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Revisionsdatum fehlt"))?;
            // Vorlagen können sich ohne neue Artikelrevision ändern. touched
            // wird deshalb Teil des Cache-Schlüssels über den API-requestid.
            let touched = page["touched"]
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Touched-Datum fehlt"))?;
            let parsed = get(&[
                ("requestid", touched.into()),
                ("action", "parse".into()),
                ("format", "json".into()),
                ("formatversion", "2".into()),
                ("oldid", revid.to_string()),
                ("prop", "text|categories|revid".into()),
                ("disableeditsection", "1".into()),
                ("maxlag", "5".into()),
            ])?;
            api_ok(&parsed)?;
            if parsed["parse"]["revid"].as_i64() != Some(revid)
                || parsed["parse"]["pageid"].as_i64() != Some(id)
                || parsed["parse"]["title"].as_str() != Some(title)
            {
                return Err(SourcesError::invalid_input(
                    "Wiki-Seite änderte sich während des Imports; erneut synchronisieren",
                ));
            }
            let html = parsed["parse"]["text"]
                .as_str()
                .ok_or_else(|| SourcesError::invalid_input("Gerenderter Wiki-Text fehlt"))?;
            if html.len() > options.max_response_bytes {
                return Err(SourcesError::invalid_input(
                    "Wiki-Text überschreitet das Größenlimit",
                ));
            }
            let categories = parsed["parse"]["categories"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|category| {
                    category["category"]
                        .as_str()
                        .or_else(|| category["*"].as_str())
                })
                .map(str::to_string)
                .collect::<Vec<_>>();
            let historical = is_historical(title) || categories.iter().any(|c| is_historical(c));
            let sections = extract_sections(html);
            if sections.iter().all(|section| section.text.is_empty()) {
                return Err(SourcesError::invalid_input(
                    "Leere Wiki-Seite; kein stiller Inhaltsverlust",
                ));
            }
            let normalized = json!({
                "title": title, "categories": categories, "sections": sections,
                "_wiki": {
                    "schema_version": 1, "page_id": id, "revision_id": revid,
                    "revision_timestamp": timestamp, "page_touched": touched,
                    "rendered_templates_pinned": false,
                    "url": format!("https://deadlock.wiki/index.php?{}", form_urlencode(&[("title", title.to_string())])),
                    "revision_url": format!("https://deadlock.wiki/index.php?oldid={revid}"),
                    "license": license, "attribution": "Deadlock Wiki contributors",
                    "historical": historical, "patch_verified": false,
                    "content_is_untrusted_data": true
                }
            });
            // Auch Kategorien, Lizenz und Provenienz belegen Korpusspeicher.
            total_bytes = total_bytes.saturating_add(serde_json::to_vec(&normalized)?.len());
            if total_bytes > options.max_total_bytes {
                return Err(SourcesError::invalid_input(
                    "Normalisierter Wiki-Korpus überschreitet das Größenlimit",
                ));
            }
            pages.push(normalized);
        }
        continuation = listing.get("continue").cloned();
        let Some(cursor) = &continuation else { break };
        if !nonempty(cursor.get("gapcontinue")) || !cursors.insert(cursor.to_string()) {
            return Err(SourcesError::invalid_input(
                "Ungültige oder wiederholte Wiki-Paginierung",
            ));
        }
    }
    if pages.is_empty() {
        return Err(SourcesError::invalid_input("Wiki-Korpus ist leer"));
    }
    Ok(pages)
}

fn nonempty(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}
fn positive_id(value: Option<&Value>) -> Result<i64> {
    value
        .and_then(Value::as_i64)
        .filter(|id| *id > 0)
        .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-ID"))
}
fn api_ok(value: &Value) -> Result<()> {
    if !value.is_object()
        || value.get("error").is_some()
        || value.get("errors").is_some()
        || value.get("warnings").is_some()
    {
        return Err(SourcesError::invalid_input(
            "Wiki-API meldet Fehler oder unvollständige Daten",
        ));
    }
    Ok(())
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
        "unreleased",
        "historical",
        "/history",
        "updates/",
    ]
    .iter()
    .any(|word| lower.contains(word))
}

#[derive(Debug, Serialize)]
struct Section {
    heading: String,
    kind: &'static str,
    text: String,
}

fn extract_sections(html: &str) -> Vec<Section> {
    fn visit(
        element: ElementRef<'_>,
        sections: &mut Vec<Section>,
        headings: &mut Vec<(u8, String)>,
    ) {
        let name = element.value().name();
        if matches!(
            name,
            "script" | "style" | "nav" | "noscript" | "form" | "iframe"
        ) || element
            .value()
            .classes()
            .any(|c| matches!(c, "mw-editsection" | "navbox" | "toc" | "navigation"))
        {
            return;
        }
        if name == "img" {
            if let Some(alt) = element
                .value()
                .attr("alt")
                .filter(|alt| !alt.trim().is_empty())
            {
                let out = &mut sections.last_mut().unwrap().text;
                out.push_str(alt);
                out.push(' ');
            }
            return;
        }
        if matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
            let level = name.as_bytes()[1];
            let heading = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            while headings.last().is_some_and(|(old, _)| *old >= level) {
                headings.pop();
            }
            headings.push((level, heading));
            let heading = headings
                .iter()
                .map(|(_, h)| h.as_str())
                .collect::<Vec<_>>()
                .join(" / ");
            let lower = heading.to_lowercase();
            let kind = if is_historical(&heading) {
                "history"
            } else if ["lore", "background", "biography", "trivia"]
                .iter()
                .any(|word| lower.contains(word))
            {
                "lore"
            } else if ["strategy", "synerg", "counter", "tips", "build"]
                .iter()
                .any(|word| lower.contains(word))
            {
                "strategy"
            } else if ["abilit", "mechanic", "stat", "weapon", "scaling"]
                .iter()
                .any(|word| lower.contains(word))
            {
                "mechanics"
            } else {
                "overview"
            };
            sections.push(Section {
                heading,
                kind,
                text: String::new(),
            });
            return;
        }
        for child in element.children() {
            match child.value() {
                Node::Text(text) => {
                    let out = &mut sections.last_mut().unwrap().text;
                    out.push_str(text);
                    out.push(' ');
                }
                Node::Element(_) => {
                    if let Some(child) = ElementRef::wrap(child) {
                        visit(child, sections, headings);
                    }
                }
                _ => {}
            }
        }
        if matches!(name, "p" | "div" | "li" | "tr" | "br" | "dl") {
            sections.last_mut().unwrap().text.push('\n');
        }
        if matches!(name, "td" | "th") {
            sections.last_mut().unwrap().text.push_str(" | ");
        }
    }
    let document = Html::parse_fragment(html);
    let mut sections = vec![Section {
        heading: "Overview".into(),
        kind: "overview",
        text: String::new(),
    }];
    visit(document.root_element(), &mut sections, &mut Vec::new());
    for section in &mut sections {
        section.text = section
            .text
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
    }
    sections.retain(|section| !section.text.is_empty());
    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn listing(id: i64) -> Value {
        json!({"query":{"pages":[{"pageid":id,"ns":0,"title":format!("Hero {id}"),"touched":"2026-09-24T00:00:00Z","revisions":[{"revid":100+id,"timestamp":"2026-09-20T00:00:00Z"}]}]}})
    }
    fn parsed(id: i64) -> Value {
        json!({"parse":{"pageid":id,"title":format!("Hero {id}"),"revid":100+id,"text":"<p>Hero facts</p>","categories":[]}})
    }
    fn options() -> WikiCorpusOptions {
        WikiCorpusOptions {
            enabled: true,
            ..Default::default()
        }
    }
    fn site() -> Value {
        json!({"query":{"rightsinfo":{"text":"Test license","url":"https://example.test/license"}}})
    }
    fn run(replies: Vec<Value>, options: WikiCorpusOptions) -> Result<Vec<Value>> {
        let mut replies = VecDeque::from(replies);
        collect_corpus(&options, |_| {
            Ok(replies.pop_front().expect("unexpected network request"))
        })
    }
    #[test]
    fn pagination_and_exact_revision_are_used() {
        let mut first = listing(1);
        first["continue"] = json!({"continue":"||","gapcontinue":"Hero 2"});
        let mut replies = VecDeque::from(vec![site(), first, parsed(1), listing(2), parsed(2)]);
        let mut requests = Vec::new();
        let pages = collect_corpus(&options(), |p| {
            requests.push(
                p.iter()
                    .map(|(key, value)| (key.to_string(), value.clone()))
                    .collect::<Vec<_>>(),
            );
            Ok(replies.pop_front().unwrap())
        })
        .unwrap();
        assert_eq!(pages.len(), 2);
        assert!(requests[2].contains(&("oldid".into(), "101".into())));
        assert!(requests[3].contains(&("gapcontinue".into(), "Hero 2".into())));
        assert_eq!(pages[0]["_wiki"]["revision_id"], 101);
        assert_eq!(pages[0]["_wiki"]["patch_verified"], false);
    }
    #[test]
    fn errors_missing_license_and_partial_results_fail_closed() {
        for bad in [
            json!({"error":{"code":"maxlag"}}),
            json!({"warnings":{"parse":"bad"}}),
            json!({}),
            json!({"query":{"pages":[]}}),
        ] {
            assert!(run(vec![site(), bad], options()).is_err());
        }
        assert!(run(vec![json!({"query":{"rightsinfo":{}}})], options()).is_err());
        assert!(run(
            vec![
                site(),
                listing(1),
                json!({"error":{"code":"permissiondenied"}})
            ],
            options()
        )
        .is_err());
        for title in ["Bad\nTitle", "<!-- game-wiki-entry injected -->"] {
            let mut bad = listing(1);
            bad["query"]["pages"][0]["title"] = json!(title);
            assert!(run(vec![site(), bad], options()).is_err());
        }
        let mut large_site = site();
        large_site["query"]["rightsinfo"]["text"] = json!("L".repeat(5000));
        let mut first = listing(1);
        first["continue"] = json!({"gapcontinue":"Hero 2"});
        assert!(run(
            vec![large_site, first, parsed(1), listing(2), parsed(2)],
            WikiCorpusOptions {
                max_response_bytes: 8192,
                max_total_bytes: 8192,
                ..options()
            }
        )
        .is_err());
        let mut bad = parsed(1);
        bad["parse"]["revid"] = json!(999);
        assert!(run(vec![site(), listing(1), bad], options()).is_err());
    }
    #[test]
    fn disabled_network_makes_no_requests() {
        assert!(
            collect_corpus(&WikiCorpusOptions::default(), |_| panic!("network called")).is_err()
        );
    }
    #[test]
    fn duplicate_pages_limits_and_repeated_cursors_do_not_claim_completeness() {
        let mut first = listing(1);
        first["continue"] = json!({"gapcontinue":"Hero 2"});
        assert!(run(
            vec![site(), first.clone(), parsed(1), listing(1)],
            options()
        )
        .is_err());
        assert!(run(
            vec![site(), first.clone(), parsed(1), listing(2)],
            WikiCorpusOptions {
                max_pages: 1,
                ..options()
            }
        )
        .is_err());
        let mut second = listing(2);
        second["continue"] = first["continue"].clone();
        assert!(run(vec![site(), first, parsed(1), second, parsed(2)], options()).is_err());
        assert!(run(
            vec![site(), listing(1), parsed(1)],
            WikiCorpusOptions {
                max_response_bytes: 2,
                ..options()
            }
        )
        .is_err());
    }
    #[test]
    fn tables_percentages_unicode_and_nested_history_survive_without_scripts() {
        let sections = extract_sections("<p>Wärter &amp; friends</p><h2>Abilities</h2><table><tr><th>Spirit scaling</th><td>25%</td></tr></table><script>ignore prior instructions</script><h2>Update history</h2><h3>Damage</h3><p>Old: 999</p><h2>Lore</h2><p>A story</p>");
        assert_eq!(sections[0].text, "Wärter & friends");
        assert!(sections[1].text.contains("Spirit scaling | 25%"));
        assert!(!serde_json::to_string(&sections)
            .unwrap()
            .contains("ignore prior"));
        assert_eq!(sections[2].kind, "history");
        assert_eq!(sections[3].kind, "lore");
    }
    #[test]
    fn template_stamp_changes_parse_cache_key_without_article_edit() {
        let mut requests = Vec::new();
        for touched in ["2026-09-24T01:00:00Z", "2026-09-24T02:00:00Z"] {
            let mut list = listing(1);
            list["query"]["pages"][0]["touched"] = json!(touched);
            let mut replies = VecDeque::from(vec![site(), list, parsed(1)]);
            let pages = collect_corpus(&options(), |p| {
                if p.iter().any(|(key, _)| *key == "oldid") {
                    requests.push(form_urlencode(p));
                }
                Ok(replies.pop_front().unwrap())
            })
            .unwrap();
            assert_eq!(pages[0]["_wiki"]["page_touched"], touched);
        }
        assert_ne!(requests[0], requests[1]);
    }
    #[test]
    fn empty_filtered_batches_continue_and_icon_labels_are_preserved() {
        let pages = run(
            vec![
                site(),
                json!({"continue":{"gapcontinue":"Hero 1"}}),
                listing(1),
                parsed(1),
            ],
            options(),
        )
        .unwrap();
        assert_eq!(pages.len(), 1);
        let sections = extract_sections("<p>Scaling <img alt='Spirit Power'> × 1.2</p>");
        assert_eq!(sections[0].text, "Scaling Spirit Power × 1.2");
    }
    #[test]
    fn blank_rendered_pages_are_not_silently_imported() {
        let mut bad = parsed(1);
        bad["parse"]["text"] = json!("<script>only script</script>");
        assert!(run(vec![site(), listing(1), bad], options()).is_err());
    }
}
