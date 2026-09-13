use std::collections::BTreeSet;
use std::fmt;

use crate::{AuthorBuild, BacktestMetrics, BuildObject, HeroBacktest};

impl fmt::Display for crate::BacktestReport {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        for hero in &self.heroes {
            for (author, metrics) in hero
                .per_author
                .iter()
                .map(|(author, metrics)| (author.as_str(), metrics))
                .chain(std::iter::once(("Gesamt", &hero.aggregate)))
            {
                write!(
                    output,
                    "{} | {author} | Kern: {:.4} | Reihenfolge: ",
                    hero.hero_name, metrics.core_coverage
                )?;
                match metrics.order_proximity {
                    Some(value) => write!(output, "{value:.4}")?,
                    None => output.write_str("nicht messbar")?,
                }
                writeln!(
                    output,
                    " | Recall: {:.4} | Jaccard: {:.4}",
                    metrics.reference_recall, metrics.core_jaccard
                )?;
            }
        }
        Ok(())
    }
}

fn unique(values: impl Iterator<Item = i64>) -> BTreeSet<i64> {
    values.filter(|value| *value != 0).collect()
}

fn build_order(build: &BuildObject) -> Vec<i64> {
    build.core.iter().map(|item| item.item_id).collect()
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
    let order_proximity = if common.len() < 2 || reasoner_order.len() < 2 || author_order.len() < 2
    {
        None
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
        Some(distance.min(1.0))
    };
    BacktestMetrics {
        core_coverage: coverage,
        reference_recall: if author_core.is_empty() {
            0.0
        } else {
            reasoner_core.intersection(&author_core).count() as f64 / author_core.len() as f64
        },
        core_jaccard: core_jaccard(build, author),
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
            reference_recall: 0.0,
            core_jaccard: 0.0,
            order_proximity: None,
            switch_detected: None,
        }
    } else {
        BacktestMetrics {
            reference_recall: per_author
                .iter()
                .map(|(_, metrics)| metrics.reference_recall)
                .sum::<f64>()
                / per_author.len() as f64,
            core_jaccard: per_author
                .iter()
                .map(|(_, metrics)| metrics.core_jaccard)
                .sum::<f64>()
                / per_author.len() as f64,
            core_coverage: per_author
                .iter()
                .map(|(_, metrics)| metrics.core_coverage)
                .sum::<f64>()
                / per_author.len() as f64,
            order_proximity: {
                let values = per_author
                    .iter()
                    .filter_map(|(_, metrics)| metrics.order_proximity);
                let count = values.clone().count();
                (count > 0).then(|| values.sum::<f64>() / count as f64)
            },
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
    fn reference_metrics_use_unique_reference_core_and_survive_serialization() {
        let report = backtest_hero_with_build(
            25,
            "Warden",
            &build(&[1, 1, 2, 0]),
            &[author(&[2, 2, 3, 4, 0], &[]), author(&[], &[])],
        );
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(json["per_author"][0][1]["reference_recall"], 1.0 / 3.0);
        assert_eq!(json["per_author"][0][1]["core_jaccard"], 0.25);
        assert_eq!(json["aggregate"]["reference_recall"], 1.0 / 6.0);
        assert_eq!(json["aggregate"]["core_jaccard"], 0.125);
        assert_eq!(crate::persistence_json(&report).unwrap(), json);
        let no_authors =
            serde_json::to_value(backtest_hero_with_build(25, "Warden", &build(&[]), &[])).unwrap();
        assert_eq!(no_authors["aggregate"]["reference_recall"], 0.0);
        assert_eq!(no_authors["aggregate"]["core_jaccard"], 0.0);
        let empty = serde_json::to_value(backtest_metrics(&build(&[]), &author(&[], &[]))).unwrap();
        assert_eq!(empty["reference_recall"], 0.0);
        assert_eq!(empty["core_jaccard"], 1.0);
        let text = crate::BacktestReport {
            heroes: vec![report],
        }
        .to_string();
        assert!(text.contains("Recall: 0.3333 | Jaccard: 0.2500"));
    }

    #[test]
    fn measures_warden_seed_core_and_order() {
        let metrics = backtest_metrics(&build(&[1, 2, 3]), &author(&[2, 3, 4], &[1, 2, 3]));
        assert!((metrics.core_coverage - 2.0 / 3.0).abs() < 0.0001);
        assert_eq!(metrics.order_proximity, Some(0.0));
        assert!(
            (core_jaccard(&build(&[1, 2, 3]), &author(&[2, 3, 4], &[1, 2, 3])) - 0.5).abs()
                < 0.0001
        );
    }

    #[test]
    fn unmeasurable_orders_are_null_and_excluded_from_aggregate() {
        for (reasoner, order) in [
            (vec![], vec![1, 2]),
            (vec![1, 2], vec![]),
            (vec![1], vec![1, 2]),
            (vec![1, 2], vec![1]),
            (vec![1, 2], vec![3, 4]),
        ] {
            let metrics = backtest_metrics(&build(&reasoner), &author(&order, &order));
            assert!(serde_json::to_value(metrics).unwrap()["order_proximity"].is_null());
        }
        let build = build(&[1, 2]);
        let report = backtest_hero_with_build(
            25,
            "Warden",
            &build,
            &[
                author(&[1, 2], &[1, 2]),
                author(&[1, 2], &[2, 1]),
                author(&[3, 4], &[3, 4]),
            ],
        );
        assert_eq!(
            serde_json::to_value(report.aggregate).unwrap()["order_proximity"],
            0.5
        );
        for authors in [vec![], vec![author(&[], &[])]] {
            let report = backtest_hero_with_build(25, "Warden", &build, &authors);
            assert!(serde_json::to_value(report.aggregate).unwrap()["order_proximity"].is_null());
        }
    }

    #[test]
    fn report_labels_missing_measurements_and_preserves_measured_values() {
        let report = crate::BacktestReport {
            heroes: vec![backtest_hero_with_build(
                25,
                "Warden",
                &build(&[1, 2]),
                &[author(&[1, 2], &[1, 2]), author(&[], &[])],
            )],
        };
        let text = report.to_string();
        assert!(text.contains("Reihenfolge: nicht messbar"));
        assert!(text.contains("Warden | Gesamt | Kern: 0.5000 | Reihenfolge: 0.0000"));
        let restored: crate::BacktestReport =
            serde_json::from_value(serde_json::to_value(&report).unwrap()).unwrap();
        assert_eq!(restored, report);
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
