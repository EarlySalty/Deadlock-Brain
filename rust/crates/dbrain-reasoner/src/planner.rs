use std::collections::{BTreeMap, BTreeSet};

use crate::combat::{
    evaluate_inventory, evaluate_inventory_refs_fast_with_bindings,
    evaluate_inventory_with_bindings, InventoryEvaluation,
};
use crate::inventory::{Inventory, InventoryRules, PurchaseTransition};
use crate::progression::{at_souls, ProgressionEvidence};
use crate::{AbilityStep, CoreLayoutStats, HeroModel, ItemModel, ReasonerConfig, ScoredItem};

const BEAM_WIDTH: usize = 4;
type EvaluationKey = (i64, Vec<i64>, Vec<(i64, i64)>);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ChoiceKey {
    earned: i64,
    spent: i64,
    before: u64,
    held: Vec<i64>,
    used: Vec<i64>,
    remaining: Vec<(i64, usize)>,
    bindings: Vec<(i64, i64)>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EconomyPolicy {
    pub checkpoints: Vec<i64>,
}
impl Default for EconomyPolicy {
    fn default() -> Self {
        Self {
            checkpoints: vec![
                800, 1600, 2400, 3200, 4000, 4800, 6400, 8000, 9600, 11200, 12800, 14400, 16000,
                19200, 22400, 25600, 28800, 32000, 35200, 38400, 41600, 44800, 48000, 54400, 60800,
                67200, 73600, 80000,
            ],
        }
    }
}

pub struct PlanningContext<'a> {
    pub layout: &'a CoreLayoutStats,
    pub rules: &'a InventoryRules,
    pub combinations: &'a BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
    pub order: &'a [AbilityStep],
    pub economy: &'a EconomyPolicy,
    pub population: Option<&'a crate::PopulationPrior>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PurchaseStep {
    pub transition: PurchaseTransition,
    pub evaluation: InventoryEvaluation,
    pub marginal_value: f64,
    pub progression: ProgressionEvidence,
    pub imbue_targets: BTreeMap<i64, i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SavingDecision {
    pub earned_souls: i64,
    pub available_souls: i64,
    pub reason: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PurchasePlan {
    pub steps: Vec<PurchaseStep>,
    pub final_evaluation: InventoryEvaluation,
    pub assumptions: Vec<String>,
    pub ability_order: Vec<AbilityStep>,
    pub saving_decisions: Vec<SavingDecision>,
}

struct Search<'a> {
    hero: &'a HeroModel,
    cfg: &'a ReasonerConfig,
    order: &'a [AbilityStep],
    catalog: Vec<ItemModel>,
    candidates: &'a [&'a ScoredItem],
    rules: &'a InventoryRules,
    combinations: &'a BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
    population: Option<&'a crate::PopulationPrior>,
    prior_slot: BTreeMap<i64, f64>,
    cache: BTreeMap<EvaluationKey, Option<InventoryEvaluation>>,
    invalid_metrics: BTreeSet<String>,
    choices_cache: BTreeMap<ChoiceKey, Vec<PurchaseStep>>,
    progression_cache: BTreeMap<i64, (HeroModel, ProgressionEvidence)>,
}

