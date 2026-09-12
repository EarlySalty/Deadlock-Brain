# Review-Briefing: build-reasoner (Paket F, Runde 1)

[Orchestrator] Review Runde 1 für Paket F (Kern als Einkaufskurve nach
Kostenband). Lesend, kein Code, kein Branch. Du bist der einzige Thread für
dieses Review. Keine Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-f` (Branch
  `feat/build-reasoner-f`, Commit 5676a74, Basis main 80417b9 mit Paket E)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-f diff 80417b9..5676a74 -- rust/`
- Auftrag `BRIEFING-F.md`, Fertigmeldung im Worktree
  `.tasks/2026-09-12-build-reasoner/FERTIG-F.md`, Vorgeschichte `FERTIG-E.md`,
  `REVIEW-E.md`, Spec `ARCHITEKTUR.md` Abschnitte 8 und 9, `MECHANIK.md`
  Abschnitt 13, Seed `referenz/lightbringer-warden.json`, alles im Hauptordner
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`.
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Im Worktree liegen uncommittete Formatierungsänderungen an `ai_roles.rs`
  und `patch_tests.rs`, die nicht zu F gehören; ignorieren.

## Was du prüfst

1. Layout-Ableitung (`meta.rs:103` bis `:186`, `data.rs:1069`): Erkennung
   der Kern-Kategorie (Core, Kern, Standard, Main, Primary, Basis, Default,
   sonst größte Kategorie) an echten `details.modCategories` in
   `tierlist.hero_build_sources` prüfen (read-only, Zugang wie in `scripts/`,
   Secrets aus Infisical, nie ausgeben): Trifft sie bei Build 779996 die
   Kategorie "Core Items" mit 19 Items? FERTIG-F nennt für 779996 28
   Kern-Items, der Seed hat 19; kläre die Differenz (mehrere Kategorien
   erkannt, andere Version, oder Falscherkennung).
2. Zielgröße: FERTIG-F summiert gerundete Band-Mediane (Warden 0+1+2+5 = 8),
   während der Median der Kern-Gesamtgröße 10,5 ist und der Referenzbuild
   19 Kern-Items hat. Prüfe, ob die Rundung je Band systematisch Items
   verliert und ob der Kern von 8 Items der Absicht des Briefings ("Kern als
   Einkaufskurve über das Spiel") entspricht oder ein Rechenartefakt ist.
   Fix-Vorschlag, wenn Mangel: Bandziele so runden, dass die Summe den
   Median der Gesamtgröße trifft (größte Reste zuerst), oder Q3 statt Median
   als Ziel, jeweils mit Begründung, keine Gewichte.
3. Composer (`composer.rs:135`, `:266`, `:321`): Tier 1 und 2 nach
   `per_soul_value`, ab Tier 3 nach `total`; Slot-Caps 4/4/4 plus Flex;
   Reihenfolge Lane, Mid, Core, Late; Optional 12; Kern nur positive Scores
   (Regel aus E darf nicht verloren gehen). Prüfe die Nebenwirkung der
   neuen Mid-Phase in `mechanics.rs:210` und `:237` auf `item.rs`
   (Kaufphase steuert dort Zustandsfaktor oder per_soul-Gewichtung?) und ob
   `mechanics.rs` laut Briefing überhaupt angefasst werden durfte (nur mit
   Begründung; FERTIG-F begründet es mit der Phasenreihenfolge).
4. Backtest-Metriken: Kern-Überdeckung ist auf den Reasoner-Kern normiert;
   ein kleinerer Kern erhöht sie mechanisch (E: 19 Items, 0,2105; F: 8
   Items, 0,375 gegen den Seed, mittlere Autoren-Überdeckung aber nur
   0,1137). Beurteile, ob F wirklich besser ist oder die Metrik nur kleiner
   teilt; schlage die Kennzahl vor, mit der Paket F und E fair verglichen
   werden (zum Beispiel Treffer je Referenz-Kern-Item, Jaccard).
5. Loader-Fehler bei anderen Helden ("Ability 0 gehört nicht zum geladenen
   Helden", unvollständiges Waffenprofil bei Lady Geist, Infernus und
   weiteren): reproduziere für einen Helden mit `reason build <Held>
   --no-ai --no-persist --json` (Debug-Binary im Worktree) und benenne die
   Ursache (Snapshot-Daten oder Loader); das wird ein eigenes Paket, kein
   F-Mangel, aber die Ursache muss im Review stehen.

Tests selbst laufen lassen und Zahlen nennen: Workspace-Tests ohne DSN
(Fixer: Reasoner 90 bestanden, 16 ignoriert; E-Baseline Workspace 273
bestanden, 58 ignoriert, prüfe, ob Tests verschwunden sind), Clippy
`-D warnings`. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`, im
Verzeichnis `rust/`. Kein `--release`. Keine Schreibzugriffe auf den
Central-Pool.

## Ergebnis

`REVIEW-F.md` im Hauptordner: Mängelliste mit Datei:Zeile, Schwere
(blockierend, wichtig, nit), je Mangel Fix-Vorschlag; Kalibrierungsfragen
getrennt mit Empfehlung. Urteil: FREIGABE oder NACHBESSERN. Fertigmeldung in
diesem Thread mit Urteil. Deutsch, echte Umlaute, keine Gedankenstriche.
