# Fix-Briefing: Autoren-Scan je Held (Paket S3, Fixrunde 2, Merge-Kritiker)

[Orchestrator] Fixer für Paket S3 nach dem Merge-Kritiker. Review Runde 2 hat
FREIGABE gegeben; beim Merge nach main hat der Kritiker sieben Funde
gemeldet, einer blockierend.

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-je-held` (Branch
  `fix/autoren-scan-je-held`, Commit ffc34fa, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Zeilennummern des Kritikers stammen aus Diff-Hunks, Abweichung um 1 Zeile
  möglich.

## Befunde des Merge-Kritikers, wörtlich

1. `discovery.rs:408 (run_discovery_block_with, upsert_hero_build_source)` |
   BLOCKING | Die Ergebnisschleife (Zeilen 364 bis 439) prüft
   `author_stats.get_mut(&author_id)` nur für die Zähler, nicht für die
   Persistenz: `gc_build_to_source` (389), `get_hero_build_source` (390) und
   `upsert_hero_build_source` (408) laufen für jedes Ergebnis, obwohl der
   Request seit Zeile 314 `author_account_id: None` sendet. Folge: pro Zyklus
   landen bis zu (GC-Seitengröße mal Heldenzahl) fremde Builds in der
   Tabelle, `totalNewBuilds`, `totalUpdatedBuilds`, `heroesFound` und das Log
   "Helden-Scan beendet" zählen Fremdmaterial, und 2 DB-Roundtrips pro
   Fremdbuild verbrauchen Budget innerhalb der 420-s-Deadline. Kein Test
   deckt einen Build eines nicht beobachteten Autors ab.
2. `discovery.rs:314` | NIT | Der Ansatz setzt voraus, dass FindHeroBuilds
   ohne `author_account_id` alle Builds eines Helden liefert. Liefert der GC
   eine gekappte Seite (Top-N), erreichen Builds beobachteter Autoren mit
   wenig Popularität den Scan nie. Check: einmal live für einen Helden mit
   bekanntem Build eines beobachteten Autors `results.len()` gegen die Zahl
   bekannter Builds vergleichen.
3. `discovery.rs:480 (abort_reason)` | NIT | Bei Abbruch bekommen alle
   beobachteten Autoren unbedingt "partial" mit dem Abbruchtext; Autoren mit
   Treffern im selben Block verlieren ihre Zähler, Autoren mit "ok" aus
   einem früheren Block werden herabgestuft. Test
   `discovery_aborts_after_three_consecutive_hero_failures` zementiert das.
4. `discovery.rs:466 ("keine Builds im Katalog")` | NIT | Helden-Fehler
   hängen an keinem Autor. Ein Autor, dessen einziger Held im GC fehlschlug,
   bekommt "0 builds from 0 heroes; keine Builds im Katalog" statt eines
   Hinweises auf den GC-Fehler.
5. `discovery.rs:501` | NIT | Bei Mehrblock-Zyklen überschreibt ein späterer
   Block mit Treffern die Zähler des früheren Blocks; die Nachricht zeigt nur
   den letzten Block, nicht den Zyklus.
6. `discovery.rs:325 ("hero scan exceeded 420s")` | NIT | Der Text
   hartkodiert 420, während die Deadline aus `CATALOG_TASK_TIMEOUT -
   DISCOVERY_RESERVE` berechnet wird.
7. `discovery.rs:89 (discovery_tasks, discovery_pending)` | NIT | Guard gibt
   bei laufenden Discovery-Tasks leise `Vec::new()` zurück, kein Log; rohes
   `sqlx::query_scalar` in steam-core statt in steam-persistence (Muster
   `has_open_catalog_maintenance_task`, Zeilen 946 bis 958).

## Entscheidungen des Delegators

1. Fund 1: die Annahme des Kritikers ("vorher kamen nur Builds des
   angefragten Autors an") ist empirisch falsch, Befund 11 in `PAKETE.md`:
   der GC hat den Autorenfilter nie angewendet, jeder Autoren-Scan lieferte
   dieselben rund 1522 Builds, und genau diese Tabelle ist die Meta- und
   Backtest-Grundlage des Reasoners (`tierlist.hero_build_sources`, 42
   Warden-Builds im Backtest). Das Speichern aller Builds bleibt also
   ausdrücklich gewollt. Was fehlt, ist die Absicht im Code und im Test:
   Zähler trennen in Katalog (alle Builds: `totalNewBuilds`,
   `totalUpdatedBuilds`, `heroesFound`) und Autoren (`watchedBuilds`), Log
   "Helden-Scan beendet" nennt beide; ein Test mit einem Build eines nicht
   beobachteten Autors: persistiert, im Katalogzähler, in keinem
   Autorenstatus. Budget: die zwei DB-Roundtrips je Build waren vorher
   dieselben (je Autor 13-fach), jetzt einmal; im Bericht beziffern
   (1522 Builds mal 2 Abfragen gegen die Deadline).
2. Fund 2: lesende Gegenprobe über die bestehende Task-API auf dem laufenden
   Bot, wie in S2 (Task `DISCOVER_BUILDS_VIA_HEROES` oder der exakte Lookup):
   für Held 25 (Warden) `results.len()` und ob Build 779996 (Autor
   1650097169) enthalten ist. Ergebnis mit Zahl in den Bericht; keine
   Codeänderung, außer der GC kappt sichtbar.
3. Fund 3: beim Abbruch behalten Autoren mit Treffern im Block ihre Zähler
   und ihren Status (`ok`), nur Autoren ohne Treffer bekommen `partial` mit
   dem Abbruchtext; Autoren mit `ok` aus einem früheren Block des Zyklus
   (Schutz aus Fixrunde 1) werden nicht herabgestuft. Test anpassen.
4. Fund 4: je Autor die Helden seiner bekannten Builds (aus
   `hero_build_sources`) gegen die fehlgeschlagenen Helden prüfen; hat ein
   Autor keine Treffer und mindestens einer seiner Helden ist fehlgeschlagen,
   Nachricht "GC-Fehler bei Held X, Y" statt "keine Builds im Katalog".
5. Fund 5: Nachricht bei Mehrblock-Zyklen als Summe je Zyklus ist nicht
   nötig; stattdessen Nachricht mit Blockangabe ("Block 2 von 3: ...") und
   Zähler additiv über `cycle_started_at` nur, wenn das ohne neue Tabelle
   geht; sonst Blockangabe allein und im Bericht begründen.
6. Fund 6: Zahl aus der Konstante formatieren.
7. Fund 7: `info!`-Zeile "Discovery-Planung übersprungen, N Tasks offen" und
   die Abfrage als Funktion in steam-persistence neben
   `has_open_catalog_maintenance_task`.

## Regeln

- Dateien: `discovery.rs`, `catalog.rs`, `steam-persistence` (nur die neue
  Abfrage), Tests. Keine Änderungen an Lanes, Runner, Proto.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `steam-core` (Baseline-Hänger `gc_health.rs:45` bekannt), Offline-Tests
  von `steam-core` mit `--features testing` (Baseline 191) und die
  DB-Katalogtests mit der Test-DB aus Deadlock-Bots (Baseline 40). Kein
  `--release`. Keine Schreibzugriffe auf die Prod-DB; auf dem Live-Bot nur
  lesende Task-Anfragen wie in S2.
- Selbstprüfung vor der Fertigmeldung (`gate_hook.py --review` gegen die
  eigene Arbeit, wenn verfügbar). Neue Commits obendrauf, kein `--amend`.
  Nur `fix/autoren-scan-je-held` pushen, nie main. Commit-Trailer
  `Co-authored-by: <dein Modell> <modell@local>`. Echte Umlaute, keine
  Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 2 (Merge-Kritiker)" in
`REVIEW-S3.md`: je Fund Datei:Zeile, Änderung, Commit-SHA, Testzahlen
(Baseline und Endstand), Zahl aus der Gegenprobe zu Fund 2.
