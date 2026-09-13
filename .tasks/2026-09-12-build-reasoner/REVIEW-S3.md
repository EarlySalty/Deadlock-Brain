# REVIEW-S3: Autoren-Scan je Held (Runde 1)

**Urteil: NACHBESSERN.**

Diff `ad5f00e..0cd59bf`, Branch `fix/autoren-scan-je-held`. Ein blockierender
Mangel, ein wichtiger, drei Nits. Die Grundidee (GC-Anfrage je Held ohne
Autorenfilter, clientseitige Zuordnung, Heldenblöcke) ist sauber umgesetzt und
belegt; der blockierende Punkt betrifft das Fehlerverhalten unter GC-Timeouts.

## Mängelliste

### 1. Blockierend: ein GC-Fehler bei einem Helden kippt alle Autoren
`discovery.rs:245-271` (Fehler- und Response-Code-Zweig) plus `discovery.rs:358`
(`error_count = author_scan.errors + u32::from(scan_failed)`).

Trifft ein einzelner GC-Timeout oder Response-Code != 1 bei irgendeinem Helden,
wird `scan_failed = true` gesetzt und der Heldenloop mit `break` abgebrochen. Zwei
Folgen:

- Alle restlichen Helden des Blocks werden nicht mehr gescannt; ihre Builds
  fehlen diesen Zyklus komplett.
- In der Statusschleife bekommt dadurch **jeder** beobachtete Autor
  `error_count > 0`, also `error` (ohne persistierte Builds) oder `partial` (mit).
  Kein Autor bleibt `ok`.

Das verletzt die Abnahmebedingung aus dem Briefing Punkt 2 wörtlich: "ein
GC-Fehler bei einem Helden darf nicht alle Autoren auf `error` setzen". Es ist
zugleich ein Robustheitsrückschritt gegenüber dem alten Je-Autor-Scan, wo ein
GC-Aussetzer nur einen Autor traf. Bei `CATALOG_GC_TIMEOUT = 25 s` reicht ein
einzelner langsamer Request (transient, kein toter GC), um den ganzen Zyklus
fehlschlagen zu lassen (`into_task_result` liefert `Err`, wenn kein Autor Builds
hatte) und die Dashboard-Status aller Streamer kurzzeitig auf `error` zu
schalten. Widerspricht dem Grundsatz "transient != fatal".

**Fix-Vorschlag:** Bei einem einzelnen Helden-Fehler den Fehler erfassen und mit
dem nächsten Helden weitermachen, nicht `break`. Erst nach einer begründeten
Schwelle aufeinanderfolgender Fehlschläge (Indiz für abgerissene GC-Verbindung)
den Block abbrechen. `scan_failed` nicht pauschal in jeden Autor-`error_count`
mischen; Autoren mit erfolgreich persistierten Builds nicht auf `error`/`partial`
zwingen. Der umbenannte Test `discovery_gc_errors_are_recorded_for_all_authors_in_a_block`
(`catalog.rs:1499`) zementiert genau dieses Fehlverhalten und muss mit dem Fix
auf die Briefing-Erwartung korrigiert werden (ein GC-Fehler bei einem Helden lässt
nicht betroffene Autoren `ok`).

### 2. Wichtig: Block 2 überschreibt Autorenstatus aus Block 1
`discovery.rs:354-400` (Statusschleife je Block) plus Planung `catalog.rs:96-116`
und `discovery.rs:73-88`.

Jeder `DISCOVER_WATCHED_BUILDS`-Task verarbeitet einen 100er-Heldenblock, leitet
den Autorenstatus aber aus **nur diesem Block** ab und schreibt ihn für **alle 13**
beobachteten Autoren. Bei mehr als 100 Helden (mehrere Blöcke) überschreibt der
zweite Task den `ok`-Status eines Autors, dessen Builds nur in Block 1 lagen, mit
`partial` "keine Builds im Katalog". Genau der im Briefing Punkt 2 genannte Fall.

Aktuell live nicht auslösbar (rund 38 Helden, also 1 Block), daher wichtig statt
blockierend. Es ist aber ein latenter Korrektheitsfehler, der genau dann kippt,
wenn der Blockmechanismus (der Sinn von S3) tatsächlich mehrere Blöcke erzeugt.

**Fix-Vorschlag:** Autorenstatus nicht je Block schreiben. Entweder alle Blöcke
in einem Task scannen (Budget prüfen), oder Status nur additiv/mergend pro Autor
über den Zyklus führen (Treffer je Block akkumulieren, Status erst am Zyklusende
ableiten), oder `partial` "keine Builds" nur setzen, wenn der Autor im ganzen
Zyklus keinen Treffer hatte, nicht je Block.

