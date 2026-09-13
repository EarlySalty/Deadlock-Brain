use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};

type Error = Box<dyn std::error::Error>;
mod support;

fn number(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str()?.trim_end_matches('%').parse().ok())
        .filter(|value| value.is_finite())
}

fn category(name: &str, property: &Value) -> Option<&'static str> {
    let key = name.to_lowercase();
    let css = property["css_class"]
        .as_str()
        .unwrap_or_default()
        .to_lowercase();
    if [
        "damage",
        "dps",
        "lifedrain",
        "bleed",
        "burn",
        "healthtodamage",
    ]
    .iter()
    .any(|part| key.contains(part))
        || css.contains("damage")
    {
        Some("Schaden oder schadensbezogenes Hilfsfeld")
    } else if ["heal", "regen", "lifesteal", "healthswap"]
        .iter()
        .any(|part| key.contains(part))
        || css.contains("heal")
    {
        Some("Heilung oder Heilungsbedingung")
    } else if [
        "stun", "slow", "root", "silence", "disarm", "knock", "duration", "tick", "interval",
        "channel", "cooldown",
    ]
    .iter()
    .any(|part| key.contains(part))
    {
        Some("Kontrolle oder Zeitbedingung")
    } else {
        None
    }
}

fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [input, output, markdown] = args.as_slice() else {
        return Err("Aufruf: ability_coverage FROZEN JSON MARKDOWN".into());
    };
    if std::path::Path::new(output).exists() || std::path::Path::new(markdown).exists() {
        return Err("Ausgabe existiert bereits".into());
    }
    let destination = |value: &str| -> Result<std::path::PathBuf, Error> {
        let path = std::path::Path::new(value);
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new("."));
        Ok(parent
            .canonicalize()?
            .join(path.file_name().ok_or("Ausgabedateiname fehlt")?))
    };
    if destination(output)? == destination(markdown)? {
        return Err("JSON und Markdown benötigen verschiedene Ausgabedateien".into());
    }
    let frozen: Value = serde_json::from_slice(&fs::read(input)?)?;
    let mut raw = frozen["raw_snapshots"]
        .as_array()
        .ok_or("Rohsnapshots fehlen")?
        .iter()
        .filter(|row| {
            row["source"] == "deadlock_assets_api" && row["entity_type"] == "item_or_ability"
        })
        .collect::<Vec<_>>();
    raw.sort_by(|a, b| {
        b["fetched_at"]
            .as_str()
            .cmp(&a["fetched_at"].as_str())
            .then_with(|| b["id"].as_i64().cmp(&a["id"].as_i64()))
    });
    let mut entries = Vec::new();
    let mut counts = BTreeMap::<String, usize>::new();
    let mut table = String::from("# Fähigkeitsabdeckung auf FROZEN-V2\n\nRohfelder sind Prüfkandidaten, keine ungeprüft addierbaren Effekte. Prozentwerte, Schwellen und Hilfsfelder bleiben getrennt im JSON sichtbar. `base_effect` und Skalierungen stammen aus der alten Baseline, Rohwerte aus demselben eingefrorenen Snapshot. Ein Nullwert ist bei reinen Kontroll-/Tauschfähigkeiten nicht automatisch falsch. ID 0 darf Fähigkeiten nicht aus dem Kampf entfernen; die ID bleibt für Imbue ungültig.\n\n| Held | Slot / Fähigkeit | ID | Basis / Typ | Skalierungen | Positive rohe Schadensfelder | Befund |\n|---|---|---:|---|---|---|---|\n");
    for hero in frozen["heroes"].as_array().ok_or("Helden fehlen")? {
        let model = &hero["hero"];
        let name = model["name"].as_str().ok_or("Heldenname fehlt")?;
        for ability in model["abilities"].as_array().ok_or("Fähigkeiten fehlen")? {
            let id = ability["ability_id"]
                .as_i64()
                .ok_or("Fähigkeits-ID fehlt")?;
            let class = ability["class_name"]
                .as_str()
                .ok_or("Fähigkeitsklasse fehlt")?;
            let snapshot = raw.iter().find(|row| {
                (id > 0 && row["payload"]["id"].as_i64() == Some(id))
                    || row["payload"]["class_name"].as_str() == Some(class)
            });
            let mut properties = Vec::new();
            let mut positive_damage = Vec::new();
            let mut raw_scaling = Vec::new();
            if let Some(row) = snapshot {
                for (key, value) in row["payload"]["properties"]
                    .as_object()
                    .into_iter()
                    .flatten()
                {
                    if value.get("scale_function").is_some() {
                        raw_scaling.push(json!({"property":key,"scale_function":value["scale_function"],"css_class":value["css_class"],"value":value["value"]}));
                    }
                    if let Some(kind) = category(key, value) {
                        let numeric = number(value.get("value").unwrap_or(value));
                        if numeric.is_some_and(|value| value > 0.0) && kind.starts_with("Schaden") {
                            positive_damage.push(format!("{}={}", key, numeric.unwrap()));
                        }
                        properties.push(
                            json!({"property":key,"category":kind,"numeric":numeric,"raw":value}),
                        );
                    }
                }
            }
            let base = ability["base_effect"].as_f64().unwrap_or_default();
            let mut findings = Vec::new();
            if snapshot.is_none() {
                findings.push("Rohsnapshot fehlt");
            }
            if id <= 0 {
                findings.push(
                    "Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie",
                );
            }
            if base == 0.0 && !positive_damage.is_empty() {
                findings.push(
                    "Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen",
                );
            }
            if findings.is_empty() {
                findings.push("Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe");
            }
            for finding in &findings {
                *counts.entry((*finding).into()).or_default() += 1;
            }
            let scaling = ability["scaling"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|value| {
                    format!(
                        "{}:{}",
                        value["stat"].as_str().unwrap_or("?"),
                        value["per_spirit"]
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            table.push_str(&format!(
                "| {} | {} / {} | {} | {} / {} | {} | {} | {} |\n",
                name,
                ability["slot"],
                class,
                id,
                base,
                ability["damage_type"].as_str().unwrap_or("?"),
                scaling,
                positive_damage.join(", "),
                findings.join("; ")
            ));
            entries.push(json!({"hero_id":model["hero_id"],"hero_name":name,"model":ability,"raw_snapshot_id":snapshot.map(|row|row["id"].clone()),"raw_fetched_at":snapshot.map(|row|row["fetched_at"].clone()),"raw_ability_id":snapshot.map(|row|row["payload"]["id"].clone()),"raw_description":snapshot.map(|row|row["payload"]["description"].clone()),"raw_upgrades":snapshot.map(|row|row["payload"]["upgrades"].clone()),"properties":properties,"raw_scaling":raw_scaling,"findings":findings}));
        }
    }
    support::write_new(
        output,
        &serde_json::to_vec_pretty(
            &json!({"source":"FROZEN-V2, keine neue DB-Abfrage","baseline_revision":frozen["baseline_revision"],"frozen_at":frozen["measured_at"],"ability_count":entries.len(),"finding_counts":counts,"abilities":entries}),
        )?,
    )?;
    if let Err(error) = support::write_new(markdown, table.as_bytes()) {
        fs::remove_file(output)?;
        return Err(error.into());
    }
    eprintln!(
        "{} Fähigkeiten, {} Helden",
        entries.len(),
        frozen["heroes"].as_array().map_or(0, Vec::len)
    );
    Ok(())
}
