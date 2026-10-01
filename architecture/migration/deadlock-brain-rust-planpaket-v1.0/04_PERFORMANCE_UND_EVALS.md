# Performance und Qualität: messen statt vermuten

## Ziel und Freigabe

„Beste Performance“ wird im Plan als beste gemessene Kombination aus Antwortqualität, Latenz, Durchsatz, RAM, Kosten und Aktualität unter dem verfügbaren Betriebsbudget definiert. Rust ist die verbindliche Implementierungssprache, aber kein Ersatz für Lasttests. Fremdmodellwartezeit, schlechtes Retrieval oder ein unpassendes Datenmodell verschwinden nicht durch einen Sprachwechsel.

Die konkreten absoluten Serviceziele stehen nach Chat 01 in `vorlagen/BENCHMARKPROFIL.yaml`: Hardware, Datenmenge, gleichzeitige Requests/Ankunftsrate, p95/p99, Aktualitätsbudget, Betriebsbudget, Wiederanlaufziele. Solange diese Angaben fehlen, gibt es keine belastbare Aussage „erreicht x ms“ oder „x-mal schneller“. Leere SLOs sind ein Freigabeblocker, keine unbegrenzte Toleranz.

## 1. Vergleichsvarianten

A = alter geprüfter Referenzpfad mit fixierten Daten, Modellen und Requests.
B = neuer Rust-Pfad mit vergleichbarer Daten-/Retrieval-/Modellkonfiguration, ohne aktive Jev-Optimierung.
C = B plus einzelne Such-/Daten-/Cacheoptimierung.
D = freigegebenes C plus eine Jev-Funktion; weitere Jev-Funktionen einzeln hinzufügen.

So lässt sich eine Verbesserung dem Umbau, einer besseren Datenbasis oder Jev zuordnen. Ist A nicht ausführbar, wird das dokumentiert; absolute Zieltests und eine kuratierte Qualitätsreferenz bleiben möglich, aber keine erfundene Vorher-/Nachher-Steigerung.

## 2. Messkatalog

Je Profil messen: p50/p95/p99 Gesamtzeit und Time-to-First-Token, erfolgreiche Requests pro Sekunde, Fehler/Timeouts, CPU-Zeit, Peak-/Steady-RAM, Allokations-Hotspots, I/O, DB-Pool-Wartezeit, Queue-Lag, Indexgröße, Ingest/Rebuild-Dauer und Datenaktualität. Providerlatenz getrennt nach Jev, Embedding und Antwortmodell; Token und Kosten pro akzeptabler Antwort einschließlich Fehlversuchen und Retries.

Qualität: Recall@K vor dem Filter, Evidenz-Retention nach dem Filter, NDCG bzw. Rankingqualität bei vorhandenen Labels, Antwortbarkeit, Quellenkorrektheit, unterstützte Fakten, Abstention bei fehlender Evidenz und Domain-Funktionsparität. Modell-Confidence ist nicht automatisch die Wahrscheinlichkeit einer korrekten Gesamtantwort.

## 3. Testdaten und Lastprofile

Vier verpflichtende Klassen stammen aus der Recherche: einfache öffentliche Deadlock-Fragen, komplexe Builds/Coaching-Fragen, interne Architekturfragen und Code/Operations. Ergänzt werden fehlende Evidenz, widersprüchliche Versionen, Patch-/Zeitfragen, Tippfehler/DE-EN-Mischung, private Quellen, adversariale Prompttexte und Provider-Ausfälle.

Vorschlag für den Beginn: mindestens 200 versionierte Entwicklungsfälle plus ein getrennt gehaltenes Testset. Die tatsächliche Verteilung nach Chat 01 festlegen. Nicht auf Testset-Schwellen optimieren. Kritische Domain- und Leak-Fälle sind einzeln verbindlich. Ein kleiner Datensatz liefert nur entsprechend begrenzte statistische Sicherheit.

Test auf realer Zielhardware mit Produktionsvolumen und einem begründeten Wachstumsprofil, z. B. 10-facher Dokumentmenge. Kaltstart und Warmcache separat; identische vs. diverse Fragen; concurrency 1/8/32 als anfängliche Messpunkte, ergänzt um die tatsächliche Zielankunftsrate. Indexierung während Livefragen, Coldstart nach Deployment, Queueüberlast, langsamer Provider und fehlender DB-Knoten gehören dazu.

Ankunftsrate und Wartezeiten korrekt erfassen: Ein Lastgenerator, der bei langsamen Antworten einfach weniger Anfragen sendet, darf keine Überlast verbergen. Wiederholte Läufe, identische Seeds und Rohresultate aufbewahren; Varianz und Konfidenzintervalle ausweisen. Lastgenerator und Server nicht unbemerkt um dieselben CPU-Kerne konkurrieren lassen.

