use super::{
    bullet, chunks, explicit_values, label, pieces, structural_heading, subject, Chunk,
    SUBJECT_CHARS, SUBJECT_WORDS,
};
use anyhow::{bail, Context};
use sqlx::{PgConnection, PgPool, Row as _};
use std::collections::{BTreeMap, BTreeSet};

const OBJECTIVES: &[(&str, &[&str])] = &[
    ("Urn", &["urn", "urns", "soul urn"]),
    ("Base Guardian", &["base guardian", "base guardians"]),
    ("Guardian", &["guardian", "guardians"]),
    ("Walker", &["walker", "walkers"]),
    ("Patron", &["patron", "patrons", "weakened patron"]),
    ("Shrine", &["shrine", "shrines"]),
    ("Mid Boss", &["mid boss", "midboss", "mid bosses"]),
    ("Rejuvenator", &["rejuvenator", "rejuvenators", "rejuv"]),
    (
        "Sinner's Sacrifice",
        &[
            "sinner's sacrifice",
            "sinners sacrifice",
            "sinners' sacrifice",
        ],
    ),
    ("Trooper", &["trooper", "troopers"]),
    (
        "Neutral Camp",
        &["neutral camp", "neutral camps", "neutrals"],
    ),
];
const OBJECTIVE_WORDS: usize = 4;
const CHANGE_WORDS: &[&str] = &[
    "buff", "nerf", "changed", "added", "removed", "rework", "bugfix", "rename", "fixed", "fix",
];
const UP: &[&str] = &["increased", "increase", "raised"];
const DOWN: &[&str] = &["reduced", "reduce", "decreased", "decrease", "lowered"];
const LOWER_BETTER: &[&str] = &[
    "cooldown", "cost", "delay", "windup", "reload", "recoil", "spread", "penalty", "taken",
    "cycle",
];
const CHANGE_VERBS: &[&str] = &[
    "changed",
    "adjusted",
    "updated",
    "now",
    "longer",
    "rescaled",
    "moved",
    "replaced",
    "increased",
    "reduced",
    "decreased",
    "raised",
    "lowered",
    "improved",
    "swapped",
    "reverted",
    "scales",
    "scaling",
];
const LEDGER_LOCK: &str = "SELECT pg_advisory_xact_lock(hashtext('ops.patch_ledger'))";

pub const IMPORT_COLUMNS: [&str; 18] = [
    "ledger_id",
    "line_no",
    "piece",
    "section",
    "entity_type",
    "entity_id",
    "entity_name",
    "ability_id",
    "ability_name",
    "alias_used",
    "change_type",
    "stat_name",
    "old_value",
    "new_value",
    "raw_line",
    "source",
    "correction_ref",
    "needs_review",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Hero,
    Ability,
    Item,
}

#[derive(Debug, Default)]
pub struct Catalog {
    kinds: BTreeMap<i64, Kind>,
    names: BTreeMap<i64, String>,
    aliases: BTreeMap<Vec<String>, BTreeSet<i64>>,
    abilities: BTreeMap<i64, Vec<(Vec<String>, i64)>>,
    owners: BTreeMap<i64, BTreeSet<i64>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub line_no: i32,
    pub piece: i32,
    pub section: String,
    pub entity_type: &'static str,
    pub entity_id: Option<i64>,
    pub entity_name: Option<String>,
    pub ability_id: Option<i64>,
    pub ability_name: Option<String>,
    pub alias_used: Option<String>,
    pub change_type: Option<&'static str>,
    pub stat_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub raw_line: String,
    pub needs_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Correction {
    pub ledger_id: i64,
    pub line_no: i32,
    pub piece: i32,
    pub section: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub entity_name: Option<String>,
    pub ability_id: Option<i64>,
    pub ability_name: Option<String>,
    pub alias_used: Option<String>,
    pub change_type: Option<String>,
    pub stat_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub raw_line: String,
    pub source: String,
    pub correction_ref: String,
    pub needs_review: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Refresh {
    pub rows: usize,
    pub written: u64,
    pub deleted: u64,
}

struct Token<'a> {
    text: &'a str,
    word: String,
}

fn tokens(text: &str) -> Vec<Token<'_>> {
    let mut output = Vec::new();
    let mut start = None;
    for (index, ch) in text.char_indices().chain([(text.len(), ' ')]) {
        let inside = ch.is_alphanumeric() || ch == '\'' || ch == '’';
        match (inside, start) {
            (true, None) => start = Some(index),
            (false, Some(begin)) => {
                let raw = &text[begin..index];
                output.push(Token {
                    text: raw,
                    word: raw.to_lowercase().replace('’', "'"),
                });
                start = None;
            }
            _ => {}
        }
    }
    output
}

