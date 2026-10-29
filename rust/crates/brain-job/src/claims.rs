use chrono::NaiveDate;
use regex::Regex;
use std::sync::LazyLock;

const VALUE: &str = r"\d+(?:\.\d+)?(?:\s?/\s?\d+(?:\.\d+)?)*";
const MONTHS: [(&str, u32); 23] = [
    ("januar", 1),
    ("january", 1),
    ("februar", 2),
    ("february", 2),
    ("märz", 3),
    ("maerz", 3),
    ("march", 3),
    ("april", 4),
    ("mai", 5),
    ("may", 5),
    ("juni", 6),
    ("june", 6),
    ("juli", 7),
    ("july", 7),
    ("august", 8),
    ("september", 9),
    ("oktober", 10),
    ("october", 10),
    ("november", 11),
    ("dezember", 12),
    ("december", 12),
    ("jänner", 1),
    ("sept", 9),
];

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect("constant pattern")
}

static NUMERIC_DATES: LazyLock<Regex> = LazyLock::new(|| {
    regex(
        r"\b(?:(\d{1,2})\.\s?(\d{1,2})\.\s?(\d{4})|(\d{4})-(\d{2})-(\d{2})|(\d{2})-(\d{2})-(\d{4}))\b",
    )
});
static WORD_DATES: LazyLock<Regex> = LazyLock::new(|| {
    regex(
        r"(?i)\b(?:(\d{1,2})\.?\s+([a-zä]{3,9})\s+(\d{4})|([a-zä]{3,9})\s+(\d{1,2}),?\s+(\d{4}))\b",
    )
});
static VALUES: LazyLock<Regex> = LazyLock::new(|| regex(VALUE));
static UNIT_VALUES: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"(?i)({VALUE})\s?(%|prozent|percent|sekunden|sek\.?|seconds?|s|meter|m|seelen|souls|hp|leben|health|schaden|damage|dps)(?:\b|$|[^a-zäöü])"
    ))
});
static TRANSITIONS: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"(?i)(?:\bvon|\bfrom)\s+[+\-]?({VALUE})\s?(?:%|s|m|sek\w*|seconds?|seelen|souls)?\s+(?:auf|to|zu)\s+[+\-]?({VALUE})|({VALUE})\s?%?\s?(?:->|→|=>)\s?({VALUE})"
    ))
});
static PATCH_CONTEXT: LazyLock<Regex> =
    LazyLock::new(|| regex(r"(?i)\b(?:patch\w*|\w*update\w*|changelog|hotfix|patchnotes)\b"));
static PAST: LazyLock<Regex> = LazyLock::new(|| {
    regex(
        r"(?i)\b(?:früher|frueher|vorher|davor|damals|ursprünglich|war|waren|wurde|wurden|bis|vor dem|alt\w*|previously|used to|was|were|before|originally|old)\b",
    )
});
static WORD: LazyLock<Regex> = LazyLock::new(|| regex(r"[a-zäöüß]{3,}"));

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub text: String,
    pub number: Option<f64>,
}

impl Value {
    pub fn parse(raw: &str) -> Option<Self> {
        let text: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
        if text.is_empty() || !text.starts_with(|c: char| c.is_ascii_digit()) {
            return None;
        }
        let number = (!text.contains('/'))
            .then(|| text.parse::<f64>().ok())
            .flatten();
        Some(Value { text, number })
    }

    pub fn same(&self, other: &Value) -> bool {
        match (self.number, other.number) {
            (Some(a), Some(b)) => close(a, b),
            (None, None) => self.text == other.text,
            _ => false,
        }
    }

    pub fn scaled(&self, other: &Value) -> bool {
        match (self.number, other.number) {
            (Some(a), Some(b)) => [1.0, 100.0, 0.01, 1000.0, 0.001]
                .iter()
                .any(|factor| close(a, b * factor)),
            _ => self.same(other),
        }
    }

    pub fn parts(&self) -> Vec<f64> {
        self.text
            .trim_end_matches('%')
            .split('/')
            .filter_map(|part| part.trim_end_matches('%').parse().ok())
            .collect()
    }

