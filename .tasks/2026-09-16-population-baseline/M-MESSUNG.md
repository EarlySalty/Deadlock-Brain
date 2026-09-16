# M-MESSUNG: Warden und Sechs-Helden-Vergleich (Runde 2, gegateter Hebel)

Stand 2026-09-16. Worktree `~/.worktrees/deadlock-brain-m`, Branch
`feat/build-reasoner-population`. Messung offline gegen `FROZEN-V2.json`
(`Documents/.tasks/2026-09-13-build-reasoner-ganzbuild/nachweise/`), Beispiel
`build_evaluation`, Modus `plan`, Debug-Build, kein `--release`.
Populationsdaten aus der lokalen Wegwerf-DB `population_dev` (119248
Spieler-Matches über 10059 Ranked-Matches, 38 Helden, Aggregate je Held).
Vorher = ohne Population (leerer Prior, reine Mechanik, `DEADLOCK_CENTRAL_DSN`
auf eine DB ohne Populationstabellen). Nachher = mit Population
(`DEADLOCK_CENTRAL_DSN` auf `population_dev`).

Neu gegenüber Runde 1: der Staple-Hebel umgeht die Marginal-Untergrenze und
gewinnt die kategorische Priorität nur noch, wenn der Kontext-Marginalwert des
Staples im aktuellen Inventar nicht negativ ist (REVIEW-M Mangel 2).

## Warden 779996 v45 (Referenz, 9 Waffen)

| Größe | Vorher | Nachher |
|---|---:|---:|
| Referenzwaffen | 3/9 | 6/9 |
| Kern-Recall | 0,316 | 0,421 |
| Staple-Gate | - | nicht bestanden (9/10, Enduring Speed fehlt) |
| Kendall tau (Reihenfolge) | - | 0,464 |
| Jaccard@12 (Population) | - | 0,500 |

Warden bleibt mit 6/9 über der Messlatte von 5/9. Getroffene Referenzwaffen wie
in Runde 1 (Swift Striker, Fleetfoot, Spiritual Overflow neu, dazu Titanic
Magazine, Opening Rounds, High-Velocity Rounds).

**Verlorener Staple:** Enduring Speed (Item 2447176615, Vitality Tier 2, 75,8 %
Kaufanteil, niedrigster der zehn Warden-Staples) steht nicht mehr im Build,
deshalb fällt das Staple-Gate. Ursache: der Kontext-Marginalwert von Enduring
Speed ist im vom Reasoner modellierten Kampfszenario strikt negativ; der
gegatete Hebel lehnt das Erzwingen eines anti-synergetischen Items bewusst ab
(genau der in REVIEW-M Mangel 2 geforderte Schutz). Enduring Speed ist ein
Bewegungs- und Sustain-Item; sein Nutzen liegt in einer Dimension (Mobilität,
Ausdauer), die das reine Kampfmodell nicht bewertet, dieselbe Klasse wie die
schon bekannten Grenzen bei Monster Rounds (Farm-Nutzen unmodelliert). Die
übrigen neun Warden-Staples haben nicht-negativen Marginalwert und bleiben im
Build.

## Sechs-Helden-Vergleich (beste beobachtete Referenz je Held)

| Held | Waffen vorher→nachher | Recall vorher→nachher | Staple-Gate | Kendall tau | Jaccard@12 |
|---|---|---|---|---:|---:|
| Warden | 3/9 → 6/9 | 0,316 → 0,421 | nicht bestanden (Enduring Speed) | 0,464 | 0,500 |
| Infernus | 3/7 → 5/7 | 0,222 → 0,556 | bestanden | 0,758 | 0,600 |
| Lady Geist | 1/7 → 2/7 | 0,111 → 0,333 | bestanden | 0,622 | 0,263 |
| Abrams | 0/7 → 3/7 | 0,067 → 0,400 | bestanden | 0,700 | 0,263 |
| Vindicta | 1/3 → 2/3 | 0,133 → 0,267 | bestanden | 0,344 | 0,600 |
| Bebop | 0/4 → 0/4 | 0,083 → 0,167 | bestanden | 0,587 | 0,143 |
| Ivy | 2/3 → 2/3 | 0,182 → 0,182 | null (keine Staples ab 70 %) | 0,364 | 0,091 |

Kein Held verschlechtert sich gegenüber seiner eigenen Vorher-Basis ohne Prior.
Sechs von sieben verbessern die Referenzübereinstimmung oder halten sie; Ivy
bleibt unverändert, weil die Population dort keine Staples ab 70 % trägt (der
Prior greift nicht ein, Staple-Gate ist `null`). Das Staple-Gate ist bei allen
getrackten Helden bestanden, außer bei Warden (siehe oben).

## Deutung

Der gegatete Hebel ist konservativer als der ungegatete aus Runde 1: er hebt
Populations-Staples nur dann über die Marginal-Untergrenze, wenn sie im Build
mechanisch nicht schaden. Das kostet genau die Staples, deren Nutzen außerhalb
des Kampfmodells liegt (Warden: Enduring Speed). Der Preis dafür ist Ehrlichkeit
statt eines erzwungenen Gate-Erfolgs: die Kaufkurve folgt der Mechanik, und ein
anti-synergetisches Item wird nicht allein wegen Prevalenz gekauft.

## ABWEICHUNG (Staple-Gate Warden vs. AUFTRAG)

Das AUFTRAG-Fertig-Kriterium nennt „Warden-Build: mindestens 5 der 9
Referenzwaffen und Staple-Gate bestanden". Mit dem in REVIEW-M Mangel 2 und im
Runde-2-Auftrag ausdrücklich geforderten Kontext-Marginal-Gate ist das
Staple-Gate für Warden nicht mehr erfüllbar, ohne ein Item mit negativem
Marginalwert (Enduring Speed) zu erzwingen. Umgesetzt ist der berechtigte Kern
des Review-Befunds (kein Erzwingen anti-synergetischer Items); die Folge für das
Warden-Staple-Gate ist hier dokumentiert statt am Maßstab zu drehen. Die
Waffen-Latte (6/9 ≥ 5/9) bleibt erfüllt.

## Nachweisdateien

`.tasks/2026-09-16-population-baseline/nachweise/`: `M-DIAG-fc71b5d.json`
(Baseline-Diagnose fc71b5d), `M-WARDEN-fc71b5d.json` (Warden-Baseline fc71b5d),
`M-SECHS-nopop.json`/`M-SECHS-pop.json` (Sechs-Helden inklusive Warden,
vorher/nachher mit Populations-Backtest je Held, Runde 2). Die 6/9 für Warden
und alle Zeilen der Tabelle oben stehen in `M-SECHS-pop.json`.
