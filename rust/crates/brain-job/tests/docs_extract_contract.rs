use brain_job::{
    docs::SOURCE,
    docs_extract,
    extract::{self, Atom},
};
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};

const HTML: &str = "<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title><style>main { color: red; }</style></head><body>\n<main>\n<h1>Billy</h1>\n<p>Einleitung &amp; Hinweis.</p>\n<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Billy <strong>spielt</strong> nah.</p>\n<nav>Navigation</nav><script>let fake = '<section id=\"fake\">';</script><p hidden>Unsichtbar</p>\n</section>\n</main></body></html>";

#[test]
fn shared_receipts_replay_full_bodies_metadata_and_source_lines() {
    let path = "public/deadlock-helden/billy.html";
    let mut assertions = extract::extract(SOURCE, path, HTML.as_bytes()).unwrap();
    assert_eq!(assertions.len(), 2);
    extract::prove(SOURCE, path, HTML.as_bytes(), &assertions).unwrap();
    assert_eq!(assertions[0].locator, docs_extract::INTRO_SELECTOR);
    assert_eq!(assertions[1].locator, "main > section[id=\"spielweise\"]");
    assert!(
        matches!(&assertions[0].value, Atom::Text(body) if body == "Billy Einleitung & Hinweis.")
    );
    let body = "Spielweise Billy spielt nah.";
    assert!(matches!(&assertions[1].value, Atom::Text(actual) if actual == body));
    assert_eq!(assertions[1].hash, Sha256::digest(body.as_bytes()).to_vec());
    let metadata = docs_extract::metadata(&assertions[1]).unwrap();
    assert_eq!(metadata.heading_path, ["Billy", "Spielweise"]);
    assert_eq!(
        (metadata.ordinal, metadata.line_start, metadata.line_end),
        (1, 6, 10)
    );
    let url = docs_extract::section_url(path, &"a".repeat(40), &metadata).unwrap();
    assert_eq!(
        url,
        format!(
            "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/{path}#L6-L10",
            "a".repeat(40)
        )
    );
    assertions[1].condition["line_start"] = 1.into();
    assert!(extract::prove(SOURCE, path, HTML.as_bytes(), &assertions).is_err());
    assertions = extract::extract(SOURCE, path, HTML.as_bytes()).unwrap();
    assertions[1].value = Atom::Text("Verändert".into());
    assert!(extract::prove(SOURCE, path, HTML.as_bytes(), &assertions).is_err());
    assertions = extract::extract(SOURCE, path, HTML.as_bytes()).unwrap();
    assert!(extract::prove(
        SOURCE,
        path,
        HTML.replace("nah.", "fern.").as_bytes(),
        &assertions
    )
    .is_err());
    extract::prove(SOURCE, path, HTML.as_bytes(), &assertions[1..]).unwrap();
}

#[test]
fn multiline_opening_tags_keep_the_first_source_line() {
    let path = "public/test.html";
    let html = HTML
        .replace("<h1>", "<h1\nclass=\"title\">")
        .replace("<section id=\"spielweise\">", "<section\nid=\"spielweise\">");
    let assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    let intro = docs_extract::metadata(&assertions[0]).unwrap();
    let section = docs_extract::metadata(&assertions[1]).unwrap();
    assert_eq!((intro.line_start, intro.line_end), (4, 6));
    assert_eq!((section.line_start, section.line_end), (7, 12));
    extract::prove(SOURCE, path, html.as_bytes(), &assertions).unwrap();
}

#[test]
fn active_fallbacks_and_form_controls_are_not_receipt_text() {
    let html = HTML.replace(
        "</section>",
        "<textarea>Geheim</textarea><audio>Ton</audio><video>Film</video><noembed>Ersatz</noembed><noframes>Rahmen</noframes></section>",
    );
    let assertions = extract::extract(SOURCE, "public/test.html", html.as_bytes()).unwrap();
    assert!(matches!(&assertions[1].value, Atom::Text(body) if body == "Spielweise Billy spielt nah."));
    extract::prove(SOURCE, "public/test.html", html.as_bytes(), &assertions).unwrap();
}

