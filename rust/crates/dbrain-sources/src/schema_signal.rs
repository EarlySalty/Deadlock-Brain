//! Pinned JSON schema / protobuf change signals. These are not gameplay facts.
//! The protobuf reader deliberately supports only a documented scalar/message/
//! enum subset. Unknown constructs stay quarantined; no protoc/source execution.
use crate::{
    external::{normalized_hash, SourceIr, SourceRevision},
    git_source::PinnedRepository,
    schema_watch::Compatibility,
    Result, SourcesError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    JsonSchema,
    Proto,
}
#[derive(Debug, Clone)]
pub struct PinnedSignal {
    pub ir: SourceIr,
    pub kind: SignalKind,
    shape: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDependency {
    pub id: String,
    pub pointer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalReport {
    pub classification: Compatibility,
    pub old_raw_sha256: String,
    pub new_raw_sha256: String,
    pub change_origin: String,
    pub changes: BTreeMap<String, Compatibility>,
    pub quarantined_dependencies: BTreeSet<String>,
    pub gameplay_claim: bool,
}
impl SignalReport {
    pub fn gate(&self, dependency: &str, ir: &mut SourceIr) {
        if self.quarantined_dependencies.contains(dependency) {
            ir.quarantine(format!("schema_signal:{dependency}"));
        }
    }
}
impl PinnedSignal {
    pub fn read(
        repo: &PinnedRepository,
        source: &str,
        repository_id: &str,
        path: &str,
        kind: SignalKind,
    ) -> Result<Self> {
        Self::from_blob(
            source,
            repository_id,
            repo.commit(),
            path,
            kind,
            repo.read_blob(path)?,
        )
    }
    pub fn from_blob(
        source: &str,
        repository_id: &str,
        commit: &str,
        path: &str,
        kind: SignalKind,
        raw: Vec<u8>,
    ) -> Result<Self> {
        let revision = SourceRevision::Git {
            commit: commit.into(),
        };
        let locator = format!("{repository_id}/blob/{commit}/{path}");
        let now = deadlock_brain_core::now_epoch_seconds()?;
        let mut ir = match kind {
            SignalKind::JsonSchema => {
                SourceIr::from_json(source, &locator, "dbrain-schema-json/1", revision, now, raw)?
            }
            SignalKind::Proto => SourceIr::from_text(
                source,
                &locator,
                "dbrain-proto-subset/1",
                revision,
                now,
                raw,
            )?,
        };
        ir.add_origin(&format!("{repository_id}@{commit}:{path}"))?;
        let parsed = match ir.payload() {
            Ok(value) if kind == SignalKind::JsonSchema && value.is_object() => Ok(value.clone()),
            Ok(value) if kind == SignalKind::Proto => parse_proto(value.as_str().unwrap_or("")),
            _ => Err(SourcesError::invalid_input(
                "unsupported schema signal structure",
            )),
        };
        let shape = match parsed {
            Ok(shape) => {
                ir.pin_schema(&normalized_hash(&shape));
                Some(shape)
            }
            Err(_) => {
                ir.quarantine("unsupported_or_malformed_schema_signal");
                None
            }
        };
        Ok(Self { ir, kind, shape })
    }
    pub fn compare(&self, new: &Self, dependencies: &[SignalDependency]) -> Result<SignalReport> {
        if dependencies.is_empty() || dependencies.len() > 128 {
            return Err(SourcesError::invalid_input(
                "signal dependencies must be explicit and bounded",
            ));
        }
        let mut ids = BTreeSet::new();
        if dependencies.iter().any(|d| {
            d.id.is_empty()
                || !ids.insert(&d.id)
                || (!d.pointer.is_empty() && !d.pointer.starts_with('/'))
        }) {
            return Err(SourcesError::invalid_input(
                "invalid/duplicate signal dependency",
            ));
        }
        let mut changes = BTreeMap::new();
        let compatible = self.kind == new.kind
            && self.shape.is_some()
            && new.shape.is_some()
            && !self.ir.is_quarantined()
            && !new.ir.is_quarantined();
        for dep in dependencies {
            let class = if !compatible {
                Compatibility::Unknown
            } else {
                let a = self.shape.as_ref().and_then(|v| v.pointer(&dep.pointer));
                let b = new.shape.as_ref().and_then(|v| v.pointer(&dep.pointer));
                match (a, b) {
                    (Some(a), Some(b)) if a == b => Compatibility::Unchanged,
                    (None, None) => Compatibility::Unknown,
                    (Some(_), None) => Compatibility::Breaking,
                    (None, Some(_)) => Compatibility::NonBreaking,
                    (Some(a), Some(b))
                        if self.kind == SignalKind::Proto && dep.pointer.is_empty() =>
                    {
                        compare_proto(a, b)
                    }
                    (Some(a), Some(b))
                        if self.kind == SignalKind::Proto && dep.pointer.contains("/fields/") =>
                    {
                        compare_field(a, b)
                    }
                    _ => Compatibility::Unknown,
                }
            };
            changes.insert(dep.id.clone(), class);
        }
        let classification = changes
            .values()
            .max()
            .cloned()
            .unwrap_or(Compatibility::Unknown);
        let quarantined_dependencies = changes
            .iter()
            .filter(|(_, class)| matches!(class, Compatibility::Unknown | Compatibility::Breaking))
            .map(|(id, _)| id.clone())
            .collect();
        let raw_changed = self.ir.provenance().raw_sha256 != new.ir.provenance().raw_sha256;
        let parser_changed =
            self.ir.provenance().parser_revision != new.ir.provenance().parser_revision;
        let origin = match (raw_changed, parser_changed) {
            (false, false) => "same_raw_same_parser",
            (true, false) => "source_content_change",
            (false, true) => "parser_only",
            (true, true) => "source_and_parser_change",
        };
        Ok(SignalReport {
            classification,
            old_raw_sha256: self.ir.provenance().raw_sha256.clone(),
            new_raw_sha256: new.ir.provenance().raw_sha256.clone(),
            change_origin: origin.into(),
            changes,
            quarantined_dependencies,
            gameplay_claim: false,
        })
    }
}
fn compare_field(a: &Value, b: &Value) -> Compatibility {
    if a == b {
        Compatibility::Unchanged
    } else if a.get("type") != b.get("type") || a.get("cardinality") != b.get("cardinality") {
        Compatibility::Breaking
    } else {
        Compatibility::Unknown
    }
}
fn compare_proto(a: &Value, b: &Value) -> Compatibility {
    if a.get("syntax") != b.get("syntax")
        || a.get("package") != b.get("package")
        || a.get("enums") != b.get("enums")
    {
        return Compatibility::Unknown;
    }
    let (Some(a), Some(b)) = (
        a.get("messages").and_then(Value::as_object),
        b.get("messages").and_then(Value::as_object),
    ) else {
        return Compatibility::Unknown;
    };
    let mut class = Compatibility::Unchanged;
    for message in a.keys().chain(b.keys()).collect::<BTreeSet<_>>() {
        match (a.get(message), b.get(message)) {
            (Some(_), None) => class = class.max(Compatibility::Breaking),
            (None, Some(_)) => class = class.max(Compatibility::NonBreaking),
            (Some(a), Some(b)) => {
                let (Some(a), Some(b)) = (a["fields"].as_object(), b["fields"].as_object()) else {
                    return Compatibility::Unknown;
                };
                for tag in a.keys().chain(b.keys()).collect::<BTreeSet<_>>() {
                    class = class.max(match (a.get(tag), b.get(tag)) {
                        (Some(_), None) => Compatibility::Breaking,
                        (None, Some(field)) => {
                            if field["cardinality"] == "required" {
                                Compatibility::Breaking
                            } else {
                                Compatibility::NonBreaking
                            }
                        }
                        (Some(a), Some(b)) => compare_field(a, b),
                        _ => Compatibility::Unchanged,
                    });
                }
            }
            _ => {}
        }
    }
    class
}

struct Tokens {
    tokens: Vec<String>,
    i: usize,
}
impl Tokens {
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.i).map(String::as_str)
    }
    fn take(&mut self) -> Result<String> {
        let value = self
            .tokens
            .get(self.i)
            .cloned()
            .ok_or_else(|| SourcesError::invalid_input("truncated proto"))?;
        self.i += 1;
        Ok(value)
    }
    fn expect(&mut self, value: &str) -> Result<()> {
        if self.take()? != value {
            return Err(SourcesError::invalid_input("unexpected proto token"));
        }
        Ok(())
    }
}
fn identifier(value: &str) -> bool {
    let mut chars = value.chars();
    matches!(chars.next(),Some(c) if c.is_ascii_alphabetic() || c=='_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
fn tokenize(text: &str) -> Result<Tokens> {
    if text.len() > 1024 * 1024 {
        return Err(SourcesError::invalid_input("proto byte budget"));
    }
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if bytes[i..].starts_with(b"/*") {
            i += 2;
            let mut closed = false;
            while i + 1 < bytes.len() {
                if bytes[i..].starts_with(b"*/") {
                    i += 2;
                    closed = true;
                    break;
                }
                i += 1;
            }
            if !closed {
                return Err(SourcesError::invalid_input("unterminated proto comment"));
            }
            continue;
        }
        let start = i;
        if bytes[i] == b'"' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' {
                    return Err(SourcesError::invalid_input(
                        "escaped proto strings outside supported subset",
                    ));
                }
                i += 1;
            }
            if i == bytes.len() {
                return Err(SourcesError::invalid_input("unterminated proto string"));
            }
            i += 1;
        } else if bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'_' | b'.' | b'-') {
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'_' | b'.' | b'-'))
            {
                i += 1;
            }
        } else if b"{}=;".contains(&bytes[i]) {
            i += 1;
        } else {
            return Err(SourcesError::invalid_input(
                "unsupported proto token/option/map",
            ));
        }
        tokens.push(text[start..i].to_owned());
        if tokens.len() > 65536 {
            return Err(SourcesError::invalid_input("proto token budget"));
        }
    }
    Ok(Tokens { tokens, i: 0 })
}
fn parse_proto(text: &str) -> Result<Value> {
    let mut t = tokenize(text)?;
    t.expect("syntax")?;
    t.expect("=")?;
    let syntax = t.take()?;
    if !matches!(syntax.as_str(), "\"proto2\"" | "\"proto3\"") {
        return Err(SourcesError::invalid_input(
            "unsupported proto syntax/edition",
        ));
    }
    t.expect(";")?;
    let mut package = String::new();
    let mut messages = BTreeMap::new();
    let mut enums = BTreeMap::new();
    while let Some(next) = t.peek() {
        match next {
            "package" if package.is_empty() => {
                t.take()?;
                package = t.take()?;
                if !package.split('.').all(identifier) {
                    return Err(SourcesError::invalid_input("invalid proto package"));
                }
                t.expect(";")?;
            }
            "message" => parse_message(&mut t, "", &syntax, &mut messages, &mut enums, 0)?,
            "enum" => parse_enum(&mut t, "", &syntax, &mut enums)?,
            _ => {
                return Err(SourcesError::invalid_input(
                    "unsupported proto import/options/service/extension",
                ))
            }
        }
    }
    if messages.is_empty() && enums.is_empty() {
        return Err(SourcesError::invalid_input("empty proto signal"));
    }
    let scalars = [
        "double", "float", "int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32",
        "fixed64", "sfixed32", "sfixed64", "bool", "string", "bytes",
    ];
    let known: BTreeSet<String> = messages.keys().chain(enums.keys()).cloned().collect();
    if messages.keys().any(|key| enums.contains_key(key)) {
        return Err(SourcesError::invalid_input("message/enum name collision"));
    }
    for (message, value) in &messages {
        for field in value["fields"]
            .as_object()
            .ok_or_else(|| SourcesError::invalid_input("invalid proto fields"))?
            .values()
        {
            let kind = field["type"].as_str().unwrap_or("");
            if scalars.contains(&kind) {
                continue;
            }
            let stripped = kind.trim_start_matches('.');
            let without_package = stripped
                .strip_prefix(&format!("{package}."))
                .unwrap_or(stripped);
            let mut scope = message.as_str();
            let mut resolved = known.contains(without_package);
            loop {
                resolved |= known.contains(&format!("{scope}.{kind}"));
                match scope.rsplit_once('.') {
                    Some((parent, _)) => scope = parent,
                    None => break,
                }
            }
            if !resolved {
                return Err(SourcesError::invalid_input(
                    "unresolved proto field type; imports are not guessed",
                ));
            }
        }
    }
    Ok(json!({"syntax":syntax,"package":package,"messages":messages,"enums":enums}))
}
fn parse_message(
    t: &mut Tokens,
    prefix: &str,
    syntax: &str,
    messages: &mut BTreeMap<String, Value>,
    enums: &mut BTreeMap<String, Value>,
    depth: usize,
) -> Result<()> {
    if depth > 16 {
        return Err(SourcesError::invalid_input("proto nesting budget"));
    }
    t.expect("message")?;
    let name = t.take()?;
    if !identifier(&name) {
        return Err(SourcesError::invalid_input("invalid message name"));
    }
    let full = format!("{prefix}{name}");
    t.expect("{")?;
    let mut fields = BTreeMap::new();
    let mut names = BTreeSet::new();
    while t.peek() != Some("}") {
        if t.peek() == Some("message") {
            parse_message(t, &format!("{full}."), syntax, messages, enums, depth + 1)?;
            continue;
        }
        if t.peek() == Some("enum") {
            parse_enum(t, &format!("{full}."), syntax, enums)?;
            continue;
        }
        let first = t.take()?;
        let (cardinality, kind) = if matches!(first.as_str(), "optional" | "required" | "repeated")
        {
            (first, t.take()?)
        } else {
            ("singular".into(), first)
        };
        if syntax == "\"proto3\"" && cardinality == "required" {
            return Err(SourcesError::invalid_input("required field in proto3"));
        }
        if syntax == "\"proto2\"" && cardinality == "singular" {
            return Err(SourcesError::invalid_input("unlabelled proto2 field"));
        }
        if !kind.trim_start_matches('.').split('.').all(identifier)
            || matches!(
                kind.as_str(),
                "oneof" | "reserved" | "option" | "extensions" | "group" | "map"
            )
        {
            return Err(SourcesError::invalid_input("unsupported proto field"));
        }
        let name = t.take()?;
        if !identifier(&name) || !names.insert(name.clone()) {
            return Err(SourcesError::invalid_input("duplicate/invalid field name"));
        }
        t.expect("=")?;
        let tag = t
            .take()?
            .parse::<u32>()
            .map_err(|_| SourcesError::invalid_input("invalid proto tag"))?;
        if tag == 0 || tag > 536870911 || (19000..=19999).contains(&tag) {
            return Err(SourcesError::invalid_input("reserved/invalid proto tag"));
        }
        t.expect(";")?;
        if fields
            .insert(
                tag.to_string(),
                json!({"name":name,"type":kind,"cardinality":cardinality}),
            )
            .is_some()
        {
            return Err(SourcesError::invalid_input("duplicate proto tag"));
        }
    }
    t.expect("}")?;
    if messages.insert(full, json!({"fields":fields})).is_some() {
        return Err(SourcesError::invalid_input("duplicate proto message"));
    }
    Ok(())
}
fn parse_enum(
    t: &mut Tokens,
    prefix: &str,
    syntax: &str,
    enums: &mut BTreeMap<String, Value>,
) -> Result<()> {
    t.expect("enum")?;
    let name = t.take()?;
    if !identifier(&name) {
        return Err(SourcesError::invalid_input("invalid enum name"));
    }
    t.expect("{")?;
    let mut values = BTreeMap::new();
    let mut names = BTreeSet::new();
    while t.peek() != Some("}") {
        let name = t.take()?;
        if !identifier(&name) || !names.insert(name.clone()) {
            return Err(SourcesError::invalid_input("invalid/duplicate enum name"));
        }
        t.expect("=")?;
        let number = t
            .take()?
            .parse::<i32>()
            .map_err(|_| SourcesError::invalid_input("invalid enum number"))?;
        t.expect(";")?;
        if syntax == "\"proto3\"" && values.is_empty() && number != 0 {
            return Err(SourcesError::invalid_input(
                "first proto3 enum value must be zero",
            ));
        }
        if values.insert(number.to_string(), name).is_some() {
            return Err(SourcesError::invalid_input(
                "enum aliases require explicit unsupported policy",
            ));
        }
    }
    t.expect("}")?;
    if values.is_empty()
        || enums
            .insert(format!("{prefix}{name}"), json!(values))
            .is_some()
    {
        return Err(SourcesError::invalid_input("empty/duplicate enum"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn signal(text: &str) -> PinnedSignal {
        PinnedSignal::from_blob(
            "proto-fixture",
            "fixture/repo",
            &"a".repeat(40),
            "schema.proto",
            SignalKind::Proto,
            text.as_bytes().to_vec(),
        )
        .unwrap()
    }
    fn deps() -> Vec<SignalDependency> {
        vec![SignalDependency {
            id: "decoder".into(),
            pointer: "".into(),
        }]
    }
    #[test]
    fn proto_add_remove_retag_type_and_raw_bytes() {
        let a = signal("syntax = \"proto3\"; message State { uint32 id = 1; }");
        let b = signal(
            "syntax = \"proto3\"; message State { uint32 id = 1; optional string name = 2; }",
        );
        assert_eq!(
            a.compare(&b, &deps()).unwrap().classification,
            Compatibility::NonBreaking
        );
        assert_eq!(
            b.compare(&a, &deps()).unwrap().classification,
            Compatibility::Breaking
        );
        let c = signal("syntax = \"proto3\"; message State { string id = 1; }");
        assert_eq!(
            a.compare(&c, &deps()).unwrap().classification,
            Compatibility::Breaking
        );
        let d = signal("syntax = \"proto3\"; message State { uint32 id = 2; }");
        assert_eq!(
            a.compare(&d, &deps()).unwrap().classification,
            Compatibility::Breaking
        );
        assert!(!a.compare(&b, &deps()).unwrap().gameplay_claim);
        assert!(a.ir.raw().starts_with(b"syntax"));
    }
    #[test]
    fn unsupported_proto_is_quarantined_not_silently_ignored() {
        for input in [
            "syntax=\"proto3\"; import \"other.proto\"; message X {}",
            "syntax=\"proto3\"; message X { oneof choice { uint32 a=1; } }",
            "syntax=\"proto3\"; message X { uint32 a=1; uint32 b=1; }",
            "syntax=\"proto3\"; message X { uint32 a=19000; }",
        ] {
            let ir = signal(input);
            assert!(ir.ir.is_quarantined(), "{input}");
            assert!(ir.ir.payload().is_err());
        }
    }
    #[test]
    fn unresolved_types_and_invalid_proto3_enum_are_quarantined() {
        assert!(signal("syntax=\"proto3\"; message X { Typo value=1; }")
            .ir
            .is_quarantined());
        assert!(signal("syntax=\"proto3\"; enum E { WRONG=1; }")
            .ir
            .is_quarantined());
        assert!(
            !signal("syntax=\"proto3\"; message X { Y value=1; } message Y { uint32 id=1; }")
                .ir
                .is_quarantined()
        );
    }
    #[test]
    fn comment_changes_are_not_schema_changes() {
        let a = signal("syntax=\"proto3\"; message X { uint32 a=1; }");
        let b = signal(
            "// author-owned fixture\nsyntax=\"proto3\"; /* comment */ message X { uint32 a=1; }",
        );
        let report = a.compare(&b, &deps()).unwrap();
        assert_eq!(report.classification, Compatibility::Unchanged);
        assert_ne!(report.old_raw_sha256, report.new_raw_sha256);
    }
    #[test]
    fn json_signal_only_invalidates_consumed_dependency() {
        let create = |raw: &[u8]| {
            PinnedSignal::from_blob(
                "schema-fixture",
                "fixture/repo",
                &"a".repeat(40),
                "schema.json",
                SignalKind::JsonSchema,
                raw.into(),
            )
            .unwrap()
        };
        let a = create(br#"{"Player":{"type":"u32"},"Hero":{"type":"u32"}}"#);
        let b = create(br#"{"Hero":{"type":"u32"}}"#);
        let report = a
            .compare(
                &b,
                &[
                    SignalDependency {
                        id: "player".into(),
                        pointer: "/Player".into(),
                    },
                    SignalDependency {
                        id: "hero".into(),
                        pointer: "/Hero".into(),
                    },
                ],
            )
            .unwrap();
        assert_eq!(
            report.quarantined_dependencies,
            BTreeSet::from(["player".into()])
        );
    }
}
