use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};

use crate::Result;

pub const GAME_WIKI_DIR_ENV: &str = "DEADLOCK_BRAIN_GAME_WIKI_DIR";
const DEFAULT_SEARCH_LIMIT: usize = 3;
const WIKI_SOURCES: &[&str] = &["deadlock_data", "deadlock_wiki"];
const WIKI_ENTITY_TYPES: &[&str] = &[
    "hero",
    "item",
    "item_special",
    "ability",
    "ability_card",
    "item_card",
    "npc_data",
    "objective",
    "objective_entity",
    "patchnote",
    "patchnote_structured",
    "patchnote_wikitext",
    "resource_lookup",
    "supporting_doc",
    "wiki_page",
    "json_attribute_data",
    "json_generic_data",
    "json_misc_data",
    "json_soul_unlock_data",
    "json_midtown_metadata",
    "item_component_tree",
];

pub fn default_game_wiki_dir() -> PathBuf {
    deadlock_brain_core::config::repo_root().join("game-wiki")
}

pub async fn rebuild_game_wiki(pool: &PgPool, root: &Path) -> Result<JsonValue> {
    fs::create_dir_all(root)?;
    let pages_root = root.join("pages");
    if pages_root.exists() {
        fs::remove_dir_all(&pages_root)?;
    }
    fs::create_dir_all(&pages_root)?;

    let rows = load_latest_snapshots(pool).await?;
    write_schema(root)?;

    let generated_at = Utc::now();
    let mut summaries = Vec::new();
    let mut used_anchors = HashSet::new();
    let mut shards = BTreeMap::<String, Vec<String>>::new();
    for row in rows {
        let (summary, entry) = render_snapshot_entry(&row, generated_at, &mut used_anchors)?;
        shards.entry(shard_path(&row)).or_default().push(entry);
        summaries.push(summary);
    }
    for (relative_path, entries) in &shards {
        write_shard_page(root, relative_path, entries, generated_at)?;
    }
    summaries.sort_by(|left, right| {
        left.source
            .cmp(&right.source)
            .then(left.entity_type.cmp(&right.entity_type))
            .then(left.title.cmp(&right.title))
            .then(left.path.cmp(&right.path))
    });
    write_index(root, &summaries, generated_at)?;
    append_log(root, &summaries, generated_at)?;

    let mut counts = BTreeMap::<String, usize>::new();
    for summary in &summaries {
        *counts
            .entry(format!("{}/{}", summary.source, summary.entity_type))
            .or_insert(0) += 1;
    }

    Ok(json!({
        "root": root,
        "sources": WIKI_SOURCES,
        "entity_types": WIKI_ENTITY_TYPES,
        "entries_written": summaries.len(),
        "files_written": shards.len(),
        "counts": counts,
        "index": root.join("index.md"),
        "log": root.join("log.md"),
    }))
}

pub fn search_game_wiki(
    root: Option<&Path>,
    query: &str,
    entity: &JsonValue,
    limit: usize,
) -> Result<JsonValue> {
    let root = resolve_game_wiki_dir(root);
    let pages_root = root.join("pages");
    if !pages_root.is_dir() {
        return Ok(json!({
            "available": false,
            "root": root,
            "reason": "game-wiki/pages fehlt; fuehre `deadlock-brain wiki rebuild` aus",
        }));
    }

    let query_terms = ranked_terms(query, entity);
    let mut files = Vec::new();
    collect_markdown_files(&pages_root, &mut files)?;
    files.sort();

    let mut matches = Vec::new();
    for path in files {
        let relative_path = path
            .strip_prefix(&root)
            .unwrap_or(path.as_path())
            .to_string_lossy()
            .replace('\\', "/");
        let content = fs::read_to_string(&path)?;
        for entry in split_search_entries(&content) {
            let title = page_title(entry).unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|value| value.to_str())
                    .unwrap_or("unknown")
                    .replace('-', " ")
            });
            let score = score_page(&path, &title, entry, &query_terms);
            if score <= 0 {
                continue;
            }
            matches.push(ScoredPage {
                score,
                title,
                path: entry_path(&relative_path, entry),
                content: entry.to_string(),
            });
        }
    }
    matches.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then(left.title.cmp(&right.title))
            .then(left.path.cmp(&right.path))
    });
    let limit = if limit == 0 {
        DEFAULT_SEARCH_LIMIT
    } else {
        limit
    };
    let total_matches = matches.len();
    let matches_json = matches
        .into_iter()
        .take(limit)
        .map(|page| {
            json!({
                "title": page.title,
                "path": page.path,
                "score": page.score,
                "content": page.content,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "available": true,
        "root": root,
        "query": query,
        "matches": matches_json,
        "match_count": total_matches,
    }))
}

