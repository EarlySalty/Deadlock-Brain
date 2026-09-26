use crate::{model::*, Result};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct Discovery {
    pub namespaces: BTreeMap<i64, String>,
    pub pages: BTreeMap<i64, PageProbe>,
    pub missing: BTreeSet<i64>,
    pub pending: BTreeMap<i64, BTreeMap<String, String>>,
    pub diagnostics: Vec<Diagnostic>,
}

pub(crate) fn values(value: &Value) -> Vec<&Value> {
    match value {
        Value::Array(array) => array.iter().collect(),
        Value::Object(object) => object.values().collect(),
        _ => Vec::new(),
    }
}

pub(crate) fn api_problem(value: &Value) -> bool {
    value.get("error").is_some() || value.get("errors").is_some() || value.get("warnings").is_some()
}

pub(crate) fn discover(capture: &Capture) -> Result<Discovery> {
    if api_problem(&capture.siteinfo) {
        return Err(
            "siteinfo contains an API error/warning; namespace inventory unavailable".into(),
        );
    }
    let mut namespaces = BTreeMap::new();
    let raw = capture
        .siteinfo
        .pointer("/query/namespaces")
        .ok_or("missing siteinfo namespaces")?;
    for namespace in values(raw) {
        let id = namespace
            .get("id")
            .and_then(Value::as_i64)
            .ok_or("namespace without numeric id")?;
        let name = namespace
            .get("canonical")
            .or_else(|| namespace.get("name"))
            .or_else(|| namespace.get("*"))
            .and_then(Value::as_str)
            .ok_or("namespace without name")?;
        if namespaces.insert(id, name.to_string()).is_some() {
            return Err("duplicate namespace id".into());
        }
    }
    if !namespaces.contains_key(&0) {
        return Err("siteinfo does not contain the main namespace".into());
    }
    let required: BTreeSet<_> = namespaces.keys().copied().filter(|id| *id >= 0).collect();
    let mut completed = BTreeSet::new();
    let mut pending = BTreeMap::<i64, BTreeMap<String, String>>::new();
    let mut seen_tokens = BTreeSet::new();
    let mut pages = BTreeMap::<i64, PageProbe>::new();
    let mut diagnostics = Vec::new();
    for batch in &capture.discovery {
        if !required.contains(&batch.namespace) {
            return Err("discovery batch uses an unknown or virtual namespace".into());
        }
        if completed.contains(&batch.namespace) {
            return Err("discovery continued after a completed namespace".into());
        }
        let expected = pending.get(&batch.namespace).cloned().unwrap_or_default();
        if batch.request_continue != expected {
            return Err("discovery continuation request does not match prior response".into());
        }
        if api_problem(&batch.response) {
            diagnostics.push(Diagnostic::new(
                "discovery_api_error_or_warning",
                format!("namespace:{}", batch.namespace),
            ));
            continue;
        }
        let entries = batch
            .response
            .pointer("/query/allpages")
            .and_then(Value::as_array)
            .ok_or("discovery response lacks query.allpages array")?;
        for page in entries {
            let page_id = page
                .get("pageid")
                .and_then(Value::as_i64)
                .filter(|id| *id > 0)
                .ok_or("discovery page has no positive pageid")?;
            let namespace = page
                .get("ns")
                .and_then(Value::as_i64)
                .ok_or("page lacks namespace")?;
            if namespace != batch.namespace {
                return Err("discovery page is in a different namespace".into());
            }
            let title = page
                .get("title")
                .and_then(Value::as_str)
                .filter(|v| !v.trim().is_empty())
                .ok_or("page lacks title")?;
            if pages.contains_key(&page_id) {
                return Err(
                    "duplicate pageid in discovery; reconcile the unstable snapshot".into(),
                );
            }
            pages.insert(
                page_id,
                PageProbe {
                    source_language: None,
                    upstream_url: None,
                    page_id,
                    source_id: format!("{}:page:{}", capture.source_key, page_id),
                    namespace,
                    title: title.into(),
                    kind: PageKind::Unclassified,
                    build_critical: false,
                    stage: Stage::Discovered,
                    revisions: Vec::new(),
                    latest_revision: None,
                    dependency_ids: BTreeSet::new(),
                    unresolved_dependencies: BTreeSet::new(),
                    dependencies_complete: false,
                    redirect_target: None,
                    diagnostics: Vec::new(),
                },
            );
            if pages.len() > MAX_PAGES {
                return Err("page budget exceeded".into());
            }
        }
        match batch.response.get("continue") {
            None => {
                completed.insert(batch.namespace);
                pending.remove(&batch.namespace);
            }
            Some(value) => {
                let token: BTreeMap<String, String> = serde_json::from_value(value.clone())
                    .map_err(|_| "invalid discovery continuation object")?;
                if token.get("apcontinue").is_none_or(String::is_empty) {
                    return Err("continuation without nonempty apcontinue".into());
                }
                if !seen_tokens.insert((
                    batch.namespace,
                    serde_json::to_string(&token).map_err(|_| "invalid token")?,
                )) {
                    return Err("repeated discovery continuation token".into());
                }
                pending.insert(batch.namespace, token);
            }
        }
    }
    let mut classified = BTreeSet::new();
    for item in &capture.classification {
        if !classified.insert(item.page_id) {
            return Err("duplicate page classification".into());
        }
        let page = pages
            .get_mut(&item.page_id)
            .ok_or("classification references undiscovered page")?;
        page.kind = item.kind;
        page.build_critical = item.build_critical;
        if let Some(reason) = &item.approved_exclusion {
            if reason.trim().is_empty() {
                return Err("exclusion requires a decision reference".into());
            }
            if item.build_critical {
                return Err("build-critical pages cannot be excluded by this probe".into());
            }
            page.stage = Stage::ApprovedExclusion;
            page.diagnostics.push(Diagnostic::new(
                "operator_approved_exclusion",
                "classification",
            ));
        }
    }
    Ok(Discovery {
        namespaces,
        pages,
        missing: required.difference(&completed).copied().collect(),
        pending,
        diagnostics,
    })
}
