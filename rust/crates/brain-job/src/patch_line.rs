use super::{
    annotation, bullet, chunks, explicit_values, first_word, indent, label, qualification_word,
    structural_heading, subject, Chunk, SUBJECT_CHARS, SUBJECT_WORDS,
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
const REDUCERS: &[&str] = &[
    "reduction",
    "reductions",
    "reducer",
    "resist",
    "resists",
    "resistance",
    "resistances",
];
const LESSER: &[&str] = &["reduced", "decreased", "lower", "lowered", "less"];
const VALUE_WORDS: &[&str] = &["from", "to", "by"];
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
    labels: BTreeMap<i64, String>,
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
    pub remapped: usize,
    pub archived: usize,
}

enum Mention<'a> {
    Absent,
    One(i64, &'a str),
    Ambiguous,
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
                if !alias.trim().is_empty() {
                    catalog
                        .labels
                        .entry(*id)
                        .or_insert_with(|| alias.trim().to_owned());
                }
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

    fn name(&self, id: i64, printed: &str) -> Option<String> {
        self.names
            .get(&id)
            .cloned()
            .or_else(|| {
                let printed = printed.trim();
                (!printed.is_empty()).then(|| printed.to_owned())
            })
            .or_else(|| self.labels.get(&id).cloned())
    }

    fn owner(&self, ability: i64, context: Option<i64>) -> Option<i64> {
        let owners = self.owners.get(&ability)?;
        if let Some(hero) = context.filter(|hero| owners.contains(hero)) {
            return Some(hero);
        }
        let first = *owners.first()?;
        (owners.len() == 1).then_some(first)
    }

    fn ability_in<'a>(&self, hero: i64, text: &'a str) -> Mention<'a> {
        let found = tokens(text);
        let Some(owned) = self.abilities.get(&hero) else {
            return Mention::Absent;
        };
        let mut matches: Vec<(usize, usize, i64)> = Vec::new();
        for start in 0..found.len() {
            for (key, ability) in owned {
                let end = start + key.len();
                if end <= found.len()
                    && found[start..end]
                        .iter()
                        .zip(key)
                        .all(|(token, word)| &token.word == word)
                {
                    matches.extend([(start, end, *ability)]);
                }
            }
        }
        let outer: Vec<(usize, usize, i64)> = matches
            .iter()
            .filter(|(start, end, _)| {
                !matches.iter().any(|(other_start, other_end, _)| {
                    other_start <= start
                        && end <= other_end
                        && other_end - other_start > end - start
                })
            })
            .copied()
            .collect();
        let ids: BTreeSet<i64> = outer.iter().map(|(_, _, id)| *id).collect();
        let chosen = match ids.len() {
            0 => None,
            1 => outer.first(),
            _ => {
                let leading: Vec<&(usize, usize, i64)> =
                    outer.iter().filter(|(start, _, _)| *start == 0).collect();
                let lead_ids: BTreeSet<i64> = leading.iter().map(|(_, _, id)| *id).collect();
                if lead_ids.len() != 1 {
                    return Mention::Ambiguous;
                }
                leading.first().copied()
            }
        };
        match chosen {
            Some((start, end, id)) => Mention::One(*id, span(text, &found[*start..*end])),
            None => Mention::Absent,
        }
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
        if let Some(position) = verb(&words) {
            if words[position] == "improved" {
                return Some("buff");
            }
            let up = UP.contains(&words[position].as_str());
            let inverse = lower_is_better(stat_phrase(&words, position));
            return Some(if up != inverse { "buff" } else { "nerf" });
        }
    }
    has(CHANGE_VERBS).then_some("changed")
}

fn verb(words: &[String]) -> Option<usize> {
    let candidate = |word: &String| {
        word == "improved" || UP.contains(&word.as_str()) || DOWN.contains(&word.as_str())
    };
    let followed = |index: usize| {
        words.get(index + 1).is_none_or(|next| {
            VALUE_WORDS.contains(&next.as_str()) || next.starts_with(|ch: char| ch.is_ascii_digit())
        })
    };
    (0..words.len())
        .find(|index| candidate(&words[*index]) && followed(*index))
        .or_else(|| words.iter().position(candidate))
}

