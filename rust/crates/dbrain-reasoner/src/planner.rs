use std::collections::{BTreeMap, BTreeSet};

use crate::combat::{evaluate_inventory, evaluate_inventory_refs_fast, InventoryEvaluation};
use crate::inventory::{Inventory, InventoryRules, PurchaseTransition};
use crate::{CoreLayoutStats, HeroModel, ItemModel, ReasonerConfig, ScoredItem};

const BEAM_WIDTH: usize = 4;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PurchaseStep {
    pub transition: PurchaseTransition,
    pub evaluation: InventoryEvaluation,
    pub marginal_value: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PurchasePlan {
    pub steps: Vec<PurchaseStep>,
    pub final_evaluation: InventoryEvaluation,
    pub assumptions: Vec<String>,
}

struct Search<'a> {
    hero: &'a HeroModel,
    cfg: &'a ReasonerConfig,
    catalog: Vec<ItemModel>,
    candidates: &'a [&'a ScoredItem],
    rules: &'a InventoryRules,
    combinations: &'a BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
    cache: BTreeMap<Vec<i64>, InventoryEvaluation>,
}

impl Search<'_> {
    fn supported_value(&self, step: &PurchaseStep, tier: i64) -> f64 {
        let id = step.transition.purchased_id;
        let lifts = step
            .transition
            .after
            .held_ids
            .iter()
            .filter(|other| **other != id)
            .filter_map(|other| self.combinations.get(&(id.min(*other), id.max(*other))))
            .map(|support| support.relative_lift.clamp(-0.2, 0.2))
            .collect::<Vec<_>>();
        let observed = if lifts.is_empty() {
            0.0
        } else {
            lifts.iter().sum::<f64>() / lifts.len() as f64
        };
        let mechanical = purchase_value(step, tier);
        mechanical + mechanical.abs() * observed
    }

    fn evaluate(&mut self, inventory: &Inventory) -> Option<InventoryEvaluation> {
        let key = inventory.held_ids.iter().copied().collect::<Vec<_>>();
        if let Some(evaluation) = self.cache.get(&key) {
            return Some(evaluation.clone());
        }
        let held = inventory
            .held_ids
            .iter()
            .map(|id| self.catalog.iter().find(|item| item.item_id == *id))
            .collect::<Option<Vec<_>>>()?;
        let evaluation = evaluate_inventory_refs_fast(self.hero, &held, self.cfg);
        if !evaluation.score.is_finite() {
            return None;
        }
        self.cache.insert(key, evaluation.clone());
        Some(evaluation)
    }

    fn choices(
        &mut self,
        inventory: &Inventory,
        used: &BTreeSet<i64>,
        tier: i64,
        budget: i64,
        before: f64,
    ) -> Vec<PurchaseStep> {
        let candidates = self
            .candidates
            .iter()
            .copied()
            .filter(|candidate| {
                candidate.item.tier == tier && !used.contains(&candidate.item.item_id)
            })
            .collect::<Vec<_>>();
        let mut choices = Vec::new();
        for candidate in candidates {
            let item = &candidate.item;
            let mut transitions = Vec::new();
            if let Ok(transition) = inventory.preview_purchase(item, &self.catalog, self.rules, &[])
            {
                transitions.push(transition);
            } else {
                for held_id in &inventory.held_ids {
                    if let Ok(transition) =
                        inventory.preview_purchase(item, &self.catalog, self.rules, &[*held_id])
                    {
                        transitions.push(transition);
                    }
                }
            }
            let mut best: Option<PurchaseStep> = None;
            for transition in transitions {
                if transition.after.spent_souls > budget || transition.net_cost <= 0 {
                    continue;
                }
                let Some(evaluation) = self.evaluate(&transition.after) else {
                    continue;
                };
                let marginal_value = evaluation.score - before;
                let step = PurchaseStep {
                    transition,
                    evaluation,
                    marginal_value,
                };
                if best.as_ref().is_none_or(|previous| {
                    self.supported_value(&step, tier) > self.supported_value(previous, tier)
                }) {
                    best = Some(step);
                }
            }
            if let Some(step) = best {
                choices.push(step);
            }
        }
        choices.sort_by(|left, right| {
            self.supported_value(right, tier)
                .total_cmp(&self.supported_value(left, tier))
                .then_with(|| {
                    left.transition
                        .purchased_id
                        .cmp(&right.transition.purchased_id)
                })
        });
        choices
    }
}

