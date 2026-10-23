#[path = "patch_source.rs"]
pub mod source;

use crate::{
    limits, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt, Rejection,
};
use brain_storage::gamefacts::receipts;
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use serde::Deserialize;
use serde_json::value::RawValue;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

pub const SOURCE: &str = "deadlock_patchnotes_db";

#[derive(Debug, Default)]
pub struct PatchExtractor;

struct Fields<'a>(BTreeMap<String, &'a RawValue>, bool);
impl<'de> Deserialize<'de> for Fields<'de> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Fields<'de>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("patch envelope")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> Result<Self::Value, M::Error> {
                let mut fields = Fields(BTreeMap::new(), false);
                let mut seen = BTreeSet::new();
                while let Some((key, value)) = map.next_entry::<String, &'de RawValue>()? {
                    fields.1 |= !seen.insert(limits::normalize_key(&key));
                    fields.0.insert(key, value);
                }
                Ok(fields)
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

#[derive(Clone, Copy)]
struct Segment<'a> {
    raw: &'a str,
    position: usize,
}

impl PatchExtractor {
    fn parse(&self, path: &str, bytes: &[u8]) -> Result<Option<FactSet>, &'static str> {
        let Fields(fields, collision) =
            serde_json::from_slice(bytes).map_err(|_| "envelope_invalid")?;
        if collision {
            return Err("key_collision");
        }
        if fields.len() != 5 {
            return Err("envelope_invalid");
        }
        let field = |key: &str| {
            fields
                .get(key)
                .map(|value| value.get())
                .ok_or("envelope_invalid")
        };
        let id: i64 = serde_json::from_str(field("id")?).map_err(|_| "post_id_invalid")?;
        if id.to_string() != path {
            return Err("source_identity");
        }
        let title: String = serde_json::from_str(field("title")?).map_err(|_| "title_invalid")?;
        let url: String = serde_json::from_str(field("url")?).map_err(|_| "url_invalid")?;
        let origin = origin(&url).ok_or("untrusted_origin")?;
        if title.trim().is_empty() {
            return Err("untrusted_origin");
        }
        let thread_day = match origin {
            Origin::Announcement => None,
            Origin::ForumPost { thread_slug, .. } => {
                Some(update_thread_day(&title, thread_slug).ok_or("untrusted_origin")?)
            }
        };
        let date: Option<String> =
            serde_json::from_str(field("posted_at")?).map_err(|_| "date_invalid")?;
        let content: Option<String> =
            serde_json::from_str(field("raw_content")?).map_err(|_| "content_invalid")?;
        let Some(date) = date else { return Ok(None) };
        let posted = DateTime::parse_from_rfc3339(&date)
            .map_err(|_| "date_invalid")?
            .with_timezone(&Utc);
        let (title, url) = match (origin, thread_day) {
            (Origin::ForumPost { post_id, .. }, Some(day)) => {
                forum_post_identity(&title, post_id, day, posted)?
            }
            _ => (title, url),
        };
        let Some(content) = content.filter(|text| !text.trim().is_empty()) else {
            return Ok(None);
        };
        let key = format!("patch_{id}");
        let receipt = |name: &str| -> Result<Receipt, &'static str> {
            Ok(Receipt {
                locator_kind: "json_pointer".into(),
                locator: format!("/{name}"),
                value_sha256: Sha256::digest(field(name)?.as_bytes()).into(),
            })
        };
        let mut set = FactSet::default();
        set.entities.push(Entity {
            kind: "patch".into(),
            key: key.clone(),
            receipt: Receipt {
                locator_kind: "raw_bytes".into(),
                locator: String::new(),
                value_sha256: Sha256::digest(bytes).into(),
            },
        });
        for (stat_key, name, value) in [
            ("label", "title", title),
            ("released_at", "posted_at", date),
            ("official_url", "url", url),
        ] {
            set.facts.push(Fact {
                entity_key: key.clone(),
                stat_key: stat_key.into(),
                value: FactValue::Text(value),
                receipt: receipt(name)?,
            });
        }
        let mut current = None;
        let mut enabled = !content
            .lines()
            .any(|line| line.trim_start().starts_with("## "));
        for row in self.segments(&content) {
            let body = bullet(row.raw).unwrap_or(row.raw.trim());
            if section(body) || row.raw.starts_with("## ") {
                enabled = label(body) == "Heroes";
                current = None;
                continue;
            }
            if !enabled {
                continue;
            }
            if bullet(row.raw).is_none() && structural_heading(body) {
                current = Some((label(body), row.position));
                continue;
            }
            if let Some(change) = self.change(row, current) {
                let reference = change["reference"].as_str().ok_or("reference_invalid")?;
                let fingerprint = Sha256::digest(reference.as_bytes());
                set.facts.push(Fact {
                    entity_key: key.clone(),
                    stat_key: format!("patch_change_r{fingerprint:x}_{}", row.position),
                    value: FactValue::Text(change.to_string()),
                    receipt: receipt("raw_content")?,
                });
                if row.raw.contains('\n') {
                    current = None;
                }
            } else {
                current = None;
            }
        }
        Ok(Some(set))
    }

    fn segments<'a>(&self, content: &'a str) -> Vec<Segment<'a>> {
        let mut output = Vec::new();
        let mut offset = 0;
        let mut pending: Option<(usize, usize, usize)> = None;
        for (position, line) in content.split_inclusive('\n').enumerate() {
            let text = line.trim_end_matches(['\r', '\n']);
            let depth = text.len() - text.trim_start().len();
            let body = bullet(text).unwrap_or(text.trim());
            let known_heading = structural_heading(body);
            let heading = depth == 0
                && bullet(text).is_none()
                && (section(body) || known_heading || text.starts_with("## "));
            let new_bullet = bullet(text).is_some()
                && pending.is_none_or(|(_, previous, _)| {
                    depth <= previous && (statement_start(body) || section(body) || known_heading)
                });
            if heading || new_bullet {
                if let Some((begin, _, position)) = pending.take() {
                    output.push(Segment {
                        raw: content[begin..offset].trim_end_matches(['\r', '\n']),
                        position,
                    });
                }
                if heading {
                    output.push(Segment {
                        raw: text,
                        position,
                    });
                } else {
                    pending = Some((offset, depth, position));
                }
            } else if pending.is_none() && !text.trim().is_empty() {
                pending = Some((offset, depth, position));
            }
            offset += line.len();
        }
        if let Some((begin, _, position)) = pending {
            output.push(Segment {
                raw: content[begin..].trim_end_matches(['\r', '\n']),
                position,
            });
        }
        output
    }

    fn change(
        &self,
        row: Segment<'_>,
        current: Option<(&str, usize)>,
    ) -> Option<serde_json::Value> {
        let mut body = bullet(row.raw)?;
        let mut binding = current;
        if let Some((subject, rest)) = body.split_once(':').or_else(|| body.split_once(" - ")) {
            if !structural_heading(subject) {
                return None;
            }
            binding = Some((label(subject), row.position));
            body = rest.trim();
        }
        let (subject, context) = binding.filter(|_| action_subject(body).is_some())?;
        if row.raw.lines().skip(1).any(|line| {
            !line.trim().is_empty()
                && (!line.starts_with(char::is_whitespace)
                    || !first_word(bullet(line).unwrap_or(line.trim()))
                        .is_some_and(qualification_word))
        }) {
            return None;
        }
        let lower = body.to_lowercase();
        if lower.contains(" against ") && !lower.contains("only against ") {
            return None;
        }
        let mut clauses = body
            .split(['\n', ';'])
            .flat_map(|part| part.split(" - "))
            .flat_map(|part| part.split(" and "))
            .flat_map(|part| part.split(" & "))
            .flat_map(|part| part.split(" / "))
            .flat_map(|part| part.split(". "))
            .flat_map(|part| part.split(", "));
        clauses.next();
        if clauses.any(|part| {
            part.contains(':')
                || action_subject(bullet(part).unwrap_or(part.trim()))
                    .is_some_and(|subject| !qualified(subject))
        }) {
            return None;
        }
        let values = explicit_values(body);
        Some(serde_json::json!({
            "reference": limits::normalize_alias(subject), "subject": subject,
            "statement_index": row.position, "context_index": (context != row.position).then_some(context),
            "stat_label": values.map(|v| v.2), "old": values.map(|v| v.0), "new": values.map(|v| v.1), "line_text": row.raw,
        }))
    }
}

