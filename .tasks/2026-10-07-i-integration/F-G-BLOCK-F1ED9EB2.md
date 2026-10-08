# F: fünfter Gate blockiert im G-Rechenkern

FRAGE AN ORCHESTRATOR: Bitte die zwei offenen Rechenkernbefunde dem bestehenden G-Ausführer zuordnen und geprüfte gesicherte Fixcommits an I liefern. I/F ändern keine G-Datei. Danach derselbe erhaltene F-Kontext für Integration und Folgegate, anschließend I allein für Main/Release/Install/Neustart und genau einen regulären Warden-Publish. Keine Gateübersteuerung oder weitere Produktfreigabe nötig.

## Minimaler G-Vertrag

1. `rust/crates/dbrain-reasoner/src/combat.rs:1913`: Schadensereignisse nach tatsächlicher Trefferzeit verarbeiten. Gatebeispiel: Wardens Schuss bei 0 Sekunden kann das Ziel vor der Flasche bei 0,1 Sekunden töten; derzeit rechnet der Code die Flasche zuerst. Zusammengehörige Fundstellen laut Gate: 1744, 1771, 1834, 1875, 1913, 2119 und 2141.
2. `rust/crates/dbrain-reasoner/src/calculation.rs:1555`: Bei `use_abilities=false` aktive Casts unterdrücken, passive Wirkungen erhalten. Gatebeispiel: Haze verliert sonst Fixation-Stapelschaden in calculate_hero/rank_heroes. Unsicherheitsmarkierung bei 995 mitprüfen.

Der kombinierte F/G-Kandidat bleibt bei BLOCK gesperrt. Diese Befunde stammen aus dem tatsächlichen lokalen Gate; Parent startet keinen zusätzlichen Reviewer und ändert keine geschützte Quelle. Frisch geholter origin/main 400381e681a2e08283db46094d1bbbde037e2813 hat gegenüber Gatebasis cf02c9a0 keinen Dateidelta in combat.rs oder calculation.rs. Dort ist also kein neuer Fix enthalten. Ein anderer bereits gesichert gelieferter passender Fixcommit ist nicht belegt. Keine G-WIP-Kopie.

## Gesicherter Stand

F-Baum /home/nathanael/.worktrees/brain-f-publish, Branch feat/brain-build-publish-ohne-matchgrenze. Commit f1ed9eb2b9929f2d7541e409cd313aaaf64d099f, Tree b6aa20b092548be2fcf8aaa65777a445e6959a93, sauber. Parent hat diesen geprüften Stand ausdrücklich als blockierten Featurecheckpoint regulär auf origin/feat/brain-build-publish-ohne-matchgrenze gesichert, Push Exit 0. Kein Main-Push, ALLOW, Deploy oder Publish.

Gatebasis cf02c9a06d56aeab72cc1d685cc3256f00ed9b1a. Original /home/nathanael/.worktrees/brain-f-publish/.tasks/2026-10-07-f-publish/gate-fixer5-f1ed9eb2-20261008.log normal Read geprüft. Modell claude-opus-5-5, BLOCK, Exit 1.

| Runde | Kandidat | Urteil |
| --- | --- | --- |
| 1 | ce512460 | BLOCK: Patchdatenverlust und verspätete Skill-Upgrades; Astra ohne Urteil wegen Eingabelimit, danach Opus |
| 2 | a62e09e1 | BLOCK: Discovery ersetzt Patchdaten und importiert unveränderte Einträge erneut |
| 3 | 1d18e391 | BLOCK: Build nicht an berechnete Spieldatenversion gebunden |
| 4 | 3329390a | BLOCK: falscher Skill-/Inventarstand und feststeckende gespeicherte Anfrage |
| 5 | f1ed9eb2 | BLOCK: Trefferreihenfolge und fehlende passive Fähigkeiten im G-Kern |

Runden 2 bis 5 beim gleichen tatsächlichen Urteilmodell. Keine sechste eigenmächtige G-Fixrunde. ed529bb ist eine Nachweisreparatur und hebt diesen Inhalts-BLOCK nicht auf.

## Prüfung und Grenzen

Unverändert committeter Source nach finalem Formatcheck, Check, strengem Clippy und seriellen Tests. 552 passed, 0 failed, 23 ignored, 0 filtered, Exit 0. Check/Clippy: dbrain-reasoner und deadlock-brain; Tests zusätzlich brain-storage. Wörtliche Kommandos separat in F-PRUEFBEFEHLE-F1ED9EB2.md.

23 Fälle nicht ausgeführt: Storage 4, Reasoner 17, Deadlock-Brain 2. Benötigen isolierte PostgreSQL-Fixtures oder ausdrücklich verfügbare Snapshot-/Livezugänge. Ignore-Annotationen zwischen ce512460 und f1ed9eb2 unverändert. Diese Kontrolle beweist keine Reparatur der früheren zwölf G-Fehler und keinen bestandenen DB-/Livebeweis.

TESTNACHWEIS[TW-1]: 552 passed, 23 ignored | Baseline: frühere zwölf G-Fehler nicht als behoben behauptet

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 1 | Gate: fünfter F-Gate BLOCK, claude-opus-5-5; Parent nur Featurecheckpoint gepusht

Das native F-Protokoll zählt getrennt 94 Git-Schritte und fünf Gaterunden. F/G-Writer jetzt beendet, Source und Prüfartefakte erhalten. Discovery bleibt bei P, keine eigene Patcharbeit oder Archivübernahme. E-Betriebsnachtrag 746/747 mit Version 6762 bleibt abgeschlossen; keine weitere Import-/Releasearbeit durch diesen BLOCK.

Private Originale/Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Secrets NEVER ausgeben. Kein Hookbypass, G-/K-Eigentum respektieren, keine fremden Dienste oder Q übernehmen. Kein Cleanup geschützter Zweige und kein Self-Settle vor tatsächlichem I-Gesamtabschluss.
