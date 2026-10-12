use crate::{
    extract::{Assertion, Atom},
    git::Source,
};
use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const SOURCE: Source = Source {
    key: "deadlock_patchnotes_db",
    url: "postgres://patchnotes-feed/public.patchnotes",
};
pub const MAX_ENVELOPE_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_STATEMENT_BYTES: usize = 512;
pub const MAX_CHANGES: usize = 4096;
pub const MAX_SEGMENTS: usize = 32768;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownEntity {
    pub key: String,
    pub kind: &'static str,
}

#[derive(Debug, Clone)]
pub struct AliasBinding {
    pub alias: String,
    pub entity: KnownEntity,
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StatBinding {
    pub entity: String,
    pub label: String,
    pub stat: String,
}

#[derive(Debug, Default)]
pub struct AliasIndex {
    aliases: BTreeMap<String, Vec<AliasBinding>>,
    stats: BTreeMap<(String, String), Vec<String>>,
}

impl AliasIndex {
    pub fn new(aliases: Vec<AliasBinding>, stats: Vec<StatBinding>) -> Result<Self> {
        ensure!(
            aliases.len() <= 100000 && stats.len() <= 100000,
            "patch_catalog_limit"
        );
        let mut index = Self::default();
        for binding in aliases {
            ensure!(
                valid_key(&binding.entity.key)
                    && matches!(binding.entity.kind, "hero" | "ability" | "item" | "timer")
                    && valid_label(&binding.alias)
                    && binding.scope.as_deref().is_none_or(valid_key),
                "patch_alias_invalid"
            );
            index
                .aliases
                .entry(binding.alias.clone())
                .or_default()
                .push(binding);
        }
        for binding in stats {
            ensure!(
                valid_key(&binding.entity)
                    && valid_label(&binding.label)
                    && !binding.stat.is_empty()
                    && binding.stat.len() <= 128,
                "patch_stat_invalid"
            );
            index
                .stats
                .entry((binding.entity, binding.label))
                .or_default()
                .push(binding.stat);
        }
        Ok(index)
    }

    fn resolve(
        &self,
        alias: &str,
        scope: Option<&str>,
    ) -> Result<Option<&AliasBinding>, &'static str> {
        let Some(entries) = self.aliases.get(alias) else {
            return Ok(None);
        };
        let mut matches = entries.iter().filter(|entry| match entry.scope.as_deref() {
            None => true,
            Some(owner) => Some(owner) == scope,
        });
        let Some(first) = matches.next() else {
            return Err("unscoped_alias");
        };
        if matches.any(|entry| entry.entity != first.entity || entry.scope != first.scope) {
            return Err("ambiguous_alias");
        }
        Ok(Some(first))
    }

    fn stat(&self, entity: &str, label: &str) -> Option<&str> {
        let entries = self.stats.get(&(entity.to_owned(), label.to_owned()))?;
        let first = entries.first()?;
        entries
            .iter()
            .all(|entry| entry == first)
            .then_some(first.as_str())
    }
}

fn valid_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn valid_label(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128 && !value.chars().any(char::is_control)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope<'a> {
    id: i64,
    #[serde(borrow)]
    title: &'a RawValue,
    #[serde(borrow)]
    url: &'a RawValue,
    #[serde(borrow)]
    posted_at: &'a RawValue,
    #[serde(borrow)]
    raw_content: &'a RawValue,
}

#[derive(Debug)]
pub struct Post<'a> {
    id: i64,
    title: String,
    url: String,
    posted_at: Option<DateTime<Utc>>,
    raw_content: Option<String>,
    bytes: &'a [u8],
    lexemes: BTreeMap<&'static str, &'a str>,
    content_hash: Vec<u8>,
    post_hash: String,
    trusted: bool,
}

