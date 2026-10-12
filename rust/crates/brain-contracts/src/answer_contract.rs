use crate::{Evidence, SourceVisibility};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

pub const NO_AUTHORIZED_VALUE: &str =
    "Dafür habe ich keinen gesicherten Wert aus den freigegebenen Spieldaten. Zahlen rate ich nicht.";

pub const STRUCTURED_ANSWER_INSTRUCTION: &str = " Nur JSON: {\"kind\":\"grounded\",\"parts\":[{\"kind\":\"prose\",\"text\":\"...\"}],\"cited_evidence_ids\":[\"ID\"]}. parts verketten; Prose zahlenfrei, Leerzeichen dort. Werte nur als {\"kind\":\"game_value\",\"evidence_id\":\"ID\",\"field\":\"...\",\"value\":\"...\"} exakt aus citable_values; other ebenso, nur Datum, Zählung, Version, keine Spielwerte. IDs aus evidence.id exakt und einmal zitieren. Grounded ohne Beleg: beide Listen leer. Gruß, Dank, Smalltalk ohne Spielbehauptung: social, nur Prose, keine Belege. Unsichere belegte Tipps bleiben grounded.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerKind {
    Social,
    Grounded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnswerPart {
    Prose {
        text: String,
    },
    GameValue {
        evidence_id: String,
        field: String,
        value: String,
    },
    Other {
        evidence_id: String,
        field: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuredAnswer {
    pub kind: AnswerKind,
    pub parts: Vec<AnswerPart>,
    pub cited_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerViolation {
    Format,
    Citation,
    Value,
}

impl AnswerViolation {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Format => "grounded answer envelope missing",
            Self::Citation => "unknown, duplicate or missing citation",
            Self::Value => "cited value does not match evidence",
        }
    }

    pub fn from_error(error: &crate::PortError) -> Option<Self> {
        let crate::PortError::InvalidResponse(code) = error else {
            return None;
        };
        [Self::Format, Self::Citation, Self::Value]
            .into_iter()
            .find(|violation| violation.code() == code)
    }
}

pub fn is_game_source(evidence: &Evidence) -> bool {
    if evidence.visibility == SourceVisibility::RequestScoped {
        return false;
    }
    matches!(
        evidence.source_id.as_str(),
        "deadlock-assets-heroes"
            | "deadlock-assets-items"
            | "deadlock-assets-abilities"
            | "deadlock-wiki-deadlock-data"
            | "steamtracking-gametracking-deadlock"
            | "legacy-entities"
            | "legacy-patchnotes"
            | "git-game-facts-derived"
            | "deadlock_analytics_api"
            | "deadlock-game-data-v1"
            | "deadlock-api-live-v1"
            | "brain.patch_changes"
            | "brain.entity_profile_entities_v1"
            | "deadlock_assets_api"
            | "patchnotes-feed"
    )
}

fn other_field(field: &str) -> bool {
    let key = field
        .rsplit('/')
        .next()
        .unwrap_or(field)
        .to_ascii_lowercase();
    matches!(
        key.as_str(),
        "date"
            | "published_at"
            | "updated_at"
            | "event_date"
            | "patch"
            | "patch_version"
            | "version"
            | "revision"
            | "count"
            | "member_count"
            | "occupancy"
            | "capacity"
            | "available_slots"
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CitableValue {
    pub kind: &'static str,
    pub field: String,
    pub value: String,
}

fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(_) | Value::Bool(_) => Some(value.to_string()),
        _ => None,
    }
}

fn json_fields(value: &Value, path: &str, fields: &mut Vec<(String, String)>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let key = key.replace('~', "~0").replace('/', "~1");
                json_fields(value, &format!("{path}/{key}"), fields);
            }
        }
        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                json_fields(value, &format!("{path}/{index}"), fields);
            }
        }
        _ => {
            if let Some(value) = scalar(value) {
                fields.push((path.to_owned(), value));
            }
        }
    }
}

fn compact_fields(value: &Value, fields: &mut Vec<(String, String)>) {
    for section in ["facts", "context"] {
        if let Some(facts) = value[section].as_array() {
            for (index, fact) in facts.iter().enumerate() {
                for field in ["value", "unit", "qualifiers"] {
                    json_fields(&fact[field], &format!("/{section}/{index}/{field}"), fields);
                }
                for field in [
                    "from_patch",
                    "to_patch_exclusive",
                    "through_patch_inclusive",
                ] {
                    json_fields(
                        &fact["validity"][field],
                        &format!("/{section}/{index}/validity/{field}"),
                        fields,
                    );
                }
            }
        }
    }
    if let Some(changes) = value["patch_story"].as_array() {
        for (index, change) in changes.iter().enumerate() {
            for field in ["patch_date", "old_value", "new_value", "conditions"] {
                json_fields(
                    &change[field],
                    &format!("/patch_story/{index}/{field}"),
                    fields,
                );
            }
        }
    }
    for field in ["name", "aliases"] {
        json_fields(&value["entity"][field], &format!("/entity/{field}"), fields);
    }
    json_fields(&value["patch"], "/patch", fields);
}

