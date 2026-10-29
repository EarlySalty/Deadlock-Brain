use crate::{
    claims::{self, Value},
    probe_guard,
    questions::short,
};
use chrono::{DateTime, NaiveDate, Utc};
use regex::{Regex, RegexBuilder};
use sqlx::PgPool;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

const PATCH_SOURCE: &str = "brain.patch_changes";
const RAW_LINE_LIMIT: usize = 400;
const RAW_LINE_MAX: i32 = 4000;
const NEAR_DAYS: i64 = 2;
const RETENTION_DAYS: i32 = 90;
const ATTACH_CHARS: usize = 60;
const MIN_NAME_CHARS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    WrongPatchDate,
    StaleValue,
    StaleHistory,
    UnsourcedChange,
    UnsourcedNumber,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::WrongPatchDate => "wrong_patch_date",
            Kind::StaleValue => "stale_value",
            Kind::StaleHistory => "stale_history",
            Kind::UnsourcedChange => "unsourced_change",
            Kind::UnsourcedNumber => "unsourced_number",
        }
    }
}

#[derive(Debug)]
pub struct Flag {
    pub trace_id: i64,
    pub trace_at: DateTime<Utc>,
    pub kind: Kind,
    pub claim: String,
    pub expected: String,
    pub source: String,
}

pub struct Row {
    pub date: NaiveDate,
    pub title: String,
    pub entity_type: String,
    pub entity_name: String,
    pub raw: String,
    pub long: bool,
    pub url: String,
    pub old: Vec<Value>,
    pub new: Vec<Value>,
    pub values: Vec<Value>,
    pub tokens: BTreeSet<String>,
    pub stat_tokens: BTreeSet<String>,
}

impl Row {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        date: NaiveDate,
        title: &str,
        entity_type: &str,
        entity_name: &str,
        stat: &str,
        old: &str,
        new: &str,
        raw: &str,
        url: &str,
    ) -> Self {
        let raw_text = probe_guard::normalize(raw);
        Row {
            date,
            title: title.to_owned(),
            entity_type: entity_type.to_lowercase(),
            entity_name: entity_name.to_lowercase(),
            raw: raw.to_owned(),
            long: raw.chars().count() > RAW_LINE_LIMIT,
            url: url.to_owned(),
            old: claims::values(&probe_guard::normalize(old)),
            new: claims::values(&probe_guard::normalize(new)),
            values: claims::values(&claims::masked(&raw_text)),
            tokens: claims::tokens(&format!("{raw_text} {stat}")),
            stat_tokens: claims::tokens(stat),
        }
    }

    fn has(&self, value: &Value) -> bool {
        self.values
            .iter()
            .chain(&self.old)
            .chain(&self.new)
            .any(|known| known.scaled(value))
    }

    fn directed(&self, from: &Value, to: &Value) -> bool {
        self.old.iter().any(|old| old.scaled(from)) && self.new.iter().any(|new| new.scaled(to))
    }

    fn numeric_change(&self) -> bool {
        self.old.iter().any(Value::significant) && self.new.iter().any(Value::significant)
    }

    fn entity_scoped(&self) -> bool {
        ["hero", "item", "ability"]
            .iter()
            .any(|kind| self.entity_type.starts_with(kind))
            && !self.entity_name.is_empty()
    }

    fn describe(&self) -> String {
        format!("{} {}: {}", self.date, self.title, short(&self.raw, 300))
    }

    fn source(&self) -> String {
        let source = if self.url.is_empty() {
            format!("{PATCH_SOURCE} {}", self.title)
        } else {
            self.url.clone()
        };
        source.chars().take(300).collect()
    }
}

pub fn related(anchor: &Row, later: &Row) -> bool {
    if anchor.long
        || later.long
        || later.date <= anchor.date + chrono::Duration::days(NEAR_DAYS)
        || !later.numeric_change()
    {
        return false;
    }
    if !supersedes(anchor, later) {
        return false;
    }
    let shared = anchor.tokens.intersection(&later.tokens).count();
    let stats = anchor.stat_tokens.intersection(&later.stat_tokens).count();
    let both_stats = !anchor.stat_tokens.is_empty() && !later.stat_tokens.is_empty();
    if anchor.entity_scoped() || later.entity_scoped() {
        return later.entity_name == anchor.entity_name && (stats >= 1 || shared >= 3);
    }
    later.entity_type == anchor.entity_type
        && shared >= 2
        && (if both_stats { stats >= 1 } else { shared >= 3 })
}

