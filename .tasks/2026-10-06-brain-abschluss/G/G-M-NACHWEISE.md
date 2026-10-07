# G-M: Rechenkern-Rückgabe vor Fortsetzung 06:45

Stand: 07.10.2026. Workflow `wf_08b3462e-169`, Resume-Task `wthzcnb2d`, tatsächlich abgeschlossen. Produkt-WIP uncommittiert; kein Gate, Main oder Laufzeitabschluss. Nächster begrenzter Auftrag: `G/BRIEFING-G-M-0645.md`.

## Tatsächliche reine Schnittstelle

```rust
pub fn calculate_hero(models: &CalculationModels, hero_id: i64, scenario: &CalculationScenario) -> Result<CalculationResult>
pub fn rank_heroes(models: &CalculationModels, scenario: &CalculationScenario, metric: &str, direction: MetricDirection) -> Result<PopulationRanking>
pub fn validate_calculation_scenario(scenario: &CalculationScenario) -> Result<()>
pub fn damage_breakdown(raw_damage: f64, modifiers: &DamageModifiers) -> Result<DamageFactors>
pub fn calculation_models_from_payloads(heroes: &Value, items: &Value, hero_source: &ModelSource, item_source: &ModelSource) -> Result<CalculationModels>
```

Reine bestehende Hero-/Ability-/Itemkonverter sind ebenfalls öffentlich. `ModelSource`, `MeasuredValue`, Entitätsarten, Rechenmodelle, Szenario, Ergebnis und Ranking bleiben im vorhandenen Reasoner. Rechnung benutzt vorhandene Fortschritts-/Inventaraggregation und Simulation. Keine zweite Pipeline. Fs Loader und Planer-Produktlogik unverändert.

Waffenereignisse verwenden ganze Schüsse und tatsächliche Zeitpunkte. Unquantifizierte Effekte sperren bestätigte Simulationsmetriken/TTK; unabhängige bekannte Werte bleiben verfügbar. Diskrete Rechnung korrigierte Rundungsreste beim Kill, Reloadplanung bei Zielwechsel, vorzeitiges aktives Nachladen und die Überlebensbewertung bis zum tatsächlichen Waffen-Kill.

## Zahlen und Herkunftsgrenze

Bei 0 Boons, insgesamt 0 Spirit, 2.000 Ziel-HP, vollständigen Treffern und `AfterFireInterval` meldet der geprüfte Rechenlauf:

| Held | Schaden je Projektil | Waffen-DPS | Magazinschaden | TTK |
| --- | ---: | ---: | ---: | ---: |
| Haze | 5,26 | 50,095238 | 131,50 | 75,1500 s |
| Warden | 17,34 | 66,057143 | 294,78 | 47,6715 s |
| Wraith | 5,64 | 59,682540 | 293,28 | 50,3730 s |

Zwölf Kombinationen aus drei Helden, 0/35 Boons und 0/38 Gesamt-Spirit wurden im Rechentest ausgeführt. Weitere Fälle: Abrams mit neun Pellets und Einzelnachladen, Seven-Burst, Fixation, Skill-/Imbue-/Itemübergänge, Schilde, Regen und Resistwechsel. Keine zusätzliche 0,25-Sekunden-Konstante aus der beobachteten API-Reloadabweichung eingebaut; Rohmetriken separat erhalten.

Fixtureaufzeichnungen haben selbst keine bestätigte E-Laufzeitbindung; Testversion 1 ist künstlich. Der frühere numerische Vergleich mit 6759 ersetzt weder Originalhashbindung der Fixtures noch eine aktive Produktions-/Balancepatchzuordnung. Bereichsführung hat Original-Sheet-/Payloadhashes getrennt gegen Es 6759-Probe geprüft. Diese Grenze muss die Fortsetzung bei jedem Zahlenbeweis beibehalten.

## Prüfungen und neue Regression

Worker meldet Format, striktes Clippy und Verbrauchercompiler Exit 0. Rohbelege unter `G/pruefungen/g-m/release-hold-*.log`. Bereichsführung las die tatsächlichen Testmarker: Rechenlauf 17 passed, 0 failed, 0 ignored, 294 filtered. Isolierte Bestandssuite 303 passed, 5 failed, 0 ignored, 3 filtered. Der Rechenlauf ist Teilabdeckung, keine zusätzlich zu 303 addierbare Gesamtzahl. Baseline 287 passed, 4 failed, 0 ignored, 3 filtered. Drei Produktionsfälle ausdrücklich nicht ausgeführt.

Neue Regression: `planner::tests::later_unlock_keeps_the_binding_chosen_at_purchase` erwartet zwei Käufe. Mit diskreten Schüssen beträgt die Bewertung vor/nach dem zweiten Fixture-Item jeweils 307,10185876623376. Der zweite Kauf ist damit im aktuellen Fixture ohne Grenznutzen. Fortsetzung prüft eine wirksame Fixtureanschaffung und erhält die Imbue-Bindungsassertionen; keine Planer-Produktänderung. Die vier anderen Fehler entsprechen den Baselinefällen mit fehlendem `hero_build_id`; nicht als neue Regression oder grüne Gesamtsuite ausgeben.

## Offener Fachumfang

Die ursprüngliche Rückgabe behandelt DNS-/Scratchpadfehler noch als beschädigte Stellen. Abschnitt 14 des inzwischen rekonstruierten Sheetmodells ist zusätzlich umzusetzen: Meleeauflösung, Nicht-Spirit-Skalierungen, getrennte Schadens-/Heil-/Zustandswerte und strukturierter Gegenvergleich. Gemeinsame Wachstumskurven, Überholpunkte und F-Anschluss müssen dieselbe skalare Projektion verwenden.

Offen bleiben globale E-Regelbindung, schwere Melee-Boonregel, vollständige Ressourcenrotation, versionsgebundene AP-Kosten, wiederholte Procs am Stacklimit, Fremdstat-Skalierungen, Falloff-/Crit-Semantik, `recycle_time`, optimale Reloadabbrüche sowie Fähigkeiten und zeitweise Effekte als vollständige TTK. Vorhandene numerische Teilwerte sind keine Vollständigkeitsbehauptung. Grafiken/Webseiten und Roadmapeintrag sind kein G-Auftrag.

TESTNACHWEIS[TW-1]: 303 passed, 0 ignored | Baseline: 4 rot
