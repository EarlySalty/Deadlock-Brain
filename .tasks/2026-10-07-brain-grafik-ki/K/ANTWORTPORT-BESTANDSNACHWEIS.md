# K: Tatsächlich verglichene Antwortport-Bestandsprüfungen

Nativer Prüfer a7fe6a42abeb6ebb1 abgeschlossen. Haupt-K hat die sechs Fixturediffs, Testmarker, Checkexits, Cleanupmarker und tatsächliches Worktreeregister selbst geprüft. Passender15-Dateien-Corecheckpoint a80b51a4 plus enger bestätigter Erfolgsfixturefix c64de6a2 gemeinsam regulär ALLOW und auf origin. Spätere unbewiesene Source-JSON-Naht nicht enthalten. Kein neuer Compiler-/Testlauf nach dem Erfolgsfixturefix, keine Gesamt- oder Liveabnahme.

## Gleiche Vorher-/Nachherauswahl

Unveränderte Produktbasis exakt 3c6f220b859936b9975ef54b76cd5a86c9ce08ab. Dieselbe Cargo-/Rust1.97.1-Auswahl, `--locked --offline --jobs 3`, vier Pakete brain-contracts, brain-providers, brain-kernel, brain-serve, `--no-fail-fast -- --include-ignored`. SQLX_OFFLINE=true und eigene Scratchvoraussetzungen, keine produktive Konfigänderung.

| Auswahl | passed | failed | ignored | filtered |
| --- | --- | --- | --- | --- |
| Unveränderte Basis ohne PG | 172 | 26 | 0 | 0 |
| Ursprünglicher K-WIP ohne PG | 178 | 26 | 0 | 0 |
| Unveränderte Basis mit eigenem PG | 173 | 25 | 0 | 0 |
| Ursprünglicher K-WIP mit demselben PG | 179 | 25 | 0 | 0 |
| K-WIP nach erlaubten Fixturekorrekturen mit PG | 195 | 9 | 0 | 0 |

Die sechs zusätzlichen WIP-Erfolge sind die neuen konkreten Enum-/HTTP-Fälle. Sämtliche ursprünglichen26 roten Testnamen sind im tatsächlichen unveränderten Vergleich reproduziert, nicht aus einem Fehlertext abgeleitet. Gleiche Namen beweisen keine identischen Ursachen: regulärer Gate des Sourcecheckpoints bestätigte später zusätzlich fehlendes finish_reason im bestehenden Discord-Erfolgsfixture, das der neue Parser verlangt. Dieser enge eigene Anschlussfehler wurde durch frischen Fixer in c64de6a2 ausschließlich als23-Byte-Fixtureergänzung korrigiert; neuer Compiler-/Testlauf offen, gemeinsamer regulärer Folgegate ALLOW. K/BRIEFING-ANTWORTPORT-R1.md und K/REVIEW.md. Finale breite Auswahl Exit101. PG plus gültige synthetische Fixtures beseitigen17 der ursprünglichen26 roten Fälle. Aussagen zur unveränderten Basis betreffen diese gemessenen Stände, nicht einen späteren origin/main.

Logs unter /tmp/brain-k-antwortport-bestand-20261007/: baseline.log, wip.log, baseline-pg-full.tests.log, wip-pg-full.tests.log, wip-after-pg-4.tests.log. Haupt-K hat die Summen aller 20 Resultatmarker je Vollauswahl selbst berechnet.

## Abgenommener Fixtureumfang

Genau sechs Dateien: brain-kernel/tests/core_kernel.rs, evidence_errors.rs, review_dependencies.rs; brain-kernel/src/flight/review_dependencies.rs im Testmodul; brain-serve/src/discord_live.rs im Testmodul; brain-serve/tests/process_e2e.rs. Platzhalterhashes durch echte SHA-256 der jeweiligen synthetischen Inhalte ersetzt, auch nach Inhaltsänderungen. Prozess-HTTP-Antwort um erforderliches finish_reason="stop" ergänzt. Zusätzliche Diagnose und strikte Dezimalgleichheitsassertion, keine gelöschte oder gelockerte Assertion. Kein Produktcode in diesen Änderungen, kein Budget-/Rechte-/Modell-/Timeoutwechsel. Vorhandene Kommentare entfernt, keine neuen Code-Kommentare.

Compiler `check -p brain-kernel -p brain-serve --all-targets`, striktes `clippy` derselben Auswahl mit `--no-deps -- -D warnings`, scoped fmt mit `--check`: Exit0,0,0. Tatsächliche Abschlussmarker von K gelesen: Compiler6,79s, Clippy20,93s. verification.results.log, check.log, clippy.log, fmt.log und verification.run.log. `git diff --check` durch K Exit0.

## Vier echte Produktgrenzen im finalen Lauf

