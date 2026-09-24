# Gesamtdatenmigration: erfassen, übernehmen, prüfen, fortführen

## 1. Daten nicht erst nach dem Code planen

Die Datenmigration ist ein Hauptstrang derselben Umstellung. Chat 01 erfasst den Bestand, Chat 03 besitzt Modell und Migration, Chat 04 die gemeinsame laufende Versorgung mit den Quellenmodulen aus 12–14. Chat 06 bestimmt mit 03 die Suchprojektionen, Chat 05 die fachlichen Datensätze, Chat 10 prüft unabhängig. Ein Funktionsport gilt nicht als fertig, solange seine produktiv benötigten Daten fehlen.

Die folgende Matrix beschreibt zu prüfende Klassen. Die Recherche belegt nicht, dass jede mögliche Runtime-Datenklasse tatsächlich existiert; unbekannte Einträge sind Inventaraufträge und werden nicht als vorhandene Tabellen ausgegeben.

| Klasse | Ziel | Behandlung |
|---|---|---|
| öffentliche Deadlock-Dokumentation | kuratierte Knowledge Source | Inhalte, Anhänge, Links, Sprache, Quelle und Version erhalten |
| internes Second Brain | berechtigtes internes Corpus | Systeme/Projekte/Entscheidungen/Historie getrennt von Öffentlichkeit |
| generierte Daily-/Session-/GitHub-Digests | generiertes Corpus | Elternquellen, Modellversion, Erstellungszeit und Vertrauensklasse erhalten |
| Code-/Konfigurationswissen | verknüpfte Code-Snapshots | Repository, Pfad, Commit und Zugriff; keine Secret-Inhalte indexieren |
| Heroes/Items/Builds/Patches und andere Domainfakten | strukturierter Domainstore | IDs, Einheiten, Patchgültigkeit, Fremdschlüssel und Herkunft |
| vorhandene Matches/Analytics/Training-/Learning-Daten | strukturierter Store bzw. versionierte Rohobjekte | nicht jeden Datensatz ungeprüft zu RAG-Text machen |
| gelernte Artefakte/Parameter/Lineage | versioniertes Artefaktregister | Eingabedatenversion, Algorithmus/Modell, Hash und Reproduzierbarkeit |
| Conversations/Feedback/Präferenzen, sofern vorhanden | private Laufzeitdaten | Besitzer, Löschung, zulässige Aufbewahrung, Schemaübernahme |
| Feeder-Scanstate/Sync-Cursor/Jobs | zentraler Job-/Checkpoint-Store | Watermarks, Wiederanlauf, doppelte Ereignisse und Deletes |
| bestehende Embeddings/Chunk-/Suchindizes | abgeleitete Projektion | nur bei nachgewiesener Kompatibilität wiederverwenden, sonst neu erzeugen |
| Caches und temporäre Dateien | abgeleitet/wegwerfbar | normalerweise nicht migrieren; Kaltstart-/Warmup-Plan |
| Logs/Traces/Betriebsmetriken | geschützter Observability-Store | nach freigegebener Aufbewahrung, nicht automatisch Knowledge |
| Tokens, Secrets und Zugangsdaten | Secret-Verwaltung | niemals ins Corpus oder Git; Verweise/Rotation gesondert planen |
| Assets, Anhänge und Binärdateien | versionierter Objektspeicher | Referenzen, Zugriffsrechte und Prüfsummen mitübernehmen |

## 2. Source Registry und unverwechselbare Identitäten

Für jede Quelle wird `vorlagen/DATENINVENTAR.csv` ausgefüllt: tatsächlicher Ort und verantwortlicher Dienst, Source-of-Truth-Status, Datentyp, Volumen, Schema, Version/Commit, Besitzer, ACL, Egress-Freigabe, Änderungsfrequenz, Löschsignal, Exportverfahren, Ziel und Prüfverfahren.

Identitäten dürfen nicht nur aus einem Text-Hash bestehen. Derselbe Text kann in unterschiedlichen Quellen mit verschiedenen Berechtigungen vorkommen. `document_id` identifiziert ein logisches Quelldokument; `revision_id` eine konkrete Version; `chunk_id` umfasst mindestens Revision, Chunking-Version und stabile Position. Deduplikation kann Speicher sparen, darf aber Quellbezüge, ACLs und Gültigkeiten nicht verschmelzen.

