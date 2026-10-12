use brain_contracts::PortError;
use sqlx::PgConnection;
use std::{iter::Peekable, str::Chars};

const MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p5-docs-v1.sql");

const CONTRACT: &str = "WITH relations AS (
    SELECT c.oid,c.relname,c.relkind,c.relowner FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        WHERE n.nspname='docs')
    SELECT
    (SELECT count(*)=3 FROM (VALUES ('page','r'),('section','r'),('current_section','v')) e(relation,kind)
        JOIN relations c ON c.relname=e.relation AND c.relkind::text=e.kind
        JOIN pg_roles r ON r.oid=c.relowner WHERE r.rolname='brain_migrate')
    AND (SELECT count(*)=18 FROM (VALUES
        ('page','id','int8'),('page','source_id','int8'),('page','path','text'),('page','title','text'),
        ('page','lang','text'),('page','scope','text'),('page','raw_object_id','int8'),
        ('page','valid_from_revision_id','int8'),('page','valid_to_revision_id','int8'),
        ('section','id','int8'),('section','page_id','int8'),('section','ordinal','int4'),
        ('section','heading_path','_text'),('section','body','text'),('section','entity_ids','_int8'),
        ('section','url','text'),('section','receipt_id','int8'),('section','tsv','tsvector')) e(relation,name,type)
        JOIN relations c ON c.relname=e.relation
        JOIN pg_attribute a ON a.attrelid=c.oid
            AND a.attname=e.name AND a.atttypid=e.type::regtype AND NOT a.attisdropped
            AND a.attnotnull=(e.name<>'valid_to_revision_id'))
    AND (SELECT count(*)=16 FROM (VALUES
        ('section_id','int8'),('page_id','int8'),('source_id','int8'),('path','text'),('title','text'),
        ('lang','text'),('scope','text'),('ordinal','int4'),('heading_path','_text'),('body','text'),
        ('entity_ids','_int8'),('url','text'),('receipt_id','int8'),('source_revision','text'),
        ('revision_id','int8'),('tsv','tsvector')) e(name,type)
        JOIN pg_attribute a ON a.attrelid=(SELECT oid FROM relations WHERE relname='current_section') AND a.attname=e.name
            AND a.atttypid=e.type::regtype AND NOT a.attisdropped)
    AND (SELECT count(*)=2 FROM pg_attribute a JOIN pg_attrdef d ON d.adrelid=a.attrelid AND d.adnum=a.attnum
        WHERE a.attgenerated='s' AND ((a.attrelid=(SELECT oid FROM relations WHERE relname='section') AND a.attname='tsv'
            AND pg_get_expr(d.adbin,d.adrelid) IN ('docs.section_tsv(heading_path, body)','section_tsv(heading_path, body)'))
            OR (a.attrelid=(SELECT oid FROM relations WHERE relname='page') AND a.attname='scope'
                AND regexp_replace(pg_get_expr(d.adbin,d.adrelid),'[[:space:]]+','','g')=
                    'CASEWHEN(path~~''public/%''::text)THEN''public''::textELSE''internal''::textEND')))
    AND (SELECT count(*)=2 FROM pg_attribute WHERE attrelid IN ((SELECT oid FROM relations WHERE relname='page'),(SELECT oid FROM relations WHERE relname='section'))
        AND attname='id' AND attidentity='a')
    AND (SELECT count(*)=2 FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid JOIN pg_am m ON m.oid=c.relam
        JOIN pg_attribute a ON a.attrelid=i.indrelid AND a.attnum=i.indkey[0]
        WHERE i.indrelid=(SELECT oid FROM relations WHERE relname='section') AND m.amname='gin' AND a.attname IN ('tsv','entity_ids')
            AND i.indisvalid AND i.indisready AND i.indpred IS NULL AND i.indexprs IS NULL AND i.indnatts=1)
    AND EXISTS (SELECT 1 FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid
        WHERE c.relname='page_active_idx' AND i.indrelid=(SELECT oid FROM relations WHERE relname='page') AND i.indisunique AND i.indisvalid
            AND pg_get_expr(i.indpred,i.indrelid)='(valid_to_revision_id IS NULL)')
    AND (SELECT count(*)=5 FROM (VALUES
        ('docs.page','page_write_lock','serialize_write',30),
        ('docs.page','page_guard','guard_page',31),
        ('docs.section','section_write_lock','serialize_write',30),
        ('docs.section','section_guard','guard_section',31),
        ('canon.source_head','docs_publication_guard','guard_publication',23)) e(relation,name,function,type)
        JOIN pg_class c ON c.relname=split_part(e.relation,'.',2)
        JOIN pg_namespace relation_namespace ON relation_namespace.oid=c.relnamespace
            AND relation_namespace.nspname=split_part(e.relation,'.',1)
        JOIN pg_trigger t ON t.tgrelid=c.oid AND t.tgname=e.name AND t.tgtype=e.type
        JOIN pg_proc p ON p.oid=t.tgfoid AND p.proname=e.function
        JOIN pg_namespace n ON n.oid=p.pronamespace
        WHERE n.nspname='docs' AND t.tgenabled='O' AND NOT t.tgisinternal)
    AND NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid IN ((SELECT oid FROM relations WHERE relname='page'),(SELECT oid FROM relations WHERE relname='section'))
        AND NOT convalidated)
    AND (SELECT count(*)=4 FROM pg_constraint WHERE conrelid=(SELECT oid FROM relations WHERE relname='page') AND contype='f')
    AND (SELECT count(*)=2 FROM pg_constraint WHERE conrelid=(SELECT oid FROM relations WHERE relname='section') AND contype='f')
    AND (SELECT count(*)=4 FROM pg_constraint WHERE conrelid=(SELECT oid FROM relations WHERE relname='page') AND contype='c')
    AND (SELECT count(*)=5 FROM pg_constraint WHERE conrelid=(SELECT oid FROM relations WHERE relname='section') AND contype='c')";

