//! Fireworks management catalog contract:
//! https://docs.fireworks.ai/api-reference/list-models
//! Inference /models is deliberately not used: it lacks the required metadata.

use chrono::{DateTime, NaiveDate};
use serde_json::Value;

use super::{numeric_version, Candidate, Policy, Result, SelectionError};

pub(super) fn parse_page(body: &Value, policy: &Policy, now: u64) -> Result<(Vec<Candidate>, Option<String>)> {
    let entries = body.get("models").and_then(Value::as_array).ok_or(SelectionError::CatalogInvalid)?;
    if entries.len() > usize::from(policy.config.ai().selection.page_size) {
        return Err(SelectionError::CatalogInvalid);
    }
    let next = match body.get("nextPageToken") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.is_empty() => None,
        Some(Value::String(s)) if s.len() <= 4096 && !s.chars().any(char::is_control) => Some(s.clone()),
        _ => return Err(SelectionError::CatalogInvalid),
    };
    Ok((entries.iter().filter_map(|e| candidate(e, policy, now)).collect(), next))
}

fn candidate(entry: &Value, policy: &Policy, now: u64) -> Option<Candidate> {
    let model = entry.get("name")?.as_str()?;
    let version = numeric_version(model)?;
    if entry.get("state")?.as_str()? != "READY"
        || entry.get("kind")?.as_str()? != "HF_BASE_MODEL"
        || !entry.get("public")?.as_bool()?
        || !entry.get("supportsServerless")?.as_bool()?
        || !entry.get("supportsTools")?.as_bool()?
        || entry.get("contextLength")?.as_u64()? < policy.config.ai().max_completion_tokens.saturating_add(1024)
    { return None; }
    if let Some(code) = entry.get("status").and_then(|s| s.get("code")) {
        if code.as_str() != Some("OK") && code.as_u64() != Some(0) { return None; }
    }
    if entry.get("fineTuningJob").and_then(Value::as_str).is_some_and(|s| !s.is_empty())
        || entry.get("peftDetails").is_some_and(|v| !v.is_null())
        || entry.get("experimental").and_then(Value::as_bool) == Some(true)
        || entry.get("preview").and_then(Value::as_bool) == Some(true)
    { return None; }
    for field in ["displayName", "description", "stage"] {
        if let Some(s) = entry.get(field).and_then(Value::as_str) {
            let s = s.to_ascii_lowercase();
            if ["preview", "experimental", "distill", "beta"].iter().any(|word| s.contains(word)) { return None; }
        }
    }
    // A regular Flash model may support images; that does not make it a vision-special
    // variant. Dedicated suffixes are already rejected by numeric_version's exact grammar.
    let created_at = DateTime::parse_from_rfc3339(entry.get("createTime")?.as_str()?).ok()?.timestamp();
    let created_at = u64::try_from(created_at).ok().filter(|t| *t > 0 && *t <= now)?;
    if let Some(updated) = entry.get("updateTime").and_then(Value::as_str) {
        let updated = DateTime::parse_from_rfc3339(updated).ok()?.timestamp();
        if updated < i64::try_from(created_at).ok()? || u64::try_from(updated).ok()? > now { return None; }
    }
    if let Some(date) = entry.get("deprecationDate").filter(|v| !v.is_null()) {
        let year = i32::try_from(date.get("year")?.as_i64()?).ok()?;
        let month = u32::try_from(date.get("month")?.as_u64()?).ok()?;
        let day = u32::try_from(date.get("day")?.as_u64()?).ok()?;
        let deadline = NaiveDate::from_ymd_opt(year, month, day)?.and_hms_opt(0, 0, 0)?.and_utc().timestamp();
        if u64::try_from(deadline).ok()? <= now { return None; }
    }
    Some(Candidate { model: model.to_owned(), version, created_at })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn policy() -> Policy { super::super::tests::policy(None) }
    pub(crate) fn entry(id: &str) -> Value {
        json!({"name":id,"displayName":"Stable Flash","description":"General purpose model",
            "createTime":"2026-01-01T00:00:00Z","updateTime":"2026-01-02T00:00:00Z",
            "state":"READY","kind":"HF_BASE_MODEL","public":true,"supportsServerless":true,
            "supportsTools":true,"supportsImageInput":true,"contextLength":1000000,"status":{"code":"OK"}})
    }

    #[test]
    fn requires_metadata_availability_and_capabilities() {
        let p = policy();
        let base = entry("accounts/fireworks/models/deepseek-v4p1-flash");
        assert!(candidate(&base, &p, 1_800_000_000).is_some());
        for key in ["state", "kind", "public", "supportsServerless", "supportsTools", "contextLength", "createTime"] {
            let mut changed = base.clone(); changed.as_object_mut().unwrap().remove(key);
            assert!(candidate(&changed, &p, 1_800_000_000).is_none(), "{key}");
        }
        for (key, value) in [
            ("state", json!("UPLOADING")), ("supportsServerless", json!(false)),
            ("supportsTools", json!(false)), ("contextLength", json!(8)),
            ("kind", json!("EMBEDDING")), ("fineTuningJob", json!("job")),
            ("createTime", json!("2040-01-01T00:00:00Z")), ("updateTime", json!("2020-01-01T00:00:00Z")),
            ("description", json!("Experimental preview")), ("displayName", json!("Distill")),
            ("status", json!({"code":"UNAVAILABLE"})), ("public", json!(false)),
            ("deprecationDate", json!({"year":2020,"month":1,"day":1})),
        ] {
            let mut changed=base.clone(); changed[key]=value;
            assert!(candidate(&changed, &p, 1_800_000_000).is_none(), "{key}");
        }
    }

    #[test]
    fn malformed_page_not_confused_with_empty_catalog() {
        let p = policy();
        assert!(parse_page(&json!({"data":[]}), &p, 1_800_000_000).is_err());
        assert!(parse_page(&json!({"models":[],"nextPageToken":42}), &p, 1_800_000_000).is_err());
        let (items,next)=parse_page(&json!({"models":[]}), &p, 1_800_000_000).unwrap();
        assert!(items.is_empty()); assert!(next.is_none());
    }
}
