use std::collections::{BTreeMap, BTreeSet};

pub const POPULATION_PRIOR_WEIGHT: f64 = 0.30;

pub struct PopulationItem {
    pub item_id: i64,
    pub prevalence: f64,
    pub median_position: Option<f64>,
    pub is_staple: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PopulationPrior {
    prevalence: BTreeMap<i64, f64>,
    median_position: BTreeMap<i64, f64>,
    staples: BTreeSet<i64>,
}

impl PopulationPrior {
    pub fn from_items(items: impl IntoIterator<Item = PopulationItem>) -> Self {
        let mut prior = Self::default();
        for item in items {
            if item.prevalence.is_finite() && item.prevalence > 0.0 {
                prior
                    .prevalence
                    .insert(item.item_id, item.prevalence.clamp(0.0, 1.0));
            }
            if let Some(position) = item.median_position.filter(|value| value.is_finite()) {
                prior.median_position.insert(item.item_id, position);
            }
            if item.is_staple {
                prior.staples.insert(item.item_id);
            }
        }
        prior
    }

    pub fn is_empty(&self) -> bool {
        self.prevalence.is_empty() && self.staples.is_empty()
    }

    pub fn prevalence(&self, item_id: i64) -> f64 {
        self.prevalence.get(&item_id).copied().unwrap_or(0.0)
    }

    pub fn median_position(&self, item_id: i64) -> Option<f64> {
        self.median_position.get(&item_id).copied()
    }

    pub fn is_staple(&self, item_id: i64) -> bool {
        self.staples.contains(&item_id)
    }

    pub fn staples(&self) -> Vec<i64> {
        self.staples.iter().copied().collect()
    }

    pub fn positions(&self) -> Vec<(i64, f64)> {
        self.median_position
            .iter()
            .map(|(id, position)| (*id, *position))
            .collect()
    }

    pub fn ranked_by_prevalence(&self) -> Vec<i64> {
        let mut items = self
            .prevalence
            .iter()
            .map(|(id, value)| (*id, *value))
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            right
                .1
                .total_cmp(&left.1)
                .then_with(|| left.0.cmp(&right.0))
        });
        items.into_iter().map(|(id, _)| id).collect()
    }

    pub fn support(&self, item_id: i64, mechanic_slot_value: f64) -> f64 {
        let prevalence = self.prevalence(item_id);
        if prevalence <= 0.0 {
            return 0.0;
        }
        POPULATION_PRIOR_WEIGHT * prevalence * mechanic_slot_value.max(0.0)
    }

    pub fn thin_coverage_note(&self, core_ids: &[i64]) -> Option<String> {
        let total = self.staples.len();
        if total == 0 {
            return None;
        }
        let covered = core_ids.iter().filter(|id| self.is_staple(**id)).count();
        if covered * 2 < total {
            Some(format!(
                "Populations-Deckung dünn: nur {covered} von {total} Staples echter Spieler im Build; die Kaufkurve weicht stark von der Population ab."
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prior() -> PopulationPrior {
        PopulationPrior::from_items([
            PopulationItem {
                item_id: 1,
                prevalence: 0.9,
                median_position: Some(3.0),
                is_staple: true,
            },
            PopulationItem {
                item_id: 2,
                prevalence: 0.4,
                median_position: Some(8.0),
                is_staple: false,
            },
        ])
    }

    #[test]
    fn support_scales_with_prevalence_and_positive_mechanics_only() {
        let prior = prior();
        assert!((prior.support(1, 40.0) - POPULATION_PRIOR_WEIGHT * 0.9 * 40.0).abs() < 1e-9);
        assert_eq!(prior.support(1, -5.0), 0.0);
        assert_eq!(prior.support(999, 40.0), 0.0);
        assert!(prior.support(1, 40.0) > prior.support(2, 40.0));
    }

    #[test]
    fn staples_and_positions_are_reported() {
        let prior = prior();
        assert_eq!(prior.staples(), vec![1]);
        assert_eq!(prior.median_position(1), Some(3.0));
        assert!(prior.is_staple(1));
        assert!(!prior.is_staple(2));
        assert!(!prior.is_empty());
        assert!(PopulationPrior::default().is_empty());
    }

    #[test]
    fn thin_coverage_note_fires_only_below_half_of_the_staples() {
        let prior = PopulationPrior::from_items((0..10).map(|id| PopulationItem {
            item_id: id,
            prevalence: 0.8,
            median_position: Some(1.0),
            is_staple: true,
        }));
        assert!(prior.thin_coverage_note(&[0, 1, 2]).is_some());
        assert!(prior.thin_coverage_note(&(0..9).collect::<Vec<_>>()).is_none());
        assert!(PopulationPrior::default().thin_coverage_note(&[]).is_none());
    }
}
