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

V2 ist erfolgreich eingefroren, alle 38 alten BuildObjects stimmen vollständig
mit der alten Fassade überein, alle geladenen Item-IDs haben einen Rohsnapshot.
SHA256 `FROZEN-V2.json`:
`5451b0b4f3cde928f12f09a4ea5f4fa3128c93fd8ca227f8208a1357f6cb7b1a`.
Die alte Baseline ist reproduzierbar aus `0af63da` plus `BASELINE-V2.patch` in
diesem Ordner; die Patchdatei enthält ausschließlich die Rohdatenquery und die
Roh-ID-Vollständigkeitsprüfung. Binary und SHA liegen im Nachweisordner als
`build-evaluation-baseline-v2-a57382a` und `baseline-v2-binary.sha256`.

## FreezeGuard nach unabhängiger Prüfung

Die maximale Poolgröße eins sichert nicht dauerhaft dieselbe physische
Postgresverbindung. Das Freeze-Werkzeug schaltet deshalb Idle-/Lifetime-Reaping
aus und vergleicht vor der ersten Loaderabfrage sowie nach sämtlichen Loadern
unmittelbar vor dem Dateischreiben Backend-PID, Transaktionssnapshot-ID,
`transaction_isolation=repeatable read` und `transaction_read_only=on`.
Abweichungen oder eine fehlgeschlagene Prüfung erzeugen einen Fehler, bevor
die Ausgabedatei geschrieben wird. Neue Freeze-Dateien tragen die geprüfte
Identität als optionale Metadaten; sie enthält keine Secrets.

Die isolierte Gegenprobe verändert jede der vier Eigenschaften einzeln und
prüft am tatsächlich verwendeten Schreibhelfer, dass keine Datei entsteht.
Unveränderte Identität schreibt erfolgreich. Die bestehende `FROZEN-V2.json`
wird nicht neu erzeugt: Sie hat diesen zusätzlichen PID-/Snapshotnachweis nicht,
und er wird nicht rückwirkend behauptet. Ihre unveränderten Inhalte bleiben
gemeinsame Eingabe beider Algorithmusstände. Die nachgewiesene alte
Fassaden-/Offline-Parität ist von der neu ergänzten Verbindungsprüfung getrennt.

## Kompilierte Quellprovenienz und exklusive Ausgaben

Künftige Messbinaries enthalten den vollständigen Git-HEAD und den Sauberkeitszustand beim Cargo-Bau. `build.rs` wird bei jedem Cargo-Aufruf erneut ausgewertet, daher auch nach Worktree-HEAD-/Ref-Wechseln. Cargo-OUT_DIR dient nur als standardmäßiges Buildmetadatum, nicht als Konfigurations- oder Secretweg. Unbekannte oder nicht eingecheckte Quellen verhindern beweisfähige CLI-Messungen; normale Tests bleiben möglich. Während des Baus dürfen die Quellen nicht parallel verändert werden.

Ein neuer Freeze bezeichnet seine tatsächlich eingebettete Fassade mit dieser Quellrevision. Nachherberichte trennen `algorithm_revision` des laufenden Binaries von `baseline_revision` der eingelesenen eingefrorenen Fassade. Die historisch korrekt erstellte FROZEN-V2 und ihre gesicherte alte Baseline bleiben unverändert; die neue Sicherung wird nicht rückwirkend behauptet.

Alle Ausgaben werden abschließend exklusiv mit `create_new` angelegt. Eine zwischen früher Pfadprüfung und Schreibbeginn erzeugte Datei wird niemals überschrieben. Schreibfehler entfernen die eigene unvollständige Ausgabe und werden propagiert. Scheitert die zweite Coverage-Datei, wird die zuvor erzeugte erste zurückgenommen; bestehende Zieldateien bleiben unangetastet.