#[test]
fn section_subheadings_preserve_the_first_heading_and_full_body() {
    let path = "internal/test.html";
    let html = HTML.replace(
        "</section>",
        "<h3>Details</h3><p>Weitere Hinweise.</p></section>",
    );
    let assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    let body = "Spielweise Billy spielt nah. Details Weitere Hinweise.";
    assert_eq!(assertions.len(), 2);
    assert_eq!(
        docs_extract::metadata(&assertions[1]).unwrap().heading_path,
        ["Billy", "Spielweise"]
    );
    assert!(matches!(&assertions[1].value, Atom::Text(actual) if actual == body));
    assert_eq!(assertions[1].hash, Sha256::digest(body.as_bytes()).to_vec());
    extract::prove(SOURCE, path, html.as_bytes(), &assertions).unwrap();
}

#[test]
fn html5_repaired_paragraph_membership_is_not_a_receipt() {
    let path = "public/test.html";
    let html = HTML
        .replace("<h1>Billy</h1>", "<h1>T</h1>")
        .replace("<p>Einleitung &amp; Hinweis.</p>", "<p>Intro <nav>Ignored</nav>tail</p>");
    let document = Html::parse_document(&html);
    let selector = Selector::parse(docs_extract::INTRO_SELECTOR).unwrap();
    let selected = document
        .select(&selector)
        .flat_map(|element| element.text())
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(docs_extract::normalize(&selected), "T Intro");
    assert!(extract::extract(SOURCE, path, html.as_bytes()).is_err());
    let mut forged = extract::extract(SOURCE, path, HTML.as_bytes()).unwrap();
    forged[0].value = Atom::Text("T Intro tail".into());
    forged[0].hash = Sha256::digest(b"T Intro tail").to_vec();
    assert!(extract::prove(SOURCE, path, html.as_bytes(), &forged[..1]).is_err());
}

#[test]
fn ancestor_paragraph_inline_list_and_table_repairs_fail_closed() {
    let path = "public/test.html";
    for paragraph in [
        "<p>Intro <span><nav>Ignored</nav>tail</span></p>",
        "<p>Intro <span><aside>Ignored</aside>tail</span></p>",
        "<p>Intro <span><address>Adresse</address>tail</span></p>",
        "<p>Intro <span><details><summary>Mehr</summary>Details</details>tail</span></p>",
        "<p>Intro <span><fieldset>Feld</fieldset>tail</span></p>",
        "<p>Intro <span><figure>Bild</figure>tail</span></p>",
        "<p>Intro <span><footer>Fuß</footer>tail</span></p>",
        "<p>Intro <span><header>Kopf</header>tail</span></p>",
        "<p>Intro <span><menu><li>Menü</li></menu>tail</span></p>",
        "<p>Intro <span><p>Zweiter Absatz</p>tail</span></p>",
        "<p>Intro <nobr>A<nobr>B</nobr>C</nobr></p>",
    ] {
        let html = HTML.replace("<p>Einleitung &amp; Hinweis.</p>", paragraph);
        assert!(extract::extract(SOURCE, path, html.as_bytes()).is_err(), "{paragraph}");
    }
    for content in [
        "<table><tr><td>Zelle</td></tr></table>",
        "<table>Verschoben<tbody><tr><td>Zelle</td></tr></tbody></table>",
        "<table><tbody><tr><td><span><tr><td>Repariert</td></tr></span></td></tr></tbody></table>",
        "<ul><li>A<span><li>B</li>C</span></li></ul>",
        "<dl><dt>A<span><dd>B</dd>C</span></dt></dl>",
        "<button>A<button>B</button>C</button>",
        "<p><b><div>Absatz</div>Weiter</b></p>",
    ] {
        let html = HTML.replace("<p>Billy <strong>spielt</strong> nah.</p>", content);
        assert!(extract::extract(SOURCE, path, html.as_bytes()).is_err(), "{content}");
    }
}

