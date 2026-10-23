use anyhow::{Context, Result};
use serde_json::{json, Value};
use sqlx::{PgConnection, Row};

use super::profiles::{display_name, escape, fact_rows};

pub(super) struct Ability {
    slot: i32,
    identity: Value,
    link: Value,
    facts: Vec<Value>,
}

pub(super) struct Change {
    label: String,
    date: Option<String>,
    text: String,
    citation: Value,
}

pub(super) async fn abilities(connection: &mut PgConnection, key: &str) -> Result<Vec<Ability>> {
    let rows = sqlx::query(
        "SELECT slot,ability,link,facts FROM serve.site_hero_abilities($1) ORDER BY slot LIMIT 8",
    )
    .bind(key)
    .fetch_all(connection)
    .await?;
    rows.into_iter()
        .map(|row| {
            let facts: Value = row.try_get("facts")?;
            Ok(Ability {
                slot: row.try_get("slot")?,
                identity: row.try_get("ability")?,
                link: row.try_get("link")?,
                facts: facts.as_array().cloned().unwrap_or_default(),
            })
        })
        .collect()
}

pub(super) async fn changes(connection: &mut PgConnection, key: &str) -> Result<Vec<Change>> {
    let rows = sqlx::query("SELECT patch_label,to_char(released_at AT TIME ZONE 'UTC','DD.MM.YYYY') AS released,line_text,citation,source_revision FROM serve.site_hero_patch_changes($1) LIMIT 160")
        .bind(key)
        .fetch_all(connection)
        .await?;
    rows.into_iter()
        .map(|row| {
            let line: String = row.try_get("line_text")?;
            Ok(Change {
                label: row
                    .try_get::<Option<String>, _>("patch_label")?
                    .unwrap_or_default(),
                date: row.try_get("released")?,
                text: line.trim().trim_start_matches("- ").trim().to_owned(),
                citation: json!({
                    "citation": row.try_get::<String, _>("citation")?,
                    "source_revision": row.try_get::<String, _>("source_revision")?,
                }),
            })
        })
        .collect()
}

pub(super) async fn owner(connection: &mut PgConnection, key: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar("SELECT serve.site_ability_hero($1)")
        .bind(key)
        .fetch_one(connection)
        .await?)
}

pub(super) fn anchor(key: &str) -> String {
    key.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

pub(super) fn plain(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start..].find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[start + 1..start + end];
        let name = tag
            .trim_start_matches('/')
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        if name == "br" {
            out.push('\n');
        } else if name == "img" {
            if let Some(alt) = attribute(tag, "alt").filter(|alt| !alt.starts_with('_')) {
                out.push_str(alt);
            }
        }
        rest = &rest[start + end + 1..];
    }
    out.push_str(rest);
    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");
    decoded
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("{name}=\"");
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].trim())
}

fn paragraph(text: &str) -> String {
    plain(text)
        .lines()
        .map(escape)
        .collect::<Vec<_>>()
        .join("<br>")
}

fn beleg(fact: &Value) -> String {
    let citation = fact["citation"].as_str().unwrap_or("");
    format!(" <a href=\"#beleg-{0}\">Beleg {0}</a>", escape(citation))
}

fn condition<'a>(fact: &'a Value, field: &str) -> Option<&'a Value> {
    fact["condition"].get(field)
}

fn is_delta(fact: &Value) -> bool {
    condition(fact, "operation").and_then(Value::as_str) == Some("delta")
}

fn pick<'a>(facts: &'a [Value], stat_key: &str) -> Option<&'a Value> {
    let matching = || {
        facts.iter().filter(move |fact| {
            fact["stat_key"] == stat_key
                && fact["value"]
                    .as_str()
                    .is_some_and(|value| !plain(value).is_empty())
        })
    };
    ["de", "en"]
        .iter()
        .find_map(|lang| {
            matching().find(|fact| condition(fact, "lang").and_then(Value::as_str) == Some(*lang))
        })
        .or_else(|| matching().next())
}

fn label(fact: &Value) -> Option<&str> {
    fact["label"]
        .as_str()
        .map(str::trim)
        .filter(|label| !label.is_empty())
}

fn shown_value(value: &Value) -> bool {
    match value {
        Value::Number(number) => number.as_f64().is_some_and(|n| n != 0.0),
        Value::String(text) => {
            let text = text.trim();
            let number = text.trim_end_matches(|c: char| c.is_alphabetic() || c == '%' || c == ' ');
            match number.parse::<f64>() {
                Ok(number) => number != 0.0,
                Err(_) => !text.is_empty(),
            }
        }
        Value::Bool(flag) => *flag,
        _ => false,
    }
}

fn amount(fact: &Value, unit: Option<&str>, signed: bool) -> String {
    let value = &fact["value"];
    let mut text = match value {
        Value::String(text) => text.trim().to_owned(),
        other => other.to_string(),
    };
    if signed && value.as_f64().is_some_and(|n| n > 0.0) {
        text.insert(0, '+');
    }
    match unit.filter(|unit| !unit.is_empty() && !text.ends_with(unit)) {
        Some(unit) => format!("{text} {unit}"),
        None => text,
    }
}

