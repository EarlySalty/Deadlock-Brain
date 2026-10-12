use super::json_text;
use anyhow::{ensure, Result};
use serde_json::Value;

pub struct Leaf {
    pub pointer: String,
    pub value: Value,
    pub lexeme: String,
    pub numeric: bool,
}

pub fn parse(input: &str) -> Result<Vec<Leaf>> {
    ensure!(input.len() <= 2 * 1024 * 1024, "assets_record_limit");
    Ok(json_text::parse(input)
        .map_err(anyhow::Error::msg)?
        .into_iter()
        .map(|leaf| Leaf {
            pointer: leaf.pointer,
            value: leaf.value,
            lexeme: leaf.lexeme,
            numeric: leaf.numeric_representation.is_some(),
        })
        .collect())
}

pub fn number(value: &str) -> bool {
    value.len() <= 64 && json_text::number(value).is_ok()
}
