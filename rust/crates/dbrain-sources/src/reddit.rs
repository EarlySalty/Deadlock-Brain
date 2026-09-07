use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    store::{complete_run, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    Result, SourcesError,
};

pub const SOURCE: &str = "reddit";
pub const DEFAULT_SUBREDDIT: &str = "Deadlock";
pub const REDDIT_BASE_URL: &str = "https://www.reddit.com";

#[derive(Debug, Clone)]
pub struct PullRedditOptions {
    pub subreddits: Vec<String>,
    pub limit: usize,
    pub delay_seconds: f64,
    pub cache_ttl_seconds: u64,
    pub refresh_existing: bool,
}

impl Default for PullRedditOptions {
    fn default() -> Self {
        Self {
            subreddits: vec![DEFAULT_SUBREDDIT.to_string()],
            limit: 25,
            delay_seconds: 2.0,
            cache_ttl_seconds: 86_400,
            refresh_existing: false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ListingThread {
    pub(crate) subreddit: String,
    pub(crate) thread_id: String,
    pub(crate) title: String,
    pub(crate) author: Option<String>,
    pub(crate) author_flair: Option<String>,
    pub(crate) permalink: String,
    pub(crate) url: Option<String>,
    pub(crate) selftext: String,
    pub(crate) created_utc: Option<f64>,
    pub(crate) num_comments: i64,
    pub(crate) stickied: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedRedditThread {
    pub(crate) thread: ListingThread,
    pub(crate) comments: Vec<ParsedComment>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedComment {
    pub(crate) comment_id: String,
    pub(crate) author: Option<String>,
    pub(crate) author_flair: Option<String>,
    pub(crate) body: String,
    pub(crate) created_utc: Option<f64>,
    pub(crate) permalink: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct RssThread {
    pub(crate) subreddit: String,
    pub(crate) thread_id: String,
    pub(crate) title: String,
    pub(crate) author: Option<String>,
    pub(crate) content: String,
    pub(crate) permalink: String,
    pub(crate) updated: Option<String>,
    pub(crate) comments: Vec<RssComment>,
}

#[derive(Debug, Clone)]
pub(crate) struct RssComment {
    pub(crate) comment_id: String,
    pub(crate) author: Option<String>,
    pub(crate) content: String,
    pub(crate) updated: Option<String>,
    pub(crate) permalink: Option<String>,
}

pub(crate) fn parse_listing(listing_json: &str) -> Result<Vec<ListingThread>> {
    let value = parse_json(listing_json, "Reddit-Listing-JSON unlesbar")?;
    let children = value
        .get("data")
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
        .ok_or_else(|| SourcesError::invalid_input("Reddit-Listing ohne data.children."))?;
    let mut threads = Vec::new();
    for child in children {
        if child.get("kind").and_then(Value::as_str) != Some("t3") {
            continue;
        }
        let Some(data) = child.get("data") else {
            continue;
        };
        if let Some(thread) = listing_thread_from_data(data) {
            threads.push(thread);
        }
    }
    Ok(threads)
}

pub(crate) fn parse_listing_rss(xml: &str) -> Result<Vec<ListingThread>> {
    let document = parse_xml(xml)?;
    let mut threads = Vec::new();
    for entry in document.descendants().filter(|node| node.has_tag_name("entry")) {
        let title = child_text(&entry, "title")
            .map(ToString::to_string)
            .unwrap_or_default();
        let Some(link) = link_href(&entry) else {
            continue;
        };
        let Some(thread_id) = thread_id_from_permalink(&link) else {
            continue;
        };
        threads.push(ListingThread {
            subreddit: subreddit_from_permalink(&link),
            thread_id,
            title,
            author: entry_author(&entry),
            author_flair: None,
            permalink: permalink_url(&link),
            url: None,
            selftext: html_to_text(child_text(&entry, "content").unwrap_or_default()),
            created_utc: None,
            num_comments: 0,
            stickied: false,
        });
    }
    Ok(threads)
}

pub(crate) fn parse_thread(thread_json: &str) -> Result<ParsedRedditThread> {
    let value = parse_json(thread_json, "Reddit-Thread-JSON unlesbar")?;
    let pages = value
        .as_array()
        .ok_or_else(|| SourcesError::invalid_input("Reddit-Thread-JSON ist kein Array."))?;
    let thread = pages
        .first()
        .and_then(|listing| listing.get("data"))
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
        .and_then(|children| children.first())
        .filter(|child| child.get("kind").and_then(Value::as_str) == Some("t3"))
        .and_then(|child| child.get("data"))
        .and_then(listing_thread_from_data)
        .ok_or_else(|| SourcesError::invalid_input("Reddit-Thread-JSON ohne t3-Thread."))?;
    let mut comments = Vec::new();
    if let Some(children) = pages
        .get(1)
        .and_then(|page| page.get("data"))
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
    {
        collect_comments(children, &mut comments);
    }
    Ok(ParsedRedditThread { thread, comments })
}

pub(crate) fn parse_thread_rss(xml: &str) -> Result<Option<RssThread>> {
    let document = parse_xml(xml)?;
    let entries = document
        .descendants()
        .filter(|node| node.has_tag_name("entry"))
        .collect::<Vec<_>>();
    let mut submission: Option<(&roxmltree::Node, String, String)> = None;
    let mut comments = Vec::new();
    for entry in entries.iter() {
        let Some(link) = link_href(entry) else {
            continue;
        };
        let Some((thread_id, comment_id)) = split_reddit_permalink(&link) else {
            continue;
        };
        match comment_id {
            Some(comment_id) => comments.push(RssComment {
                comment_id,
                author: entry_author(entry),
                content: html_to_text(child_text(entry, "content").unwrap_or_default()),
                updated: child_text(entry, "updated").map(ToString::to_string),
                permalink: Some(permalink_url(&link)),
            }),
            None if submission.is_none() => submission = Some((entry, thread_id, link)),
            None => continue,
        }
    }
    let Some((first, thread_id, link)) = submission else {
        return Ok(None);
    };
    Ok(Some(RssThread {
        subreddit: subreddit_from_permalink(&link),
        thread_id,
        title: child_text(first, "title")
            .map(ToString::to_string)
            .unwrap_or_default(),
        author: entry_author(first),
        content: html_to_text(child_text(first, "content").unwrap_or_default()),
        permalink: permalink_url(&link),
        updated: child_text(first, "updated").map(ToString::to_string),
        comments,
    }))
}

pub(crate) fn thread_id_from_permalink(url: &str) -> Option<String> {
    split_reddit_permalink(url).map(|(thread_id, _)| thread_id)
}

fn split_reddit_permalink(link: &str) -> Option<(String, Option<String>)> {
    let path = link
        .split_once("://")
        .and_then(|(_, rest)| rest.split_once('/').map(|(_, path)| path))
        .unwrap_or(link)
        .split(['?', '#'])
        .next()?
        .trim_end_matches('/');
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let comments_index = segments.iter().position(|segment| *segment == "comments")?;
    let thread_id = segments.get(comments_index + 1)?;
    if thread_id.is_empty() || !thread_id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        return None;
    }
    let comment_id = segments.get(comments_index + 3).map(ToString::to_string);
    Some(((*thread_id).to_string(), comment_id))
}

fn listing_thread_from_data(data: &Value) -> Option<ListingThread> {
    let thread_id = value_string(data.get("id"))?;
    let permalink = value_string(data.get("permalink"))?;
    Some(ListingThread {
        subreddit: value_string(data.get("subreddit"))
            .unwrap_or_else(|| DEFAULT_SUBREDDIT.to_string()),
        thread_id,
        title: value_string(data.get("title")).unwrap_or_default(),
        author: value_string(data.get("author")),
        author_flair: value_string(data.get("author_flair_text")),
        permalink: permalink_url(&permalink),
        url: value_string(data.get("url")),
        selftext: value_string(data.get("selftext")).unwrap_or_default(),
        created_utc: data.get("created_utc").and_then(Value::as_f64),
        num_comments: data.get("num_comments").and_then(Value::as_i64).unwrap_or(0),
        stickied: data.get("stickied").and_then(Value::as_bool).unwrap_or(false),
    })
}

fn collect_comments(children: &[Value], out: &mut Vec<ParsedComment>) {
    for child in children {
        if child.get("kind").and_then(Value::as_str) != Some("t1") {
            continue;
        }
        let Some(data) = child.get("data") else {
            continue;
        };
        let Some(comment_id) = value_string(data.get("id")) else {
            continue;
        };
        out.push(ParsedComment {
            comment_id,
            author: value_string(data.get("author")),
            author_flair: value_string(data.get("author_flair_text")),
            body: value_string(data.get("body")).unwrap_or_default(),
            created_utc: data.get("created_utc").and_then(Value::as_f64),
            permalink: data
                .get("permalink")
                .and_then(Value::as_str)
                .map(permalink_url),
        });
        if let Some(replies) = data
            .get("replies")
            .and_then(Value::as_object)
            .and_then(|replies| replies.get("data"))
            .and_then(|replies| replies.get("children"))
            .and_then(Value::as_array)
        {
            collect_comments(replies, out);
        }
    }
}

fn permalink_url(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return trimmed.to_string();
    }
    if trimmed.starts_with('/') {
        return format!("{REDDIT_BASE_URL}{trimmed}");
    }
    format!("{REDDIT_BASE_URL}/{trimmed}")
}

fn subreddit_from_permalink(url: &str) -> String {
    let path = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    let segments = path.split(['?', '#']).next().unwrap_or(path);
    for window in segments.split('/').collect::<Vec<_>>().windows(2) {
        if window[0] == "r" && !window[1].is_empty() {
            return window[1].to_string();
        }
    }
    DEFAULT_SUBREDDIT.to_string()
}

fn parse_json(json: &str, context: &str) -> Result<Value> {
    serde_json::from_str(json)
        .map_err(|error| SourcesError::invalid_input(format!("{context}: {error}")))
}

fn parse_xml(xml: &str) -> Result<roxmltree::Document<'_>> {
    roxmltree::Document::parse(xml)
        .map_err(|error| SourcesError::invalid_input(format!("Reddit-RSS unlesbar: {error}")))
}

fn child_text<'a, 'doc>(node: &roxmltree::Node<'a, 'doc>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|child| child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn link_href(entry: &roxmltree::Node<'_, '_>) -> Option<String> {
    entry
        .children()
        .find(|child| child.has_tag_name("link"))
        .and_then(|link| link.attribute("href"))
        .map(ToString::to_string)
}

fn entry_author(entry: &roxmltree::Node<'_, '_>) -> Option<String> {
    entry
        .children()
        .find(|child| child.has_tag_name("author"))
        .and_then(|author| author.children().find(|child| child.has_tag_name("name")))
        .and_then(|name| name.text())
        .map(|name| name.trim().trim_start_matches("/u/").trim().to_string())
        .filter(|value| !value.is_empty())
}

fn html_to_text(html: &str) -> String {
    if html.trim().is_empty() {
        return String::new();
    }
    let fragment = scraper::Html::parse_fragment(html);
    let text = fragment
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join("");
    collapse_whitespace(&text)
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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

pub async fn pull_reddit(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullRedditOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run(SOURCE).await?;
    let outcome = pull_reddit_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_reddit_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullRedditOptions,
) -> Result<Value> {
    validate_options(options)?;
    let started = Instant::now();
    let mut threads_discovered = 0usize;
    let mut threads_fetched = 0usize;
    let mut threads_skipped_existing = 0usize;
    let mut thread_snapshots = 0usize;
    let mut comment_snapshots = 0usize;
    let mut rss_fallback_threads = 0usize;
    let mut failed_threads = Vec::new();
    let mut failed_listings = Vec::new();
    let mut first_thread_id = None;
    let mut last_thread_id = None;

    for subreddit in effective_subreddits(options) {
        let threads = match discover_threads(store, http, options, &subreddit).await {
            Ok(threads) => threads,
            Err(error) => {
                if failed_listings.len() < 25 {
                    failed_listings.push(json!({
                        "subreddit": subreddit,
                        "error": error.to_string(),
                    }));
                }
                continue;
            }
        };
        threads_discovered += threads.len();
        let mut fetched_in_subreddit = 0usize;

        for thread in threads.iter() {
            if options.limit > 0 && fetched_in_subreddit >= options.limit {
                break;
            }
            let external_id = thread_external_id(&thread.thread_id);
            let document_exists =
                thread_document_exists(store.pool(), &external_id).await?;
            if should_skip_existing(options.refresh_existing, document_exists) {
                threads_skipped_existing += 1;
                continue;
            }

            match fetch_and_store_thread(store, http, options, thread, &external_id).await {
                Ok(summary) => {
                    if first_thread_id.is_none() {
                        first_thread_id = Some(thread.thread_id.clone());
                    }
                    last_thread_id = Some(thread.thread_id.clone());
                    fetched_in_subreddit += 1;
                    threads_fetched += 1;
                    thread_snapshots += 1;
                    comment_snapshots += summary.comment_count;
                    rss_fallback_threads += summary.from_rss as usize;
                }
                Err(error) => {
                    if failed_threads.len() < 25 {
                        failed_threads.push(json!({
                            "thread_id": thread.thread_id,
                            "permalink": thread.permalink,
                            "error": error.to_string(),
                        }));
                    }
                    if error.to_string().contains("429") {
                        sleep_delay(60.0);
                    }
                }
            }
        }
    }

    Ok(json!({
        "source": SOURCE,
        "subreddits": effective_subreddits(options),
        "threads_discovered": threads_discovered,
        "threads_fetched": threads_fetched,
        "threads_skipped_existing": threads_skipped_existing,
        "thread_snapshots": thread_snapshots,
        "comment_snapshots": comment_snapshots,
        "rss_fallback_threads": rss_fallback_threads,
        "failed_threads": failed_threads,
        "failed_listings": failed_listings,
        "first_thread_id": first_thread_id,
        "last_thread_id": last_thread_id,
        "limit": options.limit,
        "delay_seconds": options.delay_seconds,
        "refresh_existing": options.refresh_existing,
        "elapsed_seconds": started.elapsed().as_secs_f64(),
    }))
}

#[derive(Debug, Clone, Copy)]
struct StoredThreadSummary {
    comment_count: usize,
    from_rss: bool,
}

async fn discover_threads(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullRedditOptions,
    subreddit: &str,
) -> Result<Vec<ListingThread>> {
    let json_url = listing_json_url(subreddit);
    if let Ok(response) = http.get(&json_url, http_options(options, Duration::from_secs(30))) {
        let body = response.text();
        if let Ok(threads) = parse_listing(&body) {
            if !threads.is_empty() {
                store_listing_document(
                    store,
                    &format!("listing:{subreddit}"),
                    &json_url,
                    "application/json",
                    "json",
                    &body,
                )
                .await?;
                if !response.from_cache {
                    sleep_delay(options.delay_seconds);
                }
                return Ok(threads);
            }
        }
    }

    let rss_url = listing_rss_url(subreddit);
    let response = http.get(&rss_url, http_options(options, Duration::from_secs(30)))?;
    let xml = response.text();
    let threads = parse_listing_rss(&xml)?;
    store_listing_document(
        store,
        &format!("listing-rss:{subreddit}"),
        &rss_url,
        "application/atom+xml",
        "xml",
        &xml,
    )
    .await?;
    if !response.from_cache {
        sleep_delay(options.delay_seconds);
    }
    Ok(threads)
}

async fn store_listing_document(
    store: &SourceStore<'_>,
    external_id: &str,
    url: &str,
    content_type: &str,
    suffix: &str,
    body: &str,
) -> Result<()> {
    let raw_path = store.write_raw(SOURCE, external_id, body.as_bytes(), suffix)?;
    let metadata = json!({ "kind": "listing" });
    store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id,
            title: Some("Reddit Listing"),
            url: Some(url),
            content_type,
            raw_path: &raw_path,
            content: body.as_bytes(),
            metadata: &metadata,
        })
        .await?;
    Ok(())
}

async fn fetch_and_store_thread(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullRedditOptions,
    thread: &ListingThread,
    external_id: &str,
) -> Result<StoredThreadSummary> {
    match fetch_thread_json(store, http, options, thread, external_id).await {
        Ok(summary) => Ok(summary),
        Err(json_error) => match fetch_thread_rss(store, http, options, thread, external_id).await
        {
            Ok(summary) => Ok(summary),
            Err(_) => Err(json_error),
        },
    }
}

async fn fetch_thread_json(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullRedditOptions,
    listing_thread: &ListingThread,
    external_id: &str,
) -> Result<StoredThreadSummary> {
    let url = thread_json_url(&listing_thread.subreddit, &listing_thread.thread_id);
    let response = http.get(&url, http_options(options, Duration::from_secs(45)))?;
    let body = response.text();
    let parsed = parse_thread(&body)?;
    let thread = &parsed.thread;
    let raw_path = store.write_raw(SOURCE, external_id, body.as_bytes(), "json")?;
    let metadata = json!({
        "kind": "thread_json",
        "thread_id": &thread.thread_id,
        "subreddit": &thread.subreddit,
        "permalink": &thread.permalink,
        "from_cache": response.from_cache,
        "comment_count": parsed.comments.len(),
        "robots_policy": {
            "writes_on_reddit": false,
            "notes": "Es wurden nur öffentliche JSON-/RSS-Daten ohne Login abgerufen. Kommentare über kind=more wurden nicht nachgeladen."
        }
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id,
            title: Some(&thread.title),
            url: Some(&thread.permalink),
            content_type: "application/json",
            raw_path: &raw_path,
            content: body.as_bytes(),
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::with_capacity(parsed.comments.len() + 1);
    snapshots.push(EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "reddit_thread".to_string(),
        external_id: thread.thread_id.clone(),
        canonical_name: Some(thread.title.clone()),
        payload: json!({
            "kind": "thread",
            "thread_id": &thread.thread_id,
            "subreddit": &thread.subreddit,
            "title": &thread.title,
            "permalink": &thread.permalink,
            "url": &thread.url,
            "author": &thread.author,
            "author_flair": &thread.author_flair,
            "created_utc": thread.created_utc,
            "posted_at": thread.created_utc.map(epoch_to_rfc3339),
            "num_comments": thread.num_comments,
            "stickied": thread.stickied,
            "text": &thread.selftext,
            "comment_count": parsed.comments.len(),
            "comment_ids": parsed.comments.iter().map(|comment| &comment.comment_id).collect::<Vec<_>>(),
            "from_rss": false,
        }),
    });

    for (index, comment) in parsed.comments.iter().enumerate() {
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "reddit_post".to_string(),
            external_id: comment.comment_id.clone(),
            canonical_name: Some(thread.title.clone()),
            payload: json!({
                "kind": "comment",
                "thread_id": &thread.thread_id,
                "thread_title": &thread.title,
                "subreddit": &thread.subreddit,
                "thread_permalink": &thread.permalink,
                "post_id": &comment.comment_id,
                "post_index": index,
                "author": &comment.author,
                "author_flair": &comment.author_flair,
                "created_utc": comment.created_utc,
                "posted_at": comment.created_utc.map(epoch_to_rfc3339),
                "permalink": &comment.permalink,
                "text": &comment.body,
                "from_rss": false,
            }),
        });
    }

    store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    if !response.from_cache {
        sleep_delay(options.delay_seconds);
    }

    Ok(StoredThreadSummary {
        comment_count: parsed.comments.len(),
        from_rss: false,
    })
}

async fn fetch_thread_rss(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullRedditOptions,
    listing_thread: &ListingThread,
    external_id: &str,
) -> Result<StoredThreadSummary> {
    let url = thread_rss_url(&listing_thread.subreddit, &listing_thread.thread_id);
    let response = http.get(&url, http_options(options, Duration::from_secs(45)))?;
    let xml = response.text();
    let Some(parsed) = parse_thread_rss(&xml)? else {
        return Err(SourcesError::invalid_input(
            "Reddit-Thread-RSS enthält keine Einträge.",
        ));
    };
    let raw_path = store.write_raw(SOURCE, external_id, xml.as_bytes(), "xml")?;
    let metadata = json!({
        "kind": "thread_rss",
        "thread_id": &parsed.thread_id,
        "subreddit": &parsed.subreddit,
        "permalink": &parsed.permalink,
        "from_cache": response.from_cache,
        "comment_count": parsed.comments.len(),
        "robots_policy": {
            "writes_on_reddit": false,
            "notes": "Es wurden nur öffentliche JSON-/RSS-Daten ohne Login abgerufen."
        }
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id,
            title: Some(&parsed.title),
            url: Some(&parsed.permalink),
            content_type: "application/atom+xml",
            raw_path: &raw_path,
            content: xml.as_bytes(),
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::with_capacity(parsed.comments.len() + 1);
    snapshots.push(EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "reddit_thread".to_string(),
        external_id: parsed.thread_id.clone(),
        canonical_name: Some(parsed.title.clone()),
        payload: json!({
            "kind": "thread",
            "thread_id": &parsed.thread_id,
            "subreddit": &parsed.subreddit,
            "title": &parsed.title,
            "permalink": &parsed.permalink,
            "url": Value::Null,
            "author": &parsed.author,
            "author_flair": Value::Null,
            "created_utc": Value::Null,
            "posted_at": &parsed.updated,
            "num_comments": parsed.comments.len(),
            "stickied": false,
            "text": &parsed.content,
            "comment_count": parsed.comments.len(),
            "comment_ids": parsed.comments.iter().map(|comment| &comment.comment_id).collect::<Vec<_>>(),
            "from_rss": true,
        }),
    });

    for (index, comment) in parsed.comments.iter().enumerate() {
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "reddit_post".to_string(),
            external_id: comment.comment_id.clone(),
            canonical_name: Some(parsed.title.clone()),
            payload: json!({
                "kind": "comment",
                "thread_id": &parsed.thread_id,
                "thread_title": &parsed.title,
                "subreddit": &parsed.subreddit,
                "thread_permalink": &parsed.permalink,
                "post_id": &comment.comment_id,
                "post_index": index,
                "author": &comment.author,
                "author_flair": Value::Null,
                "created_utc": Value::Null,
                "posted_at": &comment.updated,
                "permalink": &comment.permalink,
                "text": &comment.content,
                "from_rss": true,
            }),
        });
    }

    store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    if !response.from_cache {
        sleep_delay(options.delay_seconds);
    }

    Ok(StoredThreadSummary {
        comment_count: parsed.comments.len(),
        from_rss: true,
    })
}

async fn thread_document_exists(pool: &PgPool, external_id: &str) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM brain.source_documents WHERE source=$1 AND external_id=$2 LIMIT 1",
    )
    .bind(SOURCE)
    .bind(external_id)
    .fetch_optional(pool)
    .await?
    .is_some();
    Ok(exists)
}