fn words(text: &str) -> Vec<String> {
    tokens(text).into_iter().map(|token| token.word).collect()
}

fn span<'a>(text: &'a str, found: &[Token<'a>]) -> &'a str {
    match (found.first(), found.last()) {
        (Some(first), Some(last)) => {
            let begin = first.text.as_ptr() as usize - text.as_ptr() as usize;
            let end = last.text.as_ptr() as usize - text.as_ptr() as usize + last.text.len();
            &text[begin..end]
        }
        _ => "",
    }
}

fn differs(printed: &str, canonical: &str) -> Option<String> {
    (words(printed) != words(canonical)).then(|| printed.trim().to_owned())
}

impl Catalog {
    pub fn new(
        entities: &[(i64, String, Option<String>)],
        aliases: &[(i64, String)],
        relations: &[(i64, i64)],
    ) -> Self {
        let mut catalog = Catalog::default();
        let mut spelled: BTreeMap<i64, BTreeSet<Vec<String>>> = BTreeMap::new();
        for (id, kind, name) in entities {
            let kind = match kind.as_str() {
                "hero" => Kind::Hero,
                "ability" => Kind::Ability,
                "item" => Kind::Item,
                _ => continue,
            };
            catalog.kinds.insert(*id, kind);
            if let Some(name) = name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
                catalog.names.insert(*id, name.to_owned());
                spelled.entry(*id).or_default().insert(words(name));
            }
        }
        for (id, alias) in aliases {
            if catalog.kinds.contains_key(id) {
                spelled.entry(*id).or_default().insert(words(alias));
            }
        }
        for (id, keys) in &spelled {
            for key in keys.iter().filter(|key| !key.is_empty()) {
                catalog.aliases.entry(key.clone()).or_default().insert(*id);
            }
        }
        for (hero, ability) in relations {
            if catalog.kinds.get(hero) != Some(&Kind::Hero)
                || catalog.kinds.get(ability) != Some(&Kind::Ability)
            {
                continue;
            }
            catalog.owners.entry(*ability).or_default().insert(*hero);
            let owned = catalog.abilities.entry(*hero).or_default();
            for key in spelled.get(ability).into_iter().flatten() {
                if !key.is_empty() {
                    owned.push((key.clone(), *ability));
                }
            }
        }
        catalog
    }

    fn resolve(&self, text: &str) -> Option<(i64, Kind)> {
        let ids = self.aliases.get(&words(text))?;
        let id = *ids.first()?;
        (ids.len() == 1)
            .then(|| self.kinds.get(&id).map(|kind| (id, *kind)))
            .flatten()
    }

    fn name(&self, id: i64, fallback: &str) -> String {
        self.names
            .get(&id)
            .cloned()
            .unwrap_or_else(|| fallback.trim().to_owned())
    }

    fn owner(&self, ability: i64, context: Option<i64>) -> Option<i64> {
        let owners = self.owners.get(&ability)?;
        if let Some(hero) = context.filter(|hero| owners.contains(hero)) {
            return Some(hero);
        }
        let first = *owners.first()?;
        (owners.len() == 1).then_some(first)
    }

    fn ability_in<'a>(&self, hero: i64, text: &'a str) -> Option<(i64, &'a str)> {
        let found = tokens(text);
        let owned = self.abilities.get(&hero)?;
        let mut best: Vec<(usize, i64, &'a str)> = Vec::new();
        for start in 0..found.len() {
            for (key, ability) in owned {
                let end = start + key.len();
                if end <= found.len()
                    && found[start..end]
                        .iter()
                        .zip(key)
                        .all(|(token, word)| &token.word == word)
                {
                    best.push((key.len(), *ability, span(text, &found[start..end])));
                }
            }
        }
        let longest = best.iter().map(|(length, _, _)| *length).max()?;
        best.retain(|(length, _, _)| *length == longest);
        let ids: BTreeSet<i64> = best.iter().map(|(_, id, _)| *id).collect();
        (ids.len() == 1)
            .then(|| best.first().map(|(_, id, printed)| (*id, *printed)))
            .flatten()
    }
}

