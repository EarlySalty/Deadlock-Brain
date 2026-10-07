# G: Rechenkernvertrag für I und K

status: Öffentlicher S3-Rechenkern 2b519b04 und S4-Abnahme a35bd814 regulär ALLOW und auf origin gesichert, 07.10.2026

## Konsumierbare Lieferung

S3-SHA: 2b519b0470fdb2d0ae9fde7791dff0c820ad3d1b. S4-SHA: a35bd8142146bd1ea0d8b342625ac6cf2e0bd278, enthält S3. G konsumierte wbhr8f9ck / wf_b82d8d03-5e4 und nachweise.json samt tatsächlichen Gruppenbelegen. S3 gegen e58a5c59, S4 gegen 2b519b04, jeweils gpt-6.1-sol und Exit 0/ALLOW. Eigenes ls-remote bestätigt origin-a35bd814; eigene Ancestorprüfung S3->S4 Exit 0, Rustbaum sauber.

Compiler, striktes Clippy all-targets und Format auf beiden committed Ständen Exit 0. 38 Rechenfälle bestanden, darunter alle 34 ursprünglichen Fälle, zwei kontrollierte Deadlinefälle und zwei Projektionsfälle. S3-Reasonersuite 304/12, S4 342/12, identische zwölf Fehlernamen gegenüber abgeschlossener S2-Baseline 304/12; Contracts jeweils 43+23+11 bestanden. Keine grüne Gesamtsuite und kein aktueller Balancepatch-/Produktionsbeweis aus Originalprobe 6759.

Boonboni bewahren Spiritprojektion bei Nahkampf und Regeneration; ein Nullkoeffizient verlangt keinen fehlenden Nahkampfwert. Numerischer synthetischer Gegenfall bei 35 Boons und 38 Gesamt-Spirit: Nahkampf 186,3, Regeneration 26,5, Flying Slash 223,56. Originalfixtures unverändert. Kontrollierte Fristen unterbrechen tatsächliche Simulation, Inventarabdeckung, Wachstum, Kurve und Sheetrechnung; Klone teilen Abbruch und Uhr.

I/F/K können diesen gesicherten gemeinsamen Rechenkern übernehmen, ohne auf G-main zu warten. Tatsächliche Receipt-/F-Parameter-/Produktionsportbindung bleibt zusätzliche G-V-Arbeit, keine vorweggenommene Produktionsabnahme im Rechenvertrag. Vier nicht blockierende S3/S4-NITs offen. Nachweis: G/pruefungen/g-m-s3-s4-fortsetzung/runde-2/nachweise.json. Noch kein Main-Merge, Deploy oder Liveabschluss.

## Historischer S2-Übergang

S2-Base für S3 ist e58a5c59d874848cf977bd0c3e7e77d7a4157d74. G konsumierte w64ib10gt, Rohgate und tatsächliche Befehlsbindung: volle S2-Gruppe gegen bd83d7ab mit gpt-6.1-sol Exit 0/ALLOW. Committed Combat 57 passed/0 failed/0 ignored, Compiler/striktes Clippy/Format Exit 0. Reasoner 304/12 gegenüber abgeschlossenem vergleichbarem Vorlauf 300/12 mit identischen zwölf Fehlernamen, Contracts 43+23+11 bestanden. Gesamtprüfung weiterhin rot; neuer Baselineversuch nicht abgeschlossen und abgeschlossener Vorlauf ausdrücklich verwendet. Eigenes ls-remote bestätigt e58a5c59 auf origin. S3/S4-WIP erhalten, noch kein konsumierbarer öffentlicher S3-SHA oder Main-/Livebeweis. S3/S4 folgt jetzt nach BRIEFING-G-M-S3-S4-FORTSETZUNG.md. Ks exklusive Ortsvertragsdateien bleiben unangetastet.

## Historischer Gate vor der jetzigen Sicherung

Aktuell b6c1153363f817ff1056a5fd28de13eec00cc58d: beide alten Restkerne und kontrollierte RequestDeadline committed; Combat 53 passed, Deadline-Teilmenge 6 passed, Compiler/Clippy/Format auf committed Stand Exit 0. Reasoner-Vollsuite 300/12 gegenüber gemessener gleicher Baseline 294/12. Voller S2-Gate gegen S1 tatsächlich BLOCK zu use_abilities=false und Untyped/Spiritmodifikatoren. Neuer frischer Workflow w64ib10gt / wf_966396fc-1d9 korrigiert genau diese bestätigten combat.rs-Kerne und prüft dieselbe volle Gruppe mit gpt-6.1-sol. Kein S2-Push oder konsumierbarer S3-SHA, kein Main-/Livebeweis. Der vollständige erhaltene S3/S4-WIP bleibt ungesichert, folgt unmittelbar nach echtem S2-ALLOW und Originbeweis. Folgebriefing BRIEFING-G-M-S3-S4-FORTSETZUNG.md vorbereitet.

