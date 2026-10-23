use brain_contracts::{ProviderAnswer, ToolResult};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct ValueLedger(BTreeMap<String, BTreeSet<String>>, BTreeSet<String>);

fn patch_labels(key: &str, value: &Value) -> Vec<String> {
    let Some(text) = value.as_str() else {
        return Vec::new();
    };
    let label = match key {
        "patch" => text.split_whitespace().next(),
        "released_at" => text.get(..10),
        _ => None,
    };
    [Some(text), label]
        .into_iter()
        .flatten()
        .filter(|label| label.len() >= 8 && label.bytes().any(|byte| !byte.is_ascii_digit()))
        .map(str::to_owned)
        .collect()
}

fn normalize(text: &str) -> Option<String> {
    let text = text.replace(',', ".");
    let (mantissa, exponent) = text
        .split_once(['e', 'E'])
        .map_or((text.as_str(), Some(0)), |(value, exponent)| {
            (value, exponent.parse::<i32>().ok())
        });
    let negative = mantissa.starts_with('-');
    let mantissa = mantissa.trim_start_matches(['-', '+']);
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{whole}{fraction}");
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let scale = exponent?.checked_sub(i32::try_from(fraction.len()).ok()?)?;
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return Some("0".into());
    }
    let trimmed = digits.trim_end_matches('0');
    let scale = scale.checked_add(i32::try_from(digits.len() - trimmed.len()).ok()?)?;
    Some(format!(
        "{}{trimmed}e{scale}",
        if negative { "-" } else { "" }
    ))
}

fn follows_word(bytes: &[u8], index: usize) -> bool {
    index > 0 && bytes[index - 1].is_ascii_alphanumeric()
}

fn numbers(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut result = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let start = index;
        if matches!(bytes[index], b'-' | b'+') && !follows_word(bytes, index) {
            index += 1;
        }
        let digits = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index == digits {
            index = start + 1;
            continue;
        }
        if index + 1 < bytes.len()
            && matches!(bytes[index], b'.' | b',')
            && bytes[index + 1].is_ascii_digit()
        {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
            let exponent = index;
            index += 1;
            if index < bytes.len() && matches!(bytes[index], b'-' | b'+') {
                index += 1;
            }
            let digits = index;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
            if index == digits {
                index = exponent;
            }
        }
        if let Some(value) = normalize(&text[start..index]) {
            result.push(value);
        }
    }
    result
}

impl ValueLedger {
    pub(super) fn insert(&mut self, result: &ToolResult) {
        if !result.is_error {
            self.row(&result.result, None, &result.evidence_ids);
        }
    }

    fn row(&mut self, value: &Value, inherited: Option<&str>, allowed: &[String]) {
        match value {
            Value::Object(fields) => {
                let citation = fields
                    .get("citation")
                    .or_else(|| fields.get("evidence_id"))
                    .and_then(Value::as_str)
                    .or(inherited)
                    .filter(|id| allowed.iter().any(|known| known == id));
                if let Some(id) = citation {
                    self.0.entry(id.into()).or_default();
                    for (key, value) in fields {
                        self.1.extend(patch_labels(key, value));
                    }
                }
                for (key, value) in fields {
                    if !matches!(
                        key.as_str(),
                        "citation"
                            | "evidence_id"
                            | "source_revision"
                            | "patch"
                            | "entity_id"
                            | "score"
                    ) {
                        self.row(value, citation, allowed);
                    }
                }
            }
            Value::Array(values) => {
                for value in values {
                    self.row(value, inherited, allowed);
                }
            }
            Value::Number(number) => {
                if let (Some(id), Some(number)) = (inherited, normalize(&number.to_string())) {
                    self.0.entry(id.into()).or_default().insert(number);
                }
            }
            Value::String(text) => {
                if let Some(id) = inherited {
                    self.0.entry(id.into()).or_default().extend(numbers(text));
                }
            }
            _ => {}
        }
    }

