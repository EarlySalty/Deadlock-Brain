//! Phase-1 SQLite -> zentrale Postgres One-Shot-Migration fuer das Deadlock-Brain.
//!
//! Laedt die 36 Basistabellen der Brain-SQLite deterministisch und FK-sicher nach
//! `brain.*` in Postgres. Liest SQLite strikt read-only. Schreibt PG in EINER
//! Transaktion (DELETE der Ziel-Tabellen in umgekehrter FK-Reihenfolge -> Reload).
//!
//! NICHT Teil dieser Migration (Phase 1b, bleiben unberuehrt):
//!   brain.knowledge_events, brain.current_entity_state, brain.insight_records.
//!
//! FK-Aufloesung: Surrogat-PK-Tabellen bekommen frische BIGSERIAL-`id`; die alte
//! SQLite-PK landet in `legacy_sqlite_id`, jeder FK-Rohwert in `legacy_<fk>_id`.
//! Die echte `<fk>_id` wird beim Insert direkt aus einer In-Memory-Map
//! (legacy_sqlite_id -> neue id des Parents) aufgeloest. Inline-Resolve statt
//! Post-hoc-UPDATE, weil mehrere Ziel-FK-Spalten NOT NULL sind (hero_stat_*,
//! sheet_*), fuer die "erst NULL, dann UPDATE" schema-technisch unmoeglich ist.
//! Ergebnis ist identisch: korrekt aufgeloeste FKs + vollstaendige legacy-Provenienz.

use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use clap::Parser;
use postgres::types::ToSql;
use postgres::{Client, NoTls};
use rusqlite::types::Value as SqliteValue;
use rusqlite::OpenFlags;

/// Ladereihenfolge Tier 0->4: jede Tabelle steht hinter allen ihren Parents.
/// DELETE laeuft in umgekehrter Reihenfolge.
const LOAD_ORDER: &[&str] = &[
    // Tier 0 - keine brain-FK
    "source_runs",
    "source_documents",
    "hero_catalog",
    "item_catalog",
    "youtube_feed_sources",
    "learned_builds",
    "analysis_notes",
    "player_match_decision_notes",
    "meta_trend_notes",
    "mechanic_notes",
    // Tier 1
    "entity_snapshots",
    "youtube_videos",
    "hero_ability_orders",
    "hero_item_stats",
    "hero_item_synergies",
    "build_learning_notes",
    // Tier 2
    "entities",
    "patch_events",
    "forum_claims",
    "sheet_boons_ap",
    "sheet_items",
    "sheet_shop_bonuses",
    "sheet_tab_rows",
    "youtube_transcripts",
    "youtube_learning_claims",
    "youtube_transcript_claim_attempts",
    // Tier 3
    "entity_aliases",
    "patch_event_enrichments",
    "entity_lineage",
    "legacy_entities",
    "patch_impact_notes",
    "hero_stat_profiles",
    "sheet_hero_rankings",
    "sheet_heroes_stats",
    "sheet_raw_heroes",
    // Tier 4
    "hero_stat_values",
];

/// Surrogat-Tabellen, deren neue id von Kindern referenziert wird -> Map noetig.
const PARENT_TABLES: &[&str] = &[
    "source_documents",
    "entity_snapshots",
    "entities",
    "patch_events",
    "hero_stat_profiles",
    "learned_builds",
];