## Historischer Übergang der ersten Fortsetzung

Neuer eigener Root ist tatsächlich bestätigt. Der alte Cargo-Exit 101 entstand laut gelesenem Rohlog durch das unbekannte Testhelferfeld DamageModifiers.resistances in combat.rs:3478; kein ausgeführter Testfall. Erste frische Runde korrigierte beide S2-Kerne und den Helfer als WIP; drei unterschiedliche Stackfälle tatsächlich bestanden, noch kein voller S2-Gate oder Commit. Workflow we899a9xq regulär gestoppt, weil die danach wiederholte fehlende Testuhr ein Scopeblocker ohne Gateurteil war. G gibt seinen bestehenden deadline.rs eng für die kompatible kontrollierte Zeitquelle frei; neuer frischer Workflow wck1q4g81 / wf_f845def0-25a prüft damit beide öffentlichen Simulationsgrenzen. Fünf frühere Fixcommits und vollständiger S3/S4-WIP bleiben erhalten. Noch kein neuer ALLOW, vollständiger Zahlenbeweis oder konsumierbarer S3-SHA. Starre Merge-Wartepflicht durch ENTSCHEIDUNG-PARALLEL-FERTIGSTELLEN.md ersetzt; Übernahme weiterhin ausschließlich nach echtem geprüftem und gepushtem Featurecheckpoint.

Historische Rückgabe wunjz2v6f: Cargo-Testaufruf Exit 101, Testzahlen und Fehlerursache wegen context-mode-Projektroot-Deny nicht verifiziert. Drei unmittelbare Regressionen ungeprüft/uncommitted erhalten, Produktkerne nicht gefixt. HEAD 8feb8b6e und elf unveränderte Quell-/Fixturehashes durch Bereichsführung bestätigt, nur Combat-Testergänzungen. Kein neuer Gate oder Push, kein konsumierbarer S3-SHA. Präziser Bericht G-M-S2-CARGO-SLOT-SPERRE.md. Keine Umgehung des verweigerten Logzugriffs, keine aktive eigene Produktarbeit.

## Übernahmegrenze

Dieser Vertrag beschreibt den vorhandenen G-Rechenkern, keinen Ersatzbau. I/K übernehmen erst den tatsächlich geprüften und gepushten G-Feature-SHA aus AN_HAUPT-G.md. Uncommittierte Dateien nicht kopieren. Der Rechenkern braucht keinen G-Main-Merge, Importer, Provider, Publishaufruf oder neue Persistenz. E-Receipt und Fs erweiterter reiner Eingang werden unter I separat integriert und gemeinsam compilergeprüft.

Aktueller Arbeitsort `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`. Originalfixtures F1/F2 sind als `bd1284ac`/`2b67796f` je isoliert compiler-/clippygeprüft, regulär ALLOW und auf origin gesichert. Finales S1 mit Bool- und endlichem Waffenratenfix `bd83d7abdef812a30daa47aa5f6a78de4f42263a` ist committed, isoliert compiler-/clippygeprüft, regulär gemeinsam gegen F2 ALLOW und durch Bereichsführung auf origin bestätigt. R2-Tests wurden nach einmaliger Slotablehnung nicht ausgeführt. S2 nach fünf frischen Fixrunden und sechs Gates weiterhin BLOCK auf lokalem HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206: Stackschaden ohne Rüstungsverringerung und mehrfach ausgelöste Effekte bei doppelten Item-IDs. Compiler/Clippy/Format Exit 0, Tests nicht ausgeführt; kein S2-Push. S3/S4 nicht begonnen, Eskalation über AN_HAUPT-G.md. Die öffentlichen Rechenfunktionen liegen weiterhin im ungesicherten Rest-WIP, kein konsumierbarer S3-Vertrag. Paketgrenzen und tatsächliche Gatebelege stehen in G/CHECKPOINTS.md.

## Gemeinsame Modellbildung

Öffentlicher Eingang `dbrain_reasoner::calculation_models_from_payloads` in data.rs:736:

```rust
pub fn calculation_models_from_payloads(
    heroes: &serde_json::Value,
    items: &serde_json::Value,
    hero_source: &ModelSource,
    item_source: &ModelSource,
) -> Result<CalculationModels>
```

Die beiden Payloads kommen aus demselben gebundenen E-Spiegelstand. ModelSource trägt client_version, document_id, original_url, kind, language und json_pointer. Der Konverter prüft gleiche positive Version, gleiche Sprachangabe, passende heroes/heroes_all- und items-Art sowie nichtleere Dokument-IDs. Eine damit belegte echte Dokumentzuordnung oder Freigabe behauptet er nicht. Source-Run/Hash/Revision/Rechte bindet der E/G-V-Adapter zusätzlich über den echten Receipt; ModelSource allein ist keine Freigabe oder Ersatz für diesen Receipt.