fn purchase_value(step: &PurchaseStep, tier: i64) -> f64 {
    if tier <= 2 {
        step.marginal_value / step.transition.net_cost.max(1) as f64 * 800.0
    } else {
        step.marginal_value
    }
}

pub fn plan_purchases(
    hero: &HeroModel,
    catalog: &[ScoredItem],
    candidates: &[&ScoredItem],
    layout: &CoreLayoutStats,
    rules: &InventoryRules,
    combinations: &BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
    cfg: &ReasonerConfig,
) -> PurchasePlan {
    let mut search = Search {
        hero,
        cfg,
        catalog: catalog.iter().map(|item| item.item.clone()).collect(),
        candidates,
        rules,
        combinations,
        cache: BTreeMap::new(),
    };
    let schedule = (1..=5)
        .flat_map(|tier| std::iter::repeat_n(tier, layout.target_for_tier(tier)))
        .collect::<Vec<_>>();
    let mut budget = 0i64;
    let budgets = schedule
        .iter()
        .map(|tier| {
            let maximum = candidates
                .iter()
                .filter(|item| item.item.tier == *tier)
                .map(|item| item.item.cost)
                .max()
                .unwrap_or(0);
            budget = budget.saturating_add(maximum);
            budget
        })
        .collect::<Vec<_>>();
    let mut inventory = Inventory::default();
    let mut used = BTreeSet::new();
    let mut evaluation = evaluate_inventory(hero, &[], cfg);
    let mut steps = Vec::new();
    let mut skipped = 0;
    for (index, tier) in schedule.iter().copied().enumerate() {
        let choices = search.choices(&inventory, &used, tier, budgets[index], evaluation.score);
        let mut beam = choices
            .into_iter()
            .take(BEAM_WIDTH)
            .map(|step| {
                let immediate = search.supported_value(&step, tier);
                let mut horizon = immediate;
                if let Some(next_tier) = schedule.get(index + 1) {
                    let mut next_used = used.clone();
                    next_used.insert(step.transition.purchased_id);
                    if let Some(next) = search
                        .choices(
                            &step.transition.after,
                            &next_used,
                            *next_tier,
                            budgets[index + 1],
                            step.evaluation.score,
                        )
                        .first()
                    {
                        horizon += 0.5 * search.supported_value(next, *next_tier);
                    }
                }
                (step, horizon)
            })
            .collect::<Vec<_>>();
        beam.sort_by(|(left, left_value), (right, right_value)| {
            right_value.total_cmp(left_value).then_with(|| {
                left.transition
                    .purchased_id
                    .cmp(&right.transition.purchased_id)
            })
        });
        let Some((mut step, _)) = beam.into_iter().next() else {
            skipped += 1;
            continue;
        };
        if step.marginal_value <= evaluation.score.abs().max(1.0) * 1e-9 {
            skipped += 1;
            continue;
        }
        if inventory.apply_transition(&step.transition).is_err() {
            skipped += 1;
            continue;
        }
        used.insert(step.transition.purchased_id);
        if let Ok(held) = inventory.held_items(&search.catalog) {
            step.evaluation = evaluate_inventory(hero, &held, cfg);
        }
        evaluation = step.evaluation.clone();
        steps.push(step);
    }
    PurchasePlan {
        steps,
        final_evaluation: evaluation,
        assumptions: vec![
            "Kaufplanung mit vier Kandidaten und einem weiteren Kauf als Vorschau; begrenzte Suche, kein Beweis für das globale Optimum. Frühe Käufe gewichten gemeinsamen Mehrwert je Seele, spätere den Mehrwert im verfügbaren Inventar.".into(),
            format!("Kaufstufen folgen dem beobachteten Layout. Das Seelenbudget je Stufe ist die kumulierte Obergrenze der Kandidatenpreise, kein gemessener Spielzeitpunkt. {skipped} geplante Käufe ohne positiven zulässigen Übergang ausgelassen."),
            format!("Inventarregel: {} universelle Plätze, Verkaufserlös {:.0}% des Gesamtpreises; Verkaufsentscheidungen berücksichtigen den verlorenen gemeinsamen Nutzen. Freischaltzeitpunkte und Kulanz-Erstattungen werden nicht simuliert.", rules.max_slots, rules.resale_fraction * 100.0),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hero() -> HeroModel {
        serde_json::from_value(serde_json::json!({
            "hero_id": 1, "name": "Prüfheld", "archetype": "weapon", "base_health": 600.0,
            "level_curve": [], "purchase_bonuses": {"spirit": [], "weapon": [], "vitality": []},
            "scaling": [], "weapon": {"bullet_damage": 30.0, "shots_per_second": 4.0, "clip_size": 16.0,
                "reload_duration": 2.0, "range": 20.0, "falloff_start_range": 20.0,
                "falloff_end_range": 50.0, "sustained_dps": 80.0}, "abilities": [],
            "damage_plan": {"weapon_dps": 80.0, "spirit_dps": 0.0, "weapon_share": 1.0, "primary_axis": "Weapon"}
        })).unwrap()
    }

    fn item(id: i64, damage: f64, legacy_score: f64) -> ScoredItem {
        serde_json::from_value(serde_json::json!({
            "item": {"item_id": id, "name": format!("Item {id}"), "slot": "Weapon", "tier": 1,
                "cost": 800, "is_active": false, "shopable": true, "disabled": false,
                "damage_axis": "Weapon", "defense_kind": [], "properties": {"BaseAttackDamagePercent": damage},
                "passive_properties": {}, "condition": "None", "proc_cooldown": null, "imbueable": false},
            "score": {"combat_value": legacy_score, "per_slot_value": legacy_score, "per_soul_value": legacy_score,
                "purchase_bonus_value": 0.0, "condition_factor": 1.0, "active_value": 0.0,
                "passive_value": legacy_score, "meta_support": 0.0, "total": legacy_score},
            "confidence": "Low", "buy_phase": "Lane", "sources": []
        })).unwrap()
    }

    #[test]
    fn selection_uses_inventory_effect_instead_of_legacy_item_scores() {
        let hero = hero();
        let items = vec![item(1, 5.0, 10000.0), item(2, 50.0, -10000.0)];
        let candidates = items.iter().collect::<Vec<_>>();
        let mut layout = CoreLayoutStats::default();
        layout.bands.insert(
            1,
            crate::CoreLayoutBand {
                tier: 1,
                median: 1.0,
                lower_quartile: 1.0,
                upper_quartile: 1.0,
                target: 1,
            },
        );
        let rules = InventoryRules::from_catalog(
            &items
                .iter()
                .map(|item| item.item.clone())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let plan = plan_purchases(
            &hero,
            &items,
            &candidates,
            &layout,
            &rules,
            &BTreeMap::new(),
            &ReasonerConfig::default(),
        );
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps[0].transition.purchased_id, 2);
        assert_eq!(plan.steps[0].transition.after.spent_souls, 800);
        assert!(plan.steps[0].marginal_value > 0.0);
        assert_eq!(plan.final_evaluation, plan.steps[0].evaluation);
    }

    #[test]
    fn replacement_counts_the_lost_item_and_never_exceeds_slot_capacity() {
        let hero = hero();
        let mut items = vec![item(1, 10.0, 1.0), item(2, 60.0, 2.0)];
        items[1].item.tier = 2;
        items[1].item.cost = 1600;
        let candidates = items.iter().collect::<Vec<_>>();
        let mut layout = CoreLayoutStats::default();
        for tier in [1, 2] {
            layout.bands.insert(
                tier,
                crate::CoreLayoutBand {
                    tier,
                    median: 1.0,
                    lower_quartile: 1.0,
                    upper_quartile: 1.0,
                    target: 1,
                },
            );
        }
        let rules = InventoryRules {
            max_slots: 1,
            upgrade_components: BTreeMap::new(),
            resale_fraction: 0.5,
        };
        let cfg = ReasonerConfig::default();
        let plan = plan_purchases(
            &hero,
            &items,
            &candidates,
            &layout,
            &rules,
            &BTreeMap::new(),
            &cfg,
        );
        assert_eq!(plan.steps.len(), 2);
        let replacement = &plan.steps[1];
        assert_eq!(replacement.transition.sold_ids, [1]);
        assert_eq!(replacement.transition.after.held_ids, BTreeSet::from([2]));
        assert_eq!(replacement.transition.after.spent_souls, 2000);
        let standalone = evaluate_inventory(&hero, &[items[1].item.clone()], &cfg);
        assert_eq!(replacement.evaluation, standalone);
        assert!(
            (replacement.marginal_value - (standalone.score - plan.steps[0].evaluation.score))
                .abs()
                < 1e-9
        );
    }
}
