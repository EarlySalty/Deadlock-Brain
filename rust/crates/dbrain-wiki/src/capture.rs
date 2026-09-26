//! Bounded MediaWiki request planner with injected transport. No HTTP client,
//! auth bypass, template execution or implicit corpus mirror lives here.
//! API references: MediaWiki API:Continue, API:Revisions and API:Templates.
use crate::{
    discovery::{api_problem, values},
    model::*,
    Result,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub type Params = BTreeMap<String, String>;
#[derive(Debug, Clone)]
pub struct CaptureOptions {
    pub source_key: String,
    pub retrieved_at: i64,
    pub policy: CapturePolicy,
    /// Explicit pilot selection. Discovery inventories pages; only these bodies
    /// are fetched. An empty selection is metadata-only, never an implicit mirror.
    pub page_ids: BTreeSet<i64>,
    pub max_requests: usize,
    pub max_pages: usize,
    pub max_total_bytes: usize,
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
            .any(|(k, v)| !allowed.contains(&k.as_str()) || v.is_empty())
        || !token.keys().any(|k| k != "continue")
    {
        return Err("unsafe/empty continuation token".into());
    }
    Ok(Some(token))
}
fn one_page(response: &Value, id: i64) -> Result<Value> {
    let found = values(
        response
            .pointer("/query/pages")
            .ok_or("missing page response")?,
    );
    if found.len() != 1
        || found[0]["pageid"].as_i64() != Some(id)
        || found[0].get("missing").is_some()
        || found[0].get("invalid").is_some()
    {
        return Err("page missing or identity changed".into());
    }
    Ok(found[0].clone())
}

/// Stops on transport errors including HTTP 403. The caller must enforce network
/// policy and HTTP response limits BEFORE parsing; we also bound aggregate JSON.
/// No partial capture is returned as a complete result after budget exhaustion.
pub fn collect(
    options: &CaptureOptions,
    mut get: impl FnMut(&Params) -> Result<Value>,
) -> Result<Capture> {
    if !options.policy.offline_review_allowed
        || options
            .policy
            .decision_ref
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        || options.retrieved_at <= 0
        || options.max_requests == 0
        || options.max_requests > MAX_PAGES
        || options.max_pages == 0
        || options.max_pages > MAX_PAGES
        || options.page_ids.len() > 64
        || options.max_total_bytes == 0
        || options.max_total_bytes > MAX_INPUT_BYTES
    {
        return Err("invalid or unapproved capture budget".into());
    }
    let mut requests = 0;
    let mut total_bytes = 0usize;
    let mut request = |p: &Params| -> Result<Value> {
        if requests >= options.max_requests {
            return Err("request budget exhausted".into());
        }
        requests += 1;
        let value = get(p)?;
        total_bytes = total_bytes
            .checked_add(
                serde_json::to_vec(&value)
                    .map_err(|_| "response encoding")?
                    .len(),
            )
            .ok_or("byte budget overflow")?;
        if total_bytes > options.max_total_bytes {
            return Err("aggregate capture byte budget exhausted".into());
        }
        if !value.is_object() || api_problem(&value) {
            return Err("API error/warning; capture incomplete".into());
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
        if id >= 0 && !namespaces.insert(id) {
            return Err("duplicate namespace".into());
        }
    }
    if !namespaces.contains(&0) {
        return Err("main namespace absent".into());
    }
    let mut discovered = BTreeMap::new();
    let mut discovery = Vec::new();
    for namespace in namespaces {
        let mut cursor = Params::new();
        let mut seen = BTreeSet::new();
        loop {
            let mut p = params(&[
                ("list", "allpages"),
                ("apfilterredir", "all"),
                ("aplimit", "50"),
            ]);
            p.insert("apnamespace".into(), namespace.to_string());
            p.extend(cursor.clone());
            let response = request(&p)?;
            for page in response
                .pointer("/query/allpages")
                .and_then(Value::as_array)
                .ok_or("allpages array missing")?
            {
                let id = page["pageid"]
                    .as_i64()
                    .filter(|id| *id > 0)
                    .ok_or("invalid page ID")?;
                if page["ns"].as_i64() != Some(namespace)
                    || discovered.insert(id, page.clone()).is_some()
                {
                    return Err("unstable discovery identity".into());
                }
                if discovered.len() > options.max_pages {
                    return Err("page budget exhausted".into());
                }
            }
            let next = continuation(&response, &["continue", "apcontinue"])?;
            discovery.push(DiscoveryBatch {
                namespace,
                request_continue: cursor,
                response,
            });
            let Some(next) = next else {
                break;
            };
            if !seen.insert(next.clone()) {
                return Err("repeated discovery continuation".into());
            }
            cursor = next;
        }
    }
    let mut pages = Vec::new();
    for id in &options.page_ids {
        let identity = discovered
            .get(id)
            .ok_or("selected page was not discovered")?;
        let mut p = params(&[
            ("prop", "info|revisions"),
            ("inprop", "url"),
            ("rvprop", "ids|timestamp|content|contentmodel"),
            ("rvslots", "main"),
            ("rvlimit", "2"),
        ]);
        p.insert("pageids".into(), id.to_string());
        let response = request(&p)?;
        let mut page = one_page(&response, *id)?;
        if page["title"] != identity["title"] || page["ns"] != identity["ns"] {
            return Err("page moved during capture".into());
        }
        let head = page["lastrevid"]
            .as_i64()
            .filter(|n| *n > 0)
            .ok_or("advertised head missing")?;
        let revisions = page["revisions"]
            .as_array()
            .ok_or("revision array missing")?;
        if revisions.is_empty()
            || revisions.len() > 2
            || revisions[0]["revid"].as_i64() != Some(head)
        {
            return Err("exact current revision unavailable".into());
        }
        // rvlimit=2 is a deliberate history window, not an assertion that older
        // history does not exist. Preserve the server token in raw metadata.
        if let Some(token) = continuation(&response, &["continue", "rvcontinue"])? {
            page["_capture_older_history_continue"] =
                serde_json::to_value(token).map_err(|_| "history token encoding")?;
        }
        let mut dependencies = BTreeMap::<&str, BTreeMap<String, Value>>::from([
            ("templates", BTreeMap::new()),
            ("links", BTreeMap::new()),
        ]);
        let mut cursor = Params::new();
        let mut seen = BTreeSet::new();
        loop {
            let mut p = params(&[
                ("prop", "info|templates|links"),
                ("tllimit", "max"),
                ("pllimit", "max"),
            ]);
            p.insert("pageids".into(), id.to_string());
            p.extend(cursor);
            let response = request(&p)?;
            let dependency_page = one_page(&response, *id)?;
            if dependency_page["lastrevid"].as_i64() != Some(head)
                || dependency_page["title"] != page["title"]
                || dependency_page["ns"] != page["ns"]
            {
                return Err("dependency capture changed identity/revision".into());
            }
            for field in ["templates", "links"] {
                // Omitted property after a requested/complete API response means
                // no members; arrays, when supplied, must have the correct type.
                if let Some(items) = dependency_page.get(field) {
                    for item in items.as_array().ok_or("invalid dependency array")? {
                        let title = item["title"]
                            .as_str()
                            .filter(|t| !t.trim().is_empty())
                            .ok_or("dependency title missing")?;
                        dependencies
                            .get_mut(field)
                            .unwrap()
                            .insert(title.into(), item.clone());
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
        pages.push(PageCapture {
            page_id: *id,
            response: json!({"batchcomplete":true,"query":{"pages":[page]}}),
            dependencies_complete: true,
            access_allowed: true,
        });
    }
    let capture = Capture {
        format: CAPTURE_VERSION.into(),
        source_key: options.source_key.clone(),
        retrieved_at: options.retrieved_at,
        policy: options.policy.clone(),
        siteinfo,
        discovery,
        pages,
        classification: Vec::new(),
        hero_bindings: Vec::new(),
    };
    // Validate complete namespace/continuation/source identities before returning.
    let encoded = serde_json::to_vec(&capture).map_err(|_| "capture encoding")?;
    crate::analyze(&encoded)?;
    Ok(capture)
}