## 4. Erste Auswahlregeln — Vorschläge, keine bereits gemessenen Ergebnisse

| Bereich | Freigaberegel |
|---|---|
| Rechte/Secrets | null bekannte Leaks und alle definierten Negativtests bestanden; kein Beweis absoluter Fehlerfreiheit |
| Daten | keine ungeklärten Verluste in freigegebenen Quellen; Quarantäne/Ausschlüsse nachvollziehbar |
| Domain | alle erforderlichen Features und Invarianten; numerische Toleranzen vorher fachlich festlegen |
| Retrieval | kritische Quellenfälle vollständig; globale Nichtunterlegenheitsmarge vorab festlegen, Startvorschlag max. 1 Prozentpunkt Recall-Verlust |
| Jev-Relevanz | kritische Evidenz nicht verlieren; Startziel mindestens 98 % Retention relevanter bereits gefundener Evidenz, auf separaten Labels prüfen |
| Antwortqualität | keine kritische Regression; vorab definierter Score und Nichtunterlegenheitsbereich |
| Latenz/Kosten | freigegebene absolute SLOs einhalten; Optimierung nur übernehmen, wenn ihr Vorteil außerhalb des Messrauschens liegt |
| Betrieb | begrenzte Ressourcen unter Last, keine unendlichen Queues; Restore und Wiederanlauf innerhalb festgelegter Ziele |

Eine Retention von 98 % nach Jev bedeutet nicht 98 % End-to-End-Recall: fehlende Erstsuche kann der Filter nicht heilen. Kein Durchschnitt darf einen Scope-Leak, eine kaputte Domainfunktion oder einen systematischen Ausfall einer Frageklasse ausgleichen.

## 5. Optimierungsreihenfolge

**Zuerst Daten und unnötige Arbeit.** Exakte Item-/Patch-/Statistikfragen aus strukturierten Fakten beantworten; vorgefertigte Aggregate für teure Analytics; keine ganzen Quellseiten an Modelle. Unveränderte Dokumente nicht neu chunken/einbetten. Deduplikation mit erhaltener ACL und Provenienz.

**Dann I/O und Parallelität.** Wiederverwendete HTTP-/DB-Clients, begrenzte Parallelität, Backpressure, Batches, Timeouts und Cancel-Budgets. Lexical- und dense-Suche nur soweit parallelisieren, wie sie nicht dieselbe knappe Ressource überlasten. CPU-lastiges Chunking/Ranking in begrenzte CPU-Pools; Worker und Livepfad bekommen getrennte Ressourcenbudgets. Die Tokio-Dokumentation warnt vor unbeschränkter CPU-Arbeit im Blocking-Pool [E3].

**Dann Retrieval.** BM25/Tantivy gegen FTS-Baseline; dense mit gleichen Embeddings; Hybridfusion, Metadaten-/ACL-/Patchfilter und Nachbarschaftskontext. Je Kandidat dasselbe Corpus und dieselben Labels. pgvector dokumentiert, dass Filter bei ANN-Scans die Treffermenge und Recall beeinflussen können; deshalb den echten Queryplan mit restriktiven Filtern messen [E4]. PostgreSQL-FTS wird nicht als BM25 ausgegeben [E5/E8].

**Dann Jev.** Relevanz nach Retrieval und Feeder-Klassifikation zuerst. Mehrere unabhängige Entscheidungen pro passendem State bündeln; keine unnötigen seriellen Netzwerkrunden und kein HTTP-Aufruf pro Chunk als ungemessener Default. Query-Routing, Answerability und Model-Routing getrennt aktivierbar. Bei Ausfall sichere Rust-Baseline mit erlaubtem Evidenz-/Kostenbudget oder kontrolliert keine Antwort. Jev darf kein Single Point of Failure für Berechtigungen sein.

**Dann Caches und Feintuning.** Zuerst exakte versionierte Caches und Request-Zusammenfassung für identische laufende Arbeit. Ein Answer-Cache hängt u. a. von ACL-Fingerprint, Policyversion, Corpusrelease, Frage, Sprache, Antwortprofil und privatem Gesprächskontext ab. Cachelesen prüft die aktuelle Berechtigung. Semantische Antwortcaches zunächst nicht einführen. Quantisierung, mmap, spezielle Allokatoren, LTO/PGO oder weitere Datenbankdienste erst nach Profiling und Qualitätsvergleich.

