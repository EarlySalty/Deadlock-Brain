use brain_contracts::{NumericClaim, ProviderAnswer, ToolResult};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct ValueLedger {
    values: BTreeMap<String, BTreeSet<String>>,
    labels: BTreeMap<String, BTreeSet<String>>,
    names: BTreeMap<String, BTreeSet<String>>,
    percents: BTreeMap<String, BTreeSet<String>>,
}

fn patch_labels(key: &str, value: &Value) -> Vec<String> {
    let Some(text) = value.as_str() else {
        return Vec::new();
    };
    let label = match key {
        "patch" => text.split_whitespace().next(),
        "released_at" => text.get(..10),
        _ => None,
    };
    let dates = (key == "released_at")
        .then(|| german_dates(text))
        .into_iter()
        .flatten();
    label
        .map(|label| [text, label])
        .into_iter()
        .flatten()
        .filter(|label| label.len() >= 8 && label.bytes().any(|byte| !byte.is_ascii_digit()))
        .map(str::to_owned)
        .chain(dates)
        .collect()
}

fn german_dates(text: &str) -> Vec<String> {
    let parts: Option<Vec<u32>> = text
        .get(..10)
        .and_then(|day| day.split('-').map(|part| part.parse().ok()).collect());
    let Some([year, month, day]) = parts.and_then(|parts| <[u32; 3]>::try_from(parts).ok()) else {
        return Vec::new();
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Vec::new();
    }
    let short = year % 100;
    vec![
        format!("{day:02}.{month:02}.{year}"),
        format!("{day}.{month}.{year}"),
        format!("{day:02}.{month:02}.{short:02}"),
        format!("{day}.{month}.{short:02}"),
        format!("{day:02}.{month:02}."),
        format!("{day}.{month}."),
    ]
}

fn complement(value: &str) -> Option<String> {
    let number = value.parse::<f64>().ok()?;
    (number > 0.0 && number < 100.0 && number.fract() == 0.0)
        .then(|| normalize(&format!("{}", 100.0 - number)))
        .flatten()
}

const NAME_FIELDS: [&str; 5] = ["line_text", "name", "label", "text", "title"];

fn is_name(token: &str) -> bool {
    let bytes = token.as_bytes();
    bytes
        .windows(2)
        .any(|pair| pair[0].is_ascii_alphabetic() && pair[1].is_ascii_digit())
        || matches!(bytes, [digit, b'A', b'P'] if digit.is_ascii_digit())
}

fn touches_number(bytes: &[u8], start: usize, end: usize) -> bool {
    let separator = |byte: &u8| matches!(byte, b'.' | b',' | b'-' | b'+');
    let before = start.checked_sub(1).and_then(|index| bytes.get(index));
    let signed = bytes[start].is_ascii_digit() && before.is_some_and(separator);
    let glued_before = before.is_some_and(separator)
        && start
            .checked_sub(2)
            .and_then(|index| bytes.get(index))
            .is_some_and(u8::is_ascii_digit);
    let glued_after =
        bytes.get(end).is_some_and(separator) && bytes.get(end + 1).is_some_and(u8::is_ascii_digit);
    signed || glued_before || glued_after
}

