# FERTIG-M: Paket M Messlatte, Populations-Signal, Warden 6/9

Branch `feat/build-reasoner-population` (Worktree `~/.worktrees/deadlock-brain-m`,
von `fc71b5d`), Paket P `origin/feat/population-baseline` gemergt.

Hinweis: Die Zahlen unten sind der Runde-1-Stand mit dem ungegateten Hebel.
Runde 2 (REVIEW-M) hat den Staple-Hebel an einen nicht-negativen
Kontext-Marginalwert gebunden und alle sieben Helden neu gemessen; der aktuelle
Stand samt der Warden-Staple-Gate-Abweichung steht in `M-MESSUNG.md` und
`FERTIG-FIX-M2.md`.

## Ergebnis (TLDR)

- Warden 779996: **3/9 → 6/9 Referenzwaffen**, Staple-Gate **bestanden**. Ziel
  von mindestens 5/9 übertroffen.
- Sechs-Helden-Vergleich: kein Held verschlechtert sich, fünf verbessern sich
  deutlich, Staple-Gate bei allen getrackten Helden bestanden (Details unten).
- Kein Warden-Sondergewicht, keine Referenz-Itemnamen im Produktcode; der Prior
  ist helden-unabhängig.

## Commits (auf dem Branch, ohne die gemergten P-Commits)

- `b21b3ea` mess(reasoner): Referenzwaffen-Diagnose im Backtest-Example
- `c01f5a8` fix(reasoner): Gate-NITs Rerun-Provenienz und bindings-Verschattung
- `eac1875` feat(reasoner): Populations-Prior als Meta-Stütze im Planner
- `ef94952` feat(reasoner): Populations-Prior nur für Staples, Staples umgehen Marginal-Untergrenze
- `86804c0` feat(reasoner): kategorische Staple-Priorität in Auswahl, Beam und Sparen
- `3ff5db4` feat(reasoner): Populations-Backtest und Staple-Begründung je Item

## Geänderte und neue Dateien (dbrain-reasoner)

- neu `src/population_prior.rs` (PopulationPrior, PopulationItem, POPULATION_PRIOR_WEIGHT)
- geändert `src/lib.rs` (Modul und Re-Export), `src/meta.rs` (MetaIndexWithSources.population)
- geändert `src/planner.rs` (PlanningContext.population, Search-Prior, supported_value, kategorische Staple-Priorität in choices/beam/Sparen)
- geändert `src/composer.rs` (Prior durch PlanContext, Staple-Begründung je Kern-Item)
- geändert `src/backtest.rs` (kendall_tau, jaccard_at, PopulationBacktest, population_backtest)
- geändert `src/combat.rs` (Gate-NIT: `active_bindings` statt Verschattung), `build.rs` (Gate-NIT: klarer Sentinel)
- geändert `examples/build_evaluation.rs` (Referenzwaffen-Diagnose, Population laden, Populations-Backtest ausweisen), `Cargo.toml` (dev-dep dbrain-population)

## Testzahlen vorher/nachher

- Reasoner-Lib: 167 (fc71b5d) → **170** bestanden, 16 ignoriert, 0 rot.
- Workspace ohne DSN: 359 (Welle-2-Baseline) → **384** bestanden, 58 ignoriert, 0 rot.
- `cargo clippy -p dbrain-reasoner -p dbrain-population --all-targets -- -D warnings`: sauber.
- fmt nur auf eigene Dateien (`rustfmt --edition 2021`); kein `cargo fmt`.

## Warden und Sechs-Helden (vorher = ohne Population, nachher = mit)

| Held | Waffen | Recall | Staple-Gate | Kendall tau | Jaccard@12 |
|---|---|---|---|---:|---:|
| Warden | 3/9 → 6/9 | 0,316 → 0,474 | bestanden (0/10) | 0,639 | 0,600 |
| Infernus | 3/7 → 4/7 | 0,222 → 0,500 | bestanden (0/8) | 0,709 | 0,500 |
| Lady Geist | 1/7 → 3/7 | 0,111 → 0,444 | bestanden (0/4) | 0,522 | 0,263 |
| Abrams | 0/7 → 3/7 | 0,067 → 0,400 | bestanden (0/4) | 0,700 | 0,263 |
| Vindicta | 1/3 → 2/3 | 0,133 → 0,267 | bestanden (0/9) | 0,596 | 0,600 |
| Bebop | 0/4 → 0/4 | 0,083 → 0,167 | bestanden (0/2) | 0,587 | 0,143 |
| Ivy | 2/3 → 2/3 | 0,273 → 0,273 | null (keine Staples ab 70 %) | 0,364 | 0,091 |