/// Echte FK-Spalten auf Surrogat-Parents: `child.<col>` wird via Parent-Map aufgeloest.
/// (Natur-Schluessel-FKs wie feed_key/video_id/hero_id->catalog werden 1:1 kopiert.)
fn remap_for(table: &str, col: &str) -> Option<&'static str> {
    let m: &[(&str, &str, &str)] = &[
        ("entity_snapshots", "source_document_id", "source_documents"),
        ("entities", "first_snapshot_id", "entity_snapshots"),
        ("entity_aliases", "entity_id", "entities"),
        ("entity_aliases", "snapshot_id", "entity_snapshots"),
        ("patch_events", "patch_snapshot_id", "entity_snapshots"),
        ("patch_event_enrichments", "patch_event_id", "patch_events"),
        ("entity_lineage", "patch_event_id", "patch_events"),
        ("legacy_entities", "first_patch_event_id", "patch_events"),
        ("legacy_entities", "last_patch_event_id", "patch_events"),
        ("forum_claims", "post_snapshot_id", "entity_snapshots"),
        ("hero_stat_profiles", "snapshot_id", "entity_snapshots"),
        ("hero_stat_profiles", "entity_id", "entities"),
        ("hero_stat_values", "profile_id", "hero_stat_profiles"),
        ("hero_stat_values", "entity_id", "entities"),
        ("sheet_boons_ap", "snapshot_id", "entity_snapshots"),
        ("sheet_hero_rankings", "snapshot_id", "entity_snapshots"),
        ("sheet_hero_rankings", "entity_id", "entities"),
        ("sheet_heroes_stats", "snapshot_id", "entity_snapshots"),
        ("sheet_heroes_stats", "entity_id", "entities"),
        ("sheet_items", "snapshot_id", "entity_snapshots"),
        ("sheet_raw_heroes", "snapshot_id", "entity_snapshots"),
        ("sheet_raw_heroes", "entity_id", "entities"),
        ("sheet_shop_bonuses", "snapshot_id", "entity_snapshots"),
        ("sheet_tab_rows", "snapshot_id", "entity_snapshots"),
        ("build_learning_notes", "learned_build_id", "learned_builds"),
        ("patch_impact_notes", "entity_id", "entities"),
        ("youtube_transcripts", "source_document_id", "source_documents"),
    ];
    m.iter()
        .find(|(t, c, _)| *t == table && *c == col)
        .map(|(_, _, p)| *p)
}

/// Ziel-Spalte -> SQLite-Quellspalte fuer die wenigen Umbenennungen ausserhalb
/// des generischen `_json`-Suffix-Drops.
fn special_src(col: &str) -> Option<&'static str> {
    match col {
        "row_data" => Some("row_json"),
        "sqlite_rowid" => Some("rowid"),
        _ => None,
    }
}

#[derive(Parser)]
#[command(
    name = "dbrain-pg-migrate",
    about = "Laedt die 36 Brain-SQLite-Basistabellen FK-sicher nach brain.* in Postgres."
)]
struct Cli {
    /// Pfad zur Brain-SQLite (wird read-only geoeffnet).
    #[arg(long, default_value = "data/deadlock_brain.sqlite3")]
    sqlite: String,
    /// Name der Env-Variable mit dem Postgres-DSN.
    #[arg(long, default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    /// Nur SQLite-Zeilen zaehlen, kein PG-Zugriff/-Write.
    #[arg(long)]
    dry_run: bool,
    /// Zielschema in Postgres.
    #[arg(long, default_value = "brain")]
    schema: String,
    /// Ziel-Batchgroesse (Zeilen pro INSERT).
    #[arg(long, default_value_t = 1000)]
    batch_size: usize,
}

/// Was aus welcher SQLite-Spalte wie in die Ziel-Spalte konvertiert wird.
enum Extractor {
    /// Rohkopie einer INTEGER-Spalte -> bigint (legacy_sqlite_id, legacy_<fk>_id, plain int).
    CopyInt(String),
    /// REAL/INTEGER -> double precision.
    CopyFloat(String),
    /// TEXT -> text.
    CopyText(String),
    /// TEXT-JSON -> jsonb. `nullable`+`default_kind` fuer NULL-Quelle bei NOT-NULL-Ziel.
    Json {
        src: String,
        nullable: bool,
        default: JsonDefault,
    },
    /// INTEGER-Unix-Sekunden -> timestamptz.
    IntTs(String),
    /// TEXT-ISO (auch `-0700`-Offset, date-only, unix-sec-als-Text) -> timestamptz.
    IsoTs(String),
    /// FK-Aufloesung ueber Parent-Map.
    Remap {
        src: String,
        parent: &'static str,
        nullable: bool,
    },
}

