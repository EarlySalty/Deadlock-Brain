use anyhow::{ensure, Context, Result};
use html5ever::{
    tendril::StrTendril,
    tokenizer::{
        states::RawKind, BufferQueue, Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer,
    },
};
use scraper::{ElementRef, Html, Node, Selector};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
const INTRO_SELECTOR: &str = "main > h1, main > p";
#[derive(Clone, Copy)]
pub(super) struct ParsingBounds {
    pub raw_bytes: usize,
    pub tokens: usize,
    pub depth: usize,
    pub sections: usize,
    pub output_bytes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SectionProjection {
    pub ordinal: usize,
    pub source_key: Option<String>,
    pub locator: String,
    pub heading_path: Vec<String>,
    pub lines: (u64, u64),
    pub body: String,
    pub value_sha256: [u8; 32],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DocumentProjection {
    pub title: String,
    pub lang: String,
    pub sections: Vec<SectionProjection>,
}
pub(super) enum ReplayCoverage {
    Complete,
    Subset,
}
#[derive(Default)]
struct Fingerprint {
    hash: Sha256,
    text: String,
}
impl Fingerprint {
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
    fn finish(mut self) -> [u8; 32] {
        self.flush();
        self.hash.finalize().into()
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
                && (raw.as_bytes().get(offset + 1) == Some(&b'/')) == (tag.kind == TagKind::EndTag)
                && raw
                    .as_bytes()
                    .get(name_end)
                    .is_some_and(|b| b.is_ascii_whitespace() || matches!(b, b'/' | b'>')),
            "docs_raw_tag_name"
        );
        let span = (
            self.markup_line + raw[..offset].bytes().filter(|b| *b == b'\n').count() as u64,
            self.feed_line,
        );
        self.advance();
        Ok(span)
    }
}
struct Frame {
    name: String,
    excluded: bool,
    member: Option<usize>,
}
struct HtmlSink<'a> {
    raw: RawSource<'a>,
    tree: Fingerprint,
    frames: Vec<Frame>,
    spans: Vec<(u64, u64)>,
    ids: BTreeSet<String>,
    counts: [usize; 4],
    tokens: usize,
    bounds: ParsingBounds,
    error: Option<String>,
}
fn excluded(name: &str) -> bool {
    "head nav style script iframe object embed svg canvas template noscript form button input select textarea audio video noembed noframes"
        .split_ascii_whitespace().any(|tag| tag == name)
}
fn void(name: &str) -> bool {
    "area base br col embed hr img input link meta param source track wbr"
        .split_ascii_whitespace()
        .any(|tag| tag == name)
}
fn block(name: &str) -> bool {
    "h1 h2 h3 h4 h5 h6 p div section article li ul ol table tr td th dl dt dd pre blockquote br hr"
        .split_ascii_whitespace()
        .any(|tag| tag == name)
}
fn hidden(element: ElementRef<'_>) -> bool {
    element.value().attr("hidden").is_some() || element.value().attr("aria-hidden") == Some("true")
}
impl HtmlSink<'_> {
    fn tag(&mut self, tag: Tag) -> Result<TokenSinkResult<()>> {
        let span = self.raw.span(&tag)?;
        let name = tag.name.to_string();
        if tag.kind == TagKind::EndTag {
            let frame = self.frames.pop().context("docs_unmatched_end")?;
            ensure!(frame.name == name, "docs_unbalanced_html");
            if let Some(index) = frame.member {
                self.spans[index].1 = span.1;
                self.tree.end(&name);
            }
            return Ok(TokenSinkResult::Continue);
        }
        ensure!(self.frames.len() < self.bounds.depth, "docs_depth_budget");
        let parent = self.frames.last();
        ensure!(parent.is_some() || name == "html", "docs_html_root");
        let skip = parent.is_some_and(|p| p.excluded)
            || excluded(&name)
            || tag.attrs.iter().any(|a| {
                a.name.local.as_ref() == "hidden"
                    || a.name.local.as_ref() == "aria-hidden" && a.value.as_ref() == "true"
            });
        if let Some(index) = ["html", "head", "body", "main"]
            .iter()
            .position(|n| *n == name)
        {
            self.counts[index] += 1;
            ensure!(self.counts[index] == 1, "docs_html_structure");
            ensure!(
                match name.as_str() {
                    "html" => parent.is_none(),
                    "head" => parent.is_some_and(|p| p.name == "html") && self.counts[2] == 0,
                    "body" => parent.is_some_and(|p| p.name == "html") && !skip,
                    _ => parent.is_some_and(|p| p.name == "body") && !skip,
                },
                "docs_html_structure"
            );
        }
        for id in tag.attrs.iter().filter(|a| a.name.local.as_ref() == "id") {
            ensure!(
                !id.value.is_empty() && self.ids.insert(id.value.to_string()),
                "docs_duplicate_id"
            );
        }
        let member = if name == "body" || parent.is_some_and(|p| p.member.is_some() && !p.excluded)
        {
            self.tree.start(
                &name,
                tag.attrs
                    .iter()
                    .map(|a| (a.name.local.as_ref(), a.value.as_ref())),
            );
            let index = self.spans.len();
            self.spans.push(span);
            if void(&name) {
                self.tree.end(&name);
            }
            Some(index)
        } else {
            None
        };
        if !void(&name) {
            ensure!(!tag.self_closing, "docs_self_closing");
            self.frames.push(Frame {
                name: name.clone(),
                excluded: skip,
                member,
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
    fn token(&mut self, token: Token) -> Result<TokenSinkResult<()>> {
        self.tokens += 1;
        ensure!(self.tokens <= self.bounds.tokens, "docs_token_budget");
        match token {
            Token::TagToken(tag) => return self.tag(tag),
            Token::CharacterTokens(text) => match self.frames.last() {
                None => ensure!(text.chars().all(char::is_whitespace), "docs_outside_html"),
                Some(frame) if frame.member.is_some() && !frame.excluded => {
                    self.tree.text.push_str(&text);
                    ensure!(
                        self.tree.text.len() <= self.bounds.raw_bytes,
                        "docs_text_budget"
                    );
                }
                _ => {}
            },
            Token::ParseError(_) | Token::NullCharacterToken => anyhow::bail!("docs_html_parse"),
            Token::CommentToken(_) | Token::DoctypeToken(_) => self.raw.advance(),
            Token::EOFToken => ensure!(self.frames.is_empty(), "docs_truncated_html"),
        }
        Ok(TokenSinkResult::Continue)
    }
}
impl TokenSink for HtmlSink<'_> {
    type Handle = ();
    fn process_token(&mut self, token: Token, _line: u64) -> TokenSinkResult<()> {
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
fn normalize(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}
struct DomRange<'a> {
    element: ElementRef<'a>,
    start: usize,
    end: usize,
    lines: (u64, u64),
}
struct DomWalk<'a, 's> {
    spans: std::slice::Iter<'s, (u64, u64)>,
    tree: Fingerprint,
    text: String,
    ranges: Vec<DomRange<'a>>,
    bounds: ParsingBounds,
}
impl<'a> DomWalk<'a, '_> {
    fn walk(&mut self, element: ElementRef<'a>, depth: usize) -> Result<()> {
        ensure!(depth < self.bounds.depth, "docs_depth_budget");
        let name = element.value().name();
        self.tree.start(name, element.value().attrs());
        let index = self.ranges.len();
        self.ranges.push(DomRange {
            element,
            start: self.text.len(),
            end: self.text.len(),
            lines: *self.spans.next().context("docs_html5_members")?,
        });
        if !excluded(name) && !hidden(element) {
            if block(name) {
                self.text.push(' ');
            }
            for child in element.children() {
                match child.value() {
                    Node::Text(text) => {
                        self.tree.text.push_str(text);
                        self.text.push_str(text);
                    }
                    Node::Element(_) => self.walk(
                        ElementRef::wrap(child).context("docs_html5_element")?,
                        depth + 1,
                    )?,
                    _ => {}
                }
                ensure!(
                    self.text.len() <= self.bounds.output_bytes,
                    "docs_projection_budget"
                );
            }
            if block(name) && !void(name) {
                self.text.push(' ');
            }
        }
        ensure!(
            self.text.len() <= self.bounds.output_bytes,
            "docs_projection_budget"
        );
        self.ranges[index].end = self.text.len();
        self.tree.end(name);
        Ok(())
    }
}
fn css(value: &str) -> Result<Selector> {
    Selector::parse(value).map_err(|_| anyhow::anyhow!("docs_selector"))
}
fn quoted_id(id: &str) -> String {
    let mut result = String::new();
    for ch in id.chars() {
        match ch {
            '"' | '\\' => {
                result.push('\\');
                result.push(ch);
            }
            ch if ch.is_control() => result.push_str(&format!("\\{:x} ", u32::from(ch))),
            ch => result.push(ch),
        }
    }
    result
}
pub(super) fn parse(bytes: &[u8], bounds: ParsingBounds) -> Result<DocumentProjection> {
    ensure!(bytes.len() <= bounds.raw_bytes, "docs_raw_budget");
    let text = std::str::from_utf8(bytes).context("docs_utf8")?;
    ensure!(
        bytes
            .iter()
            .enumerate()
            .all(|(i, b)| *b != b'\r' || bytes.get(i + 1) == Some(&b'\n')),
        "docs_line_endings"
    );
    let sink = HtmlSink {
        raw: RawSource {
            text,
            feed_line: 1,
            markup_line: 1,
            ..RawSource::default()
        },
        tree: Fingerprint::default(),
        frames: Vec::new(),
        spans: Vec::new(),
        ids: BTreeSet::new(),
        counts: [0; 4],
        tokens: 0,
        bounds,
        error: None,
    };
    let mut tokenizer = Tokenizer::new(sink, Default::default());
    let mut input = BufferQueue::default();
    for chunk in text.split_inclusive('>') {
        tokenizer.sink.raw.feed_end += chunk.len();
        tokenizer.sink.raw.feed_line += chunk.bytes().filter(|b| *b == b'\n').count() as u64;
        input.push_back(StrTendril::from(chunk));
        let _ = tokenizer.feed(&mut input);
        if let Some(error) = &tokenizer.sink.error {
            anyhow::bail!(error.clone());
        }
    }
    tokenizer.end();
    let sink = tokenizer.sink;
    if let Some(error) = sink.error {
        anyhow::bail!(error);
    }
    ensure!(
        sink.counts[0] == 1 && sink.counts[2] == 1 && sink.counts[3] == 1,
        "docs_html_structure"
    );
    let document = Html::parse_document(text);
    let body = document
        .select(&css("body")?)
        .next()
        .context("docs_html5_body")?;
    let mut walk = DomWalk {
        spans: sink.spans.iter(),
        tree: Fingerprint::default(),
        text: String::new(),
        ranges: Vec::new(),
        bounds,
    };
    walk.walk(body, 0)?;
    ensure!(
        walk.spans.next().is_none() && walk.tree.finish() == sink.tree.finish(),
        "docs_html5_repair"
    );
    let ranges: HashMap<_, _> = walk.ranges.iter().map(|r| (r.element.id(), r)).collect();
    let range_of = |e: ElementRef<'_>| ranges.get(&e.id()).copied().context("docs_html5_range");
    let html = document.root_element();
    let lang = html
        .value()
        .attr("lang")
        .filter(|v| !v.is_empty())
        .context("docs_lang")?
        .to_owned();
    let main = document
        .select(&css("body > main")?)
        .next()
        .context("docs_main")?;
    let mut roots = Vec::new();
    let mut section_seen = false;
    let mut title = None;
    for child in main.children() {
        if let Some(element) = ElementRef::wrap(child) {
            if excluded(element.value().name()) || hidden(element) {
                continue;
            }
            match element.value().name() {
                "h1" | "p" => {
                    ensure!(!section_seen, "docs_intro_order");
                    if element.value().name() == "h1" {
                        ensure!(title.is_none(), "docs_h1");
                        let range = range_of(element)?;
                        title = Some(normalize(&walk.text[range.start..range.end]));
                    }
                    roots.push(element);
                }
                "section" => {
                    ensure!(title.is_some(), "docs_h1");
                    section_seen = true;
                }
                _ => anyhow::bail!("docs_main_child"),
            }
        } else if let Node::Text(text) = child.value() {
            ensure!(text.chars().all(char::is_whitespace), "docs_main_text");
        }
    }
    let title = title.filter(|v| !v.is_empty()).context("docs_title")?;
    let first = range_of(*roots.first().context("docs_intro")?)?;
    let last = range_of(*roots.last().context("docs_intro")?)?;
    let mut result = DocumentProjection {
        title: title.clone(),
        lang,
        sections: Vec::new(),
    };
    let mut remaining = bounds.output_bytes;
    push_section(
        &mut result,
        (None, INTRO_SELECTOR.into()),
        vec![title],
        (first.lines.0, last.lines.1),
        &walk.text[first.start..last.end],
        &mut remaining,
        bounds,
    )?;
    let mut parents = HashMap::new();
    let heading_selector = css("h2, h3, h4, h5, h6")?;
    let section_selector = css("main section")?;
    for range in &walk.ranges {
        let element = range.element;
        if hidden(element) {
            continue;
        }
        if element.value().name() == "h1" {
            ensure!(roots.contains(&element), "docs_h1_position");
        }
        if !section_selector.matches(&element) {
            continue;
        }
        let parent = element
            .ancestors()
            .find_map(|n| parents.get(&n.id()).copied());
        let heading = element
            .children()
            .filter_map(ElementRef::wrap)
            .find(|e| heading_selector.matches(e) && !hidden(*e))
            .context("docs_heading")?;
        let heading_range = range_of(heading)?;
        let heading = normalize(&walk.text[heading_range.start..heading_range.end]);
        ensure!(!heading.is_empty(), "docs_heading");
        let mut heading_path = result.sections[parent.unwrap_or(0)].heading_path.clone();
        heading_path.push(heading);
        ensure!(heading_path.len() <= bounds.depth, "docs_depth_budget");
        let id = element.value().id().context("docs_section_id")?;
        let selector = format!(
            "main{} section[id=\"{}\"]",
            if parent.is_some() { "" } else { " >" },
            quoted_id(id)
        );
        ensure!(css(&selector)?.matches(&element), "docs_html5_selector");
        parents.insert(element.id(), result.sections.len());
        push_section(
            &mut result,
            (Some(id.into()), selector),
            heading_path,
            range.lines,
            &walk.text[range.start..range.end],
            &mut remaining,
            bounds,
        )?;
    }
    Ok(result)
}
fn push_section(
    document: &mut DocumentProjection,
    key: (Option<String>, String),
    heading_path: Vec<String>,
    lines: (u64, u64),
    text: &str,
    remaining: &mut usize,
    bounds: ParsingBounds,
) -> Result<()> {
    ensure!(
        document.sections.len() < bounds.sections,
        "docs_section_budget"
    );
    let (source_key, locator) = key;
    let body = normalize(text);
    ensure!(
        !body.is_empty() && lines.0 > 0 && lines.1 >= lines.0,
        "docs_section_projection"
    );
    for length in [
        body.len(),
        locator.len(),
        source_key.as_ref().map_or(0, String::len),
    ]
    .into_iter()
    .chain(heading_path.iter().map(String::len))
    {
        *remaining = remaining
            .checked_sub(length)
            .context("docs_projection_budget")?;
    }
    document.sections.push(SectionProjection {
        ordinal: document.sections.len(),
        source_key,
        locator,
        heading_path,
        lines,
        value_sha256: Sha256::digest(body.as_bytes()).into(),
        body,
    });
    Ok(())
}
pub(super) fn replay(
    bytes: &[u8],
    bounds: ParsingBounds,
    actual: &DocumentProjection,
    coverage: ReplayCoverage,
) -> Result<()> {
    let expected = parse(bytes, bounds)?;
    ensure!(
        actual.title == expected.title && actual.lang == expected.lang,
        "docs_proof_metadata"
    );
    if matches!(coverage, ReplayCoverage::Complete) {
        ensure!(actual == &expected, "docs_proof_complete");
    } else {
        let mut seen = BTreeSet::new();
        for section in &actual.sections {
            ensure!(seen.insert(&section.locator), "docs_proof_duplicate");
            let expected = expected
                .sections
                .get(section.ordinal)
                .context("docs_proof_locator")?;
            ensure!(section == expected, "docs_proof_projection");
        }
    }
    Ok(())
}