fn name_spans(text: &str) -> Vec<std::ops::Range<usize>> {
    let bytes = text.as_bytes();
    let mut start = None;
    let mut spans = Vec::new();
    for (index, character) in text.char_indices().chain([(text.len(), ' ')]) {
        match (start, character.is_alphanumeric()) {
            (None, true) => start = Some(index),
            (Some(from), false) => {
                if is_name(&text[from..index]) && !touches_number(bytes, from, index) {
                    spans.push(from..index);
                }
                start = None;
            }
            _ => {}
        }
    }
    spans
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

fn percents(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut offset = 0;
    number_spans(text)
        .into_iter()
        .filter_map(|(raw, value)| {
            let end = offset + text[offset..].find(raw)? + raw.len();
            offset = end;
            let next = end
                + bytes[end..]
                    .iter()
                    .take_while(|byte| **byte == b' ')
                    .count();
            (bytes.get(next) == Some(&b'%')).then_some(value)
        })
        .collect()
}

pub(super) fn normalized_set(values: &[String]) -> BTreeSet<String> {
    values
        .iter()
        .map(|value| normalize(value).unwrap_or_else(|| value.clone()))
        .collect()
}

fn numbers(text: &str) -> Vec<String> {
    number_spans(text)
        .into_iter()
        .map(|(_, value)| value)
        .collect()
}

fn number_spans(text: &str) -> Vec<(&str, String)> {
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
            result.push((&text[start..index], value));
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
                let returned = |id: &&str| allowed.iter().any(|known| known == id);
                let own = fields
                    .get("citation")
                    .or_else(|| fields.get("evidence_id"))
                    .and_then(Value::as_str);
                let citation = own.or(inherited).filter(returned);
                if let Some(id) = own.filter(returned) {
                    let labels = fields
                        .iter()
                        .flat_map(|(key, value)| patch_labels(key, value));
                    self.labels.entry(id.into()).or_default().extend(labels);
                }
                if let Some(id) = citation {
                    self.values.entry(id.into()).or_default();
                    let names = NAME_FIELDS
                        .iter()
                        .filter_map(|key| fields.get(*key).and_then(Value::as_str))
                        .flat_map(|text| {
                            name_spans(text)
                                .into_iter()
                                .map(|span| text[span].to_owned())
                        });
                    self.names.entry(id.into()).or_default().extend(names);
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
                    self.values.entry(id.into()).or_default().insert(number);
                }
            }
            Value::String(text) => {
                if let Some(id) = inherited {
                    self.values
                        .entry(id.into())
                        .or_default()
                        .extend(numbers(text));
                    self.percents
                        .entry(id.into())
                        .or_default()
                        .extend(percents(text));
                }
            }
            _ => {}
        }
    }

    pub(super) fn accepts(&self, answer: &ProviderAnswer) -> bool {
        answer
            .claims
            .iter()
            .all(|claim| self.claim_holds(answer, claim))
            && self.unsupported(answer).is_empty()
    }

    pub(super) fn unsupported(&self, answer: &ProviderAnswer) -> Vec<String> {
        let stated: BTreeSet<String> = numbers(&answer.text).into_iter().collect();
        let cited: BTreeSet<&String> = answer
            .cited_evidence_ids
            .iter()
            .filter_map(|id| self.values.get(id))
            .flatten()
            .collect();
        let complements: BTreeSet<String> = answer
            .cited_evidence_ids
            .iter()
            .filter_map(|id| self.percents.get(id))
            .flatten()
            .filter(|value| stated.contains(value.as_str()))
            .filter_map(|value| complement(value))
            .collect();
        let covered: BTreeSet<&String> = cited.into_iter().chain(&complements).collect();
        let mut labels: Vec<_> = answer
            .cited_evidence_ids
            .iter()
            .filter_map(|id| self.labels.get(id))
            .flatten()
            .collect();
        labels.sort_by_key(|label| std::cmp::Reverse(label.len()));
        let text = labels.into_iter().fold(answer.text.clone(), |text, label| {
            text.replace(label.as_str(), " ")
        });
        let names: BTreeSet<&str> = answer
            .cited_evidence_ids
            .iter()
            .filter_map(|id| self.names.get(id))
            .flatten()
            .map(String::as_str)
            .collect();
        let text = name_spans(&text)
            .into_iter()
            .rev()
            .filter(|span| names.contains(&text[span.clone()]))
            .fold(text.clone(), |mut text, span| {
                text.replace_range(span, " ");
                text
            });
        let mut missing: Vec<String> = Vec::new();
        for (raw, value) in number_spans(&text) {
            if !covered.contains(&value) && !missing.iter().any(|seen| seen == raw) {
                missing.push(raw.to_string());
            }
        }
        missing
    }

    fn claim_holds(&self, answer: &ProviderAnswer, claim: &NumericClaim) -> bool {
        normalize(&claim.value.to_string()).is_some_and(|value| {
            answer.cited_evidence_ids.contains(&claim.citation_id)
                && (self
                    .values
                    .get(&claim.citation_id)
                    .is_some_and(|values| values.contains(&value))
                    || self.percents.get(&claim.citation_id).is_some_and(|values| {
                        values
                            .iter()
                            .any(|known| complement(known).as_ref() == Some(&value))
                    }))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{ToolName, Usage};
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
    fn every_text_number_must_come_from_a_cited_row() {
        let mut answer = answer(780, "health");
        answer.claims.clear();
        answer.text.push_str(" Schaden: 12.");
        assert!(ledger().accepts(&answer));
        answer.text = "Baba hat 781 Leben.".into();
        assert!(!ledger().accepts(&answer));
        answer.text = "Baba hat 780 Leben. Schaden: 12.".into();
        answer.cited_evidence_ids = vec!["health".into()];
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
        assert!(ledger.accepts(&named("Am 15.10.2026: von 50 auf 75.")));
        assert!(!ledger.accepts(&named("Am 16.10.2026: von 50 auf 75.")));
        let mut uncited = named("Patch 09-29-2026: von 50 auf 75.");
        uncited.cited_evidence_ids = vec!["other".into()];
        uncited.claims.clear();
        assert!(!ledger.accepts(&uncited));
        let mut quoted = named("Bonus increased from +50% to +76%");
        quoted.claims.clear();
        assert!(!ledger.accepts(&quoted));
        let mut nested = ValueLedger::default();
        nested.insert(&ToolResult { call_id: "patch".into(), name: ToolName::PatchHistory,
            result: json!({"citation":"line","line_text":"Patch notes","unreturned":{"patch":"10-01-2026 (Post 1)"}}),
            evidence_ids: vec!["line".into()], is_error: false });
        let mut inherited = named("Patch 10-01-2026.");
        inherited.claims.clear();
        assert!(!nested.accepts(&inherited));
    }

    #[test]
    fn returned_names_with_digits_need_no_claim() {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult { call_id: "patch".into(), name: ToolName::PatchHistory,
            result: json!({"changes":[{"citation":"line","line_text":"- Deadman Danny: Chimera Mk1 - Slow reduced by 15%, Close the Deal 1AP from +50 to +90, 25s"}]}),
            evidence_ids: vec!["line".into()], is_error: false });
        let named = |text: &str, values: &[i64]| ProviderAnswer {
            text: text.into(),
            cited_evidence_ids: vec!["line".into()],
            claims: values
                .iter()
                .map(|value| NumericClaim {
                    citation_id: "line".into(),
                    value: (*value).into(),
                    unit: None,
                })
                .collect(),
            usage: Usage::default(),
        };
        assert!(ledger.accepts(&named("Chimera Mk1: Slow um 15% verringert.", &[15])));
        assert!(ledger.accepts(&named("Close the Deal: 1AP von +50 auf +90.", &[50, 90])));
        assert!(!ledger.accepts(&named("Chimera Mk2: Slow um 15% verringert.", &[15])));
        assert!(!ledger.accepts(&named("Chimera Mk1: Slow um 16% verringert.", &[15])));
        assert!(ledger.accepts(&named("Abklingzeit 25s.", &[])));
        assert!(!ledger.accepts(&named("Abklingzeit 26s.", &[])));
        assert!(!ledger.accepts(&named("Chimera Mk11: Slow um 15% verringert.", &[15])));
        let mut uncited = named("Chimera Mk1.", &[]);
        uncited.cited_evidence_ids = vec!["other".into()];
        assert!(!ledger.accepts(&uncited));
    }

    #[test]
    fn names_glued_to_numbers_stay_numbers() {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult { call_id: "patch".into(), name: ToolName::PatchHistory,
            result: json!({"changes":[{"citation":"line","line_text":"Spirit x1.5, Scale x2,5, Burst 1.5AP, Drain -50HP, Lvl1-3, Shield 300HP, Live 2026-10-15T03:12","released_at":"2026-10-15T03:12:00+00:00"}]}),
            evidence_ids: vec!["line".into()], is_error: false });
        let named = |text: &str, values: &[f64]| ProviderAnswer {
            text: text.into(),
            cited_evidence_ids: vec!["line".into()],
            claims: values
                .iter()
                .filter_map(|value| serde_json::Number::from_f64(*value))
                .map(|value| NumericClaim {
                    citation_id: "line".into(),
                    value,
                    unit: None,
                })
                .collect(),
            usage: Usage::default(),
        };
        assert!(!ledger.accepts(&named("Skalierung x2", &[])));
        assert!(!ledger.accepts(&named("Burst 5AP", &[])));
        assert!(!ledger.accepts(&named("Drain +50HP", &[])));
        assert!(!ledger.accepts(&named("Drain 50HP", &[])));
        assert!(!ledger.accepts(&named("Schild 301HP", &[])));
        assert!(!ledger.accepts(&named("Schild 300HP", &[301.0])));
        assert!(ledger.accepts(&named("Schild 300HP", &[])));
        assert!(ledger.accepts(&named("Spirit x1.5", &[1.5])));
        assert!(ledger.accepts(&named("Scale x2,5", &[2.5])));
        assert!(ledger.accepts(&named("Lvl1-3", &[1.0, 3.0])));
        assert!(ledger.accepts(&named("Drain -50HP", &[-50.0])));
    }

    #[test]
    fn patch_split_accepts_dates_and_percentage_complements_only() {
        let mut ledger = ValueLedger::default();
        ledger.insert(&ToolResult { call_id: "patch".into(), name: ToolName::PatchHistory,
            result: json!({"changes":[{"citation":"orb","patch":"11-21-2025 Update","released_at":"2025-11-21T22:53:12+00:00",
                "line_text":"Amount of souls in the flying orb that can be secured/denied reduced from 50% to 40%"}]}),
            evidence_ids: vec!["orb".into()], is_error: false });
        let cited = |text: &str| ProviderAnswer {
            text: text.into(),
            cited_evidence_ids: vec!["orb".into()],
            claims: Vec::new(),
            usage: Usage::default(),
        };
        assert!(ledger.accepts(&cited(
            "Seit dem 21.11.2025 sind 40% im fliegenden Orb und 60% fallen auf den Boden, vorher 50/50."
        )));
        assert!(ledger.accepts(&cited("Ja, 60/40 stimmt: seit 21.11. sind es 40 zu 60.")));
        assert!(!ledger.accepts(&cited("Seit dem 21.11.2025 sind es 45% im Orb.")));
        assert!(!ledger.accepts(&cited("Seit dem 22.11.2025 sind es 40% im Orb.")));
        assert!(!ValueLedger::default().accepts(&cited("Es sind 60% im Orb.")));
        let mut damage = ValueLedger::default();
        damage.insert(&ToolResult { call_id: "dmg".into(), name: ToolName::PatchHistory,
            result: json!({"changes":[{"citation":"orb","line_text":"Bullet damage increased from 10 to 12"}]}),
            evidence_ids: vec!["orb".into()], is_error: false });
        assert!(damage.accepts(&cited("Der Schaden stieg von 10 auf 12.")));
        assert!(!damage.accepts(&cited("12 Schaden und 88 Leben.")));
        assert!(!damage.accepts(&cited("Baba hat 999 Leben.")));
        assert_eq!(
            normalized_set(&["45".into(), "40".into()]),
            normalized_set(&["40.0".into(), "45".into()])
        );
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
