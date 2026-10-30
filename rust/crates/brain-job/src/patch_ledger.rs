use chrono::DateTime;
use sqlx::PgPool;
use std::collections::{BTreeMap, BTreeSet};

const WINDOW_MICROS: i64 = 36 * 3_600_000_000;
const CONTENT_WINDOW_MICROS: i64 = 7 * 24 * 3_600_000_000;
const CONTENT_MIN_LINES: usize = 3;
const MIRROR_MIN_LINES: usize = 2;
const SAME_TITLE_MICROS: i64 = 3_600_000_000;
const MAJOR_CHARS: usize = 15_000;
const GAMEPLAY_CHARS: usize = 1_500;
const PATCH_LINES: usize = 20;

#[derive(Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub posted_micros: i64,
    pub raw_content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerPatch {
    pub ledger_id: i64,
    pub dated_post_id: i64,
    pub released_micros: i64,
    pub kind: String,
    pub label: String,
    pub source_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerPost {
    pub post_id: i64,
    pub ledger_id: Option<i64>,
    pub disposition: String,
    pub reason: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Ledger {
    pub patches: Vec<LedgerPatch>,
    pub posts: Vec<LedgerPost>,
}

struct Prepared<'a> {
    post: &'a Post,
    lines: BTreeSet<String>,
    chars: usize,
    steam: bool,
}

fn change_lines(text: &str) -> BTreeSet<String> {
    text.lines()
        .map(str::trim)
        .filter_map(|line| {
            let mut chars = line.chars();
            matches!(chars.next(), Some('-' | '*' | '•')).then(|| chars.as_str())
        })
        .map(|rest| {
            rest.to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect()
}

fn word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn has_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(start, found)| {
        let before = text[..start].chars().next_back();
        let after = text[start + found.len()..].chars().next();
        !before.is_some_and(word_char) && !after.is_some_and(word_char)
    })
}

fn has_title_date(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let shape = "dd-dd-dddd";
    chars.windows(shape.len()).any(|window| {
        window
            .iter()
            .zip(shape.chars())
            .all(|(ch, want)| match want {
                'd' => ch.is_ascii_digit(),
                _ => *ch == want,
            })
    })
}

fn is_patch(post: &Prepared) -> bool {
    let title = post.post.title.trim().to_lowercase();
    has_word(&title, "update")
        || has_word(&title, "patch")
        || has_title_date(&title)
        || post.lines.len() >= PATCH_LINES
}

fn head(text: &str) -> String {
    text.chars().take(300).collect()
}

fn stub_of(stub: &Prepared, other: &Prepared) -> bool {
    let start = head(&stub.post.raw_content);
    start.contains("Steam News")
        && start
            .to_lowercase()
            .contains(&other.post.title.trim().to_lowercase())
}

fn same_notes(a: &Prepared, b: &Prepared, min_lines: usize) -> bool {
    let (small, large) = if a.lines.len() <= b.lines.len() {
        (&a.lines, &b.lines)
    } else {
        (&b.lines, &a.lines)
    };
    let shared = small.intersection(large).count();
    small.len() >= min_lines.max(MIRROR_MIN_LINES)
        && shared * 5 >= small.len() * 4
        && shared * 2 >= large.len()
}

fn duplicate(a: &Prepared, b: &Prepared) -> bool {
    let gap = (a.post.posted_micros - b.post.posted_micros).abs();
    if gap > WINDOW_MICROS {
        return gap <= CONTENT_WINDOW_MICROS && same_notes(a, b, CONTENT_MIN_LINES);
    }
    if stub_of(a, b) || stub_of(b, a) {
        return true;
    }
    if gap <= SAME_TITLE_MICROS
        && a.post.title.trim().to_lowercase() == b.post.title.trim().to_lowercase()
    {
        return true;
    }
    if !a.post.raw_content.is_empty() && a.post.raw_content.trim() == b.post.raw_content.trim() {
        return true;
    }
    same_notes(a, b, MIRROR_MIN_LINES)
}

fn root(parent: &mut [usize], mut node: usize) -> usize {
    while parent[node] != node {
        parent[node] = parent[parent[node]];
        node = parent[node];
    }
    node
}

fn groups(prepared: &[Prepared]) -> Vec<Vec<usize>> {
    let mut order: Vec<usize> = (0..prepared.len()).collect();
    order.sort_by_key(|&index| (prepared[index].post.posted_micros, prepared[index].post.id));
    let mut parent: Vec<usize> = (0..prepared.len()).collect();
    for (position, &a) in order.iter().enumerate() {
        for &b in &order[position + 1..] {
            if prepared[b].post.posted_micros - prepared[a].post.posted_micros
                > CONTENT_WINDOW_MICROS
            {
                break;
            }
            if duplicate(&prepared[a], &prepared[b]) {
                let (left, right) = (root(&mut parent, a), root(&mut parent, b));
                parent[left] = right;
            }
        }
    }
    let mut grouped: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for index in 0..prepared.len() {
        let group = root(&mut parent, index);
        grouped.entry(group).or_default().push(index);
    }
    grouped.into_values().collect()
}

fn rank(
    post: &Prepared,
) -> (
    bool,
    std::cmp::Reverse<usize>,
    std::cmp::Reverse<usize>,
    i64,
) {
    (
        !(post.steam && !post.lines.is_empty()),
        std::cmp::Reverse(post.lines.len()),
        std::cmp::Reverse(post.chars),
        post.post.id,
    )
}

fn kind(chars: usize) -> &'static str {
    if chars >= MAJOR_CHARS {
        "major"
    } else if chars >= GAMEPLAY_CHARS {
        "gameplay"
    } else {
        "hotfix"
    }
}