#[derive(Clone, Copy)]
enum JsonDefault {
    Object,
    Array,
    None,
}

struct ColPlan {
    target: String,
    ext: Extractor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let sconn = rusqlite::Connection::open_with_flags(
        &cli.sqlite,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )
    .with_context(|| format!("SQLite read-only oeffnen: {}", cli.sqlite))?;

    // Quell-Zeilenzahlen (immer, auch dry-run).
    let mut src_counts: HashMap<String, i64> = HashMap::new();
    for &t in LOAD_ORDER {
        let c: i64 = sconn
            .query_row(&format!("SELECT COUNT(*) FROM \"{t}\""), [], |r| r.get(0))
            .with_context(|| format!("count {t}"))?;
        src_counts.insert(t.to_string(), c);
    }
    let total: i64 = src_counts.values().sum();
    println!("SQLite: {} Basistabellen, {} Zeilen gesamt", LOAD_ORDER.len(), total);

    if cli.dry_run {
        println!("\n-- DRY RUN (kein PG-Zugriff) --");
        for &t in LOAD_ORDER {
            println!("  {:>34}  rows={}", t, src_counts[t]);
        }
        return Ok(());
    }

    let dsn = std::env::var(&cli.dsn_env)
        .with_context(|| format!("DSN-Env-Variable {} nicht gesetzt", cli.dsn_env))?;
    let mut client = Client::connect(&dsn, NoTls).context("Postgres-Verbindung")?;
    // DSN wird bewusst nie geloggt.
    println!("Postgres verbunden (Schema {}).", cli.schema);

    let mut txn = client.transaction().context("BEGIN")?;

    // DELETE der 36 Ziel-Tabellen in umgekehrter FK-Reihenfolge (die 3 PG-only
    // Tabellen bleiben unangetastet; kein CASCADE).
    for &t in LOAD_ORDER.iter().rev() {
        txn.execute(&format!("DELETE FROM {}.\"{}\"", cli.schema, t), &[])
            .with_context(|| format!("DELETE {t}"))?;
    }

    // Parent-Maps: parent -> (legacy_sqlite_id -> neue id).
    let mut maps: HashMap<&'static str, HashMap<i64, i64>> = HashMap::new();
    for &p in PARENT_TABLES {
        maps.insert(p, HashMap::new());
    }

    let mut inserted: HashMap<String, i64> = HashMap::new();
    for &table in LOAD_ORDER {
        let n = load_table(&mut txn, &sconn, &cli.schema, table, &mut maps, cli.batch_size)
            .with_context(|| format!("Laden {table}"))?;
        inserted.insert(table.to_string(), n);
        println!("  [ok] {:>34}  {} Zeilen", table, n);
    }

    // Sequenzen der Surrogat-Tabellen an MAX(id) angleichen.
    for &table in LOAD_ORDER {
        let has_id: bool = txn
            .query_one(
                "SELECT EXISTS (SELECT 1 FROM information_schema.columns \
                 WHERE table_schema=$1 AND table_name=$2 AND column_name='id')",
                &[&cli.schema, &table],
            )?
            .get(0);
        if has_id {
            txn.execute(
                &format!(
                    "SELECT setval(pg_get_serial_sequence('{s}.{t}','id'), \
                     COALESCE((SELECT MAX(id) FROM {s}.\"{t}\"),1), \
                     (SELECT COUNT(*)>0 FROM {s}.\"{t}\"))",
                    s = cli.schema,
                    t = table
                ),
                &[],
            )
            .ok();
        }
    }

    txn.commit().context("COMMIT")?;
    println!("\nCOMMIT ok.");

