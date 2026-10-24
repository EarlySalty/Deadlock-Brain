use brain_contracts::Evidence;
use serde_json::Value;

const PREFIX: &str = "ev_";
const MIN_ID_LEN: usize = 24;

pub(super) struct EvidenceAliases {
    ids: Vec<Option<String>>,
}

impl EvidenceAliases {
    pub(super) fn shorten(payload: &mut Value, evidence: &[Evidence]) -> Self {
        if mentions_prefix(payload) {
            return Self { ids: Vec::new() };
        }
        let ids = evidence
            .iter()
            .enumerate()
            .map(|(index, item)| Some(item.evidence_id.clone()).filter(|id| aliasable(id, index)))
            .collect();
        let aliases = Self { ids };
        aliases.replace(payload);
        aliases
    }

    pub(super) fn restore(&self, bytes: Vec<u8>) -> Vec<u8> {
        if self.ids.iter().all(Option::is_none) {
            return bytes;
        }
        let mut out = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            match self.alias_at(&bytes, index) {
                Some((id, end)) => {
                    out.extend_from_slice(id.as_bytes());
                    index = end;
                }
                None => {
                    out.push(bytes[index]);
                    index += 1;
                }
            }
        }
        out
    }

    fn alias_at(&self, bytes: &[u8], start: usize) -> Option<(&str, usize)> {
        if !bytes[start..].starts_with(PREFIX.as_bytes()) {
            return None;
        }
        if start
            .checked_sub(1)
            .and_then(|before| bytes.get(before))
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        {
            return None;
        }
        let digits = start + PREFIX.len();
        let end = bytes[digits..]
            .iter()
            .position(|byte| !byte.is_ascii_digit())
            .map_or(bytes.len(), |offset| digits + offset);
        if end == digits
            || bytes[digits] == b'0'
            || bytes
                .get(end)
                .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_')
        {
            return None;
        }
        let number: usize = std::str::from_utf8(&bytes[digits..end])
            .ok()?
            .parse()
            .ok()?;
        let id = self.ids.get(number.checked_sub(1)?)?.as_deref()?;
        Some((id, end))
    }

    fn replace(&self, value: &mut Value) {
        match value {
            Value::String(text) => {
                if let Some(alias) = self.exact(text) {
                    *text = alias;
                } else {
                    *text = self.quoted(text);
                }
            }
            Value::Array(values) => values.iter_mut().for_each(|value| self.replace(value)),
            Value::Object(fields) => fields.values_mut().for_each(|value| self.replace(value)),
            _ => {}
        }
    }

    fn exact(&self, text: &str) -> Option<String> {
        self.ids
            .iter()
            .position(|id| id.as_deref() == Some(text))
            .map(alias)
    }

    fn quoted(&self, text: &str) -> String {
        self.ids
            .iter()
            .enumerate()
            .filter_map(|(index, id)| id.as_deref().map(|id| (index, id)))
            .fold(text.to_owned(), |text, (index, id)| {
                text.replace(&format!("\"{id}\""), &format!("\"{}\"", alias(index)))
            })
    }
}

fn alias(index: usize) -> String {
    format!("{PREFIX}{}", index + 1)
}

fn aliasable(id: &str, index: usize) -> bool {
    id.len() >= MIN_ID_LEN.max(alias(index).len() + 1)
        && !id.contains(['"', '\\'])
        && !id.chars().any(char::is_control)
}

fn mentions_prefix(value: &Value) -> bool {
    match value {
        Value::String(text) => text.contains(PREFIX),
        Value::Array(values) => values.iter().any(mentions_prefix),
        Value::Object(fields) => fields
            .iter()
            .any(|(key, value)| key.contains(PREFIX) || mentions_prefix(value)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{EvidenceKind, SourceVisibility};
    use serde_json::json;

    fn evidence(id: &str) -> Evidence {
        Evidence {
            evidence_id: id.into(),
            citation: id.into(),
            source_id: "source".into(),
            logical_id: "logical".into(),
            revision: 1,
            kind: EvidenceKind::Fact,
            content: String::new(),
            visibility: SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            provenance: None,
            patch: None,
        }
    }

    #[test]
    fn long_ids_travel_short_and_come_back_exactly() {
        let long = format!("gamefacts.receipt.v1:sha256:{}", "a".repeat(64));
        let prefix = "docs/page/1#chunk-000001";
        let items = [
            evidence(&long),
            evidence(prefix),
            evidence("docs/page/1#chunk-0000012"),
        ];
        let mut payload = json!({"messages":[{"content":format!("IDs: [\"{long}\",\"docs/page/1#chunk-0000012\"] {{\"citation\":\"{prefix}\"}}")},
            {"id":long.clone()}]});
        let aliases = EvidenceAliases::shorten(&mut payload, &items);
        assert_eq!(
            payload["messages"][0]["content"],
            "IDs: [\"ev_1\",\"ev_3\"] {\"citation\":\"ev_2\"}"
        );
        assert_eq!(payload["messages"][1]["id"], "ev_1");
        let answer = br#"{"text":"{\"cited_evidence_ids\":[\"ev_1\",\"ev_3\"],\"text\":\"ev_10 ev_01 xev_1 ev_2a ev_2\"}"}"#;
        let restored = String::from_utf8(aliases.restore(answer.to_vec())).unwrap_or_default();
        assert_eq!(
            restored,
            format!(
                r#"{{"text":"{{\"cited_evidence_ids\":[\"{long}\",\"docs/page/1#chunk-0000012\"],\"text\":\"ev_10 ev_01 xev_1 ev_2a {prefix}\"}}"}}"#
            )
        );
    }

    #[test]
    fn payloads_that_already_use_the_prefix_stay_untouched() {
        let items = [evidence("doc/1")];
        let mut payload = json!({"content":"ev_1 \"doc/1\""});
        let aliases = EvidenceAliases::shorten(&mut payload, &items);
        assert_eq!(payload["content"], "ev_1 \"doc/1\"");
        assert_eq!(aliases.restore(b"ev_1".to_vec()), b"ev_1".to_vec());
        let mut short = json!({"content":"\"e1\""});
        let aliases = EvidenceAliases::shorten(&mut short, &[evidence("e1")]);
        assert_eq!(short["content"], "\"e1\"");
        assert_eq!(aliases.restore(b"ev_1".to_vec()), b"ev_1".to_vec());
    }
}