### 3. Nit: `error_count`-Zahl passt nicht zu angehängten Meldungen
`discovery.rs:374-389`. Die Nachricht sagt "{error_count} errors", hängt danach
aber sowohl `scan_error_messages` (blockweit, ggf. mehrere) als auch
`author_scan.error_messages` an. Zahl und Anzahl angehängter Zeilen können
auseinanderlaufen. Kosmetisch.

### 4. Nit: `response_codes` im Blocklog kann bis zu 100 Einträge haben
`discovery.rs:414`. "Helden-Scan beendet" loggt `response_codes = ?response_codes`
mit einem Eintrag je Held. Bei vollem 100er-Block eine lange Zeile. Erwägen, nur
abweichende Codes oder eine Zählung zu loggen.

### 5. Nit: Planung nutzt alle Helden, nicht nur aktive
`discovery.rs:73-88` plant über `all_hero_names()` (alle je gesehenen Helden),
`id > 0` gefiltert und dedupliziert. FERTIG spricht von "aktiven Helden". Kein
Korrektheitsfehler, nur Wording und ggf. unnötige Anfragen für inaktive Helden.

## Geprüft und in Ordnung

- **Proto/Anfrageform:** Request setzt nur `hero_id` und `language: [0,0]`,
  `author_account_id` bleibt leer (`discovery.rs:234-238`); Proto bestätigt beide
  Felder optional. Alte Payloads mit `author_account_id` bleiben verarbeitbar
  (`discovery.rs:49-70`), werden aber nicht mehr geplant.
- **Dedup und Persistenz:** `seen_builds` über `(hero_build_id, language)` auf
  Blockebene (`discovery.rs:220,284`); jede Quelle via `upsert_hero_build_source`,
  keine Löschung.
- **Statusableitung Normalfall:** Block ohne jeden Treffer meldet `partial`
  "keine Builds im Katalog", nicht `ok` (`discovery.rs:366-372`). Korrekt.
- **Budget/Reserve:** Blockdeadline `timeout_at` = 420 s (`discovery.rs:218`)
  deckelt die Wandzeit hart unter dem 480-s-Taskbudget, unabhängig von
  `CATALOG_GC_TIMEOUT = 25 s`. 100 Helden mal rund 3,6 s liegen bei rund 360 s,
  Reserve rund 60 s. Kein Stale-Reap-Risiko im Normalfall. Hinweis: bei langsamem
  GC scannt ein Block vor 420 s nur einen Bruchteil der Helden und schlägt dann
  fehl, was Mangel 1 verstärkt.
- **Journal:** Eine Zeile "Helden-Scan beendet" je Block mit Helden, Builds,
  Autoren-Treffern (`discovery.rs:409-416`); keine Zeile je Held.
- **Task-Zahl:** `ceil(Helden/100)` statt 13 je Autor; live 1 Task
  (`catalog.rs`-Test `discoveryUnits == 1`).

## Tests

TESTNACHWEIS[TW-1]: 191 passed, 0 ignored | Baseline: 0 rot (offline)

- Befehl: `cargo test -p steam-core --features testing` im Worktree, ohne
  `CENTRAL_TEST_DSN`. Ergebnis: 271 Tests, 191 passed, 80 failed, Exit 101.
  Alle 80 Fehler sind `CENTRAL_TEST_DSN muss gesetzt sein` (DB-Tests ohne
  Test-DB), keine Logikfehler. Offlinepfad also grün.
- DB-Katalogtests **nicht selbst gelaufen:** kein Docker/DSN erreichbar
  (`central_test_db.sh` liegt in Deadlock-Bots, kein Container aktiv). Die neuen
  S3-Tests (`discovery_maps_results_to_watched_authors_without_author_filter`,
  `discovery_tasks_are_bounded_hero_blocks` und die geänderten) sind DB-Tests und
  wurden daher von mir nicht ausgeführt. Der Endstand "36 zentrale DB-Tests" ist
  **Fremdnachweis des Fixers** aus FERTIG-S3, von mir nicht verifiziert.
- Die im Briefing genannte Offline-Baseline (174) und mein Zählwert (191)
  differieren durch unterschiedlichen Zählumfang (`-p steam-core` erfasst hier
  alle Bin-Tests); keine roten Offlinetests.
- Clippy nicht neu gefahren (bekannter S2-Baseline-Hänger `gc_health.rs:45`, kein
  S3-Mangel laut Briefing).

## Fazit

