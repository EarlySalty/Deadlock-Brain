use crate::{
    extract::{Assertion, Atom},
    git::Source,
};
use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, value::RawValue};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

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
                    && matches!(binding.entity.kind, "hero" | "ability" | "item")
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

#[derive(Clone, Copy)]
struct Segment<'a> {
    raw: &'a str,
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

fn segments(content: &str) -> Result<Vec<Segment<'_>>> {
    let mut output = Vec::new();
    let mut start = 0;
    let mut pending: Option<usize> = None;
    for line in content.split_inclusive('\n') {
        let text = line.trim_end_matches(['\r', '\n']);
        let continuation = text.starts_with(char::is_whitespace)
            || text.trim_start().starts_with("only ")
            || text.trim_start().starts_with("Only ");
        let heading = !continuation && bullet(text).is_none();
        if text.trim().is_empty() || bullet(text).is_some() || heading {
            if let Some(begin) = pending.take() {
                output.push(Segment {
                    raw: content[begin..start].trim_end_matches(['\r', '\n']),
                });
            }
            if bullet(text).is_some() {
                pending = Some(start);
            } else if !text.trim().is_empty() {
                output.push(Segment { raw: text });
            }
        } else if pending.is_none() {
            output.push(Segment { raw: text });
        }
        start += line.len();
        ensure!(output.len() <= MAX_SEGMENTS, "patch_segment_limit");
    }
    if let Some(begin) = pending {
        output.push(Segment {
            raw: content[begin..].trim_end_matches(['\r', '\n']),
        });
    }
    ensure!(output.len() <= MAX_SEGMENTS, "patch_segment_limit");
    Ok(output)
}

struct Binding<'a> {
    entity: &'a AliasBinding,
    scope: Option<&'a AliasBinding>,
    contexts: Vec<usize>,
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
    let body = body.to_ascii_lowercase();
    body.split(|ch: char| !ch.is_ascii_alphabetic())
        .any(|word| {
            matches!(
                word,
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
            )
        })
}

fn explicit_values(body: &str) -> (Option<String>, Option<String>, Option<&str>) {
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
    let Some((old, rest)) = values.split_once(" to ") else {
        return (None, None, None);
    };
    let new = rest
        .split([',', ';', '\n'])
        .next()
        .unwrap_or(rest)
        .split(" - ")
        .next()
        .unwrap_or(rest)
        .trim()
        .trim_end_matches('.');
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
    let rows = segments(content)?;
    let mut current: Option<Binding<'_>> = None;
    for (position, row) in rows.iter().enumerate() {
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
        if body.split(" - ").skip(1).any(|part| {
            part.split_once(':')
                .is_some_and(|(subject, _)| aliases.aliases.contains_key(label(subject)))
        }) {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "multiple_entity_statements",
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
        if binding.contexts.len() > 3 {
            current = None;
            output.skipped.push(Skipped {
                statement_index: Some(position),
                reason: "context_limit",
            });
            continue;
        }
        let (old_value, new_value, stat_label) = explicit_values(body);
        let stat = stat_label
            .and_then(|label| aliases.stat(&binding.entity.entity.key, label))
            .map(str::to_owned);
        let change_kind = "changed";
        let condition = json!({
            "post_hash": post.post_hash, "statement_index": position,
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
    for assertion in assertions {
        let condition = &assertion.condition;
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
                let kind = match condition["scope_kind"].as_str() {
                    Some("hero") => "hero",
                    Some("item") => "item",
                    _ => anyhow::bail!("patch_proof_scope"),
                };
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
