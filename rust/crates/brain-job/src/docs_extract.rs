use crate::extract::{Assertion, Atom};
use anyhow::{ensure, Context, Result};
use html5ever::{
    tendril::StrTendril,
    tokenizer::{
        states::RawKind, BufferQueue, Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer,
    },
};
use scraper::{ElementRef, Html, Node, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_BYTES: usize = 8 * 1024 * 1024;
pub const INTRO_SELECTOR: &str = "main > h1, main > p";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SectionMetadata {
    pub ordinal: i32,
    pub title: String,
    pub lang: String,
    pub heading_path: Vec<String>,
    pub line_start: u64,
    pub line_end: u64,
}

pub fn supported(path: &str) -> bool {
    (path.starts_with("public/") || path.starts_with("internal/"))
        && path.ends_with(".html")
        && path.len() <= 512
        && path.split('/').all(|part| {
            !matches!(part, "" | "." | "..")
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        })
}

pub fn metadata(assertion: &Assertion) -> Result<SectionMetadata> {
    serde_json::from_value(assertion.condition.clone()).context("docs_section_metadata")
}

pub fn section_url(path: &str, revision: &str, section: &SectionMetadata) -> Result<String> {
    ensure!(supported(path), "docs_path");
    ensure!(
        revision.len() == 40
            && revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            && section.line_start > 0
            && section.line_end >= section.line_start,
        "docs_url_bounds"
    );
    let url = if path.starts_with("public/") {
        format!(
            "https://github.com/EarlySalty/Deadlock-Docs/blob/{revision}/{path}#L{}-L{}",
            section.line_start, section.line_end
        )
    } else {
        format!(
            "https://internal.invalid/docs/{revision}/{}#L{}-L{}",
            hex::encode(Sha256::digest(path.as_bytes())),
            section.line_start,
            section.line_end
        )
    };
    ensure!(url.len() <= 256, "docs_url_length");
    Ok(url)
}

#[derive(Default)]
struct Projection {
    selector: String,
    id: Option<String>,
    parent: Option<usize>,
    depth: usize,
    members: usize,
    body: String,
    heading: String,
    start: u64,
    end: u64,
    headings: usize,
}

struct Frame {
    name: String,
    excluded: bool,
    projection: Option<usize>,
    heading: bool,
    root: bool,
    tree_member: bool,
}

#[derive(Default)]
struct TreeFingerprint {
    hash: Sha256,
    text: String,
}

impl TreeFingerprint {
    fn field(&mut self, value: &str) {
        self.hash.update((value.len() as u64).to_be_bytes());
        self.hash.update(value.as_bytes());
    }

    fn flush(&mut self) {
        let text = normalize(&self.text);
        self.text.clear();
        if !text.is_empty() {
            self.hash.update([2]);
            self.field(&text);
        }
    }

    fn start<'a>(&mut self, name: &str, attrs: impl Iterator<Item = (&'a str, &'a str)>) {
        self.flush();
        self.hash.update([0]);
        self.field(name);
        let mut attrs: Vec<_> = attrs.collect();
        attrs.sort_unstable();
        self.hash.update((attrs.len() as u64).to_be_bytes());
        for (name, value) in attrs {
            self.field(name);
            self.field(value);
        }
    }

    fn end(&mut self, name: &str) {
        self.flush();
        self.hash.update([1]);
        self.field(name);
    }

    fn finish(mut self) -> Vec<u8> {
        self.flush();
        self.hash.finalize().to_vec()
    }
}

#[derive(Default)]
struct RawSource<'a> {
    text: &'a str,
    feed_end: usize,
    feed_line: u64,
    markup_end: usize,
    markup_line: u64,
}

