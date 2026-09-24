# S14: Referenz-, Ressourcen- und Holdoutabnahme

**Testdesign, keine durchgeführte Replayabnahme.** Alle nachfolgenden echten
Replay-/Worker-/Storeprüfungen stehen aus. `audit/` testet Inventarmetadaten,
nicht Gameplay, Replayhashes, Dekompression oder eine produktive Sandbox.

## Zuerst benötigter echter Pilot

Ein durch S00/S01 freigegebenes Replay mit internem Rawlocator, Herkunft,
Nutzungs-/Speicher-/Weitergabestatus, Dateigröße, SHA-256, Matchidentität,
Patch-/Build-/Modebeleg und Aufbewahrungs-/Löschregel. Dieselben Rechte gelten
für Referenzartefakte. Keine Annahme, `.dem` sei durch Entfernen eines Namens
anonym. Zugangs-/Rechtefreigabe nicht aus öffentlich erreichbarer URL ableiten.

S14 und S13 wählen danach den tatsächlich kompatiblen Decoder-/Schema-Pin aus
`DECODER_CANDIDATES.json`; S02 integriert den reproduzierbaren Build samt
Registry-Closure und nativen Buildvoraussetzungen. Rust-Production-/Rebuildpfad
ohne notwendigen Python-/JVM-/.NET-Prozess. Ein optionaler Fremdparser darf
nur separat eine freigegebene Referenz erzeugen.

G2: Raw → begrenzter Rust-Decoder → gemeinsame Observations → selektierte
abfragbare Brain-Evidenz für einen echten freigegebenen Match. Gleicher aktiver
Knowledge-Release wie andere Pilotsourcen, keine Teilpublikation bei Fehler.
Ein korruptes Gegenbeispiel muss sicher begrenzt in Quarantäne enden.

## Vor dem Vergleich festzulegende Semantik

| Pflichtbereich | Vergleich/Unbekanntbehandlung | Toleranzentwurf, durch S05/S10 zu bestätigen |
|---|---|---|
| Match/Build/Mode | exakt belegte Matchidentität und gültiges Mapping, Unknown getrennt | exakte IDs, keine aus Dateinamen erfundene Patchversion |
| Teams/Hero | Spieler-Slot, Team-/Heroidentität und ggf. Wechsel getrennt | exakte IDs bei bekanntem Zustand, keine Namen in öffentlichen Fixtures |
| Dauer/Timeline | Gamezeit-Ursprung, Pregame, Pausen und Matchende dokumentieren | höchstens ein verifizierter Tick nach gleicher Zeitdefinition; kein pauschaler 60-Hz-Faktor |
| Items | Bestandssnapshot ist kein bewiesenes Kauf-, Verkaufs- oder Transferevent | exakte Item-IDs/Counts; Timing nur mit Rohereignisreferenz |
| Ability-Level | Ability-Slot/ID, Zustand und Upgradeereignis getrennt | exakte Level, keine Annahme eines Upgrades aus fehlenden Zwischenständen |
| KDA | Scoreboardzähler und Kill-/Death-/Assist-Events semantisch abgleichen | exakte Zähler bei gleicher Definition; unvollständige Eventcoverage ausweisen |
| Positionen | Koordinatensystem, Zell-/Offsetrekonstruktion und Unit belegen | maximal ein verifizierter Tick und eine belegte Quantisierungseinheit; Zahlenwerte erst nach Corpusprobe |
| Objectives | Objektidentität, Typ, Completionzeit und Akteurzuordnung unterscheiden | exakte Identitäten; Ereigniszeit wie Timeline |

Damage/Heal, Modifier, Farmkurven, Teamfightfenster und Coaching bleiben außerhalb
der Pflichtfelder, bis unterstützte Rohereignisse, Algorithmen und eigene
Referenzen belegt sind. Kein kanonischer Game-Fact aus einer bloßen Beobachtung;
kein kausaler Itemnutzen aus einer Item-Winrate.

## Golden-Corpus und Referenzbericht

Zehn bis zwanzig **reale freigegebene** Matches bzw. explizit genehmigter Umfang.
Nicht künstlich aus mehreren Feeds/Parserrevisionen desselben Matches aufblasen.
Patch/Mode/Heroabdeckung, beschädigte/unvollständige Eingaben und mindestens
einen Schemawechsel abdecken. Zugangslücken im echten Register stehen lassen.

Pro Feld/Patch/Parser: `supported`, `derived`, `unavailable` oder `unverified`.
Für `derived` Algorithmus-/Inputrevision, für alle tatsächlichen Aussagen
Units, Zeitbasis, Rawlocator, Referenzrevision, Prüfstatus und Abweichung erhalten.
Parser/Schema/Extractor und normalisierte Referenzresultate pinnen. Gemeinsame
Proto-/Parser-Vorfahren im Referenzbericht offenlegen; korrelierte Ableitungen
nicht als vollständig unabhängigen Test verkaufen.

