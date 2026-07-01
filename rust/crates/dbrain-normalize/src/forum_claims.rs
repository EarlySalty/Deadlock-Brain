use std::collections::BTreeMap;

use rusqlite::{params, Connection};
use serde_json::{json, Value};

use crate::{
    util::{json_string, normalize_scan_text, stable_hash_text},
    Result,
};

const FORUM_SOURCE: &str = "playdeadlock_forum";

#[derive(Debug)]
struct ForumPost {
    snapshot_id: i64,
    thread_id: String,
    post_id: String,
    thread_title: Option<String>,
    thread_url: Option<String>,
    posted_at: Option<String>,
    author: Option<String>,
    author_role: Option<String>,
    text: String,
}

#[derive(Debug)]
struct EntityHit {
    entity_type: String,
    canonical_name: String,
    alias: String,
}

#[derive(Debug)]
struct ClaimCandidate {
    claim_type: &'static str,
    source_trust: &'static str,
    validity_status: &'static str,
    currentness: &'static str,
    confidence: f64,
    safety_labels: Vec<&'static str>,
    claim_text: String,
    evidence_quote: String,
}

pub fn parse_forum_claims(conn: &Connection, rebuild: bool) -> Result<Value> {
    if rebuild {
        conn.execute("DELETE FROM forum_claims", [])?;
    }

    let entity_aliases = load_entity_aliases(conn)?;
    let posts = load_forum_posts(conn)?;
    let mut posts_seen = 0usize;
    let mut inserted = 0usize;
    let mut skipped_duplicates = 0usize;
    let mut by_type = BTreeMap::<String, usize>::new();
    let mut by_validity = BTreeMap::<String, usize>::new();

    for post in posts {
        posts_seen += 1;
        let mentioned_entities = detect_entities(&entity_aliases, &post);
        let candidates = claim_candidates(&post);
        for (claim_index, candidate) in candidates.into_iter().enumerate() {
            let source_url = source_url(&post);
            let (entity_type, entity_name) = mentioned_entities
                .first()
                .map(|entity| {
                    (
                        Some(entity.entity_type.as_str()),
                        Some(entity.canonical_name.as_str()),
                    )
                })
                .unwrap_or((None, None));
            let source_references = json!([{
                "source": FORUM_SOURCE,
                "url": source_url,
                "thread_id": post.thread_id,
                "post_id": post.post_id,
                "thread_title": post.thread_title,
                "posted_at": post.posted_at,
            }]);
            let metadata = json!({
                "mentioned_entities": mentioned_entities.iter().map(|entity| json!({
                    "entity_type": entity.entity_type,
                    "canonical_name": entity.canonical_name,
                    "matched_alias": entity.alias,
                })).collect::<Vec<_>>(),
                "storage_policy": {
                    "historical_only": true,
                    "may_override_current_game_data": false,
                    "default_retrieval": "excluded_until_explicitly_requested"
                }
            });
            let claim_hash = stable_hash_text(&format!(
                "{FORUM_SOURCE}|{}|{}|{}|{}",
                post.post_id, claim_index, candidate.claim_type, candidate.claim_text
            ));
            let now = crate::util::now()?;
            let changed = conn.execute(
                r#"
                INSERT OR IGNORE INTO forum_claims(
                  post_snapshot_id, thread_id, post_id, thread_title, source_url,
                  posted_at, author, author_role, claim_hash, claim_index,
                  claim_type, entity_type, entity_name, claim_text, evidence_quote,
                  source_trust, validity_status, currentness, confidence,
                  safety_labels_json, source_references_json, metadata_json,
                  created_at, updated_at
                )
                VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24)
                "#,
                params![
                    post.snapshot_id,
                    post.thread_id,
                    post.post_id,
                    post.thread_title,
                    source_url,
                    post.posted_at,
                    post.author,
                    post.author_role,
                    claim_hash,
                    claim_index as i64,
                    candidate.claim_type,
                    entity_type,
                    entity_name,
                    candidate.claim_text,
                    candidate.evidence_quote,
                    candidate.source_trust,
                    candidate.validity_status,
                    candidate.currentness,
                    candidate.confidence,
                    json_string(&json!(candidate.safety_labels))?,
                    json_string(&source_references)?,
                    json_string(&metadata)?,
                    now,
                    now,
                ],
            )?;
            if changed > 0 {
                inserted += 1;
                *by_type.entry(candidate.claim_type.to_string()).or_default() += 1;
                *by_validity
                    .entry(candidate.validity_status.to_string())
                    .or_default() += 1;
            } else {
                skipped_duplicates += 1;
            }
        }
    }

    let total_claims: i64 = conn.query_row("SELECT COUNT(*) FROM forum_claims", [], |row| {
        row.get(0)
    })?;

    Ok(json!({
        "source": FORUM_SOURCE,
        "posts_seen": posts_seen,
        "claims_inserted": inserted,
        "claims_skipped_duplicates": skipped_duplicates,
        "claims_total": total_claims,
        "by_type": by_type,
        "by_validity": by_validity,
        "currentness_policy": "historical_quarantine",
        "retrieval_policy": "excluded_from_default_current_answers",
    }))
}

