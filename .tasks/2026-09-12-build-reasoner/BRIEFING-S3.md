# Briefing: Autoren-Scan einmal je Held (Paket S3, klein)

[Orchestrator] Folgeauftrag zu Paket S (Deadlock-Steam-Bot). Befund 11 in
`PAKETE.md`: der GC filtert `CMsgClientToGCFindHeroBuilds` nicht nach
`author_account_id`. Jeder Autoren-Scan liefert dieselben rund 1522 Builds, die
Schleife je Autor (13 Autoren mal 38 Helden) ist 13-fach redundant und kostet
rund 2,3 Minuten GC-Verkehr je Autor.

- Repo: `/home/nathanael/repos/Deadlock-Steam-Bot`, Worktree
  `/home/nathanael/.worktrees/steam-bot-autoren-je-held` (existiert, Branch
  `fix/autoren-scan-je-held` ab main ad5f00e, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Kontext: `FERTIG-S2.md`, `REVIEW-S2.md` in
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`.
  Code: `rust/crates/steam-core/src/task/handlers/builds/discovery.rs`
  (`run_discovery_scoped_with` ab Zeile 123, Schleife Autor mal Held ab 155,
  `discovery_tasks` Zeile 66 plant je Autor einen Task
  `DISCOVER_WATCHED_BUILDS`), `catalog.rs:105` (Planung im Zyklus).

## Was du tust

1. Die Discovery fragt den GC einmal je Held (`hero_id`, Sprache wie bisher,
   ohne `author_account_id` oder mit dem ersten Autor, falls das Feld Pflicht
   ist; belege am Proto und mit einem lesenden Testtask, ob das Feld
   weggelassen werden darf). Alle Ergebnisse werden wie bisher per
   `upsert_hero_build_source` gespeichert.
2. Die Autoren aus `tierlist.watched_build_authors` werden clientseitig aus
   dem Ergebnis zugeordnet (`author_account_id` des Builds). Status je Autor
   (`update_watched_author_status`: `ok`, `partial`, `error`, Nachricht mit
   Anzahl gefundener Builds und Helden) wird aus dieser Zuordnung abgeleitet,
   ein Autor ohne Treffer bekommt `partial` mit "keine Builds im Katalog".
3. Planung im Zyklus (`discovery_tasks`): statt 13 Tasks je Autor eine
   begrenzte Zahl Teil-Tasks je Helden-Block (das 480-s-Budget je Task aus
   S2 bleibt; wähle die Blockgröße so, dass ein Block sicher unter 480 s
   bleibt, rund 3,6 s je Held laut S2). Payload `hero_ids: [..]` statt
   `author_account_id`; alte Payloads mit `author_account_id` bleiben
   verarbeitbar (Übergang), werden aber nicht mehr geplant.
4. Journalzeile je Block ("Helden-Scan beendet", Anzahl Helden, Builds,
   Autoren-Treffer) wie in S2 je Autor.
5. Tests nachziehen (`catalog.rs` Tests ab 1175 prüfen Payloads und
   Autorenstatus). Live-Nachweis nach dem Merge macht der Orchestrator.

## Regeln

- Dateien: `discovery.rs`, `catalog.rs`, bei Bedarf `steam-persistence`
  (Autorenstatus) additiv. Keine Änderungen an Lanes, Runner, Proto.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `steam-core` mit `--all-targets -- -D warnings`, die Tests von `steam-core`
  (DB-Tests wie in der Memory `steam-bot-deploy-und-test-weg`: Test-DB über
  `central_test_db.sh` in Deadlock-Bots plus docker-shim; nenne Baseline und
  Endstand). Kein `--release`. Keine Schreibzugriffe auf die Prod-DB, keine
  Steam-Tasks auf dem Live-Bot außer lesenden Testanfragen über die
  bestehende Task-API.
- Selbstprüfung vor der Fertigmeldung. Nur `fix/autoren-scan-je-held`
  committen und pushen, nie main. Commit-Trailer `Co-authored-by: <dein
  Modell> <modell@local>`. Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als `FERTIG-S3.md` im Task-Ordner des Brain-Repos:
Proto-Beleg zum Autorenfeld, Änderung mit Datei:Zeile, Commit-SHA, Testzahlen
(Baseline und Endstand), geplante Task-Zahl je Zyklus vorher und nachher.