Neu getroffene Warden-Referenzwaffen: Swift Striker, Fleetfoot, Spiritual
Overflow. Alle zehn Warden-Staples im Build. Weiter offen: Frenzy (kein
Populations-Staple), Monster Rounds (Farm-Nutzen unmodelliert), Blood Tribute
(negativer Mechanikwert, ausgeschlossen).

## Wie der Prior wirkt

`PopulationPrior` trägt je Item Kaufanteil, Median-Position und Staple-Flag. Im
Planner bekommen Staples mit positivem Solo-Mechanikwert (`per_slot_value > 0`)
eine kategorische Priorität in Kandidatenauswahl, Beam und Spar-Entscheidung,
sodass sie ihre Kostenbänder gewinnen und das Staple-Gate erfüllen; innerhalb
der Staples entscheidet der Mechanik-Marginalwert plus eine Prevalenz-Stütze
(`POPULATION_PRIOR_WEIGHT` auf den Mechanik-Slotwert) über die Reihenfolge. Ein
Item mit negativem Solo-Mechanikwert wird nie allein wegen Prevalenz gewählt.
Die Begründung je Kern-Staple nennt Kaufanteil, Median-Position und
Mechanik-Slotwert.

## ABWEICHUNGEN

- ABWEICHUNG (Jaccard@12-Decke): Der Auftrag nennt „Jaccard@12 gegen die
  Population mit Spieler-gegen-Spieler-Decke". Die Decke (Normalisierung gegen
  den mittleren Spieler-zu-Spieler-Jaccard) braucht einzelne Spieler-Inventare,
  die der aggregierte `PopulationPrior` nicht trägt. Umgesetzt ist der rohe
  Jaccard@12 der Build-Top-12 gegen die zwölf prevalentesten Populations-Items.
  Folge: der Wert ist absolut, nicht auf die erreichbare Obergrenze normiert.
- ABWEICHUNG (Begründung im Code vs. Kommentarverbot): Der Auftrag will die
  Gewicht-Begründung „im Code". Das globale Kommentarverbot schlägt das; die
  Konstante `POPULATION_PRIOR_WEIGHT` trägt den Zweck im Namen, die inhaltliche
  Begründung steht als nutzersichtbarer Evidence-Text je Staple-Item.
- Hinweis (build.rs-NIT): Der Rerun-Sentinel bleibt ein nie existierender Pfad
  (dokumentiertes Cargo-Verhalten „nicht existierender Pfad → Rerun bei jedem
  Build"), nötig damit Git-HEAD und Sauberkeitszustand nach jedem Commit frisch
  eingebettet werden; der irreführende Kommentar ist entfernt, der Name ist
  selbsterklärend.
- Hinweis (Messbinaries): Vorher-Lauf mit Binary `86804c0`, Nachher-Lauf mit
  `3ff5db4`. Die Kaufkurven-Logik ist zwischen beiden identisch (planner.rs seit
  `86804c0` unverändert, `3ff5db4` fügt nur Backtest-Ausgabe und Evidence-Text
  hinzu). Beleg: die Vorher-Warden-Baseline ohne Population (3/9, Recall 0,3158)
  ist identisch zur ursprünglichen fc71b5d-Messung.

## Population als Datenquelle

Aggregate in lokaler Wegwerf-DB `population_dev` (P-Ingest: 119248
Spieler-Matches aus 10059 Ranked-Matches, 38 Helden; die frühere Zahl 80641 war
ein älterer Ingest-Zwischenstand). `population stats` je Held persistiert die Aggregate;
das Mess-Example lädt sie über `POPULATION_DB_DSN` und `PopulationIndex::load`.
Die zentrale DB wurde nicht beschrieben; die Produktiv-Migration spielt der
Delegator ein.

## Offene Punkte

- Produktiv-Verdrahtung: `reason_build_with_options` (`lib.rs`) setzt den Prior
  noch auf `default()`. Für den Live-Build muss der Produktivpfad die Population
  aus der zentralen DB laden und in `MetaIndexWithSources.population` setzen
  (analog zum Mess-Example). Der Modell- und Backtest-Teil steht; nur der
  DB-Ladepfad im Produktivlauf fehlt.
- Jaccard@12-Decke (siehe ABWEICHUNG), falls eine normierte Kennzahl gewünscht
  ist: dazu die Spieler-Inventare im Populations-Crate exponieren.
- Frozen mit Population: ein Freeze-Neubau könnte die Population mit einfrieren,
  damit die Messung ohne laufende Populations-DB reproduzierbar ist.

## Pfade

- `.tasks/2026-09-16-population-baseline/M-DIAGNOSE.md`, `M-MESSUNG.md`, `FERTIG-M.md`
- Nachweise: `.tasks/2026-09-16-population-baseline/nachweise/`