    // In-Prozess-Paritaetscheck (Counts).
    println!("\n-- Paritaet (SQLite == PG) --");
    let mut all_ok = true;
    for &t in LOAD_ORDER {
        let pg: i64 = client
            .query_one(&format!("SELECT COUNT(*) FROM {}.\"{}\"", cli.schema, t), &[])?
            .get(0);
        let ok = pg == src_counts[t];
        if !ok {
            all_ok = false;
        }
        println!(
            "  {} {:>34}  sqlite={} pg={}",
            if ok { "OK " } else { "XX " },
            t,
            src_counts[t],
            pg
        );
    }
    if all_ok {
        println!("\nPARITAET OK: alle 36 Tabellen count-gleich.");
        Ok(())
    } else {
        bail!("PARITAET FEHLGESCHLAGEN: mindestens eine Tabelle weicht ab.");
    }
}

/// SQLite-Deklarationstypen einer Tabelle (Spaltenname -> decltype uppercase).
fn sqlite_decltypes(conn: &rusqlite::Connection, table: &str) -> Result<HashMap<String, String>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info(\"{table}\")"))?;
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, String>(1)?, r.get::<_, String>(2)?))
    })?;
    let mut out = HashMap::new();
    for r in rows {
        let (name, ty) = r?;
        out.insert(name, ty.to_uppercase());
    }
    Ok(out)
}

/// Ziel-Spalten aus information_schema (in ordinal-Reihenfolge).
struct TargetCol {
    name: String,
    data_type: String,
    nullable: bool,
    has_serial_default: bool,
    default_kind: JsonDefault,
}

fn target_cols(txn: &mut postgres::Transaction, schema: &str, table: &str) -> Result<Vec<TargetCol>> {
    let rows = txn.query(
        "SELECT column_name, data_type, is_nullable, column_default \
         FROM information_schema.columns \
         WHERE table_schema=$1 AND table_name=$2 ORDER BY ordinal_position",
        &[&schema, &table],
    )?;
    if rows.is_empty() {
        bail!("Ziel-Tabelle {schema}.{table} hat keine Spalten (Migration nicht eingespielt?)");
    }
    Ok(rows
        .iter()
        .map(|row| {
            let default: Option<String> = row.get(3);
            let d = default.unwrap_or_default();
            let default_kind = if d.contains("'[]'") {
                JsonDefault::Array
            } else if d.contains("'{}'") {
                JsonDefault::Object
            } else {
                JsonDefault::None
            };
            TargetCol {
                name: row.get(0),
                data_type: row.get(1),
                nullable: row.get::<_, String>(2) == "YES",
                has_serial_default: d.contains("nextval"),
                default_kind,
            }
        })
        .collect())
}

