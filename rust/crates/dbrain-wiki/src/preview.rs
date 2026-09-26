use crate::{model::*, sha256, Result};
use std::collections::{BTreeMap, BTreeSet};

/// Reference-only layout preview. Does not produce a HeroKnowledgeCard or copy
/// stats, interpret game rules, assign a patch, or invent a knowledge release.
pub(crate) fn project(
    bindings: &[HeroBinding],
    pages: &BTreeMap<i64, PageProbe>,
) -> Result<Vec<CardPreview>> {
    let mut previews = Vec::new();
    let mut seen = BTreeSet::new();
    let mut reference_budget = 50_000usize;
    for binding in bindings {
        if binding.locale.is_empty() || binding.locale.len() > 32 {
            return Err("invalid preview locale".into());
        }
        if !seen.insert((binding.hero_page_id, binding.locale.clone())) {
            return Err("duplicate hero/locale preview binding".into());
        }
        let hero = pages
            .get(&binding.hero_page_id)
            .ok_or("hero binding references undiscovered page")?;
        if hero.kind != PageKind::Hero {
            return Err("hero binding points to a non-hero page".into());
        }
        let mut unknowns: BTreeSet<String> = [
            "canonical_fact_rule_card_contract_missing",
            "patch_unknown",
            "mode_unknown",
            "canonical_knowledge_release_unknown",
            "build_legality_not_evaluated",
            "strategy_and_empirical_evidence_not_validated",
            "source_syntax_is_not_fact_validation",
            "localized_text_provenance_not_validated",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        let mut sections = BTreeMap::new();
        let mut all_dependencies = BTreeSet::new();
        if binding.ability_page_ids.len()
            + binding.mechanic_page_ids.len()
            + binding.item_page_ids.len()
            + binding.rule_page_ids.len()
            + binding.alias_page_ids.len()
            > 256
        {
            return Err("preview binding budget exceeded".into());
        }
        for (name, ids, expected_kind) in [
            ("profile", vec![binding.hero_page_id], PageKind::Hero),
            (
                "abilities",
                binding.ability_page_ids.clone(),
                PageKind::Ability,
            ),
            (
                "mechanics",
                binding.mechanic_page_ids.clone(),
                PageKind::Mechanic,
            ),
            ("items", binding.item_page_ids.clone(), PageKind::Item),
            ("build_rules", binding.rule_page_ids.clone(), PageKind::Rule),
            (
                "aliases",
                binding.alias_page_ids.clone(),
                PageKind::Redirect,
            ),
        ] {
            let mut refs = Vec::new();
            if ids.is_empty() {
                unknowns.insert(format!("{name}:no_bound_sources"));
            }
            for id in ids.into_iter().collect::<BTreeSet<_>>() {
                let Some(page) = pages.get(&id) else {
                    unknowns.insert(format!("{name}:unknown_page:{id}"));
                    continue;
                };
                if page.kind != expected_kind {
                    unknowns.insert(format!("{name}:wrong_page_kind:{id}"));
                    continue;
                }
                if expected_kind == PageKind::Redirect
                    && page.redirect_target != Some(binding.hero_page_id)
                {
                    unknowns.insert(format!("{name}:unverified_redirect:{id}"));
                    continue;
                }
                match safe_closure(id, pages) {
                    Some(closure) => {
                        if let Some(reference) = source_ref(page) {
                            spend_budget(&reference, &mut reference_budget)?;
                            refs.push(reference);
                            all_dependencies.extend(closure);
                        }
                    }
                    None => {
                        unknowns
                            .insert(format!("{name}:source_or_dependency_not_previewable:{id}"));
                    }
                }
            }
            sections.insert(name.to_string(), refs);
        }
        let mut dependencies = Vec::new();
        for reference in all_dependencies
            .iter()
            .filter_map(|id| pages.get(id).and_then(source_ref))
        {
            spend_budget(&reference, &mut reference_budget)?;
            dependencies.push(reference);
        }
        sections.insert("dependency_evidence".into(), dependencies);
        let mut preview = CardPreview {
            hero_source_id: hero.source_id.clone(),
            locale: binding.locale.clone(),
            patch: None,
            mode: None,
            canonical_knowledge_release: None,
            sections,
            unknowns,
            publishable: false,
            preview_sha256: String::new(),
        };
        preview.preview_sha256 =
            sha256(&serde_json::to_vec(&preview).map_err(|_| "preview serialization failed")?);
        previews.push(preview);
    }
    previews.sort_by(|a, b| (&a.hero_source_id, &a.locale).cmp(&(&b.hero_source_id, &b.locale)));
    Ok(previews)
}

pub(crate) fn safe_closure<P: std::borrow::Borrow<PageProbe>>(
    start: i64,
    pages: &BTreeMap<i64, P>,
) -> Option<BTreeSet<i64>> {
    let mut visited = BTreeSet::new();
    let mut pending = vec![start];
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        let page: &PageProbe = std::borrow::Borrow::borrow(pages.get(&id)?);
        if !page.previewable() || !page.dependencies_complete {
            return None;
        }
        pending.extend(&page.dependency_ids);
    }
    Some(visited)
}

fn source_ref(page: &PageProbe) -> Option<SourceRef> {
    let revision = page.latest()?;
    Some(SourceRef {
        source_id: page.source_id.clone(),
        revision_id: revision.revision_id,
        raw_sha256: revision.raw_sha256.clone()?,
        candidate_locators: revision
            .candidates
            .iter()
            .map(|c| c.locator.clone())
            .collect(),
    })
}

fn spend_budget(reference: &SourceRef, budget: &mut usize) -> Result<()> {
    *budget = budget
        .checked_sub(reference.candidate_locators.len().max(1))
        .ok_or("preview reference budget exceeded")?;
    Ok(())
}