Quellzeit, Beobachtungszeit und Ingestzeit sind getrennte Felder. Ein Import am heutigen Tag macht eine alte Architekturentscheidung nicht aktuell. Bei unbekannter Quellrevision wird der exakte Snapshot-Hash plus Beobachtungszeit gespeichert; kein Commit wird erfunden.

## 3. Kanonisches Modell und abgeleitete Daten

Vorzusehen sind Source Registry, Raw-Snapshot-/Objektregister, Dokumentrevisionen, normalisierte Domainrecords, Chunks, Provenienzbeziehungen, ACL-/Policy-Versionen, Ingestjobs, Checkpoints, Tombstones, Embedding-/Indexgenerationen und aktiver Release-Manifestzeiger.

Ein `CorpusRelease` referenziert zusammenpassende Dokument-/Domain-Snapshots, Chunking-/Embedding-Konfigurationen und lexical-/dense-Indexgenerationen. Ein Request pinnt ein Release. Ein Lexical-Treffer aus Generation N wird nicht stillschweigend mit einem Embedding aus N−1 und einer geänderten Quelle beantwortet.

PostgreSQL ist ein möglicher kanonischer Store, nicht die bereits bewiesene Bestandsdatenbank. Große unveränderliche Rohobjekte können außerhalb der DB gespeichert werden. Suchindizes sind Projektionen, keine einzige Kopie der Originaldaten. Git enthält nur geeignete Quellen, Schemas, Konfiguration und kleine bereinigte Testfixtures.

Für Domainantworten gibt es typisierte Fakten-/Berechnungsevidenz: Datasetversion, Eingabehash, Algorithmusrevision, Einheiten und Resultat. Ein Zahlenwert muss nicht erst über eine generierte Markdown-Zusammenfassung zurückgewonnen werden.

## 4. Sichtbarkeit und erlaubte Verarbeitung

Vor dem ersten Import die tatsächliche Sichtbarkeit des Zielrepos prüfen. Bei einem öffentlichen Code-Repo bleiben interne/private Inhalte im geschützten Datenstore. Bei einem privaten Repo dürfen dort nur Daten liegen, die allen berechtigten Repositorylesern zugänglich sein sollen. Ein einziges aktives Code-Repository verlangt kein einziges physisches Datenlager.

Generierte Inhalte übernehmen mindestens die strengste Vertraulichkeit ihrer Eltern und höchstens deren gemeinsame Leseberechtigung. Auch Embeddings, Extrakte und Prompttraces gelten als abgeleitete sensible Daten. Egress zu Jev, LLM und Embeddinganbieter braucht eine eigene Allowlist. Ein Benutzer darf eine interne Quelle lesen können, ohne dass diese deshalb an einen externen Modellanbieter gesendet werden darf.

Quarantäne schützt unbekannte/defekte Inhalte; sie ist kein Nachweis erfolgreicher Migration. Kritische Quellen mit ungeklärter Quarantäne blockieren die Umschaltung. Weglassen/Löschen braucht einen begründeten, freigegebenen Entscheid. Keine pauschale Übernahme vermeintlich nützlicher privater Sessiondaten.

## 5. Ausführung in überprüfbaren Schritten

### M0 — Bestand einfrieren und Restore beweisen

Repos auf Commitstände, Datenquellen auf exportierbare Snapshots bzw. konsistente Checkpoints pinnen. Jede Quelle hat einen eigenen Watermark; ein Zeitstempel über verschiedene Systeme behauptet keine gemeinsame Transaktion. Backups mit Hashmanifest erfassen, Wiederherstellung tatsächlich erproben. Alte Systeme zunächst weiter betreiben, aber Migration nicht aus sich beliebig verändernden Dateien speisen.

### M1 — Probe und Vertragsfreigabe

Eine repräsentative Mischung aus öffentlichen/internen Docs, Domainrecords, großen/kleinen Quellen, Umlauten, Code, Anhängen und historischen Versionen verarbeiten. IDs, ACLs, Herkunft, Referenzen und Transformationsregeln prüfen. Such-/Embeddingoptionen zunächst auf dieser Probe vergleichen. Vor der vollständigen Neueinbettung Modell und Schema fixieren.