impl Extractor for PatchExtractor {
    fn name(&self) -> &'static str {
        "patchnotes"
    }
    fn version(&self) -> &'static str {
        "2"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == SOURCE
            && path
                .parse::<i64>()
                .ok()
                .is_some_and(|id| id > 0 && id.to_string() == path)
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        match self.parse(&raw.path, &raw.bytes) {
            Ok(Some(set)) => {
                match limits::validate(&set).and_then(|()| receipts::verify(&raw.bytes, &set)) {
                    Ok(()) => Outcome::Accepted(set),
                    Err(rejection) => Outcome::Rejected(rejection),
                }
            }
            Ok(None) => Outcome::Ignored,
            Err(code) => Outcome::Rejected(Rejection::new(
                code,
                "Patchdaten entsprechen nicht dem Quellenvertrag",
            )),
        }
    }
}
const FORUM: &str = "https://forums.playdeadlock.com";
const PRIMARY_POST_WINDOW_HOURS: i64 = 32;

#[derive(Clone, Copy)]
enum Origin<'a> {
    Announcement,
    ForumPost {
        post_id: &'a str,
        thread_slug: Option<&'a str>,
    },
}

fn origin(value: &str) -> Option<Origin<'_>> {
    if let Some(post) = forum_post(value) {
        return Some(post);
    }
    let value = value.strip_suffix('/').unwrap_or(value);
    for prefix in [
        "https://store.steampowered.com/news/app/1422450/view/",
        "https://steamcommunity.com/games/1422450/announcements/detail/",
    ] {
        if let Some(id) = value.strip_prefix(prefix) {
            return decimal_id(id).then_some(Origin::Announcement);
        }
    }
    value
        .strip_prefix("https://forums.playdeadlock.com/threads/")
        .and_then(thread_slug)
        .map(|_| Origin::Announcement)
}

