use std::collections::BTreeSet;

use crate::{
    AuthorBuild, BacktestMetrics, BuildObject, HeroBacktest, ReasonerCtx, ReasonerError, Result,
};

fn unique(values: impl Iterator<Item = i64>) -> BTreeSet<i64> {
    values.filter(|value| *value != 0).collect()
}

fn build_order(build: &BuildObject) -> Vec<i64> {
    build
        .core
        .iter()
        .chain(build.situations.iter().flat_map(|block| block.items.iter()))
        .map(|item| item.item_id)
        .collect()
}

pub fn core_jaccard(build: &BuildObject, author: &AuthorBuild) -> f64 {
    let reasoner = unique(build.core.iter().map(|item| item.item_id));
    let author = unique(author.core_item_ids.iter().copied());
    if reasoner.is_empty() && author.is_empty() {
        return 1.0;
    }
    let intersection = reasoner.intersection(&author).count() as f64;
    let union = reasoner.union(&author).count() as f64;
    if union > 0.0 {
        intersection / union
    } else {
        0.0
    }
}

pub fn backtest_metrics(build: &BuildObject, author: &AuthorBuild) -> BacktestMetrics {
    let reasoner_core = unique(build.core.iter().map(|item| item.item_id));
    let author_core = unique(author.core_item_ids.iter().copied());
    let coverage = if reasoner_core.is_empty() {
        0.0
    } else {
        reasoner_core.intersection(&author_core).count() as f64 / reasoner_core.len() as f64
    };
    let reasoner_order = build_order(build);
    let author_order = &author.buy_order;
    let common = reasoner_order
        .iter()
        .filter(|id| author_order.contains(id))
        .collect::<Vec<_>>();
    let order_proximity = if common.is_empty() || reasoner_order.len() < 2 || author_order.len() < 2
    {
        1.0
    } else {
        let denominator = (common.len() as f64).max(1.0);
        let distance = common
            .iter()
            .map(|id| {
                let left = reasoner_order
                    .iter()
                    .position(|value| value == *id)
                    .unwrap() as f64
                    / (reasoner_order.len() - 1) as f64;
                let right = author_order
                    .iter()
                    .position(|value| *value == **id)
                    .unwrap() as f64
                    / (author_order.len() - 1) as f64;
                (left - right).abs()
            })
            .sum::<f64>()
            / denominator;
        distance.min(1.0)
    };
    BacktestMetrics {
        core_coverage: coverage,
        order_proximity,
        switch_detected: None,
    }
}

pub fn detect_patch_switch(
    reasoner_before: &BuildObject,
    reasoner_after: &BuildObject,
    author_before: &AuthorBuild,
    author_after: &AuthorBuild,
) -> bool {
    let reasoner_before = unique(reasoner_before.core.iter().map(|item| item.item_id));
    let reasoner_after = unique(reasoner_after.core.iter().map(|item| item.item_id));
    let author_before = unique(author_before.core_item_ids.iter().copied());
    let author_after = unique(author_after.core_item_ids.iter().copied());
    let reasoner_added = reasoner_after
        .difference(&reasoner_before)
        .copied()
        .collect::<BTreeSet<_>>();
    let author_added = author_after
        .difference(&author_before)
        .copied()
        .collect::<BTreeSet<_>>();
    let reasoner_removed = reasoner_before
        .difference(&reasoner_after)
        .copied()
        .collect::<BTreeSet<_>>();
    let author_removed = author_before
        .difference(&author_after)
        .copied()
        .collect::<BTreeSet<_>>();
    (!reasoner_added.is_empty() || !reasoner_removed.is_empty())
        && (!author_added.is_empty() || !author_removed.is_empty())
        && (!reasoner_added.is_disjoint(&author_added)
            || !reasoner_removed.is_disjoint(&author_removed))
}

