# S12-Prüfbericht — Offline-Vorbereitung

Stand: 24.09.2026. Getesteter Code-Commit: `b93002f744ecb86cb3d0decee4067a6331c984c3`.

**Ergebnis: 73 Tests bestanden, 0 fehlgeschlagen, 0 ignoriert. Produktionsfreigabe und echte Integration: nicht gegeben.**

## Tatsächlich ausgeführt

Reproduzierbarer Gesamtbefehl aus dem Repo:

```sh
PATH="$HOME/.cargo/bin:$PATH" bash architecture/migration/s12/check.sh
```

Auf anderen Rechnern Rust/Cargo regulär auf PATH bereitstellen; keine bestimmte Benutzerinstallation voraussetzen. Toolchain des gemessenen Laufs: Rust 1.97.1. Alle Cargo-Aufrufe gegen den paketlokalen Lockfile, ohne Netzwerk. Vollständige normalisierte Ausgabe: `checks.log`; Maschinenbericht mit Quellfile-Hashes: `test-results.json`.

| Prüfung | Ergebnis |
|---|---|
| `cargo fmt --all -- --check` im S12-Workspace | bestanden |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | bestanden |
| `cargo test --all-targets --locked --offline` | 64 Fixture-/Regressionstests + 9 CLI-Tests bestanden |
| `cargo build --release --locked --offline` | bestanden |
| CLI `analyze` auf synthetischem Pilot | Exit 0, Bericht ausdrücklich `prepare_only` |
| CLI `delta` auf unveränderten Captures | keine Reparse-/Reprojektionsanforderungen, 0 Embeddingjobs |
| CLI `analyze --require-production-ready` | erwarteter Exit 3, `production_publishable=false` |
| `git diff --check` / gestagter Diff vor Code-Commit | bestanden |

Der identische Code-Commit wurde nach dem Commit erneut durch `check.sh` geprüft. Die anschließenden Bericht-/Übergabedateien ändern keine getesteten Rust-Quellen, Fixtures, Dependencies oder Prüfskripte.

## Gezielte Gegenprüfung statt nur erster grüner Lauf

Ein adversarieller Nachtest zeigte, dass der erste Delta-Entwurf gleiche Revisions-IDs mit geändertem Content oder geänderter Quellzeit zwischen zwei Captures noch nicht zurückwies. Beide Tests wurden zunächst **rot** ausgeführt (`regression-red.log`). Der endgültige Code prüft die Unveränderlichkeit aller gemeinsam vorhandenen sichtbaren Revisionen captureübergreifend, einschließlich Contentmodell. Die Regressionstests sind Bestandteil der 73 bestandenen Tests. Rechteentzug/Suppression dürfen Inhalte dagegen entfernen und lösen sichere Invalidierung aus.

Zusätzliche Negativfälle: fehlende/zyklische Continuation, unbekannte Namespaces/Entities, doppelte JSON-Keys, falsche Page-ID, übergroße/nestende Eingaben, fehlende oder unterdrückte Head-Revision, Zukunftszeit, ältere Revision, Umbenennung während eines Captures, Quarantäne, aktueller ACL-Revoke trotz historischem Allow, unbekannte Template-/Lua-/HTML-Semantik, unvollständige Dependencies, Exclusion von Buildpflichtseiten, unzulässige Aktivierungsargumente und fehlende Offline-Freigabe.

## Synthetischer Korpus — kein Live-Denominator

Der frei erstellte Capture enthält 19 Seiten, 20 Revisionen, fünf enumerierbare Namespaces plus einen virtuellen Namespace. Der Hauptpilot enthält einen Hero mit zwei Abilities, zwei Items, zwei getrennten Mechaniken, einer Regel, Alias und zwei Hero-Revisionen. Ein unabhängiger zweiter Hero prüft selektive Invalidierung. DE/EN sind zwei Referenzvorschauen derselben Quellen, keine separaten Zahlenbestände.

Coverage: 19 entdeckt/abgerufen, 18 syntaktisch gelesen, 1 erwartbar quarantiniertes Scribunto-Modul; **0 normalisiert, 0 kanonisch validiert, 0 veröffentlicht**. Zehn als buildkritisch markierte Seiten bleiben fachlich unvalidiert. Medien sind ausschließlich synthetische Beschreibungstexte, keine heruntergeladenen Dateien.

`SOURCE_MANIFEST.json` führt jede Seite/Revision und ihre Abhängigkeiten. `WIKI_COVERAGE.csv` trägt in jeder Zeile den synthetischen Scope und den Capture-Hash. `pilot-report.json` enthält Quellenpositionen, Unknowns, Metadaten und die drei Vorschauen. `fixtures/hero-preview.golden.json` wird mit zusätzlichen unabhängigen Abschnitts-/Hash-/Locator-Assertions geprüft.

## Nicht nachgewiesen

Kein echter Wiki-Import, keine Live-Vollständigkeit oder Rechtefreigabe; der einmalige Metadatenprobeabruf ergab HTTP 403. Keine echten kanonischen Fact-/Rule-/Effect-/HeroKnowledgeCard-Contracts, kein Store-/Worker-/Build-/API-Durchstich, keine G2/G3/G4-Abnahme, keine Postgres-/Produktivmigration. Der unveränderte Produktionsworkspace wurde nicht erneut umfassend getestet und ist nicht in den 73 Tests enthalten. Die deklarierte MSRV 1.85 wurde nicht eigens ausgeführt.

Die Fixture-Laufzeiten sind keine Last-/Latenzzusage für Produktion. Es werden keine Embeddings, Modelle, Egresskosten oder Livejobs gemessen. Das bestehende GitHub-Workspace-Compile umfasst diesen isolierten S12-Workspace nicht; CI-Anbindung bleibt ein expliziter CR an S02/S10. Lokale Testergebnisse werden nicht als grünes GitHub-Review-Gate ausgegeben.