impl Search<'_> {
    fn supported_value(&self, step: &PurchaseStep) -> f64 {
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
        let mechanic = step.marginal_value + step.marginal_value.abs() * observed;
        let population = self.population.map_or(0.0, |prior| {
            if prior.is_staple(id) {
                prior.support(id, self.prior_slot.get(&id).copied().unwrap_or(0.0))
            } else {
                0.0
            }
        });
        mechanic + population
    }

    fn is_priority_staple(&self, item_id: i64) -> bool {
        self.population
            .is_some_and(|prior| prior.is_staple(item_id))
            && self.prior_slot.get(&item_id).copied().unwrap_or(0.0) > 0.0
    }

    fn is_priority_staple_step(&self, step: &PurchaseStep) -> bool {
        self.is_priority_staple(step.transition.purchased_id) && step.marginal_value >= 0.0
    }

    fn evaluate(
        &mut self,
        inventory: &Inventory,
        earned: i64,
        bindings: &BTreeMap<i64, i64>,
    ) -> Option<InventoryEvaluation> {
        let (hero, progression) = self
            .progression_cache
            .entry(earned)
            .or_insert_with(|| at_souls(self.hero, self.order, earned, self.cfg));
        let key = (
            progression.reached_level,
            inventory.held_ids.iter().copied().collect::<Vec<_>>(),
            bindings
                .iter()
                .map(|(id, target)| (*id, *target))
                .collect::<Vec<_>>(),
        );
        if let Some(evaluation) = self.cache.get(&key) {
            return evaluation.clone();
        }
        let held = inventory
            .held_ids
            .iter()
            .map(|id| self.catalog.iter().find(|item| item.item_id == *id))
            .collect::<Option<Vec<_>>>()?;
        let evaluation =
            evaluate_inventory_refs_fast_with_bindings(hero, &held, self.cfg, bindings);
        let invalid = !evaluation.score.is_finite()
            || !evaluation.utility.is_finite()
            || !evaluation.effective_health.is_finite()
            || evaluation.effective_health < 0.0
            || [
                evaluation.weapon_damage,
                evaluation.ability_damage,
                evaluation.proc_damage,
            ]
            .iter()
            .any(|damage| !damage.is_finite() || *damage < 0.0);
        if invalid {
            self.invalid_metrics.insert("Inventarzustände mit nicht endlichen Kampfwerten, negativem Schaden oder negativem abgeleitetem effektivem Leben wurden als ungültig verworfen; ihre Werte wurden nicht geklemmt. Tod im Szenario allein disqualifiziert keinen Zustand.".into());
            self.cache.insert(key, None);
            return None;
        }
        self.cache.insert(key, Some(evaluation.clone()));
        Some(evaluation)
    }

    fn choices(
        &mut self,
        inventory: &Inventory,
        used: &BTreeSet<i64>,
        remaining: &BTreeMap<i64, usize>,
        earned: i64,
        before: f64,
        bindings: &BTreeMap<i64, i64>,
    ) -> Vec<PurchaseStep> {
        let key = ChoiceKey {
            earned,
            spent: inventory.spent_souls,
            before: before.to_bits(),
            held: inventory.held_ids.iter().copied().collect(),
            used: used.iter().copied().collect(),
            remaining: remaining
                .iter()
                .map(|(tier, count)| (*tier, *count))
                .collect(),
            bindings: bindings.iter().map(|(id, target)| (*id, *target)).collect(),
        };
        if let Some(choices) = self.choices_cache.get(&key) {
            return choices.clone();
        }
        let candidates = self
            .candidates
            .iter()
            .copied()
            .filter(|candidate| {
                remaining.get(&candidate.item.tier).copied().unwrap_or(0) > 0
                    && !used.contains(&candidate.item.item_id)
            })
            .collect::<Vec<_>>();
        let mut choices = Vec::new();
        for candidate in candidates {
            let item = &candidate.item;
            let forced = self.is_priority_staple(item.item_id);
            let mut transitions = Vec::new();
            let direct = inventory.preview_purchase(item, &self.catalog, self.rules, &[]);
            if let Some(transition) = direct
                .ok()
                .filter(|transition| transition.after.spent_souls <= earned)
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
                if transition.after.spent_souls > earned || transition.net_cost <= 0 {
                    continue;
                }
                let mut next_bindings = bindings
                    .iter()
                    .filter(|(id, _)| transition.after.held_ids.contains(id))
                    .map(|(id, target)| (*id, *target))
                    .collect::<BTreeMap<_, _>>();
                if item.imbueable {
                    let (hero, _) = self
                        .progression_cache
                        .entry(earned)
                        .or_insert_with(|| at_souls(self.hero, self.order, earned, self.cfg));
                    if let Some(target) = crate::mechanics::imbue_target(item, hero, self.cfg) {
                        next_bindings.insert(item.item_id, target);
                    }
                }
                let Some(evaluation) = self.evaluate(&transition.after, earned, &next_bindings)
                else {
                    continue;
                };
                let marginal_value = evaluation.score - before;
                let staple_rescue = forced && marginal_value >= 0.0;
                if marginal_value <= before.abs().max(1.0) * 1e-9 && !staple_rescue {
                    continue;
                }
                let step = PurchaseStep {
                    transition,
                    evaluation,
                    marginal_value,
                    progression: ProgressionEvidence::default(),
                    imbue_targets: next_bindings,
                };
                if best.as_ref().is_none_or(|previous| {
                    self.supported_value(&step) > self.supported_value(previous)
                }) {
                    best = Some(step);
                }
            }
            if let Some(step) = best {
                choices.push(step);
            }
        }
        choices.sort_by(|left, right| {
            let left_staple = self.is_priority_staple_step(left);
            let right_staple = self.is_priority_staple_step(right);
            right_staple
                .cmp(&left_staple)
                .then_with(|| {
                    self.supported_value(right)
                        .total_cmp(&self.supported_value(left))
                })
                .then_with(|| {
                    left.transition
                        .purchased_id
                        .cmp(&right.transition.purchased_id)
                })
        });
        choices.truncate(BEAM_WIDTH);
        self.choices_cache.insert(key, choices.clone());
        choices
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
    plan_with_economy(
        hero,
        catalog,
        candidates,
        cfg,
        PlanningContext {
            layout,
            rules,
            combinations,
            order: &[],
            economy: &EconomyPolicy::default(),
            population: None,
        },
    )
}

