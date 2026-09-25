# Rust-Replays, Beobachtungen und Population

**Grundlage:** U4, insbesondere Replayintegration, zweistufige Speicherung und zeitliche Holdouts. Die vorgeschlagenen Decoder/Felder müssen an vorhandenen erlaubten Replays tatsächlich verifiziert werden. Dies ist ein Pflicht-Ausbaustrang mit klar begrenztem Pilot, kein Leistungsversprechen für jeden historischen Patch.

## 1. Wer macht was?

14 besitzt Rust-Replayimport/-decoder, Capability-Matrix und normalisierte Observations. 13 liefert gepinnte Schemas/Protokolländerungen sowie Match-/Metafeeds; 04 gemeinsame Warteschlangen/Jobs; 03 Storage/Migrationen; 05 Population, Learning, Reasoner-/Coachingintegration; 10 Referenzprüfung/Holdouts. 08/09 liefern Ergebnisse nur über denselben Kernel/Exportvertrag aus.

## 2. Pilot statt unbegrenzter Massenspeicherung

U4 schlägt zehn bis zwanzig Golden Replays vor. Als Startumfang übernehmen, soweit zugänglich und zur Prüfung freigegeben; nicht als statistisch repräsentative Population ausgeben. Patch-/Mode-/Heroabdeckung, teilweise beschädigte und unvollständige Dateien sowie mindestens einen Schemawechsel aufnehmen. Fehlender Zugriff ist ein Gateblocker, keine fiktive Testdatei.

Pflicht-Kandidatenfelder: Match-ID, Version/Mode, Teams/Herozuordnung, Dauer/Timeline, Items, Ability-Level, Kills/Deaths/Assists, grob gesampelte Positionen und Objectives. Damage/Heal, Modifier, Farmkurven und Teamfightfenster nur aus unterstützten belegbaren Events erzeugen. Pro Patch und Parserfeature `supported`, `unavailable`, `derived` oder `unverified` ausweisen. Ein Low-Level-Decoder ist kein Beweis, dass jedes gewünschte fachliche Event zuverlässig verfügbar ist.

## 3. Datenpfad und Rustgrenze

```text
freigegebene .dem / Replayreferenz
    -> Größe/Format/Hash/Rechte prüfen
    -> unverändertes Raw-Artefakt + Source-Revision
    -> begrenzter Rust-Decoder
    -> Schema-/Capability-Validierung
    -> normalisierte Observation mit Zeit/Entity/Evidenz
    -> geprüfte Aggregate/Population/Reasoner
    -> versionierte Ergebnisse im gemeinsamen Brain
```

`haste` und `valveprotos-rs` sind U4-Kandidaten; Versionen/Features/Transitivabhängigkeiten pinnen und anhand des Pilots entscheiden. Fehlende Funktionen nicht durch notwendige Python-/.NET-/JVM-Sidecars kaschieren. Optionale Fremdparser dürfen lokal/gesondert Referenzergebnisse erzeugen; die regulären Rusttests können diese gepinnten Referenzen prüfen, ohne eine Fremdruntime zu starten.

## 4. Zweistufiger Store und Ressourcenbudgets

Komprimierte Rohreplays oder zugelassene referenzierte Artefakte mit Hash/Location/Verfügbarkeit außerhalb Git aufbewahren. Große Tickströme nicht vollständig als relationale Einzelzeilen und nicht als RAG-Chunks speichern. Selektierte normalisierte Beobachtungen und abgeleitete Aggregate dienen Anfragen; für tiefe Diagnose eine klar begrenzte Detailansicht aus dem Raw-Artefakt erzeugen.

Partitionierung und Aufbewahrung nach gemessenen Volumina wählen. Deduplikation über Replayhash plus fachliche Match-/Versionreferenz; derselbe Match aus zwei Feeds darf keine doppelte Population werden. Beobachtungsidentität berücksichtigt Parser-/Extraktionsversion und Eventposition; ein Reparse darf alte Ergebnisse nicht unbemerkt mit neuen mischen.

Replayworker: maximale Datei-/Dekompressionsgröße, Zeit-/Speicherlimit, begrenzte CPUslots, Backpressure, priorisierte interaktive Anfragen, Checkpoint/Retry und Quarantäne. Neustart/Wiederverarbeitung idempotent; fehlerhafter Datenstrom kann nicht endlos Ressourcen binden. Bei nicht sicher abbrechbarer Decoderarbeit einen isolierten, begrenzten Rust-Workerprozess vorsehen und prüfen, statt eine nicht existierende Abbruchgarantie zu behaupten.

Ziel: p95/p99 des Antwortpfads unter gleichzeitigem Parsing und Reembedding messen. Decoder-Bytes/s, Peak-RAM pro Datei, CPU pro Match, Observation-/Raw-Speicherverhältnis, Queuewartezeit und Rebuilddauer getrennt erfassen.

## 5. Observationvertrag und Validierung

