# S06 — Pilotprotokoll und Suchentscheidungsentwurf

Status: **Entwurf / nicht ausgeführt / keine Such- oder Modellentscheidung**.
Basis, Gates, Befunde und Testergebnisse: [README](README.md).
Dieses Dokument beschreibt das Vorgehen nach Freigabe. Es ist weder ein zusätzlicher Laufzeitvertrag noch eine Freigabe für neue Abhängigkeiten.

## 1. Vor jeder Messung: tatsächlich reproduzierbarer Freeze

S01 liefert Quellen-/Rechteinventar, aktiven Referenzpfad und Hardwareprofil. S03/S04 liefern einen zugelassenen, unveränderlichen Corpus mit kanonischen IDs, Revisionen, Tombstones und Knowledge-Release. S05/S12 liefern passende Fact-/Graph-/Kartenrevisionen. S07 liefert den gemeinsamen Embeddingport und eine nachweisbare feste Modellkonfiguration. S10 hält unabhängige Relevanzlabels, Querysplit und Qualitätsgrenzen fest. S00 bestätigt die Gates und Besitzer.

`pilot-manifest.json` ist nur eine Checkliste dieser Eingänge. Der Code-Commit und der Planpakethash sind bekannt; **Corpus und Labels sind noch nicht gefroren**. Leere Pflichtfelder dürfen nicht mit erfundenen Releasenamen, einem Hash leerer Daten oder selbst bewerteten Retrievalergebnissen geschlossen werden. `CASES.csv` ist ein Fallentwurf und ersetzt weder Quellen noch Labels.

Der spätere Freeze enthält mindestens:

| Bereich | Verbindlicher Nachweis vor Ausführung |
|---|---|
| Code und Umgebung | integrierte Basis-/Kandidatencommits; Contract-/Schema-Version; Lockfilehash; freigegebene Toolchain; Betriebssystem, CPU/RAM und DB-/Indexversionen |
| Corpus | Manifest und Hash; Knowledge-Release; Source-/Document-/Chunk-/Fact-/Rule-/Kartenrevisionen; Parser/Chunker; Rechte, Egress und kontrollierte Artefaktreferenzen |
| Embedding | Anbieter/Modell und aufgelöste Revision; Tokenisierung, Präfixe und Vorverarbeitung; Dimension, Pooling, Normalisierung und Distanz; Vektorhash bzw. versionsgebundener Cache |
| Labels | Query-/Labelhashes, Relevanzdefinition, unabhängige Bewertung durch S10, unbeurteilte Fälle, Entwicklungs-/Holdoutsplit und Quellenfreigabe |
| Messung | Seed, Wiederholungen, Reihenfolge, Kalt-/Warmzustand, Lastprofil, Kandidaten-/Token-/Zeit-/RAMbudgets, Messgrenzen und vorab freigegebene Nichtunterlegenheit |

Public, Internal und Private bleiben getrennt. Rohtexte aus privaten Quellen, Rohreplays, interne Pfade, Secrets und nicht bereinigte Querylogs werden nicht in Git oder öffentliche CI-Artefakte übernommen. Rechte gelten auch für einen kleinen Pilot.

## 2. Begrenzte Vergleichsmatrix

Jede Variante nutzt dieselben **zulässigen** Dokumentrevisionen, Querys und unabhängigen Labels. Hat die Altbaseline nur Mechaniknotizen, wird ein identischer gemeinsamer Teilcorpus separat verglichen. Die Gesamtcorpusqualität ist ein zusätzlicher Test, kein direkt vergleichbarer Gewinn gegen einen kleineren Altcorpus.

| Variante | Zweck und Status |
|---|---|
| L0 | Bestehende Rust-Claims-Heuristik bzw. Wiki-Dateiscan als getrennt benannte Referenzpfade. Keine Umbenennung zu FTS/BM25. |
| L1 | Einfache PostgreSQL-FTS als begrenzter Kandidat; Schema-/Indexänderungen ausschließlich durch S03. |
| L2 | Tantivy/BM25 als ein lexical Kandidat, nur mit S02 abgestimmter Abhängigkeit und identischem zulässigem Corpus. |
| D0 | SQLite-vec-Altpfad nur nach S01-Nachweis von aktivem Bestand, erfolgreicher echter Vektorsuche und reproduzierbarem Modell. Keine neue Python-Betriebsabhängigkeit. Nicht ausführbar bedeutet sichtbar `unavailable`, nicht null Recall oder null Laufzeit. |
| D1 | pgvector als ein Dense-Startkandidat mit identischen eingefrorenen Vektoren. Zuerst exakte Suche zur Kontrolle; danach höchstens eine begründete ANN-Konfiguration unter denselben Filtern. |
| H1 | Erst nach L-/D-Einzelmessungen: deterministische Fusion plus autorisierte strukturierte Evidenz. |
| J1 | Erst nach H1: genau ein abschaltbarer Jev-Relevanzschritt, zunächst Shadow, gemeinsam mit S07/S08/S10. |

