# Fertigmeldung Paket C

Branch: `feat/build-reasoner-c`

Commit: `1e23609a6179c22d4fc6366760e5fe67743cbc0e` (`feat(reasoner): compose builds and patch evidence`), auf `origin/feat/build-reasoner-c` gepusht. Basis war Paket A `3bef602`.

## Geändert

- `rust/crates/dbrain-reasoner/src/patch.rs`: Patch-Delta aus `patch_events` und Enrichment, echte Differenzgrößen, Vorzeichen, Zielauflösung und Anwendung auf Helden-, Ability- und Item-Werte.
- `rust/crates/dbrain-reasoner/src/meta.rs`: gewichteter Meta-Index, Mindeststichproben-Dämpfung, Autoren- und Claim-Treffer sowie konfigurierbarer Seed-Loader für `referenz/*.json`.
- `rust/crates/dbrain-reasoner/src/composer.rs`: Core und Situationsblöcke in Referenzreihenfolge, Belege, Konfidenz, Imbue-Ziel und Sperrliste für Recompose.
- `rust/crates/dbrain-reasoner/src/backtest.rs`: Core-Überdeckung, Jaccard-Hilfsmetrik, Rangnähe, Patch-Wechsel-Erkennung und Autorenaggregation.
- `rust/crates/dbrain-reasoner/src/publish.rs`: BuildObject zu `BuildSpecPayload` sowie Enqueue für `BUILD_PUBLISH_ORIGINAL`.
- `rust/crates/dbrain-builds/src/spec.rs`: optionale Kategorie-Layoutfelder sowie `imbue` und `sell_priority` je Mod.

`lib.rs` wurde nur temporär für den Testlauf um Moduldeklarationen ergänzt und ist nicht Teil des Commits.

## Tests

Baseline aus `FERTIG-A.md`: Reasoner 7 bestanden, 0 fehlgeschlagen, 1 ignoriert.

Endstand mit temporärer Moduldeklaration: Reasoner 15 bestanden, 0 fehlgeschlagen, 1 ignoriert; dbrain-builds 7 bestanden, 0 fehlgeschlagen, 5 ignoriert. Die fünf beziehungsweise ein ignorierter Test benötigen Scratch-Postgres.

Finaler Lauf nach Rücknahme der temporären `lib.rs`-Änderung: Reasoner 7 bestanden, 0 fehlgeschlagen, 1 ignoriert; dbrain-builds 7 bestanden, 0 fehlgeschlagen, 5 ignoriert. `cargo clippy -p dbrain-reasoner -p dbrain-builds -- -D warnings` war mit den C-Modulen eingebunden sauber.

## Warden-Seed und Spec-Lücken

Der Seed enthält 19 Core-Items. Ein echter Reasoner-vs-Seed-Backtest ist in diesem Branch noch nicht ehrlich ausführbar, weil die B-ScoredItems und die D-Fassade beziehungsweise CLI noch fehlen. Der Warden-Test prüft die Metrikformel mit erwarteten Werten `core_coverage = 0,6667`, Jaccard `= 0,5`, Reihenfolge-Nähe `= 0,0`; die strukturelle Seed-Selbstabbildung wäre `1,0 / 1,0 / 0,0`, ist aber kein Erfolgswert für einen Reasoner-Vergleich. Patch-Wechsel bleibt bis zur zweistufigen Build-Erzeugung `None`.

Entscheidungen:

- Patch-Events ohne numerische Item-ID werden nicht auf eine geratene ID abgebildet. D muss bei Bedarf die Enrichment-Ladung um eine Katalogauflösung ergänzen.
- Da `BacktestMetrics` aus Paket A kein Jaccard-Feld enthält, ist Jaccard als `core_jaccard` separat verfügbar; `core_coverage` bleibt die vorgeschriebene Schnittmengenquote.
- Der Seed-Loader erhält Pfad und Item-Modelle explizit. So bleiben `referenz/*.json` lesend und der Pfad konfigurierbar.
- Publish schreibt nur `steam.steam_tasks` mit Typ `BUILD_PUBLISH_ORIGINAL`; der Steam-Bot bleibt unverändert.

## Offen für D

- C-Module in `lib.rs` öffentlich exportieren und mit Paket B sowie `reason_build`, `reason_patch_impact` und `reason_backtest` verdrahten.
- Reasoner-ScoredItems gegen echte Warden-Snapshots erzeugen und den Backtest mit dem Seed sowie vorhandenen Autoren-Builds ausführen.
- Seed-Pfad in CLI beziehungsweise Config anschließen, Warden-Patchstände zeitlich zuordnen und die echten Backtest-Zahlen in den Report übernehmen.
- CLI `reason` und den Publish-Aufruf integrieren; anschließend den End-to-End-Lauf mit Scratch-Postgres prüfen.
