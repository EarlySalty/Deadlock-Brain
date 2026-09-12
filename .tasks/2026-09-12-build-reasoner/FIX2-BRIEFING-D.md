# Fix-Briefing: build-reasoner (Paket D, Fixrunde 2, Merge-Kritiker)

[Orchestrator] Fixer für Paket D nach dem Merge-Kritiker. Paket D ist nach
Review Runde 2 freigegeben; beim Merge nach main hat der Kritiker vier Funde
gemeldet, davon einer blockierend.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-d` (Branch
  `feat/build-reasoner-d`, Commit 54252b0, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Befunde des Merge-Kritikers, wörtlich

1. `docs/BUILD_REASONER.md:29` | BLOCKING | „Ohne `--publish` wird keine
   Schreiboperation durch den Reasoner ausgelöst." ist nach dem Fix falsch:
   `reason_build_with_seed_path`, `reason_patch_impact` und
   `reason_backtest_with_seed_path` rufen am Ende bedingungslos
   `persist_build`/`persist_patch_impact`/`persist_backtest` mit
   `ctx.pool.begin()` + INSERT auf (lib.rs:99,166,216,249,288,312). Ein
   Operator, der laut Doku `reason build Warden --no-ai --json` für
   nebenwirkungsfrei hält, schreibt in die Produktions-Central-DB. Fix: Satz
   auf „schreibt Build-, Score-, Delta- und Backtest-Zeilen in
   `brain.reasoner_*`; nur der Steam-Publish hängt an `--publish`" ändern.
2. `rust/crates/dbrain-reasoner/src/backtest.rs` (backtest_hero) | NIT | Die
   öffentliche `async fn backtest_hero` gibt bei nicht-leeren Autoren
   `Err(Data("... benötigt ScoredItem-Bestand"))` zurück und wird nirgends
   aufgerufen (lib nutzt `backtest_hero_with_build`). Toter, garantiert
   fehlschlagender Pfad. Entfernen oder implementieren.
3. `rust/crates/dbrain-reasoner/src/lib.rs` (persist_build) | NIT | Die
   Fassade schreibt jetzt immer; gegen eine read-only Central-Verbindung
   schlägt `reason build` mit `Db(...)` fehl statt einen Build zu liefern.
   Bewusst? Dann in Doku und Deploy-Note festhalten.
4. `scripts/run_build_data_with_infisical.sh:5` | NIT | Fallback-Pfade
   (`/home/naniadm/.config/...`, Token-Datei) und die systemd
   `WorkingDirectory=/home/naniadm/Documents/Deadlock-Brain` sind fest
   verdrahtet. Bricht still, wenn Deploy-User oder Pfad abweichen.

## Entscheidungen des Delegators

1. Doku wie vom Kritiker vorgeschlagen korrigieren; zusätzlich in
   `docs/BUILD_REASONER.md` einen kurzen Abschnitt "Was der Reasoner
   schreibt" mit den vier Tabellen und der Regel, dass nur `--publish` den
   Steam-Bot erreicht.
2. `backtest_hero` entfernen (kein Aufrufer, kein Nutzen).
3. Persistenz bleibt immer an. Ergänze einen Schalter `--no-persist` an
   allen drei `reason`-Befehlen für Trockenläufe und read-only Verbindungen;
   ohne Schalter schreibt die Fassade wie jetzt. Doku und CLI-Hilfe nennen
   ihn.
4. Pfade: Der Dienst läuft als Nutzer `nathanael` mit User-Units unter
   `~/.config/systemd/user/`; das Repo liegt unter
   `/home/nathanael/repos/Deadlock-Brain` (der Pfad
   `/home/nathanael/Documents/Deadlock-Brain` ist ein Symlink darauf). Nimm
   die bestehenden Brain-Timer als Muster (`deadlock-brain-sheet-sync.service`
   in `~/.config/systemd/user/`, dazu ihr Drop-in unter `.service.d/`), nutze
   `%h` statt fester Home-Pfade, dieselbe Infisical-Konfiguration wie dort
   und keinen `naniadm`-Pfad. Prüfe die Unit mit `systemd-analyze --user
   verify`.

## Regeln

- Dateien: `docs/BUILD_REASONER.md`, `dbrain-reasoner/src/backtest.rs`,
  `dbrain-reasoner/src/lib.rs`, `deadlock-brain/src/main.rs`,
  `scripts/run_build_data_with_infisical.sh`,
  `service/systemd/deadlock-brain-build-data.*`, Tests.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy -p
  dbrain-reasoner -p deadlock-brain --all-targets -- -D warnings`, `cargo
  test --workspace` ohne DSN (Baseline 256 bestanden, 53 ignoriert). Kein
  `--release`. Keine Schreibzugriffe auf den Central-Pool.
- Neue Commits obendrauf, kein `--amend`. Nur `feat/build-reasoner-d` pushen,
  nie main. Commit-Trailer `Co-authored-by: <dein Modell> <modell@local>`.
  Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 2 (Merge-Kritiker)" in
`REVIEW-D.md` (Hauptordner): je Fund Datei:Zeile, Änderung, Commit-SHA,
Testzahlen.
