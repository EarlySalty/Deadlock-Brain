//! Versioned OpenAPI observations and conservative consumer-contract drift.
//! Deterministic fixture checks are separate from the explicit one-shot watcher.
//! Unknown semantics are never automatically accepted as compatible.
use crate::{
    external::{normalized_hash, sha256, SourceIr},
    Result, SourcesError,
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub const OPENAPI_URL: &str = "https://api.deadlock-api.com/openapi.json";
pub const PINNED_OPENAPI: &[u8] =
    include_bytes!("../tests/fixtures/external/openapi-20260925.json");
const MAX_SCHEMA_BYTES: usize = 1024 * 1024;
const MAX_CHANGES: usize = 2048;

#[derive(Debug, Clone)]
pub struct OpenApiSnapshot {
    raw: Vec<u8>,
    document: Value,
    pub raw_sha256: String,
    pub schema_sha256: String,
    pub openapi_version: String,
    pub api_version: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Compatibility {
    Unchanged,
    NonBreaking,
    Unknown,
    Breaking,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaChange {
    pub dependency: String,
    pub pointer: String,
    pub classification: Compatibility,
    pub reason: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDependency {
    pub id: String,
    pub path: String,
    pub method: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftReport {
    pub old_raw_sha256: String,
    pub new_raw_sha256: String,
    pub old_schema_sha256: String,
    pub new_schema_sha256: String,
    pub old_api_version: String,
    pub new_api_version: String,
    pub classification: Compatibility,
    pub changes: Vec<SchemaChange>,
    pub quarantined_dependencies: BTreeSet<String>,
}
impl DriftReport {
    pub fn gate(&self, dependency: &str, ir: &mut SourceIr) {
        ir.pin_schema(&self.new_schema_sha256);
        if ir.pin_schema_version(&self.new_api_version).is_err() {
            ir.quarantine("missing_api_schema_version");
        }
        if self.quarantined_dependencies.contains(dependency) {
            ir.quarantine(format!("schema_drift:{dependency}"));
        }
    }
}

impl OpenApiSnapshot {
    pub fn from_raw(raw: Vec<u8>) -> Result<Self> {
        if raw.len() > MAX_SCHEMA_BYTES {
            return Err(SourcesError::invalid_input("OpenAPI byte limit"));
        }
        let document: Value = crate::external::parse_json_strict(&raw)?;
        let version = document
            .get("openapi")
            .and_then(Value::as_str)
            .ok_or_else(|| SourcesError::invalid_input("OpenAPI version missing"))?;
        if !(version.starts_with("3.0.") || version.starts_with("3.1."))
            || version.split('.').count() != 3
            || version
                .split('.')
                .any(|v| v.is_empty() || !v.bytes().all(|c| c.is_ascii_digit()))
        {
            return Err(SourcesError::invalid_input(
                "unsupported OpenAPI dialect; quarantine required",
            ));
        }
        let api_version = document
            .pointer("/info/version")
            .and_then(Value::as_str)
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| SourcesError::invalid_input("API info.version missing"))?
            .to_owned();
        if !document.get("paths").is_some_and(Value::is_object) {
            return Err(SourcesError::invalid_input(
                "OpenAPI paths must be an object",
            ));
        }
        Ok(Self {
            raw_sha256: sha256(&raw),
            schema_sha256: normalized_hash(&document),
            openapi_version: version.into(),
            api_version,
            raw,
            document,
        })
    }
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    pub fn pinned() -> Result<Self> {
        Self::from_raw(PINNED_OPENAPI.to_vec())
    }
    pub fn response_schema(&self, dependency: &SchemaDependency) -> Result<Value> {
        let pointer = format!(
            "/paths/{}/{}/responses/200/content/application~1json/schema",
            escape(&dependency.path),
            escape(&dependency.method)
        );
        let value = self
            .document
            .pointer(&pointer)
            .ok_or_else(|| SourcesError::invalid_input("consumed JSON response schema missing"))?;
        let mut remaining = 100_000;
        expand_refs(
            &self.document,
            value,
            &mut BTreeSet::new(),
            0,
            &mut remaining,
        )
    }
    pub fn compare(&self, new: &Self, dependencies: &[SchemaDependency]) -> Result<DriftReport> {
        if dependencies.is_empty() || dependencies.len() > 128 {
            return Err(SourcesError::invalid_input(
                "schema watch needs 1..128 explicit dependencies",
            ));
        }
        let mut ids = BTreeSet::new();
        if dependencies
            .iter()
            .any(|d| d.id.is_empty() || !ids.insert(d.id.clone()))
        {
            return Err(SourcesError::invalid_input(
                "schema dependency IDs must be unique and nonempty",
            ));
        }
        let mut changes = Vec::new();
        for dependency in dependencies {
            if self.api_version != new.api_version {
                push(
                    &mut changes,
                    &dependency.id,
                    "/info/version",
                    Compatibility::Unknown,
                    "API version changed; semantics require review",
                );
            }
            if self.openapi_version != new.openapi_version {
                push(
                    &mut changes,
                    &dependency.id,
                    "/openapi",
                    Compatibility::Unknown,
                    "OpenAPI dialect version changed",
                );
            }
            match (
                self.response_schema(dependency),
                new.response_schema(dependency),
            ) {
                (Ok(old), Ok(new)) => {
                    compare_schema(&old, &new, &dependency.id, "", &mut changes, 0)
                }
                (Ok(_), Err(_)) => push(
                    &mut changes,
                    &dependency.id,
                    "",
                    Compatibility::Breaking,
                    "consumed response removed, unresolved or unsupported",
                ),
                (Err(_), _) => push(
                    &mut changes,
                    &dependency.id,
                    "",
                    Compatibility::Unknown,
                    "baseline does not describe consumed JSON response",
                ),
            }
            // Request/security semantics are not inferred from response compatibility.
            let base = format!("/paths/{}", escape(&dependency.path));
            for suffix in [
                "parameters".to_owned(),
                format!("{}/parameters", dependency.method),
                format!("{}/requestBody", dependency.method),
                format!("{}/security", dependency.method),
            ] {
                let p = format!("{base}/{suffix}");
                if self.document.pointer(&p) != new.document.pointer(&p) {
                    push(
                        &mut changes,
                        &dependency.id,
                        &p,
                        Compatibility::Unknown,
                        "request/security contract changed",
                    );
                }
            }
            if self.document.get("security") != new.document.get("security")
                || self.document.get("servers") != new.document.get("servers")
            {
                push(
                    &mut changes,
                    &dependency.id,
                    "/",
                    Compatibility::Unknown,
                    "server/security contract changed",
                );
            }
        }
        if changes.len() >= MAX_CHANGES {
            return Err(SourcesError::invalid_input("schema change budget exceeded"));
        }
        let quarantined_dependencies = changes
            .iter()
            .filter(|c| {
                matches!(
                    c.classification,
                    Compatibility::Unknown | Compatibility::Breaking
                )
            })
            .map(|c| c.dependency.clone())
            .collect();
        let classification = changes
            .iter()
            .map(|c| c.classification.clone())
            .max()
            .unwrap_or_else(|| {
                if self.schema_sha256 == new.schema_sha256 {
                    Compatibility::Unchanged
                } else {
                    Compatibility::NonBreaking
                }
            });
        Ok(DriftReport {
            old_raw_sha256: self.raw_sha256.clone(),
            new_raw_sha256: new.raw_sha256.clone(),
            old_schema_sha256: self.schema_sha256.clone(),
            new_schema_sha256: new.schema_sha256.clone(),
            old_api_version: self.api_version.clone(),
            new_api_version: new.api_version.clone(),
            classification,
            changes,
            quarantined_dependencies,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum WatchObservation {
    Compared {
        report: DriftReport,
        raw_schema: Vec<u8>,
    },
    /// Availability/HTTP failures are NOT a breaking schema observation.
    Unavailable { reason: String },
    InvalidSchema {
        raw_sha256: String,
        raw_schema: Vec<u8>,
        reason: String,
    },
}
/// Explicit one-shot library call, no cron, production job or automatic baseline update.
pub fn watch_once(
    http: &HttpClient,
    baseline: &OpenApiSnapshot,
    dependencies: &[SchemaDependency],
) -> WatchObservation {
    let response = match http.get_bounded(
        OPENAPI_URL,
        SourceHttpOptions {
            max_bytes: MAX_SCHEMA_BYTES,
            ..Default::default()
        },
    ) {
        Ok(response) => response,
        Err(_) => {
            return WatchObservation::Unavailable {
                reason: "bounded HTTP transport failed".into(),
            }
        }
    };
    if response.status != 200 {
        return WatchObservation::Unavailable {
            reason: format!(
                "http_status:{}; retry_after:{:?}",
                response.status,
                response.headers.get("retry-after")
            ),
        };
    }
    let hash = sha256(&response.content);
    match OpenApiSnapshot::from_raw(response.content.clone())
        .and_then(|new| baseline.compare(&new, dependencies))
    {
        Ok(report) => WatchObservation::Compared {
            report,
            raw_schema: response.content,
        },
        Err(_) => WatchObservation::InvalidSchema {
            raw_sha256: hash,
            raw_schema: response.content,
            reason: "invalid/unsupported schema or comparison budget".into(),
        },
    }
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
fn expand_refs(
    root: &Value,
    value: &Value,
    stack: &mut BTreeSet<String>,
    depth: usize,
    remaining: &mut usize,
) -> Result<Value> {
    if depth > 64 || *remaining == 0 {
        return Err(SourcesError::invalid_input(
            "schema resolution budget/cycle",
        ));
    }
    *remaining -= 1;
    if let Some(reference) = value.get("$ref") {
        let reference = reference
            .as_str()
            .ok_or_else(|| SourcesError::invalid_input("invalid schema reference"))?;
        let pointer = reference
            .strip_prefix('#')
            .filter(|p| p.starts_with('/'))
            .ok_or_else(|| SourcesError::invalid_input("external schema references disabled"))?;
        if !stack.insert(reference.to_owned()) {
            return Err(SourcesError::invalid_input("cyclic schema reference"));
        }
        let target = root
            .pointer(pointer)
            .ok_or_else(|| SourcesError::invalid_input("unresolved schema reference"))?;
        let resolved = expand_refs(root, target, stack, depth + 1, remaining)?;
        stack.remove(reference);
        let mut siblings = value.as_object().cloned().unwrap_or_default();
        siblings.remove("$ref");
        siblings.retain(|key, _| !matches!(key.as_str(), "description" | "summary"));
        if siblings.is_empty() {
            return Ok(resolved);
        }
        return Ok(
            json!({"allOf":[resolved, expand_refs(root, &Value::Object(siblings), stack, depth + 1, remaining)?]}),
        );
    }
    match value {
        Value::Object(values) => {
            let mut out = serde_json::Map::new();
            for (key, value) in values {
                out.insert(
                    key.clone(),
                    expand_refs(root, value, stack, depth + 1, remaining)?,
                );
            }
            Ok(Value::Object(out))
        }
        Value::Array(values) => Ok(Value::Array(
            values
                .iter()
                .map(|v| expand_refs(root, v, stack, depth + 1, remaining))
                .collect::<Result<Vec<_>>>()?,
        )),
        other => Ok(other.clone()),
    }
}
fn push(
    changes: &mut Vec<SchemaChange>,
    dependency: &str,
    pointer: &str,
    classification: Compatibility,
    reason: &str,
) {
    if changes.len() < MAX_CHANGES {
        changes.push(SchemaChange {
            dependency: dependency.into(),
            pointer: pointer.into(),
            classification,
            reason: reason.into(),
        });
    }
}
fn compare_schema(
    old: &Value,
    new: &Value,
    dep: &str,
    pointer: &str,
    changes: &mut Vec<SchemaChange>,
    depth: usize,
) {
    if old == new {
        return;
    }
    if depth > 64 || changes.len() >= MAX_CHANGES {
        push(
            changes,
            dep,
            pointer,
            Compatibility::Unknown,
            "comparison budget",
        );
        return;
    }
    let (Some(old), Some(new)) = (old.as_object(), new.as_object()) else {
        push(
            changes,
            dep,
            pointer,
            Compatibility::Unknown,
            "schema shape changed",
        );
        return;
    };
    let keys: BTreeSet<_> = old.keys().chain(new.keys()).collect();
    for key in keys {
        let a = old.get(key);
        let b = new.get(key);
        if a == b {
            continue;
        }
        let p = format!("{pointer}/{}", escape(key));
        match key.as_str() {
            "description" | "title" | "examples" | "example" | "deprecated" => push(
                changes,
                dep,
                &p,
                Compatibility::NonBreaking,
                "annotation changed; no gameplay inference",
            ),
            "properties" => {
                let a = a.and_then(Value::as_object);
                let b = b.and_then(Value::as_object);
                let fields: BTreeSet<_> = a
                    .into_iter()
                    .flat_map(|m| m.keys())
                    .chain(b.into_iter().flat_map(|m| m.keys()))
                    .collect();
                for field in fields {
                    let child = format!("{p}/{}", escape(field));
                    match (a.and_then(|v| v.get(field)), b.and_then(|v| v.get(field))) {
                        (None, Some(_)) => push(
                            changes,
                            dep,
                            &child,
                            Compatibility::NonBreaking,
                            "additive response field; retain but do not infer semantics",
                        ),
                        (Some(_), None) => push(
                            changes,
                            dep,
                            &child,
                            Compatibility::Breaking,
                            "response field removed",
                        ),
                        (Some(a), Some(b)) => compare_schema(a, b, dep, &child, changes, depth + 1),
                        _ => {}
                    }
                }
                if a.is_none() || b.is_none() {
                    push(
                        changes,
                        dep,
                        &p,
                        Compatibility::Unknown,
                        "properties container changed",
                    );
                }
            }
            "items" if a.is_some() && b.is_some() => {
                compare_schema(a.unwrap(), b.unwrap(), dep, &p, changes, depth + 1)
            }
            "required" | "enum" => {
                if key == "enum" && b.is_none() {
                    push(
                        changes,
                        dep,
                        &p,
                        Compatibility::Breaking,
                        "response enum restriction removed; new values may appear",
                    );
                    continue;
                }
                if key == "enum" && a.is_none() {
                    push(
                        changes,
                        dep,
                        &p,
                        Compatibility::NonBreaking,
                        "response enum narrowed from unconstrained",
                    );
                    continue;
                }
                let a = a.and_then(Value::as_array).cloned().unwrap_or_default();
                let b = b.and_then(Value::as_array).cloned().unwrap_or_default();
                let breaking = if key == "required" {
                    a.iter().any(|v| !b.contains(v))
                } else {
                    b.iter().any(|v| !a.contains(v))
                };
                push(
                    changes,
                    dep,
                    &p,
                    if breaking {
                        Compatibility::Breaking
                    } else {
                        Compatibility::NonBreaking
                    },
                    "response required/enum set changed",
                );
            }
            "type" | "format" | "const" => push(
                changes,
                dep,
                &p,
                Compatibility::Breaking,
                "response type/format/value contract changed",
            ),
            _ => push(
                changes,
                dep,
                &p,
                Compatibility::Unknown,
                "constraint, composition, unit or unknown semantic change",
            ),
        }
    }
}

/// Strict consumed-field validation. Additional fields remain visible but do not
/// become facts. Unknown schema keywords are quarantined, not guessed.
pub fn validate_consumed(
    schema: &Value,
    value: &Value,
) -> std::result::Result<Vec<String>, Vec<String>> {
    let mut extra = Vec::new();
    let mut errors = Vec::new();
    let mut budget = 100_000;
    validate(schema, value, "", 0, &mut budget, &mut extra, &mut errors);
    if errors.is_empty() {
        extra.sort();
        extra.dedup();
        Ok(extra)
    } else {
        errors.sort();
        errors.dedup();
        Err(errors)
    }
}
fn validate(
    schema: &Value,
    value: &Value,
    path: &str,
    depth: usize,
    budget: &mut usize,
    extra: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    if depth > 64 || *budget == 0 {
        errors.push(format!("{path}:validation_budget"));
        return;
    }
    *budget -= 1;
    let Some(s) = schema.as_object() else {
        errors.push(format!("{path}:unknown_schema"));
        return;
    };
    for key in s.keys() {
        if !matches!(
            key.as_str(),
            "type"
                | "properties"
                | "required"
                | "items"
                | "enum"
                | "additionalProperties"
                | "description"
                | "title"
                | "minimum"
                | "maximum"
                | "format"
                | "minLength"
                | "maxLength"
        ) {
            errors.push(format!("{path}:unsupported_keyword:{key}"));
        }
    }
    if let Some(kind) = s.get("type").and_then(Value::as_str) {
        let valid = match kind {
            "object" => value.is_object(),
            "array" => value.is_array(),
            "integer" => value.is_i64() || value.is_u64(),
            "number" => value.is_number(),
            "string" => value.is_string(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            _ => false,
        };
        if !valid {
            errors.push(format!("{path}:expected_{kind}"));
            return;
        }
    } else {
        errors.push(format!("{path}:missing_type"));
    }
    if let Some(values) = s.get("enum").and_then(Value::as_array) {
        if !values.contains(value) {
            errors.push(format!("{path}:unknown_enum"));
        }
    }
    if let Some(n) = value.as_f64() {
        if s.get("minimum")
            .and_then(Value::as_f64)
            .is_some_and(|v| n < v)
            || s.get("maximum")
                .and_then(Value::as_f64)
                .is_some_and(|v| n > v)
        {
            errors.push(format!("{path}:numeric_bounds"));
        }
    }
    if let Some(text) = value.as_str() {
        let len = text.chars().count() as u64;
        if s.get("minLength")
            .and_then(Value::as_u64)
            .is_some_and(|v| len < v)
            || s.get("maxLength")
                .and_then(Value::as_u64)
                .is_some_and(|v| len > v)
        {
            errors.push(format!("{path}:string_bounds"));
        }
    }
    if let Some(values) = value.as_array() {
        if let Some(items) = s.get("items") {
            for (i, v) in values.iter().enumerate() {
                validate(
                    items,
                    v,
                    &format!("{path}/{i}"),
                    depth + 1,
                    budget,
                    extra,
                    errors,
                );
                if *budget == 0 {
                    break;
                }
            }
        } else {
            errors.push(format!("{path}:missing_items_schema"));
        }
    }
    if let Some(object) = value.as_object() {
        if let Some(required) = s.get("required").and_then(Value::as_array) {
            for field in required {
                if !field.as_str().is_some_and(|k| object.contains_key(k)) {
                    errors.push(format!("{path}:missing_required:{field}"));
                }
            }
        }
        let properties = s.get("properties").and_then(Value::as_object);
        for (key, v) in object {
            let p = format!("{path}/{}", escape(key));
            if let Some(schema) = properties.and_then(|m| m.get(key)) {
                validate(schema, v, &p, depth + 1, budget, extra, errors);
            } else {
                match s.get("additionalProperties") {
                    Some(Value::Bool(false)) => errors.push(format!("{p}:unexpected_field")),
                    Some(schema) if schema.is_object() => {
                        validate(schema, v, &p, depth + 1, budget, extra, errors)
                    }
                    _ => extra.push(p),
                }
            }
        }
    }
}

pub fn asset_dependencies() -> Vec<SchemaDependency> {
    crate::assets_api::ENDPOINTS
        .iter()
        .map(|(id, path)| SchemaDependency {
            id: (*id).into(),
            path: path.split('?').next().unwrap_or(path).into(),
            method: "get".into(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn snapshot(schema: Value, version: &str) -> OpenApiSnapshot {
        OpenApiSnapshot::from_raw(json!({"openapi":"3.1.0","info":{"version":version},"paths":{"/test":{"get":{"responses":{"200":{"content":{"application/json":{"schema":schema}}}}}}}}).to_string().into_bytes()).unwrap()
    }
    fn deps() -> Vec<SchemaDependency> {
        vec![SchemaDependency {
            id: "test".into(),
            path: "/test".into(),
            method: "get".into(),
        }]
    }
    fn schema() -> Value {
        json!({"type":"object","required":["id"],"properties":{"id":{"type":"integer"},"mode":{"type":"string","enum":["a","b"]}}})
    }
    #[test]
    fn current_assets_routes_are_pinned_and_resolvable() {
        let pinned = OpenApiSnapshot::pinned().unwrap();
        assert_eq!(pinned.openapi_version, "3.1.0");
        for dep in asset_dependencies() {
            let schema = pinned.response_schema(&dep).unwrap();
            assert!(schema.is_object(), "{}", dep.id);
        }
        assert_eq!(
            pinned.raw_sha256,
            "341bc2b2681d0bc353df721974bdbf46290c1f0645eba594d18381c3a2cbbc70"
        );
    }
    #[test]
    fn detects_breaking_nonbreaking_and_version_changes() {
        let a = snapshot(schema(), "1");
        let mut s = schema();
        s["properties"]["new"] = json!({"type":"string"});
        assert_eq!(
            a.compare(&snapshot(s.clone(), "1"), &deps())
                .unwrap()
                .classification,
            Compatibility::NonBreaking
        );
        s["properties"].as_object_mut().unwrap().remove("id");
        assert_eq!(
            a.compare(&snapshot(s, "1"), &deps())
                .unwrap()
                .classification,
            Compatibility::Breaking
        );
        assert_eq!(
            a.compare(&snapshot(schema(), "2"), &deps())
                .unwrap()
                .classification,
            Compatibility::Unknown
        );
        let mut s = schema();
        s["properties"]["mode"]["enum"] = json!(["a", "b", "unknown"]);
        assert_eq!(
            a.compare(&snapshot(s, "1"), &deps())
                .unwrap()
                .classification,
            Compatibility::Breaking
        );
    }
    #[test]
    fn consumed_missing_extra_malformed_and_unknown_fields() {
        assert!(validate_consumed(&schema(), &json!({})).is_err());
        assert!(validate_consumed(&schema(), &json!({"id":"1"})).is_err());
        assert_eq!(
            validate_consumed(&schema(), &json!({"id":0,"new":42})).unwrap(),
            vec!["/new"]
        );
        assert!(validate_consumed(&schema(), &json!({"id":1,"mode":"c"})).is_err());
        assert!(OpenApiSnapshot::from_raw(b"{bad".to_vec()).is_err());
        assert!(OpenApiSnapshot::from_raw(
            br#"{"openapi":"4.0.0","info":{"version":"1"},"paths":{}}"#.to_vec()
        )
        .is_err());
    }
    #[test]
    fn refs_cannot_fetch_external_or_loop() {
        let schema = json!({"$ref":"https://example.invalid/private"});
        assert!(snapshot(schema, "1").response_schema(&deps()[0]).is_err());
        let raw = json!({"openapi":"3.1.0","info":{"version":"1"},"paths":{},"components":{"schemas":{"a":{"$ref":"#/components/schemas/a"}}}});
        assert!(expand_refs(
            &raw,
            &json!({"$ref":"#/components/schemas/a"}),
            &mut BTreeSet::new(),
            0,
            &mut 100
        )
        .is_err());
    }
    #[test]
    fn unit_changes_are_unknown_and_targeted() {
        let a = snapshot(schema(), "1");
        let mut s = schema();
        s["properties"]["id"]["x-unit"] = json!("seconds");
        let report = a.compare(&snapshot(s, "1"), &deps()).unwrap();
        assert_eq!(report.classification, Compatibility::Unknown);
        assert_eq!(
            report.quarantined_dependencies,
            BTreeSet::from(["test".into()])
        );
    }
}