## 6. Jev-Auswertung ohne Selbsttäuschung

Relevanzlabels von unabhängig bewerteten Quellen beziehen, nicht Jev mit sich selbst benoten. Schwellen auf Entwicklungsdaten kalibrieren, auf zurückgehaltenen Fällen auswerten. Provider-/Modellrevision, Fragenrubrik und kompakter State gehören in den Trace. Testen, ob gekürzte Kandidaten gerade die entscheidende Passage verlieren. 20–50 Kandidaten und 5–8 Evidenzen sind Startparameter aus der Recherche, keine harten Naturgesetze.

Jev bleibt bei keinem nachgewiesenen Vorteil deaktiviert oder auf die profitablen Einsatzfälle begrenzt. Das System erfüllt die Integrationsaufgabe dann technisch, aber behauptet keinen unbelegten Produktivnutzen.

## 7. Ergebnisformat

Jeder Bericht enthält Basis-/Kandidatencommit, Hardware, komplette Konfiguration, Source-/Index-/Modellversionen, Dataset-Hash, Lastprofil, Rohmessdaten, Wiederholungen, Qualitätsauswertung und Entscheid. Geschwindigkeit mit reduzierter Datenmenge, schwächerem Modell oder schlechterer Qualität darf nicht als isolierter Rust-Vorteil ausgegeben werden.

Quellenkürzel siehe `quellen/QUELLEN_UND_ANNAHMEN.md`.


## 8. Ergänzte Messprofile für Wiki, Builds und Replays

Die vorhandenen v0.9-Vergleiche bleiben bestehen. Neue Quellen/Funktionen nicht gleichzeitig mit neuen Modellen und Jev aktivieren und den Gesamtgewinn ausschließlich Rust zuschreiben. Für neue Fähigkeiten ohne Altbaseline absolute Ziele und kuratierte Referenzen festhalten.

| Pfad | Verpflichtende Zusatzmessung |
|---|---|
| Wiki-/Git-Delta | Abruf/Parse/Normalize/Validate/Publish-Zeit; unveränderte Revision löst keine unnötige Neubearbeitung aus |
| Hero-Wissenskarte | Zeit/Bytes/Token einer kompakten Karte, Fact-/Patchkorrektheit und selektive Invalidierung |
| Build Planner | CPU/RAM/p95 je Kandidatenumfang, Constraints, Suchbudget, deterministische Reproduktion, kleine Fälle gegen exakte Referenz |
| Source Contract/Schema-Watch | Drift-/Semantikfehler erkennen; keine falsche Veröffentlichung bei Endpointausfall oder unvollständigem Snapshot |
| Replayworker | MB/s bzw. Matches/s, Peak-RAM, Dekompressions-/Zeitschranken, Queue-Lag, idempotenter Reparse |
| Population/Analytics | Kohortengröße, Queryplan, Aggregationszeit, Missingness, Patch-/Zeitschnitt und Holdout-Leakage |
| Gemischte Last | interaktive p95/p99 bei gleichzeitigem Wiki-/Gitupdate, Replayparsing und Reembedding; Priorisierung nach freigegebenem Budget |

U4 nennt bereits deutschsprachige Support-Evals in Docs. 01/10 prüfen tatsächlichen Bestand und Versionsbindung und übernehmen ihn; die historische Anzahl ist kein harter Test- oder Coveragewert. Öffentliche Support-, interne Architektur-, Zahlen-/Build-, Mechanik-/Patch- und Replayfälle getrennt bewerten.

RAG-Evals zusätzlich: strukturierte Fact-/Aliasauflösung, gezielte Mechanikbeziehungen, gemischte Facts+Prosa, unbekannte Entities und veraltete Karten. Ein kleiner Prompt allein ist kein Erfolg, wenn entscheidende Regeln oder Quellen fehlen. Jev darf hard constraints, Rechte oder belegte Rechenwerte nicht herausfiltern/ersetzen.

Empirische Evals nutzen getrennte Matches/Zeitfenster und nur bis Cutoff verfügbare Daten. Korrelierte Ableitungsketten nicht als unabhängige Ground-Truthvotes werten. Replay-/Buildqualität wird nicht allein vom gleichen LLM bewertet, das die Antwort erzeugt.

Vorgeschlagene harte Testkriterien aus U3: 100 % Fact-/Quellenreferenzen für numerische Golden-Aussagen, keine verwaisten Kernreferenzen und 100 % Einhaltung aller modellierten Hard Constraints. Das sind Tests auf den vereinbarten Fällen, kein Beweis lückenloser Korrektheit im gesamten Spiel.
