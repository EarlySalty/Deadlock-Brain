#[path = "../src/docs_extract.rs"]
mod docs_extract;
use docs_extract::{
    parse, replay, DocumentProjection, ParsingBounds,
    ReplayCoverage::{Complete, Subset},
};
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};
const BOUNDS: ParsingBounds = ParsingBounds {
    raw_bytes: 5_000_000,
    tokens: 100_000,
    depth: 64,
    sections: 1000,
    output_bytes: 5_000_000,
};
const HTML: &str = "<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title><style>main { color: red; }</style></head><body>\n<main>\n<h1>Billy</h1>\n<p>Einleitung &amp; Hinweis.</p>\n<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Billy <strong>spielt</strong> nah.</p>\n<nav>Navigation</nav><script>let fake = '<section id=\"fake\">';</script><p hidden>Unsichtbar</p>\n</section>\n</main></body></html>";
const BODY: &str = "Spielweise Billy spielt nah.";
const NESTED: &str = "<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title></head><body>\n<main>\n<h1>Billy</h1>\n<p>Einleitung.</p>\n<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Außen.</p>\n<div><section id=\"details\">\n<h3>Details</h3>\n<p>Innen.</p>\n<section id=\"taktik\">\n<h4>Taktik</h4><p>Nah.</p>\n</section>\n<nav><section id=\"navigation\"><h4>Navigation</h4></section></nav>\n</section></div>\n<p>Danach.</p>\n</section>\n<section id=\"abschluss\"><h2>Abschluss</h2><p>Ende.</p></section>\n</main></body></html>";
const REPAIRS: &str = "<p>Intro <nav>Ignored</nav>tail</p>|<p>Intro <span><nav>Ignored</nav>tail</span></p>|<p>Intro <span><aside>Ignored</aside>tail</span></p>|<p>Intro <span><address>Adresse</address>tail</span></p>|<p>Intro <span><details><summary>Mehr</summary>Details</details>tail</span></p>|<p>Intro <span><fieldset>Feld</fieldset>tail</span></p>|<p>Intro <span><figure>Bild</figure>tail</span></p>|<p>Intro <span><footer>Fuß</footer>tail</span></p>|<p>Intro <span><header>Kopf</header>tail</span></p>|<p>Intro <span><menu><li>Menü</li></menu>tail</span></p>|<p>Intro <span><p>Zweiter Absatz</p>tail</span></p>|<p>Intro <nobr>A<nobr>B</nobr>C</nobr></p>|<table><tr><td>Zelle</td></tr></table>|<table>Verschoben<tbody><tr><td>Zelle</td></tr></tbody></table>|<table><tbody><tr><td><span><tr><td>Repariert</td></tr></span></td></tr></tbody></table>|<ul><li>A<span><li>B</li>C</span></li></ul>|<dl><dt>A<span><dd>B</dd>C</span></dt></dl>|<button>A<button>B</button>C</button>|<p><b><div>Absatz</div>Weiter</b></p>";
const MALFORMED: &str = "id=\"spielweise\"~id=\"spielweise\" id=\"zweites\"|<nav>~<nav id=\"spielweise\">|</main>~<main></main></main>|</section>~|id=\"spielweise\"~id=\"unsicher\\\"id\"|<h1>Billy</h1>~<h1>Billy</h1><h1>Mehrdeutig</h1>|<h1>Billy</h1>~<div><h1>Billy</h1></div>|<h1>Billy</h1>~<h1 hidden>Billy</h1>|</main>~<p>Späterer Absatz</p></main>|</body>~</body><body><p>Zweiter Körper</p></body>|<body>~<head><title>Zweiter Kopf</title></head><body>|</html>~</html><p>Außerhalb</p>";
const OPENINGS: &str = "<section id=\n\"spielweise\">~11|<section id=\n'spielweise'>~11|<section id=\nspielweise>~11|<section id=\n\n\"spielweise\">~12|<section id=\n'spielweise'\ndata-note=\"a >\nb <section> c\">~13";
const EXPECTED: &str = "spielweise;6;18;Billy/Spielweise;Spielweise Außen. Details Innen. Taktik Nah. Danach.|details;9;16;Billy/Spielweise/Details;Details Innen. Taktik Nah.|taktik;12;14;Billy/Spielweise/Details/Taktik;Taktik Nah.|abschluss;19;19;Billy/Abschluss;Abschluss Ende.";
fn extracted(html: &str) -> DocumentProjection {
    parse(html.as_bytes(), BOUNDS).expect("fixture extraction")
}
fn spans(html: &str, lines: (u64, u64)) {
    let actual = extracted(html);
    assert_eq!(actual.sections[1].lines, lines);
    assert_eq!(actual.sections[1].body, BODY);
    assert!(replay(html.as_bytes(), BOUNDS, &actual, Complete).is_ok());
    let mut forged = actual;
    forged.sections[1].lines.1 = 0;
    assert!(replay(html.as_bytes(), BOUNDS, &forged, Subset).is_err());
}
#[test]
fn bodies_visibility_and_hashes() {
    let original = extracted(HTML);
    assert_eq!(original.sections[0].body, "Billy Einleitung & Hinweis.");
    assert_eq!(original.sections[0].locator, "main > h1, main > p");
    for (content, expected) in [
        ("<textarea>Geheim</textarea><audio>Ton</audio><video>Film</video><noembed>Ersatz</noembed><noframes>Rahmen</noframes><form>Formular</form><svg><text>Bild</text></svg><canvas>Zeichenfläche</canvas><iframe>Rahmen</iframe><object>Objekt</object><select><option>Auswahl</option></select><noscript>Ersatz</noscript><p aria-hidden=\"true\">Versteckt</p>", BODY),
        ("<h3>Details</h3><p>Weitere Hinweise.</p>", "Spielweise Billy spielt nah. Details Weitere Hinweise."),
        ("<table><tbody><tr><td>Zelle</td></tr></tbody></table><template><section id=\"inert\"><h3>Inert</h3><p>Unsichtbar</p></section></template>", "Spielweise Billy spielt nah. Zelle"),
    ] {
        let html = HTML.replace("</section>\n</main>", &format!("{content}</section>\n</main>"));
        let actual = extracted(&html);
        assert_eq!(actual.sections.len(), 2);
        assert_eq!(actual.sections[1].body, expected);
        assert_eq!(actual.sections[1].heading_path, ["Billy", "Spielweise"]);
        assert_eq!(actual.sections[1].value_sha256, <[u8; 32]>::from(Sha256::digest(expected.as_bytes())));
        assert!(replay(html.as_bytes(), BOUNDS, &actual, Complete).is_ok());
    }
}
#[test]
fn physical_multiline_crlf_comments_and_script_spans() {
    for row in OPENINGS.split('|') {
        let (opening, end) = row.split_once('~').expect("opening");
        let end = end.parse().expect("physical line");
        let html = HTML.replace("<section id=\"spielweise\">", opening);
        for raw in [html.clone(), html.replace('\n', "\r\n")] {
            spans(&raw, (6, end));
        }
    }
    let html = HTML.replace("<h1>", "<h1\nclass=\"title\">").replace(
        "<section id=\"spielweise\">",
        "<section\nid=\"spielweise\">",
    );
    let intro = &extracted(&html).sections[0];
    assert_eq!(intro.lines, (4, 6));
    spans(&html, (7, 12));
    spans(&HTML.replace("</section>", "</section\n>"), (6, 11));
    let commented = HTML.replace("<section id=\"spielweise\">", "<!-- <section id='falsch'>\n</section> -->\n<section data-note=\"<section> >\nHinweis\" id=\n\"spielweise\">").replace("let fake = '<section id=\"fake\">';", "let fake = '<section id=\"fake\">';\nlet other = '</section> >';\n").replace("</section>\n</main>", "</section\n>\n</main>");
    let escaped = HTML.replace(
        "let fake = '<section id=\"fake\">';",
        "<!--<script>\n</script>\n-->\n",
    );
    for (html, lines) in [(commented, (8, 17)), (escaped, (6, 13))] {
        for raw in [html.clone(), html.replace('\n', "\r\n")] {
            spans(&raw, lines);
        }
    }
}
#[test]
fn repairs_and_malformed_inputs_fail_closed() {
    for content in REPAIRS.split('|') {
        let html = HTML.replace("<p>Billy <strong>spielt</strong> nah.</p>", content);
        assert!(parse(html.as_bytes(), BOUNDS).is_err(), "{content}");
        assert!(replay(html.as_bytes(), BOUNDS, &extracted(HTML), Subset).is_err());
    }
    for row in MALFORMED.split('|') {
        let (find, replacement) = row.split_once('~').expect("mutation");
        assert!(parse(HTML.replace(find, replacement).as_bytes(), BOUNDS).is_err());
    }
    assert!(parse(HTML.replace('\n', "\r").as_bytes(), BOUNDS).is_err());
    assert!(parse(&[255], BOUNDS).is_err());
}
#[test]
fn nested_receipts_preserve_css_ancestry_and_detect_tampering() {
    let actual = extracted(NESTED);
    assert_eq!(actual.sections.len(), 5);
    let document = Html::parse_document(NESTED);
    for (offset, row) in EXPECTED.split('|').enumerate() {
        let fields: Vec<_> = row.split(';').collect();
        let section = &actual.sections[offset + 1];
        assert_eq!(section.ordinal, offset + 1);
        assert_eq!(section.source_key.as_deref(), Some(fields[0]));
        let lines: (u64, u64) = (
            fields[1].parse().expect("start"),
            fields[2].parse().expect("end"),
        );
        assert_eq!(section.lines, lines);
        let headings: Vec<_> = fields[3].split('/').collect();
        assert_eq!(section.heading_path, headings);
        assert_eq!(section.body, fields[4]);
        let selector = Selector::parse(&section.locator).expect("CSS");
        assert_eq!(document.select(&selector).count(), 1);
        let mut subset = actual.clone();
        subset.sections = vec![section.clone()];
        assert!(replay(NESTED.as_bytes(), BOUNDS, &subset, Subset).is_ok());
        assert!(replay(NESTED.as_bytes(), BOUNDS, &subset, Complete).is_err());
    }
    let mutations: [fn(&mut DocumentProjection); 9] = [
        |d| d.sections[2].lines.0 = 1,
        |d| d.sections[2].heading_path = vec!["Billy".into(), "Details".into()],
        |d| d.sections[2].locator = "main > section[id=\"details\"]".into(),
        |d| d.sections[2].body = "Verändert".into(),
        |d| d.sections[2].value_sha256[0] ^= 1,
        |d| d.sections[2].source_key = Some("anderes".into()),
        |d| d.sections[2].ordinal = 1,
        |d| d.lang = "en".into(),
        |d| d.title = "Verändert".into(),
    ];
    for mutate in mutations {
        let mut forged = actual.clone();
        mutate(&mut forged);
        assert!(replay(NESTED.as_bytes(), BOUNDS, &forged, Subset).is_err());
    }
    let mut duplicate = actual.clone();
    duplicate.sections.push(actual.sections[2].clone());
    assert!(replay(NESTED.as_bytes(), BOUNDS, &duplicate, Subset).is_err());
    let changed = NESTED.replace("Innen.", "Verändert.");
    assert!(replay(changed.as_bytes(), BOUNDS, &actual, Subset).is_err());
}
#[test]
fn supplied_structural_bounds_and_original_keys() {
    let mutations: [fn(&mut ParsingBounds); 5] = [
        |b| b.raw_bytes = HTML.len() - 1,
        |b| b.tokens = 5,
        |b| b.depth = 3,
        |b| b.sections = 1,
        |b| b.output_bytes = 10,
    ];
    for mutate in mutations {
        let mut bounds = BOUNDS;
        mutate(&mut bounds);
        assert!(parse(HTML.as_bytes(), bounds).is_err());
    }
    let html = HTML.replace(
        "</section>\n</main>",
        "</section><section id=\"Spielweise\"><h2>Weitere</h2></section>\n</main>",
    );
    let actual = extracted(&html);
    assert_eq!(actual.sections[1].source_key.as_deref(), Some("spielweise"));
    let section = &actual.sections[2];
    assert_eq!(section.source_key.as_deref(), Some("Spielweise"));
    assert_eq!(section.locator, "main > section[id=\"Spielweise\"]");
    let html = HTML.replace("id=\"spielweise\"", "id=\"mit&amp;&quot;Zitat\"");
    let actual = extracted(&html);
    let section = &actual.sections[1];
    assert_eq!(section.source_key.as_deref(), Some("mit&\"Zitat"));
    assert!(replay(html.as_bytes(), BOUNDS, &actual, Complete).is_ok());
    let body = "x".repeat(120000);
    let mut html = "<!doctype html><html lang=\"de\"><body><main><h1>T</h1>".to_string();
    for index in 0..35 {
        html.push_str(&format!("<section id=\"outer-{index}\"><h2>Außen</h2><section id=\"inner-{index}\"><h3>Innen</h3><p>{body}</p></section></section>"));
    }
    html.push_str("</main></body></html>");
    assert!(html.len() < BOUNDS.raw_bytes);
    assert!(parse(html.as_bytes(), BOUNDS).is_err());
}
