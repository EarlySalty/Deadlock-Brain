# Briefing Paket M: Warden auf mindestens 5/9 und Messlatte

Auftrag: `AUFTRAG.md` in diesem Ordner. Audit der Welle 2: `WELLE2-AUDIT.md`.
Du bist der einzige Worker für dieses Paket. Keine Unter-Agenten spawnen.

## Ausgangslage

- Welle 2 (Combat, Planner, Interaktionen) liegt ungemergt auf
  `feat/build-reasoner-interactions-audit` (fc71b5d), grün (Reasoner-Lib 167,
  Workspace 359 bestanden, Clippy sauber), Warden aber nur 3/9 Referenzwaffen
  (Recall 0,316, Jaccard 0,207). Das Merge-Gate hat den Fast-Forward nach main
  am 2026-09-16 blockiert, weil die Welle ihre eigene Abnahme (5/9) verfehlt.
  Gemergt wird deshalb erst der Stand, der 5/9 ehrlich schafft.
- Gate-NITs, die du mit erledigst: `dbrain-reasoner/build.rs:12` erzwingt den
  Rerun über einen nicht existierenden Pfad (sauber lösen oder begründet
  lassen); `combat.rs` `simulate()` verschattet den Parameter `bindings`
  durch `let mut bindings: Vec<Binding>` (umbenennen).
- Referenz: Warden-Build 779996 Version 45 (Lightbringer), 19 Kernitems, davon
  9 Waffen; Seed `.tasks/2026-09-12-build-reasoner/referenz/lightbringer-warden.json`.
  Der Maßstab steht in `.tasks/2026-09-12-build-reasoner/ABNAHME-FINAL.md`:
  mindestens 5 von 9 Referenzwaffen, keine Referenz-Itemnamen und keine
  heldenspezifischen Gewichte im Produktcode, keine Seed-Kopie, kein Absenken
  des Maßstabs.
- Bekannte fachliche Lücken (MESSUNG-ZWISCHENSTAND.md, Welle 2): Glass Cannon
  wird gewählt, obwohl `MaxHealthLossPercent` nicht quantifiziert ist;
  Lightning Scroll hat unbekannten Damage-/Triggernutzen; Opening Rounds hat
  eine nicht berechnete `EnemyLifeThreshold`-Bedingung; sechs Endinventare
  weisen null Proc-Schaden aus; Infernus und Bebop verschlechtern sich.

## Arbeitsort

- Worktree: `git -C /home/nathanael/repos/Deadlock-Brain worktree add
  /home/nathanael/.worktrees/deadlock-brain-m -b feat/build-reasoner-population fc71b5d`
- Sobald Paket P fertig ist (Datei `FERTIG-P.md` in diesem Ordner, Branch
  `feat/population-baseline` auf origin), diesen Branch in deinen Branch mergen
  (`git merge origin/feat/population-baseline`) und die Bibliotheks-API
  `dbrain_population::PopulationIndex` nutzen. Bis dahin Phase 1 ohne P.
- Zentrale DB nur lesend. DSN in der eigenen Shell holen, nie ausgeben:
  `eval "$(python3 /home/nathanael/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"`
  und `export PGOPTIONS='-c default_transaction_read_only=on'`; die CLI mit
  `--no-persist`. Kein Publish, keine KI-Aufrufe, kein Steam-Upload.

## Phase 1: Diagnose (vor P)

1. Warden auf fc71b5d nachmessen (Mess-Example oder `deadlock-brain reason
   build --hero Warden --no-persist` aus dem Worktree, Debug-Build, kein
   `--release`). Zahlen: Waffen x/9, Kern x/19, Recall, Jaccard, Laufzeit.
2. Je fehlender Referenzwaffe die Ursache belegen: welchen Score bekommt das
   Item im Planner, welches Item wird stattdessen gekauft, welche Mechanik
   fehlt oder ist falsch bewertet (Feld im Snapshot, Bedingung, Proc, Tickrate,
   Kaufbonus, Kostenband). Ergebnis als Tabelle in `M-DIAGNOSE.md`.
3. Daraus die generischen Mechanik-Fixes ableiten (kein Warden-Sonderfall):
   fehlende Felder quantifizieren, bedingte Items ehrlich abwerten, Proc-
   Schaden in der Simulation belegen, Kostenband-Kaufkurve prüfen.

## Phase 2: Fixes und Population als Signal (mit P)

4. Mechanik-Fixes aus Phase 1 umsetzen, jeder mit Test und Vorher/Nachher-Zahl.
5. Messlatte in `dbrain-reasoner/src/backtest.rs`: Staple-Gate (jedes Item ab
   70 % Kaufanteil der Population im Bucket des Builds muss im Build sein),
   Kendall tau der Build-Reihenfolge gegen die Median-Kaufposition, Jaccard@12
   gegen die Population; im Backtest-Report je Held ausweisen, `null` wenn die
   Population den Helden nicht trägt.
6. Population als Signal im Composer/Planner: ein generischer, helden-
   unabhängiger Prior aus Kaufanteil und Median-Position (z. B. als
   Meta-Stütze neben Mechanik-Score), dessen Gewicht als eine benannte
   Konstante mit Begründung im Code steht. Kein Item darf allein wegen
   Prevalence in den Kern, wenn die Mechanik ihn negativ bewertet; die
   Begründung je Item nennt beide Anteile.
7. Warden-Build erzeugen und messen: Ziel mindestens 5/9 Referenzwaffen und
   Staple-Gate bestanden. Dazu Sechs-Helden-Vergleich (Infernus, Bebop, Ivy,
   Lady Geist, Vindicta, Abrams) vorher/nachher gegen Autoren und Population,
   damit der Fortschritt nicht nur Warden ist. Zahlen in `M-MESSUNG.md`.
8. Publish-Payload lesend prüfen (bestehender Weg `publish.rs`), kein Upload.

## Regeln

- Kein Python, keine Code-Kommentare, echte Umlaute, keine Gedankenstriche.
- `export PATH=/home/nathanael/.cargo/bin:$PATH`; in `rust/`: `cargo fmt` nur
  eigene Dateien, `cargo clippy -p dbrain-reasoner -p dbrain-population
  --all-targets -- -D warnings`, `cargo test -p dbrain-reasoner --lib`,
  Workspace-Tests ohne DSN (Baseline auf fc71b5d: 359 bestanden). Kein
  `--release`. `set -o pipefail` bei Pipes hinter cargo.
- Keine Referenz-Itemnamen und keine Warden-Sondergewichte im Produktcode;
  Warden ist Regressionsfall in Tests und Messung, nicht im Modell.
- Fremde Worktrees nicht anfassen. Nur `feat/build-reasoner-population`
  committen (kleine Commits, Trailer `Co-Authored-By: Claude Opus 4.8
  <noreply@anthropic.com>`) und mit `git push -u origin
  feat/build-reasoner-population` pushen. Nie main.
- Fertigmeldung `FERTIG-M.md`: Commits, Dateien, Testzahlen vorher/nachher,
  Warden und Sechs-Helden-Tabelle, offene Punkte. Nicht Baubares als
  `ABWEICHUNG:` melden und den Rest fertig bauen.
