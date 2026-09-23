//! Damage-triggered effects retain their actor, refresh duration and distinct targets.
//! Asset text is parsed once at the model boundary; simulation never reads display names.
use std::collections::BTreeMap;

use crate::ConditionKind;

fn plain_text(text: &str) -> String {
    let mut in_tag = false;
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '<' => {
                in_tag = true;
                out.push(' ');
            }
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// Narrow, fail-closed grammar for the effect described by the asset. Numbers alone
/// do not establish a trigger, and unknown wording is not guessed from an item ID.
pub(crate) fn spirit_refresh_condition(
    properties: &BTreeMap<String, f64>,
    description: &str,
) -> Option<ConditionKind> {
    let amount = *properties.get("Regeneration")?;
    let seconds = *properties.get("RegenerationDuration")?;
    if !amount.is_finite() || amount <= 0.0 || !seconds.is_finite() || seconds <= 0.0 {
        return None;
    }
    let text = plain_text(description);
    if !text.contains("spirit damage to enemy heroes")
        || !text.contains("regeneration")
        || !text.contains("different heroes")
        || ["does not", "not grant", "not trigger"]
            .iter()
            .any(|word| text.contains(word))
    {
        return None;
    }
    let max_stacks = match properties.get("MaxStacks") {
        None => None,
        Some(value)
            if value.is_finite()
                && *value >= 1.0
                && value.fract() == 0.0
                && *value <= u32::MAX as f64 =>
        {
            Some(*value as u32)
        }
        Some(_) => return None,
    };
    Some(ConditionKind::SpiritDamageToHeroes {
        refresh_seconds: seconds,
        max_stacks,
    })
}

#[derive(Debug, Default)]
pub(crate) struct RefreshStacks {
    expires: BTreeMap<usize, f64>,
}

impl RefreshStacks {
    fn exposure(&self, from: f64, to: f64, max_stacks: Option<u32>) -> f64 {
        if to <= from {
            return 0.0;
        }
        let Some(cap) = max_stacks else {
            return self
                .expires
                .values()
                .map(|end| (end.min(to) - from).max(0.0))
                .sum();
        };
        let mut boundaries: Vec<_> = self
            .expires
            .values()
            .copied()
            .filter(|end| *end > from && *end < to)
            .collect();
        boundaries.sort_by(f64::total_cmp);
        boundaries.dedup();
        boundaries.push(to);
        let mut cursor = from;
        let mut total = 0.0;
        for end in boundaries {
            let active = self
                .expires
                .values()
                .filter(|expiry| **expiry > cursor)
                .count()
                .min(cap as usize);
            total += active as f64 * (end - cursor);
            cursor = end;
        }
        total
    }

    /// Integrates stack-seconds exactly within a simulator step. Events at its end
    /// refresh the next step, not the elapsed interval. Callers supply only eligible
    /// positive damage to hero targets; proc-excluded and PvE events never enter here.
    pub(crate) fn integrate(
        &mut self,
        from: f64,
        to: f64,
        refresh_seconds: f64,
        max_stacks: Option<u32>,
        events: &[(f64, usize)],
    ) -> f64 {
        if !from.is_finite()
            || !to.is_finite()
            || to <= from
            || !refresh_seconds.is_finite()
            || refresh_seconds <= 0.0
        {
            return 0.0;
        }
        let mut events: Vec<_> = events
            .iter()
            .copied()
            .filter(|(time, _)| time.is_finite() && *time >= from && *time <= to)
            .collect();
        events.sort_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let mut cursor = from;
        let mut total = 0.0;
        for (time, target) in events {
            total += self.exposure(cursor, time, max_stacks);
            let expiry = time + refresh_seconds;
            if expiry.is_finite() {
                self.expires
                    .entry(target)
                    .and_modify(|old| *old = old.max(expiry))
                    .or_insert(expiry);
            }
            cursor = time;
        }
        total += self.exposure(cursor, to, max_stacks);
        self.expires.retain(|_, expiry| *expiry > to);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refresh_stack_counts_are_time_and_target_bound() {
        let mut burst = RefreshStacks::default();
        assert_eq!(burst.integrate(0.0, 10.0, 4.0, None, &[(0.0, 7)]), 4.0);
        let mut repeated = RefreshStacks::default();
        assert_eq!(
            repeated.integrate(0.0, 10.0, 4.0, None, &[(0.0, 7), (1.0, 7)]),
            5.0
        );
        let mut distinct = RefreshStacks::default();
        assert_eq!(
            distinct.integrate(0.0, 10.0, 4.0, None, &[(0.0, 7), (1.0, 8)]),
            8.0
        );
        let mut capped = RefreshStacks::default();
        assert_eq!(
            capped.integrate(0.0, 10.0, 4.0, Some(1), &[(0.0, 7), (1.0, 8)]),
            5.0
        );
    }
    #[test]
    fn refresh_state_survives_steps_and_does_not_heal_before_hit() {
        let mut state = RefreshStacks::default();
        assert_eq!(state.integrate(0.0, 1.0, 4.0, None, &[(1.0, 2)]), 0.0);
        assert_eq!(state.integrate(1.0, 3.0, 4.0, None, &[]), 2.0);
        assert_eq!(state.integrate(3.0, 10.0, 4.0, None, &[]), 2.0);
        assert_eq!(state.integrate(10.0, 12.0, 4.0, None, &[]), 0.0);
        assert_eq!(
            state.integrate(12.0, 13.0, 4.0, None, &[(f64::NAN, 2), (14.0, 3)]),
            0.0
        );
    }
    #[test]
    fn refresh_integration_is_event_order_invariant() {
        let events = [(0.0, 1), (3.0, 1), (6.0, 1), (9.0, 1)];
        let mut reversed = events;
        reversed.reverse();
        let mut a = RefreshStacks::default();
        let mut b = RefreshStacks::default();
        assert_eq!(a.integrate(0.0, 10.0, 4.0, None, &events), 10.0);
        assert_eq!(b.integrate(0.0, 10.0, 4.0, None, &reversed), 10.0);
    }
    #[test]
    fn refresh_parser_requires_complete_mechanism_not_only_regen_numbers() {
        let mut properties = BTreeMap::from([
            ("Regeneration".into(), 4.0),
            ("RegenerationDuration".into(), 7.0),
        ]);
        let text = "Dealing <span>spirit damage</span> to enemy Heroes grants you regeneration. Stacks when dealing damage to different heroes.";
        assert!(spirit_refresh_condition(&properties, text).is_some());
        assert!(spirit_refresh_condition(&properties, "regeneration").is_none());
        assert!(
            spirit_refresh_condition(&properties, &text.replace("enemy Heroes", "creeps"))
                .is_none()
        );
        properties.insert("RegenerationDuration".into(), f64::NAN);
        assert!(spirit_refresh_condition(&properties, text).is_none());
    }
}