impl Post<'_> {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn posted_at(&self) -> Option<DateTime<Utc>> {
        self.posted_at
    }
    pub fn raw_content(&self) -> Option<&str> {
        self.raw_content.as_deref()
    }
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }
    pub fn lexical_value(&self, pointer: &str) -> Option<&str> {
        self.lexemes.get(pointer).copied()
    }
}

pub fn object_path(id: i64) -> String {
    id.to_string()
}

pub fn supported(source: Source, path: &str) -> bool {
    source == SOURCE
        && path
            .parse::<i64>()
            .ok()
            .is_some_and(|id| id > 0 && object_path(id) == path)
}

pub fn parse_post(bytes: &[u8]) -> Result<Post<'_>> {
    ensure!(bytes.len() <= MAX_ENVELOPE_BYTES, "patch_envelope_limit");
    let envelope: Envelope<'_> = serde_json::from_slice(bytes).context("patch_envelope_invalid")?;
    let title: String =
        serde_json::from_str(envelope.title.get()).context("patch_title_invalid")?;
    let url: String = serde_json::from_str(envelope.url.get()).context("patch_url_invalid")?;
    ensure!(
        envelope.id > 0 && !title.trim().is_empty() && title.len() <= 256 && url.len() <= 2048,
        "patch_metadata_invalid"
    );
    let timestamp: Option<String> =
        serde_json::from_str(envelope.posted_at.get()).context("patch_date_invalid")?;
    let posted_at = timestamp
        .map(|value| {
            DateTime::parse_from_rfc3339(&value)
                .map(|value| value.with_timezone(&Utc))
                .context("patch_date_invalid")
        })
        .transpose()?;
    let raw_content =
        serde_json::from_str(envelope.raw_content.get()).context("patch_content_invalid")?;
    let trusted = official_url(&url);
    Ok(Post {
        id: envelope.id,
        title,
        url,
        posted_at,
        raw_content,
        bytes,
        lexemes: BTreeMap::from([
            ("/title", envelope.title.get()),
            ("/url", envelope.url.get()),
            ("/posted_at", envelope.posted_at.get()),
            ("/raw_content", envelope.raw_content.get()),
        ]),
        content_hash: Sha256::digest(envelope.raw_content.get().as_bytes()).to_vec(),
        post_hash: hex::encode(Sha256::digest(bytes)),
        trusted,
    })
}

fn official_url(value: &str) -> bool {
    if value
        .chars()
        .any(|ch| ch.is_control() || ch.is_whitespace())
        || value.contains(['\\', '%'])
    {
        return false;
    }
    let Ok(url) = url::Url::parse(value) else {
        return false;
    };
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return false;
    }
    let path = url.path().strip_suffix('/').unwrap_or(url.path());
    if url.as_str() != value || value.contains(":443") {
        return false;
    }
    match url.host_str() {
        Some("store.steampowered.com") => path
            .strip_prefix("/news/app/1422450/view/")
            .is_some_and(decimal_id),
        Some("steamcommunity.com") => path
            .strip_prefix("/games/1422450/announcements/detail/")
            .is_some_and(decimal_id),
        Some("forums.playdeadlock.com") => path.strip_prefix("/threads/").is_some_and(|thread| {
            !thread.contains('/')
                && thread.rsplit_once('.').is_some_and(|(slug, id)| {
                    !slug.is_empty()
                        && slug
                            .bytes()
                            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'-')
                        && decimal_id(id)
                })
        }),
        _ => false,
    }
}

fn decimal_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 20
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.bytes().any(|byte| byte != b'0')
}