fn supersedes(anchor: &Row, later: &Row) -> bool {
    anchor.new.iter().any(|now| {
        let current = now.parts();
        later.old.iter().any(|old| {
            let previous = old.parts();
            previous.len() == current.len()
                && previous
                    .iter()
                    .zip(&current)
                    .all(|(a, b)| claims::loose(*a, *b))
        })
    })
}

#[derive(Clone)]
struct Named {
    kind: String,
    ids: Vec<i64>,
    english: Vec<String>,
}

pub struct Catalog {
    pub rows: Vec<Row>,
    by_date: BTreeMap<NaiveDate, Vec<usize>>,
    by_entity: HashMap<String, Vec<usize>>,
    pub patch_days: BTreeMap<NaiveDate, BTreeSet<String>>,
    names: Option<Regex>,
    named: HashMap<String, Named>,
}

impl Catalog {
    pub fn new(rows: Vec<Row>, patch_days: BTreeMap<NaiveDate, BTreeSet<String>>) -> Self {
        let mut by_date: BTreeMap<NaiveDate, Vec<usize>> = BTreeMap::new();
        let mut by_entity: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, row) in rows.iter().enumerate() {
            by_date.entry(row.date).or_default().push(index);
            if row.entity_scoped() {
                by_entity
                    .entry(row.entity_name.clone())
                    .or_default()
                    .push(index);
            }
        }
        Catalog {
            rows,
            by_date,
            by_entity,
            patch_days,
            names: None,
            named: HashMap::new(),
        }
    }

    fn with_names(mut self, entries: Vec<(i64, String, String, String)>) -> anyhow::Result<Self> {
        let mut english: HashMap<i64, Vec<String>> = HashMap::new();
        for (id, _, key, name) in &entries {
            if key == "name_en" {
                english.entry(*id).or_default().push(name.to_lowercase());
            }
        }
        for (id, kind, _, name) in entries {
            let name = name.trim().to_owned();
            if name.chars().count() < MIN_NAME_CHARS || !name.starts_with(char::is_uppercase) {
                continue;
            }
            let entry = self.named.entry(name).or_insert_with(|| Named {
                kind: kind.clone(),
                ids: Vec::new(),
                english: Vec::new(),
            });
            if !entry.ids.contains(&id) {
                entry.ids.push(id);
                entry
                    .english
                    .extend(english.get(&id).cloned().unwrap_or_default());
            }
            if kind == "hero" {
                entry.kind = kind;
            }
        }
        let mut names: Vec<&String> = self.named.keys().collect();
        names.sort_by_key(|name| std::cmp::Reverse(name.len()));
        if !names.is_empty() {
            let pattern = names
                .iter()
                .map(|name| regex::escape(name))
                .collect::<Vec<_>>()
                .join("|");
            self.names = Some(
                RegexBuilder::new(&format!(r"\b(?:{pattern})\b"))
                    .size_limit(64 << 20)
                    .build()?,
            );
        }
        Ok(self)
    }

    fn near_patch(&self, date: NaiveDate) -> bool {
        let span = chrono::Duration::days(NEAR_DAYS);
        self.patch_days
            .range(date - span..=date + span)
            .next()
            .is_some()
    }

    fn nearest(&self, date: NaiveDate) -> String {
        let before = self.patch_days.range(..date).next_back();
        let after = self.patch_days.range(date..).next();
        [before, after]
            .into_iter()
            .flatten()
            .map(|(day, titles)| {
                let title = titles.iter().next().map_or("", String::as_str);
                format!("{day} {title}")
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }

    fn rows_near(&self, date: NaiveDate) -> impl Iterator<Item = &Row> {
        let span = chrono::Duration::days(NEAR_DAYS);
        self.by_date
            .range(date - span..=date + span)
            .flat_map(|(_, rows)| rows.iter().filter_map(|index| self.rows.get(*index)))
            .filter(|row| !row.long)
    }

    fn entity_rows<'a>(&'a self, english: &'a [String]) -> impl Iterator<Item = &'a Row> {
        english
            .iter()
            .filter_map(|name| self.by_entity.get(name))
            .flatten()
            .filter_map(|index| self.rows.get(*index))
    }

    fn latest_related(
        &self,
        anchor: &Row,
        answer: &BTreeSet<String>,
        values: &[Value],
    ) -> Option<(Rank, &Row)> {
        let indices: Vec<usize> = if anchor.entity_scoped() {
            self.by_entity
                .get(&anchor.entity_name)
                .cloned()
                .unwrap_or_default()
        } else {
            self.by_date
                .range(anchor.date..)
                .flat_map(|(_, rows)| rows.iter().copied())
                .collect()
        };
        let parts: Vec<f64> = values.iter().flat_map(Value::parts).collect();
        indices
            .into_iter()
            .filter_map(|index| self.rows.get(index))
            .filter(|later| related(anchor, later))
            .filter(|later| later.tokens.intersection(answer).count() >= 1)
            .map(|later| {
                let reported = later
                    .new
                    .iter()
                    .flat_map(Value::parts)
                    .any(|new| parts.iter().any(|part| claims::loose(*part, new)));
                let rank = (
                    reported,
                    later.tokens.intersection(answer).count(),
                    anchor.tokens.intersection(&later.tokens).count(),
                    later.date,
                );
                (rank, later)
            })
            .max_by_key(|(rank, _)| *rank)
    }

    fn mention_ends(&self, sentence: &str) -> Vec<usize> {
        self.names
            .as_ref()
            .map(|names| names.find_iter(sentence).map(|found| found.end()).collect())
            .unwrap_or_default()
    }

    fn mentioned(&self, sentence: &str) -> Vec<Named> {
        let Some(names) = &self.names else {
            return Vec::new();
        };
        let mut seen = HashSet::new();
        names
            .find_iter(sentence)
            .filter(|found| seen.insert(found.as_str().to_owned()))
            .filter_map(|found| self.named.get(found.as_str()).cloned())
            .collect()
    }
}

type Rank = (bool, usize, usize, NaiveDate);

pub struct Trace {
    pub id: i64,
    pub at: DateTime<Utc>,
    pub question: String,
    pub history: Vec<String>,
    pub answer: String,
}

pub trait Facts {
    fn numbers(&mut self, ids: &[i64]) -> Vec<f64>;
}

impl Facts for HashMap<i64, Vec<f64>> {
    fn numbers(&mut self, ids: &[i64]) -> Vec<f64> {
        ids.iter()
            .filter_map(|id| self.get(id))
            .flatten()
            .copied()
            .collect()
    }
}

struct Audit<'a> {
    catalog: &'a Catalog,
    trace: &'a Trace,
    flags: Vec<Flag>,
    claims: usize,
}

