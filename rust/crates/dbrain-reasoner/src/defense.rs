//! Resource capacity against an explicit, constant incoming damage mixture.
//! Channel-limited shields cannot be converted into universal health.

/// Finds raw incoming damage x at which
/// max(weapon_rate*x - weapon_shield, 0) +
/// max(spirit_rate*x - spirit_shield, 0) == health + universal_shield.
/// Rates include the scenario's damage shares and relevant mitigation/debuffs.
/// This is a bounded capacity calculation, not a dynamic enemy rotation.
pub(crate) fn mixed_damage_capacity(
    health: f64,
    universal_shield: f64,
    weapon_shield: f64,
    spirit_shield: f64,
    weapon_rate: f64,
    spirit_rate: f64,
) -> f64 {
    if [
        health,
        universal_shield,
        weapon_shield,
        spirit_shield,
        weapon_rate,
        spirit_rate,
    ]
    .iter()
    .any(|v| !v.is_finite())
    {
        return 0.0;
    }
    let pool = health.max(0.0) + universal_shield.max(0.0);
    let weapon = weapon_shield.max(0.0);
    let spirit = spirit_shield.max(0.0);
    let wr = weapon_rate.max(0.0);
    let sr = spirit_rate.max(0.0);
    if wr == 0.0 {
        return if sr > 0.0 { (pool + spirit) / sr } else { 0.0 };
    }
    if sr == 0.0 {
        return (pool + weapon) / wr;
    }
    let weapon_exhausted = weapon / wr;
    let spirit_exhausted = spirit / sr;
    let (first, second, first_rate) = if weapon_exhausted <= spirit_exhausted {
        (weapon_exhausted, spirit_exhausted, wr)
    } else {
        (spirit_exhausted, weapon_exhausted, sr)
    };
    let before_second = first + pool / first_rate;
    if before_second <= second {
        before_second
    } else {
        (pool + weapon + spirit) / (wr + sr)
    }
}

/// Finite shields and cumulative incoming health damage across combat steps.
/// A constant capacity does not refill a consumed shield. A new increase grants
/// only the increase; an expired/decreased capacity clamps the remaining pool.
#[derive(Debug, Default)]
pub(crate) struct DamageLedger {
    capacities: [f64; 3],
    pub remaining: [f64; 3], // universal, weapon, spirit
    pub health_damage: f64,
}

impl DamageLedger {
    pub fn synchronize(&mut self, universal: f64, weapon: f64, spirit: f64) {
        for (idx, capacity) in [universal, weapon, spirit].into_iter().enumerate() {
            let capacity = if capacity.is_finite() {
                capacity.max(0.0)
            } else {
                0.0
            };
            let increase = (capacity - self.capacities[idx]).max(0.0);
            self.remaining[idx] = (self.remaining[idx] + increase).min(capacity);
            self.capacities[idx] = capacity;
        }
    }

    pub fn receive(&mut self, raw_damage: f64, weapon_rate: f64, spirit_rate: f64) {
        if !raw_damage.is_finite()
            || raw_damage <= 0.0
            || !weapon_rate.is_finite()
            || !spirit_rate.is_finite()
        {
            return;
        }
        let mut weapon = raw_damage * weapon_rate.max(0.0);
        let mut spirit = raw_damage * spirit_rate.max(0.0);
        let weapon_absorbed = weapon.min(self.remaining[1]);
        self.remaining[1] -= weapon_absorbed;
        weapon -= weapon_absorbed;
        let spirit_absorbed = spirit.min(self.remaining[2]);
        self.remaining[2] -= spirit_absorbed;
        spirit -= spirit_absorbed;
        let residual = weapon + spirit;
        let universal_absorbed = residual.min(self.remaining[0]);
        self.remaining[0] -= universal_absorbed;
        self.health_damage += residual - universal_absorbed;
    }
}

