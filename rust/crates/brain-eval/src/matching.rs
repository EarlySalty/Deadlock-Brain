pub fn normalize(text: &str) -> String {
    let lowered: Vec<char> = text
        .chars()
        .filter(|c| *c != '\u{00ad}')
        .map(fold_char)
        .flat_map(char::to_lowercase)
        .collect();
    lowered
        .iter()
        .enumerate()
        .map(|(i, c)| decimal_comma(&lowered, i, *c))
        .collect()
}

fn fold_char(c: char) -> char {
    match c {
        '\u{2019}' | '\u{2018}' | '`' | '\u{00b4}' => '\'',
        '\u{00a0}' | '\u{202f}' | '\u{2009}' => ' ',
        '\u{2013}' | '\u{2014}' | '\u{2212}' => '-',
        other => other,
    }
}

fn decimal_comma(chars: &[char], i: usize, c: char) -> char {
    let digit_at = |j: Option<usize>| {
        j.and_then(|j| chars.get(j))
            .is_some_and(char::is_ascii_digit)
    };
    if c == ',' && digit_at(i.checked_sub(1)) && digit_at(Some(i + 1)) {
        '.'
    } else {
        c
    }
}

const SECONDS: &[&str] = &[
    "s", "sek", "sec", "sekunde", "sekunden", "second", "seconds",
];
const MINUTES: &[&str] = &["min", "mins", "minute", "minuten", "minutes"];
const PERCENT: &[&str] = &["%", "prozent", "percent"];
const UNITS: [&[&str]; 3] = [SECONDS, MINUTES, PERCENT];

pub fn term_variants(term: &str) -> Vec<String> {
    let base = normalize(term);
    let mut variants = vec![base.clone()];
    variants.extend(thousands_variants(&base));
    variants.extend(unit_variants(&base));
    variants
}

fn thousands_variants(base: &str) -> Vec<String> {
    if base.len() < 4 || !base.chars().all(|c| c.is_ascii_digit()) {
        return Vec::new();
    }
    let grouped = group_thousands(base);
    vec![grouped.replace(' ', "."), grouped]
}

fn unit_variants(base: &str) -> Vec<String> {
    let Some((number, unit)) = split_quantity(base) else {
        return Vec::new();
    };
    let Some(spellings) = UNITS.iter().find(|spellings| spellings.contains(&unit)) else {
        return Vec::new();
    };
    spellings
        .iter()
        .flat_map(|s| [format!("{number}{s}"), format!("{number} {s}")])
        .collect()
}

fn split_quantity(base: &str) -> Option<(&str, &str)> {
    let end = base.find(|c: char| !(c.is_ascii_digit() || c == '.'))?;
    let number = &base[..end];
    let digit_edges = number.starts_with(|c: char| c.is_ascii_digit())
        && number.ends_with(|c: char| c.is_ascii_digit());
    digit_edges.then(|| (number, base[end..].trim_start()))
}

fn group_thousands(digits: &str) -> String {
    let len = digits.len();
    digits
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let sep = (i > 0 && (len - i).is_multiple_of(3)).then_some(' ');
            sep.into_iter().chain(std::iter::once(c))
        })
        .collect()
}

pub fn contains_term(normalized_text: &str, term: &str) -> bool {
    term_variants(term)
        .iter()
        .any(|variant| contains_bounded(normalized_text, variant))
}

fn contains_bounded(text: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    text.match_indices(needle).any(|(start, _)| {
        let end = start + needle.len();
        boundary_ok(text[..start].chars().next_back(), needle.chars().next())
            && boundary_ok(text[end..].chars().next(), needle.chars().next_back())
            && !decimal_continues(&text[end..], needle)
            && !decimal_precedes(&text[..start], needle)
    })
}

fn boundary_ok(neighbour: Option<char>, edge: Option<char>) -> bool {
    match (neighbour, edge) {
        (Some(n), Some(e)) if e.is_alphanumeric() => !n.is_alphanumeric(),
        _ => true,
    }
}

fn decimal_continues(rest: &str, needle: &str) -> bool {
    let mut chars = rest.chars();
    needle
        .chars()
        .next_back()
        .is_some_and(|c| c.is_ascii_digit())
        && chars.next() == Some('.')
        && chars.next().is_some_and(|c| c.is_ascii_digit())
}

fn decimal_precedes(before: &str, needle: &str) -> bool {
    let mut chars = before.chars().rev();
    needle.starts_with(|c: char| c.is_ascii_digit())
        && chars.next() == Some('.')
        && chars.next().is_some_and(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(text: &str, term: &str) -> bool {
        contains_term(&normalize(text), term)
    }

    #[test]
    fn numbers_need_token_boundaries() {
        assert!(hit("Monster Rounds kostet 800 Souls.", "800"));
        assert!(!hit("Es kostet 1800 Souls.", "800"));
        assert!(!hit("Der Wert ist 8.25", "8.2"));
        assert!(!hit("Der Wert ist 12.5", "12"));
        assert!(hit("Haze läuft mit 8,2 m/s.", "8.2"));
        assert!(hit("Bewegung 8.2.", "8.2"));
    }

    #[test]
    fn numbers_reject_leading_decimal_or_thousands_part() {
        assert!(!hit("Das kostet 1.800 Souls.", "800"));
        assert!(!hit("It costs 1,800 souls.", "800"));
        assert!(!hit("Der Faktor ist 0.12", "12"));
        assert!(!hit("Der Faktor ist 0,12", "12"));
        assert!(hit("Wert 12. Danach", "12"));
    }

    #[test]
    fn unit_terms_match_spaced_and_word_forms() {
        assert!(hit("Der Midboss respawnt nach 7 Minuten.", "7 min"));
        assert!(hit("The midboss respawns after 7 minutes.", "7 min"));
        assert!(hit("Abklingzeit jetzt 28 Sekunden", "28s"));
        assert!(hit("cooldown 28 s", "28s"));
        assert!(hit("cooldown 28 seconds", "28s"));
        assert!(hit("Lifesteal steigt auf 25 %", "25%"));
        assert!(hit("lifesteal 25 percent", "25%"));
        assert!(hit("Es ist ein Tier-3-Item.", "tier-3"));
        assert!(!hit("Abklingzeit 128 Sekunden", "28s"));
        assert!(!hit("Abklingzeit 28 Stunden", "28s"));
        assert!(!hit("range 26 m", "26s"));
    }

    #[test]
    fn thousands_separators_match_plain_terms() {
        assert!(hit("Boundless Spirit kostet 6.400 Souls.", "6400"));
        assert!(hit("Boundless Spirit costs 6,400 souls.", "6400"));
        assert!(hit("costs 6\u{202f}400 souls", "6400"));
    }

    #[test]
    fn names_ignore_case_and_apostrophe_style() {
        assert!(hit("BABA\u{2019}S BREW heilt", "Baba's Brew"));
        assert!(hit(
            "Schwächungs\u{00ad}reduzierer",
            "Schwächungsreduzierer"
        ));
        assert!(!hit("Stakeholder", "Stake"));
    }
}