#[derive(Debug)]
pub struct PatchChange {
    pub assertion: Assertion,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub stat: Option<String>,
    pub change_kind: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Skipped {
    pub statement_index: Option<usize>,
    pub reason: &'static str,
}

#[derive(Debug, Default)]
pub struct Extraction {
    pub changes: Vec<PatchChange>,
    pub skipped: Vec<Skipped>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HeadingBoundary {
    position: usize,
    key: String,
    kind: String,
    scope: Option<String>,
}

#[derive(Clone, Copy)]
struct Segment<'a> {
    raw: &'a str,
    position: usize,
    next_heading: Option<(usize, &'a str)>,
}

fn bullet(value: &str) -> Option<&str> {
    let value = value.trim();
    ["- ", "* ", "• "]
        .into_iter()
        .find_map(|prefix| value.strip_prefix(prefix))
}

fn label(value: &str) -> &str {
    value
        .trim()
        .trim_start_matches('#')
        .trim()
        .trim_end_matches(':')
        .trim()
        .trim_matches(['[', ']', '*'])
        .trim()
}

fn section(value: &str) -> bool {
    matches!(
        label(value),
        "Heroes"
            | "Items"
            | "General"
            | "New Heroes"
            | "Map"
            | "UI"
            | "Audio"
            | "Misc"
            | "Street Brawl"
    )
}

fn segments<'a>(content: &'a str, aliases: &AliasIndex) -> Result<Vec<Segment<'a>>> {
    let mut output = Vec::new();
    let mut start = 0;
    let mut pending: Option<(usize, usize, usize)> = None;
    for (position, line) in content.split_inclusive('\n').enumerate() {
        let text = line.trim_end_matches(['\r', '\n']);
        let indentation = text.len() - text.trim_start().len();
        let body = bullet(text).unwrap_or(text.trim());
        let entity_heading = structural_heading(body) && aliases.aliases.contains_key(label(body));
        let heading =
            indentation == 0 && bullet(text).is_none() && (section(text) || entity_heading);
        let new_bullet = bullet(text).is_some_and(|body| {
            pending.is_none_or(|(_, depth, _)| {
                indentation <= depth
                    && (statement_start(body) || section(body) || entity_heading)
            })
        });
        if heading || new_bullet {
            if let Some((begin, _, begin_position)) = pending.take() {
                output.push(Segment {
                    raw: content[begin..start].trim_end_matches(['\r', '\n']),
                    position: begin_position,
                    next_heading: (entity_heading && !section(body)).then_some((position, body)),
                });
            }
            if heading {
                output.push(Segment {
                    raw: text,
                    position,
                    next_heading: None,
                });
            } else {
                pending = Some((start, indentation, position));
            }
        } else if pending.is_none() && !text.trim().is_empty() {
            pending = Some((start, indentation, position));
        }
        start += line.len();
        ensure!(output.len() <= MAX_SEGMENTS, "patch_segment_limit");
    }
    if let Some((begin, _, position)) = pending {
        output.push(Segment {
            raw: content[begin..].trim_end_matches(['\r', '\n']),
            position,
            next_heading: None,
        });
    }
    ensure!(output.len() <= MAX_SEGMENTS, "patch_segment_limit");
    Ok(output)
}

fn compound_scope(body: &str, aliases: &AliasIndex) -> Result<(), &'static str> {
    let mut clauses = body
        .split(['\n', ';'])
        .flat_map(|part| part.split(" - "))
        .flat_map(|part| part.split(" and "))
        .flat_map(|part| part.split(" & "))
        .flat_map(|part| part.split(" / "))
        .flat_map(|part| part.split(". "))
        .flat_map(|part| part.split(", "));
    let first_action = clauses.next().is_some_and(action);
    for part in clauses {
        let part = bullet(part).unwrap_or(part.trim());
        if let Some((subject, _)) = part.split_once(':') {
            return Err(if aliases.aliases.contains_key(label(subject)) {
                "multiple_entity_statements"
            } else {
                "unknown_subject"
            });
        }
        if aliases.aliases.keys().any(|alias| {
            part.strip_prefix(alias).is_some_and(|rest| {
                rest.starts_with(char::is_whitespace) || rest.starts_with(['\'', '’'])
            })
        }) {
            return Err("multiple_entity_statements");
        }
        if first_action && action(part) {
            return Err("ambiguous_compound_statement");
        }
    }
    Ok(())
}