fn evidence_fields(evidence: &Evidence, compact: bool) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    if let Ok(value) = crate::provider_input::parse_unique_json(evidence.content.as_bytes()) {
        if compact
            && value["contract_version"].as_str()
                == Some(crate::entity_profile::ENTITY_PROFILE_VERSION)
        {
            compact_fields(&value, &mut fields);
        } else {
            json_fields(&value, "", &mut fields);
        }
    } else if evidence.content.trim_start().starts_with(['{', '[']) {
        return fields;
    } else {
        for line in evidence.content.lines() {
            if let Some((field, value)) = line.split_once(':') {
                fields.push((field.trim().to_owned(), value.trim().to_owned()));
            }
        }
    }
    fields
}

fn value_catalog(evidence: &Evidence, fields: Vec<(String, String)>) -> Vec<CitableValue> {
    let game = is_game_source(evidence);
    let mut seen = BTreeSet::new();
    let mut ambiguous = BTreeSet::new();
    for (field, _) in &fields {
        if !seen.insert(field) {
            ambiguous.insert(field);
        }
    }
    fields
        .iter()
        .filter_map(|(field, value)| {
            if field.is_empty()
                || value.is_empty()
                || ambiguous.contains(field)
                || (!game && !other_field(field))
                || (evidence.visibility == SourceVisibility::RequestScoped
                    && (crate::provider_input::discord_display_text(field) != *field
                        || crate::provider_input::discord_display_text(value) != *value))
            {
                return None;
            }
            Some(CitableValue {
                kind: if other_field(field) {
                    "other"
                } else {
                    "game_value"
                },
                field: field.clone(),
                value: value.clone(),
            })
        })
        .collect()
}

pub fn citable_values(evidence: &Evidence) -> Vec<CitableValue> {
    value_catalog(evidence, evidence_fields(evidence, false))
}

pub fn provider_citable_values(evidence: &Evidence) -> Vec<CitableValue> {
    value_catalog(evidence, evidence_fields(evidence, true))
        .into_iter()
        .filter(|value| value.value.chars().any(char::is_numeric))
        .collect()
}

fn number_sign(character: char) -> bool {
    matches!(character, '+' | '-' | '−' | '＋' | '－')
}

fn exponent_end(characters: &[char], start: usize, marker: char) -> Option<usize> {
    if !characters
        .get(start)
        .is_some_and(|value| value.eq_ignore_ascii_case(&marker))
    {
        return None;
    }
    let mut end = start + 1;
    if characters.get(end).is_some_and(|value| number_sign(*value)) {
        end += 1;
    }
    separated_digits_end(characters, end, char::is_numeric)
}

fn separated_digits_end(
    characters: &[char],
    start: usize,
    digit: impl Fn(char) -> bool,
) -> Option<usize> {
    let mut end = start;
    let mut has_digit = false;
    while let Some(value) = characters.get(end) {
        if digit(*value) {
            has_digit = true;
        } else if *value != '_'
            && !(has_digit
                && matches!(*value, '\'' | '’' | 'ʼ')
                && characters.get(end + 1).is_some_and(|value| digit(*value)))
        {
            break;
        }
        end += 1;
    }
    has_digit.then_some(end)
}

fn radix_end(characters: &[char], start: usize) -> Option<usize> {
    if characters.get(start) != Some(&'0') {
        return None;
    }
    let radix = match characters.get(start + 1) {
        Some('x' | 'X') => 16,
        Some('b' | 'B') => 2,
        Some('o' | 'O') => 8,
        _ => return None,
    };
    let integer = separated_digits_end(characters, start + 2, |value| value.is_digit(radix));
    if radix == 16 {
        let mut end = integer.unwrap_or(start + 2);
        let fraction = if characters.get(end) == Some(&'.') {
            end += 1;
            separated_digits_end(characters, end, |value| value.is_digit(radix))
        } else {
            None
        };
        end = fraction.unwrap_or(end);
        if integer.is_some() || fraction.is_some() {
            if let Some(end) = exponent_end(characters, end, 'p') {
                return Some(end);
            }
        }
    }
    integer
}

