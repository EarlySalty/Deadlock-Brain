use std::collections::HashMap;

pub fn kendall_tau(build_order: &[i64], population_positions: &[(i64, f64)]) -> f64 {
    let positions: HashMap<i64, f64> = population_positions.iter().copied().collect();
    let shared: Vec<(usize, f64)> = build_order
        .iter()
        .enumerate()
        .filter_map(|(index, item)| positions.get(item).map(|pos| (index, *pos)))
        .collect();
    if shared.len() < 2 {
        return f64::NAN;
    }
    let mut concordant = 0i64;
    let mut discordant = 0i64;
    let mut ties_build = 0i64;
    let mut ties_pop = 0i64;
    for i in 0..shared.len() {
        for j in (i + 1)..shared.len() {
            let build_delta = (shared[i].0 as f64) - (shared[j].0 as f64);
            let pop_delta = shared[i].1 - shared[j].1;
            let build_tied = build_delta == 0.0;
            let pop_tied = pop_delta == 0.0;
            if build_tied && pop_tied {
                ties_build += 1;
                ties_pop += 1;
            } else if build_tied {
                ties_build += 1;
            } else if pop_tied {
                ties_pop += 1;
            } else if build_delta.signum() == pop_delta.signum() {
                concordant += 1;
            } else {
                discordant += 1;
            }
        }
    }
    let denominator = (((concordant + discordant + ties_build) as f64)
        * ((concordant + discordant + ties_pop) as f64))
        .sqrt();
    if denominator == 0.0 {
        return f64::NAN;
    }
    (concordant - discordant) as f64 / denominator
}

pub fn jaccard_at(k: usize, build_items: &[i64], player_items: &[i64]) -> f64 {
    use std::collections::HashSet;
    let top_build: HashSet<i64> = build_items.iter().take(k).copied().collect();
    let top_player: HashSet<i64> = player_items.iter().take(k).copied().collect();
    if top_build.is_empty() && top_player.is_empty() {
        return f64::NAN;
    }
    let intersection = top_build.intersection(&top_player).count();
    let union = top_build.union(&top_player).count();
    intersection as f64 / union as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kendall_tau_is_one_for_identical_order() {
        let build = vec![10, 20, 30, 40];
        let population = vec![(10, 0.0), (20, 1.0), (30, 2.0), (40, 3.0)];
        assert!((kendall_tau(&build, &population) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn kendall_tau_is_minus_one_for_reversed_order() {
        let build = vec![10, 20, 30, 40];
        let population = vec![(10, 3.0), (20, 2.0), (30, 1.0), (40, 0.0)];
        assert!((kendall_tau(&build, &population) + 1.0).abs() < 1e-9);
    }

    #[test]
    fn kendall_tau_ignores_items_without_population_position() {
        let build = vec![10, 99, 20, 30];
        let population = vec![(10, 0.0), (20, 1.0), (30, 2.0)];
        assert!((kendall_tau(&build, &population) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn kendall_tau_needs_two_shared_items() {
        let build = vec![10, 20];
        let population = vec![(10, 0.0)];
        assert!(kendall_tau(&build, &population).is_nan());
    }

    #[test]
    fn jaccard_at_counts_shared_top_k() {
        let build = vec![1, 2, 3, 4, 5, 6];
        let player = vec![1, 2, 3, 7, 8, 9];
        assert!((jaccard_at(6, &build, &player) - (3.0 / 9.0)).abs() < 1e-9);
    }

    #[test]
    fn jaccard_at_full_overlap_is_one() {
        let build = vec![1, 2, 3];
        let player = vec![1, 2, 3];
        assert!((jaccard_at(3, &build, &player) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn jaccard_at_is_nan_for_two_empty_sets() {
        assert!(jaccard_at(6, &[], &[]).is_nan());
    }
}
