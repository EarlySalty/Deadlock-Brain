use anyhow::{ensure, Context, Result};
use brain_contracts::entity_profile::{
    EntityKind, EntityProfile, EntityProfileFact, PatchValidity, ENTITY_PROFILE_VERSION,
};
use brain_contracts::{value::Observed, SourceVisibility};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedEntityProfile {
    pub brain_document: String,
    pub public_html: String,
    pub public_relative_path: PathBuf,
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn kind(kind: EntityKind) -> (&'static str, &'static str) {
    match kind {
        EntityKind::Hero => ("hero", "Held"),
        EntityKind::Ability => ("ability", "Fähigkeit"),
        EntityKind::Item => ("item", "Item"),
    }
}

pub fn public_relative_path(profile: &EntityProfile) -> Result<PathBuf> {
    ensure!(
        !profile.entity.entity_key.is_empty() && profile.entity.entity_key.len() <= 100,
        "Entitätsschlüssel fehlt oder ist zu lang"
    );
    Ok(Path::new("site/entities")
        .join(kind(profile.entity.kind).0)
        .join(format!(
            "{}.html",
            hex::encode(profile.entity.entity_key.as_bytes())
        )))
}

fn public_fact(fact: &EntityProfileFact) -> bool {
    let policy = &fact.provenance.origin.policy;
    policy.publication_allowed
        && policy.visibility == SourceVisibility::Public
        && policy.allowed_scopes.is_empty()
        && matches!(&policy.authorization_ref, Observed::Known { value } if !value.trim().is_empty())
        && matches!(&policy.license, Observed::Known { value } if !value.trim().is_empty() && !value.eq_ignore_ascii_case("unverified"))
        && fact
            .provenance
            .license
            .get("redistribution_allowed")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && fact
            .provenance
            .license
            .get("name")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|name| !name.trim().is_empty() && !name.eq_ignore_ascii_case("unverified"))
}

fn validity(value: &PatchValidity) -> String {
    match value {
        PatchValidity::Unknown { reason } => format!("Unbekannt: {reason}"),
        PatchValidity::Known {
            from_patch,
            to_patch_exclusive,
            evidence_ref,
        } => format!(
            "Ab {from_patch}; bis {} (exklusiv); Beleg: {evidence_ref}",
            to_patch_exclusive.as_deref().unwrap_or("offen")
        ),
    }
}

fn fact_table(html: &mut String, facts: &[EntityProfileFact]) {
    html.push_str("<table><caption>Zugeordnete Werte und Belege</caption><thead><tr><th scope=\"col\">Fakt</th><th scope=\"col\">Wert und Typ</th><th scope=\"col\">Einheit und Bedingungen</th><th scope=\"col\">Gültigkeit</th><th scope=\"col\">Quelle</th></tr></thead><tbody>");
    for fact in facts {
        if public_fact(fact) {
            let value_type = match &fact.value {
                serde_json::Value::Null => "Unbekannt (null)",
                serde_json::Value::Bool(_) => "Boolean",
                serde_json::Value::Number(_) => "Zahl",
                serde_json::Value::String(_) => "Text",
                serde_json::Value::Array(_) => "Array",
                serde_json::Value::Object(_) => "Objekt",
            };
            html.push_str(&format!(
                "<tr><td>{}: {} ({})</td><td><code>{}</code> ({})</td><td>{}; <code>{}</code></td><td>{}</td><td>{}; Revision {}; beobachtet {}; Belegstatus {}; Quellspanne {}; Herkunft {}; Parser {}</td></tr>",
                escape(&fact.fact_id), escape(&fact.predicate), escape(&fact.subject),
                escape(&fact.value.to_string()), value_type,
                escape(fact.unit.as_deref().unwrap_or("Unbekannt")),
                escape(&serde_json::Value::Object(fact.qualifiers.clone()).to_string()),
                escape(&validity(&fact.validity)),
                escape(&fact.provenance.origin.identity.source_id),
                escape(&fact.provenance.original_revision), escape(&fact.provenance.observed_at),
                escape(&fact.evidence_status), escape(fact.provenance.source_span.as_deref().unwrap_or("Unbekannt")),
                escape(&fact.provenance.origin.locator), escape(&fact.provenance.origin.parser_revision),
            ));
        } else {
            html.push_str("<tr><td colspan=\"5\">Ein zugeordneter Beleg ist nur intern verfügbar. Eine öffentliche Freigabe fehlt.</td></tr>");
        }
    }
    if facts.is_empty() {
        html.push_str("<tr><td colspan=\"5\">Keine zugeordneten Werte vorhanden. Der Quellenstand bleibt unvollständig.</td></tr>");
    }
    html.push_str("</tbody></table>");
}

