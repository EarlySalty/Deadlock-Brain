#![forbid(unsafe_code)]

pub mod mode;
pub use mode::{JevMode, ShadowObservation};

use std::collections::BTreeSet;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{Map, Value};
use thiserror::Error;

const EPSILON: f64 = 1e-9;

#[derive(Debug, Error)]
pub enum JevError {
    #[error("JSON ungültig: {0}")]
    Json(String),
    #[error("Jev Vertrag ungültig: {0}")]
    Contract(String),
}

pub type Result<T> = std::result::Result<T, JevError>;

pub fn validate_json(request_raw: &str, response_raw: &str) -> Result<Value> {
    let request = parse_strict(request_raw)?;
    let response = parse_strict(response_raw)?;
    validate_response(&request, &response)?;
    Ok(response)
}

pub fn parse_strict(raw: &str) -> Result<Value> {
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    let StrictValue(value) = StrictValue::deserialize(&mut deserializer)
        .map_err(|error| JevError::Json(error.to_string()))?;
    deserializer
        .end()
        .map_err(|error| JevError::Json(error.to_string()))?;
    Ok(value)
}

pub fn validate_response(request: &Value, response: &Value) -> Result<()> {
    let request_object = object(request, "request")?;
    let response_object = object(response, "response")?;
    require_fields(
        response_object,
        &["model", "answers"],
        &["model", "answers", "usage"],
        "response",
    )?;

    let request_model = string_field(request_object, "model", "request")?;
    let response_model = string_field(response_object, "model", "response")?;
    if request_model != response_model {
        return Err(contract("model mismatch"));
    }

    let questions = object_field(request_object, "questions", "request")?;
    let answers = object_field(response_object, "answers", "response")?;
    let expected_ids: BTreeSet<_> = questions.keys().collect();
    let actual_ids: BTreeSet<_> = answers.keys().collect();
    if expected_ids != actual_ids {
        return Err(contract("answer question set mismatch"));
    }

    for (question_id, question) in questions {
        validate_answer(
            question_id,
            object(question, &format!("question {question_id}"))?,
            object(
                answers
                    .get(question_id)
                    .ok_or_else(|| contract("missing answer"))?,
                &format!("answer {question_id}"),
            )?,
        )?;
    }

    if let Some(usage) = response_object.get("usage") {
        validate_usage(usage)?;
    }
    Ok(())
}

fn validate_answer(
    question_id: &str,
    question: &Map<String, Value>,
    answer: &Map<String, Value>,
) -> Result<()> {
    let question_type = string_field(question, "type", question_id)?;
    let answer_type = string_field(answer, "type", question_id)?;
    if question_type != answer_type {
        return Err(contract(format!("{question_id}: answer type mismatch")));
    }

    match question_type {
        "noul" => validate_noul(question_id, answer),
        "choice" => validate_choice(question_id, question, answer),
        "score" => validate_score(question_id, question, answer),
        other => Err(contract(format!("{question_id}: unsupported type {other}"))),
    }
}

fn validate_noul(question_id: &str, answer: &Map<String, Value>) -> Result<()> {
    require_fields(answer, &["type", "noul"], &["type", "noul"], question_id)?;
    let value = finite_number(
        answer
            .get("noul")
            .ok_or_else(|| contract(format!("{question_id}: missing noul")))?,
        question_id,
    )?;
    if !(0.0..=1.0).contains(&value) {
        return Err(contract(format!("{question_id}: noul out of range")));
    }
    Ok(())
}

