use crate::{
    limits, Entity, Extractor, Fact, FactSet, FactValue, Outcome, RawObject, Receipt, Rejection,
};
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
struct SectionProjection {
    pub source_key: Option<String>,
    pub locator: String,
    pub heading_path: Vec<String>,
    pub lines: (u64, u64),
    pub body: String,
    pub byte_ranges: Vec<(usize, usize)>,
}
struct DocumentProjection {
    pub title: String,
    pub lang: String,
    pub sections: Vec<SectionProjection>,
}
#[derive(Clone, Copy)]
struct Span {
    lines: (u64, u64),
    bytes: (usize, usize),
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
    fn span(&mut self, tag: &Tag) -> Result<Span> {
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
        let span = Span {
            lines: (
                self.markup_line + raw[..offset].bytes().filter(|b| *b == b'\n').count() as u64,
                self.feed_line,
            ),
            bytes: (self.markup_end + offset, self.feed_end),
        };
        self.advance();
        Ok(span)
    }
}
struct Frame {
    name: String,
    excluded: bool,
    member: Option<usize>,
}
#[derive(Default)]
struct HtmlSink<'a> {
    raw: RawSource<'a>,
    tree: Fingerprint,
    frames: Vec<Frame>,
    spans: Vec<Span>,
    ids: BTreeSet<String>,
    counts: [usize; 4],
    tokens: usize,
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
                self.spans[index].lines.1 = span.lines.1;
                self.spans[index].bytes.1 = span.bytes.1;
                self.tree.end(&name);
            }
            return Ok(TokenSinkResult::Continue);
        }
        ensure!(self.frames.len() < 64, "docs_depth_budget");
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
        ensure!(self.tokens <= limits::ROWS * 64, "docs_token_budget");
        match token {
            Token::TagToken(tag) => return self.tag(tag),
            Token::CharacterTokens(text) => match self.frames.last() {
                None => ensure!(text.chars().all(char::is_whitespace), "docs_outside_html"),
                Some(frame) if frame.member.is_some() && !frame.excluded => {
                    self.tree.text.push_str(&text);
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
        self.token(token).unwrap_or_else(|error| {
            self.error = Some(error.to_string());
            TokenSinkResult::Continue
        })
    }
}
fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
struct DomRange<'a> {
    element: ElementRef<'a>,
    start: usize,
    end: usize,
    span: Span,
}
#[derive(Default)]
struct DomWalk<'a, 's> {
    spans: std::slice::Iter<'s, Span>,
    tree: Fingerprint,
    text: String,
    ranges: Vec<DomRange<'a>>,
}
impl<'a> DomWalk<'a, '_> {
    fn walk(&mut self, element: ElementRef<'a>, depth: usize) -> Result<()> {
        ensure!(depth < 64, "docs_depth_budget");
        let name = element.value().name();
        self.tree.start(name, element.value().attrs());
        let index = self.ranges.len();
        self.ranges.push(DomRange {
            element,
            start: self.text.len(),
            end: self.text.len(),
            span: *self.spans.next().context("docs_html5_members")?,
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
            }
            if block(name) && !void(name) {
                self.text.push(' ');
            }
        }
        ensure!(
            self.text.len() <= limits::RAW_BYTES,
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
fn parse(bytes: &[u8]) -> Result<DocumentProjection> {
    ensure!(bytes.len() <= limits::RAW_BYTES, "docs_raw_budget");
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
        ..HtmlSink::default()
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
        ..DomWalk::default()
    };
    walk.walk(body, 0)?;
    ensure!(
        walk.spans.next().is_none() && walk.tree.finish() == sink.tree.finish(),
        "docs_html5_repair"
    );
    let ranges: HashMap<_, _> = walk.ranges.iter().map(|r| (r.element.id(), r)).collect();
    let range_of = |e: ElementRef<'_>| ranges.get(&e.id()).copied().context("docs_html5_range");
    let html = document.root_element();
    let lang = html.value().attr("lang").context("docs_lang")?.to_owned();
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
    let mut remaining = limits::RAW_BYTES;
    push_section(
        &mut result,
        (None, "main > h1, main > p".into()),
        vec![title],
        (
            (first.span.lines.0, last.span.lines.1),
            roots
                .iter()
                .map(|e| range_of(*e).map(|r| r.span.bytes))
                .collect::<Result<_>>()?,
        ),
        &walk.text[first.start..last.end],
        &mut remaining,
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
        let id = element.value().id().context("docs_section_id")?;
        ensure!(!id.trim().is_empty(), "docs_section_id");
        let selector = format!(
            "main{} section[id={}]",
            if parent.is_some() { "" } else { " >" },
            serde_json::to_string(id)?
        );
        ensure!(css(&selector)?.matches(&element), "docs_html5_selector");
        parents.insert(element.id(), result.sections.len());
        push_section(
            &mut result,
            (Some(id.into()), selector),
            heading_path,
            (range.span.lines, vec![range.span.bytes]),
            &walk.text[range.start..range.end],
            &mut remaining,
        )?;
    }
    Ok(result)
}
fn push_section(
    document: &mut DocumentProjection,
    key: (Option<String>, String),
    heading_path: Vec<String>,
    source: ((u64, u64), Vec<(usize, usize)>),
    text: &str,
    remaining: &mut usize,
) -> Result<()> {
    ensure!(
        document.sections.len() < limits::ROWS,
        "docs_section_budget"
    );
    let (source_key, locator) = key;
    let (lines, byte_ranges) = source;
    let body = normalize(text);
    *remaining = remaining
        .checked_sub(body.len())
        .context("docs_projection_budget")?;
    document.sections.push(SectionProjection {
        source_key,
        locator,
        heading_path,
        lines,
        byte_ranges,
        body,
    });
    Ok(())
}
pub struct DocsExtractor;
impl Extractor for DocsExtractor {
    fn name(&self) -> &'static str {
        "deadlock-docs"
    }
    fn accepts(&self, source: &str, path: &str) -> bool {
        source == "deadlock-docs"
            && (path.starts_with("public/") || path.starts_with("internal/"))
            && path.ends_with(".html")
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        if !self.accepts(&raw.source, &raw.path) {
            return Outcome::Ignored;
        }
        match extract(raw) {
            Ok(set) => Outcome::Accepted(set),
            Err(error) => Outcome::Rejected(error),
        }
    }
}
fn bounded(value: &str, max: usize) -> std::result::Result<(), Rejection> {
    if value.trim().is_empty() || value.len() > max || value.chars().any(char::is_control) {
        return Err(Rejection::new(
            "value_too_large",
            "Dokumentwert überschreitet den Ausgabevertrag",
        ));
    }
    Ok(())
}
fn extract(raw: &RawObject) -> std::result::Result<FactSet, Rejection> {
    bounded(&raw.path, 2048)?;
    if raw.revision.len() != 40
        || !raw
            .revision
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        || !raw
            .path
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/-_.".contains(&b))
        || raw
            .path
            .split('/')
            .any(|p| p.is_empty() || matches!(p, "." | ".."))
    {
        return Err(Rejection::new(
            "invalid_docs_path",
            "Dokumentpfad oder Revision ist ungültig",
        ));
    }
    let document = parse(&raw.bytes).map_err(|_| {
        Rejection::new(
            "invalid_docs",
            "Dokumentstruktur oder Extraktionsgrenze ist ungültig",
        )
    })?;
    bounded(&document.lang, limits::TEXT_BYTES)?;
    let receipt = Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(&raw.bytes).into(),
    };
    let identity = format!("{}\0{}", raw.source, raw.path);
    let key = format!("doc_{:x}", Sha256::digest(identity.as_bytes()));
    let mut set = FactSet::default();
    set.entities.push(Entity {
        kind: "doc_page".into(),
        key: key.clone(),
        receipt: receipt.clone(),
    });
    let mut add = |stat_key: String, value: FactValue, receipt: &Receipt| {
        set.facts.push(Fact {
            entity_key: key.clone(),
            stat_key,
            value,
            receipt: receipt.clone(),
        });
    };
    for (name, value) in [
        ("doc_path", FactValue::Text(raw.path.clone())),
        ("doc_title", FactValue::Text(document.title)),
        ("doc_lang", FactValue::Text(document.lang)),
        (
            "doc_section_count",
            FactValue::Number(document.sections.len() as f64),
        ),
    ] {
        add(name.into(), value, &receipt);
    }
    for (ordinal, section) in document.sections.into_iter().enumerate() {
        if section.heading_path.len() > 16 || section.byte_ranges.len() > limits::BYTE_RANGES {
            return Err(Rejection::new(
                "value_too_large",
                "Dokumentabschnitt überschreitet die Beleggrenze",
            ));
        }
        for heading in &section.heading_path {
            bounded(heading, 256)?;
        }
        bounded(&section.locator, limits::LOCATOR_BYTES)?;
        if raw.path.starts_with("public/") {
            let url = format!(
                "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/{}#L{}-L{}",
                raw.revision, raw.path, section.lines.0, section.lines.1
            );
            bounded(&url, 256)?;
        }
        let mut hash = Sha256::new();
        let mut size = 0;
        let mut ranges = Vec::new();
        for (start, end) in section.byte_ranges {
            size += end - start;
            if size > limits::BYTE_RANGE_BYTES {
                return Err(Rejection::new(
                    "value_too_large",
                    "Abschnittsbeleg überschreitet die Byte-Grenze",
                ));
            }
            hash.update(&raw.bytes[start..end]);
            ranges.push(format!("{start}-{end}"));
        }
        let receipt = Receipt {
            locator_kind: "byte_ranges".into(),
            locator: ranges.join(","),
            value_sha256: hash.finalize().into(),
        };
        let stem = match section.source_key {
            None => "doc_intro".into(),
            Some(id) => format!("doc_section_{}", limits::normalize_key(&id)),
        };
        let mut body = section.body.as_str();
        let mut chunks = 0;
        while !body.is_empty() {
            let mut end = body.len().min(limits::TEXT_BYTES);
            while !body.is_char_boundary(end) {
                end -= 1;
            }
            add(
                format!("{stem}_body_{chunks}"),
                FactValue::Text(body[..end].into()),
                &receipt,
            );
            body = &body[end..];
            chunks += 1;
        }
        let metadata = serde_json::json!({"ordinal":ordinal,"locator":section.locator,"heading_path":section.heading_path,"line_start":section.lines.0,"line_end":section.lines.1,"chunks":chunks});
        add(
            format!("{stem}_meta"),
            FactValue::Text(metadata.to_string()),
            &receipt,
        );
    }
    limits::validate(&set)?;
    Ok(set)
}