fn stat_phrase(words: &[String], position: usize) -> &[String] {
    if position > 0 {
        return &words[..position];
    }
    let rest = &words[1..];
    let end = rest
        .iter()
        .position(|word| VALUE_WORDS.contains(&word.as_str()))
        .unwrap_or(rest.len());
    &rest[..end]
}

fn lower_is_better(phrase: &[String]) -> bool {
    let mut inverse = false;
    let mut previous: Option<&str> = None;
    for word in phrase {
        let word = word.as_str();
        if REDUCERS.contains(&word) {
            inverse = false;
        } else if LOWER_BETTER.contains(&word) {
            inverse = !previous.is_some_and(|before| LESSER.contains(&before));
        }
        previous = Some(word);
    }
    inverse
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
        let printed = printed.trim();
        Binding {
            entity_type,
            entity_id: None,
            entity_name: (!printed.is_empty()).then(|| printed.to_owned()),
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
    fn hero<'a>(
        &self,
        hero: i64,
        printed: Option<&str>,
        fallback: &str,
        statement: &'a str,
    ) -> Binding<'a> {
        let entity_name = self.catalog.name(hero, printed.unwrap_or(fallback));
        let hero_alias = printed.and_then(|printed| {
            entity_name
                .as_deref()
                .and_then(|name| differs(printed, name))
        });
        let unnamed = entity_name.is_none();
        let plain = |needs_review: bool| Binding {
            entity_type: "hero",
            entity_id: Some(hero),
            entity_name: entity_name.clone(),
            ability: None,
            alias_used: hero_alias.clone(),
            needs_review: needs_review || unnamed,
        };
        match self.catalog.ability_in(hero, statement) {
            Mention::One(ability, spelled) => match self.catalog.name(ability, spelled) {
                Some(ability_name) => Binding {
                    entity_type: "ability",
                    entity_id: Some(hero),
                    alias_used: differs(spelled, &ability_name).or(hero_alias.clone()),
                    entity_name: entity_name.clone(),
                    ability: Some((ability, ability_name, spelled)),
                    needs_review: unnamed,
                },
                None => plain(true),
            },
            Mention::Absent => plain(false),
            Mention::Ambiguous => plain(true),
        }
    }

    fn entity<'a>(&self, id: i64, kind: Kind, printed: &str, statement: &'a str) -> Binding<'a> {
        let heading_hero = self.heading_hero();
        match kind {
            Kind::Hero => self.hero(id, Some(printed), printed, statement),
            Kind::Item => {
                let entity_name = self.catalog.name(id, printed);
                Binding {
                    entity_type: "item",
                    entity_id: Some(id),
                    alias_used: entity_name
                        .as_deref()
                        .and_then(|name| differs(printed, name)),
                    needs_review: entity_name.is_none(),
                    entity_name,
                    ability: None,
                }
            }
            Kind::Ability => {
                let owner = self.catalog.owner(id, heading_hero);
                let owner_name = owner.and_then(|hero| self.catalog.name(hero, ""));
                match self.catalog.name(id, printed) {
                    Some(ability_name) => Binding {
                        entity_type: "ability",
                        entity_id: owner,
                        needs_review: owner_name.is_none(),
                        entity_name: owner_name,
                        alias_used: differs(printed, &ability_name),
                        ability: Some((id, ability_name, "")),
                    },
                    None => Binding::unresolved("ability", printed),
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

    fn unresolved_scope(&self, printed: &str) -> Option<Binding<'static>> {
        match self.scope {
            Scope::Hero => Some(Binding::unresolved("hero", printed)),
            Scope::Item => Some(Binding::unresolved("item", printed)),
            Scope::Other => None,
        }
    }

    fn by_heading<'a>(&self, statement: &'a str, is_bullet: bool) -> Binding<'a> {
        if let Some(heading) = self.heading {
            if let Some((id, kind)) = self.catalog.resolve(heading) {
                return match kind {
                    Kind::Hero => self.hero(id, None, heading, statement),
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
            if let Some(binding) = self.unresolved_scope(heading) {
                return binding;
            }
        }
        match objective_at(statement, OBJECTIVE_WORDS, false).filter(|_| is_bullet) {
            Some((name, printed)) => Binding::objective(name, printed),
            None => Binding::general(),
        }
    }

    fn bind<'a>(&self, body: &'a str, bulleted: bool) -> (Binding<'a>, &'a str) {
        let Some((name, rest)) = subject(body) else {
            return (self.by_heading(body, bulleted), body);
        };
        if let Some((id, kind)) = self.catalog.resolve(name) {
            return (self.entity(id, kind, name, rest), rest);
        }
        if let Some((objective, printed)) = objective_at(name, 1, true) {
            return (Binding::objective(objective, printed), rest);
        }
        if let Some(hero) = self.heading_hero() {
            let heading = self.heading.unwrap_or_default();
            return (self.hero(hero, None, heading, body), body);
        }
        let binding = self.unresolved_scope(name).unwrap_or_else(Binding::general);
        (binding, rest)
    }
}

