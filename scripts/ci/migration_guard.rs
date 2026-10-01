use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::process::{Command, Output};

const MIGRATION_DIR: &str = "scripts/migrations";

#[derive(Debug)]
struct GuardError(String);

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GuardError {}

type Result<T> = std::result::Result<T, GuardError>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TableName {
    schema: String,
    name: String,
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.schema, self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TableDefinition {
    signature: Vec<Token>,
    if_not_exists: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TreeEntry {
    mode: String,
    oid: String,
}

fn git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| GuardError(format!("could not run git: {error}")))?;
    checked_output(output, &format!("git {}", args.join(" ")))
}

fn checked_output(output: Output, command: &str) -> Result<String> {
    if !output.status.success() {
        return Err(GuardError(format!(
            "{command} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| GuardError(format!("{command} returned non-UTF-8 output: {error}")))
}

fn resolve_ref(reference: &str) -> Result<String> {
    let output = git(&[
        "rev-parse",
        "--verify",
        "--end-of-options",
        &format!("{reference}^{{commit}}"),
    ])?;
    Ok(output.trim().to_owned())
}

fn migration_blobs(reference: &str) -> Result<BTreeMap<String, TreeEntry>> {
    let output = git(&["ls-tree", "-r", "-z", reference, "--", MIGRATION_DIR])?;
    let mut files = BTreeMap::new();
    for entry in output.split('\0').filter(|entry| !entry.is_empty()) {
        let (metadata, path) = entry
            .split_once('\t')
            .ok_or_else(|| GuardError("unexpected git ls-tree output".to_owned()))?;
        if !path.ends_with(".sql") {
            continue;
        }
        let fields: Vec<_> = metadata.split_whitespace().collect();
        if fields.len() != 3 || fields[1] != "blob" || fields[0] == "120000" {
            return Err(GuardError(format!(
                "unexpected migration tree entry: {path}"
            )));
        }
        files.insert(
            path.to_owned(),
            TreeEntry {
                mode: fields[0].to_owned(),
                oid: fields[2].to_owned(),
            },
        );
    }
    Ok(files)
}

fn blob(reference: &str, path: &str) -> Result<String> {
    let spec = format!("{reference}:{path}");
    let output = Command::new("git")
        .args(["show", &spec])
        .output()
        .map_err(|error| GuardError(format!("could not read {spec}: {error}")))?;
    checked_output(output, &format!("git show {spec}"))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Word(String),
    QuotedIdentifier(String),
    Literal(String),
    Operator(String),
    Symbol(char),
}

fn tokenize(sql: &str, path: &str) -> Result<Vec<Token>> {
    let bytes = sql.as_bytes();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < bytes.len() {
        match bytes[i] {
            b if b.is_ascii_whitespace() => i += 1,
            b'-' if bytes.get(i + 1) == Some(&b'-') => {
                i += 2;
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i += 2;
                let mut depth = 1usize;
                while i < bytes.len() && depth > 0 {
                    if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
                        depth += 1;
                        i += 2;
                    } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if depth != 0 {
                    return Err(GuardError(format!("{path}: unterminated block comment")));
                }
            }
            b'\'' => {
                let escape_string = i >= 2
                    && matches!(bytes[i - 1], b'e' | b'E')
                    && !(bytes[i - 2].is_ascii_alphanumeric()
                        || matches!(bytes[i - 2], b'_' | b'$'));
                let start = i;
                i += 1;
                let mut closed = false;
                while i < bytes.len() {
                    if bytes[i] == b'\'' {
                        if bytes.get(i + 1) == Some(&b'\'') {
                            i += 2;
                        } else {
                            i += 1;
                            closed = true;
                            break;
                        }
                    } else if bytes[i] == b'\\' && escape_string {
                        i = (i + 2).min(bytes.len());
                    } else {
                        i += 1;
                    }
                }
                if !closed {
                    return Err(GuardError(format!("{path}: unterminated string literal")));
                }
                tokens.push(Token::Literal(format!(
                    "{}{}",
                    if escape_string { "E" } else { "" },
                    std::str::from_utf8(&bytes[start..i]).map_err(|error| GuardError(format!(
                        "{path}: invalid string literal: {error}"
                    )))?
                )));
            }
            b'"' => {
                i += 1;
                let start = i;
                let mut value = String::new();
                let mut closed = false;
                while i < bytes.len() {
                    if bytes[i] == b'"' {
                        if bytes.get(i + 1) == Some(&b'"') {
                            i += 2;
                        } else {
                            value = std::str::from_utf8(&bytes[start..i])
                                .map_err(|error| {
                                    GuardError(format!(
                                        "{path}: invalid identifier encoding: {error}"
                                    ))
                                })?
                                .replace("\"\"", "\"");
                            i += 1;
                            closed = true;
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                if !closed {
                    return Err(GuardError(format!(
                        "{path}: unterminated quoted identifier"
                    )));
                }
                tokens.push(Token::QuotedIdentifier(value));
            }
            b'$' => {
                let mut delimiter_end = i + 1;
                while delimiter_end < bytes.len()
                    && (bytes[delimiter_end].is_ascii_alphanumeric()
                        || bytes[delimiter_end] == b'_')
                {
                    delimiter_end += 1;
                }
                if bytes.get(delimiter_end) == Some(&b'$') {
                    let delimiter = &bytes[i..=delimiter_end];
                    i = delimiter_end + 1;
                    let body_start = i;
                    let mut found = None;
                    while i + delimiter.len() <= bytes.len() {
                        if &bytes[i..i + delimiter.len()] == delimiter {
                            found = Some(i);
                            break;
                        }
                        i += 1;
                    }
                    let body_end = found.ok_or_else(|| {
                        GuardError(format!("{path}: unterminated dollar-quoted string"))
                    })?;
                    let body = std::str::from_utf8(&bytes[body_start..body_end])
                        .map_err(|error| {
                            GuardError(format!("{path}: invalid dollar string: {error}"))
                        })?
                        .to_owned();
                    i = body_end + delimiter.len();
                    tokens.push(Token::Literal(body));
                } else {
                    tokens.push(Token::Symbol('$'));
                    i += 1;
                }
            }
            b if b.is_ascii_alphabetic() || b == b'_' => {
                let start = i;
                i += 1;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'_' | b'$'))
                {
                    i += 1;
                }
                tokens.push(Token::Word(
                    std::str::from_utf8(&bytes[start..i])
                        .map_err(|error| GuardError(format!("{path}: invalid SQL: {error}")))?
                        .to_ascii_uppercase(),
                ));
            }
            b if b.is_ascii_digit() => {
                let start = i;
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.') {
                    i += 1;
                }
                tokens.push(Token::Literal(
                    std::str::from_utf8(&bytes[start..i])
                        .map_err(|error| {
                            GuardError(format!("{path}: invalid numeric literal: {error}"))
                        })?
                        .to_owned(),
                ));
            }
            b if is_operator_byte(b) => {
                let start = i;
                i += 1;
                while i < bytes.len()
                    && is_operator_byte(bytes[i])
                    && !(bytes[i] == b'-' && bytes.get(i + 1) == Some(&b'-'))
                    && !(bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*'))
                {
                    i += 1;
                }
                tokens.push(Token::Operator(
                    std::str::from_utf8(&bytes[start..i])
                        .map_err(|error| GuardError(format!("{path}: invalid operator: {error}")))?
                        .to_owned(),
                ));
            }
            b => {
                tokens.push(Token::Symbol(b as char));
                i += 1;
            }
        }
    }
    Ok(tokens)
}

fn is_operator_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'+' | b'-'
            | b'*'
            | b'/'
            | b'<'
            | b'>'
            | b'='
            | b'~'
            | b'!'
            | b'@'
            | b'#'
            | b'%'
            | b'^'
            | b'&'
            | b'|'
            | b'`'
            | b'?'
            | b'\\'
    )
}

fn identifier(token: &Token, path: &str) -> Result<String> {
    match token {
        Token::Word(word) => Ok(word.to_ascii_lowercase()),
        Token::QuotedIdentifier(name) if !name.is_empty() => Ok(name.clone()),
        _ => Err(GuardError(format!(
            "{path}: unsupported CREATE TABLE identifier"
        ))),
    }
}

fn parse_tables(
    sql: &str,
    path: &str,
    reject_dynamic: bool,
) -> Result<BTreeMap<TableName, TableDefinition>> {
    let tokens = tokenize(sql, path)?;
    let mut tables = BTreeMap::<TableName, TableDefinition>::new();
    let mut i = 0;
    while i < tokens.len() {
        if tokens.get(i) == Some(&Token::Word("SET".to_owned()))
            && tokens[i + 1..]
                .iter()
                .take_while(|token| **token != Token::Symbol(';'))
                .any(|token| {
                    identifier(token, path).is_ok_and(|word| {
                        word == "standard_conforming_strings" || word == "search_path"
                    })
                })
        {
            return Err(GuardError(format!(
                "{path}: SET search/lexer settings are outside the static DDL contract"
            )));
        }
        if matches!(
            tokens.get(i),
            Some(Token::Word(word)) if word == "DO" || (reject_dynamic && (word == "EXECUTE" || word == "CALL"))
        ) {
            if reject_dynamic {
                return Err(GuardError(format!(
                    "{path}: dynamic SQL is outside the static DDL guard"
                )));
            }
            i += 1;
            continue;
        }
        if tokens.get(i) != Some(&Token::Word("CREATE".to_owned())) {
            i += 1;
            continue;
        }
        let mut cursor = i + 1;
        if tokens.get(cursor) == Some(&Token::Word("OR".to_owned()))
            && tokens.get(cursor + 1) == Some(&Token::Word("REPLACE".to_owned()))
        {
            cursor += 2;
        }
        if reject_dynamic
            && matches!(
                tokens.get(cursor),
                Some(Token::Word(word)) if word == "FUNCTION" || word == "PROCEDURE" || word == "TRIGGER"
            )
        {
            return Err(GuardError(format!(
                "{path}: executable DDL is outside the static migration guard"
            )));
        }
        if reject_dynamic
            && tokens.get(cursor) == Some(&Token::Word("EVENT".to_owned()))
            && tokens.get(cursor + 1) == Some(&Token::Word("TRIGGER".to_owned()))
        {
            return Err(GuardError(format!(
                "{path}: executable DDL is outside the static migration guard"
            )));
        }
        if matches!(tokens.get(cursor), Some(Token::Word(word)) if word == "TEMP" || word == "TEMPORARY" || word == "UNLOGGED")
        {
            cursor += 1;
        }
        if tokens.get(cursor) != Some(&Token::Word("TABLE".to_owned())) {
            i += 1;
            continue;
        }
        cursor += 1;
        if tokens.get(cursor) == Some(&Token::Word("IF".to_owned())) {
            if tokens.get(cursor + 1) != Some(&Token::Word("NOT".to_owned()))
                || tokens.get(cursor + 2) != Some(&Token::Word("EXISTS".to_owned()))
            {
                return Err(GuardError(format!(
                    "{path}: unsupported CREATE TABLE IF form"
                )));
            }
            cursor += 3;
        }
        let first = identifier(
            tokens
                .get(cursor)
                .ok_or_else(|| GuardError(format!("{path}: CREATE TABLE is missing a name")))?,
            path,
        )?;
        cursor += 1;
        let qualified = tokens.get(cursor) == Some(&Token::Symbol('.'));
        if !qualified {
            return Err(GuardError(format!(
                "{path}: migrations must schema-qualify CREATE TABLE names"
            )));
        }
        let (schema, name) = if qualified {
            cursor += 1;
            let second = identifier(
                tokens.get(cursor).ok_or_else(|| {
                    GuardError(format!(
                        "{path}: CREATE TABLE has an incomplete qualified name"
                    ))
                })?,
                path,
            )?;
            cursor += 1;
            (first, second)
        } else {
            unreachable!("unqualified table names are rejected above")
        };
        if tokens.get(cursor) != Some(&Token::Symbol('(')) {
            return Err(GuardError(format!(
                "{path}: unsupported CREATE TABLE form for {schema}.{name}; expected a column list"
            )));
        }
        let column_start = cursor;
        let mut depth = 0usize;
        let mut end = cursor;
        loop {
            match tokens.get(end) {
                Some(Token::Symbol('(')) => depth += 1,
                Some(Token::Symbol(')')) => {
                    depth = depth.checked_sub(1).ok_or_else(|| {
                        GuardError(format!("{path}: unbalanced CREATE TABLE column list"))
                    })?;
                    if depth == 0 {
                        end += 1;
                        break;
                    }
                }
                Some(_) => {}
                None => {
                    return Err(GuardError(format!(
                        "{path}: unterminated CREATE TABLE column list"
                    )))
                }
            }
            end += 1;
        }
        let mut signature_tokens = tokens[column_start..end].to_vec();
        while end < tokens.len() && tokens[end] != Token::Symbol(';') {
            if tokens[end] == Token::Word("CREATE".to_owned()) {
                return Err(GuardError(format!(
                    "{path}: CREATE TABLE statement is missing a semicolon"
                )));
            }
            signature_tokens.push(tokens[end].clone());
            end += 1;
        }
        let modifiers = tokens[i + 1..cursor]
            .iter()
            .filter(|token| matches!(token, Token::Word(word) if word == "TEMP" || word == "TEMPORARY" || word == "UNLOGGED"))
            .cloned()
            .collect::<Vec<_>>();
        let mut signature = modifiers;
        signature.push(Token::Symbol('|'));
        signature.extend(signature_tokens);
        let table = TableName { schema, name };
        let definition = TableDefinition {
            signature,
            if_not_exists: tokens.get(i + 1..cursor).is_some_and(|slice| {
                slice.windows(3).any(|window| {
                    window
                        == [
                            Token::Word("IF".to_owned()),
                            Token::Word("NOT".to_owned()),
                            Token::Word("EXISTS".to_owned()),
                        ]
                })
            }),
        };
        if let Some(previous) = tables.get(&table) {
            if reject_dynamic && previous != &definition {
                return Err(GuardError(format!(
                    "{path}: conflicting CREATE TABLE definitions for {table}"
                )));
            }
        } else {
            tables.insert(table, definition);
        }
        i = if end < tokens.len() { end + 1 } else { end };
    }
    Ok(tables)
}

fn ensure_immutable(
    base: &BTreeMap<String, TreeEntry>,
    candidate: &BTreeMap<String, TreeEntry>,
    reference: &str,
) -> Result<()> {
    for (path, base_oid) in base {
        match candidate.get(path) {
            Some(candidate_oid) if candidate_oid == base_oid => {}
            Some(_) => {
                return Err(GuardError(format!(
                    "{reference} modifies applied migration {path}"
                )))
            }
            None => {
                return Err(GuardError(format!(
                    "{reference} deletes or renames applied migration {path}"
                )))
            }
        }
    }
    Ok(())
}

fn baseline_tables(
    reference: &str,
    files: &BTreeMap<String, TreeEntry>,
) -> Result<BTreeMap<TableName, Vec<(String, TableDefinition)>>> {
    let mut tables = BTreeMap::<TableName, Vec<(String, TableDefinition)>>::new();
    for path in files.keys() {
        for (table, definition) in parse_tables(&blob(reference, path)?, path, false)? {
            tables
                .entry(table)
                .or_default()
                .push((path.clone(), definition));
        }
    }
    Ok(tables)
}

fn new_tables(
    reference: &str,
    base: &BTreeMap<String, TreeEntry>,
    candidate: &BTreeMap<String, TreeEntry>,
    baseline: &BTreeMap<TableName, Vec<(String, TableDefinition)>>,
) -> Result<BTreeMap<TableName, (String, TableDefinition)>> {
    ensure_immutable(base, candidate, reference)?;
    let mut tables = BTreeMap::<TableName, (String, TableDefinition)>::new();
    for path in candidate.keys().filter(|path| !base.contains_key(*path)) {
        if candidate
            .get(path)
            .is_some_and(|entry| entry.mode != "100644")
        {
            return Err(GuardError(format!(
                "{reference} adds non-regular migration file {path}"
            )));
        }
        for (table, definition) in parse_tables(&blob(reference, path)?, path, true)? {
            if let Some(existing) = baseline.get(&table) {
                if !definition.if_not_exists
                    || !existing.iter().any(|(_, base_definition)| {
                        base_definition.signature == definition.signature
                    })
                {
                    let existing_paths = existing
                        .iter()
                        .map(|(existing_path, _)| existing_path.as_str())
                        .collect::<Vec<_>>();
                    return Err(GuardError(format!(
                        "{reference}:{path} conflicts with applied table {table} from {existing_paths:?}"
                    )));
                }
                continue;
            }
            if let Some((previous_path, previous)) = tables.get(&table) {
                if !definition.if_not_exists
                    || !previous.if_not_exists
                    || definition.signature != previous.signature
                {
                    return Err(GuardError(format!(
                        "{reference} has conflicting CREATE TABLE definitions for {table} in {previous_path} and {path}"
                    )));
                }
            } else {
                tables.insert(table, (path.clone(), definition));
            }
        }
    }
    Ok(tables)
}

fn parse_args() -> Result<(String, String, Vec<String>)> {
    let mut args = env::args().skip(1);
    let mut base = "origin/main".to_owned();
    let mut head = "HEAD".to_owned();
    let mut peers = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--base" => {
                base = args
                    .next()
                    .ok_or_else(|| GuardError("--base needs a ref".to_owned()))?
            }
            "--head" => {
                head = args
                    .next()
                    .ok_or_else(|| GuardError("--head needs a ref".to_owned()))?
            }
            "--peer" => peers.push(
                args.next()
                    .ok_or_else(|| GuardError("--peer needs a ref".to_owned()))?,
            ),
            "--help" | "-h" => {
                println!(
                    "Usage: migration-guard [--base REF] [--head REF] [--peer EXPLICIT_HEAD]..."
                );
                std::process::exit(0);
            }
            _ => return Err(GuardError(format!("unknown argument: {arg}"))),
        }
    }
    Ok((base, head, peers))
}

