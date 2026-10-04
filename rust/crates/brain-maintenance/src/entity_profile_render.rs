use anyhow::{ensure, Context, Result};
use brain_contracts::entity_profile::{
    EntityKind, EntityProfile, EntityProfileFact, PatchStoryChange, PatchValidity,
    ProfileSourceKind, ENTITY_PROFILE_VERSION,
};
use brain_contracts::SourceVisibility;
use brain_storage::entity_profile::derivation::{
    public_qualifier_text, public_statement_qualifiers,
};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
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

fn public_label(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty() && !value.contains(['/', '\\', ':', '.']) && !value.contains("game_file"))
        .then_some(value)
}

fn public_fact_source(fact: &EntityProfileFact) -> bool {
    fact.provenance.source_kind == ProfileSourceKind::GameFile
        && matches!(
            fact.provenance.origin.source_revision,
            brain_contracts::source::SourceRevision::Git { .. }
        )
        && fact.provenance.origin.policy.visibility == SourceVisibility::Public
        && fact.provenance.origin.policy.allowed_scopes.is_empty()
        && fact.provenance.origin.policy.publication_allowed
}

fn public_fact(fact: &EntityProfileFact) -> bool {
    public_fact_source(fact)
        && public_statement_qualifiers(&fact.qualifiers)
        && fact.unit.as_ref().is_none_or(|unit| {
            public_qualifier_text(&serde_json::Value::String(unit.clone())).is_some()
        })
        && !fact.qualifiers.contains_key("semantic_scope")
        && public_label(&fact.predicate).is_some()
        && fact
            .qualifiers
            .get("gameplay_binding")
            .and_then(serde_json::Value::as_str)
            != Some("uninterpreted")
        && (fact.value.is_number()
            || (fact
                .qualifiers
                .get("numeric_representation")
                .and_then(serde_json::Value::as_str)
                == Some("source_numeric_lexeme")
                && story_number(&fact.value).is_some()))
}

fn validity(value: &PatchValidity) -> String {
    match value {
        PatchValidity::Unknown { .. } => "Unbekannt: Eine belegte Patchgrenze fehlt.".into(),
        PatchValidity::Known {
            from_patch,
            through_patch_inclusive,
            ..
        } if through_patch_inclusive.is_some() => format!(
            "Ab {}; belegt bis einschließlich {}",
            public_label(from_patch).unwrap_or("Unbekannt"),
            through_patch_inclusive
                .as_deref()
                .and_then(public_label)
                .unwrap_or("Unbekannt"),
        ),
        PatchValidity::Known {
            from_patch,
            to_patch_exclusive,
            ..
        } => format!(
            "Ab {}; bis {} (exklusiv)",
            public_label(from_patch).unwrap_or("Unbekannt"),
            match to_patch_exclusive.as_deref() {
                Some(patch) => public_label(patch).unwrap_or("Unbekannt"),
                None => "offen",
            }
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
                "<tr><td>{}</td><td><code>{}</code> ({})</td><td>{}; {}</td><td>{}</td><td>Git-Spieldaten</td></tr>",
                escape(public_label(&fact.predicate).unwrap_or("Unbekannter Wert")),
                escape(&fact.value.to_string()), value_type,
                escape(fact.unit.as_deref().and_then(public_label).unwrap_or("Unbekannt")),
                public_conditions(fact),
                escape(&validity(&fact.validity)),
            ));
        } else if public_fact_source(fact) && !public_statement_qualifiers(&fact.qualifiers) {
            html.push_str("<tr><td colspan=\"5\">Für einen Wert fehlt eine belegte öffentliche Beschreibung der nötigen Variante oder Bedingung.</td></tr>");
        } else {
            html.push_str(
                "<tr><td colspan=\"5\">Ein zugeordneter Beleg ist nur intern verfügbar.</td></tr>",
            );
        }
    }
    if facts.is_empty() {
        html.push_str("<tr><td colspan=\"5\">Keine zugeordneten Werte vorhanden. Der Quellenstand bleibt unvollständig.</td></tr>");
    }
    html.push_str("</tbody></table>");
}

fn public_conditions(fact: &EntityProfileFact) -> String {
    public_qualifiers(&fact.qualifiers)
}