struct Binding<'a> {
    entity: &'a AliasBinding,
    scope: Option<&'a AliasBinding>,
    contexts: Vec<usize>,
}

fn heading_boundary(
    next_heading: Option<(usize, &str)>,
    current: Option<&Binding<'_>>,
    aliases: &AliasIndex,
) -> Result<Option<HeadingBoundary>, &'static str> {
    next_heading
        .map(|(position, body)| {
            let scope =
                current.map(|entry| entry.scope.unwrap_or(entry.entity).entity.key.as_str());
            let entity = aliases
                .resolve(label(body), scope)?
                .ok_or("unknown_boundary")?;
            Ok(HeadingBoundary {
                position,
                key: entity.entity.key.clone(),
                kind: entity.entity.kind.to_owned(),
                scope: entity.scope.clone(),
            })
        })
        .transpose()
}

fn bind<'a>(
    body: &'a str,
    current: Option<&Binding<'a>>,
    index: &'a AliasIndex,
    position: usize,
) -> Result<(Option<Binding<'a>>, &'a str), &'static str> {
    let mut binding = current.map(|entry| Binding {
        entity: entry.entity,
        scope: entry.scope,
        contexts: entry.contexts.clone(),
    });
    let mut body = body;
    for _ in 0..3 {
        let split = body.split_once(':').or_else(|| body.split_once(" - "));
        let Some((subject, remainder)) = split else {
            break;
        };
        let scope = binding
            .as_ref()
            .map(|entry| entry.scope.unwrap_or(entry.entity).entity.key.as_str());
        let Some(entity) = index.resolve(label(subject), scope)? else {
            if body.contains(':') && !subject.contains('\n') {
                let known_stat = binding.as_ref().is_some_and(|entry| {
                    index
                        .stat(&entry.entity.entity.key, label(subject))
                        .is_some()
                });
                if !known_stat {
                    return Err("unknown_subject");
                }
            }
            break;
        };
        let parent = if entity.scope.is_some() {
            binding
                .as_ref()
                .map(|entry| entry.scope.unwrap_or(entry.entity))
        } else {
            None
        };
        let mut contexts = if parent.is_some() {
            binding
                .as_ref()
                .map_or_else(Vec::new, |entry| entry.contexts.clone())
        } else {
            Vec::new()
        };
        if contexts.last() != Some(&position) {
            contexts.push(position);
        }
        binding = Some(Binding {
            entity,
            scope: parent,
            contexts,
        });
        body = remainder.trim();
    }
    Ok((binding, body))
}

fn action(body: &str) -> bool {
    action_subject(body).is_some()
}

fn action_subject(body: &str) -> Option<&str> {
    action_position(body).map(|(position, _)| body[..position].trim())
}

fn action_position(body: &str) -> Option<(usize, usize)> {
    let lowercase = body.to_ascii_lowercase();
    let mut position = 0;
    for word in lowercase.split_inclusive(|ch: char| !ch.is_ascii_alphabetic()) {
        let action = word.trim_end_matches(|ch: char| !ch.is_ascii_alphabetic());
        if matches!(
            action,
            "increased"
                | "decreased"
                | "reduced"
                | "raised"
                | "lowered"
                | "fixed"
                | "added"
                | "removed"
                | "changed"
                | "adjusted"
                | "updated"
                | "reworked"
                | "renamed"
                | "now"
        ) {
            return Some((position, position + action.len()));
        }
        position += word.len();
    }
    None
}

fn qualification_word(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "when"
            | "whenever"
            | "while"
            | "whilst"
            | "every"
            | "each"
            | "if"
            | "unless"
            | "only"
            | "except"
            | "against"
            | "during"
            | "until"
            | "after"
            | "before"
            | "once"
            | "provided"
            | "assuming"
            | "given"
            | "with"
            | "without"
            | "for"
            | "on"
            | "in"
            | "at"
            | "as"
            | "per"
            | "under"
            | "upon"
            | "otherwise"
            | "instead"
            | "where"
            | "whether"
            | "not"
            | "but"
            | "and"
            | "or"
            | "however"
            | "though"
            | "although"
            | "because"
            | "since"
            | "despite"
            | "even"
            | "else"
            | "then"
            | "which"
    )
}