pub fn backtest_hero_with_build(
    hero_id: i64,
    hero_name: &str,
    build: &BuildObject,
    authors: &[AuthorBuild],
) -> HeroBacktest {
    let mut per_author = authors
        .iter()
        .map(|author| (author.author.clone(), backtest_metrics(build, author)))
        .collect::<Vec<_>>();
    per_author.sort_by(|left, right| left.0.cmp(&right.0));
    let aggregate = if per_author.is_empty() {
        BacktestMetrics {
            core_coverage: 0.0,
            order_proximity: 1.0,
            switch_detected: None,
        }
    } else {
        BacktestMetrics {
            core_coverage: per_author
                .iter()
                .map(|(_, metrics)| metrics.core_coverage)
                .sum::<f64>()
                / per_author.len() as f64,
            order_proximity: per_author
                .iter()
                .map(|(_, metrics)| metrics.order_proximity)
                .sum::<f64>()
                / per_author.len() as f64,
            switch_detected: None,
        }
    };
    HeroBacktest {
        hero_id,
        hero_name: hero_name.to_string(),
        per_author,
        aggregate,
    }
}

pub async fn backtest_hero(ctx: &ReasonerCtx, hero: &str) -> Result<HeroBacktest> {
    let hero_model = crate::load_hero_model(ctx, hero).await?;
    let authors = crate::load_author_builds(ctx, hero_model.hero_id).await?;
    if authors.is_empty() {
        let empty = BuildObject {
            hero_id: hero_model.hero_id,
            hero_name: hero_model.name.clone(),
            patch_tag: ctx.config.patch_tag.clone(),
            name: "empty".to_string(),
            core: Vec::new(),
            situations: Vec::new(),
            ability_order: Vec::new(),
            confidence: crate::Confidence::Low,
            rationale: String::new(),
        };
        return Ok(backtest_hero_with_build(
            hero_model.hero_id,
            &hero_model.name,
            &empty,
            &[],
        ));
    }
    Err(ReasonerError::Data(
        "Backtest benötigt den von Paket B erzeugten ScoredItem-Bestand".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BuildItem, BuyPhase, Confidence, Evidence, EvidenceKind, SituationBlock, SituationKind,
    };

    fn build(ids: &[i64]) -> BuildObject {
        BuildObject {
            hero_id: 25,
            hero_name: "Warden".to_string(),
            patch_tag: "current".to_string(),
            name: "test".to_string(),
            core: ids
                .iter()
                .map(|id| BuildItem {
                    item_id: *id,
                    name: id.to_string(),
                    tier: 1,
                    buy_phase: BuyPhase::Core,
                    why: String::new(),
                    confidence: Confidence::High,
                    imbue_target: None,
                    sell_priority: None,
                    sources: vec![Evidence {
                        kind: EvidenceKind::Mechanic,
                        detail: String::new(),
                    }],
                })
                .collect(),
            situations: vec![SituationBlock {
                label: "Optional".to_string(),
                optional: true,
                kind: SituationKind::Optional,
                items: Vec::new(),
            }],
            ability_order: Vec::new(),
            confidence: Confidence::High,
            rationale: String::new(),
        }
    }

    fn author(core: &[i64], order: &[i64]) -> AuthorBuild {
        AuthorBuild {
            author: "A".to_string(),
            version: 1,
            published_at: None,
            last_updated_at: None,
            patch_tag: None,
            core_item_ids: core.to_vec(),
            buy_order: order.to_vec(),
        }
    }

    #[test]
    fn measures_warden_seed_core_and_order() {
        let metrics = backtest_metrics(&build(&[1, 2, 3]), &author(&[2, 3, 4], &[1, 2, 3]));
        assert!((metrics.core_coverage - 2.0 / 3.0).abs() < 0.0001);
        assert_eq!(metrics.order_proximity, 0.0);
        assert!(
            (core_jaccard(&build(&[1, 2, 3]), &author(&[2, 3, 4], &[1, 2, 3])) - 0.5).abs()
                < 0.0001
        );
    }

    #[test]
    fn detects_shared_patch_direction() {
        assert!(detect_patch_switch(
            &build(&[1, 2]),
            &build(&[2, 3]),
            &author(&[1, 2], &[1, 2]),
            &author(&[2, 3], &[2, 3])
        ));
    }
}