### M2 — Rust-Migrationswerkzeug

`brain-admin` erhält geplante Funktionen `inventory`, `export`, `migrate`, `verify`, `rebuild` und `replay`. Konkrete CLI-Flags werden in Chat 03 spezifiziert und getestet, nicht aus diesem Dokument als bereits vorhanden vorausgesetzt.

Der Runner muss Dry-Run, begrenzte Batches, Jobs/Leases, Checkpoints, Idempotenz, kontrollierte Retries, Abbruch/Wiederanlauf und Fehlerberichte besitzen. Erfolgreicher Persistenzschritt und Verarbeitungsfortschritt gehören in eine sichere Transaktions-/Outbox-Grenze. Ein Absturz zwischen Speichern und Bestätigen darf weder Daten verlieren noch Doppelverarbeitung fachlich wirksam machen.

### M3 — Snapshot übernehmen

Zunächst die Original-/kanonischen Daten übernehmen und deren Referenzen abgleichen. Danach abgeleitete Chunks/Embeddings/Indizes erzeugen. Einmalige Fremdformatkonvertierung darf ein dokumentiertes Hilfsskript verwenden; der regelmäßige Import und der neue produktive Rebuild brauchen es nicht. Historienimporte vorab auf Secrets und unzulässige öffentliche Daten prüfen; bei notwendigem History-Rewrite alte und neue Revisionen verknüpfen.

### M4 — Änderungen und Löschungen nachziehen

Ab Snapshot-Watermark neue/geänderte/gelöschte Datensätze replayen. Bei Quellen ohne zuverlässigen Änderungsstrom regelmäßig Manifest-Differenzen bilden und für den finalen Abgleich ein Schreibfenster sperren. Keine Delta-Fähigkeit erfinden. Revisionen monoton übernehmen; alte verspätete Events dürfen eine neuere Löschung nicht rückgängig machen.

Delete/ACL-Revoke aktualisiert sofort eine Berechtigungs-/Tombstone-Sperre und Cacheinvalidierung. Der physische Neuaufbau großer Indizes darf später folgen, aber die alte Passage darf schon vorher weder Retrieval, Jev, LLM noch Zitat-Endpoint erreichen. Auch noch gepinnte ältere Releases beachten die aktuelle Sperre.

### M5 — Release publizieren

Die neue Index-/Datenkombination zunächst unsichtbar aufbauen. Alle erforderlichen Projektionen und Checkpoints müssen ready sein. Anschließend den aktiven Manifestzeiger atomar bzw. über Compare-and-Swap ändern. Repliken melden geladene Generationen; alte Leser werden kontrolliert ausgelaufen. Nicht kompatible Generationen ergeben einen expliziten Fehler statt einen unbemerkten Mix.

### M6 — Writer-Cutover

Alte Writer anhalten oder eindeutig fencen, letzten akzeptierten Eventoffset festhalten, verbleibendes Delta verarbeiten und Abgleich wiederholen. Dann genau den neuen Writer freigeben. Das Abschalten der alten Feeder-Cronjobs ist Teil dieses Schritts. Ein neues UI allein verhindert keine doppelten Hintergrundschreibvorgänge.

## 6. Vollständigkeitsnachweis

Für jede Quellklasse gilt ein protokollierter Abgleich: Jeder logische Quelldatensatz im fixierten Snapshot ist übernommen, auf einen Duplikatdatensatz abgebildet, nachvollziehbar quarantänisiert oder ausdrücklich ausgeschlossen. Jede Transformation dokumentiert ihre Kardinalität; eins-zu-viele Chunking darf nicht als Count-Abweichung fehlinterpretiert werden.

Zu prüfen: Originalhashes, normalisierte Kontrollsummen, IDs, Referenzintegrität, Anzahl pro Status/Scope/Patch, Zeitfenster, Anhangsreferenzen, Löschungen, ACLs, Jobs/Checkpoints und fachliche Kontrollaggregate. Reine Zählgleichheit reicht nicht. Für abgeleitete Indizes zusätzlich alle vorgesehenen Revisionen/Chunks und dieselbe aktive Generation prüfen.