    pub fn significant(&self) -> bool {
        self.number.is_none_or(|n| n >= 2.0 || n.fract() != 0.0)
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-6_f64.max(b.abs() * 1e-6)
}

pub fn loose(a: f64, b: f64) -> bool {
    (a - b).abs() <= 0.051_f64.max(b.abs() * 0.04)
}

pub fn values(text: &str) -> Vec<Value> {
    VALUES
        .find_iter(text)
        .filter(|found| boundary(text, found.start(), found.end()))
        .filter_map(|found| Value::parse(found.as_str()))
        .collect()
}

fn boundary(text: &str, start: usize, end: usize) -> bool {
    let before = text[..start].chars().next_back();
    let after = text[end..].chars().next();
    !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        && !after.is_some_and(|c| c.is_ascii_digit())
}

pub struct Dated {
    pub date: NaiveDate,
    pub start: usize,
    pub end: usize,
}

fn month(name: &str) -> Option<u32> {
    let lower = name.to_lowercase();
    MONTHS
        .iter()
        .find(|(month, _)| *month == lower)
        .map(|(_, number)| *number)
}

fn number(captures: &regex::Captures<'_>, index: usize) -> Option<u32> {
    captures.get(index)?.as_str().parse().ok()
}

fn year(captures: &regex::Captures<'_>, index: usize) -> Option<i32> {
    captures.get(index)?.as_str().parse().ok()
}

pub fn dates(text: &str) -> Vec<Dated> {
    let mut found = Vec::new();
    for captures in NUMERIC_DATES.captures_iter(text) {
        let Some(whole) = captures.get(0) else {
            continue;
        };
        let date = if captures.get(1).is_some() {
            NaiveDate::from_ymd_opt(
                year(&captures, 3).unwrap_or(0),
                number(&captures, 2).unwrap_or(0),
                number(&captures, 1).unwrap_or(0),
            )
        } else if captures.get(4).is_some() {
            NaiveDate::from_ymd_opt(
                year(&captures, 4).unwrap_or(0),
                number(&captures, 5).unwrap_or(0),
                number(&captures, 6).unwrap_or(0),
            )
        } else {
            NaiveDate::from_ymd_opt(
                year(&captures, 9).unwrap_or(0),
                number(&captures, 7).unwrap_or(0),
                number(&captures, 8).unwrap_or(0),
            )
        };
        if let Some(date) = date.filter(|date| plausible(*date)) {
            found.push(Dated {
                date,
                start: whole.start(),
                end: whole.end(),
            });
        }
    }
    for captures in WORD_DATES.captures_iter(text) {
        let Some(whole) = captures.get(0) else {
            continue;
        };
        let (day, name, year_index) = match captures.get(1) {
            Some(_) => (number(&captures, 1), captures.get(2), 3),
            None => (number(&captures, 5), captures.get(4), 6),
        };
        let date = name
            .and_then(|name| month(name.as_str()))
            .and_then(|month| NaiveDate::from_ymd_opt(year(&captures, year_index)?, month, day?));
        if let Some(date) = date.filter(|date| plausible(*date)) {
            found.push(Dated {
                date,
                start: whole.start(),
                end: whole.end(),
            });
        }
    }
    found.sort_by_key(|dated| dated.start);
    found
}

fn plausible(date: NaiveDate) -> bool {
    let year = chrono::Datelike::year(&date);
    (2020..=2035).contains(&year)
}

pub fn title_date(title: &str) -> Option<NaiveDate> {
    dates(title).first().map(|dated| dated.date)
}

pub fn sentences(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for (position, &(index, c)) in chars.iter().enumerate() {
        let hard = c == '\n' || c == '•';
        let soft = matches!(c, '.' | '!' | '?')
            && chars
                .get(position + 1)
                .is_some_and(|(_, next)| next.is_whitespace())
            && chars[position + 1..]
                .iter()
                .map(|(_, next)| *next)
                .find(|next| {
                    !next.is_whitespace() && !matches!(next, '*' | '_' | '"' | '`' | '-' | '„')
                })
                .is_some_and(char::is_uppercase)
            && !chars[..position]
                .iter()
                .next_back()
                .is_some_and(|(_, previous)| previous.is_ascii_digit());
        if hard || soft {
            let end = if hard { index } else { index + c.len_utf8() };
            push(&mut out, &text[start..end]);
            start = index + c.len_utf8();
        }
    }
    push(&mut out, &text[start..]);
    out
}

fn push<'a>(out: &mut Vec<&'a str>, sentence: &'a str) {
    let trimmed = sentence.trim();
    if !trimmed.is_empty() {
        out.push(trimmed);
    }
}

pub fn masked(text: &str) -> String {
    let mut out = text.to_owned();
    for dated in dates(text).iter().rev() {
        out.replace_range(dated.start..dated.end, &" ".repeat(dated.end - dated.start));
    }
    out
}

pub fn transitions(sentence: &str) -> Vec<(Value, Value)> {
    TRANSITIONS
        .captures_iter(&masked(sentence))
        .filter_map(|captures| {
            let (from, to) = match captures.get(1) {
                Some(from) => (from, captures.get(2)?),
                None => (captures.get(3)?, captures.get(4)?),
            };
            Some((Value::parse(from.as_str())?, Value::parse(to.as_str())?))
        })
        .collect()
}

