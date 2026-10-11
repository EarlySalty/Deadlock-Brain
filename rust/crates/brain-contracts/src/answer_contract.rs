use std::collections::BTreeSet;

use crate::{tools::ToolConversation, Evidence, Query};

pub const NO_AUTHORIZED_VALUE: &str =
    "Dafür habe ich keinen gesicherten Wert aus den freigegebenen Spieldaten. Zahlen rate ich nicht.";

const MAX_EXPONENT: u64 = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Original {
    Text(String),
    Number(String),
}

const SEQUENCE_WORDS: &[&str] = &[
    "minute",
    "minuten",
    "min",
    "sekunde",
    "sekunden",
    "sek",
    "sec",
    "second",
    "seconds",
    "stunde",
    "stunden",
    "hour",
    "stufe",
    "level",
    "lvl",
    "rang",
    "rank",
    "platz",
    "welle",
    "wave",
    "runde",
    "round",
    "patch",
    "tag",
    "day",
    "woche",
    "week",
    "monat",
    "month",
    "januar",
    "january",
    "februar",
    "february",
    "märz",
    "march",
    "april",
    "mai",
    "may",
    "juni",
    "june",
    "juli",
    "july",
    "august",
    "september",
    "oktober",
    "october",
    "november",
    "dezember",
    "december",
];

struct Number {
    text: String,
    raw: String,
    sign: Option<char>,
    mantissa: String,
    exponent: i64,
    typed: bool,
    marker: Option<(u64, bool)>,
}

pub fn unbound_provider_values(
    answer: &str,
    query: &Query,
    evidence: &[Evidence],
    conversation: &ToolConversation,
) -> Vec<String> {
    let originals = crate::provider_input::provider_data_texts(query, evidence, conversation);
    unbound_values(answer, &originals)
}

