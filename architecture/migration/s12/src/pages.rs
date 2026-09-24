use crate::{
    discovery::{api_problem, values},
    model::*,
    parse_json, sha256, Result,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn title_key(title: &str) -> String {
    title.replace('_', " ").trim().to_string()
}

pub(crate) fn inspect(capture: &Capture, pages: &mut BTreeMap<i64, PageProbe>) -> Result<()> {
    let mut titles = BTreeMap::new();
    for page in pages.values() {
        if titles
            .insert(title_key(&page.title), page.page_id)
            .is_some()
        {
            return Err("ambiguous discovered title".into());
        }
    }
    let denied: BTreeSet<_> = capture
        .pages
        .iter()
        .filter(|p| !p.access_allowed)
        .map(|p| p.page_id)
        .collect();
    let mut captures = BTreeMap::<i64, Vec<&PageCapture>>::new();
    for input in &capture.pages {
        if !pages.contains_key(&input.page_id) {
            return Err("capture references undiscovered pageid".into());
        }
        captures.entry(input.page_id).or_default().push(input);
    }
    for (id, inputs) in captures {
        let page = pages.get_mut(&id).ok_or("missing discovered page")?;
        if denied.contains(&id) {
            // Current denial dominates any historical/duplicate allow. Never emit raw
            // bytes, hashes, candidates or dependencies from a denied capture.
            page.stage = Stage::PolicyBlocked;
            page.diagnostics
                .push(Diagnostic::new("access_revoked", "capture.access_allowed"));
            continue;
        }
        if page.stage == Stage::ApprovedExclusion {
            continue;
        }
        let mut revisions = BTreeMap::<i64, RevisionProbe>::new();
        let mut advertised_heads = BTreeSet::new();
        let mut failed = false;
        let mut conflict = false;
        let mut dependencies_complete = true;
        for input in inputs {
            if api_problem(&input.response) {
                failed = true;
                page.diagnostics
                    .push(Diagnostic::new("page_api_error_or_warning", "response"));
                continue;
            }
            if input.response.get("continue").is_some() {
                dependencies_complete = false;
                page.diagnostics.push(Diagnostic::new(
                    "page_response_continuation_pending",
                    "response/continue",
                ));
            }
            let raw_pages = input
                .response
                .pointer("/query/pages")
                .ok_or("page capture lacks query.pages")?;
            let returned = values(raw_pages);
            if returned.len() != 1 {
                return Err("each PageCapture must contain exactly one requested page".into());
            }
            let raw_page = returned[0];
            if raw_page.get("missing").is_some() || raw_page.get("invalid").is_some() {
                failed = true;
                page.diagnostics
                    .push(Diagnostic::new("page_unavailable", "response/query/pages"));
                continue;
            }
            if raw_page.get("pageid").and_then(Value::as_i64) != Some(id)
                || raw_page.get("ns").and_then(Value::as_i64) != Some(page.namespace)
            {
                return Err("fetched page identity differs from requested discovery entry".into());
            }
            if raw_page.get("title").and_then(Value::as_str).map(title_key)
                != Some(title_key(&page.title))
            {
                conflict = true;
                page.diagnostics.push(Diagnostic::new(
                    "move_during_capture_reconcile_required",
                    "response/title",
                ));
            }
            page.source_language = raw_page
                .get("pagelanguage")
                .and_then(Value::as_str)
                .map(str::to_string);
            page.upstream_url = raw_page
                .get("fullurl")
                .and_then(Value::as_str)
                .map(str::to_string);
            if let Some(head) = raw_page
                .get("lastrevid")
                .and_then(Value::as_i64)
                .filter(|n| *n > 0)
            {
                advertised_heads.insert(head);
            } else {
                conflict = true;
                page.diagnostics.push(Diagnostic::new(
                    "head_revision_not_verified",
                    "response/lastrevid",
                ));
            }
            dependencies_complete &= input.dependencies_complete;
            for field in ["templates", "links"] {
                if let Some(dependencies) = raw_page.get(field).and_then(Value::as_array) {
                    for dependency in dependencies {
                        let title = dependency
                            .get("title")
                            .and_then(Value::as_str)
                            .ok_or("dependency lacks title")?;
                        if let Some(target) = titles.get(&title_key(title)) {
                            page.dependency_ids.insert(*target);
                        } else {
                            page.unresolved_dependencies.insert(title_key(title));
                        }
                    }
                } else {
                    dependencies_complete = false;
                }
            }
            let raw_revisions = raw_page.get("revisions").and_then(Value::as_array);
            if raw_revisions.is_none_or(Vec::is_empty) {
                failed = true;
                page.diagnostics.push(Diagnostic::new(
                    "revision_content_unavailable",
                    "response/revisions",
                ));
                continue;
            }
            if raw_revisions.is_some_and(|r| r.len() > 128) {
                return Err("revision history budget exceeded".into());
            }
            for raw in raw_revisions.into_iter().flatten() {
                let revision_id = raw
                    .get("revid")
                    .and_then(Value::as_i64)
                    .filter(|id| *id > 0)
                    .ok_or("revision lacks positive revid")?;
                let probe = inspect_revision(raw, revision_id, capture.retrieved_at)?;
                if let Some(previous) = revisions.get(&revision_id) {
                    if previous != &probe {
                        conflict = true;
                        page.diagnostics.push(Diagnostic::new(
                            "same_revision_conflicting_content",
                            format!("revision:{revision_id}"),
                        ));
                    }
                } else {
                    revisions.insert(revision_id, probe);
                }
            }
        }
        page.latest_revision = advertised_heads.last().copied();
        if advertised_heads.len() > 1 {
            conflict = true;
            page.diagnostics.push(Diagnostic::new(
                "inconsistent_capture_heads",
                "response/lastrevid",
            ));
        }
        if let Some(head) = page.latest_revision {
            if revisions.keys().any(|id| *id > head) {
                conflict = true;
                page.diagnostics.push(Diagnostic::new(
                    "revision_newer_than_advertised_head",
                    "response/revisions",
                ));
            }
        }
        page.revisions = revisions.into_values().collect();
        let mut previous_time = None;
        for revision in &page.revisions {
            if let Some(time) = revision.source_time {
                if previous_time.is_some_and(|prior| time < prior) {
                    conflict = true;
                    page.diagnostics.push(Diagnostic::new(
                        "revision_time_order_conflict",
                        format!("revision:{}", revision.revision_id),
                    ));
                }
                previous_time = Some(time);
            }
        }
        page.dependencies_complete = dependencies_complete
            && page.unresolved_dependencies.is_empty()
            && !failed
            && !conflict;
        if let Some(raw) = page.latest().and_then(|r| r.candidates.first()) {
            if let Some(text) = raw.value.as_str() {
                if let Some(target) = redirect_target(text) {
                    if let Some(target_id) = titles.get(&title_key(target)) {
                        page.redirect_target = Some(*target_id);
                        page.dependency_ids.insert(*target_id);
                    } else {
                        page.unresolved_dependencies.insert(title_key(target));
                        page.dependencies_complete = false;
                    }
                }
            }
        }
        page.stage = if failed {
            Stage::Unavailable
        } else if conflict {
            Stage::Quarantined
        } else if page.latest().is_some_and(|r| r.syntax_parsed) {
            Stage::Parsed
        } else if !page.revisions.is_empty() {
            Stage::Quarantined
        } else {
            Stage::Fetched
        };
    }
    Ok(())
}

fn inspect_revision(raw: &Value, id: i64, retrieved_at: i64) -> Result<RevisionProbe> {
    let slot = raw.pointer("/slots/main").unwrap_or(raw);
    let model = slot
        .get("contentmodel")
        .or_else(|| raw.get("contentmodel"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let time = raw
        .get("timestamp")
        .and_then(Value::as_str)
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|t| t.timestamp());
    let mut result = RevisionProbe {
        revision_id: id,
        source_time: time,
        retrieved_at,
        content_model: model.into(),
        raw_sha256: None,
        candidates: Vec::new(),
        diagnostics: Vec::new(),
        syntax_parsed: false,
    };
    if time.is_none_or(|time| time <= 0 || time > retrieved_at) {
        result.diagnostics.push(Diagnostic::new(
            "source_time_invalid_or_future",
            "revision/timestamp",
        ));
    }
    if raw.get("texthidden").is_some()
        || slot.get("texthidden").is_some()
        || raw.get("suppressed").is_some()
        || slot.get("suppressed").is_some()
    {
        result.diagnostics.push(Diagnostic::new(
            "revision_suppressed",
            "revision/slots/main",
        ));
        return Ok(result);
    }
    let content = slot
        .get("content")
        .or_else(|| slot.get("*"))
        .and_then(Value::as_str);
    let Some(content) = content else {
        result.diagnostics.push(Diagnostic::new(
            "content_missing_not_empty",
            "revision/slots/main",
        ));
        return Ok(result);
    };
    if content.len() > MAX_CONTENT_BYTES {
        result.diagnostics.push(Diagnostic::new(
            "content_byte_budget_exceeded",
            "revision/slots/main",
        ));
        return Ok(result);
    }
    result.raw_sha256 = Some(sha256(content.as_bytes()));
    let valid_time = result.diagnostics.is_empty();
    match model {
        "json" => match parse_json(content.as_bytes()) {
            Ok(value) => {
                if flatten(&value, "", 0, &mut result.candidates).is_err() {
                    result.candidates.clear();
                    result.diagnostics.push(Diagnostic::new(
                        "structured_value_budget_exceeded",
                        "revision/slots/main/content",
                    ));
                } else {
                    result.syntax_parsed = valid_time;
                    result.diagnostics.push(Diagnostic::new(
                        "syntactic_values_not_validated_facts",
                        "revision/slots/main/content",
                    ));
                }
            }
            Err(_) => result.diagnostics.push(Diagnostic::new(
                "invalid_structured_json",
                "revision/slots/main/content",
            )),
        },
        "wikitext" => {
            // Deliberately no template expansion, Lua execution, numeric inference or
            // implicit HTML fallback. Unknown expressions retain exact byte locators.
            let markers = ["{{", "}}", "<", "{|", "|}"];
            let mut unsupported = false;
            for marker in markers {
                if let Some(offset) = content.find(marker) {
                    unsupported = true;
                    result.diagnostics.push(Diagnostic::new(
                        "unsupported_wikitext_semantics",
                        format!("utf8:{offset}..{}", offset + marker.len()),
                    ));
                }
            }
            if !unsupported {
                result.candidates.push(Candidate {
                    locator: format!("utf8:0..{}", content.len()),
                    value: content.into(),
                });
                result.syntax_parsed = valid_time;
                result.diagnostics.push(Diagnostic::new(
                    "prose_or_redirect_not_a_game_fact",
                    "revision/slots/main/content",
                ));
            }
        }
        _ => result.diagnostics.push(Diagnostic::new(
            "unsupported_content_model_no_execution",
            "revision/slots/main/contentmodel",
        )),
    }
    Ok(result)
}

fn flatten(value: &Value, pointer: &str, depth: usize, output: &mut Vec<Candidate>) -> Result<()> {
    if depth > 32 || output.len() >= MAX_CANDIDATES {
        return Err("structured value budget exceeded".into());
    }
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                flatten(value, &format!("{pointer}/{escaped}"), depth + 1, output)?;
            }
        }
        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                flatten(value, &format!("{pointer}/{index}"), depth + 1, output)?;
            }
        }
        _ => output.push(Candidate {
            locator: format!("json:{pointer}"),
            value: value.clone(),
        }),
    }
    Ok(())
}

fn redirect_target(text: &str) -> Option<&str> {
    let text = text.trim();
    let prefix = text.get(..9)?;
    if !prefix.eq_ignore_ascii_case("#redirect") {
        return None;
    }
    let target = text
        .get(9..)?
        .trim()
        .strip_prefix("[[")?
        .strip_suffix("]]")?;
    if target.is_empty() || target.contains(['|', '#', '[', ']']) {
        None
    } else {
        Some(target)
    }
}
