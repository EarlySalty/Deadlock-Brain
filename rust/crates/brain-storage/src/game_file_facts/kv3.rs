use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::budget::{escaped_len, Budget, Selection};
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
    parse_with_budget(input, Budget::new())
}

fn body(input: &str) -> Result<&str, &'static str> {
    if input.starts_with("<!--") {
        let end = input.find("-->").ok_or("Nicht geschlossener KV3-Kopf")?;
        Ok(&input[end + 3..])
    } else {
        Ok(input)
    }
}

pub(super) fn parse_with_budget(
    input: &str,
    mut budget: Budget,
) -> Result<Vec<Leaf>, &'static str> {
    let tokens = tokenize(body(input)?, &mut budget)?;
    let mut parser = Parser {
        budget: &mut budget,
        tokens: Tokens::Buffered(&tokens),
        index: 0,
        leaves: Vec::new(),
        selection: None,
    };
    parser.value("", &[], 0)?;
    if parser.token(0)?.is_some() {
        return Err("Nicht vollständig ausgewertete KV3-Tokens");
    }
    Ok(parser.leaves)
}

pub(super) fn selected_hashes(
    input: &str,
    requested: &BTreeSet<&str>,
    mut budget: Budget,
) -> Result<BTreeMap<String, [u8; 32]>, &'static str> {
    let selection = Selection::new(input, requested, 8, &mut budget)?;
    let mut parser = Parser {
        budget: &mut budget,
        tokens: Tokens::Streaming(
            Scanner {
                input: body(input)?,
                index: 0,
            },
            VecDeque::new(),
        ),
        index: 0,
        leaves: Vec::new(),
        selection: Some(selection),
    };
    parser.value("", &[], 0)?;
    if parser.token(0)?.is_some() {
        return Err("Nicht vollständig ausgewertete KV3-Tokens");
    }
    Ok(parser.selection.map(|s| s.hashes).unwrap_or_default())
}