Migration zweimal ausführen: Der zweite unveränderte Durchlauf darf keine neuen fachlichen Duplikate erzeugen. Zusätzlich Absturz in mehreren Stufen provozieren, fortsetzen und dasselbe überprüfte Ergebnis erhalten.

## 7. Rebuild und Rollback

Ein neues leeres Ziel muss aus gesicherten erlaubten Quellen, fixiertem Schema und Konfiguration vollständig wiederherstellbar sein. Embedding-Rohoutputs als versionierte Artefakte sichern, wenn ein Provider keine reproduzierbare alte Modellrevision garantiert; ein erneuter API-Aufruf ist dann ein neuer Build, kein behaupteter bitidentischer Rebuild.

Rollback ist eine kompatible Kombination aus Binary, Schema, Datenrelease, Policy und Checkpoint. Neue Writes nach Cutover separat nachvollziehbar halten. Code zurückstellen und neue Writes verlieren ist kein erfolgreicher Rollback. Ist Rückwärtskompatibilität nicht vorhanden, Schreiben sperren, Export/Replay/Restore durchführen und erst nach Abgleich wieder öffnen. Aktuelle Löschungen, ACL-Revoke und Secret-Rotation werden dabei niemals rückgängig gemacht.


## 8. Wiki-, Gamefile- und Replaybestände im selben Migrationslauf

Die Quellenbesitzer sind jetzt 04 (Worker/Feeder), 12 (Wiki), 13 (externe Feeds) und 14 (Replays). 03 bleibt alleiniger Schema-/Migrationsbesitzer. Alle laufen über Source Contract v2, nicht über parallele Bestandsdatenbanken.

Zusätzlich zu Abschnitt 1 explizit erfassen: Wiki-Namespaces/Templates/Module/Revisionen und Abhängigkeiten; Hero-/Ability-/Item-/Mechanik-Facts und Varianten; Rules/Effects/Graphkanten; Alias/Lineage und Sprachtexte; generierte Hero-Wissenskarten; Git-Gamefile-Historie und Upstream-Generatorversion; gepinnte OpenAPI-/Source-2-/Protobufschemata; Replays/Observations, Populationkohorten, Golden-Referenzen und Frozen-Holdouts. Medien-/Rohreplaydaten nur gemäß freigegebener Politik speichern.

Vollständigkeit pro Quelle **und** pro Fachfunktion messen: entdeckte Seiten ≠ validierte Facts ≠ auslieferbare Karten/Builds. `QUELLENREGISTER.csv`, `WIKI_COVERAGE.csv`, `REPLAY_CAPABILITIES.csv` und `DATENABGLEICH.csv` gemeinsam führen. Quarantäne ist kein Erfolgsstatus; unzugängliche Pflichtquellen bleiben Blocker, bis eine ausdrücklich freigegebene Scope-/Ersatzentscheidung vorliegt.

Raw-Hashes, normalisierte semantische Hashes und Versionsketten getrennt halten. Parserkorrektur bei gleichem Raw-Hash ist kein Spielpatch. Kopien desselben Ursprungs erhalten mehrere Sourcebezüge, aber nicht mehrere unabhängige Evidenzstimmen. Factupdates invalidieren abhängige Regeln/Graphprojektionen/Karten/Caches und nur tatsächlich geänderte Text-/Embeddingprojektionen.

G2 ist vor teurem Vollimport aller Wiki-/Git-Historien, Replaystapel und vollständigem Reembedding Pflicht. Danach inkrementelle Batchübernahme mit Watermarks, Restarts, Reconciliation und letzter konsistenter Knowledge-Version. Keine unkontrollierte Live-Wiki- oder Replaysammlung im interaktiven Antwortpfad.

Knowledge-Releases enthalten jetzt auch Rules, Graph-/Kartenmaterialisierungen, Parser-/Schemafamilien und freigegebene empirische Daten. Schemarollback und Datenrollback getrennt prüfen; aktuelle Löschungen/ACLs bleiben erhalten. Ungeklärte Lizenz-/Exportrechte blockieren betroffene Publikation auch dann, wenn technischer Import funktioniert.
