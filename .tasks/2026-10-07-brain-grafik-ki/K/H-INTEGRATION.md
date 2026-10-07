# K: Bestätigte H-Integration und zwei Randfixes

H-Teilübergabe angenommen: Code 26859fda4b5e77a29b3af4cea0411d304a04eb2f, Nachweis-HEAD 65f33cb1aadef755db0d2ff6631342d04fa398b3. Drei Quelldateien vor Änderungen bytegleich mit diesen bestätigten Gitständen. Native K-Fortsetzung ad9015ecebebc29f1 hat Renderer, Tests und vorhandene Vorschau selektiv übernommen, keine ganze H-Branchintegration und kein Renderer-Neubau. H-Worktree unverändert.

## Änderung

`brain-maintenance/src/hero_compare_render.rs`: vollständige gültige lange Heldennamen mit reservierter SVG-Breite, zentrale Ablehnung XML-unzulässiger U+FFFE/U+FFFF. Zahlen, Bindungen und vollständige 36-Punkt-Reihe erhalten. Keine Spielrechnung oder andere Datenquelle.

`tests/hero_compare_render.rs`: zwölf vorhandene H-Fälle plus zwei Regressionstests, jetzt über den echten K-Modulexport. `examples/hero_compare_preview.rs`: vorhandene Vorschau um synthetische 40-Zeichen-Namen ergänzt. `src/lib.rs`: ausschließlich pub mod hero_compare_render ergänzt. Keine Manifeste, Lockdateien, aktiven G-/E-/F-/I-Dateien oder Botpfade geändert.

## Eigene Prüfung

Gepinnte Toolchain +1.97.1, bestehender sccache und zentraler Buildcache `/home/nathanael/.cache/rust-build/{workspace-path-hash}`, tatsächlich gehaltene Slots 1/2, `SQLX_OFFLINE=true`, `--locked --offline --jobs 3`.

Compiler und striktes Clippy geprüft für `-p brain-maintenance --lib --test hero_compare_render --example hero_compare_preview`, Clippy zusätzlich `--no-deps -- -D warnings`, jeweils Exit 0. Format/Rustfmt und Vorschau-Build Exit 0. Kein pauschales aktuelles Cargo-1.99- oder reposweites Clippyurteil.

Tests jeweils mit `--include-ignored --test-threads=1`: Integrationstest hero_compare_render 14 passed/0 failed/0 ignored; vorhandene lib html::tests 5 passed/0 failed/0 ignored/55 filtered; entity_profile_render::tests 17 passed/0 failed/0 ignored/43 filtered. Zusammen 36 passed, kein vollständiger Maintenance-Testlauf. Haupt-K hat echte Logmarker und Compiler-/Clippyexits nachgelesen.

Rohlogs `/tmp/brain-k-hero-compare-{format,cargo-format,check,clippy,tests,html-tests,profile-tests,preview-build}.log`. Tatsächlich erzeugtes synthetisches Vorschau-SVG zusätzlich als XML geparst; vier vollständige Langnamenlabels innerhalb der Zeichenfläche bestätigt.

TESTNACHWEIS[TW-1]: 36 passed, 0 ignored | Baseline: 0 rot

## Sicht- und Veröffentlichungsgrenze

Keine neue Screenshotabnahme. Nativer lokaler Screenshotversuch ohne Ergebnis, eigene Browser/Vorschauen beendet. Haupt-K prüfte T3-Browserverfügbarkeit zusätzlich: `No preview automation host is available for open in environment e5040875-1ea7-43d3-9664-2a076d8443f5.` Kein neuer Browser installiert oder fremde Session als Ausweg benutzt. Vorhandene H-Desktop-/Mobilsichtung bleibt synthetische Vorschau, nicht echte G-/Liveabnahme.

G liefert Berechnung, strukturierte Reihen, Szenario, Version und belegte Werkzeug-/Quellenabhängigkeiten. K verantwortet die eigene Artefakthülle, Artefakt-ID, Bindung an HTML/SVG, Veröffentlichungsquittung, Postgres-Speicherung, Rechte und Auslieferung. Dafür fehlt keine weitere Produktfreigabe. Der reale G-Vertragsentwurf darf lesend für den Anschluss geprüft werden; aktive G-Dateien bleiben unverändert und G-WIP wird nicht als verifiziert übernommen. Ohne freigegebenen echten G-Eingang bleiben Veröffentlichung und Livebeweis gesperrt. Die bestehende Site-Allowlist öffnet noch keinen Vergleichsnamespace. Eigene Quellen sichern und lokalen Gate ausführen, kein Main-/Deploy-/Liveabschluss behaupten.
