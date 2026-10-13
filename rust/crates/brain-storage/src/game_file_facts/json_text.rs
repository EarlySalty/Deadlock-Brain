use super::{
    budget::{escaped_len, Budget, Selection},
    kv3::Leaf,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

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
    parse_with_budget(input, Budget::new())
}
pub(super) fn parse_with_budget(input: &str, budget: Budget) -> Result<Vec<Leaf>, &'static str> {
    let mut p = Parser {
        input,
        index: 0,
        budget,
        leaves: Vec::new(),
        selection: None,
        active: Vec::new(),
    };
    p.value("", 0)?;
    p.space();
    if p.index != input.len() {
        return Err("invalid_json_preserved_as_text");
    }
    Ok(p.leaves)
}

pub(super) fn selected_hashes(
    input: &str,
    requested: &BTreeSet<&str>,
    mut budget: Budget,
) -> Result<BTreeMap<String, [u8; 32]>, &'static str> {
    let selection = Selection::new(input, requested, 4, &mut budget)?;
    let mut p = Parser {
        input,
        index: 0,
        budget,
        leaves: Vec::new(),
        selection: Some(selection),
        active: requested.iter().map(|r| (*r, *r)).collect(),
    };
    p.value("", 0)?;
    p.space();
    if p.index != input.len() {
        return Err("invalid_json_preserved_as_text");
    }
    Ok(p.selection.map(|s| s.hashes).unwrap_or_default())
}
struct Parser<'a, 'r> {
    input: &'a str,
    index: usize,
    budget: Budget,
    leaves: Vec<Leaf>,
    selection: Option<Selection<'r>>,
    active: Vec<(&'r str, &'r str)>,
}
impl Parser<'_, '_> {
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
        if self.selection.is_none() {
            self.budget.expanded(&[lexeme.len()], 128)?;
        }
        let value =
            serde_json::from_str::<String>(lexeme).map_err(|_| "invalid_json_preserved_as_text")?;
        Ok((value, lexeme.to_owned()))
    }
    fn value(&mut self, pointer: &str, depth: usize) -> Result<(), &'static str> {
        self.space();
        let start = self.index;
        let checkpoint = self.budget.checkpoint();
        self.value_inner(pointer, depth)?;
        if let Some(selection) = &mut self.selection {
            for (original, remaining) in &self.active {
                if remaining.is_empty() {
                    selection.record(original, &self.input[start..self.index])?;
                }
            }
            self.budget.restore(checkpoint);
        }
        Ok(())
    }
    fn child_value(
        &mut self,
        pointer: &str,
        token: &str,
        element: Option<usize>,
        depth: usize,
    ) -> Result<(), &'static str> {
        if self.selection.is_none() {
            let escaped = token.replace('~', "~0").replace('/', "~1");
            return self.value(&format!("{pointer}/{escaped}"), depth);
        }
        let checkpoint = self.budget.checkpoint();
        self.budget.charge(
            self.active
                .len()
                .checked_mul(64)
                .ok_or(super::budget::EXCEEDED)?,
        )?;
        let child = self
            .active
            .iter()
            .filter_map(|(original, remaining)| {
                let path = remaining.strip_prefix('/')?;
                let (key, rest) = path
                    .split_once('/')
                    .map_or((path, ""), |(key, _)| (key, &path[key.len()..]));
                let key = key.replace("~1", "/").replace("~0", "~");
                let matches = match element {
                    Some(index) => {
                        !(key.len() > 1 && key.starts_with('0'))
                            && key.parse::<usize>().ok() == Some(index)
                    }
                    None => key == token,
                };
                matches.then_some((*original, rest))
            })
            .collect();
        let parent = std::mem::replace(&mut self.active, child);
        let result = self.value("", depth);
        self.active = parent;
        self.budget.restore(checkpoint);
        result
    }
    fn value_inner(&mut self, pointer: &str, depth: usize) -> Result<(), &'static str> {
        if depth > 128 {
            return Err("json_depth_exceeded_preserved_as_text");
        }
        self.space();
        if self.consume(b'{') {
            let mut keys = BTreeSet::new();
            let mut audit_keys = Vec::<String>::new();
            if self.consume(b'}') {
                return Ok(());
            }
            loop {
                let (key, _) = self.string()?;
                if self.selection.is_some() {
                    if audit_keys.len() == audit_keys.capacity() {
                        let additional = audit_keys.capacity().max(4);
                        self.budget.charge(
                            additional
                                .checked_mul(std::mem::size_of::<String>())
                                .ok_or(super::budget::EXCEEDED)?,
                        )?;
                        audit_keys.reserve_exact(additional);
                    }
                    self.budget.charge(key.len())?;
                    audit_keys.push(key.clone());
                } else {
                    self.budget
                        .expanded(&[key.len(), pointer.len(), escaped_len(&key)], 128)?;
                    if !keys.insert(key.clone()) {
                        return Err("duplicate_json_keys_preserved_as_text");
                    }
                }
                if !self.consume(b':') {
                    return Err("invalid_json_preserved_as_text");
                }
                self.child_value(pointer, &key, None, depth + 1)?;
                if self.consume(b'}') {
                    audit_keys.sort_unstable();
                    if audit_keys.windows(2).any(|pair| pair[0] == pair[1]) {
                        return Err("duplicate_json_keys_preserved_as_text");
                    }
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
                if self.selection.is_none() {
                    self.budget.expanded(&[pointer.len()], 128)?;
                }
                self.child_value(pointer, &index.to_string(), Some(index), depth + 1)?;
                index += 1;
                if self.consume(b']') {
                    return Ok(());
                }
                if !self.consume(b',') {
                    return Err("invalid_json_preserved_as_text");
                }
            }
        }
        if self.selection.is_none() {
            self.budget.expanded(&[pointer.len()], 1536)?;
        }
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
                if self.selection.is_none() {
                    self.budget.expanded(&[text.len()], 0)?;
                }
                let (value, numeric) = match text {
                    "true" => (Value::Bool(true), None),
                    "false" => (Value::Bool(false), None),
                    "null" => (Value::Null, None),
                    _ => number(text)?,
                };
                (value, text.to_owned(), numeric)
            };
        if self.selection.is_none() {
            self.leaves.push(Leaf {
                pointer: pointer.to_owned(),
                value,
                lexeme,
                numeric_representation,
                flags: Vec::new(),
            });
        }
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