#[derive(Debug, Clone)]
struct SnapshotRow {
    id: i64,
    source: String,
    entity_type: String,
    external_id: String,
    canonical_name: Option<String>,
    payload_hash: String,
    payload: JsonValue,
    fetched_at: Option<DateTime<Utc>>,
    source_document_id: Option<i64>,
    source_title: Option<String>,
    source_url: Option<String>,
    source_content_hash: Option<String>,
    source_raw_path: Option<String>,
}

#[derive(Debug, Clone)]
struct PageSummary {
    source: String,
    entity_type: String,
    title: String,
    path: String,
    external_id: String,
    snapshot_id: i64,
    summary: String,
}

#[derive(Debug)]
struct ScoredPage {
    score: i64,
    title: String,
    path: String,
    content: String,
}

async fn load_latest_snapshots(pool: &PgPool) -> Result<Vec<SnapshotRow>> {
    let sources = WIKI_SOURCES
        .iter()
        .map(|value| (*value).to_string())
        .collect::<Vec<_>>();
    let entity_types = WIKI_ENTITY_TYPES
        .iter()
        .map(|value| (*value).to_string())
        .collect::<Vec<_>>();
    let rows = sqlx::query(
        r#"
        WITH ranked AS (
            SELECT
                es.id,
                es.source,
                es.entity_type,
                es.external_id,
                es.canonical_name,
                es.payload_hash,
                es.payload,
                es.fetched_at,
                es.source_document_id,
                sd.title AS source_title,
                sd.url AS source_url,
                sd.content_hash AS source_content_hash,
                sd.raw_path AS source_raw_path,
                row_number() OVER (
                    PARTITION BY es.source, es.entity_type, es.external_id
                    ORDER BY es.fetched_at DESC NULLS LAST, es.id DESC
                ) AS rn
            FROM brain.entity_snapshots es
            LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
            WHERE es.source = ANY($1)
              AND es.entity_type = ANY($2)
        )
        SELECT *
        FROM ranked
        WHERE rn = 1
        ORDER BY source, entity_type, COALESCE(canonical_name, external_id), id
        "#,
    )
    .bind(&sources)
    .bind(&entity_types)
    .fetch_all(pool)
    .await?;

    let mut snapshots = Vec::with_capacity(rows.len());
    for row in rows {
        snapshots.push(SnapshotRow {
            id: row.try_get("id")?,
            source: row.try_get("source")?,
            entity_type: row.try_get("entity_type")?,
            external_id: row.try_get("external_id")?,
            canonical_name: row.try_get("canonical_name")?,
            payload_hash: row.try_get("payload_hash")?,
            payload: row.try_get("payload")?,
            fetched_at: row.try_get("fetched_at")?,
            source_document_id: row.try_get("source_document_id")?,
            source_title: row.try_get("source_title")?,
            source_url: row.try_get("source_url")?,
            source_content_hash: row.try_get("source_content_hash")?,
            source_raw_path: row.try_get("source_raw_path")?,
        });
    }
    Ok(snapshots)
}