fn validate_choice(
    question_id: &str,
    question: &Map<String, Value>,
    answer: &Map<String, Value>,
) -> Result<()> {
    require_fields(
        answer,
        &["type", "choice", "probabilities", "confidence"],
        &["type", "choice", "probabilities", "confidence"],
        question_id,
    )?;
    let criteria = object_field(question, "criteria", question_id)?;
    let labels: BTreeSet<_> = criteria.keys().cloned().collect();
    let choice = string_field(answer, "choice", question_id)?;
    if !labels.contains(choice) {
        return Err(contract(format!("{question_id}: unknown choice")));
    }
    let probabilities = object_field(answer, "probabilities", question_id)?;
    validate_probabilities(question_id, &labels, probabilities)?;
    let selected = finite_number(
        probabilities
            .get(choice)
            .ok_or_else(|| contract(format!("{question_id}: missing choice probability")))?,
        question_id,
    )?;
    let maximum = probabilities
        .values()
        .map(|value| finite_number(value, question_id))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max);
    if selected + EPSILON < maximum {
        return Err(contract(format!("{question_id}: choice is not maximum")));
    }
    validate_confidence(question_id, answer)
}

fn validate_score(
    question_id: &str,
    question: &Map<String, Value>,
    answer: &Map<String, Value>,
) -> Result<()> {
    require_fields(
        answer,
        &["type", "score", "legend", "probabilities", "confidence"],
        &["type", "score", "legend", "probabilities", "confidence"],
        question_id,
    )?;
    let criteria = question
        .get("criteria")
        .and_then(Value::as_array)
        .ok_or_else(|| contract(format!("{question_id}: score criteria must be array")))?;
    if criteria.is_empty() {
        return Err(contract(format!("{question_id}: empty score criteria")));
    }

    let labels: BTreeSet<String> = (0..criteria.len()).map(|index| index.to_string()).collect();
    let legend = object_field(answer, "legend", question_id)?;
    if legend.len() != criteria.len() {
        return Err(contract(format!("{question_id}: legend mismatch")));
    }
    for (index, description) in criteria.iter().enumerate() {
        let expected = description
            .as_str()
            .ok_or_else(|| contract(format!("{question_id}: invalid criterion")))?;
        if legend.get(&index.to_string()).and_then(Value::as_str) != Some(expected) {
            return Err(contract(format!("{question_id}: legend mismatch")));
        }
    }

    let probabilities = object_field(answer, "probabilities", question_id)?;
    validate_probabilities(question_id, &labels, probabilities)?;
    let expected_score = probabilities
        .iter()
        .map(|(label, probability)| {
            let index = label
                .parse::<f64>()
                .map_err(|_| contract(format!("{question_id}: invalid score label")))?;
            Ok(index * finite_number(probability, question_id)?)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<f64>();
    let score = finite_number(
        answer
            .get("score")
            .ok_or_else(|| contract(format!("{question_id}: missing score")))?,
        question_id,
    )?;
    if score < -EPSILON || score > (criteria.len() - 1) as f64 + EPSILON {
        return Err(contract(format!("{question_id}: score out of range")));
    }
    if (score - expected_score).abs() > EPSILON {
        return Err(contract(format!(
            "{question_id}: score expectation mismatch"
        )));
    }
    validate_confidence(question_id, answer)
}

fn validate_probabilities(
    question_id: &str,
    labels: &BTreeSet<String>,
    probabilities: &Map<String, Value>,
) -> Result<()> {
    let actual: BTreeSet<_> = probabilities.keys().cloned().collect();
    if &actual != labels {
        return Err(contract(format!(
            "{question_id}: probability label mismatch"
        )));
    }
    let mut sum = 0.0;
    for value in probabilities.values() {
        let probability = finite_number(value, question_id)?;
        if !(0.0..=1.0).contains(&probability) {
            return Err(contract(format!("{question_id}: probability out of range")));
        }
        sum += probability;
    }
    if (sum - 1.0).abs() > EPSILON {
        return Err(contract(format!(
            "{question_id}: probabilities do not sum to one"
        )));
    }
    Ok(())
}

fn validate_confidence(question_id: &str, answer: &Map<String, Value>) -> Result<()> {
    let confidence = finite_number(
        answer
            .get("confidence")
            .ok_or_else(|| contract(format!("{question_id}: missing confidence")))?,
        question_id,
    )?;
    if !(0.0..=1.0).contains(&confidence) {
        return Err(contract(format!("{question_id}: confidence out of range")));
    }
    Ok(())
}

fn validate_usage(value: &Value) -> Result<()> {
    let usage = object(value, "usage")?;
    require_fields(
        usage,
        &["input_tokens", "output_tokens"],
        &["input_tokens", "output_tokens"],
        "usage",
    )?;
    for field in ["input_tokens", "output_tokens"] {
        if usage.get(field).and_then(Value::as_u64).is_none() {
            return Err(contract(format!("usage: invalid {field}")));
        }
    }
    Ok(())
}

fn require_fields(
    object: &Map<String, Value>,
    required: &[&str],
    allowed: &[&str],
    context: &str,
) -> Result<()> {
    for field in required {
        if !object.contains_key(*field) {
            return Err(contract(format!("{context}: missing field {field}")));
        }
    }
    for field in object.keys() {
        if !allowed.contains(&field.as_str()) {
            return Err(contract(format!("{context}: unknown field {field}")));
        }
    }
    Ok(())
}

fn object<'a>(value: &'a Value, context: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| contract(format!("{context}: expected object")))
}