pub fn render_entity_profile(profile: &EntityProfile) -> Result<RenderedEntityProfile> {
    ensure!(
        profile.contract_version == ENTITY_PROFILE_VERSION,
        "Unbekannter Steckbriefvertrag"
    );
    for fact in profile.facts.iter().chain(&profile.context) {
        fact.provenance
            .origin
            .validate()
            .map_err(anyhow::Error::msg)?;
    }
    let public_relative_path = public_relative_path(profile)?;
    let brain_document = serde_json::to_string(profile)?;
    let mut html = format!(
        "<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"source-commit\" content=\"{}\"><meta name=\"documentation-status\" content=\"Quellengebundener Steckbrief\"><meta name=\"documentation-version\" content=\"{}\"><title>{}</title></head><body><main><h1>{}</h1><p>{}; Patchstand: {}</p>",
        escape(profile.patch.as_deref().unwrap_or("Unbekannt")), ENTITY_PROFILE_VERSION,
        escape(&profile.entity.name), escape(&profile.entity.name), kind(profile.entity.kind).1,
        escape(profile.patch.as_deref().unwrap_or("Unbekannt")),
    );
    html.push_str("<section id=\"werte\"><h2>Aktuelle Werte</h2>");
    fact_table(&mut html, &profile.facts);
    html.push_str("</section><section id=\"kontext\"><h2>Beschreibung und Kontext</h2>");
    fact_table(&mut html, &profile.context);
    html.push_str("</section><section id=\"konflikte\"><h2>Quellenkonflikte</h2>");
    if profile.conflicts.is_empty() {
        html.push_str("<p>Keine Quellenkonflikte gespeichert.</p>");
    }
    for conflict in &profile.conflicts {
        html.push_str(
            "<p>Quellenkonflikt gespeichert. Abweichende Belege bleiben erhalten.</p><ul>",
        );
        for fact in profile
            .facts
            .iter()
            .chain(&profile.context)
            .filter(|fact| conflict.fact_ids.contains(&fact.fact_id) && public_fact(fact))
        {
            html.push_str(&format!(
                "<li>{}: {}{}</li>",
                escape(&fact.fact_id),
                escape(&fact.value.to_string()),
                if conflict.preferred_fact_id.as_ref() == Some(&fact.fact_id) {
                    " (bevorzugter Beleg)"
                } else {
                    ""
                }
            ));
        }
        html.push_str("</ul>");
    }
    html.push_str("</section><section id=\"patch-story\"><h2>Patch-Story</h2>");
    if profile.patch_story.is_empty() {
        html.push_str(
            "<p>Keine zugeordneten Änderungen gespeichert. Historische Lücken bleiben offen.</p>",
        );
    } else {
        html.push_str(&format!("<p>{} Änderungen gespeichert. Die Einzelbelege sind intern verfügbar; ein öffentlicher Freigabevertrag fehlt.</p>", profile.patch_story.len()));
    }
    html.push_str("</section><section id=\"quellenstand\"><h2>Quellenstand und Lücken</h2>");
    html.push_str(&format!("<p>{} Quellenstände und {} offene Angaben gespeichert. Ein fehlender Patchstand wird nicht aus Beobachtungsdaten abgeleitet.</p>", profile.source_state.len(), profile.unknowns.len()));
    html.push_str("</section></main></body></html>");
    Ok(RenderedEntityProfile {
        brain_document,
        public_html: html,
        public_relative_path,
    })
}

pub fn write_public_html(corpus_root: &Path, profile: &EntityProfile) -> Result<PathBuf> {
    let rendered = render_entity_profile(profile)?;
    let root = corpus_root
        .canonicalize()
        .context("Corpusroot ist nicht verfügbar")?;
    let target = root.join(&rendered.public_relative_path);
    let mut directory = root;
    for component in rendered
        .public_relative_path
        .parent()
        .context("Ausgabepfad fehlt")?
        .components()
    {
        directory.push(component);
        match fs::create_dir(&directory) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.into()),
        }
        let metadata = fs::symlink_metadata(&directory)?;
        ensure!(
            metadata.is_dir() && !metadata.file_type().is_symlink(),
            "Ausgabeverzeichnis ist kein reguläres Verzeichnis"
        );
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
        .context("HTML-Datei lässt sich nicht neu anlegen; vorhandene Seiten bleiben erhalten")?;
    file.write_all(rendered.public_html.as_bytes())?;
    file.sync_all()?;
    Ok(target)
}
