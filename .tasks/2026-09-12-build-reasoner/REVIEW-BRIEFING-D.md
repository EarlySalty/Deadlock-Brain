# Review-Briefing: build-reasoner (Paket D, Runde 1)

[Orchestrator] Review Runde 1 für Paket D (Integration, CLI, Backtest).
Lesend, kein Code, kein Branch. Du bist der einzige Thread für dieses Review.
Keine Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-d` (Branch
  `feat/build-reasoner-d`, Commits 5f438d2 und 7fbb128, Basis cbfbc98 = main
  plus B mit A plus C)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-d diff cbfbc98..HEAD`
- Auftrag `AUFTRAG.md`, Spec `ARCHITEKTUR.md` (Abschnitte 4, 9, 11, 12),
  Worker-Briefing `BRIEFING-D.md`, Fertigmeldung und Report im Worktree unter
  `.tasks/2026-09-12-build-reasoner/FERTIG-D.md` und `REPORT-D.md`, dazu die
  "für D"-Abschnitte in `REVIEW-A.md`, `REVIEW-B.md`, `REVIEW-C.md` (jeweils
  Runde 2) im Hauptordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Fassade in `dbrain-reasoner/src/lib.rs`: Reihenfolge nach ARCHITEKTUR.md
   Abschnitt 9 (Laden, Patch-Delta vor dem Scoring, Scoring, Composer, KI nur
   bei `use_ai`, Kritiker mit genau einer Recompose-Runde), Item-Analyst-Filter
   (fremde Item-Texte verwerfen), `compose_build_with_sources` verdrahtet
   (sonst bleibt `ability_order` leer, REVIEW-C Runde 2), `--no-ai` erreicht
   keinen Modellaufruf, `--publish` ist der einzige Schreibpfad.
2. CLI in `deadlock-brain/src/main.rs`: `reason build|patch-impact|backtest`
   mit den Optionen aus Abschnitt 12, Seed-Pfad konfigurierbar, JSON-Ausgabe
   stabil, Fehlerfälle (unbekannter Held, keine Daten) sauber statt Panic.
3. `dbrain-builds/src/sync.rs`: echter `patch_tag` aus `brain.patch_events`,
   Upsert auf `(hero_id, item_id, bracket, patch_tag)`, keine Löschung alter
   Zeilen; prüfe, ob die Ladeschicht (`data.rs`) und der Meta-Index denselben
   Tag-Wert erzeugen wie der Sync, sonst liest der Reasoner ins Leere. Das ist
   laut REPORT-D genau der Grund für `meta_support = 0`: bestätige oder
   widerlege das am Code und an der DB (Zugang wie in `scripts/`, Secrets aus
   Infisical, nie ausgeben, read-only).
4. `deadlock-brain-core`: Modell-Resolver (`model_resolver.rs`) und der
   404-Retry in `ai.rs`: Rangfolge Env, Resolver, Kompilat-Default; nur die
   Familie `deepseek-v4-flash`; kein anderes Modell, kein anderer Anbieter;
   Worker-Thread-Umbau bricht keine bestehenden Aufrufer (enrich, learn,
   player). Ein echter Aufruf mit Key aus Infisical ist erlaubt, wenn er
   klein ist; nenne, welches Modell der Resolver auflöst.
5. Migration `scripts/migrations/2026-09-12-reasoner.sql`: nur Tabellen, die
   der Code wirklich nutzt (ARCHITEKTUR Abschnitt 4), Rechte für die
   Dienstrollen bedacht, idempotent, keine bestehende Tabelle verändert.
   Timer und Skript `run_build_data_with_infisical.sh`: Muster der
   vorhandenen Brain-Timer, `pull build-data --hero all` existiert im CLI,
   Secrets nur über den bestehenden Infisical-Weg, kein EnvironmentFile.
6. Backtest und Report: Kern-Überdeckung 0,158 gegen den
   Lightbringer-Seed, Reihenfolge-Nähe 0,435, Patch-Wechsel nicht messbar.
   Prüfe die Ursachentabelle in REPORT-D.md: sind "Kaufphase Lane schließt
   vom Core aus", "Zustandsfaktor 0,6 auf Imbue-Items" und "Core-Cutoff bei
   19" Regelentscheidungen aus MECHANIK.md oder Implementierungsartefakte von
   B und C? Nenne je Ursache, ob sie ein Mangel (falsch gegen Spec) oder eine
   Kalibrierungsfrage (Spec erfüllt, Ergebnis schwach) ist. Ein schwaches
   Backtest-Ergebnis allein ist kein Blocker, ein Spec-Verstoß schon.
   Vergleiche zusätzlich gegen Build 779996 (jetzt in
   `tierlist.hero_build_sources`, Version 45, 5 Kategorien), das zur Zeit des
   D-Laufs noch fehlte.
7. Hygiene: keine Secrets, keine Code-Kommentare, keine neuen externen Crates
   ohne Grund, sqlx-Offline-Cache passt; `cargo test --workspace` (D: grün),
   `cargo clippy --workspace --all-targets -- -D warnings` (vorbestehende
   Warnungen als Baseline), `cargo test -p dbrain-reasoner -- --include-ignored`
   mit Central-DSN (D: 70 bestanden, 5 Scratch-Tests brauchen
   `REASONER_SCRATCH_DSN`; beurteile, ob diese fünf Tests so bleiben dürfen).

## Ergebnis

Datei `REVIEW-D.md` im Hauptordner: Mängelliste mit Nummer, Datei:Zeile,
Befund, Schwere (blockierend, wichtig, nit), Vorschlag; getrennt davon eine
Liste "Kalibrierung" für Backtest-Schwächen ohne Spec-Verstoß. Urteil:
FREIGABE oder NACHBESSERN, plus Deploy-Reihenfolge (Migration, Release-Build,
Timer, erster Sync, Backtest-Wiederholung). Fertigmeldung in diesem Thread
mit Pfad und Urteil. Deutsch, echte Umlaute, keine Gedankenstriche.
