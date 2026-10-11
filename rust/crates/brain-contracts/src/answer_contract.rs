use std::collections::BTreeSet;

use crate::{tools::ToolConversation, Evidence, Query};

pub const NO_AUTHORIZED_VALUE: &str =
    "Dafür habe ich keinen gesicherten Wert aus den freigegebenen Spieldaten. Zahlen rate ich nicht.";

pub fn unbound_provider_values(
    answer: &str,
    query: &Query,
    evidence: &[Evidence],
    conversation: &ToolConversation,
) -> Vec<String> {
    let originals = crate::provider_input::provider_data_texts(query, evidence, conversation);
    unbound_values(answer, originals.iter().map(String::as_str))
}

pub fn unbound_values<'a>(
    answer: &str,
    originals: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let authorized: BTreeSet<String> = originals
        .into_iter()
        .flat_map(|original| numbers(original, false))
        .flat_map(|token| readings(&token))
        .flat_map(|(integer, fraction)| {
            (0..=fraction.len())
                .map(|precision| rounded(&integer, &fraction, precision))
                .collect::<Vec<_>>()
        })
        .collect();
    numbers(answer, true)
        .into_iter()
        .filter(|token| {
            readings(token)
                .iter()
                .all(|(integer, fraction)| !authorized.contains(&canonical(integer, fraction)))
        })
        .collect()
}

fn numbers(text: &str, skip_list_markers: bool) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let identifier =
            index > 0 && (chars[index - 1].is_alphanumeric() || chars[index - 1] == '_');
        if !chars[index].is_ascii_digit() || identifier {
            index += 1;
            continue;
        }
        let start = index;
        while index < chars.len()
            && (chars[index].is_ascii_digit()
                || (matches!(chars[index], '.' | ',')
                    && chars.get(index + 1).is_some_and(char::is_ascii_digit)))
        {
            index += 1;
        }
        if !(skip_list_markers && list_marker(&chars, start, index)) {
            found.push(chars[start..index].iter().collect());
        }
    }
    found
}

fn list_marker(chars: &[char], start: usize, end: usize) -> bool {
    let line_start = chars[..start]
        .iter()
        .rposition(|character| *character == '\n')
        .map_or(0, |position| position + 1);
    chars[line_start..start]
        .iter()
        .all(|character| character.is_whitespace() || matches!(character, '*' | '#' | '>'))
        && matches!(chars.get(end), Some('.' | ')'))
        && chars.get(end + 1).is_none_or(|next| next.is_whitespace())
}

fn readings(token: &str) -> Vec<(String, String)> {
    let groups: Vec<&str> = token.split(['.', ',']).collect();
    let mut readings = vec![(groups.concat(), String::new())];
    if let Some((fraction, integer)) = groups.split_last().filter(|_| groups.len() > 1) {
        readings.push((integer.concat(), (*fraction).to_owned()));
    }
    readings
}

fn rounded(integer: &str, fraction: &str, precision: usize) -> String {
    let mut digits: Vec<char> = integer
        .chars()
        .chain(fraction.chars().take(precision))
        .collect();
    if fraction
        .chars()
        .nth(precision)
        .is_some_and(|next| next >= '5')
    {
        let mut position = digits.len();
        loop {
            if position == 0 {
                digits.insert(0, '1');
                break;
            }
            position -= 1;
            if digits[position] == '9' {
                digits[position] = '0';
            } else {
                digits[position] = char::from(digits[position] as u8 + 1);
                break;
            }
        }
    }
    let split = digits.len() - precision;
    canonical(
        &digits[..split].iter().collect::<String>(),
        &digits[split..].iter().collect::<String>(),
    )
}

fn canonical(integer: &str, fraction: &str) -> String {
    let integer = integer.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');
    let integer = if integer.is_empty() { "0" } else { integer };
    if fraction.is_empty() {
        integer.to_owned()
    } else {
        format!("{integer}.{fraction}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_need_a_matching_number_in_an_authorized_original() {
        let original = r#"{"name":"Mystic Burst","m_flBonusSpiritDamage":"1.500","m_flCooldown":"12.50","patch":"2026-10-02"}"#;
        for answer in [
            "Mystic Burst macht 1500 Bonus-Schaden.",
            "Mystic Burst macht 1.500 Bonus-Schaden bei 12,5 s Abklingzeit.",
            "Stand: Patch vom 2. Oktober 2026.",
            "Keine Zahl, nur eine Erklärung.",
            "1. Kauf zuerst Mystic Burst\n2) danach etwas anderes\n- T3 ist teuer",
        ] {
            assert!(unbound_values(answer, [original]).is_empty(), "{answer}");
        }
        assert_eq!(
            unbound_values(
                "Mystic Burst macht 80 Schaden und lädt in 12,5 s.",
                [original]
            ),
            ["80"]
        );
        assert_eq!(unbound_values("Kostet 6200 Seelen.", []), ["6200"]);
        let observed = r#"{"matches":1834,"winrate_percent":52.3456,"pick":99.96}"#;
        assert!(unbound_values(
            "In 1.834 Matches 52,3 % Siegquote, gerundet 52 %, Pickrate 100 %.",
            [observed]
        )
        .is_empty());
        assert_eq!(
            unbound_values("Siegquote 52,4 % oder 53 %.", [observed]),
            ["52,4", "53"]
        );
    }

    #[test]
    fn timer_seconds_also_back_their_minute_reading() {
        let query: Query = serde_json::from_value(
            serde_json::json!({"request_id":"r","conversation_id":"c","text":"Wann kommt die Urne?"}),
        )
        .expect("query");
        let conversation = ToolConversation {
            messages: vec![crate::tools::ToolMessage::ToolResults {
                results: vec![crate::tools::ToolResult {
                    call_id: "timer-call".into(),
                    name: crate::tools::ToolName::DeadlockData,
                    result: serde_json::json!({"timers":[{"objective":"urn",
                        "first_spawn_seconds":600,"first_spawn_clock":"10:00",
                        "respawn_interval_seconds":300}]}),
                    evidence_ids: Vec::new(),
                    is_error: false,
                }],
            }],
        };
        assert!(unbound_provider_values(
            "Die Urne kommt bei 10:00 und danach alle 5 Minuten, also alle 300 s.",
            &query,
            &[],
            &conversation,
        )
        .is_empty());
        assert_eq!(
            unbound_provider_values("Die Urne kommt alle 4 Minuten.", &query, &[], &conversation),
            ["4"]
        );
    }
}