NACHBESSERN wegen Mangel 1 (blockierend, Abnahmebedingung verletzt und
Produktionsrobustheit). Mangel 2 im selben Zug beheben, sonst kippt der
Blockmechanismus, sobald er mehr als einen Block erzeugt. Nits nach Ermessen.

## Fixrunde 1

Abgeschlossen am 13.09.2026, alle fünf Mängel bearbeitet. Code-Commit
`ffc34faa491d8e1f14ef54d33b5fe8f94f67f3f4` auf
`fix/autoren-scan-je-held`, aufbauend auf `0cd59bf`, nach `origin`
gepusht. Keine Unter-Threads. Das ursprüngliche Review-Urteil bleibt als
Historie erhalten; die erneute unabhängige Abnahme liegt beim Orchestrator.

Die Rust-Pfade unten sind relativ zu
`rust/crates/steam-core/src/task/handlers/builds/` im beauftragten Worktree.

| Mangel | Datei:Zeile | Änderung und Nachweis |
| --- | --- | --- |
| 1, blockierend | `discovery.rs:20`, `discovery.rs:337`, `discovery.rs:446` | Einzelne Timeouts und Response-Codes ungleich 1 werden erfasst, danach folgt der nächste Held. `MAX_CONSECUTIVE_HERO_FAILURES = 3` begrenzt aufeinanderfolgende Fehler; Erfolg setzt den Zähler zurück. Beim Abbruch erhalten alle Autoren `partial` mit exakt „Scan abgebrochen nach 3 Helden-Fehlern“, der Task schlägt sichtbar fehl. Scanfehler fließen nicht in Autorenfehler ein. Tests: `catalog.rs:1533`, `catalog.rs:1574`, `catalog.rs:1610`. Commit `ffc34fa`. |
| 2, wichtig | `discovery.rs:77`, `discovery.rs:484`, `catalog.rs:105` | Ein Task für alle Helden, solange Heldenzahl × 3,6 s + 60 s unter 480 s liegt; sonst höchstens 100 Helden je Block. Alle Payloads tragen denselben DB-Zeitpunkt `cycle_started_at`, der letzte Block `is_last_block: true`. Frühere Blöcke schreiben nur Autoren mit Treffern beziehungsweise eigenen Persistenzfehlern. Der letzte Block setzt fehlende Treffer atomar nur bei `last_checked_at IS NULL` oder vor Zyklusstart auf `partial`. Bereits offene Discovery-Blöcke desselben Botkontos verhindern doppelte Planung trotz unterschiedlicher Zeitstempel. Tests: `catalog.rs:1249`, `catalog.rs:1276`, `catalog.rs:1637` sowie vorhandener Zyklus-Deduplizierungstest. Commit `ffc34fa`. |
| 3, Nit | `discovery.rs:446`, `discovery.rs:477` | `error_count` wird unmittelbar aus der Anzahl eigener Fehlermeldungen abgeleitet; nur diese Meldungen werden angehängt. Blockfehler bleiben im Task-Ergebnis. Der Test für isolierte GC-Fehler prüft ausdrücklich „0 errors“ ohne angehängte GC-Meldung; der bestehende Persistenzfehlertest bleibt grün. Commit `ffc34fa`. |
| 4, Nit | `discovery.rs:300`, `discovery.rs:337`, `discovery.rs:529` | `response_codes` ist eine sortierte Zählung je Response-Code statt einer Liste je Held. Commit `ffc34fa`. |
| 5, Nit | `FERTIG-S3.md:42` | Wording auf „alle bekannten Helden“ korrigiert. Diese Dokumentationskorrektur und der vorliegende Anhang liegen im Brain-Taskordner, außerhalb des Steam-Code-Commits; kein Push auf Brain/main. |

Bei 38 bekannten Helden bleibt es bei genau einem Discovery-Task
(rechnerisch 196,8 s einschließlich Reserve). Auch 101 und 116 Helden
passen jeweils in einen Task; 117 Helden ergeben zwei Blöcke mit 100 und
17 Helden. Das bestehende Ausführungsbudget von 420 s zuzüglich Reserve
bleibt erhalten. Leerer Heldenkatalog und Budgetablauf bleiben sichtbare
Abbrüche mit `partial` statt pauschalen Autorenfehlern. Alte Payloads ohne
Zyklusfelder bleiben verarbeitbar.

### Tests, Rot-Gegenprobe und Selbstprüfung

| Prüfung | Baseline `0cd59bf` | Endstand `ffc34fa` |
| --- | --- | --- |
| Offline-Tests `steam-core --features testing` | 191 bestanden, 0 logische Fehler | 191 bestanden, 0 fehlgeschlagen |
| Katalogtests mit zentraler Test-DB | 36 bestanden, 0 fehlgeschlagen | 40 bestanden, 0 fehlgeschlagen |
| Rot-Gegenprobe mit den neuen Erwartungen vor dem Codefix | 30 bestanden, 10 fehlgeschlagen | Alle 10 zuvor roten Fälle bestanden |

