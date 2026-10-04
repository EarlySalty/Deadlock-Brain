use std::collections::BTreeMap;

use super::budget::{escaped_len, Budget};
use serde_json::Value;

#[derive(Debug)]
pub(super) struct Leaf {
    pub pointer: String,
    pub value: Value,
    pub flags: Vec<String>,
    pub lexeme: String,
    pub numeric_representation: Option<&'static str>,
}

#[derive(Clone, Debug)]
enum Token {
    OpenObject,
    CloseObject,
    OpenArray,
    CloseArray,
    Equals,
    Comma,
    Colon,
    String(String, String),
    Word(String),
}

pub(super) fn parse(input: &str) -> Result<Vec<Leaf>, &'static str> {
    let body = if input.starts_with("<!--") {
        let end = input.find("-->").ok_or("Nicht geschlossener KV3-Kopf")?;
        &input[end + 3..]
    } else {
        input
    };
    let mut budget = Budget::new();
    let tokens = tokenize(body, &mut budget)?;
    let mut parser = Parser {
        budget: &mut budget,
        tokens: &tokens,
        index: 0,
        leaves: Vec::new(),
    };
    parser.value("", &[], 0)?;
    if parser.index != tokens.len() {
        return Err("Nicht vollständig ausgewertete KV3-Tokens");
    }
    Ok(parser.leaves)
}

struct Parser<'a> {
    budget: &'a mut Budget,
    tokens: &'a [Token],
    index: usize,
    leaves: Vec<Leaf>,
}

impl Parser<'_> {
    fn value(&mut self, pointer: &str, flags: &[String], depth: usize) -> Result<(), &'static str> {
        if depth > 128 {
            return Err("KV3-Verschachtelung überschreitet die Grenze");
        }
        if matches!(self.tokens.get(self.index + 1), Some(Token::Colon)) {
            if let Some(Token::Word(flag)) = self.tokens.get(self.index) {
                self.budget
                    .expanded(&[flag.len(), flags.iter().map(String::len).sum()], 128)?;
                let mut child_flags = flags.to_vec();
                child_flags.push(flag.clone());
                self.index += 2;
                return self.value(pointer, &child_flags, depth + 1);
            }
        }
        match self.tokens.get(self.index) {
            Some(Token::OpenObject) => {
                self.index += 1;
                let mut occurrences = BTreeMap::<String, usize>::new();
                loop {
                    if matches!(self.tokens.get(self.index), Some(Token::CloseObject)) {
                        self.index += 1;
                        return Ok(());
                    }
                    let key = match self.tokens.get(self.index) {
                        Some(Token::Word(key)) | Some(Token::String(key, _)) => {
                            self.budget
                                .expanded(&[key.len(), escaped_len(key), pointer.len()], 128)?;
                            key.clone()
                        }
                        _ => return Err("Ungültiger KV3-Objektschlüssel"),
                    };
                    self.index += 1;
                    if !matches!(self.tokens.get(self.index), Some(Token::Equals)) {
                        return Err("Fehlendes KV3-Gleichheitszeichen");
                    }
                    self.index += 1;
                    let occurrence = occurrences.entry(key.clone()).or_default();
                    let escaped = key.replace('~', "~0").replace('/', "~1");
                    let child_pointer = format!("{pointer}/{escaped}/{}", *occurrence);
                    *occurrence += 1;
                    self.value(&child_pointer, flags, depth + 1)?;
                    if matches!(self.tokens.get(self.index), Some(Token::Comma)) {
                        self.index += 1;
                    }
                }
            }
            Some(Token::OpenArray) => {
                self.index += 1;
                let mut element = 0;
                loop {
                    if matches!(self.tokens.get(self.index), Some(Token::CloseArray)) {
                        self.index += 1;
                        return Ok(());
                    }
                    self.budget.expanded(&[pointer.len()], 128)?;
                    self.value(&format!("{pointer}/[{element}]"), flags, depth + 1)?;
                    element += 1;
                    if matches!(self.tokens.get(self.index), Some(Token::Comma)) {
                        self.index += 1;
                    } else if !matches!(self.tokens.get(self.index), Some(Token::CloseArray)) {
                        return Err("Fehlendes KV3-Arraytrennzeichen");
                    }
                }
            }
            Some(Token::String(value, lexeme)) => {
                self.budget.expanded(
                    &[
                        pointer.len(),
                        value.len(),
                        lexeme.len(),
                        flags.iter().map(String::len).sum(),
                    ],
                    256 + flags.len() * 128,
                )?;
                self.leaves.push(Leaf {
                    numeric_representation: None,
                    pointer: pointer.to_owned(),
                    value: Value::String(value.clone()),
                    flags: flags.to_vec(),
                    lexeme: lexeme.clone(),
                });
                self.index += 1;
                Ok(())
            }
            Some(Token::Word(word)) => {
                self.budget.expanded(
                    &[
                        pointer.len(),
                        word.len(),
                        flags.iter().map(String::len).sum(),
                    ],
                    256 + flags.len() * 128,
                )?;
                let (value, numeric_representation) = match word.as_str() {
                    "true" => (Value::Bool(true), None),
                    "false" => (Value::Bool(false), None),
                    "null" => (Value::Null, None),
                    _ => super::json_text::number(word)?,
                };
                self.leaves.push(Leaf {
                    numeric_representation,
                    pointer: pointer.to_owned(),
                    value,
                    flags: flags.to_vec(),
                    lexeme: word.clone(),
                });
                self.index += 1;
                Ok(())
            }
            _ => Err("Fehlender KV3-Wert"),
        }
    }
}