impl Audit<'_> {
    fn flag(&mut self, kind: Kind, claim: &str, expected: String, source: String) {
        let duplicate = self
            .flags
            .iter()
            .any(|flag| flag.kind == kind && flag.expected == expected);
        if !duplicate {
            self.flags.push(Flag {
                trace_id: self.trace.id,
                trace_at: self.trace.at,
                kind,
                claim: short(claim, 590),
                expected: expected.chars().take(600).collect(),
                source: source.chars().take(300).collect(),
            });
        }
    }
}

fn mentions_near(dates: &[NaiveDate], date: NaiveDate) -> bool {
    dates
        .iter()
        .any(|day| (*day - date).num_days().abs() <= NEAR_DAYS)
}

pub fn audit(catalog: &Catalog, facts: &mut dyn Facts, trace: &Trace) -> (usize, Vec<Flag>) {
    let text = probe_guard::normalize(&trace.answer);
    let context =
        probe_guard::normalize(&format!("{} {}", trace.question, trace.history.join(" ")));
    let given = claims::values(&claims::masked(&context));
    let answer_dates: Vec<NaiveDate> = claims::dates(&text)
        .iter()
        .map(|dated| dated.date)
        .collect();
    let answer_values = claims::values(&claims::masked(&text));
    let answer_tokens = claims::tokens(&text);
    let mut audit = Audit {
        catalog,
        trace,
        flags: Vec::new(),
        claims: 0,
    };
    let mut anchors: Vec<(&Row, String)> = Vec::new();
    for sentence in claims::sentences(&text) {
        let dates: Vec<NaiveDate> = claims::dates(sentence)
            .iter()
            .map(|dated| dated.date)
            .collect();
        let patch = claims::patch_context(sentence);
        if patch {
            for date in &dates {
                audit.claims += 1;
                if !catalog.near_patch(*date) {
                    let expected =
                        format!("Kein Patch am {date}; nächste: {}", catalog.nearest(*date));
                    audit.flag(
                        Kind::WrongPatchDate,
                        sentence,
                        expected,
                        "brain.patch_changes, ops.patch_listing".into(),
                    );
                }
            }
        }
        let transitions = claims::transitions(sentence);
        let sentence_tokens = claims::tokens(sentence);
        for (from, to) in &transitions {
            audit.claims += 1;
            let named: Vec<String> = catalog
                .mentioned(sentence)
                .into_iter()
                .flat_map(|entity| entity.english)
                .collect();
            let sourced = catalog.rows.iter().any(|row| row.has(from) && row.has(to));
            let matches: Vec<&Row> = catalog
                .rows
                .iter()
                .filter(|row| !row.long && row.has(from) && row.has(to))
                .filter(|row| {
                    (row.entity_scoped() && named.contains(&row.entity_name))
                        || row.tokens.intersection(&sentence_tokens).count() >= 2
                })
                .collect();
            if matches.is_empty() {
                if patch && !sourced {
                    let expected = format!("Keine Patchzeile mit {} → {}", from.text, to.text);
                    audit.flag(
                        Kind::UnsourcedChange,
                        sentence,
                        expected,
                        PATCH_SOURCE.into(),
                    );
                }
                continue;
            }
            let directed: Vec<&&Row> = matches
                .iter()
                .filter(|row| row.directed(from, to))
                .collect();
            let cited = dates.iter().any(|date| catalog.near_patch(*date));
            let dated = matches.iter().any(|row| mentions_near(&dates, row.date));
            if cited && !dated && !directed.is_empty() {
                let expected = directed
                    .iter()
                    .take(3)
                    .map(|row| row.describe())
                    .collect::<Vec<_>>()
                    .join(" | ");
                let source = directed.first().map(|row| row.source()).unwrap_or_default();
                audit.flag(Kind::WrongPatchDate, sentence, expected, source);
            }
            anchors.extend(matches.into_iter().map(|row| (row, sentence.to_owned())));
        }
        entity_numbers(&mut audit, facts, sentence, &given, &transitions);
    }
    for date in &answer_dates {
        for row in catalog.rows_near(*date) {
            let valued = row
                .old
                .iter()
                .chain(&row.new)
                .filter(|value| value.significant())
                .any(|value| answer_values.iter().any(|known| known.same(value)));
            if valued && row.tokens.intersection(&answer_tokens).count() >= 2 {
                anchors.push((row, text.clone()));
            }
        }
    }
    let mut checked = HashSet::new();
    let mut stale: Option<(Rank, &Row, &Row, String)> = None;
    for (anchor, sentence) in anchors {
        if !checked.insert((anchor.date, anchor.raw.clone())) {
            continue;
        }
        audit.claims += 1;
        let Some((rank, later)) = catalog.latest_related(anchor, &answer_tokens, &answer_values)
        else {
            continue;
        };
        let covered = answer_dates
            .iter()
            .any(|day| *day >= later.date - chrono::Duration::days(NEAR_DAYS));
        if !covered && stale.as_ref().is_none_or(|(best, ..)| rank > *best) {
            stale = Some((rank, anchor, later, sentence));
        }
    }
    if let Some((_, anchor, later, sentence)) = stale {
        let claim = format!("{sentence} [Stand {} {}]", anchor.date, anchor.title);
        audit.flag(Kind::StaleHistory, &claim, later.describe(), later.source());
    }
    (audit.claims, audit.flags)
}

