use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process,
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
    let rows = load_latest_snapshots(pool).await?;
    write_schema(root)?;

    let generated_at = Utc::now();
    let build_root = root.join(format!(
        ".pages-rebuild-{}-{}",
        process::id(),
        generated_at.timestamp_nanos_opt().unwrap_or_default()
    ));
    if build_root.exists() {
        fs::remove_dir_all(&build_root)?;
    }
    fs::create_dir_all(build_root.join("pages"))?;

    let mut summaries = Vec::new();
    let mut used_anchors = HashSet::new();
    let mut shards = BTreeMap::<String, Vec<String>>::new();
    for row in rows {
        let (summary, entry) = render_snapshot_entry(&row, &mut used_anchors)?;
        shards.entry(shard_path(&row)).or_default().push(entry);
        summaries.push(summary);
    }
    for (relative_path, entries) in &shards {
        write_shard_page(&build_root, relative_path, entries)?;
    }
    replace_pages_atomically(root, &build_root.join("pages"))?;
    let _ = fs::remove_dir_all(&build_root);

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
    search_game_wiki_inner(root, query, entity, limit, false)
}

/// Answer context may include one complete hero and its bound ability cards.
/// The general wiki search keeps its strict caller limit unchanged.
pub(crate) fn search_game_wiki_for_answer(
    root: Option<&Path>,
    query: &str,
    entity: &JsonValue,
    intent: &str,
    limit: usize,
) -> Result<JsonValue> {
    let ability_overview = intent == "hero_overview"
        && words(query).iter().any(|word| {
            matches!(
                word.as_str(),
                "fähigkeiten"
                    | "faehigkeiten"
                    | "fertigkeiten"
                    | "abilities"
                    | "skills"
                    | "skillset"
            )
        });
    search_game_wiki_inner(root, query, entity, limit, ability_overview)
}