fn tokenize(input: &str, budget: &mut Budget) -> Result<Vec<Token>, &'static str> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index].is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if bytes[index..].starts_with(b"//") {
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if bytes[index..].starts_with(b"/*") {
            index += 2;
            while index + 1 < bytes.len() && !bytes[index..].starts_with(b"*/") {
                index += 1;
            }
            if index + 1 >= bytes.len() {
                return Err("Nicht geschlossener KV3-Kommentar");
            }
            index += 2;
            continue;
        }
        budget.charge(128)?;
        let token = match bytes[index] {
            b'{' => Token::OpenObject,
            b'}' => Token::CloseObject,
            b'[' => Token::OpenArray,
            b']' => Token::CloseArray,
            b'=' => Token::Equals,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b'"' => {
                let start = index;
                if bytes[index..].starts_with(b"\"\"\"") {
                    index += 3;
                    let value_start = index;
                    while index + 2 < bytes.len() && !bytes[index..].starts_with(b"\"\"\"") {
                        index += 1;
                    }
                    if index + 2 >= bytes.len() {
                        return Err("Nicht geschlossene KV3-Mehrzeilenzeichenfolge");
                    }
                    budget.expanded(&[index - start + 3], 0)?;
                    let value = input[value_start..index].to_owned();
                    index += 3;
                    tokens.push(Token::String(value, input[start..index].to_owned()));
                    continue;
                }
                index += 1;
                loop {
                    if index >= bytes.len() {
                        return Err("Nicht geschlossene KV3-Zeichenfolge");
                    }
                    match bytes[index] {
                        b'\\' => {
                            index += 2;
                            if index > bytes.len() {
                                return Err("Unvollständige KV3-Escapesequenz");
                            }
                        }
                        b'"' => {
                            index += 1;
                            break;
                        }
                        _ => index += 1,
                    }
                }
                budget.expanded(&[index - start], 0)?;
                let lexeme = &input[start..index];
                let value: String = serde_json::from_str(lexeme)
                    .map_err(|_| "Nicht unterstützte KV3-Zeichenfolgenkodierung")?;
                tokens.push(Token::String(value, lexeme.to_owned()));
                continue;
            }
            _ => {
                let start = index;
                while index < bytes.len()
                    && !bytes[index].is_ascii_whitespace()
                    && !matches!(
                        bytes[index],
                        b'{' | b'}' | b'[' | b']' | b'=' | b',' | b':' | b'"'
                    )
                {
                    index += 1;
                }
                if start == index {
                    return Err("Unbekanntes KV3-Zeichen");
                }
                budget.charge(index - start)?;
                tokens.push(Token::Word(input[start..index].to_owned()));
                continue;
            }
        };
        tokens.push(token);
        index += 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source2_arrays_flags_and_duplicate_properties_are_preserved() {
        let leaves = parse(r#"<!-- kv3 encoding:text --> { _include = [ resource_name:"scripts/abilities/hero.vdata_inc", ] cooldown = 12 cooldown = 13 flags = [ true, null ] }"#).unwrap();
        assert_eq!(leaves.len(), 5);
        assert_eq!(leaves[0].flags, ["resource_name"]);
        assert_eq!(leaves[0].value, "scripts/abilities/hero.vdata_inc");
        assert_eq!(leaves[1].value, 12);
        assert_ne!(leaves[1].pointer, leaves[2].pointer);
        assert_eq!(leaves[4].value, Value::Null);
    }

    #[test]
    fn malformed_and_unsupported_syntax_is_rejected_whole() {
        assert!(parse("{ value = invalid }").is_err());
        assert!(parse("{ value = [ 1 2 ] }").is_err());
        assert!(parse("{ value = 1").is_err());
        assert!(parse("{ value = #[ ff ] }").is_err());
    }
}
