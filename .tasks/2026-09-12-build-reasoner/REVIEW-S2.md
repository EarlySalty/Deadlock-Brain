# Review-S2: build-reasoner Paket S, Runde 3 (Live-Fix Autoren-Scan)

Lesend gegen Worktree `/home/nathanael/.worktrees/steam-bot-autoren-scan-2`,
Branch `fix/autoren-scan-live`, Commit `bc08e23`, Diff `aba7636..HEAD`
(514 neu, 204 entfernt). Intent-Thread `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.

## Urteil: FREIGABE

Die Ursache ist empirisch belegt und der Fix trifft sie: Die GC-Suche braucht
je Held eine eigene Anfrage mit `hero_id`, der Zyklus zerfaellt in beschraenkte
Teilaufgaben, jeder Autor bekommt Status und Journalzeile. Testzahlen des
Fixers reproduziert (174 offline, 34 Docker-Katalog). Drei Restpunkte sind
gering und blockieren nicht; einer gehoert in die Live-Pruefung.

## Was geprueft wurde, Befund je Punkt

1. **Ursache (belegt).** FERTIG-S2 Tabelle: Autor allein liefert Response 0,
   Autor plus Held 25 liefert Response 1 mit 40 Treffern und Build 779996. Der
   Code fragt jetzt je Held gezielt (`discovery.rs`
   `run_discovery_scoped_with`, Request mit `hero_id: Some(*hero_id)`). Kein
   Held 0, keine Umdeutung von Code 0. GC-Last: 13 Autoren mal die Helden aus
   dem Katalog, seriell in Lane `background` mit Parallelitaet 1
   (`lanes.rs:78-91`), 3 s Abstand. Rund 114 bis 180 s je Autorentask, unter
   dem 420-s-GC-Budget und dem 480-s-Taskbudget. Drosselung (Code 5) bricht
   nur den betroffenen Autor ab (`break`), der naechste Autorentask laeuft
   weiter. Vertraeglich.

2. **Laufzeit (in Ordnung).** `BUILD_CATALOG_CYCLE` plant nur ein
   (`catalog.rs` `enqueue_catalog_tasks`, reine DB-Transaktion) und endet
   schnell. Teilaufgaben sind durch `bounded_catalog_task` auf 480 s
   gedeckelt (`shared.rs:12`), sicher unter dem Stale-Limit 600 s
   (`runner.rs:34`). Dedup gegen offene Tasks gleichen Typs und gleicher
   Payload in derselben Transaktion (`builds.rs:961`); `has_open_catalog_
   maintenance_task` deckt jetzt auch die beiden DISCOVER-Typen ab
   (`builds.rs:948-951`), ein neuer Zyklus wird also nicht gestartet, solange
   Kinder offen sind. Task 4047138: ein zweiter Zyklus plant nur noch, seine
   Kinder deduplizieren gegen die offenen und er endet schnell, kein Haengen
   mehr hinter einem Langlaeufer. Keine neuen Task-Typen; alle vier stehen im
   `background`-Lane-Register. Cleanup laeuft genau einmal (cleanup_only-Task),
   Per-Build-Tasks ueberspringen Phase 2 (`catalog.rs:193`) und machen genau
   einen GC-Call.

3. **Sichtbarkeit (in Ordnung).** `Autoren-Scan beendet` je Autor mit
   Response-Codes, Builds, Helden, Neu, Update, Fehlern, Status
   (`discovery.rs:282`). `Katalogzyklus beendet` je Zyklus mit
   `phase="planning"` (`catalog.rs`). Keine Zeile je Held.

4. **Persistenz (in Ordnung).** Upsert in `hero_build_sources` unveraendert
   inklusive `details`, `version`, Zeitstempel. `update_watched_author_status`
   je Autor mit ok/partial/error und Meldung (`discovery.rs:271`); Status
   `error` zaehlt den Autor nicht als geprueft. Kein Overwrite mit 0 (COALESCE
   aus Runde 1). SQLx-Cache konsistent: der Offline-Lauf mit `SQLX_OFFLINE=true`
   kompiliert alle `query!`-Makros gegen den Cache und ist gruen, also passen
   die neue `enqueue_catalog_tasks`-Query und die geaenderte
   `has_open_catalog_maintenance_task`-Query zum Cache.

5. **Scope und Hygiene (sauber).** Nur Katalog-, Discovery-, Persistenz- und
   `shared.rs`-Pfad plus eine Dev-Dependency. Keine Lobby-, Rank-, Invite-,
   Publish-Aenderung, keine Migration, keine neue Poll-Schleife, keine neuen
   Code-Kommentare (die veraltete Modul-Doku zum Ein-Task-Zyklus wurde
   entfernt, `mod.rs`). Neue Abhaengigkeit `tokio` mit Feature `test-util` nur
   als Dev-Dependency (`Cargo.toml:56`), begruendet durch den
   `start_paused`-Timeout-Test. Keine Secrets.

6. **Tests selbst gefahren.**

```
export PATH=/home/nathanael/.cargo/bin:$PATH
SQLX_OFFLINE=true cargo test -p steam-core -p steam-persistence --manifest-path rust/Cargo.toml
central_test_db.sh cargo test ... -p steam-core --features testing -- task::handlers::builds::catalog::tests --include-ignored
```

TESTNACHWEIS[TW-1]: 174 passed, 0 ignored | Baseline: 0 rot (offline, exit 0)
TESTNACHWEIS[TW-2]: 34 passed, 0 ignored (235 filtered) | Baseline: 26 rot-frei vor Fix, jetzt 34 (Docker-Katalog, exit 0)

Beide Endstaende decken sich mit FERTIG-S2. Der riskante Teil (echter GC) lief
in den Tests bewusst simuliert; echtes PostgreSQL im Wegwerfcontainer.

## Maengel

### 1. Discovery ignoriert `is_active` der Helden (gering)
`rust/crates/steam-persistence/src/builds.rs:145` (`all_hero_names`) liest alle
Zeilen aus `tierlist.deadlock_heroes` ohne `is_active`-Filter; `discovery.rs`
fragt jeden dieser Helden per GC ab. `load_configured_builds` filtert dagegen
`h.is_active = true` (`builds.rs:326`). Folge: fuer stillgelegte oder noch nicht
freigeschaltete Helden entstehen ueberfluessige GC-Reads je Autor, und die
Zahl pro Autorentask haengt an der Tabellengroesse statt an den aktiven
Helden. Bei rund 40 bis 60 Zeilen bleibt es im 420-s-Budget, ist also nicht
brechend, aber inkonsistent. Vorschlag beim naechsten Anfassen: Helden analog
zu `load_configured_builds` auf `is_active` filtern.

### 2. Code-1-Annahme bei buildlosem Helden nur simuliert (gering, Live-Punkt)
`discovery.rs`: Response ungleich 1 bricht die Heldenschleife des Autors ab
(`break`). Der Live-Beleg zeigt fuer Held 0 (ungueltig) Response 1 mit 0
Treffern, also liefert der GC auch ohne Treffer Code 1. Ein echter Held, den
der Autor nicht gebaut hat, wurde live nicht direkt getestet. Faellt er wider
Erwarten auf Code 0, wuerde die Schleife nach dem ersten leeren Helden abbrechen
und die restlichen Helden dieses Autors als Fehler markieren. Im Test nur
simuliert. Muss die Live-Pruefung mit abdecken (779996 fuer 1650097169 wird
trotz der vielen anderen Heldenabfragen gefunden). Nicht blockierend.

### 3. Unskaliertes MAINTAIN_BUILD_CATALOG liefe ins 480-s-Budget (Hinweis)
Ein `MAINTAIN_BUILD_CATALOG` ganz ohne `hero_id`/`build_id` wuerde alle
konfigurierten Builds seriell mit GC in einem Task abarbeiten und am
480-s-Budget scheitern statt fertig zu werden. Runde 1 hat belegt, dass kein
Enqueuer das tut; der Scheduler stellt nur `BUILD_CATALOG_CYCLE` ein, der jetzt
faechert. Verhalten ist graceful (sauberer Abbruch statt Stale-Haenger), kein
Handlungsbedarf.

## Deploy-Pruefung

Nach Merge, Release-Build und `steam-core`-Neustart einen Zyklus einplanen und
die Teilaufgaben verfolgen (nicht das DONE des Planungstasks als Scan-Erfolg
lesen):

```bash
curl -fsS -X POST http://127.0.0.1:8782/tasks \
  -H 'Content-Type: application/json' \
  --data '{"type":"BUILD_CATALOG_CYCLE","payload":{},"bot_account_id":1}'
