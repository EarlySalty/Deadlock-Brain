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