fn row(body: &str, bulleted: bool, key: (usize, usize), section: String, context: &Scene) -> Row {
    let (binding, statement) = context.bind(body, bulleted);
    let printed_ability = binding
        .ability
        .as_ref()
        .map(|(_, _, printed)| *printed)
        .filter(|printed| !printed.is_empty());
    let (stat_name, old_value, new_value) = stat(statement, printed_ability);
    sanitize(Row {
        line_no: i32::try_from(key.0).unwrap_or(i32::MAX),
        piece: i32::try_from(key.1).unwrap_or(i32::MAX),
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
        raw_line: body.trim().to_owned(),
        needs_review: binding.needs_review,
    })
}

fn filled(value: Option<String>) -> Option<String> {
    value.filter(|text| !text.trim().is_empty())
}

fn change_word(value: &Option<String>) -> bool {
    value
        .as_deref()
        .is_some_and(|text| CHANGE_WORDS.contains(&text.trim().to_lowercase().as_str()))
}

fn sanitize(mut row: Row) -> Row {
    row.entity_name = filled(row.entity_name);
    row.ability_name = filled(row.ability_name);
    row.alias_used = filled(row.alias_used);
    row.stat_name = filled(row.stat_name);
    if change_word(&row.stat_name) {
        row.stat_name = None;
    }
    row.old_value = filled(row.old_value);
    row.new_value = filled(row.new_value);
    if row.old_value.is_none()
        || row.new_value.is_none()
        || change_word(&row.old_value)
        || change_word(&row.new_value)
    {
        row.old_value = None;
        row.new_value = None;
    }
    if row.entity_type == "ability" && row.ability_name.is_none() {
        row.entity_type = if row.entity_id.is_some() {
            "hero"
        } else {
            "general"
        };
        row.needs_review = true;
    }
    if row.entity_type != "ability" {
        row.ability_id = None;
        row.ability_name = None;
    }
    if row.entity_type == "general" {
        row.entity_id = None;
    }
    row
}

fn qualifier_row(parent: &Row, body: &str, key: (usize, usize), section: String) -> Row {
    sanitize(Row {
        line_no: i32::try_from(key.0).unwrap_or(i32::MAX),
        piece: i32::try_from(key.1).unwrap_or(i32::MAX),
        section,
        entity_type: parent.entity_type,
        entity_id: parent.entity_id,
        entity_name: parent.entity_name.clone(),
        ability_id: parent.ability_id,
        ability_name: parent.ability_name.clone(),
        alias_used: None,
        change_type: change_type(body, parent.entity_type),
        stat_name: None,
        old_value: None,
        new_value: None,
        raw_line: body.trim().to_owned(),
        needs_review: parent.needs_review,
    })
}

