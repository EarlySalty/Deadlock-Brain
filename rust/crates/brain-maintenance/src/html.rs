use anyhow::{ensure, Result};
use scraper::{Html, Selector};
use std::collections::{BTreeMap, BTreeSet};

fn selector(value: &str) -> Selector {
    Selector::parse(value).expect("Feste CSS-Auswahl ist gültig")
}

fn safe_css(css: &str) -> bool {
    let compact = css
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    !compact.contains(['\\', '<', '>'])
        && !compact.contains("/*")
        && !compact.contains("url(")
        && !compact.contains("@import")
        && !compact.contains("expression(")
        && !compact.contains("-moz-binding")
        && !compact.contains("behavior:")
        && !compact.contains("image-set(")
        && !compact.contains("-webkit-image-set(")
}

fn safe_url(value: &str) -> bool {
    if value.chars().any(char::is_control) || value.contains('\\') {
        return false;
    }
    let value = value.trim();
    if value.starts_with('#') {
        return true;
    }
    if value.starts_with("https://") || value.starts_with("http://") {
        return !value.contains(['\'', '"', '<', '>']);
    }
    !value.starts_with("//") && !value.contains(':') && !value.contains(['\'', '"', '<', '>'])
}

/// Nur sichtbare Textbelege und beschriebene Bildtexte, keine Bildinterpretation.
pub fn visible_text(source: &str) -> String {
    let document = Html::parse_document(source);
    let mut lines = Vec::new();
    for text in document
        .tree
        .nodes()
        .filter_map(|node| node.value().as_text().map(|text| (node, text)))
    {
        if !text.0.ancestors().any(|node| {
            node.value()
                .as_element()
                .is_some_and(|element| matches!(element.name(), "script" | "style" | "head"))
        }) {
            let value = text.1.text.trim();
            if !value.is_empty() {
                lines.push(value.to_owned());
            }
        }
    }
    for image in document.select(&selector("img")) {
        if let Some(alt) = image.value().attr("alt") {
            lines.push(format!("Bildbeschreibung: {alt}"));
        }
    }
    lines.join("\n")
}

pub fn validate_writing(source: &str, is_html: bool) -> Result<()> {
    let text = if is_html {
        visible_text(source)
    } else {
        Html::parse_fragment(source)
            .root_element()
            .text()
            .collect::<Vec<_>>()
            .join(" ")
    };
    ensure!(
        !text.contains('\u{2014}') && !text.contains(" -- ") && !text.contains(" - "),
        "Sichtbarer Text enthält einen Gedankenstrich als Satzpause"
    );
    Ok(())
}

type ScaffoldElement = (String, Vec<(String, String)>, Option<String>);
fn scaffolding(document: &Html) -> Vec<ScaffoldElement> {
    document
        .select(&selector(
            "html,head,body,main,header,nav,footer,style,link,title",
        ))
        .map(|e| {
            (
                e.value().name().into(),
                e.value()
                    .attrs()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
                matches!(
                    e.value().name(),
                    "header" | "nav" | "footer" | "style" | "title"
                )
                .then(|| e.inner_html()),
            )
        })
        .collect()
}

pub fn validate_html(original: Option<&str>, proposed: &str) -> Result<()> {
    validate_html_with_assets(original, proposed, &[])
}

