//! JSON objects with duplicate keys are ambiguous source evidence, not a
//! last-key-wins success. serde_json retains its normal recursion/number limits.
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::{value::RawValue, Value};
use std::collections::BTreeSet;

struct UniqueObject<'a>(Vec<(String, &'a RawValue)>);
impl<'de> Deserialize<'de> for UniqueObject<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct UniqueVisitor;
        impl<'de> Visitor<'de> for UniqueVisitor {
            type Value = UniqueObject<'de>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("JSON with unique object keys")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut keys = BTreeSet::new();
                let mut values = Vec::new();
                while let Some((key, value)) = map.next_entry::<String, &'de RawValue>()? {
                    if !keys.insert(key.clone()) {
                        return Err(serde::de::Error::custom("duplicate JSON object key"));
                    }
                    values.push((key, value));
                }
                Ok(UniqueObject(values))
            }
        }
        d.deserialize_map(UniqueVisitor)
    }
}

fn parse_value(raw: &RawValue, depth: usize) -> Result<Value, serde_json::Error> {
    if depth > 128 {
        return Err(serde::de::Error::custom("JSON recursion limit"));
    }
    let text = raw.get().trim();
    match text.as_bytes().first() {
        Some(b'{') => {
            let UniqueObject(entries) = serde_json::from_str(text)?;
            let mut values = serde_json::Map::new();
            for (key, value) in entries {
                values.insert(key, parse_value(value, depth + 1)?);
            }
            Ok(Value::Object(values))
        }
        Some(b'[') => {
            let entries: Vec<&RawValue> = serde_json::from_str(text)?;
            entries
                .into_iter()
                .map(|value| parse_value(value, depth + 1))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array)
        }
        Some(b'-' | b'0'..=b'9') => serde_json::from_str(text).map(Value::Number),
        _ => serde_json::from_str(text),
    }
}

pub(super) fn parse(raw: &[u8]) -> Result<Value, serde_json::Error> {
    let raw: &RawValue = serde_json::from_slice(raw)?;
    parse_value(raw, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_fractional_negative_zero_and_large_json_numbers() {
        let raw = br#"{"fraction":0.1,"negative":-0.25,"zero":0.0,"negative_zero":-0.0,"large":18446744073709551616,"nested":[0.125,1e-3]}"#;
        let parsed = parse(raw).unwrap();
        for key in ["fraction", "negative", "zero", "negative_zero", "large"] {
            assert!(parsed[key].is_number(), "{key}: {:?}", parsed[key]);
        }
        assert_eq!(parsed["fraction"].as_f64(), Some(0.1));
        assert_eq!(parsed["negative"].as_f64(), Some(-0.25));
        assert_eq!(parsed["large"].to_string(), "18446744073709551616");
        assert!(parsed["nested"][0].is_number());
        assert_eq!(parsed["nested"][1].as_f64(), Some(0.001));
    }

    #[test]
    fn duplicate_guard_still_rejects_nested_and_escaped_equivalent_keys() {
        for raw in [
            br#"{"a":0.1,"a":0.2}"#.as_slice(),
            br#"{"nested":{"a":0.1,"a":0.2}}"#,
            br#"{"a":1,"a":2}"#,
        ] {
            assert!(parse(raw).is_err());
        }
        let escaped = format!(r#"{{"a":1,"{}u0061":2}}"#, char::from(92));
        assert!(parse(escaped.as_bytes()).is_err());
    }

    #[test]
    fn private_looking_json_object_keys_are_not_number_payloads() {
        let raw = br#"{"$serde_json::private::Number":"0.1","ordinary":2}"#;
        let parsed = parse(raw).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed["$serde_json::private::Number"], "0.1");
        assert_eq!(parsed["ordinary"], 2);
    }
}
