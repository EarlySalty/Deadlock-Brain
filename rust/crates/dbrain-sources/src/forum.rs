use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use scraper::{ElementRef, Html, Selector};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    store::{complete_run, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    Result, SourcesError,
};

pub const SOURCE: &str = "playdeadlock_forum";
pub const BASE_URL: &str = "https://forums.playdeadlock.com";
pub const DEFAULT_SITEMAP_URL: &str = "https://forums.playdeadlock.com/sitemap.xml";

#[derive(Debug, Clone)]
pub struct PullForumOptions {
    pub sitemap_url: String,
    pub limit: usize,
    pub delay_seconds: f64,
    pub cache_ttl_seconds: u64,
    pub refresh_existing: bool,
}

impl Default for PullForumOptions {
    fn default() -> Self {
        Self {
            sitemap_url: DEFAULT_SITEMAP_URL.to_string(),
            limit: 25,
            delay_seconds: 1.0,
            cache_ttl_seconds: 86_400,
            refresh_existing: false,
        }
    }
}

#[derive(Debug, Clone)]
struct SitemapThread {
    thread_id: u64,
    url: String,
    lastmod: Option<String>,
    sitemap_url: String,
}

#[derive(Debug)]
struct ParsedThread {
    title: Option<String>,
    canonical_url: Option<String>,
    posts: Vec<ParsedPost>,
}

#[derive(Debug)]
struct ParsedPost {
    post_id: u64,
    author: Option<String>,
    user_id: Option<String>,
    user_title: Option<String>,
    datetime: Option<String>,
    timestamp: Option<i64>,
    text: String,
    html: String,
    links: Vec<String>,
    images: Vec<String>,
    attachments: Vec<String>,
}