pub fn validate_html_with_assets(
    original: Option<&str>,
    proposed: &str,
    assets: &[crate::config::AssetProvenance],
) -> Result<()> {
    validate_writing(proposed, true)?;
    let document = Html::parse_document(proposed);
    let original_document = original.map(Html::parse_document);
    let mut ids = BTreeSet::new();
    let allowed: BTreeSet<&str> = [
        "html",
        "head",
        "body",
        "meta",
        "title",
        "link",
        "style",
        "main",
        "article",
        "section",
        "header",
        "footer",
        "nav",
        "div",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "p",
        "ul",
        "ol",
        "li",
        "a",
        "code",
        "pre",
        "strong",
        "b",
        "em",
        "i",
        "span",
        "br",
        "hr",
        "table",
        "caption",
        "thead",
        "tbody",
        "tr",
        "th",
        "td",
        "details",
        "summary",
        "blockquote",
        "figure",
        "figcaption",
        "img",
        "svg",
        "g",
        "path",
        "rect",
        "circle",
        "ellipse",
        "line",
        "polyline",
        "polygon",
        "text",
        "tspan",
        "defs",
        "marker",
        "title",
        "desc",
    ]
    .into();
    for element in document.select(&selector("*")) {
        let name = element.value().name();
        ensure!(
            allowed.contains(name),
            "HTML enthält ein nicht erlaubtes Element"
        );
        if name == "meta" {
            ensure!(
                element.value().attr("http-equiv").is_none(),
                "Aktive HTML-Weiterleitung gesperrt"
            );
        }
        if name == "link" {
            ensure!(
                original_document.as_ref().is_some_and(|old| old
                    .select(&selector("link"))
                    .any(|e| e.html() == element.html())),
                "Neue externe oder aktive Ressource gesperrt"
            );
        }
        if name == "style" {
            ensure!(
                original_document.as_ref().is_some_and(|old| old
                    .select(&selector("style"))
                    .any(|e| e.html() == element.html())),
                "Neue Styles sind gesperrt"
            );
        }
        for (key, value) in element.value().attrs() {
            let attribute = key.to_ascii_lowercase();
            ensure!(
                [
                    "id",
                    "class",
                    "title",
                    "lang",
                    "dir",
                    "role",
                    "aria-label",
                    "aria-labelledby",
                    "aria-describedby",
                    "aria-hidden",
                    "charset",
                    "name",
                    "content",
                    "rel",
                    "href",
                    "target",
                    "width",
                    "height",
                    "alt",
                    "src",
                    "viewbox",
                    "xmlns",
                    "x",
                    "y",
                    "x1",
                    "y1",
                    "x2",
                    "y2",
                    "rx",
                    "ry",
                    "r",
                    "cx",
                    "cy",
                    "d",
                    "points",
                    "fill",
                    "stroke",
                    "stroke-width",
                    "marker-end",
                    "marker-start",
                    "text-anchor",
                    "dominant-baseline",
                    "font-size",
                    "font-family",
                    "opacity",
                    "transform",
                    "preserveaspectratio",
                    "orient",
                    "markerwidth",
                    "markerheight",
                    "refx",
                    "refy",
                    "colspan",
                    "rowspan"
                ]
                .contains(&attribute.as_str())
                    || (attribute == "scope"
                        && name == "th"
                        && matches!(
                            value.to_ascii_lowercase().as_str(),
                            "row" | "col" | "rowgroup" | "colgroup"
                        ))
                    || (attribute == "style"
                        && original_document.as_ref().is_some_and(|old| old
                            .select(&selector("[style]"))
                            .any(|e| e.value().name() == element.value().name()
                                && e.value()
                                    .attrs()
                                    .collect::<std::collections::BTreeMap<_, _>>()
                                    == element
                                        .value()
                                        .attrs()
                                        .collect::<std::collections::BTreeMap<_, _>>()))),
                "HTML enthält ein nicht erlaubtes Attribut"
            );
            let lower = value.trim().to_ascii_lowercase();
            if matches!(key, "href" | "src" | "poster" | "cite" | "background") {
                ensure!(safe_url(value), "Ungültiges Ressourcen- oder Linkziel");
            }
            ensure!(
                !matches!(key, "srcset" | "ping" | "manifest"),
                "Nicht geprüfte externe Ressourcenattribute gesperrt"
            );
            ensure!(
                !key.to_ascii_lowercase().starts_with("on")
                    && !matches!(key, "srcdoc" | "formaction" | "action" | "xlink:href")
                    && !lower.starts_with("javascript:")
                    && !lower.starts_with("data:")
                    && !lower.starts_with("vbscript:"),
                "HTML enthält aktiven Inhalt"
            );
            if name == "svg"
                || element
                    .ancestors()
                    .any(|e| e.value().as_element().is_some_and(|v| v.name() == "svg"))
            {
                ensure!(
                    !matches!(key, "href" | "src" | "style"),
                    "SVG enthält eine aktive oder externe Referenz"
                );
                ensure!(
                    !value.contains('\\'),
                    "SVG enthält maskierte Ressourcenreferenzen"
                );
                ensure!(
                    !lower.contains("url(")
                        || (lower.starts_with("url(#")
                            && lower.ends_with(')')
                            && !lower.contains(['\'', '"', '\\'])),
                    "SVG verweist auf externe Ressourcen"
                );
            }
            if key == "style" {
                ensure!(
                    safe_css(value),
                    "Aktive oder maskierte CSS-Referenz gesperrt"
                );
            }
        }
        if let Some(id) = element.value().attr("id") {
            ensure!(
                !id.is_empty() && ids.insert(id.to_owned()),
                "Abschnitt-ID fehlt oder ist doppelt"
            );
        }
        if name == "img" {
            ensure!(
                element
                    .ancestors()
                    .any(|e| e.value().as_element().is_some_and(|v| v.name() == "figure")),
                "Bild braucht eine beschriebene Abbildung"
            );
            ensure!(
                element
                    .value()
                    .attr("src")
                    .is_some_and(|s| !s.trim().is_empty()),
                "Bildquelle fehlt"
            );
            ensure!(
                element
                    .value()
                    .attr("alt")
                    .is_some_and(|s| !s.trim().is_empty()),
                "Bildbeschreibung fehlt"
            );
        }
        if name == "figure" {
            ensure!(
                element.select(&selector("figcaption")).next().is_some(),
                "Abbildungsunterschrift fehlt"
            );
            ensure!(
                element.select(&selector("p")).any(|p| !p
                    .text()
                    .collect::<String>()
                    .trim()
                    .is_empty()),
                "Erklärung zur Abbildung fehlt"
            );
        }
        if name == "svg" {
            ensure!(
                element
                    .ancestors()
                    .any(|e| e.value().as_element().is_some_and(|v| v.name() == "figure")),
                "Grafik braucht eine beschriebene Abbildung"
            );
        }
    }
    for style in document.select(&selector("style")) {
        let css = style.text().collect::<String>().to_ascii_lowercase();
        ensure!(safe_css(&css), "Externe oder aktive CSS-Ressource gesperrt");
    }
    if let Some(old) = &original_document {
        ensure!(
            scaffolding(old) == scaffolding(&document),
            "Gemeinsame HTML-Struktur wurde verändert"
        );
        let old_ids: BTreeSet<_> = old
            .select(&selector("[id]"))
            .filter_map(|e| e.value().attr("id").map(str::to_owned))
            .collect();
        ensure!(
            old_ids.is_subset(&ids),
            "Bestehende Abschnitt-ID wurde entfernt"
        );
        let old_images: BTreeMap<_, _> = old
            .select(&selector("img"))
            .filter_map(|e| {
                e.value().attr("src").map(|src| {
                    (
                        src.to_owned(),
                        e.value().attr("alt").unwrap_or("").to_owned(),
                    )
                })
            })
            .collect();
        let images: BTreeMap<_, _> = document
            .select(&selector("img"))
            .filter_map(|e| {
                e.value().attr("src").map(|src| {
                    (
                        src.to_owned(),
                        e.value().attr("alt").unwrap_or("").to_owned(),
                    )
                })
            })
            .collect();
        ensure!(
            old_images
                .iter()
                .all(|(src, alt)| images.get(src) == Some(alt)),
            "Bestehende Bildquelle oder Provenanz wurde verändert"
        );
        for (src, alt) in images
            .iter()
            .filter(|(src, _)| !old_images.contains_key(*src))
        {
            ensure!(
                assets
                    .iter()
                    .any(|asset| asset.src == *src && asset.alt == *alt),
                "Neue Bildquelle ist nicht registriert"
            );
        }
    } else {
        ensure!(
            document.select(&selector("h1")).count() == 1
                && document.select(&selector("main")).count() == 1,
            "Neues HTML braucht genau einen Titel und Hauptbereich"
        );
        for image in document.select(&selector("img")) {
            ensure!(
                assets.iter().any(
                    |asset| image.value().attr("src") == Some(asset.src.as_str())
                        && image.value().attr("alt") == Some(asset.alt.as_str())
                ),
                "Neue Bildquelle ist nicht registriert"
            );
        }
        ensure!(
            document
                .select(&selector("section"))
                .all(|e| e.value().attr("id").is_some()),
            "Neue Abschnitte brauchen stabile IDs"
        );
        for name in [
            "source-commit",
            "documentation-status",
            "documentation-version",
        ] {
            ensure!(
                document
                    .select(&selector("meta"))
                    .any(|e| e.value().attr("name") == Some(name)
                        && e.value().attr("content").is_some_and(|s| !s.is_empty())),
                "Quellen-, Versions- oder Arbeitsstandmetadaten fehlen"
            );
        }
    }
    ensure!(!visible_text(proposed).trim().is_empty(), "Textkern fehlt");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_caption_keeps_text_without_active_attributes() {
        let old = "<main><table><tr><th>Titel</th></tr></table></main>";
        assert!(validate_html(Some(old), "<main><table><caption>Übersicht</caption><tr><th scope='col'>Titel</th></tr></table></main>").is_ok());
        for attribute in [
            "onclick='true'",
            "style='color:red'",
            "href='javascript:alert(1)'",
        ] {
            let html =
                format!("<main><table><caption {attribute}>Übersicht</caption></table></main>");
            assert!(validate_html(Some(old), &html).is_err());
        }
    }
    #[test]
    fn table_header_scope_accepts_only_semantic_header_values() {
        let old = "<main><table><tr><th>Titel</th></tr></table></main>";
        for value in ["row", "col", "rowgroup", "colgroup", "COL"] {
            let html =
                format!("<main><table><tr><th scope='{value}'>Titel</th></tr></table></main>");
            assert!(validate_html(Some(old), &html).is_ok(), "{value}");
        }
        for value in ["", "auto", "unknown", " col ", "javascript:alert(1)"] {
            let html =
                format!("<main><table><tr><th scope='{value}'>Titel</th></tr></table></main>");
            assert!(validate_html(Some(old), &html).is_err(), "{value}");
        }
        for html in [
            "<main><table><tr><td scope='col'>Text</td></tr></table></main>",
            "<main><p scope='row'>Text</p></main>",
            "<main><table><tr><th scope='col' onclick='true'>Titel</th></tr></table></main>",
        ] {
            assert!(validate_html(Some(old), html).is_err());
        }
    }
    #[test]
    fn unchanged_inline_style_allows_changed_text_but_rejects_attribute_changes() {
        let old = "<main><p id='satz' style='color:red'>Alt</p></main>";
        assert!(validate_html(
            Some(old),
            "<main><p id='satz' style='color:red'>Neu</p></main>"
        )
        .is_ok());
        assert!(validate_html(
            Some(old),
            "<main><p id='satz' style='color:blue'>Neu</p></main>"
        )
        .is_err());
        assert!(validate_html(
            Some(old),
            "<main><p id='anderer-satz' style='color:red'>Neu</p></main>"
        )
        .is_err());
    }
    #[test]
    fn text_only_extraction_keeps_caption_and_alt_and_excludes_script() {
        let text=visible_text("<main><h1>Quelle</h1><img src='x' alt='Ablauf'><figcaption>Erklärung</figcaption></main><script>unsichtbar</script>");
        assert!(
            text.contains("Quelle")
                && text.contains("Ablauf")
                && text.contains("Erklärung")
                && !text.contains("unsichtbar")
        );
    }
    #[test]
    fn entities_and_active_content_are_rejected() {
        for entity in ["&mdash;", "&#8212;", "&#x2014;"] {
            assert!(validate_writing(&format!("<p>Text{entity}Text</p>"), true).is_err());
        }
        assert!(validate_html(
            Some("<main><p>Text</p></main>"),
            "<main><p onclick='true'>Text</p></main>"
        )
        .is_err());
        assert!(validate_html(
            Some("<main><p>Text</p></main>"),
            "<main><a href='jav&#x09;ascript:alert(1)'>Text</a></main>"
        )
        .is_err());
        assert!(validate_html(
            Some("<main><p>Text</p></main>"),
            "<main><p style='background:u\\72l(https://example.invalid/x)'>Text</p></main>"
        )
        .is_err());
        assert!(validate_html(
            Some("<main><img src='x.png' alt='Text'></main>"),
            "<main><img src='x.png' alt='Text' srcset='https://example.invalid/x.png 2x'></main>"
        )
        .is_err());
        assert!(validate_html(
            Some("<main><p>Text</p></main>"),
            "<main><svg><foreignObject>Text</foreignObject></svg></main>"
        )
        .is_err());
        assert!(validate_html(
            Some("<main><p>Text</p></main>"),
            "<main><svg><rect fill='u\\72l(https://example.invalid/p.svg#p)'/></svg></main>"
        )
        .is_err());
    }
}