pub fn plan_with_economy(
    hero: &HeroModel,
    catalog: &[ScoredItem],
    candidates: &[&ScoredItem],
    cfg: &ReasonerConfig,
    context: PlanningContext<'_>,
) -> PurchasePlan {
    let PlanningContext {
        layout,
        rules,
        combinations,
        order,
        economy,
        population,
    } = context;
    let prior_slot = population
        .map(|_| {
            catalog
                .iter()
                .map(|item| (item.item.item_id, item.score.per_slot_value))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let mut search = Search {
        hero,
        cfg,
        order,
        catalog: catalog.iter().map(|item| item.item.clone()).collect(),
        candidates,
        rules,
        combinations,
        population,
        prior_slot,
        cache: BTreeMap::new(),
        invalid_metrics: BTreeSet::new(),
        choices_cache: BTreeMap::new(),
        progression_cache: BTreeMap::new(),
    };
    let mut plan=PurchasePlan { steps:Vec::new(), final_evaluation:evaluate_inventory(hero,&[],cfg), ability_order:order.to_vec(),saving_decisions:Vec::new(), assumptions:vec![
        "Feste globale Seelen-Checkpoints, unabhängig von Kandidatenpreisen. Das Layout begrenzt Käufe je Kostenband; offene Bänder konkurrieren um das vorhandene Geld. Sparen bleibt eine bewertete Alternative.".into(),
        "Begrenzte Zweischrittsuche mit vier Kandidaten: aktueller Mehrwert zählt zur Hälfte, der Zustand am nächsten Checkpoint voll. Das ist eine gesetzte Planungspräferenz, kein gemessener Spielverlauf und kein globales Optimum.".into(),
        "Budget und Levelkurve verwenden dieselben verdienten Szenarioseelen. Anfangsgeld, Einnahmetempo, Zeitverluste und zusätzliche AP aus Spielereignissen werden nicht erfunden. Verkäufe verändern nur Ausgaben, nicht den Fortschritt.".into(),
        format!("Inventarregel: {} universelle Plätze und höchstens vier aktive Items; Verkaufserlös {:.0}%. Freischaltzeitpunkte von Inventarplätzen und Kulanz-Erstattungen bleiben unmodelliert.",rules.max_slots,rules.resale_fraction*100.0),
    ]};
    if economy.checkpoints.is_empty()
        || economy.checkpoints.iter().any(|point| *point < 0)
        || economy
            .checkpoints
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
    {
        plan.assumptions
            .push("Ungültige Budget-Checkpoints: keine Kaufkurve berechnet.".into());
        return plan;
    }
    let mut inventory = Inventory::default();
    let mut bindings = BTreeMap::new();
    let mut used = BTreeSet::new();
    let mut remaining = layout
        .bands
        .iter()
        .map(|(tier, band)| (*tier, band.target))
        .collect::<BTreeMap<_, _>>();
    for (index, earned) in economy.checkpoints.iter().copied().enumerate() {
        if remaining.values().all(|count| *count == 0) {
            break;
        }
        loop {
            let Some(before) = search.evaluate(&inventory, earned, &bindings) else {
                plan.assumptions
                    .push("Ungültiger Inventar-/Kampfzustand beendet die Kaufplanung.".into());
                plan.assumptions.extend(search.invalid_metrics);
                return plan;
            };
            let choices = search.choices(
                &inventory,
                &used,
                &remaining,
                earned,
                before.score,
                &bindings,
            );
            let next_earned = economy.checkpoints.get(index + 1).copied();
            let future_before = next_earned
                .and_then(|next| search.evaluate(&inventory, next, &bindings))
                .map(|evaluation| evaluation.score)
                .unwrap_or(before.score);
            let save_value = if let Some(next) = next_earned {
                search
                    .choices(
                        &inventory,
                        &used,
                        &remaining,
                        next,
                        future_before,
                        &bindings,
                    )
                    .first()
                    .map(|step| search.supported_value(step))
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            let mut beam = Vec::new();
            for step in choices.into_iter().take(BEAM_WIDTH) {
                let mut horizon = search.supported_value(&step);
                if let Some(next) = next_earned {
                    let mut next_used = used.clone();
                    next_used.insert(step.transition.purchased_id);
                    let mut next_remaining = remaining.clone();
                    let tier = catalog
                        .iter()
                        .find(|item| item.item.item_id == step.transition.purchased_id)
                        .map(|item| item.item.tier)
                        .unwrap_or(0);
                    if let Some(count) = next_remaining.get_mut(&tier) {
                        *count = count.saturating_sub(1);
                    }
                    if let Some(future) =
                        search.evaluate(&step.transition.after, next, &step.imbue_targets)
                    {
                        let next_gain = search
                            .choices(
                                &step.transition.after,
                                &next_used,
                                &next_remaining,
                                next,
                                future.score,
                                &step.imbue_targets,
                            )
                            .first()
                            .map(|step| search.supported_value(step))
                            .unwrap_or(0.0);
                        horizon = 0.5 * horizon + future.score - future_before + next_gain;
                    }
                }
                beam.push((step, horizon));
            }
            beam.sort_by(|(left, l), (right, r)| {
                let left_staple = search.is_priority_staple_step(left);
                let right_staple = search.is_priority_staple_step(right);
                right_staple
                    .cmp(&left_staple)
                    .then_with(|| r.total_cmp(l))
                    .then_with(|| {
                        left.transition
                            .purchased_id
                            .cmp(&right.transition.purchased_id)
                    })
            });
            let Some((mut step, buy_value)) = beam.into_iter().next() else {
                plan.saving_decisions.push(SavingDecision { earned_souls:earned,available_souls:earned-inventory.spent_souls,reason:"Kein bezahlbarer Kauf mit positivem gemeinsamen Mehrwert; Geld bleibt verfügbar.".into() });
                break;
            };
            let buying_staple = search.is_priority_staple_step(&step);
            if !buying_staple && save_value > buy_value + before.score.abs().max(1.0) * 1e-9 {
                plan.saving_decisions.push(SavingDecision { earned_souls:earned,available_souls:earned-inventory.spent_souls,reason:"Sparen ermöglicht am nächsten Checkpoint den stärkeren gemeinsamen Zustand als Kauf plus Folgeentscheidung.".into() });
                break;
            }
            if let Err(error) = inventory.apply_transition(&step.transition) {
                plan.assumptions.push(format!(
                    "Kaufplanung wegen ungültigem Übergang beendet: {error}"
                ));
                return plan;
            }
            let (progressed, evidence) = at_souls(hero, order, earned, cfg);
            let held = match inventory.held_items(&search.catalog) {
                Ok(held) => held,
                Err(error) => {
                    plan.assumptions
                        .push(format!("Kaufbeleg kann nicht erstellt werden: {error}"));
                    return plan;
                }
            };
            step.evaluation =
                evaluate_inventory_with_bindings(&progressed, &held, cfg, &step.imbue_targets);
            bindings = step.imbue_targets.clone();
            step.progression = evidence;
            used.insert(step.transition.purchased_id);
            if let Some(tier) = catalog
                .iter()
                .find(|item| item.item.item_id == step.transition.purchased_id)
                .map(|item| item.item.tier)
            {
                if let Some(count) = remaining.get_mut(&tier) {
                    *count = count.saturating_sub(1);
                }
            }
            plan.final_evaluation = step.evaluation.clone();
            plan.steps.push(step);
            if remaining.values().all(|count| *count == 0) {
                break;
            }
        }
    }
    if let Some(last) = plan.steps.last() {
        plan.assumptions
            .extend(last.progression.assumptions.iter().cloned());
        plan.assumptions
            .extend(last.progression.unknown_effects.iter().cloned());
    } else if let Some(earned) = economy.checkpoints.last() {
        let (progressed, evidence) = at_souls(hero, order, *earned, cfg);
        plan.final_evaluation = evaluate_inventory_with_bindings(&progressed, &[], cfg, &bindings);
        plan.assumptions.extend(evidence.assumptions);
        plan.assumptions.extend(evidence.unknown_effects);
    }
    plan.assumptions.push(format!(
        "{} ungefüllte Layoutplätze am letzten Budgetpunkt. Keine unbezahlbaren Käufe ergänzt.",
        remaining.values().sum::<usize>()
    ));
    plan.assumptions.extend(search.invalid_metrics);
    plan
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
    fn frozen_acquisition_states_allow_funding_sales_before_slots_are_full() {
        for (held, spent, earned, buy_id, buy_cost, sell_id, expected_spent) in [
            (
                vec![
                    (968099481, 800),
                    (1763073141, 1600),
                    (1770441818, 1600),
                    (2356412290, 1600),
                    (2678489038, 3200),
                    (2971868509, 1600),
                    (3077079169, 800),
                    (3633614685, 800),
                    (3919289022, 6400),
                    (3977876567, 1600),
                    (4104549924, 1600),
                ],
                21600,
                25600,
                365620721,
                6400,
                3919289022,
                24800,
            ),
            (
                vec![
                    (381961617, 1600),
                    (668299740, 800),
                    (1770441818, 1600),
                    (2356412290, 1600),
                    (2678489038, 3200),
                    (2971868509, 1600),
                    (3633614685, 800),
                    (3977876567, 1600),
                ],
                12800,
                14400,
                3791587546,
                3200,
                2678489038,
                14400,
            ),
        ] {
            let mut catalog = held
                .iter()
                .map(|(id, cost)| {
                    let mut held_item = item(*id, 0.0, 0.0).item;
                    held_item.cost = *cost;
                    held_item
                })
                .collect::<Vec<_>>();
            let mut purchase = item(buy_id, 0.0, 0.0).item;
            purchase.cost = buy_cost;
            catalog.push(purchase.clone());
            let rules = InventoryRules::from_catalog(&catalog).unwrap();
            let inventory = Inventory {
                held_ids: held.iter().map(|(id, _)| *id).collect(),
                spent_souls: spent,
            };
            assert!(inventory.held_ids.len() < rules.max_slots);
            let direct = inventory
                .preview_purchase(&purchase, &catalog, &rules, &[])
                .unwrap();
            assert!(direct.after.spent_souls > earned);
            let funded = inventory
                .preview_purchase(&purchase, &catalog, &rules, &[sell_id])
                .unwrap();
            assert_eq!(funded.after.spent_souls, expected_spent);
            assert!(funded.after.spent_souls <= earned);
            assert_eq!(funded.after.held_ids.len(), inventory.held_ids.len());
        }
    }

    #[test]
    fn affordable_sale_is_evaluated_with_free_slots_and_can_be_rejected() {
        let hero = hero();
        let cfg = ReasonerConfig::default();
        let mut items = [item(1, 1.0, 0.0), item(2, 40.0, 0.0), item(3, 20.0, 0.0)];
        items[0].item.cost = 1600;
        items[2].item.cost = 1600;
        let catalog = items
            .iter()
            .map(|item| item.item.clone())
            .collect::<Vec<_>>();
        let rules = InventoryRules::from_catalog(&catalog).unwrap();
        let candidates = vec![&items[2]];
        let combinations = BTreeMap::new();
        let inventory = Inventory {
            held_ids: BTreeSet::from([1, 2]),
            spent_souls: 2400,
        };
        let bindings = BTreeMap::new();
        let mut search = Search {
            hero: &hero,
            cfg: &cfg,
            order: &[],
            catalog,
            candidates: &candidates,
            rules: &rules,
            combinations: &combinations,
            population: None,
            prior_slot: BTreeMap::new(),
            cache: BTreeMap::new(),
            invalid_metrics: BTreeSet::new(),
            choices_cache: BTreeMap::new(),
            progression_cache: BTreeMap::new(),
        };
        let before = search.evaluate(&inventory, 3200, &bindings).unwrap();
        let choices = search.choices(
            &inventory,
            &BTreeSet::from([1, 2]),
            &BTreeMap::from([(1, 1)]),
            3200,
            before.score,
            &bindings,
        );
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].transition.sold_ids, vec![1]);
        assert_eq!(choices[0].transition.net_cost, 800);
        assert_eq!(choices[0].transition.after.spent_souls, 3200);
        assert!(choices[0].transition.after.held_ids.len() < rules.max_slots);
        assert!(choices[0].marginal_value > 0.0);

        let stronger_owned = Inventory {
            held_ids: BTreeSet::from([2]),
            spent_souls: 800,
        };
        let before = search.evaluate(&stronger_owned, 2000, &bindings).unwrap();
        let rejected = search.choices(
            &stronger_owned,
            &BTreeSet::from([2]),
            &BTreeMap::from([(1, 1)]),
            2000,
            before.score,
            &bindings,
        );
        assert!(rejected.is_empty());
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
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps[0].transition.purchased_id, 2);
        assert_eq!(plan.steps[0].progression.earned_souls, 1600);
        assert!(plan
            .saving_decisions
            .iter()
            .any(|decision| decision.earned_souls == 800
                && decision.reason.starts_with("Sparen ermöglicht")));
        let before = Inventory {
            held_ids: BTreeSet::from([1]),
            spent_souls: 800,
        };
        let prior = evaluate_inventory(&hero, std::slice::from_ref(&items[0].item), &cfg);
        let combinations = BTreeMap::new();
        let mut search = Search {
            hero: &hero,
            cfg: &cfg,
            order: &[],
            catalog: items.iter().map(|item| item.item.clone()).collect(),
            candidates: &candidates,
            rules: &rules,
            combinations: &combinations,
            population: None,
            prior_slot: BTreeMap::new(),
            cache: BTreeMap::new(),
            invalid_metrics: BTreeSet::new(),
            choices_cache: BTreeMap::new(),
            progression_cache: BTreeMap::new(),
        };
        let choices = search.choices(
            &before,
            &BTreeSet::from([1]),
            &BTreeMap::from([(2, 1)]),
            2400,
            prior.score,
            &BTreeMap::new(),
        );
        let replacement = &choices[0];
        assert_eq!(replacement.transition.sold_ids, [1]);
        assert_eq!(replacement.transition.after.held_ids, BTreeSet::from([2]));
        assert_eq!(replacement.transition.after.spent_souls, 2000);
        let standalone = evaluate_inventory(&hero, &[items[1].item.clone()], &cfg);
        assert_eq!(replacement.evaluation.score, standalone.score);
        assert!((replacement.marginal_value - (standalone.score - prior.score)).abs() < 1e-9);
    }

    #[test]
    fn later_unlock_keeps_the_binding_chosen_at_purchase() {
        let mut hero = hero();
        hero.level_curve = vec![
            crate::LevelPoint {
                level: 1,
                required_souls: 0,
            },
            crate::LevelPoint {
                level: 2,
                required_souls: 200,
            },
            crate::LevelPoint {
                level: 3,
                required_souls: 500,
            },
        ];
        hero.level_rewards = BTreeMap::from([
            (1, vec!["EAbilityUnlocks".into()]),
            (2, vec!["EAbilityPoints".into()]),
            (3, vec!["EAbilityUnlocks".into()]),
        ]);
        let ability = |id, damage, cooldown| {
            serde_json::from_value(serde_json::json!({"ability_id":id,"class_name":format!("ability_{id}"),"slot":1,"roles":["Damage"],"scaling":[],"channel_time":null,"charges":1,"cooldown":cooldown,"scaling_step":null,"damage_type":"Spirit","base_effect":damage,"properties":{"Damage":damage,"AbilityCooldown":cooldown}})).unwrap()
        };
        hero.abilities = vec![ability(10, 60.0, 10.0), ability(20, 200.0, 2.0)];
        let order = vec![
            AbilityStep {
                ability_id: 10,
                currency_type: 2,
                delta: -1,
            },
            AbilityStep {
                ability_id: 20,
                currency_type: 2,
                delta: -1,
            },
        ];
        let mut items = vec![item(1, 100.0, 1.0), item(2, 20.0, 1.0)];
        items[0].item.cost = 100;
        items[0].item.imbueable = true;
        items[1].item.cost = 1000;
        let candidates = items.iter().collect::<Vec<_>>();
        let mut layout = CoreLayoutStats::default();
        layout.bands.insert(
            1,
            crate::CoreLayoutBand {
                tier: 1,
                median: 2.0,
                lower_quartile: 2.0,
                upper_quartile: 2.0,
                target: 2,
            },
        );
        let rules = InventoryRules::from_catalog(
            &items
                .iter()
                .map(|item| item.item.clone())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let cfg = ReasonerConfig::default();
        let plan = plan_with_economy(
            &hero,
            &items,
            &candidates,
            &cfg,
            PlanningContext {
                layout: &layout,
                rules: &rules,
                combinations: &BTreeMap::new(),
                order: &order,
                economy: &EconomyPolicy {
                    checkpoints: vec![100, 800, 1600],
                },
                population: None,
            },
        );
        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[0].progression.earned_souls, 100);
        assert_eq!(plan.steps[0].imbue_targets.get(&1), Some(&10));
        assert!(plan.steps[1].progression.ability_ranks.contains_key(&20));
        assert_eq!(plan.steps[1].imbue_targets.get(&1), Some(&10));
        let (later, _) = at_souls(&hero, &order, 1600, &cfg);
        assert_eq!(
            crate::mechanics::imbue_target(&items[0].item, &later, &cfg),
            Some(20)
        );
    }
}