const GRANTS: &str = "WITH namespace AS (SELECT oid FROM pg_namespace WHERE nspname='docs'),
    relations AS (SELECT c.oid,c.relname FROM pg_class c JOIN namespace n ON n.oid=c.relnamespace),
    functions AS (SELECT p.oid,p.proname FROM pg_proc p JOIN namespace n ON n.oid=p.pronamespace)
    SELECT
    EXISTS (SELECT 1 FROM pg_namespace n JOIN pg_roles r ON r.oid=n.nspowner
        WHERE n.nspname='docs' AND r.rolname='brain_migrate')
    AND NOT EXISTS (SELECT 1 FROM (VALUES ('brain_tool'),('brain_ingest'),('brain_service'),('brain_readonly')) r(name)
        WHERE has_schema_privilege(r.name,(SELECT oid FROM namespace),'CREATE')
            OR has_schema_privilege(r.name,(SELECT oid FROM namespace),'USAGE')<>(r.name IN ('brain_tool','brain_ingest')))
    AND NOT EXISTS (SELECT 1 FROM (VALUES ('brain_tool'),('brain_ingest'),('brain_service'),('brain_readonly')) r(name)
        CROSS JOIN relations t
        CROSS JOIN (VALUES ('SELECT'),('INSERT'),('UPDATE'),('DELETE'),('TRUNCATE'),('REFERENCES'),('TRIGGER')) p(name)
        WHERE t.relname IN ('page','section','current_section')
            AND has_table_privilege(r.name,t.oid,p.name)<>(p.name='SELECT' AND r.name IN ('brain_tool','brain_ingest')))
    AND NOT EXISTS (SELECT 1 FROM (VALUES ('brain_tool'),('brain_ingest'),('brain_service'),('brain_readonly')) r(name)
        CROSS JOIN (VALUES ('SELECT'),('INSERT'),('UPDATE'),('REFERENCES')) p(name)
        CROSS JOIN pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        JOIN pg_attribute a ON a.attrelid=c.oid AND a.attnum>0 AND NOT a.attisdropped
        WHERE n.nspname='docs' AND c.relname IN ('page','section','current_section')
            AND has_column_privilege(r.name,c.oid,a.attnum,p.name)<>
                (p.name='SELECT' AND r.name IN ('brain_tool','brain_ingest')
                OR r.name='brain_ingest' AND p.name='INSERT' AND
                    (c.relname='page' AND a.attname IN ('source_id','path','title','lang','raw_object_id','valid_from_revision_id')
                    OR c.relname='section' AND a.attname IN ('page_id','ordinal','heading_path','body','entity_ids','url','receipt_id'))
                OR r.name='brain_ingest' AND p.name='UPDATE' AND c.relname='page' AND a.attname='valid_to_revision_id'))
    AND NOT EXISTS (SELECT 1 FROM (VALUES ('brain_tool'),('brain_ingest'),('brain_service'),('brain_readonly')) r(name)
        CROSS JOIN relations s
        CROSS JOIN (VALUES ('USAGE'),('SELECT'),('UPDATE')) p(name)
        WHERE s.relname IN ('page_id_seq','section_id_seq')
            AND has_sequence_privilege(r.name,s.oid,p.name)<>(r.name='brain_ingest' AND p.name='USAGE'))
    AND NOT EXISTS (SELECT 1 FROM (VALUES ('brain_tool'),('brain_ingest'),('brain_service'),('brain_readonly')) r(name)
        CROSS JOIN (VALUES ('valid_heading_path',true),('valid_entity_ids',true),
            ('section_tsv',true),('serialize_write',false),('guard_page',false),
            ('guard_section',false),('guard_publication',false)) f(name,ingest)
        JOIN functions p ON p.proname=f.name
        WHERE has_function_privilege(r.name,p.oid,'EXECUTE')<>(r.name='brain_ingest' AND f.ingest))
    AND NOT EXISTS (SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        CROSS JOIN LATERAL aclexplode(coalesce(c.relacl,acldefault(CASE WHEN c.relkind='S' THEN 'S'::\"char\" ELSE 'r'::\"char\" END,c.relowner))) acl
        WHERE n.nspname='docs' AND c.relname IN ('page','section','current_section','page_id_seq','section_id_seq')
            AND acl.grantee=0)
    AND NOT EXISTS (SELECT 1 FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid
        JOIN pg_namespace n ON n.oid=c.relnamespace CROSS JOIN LATERAL aclexplode(a.attacl) acl
        WHERE n.nspname='docs' AND c.relname IN ('page','section','current_section') AND acl.grantee=0)
    AND NOT EXISTS (SELECT 1 FROM pg_namespace n CROSS JOIN LATERAL aclexplode(coalesce(n.nspacl,acldefault('n',n.nspowner))) acl
        WHERE n.nspname='docs' AND acl.grantee=0)
    AND NOT EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace
        CROSS JOIN LATERAL aclexplode(coalesce(p.proacl,acldefault('f',p.proowner))) acl
        WHERE n.nspname='docs' AND p.proname IN ('valid_heading_path','valid_entity_ids','section_tsv','serialize_write',
            'guard_page','guard_section','guard_publication') AND acl.grantee=0)";

fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Atom(Vec<String>),
    Comparison(Vec<String>, String, Vec<String>),
    IsNull(Vec<String>),
    And(Vec<Self>),
    Or(Vec<Self>),
}

fn definition_error() -> PortError {
    invalid("Dokumentdefinition erfüllt den Speichervertrag nicht")
}

fn quoted(chars: &mut Peekable<Chars<'_>>, escaped: bool) -> Result<String, PortError> {
    let mut value = String::new();
    loop {
        match chars.next() {
            Some('\'') if chars.peek() == Some(&'\'') => {
                chars.next();
                value.push('\'');
            }
            Some('\'') => return Ok(format!("'{}'", value.replace('\'', "''"))),
            Some('\\') if escaped => match chars.next() {
                Some(c @ ('\\' | '\'')) => value.push(c),
                _ => return Err(definition_error()),
            },
            Some(c) => value.push(c),
            None => return Err(definition_error()),
        }
    }
}

fn tokens(sql: &str) -> Result<Vec<String>, PortError> {
    if sql.len() > 16384 {
        return Err(definition_error());
    }
    let mut chars = sql.chars().peekable();
    let mut result = Vec::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_whitespace() {
            continue;
        }
        let token = if c == '\'' {
            quoted(&mut chars, false)?
        } else if matches!(c, 'e' | 'E') && chars.peek() == Some(&'\'') {
            chars.next();
            quoted(&mut chars, true)?
        } else if c.is_ascii_alphanumeric() || c == '_' {
            let mut word = c.to_string();
            while chars
                .peek()
                .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_')
            {
                if let Some(c) = chars.next() {
                    word.push(c);
                }
            }
            word.to_ascii_lowercase()
        } else if matches!(c, '(' | ')' | '[' | ']' | ',' | '.' | ';' | '=' | '~') {
            c.to_string()
        } else if matches!(c, '<' | '>' | '!' | ':') {
            let mut operator = c.to_string();
            if chars.peek().is_some_and(|next| {
                matches!((c, *next), ('<', '=') | ('>', '=') | ('!', '~') | (':', ':'))
            }) {
                if let Some(next) = chars.next() {
                    operator.push(next);
                }
            }
            operator
        } else {
            return Err(definition_error());
        };
        result.push(token);
    }
    Ok(result)
}

fn closing_paren(tokens: &[String]) -> Result<usize, PortError> {
    if tokens.first().map(String::as_str) != Some("(") {
        return Err(definition_error());
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" => {
                depth += 1;
                if depth > 64 {
                    return Err(definition_error());
                }
            }
            ")" => {
                depth = depth.checked_sub(1).ok_or_else(definition_error)?;
                if depth == 0 {
                    return Ok(index);
                }
            }
            _ => {}
        }
    }
    Err(definition_error())
}

fn unwrapped(mut tokens: &[String]) -> Result<&[String], PortError> {
    while tokens.first().map(String::as_str) == Some("(")
        && closing_paren(tokens)? == tokens.len() - 1
    {
        tokens = &tokens[1..tokens.len() - 1];
    }
    if tokens.is_empty() {
        return Err(definition_error());
    }
    Ok(tokens)
}