fn render_snapshot_entry(
    row: &SnapshotRow,
    generated_at: DateTime<Utc>,
    used_anchors: &mut HashSet<String>,
) -> Result<(PageSummary, String)> {
    let title = snapshot_title(row);
    let summary = snapshot_summary(row, &title);
    let mut anchor = nonempty_slug(&title, &row.external_id);
    let shard = shard_path(row);
    let anchor_key = format!("{shard}#{anchor}");
    if !used_anchors.insert(anchor_key) {
        let suffix = stable_hash(&format!(
            "{}:{}:{}:{}",
            row.source, row.entity_type, row.external_id, row.id
        ));
        anchor = format!("{anchor}-{}", &suffix[..8]);
        used_anchors.insert(format!("{shard}#{anchor}"));
    }
    let payload_json = serde_json::to_string_pretty(&row.payload)?;
    let fence = code_fence_for(&payload_json);
    let source_raw_path = portable_raw_path(row.source_raw_path.as_deref());
    let entry = format!(
        r#"<!-- game-wiki-entry source={source_yaml} entity_type={entity_type_yaml} external_id={external_id_yaml} title={title_yaml} -->

## {title}

### Kurzueberblick

- Typ: `{entity_type}`
- Quelle: `{source}`
- External ID: `{external_id}`
- Snapshot ID: `{snapshot_id}`
- Source-Dokument: `{source_document}`
- Kurzinfo: {summary}

### Provenienz

- Canonical Name: `{canonical_name}`
- Payload Hash: `{payload_hash}`
- Source Content Hash: `{source_content_hash}`
- Source URL: `{source_url}`
- Source Raw Path: `{source_raw_path}`
- Fetched At: `{fetched_at}`
- Generated At: `{generated_at}`

### Vollstaendige Payload

{fence}json
{payload_json}
{fence}
"#,
        title_yaml = yaml_string(&title),
        entity_type_yaml = yaml_string(&row.entity_type),
        source_yaml = yaml_string(&row.source),
        external_id_yaml = yaml_string(&row.external_id),
        snapshot_id = row.id,
        entity_type = row.entity_type.as_str(),
        source = row.source.as_str(),
        external_id = row.external_id.as_str(),
        source_document = row
            .source_document_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "nicht verknuepft".to_string()),
        canonical_name = row.canonical_name.as_deref().unwrap_or("null"),
        payload_hash = row.payload_hash.as_str(),
        source_content_hash = row.source_content_hash.as_deref().unwrap_or("null"),
        source_url = row.source_url.as_deref().unwrap_or("null"),
        source_raw_path = source_raw_path.as_deref().unwrap_or("null"),
        fetched_at = row
            .fetched_at
            .map(|value| value.to_rfc3339())
            .unwrap_or_else(|| "null".to_string()),
        generated_at = generated_at.to_rfc3339(),
        summary = summary,
        payload_json = payload_json,
        fence = fence,
    );

    Ok((PageSummary {
        source: row.source.clone(),
        entity_type: row.entity_type.clone(),
        title,
        path: format!("{shard}#{anchor}"),
        external_id: row.external_id.clone(),
        snapshot_id: row.id,
        summary,
    }, entry))
}

fn write_shard_page(
    root: &Path,
    relative_path: &str,
    entries: &[String],
    generated_at: DateTime<Utc>,
) -> Result<()> {
    let page_path = root.join(relative_path);
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let title = page_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown")
        .replace('-', " ");
    let mut content = format!(
        "---\ntitle: {}\ngenerated_at: {}\nentries: {}\n---\n\n# {}\n\n",
        yaml_string(&title),
        yaml_string(&generated_at.to_rfc3339()),
        entries.len(),
        title
    );
    for entry in entries {
        content.push_str(entry);
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push('\n');
    }
    fs::write(page_path, content)?;
    Ok(())
}