fn object_field<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a Map<String, Value>> {
    object
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| contract(format!("{context}: {field} must be object")))
}

fn string_field<'a>(object: &'a Map<String, Value>, field: &str, context: &str) -> Result<&'a str> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| contract(format!("{context}: {field} must be nonempty string")))
}

fn finite_number(value: &Value, context: &str) -> Result<f64> {
    value
        .as_f64()
        .filter(|number| number.is_finite())
        .ok_or_else(|| contract(format!("{context}: expected finite number")))
}

fn contract(message: impl Into<String>) -> JevError {
    JevError::Contract(message.into())
}

struct StrictValue(Value);

impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictValueVisitor)
    }
}

struct StrictValueVisitor;

impl<'de> Visitor<'de> for StrictValueVisitor {
    type Value = StrictValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("valid JSON without duplicate object keys")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .map(StrictValue)
            .ok_or_else(|| E::custom("non finite number"))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::String(value.to_string())))
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::String(value)))
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::Null))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E> {
        Ok(StrictValue(Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        StrictValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(StrictValue(value)) = sequence.next_element::<StrictValue>()? {
            values.push(value);
        }
        Ok(StrictValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom(format!("duplicate key {key}")));
            }
            let StrictValue(value) = map.next_value::<StrictValue>()?;
            values.insert(key, value);
        }
        Ok(StrictValue(Value::Object(values)))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    fn fixtures() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("architecture/migration/s07/fixtures")
    }

    #[test]
    fn manifest_jev_fixtures_match_validator_expectations() {
        let root = fixtures();
        let manifest: Value =
            serde_json::from_str(&fs::read_to_string(root.join("MANIFEST.json")).unwrap()).unwrap();
        let files = manifest["files"].as_array().unwrap();

        for entry in files {
            let file = entry["file"].as_str().unwrap();
            let expectation = entry["expectation"].as_str().unwrap();
            let request = entry["request"].as_str();

            if expectation == "reject_json" || expectation == "reject_duplicate_keys" {
                let raw = fs::read_to_string(root.join(file)).unwrap();
                assert!(
                    parse_strict(&raw).is_err(),
                    "{file} should fail strict JSON parsing"
                );
                continue;
            }

            let Some(request_file) = request.filter(|name| name.starts_with("jev-")) else {
                continue;
            };
            if !file.starts_with("jev-") && !file.starts_with("invalid-") {
                continue;
            }

            let request_raw = fs::read_to_string(root.join(request_file)).unwrap();
            let response_raw = fs::read_to_string(root.join(file)).unwrap();
            let result = validate_json(&request_raw, &response_raw);
            if expectation.starts_with("accept") {
                assert!(result.is_ok(), "{file} should be accepted: {result:?}");
            } else if expectation.starts_with("reject") {
                assert!(result.is_err(), "{file} should be rejected");
            }
        }
    }

    #[test]
    fn duplicate_keys_are_rejected_recursively() {
        let raw = r#"{"a":{"b":1,"b":2}}"#;
        assert!(matches!(parse_strict(raw), Err(JevError::Json(_))));
    }
}