fn search_game_wiki_inner(
    root: Option<&Path>,
    query: &str,
    entity: &JsonValue,
    limit: usize,
    expand_hero: bool,
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
            let binding = entry_binding(entry);
            if score <= 0 && binding.is_none() {
                continue;
            }
            matches.push(ScoredPage {
                score,
                title,
                path: entry_path(&relative_path, entry),
                content: entry.to_string(),
                binding,
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
    let mut limit = if limit == 0 {
        DEFAULT_SEARCH_LIMIT
    } else {
        limit
    };
    let mut hero_context = JsonValue::Null;
    if expand_hero {
        if let Some(hero_index) = resolved_hero(&matches, query, entity) {
            let hero = matches.remove(hero_index);
            let binding = hero.binding.as_ref().expect("resolved hero binding");
            let bound = binding.bound.as_ref().expect("bound abilities");
            const MAX_HERO_ABILITIES: usize = 8;
            limit = 1 + bound.len().min(MAX_HERO_ABILITIES);
            let mut selected = Vec::new();
            let mut missing = Vec::new();
            for (key, name) in bound.iter().take(MAX_HERO_ABILITIES) {
                // BoundAbilities is the explicit hero-to-ability relation. A
                // card's display HeroKey can name another form sharing it or
                // be stale; use the unique stable ability key, never an
                // arbitrary first card when snapshots disagree on that key.
                let indices = matches
                    .iter()
                    .enumerate()
                    .filter(|(_, page)| {
                        page.binding
                            .as_ref()
                            .is_some_and(|ability| ability.key == *key && ability.is_ability_card)
                    })
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();
                if indices.len() == 1 {
                    selected.push(matches.remove(indices[0]));
                } else {
                    missing.push(json!({"key": key, "name": name}));
                }
            }
            hero_context = json!({
                "hero": binding.name, "bound_abilities": bound.len(),
                "binding_source": "hero_bound_abilities",
                "ability_bindings": bound.iter().take(MAX_HERO_ABILITIES).map(|(key, name)| json!({"key":key,"name":name})).collect::<Vec<_>>(),
                "missing_abilities": missing, "omitted_abilities": bound.len().saturating_sub(MAX_HERO_ABILITIES),
                "complete": missing.is_empty() && bound.len() <= MAX_HERO_ABILITIES,
            });
            selected.insert(0, hero);
            matches = selected;
        }
    }
    matches.retain(|page| page.score > 0 || !hero_context.is_null());
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
        "match_limit": limit,
        "hero_context": hero_context,
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
    binding: Option<EntryBinding>,
}

#[derive(Debug)]
struct EntryBinding {
    key: String,
    name: String,
    is_ability_card: bool,
    bound: Option<Vec<(String, String)>>,
}

fn entry_binding(entry: &str) -> Option<EntryBinding> {
    let kind = entry.lines().next().unwrap_or_default();
    if !["entity_type=\"hero\"", "entity_type=\"ability_card\""]
        .iter()
        .any(|value| kind.contains(value))
    {
        return None;
    }
    let mut lines = entry.lines();
    let fence = lines.find(|line| line.starts_with("```") && line.trim_end().ends_with("json"))?;
    let closing = fence.trim_end().strip_suffix("json")?;
    let body = lines
        .take_while(|line| line.trim_end() != closing)
        .collect::<Vec<_>>()
        .join("\n");
    let payload: JsonValue = serde_json::from_str(&body).ok()?;
    let key = payload.get("Key")?.as_str()?.to_string();
    let bound = match payload.get("BoundAbilities") {
        Some(slots) => Some(
            slots
                .as_object()?
                .values()
                .map(|ability| {
                    Some((
                        ability.get("Key")?.as_str()?.to_string(),
                        ability.get("Name")?.as_str()?.to_string(),
                    ))
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        None => None,
    };
    Some(EntryBinding {
        key,
        name: payload
            .get("Name")
            .and_then(JsonValue::as_str)
            .unwrap_or_default()
            .to_string(),
        is_ability_card: kind.contains("entity_type=\"ability_card\""),
        bound,
    })
}

fn words(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn resolved_hero(pages: &[ScoredPage], query: &str, entity: &JsonValue) -> Option<usize> {
    let query_words = words(query);
    let mut spans = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        let Some(binding) = page
            .binding
            .as_ref()
            .filter(|binding| binding.bound.is_some())
        else {
            continue;
        };
        let name = words(&binding.name);
        if name.is_empty() {
            continue;
        }
        for (start, part) in query_words.windows(name.len()).enumerate() {
            if part == name {
                spans.push((index, start, start + name.len()));
            }
        }
    }
    // Suppress a shorter name only at the exact occurrence enclosed by a
    // longer name. A separate mention elsewhere still represents a comparison.
    let explicit = spans
        .iter()
        .filter(|(_, start, end)| {
            !spans.iter().any(|(_, other_start, other_end)| {
                other_start <= start && other_end >= end && other_end - other_start > end - start
            })
        })
        .map(|(index, _, _)| *index)
        .collect::<HashSet<_>>();
    if explicit.len() == 1 {
        return explicit.iter().next().copied();
    }
    // Never let a singular planner result override an explicit comparison.
    if !explicit.is_empty() {
        return None;
    }
    let entity_name = entity
        .get("canonical_name")
        .or_else(|| entity.get("name"))?
        .as_str()?;
    let matched = pages
        .iter()
        .enumerate()
        .filter(|(_, page)| {
            page.binding.as_ref().is_some_and(|binding| {
                binding.bound.is_some() && words(&binding.name) == words(entity_name)
            })
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    (matched.len() == 1).then(|| matched[0])
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
        summary = summary,
        payload_json = payload_json,
        fence = fence,
    );

    Ok((
        PageSummary {
            source: row.source.clone(),
            entity_type: row.entity_type.clone(),
            title,
            path: format!("{shard}#{anchor}"),
            external_id: row.external_id.clone(),
            snapshot_id: row.id,
            summary,
        },
        entry,
    ))
}

fn write_shard_page(root: &Path, relative_path: &str, entries: &[String]) -> Result<()> {
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
        "---\ntitle: {}\nentries: {}\n---\n\n# {}\n\n",
        yaml_string(&title),
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

fn replace_pages_atomically(root: &Path, built_pages: &Path) -> Result<()> {
    let live_pages = root.join("pages");
    let backup_pages = root.join(format!(".pages-backup-{}", process::id()));
    if backup_pages.exists() {
        fs::remove_dir_all(&backup_pages)?;
    }
    let had_live_pages = live_pages.exists();
    if had_live_pages {
        fs::rename(&live_pages, &backup_pages)?;
    }
    if let Err(error) = fs::rename(built_pages, &live_pages) {
        if had_live_pages && backup_pages.exists() {
            let _ = fs::rename(&backup_pages, &live_pages);
        }
        return Err(error.into());
    }
    if backup_pages.exists() {
        fs::remove_dir_all(&backup_pages)?;
    }
    Ok(())
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
        let end = starts.get(idx + 1).copied().unwrap_or(content.len());
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
    "hat",
    "haben",
    "hatte",
    "welche",
    "welcher",
    "welches",
    "welchen",
    "ist",
    "sind",
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
    let path_terms = words(&path.to_string_lossy())
        .into_iter()
        .collect::<HashSet<_>>();
    let title_terms = words(title).into_iter().collect::<HashSet<_>>();
    let content_terms = words(content).into_iter().collect::<HashSet<_>>();
    let mut score = 0;
    for term in terms {
        if title_terms.contains(term) {
            score += 80;
        }
        if path_terms.contains(term) {
            score += 40;
        }
        if content_terms.contains(term) {
            score += 5;
        }
    }
    if terms
        .iter()
        .filter(|term| title_terms.contains(term.as_str()) || path_terms.contains(term.as_str()))
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

    fn write_bound_hero_fixture(root: &Path, missing_last: bool) {
        let pages = root.join("pages/deadlock-data");
        fs::create_dir_all(&pages).unwrap();
        let mut slots = serde_json::Map::new();
        let mut cards = String::new();
        for index in 1..=4 {
            let key = format!("ability_beacon_{index}");
            let name = format!("Beacon Skill {index}");
            slots.insert(index.to_string(), json!({"Key":key,"Name":name}));
            if index == 4 && missing_last {
                continue;
            }
            cards.push_str(&format!("<!-- game-wiki-entry entity_type=\"ability_card\" -->\n## {name}\n````json\n{}\n````\n", json!({"Key":key,"HeroKey":"hero_beacon","HeroName":"Beacon","Name":name,"Slot":index.to_string(),"Info1":{"Main":{"Props":[{"Name":"Damage","Value":index * 10}]}}})));
        }
        cards.push_str("<!-- game-wiki-entry entity_type=\"ability_card\" -->\n## Hat Trick\n````json\n{\"Key\":\"ability_foreign\",\"HeroKey\":\"hero_foreign\",\"HeroName\":\"Other Hero\",\"Name\":\"Hat Trick\",\"Note\":\"Beacon is mentioned but not the owner\"}\n````\n");
        fs::write(
            pages.join("hero.md"),
            format!(
                "<!-- game-wiki-entry entity_type=\"hero\" -->\n## Beacon\n````json\n{}\n````\n",
                json!({"Key":"hero_beacon","Name":"Beacon","BoundAbilities":slots})
            ),
        )
        .unwrap();
        fs::write(pages.join("ability-card.md"), cards).unwrap();
    }

    #[test]
    fn answer_hero_bundle_uses_bound_keys_and_preserves_all_four_cards() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), false);
        let result = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Welche Fähigkeiten hat Beacon?",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        let matches = result["matches"].as_array().unwrap();
        assert_eq!(matches.len(), 5);
        assert_eq!(result["match_limit"], 5);
        assert_eq!(result["hero_context"]["complete"], true);
        assert_eq!(matches[0]["title"], "Beacon");
        for (index, page) in matches.iter().enumerate().skip(1) {
            assert_eq!(page["title"], format!("Beacon Skill {index}"));
            assert!(page["content"].as_str().unwrap().contains("Damage"));
        }
        assert!(!matches.iter().any(|entry| entry["title"] == "Hat Trick"));
        let general = search_game_wiki(
            Some(tmp.path()),
            "Welche Fähigkeiten hat Beacon?",
            &JsonValue::Null,
            3,
        )
        .unwrap();
        assert_eq!(general["matches"].as_array().unwrap().len(), 3);
        assert!(general["hero_context"].is_null());
    }

    #[test]
    fn missing_bound_card_is_explicit_without_foreign_filler() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), true);
        let result = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Beacon Fähigkeiten",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        assert_eq!(result["matches"].as_array().unwrap().len(), 4);
        assert_eq!(result["hero_context"]["complete"], false);
        assert_eq!(
            result["hero_context"]["missing_abilities"][0]["key"],
            "ability_beacon_4"
        );
    }

    #[test]
    fn hero_names_require_complete_tokens_and_comparisons_are_not_collapsed() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), false);
        fs::write(tmp.path().join("pages/deadlock-data/second.md"), "<!-- game-wiki-entry entity_type=\"hero\" -->\n## Second Hero\n````json\n{\"Key\":\"hero_second\",\"Name\":\"Second Hero\",\"BoundAbilities\":{}}\n````\n").unwrap();
        let false_match = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Beaconess Fähigkeiten",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        assert!(false_match["hero_context"].is_null());
        assert!(false_match["matches"].as_array().unwrap().is_empty());
        let comparison = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Fähigkeiten von Beacon und Second Hero",
            &json!({"canonical_name":"Beacon","entity_type":"hero"}),
            "hero_overview",
            3,
        )
        .unwrap();
        assert!(comparison["hero_context"].is_null());
        assert!(comparison["matches"].as_array().unwrap().len() <= 3);
        assert_eq!(
            score_page(
                Path::new("ability-card.md"),
                "Hat Trick",
                "Shatter Cannon",
                &ranked_terms("Welche Fähigkeiten hat Unbekannt?", &JsonValue::Null)
            ),
            0
        );
    }

    #[test]
    fn nested_name_only_suppresses_the_enclosed_occurrence() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), false);
        fs::write(tmp.path().join("pages/transformed.md"), "<!-- game-wiki-entry entity_type=\"hero\" -->\n## Beacon (Transformed)\n````json\n{\"Key\":\"hero_beacon_transformed\",\"Name\":\"Beacon (Transformed)\",\"BoundAbilities\":{\"1\":{\"Key\":\"ability_beacon_1\",\"Name\":\"Beacon Skill 1\"}}}\n````\n").unwrap();
        let transformed = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Welche Fähigkeiten hat Beacon (Transformed)?",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        assert_eq!(transformed["hero_context"]["hero"], "Beacon (Transformed)");
        assert_eq!(transformed["hero_context"]["complete"], true);
        let comparison = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Welche Fähigkeiten haben Beacon und Beacon (Transformed)?",
            &json!({"canonical_name":"Beacon (Transformed)"}),
            "hero_overview",
            3,
        )
        .unwrap();
        assert!(comparison["hero_context"].is_null());
    }

    #[test]
    fn bound_key_supports_shared_cards_but_ambiguous_keys_fail_closed() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), false);
        let cards_path = tmp.path().join("pages/deadlock-data/ability-card.md");
        let cards = fs::read_to_string(&cards_path).unwrap().replace(
            "\"HeroKey\":\"hero_beacon\"",
            "\"HeroKey\":\"hero_other_form\"",
        );
        fs::write(&cards_path, &cards).unwrap();
        let shared = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Beacon Fähigkeiten",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        assert_eq!(shared["hero_context"]["complete"], true);
        assert_eq!(
            shared["hero_context"]["binding_source"],
            "hero_bound_abilities"
        );
        assert_eq!(
            shared["hero_context"]["ability_bindings"][0]["key"],
            "ability_beacon_1"
        );
        assert_eq!(shared["matches"].as_array().unwrap().len(), 5);
        fs::write(tmp.path().join("pages/conflict.md"), "<!-- game-wiki-entry entity_type=\"ability_card\" -->\n## Conflicting Skill\n````json\n{\"Key\":\"ability_beacon_1\",\"HeroKey\":\"hero_foreign\",\"Name\":\"Conflicting Skill\"}\n````\n").unwrap();
        let ambiguous = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Beacon Fähigkeiten",
            &JsonValue::Null,
            "hero_overview",
            3,
        )
        .unwrap();
        assert_eq!(ambiguous["hero_context"]["complete"], false);
        assert_eq!(
            ambiguous["hero_context"]["missing_abilities"][0]["key"],
            "ability_beacon_1"
        );
        assert!(ambiguous["matches"]
            .as_array()
            .unwrap()
            .iter()
            .all(
                |entry| entry["title"] != "Conflicting Skill" && entry["title"] != "Beacon Skill 1"
            ));
    }

    #[test]
    fn incidental_hero_does_not_replace_item_or_patch_search() {
        let tmp = tempfile::tempdir().unwrap();
        write_bound_hero_fixture(tmp.path(), false);
        fs::write(
            tmp.path().join("pages/drain.md"),
            "# Spirit Lifesteal\nSpirit Lifesteal beschreibt Heilung aus Fähigkeitenschaden.",
        )
        .unwrap();
        let entity = json!({"canonical_name":"Beacon","entity_type":"hero"});
        // Even a broad planner overview classification may not turn an item
        // question into a request for every hero ability.
        for intent in ["item_question", "hero_overview"] {
            let result = search_game_wiki_for_answer(
                Some(tmp.path()),
                "Wie funktioniert Spirit Lifesteal bei Beacon?",
                &entity,
                intent,
                3,
            )
            .unwrap();
            assert!(result["hero_context"].is_null());
            assert!(result["matches"]
                .as_array()
                .unwrap()
                .iter()
                .any(|page| page["title"] == "Spirit Lifesteal"));
        }
        let patch = search_game_wiki_for_answer(
            Some(tmp.path()),
            "Welche Fähigkeiten von Beacon wurden im Patch geändert?",
            &entity,
            "patch_changes",
            3,
        )
        .unwrap();
        assert!(patch["hero_context"].is_null());
    }

    #[test]
    #[ignore = "Explizite Abnahme am versionierten echten Game-Wiki"]
    fn live_wiki_hero_bundle_and_nonhero_search() {
        let root = Path::new("../../../game-wiki");
        let all_cards =
            fs::read_to_string(root.join("pages/deadlock-data/ability-card.md")).unwrap();
        let mut keys = HashSet::new();
        for binding in split_search_entries(&all_cards)
            .into_iter()
            .filter_map(entry_binding)
        {
            assert!(
                keys.insert(binding.key),
                "Ability-Key muss im gesamten Roster eindeutig sein"
            );
        }
        assert!(keys.len() > 100);
        let mut other_heroes = Vec::new();
        for hero in ["Paradox", "Silver", "Silver (Transformed)"] {
            let bundle = search_game_wiki_for_answer(
                Some(root),
                &format!("Welche Fähigkeiten hat {hero}?"),
                &JsonValue::Null,
                "hero_overview",
                3,
            )
            .unwrap();
            assert_eq!(bundle["hero_context"]["hero"], hero);
            assert_eq!(bundle["hero_context"]["complete"], true, "{hero}");
            assert_eq!(bundle["matches"].as_array().unwrap().len(), 5, "{hero}");
            other_heroes.push(json!({"hero":hero,"titles":bundle["matches"].as_array().unwrap().iter().map(|page| &page["title"]).collect::<Vec<_>>()}));
        }
        let comparison = search_game_wiki_for_answer(
            Some(root),
            "Welche Fähigkeiten haben Silver und Silver (Transformed)?",
            &json!({"canonical_name":"Silver"}),
            "hero_overview",
            3,
        )
        .unwrap();
        assert!(comparison["hero_context"].is_null());
        let result = search_game_wiki_for_answer(
            Some(root),
            "Welche Fähigkeiten hat Warden?",
            &json!({"canonical_name":"Warden","entity_type":"hero"}),
            "hero_overview",
            3,
        )
        .unwrap();
        let titles = result["matches"]
            .as_array()
            .unwrap()
            .iter()
            .map(|page| page["title"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            titles,
            vec![
                "Warden",
                "Alchemical Flask",
                "Willpower",
                "Binding Word",
                "Last Stand"
            ]
        );
        assert_eq!(result["hero_context"]["complete"], true);
        let item = search_game_wiki_for_answer(
            Some(root),
            "Wie funktioniert Spirit-Lifesteal?",
            &JsonValue::Null,
            "item_question",
            3,
        )
        .unwrap();
        assert!(item["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|page| page["title"] == "Spirit Lifesteal"));
        let combined = search_game_wiki_for_answer(
            Some(root),
            "Wie funktioniert Spirit Lifesteal bei Warden?",
            &json!({"canonical_name":"Warden","entity_type":"hero"}),
            "hero_overview",
            3,
        )
        .unwrap();
        assert!(combined["hero_context"].is_null());
        assert!(combined["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|page| page["title"] == "Spirit Lifesteal"));
        let unknown = search_game_wiki_for_answer(
            Some(root),
            "Blorplequarz",
            &JsonValue::Null,
            "item_question",
            3,
        )
        .unwrap();
        assert!(unknown["matches"].as_array().unwrap().is_empty());
        println!(
            "{}",
            json!({"hero":result,"other_heroes":other_heroes,"unique_ability_keys":keys.len(),"item_titles":item["matches"].as_array().unwrap().iter().map(|page| &page["title"]).collect::<Vec<_>>(),"unknown":unknown})
        );
    }

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

    #[test]
    fn replace_pages_atomically_restores_old_pages_when_new_pages_missing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let live_pages = tmp.path().join("pages");
        fs::create_dir_all(&live_pages).expect("live pages");
        fs::write(live_pages.join("old.md"), "old wiki").expect("old page");

        let result = replace_pages_atomically(tmp.path(), &tmp.path().join("missing/pages"));

        assert!(result.is_err());
        assert_eq!(
            fs::read_to_string(live_pages.join("old.md")).expect("old page restored"),
            "old wiki"
        );
    }
}
