# Paket S, Fixrunde 2

Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan-2`.
Branch: `fix/autoren-scan-live`, Basis `aba7636`.

Fertig: Commit `bc08e23989131aa742edba86de601d39a5f3c26e`, erfolgreich nach
`origin/fix/autoren-scan-live` gepusht. Worktree sauber. Kein Merge und kein
Push nach `main`. Trailer: `Co-authored-by: GPT-6 <gpt-6@local>`.

## Ursachenbeleg vor dem Fix

1. `citadel_gcmessages_client.proto:1383` definiert `k_eInternalError = 0`.
   Das Antwortfeld hat diesen Default (`:1388`). Der bestehende Discovery-Request
   (`discovery.rs:111`) enthält Autor und Sprache `[0,0]`, aber keinen Helden.
   Der erfolgreiche exakte Lookup (`discovery.rs:256`) enthält `hero_id`.
   Fünf lesende GC-Suchanfragen über den vorhandenen Task-Handler auf dem
   laufenden zweiten Bot, seriell, ohne Retry:

   | Task | Anfrage neben Sprache `[0,0]` | Echte Antwort |
   | --- | --- | --- |
   | 4047536 | Autor 1650097169 | FAILED, `Response code: 0` |
   | 4047539 | gleicher Autor, Held 25 | DONE, response 1, result_count 40 |
   | 4047541 | gleicher Autor, Held 0 | DONE, response 1, result_count 0 |
   | 4047556 | gleicher Autor, Held 25, Build 779996 | DONE, response 1, result_count 1 |
   | 4047566 | gleicher Autor, Suchtext LIGHTBRINGERxSITUATION WARDEN BUILD | FAILED, `Response code: 0` |

   Damit ist ein konkreter Held für diese GC-Suche erforderlich. Held 0 ist
   kein brauchbarer Platzhalter für alle Helden. Sprache war bereits identisch;
   Suchtext ersetzt den Helden nicht. Der existierende API-Handler gibt bei
   Code 0 keine Roh-Ergebnisliste heraus. Deshalb ist deren Leerheit nicht
   behauptet; die erfolgreiche Gegenprobe benötigt keine Umdeutung von Code 0.

2. `catalog.rs:99-102` führt alle Autoren und anschließend alle konfigurierten
   Builds seriell in einem Task aus. Live enthält der Bestand 13 Autoren,
   38 Helden und 71 aktive Build-Konfigurationen über 36 Helden.
   `tasks.rs:334-346` und `:280-304` prüfen Stale anhand `started_at`, nicht
   `updated_at`. Ein Heartbeat nur auf `updated_at` hilft damit nicht.
   `runner.rs:130-140` hält das Lane-Permit bis zum tatsächlichen Handler-Ende;
   `:214-244` ändert beim Stale-Reap nur den DB-Status. Background hat
   Parallelität 1 (`lanes.rs:92-105`).

3. Journal bestätigt Start 19:26:32 UTC und Stale-Reap 19:36:33 UTC für
   Task 4047117. Eine weitere lesende DB-Prüfung während dieser Fixrunde zeigt:
   Erst um 19:41:03 UTC beendet der ursprüngliche Handler seinen Lauf und
   überschreibt den Stale-Fehler mit `BUILD_CATALOG_CYCLE: discovery failed`.
   Gleichzeitig wechselt Task 4047138 von PENDING zu RUNNING, started_at
   19:41:03 UTC. Ursache ist das belegte Lane-Permit, keine Dedup-Sperre und
   keine hängende DB-Lease. Er wurde somit schon vor dem Fix von selbst abgeholt.

4. Die Discovery hat im vorhandenen Code keinen Journal-Aufruf pro Autor;
   der Zyklus-Handler keinen End-Log. Statuswerte allein in der DB erklären
   den fehlenden Einblick im Journal.

## Fix

Dateiangaben beziehen sich auf den oben genannten Worktree.

| Änderung | Datei und Zeile |
| --- | --- |
| Autorenscope aus Payload | `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:41` |
| Heldenkatalog und GC-Anfrage | `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:123`, `:174` |
| Journal pro Autor | `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:282` |
| Zyklus plant Teilaufgaben, End-Log | `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:95`, `:114` |
| Wartung pro Build | `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:121`, `:247` |
| Ausführungsbudget | `rust/crates/steam-core/src/task/handlers/builds/shared.rs:12` |
| Scheduler-Dedup und atomare Einplanung | `rust/crates/steam-persistence/src/builds.rs:946`, `:961` |

- `rust/crates/steam-core/src/task/handlers/builds/discovery.rs`: Jeder
  Autorentask nimmt nur den aktiven Autor aus `author_account_id` im Payload.
  Er fragt die konkreten Helden aus dem vorhandenen DB-Katalog seriell ab,
  jeweils mit Autor, Held und Sprache `[0,0]`. Kein Held 0, keine Umdeutung
  von Code 0. Der tatsächliche Autor aus dem GC-Build bleibt erhalten.
  Fehlender Heldenkatalog wird sichtbar als Fehler des Autors gespeichert.
- Die existierenden Status-Updates bleiben erhalten. Je Autor erscheint
  `Autoren-Scan beendet` mit `author_account_id`, `response_codes`, `builds`,
  `heroes`, `new_builds`, `updated_builds`, `errors` und `status`.
  Keine Journal-Zeile pro Held. Bei GC-Fehler oder Timeout endet die weitere
  Heldensuche dieses Autors, ohne Retry; der nächste Autor bleibt ausführbar.
- `rust/crates/steam-core/src/task/handlers/builds/catalog.rs`:
  `BUILD_CATALOG_CYCLE` plant einen `DISCOVER_WATCHED_BUILDS` je Autor,
  einen bestehenden Cleanup-Durchlauf ohne GC und einen
  `MAINTAIN_BUILD_CATALOG` je konfiguriertem Build mit `hero_id` und `build_id`.
  Die bisherigen Maintenance-Regeln und Publish-/Delete-Handler bleiben
  erhalten. Direkte bisherige Maintenance-Aufrufe behalten ihre Stats-Antwort.
- `rust/crates/steam-core/src/task/handlers/builds/shared.rs`:
  Katalog-, Discovery- und Maintenance-Handler sind auf 480 Sekunden begrenzt.
  Der Timeout verwirft das laufende Future, bevor der Runner bei 600 Sekunden
  nur den DB-Status bereinigen würde. Der Autorenscan begrenzt seine GC-Phase
  auf 420 Sekunden, damit Zeit für Status-Persistenz bleibt. Bereits gespeicherte
  Ergebnisse gehen bei einem Abbruch nicht verloren; ein unvollständiger Scan
  steht ausdrücklich als `partial` oder `error` mit Fehlermeldung in der DB.
  Eine Fortsetzung nach Timeout erfolgt nicht automatisch innerhalb desselben
  Zyklus; der nächste reguläre Tageslauf versucht den Autor erneut.
- `rust/crates/steam-persistence/src/builds.rs`:
  Die Teilaufgaben werden zusammen in einer Transaktion eingeplant und gegen
  bereits offene Tasks desselben Bots mit gleichem Typ und Payload dedupliziert.
  Die bestehende Scheduler-Dedup erkennt auch offene Discovery-Tasks.
  Die neuen SQLx-Metadaten stammen aus dem Compilerlauf gegen die migrierte
  Wegwerf-DB. Keine Migration und keine Handänderung am Cache.
- Ein Zyklus-Ende wird mit `Katalogzyklus beendet`, `phase="planning"`,
  `success`, Laufzeit und Ergebnis geloggt. Das Ergebnis enthält `scheduled`,
  `authorsScheduled`, `maintenanceUnits` und die neu angelegten `taskIds`.
  `DONE` des Zyklus bedeutet jetzt erfolgreiche Einplanung. Den tatsächlichen
  Scan-Erfolg zeigen seine Teilaufgaben und die Autorenstatuswerte.

## Laufzeit und Task 4047138

Der gesamte Tageslauf darf durch die seriellen Teilaufgaben länger als zehn
Minuten dauern. Entscheidend ist: Kein einzelner Kataloghandler hält das
Lane-Permit länger als sein 480-Sekunden-Budget. Für den live geprüften Bestand
werden regulär 13 Autorentasks, ein Cleanup und bis zu 71 Build-Tasks erzeugt.
Bei 38 Helden sind bis zu 494 Autoren-GC-Reads pro Tageslauf nötig, zuzüglich
Maintenance. Der vorhandene GC-Abstand bleibt bestehen, keine parallelen
GC-Abfragen und keine neue Poll-Schleife. Drosselung stoppt den jeweiligen Autor.

Task 4047138 wurde bereits vor dem Fix um 19:41:03 UTC automatisch aufgenommen.
Sollte beim Deploy ein Task noch PENDING sein, kann der neue Runner ihn normal
übernehmen. Ein bereits FAILED abgeschlossener Task wird nicht automatisch
wiederholt. Ein Neustart beseitigt auch die alten In-Memory-Permits; noch RUNNING
markierte alte Tasks werden vom bestehenden Startup-Reap beendet. Für den
Nachweis anschließend einen neuen Zyklus einplanen, keine DB-Statuswerte von
Hand korrigieren.

## Live-Prüfung nach dem Deploy

Merge, Deployment und Service-Neustart liegen beim Delegator. Es wurden nur
fünf lesende GC-Suchaufträge über die laufende API ausgeführt. Die beiden
vorherigen HTTP-422-Anfragen mit dem falschen Feldnamen `task_type` wurden
vor dem Enqueue abgewiesen und erzeugten keinen GC-Aufruf. Das korrekte
Wire-Feld heißt `type` (`api-contract/src/lib.rs:81`). Rohbelege der fünf
Task-Antworten: `fix2-s-gc-live.json` in diesem Ordner.

Nach Deployment einmal über die bestehende API einplanen. Falls Token-Auth
aktiviert ist, den vorhandenen `X-Internal-Token`-Header ergänzen:

```bash
curl -fsS -X POST http://127.0.0.1:8782/tasks \
  -H 'Content-Type: application/json' \
  --data '{"type":"BUILD_CATALOG_CYCLE","payload":{},"bot_account_id":1}'
