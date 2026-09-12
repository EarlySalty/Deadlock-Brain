# Fix-Briefing: build-reasoner (Paket F, Fixrunde 1)

[Orchestrator] Fixer für Paket F nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-F.md`
(vollständig lesen). Auftrag `BRIEFING-F.md`, Fertigmeldung im Worktree
`.tasks/2026-09-12-build-reasoner/FERTIG-F.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-f` (Branch
  `feat/build-reasoner-f`, Commit 5676a74, ausgecheckt; uncommittete
  Formatierungsänderungen an `ai_roles.rs` und `patch_tests.rs` gehören nicht
  zu F: mit `git checkout -- <datei>` verwerfen, nicht committen)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Entscheidungen des Delegators

1. Mangel 1 (wichtig), KF1: Bandziele per Größte-Reste-Verfahren so
   verteilen, dass ihre Summe `total_median.round()` trifft (Nachkommaanteile
   der Bandmediane absteigend aufrunden, Rest abrunden; bei exakt 0,5 im
   Gesamtmedian aufrunden). Kein Q3. Test: Warden-Layout aus Fixture mit
   Medianen 0 / 1 / 2 / 5 und Gesamtmedian 10,5 ergibt Summe 11.
2. Mangel 2 (wichtig): Flex-Budget nicht mehr aus `total_target - 12`,
   sondern aus der Slot-Kapazität des Snapshots: Flex-Slots sind die
   Plätze über den 4/4/4 je Kategorie, die das Spiel ab der Tier-Regel
   freigibt; ist das im Snapshot nicht ableitbar, gilt Flex = max(0,
   total_target - 12) mit dem korrigierten Ziel und einer benannten
   Konstante. Test dazu.
3. Nit 3: rustfmt-Churn bleibt (kein Rückbau, macht den Diff nur größer).
4. Nit 4: Edge-Case dokumentieren durch einen Test, der zeigt, dass ein
   Band bei erschöpftem Flex unterfüllt bleibt und der Rest in Optional
   landet; kein Auffüllen aus anderen Bändern.
5. KF2 (Kennzahl): `backtest.rs` liefert zusätzlich je Vergleich
   `reference_recall` (Treffer geteilt durch Größe des Referenz-Kerns) und
   den bestehenden Jaccard; die CLI-Ausgabe und die Persistenz
   (`brain.reasoner_backtests`) bekommen das Feld additiv (Migration nur,
   wenn die Tabelle es braucht, dann als neue Datei unter
   `scripts/migrations/`, nie die alte ändern). Im Bericht die Tabelle E
   gegen F mit Recall und Jaccard gegen Seed und Build 779996: E hatte 4
   von 19 Seed-Kern-Items (Recall 0,2105), F mit 8 Items 3 von 19 (0,158).
   Erwartung nach Fix 1: Kern 11 Items, Recall mindestens auf E-Niveau;
   wenn nicht, Ursache je fehlendem Referenz-Item benennen, keine Gewichte
   drehen.
6. Nachweis wie in BRIEFING-F Punkt 5: `reason build Warden --no-ai
   --no-persist --json`, Backtests Warden, Abrams, Dynamo, Kelvin, Debug-Binary,
   read-only.

## Regeln

- Dateien: `dbrain-reasoner/src/meta.rs`, `composer.rs`, `backtest.rs`,
  `types.rs`, `lib.rs`, Tests, bei Bedarf `deadlock-brain/src/main.rs` für
  die Ausgabe. Keine Änderungen an `item.rs`, `mechanics.rs`, `patch.rs`,
  `hero.rs` (Loader-Fehler bei anderen Helden ist Paket G).
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `dbrain-reasoner` mit `--all-targets -- -D warnings`, Workspace-Tests ohne
  DSN (Baseline 278 bestanden, 58 ignoriert), Reasoner-Tests mit
  `--include-ignored` gegen Central read-only (Baseline: 9 Scratch-Tests ohne
  `REASONER_SCRATCH_DSN` rot, Rest grün). Kein `--release`. Keine
  Schreibzugriffe auf den Central-Pool.
- Selbstprüfung vor der Fertigmeldung. Neue Commits obendrauf, kein
  `--amend`. Nur `feat/build-reasoner-f` pushen, nie main. Commit-Trailer
  `Co-authored-by: <dein Modell> <modell@local>`. Echte Umlaute, keine
  Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 1" in `REVIEW-F.md`
(Hauptordner): je Mangel Datei:Zeile, Änderung, Commit-SHA, Testzahlen
(Baseline und Endstand), Warden-Kern nach Fix (Items je Tier), Tabelle E
gegen F mit Recall und Jaccard.