fn shard_path(row: &SnapshotRow) -> String {
    format!(
        "pages/{}/{}.md",
        safe_segment(&row.source),
        safe_segment(&row.entity_type)
    )
}

fn write_schema(root: &Path) -> Result<()> {
    fs::write(
        root.join("AGENTS.md"),
        r#"# Game Wiki Schema

Diese Wiki-Schicht folgt dem LLM-Wiki-Muster fuer Deadlock-Brain.

## Schichten

- Raw Sources: `brain.source_documents` und `brain.entity_snapshots`. Diese Daten sind die Quelle der Wahrheit und werden hier nicht editiert.
- Wiki: Markdown unter `game-wiki/pages/`. Diese Corpus-Shards werden durch `deadlock-brain wiki rebuild` erzeugt und duerfen vom Agenten gepflegt werden.
- Schema: diese Datei. Aendere sie nur, wenn sich Struktur oder Arbeitsweise bewusst aendern.

## Regeln

- Fakten duerfen nur aus der Payload oder aus verlinkten Source-Metadaten stammen.
- Jeder Eintrag muss Provenienz behalten.
- Die vollstaendige JSON-Payload bleibt im Eintrag erhalten.
- `index.md` ist der Inhaltskatalog.
- `log.md` ist append-only.
- Bei Widerspruch gewinnt die neueste `deadlock_data`-Payload gegen aeltere Wiki- oder Creator-Aussagen.

## Query-Workflow

1. `index.md` oder die lokale Wiki-Suche lesen.
2. Relevante Eintraege vollstaendig lesen.
3. Antwort nur aus Ground-Truth, verifizierten Creator-Claims und den gelesenen Wiki-Eintraegen synthetisieren.
4. Unsichere oder fehlende Fakten offen markieren.
"#,
    )?;
    Ok(())
}

fn write_index(root: &Path, summaries: &[PageSummary], generated_at: DateTime<Utc>) -> Result<()> {
    let mut by_category = BTreeMap::<String, Vec<&PageSummary>>::new();
    for summary in summaries {
        by_category
            .entry(format!("{}/{}", summary.source, summary.entity_type))
            .or_default()
            .push(summary);
    }
    let mut out = String::new();
    out.push_str("# Game Wiki Index\n\n");
    out.push_str(&format!(
        "- Generated at: `{}`\n- Entries: `{}`\n- Sources: `{}`\n\n",
        generated_at.to_rfc3339(),
        summaries.len(),
        WIKI_SOURCES.join(", ")
    ));
    out.push_str("## Kategorien\n\n");
    for (category, pages) in by_category {
        out.push_str(&format!("### {category}\n\n"));
        for page in pages {
            out.push_str(&format!(
                "- [{}]({}) - {} (`external_id={}`, `snapshot_id={}`)\n",
                page.title, page.path, page.summary, page.external_id, page.snapshot_id
            ));
        }
        out.push('\n');
    }
    while out.ends_with("\n\n") {
        out.pop();
    }
    fs::write(root.join("index.md"), out)?;
    Ok(())
}

fn append_log(root: &Path, summaries: &[PageSummary], generated_at: DateTime<Utc>) -> Result<()> {
    let log_path = root.join("log.md");
    if !log_path.exists() {
        fs::write(&log_path, "# Game Wiki Log\n\n")?;
    }
    let mut file = OpenOptions::new().append(true).open(&log_path)?;
    writeln!(
        file,
        "## [{}] rebuild | deadlock-data snapshots\n\n- Entries written: `{}`\n- Sources: `{}`",
        generated_at.format("%Y-%m-%d %H:%M:%SZ"),
        summaries.len(),
        WIKI_SOURCES.join(", ")
    )?;
    Ok(())
}

fn resolve_game_wiki_dir(root: Option<&Path>) -> PathBuf {
    if let Some(root) = root {
        return root.to_path_buf();
    }
    env::var_os(GAME_WIKI_DIR_ENV)
        .map(PathBuf::from)
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(default_game_wiki_dir)
}