fn validate_options(options: &PullRedditOptions) -> Result<()> {
    if options.delay_seconds.is_sign_negative() {
        return Err(SourcesError::invalid_input(
            "delay_seconds darf nicht negativ sein.",
        ));
    }
    Ok(())
}

fn should_skip_existing(refresh_existing: bool, document_exists: bool) -> bool {
    !refresh_existing && document_exists
}

fn thread_external_id(thread_id: &str) -> String {
    format!("thread:{thread_id}")
}

fn effective_subreddits(options: &PullRedditOptions) -> Vec<String> {
    if options.subreddits.is_empty() {
        vec![DEFAULT_SUBREDDIT.to_string()]
    } else {
        options.subreddits.clone()
    }
}

fn listing_json_url(subreddit: &str) -> String {
    format!("{REDDIT_BASE_URL}/r/{subreddit}/new.json?limit=100&raw_json=1")
}

fn listing_rss_url(subreddit: &str) -> String {
    format!("{REDDIT_BASE_URL}/r/{subreddit}/new/.rss")
}

fn thread_json_url(subreddit: &str, thread_id: &str) -> String {
    format!("{REDDIT_BASE_URL}/r/{subreddit}/comments/{thread_id}.json?raw_json=1")
}

fn thread_rss_url(subreddit: &str, thread_id: &str) -> String {
    format!("{REDDIT_BASE_URL}/r/{subreddit}/comments/{thread_id}.rss")
}