fn run() -> Result<()> {
    let (base_arg, head_arg, peer_args) = parse_args()?;
    let base_ref = resolve_ref(&base_arg)?;
    let head_ref = resolve_ref(&head_arg)?;
    let base = migration_blobs(&base_ref)?;
    if base.is_empty() {
        return Err(GuardError(format!(
            "base {base_arg} has no SQL migrations under {MIGRATION_DIR}"
        )));
    }

    let baseline = baseline_tables(&base_ref, &base)?;

    let mut candidates = vec![(
        head_arg,
        new_tables(&head_ref, &base, &migration_blobs(&head_ref)?, &baseline)?,
    )];
    let mut seen_refs = BTreeSet::new();
    seen_refs.insert(head_ref);
    for peer_arg in peer_args {
        let peer_ref = resolve_ref(&peer_arg)?;
        if seen_refs.insert(peer_ref.clone()) {
            let peer_files = migration_blobs(&peer_ref)?;
            candidates.push((
                peer_arg.clone(),
                new_tables(&peer_ref, &base, &peer_files, &baseline)?,
            ));
        }
    }

    let mut owners = BTreeMap::<TableName, (String, String, TableDefinition)>::new();
    for (reference, tables) in candidates {
        for (table, (path, definition)) in tables {
            if let Some((previous_ref, previous_path, previous)) = owners.get(&table) {
                if definition.signature != previous.signature
                    || !definition.if_not_exists
                    || !previous.if_not_exists
                {
                    return Err(GuardError(format!(
                        "migration table conflict: {table} is created by {previous_ref}:{previous_path} and {reference}:{path}"
                    )));
                }
            } else {
                owners.insert(table, (reference.clone(), path, definition));
            }
        }
    }
    println!("migration guard passed: {} applied migrations immutable; no duplicate CREATE TABLE across {} candidate head(s)", base.len(), seen_refs.len());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("migration guard failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_and_string_bodies_do_not_create_false_tables() {
        let sql = r#"
            -- CREATE TABLE comment_only (id int);
            /* outer /* nested CREATE TABLE ignored (x int); */ done */
            SELECT 'CREATE TABLE string_only (id int)';
            DO $$ BEGIN RAISE NOTICE 'CREATE TABLE body_only (x int)'; END $$;
            CREATE TABLE IF NOT EXISTS brain.real_table (id int);
        "#;
        assert_eq!(
            parse_tables(sql, "fixture.sql", false)
                .unwrap()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            [TableName {
                schema: "brain".to_owned(),
                name: "real_table".to_owned()
            }]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn quoted_schema_and_identifier_components_remain_distinct() {
        let sql = "CREATE TABLE \"Odd.Schema\".\"Mixed\" (id int);";
        assert_eq!(
            parse_tables(sql, "fixture.sql", false)
                .unwrap()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            [TableName {
                schema: "Odd.Schema".to_owned(),
                name: "Mixed".to_owned()
            }]
            .into_iter()
            .collect()
        );
        assert!(parse_tables("CREATE TABLE plain_table (id int);", "fixture.sql", false).is_err());
    }

    #[test]
    fn unsupported_create_table_form_fails_closed() {
        assert!(parse_tables("CREATE TABLE x AS SELECT 1", "fixture.sql", true).is_err());
    }

    #[test]
    fn dynamic_ddl_fails_closed_in_new_migration() {
        assert!(parse_tables(
            "DO $$ BEGIN EXECUTE 'CREATE TABLE x (id int)'; END $$",
            "fixture.sql",
            true
        )
        .is_err());
    }

    #[test]
    fn standard_and_escape_string_quoting_are_distinguished() {
        let sql = "SELECT 'E\\'; CREATE TABLE brain.visible (id int); SELECT 'tail';";
        let found = parse_tables(sql, "fixture.sql", false).unwrap();
        assert!(found.contains_key(&TableName {
            schema: "brain".to_owned(),
            name: "visible".to_owned(),
        }));
        assert!(parse_tables(
            "SET SESSION standard_conforming_strings = off;",
            "fixture.sql",
            true
        )
        .is_err());
        assert!(parse_tables("SET LOCAL search_path TO brain;", "fixture.sql", false).is_err());
    }

    #[test]
    fn new_migrations_require_schema_qualified_tables() {
        assert!(parse_tables("CREATE TABLE plain_table (id int);", "fixture.sql", true).is_err());
    }

    #[test]
    fn table_signatures_preserve_operator_token_boundaries() {
        let adjacent = parse_tables(
            "CREATE TABLE brain.ops (payload jsonb CHECK (payload @> '{}'::jsonb));",
            "fixture.sql",
            false,
        )
        .unwrap();
        let separated = parse_tables(
            "CREATE TABLE brain.ops (payload jsonb CHECK (payload @ > '{}'::jsonb));",
            "fixture.sql",
            false,
        )
        .unwrap();
        assert_ne!(adjacent, separated);
    }
}