fn objective_at(text: &str, limit: usize, whole: bool) -> Option<(&'static str, &str)> {
    let found = tokens(text);
    for start in 0..found.len().min(limit) {
        let mut best: Option<(usize, &'static str)> = None;
        for (name, spellings) in OBJECTIVES {
            for spelling in *spellings {
                let key = words(spelling);
                let end = start + key.len();
                let matched = end <= found.len()
                    && (!whole || end == found.len())
                    && found[start..end]
                        .iter()
                        .zip(&key)
                        .all(|(token, word)| &token.word == word);
                if matched && best.is_none_or(|(length, _)| key.len() > length) {
                    best = Some((key.len(), name));
                }
            }
        }
        if let Some((length, name)) = best {
            return Some((name, span(text, &found[start..start + length])));
        }
    }
    None
}

fn change_type(statement: &str, entity_type: &str) -> Option<&'static str> {
    let words = words(statement);
    let has = |set: &[&str]| words.iter().any(|word| set.contains(&word.as_str()));
    let first = words.first().map(String::as_str);
    if has(&["renamed", "rename"]) {
        return Some("rename");
    }
    if first == Some("fixed") || has(&["fix", "fixes", "bug", "bugs", "exploit"]) {
        return Some("bugfix");
    }
    if has(&["reworked", "rework", "redesigned"]) {
        return Some("rework");
    }
    if has(&["removed"]) {
        return Some("removed");
    }
    if has(&["added"]) || first == Some("new") {
        return Some("added");
    }
    if words.windows(2).any(|pair| pair == ["changed", "from"]) {
        return Some("changed");
    }
    if matches!(entity_type, "hero" | "ability" | "item") {
        if let Some(position) = words.iter().position(|word| {
            word == "improved" || UP.contains(&word.as_str()) || DOWN.contains(&word.as_str())
        }) {
            if words[position] == "improved" {
                return Some("buff");
            }
            let up = UP.contains(&words[position].as_str());
            let inverse = words[..position]
                .iter()
                .any(|word| LOWER_BETTER.contains(&word.as_str()));
            return Some(if up != inverse { "buff" } else { "nerf" });
        }
    }
    has(CHANGE_VERBS).then_some("changed")
}

fn clean_value(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty() && !CHANGE_WORDS.contains(&value.to_lowercase().as_str()))
        .then(|| value.to_owned())
}