Alle vier neuen Tests waren in der Rot-Gegenprobe einzeln rot:
`discovery_aborts_after_three_consecutive_hero_failures`,
`discovery_success_resets_consecutive_hero_failures`,
`discovery_keeps_all_heroes_in_one_task_when_they_fit_the_budget` und
`discovery_last_block_preserves_earlier_hits_and_finalizes_only_unchecked_authors`.
Der Zweiblocktest prüft außerdem, dass ein früherer Treffer unverändert
`ok` bleibt und ein Status mit Zeitstempel genau am Zyklusstart erhalten
bleibt.

Der ungefilterte Lauf ohne `CENTRAL_TEST_DSN` lieferte in der Baseline
191 bestandene und 80 DB-Fehler, im Endstand 191 bestandene und 84 DB-Fehler.
Sämtliche DB-Fehler waren „CENTRAL_TEST_DSN muss gesetzt sein“. Der
anschließende Offline-Lauf mit genau diesen 84 DB-Tests per `--skip`
ausgenommen war grün: 191 bestanden, 0 fehlgeschlagen, 84 gefiltert.

Die DB-Läufe nutzten ausschließlich den wegwerfbaren Testcontainer mit
Migrationen über `Deadlock-Bots/rust/scripts/central_test_db.sh` und den
vorhandenen Docker-Shim. Befehl:

```bash
export PATH=/tmp/steam-scrim-dockershim:/home/nathanael/.cargo/bin:$PATH
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-je-held/rust/Cargo.toml -p steam-core --features testing task::handlers::builds::catalog::tests
```

`cargo check -p steam-core --features testing`, `rustfmt --check` nur auf
den beiden geänderten Dateien und `git diff --check` sind grün.
`cargo clippy -p steam-core --all-targets --features testing -- -D warnings`
meldet ausschließlich den bekannten Baseline-Hänger
`gc_health.rs:45` (`RateLimitOutcome::cooldown`, `dead_code`), Exit 101.
Keine neue Clippy-Diagnose.

Prüflogs: `/tmp/steam-s3-fix1-baseline-db.log`,
`/tmp/steam-s3-fix1-red-db.log`, `/tmp/steam-s3-fix1-final-db.log`,
`/tmp/steam-s3-fix1-baseline-offline.log`,
`/tmp/steam-s3-fix1-final-offline.log`,
`/tmp/steam-s3-fix1-offline-filtered.log`,
`/tmp/steam-s3-fix1-check.log` und
`/tmp/steam-s3-fix1-final-clippy.log`.

Selbstprüfung abgeschlossen: Codeänderungen ausschließlich in
`discovery.rs` und `catalog.rs`, keine neuen Code-Kommentare, keine
Änderungen an Lanes, Runner oder Proto und keine Schreibzugriffe auf
Produktivdaten. Kein Merge oder Deployment; der Live-Nachweis bleibt wie
beauftragt beim Orchestrator.

## Review Runde 2

**Urteil: FREIGABE.** Diff der Fixrunde `0cd59bf..ffc34fa`, gelesen, kein
Code. Alle fünf Mängel behoben, keine neuen Befunde aus dem Fix.

