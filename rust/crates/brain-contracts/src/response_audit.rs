use crate::PortError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseDisposition {
    Rejected,
    UncheckedReturned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseCheck {
    pub check: String,
    pub field: String,
    pub expected: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseDeviation {
    pub request_id: String,
    pub model: String,
    pub check: ResponseCheck,
    pub raw_output: Vec<u8>,
    pub raw_output_complete: bool,
    pub source_ids: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub disposition: ResponseDisposition,
    pub identifiers_redacted: bool,
}

pub trait ResponseAuditPort: Send + Sync {
    fn append(&self, deviation: &ResponseDeviation) -> Result<(), PortError>;

    fn append_all(&self, deviations: &[ResponseDeviation]) -> Result<(), PortError> {
        match deviations {
            [] => Ok(()),
            [deviation] => self.append(deviation),
            _ => Err(PortError::Unavailable(
                "atomic response audit unavailable".into(),
            )),
        }
    }
}

fn encoded_digit(text: &str) -> Option<(u8, usize)> {
    let first = *text.as_bytes().first()?;
    if first.is_ascii_digit() {
        return Some((first, 1));
    }
    let slashes = text.bytes().take_while(|byte| *byte == b'\\').count();
    if slashes == 0 {
        return None;
    }
    let code = text.get(slashes..)?.strip_prefix('u')?.get(..4)?;
    let value = u8::from_str_radix(code, 16).ok()?;
    value.is_ascii_digit().then_some((value, slashes + 5))
}

pub fn diagnostic_text(text: &str, person_id: Option<u64>) -> String {
    let person = person_id.map(|id| id.to_string());
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while !rest.is_empty() {
        if rest.starts_with("<@") || rest.starts_with("<#") {
            if let Some((end, _)) = rest
                .char_indices()
                .take(32)
                .find(|(_, value)| *value == '>')
            {
                output.push_str(if end + 1 < "[discord-id]".len() {
                    "?"
                } else {
                    "[discord-id]"
                });
                rest = &rest[end + 1..];
                continue;
            }
        }
        let mut consumed = 0;
        let mut digits = 0;
        let mut matches_person = person.is_some();
        while let Some((digit, length)) = encoded_digit(&rest[consumed..]) {
            matches_person &=
                person.as_ref().and_then(|id| id.as_bytes().get(digits)) == Some(&digit);
            consumed += length;
            digits += 1;
        }
        if consumed > 0 {
            matches_person &= person.as_ref().is_some_and(|id| id.len() == digits);
            if digits >= 15 || matches_person {
                output.push_str(if consumed < "[discord-id]".len() {
                    "?"
                } else {
                    "[discord-id]"
                });
            } else {
                output.push_str(&rest[..consumed]);
            }
            rest = &rest[consumed..];
        } else {
            let length = if rest.starts_with('\\') {
                rest.bytes().take_while(|byte| *byte == b'\\').count()
            } else {
                rest.chars().next().expect("nonempty text").len_utf8()
            };
            output.push_str(&rest[..length]);
            rest = &rest[length..];
        }
    }
    output
}

pub fn diagnostic_bytes(mut bytes: &[u8], person_id: Option<u64>) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    while !bytes.is_empty() {
        match std::str::from_utf8(bytes) {
            Ok(text) => {
                output.extend_from_slice(diagnostic_text(text, person_id).as_bytes());
                break;
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let text = std::str::from_utf8(&bytes[..valid]).expect("valid UTF-8 prefix");
                output.extend_from_slice(diagnostic_text(text, person_id).as_bytes());
                let end = valid + error.error_len().unwrap_or(bytes.len() - valid);
                output.extend_from_slice(&bytes[valid..end]);
                bytes = &bytes[end..];
            }
        }
    }
    output
}

impl ResponseDeviation {
    pub fn validate(&self) -> Result<(), PortError> {
        let metadata = [
            &self.request_id,
            &self.model,
            &self.check.check,
            &self.check.field,
            &self.check.expected,
        ];
        if metadata.iter().any(|text| {
            text.trim().is_empty() || text.len() > 1024 || text.chars().any(char::is_control)
        }) || self.raw_output.len() > 16 * 1024 * 1024
            || self.source_ids.len() > 512
            || self.evidence_ids.len() > 512
            || self
                .source_ids
                .iter()
                .chain(&self.evidence_ids)
                .any(|id| id.is_empty() || id.len() > 1024 || id.chars().any(char::is_control))
            || metadata
                .iter()
                .any(|text| diagnostic_text(text, None) != **text)
            || diagnostic_bytes(&self.raw_output, None) != self.raw_output
            || self
                .source_ids
                .iter()
                .chain(&self.evidence_ids)
                .any(|id| diagnostic_text(id, None) != *id)
        {
            return Err(PortError::InvalidResponse(
                "invalid response audit record".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_output_preserves_invalid_utf8_and_nul_while_redacting_identifiers() {
        let raw = b"\0wire\xff123456789012345678 damage=42";
        assert_eq!(
            diagnostic_bytes(raw, None),
            b"\0wire\xff[discord-id] damage=42"
        );
    }

    #[test]
    fn identifiers_are_removed_without_erasing_small_game_values() {
        for raw in ["<@!123456789012345678>", "<#123456789012345678>", "123456789012345678", "\\u0031\\u0032\\u0033\\u0034\\u0035\\u0036\\u0037\\u0038\\u0039\\u0030\\u0031\\u0032\\u0033\\u0034\\u0035"] {
            let projected = diagnostic_text(raw, None);
            assert!(!projected.contains("123456789"));
            assert_eq!(diagnostic_text(&projected, None), projected);
        }
        let nested: String = b"123456789012345678"
            .iter()
            .map(|byte| format!("\\\\\\\\u{byte:04x}"))
            .collect();
        assert_eq!(diagnostic_text(&nested, None), "[discord-id]");
        let unfinished_mentions = "<@".repeat(16 * 1024);
        assert_eq!(
            diagnostic_text(&unfinished_mentions, None),
            unfinished_mentions
        );
        let slashes = "\\".repeat(256 * 1024);
        let raw = format!("{slashes}x123456789012345678");
        assert_eq!(
            diagnostic_text(&raw, None),
            format!("{slashes}x[discord-id]")
        );
        let raw = format!("{slashes}u0033 item=42");
        assert_eq!(diagnostic_text(&raw, Some(3)), "[discord-id] item=42");
        let original = r#"{"damage":"42", "item":1234}"#;
        assert_eq!(diagnostic_text(original, None), original);
        assert_eq!(
            diagnostic_text("damage=42 item=1234 person=3", Some(3)),
            "damage=42 item=1234 person=?"
        );
        let raw = "<@3>3".repeat(1024);
        let projected = diagnostic_bytes(raw.as_bytes(), Some(3));
        assert!(!projected.contains(&b'3'));
        assert!(projected.len() <= raw.len());
    }
}