impl RawSource<'_> {
    fn advance(&mut self) {
        self.markup_end = self.feed_end;
        self.markup_line = self.feed_line;
    }

    fn span(&mut self, tag: &Tag) -> Result<(u64, u64)> {
        let raw = self
            .text
            .get(self.markup_end..self.feed_end)
            .context("docs_raw_bounds")?;
        ensure!(raw.ends_with('>'), "docs_raw_tag_end");
        let offset = if tag.kind == TagKind::EndTag {
            raw.rfind('<')
        } else {
            raw.find('<')
        }
        .context("docs_raw_tag_start")?;
        let prefix = if tag.kind == TagKind::EndTag { 2 } else { 1 };
        let name_end = offset + prefix + tag.name.len();
        let name = raw
            .get(offset + prefix..name_end)
            .context("docs_raw_tag_name")?;
        ensure!(
            name.eq_ignore_ascii_case(tag.name.as_ref())
                && ((raw.as_bytes().get(offset + 1) == Some(&b'/'))
                    == (tag.kind == TagKind::EndTag))
                && raw.as_bytes().get(name_end).is_some_and(|byte| {
                    byte.is_ascii_whitespace() || matches!(byte, b'/' | b'>')
                }),
            "docs_raw_tag_name"
        );
        let start = self.markup_line
            + raw[..offset]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count() as u64;
        let end = self.feed_line;
        self.advance();
        Ok((start, end))
    }
}

#[derive(Default)]
struct HtmlSink<'a> {
    raw: RawSource<'a>,
    tree: TreeFingerprint,
    frames: Vec<Frame>,
    ids: BTreeSet<String>,
    sections: Vec<Projection>,
    lang: String,
    main_count: usize,
    h1_count: usize,
    html_count: usize,
    head_count: usize,
    body_count: usize,
    token_count: usize,
    bytes: usize,
    error: Option<String>,
}

fn excluded(name: &str) -> bool {
    matches!(
        name,
        "head"
            | "nav"
            | "style"
            | "script"
            | "iframe"
            | "object"
            | "embed"
            | "svg"
            | "canvas"
            | "template"
            | "noscript"
            | "form"
            | "button"
            | "input"
            | "select"
            | "textarea"
            | "audio"
            | "video"
            | "noembed"
            | "noframes"
    )
}

