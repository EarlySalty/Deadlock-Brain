# FERTIG-FIX-M1: Fixrunde 1 Paket M

Branch `feat/build-reasoner-population`, neue Commits auf 23e0935, kein amend.

## Commits

- `73e26b9` fix(population): P-Nits aus REVIEW-P abarbeiten
- `09198ff` feat(reasoner): Populations-Prior produktiv laden und im Backtest ausweisen
- (dieser Commit) chore(m): Timer-Schritte, Nachweise verschoben, Pfade, Fertigmeldung

## Punkte

1. **Produktiv-Verdrahtung.** Erledigt. `load_reasoning_inputs` und der
   Backtest-Pfad laden die Population über `dbrain_population::PopulationIndex::load`
   aus `ctx.pool` (DEADLOCK_CENTRAL_DSN, kein zweiter Env-Name), Helper
   `load_population_prior` mit `to_regclass`-Guard (fehlende Tabellen oder leerer
   Held liefern einen leeren Prior). `MetaIndexWithSources.population` gesetzt.
   Fehlt die Population, trägt der Build sichtbar "Population: keine Daten"
   (composer.rs), die Confidence bleibt unverändert (leerer Prior wirkt nicht).
   `reason backtest` weist je Held Staple-Gate, Kendall tau und Jaccard@12 aus
   (null ohne Population), im CLI-Text und im gespeicherten `reasoner_backtests.detail`
   (jsonb, keine neue Migration nötig). `examples/build_evaluation.rs`:
   `POPULATION_DB_DSN` entfernt, denselben zentralen Pool genutzt; der Freeze-Replay
   nutzt den echten je-Held-Prior statt `default()`.
2. **Timer.** Erledigt. Startskript `scripts/run_build_data_with_infisical.sh`
   bekommt nach `pull build-data` die zwei Schritte `population sync --matches 2000`
   und `population stats`, beide nicht-fatal (`|| echo ... >&2`), sodass ein Fehler
   den bestehenden Sync nicht rückwirkend als gescheitert markiert, aber im Journal
   sichtbar bleibt. Keine EnvironmentFile-Änderung. Die Timer-Unit selbst
   (`~/.config/systemd/user/deadlock-brain-build-data.timer`, Service `.service`)
   triggert nur und bleibt unverändert.
3. **Nachweise ins Repo.** Erledigt. Acht Messdateien (alle unter 5 MB) von
   `Documents/.tasks/2026-09-16-population-baseline-mess/` nach
   `.tasks/2026-09-16-population-baseline/nachweise/` verschoben und committet.
   Pfadangaben in M-DIAGNOSE.md, M-MESSUNG.md, FERTIG-M.md nachgezogen.
4. **P-Nits.** Erledigt. `population stats` mit `assert_writable`; `WIN_WEIGHT`
   auf den Python-Default 1.0 (`sequence.row_weights`, Sieg-Gewichtung dort aus);
   `--rebuild` entfernt (stats rechnet ohnehin immer neu); Test für den
   `--hero`-Filterpfad; `average_badge`-Filter wie Python (nur fehlende Werte
   neutral, exakt 0 bleibt echter Wert), mit Test.
5. **Umlaute.** Ab dieser Runde echte Umlaute in Commit-Betreffzeilen und Texten.

## Tests

- `cargo test -p dbrain-population`: 24 bestanden (vorher 22, plus zwei neue Tests),
  0 ignoriert, 0 rot.
- `cargo test -p dbrain-reasoner --lib`: 170 bestanden, 16 ignoriert, 0 rot.
- Workspace ohne DSN: 386 bestanden, 58 ignoriert, 0 rot (Baseline 384, plus die
  zwei neuen Population-Tests).
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: sauber. Kein `--release`.

## Grenzen

- Ende-zu-Ende gegen die zentrale DB ist hier nicht möglich: die Population liegt
  bisher nur in der lokalen `population_dev`, die zentrale DB bekommt die Migration
  erst beim Deploy. Der Wirkungsnachweis der Verdrahtung läuft über die
  Reasoner-Lib-Tests und das Mess-Example gegen `population_dev`.

## Pfade

- Timer-Unit: `~/.config/systemd/user/deadlock-brain-build-data.timer` (Service
  `deadlock-brain-build-data.service`).
- Startskript: `scripts/run_build_data_with_infisical.sh`.