fn public_qualifiers(qualifiers: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut values = Vec::new();
    for (key, label) in [
        ("level", "Stufe"),
        ("variant", "Variante"),
        ("condition", "Bedingung"),
        ("ability_name", "Fähigkeit"),
        ("unit", "Einheit"),
    ] {
        if let Some(value) = qualifiers.get(key) {
            let text = public_qualifier_text(value);
            if let Some(text) = text {
                values.push(format!("{label}: {}", escape(&text)));
            }
        }
    }
    if values.is_empty() {
        "Keine öffentliche Bedingung angegeben".into()
    } else {
        values.join("; ")
    }
}

fn story_number(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Number(number) => Some(number.to_string()),
        serde_json::Value::String(value) => {
            serde_json::from_str::<serde_json::Number>(value.trim())
                .ok()
                .map(|_| value.trim().to_owned())
        }
        _ => None,
    }
}

fn patch_story(html: &mut String, changes: &[PatchStoryChange]) -> Result<()> {
    if changes.is_empty() {
        html.push_str(
            "<p>Keine zugeordneten Änderungen gespeichert. Historische Lücken bleiben offen.</p>",
        );
        return Ok(());
    }
    html.push_str("<ol>");
    for change in changes {
        ensure!(
            change.provenance.relation == "brain.patch_changes"
                && !change.provenance.evidence_ref.trim().is_empty(),
            "Patchänderung hat keine belegte Herkunft aus brain.patch_changes"
        );
        if !public_statement_qualifiers(&change.additional_fields) {
            html.push_str("<li>Eine Patchaussage wird zurückgehalten: Eine belegte öffentliche Beschreibung der nötigen Bedingung oder Variante fehlt.</li>");
            continue;
        }
        let date = chrono::NaiveDate::parse_from_str(&change.patch_date, "%Y-%m-%d")
            .map(|date| date.format("%d.%m.%Y").to_string())
            .unwrap_or_else(|_| "Unbekannt".into());
        let entity_kind = match change.entity_type.as_deref() {
            Some("hero") => "Held",
            Some("ability") => "Fähigkeit",
            Some("item") => "Item",
            _ => "Entität",
        };
        let values = match (
            story_number(&change.old_value),
            story_number(&change.new_value),
        ) {
            (Some(old), Some(new)) => format!("Der Wert wurde von {old} auf {new} geändert."),
            (None, Some(new)) => format!("Neuer Wert: {new}. Der frühere Wert ist unbekannt."),
            (Some(old), None) => format!("Früherer Wert: {old}. Der neue Wert ist unbekannt."),
            (None, None) => "Zahlenwerte sind unbekannt.".into(),
        };
        let change_type = match change.change_type.as_deref() {
            Some("buff") => "Verstärkung",
            Some("nerf") => "Abschwächung",
            Some("added") => "Ergänzung",
            Some("removed") => "Entfernung",
            Some("bugfix") => "Fehlerkorrektur",
            Some("rework") => "Überarbeitung",
            Some("changed") => "Änderung",
            _ => "Unbekannt",
        };
        let direction = match change.numeric_direction.as_deref() {
            Some("increase" | "increased" | "up") => "gestiegen",
            Some("decrease" | "decreased" | "down") => "gesunken",
            Some("unchanged" | "equal") => "unverändert",
            _ => "unbekannt",
        };
        html.push_str(&format!(
            "<li><h3>Patch vom {}: {}</h3><p>{}: {}; Fähigkeit: {}; Wert: {}.</p><p>{} Änderungsart: {}; Zahlenrichtung: {}; Konfidenz: {}.</p><p>{}</p><p>Quelle: Patchhistorie.</p></li>",
            escape(&date), escape(change.patch_title.as_deref().and_then(public_label).unwrap_or("Titel unbekannt")),
            entity_kind, escape(change.entity_name.as_deref().and_then(public_label).unwrap_or("Unbekannt")),
            escape(change.ability_name.as_deref().and_then(public_label).unwrap_or("Unbekannt")),
            escape(change.stat_name.as_deref().and_then(public_label).unwrap_or("Unbekannt")), escape(&values),
            change_type, direction,
            escape(&story_number(&change.confidence).unwrap_or_else(|| "Unbekannt".into())),
            public_qualifiers(&change.additional_fields),
        ));
    }
    html.push_str("</ol>");
    Ok(())
}

