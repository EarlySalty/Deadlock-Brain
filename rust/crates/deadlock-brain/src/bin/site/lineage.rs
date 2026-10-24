use anyhow::Result;
use serde_json::Value;
use sqlx::PgConnection;

use super::profiles::{escape, web_link};

pub(super) async fn load(connection: &mut PgConnection, key: &str) -> Result<Value> {
    Ok(
        sqlx::query_scalar::<_, Option<Value>>("SELECT serve.site_item_lineage($1)")
            .bind(key)
            .fetch_one(connection)
            .await?
            .unwrap_or(Value::Null),
    )
}

fn german_date(day: &str) -> Option<String> {
    chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d")
        .ok()
        .map(|date| date.format("%d.%m.%Y").to_string())
}

fn patch(row: &Value) -> Option<String> {
    let label = row["patch_label"].as_str().map(str::trim).unwrap_or("");
    let date = row["patch_date"].as_str().and_then(german_date);
    match (label.is_empty(), date) {
        (false, Some(date)) => Some(format!("{label} ({date})")),
        (true, Some(date)) => Some(date),
        (false, None) => Some(label.to_owned()),
        (true, None) => None,
    }
}

fn with_patch(verb: &str, row: &Value) -> String {
    match patch(row) {
        Some(patch) => format!("{verb} mit {}", escape(&patch)),
        None => format!("{verb}, Patch nicht dokumentiert"),
    }
}

fn link(url: &Value, text: &str) -> String {
    url.as_str()
        .and_then(web_link)
        .map(|url| format!(" <a href=\"{}\">{text}</a>", escape(url)))
        .unwrap_or_default()
}

fn beleg(row: &Value) -> String {
    row["citation"]
        .as_str()
        .map(|id| format!(" <a href=\"#beleg-{0}\">Beleg {0}</a>", escape(id)))
        .unwrap_or_default()
}

fn sources(row: &Value) -> String {
    let patch = link(&row["patch_url"], "Patchnotes");
    let source = if row["source_url"] == row["patch_url"] {
        String::new()
    } else {
        link(&row["source_url"], "Quelle")
    };
    format!("{patch}{source}{}", beleg(row))
}

fn item_name(names: &Value) -> Option<String> {
    let name = |field: &str| {
        names[field]
            .as_str()
            .map(str::trim)
            .filter(|name| !name.is_empty())
    };
    match (name("name_de"), name("name_en")) {
        (Some(de), Some(en)) if de != en => Some(format!("{de} ({en})")),
        (Some(name), _) | (None, Some(name)) => Some(name.to_owned()),
        (None, None) => None,
    }
}

fn item_link(names: &Value) -> Option<String> {
    let title = item_name(names)?;
    let key = names["key"].as_str()?;
    Some(format!(
        "<a href=\"/brain/site/steckbriefe/item/{}\">{}</a>",
        hex::encode(key),
        escape(&title)
    ))
}

fn rows<'a>(lineage: &'a Value, key: &str) -> &'a [Value] {
    lineage[key].as_array().map(Vec::as_slice).unwrap_or(&[])
}

pub(super) fn removed(lineage: &Value) -> String {
    let removed = &lineage["removed"];
    if lineage["status"] != "removed" || !removed.is_object() {
        return String::new();
    }
    let successor = item_link(&removed["successor"])
        .map(|link| format!(" Nachfolger: {link}."))
        .unwrap_or_default();
    format!(
        "<h2 id=\"entfernt\">Nicht mehr im Shop</h2><p>Dieses Item wurde {}.{successor}{}</p>",
        with_patch("entfernt", removed),
        sources(removed)
    )
}

fn former_names(lineage: &Value) -> String {
    let names = rows(lineage, "former_names");
    if names.is_empty() {
        return String::new();
    }
    let mut html = String::from("<h2 id=\"fruehere-namen\">Frühere Namen</h2><ul>");
    for name in names {
        let Some(former) = name["name"].as_str() else {
            continue;
        };
        html += &format!(
            "<li><strong>{}</strong>, {}{}</li>",
            escape(former),
            with_patch("umbenannt", name),
            sources(name)
        );
    }
    html + "</ul>"
}

fn predecessors(lineage: &Value) -> String {
    let items: Vec<String> = rows(lineage, "predecessors")
        .iter()
        .filter_map(|item| {
            let name = escape(&item_name(item)?);
            let removed = &item["removed"];
            Some(format!(
                "<li><strong>{name}</strong>, {}{}</li>",
                with_patch("entfernt", removed),
                sources(removed)
            ))
        })
        .collect();
    if items.is_empty() {
        return String::new();
    }
    format!(
        "<h2 id=\"ersetzte-items\">Ersetzte Items</h2><p>Dieses Item hat folgende Items abgelöst:</p><ul>{}</ul>",
        items.concat()
    )
}

fn history(lineage: &Value) -> String {
    let changes = rows(&lineage["history"], "changes");
    if changes.is_empty() {
        return String::new();
    }
    let mut html = String::from("<h2 id=\"aenderungsverlauf\">Änderungsverlauf</h2><ul>");
    for change in changes {
        let Some(text) = change["line_text"].as_str() else {
            continue;
        };
        html += &format!(
            "<li><strong>{}</strong>: {}{}</li>",
            escape(&patch(change).unwrap_or_else(|| "Patch".into())),
            escape(super::hero::strip_bullet(text)),
            sources(change)
        );
    }
    html += "</ul>";
    if let Some(omitted) = lineage["history"]["omitted_count"]
        .as_u64()
        .filter(|omitted| *omitted > 0)
    {
        html += &format!("<p>{omitted} ältere Einträge sind nicht aufgeführt.</p>");
    }
    html
}

pub(super) fn render(lineage: &Value) -> String {
    if !lineage.is_object() {
        return String::new();
    }
    [
        former_names(lineage),
        predecessors(lineage),
        history(lineage),
    ]
    .concat()
}
