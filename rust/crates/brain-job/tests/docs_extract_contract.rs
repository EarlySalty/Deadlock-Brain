use brain_job::docs::DocsExtractor;
pub use brain_job::*;
use brain_storage::gamefacts::receipts::verify;
use sha2::{Digest, Sha256};
const BILLY: &str = "<!doctype html>\n<html lang=\"de\">\n<head>\n  <meta charset=\"utf-8\">\n  <title>Billy (Spielweise, Build, Stärken und Schwächen)</title>\n  <meta name=\"tags\" content=\"deadlock, helden, billy, build, spielweise, guide\">\n  <meta name=\"stand\" content=\"2026-07-07\">\n  <meta name=\"quelle\" content=\"Community-Build-Analyse, automatisch abgeleitet, ohne Gewähr\">\n  <style>\n    body { font-family: system-ui, sans-serif; line-height: 1.6; max-width: 46rem; margin: 2rem auto; padding: 0 1rem; color: #1b1b1b; }\n    h1, h2 { line-height: 1.25; }\n    code { background: #f2f2f2; padding: 0.1rem 0.3rem; border-radius: 0.2rem; }\n    table { border-collapse: collapse; width: 100%; }\n    th, td { border: 1px solid #ccc; padding: 0.4rem 0.6rem; text-align: left; vertical-align: top; }\n    nav a { margin-right: 0.8rem; }\n  </style>\n</head>\n<body>\n<main>\n  <h1>Billy</h1>\n\n  <p>Kurzer Hinweis: Diese Einschätzung stammt aus einer Community-Analyse von Build-Daten und Winraten, Stand Juli 2026. Sie kann nach Patches veraltet sein, alles ohne Gewähr.</p>\n  <section id=\"wer-ist-billy-und-wie-gewinnt-man-mit-ihm\">\n    <h2>Wer ist Billy und wie gewinnt man mit ihm?</h2>\n    <p>Billy ist ein Nahkampf Brawler, dessen Gun durch Melee Treffer gefüttert wird. Sein Win Plan ist Rising Ram zum Start, Bashdown zur Kontrolle, Melee für Wrecked und Reload, danach die verstärkte Gun entladen. Er ist eher Weapon und Melee als Spirit Caster.</p>\n  </section>\n\n  <section id=\"die-faehigkeiten-kurz-erklaert\">\n    <h2>Die Fähigkeiten kurz erklärt</h2>\n    <p>Slot 1, Bashdown: Slammt im Nahkampf, zieht Gegner herunter und wirft sie hoch, zählt als Light Melee und füttert dadurch den Blasted Motor. Slot 2, Rising Ram: Charged in einen Gegner, schleudert beide in die Luft und reduziert bei Treffer effektiv die eigene Abklingzeit. Slot 3, Blasted: Lässt Melee Treffer die Gun nachladen und Wrecked anwenden, wodurch das Ziel mehr Bullet Schaden nimmt. Slot 4, Chain Gang: Verbindet nahe Gegner, nimmt ihnen Bewegungsfähigkeiten und zieht sie nach kurzer Zeit zu Billy.</p>\n  </section>\n\n  <section id=\"wie-spielt-man-billy\">\n    <h2>Wie spielt man Billy?</h2>\n    <p>Billy muss in Armlänge spielen und seine CC Kette sauber anbringen. Rising Ram ist der Zugang, danach halten Bashdown, Melee und Chain Gang die Ziele im Schussfeld. Misslingt der Einstieg, fehlt ihm ein zweiter sicherer Weg in den Kampf.</p>\n  </section>\n\n  <section id=\"was-baut-man-auf-billy\">\n    <h2>Was baut man auf Billy?</h2>\n    <p>Die Builds sind laut Analyse stark auf Vitality und Resistenzen ausgelegt, nicht auf rohen Schaden. Der Schaden kommt aus dem Kit Loop mit Wrecked, während Items Überleben, Stamina, Zugriff und Uptime kaufen. Konkrete Kern Items nennt der Auszug in diesem Abschnitt kaum, er beschreibt vor allem Kategorien wie Resist, Sustain und Mobilität.</p>\n  </section>\n\n  <section id=\"schwaechen-und-gegenmittel\">\n    <h2>Schwächen und Gegenmittel</h2>\n    <p>Billy hat keine Grundresistenzen, startet ohne Weapon Power und hat nur drei Stamina. Er ist gezwungen, auf kurze Distanz zu gehen, und wird hart bestraft, wenn Rising Ram verfehlt oder er vor Kontakt kontrolliert wird.</p>\n  </section>\n</main>\n</body>\n</html>\n";
const HTML: &str = "<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title></head><body>\n<main>\n<h1>Billy</h1>\n<p>Einleitung &amp; Hinweis.</p>\n<section id=\"spielweise\">\n<h2>Spielweise</h2>\n<p>Billy <strong>spielt</strong> nah.</p>\n<nav>Navigation</nav><script>let fake = '<section id=\"fake\">';</script><p hidden>Unsichtbar</p>\n</section>\n</main></body></html>";
const REPAIRS: &str = "<p>Intro <nav>Ignored</nav>tail</p>|<p>Intro <span><nav>Ignored</nav>tail</span></p>|<p>Intro <span><aside>Ignored</aside>tail</span></p>|<p>Intro <span><address>Adresse</address>tail</span></p>|<p>Intro <span><details><summary>Mehr</summary>Details</details>tail</span></p>|<p>Intro <span><fieldset>Feld</fieldset>tail</span></p>|<p>Intro <span><figure>Bild</figure>tail</span></p>|<p>Intro <span><footer>Fuß</footer>tail</span></p>|<p>Intro <span><header>Kopf</header>tail</span></p>|<p>Intro <span><menu><li>Menü</li></menu>tail</span></p>|<p>Intro <span><p>Zweiter Absatz</p>tail</span></p>|<p>Intro <nobr>A<nobr>B</nobr>C</nobr></p>|<table><tr><td>Zelle</td></tr></table>|<table>Verschoben<tbody><tr><td>Zelle</td></tr></tbody></table>|<table><tbody><tr><td><span><tr><td>Repariert</td></tr></span></td></tr></tbody></table>|<ul><li>A<span><li>B</li>C</span></li></ul>|<dl><dt>A<span><dd>B</dd>C</span></dt></dl>|<button>A<button>B</button>C</button>|<p><b><div>Absatz</div>Weiter</b></p>";
const MALFORMED: &str = "id=\"spielweise\"~id=\"spielweise\" id=\"zweites\"|<nav>~<nav id=\"spielweise\">|</main>~<main></main></main>|</section>~|id=\"spielweise\"~id=\"unsicher\\\"id\"|<h1>Billy</h1>~<h1>Billy</h1><h1>Mehrdeutig</h1>|<h1>Billy</h1>~<div><h1>Billy</h1></div>|<h1>Billy</h1>~<h1 hidden>Billy</h1>|</main>~<p>Späterer Absatz</p></main>|</body>~</body><body><p>Zweiter Körper</p></body>|<body>~<head><title>Zweiter Kopf</title></head><body>|</html>~</html><p>Außerhalb</p>|id=\"spielweise\"~id=\" \"|\n~\r";
const OPENINGS: &str = "<section id=\n\"spielweise\">~6:11|<section id=\n'spielweise'>~6:11|<section id=\nspielweise>~6:11|<section id=\n\n\"spielweise\">~6:12|<section id=\n'spielweise'\ndata-note=\"a >\nb <section> c\">~6:13|<!-- <section id='falsch'>\n</section> -->\n<section id=\"spielweise\">~8:12|<section id=\"spielweise\">\n<script>let other = '</section> >';\n</script>~6:12";
fn raw(html: &str) -> RawObject {
    RawObject {
        id: 1,
        source: "deadlock-docs".into(),
        revision: "a".repeat(40),
        path: "public/deadlock-helden/billy.html".into(),
        content_sha256: Sha256::digest(html.as_bytes()).into(),
        bytes: html.as_bytes().to_vec(),
    }
}
fn accepted(html: &str) -> FactSet {
    let Outcome::Accepted(set) = DocsExtractor.extract(&raw(html)) else {
        panic!("expected accepted document")
    };
    verify(html.as_bytes(), &set).expect("receipts");
    set
}
fn text<'a>(set: &'a FactSet, name: &str) -> &'a str {
    let row = set.facts.iter().find(|f| f.stat_key == name).expect("fact");
    let FactValue::Text(text) = &row.value else {
        panic!("text fact")
    };
    text
}
fn rejected(html: &str) -> Rejection {
    let Outcome::Rejected(error) = DocsExtractor.extract(&raw(html)) else {
        panic!("expected rejection")
    };
    error
}
#[test]
fn wire_receipts_and_utf8() {
    let real = accepted(BILLY);
    assert!(real.facts.len() == 16);
    let set = accepted(HTML);
    let mut forged = set.clone();
    forged.facts[4].receipt.value_sha256[0] ^= 1;
    assert!(verify(HTML.as_bytes(), &forged).is_err());
    assert!(set.entities[0].kind == "doc_page" && set.aliases.is_empty());
    assert!(text(&set, "doc_intro_body_0") == "Billy Einleitung & Hinweis.");
    assert!(text(&set, "doc_section_spielweise_body_0") == "Spielweise Billy spielt nah.");
    let start = HTML.find("<section").expect("start");
    let end = HTML.find("</section>").expect("end") + "</section>".len();
    let fact = set
        .facts
        .iter()
        .find(|f| f.stat_key.ends_with("spielweise_meta"))
        .expect("fact");
    assert!(fact.receipt.locator == format!("{start}-{end}"));
    let hash: [u8; 32] = Sha256::digest(&HTML.as_bytes()[start..end]).into();
    assert!(fact.receipt.value_sha256 == hash);
    let nested = HTML.replace(
        "</section>\n</main>",
        "<section id=\"details\"><h3>Details</h3><p>Innen.</p></section></section>\n</main>",
    );
    let set = accepted(&nested.replace('\n', ""));
    assert!(text(&set, "doc_section_spielweise_body_0").ends_with("Details Innen."));
    let child = set
        .facts
        .iter()
        .find(|f| f.stat_key.ends_with("details_meta"))
        .expect("child");
    let parent = set
        .facts
        .iter()
        .find(|f| f.stat_key.ends_with("spielweise_meta"))
        .expect("parent");
    assert!(child.receipt.locator != parent.receipt.locator);
    let body = "ä".repeat(limits::TEXT_BYTES);
    let set = accepted(&HTML.replace("Billy <strong>spielt</strong> nah.", &body));
    let chunks: Vec<_> = (0..3)
        .map(|i| text(&set, &format!("doc_section_spielweise_body_{i}")))
        .collect();
    assert!(chunks.iter().all(|c| c.len() <= limits::TEXT_BYTES));
    assert!(chunks.concat() == format!("Spielweise {body}"));
}
#[test]
fn physical_lines_and_membership() {
    for row in OPENINGS.split('|') {
        let (opening, end) = row.split_once('~').expect("opening");
        let (start, end) = end.split_once(':').expect("lines");
        let html = HTML.replace("<section id=\"spielweise\">", opening);
        for raw in [html.clone(), html.replace('\n', "\r\n")] {
            let set = accepted(&raw);
            let meta: serde_json::Value =
                serde_json::from_str(text(&set, "doc_section_spielweise_meta")).expect("metadata");
            assert!(meta["line_start"] == start.parse::<u64>().expect("line"));
            assert!(meta["line_end"] == end.parse::<u64>().expect("line"));
        }
    }
    for row in MALFORMED.split('|') {
        let (find, replacement) = row.split_once('~').expect("mutation");
        rejected(&HTML.replace(find, replacement));
    }
    for repair in REPAIRS.split('|') {
        rejected(&HTML.replace("<p>Billy <strong>spielt</strong> nah.</p>", repair));
    }
}
#[test]
fn concrete_poison_inputs() {
    let oversized = HTML.replace(
        "Billy</h1>",
        &format!("{}</h1>", "x".repeat(limits::TEXT_BYTES + 1)),
    );
    assert!(rejected(&oversized).code == "value_too_large");
    let collision = HTML.replace(
        "</section>\n</main>",
        "</section><section id=\" Spielweise \"><h2>Weitere</h2></section>\n</main>",
    );
    assert!(rejected(&collision).code == "key_collision");
    let mut long_path = raw(HTML);
    long_path.path = format!("public/{}.html", "x".repeat(180));
    assert!(matches!(
        DocsExtractor.extract(&long_path),
        Outcome::Rejected(_)
    ));
    let too_large = HTML.replace(
        "Billy <strong>spielt</strong> nah.",
        &"x".repeat(limits::BYTE_RANGE_BYTES),
    );
    assert!(rejected(&too_large).code == "value_too_large");
}