fn collect_markdown_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, out)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

fn split_search_entries(content: &str) -> Vec<&str> {
    const MARKER: &str = "<!-- game-wiki-entry ";
    let starts = content
        .match_indices(MARKER)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if starts.is_empty() {
        return vec![content];
    }
    let mut entries = Vec::with_capacity(starts.len());
    for (idx, start) in starts.iter().enumerate() {
        let end = starts
            .get(idx + 1)
            .copied()
            .unwrap_or_else(|| content.len());
        entries.push(content[*start..end].trim());
    }
    entries
}

fn entry_path(relative_path: &str, entry: &str) -> String {
    page_title(entry)
        .map(|title| format!("{relative_path}#{}", slugify(&title)))
        .unwrap_or_else(|| relative_path.to_string())
}

fn ranked_terms(query: &str, entity: &JsonValue) -> Vec<String> {
    let mut terms = HashSet::new();
    for term in tokenize(query) {
        terms.insert(term);
    }
    for pointer in ["/canonical_name", "/entity_type", "/name"] {
        if let Some(value) = entity.pointer(pointer).and_then(JsonValue::as_str) {
            for term in tokenize(value) {
                terms.insert(term);
            }
        }
    }
    let mut terms = terms.into_iter().collect::<Vec<_>>();
    terms.sort_by(|left, right| right.len().cmp(&left.len()).then(left.cmp(right)));
    terms
}

fn tokenize(value: &str) -> Vec<String> {
    value
        .split(|ch: char| !ch.is_alphanumeric())
        .map(|term| term.trim().to_lowercase())
        .filter(|term| term.len() >= 3)
        .filter(|term| !WIKI_SEARCH_STOPWORDS.contains(&term.as_str()))
        .collect()
}

const WIKI_SEARCH_STOPWORDS: &[&str] = &[
    "wie",
    "was",
    "wird",
    "funktioniert",
    "gegen",
    "und",
    "oder",
    "the",
    "for",
    "with",
    "how",
    "does",
    "item",
    "hero",
    "ability",
    "deadlock",
];

fn score_page(path: &Path, title: &str, content: &str, terms: &[String]) -> i64 {
    if terms.is_empty() {
        return 0;
    }
    let path_text = path.to_string_lossy().to_lowercase();
    let title_text = title.to_lowercase();
    let content_text = content.to_lowercase();
    let mut score = 0;
    for term in terms {
        if title_text.contains(term) {
            score += 80;
        }
        if path_text.contains(term) {
            score += 40;
        }
        if content_text.contains(term) {
            score += 5;
        }
    }
    if terms
        .iter()
        .filter(|term| title_text.contains(term.as_str()) || path_text.contains(term.as_str()))
        .count()
        >= 2
    {
        score += 60;
    }
    score
}

fn page_title(content: &str) -> Option<String> {
    for line in content.lines().take(20) {
        let line = line.trim();
        if let Some(title) = line.strip_prefix("title:") {
            return Some(title.trim().trim_matches('"').to_string());
        }
        if let Some(title) = line.strip_prefix("# ") {
            return Some(title.trim().to_string());
        }
        if let Some(title) = line.strip_prefix("## ") {
            return Some(title.trim().to_string());
        }
    }
    None
}

fn snapshot_title(row: &SnapshotRow) -> String {
    row.canonical_name
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_string())
        .or_else(|| payload_string(&row.payload, &["name", "title", "hero_name", "item_name"]))
        .or_else(|| row.source_title.clone())
        .unwrap_or_else(|| row.external_id.clone())
}

