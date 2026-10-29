use brain_contracts::tools::MechanicKind;

const TERMS: &[(&str, MechanicKind)] = &[
    ("objective", MechanicKind::Objectives),
    ("objectives", MechanicKind::Objectives),
    ("guardian", MechanicKind::Objectives),
    ("guardians", MechanicKind::Objectives),
    ("base guardian", MechanicKind::Objectives),
    ("base guardians", MechanicKind::Objectives),
    ("walker", MechanicKind::Objectives),
    ("walkers", MechanicKind::Objectives),
    ("patron", MechanicKind::Objectives),
    ("shrine", MechanicKind::Objectives),
    ("shrines", MechanicKind::Objectives),
    ("schrein", MechanicKind::Objectives),
    ("schreine", MechanicKind::Objectives),
    ("mid boss", MechanicKind::MidBoss),
    ("midboss", MechanicKind::MidBoss),
    ("urn", MechanicKind::Urn),
    ("urne", MechanicKind::Urn),
    ("soul urn", MechanicKind::Urn),
    ("seelenurne", MechanicKind::Urn),
    ("sinner's sacrifice", MechanicKind::SinnersSacrifice),
    ("sinners sacrifice", MechanicKind::SinnersSacrifice),
    ("vault", MechanicKind::SinnersSacrifice),
    ("vaults", MechanicKind::SinnersSacrifice),
    ("rejuvenator", MechanicKind::Rejuvenator),
    ("rejuv", MechanicKind::Rejuvenator),
];

pub(super) fn resolve(text: &str) -> Vec<MechanicKind> {
    let phrases = phrases(text);
    let mut kinds = Vec::new();
    for (term, kind) in TERMS {
        if !kinds.contains(kind) && phrases.iter().any(|phrase| close(phrase, term)) {
            kinds.push(*kind);
        }
    }
    kinds
}

fn phrases(text: &str) -> Vec<String> {
    let lowered = text.trim().to_lowercase();
    let words: Vec<&str> = lowered
        .split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .filter(|word| !word.is_empty())
        .collect();
    let pairs = words.windows(2).map(|pair| pair.join(" "));
    std::iter::once(words.join(" "))
        .chain(words.iter().map(|word| (*word).to_owned()))
        .chain(pairs)
        .collect()
}

fn close(phrase: &str, term: &str) -> bool {
    let allowed = match term.chars().count() {
        0..=4 => 0,
        5 => 1,
        _ => 2,
    };
    distance(phrase, term) <= allowed
}

fn distance(left: &str, right: &str) -> usize {
    let a: Vec<char> = left.chars().collect();
    let b: Vec<char> = right.chars().collect();
    let width = b.len() + 1;
    let mut table: Vec<usize> = (0..(a.len() + 1) * width).collect();
    for i in 0..=a.len() {
        table[i * width] = i;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (table[(i - 1) * width + j] + 1)
                .min(table[i * width + j - 1] + 1)
                .min(table[(i - 1) * width + j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(table[(i - 2) * width + j - 2] + 1);
            }
            table[i * width + j] = best;
        }
    }
    table[a.len() * width + b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misspelled_objectives_resolve_to_their_mechanic() {
        for text in [
            "Shiren",
            "Base Guardien",
            "guardien",
            "Schrein",
            "Walker",
            "patron",
            "ein Shiren und ein Base Guardien",
        ] {
            assert_eq!(resolve(text), vec![MechanicKind::Objectives], "{text}");
        }
        assert_eq!(resolve("Mid Boss"), vec![MechanicKind::MidBoss]);
        assert_eq!(resolve("Urn"), vec![MechanicKind::Urn]);
        assert_eq!(
            resolve("Sinner's Sacrifice"),
            vec![MechanicKind::SinnersSacrifice]
        );
        assert_eq!(resolve("Vaults"), vec![MechanicKind::SinnersSacrifice]);
        assert_eq!(resolve("Rejuvenator"), vec![MechanicKind::Rejuvenator]);
    }

    #[test]
    fn hero_names_do_not_resolve_to_mechanics() {
        for text in [
            "Shiv", "Warden", "Paige", "Seven", "Pocket", "Vyper", "Haze", "und",
        ] {
            assert!(resolve(text).is_empty(), "{text}");
        }
    }

    #[test]
    fn transpositions_count_once() {
        assert_eq!(distance("shiren", "shrine"), 2);
        assert_eq!(distance("guardien", "guardian"), 1);
        assert_eq!(distance("shiv", "shrine"), 3);
    }
}
