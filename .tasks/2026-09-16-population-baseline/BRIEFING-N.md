# Briefing Paket N: Planner und Population in den Produktivpfad

Stand 2026-09-16, main 7277ff5 (Welle 2 plus P plus M gemergt, Release
`57d3a07a2c87a657` gebaut, Migration und 10k-Ingest auf der zentralen DB durch:
118572 Spieler-Matches, 38 Helden, Warden 10 Staples).

## Befund

Der Live-Lauf `deadlock-brain reason build Warden --json` mit dem Release-Binary
gegen die zentrale DB liefert einen Kern aus Extra Health, Rapid Rounds, Swift
Striker, Fleetfoot, Duration Extender, Restorative Shot, Enduring Speed,
Enchanter's Emblem, Bullet Resist Shredder, Superior Duration, Rebuttal, Headshot
Booster, Diviner's Kevlar, Ethereal Shift, Crushing Fists, Witchmail: nur 4
Treffer im Lightbringer-Seed, keine der sechs großen Staples (Quicksilver Reload,
Mercurial Magnum, Opening Rounds, Titanic Magazine, High-Velocity Rounds,
Spiritual Overflow). `reason backtest --hero Warden` meldet Population
Staple-Gate nicht bestanden, tau 0,58, Jaccard@12 0,50.

Die 6/9-Messung aus M lief über `examples/build_evaluation.rs` mit
`planner::plan_with_economy`, Progression, Combat-Simulation und
Populations-Prior. Der Produktivpfad `reason_build_with_options` in
`dbrain-reasoner/src/lib.rs` nutzt offenbar weiter den Welle-1-Composer; Welle
2 hat laut `WELLE2-AUDIT.md` keinen Produktivpfad angefasst. Hypothese zuerst
am Code belegen (Graphify, dann lesen), dann bauen.

## Ziel

`deadlock-brain reason build <held>` erzeugt dasselbe Build wie das
Mess-Example: Planner mit Kaufkurve, Progression, Combat-Bewertung und
Populations-Prior, aus den Live-Snapshots der zentralen DB. `reason backtest`
und `reason patch-impact` arbeiten auf demselben Build-Objekt. Publish-Payload
(`publish.rs`) bleibt kompatibel. `--no-ai`, `--no-persist`, `--json`,
`--publish` funktionieren weiter.

## Arbeit

1. Worktree `git -C /home/nathanael/repos/Deadlock-Brain worktree add
   /home/nathanael/.worktrees/deadlock-brain-n -b feat/reasoner-planner-produktiv main`.
2. Belegen, welcher Pfad `reason build` heute nimmt und was das Example anders
   macht (Eingaben, Snapshot-Auswahl, FROZEN-V2 gegen Live-DB, Population
   über `load_population_prior`). Kurz in `N-BEFUND.md` festhalten.
3. Produktivpfad auf die Planner-Pipeline umstellen, ohne Doppelcode: das
   Example ruft danach dieselbe Bibliotheksfunktion wie die CLI. Laufzeit
   messen; über 120 s je Held ist ein Befund, kein Blocker.
4. Ohne Population (Tabellen leer) läuft der Pfad weiter und sagt es sichtbar.
5. Livebeweis mit dem Debug-Binary aus dem Worktree gegen die zentrale DB,
   lesend und `--no-persist` (DSN per `eval "$(python3
   /home/nathanael/Documents/Infisical/export_gpt_secret.py --secret
   DEADLOCK_CENTRAL_DSN)"`, dazu `export PGOPTIONS='-c
   default_transaction_read_only=on'`, DSN nie ausgeben): `reason build Warden
   --json`; Referenzwaffen aus dem Seed
   `.tasks/2026-09-12-build-reasoner/referenz/lightbringer-warden.json`
   (Block "Core Items", Waffen-Tab) gegen den Build zählen, Ziel mindestens
   5/9; `reason backtest --hero Warden` muss dasselbe Build bewerten
   (Staple-Gate-Ergebnis und tau konsistent). Dazu die sechs Helden
   (Infernus, Lady Geist, Abrams, Vindicta, Bebop, Ivy) einmal durchlaufen und
   Recall gegen die Autoren-Referenz mit M-MESSUNG.md vergleichen; keine
   Regression gegen die dort gemessenen Nachher-Werte um mehr als eine Waffe.
   Zahlen nach `N-MESSUNG.md`.
6. Falls Live-Snapshots und FROZEN-V2 auseinanderliegen und deshalb Waffen
   fehlen: Ursache je Waffe benennen (Snapshot-Feld, Patch-Delta, Population),
   nicht am Maßstab drehen.

## Regeln

- Kein Python im Repo (Vergleichsskripte nur ad hoc in der Shell), keine
  Code-Kommentare, echte Umlaute auch in Commit-Betreffs, keine Gedankenstriche.
- `export PATH=/home/nathanael/.cargo/bin:$PATH`; in `rust/`: `cargo test -p
  dbrain-reasoner --lib`, Workspace-Tests ohne DSN (Baseline 386 bestanden,
  58 ignoriert), `cargo clippy -p dbrain-reasoner -p deadlock-brain
  --all-targets -- -D warnings`. Kein `--release`. `set -o pipefail`.
- Keine Referenz-Itemnamen und keine Warden-Sondergewichte im Produktcode.
- Zentrale DB nur lesend; kein Publish, keine KI-Aufrufe (`--no-ai`).
- Nur `feat/reasoner-planner-produktiv` committen und pushen, nie main.
- Fertigmeldung `FERTIG-N.md`: Commits, Dateien, Testzahlen, Warden x/9 live,
  Sechs-Helden-Tabelle, Laufzeit je Held, offene Punkte, Abweichungen.