#[test]
fn unrepaired_explicit_table_inline_and_excluded_subtrees_keep_receipts() {
    let html = HTML.replace(
        "<p>Billy <strong>spielt</strong> nah.</p>",
        "<p>Billy <span><strong>spielt</strong></span> nah.</p><table><tbody><tr><td>Zelle</td></tr></tbody></table><template><section id=\"inert\"><h3>Inert</h3><p>Unsichtbar</p></section></template>",
    );
    let assertions = extract::extract(SOURCE, "public/test.html", html.as_bytes()).unwrap();
    assert_eq!(assertions.len(), 2);
    assert!(matches!(&assertions[1].value, Atom::Text(body) if body == "Spielweise Billy spielt nah. Zelle"));
    extract::prove(SOURCE, "public/test.html", html.as_bytes(), &assertions).unwrap();
}

#[test]
fn raw_attribute_value_newlines_and_closing_tags_have_physical_spans() {
    let path = "public/test.html";
    for (opening, end) in [
        ("<section id=\n\"spielweise\">", 11),
        ("<section id=\n'spielweise'>", 11),
        ("<section id=\nspielweise>", 11),
        ("<section id=\n\n\"spielweise\">", 12),
        ("<section id=\n'spielweise'\ndata-note=\"a >\nb <section> c\">", 13),
    ] {
        let html = HTML.replace("<section id=\"spielweise\">", opening);
        for raw in [html.clone(), html.replace('\n', "\r\n")] {
            let mut assertions = extract::extract(SOURCE, path, raw.as_bytes()).unwrap();
            let metadata = docs_extract::metadata(&assertions[1]).unwrap();
            assert_eq!((metadata.line_start, metadata.line_end), (6, end), "{opening}");
            extract::prove(SOURCE, path, raw.as_bytes(), &assertions).unwrap();
            assertions[1].condition["line_end"] = 10.into();
            assert!(extract::prove(SOURCE, path, raw.as_bytes(), &assertions[1..]).is_err());
        }
    }
    let html = HTML.replace("</section>", "</section\n>");
    let assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    let metadata = docs_extract::metadata(&assertions[1]).unwrap();
    assert_eq!((metadata.line_start, metadata.line_end), (6, 11));
    extract::prove(SOURCE, path, html.as_bytes(), &assertions).unwrap();
}

#[test]
fn comments_raw_text_and_quoted_fake_tags_do_not_move_span_boundaries() {
    let path = "public/test.html";
    let html = HTML
        .replace("<section id=\"spielweise\">", "<!-- <section id='falsch'>\n</section> -->\n<section data-note=\"<section> >\nHinweis\" id=\n\"spielweise\">")
        .replace("let fake = '<section id=\"fake\">';", "let fake = '<section id=\"fake\">';\nlet other = '</section> >';\n")
        .replace("</section>\n</main>", "</section\n>\n</main>");
    for raw in [html.clone(), html.replace('\n', "\r\n")] {
        let assertions = extract::extract(SOURCE, path, raw.as_bytes()).unwrap();
        assert_eq!(assertions.len(), 2);
        let metadata = docs_extract::metadata(&assertions[1]).unwrap();
        assert_eq!((metadata.line_start, metadata.line_end), (8, 17));
        assert!(matches!(&assertions[1].value, Atom::Text(body) if body == "Spielweise Billy spielt nah."));
        extract::prove(SOURCE, path, raw.as_bytes(), &assertions).unwrap();
    }
}

#[test]
fn double_escaped_script_text_keeps_the_real_closing_tag_boundary() {
    let path = "public/test.html";
    let html = HTML.replace(
        "let fake = '<section id=\"fake\">';",
        "<!--<script>\n</script>\n-->\n",
    );
    for raw in [html.clone(), html.replace('\n', "\r\n")] {
        let assertions = extract::extract(SOURCE, path, raw.as_bytes()).unwrap();
        let metadata = docs_extract::metadata(&assertions[1]).unwrap();
        assert_eq!((metadata.line_start, metadata.line_end), (6, 13));
        assert!(matches!(&assertions[1].value, Atom::Text(body) if body == "Spielweise Billy spielt nah."));
        extract::prove(SOURCE, path, raw.as_bytes(), &assertions).unwrap();
    }
}