pub fn unbound_values(answer: &str, originals: &[Original]) -> Vec<String> {
    let authorized: BTreeSet<String> = originals
        .iter()
        .flat_map(|original| match original {
            Original::Text(text) => numbers(text, false, false),
            Original::Number(text) => numbers(text, true, false),
        })
        .flat_map(|number| authorized_keys(&number))
        .collect();
    without_list_markers(numbers(answer, false, true))
        .into_iter()
        .filter(|number| {
            !values(number)
                .iter()
                .map(|(integer, fraction)| canonical(integer, fraction))
                .chain([raw_key(number)])
                .any(|value| authorized.contains(&signed(number.sign, &value)))
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
        .chain([raw_key(number)])
        .flat_map(|value| [signed(Some(explicit), &value), value])
        .collect()
}

fn raw_key(number: &Number) -> String {
    format!("raw:{}", number.raw)
}

fn without_list_markers(numbers: Vec<Number>) -> Vec<Number> {
    let mut structural = vec![false; numbers.len()];
    let mut run: Vec<usize> = Vec::new();
    let mut close = |run: &mut Vec<usize>| {
        if run.len() >= 2
            && run
                .iter()
                .all(|&index| numbers[index].marker.is_some_and(|(_, unit)| !unit))
        {
            run.iter().for_each(|&index| structural[index] = true);
        }
        run.clear();
    };
    for (index, number) in numbers.iter().enumerate() {
        let Some((value, _)) = number.marker else {
            continue;
        };
        let continues = run
            .last()
            .and_then(|&last| numbers[last].marker)
            .is_some_and(|(previous, _)| value == previous + 1);
        if !continues {
            close(&mut run);
        }
        if continues || value == 1 {
            run.push(index);
        }
    }
    close(&mut run);
    numbers
        .into_iter()
        .zip(structural)
        .filter_map(|(number, structural)| (!structural).then_some(number))
        .collect()
}

fn signed(sign: Option<char>, value: &str) -> String {
    sign.map_or_else(|| value.to_owned(), |sign| format!("{sign}{value}"))
}

fn values(number: &Number) -> Vec<(String, String)> {
    readings(&number.mantissa, number.typed)
        .into_iter()
        .filter_map(|(integer, fraction)| shifted(&integer, &fraction, number.exponent))
        .collect()
}

fn word_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn numbers(text: &str, typed: bool, skip_list_markers: bool) -> Vec<Number> {
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
        let marker = (skip_list_markers
            && sign.is_none()
            && start == digits_start
            && end == index
            && list_marker(&chars, start, end))
        .then(|| mantissa.parse::<u64>().ok())
        .flatten()
        .map(|value| (value, sequence_word(&chars, end + 1)));
        found.push(Number {
            text: chars[text_start..end].iter().collect(),
            raw: chars[start..end].iter().collect(),
            sign,
            mantissa,
            exponent,
            typed,
            marker,
        });
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
    let line_end = chars[end..]
        .iter()
        .position(|character| *character == '\n')
        .map_or(chars.len(), |offset| end + offset);
    chars[line_start..start]
        .iter()
        .all(|character| character.is_whitespace())
        && matches!(chars.get(end), Some('.' | ')'))
        && chars.get(end + 1).is_some_and(|next| next.is_whitespace())
        && chars[(end + 1).min(line_end)..line_end]
            .iter()
            .any(|character| !character.is_whitespace())
}

fn sequence_word(chars: &[char], from: usize) -> bool {
    let word: String = chars[from.min(chars.len())..]
        .iter()
        .skip_while(|character| character.is_whitespace())
        .take_while(|character| character.is_alphabetic())
        .flat_map(|character| character.to_lowercase())
        .collect();
    SEQUENCE_WORDS.contains(&word.as_str())
}

fn readings(mantissa: &str, typed: bool) -> Vec<(String, String)> {
    let groups: Vec<&str> = mantissa.split(['.', ',']).collect();
    let separators: Vec<char> = mantissa
        .chars()
        .filter(|character| matches!(character, '.' | ','))
        .collect();
    let (Some((fraction, integer)), Some((decimal, thousands))) =
        (groups.split_last(), separators.split_last())
    else {
        return vec![(mantissa.to_owned(), String::new())];
    };
    if typed {
        return if thousands.is_empty() && *decimal == '.' {
            vec![(integer.concat(), (*fraction).to_owned())]
        } else {
            Vec::new()
        };
    }
    let grouped = |groups: &[&str]| {
        groups.split_first().is_some_and(|(first, rest)| {
            (1..=3).contains(&first.len()) && rest.iter().all(|group| group.len() == 3)
        })
    };
    let mut readings = Vec::new();
    if separators.iter().all(|separator| separator == decimal) && grouped(&groups) {
        readings.push((groups.concat(), String::new()));
    }
    let consistent = thousands
        .iter()
        .all(|separator| separator != decimal && Some(separator) == thousands.first());
    if consistent && (integer.len() == 1 || grouped(integer)) {
        readings.push((integer.concat(), (*fraction).to_owned()));
    }
    readings
}

fn shifted(integer: &str, fraction: &str, exponent: i64) -> Option<(String, String)> {
    if exponent.unsigned_abs() > MAX_EXPONENT {
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

    fn text(values: &[&str]) -> Vec<Original> {
        values
            .iter()
            .map(|value| Original::Text((*value).to_owned()))
            .collect()
    }

    fn typed(values: &[&str]) -> Vec<Original> {
        values
            .iter()
            .map(|value| Original::Number((*value).to_owned()))
            .collect()
    }

    #[test]
    fn values_need_a_matching_number_in_an_authorized_original() {
        let original = text(&[
            r#"{"name":"Mystic Burst","m_flBonusSpiritDamage":"1.500","m_flCooldown":"12.50","patch":"2026-10-02"}"#,
        ]);
        for answer in [
            "Mystic Burst macht 1500 Bonus-Schaden.",
            "Mystic Burst macht 1.500 Bonus-Schaden bei 12,5 s Abklingzeit.",
            "Stand: Patch vom 2. Oktober 2026.",
            "Keine Zahl, nur eine Erklärung.",
            "1. Kauf zuerst Mystic Burst\n2) danach etwas anderes\n- T3 ist teuer",
        ] {
            assert!(unbound_values(answer, &original).is_empty(), "{answer}");
        }
        assert_eq!(
            unbound_values(
                "Mystic Burst macht 80 Schaden und lädt in 12,5 s.",
                &original
            ),
            ["80"]
        );
        assert_eq!(unbound_values("Kostet 6200 Seelen.", &[]), ["6200"]);
        let observed = text(&[r#"{"matches":1834,"winrate_percent":52.3456,"pick":99.96}"#]);
        assert!(unbound_values(
            "In 1.834 Matches 52,3 % Siegquote, gerundet 52 %, Pickrate 100 %.",
            &observed
        )
        .is_empty());
        assert_eq!(
            unbound_values("Siegquote 52,4 % oder 53 %.", &observed),
            ["52,4", "53"]
        );
    }

    #[test]
    fn separators_signs_and_exponents_keep_their_numeric_meaning() {
        assert_eq!(
            unbound_values("Abklingzeit 125 s.", &text(&["12.5"])),
            ["125"]
        );
        assert_eq!(
            unbound_values("Abklingzeit 1.25 s.", &text(&["125"])),
            ["1.25"]
        );
        assert_eq!(
            unbound_values("Abklingzeit 1,250 s.", &text(&["12.5"])),
            ["1,250"]
        );
        assert_eq!(unbound_values("Schaden 1234", &typed(&["1.234"])), ["1234"]);
        assert!(unbound_values("Schaden 1,234 oder 1.234", &typed(&["1.234"])).is_empty());
        assert!(unbound_values("Leben 1.234", &typed(&["1234"])).is_empty());
        assert_eq!(
            unbound_values("Wert 1234,567", &text(&["1,234,567"])),
            ["1234,567"]
        );
        assert!(unbound_values("Wert 1.234.567", &text(&["1,234,567"])).is_empty());
        assert!(unbound_values("Wert 1234,5", &text(&["1.234,5"])).is_empty());
        assert!(unbound_values("Verlangsamt um 30 % oder -30 %.", &typed(&["-30"])).is_empty());
        assert_eq!(unbound_values("Bonus +30 %.", &typed(&["-30"])), ["+30"]);
        assert_eq!(unbound_values("Malus −30 %.", &text(&["30"])), ["−30"]);
        assert!(unbound_values("Bonus +30 % von 10-20 s.", &text(&["30", "10", "20"])).is_empty());
        assert_eq!(unbound_values("Faktor .5", &text(&["5"])), [".5"]);
        assert!(unbound_values("Faktor 0,5", &text(&[".5"])).is_empty());
        assert_eq!(unbound_values("Wert 1e3", &text(&["1"])), ["1e3"]);
        assert!(unbound_values("Wert 1.500 oder 1500", &typed(&["1.5e3"])).is_empty());
        assert!(unbound_values("Wert 0,0000001", &typed(&["1e-7"])).is_empty());
        for huge in ["1e999", "1e-9223372036854775808", "1e99999999999999999999"] {
            assert_eq!(unbound_values(huge, &typed(&["1"])), [huge]);
            assert!(unbound_values(huge, &typed(&[huge])).is_empty());
        }
        assert!(unbound_values("Patch vom 02.10.2026", &text(&["02.10.2026"])).is_empty());
        assert!(unbound_values("Version 1.2.3", &text(&["v1.2.3", "1.2.3"])).is_empty());
        assert_eq!(
            unbound_values("Patch vom 03.10.2026", &text(&["02.10.2026"])),
            ["03.10.2026"]
        );
    }

    #[test]
    fn only_real_list_markers_hide_a_number() {
        assert_eq!(unbound_values("6200.", &[]), ["6200"]);
        assert_eq!(unbound_values("1.", &[]), ["1"]);
        assert_eq!(unbound_values("1. Minute: Urne", &[]), ["1"]);
        assert_eq!(
            unbound_values("1. Erst kaufen\n30. Minute: Urne", &[]),
            ["1", "30"]
        );
        assert_eq!(
            unbound_values("1. Minute: Urne\n2. Minute: Rift", &[]),
            ["1", "2"]
        );
        assert_eq!(unbound_values("# 1. Kauf\n# 2. Dann", &[]), ["1", "2"]);
        assert!(unbound_values("1. Erst\n2. Dann\n3) Zuletzt", &[]).is_empty());
        assert!(unbound_values(
            "1. Erst für 800\n2. Dann\n\n1. Neu\n2. Fertig",
            &text(&["800"])
        )
        .is_empty());
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
                        "respawn_interval_seconds":300,"cooldown_milliseconds":60000}]}),
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
                "Ja, alle 7 Minuten, und sie kostet 6200 Seelen bei 1000 Abklingzeit.",
                &query,
                &[],
                &conversation
            ),
            ["7", "6200", "1000"]
        );
    }
}
