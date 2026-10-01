//! Finite MediaWiki pilot. Namespace IDs are filters, never crawl instructions.
//! No allpages, redirect expansion, dependency traversal, parse/expandtemplates,
//! Lua execution, retries or partial-success publication is permitted here.
use crate::{
    discovery::{api_problem, values},
    model::*,
    parse_json, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    thread,
    time::{Duration, Instant},
};

pub type Params = BTreeMap<String, String>;
pub const MAX_PILOT_PAGES: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureOptions {
    pub source_key: String,
    pub retrieved_at: i64,
    pub policy: CapturePolicy,
    pub scope: WikiScope,
    pub max_requests: usize,
    pub max_pages: usize,
    pub max_total_bytes: usize,
    /// Minimum gap between request starts. Live I/O additionally requires >=5s.
    pub request_interval_ms: u64,
    /// Wall-clock budget for the whole capture, including pacing and responses.
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct RequestLimit {
    /// Remaining aggregate entity-byte budget, also enforced while streaming.
    pub max_bytes: usize,
    pub timeout: Duration,
}

fn stable(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 512 && !s.chars().any(char::is_control)
}
fn title(s: &str) -> bool {
    stable(s) && !s.contains(['|', '*', '#', '[', ']'])
}
fn title_key(s: &str) -> String {
    s.replace('_', " ").trim().to_string()
}

pub fn validate_scope(scope: &WikiScope) -> Result<()> {
    if scope.namespace_allowlist.is_empty()
        || scope.namespace_allowlist.len() > 8
        || scope.namespace_allowlist.iter().any(|id| *id < 0)
        || scope.page_ids.iter().any(|id| *id <= 0)
        || scope.page_ids.len() > MAX_PILOT_PAGES
        || scope.pages.len() > MAX_PILOT_PAGES
        || scope.categories.len() > 8
        || scope.heroes.len() > 8
        || scope.pages.iter().any(|s| !title(s))
        || scope
            .categories
            .iter()
            .any(|s| !title(s) || !s.starts_with("Category:") || s.len() <= 9)
        || scope.heroes.iter().any(|h| {
            !title(&h.title)
                || !stable(&h.entity_id)
                || !stable(&h.locale)
                || h.locale.len() > 32
                || h.pages.len() > MAX_PILOT_PAGES
                || h.pages.iter().any(|s| !title(s))
        })
        || (scope.page_ids.is_empty()
            && scope.pages.is_empty()
            && scope.categories.is_empty()
            && scope.heroes.is_empty())
    {
        return Err("invalid/empty explicit Wiki scope or namespace allowlist".into());
    }
    let mut heroes = BTreeSet::new();
    let mut entities = BTreeSet::new();
    for hero in &scope.heroes {
        if !heroes.insert(title_key(&hero.title)) || !entities.insert(&hero.entity_id) {
            return Err("duplicate hero scope/entity".into());
        }
    }
    Ok(())
}
impl CaptureOptions {
    pub fn validate(&self) -> Result<()> {
        validate_scope(&self.scope)?;
        if self.source_key.is_empty()
            || self.source_key.len() > 128
            || !self
                .source_key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            || !self.policy.offline_review_allowed
            || self
                .policy
                .decision_ref
                .as_deref()
                .is_none_or(|s| !stable(s))
            || self.retrieved_at <= 0
            || !(1..=512).contains(&self.max_requests)
            || !(1..=MAX_PILOT_PAGES).contains(&self.max_pages)
            || !(1..=MAX_INPUT_BYTES).contains(&self.max_total_bytes)
            || !(1..=3_600_000).contains(&self.deadline_ms)
            || self.request_interval_ms > 60_000
        {
            return Err("invalid or unapproved capture budget".into());
        }
        Ok(())
    }
}
fn params(pairs: &[(&str, &str)]) -> Params {
    let mut p: Params = pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    p.insert("action".into(), "query".into());
    p.insert("format".into(), "json".into());
    p.insert("formatversion".into(), "2".into());
    p.insert("maxlag".into(), "5".into());
    p
}
fn continuation(response: &Value, allowed: &[&str]) -> Result<Option<Params>> {
    let Some(value) = response.get("continue") else {
        return Ok(None);
    };
    let token: Params =
        serde_json::from_value(value.clone()).map_err(|_| "invalid continuation object")?;
    if token.is_empty()
        || token
            .iter()
            .any(|(k, v)| !allowed.contains(&k.as_str()) || v.is_empty() || v.len() > 4096)
        || !token.keys().any(|k| k != "continue")
    {
        return Err("unsafe/empty continuation token".into());
    }
    Ok(Some(token))
}
fn one_page(response: &Value, expected_id: Option<i64>) -> Result<Value> {
    let found = values(
        response
            .pointer("/query/pages")
            .ok_or("missing page response")?,
    );
    if found.len() != 1
        || found[0]["pageid"].as_i64().is_none_or(|id| id <= 0)
        || expected_id.is_some_and(|id| found[0]["pageid"].as_i64() != Some(id))
        || found[0].get("missing").is_some()
        || found[0].get("invalid").is_some()
    {
        return Err("page missing or identity changed".into());
    }
    Ok(found[0].clone())
}
fn insert_page(
    selected: &mut BTreeMap<i64, Value>,
    page: &Value,
    options: &CaptureOptions,
) -> Result<()> {
    let id = page["pageid"]
        .as_i64()
        .filter(|id| *id > 0)
        .ok_or("invalid page ID")?;
    let namespace = page["ns"].as_i64().ok_or("page namespace missing")?;
    if !options.scope.namespace_allowlist.contains(&namespace) {
        return Err("page outside namespace allowlist".into());
    }
    let name = page["title"]
        .as_str()
        .filter(|s| title(s))
        .ok_or("page title missing/invalid")?;
    if let Some(old) = selected.get(&id) {
        if old["title"].as_str() != Some(name) || old["ns"] != page["ns"] {
            return Err("unstable discovery identity".into());
        }
    } else {
        if selected.len() >= options.max_pages {
            return Err("page budget exhausted".into());
        }
        selected.insert(id, page.clone());
    }
    Ok(())
}

/// Fixture/contract convenience. Real HTTP callers must use collect_bytes so
/// whitespace/encoding is counted from received entity bytes, not normalized JSON.
pub fn collect(
    options: &CaptureOptions,
    mut get: impl FnMut(&Params) -> Result<Value>,
) -> Result<Capture> {
    collect_bytes(options, |p, _| {
        serde_json::to_vec(&get(p)?).map_err(|_| "response encoding".into())
    })
}

/// All requests, including discovery, history and dependency continuations, share
/// ONE budget. Transport must enforce RequestLimit before buffering/parsing.
/// Any error returns no completed capture. Tokens never override query selectors.
pub fn collect_bytes(
    options: &CaptureOptions,
    mut get: impl FnMut(&Params, RequestLimit) -> Result<Vec<u8>>,
) -> Result<Capture> {
    options.validate()?;
    let started = Instant::now();
    let deadline = Duration::from_millis(options.deadline_ms);
    let interval = Duration::from_millis(options.request_interval_ms);
    let mut last_request: Option<Instant> = None;
    let mut requests = 0usize;
    let mut total_bytes = 0usize;
    let mut request = |p: &Params| -> Result<Value> {
        if requests >= options.max_requests {
            return Err("request budget exhausted".into());
        }
        let remaining = options.max_total_bytes.saturating_sub(total_bytes);
        if remaining == 0 {
            return Err("aggregate capture byte budget exhausted".into());
        }
        let wait = last_request.map_or(Duration::ZERO, |at| interval.saturating_sub(at.elapsed()));
        if started.elapsed().saturating_add(wait) >= deadline {
            return Err("capture deadline exhausted".into());
        }
        if !wait.is_zero() {
            thread::sleep(wait);
        }
        let timeout = deadline.saturating_sub(started.elapsed());
        if timeout.is_zero() {
            return Err("capture deadline exhausted".into());
        }
        requests += 1;
        last_request = Some(Instant::now());
        let bytes = get(
            p,
            RequestLimit {
                max_bytes: remaining,
                timeout,
            },
        )?;
        if started.elapsed() >= deadline {
            return Err("capture deadline exhausted".into());
        }
        if bytes.len() > remaining {
            return Err("aggregate capture byte budget exhausted".into());
        }
        total_bytes += bytes.len();
        let value = parse_json(&bytes)?;
        if !value.is_object() || api_problem(&value) || value.get("query-continue").is_some() {
            return Err("API error/warning/legacy continuation; capture incomplete".into());
        }
        Ok(value)
    };
    let siteinfo = request(&params(&[
        ("meta", "siteinfo"),
        ("siprop", "general|namespaces|rightsinfo"),
    ]))?;
    let mut namespaces = BTreeSet::new();
    for ns in values(
        siteinfo
            .pointer("/query/namespaces")
            .ok_or("namespaces missing")?,
    ) {
        let id = ns["id"].as_i64().ok_or("invalid namespace ID")?;
        if !namespaces.insert(id) {
            return Err("duplicate namespace".into());
        }
    }
    if !namespaces.contains(&0) || !options.scope.namespace_allowlist.is_subset(&namespaces) {
        return Err("namespace allowlist contains an unknown namespace".into());
    }
    let mut selected = BTreeMap::new();
    let mut title_ids = BTreeMap::new();
    let mut titles = options.scope.pages.clone();
    for hero in &options.scope.heroes {
        titles.insert(hero.title.clone());
        titles.extend(hero.pages.iter().cloned());
    }
    if titles.len() > MAX_PILOT_PAGES {
        return Err("explicit title budget exceeded".into());
    }
    for id in &options.scope.page_ids {
        let mut p = params(&[("prop", "info"), ("inprop", "url")]);
        p.insert("pageids".into(), id.to_string());
        let response = request(&p)?;
        if response.get("continue").is_some() {
            return Err("unexpected identity continuation".into());
        }
        insert_page(&mut selected, &one_page(&response, Some(*id))?, options)?;
    }
    for name in titles {
        let mut p = params(&[("prop", "info"), ("inprop", "url")]);
        p.insert("titles".into(), name.clone());
        let response = request(&p)?;
        if response.get("continue").is_some() || response.pointer("/query/redirects").is_some() {
            return Err("unexpected identity continuation/redirect expansion".into());
        }
        let page = one_page(&response, None)?;
        // MediaWiki may normalize spaces/first letters; it may not substitute an
        // unrelated title. Preserve the server's explicit normalization mapping.
        let canonical = page["title"].as_str().ok_or("page title missing")?;
        let normalized = response
            .pointer("/query/normalized")
            .and_then(Value::as_array)
            .is_some_and(|items| {
                items.iter().any(|n| {
                    n["from"].as_str() == Some(&name) && n["to"].as_str() == Some(canonical)
                })
            });
        if title_key(canonical) != title_key(&name) && !normalized {
            return Err("title identity mismatch".into());
        }
        insert_page(&mut selected, &page, options)?;
        title_ids.insert(name, page["pageid"].as_i64().unwrap());
    }
    for category in &options.scope.categories {
        for namespace in &options.scope.namespace_allowlist {
            let mut cursor = Params::new();
            let mut seen_tokens = BTreeSet::new();
            let mut seen_pages = BTreeSet::new();
            loop {
                let mut p = params(&[
                    ("list", "categorymembers"),
                    ("cmtype", "page"),
                    ("cmprop", "ids|title"),
                ]);
                p.insert("cmtitle".into(), category.clone());
                p.insert("cmnamespace".into(), namespace.to_string());
                p.insert(
                    "cmlimit".into(),
                    (options.max_pages.saturating_sub(selected.len()) + 1)
                        .min(50)
                        .to_string(),
                );
                p.extend(cursor);
                let response = request(&p)?;
                for page in response
                    .pointer("/query/categorymembers")
                    .and_then(Value::as_array)
                    .ok_or("categorymembers array missing")?
                {
                    if page["ns"].as_i64() != Some(*namespace)
                        || !seen_pages
                            .insert(page["pageid"].as_i64().ok_or("category page ID missing")?)
                    {
                        return Err("unstable category discovery identity".into());
                    }
                    insert_page(&mut selected, page, options)?;
                }
                let Some(next) = continuation(&response, &["continue", "cmcontinue"])? else {
                    break;
                };
                if !seen_tokens.insert(next.clone()) {
                    return Err("repeated discovery continuation".into());
                }
                cursor = next; // even an EMPTY batch can carry a valid continuation
            }
        }
    }
    if selected.is_empty() {
        return Err("explicit scope contains no available pages".into());
    }
    let mut pages = Vec::new();
    let mut classification = Vec::new();
    for (id, identity) in &selected {
        let mut base = params(&[
            ("prop", "info|revisions"),
            ("inprop", "url"),
            ("rvprop", "ids|timestamp|content|contentmodel"),
            ("rvslots", "main"),
        ]);
        base.insert("pageids".into(), id.to_string());
        let mut cursor = Params::new();
        let mut seen = BTreeSet::new();
        let mut revisions = Vec::<Value>::new();
        let mut captured_page: Option<Value> = None;
        loop {
            let mut p = base.clone();
            p.insert("rvlimit".into(), (2 - revisions.len()).to_string());
            p.extend(cursor);
            let response = request(&p)?;
            let page = one_page(&response, Some(*id))?;
            if page["title"] != identity["title"] || page["ns"] != identity["ns"] {
                return Err("page moved during capture".into());
            }
            let head = page["lastrevid"]
                .as_i64()
                .filter(|n| *n > 0)
                .ok_or("advertised head missing")?;
            if captured_page
                .as_ref()
                .is_some_and(|old| old["lastrevid"] != page["lastrevid"])
            {
                return Err("revision capture changed head".into());
            }
            let batch = page["revisions"]
                .as_array()
                .ok_or("revision array missing")?;
            if batch.is_empty() || batch.len() > 2 - revisions.len() {
                return Err("exact revision window unavailable".into());
            }
            for revision in batch {
                let rid = revision["revid"]
                    .as_i64()
                    .filter(|n| *n > 0)
                    .ok_or("invalid revision ID")?;
                if (revisions.is_empty() && rid != head)
                    || revisions
                        .last()
                        .is_some_and(|last| last["revid"].as_i64().unwrap() <= rid)
                {
                    return Err("unordered, repeated or non-head revision".into());
                }
                revisions.push(revision.clone());
            }
            captured_page.get_or_insert(page);
            let next = continuation(&response, &["continue", "rvcontinue"])?;
            if revisions.len() == 2 || next.is_none() {
                if let Some(token) = next {
                    captured_page.as_mut().unwrap()["_capture_older_history_continue"] =
                        json!(token);
                }
                break;
            }
            let next = next.unwrap();
            if !seen.insert(next.clone()) {
                return Err("repeated revision continuation".into());
            }
            cursor = next;
        }
        let mut page = captured_page.unwrap();
        page["revisions"] = json!(revisions);
        let mut dependencies = BTreeMap::<&str, BTreeMap<String, Value>>::from([
            ("templates", BTreeMap::new()),
            ("links", BTreeMap::new()),
        ]);
        let mut cursor = Params::new();
        let mut seen = BTreeSet::new();
        loop {
            let mut p = params(&[
                ("prop", "info|templates|links"),
                ("tllimit", "50"),
                ("pllimit", "50"),
            ]);
            p.insert("pageids".into(), id.to_string());
            p.extend(cursor);
            let response = request(&p)?;
            let dependency_page = one_page(&response, Some(*id))?;
            if dependency_page["lastrevid"] != page["lastrevid"]
                || dependency_page["title"] != page["title"]
                || dependency_page["ns"] != page["ns"]
            {
                return Err("dependency capture changed identity/revision".into());
            }
            for field in ["templates", "links"] {
                if let Some(items) = dependency_page.get(field) {
                    for item in items.as_array().ok_or("invalid dependency array")? {
                        let name = item["title"]
                            .as_str()
                            .filter(|s| !s.trim().is_empty())
                            .ok_or("dependency title missing")?;
                        if let Some(old) = dependencies
                            .get_mut(field)
                            .unwrap()
                            .insert(name.into(), item.clone())
                        {
                            if old != *item {
                                return Err("conflicting dependency identity".into());
                            }
                        }
                    }
                }
            }
            let Some(next) = continuation(&response, &["continue", "tlcontinue", "plcontinue"])?
            else {
                break;
            };
            if !seen.insert(next.clone()) {
                return Err("repeated dependency continuation".into());
            }
            cursor = next;
        }
        for (field, items) in dependencies {
            page[field] = Value::Array(items.into_values().collect());
        }
        let kind = if page.get("redirect").is_some() {
            PageKind::Redirect
        } else if page["ns"] == 10 {
            PageKind::Template
        } else if page["ns"] == 828 {
            PageKind::Module
        } else {
            PageKind::Other
        };
        classification.push(Classification {
            page_id: *id,
            kind,
            build_critical: false,
            approved_exclusion: None,
        });
        pages.push(PageCapture {
            page_id: *id,
            response: json!({"batchcomplete":true,"query":{"pages":[page]}}),
            dependencies_complete: true,
            access_allowed: true,
        });
    }
    let mut hero_bindings = Vec::new();
    for hero in &options.scope.heroes {
        let id = title_ids[&hero.title];
        let entry = classification.iter_mut().find(|c| c.page_id == id).unwrap();
        if entry.kind == PageKind::Redirect {
            return Err("hero scope must select its canonical page, not a redirect".into());
        }
        entry.kind = PageKind::Hero;
        hero_bindings.push(HeroBinding {
            hero_page_id: id,
            locale: hero.locale.clone(),
            ability_page_ids: Vec::new(),
            mechanic_page_ids: Vec::new(),
            item_page_ids: Vec::new(),
            rule_page_ids: Vec::new(),
            alias_page_ids: Vec::new(),
        });
    }
    // A NORMALIZED SELECTION MANIFEST, explicitly not a server allpages response.
    // Historical offline v1 captures retain their original allpages representation.
    let discovery = options.scope.namespace_allowlist.iter().map(|namespace| DiscoveryBatch {
        namespace: *namespace,
        request_continue: Params::new(),
        response: json!({"query":{"selected_pages":selected.values().filter(|p| p["ns"].as_i64() == Some(*namespace))
            .map(|p| json!({"pageid":p["pageid"],"ns":p["ns"],"title":p["title"]})).collect::<Vec<_>>()}}),
    }).collect();
    let capture = Capture {
        format: SCOPED_CAPTURE_VERSION.into(),
        discovery_scope: Some(DiscoveryScope {
            selection: options.scope.clone(),
            selected_page_ids: selected.keys().copied().collect(),
        }),
        source_key: options.source_key.clone(),
        retrieved_at: options.retrieved_at,
        policy: options.policy.clone(),
        siteinfo,
        discovery,
        pages,
        classification,
        hero_bindings,
    };
    let encoded = serde_json::to_vec(&capture).map_err(|_| "capture encoding")?;
    crate::analyze(&encoded)?;
    Ok(capture)
}
