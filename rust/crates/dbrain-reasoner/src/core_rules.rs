//! Deterministic, side-effect-free evaluator of verified, release-pinned numeric facts.
use brain_contracts::{domain::*, CorpusRelease, DocumentRevision, PortError};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, Default)]
pub struct CoreRuleEvaluator;
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
type Unit = BTreeMap<String, i32>;
fn unit(raw: &str) -> Result<Unit, PortError> {
    if raw.is_empty() || raw.len() > 128 {
        return Err(invalid("invalid unit"));
    }
    let mut unit = Unit::new();
    let mut denominator = false;
    for part in raw.split('/') {
        if denominator && raw.matches('/').count() > 1 {
            return Err(invalid("ambiguous unit denominator"));
        }
        for factor in part.split('*') {
            if factor == "1" {
                continue;
            }
            let (name, power) = match factor.split_once('^') {
                Some((n, p)) => (
                    n,
                    p.parse::<i32>()
                        .map_err(|_| invalid("invalid unit power"))?,
                ),
                None => (factor, 1),
            };
            if name.is_empty()
                || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                || !(1..=8).contains(&power)
            {
                return Err(invalid("invalid unit atom"));
            }
            *unit.entry(name.into()).or_default() += if denominator { -power } else { power };
        }
        denominator = true;
    }
    unit.retain(|_, power| *power != 0);
    Ok(unit)
}
fn unit_text(unit: &Unit) -> String {
    let side = |positive: bool| {
        let terms: Vec<_> = unit
            .iter()
            .filter(|(_, p)| (**p > 0) == positive)
            .map(|(n, p)| {
                if p.abs() == 1 {
                    n.clone()
                } else {
                    format!("{n}^{}", p.abs())
                }
            })
            .collect();
        if terms.is_empty() {
            "1".into()
        } else {
            terms.join("*")
        }
    };
    if unit.values().any(|p| *p < 0) {
        format!("{}/{}", side(true), side(false))
    } else {
        side(true)
    }
}
fn pinned(source: &DocumentRevision, release: &CorpusRelease) -> bool {
    !source.content_hash.trim().is_empty()
        && release
            .source_revisions
            .get(&source.source_id)
            .and_then(|d| d.get(&source.logical_id))
            == Some(&source.revision)
        && source.revision > 0
}
impl RuleEvaluatorPort for CoreRuleEvaluator {
    fn evaluate(
        &self,
        rule: &TypedRule,
        facts: &[NumericFact],
        release: &CorpusRelease,
        validity: &Validity,
    ) -> Result<RuleEvaluation, PortError> {
        if !rule.verified
            || rule.rule_id.trim().is_empty()
            || rule.rule_version.trim().is_empty()
            || rule.validity != *validity
            || validity.patch != release.patch
            || validity.mode.trim().is_empty()
            || !pinned(&rule.source, release)
            || facts.len() > 10000
        {
            return Err(invalid("rule validity, release or verification mismatch"));
        }
        let mut by_id = BTreeMap::new();
        let mut subjects: BTreeMap<(&str, &str), &Quantity> = BTreeMap::new();
        for fact in facts {
            if !fact.verified || fact.validity != *validity || !pinned(&fact.source, release) {
                continue;
            }
            if !fact.quantity.value.is_finite() || fact.fact_id.trim().is_empty() {
                return Err(invalid("invalid fact"));
            }
            unit(&fact.quantity.unit)?;
            if let Some(old) = by_id.insert(fact.fact_id.as_str(), fact) {
                if old != fact {
                    return Err(invalid("conflicting fact identity"));
                }
            }
            if let Some(old) = subjects.insert((&fact.subject_id, &fact.predicate), &fact.quantity)
            {
                if old != &fact.quantity {
                    return Err(invalid("conflicting verified facts"));
                }
            }
        }
        let mut used = BTreeSet::new();
        let mut remaining = 256;
        let (value, unit) = eval(&rule.expression, &by_id, &mut used, &mut remaining, 0)?;
        Ok(RuleEvaluation {
            quantity: Quantity {
                value,
                unit: unit_text(&unit),
            },
            rule_id: rule.rule_id.clone(),
            rule_version: rule.rule_version.clone(),
            evaluator_version: RULE_EVALUATOR_VERSION.into(),
            knowledge_release: release.release_id.clone(),
            input_fact_ids: used,
        })
    }
}
fn eval(
    expr: &RuleExpr,
    facts: &BTreeMap<&str, &NumericFact>,
    used: &mut BTreeSet<String>,
    remaining: &mut usize,
    depth: usize,
) -> Result<(f64, Unit), PortError> {
    if depth > 32 || *remaining == 0 {
        return Err(invalid("rule complexity limit"));
    }
    *remaining -= 1;
    let result = match expr {
        RuleExpr::Constant { quantity } => (quantity.value, unit(&quantity.unit)?),
        RuleExpr::Fact { fact_id } => {
            let fact = facts
                .get(fact_id.as_str())
                .ok_or_else(|| invalid("verified release-pinned fact missing"))?;
            used.insert(fact_id.clone());
            (fact.quantity.value, unit(&fact.quantity.unit)?)
        }
        RuleExpr::Add { left, right }
        | RuleExpr::Subtract { left, right }
        | RuleExpr::Multiply { left, right }
        | RuleExpr::Divide { left, right }
        | RuleExpr::Min { left, right }
        | RuleExpr::Max { left, right } => {
            let (a, mut au) = eval(left, facts, used, remaining, depth + 1)?;
            let (b, bu) = eval(right, facts, used, remaining, depth + 1)?;
            match expr {
                RuleExpr::Multiply { .. } | RuleExpr::Divide { .. } => {
                    let divide = matches!(expr, RuleExpr::Divide { .. });
                    if divide && b == 0.0 {
                        return Err(invalid("division by zero"));
                    }
                    for (name, power) in bu {
                        *au.entry(name).or_default() += if divide { -power } else { power };
                    }
                    au.retain(|_, p| *p != 0);
                    if au.values().any(|p| p.abs() > 8) {
                        return Err(invalid("unit exponent limit"));
                    }
                    (if divide { a / b } else { a * b }, au)
                }
                _ => {
                    if au != bu {
                        return Err(invalid("unit mismatch"));
                    }
                    let value = match expr {
                        RuleExpr::Add { .. } => a + b,
                        RuleExpr::Subtract { .. } => a - b,
                        RuleExpr::Min { .. } => a.min(b),
                        RuleExpr::Max { .. } => a.max(b),
                        _ => unreachable!(),
                    };
                    (value, au)
                }
            }
        }
    };
    if !result.0.is_finite() {
        return Err(invalid("nonfinite rule result"));
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn data() -> (TypedRule, Vec<NumericFact>, CorpusRelease, Validity) {
        let source = DocumentRevision {
            source_id: "fixture".into(),
            logical_id: "stats".into(),
            revision: 1,
            content_hash: "fixture-hash".into(),
        };
        let validity = Validity {
            patch: "p1".into(),
            mode: "ranked".into(),
        };
        let fact = NumericFact {
            fact_id: "damage".into(),
            subject_id: "hero".into(),
            predicate: "damage".into(),
            quantity: Quantity {
                value: 40.0,
                unit: "damage".into(),
            },
            validity: validity.clone(),
            source: source.clone(),
            verified: true,
        };
        let rule = TypedRule {
            rule_id: "dps".into(),
            rule_version: "1".into(),
            validity: validity.clone(),
            source,
            verified: true,
            expression: RuleExpr::Divide {
                left: Box::new(RuleExpr::Fact {
                    fact_id: "damage".into(),
                }),
                right: Box::new(RuleExpr::Constant {
                    quantity: Quantity {
                        value: 5.0,
                        unit: "s".into(),
                    },
                }),
            },
        };
        let release = CorpusRelease {
            release_id: "r1".into(),
            knowledge_version: "v1".into(),
            patch: "p1".into(),
            created_at_epoch: 0,
            source_revisions: BTreeMap::from([(
                "fixture".into(),
                BTreeMap::from([("stats".into(), 1)]),
            )]),
        };
        (rule, vec![fact], release, validity)
    }
    #[test]
    fn deterministic_units_and_provenance() {
        let (r, f, s, v) = data();
        let a = CoreRuleEvaluator.evaluate(&r, &f, &s, &v).unwrap();
        assert_eq!(
            a.quantity,
            Quantity {
                value: 8.0,
                unit: "damage/s".into()
            }
        );
        assert_eq!(a.input_fact_ids, BTreeSet::from(["damage".into()]));
        assert_eq!(a, CoreRuleEvaluator.evaluate(&r, &f, &s, &v).unwrap());
    }
    #[test]
    fn wrong_patch_unverified_missing_and_conflicting_facts_fail() {
        let (r, mut f, s, mut v) = data();
        v.patch = "p2".into();
        assert!(CoreRuleEvaluator.evaluate(&r, &f, &s, &v).is_err());
        v.patch = "p1".into();
        f[0].verified = false;
        assert!(CoreRuleEvaluator.evaluate(&r, &f, &s, &v).is_err());
        f[0].verified = true;
        let mut conflict = f[0].clone();
        conflict.fact_id = "other".into();
        conflict.quantity.value = 50.0;
        f.push(conflict);
        assert!(CoreRuleEvaluator.evaluate(&r, &f, &s, &v).is_err());
    }
    #[test]
    fn zero_unit_mismatch_nonfinite_and_depth_fail() {
        let (mut r, f, s, v) = data();
        let c = |n, u: &str| {
            Box::new(RuleExpr::Constant {
                quantity: Quantity {
                    value: n,
                    unit: u.into(),
                },
            })
        };
        for expression in [
            RuleExpr::Divide {
                left: c(1.0, "damage"),
                right: c(0.0, "s"),
            },
            RuleExpr::Add {
                left: c(1.0, "s"),
                right: c(1.0, "damage"),
            },
            RuleExpr::Constant {
                quantity: Quantity {
                    value: f64::NAN,
                    unit: "1".into(),
                },
            },
        ] {
            r.expression = expression;
            assert!(CoreRuleEvaluator.evaluate(&r, &f, &s, &v).is_err());
        }
        r.expression = RuleExpr::Constant {
            quantity: Quantity {
                value: 1.0,
                unit: "1".into(),
            },
        };
        for _ in 0..40 {
            r.expression = RuleExpr::Add {
                left: Box::new(r.expression),
                right: c(1.0, "1"),
            };
        }
        assert!(CoreRuleEvaluator.evaluate(&r, &f, &s, &v).is_err());
    }
}