fn qualified_subject(subject: &str) -> bool {
    subject
        .split(|ch: char| !ch.is_ascii_alphabetic())
        .any(qualification_word)
}

fn qualified_continuation(body: &str) -> bool {
    body.split(|ch: char| !ch.is_ascii_alphabetic())
        .find(|word| !word.is_empty())
        .is_some_and(qualification_word)
}

fn structural_heading(body: &str) -> bool {
    let subject = label(body);
    valid_label(subject)
        && !qualified_subject(subject)
        && !action(subject)
        && subject.chars().any(char::is_alphabetic)
        && subject.chars().all(|ch| {
            ch.is_alphanumeric() || ch.is_whitespace() || matches!(ch, '-' | '_' | '\'' | '’')
        })
}

fn property_subject(subject: &str) -> bool {
    !subject.is_empty()
        && subject.split_whitespace().all(|word| {
            word.starts_with(|ch: char| ch.is_ascii_uppercase())
                && word
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
        })
}

fn statement_start(body: &str) -> bool {
    action_subject(body).is_some_and(|subject| {
        let subject = label(subject);
        !qualified_subject(subject)
            && (subject.is_empty()
                || property_subject(subject)
                || body
                    .split_once(':')
                    .or_else(|| body.split_once(" - "))
                    .is_some_and(|(subject, _)| structural_heading(subject)))
    })
}

fn reference_positions<'a>(body: &'a str, alias: &'a str) -> impl Iterator<Item = usize> + 'a {
    body.match_indices(alias).filter_map(move |(position, _)| {
        let before = body[..position].chars().next_back();
        let after = body[position + alias.len()..].chars().next();
        (before.is_none_or(|ch| !ch.is_alphanumeric() && ch != '_')
            && after.is_none_or(|ch| !ch.is_alphanumeric() && ch != '_'))
        .then_some(position)
    })
}

fn mentions_other_entity(body: &str, binding: &Binding<'_>, aliases: &AliasIndex) -> bool {
    aliases.aliases.iter().any(|(alias, entries)| {
        entries
            .iter()
            .any(|entry| entry.entity != binding.entity.entity)
            && reference_positions(body, alias).next().is_some()
    })
}

fn explicit_values(body: &str) -> (Option<String>, Option<String>, Option<&str>) {
    if body.contains(['\r', '\n']) {
        return (None, None, None);
    }
    let candidate = [
        " increased from ",
        " reduced from ",
        " decreased from ",
        " changed from ",
        " increased ",
        " reduced ",
        " decreased ",
    ]
    .into_iter()
    .find_map(|marker| body.split_once(marker));
    let Some((subject, values)) = candidate else {
        return (None, None, None);
    };
    if qualified_subject(subject) {
        return (None, None, None);
    }
    let Some((old, rest)) = values.split_once(" to ") else {
        return (None, None, None);
    };
    let new = rest.trim().trim_end_matches('.');
    let old = old.trim();
    if !explicit_number(old) || !explicit_number(new) {
        return (None, None, None);
    }
    (
        Some(old.to_owned()),
        Some(new.to_owned()),
        Some(subject.trim()),
    )
}

fn explicit_number(value: &str) -> bool {
    if value.is_empty() || value.len() > 64 {
        return false;
    }
    let number = value.trim_start_matches(['+', '-']);
    let digits = number
        .bytes()
        .take_while(|byte| byte.is_ascii_digit() || *byte == b'.')
        .count();
    if digits == 0 || number[..digits].parse::<f64>().is_err() {
        return false;
    }
    matches!(number[digits..].trim(), "" | "%" | "s" | "m" | "m/s" | "HP")
}