fn stat(
    statement: &str,
    ability: Option<&str>,
) -> (Option<String>, Option<String>, Option<String>) {
    let Some((old, new, name)) = explicit_values(statement) else {
        return (None, None, None);
    };
    let name = ability
        .and_then(|printed| {
            name.get(..printed.len())
                .filter(|head| head.eq_ignore_ascii_case(printed))
                .map(|_| &name[printed.len()..])
        })
        .unwrap_or(name);
    match (clean_value(old), clean_value(new)) {
        (Some(old), Some(new)) => (clean_value(name), Some(old), Some(new)),
        _ => (None, None, None),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scope {
    Hero,
    Item,
    Other,
}

fn scope(section: &str) -> Scope {
    let lower = section.to_ascii_lowercase();
    if lower.contains("hero") {
        Scope::Hero
    } else if lower.contains("item") {
        Scope::Item
    } else {
        Scope::Other
    }
}

struct Binding<'a> {
    entity_type: &'static str,
    entity_id: Option<i64>,
    entity_name: Option<String>,
    ability: Option<(i64, String, &'a str)>,
    alias_used: Option<String>,
    needs_review: bool,
}

impl<'a> Binding<'a> {
    fn general() -> Self {
        Binding {
            entity_type: "general",
            entity_id: None,
            entity_name: None,
            ability: None,
            alias_used: None,
            needs_review: false,
        }
    }

    fn objective(name: &'static str, printed: &str) -> Self {
        Binding {
            entity_type: "objective",
            entity_id: None,
            entity_name: Some(name.to_owned()),
            ability: None,
            alias_used: differs(printed, name),
            needs_review: false,
        }
    }

    fn unresolved(entity_type: &'static str, printed: &str) -> Self {
        Binding {
            entity_type,
            entity_id: None,
            entity_name: Some(printed.trim().to_owned()),
            ability: None,
            alias_used: None,
            needs_review: true,
        }
    }
}

struct Scene<'c> {
    catalog: &'c Catalog,
    heading: Option<&'c str>,
    scope: Scope,
}

impl<'c> Scene<'c> {
    fn hero<'a>(&self, hero: i64, printed: Option<&str>, statement: &'a str) -> Binding<'a> {
        let entity_name = self.catalog.name(hero, printed.unwrap_or_default());
        let hero_alias = printed.and_then(|printed| differs(printed, &entity_name));
        match self.catalog.ability_in(hero, statement) {
            Some((ability, spelled)) => {
                let ability_name = self.catalog.name(ability, spelled);
                Binding {
                    entity_type: "ability",
                    entity_id: Some(hero),
                    alias_used: differs(spelled, &ability_name).or(hero_alias),
                    entity_name: Some(entity_name),
                    ability: Some((ability, ability_name, spelled)),
                    needs_review: false,
                }
            }
            None => Binding {
                entity_type: "hero",
                entity_id: Some(hero),
                entity_name: Some(entity_name),
                ability: None,
                alias_used: hero_alias,
                needs_review: false,
            },
        }
    }

    fn entity<'a>(&self, id: i64, kind: Kind, printed: &str, statement: &'a str) -> Binding<'a> {
        let heading_hero = self.heading_hero();
        match kind {
            Kind::Hero => self.hero(id, Some(printed), statement),
            Kind::Item => {
                let entity_name = self.catalog.name(id, printed);
                Binding {
                    entity_type: "item",
                    entity_id: Some(id),
                    alias_used: differs(printed, &entity_name),
                    entity_name: Some(entity_name),
                    ability: None,
                    needs_review: false,
                }
            }
            Kind::Ability => {
                let owner = self.catalog.owner(id, heading_hero);
                let ability_name = self.catalog.name(id, printed);
                Binding {
                    entity_type: "ability",
                    entity_id: owner,
                    entity_name: owner.map(|hero| self.catalog.name(hero, "")),
                    alias_used: differs(printed, &ability_name),
                    ability: Some((id, ability_name, "")),
                    needs_review: owner.is_none(),
                }
            }
        }
    }

    fn heading_hero(&self) -> Option<i64> {
        self.heading
            .and_then(|heading| self.catalog.resolve(heading))
            .filter(|(_, kind)| *kind == Kind::Hero)
            .map(|(id, _)| id)
    }

    fn by_heading<'a>(&self, statement: &'a str, is_bullet: bool) -> Binding<'a> {
        if let Some(heading) = self.heading {
            if let Some((id, kind)) = self.catalog.resolve(heading) {
                return match kind {
                    Kind::Hero => self.hero(id, None, statement),
                    _ => {
                        let mut binding = self.entity(id, kind, heading, statement);
                        binding.alias_used = None;
                        binding
                    }
                };
            }
            if let Some((name, _)) = objective_at(heading, 1, true) {
                return Binding::objective(name, name);
            }
        }
        match objective_at(statement, OBJECTIVE_WORDS, false).filter(|_| is_bullet) {
            Some((name, printed)) => Binding::objective(name, printed),
            None => Binding::general(),
        }
    }

    fn bind<'a>(&self, part: &'a str) -> (Binding<'a>, &'a str) {
        let body = bullet(part).unwrap_or(part.trim());
        let Some((name, rest)) = subject(part) else {
            return (self.by_heading(body, bullet(part).is_some()), body);
        };
        if let Some((id, kind)) = self.catalog.resolve(name) {
            return (self.entity(id, kind, name, rest), rest);
        }
        if let Some((objective, printed)) = objective_at(name, 1, true) {
            return (Binding::objective(objective, printed), rest);
        }
        if let Some(hero) = self.heading_hero() {
            return (self.hero(hero, None, body), body);
        }
        let binding = match self.scope {
            Scope::Hero => Binding::unresolved("hero", name),
            Scope::Item => Binding::unresolved("item", name),
            Scope::Other => Binding::general(),
        };
        (binding, rest)
    }
}

fn row(part: &str, line_no: usize, piece: usize, section: String, context: &Scene) -> Row {
    let (binding, statement) = context.bind(part);
    let printed_ability = binding
        .ability
        .as_ref()
        .map(|(_, _, printed)| *printed)
        .filter(|printed| !printed.is_empty());
    let (stat_name, old_value, new_value) = stat(statement, printed_ability);
    Row {
        line_no: i32::try_from(line_no).unwrap_or(i32::MAX),
        piece: i32::try_from(piece).unwrap_or(i32::MAX),
        section,
        entity_type: binding.entity_type,
        entity_id: binding.entity_id,
        entity_name: binding.entity_name,
        ability_id: binding.ability.as_ref().map(|(id, _, _)| *id),
        ability_name: binding.ability.map(|(_, name, _)| name),
        alias_used: binding.alias_used,
        change_type: change_type(statement, binding.entity_type),
        stat_name,
        old_value,
        new_value,
        raw_line: part.trim().to_owned(),
        needs_review: binding.needs_review,
    }
}

