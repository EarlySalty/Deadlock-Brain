# Briefing: build-reasoner (Paket S, Autoren-Scan im Steam-Bot)

[Orchestrator] Paket S, Repo Deadlock-Steam-Bot. Hintergrund:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
und `ARCHITEKTUR.md` Abschnitt 11 (Backtest, Vergleichsbasis). Ziel dieses
Pakets: der Backtest des Build-Reasoners braucht frische Builds der
Top-Autoren in `tierlist.hero_build_sources`; der Scan liefert seit
2025-12-22 "0 builds from 0 heroes" (`watched_build_authors.last_checked_*`,
Status `partial`).

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan` (existiert,
  Branch ist ausgecheckt)
- Branch: `feat/autoren-scan-reaktivieren`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Fundstellen

- `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:26`
  `DiscoverBuildsViaHeroesHandler`: der Hero-Scan.
- `rust/crates/steam-persistence/src/builds.rs:841`: liest
  `tierlist.watched_build_authors`; `SourceRow` L79, `HeroBuildSource` L10,
  `load_configured_builds` L312.
- `rust/crates/steam-core/src/task/handlers/builds/convert.rs:122`
  `gc_build_to_source`.
- Erst `graphify query` im Repo-Root, dann `sed -n` an der Fundstelle.

## Was du tust

1. Ursache statt Symptom: Warum liefert der Scan 0 Builds aus 0 Helden? Wer
   stellt den Discover-Task ein (Timer, Loop, Admin-Befehl), läuft das noch,
   und antwortet der Game Coordinator auf die Hero-Suche noch (Rate-Limit
   Code 5, leerer Helden-Katalog, geänderte Proto-Felder)? Befund mit
   Fundstelle und Log-Beleg (Journal `steam-core`, `steam-bot`) in
   `FERTIG-S.md` im Task-Ordner des Brains, bevor du änderst.
2. Scan reparieren, so dass er je beobachtetem Autor dessen aktuelle Builds
   je Held holt und in `hero_build_sources` aktualisiert (`version`,
   `last_updated_at`, `details` mit Item-Reihenfolge und Kategorien, sofern
   der GC sie liefert). Bestehende Tabellen erweitern statt neue anlegen;
   Migrationen zentral in Deadlock-Bots `dl-central-db/migrations`, nur wenn
   wirklich nötig, und dann in der Fertigmeldung als Deploy-Voraussetzung.
3. Autoren ergänzen: Lightbringer und Situation (Warden-Build
   "LIGHTBRINGERxSITUATION WARDEN BUILD"). Account-IDs über die bestehenden
   Wege ermitteln (GC-Build-Suche nach Name oder deadlock-api), nicht raten;
   Einfügen in `watched_build_authors` gehört in die Fertigmeldung als
   SQL-Vorschlag, wird aber nicht von dir auf Prod ausgeführt.
4. Live-Beweis nur lesend vorbereiten: Kommando oder Task, mit dem der
   Delegator nach dem Deploy einen Scan anstößt, und die Abfrage, die den
   Warden-Build danach in `hero_build_sources` zeigt.

## Regeln

- Nur Scan- und Persistenzpfad der Autoren-Builds. Keine Änderungen an
  Lobby, Rank, Invite, Publish.
- Steam-Bot-Fallen: Valve drosselt bei zu vielen Abfragen (Code 5),
  Watchdog-Regeln in `supervisor.rs` beachten; keine neuen Poll-Schleifen,
  die den GC belasten, Scan bleibt selten (einmal täglich reicht).
- Keine Code-Kommentare. Kein Python.
- Toolchain: `export PATH=/home/nathanael/.cargo/bin:$PATH`. `cargo fmt` nur
  eigene Dateien, `cargo clippy` und `cargo test` je angefasstem Crate; Tests
  gegen die Docker-Test-DB laut Repo-Doku. Kein `--release`.
- Nur den eigenen Branch committen und pushen (`git push origin
  feat/autoren-scan-reaktivieren`), nie main. Commit-Trailer:
  `Co-authored-by: GPT 5.6 Luna <luna@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Bump-up

```
[Bump-up] Paket S: Grund: ... Erledigt: ... Worktree: /home/nathanael/.worktrees/steam-bot-autoren-scan Offen: ...
```

## Fertigmeldung

In diesem Thread und als Datei `FERTIG-S.md` unter
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`:
Ursache mit Beleg, Branch, Commit-SHAs, geänderte Dateien, Testlauf mit Zahlen
(Baseline und Endstand), Deploy-Voraussetzungen, SQL-Vorschlag für die neuen
Autoren, Live-Beweis-Kommando.