    pub(super) fn accepts(&self, answer: &ProviderAnswer) -> bool {
        let mut covered = BTreeSet::new();
        for claim in &answer.claims {
            let Some(value) = normalize(&claim.value.to_string()) else {
                return false;
            };
            if !answer.cited_evidence_ids.contains(&claim.citation_id)
                || !self
                    .0
                    .get(&claim.citation_id)
                    .is_some_and(|values| values.contains(&value))
            {
                return false;
            }
            covered.insert(value);
        }
        let mut labels: Vec<_> = self.1.iter().collect();
        labels.sort_by_key(|label| std::cmp::Reverse(label.len()));
        let text = labels.into_iter().fold(answer.text.clone(), |text, label| {
            text.replace(label.as_str(), " ")
        });
        numbers(&text).iter().all(|value| covered.contains(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{NumericClaim, ToolName, Usage};
    use serde_json::json;

    fn ledger() -> ValueLedger {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult {
            call_id: "hero".into(), name: ToolName::GetHero,
            result: json!({"stats":{"facts":[{"citation":"health","value":780.0},{"citation":"damage","value":12}]}}),
            evidence_ids: vec!["health".into(), "damage".into()], is_error: false,
        });
        ledger
    }

    fn answer(value: i64, citation: &str) -> ProviderAnswer {
        ProviderAnswer {
            text: "Baba hat 780 Leben.".into(),
            cited_evidence_ids: vec!["health".into(), "damage".into()],
            claims: vec![NumericClaim {
                citation_id: citation.into(),
                value: value.into(),
                unit: None,
            }],
            usage: Usage::default(),
        }
    }

    #[test]
    fn matching_claim_passes_with_format_only_normalization() {
        assert!(ledger().accepts(&answer(780, "health")));
        assert_eq!(normalize("1.0"), normalize("1"));
        assert_eq!(normalize("1,25"), normalize("1.25"));
        assert_eq!(normalize("-0.000"), normalize("0"));
        assert_eq!(normalize("1e3"), normalize("1000"));
        assert_ne!(normalize("1.00000001"), normalize("1"));
    }

    #[test]
    fn hyphen_after_digit_or_letter_is_not_a_sign() {
        assert_eq!(
            numbers("10-20 Sekunden"),
            [normalize("10"), normalize("20")].map(Option::unwrap)
        );
        assert_eq!(numbers("Stufe-3"), [normalize("3")].map(Option::unwrap));
        assert_eq!(numbers("Wert -5"), [normalize("-5")].map(Option::unwrap));
    }

    #[test]
    fn wrong_value_wrong_citation_and_unknown_citation_fail() {
        assert!(!ledger().accepts(&answer(781, "health")));
        assert!(!ledger().accepts(&answer(780, "damage")));
        assert!(!ledger().accepts(&answer(780, "missing")));
    }

    #[test]
    fn every_text_number_requires_a_claim() {
        let mut answer = answer(780, "health");
        answer.text.push_str(" Schaden: 12.");
        assert!(!ledger().accepts(&answer));
        answer.claims.clear();
        assert!(!ledger().accepts(&answer));
        answer.text = "Baba ist ein Held.".into();
        assert!(ledger().accepts(&answer));
    }

    #[test]
    fn returned_patch_identity_may_be_named_without_claims() {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult { call_id: "patch".into(), name: ToolName::PatchHistory,
            result: json!({"changes":[{"citation":"line","line_text":"Bonus increased from +50% to +75%","patch":"09-29-2026 (Post 330973, 2026-10-15)","released_at":"2026-10-15T03:12:00+00:00"}],"unreturned":{"patch":"10-01-2026 (Post 1)"}}),
            evidence_ids: vec!["line".into()], is_error: false });
        let named = |text: &str| ProviderAnswer {
            text: text.into(),
            cited_evidence_ids: vec!["line".into()],
            claims: [50, 75]
                .map(|value| NumericClaim {
                    citation_id: "line".into(),
                    value: value.into(),
                    unit: None,
                })
                .into(),
            usage: Usage::default(),
        };
        assert!(ledger.accepts(&named("Patch 09-29-2026 vom 2026-10-15: von 50 auf 75.")));
        assert!(ledger.accepts(&named("09-29-2026 (Post 330973, 2026-10-15): 50 auf 75")));
        assert!(!ledger.accepts(&named("Patch 09-29-2026: von 50 auf 76.")));
        assert!(!ledger.accepts(&named("Patch 10-01-2026: von 50 auf 75.")));
        assert!(!ledger.accepts(&named("Am 15.10.2026: von 50 auf 75.")));
    }

    #[test]
    fn metadata_and_unreturned_receipts_do_not_supply_values() {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult { call_id: "hero".into(), name: ToolName::GetHero,
            result: json!({"citation":"health", "entity_id":780, "source_revision":"780", "nested":{"citation":"foreign","value":780}}),
            evidence_ids: vec!["health".into()], is_error: false });
        assert!(!ledger.accepts(&answer(780, "health")));
    }
}