| Mangel | Behoben | Nachweis |
| --- | --- | --- |
| 1 blockierend | ja | Einzelner Helden-Fehler zählt nur weiter (`discovery.rs:353-360`, `continue` statt `break`), Abbruch erst nach `MAX_CONSECUTIVE_HERO_FAILURES = 3` (`discovery.rs:20,354-358`), Erfolg setzt den Zähler zurück (`discovery.rs:363`). Scan-Fehler fließen nicht mehr in den Autorenstatus: Feld `errors` aus `AuthorScanStats` entfernt, `error_count = author_scan.error_messages.len()` speist sich allein aus eigenen Upsert-Fehlern (`discovery.rs:446`). Nach Abbruch bekommen alle Autoren `partial` mit exakt „Scan abgebrochen nach 3 Helden-Fehlern" (`discovery.rs:481-483`), und der Task schlägt sichtbar fehl (`scan_aborted` → `into_task_result` `Err`, `discovery.rs:146,151-154`). Test umgeschrieben auf die Briefing-Erwartung (`catalog.rs:1533`, nicht betroffener Autor bleibt `ok`), plus Abbruchtest (`catalog.rs:1574`) und Reset-Test (`catalog.rs:1610`). |
| 2 wichtig | ja | Blockgröße budgetabhängig: alle Helden in einem Block, wenn `ESTIMATED_HERO_SCAN_TIME * count + DISCOVERY_RESERVE < CATALOG_TASK_TIMEOUT` (480 s), sonst 100 (`discovery.rs:104-113`). 38 Helden ergeben einen Block, 117 ergeben 100+17 (Test `catalog.rs:1276`). Frühere Blöcke schreiben Autoren ohne Treffer nicht (`discovery.rs:448-452`), der letzte Block finalisiert nur mit Zeitstempel-Guard `last_checked_at < cycle_started_at` (`discovery.rs:488-491`). Der `ok`-Schreiber aus Block 1 setzt `last_checked_at = now()` (verifiziert `steam-persistence/src/builds.rs:865`), liegt also nach `cycle_started_at`; der Guard des letzten Blocks lässt ihn unangetastet. Doppelplanung eines Zyklus verhindert der Pending-Check (`discovery.rs:80-90`). Zweiblocktest `catalog.rs:1637` belegt: Autor mit Treffer nur in Block 1 bleibt `ok`, ein exakt am Zyklusstart geschriebener Fremdstatus (`error`) bleibt ebenfalls erhalten. |
| 3 Nit | ja | `error_count` und angehängte Meldungen deckungsgleich, nur eigene Fehlermeldungen werden angehängt (`discovery.rs:446,477-480`); blockweite Meldungen entfernt. Test prüft „0 errors" ohne GC-Meldung (`catalog.rs:1533`). |
| 4 Nit | ja | `response_codes` ist `BTreeMap<i32,u32>`, sortierte Zählung je Code statt Liste je Held (`discovery.rs:300-302,528`). |
| 5 Nit | ja | Wording in `FERTIG-S3.md:42` auf „alle bekannten Helden" geändert. |

### Nebenwirkungen und Beobachtungen

- Ein abgebrochener Block überschreibt alle Autoren auf `partial` ohne
  Zeitstempel-Guard (`discovery.rs:501-504`), auch solche, die in einem
  früheren Block desselben Zyklus schon `ok` waren. Das entspricht der
  Delegator-Entscheidung 1 („nur ein abgebrochener Block markiert alle
  Autoren mit partial") und ist live nicht auslösbar (38 Helden, ein Block).
  Kein Mangel, nur festgehalten.
- Ein Autor mit gefundenem Build aber fehlgeschlagenem Upsert hat
  `builds > 0` und wird daher auch in einem Nicht-Endblock als `error`
  geschrieben. Deckt die Entscheidung „bzw. eigenen Persistenzfehlern" ab.

### Tests

TESTNACHWEIS[TW-1]: 191 passed, 0 ignored | Baseline: 0 rot (offline)

- Befehl: `cargo test -p steam-core --features testing` im Worktree, ohne
  `CENTRAL_TEST_DSN`. Ergebnis: 191 passed, 84 failed, 0 ignored, Exit 101.
  Alle 84 Fehler sind „CENTRAL_TEST_DSN muss gesetzt sein" (DB-Tests ohne
  Test-DB), 84 Panics, 84 DSN-Meldungen, keine Logikfehler. Offlinepfad grün,
  deckt sich mit dem Fixer-Wert 191.
- DB-Katalogtests nicht selbst gefahren (keine Test-DB erreichbar). Der
  Endstand 40 zentrale DB-Tests ist **Fremdnachweis des Fixers**, von mir
  nicht verifiziert. Die vier neuen S3-Tests
  (`discovery_keeps_all_heroes_in_one_task_when_they_fit_the_budget`,
  `discovery_aborts_after_three_consecutive_hero_failures`,
  `discovery_success_resets_consecutive_hero_failures`,
  `discovery_last_block_preserves_earlier_hits_and_finalizes_only_unchecked_authors`)
  sowie die umgeschriebenen Bestandstests prüfen die Briefing-Erwartung,
  nicht das alte Verhalten; per Codelesung bestätigt.

### Fazit

FREIGABE. Beide Kernmängel korrekt behoben und durch DB-Tests abgedeckt, die
das neue Verhalten prüfen; Nits erledigt. Offlinepfad grün, DB-Zahl als
Fremdnachweis gekennzeichnet.

## Fixrunde 2 (Merge-Kritiker)

Bearbeitet am 13.09.2026 gemäß `FIX2-BRIEFING-S3.md`, ohne Unter-Threads.
Code-Commit `362284e198a67870b717731e1c84f217ac56f672` auf
`fix/autoren-scan-je-held`, direkt aufbauend auf `ffc34fa`.
Die unabhängige Abnahme bleibt beim Orchestrator.

