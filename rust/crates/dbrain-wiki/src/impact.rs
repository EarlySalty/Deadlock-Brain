use crate::{model::*, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn signature(page: &PageProbe) -> Value {
    json!({
        "source_id": page.source_id, "namespace": page.namespace, "title": page.title,
        "kind": page.kind, "build_critical": page.build_critical, "stage": page.stage,
        "source_language": page.source_language, "upstream_url": page.upstream_url,
        "head": page.latest_revision, "hash": page.latest().and_then(|r| r.raw_sha256.as_ref()),
        "source_time": page.latest().and_then(|r| r.source_time),
        "dependencies": page.dependency_ids, "unresolved": page.unresolved_dependencies,
        "dependencies_complete": page.dependencies_complete, "redirect": page.redirect_target,
    })
}

/// Pure invalidation proposal. Never schedules embeddings or writes a release.
/// Missing entries are deletions only for two fully reconciled discovery captures.
pub fn compare(before: &Report, after: &Report) -> Result<Impact> {
    if before.report_version != after.report_version {
        return Err("cannot compare incompatible report formats".into());
    }
    if before.source_key != after.source_key {
        return Err("cannot compare different source identities".into());
    }
    if after.retrieved_at < before.retrieved_at {
        return Err("stale capture must not replace a newer capture".into());
    }
    if before.discovery_scope.as_ref().map(|s| &s.selection)
        != after.discovery_scope.as_ref().map(|s| &s.selection)
    {
        return Err("cannot delta different discovery scopes; omission is not deletion".into());
    }
    let old: BTreeMap<_, _> = before.pages.iter().map(|p| (p.page_id, p)).collect();
    let new: BTreeMap<_, _> = after.pages.iter().map(|p| (p.page_id, p)).collect();
    let ids: BTreeSet<_> = old.keys().chain(new.keys()).copied().collect();
    for id in old.keys().filter(|id| new.contains_key(id)) {
        if let (Some(previous), Some(next)) = (old[id].latest_revision, new[id].latest_revision) {
            if next < previous {
                return Err("stale page revision must not replace a newer revision".into());
            }
        }
    }
    // A revision is immutable across captures too, not just within one API
    // response. Revocation/suppression may remove content, but must never rewrite
    // previously visible bytes or their source timestamp/content model.
    for id in old.keys().filter(|id| new.contains_key(id)) {
        let new_revisions: BTreeMap<_, _> = new[id]
            .revisions
            .iter()
            .map(|r| (r.revision_id, r))
            .collect();
        for previous in &old[id].revisions {
            if let Some(next) = new_revisions.get(&previous.revision_id) {
                if previous.raw_sha256.is_some()
                    && next.raw_sha256.is_some()
                    && (previous.raw_sha256 != next.raw_sha256
                        || previous.source_time != next.source_time
                        || previous.content_model != next.content_model)
                {
                    return Err("immutable revision conflicts between captures; quarantine/reconcile required".into());
                }
            }
        }
    }
    let full_reconcile_required = !before.discovery_complete
        || !after.discovery_complete
        || before
            .pages
            .iter()
            .chain(&after.pages)
            .any(|p| !p.dependencies_complete);
    let removed_pages: BTreeSet<_> = if before.discovery_complete && after.discovery_complete {
        old.keys()
            .filter(|id| !new.contains_key(id))
            .copied()
            .collect()
    } else {
        BTreeSet::new()
    };
    let mut changed_pages = BTreeSet::new();
    let mut raw_content_changed = BTreeSet::new();
    let mut reverse = BTreeMap::<i64, BTreeSet<i64>>::new();
    for page in before.pages.iter().chain(&after.pages) {
        for dependency in &page.dependency_ids {
            reverse.entry(*dependency).or_default().insert(page.page_id);
        }
    }
    for id in &ids {
        if old.get(id).map(|p| signature(p)) != new.get(id).map(|p| signature(p)) {
            changed_pages.insert(*id);
        }
        let old_hash = old
            .get(id)
            .and_then(|p| p.latest())
            .and_then(|r| r.raw_sha256.as_ref());
        let new_hash = new
            .get(id)
            .and_then(|p| p.latest())
            .and_then(|r| r.raw_sha256.as_ref());
        if old_hash != new_hash {
            raw_content_changed.insert(*id);
        }
    }
    // A source-wide ACL/license/egress decision is not a raw content change,
    // but cached downstream projections must still be invalidated.
    if before.source_policy != after.source_policy {
        changed_pages.extend(&ids);
    }
    let mut reparse_pages = changed_pages.clone();
    if full_reconcile_required || before.parser_version != after.parser_version {
        reparse_pages.extend(&ids);
    }
    let mut pending: Vec<_> = reparse_pages.iter().copied().collect();
    while let Some(id) = pending.pop() {
        for dependant in reverse.get(&id).into_iter().flatten() {
            if reparse_pages.insert(*dependant) {
                pending.push(*dependant);
            }
        }
    }
    let changed_sources: BTreeSet<_> = before
        .pages
        .iter()
        .chain(&after.pages)
        .filter(|p| reparse_pages.contains(&p.page_id))
        .map(|p| &p.source_id)
        .collect();
    let mut reproject_heroes = BTreeSet::new();
    for card in before.card_previews.iter().chain(&after.card_previews) {
        if changed_sources.contains(&card.hero_source_id)
            || card
                .sections
                .values()
                .flatten()
                .any(|r| changed_sources.contains(&r.source_id))
        {
            reproject_heroes.insert(card.hero_source_id.clone());
        }
    }
    // Binding, locale and exclusion changes can alter a projection without a raw
    // page change. Compare full reference-only previews as well as dependencies.
    for card in before.card_previews.iter().chain(&after.card_previews) {
        let old_card = before
            .card_previews
            .iter()
            .find(|c| c.hero_source_id == card.hero_source_id && c.locale == card.locale);
        let new_card = after
            .card_previews
            .iter()
            .find(|c| c.hero_source_id == card.hero_source_id && c.locale == card.locale);
        if old_card != new_card {
            reproject_heroes.insert(card.hero_source_id.clone());
        }
    }
    Ok(Impact {
        changed_pages,
        removed_pages,
        reparse_pages,
        reproject_heroes,
        raw_content_changed,
        full_reconcile_required,
        embedding_jobs_scheduled: 0,
        publication_performed: false,
    })
}