fn sub_heading(part: &str) -> bool {
    let name = label(part);
    bullet(part).is_none()
        && structural_heading(part)
        && name.chars().count() <= SUBJECT_CHARS
        && name.split_whitespace().count() <= SUBJECT_WORDS
}

pub fn rows(content: &str, catalog: &Catalog) -> Vec<Row> {
    let mut output = Vec::new();
    let mut top = "General".to_owned();
    let mut heading: Option<String> = None;
    for (line_no, text) in content.lines().enumerate() {
        let parts = chunks(text);
        let single = parts.len() == 1;
        let mut piece = 0;
        for chunk in parts {
            match chunk {
                Chunk::Section(name) => {
                    top = if name.is_empty() { "General" } else { name }.to_owned();
                    heading = None;
                }
                Chunk::Text(body) => {
                    let split = pieces(body);
                    let alone = single && split.len() == 1;
                    for part in split {
                        let index = piece;
                        piece += 1;
                        if part.trim().is_empty() {
                            continue;
                        }
                        if alone && sub_heading(part) {
                            heading = Some(label(part).to_owned());
                            continue;
                        }
                        let section = match &heading {
                            Some(name) => format!("{top}/{name}"),
                            None => top.clone(),
                        };
                        let context = Scene {
                            catalog,
                            heading: heading.as_deref(),
                            scope: scope(&top),
                        };
                        output.push(row(part, line_no, index, section, &context));
                    }
                }
            }
        }
    }
    output
}

type EntityRow = (i64, String, Option<String>);

async fn load_catalog(connection: &mut PgConnection) -> anyhow::Result<Catalog> {
    let entities: Vec<EntityRow> = sqlx::query_as(
        "SELECT e.id,e.kind::text,n.name FROM canon.entity e
         LEFT JOIN LATERAL (
            SELECT f.value_text::text AS name FROM canon.fact f
            WHERE f.entity_id=e.id AND f.stat_key='name_en' AND f.value_text IS NOT NULL
                AND NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=f.receipt_id)
            ORDER BY f.valid_to_run IS NULL DESC,f.valid_from_run DESC LIMIT 1
         ) n ON true
         WHERE e.kind IN ('hero','ability','item') ORDER BY e.id",
    )
    .fetch_all(&mut *connection)
    .await?;
    let aliases: Vec<(i64, String)> = sqlx::query_as(
        "SELECT DISTINCT a.entity_id,a.alias_norm::text FROM canon.alias a
         JOIN canon.entity e ON e.id=a.entity_id
         WHERE e.kind IN ('hero','ability','item') AND a.lang IN ('en','und')
            AND NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=a.receipt_id)
         ORDER BY 1,2",
    )
    .fetch_all(&mut *connection)
    .await?;
    let relations: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT DISTINCT r.from_entity,r.to_entity FROM canon.relation r
         WHERE r.kind='has_ability'
            AND NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.receipt_id)
         ORDER BY 1,2",
    )
    .fetch_all(&mut *connection)
    .await?;
    Ok(Catalog::new(&entities, &aliases, &relations))
}

