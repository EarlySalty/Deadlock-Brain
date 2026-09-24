use serde_json::Value;

pub(crate) fn value_is_python_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(number) => {
            number.as_i64().map(|value| value != 0).unwrap_or(true)
                && number.as_u64().map(|value| value != 0).unwrap_or(true)
                && number.as_f64().map(|value| value != 0.0).unwrap_or(true)
        }
        Value::String(value) => !value.is_empty(),
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => !value.is_empty(),
    }
}

pub(crate) fn python_or_string(value: Option<&Value>) -> Option<String> {
    value
        .filter(|value| value_is_python_truthy(value))
        .map(value_to_python_string)
}

pub(crate) fn value_to_python_string(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(_) | Value::Object(_) => value.to_string(),
    }
}

pub(crate) fn form_urlencode(params: &[(&str, String)]) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", quote_plus(key), quote_plus(value)))
        .collect::<Vec<_>>()
        .join("&")
}

pub(crate) fn quote_path(value: &str) -> String {
    percent_encode(value, false, true)
}

fn quote_plus(value: &str) -> String {
    percent_encode(value, true, false)
}

fn percent_encode(value: &str, plus_for_space: bool, safe_slash: bool) -> String {
    let mut output = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-' | b'~' => {
                output.push(byte as char)
            }
            b'/' if safe_slash => output.push('/'),
            b' ' if plus_for_space => output.push('+'),
            _ => {
                output.push('%');
                output.push(upper_hex(byte >> 4));
                output.push(upper_hex(byte & 0x0f));
            }
        }
    }
    output
}

fn upper_hex(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'A' + (nibble - 10)) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_urlencode_matches_urllib_quote_plus_for_used_values() {
        let encoded = form_urlencode(&[
            ("rank", "rank_10,rank_11".to_string()),
            ("teamComp", "Average Comp".to_string()),
            ("hero", "Mo & Krill".to_string()),
        ]);

        assert_eq!(
            encoded,
            "rank=rank_10%2Crank_11&teamComp=Average+Comp&hero=Mo+%26+Krill"
        );
    }

    #[test]
    fn python_or_string_treats_zero_as_falsy_for_or_expressions() {
        assert_eq!(python_or_string(Some(&serde_json::json!(0))), None);
        assert_eq!(
            python_or_string(Some(&serde_json::json!("0"))),
            Some("0".to_string())
        );
    }
}