fn forum_post(value: &str) -> Option<Origin<'_>> {
    if let Some(id) = value.strip_prefix("https://forums.playdeadlock.com/posts/") {
        let id = id.strip_suffix('/').unwrap_or(id);
        return decimal_id(id).then_some(Origin::ForumPost {
            post_id: id,
            thread_slug: None,
        });
    }
    let (thread, post_id) = value
        .strip_prefix("https://forums.playdeadlock.com/threads/")?
        .split_once("#post-")?;
    let slug = thread_slug(thread.strip_suffix('/').unwrap_or(thread))?;
    decimal_id(post_id).then_some(Origin::ForumPost {
        post_id,
        thread_slug: Some(slug),
    })
}

fn thread_slug(thread: &str) -> Option<&str> {
    let (slug, id) = thread.rsplit_once('.')?;
    (!slug.is_empty()
        && slug
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'-')
        && decimal_id(id))
    .then_some(slug)
}

fn update_thread_day(title: &str, slug: Option<&str>) -> Option<NaiveDate> {
    let (date, suffix) = title.split_at_checked(10)?;
    let numbered = suffix
        .strip_prefix(" Update")
        .and_then(|rest| rest.strip_prefix(' '))
        .is_some_and(|number| decimal_id(number) && number.len() <= 2);
    let shape = date.bytes().enumerate().all(|(index, byte)| match index {
        2 | 5 => byte == b'-',
        _ => byte.is_ascii_digit(),
    });
    if !shape || !(suffix.is_empty() || suffix == " Update" || numbered) {
        return None;
    }
    if slug.is_some_and(|slug| slug != title.to_ascii_lowercase().replace(' ', "-")) {
        return None;
    }
    NaiveDate::parse_from_str(date, "%m-%d-%Y").ok()
}

fn forum_post_identity(
    title: &str,
    post_id: &str,
    day: NaiveDate,
    posted: DateTime<Utc>,
) -> Result<(String, String), &'static str> {
    let opened = day.and_time(chrono::NaiveTime::MIN).and_utc();
    if posted < opened {
        return Err("untrusted_origin");
    }
    let url = format!("{FORUM}/posts/{post_id}/");
    if posted < opened + TimeDelta::hours(PRIMARY_POST_WINDOW_HOURS) {
        return Ok((title.to_owned(), url));
    }
    let label = format!(
        "{title} Hotfix {} (Post {post_id})",
        posted.format("%Y-%m-%d")
    );
    Ok((label, url))
}