journalctl --user -u steam-core --since '10 minutes ago' --no-pager \
  | rg 'Katalogzyklus beendet|Autoren-Scan beendet'
```

Erwartet: Ein rascher Zyklus-Abschluss mit `phase="planning"`, danach je
abgeschlossenem Autor eine Zeile mit den tatsächlichen Antwortcodes und
Buildzahlen. Erfolgreiche Antworten haben Code 1. Wegen der seriellen Queue
kann die letzte Autorenzeile deutlich später als zehn Minuten erscheinen.
Für 1650097169 sollen Builds gefunden werden; keine neuen Stale-Fehler für
die Katalog-Teilaufgaben. Neu angelegte Teilaufgaben über die `taskIds` aus
`GET /tasks/<cycle_id>` prüfen, nicht aus dem DONE des Planungstasks einen
fertigen Scan ableiten.

```sql
SELECT hero_build_id, hero_id, author_account_id, name, version,
       last_updated_at, last_seen_at,
       details->'modCategories' AS categories,
       details->'abilityOrder' AS ability_order
FROM tierlist.hero_build_sources
WHERE hero_build_id = 779996
  AND hero_id = 25
  AND author_account_id = 1650097169
  AND version >= 31;

SELECT author_account_id, last_checked_at, last_checked_status, last_checked_message
FROM tierlist.watched_build_authors
ORDER BY author_account_id;

