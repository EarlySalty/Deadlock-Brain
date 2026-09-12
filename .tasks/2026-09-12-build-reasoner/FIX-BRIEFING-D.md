# Fix-Briefing: build-reasoner (Paket D, Fixrunde 1)

[Orchestrator] Fixer für Paket D nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-D.md`
(vollständig lesen). Spec `ARCHITEKTUR.md` Abschnitte 4, 9, 11, 12,
Worker-Briefing `BRIEFING-D.md`, Report und Fertigmeldung im Worktree unter
`.tasks/2026-09-12-build-reasoner/REPORT-D.md` und `FERTIG-D.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-d` (Branch
  `feat/build-reasoner-d`, Commit 7fbb128, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du tust

1. Mangel 1 (blockierend), Entscheidung des Delegators: der spec-konforme
   Weg. Die Fassade persistiert: `reason build` schreibt `reasoner_builds`
   und je Item `reasoner_item_scores` mit allen Score-Komponenten aus
   `ScoredItem`; `reason patch-impact` schreibt `reasoner_patch_deltas`;
   `reason backtest` schreibt `reasoner_backtests`. Schreiben nur mit dem
   Central-Pool, idempotent je Held und Patch-Tag (Upsert, keine Löschung),
   Tests gegen eine Wegwerf-DB, wo vorhanden (`REASONER_SCRATCH_DSN`), sonst
   gegen Fixtures des SQL-Textes. Das Fertig-Kriterium "reproduzierbar ohne
   KI-Aufruf" ist damit über `reasoner_item_scores` belegt.
2. Mangel 2 (wichtig): eine gemeinsame Funktion für die Patch-Tag-Auflösung,
   die Sync (`dbrain-builds/src/sync.rs`) und Fassade (`dbrain-reasoner/src/lib.rs`)
   teilen; Ort so, dass keine Zirkelabhängigkeit entsteht (Vorschlag:
   `dbrain-builds`, da der Reasoner davon schon abhängt). Test, dass beide
   denselben Tag liefern.
3. Mangel 3 (nit): Familien-Revision im Modell-Resolver numerisch statt
   lexikografisch wählen, Test mit 0731 gegen 1015 und ähnlichen Fällen.
4. Mangel 4 (nit): Fallback-Skill-Order behält Punkt-Typ und Delta.
5. Mangel 5 (nit): Migration und Spec-DDL bei `order_proximity` angleichen
   (nullable, weil `Option<f64>`).

Kalibrierungspunkte aus REVIEW-D.md (Core-Cutoff, Zustandsfaktor, Lane-Phase)
sind nicht dein Thema.

## Regeln

- Dateien: `dbrain-reasoner/src/lib.rs`, `dbrain-builds/src/sync.rs` und eine
  neue gemeinsame Stelle für den Tag, `deadlock-brain-core/src/model_resolver.rs`,
  `scripts/migrations/2026-09-12-reasoner.sql`, Tests. `types.rs` nur, wenn
  die Persistenz ein Feld braucht, dann additiv und ausgewiesen.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy -p
  dbrain-reasoner -p dbrain-builds -p deadlock-brain-core -p deadlock-brain
  --all-targets -- -D warnings`, `cargo test --workspace` ohne DSN
  (Baseline: grün) und `cargo test -p dbrain-reasoner -- --include-ignored`
  mit `DEADLOCK_CENTRAL_DSN` (Baseline 70 bestanden, 5 Scratch-Tests ohne
  `REASONER_SCRATCH_DSN` rot). Zugang wie in `scripts/`, Secrets aus Infisical,
  nie ausgeben. Kein `--release`. Keine Schreibzugriffe auf den Central-Pool
  aus Tests.
- Selbstprüfung vor der Fertigmeldung. Nur `feat/build-reasoner-d` committen
  und pushen, nie main. Neue Commits obendrauf. Commit-Trailer
  `Co-authored-by: <dein Modell> <modell@local>`. Echte Umlaute, keine
  Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 1" in `REVIEW-D.md` (Hauptordner):
je Mangel Datei:Zeile, Änderung, Commit-SHA, Testzahlen (Baseline und
Endstand, ohne und mit DSN).