fn entity_numbers(
    audit: &mut Audit<'_>,
    facts: &mut dyn Facts,
    sentence: &str,
    given: &[Value],
    transitions: &[(Value, Value)],
) {
    let entities = audit.catalog.mentioned(sentence);
    if entities.is_empty() {
        return;
    }
    let ids: Vec<i64> = entities
        .iter()
        .flat_map(|named| named.ids.clone())
        .collect();
    let english: Vec<String> = entities
        .iter()
        .flat_map(|named| named.english.clone())
        .collect();
    let known = facts.numbers(&ids);
    let rows: Vec<&Row> = audit.catalog.entity_rows(&english).collect();
    let past = claims::past(sentence);
    let heroes_only_items = entities.iter().all(|named| named.kind != "hero");
    let dated = !claims::dates(sentence).is_empty();
    let units = claims::unit_values(sentence);
    let attached: HashSet<usize> = audit
        .catalog
        .mention_ends(sentence)
        .into_iter()
        .filter_map(|end| {
            units
                .iter()
                .map(|(_, _, start)| *start)
                .filter(|start| *start >= end && *start - end <= ATTACH_CHARS)
                .min()
        })
        .collect();
    for (value, _, start) in units.iter().cloned() {
        let Some(number) = value.number else {
            continue;
        };
        let premise = given.iter().any(|known| known.same(&value));
        let changed = transitions
            .iter()
            .any(|(from, to)| from.same(&value) || to.same(&value));
        if premise || changed || !value.significant() {
            continue;
        }
        audit.claims += 1;
        let factual = known.iter().any(|fact| {
            [1.0, 100.0, 0.01, 1000.0, 0.001]
                .iter()
                .any(|factor| claims::loose(number, fact * factor))
        });
        if factual {
            continue;
        }
        let latest = rows
            .iter()
            .filter(|row| row.has(&value))
            .max_by_key(|row| row.date);
        match latest {
            Some(row)
                if row.old.iter().any(|old| old.scaled(&value))
                    && !row.new.iter().any(|new| new.scaled(&value)) =>
            {
                if !past && !dated {
                    let now = row
                        .new
                        .iter()
                        .map(|new| new.text.clone())
                        .collect::<Vec<_>>()
                        .join("/");
                    let expected = format!("Aktuell {now} seit {}", row.describe());
                    audit.flag(Kind::StaleValue, sentence, expected, row.source());
                }
            }
            Some(_) => {}
            None if heroes_only_items && !past && attached.contains(&start) => {
                let names = entities
                    .iter()
                    .flat_map(|named| named.english.first().cloned())
                    .collect::<Vec<_>>()
                    .join(", ");
                let expected = format!(
                    "{} fehlt für {names} in canon.fact und {PATCH_SOURCE}",
                    value.text
                );
                audit.flag(
                    Kind::UnsourcedNumber,
                    sentence,
                    expected,
                    "canon.fact".into(),
                );
            }
            None => {}
        }
    }
}