fn load_forum_posts(conn: &Connection) -> Result<Vec<ForumPost>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, payload_json
        FROM entity_snapshots
        WHERE source=?1 AND entity_type='forum_post'
        ORDER BY CAST(external_id AS INTEGER), id
        "#,
    )?;
    let rows = stmt.query_map(params![FORUM_SOURCE], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut posts = Vec::new();
    for row in rows {
        let (snapshot_id, payload_json) = row?;
        let payload: Value = serde_json::from_str(&payload_json)?;
        posts.push(ForumPost {
            snapshot_id,
            thread_id: value_string(payload.get("thread_id")).unwrap_or_default(),
            post_id: value_string(payload.get("post_id")).unwrap_or_default(),
            thread_title: value_string(payload.get("thread_title")),
            thread_url: value_string(payload.get("thread_url"))
                .or_else(|| value_string(payload.get("canonical_thread_url"))),
            posted_at: value_string(payload.get("datetime")),
            author: value_string(payload.get("author")),
            author_role: value_string(payload.get("user_title")),
            text: value_string(payload.get("text")).unwrap_or_default(),
        });
    }
    Ok(posts)
}

fn load_entity_aliases(conn: &Connection) -> Result<Vec<EntityHit>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT e.entity_type, e.canonical_name, a.alias_norm
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE length(a.alias_norm) >= 3
        ORDER BY length(a.alias_norm) DESC
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(EntityHit {
            entity_type: row.get(0)?,
            canonical_name: row.get(1)?,
            alias: row.get(2)?,
        })
    })?;
    let mut aliases = Vec::new();
    for row in rows {
        let alias = row?;
        if is_useful_alias(&alias.alias) {
            aliases.push(alias);
        }
    }
    Ok(aliases)
}

fn detect_entities(entity_aliases: &[EntityHit], post: &ForumPost) -> Vec<EntityHit> {
    let haystack = normalize_scan_text(&format!(
        "{} {}",
        post.thread_title.as_deref().unwrap_or_default(),
        post.text
    ));
    let mut hits = Vec::<EntityHit>::new();
    for alias in entity_aliases {
        let needle = normalize_scan_text(&alias.alias);
        if haystack.contains(&needle)
            && !hits
                .iter()
                .any(|hit| hit.entity_type == alias.entity_type && hit.canonical_name == alias.canonical_name)
        {
            hits.push(EntityHit {
                entity_type: alias.entity_type.clone(),
                canonical_name: alias.canonical_name.clone(),
                alias: alias.alias.clone(),
            });
        }
        if hits.len() >= 8 {
            break;
        }
    }
    hits
}