#[cfg(test)]
mod ledger_tests {
    use super::*;
    #[test]
    fn finite_shield_is_consumed_once_not_refilled_per_tick() {
        let mut state = DamageLedger::default();
        state.synchronize(0.0, 100.0, 0.0);
        state.receive(80.0, 1.0, 0.0);
        assert_eq!(state.remaining[1], 20.0);
        state.synchronize(0.0, 100.0, 0.0);
        state.receive(80.0, 1.0, 0.0);
        assert_eq!(state.remaining[1], 0.0);
        assert_eq!(state.health_damage, 60.0);
        state.synchronize(0.0, 150.0, 0.0);
        assert_eq!(state.remaining[1], 50.0);
        state.synchronize(0.0, 0.0, 0.0);
        assert_eq!(state.remaining[1], 0.0);
    }
    #[test]
    fn typed_and_shared_shields_deplete_before_health_without_cross_channel_credit() {
        let mut state = DamageLedger::default();
        state.synchronize(20.0, 100.0, 0.0);
        state.receive(200.0, 0.5, 0.5);
        assert_eq!(state.remaining, [0.0, 0.0, 0.0]);
        assert_eq!(state.health_damage, 80.0);
        state.synchronize(20.0, 100.0, 0.0);
        state.receive(100.0, 0.0, 1.0);
        assert_eq!(state.health_damage, 180.0);
    }
    #[test]
    fn integrating_constant_damage_in_steps_preserves_total_resources() {
        let mut full = DamageLedger::default();
        let mut split = DamageLedger::default();
        full.synchronize(40.0, 90.0, 50.0);
        split.synchronize(40.0, 90.0, 50.0);
        full.receive(1000.0, 0.2, 0.7);
        for _ in 0..10 {
            split.synchronize(40.0, 90.0, 50.0);
            split.receive(100.0, 0.2, 0.7);
        }
        assert_eq!(full.remaining, split.remaining);
        assert!((full.health_damage - split.health_damage).abs() < 1e-9);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrong_channel_is_worth_nothing_against_pure_damage() {
        assert_eq!(
            mixed_damage_capacity(600.0, 0.0, 100.0, 0.0, 0.0, 1.0),
            600.0
        );
        assert_eq!(
            mixed_damage_capacity(600.0, 0.0, 0.0, 100.0, 1.0, 0.0),
            600.0
        );
        assert_eq!(
            mixed_damage_capacity(600.0, 0.0, 100.0, 0.0, 1.0, 0.0),
            700.0
        );
    }
    #[test]
    fn unspent_channel_shield_cannot_pay_for_health_loss() {
        let value = mixed_damage_capacity(600.0, 0.0, 1000.0, 0.0, 0.1, 0.9);
        assert!((value - 600.0 / 0.9).abs() < 1e-9);
        assert!(value < 1600.0);
        assert_eq!(
            mixed_damage_capacity(600.0, 1000.0, 0.0, 0.0, 0.1, 0.9),
            1600.0
        );
    }
    #[test]
    fn resistances_apply_to_their_channel_and_exchange_symmetrically() {
        let a = mixed_damage_capacity(600.0, 20.0, 100.0, 50.0, 0.2, 0.5);
        let b = mixed_damage_capacity(600.0, 20.0, 50.0, 100.0, 0.5, 0.2);
        assert_eq!(a, b);
        assert!((a - 770.0 / 0.7).abs() < 1e-9);
        assert_eq!(
            mixed_damage_capacity(600.0, 0.0, 100.0, 0.0, 0.5, 0.0),
            1400.0
        );
    }
    #[test]
    fn capacity_has_a_continuous_channel_exhaustion_boundary() {
        for health in [49.999, 50.0, 50.001] {
            let capacity = mixed_damage_capacity(health, 0.0, 100.0, 0.0, 0.5, 0.5);
            let actual = (0.5 * capacity - 100.0).max(0.0) + (0.5 * capacity).max(0.0);
            assert!((actual - health).abs() < 1e-9);
        }
    }
}