CalculationModels enthält client_version, heroes als SourcedHeroModel-Karte, weapons, items, item_sources und item_payloads. Die vorhandenen reinen ability_model_from_payload, hero_model_from_payload und item_model_from_payload bleiben öffentliche gemeinsame Konverter. Keine zweite Parserstrecke für F oder K.

## Öffentliche Rechnung

Alle Eingänge sind über dbrain_reasoner reexportiert:

| Eingang | Tatsächliche Parameter | Ergebnis |
| --- | --- | --- |
| calculate_hero | &CalculationModels, hero_id: i64, &CalculationScenario | Result<CalculationResult> |
| calculate_hero_with_deadline | dieselben plus &RequestDeadline | Result<CalculationResult> |
| project_hero | &CalculationModels, hero_id, &CalculationScenario, &RequestDeadline | Result<CalculationResult>, gemeinsame skalare Projektion ohne Simulation |
| hero_growth | &CalculationModels, hero_id, &CalculationScenario, BoonRange, &RequestDeadline | Result<HeroGrowth> |
| compare_hero_curves | &CalculationModels, hero_ids: [i64; 2], &CalculationScenario, BoonRange, GrowthMetric, &RequestDeadline | Result<HeroCurveComparison> |

Zusätzliche bestehende Reexports: validate_calculation_scenario, damage_breakdown, rank_heroes, compare_sheet_scenarios. Letzterer erhält models, hero_id, &SheetComparisonInput und &RequestDeadline und liefert SheetComparisonResult. Die vorhandene Simulation und Inventarauswertung werden weiterverwendet.

CalculationScenario verwendet genau einen Fortschrittseingang: ProgressionInput::Souls(i64) oder Boons(usize). SpiritInput::Derived leitet ab, Total(f64) benennt ausdrücklich den Gesamt-Spirit. Zusätzlicher Grund-Spirit wird darauf nicht erneut addiert. Inventar/Käufe, Imbues, Abilityreihenfolge, erwarteter Level/unverbrauchte AP, Boni und Zielszenario bleiben typisiert. Unbekannte Rohfelder sind keine Nullwerte.

CalculationResult enthält Version und Hero-ID, das tatsächliche Szenario, metrics, separate api_weapon_metrics, ability_properties/-views, Fortschritt, Shopboni, optionalen CombatScenarioEvaluation sowie assumptions und unknowns. MeasuredValue unterscheidet bekannte, unbekannte und nicht anwendbare Werte. Die API-Quellmetrik ist keine automatisch bewiesene vollständige Simulationsmetrik.

## F-Wachstum und ursprünglicher Abbruch

BoonRange enthält einschließlich min_boons und max_boons. GrowthMetric kennt WeaponDps, DamagePerMagazine und Health. HeroGrowth liefert Grundpunkt, Punkte, per_boon und early_to_late; HeroCurveComparison liefert die beiden Wachstumsansichten, Punkte, Überholungen, Gleichstandsboons und Status.

Wachstum benutzt dieselbe project_hero-Funktion wie die skalare Einzelrechnung. I benutzt diesen Eingang für Fs Build-/Wachstumsvergleich, statt eine zweite Formel anzulegen. Mehrstufige Boonkurven dürfen erwarteten Level/AP nicht auf einen einzelnen Zustand festpinnen. Datenlücken werden nicht interpoliert; relatives Wachstum bei Grundwert 0 bleibt nicht anwendbar. Ursprüngliche RequestDeadline weiterreichen, keinen neuen Abbruch oder Budgetrahmen eröffnen.

## Nachweisstand

Gelieferte Rechenabnahme: 34 passed, 0 failed, 0 ignored; Gesamtsuite 321 passed, 4 failed, 0 ignored, 3 filtered gegenüber unverändertem Ausgang 287 passed/4 failed. Dieselben vier DB-Fixturefehler: fehlende hero_build_id-Spalte. Drei Produktionsfixtures ausdrücklich nicht abgedeckt. Originalprobe 6759 mit ausdrücklich 38 Gesamt-Spirit; kein aktueller Produktions-/Balancepatchbeweis. Bericht G/G-M-0645-NACHWEISE.md.

Nachfolgende isolierte Compiler-/Clippy-/Gatebelege je committed Gruppe und der tatsächlich gepushte Feature-SHA werden in G/CHECKPOINTS.md und AN_HAUPT-G.md ergänzt. Dieser vorbereitete Text allein ist kein gesicherter Featurecheckpoint und keine G-/F-/K-/Liveabnahme.
