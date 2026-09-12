# Review Runde 1: Paket S (Autoren-Scan reaktivieren)

Branch `feat/autoren-scan-reaktivieren`, Commit `f82c21c`, Basis `796bb3c`.
Lesendes Review, kein Code, kein Branch. Diff, Fertigmeldung, echtes Schema und
Referenz-IDs (deadlock-api, lesend) geprueft.

## Urteil

**NACHBESSERN.** Der Kernfix (Scheduling und Handler) stimmt und die Persistenz
ist korrekt. Zwei Punkte aus dem Briefing sind offen: die
`watched_build_authors.last_checked_*`-Rueckschreibung fehlt vollstaendig
(Punkt 3), und der Referenzfall greift mit den vorgeschlagenen Autoren nicht auf
die gepflegte Warden-Fassung zu (Punkt 4). Beides ohne Migration loesbar.

## Maengel

### 1. watched_build_authors.last_checked_* wird nie gesetzt (wichtig)
`rust/crates/steam-core/src/task/handlers/builds/discovery.rs:66-153`
(`run_discovery`) liest `load_watched_authors`, schreibt aber nach dem Lauf
nichts zurueck. In `rust/crates/steam-persistence/src/builds.rs:57-59` traegt
`WatchedAuthor` nur `author_account_id`; eine Update-Funktion fuer
`tierlist.watched_build_authors` existiert nicht (die `last_checked_*`-Updates in
`builds.rs:857-918` betreffen `deadlock_hero_builds`, nicht die Autoren).
Genau ueber diese Spalten wurde der Ausfall erkannt (AUFTRAG: "0 builds from 0
heroes", Status `partial`). Nach der Reaktivierung bleibt dieses Signal aus, ein
kuenftiger Null-Lauf ist in der DB nicht mehr sichtbar. Review-Briefing Punkt 3
verlangt das ausdruecklich, FERTIG-S erwaehnt es nicht.
Vorschlag: pro Autor nach dem GC-Call `last_checked_at = now()` plus
`last_checked_status` (`ok`, `partial`, `error`) und bei Fehler die Meldung
schreiben; kleine Update-Funktion in `builds.rs` und Aufruf in `run_discovery`.
Spaltennamen vorher gegen das echte Schema pruefen (AUFTRAG legt ihre Existenz
nahe).

### 2. Referenzfall verfehlt die gepflegte Warden-Fassung (wichtig)
Verifiziert ueber deadlock-api (lesend):
- Build `779996`: hero_id 25, Autor `1650097169`, Version 31, Name
  "LIGHTBRINGERxSITUATION WARDEN BUILD". Das ist die aktive, gepflegte Fassung.
- Build `782057`: hero_id 25, Autor `13446690` (Lightbringer), Version 1,
  gleicher Name. Eine veraltete Kopie.
- `13446690` = Lightbringer (Builds "( LIGHTBRINGER )"), `34634349` = Situation
  ("TheS1tuation ..."). Beide Zuordnungen stimmen.

Der Autoren-Scan sucht per `author_account_id`. Ueber die in FERTIG-S
vorgeschlagenen Autoren `13446690` und `34634349` findet er die Warden-Version 31
nicht: `13446690` liefert nur die veraltete Kopie `782057` (v1), `34634349`
liefert gar keinen Warden-Build. Die v31 haengt an `1650097169`. FERTIG-S
schliesst diesen Account bewusst aus ("Verifikationshinweis, kein zusaetzlicher
Eintrag"). Damit landet fuer den Backtest nur eine v1-Altfassung in
`hero_build_sources`, nicht der Live-Build.
Vorschlag: `1650097169` in die SQL-Empfehlung aufnehmen. Er fuehrt zusaetzlich
weitere aktuelle "TheS1tuation"-Builds und wirkt wie der aktive Situation-Account;
`34634349` und `1650097169` gemeinsam beobachten, sonst ist der Referenzfall des
Auftrags nicht erfuellt.

### 3. Totalausfall der Discovery meldet trotzdem Erfolg (nit)
`discovery.rs:97-149` und `catalog.rs:99-114`: fallen alle Autoren mit GC-Fehler
(z. B. Code 2 `k_eTooBusy` oder Timeout) aus, bleibt `authors_checked = 0`, das
Task-Ergebnis ist aber `success: true` mit gefuellter `errors`-Liste. Zusammen
mit Mangel 1 ist ein kompletter Fehllauf in der DB unsichtbar. Mit Mangel 1
behoben; sonst mindestens eine Log-Warnung bei `authors_checked == 0 &&
!errors.is_empty()`.

### 4. Autor-Ueberschreibung mit 0 moeglich (nit)
`builds.rs:287` setzt jetzt `author_account_id = EXCLUDED.author_account_id` im
Upsert; `convert.rs:177` liefert bei fehlendem GC-Autor `0` (`unwrap_or(0)`).
Ein Re-Scan ohne Autor koennte einen bekannten Autor mit 0 ueberschreiben. Bei
autorbasierter Suche praktisch immer gesetzt, daher gering. Optional: bei 0 nicht
ueberschreiben (`NULLIF`/`COALESCE`).

### 5. Beide DISCOVER_*-Handler laufen jetzt volle Discovery (nit)
`discovery.rs:14-30`: `DISCOVER_WATCHED_BUILDS` und `DISCOVER_BUILDS_VIA_HEROES`
rufen beide `run_discovery_task`. Im Repo plant nur `main.rs` den
`BUILD_CATALOG_CYCLE` ein, also kein Doppellauf pro Tag. Falls ein externer
Enqueuer (Python-Dashboard) diese Task-Typen noch einstellt, faellt GC-Last
doppelt an. Kurz gegen die Enqueuer pruefen.

## Bestaetigt korrekt

- **Ursache und Fix:** Der Scheduler plante nur `MAINTAIN_BUILD_CATALOG`, die
  Discovery-Handler waren No-op-Stubs (Commit `211a6474`). `main.rs:271` enqueued
  jetzt `BUILD_CATALOG_CYCLE`, der Handler fuehrt Discovery und Maintenance aus
  (`catalog.rs:87-138`), die Dedup deckt beide Typen ab (`builds.rs:932`). Kein
  Maintenance-Verlust, kein doppelter Lauf pro Tag.
- **GC-Last:** seriell mit `GC_REQUEST_DELAY` zwischen Autoren, keine neue
  Poll-Schleife, keine Retries bei Code 2/5, ein Autorenfehler bricht die
  anderen nicht ab und steht in der Rueckgabe (`discovery.rs:82-149`).
  Watchdog-Pfad (`supervisor.rs`) unveraendert.
- **Persistenz:** Upsert schreibt `details` (`modCategories`, `abilityOrder` via
  `gc_build_to_source`, unveraendert), `version`, `published_at`,
  `last_updated_at`, `last_seen_at` (`builds.rs:277-314`). Keine Migration noetig,
  keine geratenen Spalten.
- **.sqlx-Cache:** die zwei geaenderten Queries (Upsert, Dedup) haben passende,
  neu generierte Cache-Dateien, kein Handedit.
- **Scope:** nur Katalog- und Persistenzpfad, keine Lobby-, Rank-, Invite- oder
  Publish-Aenderung.
- **Clippy:** Warnungen in `gc_health.rs:45` und `scrim_lobby.rs:852` sind
  vorbestehend, beide Dateien liegen nicht im Diff.
- **Deploy-Risiko (Punkt 6):** 10 Autoren mal ein GC-Request mit 3 s Abstand
  ergibt rund 27 s Discovery, danach Maintenance (bis zu 12 Sprachvarianten je
  konfiguriertem Build). Seriell und vertretbar, kein Sturm.

## Deploy-Reihenfolge (nach der Nachbesserung)

1. Merge des Branches nach `main` durch die Merge-Schleuse.
2. `cargo build --release` fuer `steam-core`.
3. Service-Neustart `steam-core`.
4. SQL auf Prod: `watched_build_authors` um Lightbringer (13446690), Situation
   (34634349) und den aktiven Kollab-Account (1650097169) ergaenzen.
5. Live-Proof: `BUILD_CATALOG_CYCLE` anstossen, dann
   `SELECT ... FROM tierlist.hero_build_sources WHERE hero_id = 25` prueft, dass
   der Warden-Build in Version 31 (Autor 1650097169) erscheint, nicht nur die
   v1-Kopie 782057.

## Fixrunde 1

Stand: 2026-09-12. Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan`.
Branch: `feat/autoren-scan-reaktivieren`, Basis `f82c21c`.
Fix-Commit für die folgenden Änderungen: `102ec83fe91548e79963be9aa93dc5d7baee3c66`.

### Abgleich der fünf Mängel mit dem eigenen Diff

1. **Behoben.** `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:184`
   bestimmt pro Autor `ok`, `partial` oder `error` und schreibt auch bei
   GC-Code 2/5 und Timeout Zeitstempel und Meldung zurück. Die Meldung enthält
   gefundene Builds, unterschiedliche Helden, neue und aktualisierte Quellen
   sowie Fehlerzahl und Fehlerdetails. Eine erfolgreiche leere Antwort wird
   `partial` mit `0 builds from 0 heroes`. Teilweise gespeicherte Ergebnisse
   mit Fehlern werden `partial`, ein vollständiger Fehlschlag `error`.
   `rust/crates/steam-persistence/src/builds.rs:857` ergänzt den vorhandenen
   Persistenzpfad; der Db-Wrapper steht bei `:1224`. Fehler beim Status-Update
   stehen selbst im Task-Ergebnis und zählen nicht als erfolgreicher Autor.
   Die acht tatsächlichen Spalten von `tierlist.watched_build_authors` wurden
   zusätzlich lesend in der Live-DB über `information_schema.columns` geprüft.
   Keine Migration.
2. **Behoben.** `FERTIG-S.md:78` empfiehlt jetzt alle drei Accounts einschließlich
   `1650097169` mit der Notiz `Kollab-Autor Lightbringer x Situation`.
   `FERTIG-S.md:134` verlangt im Live-Proof ausdrücklich `779996`, Held `25`,
   GC-Autor `1650097169`, Version mindestens `31` und einen frischen
   `last_seen_at`. Die v1-Kopie `782057` genügt nicht. Die Status-Abfrage prüft
   zusätzlich alle drei Autoren. Der DB-Regressionstest
   `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:1213` sucht unter
   `13446690`, bekommt einen Build von `1650097169` und erhält diesen GC-Autor
   beim Einfügen und Aktualisieren auf Version 31. Es erfolgt keine
   Normalisierung auf den beobachteten Account. Die Testantwort ist simuliert;
   der API-Befund bleibt der lesende Nachweis aus Review Runde 1.
3. **Behoben.** `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:59`
   gibt bei vollständigem Fehlschlag `TaskFailure` mit Zählern und Fehlerdetails
   zurück. Dadurch setzt der vorhandene Runner den Task auf `FAILED` und
   `ok: false`. `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:108`
   übernimmt dies auch für `BUILD_CATALOG_CYCLE`. Maintenance läuft weiterhin
   nach Discovery und ihr Ergebnis bleibt im Fehlerobjekt erhalten. Ohne aktive
   Autoren bleibt der leere Lauf erfolgreich. Beleg: `catalog.rs:1359` und
   `:1424`.
4. **Behoben.** `rust/crates/steam-persistence/src/builds.rs:287` erhält beim
   Upsert mit `COALESCE(NULLIF(EXCLUDED.author_account_id, 0), ...)` den bekannten
   Autor bei fehlender oder explizit nullwertiger GC-ID. Eine tatsächlich
   gelieferte andere Account-ID darf weiterhin aktualisieren. Regressionstest:
   `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:1333`.
5. **Geprüft und begründet belassen.**
   `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:14` und `:23`
   bleiben gemeinsame Einstiegspunkte in dieselbe Discovery. Der Reviewer
   verlangt eine Enqueuer-Prüfung, keine zwingende Trennung. Die Suche in den
   aktuellen Rust-, Python-, JavaScript- und TypeScript-Quellen unter
   `/home/nathanael/repos` fand nur Registrierung und Lane-Zuordnung im Steam-Bot,
   keine Enqueuer für diese beiden Task-Typen. In Deadlock-Bots gibt es nur einen
   historischen Doku-Verweis. Eine lesende gruppierte Abfrage auf
   `steam.steam_tasks` lieferte für beide Typen keine Zeilen. Der aktive
   Tages-Scheduler stellt `BUILD_CATALOG_CYCLE` ein. Externe, nicht vorliegende
   dynamische Enqueuer lassen sich damit nicht grundsätzlich ausschließen;
   für eine zusätzliche Trennung gibt es im geprüften Bestand keinen Anlass.

### Prüfungen

| Prüfung | Baseline | Rot-Gegenprobe | Endstand |
| --- | --- | --- | --- |
| Offline: steam-core und steam-persistence | 174 bestanden, 0 fehlgeschlagen | unverändert | 174 bestanden, 0 fehlgeschlagen |
| Docker-Katalogtests | 18 bestanden | 19 bestanden, 7 fehlgeschlagen | 26 bestanden, 0 fehlgeschlagen |
| Docker-Persistenztransaktionen | 2 bestanden | unverändert | 2 bestanden, 0 fehlgeschlagen |

Die Offline-Baseline wurde auf `f82c21c` erneut ausgeführt. Für die
Rot-Gegenprobe wurde zuerst nur der bestehende GC-Aufruf als injizierbarer
Callback und die bestehende Ergebnisbildung als Funktion herausgezogen,
noch ohne Verhaltensfix. Die sieben neuen Fehlerfalltests scheiterten
anschließend an den Review-Mängeln; die 18 bestehenden Tests und die neue
Kontrollprobe ohne aktive Autoren bestanden. Nach den Fixes bestehen alle acht
neuen Tests. Auch der echte PostgreSQL-Constraint-Fehler beim Quellen-Upsert
und ein abgewiesenes Autoren-Status-Update sind abgedeckt.

Ausgeführte Testbefehle:

```bash
export PATH=/home/nathanael/.cargo/bin:$PATH
SQLX_OFFLINE=true cargo test -p steam-core -p steam-persistence \
  --manifest-path rust/Cargo.toml
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh \
  cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-scan/rust/Cargo.toml \
  -p steam-core --features testing -- task::handlers::builds::catalog::tests --include-ignored
/home/nathanael/repos/Deadlock-Bots/rust/scripts/central_test_db.sh \
  cargo test --manifest-path /home/nathanael/.worktrees/steam-bot-autoren-scan/rust/Cargo.toml \
  -p steam-persistence --features testing --test build_catalog_transactions -- --include-ignored
```

Der im Briefing genannte Wrapper existiert im Steam-Bot nicht. Genutzt wurde
unverändert der vorhandene Wrapper unter Deadlock-Bots; dessen Wegwerf-DB
enthält die zentralen Migrationen. Es wurde kein Ersatzskript ins Repo gebaut.

- Compiler: `cargo check -p steam-persistence` gegen die migrierte Wegwerf-DB
  erfolgreich; beide Crates zusätzlich durch die Testläufe kompiliert.
- SQLx: die Metadaten wurden vom SQLx-Makro beim Online-Check mit
  `SQLX_OFFLINE_DIR` erzeugt, ohne JSON-Handedit. Neuer Status-Cache
  `query-c23a021af7966dcf82b69e0881cdb2df87de08962690828d33e377bb7dcdd33a.json`;
  Upsert-Cache `query-c447440a5aa3b0f2a7b6db391a4de0d309559870679e347dd6f498f0eda5bdb3.json`
  ersetzt `query-12f619db3a07acfb66b621f840990c45c250a08d4bb562944690722d4df1924b.json`.
- Formatter: `rustfmt --check` für die beiden angefassten Core-Dateien
  erfolgreich. Die drei eigenen Persistenzfunktionen stimmen mit der
  rustfmt-Ausgabe überein; vorbestehende Formatabweichungen außerhalb des Diffs
  bleiben erhalten. `git diff --check` erfolgreich.
- Clippy: `steam-core --all-targets --features testing` sowie
  `steam-persistence --all-targets` offline erfolgreich. Der gemeinsame
  Offline-Lauf mit allen Testfeatures traf auf neun fehlende Cache-Einträge
  älterer Tests in `commands.rs`, `links.rs` und `tasks.rs`; diese Dateien
  gehören nicht zum Fix. Der gemeinsame Lauf wurde daher gegen die migrierte
  Wegwerf-DB mit `DATABASE_URL` und `SQLX_OFFLINE=false` wiederholt:
  erfolgreich (Exit 0), keine Warnung in den geänderten Dateien.
  Verbleibend sind die vorbestehende Warnung `gc_health.rs:45` (`cooldown`)
  und der Future-Incompatibility-Hinweis für `binrw 0.15.1`.
- Kein Release-Build, keine neue Poll-Schleife, keine zusätzlichen GC-Retries,
  keine Änderungen an Lobby, Rank, Invite oder Publish. Keine Unter-Agenten.
- Logs: `fixrunde-1-s-baseline.log`, `fixrunde-1-s-red.log`,
  `fixrunde-1-s-green.log`, `fixrunde-1-s-offline.log` und
  `fixrunde-1-s-clippy-db.log` in diesem Task-Ordner.

### Aktualisierter SQL-Vorschlag

```sql
INSERT INTO tierlist.watched_build_authors
    (author_account_id, notes, is_active)
VALUES
    (13446690, 'Lightbringer', true),
    (34634349, 'Situation', true),
    (1650097169, 'Kollab-Autor Lightbringer x Situation', true)
ON CONFLICT (author_account_id) DO UPDATE
SET notes = EXCLUDED.notes,
    is_active = EXCLUDED.is_active;
```

Nur vorbereitet, nicht auf Prod ausgeführt. Der aktualisierte Live-Proof steht
vollständig in `FERTIG-S.md`. Deployment und anschließender echter GC-Scan
bleiben bei der Merge-Schleuse beziehungsweise dem Delegator.

Push: `origin/feat/autoren-scan-reaktivieren` auf `102ec83fe91548e79963be9aa93dc5d7baee3c66` bestätigt.
Es wurde ausschließlich der eigene Feature-Branch gepusht, niemals `main`.