fn top_level(tokens: &[String], wanted: &str) -> Result<Vec<usize>, PortError> {
    let mut depth = 0usize;
    let mut between = false;
    let mut result = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" => {
                depth += 1;
                if depth > 64 {
                    return Err(definition_error());
                }
            }
            ")" => depth = depth.checked_sub(1).ok_or_else(definition_error)?,
            "between" if depth == 0 => between = true,
            "and" if depth == 0 && between => between = false,
            word if depth == 0 && word == wanted => result.push(index),
            _ => {}
        }
    }
    if depth != 0 || between {
        return Err(definition_error());
    }
    Ok(result)
}

fn atom(tokens: &[String]) -> Result<Vec<String>, PortError> {
    let mut tokens = unwrapped(tokens)?;
    if tokens.len() >= 3
        && tokens[tokens.len() - 2] == "::"
        && tokens[tokens.len() - 1] == "text"
    {
        tokens = unwrapped(&tokens[..tokens.len() - 2])?;
        if tokens.len() != 1 || !tokens[0].starts_with('\'') {
            return Err(definition_error());
        }
    }
    if tokens.len() >= 3
        && tokens[1] == "."
        && (tokens[0] == "docs"
            && matches!(tokens[2].as_str(), "valid_heading_path" | "valid_entity_ids")
            || tokens[0] == "pg_catalog"
                && matches!(tokens[2].as_str(), "octet_length" | "length"))
    {
        tokens = &tokens[2..];
    }
    Ok(tokens.to_vec())
}

fn logical(tokens: &[String], operator: &str, cuts: &[usize]) -> Result<Expression, PortError> {
    let mut parts = Vec::new();
    let mut start = 0;
    for end in cuts.iter().copied().chain(std::iter::once(tokens.len())) {
        let part = expression(&tokens[start..end])?;
        match (operator, part) {
            ("and", Expression::And(nested)) | ("or", Expression::Or(nested)) => {
                parts.extend(nested);
            }
            (_, part) => parts.push(part),
        }
        start = end + 1;
    }
    Ok(if operator == "and" {
        Expression::And(parts)
    } else {
        Expression::Or(parts)
    })
}

fn expression(tokens: &[String]) -> Result<Expression, PortError> {
    let tokens = unwrapped(tokens)?;
    for operator in ["or", "and"] {
        let cuts = top_level(tokens, operator)?;
        if !cuts.is_empty() {
            return logical(tokens, operator, &cuts);
        }
    }
    if let Some(index) = tokens.iter().position(|token| token == "between") {
        let rest = &tokens[index + 1..];
        let bound = rest.iter().position(|token| token == "and").ok_or_else(definition_error)?;
        let left = atom(&tokens[..index])?;
        return Ok(Expression::And(vec![
            Expression::Comparison(left.clone(), ">=".into(), atom(&rest[..bound])?),
            Expression::Comparison(left, "<=".into(), atom(&rest[bound + 1..])?),
        ]));
    }
    for operator in ["=", "<", ">", "<=", ">=", "~", "!~"] {
        let positions = top_level(tokens, operator)?;
        if positions.len() == 1 {
            let index = positions[0];
            return Ok(Expression::Comparison(
                atom(&tokens[..index])?,
                operator.into(),
                atom(&tokens[index + 1..])?,
            ));
        }
    }
    if tokens.len() >= 3
        && tokens[tokens.len() - 2] == "is"
        && tokens[tokens.len() - 1] == "null"
    {
        return Ok(Expression::IsNull(atom(&tokens[..tokens.len() - 2])?));
    }
    Ok(Expression::Atom(atom(tokens)?))
}

fn view_prefix(tokens: &[String]) -> Vec<&str> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        if index + 2 < tokens.len()
            && matches!(tokens[index].as_str(), "docs" | "canon")
            && tokens[index + 1] == "."
            && matches!(tokens[index + 2].as_str(), "page" | "section" | "source_head" | "revision")
        {
            index += 2;
        }
        if !matches!(tokens[index].as_str(), "(" | ")") {
            result.push(tokens[index].as_str());
        }
        index += 1;
    }
    result
}

fn view_definition(sql: &str) -> Result<(Vec<String>, Expression), PortError> {
    let mut tokens = tokens(sql)?;
    if tokens.last().map(String::as_str) == Some(";") {
        tokens.pop();
    }
    let index = tokens.iter().position(|token| token == "where").ok_or_else(definition_error)?;
    let predicate = expression(&tokens[index + 1..])?;
    let prefix = view_prefix(&tokens[..index]).into_iter().map(str::to_owned).collect();
    Ok((prefix, predicate))
}

fn table_checks(table: &str) -> Result<Vec<Expression>, PortError> {
    let marker = format!("CREATE TABLE docs.{table} (");
    let body = MIGRATION
        .split_once(marker.as_str())
        .and_then(|(_, rest)| rest.split_once("\n);"))
        .map(|(body, _)| body)
        .ok_or_else(definition_error)?;
    let tokens = tokens(body)?;
    let mut checks = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        if tokens[index] == "check" {
            let start = index + 1;
            let end = start + closing_paren(&tokens[start..])?;
            checks.push(expression(&tokens[start + 1..end])?);
            index = end;
        }
        index += 1;
    }
    Ok(checks)
}

