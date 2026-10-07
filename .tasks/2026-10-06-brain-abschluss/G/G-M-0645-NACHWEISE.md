# G-M-06:45: Wachstum und Sheetrechnung

status: gebaut und lokal geprüft, noch ohne Produktgate, 07.10.2026

Task `wq8uvf8ah`, Run `wf_fca62072-53f`, Agent `a200d2e1a634c7d1f` ist tatsächlich abgeschlossen. Kein Reasonerschreiber mehr aktiv. Rückgabe unter `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/030a7b6f-d25c-482d-b66c-68185cd05dbb/tasks/wq8uvf8ah.output`; vollständige Befehle und Logs in `G/pruefungen/g-m-0645/`.

## Anschluss

Öffentliche gemeinsame Eingänge: `calculate_hero`, `calculate_hero_with_deadline`, `project_hero`, `hero_growth`, `compare_hero_curves`, `compare_sheet_scenarios`. Wachstum verwendet dieselbe skalare Projektion. `BoonRange` enthält inklusive Grenzen; `GrowthMetric` unterstützt WeaponDps, DamagePerMagazine und Health. Punkte, absolute Boonzuwächse, Früh-zu-Spät-Wachstum und Überholungen sind strukturiert. Keine Interpolation durch Datenlücken; relatives Wachstum bei Ausgangswert 0 bleibt NotApplicable.

`SheetComparisonInput` bindet Szenario, optionale Beiträge und Sheetfaktor sowie HoldFixed/Reevaluate. Ursprüngliche Frist und Abbruch reichen bis Schussiteration und Abdeckung. F sollte bei mehreren Boonständen expected_level und expected_unspent_ap nicht auf einen einzelnen Zustand pinnen. Loader, SQL und Spiegelauswahl blieben unverändert.

## Zahlen und Herkunft

Originalproben an Clientversion 6759 gebunden, keine aktuelle Produktionsversion oder bestätigte Balancepatchzuordnung. Fixture `sheet-6759.json`, SHA-256 `fd2984bb5b23ffaa59fa5a815ca7c2b67542221a57be677384cccbb0934ffbff`. `fixture-integrity.json` enthält 30 byteinhaltlich gleiche Originalobjekte mit echten Pointern: vier Helden und 26 Items. Originalmaterial nicht zurechtgeschnitten.

Bei ausdrücklich 38 Gesamt-Spirit:

| Held | Boons | Waffen-DPS | Magazinschaden | HP |
| --- | ---: | ---: | ---: | ---: |
| Warden | 0 | 71.32850285714285 | 294.78 | 805 |
| Warden | 20 | 91.89612190476191 | 379.78 | 2005 |
| Warden | 35 | 107.32183619047619 | 443.53 | 2905 |
| Wraith | 20 | 89.3121693121693 | 438.88 | 1430 |
| Wraith | 35 | 111.53439153439153 | 548.0799999999999 | 1955 |
| Haze | 35 | 97.76190476190477 | 451.66 | 1885 |
| Yamato | 35 | 127.38095238095238 | 909.5 | 2305 |

Alle 36 Boonstände je Fixtureheld gegen project_hero; 0/20/35 zusätzlich gegen calculate_hero_with_deadline. Randfallkurven sind kontrollierte Fälle, keine Messwerte der Originalhelden.

## Acht Sheetstellen

1. Melee über echte weapon_melee-Referenz `/262`: Light bei 0/20/35 Boons 55/86.6/110.3; Heavy bei 0: 128. Positive Heavy-Boonregel unbekannt.
2. Power Slash `/263`: Spirit 38 ergibt 215.3, T3 384.3; Spirit 168 mit T3 ergibt 689.8. Kein Hinbiegen auf 698.2.
3. Flying Slash `/264`: gültige Rohbasis 0, Light-Melee-Koeffizient 1.2. Bei 0/20/35 Boons 66/103.92/132.36; vollständige Rotation unbekannt.
4. Crimson Slash `/265`: Schaden bei Spirit 38: 69.06, T3 91.86. Heilbasis 55 und T3-Flat 6 erhalten; Heilskalierung unbekannt.
5. Shadow Transformation `/266`: Zustandsblock. Dauer 5 → 8, Resistenzen 30 → 60, Cooldown 150 → 130, Waffenbonus 0 → 7.
6. Scratchpad ohne Waffenbonus: `r*(b+q)*(1+s)` ergibt 330.5357142857143.
7. Ohne Ratenbonus: `r0*(b*(1+w)+q)*(1+s)` ergibt 302.35714285714283.
8. Baseline: `r0*(q+b)` ergibt 220.35714285714286.

