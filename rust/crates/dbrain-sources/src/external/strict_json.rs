use serde_json::Value;

pub(super) fn parse(raw: &[u8]) -> Result<Value, serde_json::Error> {
    brain_contracts::provider_input::parse_unique_json(raw)
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
