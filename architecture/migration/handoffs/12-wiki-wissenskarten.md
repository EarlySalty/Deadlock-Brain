# Übergabe — S12 / Wiki-Integration und Wissenskarten

Status: **getestete Offline-Vorbereitung; Runtime-Implementierung/Integration blockiert**.
Arbeitsmodus: `prepare_only`; keine Gatefreigabe verändert.
Basis-Commit: `3032651` (`origin/main`, zuletzt am 24.09.2026 erneut geprüft).
Implementierungs-/getesteter Commit: `b93002f744ecb86cb3d0decee4067a6331c984c3`.
Review-Branch: `migration/s12-wiki-preparation-20260924` → `main`, als Entwurfs-PR vorgesehen.
Contract-/Schema-Version: auf integriertem main weiterhin offen.
Source-/Corpus-/Modellversion: nur `s12-capture-v1`/`s12-syntax-probe-v1`-Diagnoseformat und synthetischer Fixture-Hash; kein produktives Corpus-/Knowledge-Release, kein Modell.
Requirements: R26–R41/R47/R60 soweit S12 zugeordnet; Einzelabgleich in `s12/CR-S12-001.md`.

## Ergebnis und Dateien

Unter `architecture/migration/s12/` liegt ein eigenständiges, nicht in den Produktionsworkspace eingebundenes Rust-Werkzeug mit eigenem Cargo.toml/Cargo.lock:

- `src/discovery.rs`: dynamische Namespaces, vollständige API-Continuation, stabile Page-ID, sichtbare Pflichtlücken/Exclusions.
- `src/pages.rs`: bounded Raw-/Revisions-/Metadaten-/Dependencyprüfung, Syntaxkandidaten mit exakten JSON-/UTF-8-Locatoren, Quarantäne statt Template-/Lua-Ausführung.
- `src/preview.rs`: reproduzierbare **Referenzvorschauen**, keine kanonischen HeroKnowledgeCards und keine kopierten Stats; Quellbelege und Unknowns, DE/EN ohne zweite numerische Wahrheit.
- `src/impact.rs`: unveränderliche Revisionen, stale-/same-revision-Schutz, transitive/cyclensichere Delta-Anforderungen, vollständiger Abgleich bei unvollständigem Inventar; keine Jobs oder Veröffentlichung.
- `src/model.rs`, `src/lib.rs`, `src/main.rs`: ausschließlich lokale Capture-/Reporttypen, striktes Duplicate-Key-Parsing, Budgets, reine Offline-CLI und explizit blockierter Produktionscheck.
- `tests/`, `fixtures/`, `check.sh`: synthetischer Pilot, Golden-/Grounding-/Unknown-/Policy-/Delta-/CLI-Negativtests und reproduzierbare Prüfung.
- `reports/`: vollständige Testausgabe/Quellhashes, Source-/Dependency-Manifest, `WIKI_COVERAGE.csv`, Pilotbericht, unveränderte Deltaausgabe und rot→grün belegter Regressionsfall.
- `README.md`, `SOURCE_CHECK.md`, `CR-S12-001.md`: Benutzung, tatsächlich geprüfte Quellen/Basis, klare offene Integration.

Keine Änderungen an gemeinsamem Root-Manifest/Lockfile, Contracts, SQL-Migrationen, Runtime, produktivem Wiki, globaler CI, STATUS/Gates oder anderen Owner-Bereichen. Neue Dependencies nur im Offline-Paket: serde/serde_json/sha2/chrono. Kein neuer Python-/JVM-/Lua-/LLM-/Inferenzpfad.

## Tatsächlicher Nachweis

Befehl: `bash architecture/migration/s12/check.sh`, Rust/Cargo 1.97.1 auf PATH.