async fn check_definitions(connection: &mut PgConnection) -> Result<(), PortError> {
    let expected_view = MIGRATION
        .split_once("CREATE VIEW docs.current_section AS")
        .and_then(|(_, rest)| rest.split_once(';'))
        .map(|(sql, _)| sql)
        .ok_or_else(definition_error)?;
    let (view, dependencies): (String, bool) = sqlx::query_as("WITH relations AS (
        SELECT c.oid,c.relname,n.nspname FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
            WHERE n.nspname IN ('docs','canon'))
        SELECT pg_get_viewdef((SELECT oid FROM relations WHERE nspname='docs' AND relname='current_section'),false),
        NOT EXISTS (SELECT 1 FROM pg_rewrite r JOIN pg_depend d ON d.classid='pg_rewrite'::regclass AND d.objid=r.oid
            WHERE r.ev_class=(SELECT oid FROM relations WHERE nspname='docs' AND relname='current_section') AND d.refclassid='pg_class'::regclass
                AND d.refobjid NOT IN (SELECT oid FROM relations WHERE nspname='docs' AND relname IN ('current_section','page','section')
                    OR nspname='canon' AND relname IN ('source_head','revision')))
        AND NOT EXISTS (SELECT 1 FROM (VALUES ('docs','page'),('docs','section'),('canon','source_head'),('canon','revision')) e(namespace,relation)
            WHERE NOT EXISTS (SELECT 1 FROM pg_rewrite r JOIN pg_depend d ON d.classid='pg_rewrite'::regclass AND d.objid=r.oid
                JOIN relations c ON c.oid=d.refobjid AND c.nspname=e.namespace AND c.relname=e.relation
                WHERE r.ev_class=(SELECT oid FROM relations WHERE nspname='docs' AND relname='current_section') AND d.refclassid='pg_class'::regclass))")
        .fetch_one(&mut *connection).await.map_err(|_| definition_error())?;
    if !dependencies || view_definition(&view)? != view_definition(expected_view)? {
        return Err(definition_error());
    }
    let checks: Vec<(String, String, bool)> = sqlx::query_as("WITH namespace AS (SELECT oid FROM pg_namespace WHERE nspname='docs'),
        relations AS (SELECT c.oid,c.relname FROM pg_class c JOIN namespace n ON n.oid=c.relnamespace),
        functions AS (SELECT p.oid FROM pg_proc p JOIN namespace n ON n.oid=p.pronamespace
            WHERE p.pronargs=1 AND (p.proname='valid_heading_path' AND p.proargtypes[0]='text[]'::regtype
                OR p.proname='valid_entity_ids' AND p.proargtypes[0]='bigint[]'::regtype))
        SELECT r.relname::text,pg_get_expr(c.conbin,c.conrelid,false),
        NOT EXISTS (SELECT 1 FROM pg_depend d WHERE d.classid='pg_constraint'::regclass AND d.objid=c.oid
            AND d.refclassid='pg_proc'::regclass AND d.refobjid NOT IN (SELECT oid FROM functions))
        FROM pg_constraint c JOIN relations r ON r.oid=c.conrelid
        WHERE r.relname IN ('page','section') AND c.contype='c' AND c.convalidated
        ORDER BY r.relname,c.oid LIMIT 10")
        .fetch_all(&mut *connection).await.map_err(|_| definition_error())?;
    let mut expected = Vec::new();
    for table in ["page", "section"] {
        expected.extend(table_checks(table)?.into_iter().map(|check| (table, check)));
    }
    for (table, sql, dependencies) in checks {
        let actual = expression(&tokens(&sql)?)?;
        let index = expected.iter().position(|(name, check)| *name == table && *check == actual)
            .ok_or_else(definition_error)?;
        if !dependencies {
            return Err(definition_error());
        }
        expected.remove(index);
    }
    if !expected.is_empty() {
        return Err(definition_error());
    }
    Ok(())
}

pub(super) async fn check(connection: &mut PgConnection) -> Result<(), PortError> {
    let valid: bool = sqlx::query_scalar(CONTRACT)
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| invalid("Dokumentschema erfüllt den Speichervertrag nicht"))?;
    if !valid {
        return Err(invalid("Dokumentschema erfüllt den Speichervertrag nicht"));
    }
    check_definitions(connection).await?;
    let valid: bool = sqlx::query_scalar(GRANTS)
        .fetch_one(&mut *connection)
        .await
        .map_err(|_| invalid("Dokumentrechte sind nicht prüfbar"))?;
    if !valid {
        return Err(invalid("Dokumentrechte erfüllen den Speichervertrag nicht"));
    }
    for (name, arguments, result, immutable, tag) in [
        (
            "valid_heading_path",
            &["text[]"][..],
            "bool",
            true,
            "heading",
        ),
        (
            "valid_entity_ids",
            &["bigint[]"][..],
            "bool",
            true,
            "entities",
        ),
        (
            "section_tsv",
            &["text[]", "text"][..],
            "tsvector",
            true,
            "fts",
        ),
        (
            "serialize_write",
            &[][..],
            "trigger",
            false,
            "serialize",
        ),
        (
            "guard_page",
            &[][..],
            "trigger",
            false,
            "page",
        ),
        (
            "guard_section",
            &[][..],
            "trigger",
            false,
            "section",
        ),
        (
            "guard_publication",
            &[][..],
            "trigger",
            false,
            "publish",
        ),
    ] {
        let delimiter = format!("${tag}$");
        let marker = format!("AS {delimiter}");
        let expected = MIGRATION
            .split_once(marker.as_str())
            .and_then(|(_, rest)| rest.split_once(delimiter.as_str()))
            .map(|(source, _)| source)
            .ok_or_else(|| invalid("Eingebetteter Dokumentvertrag ist ungültig"))?;
        let matches: bool = sqlx::query_scalar("SELECT count(*)=1 AND coalesce(bool_and(p.pronargs=cardinality($3::text[]) AND NOT EXISTS (SELECT 1 FROM generate_subscripts($3::text[],1) i WHERE p.proargtypes[i-1]<>($3[i])::regtype) AND p.prorettype=$4::regtype AND p.prokind='f' AND NOT p.proretset AND l.lanname=CASE WHEN $5 THEN 'sql' ELSE 'plpgsql' END AND p.provolatile=CASE WHEN $5 THEN 'i'::\"char\" ELSE 'v'::\"char\" END AND p.proparallel=CASE WHEN $5 THEN 's'::\"char\" ELSE 'u'::\"char\" END AND p.prosrc=$2 AND NOT p.prosecdef AND p.proconfig=ARRAY['search_path=pg_catalog, pg_temp'] AND r.rolname='brain_migrate'),false) FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace JOIN pg_roles r ON r.oid=p.proowner JOIN pg_language l ON l.oid=p.prolang WHERE n.nspname='docs' AND p.proname=$1")
            .bind(name)
            .bind(expected)
            .bind(arguments)
            .bind(result)
            .bind(immutable)
            .fetch_one(&mut *connection)
            .await
            .map_err(|_| invalid("Dokumentfunktionen sind nicht prüfbar"))?;
        if !matches {
            return Err(invalid(
                "Dokumentfunktionen erfüllen den Speichervertrag nicht",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "../tests/support/scratch_pg.rs"]
mod scratch_pg;

#[cfg(test)]
mod tests {
    use super::{check, scratch_pg::ScratchPg};
    use crate::PgStore;
    use serde_json::Value;
    use sqlx::{postgres::PgPoolOptions, PgConnection, PgPool, Row};

    const MIGRATION: &str =
        include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p5-docs-v1.sql");

    struct Database {
        scratch: ScratchPg,
        admin: PgPool,
        owner: PgPool,
        ingest: PgPool,
        reader: PgPool,
        service: PgPool,
    }

    impl Database {
        async fn new() -> Self {
            let scratch = ScratchPg::start();
            let admin = pool(&scratch, "postgres", "brain_core_test").await;
            let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
                .fetch_one(&admin)
                .await
                .unwrap();
            assert_eq!(directory, scratch.directory.join("data").to_str().unwrap());
            let listeners: String = sqlx::query_scalar("SELECT current_setting('listen_addresses')")
                .fetch_one(&admin)
                .await
                .unwrap();
            assert_eq!(listeners, "");
            sqlx::raw_sql(
                "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
                CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
                CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
                CREATE ROLE brain_readonly LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE",
            )
            .execute(&admin)
            .await
            .unwrap();
            sqlx::query("CREATE DATABASE brain_docs OWNER brain_migrate")
                .execute(&admin)
                .await
                .unwrap();
            let migration_admin = pool(&scratch, "brain_docs", "brain_core_test").await;
            PgStore::new(migration_admin.clone())
                .migrate_gamefacts()
                .await
                .unwrap();
            migration_admin.close().await;
            let owner = pool(&scratch, "brain_docs", "brain_migrate").await;
            PgStore::new(owner.clone())
                .migrate_gamefacts()
                .await
                .unwrap();
            check(&mut owner.acquire().await.unwrap())
                .await
                .unwrap();
            PgStore::new(owner.clone())
                .check_gamefacts_schema()
                .await
                .unwrap();
            let ingest = pool(&scratch, "brain_docs", "brain_ingest").await;
            let reader = pool(&scratch, "brain_docs", "brain_readonly").await;
            let service = pool(&scratch, "brain_docs", "brain_service").await;
            Self {
                scratch,
                admin,
                owner,
                ingest,
                reader,
                service,
            }
        }

        async fn close(self) {
            self.service.close().await;
            self.reader.close().await;
            self.ingest.close().await;
            self.owner.close().await;
            self.admin.close().await;
            drop(self.scratch);
        }
    }

    async fn pool(scratch: &ScratchPg, database: &str, username: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(scratch.directory.join("socket").to_str().unwrap())
                    .port(55439)
                    .database(database)
                    .username(username)
                    .password("")
                    .ssl_mode(sqlx::postgres::PgSslMode::Disable),
            )
            .await
            .unwrap()
    }

    struct Revision {
        source: i64,
        run: i64,
        id: i64,
        target: String,
    }

    async fn revision(ingest: &PgPool, source: Option<i64>, target: char) -> Revision {
        let mut tx = ingest.begin().await.unwrap();
        let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-docs','docs','v1') RETURNING id")
            .fetch_one(&mut *tx).await.unwrap();
        let source = match source {
            Some(source) => source,
            None => sqlx::query_scalar("INSERT INTO raw.source(kind,locator) VALUES ('docs','synthetic://docs-'||$1::text) RETURNING id")
                .bind(job).fetch_one(&mut *tx).await.unwrap(),
        };
        let target = target.to_string().repeat(40);
        let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,base_revision,target_revision,extractor,extractor_version) SELECT id,$2,last_complete_revision,$3,'docs','v1' FROM raw.source WHERE id=$1 RETURNING id")
            .bind(source).bind(job).bind(&target).fetch_one(&mut *tx).await.unwrap();
        let id: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,'docs','v1') RETURNING id")
            .bind(source).bind(run).bind(&target).fetch_one(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        Revision {
            source,
            run,
            id,
            target,
        }
    }

    struct Page {
        id: i64,
        receipt: i64,
        url: String,
    }

    async fn page(ingest: &PgPool, revision: &Revision, path: &str) -> Page {
        let mut tx = ingest.begin().await.unwrap();
        let object: i64 = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,decode($3||repeat('01',12),'hex'),'text/html',convert_to('synthetic-'||$3,'UTF8')) RETURNING id")
            .bind(revision.source).bind(path).bind(&revision.target).fetch_one(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES ($1,$2,$3,'upsert','succeeded')")
            .bind(revision.run).bind(path).bind(object).execute(&mut *tx).await.unwrap();
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,'line_range','L1-L2',decode(repeat('02',32),'hex'),'docs','v1') RETURNING id")
            .bind(object).bind(revision.source).bind(revision.id).fetch_one(&mut *tx).await.unwrap();
        let id: i64 = sqlx::query_scalar("INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id) VALUES ($1,$2,'Vertrag','de',$3,$4) RETURNING id")
            .bind(revision.source).bind(path).bind(object).bind(revision.id).fetch_one(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        Page {
            id,
            receipt,
            url: format!(
                "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/{path}#L1-L2",
                revision.target
            ),
        }
    }

    async fn section(ingest: &PgPool, page: &Page, body: &str) {
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,heading_path,body,url,receipt_id) VALUES ($1,0,ARRAY['Frühes Spiel'],$2,$3,$4)")
            .bind(page.id).bind(body).bind(&page.url).bind(page.receipt).execute(ingest).await.unwrap();
    }

    async fn publish(ingest: &PgPool, revision: &Revision) -> Result<(), sqlx::Error> {
        let mut tx = ingest.begin().await?;
        sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
            .bind(revision.run)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2) ON CONFLICT(source_id) DO UPDATE SET published_revision_id=excluded.published_revision_id")
            .bind(revision.source).bind(revision.id).execute(&mut *tx).await?;
        sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
            .bind(revision.source)
            .bind(&revision.target)
            .execute(&mut *tx)
            .await?;
        tx.commit().await
    }

    async fn reject(connection: &mut PgConnection, sql: &str, code: &str) {
        let error = sqlx::raw_sql(sql).execute(connection).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some(code),
            "{sql}: {error}"
        );
    }

    async fn count(pool: &PgPool, scope: &str) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM docs.current_section WHERE scope=$1")
            .bind(scope)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn real_postgres_docs_receipts_publication_and_path_scopes() {
        let db = Database::new().await;
        let first = revision(&db.ingest, None, 'a').await;
        let public = page(&db.ingest, &first, "public/hero.html").await;
        let internal = page(&db.ingest, &first, "internal/private.html").await;
        section(&db.ingest, &public, "Erster öffentlicher Abschnitt").await;
        section(&db.ingest, &internal, "Interner Abschnitt").await;
        assert_eq!(count(&db.owner, "public").await, 0);
        let mut writer = db.ingest.acquire().await.unwrap();
        reject(&mut writer, &format!("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},1,'falsch','https://example.invalid',{})",public.id,internal.receipt), "P0001").await;
        let foreign = revision(&db.ingest, None, 'f').await;
        let foreign_page = page(&db.ingest, &foreign, "public/foreign.html").await;
        reject(&mut writer, &format!("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},1,'falsch','https://example.invalid',{})",public.id,foreign_page.receipt), "P0001").await;
        reject(&mut writer, &format!("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id,entity_ids) VALUES ({},1,'falsch','https://example.invalid',{},ARRAY[9223372036854775807])",public.id,public.receipt), "P0001").await;
        reject(&mut writer, "INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id,scope) VALUES (1,'public/fake.html','Fake','de',1,1,'internal')", "428C9").await;
        reject(
            &mut writer,
            &format!(
                "UPDATE docs.page SET title='verändert' WHERE id={}",
                public.id
            ),
            "42501",
        )
        .await;
        reject(
            &mut writer,
            "UPDATE docs.section SET body='verändert'",
            "42501",
        )
        .await;
        drop(writer);
        let mut owner = db.owner.acquire().await.unwrap();
        reject(
            &mut owner,
            "UPDATE docs.section SET body='verändert'",
            "P0001",
        )
        .await;
        reject(&mut owner, "DELETE FROM docs.section", "P0001").await;
        reject(
            &mut owner,
            &format!(
                "UPDATE docs.page SET title='verändert' WHERE id={}",
                public.id
            ),
            "P0001",
        )
        .await;
        drop(owner);
        publish(&db.ingest, &first).await.unwrap();
        assert_eq!(count(&db.owner, "public").await, 1);
        assert_eq!(count(&db.owner, "internal").await, 1);
        let mut reader = db.reader.acquire().await.unwrap();
        sqlx::query("SET SESSION ROLE brain_tool")
            .execute(&mut *reader)
            .await
            .unwrap();
        let rows = sqlx::query("SELECT scope,path,receipt_id,source_revision FROM docs.current_section WHERE scope='public'")
            .fetch_all(&mut *reader).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get::<String, _>("path"), "public/hero.html");
        assert_eq!(rows[0].get::<i64, _>("receipt_id"), public.receipt);
        assert_eq!(rows[0].get::<String, _>("source_revision"), first.target);
        reject(&mut reader, "INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES (1,1,'x','https://example.invalid',1)", "42501").await;
        drop(reader);
        let second = revision(&db.ingest, Some(first.source), 'b').await;
        sqlx::query("INSERT INTO raw.run_object(run_id,path,action,parse_status) VALUES ($1,'public/hero.html','delete','succeeded')")
            .bind(second.run).execute(&db.ingest).await.unwrap();
        sqlx::query("UPDATE docs.page SET valid_to_revision_id=$2 WHERE id=$1")
            .bind(public.id)
            .bind(second.id)
            .execute(&db.ingest)
            .await
            .unwrap();
        assert_eq!(count(&db.owner, "public").await, 1);
        publish(&db.ingest, &second).await.unwrap();
        assert_eq!(count(&db.owner, "public").await, 0);
        assert_eq!(count(&db.owner, "internal").await, 1);
        let mut writer = db.ingest.acquire().await.unwrap();
        reject(
            &mut writer,
            &format!(
                "UPDATE docs.page SET valid_to_revision_id={} WHERE id={}",
                second.id, public.id
            ),
            "P0001",
        )
        .await;
        drop(writer);
        check(&mut db.owner.acquire().await.unwrap())
            .await
            .unwrap();
        db.close().await;
    }

    #[tokio::test]
    async fn real_postgres_docs_failed_replacement_stays_unpublished() {
        let db = Database::new().await;
        let first = revision(&db.ingest, None, 'a').await;
        let old = page(&db.ingest, &first, "public/hero.html").await;
        section(&db.ingest, &old, "Alter Abschnitt").await;
        publish(&db.ingest, &first).await.unwrap();
        let second = revision(&db.ingest, Some(first.source), 'b').await;
        sqlx::query("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,'public/hero.html',$2,decode(repeat('03',32),'hex'),'text/html','replacement')")
            .bind(first.source).bind(&second.target).execute(&db.ingest).await.unwrap();
        let object: i64 = sqlx::query_scalar("SELECT max(id) FROM raw.object")
            .fetch_one(&db.owner)
            .await
            .unwrap();
        sqlx::query("INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES ($1,'public/hero.html',$2,'upsert','succeeded')")
            .bind(second.run).bind(object).execute(&db.ingest).await.unwrap();
        let error = publish(&db.ingest, &second).await.unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("P0001")
        );
        sqlx::query("UPDATE docs.page SET valid_to_revision_id=$2 WHERE id=$1")
            .bind(old.id)
            .bind(second.id)
            .execute(&db.ingest)
            .await
            .unwrap();
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,'line_range','L1-L2',decode(repeat('04',32),'hex'),'docs','v1') RETURNING id")
            .bind(object).bind(first.source).bind(second.id).fetch_one(&db.ingest).await.unwrap();
        let id: i64 = sqlx::query_scalar("INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id) VALUES ($1,'public/hero.html','Neu','de',$2,$3) RETURNING id")
            .bind(first.source).bind(object).bind(second.id).fetch_one(&db.ingest).await.unwrap();
        assert!(publish(&db.ingest, &second).await.is_err());
        let visible: String = sqlx::query_scalar("SELECT body FROM docs.current_section")
            .fetch_one(&db.owner)
            .await
            .unwrap();
        assert_eq!(visible, "Alter Abschnitt");
        let new = Page {
            id,
            receipt,
            url: format!(
                "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/public/hero.html#L1-L2",
                second.target
            ),
        };
        let mut writer = db.ingest.acquire().await.unwrap();
        reject(&mut writer, &format!("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},0,'falsch','https://example.invalid',{})",new.id,old.receipt), "P0001").await;
        drop(writer);
        section(&db.ingest, &new, "Neuer Abschnitt").await;
        publish(&db.ingest, &second).await.unwrap();
        let row = sqlx::query("SELECT body,receipt_id,source_revision FROM docs.current_section")
            .fetch_one(&db.owner)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>("body"), "Neuer Abschnitt");
        assert_eq!(row.get::<i64, _>("receipt_id"), receipt);
        assert_eq!(row.get::<String, _>("source_revision"), second.target);
        let third = revision(&db.ingest, Some(first.source), 'c').await;
        publish(&db.ingest, &third).await.unwrap();
        assert_eq!(count(&db.owner, "public").await, 1);
        let objects: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
            .fetch_one(&db.owner)
            .await
            .unwrap();
        assert_eq!(objects, 2);
        PgStore::new(db.owner.clone())
            .migrate_gamefacts()
            .await
            .unwrap();
        check(&mut db.owner.acquire().await.unwrap())
            .await
            .unwrap();
        db.close().await;
    }

    #[tokio::test]
    async fn real_postgres_docs_bounds_indexes_and_shared_write_lock() {
        let db = Database::new().await;
        let revision = revision(&db.ingest, None, 'a').await;
        let bounded = page(&db.ingest, &revision, "public/bounded.html").await;
        let page = page(&db.ingest, &revision, "public/hero.html").await;
        let entity: i64 = sqlx::query_scalar("INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','contract',$1) RETURNING id")
            .bind(page.receipt).fetch_one(&db.ingest).await.unwrap();
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,heading_path,body,entity_ids,url,receipt_id) VALUES ($1,0,array_fill(repeat('ü',128),ARRAY[16]),'Fähigkeiten kontrollieren die frühe Lane',array_fill($2::bigint,ARRAY[64]),$3,$4)")
            .bind(page.id).bind(entity).bind(&page.url).bind(page.receipt).execute(&db.ingest).await.unwrap();
        let mut writer = db.ingest.acquire().await.unwrap();
        for (expression, code) in [("0", "23505"), ("-1", "23514"), ("4096", "23514")] {
            reject(&mut writer, &format!("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},{expression},'x','https://example.invalid',{})",page.id,page.receipt),code).await;
        }
        for columns in ["heading_path,body,url,receipt_id", "body,url,receipt_id"] {
            let values = if columns.starts_with("heading_path") {
                format!(
                    "array_fill('x'::text,ARRAY[17]),'x','https://example.invalid',{}",
                    page.receipt
                )
            } else {
                format!(
                    "repeat('x',262145),'https://example.invalid',{}",
                    page.receipt
                )
            };
            reject(
                &mut writer,
                &format!(
                    "INSERT INTO docs.section(page_id,ordinal,{columns}) VALUES ({},1,{values})",
                    page.id
                ),
                "23514",
            )
            .await;
        }
        for (body, url) in [
            ("''", "'https://example.invalid'"),
            ("repeat('ü',131073)", "'https://example.invalid'"),
            ("'x'", "'http://example.invalid'"),
            ("'x'", "'https://example.invalid/with space'"),
            ("'x'", "'https://'||repeat('x',2041)"),
        ] {
            reject(
                &mut writer,
                &format!(
                    "INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},1,{body},{url},{})",
                    page.id, page.receipt
                ),
                "23514",
            )
            .await;
        }
        drop(writer);
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ($1,2,repeat('ü',131072),$2,$3)")
            .bind(page.id).bind(&page.url).bind(page.receipt).execute(&db.ingest).await.unwrap();
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) SELECT $1,ordinal,repeat('x',262144),$2,$3 FROM generate_series(0,31) ordinal")
            .bind(bounded.id).bind(&bounded.url).bind(bounded.receipt).execute(&db.ingest).await.unwrap();
        let mut writer = db.ingest.acquire().await.unwrap();
        reject(
            &mut writer,
            &format!(
                "INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ({},32,'x','https://example.invalid',{})",
                bounded.id, bounded.receipt
            ),
            "P0001",
        )
        .await;
        drop(writer);
        for sql in [
            "SELECT docs.valid_heading_path(ARRAY[''])",
            "SELECT docs.valid_heading_path(ARRAY[NULL]::text[])",
            "SELECT docs.valid_heading_path('[0:0]={x}'::text[])",
            "SELECT docs.valid_heading_path(ARRAY[['x'],['y']])",
            "SELECT docs.valid_heading_path(ARRAY[repeat('ü',129)])",
            "SELECT docs.valid_entity_ids(ARRAY[0]::bigint[])",
            "SELECT docs.valid_entity_ids(ARRAY[NULL]::bigint[])",
            "SELECT docs.valid_entity_ids('[0:0]={1}'::bigint[])",
            "SELECT docs.valid_entity_ids(ARRAY[[1],[2]]::bigint[])",
            "SELECT docs.valid_entity_ids(array_fill(1::bigint,ARRAY[65]))",
        ] {
            let valid: bool = sqlx::query_scalar(sql).fetch_one(&db.owner).await.unwrap();
            assert!(!valid, "{sql}");
        }
        let mut owner = db.owner.acquire().await.unwrap();
        sqlx::query("SET enable_seqscan=off")
            .execute(&mut *owner)
            .await
            .unwrap();
        for (predicate, index) in [
            (
                "tsv @@ websearch_to_tsquery('german','fähigkeiten')",
                "section_tsv_idx",
            ),
            (
                &format!("entity_ids @> ARRAY[{entity}]::bigint[]"),
                "section_entity_ids_idx",
            ),
        ] {
            let plan: Value = sqlx::query_scalar(&format!(
                "EXPLAIN (FORMAT JSON) SELECT id FROM docs.section WHERE {predicate}"
            ))
            .fetch_one(&mut *owner)
            .await
            .unwrap();
            assert!(plan.to_string().contains(index), "{plan}");
        }
        drop(owner);
        let matches: i64 = sqlx::query_scalar("SELECT count(*) FROM docs.section WHERE tsv @@ websearch_to_tsquery('german','fähigkeiten')")
            .fetch_one(&db.owner).await.unwrap();
        assert_eq!(matches, 1);
        let mut blocker = db.owner.begin().await.unwrap();
        sqlx::query("SELECT pg_advisory_xact_lock(742110026114::bigint)")
            .execute(&mut *blocker)
            .await
            .unwrap();
        let blocker_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *blocker)
            .await
            .unwrap();
        let mut blocked = db.ingest.acquire().await.unwrap();
        let blocked_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *blocked)
            .await
            .unwrap();
        let waiting = tokio::spawn(async move {
            sqlx::query("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ($1,1,'zweiter Abschnitt',$2,$3)")
                .bind(page.id).bind(&page.url).bind(page.receipt).execute(&mut *blocked).await.unwrap();
        });
        tokio::time::timeout(std::time::Duration::from_secs(8), async {
            loop {
                let blocked: bool = sqlx::query_scalar("SELECT $2=ANY(pg_blocking_pids($1))")
                    .bind(blocked_pid)
                    .bind(blocker_pid)
                    .fetch_one(&db.owner)
                    .await
                    .unwrap();
                if blocked {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        blocker.commit().await.unwrap();
        waiting.await.unwrap();
        let mut writer = db.ingest.acquire().await.unwrap();
        sqlx::query("BEGIN ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *writer)
            .await
            .unwrap();
        reject(
            &mut writer,
            "UPDATE docs.page SET valid_to_revision_id=valid_from_revision_id+1",
            "0A000",
        )
        .await;
        sqlx::query("ROLLBACK").execute(&mut *writer).await.unwrap();
        drop(writer);
        publish(&db.ingest, &revision).await.unwrap();
        let mut service = db.service.acquire().await.unwrap();
        reject(&mut service, "SELECT * FROM docs.current_section", "42501").await;
        drop(service);
        let mut owner = db.owner.acquire().await.unwrap();
        sqlx::query("DROP INDEX docs.section_tsv_idx")
            .execute(&mut *owner)
            .await
            .unwrap();
        assert!(check(&mut owner).await.is_err());
        sqlx::query("CREATE INDEX section_tsv_idx ON docs.section USING gin(tsv)")
            .execute(&mut *owner)
            .await
            .unwrap();
        check(&mut owner).await.unwrap();
        let marker: Option<String> =
            sqlx::query_scalar("SELECT to_regclass('ops.docs_schema_version')::text")
                .fetch_one(&mut *owner)
                .await
                .unwrap();
        assert!(marker.is_none());
        drop(owner);
        PgStore::new(db.owner.clone())
            .check_gamefacts_schema()
            .await
            .unwrap();
        db.close().await;
    }

    #[tokio::test]
    async fn real_postgres_docs_preflight_rejects_same_shape_publication_view_drift() {
        let db = Database::new().await;
        let first = revision(&db.ingest, None, 'a').await;
        let old = page(&db.ingest, &first, "public/hero.html").await;
        section(&db.ingest, &old, "Historischer Abschnitt").await;
        publish(&db.ingest, &first).await.unwrap();
        let second = revision(&db.ingest, Some(first.source), 'b').await;
        sqlx::query("UPDATE docs.page SET valid_to_revision_id=$2 WHERE id=$1")
            .bind(old.id)
            .bind(second.id)
            .execute(&db.ingest)
            .await
            .unwrap();
        let current = page(&db.ingest, &second, "public/hero.html").await;
        section(&db.ingest, &current, "Veröffentlichter Abschnitt").await;
        publish(&db.ingest, &second).await.unwrap();
        let third = revision(&db.ingest, Some(first.source), 'c').await;
        let pending = page(&db.ingest, &third, "public/pending.html").await;
        section(&db.ingest, &pending, "Unveröffentlichter Abschnitt").await;
        assert_eq!(count(&db.owner, "public").await, 1);
        let original = MIGRATION
            .split_once("CREATE VIEW docs.current_section AS")
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0;
        let start = "p.valid_from_revision_id <= h.published_revision_id";
        let end = "(p.valid_to_revision_id IS NULL OR p.valid_to_revision_id > h.published_revision_id)";
        for (predicate, visible) in [
            (end.to_owned(), 2),
            (start.to_owned(), 2),
            (String::new(), 3),
            (format!("({start} AND {end}) OR TRUE"), 3),
            (
                format!("{start} AND p.valid_to_revision_id IS NULL OR p.valid_to_revision_id > h.published_revision_id"),
                1,
            ),
        ] {
            let prefix = original.split_once("WHERE").unwrap().0;
            let mutation = if predicate.is_empty() {
                format!("CREATE OR REPLACE VIEW docs.current_section AS {prefix}")
            } else {
                format!("CREATE OR REPLACE VIEW docs.current_section AS {prefix} WHERE {predicate}")
            };
            let mut tx = db.owner.begin().await.unwrap();
            sqlx::raw_sql(&mutation).execute(&mut *tx).await.unwrap();
            let columns: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_attribute WHERE attrelid='docs.current_section'::regclass AND attnum>0 AND NOT attisdropped")
                .fetch_one(&mut *tx).await.unwrap();
            assert_eq!(columns, 16);
            let sections: i64 = sqlx::query_scalar("SELECT count(*) FROM docs.current_section WHERE scope='public'")
                .fetch_one(&mut *tx).await.unwrap();
            assert_eq!(sections, visible, "{predicate}");
            assert!(check(&mut tx).await.is_err(), "{predicate}");
            tx.rollback().await.unwrap();
            check(&mut db.owner.acquire().await.unwrap())
                .await
                .unwrap();
        }
        db.close().await;
    }

    #[tokio::test]
    async fn real_postgres_docs_preflight_rejects_validated_same_count_weakened_body_check() {
        let db = Database::new().await;
        let revision = revision(&db.ingest, None, 'a').await;
        let page = page(&db.ingest, &revision, "public/bounded.html").await;
        let constraint: String = sqlx::query_scalar("SELECT c.conname::text FROM pg_constraint c JOIN pg_attribute a ON a.attrelid=c.conrelid AND a.attnum=ANY(c.conkey) WHERE c.conrelid='docs.section'::regclass AND c.contype='c' AND a.attname='body'")
            .fetch_one(&db.owner).await.unwrap();
        let constraint = constraint.replace('"', "\"\"");
        for predicate in [
            "octet_length(body) BETWEEN 1 AND 524288",
            "octet_length(body) BETWEEN 1 AND 262144 OR TRUE",
        ] {
            let mut tx = db.owner.begin().await.unwrap();
            sqlx::raw_sql(&format!("ALTER TABLE docs.section DROP CONSTRAINT \"{constraint}\"; ALTER TABLE docs.section ADD CONSTRAINT \"{constraint}\" CHECK ({predicate})"))
                .execute(&mut *tx).await.unwrap();
            let (checks, validated): (i64, bool) = sqlx::query_as("SELECT count(*),bool_and(convalidated) FROM pg_constraint WHERE conrelid='docs.section'::regclass AND contype='c'")
                .fetch_one(&mut *tx).await.unwrap();
            assert_eq!(checks, 5);
            assert!(validated);
            sqlx::query("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES ($1,0,repeat('x',262145),$2,$3)")
                .bind(page.id).bind(&page.url).bind(page.receipt).execute(&mut *tx).await.unwrap();
            let bytes: i32 = sqlx::query_scalar("SELECT octet_length(body) FROM docs.section WHERE page_id=$1")
                .bind(page.id).fetch_one(&mut *tx).await.unwrap();
            assert_eq!(bytes, 262145);
            assert!(check(&mut tx).await.is_err(), "{predicate}");
            tx.rollback().await.unwrap();
            check(&mut db.owner.acquire().await.unwrap())
                .await
                .unwrap();
        }
        db.close().await;
    }

    #[tokio::test]
    async fn real_postgres_docs_preflight_is_read_only_and_rejects_grant_shape_drift() {
        let db = Database::new().await;
        let mut read_only = db.owner.begin().await.unwrap();
        sqlx::query("SET TRANSACTION READ ONLY")
            .execute(&mut *read_only)
            .await
            .unwrap();
        check(&mut read_only).await.unwrap();
        let serializers: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_trigger WHERE tgfoid='ops.serialize_gamefacts_write()'::regprocedure AND tgenabled='O' AND tgtype & 1=0 AND tgtype & 2=2")
            .fetch_one(&mut *read_only).await.unwrap();
        assert_eq!(serializers, 14);
        read_only.commit().await.unwrap();
        let mut reader = db.reader.begin().await.unwrap();
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL ROLE brain_tool")
            .execute(&mut *reader)
            .await
            .unwrap();
        check(&mut reader).await.unwrap();
        reader.commit().await.unwrap();
        for pool in [&db.service, &db.reader, &db.ingest] {
            let mut tx = pool.begin().await.unwrap();
            sqlx::query("SET TRANSACTION READ ONLY")
                .execute(&mut *tx)
                .await
                .unwrap();
            check(&mut tx).await.unwrap();
            tx.commit().await.unwrap();
        }
        for mutation in [
            "GRANT SELECT ON docs.current_section TO brain_service",
            "GRANT UPDATE (title) ON docs.page TO brain_tool",
            "GRANT UPDATE (body) ON docs.section TO brain_ingest",
            "GRANT INSERT (id) ON docs.section TO brain_ingest",
            "REVOKE INSERT (body) ON docs.section FROM brain_ingest",
            "REVOKE SELECT ON docs.page FROM brain_ingest",
            "GRANT UPDATE ON SEQUENCE docs.section_id_seq TO brain_ingest",
            "REVOKE USAGE ON SEQUENCE docs.page_id_seq FROM brain_ingest",
            "GRANT EXECUTE ON FUNCTION docs.serialize_write() TO brain_ingest",
            "GRANT SELECT ON docs.current_section TO PUBLIC",
            "GRANT UPDATE (title) ON docs.page TO PUBLIC",
            "ALTER FUNCTION docs.valid_heading_path(text[]) VOLATILE",
            "ALTER FUNCTION docs.valid_entity_ids(bigint[]) SECURITY DEFINER",
            "ALTER FUNCTION docs.section_tsv(text[],text) PARALLEL UNSAFE",
            "ALTER TABLE docs.page ALTER COLUMN valid_to_revision_id SET NOT NULL",
            "ALTER TABLE docs.section DISABLE TRIGGER section_guard",
        ] {
            let mut tx = db.owner.begin().await.unwrap();
            sqlx::raw_sql(mutation)
                .execute(&mut *tx)
                .await
                .unwrap();
            assert!(check(&mut tx).await.is_err(), "{mutation}");
            tx.rollback().await.unwrap();
            check(&mut db.owner.acquire().await.unwrap())
                .await
                .unwrap();
        }
        db.close().await;
    }
}