Storevergleich und Modellvergleich sind zwei Experimente. Identische Dimension reicht nicht für Modellkompatibilität. Alte Vektoren ohne belegte Modellrevision nicht mit neuen Queryvektoren mischen. Bei fehlender reproduzierbarer Altbaseline bleiben absolute Qualitäts-/Sicherheitsziele möglich; eine Vorher-/Nachher-Steigerung wird dann nicht behauptet.

## 3. Autorisierter Ablauf nach den gemeinsamen G1-Verträgen

1. Vom Kernel den verifizierten Kontext und genau ein gepinntes Knowledge-Release übernehmen. Requested Scopes, Routing und Kandidatenbudgets dürfen den serverseitig erlaubten Bereich nur einschränken. Query-Egress für Embeddings vor dem Provideraufruf prüfen; auch der Querytext kann vertraulich sein.
2. Exakte Entities/Facts, passende Mechaniknachbarn, lexical und dense über die festgelegten Ports lesen. Patch, Mode, Locale/Aliasse, Source-/Object-ACL, aktuelle Löschungen und gültige Revisionen berücksichtigen. Numerische Ergebnisse kommen aus S05 mit Inputreferenzen, Einheiten und Algorithmusversion, nicht aus Kartenähnlichkeit.
3. Bei zu wenigen berechtigten ANN-Treffern nur innerhalb desselben Scopes, Releases, Filters und Budgets begrenzt erweitern. Wiederholungen, gescannte Kandidaten, berechtigte Treffer und Erschöpfungsgrund erfassen. Kein Abschalten von Filtern, kein unbeschränktes Nachladen und kein stiller Wechsel in ein anderes Release.
4. Aktuelle Rechte/Tombstones vor der Evidenzweitergabe erneut prüfen. Bei ACL-/Releaseprüffehlern sicher abbrechen oder ausdrücklich erlaubten konsistenten Fallback nutzen. Unverifizierte oder nicht passende Indexgenerationen nicht ausgeben.
5. Vor jedem Jev-/LLM-Aufruf zusätzlich den Egress für diesen konkreten Provider prüfen. Ein zugriffsberechtigter Nutzer bedeutet nicht automatisch eine erlaubte externe Übermittlung. Jev darf weder Rechte, Quellen noch Evidenz-IDs hinzufügen.
6. Vor finaler Ausgabe/Zitaten erneut die aktuelle Berechtigung prüfen. Cachehits, Graphnachbarn und historische Releases müssen diese Schranke ebenfalls passieren. S08 besitzt die letzte Kernel-/Zitatprüfung; S06 liefert die erforderlichen unverfälschten Referenzen.

Dynamische Widerrufstests haben einen eigenen zeitlichen Ablauf: Widerruf vor Retrieval, zwischen Retrieval und Jev, zwischen Jev und Antwort, sowie nach Cachebefüllung. Sie werden nicht in eine statische Recallmessung hineingerechnet, deren Goldmenge eine andere Policyzeit verwendet.

## 4. Fusion und Deduplikation als zu prüfende Variante

Für H1 wird RRF als Kandidat betrachtet: `score(e) = Summe(1 / (k + Rang(e)))`, mit Rängen ab 1 und festgehaltenem `k`. Der Parameter wird nur auf Entwicklungsdaten ausgewählt. Keine behauptete Qualitätsverbesserung vor der Messung.

Ein Evidenzobjekt zählt je Retriever höchstens einmal; Eingabereihenfolge oder mehrfach gelieferte Treffer dürfen den Rang nicht verändern. Bei gleichem Fusionsscore entscheidet eine im gemeinsamen Vertrag stabile, reproduzierbare Evidence-ID-/Revisionsordnung, nicht Hash-Map-Reihenfolge. Eine gleichlautende ID mit abweichender Revision, Sichtbarkeit oder Provenienz ist ein Konflikt, kein stilles Überschreiben.

Gleicher Contenthash darf keine ACLs, Source-IDs oder Herkunftsfamilien verschmelzen. Berechtigte und gesperrte Kopien werden nicht zu einer erweiterten Sichtbarkeit vereint. Korrelierte Ableitungen können zur Erklärung erhalten bleiben, zählen aber nicht als unabhängige Bestätigung. Graphnachbarn benötigen dieselbe Freigabe wie direkte Treffer und ein festes Hop-/Kandidatenbudget.