Die Kurzpfade `discovery.rs` und `catalog.rs` beziehen sich auf
`rust/crates/steam-core/src/task/handlers/builds/` im beauftragten Worktree;
`builds.rs` auf `rust/crates/steam-persistence/src/`.
Alle Codeänderungen der folgenden Tabelle liegen in Commit `362284e`.

| Fund | Datei:Zeile | Änderung und Nachweis |
| --- | --- | --- |
| 1, BLOCKING laut Kritiker | `discovery.rs:156`, `discovery.rs:388`, `discovery.rs:582`; Test `catalog.rs:1617` | Die verbindliche Entscheidung des Delegators wird umgesetzt: Alle Builds bleiben Katalogmaterial. `totalNewBuilds`, `totalUpdatedBuilds` und `heroesFound` zählen weiterhin den gesamten Katalog; neu zählt `watchedBuilds` ausschließlich Treffer der im Task betrachteten Autoren. Das Log „Helden-Scan beendet“ nennt `catalog_builds`, `watched_builds`, `catalog_heroes`, `catalog_new_builds` und `catalog_updated_builds`. Der neue Test liefert einen beobachteten und einen fremden Autor auf unterschiedlichen Helden: beide werden gespeichert und beim zweiten Lauf aktualisiert, Katalogzähler jeweils 2, `watchedBuilds` 1, genau ein Autorenstatus mit genau einem Build und einem Helden. Es entsteht kein Status für den fremden Autor. |
| 2, NIT | Unveränderter Request `discovery.rs:320`; vorhandener Live-Lesepfad `rust/crates/steam-core/src/task/handlers/gc.rs:103` | Live-Task 4053192, Warden ohne Autorenfilter: Response 1, `results.len() = 40`. Exakter Live-Lookup 4053206 für Held 25, Build 779996: Response 1, genau 1 Treffer. Lesende Katalogabfrage: 42 Warden-Builds, alle Sprache 0; Build 779996 gehört Autor 1650097169, Version 45. Aussagegrenze und Kappungsverdacht unten. Keine Codeänderung für diesen Fund. |
| 3, NIT | `discovery.rs:492`, `discovery.rs:528`, `discovery.rs:541`; Tests `catalog.rs:1578`, `catalog.rs:1693` | Ein Abbruch überschreibt weder Zähler noch `ok` eines Autors mit erfolgreich persistierten Treffern im Block. Eigene Persistenzfehler behalten ihre vorhandene Einstufung. Autoren ohne Treffer erhalten `partial` und den Abbruchtext. Der atomare Schutz über `cycle_started_at` gilt nun auch beim Abbruch und erhält frühere Zyklustreffer. Angepasster Dreifehlertest sowie neuer Dreiblocktest mit Treffern in Block 1 und 2, Abbruch in Block 3. |
| 4, NIT | `discovery.rs:460`, `discovery.rs:483`, `discovery.rs:504`; Test `catalog.rs:1653` | Fehlgeschlagene Helden werden gesammelt und mit den bekannten Builds der beobachteten Autoren abgeglichen, in einer gemeinsamen zusätzlichen Leseabfrage nur bei Heldenfehlern. Ohne Treffer lautet der Hinweis beispielsweise „GC-Fehler bei Held 1, 2“. Ein Autor mit Treffern bleibt `ok`, ein unbeteiligter Autor behält den Hinweis auf fehlende Katalogtreffer. Ein früher Block mit bekanntem Heldenfehler schreibt den Hinweis bereits; spätere leere Blöcke löschen ihn nicht. |
| 5, NIT | `discovery.rs:33`, `discovery.rs:125`, `discovery.rs:538`; Tests `catalog.rs:1249`, `catalog.rs:1693` | Geplante Payloads tragen `block_number` und `block_count`. Statusmeldungen beginnen mit „Block 2 von 3: ...“. Der Dreiblocktest prüft einen weiteren Treffer im zweiten Block und den Erhalt dieser ausdrücklich auf Block 2 bezogenen Zähler. Alte Payloads ohne Blockfelder bleiben kompatibel. Keine Zyklusaddition, Begründung unten. |
| 6, NIT | `discovery.rs:24`, `discovery.rs:304`, `discovery.rs:331` | Deadline und Meldung verwenden dieselbe abgeleitete Konstante `DISCOVERY_SCAN_BUDGET = CATALOG_TASK_TIMEOUT - DISCOVERY_RESERVE`, derzeit 420 s. Die Meldung formatiert deren Sekundenwert. |
| 7, NIT | `builds.rs:946`, `discovery.rs:84`; Test `catalog.rs:1743` | Neue Persistenzfunktion `count_open_discovery_tasks` neben `has_open_catalog_maintenance_task`; das SQL ist aus dem Planungs-Guard entfernt. Der Guard loggt „Discovery-Planung übersprungen, N Tasks offen“. Der neue Test prüft beide Tasktypen, PENDING/RUNNING, abgeschlossene Tasks, andere Bots, fremde Tasktypen, leere und nullbelegte Payloads sowie die tatsächliche Planungssperre. |