#[test]
fn original_nested_sections_have_independent_css_receipts_and_ancestor_headings() {
    let path = "internal/test.html";
    let html = "<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title></head><body>\n<main>\n<h1>Billy</h1>\n<p>Einleitung.</p>\n<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Außen.</p>\n<div><section id=\"details\">\n<h3>Details</h3>\n<p>Innen.</p>\n<section id=\"taktik\">\n<h4>Taktik</h4><p>Nah.</p>\n</section>\n<nav><section id=\"navigation\"><h4>Navigation</h4></section></nav>\n</section></div>\n<p>Danach.</p>\n</section>\n<section id=\"abschluss\"><h2>Abschluss</h2><p>Ende.</p></section>\n</main></body></html>";
    let mut assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    assert_eq!(assertions.len(), 5);
    for (index, selector, headings, lines, body) in [
        (1, "main > section[id=\"spielweise\"]", vec!["Billy", "Spielweise"], (6, 18), "Spielweise Außen. Details Innen. Taktik Nah. Danach."),
        (2, "main section[id=\"details\"]", vec!["Billy", "Spielweise", "Details"], (9, 16), "Details Innen. Taktik Nah."),
        (3, "main section[id=\"taktik\"]", vec!["Billy", "Spielweise", "Details", "Taktik"], (12, 14), "Taktik Nah."),
        (4, "main > section[id=\"abschluss\"]", vec!["Billy", "Abschluss"], (19, 19), "Abschluss Ende."),
    ] {
        let assertion = &assertions[index];
        assert_eq!(assertion.locator, selector);
        let metadata = docs_extract::metadata(assertion).unwrap();
        assert_eq!(metadata.heading_path, headings);
        assert_eq!(metadata.ordinal, index as i32);
        assert_eq!((metadata.line_start, metadata.line_end), lines);
        assert!(matches!(&assertion.value, Atom::Text(actual) if actual == body));
        assert_eq!(assertion.hash, Sha256::digest(body.as_bytes()).to_vec());
        let document = Html::parse_document(html);
        let selector = Selector::parse(selector).unwrap();
        assert_eq!(document.select(&selector).count(), 1);
        extract::prove(SOURCE, path, html.as_bytes(), std::slice::from_ref(assertion)).unwrap();
    }
    extract::prove(SOURCE, path, html.as_bytes(), &assertions).unwrap();
    assertions[2].condition["heading_path"] = serde_json::json!(["Billy", "Details"]);
    assert!(extract::prove(SOURCE, path, html.as_bytes(), &assertions[2..3]).is_err());
    let mut assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    assertions[2].locator = "main > section[id=\"details\"]".into();
    assert!(extract::prove(SOURCE, path, html.as_bytes(), &assertions[2..3]).is_err());
}

#[test]
fn nested_receipts_enforce_heading_depth_count_and_body_limits() {
    let path = "public/test.html";
    let nested = |count| {
        let mut html = "<!doctype html><html lang=\"de\"><head><title>T</title></head><body><main><h1>T</h1>".to_string();
        for index in 0..count {
            html.push_str(&format!("<section id=\"s{index}\"><h2>H{index}</h2>"));
        }
        html.push_str("<p>Text.</p>");
        html.push_str(&"</section>".repeat(count));
        html.push_str("</main></body></html>");
        html
    };
    let html = nested(15);
    let assertions = extract::extract(SOURCE, path, html.as_bytes()).unwrap();
    assert_eq!(assertions.len(), 16);
    assert_eq!(docs_extract::metadata(&assertions[15]).unwrap().heading_path.len(), 16);
    extract::prove(SOURCE, path, html.as_bytes(), &assertions).unwrap();
    assert!(extract::extract(SOURCE, path, nested(16).as_bytes()).is_err());
    let oversized = HTML.replace("<p>Billy <strong>spielt</strong> nah.</p>", &format!("<p>{}</p>", "x".repeat(262144)));
    assert!(extract::extract(SOURCE, path, oversized.as_bytes()).is_err());
    let sections = (0..4096).map(|index| format!("<section id=\"s{index}\"><h2>H</h2><p>Text.</p></section>")).collect::<String>();
    let html = HTML.replace("<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Billy <strong>spielt</strong> nah.</p>\n<nav>Navigation</nav><script>let fake = '<section id=\"fake\">';</script><p hidden>Unsichtbar</p>\n</section>", &sections);
    assert!(extract::extract(SOURCE, path, html.as_bytes()).is_err());
}

