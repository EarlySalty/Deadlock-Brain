# G: Ausgangsprüfung

status: erledigt für Baseline, 07.10.2026. Unveränderter Ausgangscode rot, keine Produktkorrektur durch den Prüfer.

## Gemessener Stand

Start `bfda408c`, Ende `96e6a8da`. Der zwischenzeitliche Commit enthält Aufgabenunterlagen. Der Prüfer verglich 419 Rust-/Cargo-Dateien ohne Produktabweichung. Eigene Targets, Testhome und drei private PostgreSQL-Cluster; deren Abschluss jeweils Exit 0. Keine Produktionsänderung, Git-Aktion, Gateprüfung oder Veröffentlichung.

| Paket | passed | failed | ignored | eindeutig gefiltert |
| --- | ---: | ---: | ---: | ---: |
| `dbrain-reasoner` | 287 | 4 | 0 | 3 |
| `dbrain-retrieval` | 92 | 15 | 0 | 12 |
| `brain-serve` | 61 | 1 | 0 | 5 |
| `brain-kernel` | 34 | 18 | 0 | 0 |

474 eindeutige bestandene und 38 eindeutige fehlgeschlagene Fälle. Rohzählung über Wiederholungen: 39 fehlgeschlagene und 168 gefilterte Fälle. Das ist kein zweiter unabhängiger Baselinevergleich. Vier leere Doc-Suiten zählen nicht als Testbeweis.

`cargo fmt --check` und `cargo check --all-targets`: Exit 0. Striktes Clippy: Reasoner, Serve und Kernel Exit 0; Retrieval Exit 101, `items_after_test_module` in `src/release_port.rs:510`.

## Ursachen und Grenzen

- Reasoner-DB-Fixture: Spalte `hero_build_id` fehlt.
- Retrieval: ungültige Release-Lesemanifeste.
- Kernel: Assertions und Timeouts; fallgenaue Ergebnisse im Rohbeleg, kein pauschales Produktfehlerurteil.
- Serve-E2E: `release_unavailable` statt `knowledge_version_mismatch`; ein Setupfehlversuch wurde getrennt dokumentiert und mit vollständiger DB-Vorbereitung wiederholt.
- 20 Fälle ausdrücklich ausgeschlossen: 15 DB-Snapshotfälle, vier Pilotfälle, ein Liveabruf. Diese Grenze bleibt offen; keine vollständige Suiteabdeckung behauptet.

Befehle verwendeten `--locked --offline --jobs 3` sowie bei Tests `--include-ignored --test-threads=1`. Ausstehende Integrationssuites wurden separat nachgeholt. Vollständige Befehle, Umfeld und unveränderte Exitcodes stehen in `pruefungen/baseline/baseline-results.json`; Fallzählung in `test-cases.json`. Beide Rohbelege liegen im ursprünglich zugewiesenen Bereich `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-06-brain-abschluss/G/`.

## Herkunft

Workflow `brain-g-baseline`, Task `wih3iu8uu`, Run `wf_ccd9af54-f71`, Fertignachricht bestätigt. Ein Worker, 58 Werkzeugaufrufe; gemeldete Dauer rund 40,7 Minuten. Der 20-Minuten-Timer wurde überschritten. Der Worker gab den Bericht zurück, statt `G/BASELINE.md` zu schreiben, weil sein eigener Arbeitsmodus Markdown-Berichte ausschloss. Bereichsführung hat das Ergebnis in die eigene Worktree-Akte übernommen.

TESTNACHWEIS[TW-1]: 474 passed, 0 ignored | Baseline: 38 rot