Scratchpadwerte verwenden ausdrücklich Yamato, 0 Boons, Spirit 38, Waffenbonus 50 %, Ratenbonus 25 %, s=0.2 und Flying-Slash-Beitrag q=66 je Schuss. Vollvariante 377.9464285714285. Das rekonstruiert die Rechnung, nicht die gelöschten Originaleingaben oder eine gemessene Spielrotation. Fehlendes q sperrt die Rechnung, eine ausdrücklich leere Beitragsliste erlaubt q=0.

Mystic Shot wird über vorhandene Proc-Ereignisse neu ausgewertet. Bei Spirit 38 und Ziel-Spiritresist 0.25: Vollvariante numerische Teilrechnung 166.95000000000002 bei 18 Schüssen, ohne Ratenbonus 222.60000000000002 bei 17 Schüssen. Ungeklärte Radiuswirkung verhindert bestätigte Gesamtmetriken; combat-Teilrechnung und unknowns bleiben getrennt von roher Sheet-DPS.

## Tatsächliche Prüfungen

Bereichsführung bestätigte tatsächliche Schlussmarker: Rechenabnahme 34 passed/0 failed/0 ignored/294 filtered, Gesamtsuite 321 passed/4 failed/0 ignored/3 filtered, Doc-Tests 0. Ausgangsbaseline 287/4, Vorgänger 303/5. Identische vier Restfehler: fix2_backtest_no_persist_on_read_only_connection, fix2_build_no_persist_on_read_only_connection, fix_facade_build_persists_scores_and_keeps_patch_history, fix_facade_patch_impact_and_backtest_persist_idempotently. Ursache PostgreSQL 42703, fehlende hero_build_id-Spalte. Kein grüner Gesamtlauf.

Betroffene Planerfixture nachgezogen, Produktplanung unverändert: zweiter Fixture-Waffenbonus 100 statt 20. Originaler zweiter Kauf brachte bei diskreten Schüssen keinen Grenznutzen (307.10185876623376 unverändert), korrigierte Fixture 449.32173611111114. Bindungsassertionen erhalten. Einzeltest 1 passed, 0 failed, 0 ignored.

Format und Verbrauchercompiler Exit 0. Striktes Reasoner-Clippy mit --no-deps Exit 0. Früherer Abhängigkeitslauf Exit 101 wegen drei result_large_err in dem damals noch großen PortFailure. G-K-R1 liefert inzwischen dessen boxed accounting und eigene grüne Clippyprüfung; gemeinsamer finaler Abhängigkeitscheck bleibt nötig.

Drei Produktionsfixtures ausdrücklich ausgeschlossen: loads_warden_and_reference_items_from_real_snapshot, fix_e_live_warden_evidence, scores_warden_reference_items_from_real_snapshot. Test-Postgres lief nur am isolierten Unix-Sockel und ist beendet. Keine Produktions-DB benutzt.

## Grenze

Offen: Heavy-Melee-Boonregel, Heilskalierung, EBaseWeaponDamageIncrease-Einheit, Proc-Radius, vollständige Rotation und globale Bounty-/Comeback-/Urn-/Midbossdaten. Kein Gate, Commit, Main, Deploy oder Liveabschluss durch Worker. G-V muss die echten Spiegel-/Rechteverträge anbinden; F konsumiert denselben Kern.

TESTNACHWEIS[TW-1]: 321 passed, 0 ignored | Baseline: 4 rot