| Prüfung | Ergebnis | Artefakt |
|---|---|---|
| Formatierung | bestanden | `s12/reports/checks.log` |
| Clippy alle Targets, Warnungen als Fehler | bestanden | ebenda |
| 64 Fixture-/Regressionstests + 9 CLI-Tests | **73 bestanden, 0 fehlgeschlagen, 0 ignoriert** | ebenda; `test-results.json` |
| Release-Build, locked/offline | bestanden | ebenda |
| Rebuildbare Golden-Vorschau und individuelle Locator-/Hashauflösung | bestanden | `tests/probe.rs`, Golden-Fixture |
| Unverändertes Delta | keine Reparse-/Reprojektionsanforderung, 0 Embeddingjobs | `unchanged-delta.json` |
| Produktionsbereitschaft auf gültigem Fixture | **erwartet blockiert, Exit 3** | `test-results.json` |
| Einzelner Wiki-Metadatenabruf | **HTTP 403**, kein verwendbarer Live-Capture | `SOURCE_CHECK.md` |

Code-Commit nach Commit erneut getestet. Nachfolgender Berichtscommit verändert keine geprüften Quellen/Tests/Fixtures. Absolute Arbeitsverzeichnisse wurden in gespeicherten Compilerlogs zu `<S12_WORKTREE>` normalisiert.

## Folgen und Grenzen

Kein produktiver Writer, keine DB-Verbindung, kein Scheduler, keine Modell-/Embeddingaufrufe, kein Netzwerkpfad, keine Mediendownloads, keine Migration oder Neustarts. Fremde Änderungen im Hauptcheckout und in bestehenden Worktrees wurden nicht verändert. Eigener S12-Worktree bleibt für den Review erhalten.

19 Seiten/20 Revisionen und die daraus erzeugten drei Vorschauen beschreiben ausschließlich den selbst erstellten Fixture-Korpus. 18 Seiten sind syntaktisch gelesen, ein Scribunto-Modul erwartbar quarantiniert. **Alle normalisierten/validierten/veröffentlichten Zähler bleiben 0.** Ein JSON-Parser oder ein Referenzlayout ersetzt weder einen typisierten Fachparser noch freigegebene Fakten oder einen legalen Buildplanner.

Capture-Policyfelder sind lokale, explizite Operatorangaben, keine authentifizierten Runtime-Credentials. Der spätere S02-Policyport muss Rechte aus verifizierter Identität beziehen. Rechteentzug wird in der Probe konservativ angewendet, ersetzt aber keinen echten ACL-/Release-/Storeintegrationstest.

Nicht ausgeführt: echte Wiki-Coverage/Rechtefreigabe, Store-/Worker-/Domain-/API-Pilot, vollständige Produktionstests, Lasttests, MSRV-Matrix, G2/G3/G4, Merge/Deploy/Cutover. Der bestehende GitHub-Rust-Compile-Job führt diese isolierte Suite nicht aus; CI-Ergänzung ist ein eigener CR an S02/S10. Keine unbestätigten GitHub-Checks als grün bewertet.

## Blocker und nächster Besitzer

S00/S01: G0 und freigegebenen Wiki-Zugang/Pflichtscope/Rechte klären. Der tatsächliche 403 darf nicht als „keine Seiten“ verschwinden.

S02/S03: G1 mit integrierten Source-/Fact-/Effect-/Rule-/HeroKnowledgeCard-/Policy-/Releaseverträgen und konkreten S12-Pfaden freigeben. Auf `main` stehen G0/G1 weiterhin offen; PR #25 ist noch offen und wurde nicht als integrierte Basis verwendet.

S03/S04/S05: echte Raw-/History-/Dependency-Stores, gemeinsame Jobs und validierte Fachinputs. Danach S12 im vorhandenen `wiki.rs`/`game_wiki.rs`/`wiki_refresh.rs` integrieren, mit PR #13 abgleichen und brauchbare Vorarbeit erhalten. Die lokalen Probe-Typen nicht in die Runtime übernehmen.

S05/S08/S12: echter freigegebener Hero-Pilot bis Karte/Build/API einschließlich Alias, Bedingungen, zwei Mechaniken und zwei Revisionen; danach Gesamtkorpus/G3 und unabhängige Qualität/G4 mit S10. Detaillierte Kriterien: `CR-S12-001.md`.

## Integration durch S00

Merge-Commit: keiner. Gate-/STATUS-Änderung: keine. Neue Arbeitswelle oder Produktivwechsel: nicht freigegeben. **S12 insgesamt nicht als done markieren.**