fn void(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn block(name: &str) -> bool {
    matches!(
        name,
        "h1" | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "p"
            | "div"
            | "section"
            | "article"
            | "li"
            | "ul"
            | "ol"
            | "table"
            | "tr"
            | "td"
            | "th"
            | "dl"
            | "dt"
            | "dd"
            | "pre"
            | "blockquote"
            | "br"
            | "hr"
    )
}

fn valid_child(parent: &str, child: &str) -> bool {
    if matches!(child, "head" | "body") && parent != "html" {
        return false;
    }
    match parent {
        "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => !block(child) || child == "br",
        "table" => matches!(
            child,
            "caption" | "colgroup" | "thead" | "tbody" | "tfoot" | "tr" | "script" | "template"
        ),
        "thead" | "tbody" | "tfoot" => matches!(child, "tr" | "script" | "template"),
        "tr" => matches!(child, "td" | "th" | "script" | "template"),
        _ => true,
    }
}

impl HtmlSink<'_> {
    fn append(&mut self, mut index: usize, text: &str, heading: bool) -> Result<()> {
        if heading {
            self.sections[index].heading.push_str(text);
        }
        loop {
            self.bytes = self
                .bytes
                .checked_add(text.len())
                .context("docs_body_budget")?;
            ensure!(self.bytes <= MAX_BYTES, "docs_body_budget");
            self.sections[index].body.push_str(text);
            let Some(parent) = self.sections[index].parent else {
                return Ok(());
            };
            index = parent;
        }
    }

    fn add_section(&mut self, id: &str, parent: Option<usize>, start: u64) -> Result<usize> {
        ensure!(
            self.h1_count == 1 && self.sections.len() < 4096,
            "docs_section_count"
        );
        ensure!(
            id.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | '_')),
            "docs_section_id"
        );
        let depth = parent.map_or(1, |index| self.sections[index].depth + 1);
        ensure!(depth < 16, "docs_heading_depth");
        let index = self.sections.len();
        self.sections.push(Projection {
            selector: if parent.is_some() {
                format!("main section[id=\"{id}\"]")
            } else {
                format!("main > section[id=\"{id}\"]")
            },
            id: Some(id.into()),
            parent,
            depth,
            members: 1,
            start,
            ..Projection::default()
        });
        Ok(index)
    }

    fn token(&mut self, token: Token) -> Result<TokenSinkResult<()>> {
        self.token_count += 1;
        ensure!(self.token_count <= 200_000, "docs_token_limit");
        match token {
            Token::TagToken(tag) => {
                let (start, line) = self.raw.span(&tag)?;
                let name = tag.name.to_string();
                if tag.kind == TagKind::EndTag {
                    let frame = self.frames.pop().context("docs_unmatched_end")?;
                    ensure!(frame.name == name, "docs_unbalanced_html");
                    if frame.tree_member {
                        self.tree.end(&name);
                    }
                    if let Some(index) = frame.projection {
                        if !frame.excluded && block(&name) {
                            self.append(index, " ", frame.heading)?;
                        }
                        if frame.root {
                            self.sections[index].end = line;
                        }
                    }
                    return Ok(TokenSinkResult::Continue);
                }
                ensure!(self.frames.len() < 128, "docs_depth");
                let parent = self.frames.last();
                ensure!(parent.is_some() || name == "html", "docs_html_root");
                if let Some(parent) = parent {
                    ensure!(valid_child(&parent.name, &name), "docs_html_structure");
                    ensure!(
                        name != "a" || !self.frames.iter().any(|frame| frame.name == "a"),
                        "docs_nested_link"
                    );
                }
                let direct_main = parent.is_some_and(|parent| parent.name == "main");
                let tree_member = name == "body"
                    || parent.is_some_and(|parent| !parent.excluded)
                        && self.frames.iter().any(|frame| frame.name == "body");
                if tree_member {
                    self.tree.start(
                        &name,
                        tag.attrs
                            .iter()
                            .map(|a| (a.name.local.as_ref(), a.value.as_ref())),
                    );
                    if void(&name) {
                        self.tree.end(&name);
                    }
                }
                let hidden = tag.attrs.iter().any(|a| {
                    a.name.local.as_ref() == "hidden"
                        || a.name.local.as_ref() == "aria-hidden" && a.value.as_ref() == "true"
                });
                let skip =
                    parent.is_some_and(|parent| parent.excluded) || excluded(&name) || hidden;
                let mut projection = parent.and_then(|parent| parent.projection);
                let mut heading = parent.is_some_and(|parent| parent.heading);
                let mut root = false;
                let id = tag.attrs.iter().find(|a| a.name.local.as_ref() == "id");
                if let Some(id) = id {
                    ensure!(
                        !id.value.is_empty()
                            && id.value.len() <= 256
                            && self.ids.insert(id.value.to_string()),
                        "docs_duplicate_id"
                    );
                }
                if name == "html" {
                    self.html_count += 1;
                    ensure!(
                        self.frames.is_empty() && self.html_count == 1,
                        "docs_html_root"
                    );
                    self.lang = tag
                        .attrs
                        .iter()
                        .find(|a| a.name.local.as_ref() == "lang")
                        .context("docs_lang")?
                        .value
                        .to_string();
                }
                if name == "head" {
                    self.head_count += 1;
                    ensure!(self.head_count == 1 && self.body_count == 0, "docs_head");
                }
                if name == "body" {
                    self.body_count += 1;
                    ensure!(self.body_count == 1 && !skip, "docs_body");
                }
                if name == "main" {
                    self.main_count += 1;
                    ensure!(
                        self.main_count == 1 && !skip && parent.is_some_and(|p| p.name == "body"),
                        "docs_main"
                    );
                    self.sections.push(Projection {
                        selector: INTRO_SELECTOR.into(),
                        ..Projection::default()
                    });
                } else if direct_main && !skip {
                    match name.as_str() {
                        "h1" | "p" => {
                            ensure!(self.sections.len() == 1, "docs_intro_order");
                            projection = Some(0);
                            root = true;
                            self.sections[0].members += 1;
                            if self.sections[0].start == 0 {
                                self.sections[0].start = start;
                            }
                            if name == "h1" {
                                self.h1_count += 1;
                                ensure!(self.h1_count == 1, "docs_h1");
                                heading = true;
                            }
                        }
                        "section" => {
                            let id = id.context("docs_section_id")?.value.as_ref();
                            projection = Some(self.add_section(id, None, start)?);
                            heading = false;
                            root = true;
                        }
                        _ => anyhow::bail!("docs_main_child"),
                    }
                } else if !skip && name == "section" && projection.is_some() {
                    let parent = projection.context("docs_section_parent")?;
                    ensure!(parent > 0, "docs_section_parent");
                    let id = id.context("docs_section_id")?.value.as_ref();
                    projection = Some(self.add_section(id, Some(parent), start)?);
                    heading = false;
                    root = true;
                } else if !skip && name == "h1" {
                    anyhow::bail!("docs_h1_position");
                } else if !skip
                    && matches!(name.as_str(), "h2" | "h3" | "h4" | "h5" | "h6")
                    && parent.is_some_and(|p| p.root && p.name == "section")
                {
                    let section = projection.context("docs_heading_section")?;
                    self.sections[section].headings += 1;
                    heading = self.sections[section].headings == 1;
                }
                if let Some(index) = projection {
                    if !skip && block(&name) {
                        self.append(index, " ", heading)?;
                    }
                }
                if !void(&name) {
                    ensure!(!tag.self_closing, "docs_self_closing");
                    self.frames.push(Frame {
                        name: name.clone(),
                        excluded: skip,
                        projection,
                        heading,
                        root,
                        tree_member,
                    });
                }
                Ok(match name.as_str() {
                    "script" => TokenSinkResult::RawData(RawKind::ScriptData),
                    "style" | "iframe" | "noembed" | "noframes" => {
                        TokenSinkResult::RawData(RawKind::Rawtext)
                    }
                    "title" | "textarea" => TokenSinkResult::RawData(RawKind::Rcdata),
                    _ => TokenSinkResult::Continue,
                })
            }
            Token::CharacterTokens(text) => {
                let Some(frame) = self.frames.last() else {
                    ensure!(text.chars().all(char::is_whitespace), "docs_outside_html");
                    return Ok(TokenSinkResult::Continue);
                };
                if !frame.excluded {
                    if self.frames.iter().any(|frame| frame.name == "body") {
                        self.tree.text.push_str(&text);
                    }
                    let projection = frame.projection;
                    let heading = frame.heading;
                    let main_text = frame.name == "main";
                    if let Some(index) = projection {
                        self.append(index, &text, heading)?;
                    } else if main_text {
                        ensure!(text.chars().all(char::is_whitespace), "docs_main_text");
                    }
                }
                Ok(TokenSinkResult::Continue)
            }
            Token::ParseError(_) | Token::NullCharacterToken => anyhow::bail!("docs_html_parse"),
            Token::CommentToken(_) | Token::DoctypeToken(_) => {
                self.raw.advance();
                Ok(TokenSinkResult::Continue)
            }
            Token::EOFToken => {
                ensure!(self.frames.is_empty(), "docs_truncated_html");
                Ok(TokenSinkResult::Continue)
            }
        }
    }
}