/// Baut den Spaltenplan (welche Ziel-Spalte aus welcher Quelle/Konvertierung).
fn build_plan(
    schema_table: &str,
    tcols: &[TargetCol],
    src_types: &HashMap<String, String>,
) -> Result<Vec<ColPlan>> {
    let src_has = |c: &str| src_types.contains_key(c);
    let mut plan = Vec::new();
    for tc in tcols {
        let name = tc.name.as_str();
        // id via Sequenz -> auslassen.
        if tc.has_serial_default {
            continue;
        }
        // synthetische imported_at-Provenienzspalte (DEFAULT now(), keine Quelle) -> auslassen.
        if name == "imported_at" && !src_has("imported_at") {
            continue;
        }
        let ext = if name == "legacy_sqlite_id" {
            require_src(schema_table, name, "id", &src_has)?;
            Extractor::CopyInt("id".into())
        } else if let Some(parent) = remap_for(schema_table, name) {
            require_src(schema_table, name, name, &src_has)?;
            Extractor::Remap {
                src: name.into(),
                parent,
                nullable: tc.nullable,
            }
        } else if let Some(base) = name
            .strip_prefix("legacy_")
            .filter(|b| remap_for(schema_table, b).is_some())
        {
            require_src(schema_table, name, base, &src_has)?;
            Extractor::CopyInt(base.into())
        } else if tc.data_type == "jsonb" {
            let src = special_src(name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{name}_json"));
            let src = if src_has(&src) {
                src
            } else if src_has(name) {
                name.to_string()
            } else {
                bail!("{schema_table}.{name} (jsonb): keine Quellspalte {src}/{name}");
            };
            Extractor::Json {
                src,
                nullable: tc.nullable,
                default: tc.default_kind,
            }
        } else if let Some(s) = special_src(name) {
            require_src(schema_table, name, s, &src_has)?;
            Extractor::CopyInt(s.into())
        } else if tc.data_type == "timestamp with time zone" {
            require_src(schema_table, name, name, &src_has)?;
            let st = src_types.get(name).map(|s| s.as_str()).unwrap_or("");
            if st.contains("INT") {
                Extractor::IntTs(name.into())
            } else {
                Extractor::IsoTs(name.into())
            }
        } else {
            require_src(schema_table, name, name, &src_has)?;
            match tc.data_type.as_str() {
                "bigint" => Extractor::CopyInt(name.into()),
                "double precision" => Extractor::CopyFloat(name.into()),
                "text" => Extractor::CopyText(name.into()),
                other => bail!("{schema_table}.{name}: unerwarteter Zieltyp {other}"),
            }
        };
        plan.push(ColPlan {
            target: name.to_string(),
            ext,
        });
    }
    Ok(plan)
}

fn require_src(table: &str, tcol: &str, scol: &str, src_has: &impl Fn(&str) -> bool) -> Result<()> {
    if src_has(scol) {
        Ok(())
    } else {
        Err(anyhow!("{table}.{tcol}: Quellspalte {scol} fehlt in SQLite"))
    }
}

#[allow(clippy::too_many_arguments)]
fn load_table(
    txn: &mut postgres::Transaction,
    sconn: &rusqlite::Connection,
    schema: &str,
    table: &str,
    maps: &mut HashMap<&'static str, HashMap<i64, i64>>,
    batch_size: usize,
) -> Result<i64> {
    let src_types = sqlite_decltypes(sconn, table)?;
    let tcols = target_cols(txn, schema, table)?;
    let plan = build_plan(table, &tcols, &src_types)?;
    let is_parent = PARENT_TABLES.contains(&table);

    let ncols = plan.len();
    if ncols == 0 {
        bail!("{table}: leerer Spaltenplan");
    }
    // Param-Limit (65535) respektieren.
    let max_rows = (60000 / ncols).max(1);
    let batch_rows = batch_size.min(max_rows).max(1);

    let col_list = plan
        .iter()
        .map(|c| format!("\"{}\"", c.target))
        .collect::<Vec<_>>()
        .join(", ");

    // SQLite lesen: alle Spalten, Index nach Name.
    let mut stmt = sconn.prepare(&format!("SELECT * FROM \"{table}\""))?;
    let sncol = stmt.column_count();
    let sidx: HashMap<String, usize> = (0..sncol)
        .map(|i| (stmt.column_name(i).unwrap().to_string(), i))
        .collect();

    let mut rows = stmt.query([])?;
    let mut batch: Vec<Vec<Box<dyn ToSql + Sync>>> = Vec::with_capacity(batch_rows);
    let mut inserted: i64 = 0;

    while let Some(row) = rows.next()? {
        let svals: Vec<SqliteValue> = (0..sncol)
            .map(|i| row.get::<_, SqliteValue>(i))
            .collect::<rusqlite::Result<_>>()?;
        let get = |col: &str| -> &SqliteValue {
            &svals[*sidx.get(col).expect("plan referenziert vorhandene Quellspalte")]
        };

        let mut prow: Vec<Box<dyn ToSql + Sync>> = Vec::with_capacity(ncols);
        for c in &plan {
            prow.push(convert(table, &c.target, &c.ext, &get, maps)?);
        }
        batch.push(prow);

        if batch.len() >= batch_rows {
            inserted += flush(txn, schema, table, &col_list, &plan, &batch, is_parent, maps)?;
            batch.clear();
        }
    }
    if !batch.is_empty() {
        inserted += flush(txn, schema, table, &col_list, &plan, &batch, is_parent, maps)?;
    }
    Ok(inserted)
}

/// Ein SQLite-Feld gemaess Extractor in einen PG-Parameter (Box<dyn ToSql>) wandeln.
fn convert<'a>(
    table: &str,
    tcol: &str,
    ext: &Extractor,
    get: &impl Fn(&str) -> &'a SqliteValue,
    maps: &HashMap<&'static str, HashMap<i64, i64>>,
) -> Result<Box<dyn ToSql + Sync>> {
    Ok(match ext {
        Extractor::CopyInt(c) => match get(c) {
            SqliteValue::Integer(i) => Box::new(Some(*i)),
            SqliteValue::Null => Box::new(None::<i64>),
            SqliteValue::Real(f) => Box::new(Some(*f as i64)),
            SqliteValue::Text(t) => Box::new(t.parse::<i64>().ok()),
            SqliteValue::Blob(_) => bail!("{table}.{tcol}: Blob in bigint-Spalte"),
        },
        Extractor::CopyFloat(c) => match get(c) {
            SqliteValue::Real(f) => Box::new(Some(*f)),
            SqliteValue::Integer(i) => Box::new(Some(*i as f64)),
            SqliteValue::Null => Box::new(None::<f64>),
            SqliteValue::Text(t) => Box::new(t.parse::<f64>().ok()),
            SqliteValue::Blob(_) => bail!("{table}.{tcol}: Blob in double-Spalte"),
        },
        Extractor::CopyText(c) => match get(c) {
            SqliteValue::Text(s) => Box::new(Some(s.clone())),
            SqliteValue::Null => Box::new(None::<String>),
            SqliteValue::Integer(i) => Box::new(Some(i.to_string())),
            SqliteValue::Real(f) => Box::new(Some(f.to_string())),
            SqliteValue::Blob(_) => bail!("{table}.{tcol}: Blob in text-Spalte"),
        },
        Extractor::Json {
            src,
            nullable,
            default,
        } => match get(src) {
            SqliteValue::Text(s) => {
                let v: serde_json::Value = serde_json::from_str(s)
                    .with_context(|| format!("{table}.{tcol}: ungueltiges JSON"))?;
                Box::new(Some(v))
            }
            SqliteValue::Null => {
                if *nullable {
                    Box::new(None::<serde_json::Value>)
                } else {
                    match default {
                        JsonDefault::Array => Box::new(Some(serde_json::json!([]))),
                        JsonDefault::Object => Box::new(Some(serde_json::json!({}))),
                        JsonDefault::None => {
                            bail!("{table}.{tcol}: NULL in NOT-NULL-jsonb ohne Default")
                        }
                    }
                }
            }
            _ => bail!("{table}.{tcol}: nicht-Text in jsonb-Spalte"),
        },
        Extractor::IntTs(c) => match get(c) {
            SqliteValue::Integer(i) => Box::new(unix_secs(*i)),
            SqliteValue::Real(f) => Box::new(unix_secs(*f as i64)),
            SqliteValue::Null => Box::new(None::<DateTime<Utc>>),
            _ => bail!("{table}.{tcol}: nicht-Integer in Unix-Sekunden-Zeitspalte"),
        },
        Extractor::IsoTs(c) => match get(c) {
            SqliteValue::Text(s) => {
                let ts = parse_iso_ts(s)
                    .ok_or_else(|| anyhow!("{table}.{tcol}: unparsbare Zeit {s:?}"))?;
                Box::new(Some(ts))
            }
            SqliteValue::Integer(i) => Box::new(unix_secs(*i)),
            SqliteValue::Null => Box::new(None::<DateTime<Utc>>),
            _ => bail!("{table}.{tcol}: unerwarteter Typ in ISO-Zeitspalte"),
        },
        Extractor::Remap {
            src,
            parent,
            nullable,
        } => match get(src) {
            SqliteValue::Integer(i) => {
                let m = maps.get(parent).expect("parent map existiert");
                match m.get(i) {
                    Some(newid) => Box::new(Some(*newid)),
                    None => bail!("{table}.{tcol}: FK-Orphan {i} in Parent {parent}"),
                }
            }
            SqliteValue::Null => {
                if *nullable {
                    Box::new(None::<i64>)
                } else {
                    bail!("{table}.{tcol}: NULL in NOT-NULL-FK")
                }
            }
            _ => bail!("{table}.{tcol}: nicht-Integer in FK-Spalte"),
        },
    })
}

