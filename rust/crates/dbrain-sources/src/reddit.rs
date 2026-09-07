use serde_json::Value;

use crate::{Result, SourcesError};

pub const SOURCE: &str = "reddit";
pub const DEFAULT_SUBREDDIT: &str = "Deadlock";
pub const REDDIT_BASE_URL: &str = "https://www.reddit.com";

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
            subreddit: DEFAULT_SUBREDDIT.to_string(),
            thread_id,
            title,
            author: entry_author(&entry),
            author_flair: None,
            permalink: permalink_url(&link),
            url: None,
            selftext: html_to_text(&child_text(&entry, "content").unwrap_or_default()),
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
    let Some(first) = entries.first() else {
        return Ok(None);
    };
    let Some(link) = link_href(first) else {
        return Ok(None);
    };
    let Some(thread_id) = thread_id_from_permalink(&link) else {
        return Ok(None);
    };
    let mut comments = Vec::new();
    for entry in entries.iter().skip(1) {
        let Some(entry_link) = link_href(entry) else {
            continue;
        };
        let Some(comment_id) = last_path_segment(&entry_link) else {
            continue;
        };
        comments.push(RssComment {
            comment_id,
            author: entry_author(entry),
            content: html_to_text(&child_text(entry, "content").unwrap_or_default()),
            updated: child_text(entry, "updated").map(ToString::to_string),
            permalink: Some(permalink_url(&entry_link)),
        });
    }
    Ok(Some(RssThread {
        subreddit: subreddit_from_permalink(&link),
        thread_id,
        title: child_text(first, "title")
            .map(ToString::to_string)
            .unwrap_or_default(),
        author: entry_author(first),
        content: html_to_text(&child_text(first, "content").unwrap_or_default()),
        permalink: permalink_url(&link),
        updated: child_text(first, "updated").map(ToString::to_string),
        comments,
    }))
}

pub(crate) fn thread_id_from_permalink(url: &str) -> Option<String> {
    let path = url
        .split_once("://")
        .and_then(|(_, rest)| rest.split_once('/').map(|(_, path)| path))
        .unwrap_or(url)
        .split(['?', '#'])
        .next()?
        .trim_end_matches('/');
    let segments = path.split('/').collect::<Vec<_>>();
    let comments_index = segments.iter().position(|segment| *segment == "comments")?;
    let thread_id = segments.get(comments_index + 1)?;
    if thread_id.is_empty() || !thread_id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        return None;
    }
    Some((*thread_id).to_string())
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

fn last_path_segment(url: &str) -> Option<String> {
    let path = url
        .split_once("://")
        .and_then(|(_, rest)| rest.split_once('/').map(|(_, path)| path))
        .unwrap_or(url)
        .split(['?', '#'])
        .next()?
        .trim_end_matches('/');
    path.rsplit('/').next().map(ToString::to_string)
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
        assert_eq!(thread.title, "Vyper feels weak after patch");
        assert_eq!(thread.author.as_deref(), Some("lane_goblin"));
        assert_eq!(
            thread.permalink,
            "https://www.reddit.com/r/Deadlock/comments/1f2abc9/vyper_feels_weak_after_patch/"
        );
        assert_eq!(thread.selftext, "Winrate dropped hard.");
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
}