SELECT id, type, status, payload, error, started_at, finished_at
FROM steam.steam_tasks
WHERE type IN ('BUILD_CATALOG_CYCLE', 'DISCOVER_WATCHED_BUILDS', 'MAINTAIN_BUILD_CATALOG')
  AND created_at >= now() - interval '2 hours'
ORDER BY id;
```

Build 779996 muss mit frischem `last_seen_at`, Version mindestens 31 und den
GC-Details vorliegen. Der Live-GC bestätigte seine Erreichbarkeit per exakter
Suche mit Held 25; der gesamte neue Discovery-Pfad ist vor Deployment nur gegen
simulierte GC-Antworten und echtes PostgreSQL getestet. Der konkrete Treffer in
der späteren unbeschränkten Helden-Autoren-Suche bleibt Teil dieser Live-Prüfung.

Kein Deploy, keine manuellen Korrekturen an Produktionsdaten, keine Unter-Agenten.

## Testnachweise und Selbstprüfung

| Prüfung | Baseline | Rot-Gegenprobe | Endstand |
| --- | --- | --- | --- |
| Offline steam-core und steam-persistence | 174 bestanden | unverändert | 174 bestanden, 0 fehlgeschlagen |
| Docker-Katalog | 26 bestanden | 27 bestanden, 2 fehlgeschlagen vor Fix | 34 bestanden, 0 fehlgeschlagen |
| Zusätzliche Katalog-Schutzmechanismen | Teil der erweiterten Suite | 28 bestanden, 6 fehlgeschlagen mit gezielten Rücknahmen | 34 bestanden, 0 fehlgeschlagen |
| Docker-Persistenztransaktionen | 2 bestanden laut Fixrunde 1 | unverändert | 2 bestanden, 0 fehlgeschlagen |

Die erste Rot-Probe enthält die 26 Bestandstests, eine grüne Kontrollprobe
für Code 0 und zwei rote Tests für `hero_id` und Zyklus-Einplanung. Die sechs
weiteren Rot-Gegenproben wurden durch gezielte temporäre Rücknahmen getestet:
zu spätes Ausführungsbudget, Einplanung ohne Transaktion, fehlender Build-Scope,
fehlender Autorenscope, stiller leerer Heldenkatalog und falsche Akzeptanz von
Code 0 beziehungsweise fehlendem Antwortcode. Damit hat jeder der acht neuen
Tests einen belegten roten Gegenfall. Anschließend wurden die Originaldateien
vollständig wiederhergestellt und die komplette Suite erneut grün ausgeführt.
Die zweite Gegenprobe ist ein Mutationstest, kein Lauf der unveränderten Basis.

Der Timeout-Test nutzt angehaltene Tokio-Zeit und ein Semaphore, um das
Verwerfen einer simulierten 600-Sekunden-Arbeit und die Ressourcenfreigabe vor
dem Stale-Limit zu prüfen. Dafür wurde ausschließlich im Dev-Dependency die
bestehende Tokio-Funktion `test-util` aktiviert. Alle Datenbanktests verwenden
echtes PostgreSQL in einem Wegwerfcontainer, ihre GC-Antworten sind simuliert.

Ausgeführte Befehle:

```bash
export PATH=/home/nathanael/.cargo/bin:$PATH
SQLX_OFFLINE=true cargo test -p steam-core -p steam-persistence --manifest-path rust/Cargo.toml
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh \
  cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-scan-2/rust/Cargo.toml \
  -p steam-core --features testing -- task::handlers::builds::catalog::tests --include-ignored
