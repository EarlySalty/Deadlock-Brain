use super::artifacts::Artifacts;
use anyhow::{ensure, Result};
use brain_contracts::entity_profile::EntityProfile;
use std::{
    collections::BTreeSet,
    path::{Component, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedEntityProfile {
    pub entity_key: String,
    pub profile_sha256: String,
    pub brain_document_ref: String,
    pub public_html_ref: String,
    pub public_relative_path: PathBuf,
}

pub fn stage_profiles<F>(
    artifacts: &Artifacts,
    profiles: &[EntityProfile],
    mut render: F,
) -> Result<Vec<PreparedEntityProfile>>
where
    F: FnMut(&EntityProfile) -> Result<(String, String, PathBuf)>,
{
    let mut identities = BTreeSet::new();
    let mut prepared = Vec::new();
    for profile in profiles {
        ensure!(
            identities.insert(&profile.entity.entity_key),
            "Entität ist im Wartungslauf mehrfach vorhanden"
        );
        let profile_sha256 = crate::digest(&serde_json::to_vec(profile)?);
        let (document, html, path) = render(profile)?;
        ensure!(
            !path.as_os_str().is_empty()
                && path
                    .components()
                    .all(|part| matches!(part, Component::Normal(_))),
            "Steckbriefpfad muss relativ sein"
        );
        ensure!(
            path.starts_with("site/entities")
                && path
                    .extension()
                    .is_some_and(|extension| extension == "html"),
            "Steckbriefpfad liegt außerhalb der vorhandenen Site"
        );
        prepared.push(PreparedEntityProfile {
            entity_key: profile.entity.entity_key.clone(),
            profile_sha256,
            brain_document_ref: artifacts.put(document.as_bytes(), "json")?,
            public_html_ref: artifacts.put(html.as_bytes(), "html")?,
            public_relative_path: path,
        });
    }
    Ok(prepared)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::entity_profile::{EntityIdentity, EntityKind, ENTITY_PROFILE_VERSION};

    #[test]
    fn repeating_profile_preserves_artifacts_and_patch_update_creates_new_outputs() {
        let directory = tempfile::tempdir().unwrap();
        let artifacts = Artifacts::open(&directory.path().join("artifacts")).unwrap();
        let mut profile = EntityProfile {
            contract_version: ENTITY_PROFILE_VERSION.into(),
            entity: EntityIdentity {
                entity_key: "hero/fixture".into(),
                kind: EntityKind::Hero,
                name: "Fixture".into(),
                aliases: Vec::new(),
                identity_evidence: vec!["fixture".into()],
            },
            patch: None,
            source_state: vec!["fixture/revision/1".into()],
            facts: Vec::new(),
            context: Vec::new(),
            conflicts: Vec::new(),
            patch_story: Vec::new(),
            unknowns: vec!["Patchstand unbekannt".into()],
        };
        let render = |profile: &EntityProfile| {
            Ok((
                serde_json::to_string(profile)?,
                format!("<p>{:?}</p>", profile.patch),
                PathBuf::from("site/entities/hero/fixture.html"),
            ))
        };
        let initial = stage_profiles(&artifacts, &[profile.clone()], render).unwrap();
        assert_eq!(
            stage_profiles(&artifacts, &[profile.clone()], render).unwrap(),
            initial
        );
        profile.patch = Some("fixture-patch".into());
        let updated = stage_profiles(&artifacts, &[profile], render).unwrap();
        assert_ne!(updated[0].brain_document_ref, initial[0].brain_document_ref);
        assert_ne!(updated[0].public_html_ref, initial[0].public_html_ref);
        assert!(artifacts.read(&initial[0].brain_document_ref).is_ok());
        assert!(artifacts.read(&updated[0].brain_document_ref).is_ok());
    }
}