pub struct Options {
    pub since: DateTime<Utc>,
    pub trace: Option<i64>,
    pub record: bool,
    pub limit: i64,
}

type TraceRow = (
    i64,
    DateTime<Utc>,
    String,
    Option<serde_json::Value>,
    String,
);

async fn traces(pool: &PgPool, options: &Options) -> anyhow::Result<Vec<Trace>> {
    let rows: Vec<TraceRow> = sqlx::query_as(
        "SELECT id,recorded_at,coalesce(question,''),history,answer_text FROM ops.answer_trace
         WHERE status='answered' AND answer_text IS NOT NULL AND btrim(answer_text)<>''
           AND ($1::bigint IS NULL OR id=$1) AND ($1::bigint IS NOT NULL OR recorded_at>=$2)
         ORDER BY id LIMIT $3",
    )
    .bind(options.trace)
    .bind(options.since)
    .bind(options.limit)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, at, question, history, answer)| Trace {
            id,
            at,
            question,
            history: history
                .and_then(|value| serde_json::from_value::<Vec<String>>(value).ok())
                .unwrap_or_default(),
            answer,
        })
        .collect())
}

type PatchRow = (
    NaiveDate,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

async fn catalog(pool: &PgPool) -> anyhow::Result<Catalog> {
    let raw: Vec<PatchRow> = sqlx::query_as(
        "SELECT DISTINCT ON (patch_date,raw_line) patch_date,coalesce(patch_title,''),coalesce(entity_type,''),
                coalesce(entity_name,''),coalesce(stat_name,''),coalesce(old_value,''),coalesce(new_value,''),
                raw_line,coalesce(patch_url,'')
         FROM brain.patch_changes
         WHERE patch_date IS NOT NULL AND raw_line IS NOT NULL AND char_length(raw_line)<=$1
         ORDER BY patch_date,raw_line,patch_title",
    )
    .bind(RAW_LINE_MAX)
    .fetch_all(pool)
    .await?;
    let rows = raw
        .iter()
        .map(|(date, title, kind, name, stat, old, new, line, url)| {
            Row::new(*date, title, kind, name, stat, old, new, line, url)
        })
        .collect();
    let listed: Vec<(Option<NaiveDate>, String)> = sqlx::query_as(
        "SELECT DISTINCT patch_date,coalesce(patch_title,'') FROM brain.patch_changes WHERE patch_date IS NOT NULL
         UNION SELECT (published_at AT TIME ZONE 'UTC')::date,coalesce(label,'') FROM ops.patch_listing",
    )
    .fetch_all(pool)
    .await?;
    let mut days: BTreeMap<NaiveDate, BTreeSet<String>> = BTreeMap::new();
    for (date, title) in listed {
        for day in [date, claims::title_date(&title)].into_iter().flatten() {
            days.entry(day).or_default().insert(title.clone());
        }
    }
    let names: Vec<(i64, String, String, String)> = sqlx::query_as(
        "SELECT f.entity_id,e.kind::text,f.stat_key::text,f.value_text::text
         FROM canon.fact f JOIN canon.entity e ON e.id=f.entity_id
         WHERE f.valid_to_run IS NULL AND f.stat_key IN ('name_en','name_de')
           AND e.kind IN ('hero','item','ability') AND f.value_text IS NOT NULL",
    )
    .fetch_all(pool)
    .await?;
    Catalog::new(rows, days).with_names(names)
}

struct DbFacts<'a> {
    pool: &'a PgPool,
    cache: HashMap<i64, Vec<f64>>,
    pending: Vec<i64>,
}

