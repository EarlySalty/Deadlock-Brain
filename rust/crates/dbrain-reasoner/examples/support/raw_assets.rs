//! Read-only inventory of cached API payloads, not a simulation coverage claim.
//! All names are display data. Decisions use only structural fields and references.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

type Error = Box<dyn std::error::Error>;

fn load(path: &str) -> Result<(Value, Value), Error> {
    let bytes = fs::read(path)?;
    let value: Value = serde_json::from_slice(&bytes)?;
    let rows = value
        .as_array()
        .ok_or("Expected an API array; unsupported schema")?;
    if rows.is_empty() || rows.iter().any(|row| !row.is_object()) {
        return Err("Expected a nonempty array of object records".into());
    }
    let source = json!({
        "path": path,
        "sha256": format!("{:x}", Sha256::digest(&bytes)),
        "bytes": bytes.len(),
        "records": rows.len(),
        "client_version": null,
        "version_status": "Not inferred from filename, mtime or retrieval order"
    });
    Ok((value, source))
}

fn number(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str()?.trim().parse().ok())
        .filter(|value: &f64| value.is_finite())
}

fn pointer(parent: &str, key: &str) -> String {
    format!("{parent}/{}", key.replace('~', "~0").replace('/', "~1"))
}

fn scaling_edges(value: &Value, path: &str, edges: &mut Vec<Value>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let location = pointer(path, key);
                if key == "scale_function" && !child.is_null() {
                    let function = child.get("subclass").unwrap_or(child);
                    let magnitude = function.get("stat_scale").and_then(number);
                    edges.push(json!({
                        "path": location,
                        "kind": "property_scale_function",
                        "input_stat": function.get("specific_stat_scale_type"),
                        "coefficient": magnitude,
                        "negative": magnitude.is_some_and(|v| v < 0.0),
                        "quantified": false,
                        "raw": child
                    }));
                }
                if key == "scaling_stats" {
                    if let Some(stats) = child.as_object() {
                        for (stat, raw) in stats {
                            edges.push(json!({
                                "path": pointer(&location, stat),
                                "kind": "hero_stat_scaling",
                                "target_stat": stat,
                                "raw": raw,
                                "quantified": false
                            }));
                        }
                    }
                }
                scaling_edges(child, &location, edges);
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                scaling_edges(child, &format!("{path}/{index}"), edges);
            }
        }
        _ => {}
    }
}

fn record(row: &Value) -> Value {
    let mut edges = Vec::new();
    scaling_edges(row, "", &mut edges);
    let properties: Vec<Value> = row
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .map(|(name, property)| {
            let value = property.get("value").unwrap_or(property);
            json!({
                "path": pointer("/properties", name),
                "name": name,
                "numeric_literal": number(value),
                "raw_value": value,
                "conditional": property.get("conditional"),
                "negative_attribute": property.get("negative_attribute"),
                "provided_property_type": property.get("provided_property_type"),
                "display_units": property.get("display_units"),
                "usage_flags": property.get("usage_flags"),
                "raw": property
            })
        })
        .collect();
    json!({
        "id": row.get("id"), "name": row.get("name"),
        "class_name": row.get("class_name"), "type": row.get("type"),
        "description": row.get("description"), "item_card": row.get("item_card"),
        "properties": properties, "scaling_edges": edges,
        "simulation_coverage": "not_measured"
    })
}

fn audit(heroes: &Value, items: &Value) -> Result<Value, Error> {
    let heroes = heroes.as_array().ok_or("Heroes must be an array")?;
    let items = items.as_array().ok_or("Items must be an array")?;
    if heroes.is_empty() || items.is_empty() {
        return Err("Empty assets cannot establish roster coverage".into());
    }
    let mut by_class: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (index, item) in items.iter().enumerate() {
        if let Some(class) = item.get("class_name").and_then(Value::as_str) {
            if !class.is_empty() {
                by_class.entry(class).or_default().push(index);
            }
        }
    }
    let item_records: Vec<_> = items.iter().map(record).collect();
    let mut unresolved = 0usize;
    let mut hero_records = Vec::new();
    for hero in heroes {
        let mut entry = record(hero);
        let mut bindings = Vec::new();
        for slot in [
            "weapon_primary",
            "signature1",
            "signature2",
            "signature3",
            "signature4",
        ] {
            let class = hero
                .get("items")
                .and_then(|items| items.get(slot))
                .and_then(Value::as_str)
                .filter(|class| !class.is_empty());
            let candidates = class.and_then(|class| by_class.get(class));
            let indices = candidates.cloned().unwrap_or_default();
            let status = match (class, indices.len()) {
                (None, _) => "missing_reference",
                (Some(_), 0) => "missing_definition",
                (Some(_), 1) => "resolved",
                _ => "ambiguous_definition",
            };
            if status != "resolved" {
                unresolved += 1;
            }
            bindings.push(json!({
                "slot": slot, "class_name": class, "status": status,
                "item_indices": indices
            }));
        }
        entry["bindings"] = json!(bindings);
        entry["active"] = hero.get("in_roster").cloned().unwrap_or(Value::Null);
        hero_records.push(entry);
    }
    let property_count: usize = item_records
        .iter()
        .map(|item| item["properties"].as_array().map_or(0, Vec::len))
        .sum();
    let mut raw_conditions = 0usize;
    let mut nonnumeric = 0usize;
    let mut negative_scales = 0usize;
    let mut edge_count = 0usize;
    for item in item_records.iter().chain(hero_records.iter()) {
        for property in item["properties"].as_array().into_iter().flatten() {
            if !property["conditional"].is_null()
                && property["conditional"]
                    .as_str()
                    .is_some_and(|s| !s.trim().is_empty())
            {
                raw_conditions += 1;
            }
            if property["numeric_literal"].is_null() {
                nonnumeric += 1;
            }
        }
        for edge in item["scaling_edges"].as_array().into_iter().flatten() {
            edge_count += 1;
            if edge["negative"] == true {
                negative_scales += 1;
            }
        }
    }
    Ok(json!({
        "contract": "Raw data inventory only. No item-purpose labels, scores or assertion of understood mechanics. Missing and ambiguous references stay unresolved. No live data, database or AI call.",
        "summary": {"heroes": heroes.len(), "definitions": items.len(),
            "item_properties": property_count, "explicit_condition_fields": raw_conditions,
            "non_numeric_literals": nonnumeric, "raw_scaling_edges": edge_count,
            "negative_property_scales": negative_scales, "unresolved_bindings": unresolved},
        "heroes": hero_records, "items": item_records
    }))
}