fn http_options(options: &PullRedditOptions, timeout: Duration) -> HttpGetOptions {
    HttpGetOptions {
        cache_ttl_seconds: Some(if options.refresh_existing {
            0
        } else {
            options.cache_ttl_seconds
        }),
        timeout,
        ..HttpGetOptions::default()
    }
}

fn sleep_delay(seconds: f64) {
    if seconds <= 0.0 {
        return;
    }
    thread::sleep(Duration::from_secs_f64(seconds));
}

fn epoch_to_rfc3339(epoch_seconds: f64) -> String {
    let total_seconds = epoch_seconds.floor() as i64;
    let days = total_seconds.div_euclid(86_400);
    let seconds_of_day = total_seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_period = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_period + 2) / 5 + 1;
    let month = if month_period < 10 {
        month_period + 3
    } else {
        month_period - 9
    };
    (
        if month <= 2 { year + 1 } else { year },
        month as u32,
        day as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listing_json() -> String {
        r#"{
          "kind": "Listing",
          "data": {
            "children": [
              {
                "kind": "t3",
                "data": {
                  "id": "1f2abc9",
                  "title": "Vyper feels weak after patch",
                  "selftext": "Winrate dropped hard since the last patch.",
                  "author": "lane_goblin",
                  "author_flair_text": "Vyper",
                  "subreddit": "Deadlock",
                  "created_utc": 1725700000.0,
                  "permalink": "/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/",
                  "url": "https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/",
                  "num_comments": 42,
                  "stickied": false
                }
              },
              {
                "kind": "t1",
                "data": { "id": "lmnoop", "body": "kein Thread" }
              },
              {
                "kind": "t3",
                "data": {
                  "id": "1f2def8",
                  "title": "Patch notes discussion",
                  "selftext": "",
                  "author": "AutoModerator",
                  "subreddit": "Deadlock",
                  "created_utc": 1725700100.0,
                  "permalink": "/r/Deadlock/comments/1f2def8/patch_notes_discussion/",
                  "num_comments": 7,
                  "stickied": true
                }
              }
            ]
          }
        }"#
        .to_string()
    }

    #[test]
    fn parse_listing_extracts_thread_id_title_selftext_author_and_permalink() {
        let threads = parse_listing(&listing_json()).expect("parse listing");
        assert_eq!(threads.len(), 2);
        let thread = &threads[0];
        assert_eq!(thread.thread_id, "1f2abc9");
        assert_eq!(thread.title, "Vyper feels weak after patch");
        assert_eq!(thread.selftext, "Winrate dropped hard since the last patch.");
        assert_eq!(thread.author.as_deref(), Some("lane_goblin"));
        assert_eq!(thread.author_flair.as_deref(), Some("Vyper"));
        assert_eq!(
            thread.permalink,
            "https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"
        );
        assert_eq!(thread.subreddit, "Deadlock");
        assert_eq!(thread.num_comments, 42);
        assert!(!thread.stickied);
        assert!(threads[1].stickied);
    }

    fn thread_json() -> String {
        r#"[
          {
            "kind": "Listing",
            "data": {
              "children": [
                {
                  "kind": "t3",
                  "data": {
                    "id": "1f2abc9",
                    "title": "Vyper feels weak after patch",
                    "selftext": "Winrate dropped hard since the last patch.",
                    "author": "lane_goblin",
                    "author_flair_text": "Vyper",
                    "subreddit": "Deadlock",
                    "created_utc": 1725700000.0,
                    "permalink": "/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/",
                    "num_comments": 2,
                    "stickied": false
                  }
                }
              ]
            }
          },
          {
            "kind": "Listing",
            "data": {
              "children": [
                {
                  "kind": "t1",
                  "data": {
                    "id": "lmroot1",
                    "author": "yoshi",
                    "author_flair_text": "Valve",
                    "body": "We are looking into it.",
                    "created_utc": 1725700100.0,
                    "permalink": "/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmroot1/",
                    "replies": {
                      "kind": "Listing",
                      "data": {
                        "children": [
                          {
                            "kind": "t1",
                            "data": {
                              "id": "lmleaf2",
                              "author": "stack_and_splash",
                              "body": "Thanks for the info!",
                              "created_utc": 1725700200.0,
                              "permalink": "/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmleaf2/",
                              "replies": ""
                            }
                          }
                        ]
                      }
                    }
                  }
                },
                {
                  "kind": "more",
                  "data": { "id": "lmhole3", "children": ["lmhidden4"] }
                }
              ]
            }
          }
        ]"#
        .to_string()
    }

    #[test]
    fn parse_thread_extracts_post_and_comments_and_ignores_more() {
        let parsed = parse_thread(&thread_json()).expect("parse thread");
        assert_eq!(parsed.thread.thread_id, "1f2abc9");
        assert_eq!(parsed.thread.title, "Vyper feels weak after patch");
        assert_eq!(parsed.comments.len(), 2);
        assert_eq!(parsed.comments[0].comment_id, "lmroot1");
        assert_eq!(parsed.comments[0].author.as_deref(), Some("yoshi"));
        assert_eq!(parsed.comments[0].author_flair.as_deref(), Some("Valve"));
        assert_eq!(parsed.comments[0].body, "We are looking into it.");
        assert_eq!(parsed.comments[0].created_utc, Some(1725700100.0));
        assert_eq!(parsed.comments[1].comment_id, "lmleaf2");
        assert!(parsed
            .comments
            .iter()
            .all(|comment| comment.comment_id != "lmhidden4"));
        assert!(parsed.comments[0].permalink.as_deref().is_some_and(|value| {
            value.starts_with("https://www.reddit.com/r/Deadlock/comments/1f2abc9/")
        }));
    }

    #[test]
    fn thread_permalink_parser_rejects_non_thread_urls() {
        assert_eq!(
            thread_id_from_permalink(
                "https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak/?context=3"
            ),
            Some("1f2abc9".to_string())
        );
        assert_eq!(
            thread_id_from_permalink("https://old.reddit.com/r/DeadlockNew/comments/abc123/"),
            Some("abc123".to_string())
        );
        assert_eq!(
            thread_id_from_permalink("https://www.reddit.com/r/Deadlock/new/"),
            None
        );
        assert_eq!(
            thread_id_from_permalink("https://www.reddit.com/r/Deadlock/comments/"),
            None
        );
        assert_eq!(
            thread_id_from_permalink("https://www.reddit.com/user/yoshi/posts/"),
            None
        );
    }

    fn listing_rss() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
          <entry>
            <title>Vyper feels weak after patch</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"/>
            <author><name>/u/lane_goblin</name></author>
            <content type="html">&lt;!-- SC_OFF --&gt;&lt;div class="md"&gt;&lt;p&gt;Winrate dropped &lt;strong&gt;hard&lt;/strong&gt;.&lt;/p&gt;&lt;/div&gt;&lt;!-- SC_ON --&gt;</content>
            <updated>2024-09-07T08:26:40+00:00</updated>
          </entry>
          <entry>
            <title>Daily bug thread</title>
            <link href="https://www.reddit.com/r/Deadlock/new/"/>
            <content type="html">&lt;p&gt;no thread&lt;/p&gt;</content>
          </entry>
        </feed>"#
        .to_string()
    }

    #[test]
    fn listing_rss_fallback_parses_title_link_and_content() {
        let threads = parse_listing_rss(&listing_rss()).expect("parse listing rss");
        assert_eq!(threads.len(), 1);
        let thread = &threads[0];
        assert_eq!(thread.thread_id, "1f2abc9");
        assert_eq!(thread.subreddit, "Deadlock");
        assert_eq!(thread.title, "Vyper feels weak after patch");
        assert_eq!(thread.author.as_deref(), Some("lane_goblin"));
        assert_eq!(
            thread.permalink,
            "https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"
        );
        assert_eq!(thread.selftext, "Winrate dropped hard.");
    }

    #[test]
    fn thread_rss_without_submission_entry_is_rejected() {
        let comments_only = r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
          <entry>
            <title>Comment by /u/yoshi</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmroot1/"/>
            <author><name>/u/yoshi</name></author>
            <content type="html">&lt;div class="md"&gt;&lt;p&gt;We are looking into it.&lt;/p&gt;&lt;/div&gt;</content>
          </entry>
        </feed>"#;
        let parsed = parse_thread_rss(comments_only).expect("parse thread rss");
        assert!(parsed.is_none());
    }

    #[test]
    fn thread_rss_keeps_comments_that_precede_the_submission_entry() {
        let comment_first = r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
          <entry>
            <title>Comment by /u/yoshi</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmroot1/"/>
            <author><name>/u/yoshi</name></author>
            <content type="html">&lt;div class="md"&gt;&lt;p&gt;We are looking into it.&lt;/p&gt;&lt;/div&gt;</content>
            <updated>2024-09-07T08:40:00+00:00</updated>
          </entry>
          <entry>
            <title>Vyper feels weak after patch</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"/>
            <author><name>/u/lane_goblin</name></author>
            <content type="html">&lt;div class="md"&gt;&lt;p&gt;Winrate dropped hard.&lt;/p&gt;&lt;/div&gt;</content>
            <updated>2024-09-07T08:26:40+00:00</updated>
          </entry>
        </feed>"#;
        let parsed = parse_thread_rss(comment_first)
            .expect("parse thread rss")
            .expect("thread present");
        assert_eq!(parsed.thread_id, "1f2abc9");
        assert_eq!(parsed.author.as_deref(), Some("lane_goblin"));
        assert_eq!(parsed.content, "Winrate dropped hard.");
        assert_eq!(parsed.comments.len(), 1);
        assert_eq!(parsed.comments[0].comment_id, "lmroot1");
        assert_eq!(parsed.comments[0].author.as_deref(), Some("yoshi"));
    }

    #[test]
    fn refresh_existing_bypasses_the_http_cache() {
        let mut options = PullRedditOptions::default();
        assert_eq!(
            http_options(&options, Duration::from_secs(30)).cache_ttl_seconds,
            Some(86_400)
        );
        options.refresh_existing = true;
        assert_eq!(
            http_options(&options, Duration::from_secs(30)).cache_ttl_seconds,
            Some(0)
        );
    }

    fn thread_rss() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
          <entry>
            <title>Vyper feels weak after patch</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"/>
            <author><name>/u/lane_goblin</name></author>
            <content type="html">&lt;div class="md"&gt;&lt;p&gt;Winrate dropped hard.&lt;/p&gt;&lt;/div&gt;</content>
            <updated>2024-09-07T08:26:40+00:00</updated>
          </entry>
          <entry>
            <title>Comment by /u/yoshi</title>
            <link href="https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmroot1/"/>
            <author><name>/u/yoshi</name></author>
            <content type="html">&lt;div class="md"&gt;&lt;p&gt;We are &lt;em&gt;looking&lt;/em&gt; into it.&lt;/p&gt;&lt;/div&gt;</content>
            <updated>2024-09-07T08:40:00+00:00</updated>
          </entry>
        </feed>"#
        .to_string()
    }

    #[test]
    fn thread_rss_fallback_parses_submission_and_comment_entries() {
        let parsed = parse_thread_rss(&thread_rss())
            .expect("parse thread rss")
            .expect("thread present");
        assert_eq!(parsed.thread_id, "1f2abc9");
        assert_eq!(parsed.subreddit, "Deadlock");
        assert_eq!(parsed.title, "Vyper feels weak after patch");
        assert_eq!(parsed.author.as_deref(), Some("lane_goblin"));
        assert_eq!(parsed.content, "Winrate dropped hard.");
        assert_eq!(parsed.updated.as_deref(), Some("2024-09-07T08:26:40+00:00"));
        assert_eq!(parsed.comments.len(), 1);
        let comment = &parsed.comments[0];
        assert_eq!(comment.comment_id, "lmroot1");
        assert_eq!(comment.author.as_deref(), Some("yoshi"));
        assert_eq!(comment.content, "We are looking into it.");
        assert_eq!(
            comment.permalink.as_deref(),
            Some("https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/lmroot1/")
        );
    }

    #[test]
    fn negative_delay_seconds_are_rejected() {
        let options = PullRedditOptions {
            delay_seconds: -0.5,
            ..Default::default()
        };
        let error = validate_options(&options).expect_err("negative delay");
        assert!(error.to_string().contains("negativ"));
        assert!(validate_options(&PullRedditOptions::default()).is_ok());
    }

    #[test]
    fn defaults_match_the_gentle_ingest_profile() {
        let options = PullRedditOptions::default();
        assert_eq!(options.subreddits, vec!["Deadlock".to_string()]);
        assert_eq!(options.limit, 25);
        assert_eq!(options.delay_seconds, 2.0);
        assert_eq!(options.cache_ttl_seconds, 86_400);
        assert!(!options.refresh_existing);
    }

    #[test]
    fn skip_existing_only_applies_without_refresh() {
        assert!(should_skip_existing(false, true));
        assert!(!should_skip_existing(true, true));
        assert!(!should_skip_existing(false, false));
        assert_eq!(
            thread_external_id("1f2abc9"),
            "thread:1f2abc9".to_string()
        );
    }

    #[test]
    fn empty_subreddits_fall_back_to_deadlock() {
        let mut options = PullRedditOptions::default();
        assert_eq!(
            effective_subreddits(&options),
            vec!["Deadlock".to_string()]
        );
        options.subreddits = vec![
            "DeadlockTheGame".to_string(),
            "DeadlockMemes".to_string(),
        ];
        assert_eq!(
            effective_subreddits(&options),
            vec![
                "DeadlockTheGame".to_string(),
                "DeadlockMemes".to_string()
            ]
        );
    }

    #[test]
    fn request_urls_target_the_public_json_and_rss_endpoints() {
        assert_eq!(
            listing_json_url("Deadlock"),
            "https://www.reddit.com/r/Deadlock/new.json?limit=100&raw_json=1"
        );
        assert_eq!(
            listing_rss_url("Deadlock"),
            "https://www.reddit.com/r/Deadlock/new/.rss"
        );
        assert_eq!(
            thread_json_url("Deadlock", "1f2abc9"),
            "https://www.reddit.com/r/Deadlock/comments/1f2abc9.json?raw_json=1"
        );
        assert_eq!(
            thread_rss_url("Deadlock", "1f2abc9"),
            "https://www.reddit.com/r/Deadlock/comments/1f2abc9.rss"
        );
    }

    #[test]
    fn created_utc_becomes_rfc3339_utc() {
        assert_eq!(epoch_to_rfc3339(0.0), "1970-01-01T00:00:00Z");
        assert_eq!(
            epoch_to_rfc3339(1_000_000_000.0),
            "2001-09-09T01:46:40Z"
        );
        assert_eq!(
            epoch_to_rfc3339(1_725_700_100.9),
            "2024-09-07T09:08:20Z"
        );
    }
}