fn sub_heading(body: &str) -> bool {
    let name = label(body);
    structural_heading(body)
        && name.chars().count() <= SUBJECT_CHARS
        && name.split_whitespace().count() <= SUBJECT_WORDS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece<'a> {
    Section(&'a str),
    Statement { text: &'a str, bulleted: bool },
}

const STATEMENT_SEPARATORS: [&str; 3] = [" - ", " * ", " • "];

fn split_statements(body: &str) -> Vec<&str> {
    let mut output = Vec::new();
    let mut rest = body;
    while let Some((index, width)) = STATEMENT_SEPARATORS
        .into_iter()
        .filter_map(|separator| rest.find(separator).map(|index| (index, separator.len())))
        .min()
    {
        output.extend([rest[..index].trim()]);
        rest = &rest[index + width..];
    }
    output.extend([rest.trim()]);
    output
        .into_iter()
        .map(strip_marker)
        .filter(|piece| !piece.is_empty())
        .collect()
}

fn strip_marker(piece: &str) -> &str {
    let piece = piece.trim();
    match piece.strip_prefix(['-', '*', '•']) {
        Some(rest) if rest.is_empty() || rest.starts_with(char::is_whitespace) => rest.trim(),
        _ => piece,
    }
}

pub fn statements(line: &str) -> Vec<Piece<'_>> {
    let mut output = Vec::new();
    for chunk in chunks(line) {
        match chunk {
            Chunk::Section(name) => output.extend([Piece::Section(name)]),
            Chunk::Text(body) => {
                let (rest, bulleted) = match bullet(body) {
                    Some(rest) => (rest, true),
                    None => (body.trim(), false),
                };
                for (index, text) in split_statements(rest).into_iter().enumerate() {
                    output.extend([Piece::Statement {
                        text,
                        bulleted: bulleted || index > 0,
                    }]);
                }
            }
        }
    }
    output
}

pub fn statement_text(raw: &str) -> String {
    bullet(raw).unwrap_or(raw.trim()).trim().to_owned()
}

fn inline_qualifier(body: &str) -> bool {
    subject(body).is_none()
        && (annotation(body) || first_word(body).is_some_and(qualification_word))
}

struct Open {
    parent: usize,
    indent: usize,
}

pub fn rows(content: &str, catalog: &Catalog) -> Vec<Row> {
    let mut output: Vec<Row> = Vec::new();
    let mut top = "General".to_owned();
    let mut heading: Option<String> = None;
    let mut open: Option<Open> = None;
    for (line_no, text) in content.lines().enumerate() {
        if text.trim().is_empty() {
            open = None;
            continue;
        }
        let parts = statements(text);
        let alone = parts.len() == 1;
        let line_indent = indent(text);
        let mut piece = 0;
        let mut first = true;
        for part in parts {
            let (body, bulleted) = match part {
                Piece::Section(name) => {
                    top = if name.is_empty() { "General" } else { name }.to_owned();
                    heading = None;
                    open = None;
                    first = false;
                    continue;
                }
                Piece::Statement { text, bulleted } => (text, bulleted),
            };
            let key = (line_no, piece);
            piece += 1;
            let leading = first;
            first = false;
            let heading_line = alone && !bulleted && sub_heading(body);
            let continues = open.as_ref().is_some_and(|open| {
                if leading {
                    line_indent > open.indent
                        || annotation(body)
                        || (!bulleted && subject(body).is_none() && !heading_line)
                } else {
                    inline_qualifier(body)
                }
            });
            let section = match &heading {
                Some(name) => format!("{top}/{name}"),
                None => top.clone(),
            };
            if let (true, Some(parent)) = (continues, open.as_ref().map(|open| open.parent)) {
                if let Some(row) = output.get_mut(parent) {
                    row.stat_name = None;
                    row.old_value = None;
                    row.new_value = None;
                }
                if subject(body).is_none() {
                    let qualifier = output
                        .get(parent)
                        .map(|row| qualifier_row(row, body, key, section));
                    output.extend(qualifier);
                    continue;
                }
            } else if heading_line {
                heading = Some(label(body).to_owned());
                open = None;
                continue;
            }
            let context = Scene {
                catalog,
                heading: heading.as_deref(),
                scope: scope(&top),
            };
            output.extend([row(body, bulleted, key, section, &context)]);
            if !continues {
                open = Some(Open {
                    parent: output.len() - 1,
                    indent: line_indent,
                });
            }
        }
    }
    output
}

type EntityRow = (i64, String, Option<String>);

async fn load_catalog(connection: &mut PgConnection) -> anyhow::Result<Catalog> {
    let entities: Vec<EntityRow> =
        sqlx::query_as("SELECT id,kind,name FROM ops.patch_line_entities()")
            .fetch_all(&mut *connection)
            .await?;
    let aliases: Vec<(i64, String)> =
        sqlx::query_as("SELECT entity_id,alias FROM ops.patch_line_aliases()")
            .fetch_all(&mut *connection)
            .await?;
    let relations: Vec<(i64, i64)> =
        sqlx::query_as("SELECT hero_id,ability_id FROM ops.patch_line_relations()")
            .fetch_all(&mut *connection)
            .await?;
    Ok(Catalog::new(&entities, &aliases, &relations))
}

pub type Key = (i64, i32, i32);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub remap: Vec<(Key, Key)>,
    pub archive: Vec<(Key, &'static str)>,
}

pub fn plan(
    held: &[(Key, String)],
    ledgers: &BTreeSet<i64>,
    groups: &BTreeMap<i64, i64>,
    fresh: &BTreeMap<Key, String>,
) -> Plan {
    let held_keys: BTreeSet<Key> = held.iter().map(|(key, _)| *key).collect();
    let mut taken = BTreeSet::new();
    let mut output = Plan::default();
    for (key, raw_line) in held {
        if !ledgers.contains(&key.0) {
            let target = groups.get(&key.0).and_then(|group| {
                let mut found = fresh
                    .iter()
                    .filter(|(candidate, text)| candidate.0 == *group && *text == raw_line)
                    .map(|(candidate, _)| *candidate);
                match (found.next(), found.next()) {
                    (Some(only), None) => Some(only),
                    _ => None,
                }
            });
            match target {
                Some(target) if !held_keys.contains(&target) && taken.insert(target) => {
                    output.remap.extend([(*key, target)]);
                }
                _ => output.archive.extend([(*key, "ledger_vanished")]),
            }
            continue;
        }
        match fresh.get(key) {
            None => output.archive.extend([(*key, "key_vanished")]),
            Some(text) if text != raw_line => output.archive.extend([(*key, "raw_line_changed")]),
            Some(_) => {}
        }
    }
    output
}

async fn reconcile(
    connection: &mut PgConnection,
    run_id: i64,
    keyed: &[(i64, Row)],
) -> anyhow::Result<Plan> {
    let held: Vec<(i64, i32, i32, String)> = sqlx::query_as(
        "SELECT ledger_id,line_no,piece,raw_line FROM canon.patch_line
         WHERE source<>'extract' ORDER BY ledger_id,line_no,piece",
    )
    .fetch_all(&mut *connection)
    .await?;
    if held.is_empty() {
        return Ok(Plan::default());
    }
    let ledgers: Vec<i64> = sqlx::query_scalar("SELECT ledger_id FROM ops.patch_ledger")
        .fetch_all(&mut *connection)
        .await?;
    let groups: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT post_id,ledger_id FROM ops.patch_ledger_post WHERE ledger_id IS NOT NULL",
    )
    .fetch_all(&mut *connection)
    .await?;
    let fresh: BTreeMap<Key, String> = keyed
        .iter()
        .map(|(ledger, row)| ((*ledger, row.line_no, row.piece), row.raw_line.clone()))
        .collect();
    let held: Vec<(Key, String)> = held
        .into_iter()
        .map(|(ledger, line, piece, raw)| ((ledger, line, piece), raw))
        .collect();
    let decided = plan(
        &held,
        &ledgers.into_iter().collect(),
        &groups.into_iter().collect(),
        &fresh,
    );
    archive(connection, run_id, &decided.archive).await?;
    remap(connection, &decided.remap).await?;
    Ok(decided)
}