### Gegenprobe zu Fund 2 und Aussagegrenze

Rohbeleg: `fix2-s3-live.json` neben diesem Bericht. Es wurden genau zwei
lesende `GC_SEARCH_BUILDS`-Tasks über die bestehende API angelegt; kein
Discovery- oder Maintenance-Handler mit Katalogschreibzugriff wurde live
angestoßen. Die ergänzenden SELECTs liefen mit
`default_transaction_read_only=on`.

Die aktuelle ungefilterte Suche liefert 40 Ergebnisse, der gespeicherte
Bestand umfasst 42. Damit ist die Vollständigkeit des Helden-Scans gegenüber
dem historischen Bestand nicht belegt. Die Differenz ist kein Sprachproblem;
sie beweist für sich aber auch kein festes Top-N-Limit, weil der Bestand
historisch ist und Builds inzwischen verschwunden sein können. Das vorhandene
Proto bietet weder Offset noch Cursor oder Seitengröße. Eine erfundene
Pagination wurde deshalb nicht ergänzt.

Build 779996 ist live per exakter Suche erreichbar. Ob er in den aktuellen
40 ungefilterten Ergebnissen enthalten ist, lässt sich über die vorhandene
Task-API nicht direkt feststellen: `GcSearchBuildsHandler` gibt nur
`response` und `result_count`, keine Ergebnis-IDs aus. Der Katalog belegt ihn
zuletzt am 12.09.2026 um 20:47:11 UTC mit dem angeforderten Autor und Version
45. Dies ist ausdrücklich kein neuer Live-Beleg für seine Mitgliedschaft in
der ungefilterten Antwort. Der im Briefing erlaubte exakte Lookup ist
abgeschlossen; das Top-N-Risiko bleibt dokumentiert. Für einen direkten
Mitgliedschaftsbeweis wäre eine Erweiterung des lesenden API-Ergebnisses in
`gc.rs` samt Deployment nötig, außerhalb der erlaubten Dateien dieser Runde.

### Budget und Mehrblock-Zähler

1522 Builds × 2 DB-Abfragen ergeben 3044 Abfragen je Heldenzyklus.
Vorher waren es bei 13 redundanten Autorenscans 39.572 Abfragen, also 36.528
mehr. Die zwei Abfragen pro Build sind unverändert; die Wiederholung je
Autor entfällt. Die Rechnung umfasst Lookup und erfolgreichen Upsert,
keine Status- und Planungsabfragen.

420 s / 3044 ergeben rechnerisch etwa 138 ms je DB-Abfrage, wenn man alle
anderen Kosten ignoriert. Bei 38 Helden verbrauchen schon 37 Abstände à 3 s
111 s; die verbleibenden 309 s entsprechen höchstens etwa 101,5 ms je
DB-Abfrage, noch vor GC-Antwortzeiten und Statuskosten. Das ist eine
Budgetrechnung, keine Messung produktiver Schreiblatenzen und keine
Laufzeitgarantie. Der äußere Task-Timeout bleibt 480 s, der Scan reserviert
60 s. Diese Runde führt keine produktiven Upsert-Benchmarks durch.

Eine verlässliche Addition über `cycle_started_at` ist mit den vorhandenen
strukturierten Daten nicht möglich: `watched_build_authors` speichert Status,
Zeit und einen Freitext, aber keine Zähler oder Heldenmengen je Zyklus. Die
Quelltabelle erhält nicht die Einordnung „neu/aktualisiert“ jedes Blocks.
Freitext zu parsen wäre von alten Statusformaten abhängig; Heldenmengen
ließen sich daraus nicht zuverlässig deduplizieren. Deshalb wird die im
Briefing ausdrücklich erlaubte Alternative mit Blockangabe verwendet, ohne
neue Tabelle oder Migration.

### Tests und Selbstprüfung

