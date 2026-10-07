# K: SVG-Quellenbelege vollständig darstellen

Frischer nativer Fixer a40307d90f7bb8bdb hat den NIT des regulären H-Gates zu langen CJK-Bedingungen und Quellenbelegen behoben. Zwei eigene Dateien geändert: hero_compare_render.rs und tests/hero_compare_render.rs. Kein Export-, Manifest-, Storage-, Site- oder G-Diff durch diesen Fixer.

Footer-Zeilen verwenden explizite SVG-Textbreite mit spacingAndGlyphs innerhalb der Seitenränder. Vollständiger Text erhalten, keine strengeren Eingabelimits. Neuer Test deckt maximale Quellen-/Bedingungsmengen mit CJK, breitem ASCII, XML-Sonderzeichen und Nicht-BMP-Zeichen ab. Renderer deterministisch, HTML-/SVG-Belegtext gleich, Zahlenreihen unverändert. Bestehenden Langnamen-Testselektor an die zusätzlich begrenzten Footertexte angepasst.

## Prüfung

Toolchain +1.97.1, vorhandener Cache/sccache, tatsächlich gehaltener Buildslot2, `--locked --offline --jobs 3`. Format, Compiler und scoped Clippy mit `-- -D warnings` bestanden. Haupt-K prüfte tatsächliche Finished-/Testmarker. Renderer15 passed/0 failed/0 ignored; HTML5 passed/0 failed/0 ignored/55 filtered; Profilrenderer17 passed/0 failed/0 ignored/43 filtered. Kein vollständiger Maintenance-Lauf. Zwei zwischenzeitliche eigene XML-Namensraum-Testfehler korrigiert; keine unveränderte Reposuite als Baseline behauptet.

Logs /tmp/brain-k-footer-{check-retry,format-final,cargo-format,clippy-final,tests-verified,html-tests,profile-tests}.log. Noch keine neue Browser- oder echte G-/Liveabnahme. Haupt-K sichert den gezielten Fixcheckpoint und fährt den regulären Gate.

TESTNACHWEIS[TW-1]: 37 passed, 0 ignored | Baseline: 0 rot (bestätigter H-Ausgangsnachweis)