#[test]
fn nested_css_bodies_share_the_page_budget_even_when_raw_bytes_fit() {
    let body = "x".repeat(120000);
    let mut html = "<!doctype html><html lang=\"de\"><head><title>T</title></head><body><main><h1>T</h1>".to_string();
    for index in 0..35 {
        html.push_str(&format!("<section id=\"outer-{index}\"><h2>Außen</h2><section id=\"inner-{index}\"><h3>Innen</h3><p>{body}</p></section></section>"));
    }
    html.push_str("</main></body></html>");
    assert!(html.len() < docs_extract::MAX_BYTES);
    assert!(extract::extract(SOURCE, "public/test.html", html.as_bytes()).is_err());
}

#[test]
fn malformed_ambiguous_and_unsafe_inputs_fail_closed() {
    let path = "public/test.html";
    for html in [
        HTML.replace("id=\"spielweise\"", "id=\"spielweise\" id=\"zweites\""),
        HTML.replace("<nav>", "<nav id=\"spielweise\">"),
        HTML.replace("</main>", "<main></main></main>"),
        HTML.replace("</section>", ""),
        HTML.replace("id=\"spielweise\"", "id=\"unsicher\\\"id\""),
        HTML.replace("<h1>Billy</h1>", "<h1>Billy</h1><h1>Mehrdeutig</h1>"),
        HTML.replace("<h1>Billy</h1>", "<div><h1>Billy</h1></div>"),
        HTML.replace("<h1>Billy</h1>", "<h1 hidden>Billy</h1>"),
        HTML.replace("</main>", "<p>Späterer Absatz</p></main>"),
        HTML.replace("</body>", "</body><body><p>Zweiter Körper</p></body>"),
        HTML.replace("<body>", "<head><title>Zweiter Kopf</title></head><body>"),
        HTML.replace("</html>", "</html><p>Außerhalb</p>"),
        HTML.replace("lang=\"de\"", "lang=\"de-Latn-DE\""),
    ] {
        assert!(extract::extract(SOURCE, path, html.as_bytes()).is_err());
    }
    for path in [
        "public/../test.html",
        "public/%2e%2e/test.html",
        "public/test.html?x",
        "internal/x\\y.html",
        "public//test.html",
        "../public/test.html",
    ] {
        assert!(!docs_extract::supported(path));
    }
    let mut invalid = HTML.as_bytes().to_vec();
    invalid[0] = 255;
    assert!(extract::extract(SOURCE, path, &invalid).is_err());
    assert!(extract::extract(SOURCE, path, &vec![b' '; docs_extract::MAX_BYTES + 1]).is_err());
    let facts = extract::extract(SOURCE, path, HTML.as_bytes()).unwrap();
    let mut metadata = docs_extract::metadata(&facts[0]).unwrap();
    metadata.line_start = 0;
    assert!(docs_extract::section_url(path, &"a".repeat(40), &metadata).is_err());
    metadata.line_start = 1;
    for revision in ["a".repeat(64), "A".repeat(40), "../escape".into()] {
        assert!(docs_extract::section_url(path, &revision, &metadata).is_err());
    }
}

#[test]
fn published_aliases_match_normalized_whole_words_without_guesses() {
    let aliases = [
        ("billy".into(), 9),
        ("grey talon".into(), 5),
        ("ivy".into(), 1),
        ("billy".into(), 10),
    ];
    assert_eq!(
        docs_extract::linked_entities("BILLY, und Grey   Talon. Privy und Billys", &aliases)
            .unwrap(),
        [5, 9, 10]
    );
    assert!(docs_extract::linked_entities("Billys billy_hero", &aliases)
        .unwrap()
        .is_empty());
    let aliases: Vec<_> = (1..=65).map(|id| (format!("e{id}"), id)).collect();
    let body = aliases
        .iter()
        .map(|(alias, _)| alias.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(docs_extract::linked_entities(&body, &aliases).is_err());
}