1. Snapshotfall: 257 vollständige Reads statt1. SnapshotReadPort::read_manifest_until in brain-contracts/src/store.rs:169 delegiert standardmäßig an read_snapshot_until; MemoryRepository::read_snapshot in brain-storage/src/memory_repository.rs:206 rekonstruiert den kompletten Release. Gleicher korrigierter Fixturelauf gegen unveränderte Basis und WIP jeweils5 passed/1 failed/0 ignored/0 filtered, Exit101. baseline-shared-counter.log und wip-shared-counter.log. Die ursprüngliche Limitassertion bleibt erhalten, ein irreführender Counterumbau wurde verworfen. Keine Produktreparatur durch den Prüfer.
2. Source-JSON: 6.5 wird im vorhandenen eigenen Source-Parser als `$serde_json::private::Number`-Objekt statt Zahl geliefert. Haupt-K las strict_json.rs und den bestätigten G-Gitblob: die ausdrücklich bestätigte gemeinsame Parsernaht besitzt bereits die kompatible fünfzeilige Source-Delegation. Sie fehlte in der bisherigen neun-Dateien-Übernahme. Frischer Worker afc3c5b214bb2a520 beendet: ausschließlich bestätigten Blob übernommen, keine zweite Parserimplementierung. Haupt-K hat selbst Bytegleichheit und Quellenmanifest geprüft, 468 Dateien mit ausschließlich dieser Änderung. Danach Schutzablehnung des isolierten Prüfwegs vor Ausführung; Compiler, Format, Clippy und Tests nicht gestartet, keine Wiederholung oder Umgehung. Briefing K/BRIEFING-ANTWORTPORT-JSON-NAHT.md. Das 195/9-Resultat liegt vor dieser Anschlusskorrektur und beweist sie nicht.
3. Discordpacking: bei850 Panelbytes sechs Dokumente plus ein Live-Beleg, aber Anfänger-Lane fehlt; gemeldete Eingabeobergrenze8671. Bestehende Packpriorisierung in brain-serve/src/discord_live.rs:499 und dbrain-retrieval/src/release_port.rs:641. Budgetassertion und Laneassertion unverändert. Kein größerer Budgetrahmen oder anderer Prompt als Testreparatur.
4. Prozesscache: nach Rücknahme ausschließlich der Modellweitergabe UnauthorizedEvidence statt weiter veröffentlichbarer Cacheantwort. Fehlermeldung stammt aus brain-kernel/src/execution.rs:337 vor dem Provideraufruf. Providercounter vorher/nachher1813, kein neuer HTTP-Aufruf. Gleicher korrigierter Prozessfixture gegen unveränderte Basis und WIP jeweils0 passed/1 failed/0 ignored/0 filtered, Exit101; baseline-shared-fixture.tests.log und wip-shared-fixture.tests.log. Exakte Ursache des ausgebliebenen Cachetreffers bleibt offen. K übernimmt keine ungeprüfte Kerneländerung oder Ersatz-G-V-Bindung.

## Fünf reale Voraussetzungen, nicht ersetzt

Öffentlicher echter Discordresolvercase scheitert an ConfigMissing im absichtlich abgeschirmten Prüfkontext. Keine Aussage, dass die Produktionskonfiguration fehlt. Vier Pilotphasen benötigen einen ausdrücklich geeigneten BRAIN_PILOT_ROOT samt Dokumentbestand. Eigene Pilotdatenbank war vorhanden, kein freigegebener Corpus. Keine erfundenen oder privaten Originaldaten und kein tatsächlicher Kanal-/Modellaufruf zur Herstellung eines grünen Logs.

Der echte SCRAM-Prüffall läuft mit der vollständigen eigenen PG-Voraussetzung grün. Eigene PG-Cluster laut nachgelesenem Stopmarker beendet, postgres_stop_exit=0. Der eigene detached Baselineworktree ist im tatsächlichen Gitregister nicht mehr enthalten. Logs und eigene Buildartefakte bleiben erhalten.

Der äußere Brain-K-rust/target kann frühe gemischte Baselineartefakte enthalten. Nicht daraus blind einen finalen Buildbeweis ableiten, nichts pauschal löschen. Finaler Vollauswahllog zeigt tatsächliche Neucompilation der betroffenen K-Quellen, 55,15s. Neue Prüfungen binden Quellen und Artefakte erneut; die laufende JSON-Naht ist nicht durch das frühere Ergebnis abgenommen.

TESTNACHWEIS[TW-1]: 195 passed, 0 ignored | Baseline: 26 rot

Private Verarbeitung, tatsächlicher G-V-Port und gemeinsame Release-/Liveabnahme bleiben offen. Keine Mainintegration, Migration, Veröffentlichung, Deployment, Neustart, Liveprobe oder Cleanup eigener Featureworktrees aus diesem Bestandsnachweis.