async fn upsert(
    connection: &mut PgConnection,
    run_id: i64,
    keyed: &[(i64, Row)],
) -> anyhow::Result<u64> {
    let column = |pick: fn(&Row) -> Option<String>| -> Vec<Option<String>> {
        keyed.iter().map(|(_, row)| pick(row)).collect()
    };
    let result = sqlx::query(
        "INSERT INTO canon.patch_line AS l (ledger_id,line_no,piece,section,entity_type,entity_id,
            entity_name,ability_id,ability_name,alias_used,change_type,stat_name,old_value,new_value,
            raw_line,needs_review,source,correction_ref,run_id)
         SELECT s.*,'extract',NULL,$17 FROM unnest($1::bigint[],$2::int[],$3::int[],$4::text[],
            $5::text[],$6::bigint[],$7::text[],$8::bigint[],$9::text[],$10::text[],$11::text[],
            $12::text[],$13::text[],$14::text[],$15::text[],$16::boolean[]) AS s
         ON CONFLICT (ledger_id,line_no,piece) DO UPDATE SET section=EXCLUDED.section,
            entity_type=EXCLUDED.entity_type,entity_id=EXCLUDED.entity_id,
            entity_name=EXCLUDED.entity_name,ability_id=EXCLUDED.ability_id,
            ability_name=EXCLUDED.ability_name,alias_used=EXCLUDED.alias_used,
            change_type=EXCLUDED.change_type,stat_name=EXCLUDED.stat_name,
            old_value=EXCLUDED.old_value,new_value=EXCLUDED.new_value,raw_line=EXCLUDED.raw_line,
            needs_review=EXCLUDED.needs_review,run_id=EXCLUDED.run_id
         WHERE l.source='extract' AND (l.section,l.entity_type,l.entity_id,l.entity_name,
                l.ability_id,l.ability_name,l.alias_used,l.change_type,l.stat_name,l.old_value,
                l.new_value,l.raw_line,l.needs_review)
            IS DISTINCT FROM (EXCLUDED.section,EXCLUDED.entity_type,EXCLUDED.entity_id,
                EXCLUDED.entity_name,EXCLUDED.ability_id,EXCLUDED.ability_name,EXCLUDED.alias_used,
                EXCLUDED.change_type,EXCLUDED.stat_name,EXCLUDED.old_value,EXCLUDED.new_value,
                EXCLUDED.raw_line,EXCLUDED.needs_review)",
    )
    .bind(keyed.iter().map(|(ledger, _)| *ledger).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.line_no).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.piece).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.section.clone()).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.entity_type).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.entity_id).collect::<Vec<_>>())
    .bind(column(|row| row.entity_name.clone()))
    .bind(keyed.iter().map(|(_, row)| row.ability_id).collect::<Vec<_>>())
    .bind(column(|row| row.ability_name.clone()))
    .bind(column(|row| row.alias_used.clone()))
    .bind(column(|row| row.change_type.map(str::to_owned)))
    .bind(column(|row| row.stat_name.clone()))
    .bind(column(|row| row.old_value.clone()))
    .bind(column(|row| row.new_value.clone()))
    .bind(keyed.iter().map(|(_, row)| row.raw_line.clone()).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.needs_review).collect::<Vec<_>>())
    .bind(run_id)
    .execute(&mut *connection)
    .await?;
    Ok(result.rows_affected())
}

async fn delete_stale(connection: &mut PgConnection, keyed: &[(i64, Row)]) -> anyhow::Result<u64> {
    let result = sqlx::query(
        "DELETE FROM canon.patch_line l WHERE l.source='extract'
            AND NOT EXISTS(SELECT 1 FROM unnest($1::bigint[],$2::int[],$3::int[]) AS k(ledger_id,line_no,piece)
                WHERE k.ledger_id=l.ledger_id AND k.line_no=l.line_no AND k.piece=l.piece)",
    )
    .bind(keyed.iter().map(|(ledger, _)| *ledger).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.line_no).collect::<Vec<_>>())
    .bind(keyed.iter().map(|(_, row)| row.piece).collect::<Vec<_>>())
    .execute(&mut *connection)
    .await?;
    Ok(result.rows_affected())
}

pub async fn refresh(pool: &PgPool, run_id: i64) -> anyhow::Result<Refresh> {
    let mut transaction = pool.begin().await?;
    sqlx::query(LEDGER_LOCK).execute(&mut *transaction).await?;
    let catalog = load_catalog(&mut transaction).await?;
    let posts: Vec<(i64, String)> = sqlx::query_as(
        "SELECT s.post_id,s.raw_content FROM ops.patch_ledger_source() s
         JOIN ops.patch_ledger_post p ON p.post_id=s.post_id AND p.disposition='primary'
         ORDER BY s.post_id",
    )
    .fetch_all(&mut *transaction)
    .await?;
    let keyed: Vec<(i64, Row)> = posts
        .iter()
        .flat_map(|(ledger, content)| {
            rows(content, &catalog)
                .into_iter()
                .map(move |row| (*ledger, row))
        })
        .collect();
    let written = upsert(&mut transaction, run_id, &keyed).await?;
    let deleted = delete_stale(&mut transaction, &keyed).await?;
    transaction.commit().await?;
    Ok(Refresh {
        rows: keyed.len(),
        written,
        deleted,
    })
}