enum Tokens<'a> {
    Buffered(&'a [Token]),
    Streaming(Scanner<'a>, VecDeque<Token>),
}
struct Parser<'a, 'r> {
    budget: &'a mut Budget,
    tokens: Tokens<'a>,
    index: usize,
    leaves: Vec<Leaf>,
    selection: Option<Selection<'r>>,
}

impl Parser<'_, '_> {
    fn token(&mut self, offset: usize) -> Result<Option<Token>, &'static str> {
        match &mut self.tokens {
            Tokens::Buffered(tokens) => Ok(tokens.get(self.index + offset).cloned()),
            Tokens::Streaming(scanner, lookahead) => {
                while lookahead.len() <= offset {
                    let mut scratch = Budget::with_limit(
                        scanner
                            .input
                            .len()
                            .checked_mul(8)
                            .and_then(|n| n.checked_add(1024))
                            .ok_or(super::budget::EXCEEDED)?,
                    );
                    let Some(token) = scanner.next(&mut scratch)? else {
                        break;
                    };
                    lookahead.push_back(token);
                }
                Ok(lookahead.get(offset).cloned())
            }
        }
    }
    fn advance(&mut self, count: usize) {
        self.index += count;
        if let Tokens::Streaming(_, lookahead) = &mut self.tokens {
            for _ in 0..count {
                lookahead.pop_front();
            }
        }
    }
    fn child_pointer(&self, pointer: &str, suffix: &str) -> String {
        if self
            .selection
            .as_ref()
            .is_some_and(|s| !s.contains(pointer))
        {
            "\0".into()
        } else {
            let child = format!("{pointer}/{suffix}");
            if self.selection.as_ref().is_some_and(|s| !s.contains(&child)) {
                "\0".into()
            } else {
                child
            }
        }
    }
    fn value(&mut self, pointer: &str, flags: &[String], depth: usize) -> Result<(), &'static str> {
        let checkpoint = self.budget.checkpoint();
        self.value_inner(pointer, flags, depth)?;
        if self.selection.is_some() {
            self.budget.restore(checkpoint);
        }
        Ok(())
    }
    fn value_inner(
        &mut self,
        pointer: &str,
        flags: &[String],
        depth: usize,
    ) -> Result<(), &'static str> {
        if depth > 128 {
            return Err("KV3-Verschachtelung überschreitet die Grenze");
        }
        if matches!(self.token(1)?, Some(Token::Colon)) {
            if let Some(Token::Word(flag)) = self.token(0)? {
                self.advance(2);
                if self.selection.is_some() {
                    return self.value(pointer, flags, depth + 1);
                }
                self.budget
                    .expanded(&[flag.len(), flags.iter().map(String::len).sum()], 128)?;
                let mut child_flags = flags.to_vec();
                child_flags.push(flag);
                return self.value(pointer, &child_flags, depth + 1);
            }
        }
        match self.token(0)? {
            Some(Token::OpenObject) => {
                self.advance(1);
                let mut occurrences = BTreeMap::<String, usize>::new();
                loop {
                    if matches!(self.token(0)?, Some(Token::CloseObject)) {
                        self.advance(1);
                        return Ok(());
                    }
                    let key = match self.token(0)? {
                        Some(Token::Word(key)) | Some(Token::String(key, _)) => key,
                        _ => return Err("Ungültiger KV3-Objektschlüssel"),
                    };
                    self.advance(1);
                    if !matches!(self.token(0)?, Some(Token::Equals)) {
                        return Err("Fehlendes KV3-Gleichheitszeichen");
                    }
                    self.advance(1);
                    let escaped = key.replace('~', "~0").replace('/', "~1");
                    let relevant = self.selection.as_ref().is_none_or(|s| {
                        s.contains(pointer) && s.contains(&format!("{pointer}/{escaped}"))
                    });
                    let child_pointer = if relevant {
                        if self.selection.is_some() {
                            if !occurrences.contains_key(&key) {
                                self.budget.charge(
                                    key.len().checked_add(96).ok_or(super::budget::EXCEEDED)?,
                                )?;
                            }
                        } else {
                            self.budget
                                .expanded(&[key.len(), escaped_len(&key), pointer.len()], 128)?;
                        }
                        let occurrence = occurrences.entry(key).or_default();
                        let child =
                            self.child_pointer(pointer, &format!("{escaped}/{}", *occurrence));
                        *occurrence += 1;
                        child
                    } else {
                        "\0".into()
                    };
                    self.value(&child_pointer, flags, depth + 1)?;
                    if matches!(self.token(0)?, Some(Token::Comma)) {
                        self.advance(1);
                    }
                }
            }
            Some(Token::OpenArray) => {
                self.advance(1);
                let mut element = 0;
                loop {
                    if matches!(self.token(0)?, Some(Token::CloseArray)) {
                        self.advance(1);
                        return Ok(());
                    }
                    if self.selection.is_none() {
                        self.budget.expanded(&[pointer.len()], 128)?;
                    }
                    self.value(
                        &self.child_pointer(pointer, &format!("[{element}]")),
                        flags,
                        depth + 1,
                    )?;
                    element += 1;
                    if matches!(self.token(0)?, Some(Token::Comma)) {
                        self.advance(1);
                    } else if !matches!(self.token(0)?, Some(Token::CloseArray)) {
                        return Err("Fehlendes KV3-Arraytrennzeichen");
                    }
                }
            }
            Some(Token::String(value, lexeme)) => {
                if let Some(selection) = &mut self.selection {
                    selection.record(pointer, &lexeme)?;
                } else {
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
                        value: Value::String(value),
                        flags: flags.to_vec(),
                        lexeme,
                    });
                }
                self.advance(1);
                Ok(())
            }
            Some(Token::Word(word)) => {
                if self.selection.is_none() {
                    self.budget.expanded(
                        &[
                            pointer.len(),
                            word.len(),
                            flags.iter().map(String::len).sum(),
                        ],
                        256 + flags.len() * 128,
                    )?;
                }
                let (value, numeric_representation) = match word.as_str() {
                    "true" => (Value::Bool(true), None),
                    "false" => (Value::Bool(false), None),
                    "null" => (Value::Null, None),
                    _ => super::json_text::number(&word)?,
                };
                if let Some(selection) = &mut self.selection {
                    selection.record(pointer, &word)?;
                } else {
                    self.leaves.push(Leaf {
                        numeric_representation,
                        pointer: pointer.to_owned(),
                        value,
                        flags: flags.to_vec(),
                        lexeme: word,
                    });
                }
                self.advance(1);
                Ok(())
            }
            _ => Err("Fehlender KV3-Wert"),
        }
    }
}

fn tokenize(input: &str, budget: &mut Budget) -> Result<Vec<Token>, &'static str> {
    let mut scanner = Scanner { input, index: 0 };
    let mut tokens = Vec::new();
    while let Some(token) = scanner.next(budget)? {
        tokens.push(token);
    }
    Ok(tokens)
}
struct Scanner<'a> {
    input: &'a str,
    index: usize,
}
impl Scanner<'_> {
    fn next(&mut self, budget: &mut Budget) -> Result<Option<Token>, &'static str> {
        let input = self.input;
        let bytes = input.as_bytes();
        let mut index = self.index;
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
                        self.index = index;
                        return Ok(Some(Token::String(value, input[start..index].to_owned())));
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
                    self.index = index;
                    return Ok(Some(Token::String(value, lexeme.to_owned())));
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
                    self.index = index;
                    return Ok(Some(Token::Word(input[start..index].to_owned())));
                }
            };
            index += 1;
            self.index = index;
            return Ok(Some(token));
        }
        self.index = index;
        Ok(None)
    }
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