fn unix_secs(i: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(i, 0).single()
}

/// Robuster, NULL-toleranter ISO/Unix-Zeitparser fuer die TEXT-Zeitspalten.
/// Deckt: unix-Sekunden-als-Text, RFC3339 (`+00:00`/`Z`), Offset ohne Doppelpunkt
/// (`-0700`), naive Datetimes, reine Datumsangaben.
fn parse_iso_ts(raw: &str) -> Option<DateTime<Utc>> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    // unix-Sekunden als Text ("1772833020")
    let digits = s.strip_prefix('-').unwrap_or(s);
    if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(n) = s.parse::<i64>() {
            return unix_secs(n);
        }
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    for fmt in [
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%dT%H:%M:%S%.f%z",
        "%Y-%m-%d %H:%M:%S%z",
    ] {
        if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
            return Some(dt.with_timezone(&Utc));
        }
    }
    for fmt in [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
    ] {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(Utc.from_utc_datetime(&ndt));
        }
    }
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()));
    }
    None
}

/// Multi-Row-INSERT eines Batches; fuer Parent-Tabellen mit RETURNING zum Map-Aufbau.
#[allow(clippy::too_many_arguments)]
fn flush(
    txn: &mut postgres::Transaction,
    schema: &str,
    table: &str,
    col_list: &str,
    plan: &[ColPlan],
    batch: &[Vec<Box<dyn ToSql + Sync>>],
    is_parent: bool,
    maps: &mut HashMap<&'static str, HashMap<i64, i64>>,
) -> Result<i64> {
    let ncols = plan.len();
    let mut placeholders = Vec::with_capacity(batch.len());
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(batch.len() * ncols);
    let mut p = 1;
    for prow in batch {
        let ph = (0..ncols)
            .map(|_| {
                let s = format!("${p}");
                p += 1;
                s
            })
            .collect::<Vec<_>>()
            .join(", ");
        placeholders.push(format!("({ph})"));
        for v in prow {
            params.push(&**v as &(dyn ToSql + Sync));
        }
    }

    if is_parent {
        // legacy_sqlite_id-Position im Plan finden.
        let legacy_pos = plan
            .iter()
            .position(|c| c.target == "legacy_sqlite_id")
            .ok_or_else(|| anyhow!("{table}: Parent ohne legacy_sqlite_id"))?;
        let sql = format!(
            "INSERT INTO {schema}.\"{table}\" ({col_list}) VALUES {} RETURNING id, legacy_sqlite_id",
            placeholders.join(", ")
        );
        let rows = txn.query(&sql, &params)?;
        let map = maps.get_mut(table).expect("parent map");
        for r in &rows {
            let newid: i64 = r.get(0);
            let legacy: i64 = r.get(1);
            map.insert(legacy, newid);
        }
        let _ = legacy_pos; // Position nur zur Absicherung geprueft.
        Ok(rows.len() as i64)
    } else {
        let sql = format!(
            "INSERT INTO {schema}.\"{table}\" ({col_list}) VALUES {}",
            placeholders.join(", ")
        );
        let n = txn.execute(&sql, &params)?;
        Ok(n as i64)
    }
}
