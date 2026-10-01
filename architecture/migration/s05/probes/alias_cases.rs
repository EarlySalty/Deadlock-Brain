// S05-ALIAS-20260924-v1: synthetic, no player data or external game dataset.
// Legacy outputs were executed from the actual Python FunctionDef at base c00fc893.
// Tuple: ID, input, frozen legacy output, observed Rust baseline, disposition.
// Baseline differences are NOT approved product behavior; --strict rejects them.
const CASES: &[(&str, &str, &str, &str, &str)] = &[
    ("empty", "", "", "", "exact"),
    ("hero_punctuation", "Mo & Krill", "mo & krill", "mo & krill", "exact"),
    ("internal_separators", "  MO__&__KRILL  ", "mo & krill", "mo & krill", "exact"),
    ("underscore", "LADY_GEIST", "lady geist", "lady geist", "exact"),
    ("ascii_whitespace", "\tABRAMS\n", "abrams", "abrams", "exact"),
    ("sharp_s", "ß", "ss", "ß", "casefold_decision_required"),
    ("german_word", "Straße", "strasse", "straße", "casefold_decision_required"),
    ("greek_sigma", "Σςσ", "σσσ", "σςσ", "casefold_decision_required"),
    ("dotted_i", "İ", "i\u{307}", "i\u{307}", "exact"),
    ("ligature", "ﬀ", "ff", "ﬀ", "casefold_decision_required"),
    ("edge_underscores", "__MO__", " mo ", "mo", "legacy_non_idempotence_decision_required"),
    ("only_underscores", "___", " ", "", "legacy_non_idempotence_decision_required"),
    ("control_separator", "A\u{1c}B", "a b", "a\u{1c}b", "whitespace_contract_required"),
    ("nbsp", "A\u{a0}B", "a b", "a b", "exact"),
    ("umlaut", "MÖ & KRILL", "mö & krill", "mö & krill", "exact"),
    ("kelvin", "Kelvin K", "kelvin k", "kelvin k", "exact"),
    ("internal_id", "hero_bebop", "hero bebop", "hero bebop", "exact"),
    ("collapsed_space", " A\t B ", "a b", "a b", "exact"),
];