pub fn unit_values(sentence: &str) -> Vec<(Value, String, usize)> {
    let masked = masked(sentence);
    UNIT_VALUES
        .captures_iter(&masked)
        .filter_map(|captures| {
            let value = captures.get(1)?;
            if !boundary(&masked, value.start(), value.end()) {
                return None;
            }
            let parsed = Value::parse(value.as_str())?;
            parsed.number.is_some().then(|| {
                (
                    parsed,
                    captures
                        .get(2)
                        .map_or("", |unit| unit.as_str())
                        .to_lowercase(),
                    value.start(),
                )
            })
        })
        .collect()
}

pub fn patch_context(sentence: &str) -> bool {
    PATCH_CONTEXT.is_match(sentence)
}

pub fn past(sentence: &str) -> bool {
    PAST.is_match(sentence)
}

pub fn tokens(text: &str) -> std::collections::BTreeSet<String> {
    const GENERIC: &[&str] = &[
        "changed",
        "change",
        "increased",
        "reduced",
        "decreased",
        "from",
        "with",
        "that",
        "this",
        "when",
        "base",
        "now",
        "also",
        "will",
        "into",
        "than",
        "have",
        "been",
        "damage",
        "duration",
        "cooldown",
        "range",
        "radius",
        "bonus",
        "spirit",
        "power",
        "scaling",
        "while",
        "after",
        "before",
        "their",
        "them",
        "they",
        "your",
        "the",
        "and",
        "for",
        "you",
        "are",
        "was",
        "its",
        "has",
        "but",
        "not",
        "per",
        "all",
        "der",
        "die",
        "das",
        "und",
        "mit",
        "den",
        "dem",
        "des",
        "ein",
        "eine",
        "auf",
        "von",
        "bei",
        "für",
        "aus",
        "ist",
        "war",
        "nur",
        "bis",
        "vom",
        "zum",
        "zur",
        "wie",
        "was",
        "sich",
        "seit",
        "jetzt",
        "wird",
        "update",
    ];
    WORD.find_iter(&text.to_lowercase())
        .map(|word| word.as_str())
        .filter(|word| !GENERIC.contains(word))
        .map(|word| match word.strip_suffix('s') {
            Some(stem) if stem.chars().count() >= 3 && !stem.ends_with('s') => stem.to_owned(),
            _ => word.to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> Vec<String> {
        dates(text)
            .iter()
            .map(|dated| dated.date.to_string())
            .collect()
    }

    #[test]
    fn dates_cover_german_iso_valve_and_written_forms() {
        assert_eq!(
            day("Update vom 21.11.2024, Minor Update - 06-30-2026, am 2025-11-21 und 21. November 2025 sowie June 30, 2026"),
            ["2024-11-21", "2026-06-30", "2025-11-21", "2025-11-21", "2026-06-30"]
        );
        assert!(day("Version 1.2.3 und 60/40").is_empty());
        assert_eq!(
            title_date("Minor Update - 06-30-2026")
                .map(|d| d.to_string())
                .as_deref(),
            Some("2026-06-30")
        );
    }

    #[test]
    fn sentences_keep_dates_and_split_bullets() {
        assert_eq!(
            sentences("Am 21. November 2025 kam es. Danach 50/50 • Seit 30.06.2026 gilt 50"),
            [
                "Am 21. November 2025 kam es.",
                "Danach 50/50",
                "Seit 30.06.2026 gilt 50"
            ]
        );
    }

    #[test]
    fn transitions_and_units_ignore_dates() {
        let found = transitions("Am 21.11.2024 von 60/40 auf 65/35 geändert, dann 40 -> 50");
        let texts: Vec<_> = found
            .iter()
            .map(|(a, b)| (a.text.as_str(), b.text.as_str()))
            .collect();
        assert_eq!(texts, [("60/40", "65/35"), ("40", "50")]);
        let signed = transitions("Feuerrate von +32% auf +30%");
        assert!(matches!(signed.as_slice(), [(a, b)] if a.text == "32" && b.text == "30"));
        let units: Vec<_> = unit_values("Kostet 1250 Seelen, 12 % mehr, 3.5s Dauer am 21.11.2024")
            .into_iter()
            .map(|(value, unit, _)| (value.text, unit))
            .collect();
        assert_eq!(
            units,
            [
                ("1250".into(), "seelen".into()),
                ("12".into(), "%".into()),
                ("3.5".into(), "s".into())
            ]
        );
    }

    #[test]
    fn values_compare_with_percent_and_millisecond_scaling() {
        let half = Value::parse("0.5");
        let fifty = Value::parse("50");
        assert!(matches!((&half, &fifty), (Some(a), Some(b)) if a.scaled(b) && !a.same(b)));
        assert!(
            matches!((Value::parse("60/40"), Value::parse("60 / 40")), (Some(a), Some(b)) if a.same(&b))
        );
        assert_eq!(
            values("von 200/230/260% auf 170/200/230%")
                .iter()
                .map(Value::parts)
                .collect::<Vec<_>>(),
            [vec![200.0, 230.0, 260.0], vec![170.0, 200.0, 230.0]]
        );
    }
}