pub fn extract(post: &Post<'_>, aliases: &AliasIndex) -> Result<Extraction> {
    let mut output = Extraction::default();
    let reason = if !post.trusted {
        Some("untrusted_origin")
    } else if post.posted_at.is_none() {
        Some("missing_posted_at")
    } else if post
        .raw_content
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
    {
        Some("missing_content")
    } else {
        None
    };
    if let Some(reason) = reason {
        output.skipped.push(Skipped {
            statement_index: None,
            reason,
        });
        return Ok(output);
    }
    let content = post
        .raw_content
        .as_deref()
        .context("patch_content_missing")?;
    let rows = segments(content, aliases)?;
    let mut current: Option<Binding<'_>> = None;
    for row in &rows {
        let position = row.position;
        if section(row.raw) {
            current = None;
            continue;
        }
        if row.raw.len() > MAX_STATEMENT_BYTES {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "statement_limit",
            });
            continue;
        }
        let body = bullet(row.raw).unwrap_or(row.raw.trim());
        let heading = match aliases.resolve(
            label(body),
            current
                .as_ref()
                .map(|entry| entry.scope.unwrap_or(entry.entity).entity.key.as_str()),
        ) {
            Ok(heading) => heading,
            Err(reason) => {
                current = None;
                output.skipped.push(Skipped {
                    statement_index: Some(position),
                    reason,
                });
                continue;
            }
        };
        if let Some(entity) = heading {
            let scope = if entity.scope.is_some() {
                current
                    .as_ref()
                    .map(|entry| entry.scope.unwrap_or(entry.entity))
            } else {
                None
            };
            let mut contexts = if scope.is_some() {
                current
                    .as_ref()
                    .map_or_else(Vec::new, |entry| entry.contexts.clone())
            } else {
                Vec::new()
            };
            contexts.push(position);
            current = Some(Binding {
                entity,
                scope,
                contexts,
            });
            continue;
        }
        if bullet(row.raw).is_none() && !body.contains(':') {
            current = None;
        }
        if let Err(reason) = compound_scope(body, aliases) {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason,
            });
            continue;
        }
        let (binding, body) = match bind(body, current.as_ref(), aliases, position) {
            Ok(value) => value,
            Err(reason) => {
                current = None;
                output.skipped.push(Skipped {
                    statement_index: Some(position),
                    reason,
                });
                continue;
            }
        };
        if row.raw.lines().skip(1).any(|line| {
            !line.trim().is_empty()
                && (!line.starts_with(char::is_whitespace)
                    || !qualified_continuation(bullet(line).unwrap_or(line.trim())))
        }) {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "ambiguous_continuation",
            });
            continue;
        }
        if !action(body) {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "not_a_change",
            });
            continue;
        }
        let Some(binding) = binding else {
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "unknown_or_unscoped_entity",
            });
            continue;
        };
        if mentions_other_entity(body, &binding, aliases) {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "multiple_entity_statements",
            });
            continue;
        }
        if binding.contexts.len() > 3 {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "context_limit",
            });
            continue;
        }
        if row.raw.contains('\n') {
            current = None;
        }
        let next_heading = match heading_boundary(row.next_heading, current.as_ref(), aliases) {
            Ok(boundary) => boundary,
            Err(reason) => {
                current = None;
                output.skipped.push(Skipped {
                    statement_index: Some(position),
                    reason,
                });
                continue;
            }
        };
        let (old_value, new_value, stat_label) = explicit_values(body);
        let stat = stat_label
            .and_then(|label| aliases.stat(&binding.entity.entity.key, label))
            .map(str::to_owned);
        let change_kind = "changed";
        let condition = json!({
            "post_hash": post.post_hash, "statement_index": position,
            "next_heading": next_heading,
            "context_indices": binding.contexts, "entity_alias": binding.entity.alias,
            "scope_alias": binding.scope.map(|entry| entry.alias.as_str()),
            "scope_key": binding.scope.map(|entry| entry.entity.key.as_str()),
            "scope_kind": binding.scope.map(|entry| entry.entity.kind),
            "entity_key": binding.entity.entity.key, "entity_kind": binding.entity.entity.kind,
            "old_value": old_value, "new_value": new_value, "stat": stat,
            "stat_label": stat_label, "change_kind": change_kind,
        });
        ensure!(condition.to_string().len() <= 1024, "patch_condition_limit");
        ensure!(output.changes.len() < MAX_CHANGES, "patch_change_limit");
        output.changes.push(PatchChange {
            assertion: Assertion {
                entity: binding.entity.entity.key.clone(),
                kind: binding.entity.entity.kind,
                locator_kind: "json_pointer",
                locator: "/raw_content".into(),
                hash: post.content_hash.clone(),
                value: Atom::Text(row.raw.to_owned()),
                condition,
                alias: None,
                relation: None,
                stat: "patch.statement".into(),
                section: "patch",
                unit: None,
                order: position as i32,
            },
            old_value,
            new_value,
            stat,
            change_kind,
        });
    }
    Ok(output)
}