fn number_end(characters: &[char], start: usize) -> Option<usize> {
    let mut end = start;
    if characters.get(end).is_some_and(|value| number_sign(*value)) {
        end += 1;
    }
    if let Some(end) = radix_end(characters, end) {
        return Some(end);
    }
    if matches!(characters.get(end), Some('.' | ',' | '٫' | '．' | '，')) {
        end += 1;
    }
    let digits = end;
    while characters.get(end).is_some_and(|value| value.is_numeric()) {
        end += 1;
    }
    if end == digits {
        return None;
    }
    loop {
        let separator = end;
        match characters.get(end) {
            Some('_') => {
                while characters.get(end) == Some(&'_') {
                    end += 1;
                }
                while characters.get(end).is_some_and(|value| value.is_numeric()) {
                    end += 1;
                }
            }
            Some('.' | ',' | '٫' | '٬' | '．' | '，' | '\'' | '’' | 'ʼ') => {
                end += 1;
                while characters.get(end).is_some_and(|value| value.is_numeric()) {
                    end += 1;
                }
                if end == separator + 1 {
                    if matches!(characters[separator], '.' | ',' | '٫' | '．' | '，') {
                        if let Some(end) = exponent_end(characters, end, 'e') {
                            return Some(end);
                        }
                    }
                    end = separator;
                    break;
                }
            }
            Some(' ' | '\u{00a0}' | '\u{2007}' | '\u{2009}' | '\u{202f}') => {
                end += 1;
                while characters.get(end).is_some_and(|value| value.is_numeric()) {
                    end += 1;
                }
                if end != separator + 4 {
                    end = separator;
                    break;
                }
            }
            _ => break,
        }
    }
    Some(exponent_end(characters, end, 'e').unwrap_or(end))
}

fn numbers_are_field_bound(output: &str, value_ranges: &[std::ops::Range<usize>]) -> bool {
    let (mut offsets, characters): (Vec<_>, Vec<_>) = output.char_indices().unzip();
    offsets.push(output.len());
    let mut start = 0;
    let mut ranges = value_ranges.iter().peekable();
    while start < characters.len() {
        let Some(end) = number_end(&characters, start) else {
            start += 1;
            continue;
        };
        while ranges
            .peek()
            .is_some_and(|range| range.end <= offsets[start])
        {
            ranges.next();
        }
        if !ranges
            .peek()
            .is_some_and(|range| range.start <= offsets[start] && offsets[end] <= range.end)
        {
            return false;
        }
        start = end;
    }
    true
}

impl StructuredAnswer {
    pub fn prose(text: String, cited_evidence_ids: Vec<String>) -> Self {
        Self {
            kind: AnswerKind::Grounded,
            parts: if text.is_empty() {
                Vec::new()
            } else {
                vec![AnswerPart::Prose { text }]
            },
            cited_evidence_ids,
        }
    }

    pub fn render(&self, evidence: &[Evidence]) -> Result<String, AnswerViolation> {
        if self.parts.len() > 512 || self.cited_evidence_ids.len() > 100 {
            return Err(AnswerViolation::Format);
        }
        let ids: BTreeSet<_> = self.cited_evidence_ids.iter().collect();
        if ids.len() != self.cited_evidence_ids.len()
            || ids.iter().any(|id| {
                evidence
                    .iter()
                    .filter(|item| &item.evidence_id == *id)
                    .count()
                    != 1
            })
        {
            return Err(AnswerViolation::Citation);
        }
        if self.kind == AnswerKind::Social
            && (!ids.is_empty()
                || self
                    .parts
                    .iter()
                    .any(|part| !matches!(part, AnswerPart::Prose { .. })))
        {
            return Err(AnswerViolation::Format);
        }
        if self.kind == AnswerKind::Grounded && (self.parts.is_empty() != ids.is_empty()) {
            return Err(AnswerViolation::Citation);
        }
        let mut output = String::new();
        let mut value_ranges = Vec::new();
        for part in &self.parts {
            match part {
                AnswerPart::Prose { text } => {
                    if text.chars().any(char::is_numeric) {
                        return Err(AnswerViolation::Format);
                    }
                    output.push_str(text);
                }
                AnswerPart::GameValue {
                    evidence_id,
                    field,
                    value,
                }
                | AnswerPart::Other {
                    evidence_id,
                    field,
                    value,
                } => {
                    if !ids.contains(evidence_id) {
                        return Err(AnswerViolation::Citation);
                    }
                    let item = evidence
                        .iter()
                        .find(|item| &item.evidence_id == evidence_id)
                        .ok_or(AnswerViolation::Citation)?;
                    let game = matches!(part, AnswerPart::GameValue { .. });
                    if game && !is_game_source(item) {
                        return Err(AnswerViolation::Value);
                    }
                    if !citable_values(item).iter().any(|candidate| {
                        candidate.field == *field
                            && candidate.value == *value
                            && (candidate.kind == "game_value") == game
                    }) {
                        return Err(AnswerViolation::Value);
                    }
                    let start = output.len();
                    output.push_str(value);
                    value_ranges.push(start..output.len());
                }
            }
            if output.len() > 64 * 1024 {
                return Err(AnswerViolation::Format);
            }
        }
        if !self.parts.is_empty() && output.trim().is_empty() {
            return Err(AnswerViolation::Format);
        }
        if !numbers_are_field_bound(&output, &value_ranges) {
            return Err(AnswerViolation::Value);
        }
        Ok(output)
    }
}