impl TokenSink for HtmlSink<'_> {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        if self.error.is_some() {
            return TokenSinkResult::Continue;
        }
        match self.token(token) {
            Ok(result) => result,
            Err(error) => {
                self.error = Some(error.to_string());
                TokenSinkResult::Continue
            }
        }
    }
}

pub fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn valid_lang(lang: &str) -> bool {
    let mut parts = lang.split('-');
    let Some(first) = parts.next() else {
        return false;
    };
    (2..=8).contains(&first.len())
        && first.bytes().all(|b| b.is_ascii_alphabetic())
        && lang.len() <= 16
        && parts.next().is_none_or(|part| {
            !part.is_empty() && part.len() <= 8 && part.bytes().all(|b| b.is_ascii_alphanumeric())
        })
        && parts.next().is_none()
}

fn hidden_element(element: ElementRef<'_>) -> bool {
    element.value().attr("hidden").is_some()
        || element.value().attr("aria-hidden") == Some("true")
}

fn walk_html5(
    element: ElementRef<'_>,
    tree: &mut TreeFingerprint,
    body: &mut String,
    depth: usize,
) -> Result<()> {
    ensure!(depth < 128, "docs_depth");
    let name = element.value().name();
    tree.start(name, element.value().attrs());
    if !excluded(name) && !hidden_element(element) {
        if block(name) {
            body.push(' ');
        }
        for child in element.children() {
            match child.value() {
                Node::Text(text) => {
                    tree.text.push_str(text);
                    body.push_str(text);
                }
                Node::Element(_) => {
                    let element = ElementRef::wrap(child).context("docs_html5_element")?;
                    walk_html5(element, tree, body, depth + 1)?;
                }
                _ => {}
            }
            ensure!(body.len() <= MAX_BYTES, "docs_body_budget");
        }
        if block(name) && !void(name) {
            body.push(' ');
        }
    }
    tree.end(name);
    Ok(())
}