impl DbFacts<'_> {
    async fn prefetch(&mut self) -> anyhow::Result<()> {
        let missing: Vec<i64> = std::mem::take(&mut self.pending)
            .into_iter()
            .filter(|id| !self.cache.contains_key(id))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if missing.is_empty() {
            return Ok(());
        }
        let rows: Vec<(i64, Option<f64>, Option<String>)> = sqlx::query_as(
            "SELECT entity_id,value_num,value_text::text FROM canon.fact
             WHERE entity_id=ANY($1) AND valid_to_run IS NULL AND (value_num IS NOT NULL OR value_text::text ~ '\\d')",
        )
        .bind(&missing)
        .fetch_all(self.pool)
        .await?;
        for id in &missing {
            self.cache.entry(*id).or_default();
        }
        for (id, number, text) in rows {
            let entry = self.cache.entry(id).or_default();
            entry.extend(number);
            if let Some(text) = text {
                entry.extend(
                    claims::values(&probe_guard::normalize(&text))
                        .into_iter()
                        .filter_map(|value| value.number),
                );
            }
        }
        Ok(())
    }
}

fn needed(catalog: &Catalog, trace: &Trace) -> Vec<i64> {
    claims::sentences(&probe_guard::normalize(&trace.answer))
        .into_iter()
        .flat_map(|sentence| catalog.mentioned(sentence))
        .flat_map(|named| named.ids)
        .collect()
}

fn probe_questions() -> HashSet<String> {
    probe_guard::probes()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|probe| probe.turns)
        .map(|turn| turn.trim().to_owned())
        .collect()
}

pub struct Scan {
    pub traces: usize,
    pub skipped: usize,
    pub claims: usize,
    pub flagged: usize,
    pub flags: Vec<Flag>,
}

pub async fn scan(pool: &PgPool, options: &Options) -> anyhow::Result<Scan> {
    let catalog = catalog(pool).await?;
    let traces = traces(pool, options).await?;
    let probes = probe_questions();
    let mut facts = DbFacts {
        pool,
        cache: HashMap::new(),
        pending: Vec::new(),
    };
    let mut scan = Scan {
        traces: 0,
        skipped: 0,
        claims: 0,
        flagged: 0,
        flags: Vec::new(),
    };
    for trace in &traces {
        if options.trace.is_none() && probes.contains(trace.question.trim()) {
            scan.skipped += 1;
            continue;
        }
        facts.pending = needed(&catalog, trace);
        facts.prefetch().await?;
        let (claims, found) = audit(&catalog, &mut facts.cache, trace);
        scan.traces += 1;
        scan.claims += claims;
        scan.flagged += usize::from(!found.is_empty());
        scan.flags.extend(found);
    }
    Ok(scan)
}

