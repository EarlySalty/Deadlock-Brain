use super::budget::{escaped_len, Budget};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(super) struct Entry {
    pub pointer: String,
    pub key: String,
    pub value: String,
    pub occurrence: usize,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone)]
enum Token {
    Text(String),
    Open,
    Close,
    Condition(String),
}

pub(super) fn parse(input: &str) -> Result<Vec<Entry>, &'static str> {
    let mut budget = Budget::new();
    let tokens = tokenize(input, &mut budget)?;
    if tokens.is_empty() {
        return Ok(Vec::new());
    }
    let mut index = 0;
    let mut entries = Vec::new();
    parse_scope(&tokens, &mut index, "", false, 0, &mut entries, &mut budget)?;
    if index != tokens.len() {
        return Err("Nicht vollständig ausgewertete KV-Tokens");
    }
    Ok(entries)
}

fn parse_scope(
    tokens: &[Token],
    index: &mut usize,
    prefix: &str,
    nested: bool,
    depth: usize,
    entries: &mut Vec<Entry>,
    budget: &mut Budget,
) -> Result<(), &'static str> {
    if depth > 128 {
        return Err("KV-Verschachtelung überschreitet die Grenze");
    }
    let mut occurrences = BTreeMap::<String, usize>::new();
    while *index < tokens.len() {
        if matches!(tokens[*index], Token::Close) {
            if !nested {
                return Err("Unerwartete schließende KV-Klammer");
            }
            *index += 1;
            return Ok(());
        }
        let key = match &tokens[*index] {
            Token::Text(key) if !key.starts_with('#') => {
                budget.expanded(&[key.len(), prefix.len(), escaped_len(key)], 128)?;
                key.clone()
            }
            _ => return Err("Unbekanntes KV-Konstrukt oder externe Referenz"),
        };
        *index += 1;
        let occurrence = occurrences.entry(key.clone()).or_default();
        let current_occurrence = *occurrence;
        *occurrence += 1;
        let escaped = key.replace('~', "~0").replace('/', "~1");
        let pointer = format!("{prefix}/{escaped}/{current_occurrence}");
        let start = entries.len();
        match tokens.get(*index) {
            Some(Token::Text(value)) => {
                budget.expanded(&[pointer.len(), key.len(), value.len()], 1536)?;
                entries.push(Entry {
                    pointer: pointer.clone(),
                    key,
                    value: value.clone(),
                    occurrence: current_occurrence,
                    conditions: Vec::new(),
                });
                *index += 1;
            }
            Some(Token::Open) => {
                *index += 1;
                parse_scope(tokens, index, &pointer, true, depth + 1, entries, budget)?;
            }
            _ => return Err("Fehlender KV-Wert"),
        }
        while let Some(Token::Condition(condition)) = tokens.get(*index) {
            for entry in &mut entries[start..] {
                budget.expanded(&[condition.len()], 128)?;
                entry.conditions.push(condition.clone());
            }
            *index += 1;
        }
    }
    if nested {
        Err("Nicht geschlossene KV-Klammer")
    } else {
        Ok(())
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
                return Err("Nicht geschlossener KV-Kommentar");
            }
            index += 2;
            continue;
        }
        budget.charge(64)?;
        match bytes[index] {
            b'{' => {
                tokens.push(Token::Open);
                index += 1;
            }
            b'}' => {
                tokens.push(Token::Close);
                index += 1;
            }
            b'[' => {
                index += 1;
                let start = index;
                while index < bytes.len() && bytes[index] != b']' {
                    index += 1;
                }
                if index == bytes.len() {
                    return Err("Nicht geschlossene KV-Bedingung");
                }
                budget.charge(index - start)?;
                tokens.push(Token::Condition(input[start..index].to_owned()));
                index += 1;
            }
            b'"' => {
                index += 1;
                let start = index;
                loop {
                    if index >= bytes.len() {
                        return Err("Nicht geschlossene KV-Zeichenfolge");
                    }
                    match bytes[index] {
                        b'\\' => {
                            // Keep escape bytes. KV callers differ in whether escape processing is enabled.
                            index += 2;
                            if index > bytes.len() {
                                return Err("Unvollständige KV-Escapesequenz");
                            }
                        }
                        b'"' => break,
                        _ => index += 1,
                    }
                }
                budget.charge(index - start)?;
                tokens.push(Token::Text(input[start..index].to_owned()));
                index += 1;
            }
            _ => {
                let start = index;
                while index < bytes.len()
                    && !bytes[index].is_ascii_whitespace()
                    && !matches!(bytes[index], b'{' | b'}' | b'[' | b']' | b'"')
                {
                    index += 1;
                }
                if start == index {
                    return Err("Unbekanntes KV-Zeichen");
                }
                budget.charge(index - start)?;
                tokens.push(Token::Text(input[start..index].to_owned()));
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_nested_keys_have_distinct_paths_and_parent_conditions() {
        let entries = parse("root { block { x 1 x 2 } [$LINUX] block { x 3 } }").unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].pointer, "/root/0/block/0/x/0");
        assert_eq!(entries[1].pointer, "/root/0/block/0/x/1");
        assert_eq!(entries[2].pointer, "/root/0/block/1/x/0");
        assert_eq!(entries[0].conditions, ["$LINUX"]);
        assert!(entries[2].conditions.is_empty());
    }

    #[test]
    fn external_references_and_malformed_content_require_text_preservation() {
        assert!(parse("#base \"base.txt\" root { x 1 }").is_err());
        assert!(parse("root { x 1").is_err());
        assert!(parse("root { x }").is_err());
    }

    #[test]
    fn escape_bytes_are_preserved_without_selecting_runtime_escape_policy() {
        let entries = parse(r#"root { path "a\\b" newline "a\nb" }"#).unwrap();
        assert_eq!(entries[0].value, r"a\\b");
        assert_eq!(entries[1].value, r"a\nb");
    }
}