fn css_selector(selector: &str) -> Result<Selector> {
    Selector::parse(selector).map_err(|_| anyhow::anyhow!("docs_selector"))
}

fn verify_html5(text: &str, sink: &mut HtmlSink<'_>) -> Result<()> {
    let document = Html::parse_document(text);
    let body_selector = css_selector("body")?;
    let mut bodies = document.select(&body_selector);
    let body = bodies.next().context("docs_html5_body")?;
    ensure!(bodies.next().is_none(), "docs_html5_body");
    let mut tree = TreeFingerprint::default();
    walk_html5(body, &mut tree, &mut String::new(), 0)?;
    ensure!(
        tree.finish() == std::mem::take(&mut sink.tree).finish(),
        "docs_html5_repair"
    );
    let mut ids = BTreeMap::new();
    for node in document.tree.nodes() {
        if let Some(element) = ElementRef::wrap(node) {
            if let Some(id) = element.value().id() {
                ensure!(ids.insert(id, element).is_none(), "docs_duplicate_id");
            }
        }
    }
    for section in &sink.sections {
        let selector = css_selector(&section.selector)?;
        let roots = if let Some(id) = &section.id {
            let root = *ids.get(id.as_str()).context("docs_html5_selector")?;
            ensure!(selector.matches(&root), "docs_html5_selector");
            vec![root]
        } else {
            document
                .select(&selector)
                .filter(|root| !hidden_element(*root))
                .collect()
        };
        ensure!(roots.len() == section.members, "docs_html5_members");
        let mut body = String::new();
        for root in roots {
            walk_html5(root, &mut TreeFingerprint::default(), &mut body, 0)?;
        }
        ensure!(
            normalize(&body) == normalize(&section.body),
            "docs_html5_projection"
        );
    }
    Ok(())
}

