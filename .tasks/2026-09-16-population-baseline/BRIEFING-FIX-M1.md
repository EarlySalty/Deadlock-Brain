# Briefing Fixrunde 1 (Paket M, bekannte offene Punkte)

Branch `feat/build-reasoner-population`, Worktree `~/.worktrees/deadlock-brain-m`,
Spitze 23e0935. Neue Commits obendrauf, kein `--amend`, kein Rebase. Du bist
der einzige Worker dieser Runde. Parallel läuft ein Reviewer lesend auf dem
Stand 3ff5db4; seine Liste (`REVIEW-M.md`) kommt in Runde 2, nicht hier.

## Punkte

1. **Produktiv-Verdrahtung.** `reason_build_with_options` und der Backtest-Pfad
   in `dbrain-reasoner/src/lib.rs` laden die Population über
   `dbrain_population::PopulationIndex::load(pool, hero_id)` aus demselben Pool
   wie alles andere (zentrale DB, `DEADLOCK_CENTRAL_DSN`, kein zweiter
   Env-Name) und setzen `MetaIndexWithSources.population`. Fehlen die Tabellen
   oder sind sie für den Helden leer, läuft das Build ohne Prior weiter, sagt
   das aber sichtbar (Evidence-Zeile "Population: keine Daten", Confidence
   nicht höher als ohne). `deadlock-brain reason backtest` weist je Held
   Staple-Gate, Kendall tau und Jaccard@12 aus (`null` ohne Population), auch
   in der gespeicherten `brain.reasoner_backtests`-Zeile, sofern die Spalten
   das hergeben; sonst neue Migration `scripts/migrations/2026-09-16-population-backtest.sql`.
   `examples/build_evaluation.rs`: `POPULATION_DB_DSN` entfernen, denselben
   Pool nutzen.
2. **Timer.** Der tägliche Oneshot hinter `deadlock-brain-build-data.timer`
   (Unit und Startskript per Graphify/Akte `2026-09-12-build-reasoner`
   finden; Paket D hat ihn angelegt) bekommt nach dem bestehenden Sync zwei
   Schritte: `deadlock-brain population sync --matches 2000` (inkrementell,
   idempotent) und `deadlock-brain population stats`. Fehler dieser Schritte
   dürfen den bestehenden Sync nicht rückwirkend als gescheitert markieren,
   müssen aber im Journal sichtbar sein. Keine EnvironmentFile-Änderung, das
   DSN kommt wie bisher.
3. **Nachweise ins Repo.** Die Messdateien aus
   `/home/nathanael/Documents/.tasks/2026-09-16-population-baseline-mess/`
   nach `.tasks/2026-09-16-population-baseline/nachweise/` im Worktree
   verschieben und committen (Dateien über 5 MB nicht committen, sondern in
   `nachweise/README.md` mit Pfad und SHA256 nennen). Pfadangaben in
   M-DIAGNOSE.md, M-MESSUNG.md, FERTIG-M.md nachziehen.
4. **P-Nits aus `REVIEW-P.md`:** `population stats` prüft vor dem Schreiben
   `assert_writable`; `WIN_WEIGHT` auf den Wert aus dem Python-Original
   setzen (in `/tmp/dbm-eval/src/deadlock/` nachlesen, `dataset.py` oder
   `sequence.py`, Gewicht für Sieg) oder, wenn Python keins hat, auf 1,0 und
   die Konstante benennen; `--rebuild` entweder echt (Aggregate löschen und
   neu) oder Flag entfernen; Test für den `--hero`-Filterpfad; `average_badge`-
   Filter wie Python (nur fehlende Werte neutral, nicht `> 0`).
5. **Umlaute.** Ab jetzt echte Umlaute auch in Commit-Betreffzeilen. Bereits
   committete Betreffzeilen bleiben.

## Prüfung vor der Fertigmeldung

- `export PATH=/home/nathanael/.cargo/bin:$PATH`; in `rust/`: `cargo test -p
  dbrain-reasoner --lib`, `cargo test -p dbrain-population`, Workspace-Tests
  ohne DSN (Baseline 384 bestanden, 58 ignoriert, nichts brechen), `cargo
  clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`. Kein `--release`. `set -o pipefail`.
- Wirkungsprobe der Verdrahtung: Unit-Test, der `reason_build_with_options`
  mit gesetzter Population den Prior im Planner sehen lässt, und ein Test
  ohne Population mit der sichtbaren Evidence-Zeile. Die Population liegt
  bisher nur in der lokalen `population_dev` (zentrale DB bekommt die
  Migration erst beim Deploy), ein Ende-zu-Ende-Lauf gegen die zentrale DB
  ist deshalb hier nicht möglich; das steht so in der Fertigmeldung.
- Keine Code-Kommentare, keine Modellnamen, keine Gedankenstriche, echte
  Umlaute. Nur den eigenen Branch committen und pushen (`git push origin
  feat/build-reasoner-population`), nie main. Fremde Worktrees nicht anfassen.
- Fertigmeldung `FERTIG-FIX-M1.md` in der Akte: Commits, je Punkt was
  gemacht, Testzahlen, offene Punkte.
