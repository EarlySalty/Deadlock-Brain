use serde_json::{json, Value};
use std::path::Path;

#[path = "game_file_facts/budget.rs"]
pub mod budget;
#[path = "game_file_facts/json_text.rs"]
mod json_text;
#[path = "game_file_facts/kv.rs"]
mod kv;
#[path = "game_file_facts/kv3.rs"]
mod kv3;

pub fn extract_facts(path: &str, content: &str) -> (Vec<Value>, String) {
    let trimmed = content.trim_start_matches('\u{feff}').trim_start();
    if path.to_ascii_lowercase().ends_with(".json") {
        return match json_text::parse(trimmed) {
            Ok(leaves) => match leaf_facts(path, leaves, "json", "file.json_value") {
                Ok(facts) => (facts, "json".to_owned()),
                Err(reason) => (Vec::new(), reason.to_owned()),
            },
            Err(reason) => (Vec::new(), reason.to_owned()),
        };
    }
    if trimmed.starts_with("<!-- kv3") || path.to_ascii_lowercase().ends_with(".kv3") {
        return match kv3::parse(trimmed) {
            Ok(leaves) => match leaf_facts(path, leaves, "kv3", "file.kv3_value") {
                Ok(facts) => (facts, "kv3_text_values".to_owned()),
                Err(reason) => (Vec::new(), reason.to_owned()),
            },
            Err(reason) => (Vec::new(), format!("kv3_text_preserved: {reason}")),
        };
    }
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(
        extension.as_str(),
        "txt" | "kv" | "res" | "vdf" | "vdata" | "vdata_inc" | "gi" | "kv1"
    ) {
        return match kv::parse(trimmed) {
            Ok(entries) => {
                let mut budget = budget::Budget::new();
                let facts: Result<Vec<Value>, &'static str> = entries.into_iter().map(|entry| {
                    budget.expanded(&[path.len(), entry.pointer.len(), entry.pointer.len(), entry.value.len(), entry.key.len(), entry.conditions.iter().map(String::len).sum()], 1536 + entry.conditions.len() * 128)?;
                    Ok(json!({
                        "fact_id": format!("kv:{}", entry.pointer),
                        "subject": format!("game_file:{path}"),
                        "predicate": "file.kv_value",
                        "value": entry.value,
                        "unit": null,
                        "evidence_status": "extracted_value",
                        "source_span": format!("{path}:{}", entry.pointer),
                        "qualifiers": {"source_pointer": entry.pointer, "source_key": entry.key, "occurrence": entry.occurrence, "conditions": entry.conditions, "string_encoding": "source_escape_bytes_preserved", "unit_status": "unknown", "gameplay_binding": "uninterpreted"}
                    }))
                }).collect();
                match facts {
                    Ok(facts) => (facts, "kv1_lossless_entries".to_owned()),
                    Err(reason) => (Vec::new(), reason.to_owned()),
                }
            }
            Err(reason) => (
                Vec::new(),
                format!("text_preserved_without_semantic_parser: {reason}"),
            ),
        };
    }
    (
        Vec::new(),
        "text_preserved_without_semantic_parser".to_owned(),
    )
}

fn leaf_facts(
    path: &str,
    leaves: Vec<kv3::Leaf>,
    kind: &str,
    predicate: &str,
) -> Result<Vec<Value>, &'static str> {
    let mut budget = budget::Budget::new();
    leaves.into_iter().map(|leaf| {
        budget.expanded(&[path.len(), leaf.pointer.len(), leaf.lexeme.len(), leaf.flags.iter().map(String::len).sum()], 1536 + leaf.flags.len() * 128)?;
        Ok(json!({
            "fact_id": format!("{kind}:{}", leaf.pointer),
            "subject": format!("game_file:{path}"),
            "predicate": predicate,
            "value": leaf.value,
            "unit": null,
            "evidence_status": "extracted_value",
            "source_span": format!("{path}:{}", leaf.pointer),
            "qualifiers": {"source_pointer": leaf.pointer, "json_pointer": if kind == "json" { Some(&leaf.pointer) } else { None }, "type_flags": leaf.flags, "source_lexeme": leaf.lexeme, "numeric_representation": leaf.numeric_representation, "unit_status": "unknown", "gameplay_binding": "uninterpreted", "references_resolved": false}
        }))
    }).collect()
}

pub fn canonical_resource_path<'a>(
    layout: Option<&str>,
    path: &'a str,
    loose: bool,
) -> std::borrow::Cow<'a, str> {
    if loose
        && matches!(
            layout,
            Some("gametracking-citadel-pak01-dir" | "gametracking-pak01-dir")
        )
    {
        for (presentation, resource_root) in [
            ("game/citadel/pak01_dir/", "game/citadel/"),
            ("game/core/pak01_dir/", "game/core/"),
        ] {
            if let Some(resource) = path.strip_prefix(presentation) {
                return format!("{resource_root}{resource}").into();
            }
        }
    }
    path.into()
}