## 5. Messgrößen ohne Scheinerfolge

**Relevanz:** Recall@K gegen die unabhängig gelabelte, im fixierten Kontext berechtigte und zeitlich gültige Goldmenge. Ein verbotener Treffer zählt als Sicherheitsfehler, nie als Recallgewinn. Bei leerer Goldmenge wird Abstention/False-Positive-Verhalten gemessen, nicht künstlich Recall=1. Unbeurteilte Treffer nicht still als irrelevant etikettieren. Graded nDCG/MRR nur mit passender, dokumentierter Labeldefinition.

**ANN-Treffertreue:** Separat von menschlicher Relevanz gegen die exakte Dense-Top-K unter denselben Scopes und Filtern vergleichen. Kandidatenunterfüllung, leere Ergebnisse, Timeouts und Fehler pro Filter-/Sprach-/Patchklasse ausweisen. Gute Latenz bei verlorenen Treffern ist keine erfolgreiche Optimierung.

**Jev:** Retention der zuvor gefundenen relevanten Evidenz und End-to-End-Recall getrennt berichten. Kritische Fakten, Berechnungen und Hard Constraints nicht zugunsten eines guten Durchschnittswerts verlieren. Nicht auf dem Holdout kalibrieren.

**Betrieb:** p50/p95/p99, Fehler/Timeouts und tatsächlich erreichte Last; DB-/Queuewartezeit, CPU, Peak-/Steady-RAM, Indexgröße, Aufbau-/Delta-/Wiederaufbauzeit, Providerlatenz/Token/Kosten einschließlich fehlgeschlagener Versuche. Warm-/Kaltlauf und gleichzeitige Indexierung/Replayarbeit getrennt. Der Lastgenerator darf Überlast nicht durch sinkende Ankunftsrate verbergen. Rohresultate und Varianz mit unveränderlichem Runmanifest aufbewahren.

Die im Plan genannten Startwerte wie Recall-Marge, Jev-Retention, Kandidatenbudgets und Concurrency-Punkte sind **Vorschläge, keine hier freigegebenen Grenzen**. Konkrete Werte und statistischer Vergleich kommen von S10 nach G0. Unbekannte Kosten oder fehlende Messungen werden nicht mit 0 ersetzt.

## 6. Reproduzierbarer Indexbau und gezielte Aktualisierung

Nach G1 setzt Rust den Indexbau auf S03-Revisionen und S04-Jobs auf. Reihenfolge, Tokenizer, Chunker, Modell und Generation sind gepinnt. Unveränderter Inhalts-/Konfigurationsfingerprint erzeugt kein neues Embedding; Parser-/Modell-/Chunkeränderungen invalidieren die tatsächlich betroffenen Ableitungen. Dokument-/Fact-/Kartenänderungen werden über die gemeinsame Abhängigkeitsprovenienz verfolgt, nicht durch einen zweiten unabhängigen Releasezeiger.

Kompakte Abschnitte und erklärende Prosa indexieren; keine pauschalen Hero-Megakarten und keine Rohreplay-/Tick-Embeddings. In Staging bauen, vollständige Manifest-/Rechte-/Generationsprüfung vor Veröffentlichung, dann atomare Veröffentlichung über S03/S04. Crash vor Publish, deterministischer Rebuild aus leeren abgeleiteten Stores und Rollback unter **aktuellen** ACLs sind Pflichtfälle.

## 7. Such-/Embedding-ADR: Entscheidung bewusst offen

ADR-Entwurf S06-SEARCH-001, Status `proposed`, Entscheider S00 mit S02/S03/S04/S07/S10.

Noch nicht entschieden: lexical Stack; Vectorstore/ANN-Verfahren; Modell-/Tokenizerrevision und Vorverarbeitung; Dimension/Distanz/Normalisierung; Chunking; Kandidatenbudgets; RRF-Parameter; Indexkonfiguration; Ressourcenbudgets; Jev-Aktivierung.

Die spätere Annahme muss auf tatsächliche Run-IDs, Commits, Corpus-/Label-/Modellhashes, kritische Negativfälle und vorab freigegebene Nichtunterlegenheit verweisen. Abgelehnte Varianten und Messgrenzen werden mit dokumentiert. **Kein Voll-Reembedding, keine Vollmigration und kein zweiter produktiver Suchstack vor G2-Entscheid und Übergabe an S03/S04.**