fn unescape(field: &str) -> String {
    let mut output = String::with_capacity(field.len());
    let mut chars = field.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        match chars.next() {
            Some('t') => output.push('\t'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some(other) => output.push(other),
            None => output.push('\\'),
        }
    }
    output
}

fn optional(field: &str) -> Option<String> {
    (!field.is_empty() && field != "\\N").then(|| unescape(field))
}

fn required(field: &str, name: &str, line: usize) -> anyhow::Result<String> {
    optional(field).with_context(|| format!("Zeile {line}: Pflichtfeld {name} fehlt"))
}

fn number<T: std::str::FromStr>(field: &str, name: &str, line: usize) -> anyhow::Result<T> {
    required(field, name, line)?
        .parse()
        .map_err(|_| anyhow::anyhow!("Zeile {line}: {name} ist keine Zahl"))
}

fn optional_id(field: &str, name: &str, line: usize) -> anyhow::Result<Option<i64>> {
    optional(field)
        .map(|value| {
            value
                .parse()
                .map_err(|_| anyhow::anyhow!("Zeile {line}: {name} ist keine Zahl"))
        })
        .transpose()
}

fn flag(field: &str, line: usize) -> anyhow::Result<bool> {
    match field {
        "true" | "t" | "1" => Ok(true),
        "false" | "f" | "0" | "" => Ok(false),
        _ => bail!("Zeile {line}: needs_review muss true oder false sein"),
    }
}

fn correction(fields: &[&str], line: usize) -> anyhow::Result<Correction> {
    let [ledger_id, line_no, piece, section, entity_type, entity_id, entity_name, ability_id, ability_name, alias_used, change_type, stat_name, old_value, new_value, raw_line, source, correction_ref, needs_review] =
        fields
    else {
        bail!(
            "Zeile {line}: erwartet {} Spalten, gefunden {}",
            IMPORT_COLUMNS.len(),
            fields.len()
        );
    };
    let source = required(source, "source", line)?;
    if source == "extract" {
        bail!("Zeile {line}: source extract ist für Korrekturen nicht erlaubt");
    }
    Ok(Correction {
        ledger_id: number(ledger_id, "ledger_id", line)?,
        line_no: number(line_no, "line_no", line)?,
        piece: number(piece, "piece", line)?,
        section: required(section, "section", line)?,
        entity_type: required(entity_type, "entity_type", line)?,
        entity_id: optional_id(entity_id, "entity_id", line)?,
        entity_name: optional(entity_name),
        ability_id: optional_id(ability_id, "ability_id", line)?,
        ability_name: optional(ability_name),
        alias_used: optional(alias_used),
        change_type: optional(change_type),
        stat_name: optional(stat_name),
        old_value: optional(old_value),
        new_value: optional(new_value),
        raw_line: required(raw_line, "raw_line", line)?,
        source,
        correction_ref: required(correction_ref, "correction_ref", line)?,
        needs_review: flag(needs_review, line)?,
    })
}

pub fn parse_corrections(text: &str) -> anyhow::Result<Vec<Correction>> {
    let mut lines = text.lines().enumerate();
    let header = lines
        .next()
        .map(|(_, header)| header.trim_end_matches('\r'));
    if header != Some(IMPORT_COLUMNS.join("\t").as_str()) {
        bail!("Kopfzeile muss genau lauten: {}", IMPORT_COLUMNS.join(","));
    }
    let mut output = Vec::new();
    let mut keys = BTreeSet::new();
    for (index, text) in lines {
        let text = text.trim_end_matches('\r');
        if text.is_empty() {
            continue;
        }
        let fields: Vec<&str> = text.split('\t').collect();
        let parsed = correction(&fields, index + 1)?;
        if !keys.insert((parsed.ledger_id, parsed.line_no, parsed.piece)) {
            bail!(
                "Zeile {}: Schlüssel ({},{},{}) doppelt",
                index + 1,
                parsed.ledger_id,
                parsed.line_no,
                parsed.piece
            );
        }
        output.push(parsed);
    }
    Ok(output)
}

const IMPORTED: &str = "WITH i AS (SELECT * FROM unnest($1::bigint[],$2::int[],$3::int[],$4::text[],
        $5::text[],$6::bigint[],$7::text[],$8::bigint[],$9::text[],$10::text[],$11::text[],$12::text[],
        $13::text[],$14::text[],$15::text[],$16::text[],$17::text[],$18::boolean[])
    AS s(ledger_id,line_no,piece,section,entity_type,entity_id,entity_name,ability_id,ability_name,
        alias_used,change_type,stat_name,old_value,new_value,raw_line,source,correction_ref,
        needs_review)) ";