fn claim_candidates(post: &ForumPost) -> Vec<ClaimCandidate> {
    let title = post.thread_title.as_deref().unwrap_or("Forum thread");
    let text = collapse_ws(&post.text);
    let evidence = truncate_chars(&text, 700);
    if evidence.is_empty() && title.trim().is_empty() {
        return Vec::new();
    }

    if is_developer(post) {
        return vec![developer_candidate(title, &evidence)];
    }

    let blob = format!("{} {}", title, text).to_lowercase();
    if has_any(
        &blob,
        &[
            "exploit",
            "infinite dash",
            "infinite dashing",
            "out of bounds",
            "clip through",
            "through walls",
            "top of the map",
            "enemy base",
            "duplicate",
            "duplicated",
        ],
    ) {
        return vec![community_candidate(
            "exploit_risk",
            "historical_unverified",
            0.45,
            vec!["historical_only", "not_current_fact", "exploit_risk"],
            format!("Historical forum exploit-risk report: {title}. {evidence}"),
            evidence,
        )];
    }

    if has_any(
        &blob,
        &[
            "bug",
            "broken",
            "stuck",
            "crash",
            "incorrect",
            "not working",
            "wrong",
            "failed",
            "unable",
            "cannot",
            "can't",
            "hidden",
            "locked",
        ],
    ) {
        return vec![community_candidate(
            "bug_report",
            "historical_unverified",
            0.5,
            vec!["historical_only", "not_current_fact"],
            format!("Historical forum bug report: {title}. {evidence}"),
            evidence,
        )];
    }

    if has_any(
        &blob,
        &[
            "feedback",
            "balance",
            "meta",
            "overpowered",
            "underpowered",
            "nerf",
            "buff",
        ],
    ) {
        return vec![community_candidate(
            "forum_feedback",
            "historical_unverified",
            0.35,
            vec!["historical_only", "not_current_fact"],
            format!("Historical forum feedback/discussion: {title}. {evidence}"),
            evidence,
        )];
    }

    Vec::new()
}

fn developer_candidate(title: &str, evidence: &str) -> ClaimCandidate {
    let evidence_lower = evidence.to_lowercase();
    if has_any(
        &evidence_lower,
        &[
            "fixed",
            "fix internally",
            "fixed internally",
            "checked in a fix",
            "should be fixed",
        ],
    ) {
        return ClaimCandidate {
            claim_type: "developer_fix_status",
            source_trust: "developer_response",
            validity_status: "fixed_or_obsolete",
            currentness: "historical_quarantine",
            confidence: 0.85,
            safety_labels: vec!["historical_only", "not_current_fact", "developer_sourced"],
            claim_text: format!("Developer response indicates the historical forum report was fixed or queued for a fix: {title}. {evidence}"),
            evidence_quote: evidence.to_string(),
        };
    }

    ClaimCandidate {
        claim_type: "developer_note",
        source_trust: "developer_response",
        validity_status: "historical_developer_note",
        currentness: "historical_quarantine",
        confidence: 0.75,
        safety_labels: vec!["historical_only", "not_current_fact", "developer_sourced"],
        claim_text: format!("Historical developer forum note: {title}. {evidence}"),
        evidence_quote: evidence.to_string(),
    }
}

fn community_candidate(
    claim_type: &'static str,
    validity_status: &'static str,
    confidence: f64,
    safety_labels: Vec<&'static str>,
    claim_text: String,
    evidence_quote: String,
) -> ClaimCandidate {
    ClaimCandidate {
        claim_type,
        source_trust: "community_report",
        validity_status,
        currentness: "historical_quarantine",
        confidence,
        safety_labels,
        claim_text: truncate_chars(&claim_text, 1000),
        evidence_quote,
    }
}

fn source_url(post: &ForumPost) -> String {
    let thread_url = post
        .thread_url
        .as_deref()
        .unwrap_or("https://forums.playdeadlock.com/");
    format!("{}/post-{}", thread_url.trim_end_matches('/'), post.post_id)
}

fn is_developer(post: &ForumPost) -> bool {
    let role = post.author_role.as_deref().unwrap_or_default().to_lowercase();
    let author = post.author.as_deref().unwrap_or_default().to_lowercase();
    role.contains("valve developer") || author == "valve"
}

fn has_any(blob: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| blob.contains(needle))
}

fn is_useful_alias(alias: &str) -> bool {
    let alias = alias.trim();
    if alias.len() < 3 || alias.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }
    !matches!(
        alias,
        "the" | "and" | "for" | "you" | "all" | "new" | "old" | "base"
    )
}

fn value_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(text)) => non_empty(text),
        Some(Value::Number(number)) => Some(number.to_string()),
        Some(Value::Bool(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn collapse_ws(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_chars(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}
