# Auftrag: Populations-Baseline und gutes Warden-Build

Stand: 2026-09-16. Stufe groß. Delegator: Fable (Claude-Session), Worker: Opus 4.8
als Claude-Subagenten (Nutzervorgabe vom 2026-09-16, T3-Threads sind wegen leerem
Codex-Kontingent bis 19.09. nicht nutzbar).

## Ziel in Nutzerworten

Die guten Teile aus `quinn-zilly/deadlock-build-modeling` (MIT, Python, Klon unter
`/tmp/dbm-eval`) sauber in Rust nachbauen und damit ein gutes Warden-Build aus dem
Build-Reasoner bekommen. Kein Python, keine Portierung eins zu eins: übernommen
werden die gemessenen Datenfallen, die Populations-Aggregate und die Messlatte,
nicht der Archetyp-Clusterer und nicht das Backoff-Modell als Build-Generator.

## Was aus dem Python-Repo übernommen wird

1. **Datenbereinigung der deadlock-api-Matchdaten** (`src/deadlock/features.py`,
   `dataset.py`, `README.md` Abschnitt "Source data caveats"):
   - rund 46 % der `items`-Einträge je Spieler sind Skillpunkte, keine Käufe;
     sie tragen die Skill-Reihenfolge und werden getrennt gespeichert.
   - rund 11,5 % der Item-Arrays sind unsortiert; Kaufreihenfolge immer nach
     `game_time_s` sortieren.
   - rund 9 % `net_worth_at_buy` sind korrupt (gleich dem Endvermögen); nur als
     Rang nutzen, nie als absolute Seelen.
   - Tier-5-Items (9999 Seelen) werden nie gekauft und fliegen aus dem Vokabular.
   - Item-IDs überschreiten int32: überall `i64`.
   - Kein Item wird zweimal vom selben Spieler gekauft; Verkauf ist zu 70 %
     Komponenten-Aufwertung, Mitgliedschaftsprüfungen laufen über die
     Kaufsequenz, nie über das Endinventar.
   - Der Metadaten-Endpunkt hat kein per-Spieler-`won`; Sieg aus
     `winning_team` gegen `team` ableiten.
2. **Populations-Aggregate je Held** (`evaluate.py`, `imbue.py`,
   `abilityorder.py`, `economy.py`): Kaufanteil je Item (Prevalence),
   Staples ab 70 %, Median-Kaufposition und Median-Kaufzeit, Verkaufsrate,
   Bigram nächster Kauf, Imbue-Ziel je Item (Modus mit Zählung, "split" unter
   50 %, "thin" unter 30), Skillpunkt-Reihenfolge (Modus). Gewichtung nach Rang
   (`average_badge`, Python: Zentrum 80) und Sieg als Zeilengewichte, nicht als
   Filter.
3. **Messlatte** (`evaluate.py`, README "Approach"): Staple-Gate (jedes Item ab
   70 % Kaufanteil muss im Build sein, sonst ist das Build falsch), Kendall tau
   der Build-Reihenfolge gegen die Median-Kaufposition der Population,
   Jaccard@12 gegen die Population mit Spieler-gegen-Spieler-Decke.

Nicht übernommen: Archetyp-Clustering (k-means auf Souls-Anteilen), Backoff-Kette
als Generator, In-Match-Advisor, HTML-Seite. Statt Clustering reicht vorerst je
Held ein Gesamtbucket plus drei Neigungs-Buckets (Weapon-, Spirit-, Vitality-lastig
nach größtem Seelenanteil der Shop-Tabs), jeweils nur, wenn der Bucket mindestens
25 % der Spieler des Helden hält.

## Pakete

| Paket | Inhalt | Worker | Abhängigkeit |
|---|---|---|---|
| W | Audit der ungemergten Welle 2 (`feat/build-reasoner-*`, Spitze `feat/build-reasoner-interactions-audit` fc71b5d): Branch-Topologie, was die Nebenbranches (planner, combat, ability-interactions, evaluation, holdout-final, planner-audit, review) an eigenen Commits tragen, Test- und Clippy-Stand der Spitze, Warden-Zahlen, Merge-Empfehlung. Nur lesen, Bericht `WELLE2-AUDIT.md`. | Opus 4.8 | keine |
| P | Neues Crate `rust/crates/dbrain-population`: Ingest der Matchdaten aus `api.deadlock-api.com` (Bulk-Metadaten, paginiert, gedrosselt), Bereinigung nach Liste oben, kompakte Speicherung je Spieler-Match in Postgres (`brain.population_*`), Aggregate je Held und Bucket, CLI `deadlock-brain population sync|stats|show`. Worktree `~/.worktrees/deadlock-brain-population`, Branch `feat/population-baseline` von `main`. | Opus 4.8 | keine |
| M | Messlatte in `dbrain-reasoner/src/backtest.rs` (Staple-Gate, Kendall tau, Jaccard@12) plus Population als Signal im Composer/Planner; Warden-Build erzeugen und gegen 9 Referenzwaffen und Staples messen. | Opus 4.8 | W und P |

Dateien sind disjunkt: P fasst nur das neue Crate, `rust/Cargo.toml`
(Workspace-Eintrag), eine neue Migration unter `scripts/migrations/` und einen
neuen Top-Level-Befehl `population` in `deadlock-brain/src/main.rs` an. Alles
unter `dbrain-reasoner/` gehört zu M und wartet auf den Audit W.

## Fertig-Kriterium

- `deadlock-brain population sync --matches 10000` läuft ohne Absturz durch,
  respektiert die Ratenlimits und ist idempotent (zweiter Lauf lädt nichts
  doppelt).
- `deadlock-brain population stats --hero Warden` zeigt Staples, Median-
  Reihenfolge, Imbue-Ziele und Skill-Reihenfolge mit Zählungen.
- Reasoner-Backtest meldet je Held Staple-Gate, Kendall tau und Jaccard@12.
- Warden-Build: mindestens 5 der 9 Referenzwaffen (Maßstab aus
  `.tasks/2026-09-12-build-reasoner/ABNAHME-FINAL.md`) und Staple-Gate bestanden.
  Keine Referenz-Itemnamen und keine heldenspezifischen Gewichte im Produktcode.
- cargo fmt (eigene Dateien), clippy `-D warnings` auf eigenen Crates, Tests grün;
  Baseline Workspace ohne DSN 273 bestanden, 58 ignoriert.

## Rahmen

- Kein Python, keine Code-Kommentare, echte Umlaute in allem Sichtbaren, keine
  Gedankenstriche.
- Kein `cargo build --release` in Worktrees; höchstens ein Release-Build auf
  dem Host, und den macht der Delegator.
- Zentrale DB nur lesend (`DEADLOCK_CENTRAL_DSN` erzwingt
  `default_transaction_read_only=on`); Entwicklung und der erste echte
  Ingest laufen gegen eine lokale Wegwerf-DB über Unix-Socket wie in
  `.tasks/2026-09-12-build-reasoner/FERTIG-E.md`. Die Migration auf die
  zentrale DB spielt der Delegator beim Deploy ein.
- Rohantworten der API nicht in `source_documents` oder `entity_snapshots`
  ablegen (5 GB für 25k Matches); nur normalisierte Zeilen speichern.
- Toolchain: `export PATH=/home/nathanael/.cargo/bin:$PATH`, rustc 1.97.
- Fremde Worktrees und Branches nicht anfassen. Nur den eigenen Branch
  committen und pushen, nie main.