async fn archive(
    connection: &mut PgConnection,
    run_id: i64,
    archived: &[(Key, &'static str)],
) -> anyhow::Result<()> {
    if archived.is_empty() {
        return Ok(());
    }
    sqlx::query(
        "WITH k AS (SELECT * FROM unnest($1::bigint[],$2::int[],$3::int[],$4::text[])
                AS k(ledger_id,line_no,piece,reason)),
            moved AS (DELETE FROM canon.patch_line l USING k
                WHERE l.ledger_id=k.ledger_id AND l.line_no=k.line_no AND l.piece=k.piece
                    AND l.source<>'extract'
                RETURNING l.*,k.reason)
         INSERT INTO ops.patch_line_correction_archive (ledger_id,line_no,piece,section,entity_type,
            entity_id,entity_name,ability_id,ability_name,alias_used,change_type,stat_name,old_value,
            new_value,raw_line,source,correction_ref,needs_review,line_run_id,reason,run_id)
         SELECT ledger_id,line_no,piece,section,entity_type,entity_id,entity_name,ability_id,
            ability_name,alias_used,change_type,stat_name,old_value,new_value,raw_line,source,
            correction_ref,needs_review,run_id,reason,$5 FROM moved",
    )
    .bind(archived.iter().map(|(key, _)| key.0).collect::<Vec<_>>())
    .bind(archived.iter().map(|(key, _)| key.1).collect::<Vec<_>>())
    .bind(archived.iter().map(|(key, _)| key.2).collect::<Vec<_>>())
    .bind(archived.iter().map(|(_, reason)| *reason).collect::<Vec<_>>())
    .bind(run_id)
    .execute(&mut *connection)
    .await?;
    Ok(())
}

async fn remap(connection: &mut PgConnection, moves: &[(Key, Key)]) -> anyhow::Result<()> {
    if moves.is_empty() {
        return Ok(());
    }
    let column = |pick: fn(&(Key, Key)) -> i64| moves.iter().map(pick).collect::<Vec<_>>();
    let small = |pick: fn(&(Key, Key)) -> i32| moves.iter().map(pick).collect::<Vec<_>>();
    sqlx::query(
        "DELETE FROM canon.patch_line l
         USING unnest($1::bigint[],$2::int[],$3::int[]) AS t(ledger_id,line_no,piece)
         WHERE l.ledger_id=t.ledger_id AND l.line_no=t.line_no AND l.piece=t.piece
            AND l.source='extract'",
    )
    .bind(column(|(_, to)| to.0))
    .bind(small(|(_, to)| to.1))
    .bind(small(|(_, to)| to.2))
    .execute(&mut *connection)
    .await?;
    sqlx::query(
        "UPDATE canon.patch_line l SET ledger_id=m.to_ledger,line_no=m.to_line,piece=m.to_piece
         FROM unnest($1::bigint[],$2::int[],$3::int[],$4::bigint[],$5::int[],$6::int[])
            AS m(ledger_id,line_no,piece,to_ledger,to_line,to_piece)
         WHERE l.ledger_id=m.ledger_id AND l.line_no=m.line_no AND l.piece=m.piece
            AND l.source<>'extract'",
    )
    .bind(column(|(from, _)| from.0))
    .bind(small(|(from, _)| from.1))
    .bind(small(|(from, _)| from.2))
    .bind(column(|(_, to)| to.0))
    .bind(small(|(_, to)| to.1))
    .bind(small(|(_, to)| to.2))
    .execute(&mut *connection)
    .await?;
    Ok(())
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
    let result = sync(&mut transaction, run_id).await?;
    transaction.commit().await?;
    Ok(result)
}

pub async fn sync(connection: &mut PgConnection, run_id: i64) -> anyhow::Result<Refresh> {
    let catalog = load_catalog(connection).await?;
    let posts: Vec<(i64, String)> = sqlx::query_as(
        "SELECT s.post_id,s.raw_content FROM ops.patch_ledger_source() s
         JOIN ops.patch_ledger_post p ON p.post_id=s.post_id AND p.disposition='primary'
         ORDER BY s.post_id",
    )
    .fetch_all(&mut *connection)
    .await?;
    let keyed: Vec<(i64, Row)> = posts
        .iter()
        .flat_map(|(ledger, content)| {
            rows(content, &catalog)
                .into_iter()
                .map(move |row| (*ledger, row))
        })
        .collect();
    let decided = reconcile(connection, run_id, &keyed).await?;
    let written = upsert(connection, run_id, &keyed).await?;
    let deleted = delete_stale(connection, &keyed).await?;
    let result = Refresh {
        rows: keyed.len(),
        written,
        deleted,
        remapped: decided.remap.len(),
        archived: decided.archive.len(),
    };
    if result.remapped + result.archived > 0 {
        eprintln!(
            "{}",
            serde_json::json!({
                "event": "patch_line_corrections_reconciled",
                "run_id": run_id,
                "remapped": result.remapped,
                "archived": result.archived,
            })
        );
    }
    Ok(result)
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
        raw_line: Some(statement_text(&required(raw_line, "raw_line", line)?))
            .filter(|text| !text.is_empty())
            .with_context(|| format!("Zeile {line}: Pflichtfeld raw_line fehlt"))?,
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