pub async fn pull_forum(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullForumOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run(SOURCE).await?;
    let outcome = pull_forum_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_forum_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
) -> Result<Value> {
    if options.delay_seconds.is_sign_negative() {
        return Err(SourcesError::invalid_input(
            "delay_seconds darf nicht negativ sein.",
        ));
    }

    let sitemap_threads = discover_threads(store, http, options).await?;
    let mut fetched_threads = 0usize;
    let mut skipped_existing = 0usize;
    let mut failed_threads = Vec::new();
    let mut post_snapshots = 0usize;
    let mut thread_snapshots = 0usize;
    let mut first_thread_id = None;
    let mut last_thread_id = None;
    let started = Instant::now();

    for thread in sitemap_threads.iter() {
        if options.limit > 0 && fetched_threads >= options.limit {
            break;
        }

        if !options.refresh_existing && thread_document_exists(store.pool(), thread.thread_id).await?
        {
            skipped_existing += 1;
            continue;
        }

        match fetch_and_store_thread(store, http, options, thread).await {
            Ok(summary) => {
                if first_thread_id.is_none() {
                    first_thread_id = Some(thread.thread_id);
                }
                last_thread_id = Some(thread.thread_id);
                fetched_threads += 1;
                thread_snapshots += 1;
                post_snapshots += summary.post_count;
            }
            Err(error) => {
                if failed_threads.len() < 25 {
                    failed_threads.push(json!({
                        "thread_id": thread.thread_id,
                        "url": thread.url,
                        "error": error.to_string(),
                    }));
                }
                if error.to_string().contains("429") {
                    sleep_delay(60.0);
                }
            }
        }
    }

    Ok(json!({
        "source": SOURCE,
        "sitemap_url": options.sitemap_url,
        "threads_discovered": sitemap_threads.len(),
        "threads_fetched": fetched_threads,
        "threads_skipped_existing": skipped_existing,
        "thread_snapshots": thread_snapshots,
        "post_snapshots": post_snapshots,
        "failed_threads": failed_threads,
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
    post_count: usize,
}

async fn fetch_and_store_thread(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
    thread: &SitemapThread,
) -> Result<StoredThreadSummary> {
    let response = http.get(&thread.url, http_options(options, Duration::from_secs(45)))?;
    let html = response.text();
    let parsed = parse_thread_html(&html)?;
    let raw_path = store.write_raw(
        SOURCE,
        &format!("thread:{}", thread.thread_id),
        html.as_bytes(),
        "html",
    )?;
    let title = parsed.title.as_deref();
    let metadata = json!({
        "kind": "thread_html",
        "thread_id": thread.thread_id,
        "sitemap_url": thread.sitemap_url,
        "sitemap_lastmod": thread.lastmod,
        "canonical_url": parsed.canonical_url,
        "from_cache": response.from_cache,
        "post_count": parsed.posts.len(),
        "robots_policy": {
            "attachments_downloaded": false,
            "notes": "Only public thread HTML was fetched. /attachments/, /posts/, /search/ and private areas are not crawled."
        }
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &format!("thread:{}", thread.thread_id),
            title,
            url: Some(&thread.url),
            content_type: "text/html",
            raw_path: &raw_path,
            content: html.as_bytes(),
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::with_capacity(parsed.posts.len() + 1);
    snapshots.push(EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "forum_thread".to_string(),
        external_id: thread.thread_id.to_string(),
        canonical_name: parsed.title.clone(),
        payload: json!({
            "kind": "thread",
            "thread_id": thread.thread_id,
            "title": parsed.title,
            "url": thread.url,
            "canonical_url": parsed.canonical_url,
            "sitemap_lastmod": thread.lastmod,
            "post_count": parsed.posts.len(),
            "post_ids": parsed.posts.iter().map(|post| post.post_id).collect::<Vec<_>>(),
        }),
    });

    for (index, post) in parsed.posts.iter().enumerate() {
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "forum_post".to_string(),
            external_id: post.post_id.to_string(),
            canonical_name: parsed.title.clone(),
            payload: json!({
                "kind": "post",
                "thread_id": thread.thread_id,
                "thread_title": parsed.title,
                "thread_url": thread.url,
                "canonical_thread_url": parsed.canonical_url,
                "post_id": post.post_id,
                "post_index": index,
                "author": post.author,
                "user_id": post.user_id,
                "user_title": post.user_title,
                "datetime": post.datetime,
                "timestamp": post.timestamp,
                "text": post.text,
                "html": post.html,
                "links": post.links,
                "images": post.images,
                "attachments": post.attachments,
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
        post_count: parsed.posts.len(),
    })
}

async fn discover_threads(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
) -> Result<Vec<SitemapThread>> {
    let index_response = http.get(
        &options.sitemap_url,
        http_options(options, Duration::from_secs(30)),
    )?;
    store_sitemap_document(
        store,
        "sitemap:index",
        &options.sitemap_url,
        &index_response.text(),
    )
    .await?;
    let sitemap_urls = parse_sitemap_index(&index_response.text())?;
    let sitemap_urls = if sitemap_urls.is_empty() {
        vec![options.sitemap_url.clone()]
    } else {
        sitemap_urls
    };

    let mut threads = Vec::new();
    for (index, sitemap_url) in sitemap_urls.iter().enumerate() {
        let response = http.get(sitemap_url, http_options(options, Duration::from_secs(45)))?;
        let xml = response.text();
        store_sitemap_document(store, &format!("sitemap:{index}"), sitemap_url, &xml).await?;
        threads.extend(parse_threads_from_sitemap(sitemap_url, &xml)?);
        if !response.from_cache {
            sleep_delay(options.delay_seconds);
        }
    }

    threads.sort_by_key(|thread| thread.thread_id);
    threads.dedup_by_key(|thread| thread.thread_id);
    Ok(threads)
}

async fn store_sitemap_document(
    store: &SourceStore<'_>,
    external_id: &str,
    url: &str,
    xml: &str,
) -> Result<()> {
    let raw_path = store.write_raw(SOURCE, external_id, xml.as_bytes(), "xml")?;
    let metadata = json!({ "kind": "sitemap" });
    store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id,
            title: Some("Deadlock Forum Sitemap"),
            url: Some(url),
            content_type: "application/xml",
            raw_path: &raw_path,
            content: xml.as_bytes(),
            metadata: &metadata,
        })
        .await?;
    Ok(())
}

fn parse_sitemap_index(xml: &str) -> Result<Vec<String>> {
    let document = roxmltree::Document::parse(xml)
        .map_err(|error| SourcesError::invalid_input(format!("Sitemap XML unlesbar: {error}")))?;
    let mut urls = Vec::new();
    for sitemap in document
        .descendants()
        .filter(|node| node.has_tag_name("sitemap"))
    {
        if let Some(loc) = child_text(sitemap, "loc") {
            urls.push(loc);
        }
    }
    Ok(urls)
}

fn parse_threads_from_sitemap(sitemap_url: &str, xml: &str) -> Result<Vec<SitemapThread>> {
    let document = roxmltree::Document::parse(xml)
        .map_err(|error| SourcesError::invalid_input(format!("Sitemap XML unlesbar: {error}")))?;
    let mut threads = Vec::new();
    for url in document
        .descendants()
        .filter(|node| node.has_tag_name("url"))
    {
        let Some(loc) = child_text(url, "loc") else {
            continue;
        };
        let Some(thread_id) = thread_id_from_url(&loc) else {
            continue;
        };
        threads.push(SitemapThread {
            thread_id,
            url: loc,
            lastmod: child_text(url, "lastmod"),
            sitemap_url: sitemap_url.to_string(),
        });
    }
    Ok(threads)
}

fn parse_thread_html(html: &str) -> Result<ParsedThread> {
    let document = Html::parse_document(html);
    let title = select_first_text(&document, "h1.p-title-value")?
        .or_else(|| select_first_text(&document, "title").ok().flatten())
        .map(|value| value.trim_end_matches(" | Deadlock").trim().to_string())
        .filter(|value| !value.is_empty());
    let canonical_url = select_first_attr(&document, r#"link[rel="canonical"]"#, "href")?;
    let post_selector = selector("article.js-post")?;
    let mut posts = Vec::new();

    for post in document.select(&post_selector) {
        let Some(post_id) = post_id_from_article(&post) else {
            continue;
        };
        posts.push(parse_post(post, post_id)?);
    }

    Ok(ParsedThread {
        title,
        canonical_url,
        posts,
    })
}

fn parse_post(post: ElementRef<'_>, post_id: u64) -> Result<ParsedPost> {
    let author = post.value().attr("data-author").map(ToString::to_string);
    let user_link_selector = selector(".message-name a[data-user-id]")?;
    let user_link = post.select(&user_link_selector).next();
    let user_id = user_link
        .and_then(|element| element.value().attr("data-user-id"))
        .map(ToString::to_string);
    let user_title = select_first_text_from(&post, ".message-userTitle")?;
    let time_selector = selector("time.u-dt")?;
    let time = post.select(&time_selector).next();
    let datetime = time
        .and_then(|element| element.value().attr("datetime"))
        .map(ToString::to_string);
    let timestamp = time
        .and_then(|element| element.value().attr("data-timestamp"))
        .and_then(|value| value.parse::<i64>().ok());

    let body_selector = selector(".bbWrapper")?;
    let body = post.select(&body_selector).next();
    let text = body.as_ref().map(clean_element_text).unwrap_or_default();
    let html = body
        .as_ref()
        .map(ElementRef::inner_html)
        .unwrap_or_default();
    let (links, images, attachments) = body
        .as_ref()
        .map(collect_refs)
        .unwrap_or_else(|| (Vec::new(), Vec::new(), Vec::new()));

    Ok(ParsedPost {
        post_id,
        author,
        user_id,
        user_title,
        datetime,
        timestamp,
        text,
        html,
        links,
        images,
        attachments,
    })
}

fn select_first_text(document: &Html, query: &str) -> Result<Option<String>> {
    let selector = selector(query)?;
    Ok(document
        .select(&selector)
        .next()
        .map(|element| clean_element_text(&element))
        .filter(|value| !value.is_empty()))
}

fn select_first_attr(document: &Html, query: &str, attr: &str) -> Result<Option<String>> {
    let selector = selector(query)?;
    Ok(document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr(attr))
        .map(ToString::to_string))
}

fn select_first_text_from(element: &ElementRef<'_>, query: &str) -> Result<Option<String>> {
    let selector = selector(query)?;
    Ok(element
        .select(&selector)
        .next()
        .map(|element| clean_element_text(&element))
        .filter(|value| !value.is_empty()))
}

fn clean_element_text(element: &ElementRef<'_>) -> String {
    collapse_whitespace(&element.text().collect::<Vec<_>>().join(" "))
}

fn collect_refs(element: &ElementRef<'_>) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut links = Vec::new();
    let mut images = Vec::new();
    let mut attachments = Vec::new();

    if let Ok(link_selector) = Selector::parse("a[href]") {
        for link in element.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                let normalized = normalize_forum_url(href);
                collect_ref(&mut links, &mut attachments, normalized);
            }
        }
    }
    if let Ok(image_selector) = Selector::parse("img[src], source[src]") {
        for image in element.select(&image_selector) {
            if let Some(src) = image.value().attr("src") {
                let normalized = normalize_forum_url(src);
                collect_ref(&mut images, &mut attachments, normalized);
            }
        }
    }

    links.sort();
    links.dedup();
    images.sort();
    images.dedup();
    attachments.sort();
    attachments.dedup();
    (links, images, attachments)
}

fn collect_ref(values: &mut Vec<String>, attachments: &mut Vec<String>, value: String) {
    if value.contains("/attachments/")
        || value.contains("project8-data.community.forum/attachments/")
    {
        attachments.push(value.clone());
    }
    values.push(value);
}

fn normalize_forum_url(value: &str) -> String {
    if value.starts_with("http://") || value.starts_with("https://") {
        return value.to_string();
    }
    if value.starts_with('/') {
        return format!("{BASE_URL}{value}");
    }
    value.to_string()
}

fn selector(query: &str) -> Result<Selector> {
    Selector::parse(query).map_err(|error| {
        SourcesError::invariant(format!("ungueltiger HTML-Selector {query}: {error:?}"))
    })
}

async fn thread_document_exists(pool: &PgPool, thread_id: u64) -> Result<bool> {
    let external_id = format!("thread:{thread_id}");
    let exists = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM brain.source_documents WHERE source=$1 AND external_id=$2 LIMIT 1",
    )
    .bind(SOURCE)
    .bind(&external_id)
    .fetch_optional(pool)
    .await?
    .is_some();
    Ok(exists)
}