journalctl --user -u steam-core --since '30 minutes ago' --no-pager \
  | rg 'Katalogzyklus beendet|Autoren-Scan beendet'
```

Erwartet: zuerst `Katalogzyklus beendet` mit `phase="planning"` und
`success=true`, dann je abgeschlossenem Autor eine `Autoren-Scan beendet`-Zeile
mit den echten Response-Codes (erfolgreich = 1) und Buildzahlen. Wegen der
seriellen Queue kann die letzte Autorenzeile deutlich nach zehn Minuten
erscheinen. Fuer 1650097169 muessen Builds gefunden werden, keine neuen
Stale-Fehler fuer die Katalog-Teilaufgaben.

DB-Nachweis (der GC-belegte Kernfall):

```sql
SELECT hero_build_id, hero_id, author_account_id, version, last_seen_at
FROM tierlist.hero_build_sources
WHERE hero_build_id = 779996 AND hero_id = 25
  AND author_account_id = 1650097169 AND version >= 31;

SELECT author_account_id, last_checked_at, last_checked_status, last_checked_message
FROM tierlist.watched_build_authors ORDER BY author_account_id;

SELECT id, type, status, error, started_at, finished_at
FROM steam.steam_tasks
WHERE type IN ('BUILD_CATALOG_CYCLE','DISCOVER_WATCHED_BUILDS','MAINTAIN_BUILD_CATALOG')
  AND created_at >= now() - interval '2 hours' ORDER BY id;
```

Erfolg heisst: 779996 mit frischem `last_seen_at`, Version mindestens 31 und
GC-Details vorhanden; alle Autoren mit frischem `last_checked_at` und Status
(fuer 1650097169 `ok` oder `partial`, nicht `error`); die per `taskIds`
angelegten Teilaufgaben DONE, keine auf FAILED mit Stale-Meldung.