## Replay-/Integrationssuite nach G1

| Prüfung | Erwartetes Ergebnis | Aktueller Nachweis |
|---|---|---|
| Erlaubter echter Match | Rawhash und Versionen erhalten, nur belegte selektierte Observations | nicht ausgeführt |
| Unbekannte Rechte | vor Download/Decode/Publikation verweigern, kein Egress | nur Metadatenprüfung im Audit; Runtime offen |
| Ungültige Magic/Truncation | begrenzter Fehler und Quarantäne, keine Teilpublikation | nicht ausgeführt |
| Dekompressionsbombe | Byte-/RAM-/Walltimegrenze erzwingen | nicht ausgeführt |
| Endlosschleife/Crash im Decoder | Parent beendet und reapt Worker, keine unbegrenzten Retries | nicht ausgeführt |
| Gleicher Match, mehrere Feeds | Raw-/Matchidentität erhalten, Population zählt einmal | nur Metadatenprüfung im Audit; Store offen |
| Reparse gleiche Revision | identische Observation-ID/-Menge; Checkpoint/Restart idempotent | nicht ausgeführt; keine Normalizer-IDs erfunden |
| Parser-/Schemawechsel | getrennte vollständige Version, keine Vermischung | nur Pin-/Case-Metadatenprüfung; echter Reparse offen |
| Fehlende Units/Tickbasis | Unknown/unsupported statt Null oder angenommener Umrechnung | nur Metadatenprüfung im Audit; Decoder offen |
| Bereinigung/Löschung | aktuelle Rechte/Tombstones gelten auch für historische Releases | nicht ausgeführt |
| Quelle/Hash/Locator manipuliert | Integrität verweigern, Daten nicht als valide markieren | nur deklarierte Konflikte im Audit; echte Hashprüfung offen |
| Wiederaufbau ohne Fremdruntime | gleicher freigegebener Output nur mit Rust/Store | nicht ausgeführt |

## Ressourcenprofil zur Freigabe, keine aktiven Produktionswerte

Konservativer **Startvorschlag** für die isolierte Pilotmessung, durch S04/S10 an
Zielhardware und realen Dateigrößen zu prüfen: ein Worker, Queue maximal vier
Dateien, Rawgröße maximal 512 MiB, kumulierte Dekompression maximal 2 GiB,
Worker-RAM maximal 768 MiB, Walltime maximal 60 Sekunden, selektierter Output
maximal 16 MiB oder 200.000 Observations (jeweils zuerst erreichte Grenze).
Nicht vorhandene Kapazität nicht voraussetzen; abgewiesene gültige große Dateien
als Budgetblocker ausweisen statt die Limits automatisch aufzuweichen.

Harte CPU-/RAM-/Prozessgrenzen über S04s gemeinsamen isolierten Rust-Worker;
kein privater Scheduler, kein unbeschränktes spawn_blocking und keine behauptete
Abbruchgarantie durch Timeout eines Futures. Netzwerk aus; nur autorisierte
Input-/Outputdeskriptoren. Nach Crash/Timeout sichere Quarantänemetadaten und
begrenzte Retries; Raw und abfragbare Ergebnisse bleiben getrennt.

Messen: Decoder-Bytes/s, CPUzeit/Match, Peak-RSS, rohe/dekomprimierte/selektierte
Bytes, Queuewartezeit, maximale laufende Worker und Reparsedauer. Gemischte Last
mit Queries und Reembedding: p95/p99 und Fehlerquote gegen dasselbe von S10
freigegebene Basisprofil vergleichen. Kein Leistungsversprechen aus Upstream-
README-Benchmarks, aus Metadatentests oder ohne Zielhardware.

## Zeitliche Holdouts

S05/S10 frieren Cutoff T, Rubrik, Zielmetrik, Mapping-/Rule-/Knowledge-/Parser-
revisionen vor dem Tuning ein. Entwicklungsfälle dürfen nur bis T tatsächlich
verfügbare Quellen nutzen; Spielgültigkeit separat filtern. Holdout-Matches
müssen nach T liegen und dürfen nicht über andere Feeds/Hashes/Spielerableitungen
in der Entwicklungsmenge wiederkehren. Neu korrigierte Wiki- oder Mappingstände
sind kein historisch damals verfügbares Wissen.

Retrospektiv erzeugte Parserresultate als solche kennzeichnen; heutiger Parser
beweist keine damalige Observability. Minikorpus nicht als repräsentative
Population darstellen. Coverage, n, Patch/Mode, Selektion und Unsicherheit im
S05-Ausgabevertrag sichtbar halten. Keine automatische Trainings- oder Coaching-
freigabe durch ein syntaktisch vollständiges Corpusmanifest.