pub fn extract(path: &str, bytes: &[u8]) -> Result<Vec<Assertion>> {
    ensure!(supported(path) && bytes.len() <= MAX_BYTES, "docs_input");
    let text = std::str::from_utf8(bytes).context("docs_utf8")?;
    ensure!(
        bytes.iter().enumerate().all(|(index, byte)| {
            *byte != b'\r' || bytes.get(index + 1) == Some(&b'\n')
        }),
        "docs_line_endings"
    );
    let sink = HtmlSink {
        raw: RawSource {
            text,
            feed_line: 1,
            markup_line: 1,
            ..RawSource::default()
        },
        ..HtmlSink::default()
    };
    let mut tokenizer = Tokenizer::new(sink, Default::default());
    let mut input = BufferQueue::default();
    for chunk in text.split_inclusive('>') {
        tokenizer.sink.raw.feed_end += chunk.len();
        tokenizer.sink.raw.feed_line += chunk.bytes().filter(|byte| *byte == b'\n').count() as u64;
        input.push_back(StrTendril::from(chunk));
        let _ = tokenizer.feed(&mut input);
        if let Some(error) = &tokenizer.sink.error {
            anyhow::bail!(error.clone());
        }
    }
    tokenizer.end();
    let mut sink = tokenizer.sink;
    if let Some(error) = &sink.error {
        anyhow::bail!(error.clone());
    }
    ensure!(
        sink.main_count == 1 && sink.h1_count == 1 && sink.html_count == 1 && sink.body_count == 1,
        "docs_html_invalid"
    );
    ensure!(valid_lang(&sink.lang), "docs_lang");
    verify_html5(text, &mut sink)?;
    let title = normalize(&sink.sections[0].heading);
    ensure!(!title.is_empty() && title.len() <= 256, "docs_title");
    let mut heading_paths: Vec<Vec<String>> = vec![vec![title.clone()]];
    for section in sink.sections.iter().skip(1) {
        let heading = normalize(&section.heading);
        ensure!(
            section.headings > 0 && !heading.is_empty() && heading.len() <= 256,
            "docs_heading"
        );
        let mut path = heading_paths[section.parent.unwrap_or(0)].clone();
        path.push(heading);
        ensure!(path.len() <= 16, "docs_heading_depth");
        heading_paths.push(path);
    }
    let total_lines = text.bytes().filter(|b| *b == b'\n').count() as u64 + 1;
    sink.sections
        .into_iter()
        .enumerate()
        .map(|(ordinal, section)| {
            let body = normalize(&section.body);
            ensure!(
                !body.is_empty() && body.len() <= 262144,
                "docs_section_body"
            );
            ensure!(
                section.start > 0 && section.end >= section.start && section.end <= total_lines,
                "docs_section_lines"
            );
            let heading_path = heading_paths[ordinal].clone();
            let ordinal = i32::try_from(ordinal).context("docs_ordinal")?;
            let metadata = SectionMetadata {
                ordinal,
                title: title.clone(),
                lang: sink.lang.clone(),
                heading_path,
                line_start: section.start,
                line_end: section.end,
            };
            Ok(Assertion {
                entity: String::new(),
                kind: "docs_section",
                locator_kind: "css_selector",
                locator: section.selector,
                hash: Sha256::digest(body.as_bytes()).to_vec(),
                value: Atom::Text(body),
                condition: serde_json::to_value(metadata)?,
                alias: None,
                relation: None,
                stat: "docs.section".into(),
                section: "docs",
                unit: None,
                order: ordinal,
            })
        })
        .collect()
}

pub fn prove(path: &str, bytes: &[u8], assertions: &[Assertion]) -> Result<()> {
    let expected = extract(path, bytes)?;
    let mut seen = BTreeSet::new();
    for actual in assertions {
        ensure!(seen.insert(actual.locator.as_str()), "docs_proof_duplicate");
        let expected = expected
            .iter()
            .find(|expected| expected.locator == actual.locator)
            .context("docs_proof_locator")?;
        ensure!(
            actual.locator_kind == expected.locator_kind
                && actual.locator == expected.locator
                && actual.hash == expected.hash
                && actual.condition == expected.condition
                && actual.entity == expected.entity
                && actual.kind == expected.kind
                && actual.stat == expected.stat
                && actual.section == expected.section
                && actual.order == expected.order
                && actual.alias.is_none()
                && actual.relation.is_none()
                && actual.unit.is_none(),
            "docs_proof_metadata"
        );
        ensure!(
            matches!((&actual.value, &expected.value), (Atom::Text(a), Atom::Text(b)) if a == b),
            "docs_proof_body"
        );
    }
    Ok(())
}

pub fn linked_entities(body: &str, aliases: &[(String, i64)]) -> Result<Vec<i64>> {
    let text = normalize(body).to_lowercase();
    let mut ids = BTreeSet::new();
    for (alias, id) in aliases {
        ensure!(
            *id > 0 && !alias.is_empty() && alias.len() <= 256,
            "docs_alias"
        );
        if text.match_indices(alias).any(|(offset, matched)| {
            text[..offset]
                .chars()
                .next_back()
                .is_none_or(|c| !c.is_alphanumeric() && c != '_')
                && text[offset + matched.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| !c.is_alphanumeric() && c != '_')
        }) {
            ids.insert(*id);
            ensure!(ids.len() <= 64, "docs_entity_limit");
        }
    }
    Ok(ids.into_iter().collect())
}