fn compact(report: &Value) -> Value {
    let mut conversions = Vec::new();
    let mut negative = Vec::new();
    let mut conditions = Vec::new();
    let mut unresolved = Vec::new();
    for hero in report["heroes"].as_array().into_iter().flatten() {
        let edges: Vec<_> = hero["scaling_edges"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|edge| edge["kind"] == "hero_stat_scaling")
            .filter(|edge| {
                ["per_spirit", "spirit_scale", "scale"].iter().any(|key| {
                    edge["raw"]
                        .get(key)
                        .and_then(number)
                        .is_some_and(|v| v != 0.0)
                })
            })
            .cloned()
            .collect();
        if !edges.is_empty() {
            conversions.push(json!({"hero_id":hero["id"], "name":hero["name"], "edges":edges}));
        }
        for binding in hero["bindings"].as_array().into_iter().flatten() {
            if binding["status"] != "resolved" {
                unresolved
                    .push(json!({"hero_id":hero["id"], "name":hero["name"], "binding":binding}));
            }
        }
    }
    for item in report["items"].as_array().into_iter().flatten() {
        for edge in item["scaling_edges"].as_array().into_iter().flatten() {
            if edge["negative"] == true {
                negative.push(json!({"id":item["id"], "name":item["name"], "class_name":item["class_name"], "edge":edge}));
            }
        }
        for property in item["properties"].as_array().into_iter().flatten() {
            if property["conditional"]
                .as_str()
                .is_some_and(|s| !s.trim().is_empty())
            {
                conditions.push(json!({"id":item["id"], "name":item["name"], "property":property}));
            }
        }
    }
    json!({"format_version":1, "contract":report["contract"], "summary":report["summary"],
        "sources":report["sources"], "hero_scaling_candidates":conversions,
        "negative_property_scale_candidates":negative, "explicit_conditions":conditions,
        "unresolved_bindings":unresolved})
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let (heroes_path, items_path, output, summary) =
        match args {
            [heroes, items, output] => (heroes, items, output, None),
            [heroes, items, output, summary] => (heroes, items, output, Some(summary)),
            _ => return Err(
                "Aufruf: ability_coverage raw HEROES.json ITEMS.json OUTPUT.json [SUMMARY.json]"
                    .into(),
            ),
        };
    if Path::new(output).exists() || summary.is_some_and(|path| Path::new(path).exists()) {
        return Err("Ausgabe existiert bereits; Eingabedaten werden nie ueberschrieben".into());
    }
    if summary == Some(output) {
        return Err("Vollbericht und Zusammenfassung benoetigen verschiedene Pfade".into());
    }
    let (heroes, hero_source) = load(heroes_path)?;
    let (items, item_source) = load(items_path)?;
    let mut report = audit(&heroes, &items)?;
    report["sources"] = json!({"heroes": hero_source, "items": item_source});
    let summary_bytes = serde_json::to_vec_pretty(&compact(&report))?;
    eprintln!("{}", serde_json::to_string(&report["summary"])?);
    super::support::write_new(output, &serde_json::to_vec_pretty(&report)?)?;
    if let Some(path) = summary {
        if let Err(error) = super::support::write_new(path, &summary_bytes) {
            fs::remove_file(output).map_err(|cleanup| {
                format!("{error}; Ausgaberuecknahme fehlgeschlagen: {cleanup}")
            })?;
            return Err(error.into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_report_keeps_non_fire_rate_conversions_and_sources() {
        let heroes = json!([{"id":1,"scaling_stats":{"EClipSize":{"scale":0.5,"scaling_stat":"ETechPower"}}}]);
        let items = json!([{"class_name":"a"}]);
        let mut report = audit(&heroes, &items).unwrap();
        report["sources"] = json!({"fixture":"explicit"});
        let small = compact(&report);
        assert_eq!(
            small["hero_scaling_candidates"][0]["edges"][0]["target_stat"],
            "EClipSize"
        );
        assert_eq!(small["sources"]["fixture"], "explicit");
        let bytes = serde_json::to_vec(&small).unwrap();
        for _ in 0..3 {
            assert_eq!(bytes, serde_json::to_vec(&compact(&report)).unwrap());
        }
    }

    #[test]
    fn preexisting_summary_prevents_any_full_report_write() {
        let dir = std::env::temp_dir().join(format!("brain-raw-summary-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let summary = dir.join("summary.json");
        let output = dir.join("full.json");
        fs::write(&summary, b"original").unwrap();
        assert!(run(&[
            "missing-heroes".into(),
            "missing-items".into(),
            output.to_string_lossy().into(),
            summary.to_string_lossy().into()
        ])
        .is_err());
        assert!(!output.exists());
        assert_eq!(fs::read(&summary).unwrap(), b"original");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn numeric_literals_preserve_sign_but_do_not_guess_units_or_invalid_data() {
        assert_eq!(number(&json!("-0.186")), Some(-0.186));
        assert_eq!(number(&json!(0)), Some(0.0));
        for value in [
            json!("NaN"),
            json!("inf"),
            json!("13%"),
            json!("5s"),
            json!(true),
            Value::Null,
        ] {
            assert_eq!(number(&value), None);
        }
    }

    #[test]
    fn nested_scaling_keeps_target_source_sign_and_raw_provenance() {
        let raw = json!({"properties":{"x/y~z":{"scale_function":{"subclass":{
            "specific_stat_scale_type":"ETechPower", "stat_scale":"-0.186"
        }}}}});
        let entry = record(&raw);
        let edge = &entry["scaling_edges"][0];
        assert_eq!(edge["path"], "/properties/x~1y~0z/scale_function");
        assert_eq!(edge["coefficient"], -0.186);
        assert_eq!(edge["negative"], true);
        assert_eq!(edge["quantified"], false);
        assert_eq!(edge["input_stat"], "ETechPower");
    }

    #[test]
    fn unknown_scaling_is_retained_not_replaced_with_zero() {
        let entry = record(&json!({"properties":{"p":{"scale_function":{"new_schema":7}}}}));
        assert!(entry["scaling_edges"][0]["coefficient"].is_null());
        assert_eq!(entry["scaling_edges"][0]["raw"]["new_schema"], 7);
    }

    #[test]
    fn hero_conversions_and_conditional_metadata_are_preserved() {
        let entry = record(
            &json!({"scaling_stats":{"EFireRate":{"scaling_stat":"ETechPower","scale":0.25}},
            "properties":{"HealthRegen":{"value":"4", "conditional":"on_damage", "negative_attribute":false}}}),
        );
        assert_eq!(entry["scaling_edges"][0]["target_stat"], "EFireRate");
        assert_eq!(entry["properties"][0]["conditional"], "on_damage");
        assert_eq!(entry["properties"][0]["numeric_literal"], 4.0);
    }

    #[test]
    fn class_resolution_rejects_ambiguous_or_missing_assets() {
        let heroes = json!([{"name":"neutral", "items":{"signature1":"a", "signature2":"b", "signature3":"gone"}}]);
        let items = json!([{"class_name":"a"}, {"class_name":"b"}, {"class_name":"b"}]);
        let report = audit(&heroes, &items).unwrap();
        assert_eq!(report["summary"]["unresolved_bindings"], 4);
        assert_eq!(report["heroes"][0]["bindings"][1]["status"], "resolved");
        assert_eq!(
            report["heroes"][0]["bindings"][2]["status"],
            "ambiguous_definition"
        );
        assert_eq!(
            report["heroes"][0]["bindings"][3]["status"],
            "missing_definition"
        );
    }

    #[test]
    fn display_names_cannot_change_mechanical_summary() {
        let heroes = json!([{"name":"any", "items":{"signature1":"a"}}]);
        let items = json!([{"name":"any", "class_name":"a", "properties":{"p":{"value":"-3"}}}]);
        let before = audit(&heroes, &items).unwrap();
        let mut renamed = items.clone();
        renamed[0]["name"] = json!("unrelated reference name");
        assert_eq!(
            before["summary"],
            audit(&heroes, &renamed).unwrap()["summary"]
        );
        assert!(audit(&json!([]), &items).is_err());
    }

    #[test]
    fn existing_input_cannot_be_used_as_output() {
        let dir = std::env::temp_dir().join(format!("brain-raw-assets-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let input = dir.join("input.json");
        fs::write(&input, b"[{\"id\":1}]").unwrap();
        let path = input.to_string_lossy().to_string();
        assert!(run(&[path.clone(), path.clone(), path]).is_err());
        assert_eq!(fs::read(&input).unwrap(), b"[{\"id\":1}]");
        fs::remove_dir_all(dir).unwrap();
    }
}
