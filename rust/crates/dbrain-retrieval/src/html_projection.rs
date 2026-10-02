//! Deterministische Textableitung aus geprüftem HTML ohne Ressourcenabruf.
use brain_contracts::PortError;
use scraper::{node::Node, Html};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const HTML_PROJECTION_VERSION: &str = "html-semantic-v1";
const MAX_BYTES: usize = 8 * 1024 * 1024;

pub struct HtmlProjection {
    pub text: String,
    pub raw_sha256: String,
    pub semantic_sha256: String,
}

impl HtmlProjection {
    pub fn bind_metadata(&self, metadata: &mut BTreeMap<String, String>) {
        metadata.insert("content_format".into(), "html".into());
        metadata.insert(
            "html_projection_version".into(),
            HTML_PROJECTION_VERSION.into(),
        );
        metadata.insert("html_raw_sha256".into(), self.raw_sha256.clone());
        metadata.insert("html_semantic_sha256".into(), self.semantic_sha256.clone());
    }
}

pub fn project_html(raw: &str) -> Result<HtmlProjection, PortError> {
    if raw.len() > MAX_BYTES {
        return Err(PortError::BudgetExceeded);
    }
    let document = Html::parse_document(raw);
    let mut text = String::new();
    for node in document.tree.root().descendants() {
        let mut skip = false;
        for (depth, ancestor) in node.ancestors().enumerate() {
            if depth > 256 {
                return Err(PortError::BudgetExceeded);
            }
            if let Node::Element(element) = ancestor.value() {
                if matches!(
                    element.name(),
                    "script"
                        | "style"
                        | "iframe"
                        | "object"
                        | "embed"
                        | "template"
                        | "noscript"
                        | "head"
                ) {
                    skip = true;
                    break;
                }
            }
        }
        if skip {
            continue;
        }
        match node.value() {
            Node::Text(value) => text.push_str(value),
            Node::Element(element) => {
                let name = element.name();
                if matches!(
                    name,
                    "h1" | "h2"
                        | "h3"
                        | "h4"
                        | "h5"
                        | "h6"
                        | "p"
                        | "div"
                        | "section"
                        | "article"
                        | "li"
                        | "tr"
                        | "br"
                        | "figure"
                        | "figcaption"
                        | "pre"
                ) {
                    text.push('\n');
                }
                if matches!(name, "td" | "th") {
                    text.push_str(" | ");
                }
                if matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
                    if let Some(id) = element.attr("id") {
                        text.push_str("[#");
                        text.push_str(id);
                        text.push_str("] ");
                    }
                }
                if name == "img" {
                    if let Some(alt) = element.attr("alt") {
                        text.push(' ');
                        text.push_str(alt);
                        text.push(' ');
                    }
                }
            }
            _ => {}
        }
        if text.len() > MAX_BYTES {
            return Err(PortError::BudgetExceeded);
        }
    }
    let text = text
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(HtmlProjection {
        raw_sha256: format!("{:x}", Sha256::digest(raw.as_bytes())),
        semantic_sha256: format!("{:x}", Sha256::digest(text.as_bytes())),
        text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn html_preserves_semantics_without_active_content_or_requests() {
        let raw = "<html><head><title>Nicht lesen</title><script>GIFT</script></head><body><h1 id='setup'>Einrichtung</h1><p>Mit <b>Steam</b> verbinden.</p><figure><img src='https://invalid.test/never-fetch' alt='Steam-Knopf'><figcaption>Abbildung 1</figcaption></figure><script>GEHEIM</script><iframe>VERBOTEN</iframe></body></html>";
        let projection = project_html(raw).unwrap();
        assert_eq!(
            projection.text,
            "[#setup] Einrichtung\nMit Steam verbinden.\nSteam-Knopf\nAbbildung 1"
        );
        assert_eq!(
            projection.raw_sha256,
            format!("{:x}", Sha256::digest(raw.as_bytes()))
        );
        assert_eq!(
            projection.semantic_sha256,
            project_html(raw).unwrap().semantic_sha256
        );
        assert_ne!(projection.raw_sha256, projection.semantic_sha256);
    }
}