fn compact_document(profile: &EntityProfile) -> Result<String> {
    Ok(brain_storage::entity_profile::compact::compact_document(
        profile,
    )?)
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
    let brain_document = compact_document(profile)?;
    let patch = profile
        .patch
        .as_deref()
        .and_then(public_label)
        .unwrap_or("Unbekannt");
    let name = public_label(&profile.entity.name).unwrap_or("Entitätsname unbekannt");
    let mut html = format!(
        "<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"source-commit\" content=\"{}\"><meta name=\"documentation-status\" content=\"Quellengebundener Steckbrief\"><meta name=\"documentation-version\" content=\"{}\"><title>{}</title></head><body><main><h1>{}</h1><p>{}; Patchstand: {}</p>",
        escape(patch), ENTITY_PROFILE_VERSION,
        escape(name), escape(name), kind(profile.entity.kind).1,
        escape(patch),
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
                "<li>{}: {}{}; {}</li>",
                escape(public_label(&fact.predicate).unwrap_or("Unbekannter Wert")),
                escape(&fact.value.to_string()),
                if conflict.preferred_fact_id.as_ref() == Some(&fact_reference(fact)) {
                    " (bevorzugter Beleg)"
                } else {
                    ""
                },
                public_conditions(fact)
            ));
        }
        html.push_str("</ul>");
    }
    html.push_str("</section><section id=\"patch-story\"><h2>Patch-Story</h2>");
    patch_story(&mut html, &profile.patch_story)?;
    html.push_str("</section><section id=\"quellenstand\"><h2>Quellenstand und Lücken</h2>");
    html.push_str(&format!("<p>{} Quellenstände und {} offene Angaben gespeichert. Ein fehlender Patchstand wird nicht aus Beobachtungsdaten abgeleitet.</p>", profile.source_state.len(), profile.unknowns.len()));
    html.push_str("<p>Git-Spieldaten liefern zugeordnete Zahlen. Wiki-Belege bleiben intern. Die Freigabe der Spielwerte bestätigt keinen aktuellen Patchstand. Technische Herkunft und Einzelheiten zu offenen Angaben bleiben intern.</p>");
    html.push_str("</section></main></body></html>");
    Ok(RenderedEntityProfile {
        brain_document,
        public_html: html,
        public_relative_path,
    })
}

