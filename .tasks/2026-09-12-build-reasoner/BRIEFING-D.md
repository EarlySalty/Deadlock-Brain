# Briefing: build-reasoner (Paket D, Integration, CLI, Backtest)

[Orchestrator] Paket D. Auftrag:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
(vollständig lesen), Spec `ARCHITEKTUR.md` (Abschnitte 2, 3, 4, 9, 10, 11,
12) und `MECHANIK.md`, Paketstand in `PAKETE.md` (Befunde 6 bis 10),
Fertigmeldungen `FERTIG-A.md`, `FERTIG-B.md`, `FERTIG-C.md`, `FERTIG-S.md`,
Reviews `REVIEW-A.md`, `REVIEW-B.md`, `REVIEW-C.md` (jeweils die Punkte
"für D"), Vorcheck `VORCHECK-ERGEBNIS.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-d` (existiert, Branch
  `feat/build-reasoner-d` ist ausgecheckt und enthält A, B und C bereits
  zusammengeführt; der Delegator hat die `mod`-Zeilen in `lib.rs`
  nachgetragen und `cargo check` bestanden, oder er nennt dir im Startbefehl
  den offenen Rest)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du baust

1. Fassade in `rust/crates/dbrain-reasoner/src/lib.rs`: `reason_build`,
   `reason_patch_impact`, `reason_backtest` nach ARCHITEKTUR.md Abschnitt 9
   (Reihenfolge, Übergabeobjekte, Abbruchregeln, `use_ai`-Schalter, genau
   eine Recompose-Runde). Die Item-Analyst-Regel "fremde Item-Texte
   verwerfen" sitzt beim Zusammenführen der Warum-Texte (siehe REVIEW-A.md
   Mangel 6).
2. CLI in `rust/crates/deadlock-brain/src/main.rs`: Gruppe `reason` mit
   `build <hero> [--no-ai] [--patch <tag>] [--publish] [--json]`,
   `patch-impact <hero> [--patch <tag>] [--json]`, `backtest [--hero <hero>]
   [--patch <tag>] [--json]` nach dem Muster der bestehenden Gruppen
   (`Wiki`, `Learn`). Seed-Pfad für Referenz-Builds konfigurierbar, Default
   `.tasks/2026-09-12-build-reasoner/referenz/`. Ohne `--publish` wird kein
   Schreibpfad erreicht.
3. `rust/crates/dbrain-builds/src/sync.rs`: `patch_tag` bekommt beim Sync
   einen echten Wert (Datum oder `patch_external_id` des jüngsten Patches aus
   `brain.patch_events`) statt `current`; Historie je Patch-Stand bleibt
   erhalten (Upsert auf `(hero_id, item_id, bracket, patch_tag)`), keine
   bestehende Zeile wird gelöscht. Dazu ein systemd-User-Timer
   `deadlock-brain-build-data.timer` (täglich, alle Helden seriell mit dem
   bestehenden `analytics_delay_ms`) nach dem Muster der vorhandenen
   Brain-Timer, Skript unter `scripts/`, Secrets über den bestehenden
   Infisical-Weg.
4. Neue Tabellen aus ARCHITEKTUR.md Abschnitt 4, sofern A, B oder C sie
   nutzen: Migration als SQL-Datei im Repo, Anwendung auf Prod macht der
   Delegator; ohne Nutzung keine Tabelle.
5. Fireworks-Modell: das einkompilierte `deepseek-v4-flash` liefert 404.
   Baue in `deadlock-brain-core` eine Auflösung wie im Twitch-Bot
   (`tb-llm/src/model_resolver.rs`): bei 404 einmal `GET
   {base_url}/models` abfragen, neueste Fassung der Familie `deepseek-v4-flash`
   wählen, Ergebnis im Prozess merken; Rangfolge Env `FIREWORKS_MODEL`, dann
   Resolver, dann Kompilat-Default. Kein anderes Modell, kein anderer Anbieter.
6. Backtest echt fahren: `reason backtest --hero Warden` gegen den Seed
   `referenz/lightbringer-warden.json` und gegen die Autoren-Builds in
   `tierlist.hero_build_sources` (nach dem S-Deploy liegt der Build 779996
   dort), dazu `reason build Warden --no-ai --json` und einmal mit KI. Alle
   Zahlen (Kern-Überdeckung, Jaccard, Reihenfolge-Nähe, Patch-Wechsel, Top-10
   Items mit Scores, Konfidenz je Item) in `REPORT-D.md` im Task-Ordner. Ist
   die Kern-Überdeckung unter 0,5, ist das kein Grund, Gewichte zu drehen:
   Ursache je fehlendem Kern-Item benennen (welche Regel drückt es), Vorschlag
   in den Report, keine stillen Anpassungen an B oder C.
7. Doku `docs/BUILD_REASONER.md`: Zweck, Datenfluss, Befehle, Backtest-Lesart,
   Grenzen (aus MECHANIK.md Abschnitt 16), kurz.

## Regeln

- Nur die Dateien dieses Pakets plus `lib.rs`. Änderungen an B- und
  C-Dateien nur, wenn die Integration sonst nicht kompiliert; jede solche
  Änderung in der Fertigmeldung mit Zeile und Grund.
- Keine Code-Kommentare. Kein Python. Kein neues LLM.
- Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis
  `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy --workspace
  --all-targets -- -D warnings` (vorbestehende Warnungen als Baseline
  melden), `cargo test --workspace` ohne DSN und `cargo test -p
  dbrain-reasoner -- --include-ignored` mit `DEADLOCK_CENTRAL_DSN` (Zugang
  wie in `scripts/`, Secrets aus Infisical, nie ausgeben, nie in Dateien;
  Verbindung read-only außer beim Publish-Test gegen eine Wegwerf-DB). Höchstens
  ein Release-Build auf dem Host; den Release-Build für den Backtest darfst
  du einmal fahren.
- Nur `feat/build-reasoner-d` committen und pushen, nie main. Commit-Trailer:
  `Co-authored-by: GPT 5.6 Luna <luna@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Bump-up

```
[Bump-up] Paket D: Grund: ... Erledigt: ... Worktree: /home/nathanael/.worktrees/deadlock-brain-d Offen: ...
```

## Fertigmeldung

In diesem Thread und als `FERTIG-D.md`: Branch, Commit-SHAs, geänderte
Dateien, Testläufe mit Zahlen (Baseline und Endstand, ohne und mit DSN),
Pfad zu `REPORT-D.md`, Migrationen und Timer als Deploy-Voraussetzungen,
offene Punkte.
