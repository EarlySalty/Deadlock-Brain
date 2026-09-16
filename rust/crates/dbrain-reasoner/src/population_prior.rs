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

    pub fn support(&self, item_id: i64, mechanic_slot_value: f64) -> f64 {
        let prevalence = self.prevalence(item_id);
        if prevalence <= 0.0 {
            return 0.0;
        }
        POPULATION_PRIOR_WEIGHT * prevalence * mechanic_slot_value.max(0.0)
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
}