fn scaling(fact: &Value) -> String {
    let scale = condition(fact, "scale_value").and_then(Value::as_str);
    match (condition(fact, "scale_type").and_then(Value::as_str), scale) {
        (Some("ETechPower"), Some(scale)) => {
            format!("skaliert mit Spirit Power (×{})", escape(scale))
        }
        _ => String::new(),
    }
}

fn base_facts(facts: &[Value]) -> Vec<&Value> {
    facts
        .iter()
        .filter(|fact| {
            !is_delta(fact)
                && condition(fact, "lang").is_none()
                && label(fact).is_some()
                && shown_value(&fact["value"])
        })
        .collect()
}

fn values_table(facts: &[Value], shown: &mut Vec<Value>) -> String {
    let rows = base_facts(facts);
    if rows.is_empty() {
        return String::new();
    }
    let mut html = String::from("<h4>Werte</h4><div class=\"scroll\"><table><thead><tr><th scope=\"col\">Angabe</th><th scope=\"col\">Wert</th><th scope=\"col\">Hinweis und Beleg</th></tr></thead><tbody>");
    for fact in rows {
        html.push_str(&format!(
            "<tr><th scope=\"row\">{}</th><td>{}</td><td>{}{}</td></tr>",
            escape(label(fact).unwrap_or("")),
            escape(&amount(fact, fact["unit"].as_str(), false)),
            scaling(fact),
            beleg(fact)
        ));
        shown.push(fact.clone());
    }
    html.push_str("</tbody></table></div>");
    html
}

fn base_for<'a>(facts: &'a [Value], stat_key: &Value) -> Option<&'a Value> {
    facts
        .iter()
        .find(|fact| !is_delta(fact) && &fact["stat_key"] == stat_key && label(fact).is_some())
}

fn tier_deltas(facts: &[Value], tier: u64) -> Vec<(&Value, &Value)> {
    let mut deltas: Vec<_> = facts
        .iter()
        .filter(|fact| {
            is_delta(fact)
                && condition(fact, "upgrade_tier").and_then(Value::as_u64) == Some(tier)
                && fact["value"].as_f64().is_some_and(|n| n != 0.0)
        })
        .filter_map(|fact| base_for(facts, &fact["stat_key"]).map(|base| (fact, base)))
        .collect();
    deltas.sort_by_key(|(fact, _)| condition(fact, "upgrade_entry").and_then(Value::as_u64));
    deltas
}

fn upgrades(facts: &[Value], shown: &mut Vec<Value>) -> String {
    let mut html = String::new();
    for tier in 1..=3u64 {
        let description = pick(facts, &format!("ability_description_t{tier}_desc"));
        let deltas = tier_deltas(facts, tier);
        if description.is_none() && deltas.is_empty() {
            continue;
        }
        html.push_str(&format!("<li><strong>Stufe {tier}</strong>"));
        if let Some(fact) = description {
            html.push_str(&format!(
                "<p>{}{}</p>",
                paragraph(fact["value"].as_str().unwrap_or("")),
                beleg(fact)
            ));
            shown.push(fact.clone());
        }
        if !deltas.is_empty() {
            html.push_str("<ul>");
            for (fact, base) in deltas {
                let unit = fact["unit"].as_str().or_else(|| base["unit"].as_str());
                html.push_str(&format!(
                    "<li>{}: {}{}</li>",
                    escape(label(base).unwrap_or("")),
                    escape(&amount(fact, unit, true)),
                    beleg(fact)
                ));
                shown.push(fact.clone());
                shown.push(base.clone());
            }
            html.push_str("</ul>");
        }
        html.push_str("</li>");
    }
    if html.is_empty() {
        return html;
    }
    format!("<h4>Upgrades</h4><ol>{html}</ol>")
}

fn slot_label(slot: i32) -> String {
    if slot == 4 {
        "Ultimative Fähigkeit".into()
    } else {
        format!("Fähigkeit {slot}")
    }
}

fn ability_title(ability: &Ability) -> String {
    display_name(&ability.identity)
        .map(str::to_owned)
        .unwrap_or_else(|| slot_label(ability.slot))
}

fn ability_names(ability: &Ability) -> Vec<String> {
    let mut names: Vec<String> = ability
        .facts
        .iter()
        .filter(|fact| fact["stat_key"] == "name_en" || fact["stat_key"] == "name_de")
        .filter_map(|fact| fact["value"].as_str())
        .chain(display_name(&ability.identity))
        .map(|name| name.trim().to_lowercase())
        .filter(|name| name.chars().count() >= 3)
        .collect();
    names.sort();
    names.dedup();
    names
}

fn change_items(changes: &[&Change], shown: &mut Vec<Value>) -> String {
    let mut html = String::from("<ul>");
    for change in changes {
        let when = match &change.date {
            Some(date) if !change.label.is_empty() => format!("{} ({date})", change.label),
            Some(date) => date.clone(),
            None => change.label.clone(),
        };
        html.push_str(&format!(
            "<li><strong>{}</strong>: {}{}</li>",
            escape(&when),
            escape(&change.text),
            beleg(&change.citation)
        ));
        shown.push(change.citation.clone());
    }
    html.push_str("</ul>");
    html
}