pub async fn run(pool: &PgPool, options: &Options) -> anyhow::Result<()> {
    let Scan {
        traces: audited,
        skipped,
        claims: claims_total,
        flagged,
        flags,
    } = scan(pool, options).await?;
    let day = Utc::now().date_naive();
    println!(
        "answer_audit day={day} since={} traces={audited} skipped_probe_traces={skipped} claims={claims_total} flagged_traces={flagged} flags={}",
        options.since.format("%Y-%m-%dT%H:%MZ"),
        flags.len()
    );
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for flag in &flags {
        *kinds.entry(flag.kind.as_str()).or_default() += 1;
    }
    let breakdown: Vec<String> = kinds
        .iter()
        .map(|(kind, count)| format!("{kind}={count}"))
        .collect();
    println!("flag_kinds {}", breakdown.join(" "));
    for flag in &flags {
        println!(
            "\ntrace={} at={} kind={}\n  claim:    {}\n  expected: {}\n  source:   {}",
            flag.trace_id,
            flag.trace_at.format("%Y-%m-%d %H:%M"),
            flag.kind.as_str(),
            short(&flag.claim, 600),
            short(&flag.expected, 600),
            short(&flag.source, 300)
        );
    }
    if options.record {
        record(pool, options, day, audited, claims_total, flagged, &flags).await?;
    }
    history(pool).await
}

