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

fn fact_reference(fact: &EntityProfileFact) -> String {
    format!(
        "{}:{}:{}:{}",
        fact.provenance.origin.identity.source_id,
        fact.provenance.origin.identity.logical_id,
        fact.provenance.original_revision,
        fact.fact_id
    )
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
        for fact in
            profile.facts.iter().chain(&profile.context).filter(|fact| {
                conflict.fact_ids.contains(&fact_reference(fact)) && public_fact(fact)
            })
        {
            html.push_str(&format!(
                "<li>{}: {}{}</li>",
                escape(&fact.fact_id),
                escape(&fact.value.to_string()),
                if conflict.preferred_fact_id.as_ref() == Some(&fact_reference(fact)) {
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
    html.push_str("<ul>");
    for source in &profile.source_state {
        html.push_str(&format!("<li>{}</li>", escape(source)));
    }
    for unknown in &profile.unknowns {
        html.push_str(&format!("<li>Unbekannt: {}</li>", escape(unknown)));
    }
    html.push_str("</ul>");
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

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::entity_profile::{
        EntityIdentity, ProfileConflict, ProfileProvenance, ProfileSourceKind,
    };
    use brain_contracts::source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision,
    };
    use serde_json::json;
    use std::collections::BTreeSet;

    fn fixture(entity_kind: EntityKind) -> EntityProfile {
        let origin = OriginArtifact {
            identity: SourceIdentity {
                source_id: "git-spieldaten".into(),
                logical_id: "hero/abrams".into(),
            },
            source_revision: SourceRevision::Git {
                commit: "a".repeat(40),
            },
            raw_sha256: "b".repeat(64),
            locator: "git:hero/abrams".into(),
            parser_revision: "fixture-v1".into(),
            parser_family: "fixture".into(),
            schema_version: Observed::known("1".into()),
            schema_sha256: Observed::known("c".repeat(64)),
            retrieved_at: Observed::unknown(brain_contracts::value::UnknownReason::NotPresent),
            source_time: Observed::unknown(brain_contracts::value::UnknownReason::NotPresent),
            language: Observed::known("de".into()),
            origin_artifacts: BTreeSet::from(["fixture".into()]),
            derivation_family: Observed::known("fixture".into()),
            policy: SourcePolicy {
                visibility: SourceVisibility::Public,
                allowed_scopes: BTreeSet::new(),
                authorization_ref: Observed::known("fixture-freigabe".into()),
                license: Observed::known("fixture-lizenz".into()),
                publication_allowed: true,
                provider_egress_allowed: true,
                raw_retention_allowed: true,
            },
            validity: GameValidity::unknown(),
        };
        let fact = EntityProfileFact {
            fact_id: "health-1".into(),
            subject: "abrams".into(),
            predicate: "Lebenspunkte".into(),
            value: json!(700),
            unit: Some("HP".into()),
            qualifiers: serde_json::from_value(json!({"level":1})).unwrap(),
            evidence_status: "extracted_value".into(),
            validity: PatchValidity::Unknown {
                reason: "Frühere Patchgrenze fehlt".into(),
            },
            provenance: ProfileProvenance {
                source_kind: ProfileSourceKind::GameFile,
                origin,
                original_revision: "a".repeat(40),
                observed_at: "2026-10-04T12:00:00Z".into(),
                source_span: Some("/hero/health".into()),
                license: json!({"name":"fixture-lizenz","redistribution_allowed":true}),
                document_metadata: Default::default(),
            },
        };
        EntityProfile {
            contract_version: ENTITY_PROFILE_VERSION.into(),
            entity: EntityIdentity {
                entity_key: "../abrams/<script>".into(),
                kind: entity_kind,
                name: "Abrams <script> & \"Test\"".into(),
                aliases: vec![],
                identity_evidence: vec![],
            },
            patch: None,
            source_state: vec!["Git-Revision bekannt; Patch unbekannt".into()],
            facts: vec![fact],
            context: vec![],
            conflicts: vec![],
            patch_story: vec![],
            unknowns: vec!["Historie vor Aufnahme unbekannt".into()],
        }
    }

    #[test]
    fn all_entity_kinds_preserve_the_stored_profile_and_public_values() {
        for entity_kind in [EntityKind::Hero, EntityKind::Ability, EntityKind::Item] {
            let profile = fixture(entity_kind);
            let rendered = render_entity_profile(&profile).unwrap();
            assert_eq!(
                serde_json::from_str::<EntityProfile>(&rendered.brain_document).unwrap(),
                profile
            );
            for expected in [
                "700",
                "HP",
                "health-1",
                "Frühere Patchgrenze fehlt",
                "Patchstand: Unbekannt",
                "Patch-Story",
            ] {
                assert!(rendered.public_html.contains(expected), "{expected}");
            }
            assert!(rendered
                .public_relative_path
                .starts_with(Path::new("site/entities").join(kind(entity_kind).0)));
        }
    }

    #[test]
    fn restricted_wiki_text_and_arbitrary_patch_text_remain_internal() {
        let mut profile = fixture(EntityKind::Hero);
        let mut wiki = profile.facts[0].clone();
        wiki.fact_id = "wiki-1".into();
        wiki.value = json!("Gesperrter Wiki-Originaltext");
        wiki.provenance.source_kind = ProfileSourceKind::Wiki;
        wiki.provenance.license = json!({"name":"unverified","redistribution_allowed":false});
        wiki.provenance.origin.policy.publication_allowed = false;
        profile.context.push(wiki);
        profile
            .patch_story
            .push(json!({"raw_text":"Gesperrter Wiki-Patchtext","redistribution_allowed":false}));
        let rendered = render_entity_profile(&profile).unwrap();
        for text in ["Gesperrter Wiki-Originaltext", "Gesperrter Wiki-Patchtext"] {
            assert!(rendered.brain_document.contains(text));
            assert!(!rendered.public_html.contains(text));
        }
        assert!(rendered.public_html.contains("nur intern verfügbar"));
    }

    #[test]
    fn conflict_retains_both_public_values_and_marks_preference() {
        let mut profile = fixture(EntityKind::Hero);
        let mut other = profile.facts[0].clone();
        other.fact_id = "health-2".into();
        other.value = json!(650);
        profile.facts.push(other);
        profile.conflicts.push(ProfileConflict {
            predicate: "Lebenspunkte".into(),
            preferred_fact_id: Some(fact_reference(&profile.facts[0])),
            fact_ids: profile.facts.iter().map(fact_reference).collect(),
            reason: "Git hat Vorrang".into(),
        });
        let rendered = render_entity_profile(&profile).unwrap();
        for text in ["Quellenkonflikt", "700", "650", "bevorzugter Beleg"] {
            assert!(rendered.public_html.contains(text));
        }
    }

    #[test]
    fn html_escapes_input_and_file_output_preserves_existing_pages() {
        let profile = fixture(EntityKind::Item);
        let root = tempfile::tempdir().unwrap();
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(!rendered.public_html.contains("<script>"));
        assert!(rendered.public_html.contains("&lt;script&gt;"));
        let target = write_public_html(root.path(), &profile).unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), rendered.public_html);
        assert!(target.starts_with(root.path()));
        assert!(write_public_html(root.path(), &profile).is_err());
        assert_eq!(fs::read_to_string(target).unwrap(), rendered.public_html);
    }

    #[test]
    fn scoped_or_unverified_facts_are_not_published() {
        let mut profile = fixture(EntityKind::Ability);
        profile.facts[0].value = json!("Nur mit Grant");
        profile.facts[0]
            .provenance
            .origin
            .policy
            .allowed_scopes
            .insert("private".into());
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("Nur mit Grant"));
        profile.facts[0]
            .provenance
            .origin
            .policy
            .allowed_scopes
            .clear();
        profile.facts[0].provenance.origin.policy.license =
            Observed::unknown(brain_contracts::value::UnknownReason::NotPresent);
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("Nur mit Grant"));
    }
}