fn ability_section(
    ability: &Ability,
    changes: &[&Change],
    shown: &mut Vec<Value>,
) -> Result<String> {
    let key = ability.identity["key"].as_str().context("ability_key")?;
    let mut html = format!(
        "<section id=\"{}\"><h3>{}</h3><p>{}{}</p>",
        escape(&anchor(key)),
        escape(&ability_title(ability)),
        escape(&slot_label(ability.slot)),
        beleg(&ability.link)
    );
    shown.push(ability.link.clone());
    if let Some(quip) = pick(&ability.facts, "ability_description_quip") {
        html.push_str(&format!(
            "<p><em>{}</em>{}</p>",
            paragraph(quip["value"].as_str().unwrap_or("")),
            beleg(quip)
        ));
        shown.push(quip.clone());
    }
    if let Some(description) = pick(&ability.facts, "ability_description_desc") {
        html.push_str(&format!(
            "<p>{}{}</p>",
            paragraph(description["value"].as_str().unwrap_or("")),
            beleg(description)
        ));
        shown.push(description.clone());
    }
    html.push_str(&values_table(&ability.facts, shown));
    html.push_str(&upgrades(&ability.facts, shown));
    if !changes.is_empty() {
        html.push_str("<h4>Patchverlauf</h4>");
        html.push_str(&change_items(changes, shown));
    }
    html.push_str("<p><a href=\"#inhalt\">Zum Inhalt</a></p></section>");
    Ok(html)
}

fn contents(abilities: &[Ability]) -> Result<String> {
    let mut html = String::from("<nav id=\"inhalt\" aria-label=\"Inhalt\"><h2>Inhalt</h2><ul><li><a href=\"#grundwerte\">Grundwerte</a></li><li><a href=\"#waffe\">Waffe</a></li><li><a href=\"#faehigkeiten\">Fähigkeiten</a><ul>");
    for ability in abilities {
        let key = ability.identity["key"].as_str().context("ability_key")?;
        html.push_str(&format!(
            "<li><a href=\"#{}\">{}</a></li>",
            escape(&anchor(key)),
            escape(&ability_title(ability))
        ));
    }
    html.push_str("</ul></li><li><a href=\"#patchverlauf\">Patchverlauf</a></li><li><a href=\"#quellen\">Quellen</a></li></ul></nav>");
    Ok(html)
}

pub(super) fn render(
    profile: &Value,
    abilities: &[Ability],
    changes: &[Change],
    shown: &mut Vec<Value>,
) -> Result<String> {
    let mut html = contents(abilities)?;
    html.push_str(&format!(
        "<h2 id=\"grundwerte\">Grundwerte</h2>{}",
        fact_rows(&profile["stats"])
    ));
    html.push_str(&format!(
        "<h2 id=\"waffe\">Waffe</h2>{}",
        fact_rows(&profile["weapon"])
    ));
    shown.push(json!({"stats": profile["stats"], "weapon": profile["weapon"], "identity": profile["identity"]}));
    html.push_str("<h2 id=\"faehigkeiten\">Fähigkeiten</h2>");
    if abilities.is_empty() {
        html.push_str("<p>Für diesen Helden sind keine belegten Fähigkeiten verfügbar.</p>");
    }
    let mut used = vec![false; changes.len()];
    for ability in abilities {
        let names = ability_names(ability);
        let mut own = Vec::new();
        for (index, change) in changes.iter().enumerate() {
            let text = change.text.to_lowercase();
            if !used[index] && names.iter().any(|name| text.contains(name.as_str())) {
                used[index] = true;
                own.push(change);
            }
        }
        html.push_str(&ability_section(ability, &own, shown)?);
    }
    let rest: Vec<&Change> = changes
        .iter()
        .zip(&used)
        .filter(|(_, used)| !**used)
        .map(|(change, _)| change)
        .collect();
    html.push_str("<h2 id=\"patchverlauf\">Patchverlauf</h2>");
    if rest.is_empty() {
        html.push_str("<p>Keine weiteren belegten Patchänderungen.</p>");
    } else {
        html.push_str(&change_items(&rest, shown));
    }
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_is_reduced_to_plain_text() {
        assert_eq!(
            plain("<span class=\"highlight\">+1 s</span> Schlafdauer<br>Fügt <img src=\"x\" alt=\"_icon:' slow\"><img alt=\"Geist\"/> &amp; mehr <svg><g></g></svg>"),
            "+1 s Schlafdauer\nFügt Geist & mehr"
        );
        assert_eq!(paragraph("a<br><script>x</script>"), "a<br>x");
        assert_eq!(paragraph("&lt;b&gt;"), "&lt;b&gt;");
        assert_eq!(
            anchor("ability_sleep_dagger\"><x"),
            "ability_sleep_dagger---x"
        );
    }
}