| Prüfung | Baseline | Endstand `362284e` |
| --- | --- | --- |
| Offline `steam-core --features testing` | 191 bestanden, laut Briefing und dokumentierter Fixrunde 1 | 191 bestanden, 0 fehlgeschlagen, 88 DB-Tests gezielt gefiltert |
| Katalog mit zentraler Wegwerf-Test-DB | 40 bestanden, 0 fehlgeschlagen, hier erneut gegen `ffc34fa` gemessen | 44 bestanden, 0 fehlgeschlagen |
| Erste Rot-Gegenprobe vor Implementierung | 38 bestanden, 5 erwartete Fehler | Alle 5 Fälle grün |
| Zusätzliche Rot-Gegenprobe mit gezielter Rücknahme | 16 bestanden, 4 erwartete Fehler in 20 Discovery-Tests | Alle 4 Fälle grün |

Vier neue Tests: Fremdautor/Katalogzähler, Zuordnung bekannter Heldenfehler,
Dreiblock-Abbruchschutz, offener Taskzähler. Jeder neue Test hat einen roten
Gegenfall. Die erste Rotprobe scheitert außerdem an den angepassten
Erwartungen für Dreifehlerabbruch und Block-Payloads. Die zweite Rotprobe
setzt ausschließlich den alten Abbruch-Guard zurück und verändert den
Taskzähler um +1; sie belegt den Statusverlust nach früheren Zyklustreffern
unabhängig vom Blockpräfix sowie die exakte Zählung und Planungssperre.
Beide Mutationen wurden vor dem Endlauf vollständig zurückgenommen.

DB-Befehl wie in Fixrunde 1, mit unverändertem Docker-Shim:

```bash
export PATH=/tmp/steam-scrim-dockershim:/home/nathanael/.cargo/bin:$PATH
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-je-held/rust/Cargo.toml -p steam-core --features testing task::handlers::builds::catalog::tests
```

`cargo check -p steam-core --features testing` ist grün.
`rustfmt --check` ausschließlich auf den drei eigenen Dateien und
`git diff --check` sind grün. Clippy auf `steam-core`, allen Targets und
`--features testing` ist ohne Warnungsverschärfung grün. Mit `-D warnings`
bleibt ausschließlich der bekannte Baseline-Fehler `gc_health.rs:45`
(`RateLimitOutcome::cooldown`, `dead_code`), Exit 101. Keine neue
Clippy-Diagnose; der bekannte Future-Incompatibility-Hinweis für
`binrw 0.15.1` bleibt.

Prüflogs: `/tmp/steam-s3-fix2-baseline-db.log`,
`/tmp/steam-s3-fix2-red-db.log`, `/tmp/steam-s3-fix2-red-guards.log`,
`/tmp/steam-s3-fix2-final-db.log`, `/tmp/steam-s3-fix2-offline.log`,
`/tmp/steam-s3-fix2-check.log`, `/tmp/steam-s3-fix2-clippy.log` und
`/tmp/steam-s3-fix2-clippy-baseline-allowed.log`.

Die Selbstprüfung umfasst Diff, Scope, Statusschutz, parametrisierte
SQL-Abfragen, Autorenzuordnung und Testgegenproben. Keine neuen
Code-Kommentare, keine Änderungen an Lanes, Runner oder Proto, kein
Release-Build, keine Migration und keine produktiven Katalogschreibzugriffe.
Report und Live-Beleg liegen im Brain-Taskordner außerhalb des Code-Commits.

Die angeforderte zusätzliche Gate-Prüfung ist abgeschlossen, Exit 0,
`ALLOW`, keine blockierenden Befunde. Aufruf:

```bash
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/steam-bot-autoren-je-held --base ffc34fa --head HEAD
```

Der Gate-Log liegt unter `/tmp/steam-s3-fix2-gate.log`. Zwei Hinweise des
Gates werden bewusst nicht in weitere Scope-Änderungen übersetzt:

1. `watchedBuilds` zählt wie der bestehende Autorenzähler je
   `(build_id, language)`, also gegebenenfalls mehrere Sprachvarianten.
   Das ist die bestehende Ergebnis-Deduplizierung, keine neue Zusicherung
   eindeutiger Build-IDs über alle Sprachen.
2. Erfolgreiche Autorentreffer bleiben bei einem späteren Blockabbruch
   `ok`; der Task selbst schlägt weiterhin sichtbar fehl. Das ist genau
   die verbindliche Entscheidung zu Fund 3, kein offener Fix.

Push abgeschlossen: `origin/fix/autoren-scan-je-held` enthält
`362284e198a67870b717731e1c84f217ac56f672`, Push-Ausgabe
`ffc34fa..362284e`. Der Steam-Worktree ist sauber. Kein Push nach main,
kein Merge, Deployment oder Service-Neustart aus diesem Worktree.