type Query<'q> = sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>;

fn bound<'q>(sql: &'q str, corrections: &[Correction]) -> Query<'q> {
    let text = |pick: fn(&Correction) -> Option<String>| -> Vec<Option<String>> {
        corrections.iter().map(pick).collect()
    };
    sqlx::query(sql)
        .bind(corrections.iter().map(|c| c.ledger_id).collect::<Vec<_>>())
        .bind(corrections.iter().map(|c| c.line_no).collect::<Vec<_>>())
        .bind(corrections.iter().map(|c| c.piece).collect::<Vec<_>>())
        .bind(text(|c| Some(c.section.clone())))
        .bind(text(|c| Some(c.entity_type.clone())))
        .bind(corrections.iter().map(|c| c.entity_id).collect::<Vec<_>>())
        .bind(text(|c| c.entity_name.clone()))
        .bind(corrections.iter().map(|c| c.ability_id).collect::<Vec<_>>())
        .bind(text(|c| c.ability_name.clone()))
        .bind(text(|c| c.alias_used.clone()))
        .bind(text(|c| c.change_type.clone()))
        .bind(text(|c| c.stat_name.clone()))
        .bind(text(|c| c.old_value.clone()))
        .bind(text(|c| c.new_value.clone()))
        .bind(text(|c| Some(c.raw_line.clone())))
        .bind(text(|c| Some(c.source.clone())))
        .bind(text(|c| Some(c.correction_ref.clone())))
        .bind(
            corrections
                .iter()
                .map(|c| c.needs_review)
                .collect::<Vec<_>>(),
        )
}

pub async fn import(pool: &PgPool, corrections: &[Correction]) -> anyhow::Result<u64> {
    let mut transaction = pool.begin().await?;
    sqlx::query(LEDGER_LOCK).execute(&mut *transaction).await?;
    let check = format!(
        "{IMPORTED}SELECT count(*),min(concat_ws(',',i.ledger_id,i.line_no,i.piece)||': '||CASE
                WHEN l.ledger_id IS NULL THEN 'Schlüssel fehlt'
                WHEN l.raw_line<>i.raw_line THEN 'raw_line weicht ab'
                ELSE 'Entität passt nicht zu entity_type' END)
         FROM i
         LEFT JOIN canon.patch_line l ON l.ledger_id=i.ledger_id AND l.line_no=i.line_no AND l.piece=i.piece
         LEFT JOIN canon.entity e ON e.id=i.entity_id
         LEFT JOIN canon.entity a ON a.id=i.ability_id
         WHERE l.ledger_id IS NULL OR l.raw_line<>i.raw_line
            OR (i.entity_id IS NOT NULL AND e.kind::text IS DISTINCT FROM CASE i.entity_type
                WHEN 'hero' THEN 'hero' WHEN 'ability' THEN 'hero' WHEN 'item' THEN 'item' END)
            OR (i.ability_id IS NOT NULL AND a.kind::text IS DISTINCT FROM 'ability')"
    );
    let found = bound(&check, corrections)
        .fetch_one(&mut *transaction)
        .await?;
    let failed: i64 = found.try_get(0)?;
    let first: Option<String> = found.try_get(1)?;
    if failed > 0 {
        bail!(
            "Import abgebrochen, {failed} fehlerhafte Zeilen, erste {}",
            first.unwrap_or_default()
        );
    }
    let update = format!(
        "{IMPORTED}UPDATE canon.patch_line l SET section=i.section,entity_type=i.entity_type,
            entity_id=i.entity_id,entity_name=i.entity_name,ability_id=i.ability_id,
            ability_name=i.ability_name,alias_used=i.alias_used,change_type=i.change_type,
            stat_name=i.stat_name,old_value=i.old_value,new_value=i.new_value,source=i.source,
            correction_ref=i.correction_ref,needs_review=i.needs_review
         FROM i WHERE l.ledger_id=i.ledger_id AND l.line_no=i.line_no AND l.piece=i.piece"
    );
    let updated = bound(&update, corrections)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
    if updated != corrections.len() as u64 {
        bail!(
            "Import abgebrochen, {updated} von {} Zeilen geschrieben",
            corrections.len()
        );
    }
    transaction.commit().await?;
    Ok(updated)
}

#[cfg(test)]
#[path = "patch_line_tests.rs"]
mod tests;