pub fn prove(post: &Post<'_>, aliases: &AliasIndex, assertions: &[Assertion]) -> Result<()> {
    let expected = extract(post, aliases)?;
    ensure!(
        expected.changes.len() == assertions.len(),
        "patch_proof_count"
    );
    for (expected, actual) in expected
        .changes
        .iter()
        .map(|change| &change.assertion)
        .zip(assertions)
    {
        ensure!(same_assertion(expected, actual), "patch_proof_binding");
    }
    Ok(())
}

fn same_assertion(expected: &Assertion, actual: &Assertion) -> bool {
    expected.entity == actual.entity
        && expected.kind == actual.kind
        && expected.locator_kind == actual.locator_kind
        && expected.locator == actual.locator
        && expected.hash == actual.hash
        && expected.condition == actual.condition
        && expected.alias == actual.alias
        && expected.relation == actual.relation
        && expected.stat == actual.stat
        && expected.section == actual.section
        && expected.unit == actual.unit
        && expected.order == actual.order
        && matches!((&expected.value, &actual.value), (Atom::Text(left), Atom::Text(right)) if left == right)
}

pub(crate) fn dispatch_extract(
    source: Source,
    path: &str,
    bytes: &[u8],
    identities: &BTreeMap<String, &'static str>,
) -> Result<Vec<Assertion>> {
    ensure!(supported(source, path), "patch_source_invalid");
    let post = parse_post(bytes)?;
    ensure!(object_path(post.id) == path, "patch_source_identity");
    let aliases = AliasIndex::new(
        identities
            .iter()
            .map(|(key, kind)| AliasBinding {
                alias: key.clone(),
                entity: KnownEntity {
                    key: key.clone(),
                    kind,
                },
                scope: None,
            })
            .collect(),
        Vec::new(),
    )?;
    Ok(extract(&post, &aliases)?
        .changes
        .into_iter()
        .map(|change| change.assertion)
        .collect())
}

fn replay_kind(value: &str) -> Result<&'static str> {
    match value {
        "hero" => Ok("hero"),
        "ability" => Ok("ability"),
        "item" => Ok("item"),
        "timer" => Ok("timer"),
        _ => anyhow::bail!("patch_proof_kind"),
    }
}