fn utc_date(micros: i64) -> String {
    DateTime::from_timestamp_micros(micros)
        .map(|time| time.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| micros.to_string())
}

fn prepare(post: &Post) -> Prepared<'_> {
    Prepared {
        post,
        lines: change_lines(&post.raw_content),
        chars: post.raw_content.chars().count(),
        steam: post.url.contains("steam"),
    }
}

pub fn classify(posts: &[Post]) -> Ledger {
    let prepared: Vec<Prepared> = posts.iter().map(prepare).collect();
    let mut ledger = Ledger::default();
    let mut entries = Vec::new();
    for group in groups(&prepared) {
        let Some(&primary) = group.iter().min_by_key(|&&index| rank(&prepared[index])) else {
            continue;
        };
        if !group.iter().any(|&index| is_patch(&prepared[index])) {
            for &index in &group {
                ledger.posts.push(LedgerPost {
                    post_id: prepared[index].post.id,
                    ledger_id: None,
                    disposition: "excluded".into(),
                    reason: "not a patch: title has no update, patch or date and fewer than 20 change lines".into(),
                });
            }
            continue;
        }
        let Some(&dated) = group
            .iter()
            .min_by_key(|&&index| (prepared[index].post.posted_micros, prepared[index].post.id))
        else {
            continue;
        };
        let primary_post = prepared[primary].post;
        for &index in &group {
            let post = prepared[index].post;
            let is_primary = index == primary;
            ledger.posts.push(LedgerPost {
                post_id: post.id,
                ledger_id: Some(primary_post.id),
                disposition: if is_primary { "primary" } else { "duplicate" }.into(),
                reason: if is_primary {
                    "primary source".into()
                } else {
                    format!("duplicate of post {}", primary_post.id)
                },
            });
        }
        entries.push(LedgerPatch {
            ledger_id: primary_post.id,
            dated_post_id: prepared[dated].post.id,
            released_micros: prepared[dated].post.posted_micros,
            kind: kind(prepared[primary].chars).into(),
            label: primary_post.title.trim().to_owned(),
            source_url: primary_post.url.clone(),
        });
    }
    entries.sort_by_key(|patch| (patch.released_micros, patch.ledger_id));
    let mut labels = BTreeSet::new();
    for patch in &mut entries {
        if !labels.insert(patch.label.clone()) {
            patch.label = format!("{} ({})", patch.label, utc_date(patch.released_micros));
            if !labels.insert(patch.label.clone()) {
                patch.label = format!("{} #{}", patch.label, patch.ledger_id);
                labels.insert(patch.label.clone());
            }
        }
    }
    ledger.patches = entries;
    ledger.posts.sort_by_key(|post| post.post_id);
    ledger
}

type PostRow = (i64, String, String, i64, String);
type PatchRow = (i64, i64, i64, String, String, String);
type MemberRow = (i64, Option<i64>, String, String);

async fn load_posts(transaction: &mut sqlx::PgConnection) -> anyhow::Result<Vec<Post>> {
    let rows: Vec<PostRow> = sqlx::query_as(
        "SELECT post_id,title,url,(extract(epoch FROM published_at)*1000000)::bigint,raw_content
         FROM ops.patch_ledger_source() ORDER BY post_id",
    )
    .fetch_all(&mut *transaction)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, title, url, posted_micros, raw_content)| Post {
            id,
            title,
            url,
            posted_micros,
            raw_content,
        })
        .collect())
}

