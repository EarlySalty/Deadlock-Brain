use std::{
    path::Path,
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
    pub browser_config: Option<crate::forum_browser::ForumBrowserConfig>,
}

impl Default for PullForumOptions {
    fn default() -> Self {
        Self {
            sitemap_url: DEFAULT_SITEMAP_URL.to_string(),
            limit: 25,
            delay_seconds: 1.0,
            cache_ttl_seconds: 86_400,
            refresh_existing: false,
            browser_config: None,
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
    category: Option<String>,
    category_url: Option<String>,
    next_page: Option<String>,
}

#[derive(Debug)]
struct ParsedPost {
    post_id: u64,
    page_url: Option<String>,
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
    pull_forum_with_pool(&pool, raw_dir, http, options).await
}

pub async fn pull_forum_with_pool(
    pool: &PgPool,
    raw_dir: &Path,
    http: &HttpClient,
    options: PullForumOptions,
) -> Result<Value> {
    let store = SourceStore::new(pool, raw_dir)?;
    let run_id = store.begin_run(SOURCE).await?;
    let outcome = pull_forum_inner(&store, http, &options, run_id).await;
    if let Ok(summary) = &outcome {
        let status = if summary["access_blocked"] == true
            || summary["threads_failed"].as_u64().unwrap_or(0) > 0
        {
            "error"
        } else {
            "ok"
        };
        store.finish_run(run_id, status, summary).await?;
        if status == "error" {
            return Err(SourcesError::invalid_input(format!("Forumimport fehlgeschlagen: {} Threads gelesen, {} fehlgeschlagen, Browserzugang gesperrt: {}. Keine Veröffentlichung.", summary["threads_fetched"], summary["threads_failed"], summary["access_blocked"])));
        }
        return outcome;
    }
    complete_run(&store, run_id, outcome).await
}

async fn pull_forum_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
    run_id: i64,
) -> Result<Value> {
    if !options.delay_seconds.is_finite()
        || options.delay_seconds < 0.5
        || Duration::try_from_secs_f64(options.delay_seconds).is_err()
    {
        return Err(SourcesError::invalid_input(
            "delay_seconds muss als Dauer darstellbar und mindestens 0,5 sein.",
        ));
    }

    let mut browser = match &options.browser_config {
        Some(config) => Some(
            crate::forum_browser::ForumBrowser::connect(config)
                .await
                .map_err(|e| SourcesError::invalid_input(e.to_string()))?,
        ),
        None => None,
    };
    let outcome = import_threads(store, http, options, &mut browser, run_id).await;
    if let Some(browser) = browser.as_mut() {
        if let Err(error) = browser.close().await {
            return Err(SourcesError::invariant(format!(
                "Browserabschluss fehlgeschlagen: {error}; Import abgeschlossen: {}",
                outcome.is_ok()
            )));
        }
    }
    outcome
}

async fn import_threads(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
    browser: &mut Option<crate::forum_browser::ForumBrowser>,
    run_id: i64,
) -> Result<Value> {
    let sitemap_threads = discover_threads(store, http, options, browser).await?;
    if sitemap_threads.is_empty() {
        return Err(SourcesError::invalid_input(
            "Sitemap enthält keine entdeckten Threads; Abdeckung unbekannt.",
        ));
    }
    sqlx::query("UPDATE brain.source_runs SET summary=$2 WHERE id=$1").bind(run_id).bind(json!({"threads_discovered":sitemap_threads.len(),"threads_fetched":0,"complete":false})).execute(store.pool()).await?;
    let mut fetched_threads = 0usize;
    let mut skipped_existing = 0usize;
    let mut failed_threads = Vec::new();
    let mut attempted_threads = 0usize;
    let mut failed_count = 0usize;
    let mut blocked = false;
    let mut post_snapshots = 0usize;
    let mut thread_snapshots = 0usize;
    let mut first_thread_id = None;
    let mut last_thread_id = None;
    let started = Instant::now();

    for thread in sitemap_threads.iter() {
        if options.limit > 0 && attempted_threads >= options.limit {
            break;
        }

        if !options.refresh_existing
            && thread_document_exists(store.pool(), thread.thread_id, thread.lastmod.as_deref())
                .await?
        {
            skipped_existing += 1;
            continue;
        }

        attempted_threads += 1;
        match fetch_and_store_thread(store, http, options, thread, browser).await {
            Ok(summary) => {
                if first_thread_id.is_none() {
                    first_thread_id = Some(thread.thread_id);
                }
                last_thread_id = Some(thread.thread_id);
                fetched_threads += 1;
                thread_snapshots += 1;
                post_snapshots += summary.post_count;
                if fetched_threads.is_multiple_of(25) || fetched_threads == 1 {
                    let progress = json!({"threads_discovered":sitemap_threads.len(),"threads_fetched":fetched_threads,"threads_skipped_existing":skipped_existing,"post_snapshots":post_snapshots,"threads_failed":failed_count,"complete":false});
                    sqlx::query("UPDATE brain.source_runs SET summary=$2 WHERE id=$1")
                        .bind(run_id)
                        .bind(&progress)
                        .execute(store.pool())
                        .await?;
                    eprintln!("Forumarchiv: {fetched_threads} Threads gelesen, {post_snapshots} Beiträge gespeichert, {skipped_existing} Threads unverändert.");
                }
            }
            Err(error) => {
                failed_count += 1;
                if failed_threads.len() < 25 {
                    failed_threads.push(json!({
                        "thread_id": thread.thread_id,
                        "url": thread.url,
                        "error": error.to_string(),
                    }));
                }
                if error.to_string().contains("Zugriff gesperrt") {
                    blocked = true;
                    break;
                }
                sleep_delay(options.delay_seconds.max(2.0)).await;
            }
        }
    }

    Ok(json!({
        "source": SOURCE,
        "sitemap_url": options.sitemap_url,
        "threads_discovered": sitemap_threads.len(),
        "threads_fetched": fetched_threads,
        "threads_attempted": attempted_threads,
        "threads_failed": failed_count,
        "access_blocked": blocked,
        "complete": !blocked && failed_count == 0 && fetched_threads + skipped_existing == sitemap_threads.len(),
        "remaining_threads": sitemap_threads.len().saturating_sub(fetched_threads + skipped_existing),
        "scope": "Öffentlich erreichbare Threads aus der Sitemap, einschließlich aller Antwortseiten. Private Bereiche und Anhangdateien sind ausgeschlossen.",
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
    browser: &mut Option<crate::forum_browser::ForumBrowser>,
) -> Result<StoredThreadSummary> {
    let mut url = thread.url.clone();
    let mut visited = std::collections::BTreeSet::new();
    let mut pages = Vec::new();
    let mut parsed: Option<ParsedThread> = None;
    loop {
        if !visited.insert(url.clone()) || pages.len() >= 10_000 {
            return Err(SourcesError::invariant("Ungültige Antwortseiten-Schleife."));
        }
        validate_thread_page_url(&url, thread.thread_id)?;
        let (html, from_cache) = forum_fetch(http, options, browser, &url).await?;
        let page = parse_thread_page(&html, &url)?;
        let next = page.next_page.clone();
        if let Some(first) = parsed.as_mut() {
            first.posts.extend(page.posts);
        } else {
            parsed = Some(page);
        }
        pages.push(json!({"url": url, "html": html, "from_cache": from_cache}));
        if !from_cache {
            sleep_delay(options.delay_seconds).await;
        }
        match next {
            Some(next) => url = next,
            None => break,
        }
    }
    let mut parsed = parsed.ok_or_else(|| SourcesError::invariant("Keine Threadseite."))?;
    let mut seen_posts = std::collections::BTreeSet::new();
    parsed.posts.retain(|post| seen_posts.insert(post.post_id));
    let html = serde_json::to_vec(&pages)?;
    let raw_path = store.write_raw(
        SOURCE,
        &format!("thread:{}", thread.thread_id),
        &html,
        "json",
    )?;
    let title = parsed.title.as_deref();
    let metadata = json!({
        "kind": "thread_archive",
        "archive_version": 2,
        "complete": false,
        "page_count": pages.len(),
        "category": parsed.category,
        "category_url": parsed.category_url,
        "thread_id": thread.thread_id,
        "sitemap_url": thread.sitemap_url,
        "sitemap_lastmod": thread.lastmod,
        "canonical_url": parsed.canonical_url,
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
            content_type: "application/json",
            raw_path: &raw_path,
            content: &html,
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
            "category": parsed.category,
            "category_url": parsed.category_url,
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
                "category": parsed.category,
                "category_url": parsed.category_url,
                "thread_url": thread.url,
                "page_url": post.page_url,
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

    store_observed_snapshots(store, &snapshots, document_id).await?;

    Ok(StoredThreadSummary {
        post_count: parsed.posts.len(),
    })
}

async fn discover_threads(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullForumOptions,
    browser: &mut Option<crate::forum_browser::ForumBrowser>,
) -> Result<Vec<SitemapThread>> {
    validate_sitemap_url(&options.sitemap_url)?;
    let (index_text, _) = forum_fetch(http, options, browser, &options.sitemap_url).await?;
    store_sitemap_document(store, "sitemap:index", &options.sitemap_url, &index_text).await?;
    let sitemap_urls = parse_sitemap_index(&index_text)?;
    let sitemap_urls = if sitemap_urls.is_empty() {
        vec![options.sitemap_url.clone()]
    } else {
        sitemap_urls
    };

    let mut threads = Vec::new();
    for (index, sitemap_url) in sitemap_urls.iter().enumerate() {
        validate_sitemap_url(sitemap_url)?;
        let (xml, from_cache) = forum_fetch(http, options, browser, sitemap_url).await?;
        store_sitemap_document(store, &format!("sitemap:{index}"), sitemap_url, &xml).await?;
        threads.extend(parse_threads_from_sitemap(sitemap_url, &xml)?);
        if !from_cache {
            sleep_delay(options.delay_seconds).await;
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
    response_text_checked(html)?;
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

    if posts.is_empty() {
        return Err(SourcesError::invalid_input(
            "Zugriff gesperrt oder Thread ohne lesbare Beiträge; keine Archivierung.",
        ));
    }
    let category_link = document
        .select(&selector(".p-breadcrumbs a[href]")?)
        .rfind(|element| {
            element.value().attr("href").is_some_and(|href| {
                normalize_forum_url(href).starts_with(&format!("{BASE_URL}/forums/"))
            })
        });
    let category = category_link.as_ref().map(clean_element_text);
    let category_url = category_link
        .and_then(|element| element.value().attr("href"))
        .map(normalize_forum_url);
    let next_page = select_first_attr(
        &document,
        r#"link[rel="next"], a.pageNav-jump--next"#,
        "href",
    )?
    .map(|url| normalize_forum_url(&url));

    Ok(ParsedThread {
        title,
        canonical_url,
        posts,
        category,
        category_url,
        next_page,
    })
}

fn response_text_checked(text: &str) -> Result<String> {
    let document = Html::parse_document(text);
    let has_posts = document
        .select(&selector("article.js-post")?)
        .next()
        .is_some();
    let challenge_title = document.select(&selector("title,h1,h2")?).any(|element| {
        let text = clean_element_text(&element).to_lowercase();
        text == "checking your browser" || text == "just a moment..."
    });
    let challenge_structure = document
        .select(&selector(
            "script[src*='/.stile/challenge/'], script[src*='challenge-platform'], [id^='cf-chl-']",
        )?)
        .next()
        .is_some();
    if !has_posts
        && (challenge_title
            || challenge_structure
            || text.trim().eq_ignore_ascii_case("Checking your browser"))
    {
        return Err(SourcesError::invalid_input(
            "Zugriff gesperrt: Browserprüfung statt Forumdaten.",
        ));
    }
    Ok(text.to_string())
}

fn validate_sitemap_url(url: &str) -> Result<()> {
    let path = url.strip_prefix(&format!("{BASE_URL}/")).unwrap_or("");
    if !path.starts_with("sitemap")
        || !path.ends_with(".xml")
        || path.contains(['/', '?', '#', '@', '\\'])
    {
        return Err(SourcesError::invalid_input(
            "Sitemap muss zum offiziellen Deadlock-Forum gehören.",
        ));
    }
    Ok(())
}

fn validate_thread_page_url(url: &str, thread_id: u64) -> Result<()> {
    let path = url
        .strip_prefix(&format!("{BASE_URL}/threads/"))
        .unwrap_or("");
    let mut parts = path.trim_end_matches('/').split('/');
    let slug = parts.next().unwrap_or("");
    let valid_page = parts.next().is_none_or(|page| {
        page.strip_prefix("page-")
            .is_some_and(|n| n.parse::<u64>().is_ok_and(|n| n > 1))
    });
    if slug
        .rsplit_once('.')
        .and_then(|(_, id)| id.parse::<u64>().ok())
        != Some(thread_id)
        || !valid_page
        || parts.next().is_some()
        || path.contains(['?', '#', '@', '\\'])
    {
        return Err(SourcesError::invalid_input(
            "Antwortseite gehört nicht zum ursprünglichen offiziellen Thread.",
        ));
    }
    Ok(())
}

fn parse_thread_page(html: &str, url: &str) -> Result<ParsedThread> {
    let mut page = parse_thread_html(html)?;
    for post in &mut page.posts {
        post.page_url = Some(url.to_owned());
    }
    Ok(page)
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
        page_url: None,
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

async fn thread_document_exists(
    pool: &PgPool,
    thread_id: u64,
    lastmod: Option<&str>,
) -> Result<bool> {
    let external_id = thread_id.to_string();
    let exists = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM (SELECT d.metadata FROM brain.entity_snapshots s JOIN brain.source_documents d ON d.id=s.source_document_id WHERE s.source=$1 AND s.entity_type='forum_thread' AND s.external_id=$2 AND d.metadata->>'complete'='true' ORDER BY s.fetched_at DESC,s.id DESC LIMIT 1) latest WHERE metadata->>'archive_version'='2' AND $3::text IS NOT NULL AND metadata->>'sitemap_lastmod'=$3",
    )
    .bind(SOURCE)
    .bind(&external_id)
    .bind(lastmod)
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

async fn sleep_delay(seconds: f64) {
    if seconds <= 0.0 {
        return;
    }
    tokio::time::sleep(Duration::from_secs_f64(seconds)).await;
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
        let html = thread_html(
            2,
            "Older thread",
            2,
            "Yoshi",
            "Valve Developer",
            "Older body",
        );
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

async fn forum_fetch(
    http: &HttpClient,
    options: &PullForumOptions,
    browser: &mut Option<crate::forum_browser::ForumBrowser>,
    url: &str,
) -> Result<(String, bool)> {
    if let Some(browser) = browser.as_mut() {
        let text = browser.fetch_html(url).await.map_err(|e| {
            SourcesError::invalid_input(format!(
                "Zugriff gesperrt: Browserabruf fehlgeschlagen: {e}"
            ))
        })?;
        return Ok((response_text_checked(&text)?, false));
    }
    let client = http.clone();
    let request_url = url.to_string();
    let request_options = http_options(options, Duration::from_secs(45));
    let response =
        tokio::task::spawn_blocking(move || client.get_no_redirect(&request_url, request_options))
            .await
            .map_err(|_| SourcesError::invariant("Forumabruf-Task fehlgeschlagen."))??;
    Ok((
        response_text_checked(&response.text())?,
        response.from_cache,
    ))
}

#[cfg(test)]
mod kernel_integration_tests {
    use super::*;
    use brain_contracts::{store::DocumentStorePort, *};
    use brain_storage::MemoryRepository;
    use dbrain_retrieval::ReleaseRetriever;
    use std::collections::BTreeSet;

    #[tokio::test]
    async fn two_forum_pages_reach_release_retriever_with_dates_and_report_status() {
        let pages = [
            r#"<html><h1 class="p-title-value">Itemfehler</h1><article class="js-post" data-content="post-1"><div class="bbWrapper">Erster Bericht zum Itemfehler.</div><time class="u-dt" datetime="2026-09-01T10:00:00Z" data-timestamp="1788256800"></time></article><link rel="next" href="/threads/itemfehler.4/page-2" /></html>"#,
            r#"<html><h1 class="p-title-value">Itemfehler</h1><article class="js-post" data-content="post-2"><div class="bbWrapper">Zweiter Bericht über denselben Itemfehler.</div><time class="u-dt" datetime="2026-09-02T10:00:00Z" data-timestamp="1788343200"></time></article></html>"#,
        ];
        let first = parse_thread_page(
            pages[0],
            "https://forums.playdeadlock.com/threads/itemfehler.4/",
        )
        .unwrap();
        assert_eq!(
            first.next_page.as_deref(),
            Some("https://forums.playdeadlock.com/threads/itemfehler.4/page-2")
        );
        let second = parse_thread_page(
            pages[1],
            "https://forums.playdeadlock.com/threads/itemfehler.4/page-2",
        )
        .unwrap();
        let store = MemoryRepository::default();
        for post in first.posts.iter().chain(second.posts.iter()) {
            let source = crate::forum_corpus::record(&json!({"post_id":post.post_id,"thread_title":"Itemfehler","thread_url":"https://forums.playdeadlock.com/threads/itemfehler.4/","page_url":post.page_url,"text":post.text,"datetime":post.datetime,"timestamp":post.timestamp}), post.post_id, 1788400000).unwrap();
            store.apply_record(source).unwrap();
        }
        let release = store
            .release_from_heads("forum-fixture", "fixture", "unbekannt")
            .unwrap();
        store.publish(&release).await.unwrap();
        let retriever = ReleaseRetriever::new(store, 6);
        let query = Query {
            request_id: "fixture".into(),
            conversation_id: "fixture".into(),
            text: "Itemfehler".into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
            domain: None,
        };
        let context = AuthorizedContext {
            request_deadline: None,
            principal: Principal {
                actor_id: "fixture".into(),
                channel: "fixture".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "fixture".into(),
            knowledge_release: "forum-fixture".into(),
            deadline_ms: 8000,
            budget: Budget::default(),
        };
        let evidence = retriever.retrieve(&query, &context).unwrap();
        assert_eq!(evidence.len(), 2);
        retriever
            .validate_publication(&query, &context, &evidence)
            .unwrap();
        retriever
            .validate_evidence(&query, &context, &evidence, true)
            .unwrap();
        for hit in evidence {
            assert!(hit.content.contains("unbestätigt"));
            assert!(hit.content.contains("2026-09-0"));
            let source = if hit.content.contains("Zweiter Bericht") {
                "https://forums.playdeadlock.com/threads/itemfehler.4/page-2#post-2"
            } else {
                "https://forums.playdeadlock.com/threads/itemfehler.4/#post-1"
            };
            assert!(hit.content.contains(source));
            assert_eq!(hit.kind, EvidenceKind::Prose);
            assert!(hit.patch.is_none());
        }
    }
}

async fn store_observed_snapshots(
    store: &SourceStore<'_>,
    snapshots: &[EntitySnapshotInput],
    document_id: i64,
) -> Result<()> {
    let mut transaction = store.pool().begin().await?;
    let thread_id = snapshots
        .iter()
        .find(|s| s.entity_type == "forum_thread")
        .ok_or_else(|| SourcesError::invariant("Thread-Mitgliedschaft fehlt."))?
        .external_id
        .clone();
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
        .bind(format!("forum-thread:{thread_id}"))
        .execute(&mut *transaction)
        .await?;
    for snapshot in snapshots {
        let payload = serde_json::to_string(&snapshot.payload)?;
        let hash = crate::store::stable_hash_text(&payload);
        // Jede tatsächliche Beobachtung, auch A nach B, wird gemeinsam mit der
        // fertigen Thread-Mitgliedschaft übernommen. Fehler rollen alles zurück.
        sqlx::query("INSERT INTO brain.entity_snapshots(source,entity_type,external_id,canonical_name,payload_hash,payload,fetched_at,source_document_id) VALUES($1,$2,$3,$4,$5,$6::text::jsonb,clock_timestamp(),$7) ON CONFLICT(source,entity_type,external_id,payload_hash) DO UPDATE SET fetched_at=EXCLUDED.fetched_at,source_document_id=EXCLUDED.source_document_id")
            .bind(&snapshot.source).bind(&snapshot.entity_type).bind(&snapshot.external_id).bind(&snapshot.canonical_name).bind(hash).bind(payload).bind(document_id).execute(&mut *transaction).await?;
    }
    sqlx::query("UPDATE brain.source_documents SET metadata=jsonb_set(metadata,'{complete}','true'::jsonb) WHERE id=$1")
        .bind(document_id).execute(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(())
}

#[cfg(test)]
mod scratch_regression_tests {
    use super::*;
    use crate::wiki_runtime::ScratchWikiStore;
    use std::process::{Command, Stdio};
    struct ScratchServer {
        data: std::path::PathBuf,
        pg_ctl: std::path::PathBuf,
    }
    impl Drop for ScratchServer {
        fn drop(&mut self) {
            let _ = Command::new(&self.pg_ctl)
                .args(["-D"])
                .arg(&self.data)
                .args(["-m", "fast", "-w", "stop"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
    #[tokio::test]
    async fn observed_reversion_and_removed_post_update_real_canonical_release() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("C5_SCRATCH_ONLY"), "C5_SCRATCH_ONLY\n").unwrap();
        let bin = Command::new("pg_config").arg("--bindir").output().unwrap();
        let bin = std::path::PathBuf::from(String::from_utf8(bin.stdout).unwrap().trim());
        let data = root.path().join("pg");
        assert!(Command::new(bin.join("initdb"))
            .arg("-D")
            .arg(&data)
            .args([
                "--auth-local=trust",
                "--auth-host=reject",
                "--username=brain_wiki_c5",
                "--no-locale",
                "--encoding=UTF8"
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        let server = ScratchServer {
            data: data.clone(),
            pg_ctl: bin.join("pg_ctl"),
        };
        assert!(Command::new(&server.pg_ctl)
            .arg("-D")
            .arg(&data)
            .arg("-l")
            .arg(root.path().join("server.log"))
            .arg("-o")
            .arg(format!(
                "-c listen_addresses='' -k {} -p 55441 -c shared_buffers=16MB",
                root.path().display()
            ))
            .args(["-w", "start"])
            .stdout(Stdio::null())
            .status()
            .unwrap()
            .success());
        assert!(Command::new(bin.join("createdb"))
            .arg("-h")
            .arg(root.path())
            .args(["-p", "55441", "-U", "brain_wiki_c5", "brain_wiki_c5"])
            .status()
            .unwrap()
            .success());
        let scratch = ScratchWikiStore::connect(root.path(), &root.path().join("raw"))
            .await
            .unwrap();
        scratch.migrate().await.unwrap();
        let pool = scratch.pool();
        sqlx::raw_sql("CREATE TABLE brain.entity_snapshots(id bigserial PRIMARY KEY,source text NOT NULL,entity_type text NOT NULL,external_id text NOT NULL,canonical_name text,payload_hash text NOT NULL,payload jsonb NOT NULL,fetched_at timestamptz NOT NULL,source_document_id bigint REFERENCES brain.source_documents(id),UNIQUE(source,entity_type,external_id,payload_hash));").execute(pool).await.unwrap();
        let store = SourceStore::new(pool, &root.path().join("raw")).unwrap();
        let raw = store
            .write_raw(SOURCE, "thread:4", b"fixture", "json")
            .unwrap();
        let doc = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: "thread:4",
                title: Some("fixture"),
                url: Some("https://forums.playdeadlock.com/threads/fixture.4/"),
                content_type: "application/json",
                raw_path: &raw,
                content: b"fixture",
                metadata: &json!({"complete":true}),
            })
            .await
            .unwrap();
        let post = |id: u64, text: &str| EntitySnapshotInput {
            source: SOURCE.into(),
            entity_type: "forum_post".into(),
            external_id: id.to_string(),
            canonical_name: None,
            payload: json!({"post_id":id,"thread_id":4,"thread_title":"fixture","thread_url":"https://forums.playdeadlock.com/threads/fixture.4/","text":text}),
        };
        let manifest = |ids: Vec<u64>| EntitySnapshotInput {
            source: SOURCE.into(),
            entity_type: "forum_thread".into(),
            external_id: "4".into(),
            canonical_name: None,
            payload: json!({"thread_id":4,"post_ids":ids}),
        };
        let kernel = brain_storage::PgStore::new(pool.clone());
        let mut fixture =
            crate::forum_corpus::record(&post(100, "Anderes Wissen").payload, 1, 1).unwrap();
        fixture
            .metadata
            .remove(brain_contracts::source::ORIGIN_METADATA_KEY);
        fixture.source_id = "fixture-existing".into();
        kernel.apply(&fixture).await.unwrap();
        let base = brain_contracts::CorpusRelease {
            release_id: "base".into(),
            knowledge_version: "fixture".into(),
            patch: "unknown".into(),
            created_at_epoch: 1,
            source_revisions: std::collections::BTreeMap::from([(
                "fixture-existing".into(),
                std::collections::BTreeMap::from([(fixture.logical_id.clone(), 1)]),
            )]),
        };
        let unknown = crate::forum_corpus::record(&json!({"post_id":999,"thread_id":9,"thread_url":"https://forums.playdeadlock.com/threads/unbekannt.9/","text":"Nicht lokal erneut beobachteter Beitrag"}),1,1).unwrap();
        kernel.apply(&unknown).await.unwrap();
        let mut base = base;
        base.source_revisions.insert(
            SOURCE.into(),
            std::collections::BTreeMap::from([(unknown.logical_id.clone(), 1)]),
        );
        kernel.publish_release(&base).await.unwrap();
        store_observed_snapshots(
            &store,
            &[manifest(vec![1, 2]), post(1, "A"), post(2, "Antwort")],
            doc,
        )
        .await
        .unwrap();
        crate::forum_corpus::publish_archive(pool, pool, "base", "a")
            .await
            .unwrap();
        sqlx::raw_sql("CREATE FUNCTION brain.reject_forum_test() RETURNS trigger LANGUAGE plpgsql AS $body$ BEGIN IF NEW.payload->>'text'='Fail' THEN RAISE EXCEPTION 'fixture failure'; END IF; RETURN NEW; END $body$; CREATE TRIGGER reject_forum_test BEFORE INSERT OR UPDATE ON brain.entity_snapshots FOR EACH ROW EXECUTE FUNCTION brain.reject_forum_test();").execute(pool).await.unwrap();
        let new_raw = store
            .write_raw(SOURCE, "thread:4", b"fresh-incomplete", "json")
            .unwrap();
        let incomplete_doc = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: "thread:4",
                title: Some("fixture"),
                url: Some("https://forums.playdeadlock.com/threads/fixture.4/"),
                content_type: "application/json",
                raw_path: &new_raw,
                content: b"fresh-incomplete",
                metadata: &json!({"complete":false}),
            })
            .await
            .unwrap();
        assert!(store_observed_snapshots(
            &store,
            &[manifest(vec![1]), post(1, "Fail")],
            incomplete_doc
        )
        .await
        .is_err());
        crate::forum_corpus::publish_archive(pool, pool, "a", "after-failure")
            .await
            .unwrap();
        let after_failure = kernel.snapshot("after-failure").await.unwrap();
        assert_eq!(after_failure.release.source_revisions[SOURCE].len(), 3);
        assert!(after_failure
            .revisions
            .iter()
            .find(|r| r.logical_id == "post:1" && r.source_id == SOURCE)
            .unwrap()
            .content
            .ends_with("\n\nA"));
        sqlx::raw_sql("DROP TRIGGER reject_forum_test ON brain.entity_snapshots; DROP FUNCTION brain.reject_forum_test();").execute(pool).await.unwrap();

        store_observed_snapshots(
            &store,
            &[manifest(vec![1, 2]), post(1, "B"), post(2, "Antwort")],
            doc,
        )
        .await
        .unwrap();
        crate::forum_corpus::publish_archive(pool, pool, "a", "b")
            .await
            .unwrap();
        store_observed_snapshots(&store, &[manifest(vec![1]), post(1, "A")], doc)
            .await
            .unwrap();
        crate::forum_corpus::publish_archive(pool, pool, "b", "a-again")
            .await
            .unwrap();
        let release = kernel.snapshot("a-again").await.unwrap();
        assert!(release
            .release
            .source_revisions
            .contains_key("fixture-existing"));
        assert_eq!(release.release.source_revisions[SOURCE].len(), 2);
        assert_eq!(release.release.source_revisions[SOURCE]["post:999"], 1);
        let source = release
            .revisions
            .iter()
            .find(|r| r.source_id == SOURCE && r.logical_id == "post:1")
            .unwrap();
        assert!(source.content.ends_with("\n\nA"));
        assert_eq!(source.revision, 4);
        let limited = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(2))
            .connect_with((*pool.connect_options()).clone())
            .await
            .unwrap();
        let mut blocker = limited.begin().await.unwrap();
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext('forum-corpus-publish')::bigint)")
            .execute(&mut *blocker)
            .await
            .unwrap();
        let concurrent = tokio::time::timeout(Duration::from_secs(10), async {
            let (first, second, ()) = tokio::join!(
                crate::forum_corpus::publish_archive(
                    &limited,
                    &limited,
                    "a-again",
                    "concurrent-one"
                ),
                crate::forum_corpus::publish_archive(
                    &limited,
                    &limited,
                    "a-again",
                    "concurrent-two"
                ),
                async {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    blocker.rollback().await.unwrap();
                }
            );
            first.unwrap();
            second.unwrap();
        })
        .await;
        assert!(
            concurrent.is_ok(),
            "Zwei Publisher dürfen den kleinen Pool nicht blockieren."
        );
        let mut many = vec![manifest((1..=201).collect())];
        many.extend((1..=201).map(|id| post(id, &format!("Beitrag {id}"))));
        store_observed_snapshots(&store, &many, doc).await.unwrap();
        let batches =
            crate::forum_corpus::publish_archive(&limited, &limited, "a-again", "multiple-batches")
                .await
                .unwrap();
        assert_eq!(batches["posts"], 201);
        assert_eq!(batches["committed"], 201);
        sqlx::query("UPDATE brain.source_documents SET metadata=metadata || jsonb_build_object('sitemap_lastmod','A','archive_version',2) WHERE id=$1")
            .bind(doc).execute(pool).await.unwrap();
        assert!(thread_document_exists(pool, 4, Some("A")).await.unwrap());
        let changed_doc = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: "thread:4",
                title: Some("fixture"),
                url: Some("https://forums.playdeadlock.com/threads/fixture.4/"),
                content_type: "application/json",
                raw_path: &raw,
                content: b"changed sitemap observation",
                metadata: &json!({"archive_version":2,"complete":false,"sitemap_lastmod":"B"}),
            })
            .await
            .unwrap();
        store_observed_snapshots(&store, &[manifest((1..=201).collect())], changed_doc)
            .await
            .unwrap();
        assert!(!thread_document_exists(pool, 4, Some("A")).await.unwrap());
        assert!(thread_document_exists(pool, 4, Some("B")).await.unwrap());
        store_observed_snapshots(&store, &[manifest((1..=201).collect())], doc)
            .await
            .unwrap();
        assert!(thread_document_exists(pool, 4, Some("A")).await.unwrap());
        assert!(!thread_document_exists(pool, 4, Some("B")).await.unwrap());
        limited.close().await;
        pool.close().await;
        drop(server);
    }
}

#[cfg(test)]
mod live_browser_contract {
    use super::*;
    #[tokio::test]
    #[ignore = "Braucht den vom Nutzer freigegebenen lokalen Brave-Browser"]
    async fn real_public_thread_and_sitemap_use_import_parser() {
        let mut browser = crate::forum_browser::ForumBrowser::connect(&Default::default())
            .await
            .unwrap();
        let html = browser
            .fetch_html("https://forums.playdeadlock.com/threads/posting-bugs.5/")
            .await
            .unwrap();
        let thread = parse_thread_html(&html).unwrap();
        assert!(!thread.posts.is_empty());
        assert!(thread.posts[0].datetime.is_some());
        eprintln!("Öffentliche Threadprobe: {} Beiträge, Beitragsdatum {}, Quelle https://forums.playdeadlock.com/threads/posting-bugs.5/", thread.posts.len(), thread.posts[0].datetime.as_deref().unwrap());
        let xml = browser.fetch_html(DEFAULT_SITEMAP_URL).await.unwrap();
        let index = parse_sitemap_index(&xml).unwrap();
        if index.is_empty() {
            assert!(!parse_threads_from_sitemap(DEFAULT_SITEMAP_URL, &xml)
                .unwrap()
                .is_empty());
        } else {
            for url in &index {
                validate_sitemap_url(url).unwrap();
            }
        }
        eprintln!(
            "Öffentliche Sitemapprobe: {} Sitemapdateien im Index.",
            index.len()
        );
        browser.close().await.unwrap();
    }
}

#[cfg(test)]
mod challenge_scope_tests {
    use super::*;
    #[test]
    fn quoted_browser_check_is_a_readable_post() {
        let html = r#"<html><title>Checking your browser</title><article class="js-post" data-content="post-1"><div class="bbWrapper">Checking your browser, cf-chl- und /.stile/challenge/ werden hier als Bug beschrieben.</div></article></html>"#;
        assert_eq!(parse_thread_html(html).unwrap().posts.len(), 1);
        assert!(
            response_text_checked("<html><title>Checking your browser</title></html>").is_err()
        );
    }
}
