//! Literal data only. This is NOT a Lua interpreter or a template expander.
//! No globals, calls, operators, metatables, includes or computed keys exist here.
use crate::{
    model::{Candidate, MAX_CANDIDATES, MAX_CONTENT_BYTES},
    Result,
};
use serde_json::{Map, Value};

fn escaped(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}

/// A complete, flat template invocation. Values remain strings: template names
/// and parameter names confer no gameplay meaning or execution permission.
pub fn template(text: &str) -> Result<Vec<Candidate>> {
    if text.len() > MAX_CONTENT_BYTES {
        return Err("template byte budget".into());
    }
    let trimmed = text.trim();
    let body = trimmed
        .strip_prefix("{{")
        .and_then(|s| s.strip_suffix("}}"))
        .ok_or("not a complete flat template")?;
    if body.contains(['{', '}', '<', '>', '[', ']']) {
        return Err("dynamic/nested template".into());
    }
    let mut parts = body.split('|');
    let name = parts.next().unwrap_or_default().trim();
    if name.is_empty() || name.starts_with('#') || name.contains(':') {
        return Err("template program/function unsupported".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    let mut offset =
        text.len() - text.trim_start().len() + 2 + body.find('|').unwrap_or(body.len()) + 1;
    for part in parts {
        let (key, value) = part
            .split_once('=')
            .ok_or("positional template parameter unsupported")?;
        let key = key.trim();
        if key.is_empty() || !seen.insert(key) {
            return Err("duplicate/empty template parameter".into());
        }
        let start = offset + part.find('=').unwrap() + 1;
        out.push(Candidate {
            locator: format!(
                "template:{}/{}@utf8:{}..{}",
                escaped(name),
                escaped(key),
                start,
                start + value.len()
            ),
            value: Value::String(value.to_string()),
        });
        offset += part.len() + 1;
        if out.len() > MAX_CANDIDATES {
            return Err("template candidate budget".into());
        }
    }
    if out.is_empty() {
        return Err("template has no literal parameters".into());
    }
    Ok(out)
}

/// Parse `return { key = literal, ["key"] = literal, ... }`. Tables may also
/// contain only positional literals. Mixing keyed/positional entries is refused.
/// Each leaf is located in the original UTF-8 bytes, including its quotes.
pub fn lua(text: &str) -> Result<Vec<Candidate>> {
    if text.len() > MAX_CONTENT_BYTES {
        return Err("literal byte budget".into());
    }
    let mut p = Parser {
        text,
        pos: 0,
        leaves: Vec::new(),
        nodes: 0,
    };
    p.ws();
    if p.word()? != "return" {
        return Err("only literal return tables are supported".into());
    }
    p.ws();
    if !p.peek('{') {
        return Err("root must be a literal table".into());
    }
    p.value("", 0)?;
    p.ws();
    if p.pos != text.len() {
        return Err("trailing program/expression".into());
    }
    Ok(p.leaves)
}

struct Parser<'a> {
    text: &'a str,
    pos: usize,
    leaves: Vec<Candidate>,
    nodes: usize,
}
impl Parser<'_> {
    fn ws(&mut self) {
        loop {
            while self
                .text
                .as_bytes()
                .get(self.pos)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.pos += 1;
            }
            if self.text[self.pos..].starts_with("--") && !self.text[self.pos..].starts_with("--[")
            {
                self.pos += self.text[self.pos..]
                    .find('\n')
                    .unwrap_or(self.text.len() - self.pos);
            } else {
                break;
            }
        }
    }
    fn peek(&self, ch: char) -> bool {
        self.text[self.pos..].starts_with(ch)
    }
    fn take(&mut self, ch: char) -> bool {
        self.ws();
        if self.peek(ch) {
            self.pos += ch.len_utf8();
            true
        } else {
            false
        }
    }
    fn need(&mut self, ch: char) -> Result<()> {
        if self.take(ch) {
            Ok(())
        } else {
            Err(format!("expected literal punctuation at {}", self.pos))
        }
    }
    fn word(&mut self) -> Result<&str> {
        self.ws();
        let start = self.pos;
        while self
            .text
            .as_bytes()
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
        {
            self.pos += 1;
        }
        if self.pos == start {
            return Err("expected identifier".into());
        }
        Ok(&self.text[start..self.pos])
    }
    fn string(&mut self) -> Result<String> {
        self.ws();
        let quote = self
            .text
            .as_bytes()
            .get(self.pos)
            .copied()
            .ok_or("missing string")?;
        if quote != b'\'' && quote != b'"' {
            return Err("expected quoted key".into());
        }
        self.pos += 1;
        let mut out = String::new();
        loop {
            let ch = self.text[self.pos..]
                .chars()
                .next()
                .ok_or("unterminated string")?;
            self.pos += ch.len_utf8();
            if ch as u32 == u32::from(quote) {
                return Ok(out);
            }
            if ch == '\\' {
                let next = self.text[self.pos..]
                    .chars()
                    .next()
                    .ok_or("unterminated escape")?;
                self.pos += next.len_utf8();
                out.push(match next {
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err("unsupported Lua escape".into()),
                });
            } else if ch.is_control() {
                return Err("unescaped control in string".into());
            } else {
                out.push(ch);
            }
        }
    }
    fn value(&mut self, path: &str, depth: usize) -> Result<Value> {
        self.ws();
        self.nodes += 1;
        if depth > 32 || self.nodes > MAX_CANDIDATES * 2 {
            return Err("literal structural budget".into());
        }
        if self.take('{') {
            let mut object = Map::new();
            let mut array = Vec::new();
            let mut keyed = None;
            while !self.take('}') {
                self.ws();
                let saved = self.pos;
                let key = if self.take('[') {
                    let key = self.string()?;
                    self.need(']')?;
                    self.need('=')?;
                    Some(key)
                } else if self
                    .text
                    .as_bytes()
                    .get(self.pos)
                    .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'_')
                {
                    let word = self.word()?.to_string();
                    if self.take('=') {
                        Some(word)
                    } else {
                        self.pos = saved;
                        None
                    }
                } else {
                    None
                };
                let is_keyed = key.is_some();
                if keyed.is_some_and(|previous| previous != is_keyed) {
                    return Err("mixed table unsupported".into());
                }
                keyed = Some(is_keyed);
                if let Some(key) = key {
                    if object.contains_key(&key) {
                        return Err("duplicate literal key".into());
                    }
                    let value = self.value(&format!("{path}/{}", escaped(&key)), depth + 1)?;
                    object.insert(key, value);
                } else {
                    let value = self.value(&format!("{path}/{}", array.len()), depth + 1)?;
                    // Lua nil creates holes, not JSON array entries. Never pretend otherwise.
                    if value.is_null() {
                        return Err("nil in positional table unsupported".into());
                    }
                    array.push(value);
                }
                if self.take('}') {
                    break;
                }
                if !self.take(',') && !self.take(';') {
                    return Err("expression or missing delimiter".into());
                }
            }
            return Ok(if keyed == Some(false) {
                Value::Array(array)
            } else {
                Value::Object(object)
            });
        }
        let start = self.pos;
        let value = if self.peek('"') || self.peek('\'') {
            Value::String(self.string()?)
        } else {
            while self
                .text
                .as_bytes()
                .get(self.pos)
                .is_some_and(|b| !b.is_ascii_whitespace() && !matches!(b, b',' | b';' | b'}'))
            {
                self.pos += 1;
            }
            let raw = &self.text[start..self.pos];
            match raw {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "nil" => Value::Null,
                _ => {
                    let value = crate::parse_json(raw.as_bytes())?;
                    if !value.is_number() {
                        return Err("nonliteral expression".into());
                    }
                    value
                }
            }
        };
        if self.leaves.len() >= MAX_CANDIDATES {
            return Err("literal candidate budget".into());
        }
        self.leaves.push(Candidate {
            locator: format!("lua:{path}@utf8:{start}..{}", self.pos),
            value: value.clone(),
        });
        Ok(value)
    }
}