async fn load_ledger(transaction: &mut sqlx::PgConnection) -> anyhow::Result<Ledger> {
    let patches: Vec<PatchRow> = sqlx::query_as(
        "SELECT ledger_id,dated_post_id,(extract(epoch FROM released_at)*1000000)::bigint,
            kind,label,source_url
         FROM ops.patch_ledger ORDER BY released_at,ledger_id",
    )
    .fetch_all(&mut *transaction)
    .await?;
    let posts: Vec<MemberRow> = sqlx::query_as(
        "SELECT post_id,ledger_id,disposition,reason FROM ops.patch_ledger_post ORDER BY post_id",
    )
    .fetch_all(&mut *transaction)
    .await?;
    Ok(Ledger {
        patches: patches
            .into_iter()
            .map(
                |(ledger_id, dated_post_id, released_micros, kind, label, source_url)| {
                    LedgerPatch {
                        ledger_id,
                        dated_post_id,
                        released_micros,
                        kind,
                        label,
                        source_url,
                    }
                },
            )
            .collect(),
        posts: posts
            .into_iter()
            .map(|(post_id, ledger_id, disposition, reason)| LedgerPost {
                post_id,
                ledger_id,
                disposition,
                reason,
            })
            .collect(),
    })
}

async fn replace(
    transaction: &mut sqlx::PgConnection,
    run_id: i64,
    ledger: &Ledger,
) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM ops.patch_ledger_post")
        .execute(&mut *transaction)
        .await?;
    sqlx::query("DELETE FROM ops.patch_ledger")
        .execute(&mut *transaction)
        .await?;
    let patches = &ledger.patches;
    sqlx::query(
        "INSERT INTO ops.patch_ledger
            (ledger_id,dated_post_id,released_at,kind,label,source_url,run_id)
         SELECT l,d,timestamptz 'epoch'+r*interval '1 microsecond',k,n,u,$7
         FROM unnest($1::bigint[],$2::bigint[],$3::bigint[],$4::text[],$5::text[],$6::text[])
            AS s(l,d,r,k,n,u)",
    )
    .bind(patches.iter().map(|p| p.ledger_id).collect::<Vec<_>>())
    .bind(patches.iter().map(|p| p.dated_post_id).collect::<Vec<_>>())
    .bind(
        patches
            .iter()
            .map(|p| p.released_micros)
            .collect::<Vec<_>>(),
    )
    .bind(patches.iter().map(|p| p.kind.clone()).collect::<Vec<_>>())
    .bind(patches.iter().map(|p| p.label.clone()).collect::<Vec<_>>())
    .bind(
        patches
            .iter()
            .map(|p| p.source_url.clone())
            .collect::<Vec<_>>(),
    )
    .bind(run_id)
    .execute(&mut *transaction)
    .await?;
    let posts = &ledger.posts;
    sqlx::query(
        "INSERT INTO ops.patch_ledger_post (post_id,ledger_id,disposition,reason,run_id)
         SELECT p,l,d,r,$5 FROM unnest($1::bigint[],$2::bigint[],$3::text[],$4::text[]) AS s(p,l,d,r)",
    )
    .bind(posts.iter().map(|p| p.post_id).collect::<Vec<_>>())
    .bind(posts.iter().map(|p| p.ledger_id).collect::<Vec<_>>())
    .bind(posts.iter().map(|p| p.disposition.clone()).collect::<Vec<_>>())
    .bind(posts.iter().map(|p| p.reason.clone()).collect::<Vec<_>>())
    .bind(run_id)
    .execute(&mut *transaction)
    .await?;
    crate::patch_extract::line::sync(transaction, run_id).await?;
    Ok(())
}

pub async fn refresh(pool: &PgPool, run_id: i64) -> anyhow::Result<bool> {
    let mut transaction = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext('ops.patch_ledger'))")
        .execute(&mut *transaction)
        .await?;
    let ledger = classify(&load_posts(&mut transaction).await?);
    if load_ledger(&mut transaction).await? == ledger {
        transaction.rollback().await?;
        return Ok(false);
    }
    replace(&mut transaction, run_id, &ledger).await?;
    transaction.commit().await?;
    Ok(true)
}

#[cfg(test)]
#[path = "patch_ledger_tests.rs"]
mod tests;
