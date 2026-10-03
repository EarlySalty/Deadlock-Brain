use crate::{ident, require, Error, Result, SchemaFingerprint};
use postgres::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SequenceFingerprint {
    pub name: String,
    pub oid: u32,
    pub table: String,
    pub column: String,
    pub identity: bool,
    pub data_type: String,
    pub start: i64,
    pub increment: i64,
    pub minimum: i64,
    pub maximum: i64,
    pub cache: i64,
    pub cycle: bool,
    pub value: i64,
    pub called: bool,
    pub owner: String,
    pub acl: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Column {
    name: String,
    data_type: String,
    not_null: bool,
    identity: String,
    default: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Constraint {
    name: String,
    definition: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Index {
    name: String,
    unique: bool,
    columns: Vec<(String, i16)>,
    predicate: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Shape {
    columns: Vec<Column>,
    constraints: Vec<Constraint>,
    indexes: Vec<Index>,
}

fn literal(expression: &str) -> bool {
    let bytes = expression.as_bytes();
    if bytes.first() != Some(&b'\'') || bytes.last() != Some(&b'\'') {
        return false;
    }
    let mut pos = 1;
    while pos + 1 < bytes.len() {
        if bytes[pos] == b'\'' {
            if bytes.get(pos + 1) != Some(&b'\'') || pos + 2 >= bytes.len() {
                return false;
            }
            pos += 2;
        } else {
            pos += 1;
        }
    }
    pos == bytes.len() - 1
}
fn allowed_type(data_type: &str) -> bool {
    matches!(
        data_type,
        "smallint"
            | "integer"
            | "bigint"
            | "text"
            | "boolean"
            | "double precision"
            | "jsonb"
            | "timestamp with time zone"
            | "uuid"
            | "smallint[]"
            | "integer[]"
            | "bigint[]"
            | "text[]"
    )
}
fn checked_default(
    expression: &str,
    schema: &str,
    sequences: &[SequenceFingerprint],
) -> Result<String> {
    if expression.is_empty()
        || matches!(expression, "now()" | "gen_random_uuid()" | "true" | "false")
    {
        return Ok(expression.into());
    }
    if expression
        .bytes()
        .all(|b| b.is_ascii_digit() || b == b'-' || b == b'.')
        && expression.bytes().any(|b| b.is_ascii_digit())
    {
        return Ok(expression.into());
    }
    if let Some((value, ty)) = expression.rsplit_once("::") {
        if literal(value) && allowed_type(ty) {
            return Ok(expression.into());
        }
    }
    for seq in sequences {
        if expression == format!("nextval('{schema}.{}'::regclass)", seq.name) {
            return Ok(format!("nextval('__archive__.{}'::regclass)", seq.name));
        }
    }
    Err(Error::Contract(
        "Defaultklasse benötigt einen geprüften Adapter",
    ))
}
fn check_enum(expression: &str, columns: &[Column]) -> bool {
    let Some(body) = expression
        .strip_prefix("CHECK ((")
        .and_then(|s| s.strip_suffix(")))"))
    else {
        return false;
    };
    let Some((column, values)) = body.split_once(" = ANY (ARRAY[") else {
        return false;
    };
    let Some(values) = values.strip_suffix(']') else {
        return false;
    };
    if !columns
        .iter()
        .any(|c| c.name == column && c.data_type == "text")
    {
        return false;
    }
    let mut rest = values;
    let mut count = 0;
    loop {
        if !rest.starts_with('\'') {
            return false;
        }
        let b = rest.as_bytes();
        let mut pos = 1;
        loop {
            if pos >= b.len() {
                return false;
            }
            if b[pos] == b'\'' {
                if b.get(pos + 1) == Some(&b'\'') {
                    pos += 2;
                } else {
                    break;
                }
            } else {
                pos += 1;
            }
        }
        rest = &rest[pos + 1..];
        let Some(next) = rest.strip_prefix("::text") else {
            return false;
        };
        count += 1;
        if next.is_empty() {
            return count > 0;
        }
        let Some(next) = next.strip_prefix(", ") else {
            return false;
        };
        rest = next;
    }
}
fn checked_index_expression(expression: &str, columns: &[Column]) -> Result<String> {
    let body = expression
        .strip_prefix("COALESCE(")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or(Error::Contract("Indexausdruckklasse nicht unterstützt"))?;
    let (left, right) = body
        .split_once(", ")
        .ok_or(Error::Contract("Indexausdruck nicht eindeutig"))?;
    let left_column = columns
        .iter()
        .find(|c| c.name == left)
        .ok_or(Error::Contract("Indexspalte fehlt"))?;
    let right = if let Some(right_column) = columns.iter().find(|c| c.name == right) {
        require(
            right_column.data_type == left_column.data_type,
            "Impliziter Indexcast nicht unterstützt",
        )?;
        ident(&right_column.name)?
    } else {
        let value = right
            .strip_suffix("::text")
            .ok_or(Error::Contract("Indexliteraltyp nicht unterstützt"))?;
        require(
            left_column.data_type == "text" && literal(value),
            "Unsicheres Indexliteral",
        )?;
        right.into()
    };
    Ok(format!("COALESCE({}, {right})", ident(left)?))
}
fn checked_predicate(predicate: &str, columns: &[Column]) -> Result<String> {
    if predicate.is_empty() {
        return Ok(String::new());
    }
    let column = predicate
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(" IS NOT NULL)"))
        .ok_or(Error::Contract("Partialindexklasse nicht unterstützt"))?;
    require(
        columns.iter().any(|c| c.name == column),
        "Partialindexspalte fehlt",
    )?;
    Ok(format!("({} IS NOT NULL)", ident(column)?))
}
fn column_names(tx: &mut Transaction<'_>, table: u32, numbers: &[i16]) -> Result<Vec<String>> {
    numbers.iter().map(|number| {
        let name: String = tx.query_one("SELECT attname FROM pg_attribute WHERE attrelid=$1 AND attnum=$2 AND NOT attisdropped", &[&table, number])?.get(0);
        ident(&name)
    }).collect()
}
fn action(code: i8) -> Result<&'static str> {
    match code as u8 {
        b'a' => Ok("NO ACTION"),
        b'r' => Ok("RESTRICT"),
        b'c' => Ok("CASCADE"),
        b'n' => Ok("SET NULL"),
        b'd' => Ok("SET DEFAULT"),
        _ => Err(Error::Contract("Unbekannte Fremdschlüsselaktion")),
    }
}
type TableShapes = Vec<(u32, String, String)>;
pub(crate) fn schema_shape(
    tx: &mut Transaction<'_>,
    schema: &str,
    oid: u32,
    excluded: &[String],
) -> Result<(TableShapes, Vec<SequenceFingerprint>)> {
    let mut sequences = Vec::new();
    for row in tx.query("SELECT s.oid,s.relname,t.relname,a.attname,d.deptype,format_type(q.seqtypid,NULL),q.seqstart,q.seqincrement,q.seqmin,q.seqmax,q.seqcache,q.seqcycle,pg_get_userbyid(s.relowner),coalesce((SELECT json_agg(json_build_array(x.grantee,x.grantor,x.privilege_type,x.is_grantable) ORDER BY x.grantee,x.grantor,x.privilege_type)::text FROM aclexplode(coalesce(s.relacl,acldefault('s',s.relowner))) x),'[]'), t.relnamespace FROM pg_class s JOIN pg_sequence q ON q.seqrelid=s.oid LEFT JOIN pg_depend d ON d.classid='pg_class'::regclass AND d.objid=s.oid AND d.refclassid='pg_class'::regclass AND d.deptype IN ('a','i') LEFT JOIN pg_class t ON t.oid=d.refobjid LEFT JOIN pg_attribute a ON a.attrelid=t.oid AND a.attnum=d.refobjsubid WHERE s.relnamespace=$1 ORDER BY s.relname COLLATE \"C\"", &[&oid])? {
        let table: Option<String> = row.get(2);
        let table = table.ok_or(Error::Contract("Sequenz ohne eindeutige lokale Spaltenbindung"))?;
        if excluded.contains(&table) { continue; }
        require(row.get::<_, Option<u32>>(14) == Some(oid), "Sequenz verweist außerhalb des Archivs")?;
        let name: String = row.get(1);
        let column: String = row.get::<_, Option<String>>(3).ok_or(Error::Contract("Sequenzspalte fehlt"))?;
        ident(&table)?; ident(&column)?;
        let state = tx.query_one(&format!("SELECT last_value,is_called FROM {}.{}", ident(schema)?, ident(&name)?), &[])?;
        sequences.push(SequenceFingerprint { name, oid: row.get(0), table, column, identity: row.get::<_, Option<i8>>(4) == Some(b'i' as i8), data_type: row.get(5), start: row.get(6), increment: row.get(7), minimum: row.get(8), maximum: row.get(9), cache: row.get(10), cycle: row.get(11), value: state.get(0), called: state.get(1), owner: row.get(12), acl: row.get(13) });
    }
    let mut tables = Vec::new();
    for table in tx.query("SELECT oid,relname FROM pg_class WHERE relnamespace=$1 AND relkind='r' AND relname <> ALL($2) ORDER BY relname COLLATE \"C\"", &[&oid, &excluded])? {
        let table_oid: u32 = table.get(0);
        let name: String = table.get(1);
        ident(&name)?;
        let metadata_bad: i64 = tx.query_one("SELECT (SELECT count(*) FROM pg_class c JOIN pg_am a ON a.oid=c.relam WHERE c.oid=$1 AND (a.amname<>'heap' OR c.reloptions IS NOT NULL OR EXISTS(SELECT FROM pg_inherits WHERE inhrelid=c.oid))) + (SELECT count(*) FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid JOIN pg_am a ON a.oid=c.relam WHERE i.indrelid=$1 AND (a.amname<>'btree' OR NOT (i.indisvalid AND i.indisready AND i.indislive) OR i.indnatts<>i.indnkeyatts OR i.indnullsnotdistinct OR c.reloptions IS NOT NULL OR EXISTS(SELECT FROM unnest(i.indcollation::oid[]) k JOIN pg_collation co ON co.oid=k WHERE co.collname<>'default' OR co.collnamespace<>'pg_catalog'::regnamespace) OR (EXISTS(SELECT FROM pg_constraint k WHERE k.conindid=i.indexrelid AND k.contype IN ('p','u')) AND EXISTS(SELECT FROM unnest(i.indoption::smallint[]) o WHERE o<>0)))) + (SELECT count(*) FROM pg_constraint WHERE conrelid=$1 AND confdelsetcols IS NOT NULL)", &[&table_oid])?.get(0);
        require(metadata_bad == 0, "Vererbung, Storageoptionen oder nicht unterstützte Constraint-/Indexsemantik")?;
        let mut columns = Vec::new();
        for row in tx.query("SELECT a.attname,format_type(a.atttypid,a.atttypmod),a.attnotnull,a.attidentity::text,coalesce(pg_get_expr(d.adbin,d.adrelid),''),n.nspname,a.attgenerated::text,coalesce(co.collname,''),coalesce(cn.nspname,'') FROM pg_attribute a JOIN pg_type t ON t.oid=a.atttypid JOIN pg_namespace n ON n.oid=t.typnamespace LEFT JOIN pg_attrdef d ON d.adrelid=a.attrelid AND d.adnum=a.attnum LEFT JOIN pg_collation co ON co.oid=a.attcollation LEFT JOIN pg_namespace cn ON cn.oid=co.collnamespace WHERE a.attrelid=$1 AND a.attnum>0 AND NOT a.attisdropped ORDER BY a.attnum", &[&table_oid])? {
            let col: String = row.get(0);
            let ty: String = row.get(1);
            let identity: String = row.get(3);
            ident(&col)?;
            require(row.get::<_, String>(5) == "pg_catalog" && allowed_type(&ty) && row.get::<_, String>(6).is_empty() && (row.get::<_, String>(8).is_empty() || (row.get::<_, String>(8) == "pg_catalog" && row.get::<_, String>(7) == "default")), "Nicht unterstützter Typ, generierte Spalte oder Kollation")?;
            require(matches!(identity.as_str(), "" | "a" | "d"), "Unbekannte Identityklasse")?;
            let default = checked_default(&row.get::<_, String>(4), schema, &sequences)?;
            if !identity.is_empty() { require(sequences.iter().filter(|s| s.table == name && s.column == col && s.identity).count() == 1, "Identitysequenz nicht eindeutig gebunden")?; }
            columns.push(Column { name: col, data_type: ty, not_null: row.get(2), identity, default });
        }
        require(!columns.is_empty(), "Tabelle ohne Spalten")?;
        let mut constraints = Vec::new();
        for row in tx.query("SELECT conname,contype,conkey,confrelid,confkey,confupdtype,confdeltype,confmatchtype,condeferrable,condeferred,convalidated,pg_get_constraintdef(oid,false) FROM pg_constraint WHERE conrelid=$1 ORDER BY conname COLLATE \"C\"", &[&table_oid])? {
            let cname: String = row.get(0); ident(&cname)?;
            require(row.get::<_, bool>(10), "Nicht validierte Constraint")?;
            let definition = match row.get::<_, i8>(1) as u8 {
                b'p' | b'u' => {
                    let cols = column_names(tx, table_oid, &row.get::<_, Vec<i16>>(2))?;
                    format!("{} ({}){}", if row.get::<_, i8>(1) as u8 == b'p' { "PRIMARY KEY" } else { "UNIQUE" }, cols.join(", "), if row.get::<_, bool>(8) { if row.get::<_, bool>(9) { " DEFERRABLE INITIALLY DEFERRED" } else { " DEFERRABLE INITIALLY IMMEDIATE" } } else { "" })
                }
                b'f' => {
                    let target_oid: u32 = row.get(3);
                    let target = tx.query_one("SELECT relname,relnamespace FROM pg_class WHERE oid=$1", &[&target_oid])?;
                    let target_name: String = target.get(0);
                    require(target.get::<_, u32>(1) == oid && !excluded.contains(&target_name), "Fremdschlüssel außerhalb übernommener Tabellen")?;
                    require(row.get::<_, i8>(7) as u8 == b's', "Nicht unterstützter Fremdschlüsselmatch")?;
                    format!("FOREIGN KEY ({}) REFERENCES __archive__.{} ({}) ON UPDATE {} ON DELETE {}{}", column_names(tx, table_oid, &row.get::<_, Vec<i16>>(2))?.join(", "), ident(&target_name)?, column_names(tx, target_oid, &row.get::<_, Vec<i16>>(4))?.join(", "), action(row.get(5))?, action(row.get(6))?, if row.get::<_, bool>(8) { if row.get::<_, bool>(9) { " DEFERRABLE INITIALLY DEFERRED" } else { " DEFERRABLE INITIALLY IMMEDIATE" } } else { "" })
                }
                b'c' => {
                    let expr: String = row.get(11);
                    require(check_enum(&expr, &columns), "Checkklasse benötigt einen geprüften Adapter")?;
                    expr
                }
                _ => return Err(Error::Contract("Nicht unterstützte Constraintklasse")),
            };
            constraints.push(Constraint { name: cname, definition });
        }
        let mut indexes = Vec::new();
        for row in tx.query("SELECT c.relname,i.indisunique,i.indkey::smallint[],i.indoption::smallint[],a.amname,i.indisvalid AND i.indisready AND i.indislive,i.indexrelid,coalesce(pg_get_expr(i.indpred,i.indrelid),''),i.indnatts=i.indnkeyatts FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid JOIN pg_am a ON a.oid=c.relam WHERE i.indrelid=$1 AND NOT EXISTS(SELECT FROM pg_constraint k WHERE k.conindid=i.indexrelid AND k.contype IN ('p','u')) ORDER BY c.relname COLLATE \"C\"", &[&table_oid])? {
            let iname: String = row.get(0); ident(&iname)?;
            require(row.get::<_, String>(4) == "btree" && row.get::<_, bool>(5) && row.get::<_, bool>(8), "Nicht unterstützter Indexadapter")?;
            let numbers: Vec<i16> = row.get(2);
            let mut keys = Vec::new();
            for (position, number) in numbers.iter().enumerate() {
                if *number > 0 { keys.push(column_names(tx, table_oid, &[*number])?.remove(0)); }
                else {
                    require(*number == 0, "Systemspaltenindex nicht unterstützt")?;
                    let expression: String = tx.query_one("SELECT pg_get_indexdef($1,$2,false)", &[&row.get::<_,u32>(6), &((position+1) as i32)])?.get(0);
                    keys.push(format!("({})", checked_index_expression(&expression, &columns)?));
                }
            }
            let options: Vec<i16> = row.get(3);
            require(keys.len() == options.len() && options.iter().all(|v| (0..=3).contains(v)), "Indexoptionen nicht unterstützt")?;
            let predicate = checked_predicate(&row.get::<_,String>(7), &columns)?;
            indexes.push(Index { name: iname, unique: row.get(1), columns: keys.into_iter().zip(options).collect(), predicate });
        }
        let custom: i64 = tx.query_one("SELECT count(*) FROM pg_index i CROSS JOIN LATERAL unnest(i.indclass::oid[]) o JOIN pg_opclass c ON c.oid=o JOIN pg_namespace n ON n.oid=c.opcnamespace WHERE i.indrelid=$1 AND (n.nspname <> 'pg_catalog' OR NOT c.opcdefault)", &[&table_oid])?.get(0);
        require(custom == 0, "Eigene oder nicht standardmäßige Index-Operatorklasse")?;
        tables.push((table_oid, name, serde_json::to_string(&Shape { columns, constraints, indexes })?));
    }
    Ok((tables, sequences))
}

pub(crate) fn create_structure(
    tx: &mut Transaction<'_>,
    generation: &str,
    source: &SchemaFingerprint,
) -> Result<()> {
    let schema = ident(generation)?;
    for table in &source.tables {
        let shape: Shape = serde_json::from_str(&table.definition)?;
        let mut columns = Vec::new();
        for col in &shape.columns {
            columns.push(format!(
                "{} {}{}",
                ident(&col.name)?,
                col.data_type,
                if col.not_null { " NOT NULL" } else { "" }
            ));
        }
        tx.batch_execute(&format!(
            "CREATE TABLE {schema}.{} ({})",
            ident(&table.name)?,
            columns.join(", ")
        ))?;
    }
    for seq in &source.sequences {
        let cycle = if seq.cycle { "CYCLE" } else { "NO CYCLE" };
        let options = format!(
            "START WITH {} INCREMENT BY {} MINVALUE {} MAXVALUE {} CACHE {} {cycle}",
            seq.start, seq.increment, seq.minimum, seq.maximum, seq.cache
        );
        if seq.identity {
            let table = source
                .tables
                .iter()
                .find(|t| t.name == seq.table)
                .ok_or(Error::Contract("Identitytabelle fehlt"))?;
            let shape: Shape = serde_json::from_str(&table.definition)?;
            let column = shape
                .columns
                .iter()
                .find(|c| c.name == seq.column)
                .ok_or(Error::Contract("Identityspalte fehlt"))?;
            tx.batch_execute(&format!("ALTER TABLE {schema}.{} ALTER COLUMN {} ADD GENERATED {} AS IDENTITY (SEQUENCE NAME {schema}.{} {options})", ident(&seq.table)?, ident(&seq.column)?, if column.identity == "a" { "ALWAYS" } else { "BY DEFAULT" }, ident(&seq.name)?))?;
        } else {
            require(
                matches!(seq.data_type.as_str(), "smallint" | "integer" | "bigint"),
                "Sequenztyp nicht unterstützt",
            )?;
            tx.batch_execute(&format!(
                "CREATE SEQUENCE {schema}.{} AS {} {options} OWNED BY {schema}.{}.{}",
                ident(&seq.name)?,
                seq.data_type,
                ident(&seq.table)?,
                ident(&seq.column)?
            ))?;
        }
        tx.query_one(
            "SELECT pg_catalog.setval($1::text::regclass,$2,$3)",
            &[
                &format!("{generation}.{}", seq.name),
                &seq.value,
                &seq.called,
            ],
        )?;
    }
    for table in &source.tables {
        let shape: Shape = serde_json::from_str(&table.definition)?;
        for col in &shape.columns {
            if !col.default.is_empty() {
                let default = if col.default.starts_with("nextval('__archive__.") {
                    col.default
                        .replacen("__archive__.", &format!("{generation}."), 1)
                } else {
                    col.default.clone()
                };
                tx.batch_execute(&format!(
                    "ALTER TABLE {schema}.{} ALTER COLUMN {} SET DEFAULT {default}",
                    ident(&table.name)?,
                    ident(&col.name)?
                ))?;
            }
        }
        for constraint in shape
            .constraints
            .iter()
            .filter(|c| !c.definition.starts_with("FOREIGN KEY"))
        {
            tx.batch_execute(&format!(
                "ALTER TABLE {schema}.{} ADD CONSTRAINT {} {}",
                ident(&table.name)?,
                ident(&constraint.name)?,
                constraint.definition
            ))?;
        }
        for index in &shape.indexes {
            let columns: Vec<_> = index
                .columns
                .iter()
                .map(|(col, opt)| {
                    format!(
                        "{col} {} NULLS {}",
                        if opt & 1 == 1 { "DESC" } else { "ASC" },
                        if opt & 2 == 2 { "FIRST" } else { "LAST" }
                    )
                })
                .collect();
            tx.batch_execute(&format!(
                "CREATE {}INDEX {} ON {schema}.{} USING btree ({}){}",
                if index.unique { "UNIQUE " } else { "" },
                ident(&index.name)?,
                ident(&table.name)?,
                columns.join(", "),
                if index.predicate.is_empty() {
                    String::new()
                } else {
                    format!(" WHERE {}", index.predicate)
                }
            ))?;
        }
    }
    Ok(())
}
pub(crate) fn add_foreign_keys(
    tx: &mut Transaction<'_>,
    generation: &str,
    source: &SchemaFingerprint,
) -> Result<()> {
    for table in &source.tables {
        let shape: Shape = serde_json::from_str(&table.definition)?;
        for constraint in shape
            .constraints
            .iter()
            .filter(|c| c.definition.starts_with("FOREIGN KEY"))
        {
            tx.batch_execute(&format!(
                "ALTER TABLE {}.{} ADD CONSTRAINT {} {}",
                ident(generation)?,
                ident(&table.name)?,
                ident(&constraint.name)?,
                constraint.definition.replacen(
                    "__archive__.",
                    &format!("{}.", ident(generation)?),
                    1
                )
            ))?;
        }
    }
    Ok(())
}