pub(crate) fn dispatch_prove(
    source: Source,
    path: &str,
    bytes: &[u8],
    assertions: &[Assertion],
) -> Result<()> {
    ensure!(supported(source, path), "patch_source_invalid");
    let post = parse_post(bytes)?;
    ensure!(object_path(post.id) == path, "patch_source_identity");
    ensure!(assertions.len() <= MAX_CHANGES, "patch_proof_limit");
    let mut bindings = Vec::new();
    let mut stats = Vec::new();
    let mut boundaries = BTreeMap::<usize, HeadingBoundary>::new();
    let mut statement_positions = BTreeSet::new();
    for assertion in assertions {
        ensure!(
            assertion.order >= 0 && statement_positions.insert(assertion.order),
            "patch_proof_statement"
        );
        let condition = &assertion.condition;
        ensure!(condition.to_string().len() <= 1024, "patch_condition_limit");
        if !condition["next_heading"].is_null() {
            let boundary: HeadingBoundary =
                serde_json::from_value(condition["next_heading"].clone())
                    .context("patch_proof_boundary")?;
            ensure!(
                boundary.position > assertion.order as usize
                    && boundary.position < MAX_ENVELOPE_BYTES
                    && valid_key(&boundary.key)
                    && boundary.scope.as_deref().is_none_or(valid_key),
                "patch_proof_boundary"
            );
            replay_kind(&boundary.kind)?;
            if let Some(previous) = boundaries.insert(boundary.position, boundary.clone()) {
                ensure!(previous == boundary, "patch_proof_boundary");
            }
        }
        let alias = condition["entity_alias"]
            .as_str()
            .context("patch_proof_alias")?;
        ensure!(
            condition["entity_key"].as_str() == Some(assertion.entity.as_str())
                && condition["entity_kind"].as_str() == Some(assertion.kind),
            "patch_proof_entity"
        );
        let scope = condition["scope_alias"]
            .as_str()
            .map(|alias| {
                let key = condition["scope_key"]
                    .as_str()
                    .context("patch_proof_scope")?;
                let kind = replay_kind(
                    condition["scope_kind"]
                        .as_str()
                        .context("patch_proof_scope")?,
                )?;
                bindings.push(AliasBinding {
                    alias: alias.to_owned(),
                    entity: KnownEntity {
                        key: key.to_owned(),
                        kind,
                    },
                    scope: None,
                });
                Ok::<_, anyhow::Error>(key.to_owned())
            })
            .transpose()?;
        bindings.push(AliasBinding {
            alias: alias.to_owned(),
            entity: KnownEntity {
                key: assertion.entity.clone(),
                kind: assertion.kind,
            },
            scope,
        });
        if let Some(stat) = condition["stat"].as_str() {
            stats.push(StatBinding {
                entity: assertion.entity.clone(),
                label: condition["stat_label"]
                    .as_str()
                    .context("patch_proof_stat")?
                    .to_owned(),
                stat: stat.to_owned(),
            });
        }
    }
    let mut seen_boundaries = 0;
    for (position, line) in post
        .raw_content()
        .unwrap_or_default()
        .split_inclusive('\n')
        .enumerate()
    {
        let Some(boundary) = boundaries.get(&position) else {
            continue;
        };
        let text = line.trim_end_matches(['\r', '\n']);
        let body = bullet(text).unwrap_or(text.trim());
        ensure!(
            structural_heading(body) && !section(body),
            "patch_proof_boundary"
        );
        bindings.push(AliasBinding {
            alias: label(body).to_owned(),
            entity: KnownEntity {
                key: boundary.key.clone(),
                kind: replay_kind(&boundary.kind)?,
            },
            scope: boundary.scope.clone(),
        });
        seen_boundaries += 1;
    }
    ensure!(seen_boundaries == boundaries.len(), "patch_proof_boundary");
    let mut entity_kinds = BTreeMap::new();
    for binding in &bindings {
        if let Some(kind) = entity_kinds.insert(&binding.entity.key, binding.entity.kind) {
            ensure!(kind == binding.entity.kind, "patch_proof_entity");
        }
    }
    let aliases = AliasIndex::new(bindings, stats)?;
    let expected = extract(&post, &aliases)?;
    let expected: BTreeMap<_, _> = expected
        .changes
        .iter()
        .map(|change| (change.assertion.order, &change.assertion))
        .collect();
    for assertion in assertions {
        let expected = expected
            .get(&assertion.order)
            .context("patch_proof_statement")?;
        ensure!(same_assertion(expected, assertion), "patch_proof_receipt");
    }
    Ok(())
}