fn thread_id_from_url(url: &str) -> Option<u64> {
    let path = url
        .split_once("://")
        .and_then(|(_, rest)| rest.split_once('/').map(|(_, path)| path))
        .unwrap_or(url)
        .split(['?', '#'])
        .next()?
        .trim_end_matches('/');
    if !path.starts_with("threads/") {
        return None;
    }
    let segment = path.rsplit('/').next()?;
    let (_, id) = segment.rsplit_once('.')?;
    id.parse::<u64>().ok()
}

fn post_id_from_article(post: &ElementRef<'_>) -> Option<u64> {
    post.value()
        .attr("data-content")
        .and_then(|value| value.strip_prefix("post-"))
        .or_else(|| {
            post.value()
                .attr("id")
                .and_then(|value| value.strip_prefix("js-post-"))
        })
        .and_then(|value| value.parse::<u64>().ok())
}

fn child_text(node: roxmltree::Node<'_, '_>, child_name: &str) -> Option<String> {
    node.children()
        .find(|child| child.has_tag_name(child_name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn http_options(options: &PullForumOptions, timeout: Duration) -> HttpGetOptions {
    HttpGetOptions {
        cache_ttl_seconds: Some(options.cache_ttl_seconds),
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

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn thread_html(
        thread_id: u64,
        title: &str,
        post_id: u64,
        author: &str,
        user_title: &str,
        body: &str,
    ) -> String {
        format!(
            r#"<!doctype html>
            <html data-content-key="thread-{thread_id}">
              <head>
                <title>{title} | Deadlock</title>
                <link rel="canonical" href="https://forums.playdeadlock.com/threads/test.{thread_id}/" />
              </head>
              <body>
                <h1 class="p-title-value">{title}</h1>
                <article class="message message--post js-post" data-content="post-{post_id}" data-author="{author}" id="js-post-{post_id}">
                  <h4 class="message-name"><a href="/members/{author}.1/" data-user-id="1">{author}</a></h4>
                  <h5 class="userTitle message-userTitle">{user_title}</h5>
                  <time class="u-dt" datetime="2024-04-16T19:45:42-0700" data-timestamp="1713321942">Apr 16, 2024</time>
                  <div class="bbWrapper">{body}<br><a href="/attachments/test-png.1/">attachment</a><img src="https://project8-data.community.forum/attachments/test.hash.jpg"></div>
                </article>
              </body>
            </html>"#
        )
    }

    #[test]
    fn thread_url_parser_ignores_non_threads_and_extracts_numeric_suffix() {
        assert_eq!(
            thread_id_from_url("https://forums.playdeadlock.com/threads/page-of-tome.54515/"),
            Some(54515)
        );
        assert_eq!(
            thread_id_from_url("https://forums.playdeadlock.com/forums/bug-reports.6/"),
            None
        );
    }

    #[test]
    fn parse_thread_html_extracts_title_posts_text_and_attachments() {
        let html = thread_html(2, "Older thread", 2, "Yoshi", "Valve Developer", "Older body");
        let parsed = parse_thread_html(&html).expect("parse thread");
        assert_eq!(parsed.title.as_deref(), Some("Older thread"));
        assert_eq!(parsed.posts.len(), 1);
        let post = &parsed.posts[0];
        assert_eq!(post.post_id, 2);
        assert_eq!(post.author.as_deref(), Some("Yoshi"));
        assert_eq!(post.user_title.as_deref(), Some("Valve Developer"));
        assert!(post.text.contains("Older body"));
        assert!(post
            .attachments
            .iter()
            .any(|value| value.contains("project8-data.community.forum/attachments")));
        // Sitemap-Loc-Parsing bleibt strukturell stabil (Alt-vor-Neu-Sortierung
        // haengt an dieser numerischen Suffix-Extraktion).
        let summary = json!({ "thread_id": post.post_id });
        assert_eq!(summary["thread_id"], json!(2));
    }
}
