# Chat 04 · Rust-Ingestion, Connectors und Brain Feeder

**Startbedingung:** Nach G1 parallel; echter Pilot für G2, Vollversorgung bis G3.

## Kontext zum Mitgeben

Source Registry, Contracts/Storageports, Datenschutz-/Egress-Regeln, Chunk-/Embeddingentscheidung aus 06, Providerport aus 07.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **04 — Rust-Ingestion, Connectors und Brain Feeder** im Umbau von Deadlock Brain zuständig.

**Ziel:** Alle freigegebenen Quellen werden vom neuen Rust-System laufend gepflegt. Der Feeder verlässt Docs und benötigt keine Pythonlaufzeit.

**Deine Eigentümerschaft:** `crates/brain-ingestion/`, `apps/brain-worker/`, gemeinsame Connector-Laufzeit und Feeder-Connectoren; fachliche Wiki-/API-/Replaymodule gehören 12/13/14, keine konkurrierenden DB-Schemaänderungen.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Implementiere nur nachgewiesene Quellenadapter für Dateien/Docs, interne GitHub-Ereignisse und freigegebene Sessionquellen; Auth, Pagination, Quellversion, Limits, Zeitouts und Fehlerzustände einheitlich abbilden.
2. Normalisiere Text/Metadaten deterministisch. Erhalte Überschriften, Tabellen, Code und Quellenpositionen. Chunkgröße und Nachbarschaftskontext über Evals wählen, nicht als starre universelle Zahl setzen.
3. Dedupliziere mit getrennten Quellenrechten. Veränderte Daten aktualisieren, unveränderte nicht neu einbetten; Delete und ACL-Revoke zeitnah sperren und Projektionen/Cache nachführen.
4. Plane Jobs mit begrenzten Ressourcen, Checkpoints, Replay und sicherem Persistenz-/Ack-Verhalten. Aktive Livefragen dürfen nicht durch Voll-Reembedding verdrängt werden.
5. Trenne Feeder-Relevanzentscheidung von generativem Schreiben. Jev hinter DecisionProvider zunächst Shadow; generierte Zusammenfassungen nur aus erlaubten Elternquellen mit konservativen ACLs und Provenienz.
6. Verhindere rekursive Digest-Fluten: generierte Ergebnisse nicht ungeprüft erneut als neue Originalereignisse einspeisen. Redaktions-, Quarantäne- und Aufbewahrungsregeln umsetzen.
7. Arbeite mit 03 an leerem Rebuild und Deltaabgleich; entferne alte Cronjobs erst im kontrollierten Writer-Cutover von 11.

**Liefergegenstände:** Rust-Worker/Connectoren, Parser-/Chunkingfixtures, Job-/Retrytests, Rebuildbericht, Feeder-A/B-Messung und Source-Versorgungsstatus.

**Abnahme:** Reguläre Versorgung, Deletion, Rechteänderung, Wiederanlauf und kompletter Rebuild funktionieren in Rust. Eine einmalige Konvertierung ist keine dauerhafte versteckte Pythonabhängigkeit.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13. Baue die gemeinsame Worker-/Fetch-/Raw-/Checkpoint-/Publishinfrastruktur; 12 liefert Wiki-, 13 externe API/Git-/Schema- und 14 Replaymodule. Keine konkurrierenden eigenständigen Scheduler/Publisher.

Vorhandene unveränderte Revisionen nicht neu verarbeiten. Template-/Data-/Schema-/Fact-Abhängigkeiten gezielt invalidieren; LLM-Digests nicht als Originalfakten zurückeinspeisen. Quarantäne muss Aktivierung verhindern. Replay-/Bulk-/Embeddingjobs erhalten eigene Ressourcenbudgets und begrenzte Queues. Endabnahme zeigt Betrieb unter parallelem Liveverkehr.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Implementiere einen deterministischen Ende-zu-Ende-Connector mit Update, Delete und Wiederanlauf, bevor du mehrere Quellen parallel ausbaust.
