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
