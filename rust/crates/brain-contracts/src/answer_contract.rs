use std::collections::BTreeSet;

use crate::{tools::ToolConversation, Evidence, Query};

pub const NO_AUTHORIZED_VALUE: &str =
    "Dafür habe ich keinen gesicherten Wert aus den freigegebenen Spieldaten. Zahlen rate ich nicht.";

const MAX_EXPONENT: i64 = 40;

struct Number {
    text: String,
    sign: Option<char>,
    mantissa: String,
    exponent: i64,
}

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
        .flat_map(|number| authorized_keys(&number))
        .collect();
    numbers(answer, true)
        .into_iter()
        .filter(|number| {
            !values(number).iter().any(|(integer, fraction)| {
                authorized.contains(&signed(number.sign, &canonical(integer, fraction)))
            })
        })
        .map(|number| number.text)
        .collect()
}

fn authorized_keys(number: &Number) -> Vec<String> {
    let explicit = if number.sign == Some('-') { '-' } else { '+' };
    values(number)
        .iter()
        .flat_map(|(integer, fraction)| {
            (0..=fraction.len()).map(|precision| rounded(integer, fraction, precision))
        })
        .flat_map(|value| [signed(Some(explicit), &value), value])
        .collect()
}

fn signed(sign: Option<char>, value: &str) -> String {
    sign.map_or_else(|| value.to_owned(), |sign| format!("{sign}{value}"))
}

fn values(number: &Number) -> Vec<(String, String)> {
    readings(&number.mantissa)
        .into_iter()
        .filter_map(|(integer, fraction)| shifted(&integer, &fraction, number.exponent))
        .collect()
}

fn word_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn numbers(text: &str, skip_list_markers: bool) -> Vec<Number> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        if !chars[index].is_ascii_digit() || (index > 0 && word_char(chars[index - 1])) {
            index += 1;
            continue;
        }
        let digits_start = index;
        let start = if index > 0
            && matches!(chars[index - 1], '.' | ',')
            && (index < 2 || !word_char(chars[index - 2]))
        {
            index - 1
        } else {
            index
        };
        while index < chars.len()
            && (chars[index].is_ascii_digit()
                || (matches!(chars[index], '.' | ',')
                    && chars.get(index + 1).is_some_and(char::is_ascii_digit)))
        {
            index += 1;
        }
        let mantissa: String = chars[start..index].iter().collect();
        let (exponent, end) = exponent(&chars, index);
        let sign = start
            .checked_sub(1)
            .filter(|&position| position == 0 || !word_char(chars[position - 1]))
            .and_then(|position| match chars[position] {
                '-' | '−' => Some('-'),
                '+' => Some('+'),
                _ => None,
            });
        let text_start = start - usize::from(sign.is_some());
        let marker = sign.is_none()
            && start == digits_start
            && end == index
            && list_marker(&chars, start, end);
        if !(skip_list_markers && marker) {
            found.push(Number {
                text: chars[text_start..end].iter().collect(),
                sign,
                mantissa,
                exponent,
            });
        }
        index = end;
    }
    found
}

fn exponent(chars: &[char], end: usize) -> (i64, usize) {
    if !matches!(chars.get(end), Some('e' | 'E')) {
        return (0, end);
    }
    let digits_start = end + 1 + usize::from(matches!(chars.get(end + 1), Some('+' | '-')));
    let digits_end = chars[digits_start.min(chars.len())..]
        .iter()
        .position(|character| !character.is_ascii_digit())
        .map_or(chars.len(), |offset| digits_start + offset);
    if digits_end <= digits_start {
        return (0, end);
    }
    let value: String = chars[end + 1..digits_end].iter().collect();
    (value.parse().unwrap_or(i64::MAX), digits_end)
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

fn readings(mantissa: &str) -> Vec<(String, String)> {
    let groups: Vec<&str> = mantissa.split(['.', ',']).collect();
    let thousands = |groups: &[&str]| {
        groups.split_first().is_some_and(|(first, rest)| {
            (1..=3).contains(&first.len()) && rest.iter().all(|group| group.len() == 3)
        })
    };
    let mut readings = Vec::new();
    if groups.len() == 1 || thousands(&groups) {
        readings.push((groups.concat(), String::new()));
    }
    if let Some((fraction, integer)) = groups.split_last().filter(|_| groups.len() > 1) {
        if integer.len() == 1 || thousands(integer) {
            readings.push((integer.concat(), (*fraction).to_owned()));
        }
    }
    readings
}

fn shifted(integer: &str, fraction: &str, exponent: i64) -> Option<(String, String)> {
    if exponent.abs() > MAX_EXPONENT {
        return None;
    }
    let digits = format!("{integer}{fraction}");
    let point = integer.len() as i64 + exponent;
    let left = usize::try_from(-point).unwrap_or(0);
    let right = usize::try_from(point - digits.len() as i64).unwrap_or(0);
    let digits = format!("{}{digits}{}", "0".repeat(left), "0".repeat(right));
    let point = usize::try_from(point).unwrap_or(0);
    Some((digits[..point].to_owned(), digits[point..].to_owned()))
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
    fn separators_signs_and_exponents_keep_their_numeric_meaning() {
        assert_eq!(unbound_values("Abklingzeit 125 s.", ["12.5"]), ["125"]);
        assert_eq!(unbound_values("Abklingzeit 1.25 s.", ["125"]), ["1.25"]);
        assert_eq!(unbound_values("Abklingzeit 1,250 s.", ["12.5"]), ["1,250"]);
        assert!(unbound_values("Verlangsamt um 30 % oder -30 %.", ["-30"]).is_empty());
        assert_eq!(unbound_values("Bonus +30 %.", ["-30"]), ["+30"]);
        assert_eq!(unbound_values("Malus −30 %.", ["30"]), ["−30"]);
        assert!(unbound_values("Bonus +30 % von 10-20 s.", ["30", "10", "20"]).is_empty());
        assert_eq!(unbound_values("Faktor .5", ["5"]), [".5"]);
        assert!(unbound_values("Faktor 0,5", [".5"]).is_empty());
        assert_eq!(unbound_values("Wert 1e3", ["1"]), ["1e3"]);
        assert!(unbound_values("Wert 1.500 oder 1500", ["1.5e3"]).is_empty());
        assert!(unbound_values("Wert 0,0000001", ["1e-7"]).is_empty());
        assert_eq!(unbound_values("Wert 1e999", ["1e999"]), ["1e999"]);
    }

    #[test]
    fn only_evidence_and_tool_results_back_values() {
        let query: Query = serde_json::from_value(serde_json::json!({
            "request_id":"r","conversation_id":"c",
            "text":"Kommt die Urne alle 7 Minuten und kostet 6200 Seelen?"
        }))
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
            unbound_provider_values(
                "Ja, alle 7 Minuten, und sie kostet 6200 Seelen.",
                &query,
                &[],
                &conversation
            ),
            ["7", "6200"]
        );
    }
}
