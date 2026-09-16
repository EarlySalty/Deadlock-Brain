# M-MESSUNG: Warden 6/9 und Sechs-Helden-Vergleich

Stand 2026-09-16. Worktree `~/.worktrees/deadlock-brain-m`, Branch
`feat/build-reasoner-population`. Messung offline gegen `FROZEN-V2.json`
(SHA256 `5451b0b4...cb7b1a`), Beispiel `build_evaluation`, Modus `plan`,
Debug-Build, kein `--release`. Populationsdaten aus der lokalen Wegwerf-DB
`population_dev` (80641 Spieler-Matches aus 10000 Ranked-Matches, Aggregate je
Held). Vorher = ohne Population (`POPULATION_DB_DSN` ungesetzt, leerer Prior,
reine Mechanik). Nachher = mit Population.

## Warden 779996 v45 (Referenz, 9 Waffen)

| Größe | Vorher | Nachher |
|---|---:|---:|
| Referenzwaffen | 3/9 | 6/9 |
| Kern-Recall | 0,3158 | 0,4737 |
| Jaccard (Kern) | 0,2069 | 0,3462 |
| Staple-Gate | - | bestanden (0/10 fehlen) |
| Kendall tau (Reihenfolge) | - | 0,639 |
| Jaccard@12 (Population) | - | 0,600 |

Neu getroffene Referenzwaffen: Swift Striker, Fleetfoot, Spiritual Overflow.
Bereits vorher: Titanic Magazine, Opening Rounds, High-Velocity Rounds. Alle
zehn Warden-Staples (Quicksilver Reload, Mercurial Magnum, Opening Rounds,
Titanic Magazine, High-Velocity Rounds, Spiritual Overflow, Fleetfoot, Swift
Striker, Extended Magazine, Enduring Speed) stehen im Build. Weiter offen:
Frenzy (kein Populations-Staple, unter 70 % Kaufanteil), Monster Rounds (Farm-
Nutzen im Heldenkampf nicht modelliert), Blood Tribute (negativer Solo-
Mechanikwert, von der Prevalenz-Stütze ausdrücklich ausgeschlossen).

## Sechs-Helden-Vergleich (beste beobachtete Referenz je Held)

| Held | Waffen vorher→nachher | Recall vorher→nachher | Staple-Gate | Kendall tau | Jaccard@12 |
|---|---|---|---|---:|---:|
| Warden | 3/9 → 6/9 | 0,316 → 0,474 | bestanden (0/10) | 0,639 | 0,600 |
| Infernus | 3/7 → 4/7 | 0,222 → 0,500 | bestanden (0/8) | 0,709 | 0,500 |
| Lady Geist | 1/7 → 3/7 | 0,111 → 0,444 | bestanden (0/4) | 0,522 | 0,263 |
| Abrams | 0/7 → 3/7 | 0,067 → 0,400 | bestanden (0/4) | 0,700 | 0,263 |
| Vindicta | 1/3 → 2/3 | 0,133 → 0,267 | bestanden (0/9) | 0,596 | 0,600 |
| Bebop | 0/4 → 0/4 | 0,083 → 0,167 | bestanden (0/2) | 0,587 | 0,143 |
| Ivy | 2/3 → 2/3 | 0,273 → 0,273 | null (keine Staples ab 70 %) | 0,364 | 0,091 |

Kein Held verschlechtert sich. Fünf von sieben verbessern die
Referenzübereinstimmung deutlich; Bebop hebt den Recall trotz gleicher
Waffenzahl; Ivy bleibt unverändert, weil die Population dort keine Staples ab
70 % trägt (der Prior greift nicht ein, Staple-Gate ist `null`). Bei allen
getrackten Helden ist das Staple-Gate bestanden (kein Staple fehlt).

## Deutung

Der Fortschritt ist kein Warden-Sonderfall: derselbe helden-unabhängige
Populations-Prior hebt sechs Helden und lässt den einen ohne Staple-Grundlage
unangetastet. Die Referenzwaffen, die vorher am gesättigten marginalen
Inventar-Kampfwert scheiterten (Spiritual Overflow, Fleetfoot, Swift Striker
bei Warden), kommen über die Staple-Priorität in ihre Kostenbänder, weil sie
echte Populations-Staples mit positivem Solo-Mechanikwert sind. Mechanisch
negative Items (Blood Tribute) bleiben ausgeschlossen. Der Prior ergänzt die
Mechanik als Meta-Stütze und ersetzt sie nicht.

## Nachweisdateien

`.tasks/2026-09-16-population-baseline/nachweise/`: `M-DIAG-fc71b5d.json`
(Baseline-Diagnose), `M-WARDEN-nopop.json`/`M-WARDEN-pop3.json` (Warden
vorher/nachher), `M-SECHS-nopop.json`/`M-SECHS-pop.json` (Sechs-Helden
vorher/nachher mit Populations-Backtest je Held).
