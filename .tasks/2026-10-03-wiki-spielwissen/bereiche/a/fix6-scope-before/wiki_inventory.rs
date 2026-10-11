use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions, RetryPolicy};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{util::form_urlencode, Result, SourcesError};

mod normalize;
mod storage;

#[cfg(test)]
mod tests;

pub use normalize::{
    normalize_api_response, normalize_mediawiki_export, WikiGap, WikiNormalization,
    WikiSourceContext,
};

pub const CONTRACT_VERSION: &str = "wiki-spielwissen-v1";
pub const SOURCE_ID: &str = "deadlock-wiki";
pub const API_URL: &str = "https://deadlock.wiki/api.php";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WikiInventoryOptions {
    pub enabled: bool,
    pub access_policy_reviewed: bool,
    pub observed_at: String,
    pub refresh_inventory: bool,
    pub clear_access_block: bool,
    pub min_delay_seconds: f64,
    pub timeout_seconds: u64,
    pub max_pages: usize,
    pub max_response_bytes: usize,
    pub max_total_bytes: usize,
    pub batch_size: usize,
}

impl Default for WikiInventoryOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            access_policy_reviewed: false,
            observed_at: String::new(),
            refresh_inventory: false,
            clear_access_block: false,
            min_delay_seconds: 5.0,
            timeout_seconds: 60,
            max_pages: 200_000,
            max_response_bytes: 8 * 1024 * 1024,
            max_total_bytes: 512 * 1024 * 1024,
            batch_size: 50,
        }
    }
}

