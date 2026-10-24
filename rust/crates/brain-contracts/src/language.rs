const ENGLISH_MARKERS: &[&str] = &[
    "what",
    "how",
    "which",
    "who",
    "why",
    "when",
    "where",
    "does",
    "do",
    "did",
    "is",
    "are",
    "the",
    "much",
    "many",
    "changed",
    "cost",
    "costs",
    "health",
    "damage",
    "ability",
    "abilities",
    "counter",
    "counters",
    "counts",
    "should",
    "can",
    "best",
    "against",
    "latest",
    "fast",
    "speed",
    "bullet",
    "ultimate",
    "of",
    "for",
    "in",
    "it",
    "i",
];

const GERMAN_MARKERS: &[&str] = &[
    "was",
    "wie",
    "welche",
    "welcher",
    "welches",
    "wer",
    "warum",
    "wann",
    "wo",
    "ist",
    "sind",
    "hat",
    "haben",
    "der",
    "die",
    "das",
    "den",
    "dem",
    "und",
    "im",
    "bei",
    "wurde",
    "viel",
    "viele",
    "macht",
    "gegen",
    "kostet",
    "schnell",
    "hoch",
    "von",
    "mit",
    "ich",
    "kann",
    "soll",
    "neuesten",
    "letzten",
    "geändert",
    "lebenspunkte",
    "fähigkeit",
    "fähigkeiten",
    "grundschaden",
    "seelen",
    "helfen",
    "läuft",
    "spiele",
    "man",
    "auch",
    "ein",
    "eine",
];

const SHARED_WORDS: &[&str] = &["was", "die", "man", "hat"];

fn words(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn count(words: &[String], markers: &[&str], excluded: &[&str]) -> usize {
    words
        .iter()
        .filter(|word| markers.contains(&word.as_str()) && !excluded.contains(&word.as_str()))
        .count()
}

fn umlauts(text: &str) -> usize {
    usize::from(text.chars().any(|character| "äöüßÄÖÜ".contains(character)))
}

pub fn english(text: &str) -> bool {
    let words = words(text);
    count(&words, ENGLISH_MARKERS, &[]) > count(&words, GERMAN_MARKERS, &[]) + umlauts(text)
}

pub fn clearly_english(text: &str) -> bool {
    let words = words(text);
    let english = count(&words, ENGLISH_MARKERS, &["in"]);
    english >= 2 && english > count(&words, GERMAN_MARKERS, &[]) + umlauts(text)
}

pub fn german_leaning(text: &str) -> bool {
    let words = words(text);
    let german = count(&words, GERMAN_MARKERS, SHARED_WORDS) + umlauts(text);
    german >= 2 && german > count(&words, ENGLISH_MARKERS, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn german_answer_prose_is_told_apart_from_english_prose() {
        assert!(german_leaning(
            "Metal Skin kostet 3200 Souls. Der Effekt laut Spieldaten: Become immune to bullets. Du wirst also immun gegen Kugelschaden."
        ));
        assert!(!german_leaning("Minions die faster near the Walker."));
        assert!(!german_leaning("Was it worth it? The man hat a plan."));
        assert!(!german_leaning(
            "Metal Skin makes you immune to bullets for a few seconds."
        ));
        assert!(english("What does Metal Skin cost?"));
        assert!(clearly_english("What does Metal Skin cost?"));
        assert!(clearly_english("What counters Infernus?"));
        assert!(!clearly_english("Haze Build in Street Brawl?"));
        assert!(!clearly_english("Was kostet Metal Skin?"));
        assert!(!english("Was kostet Metal Skin?"));
    }
}
