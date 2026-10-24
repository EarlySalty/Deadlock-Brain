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

fn marker_counts(text: &str) -> (usize, usize) {
    let words = text
        .split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>();
    let count = |markers: &[&str]| {
        words
            .iter()
            .filter(|word| markers.contains(&word.as_str()))
            .count()
    };
    let umlauts = text.chars().any(|character| "äöüßÄÖÜ".contains(character));
    (
        count(ENGLISH_MARKERS),
        count(GERMAN_MARKERS) + usize::from(umlauts),
    )
}

pub fn english(text: &str) -> bool {
    let (english, german) = marker_counts(text);
    english > german
}

pub fn german_leaning(text: &str) -> bool {
    let (english, german) = marker_counts(text);
    german > english
}