Jede Observation: stabile ID, Match-/Hero-/Entityreferenz, Eventtyp, Gamezeit/Tick und Zeitbasis, Werte/Units, Source-/Replayhash, Parser-/Schema-/Extraktionsversion, Mode/Patch soweit belegt, Prüfstatus und Locator zur Rohquelle. Abgeleitete Events tragen Algorithmus und Inputreferenzen. Keine Einheiten-/Zeitbasis raten.

Für Golden Replays ausgewählte Kernfelder mit einem geeigneten unabhängigen Parser oder einer gleichwertigen kuratierten Referenz vergleichen. Gemeinsam genutzte Upstreamprotokolle/Parserteile als korrelierte Ableitung dokumentieren. Gleiche Feldnamen sind nicht automatisch gleiche Eventsemantik; Toleranzen für Zeit/Position und Definition von Kills/Objectives vor dem Vergleich festlegen.

Fixtures enthalten validierte Referenzresultate, Replays nur soweit Speicherung/Weitergabe erlaubt. Namensentfernung oder Pseudonymisierung allein garantiert keine Anonymität der .dem-Datei. Rohdateien zugriffsbeschränkt, öffentliche Testartefakte bereinigt/synthetisch oder einzeln freigegeben; Spielerbezug und Löschfristen nach Datenpolitik behandeln.

## 6. Population, Builds und Coaching

Meta-/Matchaggregat enthält Datenfenster, Patch/Mode, Kohortenfilter, n, Coverage/fehlende Daten, Selection-Bias-Hinweise und Quelle. Player-/Matchdublettenkontrolle; Trennung von Rang, Region, Teamkontext oder anderen verfügbaren Einflussgrößen, soweit für die Frage relevant und zulässig. Seltene Kombinationen nicht als belastbare allgemeine Empfehlung ausgeben.

Mechanik sagt, welche Interaktion modelliert ist; Replay zeigt einen beobachteten Ablauf; Population zeigt Häufigkeit/Ergebnis einer Stichprobe. Item-Winrate ist nicht automatisch kausaler Itemnutzen; Kaufzeitpunkt, Spielstand und Auswahlverzerrung können zusammenhängen. Keine mechanischen Hard Constraints aus Mehrheits-Winrate ableiten.

05 kann empirische Signale für Ranking und Hypothesenprüfung nutzen, muss sie getrennt von kanonischen Game-Facts ausweisen. Komplexes Coaching wie fehlerhafte Rotation oder schlechter Engage erst aktivieren, wenn Events und Bewertungsdefinition unabhängig getestet sind. Dieser Ausbau bleibt im gemeinsamen Anforderungsregister; nicht durch eine sprachlich überzeugende, ungeprüfte LLM-Bewertung ersetzen.

## 7. Zeitliche Holdouts ohne Datenleck

Entwicklungs-/Trainings- und Testfälle zeitlich und nach Match trennen. Für eine Bewertung „mit Wissen bis T“ zählen nur Quellen, deren Inhalte bis T tatsächlich verfügbar waren; zusätzlich ihre Game-Gültigkeit prüfen. Ein heute korrigiertes Wiki darf nicht rückwirkend als damals bekannt in den Trainingsinput gelangen. `observed_at/available_at` und `valid_from` sind unterschiedliche Filter.

Vergangene Knowledge-Releases bzw. verfügbare Rohrevisionen einfrieren. Gegen Beobachtungen aus T+Δ vergleichen, ohne sie beim Tuning derselben Auswertung zu verwenden. Retrospektiv rekonstruierte Fälle als solche markieren; ein heute verwendeter Parser kann fehlende damalige Observability nicht rückwirkend beweisen. Modelle, Rubriken, Entity-Mappings und ähnliche Artefakte auf Leakage prüfen.

Auswertung: fachliche Legalität/Faktentreue, Kalibrierung/Ranking unter definiertem Ziel, ausreichend große Kohorten und Unsicherheit. Die Beobachtungsdaten beweisen keine optimale Strategie für jeden Spieler. Keine jeweilige Zielmetrik nach Sichtung des Testsets ändern.

## 8. Gates und Abschluss

G1: Observation-/Replay-/Population-Port, Zeitbasis, Source Contract, Pflicht-Capabilities und Budgetprofil festgelegt.

G2: ein echter freigegebener Replayfall deterministisch durch Raw→Decoder→Observation→abfragbare Brain-Evidenz; ein korruptes Beispiel sicher begrenzt/quarantiniert. Mit anderen Pilotsourcen konsistentes Release.

G3: vereinbarte 10–20 Golden Replays bzw. ausdrücklich begründeter Umfang, Feld-/Patch-Capability-Matrix, Referenzabgleich und idempotenter Reparse. Freigegebene Match-/Populationdaten vorhanden; alle Pflichtfähigkeiten belegt oder als ungelöster Scopeblocker geführt.

G4: Tests unter gemischter Last, Restore/Reparse ohne Fremdruntime, Daten-/Rechtegrenzen, Schemawechsel und echte zeitliche Holdouts bestanden. Komplexere Coaching-/Learningfunktionen nur bei eigener fachlicher Abnahme aktivieren. Kein automatisches Training auf ungeprüften neuen Daten und kein Vervielfachen korrelierter Quellen.
