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

pub fn term_variants(term: &str) -> Vec<String> {
    let base = normalize(term);
    let mut variants = vec![base.clone()];
    if base.len() >= 4 && base.chars().all(|c| c.is_ascii_digit()) {
        let grouped = group_thousands(&base);
        variants.push(grouped.replace(' ', "."));
        variants.push(grouped);
    }
    variants
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