impl WikiInventoryOptions {
    fn validate(&self, network: bool) -> Result<()> {
        normalize::validate_observed_at(&self.observed_at)?;
        if network && (!self.enabled || !self.access_policy_reviewed) {
            return Err(SourcesError::invalid_input(
                "Wiki-Netzwerkzugriff und geprüfte Zugriffsregeln sind erforderlich",
            ));
        }
        if !self.min_delay_seconds.is_finite()
            || self.min_delay_seconds < 5.0
            || self.timeout_seconds == 0
            || self.timeout_seconds > 600
            || self.max_pages == 0
            || self.max_response_bytes == 0
            || self.max_response_bytes > 64 * 1024 * 1024
            || self.max_total_bytes < self.max_response_bytes
            || !(1..=50).contains(&self.batch_size)
        {
            return Err(SourcesError::invalid_input(
                "Ungültige Wiki-Inventargrenzen",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WikiInventoryPage {
    pub page_id: Option<i64>,
    pub namespace_id: Option<i64>,
    pub title: String,
    pub document_id: String,
    pub revision: Option<String>,
    pub page_kind: String,
    pub content_available: bool,
    #[serde(default)]
    pub available_revision: Option<String>,
}

fn merge_inventory_page(current: &mut WikiInventoryPage, mut incoming: WikiInventoryPage) {
    if current.available_revision.is_none() && current.content_available {
        current.available_revision = current.revision.clone();
    }
    if incoming.available_revision.is_none() && incoming.content_available {
        incoming.available_revision = incoming.revision.clone();
    }
    let available = match (&current.available_revision, &incoming.available_revision) {
        (Some(previous), Some(new)) if !normalize::newer_revision(new, Some(previous)) => {
            current.available_revision.clone()
        }
        (_, Some(_)) => incoming.available_revision.clone(),
        _ => current.available_revision.clone(),
    };
    if incoming.revision == current.revision {
        current.content_available |= incoming.content_available;
        if incoming.content_available {
            current.page_kind = incoming.page_kind;
        }
    } else if incoming
        .revision
        .as_deref()
        .is_some_and(|revision| normalize::newer_revision(revision, current.revision.as_deref()))
    {
        *current = incoming;
    }
    current.available_revision = available;
}

fn merge_page(pages: &mut BTreeMap<String, WikiInventoryPage>, page: WikiInventoryPage) {
    match pages.entry(page.document_id.clone()) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(page);
        }
        std::collections::btree_map::Entry::Occupied(mut entry) => {
            merge_inventory_page(entry.get_mut(), page);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiInventoryReport {
    pub contract_version: String,
    pub source_id: String,
    pub inventory_complete: bool,
    pub content_complete: bool,
    pub namespace_counts: BTreeMap<i64, usize>,
    pub namespaces_completed: Vec<i64>,
    pub namespaces_expected: Vec<i64>,
    pub inventory_pages: usize,
    pub documents: usize,
    pub unknown_revisions: usize,
    pub gaps: Vec<WikiGap>,
    pub access_block: Option<String>,
    pub documents_path: PathBuf,
    pub inventory_path: PathBuf,
    pub checkpoint_path: PathBuf,
    pub scope: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct NamespaceState {
    complete: bool,
    continuation: Option<Value>,
    seen_cursors: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Checkpoint {
    version: String,
    source_id: String,
    context: Option<WikiSourceContext>,
    namespaces: BTreeMap<i64, NamespaceState>,
    pages: BTreeMap<String, WikiInventoryPage>,
    gaps: Vec<WikiGap>,
    access_block: Option<String>,
    scope: String,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            version: CONTRACT_VERSION.into(),
            source_id: SOURCE_ID.into(),
            context: None,
            namespaces: BTreeMap::new(),
            pages: BTreeMap::new(),
            gaps: Vec::new(),
            access_block: None,
            scope: "offline_partial_source_documents".into(),
        }
    }
}

pub fn collect_wiki_inventory(
    work_dir: &Path,
    http: &HttpClient,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport> {
    options.validate(true)?;
    let rate_lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(http.cache_dir().join("wiki-corpus.lock"))?;
    rate_lock.try_lock().map_err(|error| {
        SourcesError::invalid_input(format!("Ein Wiki-Abruf läuft bereits: {error}"))
    })?;
    collect_wiki_inventory_with_transport(work_dir, options, |params| {
        wait_for_request(http.cache_dir(), options.min_delay_seconds)?;
        let pairs = params
            .iter()
            .map(|(key, value)| (key.as_str(), value.clone()))
            .collect::<Vec<_>>();
        let response = http.get_no_redirect(
            &format!("{API_URL}?{}", form_urlencode(&pairs)),
            HttpGetOptions {
                cache_ttl_seconds: Some(0),
                timeout: Duration::from_secs(options.timeout_seconds),
                retry: RetryPolicy {
                    attempts: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        if response.content.len() > options.max_response_bytes {
            return Err(SourcesError::invalid_input(
                "Wiki-Antwort überschreitet das Größenlimit",
            ));
        }
        if response
            .content_type
            .to_ascii_lowercase()
            .contains("text/html")
        {
            return Err(SourcesError::invalid_input(
                "Wiki-Zugriff verweigert: HTML statt API-JSON",
            ));
        }
        Ok(serde_json::from_slice(&response.content)?)
    })
}

pub fn collect_wiki_inventory_with_transport(
    work_dir: &Path,
    options: &WikiInventoryOptions,
    mut get: impl FnMut(&[(String, String)]) -> Result<Value>,
) -> Result<WikiInventoryReport> {
    options.validate(true)?;
    let store = storage::WikiSpool::open(work_dir, options.max_total_bytes)?;
    let mut state = store.read_checkpoint()?;
    store.bind_source(&mut state, Some("https://deadlock.wiki"))?;
    if state.source_id != SOURCE_ID
        || state
            .context
            .as_ref()
            .is_some_and(|context| context.source_origin != "https://deadlock.wiki")
    {
        return Err(SourcesError::invalid_input(
            "Live-Sammlung ist ausschließlich für deadlock.wiki erlaubt",
        ));
    }
    if state.access_block.is_some() && !options.clear_access_block {
        return Err(SourcesError::invalid_input(
            "Der Wiki-Zugang ist gesperrt; nach erneut geprüfter Zugriffsfreigabe ausdrücklich entsperren",
        ));
    }
    if options.clear_access_block {
        state.access_block = None;
        store.write_checkpoint(&state)?;
    }
    if options.refresh_inventory {
        state.context = None;
        state.namespaces.clear();
        state.pages.clear();
        state.gaps.clear();
    }
    state.scope = "all_real_namespaces_latest_revision_text_without_media_binaries".into();
    let result = collect_inner(&store, &mut state, options, &mut get);
    if let Err(error) = &result {
        if access_denied(error) {
            state.access_block =
                Some("Wiki-Zugriff verweigert; keine automatische Wiederholung".into());
        }
        store.write_checkpoint(&state)?;
        store.publish(&state)?;
    }
    result
}

fn collect_inner(
    store: &storage::WikiSpool,
    state: &mut Checkpoint,
    options: &WikiInventoryOptions,
    get: &mut impl FnMut(&[(String, String)]) -> Result<Value>,
) -> Result<WikiInventoryReport> {
    if state.context.is_none() {
        let site = get(&params(&[
            ("meta", "siteinfo"),
            ("siprop", "general|namespaces|statistics|rightsinfo"),
        ]))?;
        check_api(&site)?;
        let mut context = WikiSourceContext::from_siteinfo(&site)?;
        if context.source_origin != "https://deadlock.wiki" {
            return Err(SourcesError::invalid_input(
                "Live-siteinfo stammt nicht von deadlock.wiki",
            ));
        }
        context.license_observed_at = Some(options.observed_at.clone());
        for namespace in context.namespaces.keys().filter(|id| **id >= 0) {
            state
                .namespaces
                .insert(*namespace, NamespaceState::default());
        }
        if state.namespaces.is_empty() {
            return Err(SourcesError::invalid_input(
                "Wiki-Namensraumverzeichnis fehlt",
            ));
        }
        state.context = Some(context);
        store.write_checkpoint(state)?;
    }
    let context = state
        .context
        .as_ref()
        .ok_or_else(|| SourcesError::invariant("Wiki-Quellenkontext fehlt"))?
        .clone();
    let namespace_ids = state.namespaces.keys().copied().collect::<Vec<_>>();
    for namespace_id in namespace_ids {
        loop {
            let namespace = state
                .namespaces
                .get(&namespace_id)
                .ok_or_else(|| SourcesError::invariant("Wiki-Namensraumzustand fehlt"))?;
            if namespace.complete {
                break;
            }
            let mut request = params(&[
                ("generator", "allpages"),
                ("gapfilterredir", "all"),
                ("prop", "info|revisions"),
                ("rvprop", "ids|timestamp|content|contentmodel|user"),
                ("rvslots", "main"),
                ("rvlimit", "1"),
            ]);
            request.push(("gapnamespace".into(), namespace_id.to_string()));
            request.push(("gaplimit".into(), options.batch_size.to_string()));
            if let Some(cursor) = &namespace.continuation {
                for (key, value) in continuation_pairs(cursor)? {
                    request.push((key, value));
                }
            }
            let reply = get(&request)?;
            check_api(&reply)?;
            if serde_json::to_vec(&reply)?.len() > options.max_response_bytes {
                return Err(SourcesError::invalid_input(
                    "Wiki-Antwort überschreitet das Größenlimit",
                ));
            }
            let normalization = normalize_api_response(&reply, &context, &options.observed_at)?;
            check_page_limit(state, &normalization, options.max_pages)?;
            for page in &normalization.pages {
                if page.namespace_id != Some(namespace_id) {
                    return Err(SourcesError::invalid_input(
                        "Wiki-Seite gehört zum falschen Namensraum",
                    ));
                }
            }
            let next = reply.get("continue").cloned();
            if let Some(cursor) = &next {
                continuation_pairs(cursor)?;
                if state.namespaces[&namespace_id]
                    .seen_cursors
                    .contains(&cursor.to_string())
                {
                    return Err(SourcesError::invalid_input(
                        "Wiederholte Wiki-Fortsetzung; Inventar bleibt unvollständig",
                    ));
                }
            }
            for document in &normalization.documents {
                store.persist_document(document)?;
            }
            for page in normalization.pages {
                merge_page(&mut state.pages, page);
            }
            add_gaps(&mut state.gaps, normalization.gaps);
            let namespace = state
                .namespaces
                .get_mut(&namespace_id)
                .ok_or_else(|| SourcesError::invariant("Wiki-Namensraumzustand fehlt"))?;
            namespace.complete = next.is_none();
            if let Some(cursor) = &next {
                namespace.seen_cursors.insert(cursor.to_string());
            }
            namespace.continuation = next;
            store.write_checkpoint(state)?;
        }
    }
    store.publish(state)
}

pub fn write_wiki_api_capture(
    work_dir: &Path,
    payload: &Value,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport> {
    options.validate(false)?;
    if serde_json::to_vec(payload)?.len() > options.max_response_bytes {
        return Err(SourcesError::invalid_input(
            "Wiki-Aufzeichnung überschreitet das Größenlimit",
        ));
    }
    let normalization = normalize_api_response(payload, context, &options.observed_at)?;
    persist_offline(work_dir, context, options, normalization)
}

pub fn write_wiki_export_capture(
    work_dir: &Path,
    xml: &str,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport> {
    options.validate(false)?;
    if xml.len() > options.max_total_bytes {
        return Err(SourcesError::invalid_input(
            "Wiki-Export überschreitet das Größenlimit",
        ));
    }
    let normalization = normalize_mediawiki_export(xml, context, &options.observed_at)?;
    persist_offline(work_dir, context, options, normalization)
}

fn persist_offline(
    work_dir: &Path,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
    normalization: WikiNormalization,
) -> Result<WikiInventoryReport> {
    let store = storage::WikiSpool::open(work_dir, options.max_total_bytes)?;
    let mut state = store.read_checkpoint()?;
    if state.scope != "offline_partial_source_documents" && !state.pages.is_empty() {
        return Err(SourcesError::invalid_input(
            "Offline-Aufzeichnungen benötigen ein getrenntes Inventarverzeichnis",
        ));
    }
    if (state.context.is_some() || !state.pages.is_empty())
        && (state.source_id != context.source_id()?
            || state
                .context
                .as_ref()
                .is_some_and(|previous| previous.source_origin != context.source_origin))
    {
        return Err(SourcesError::invalid_input(
            "Verschiedene Wiki-Quellen benötigen getrennte Inventarverzeichnisse",
        ));
    }
    check_page_limit(&state, &normalization, options.max_pages)?;
    store.bind_source(&mut state, Some(&context.source_origin))?;
    state.context = Some(context.clone());
    for document in &normalization.documents {
        store.persist_document(document)?;
    }
    for page in normalization.pages {
        merge_page(&mut state.pages, page);
    }
    add_gaps(&mut state.gaps, normalization.gaps);
    store.write_checkpoint(&state)?;
    store.publish(&state)
}

pub fn read_wiki_inventory_report(
    work_dir: &Path,
    max_total_bytes: usize,
) -> Result<WikiInventoryReport> {
    let store = storage::WikiSpool::open(work_dir, max_total_bytes)?;
    let mut state = store.read_checkpoint()?;
    store.bind_source(&mut state, None)?;
    store.publish(&state)
}

fn params(values: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut params = vec![
        ("action".into(), "query".into()),
        ("format".into(), "json".into()),
        ("formatversion".into(), "2".into()),
        ("maxlag".into(), "5".into()),
    ];
    params.extend(
        values
            .iter()
            .map(|(key, value)| ((*key).into(), (*value).into())),
    );
    params
}

pub(crate) fn check_api(value: &Value) -> Result<()> {
    if !value.is_object() {
        return Err(SourcesError::invalid_input("Ungültige Wiki-API-Antwort"));
    }
    if let Some(error) = value.get("error") {
        let code = error
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        return Err(SourcesError::invalid_input(format!(
            "Wiki-API-Fehler: {code}"
        )));
    }
    if value.get("errors").is_some() || value.get("warnings").is_some() {
        return Err(SourcesError::invalid_input(
            "Wiki-API meldet Fehler oder unvollständige Daten",
        ));
    }
    Ok(())
}

fn continuation_pairs(value: &Value) -> Result<Vec<(String, String)>> {
    let object = value
        .as_object()
        .filter(|object| !object.is_empty())
        .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Fortsetzung"))?;
    let mut pairs = Vec::with_capacity(object.len());
    for (key, value) in object {
        if !matches!(key.as_str(), "continue" | "gapcontinue" | "rvcontinue") {
            return Err(SourcesError::invalid_input(
                "Unbekannter Wiki-Fortsetzungsschlüssel",
            ));
        }
        let value = value
            .as_str()
            .filter(|value| {
                !value.is_empty() && value.len() <= 8192 && !value.chars().any(char::is_control)
            })
            .ok_or_else(|| SourcesError::invalid_input("Ungültiger Wiki-Fortsetzungswert"))?;
        pairs.push((key.clone(), value.to_string()));
    }
    if !object.contains_key("gapcontinue") && !object.contains_key("rvcontinue") {
        return Err(SourcesError::invalid_input(
            "Wiki-Fortsetzung enthält keinen Fortschrittszeiger",
        ));
    }
    Ok(pairs)
}

fn access_denied(error: &SourcesError) -> bool {
    match error {
        SourcesError::Core(deadlock_brain_core::CoreError::HttpStatus { status, .. }) => {
            matches!(status.as_u16(), 401 | 403)
        }
        SourcesError::InvalidInput(message)
            if message == "Wiki-Zugriff verweigert: HTML statt API-JSON" =>
        {
            true
        }
        SourcesError::InvalidInput(message) => [
            "permissiondenied",
            "readapidenied",
            "badaccess",
            "loginrequired",
        ]
        .iter()
        .any(|code| message == &format!("Wiki-API-Fehler: {code}")),
        _ => false,
    }
}

fn check_page_limit(
    state: &Checkpoint,
    normalization: &WikiNormalization,
    maximum: usize,
) -> Result<()> {
    let incoming = normalization
        .pages
        .iter()
        .filter(|page| !state.pages.contains_key(&page.document_id))
        .map(|page| page.document_id.as_str())
        .collect::<BTreeSet<_>>();
    if state.pages.len().saturating_add(incoming.len()) > maximum {
        return Err(SourcesError::invalid_input(
            "Wiki-Seitenlimit erreicht; Inventar bleibt unvollständig",
        ));
    }
    Ok(())
}

fn add_gaps(target: &mut Vec<WikiGap>, gaps: Vec<WikiGap>) {
    for gap in gaps {
        if !target.contains(&gap) {
            target.push(gap);
        }
    }
}

fn wait_for_request(cache_dir: &Path, delay: f64) -> Result<()> {
    let path = cache_dir.join("wiki_last_request.txt");
    let previous = match fs::read_to_string(&path) {
        Ok(value) => value
            .trim()
            .parse::<f64>()
            .map_err(|_| SourcesError::invalid_input("Wiki-Abrufzeitpunkt ist beschädigt"))?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0.0,
        Err(error) => return Err(error.into()),
    };
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
    if !previous.is_finite() || previous < 0.0 || previous > now + 60.0 {
        return Err(SourcesError::invalid_input(
            "Ungültiger Wiki-Abrufzeitpunkt",
        ));
    }
    let wait = delay - (now - previous);
    if wait > 0.0 {
        thread::sleep(Duration::from_secs_f64(wait));
    }
    storage::write_atomic(
        &path,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs_f64()
            .to_string()
            .as_bytes(),
    )
}