fn publish_html(
    target: &Path,
    write: impl FnOnce(&mut fs::File) -> std::io::Result<()>,
) -> Result<()> {
    match fs::symlink_metadata(target) {
        Ok(metadata) => ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink(),
            "HTML-Ziel ist keine reguläre Datei"
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let mut temporary = tempfile::Builder::new()
        .permissions(fs::Permissions::from_mode(0o666))
        .tempfile_in(target.parent().context("Ausgabeverzeichnis fehlt")?)?;
    write(temporary.as_file_mut())
        .context("HTML-Datei konnte nicht vollständig gesichert werden")?;
    temporary
        .persist(target)
        .map_err(|error| error.error)
        .context("HTML-Datei konnte nicht atomar veröffentlicht werden")?;
    Ok(())
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
    publish_html(&target, |file| {
        file.write_all(rendered.public_html.as_bytes())?;
        file.sync_all()
    })?;
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::entity_profile::{
        EntityIdentity, PatchStoryProvenance, ProfileConflict, ProfileProvenance,
        ProfileSourceKind, RestrictedPatchLine,
    };
    use brain_contracts::source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision,
    };
    use brain_contracts::value::Observed;
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
            let document: serde_json::Value =
                serde_json::from_str(&rendered.brain_document).unwrap();
            assert_eq!(document["facts"][0]["value"], profile.facts[0].value);
            assert_eq!(
                document["facts"][0]["qualifiers"],
                json!(profile.facts[0].qualifiers)
            );
            assert_eq!(document["sources"].as_array().unwrap().len(), 1);
            for expected in [
                "700",
                "HP",
                "Lebenspunkte",
                "Eine belegte Patchgrenze fehlt",
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
        profile.patch_story.push(story_fixture());
        let rendered = render_entity_profile(&profile).unwrap();
        for text in ["Gesperrter Wiki-Originaltext", "Gesperrter Wiki-Patchtext"] {
            assert!(serde_json::to_string(&profile).unwrap().contains(text));
            assert!(!rendered.public_html.contains(text));
        }
        assert!(rendered
            .brain_document
            .contains("Gesperrter Wiki-Originaltext"));
        assert!(!rendered
            .brain_document
            .contains("Gesperrter Wiki-Patchtext"));
        assert!(rendered.public_html.contains("nur intern verfügbar"));
    }

    fn story_fixture() -> PatchStoryChange {
        PatchStoryChange {
            patch_date: "2026-09-16".into(),
            patch_title: Some("September <script>".into()),
            entity_type: Some("hero".into()),
            entity_name: Some("Abrams".into()),
            ability_name: Some("Siphon Life".into()),
            stat_name: Some("Schaden".into()),
            old_value: json!("100.00000000000000001"),
            new_value: json!(123),
            change_type: Some("buff".into()),
            numeric_direction: Some("increase".into()),
            confidence: json!(0.8),
            provenance: PatchStoryProvenance {
                relation: "brain.patch_changes".into(),
                source_url: Some("https://example.org/patch".into()),
                evidence_ref: "brain.patch_changes:2026-09-16:abrams:damage".into(),
            },
            original_line: RestrictedPatchLine {
                text: Some("Gesperrter Wiki-Patchtext".into()),
                redistribution_allowed: false,
            },
            additional_fields: serde_json::from_value(json!({"extra":"Interner Zusatz"})).unwrap(),
        }
    }

    #[test]
    fn restricted_original_numbers_stay_internal_until_publication_is_allowed() {
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0]
            .provenance
            .origin
            .policy
            .publication_allowed = false;
        profile.facts[0].provenance.origin.policy.license = Observed::known("unverified".into());
        profile.facts[0].provenance.license =
            json!({"name":"unverified","redistribution_allowed":false});
        let mut string = profile.facts[0].clone();
        string.fact_id = "numeric-string".into();
        string.value = json!("100.00000000000000001");
        string.qualifiers.insert(
            "numeric_representation".into(),
            json!("source_numeric_lexeme"),
        );
        string
            .qualifiers
            .insert("variant".into(), json!("Verstärkt"));
        string
            .qualifiers
            .insert("condition".into(), json!("Während der Fähigkeit"));
        profile.facts.push(string);
        let original = serde_json::to_value(&profile).unwrap();
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(!rendered.public_html.contains("<code>700</code>"));
        assert!(!rendered.public_html.contains("100.00000000000000001"));
        assert_eq!(serde_json::to_value(&profile).unwrap(), original);
        for fact in &mut profile.facts {
            fact.provenance.origin.policy.publication_allowed = true;
        }
        let original = serde_json::to_value(&profile).unwrap();
        let rendered = render_entity_profile(&profile).unwrap();
        for text in [
            "700",
            "&quot;100.00000000000000001&quot;",
            "(Text)",
            "Stufe: 1",
            "Variante: Verstärkt",
            "Bedingung: Während der Fähigkeit",
            "Patchstand: Unbekannt",
        ] {
            assert!(rendered.public_html.contains(text), "{text}");
        }
        let internal: serde_json::Value = serde_json::from_str(&rendered.brain_document).unwrap();
        for (index, fact) in profile.facts.iter().enumerate() {
            assert_eq!(internal["facts"][index]["value"], fact.value);
            assert_eq!(internal["facts"][index]["license"], fact.provenance.license);
            assert_eq!(
                internal["facts"][index]["policy"],
                json!(fact.provenance.origin.policy)
            );
        }
        assert_eq!(serde_json::to_value(&profile).unwrap(), original);
        let root = tempfile::tempdir().unwrap();
        let first = write_public_html(root.path(), &profile).unwrap();
        let second = write_public_html(root.path(), &profile).unwrap();
        assert_eq!(first, second);
        assert_eq!(fs::read_to_string(second).unwrap(), rendered.public_html);
    }

    #[test]
    fn restricted_nested_labels_stay_out_of_facts_context_and_conflicts() {
        for restriction in 0..3 {
            let mut profile = fixture(EntityKind::Hero);
            let fact = &mut profile.facts[0];
            fact.predicate = "GesperrteBezeichnung".into();
            fact.value = json!(998877);
            fact.qualifiers
                .insert("semantic_scope".into(), json!("Variants/1"));
            match restriction {
                0 => fact.provenance.origin.policy.publication_allowed = false,
                1 => fact.provenance.origin.policy.visibility = SourceVisibility::Private,
                _ => {
                    fact.provenance
                        .origin
                        .policy
                        .allowed_scopes
                        .insert("intern".into());
                }
            }
            profile.conflicts.push(ProfileConflict {
                predicate: fact.predicate.clone(),
                preferred_fact_id: Some(fact_reference(fact)),
                fact_ids: vec![fact_reference(fact)],
                reason: "Widerspruch".into(),
            });
            profile.context.push(fact.clone());
            let html = render_entity_profile(&profile).unwrap().public_html;
            for forbidden in ["GesperrteBezeichnung", "998877", "Variants/1"] {
                assert!(!html.contains(forbidden));
            }
            assert!(html.contains("nur intern verfügbar"));
        }
    }

    #[test]
    fn conditions_and_values_are_published_or_withheld_together() {
        for condition in [
            json!("health < 50.0%"),
            json!("/private/health.json"),
            json!({"private": true}),
            json!("file.json_value"),
        ] {
            let mut profile = fixture(EntityKind::Hero);
            let fact = &mut profile.facts[0];
            fact.value = json!(998877);
            fact.qualifiers
                .insert("condition".into(), condition.clone());
            profile.conflicts.push(ProfileConflict {
                predicate: fact.predicate.clone(),
                preferred_fact_id: Some(fact_reference(fact)),
                fact_ids: vec![fact_reference(fact)],
                reason: "Widerspruch".into(),
            });
            profile.context.push(fact.clone());
            let mut story = story_fixture();
            story.old_value = json!(887766);
            story.new_value = json!(776655);
            story
                .additional_fields
                .insert("condition".into(), condition.clone());
            profile.patch_story.push(story);
            let original = serde_json::to_value(&profile).unwrap();
            let rendered = render_entity_profile(&profile).unwrap();
            if condition == json!("health < 50.0%") {
                for text in [
                    "998877",
                    "887766",
                    "776655",
                    "health &lt; 50.0%",
                    "bevorzugter Beleg",
                ] {
                    assert!(rendered.public_html.contains(text), "{text}");
                }
                assert!(rendered.brain_document.contains("health < 50.0%"));
                assert!(rendered.brain_document.contains("998877"));
            } else {
                for text in ["998877", "887766", "776655", "bevorzugter Beleg"] {
                    assert!(!rendered.public_html.contains(text), "{text}");
                    assert!(!rendered.brain_document.contains(text), "{text}");
                }
                assert!(rendered.public_html.contains("öffentliche Beschreibung"));
                assert!(rendered.brain_document.contains("öffentliche Beschreibung"));
            }
            assert_eq!(serde_json::to_value(&profile).unwrap(), original);
        }
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0].validity = PatchValidity::Known {
            from_patch: "2026-09-16".into(),
            to_patch_exclusive: None,
            through_patch_inclusive: Some("2026-09-30".into()),
            evidence_ref: "fixture-patch".into(),
        };
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(rendered
            .public_html
            .contains("belegt bis einschließlich 2026-09-30"));
        assert!(rendered.brain_document.contains("through_patch_inclusive"));
    }

    #[test]
    fn unlabelled_nested_variants_report_a_gap_without_numbers_or_pointers() {
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0]
            .qualifiers
            .insert("semantic_scope".into(), json!("Variants/0"));
        let mut second = profile.facts[0].clone();
        second.value = json!(123456);
        second
            .qualifiers
            .insert("semantic_scope".into(), json!("Variants/1"));
        profile.facts.push(second);
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(rendered
            .public_html
            .contains("fehlt eine belegte öffentliche Beschreibung"));
        for forbidden in ["<code>700</code>", "123456", "Variants/0", "Variants/1"] {
            assert!(!rendered.public_html.contains(forbidden));
        }
        assert!(!rendered.brain_document.contains("Variants/0"));
        assert!(!rendered.brain_document.contains("123456"));
        assert!(rendered.brain_document.contains("öffentliche Beschreibung"));
    }

    #[test]
    fn wiki_numbers_and_text_remain_internal_even_with_public_raw_rights() {
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0].provenance.source_kind = ProfileSourceKind::Wiki;
        profile.facts[0].value = json!(987654321);
        let mut context = profile.facts[0].clone();
        context.value = json!("Wörtlicher Wiki-Text");
        profile.context.push(context);
        let rendered = render_entity_profile(&profile).unwrap();
        for text in ["987654321", "Wörtlicher Wiki-Text"] {
            assert!(rendered.brain_document.contains(text));
            assert!(!rendered.public_html.contains(text));
        }
    }

    #[test]
    fn public_output_omits_raw_paths_and_pointers_from_all_output_fields() {
        for raw in [
            "/home/operator/raw/hero.json",
            "scripts/heroes/abrams.vdata",
            "game_file:hero",
            "file.json_value",
            "C:\\raw\\hero.json",
        ] {
            let mut profile = fixture(EntityKind::Hero);
            profile.patch = Some(raw.into());
            profile.entity.name = raw.into();
            profile.source_state = vec![raw.into()];
            profile.unknowns = vec![raw.into()];
            let fact = &mut profile.facts[0];
            fact.fact_id = raw.into();
            fact.subject = raw.into();
            fact.unit = Some(raw.into());
            fact.evidence_status = raw.into();
            fact.qualifiers = serde_json::from_value(json!({
                "source_pointer":raw,"source_lexeme":raw,"condition":raw,"variant":raw,
                "ability_name":raw,"other":raw,"level":raw
            }))
            .unwrap();
            fact.validity = PatchValidity::Known {
                from_patch: raw.into(),
                to_patch_exclusive: Some(raw.into()),
                through_patch_inclusive: None,
                evidence_ref: raw.into(),
            };
            fact.provenance.origin.locator = raw.into();
            fact.provenance.origin.identity.logical_id = raw.into();
            fact.provenance.origin.identity.source_id = raw.into();
            fact.provenance.origin.parser_revision = raw.into();
            fact.provenance.source_span = Some(raw.into());
            fact.provenance.original_revision = raw.into();
            fact.provenance.observed_at = raw.into();
            fact.provenance
                .document_metadata
                .insert("raw".into(), json!(raw));
            profile.conflicts.push(ProfileConflict {
                predicate: raw.into(),
                preferred_fact_id: Some(fact_reference(fact)),
                fact_ids: vec![fact_reference(fact)],
                reason: raw.into(),
            });
            let mut story = story_fixture();
            story.patch_title = Some(raw.into());
            story.entity_name = Some(raw.into());
            story.ability_name = Some(raw.into());
            story.stat_name = Some(raw.into());
            story.provenance.source_url = Some(raw.into());
            story.provenance.evidence_ref = raw.into();
            profile.patch_story.push(story);
            let original = serde_json::to_value(&profile).unwrap();
            let rendered = render_entity_profile(&profile).unwrap();
            assert!(!rendered.public_html.contains(raw), "{raw}");
            assert!(!rendered.public_html.contains(&escape(raw)), "{raw}");
            assert!(!rendered.public_html.contains("<code>700</code>"));
            assert!(!rendered.public_html.contains("bevorzugter Beleg"));
            assert!(rendered.public_html.contains("16.09.2026"));
            assert!(
                rendered.brain_document.contains(&escape(raw))
                    || rendered
                        .brain_document
                        .contains(&serde_json::to_string(raw).unwrap())
            );
            assert_eq!(serde_json::to_value(&profile).unwrap(), original);
            profile.facts[0].predicate = raw.into();
            assert!(!render_entity_profile(&profile)
                .unwrap()
                .public_html
                .contains("<code>700</code>"));
            profile.facts[0].validity = PatchValidity::Unknown { reason: raw.into() };
            assert!(!render_entity_profile(&profile)
                .unwrap()
                .public_html
                .contains(&escape(raw)));
        }
    }

    #[test]
    #[ignore = "Externe Datenprobe: benötigt bereinigte Profile aus der getrennt ausgeführten eingefrorenen Git-/DB-Probe"]
    fn actual_frozen_profiles_keep_numbers_in_compact_document_and_html() {
        let profiles: Vec<EntityProfile> =
            serde_json::from_slice(&fs::read("/tmp/brain-a3-f1-real-profiles.json").unwrap())
                .unwrap();
        for (entity_key, predicate, expected) in [
            ("hero_inferno", "max_health", serde_json::json!("830.0")),
            (
                "upgrade_clip_size",
                "bonus_clip_size_percent",
                serde_json::json!(30),
            ),
        ] {
            let profile = profiles
                .iter()
                .find(|profile| profile.entity.entity_key == entity_key)
                .unwrap();
            let fact = profile
                .facts
                .iter()
                .find(|fact| fact.predicate == predicate)
                .unwrap();
            assert_eq!(fact.value, expected);
            let rendered = render_entity_profile(profile).unwrap();
            assert_eq!(
                rendered.brain_document,
                brain_storage::entity_profile::compact::compact_document(profile).unwrap()
            );
            assert!(rendered.public_html.contains(predicate));
            assert!(rendered
                .public_html
                .contains(&format!("<code>{}</code>", escape(&fact.value.to_string()))));
            assert!(!rendered.public_html.contains("semantic_scope"));
            assert!(!rendered.public_html.contains("source_pointer"));
            println!(
                "Bereinigtes Dokument und HTML: {} {}={}; ausdrücklich eingefrorener Originalwert",
                profile.entity.name, predicate, fact.value
            );
        }
    }

    #[test]
    fn uninterpreted_leaves_and_unmarked_numeric_strings_remain_internal() {
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0]
            .qualifiers
            .insert("gameplay_binding".into(), json!("uninterpreted"));
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("<code>700</code>"));
        profile.facts[0].qualifiers.clear();
        profile.facts[0].value = json!("700");
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("&quot;700&quot;"));
    }

    #[test]
    fn public_patch_story_uses_named_numbers_dates_and_provenance() {
        let mut profile = fixture(EntityKind::Hero);
        profile.patch_story.push(story_fixture());
        let rendered = render_entity_profile(&profile).unwrap();
        for expected in [
            "16.09.2026",
            "September &lt;script&gt;",
            "Held: Abrams",
            "Siphon Life",
            "Schaden",
            "von 100.00000000000000001 auf 123 geändert",
            "Verstärkung",
            "gestiegen",
            "Konfidenz: 0.8",
            "Quelle: Patchhistorie",
        ] {
            assert!(rendered.public_html.contains(expected), "{expected}");
        }
        for internal in ["Gesperrter Wiki-Patchtext", "Interner Zusatz"] {
            assert!(serde_json::to_string(&profile).unwrap().contains(internal));
            assert!(!rendered.brain_document.contains(internal));
            assert!(!rendered.public_html.contains(internal));
        }
        let change = &mut profile.patch_story[0];
        change.old_value = json!({"raw":"Beliebiger Originaltext"});
        change.new_value = json!("Beliebiger neuer Text");
        change.confidence = json!(["Interne Konfidenz"]);
        change.original_line.redistribution_allowed = true;
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(rendered.public_html.contains("Zahlenwerte sind unbekannt"));
        for internal in [
            "Beliebiger Originaltext",
            "Beliebiger neuer Text",
            "Interne Konfidenz",
            "Gesperrter Wiki-Patchtext",
        ] {
            assert!(serde_json::to_string(&profile).unwrap().contains(internal));
            assert!(!rendered.public_html.contains(internal));
        }
    }

    #[test]
    fn compact_document_preserves_facts_qualifiers_conflicts_and_source_rights() {
        let mut profile = fixture(EntityKind::Hero);
        profile.facts[0]
            .provenance
            .document_metadata
            .insert("original_document".into(), json!("x".repeat(4096)));
        let mut other = profile.facts[0].clone();
        other.fact_id = "health-2".into();
        other.value = json!(650);
        other.qualifiers.insert("level".into(), json!(2));
        profile.context.push(other.clone());
        profile.facts.push(other);
        profile.conflicts.push(ProfileConflict {
            predicate: "Lebenspunkte".into(),
            preferred_fact_id: Some(fact_reference(&profile.facts[0])),
            fact_ids: profile.facts.iter().map(fact_reference).collect(),
            reason: "Abweichende Werte".into(),
        });
        let original = serde_json::to_string(&profile).unwrap();
        let rendered = render_entity_profile(&profile).unwrap();
        let document: serde_json::Value = serde_json::from_str(&rendered.brain_document).unwrap();
        for (section, facts) in [("facts", &profile.facts), ("context", &profile.context)] {
            assert_eq!(document[section].as_array().unwrap().len(), facts.len());
            for (index, fact) in facts.iter().enumerate() {
                for (field, expected) in [
                    ("value", fact.value.clone()),
                    ("qualifiers", json!(fact.qualifiers)),
                    ("unit", json!(fact.unit)),
                    ("validity", json!(fact.validity)),
                    ("policy", json!(fact.provenance.origin.policy)),
                    ("license", fact.provenance.license.clone()),
                ] {
                    assert_eq!(document[section][index][field], expected);
                }
                assert_eq!(document[section][index]["source_ref"], 0);
            }
        }
        assert_eq!(document["sources"].as_array().unwrap().len(), 1);
        assert_eq!(document["sources"][0]["original_revision"], "a".repeat(40));
        assert_eq!(document["conflicts"], json!(profile.conflicts));
        assert_eq!(document["source_state"], json!(profile.source_state));
        assert_eq!(document["unknowns"], json!(profile.unknowns));
        assert!(!rendered.brain_document.contains("original_document"));
        assert!(rendered.brain_document.len() < original.len());
        assert_eq!(serde_json::to_string(&profile).unwrap(), original);
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
    fn html_escapes_input_and_file_output_updates_existing_pages() {
        let mut profile = fixture(EntityKind::Item);
        let root = tempfile::tempdir().unwrap();
        let rendered = render_entity_profile(&profile).unwrap();
        assert!(!rendered.public_html.contains("<script>"));
        assert!(rendered.public_html.contains("&lt;script&gt;"));
        let target = write_public_html(root.path(), &profile).unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), rendered.public_html);
        assert!(target.starts_with(root.path()));
        profile.facts[0].value = json!(750);
        assert_eq!(write_public_html(root.path(), &profile).unwrap(), target);
        assert_eq!(
            fs::read_to_string(target).unwrap(),
            render_entity_profile(&profile).unwrap().public_html
        );
    }

    #[test]
    fn failed_writes_and_syncs_preserve_complete_pages_and_allow_retry() {
        for existing_page in [false, true] {
            for sync_failure in [false, true] {
                let mut profile = fixture(EntityKind::Hero);
                let root = tempfile::tempdir().unwrap();
                let target = root.path().join(public_relative_path(&profile).unwrap());
                fs::create_dir_all(target.parent().unwrap()).unwrap();
                let previous = render_entity_profile(&profile).unwrap().public_html;
                if existing_page {
                    write_public_html(root.path(), &profile).unwrap();
                }
                profile.facts[0].value = json!(750);
                let next = render_entity_profile(&profile).unwrap().public_html;
                assert!(publish_html(&target, |file| {
                    file.write_all(if sync_failure {
                        next.as_bytes()
                    } else {
                        b"<!doctype html><html>"
                    })?;
                    Err(std::io::Error::other(if sync_failure {
                        "Simulierter Synchronisierungsfehler"
                    } else {
                        "Simulierter Schreibfehler"
                    }))
                })
                .is_err());
                if existing_page {
                    assert_eq!(fs::read_to_string(&target).unwrap(), previous);
                } else {
                    assert!(!target.exists());
                }
                assert_eq!(
                    fs::read_dir(target.parent().unwrap()).unwrap().count(),
                    usize::from(existing_page)
                );
                assert_eq!(write_public_html(root.path(), &profile).unwrap(), target);
                assert_eq!(fs::read_to_string(&target).unwrap(), next);
            }
        }
    }

    #[test]
    fn scoped_private_and_non_numeric_facts_are_not_published() {
        let mut profile = fixture(EntityKind::Ability);
        profile.facts[0]
            .provenance
            .origin
            .policy
            .allowed_scopes
            .insert("private".into());
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("<code>700</code>"));
        profile.facts[0]
            .provenance
            .origin
            .policy
            .allowed_scopes
            .clear();
        profile.facts[0].provenance.origin.policy.license =
            Observed::unknown(brain_contracts::value::UnknownReason::NotPresent);
        profile.facts[0].provenance.origin.policy.visibility = SourceVisibility::Private;
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("<code>700</code>"));
        profile.facts[0].provenance.origin.policy.visibility = SourceVisibility::Public;
        profile.facts[0].value = json!("Nur mit Grant");
        assert!(!render_entity_profile(&profile)
            .unwrap()
            .public_html
            .contains("Nur mit Grant"));
    }
}
