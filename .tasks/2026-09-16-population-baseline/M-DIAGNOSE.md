# M-DIAGNOSE: Warden-Waffenlücke auf fc71b5d

Stand 2026-09-16. Worktree `~/.worktrees/deadlock-brain-m`, Branch
`feat/build-reasoner-population` von `fc71b5d`. Messung offline gegen
`FROZEN-V2.json` (SHA256 `5451b0b4...cb7b1a`), Beispiel `build_evaluation`,
Modus `plan`, Debug-Build, kein `--release`. Laufzeit rund 102 s (Laden der
513-MB-Datei plus einmal Planen). Keine zentrale DB berührt.

## Baseline-Zahlen (fc71b5d)

- Warden 779996 v45: Waffen 3/9, Kern-Recall 0,3158 (6/19), Jaccard 0,2069,
  core_coverage 0,375 (6/16), order_proximity 0,3630.
- Zweite beobachtete Referenz 323228 v1: Waffen 1/4, Recall 0,25.
- 173 bewertete Items, 16 Kernkäufe, 12 am Ende gehalten.
- Getroffene Referenzwaffen: Titanic Magazine, Opening Rounds, High-Velocity Rounds.

## Die neun Referenzwaffen mit Item-Score und Kauf-Status

Score aus `item::score_item` (`total = per_slot_value + meta_support + patch`);
Rang unter allen 173 bewerteten Items. Belegdatei
`Documents/.tasks/2026-09-16-population-baseline-mess/M-DIAG-fc71b5d.json`.

| Referenzwaffe | Kosten | Rang | total | combat | condF | Status | Ursache |
|---|---:|---:|---:|---:|---:|---|---|
| Frenzy | 6400 | 5 | 63,45 | 56,05 | 1,00 | fehlt | T4-Band voll; marginaler Inventarwert unter dem schwächsten T4-Kauf |
| Spiritual Overflow | 6400 | 15 | 47,55 | 40,09 | 1,00 | fehlt | T4-Band voll; wie Frenzy |
| Titanic Magazine | 1600 | 42 | 18,86 | 15,43 | 1,00 | Kauf | - |
| Opening Rounds | 1600 | 57 | 15,44 | 11,99 | 0,50 | Kauf | - |
| Fleetfoot | 1600 | 75 | 13,07 | 9,66 | 0,31 | fehlt | bedingt abgewertet (condF 0,31); T2-Band von stärkeren Marginalkäufen belegt |
| Swift Striker | 1600 | 112 | 7,85 | 4,48 | 1,00 | fehlt | RampUp-Waffe; Zieltod-Fenster misst nur die Eröffnung, Aufbau fällt weg |
| High-Velocity Rounds | 800 | 139 | 5,13 | 3,20 | 1,00 | Kauf | - |
| Monster Rounds | 800 | 159 | 1,92 | 0,00 | 1,00 | fehlt | NonPlayer-Nutzen (Farm) im Heldenkampf = 0, nur Kaufbonus |
| Blood Tribute | 3200 | 173 | -7,09 | -12,47 | 1,00 | fehlt | aktives Item, negativer Kampfwert; Opfer-/Kanalkosten ohne gutgeschriebenen Nutzen |

## Kernursache: Kaufkurve, nicht Item-Bewertung

Der Planner (`planner::plan_with_economy`) wählt je Kostenband nach dem
**marginalen Inventar-Kampfwert** (`marginal_value = evaluation.score - before`),
nicht nach dem Item-Solo-Score `total`. Das Layout der beobachteten Autoren
begrenzt die Käufe je Tier (`remaining[tier] = layout.bands[tier].target`).

Tatsächliche Käufe je Tier: T1=5, T2=6, T3=1, T4=4; 0 ungefüllte Layoutplätze.
Die vier T4-Käufe (mit Marginalwert):

- Healing Tempo 162,37
- Mercurial Magnum 123,63
- Glass Cannon 91,83 (verkauft ein Item)
- Boundless Spirit 11,80 (schwächster T4-Kauf)

Frenzy und Spiritual Overflow haben die **höchsten Item-Solo-Scores** (rank 5
und 15), werden aber nicht gekauft, weil ihr marginaler Zuwachs im schon
waffenlastigen End-Inventar unter dem von Boundless Spirit (11,80) liegt. Reiner
Waffen-DPS sättigt (abnehmender Grenznutzen), während Heilungs- und
Survival-Items (Healing Tempo, Mercurial Magnum) im zieltod-beendeten
Kampffenster überproportionale Marginalwerte tragen. Das deckt sich mit den
dokumentierten Combat-Grenzen (`combat-throughput-ABSCHLUSS.md`): die 1,0-s-
Wechselpause deckelt den Durchsatznutzen, und Duell und bewegliches Ziel enden
beim Zieltod, sodass RampUp-Waffen (Swift Striker, Frenzy) nur in der Eröffnung
zählen. Ein vollständiger Combat-Umbau ist ausdrücklich ein eigenes Paket und
nicht Teil dieses Auftrags.

## Ableitung der Fixes (Phase 2)

1. **Population als generisches Signal (Auftragspunkt 6, Haupthebel).** Frenzy,
   Spiritual Overflow und Monster Rounds sind mechanisch positiv (Frenzy/Spiritual
   Overflow sogar top), scheitern aber am marginalen Inventarwert bzw. am
   fehlenden Heldenkampf-Nutzen. Ein helden-unabhängiger Prior aus Kaufanteil und
   Median-Position (Meta-Stütze neben dem Mechanik-Score) hebt Staples über die
   Auswahlschwelle, ohne den Maßstab zu senken. Kein Item darf allein wegen
   Prevalence in den Kern, wenn die Mechanik negativ ist (das schützt Blood
   Tribute vor einem unverdienten Kauf).
2. **Messlatte (Auftragspunkt 5).** Staple-Gate, Kendall tau gegen die
   Median-Kaufposition und Jaccard@12 gegen die Population; je Held im Backtest
   ausweisen, `null` wenn die Population den Helden nicht trägt.
3. **Bedingte Items ehrlich (Auftragspunkt 3).** condF für Fleetfoot (0,31)
   bleibt sichtbar; keine künstliche Aufwertung. Blood Tributes negativer
   Kampfwert bleibt ein Ausschlussgrund, nicht ein zu behebender Bug.
4. **Gate-NITs.** `build.rs` Rerun-Provenienz sauber lösen; `combat.rs`
   `simulate()` verschattet den Parameter `bindings`.

## Was ohne Population nicht erreichbar ist

Die zwei aussichtsreichsten fehlenden Referenzwaffen (Frenzy, Spiritual Overflow)
sind rein mechanisch top-bewertet, aber vom marginalen Survival-Übergewicht im
vollen T4-Band verdrängt. Ohne den Populations-Prior ist 5/9 daher
unwahrscheinlich, weil die verbleibende Lücke aus dem dokumentierten
Combat-Grenzverhalten stammt, dessen Umbau ein eigenes Paket wäre. Der
Populations-Prior ist der auftragskonforme Weg zu den Staples.
