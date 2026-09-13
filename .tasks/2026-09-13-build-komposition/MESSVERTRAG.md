# Eingefrorene Messbasis

Das Example `rust/crates/dbrain-reasoner/examples/build_evaluation.rs` trennt
Datengewinnung, Entwicklungsmessung und zurückgehaltene Referenzen. Baseline ist
`a57382a`; die zusätzlichen öffentlichen Loader ändern kein Rechenverhalten.

`freeze DATEI` verwendet den bestehenden `pg_pool_read_only()`-Zugang mit
Infisical-Credential-FD 5. Ein eigener Pool mit genau einer Verbindung führt alle
Loader in derselben `REPEATABLE READ READ ONLY`-Transaktion aus. Kein KI-Aufruf,
keine DB-Schreiboperation und kein Upload. Der Stand enthält alle 38 Helden,
Itemmodelle, Patch-Snapshots und Events, Meta- und Paarstatistik, Claims,
Autorenquellen, Katalog-Tiers, statistische Skillfolgen sowie die jeweils neuesten
Rohsnapshots der Assets-Quelle. Die Rohsnapshots bewahren auch bisher nicht
modellierte Felder wie `cost_bonuses` und Item-Upgrades. Vor Ausgabe wird jedes
offline erzeugte BuildObject vollständig gegen die produktive Fassade verglichen.

`evaluate FROZEN AUSGABE` berechnet Autorenübereinstimmung mit denselben
Eingangsautoren. Referenz-Recall und Jaccard verwenden die vollständige Pflicht-
Kaufkurve, zusätzlich werden Waffen-Treffer gezählt. Das ist eine Entwicklungs-
und Reproduktionsmessung, kein unabhängiger Backtest. Warden 779996 Version 45
ist ausdrücklich bekannter Entwicklungsfall; mindestens fünf von neun Waffen
bleiben Pflicht. Der Screenshot-Seed wird nicht eingelesen.

`holdout FROZEN AUSGABE` entfernt die zurückgehaltene Autoren-ID vor jeder
Ableitung global aus allen Heldenquellen. Damit sind Layout einschließlich
globalem Behelf, Kern-/Situationsrolle, Verkäufe, Autorenskillfolgen und
`author_hits` getrennt. Die dokumentierten zusammenarbeitenden Identitäten
Lightbringer 13446690, Situation 34634349 und Kollabkonto 1650097169 bilden eine
untrennbare Gruppe (Beleg: ursprüngliche Akte `REGISTER.md`, `FIX-BRIEFING-S.md`).
Sobald eine davon Referenz ist, werden alle drei entfernt. Claims und die
separate statistische Skillfolge werden ganz ausgeschlossen, weil ihre
Autorenprovenienz nicht vollständig auflösbar ist. Es gibt keinen nachträglichen
Fallback auf entfernte Quellen. Jeder Referenzbuild und die Anzahl verbleibender
Trainingsquellen stehen im Bericht.

Die zeitgleichen aggregierten Match-/Itemstatistiken bleiben im Holdout bekannt.
Das misst Übertragung auf zurückgehaltene Autoren, keine zeitlich unabhängige
Meta-Prognose. Historische Vorher-/Nachherdaten werden nicht ergänzt. Der
Orchestrator bekommt Holdout-Ergebnisse erst nach Festlegung der Algorithmus-
fassung; sie dürfen nicht als verstecktes Tuningziel an Bauagenten gehen.

`sensitivity FROZEN AUSGABE` berechnet dieselben 38 Helden zusätzlich ohne
Paarstütze, ohne numerische statistische Scores und mit Kampffenstern von 20
beziehungsweise 60 statt 40 Sekunden. Beim Abschalten statistischer Scores
bleiben Autorenlayout und Kategorien erhalten; dies ist deshalb keine vollständig
autorenfreie mechanische Prognose. Mechanische Aussagen brauchen darüber hinaus
den unabhängigen fachlichen Kritiker und die Belege der Inventarbewertung.

Große JSON-Dateien liegen ausschließlich im Documents-Nachweisordner und werden
nicht eingecheckt. Vorher/Nachher verwenden dieselbe Datei und deren SHA256.
Ändert sich die Modellbildung, müssen neue Felder aus diesen eingefrorenen
Rohsnapshots rekonstruiert werden; ein Nachladen aktueller Assets ist kein fairer
Vergleich. Die Baseline-Datei wird nicht mit neuen Ergebnissen überschrieben.

## Tatsächlich eingefrorener Stand

Datei: `/home/nathanael/Documents/.tasks/2026-09-13-build-reasoner-ganzbuild/nachweise/FROZEN.json`

SHA256: `e9c4fae08e75dd639fd1a0651aa9c61cc26f6cdd1f4e49d892d8ca4cf38e8b0b`.

Der Lauf vom 13.09.2026 hat 38/38 BuildObjects vor dem Schreiben vollständig
identisch reproduziert. Die Datei ist rund 468 MB groß. Separater Rohdatenzugang:
`RAW-SNAPSHOTS.json` im selben Ordner (17 MB, ausschließlich aus FROZEN extrahiert).
`BASELINE.json` wird mit dem unveränderten Algorithmus a57382a erzeugt. Die
Sichtbarkeits- und Serde-Erweiterungen stehen in Commit `4fee21b`.

Validierung des Messwerkzeugs: Reasoner-Lib 111 Tests bestanden, 16 bestehende
Tests ignoriert; Example gebaut und Clippy mit `-D warnings` bestanden.

## Korrektur der Rohdatenidentität

Der erste Freeze bewahrt die vollständigen alten Modelle und reproduziert sie,
aber seine Rohsnapshot-Abfrage gruppierte nach Anzeigename. Das vermischt
beispielsweise das Item Grit (1672893796) mit der gleichnamigen Fähigkeit
(2260066289). Der neue Parser hat den fehlenden Item-Snapshot ausdrücklich
abgewiesen; es wurde kein Ersatzwert erfunden. Für den neuen Vorher-/Nachher-
Vergleich gilt deshalb ausschließlich `FROZEN-V2.json` mit demselben Dateiort.

V2 gruppiert nach Quelle, Entitätstyp und Plattform-ID (bei Quellen ohne ID:
external_id, erst danach kanonischer Name) und wählt die letzte Version. Die
Item-Card-Rohquelle wird zusätzlich bewahrt. Vor dem Schreiben wird für jedes
geladene Item dessen Rohsnapshot-ID geprüft. V2 wird vom isolierten alten
Algorithmus a57382a erzeugt; dessen Offline-/Fassadenparität bleibt Pflicht.
Die ersten Dateien einschließlich Baseline bleiben als ersetzter Nachweis liegen.

Nachher reichert `enrich_frozen_models` neue Modellfelder ausschließlich aus der
neuen Frozen-Datei an. `plan FROZEN AUSGABE HELDEN` gibt zusätzlich sämtliche
Inventarübergänge und Kampfszenarien derselben Kaufplanung aus. Eine kommagetrennte
Heldenauswahl ist für `evaluate`, `sensitivity` und `plan` möglich; unbekannte
Namen werden abgewiesen. Holdout läuft grundsätzlich über den ganzen Bestand.
