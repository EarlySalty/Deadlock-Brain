use super::{
    budget::{escaped_len, Budget},
    kv3::Leaf,
};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn number(text: &str) -> Result<(Value, Option<&'static str>), &'static str> {
    let b = text.as_bytes();
    let mut i = usize::from(b.first() == Some(&b'-'));
    if b.get(i) == Some(&b'0') {
        i += 1;
    } else {
        if !b.get(i).is_some_and(|v| matches!(v, b'1'..=b'9')) {
            return Err("invalid_numeric_lexeme");
        }
        while b.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
    }
    if b.get(i) == Some(&b'.') {
        i += 1;
        let start = i;
        while b.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if i == start {
            return Err("invalid_numeric_lexeme");
        }
    }
    if b.get(i).is_some_and(|v| matches!(v, b'e' | b'E')) {
        i += 1;
        if b.get(i).is_some_and(|v| matches!(v, b'+' | b'-')) {
            i += 1;
        }
        let start = i;
        while b.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if i == start {
            return Err("invalid_numeric_lexeme");
        }
    }
    if i != b.len() {
        return Err("invalid_numeric_lexeme");
    }
    // Integer representations are exact. Decimal/exponent lexemes deliberately
    // remain strings: no IEEE-754 rounding is presented as a source value.
    if !text.contains(['.', 'e', 'E']) && text != "-0" {
        if let Ok(n) = text.parse::<i64>() {
            return Ok((Value::from(n), Some("exact_integer")));
        }
        if let Ok(n) = text.parse::<u64>() {
            return Ok((Value::from(n), Some("exact_integer")));
        }
    }
    Ok((
        Value::String(text.to_owned()),
        Some("source_numeric_lexeme"),
    ))
}

pub(super) fn parse(input: &str) -> Result<Vec<Leaf>, &'static str> {
    let mut p = Parser {
        input,
        index: 0,
        budget: Budget::new(),
        leaves: Vec::new(),
    };
    p.value("", 0)?;
    p.space();
    if p.index != input.len() {
        return Err("invalid_json_preserved_as_text");
    }
    Ok(p.leaves)
}
struct Parser<'a> {
    input: &'a str,
    index: usize,
    budget: Budget,
    leaves: Vec<Leaf>,
}
impl Parser<'_> {
    fn space(&mut self) {
        while self
            .input
            .as_bytes()
            .get(self.index)
            .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
        {
            self.index += 1;
        }
    }
    fn consume(&mut self, b: u8) -> bool {
        self.space();
        if self.input.as_bytes().get(self.index) == Some(&b) {
            self.index += 1;
            true
        } else {
            false
        }
    }
    fn string(&mut self) -> Result<(String, String), &'static str> {
        self.space();
        let start = self.index;
        if !self.consume(b'"') {
            return Err("invalid_json_preserved_as_text");
        }
        loop {
            match self.input.as_bytes().get(self.index) {
                Some(b'\\') => self.index += 2,
                Some(b'"') => {
                    self.index += 1;
                    break;
                }
                Some(_) => self.index += 1,
                None => return Err("invalid_json_preserved_as_text"),
            }
        }
        let lexeme = self
            .input
            .get(start..self.index)
            .ok_or("invalid_json_preserved_as_text")?;
        self.budget.expanded(&[lexeme.len()], 128)?;
        let value =
            serde_json::from_str::<String>(lexeme).map_err(|_| "invalid_json_preserved_as_text")?;
        Ok((value, lexeme.to_owned()))
    }
    fn value(&mut self, pointer: &str, depth: usize) -> Result<(), &'static str> {
        if depth > 128 {
            return Err("json_depth_exceeded_preserved_as_text");
        }
        self.space();
        if self.consume(b'{') {
            let mut keys = BTreeSet::new();
            if self.consume(b'}') {
                return Ok(());
            }
            loop {
                let (key, _) = self.string()?;
                self.budget
                    .expanded(&[key.len(), pointer.len(), escaped_len(&key)], 128)?;
                if !keys.insert(key.clone()) {
                    return Err("duplicate_json_keys_preserved_as_text");
                }
                if !self.consume(b':') {
                    return Err("invalid_json_preserved_as_text");
                }
                let escaped = key.replace('~', "~0").replace('/', "~1");
                self.value(&format!("{pointer}/{escaped}"), depth + 1)?;
                if self.consume(b'}') {
                    return Ok(());
                }
                if !self.consume(b',') {
                    return Err("invalid_json_preserved_as_text");
                }
            }
        }
        if self.consume(b'[') {
            if self.consume(b']') {
                return Ok(());
            }
            let mut index = 0usize;
            loop {
                self.budget.expanded(&[pointer.len()], 128)?;
                self.value(&format!("{pointer}/{index}"), depth + 1)?;
                index += 1;
                if self.consume(b']') {
                    return Ok(());
                }
                if !self.consume(b',') {
                    return Err("invalid_json_preserved_as_text");
                }
            }
        }
        self.budget.expanded(&[pointer.len()], 1536)?;
        let (value, lexeme, numeric_representation) =
            if self.input.as_bytes().get(self.index) == Some(&b'"') {
                let (value, lexeme) = self.string()?;
                (Value::String(value), lexeme, None)
            } else {
                let start = self.index;
                while self.input.as_bytes().get(self.index).is_some_and(|b| {
                    !matches!(b, b' ' | b'\t' | b'\r' | b'\n' | b',' | b']' | b'}')
                }) {
                    self.index += 1;
                }
                let text = &self.input[start..self.index];
                self.budget.expanded(&[text.len()], 0)?;
                let (value, numeric) = match text {
                    "true" => (Value::Bool(true), None),
                    "false" => (Value::Bool(false), None),
                    "null" => (Value::Null, None),
                    _ => number(text)?,
                };
                (value, text.to_owned(), numeric)
            };
        self.leaves.push(Leaf {
            pointer: pointer.to_owned(),
            value,
            lexeme,
            numeric_representation,
            flags: Vec::new(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_keys_abort_whole_document() {
        assert_eq!(
            parse(r#"{"x":1,"x":2}"#).unwrap_err(),
            "duplicate_json_keys_preserved_as_text"
        );
        assert!(parse(r#"{"x":1,"x":2}"#).is_err());
        assert!(parse(r#"{"a":{"x":1},"b":{"x":2}}"#).is_ok());
    }
    #[test]
    fn numeric_lexemes_are_never_rounded() {
        for s in [
            "18446744073709551616",
            "0.12345678901234567890123456789",
            "1e999",
            "-0",
        ] {
            let (v, q) = number(s).unwrap();
            assert_eq!(v, s);
            assert_eq!(q, Some("source_numeric_lexeme"));
        }
        assert_eq!(number("12").unwrap().0, 12);
        for s in ["", "01", "1.", ".1", "1e", "--1"] {
            assert!(number(s).is_err());
        }
    }
}