```

Wie in Fixrunde 1 liegt der vorhandene `central_test_db.sh`-Wrapper im
Deadlock-Bots-Repo, nicht im Steam-Bot. Er wurde unverändert verwendet.
SQLx wurde per `cargo check -p steam-persistence` mit `DATABASE_URL` der
migrierten Test-DB, `SQLX_OFFLINE=false` und `SQLX_OFFLINE_DIR` auf den lokalen
Cache erzeugt. Neue Dateien: `query-0206b6cd...json` und
`query-f9446923...json`; die alte Dedup-Metadatei `query-4243f9bc...json` entfällt.
Der anschließende Offline-Test beweist, dass die Metadaten verwendbar sind.

Zusätzlich erfolgreich, gemeinsamer Abschlusslauf mit Exit 0:

```bash
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh \
  cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-scan-2/rust/Cargo.toml \
  -p steam-persistence --features testing --test build_catalog_transactions -- --include-ignored
```

Clippy über beide Crates und alle Testziele mit `--features testing` wurde
im selben migrierten Wegwerfcontainer mit `DATABASE_URL="$CENTRAL_TEST_DSN"`
und `SQLX_OFFLINE=false` ausgeführt: Exit 0. Keine Warnung in den geänderten
Dateien. Verbleibend sind die vorbestehende Warnung `gc_health.rs:45`
(`cooldown`) und der Future-Incompatibility-Hinweis für `binrw 0.15.1`.
Die Offline-Tests melden außerdem den bereits bekannten ungenutzten
Test-Import in `scrim_lobby.rs:852`; auch diese Datei wurde nicht geändert.

`rustfmt --check` für alle geänderten Rust-Implementierungsdateien und
`git diff --check` sind grün. Kein Formatter-Lauf über fremde Dateien.
Die eigene Diff-Prüfung umfasst Task-Scope, Abbruchverhalten, Erhalt des
GC-Autors, DB-Transaktion, Statusmeldungen, Scheduler-Dedup und die Beschränkung
auf Katalog-/Discovery-/Persistenzpfad. Es wurde kein neuer Code-Kommentar
hinzugefügt; die veraltete Aussage zum Zyklus als einzigem langen Task wurde
aus der vorhandenen Moduldokumentation entfernt. Keine Lobby-, Rank-, Invite-
oder Publish-Handler geändert, keine Migration, kein Release-Build.

Logs im Task-Ordner:
`fix2-s-baseline.log`, `fix2-s-catalog-baseline.log`, `fix2-s-red.log`,
`fix2-s-red-guards.log`, `fix2-s-green.log`, `fix2-s-offline-final.log`,
`fix2-s-final-db-clippy.log`, `fix2-s-sqlx.log` und `fix2-s-gc-live.json`.