fn decimal_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.bytes().any(|byte| byte != b'0')
}

fn bullet(value: &str) -> Option<&str> {
    ["- ", "* ", "• "]
        .into_iter()
        .find_map(|prefix| value.trim().strip_prefix(prefix))
}

fn label(value: &str) -> &str {
    value.trim_matches(|ch: char| ch.is_whitespace() || "#*:[]`".contains(ch))
}

fn section(value: &str) -> bool {
    "Heroes|Items|General|New Heroes|Map|UI|Audio|Misc|Street Brawl"
        .split('|')
        .any(|name| name == label(value))
}

fn action_subject(body: &str) -> Option<&str> {
    let mut position = 0;
    for token in body.split_inclusive(|ch: char| !ch.is_ascii_alphabetic()) {
        let word = token.trim_end_matches(|ch: char| !ch.is_ascii_alphabetic());
        if "increased decreased reduced raised lowered fixed added removed changed adjusted updated reworked renamed now"
            .split_whitespace().any(|action| word.eq_ignore_ascii_case(action)) {
            return Some(body[..position].trim());
        }
        position += token.len();
    }
    None
}

fn qualification_word(word: &str) -> bool {
    "when whenever while whilst every each if unless only except against during until after before once provided assuming given with without for on in at as per under upon otherwise instead where whether not but and or however though although because since despite even else then which"
        .split_whitespace().any(|qualifier| word.eq_ignore_ascii_case(qualifier))
}

fn first_word(body: &str) -> Option<&str> {
    body.split(|ch: char| !ch.is_ascii_alphabetic())
        .find(|word| !word.is_empty())
}

fn qualified(body: &str) -> bool {
    body.split(|ch: char| !ch.is_ascii_alphabetic())
        .any(qualification_word)
}

fn structural_heading(body: &str) -> bool {
    let subject = label(body);
    !subject.is_empty()
        && !qualified(subject)
        && action_subject(subject).is_none()
        && subject.chars().any(char::is_alphabetic)
        && subject
            .chars()
            .all(|ch| ch.is_alphanumeric() || ch.is_whitespace() || "-_'’".contains(ch))
}

fn statement_start(body: &str) -> bool {
    action_subject(body).is_some_and(|subject| {
        let subject = label(subject);
        !qualified(subject)
            && (subject.is_empty()
                || subject.split_whitespace().all(|word| {
                    word.starts_with(|ch: char| ch.is_ascii_uppercase())
                        && word
                            .chars()
                            .all(|ch| ch.is_ascii_alphanumeric() || "-_".contains(ch))
                })
                || body
                    .split_once(':')
                    .or_else(|| body.split_once(" - "))
                    .is_some_and(|(name, _)| structural_heading(name)))
    })
}

fn explicit_values(body: &str) -> Option<(&str, &str, &str)> {
    if body.contains(['\r', '\n']) {
        return None;
    }
    let (subject, values) = [" increased ", " reduced ", " decreased ", " changed from "]
        .into_iter()
        .find_map(|marker| body.split_once(marker))
        .filter(|(subject, _)| !qualified(subject))?;
    let (old, new) = values
        .strip_prefix("from ")
        .unwrap_or(values)
        .split_once(" to ")?;
    let old = old.trim();
    let new = new.trim().strip_suffix('.').unwrap_or(new.trim());
    (explicit_number(old) && explicit_number(new)).then_some((old, new, subject.trim()))
}

fn explicit_number(value: &str) -> bool {
    let number = value.strip_prefix(['+', '-']).unwrap_or(value);
    let end = number
        .bytes()
        .take_while(|byte| byte.is_ascii_digit() || *byte == b'.')
        .count();
    let digits = &number[..end];
    digits.bytes().any(|byte| byte.is_ascii_digit())
        && digits.bytes().filter(|byte| *byte == b'.').count() <= 1
        && matches!(number[end..].trim(), "" | "%" | "s" | "m" | "m/s" | "HP")
}