fn snapshot_summary(row: &SnapshotRow, title: &str) -> String {
    payload_string(
        &row.payload,
        &[
            "description",
            "desc",
            "shop_description",
            "summary",
            "subtitle",
            "type",
        ],
    )
    .map(|value| one_line(&value, 180))
    .unwrap_or_else(|| {
        format!(
            "{} aus `{}` / `{}` mit vollstaendiger Payload.",
            title, row.source, row.entity_type
        )
    })
}

fn payload_string(payload: &JsonValue, keys: &[&str]) -> Option<String> {
    let object = payload.as_object()?;
    for key in keys {
        if let Some(value) = object.get(*key).and_then(JsonValue::as_str) {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn one_line(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        return compact;
    }
    let mut out = compact.chars().take(max_chars).collect::<String>();
    out.push_str("...");
    out
}

fn yaml_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

fn safe_segment(value: &str) -> String {
    nonempty_slug(value, "unknown")
}

fn nonempty_slug(value: &str, fallback: &str) -> String {
    let slug = slugify(value);
    if slug.is_empty() {
        slugify(fallback)
    } else {
        slug
    }
}

fn slugify(value: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

fn stable_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn portable_raw_path(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }
    if let Some(index) = value.find("/data/raw/") {
        return Some(format!("data/raw/{}", &value[index + "/data/raw/".len()..]));
    }
    Some(value.to_string())
}

fn code_fence_for(content: &str) -> String {
    let mut longest = 0usize;
    let mut current = 0usize;
    for ch in content.chars() {
        if ch == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    "`".repeat(longest.max(3) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn search_game_wiki_returns_full_matching_page() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pages = tmp.path().join("pages/abilities");
        fs::create_dir_all(&pages).expect("pages");
        let full_tail = "TAIL_TOKEN_FULL_JSON_STAYS_VISIBLE";
        fs::write(
            pages.join("kinetic-carbine.md"),
            format!(
                "---\ntitle: Kinetic Carbine\nentity_type: ability\n---\n# Kinetic Carbine\n\nFacts.\n\n```json\n{{\"name\":\"Kinetic Carbine\",\"details\":\"{}\"}}\n```\n",
                "x".repeat(2048) + full_tail
            ),
        )
        .expect("write page");

        let result = search_game_wiki(
            Some(tmp.path()),
            "Wie funktioniert Kinetic Carbine?",
            &JsonValue::Null,
            3,
        )
        .expect("search");

        assert_eq!(
            result.pointer("/available").and_then(JsonValue::as_bool),
            Some(true)
        );
        let content = result
            .pointer("/matches/0/content")
            .and_then(JsonValue::as_str)
            .expect("first match content");
        assert!(
            content.contains(full_tail),
            "matching pages must be returned complete"
        );
    }

    #[test]
    fn search_game_wiki_extracts_complete_matching_shard_entry() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pages = tmp.path().join("pages/deadlock-data");
        fs::create_dir_all(&pages).expect("pages");
        let full_tail = "TAIL_TOKEN_FULL_SHARD_ENTRY_STAYS_VISIBLE";
        fs::write(
            pages.join("ability.md"),
            format!(
                "# ability\n\n<!-- game-wiki-entry title=\"Kinetic Carbine\" -->\n\n## Kinetic Carbine\n\n```json\n{{\"details\":\"{}\"}}\n```\n\n<!-- game-wiki-entry title=\"Other Ability\" -->\n\n## Other Ability\n\nOTHER_ENTRY_TOKEN\n",
                "x".repeat(2048) + full_tail
            ),
        )
        .expect("write shard");

        let result = search_game_wiki(
            Some(tmp.path()),
            "Wie funktioniert Kinetic Carbine?",
            &JsonValue::Null,
            3,
        )
        .expect("search");

        let content = result
            .pointer("/matches/0/content")
            .and_then(JsonValue::as_str)
            .expect("first match content");
        assert!(content.contains(full_tail));
        assert!(
            !content.contains("OTHER_ENTRY_TOKEN"),
            "search should include the full matching entry, not the whole shard"
        );
    }
}