async fn record(
    pool: &PgPool,
    options: &Options,
    day: NaiveDate,
    traces: usize,
    claims: usize,
    flagged: usize,
    flags: &[Flag],
) -> anyhow::Result<()> {
    let count = |value: usize| i32::try_from(value).unwrap_or(i32::MAX);
    let mut transaction = pool.begin().await?;
    let probe_run: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM ops.answer_probe_run WHERE started_at>=now()-interval '26 hours' AND status<>'running'
         ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(&mut *transaction)
    .await?;
    let run_id: i64 = sqlx::query_scalar(
        "INSERT INTO ops.answer_audit_run (audit_day,since,traces,claims,flagged_traces,flags,probe_run)
         VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id",
    )
    .bind(day)
    .bind(options.since)
    .bind(count(traces))
    .bind(count(claims))
    .bind(count(flagged))
    .bind(count(flags.len()))
    .bind(probe_run)
    .fetch_one(&mut *transaction)
    .await?;
    for (ordinal, flag) in flags.iter().enumerate() {
        sqlx::query(
            "INSERT INTO ops.answer_audit_flag (run_id,ordinal,trace_id,trace_at,kind,claim,expected,source)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(run_id)
        .bind(count(ordinal))
        .bind(flag.trace_id)
        .bind(flag.trace_at)
        .bind(flag.kind.as_str())
        .bind(&flag.claim)
        .bind(&flag.expected)
        .bind(&flag.source)
        .execute(&mut *transaction)
        .await?;
    }
    let purged = sqlx::query(
        "DELETE FROM ops.answer_audit_run WHERE ran_at<now()-make_interval(days=>$1)",
    )
    .bind(RETENTION_DAYS)
    .execute(&mut *transaction)
    .await?
    .rows_affected()
        + sqlx::query(
            "DELETE FROM ops.answer_probe_result WHERE recorded_at<now()-make_interval(days=>$1)",
        )
        .bind(RETENTION_DAYS)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
    transaction.commit().await?;
    println!(
        "\nrecorded audit_run={run_id} probe_run={} purged={purged}",
        probe_run.map_or_else(|| "-".to_owned(), |id| id.to_string())
    );
    Ok(())
}

type DailyRow = (NaiveDate, i32, i32, i32, i32, Option<i64>);
type ProbeRow = (
    i64,
    DateTime<Utc>,
    String,
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<String>,
);

async fn history(pool: &PgPool) -> anyhow::Result<()> {
    let days: Vec<DailyRow> = sqlx::query_as(
        "SELECT DISTINCT ON (audit_day) audit_day,traces,claims,flagged_traces,flags,probe_run
         FROM ops.answer_audit_run WHERE audit_day>=current_date-7 ORDER BY audit_day DESC,id DESC",
    )
    .fetch_all(pool)
    .await?;
    if !days.is_empty() {
        println!("\ndaily day traces claims flagged_traces flags probe_run");
    }
    for (day, traces, claims, flagged, flags, probe) in days {
        let probe = probe.map_or_else(|| "-".to_owned(), |id| id.to_string());
        println!("daily {day} {traces} {claims} {flagged} {flags} {probe}");
    }
    let probe: Option<ProbeRow> = sqlx::query_as(
        "SELECT id,started_at,status,passed,partial,failed,unavailable,regressions,release
         FROM ops.answer_probe_run WHERE status<>'running' ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    let Some((id, started, status, passed, partial, failed, unavailable, regressions, release)) =
        probe
    else {
        println!("probe_run=-");
        return Ok(());
    };
    let show = |value: Option<i32>| value.map_or_else(|| "-".to_owned(), |value| value.to_string());
    println!(
        "probe_run={id} at={} status={status} release={} pass={} partial={} fail={} unavailable={} regressions={}",
        started.format("%Y-%m-%d %H:%M"),
        release.as_deref().map_or("-", |release| &release[..release.len().min(12)]),
        show(passed),
        show(partial),
        show(failed),
        show(unavailable),
        show(regressions)
    );
    let regressed: Vec<(String, Option<String>, String, String)> = sqlx::query_as(
        "SELECT probe_id,baseline_grade,grade,final_status FROM ops.answer_probe_result
         WHERE run_id=$1 AND (regressed OR grade='unavailable') ORDER BY probe_id",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    for (probe, before, grade, status) in regressed {
        println!(
            "  probe {probe}: {} -> {grade} ({status})",
            before.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn catalog() -> Catalog {
        let rows = vec![
            Row::new(
                date("2024-11-21"),
                "11-21-2024 Update",
                "general",
                "",
                "Trooper soul",
                "60/40% in the kill/orb",
                "65/35%",
                "Trooper soul changed from 60/40% in the kill/orb to 65/35%",
                "",
            ),
            Row::new(
                date("2025-02-25"),
                "Map Rework Update",
                "general",
                "",
                "Soul Orb denies",
                "65/35%",
                "50/50%",
                "Soul Orb denies changed from 65/35% to 50/50%",
                "https://example.invalid/map-rework",
            ),
            Row::new(
                date("2026-06-30"),
                "Minor Update - 06-30-2026",
                "general",
                "",
                "Trooper bounty ratio in the deniable flying orb",
                "40",
                "50",
                "Trooper bounty ratio in the deniable flying orb changed from 40 to 50",
                "https://example.invalid/patch",
            ),
        ];
        let mut days = BTreeMap::new();
        for row in &rows {
            days.entry(row.date)
                .or_insert_with(BTreeSet::new)
                .insert(row.title.clone());
        }
        days.entry(date("2025-11-21"))
            .or_insert_with(BTreeSet::new)
            .insert("11-21-2025 Update".into());
        Catalog::new(rows, days)
    }

    fn trace(answer: &str) -> Trace {
        Trace {
            id: 1715,
            at: Utc::now(),
            question: "War das nicht mal 60/40?".into(),
            history: vec!["Wie viele Seelen gibt ein Trooper?".into()],
            answer: answer.into(),
        }
    }

    fn kinds(answer: &str) -> Vec<&'static str> {
        audit(&catalog(), &mut HashMap::new(), &trace(answer))
            .1
            .iter()
            .map(|flag| flag.kind.as_str())
            .collect()
    }

    #[test]
    fn historic_f06_answer_is_flagged_as_stale_history() {
        let stale = "Nein, 60/40 war nur bis zum Update vom 21.11.2024 so. Seitdem steht die Trooper-Aufteilung laut Patchnotes bei 65/35 (Kill/Orb).";
        assert_eq!(kinds(stale), ["stale_history"]);
        let current = "Am 21.11.2024 von 60/40 auf 65/35 (Trooper Kill/Orb), am 25.02.2025 Soul-Orb-Aufteilung von 65/35 auf 50/50, am 30.06.2026 Orb-Anteil von 40 auf 50 erhöht.";
        assert!(kinds(current).is_empty());
    }

    #[test]
    fn wrong_and_unsourced_patch_claims_are_flagged() {
        assert_eq!(
            kinds("Das Trooper-Update vom 03.03.2025 änderte die Orbs."),
            ["wrong_patch_date"]
        );
        assert_eq!(
            kinds("Mit dem Update vom 30.06.2026 wurde der Trooper-Orb von 30 auf 70 geändert."),
            ["unsourced_change"]
        );
        assert_eq!(
            kinds("Mit dem Update vom 21.11.2025 wurde der Trooper-Orb von 40 auf 50 geändert."),
            ["wrong_patch_date"]
        );
    }
}
